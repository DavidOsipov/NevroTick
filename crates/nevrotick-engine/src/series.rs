// SPDX-License-Identifier: GPL-2.0-only
//! Three planned same-peer acquisitions with private, consume-once receiving state.
//!
//! One actual execution origin is retained across three distinct request/socket
//! owners. Transport completion and nominal arithmetic are historical evidence;
//! physical admission, statistical samples, authentication and eligibility remain
//! unavailable. The original two-peer pilot is not rearmed or repeatedly invoked.

use std::{
    fmt,
    net::SocketAddr,
    pin::Pin,
    sync::Arc,
    time::{Duration, Instant},
};

use garde::Validate;
use tokio::{
    sync::watch,
    time::{Instant as TokioInstant, Sleep, sleep_until},
};

use super::{
    Control, ControlHandle, ControlSchedule, ExperimentalProfile, FailureSummary, IoCause,
    IoOutcome, LiveSlot, MAX_TIMEOUT, NativeDatagramPlatform, NativeReadOutcome, PACKET_BYTES,
    PilotError, RECEIVE_BYTES, Rejection, RunClockContext, SlotReport, SoftwareClock, StagedReply,
    Status, TerminalOutcome, TimerAction, attempt_send, charge_selected_datagram,
    classify_read_result, commit_reply, finish_timer, latest, prepare_slot, stage_reply,
    start_timer,
};
use ntp_wire::FrameLimits;

/// Fixed planned attempts, never a target count of successful measurements.
const ATTEMPTS: usize = 3;

/// Raw new configuration fields share one mandatory bounded rule.
#[derive(Validate)]
struct ConfigCandidate {
    /// Endpoint and both lifetimes are admitted together before preparation.
    #[garde(custom(config_rule))]
    values: (SocketAddr, Duration, Duration),
}

/// Classify the complete fixed laboratory configuration without effects.
fn config_issue((peer, request, series): (SocketAddr, Duration, Duration)) -> Option<PilotError> {
    if !peer.ip().is_loopback() || peer.port() == 0 {
        return Some(PilotError::InvalidPeer);
    }
    if let SocketAddr::V6(address) = peer
        && (address.scope_id() != 0 || address.flowinfo() != 0)
    {
        return Some(PilotError::InvalidPeer);
    }
    if request.is_zero() || series.is_zero() || request > series || series > MAX_TIMEOUT {
        return Some(PilotError::InvalidTimeout);
    }
    None
}

/// Adapt the configuration predicate without exposing endpoint or duration data.
/// # Errors
/// Return fixed text for an invalid laboratory configuration.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde custom rules require borrowed raw fields and their unit context"
)]
fn config_rule(values: &(SocketAddr, Duration, Duration), (): &()) -> garde::Result {
    if config_issue(*values).is_none() {
        Ok(())
    } else {
        Err(garde::Error::new(
            "invalid acquisition series configuration",
        ))
    }
}

/// Checked immutable plan for exactly three sequential attempts to one peer.
///
/// No source identity, production generation, quality or eligibility is asserted.
/// Raw construction and mutation cannot bypass admission:
/// ```compile_fail,E0451
/// use nevrotick_engine::SeriesConfig;
/// fn bypass(base: SeriesConfig) {
///     let _ = SeriesConfig { request_timeout: std::time::Duration::ZERO, ..base };
/// }
/// ```
/// ```compile_fail,E0616
/// use nevrotick_engine::SeriesConfig;
/// fn mutate(mut plan: SeriesConfig) { plan.series_timeout = std::time::Duration::ZERO; }
/// ```
/// ```compile_fail,E0599
/// use nevrotick_engine::SeriesConfig;
/// let _ = SeriesConfig::default();
/// ```
pub struct SeriesConfig {
    /// One explicitly configured, checked loopback endpoint.
    peer: SocketAddr,
    /// Positive per-request lifetime, no greater than the series lifetime.
    request_timeout: Duration,
    /// Positive total lifetime, at most the existing thirty-second lab ceiling.
    series_timeout: Duration,
    /// Optional already-checked experimental declaration; not physical evidence.
    profile: Option<ExperimentalProfile>,
}

impl SeriesConfig {
    /// Admit endpoint and lifetimes through the sole mandatory Garde path.
    /// # Errors
    /// Reject endpoint first, then zero, inverted or oversized lifetimes.
    ///
    /// ```
    /// use std::{net::SocketAddr, time::Duration};
    /// use nevrotick_engine::SeriesConfig;
    /// let peer = SocketAddr::from(([127, 0, 0, 1], 41001));
    /// assert!(SeriesConfig::new(peer, Duration::from_millis(100), Duration::from_secs(1)).is_ok());
    /// assert!(SeriesConfig::new(peer, Duration::ZERO, Duration::from_secs(1)).is_err());
    /// ```
    pub fn new(
        peer: SocketAddr,
        request_timeout: Duration,
        series_timeout: Duration,
    ) -> Result<Self, PilotError> {
        let candidate = ConfigCandidate {
            values: (peer, request_timeout, series_timeout),
        };
        candidate
            .validate()
            .map_err(|_| config_issue(candidate.values).unwrap_or(PilotError::InternalState))?;
        Ok(Self {
            peer,
            request_timeout,
            series_timeout,
            profile: None,
        })
    }

    /// Attach an immutable checked declaration without granting physical admission.
    #[must_use]
    pub fn with_experimental_profile(mut self, profile: ExperimentalProfile) -> Self {
        self.profile = Some(profile);
        self
    }
}

impl fmt::Debug for SeriesConfig {
    /// Redact endpoint, durations and the experimental declaration.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SeriesConfig { .. }")
    }
}

/// Fixed entropy-derived tokens are admitted together before any socket exists.
#[derive(Validate)]
struct TokenCandidate {
    /// Three nonzero, pairwise distinct opaque request tokens.
    #[garde(custom(token_rule))]
    tokens: [[u8; 8]; ATTEMPTS],
}

/// Classify fixed token bytes without formatting, retries or external effects.
fn token_issue(tokens: &[[u8; 8]; ATTEMPTS]) -> Option<PilotError> {
    if tokens.contains(&[0; 8]) {
        Some(PilotError::ZeroToken)
    } else if tokens[0] == tokens[1] || tokens[0] == tokens[2] || tokens[1] == tokens[2] {
        Some(PilotError::DuplicateToken)
    } else {
        None
    }
}

/// Adapt fixed token admission to Garde without exposing correlation bytes.
/// # Errors
/// Reject zero or repeated tokens with fixed bounded text.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde custom rules require borrowed raw fields and their unit context"
)]
fn token_rule(tokens: &[[u8; 8]; ATTEMPTS], (): &()) -> garde::Result {
    if token_issue(tokens).is_none() {
        Ok(())
    } else {
        Err(garde::Error::new("invalid acquisition series tokens"))
    }
}

/// Construct three inert requests from one successfully filled entropy buffer.
/// # Errors
/// Reject zero/duplicate tokens through mandatory validation, without regeneration.
fn series_packets(entropy: &[u8; 24]) -> Result<[[u8; PACKET_BYTES]; ATTEMPTS], PilotError> {
    // The fixed twenty-four-byte input has exactly three complete eight-byte chunks.
    let (tokens, _) = entropy.as_chunks::<8>();
    let candidate = TokenCandidate {
        tokens: tokens.try_into().map_err(|_| PilotError::InternalState)?,
    };
    candidate
        .validate()
        .map_err(|_| token_issue(&candidate.tokens).unwrap_or(PilotError::InternalState))?;
    let mut packets = [[0; PACKET_BYTES]; ATTEMPTS];
    for (packet, token) in packets.iter_mut().zip(candidate.tokens) {
        packet[0] = 0x23;
        packet[40..48].copy_from_slice(&token);
    }
    Ok(packets)
}

/// Opaque entropy/control preparation; no socket, runtime or queue is created.
///
/// Preparation is consumed exactly once:
/// ```compile_fail,E0382
/// use nevrotick_engine::{PreparedSeries, ControlSchedule, execute_series};
/// fn reuse(plan: PreparedSeries, first: ControlSchedule, second: ControlSchedule) {
///     let _ = execute_series(plan, &first);
///     let _ = execute_series(plan, &second);
/// }
/// ```
/// ```compile_fail,E0451
/// use nevrotick_engine::PreparedSeries;
/// fn bypass(base: PreparedSeries, peer: std::net::SocketAddr) { let _ = PreparedSeries { peer, ..base }; }
/// ```
pub struct PreparedSeries {
    /// Immutable endpoint admitted by the actual configuration constructor.
    peer: SocketAddr,
    /// Admitted per-request lifetime.
    request_timeout: Duration,
    /// Admitted total lifetime.
    series_timeout: Duration,
    /// Retained optional declaration, unchanged for all requests.
    profile: Option<ExperimentalProfile>,
    /// Exactly three distinct inert requests; never published or logged.
    packets: [[u8; PACKET_BYTES]; ATTEMPTS],
    /// The unique series owner's current control stream.
    control: watch::Receiver<Control>,
    /// Internal sender makes external-handle drop harmless.
    control_handle: ControlHandle,
}

impl fmt::Debug for PreparedSeries {
    /// Redact packets, controls and configured payloads.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PreparedSeries { .. }")
    }
}

/// Read exactly twenty-four entropy bytes once and retain three checked requests.
///
/// Creates no socket/runtime; successful preparation owns one bounded watch value.
/// # Errors
/// Reject an entered runtime before entropy, then entropy or token admission.
pub fn prepare_series(
    config: &SeriesConfig,
) -> Result<(PreparedSeries, ControlHandle), PilotError> {
    prepare_series_with_entropy(config, |entropy| {
        getrandom::fill(entropy).map_err(|_| PilotError::Entropy)
    })
}

/// Invoke exactly one entropy operation after rejecting an entered runtime.
/// The private seam changes no public provider or retry policy; production uses
/// the single original getrandom fill, and fault controls establish propagation only.
/// # Errors
/// Preserve entered-runtime, entropy or subsequent mandatory token refusal.
fn prepare_series_with_entropy(
    config: &SeriesConfig,
    fill: impl FnOnce(&mut [u8; 24]) -> Result<(), PilotError>,
) -> Result<(PreparedSeries, ControlHandle), PilotError> {
    if tokio::runtime::Handle::try_current().is_ok() {
        return Err(PilotError::EnteredRuntime);
    }
    let mut entropy = [0; 24];
    fill(&mut entropy)?;
    let packets = series_packets(&entropy)?;
    let (sender, control) = watch::channel(Control {
        stop: false,
        epoch: 0,
    });
    let control_handle = ControlHandle { sender };
    Ok((
        PreparedSeries {
            peer: config.peer,
            request_timeout: config.request_timeout,
            series_timeout: config.series_timeout,
            profile: config.profile,
            packets,
            control,
            control_handle: control_handle.clone(),
        },
        control_handle,
    ))
}

/// Closed reason a remaining planned request was never started.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SeriesNotStartedCause {
    /// The owner observed a latched stop.
    Stopped,
    /// The owner observed an invalidated laboratory epoch.
    Stale,
    /// The original whole-series deadline was reached.
    SeriesDeadline,
    /// Setup or an internal invariant failed; the report retains its primary cause.
    PrimaryFailure,
}

/// Whole-series owner decision at its final checkpoint, distinct from request facts.
/// A later control/deadline observation never rewrites a retained terminal record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SeriesEnd {
    /// All three positions transferred and the final owner checkpoint completed.
    /// Individual transport results may still be timeouts, controls or refusals.
    Completed,
    /// The owner observed stop, retiring unfinished work and suppressing later sends.
    Stopped,
    /// The owner observed invalidation, preserving earlier terminal records.
    Stale,
    /// The owner observed the original total deadline at a series checkpoint.
    /// This may follow a terminal transfer; every transferred record is retained.
    SeriesDeadline,
    /// A primary setup/invariant or an independently retained cleanup cause failed.
    Failure,
}

/// Private exclusive state: a not-started position or its actual moved completion.
enum AttemptState {
    /// No registered request produced a terminal record at this position.
    NotStarted(SeriesNotStartedCause),
    /// Historical terminal record, inseparable from its matching facts.
    Report(SlotReport),
}

/// One immutable planned position; it cannot be relabelled or independently filled.
/// ```compile_fail,E0616
/// use nevrotick_engine::SeriesAttempt;
/// fn relabel(mut attempt: SeriesAttempt) { attempt.index = 0; }
/// ```
pub struct SeriesAttempt {
    /// Original fixed plan position, zero through two.
    index: u8,
    /// Sole privately transferred historical result, or explicit non-start reason.
    state: AttemptState,
    /// Actual retired request identity, including terminal outcomes without facts.
    identity: Option<Arc<()>>,
}

impl SeriesAttempt {
    /// Create a private empty position from a code-owned fixed plan index.
    fn empty(index: u8) -> Self {
        Self {
            index,
            state: AttemptState::NotStarted(SeriesNotStartedCause::PrimaryFailure),
            identity: None,
        }
    }

    /// Return the immutable original position, zero through two.
    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }

    /// Borrow only this position's actual transferred terminal report.
    #[must_use]
    pub const fn report(&self) -> Option<&SlotReport> {
        match &self.state {
            AttemptState::Report(report) => Some(report),
            AttemptState::NotStarted(_) => None,
        }
    }

    /// Return a typed reason only when this position has no terminal report.
    #[must_use]
    pub const fn not_started_cause(&self) -> Option<SeriesNotStartedCause> {
        match self.state {
            AttemptState::NotStarted(cause) => Some(cause),
            AttemptState::Report(_) => None,
        }
    }
}

impl fmt::Debug for SeriesAttempt {
    /// Redact its terminal payload, retaining only the fixed type family.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SeriesAttempt { .. }")
    }
}

/// Closed admission state; transport collection cannot create statistical samples.
enum StatisticalAdmission {
    /// Physical/date/quality admission is not implemented by this source owner.
    Unavailable,
}

/// Owned partial or complete historical series after timer/runtime cleanup.
///
/// No source quality, physical accuracy or admitted statistical sample is supplied.
/// ```compile_fail,E0616
/// use nevrotick_engine::SeriesReport;
/// fn rewrite(mut report: SeriesReport) { report.primary_failure = None; }
/// ```
pub struct SeriesReport {
    /// Actual shared execution identity/origin, retaining no native resource.
    context: Arc<RunClockContext>,
    /// Immutable configured source endpoint for this series only.
    peer: SocketAddr,
    /// All three original positions, including explicit not-started ones.
    attempts: [SeriesAttempt; ATTEMPTS],
    /// Closed whole-series outcome after cleanup reconciliation.
    end: SeriesEnd,
    /// Original primary failure; cleanup cannot replace or erase it.
    primary_failure: Option<FailureSummary>,
    /// Independently retained stop-timer cleanup cause.
    stop_cleanup_failure: Option<FailureSummary>,
    /// Independently retained invalidation-timer cleanup cause.
    invalidate_cleanup_failure: Option<FailureSummary>,
    /// Actual elapsed time after runtime drop; zero until the outer owner returns.
    done_elapsed_ns: u128,
    /// Explicit unavailable statistical admission, distinct from transport success.
    admission: StatisticalAdmission,
}

impl SeriesReport {
    /// Borrow all original planned positions without recomputation or effects.
    #[must_use]
    pub const fn attempts(&self) -> &[SeriesAttempt; ATTEMPTS] {
        &self.attempts
    }
    /// Return the whole-series outcome, including any cleanup failure.
    #[must_use]
    pub const fn end(&self) -> SeriesEnd {
        self.end
    }
    /// Return the separately retained primary cause, if any.
    #[must_use]
    pub const fn primary_failure(&self) -> Option<FailureSummary> {
        self.primary_failure
    }
    /// Return the stop job's actual cleanup failure, if any.
    #[must_use]
    pub const fn stop_cleanup_failure(&self) -> Option<FailureSummary> {
        self.stop_cleanup_failure
    }
    /// Return the invalidation job's actual cleanup failure, if any.
    #[must_use]
    pub const fn invalidate_cleanup_failure(&self) -> Option<FailureSummary> {
        self.invalidate_cleanup_failure
    }
    /// Return the actual origin captured before runtime and native setup.
    #[must_use]
    pub fn origin(&self) -> Instant {
        self.context.origin
    }
    /// Return the configured loopback endpoint, without server-authentication claims.
    #[must_use]
    pub const fn peer_endpoint(&self) -> SocketAddr {
        self.peer
    }
    /// Return elapsed active lifetime through timer cleanup and runtime drop.
    #[must_use]
    pub const fn done_elapsed_ns(&self) -> u128 {
        self.done_elapsed_ns
    }
    /// This acquisition slice never admits a statistical sample.
    #[must_use]
    pub const fn admitted_sample_count(&self) -> usize {
        match self.admission {
            StatisticalAdmission::Unavailable => 0,
        }
    }
    /// This unauthenticated acquisition slice never supplies eligible samples.
    #[must_use]
    pub const fn eligible_sample_count(&self) -> usize {
        match self.admission {
            StatisticalAdmission::Unavailable => 0,
        }
    }
}

impl fmt::Debug for SeriesReport {
    /// Redact origin, endpoint, captures, request identities and numerical payloads.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SeriesReport { .. }")
    }
}

/// Actual request/socket association copied only from the exclusive live owner.
struct RequestOwner {
    /// Fixed original series position.
    index: u8,
    /// Actual socket/request identity, distinct from its numeric endpoint.
    identity: Arc<()>,
    /// Actual series identity and shared pre-runtime origin.
    context: Arc<RunClockContext>,
    /// Actual checked destination retained by the request.
    peer: SocketAddr,
    /// Actual socket's original bound local endpoint.
    local: SocketAddr,
    /// Original immutable experimental declaration.
    profile: Option<ExperimentalProfile>,
}

impl RequestOwner {
    /// Borrow the actual live request's identity/context without cloning its socket.
    fn from_slot(index: u8, slot: &LiveSlot) -> Self {
        Self {
            index,
            identity: Arc::clone(&slot.identity),
            context: Arc::clone(&slot.run_context),
            peer: slot.peer,
            local: slot.local,
            profile: slot.profile,
        }
    }

    /// Compare complete actual request association; equal endpoints alone never pass.
    fn same_request(&self, other: &Self) -> bool {
        self.index == other.index
            && Arc::ptr_eq(&self.identity, &other.identity)
            && Arc::ptr_eq(&self.context, &other.context)
            && self.peer == other.peer
            && self.local == other.local
            && self.profile == other.profile
    }
}

/// Move-only terminal transfer, never constructible from public copied scalars.
struct Completion {
    /// Actual request that was retired before its socket was dropped.
    owner: RequestOwner,
    /// Same request's owned terminal report and optional actual transport facts.
    report: SlotReport,
}

/// Sole receiving transaction owner; prior positions cannot be overwritten.
struct SeriesReceiver {
    /// Actual shared series context created during this execution.
    context: Arc<RunClockContext>,
    /// Immutable configured source endpoint.
    peer: SocketAddr,
    /// Immutable optional experimental declaration for every issued request.
    profile: Option<ExperimentalProfile>,
    /// Next original position eligible for issue/transfer, at most three.
    next: usize,
    /// At most one outstanding actual request descriptor, retaining no socket.
    issued: Option<RequestOwner>,
    /// Privately associated historical positions, replaced only by matching moves.
    attempts: [SeriesAttempt; ATTEMPTS],
}

impl SeriesReceiver {
    /// Initialize three empty positions without native work or sample admission.
    fn new(
        context: Arc<RunClockContext>,
        peer: SocketAddr,
        profile: Option<ExperimentalProfile>,
    ) -> Self {
        Self {
            context,
            peer,
            profile,
            next: 0,
            issued: None,
            attempts: [
                SeriesAttempt::empty(0),
                SeriesAttempt::empty(1),
                SeriesAttempt::empty(2),
            ],
        }
    }

    /// Retain only the next actual request in the same immutable series context.
    /// # Errors
    /// Reject duplicate/foreign/out-of-order issue without altering prior state.
    fn issue(&mut self, owner: RequestOwner) -> Result<(), PilotError> {
        if usize::from(owner.index) != self.next
            || self.next >= ATTEMPTS
            || self.issued.is_some()
            || !Arc::ptr_eq(&owner.context, &self.context)
            || owner.peer != self.peer
            || owner.profile != self.profile
            || self.attempts.iter().any(|attempt| {
                attempt
                    .identity
                    .as_ref()
                    .is_some_and(|identity| Arc::ptr_eq(identity, &owner.identity))
            })
        {
            return Err(PilotError::InternalState);
        }
        self.issued = Some(owner);
        Ok(())
    }

    /// Move one actual completion after the same exclusive control checkpoint.
    /// # Errors
    /// Reject duplicate/foreign/stale transfers without overwriting any position.
    fn receive(&mut self, completion: Completion, state: Control) -> Result<(), PilotError> {
        let expected = self.issued.as_ref().ok_or(PilotError::InternalState)?;
        let status = completion.report.status();
        let current = if state.stop {
            status == Status::Stopped
        } else if state.epoch != 0 {
            status == Status::Stale
        } else {
            !matches!(status, Status::Stopped | Status::Stale)
        };
        let facts_match = completion.report.facts().is_none_or(|facts| {
            Arc::ptr_eq(&facts.identity, &completion.owner.identity)
                && Arc::ptr_eq(&facts.run_context, &completion.owner.context)
                && facts.peer == completion.owner.peer
                && facts.local == completion.owner.local
                && facts.profile == completion.owner.profile
        });
        if !expected.same_request(&completion.owner)
            || !current
            || !facts_match
            || completion.report.slot != completion.owner.index
            || completion.report.local != completion.owner.local
        {
            return Err(PilotError::InternalState);
        }
        let next = self
            .next
            .checked_add(1)
            .filter(|next| *next <= ATTEMPTS)
            .ok_or(PilotError::InternalState)?;
        let position = self
            .attempts
            .get_mut(self.next)
            .ok_or(PilotError::InternalState)?;
        if !matches!(position.state, AttemptState::NotStarted(_)) {
            return Err(PilotError::InternalState);
        }
        position.identity = Some(Arc::clone(&completion.owner.identity));
        position.state = AttemptState::Report(completion.report);
        self.issued = None;
        self.next = next;
        Ok(())
    }

    /// Publish all retained positions and explicitly classify every remaining one.
    fn finish(mut self, mut end: SeriesEnd, mut primary: Option<PilotError>) -> SeriesReport {
        if end == SeriesEnd::Completed && (self.next != ATTEMPTS || self.issued.is_some()) {
            end = SeriesEnd::Failure;
            primary = primary.or(Some(PilotError::InternalState));
        }
        if primary.is_some() {
            end = SeriesEnd::Failure;
        }
        let cause = match end {
            SeriesEnd::Stopped => SeriesNotStartedCause::Stopped,
            SeriesEnd::Stale => SeriesNotStartedCause::Stale,
            SeriesEnd::SeriesDeadline => SeriesNotStartedCause::SeriesDeadline,
            SeriesEnd::Completed | SeriesEnd::Failure => SeriesNotStartedCause::PrimaryFailure,
        };
        for attempt in &mut self.attempts {
            if matches!(attempt.state, AttemptState::NotStarted(_)) {
                attempt.state = AttemptState::NotStarted(cause);
            }
        }
        SeriesReport {
            context: self.context,
            peer: self.peer,
            attempts: self.attempts,
            end,
            primary_failure: primary.map(FailureSummary::capture),
            stop_cleanup_failure: None,
            invalidate_cleanup_failure: None,
            done_elapsed_ns: 0,
            admission: StatisticalAdmission::Unavailable,
        }
    }
}

/// Select control retirement before checking the unchanged whole-series deadline.
fn series_end(state: Control, now: Instant, deadline: Instant) -> Option<SeriesEnd> {
    if state.stop {
        Some(SeriesEnd::Stopped)
    } else if state.epoch != 0 {
        Some(SeriesEnd::Stale)
    } else if now >= deadline {
        Some(SeriesEnd::SeriesDeadline)
    } else {
        None
    }
}

/// Check request deadline arithmetic without resetting the original total bound.
/// # Errors
/// Reject overflowing request arithmetic before creating a request socket.
fn request_deadline(
    start: Instant,
    lifetime: Duration,
    total: Instant,
) -> Result<Instant, PilotError> {
    Ok(start
        .checked_add(lifetime)
        .ok_or(PilotError::DeadlineOverflow)?
        .min(total))
}

/// Retire an unfinished request under the actual latest control/deadline snapshot.
fn refresh_slot(slot: &mut LiveSlot, state: Control, now: Instant, deadline: Instant) {
    if state.stop {
        slot.finish(TerminalOutcome::Stopped);
    } else if state.epoch != 0 {
        slot.finish(TerminalOutcome::Stale);
    }
    if now >= deadline {
        slot.expire();
    }
}

/// One selected operation, with no borrowed buffer or pending receive future.
enum SeriesWake {
    /// The watch changed or closed.
    Control(bool),
    /// This request's bounded deadline became ready.
    Deadline,
    /// Socket readiness; the native send still requires a fresh owner guard.
    Writable(std::io::Result<()>),
    /// Completed native receive and its adjacent original capture bracket.
    Read(SeriesRead),
}

/// One completed receive and its adjacent capture; no buffer or future escapes.
struct SeriesRead {
    /// Actual native read classification, bounded by the original receive buffer.
    result: NativeReadOutcome,
    /// Original adjacent software observations.
    read: SoftwareClock,
}

/// Await one bounded request; dropped receive futures cannot escape this owner.
async fn next_series_wake(
    slot: &LiveSlot,
    buffer: &mut [u8; RECEIVE_BYTES],
    control: &mut watch::Receiver<Control>,
    open: bool,
    mut timer: Pin<&mut Sleep>,
) -> SeriesWake {
    tokio::select! { biased;
        changed = control.changed(), if open => SeriesWake::Control(changed.is_ok()),
        () = &mut timer => SeriesWake::Deadline,
        result = slot.socket.recv_from(buffer), if slot.can_receive() => {
            let read = SoftwareClock::capture();
            SeriesWake::Read(SeriesRead { result: classify_read_result(NativeDatagramPlatform::current(), result), read })
        },
        result = slot.socket.writable(), if slot.can_send() => SeriesWake::Writable(result),
    }
}

/// Stage semantic arithmetic, then recheck control/deadline immediately at commit.
fn process_series_reply(
    slot: &mut LiveSlot,
    control: &mut watch::Receiver<Control>,
    bytes: &[u8],
    source: SocketAddr,
    read: SoftwareClock,
    limits: FrameLimits,
    deadline: Instant,
) -> Control {
    let staged = stage_reply(slot, bytes, source, read, limits);
    publish_series_reply(slot, control, staged, deadline)
}

/// Publish an actual staged reply only after a fresh serialized owner checkpoint.
///
/// Staging supplies no current permission: controls and the processing deadline
/// are read here after all semantic/measurement work, with no await before commit.
/// Refusal preserves prior payloads and irreversibly retires an invalidated request.
fn publish_series_reply(
    slot: &mut LiveSlot,
    control: &mut watch::Receiver<Control>,
    staged: Result<StagedReply, Rejection>,
    deadline: Instant,
) -> Control {
    if let Err(reason) = &staged {
        slot.last_rejection = Some(*reason);
        slot.rejected += 1;
    }
    let state = latest(control);
    let processing = Instant::now();
    refresh_slot(slot, state, processing, deadline);
    if slot.done() {
        if staged.is_ok() {
            slot.rejected += 1;
        }
    } else if let Ok(candidate) = staged {
        commit_reply(slot, &candidate, processing);
    }
    state
}

/// Charge actual datagrams even after retirement; never read a refused prefix.
fn handle_series_read(
    slot: &mut LiveSlot,
    control: &mut watch::Receiver<Control>,
    selected: SeriesRead,
    buffer: &[u8; RECEIVE_BYTES],
    limits: FrameLimits,
    deadline: Instant,
    state: Control,
) -> Control {
    let SeriesRead { result, read } = selected;
    let actual = charge_selected_datagram(slot, &result);
    if slot.done() {
        if actual {
            slot.rejected += 1;
        }
        return state;
    }
    match result {
        NativeReadOutcome::Datagram { length, source } => {
            if let Some(bytes) = buffer.get(..length) {
                return process_series_reply(slot, control, bytes, source, read, limits, deadline);
            }
            slot.last_rejection = Some(Rejection::Length);
            slot.rejected += 1;
        }
        NativeReadOutcome::ConsumedOversize => {
            slot.last_rejection = Some(Rejection::Length);
            slot.rejected += 1;
        }
        NativeReadOutcome::Io(cause) => slot.finish(TerminalOutcome::Io(IoOutcome::Native(cause))),
    }
    state
}

/// Handle one observed writable result at the current exclusive owner boundary.
/// A retired slot preserves the input snapshot without another watch read or I/O.
/// Otherwise refresh the latest control/deadline once before the original native
/// send or native-error retirement; no await or operation escapes this owner.
fn handle_series_writable(
    slot: &mut LiveSlot,
    control: &mut watch::Receiver<Control>,
    result: std::io::Result<()>,
    deadline: Instant,
    mut state: Control,
) -> Control {
    if !slot.done() {
        state = latest(control);
        refresh_slot(slot, state, Instant::now(), deadline);
        if !slot.done() {
            match result {
                Ok(()) => attempt_send(slot, deadline),
                Err(error) => slot.finish(TerminalOutcome::Io(IoOutcome::Native(
                    IoCause::capture(&error),
                ))),
            }
        }
    }
    state
}

/// Drive one one-way request and retain the last exclusive transfer checkpoint.
///
/// Return immediately on terminal commit, without refreshing that snapshot:
/// control published afterward cannot erase historical completion. The next
/// planned request separately checks the then-current watch state before setup.
async fn run_request(
    mut slot: LiveSlot,
    control: &mut watch::Receiver<Control>,
    deadline: Instant,
    limits: FrameLimits,
) -> (LiveSlot, Control) {
    let mut buffer = [0; RECEIVE_BYTES];
    let timer = sleep_until(TokioInstant::from_std(deadline));
    tokio::pin!(timer);
    let mut open = true;
    loop {
        let mut state = latest(control);
        refresh_slot(&mut slot, state, Instant::now(), deadline);
        if slot.done() {
            return (slot, state);
        }
        let wake = next_series_wake(&slot, &mut buffer, control, open, timer.as_mut()).await;
        state = latest(control);
        refresh_slot(&mut slot, state, Instant::now(), deadline);
        match wake {
            SeriesWake::Control(value) => open = value,
            SeriesWake::Deadline => slot.expire(),
            SeriesWake::Writable(result) => {
                state = handle_series_writable(&mut slot, control, result, deadline, state);
            }
            SeriesWake::Read(selected) => {
                state = handle_series_read(
                    &mut slot, control, selected, &buffer, limits, deadline, state,
                );
            }
        }
        if slot.done() {
            return (slot, state);
        }
    }
}

/// Inputs to the one actual bind/registration operation for a planned request.
struct RequestSetup {
    /// Immutable checked destination for this series.
    peer: SocketAddr,
    /// This planned request's separately admitted opaque token packet.
    packet: [u8; PACKET_BYTES],
    /// Original optional checked experimental declaration.
    profile: Option<ExperimentalProfile>,
    /// Actual common series identity/origin, never a caller label.
    context: Arc<RunClockContext>,
}

/// Bind and register exactly one fresh socket through the unchanged native owners.
/// # Errors
/// Preserve the original typed bind/nonblocking/address/registration cause.
fn setup_request(setup: RequestSetup) -> Result<LiveSlot, PilotError> {
    let mut request = prepare_slot(setup.peer, setup.packet)?;
    request.profile = setup.profile;
    LiveSlot::from_prepared(request, setup.context)
}

/// Execute exactly three planned sequential requests, preserving every prior record.
/// Only the private bind/registration operation is injectable for causal failure
/// controls; the public path always passes the unchanged actual setup backend.
async fn run_series<F>(
    mut prepared: PreparedSeries,
    mut receiver: SeriesReceiver,
    deadline: Instant,
    setup: &mut F,
) -> SeriesReport
where
    F: FnMut(RequestSetup) -> Result<LiveSlot, PilotError>,
{
    let Ok(limits) = FrameLimits::new(PACKET_BYTES, 0) else {
        return receiver.finish(SeriesEnd::Failure, Some(PilotError::WireSetup));
    };
    for (index, packet) in prepared.packets.into_iter().enumerate() {
        if let Some(end) = series_end(latest(&mut prepared.control), Instant::now(), deadline) {
            return receiver.finish(end, None);
        }
        let request_end = match request_deadline(Instant::now(), prepared.request_timeout, deadline)
        {
            Ok(value) => value,
            Err(error) => return receiver.finish(SeriesEnd::Failure, Some(error)),
        };
        let slot = match setup(RequestSetup {
            peer: prepared.peer,
            packet,
            profile: prepared.profile,
            context: Arc::clone(&receiver.context),
        }) {
            Ok(value) => value,
            Err(error) => return receiver.finish(SeriesEnd::Failure, Some(error)),
        };
        let Ok(position) = u8::try_from(index) else {
            return receiver.finish(SeriesEnd::Failure, Some(PilotError::InternalState));
        };
        if let Err(error) = receiver.issue(RequestOwner::from_slot(position, &slot)) {
            return receiver.finish(SeriesEnd::Failure, Some(error));
        }
        let (slot, state) = run_request(slot, &mut prepared.control, request_end, limits).await;
        let owner = RequestOwner::from_slot(position, &slot);
        let report = match slot.into_report(position) {
            Ok(value) => value,
            Err(error) => return receiver.finish(SeriesEnd::Failure, Some(error)),
        };
        if let Err(error) = receiver.receive(Completion { owner, report }, state) {
            return receiver.finish(SeriesEnd::Failure, Some(error));
        }
        if let Some(end) = series_end(state, Instant::now(), deadline) {
            return receiver.finish(end, None);
        }
    }
    receiver.finish(SeriesEnd::Completed, None)
}

/// Preserve each cleanup cause beside the owned partial report and primary cause.
fn reconcile_cleanup(
    mut report: SeriesReport,
    stop: Result<(), PilotError>,
    invalidate: Result<(), PilotError>,
) -> SeriesReport {
    report.stop_cleanup_failure = stop.err().map(FailureSummary::capture);
    report.invalidate_cleanup_failure = invalidate.err().map(FailureSummary::capture);
    if report.stop_cleanup_failure.is_some() || report.invalidate_cleanup_failure.is_some() {
        report.end = SeriesEnd::Failure;
    }
    report
}

/// Start at most two owned control jobs and reap both after every active outcome.
async fn run_series_owned(
    prepared: PreparedSeries,
    receiver: SeriesReceiver,
    deadline: Instant,
    stop_when: Option<Instant>,
    invalidate_when: Option<Instant>,
) -> SeriesReport {
    let handle = prepared.control_handle.clone();
    let stop_job = match start_timer(stop_when, handle.clone(), TimerAction::Stop) {
        Ok(job) => job,
        Err(error) => return receiver.finish(SeriesEnd::Failure, Some(error)),
    };
    let invalidate_job = start_timer(invalidate_when, handle, TimerAction::Invalidate);
    match invalidate_job {
        Ok(job) => {
            let mut setup = setup_request;
            let report = run_series(prepared, receiver, deadline, &mut setup).await;
            let stop = finish_timer(stop_job).await;
            let invalidate = finish_timer(job).await;
            reconcile_cleanup(report, stop, invalidate)
        }
        Err(error) => {
            let report = receiver.finish(SeriesEnd::Failure, Some(error));
            reconcile_cleanup(report, finish_timer(stop_job).await, Ok(()))
        }
    }
}

/// Execute the fixed plan in one owned current-thread runtime and return all positions.
///
/// Origin precedes runtime/native setup. Once the runtime is active, setup and
/// cleanup failures return an owned partial report rather than discarding history.
/// Only the prepared acquisition is consumed; the immutable checked schedule is borrowed.
/// At most one socket, one 49-byte receive buffer and two control jobs are active.
/// No request is retransmitted, and each has the existing 64-send/64-read budgets.
/// Rust socket/runtime drop supplies no native Windows AFD-release qualification.
/// # Errors
/// Reject only entered-runtime, deadline-arithmetic or runtime-build failures that
/// occur before active execution; later primary/cleanup causes remain in the report.
///
/// ```no_run
/// use std::{net::SocketAddr, time::Duration};
/// use nevrotick_engine::{SeriesConfig, ControlSchedule, prepare_series, execute_series};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let config = SeriesConfig::new(SocketAddr::from(([127,0,0,1],41001)), Duration::from_millis(100), Duration::from_secs(1))?;
/// let (prepared, control) = prepare_series(&config)?;
/// let report = execute_series(prepared, &ControlSchedule::new(None, None)?)?;
/// assert_eq!(report.attempts().len(), 3);
/// assert_eq!(report.admitted_sample_count(), 0);
/// drop(control);
/// # Ok(()) }
/// ```
pub fn execute_series(
    prepared: PreparedSeries,
    schedule: &ControlSchedule,
) -> Result<SeriesReport, PilotError> {
    if tokio::runtime::Handle::try_current().is_ok() {
        return Err(PilotError::EnteredRuntime);
    }
    let origin = Instant::now();
    let deadline = origin
        .checked_add(prepared.series_timeout)
        .ok_or(PilotError::DeadlineOverflow)?;
    let stop_when = schedule
        .stop_after
        .map(|after| {
            origin
                .checked_add(after)
                .ok_or(PilotError::DeadlineOverflow)
        })
        .transpose()?;
    let invalidate_when = schedule
        .invalidate_after
        .map(|after| {
            origin
                .checked_add(after)
                .ok_or(PilotError::DeadlineOverflow)
        })
        .transpose()?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .max_blocking_threads(2)
        .build()
        .map_err(|error| PilotError::RuntimeBuild(IoCause::capture(&error)))?;
    let context = Arc::new(RunClockContext { origin });
    let receiver = SeriesReceiver::new(context, prepared.peer, prepared.profile);
    let mut report = runtime.block_on(run_series_owned(
        prepared,
        receiver,
        deadline,
        stop_when,
        invalidate_when,
    ));
    drop(runtime);
    report.done_elapsed_ns = Instant::now().duration_since(origin).as_nanos();
    Ok(report)
}

/// Focused private raw-admission, identity and receiving-transaction controls.
#[cfg(test)]
#[path = "../tests/private/series.rs"]
mod tests;

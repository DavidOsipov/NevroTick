// SPDX-License-Identifier: GPL-2.0-only
//! One finite, sequential laboratory acquisition across a frozen address set.
//!
//! This owner reuses the existing actual socket and request machinery, but never
//! combines endpoints into the same-peer series, a fit, a source vote or Core
//! history. A reply is unauthenticated transport evidence only. DNS refresh,
//! local-path observations, cross-round admission and protected NTS are absent.

#[path = "address_round/progress.rs"]
mod progress;

use std::{
    fmt,
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};

use garde::Validate;
use ntp_wire::FrameLimits;
use tokio::{
    sync::watch,
    time::{Instant as TokioInstant, sleep_until},
};

use super::{SeriesWake, handle_series_read, next_series_wake, refresh_slot, series_packets};
use crate::source_policy::{RoundBindings, SourceMode};
use crate::{
    Control, ControlHandle, ControlSchedule, FailureSummary, IoCause, IoOutcome, LiveSlot,
    MAX_TIMEOUT, PACKET_BYTES, PilotError, RECEIVE_BYTES, RunClockContext, SlotReport, Status,
    TerminalOutcome, TimerAction, attempt_send, finish_timer, latest, prepare_slot, start_timer,
};
use progress::Progress;

/// Fixed laboratory candidate capacity, matching the existing bounded slot indices.
const CAPACITY: usize = 3;

/// Bounded rejection categories; never retain or format input endpoints.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AddressRoundConfigError {
    /// The input contains zero or more than three destinations.
    Count,
    /// A destination is not unscoped loopback with a nonzero port.
    InvalidPeer,
    /// Two destinations denote the same admitted endpoint.
    DuplicatePeer,
    /// Durations are zero, exceed the lab ceiling or cannot reserve all attempts.
    InvalidTimeout,
}

impl fmt::Display for AddressRoundConfigError {
    /// Format only the closed category, without configuration payloads.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Count => "invalid address-round candidate count",
            Self::InvalidPeer => "invalid address-round endpoint",
            Self::DuplicatePeer => "duplicate address-round endpoint",
            Self::InvalidTimeout => "invalid address-round duration budget",
        })
    }
}

impl std::error::Error for AddressRoundConfigError {}

/// Borrow raw inputs until their complete, bounded rule succeeds.
#[derive(Validate)]
struct ConfigCandidate<'a> {
    /// One shared predicate owns count, endpoint identity and related durations.
    #[garde(custom(config_rule))]
    values: (&'a [SocketAddr], Duration, Duration),
}

/// Classify raw configuration before allocation or any external effect.
fn config_issue(
    (peers, request, total): (&[SocketAddr], Duration, Duration),
) -> Option<AddressRoundConfigError> {
    if peers.is_empty() || peers.len() > CAPACITY {
        return Some(AddressRoundConfigError::Count);
    }
    for (position, peer) in peers.iter().enumerate() {
        if !peer.ip().is_loopback() || peer.port() == 0 {
            return Some(AddressRoundConfigError::InvalidPeer);
        }
        if let SocketAddr::V6(address) = peer
            && (address.scope_id() != 0 || address.flowinfo() != 0)
        {
            return Some(AddressRoundConfigError::InvalidPeer);
        }
        if peers
            .get(..position)
            .is_none_or(|earlier| earlier.contains(peer))
        {
            return Some(AddressRoundConfigError::DuplicatePeer);
        }
    }
    let Ok(count) = u32::try_from(peers.len()) else {
        return Some(AddressRoundConfigError::Count);
    };
    if request.is_zero()
        || total.is_zero()
        || total > MAX_TIMEOUT
        || request
            .checked_mul(count)
            .is_none_or(|reserved| reserved > total)
    {
        return Some(AddressRoundConfigError::InvalidTimeout);
    }
    None
}

/// Adapt mandatory admission to Garde; rejected reports may allocate temporarily.
/// # Errors
/// Reject with one fixed message independent of configuration contents.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde custom rules require borrowed raw fields and their unit context"
)]
fn config_rule(values: &(&[SocketAddr], Duration, Duration), (): &()) -> garde::Result {
    if config_issue(*values).is_none() {
        Ok(())
    } else {
        Err(garde::Error::new("invalid address-round configuration"))
    }
}

/// Checked, immutable input order for one permitted laboratory round.
///
/// No default, clone, reset or mutable-candidate API exists. Calling the constructor
/// again does not supply production cross-round authority or refill its budgets.
/// ```compile_fail,E0616
/// use nevrotick_engine::AddressRoundConfig;
/// fn mutate(mut plan: AddressRoundConfig) { plan.peers.clear(); }
/// ```
/// ```compile_fail,E0451
/// use nevrotick_engine::AddressRoundConfig;
/// fn bypass(base: AddressRoundConfig) {
///     let _ = AddressRoundConfig { request_timeout: std::time::Duration::ZERO, ..base };
/// }
/// ```
/// ```compile_fail,E0599
/// use nevrotick_engine::AddressRoundConfig;
/// let _ = AddressRoundConfig::default();
/// ```
pub struct AddressRoundConfig {
    /// At most three distinct checked destinations, in original admission order.
    peers: Vec<SocketAddr>,
    /// Positive duration reserved for every candidate by the constructor.
    request_timeout: Duration,
    /// Original whole-operation budget; repeated events cannot extend it.
    round_timeout: Duration,
}

impl AddressRoundConfig {
    /// Admit the borrowed input before one bounded copy; perform no I/O.
    /// # Errors
    /// Reject invalid count/endpoints or insufficient total duration.
    ///
    /// ```
    /// use nevrotick_engine::{AddressRoundConfig, AddressRoundConfigError};
    /// use std::{net::SocketAddr, time::Duration};
    /// let peers = [SocketAddr::from(([127,0,0,1],41001)),
    ///              SocketAddr::from(([127,0,0,1],41002))];
    /// assert!(AddressRoundConfig::new(&peers, Duration::from_millis(20),
    ///                                Duration::from_millis(40)).is_ok());
    /// assert!(matches!(AddressRoundConfig::new(&peers, Duration::from_millis(20),
    ///                    Duration::from_millis(39)), Err(AddressRoundConfigError::InvalidTimeout)));
    /// ```
    pub fn new(
        peers: &[SocketAddr],
        request_timeout: Duration,
        round_timeout: Duration,
    ) -> Result<Self, AddressRoundConfigError> {
        let candidate = ConfigCandidate {
            values: (peers, request_timeout, round_timeout),
        };
        candidate.validate().map_err(|_| {
            config_issue(candidate.values).unwrap_or(AddressRoundConfigError::InvalidTimeout)
        })?;
        Ok(Self {
            peers: peers.to_vec(),
            request_timeout,
            round_timeout,
        })
    }
}

impl fmt::Debug for AddressRoundConfig {
    /// Configuration debugging does not disclose addresses or durations.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("AddressRoundConfig { .. }")
    }
}

/// Consume-once entropy and control ownership, without prepared sockets.
/// ```compile_fail,E0382
/// use nevrotick_engine::{PreparedAddressRound, ControlSchedule, execute_address_round};
/// fn reuse(plan: PreparedAddressRound, schedule: &ControlSchedule) {
///     let _ = execute_address_round(plan, schedule);
///     let _ = execute_address_round(plan, schedule);
/// }
/// ```
pub struct PreparedAddressRound {
    /// Actual source-owned batch, or the explicitly retained unmanaged API.
    source_policy: Option<RoundBindings>,
    /// Original checked input order and budgets, moved from its constructor.
    config: AddressRoundConfig,
    /// Exactly three independently validated opaque tokens; unused positions stay inert.
    packets: [[u8; PACKET_BYTES]; CAPACITY],
    /// One coalesced control stream for the whole round.
    control: watch::Receiver<Control>,
    /// Internal sender prevents external-handle drop from ending or spinning the round.
    handle: ControlHandle,
}

impl fmt::Debug for PreparedAddressRound {
    /// Never expose packets, controls or destination inputs.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PreparedAddressRound { .. }")
    }
}

/// Consume one checked plan, reading exactly 24 entropy bytes once and no socket.
/// # Errors
/// Reject an entered runtime, failed entropy or zero/duplicate tokens without retry.
pub fn prepare_address_round(
    config: AddressRoundConfig,
) -> Result<(PreparedAddressRound, ControlHandle), PilotError> {
    prepare_with_entropy(config, |entropy| {
        getrandom::fill(entropy).map_err(|_| PilotError::Entropy)
    })
}

impl crate::SourceHandle {
    /// Prepare one fixed round under this original stable logical source.
    /// Bindings are permanent after successful atomic catalogue admission, even
    /// if later entropy/preparation fails. No socket or send starts here.
    /// # Errors
    /// Preserve original binding, entered-runtime and entropy/token refusals.
    /// ```
    /// use nevrotick_engine::{AddressRoundConfig, ControlHandle, PilotError,
    ///     PreparedAddressRound, SourceHandle};
    /// fn prepare(source: &SourceHandle, config: AddressRoundConfig)
    ///     -> Result<(PreparedAddressRound, ControlHandle), PilotError> {
    ///     source.prepare_address_round(config)
    /// }
    /// ```
    pub fn prepare_address_round(
        &self,
        config: AddressRoundConfig,
    ) -> Result<(PreparedAddressRound, ControlHandle), PilotError> {
        let bindings = self
            .bind_round(&config.peers)
            .map_err(PilotError::SourcePolicy)?;
        prepare_with_policy(config, Some(bindings), |entropy| {
            getrandom::fill(entropy).map_err(|_| PilotError::Entropy)
        })
    }
}

/// Private causal entropy seam; production always uses the actual single OS fill.
/// # Errors
/// Preserve original context/entropy/token errors before constructing controls.
fn prepare_with_entropy(
    config: AddressRoundConfig,
    fill: impl FnOnce(&mut [u8; 24]) -> Result<(), PilotError>,
) -> Result<(PreparedAddressRound, ControlHandle), PilotError> {
    prepare_with_policy(config, None, fill)
}

/// Retain the actual issued policy through the unchanged one-fill preparation.
/// # Errors
/// Preserve original preparation failures; successful catalogue admission is not undone.
fn prepare_with_policy(
    config: AddressRoundConfig,
    source_policy: Option<RoundBindings>,
    fill: impl FnOnce(&mut [u8; 24]) -> Result<(), PilotError>,
) -> Result<(PreparedAddressRound, ControlHandle), PilotError> {
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
    let handle = ControlHandle { sender };
    Ok((
        PreparedAddressRound {
            source_policy,
            config,
            packets,
            control,
            handle: handle.clone(),
        },
        handle,
    ))
}

/// Closed current round outcome, separate from historical attempt records.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AddressRoundEnd {
    /// A current logical-source refusal prevented this candidate's native setup.
    SourceRestricted,
    /// An actual live request retained a policy refusal without claiming its scope.
    Policy,
    /// An ordinary correlated header completed; authentication and eligibility remain absent.
    Reply,
    /// A correlated control/KoD ended this source round without address evasion.
    Control,
    /// Every candidate was attempted once without an ordinary or control reply.
    Exhausted,
    /// Stop was admitted before the final serialized summary checkpoint.
    Stopped,
    /// Invalidation was admitted before the final serialized summary checkpoint.
    Stale,
    /// The original global deadline was reached; no new attempt may start.
    Deadline,
    /// A claimed attempt's cutoff elapsed before setup; no socket was started.
    SetupDeadline,
    /// A setup/invariant/cleanup error stopped the round; causes remain available.
    Failure,
}

/// Scope observed under the actual policy guard before candidate socket setup.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AddressPolicyRefusalScope {
    /// The stable logical-source record refused; no alternative may evade it.
    Source,
    /// Only this exact endpoint refused; the next candidate remains independent.
    Endpoint,
}

/// Historical pre-socket refusal; common round/runtime work may already exist.
/// Private construction prevents callers from inventing a skipped completion.
/// ```compile_fail,E0451
/// use nevrotick_engine::{AddressPolicyRefusal, AddressPolicyRefusalScope};
/// let _forged = AddressPolicyRefusal {
///     index: 0,
///     scope: AddressPolicyRefusalScope::Endpoint,
/// };
/// ```
pub struct AddressPolicyRefusal {
    /// Exact original candidate position whose socket/send never started.
    index: u8,
    /// Original guarded observation, not reusable current permission.
    scope: AddressPolicyRefusalScope,
}

impl AddressPolicyRefusal {
    /// Return the original candidate position without admitting a successor.
    #[must_use]
    pub const fn index(&self) -> u8 {
        self.index
    }

    /// Borrow the captured refusal meaning without rereading current policy.
    #[must_use]
    pub const fn scope(&self) -> AddressPolicyRefusalScope {
        self.scope
    }
}

impl fmt::Debug for AddressPolicyRefusal {
    /// Keep endpoint, source identity and current policy private.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("AddressPolicyRefusal { .. }")
    }
}

/// One actual attempt, with its original destination and consume-once terminal report.
pub struct AddressAttempt {
    /// Checked destination consumed by this actual socket owner.
    peer: SocketAddr,
    /// Historical terminal report; it is never promoted to source admission.
    report: SlotReport,
}

impl AddressAttempt {
    /// Return the original destination, not proof of source identity or independent voting.
    #[must_use]
    pub fn peer(&self) -> SocketAddr {
        self.peer
    }

    /// Borrow the actual historical terminal report without changing current authority.
    #[must_use]
    pub fn report(&self) -> &SlotReport {
        &self.report
    }
}

/// Bounded owned output, returned after both timer jobs and the private runtime end.
pub struct AddressRoundReport {
    /// At most three actual pre-socket refusals, disjoint from terminal attempts.
    policy_refusals: Vec<AddressPolicyRefusal>,
    /// At most three actual terminal records in their original attempt order.
    attempts: Vec<AddressAttempt>,
    /// Fresh final decision, which does not rewrite historical records.
    end: AddressRoundEnd,
    /// Original setup/invariant failure, kept beside any cleanup failures.
    primary: Option<PilotError>,
    /// Actual returned stop-timer cleanup failure, if any.
    stop_cleanup: Option<FailureSummary>,
    /// Actual returned invalidation-timer cleanup failure, if any.
    invalidate_cleanup: Option<FailureSummary>,
}

impl AddressRoundReport {
    /// Construct bounded empty receiving state; no attempt is reported as completed.
    fn empty() -> Self {
        Self {
            policy_refusals: Vec::with_capacity(CAPACITY),
            attempts: Vec::with_capacity(CAPACITY),
            end: AddressRoundEnd::Exhausted,
            primary: None,
            stop_cleanup: None,
            invalidate_cleanup: None,
        }
    }

    /// Borrow historical attempts; addresses never become additional source votes.
    #[must_use]
    pub fn attempts(&self) -> &[AddressAttempt] {
        &self.attempts
    }

    /// Borrow genuine skipped-candidate facts; absent tail entries stay absent.
    #[must_use]
    pub fn policy_refusals(&self) -> &[AddressPolicyRefusal] {
        &self.policy_refusals
    }

    /// Return the final serialized round summary, distinct from transport history.
    #[must_use]
    pub fn end(&self) -> AddressRoundEnd {
        self.end
    }

    /// Return the original setup/invariant failure without losing cleanup facts.
    #[must_use]
    pub fn primary_error(&self) -> Option<PilotError> {
        self.primary
    }

    /// Return independent stop/invalidation cleanup facts in that fixed order.
    #[must_use]
    pub fn cleanup_errors(&self) -> [Option<FailureSummary>; 2] {
        [self.stop_cleanup, self.invalidate_cleanup]
    }

    /// Stop on the original failure without discarding any preceding completions.
    fn fail(mut self, error: PilotError) -> Self {
        self.end = AddressRoundEnd::Failure;
        self.primary = Some(error);
        self
    }
}

/// Classify current control and original time without changing retained history.
fn current_end(
    control: &mut watch::Receiver<Control>,
    deadline: Instant,
) -> Option<AddressRoundEnd> {
    let state = control.borrow_and_update();
    let end = end_at(*state, Instant::now(), deadline);
    drop(state);
    end
}

/// Pure control/time classification shared by summary and the actual setup guard.
fn end_at(state: Control, now: Instant, deadline: Instant) -> Option<AddressRoundEnd> {
    if state.stop {
        Some(AddressRoundEnd::Stopped)
    } else if state.epoch != 0 {
        Some(AddressRoundEnd::Stale)
    } else if now >= deadline {
        Some(AddressRoundEnd::Deadline)
    } else {
        None
    }
}

/// Refresh retirement, then let the shared concrete sender own its sole final
/// control read guard. No nested watch read can deadlock behind a queued writer.
fn guarded_writable(
    slot: &mut LiveSlot,
    control: &mut watch::Receiver<Control>,
    writable: std::io::Result<()>,
    deadline: Instant,
) {
    let state = latest(control);
    refresh_slot(slot, state, Instant::now(), deadline);
    if !slot.done() {
        match writable {
            Ok(()) => {
                // This owner transfers the terminal slot alone and separately
                // samples its summary; no report/checkpoint pair is handed off.
                let _checkpoint = attempt_send(slot, deadline, control);
            }
            Err(error) => slot.finish(TerminalOutcome::Io(IoOutcome::Native(IoCause::capture(
                &error,
            )))),
        }
    }
}

/// Drive one actual request under the shared parser/commit and a serialized send guard.
/// Terminal completion returns unchanged; the caller separately resamples summary control.
async fn run_attempt(
    mut slot: LiveSlot,
    control: &mut watch::Receiver<Control>,
    deadline: Instant,
    limits: FrameLimits,
) -> LiveSlot {
    let mut buffer = [0; RECEIVE_BYTES];
    let timer = sleep_until(TokioInstant::from_std(deadline));
    tokio::pin!(timer);
    let mut open = true;
    loop {
        let mut state = latest(control);
        refresh_slot(&mut slot, state, Instant::now(), deadline);
        if slot.done() {
            return slot;
        }
        let wake = next_series_wake(&mut slot, &mut buffer, control, open, timer.as_mut()).await;
        state = latest(control);
        refresh_slot(&mut slot, state, Instant::now(), deadline);
        match wake {
            SeriesWake::SourcePolicy => {}
            SeriesWake::Control(value) => open = value,
            SeriesWake::Deadline => slot.expire(),
            SeriesWake::Writable(result) => guarded_writable(&mut slot, control, result, deadline),
            SeriesWake::Read(selected) => {
                let _state = handle_series_read(
                    &mut slot, control, selected, &buffer, limits, deadline, state,
                );
            }
        }
        if slot.done() {
            return slot;
        }
    }
}

/// Consume each exclusive claim through actual terminal socket/report transfer.
/// No successor can be claimed while a socket or an unfinished claim remains live.
async fn run_round(
    prepared: PreparedAddressRound,
    origin: Instant,
    deadline: Instant,
) -> AddressRoundReport {
    let bindings = prepared.source_policy.clone();
    run_round_with(
        prepared,
        origin,
        deadline,
        &mut |peer, packet, context, control, request_end, round_end| {
            if bindings.is_none() {
                return setup_address_request(
                    peer,
                    packet,
                    context,
                    control,
                    request_end,
                    round_end,
                );
            }
            let mode = candidate_mode(bindings.as_ref(), peer)?;
            setup_address_request_bound(
                peer,
                packet,
                context,
                control,
                request_end,
                round_end,
                mode,
            )
        },
        &mut |_, _| {},
    )
    .await
}

/// Fixed notification phases surrounding one serialized summary checkpoint.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RoundCheckpoint {
    /// Historical record and socket transfer completed; current summary not sampled yet.
    Before,
    /// Current summary sampled; later control cannot retroactively revise it.
    After,
}

/// Actual setup either owns a socket or records why none was admitted.
enum SetupOutcome {
    /// Actual guarded refusal before any socket/registration/send for this candidate.
    PolicyRefused(AddressPolicyRefusalScope),
    /// The sole actual bind/registration succeeded under the current guard.
    /// One bounded box avoids storing the large live socket state in a refusal.
    Ready(Box<LiveSlot>),
    /// Control or time prevented any socket setup for this claimed candidate.
    Retired(AddressRoundEnd),
}

/// Set up the actual sole native socket, without an experimental mapping declaration.
/// # Errors
/// Preserve original bind/nonblocking/local-address/registration failures.
fn setup_address_request(
    peer: SocketAddr,
    packet: [u8; PACKET_BYTES],
    context: Arc<RunClockContext>,
    control: &mut watch::Receiver<Control>,
    request_end: Instant,
    round_end: Instant,
) -> Result<SetupOutcome, PilotError> {
    setup_address_request_bound(
        peer,
        packet,
        context,
        control,
        request_end,
        round_end,
        SourceMode::UnmanagedLab,
    )
}

/// Derive one original candidate's issued custody; missing bindings never downgrade.
/// # Errors
/// Preserve canonical-key or foreign-batch refusal.
fn candidate_mode(
    bindings: Option<&RoundBindings>,
    peer: SocketAddr,
) -> Result<SourceMode, PilotError> {
    bindings
        .map_or(Ok(SourceMode::UnmanagedLab), |value| value.mode(peer))
        .map_err(PilotError::SourcePolicy)
}

/// Shared native setup with actual source custody and current pre-socket refusal.
/// # Errors
/// Preserve original policy and native setup causes; no permission survives this check.
fn setup_address_request_bound(
    peer: SocketAddr,
    packet: [u8; PACKET_BYTES],
    context: Arc<RunClockContext>,
    control: &mut watch::Receiver<Control>,
    request_end: Instant,
    round_end: Instant,
    mode: SourceMode,
) -> Result<SetupOutcome, PilotError> {
    // Serialize the current guard with actual synchronous bind/registration.
    // Never invoke an observer, arbitrary setup callback or await under this lock.
    let state = control.borrow_and_update();
    let now = Instant::now();
    if let Some(end) = end_at(*state, now, round_end) {
        return Ok(SetupOutcome::Retired(end));
    }
    if now >= request_end {
        return Ok(SetupOutcome::Retired(AddressRoundEnd::SetupDeadline));
    }
    if let Some(scope) = mode
        .pre_socket_refusal()
        .map_err(PilotError::SourcePolicy)?
    {
        return Ok(SetupOutcome::PolicyRefused(scope));
    }
    let mut request = prepare_slot(peer, packet)?;
    request.source_policy = mode;
    let slot = LiveSlot::from_prepared(request, context)?;
    drop(state);
    Ok(SetupOutcome::Ready(Box::new(slot)))
}

/// One immutable original candidate and its independently retained cutoffs.
struct CandidateSetup {
    /// Original checked destination; no alternate peer is admitted here.
    peer: SocketAddr,
    /// Original opaque correlation packet for this candidate.
    packet: [u8; PACKET_BYTES],
    /// Genuine shared round origin and allocation identity.
    context: Arc<RunClockContext>,
    /// Actual immutable source/endpoint custody, never cached permission.
    mode: SourceMode,
    /// Once-captured candidate monotonic cutoff.
    request_end: Instant,
    /// Original immutable whole-round monotonic cutoff.
    round_end: Instant,
}

/// Construct one immutable setup from the exact cursor's original candidate.
/// # Errors
/// Reject a missing packet or binding before any socket work; retain original cutoffs.
fn checked_candidate_setup(
    prepared: &PreparedAddressRound,
    claim: &progress::Claim<'_, '_>,
    context: &Arc<RunClockContext>,
    deadline: Instant,
) -> Result<CandidateSetup, PilotError> {
    let (index, peer, request_end) = claim.parts();
    Ok(CandidateSetup {
        peer,
        packet: prepared
            .packets
            .get(usize::from(index))
            .copied()
            .ok_or(PilotError::InternalState)?,
        context: Arc::clone(context),
        mode: candidate_mode(prepared.source_policy.as_ref(), peer)?,
        request_end,
        round_end: deadline,
    })
}

/// Retry only genuine custody contention within the original candidate budget.
/// # Errors
/// Preserve original setup/custody errors and the finite hint-budget failure.
async fn setup_candidate<F>(
    plan: &CandidateSetup,
    control: &mut watch::Receiver<Control>,
    setup: &mut F,
) -> Result<SetupOutcome, PilotError>
where
    F: FnMut(
        SocketAddr,
        [u8; PACKET_BYTES],
        Arc<RunClockContext>,
        &mut watch::Receiver<Control>,
        Instant,
        Instant,
    ) -> Result<SetupOutcome, PilotError>,
{
    let mut contention_wakes = 0_u8;
    loop {
        if let Some(end) = current_end(control, plan.round_end) {
            return Ok(SetupOutcome::Retired(end));
        }
        match setup(
            plan.peer,
            plan.packet,
            Arc::clone(&plan.context),
            control,
            plan.request_end,
            plan.round_end,
        ) {
            Err(PilotError::SourcePolicy(crate::SourcePolicyError::Contended))
                if matches!(&plan.mode, SourceMode::Managed(_)) => {}
            Ok(SetupOutcome::Ready(slot)) if !slot.policy.belongs_to(&plan.mode) => {
                return Err(PilotError::SourcePolicy(crate::SourcePolicyError::Foreign));
            }
            value => return value,
        }
        if contention_wakes == 64 {
            return Err(PilotError::PolicyAdmissionBudget);
        }
        contention_wakes += 1;
        let retry = tokio::select! { biased;
            changed = control.changed() => {
                if changed.is_err() { return Err(PilotError::InternalState); }
                continue;
            },
            () = sleep_until(TokioInstant::from_std(plan.request_end)) => {
                return Ok(SetupOutcome::Retired(current_end(control, plan.round_end).unwrap_or(AddressRoundEnd::SetupDeadline)));
            },
            result = plan.mode.retry_after_contention() => result,
        };
        match retry {
            Ok(()) | Err(crate::SourcePolicyError::Disabled) => {}
            Err(error) => return Err(PilotError::SourcePolicy(error)),
        }
        // Fresh Disabled is not scoped: only the actual guarded setup above can
        // classify it. Never infer source/endpoint or renew either cutoff here.
        tokio::task::yield_now().await;
    }
}

/// Interpret only the captured terminal facts, without fresh policy inference.
/// # Errors
/// Preserve typed live policy errors or an impossible unmanaged policy result.
fn terminal_round_end(
    status: Status,
    cause: Option<crate::SourcePolicyError>,
    managed: bool,
) -> Result<Option<AddressRoundEnd>, PilotError> {
    Ok(match status {
        Status::SourcePolicy => match (managed, cause) {
            (true, Some(crate::SourcePolicyError::Disabled)) => Some(AddressRoundEnd::Policy),
            (true, Some(error)) => return Err(PilotError::SourcePolicy(error)),
            _ => return Err(PilotError::InternalState),
        },
        Status::Header => Some(AddressRoundEnd::Reply),
        Status::Control => Some(AddressRoundEnd::Control),
        Status::Stopped => Some(AddressRoundEnd::Stopped),
        Status::Stale => Some(AddressRoundEnd::Stale),
        Status::Timeout | Status::Budget | Status::Io => None,
    })
}

/// Retain one genuine pre-socket fact before completing its exact cursor claim.
/// The fresh summary observes current control without rewriting that history.
/// # Errors
/// Preserve a cursor contradiction; it cannot admit a successor or fake native completion.
fn record_policy_refusal<O>(
    report: &mut AddressRoundReport,
    claim: progress::Claim<'_, '_>,
    scope: AddressPolicyRefusalScope,
    control: &mut watch::Receiver<Control>,
    deadline: Instant,
    observe: &mut O,
) -> Result<Option<AddressRoundEnd>, PilotError>
where
    O: FnMut(u8, RoundCheckpoint),
{
    let (index, _, _) = claim.parts();
    report
        .policy_refusals
        .push(AddressPolicyRefusal { index, scope });
    claim.finish(index)?;
    observe(index, RoundCheckpoint::Before);
    let end = current_end(control, deadline);
    observe(index, RoundCheckpoint::After);
    Ok(end
        .or((scope == AddressPolicyRefusalScope::Source)
            .then_some(AddressRoundEnd::SourceRestricted)))
}

/// Transfer actual socket retirement and its historical terminal facts exactly once.
/// Cursor acknowledgement and fresh summary follow the real report/drop transfer.
/// # Errors
/// Preserve report/cursor contradictions and the captured original policy cause.
fn transfer_candidate_report<O>(
    slot: LiveSlot,
    claim: progress::Claim<'_, '_>,
    report: &mut AddressRoundReport,
    control: &mut watch::Receiver<Control>,
    plan: &CandidateSetup,
    observe: &mut O,
) -> Result<Option<AddressRoundEnd>, PilotError>
where
    O: FnMut(u8, RoundCheckpoint),
{
    let (index, peer, _) = claim.parts();
    let completed = slot.into_report(index)?;
    let status = completed.status();
    let policy_cause = completed.source_policy_refusal();
    report.attempts.push(AddressAttempt {
        peer,
        report: completed,
    });
    claim.finish(index)?;
    observe(index, RoundCheckpoint::Before);
    let end = current_end(control, plan.round_end);
    observe(index, RoundCheckpoint::After);
    if end.is_some() {
        return Ok(end);
    }
    terminal_round_end(
        status,
        policy_cause,
        matches!(&plan.mode, SourceMode::Managed(_)),
    )
}

/// Actual driver with private causal setup/notification seams; public execution
/// always supplies the unchanged native setup and an inert notification callback.
/// Observers cannot select summaries or manufacture a transferred completion.
async fn run_round_with<F, O>(
    mut prepared: PreparedAddressRound,
    origin: Instant,
    deadline: Instant,
    setup: &mut F,
    observe: &mut O,
) -> AddressRoundReport
where
    F: FnMut(
        SocketAddr,
        [u8; PACKET_BYTES],
        Arc<RunClockContext>,
        &mut watch::Receiver<Control>,
        Instant,
        Instant,
    ) -> Result<SetupOutcome, PilotError>,
    O: FnMut(u8, RoundCheckpoint),
{
    let mut report = AddressRoundReport::empty();
    let Ok(limits) = FrameLimits::new(PACKET_BYTES, 0) else {
        return report.fail(PilotError::WireSetup);
    };
    let context = Arc::new(RunClockContext { origin });
    let mut progress = Progress::new(&prepared.config, deadline);
    loop {
        if let Some(end) = current_end(&mut prepared.control, deadline) {
            report.end = end;
            return report;
        }
        let claim = match progress.claim_next(Instant::now()) {
            Ok(Some(claim)) => claim,
            Ok(None) => {
                report.end = current_end(&mut prepared.control, deadline)
                    .unwrap_or(AddressRoundEnd::Exhausted);
                return report;
            }
            Err(error) => return report.fail(error),
        };
        let plan = match checked_candidate_setup(&prepared, &claim, &context, deadline) {
            Ok(plan) => plan,
            Err(error) => return report.fail(error),
        };
        let outcome = setup_candidate(&plan, &mut prepared.control, setup).await;
        let slot = match outcome {
            Ok(SetupOutcome::Ready(slot)) => *slot,
            Ok(SetupOutcome::PolicyRefused(scope)) => {
                match record_policy_refusal(
                    &mut report,
                    claim,
                    scope,
                    &mut prepared.control,
                    deadline,
                    observe,
                ) {
                    Ok(Some(end)) => {
                        report.end = end;
                        return report;
                    }
                    Ok(None) => {}
                    Err(error) => return report.fail(error),
                }
                continue;
            }
            Ok(SetupOutcome::Retired(end)) => {
                report.end = end;
                return report;
            }
            Err(error) => return report.fail(error),
        };
        let slot = run_attempt(slot, &mut prepared.control, plan.request_end, limits).await;
        match transfer_candidate_report(
            slot,
            claim,
            &mut report,
            &mut prepared.control,
            &plan,
            observe,
        ) {
            Ok(Some(end)) => {
                report.end = end;
                return report;
            }
            Ok(None) => {}
            Err(error) => return report.fail(error),
        }
    }
}

/// Preserve original failure plus both actual timer-cleanup outcomes.
fn reconcile_cleanup(
    mut report: AddressRoundReport,
    stop: Result<(), PilotError>,
    invalidate: Result<(), PilotError>,
) -> AddressRoundReport {
    report.stop_cleanup = stop.err().map(FailureSummary::capture);
    report.invalidate_cleanup = invalidate.err().map(FailureSummary::capture);
    if report.stop_cleanup.is_some() || report.invalidate_cleanup.is_some() {
        report.end = AddressRoundEnd::Failure;
    }
    report
}

/// Own at most two control timers and reap both on every setup/operation exit.
async fn run_owned(
    prepared: PreparedAddressRound,
    origin: Instant,
    deadline: Instant,
    stop_when: Option<Instant>,
    invalidate_when: Option<Instant>,
) -> AddressRoundReport {
    let handle = prepared.handle.clone();
    let stop = match start_timer(stop_when, handle.clone(), TimerAction::Stop) {
        Ok(job) => job,
        Err(error) => return AddressRoundReport::empty().fail(error),
    };
    let (report, invalidate_cleanup) =
        match start_timer(invalidate_when, handle, TimerAction::Invalidate) {
            Ok(invalidate) => {
                let report = run_round(prepared, origin, deadline).await;
                (report, finish_timer(invalidate).await)
            }
            Err(error) => (AddressRoundReport::empty().fail(error), Ok(())),
        };
    let stop_cleanup = finish_timer(stop).await;
    reconcile_cleanup(report, stop_cleanup, invalidate_cleanup)
}

/// Execute one permitted frozen round in a privately owned current-thread runtime.
///
/// No automatic new round, production backoff reset or sample admission follows.
/// Original active time includes runtime setup. At most one socket and two sleep
/// tasks are owned; all Rust owners end before return. This makes no native AFD
/// completion/retention claim. Historical results survive later cancellation.
/// # Errors
/// Reject entered context or deadline/runtime setup before driving acquisition.
/// Later setup/invariant and cleanup failures are retained in the returned report.
pub fn execute_address_round(
    prepared: PreparedAddressRound,
    schedule: &ControlSchedule,
) -> Result<AddressRoundReport, PilotError> {
    if tokio::runtime::Handle::try_current().is_ok() {
        return Err(PilotError::EnteredRuntime);
    }
    let origin = Instant::now();
    let deadline = origin
        .checked_add(prepared.config.round_timeout)
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
    let report = runtime.block_on(run_owned(
        prepared,
        origin,
        deadline,
        stop_when,
        invalidate_when,
    ));
    drop(runtime);
    Ok(report)
}

#[cfg(test)]
#[path = "../tests/private/address_round.rs"]
mod tests;

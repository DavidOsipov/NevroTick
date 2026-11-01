// SPDX-License-Identifier: GPL-2.0-only
//! Controlled loopback transport experiment; native qualification is incomplete.
//! Exactly two distinct controlled peers; one query per association; no retries.
//! `prepare()` performs synchronous OS entropy and bind work BEFORE active control.
//! It has no bounded RNG/startup-stop claim. The private async owner is driven
//! by one current-thread runtime with I/O/time enabled and `max_blocking_threads(2)`.
//! Public `execute()` owns that runtime for the whole operation and returns only
//! after its maximum two sleep tasks are aborted/reaped and the runtime is dropped.
//! Instants/SystemTimes below are local software observations, not kernel arrival,
//! clock-domain mapping, era resolution, sample eligibility, authentication or UTC truth.
#![forbid(unsafe_code)]

use ntp_wire::{FrameLimits, frame_no_legacy_mac_datagram};
use std::{
    io,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket as StdSocket},
    pin::Pin,
    time::{Duration, Instant, SystemTime},
};
use tokio::{
    net::UdpSocket,
    sync::watch,
    time::{Instant as TokioInstant, Sleep, sleep_until},
};

/// Declared laboratory ceilings; no production timeout or resource policy is adopted.
const MAX_TIMEOUT: Duration = Duration::from_secs(30);
/// Maximum selected datagrams per association; exhaustion retains pending state until retirement.
const RECEIVE_CAP: u16 = 64;
/// Maximum readiness-driven attempts before a full send; no protocol retransmission is allowed.
const SEND_READY_CAP: u16 = 64;
/// Complete minimal `NTPv4` laboratory header size.
const PACKET_BYTES: usize = 48;
/// One extra byte distinguishes every oversized prefix from an exact valid header.
const RECEIVE_BYTES: usize = PACKET_BYTES + 1;

/// Safe, fixed-size native error information; no localized message is retained.
#[derive(Clone, Copy, Debug)]
/// Bounded native code family; it never retains formatted error text.
pub struct IoCause {
    /// Portable native-error category.
    kind: io::ErrorKind,
    /// Optional platform code captured from the actual operation.
    native: Option<i32>,
}
impl IoCause {
    /// Capture only the bounded category and native code from this actual error.
    fn capture(error: &io::Error) -> Self {
        Self {
            kind: error.kind(),
            native: error.raw_os_error(),
        }
    }
    /// Return the portable native-error category.
    #[must_use]
    pub fn kind(self) -> io::ErrorKind {
        self.kind
    }
    /// Return the captured bounded platform code, if present.
    #[must_use]
    pub fn native_code(self) -> Option<i32> {
        self.native
    }
}

/// Bounded safe errors for the diagnostic entrypoint.
#[derive(Clone, Copy, Debug)]
pub enum PilotError {
    /// Endpoint is outside the exact loopback/nonzero-port laboratory contract.
    InvalidPeer,
    /// Two configured peers identify the same endpoint.
    DuplicatePeer,
    /// Declared lifetime is zero or above the laboratory maximum.
    InvalidTimeout,
    /// The OS entropy request failed; its buffer is never used.
    Entropy,
    /// An all-zero OS-generated token is rejected without regeneration.
    ZeroToken,
    /// Equal tokens for the two associations are rejected without regeneration.
    DuplicateToken,
    /// Native loopback bind failed with a bounded actual cause.
    Bind(IoCause),
    /// Setting the actual socket nonblocking failed.
    Nonblocking(IoCause),
    /// Reading the actual bound address failed.
    LocalAddress(IoCause),
    /// Registering the owned nonblocking socket failed.
    Registration(IoCause),
    /// The two actual local endpoint tuples were unexpectedly equal.
    DuplicateLocalAddress,
    /// Checked monotonic deadline construction failed.
    DeadlineOverflow,
    /// Control generation increment would wrap; stop was latched.
    GenerationExhausted,
    /// Internal one-way state could not produce its required terminal report.
    InternalState,
    /// A supposedly fixed pure parser setup failed before any send.
    WireSetup,
    /// `execute()` or `prepare()` was called from an already entered Tokio runtime.
    EnteredRuntime,
    /// Building the one private current-thread runtime failed.
    RuntimeBuild(IoCause),
    /// An owned timer task panicked; no panic text is retained.
    ControlTask,
    /// Timer cleanup failed; both tasks were still aborted and reaped.
    ControlCleanup {
        /// Optional primary pilot error retained as a safe fixed summary.
        primary: Option<FailureSummary>,
        /// Optional stop-task error retained independently.
        stop: Option<FailureSummary>,
        /// Optional invalidation-task error retained independently.
        invalidate: Option<FailureSummary>,
    },
}

impl PilotError {
    /// Return a fixed bounded diagnostic code without payload formatting.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InvalidPeer => "invalid_peer",
            Self::DuplicatePeer => "duplicate_peer",
            Self::InvalidTimeout => "invalid_timeout",
            Self::Entropy => "entropy_failure",
            Self::ZeroToken => "zero_random_token",
            Self::DuplicateToken => "duplicate_random_token",
            Self::Bind(_) => "bind_io",
            Self::Nonblocking(_) => "nonblocking_io",
            Self::LocalAddress(_) => "local_address_io",
            Self::Registration(_) => "registration_io",
            Self::DuplicateLocalAddress => "duplicate_local_address",
            Self::DeadlineOverflow => "deadline_overflow",
            Self::GenerationExhausted => "generation_exhausted",
            Self::InternalState => "internal_state",
            Self::WireSetup => "wire_setup",
            Self::EnteredRuntime => "entered_runtime",
            Self::RuntimeBuild(_) => "runtime_build",
            Self::ControlTask => "control_task",
            Self::ControlCleanup { .. } => "control_cleanup",
        }
    }
}
impl std::fmt::Display for PilotError {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        out.write_str(self.as_str())
    }
}
impl std::error::Error for PilotError {}

/// Private checked fixed plan; there is no mutable/default/Serde bypass.
/// Checked fixed laboratory plan; CLI/settings decoding remains its own boundary.
pub struct Config {
    /// Two distinct, nonzero, explicitly controlled loopback peers.
    peers: [SocketAddr; 2],
    /// Positive laboratory lifetime, at most `MAX_TIMEOUT`.
    timeout: Duration,
}
impl Config {
    /// Validate all raw values against this fixed laboratory contract.
    ///
    /// # Errors
    /// Reject endpoint/time inputs outside the documented private type invariants.
    ///
    /// # Examples
    /// Construction checks the laboratory plan without reading entropy or binding sockets.
    /// The caller must provide two different loopback destinations and a positive timeout.
    ///
    /// ```
    /// use std::net::SocketAddr;
    /// use std::time::Duration;
    /// use nevrotick_engine::{Config, PilotError};
    ///
    /// let peers = [
    ///     SocketAddr::from(([127, 0, 0, 1], 41001)),
    ///     SocketAddr::from(([127, 0, 0, 1], 41002)),
    /// ];
    /// assert!(Config::new(peers, Duration::from_millis(250)).is_ok());
    /// assert!(matches!(
    ///     Config::new([peers[0], peers[0]], Duration::from_millis(250)),
    ///     Err(PilotError::DuplicatePeer)
    /// ));
    /// assert!(matches!(
    ///     Config::new(peers, Duration::ZERO),
    ///     Err(PilotError::InvalidTimeout)
    /// ));
    /// ```
    pub fn new(peers: [SocketAddr; 2], timeout: Duration) -> Result<Self, PilotError> {
        for peer in peers {
            if !peer.ip().is_loopback() || peer.port() == 0 {
                return Err(PilotError::InvalidPeer);
            }
            if let SocketAddr::V6(address) = peer
                && (address.scope_id() != 0 || address.flowinfo() != 0)
            {
                return Err(PilotError::InvalidPeer);
            }
        }
        if peers[0] == peers[1] {
            return Err(PilotError::DuplicatePeer);
        }
        if timeout.is_zero() || timeout > MAX_TIMEOUT {
            return Err(PilotError::InvalidTimeout);
        }
        Ok(Self { peers, timeout })
    }
}

/// Pilot-local aggregate generation. Production config/association/clock owners
/// must retain their separate generations; this one-run invalidation is no substitute.
#[derive(Clone, Copy)]
/// One coalesced pilot-local control snapshot.
struct Control {
    /// Latched request; no clone can change true back to false.
    stop: bool,
    /// Checked aggregate invalidation generation; initial value is zero.
    epoch: u64,
}

/// One bounded shared watch value; clones cannot undo a requested stop.
#[derive(Clone)]
/// Cloneable bounded requests; sending is delivery, not owner acknowledgement.
pub struct ControlHandle {
    /// Channel for this exact prepared pilot only.
    sender: watch::Sender<Control>,
}
impl ControlHandle {
    /// Deliver a latched stop request; this is not owner admission or native-cleanup acknowledgement.
    pub fn stop(&self) {
        self.sender.send_modify(|value| value.stop = true);
    }
    /// Deliver one checked generation change; delivery is not owner acknowledgement.
    ///
    /// # Errors
    /// Latch stop and reject if the aggregate generation would wrap.
    pub fn invalidate(&self) -> Result<(), PilotError> {
        let mut result = Ok(());
        self.sender.send_modify(|value| {
            if value.stop {
                return;
            }
            if let Some(next) = value.epoch.checked_add(1) {
                value.epoch = next;
            } else {
                value.stop = true;
                result = Err(PilotError::GenerationExhausted);
            }
        });
        result
    }
}

/// One uniquely owned nonblocking socket and its one request.
struct PreparedSlot {
    /// Actual OS-bound socket; not cloned or shared.
    socket: StdSocket,
    /// The exact checked peer endpoint.
    peer: SocketAddr,
    /// Actual ephemeral local endpoint reported by the owned socket.
    local: SocketAddr,
    /// The sole wire request, containing OS-generated opaque correlation bits.
    packet: [u8; PACKET_BYTES],
}
/// Opaque preparation without caller-manufactured native facts.
pub struct PreparedPilot {
    /// Exactly two prepared associations.
    slots: [PreparedSlot; 2],
    /// Declared whole active-operation laboratory lifetime.
    timeout: Duration,
    /// Sole owner-side control stream.
    control: watch::Receiver<Control>,
    /// An internal sender keeps optional external-handle drop harmless.
    control_handle: ControlHandle,
}

/// Call once per executable run, before the active-control/elapsed window.
/// No socket or packet is published if entropy fails or either token is zero/equal.
/// No failed entropy buffer is used, and no token-generation retry occurs.
///
/// # Errors
/// Reject an entered Tokio runtime before any entropy or socket operation.
/// Return a closed entropy/token error or the bounded actual socket setup cause.
/// Partial socket construction drops its owned resources before returning.
pub fn prepare(config: &Config) -> Result<(PreparedPilot, ControlHandle), PilotError> {
    if tokio::runtime::Handle::try_current().is_ok() {
        return Err(PilotError::EnteredRuntime);
    }
    let mut entropy = [0_u8; 16];
    getrandom::fill(&mut entropy).map_err(|_| PilotError::Entropy)?;
    let (left, right) = entropy.split_at(8);
    if left == [0_u8; 8] || right == [0_u8; 8] {
        return Err(PilotError::ZeroToken);
    }
    if left == right {
        return Err(PilotError::DuplicateToken);
    }
    let mut first = [0_u8; PACKET_BYTES];
    let mut second = first;
    first[0] = 0x23;
    second[0] = 0x23; // LI=0, VN=4, client mode=3.
    first[40..48].copy_from_slice(left);
    second[40..48].copy_from_slice(right);
    let a = prepare_slot(config.peers[0], first)?;
    let b = prepare_slot(config.peers[1], second)?;
    if a.local == b.local {
        return Err(PilotError::DuplicateLocalAddress);
    }
    let (sender, control) = watch::channel(Control {
        stop: false,
        epoch: 0,
    });
    let control_handle = ControlHandle { sender };
    Ok((
        PreparedPilot {
            slots: [a, b],
            timeout: config.timeout,
            control,
            control_handle: control_handle.clone(),
        },
        control_handle,
    ))
}

/// Own one real nonblocking loopback socket; partial construction drops its resources.
///
/// # Errors
/// Return bounded bind/nonblocking/local-address failures without a query send.
fn prepare_slot(peer: SocketAddr, packet: [u8; PACKET_BYTES]) -> Result<PreparedSlot, PilotError> {
    let bind_ip = match peer {
        SocketAddr::V4(_) => IpAddr::V4(Ipv4Addr::LOCALHOST),
        SocketAddr::V6(_) => IpAddr::V6(Ipv6Addr::LOCALHOST),
    };
    let socket = StdSocket::bind(SocketAddr::new(bind_ip, 0))
        .map_err(|e| PilotError::Bind(IoCause::capture(&e)))?;
    socket
        .set_nonblocking(true)
        .map_err(|e| PilotError::Nonblocking(IoCause::capture(&e)))?;
    let local = socket
        .local_addr()
        .map_err(|e| PilotError::LocalAddress(IoCause::capture(&e)))?;
    if !local.ip().is_loopback() || local.port() == 0 {
        return Err(PilotError::InvalidPeer);
    }
    Ok(PreparedSlot {
        socket,
        peer,
        local,
        packet,
    })
}

/// One historical terminal association outcome, without current confidence,
/// sample eligibility or authentication authority. Later stop/invalidation
/// retires only unfinished slots and does not rewrite a completed outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    /// One raw ordinary header was accepted; this is not a time sample.
    Header,
    /// One correlated stratum-zero message ended the query as opaque control.
    Control,
    /// The declared exclusive processing deadline was reached.
    Timeout,
    /// The serialized owner admitted a stop request.
    Stopped,
    /// The serialized owner admitted a changed pilot generation.
    Stale,
    /// A laboratory work cap was exhausted; no extra query was sent.
    Budget,
    /// Native I/O failed or returned an anomalous short send.
    Io,
}

impl Status {
    /// Return a fixed bounded diagnostic code without payload formatting.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Header => "header",
            Self::Control => "control",
            Self::Timeout => "timeout",
            Self::Stopped => "stopped",
            Self::Stale => "stale",
            Self::Budget => "budget",
            Self::Io => "io",
        }
    }
}

/// Adjacent local software reads; neither field is a wire/kernel timestamp.
#[derive(Clone, Copy)]
/// Two adjacent local software readings, without a UTC/monotonic mapping claim.
pub struct SoftwareClock {
    /// Local monotonic reading taken first.
    mono: Instant,
    /// Immediately following system-wall reading; no accuracy assertion.
    utc: SystemTime,
}
impl SoftwareClock {
    /// Capture adjacent local monotonic and wall readings without a mapping claim.
    fn capture() -> Self {
        Self {
            mono: Instant::now(),
            utc: SystemTime::now(),
        }
    }
    /// Return the captured local monotonic observation.
    #[must_use]
    pub fn monotonic(self) -> Instant {
        self.mono
    }
    /// Return the adjacent system-wall observation without an accuracy claim.
    #[must_use]
    pub fn utc(self) -> SystemTime {
        self.utc
    }
}

/// Native-path observations only. Raw peer bits are not unfolded or used in arithmetic.
/// Read-only native-path observations; peer stamps remain unqualified raw bits.
pub struct TransportFacts {
    /// Local readings immediately before the successful `try_send_to` call.
    send_before: SoftwareClock,
    /// Local readings immediately after that synchronous call returned.
    send_after: SoftwareClock,
    /// Local readings when the selected receive future returned.
    read_return: SoftwareClock,
    /// Fresh exclusive-owner reading immediately before result commit.
    processing: Instant,
    /// Exact nonzero peer T2 representation without era interpretation.
    peer_receive_bits: u64,
    /// Exact nonzero peer T3 representation without era interpretation.
    peer_transmit_bits: u64,
}
impl TransportFacts {
    /// Return the local readings before the confirmed native send.
    #[must_use]
    pub fn send_before(&self) -> SoftwareClock {
        self.send_before
    }
    /// Return the local readings after that native send returned.
    #[must_use]
    pub fn send_after(&self) -> SoftwareClock {
        self.send_after
    }
    /// Return the actual selected receive-return readings.
    #[must_use]
    pub fn read_return(&self) -> SoftwareClock {
        self.read_return
    }
    /// Return the fresh exclusive-owner commit reading.
    #[must_use]
    pub fn processing(&self) -> Instant {
        self.processing
    }
    /// Return the exact peer T2 bits without era/eligibility interpretation.
    #[must_use]
    pub fn peer_receive_bits(&self) -> u64 {
        self.peer_receive_bits
    }
    /// Return the exact peer T3 bits without era/eligibility interpretation.
    #[must_use]
    pub fn peer_transmit_bits(&self) -> u64 {
        self.peer_transmit_bits
    }
}

/// Bounded historical per-association output; no packet/token formatting route.
/// A completed result remains historical after later stop/invalidation; it
/// confers no current confidence, sample eligibility or authentication authority.
pub struct SlotReport {
    /// Zero-based association index, exactly zero or one.
    slot: u8,
    /// Actual bound local endpoint.
    local: SocketAddr,
    /// Single terminal owner outcome.
    status: Status,
    /// Whether a complete 48-byte native send was confirmed.
    sent: bool,
    /// Selected actual datagram results, including native oversize errors, at most 64.
    received: u16,
    /// Selected datagrams not committed as header/control; at most received.
    rejected: u16,
    /// Present only for a completed ordinary header.
    stratum: Option<u8>,
    /// Present only for a completed ordinary header.
    facts: Option<TransportFacts>,
    /// Opaque reference-ID bits only for a correlated stratum-zero control.
    control_code: Option<u32>,
    /// Bounded actual I/O cause when an operation ended the slot.
    io_cause: Option<IoCause>,
    /// Most recent protocol/parser failure, without retained packet contents.
    last_rejection: Option<Rejection>,
}
impl SlotReport {
    /// Return the fixed zero-or-one association index.
    #[must_use]
    pub fn slot(&self) -> u8 {
        self.slot
    }
    /// Return the actual captured ephemeral local address.
    #[must_use]
    pub fn local_endpoint(&self) -> SocketAddr {
        self.local
    }
    /// Return this association's historical terminal owner status, not current confidence.
    #[must_use]
    pub fn status(&self) -> Status {
        self.status
    }
    /// Report whether one complete 48-byte native send was confirmed.
    #[must_use]
    pub fn sent(&self) -> bool {
        self.sent
    }
    /// Return the bounded number of selected actual datagram outcomes.
    #[must_use]
    pub fn received(&self) -> u16 {
        self.received
    }
    /// Return the bounded number of selected datagrams not committed.
    #[must_use]
    pub fn rejected(&self) -> u16 {
        self.rejected
    }
    /// Return a stratum only for a completed ordinary raw header.
    #[must_use]
    pub fn stratum(&self) -> Option<u8> {
        self.stratum
    }
    /// Borrow committed native-path facts, without a sample/trust conversion.
    #[must_use]
    pub fn facts(&self) -> Option<&TransportFacts> {
        self.facts.as_ref()
    }
    /// Return opaque reference bits only for a completed stratum-zero control.
    #[must_use]
    pub fn control_code(&self) -> Option<u32> {
        self.control_code
    }
    /// Return the bounded actual runtime I/O cause, if one ended this slot.
    #[must_use]
    pub fn io_cause(&self) -> Option<IoCause> {
        self.io_cause
    }
    /// Return the most recent protocol/parser rejection, if that stage was reached.
    ///
    /// Lifecycle guards may reject selected data without replacing this value.
    #[must_use]
    pub fn last_rejection(&self) -> Option<Rejection> {
        self.last_rejection
    }
}

/// Two historical terminal reports after Rust socket ownership was dropped.
/// Later stop/invalidation retires unfinished slots only; an earlier completed
/// result remains a historical fact and confers no current confidence.
pub struct PilotReport {
    /// Exactly two bounded terminal association reports.
    slots: [SlotReport; 2],
    /// Elapsed active operation through runtime/timer cleanup; excludes `prepare()`.
    done_elapsed_ns: u128,
    /// Rust socket drops returned; not native AFD completion evidence.
    sockets_drop_returned: Instant,
}
impl PilotReport {
    /// Borrow the two historical terminal reports; neither grants current confidence.
    #[must_use]
    pub fn slots(&self) -> &[SlotReport; 2] {
        &self.slots
    }
    /// Return active elapsed time including owned runtime/timer cleanup, excluding `prepare()`.
    #[must_use]
    pub fn done_elapsed_ns(&self) -> u128 {
        self.done_elapsed_ns
    }
    /// Rust ownership was dropped. This is NOT native AFD completion/release evidence.
    /// Return when Rust socket ownership drops returned; this is not native AFD completion.
    #[must_use]
    pub fn socket_drop_returned(&self) -> Instant {
        self.sockets_drop_returned
    }
}

#[derive(Clone, Copy)]
/// Local software bracket around one confirmed full native send.
struct SendBracket {
    /// Taken immediately before `try_send_to`.
    before: SoftwareClock,
    /// Taken immediately after its result.
    after: SoftwareClock,
}
/// Private one-way query lifecycle; no terminal slot can be rearmed.
#[derive(Clone, Copy)]
enum Phase {
    /// Waiting for writable readiness before the sole actual send.
    Sending,
    /// Full send confirmed; one matching result may still complete.
    Pending(SendBracket),
    /// Terminal for this pilot; capacity is not reused for another query.
    Done(Status),
}

/// Exclusive runtime owner; no rearm or socket-cloning API.
struct LiveSlot {
    /// Sole registered native socket owner.
    socket: UdpSocket,
    /// Immutable exact response peer.
    peer: SocketAddr,
    /// Captured actual local endpoint.
    local: SocketAddr,
    /// Immutable sole request; correlation stays private.
    packet: [u8; PACKET_BYTES],
    /// One-way sending/pending/terminal state.
    phase: Phase,
    /// A complete native send was confirmed.
    sent: bool,
    /// Bounded count of selected native datagram outcomes.
    received: u16,
    /// Bounded count of rejected actual datagrams.
    rejected: u16,
    /// At most `SEND_READY_CAP` readiness-driven `try_send_to` attempts.
    send_attempts: u16,
    /// Ordinary-header stratum after final commit.
    stratum: Option<u8>,
    /// Committed raw native observations, without sample eligibility.
    facts: Option<TransportFacts>,
    /// Committed opaque stratum-zero reference bits.
    control_code: Option<u32>,
    /// Actual native I/O code, without formatted messages.
    io_cause: Option<IoCause>,
    /// Latest bounded parser/protocol rejection; lifecycle rejection is separate.
    last_rejection: Option<Rejection>,
}
impl LiveSlot {
    /// Register one privately owned actual nonblocking socket without cloning it.
    ///
    /// # Errors
    /// Return the captured native registration error.
    fn from_prepared(value: PreparedSlot) -> Result<Self, PilotError> {
        let socket = UdpSocket::from_std(value.socket)
            .map_err(|e| PilotError::Registration(IoCause::capture(&e)))?;
        Ok(Self {
            socket,
            peer: value.peer,
            local: value.local,
            packet: value.packet,
            phase: Phase::Sending,
            sent: false,
            received: 0,
            rejected: 0,
            send_attempts: 0,
            stratum: None,
            facts: None,
            control_code: None,
            io_cause: None,
            last_rejection: None,
        })
    }
    /// Commit a terminal status once; a completed slot cannot be reopened or overwritten.
    fn finish(&mut self, status: Status) {
        if !self.done() {
            self.phase = Phase::Done(status);
        }
    }
    /// Report whether this one-query slot is terminal.
    fn done(&self) -> bool {
        matches!(self.phase, Phase::Done(_))
    }
    /// Report whether the sole request still waits for writable readiness.
    fn can_send(&self) -> bool {
        matches!(self.phase, Phase::Sending)
    }
    /// Report pending state and remaining declared receive-work budget.
    fn can_receive(&self) -> bool {
        matches!(self.phase, Phase::Pending(_)) && self.received < RECEIVE_CAP
    }
    /// Retire only from the trusted owner deadline; an exhausted read budget reports Budget.
    fn expire(&mut self) {
        self.finish(if self.received == RECEIVE_CAP {
            Status::Budget
        } else {
            Status::Timeout
        });
    }
    /// Consume a terminal owner and drop its unique Rust socket ownership.
    ///
    /// # Errors
    /// Reject a nonterminal internal state instead of manufacturing successful output.
    fn into_report(self, slot: u8) -> Result<SlotReport, PilotError> {
        let Phase::Done(status) = self.phase else {
            return Err(PilotError::InternalState);
        };
        let report = SlotReport {
            slot,
            local: self.local,
            status,
            sent: self.sent,
            received: self.received,
            rejected: self.rejected,
            stratum: self.stratum,
            facts: self.facts,
            control_code: self.control_code,
            io_cause: self.io_cause,
            last_rejection: self.last_rejection,
        };
        drop(self.socket);
        Ok(report)
    }
}

/// One selected await result; no packet bytes escape the owner buffer.
enum Wake {
    /// A watch change arrived, or its sender side closed.
    Control(bool),
    /// The declared common active-operation deadline became ready.
    Deadline,
    /// Readiness only; actual sending occurs after fresh owner guards.
    Writable {
        /// Fixed association index produced only by the two select branches.
        slot: usize,
        /// Actual readiness result.
        result: io::Result<()>,
    },
    /// A complete selected receive result and adjacent software readings.
    Read {
        /// Fixed association index produced only by the two select branches.
        slot: usize,
        /// Actual bytes copied and observed source, or the native receive failure.
        result: io::Result<(usize, SocketAddr)>,
        /// Captured only when this receive future returned ready.
        read: SoftwareClock,
    },
}

/// Copy the latest watch snapshot and release its borrow before any await/effect.
fn latest(receiver: &mut watch::Receiver<Control>) -> Control {
    *receiver.borrow_and_update()
}
/// Admit stop/generation requests serially; prior committed results remain unchanged.
fn retire_for_control(slots: &mut [LiveSlot; 2], state: Control) {
    if state.stop {
        for slot in slots {
            slot.finish(Status::Stopped);
        }
    } else if state.epoch != 0 {
        for slot in slots {
            slot.finish(Status::Stale);
        }
    }
}

/// Attempt one nonblocking native send after current owner guards; never repeat a confirmed/anomalous send.
fn attempt_send(slot: &mut LiveSlot, deadline: Instant) {
    if !slot.can_send() {
        return;
    }
    if slot.send_attempts == SEND_READY_CAP {
        slot.finish(Status::Budget);
        return;
    }
    slot.send_attempts += 1;
    let before = SoftwareClock::capture();
    if before.mono >= deadline {
        slot.expire();
        return;
    }
    let result = slot.socket.try_send_to(&slot.packet, slot.peer);
    let after = SoftwareClock::capture();
    match result {
        Ok(PACKET_BYTES) => {
            slot.sent = true;
            slot.phase = Phase::Pending(SendBracket { before, after });
        }
        Ok(_) => slot.finish(Status::Io), // No automatic repeat after an anomalous send.
        Err(error) if error.kind() == io::ErrorKind::WouldBlock => {} // Readiness re-arm, not a query retry.
        Err(error) => {
            slot.io_cause = Some(IoCause::capture(&error));
            slot.finish(Status::Io);
        }
    }
}

/// Classify only Winsock's explicit consumed oversized-datagram error; no partial bytes are parsed.
fn is_oversized(error: &io::Error) -> bool {
    // Winsock UDP receive consumes/discards the oversized datagram with 10040.
    // Never parse its partly written buffer. Native Windows observation remains required.
    cfg!(windows) && error.raw_os_error() == Some(10040)
}

/// Closed inert rejection reasons, never rejected packet contents.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Rejection {
    /// No sent request remains pending in this slot.
    NoPending,
    /// The complete datagram is outside the exact 48-byte lab profile.
    Length,
    /// Source address/port differs from the saved actual peer.
    Source,
    /// The pure complete-datagram parser rejected the input.
    Frame(ntp_wire::FrameError),
    /// The packet is not an `NTPv4` server-mode response.
    VersionOrMode,
    /// Origin bits differ from this slot's OS-generated transmitted token.
    Origin,
    /// Native software read-return time precedes the saved completed-send bracket.
    ReadBeforeSend,
    /// Ordinary header carries unsynchronized leap state or invalid stratum.
    Quality,
    /// An essential ordinary-header peer timestamp is zero.
    Timestamp,
}

/// Fully checked immutable packet facts; current control/deadline remain unproved
/// until the sole owner commits immediately after its final guards.
struct StagedReply {
    /// Actual saved send bracket captured by this owner.
    sent: SendBracket,
    /// Actual selected read-return observations.
    read: SoftwareClock,
    /// Zero is control; ordinary values are one through fifteen.
    stratum: u8,
    /// Opaque reference-ID bits; only controls expose this value.
    code: u32,
    /// Exact peer receive bits; zero is permitted only for control.
    receive_bits: u64,
    /// Exact peer transmit bits; zero is permitted only for control.
    transmit_bits: u64,
}

/// Stage a whole actual datagram with no owner mutation or external effect.
///
/// # Errors
/// Return a closed reason for invalid shape/source/token/order/ordinary fields.
fn stage_reply(
    slot: &LiveSlot,
    bytes: &[u8],
    source: SocketAddr,
    read: SoftwareClock,
    limits: FrameLimits,
) -> Result<StagedReply, Rejection> {
    let Phase::Pending(sent) = slot.phase else {
        return Err(Rejection::NoPending);
    };
    if bytes.len() != PACKET_BYTES {
        return Err(Rejection::Length);
    }
    if source != slot.peer {
        return Err(Rejection::Source);
    }
    let frame = frame_no_legacy_mac_datagram(bytes, limits).map_err(Rejection::Frame)?;
    let header = frame.header();
    if header.version() != 4 || header.mode() != 4 {
        return Err(Rejection::VersionOrMode);
    }
    if header.origin_timestamp().bits().to_be_bytes().as_slice() != &slot.packet[40..48] {
        return Err(Rejection::Origin);
    }
    if read.mono < sent.after.mono {
        return Err(Rejection::ReadBeforeSend);
    }
    let stratum = header.stratum();
    if stratum != 0 {
        if header.leap_indicator() == 3 || stratum > 15 {
            return Err(Rejection::Quality);
        }
        if header.receive_timestamp().bits() == 0 || header.transmit_timestamp().bits() == 0 {
            return Err(Rejection::Timestamp);
        }
    }
    Ok(StagedReply {
        sent,
        read,
        stratum,
        code: header.reference_id(),
        receive_bits: header.receive_timestamp().bits(),
        transmit_bits: header.transmit_timestamp().bits(),
    })
}

/// Admit current control and exclusive deadline retirement at one owner checkpoint.
/// Completed slots remain historical; no guard reopens or overwrites them.
fn refresh_owner(
    slots: &mut [LiveSlot; 2],
    control: &mut watch::Receiver<Control>,
    deadline: Instant,
) {
    retire_for_control(slots, latest(control));
    if Instant::now() >= deadline {
        for slot in slots {
            slot.expire();
        }
    }
}

/// Select one event from the same two slots and original bounded receive buffers.
/// Both receive futures are pinned and dropped before this helper returns. Control
/// and deadline stay first; the caller alternates slot priority after every wake.
async fn next_wake(
    slots: &[LiveSlot; 2],
    buffers: &mut [[u8; RECEIVE_BYTES]; 2],
    control: &mut watch::Receiver<Control>,
    control_open: bool,
    mut timer: Pin<&mut Sleep>,
    reverse: bool,
) -> Wake {
    let [a, b] = slots;
    let [ba, bb] = buffers;
    let (ra, rb, wa, wb) = (a.can_receive(), b.can_receive(), a.can_send(), b.can_send());
    let receive_a = a.socket.recv_from(ba);
    let receive_b = b.socket.recv_from(bb);
    tokio::pin!(receive_a, receive_b);
    if reverse {
        tokio::select! { biased;
            changed = control.changed(), if control_open => Wake::Control(changed.is_ok()),
            () = &mut timer => Wake::Deadline,
            result = &mut receive_b, if rb => Wake::Read { slot: 1, result, read: SoftwareClock::capture() },
            result = &mut receive_a, if ra => Wake::Read { slot: 0, result, read: SoftwareClock::capture() },
            result = b.socket.writable(), if wb => Wake::Writable { slot: 1, result },
            result = a.socket.writable(), if wa => Wake::Writable { slot: 0, result },
        }
    } else {
        tokio::select! { biased;
            changed = control.changed(), if control_open => Wake::Control(changed.is_ok()),
            () = &mut timer => Wake::Deadline,
            result = &mut receive_a, if ra => Wake::Read { slot: 0, result, read: SoftwareClock::capture() },
            result = &mut receive_b, if rb => Wake::Read { slot: 1, result, read: SoftwareClock::capture() },
            result = a.socket.writable(), if wa => Wake::Writable { slot: 0, result },
            result = b.socket.writable(), if wb => Wake::Writable { slot: 1, result },
        }
    }
}

/// One successful native read, borrowed from its original bounded owner buffer.
/// This private view retains actual source/read observations without copying bytes.
struct ReadDatagram<'a> {
    /// Entire returned byte slice; an oversized datagram may provide only a prefix.
    bytes: &'a [u8],
    /// Actual source address returned by that same native receive.
    source: SocketAddr,
    /// Adjacent software readings captured when that receive returned ready.
    read: SoftwareClock,
}

/// Commit checked packet facts only after the caller's fresh owner guards.
/// No await or external effect intervenes; an impossible local read order rejects
/// the datagram while preserving its pending slot and prior rejection cause.
fn commit_reply(slot: &mut LiveSlot, candidate: &StagedReply, processing: Instant) {
    if processing < candidate.read.mono {
        slot.rejected += 1;
        return;
    }
    if candidate.stratum == 0 {
        slot.control_code = Some(candidate.code);
        slot.finish(Status::Control);
    } else {
        slot.stratum = Some(candidate.stratum);
        slot.facts = Some(TransportFacts {
            send_before: candidate.sent.before,
            send_after: candidate.sent.after,
            read_return: candidate.read,
            processing,
            peer_receive_bits: candidate.receive_bits,
            peer_transmit_bits: candidate.transmit_bits,
        });
        slot.finish(Status::Header);
    }
}

/// Stage one selected receive slice for its fixed zero-or-one slot index.
/// Rejections retain their bounded actual cause. Fresh control and processing-time
/// guards immediately precede commit, with no await or caller-supplied timestamp.
fn process_reply(
    slots: &mut [LiveSlot; 2],
    control: &mut watch::Receiver<Control>,
    index: usize,
    datagram: &ReadDatagram<'_>,
    limits: FrameLimits,
    deadline: Instant,
) {
    let staged = stage_reply(
        &slots[index],
        datagram.bytes,
        datagram.source,
        datagram.read,
        limits,
    );
    if let Err(reason) = &staged {
        slots[index].last_rejection = Some(*reason);
        slots[index].rejected += 1;
    }
    // No await separates this fresh owner guard and the result commit.
    retire_for_control(slots, latest(control));
    let processing = Instant::now();
    if processing >= deadline {
        if staged.is_ok() {
            slots[index].rejected += 1;
        }
        slots[index].expire();
        return;
    }
    if slots[index].done() {
        if staged.is_ok() {
            slots[index].rejected += 1;
        }
        return;
    }
    if let Ok(candidate) = staged {
        commit_reply(&mut slots[index], &candidate, processing);
    }
}

/// Handle one wake after the caller's fresh post-selection owner checkpoint.
/// Read indices come only from the two private selection branches. Selected
/// datagrams are charged even after retirement; no terminal result is overwritten.
fn handle_wake(
    slots: &mut [LiveSlot; 2],
    control: &mut watch::Receiver<Control>,
    control_open: &mut bool,
    wake: Wake,
    buffers: &[[u8; RECEIVE_BYTES]; 2],
    limits: FrameLimits,
    deadline: Instant,
) {
    match wake {
        Wake::Control(open) => *control_open = open,
        Wake::Deadline => {
            for slot in slots {
                slot.expire();
            }
        }
        Wake::Writable {
            slot: index,
            result,
        } => {
            let slot = &mut slots[index];
            if slot.done() {
                return;
            }
            if let Err(error) = result {
                slot.io_cause = Some(IoCause::capture(&error));
                slot.finish(Status::Io);
            } else {
                attempt_send(slot, deadline);
            }
        }
        Wake::Read {
            slot: index,
            result,
            read,
        } => {
            let actual_datagram = match &result {
                Ok(_) => true,
                Err(error) => is_oversized(error),
            };
            if actual_datagram {
                slots[index].received += 1;
            }
            if slots[index].done() {
                if actual_datagram {
                    slots[index].rejected += 1;
                }
                return;
            }
            match result {
                Ok((length, source)) => process_reply(
                    slots,
                    control,
                    index,
                    &ReadDatagram {
                        bytes: &buffers[index][..length],
                        source,
                        read,
                    },
                    limits,
                    deadline,
                ),
                Err(error) if is_oversized(&error) => {
                    slots[index].last_rejection = Some(Rejection::Length);
                    slots[index].rejected += 1;
                }
                Err(error) => {
                    slots[index].io_cause = Some(IoCause::capture(&error));
                    slots[index].finish(Status::Io);
                }
            }
            // At the read cap, preserve pending ownership until retirement.
        }
    }
}

impl PreparedPilot {
    /// Return the two actual prepared local endpoint tuples, not caller assertions.
    #[must_use]
    pub fn local_endpoints(&self) -> [SocketAddr; 2] {
        [self.slots[0].local, self.slots[1].local]
    }

    /// Sole serialized owner. No UDP tasks, packet queues, socket clones or re-arm API.
    /// Only the private `execute` runtime polls this owner with I/O/time enabled.
    /// A closed control sender disables `changed()` polling; it neither stops nor spins.
    /// Drive the two private sockets under the one configured runtime.
    ///
    /// # Errors
    /// Return fixed setup/state failures; all terminal outcomes preserve the one-query contract.
    async fn run_inner(mut self, started: Instant) -> Result<PilotReport, PilotError> {
        let header_limits = FrameLimits::new(PACKET_BYTES, 0).map_err(|_| PilotError::WireSetup)?;
        let [a, b] = self.slots;
        let mut slots = [LiveSlot::from_prepared(a)?, LiveSlot::from_prepared(b)?];
        let mut buffers = [[0_u8; RECEIVE_BYTES]; 2];
        let deadline = started
            .checked_add(self.timeout)
            .ok_or(PilotError::DeadlineOverflow)?;
        let timer = sleep_until(TokioInstant::from_std(deadline));
        tokio::pin!(timer);
        let mut control_open = true;
        let mut reverse = false;
        loop {
            refresh_owner(&mut slots, &mut self.control, deadline);
            if slots.iter().all(LiveSlot::done) {
                break;
            }
            let wake = next_wake(
                &slots,
                &mut buffers,
                &mut self.control,
                control_open,
                timer.as_mut(),
                reverse,
            )
            .await;
            reverse = !reverse;
            // Owner observation linearizes controls before any ready I/O is used.
            refresh_owner(&mut slots, &mut self.control, deadline);
            handle_wake(
                &mut slots,
                &mut self.control,
                &mut control_open,
                wake,
                &buffers,
                header_limits,
                deadline,
            );
        }
        let [a, b] = slots;
        let reports = [a.into_report(0)?, b.into_report(1)?];
        let sockets_drop_returned = Instant::now();
        Ok(PilotReport {
            slots: reports,
            done_elapsed_ns: sockets_drop_returned.duration_since(started).as_nanos(),
            sockets_drop_returned,
        })
    }
}

/// Checked optional timer requests; construction accepts zero through `MAX_TIMEOUT`.
/// Timers belong to this whole pilot, not to individual queries.
pub struct ControlSchedule {
    /// Optional stop request time relative to `execute()`'s active-operation start.
    stop_after: Option<Duration>,
    /// Optional aggregate generation invalidation at the same time basis.
    invalidate_after: Option<Duration>,
}
impl ControlSchedule {
    /// Check both optional durations; zero is admitted before any query can send.
    ///
    /// # Errors
    /// Reject either duration above the declared laboratory maximum.
    ///
    /// # Examples
    /// A schedule is only a checked request; construction starts no timer or runtime.
    /// An immediate control request is legal, unlike a zero query timeout.
    ///
    /// ```
    /// use std::time::Duration;
    /// use nevrotick_engine::{ControlSchedule, PilotError};
    ///
    /// assert!(ControlSchedule::new(None, None).is_ok());
    /// assert!(ControlSchedule::new(Some(Duration::ZERO), None).is_ok());
    /// assert!(matches!(
    ///     ControlSchedule::new(None, Some(Duration::from_secs(30) + Duration::from_nanos(1))),
    ///     Err(PilotError::InvalidTimeout)
    /// ));
    /// ```
    pub fn new(
        stop_after: Option<Duration>,
        invalidate_after: Option<Duration>,
    ) -> Result<Self, PilotError> {
        if [stop_after, invalidate_after]
            .into_iter()
            .flatten()
            .any(|value| value > MAX_TIMEOUT)
        {
            return Err(PilotError::InvalidTimeout);
        }
        Ok(Self {
            stop_after,
            invalidate_after,
        })
    }
}

/// Safe preservation of a primary or cleanup cause, without arbitrary formatting.
#[derive(Clone, Copy, Debug)]
pub struct FailureSummary {
    /// Fixed safe code originally produced by `PilotError::as_str()`.
    code: &'static str,
    /// Optional bounded native operation cause.
    native: Option<IoCause>,
}
impl FailureSummary {
    /// Capture one closed pilot error and its bounded native cause without formatting.
    fn capture(error: PilotError) -> Self {
        let native = match error {
            PilotError::Bind(value)
            | PilotError::Nonblocking(value)
            | PilotError::LocalAddress(value)
            | PilotError::Registration(value)
            | PilotError::RuntimeBuild(value) => Some(value),
            _ => None,
        };
        Self {
            code: error.as_str(),
            native,
        }
    }
    /// Return the preserved fixed safe code.
    #[must_use]
    pub fn code(self) -> &'static str {
        self.code
    }
    /// Return the preserved bounded native cause, if any.
    #[must_use]
    pub fn native_cause(self) -> Option<IoCause> {
        self.native
    }
}

/// The only two possible owned timer actions.
#[derive(Clone, Copy)]
enum TimerAction {
    /// Latch a stop request in this pilot's own watch value.
    Stop,
    /// Advance this pilot's own aggregate generation once.
    Invalidate,
}

/// An owned cancellable sleep task; it submits no native blocking work.
type TimerJob = tokio::task::JoinHandle<Result<(), PilotError>>;

/// Apply an already-due request synchronously, or create exactly one sleep job.
///
/// # Errors
/// Propagate checked generation exhaustion before the actor can publish data.
fn start_timer(
    when: Option<Instant>,
    handle: ControlHandle,
    action: TimerAction,
) -> Result<Option<TimerJob>, PilotError> {
    let Some(when) = when else {
        return Ok(None);
    };
    if when <= Instant::now() {
        match action {
            TimerAction::Stop => handle.stop(),
            TimerAction::Invalidate => handle.invalidate()?,
        }
        return Ok(None);
    }
    Ok(Some(tokio::spawn(async move {
        sleep_until(TokioInstant::from_std(when)).await;
        match action {
            TimerAction::Stop => {
                handle.stop();
                Ok(())
            }
            TimerAction::Invalidate => handle.invalidate(),
        }
    })))
}

/// Abort and reap one owned task even if the pilot already failed.
///
/// # Errors
/// Retain task-returned errors and collapse a panic to a fixed non-payload code.
async fn finish_timer(job: Option<TimerJob>) -> Result<(), PilotError> {
    let Some(job) = job else {
        return Ok(());
    };
    job.abort();
    match job.await {
        Ok(result) => result,
        Err(error) if error.is_cancelled() => Ok(()),
        Err(_) => Err(PilotError::ControlTask),
    }
}

/// Construct timer deadlines before spawning either job, then always reap both.
///
/// # Errors
/// Preserve primary and both cleanup errors without reporting a failed run green.
async fn run_owned(
    pilot: PreparedPilot,
    schedule: ControlSchedule,
    started: Instant,
) -> Result<PilotReport, PilotError> {
    let stop_when = schedule
        .stop_after
        .map(|value| {
            started
                .checked_add(value)
                .ok_or(PilotError::DeadlineOverflow)
        })
        .transpose()?;
    let invalidate_when = schedule
        .invalidate_after
        .map(|value| {
            started
                .checked_add(value)
                .ok_or(PilotError::DeadlineOverflow)
        })
        .transpose()?;
    let handle = pilot.control_handle.clone();
    let stop_job = start_timer(stop_when, handle.clone(), TimerAction::Stop)?;
    let invalidate = start_timer(invalidate_when, handle, TimerAction::Invalidate);
    // If the second timer setup fails, still abort/reap the first before return.
    let (primary, stop_cleanup, invalidate_cleanup) = match invalidate {
        Ok(invalidate_job) => {
            let primary = pilot.run_inner(started).await;
            let stop_cleanup = finish_timer(stop_job).await;
            let invalidate_cleanup = finish_timer(invalidate_job).await;
            (primary, stop_cleanup, invalidate_cleanup)
        }
        Err(error) => (Err(error), finish_timer(stop_job).await, Ok(())),
    };
    if stop_cleanup.is_err() || invalidate_cleanup.is_err() {
        return Err(PilotError::ControlCleanup {
            primary: primary.err().map(FailureSummary::capture),
            stop: stop_cleanup.err().map(FailureSummary::capture),
            invalidate: invalidate_cleanup.err().map(FailureSummary::capture),
        });
    }
    primary
}

/// Execute one prepared whole pilot in one privately owned current-thread runtime.
///
/// This function rejects an entered Tokio context BEFORE constructing a runtime.
/// It enables I/O/time, caps the blocking pool at two (no blocking jobs are used),
/// owns at most two sleep jobs, reaps them on success/error, and drops its runtime
/// before returning. No public context-sensitive Runtime value can escape.
/// `prepare()` entropy and OS bind work remain outside this active elapsed window.
///
/// # Errors
/// Return the closed context/build/pilot/cleanup cause; never retry a query or
/// replace a runtime while native work remains. Native socket/AFD release still
/// needs separate observation; Rust Drop is not promoted into that claim.
pub fn execute(pilot: PreparedPilot, schedule: ControlSchedule) -> Result<PilotReport, PilotError> {
    if tokio::runtime::Handle::try_current().is_ok() {
        return Err(PilotError::EnteredRuntime);
    }
    let started = Instant::now();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .max_blocking_threads(2)
        .build()
        .map_err(|error| PilotError::RuntimeBuild(IoCause::capture(&error)))?;
    let result = runtime.block_on(run_owned(pilot, schedule, started));
    drop(runtime);
    result.map(|mut report| {
        report.done_elapsed_ns = Instant::now().duration_since(started).as_nanos();
        report
    })
}

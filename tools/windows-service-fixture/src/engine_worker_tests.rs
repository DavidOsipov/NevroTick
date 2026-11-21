// SPDX-License-Identifier: GPL-2.0-only
//! Observe the real Engine thread, service owner and held permit through loopback.

use std::{
    cell::RefCell,
    error::Error,
    io,
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    ops::Range,
    panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
    rc::Rc,
    sync::{
        Arc, Condvar, Mutex, MutexGuard, OnceLock,
        atomic::Ordering,
        mpsc::{Receiver, SyncSender, sync_channel},
    },
    task::Poll,
    thread,
    time::{Duration, Instant},
};

use nevrotick_engine::{
    AcquisitionError, AcquisitionReport, ControlHandle, MeasurementRefusal, SeriesConfig,
    SeriesEnd, SeriesNotStartedCause, SlotReport, SourceAddressSet, SourceConfigurationSnapshot,
    SourceHandle, SourcePolicyAuthority, SourcePolicyError, SourcePolicyLimits, Status,
    TransportFacts,
};
use windows_platform::{
    diagnostics::{ConsumerState, DiagnosticSnapshot, MemoryConsumer},
    privilege_observation::{PrivilegeLuid, PrivilegeSnapshot, ServicePrivilegeIds},
    scm::StopNotification,
    service::{LifecycleError, ServiceState, ServiceStatus, StatusReporter, StopOutcome},
};

use super::{
    AcquisitionResult, EngineWorker, Shared, any_cleanup_failure, series_operation_failed,
};
use crate::{
    FixtureError, LabRole, NativeError, NativeOperation, TokenCleanupOutcome,
    TokenObservationCause, TokenObservationFailure,
    diagnostics::Sinks,
    error::WorkerError,
    host::{
        Clock, InvocationPoll, ServiceInvocation, StatusFactory, TokenObserver, Worker,
        WorkerCompletion, WorkerFinish, WorkerWake, start_invocation_with_diagnostics,
    },
};

/// Bounded fixture errors stay distinct from assertion failures.
type Check<T = ()> = Result<T, Box<dyn Error>>;
/// Actual service owner, with the Engine worker and an independent status reporter.
type Invocation = ServiceInvocation<Reporter, EngineWorker>;
/// Independent bound for loopback observations and a selected fixture pause.
const WAIT: Duration = Duration::from_secs(5);
/// Independent bound for observing actual join even after assertion failure.
const CLOSE_WAIT: Duration = Duration::from_secs(8);
/// An ordinary Engine query/reply is exactly one NTP header here.
const PACKET_BYTES: usize = 48;
/// Independent peer receive value, unrelated to the Engine's own wall time.
const PEER_RECEIVE: u64 = 0xd001_0203_4455_6677;
/// Independent peer transmit value, unrelated to the Engine's own wall time.
const PEER_TRANSMIT: u64 = 0xd001_0203_5566_7788;
/// Original API failure to retain when Running acknowledgement is refused.
const RUNNING_ERROR: NativeError = NativeError::new(NativeOperation::Status, 711);
/// Keep actual process-wide policy transactions separate under parallel libtest too.
static CASE: Mutex<()> = Mutex::new(());
/// Each of the fifteen parent scenarios installs one source configuration.
const SOURCE_COUNT: usize = 15;
/// Each subprocess starts with its own unclaimed catalogue.
static SOURCES: OnceLock<Result<Mutex<FixtureSources>, SourcePolicyError>> = OnceLock::new();

/// Issue each process-local source once, even across separately filtered or reordered tests.
struct FixtureSources {
    /// Only this owner can install a source's first configuration.
    authority: SourcePolicyAuthority,
    /// Remaining catalogue indices, consumed before binding any loopback socket.
    unused: Range<usize>,
}

/// Independent choices at the real host observation boundaries.
#[derive(Clone, Copy)]
enum Admission {
    /// Admit startup and successfully report Running.
    Accept,
    /// Observe a genuine typed token query refusal before preparation.
    DenyToken,
    /// Refuse to acknowledge the actual attempt to report Running.
    DenyRunning,
    /// Publish genuine Stop while acknowledging Running.
    StopAtRunning,
}

/// Record status acknowledgements and observe the peer before Running can activate.
struct Reporter {
    /// Chronologically acknowledged states.
    acknowledged: Rc<RefCell<Vec<ServiceStatus>>>,
    /// Actual host control notification, used only for the selected race.
    stop: Rc<StopNotification>,
    /// Independent decision at startup or Running.
    admission: Admission,
    /// Witness to actual result and control publication.
    worker: Arc<Shared>,
    /// Real peer's empty queue observed synchronously before Running returns.
    peer: Rc<UdpSocket>,
    /// Independently select the supervised child's bounded terminal markers.
    terminal_trace: TerminalTrace,
}

impl StatusReporter for Reporter {
    type Error = NativeError;

    /// Acknowledge the actual status unless the selected observation refuses.
    fn report(&mut self, status: ServiceStatus) -> Result<(), NativeError> {
        if status.state() == ServiceState::Running {
            let dormant: bool = !self.worker.returned.load(Ordering::Acquire)
                && self.worker.result.get().is_none()
                && self
                    .worker
                    .gate
                    .lock()
                    .is_ok_and(|gate| gate.control.is_none());
            if !dormant || !peer_has_no_datagram(&self.peer).is_ok_and(|empty: bool| empty) {
                return Err(NativeError::new(NativeOperation::Status, 712));
            }
            match self.admission {
                Admission::DenyRunning => return Err(RUNNING_ERROR),
                Admission::StopAtRunning => {
                    let _control = self.stop.handle_control(1);
                }
                Admission::Accept | Admission::DenyToken => {}
            }
        }
        self.acknowledged.borrow_mut().push(status);
        if status.state() == ServiceState::Stopped {
            match self.terminal_trace {
                TerminalTrace::Silent => {}
                #[cfg(any(target_os = "linux", windows))]
                TerminalTrace::Child => unwind::observe_stopped(status.outcome()),
            }
        }
        Ok(())
    }
}

/// Hold one reporter until the actual registration consumes it.
struct Factory(Option<Reporter>);

impl StatusFactory for Factory {
    type Reporter = Reporter;

    /// Return the one fixed-role reporter without generating any status.
    fn register(&mut self, role: LabRole) -> Result<Reporter, NativeError> {
        assert_eq!(role, LabRole::Engine);
        self.0
            .take()
            .ok_or(NativeError::new(NativeOperation::Register, 713))
    }
}

/// One independently captured complete token result.
struct Token(Option<Result<PrivilegeSnapshot, TokenObservationFailure>>);

impl TokenObserver for Token {
    /// Supply the captured input once, under the real startup permit.
    fn observe(&mut self) -> Result<PrivilegeSnapshot, TokenObservationFailure> {
        self.0.take().unwrap_or_else(|| Err(token_failure()))
    }
}

/// Explicit owner time for testing service deadlines; no Engine clock is replaced.
struct OwnerClock(Instant);

impl Clock for OwnerClock {
    /// Return this exact independently selected host observation.
    fn now(&mut self) -> Instant {
        self.0
    }
}

/// An original query failure with independent token cleanup facts.
fn token_failure() -> TokenObservationFailure {
    TokenObservationFailure::new(
        TokenObservationCause::Native(NativeError::new(NativeOperation::PrivilegeQuery, 714)),
        TokenCleanupOutcome::Closed,
        TokenCleanupOutcome::NotOpened,
    )
}

/// Supply checked capture with only independently encoded `ChangeNotify` enabled.
///
/// # Errors
/// Preserve construction failures and the explicitly selected observation refusal.
fn observed_token(admission: Admission) -> Check<Token> {
    if matches!(admission, Admission::DenyToken) {
        return Ok(Token(Some(Err(token_failure()))));
    }
    let ids: ServicePrivilegeIds = ServicePrivilegeIds::new([
        PrivilegeLuid::new(71, 0),
        PrivilegeLuid::new(72, 0),
        PrivilegeLuid::new(73, 0),
        PrivilegeLuid::new(74, 0),
    ])?;
    let raw: [u8; 16] = [
        1, 0, 0, 0, // One privilege.
        74, 0, 0, 0, 0, 0, 0, 0, // Independent ChangeNotify LUID.
        2, 0, 0, 0, // Enabled.
    ];
    Ok(Token(Some(Ok(PrivilegeSnapshot::decode(&raw, ids)?))))
}

/// Serialize separate test scopes under the genuine process-once authority.
fn exclusive_case() -> MutexGuard<'static, ()> {
    CASE.lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Borrow the process's only Engine policy authority and its remaining source indices.
///
/// # Errors
/// Preserve limit, installation and mutex custody failures.
fn fixture_sources() -> Check<MutexGuard<'static, FixtureSources>> {
    let sources: &Mutex<FixtureSources> = SOURCES
        .get_or_init(|| {
            let limits: SourcePolicyLimits = SourcePolicyLimits::new(
                SOURCE_COUNT,
                16,
                Duration::from_millis(1),
                Duration::from_secs(1),
            )?;
            Ok(Mutex::new(FixtureSources {
                authority: SourcePolicyAuthority::install(limits)?,
                unused: 0..SOURCE_COUNT,
            }))
        })
        .as_ref()
        .map_err(|error: &SourcePolicyError| *error)?;
    sources
        .lock()
        .map_err(|_| "fixture authority custody was poisoned".into())
}

/// Install the reserved source's first configuration and independently select the service peer.
///
/// # Errors
/// Preserve policy installation, source/endpoint admission and worker construction failures.
fn worker_for(
    source: SourceHandle,
    installed: SocketAddr,
    selected: SocketAddr,
) -> Check<(EngineWorker, SourceConfigurationSnapshot)> {
    let addresses: SourceAddressSet = SourceAddressSet::new(&[installed])?;
    let snapshot: SourceConfigurationSnapshot = fixture_sources()?
        .authority
        .install_source_configuration(&source, addresses)?;
    let config: SeriesConfig =
        SeriesConfig::new(selected, Duration::from_secs(3), Duration::from_secs(12))?;
    Ok((EngineWorker::new(source, config)?, snapshot))
}

/// Independent choice of status observation for a test-only process.
enum TerminalTrace {
    /// Retain status observations in memory for ordinary tests.
    Silent,
    /// Publish only fixed terminal markers to the supervised parent.
    #[cfg(any(target_os = "linux", windows))]
    Child,
}

/// Retain the actual invocation through result observation and failure cleanup.
struct Rig {
    /// Actual owner, reporter, worker and work permit.
    invocation: Invocation,
    /// Independent host time shared by all its polls.
    clock: OwnerClock,
    /// Actual sticky Stop delivery.
    stop: Rc<StopNotification>,
    /// Real status acknowledgements.
    acknowledged: Rc<RefCell<Vec<ServiceStatus>>>,
    /// Witness to the real Engine report and thread tail.
    worker: Arc<Shared>,
    /// True only after actual invocation Finished.
    finished: bool,
}

impl Rig {
    /// Invoke the genuine registered host with the dormant worker.
    ///
    /// # Errors
    /// Preserve observer construction and actual host startup failure.
    fn start(worker: EngineWorker, admission: Admission, peer: &Rc<UdpSocket>) -> Check<Self> {
        Self::start_observing(worker, admission, peer, TerminalTrace::Silent)
    }

    /// Select optional bounded terminal observation before constructing the actual host.
    ///
    /// # Errors
    /// Preserve the same construction and startup failures as the ordinary fixture.
    fn start_observing(
        worker: EngineWorker,
        admission: Admission,
        peer: &Rc<UdpSocket>,
        terminal_trace: TerminalTrace,
    ) -> Check<Self> {
        Self::start_observing_with_sinks(
            worker,
            admission,
            peer,
            terminal_trace,
            Sinks::new(MemoryConsumer::new(ConsumerState::Available), None),
        )
    }

    /// Select finite concrete consumers before the same actual host invocation.
    /// This changes no Engine worker, status, clock, peer or completion owner.
    /// # Errors
    /// Preserve the same original observer/startup failures as ordinary hosting.
    fn start_observing_with_sinks(
        worker: EngineWorker,
        admission: Admission,
        peer: &Rc<UdpSocket>,
        terminal_trace: TerminalTrace,
        sinks: Sinks,
    ) -> Check<Self> {
        let proof: Arc<Shared> = Arc::clone(&worker.shared);
        let stop: Rc<StopNotification> = Rc::new(StopNotification::new());
        let acknowledged: Rc<RefCell<Vec<ServiceStatus>>> = Rc::new(RefCell::new(Vec::new()));
        let mut factory: Factory = Factory(Some(Reporter {
            acknowledged: Rc::clone(&acknowledged),
            stop: Rc::clone(&stop),
            admission,
            worker: Arc::clone(&proof),
            peer: Rc::clone(peer),
            terminal_trace,
        }));
        let mut clock: OwnerClock = OwnerClock(Instant::now());
        let mut token: Token = observed_token(admission)?;
        let invocation: Invocation = start_invocation_with_diagnostics(
            LabRole::Engine,
            0,
            &mut factory,
            &mut token,
            &mut clock,
            &stop,
            (worker, sinks),
        )
        .map_err(
            |receipt: crate::host::HostReceipt| match receipt.into_result() {
                Err(error) => error,
                Ok(()) => FixtureError::OutcomeUnavailable,
            },
        )?;
        Ok(Self {
            invocation,
            clock,
            stop,
            acknowledged,
            worker: proof,
            finished: false,
        })
    }

    /// Drive one actual owner step without acknowledging completion on its behalf.
    fn step(&mut self) -> InvocationPoll {
        let observed: InvocationPoll = self.invocation.poll(&mut self.clock, &self.stop);
        if observed == InvocationPoll::Finished {
            self.finished = true;
        }
        observed
    }

    /// Publish sticky Stop through the existing notification.
    fn request_stop(&self) {
        let _control = self.stop.handle_control(1);
    }

    /// Observe actual host completion under a separate finite fixture watchdog.
    ///
    /// # Errors
    /// A late worker never counts as Finished or releases the permit here.
    fn finish(&mut self) -> Check {
        let observed: InvocationPoll = self.step();
        self.finish_from(observed)
    }

    /// Honor the original host wait, including a notification published before park.
    /// Overdue retains the owner until a notification or the independent test watchdog.
    ///
    /// # Errors
    /// A watchdog expiry refuses another poll; it cannot promote an unnotified result.
    fn finish_from(&mut self, mut observed: InvocationPoll) -> Check {
        let started: Instant = Instant::now();
        loop {
            let remaining: Duration = CLOSE_WAIT.saturating_sub(started.elapsed());
            let wait: Duration = match observed {
                InvocationPoll::Finished => return Ok(()),
                InvocationPoll::Pending(wait) | InvocationPoll::JoinTail(wait) => {
                    wait.min(remaining)
                }
                InvocationPoll::Overdue => remaining,
            };
            thread::park_timeout(wait);
            if started.elapsed() >= CLOSE_WAIT {
                return Err("actual service fixture exceeded its join observation watchdog".into());
            }
            observed = self.step();
        }
    }

    /// Borrow the original Engine report without replacing an adverse series result.
    ///
    /// # Errors
    /// Preserve absent or failed Engine result as a fixture error.
    fn report(&self) -> Check<&AcquisitionReport> {
        let result: &AcquisitionResult = self.worker.result.get().ok_or("missing Engine result")?;
        result
            .as_ref()
            .map_err(|_| "unexpected original Engine error".into())
    }

    /// Observe acknowledged states without granting the observer further authority.
    fn states(&self) -> Vec<ServiceState> {
        self.acknowledged
            .borrow()
            .iter()
            .map(|status: &ServiceStatus| status.state())
            .collect()
    }
}

impl Drop for Rig {
    /// Request normal cleanup on failure, retaining the real fatal guard on unresolved ownership.
    fn drop(&mut self) {
        if !self.finished {
            self.request_stop();
            let _drained: Check = self.finish();
        }
    }
}

/// Pause only at an actual Engine handoff. The hold cannot run indefinitely.
struct Pause {
    /// A separate observer, which cannot consume the host thread's unpark token.
    reached: Arc<(Mutex<bool>, Condvar)>,
    /// Continue the same original thread.
    release: SyncSender<()>,
}

impl Pause {
    /// Wire one bounded pause and an independently observable reached signal.
    fn new() -> (Self, Box<dyn FnOnce() + Send>) {
        let reached: Arc<(Mutex<bool>, Condvar)> = Arc::new((Mutex::new(false), Condvar::new()));
        let witness: Arc<(Mutex<bool>, Condvar)> = Arc::clone(&reached);
        let (release, release_rx): (SyncSender<()>, Receiver<()>) = sync_channel(1);
        let hook: Box<dyn FnOnce() + Send> = Box::new(move || {
            let (flag, changed): &(Mutex<bool>, Condvar) = &witness;
            if let Ok(mut guard) = flag.lock() {
                *guard = true;
                changed.notify_one();
            }
            let _released = release_rx.recv_timeout(WAIT);
        });
        (Self { reached, release }, hook)
    }

    /// Observe that the real Engine thread reached this boundary.
    ///
    /// # Errors
    /// Preserve an absent or late worker observation.
    fn wait(&self) -> Check {
        let (flag, changed): &(Mutex<bool>, Condvar) = &self.reached;
        let guard: MutexGuard<'_, bool> = flag
            .lock()
            .map_err(|_| "fixture pause witness was poisoned")?;
        let (observed, _timeout): (MutexGuard<'_, bool>, std::sync::WaitTimeoutResult) = changed
            .wait_timeout_while(guard, WAIT, |reached: &mut bool| !*reached)
            .map_err(|_| "fixture pause witness was poisoned")?;
        if !*observed {
            return Err("expected worker handoff did not arrive before its watchdog".into());
        }
        Ok(())
    }

    /// Release that same original thread without granting completion.
    ///
    /// # Errors
    /// A missing receiver is a fixture error.
    fn release(self) -> Check {
        self.release.try_send(())?;
        Ok(())
    }
}

/// Reserve one unused source before binding its owned IPv4 loopback peer.
///
/// # Errors
/// Refuse an exhausted catalogue before socket I/O and preserve policy and socket failures.
fn source_and_peer() -> Check<(SourceHandle, Rc<UdpSocket>)> {
    let source: SourceHandle = {
        let mut sources: MutexGuard<'_, FixtureSources> = fixture_sources()?;
        let index: usize = sources
            .unused
            .next()
            .ok_or("fixture source catalogue exhausted")?;
        sources.authority.source(index)?
    };
    let socket: UdpSocket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0))?;
    socket.set_read_timeout(Some(WAIT))?;
    Ok((source, Rc::new(socket)))
}

/// Check the actual receive queue without waiting or responding.
///
/// # Errors
/// Preserve failures other than an empty queue on the owned socket.
fn peer_has_no_datagram(peer: &UdpSocket) -> io::Result<bool> {
    peer.set_nonblocking(true)?;
    let result: io::Result<(usize, SocketAddr)> = peer.recv_from(&mut [0_u8; PACKET_BYTES]);
    peer.set_nonblocking(false)?;
    match result {
        Ok(_) => Ok(false),
        Err(error) if error.kind() == io::ErrorKind::WouldBlock => Ok(true),
        Err(error) => Err(error),
    }
}

/// Observe one real bounded query with a live correlation token.
///
/// # Errors
/// Preserve the bounded native socket failure or incomplete query.
/// # Panics
/// Fail for a short or oversized query, wrong mode, or zero token.
fn receive_query(peer: &UdpSocket) -> Check<([u8; PACKET_BYTES], SocketAddr)> {
    let mut query: [u8; PACKET_BYTES + 1] = [0; PACKET_BYTES + 1];
    let (count, client): (usize, SocketAddr) = peer.recv_from(&mut query)?;
    assert_eq!(count, PACKET_BYTES);
    assert_eq!(query[0], 0x23);
    assert!(query[40..48].iter().any(|byte: &u8| *byte != 0));
    let packet: [u8; PACKET_BYTES] = query[..PACKET_BYTES].try_into()?;
    Ok((packet, client))
}

/// Respond with known independent peer facts and the live request correlation.
///
/// # Errors
/// Preserve actual peer socket send failure.
/// # Panics
/// Fail if the peer reports an incomplete send.
fn respond(peer: &UdpSocket, query: &[u8; PACKET_BYTES], client: SocketAddr) -> Check {
    let mut reply: [u8; PACKET_BYTES] = [0; PACKET_BYTES];
    reply[0] = 0x24;
    reply[1] = 2;
    reply[24..32].copy_from_slice(&query[40..48]);
    reply[32..40].copy_from_slice(&PEER_RECEIVE.to_be_bytes());
    reply[40..48].copy_from_slice(&PEER_TRANSMIT.to_be_bytes());
    assert_eq!(peer.send_to(&reply, client)?, PACKET_BYTES);
    Ok(())
}

/// Dormant misuse preserves preparation; sticky Stop forbids activation and retains its first refusal.
///
/// # Errors
/// Preserve the owned peer, checked source/session and gate-observation failures.
/// # Panics
/// Fail for lost preparation, non-idempotent readiness, activation after Stop,
/// overwritten failure, unfinished work or any acquisition publication or packet.
#[test]
fn dormant_preparation_and_cancelled_activation_preserve_the_first_inactive_failure() -> Check {
    let _case: MutexGuard<'_, ()> = exclusive_case();
    let (source, peer): (SourceHandle, Rc<UdpSocket>) = source_and_peer()?;
    let remote: SocketAddr = peer.local_addr()?;
    let (mut worker, _snapshot): (EngineWorker, SourceConfigurationSnapshot) =
        worker_for(source, remote, remote)?;
    let wake: WorkerWake = WorkerWake::current();

    assert_eq!(worker.activate(), Err(WorkerError::Activation));
    assert_eq!(worker.poll_prepare(&wake), Poll::Ready(Ok(())));
    assert_eq!(worker.poll_prepare(&wake), Poll::Ready(Ok(())));
    assert!(peer_has_no_datagram(&peer)?);

    worker.shared.stop();
    assert_eq!(worker.activate(), Err(WorkerError::Activation));
    assert_eq!(
        worker.poll_prepare(&wake),
        Poll::Ready(Err(WorkerError::Preparation))
    );
    assert_eq!(
        worker.shared.inactive_failure.get(),
        Some(&WorkerError::Activation)
    );
    worker.shared.refuse_inactive(WorkerError::Preparation);
    assert!(worker.shared.publication_failed.load(Ordering::Acquire));
    assert_eq!(
        worker.shared.inactive_failure.get(),
        Some(&WorkerError::Activation)
    );
    let finished: WorkerFinish = worker.poll_finished(&wake);
    assert_eq!(
        finished,
        WorkerFinish::Finished(WorkerCompletion {
            operation_failure: Some(WorkerError::Activation),
            cleanup_failure: None,
        })
    );
    assert!(worker.shared.result.get().is_none());
    assert!(!worker.shared.returned.load(Ordering::Acquire));
    let gate: MutexGuard<'_, super::ControlGate> = worker
        .shared
        .gate
        .lock()
        .map_err(|_| "dormant worker gate was poisoned")?;
    assert!(gate.control.is_none());
    drop(gate);
    assert!(peer_has_no_datagram(&peer)?);
    Ok(())
}

/// Actual Running admits one series and preserves all its observations after join.
///
/// # Errors
/// Preserve construction, real peer traffic and shutdown failures.
/// # Panics
/// Fail for early admission or join, incorrect report, extra work or sample admission.
#[test]
fn managed_service_acquisition_retains_real_report_without_admitting_time() -> Check {
    let _case: MutexGuard<'_, ()> = exclusive_case();
    let (source, peer): (SourceHandle, Rc<UdpSocket>) = source_and_peer()?;
    let remote: SocketAddr = peer.local_addr()?;
    let (mut worker, expected): (EngineWorker, SourceConfigurationSnapshot) =
        worker_for(source, remote, remote)?;
    let (tail, hook): (Pause, Box<dyn FnOnce() + Send>) = Pause::new();
    worker.hooks.after_return = Some(hook);
    let mut rig: Rig = Rig::start(worker, Admission::Accept, &peer)?;
    assert!(peer_has_no_datagram(&peer)?);
    assert!(matches!(rig.step(), InvocationPoll::Pending(_)));
    assert_eq!(
        rig.states(),
        [ServiceState::StartPending, ServiceState::Running]
    );
    assert_eq!(rig.invocation.outstanding_work(), 1);

    for _ in 0..3 {
        let (query, client): ([u8; PACKET_BYTES], SocketAddr) = receive_query(&peer)?;
        respond(&peer, &query, client)?;
    }
    tail.wait()?;
    let report: &AcquisitionReport = rig.report()?;
    assert_eq!(report.series().end(), SeriesEnd::Completed);
    assert_eq!(report.series().admitted_sample_count(), 0);
    assert!(report.series().primary_failure().is_none());
    assert!(report.series().stop_cleanup_failure().is_none());
    assert!(report.series().invalidate_cleanup_failure().is_none());
    for attempt in report.series().attempts() {
        let slot: &SlotReport = attempt.report().ok_or("missing real completed slot")?;
        assert!(slot.sent());
        assert_eq!(
            (slot.status(), slot.received(), slot.stratum()),
            (Status::Header, 1, Some(2))
        );
        let facts: &TransportFacts = slot.facts().ok_or("missing original header facts")?;
        assert!(
            facts
                .source_configuration()
                .is_some_and(|actual| actual.same_configuration(&expected))
        );
        assert_eq!(facts.header().receive_timestamp().bits(), PEER_RECEIVE);
        assert_eq!(facts.header().transmit_timestamp().bits(), PEER_TRANSMIT);
        assert!(matches!(
            facts.measurement(),
            Err(MeasurementRefusal::Unavailable)
        ));
    }
    assert!(matches!(rig.step(), InvocationPoll::Pending(_)));
    assert_eq!(rig.invocation.state(), Some(ServiceState::Running));
    rig.request_stop();
    assert_eq!(
        rig.step(),
        InvocationPoll::JoinTail(Duration::from_millis(20))
    );
    assert_eq!(rig.invocation.outstanding_work(), 1);
    assert!(!rig.states().contains(&ServiceState::Stopped));
    tail.release()?;
    rig.finish()?;
    assert_eq!(rig.invocation.outstanding_work(), 0);
    assert_eq!(
        rig.invocation
            .take_receipt()
            .map(crate::host::HostReceipt::into_result),
        Some(Ok(()))
    );
    assert_eq!(
        rig.invocation
            .take_receipt()
            .map(crate::host::HostReceipt::into_result),
        None
    );
    assert_eq!(rig.report()?.series().end(), SeriesEnd::Completed);
    assert!(peer_has_no_datagram(&peer)?);
    assert_eq!(
        rig.states(),
        [
            ServiceState::StartPending,
            ServiceState::Running,
            ServiceState::StopPending,
            ServiceState::Stopped
        ]
    );
    Ok(())
}

/// Independently observe all diagnostic consumer outcomes and literal healthy
/// originals after the actual Engine host has completed and joined.
/// # Panics
/// Fails if enqueue, consumption, literal fields or completeness differ.
fn assert_diagnostic_history(snapshot: &DiagnosticSnapshot, state: ConsumerState) {
    let health: &windows_platform::diagnostics::RouteHealth = snapshot.required().health();
    assert_eq!(health.queued().value(), 4);
    assert_eq!(health.consumer_attempts().value(), 4);
    assert_eq!(
        health.accepted().value(),
        if state == ConsumerState::Available {
            4
        } else {
            0
        }
    );
    assert_eq!(
        health.consumer_full().value(),
        if state == ConsumerState::Full { 4 } else { 0 }
    );
    assert_eq!(
        health.consumer_rejected().value(),
        if state == ConsumerState::Reject { 4 } else { 0 }
    );
    assert_eq!(
        health.consumer_unavailable().value(),
        if state == ConsumerState::Unavailable {
            4
        } else {
            0
        }
    );
    assert_eq!(
        snapshot.required().history_complete(),
        state == ConsumerState::Available
    );
    assert_eq!(snapshot.optional().health().accepted().value(), 4);
    if state == ConsumerState::Available {
        assert!(
            snapshot
                .required()
                .history()
                .records()
                .eq(snapshot.optional().history().records())
        );
    }
    for (sequence, (bytes, event_id)) in
        (1_u64..).zip(snapshot.optional().history().records().zip([1_u8, 2, 3, 8]))
    {
        assert_eq!(&bytes[..8], &[2, 1, 1, event_id, 3, 5, 1, 0]);
        assert_eq!(&bytes[8..16], &sequence.to_le_bytes());
        assert_eq!(bytes[16], 1);
        assert!(bytes[25..128].iter().all(|byte| *byte == 0));
    }
    assert!(snapshot.sealed());
    assert_eq!(snapshot.required().pending(), 0);
}

/// One case with four real loopback histories: diagnostic consumer refusal must
/// preserve the original completed series, zero admitted time and physical join.
/// The peer is an owned socket and the original worker guard joins on every exit.
/// # Errors
/// Preserve actual construction, peer I/O, finite pause and join failures.
/// # Panics
/// Fails if queue admission is mistaken for consumer acceptance or changes work.
#[test]
fn real_engine_join_and_series_survive_each_diagnostic_consumer_condition() -> Check {
    let _case: MutexGuard<'_, ()> = exclusive_case();
    for state in [
        ConsumerState::Available,
        ConsumerState::Full,
        ConsumerState::Reject,
        ConsumerState::Unavailable,
    ] {
        let (source, peer): (SourceHandle, Rc<UdpSocket>) = source_and_peer()?;
        let endpoint: SocketAddr = peer.local_addr()?;
        let (mut worker, expected): (EngineWorker, SourceConfigurationSnapshot) =
            worker_for(source, endpoint, endpoint)?;
        let (tail, hook): (Pause, Box<dyn FnOnce() + Send>) = Pause::new();
        worker.hooks.after_return = Some(hook);
        let mut rig: Rig = Rig::start_observing_with_sinks(
            worker,
            Admission::Accept,
            &peer,
            TerminalTrace::Silent,
            Sinks::new(
                MemoryConsumer::new(state),
                Some(MemoryConsumer::new(ConsumerState::Available)),
            ),
        )?;
        assert!(peer_has_no_datagram(&peer)?);
        assert!(matches!(rig.step(), InvocationPoll::Pending(_)));
        assert_eq!(rig.invocation.outstanding_work(), 1);
        for _ in 0..3 {
            let (query, client): ([u8; PACKET_BYTES], SocketAddr) = receive_query(&peer)?;
            respond(&peer, &query, client)?;
        }
        tail.wait()?;
        assert_eq!(rig.report()?.series().end(), SeriesEnd::Completed);
        assert_eq!(rig.report()?.series().admitted_sample_count(), 0);
        assert!(rig.report()?.series().primary_failure().is_none());
        assert!(rig.report()?.series().stop_cleanup_failure().is_none());
        assert!(
            rig.report()?
                .series()
                .invalidate_cleanup_failure()
                .is_none()
        );
        assert_eq!(rig.report()?.series().attempts().len(), 3);
        for attempt in rig.report()?.series().attempts() {
            let slot: &SlotReport = attempt.report().ok_or("missing real completed slot")?;
            assert!(slot.sent());
            assert_eq!(slot.status(), Status::Header);
            let facts: &TransportFacts = slot.facts().ok_or("missing original transport facts")?;
            assert!(
                facts
                    .source_configuration()
                    .is_some_and(|actual| actual.same_configuration(&expected))
            );
            assert!(matches!(
                facts.measurement(),
                Err(MeasurementRefusal::Unavailable)
            ));
        }
        rig.request_stop();
        assert_eq!(
            rig.step(),
            InvocationPoll::JoinTail(Duration::from_millis(20))
        );
        assert_eq!(rig.invocation.outstanding_work(), 1);
        assert!(!rig.states().contains(&ServiceState::Stopped));
        assert!(rig.invocation.take_receipt().is_none());
        tail.release()?;
        rig.finish()?;
        assert!(rig.worker.returned.load(Ordering::Acquire));
        assert_eq!(rig.invocation.outstanding_work(), 0);
        let receipt: crate::host::HostReceipt = rig
            .invocation
            .take_receipt()
            .ok_or("joined host receipt missing")?;
        assert_eq!(receipt.result(), &Ok(()));
        let snapshot: &DiagnosticSnapshot = receipt.diagnostics();
        assert_diagnostic_history(snapshot, state);
        assert_eq!(
            rig.invocation
                .take_receipt()
                .map(crate::host::HostReceipt::into_result),
            None
        );
        assert_eq!(rig.report()?.series().end(), SeriesEnd::Completed);
        assert!(peer_has_no_datagram(&peer)?);
        assert_eq!(
            rig.states(),
            [
                ServiceState::StartPending,
                ServiceState::Running,
                ServiceState::StopPending,
                ServiceState::Stopped
            ]
        );
    }
    Ok(())
}

/// Denied token or Running, and Stop while acknowledging Running, start no acquisition.
///
/// # Errors
/// Preserve actual startup or observation failures.
/// # Panics
/// Fail for an open fence, any Engine work, or replacement of the first refusal.
#[test]
fn service_admission_and_running_acknowledgement_gate_real_engine_activation() -> Check {
    let _case: MutexGuard<'_, ()> = exclusive_case();
    for admission in [
        Admission::DenyToken,
        Admission::DenyRunning,
        Admission::StopAtRunning,
    ] {
        let (source, peer): (SourceHandle, Rc<UdpSocket>) = source_and_peer()?;
        let endpoint: SocketAddr = peer.local_addr()?;
        let (worker, _snapshot): (EngineWorker, SourceConfigurationSnapshot) =
            worker_for(source, endpoint, endpoint)?;
        let mut rig: Rig = Rig::start(worker, admission, &peer)?;
        rig.finish()?;
        assert!(rig.invocation.admission_closed());
        assert_eq!(rig.invocation.outstanding_work(), 0);
        assert!(rig.worker.result.get().is_none());
        assert!(!rig.worker.returned.load(Ordering::Acquire));
        assert!(peer_has_no_datagram(&peer)?);
        let output: Option<Result<(), FixtureError>> = rig
            .invocation
            .take_receipt()
            .map(crate::host::HostReceipt::into_result);
        match admission {
            Admission::DenyToken => {
                assert_eq!(
                    output,
                    Some(Err(FixtureError::TokenObservation(token_failure())))
                );
                assert!(!rig.states().contains(&ServiceState::Running));
            }
            Admission::DenyRunning => {
                assert_eq!(
                    output,
                    Some(Err(FixtureError::Lifecycle(LifecycleError::Report(
                        RUNNING_ERROR
                    ))))
                );
                assert!(!rig.states().contains(&ServiceState::Running));
            }
            Admission::StopAtRunning => {
                assert_eq!(output, Some(Ok(())));
                assert!(rig.states().contains(&ServiceState::Running));
            }
            Admission::Accept => return Err("unselected activation fixture".into()),
        }
    }
    Ok(())
}

/// The real installed source refuses a distinct, otherwise valid loopback endpoint.
///
/// # Errors
/// Preserve construction and the original terminal observation.
/// # Panics
/// Fail for Running, any actual acquisition, or loss of the typed preparation error.
#[test]
fn real_managed_preparation_refusal_preserves_the_source_policy_error() -> Check {
    let _case: MutexGuard<'_, ()> = exclusive_case();
    let (source, peer): (SourceHandle, Rc<UdpSocket>) = source_and_peer()?;
    let installed: UdpSocket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0))?;
    let (worker, _snapshot): (EngineWorker, SourceConfigurationSnapshot) =
        worker_for(source, installed.local_addr()?, peer.local_addr()?)?;
    let mut rig: Rig = Rig::start(worker, Admission::Accept, &peer)?;
    rig.finish()?;
    assert!(!rig.states().contains(&ServiceState::Running));
    assert!(!rig.worker.returned.load(Ordering::Acquire));
    assert!(matches!(
        rig.worker.result.get(),
        Some(Err(AcquisitionError::SourcePolicy(
            SourcePolicyError::Foreign
        )))
    ));
    assert_eq!(
        rig.invocation
            .take_receipt()
            .map(crate::host::HostReceipt::into_result),
        Some(Err(FixtureError::Worker(WorkerError::Preparation)))
    );
    assert!(peer_has_no_datagram(&peer)?);
    Ok(())
}

/// Stop arriving before actual control publication remains sticky on that same control.
///
/// # Errors
/// Preserve fixture construction, synchronization and original Engine result.
/// # Panics
/// Fail if cancellation loses the charge, fails to reach the actual owner or sends.
#[test]
fn stop_between_engine_prepare_and_control_publication_is_sticky() -> Check {
    let _case: MutexGuard<'_, ()> = exclusive_case();
    let (source, peer): (SourceHandle, Rc<UdpSocket>) = source_and_peer()?;
    let endpoint: SocketAddr = peer.local_addr()?;
    let (mut worker, _snapshot): (EngineWorker, SourceConfigurationSnapshot) =
        worker_for(source, endpoint, endpoint)?;
    let (publication, hook): (Pause, Box<dyn FnOnce() + Send>) = Pause::new();
    worker.hooks.before_control = Some(hook);
    let (return_tail, hook): (Pause, Box<dyn FnOnce() + Send>) = Pause::new();
    worker.hooks.after_return = Some(hook);
    let mut rig: Rig = Rig::start(worker, Admission::Accept, &peer)?;
    assert!(matches!(rig.step(), InvocationPoll::Pending(_)));
    publication.wait()?;
    rig.request_stop();
    let pending: InvocationPoll = rig.step();
    assert!(matches!(pending, InvocationPoll::Pending(wait) if wait > CLOSE_WAIT));
    assert!(rig.invocation.admission_closed());
    assert_eq!(rig.invocation.outstanding_work(), 1);
    assert!(rig.worker.result.get().is_none());
    assert!(!rig.worker.returned.load(Ordering::Acquire));
    assert!(
        rig.worker
            .gate
            .lock()
            .is_ok_and(|gate| gate.cancelled && gate.control.is_none())
    );
    assert!(peer_has_no_datagram(&peer)?);
    publication.release()?;
    // The worker publishes its report and calls its original wake before this
    // separate Condvar observation. The host has still not polled its Pending
    // wait again, so the real unpark token must survive until it parks below.
    return_tail.wait()?;
    assert!(rig.worker.returned.load(Ordering::Acquire));
    assert!(rig.worker.result.get().is_some());
    return_tail.release()?;
    rig.finish_from(pending)?;
    let report: &AcquisitionReport = rig.report()?;
    assert_eq!(report.series().end(), SeriesEnd::Stopped);
    assert!(
        report
            .series()
            .attempts()
            .iter()
            .all(|attempt| { attempt.not_started_cause() == Some(SeriesNotStartedCause::Stopped) })
    );
    assert_eq!(report.series().admitted_sample_count(), 0);
    assert_eq!(
        rig.invocation
            .take_receipt()
            .map(crate::host::HostReceipt::into_result),
        Some(Ok(()))
    );
    assert!(
        rig.worker
            .gate
            .lock()
            .is_ok_and(|gate| gate.cancelled && gate.control.is_none())
    );
    assert!(peer_has_no_datagram(&peer)?);
    Ok(())
}

/// A real pending packet is stopped, while the report's wake precedes actual join.
///
/// # Errors
/// Preserve loopback, watchdog and joined-result failures.
/// # Panics
/// Fail for an extended deadline, early release, clean expired Stop, or extra send.
#[test]
fn stop_during_real_receive_retains_charge_through_expired_join_tail() -> Check {
    let _case: MutexGuard<'_, ()> = exclusive_case();
    let (source, peer): (SourceHandle, Rc<UdpSocket>) = source_and_peer()?;
    let endpoint: SocketAddr = peer.local_addr()?;
    let (mut worker, _snapshot): (EngineWorker, SourceConfigurationSnapshot) =
        worker_for(source, endpoint, endpoint)?;
    let (tail, hook): (Pause, Box<dyn FnOnce() + Send>) = Pause::new();
    worker.hooks.after_return = Some(hook);
    let mut rig: Rig = Rig::start(worker, Admission::Accept, &peer)?;
    assert!(matches!(rig.step(), InvocationPoll::Pending(_)));
    let (_query, _client): ([u8; PACKET_BYTES], SocketAddr) = receive_query(&peer)?;
    rig.request_stop();
    let _observation: InvocationPoll = rig.step();
    tail.wait()?;
    assert!(rig.worker.returned.load(Ordering::Acquire));
    let stopped: &AcquisitionReport = rig.report()?;
    assert_eq!(stopped.series().end(), SeriesEnd::Stopped);
    let [first, second, third] = stopped.series().attempts();
    assert!(
        first
            .report()
            .is_some_and(|first| first.sent() && first.status() == Status::Stopped)
    );
    assert_eq!(
        second.not_started_cause(),
        Some(SeriesNotStartedCause::Stopped)
    );
    assert_eq!(
        third.not_started_cause(),
        Some(SeriesNotStartedCause::Stopped)
    );
    assert_eq!(
        rig.step(),
        InvocationPoll::JoinTail(Duration::from_millis(20))
    );
    rig.clock.0 += Duration::from_millis(9_995);
    assert_eq!(
        rig.step(),
        InvocationPoll::JoinTail(Duration::from_millis(5))
    );
    rig.clock.0 += Duration::from_millis(5);
    assert_eq!(
        rig.step(),
        InvocationPoll::JoinTail(Duration::from_millis(20))
    );
    assert_eq!(rig.invocation.outstanding_work(), 1);
    assert_eq!(
        rig.invocation.failure(),
        Some(&FixtureError::Lifecycle(LifecycleError::DeadlineElapsed))
    );
    assert!(!rig.states().contains(&ServiceState::Stopped));
    assert!(peer_has_no_datagram(&peer)?);
    tail.release()?;
    rig.finish()?;
    assert_eq!(rig.invocation.outstanding_work(), 0);
    assert!(matches!(
        rig.invocation
            .take_receipt()
            .map(crate::host::HostReceipt::into_result),
        Some(Err(_))
    ));
    assert!(
        rig.acknowledged
            .borrow()
            .iter()
            .all(|status: &ServiceStatus| {
                status.state() != ServiceState::Stopped
                    || status.outcome() != Some(StopOutcome::Clean)
            })
    );
    assert_eq!(rig.report()?.series().end(), SeriesEnd::Stopped);
    assert!(peer_has_no_datagram(&peer)?);
    Ok(())
}

/// The real stale Engine report is an operation failure even inside a successful Rust Result.
///
/// # Errors
/// Preserve actual control update, socket or joined-report failure.
/// # Panics
/// Fail for stale success, lost raw report, or additional acquisition.
#[test]
fn real_stale_engine_report_closes_service_admission_without_erasing_the_report() -> Check {
    let _case: MutexGuard<'_, ()> = exclusive_case();
    let (source, peer): (SourceHandle, Rc<UdpSocket>) = source_and_peer()?;
    let endpoint: SocketAddr = peer.local_addr()?;
    let (mut worker, _snapshot): (EngineWorker, SourceConfigurationSnapshot) =
        worker_for(source, endpoint, endpoint)?;
    let (tail, hook): (Pause, Box<dyn FnOnce() + Send>) = Pause::new();
    worker.hooks.after_return = Some(hook);
    let mut rig: Rig = Rig::start(worker, Admission::Accept, &peer)?;
    assert!(matches!(rig.step(), InvocationPoll::Pending(_)));
    let (_query, _client): ([u8; PACKET_BYTES], SocketAddr) = receive_query(&peer)?;
    {
        let gate: MutexGuard<'_, super::ControlGate> = rig
            .worker
            .gate
            .lock()
            .map_err(|_| "control gate poisoned")?;
        gate.control
            .as_ref()
            .ok_or("missing original control")?
            .invalidate()?;
    }
    tail.wait()?;
    let observed: InvocationPoll = rig.step();
    let admission_closed: bool = rig.invocation.admission_closed();
    let outstanding: u32 = rig.invocation.outstanding_work();
    let stopped: bool = rig.states().contains(&ServiceState::Stopped);
    tail.release()?;
    assert!(matches!(
        observed,
        InvocationPoll::Pending(_) | InvocationPoll::JoinTail(_)
    ));
    assert!(admission_closed);
    assert_eq!(outstanding, 1);
    assert!(!stopped);
    rig.finish()?;
    assert!(rig.invocation.admission_closed());
    assert_eq!(rig.invocation.outstanding_work(), 0);
    assert_eq!(
        rig.invocation
            .take_receipt()
            .map(crate::host::HostReceipt::into_result),
        Some(Err(FixtureError::Worker(WorkerError::Running)))
    );
    let report: &AcquisitionReport = rig.report()?;
    assert_eq!(report.series().end(), SeriesEnd::Stale);
    assert_eq!(report.series().collection_end(), SeriesEnd::Stale);
    assert_eq!(report.series().admitted_sample_count(), 0);
    assert!(report.series().primary_failure().is_none());
    assert!(peer_has_no_datagram(&peer)?);
    Ok(())
}

/// A second publication of the original control cancels that same series and fences its host.
///
/// # Errors
/// Preserve real peer, original-control, pause and joined-report failures.
/// # Panics
/// Fail for accepted duplication, lost sticky cancellation, early release or additional traffic.
#[test]
fn duplicate_original_control_publication_stops_the_real_series_and_closes_admission() -> Check {
    let _case: MutexGuard<'_, ()> = exclusive_case();
    let (source, peer): (SourceHandle, Rc<UdpSocket>) = source_and_peer()?;
    let endpoint: SocketAddr = peer.local_addr()?;
    let (mut worker, _snapshot): (EngineWorker, SourceConfigurationSnapshot) =
        worker_for(source, endpoint, endpoint)?;
    let (tail, hook): (Pause, Box<dyn FnOnce() + Send>) = Pause::new();
    worker.hooks.after_return = Some(hook);
    let mut rig: Rig = Rig::start(worker, Admission::Accept, &peer)?;
    assert!(matches!(rig.step(), InvocationPoll::Pending(_)));
    let (_query, _client): ([u8; PACKET_BYTES], SocketAddr) = receive_query(&peer)?;
    let original: ControlHandle = {
        let gate: MutexGuard<'_, super::ControlGate> = rig
            .worker
            .gate
            .lock()
            .map_err(|_| "original control gate poisoned")?;
        gate.control
            .as_ref()
            .ok_or("missing original control")?
            .clone()
    };
    let cleanup_control: ControlHandle = original.clone();
    let accepted: bool = rig.worker.publish_control(original);
    let publication_failed: bool = rig.worker.publication_failed.load(Ordering::Acquire);
    let cancelled: bool = rig
        .worker
        .gate
        .lock()
        .is_ok_and(|gate: MutexGuard<'_, super::ControlGate>| gate.cancelled);
    // Record the adverse facts first. This independent control only prevents a
    // faulty implementation from stranding the test's own return barrier.
    if accepted || !publication_failed || !cancelled {
        cleanup_control.stop();
    }
    tail.wait()?;
    let observed: InvocationPoll = rig.step();
    let admission_closed: bool = rig.invocation.admission_closed();
    let outstanding: u32 = rig.invocation.outstanding_work();
    let stopped: bool = rig.states().contains(&ServiceState::Stopped);
    tail.release()?;
    assert!(!accepted);
    assert!(publication_failed);
    assert!(cancelled);
    assert!(matches!(
        observed,
        InvocationPoll::Pending(_) | InvocationPoll::JoinTail(_)
    ));
    assert!(admission_closed);
    assert_eq!(outstanding, 1);
    assert!(!stopped);
    rig.finish()?;
    assert_eq!(rig.invocation.outstanding_work(), 0);
    assert_eq!(
        rig.invocation
            .take_receipt()
            .map(crate::host::HostReceipt::into_result),
        Some(Err(FixtureError::Worker(WorkerError::Running)))
    );
    let report: &AcquisitionReport = rig.report()?;
    assert_eq!(report.series().end(), SeriesEnd::Stopped);
    assert_eq!(report.series().admitted_sample_count(), 0);
    assert!(report.series().primary_failure().is_none());
    assert!(peer_has_no_datagram(&peer)?);
    Ok(())
}

/// Check cleanup aggregation and the independent literal operation-failure table.
/// These pure controls establish policy only, not Engine cleanup production or transfer.
///
/// # Panics
/// Fail for missed cleanup, lost primary/stale failure, or a spurious operation cause.
#[test]
fn cleanup_failure_policy_reports_either_or_both_failures() {
    let cases: [(bool, bool, bool); 4] = [
        (false, false, false),
        (true, false, true),
        (false, true, true),
        (true, true, true),
    ];
    for (stop_failed, invalidate_failed, expected) in cases {
        assert_eq!(
            any_cleanup_failure(stop_failed, invalidate_failed),
            expected
        );
    }

    // The original decision already includes Engine-owned primary validation.
    // Cleanup is classified separately and cannot change these expected facts.
    let operation_cases: [(SeriesEnd, bool); 5] = [
        (SeriesEnd::Completed, false),
        (SeriesEnd::Stopped, false),
        (SeriesEnd::Stale, true),
        (SeriesEnd::SeriesDeadline, false),
        (SeriesEnd::Failure, true),
    ];
    for (end, expected) in operation_cases {
        assert_eq!(
            series_operation_failed(end),
            expected,
            "collection_end={end:?}"
        );
    }
}

/// Poison the actual control lock after Engine preparation without unwinding its worker.
///
/// # Errors
/// Preserve fixture ownership, observer and actual completion failures.
/// # Panics
/// Fail for recovery into operation, missing terminal wake, lost cause or early release.
#[test]
fn poisoned_control_publication_retires_the_real_preparation_without_sending() -> Check {
    let _case: MutexGuard<'_, ()> = exclusive_case();
    let (source, peer): (SourceHandle, Rc<UdpSocket>) = source_and_peer()?;
    let endpoint: SocketAddr = peer.local_addr()?;
    let (mut worker, _snapshot): (EngineWorker, SourceConfigurationSnapshot) =
        worker_for(source, endpoint, endpoint)?;
    let (publication, hook): (Pause, Box<dyn FnOnce() + Send>) = Pause::new();
    worker.hooks.before_control = Some(hook);
    let mut rig: Rig = Rig::start(worker, Admission::Accept, &peer)?;
    assert!(matches!(rig.step(), InvocationPoll::Pending(_)));
    publication.wait()?;
    {
        let guard = rig
            .worker
            .gate
            .lock()
            .map_err(|_| "gate was already poisoned")?;
        let caught = catch_unwind(AssertUnwindSafe(move || {
            let _held = guard;
            resume_unwind(Box::new(()));
        }));
        assert!(caught.is_err());
    }
    assert!(rig.worker.gate.is_poisoned());
    assert!(matches!(rig.step(), InvocationPoll::Pending(_)));
    assert!(rig.invocation.admission_closed());
    assert_eq!(rig.invocation.outstanding_work(), 1);
    publication.release()?;
    rig.finish()?;
    assert!(rig.worker.returned.load(Ordering::Acquire));
    assert_eq!(rig.invocation.outstanding_work(), 0);
    assert!(rig.worker.result.get().is_none());
    assert_eq!(
        rig.invocation
            .take_receipt()
            .map(crate::host::HostReceipt::into_result),
        Some(Err(FixtureError::Worker(WorkerError::Running)))
    );
    assert!(peer_has_no_datagram(&peer)?);
    Ok(())
}

/// Supervise the real worker failure in a separate process under the existing test limits.
#[cfg(any(target_os = "linux", windows))]
mod unwind {
    use std::{
        io,
        net::SocketAddr,
        rc::Rc,
        sync::MutexGuard,
        thread,
        time::{Duration, Instant},
    };

    use nevrotick_engine::{SeriesEnd, SourceConfigurationSnapshot, SourceHandle};
    use windows_platform::service::StopOutcome;

    use super::{
        Admission, Check, EngineWorker, InvocationPoll, PACKET_BYTES, Rig, TerminalTrace, WAIT,
        exclusive_case, peer_has_no_datagram, receive_query, source_and_peer, worker_for,
    };

    /// Child selection is private to the exact test executable and never reaches a native host.
    const UNWIND_CHILD_MODE: &str = "NEVROTICK_S31_ENGINE_UNWIND_CHILD";
    /// The only child selector used by the parent's owned process command.
    const UNWIND_CHILD_TEST: &str =
        "engine_worker::tests::unwind::unexpected_engine_worker_unwind_aborts_before_finished";
    /// Independent time limit for each exact child; the match never admits retries.
    const UNWIND_CHILD_WAIT: Duration = Duration::from_secs(16);
    /// Only the actual acquisition thread can publish this marker after real preparation.
    const CHILD_PREPARED: &str = "engine-worker-child:prepared";
    /// The child hook records the unwind without formatting the payload.
    const CHILD_UNWIND: &str = "engine-worker-child:unwind";
    /// The parent must never see the captured payload destroyed by the unwind guard.
    const CHILD_PAYLOAD_DROPPED: &str = "engine-worker-child:payload-dropped";
    /// Observe actual Stopped at the status reporter, independently from the driver result.
    const CHILD_STOPPED: &str = "engine-worker-child:stopped";
    /// Observe an actual Clean outcome on that same Stopped publication.
    const CHILD_CLEAN: &str = "engine-worker-child:clean";
    /// Emit original terminal facts directly from the actual reporter.
    pub(super) fn observe_stopped(outcome: Option<StopOutcome>) {
        eprintln!("{CHILD_STOPPED}");
        if outcome == Some(StopOutcome::Clean) {
            eprintln!("{CHILD_CLEAN}");
        }
    }

    /// The ordinary owner successfully obtained Finished and consumed its clean result.
    const CHILD_FINISHED: &str = "engine-worker-child:finished";
    /// Publish before any host cleanup if the worker's own unwind guard failed to abort.
    const CHILD_SURVIVED: &str = "engine-worker-child:guard-return-failure";

    /// A bounded synthetic payload observes improper destruction without exposing text.
    struct WorkerUnwindPayload;

    impl Drop for WorkerUnwindPayload {
        /// Make actual destruction visible without performing a service operation.
        fn drop(&mut self) {
            eprintln!("{CHILD_PAYLOAD_DROPPED}");
        }
    }

    /// Exactly one healthy or unwinding invocation in the owned test process.
    #[derive(Clone, Copy, Eq, PartialEq)]
    enum ChildMode {
        /// Return from the preparation hook and join a real stopped acquisition.
        Healthy,
        /// Unwind from the actual acquisition thread after preparation.
        Unwind,
    }

    impl ChildMode {
        /// Encode only the two recognized private fixture commands.
        fn value(self) -> &'static str {
            match self {
                Self::Healthy => "healthy",
                Self::Unwind => "unwind",
            }
        }
    }

    /// Verify the configured supervisor's inherited Linux no-core limit before activation.
    ///
    /// Linux pipe patterns ignore zero; the supported WSL kernel's pipe dumper
    /// returns before launching its helper at a core limit of one. A file pattern
    /// must inherit zero. An unreviewed socket pattern admits no fault execution.
    ///
    /// # Errors
    /// Preserve unreadable process observations or refuse an ineffective limit.
    #[cfg(target_os = "linux")]
    fn check_core_suppression() -> Check {
        let pattern: String = std::fs::read_to_string("/proc/sys/kernel/core_pattern")?;
        let expected: &str = match pattern.as_bytes().first() {
            Some(b'|') => "1",
            Some(b'@') => return Err("fixture does not admit a core socket pattern".into()),
            Some(_) => "0",
            None => return Err("missing kernel core pattern".into()),
        };
        let limits: String = std::fs::read_to_string("/proc/self/limits")?;
        let mut core: std::str::SplitWhitespace<'_> = limits
            .lines()
            .find_map(|line: &str| line.strip_prefix("Max core file size"))
            .ok_or("missing inherited core size limit")?
            .split_whitespace();
        if core.next() != Some(expected) || core.next() != Some(expected) {
            return Err("fixture supervisor did not disable the inherited core path".into());
        }
        Ok(())
    }

    /// Run the existing Worker and `ServiceInvocation` with an exact after-prepare hook.
    ///
    /// # Errors
    /// Preserve setup, loopback and actual joined result failures.
    /// # Panics
    /// The unwind case deliberately panics on the worker. The healthy branch rejects
    /// absent peer activity, premature completion, an unexpected report or dirty Stop.
    fn run_unwind_child(mode: ChildMode) -> Check {
        let _case: MutexGuard<'_, ()> = exclusive_case();
        let (source, peer): (SourceHandle, Rc<std::net::UdpSocket>) = source_and_peer()?;
        let endpoint: SocketAddr = peer.local_addr()?;
        // Each child has its own process-wide Engine policy authority.
        let (mut worker, _snapshot): (EngineWorker, SourceConfigurationSnapshot) =
            worker_for(source, endpoint, endpoint)?;
        if mode == ChildMode::Unwind {
            std::panic::set_hook(Box::new(|info: &std::panic::PanicHookInfo<'_>| {
                if info.payload().is::<WorkerUnwindPayload>() {
                    eprintln!("{CHILD_UNWIND}");
                }
            }));
        }
        worker.hooks.before_control = Some(Box::new(move || {
            eprintln!("{CHILD_PREPARED}");
            if mode == ChildMode::Unwind {
                #[expect(
                    clippy::panic,
                    reason = "the isolated child deliberately unwinds inside the real Engine worker"
                )]
                std::panic::panic_any(WorkerUnwindPayload);
            }
        }));
        let mut rig: Rig =
            Rig::start_observing(worker, Admission::Accept, &peer, TerminalTrace::Child)?;
        let started: InvocationPoll = rig.step();
        if mode == ChildMode::Unwind {
            // Retain the owner without polling join, requesting Stop, or dropping it.
            // A different abort path on the host must not satisfy the worker-guard test.
            if matches!(started, InvocationPoll::Pending(_)) {
                thread::sleep(WAIT);
            }
            eprintln!("{CHILD_SURVIVED}");
            return Err("the worker unwind failed to abort its process".into());
        }
        assert!(matches!(started, InvocationPoll::Pending(_)));
        let (_query, _client): ([u8; PACKET_BYTES], SocketAddr) = receive_query(&peer)?;
        assert_eq!(rig.invocation.outstanding_work(), 1);
        rig.request_stop();
        rig.finish()?;
        assert_eq!(rig.invocation.outstanding_work(), 0);
        assert_eq!(rig.report()?.series().end(), SeriesEnd::Stopped);
        assert_eq!(
            rig.invocation
                .take_receipt()
                .map(crate::host::HostReceipt::into_result),
            Some(Ok(()))
        );
        assert!(peer_has_no_datagram(&peer)?);
        eprintln!("{CHILD_FINISHED}");
        Ok(())
    }

    /// Terminate only the original child and collect its native termination status.
    ///
    /// # Errors
    /// Preserve refusal to kill or reap; an already exited child is still reaped.
    fn kill_and_reap(child: &mut std::process::Child) -> io::Result<()> {
        if let Err(error) = child.kill()
            && child.try_wait()?.is_none()
        {
            return Err(error);
        }
        child.wait().map(|_: std::process::ExitStatus| ())
    }

    /// Wait for the one owned process under an absolute budget and retain its original output.
    ///
    /// The child starts no descendant processes. Once it exits, both pipes are closed
    /// and collection cannot wait for an inherited writer handle from a later child.
    ///
    /// # Errors
    /// Preserve launch/observation/collection failures. A deadline or wait failure
    /// requests kill and reap of the original child before returning.
    fn supervise_unwind_child(mode: ChildMode) -> Check<std::process::Output> {
        use std::process::{Child, Command, ExitStatus, Stdio};

        let mut child: Child = Command::new(std::env::current_exe()?)
            .args([
                "--exact",
                UNWIND_CHILD_TEST,
                "--nocapture",
                "--test-threads=1",
            ])
            .env(UNWIND_CHILD_MODE, mode.value())
            .env("RUST_BACKTRACE", "0")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let started: Instant = Instant::now();
        loop {
            let observed: io::Result<Option<ExitStatus>> = child.try_wait();
            match observed {
                Ok(Some(_)) => return child.wait_with_output().map_err(Into::into),
                Ok(None) if started.elapsed() < UNWIND_CHILD_WAIT => {
                    thread::sleep(Duration::from_millis(10));
                }
                Ok(None) => {
                    kill_and_reap(&mut child)?;
                    return Err("Engine unwind child exceeded its wait deadline".into());
                }
                Err(error) => {
                    kill_and_reap(&mut child)?;
                    return Err(error.into());
                }
            }
        }
    }

    /// Observe the required native Linux abort and the absence of a dumped core.
    #[cfg(target_os = "linux")]
    fn is_fixture_abort(status: std::process::ExitStatus) -> bool {
        use std::os::unix::process::ExitStatusExt;

        const SIGABRT: i32 = 6;
        status.signal() == Some(SIGABRT) && !status.core_dumped()
    }

    /// The pinned Rust Windows abort uses `FAST_FAIL_FATAL_APP_EXIT` on x64.
    #[cfg(windows)]
    fn is_fixture_abort(status: std::process::ExitStatus) -> bool {
        // The service only adopts current Windows families. Retain the exact native
        // non-continuable fast-fail status, in the signed form returned by std.
        const STATUS_FAST_FAIL: i32 = i32::from_ne_bytes(0xc000_0409_u32.to_ne_bytes());
        status.code() == Some(STATUS_FAST_FAIL)
    }

    /// Match only one complete bounded marker line, tolerating Windows line endings.
    fn marked(output: &str, marker: &str) -> bool {
        output.lines().any(|line: &str| line == marker)
    }

    /// Reach the real acquisition thread and observe its mandatory process abort.
    ///
    /// # Errors
    /// Preserve configured core admission, child setup or bounded supervision failures.
    /// # Panics
    /// Fail for the wrong process outcome, missing reached markers, unwind-payload
    /// destruction, or a fault child that publishes a terminal acknowledgement.
    #[cfg_attr(
        windows,
        ignore = "requires separately admitted disposable Windows crash supervision"
    )]
    #[test]
    fn unexpected_engine_worker_unwind_aborts_before_finished() -> Check {
        #[cfg(target_os = "linux")]
        check_core_suppression()?;

        match std::env::var(UNWIND_CHILD_MODE) {
            Ok(mode) if mode == ChildMode::Healthy.value() => {
                return run_unwind_child(ChildMode::Healthy);
            }
            Ok(mode) if mode == ChildMode::Unwind.value() => {
                return run_unwind_child(ChildMode::Unwind);
            }
            Ok(_) => return Err("unsupported Engine unwind child mode".into()),
            Err(std::env::VarError::NotPresent) => {}
            Err(error) => return Err(error.into()),
        }

        for mode in [ChildMode::Healthy, ChildMode::Unwind] {
            let output: std::process::Output = supervise_unwind_child(mode)?;
            let observed: &str = std::str::from_utf8(&output.stderr)?;
            assert!(marked(observed, CHILD_PREPARED));
            assert!(!marked(observed, CHILD_SURVIVED));
            assert!(!marked(observed, CHILD_PAYLOAD_DROPPED));
            if mode == ChildMode::Healthy {
                assert!(output.status.success());
                assert!(!marked(observed, CHILD_UNWIND));
                assert!(marked(observed, CHILD_STOPPED));
                assert!(marked(observed, CHILD_CLEAN));
                assert!(marked(observed, CHILD_FINISHED));
            } else {
                assert!(is_fixture_abort(output.status));
                assert!(marked(observed, CHILD_UNWIND));
                assert!(!marked(observed, CHILD_STOPPED));
                assert!(!marked(observed, CHILD_CLEAN));
                assert!(!marked(observed, CHILD_FINISHED));
            }
        }
        Ok(())
    }
}

// SPDX-License-Identifier: GPL-2.0-only
//! Actual shared fixture owner, with narrow registration, clock and wait seams.
//!
//! This actual owner is used by both native executable compositions.
//! Tests substitute only the actual API/time/wait observations; they do not
//! impersonate native SCM evidence. No native job, clock policy or protected
//! product work is admitted. The existing one-slot lifecycle capacity owns the
//! synchronous capture and worker phases through sequential permits. The private
//! invocation retains unresolved work across deadlines and cannot return early.
//! Worker phase methods return promptly. Pending state remains observable until
//! polled; its producer publishes state before notifying the retained owner wake.
//! Cancellation and health failure never acknowledge physical completion.

use std::{
    convert::Infallible,
    marker::PhantomData,
    num::NonZeroU32,
    rc::Rc,
    sync::{
        OnceLock,
        atomic::{AtomicBool, Ordering},
    },
    task::Poll,
    thread::Thread,
    time::{Duration, Instant},
};

use windows_platform::{
    diagnostics::{ConsumerState, DiagnosticObservation, DiagnosticSnapshot, MemoryConsumer},
    privilege_observation::PrivilegeSnapshot,
    scm::StopNotification,
    service::{
        LifecycleError, PendingBudget, ServiceLifecycle, ServiceState, StatusReporter, StopOutcome,
        WorkPermit,
    },
};

use crate::{
    FixtureError, LabRole, NativeError, TokenObservationFailure,
    diagnostics::{
        CloseMode, Delivery, Diagnostics, Event, EventWriter, Sinks, lifecycle_cause,
        primary_cause, worker_cause,
    },
    error::{ServiceFailure, WorkerError},
};

#[cfg(test)]
use crate::diagnostics::{UnconfirmedWriter, WriterClose};
#[cfg(any(windows, test))]
use crate::{ClockReadRetirement, SampledClockBreak};
#[cfg(any(windows, test))]
use crate::{NativeBrokerClockRead, broker_clock_change::NativeBrokerReader};

#[cfg(test)]
#[path = "broker_observation.rs"]
mod broker_observation;
#[cfg(test)]
use broker_observation::{BrokerObservation, BrokerProbe};

/// Wake the existing owner thread with one coalesced, retained unpark token.
/// A worker retains at most one clone and publishes state before notification.
#[derive(Clone)]
pub(super) struct WorkerWake {
    /// Existing owner thread; this creates no thread, pool or replacement owner.
    owner: Thread,
}

impl WorkerWake {
    /// Bind notifications to the current serialized host invocation.
    pub(super) fn current() -> Self {
        Self {
            owner: std::thread::current(),
        }
    }

    /// Retain a wake across the owner's check-to-park race, coalescing duplicates.
    pub(super) fn notify(&self) {
        self.owner.unpark();
    }
}

/// One tagged worker-health fact; kind and code have independent meaning.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum WorkerFailure {
    /// The original operation failed; cancellation still retains its charge.
    Operation(WorkerError),
    /// Cleanup failed; actual completion is still separately observed.
    Cleanup(WorkerError),
}

/// Canonical facts published only after the worker has actually finished.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct WorkerCompletion {
    /// Original operation refusal, independent of cleanup and host Stop.
    pub(super) operation_failure: Option<WorkerError>,
    /// Original cleanup refusal, independent of the operation result.
    pub(super) cleanup_failure: Option<WorkerError>,
}

/// Closed custody observation; a returned result alone is not a joined thread.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use = "retain the original worker charge until actual completion"]
pub(super) enum WorkerFinish {
    /// No terminal marker; remain event-driven without a heartbeat.
    Pending,
    /// A terminal marker exists, but physical thread exit/join remains pending.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "the admitted thread worker is test-only; inert native defaults retain this private finish protocol"
        )
    )]
    JoinTailPending,
    /// Actual completion/join acknowledges both independent result facts.
    Finished(WorkerCompletion),
}

/// Bounded worker seam; only the host owns status, admission and work permits.
/// A supplied worker is dormant, with no already-started job. Its first operation
/// begins only in `poll_prepare` after the host admits the preparation permit.
pub(super) trait Worker {
    /// Ready acknowledges physically completed initialization and dormant readiness.
    /// Pending or failure releases no work; any future state change must wake owner.
    fn poll_prepare(&mut self, wake: &WorkerWake) -> Poll<Result<(), WorkerError>>;

    /// Activate only after acknowledged Running and a separately admitted permit.
    ///
    /// # Errors
    /// A refusal starts owned cancellation and does not acknowledge completion.
    fn activate(&mut self) -> Result<(), WorkerError>;

    /// Observe operational health without acknowledging completed work or cleanup.
    /// Future health changes publish state before waking the owner.
    ///
    /// # Errors
    /// Retain any failure as cause while the original permit remains charged.
    fn poll_running(&mut self, wake: &WorkerWake) -> Result<(), WorkerFailure>;

    /// Request cancellation once after admission is permanently fenced.
    /// This prompt request itself provides no completion or cleanup acknowledgement.
    fn request_cancel(&mut self, wake: &WorkerWake);

    /// Finished acknowledges actual jobs and resolved cleanup custody, including
    /// an acknowledged recovery handoff if required. Unresolved cleanup is Pending.
    /// A failed completion retains its typed cause alongside earlier failures.
    fn poll_finished(&mut self, wake: &WorkerWake) -> WorkerFinish;
}

/// Actual default worker for both fixed-role laboratory executable compositions.
/// It owns no product work, native jobs, accounts, clocks or network effects.
pub(super) struct InertWorker {
    /// Published before the completion wake; cancellation changes no native state.
    cancelled: bool,
}

impl InertWorker {
    /// Prepare an inert dormant worker without starting an operation.
    pub(super) const fn new() -> Self {
        Self { cancelled: false }
    }
}

impl Worker for InertWorker {
    /// The inert initializer has no outstanding operation or retained resource.
    fn poll_prepare(&mut self, _wake: &WorkerWake) -> Poll<Result<(), WorkerError>> {
        Poll::Ready(Ok(()))
    }

    /// Activation performs no product or native effect.
    fn activate(&mut self) -> Result<(), WorkerError> {
        Ok(())
    }

    /// The inert loop has no independent operational fault source.
    fn poll_running(&mut self, _wake: &WorkerWake) -> Result<(), WorkerFailure> {
        Ok(())
    }

    /// Publish the inert completion condition before its retained owner wake.
    fn request_cancel(&mut self, wake: &WorkerWake) {
        self.cancelled = true;
        wake.notify();
    }

    /// Acknowledge completion only after the cancellation condition was published.
    fn poll_finished(&mut self, _wake: &WorkerWake) -> WorkerFinish {
        if self.cancelled {
            WorkerFinish::Finished(WorkerCompletion {
                operation_failure: None,
                cleanup_failure: None,
            })
        } else {
            WorkerFinish::Pending
        }
    }
}

/// Explicit source-card start/stop wait hint in milliseconds, not a product default.
const LAB_WAIT_HINT_MS: u32 = 1000;
/// Explicit source-card start/stop phase allowance, not a product default.
const LAB_PENDING_MS: u32 = 10_000;
/// Explicit maximum useful laboratory control-loop interval.
const LAB_RUNNING: Duration = Duration::from_millis(60_000);
/// Fixture-only maximum relative wait request after a real terminal marker.
/// Early/spurious returns remain possible; this is no wake-rate guarantee.
const JOIN_TAIL_WAIT: Duration = Duration::from_millis(20);

/// The one native dispatcher call; role selection never comes from raw input.
pub(super) trait Dispatcher {
    /// Connect the fixed-role process or return its immediately captured error.
    ///
    /// # Errors
    /// Preserve the actual dispatcher failure, without falling back to a console.
    fn dispatch(&mut self, role: LabRole) -> Result<(), NativeError>;
}

/// Handler registration returns a privately constructed reporter, never a raw token.
pub(super) trait StatusFactory {
    /// Reporter exclusively retained by the serialized service owner.
    type Reporter: StatusReporter<Error = NativeError>;

    /// Register only the selected fixed role.
    ///
    /// # Errors
    /// Return actual registration failure; create no reporter on failure.
    fn register(&mut self, role: LabRole) -> Result<Self::Reporter, NativeError>;
}

/// Synchronous self-token capture: known snapshots require successful cleanup.
/// Failure supplies no snapshot and may retain a handle until process termination;
/// the actual native producer uses fixed queries and never retries a failed close.
pub(super) trait TokenObserver {
    /// Observe the current process primary token and reject owner-thread tokens.
    ///
    /// # Errors
    /// Preserve first failure and separate cleanup outcomes; never retry a capture.
    fn observe(&mut self) -> Result<PrivilegeSnapshot, TokenObservationFailure>;
}

/// Monotonic observation effect seam; the actual fixture observer uses `Instant`.
pub(super) trait Clock {
    /// Observe time without modifying or normalizing an earlier observation.
    fn now(&mut self) -> Instant;
}

/// Fixture-local observation custody; a reversal permanently ends ordinary use.
/// The retained instant is historical after a fault, never a repaired clock.
struct ObservedClock {
    /// Last observation actually accepted in nondecreasing order.
    last_trusted: Instant,
    /// First actual lifecycle observation; never rewritten on consumption or fault.
    origin: Instant,
    /// Sticky refusal of every subsequent ordinary observation.
    reversed: bool,
}

impl ObservedClock {
    /// Establish the first observation without claiming an earlier time origin.
    fn new(source: &mut impl Clock) -> Self {
        let last_trusted: Instant = source.now();
        Self {
            last_trusted,
            origin: last_trusted,
            reversed: false,
        }
    }

    /// Admit only a genuinely observed nondecreasing instant.
    ///
    /// # Errors
    /// Latch reversal before returning; a latched fault performs no further read.
    fn observe(&mut self, source: &mut impl Clock) -> Result<Instant, FixtureError> {
        if self.reversed {
            return Err(FixtureError::ClockReversed);
        }
        let now = source.now();
        if now < self.last_trusted {
            self.reversed = true;
            return Err(FixtureError::ClockReversed);
        }
        self.last_trusted = now;
        Ok(now)
    }

    /// Obtain time solely for stop bookkeeping, with failure-only historical fallback.
    /// A reversed observation first requires failure, retaining any earlier cause.
    /// Historical fallback admits no work, wait, renewed run interval or Clean
    /// outcome, and establishes neither current time nor timely completion.
    fn cleanup_observation(
        &mut self,
        source: &mut impl Clock,
        failure: &mut Option<FixtureError>,
    ) -> Instant {
        match self.observe(source) {
            Ok(now) => now,
            Err(error) => {
                if failure.is_none() {
                    *failure = Some(error);
                }
                self.last_trusted
            }
        }
    }
}

/// Finite owner-thread waiting; spurious wakes never imply Stop or completion.
pub(super) trait Wait {
    /// Wait at most the requested interval according to the selected primitive.
    /// Scheduling may delay resumption; every return requires fresh guards.
    fn park(&mut self, timeout: Duration);

    /// Retain an overdue invocation until a state-change notification arrives.
    /// There is no remaining deadline or heartbeat; this makes no bounded native
    /// termination claim. The existing thread's retained unpark token closes the
    /// worker check-to-park race.
    fn park_until_notified(&mut self) {
        std::thread::park();
    }
}

/// Attempt dispatcher connection once even if the first call fails.
///
/// # Errors
/// Reject repeated invocation; propagate the original native dispatcher failure.
pub(super) fn dispatch_once<D: Dispatcher>(
    role: LabRole,
    attempted: &AtomicBool,
    driver: &mut D,
) -> Result<(), FixtureError> {
    if attempted.swap(true, Ordering::AcqRel) {
        return Err(FixtureError::AlreadyDispatched);
    }
    driver.dispatch(role).map_err(FixtureError::Native)
}

/// Connect once and project the immutable owner-result receipt.
/// The attempt flag and failure cell belong to one process, initially false and
/// empty. Only this helper writes that failure cell; only the service owner writes
/// its result receipt. Native failure custody never clones or replaces that result.
///
/// # Errors
/// Reject duplicate entry, dispatcher failure or a failed/missing owner receipt.
/// A successful owner receipt does not authorize ignoring dispatcher failure.
pub(super) fn dispatch_with_outcome<'a, D: Dispatcher>(
    role: LabRole,
    attempted: &AtomicBool,
    driver: &mut D,
    outcome: &'a OnceLock<HostReceipt>,
    dispatch_failure: &'a OnceLock<FixtureError>,
) -> Result<(), &'a FixtureError> {
    if let Err(error) = dispatch_once(role, attempted, driver) {
        if error == FixtureError::AlreadyDispatched {
            return Err(&FixtureError::AlreadyDispatched);
        }
        return Err(dispatch_failure.get_or_init(|| error));
    }
    match outcome.get() {
        Some(receipt) => match receipt.result() {
            Ok(()) => Ok(()),
            Err(error) => Err(error),
        },
        None => Err(&FixtureError::OutcomeUnavailable),
    }
}

/// Run the actual `ServiceMain` body with an unwind-to-abort fixture boundary.
/// Normal return preserves all ordinary fallible owner results. An unwinding
/// panic retains its captured payload until abort, without resuming the body or
/// returning across the native ABI. The panic hook has already run; aborts and
/// native faults are not caught, and destructor cleanup is not promised.
pub(super) fn guard_service_main<F: FnOnce() + std::panic::UnwindSafe>(body: F) {
    if let Err(_panic_payload) = std::panic::catch_unwind(body) {
        // Keep this named binding alive until the non-returning call. Dropping
        // an arbitrary panic payload could itself execute code or panic again.
        std::process::abort();
    }
}

/// One immutable result and diagnostic history from the same actual invocation.
/// Snapshot delivery never acknowledges worker/native completion or changes failure.
#[must_use = "retain the original result together with its diagnostic history"]
pub(super) struct HostReceipt {
    /// Original result objects with unchanged primary/secondary ownership.
    actual_result: Result<(), FixtureError>,
    /// Finite consumed originals, loss and health from that same invocation.
    diagnostics: DiagnosticSnapshot,
    /// Last historical SCM-observed broker read from the actual Windows driver.
    #[cfg(any(windows, test))]
    broker_clock_read: Option<NativeBrokerClockRead>,
    /// Latest sampled revision invalidation, independent of subsequent reads.
    #[cfg(any(windows, test))]
    broker_clock_break: Option<SampledClockBreak>,
    /// Local completion and loss observation from the portable writer control.
    #[cfg(test)]
    writer_close: Option<WriterClose>,
    /// Historical result of the explicitly selected, completed broker getter.
    #[cfg(test)]
    broker_observation: Option<BrokerObservation>,
}

impl HostReceipt {
    /// Borrow the terminal record without reobserving time or policy.
    #[cfg(any(windows, test))]
    pub(super) const fn broker_clock_read(&self) -> Option<NativeBrokerClockRead> {
        self.broker_clock_read
    }

    /// Read the original historical break without starting another observation.
    #[cfg(any(windows, test))]
    pub(super) const fn broker_clock_break(&self) -> Option<SampledClockBreak> {
        self.broker_clock_break
    }
    /// Read the original result without consuming its coherent diagnostic history.
    pub(super) const fn result(&self) -> &Result<(), FixtureError> {
        &self.actual_result
    }

    /// Read actual portable consumption; this makes no native/durable delivery claim.
    pub(super) const fn diagnostics(&self) -> &DiagnosticSnapshot {
        &self.diagnostics
    }

    /// Borrow the outcome of the selected local broker read from this receipt.
    #[cfg(test)]
    const fn broker_observation(&self) -> Option<BrokerObservation> {
        self.broker_observation
    }

    /// Deliberately consume a receipt when a caller needs only the original result.
    ///
    /// # Errors
    /// Returns the original operation failure, preserving its cleanup facts.
    #[cfg(test)]
    pub(super) fn into_result(self) -> Result<(), FixtureError> {
        self.actual_result
    }

    /// Finalize an honest failure before an invocation or lifecycle observation exists.
    fn startup_failure(error: FixtureError, mut diagnostics: Diagnostics) -> Self {
        diagnostics.emit(
            Event::PrimaryFailure(primary_cause(&error)),
            DiagnosticObservation::unavailable(),
        );
        Self {
            actual_result: Err(error),
            diagnostics: diagnostics.finish(),
            #[cfg(any(windows, test))]
            broker_clock_read: None,
            #[cfg(any(windows, test))]
            broker_clock_break: None,
            #[cfg(test)]
            writer_close: None,
            #[cfg(test)]
            broker_observation: None,
        }
    }

    /// Refuse native owner publication without inventing an invocation or clock read.
    pub(super) fn owner_already_set(role: LabRole) -> Self {
        Self::startup_failure(
            FixtureError::OwnerAlreadySet,
            Diagnostics::new(
                role,
                Sinks::new(MemoryConsumer::new(ConsumerState::Available), None),
            ),
        )
    }
}

/// Finite result of borrowing the same persistent invocation for one owner step.
/// Dropping this value neither drops the invocation nor releases charged work.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum InvocationPoll {
    /// Await a coalesced worker/Stop notification, bounded by the fixed deadline.
    Pending(Duration),
    /// Observe only marked thread-exit/join custody, without a new allowance.
    JoinTail(Duration),
    /// A deadline or unusable clock ended ordinary waiting; retain the same owner.
    Overdue,
    /// The writer joined but its release remains unconfirmed. No writer wake follows.
    #[cfg(test)]
    WriterReleaseUnconfirmed(UnconfirmedWriter),
    /// Actual cleanup and the sole terminal report attempt have completed.
    Finished,
}

/// Internal worker phase, never a separately replaceable lifecycle or reporter.
#[derive(Clone, Copy, Eq, PartialEq)]
enum WorkerPhase {
    /// Host admission succeeded; worker initialization is incomplete or not begun.
    Preparing,
    /// Dormant preparation and Running acknowledgement succeeded.
    Running,
    /// Admission is permanently closed and actual completion is still observed.
    Stopping,
    /// Actual completion and the sole terminal report attempt are finished.
    Finished,
}

/// Actual worker custody, separate from intended and acknowledged service status.
#[derive(Clone, Copy, Eq, PartialEq)]
enum WorkerCustody {
    /// No worker method has run and no worker operation has begun.
    Dormant,
    /// Preparation owns its original charge until Ready or Finished.
    Preparing,
    /// Initialization completed and the worker is dormant before activation.
    Prepared,
    /// Activation was attempted under the separately admitted Running charge.
    Active,
    /// Cancellation was requested once; actual completion remains unacknowledged.
    Cancelling,
    /// A current terminal marker awaits physical exit/join under the same custody.
    Joining,
    /// Finished acknowledged resolved actual work and cleanup custody.
    Finished,
}

/// One thread-bound invocation retaining the original status and completion owners.
/// All work admission stays private and is fenced before fallible Stop handling.
/// The driver keeps this owner through Finished; unresolved whole-owner destruction
/// aborts this inert fixture before any field destructor can discard custody.
/// This guard selects no production native-job termination or recovery policy.
pub(super) struct ServiceInvocation<
    R: StatusReporter<Error = NativeError>,
    P: Worker,
    W: EventWriter = Infallible,
> {
    /// Original serialized lifecycle and reporter, never replaced after expiry.
    owner: ServiceLifecycle<R>,
    /// Original worker, including its pending state and resolved cleanup custody.
    worker: P,
    /// Sole current phase charge, completed only by an explicit acknowledgement.
    permit: Option<WorkPermit>,
    /// Current host phase, distinct from acknowledged SCM status.
    phase: WorkerPhase,
    /// Sticky fence covering every private admission path, including bad-time Stop.
    admission_closed: bool,
    /// Actual phase/cancellation acknowledgement; no independent success flags.
    custody: WorkerCustody,
    /// The original owner's level-triggered, coalesced notification target.
    wake: WorkerWake,
    /// Configured native broker reader, held by the same Running permit.
    #[cfg(any(windows, test))]
    broker_clock_reader: Option<NativeBrokerReader>,
    /// Only the explicit broker fixture can select one read before preparation.
    #[cfg(test)]
    broker_probe: Option<BrokerProbe>,
    /// Trusted observation history retained across every poll and deadline.
    observed: ObservedClock,
    /// Fixed startup or shutdown deadline; repeated polls never renew it.
    phase_deadline: Option<Instant>,
    /// Fixed laboratory maximum running interval, established at Ready.
    run_deadline: Option<Instant>,
    /// Current shutdown expiry or unusable time prohibits ordinary waiting.
    overdue: bool,
    /// Earliest cause, never overwritten by cleanup or status failures.
    failure: Option<FixtureError>,
    /// Original tagged worker cause when it occupies the primary failure slot.
    primary_worker_failure: Option<WorkerFailure>,
    /// Sole distinct later operation failure, never confused with cleanup.
    operation_failure: Option<WorkerError>,
    /// Sole later failed actual completion observation.
    completion_failure: Option<WorkerError>,
    /// Sole later failed `StopPending` operation.
    stop_failure: Option<LifecycleError<NativeError>>,
    /// Sole later failed terminal operation.
    terminal_failure: Option<LifecycleError<NativeError>>,
    /// Final result is extracted at most once; absence never becomes later Clean.
    result_taken: bool,
    /// Invocation-owned finite output, with no effect on lifecycle decisions.
    diagnostics: Diagnostics,
    /// Optional writer retains its own custody through the terminal boundary.
    writer: Option<Delivery<W>>,
    /// First emission of each existing primary/completion/Stop/terminal fact.
    diagnosed_failures: [bool; 4],
    /// Primary and secondary operation facts each route once through event 9.
    diagnosed_operations: [bool; 2],
    /// A wake is bound to this OS thread; the generic owner must not migrate.
    thread_bound: PhantomData<Rc<()>>,
}

impl<R: StatusReporter<Error = NativeError>, P: Worker, W: EventWriter> ServiceInvocation<R, P, W> {
    /// Freeze the last genuine observation before the shared queue handoff.
    /// This computes checked relative metadata and performs no new clock read.
    fn emit(&mut self, event: Event) {
        let observation: DiagnosticObservation = DiagnosticObservation::from_lifecycle(
            self.observed.origin,
            self.observed.last_trusted,
            self.observed.reversed,
        );
        if let Some(writer) = &mut self.writer
            && writer.is_open()
        {
            if let Some(record) = self.diagnostics.emit_for_writer(event, observation) {
                writer.offer(record, &self.wake);
            }
        } else {
            self.diagnostics.emit(event, observation);
        }
    }

    /// Retain kind/code once, preserving primary and each fixed secondary slot.
    fn worker_failed(&mut self, failure: WorkerFailure) {
        self.admission_closed = true;
        if self.primary_worker_failure == Some(failure) {
            return;
        }
        let error: WorkerError = match failure {
            WorkerFailure::Operation(error) | WorkerFailure::Cleanup(error) => error,
        };
        if self.failure.is_none() {
            self.primary_worker_failure = Some(failure);
            self.failure = Some(FixtureError::Worker(error));
        } else {
            let slot: &mut Option<WorkerError> = match failure {
                WorkerFailure::Operation(_) => &mut self.operation_failure,
                WorkerFailure::Cleanup(_) => &mut self.completion_failure,
            };
            if slot.is_none() {
                *slot = Some(error);
            }
        }
        self.publish_failures();
    }

    /// Retain the earliest cause and permanently close every worker admission path.
    fn fail(&mut self, error: FixtureError) {
        self.admission_closed = true;
        if self.failure.is_none() {
            self.failure = Some(error);
        }
        self.publish_failures();
    }

    /// Project each retained failure slot once; never format or consume its cause.
    /// Loss remains in the finite output owner and never becomes a service error.
    fn publish_failures(&mut self) {
        if !self.diagnosed_failures[0]
            && let Some(error) = &self.failure
        {
            self.diagnosed_failures[0] = true;
            self.emit(Event::PrimaryFailure(primary_cause(error)));
        }
        if !self.diagnosed_failures[1]
            && let Some(error) = self.completion_failure
        {
            self.diagnosed_failures[1] = true;
            self.emit(Event::CompletionFailure(worker_cause(error)));
        }
        if !self.diagnosed_failures[2]
            && let Some(error) = &self.stop_failure
        {
            self.diagnosed_failures[2] = true;
            self.emit(Event::StopReportFailure(lifecycle_cause(error)));
        }
        if !self.diagnosed_failures[3]
            && let Some(error) = &self.terminal_failure
        {
            self.diagnosed_failures[3] = true;
            self.emit(Event::TerminalReportFailure(lifecycle_cause(error)));
        }
        if !self.diagnosed_operations[0]
            && let Some(WorkerFailure::Operation(error)) = self.primary_worker_failure
        {
            self.diagnosed_operations[0] = true;
            self.emit(Event::OperationFailure(worker_cause(error)));
        }
        if !self.diagnosed_operations[1]
            && let Some(error) = self.operation_failure
        {
            self.diagnosed_operations[1] = true;
            self.emit(Event::OperationFailure(worker_cause(error)));
        }
    }

    /// Observe trusted time; a fault immediately fences admission before cleanup.
    fn observe(&mut self, clock: &mut impl Clock) -> Option<Instant> {
        match self.observed.observe(clock) {
            Ok(now) => Some(now),
            Err(error) => {
                if self.observed.reversed {
                    self.phase_deadline = None;
                }
                self.fail(error);
                None
            }
        }
    }

    /// Check the original pending deadline without reporting invented progress.
    fn check_pending(&mut self, now: Instant) {
        if let Err(error) = self.owner.check_deadline(now) {
            if error == LifecycleError::DeadlineElapsed {
                self.overdue = true;
            }
            self.fail(FixtureError::Lifecycle(error));
        }
    }

    /// Finish only the retained current charge after its actual acknowledgement.
    fn complete_permit(&mut self) {
        if let Some(permit) = self.permit.take() {
            permit.complete();
        }
    }

    /// Begin preparation under the sole startup charge and observe dormant readiness.
    fn prepare(&mut self, clock: &mut impl Clock, notification: &StopNotification) {
        let Some(now) = self.observe(clock) else {
            return;
        };
        self.check_pending(now);
        if self.failure.is_some() || notification.stop_requested() {
            self.admission_closed = true;
            return;
        }
        if self.permit.is_none() {
            match self.owner.begin_work(now) {
                Ok(permit) => self.permit = Some(permit),
                Err(error) => {
                    self.fail(FixtureError::Lifecycle(error));
                    return;
                }
            }
            self.custody = WorkerCustody::Preparing;
        }
        match self.worker.poll_prepare(&self.wake) {
            Poll::Pending => {
                if notification.stop_requested() {
                    self.admission_closed = true;
                }
                if let Some(now) = self.observe(clock) {
                    self.check_pending(now);
                }
            }
            Poll::Ready(Err(error)) => self.worker_failed(WorkerFailure::Operation(error)),
            Poll::Ready(Ok(())) => {
                self.complete_permit();
                self.custody = WorkerCustody::Prepared;
                // Ready acknowledges completed initialization, never activation.
                if notification.stop_requested() {
                    self.admission_closed = true;
                }
                let Some(now) = self.observe(clock) else {
                    return;
                };
                self.check_pending(now);
                // This fresh read admits the continuation after time acquisition;
                // a Stop published during the observation must close admission.
                if self.admission_closed || notification.stop_requested() {
                    self.admission_closed = true;
                    return;
                }
                let Some(deadline) = now.checked_add(LAB_RUNNING) else {
                    self.fail(FixtureError::ClockOverflow);
                    return;
                };
                match self.owner.ready(now) {
                    Ok(()) => {
                        self.run_deadline = Some(deadline);
                        self.phase = WorkerPhase::Running;
                        self.emit(Event::RunningAcknowledged);
                    }
                    Err(error) => self.fail(FixtureError::Lifecycle(error)),
                }
            }
        }
    }

    /// Activate and observe health under the separate acknowledged Running charge.
    fn run(&mut self, clock: &mut impl Clock, notification: &StopNotification) {
        if notification.stop_requested() {
            self.admission_closed = true;
        }
        let Some(now) = self.observe(clock) else {
            return;
        };
        let Some(deadline) = self.run_deadline else {
            self.fail(FixtureError::OutcomeUnavailable);
            return;
        };
        if let Err(error) = running_remaining(deadline, now) {
            self.overdue = true;
            self.fail(error);
            return;
        }
        if self.admission_closed || notification.stop_requested() {
            self.admission_closed = true;
            return;
        }
        if self.custody == WorkerCustody::Prepared {
            match self.owner.begin_work(now) {
                Ok(permit) => self.permit = Some(permit),
                Err(error) => {
                    self.fail(FixtureError::Lifecycle(error));
                    return;
                }
            }
            self.custody = WorkerCustody::Active;
            if let Err(error) = self.worker.activate() {
                self.worker_failed(WorkerFailure::Operation(error));
                return;
            }
            // Activation is a boundary: recheck Stop and time before health/wait.
            if notification.stop_requested() {
                self.admission_closed = true;
            }
            let Some(now) = self.observe(clock) else {
                return;
            };
            if let Err(error) = running_remaining(deadline, now) {
                self.overdue = true;
                self.fail(error);
                return;
            }
            if self.admission_closed || notification.stop_requested() {
                self.admission_closed = true;
                return;
            }
        }
        #[cfg(any(windows, test))]
        if let Some(mut reader) = self.broker_clock_reader.take() {
            reader.poll(self.observed.last_trusted, || {
                self.broker_bracket_edge(clock, notification, deadline)
            });
            self.broker_clock_reader = Some(reader);
            if self.admission_closed {
                return;
            }
        }
        #[cfg(test)]
        {
            self.poll_broker_probe(clock, notification);
            if self.admission_closed {
                return;
            }
        }
        if let Err(error) = self.worker.poll_running(&self.wake) {
            self.worker_failed(error);
            return;
        }
        // A bounded health method can still consume time. Reobserve before
        // deriving the remaining absolute wait, preserving Stop before the read.
        if notification.stop_requested() {
            self.admission_closed = true;
        }
        if let Some(now) = self.observe(clock)
            && let Err(error) = running_remaining(deadline, now)
        {
            self.overdue = true;
            self.fail(error);
        }
    }

    /// Fence first, attempt `StopPending` once, then request bounded cancellation.
    fn begin_stop(&mut self, clock: &mut impl Clock) {
        self.admission_closed = true;
        #[cfg(any(windows, test))]
        if let Some(reader) = &self.broker_clock_reader {
            reader.close();
        }
        if let Some(writer) = &mut self.writer {
            writer.fence();
        }
        self.emit(Event::AdmissionClosed);
        let now = self.observed.cleanup_observation(clock, &mut self.failure);
        self.publish_failures();
        if self.owner.state() == Some(ServiceState::StartPending) {
            self.check_pending(now);
        }
        self.phase_deadline = if self.observed.reversed {
            None
        } else {
            now.checked_add(Duration::from_millis(u64::from(LAB_PENDING_MS)))
        };
        // A valid observation anchors the separately bounded shutdown allowance.
        // Reversal leaves only historical lifecycle bookkeeping, not an allowance.
        self.overdue = self.observed.reversed || self.phase_deadline.is_none();
        if let Err(error) = self.owner.request_stop(now) {
            if self.failure.is_none() {
                self.failure = Some(FixtureError::Lifecycle(error));
            } else {
                self.stop_failure = Some(error);
            }
        }
        self.publish_failures();
        self.phase = WorkerPhase::Stopping;
        if self.custody != WorkerCustody::Dormant {
            self.custody = WorkerCustody::Cancelling;
            self.worker.request_cancel(&self.wake);
        }
    }

    /// Observe physical worker completion and settle its original charge once.
    fn poll_stop_worker(&mut self, clock: &mut impl Clock) -> bool {
        if !matches!(
            self.custody,
            WorkerCustody::Dormant | WorkerCustody::Finished
        ) {
            match self.worker.poll_finished(&self.wake) {
                pending @ (WorkerFinish::Pending | WorkerFinish::JoinTailPending) => {
                    self.custody = if matches!(pending, WorkerFinish::JoinTailPending) {
                        WorkerCustody::Joining
                    } else {
                        WorkerCustody::Cancelling
                    };
                    if let Some(now) = self.observe(clock) {
                        if self.owner.state() == Some(ServiceState::StopPending) {
                            self.check_pending(now);
                        }
                        if self.phase_deadline.is_none_or(|deadline| now >= deadline) {
                            self.overdue = true;
                        }
                    } else {
                        self.overdue = true;
                    }
                    return false;
                }
                WorkerFinish::Finished(result) => {
                    self.custody = WorkerCustody::Finished;
                    self.complete_permit();
                    if let Some(error) = result.operation_failure {
                        self.worker_failed(WorkerFailure::Operation(error));
                    }
                    if let Some(error) = result.cleanup_failure {
                        self.worker_failed(WorkerFailure::Cleanup(error));
                    }
                    self.publish_failures();
                }
            }
        }
        true
    }

    /// Observe both independently owned completions before attempting Stopped.
    fn stop(&mut self, clock: &mut impl Clock) {
        let worker_finished: bool = self.poll_stop_worker(clock);
        if !worker_finished && self.writer.is_none() {
            return;
        }
        let now: Instant = self.observe_stop(clock);
        let (now, writer_finished): (Instant, bool) = self.poll_stop_writer(clock, now);
        if !worker_finished || !writer_finished {
            return;
        }
        let outcome = if self.failure.is_some() {
            StopOutcome::Failed(NonZeroU32::MIN)
        } else {
            StopOutcome::Clean
        };
        if let Err(error) = self.owner.finish_stop(now, outcome) {
            if self.failure.is_none() {
                self.failure = Some(FixtureError::Lifecycle(error));
            } else {
                self.terminal_failure = Some(error);
            }
        }
        self.publish_failures();
        self.phase = WorkerPhase::Finished;
        self.emit(Event::Finished);
    }

    /// Give the same writer the original absolute deadline and check time after
    /// the effect. Crossing the cutoff while pending immediately retires its work.
    fn poll_stop_writer(&mut self, clock: &mut impl Clock, now: Instant) -> (Instant, bool) {
        let mode: CloseMode = match self.phase_deadline {
            Some(deadline) if !self.overdue => CloseMode::Drain(deadline),
            _ => CloseMode::Retire,
        };
        let Some(writer) = &mut self.writer else {
            return (now, true);
        };
        if writer.is_closed() {
            return (now, true);
        }
        let finished: bool = writer.poll_close(mode, &self.wake).is_ready();
        let now: Instant = self.observe_stop(clock);
        if !finished
            && self.overdue
            && matches!(mode, CloseMode::Drain(_))
            && let Some(writer) = &mut self.writer
        {
            let finished: bool = writer.poll_close(CloseMode::Retire, &self.wake).is_ready();
            return (self.observe_stop(clock), finished);
        }
        (now, finished)
    }

    /// Observe the original Stop deadline for failure bookkeeping and retirement.
    /// Reversal permits only historical failed-stop use and retires writer work.
    fn observe_stop(&mut self, clock: &mut impl Clock) -> Instant {
        let now: Instant = self.observed.cleanup_observation(clock, &mut self.failure);
        if self.observed.reversed {
            self.phase_deadline = None;
        }
        self.overdue |= self
            .phase_deadline
            .is_none_or(|deadline: Instant| now >= deadline);
        self.publish_failures();
        if self.owner.state() == Some(ServiceState::StopPending) {
            self.check_pending(now);
        }
        now
    }

    /// Take one finite, nonblocking step while retaining every original owner.
    /// Every state-changing wake must be published before the worker notification.
    /// Repeated Stop, spurious wakes and overdue polls never renew a deadline.
    fn poll_step(
        &mut self,
        clock: &mut impl Clock,
        notification: &StopNotification,
    ) -> InvocationPoll {
        if self.phase == WorkerPhase::Finished {
            return InvocationPoll::Finished;
        }
        if notification.stop_requested() {
            self.admission_closed = true;
        }
        if self.phase == WorkerPhase::Preparing && !self.admission_closed {
            self.prepare(clock, notification);
        }
        if self.phase == WorkerPhase::Running {
            self.run(clock, notification);
        }
        if notification.stop_requested() {
            self.admission_closed = true;
        }
        if self.admission_closed && self.phase != WorkerPhase::Stopping {
            self.begin_stop(clock);
        }
        if self.phase == WorkerPhase::Stopping {
            self.stop(clock);
        }
        if self.phase == WorkerPhase::Finished {
            return InvocationPoll::Finished;
        }
        #[cfg(test)]
        if matches!(
            self.custody,
            WorkerCustody::Dormant | WorkerCustody::Finished
        ) && let Some(release) = self.writer.as_ref().and_then(Delivery::release_unconfirmed)
        {
            return InvocationPoll::WriterReleaseUnconfirmed(release);
        }
        if self.custody == WorkerCustody::Joining
            || self.writer.as_ref().is_some_and(Delivery::needs_join_tail)
        {
            // The terminal marker permits only this relative cleanup observation.
            // Reversed time supplies no trusted deadline or arithmetic operand.
            let wait: Duration = if self.overdue || self.observed.reversed {
                JOIN_TAIL_WAIT
            } else {
                match self.phase_deadline.and_then(|deadline: Instant| {
                    deadline.checked_duration_since(self.observed.last_trusted)
                }) {
                    Some(remaining) if !remaining.is_zero() => remaining.min(JOIN_TAIL_WAIT),
                    _ => {
                        self.overdue = true;
                        self.fail(FixtureError::Lifecycle(LifecycleError::DeadlineElapsed));
                        JOIN_TAIL_WAIT
                    }
                }
            };
            return InvocationPoll::JoinTail(wait);
        }
        if self.overdue {
            return InvocationPoll::Overdue;
        }
        let deadline = if self.phase == WorkerPhase::Running {
            self.run_deadline
        } else {
            self.phase_deadline
        };
        let Some(remaining) = deadline
            .and_then(|deadline| deadline.checked_duration_since(self.observed.last_trusted))
        else {
            self.overdue = true;
            return InvocationPoll::Overdue;
        };
        if remaining.is_zero() {
            self.overdue = true;
            return InvocationPoll::Overdue;
        }
        #[cfg(any(windows, test))]
        if self.phase == WorkerPhase::Running
            && let Some(due) =
                self.broker_clock_reader
                    .as_ref()
                    .and_then(|reader: &NativeBrokerReader| {
                        reader.sample_wait(self.observed.last_trusted)
                    })
        {
            return InvocationPoll::Pending(remaining.min(due));
        }
        InvocationPoll::Pending(remaining)
    }

    /// Preserve the exact lifecycle step, then attempt one queued original per route.
    /// Every return path, including overdue and finished, has the same pump bound.
    pub(super) fn poll(
        &mut self,
        clock: &mut impl Clock,
        notification: &StopNotification,
    ) -> InvocationPoll {
        let poll: InvocationPoll = self.poll_step(clock, notification);
        self.diagnostics.pump_one();
        poll
    }

    /// Consume the result once, only after actual completion and terminal attempt.
    /// Secondary cleanup/report facts are wrapped once with one bounded allocation.
    pub(super) fn take_receipt(&mut self) -> Option<HostReceipt> {
        if self.phase != WorkerPhase::Finished || self.result_taken {
            return None;
        }
        self.result_taken = true;
        let result: Result<(), FixtureError> = self.take_actual_result();
        Some(HostReceipt {
            actual_result: result,
            diagnostics: self.diagnostics.finish(),
            #[cfg(any(windows, test))]
            broker_clock_read: self
                .broker_clock_reader
                .as_ref()
                .and_then(NativeBrokerReader::history),
            #[cfg(any(windows, test))]
            broker_clock_break: self
                .broker_clock_reader
                .as_ref()
                .and_then(NativeBrokerReader::last_discontinuity),
            #[cfg(test)]
            writer_close: self.writer.as_ref().and_then(Delivery::outcome),
            #[cfg(test)]
            broker_observation: self.observed_broker_probe(),
        })
    }

    /// Apply the owner-clock, cutoff and Stop rules at both read edges.
    ///
    /// # Errors
    /// Retire an inadmissible bracket and preserve the owning failure and fence.
    #[cfg(any(windows, test))]
    fn broker_bracket_edge(
        &mut self,
        clock: &mut impl Clock,
        notification: &StopNotification,
        deadline: Instant,
    ) -> Result<Instant, ClockReadRetirement> {
        if notification.stop_requested() {
            self.admission_closed = true;
        }
        let now: Instant = self.observe(clock).ok_or(ClockReadRetirement::Clock)?;
        if let Err(error) = running_remaining(deadline, now) {
            self.overdue = true;
            self.fail(error);
            return Err(ClockReadRetirement::Deadline);
        }
        if self.admission_closed || notification.stop_requested() {
            self.admission_closed = true;
            return Err(ClockReadRetirement::Stop);
        }
        Ok(now)
    }

    /// Move the unchanged original primary and secondary objects into their result.
    ///
    /// # Errors
    /// Returns the retained primary failure and any independently observed cleanup failures.
    fn take_actual_result(&mut self) -> Result<(), FixtureError> {
        let Some(primary) = self.failure.take() else {
            return Ok(());
        };
        if self.operation_failure.is_none()
            && self.completion_failure.is_none()
            && self.stop_failure.is_none()
            && self.terminal_failure.is_none()
        {
            return Err(primary);
        }
        Err(FixtureError::Cleanup(ServiceFailure::new(
            primary,
            self.operation_failure.take(),
            self.completion_failure.take(),
            self.stop_failure.take(),
            self.terminal_failure.take(),
        )))
    }

    /// Read intended state from this invocation's original lifecycle.
    #[cfg(test)]
    pub(super) fn state(&self) -> Option<ServiceState> {
        self.owner.state()
    }

    /// Read the original invocation's actual outstanding work charge.
    #[cfg(test)]
    pub(super) fn outstanding_work(&self) -> u32 {
        self.owner.outstanding_work()
    }

    /// Observe the sticky private admission fence without gaining an admission API.
    #[cfg(test)]
    pub(super) const fn admission_closed(&self) -> bool {
        self.admission_closed
    }

    /// Inspect the first retained cause before final bounded result projection.
    #[cfg(test)]
    pub(super) const fn failure(&self) -> Option<&FixtureError> {
        self.failure.as_ref()
    }
}

impl<R: StatusReporter<Error = NativeError>, P: Worker, W: EventWriter> Drop
    for ServiceInvocation<R, P, W>
{
    /// Refuse unresolved whole-owner destruction before worker/writer/reporter fields drop.
    /// Process abort is fixture-only; it promises no native termination recovery.
    fn drop(&mut self) {
        if !matches!(
            self.custody,
            WorkerCustody::Dormant | WorkerCustody::Finished
        ) || self.writer.as_ref().is_some_and(Delivery::unresolved)
        {
            std::process::abort();
        }
    }
}

/// Register, start and perform existing token admission before worker preparation.
/// A failed admitted startup remains owned for Stop/terminal reporting; registration
/// failure constructs no invocation. No worker method is invoked here.
/// The supplied worker must be dormant with no already-started job; only a later
/// admitted `poll_prepare` may begin its first operation.
///
/// # Errors
/// Preserve registration or fixed laboratory budget construction failure.
#[expect(
    clippy::result_large_err,
    reason = "the frozen startup contract retains two fixed 16-record histories and original result without adding a failure-path heap allocation"
)]
pub(super) fn start_invocation<A: StatusFactory, C: Clock, P: Worker>(
    role: LabRole,
    argc: u32,
    factory: &mut A,
    token_observer: &mut impl TokenObserver,
    clock: &mut C,
    notification: &StopNotification,
    worker: P,
) -> Result<ServiceInvocation<A::Reporter, P>, HostReceipt> {
    start_invocation_with_diagnostics(
        role,
        argc,
        factory,
        token_observer,
        clock,
        notification,
        (
            worker,
            Sinks::new(MemoryConsumer::new(ConsumerState::Available), None),
        ),
    )
}

/// Bind concrete finite sinks at the actual invocation composition boundary.
/// The worker is dormant under the same contract as `start_invocation`; sinks
/// contain no callbacks, paths or control authority. Role binds here, so a supplied
/// output composition cannot label the invocation as a different service role.
///
/// # Errors
/// Preserve pre-invocation registration/budget failure and its honest bounded
/// diagnostic snapshot without inventing a lifecycle invocation or observation.
#[expect(
    clippy::result_large_err,
    reason = "the frozen startup contract retains two fixed 16-record histories and original result without adding a failure-path heap allocation"
)]
pub(super) fn start_invocation_with_diagnostics<A: StatusFactory, C: Clock, P: Worker>(
    role: LabRole,
    argc: u32,
    factory: &mut A,
    token_observer: &mut impl TokenObserver,
    clock: &mut C,
    notification: &StopNotification,
    composition: (P, Sinks),
) -> Result<ServiceInvocation<A::Reporter, P>, HostReceipt> {
    let (worker, sinks): (P, Sinks) = composition;
    start_invocation_with_writer(
        role,
        argc,
        factory,
        token_observer,
        clock,
        notification,
        (worker, sinks, None),
    )
}

/// Bind one optional dormant writer to the actual registered lifecycle owner.
///
/// # Errors
/// Preserve startup failure without ever offering work to an unbound writer.
#[expect(
    clippy::result_large_err,
    reason = "return the original bounded startup receipt without allocating on failure"
)]
fn start_invocation_with_writer<A: StatusFactory, C: Clock, P: Worker, W: EventWriter>(
    role: LabRole,
    argc: u32,
    factory: &mut A,
    token_observer: &mut impl TokenObserver,
    clock: &mut C,
    notification: &StopNotification,
    composition: (P, Sinks, Option<W>),
) -> Result<ServiceInvocation<A::Reporter, P, W>, HostReceipt> {
    let (worker, sinks, writer): (P, Sinks, Option<W>) = composition;
    let diagnostics: Diagnostics = Diagnostics::new(role, sinks);
    let reporter: A::Reporter = match factory.register(role) {
        Ok(reporter) => reporter,
        Err(error) => {
            return Err(HostReceipt::startup_failure(
                FixtureError::Native(error),
                diagnostics,
            ));
        }
    };
    let budget: PendingBudget = match PendingBudget::new(LAB_WAIT_HINT_MS, LAB_PENDING_MS) {
        Ok(budget) => budget,
        Err(error) => {
            return Err(HostReceipt::startup_failure(
                FixtureError::Budget(error),
                diagnostics,
            ));
        }
    };
    let mut owner = ServiceLifecycle::new(
        role.service_role(),
        reporter,
        budget,
        budget,
        NonZeroU32::MIN,
    );
    let mut observed = ObservedClock::new(clock);
    let mut failure = owner
        .start(observed.last_trusted)
        .map_err(FixtureError::Lifecycle)
        .err();
    let phase_deadline = observed
        .last_trusted
        .checked_add(Duration::from_millis(u64::from(LAB_PENDING_MS)));
    if failure.is_none() && argc > 1 {
        failure = Some(FixtureError::UnsupportedArguments);
    }
    if failure.is_none() && !notification.stop_requested() {
        let capture = observed.observe(clock).and_then(|now| {
            let permit = owner.begin_work(now).map_err(FixtureError::Lifecycle)?;
            let result = token_observer.observe();
            // This synchronous return includes actual capture and explicit cleanup.
            permit.complete();
            result.map_err(FixtureError::TokenObservation)
        });
        match capture {
            Ok(snapshot) => {
                if let Err(error) = snapshot.require_role(role.service_role()) {
                    failure = Some(FixtureError::PrivilegeAdmission(error));
                }
            }
            Err(error) => failure = Some(error),
        }
    }
    let admission_closed = failure.is_some() || notification.stop_requested();
    let writer: Option<Delivery<W>> = if admission_closed {
        None
    } else {
        writer.map(Delivery::new)
    };
    let mut invocation = ServiceInvocation {
        owner,
        worker,
        permit: None,
        phase: WorkerPhase::Preparing,
        admission_closed,
        custody: WorkerCustody::Dormant,
        wake: WorkerWake::current(),
        #[cfg(any(windows, test))]
        broker_clock_reader: None,
        #[cfg(test)]
        broker_probe: None,
        observed,
        phase_deadline,
        run_deadline: None,
        overdue: false,
        failure,
        primary_worker_failure: None,
        operation_failure: None,
        completion_failure: None,
        stop_failure: None,
        terminal_failure: None,
        result_taken: false,
        diagnostics,
        writer,
        diagnosed_failures: [false; 4],
        diagnosed_operations: [false; 2],
        thread_bound: PhantomData,
    };
    invocation.emit(Event::StartObserved);
    invocation.publish_failures();
    Ok(invocation)
}

/// Construct the existing control-only laboratory invocation.
/// An overdue step retains the original lifecycle, reporter, worker and charge;
/// this function cannot return `ServiceMain` until Finished is physically observed.
///
/// # Errors
/// Return the original failure plus bounded cleanup/status facts, after completion.
pub(super) fn run_service<A: StatusFactory, C: Clock, W: Wait>(
    role: LabRole,
    argc: u32,
    factory: &mut A,
    token_observer: &mut impl TokenObserver,
    clock: &mut C,
    wait: &mut W,
    notification: &StopNotification,
) -> HostReceipt {
    let invocation: ServiceInvocation<A::Reporter, InertWorker> = match start_invocation(
        role,
        argc,
        factory,
        token_observer,
        clock,
        notification,
        InertWorker::new(),
    ) {
        Ok(invocation) => invocation,
        Err(receipt) => return receipt,
    };
    drive_service(invocation, clock, wait, notification)
}

/// Bind the real read-only broker composition to the same serialized service owner.
#[cfg(any(windows, test))]
pub(super) fn run_broker_service<A: StatusFactory, C: Clock, W: Wait>(
    argc: u32,
    factory: &mut A,
    token_observer: &mut impl TokenObserver,
    clock: &mut C,
    wait: &mut W,
    notification: &StopNotification,
    reader: NativeBrokerReader,
) -> HostReceipt {
    let mut invocation: ServiceInvocation<A::Reporter, InertWorker> = match start_invocation(
        LabRole::ClockBroker,
        argc,
        factory,
        token_observer,
        clock,
        notification,
        InertWorker::new(),
    ) {
        Ok(invocation) => invocation,
        Err(receipt) => return receipt,
    };
    invocation.broker_clock_reader = Some(reader);
    drive_service(invocation, clock, wait, notification)
}

/// Retain the same lifecycle and charge through every poll and actual finish.
fn drive_service<R: StatusReporter<Error = NativeError>>(
    mut invocation: ServiceInvocation<R, InertWorker>,
    clock: &mut impl Clock,
    wait: &mut impl Wait,
    notification: &StopNotification,
) -> HostReceipt {
    loop {
        match invocation.poll(clock, notification) {
            InvocationPoll::Pending(remaining) => wait.park(remaining),
            InvocationPoll::JoinTail(relative) => wait.park(relative),
            InvocationPoll::Overdue => wait.park_until_notified(),
            #[cfg(test)]
            // This driver selects no writer. The test driver observes this case directly.
            InvocationPoll::WriterReleaseUnconfirmed(_) => std::process::abort(),
            InvocationPoll::Finished => {
                return invocation.take_receipt().unwrap_or_else(|| HostReceipt {
                    actual_result: Err(FixtureError::OutcomeUnavailable),
                    diagnostics: invocation.diagnostics.finish(),
                    #[cfg(any(windows, test))]
                    broker_clock_read: None,
                    #[cfg(any(windows, test))]
                    broker_clock_break: None,
                    #[cfg(test)]
                    writer_close: None,
                    #[cfg(test)]
                    broker_observation: None,
                });
            }
        }
    }
}

/// Require positive time before the absolute deadline from the Ready observation.
/// `now` comes from the serialized checked clock; this pure guard neither reads
/// time, renews the deadline nor admits work by itself.
///
/// # Errors
/// Reject equality or time after the fixed deadline.
fn running_remaining(deadline: Instant, now: Instant) -> Result<Duration, FixtureError> {
    let remaining = deadline
        .checked_duration_since(now)
        .ok_or(FixtureError::RunLimitElapsed)?;
    if remaining.is_zero() {
        return Err(FixtureError::RunLimitElapsed);
    }
    Ok(remaining)
}

#[cfg(test)]
#[path = "host_tests.rs"]
mod tests;

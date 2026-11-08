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
    privilege_observation::PrivilegeSnapshot,
    scm::StopNotification,
    service::{
        LifecycleError, PendingBudget, ServiceLifecycle, ServiceState, StatusReporter, StopOutcome,
        WorkPermit,
    },
};

use crate::{
    FixtureError, LabRole, NativeError, TokenObservationFailure,
    error::{ServiceFailure, WorkerError},
};

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
    fn poll_running(&mut self, wake: &WorkerWake) -> Result<(), WorkerError>;

    /// Request cancellation once after admission is permanently fenced.
    /// This prompt request itself provides no completion or cleanup acknowledgement.
    fn request_cancel(&mut self, wake: &WorkerWake);

    /// Finished acknowledges actual jobs and resolved cleanup custody, including
    /// an acknowledged recovery handoff if required. Unresolved cleanup is Pending.
    /// A failed completion retains its typed cause alongside earlier failures.
    fn poll_finished(&mut self, wake: &WorkerWake) -> Poll<Result<(), WorkerError>>;
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
    fn poll_running(&mut self, _wake: &WorkerWake) -> Result<(), WorkerError> {
        Ok(())
    }

    /// Publish the inert completion condition before its retained owner wake.
    fn request_cancel(&mut self, wake: &WorkerWake) {
        self.cancelled = true;
        wake.notify();
    }

    /// Acknowledge completion only after the cancellation condition was published.
    fn poll_finished(&mut self, _wake: &WorkerWake) -> Poll<Result<(), WorkerError>> {
        if self.cancelled {
            Poll::Ready(Ok(()))
        } else {
            Poll::Pending
        }
    }
}

/// Explicit source-card start/stop wait hint in milliseconds, not a product default.
const LAB_WAIT_HINT_MS: u32 = 1000;
/// Explicit source-card start/stop phase allowance, not a product default.
const LAB_PENDING_MS: u32 = 10_000;
/// Explicit maximum useful laboratory control-loop interval.
const LAB_RUNNING: Duration = Duration::from_millis(60_000);

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
    /// Sticky refusal of every subsequent ordinary observation.
    reversed: bool,
}

impl ObservedClock {
    /// Establish the first observation without claiming an earlier time origin.
    fn new(source: &mut impl Clock) -> Self {
        let last_trusted = source.now();
        Self {
            last_trusted,
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
    outcome: &'a OnceLock<Result<(), FixtureError>>,
    dispatch_failure: &'a OnceLock<FixtureError>,
) -> Result<(), &'a FixtureError> {
    if let Err(error) = dispatch_once(role, attempted, driver) {
        if error == FixtureError::AlreadyDispatched {
            return Err(&FixtureError::AlreadyDispatched);
        }
        return Err(dispatch_failure.get_or_init(|| error));
    }
    match outcome.get() {
        Some(Ok(())) => Ok(()),
        Some(Err(error)) => Err(error),
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

/// Finite result of borrowing the same persistent invocation for one owner step.
/// Dropping this value neither drops the invocation nor releases charged work.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum InvocationPoll {
    /// Await a coalesced worker/Stop notification, bounded by the fixed deadline.
    Pending(Duration),
    /// A deadline or unusable clock ended ordinary waiting; retain the same owner.
    Overdue,
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
    /// Finished acknowledged resolved actual work and cleanup custody.
    Finished,
}

/// One thread-bound invocation retaining the original status and completion owners.
/// All work admission stays private and is fenced before fallible Stop handling.
/// The driver keeps this owner through Finished; unresolved whole-owner destruction
/// aborts this inert fixture before any field destructor can discard custody.
/// This guard selects no production native-job termination or recovery policy.
pub(super) struct ServiceInvocation<R: StatusReporter<Error = NativeError>, P: Worker> {
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
    /// Sole later failed actual completion observation.
    completion_failure: Option<WorkerError>,
    /// Sole later failed `StopPending` operation.
    stop_failure: Option<LifecycleError<NativeError>>,
    /// Sole later failed terminal operation.
    terminal_failure: Option<LifecycleError<NativeError>>,
    /// Final result is extracted at most once; absence never becomes later Clean.
    result_taken: bool,
    /// A wake is bound to this OS thread; the generic owner must not migrate.
    thread_bound: PhantomData<Rc<()>>,
}

impl<R: StatusReporter<Error = NativeError>, P: Worker> ServiceInvocation<R, P> {
    /// Retain the earliest cause and permanently close every worker admission path.
    fn fail(&mut self, error: FixtureError) {
        self.admission_closed = true;
        if self.failure.is_none() {
            self.failure = Some(error);
        }
    }

    /// Observe trusted time; a fault immediately fences admission before cleanup.
    fn observe(&mut self, clock: &mut impl Clock) -> Option<Instant> {
        match self.observed.observe(clock) {
            Ok(now) => Some(now),
            Err(error) => {
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
            Poll::Ready(Err(error)) => self.fail(FixtureError::Worker(error)),
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
                self.fail(FixtureError::Worker(error));
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
        if let Err(error) = self.worker.poll_running(&self.wake) {
            self.fail(FixtureError::Worker(error));
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
        let now = self.observed.cleanup_observation(clock, &mut self.failure);
        if self.owner.state() == Some(ServiceState::StartPending) {
            self.check_pending(now);
        }
        self.phase_deadline = now.checked_add(Duration::from_millis(u64::from(LAB_PENDING_MS)));
        // The failed preceding phase remains the primary cause. Its expiry does
        // not remove the new, separately bounded shutdown observation allowance.
        self.overdue = self.observed.reversed || self.phase_deadline.is_none();
        if let Err(error) = self.owner.request_stop(now) {
            if self.failure.is_none() {
                self.failure = Some(FixtureError::Lifecycle(error));
            } else {
                self.stop_failure = Some(error);
            }
        }
        self.phase = WorkerPhase::Stopping;
        if self.custody != WorkerCustody::Dormant {
            self.custody = WorkerCustody::Cancelling;
            self.worker.request_cancel(&self.wake);
        }
    }

    /// Observe actual completion before releasing a charge or attempting Stopped.
    fn stop(&mut self, clock: &mut impl Clock) {
        if self.custody != WorkerCustody::Dormant {
            match self.worker.poll_finished(&self.wake) {
                Poll::Pending => {
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
                    return;
                }
                Poll::Ready(result) => {
                    self.custody = WorkerCustody::Finished;
                    self.complete_permit();
                    if let Err(error) = result {
                        if self.failure.is_none() {
                            self.fail(FixtureError::Worker(error));
                        } else {
                            self.completion_failure = Some(error);
                        }
                    }
                }
            }
        }
        let now = self.observed.cleanup_observation(clock, &mut self.failure);
        if self.owner.state() == Some(ServiceState::StopPending) {
            self.check_pending(now);
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
        self.phase = WorkerPhase::Finished;
    }

    /// Take one finite, nonblocking step while retaining every original owner.
    /// Every state-changing wake must be published before the worker notification.
    /// Repeated Stop, spurious wakes and overdue polls never renew a deadline.
    pub(super) fn poll(
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
        InvocationPoll::Pending(remaining)
    }

    /// Consume the result once, only after actual completion and terminal attempt.
    /// Secondary cleanup/report facts are wrapped once with one bounded allocation.
    pub(super) fn take_result(&mut self) -> Option<Result<(), FixtureError>> {
        if self.phase != WorkerPhase::Finished || self.result_taken {
            return None;
        }
        self.result_taken = true;
        let Some(primary) = self.failure.take() else {
            return Some(Ok(()));
        };
        if self.completion_failure.is_none()
            && self.stop_failure.is_none()
            && self.terminal_failure.is_none()
        {
            return Some(Err(primary));
        }
        Some(Err(FixtureError::Cleanup(ServiceFailure::new(
            primary,
            self.completion_failure.take(),
            self.stop_failure.take(),
            self.terminal_failure.take(),
        ))))
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

impl<R: StatusReporter<Error = NativeError>, P: Worker> Drop for ServiceInvocation<R, P> {
    /// Refuse unresolved whole-owner destruction before worker/reporter fields drop.
    /// Process abort is fixture-only; it promises no native termination recovery.
    fn drop(&mut self) {
        if !matches!(
            self.custody,
            WorkerCustody::Dormant | WorkerCustody::Finished
        ) {
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
pub(super) fn start_invocation<A: StatusFactory, C: Clock, P: Worker>(
    role: LabRole,
    argc: u32,
    factory: &mut A,
    token_observer: &mut impl TokenObserver,
    clock: &mut C,
    notification: &StopNotification,
    worker: P,
) -> Result<ServiceInvocation<A::Reporter, P>, FixtureError> {
    let reporter = factory.register(role).map_err(FixtureError::Native)?;
    let budget =
        PendingBudget::new(LAB_WAIT_HINT_MS, LAB_PENDING_MS).map_err(FixtureError::Budget)?;
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
                if role == LabRole::Engine
                    && let Err(error) = snapshot.require_engine_absence()
                {
                    failure = Some(FixtureError::PrivilegeAdmission(error));
                }
            }
            Err(error) => failure = Some(error),
        }
    }
    let admission_closed = failure.is_some() || notification.stop_requested();
    Ok(ServiceInvocation {
        owner,
        worker,
        permit: None,
        phase: WorkerPhase::Preparing,
        admission_closed,
        custody: WorkerCustody::Dormant,
        wake: WorkerWake::current(),
        observed,
        phase_deadline,
        run_deadline: None,
        overdue: false,
        failure,
        completion_failure: None,
        stop_failure: None,
        terminal_failure: None,
        result_taken: false,
        thread_bound: PhantomData,
    })
}

/// Drive the same actual invocation for both inert native executable compositions.
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
) -> Result<(), FixtureError> {
    let mut invocation = start_invocation(
        role,
        argc,
        factory,
        token_observer,
        clock,
        notification,
        InertWorker::new(),
    )?;
    loop {
        match invocation.poll(clock, notification) {
            InvocationPoll::Pending(remaining) => wait.park(remaining),
            InvocationPoll::Overdue => wait.park_until_notified(),
            InvocationPoll::Finished => {
                return invocation
                    .take_result()
                    .unwrap_or(Err(FixtureError::OutcomeUnavailable));
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

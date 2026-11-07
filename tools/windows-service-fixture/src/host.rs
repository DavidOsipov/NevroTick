// SPDX-License-Identifier: GPL-2.0-only
//! Actual shared fixture owner, with narrow registration, clock and wait seams.
//!
//! This actual owner is used by both native executable compositions.
//! Tests substitute only the actual API/time/wait observations; they do not
//! impersonate native SCM evidence. No native job, clock policy or protected
//! product work is admitted. The existing one-slot lifecycle capacity owns the
//! synchronous capture and genuine control loop through sequential permits.

use std::{
    num::NonZeroU32,
    sync::{
        OnceLock,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use windows_platform::{
    privilege_observation::PrivilegeSnapshot,
    scm::StopNotification,
    service::{PendingBudget, ServiceLifecycle, ServiceState, StatusReporter, StopOutcome},
};

use crate::{FixtureError, LabRole, NativeError, TokenObservationFailure};

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
struct ObservedClock<'a, C> {
    /// Actual observation source, never read after a detected reversal.
    source: &'a mut C,
    /// Last observation actually accepted in nondecreasing order.
    last_trusted: Instant,
    /// Sticky refusal of every subsequent ordinary observation.
    reversed: bool,
}

impl<'a, C: Clock> ObservedClock<'a, C> {
    /// Establish the first observation without claiming an earlier time origin.
    fn new(source: &'a mut C) -> Self {
        let last_trusted = source.now();
        Self {
            source,
            last_trusted,
            reversed: false,
        }
    }

    /// Admit only a genuinely observed nondecreasing instant.
    ///
    /// # Errors
    /// Latch reversal before returning; a latched fault performs no further read.
    fn observe(&mut self) -> Result<Instant, FixtureError> {
        if self.reversed {
            return Err(FixtureError::ClockReversed);
        }
        let now = self.source.now();
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
    fn cleanup_observation(&mut self, failure: &mut Option<FixtureError>) -> Instant {
        match self.observe() {
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

/// Run the actual inert fixture lifecycle after its role-specific `ServiceMain`.
/// Argument zero is ignored metadata; no vector or string is read or copied.
/// Cleanup completes the actual control-loop permit before terminal reporting.
/// One absolute allowance from Ready is checked before work admission and waits.
/// Pending deadlines are checked before phase replacement and terminal outcome.
///
/// # Errors
/// Preserve the first register/start/argument/time/wait-limit/status failure.
/// Still attempt bounded owned cleanup; never conceal a failure with Clean.
pub(super) fn run_service<A: StatusFactory, C: Clock, W: Wait>(
    role: LabRole,
    argc: u32,
    factory: &mut A,
    token_observer: &mut impl TokenObserver,
    clock: &mut C,
    wait: &mut W,
    notification: &StopNotification,
) -> Result<(), FixtureError> {
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
    let mut control_loop = None;
    if failure.is_none() && argc > 1 {
        failure = Some(FixtureError::UnsupportedArguments);
    }
    if failure.is_none() && !notification.stop_requested() {
        let capture = observed.observe().and_then(|now| {
            let permit = owner.begin_work(now).map_err(FixtureError::Lifecycle)?;
            let result = token_observer.observe();
            // Synchronous return includes actual capture and explicit handle cleanup.
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
    if failure.is_none() && !notification.stop_requested() {
        let readiness = observed.observe().and_then(|now| {
            owner.check_deadline(now).map_err(FixtureError::Lifecycle)?;
            let deadline = now
                .checked_add(LAB_RUNNING)
                .ok_or(FixtureError::ClockOverflow)?;
            owner.ready(now).map_err(FixtureError::Lifecycle)?;
            Ok(deadline)
        });
        match readiness {
            Ok(deadline) => {
                match observed.observe().and_then(|now| {
                    running_remaining(deadline, now)
                        .and_then(|_| owner.begin_work(now).map_err(FixtureError::Lifecycle))
                }) {
                    Ok(permit) => control_loop = Some(permit),
                    Err(error) => failure = Some(error),
                }
                if failure.is_none() {
                    failure = run_control_loop(&mut observed, wait, notification, deadline).err();
                }
            }
            Err(error) => failure = Some(error),
        }
    }
    let stopping_at = observed.cleanup_observation(&mut failure);
    if owner.state() == Some(ServiceState::StartPending)
        && let Err(error) = owner.check_deadline(stopping_at)
        && failure.is_none()
    {
        failure = Some(FixtureError::Lifecycle(error));
    }
    if let Err(error) = owner.request_stop(stopping_at)
        && failure.is_none()
    {
        failure = Some(FixtureError::Lifecycle(error));
    }
    if let Some(permit) = control_loop {
        permit.complete();
    }
    let finished_at = observed.cleanup_observation(&mut failure);
    if let Err(error) = owner.check_deadline(finished_at)
        && failure.is_none()
    {
        failure = Some(FixtureError::Lifecycle(error));
    }
    let outcome = if failure.is_some() {
        StopOutcome::Failed(NonZeroU32::MIN)
    } else {
        StopOutcome::Clean
    };
    if let Err(error) = owner.finish_stop(finished_at, outcome)
        && failure.is_none()
    {
        failure = Some(FixtureError::Lifecycle(error));
    }
    failure.map_or(Ok(()), Err)
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

/// Wait for sticky Stop within the absolute Ready-anchored deadline, never renewing it.
///
/// # Errors
/// Reject reversed time and explicit laboratory run exhaustion.
fn run_control_loop<C: Clock, W: Wait>(
    clock: &mut ObservedClock<'_, C>,
    wait: &mut W,
    notification: &StopNotification,
    deadline: Instant,
) -> Result<(), FixtureError> {
    loop {
        let now = clock.observe()?;
        let remaining = running_remaining(deadline, now)?;
        if notification.stop_requested() {
            return Ok(());
        }
        wait.park(remaining);
    }
}

#[cfg(test)]
#[path = "host_tests.rs"]
mod tests;

// SPDX-License-Identifier: GPL-2.0-only
//! Independent public-consumer lifecycle traces with injected status failures.

use std::{
    cell::RefCell,
    error::Error,
    fmt,
    num::NonZeroU32,
    rc::Rc,
    time::{Duration, Instant},
};
use windows_platform::service::{
    BudgetError, LifecycleError, PendingBudget, ServiceLifecycle, ServiceRole, ServiceState,
    ServiceStatus, StatusReporter, StopOutcome,
};

/// Fixed fake sink failure, never an assertion of native Windows behavior.
#[derive(Debug, Eq, PartialEq)]
struct SinkFailure;

impl fmt::Display for SinkFailure {
    /// Keep fault text independent of supplied statuses.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("injected status failure")
    }
}
impl Error for SinkFailure {}

/// Recorded attempts include failed reports; acknowledgements belong to the owner.
#[derive(Default)]
struct Reports {
    /// Complete observed status sequence.
    attempts: Vec<ServiceStatus>,
    /// One-based attempt numbers to reject.
    reject: Vec<usize>,
}

/// Shared test log allows observation without exposing the owned reporter.
struct Reporter(Rc<RefCell<Reports>>);

impl StatusReporter for Reporter {
    type Error = SinkFailure;
    /// Record actual calls and inject only named attempts.
    fn report(&mut self, status: ServiceStatus) -> Result<(), SinkFailure> {
        let mut log = self.0.borrow_mut();
        log.attempts.push(status);
        if log.reject.contains(&log.attempts.len()) {
            Err(SinkFailure)
        } else {
            Ok(())
        }
    }
}

/// Construct explicit laboratory limits and a new independent role instance.
///
/// # Errors
/// Propagate rejection if the explicit laboratory budget is invalid.
fn owner(
    role: ServiceRole,
    log: &Rc<RefCell<Reports>>,
) -> Result<ServiceLifecycle<Reporter>, BudgetError> {
    Ok(ServiceLifecycle::new(
        role,
        Reporter(Rc::clone(log)),
        PendingBudget::new(10, 100)?,
        PendingBudget::new(20, 200)?,
        NonZeroU32::MIN,
    ))
}

/// Check absolute deadlines without producing status checkpoints.
///
/// # Panics
/// Fails if observed status, admission or cleanup differs from the expected contract.
///
/// # Errors
/// Propagate unexpected fixture construction or lifecycle failures.
#[test]
fn checking_a_stuck_phase_does_not_fabricate_progress() -> Result<(), Box<dyn Error>> {
    let log = Rc::new(RefCell::new(Reports::default()));
    let mut service = owner(ServiceRole::Engine, &log)?;
    let now = Instant::now();
    assert_eq!(
        service.check_deadline(now),
        Err(LifecycleError::InvalidTransition)
    );
    service.start(now)?;
    service.check_deadline(now + Duration::from_millis(99))?;
    assert_eq!(log.borrow().attempts.len(), 1);
    assert_eq!(
        service.check_deadline(now + Duration::from_millis(100)),
        Err(LifecycleError::DeadlineElapsed)
    );
    service.request_stop(now + Duration::from_millis(100))?;
    service.check_deadline(now + Duration::from_millis(299))?;
    assert_eq!(
        service.check_deadline(now + Duration::from_millis(300)),
        Err(LifecycleError::DeadlineElapsed)
    );
    service.finish_stop(
        now + Duration::from_millis(300),
        StopOutcome::Failed(NonZeroU32::MIN),
    )?;
    assert_eq!(
        service.check_deadline(now + Duration::from_millis(300)),
        Err(LifecycleError::InvalidTransition)
    );
    assert_eq!(log.borrow().attempts.len(), 3);
    Ok(())
}

/// Challenge zero, contradictory and maximum representable raw budgets.
///
/// # Panics
/// Fails if observed status, admission or cleanup differs from the expected contract.
#[test]
fn pending_limits_are_mandatory_without_product_defaults() {
    for (hint, timeout) in [(0, 1), (1, 0), (0, 0), (2, 1), (u32::MAX, 1)] {
        assert_eq!(PendingBudget::new(hint, timeout).err(), Some(BudgetError));
    }
    for (hint, timeout) in [(1, 1), (1, u32::MAX), (u32::MAX, u32::MAX)] {
        assert!(PendingBudget::new(hint, timeout).is_ok());
    }
}

/// Compare both process roles against the independent expected status sequence.
///
/// # Panics
/// Fails if observed status, admission or cleanup differs from the expected contract.
///
/// # Errors
/// Propagate unexpected fixture construction or lifecycle failures.
#[test]
fn both_roles_follow_the_same_truthful_status_trace() -> Result<(), Box<dyn Error>> {
    for role in [ServiceRole::Engine, ServiceRole::ClockBroker] {
        let log = Rc::new(RefCell::new(Reports::default()));
        let mut service = owner(role, &log)?;
        let now = Instant::now();
        assert_eq!(service.role(), role);
        assert_eq!(service.state(), None);
        assert_eq!(service.acknowledged(), None);
        assert_eq!(log.borrow().attempts.len(), 0);
        service.start(now)?;
        service.progress(now + Duration::from_millis(5))?;
        service.ready(now + Duration::from_millis(6))?;
        assert!(service.request_stop(now + Duration::from_millis(7))?);
        assert!(!service.request_stop(now + Duration::from_millis(8))?);
        service.finish_stop(now + Duration::from_millis(9), StopOutcome::Clean)?;
        let expected = [
            (ServiceState::StartPending, 1, 10, false, None),
            (ServiceState::StartPending, 2, 10, false, None),
            (ServiceState::Running, 0, 0, true, None),
            (ServiceState::StopPending, 1, 20, false, None),
            (ServiceState::Stopped, 0, 0, false, Some(StopOutcome::Clean)),
        ];
        let actual: Vec<_> = log
            .borrow()
            .attempts
            .iter()
            .map(|s| {
                (
                    s.state(),
                    s.checkpoint(),
                    s.wait_hint_ms(),
                    s.accepts_stop(),
                    s.outcome(),
                )
            })
            .collect();
        assert_eq!(actual, expected);
        assert_eq!(
            service.acknowledged(),
            log.borrow().attempts.last().copied()
        );
    }
    Ok(())
}

/// Inject startup acknowledgement failures beside a successful cleanup path.
///
/// # Panics
/// Fails if observed status, admission or cleanup differs from the expected contract.
///
/// # Errors
/// Propagate unexpected fixture construction or lifecycle failures.
#[test]
fn failed_start_or_running_report_fences_work_but_allows_cleanup() -> Result<(), Box<dyn Error>> {
    for failed_attempt in [1, 2] {
        let log = Rc::new(RefCell::new(Reports {
            attempts: vec![],
            reject: vec![failed_attempt],
        }));
        let mut service = owner(ServiceRole::Engine, &log)?;
        let now = Instant::now();
        if failed_attempt == 1 {
            assert_eq!(service.start(now), Err(LifecycleError::Report(SinkFailure)));
            assert_eq!(service.acknowledged(), None);
            assert_eq!(service.ready(now), Err(LifecycleError::ReportingFailed));
        } else {
            service.start(now)?;
            let before = service.acknowledged();
            assert_eq!(service.ready(now), Err(LifecycleError::Report(SinkFailure)));
            assert_eq!(service.acknowledged(), before);
        }
        assert!(matches!(
            service.begin_work(now),
            Err(LifecycleError::ReportingFailed)
        ));
        service.request_stop(now)?;
        service.finish_stop(now, StopOutcome::Failed(NonZeroU32::MIN))?;
        assert_eq!(service.state(), Some(ServiceState::Stopped));
    }
    Ok(())
}

/// Challenge failed stop acknowledgement and repeated internal stop delivery.
///
/// # Panics
/// Fails if observed status, admission or cleanup differs from the expected contract.
///
/// # Errors
/// Propagate unexpected fixture construction or lifecycle failures.
#[test]
fn failed_stop_report_never_reopens_admission_or_restarts_shutdown() -> Result<(), Box<dyn Error>> {
    let log = Rc::new(RefCell::new(Reports {
        attempts: vec![],
        reject: vec![3],
    }));
    let mut service = owner(ServiceRole::ClockBroker, &log)?;
    let now = Instant::now();
    service.start(now)?;
    service.ready(now)?;
    let prior = service.acknowledged();
    assert_eq!(
        service.request_stop(now),
        Err(LifecycleError::Report(SinkFailure))
    );
    assert_eq!(service.acknowledged(), prior);
    assert_eq!(service.state(), Some(ServiceState::StopPending));
    assert!(!service.request_stop(now + Duration::from_millis(100))?);
    assert!(matches!(
        service.begin_work(now),
        Err(LifecycleError::InvalidTransition)
    ));
    assert_eq!(service.ready(now), Err(LifecycleError::InvalidTransition));
    assert_eq!(
        service.progress(now + Duration::from_millis(200)),
        Err(LifecycleError::DeadlineElapsed)
    );
    service.finish_stop(
        now + Duration::from_millis(201),
        StopOutcome::Failed(NonZeroU32::MIN),
    )?;
    assert_eq!(log.borrow().attempts.len(), 4);
    Ok(())
}

/// Test just-before, exact and expired startup boundaries.
///
/// # Panics
/// Fails if observed status, admission or cleanup differs from the expected contract.
///
/// # Errors
/// Propagate unexpected fixture construction or lifecycle failures.
#[test]
fn progress_does_not_extend_startup_and_exact_deadline_rejects() -> Result<(), Box<dyn Error>> {
    for elapsed in [100, 101] {
        let log = Rc::new(RefCell::new(Reports::default()));
        let mut service = owner(ServiceRole::Engine, &log)?;
        let now = Instant::now();
        service.start(now)?;
        service.progress(now + Duration::from_millis(99))?;
        assert_eq!(
            service.ready(now + Duration::from_millis(elapsed)),
            Err(LifecycleError::DeadlineElapsed)
        );
        assert!(matches!(
            service.begin_work(now + Duration::from_millis(elapsed)),
            Err(LifecycleError::DeadlineElapsed)
        ));
        assert_eq!(log.borrow().attempts.len(), 2);
        service.request_stop(now + Duration::from_millis(elapsed))?;
        service.finish_stop(
            now + Duration::from_millis(elapsed),
            StopOutcome::Failed(NonZeroU32::MIN),
        )?;
    }
    Ok(())
}

/// Hold actual job custody across readiness, capacity and expired shutdown.
///
/// # Panics
/// Fails if observed status, admission or cleanup differs from the expected contract.
///
/// # Errors
/// Propagate unexpected fixture construction or lifecycle failures.
#[test]
fn completion_is_required_for_readiness_and_bounded_capacity() -> Result<(), Box<dyn Error>> {
    let log = Rc::new(RefCell::new(Reports::default()));
    let mut service = owner(ServiceRole::Engine, &log)?;
    let now = Instant::now();
    assert!(matches!(
        service.begin_work(now),
        Err(LifecycleError::InvalidTransition)
    ));
    service.start(now)?;
    let initialization = service.begin_work(now)?;
    assert_eq!(service.ready(now), Err(LifecycleError::OutstandingWork));
    assert!(matches!(
        service.begin_work(now),
        Err(LifecycleError::WorkLimit)
    ));
    initialization.complete();
    service.ready(now)?;
    let actual = service.begin_work(now)?;
    service.request_stop(now)?;
    assert_eq!(
        service.finish_stop(now, StopOutcome::Clean),
        Err(LifecycleError::OutstandingWork)
    );
    assert_eq!(
        service.progress(now + Duration::from_millis(200)),
        Err(LifecycleError::DeadlineElapsed)
    );
    assert_eq!(service.outstanding_work(), 1);
    assert_eq!(
        service.finish_stop(now + Duration::from_millis(201), StopOutcome::Clean),
        Err(LifecycleError::OutstandingWork)
    );
    actual.complete();
    assert_eq!(service.outstanding_work(), 0);
    service.finish_stop(
        now + Duration::from_millis(202),
        StopOutcome::Failed(NonZeroU32::MIN),
    )?;
    Ok(())
}

/// Drop a job permit and observe retained cleanup and capacity obligations.
///
/// # Panics
/// Fails if observed status, admission or cleanup differs from the expected contract.
///
/// # Errors
/// Propagate unexpected fixture construction or lifecycle failures.
#[test]
fn dropping_or_cancelling_a_permit_never_completes_the_operation() -> Result<(), Box<dyn Error>> {
    let log = Rc::new(RefCell::new(Reports::default()));
    let mut service = owner(ServiceRole::ClockBroker, &log)?;
    let now = Instant::now();
    service.start(now)?;
    service.ready(now)?;
    let job = service.begin_work(now)?;
    drop(job);
    service.request_stop(now)?;
    assert_eq!(service.outstanding_work(), 1);
    assert_eq!(
        service.finish_stop(now, StopOutcome::Failed(NonZeroU32::MIN)),
        Err(LifecycleError::OutstandingWork)
    );
    assert_eq!(log.borrow().attempts.len(), 3);
    Ok(())
}

/// Complete old work beside a successor with its own outstanding job.
///
/// # Panics
/// Fails if observed status, admission or cleanup differs from the expected contract.
///
/// # Errors
/// Propagate unexpected fixture construction or lifecycle failures.
#[test]
fn late_completion_is_bound_to_the_original_invocation() -> Result<(), Box<dyn Error>> {
    let old_log = Rc::new(RefCell::new(Reports::default()));
    let new_log = Rc::new(RefCell::new(Reports::default()));
    let mut old = owner(ServiceRole::Engine, &old_log)?;
    let mut successor = owner(ServiceRole::Engine, &new_log)?;
    let now = Instant::now();
    old.start(now)?;
    old.ready(now)?;
    let late = old.begin_work(now)?;
    old.request_stop(now)?;
    drop(old);
    successor.start(now)?;
    successor.ready(now)?;
    let current = successor.begin_work(now)?;
    late.complete();
    assert_eq!(successor.outstanding_work(), 1);
    successor.request_stop(now)?;
    assert_eq!(
        successor.finish_stop(now, StopOutcome::Clean),
        Err(LifecycleError::OutstandingWork)
    );
    current.complete();
    successor.finish_stop(now, StopOutcome::Clean)?;
    Ok(())
}

/// Compare successful and failed terminal reporting and reject every later transition.
///
/// # Panics
/// Fails if observed status, admission or cleanup differs from the expected contract.
///
/// # Errors
/// Propagate unexpected fixture construction or lifecycle failures.
#[test]
fn terminal_report_is_never_retried_even_when_it_fails() -> Result<(), Box<dyn Error>> {
    for reject in [vec![], vec![4]] {
        let log = Rc::new(RefCell::new(Reports {
            attempts: vec![],
            reject,
        }));
        let mut service = owner(ServiceRole::ClockBroker, &log)?;
        let now = Instant::now();
        service.start(now)?;
        service.ready(now)?;
        service.request_stop(now)?;
        let result = service.finish_stop(now, StopOutcome::Failed(NonZeroU32::MIN));
        if log.borrow().reject.is_empty() {
            result?;
        } else {
            assert_eq!(result, Err(LifecycleError::Report(SinkFailure)));
            assert_eq!(
                service.acknowledged().map(ServiceStatus::state),
                Some(ServiceState::StopPending)
            );
        }
        assert_eq!(service.state(), Some(ServiceState::Stopped));
        assert_eq!(service.start(now), Err(LifecycleError::InvalidTransition));
        assert_eq!(service.ready(now), Err(LifecycleError::InvalidTransition));
        assert_eq!(
            service.request_stop(now),
            Err(LifecycleError::InvalidTransition)
        );
        assert_eq!(
            service.progress(now),
            Err(LifecycleError::InvalidTransition)
        );
        assert_eq!(
            service.finish_stop(now, StopOutcome::Clean),
            Err(LifecycleError::InvalidTransition)
        );
        assert!(matches!(
            service.begin_work(now),
            Err(LifecycleError::InvalidTransition)
        ));
        assert_eq!(log.borrow().attempts.len(), 4);
    }
    Ok(())
}

/// Retain the regression where an earlier status error was hidden by a clean exit.
///
/// # Panics
/// Fails if observed status, admission or cleanup differs from the expected contract.
///
/// # Errors
/// Propagate unexpected fixture construction or lifecycle failures.
#[test]
fn earlier_report_failure_requires_an_explicit_failed_terminal_outcome()
-> Result<(), Box<dyn Error>> {
    let log = Rc::new(RefCell::new(Reports {
        attempts: vec![],
        reject: vec![2],
    }));
    let mut service = owner(ServiceRole::Engine, &log)?;
    let now = Instant::now();
    service.start(now)?;
    assert_eq!(service.ready(now), Err(LifecycleError::Report(SinkFailure)));
    service.request_stop(now)?;
    assert_eq!(
        service.finish_stop(now, StopOutcome::Clean),
        Err(LifecycleError::FailureOutcomeRequired)
    );
    assert_eq!(service.state(), Some(ServiceState::StopPending));
    assert_eq!(log.borrow().attempts.len(), 3);
    service.finish_stop(now, StopOutcome::Failed(NonZeroU32::MIN))?;
    assert_eq!(service.state(), Some(ServiceState::Stopped));
    Ok(())
}

/// Challenge backwards observations and unsupported phase transitions.
///
/// # Panics
/// Fails if observed status, admission or cleanup differs from the expected contract.
///
/// # Errors
/// Propagate unexpected fixture construction or lifecycle failures.
#[test]
fn backwards_time_and_invalid_transitions_never_report() -> Result<(), Box<dyn Error>> {
    let log = Rc::new(RefCell::new(Reports::default()));
    let mut service = owner(ServiceRole::Engine, &log)?;
    let earlier = Instant::now();
    let now = earlier + Duration::from_millis(1);
    assert_eq!(service.ready(now), Err(LifecycleError::InvalidTransition));
    assert_eq!(
        service.request_stop(now),
        Err(LifecycleError::InvalidTransition)
    );
    assert_eq!(
        service.progress(now),
        Err(LifecycleError::InvalidTransition)
    );
    assert_eq!(
        service.finish_stop(now, StopOutcome::Clean),
        Err(LifecycleError::InvalidTransition)
    );
    service.start(now)?;
    assert_eq!(service.start(now), Err(LifecycleError::InvalidTransition));
    assert_eq!(service.ready(earlier), Err(LifecycleError::TimeReversed));
    assert_eq!(service.progress(earlier), Err(LifecycleError::TimeReversed));
    assert_eq!(
        service.request_stop(earlier),
        Err(LifecycleError::TimeReversed)
    );
    assert!(matches!(
        service.begin_work(earlier),
        Err(LifecycleError::TimeReversed)
    ));
    service.ready(now)?;
    assert!(matches!(
        service.begin_work(earlier),
        Err(LifecycleError::TimeReversed)
    ));
    assert_eq!(
        service.progress(now),
        Err(LifecycleError::InvalidTransition)
    );
    service.request_stop(now)?;
    assert_eq!(
        service.finish_stop(earlier, StopOutcome::Clean),
        Err(LifecycleError::TimeReversed)
    );
    service.finish_stop(now, StopOutcome::Clean)?;
    Ok(())
}

/// Repeated stops remain idempotent while preserving monotonic input admission.
///
/// # Panics
/// Fails if duplicate stop delivery rewinds time, publishes or renews a deadline.
///
/// # Errors
/// Propagate unexpected fixture construction or lifecycle failures.
#[test]
fn repeated_stop_checks_time_without_repeating_the_status_effect() -> Result<(), Box<dyn Error>> {
    let log = Rc::new(RefCell::new(Reports::default()));
    let mut service = owner(ServiceRole::Engine, &log)?;
    let now = Instant::now();
    service.start(now)?;
    service.ready(now)?;
    service.request_stop(now + Duration::from_millis(10))?;
    let acknowledged = service.acknowledged();
    assert_eq!(
        service.request_stop(now + Duration::from_millis(9)),
        Err(LifecycleError::TimeReversed)
    );
    assert!(!service.request_stop(now + Duration::from_millis(20))?);
    assert_eq!(
        service.progress(now + Duration::from_millis(19)),
        Err(LifecycleError::TimeReversed)
    );
    assert_eq!(service.acknowledged(), acknowledged);
    assert_eq!(log.borrow().attempts.len(), 3);
    service.progress(now + Duration::from_millis(20))?;
    assert_eq!(
        service.check_deadline(now + Duration::from_millis(210)),
        Err(LifecycleError::DeadlineElapsed)
    );
    service.finish_stop(
        now + Duration::from_millis(210),
        StopOutcome::Failed(NonZeroU32::MIN),
    )?;
    assert_eq!(log.borrow().attempts.len(), 5);
    Ok(())
}

/// Challenge each neighboring raw budget in a finite independent input grid.
///
/// # Panics
/// Fails if zero or a contradictory hint enters the checked budget domain.
#[test]
fn budget_admission_agrees_with_the_complete_small_boundary_grid() {
    for hint in 0..=8 {
        for timeout in 0..=8 {
            let expected = hint > 0 && timeout > 0 && hint <= timeout;
            assert_eq!(PendingBudget::new(hint, timeout).is_ok(), expected);
        }
    }
}

/// Refill explicit multi-job caps after actual independent-thread completion.
///
/// # Panics
/// Fails if admission exceeds the cap, a completion is lost or a worker panics.
///
/// # Errors
/// Propagate unexpected fixture construction or lifecycle failures.
#[test]
fn parallel_completion_restores_only_the_explicit_job_capacity() -> Result<(), Box<dyn Error>> {
    for cap in [2, 3, 8] {
        let log = Rc::new(RefCell::new(Reports::default()));
        let budget = PendingBudget::new(10, 100)?;
        let Some(capacity) = NonZeroU32::new(cap) else {
            return Err(BudgetError.into());
        };
        let mut service = ServiceLifecycle::new(
            ServiceRole::Engine,
            Reporter(Rc::clone(&log)),
            budget,
            budget,
            capacity,
        );
        let now = Instant::now();
        service.start(now)?;
        service.ready(now)?;
        for _ in 0..4 {
            let mut permits = Vec::new();
            for expected in 1..=cap {
                permits.push(service.begin_work(now)?);
                assert_eq!(service.outstanding_work(), expected);
            }
            assert!(matches!(
                service.begin_work(now),
                Err(LifecycleError::WorkLimit)
            ));
            let Some(first) = permits.pop() else {
                return Err(BudgetError.into());
            };
            first.complete();
            assert_eq!(service.outstanding_work(), cap - 1);
            permits.push(service.begin_work(now)?);
            assert_eq!(service.outstanding_work(), cap);
            assert!(matches!(
                service.begin_work(now),
                Err(LifecycleError::WorkLimit)
            ));
            let barrier = std::sync::Arc::new(std::sync::Barrier::new(permits.len()));
            std::thread::scope(|scope| {
                for permit in permits {
                    let barrier = std::sync::Arc::clone(&barrier);
                    let _completion = scope.spawn(move || {
                        barrier.wait();
                        permit.complete();
                    });
                }
            });
            assert_eq!(service.outstanding_work(), 0);
        }
        assert_eq!(log.borrow().attempts.len(), 2);
        service.request_stop(now)?;
        service.finish_stop(now, StopOutcome::Clean)?;
        assert_eq!(log.borrow().attempts.len(), 4);
    }
    Ok(())
}

/// A later acknowledged progress report cannot clear an earlier startup failure.
///
/// # Panics
/// Fails if failed progress is acknowledged or healthy progress reopens readiness.
///
/// # Errors
/// Propagate unexpected fixture construction or lifecycle failures.
#[test]
fn successful_pending_report_does_not_clear_the_permanent_failure_fence()
-> Result<(), Box<dyn Error>> {
    let log = Rc::new(RefCell::new(Reports {
        attempts: vec![],
        reject: vec![2],
    }));
    let mut service = owner(ServiceRole::Engine, &log)?;
    let now = Instant::now();
    service.start(now)?;
    let initial = service.acknowledged();
    let actual = service.begin_work(now)?;
    assert_eq!(
        service.progress(now + Duration::from_millis(1)),
        Err(LifecycleError::Report(SinkFailure))
    );
    assert_eq!(service.acknowledged(), initial);
    assert_eq!(service.outstanding_work(), 1);
    service.progress(now + Duration::from_millis(2))?;
    assert_eq!(
        service.acknowledged().map(ServiceStatus::checkpoint),
        Some(3)
    );
    assert_eq!(
        service.ready(now + Duration::from_millis(2)),
        Err(LifecycleError::ReportingFailed)
    );
    assert!(matches!(
        service.begin_work(now + Duration::from_millis(2)),
        Err(LifecycleError::ReportingFailed)
    ));
    assert_eq!(
        service.check_deadline(now + Duration::from_millis(1)),
        Err(LifecycleError::TimeReversed)
    );
    assert_eq!(
        service.check_deadline(now + Duration::from_millis(100)),
        Err(LifecycleError::DeadlineElapsed)
    );
    assert_eq!(log.borrow().attempts.len(), 3);
    actual.complete();
    service.request_stop(now + Duration::from_millis(100))?;
    assert_eq!(
        service.finish_stop(now + Duration::from_millis(100), StopOutcome::Clean),
        Err(LifecycleError::FailureOutcomeRequired)
    );
    service.finish_stop(
        now + Duration::from_millis(100),
        StopOutcome::Failed(NonZeroU32::MIN),
    )?;
    Ok(())
}

/// Failed cleanup progress preserves original acknowledgement, custody and deadline.
///
/// # Panics
/// Fails if a failed progress report changes acknowledged status or releases work.
///
/// # Errors
/// Propagate unexpected fixture construction or lifecycle failures.
#[test]
fn failed_cleanup_progress_retains_custody_and_the_original_deadline() -> Result<(), Box<dyn Error>>
{
    let log = Rc::new(RefCell::new(Reports {
        attempts: vec![],
        reject: vec![4],
    }));
    let mut service = owner(ServiceRole::ClockBroker, &log)?;
    let now = Instant::now();
    service.start(now)?;
    service.ready(now)?;
    let actual = service.begin_work(now)?;
    service.request_stop(now + Duration::from_millis(10))?;
    let initial = service.acknowledged();
    assert_eq!(
        service.progress(now + Duration::from_millis(11)),
        Err(LifecycleError::Report(SinkFailure))
    );
    assert_eq!(service.acknowledged(), initial);
    assert_eq!(service.outstanding_work(), 1);
    assert_eq!(
        service.progress(now + Duration::from_millis(10)),
        Err(LifecycleError::TimeReversed)
    );
    service.progress(now + Duration::from_millis(12))?;
    assert_eq!(
        service.acknowledged().map(ServiceStatus::checkpoint),
        Some(3)
    );
    service.check_deadline(now + Duration::from_millis(209))?;
    assert_eq!(
        service.check_deadline(now + Duration::from_millis(210)),
        Err(LifecycleError::DeadlineElapsed)
    );
    assert_eq!(
        service.finish_stop(
            now + Duration::from_millis(210),
            StopOutcome::Failed(NonZeroU32::MIN),
        ),
        Err(LifecycleError::OutstandingWork)
    );
    assert_eq!(log.borrow().attempts.len(), 5);
    actual.complete();
    service.finish_stop(
        now + Duration::from_millis(210),
        StopOutcome::Failed(NonZeroU32::MIN),
    )?;
    assert_eq!(log.borrow().attempts.len(), 6);
    Ok(())
}

/// Retain the original typed reporter cause without formatting its untrusted text.
///
/// # Panics
/// Fails if formatting leaks the fixture marker or error chaining loses its cause.
#[test]
fn error_projection_is_bounded_while_the_original_cause_remains_available() {
    let marker = "private fixture marker\nincluding an untrusted line";
    let error = LifecycleError::Report(std::io::Error::other(marker));
    assert_eq!(error.to_string(), "service status was not acknowledged");
    let source = error
        .source()
        .and_then(|source| source.downcast_ref::<std::io::Error>());
    assert!(source.is_some_and(|source| {
        source.kind() == std::io::ErrorKind::Other && source.to_string() == marker
    }));
    let category: LifecycleError<std::io::Error> = LifecycleError::TimeReversed;
    assert!(category.source().is_none());
    assert!(BudgetError.source().is_none());
    assert_eq!(BudgetError.to_string(), "invalid service pending budget");
}

/// A failed startup deadline remains a service failure after successful cleanup.
///
/// # Panics
/// Fails if terminal Clean conceals a deadline error or valid cancellation fails.
///
/// # Errors
/// Propagate unexpected fixture construction or lifecycle failures.
#[test]
fn startup_deadline_failure_cannot_be_hidden_by_a_clean_terminal_outcome()
-> Result<(), Box<dyn Error>> {
    for elapsed in [99, 100, 101] {
        let log = Rc::new(RefCell::new(Reports::default()));
        let mut service = owner(ServiceRole::Engine, &log)?;
        let now = Instant::now();
        let observed = now + Duration::from_millis(elapsed);
        service.start(now)?;
        let deadline = service.check_deadline(observed);
        if elapsed < 100 {
            deadline?;
        } else {
            assert_eq!(deadline, Err(LifecycleError::DeadlineElapsed));
        }
        service.request_stop(observed)?;
        let terminal = service.finish_stop(observed, StopOutcome::Clean);
        if elapsed < 100 {
            terminal?;
        } else {
            assert_eq!(terminal, Err(LifecycleError::FailureOutcomeRequired));
            assert_eq!(service.state(), Some(ServiceState::StopPending));
            assert_eq!(log.borrow().attempts.len(), 2);
            service.finish_stop(observed, StopOutcome::Failed(NonZeroU32::MIN))?;
        }
        assert_eq!(service.state(), Some(ServiceState::Stopped));
        assert_eq!(log.borrow().attempts.len(), 3);
    }
    Ok(())
}

/// A diagnosed stop deadline overrun remains a failure after actual cleanup ends.
///
/// # Panics
/// Fails if a clean terminal outcome hides an overrun or timely cleanup is rejected.
///
/// # Errors
/// Propagate unexpected fixture construction or lifecycle failures.
#[test]
fn stop_deadline_overrun_requires_a_failed_terminal_outcome() -> Result<(), Box<dyn Error>> {
    for elapsed in [199, 200, 201] {
        let log = Rc::new(RefCell::new(Reports::default()));
        let mut service = owner(ServiceRole::ClockBroker, &log)?;
        let now = Instant::now();
        let observed = now + Duration::from_millis(elapsed);
        service.start(now)?;
        service.ready(now)?;
        let actual = service.begin_work(now)?;
        service.request_stop(now)?;
        let deadline = service.check_deadline(observed);
        if elapsed < 200 {
            deadline?;
        } else {
            assert_eq!(deadline, Err(LifecycleError::DeadlineElapsed));
        }
        assert_eq!(service.outstanding_work(), 1);
        actual.complete();
        let terminal = service.finish_stop(observed, StopOutcome::Clean);
        if elapsed < 200 {
            terminal?;
        } else {
            assert_eq!(terminal, Err(LifecycleError::FailureOutcomeRequired));
            assert_eq!(service.state(), Some(ServiceState::StopPending));
            assert_eq!(log.borrow().attempts.len(), 3);
            service.finish_stop(observed, StopOutcome::Failed(NonZeroU32::MIN))?;
        }
        assert_eq!(service.state(), Some(ServiceState::Stopped));
        assert_eq!(log.borrow().attempts.len(), 4);
    }
    Ok(())
}

/// Current guards detect deadline expiry even when the caller omits a timer poll.
///
/// # Panics
/// Fails if omitting the observer permits a clean timed-out service exit.
///
/// # Errors
/// Propagate unexpected fixture construction or lifecycle failures.
#[test]
fn terminal_deadline_guard_cannot_be_bypassed_by_omitting_the_observer()
-> Result<(), Box<dyn Error>> {
    for startup_expired in [false, true] {
        let log = Rc::new(RefCell::new(Reports::default()));
        let mut service = owner(ServiceRole::Engine, &log)?;
        let now = Instant::now();
        service.start(now)?;
        let stopped_at = if startup_expired {
            now + Duration::from_millis(100)
        } else {
            now
        };
        if !startup_expired {
            service.ready(now)?;
        }
        service.request_stop(stopped_at)?;
        let finished_at = if startup_expired {
            stopped_at
        } else {
            now + Duration::from_millis(200)
        };
        assert_eq!(
            service.finish_stop(finished_at, StopOutcome::Clean),
            Err(LifecycleError::FailureOutcomeRequired)
        );
        assert_eq!(service.state(), Some(ServiceState::StopPending));
        service.finish_stop(finished_at, StopOutcome::Failed(NonZeroU32::MIN))?;
        assert_eq!(service.state(), Some(ServiceState::Stopped));
    }
    Ok(())
}

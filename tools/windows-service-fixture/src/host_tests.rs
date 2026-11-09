// SPDX-License-Identifier: GPL-2.0-only
//! Independent facts exercised through the same private hosting core as `ServiceMain`.

use std::{
    cell::RefCell,
    error::Error,
    rc::Rc,
    sync::{
        OnceLock,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use windows_platform::{
    privilege_observation::{
        PrivilegeLuid, PrivilegeSnapshot, ServicePrivilege, ServicePrivilegeIds,
    },
    scm::{ScmControl, ScmStatus, StopNotification},
    service::{ServiceStatus, StatusReporter},
};

#[cfg(target_os = "linux")]
use super::guard_service_main;
use super::{
    Clock, Dispatcher, StatusFactory, TokenObserver, Wait, dispatch_once, dispatch_with_outcome,
    run_service,
};
use crate::{
    FixtureError, LabRole, NativeError, NativeOperation, TokenCleanupOutcome,
    TokenObservationCause, TokenObservationFailure,
};

/// A healthy native-shaped driver must return normally after its sole Stop wake.
/// The parent has no unresolved owner, so even a child's fatal custody failure
/// remains an ordinary failed behavior assertion with its original output.
///
/// # Errors
/// Propagate child launch/output errors and unexpected healthy-driver failure.
/// # Panics
/// Fails if the driver exceeds its finite wait script, changes status, or aborts.
#[cfg(target_os = "linux")]
#[test]
fn a_healthy_inert_driver_finishes_with_one_bounded_wait() -> Result<(), Box<dyn Error>> {
    use std::process::Command;

    /// Exact child admission for this bounded healthy-driver observation.
    const CHILD_MODE: &str = "NEVROTICK_FIXTURE_HEALTHY_DRIVER_CHILD";
    /// Reached only after the complete status/result assertions succeed.
    const COMPLETED: &str = "fixture-healthy-driver-completed";
    if std::env::var_os(CHILD_MODE).is_some() {
        let observation = Rc::new(RefCell::new(Observations::default()));
        let notification = StopNotification::new();
        execute(
            LabRole::Engine,
            1,
            &observation,
            None,
            &notification,
            vec![WaitAction::Stop(Duration::ZERO)],
        )?;
        assert_eq!(waits(&observation), [Duration::from_secs(60)]);
        assert_eq!(
            statuses(&observation),
            [
                [16, 2, 0, 0, 0, 1, 1000],
                [16, 4, 1, 0, 0, 0, 0],
                [16, 3, 0, 0, 0, 1, 1000],
                [16, 1, 0, 0, 0, 0, 0],
            ]
        );
        println!("{COMPLETED}");
        return Ok(());
    }
    let output = Command::new(std::env::current_exe()?)
        .args([
            "--exact",
            "host::tests::a_healthy_inert_driver_finishes_with_one_bounded_wait",
            "--nocapture",
        ])
        .env(CHILD_MODE, "1")
        .env("RUST_BACKTRACE", "0")
        .output()?;
    assert_eq!(
        output.status.code(),
        Some(0),
        "healthy driver failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(std::str::from_utf8(&output.stdout)?.contains(COMPLETED));
    Ok(())
}

/// The actual inert composition acknowledges readiness independently of cleanup.
/// These finite observations catch broken cancellation/completion without waiting
/// for the service driver or a mutation timeout.
///
/// # Panics
/// Fail if preparation or activation invents completion, or cancellation cannot
/// finish the inert worker's already-resolved work.
#[test]
fn a_inert_worker_keeps_readiness_and_completion_separate() {
    use std::task::Poll;

    use super::{InertWorker, Worker, WorkerWake};

    let wake = WorkerWake::current();
    let mut worker = InertWorker::new();
    assert_eq!(worker.poll_finished(&wake), Poll::Pending);
    assert_eq!(worker.poll_prepare(&wake), Poll::Ready(Ok(())));
    assert_eq!(worker.poll_finished(&wake), Poll::Pending);
    assert_eq!(worker.activate(), Ok(()));
    assert_eq!(worker.poll_running(&wake), Ok(()));
    assert_eq!(worker.poll_finished(&wake), Poll::Pending);
    worker.request_cancel(&wake);
    assert_eq!(worker.poll_finished(&wake), Poll::Ready(Ok(())));
    std::thread::park_timeout(Duration::ZERO);
}

/// The default overdue wait blocks on the existing owner thread until a wake.
/// A finite retry allowance tolerates spurious returns while rejecting a busy loop.
///
/// # Errors
/// Propagate bounded channel failures or a failed owner thread.
/// # Panics
/// Fails if the default wait exhausts its retry budget before publication and wake.
#[test]
fn default_overdue_wait_retains_the_owner_until_publication_and_wake() -> Result<(), Box<dyn Error>>
{
    use std::sync::{Arc, mpsc};

    /// This control deliberately inherits the real default overdue wait.
    struct DefaultWait;

    impl Wait for DefaultWait {
        /// This control exercises no timed-wait operation.
        fn park(&mut self, _timeout: Duration) {}
    }

    let published = Arc::new(AtomicBool::new(false));
    let owner_published = Arc::clone(&published);
    let (ready_sender, ready_receiver) = mpsc::sync_channel(1);
    let (done_sender, done_receiver) = mpsc::sync_channel(1);
    let owner = std::thread::spawn(move || {
        let mut wait = DefaultWait;
        std::thread::park_timeout(Duration::ZERO);
        if ready_sender.send(()).is_err() {
            return;
        }
        let mut returns = 0;
        while !owner_published.load(Ordering::Acquire) && returns < 1024 {
            wait.park_until_notified();
            returns += 1;
        }
        let _sent = done_sender.send(owner_published.load(Ordering::Acquire));
    });
    let ready = ready_receiver.recv_timeout(Duration::from_secs(2));
    let quiet = if ready.is_ok() {
        done_receiver.recv_timeout(Duration::from_millis(25))
    } else {
        Err(mpsc::RecvTimeoutError::Timeout)
    };
    let waited_for_publication = matches!(&quiet, Err(mpsc::RecvTimeoutError::Timeout));
    // Publish and wake even when the quiet control failed; assertions cannot leave
    // the healthy default implementation blocked in its actual park operation.
    published.store(true, Ordering::Release);
    owner.thread().unpark();
    let completed = match quiet {
        Ok(value) => Ok(value),
        Err(_) => done_receiver.recv_timeout(Duration::from_secs(2)),
    };
    ready?;
    let saw_publication = completed?;
    owner.join().map_err(|_| "default wait owner panicked")?;
    assert!(waited_for_publication);
    assert!(saw_publication);
    Ok(())
}

/// Recorded production entry/effect/wait calls, without synthetic transitions.
#[derive(Clone, Debug, Eq, PartialEq)]
enum Event {
    /// An actual call to the factory's registration seam.
    Register(LabRole),
    /// A real reporter invocation, whether it subsequently succeeds or fails.
    Status([u32; 7]),
    /// One finite waiting request made by the actual owner loop.
    Wait(Duration),
}

/// Bounded test observations and selected independent native failures.
#[derive(Default)]
struct Observations {
    /// Chronological calls made by the implementation.
    events: Vec<Event>,
    /// Number of actual status attempts, including failures.
    status_attempts: usize,
    /// One-based attempts to fail with their original typed causes.
    status_failures: Vec<(usize, NativeError)>,
}

/// Reporter returned only by the selected registration seam.
struct Reporter(Rc<RefCell<Observations>>);

impl StatusReporter for Reporter {
    type Error = NativeError;

    /// Observe every status and inject the exact independently selected API fault.
    fn report(&mut self, status: ServiceStatus) -> Result<(), NativeError> {
        let raw = ScmStatus::from_status(status);
        let fields = [
            raw.service_type(),
            raw.current_state(),
            raw.controls_accepted(),
            raw.win32_exit_code(),
            raw.service_specific_exit_code(),
            raw.checkpoint(),
            raw.wait_hint_ms(),
        ];
        let mut observed = self.0.borrow_mut();
        observed.events.push(Event::Status(fields));
        observed.status_attempts += 1;
        match observed
            .status_failures
            .iter()
            .find_map(|(attempt, error)| (*attempt == observed.status_attempts).then_some(*error))
        {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }
}

/// Registration can independently fail before any reporter exists.
struct Factory {
    /// Shared report and ordering observations.
    observations: Rc<RefCell<Observations>>,
    /// A registration failure, independent from all status failures.
    failure: Option<NativeError>,
}

impl StatusFactory for Factory {
    type Reporter = Reporter;

    /// Produce the actual reporter only after successful registration.
    fn register(&mut self, role: LabRole) -> Result<Reporter, NativeError> {
        self.observations
            .borrow_mut()
            .events
            .push(Event::Register(role));
        match self.failure {
            Some(error) => Err(error),
            None => Ok(Reporter(Rc::clone(&self.observations))),
        }
    }
}

/// Mutable test time with explicitly transient or persistent backwards observations.
struct Time {
    /// Current good monotonic observation.
    current: Instant,
    /// One transient bad observation; following reads retain the healthy clock.
    backwards_once: bool,
    /// Persistent faulty observations also remain backwards during cleanup.
    backwards_persistent: bool,
}

/// The real hosting core receives time exclusively through this clock seam.
struct TestClock(Rc<RefCell<Time>>);

impl Clock for TestClock {
    /// Return fixture time with the selected transient or persistent fault.
    ///
    /// # Panics
    /// Fails if the synthetic base cannot represent the selected backwards value.
    fn now(&mut self) -> Instant {
        let mut time = self.0.borrow_mut();
        if time.backwards_once || time.backwards_persistent {
            time.backwards_once = false;
            earlier_fixture_observation(time.current, Duration::from_millis(1))
        } else {
            time.current
        }
    }
}

/// Construct an exact backwards test observation without unchecked subtraction.
///
/// # Panics
/// Fails at fixture construction if the platform cannot represent the requested
/// earlier instant. The fallback is unreachable after this unconditional check.
fn earlier_fixture_observation(now: Instant, by: Duration) -> Instant {
    let earlier = now.checked_sub(by);
    assert!(earlier.is_some());
    earlier.unwrap_or(now)
}

/// Explicit finite wait outcomes independent of the implementation's chosen verdict.
#[derive(Clone, Copy)]
enum WaitAction {
    /// Wake early without any stop request.
    Spurious(Duration),
    /// Deliver a stop while the owner is waiting.
    Stop(Duration),
    /// Produce one backwards observation after an otherwise successful wait.
    BackwardsOnce,
    /// Keep every later observation below the last admitted time, including cleanup.
    BackwardsPersistent,
    /// Deliver benign and unsupported controls while the owner remains Running.
    Noise(Duration),
}

/// Waiting advances only test time and delivers selected real notifications.
struct TestWait<'a> {
    /// Shared clock updated at actual wait boundaries.
    time: Rc<RefCell<Time>>,
    /// Shared observations, including every duration requested by the owner.
    observations: Rc<RefCell<Observations>>,
    /// Fixed finite script; exhaustion advances by the requested full duration.
    actions: Vec<WaitAction>,
    /// Next script action, never inferred from an expected implementation verdict.
    next: usize,
    /// The same notification consumed by the actual hosting core.
    notification: &'a StopNotification,
}

impl Wait for TestWait<'_> {
    /// Record the requested wait and apply the selected independent event.
    ///
    /// # Panics
    /// Fails if a delivered Stop is not classified as Stop or the owner exceeds
    /// the finite event script plus its one final absolute-deadline wait.
    fn park(&mut self, timeout: Duration) {
        assert!(
            self.next <= self.actions.len(),
            "inert owner exceeded its finite event/deadline wait budget"
        );
        self.observations
            .borrow_mut()
            .events
            .push(Event::Wait(timeout));
        let action = self.actions.get(self.next).copied();
        self.next += 1;
        match action {
            Some(WaitAction::Spurious(elapsed)) => self.time.borrow_mut().current += elapsed,
            Some(WaitAction::Stop(elapsed)) => {
                self.time.borrow_mut().current += elapsed;
                assert_eq!(self.notification.handle_control(1), ScmControl::Stop);
            }
            Some(WaitAction::BackwardsOnce) => self.time.borrow_mut().backwards_once = true,
            Some(WaitAction::BackwardsPersistent) => {
                self.time.borrow_mut().backwards_persistent = true;
            }
            Some(WaitAction::Noise(elapsed)) => {
                self.time.borrow_mut().current += elapsed;
                assert_eq!(self.notification.handle_control(4), ScmControl::Interrogate);
                assert_eq!(
                    self.notification.handle_control(u32::MAX).return_code(),
                    120
                );
            }
            None => self.time.borrow_mut().current += timeout,
        }
    }

    /// Inert cancellation acknowledges completion in the same owner step, so
    /// this composition has no unresolved operation requiring an indefinite park.
    ///
    /// # Panics
    /// Fails when the actual driver requests overdue retention for the inert worker.
    #[expect(
        clippy::panic,
        reason = "a real indefinite wait is a behavioral failure in the inert-only test driver"
    )]
    fn park_until_notified(&mut self) {
        panic!("inert cancellation must finish before an overdue wait");
    }
}

/// Complete fixed test lookups, independent of the observed native token.
///
/// # Errors
/// Preserve an unexpected duplicate-ID construction failure.
fn fixture_privilege_ids()
-> Result<ServicePrivilegeIds, windows_platform::privilege_observation::PrivilegeCaptureError> {
    ServicePrivilegeIds::new([
        PrivilegeLuid::new(17, 0),
        PrivilegeLuid::new(23, 0),
        PrivilegeLuid::new(41, 0),
        PrivilegeLuid::new(59, 0),
    ])
}

/// Independently encode the exact healthy fixed-role set for existing host traces.
///
/// # Errors
/// Propagate checked construction; failed/partial API output is never simulated as success.
fn healthy_snapshot(role: LabRole) -> Result<PrivilegeSnapshot, TokenObservationFailure> {
    let entries: &[(u32, i32, u32)] = match role {
        LabRole::Engine => &[(59, 0, 2)],
        LabRole::ClockBroker => &[(59, 0, 2), (17, 0, 0)],
    };
    let mut raw = match role {
        LabRole::Engine => 1_u32,
        LabRole::ClockBroker => 2_u32,
    }
    .to_le_bytes()
    .to_vec();
    for (low, high, attributes) in entries {
        raw.extend(low.to_le_bytes());
        raw.extend(high.to_le_bytes());
        raw.extend(attributes.to_le_bytes());
    }
    fixture_privilege_ids()
        .and_then(|ids| PrivilegeSnapshot::decode(&raw, ids))
        .map_err(|error| {
            TokenObservationFailure::new(
                TokenObservationCause::Capture(error),
                TokenCleanupOutcome::NotOpened,
                TokenCleanupOutcome::NotOpened,
            )
        })
}

/// Explicit healthy observer provided to every historical actual-host trace.
struct HealthyTokenObserver(LabRole);
impl TokenObserver for HealthyTokenObserver {
    /// Supply a complete exact-role capture through the actual decoder.
    fn observe(&mut self) -> Result<PrivilegeSnapshot, TokenObservationFailure> {
        healthy_snapshot(self.0)
    }
}

/// Exercise the actual hosting core with explicit observations and finite inputs.
///
/// # Errors
/// Propagate the actual core's result, without adapting faults to an expected verdict.
fn execute(
    role: LabRole,
    argc: u32,
    observation: &Rc<RefCell<Observations>>,
    registration_failure: Option<NativeError>,
    notification: &StopNotification,
    actions: Vec<WaitAction>,
) -> Result<(), FixtureError> {
    let time = Rc::new(RefCell::new(Time {
        current: Instant::now() + Duration::from_secs(1),
        backwards_once: false,
        backwards_persistent: false,
    }));
    let mut factory = Factory {
        observations: Rc::clone(observation),
        failure: registration_failure,
    };
    let mut clock = TestClock(Rc::clone(&time));
    let mut wait = TestWait {
        time,
        observations: Rc::clone(observation),
        actions,
        next: 0,
        notification,
    };
    run_service(
        role,
        argc,
        &mut factory,
        &mut HealthyTokenObserver(role),
        &mut clock,
        &mut wait,
        notification,
    )
}

/// Inspect only effects actually delivered to the reporter.
fn statuses(observation: &Rc<RefCell<Observations>>) -> Vec<[u32; 7]> {
    observation
        .borrow()
        .events
        .iter()
        .filter_map(|event| match event {
            Event::Status(raw) => Some(*raw),
            Event::Register(_) | Event::Wait(_) => None,
        })
        .collect()
}

/// Read the actual requested waits, independently from clock advancement.
fn waits(observation: &Rc<RefCell<Observations>>) -> Vec<Duration> {
    observation
        .borrow()
        .events
        .iter()
        .filter_map(|event| match event {
            Event::Wait(duration) => Some(*duration),
            Event::Register(_) | Event::Status(_) => None,
        })
        .collect()
}

/// Both genuine callers use the fixed laboratory identities and exact status fields.
///
/// # Panics
/// Fails if role selection, fields, stop handling or registration ordering differ.
///
/// # Errors
/// Propagate unexpected hosting errors beside the independently delivered Stop.
#[test]
fn both_fixed_roles_start_and_stop_after_a_real_notification() -> Result<(), FixtureError> {
    for (role, name) in [
        (LabRole::Engine, "NevroTick-Lab-S02-Engine"),
        (LabRole::ClockBroker, "NevroTick-Lab-S02-Broker"),
    ] {
        for argc in [0, 1] {
            assert_eq!(role.name(), name);
            assert_eq!(
                role.service_role(),
                match role {
                    LabRole::Engine => windows_platform::service::ServiceRole::Engine,
                    LabRole::ClockBroker => windows_platform::service::ServiceRole::ClockBroker,
                }
            );
            let expected_wide: Vec<_> = name.encode_utf16().chain(std::iter::once(0)).collect();
            assert_eq!(role.wide_name(), expected_wide);
            let observation = Rc::new(RefCell::new(Observations::default()));
            let notification = StopNotification::new();
            execute(
                role,
                argc,
                &observation,
                None,
                &notification,
                vec![WaitAction::Stop(Duration::from_millis(1))],
            )?;
            assert_eq!(
                observation.borrow().events.first(),
                Some(&Event::Register(role))
            );
            assert_eq!(
                statuses(&observation),
                [
                    [16, 2, 0, 0, 0, 1, 1000],
                    [16, 4, 1, 0, 0, 0, 0],
                    [16, 3, 0, 0, 0, 1, 1000],
                    [16, 1, 0, 0, 0, 0, 0],
                ]
            );
            assert_eq!(waits(&observation), [Duration::from_millis(60_000)]);
        }
    }
    Ok(())
}

/// Registration failure cannot manufacture a reporter, status or running loop.
///
/// # Panics
/// Fails if a native cause changes or work begins after failed registration.
#[test]
fn registration_failure_retains_the_original_cause_without_any_status() {
    let observation = Rc::new(RefCell::new(Observations::default()));
    let notification = StopNotification::new();
    let failure = NativeError::new(NativeOperation::Register, 725);
    let result = execute(
        LabRole::Engine,
        0,
        &observation,
        Some(failure),
        &notification,
        vec![],
    );
    assert!(matches!(result, Err(FixtureError::Native(actual)) if actual == failure));
    assert_eq!(
        observation.borrow().events,
        [Event::Register(LabRole::Engine)]
    );
}

/// The native-shaped default composition emits constructor facts before any poll
/// can repeat them. Healthy startup is a neighboring control for each refusal.
///
/// # Errors
/// Propagate actual owner construction errors before comparing bounded snapshots.
/// # Panics
/// Fails if default routing is disconnected or constructor failures are deferred.
#[test]
fn diagnostics_default_entry_emits_startup_facts_before_first_poll() -> Result<(), Box<dyn Error>> {
    use super::{InertWorker, InvocationPoll, start_invocation};

    let report_error = NativeError::new(NativeOperation::Status, 7472);
    for (role, role_code) in [(LabRole::Engine, 1), (LabRole::ClockBroker, 2)] {
        for (argc, start_failure) in [(1, None), (2, None), (1, Some(report_error))] {
            let observation = Rc::new(RefCell::new(Observations {
                status_failures: start_failure.map_or_else(Vec::new, |error| vec![(1, error)]),
                ..Observations::default()
            }));
            let notification = StopNotification::new();
            let mut factory = Factory {
                observations: Rc::clone(&observation),
                failure: None,
            };
            let mut clock = ScriptedClock {
                observations: vec![Instant::now(); 16],
                reads: 0,
            };
            let mut invocation = start_invocation(
                role,
                argc,
                &mut factory,
                &mut HealthyTokenObserver(role),
                &mut clock,
                &notification,
                InertWorker::new(),
            )?;
            let expected_result = if argc == 2 {
                Err(FixtureError::UnsupportedArguments)
            } else if let Some(error) = start_failure {
                Err(FixtureError::Lifecycle(
                    windows_platform::service::LifecycleError::Report(error),
                ))
            } else {
                Ok(())
            };
            // Snapshot immediately, before Stop/poll can republish a missing fact.
            let initial_records = invocation.diagnostics.required().records().to_vec();
            let optional_was_off = invocation.diagnostics.optional().is_none();
            let initial_failure_matched = invocation.failure() == expected_result.as_ref().err();
            let initial_work = invocation.outstanding_work();
            let initial_admission_closed = invocation.admission_closed();
            let _control = notification.handle_control(1);
            let mut finished = false;
            for _ in 0..8 {
                if invocation.poll(&mut clock, &notification) == InvocationPoll::Finished {
                    finished = true;
                    break;
                }
            }
            let result = invocation.take_result();
            assert!(finished);
            assert_eq!(result, Some(expected_result));
            assert!(initial_failure_matched);
            assert_eq!(initial_work, 0);
            assert_eq!(
                initial_admission_closed,
                argc == 2 || start_failure.is_some()
            );
            assert!(optional_was_off);
            let mut expected_records = vec![[1, role_code, 1, 1, 0, 1, 0, 0, 0]];
            if argc == 2 || start_failure.is_some() {
                expected_records.push([
                    1,
                    role_code,
                    4,
                    2,
                    if argc == 2 { 4 } else { 10 },
                    2,
                    0,
                    0,
                    0,
                ]);
            }
            assert_eq!(initial_records, expected_records);
            assert_eq!(invocation.outstanding_work(), 0);
        }
    }
    Ok(())
}

/// Excessive SCM arguments fail through registered status without becoming policy.
///
/// # Panics
/// Fails if extra arguments enter Running, change identity or obtain clean success.
#[test]
fn extra_or_maximal_argc_cannot_reach_running() {
    for argc in [2, u32::MAX] {
        let observation = Rc::new(RefCell::new(Observations::default()));
        let notification = StopNotification::new();
        let result = execute(
            LabRole::ClockBroker,
            argc,
            &observation,
            None,
            &notification,
            vec![],
        );
        assert!(matches!(result, Err(FixtureError::UnsupportedArguments)));
        assert_eq!(
            statuses(&observation),
            [
                [16, 2, 0, 0, 0, 1, 1000],
                [16, 3, 0, 0, 0, 1, 1000],
                [16, 1, 0, 1066, 1, 0, 0],
            ]
        );
        assert_eq!(waits(&observation), []);
    }
}

/// A Stop already received during initialization prevents a Running admission.
///
/// # Panics
/// Fails if a coalesced preexisting Stop is cleared or ignored before readiness.
///
/// # Errors
/// Propagate unexpected hosting errors during a valid early stop.
#[test]
fn a_preexisting_stop_skips_running_and_any_wait() -> Result<(), FixtureError> {
    let observation = Rc::new(RefCell::new(Observations::default()));
    let notification = StopNotification::new();
    for _ in 0..8 {
        assert_eq!(notification.handle_control(1), ScmControl::Stop);
    }
    execute(
        LabRole::Engine,
        1,
        &observation,
        None,
        &notification,
        vec![],
    )?;
    assert_eq!(
        statuses(&observation),
        [
            [16, 2, 0, 0, 0, 1, 1000],
            [16, 3, 0, 0, 0, 1, 1000],
            [16, 1, 0, 0, 0, 0, 0],
        ]
    );
    assert_eq!(waits(&observation), []);
    assert!(notification.stop_requested());
    Ok(())
}

/// Real owner-loop wakes use the original deadline, never a new sixty-second lease.
///
/// # Panics
/// Fails if an early wake restarts the deadline or is mistaken for Stop.
///
/// # Errors
/// Propagate unexpected hosting errors beside the final genuine notification.
#[test]
fn spurious_wakes_preserve_the_original_absolute_run_limit() -> Result<(), FixtureError> {
    let observation = Rc::new(RefCell::new(Observations::default()));
    let notification = StopNotification::new();
    execute(
        LabRole::Engine,
        0,
        &observation,
        None,
        &notification,
        vec![
            WaitAction::Spurious(Duration::from_secs(1)),
            WaitAction::Spurious(Duration::from_secs(2)),
            WaitAction::Stop(Duration::ZERO),
        ],
    )?;
    assert_eq!(
        waits(&observation),
        [
            Duration::from_secs(60),
            Duration::from_secs(59),
            Duration::from_secs(57),
        ]
    );
    assert_eq!(statuses(&observation).len(), 4);
    Ok(())
}

/// Natural lab run-limit expiry remains an explicit failure after clean loop teardown.
///
/// # Panics
/// Fails if the owner waits forever, renews the limit or fabricates a clean terminal.
#[test]
fn the_exact_run_limit_stops_with_a_failed_outcome() {
    let observation = Rc::new(RefCell::new(Observations::default()));
    let notification = StopNotification::new();
    let result = execute(
        LabRole::ClockBroker,
        0,
        &observation,
        None,
        &notification,
        vec![],
    );
    assert!(matches!(result, Err(FixtureError::RunLimitElapsed)));
    assert_eq!(waits(&observation), [Duration::from_secs(60)]);
    assert_eq!(
        statuses(&observation),
        [
            [16, 2, 0, 0, 0, 1, 1000],
            [16, 4, 1, 0, 0, 0, 0],
            [16, 3, 0, 0, 0, 1, 1000],
            [16, 1, 0, 1066, 1, 0, 0],
        ]
    );
    assert!(!notification.stop_requested());
}

/// A transient backwards clock sample fails closed beside subsequent healthy cleanup.
///
/// # Panics
/// Fails if backwards time reaches clean success or consumes extra run allowance.
#[test]
fn reversed_loop_clock_is_a_failed_service_outcome() {
    let observation = Rc::new(RefCell::new(Observations::default()));
    let notification = StopNotification::new();
    let result = execute(
        LabRole::Engine,
        0,
        &observation,
        None,
        &notification,
        vec![WaitAction::BackwardsOnce],
    );
    assert!(matches!(result, Err(FixtureError::ClockReversed)));
    assert_eq!(waits(&observation), [Duration::from_secs(60)]);
    assert_eq!(
        statuses(&observation).last(),
        Some(&[16, 1, 0, 1066, 1, 0, 0])
    );
}

/// Every status call can fail, including the sole terminal-context use.
///
/// # Panics
/// Fails if a failure is swallowed, a terminal report repeats or cleanup is skipped.
#[test]
fn status_failures_never_restart_work_or_retry_the_terminal_context() {
    for (failed_attempt, code) in [(1, 801), (2, 802), (3, 803), (4, 804)] {
        let failure = NativeError::new(NativeOperation::Status, code);
        let observation = Rc::new(RefCell::new(Observations {
            events: vec![],
            status_attempts: 0,
            status_failures: vec![(failed_attempt, failure)],
        }));
        let notification = StopNotification::new();
        let result = execute(
            LabRole::Engine,
            0,
            &observation,
            None,
            &notification,
            vec![WaitAction::Stop(Duration::ZERO)],
        );
        assert!(matches!(
            result,
            Err(FixtureError::Lifecycle(windows_platform::service::LifecycleError::Report(error)))
                if error == failure
        ));
        let reported = statuses(&observation);
        assert_eq!(reported.iter().filter(|raw| raw[1] == 1).count(), 1);
        assert_eq!(reported.first(), Some(&[16, 2, 0, 0, 0, 1, 1000]));
        if failed_attempt <= 2 {
            assert_eq!(waits(&observation), []);
        }
        if failed_attempt == 1 {
            assert_eq!(reported.len(), 3);
        } else {
            assert_eq!(reported.len(), 4);
        }
        if failed_attempt < 4 {
            assert_eq!(reported.last(), Some(&[16, 1, 0, 1066, 1, 0, 0]));
        }
    }
}

/// The first actual fault remains the result when later cleanup reporting also fails.
///
/// # Panics
/// Fails if subsequent reporter faults erase the original native cause.
#[test]
fn the_first_status_failure_is_preserved_across_failed_cleanup_reports() {
    let first = NativeError::new(NativeOperation::Status, 313);
    let later = NativeError::new(NativeOperation::Status, 919);
    let observation = Rc::new(RefCell::new(Observations {
        events: vec![],
        status_attempts: 0,
        status_failures: vec![(1, first), (2, later), (3, later)],
    }));
    let notification = StopNotification::new();
    let result = execute(
        LabRole::Engine,
        0,
        &observation,
        None,
        &notification,
        vec![],
    );
    assert!(matches!(
        result,
        Err(FixtureError::Cleanup(ref failure))
            if failure.primary()
                == &FixtureError::Lifecycle(
                    windows_platform::service::LifecycleError::Report(first)
                )
                && failure.completion().is_none()
                && failure.stop_report()
                    == Some(&windows_platform::service::LifecycleError::Report(later))
                && failure.terminal_report()
                    == Some(&windows_platform::service::LifecycleError::Report(later))
    ));
    assert_eq!(statuses(&observation).len(), 3);
    assert_eq!(waits(&observation), []);
}

/// Driver observes only actual dispatch requests, never invokes a synthetic callback.
struct Driver {
    /// Fixed roles actually passed to this driver.
    roles: Vec<LabRole>,
    /// Exact native failure returned by the driver.
    failure: Option<NativeError>,
}

impl Dispatcher for Driver {
    /// Record one dispatcher operation and return the selected original native result.
    fn dispatch(&mut self, role: LabRole) -> Result<(), NativeError> {
        self.roles.push(role);
        match self.failure {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }
}

/// Native dispatcher failure is not permission to retry or select another service role.
///
/// # Panics
/// Fails if either successful or failed dispatch permits a second invocation.
#[test]
fn dispatcher_admission_is_once_even_after_the_first_native_call_fails() {
    let failure = NativeError::new(NativeOperation::Dispatch, 1063);
    for injected in [None, Some(failure)] {
        let once = AtomicBool::new(false);
        let mut driver = Driver {
            roles: vec![],
            failure: injected,
        };
        let first = dispatch_once(LabRole::Engine, &once, &mut driver);
        match injected {
            None => assert!(first.is_ok()),
            Some(expected) => {
                assert!(matches!(first, Err(FixtureError::Native(error)) if error == expected));
            }
        }
        assert!(once.load(Ordering::Acquire));
        assert!(matches!(
            dispatch_once(LabRole::ClockBroker, &once, &mut driver),
            Err(FixtureError::AlreadyDispatched)
        ));
        assert_eq!(driver.roles, [LabRole::Engine]);
    }
}

/// Benign and unsupported controls never produce extra state reports in the real loop.
///
/// # Panics
/// Fails if a control with no state effect triggers reporting or premature teardown.
///
/// # Errors
/// Propagate unexpected errors beside the final independently delivered Stop.
#[test]
fn unrelated_controls_do_not_report_or_end_the_owner_loop() -> Result<(), FixtureError> {
    let observation = Rc::new(RefCell::new(Observations::default()));
    let notification = StopNotification::new();
    execute(
        LabRole::Engine,
        0,
        &observation,
        None,
        &notification,
        vec![
            WaitAction::Noise(Duration::from_secs(1)),
            WaitAction::Stop(Duration::ZERO),
        ],
    )?;
    assert_eq!(
        waits(&observation),
        [Duration::from_secs(60), Duration::from_secs(59)]
    );
    assert_eq!(statuses(&observation).len(), 4);
    Ok(())
}

/// Native causes retain operation and raw code even when the OS reported zero.
///
/// # Panics
/// Fails if safe formatting obscures typed cause access or invents native success.
#[test]
fn typed_native_causes_survive_bounded_error_projection() {
    for operation in [
        NativeOperation::Dispatch,
        NativeOperation::Register,
        NativeOperation::Status,
    ] {
        for code in [0, u32::MAX] {
            let native = NativeError::new(operation, code);
            assert_eq!(native.operation(), operation);
            assert_eq!(native.code(), code);
            assert!(native.source().is_none());
            let wrapped = FixtureError::Native(native);
            assert_eq!(
                wrapped.to_string(),
                "laboratory native service operation failed"
            );
            assert_eq!(
                wrapped
                    .source()
                    .and_then(|source| source.downcast_ref::<NativeError>()),
                Some(&native)
            );
        }
    }
    let native = NativeError::new(NativeOperation::Status, 77);
    let wrapped =
        FixtureError::Lifecycle(windows_platform::service::LifecycleError::Report(native));
    assert_eq!(
        wrapped
            .source()
            .and_then(Error::source)
            .and_then(|source| source.downcast_ref::<NativeError>()),
        Some(&native)
    );
    assert!(FixtureError::ClockReversed.source().is_none());
}

/// Stop cannot retroactively authorize an owner loop observed at its fixed limit.
///
/// # Panics
/// Fails if a timely Stop is rejected or exact/late observation hides run exhaustion.
///
/// # Errors
/// Propagate unexpected hosting failure beside the valid just-before Stop.
#[test]
fn stop_at_or_after_the_absolute_limit_cannot_hide_run_exhaustion() -> Result<(), FixtureError> {
    for elapsed in [59_999, 60_000, 60_001] {
        let observation = Rc::new(RefCell::new(Observations::default()));
        let notification = StopNotification::new();
        let result = execute(
            LabRole::Engine,
            0,
            &observation,
            None,
            &notification,
            vec![WaitAction::Stop(Duration::from_millis(elapsed))],
        );
        let (win32_code, service_code) = if elapsed < 60_000 {
            result?;
            (0, 0)
        } else {
            assert!(matches!(result, Err(FixtureError::RunLimitElapsed)));
            (1066, 1)
        };
        assert!(notification.stop_requested());
        assert_eq!(waits(&observation), [Duration::from_secs(60)]);
        assert_eq!(
            statuses(&observation),
            [
                [16, 2, 0, 0, 0, 1, 1000],
                [16, 4, 1, 0, 0, 0, 0],
                [16, 3, 0, 0, 0, 1, 1000],
                [16, 1, 0, win32_code, service_code, 0, 0],
            ]
        );
    }
    Ok(())
}

/// A persistent bad clock cannot strand an acknowledged Running laboratory host.
///
/// # Panics
/// Fails if the original fault is lost, waits continue or cleanup never reports Stop.
#[test]
fn persistent_backwards_clock_still_retires_running_with_a_failed_terminal_outcome() {
    let observation = Rc::new(RefCell::new(Observations::default()));
    let notification = StopNotification::new();
    let result = execute(
        LabRole::Engine,
        0,
        &observation,
        None,
        &notification,
        vec![WaitAction::BackwardsPersistent],
    );
    assert!(matches!(result, Err(FixtureError::ClockReversed)));
    assert_eq!(waits(&observation), [Duration::from_secs(60)]);
    assert_eq!(
        statuses(&observation),
        [
            [16, 2, 0, 0, 0, 1, 1000],
            [16, 4, 1, 0, 0, 0, 0],
            [16, 3, 0, 0, 0, 1, 1000],
            [16, 1, 0, 1066, 1, 0, 0],
        ]
    );
    assert!(!notification.stop_requested());
}

/// Exact independent observations; reading past the script exposes a forbidden read.
struct ScriptedClock {
    /// Actual values supplied at each raw observation, including the first fault.
    observations: Vec<Instant>,
    /// Number of raw observations actually requested by the owner.
    reads: usize,
}

impl Clock for ScriptedClock {
    /// Supply only explicitly admitted observations, without repairing a fault.
    ///
    /// # Panics
    /// Fails if the owner reads again after the script's final faulty observation.
    fn now(&mut self) -> Instant {
        assert!(self.reads < self.observations.len());
        let observed = self.observations[self.reads];
        self.reads += 1;
        observed
    }
}

/// Drive the actual host with an exact raw-clock script and a genuine early Stop.
///
/// # Errors
/// Return the owner's actual verdict without extending or interpreting the script.
fn execute_scripted_clock(
    clock: &mut ScriptedClock,
    observation: &Rc<RefCell<Observations>>,
    notification: &StopNotification,
) -> Result<(), FixtureError> {
    let mut factory = Factory {
        observations: Rc::clone(observation),
        failure: None,
    };
    let mut wait = TestWait {
        time: Rc::new(RefCell::new(Time {
            current: Instant::now(),
            backwards_once: false,
            backwards_persistent: false,
        })),
        observations: Rc::clone(observation),
        actions: vec![WaitAction::Stop(Duration::ZERO)],
        next: 0,
        notification,
    };
    run_service(
        LabRole::Engine,
        0,
        &mut factory,
        &mut HealthyTokenObserver(LabRole::Engine),
        clock,
        &mut wait,
        notification,
    )
}

/// Reversal during preparation, readiness, work admission, activation/health/wake
/// observations or either cleanup read is final. No read renews the interval.
///
/// # Panics
/// Fails if any phase rereads a faulty clock, continues work or reports Clean.
///
/// # Errors
/// Propagate unexpected failure of the valid equal-time neighboring control.
#[test]
fn every_clock_boundary_fails_once_without_a_further_raw_read() -> Result<(), FixtureError> {
    let base = Instant::now() + Duration::from_secs(1);
    for (phase, fault_on_read, reported_running, waited) in [
        ("capture admission", 2, false, false),
        ("preparation admission", 3, false, false),
        ("readiness", 4, false, false),
        ("work admission", 5, true, false),
        ("initial loop observation after activation", 6, true, false),
        ("health observation before waiting", 7, true, false),
        ("wake observation", 8, true, true),
        ("stop bookkeeping", 9, true, true),
        ("terminal bookkeeping", 10, true, true),
    ] {
        let observation = Rc::new(RefCell::new(Observations::default()));
        let notification = StopNotification::new();
        let mut clock = ScriptedClock {
            observations: vec![base; fault_on_read - 1],
            reads: 0,
        };
        clock
            .observations
            .push(earlier_fixture_observation(base, Duration::from_nanos(1)));
        let result = execute_scripted_clock(&mut clock, &observation, &notification);
        assert!(
            matches!(result, Err(FixtureError::ClockReversed)),
            "{phase}"
        );
        assert_eq!(clock.reads, fault_on_read, "{phase}");
        let mut expected = vec![[16, 2, 0, 0, 0, 1, 1000]];
        if reported_running {
            expected.push([16, 4, 1, 0, 0, 0, 0]);
        }
        expected.extend([[16, 3, 0, 0, 0, 1, 1000], [16, 1, 0, 1066, 1, 0, 0]]);
        assert_eq!(statuses(&observation), expected, "{phase}");
        let expected_waits = if waited {
            vec![Duration::from_secs(60)]
        } else {
            vec![]
        };
        assert_eq!(waits(&observation), expected_waits, "{phase}");
    }

    let observation = Rc::new(RefCell::new(Observations::default()));
    let notification = StopNotification::new();
    let mut clock = ScriptedClock {
        observations: vec![base; 10],
        reads: 0,
    };
    execute_scripted_clock(&mut clock, &observation, &notification)?;
    assert_eq!(clock.reads, 10);
    assert_eq!(waits(&observation), [Duration::from_secs(60)]);
    assert_eq!(statuses(&observation).last(), Some(&[16, 1, 0, 0, 0, 0, 0]));
    Ok(())
}

/// An advanced trusted observation cannot roll back merely because it exceeds startup.
/// The neighboring trace advances once to Ready and then remains equal and valid.
///
/// # Panics
/// Fails if rollback reads again, enters the loop, reports Clean or loses its cause,
/// or if the healthy advancing/equal-time neighbor is refused.
///
/// # Errors
/// Propagate unrepresentable fixture instants and unexpected healthy-neighbor failure.
#[test]
fn advanced_clock_observation_rejects_rollback_without_returning_to_startup()
-> Result<(), FixtureError> {
    let startup = Instant::now();
    let ready = startup
        .checked_add(Duration::from_millis(100))
        .ok_or(FixtureError::ClockOverflow)?;
    for reversed in [false, true] {
        let observation = Rc::new(RefCell::new(Observations::default()));
        let notification = StopNotification::new();
        let mut clock = ScriptedClock {
            observations: if reversed {
                vec![
                    startup,
                    startup,
                    startup,
                    ready,
                    earlier_fixture_observation(ready, Duration::from_nanos(1)),
                ]
            } else {
                vec![
                    startup, startup, startup, ready, ready, ready, ready, ready, ready, ready,
                ]
            },
            reads: 0,
        };
        let result = execute_scripted_clock(&mut clock, &observation, &notification);
        let (win32_code, service_code) = if reversed {
            assert!(matches!(result, Err(FixtureError::ClockReversed)));
            assert_eq!(clock.reads, 5);
            assert_eq!(waits(&observation), []);
            (1066, 1)
        } else {
            result?;
            assert_eq!(clock.reads, 10);
            assert_eq!(waits(&observation), [Duration::from_secs(60)]);
            (0, 0)
        };
        assert_eq!(
            statuses(&observation),
            [
                [16, 2, 0, 0, 0, 1, 1000],
                [16, 4, 1, 0, 0, 0, 0],
                [16, 3, 0, 0, 0, 1, 1000],
                [16, 1, 0, win32_code, service_code, 0, 0],
            ]
        );
    }
    Ok(())
}

/// Historical negative cleanup preserves an earlier real status failure as cause.
///
/// # Panics
/// Fails if later backwards cleanup hides the earlier failure or retries observation.
#[test]
fn cleanup_clock_reversal_preserves_an_earlier_status_failure() {
    let failure = NativeError::new(NativeOperation::Status, 515);
    let observation = Rc::new(RefCell::new(Observations {
        events: vec![],
        status_attempts: 0,
        status_failures: vec![(1, failure)],
    }));
    let notification = StopNotification::new();
    let base = Instant::now() + Duration::from_secs(1);
    let mut clock = ScriptedClock {
        observations: vec![
            base,
            earlier_fixture_observation(base, Duration::from_nanos(1)),
        ],
        reads: 0,
    };
    let result = execute_scripted_clock(&mut clock, &observation, &notification);
    assert!(matches!(
        result,
        Err(FixtureError::Lifecycle(windows_platform::service::LifecycleError::Report(actual)))
            if actual == failure
    ));
    assert_eq!(clock.reads, 2);
    assert_eq!(waits(&observation), []);
    assert_eq!(
        statuses(&observation),
        [
            [16, 2, 0, 0, 0, 1, 1000],
            [16, 3, 0, 0, 0, 1, 1000],
            [16, 1, 0, 1066, 1, 0, 0],
        ]
    );
}

/// The sixty-second Running allowance starts at readiness, before permit/loop delay.
/// Distinct startup/readiness instants also reject an incorrect startup-time anchor.
///
/// # Panics
/// Fails if either delayed observation renews Running, rejects its just-before
/// neighbor, parks after expiry or fails to retire the actual permit before Stop.
/// Expired pre-work observations require seven reads, proving no activation read.
/// All six actual traces are collected before their independent comparison.
///
/// # Errors
/// Propagate an unrepresentable test observation before exercising the actual owner.
#[test]
fn running_limit_starts_at_ready_before_delayed_work_or_loop_observation()
-> Result<(), FixtureError> {
    let startup = Instant::now();
    let ready = startup
        .checked_add(Duration::from_millis(100))
        .ok_or(FixtureError::ClockOverflow)?;
    let mut actual = Vec::new();
    let mut expected = Vec::new();
    for delayed_read in [5, 6] {
        for elapsed_ms in [59_999, 60_000, 60_001] {
            let later = ready
                .checked_add(Duration::from_millis(elapsed_ms))
                .ok_or(FixtureError::ClockOverflow)?;
            let begin_work = if delayed_read == 5 { later } else { ready };
            let mut clock = ScriptedClock {
                observations: vec![
                    startup, startup, startup, ready, begin_work, later, later, later, later, later,
                ],
                reads: 0,
            };
            let observation = Rc::new(RefCell::new(Observations::default()));
            let notification = StopNotification::new();
            let result = execute_scripted_clock(&mut clock, &observation, &notification);
            actual.push((
                delayed_read,
                elapsed_ms,
                result,
                waits(&observation),
                statuses(&observation),
                clock.reads,
            ));

            let (result, waits, win32_code, service_code) = if elapsed_ms < 60_000 {
                (Ok(()), vec![Duration::from_millis(1)], 0, 0)
            } else {
                (Err(FixtureError::RunLimitElapsed), vec![], 1066, 1)
            };
            let expected_reads = if elapsed_ms < 60_000 {
                10
            } else if delayed_read == 5 {
                7
            } else {
                8
            };
            expected.push((
                delayed_read,
                elapsed_ms,
                result,
                waits,
                vec![
                    [16, 2, 0, 0, 0, 1, 1000],
                    [16, 4, 1, 0, 0, 0, 0],
                    [16, 3, 0, 0, 0, 1, 1000],
                    [16, 1, 0, win32_code, service_code, 0, 0],
                ],
                expected_reads,
            ));
        }
    }
    assert_eq!(actual, expected);
    Ok(())
}

/// Completed cleanup still needs one terminal status when its Stop deadline expires.
/// All cases use a distinct startup, Ready and Stop observation, with an optional
/// real `StopPending` report failure whose original cause must retain priority.
///
/// # Panics
/// Fails if exact/late cleanup remains pending, hides expiry, retries terminal
/// reporting, loses an earlier cause or rejects the just-before clean neighbor.
/// All six actual traces are recorded before their independent comparison.
///
/// # Errors
/// Propagate an unrepresentable fixture instant before exercising the actual owner.
#[test]
fn stop_deadline_expiry_reports_failed_terminal_once_and_preserves_the_first_cause()
-> Result<(), FixtureError> {
    let startup = Instant::now();
    let ready = startup
        .checked_add(Duration::from_millis(100))
        .ok_or(FixtureError::ClockOverflow)?;
    let stopping = ready
        .checked_add(Duration::from_millis(1))
        .ok_or(FixtureError::ClockOverflow)?;
    let status_failure = NativeError::new(NativeOperation::Status, 771);
    let mut actual = Vec::new();
    let mut expected = Vec::new();
    for earlier_failure in [false, true] {
        for elapsed_ms in [9_999, 10_000, 10_001] {
            let finished = stopping
                .checked_add(Duration::from_millis(elapsed_ms))
                .ok_or(FixtureError::ClockOverflow)?;
            let observation = Rc::new(RefCell::new(Observations {
                events: vec![],
                status_attempts: 0,
                status_failures: if earlier_failure {
                    vec![(3, status_failure)]
                } else {
                    vec![]
                },
            }));
            let notification = StopNotification::new();
            let mut clock = ScriptedClock {
                observations: vec![
                    startup, startup, startup, ready, ready, ready, ready, ready, stopping,
                    finished,
                ],
                reads: 0,
            };
            let result = execute_scripted_clock(&mut clock, &observation, &notification);
            actual.push((
                earlier_failure,
                elapsed_ms,
                result,
                waits(&observation),
                statuses(&observation),
                clock.reads,
            ));

            let result = if earlier_failure {
                Err(FixtureError::Lifecycle(
                    windows_platform::service::LifecycleError::Report(status_failure),
                ))
            } else if elapsed_ms >= 10_000 {
                Err(FixtureError::Lifecycle(
                    windows_platform::service::LifecycleError::DeadlineElapsed,
                ))
            } else {
                Ok(())
            };
            let (win32_code, service_code) = if earlier_failure || elapsed_ms >= 10_000 {
                (1066, 1)
            } else {
                (0, 0)
            };
            expected.push((
                earlier_failure,
                elapsed_ms,
                result,
                vec![Duration::from_secs(60)],
                vec![
                    [16, 2, 0, 0, 0, 1, 1000],
                    [16, 4, 1, 0, 0, 0, 0],
                    [16, 3, 0, 0, 0, 1, 1000],
                    [16, 1, 0, win32_code, service_code, 0, 0],
                ],
                10,
            ));
        }
    }
    assert_eq!(actual, expected);
    Ok(())
}

/// An already requested Stop cannot erase expiry of the preceding startup phase.
/// An earlier actual startup-report error retains priority beside the same limits.
///
/// # Panics
/// Fails if exact/late startup expiry strands `StopPending`, loses its first cause,
/// reaches Running/work/waiting or attempts anything beyond the sole terminal.
/// All six traces are collected before comparing independent expected facts.
///
/// # Errors
/// Propagate an unrepresentable fixture instant before exercising the actual owner.
#[test]
fn preexisting_stop_cannot_reset_an_expired_startup_deadline() -> Result<(), FixtureError> {
    let startup = Instant::now();
    let status_failure = NativeError::new(NativeOperation::Status, 773);
    let mut actual = Vec::new();
    let mut expected = Vec::new();
    for earlier_failure in [false, true] {
        for elapsed_ms in [9_999, 10_000, 10_001] {
            let stopping = startup
                .checked_add(Duration::from_millis(elapsed_ms))
                .ok_or(FixtureError::ClockOverflow)?;
            let observation = Rc::new(RefCell::new(Observations {
                events: vec![],
                status_attempts: 0,
                status_failures: if earlier_failure {
                    vec![(1, status_failure)]
                } else {
                    vec![]
                },
            }));
            let notification = StopNotification::new();
            assert_eq!(notification.handle_control(1), ScmControl::Stop);
            let mut clock = ScriptedClock {
                observations: vec![startup, stopping, stopping],
                reads: 0,
            };
            let result = execute_scripted_clock(&mut clock, &observation, &notification);
            actual.push((
                earlier_failure,
                elapsed_ms,
                result,
                waits(&observation),
                statuses(&observation),
                clock.reads,
            ));

            let result = if earlier_failure {
                Err(FixtureError::Lifecycle(
                    windows_platform::service::LifecycleError::Report(status_failure),
                ))
            } else if elapsed_ms >= 10_000 {
                Err(FixtureError::Lifecycle(
                    windows_platform::service::LifecycleError::DeadlineElapsed,
                ))
            } else {
                Ok(())
            };
            let (win32_code, service_code) = if earlier_failure || elapsed_ms >= 10_000 {
                (1066, 1)
            } else {
                (0, 0)
            };
            expected.push((
                earlier_failure,
                elapsed_ms,
                result,
                vec![],
                vec![
                    [16, 2, 0, 0, 0, 1, 1000],
                    [16, 3, 0, 0, 0, 1, 1000],
                    [16, 1, 0, win32_code, service_code, 0, 0],
                ],
                3,
            ));
        }
    }
    assert_eq!(actual, expected);
    Ok(())
}

/// Dispatcher failure must remain visible beside a previously successful owner.
///
/// # Panics
/// Fails if the actual production combiner replaces native failure with clean success.
#[test]
fn dispatcher_failure_cannot_be_hidden_by_a_successful_owner_receipt() {
    let once = AtomicBool::new(false);
    let outcome = OnceLock::new();
    let dispatch_failure = OnceLock::new();
    assert!(outcome.set(Ok(())).is_ok());
    let failure = NativeError::new(NativeOperation::Dispatch, 17);
    let mut driver = Driver {
        roles: vec![],
        failure: Some(failure),
    };
    let result = dispatch_with_outcome(
        LabRole::Engine,
        &once,
        &mut driver,
        &outcome,
        &dispatch_failure,
    );
    assert!(matches!(result, Err(FixtureError::Native(actual)) if *actual == failure));
    assert_eq!(outcome.get(), Some(&Ok(())));
    assert_eq!(dispatch_failure.get(), Some(&FixtureError::Native(failure)));
    assert_eq!(
        dispatch_with_outcome(
            LabRole::ClockBroker,
            &once,
            &mut driver,
            &outcome,
            &dispatch_failure,
        ),
        Err(&FixtureError::AlreadyDispatched)
    );
    assert_eq!(driver.roles, [LabRole::Engine]);
}

/// Failure custody neither manufactures a missing owner nor replaces a failed one.
///
/// # Panics
/// Fails if the original dispatcher code is lost or either owner receipt changes.
#[test]
fn dispatcher_failure_leaves_failed_or_missing_owner_receipts_unchanged() {
    for owner_failed in [false, true] {
        let once = AtomicBool::new(false);
        let outcome = OnceLock::new();
        let dispatch_failure = OnceLock::new();
        if owner_failed {
            assert!(outcome.set(Err(FixtureError::ClockReversed)).is_ok());
        }
        let failure = NativeError::new(
            NativeOperation::Dispatch,
            if owner_failed { u32::MAX } else { 0 },
        );
        let mut driver = Driver {
            roles: vec![],
            failure: Some(failure),
        };
        assert_eq!(
            dispatch_with_outcome(
                LabRole::Engine,
                &once,
                &mut driver,
                &outcome,
                &dispatch_failure,
            ),
            Err(&FixtureError::Native(failure))
        );
        let expected_owner = owner_failed.then_some(Err(FixtureError::ClockReversed));
        assert_eq!(outcome.get(), expected_owner.as_ref());
        assert_eq!(dispatch_failure.get(), Some(&FixtureError::Native(failure)));
        assert_eq!(driver.roles, [LabRole::Engine]);
    }
}

/// Successful dispatch preserves an acknowledged owner, its failure or absent receipt.
///
/// # Panics
/// Fails if a failed/missing owner becomes success or a healthy owner is rejected.
#[test]
fn successful_dispatch_projects_the_actual_owner_receipt() {
    for receipt in 0..3 {
        let once = AtomicBool::new(false);
        let outcome = OnceLock::new();
        let dispatch_failure = OnceLock::new();
        if receipt == 0 {
            assert!(outcome.set(Ok(())).is_ok());
        } else if receipt == 1 {
            assert!(outcome.set(Err(FixtureError::ClockReversed)).is_ok());
        }
        let mut driver = Driver {
            roles: vec![],
            failure: None,
        };
        let result = dispatch_with_outcome(
            LabRole::Engine,
            &once,
            &mut driver,
            &outcome,
            &dispatch_failure,
        );
        match receipt {
            0 => assert_eq!(result, Ok(())),
            1 => assert_eq!(result, Err(&FixtureError::ClockReversed)),
            _ => assert_eq!(result, Err(&FixtureError::OutcomeUnavailable)),
        }
        assert_eq!(driver.roles, [LabRole::Engine]);
        assert!(dispatch_failure.get().is_none());
    }
}

/// A successful owner receipt grants no permission to dispatch the process twice.
///
/// # Panics
/// Fails if the second entry reaches the driver or projects the earlier success.
#[test]
fn duplicate_entry_is_rejected_even_with_a_successful_owner_receipt() {
    let once = AtomicBool::new(false);
    let outcome = OnceLock::new();
    let dispatch_failure = OnceLock::new();
    assert!(outcome.set(Ok(())).is_ok());
    let mut driver = Driver {
        roles: vec![],
        failure: None,
    };
    assert_eq!(
        dispatch_with_outcome(
            LabRole::Engine,
            &once,
            &mut driver,
            &outcome,
            &dispatch_failure,
        ),
        Ok(())
    );
    assert_eq!(
        dispatch_with_outcome(
            LabRole::ClockBroker,
            &once,
            &mut driver,
            &outcome,
            &dispatch_failure,
        ),
        Err(&FixtureError::AlreadyDispatched)
    );
    assert_eq!(driver.roles, [LabRole::Engine]);
    assert!(dispatch_failure.get().is_none());
}

/// Test-only child selection, absent from every executable composition.
#[cfg(target_os = "linux")]
const PANIC_GUARD_CHILD_MODE: &str = "NEVROTICK_S02_PANIC_GUARD_CHILD_MODE";
/// Observable evidence that the healthy closure executed inside the actual guard.
#[cfg(target_os = "linux")]
const PANIC_GUARD_HEALTHY_BODY: &str = "fixture-guard-healthy-body";
/// Observable evidence immediately before the synthetic unwinding panic.
#[cfg(target_os = "linux")]
const PANIC_GUARD_PANIC_ENTERED: &str = "fixture-guard-panic-entered";
/// Reached only when the actual guard returns control to its caller.
#[cfg(target_os = "linux")]
const PANIC_GUARD_RETURNED: &str = "fixture-guard-returned";
/// Visible only if the captured synthetic payload's destructor actually executes.
#[cfg(target_os = "linux")]
const PANIC_GUARD_PAYLOAD_DROPPED: &str = "fixture-guard-payload-dropped";

/// Bounded synthetic panic payload; its destructor is an independent observer.
#[cfg(target_os = "linux")]
struct PanicPayload;

#[cfg(target_os = "linux")]
impl Drop for PanicPayload {
    /// Make actual payload destruction visible without affecting its expected fate.
    fn drop(&mut self) {
        eprintln!("{PANIC_GUARD_PAYLOAD_DROPPED}");
    }
}

/// Exercise the exact production guard in isolated healthy and aborting children.
/// The selected external runner supplies the finite deadline and disables core
/// dumps; this test launches no Windows binary or native SCM operation.
///
/// # Panics
/// Fails if the healthy body fails, panic escapes instead of aborting, the caller
/// resumes after panic, the captured payload is dropped or the wrong test runs.
///
/// # Errors
/// Propagate child launch/output failures and reject unknown test-only child modes.
#[cfg(target_os = "linux")]
#[test]
fn service_main_panic_guard_aborts_without_resuming_or_dropping_payload()
-> Result<(), Box<dyn Error>> {
    use std::{os::unix::process::ExitStatusExt, process::Command};

    match std::env::var(PANIC_GUARD_CHILD_MODE) {
        Ok(mode) => {
            match mode.as_str() {
                "healthy" => guard_service_main(|| println!("{PANIC_GUARD_HEALTHY_BODY}")),
                "panic" => guard_service_main(|| {
                    println!("{PANIC_GUARD_PANIC_ENTERED}");
                    #[expect(
                        clippy::panic,
                        reason = "intentional synthetic panic in the isolated guard test child"
                    )]
                    std::panic::panic_any(PanicPayload);
                }),
                _ => return Err("unrecognized fixture panic-guard child mode".into()),
            }
            println!("{PANIC_GUARD_RETURNED}");
            return Ok(());
        }
        Err(std::env::VarError::NotPresent) => {}
        Err(error) => return Err(Box::new(error)),
    }

    for mode in ["healthy", "panic"] {
        let output = Command::new(std::env::current_exe()?)
            .args([
                "--exact",
                "host::tests::service_main_panic_guard_aborts_without_resuming_or_dropping_payload",
                "--nocapture",
                "--test-threads=1",
            ])
            .env(PANIC_GUARD_CHILD_MODE, mode)
            .env("RUST_BACKTRACE", "0")
            .output()?;
        let stdout = std::str::from_utf8(&output.stdout)?;
        let stderr = std::str::from_utf8(&output.stderr)?;
        assert!(!stdout.contains(PANIC_GUARD_PAYLOAD_DROPPED));
        assert!(!stderr.contains(PANIC_GUARD_PAYLOAD_DROPPED));
        if mode == "healthy" {
            assert!(output.status.success());
            assert!(stdout.contains(PANIC_GUARD_HEALTHY_BODY));
            assert!(stdout.contains(PANIC_GUARD_RETURNED));
            assert!(!stdout.contains(PANIC_GUARD_PANIC_ENTERED));
        } else {
            assert_eq!(output.status.signal(), Some(6));
            assert!(stdout.contains(PANIC_GUARD_PANIC_ENTERED));
            assert!(!stdout.contains(PANIC_GUARD_RETURNED));
            assert!(!stdout.contains(PANIC_GUARD_HEALTHY_BODY));
        }
    }
    Ok(())
}

/// Independent event delivered synchronously by the actual token observation seam.
#[derive(Clone, Copy)]
enum CaptureAction {
    /// No concurrent state change during the synchronous observation.
    None,
    /// Deliver genuine sticky Stop while observation owns the lifecycle permit.
    Stop,
    /// Advance the owner clock by an explicit test interval.
    Advance(Duration),
    /// Supply a persistent backwards observation after capture returns.
    Reverse,
}

/// Fixed capture result and separate native-completion-side effects for host tests.
struct ControlledTokenObserver<'a> {
    /// Complete known snapshot or explicit indeterminate failure.
    result: Option<Result<PrivilegeSnapshot, TokenObservationFailure>>,
    /// Actual calls made by the real host, including attempted repeated captures.
    calls: usize,
    /// Same monotonic source consumed by the real host.
    time: Rc<RefCell<Time>>,
    /// Same genuine notification consumed by the real host.
    notification: &'a StopNotification,
    /// Independent effect before synchronous capture/cleanup completion returns.
    action: CaptureAction,
}
impl TokenObserver for ControlledTokenObserver<'_> {
    /// Perform the explicit script once per actual host call, without changing its verdict.
    ///
    /// # Panics
    /// Fails if the genuine Stop notification is not accepted as Stop.
    fn observe(&mut self) -> Result<PrivilegeSnapshot, TokenObservationFailure> {
        self.calls += 1;
        match self.action {
            CaptureAction::None => {}
            CaptureAction::Stop => {
                assert_eq!(self.notification.handle_control(1), ScmControl::Stop);
            }
            CaptureAction::Advance(by) => self.time.borrow_mut().current += by,
            CaptureAction::Reverse => self.time.borrow_mut().backwards_persistent = true,
        }
        self.result
            .take()
            .unwrap_or(Err(TokenObservationFailure::new(
                TokenObservationCause::InvalidHandle,
                TokenCleanupOutcome::NotOpened,
                TokenCleanupOutcome::NotOpened,
            )))
    }
}

/// Invoke the actual shared host with a controlled raw-capture result and action.
/// No wrapper implements role admission, deadlines, Stop handling or status output.
fn execute_observation(
    role: LabRole,
    argc: u32,
    notification: &StopNotification,
    result: Result<PrivilegeSnapshot, TokenObservationFailure>,
    action: CaptureAction,
    observation: &Rc<RefCell<Observations>>,
) -> (Result<(), FixtureError>, usize) {
    let time = Rc::new(RefCell::new(Time {
        current: Instant::now() + Duration::from_secs(1),
        backwards_once: false,
        backwards_persistent: false,
    }));
    let mut observer = ControlledTokenObserver {
        result: Some(result),
        calls: 0,
        time: Rc::clone(&time),
        notification,
        action,
    };
    let mut factory = Factory {
        observations: Rc::clone(observation),
        failure: None,
    };
    let mut clock = TestClock(Rc::clone(&time));
    let mut wait = TestWait {
        time,
        observations: Rc::clone(observation),
        actions: vec![WaitAction::Stop(Duration::ZERO)],
        next: 0,
        notification,
    };
    let result = run_service(
        role,
        argc,
        &mut factory,
        &mut observer,
        &mut clock,
        &mut wait,
        notification,
    );
    (result, observer.calls)
}

/// Build one exact counted target record without deriving expectations from the decoder.
///
/// # Errors
/// Propagate checked decoder/identity failure.
fn present_snapshot(
    role: LabRole,
    low: u32,
    attributes: u32,
) -> Result<PrivilegeSnapshot, windows_platform::privilege_observation::PrivilegeCaptureError> {
    let include_clock = role == LabRole::ClockBroker && low != 17;
    let mut raw = if include_clock { 3_u32 } else { 2_u32 }
        .to_le_bytes()
        .to_vec();
    for (entry_low, entry_attributes) in [(59_u32, 2_u32), (low, attributes)] {
        raw.extend(entry_low.to_le_bytes());
        raw.extend(0_i32.to_le_bytes());
        raw.extend(entry_attributes.to_le_bytes());
    }
    if include_clock {
        raw.extend(17_u32.to_le_bytes());
        raw.extend(0_i32.to_le_bytes());
        raw.extend(0_u32.to_le_bytes());
    }
    PrivilegeSnapshot::decode(&raw, fixture_privilege_ids()?)
}

/// Both actual callers enforce their exact role and retain known forbidden facts.
///
/// # Errors
/// Propagate fixture construction failure and unexpected healthy-host failure.
/// # Panics
/// Fails if a forbidden privilege reaches Running, a valid broker neighbor fails,
/// or either role skips/repeats capture.
#[test]
fn real_host_both_roles_enforce_exact_profiles_and_preserve_known_facts()
-> Result<(), Box<dyn Error>> {
    use windows_platform::privilege_observation::{PrivilegeState, ServicePrivilegeError};
    for (low, privilege) in [
        (17, ServicePrivilege::SystemTime),
        (23, ServicePrivilege::Debug),
        (41, ServicePrivilege::Impersonate),
    ] {
        for attributes in [
            0,
            1,
            2,
            3,
            0x8000_0000,
            0x8000_0001,
            0x8000_0002,
            0x8000_0003,
        ] {
            for role in [LabRole::Engine, LabRole::ClockBroker] {
                let observation = Rc::new(RefCell::new(Observations::default()));
                let notification = StopNotification::new();
                let (result, calls) = execute_observation(
                    role,
                    0,
                    &notification,
                    Ok(present_snapshot(role, low, attributes)?),
                    CaptureAction::None,
                    &observation,
                );
                assert_eq!(calls, 1);
                if role == LabRole::ClockBroker && low == 17 && attributes & 2 == 0 {
                    result?;
                    assert!(statuses(&observation).iter().any(|status| status[1] == 4));
                    assert_eq!(waits(&observation), [Duration::from_secs(60)]);
                } else {
                    let Err(FixtureError::PrivilegeAdmission(error)) = result else {
                        return Err("service admitted a forbidden privilege profile".into());
                    };
                    if role == LabRole::ClockBroker && low == 17 {
                        assert_eq!(error, ServicePrivilegeError::BrokerClockEnabled);
                    } else {
                        let ServicePrivilegeError::Forbidden(facts) = error else {
                            return Err("known forbidden privilege lost its typed facts".into());
                        };
                        assert_eq!(facts.privilege(), privilege);
                        assert_eq!(
                            facts.state(),
                            if attributes & 2 == 0 {
                                PrivilegeState::PresentDisabled
                            } else {
                                PrivilegeState::PresentEnabled
                            }
                        );
                    }
                    assert!(!statuses(&observation).iter().any(|status| status[1] == 4));
                    assert_eq!(waits(&observation), []);
                    assert_eq!(
                        statuses(&observation).last(),
                        Some(&[16, 1, 0, 1066, 1, 0, 0])
                    );
                }
            }
        }
    }
    for role in [LabRole::Engine, LabRole::ClockBroker] {
        let observation = Rc::new(RefCell::new(Observations::default()));
        let notification = StopNotification::new();
        let (result, calls) = execute_observation(
            role,
            0,
            &notification,
            healthy_snapshot(role),
            CaptureAction::None,
            &observation,
        );
        result?;
        assert_eq!(calls, 1);
        assert_eq!(waits(&observation), [Duration::from_secs(60)]);
    }
    Ok(())
}

/// Build the finite independent API/context/capture/cleanup refusal inventory.
///
/// # Errors
/// Propagate identity construction failure or unexpected acceptance of a bad input.
fn unknown_token_observation_failures() -> Result<Vec<TokenObservationFailure>, Box<dyn Error>> {
    let primary_close =
        TokenCleanupOutcome::Failed(NativeError::new(NativeOperation::TokenClose, 6));
    let thread_close =
        TokenCleanupOutcome::Failed(NativeError::new(NativeOperation::TokenClose, 7));
    let mut failures = vec![
        TokenObservationFailure::new(
            TokenObservationCause::Native(NativeError::new(NativeOperation::PrivilegeQuery, 0)),
            primary_close,
            thread_close,
        ),
        TokenObservationFailure::new(
            TokenObservationCause::UnexpectedThreadToken,
            TokenCleanupOutcome::NotOpened,
            thread_close,
        ),
        TokenObservationFailure::new(
            TokenObservationCause::InvalidHandle,
            TokenCleanupOutcome::NotOpened,
            TokenCleanupOutcome::NotOpened,
        ),
    ];
    for (primary_cleanup, thread_cleanup) in [
        (primary_close, TokenCleanupOutcome::NotOpened),
        (TokenCleanupOutcome::Closed, thread_close),
    ] {
        failures.push(TokenObservationFailure::new(
            TokenObservationCause::Native(NativeError::new(NativeOperation::TokenClose, 0)),
            primary_cleanup,
            thread_cleanup,
        ));
    }
    let collision = ServicePrivilegeIds::new([
        PrivilegeLuid::new(17, 0),
        PrivilegeLuid::new(23, 0),
        PrivilegeLuid::new(41, 0),
        PrivilegeLuid::new(17, 0),
    ])
    .err()
    .ok_or("colliding lookup identities admitted")?;
    failures.push(TokenObservationFailure::new(
        TokenObservationCause::Capture(collision),
        TokenCleanupOutcome::Closed,
        TokenCleanupOutcome::NotOpened,
    ));
    for operation in [
        NativeOperation::ThreadToken,
        NativeOperation::ProcessToken,
        NativeOperation::PrivilegeLookup,
        NativeOperation::PrivilegeQuery,
        NativeOperation::TokenClose,
    ] {
        for code in [0, 5, 1347] {
            failures.push(TokenObservationFailure::new(
                TokenObservationCause::Native(NativeError::new(operation, code)),
                TokenCleanupOutcome::Closed,
                TokenCleanupOutcome::NotOpened,
            ));
        }
    }
    for raw in [
        &[0; 3][..],
        &[1, 0, 0, 0][..],
        &[0, 0, 0, 0, 0][..],
        &[129, 0, 0, 0][..],
        &[1, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0][..],
        &[1, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0][..],
        &[
            2, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0,
        ][..],
    ] {
        let error = PrivilegeSnapshot::decode(raw, fixture_privilege_ids()?)
            .err()
            .ok_or("malformed fixture capture admitted")?;
        failures.push(TokenObservationFailure::new(
            TokenObservationCause::Capture(error),
            TokenCleanupOutcome::Closed,
            TokenCleanupOutcome::NotOpened,
        ));
    }
    Ok(failures)
}

/// Unknown API/context/capture/cleanup facts refuse both real callers without retry.
///
/// # Errors
/// Propagate independent failure-inventory construction errors.
/// # Panics
/// Fails if first-cause or separate cleanup outcomes change, or any unknown reaches Running.
#[test]
fn real_host_refuses_unknown_context_and_retains_both_cleanup_failures()
-> Result<(), Box<dyn Error>> {
    for failure in unknown_token_observation_failures()? {
        for role in [LabRole::Engine, LabRole::ClockBroker] {
            let observation = Rc::new(RefCell::new(Observations::default()));
            let notification = StopNotification::new();
            let (result, calls) = execute_observation(
                role,
                0,
                &notification,
                Err(failure),
                CaptureAction::None,
                &observation,
            );
            assert_eq!(result, Err(FixtureError::TokenObservation(failure)));
            assert_eq!(calls, 1);
            assert!(!statuses(&observation).iter().any(|status| status[1] == 4));
            assert_eq!(waits(&observation), []);
            assert_eq!(
                statuses(&observation).last(),
                Some(&[16, 1, 0, 1066, 1, 0, 0])
            );
        }
    }
    Ok(())
}

/// Fresh Stop and time guards run after the actual synchronous observer returns.
///
/// # Errors
/// Propagate independent complete-empty-capture failure.
/// # Panics
/// Fails if Stop starts Running, exact/late startup time is renewed, backwards
/// time is reused, a permit remains outstanding, or a just-before neighbor fails.
#[test]
fn real_host_rechecks_stop_and_time_after_capture() -> Result<(), Box<dyn Error>> {
    for role in [LabRole::Engine, LabRole::ClockBroker] {
        for action in [
            CaptureAction::Stop,
            CaptureAction::Advance(Duration::from_millis(9999)),
            CaptureAction::Advance(Duration::from_millis(10_000)),
            CaptureAction::Advance(Duration::from_millis(10_001)),
            CaptureAction::Reverse,
        ] {
            let observation = Rc::new(RefCell::new(Observations::default()));
            let notification = StopNotification::new();
            let (result, calls) = execute_observation(
                role,
                0,
                &notification,
                healthy_snapshot(role),
                action,
                &observation,
            );
            assert_eq!(calls, 1);
            match action {
                CaptureAction::Stop => {
                    result?;
                    assert_eq!(waits(&observation), []);
                }
                CaptureAction::Advance(by) if by == Duration::from_millis(9999) => {
                    result?;
                    assert_eq!(waits(&observation), [Duration::from_secs(60)]);
                    assert!(statuses(&observation).iter().any(|status| status[1] == 4));
                    continue;
                }
                CaptureAction::Advance(_) => assert_eq!(
                    result,
                    Err(FixtureError::Lifecycle(
                        windows_platform::service::LifecycleError::DeadlineElapsed
                    ))
                ),
                CaptureAction::Reverse => assert_eq!(result, Err(FixtureError::ClockReversed)),
                CaptureAction::None => return Err("unselected test action".into()),
            }
            assert!(!statuses(&observation).iter().any(|status| status[1] == 4));
        }
    }
    Ok(())
}

/// Invalid startup and an already delivered Stop admit no observation job at all.
///
/// # Errors
/// Propagate complete-empty-capture fixture failure.
/// # Panics
/// Fails if either role opens token work after arguments/Stop already forbid startup.
#[test]
fn denied_startup_never_invokes_the_token_observer() -> Result<(), Box<dyn Error>> {
    for role in [LabRole::Engine, LabRole::ClockBroker] {
        for stopped in [false, true] {
            let observation = Rc::new(RefCell::new(Observations::default()));
            let notification = StopNotification::new();
            if stopped {
                assert_eq!(notification.handle_control(1), ScmControl::Stop);
            }
            let (result, calls) = execute_observation(
                role,
                if stopped { 0 } else { 2 },
                &notification,
                healthy_snapshot(role),
                CaptureAction::None,
                &observation,
            );
            assert_eq!(calls, 0);
            if stopped {
                result?;
            } else {
                assert_eq!(result, Err(FixtureError::UnsupportedArguments));
            }
            assert_eq!(waits(&observation), []);
        }
    }
    Ok(())
}

/// Locate an arithmetic fixture near the host's representable `Instant` ceiling.
/// Exactly 64 checked additions search whole seconds; no clock is modified.
///
/// # Errors
/// Refuse a platform without the independently checked neighboring fixture range.
fn representability_fixture_origin() -> Result<Instant, Box<dyn Error>> {
    let mut ceiling = Instant::now();
    for bit in (0..u64::BITS).rev() {
        let seconds = 1_u64.checked_shl(bit).ok_or("fixture shift exceeded 63")?;
        if let Some(later) = ceiling.checked_add(Duration::from_secs(seconds)) {
            ceiling = later;
        }
    }
    if ceiling.checked_add(Duration::from_secs(1)).is_some() {
        return Err("fixture search did not reach a whole-second ceiling".into());
    }
    ceiling
        .checked_sub(Duration::from_secs(30))
        .ok_or_else(|| "fixture ceiling has no thirty-second predecessor".into())
}

/// A startup refusal closes ordinary admission before any further preparation
/// observation can consume the separately representable shutdown allowance.
///
/// # Errors
/// Propagate checked construction of the independent neighboring instants.
/// # Panics
/// Fails if denied startup performs an ordinary preparation read before cleanup.
#[test]
fn denied_startup_preserves_the_first_representable_shutdown_observation()
-> Result<(), Box<dyn Error>> {
    let neighbor = representability_fixture_origin()?;
    let ceiling = neighbor
        .checked_add(Duration::from_secs(30))
        .ok_or("fixture ceiling cannot be reconstructed")?;
    assert!(neighbor.checked_add(Duration::from_secs(10)).is_some());
    assert!(ceiling.checked_add(Duration::from_secs(10)).is_none());
    let observation = Rc::new(RefCell::new(Observations::default()));
    let notification = StopNotification::new();
    let mut factory = Factory {
        observations: Rc::clone(&observation),
        failure: None,
    };
    let mut clock = ScriptedClock {
        observations: vec![neighbor, neighbor, ceiling, ceiling],
        reads: 0,
    };
    let mut wait = TestWait {
        time: Rc::new(RefCell::new(Time {
            current: neighbor,
            backwards_once: false,
            backwards_persistent: false,
        })),
        observations: Rc::clone(&observation),
        actions: vec![],
        next: 0,
        notification: &notification,
    };
    assert_eq!(
        run_service(
            LabRole::Engine,
            2,
            &mut factory,
            &mut HealthyTokenObserver(LabRole::Engine),
            &mut clock,
            &mut wait,
            &notification,
        ),
        Err(FixtureError::UnsupportedArguments)
    );
    assert_eq!(clock.reads, 3);
    assert_eq!(waits(&observation), []);
    assert_eq!(
        statuses(&observation),
        [
            [16, 2, 0, 0, 0, 1, 1000],
            [16, 3, 0, 0, 0, 1, 1000],
            [16, 1, 0, 1066, 1, 0, 0],
        ]
    );
    Ok(())
}

/// Running is reported only after its absolute run allowance can be represented.
/// Both real roles exercise all three cases before comparing independent facts.
///
/// # Errors
/// Refuse unrepresentable test inputs, before any hosting case is evaluated.
/// # Panics
/// Fails if overflow reports Running or enters the control loop, startup expiry
/// loses first-cause priority, or the valid sixty-second neighbor is rejected.
#[test]
fn run_deadline_representability_precedes_running_and_preserves_startup_expiry()
-> Result<(), Box<dyn Error>> {
    let bad = representability_fixture_origin()?;
    let expired = bad
        .checked_add(Duration::from_secs(10))
        .ok_or("startup ten-second fixture unavailable")?;
    let good = bad
        .checked_sub(Duration::from_secs(60))
        .ok_or("healthy predecessor unavailable")?;
    assert!(bad.checked_add(Duration::from_secs(60)).is_none());
    assert_eq!(good.checked_add(Duration::from_secs(60)), Some(bad));
    assert!(good.checked_add(Duration::from_secs(10)).is_some());
    assert!(expired.checked_add(Duration::from_secs(60)).is_none());
    assert!(expired.checked_add(Duration::from_secs(10)).is_some());
    let mut actual = Vec::new();
    let mut expected = Vec::new();
    for role in [LabRole::Engine, LabRole::ClockBroker] {
        for (case, startup, ready, expected_error) in [
            ("healthy", good, good, None),
            ("run overflow", bad, bad, Some(FixtureError::ClockOverflow)),
            (
                "startup expired",
                bad,
                expired,
                Some(FixtureError::Lifecycle(
                    windows_platform::service::LifecycleError::DeadlineElapsed,
                )),
            ),
        ] {
            let failed = expected_error.is_some();
            let observation = Rc::new(RefCell::new(Observations::default()));
            let notification = StopNotification::new();
            let mut factory = Factory {
                observations: Rc::clone(&observation),
                failure: None,
            };
            let mut script = vec![startup, startup, startup];
            script.extend(std::iter::repeat_n(ready, if failed { 3 } else { 7 }));
            let mut clock = ScriptedClock {
                observations: script,
                reads: 0,
            };
            let mut wait = TestWait {
                time: Rc::new(RefCell::new(Time {
                    current: Instant::now(),
                    backwards_once: false,
                    backwards_persistent: false,
                })),
                observations: Rc::clone(&observation),
                actions: vec![WaitAction::Stop(Duration::ZERO)],
                next: 0,
                notification: &notification,
            };
            let result = run_service(
                role,
                0,
                &mut factory,
                &mut HealthyTokenObserver(role),
                &mut clock,
                &mut wait,
                &notification,
            );
            actual.push((
                role,
                case,
                result,
                statuses(&observation),
                waits(&observation),
                clock.reads,
            ));
            let mut expected_statuses = vec![[16, 2, 0, 0, 0, 1, 1000]];
            if !failed {
                expected_statuses.push([16, 4, 1, 0, 0, 0, 0]);
            }
            let (win32_code, service_code) = if failed { (1066, 1) } else { (0, 0) };
            expected_statuses.extend([
                [16, 3, 0, 0, 0, 1, 1000],
                [16, 1, 0, win32_code, service_code, 0, 0],
            ]);
            expected.push((
                role,
                case,
                expected_error.map_or(Ok(()), Err),
                expected_statuses,
                if failed {
                    vec![]
                } else {
                    vec![Duration::from_secs(60)]
                },
                if failed { 6 } else { 10 },
            ));
        }
    }
    assert_eq!(actual, expected);
    Ok(())
}

mod worker_tests {
    // SPDX-License-Identifier: GPL-2.0-only
    //! Independently scripted facts consumed by the actual retained service owner.
    //!
    //! These source tests are authored controls, not native SCM qualification. The
    //! worker records phase calls and publishes explicit completion; it never changes
    //! the lifecycle, completes a permit or chooses the host's result.

    use std::{
        cell::RefCell,
        error::Error,
        rc::Rc,
        task::Poll,
        time::{Duration, Instant},
    };

    use windows_platform::{
        privilege_observation::{PrivilegeLuid, PrivilegeSnapshot, ServicePrivilegeIds},
        scm::{ScmControl, ScmStatus, StopNotification},
        service::{LifecycleError, ServiceState, ServiceStatus, StatusReporter},
    };

    use super::super::{
        Clock, InvocationPoll, ServiceInvocation, StatusFactory, TokenObserver, Worker, WorkerWake,
        start_invocation_with_diagnostics,
    };
    use crate::{
        FixtureError, LabRole, NativeError, NativeOperation, TokenCleanupOutcome,
        TokenObservationCause, TokenObservationFailure,
        diagnostics::{MemorySink, Sinks},
        error::WorkerError,
    };

    /// Test construction errors remain distinct from assertions about host behavior.
    type TestResult<T = ()> = Result<T, Box<dyn Error>>;
    /// The actual host owner, with only its external observations replaced.
    type Invocation = ServiceInvocation<Reporter, ScriptedWorker>;

    /// Chronological effects, including the distinction between attempt and success.
    #[derive(Clone, Debug, Eq, PartialEq)]
    enum Event {
        /// The real host requested registration for this fixed role.
        Register(LabRole),
        /// A complete token observation was requested.
        Capture,
        /// An actual lifecycle-selected status was offered to the reporter.
        StatusAttempt([u32; 7]),
        /// That exact status was successfully acknowledged by the reporter.
        StatusAcknowledged([u32; 7]),
        /// Initialization was polled; Pending does not acknowledge completion.
        Prepare,
        /// The operational activation entry was called.
        Activate,
        /// Running worker health was observed.
        Running,
        /// Cancellation was requested without acknowledging completion.
        Cancel,
        /// Actual completion was polled independently from failure or cancellation.
        Completion,
        /// The worker's destructor actually ran.
        WorkerDropped,
        /// The reporter's destructor actually ran.
        ReporterDropped,
    }

    /// Shared independent observations; no field is a synthetic host transition.
    #[derive(Default)]
    #[expect(
        clippy::struct_excessive_bools,
        reason = "independent one-shot fixture fault controls do not encode a production state machine"
    )]
    struct Trace {
        /// Ordered effects made by the actual host and its retained dependencies.
        events: Vec<Event>,
        /// Fixed SCM states whose report attempts return the selected native cause.
        report_failures: Vec<(u32, NativeError)>,
        /// Deliver genuine Stop while a successful Running report is in flight.
        stop_during_running_ack: bool,
        /// An independently selected time published during Running acknowledgement.
        time_during_running_ack: Option<Instant>,
        /// Publish genuine Stop inside the next actual clock observation.
        stop_during_next_clock: bool,
        /// Arm the clock Stop after actual dormant preparation acknowledges Ready.
        stop_on_clock_after_prepare: bool,
        /// Arm the clock Stop after actual Running acknowledgement succeeds.
        stop_on_clock_after_running_ack: bool,
        /// Arm the clock Stop after the real operational activation call returns.
        stop_on_clock_after_activation: bool,
        /// Enable externally visible destructor markers only in the isolated child.
        destructor_markers: bool,
    }

    /// Monotonic input chosen by the test, independently of status or worker verdicts.
    struct TestClock {
        /// Shared input also available to explicitly selected effect hooks.
        time: Rc<RefCell<Instant>>,
        /// One-shot clock effects selected independently from host transitions.
        trace: Rc<RefCell<Trace>>,
        /// Actual sticky notification available to the selected observation hook.
        notification: Rc<StopNotification>,
    }

    impl Clock for TestClock {
        /// Supply exactly the chosen observation, including deliberate reversal.
        fn now(&mut self) -> Instant {
            if std::mem::take(&mut self.trace.borrow_mut().stop_during_next_clock) {
                let _control = self.notification.handle_control(1);
            }
            *self.time.borrow()
        }
    }

    /// A status sink can independently fail or publish Stop before acknowledging.
    struct Reporter {
        /// Shared effects and fixed fault inputs.
        trace: Rc<RefCell<Trace>>,
        /// The actual sticky notification read by the host.
        notification: Rc<StopNotification>,
        /// The actual clock input used for the acknowledgement race controls.
        time: Rc<RefCell<Instant>>,
    }

    impl StatusReporter for Reporter {
        type Error = NativeError;

        /// Retain all seven projected fields and acknowledge only successful reports.
        fn report(&mut self, status: ServiceStatus) -> Result<(), NativeError> {
            let raw = ScmStatus::from_status(status);
            let fields = [
                raw.service_type(),
                raw.current_state(),
                raw.controls_accepted(),
                raw.win32_exit_code(),
                raw.service_specific_exit_code(),
                raw.checkpoint(),
                raw.wait_hint_ms(),
            ];
            let mut trace = self.trace.borrow_mut();
            trace.events.push(Event::StatusAttempt(fields));
            if let Some(error) = trace
                .report_failures
                .iter()
                .find_map(|(state, error)| (*state == fields[1]).then_some(*error))
            {
                return Err(error);
            }
            if fields[1] == 4 {
                if trace.stop_during_running_ack {
                    let _control = self.notification.handle_control(1);
                }
                if let Some(now) = trace.time_during_running_ack {
                    *self.time.borrow_mut() = now;
                }
                if trace.stop_on_clock_after_running_ack {
                    trace.stop_during_next_clock = true;
                }
            }
            trace.events.push(Event::StatusAcknowledged(fields));
            Ok(())
        }
    }

    impl Drop for Reporter {
        /// Observe destruction without performing cleanup or a lifecycle transition.
        fn drop(&mut self) {
            let mut trace = self.trace.borrow_mut();
            trace.events.push(Event::ReporterDropped);
            if trace.destructor_markers {
                eprintln!("fixture-worker-reporter-dropped");
            }
        }
    }

    /// Registration constructs the only status reporter; it grants no worker access.
    struct Factory {
        /// Observations and independently injected report failures.
        trace: Rc<RefCell<Trace>>,
        /// The same notification passed to the actual service owner.
        notification: Rc<StopNotification>,
        /// The same clock input used by the actual service owner.
        time: Rc<RefCell<Instant>>,
    }

    impl StatusFactory for Factory {
        type Reporter = Reporter;

        /// Record the fixed role and return the sole reporter.
        fn register(&mut self, role: LabRole) -> Result<Reporter, NativeError> {
            self.trace.borrow_mut().events.push(Event::Register(role));
            Ok(Reporter {
                trace: Rc::clone(&self.trace),
                notification: Rc::clone(&self.notification),
                time: Rc::clone(&self.time),
            })
        }
    }

    /// Independently selected complete or denied startup token inputs.
    #[derive(Clone, Copy)]
    enum Capture {
        /// A complete independently encoded exact-role privilege observation.
        Healthy,
        /// Independent raw LUID/attribute facts, including unrecognized entries.
        Entries(&'static [(u32, i32, u32)]),
        /// A present but disabled forbidden engine privilege.
        Forbidden,
        /// An original native observation failure with explicit cleanup outcomes.
        Failed,
    }

    /// Construct validated token inputs without reproducing engine admission logic.
    ///
    /// # Errors
    /// Propagate checked privilege identity or complete snapshot construction failure.
    fn snapshot(
        capture: Capture,
        role: LabRole,
    ) -> TestResult<Result<PrivilegeSnapshot, TokenObservationFailure>> {
        let ids = ServicePrivilegeIds::new([
            PrivilegeLuid::new(17, 0),
            PrivilegeLuid::new(23, 0),
            PrivilegeLuid::new(41, 0),
            PrivilegeLuid::new(59, 0),
        ])?;
        Ok(match capture {
            Capture::Healthy => super::healthy_snapshot(role),
            Capture::Entries(entries) => {
                let mut raw = u32::try_from(entries.len())?.to_le_bytes().to_vec();
                for (low, high, attributes) in entries {
                    raw.extend(low.to_le_bytes());
                    raw.extend(high.to_le_bytes());
                    raw.extend(attributes.to_le_bytes());
                }
                Ok(PrivilegeSnapshot::decode(&raw, ids)?)
            }
            Capture::Forbidden => {
                let mut raw = 1_u32.to_le_bytes().to_vec();
                raw.extend(17_u32.to_le_bytes());
                raw.extend(0_i32.to_le_bytes());
                raw.extend(0_u32.to_le_bytes());
                Ok(PrivilegeSnapshot::decode(&raw, ids)?)
            }
            Capture::Failed => Err(TokenObservationFailure::new(
                TokenObservationCause::Native(NativeError::new(
                    NativeOperation::PrivilegeQuery,
                    7331,
                )),
                TokenCleanupOutcome::Closed,
                TokenCleanupOutcome::NotOpened,
            )),
        })
    }

    /// Synchronous capture returns only the predetermined complete result.
    struct Observer {
        /// Independent capture facts, never inferred from the expected host result.
        result: Option<Result<PrivilegeSnapshot, TokenObservationFailure>>,
        /// Ordered evidence of whether startup admission reached this seam.
        trace: Rc<RefCell<Trace>>,
    }

    impl TokenObserver for Observer {
        /// Record the call without completing worker custody or changing status.
        fn observe(&mut self) -> Result<PrivilegeSnapshot, TokenObservationFailure> {
            self.trace.borrow_mut().events.push(Event::Capture);
            self.result
                .take()
                .unwrap_or(Err(TokenObservationFailure::new(
                    TokenObservationCause::InvalidHandle,
                    TokenCleanupOutcome::NotOpened,
                    TokenCleanupOutcome::NotOpened,
                )))
        }
    }

    /// Explicit side effect immediately before a preparation result becomes visible.
    #[derive(Clone, Copy)]
    enum PreparationEffect {
        /// Publish sticky Stop before the host consumes the Ready observation.
        Stop,
        /// Publish an exact deadline or backwards time before Ready is consumed.
        Time(Instant),
    }

    /// Worker-owned phase facts remain level-triggered until the host observes them.
    struct Script {
        /// Initialization readiness or failure, independent from running health.
        preparation: Poll<Result<(), WorkerError>>,
        /// Operational activation can fail after Running acknowledgement.
        activation: Result<(), WorkerError>,
        /// A health failure never changes the separate completion observation.
        running: Result<(), WorkerError>,
        /// Only this observation acknowledges resolved completion and cleanup.
        completion: Poll<Result<(), WorkerError>>,
        /// Optional one-shot change at the preparation-to-readiness boundary.
        preparation_effect: Option<PreparationEffect>,
        /// Wake captured from the actual host, without an independently made token.
        wake: Option<WorkerWake>,
        /// Synthetic private worker input, never an admitted diagnostic field.
        private_context: Vec<u8>,
    }

    impl Default for Script {
        /// Start with unresolved initialization and independently unresolved cleanup.
        fn default() -> Self {
            Self {
                preparation: Poll::Pending,
                activation: Ok(()),
                running: Ok(()),
                completion: Poll::Pending,
                preparation_effect: None,
                wake: None,
                private_context: Vec::new(),
            }
        }
    }

    /// A nonblocking worker with no status reporter, permit or lifecycle capability.
    struct ScriptedWorker {
        /// Level-triggered phase observations and captured wake.
        script: Rc<RefCell<Script>>,
        /// Actual invocation effects observed independently from phase inputs.
        trace: Rc<RefCell<Trace>>,
        /// Actual sticky Stop used by the preparation boundary race.
        notification: Rc<StopNotification>,
        /// Actual clock input used by preparation boundary controls.
        time: Rc<RefCell<Instant>>,
    }

    impl Worker for ScriptedWorker {
        /// Pending is retained; Ready reports physically completed dormant preparation.
        fn poll_prepare(&mut self, wake: &WorkerWake) -> Poll<Result<(), WorkerError>> {
            self.trace.borrow_mut().events.push(Event::Prepare);
            let mut script = self.script.borrow_mut();
            script.wake = Some(wake.clone());
            if let Some(effect) = script.preparation_effect.take() {
                match effect {
                    PreparationEffect::Stop => {
                        let _control = self.notification.handle_control(1);
                    }
                    PreparationEffect::Time(now) => *self.time.borrow_mut() = now,
                }
            }
            if script.preparation == Poll::Ready(Ok(())) {
                let mut trace = self.trace.borrow_mut();
                if trace.stop_on_clock_after_prepare {
                    trace.stop_during_next_clock = true;
                }
            }
            script.preparation
        }

        /// Record the only operational entry and return the chosen activation result.
        fn activate(&mut self) -> Result<(), WorkerError> {
            self.trace.borrow_mut().events.push(Event::Activate);
            let mut trace = self.trace.borrow_mut();
            if trace.stop_on_clock_after_activation {
                trace.stop_during_next_clock = true;
            }
            self.script.borrow().activation
        }

        /// Return health independently from the explicit completion observation.
        fn poll_running(&mut self, wake: &WorkerWake) -> Result<(), WorkerError> {
            self.trace.borrow_mut().events.push(Event::Running);
            let mut script = self.script.borrow_mut();
            script.wake = Some(wake.clone());
            script.running
        }

        /// A cancellation request publishes no completion and releases no resource.
        fn request_cancel(&mut self, wake: &WorkerWake) {
            self.trace.borrow_mut().events.push(Event::Cancel);
            self.script.borrow_mut().wake = Some(wake.clone());
        }

        /// Only the separately selected Finished result can settle worker custody.
        fn poll_finished(&mut self, wake: &WorkerWake) -> Poll<Result<(), WorkerError>> {
            self.trace.borrow_mut().events.push(Event::Completion);
            let mut script = self.script.borrow_mut();
            script.wake = Some(wake.clone());
            script.completion
        }
    }

    impl Drop for ScriptedWorker {
        /// Observe actual destruction without resolving an outstanding operation.
        fn drop(&mut self) {
            let mut trace = self.trace.borrow_mut();
            trace.events.push(Event::WorkerDropped);
            if trace.destructor_markers {
                eprintln!("fixture-worker-worker-dropped");
            }
        }
    }

    /// Independently chosen startup inputs and finite fault script.
    struct Settings {
        /// Fixed role, never a caller-supplied service identity.
        role: LabRole,
        /// SCM's bounded argument count, without reading an argument vector.
        argc: u32,
        /// Complete healthy capture or an explicit startup refusal.
        capture: Capture,
        /// Genuine Stop published before owner construction.
        stopped_before_start: bool,
        /// Selected report failures by their native SCM state number.
        report_failures: Vec<(u32, NativeError)>,
    }

    impl Default for Settings {
        /// Healthy admission, with unresolved worker preparation supplied separately.
        fn default() -> Self {
            Self {
                role: LabRole::Engine,
                argc: 0,
                capture: Capture::Healthy,
                stopped_before_start: false,
                report_failures: Vec::new(),
            }
        }
    }

    /// Retain the real invocation while tests inspect pending/overdue observations.
    /// Ordinary assertion unwinding publishes real test-worker completion first, so
    /// a failing assertion does not accidentally abort the entire test process. The
    /// separate subprocess test exercises unresolved owner destruction directly.
    struct Rig {
        /// Retained actual owner; only the fatal-drop child removes it unresolved.
        invocation: Option<Invocation>,
        /// A refusal before owner return remains inspectable without worker calls.
        start_failure: Option<FixtureError>,
        /// Actual time seam consumed by every owner step.
        clock: TestClock,
        /// Actual sticky notification, also shared with selected seam hooks.
        notification: Rc<StopNotification>,
        /// Worker phase facts controlled independently from owner transitions.
        script: Rc<RefCell<Script>>,
        /// Ordered API and phase observations.
        trace: Rc<RefCell<Trace>>,
        /// An observed Finished step permits ordinary owner destruction.
        finished: bool,
    }

    impl Rig {
        /// Invoke the actual startup path and retain either its owner or first error.
        ///
        /// # Errors
        /// Propagate checked construction of the independently selected token fixture.
        fn start(settings: Settings) -> TestResult<Self> {
            Self::start_with_sinks(settings, Sinks::new(MemorySink::new(true), None))
        }

        /// Pass finite diagnostic sinks to the same actual startup and decision owner.
        ///
        /// # Errors
        /// Propagate checked construction of the independently selected token fixture.
        fn start_with_sinks(settings: Settings, sinks: Sinks) -> TestResult<Self> {
            let trace = Rc::new(RefCell::new(Trace {
                report_failures: settings.report_failures,
                ..Trace::default()
            }));
            let time = Rc::new(RefCell::new(Instant::now()));
            let notification = Rc::new(StopNotification::new());
            if settings.stopped_before_start {
                let _control = notification.handle_control(1);
            }
            let script = Rc::new(RefCell::new(Script::default()));
            let mut factory = Factory {
                trace: Rc::clone(&trace),
                notification: Rc::clone(&notification),
                time: Rc::clone(&time),
            };
            let mut observer = Observer {
                result: Some(snapshot(settings.capture, settings.role)?),
                trace: Rc::clone(&trace),
            };
            let mut clock = TestClock {
                time: Rc::clone(&time),
                trace: Rc::clone(&trace),
                notification: Rc::clone(&notification),
            };
            let worker = ScriptedWorker {
                script: Rc::clone(&script),
                trace: Rc::clone(&trace),
                notification: Rc::clone(&notification),
                time,
            };
            let (invocation, start_failure) = match start_invocation_with_diagnostics(
                settings.role,
                settings.argc,
                &mut factory,
                &mut observer,
                &mut clock,
                &notification,
                (worker, sinks),
            ) {
                Ok(invocation) => (Some(invocation), None),
                Err(error) => (None, Some(error)),
            };
            Ok(Self {
                invocation,
                start_failure,
                clock,
                notification,
                script,
                trace,
                finished: false,
            })
        }

        /// Borrow the actual owner without granting the test a transition capability.
        ///
        /// # Errors
        /// Reject access when startup returned no retained invocation.
        fn owner(&self) -> TestResult<&Invocation> {
            self.invocation
                .as_ref()
                .ok_or_else(|| "startup returned no retained invocation".into())
        }

        /// Make one finite step through the actual service owner.
        ///
        /// # Errors
        /// Reject a step when startup returned no retained invocation.
        fn poll(&mut self) -> TestResult<InvocationPoll> {
            let invocation = self
                .invocation
                .as_mut()
                .ok_or("startup returned no retained invocation")?;
            let poll = invocation.poll(&mut self.clock, &self.notification);
            if matches!(poll, InvocationPoll::Finished) {
                self.finished = true;
            }
            Ok(poll)
        }

        /// Advance only the independent monotonic input with checked arithmetic.
        ///
        /// # Errors
        /// Reject an interval the independent test clock cannot represent.
        fn advance(&mut self, by: Duration) -> TestResult {
            let current = *self.clock.time.borrow();
            *self.clock.time.borrow_mut() = current
                .checked_add(by)
                .ok_or("test clock cannot represent selected interval")?;
            Ok(())
        }

        /// Construct an exact independently chosen observation relative to current time.
        ///
        /// # Errors
        /// Reject an interval the independent test clock cannot represent.
        fn later(&self, by: Duration) -> TestResult<Instant> {
            self.clock
                .time
                .borrow()
                .checked_add(by)
                .ok_or_else(|| "test clock cannot represent selected interval".into())
        }

        /// Construct a representable backwards observation without unchecked math.
        ///
        /// # Errors
        /// Reject a backwards observation the independent test clock cannot represent.
        fn earlier(&self) -> TestResult<Instant> {
            self.clock
                .time
                .borrow()
                .checked_sub(Duration::from_millis(1))
                .ok_or_else(|| "test clock cannot represent reversal".into())
        }

        /// Read only actual calls, not the worker's independently selected input.
        fn events(&self) -> Vec<Event> {
            self.trace.borrow().events.clone()
        }

        /// Read complete actual status attempts, including failed acknowledgements.
        fn statuses(&self) -> Vec<[u32; 7]> {
            self.trace
                .borrow()
                .events
                .iter()
                .filter_map(|event| match event {
                    Event::StatusAttempt(raw) => Some(*raw),
                    _ => None,
                })
                .collect()
        }

        /// Count one named actual worker/API effect without deriving a host verdict.
        fn count(&self, event: &Event) -> usize {
            self.trace
                .borrow()
                .events
                .iter()
                .filter(|observed| *observed == event)
                .count()
        }

        /// Read the first actual failure while retaining the same invocation.
        fn failure(&self) -> Option<&FixtureError> {
            self.invocation
                .as_ref()
                .and_then(ServiceInvocation::failure)
                .or(self.start_failure.as_ref())
        }

        /// Observe that no result is available before Finished or after consumption.
        fn take_result(&mut self) -> Option<Result<(), FixtureError>> {
            self.invocation
                .as_mut()
                .and_then(ServiceInvocation::take_result)
        }

        /// Supply a real test-worker completion and finitely settle the same owner.
        /// No counter, status or lifecycle transition is synthesized by this helper.
        fn settle(
            &mut self,
            completion: Result<(), WorkerError>,
        ) -> Option<Result<(), FixtureError>> {
            self.script.borrow_mut().completion = Poll::Ready(completion);
            let _control = self.notification.handle_control(1);
            let Some(invocation) = self.invocation.as_mut() else {
                return self.start_failure.take().map(Err);
            };
            for _ in 0..8 {
                if matches!(
                    invocation.poll(&mut self.clock, &self.notification),
                    InvocationPoll::Finished
                ) {
                    self.finished = true;
                    return invocation.take_result();
                }
            }
            None
        }
    }

    impl Drop for Rig {
        /// Settle test-owned work on ordinary assertion unwinding. If the host fails
        /// to consume an explicit completion within eight steps, retain the owner
        /// instead of turning that unrelated test failure into a process abort.
        fn drop(&mut self) {
            drop(self.settle(Ok(())));
            if !self.finished
                && let Some(invocation) = self.invocation.take()
            {
                #[expect(
                    clippy::mem_forget,
                    reason = "test-failure containment retains an owner that refused explicit completion"
                )]
                std::mem::forget(invocation);
            }
        }
    }

    /// Independent finite destination conditions, without a callback into the owner.
    #[derive(Clone, Copy)]
    enum DiagnosticSinkCondition {
        /// Every bounded record fits and is accepted.
        Healthy,
        /// All sixteen slots were already occupied before owner construction.
        Full,
        /// The selected output destination refuses every admission attempt.
        Unavailable,
    }

    /// Construct a finite sink condition before the real service startup runs.
    fn diagnostic_sink(condition: DiagnosticSinkCondition) -> MemorySink {
        match condition {
            DiagnosticSinkCondition::Healthy => MemorySink::new(true),
            DiagnosticSinkCondition::Full => MemorySink::full(),
            DiagnosticSinkCondition::Unavailable => MemorySink::new(false),
        }
    }

    /// Compare the actual accepted bytes and independent loss facts at a sink.
    ///
    /// # Errors
    /// Reject an expected record count outside the fixed counter representation.
    /// # Panics
    /// Fails if accepted records, retained first records or loss counts differ.
    fn assert_diagnostic_sink(
        sink: &MemorySink,
        condition: DiagnosticSinkCondition,
        expected: &[[u8; 9]],
    ) -> TestResult {
        let attempted = u32::try_from(expected.len())?;
        match condition {
            DiagnosticSinkCondition::Healthy => {
                assert_eq!(sink.records(), expected);
                assert_eq!(sink.loss(), (0, 0));
            }
            DiagnosticSinkCondition::Full => {
                assert_eq!(sink.records(), &[[0; 9]; 16]);
                assert_eq!(sink.loss(), (attempted, 0));
            }
            DiagnosticSinkCondition::Unavailable => {
                let empty: &[[u8; 9]] = &[];
                assert_eq!(sink.records(), empty);
                assert_eq!(sink.loss(), (0, attempted));
            }
        }
        Ok(())
    }

    /// Actual required INFO output follows acknowledged facts, independently of
    /// optional output. Repeated Pending/Finished observations emit no duplicates.
    ///
    /// # Errors
    /// Propagate checked fixture construction or access to an absent actual owner.
    /// # Panics
    /// Fails if queued output manufactures readiness/completion or loses a transition.
    #[test]
    fn diagnostics_required_info_tracks_actual_readiness_and_completion() -> TestResult {
        for (role, role_code) in [(LabRole::Engine, 1), (LabRole::ClockBroker, 2)] {
            let mut rig = Rig::start_with_sinks(
                Settings {
                    role,
                    ..Settings::default()
                },
                Sinks::new(MemorySink::new(true), None),
            )?;
            let started = [[1, role_code, 1, 1, 0, 1, 0, 0, 0]];
            assert_eq!(rig.owner()?.diagnostics.required().records(), &started);
            assert!(rig.owner()?.diagnostics.optional().is_none());
            for _ in 0..8 {
                assert!(matches!(rig.poll()?, InvocationPoll::Pending(_)));
                assert_eq!(rig.owner()?.outstanding_work(), 1);
                assert_eq!(rig.owner()?.state(), Some(ServiceState::StartPending));
                assert!(rig.take_result().is_none());
            }
            assert_eq!(rig.owner()?.diagnostics.required().records(), &started);
            assert_eq!(rig.count(&Event::Activate), 0);
            rig.script.borrow_mut().preparation = Poll::Ready(Ok(()));
            let _observation = rig.poll()?;
            assert_eq!(rig.owner()?.state(), Some(ServiceState::Running));
            assert_eq!(
                rig.owner()?.diagnostics.required().records(),
                &[
                    [1, role_code, 1, 1, 0, 1, 0, 0, 0],
                    [1, role_code, 2, 1, 0, 2, 0, 0, 0],
                ]
            );
            let _control = rig.notification.handle_control(1);
            for _ in 0..8 {
                assert!(matches!(rig.poll()?, InvocationPoll::Pending(_)));
                assert!(rig.owner()?.admission_closed());
                assert_eq!(rig.owner()?.outstanding_work(), 1);
                assert!(rig.take_result().is_none());
            }
            assert_eq!(rig.count(&Event::Cancel), 1);
            assert_eq!(
                rig.owner()?.diagnostics.required().records(),
                &[
                    [1, role_code, 1, 1, 0, 1, 0, 0, 0],
                    [1, role_code, 2, 1, 0, 2, 0, 0, 0],
                    [1, role_code, 3, 1, 0, 3, 0, 0, 0],
                ]
            );
            assert_eq!(rig.settle(Ok(())), Some(Ok(())));
            for _ in 0..8 {
                assert_eq!(rig.poll()?, InvocationPoll::Finished);
                assert!(rig.take_result().is_none());
            }
            assert_eq!(rig.owner()?.outstanding_work(), 0);
            assert_eq!(
                rig.owner()?.diagnostics.required().records(),
                &[
                    [1, role_code, 1, 1, 0, 1, 0, 0, 0],
                    [1, role_code, 2, 1, 0, 2, 0, 0, 0],
                    [1, role_code, 3, 1, 0, 3, 0, 0, 0],
                    [1, role_code, 8, 1, 0, 4, 0, 0, 0],
                ]
            );
            assert_eq!(rig.owner()?.diagnostics.required().loss(), (0, 0));
        }
        Ok(())
    }

    /// Saturation and unavailable outputs cannot change actual owner outcomes or
    /// discard first/secondary failure facts. Optional failures cannot consume
    /// required capacity; repeated overdue polls cannot multiply diagnostics.
    ///
    /// # Errors
    /// Propagate checked fixtures or absence of required retained failure facts.
    /// # Panics
    /// Fails if any sink condition changes lifecycle, custody, original failure or output.
    #[test]
    fn diagnostics_sink_faults_preserve_primary_cleanup_and_actual_completion() -> TestResult {
        use DiagnosticSinkCondition::{Full, Healthy, Unavailable};

        let stop_error = NativeError::new(NativeOperation::Status, 7443);
        let terminal_error = NativeError::new(NativeOperation::Status, 7441);
        let expected = [
            [1, 1, 1, 1, 0, 1, 0, 0, 0],
            [1, 1, 4, 2, 20, 2, 0, 0, 0],
            [1, 1, 3, 1, 0, 3, 0, 0, 0],
            [1, 1, 6, 2, 10, 4, 0, 0, 0],
            [1, 1, 5, 2, 23, 5, 0, 0, 0],
            [1, 1, 7, 2, 10, 6, 0, 0, 0],
            [1, 1, 8, 1, 0, 7, 0, 0, 0],
        ];
        let mut outcomes = Vec::new();
        for required in [Healthy, Full, Unavailable] {
            for optional in [None, Some(Healthy), Some(Full), Some(Unavailable)] {
                let mut rig = Rig::start_with_sinks(
                    Settings {
                        report_failures: vec![(3, stop_error), (1, terminal_error)],
                        ..Settings::default()
                    },
                    Sinks::new(diagnostic_sink(required), optional.map(diagnostic_sink)),
                )?;
                rig.script.borrow_mut().preparation = Poll::Ready(Err(WorkerError::Preparation));
                let _observation = rig.poll()?;
                assert_eq!(
                    rig.failure(),
                    Some(&FixtureError::Worker(WorkerError::Preparation))
                );
                assert!(rig.owner()?.admission_closed());
                assert_eq!(rig.owner()?.outstanding_work(), 1);
                assert_eq!(rig.count(&Event::Cancel), 1);
                assert_eq!(rig.count(&Event::Activate), 0);
                assert!(rig.take_result().is_none());
                rig.advance(Duration::from_secs(10))?;
                let statuses_before_completion = rig.statuses();
                for _ in 0..8 {
                    assert_eq!(rig.poll()?, InvocationPoll::Overdue);
                    assert_eq!(rig.owner()?.outstanding_work(), 1);
                    assert!(rig.take_result().is_none());
                }
                assert_eq!(rig.statuses(), statuses_before_completion);
                assert!(!rig.statuses().iter().any(|raw| raw[1] == 1));
                assert_diagnostic_sink(
                    rig.owner()?.diagnostics.required(),
                    required,
                    &expected[..4],
                )?;
                let result = rig.settle(Err(WorkerError::Cleanup));
                let Some(Err(FixtureError::Cleanup(facts))) = &result else {
                    return Err("diagnostic sink fault changed the bounded failure result".into());
                };
                assert_eq!(
                    facts.primary(),
                    &FixtureError::Worker(WorkerError::Preparation)
                );
                assert_eq!(facts.completion(), Some(WorkerError::Cleanup));
                assert_eq!(
                    facts.stop_report(),
                    Some(&LifecycleError::Report(stop_error))
                );
                assert_eq!(
                    facts.terminal_report(),
                    Some(&LifecycleError::Report(terminal_error))
                );
                assert_eq!(rig.owner()?.outstanding_work(), 0);
                for _ in 0..8 {
                    assert_eq!(rig.poll()?, InvocationPoll::Finished);
                    assert!(rig.take_result().is_none());
                }
                assert_diagnostic_sink(rig.owner()?.diagnostics.required(), required, &expected)?;
                match (optional, rig.owner()?.diagnostics.optional()) {
                    (None, None) => {}
                    (Some(condition), Some(sink)) => {
                        assert_diagnostic_sink(sink, condition, &expected)?;
                    }
                    _ => return Err("the optional diagnostic route changed presence".into()),
                }
                outcomes.push((rig.events(), rig.statuses(), result));
            }
        }
        for neighboring in outcomes.windows(2) {
            let [first, next] = neighboring else {
                return Err("two-outcome comparison did not contain two outcomes".into());
            };
            assert_eq!(first, next);
        }
        Ok(())
    }

    /// A failed status report is not an acknowledged Running event. Deadline
    /// failure remains separately useful and neither case implies completed work.
    ///
    /// # Errors
    /// Propagate checked fixtures or absence of required actual owner observations.
    /// # Panics
    /// Fails if reporting or deadline failure is mislabeled or manufactures readiness.
    #[test]
    fn diagnostics_distinguish_failed_running_acknowledgement_from_deadline() -> TestResult {
        let report_error = NativeError::new(NativeOperation::Status, 7414);
        for deadline in [false, true] {
            let mut rig = Rig::start_with_sinks(
                Settings {
                    report_failures: if deadline {
                        Vec::new()
                    } else {
                        vec![(4, report_error)]
                    },
                    ..Settings::default()
                },
                Sinks::new(MemorySink::new(true), Some(MemorySink::new(true))),
            )?;
            if deadline {
                let _observation = rig.poll()?;
                rig.advance(Duration::from_secs(10))?;
            } else {
                rig.script.borrow_mut().preparation = Poll::Ready(Ok(()));
            }
            let _observation = rig.poll()?;
            let cause_code = if deadline { 11 } else { 10 };
            let expected = [
                [1, 1, 1, 1, 0, 1, 0, 0, 0],
                [1, 1, 4, 2, cause_code, 2, 0, 0, 0],
                [1, 1, 3, 1, 0, 3, 0, 0, 0],
            ];
            assert_eq!(rig.owner()?.diagnostics.required().records(), &expected);
            // Pending initialization retains its charge. Ready acknowledged real
            // initialization completion; refused Running admits no running charge.
            assert_eq!(rig.owner()?.outstanding_work(), u32::from(deadline));
            assert_eq!(rig.count(&Event::Activate), 0);
            assert_eq!(rig.count(&Event::Cancel), 1);
            assert!(rig.take_result().is_none());
            assert_eq!(
                rig.failure(),
                Some(&FixtureError::Lifecycle(if deadline {
                    LifecycleError::DeadlineElapsed
                } else {
                    LifecycleError::Report(report_error)
                }))
            );
            assert!(matches!(rig.settle(Ok(())), Some(Err(_))));
            assert_eq!(rig.owner()?.outstanding_work(), 0);
            let mut completed = expected.to_vec();
            completed.push([1, 1, 8, 1, 0, 4, 0, 0, 0]);
            assert_eq!(rig.owner()?.diagnostics.required().records(), completed);
            let optional = rig
                .owner()?
                .diagnostics
                .optional()
                .ok_or("healthy optional sink disappeared")?;
            assert_eq!(optional.records(), completed);
        }
        Ok(())
    }

    /// Recognizable private values stay in the actual worker input on both the
    /// success and rejection paths. Inspect both real output byte streams and
    /// retain useful permitted event/failure context beside secret exclusion.
    ///
    /// # Errors
    /// Propagate checked fixtures or absence of an expected optional destination.
    /// # Panics
    /// Fails if private worker inputs enter admitted records or useful output disappears.
    #[test]
    fn diagnostics_actual_success_and_failure_outputs_exclude_worker_secrets() -> TestResult {
        let secrets: [&[u8]; 4] = [
            b"SYNTHETIC_NTS_COOKIE_8e3d57a1",
            b"SYNTHETIC_TLS_KEY_d2099c52",
            b"SYNTHETIC_APPROVAL_TOKEN_09ac1f3b",
            b"C:\\private-user\\SYNTHETIC_PATH_448c",
        ];
        let private_input = secrets.concat();
        for failed in [false, true] {
            let mut rig = Rig::start_with_sinks(
                Settings::default(),
                Sinks::new(MemorySink::new(true), Some(MemorySink::new(true))),
            )?;
            {
                let mut script = rig.script.borrow_mut();
                script.private_context.clone_from(&private_input);
                script.preparation = if failed {
                    Poll::Ready(Err(WorkerError::Preparation))
                } else {
                    Poll::Ready(Ok(()))
                };
            }
            let _observation = rig.poll()?;
            assert_eq!(rig.script.borrow().private_context, private_input);
            let result = rig.settle(Ok(()));
            assert_eq!(
                result,
                Some(if failed {
                    Err(FixtureError::Worker(WorkerError::Preparation))
                } else {
                    Ok(())
                })
            );
            let expected = if failed {
                [
                    [1, 1, 1, 1, 0, 1, 0, 0, 0],
                    [1, 1, 4, 2, 20, 2, 0, 0, 0],
                    [1, 1, 3, 1, 0, 3, 0, 0, 0],
                    [1, 1, 8, 1, 0, 4, 0, 0, 0],
                ]
            } else {
                [
                    [1, 1, 1, 1, 0, 1, 0, 0, 0],
                    [1, 1, 2, 1, 0, 2, 0, 0, 0],
                    [1, 1, 3, 1, 0, 3, 0, 0, 0],
                    [1, 1, 8, 1, 0, 4, 0, 0, 0],
                ]
            };
            let optional = rig
                .owner()?
                .diagnostics
                .optional()
                .ok_or("healthy optional sink disappeared")?;
            for sink in [rig.owner()?.diagnostics.required(), optional] {
                assert_eq!(sink.records(), &expected);
                let emitted = sink.records().iter().flatten().copied().collect::<Vec<_>>();
                for secret in secrets {
                    assert!(!emitted.windows(secret.len()).any(|bytes| bytes == secret));
                }
                assert_eq!(sink.loss(), (0, 0));
            }
        }
        Ok(())
    }

    /// Native numeric failure payloads remain available in original primary and
    /// secondary facts, while only reviewed cause categories reach both sinks.
    ///
    /// # Errors
    /// Propagate fixture construction or absence of the expected bounded failure.
    /// # Panics
    /// Fails if native payloads escape, useful categories vanish or original facts change.
    #[test]
    fn diagnostics_native_primary_and_secondary_payloads_remain_unformatted() -> TestResult {
        let native_secrets = [*b"KEY!", *b"TOKN", *b"PATH"];
        let running_error = NativeError::new(
            NativeOperation::Status,
            u32::from_le_bytes(native_secrets[0]),
        );
        let stop_error = NativeError::new(
            NativeOperation::Status,
            u32::from_le_bytes(native_secrets[1]),
        );
        let terminal_error = NativeError::new(
            NativeOperation::Status,
            u32::from_le_bytes(native_secrets[2]),
        );
        let mut rig = Rig::start_with_sinks(
            Settings {
                report_failures: vec![(4, running_error), (3, stop_error), (1, terminal_error)],
                ..Settings::default()
            },
            Sinks::new(MemorySink::new(true), Some(MemorySink::new(true))),
        )?;
        rig.script.borrow_mut().preparation = Poll::Ready(Ok(()));
        let _observation = rig.poll()?;
        assert_eq!(rig.count(&Event::Activate), 0);
        assert_eq!(rig.owner()?.outstanding_work(), 0);
        assert_eq!(rig.count(&Event::Cancel), 1);
        assert!(rig.take_result().is_none());
        let result = rig.settle(Err(WorkerError::Cleanup));
        let Some(Err(FixtureError::Cleanup(facts))) = &result else {
            return Err("native diagnostic projection changed the original failure".into());
        };
        assert_eq!(
            facts.primary(),
            &FixtureError::Lifecycle(LifecycleError::Report(running_error))
        );
        assert_eq!(facts.completion(), Some(WorkerError::Cleanup));
        assert_eq!(
            facts.stop_report(),
            Some(&LifecycleError::Report(stop_error))
        );
        assert_eq!(
            facts.terminal_report(),
            Some(&LifecycleError::Report(terminal_error))
        );
        let expected = [
            [1, 1, 1, 1, 0, 1, 0, 0, 0],
            [1, 1, 4, 2, 10, 2, 0, 0, 0],
            [1, 1, 3, 1, 0, 3, 0, 0, 0],
            [1, 1, 6, 2, 10, 4, 0, 0, 0],
            [1, 1, 5, 2, 23, 5, 0, 0, 0],
            [1, 1, 7, 2, 10, 6, 0, 0, 0],
            [1, 1, 8, 1, 0, 7, 0, 0, 0],
        ];
        let optional = rig
            .owner()?
            .diagnostics
            .optional()
            .ok_or("healthy optional sink disappeared")?;
        for sink in [rig.owner()?.diagnostics.required(), optional] {
            assert_eq!(sink.records(), &expected);
            let emitted = sink.records().iter().flatten().copied().collect::<Vec<_>>();
            for secret in native_secrets {
                assert!(!emitted.windows(secret.len()).any(|bytes| bytes == secret));
            }
            assert_eq!(sink.loss(), (0, 0));
        }
        Ok(())
    }

    /// Actual time observations can publish Stop before readiness, activation or
    /// health. A preparation observation also retains the existing fresh guard.
    ///
    /// # Errors
    /// Propagate checked fixture construction or missing retained-owner errors.
    /// # Panics
    /// Fails if Stop observed during time acquisition admits a later phase effect.
    fn check_stop_during_clock_observation(boundary: &str) -> TestResult {
        for publish_stop in [false, true] {
            let mut rig = Rig::start(Settings::default())?;
            let prepared = boundary != "preparation";
            if prepared {
                rig.script.borrow_mut().preparation = Poll::Ready(Ok(()));
            }
            {
                let mut trace = rig.trace.borrow_mut();
                match boundary {
                    "preparation" => trace.stop_during_next_clock = publish_stop,
                    "readiness" => trace.stop_on_clock_after_prepare = publish_stop,
                    "activation" => trace.stop_on_clock_after_running_ack = publish_stop,
                    _ => trace.stop_on_clock_after_activation = publish_stop,
                }
            }
            let _observation = rig.poll()?;
            let calls = [
                rig.count(&Event::Prepare),
                rig.count(&Event::Activate),
                rig.count(&Event::Running),
                rig.count(&Event::Cancel),
            ];
            let statuses = rig.statuses();
            let admission_closed = rig.owner()?.admission_closed();
            let outstanding_work = rig.owner()?.outstanding_work();
            let failed = rig.failure().is_some();
            let settled = rig.settle(Ok(()));
            assert_eq!(
                calls,
                [
                    usize::from(!publish_stop || prepared),
                    usize::from(prepared && (!publish_stop || boundary == "health")),
                    usize::from(!publish_stop && prepared),
                    usize::from(publish_stop && prepared),
                ],
                "{boundary}, Stop={publish_stop}"
            );
            assert_eq!(
                statuses.iter().filter(|raw| raw[1] == 4).count(),
                usize::from(
                    prepared && (!publish_stop || matches!(boundary, "activation" | "health"))
                ),
                "{boundary}, Stop={publish_stop}"
            );
            assert_eq!(admission_closed, publish_stop);
            assert_eq!(
                outstanding_work,
                u32::from(!publish_stop || boundary == "health")
            );
            assert!(!failed);
            assert_eq!(settled, Some(Ok(())));
        }
        Ok(())
    }

    /// The existing preparation guard sees Stop published by its clock read.
    ///
    /// # Errors
    /// Propagate checked fixture construction or missing retained-owner errors.
    /// # Panics
    /// Fails if initialization begins after the real clock publishes Stop.
    #[test]
    fn preparation_clock_stop_prevents_initialization() -> TestResult {
        check_stop_during_clock_observation("preparation")
    }

    /// Ready cannot be reported after its time observation publishes Stop.
    ///
    /// # Errors
    /// Propagate checked fixture construction or missing retained-owner errors.
    /// # Panics
    /// Fails if Running or activation follows a Stop during readiness observation.
    #[test]
    fn readiness_clock_stop_prevents_running_acknowledgement() -> TestResult {
        check_stop_during_clock_observation("readiness")
    }

    /// Acknowledged Running grants no activation after a fresh clock Stop.
    ///
    /// # Errors
    /// Propagate checked fixture construction or missing retained-owner errors.
    /// # Panics
    /// Fails if operational work starts after the activation guard's clock Stop.
    #[test]
    fn activation_clock_stop_prevents_operational_work() -> TestResult {
        check_stop_during_clock_observation("activation")
    }

    /// Activation's later time observation can close admission before health work.
    ///
    /// # Errors
    /// Propagate checked fixture construction or missing retained-owner errors.
    /// # Panics
    /// Fails if health work follows Stop published by the post-activation clock.
    #[test]
    fn post_activation_clock_stop_prevents_health_work() -> TestResult {
        check_stop_during_clock_observation("health")
    }

    /// Preparation observation is useful only before the fixed startup deadline;
    /// expiry transitions directly to cancellation while retaining the same charge.
    ///
    /// # Errors
    /// Propagate checked fixture construction or clock arithmetic.
    /// # Panics
    /// Fails if preparation is polled again after the startup allowance expires.
    #[test]
    fn startup_expiry_cancels_without_another_preparation_poll() -> TestResult {
        let mut rig = Rig::start(Settings::default())?;
        assert_eq!(
            rig.poll()?,
            InvocationPoll::Pending(Duration::from_secs(10))
        );
        rig.advance(Duration::from_millis(9_999))?;
        assert_eq!(
            rig.poll()?,
            InvocationPoll::Pending(Duration::from_millis(1))
        );
        assert_eq!(rig.count(&Event::Prepare), 2);
        rig.advance(Duration::from_millis(1))?;
        assert_eq!(
            rig.poll()?,
            InvocationPoll::Pending(Duration::from_secs(10))
        );
        assert_eq!(rig.count(&Event::Prepare), 2);
        assert_eq!(rig.count(&Event::Cancel), 1);
        assert_eq!(rig.owner()?.outstanding_work(), 1);
        assert_eq!(
            rig.failure(),
            Some(&FixtureError::Lifecycle(LifecycleError::DeadlineElapsed))
        );
        assert!(matches!(rig.settle(Ok(())), Some(Err(_))));
        Ok(())
    }

    /// The private pending checker itself marks an elapsed allowance overdue,
    /// before a caller replaces that phase with a separate shutdown budget.
    ///
    /// # Errors
    /// Propagate checked fixture construction or clock arithmetic.
    /// # Panics
    /// Fails if the helper omits exact expiry or misclassifies the valid neighbor.
    #[test]
    fn pending_checker_marks_exact_expiry_before_shutdown_can_replace_the_budget() -> TestResult {
        for expired in [false, true] {
            let mut rig = Rig::start(Settings::default())?;
            let _observation = rig.poll()?;
            rig.advance(Duration::from_millis(if expired { 10_000 } else { 9_999 }))?;
            let now = *rig.clock.time.borrow();
            let invocation = rig
                .invocation
                .as_mut()
                .ok_or("missing retained invocation")?;
            invocation.check_pending(now);
            assert_eq!(invocation.overdue, expired);
            assert_eq!(invocation.admission_closed(), expired);
            assert_eq!(invocation.outstanding_work(), 1);
            if expired {
                assert_eq!(
                    invocation.failure(),
                    Some(&FixtureError::Lifecycle(LifecycleError::DeadlineElapsed))
                );
            } else {
                assert!(invocation.failure().is_none());
            }
            assert_eq!(
                rig.settle(Ok(())).is_some_and(|result| result.is_err()),
                expired
            );
        }
        Ok(())
    }

    /// A reversed source makes ordinary waiting unusable at the Stop helper's
    /// return, even while its historical shutdown deadline remains representable.
    ///
    /// # Errors
    /// Propagate checked fixture construction or a representable earlier instant.
    /// # Panics
    /// Fails if reversed cleanup time leaves the helper permitting ordinary waits.
    #[test]
    fn begin_stop_marks_unusable_time_overdue_before_completion_polling() -> TestResult {
        for reversed in [false, true] {
            let mut rig = Rig::start(Settings::default())?;
            let _observation = rig.poll()?;
            if reversed {
                *rig.clock.time.borrow_mut() = rig.earlier()?;
            }
            let invocation = rig
                .invocation
                .as_mut()
                .ok_or("missing retained invocation")?;
            invocation.begin_stop(&mut rig.clock);
            assert!(invocation.phase_deadline.is_some());
            assert_eq!(invocation.overdue, reversed);
            assert!(invocation.admission_closed());
            assert_eq!(invocation.outstanding_work(), 1);
            if reversed {
                assert_eq!(invocation.failure(), Some(&FixtureError::ClockReversed));
            } else {
                assert!(invocation.failure().is_none());
            }
            assert_eq!(rig.count(&Event::Cancel), 1);
            assert_eq!(
                rig.settle(Ok(())).is_some_and(|result| result.is_err()),
                reversed
            );
        }
        Ok(())
    }

    /// The first exact Stop expiry is immediately inspectable; a later failed
    /// completion remains a secondary fact rather than replacing that cause.
    ///
    /// # Errors
    /// Propagate checked fixture construction or clock arithmetic.
    /// # Panics
    /// Fails if overdue observation delays the first cause until worker completion.
    #[test]
    fn stop_expiry_precedes_and_survives_late_failed_completion() -> TestResult {
        let mut rig = Rig::start(Settings::default())?;
        let _observation = rig.poll()?;
        let _control = rig.notification.handle_control(1);
        assert_eq!(
            rig.poll()?,
            InvocationPoll::Pending(Duration::from_secs(10))
        );
        rig.advance(Duration::from_millis(9_999))?;
        assert_eq!(
            rig.poll()?,
            InvocationPoll::Pending(Duration::from_millis(1))
        );
        assert!(rig.failure().is_none());
        rig.advance(Duration::from_millis(1))?;
        assert_eq!(rig.poll()?, InvocationPoll::Overdue);
        assert_eq!(
            rig.failure(),
            Some(&FixtureError::Lifecycle(LifecycleError::DeadlineElapsed))
        );
        assert_eq!(rig.owner()?.outstanding_work(), 1);
        assert!(!rig.statuses().iter().any(|raw| raw[1] == 1));
        let result = rig.settle(Err(WorkerError::Cleanup));
        assert!(matches!(&result, Some(Err(FixtureError::Cleanup(_)))));
        let Some(Err(FixtureError::Cleanup(facts))) = result else {
            return Err("late completion lost its earlier deadline cause".into());
        };
        assert_eq!(
            facts.primary(),
            &FixtureError::Lifecycle(LifecycleError::DeadlineElapsed)
        );
        assert_eq!(facts.completion(), Some(WorkerError::Cleanup));
        assert!(facts.stop_report().is_none());
        assert!(facts.terminal_report().is_none());
        assert_eq!(rig.owner()?.outstanding_work(), 0);
        Ok(())
    }

    /// Every permitted independent secondary fact survives by itself; successful
    /// neighboring effects cannot authorize discarding that one failure.
    ///
    /// # Errors
    /// Propagate checked fixture construction or absent bounded failure facts.
    /// # Panics
    /// Fails if an individual completion, Stop or terminal failure loses custody.
    #[test]
    fn each_single_secondary_failure_retains_the_original_primary() -> TestResult {
        let stop_error = NativeError::new(NativeOperation::Status, 7453);
        let terminal_error = NativeError::new(NativeOperation::Status, 7451);
        for secondary in ["none", "completion", "stop", "terminal"] {
            let mut rig = Rig::start(Settings {
                report_failures: match secondary {
                    "stop" => vec![(3, stop_error)],
                    "terminal" => vec![(1, terminal_error)],
                    _ => vec![],
                },
                ..Settings::default()
            })?;
            rig.script.borrow_mut().preparation = Poll::Ready(Err(WorkerError::Preparation));
            let _observation = rig.poll()?;
            let result = rig.settle(if secondary == "completion" {
                Err(WorkerError::Cleanup)
            } else {
                Ok(())
            });
            if secondary == "none" {
                assert_eq!(
                    result,
                    Some(Err(FixtureError::Worker(WorkerError::Preparation)))
                );
            } else {
                assert!(matches!(&result, Some(Err(FixtureError::Cleanup(_)))));
                let Some(Err(FixtureError::Cleanup(facts))) = result else {
                    return Err(format!("lost the sole {secondary} secondary failure").into());
                };
                assert_eq!(
                    facts.primary(),
                    &FixtureError::Worker(WorkerError::Preparation)
                );
                assert_eq!(
                    facts.completion(),
                    (secondary == "completion").then_some(WorkerError::Cleanup)
                );
                assert_eq!(
                    facts.stop_report(),
                    (secondary == "stop").then_some(&LifecycleError::Report(stop_error))
                );
                assert_eq!(
                    facts.terminal_report(),
                    (secondary == "terminal").then_some(&LifecycleError::Report(terminal_error))
                );
            }
            assert_eq!(rig.owner()?.outstanding_work(), 0);
            assert_eq!(rig.statuses().iter().filter(|raw| raw[1] == 3).count(), 1);
            assert_eq!(rig.statuses().iter().filter(|raw| raw[1] == 1).count(), 1);
            assert!(rig.take_result().is_none());
        }
        Ok(())
    }

    /// Neither unresolved preparation nor a status attempt admits operational work.
    /// Both roles retain the exact seven-field status contract through healthy Stop.
    ///
    /// # Panics
    /// Fails if readiness, successful Running acknowledgement, custody or exact status fields differ.
    ///
    /// # Errors
    /// Propagate checked fixture construction or missing retained-owner errors.
    #[test]
    fn readiness_and_acknowledged_running_precede_operational_activation() -> TestResult {
        for role in [LabRole::Engine, LabRole::ClockBroker] {
            let mut rig = Rig::start(Settings {
                role,
                ..Settings::default()
            })?;
            assert!(matches!(rig.poll()?, InvocationPoll::Pending(_)));
            assert_eq!(rig.owner()?.state(), Some(ServiceState::StartPending));
            assert_eq!(rig.owner()?.outstanding_work(), 1);
            assert!(!rig.statuses().iter().any(|raw| raw[1] == 4));
            assert_eq!(rig.count(&Event::Activate), 0);

            rig.script.borrow_mut().preparation = Poll::Ready(Ok(()));
            let _observation = rig.poll()?;
            assert_eq!(rig.owner()?.state(), Some(ServiceState::Running));
            assert_eq!(rig.owner()?.outstanding_work(), 1);
            assert_eq!(rig.count(&Event::Activate), 1);
            let events = rig.events();
            let acknowledged = events
                .iter()
                .position(|event| matches!(event, Event::StatusAcknowledged(raw) if raw[1] == 4));
            let activated = events.iter().position(|event| *event == Event::Activate);
            assert!(
                acknowledged
                    .zip(activated)
                    .is_some_and(|(ack, run)| ack < run)
            );

            let _control = rig.notification.handle_control(1);
            assert!(matches!(rig.poll()?, InvocationPoll::Pending(_)));
            assert!(rig.owner()?.admission_closed());
            assert_eq!(rig.owner()?.outstanding_work(), 1);
            assert_eq!(rig.count(&Event::Cancel), 1);
            assert!(!rig.statuses().iter().any(|raw| raw[1] == 1));
            assert_eq!(rig.settle(Ok(())), Some(Ok(())));
            assert_eq!(rig.owner()?.outstanding_work(), 0);
            assert_eq!(
                rig.statuses(),
                [
                    [16, 2, 0, 0, 0, 1, 1000],
                    [16, 4, 1, 0, 0, 0, 0],
                    [16, 3, 0, 0, 0, 1, 1000],
                    [16, 1, 0, 0, 0, 0, 0],
                ]
            );
        }
        Ok(())
    }

    /// Architecture §4.1.1 admits exactly `ChangeNotify` for Engine and additionally
    /// disabled Systemtime for Broker. LUID 59 is an independent fixture lookup
    /// fact for `ChangeNotify`, not an assumed Windows identifier or policy output.
    ///
    /// # Errors
    /// Propagate checked raw decoding or access to the actual retained owner.
    /// # Panics
    /// Fails if either exact neighboring profile cannot prepare and become Running.
    #[test]
    fn exact_role_profiles_reach_worker_readiness() -> TestResult {
        /// Fixed independent lookup and enabled-state facts for the healthy engine.
        const ENGINE: &[(u32, i32, u32)] = &[(59, 0, 2)];
        /// Broker ordinary work adds present, disabled Systemtime.
        const BROKER: &[(u32, i32, u32)] = &[(59, 0, 2), (17, 0, 0)];
        for (role, entries) in [(LabRole::Engine, ENGINE), (LabRole::ClockBroker, BROKER)] {
            let mut rig = Rig::start(Settings {
                role,
                capture: Capture::Entries(entries),
                ..Settings::default()
            })?;
            assert!(rig.failure().is_none());
            assert!(!rig.owner()?.admission_closed());
            assert_eq!(rig.count(&Event::Prepare), 0);
            rig.script.borrow_mut().preparation = Poll::Ready(Ok(()));
            let _observation = rig.poll()?;
            assert_eq!(rig.owner()?.state(), Some(ServiceState::Running));
            assert_eq!(
                [
                    rig.count(&Event::Capture),
                    rig.count(&Event::Prepare),
                    rig.count(&Event::Activate),
                    rig.count(&Event::Running),
                ],
                [1, 1, 1, 1]
            );
            assert_eq!(
                rig.statuses(),
                [[16, 2, 0, 0, 0, 1, 1000], [16, 4, 1, 0, 0, 0, 0]]
            );
            assert_eq!(rig.settle(Ok(())), Some(Ok(())));
        }
        Ok(())
    }

    /// A disabled `ChangeNotify` is still present and explicitly admitted for both
    /// roles; this must remain distinct from the missing-required counterexamples.
    ///
    /// # Errors
    /// Propagate independent checked fixtures or missing actual owner errors.
    /// # Panics
    /// Fails if `ChangeNotify` enabledness becomes an additional startup requirement.
    #[test]
    fn both_roles_admit_disabled_change_notify() -> TestResult {
        /// Independently encoded ordinary engine profile with Notify disabled.
        const ENGINE: &[(u32, i32, u32)] = &[(59, 0, 0)];
        /// Independently encoded broker profile with both required entries disabled.
        const BROKER: &[(u32, i32, u32)] = &[(59, 0, 0), (17, 0, 0)];
        for (role, entries) in [(LabRole::Engine, ENGINE), (LabRole::ClockBroker, BROKER)] {
            let mut rig = Rig::start(Settings {
                role,
                capture: Capture::Entries(entries),
                ..Settings::default()
            })?;
            rig.script.borrow_mut().preparation = Poll::Ready(Ok(()));
            let _observation = rig.poll()?;
            assert!(rig.failure().is_none());
            assert_eq!(rig.owner()?.state(), Some(ServiceState::Running));
            assert_eq!(rig.count(&Event::Prepare), 1);
            assert_eq!(rig.count(&Event::Activate), 1);
            assert_eq!(rig.settle(Ok(())), Some(Ok(())));
        }
        Ok(())
    }

    /// Observe every supplied counterexample before asserting, so an early RED
    /// cannot hide its neighboring case. Only the actual owner makes transitions;
    /// explicit Ready would expose any accidental worker admission immediately.
    ///
    /// # Errors
    /// Propagate checked decoding or access to the actual retained owner.
    /// # Panics
    /// Fails if refusal is absent, untyped, late, or reports a healthy terminal state.
    fn assert_profiles_refused_before_worker(
        role: LabRole,
        captures: &[&'static [(u32, i32, u32)]],
    ) -> TestResult {
        let mut observations = Vec::new();
        for &entries in captures {
            let mut rig = Rig::start(Settings {
                role,
                capture: Capture::Entries(entries),
                ..Settings::default()
            })?;
            let typed_startup_refusal =
                matches!(rig.failure(), Some(FixtureError::PrivilegeAdmission(_)));
            let closed_before_worker = rig.owner()?.admission_closed();
            rig.script.borrow_mut().preparation = Poll::Ready(Ok(()));
            let _observation = rig.poll()?;
            let calls = [
                rig.count(&Event::Capture),
                rig.count(&Event::Prepare),
                rig.count(&Event::Activate),
                rig.count(&Event::Running),
                rig.count(&Event::Cancel),
                rig.count(&Event::Completion),
            ];
            let result = rig.settle(Ok(()));
            observations.push((
                typed_startup_refusal,
                closed_before_worker,
                calls,
                rig.statuses(),
                matches!(
                    result.as_ref(),
                    Some(Err(FixtureError::PrivilegeAdmission(_)))
                ),
                matches!(result.as_ref(), Some(Ok(()))),
            ));
        }
        assert_eq!(
            observations,
            vec![
                (
                    true,
                    true,
                    [1, 0, 0, 0, 0, 0],
                    vec![
                        [16, 2, 0, 0, 0, 1, 1000],
                        [16, 3, 0, 0, 0, 1, 1000],
                        [16, 1, 0, 1066, 1, 0, 0],
                    ],
                    true,
                    false,
                );
                captures.len()
            ]
        );
        Ok(())
    }

    /// Complete-set admission must reject missing `ChangeNotify` and a disabled
    /// unknown extra, even though all three formerly checked privileges are absent.
    ///
    /// # Errors
    /// Propagate checked fixtures or missing actual owner errors.
    /// # Panics
    /// Fails if either independently encoded profile reaches a worker phase.
    #[test]
    fn engine_missing_or_extra_disabled_privileges_refuse_before_worker() -> TestResult {
        assert_profiles_refused_before_worker(LabRole::Engine, &[&[], &[(59, 0, 2), (73, 0, 0)]])
    }

    /// Broker ordinary startup requires disabled Systemtime and no extra entry;
    /// the disabled unknown privilege cannot disappear during snapshot decoding.
    ///
    /// # Errors
    /// Propagate checked fixtures or missing actual owner errors.
    /// # Panics
    /// Fails if enabled clock privilege or a disabled extra reaches ordinary work.
    #[test]
    fn broker_enabled_clock_or_extra_disabled_privileges_refuse_before_worker() -> TestResult {
        assert_profiles_refused_before_worker(
            LabRole::ClockBroker,
            &[
                &[(59, 0, 2), (17, 0, 2)],
                &[(59, 0, 2), (17, 0, 0), (73, 0, 0)],
            ],
        )
    }

    /// Additional missing, known-extra and unknown/full-LUID counterexamples use
    /// the same actual consumer and forbidden-effect observations as retained RED.
    ///
    /// # Errors
    /// Propagate independently encoded fixtures or missing actual owner errors.
    /// # Panics
    /// Fails if a partial set or enabled/disabled extra invokes any worker phase.
    #[test]
    fn both_roles_reject_partial_and_extra_profiles_before_worker() -> TestResult {
        assert_profiles_refused_before_worker(
            LabRole::Engine,
            &[
                &[(59, 0, 0), (73, 0, 2)],
                &[(59, 0, 0), (59, 1, 0)],
                &[(73, 0, 0)],
            ],
        )?;
        assert_profiles_refused_before_worker(
            LabRole::ClockBroker,
            &[
                &[],
                &[(59, 0, 0)],
                &[(17, 0, 0)],
                &[(59, 0, 0), (17, 0, 0), (23, 0, 0)],
                &[(59, 0, 0), (17, 0, 0), (41, 0, 2)],
                &[(59, 0, 0), (17, 0, 0), (73, 0, 2)],
                &[(59, 0, 0), (17, 0, 0), (59, 1, 0)],
            ],
        )
    }

    /// New profile failures retain the existing fixed required cause and never
    /// emit a readiness event, even when the optional sink is absent.
    ///
    /// # Errors
    /// Propagate independent raw fixtures or access to the actual retained owner.
    /// # Panics
    /// Fails if role refusal changes required routing, cause category or event order.
    #[test]
    fn both_role_refusals_emit_required_profile_failure_without_readiness() -> TestResult {
        /// Missing required Engine privilege, independently encoded.
        const ENGINE: &[(u32, i32, u32)] = &[];
        /// Enabled clock privilege on otherwise exact Broker profile.
        const BROKER: &[(u32, i32, u32)] = &[(59, 0, 2), (17, 0, 2)];
        for (role, role_code, entries) in [
            (LabRole::Engine, 1, ENGINE),
            (LabRole::ClockBroker, 2, BROKER),
        ] {
            let mut rig = Rig::start_with_sinks(
                Settings {
                    role,
                    capture: Capture::Entries(entries),
                    ..Settings::default()
                },
                Sinks::new(MemorySink::new(true), None),
            )?;
            assert!(matches!(
                rig.settle(Ok(())),
                Some(Err(FixtureError::PrivilegeAdmission(_)))
            ));
            assert_eq!(
                rig.owner()?.diagnostics.required().records(),
                &[
                    [1, role_code, 1, 1, 0, 1, 0, 0, 0],
                    [1, role_code, 4, 2, 25, 2, 0, 0, 0],
                    [1, role_code, 3, 1, 0, 3, 0, 0, 0],
                    [1, role_code, 8, 1, 0, 4, 0, 0, 0],
                ]
            );
            assert!(rig.owner()?.diagnostics.optional().is_none());
            assert_eq!(rig.count(&Event::Prepare), 0);
            assert_eq!(rig.count(&Event::Activate), 0);
        }
        Ok(())
    }

    /// Valid-shaped startup refusals use the actual entry and invoke no worker phase.
    ///
    /// # Panics
    /// Fails if a startup refusal invokes any worker phase or reports Running.
    ///
    /// # Errors
    /// Propagate checked fixture construction or missing retained-owner errors.
    #[test]
    fn denied_startup_never_calls_worker() -> TestResult {
        let status_error = NativeError::new(NativeOperation::Status, 7411);
        for settings in [
            Settings {
                argc: 2,
                ..Settings::default()
            },
            Settings {
                capture: Capture::Forbidden,
                ..Settings::default()
            },
            Settings {
                capture: Capture::Failed,
                ..Settings::default()
            },
            Settings {
                report_failures: vec![(2, status_error)],
                ..Settings::default()
            },
        ] {
            let mut rig = Rig::start(settings)?;
            let result = rig.settle(Ok(()));
            assert!(matches!(result, Some(Err(_))));
            for event in [
                Event::Prepare,
                Event::Activate,
                Event::Running,
                Event::Cancel,
                Event::Completion,
            ] {
                assert_eq!(rig.count(&event), 0);
            }
            assert!(!rig.statuses().iter().any(|raw| raw[1] == 4));
        }
        Ok(())
    }

    /// A Stop already published before startup admits no token capture or worker.
    ///
    /// # Panics
    /// Fails if an already published Stop admits capture, preparation or activation.
    ///
    /// # Errors
    /// Propagate checked fixture construction or missing retained-owner errors.
    #[test]
    fn stop_before_start_keeps_worker_dormant() -> TestResult {
        let mut rig = Rig::start(Settings {
            stopped_before_start: true,
            ..Settings::default()
        })?;
        assert_eq!(rig.settle(Ok(())), Some(Ok(())));
        assert_eq!(rig.count(&Event::Capture), 0);
        assert_eq!(rig.count(&Event::Prepare), 0);
        assert_eq!(rig.count(&Event::Activate), 0);
        assert_eq!(rig.count(&Event::Cancel), 0);
        assert!(!rig.statuses().iter().any(|raw| raw[1] == 4));
        Ok(())
    }

    /// Preparation Ready cannot bypass a new Stop, exact deadline or time reversal.
    ///
    /// # Panics
    /// Fails if Ready bypasses a new Stop, the exact deadline or a reversed clock.
    ///
    /// # Errors
    /// Propagate checked fixture construction or missing retained-owner errors.
    #[test]
    fn readiness_boundary_rechecks_stop_deadline_and_clock() -> TestResult {
        for boundary in 0..3 {
            let mut rig = Rig::start(Settings::default())?;
            let effect = match boundary {
                0 => PreparationEffect::Stop,
                1 => PreparationEffect::Time(rig.later(Duration::from_secs(10))?),
                _ => PreparationEffect::Time(rig.earlier()?),
            };
            {
                let mut script = rig.script.borrow_mut();
                script.preparation = Poll::Ready(Ok(()));
                script.preparation_effect = Some(effect);
            }
            let _observation = rig.poll()?;
            assert!(rig.owner()?.admission_closed());
            assert_eq!(rig.count(&Event::Activate), 0);
            assert!(!rig.statuses().iter().any(|raw| raw[1] == 4));
            if boundary == 0 {
                assert!(rig.failure().is_none());
            } else {
                assert!(rig.failure().is_some());
            }
            let result = rig.settle(Ok(()));
            assert!(result.is_some());
            assert_eq!(matches!(result, Some(Ok(()))), boundary == 0);
        }
        Ok(())
    }

    /// Running acknowledgement can itself race with Stop; successful status is not
    /// permission to activate after that newly published control.
    ///
    /// # Panics
    /// Fails if successful Running acknowledgement bypasses Stop before activation.
    ///
    /// # Errors
    /// Propagate checked fixture construction or missing retained-owner errors.
    #[test]
    fn stop_during_successful_running_acknowledgement_prevents_activation() -> TestResult {
        let mut rig = Rig::start(Settings::default())?;
        rig.script.borrow_mut().preparation = Poll::Ready(Ok(()));
        rig.trace.borrow_mut().stop_during_running_ack = true;
        let _observation = rig.poll()?;
        assert!(
            rig.events()
                .iter()
                .any(|event| { matches!(event, Event::StatusAcknowledged(raw) if raw[1] == 4) })
        );
        assert_eq!(rig.count(&Event::Activate), 0);
        assert_eq!(rig.count(&Event::Running), 0);
        assert!(rig.owner()?.admission_closed());
        assert_eq!(rig.count(&Event::Cancel), 1);
        assert_eq!(rig.settle(Ok(())), Some(Ok(())));
        Ok(())
    }

    /// Time must also be reobserved after a potentially fallible Running report.
    /// Both a reversal and the exact run deadline prevent operational activation.
    ///
    /// # Panics
    /// Fails if Running acknowledgement bypasses the fresh run deadline or clock guard.
    ///
    /// # Errors
    /// Propagate checked fixture construction or missing retained-owner errors.
    #[test]
    fn time_guard_during_running_acknowledgement_prevents_activation() -> TestResult {
        for reverse_clock in [false, true] {
            let mut rig = Rig::start(Settings::default())?;
            rig.script.borrow_mut().preparation = Poll::Ready(Ok(()));
            let now = if reverse_clock {
                rig.earlier()?
            } else {
                rig.later(Duration::from_secs(60))?
            };
            rig.trace.borrow_mut().time_during_running_ack = Some(now);
            let _observation = rig.poll()?;
            assert_eq!(rig.count(&Event::Activate), 0);
            assert!(rig.owner()?.admission_closed());
            assert_eq!(
                rig.failure(),
                Some(&if reverse_clock {
                    FixtureError::ClockReversed
                } else {
                    FixtureError::RunLimitElapsed
                })
            );
            assert!(matches!(rig.settle(Ok(())), Some(Err(_))));
        }
        Ok(())
    }

    /// A failed Running report is never an acknowledgement or an activation permit.
    ///
    /// # Panics
    /// Fails if a failed report acknowledges Running, activates or conceals its original cause.
    ///
    /// # Errors
    /// Propagate checked fixture construction or missing retained-owner errors.
    #[test]
    fn failed_running_report_cancels_prepared_worker_without_activation() -> TestResult {
        let error = NativeError::new(NativeOperation::Status, 7422);
        let mut rig = Rig::start(Settings {
            report_failures: vec![(4, error)],
            ..Settings::default()
        })?;
        rig.script.borrow_mut().preparation = Poll::Ready(Ok(()));
        let _observation = rig.poll()?;
        assert_eq!(rig.count(&Event::Activate), 0);
        assert_eq!(rig.count(&Event::Running), 0);
        assert_eq!(rig.count(&Event::Cancel), 1);
        assert!(rig.owner()?.admission_closed());
        assert_eq!(
            rig.failure(),
            Some(&FixtureError::Lifecycle(LifecycleError::Report(error)))
        );
        assert!(
            !rig.events()
                .iter()
                .any(|event| { matches!(event, Event::StatusAcknowledged(raw) if raw[1] == 4) })
        );
        assert!(!rig.statuses().iter().any(|raw| raw[1] == 1));
        assert!(matches!(rig.settle(Ok(())), Some(Err(_))));
        Ok(())
    }

    /// Worker failure in each reached phase starts cancellation, never completion.
    ///
    /// # Panics
    /// Fails if worker failure completes custody, allows further admission or reports Clean.
    ///
    /// # Errors
    /// Propagate checked fixture construction or missing retained-owner errors.
    #[test]
    fn worker_failures_retain_charge_until_explicit_finished() -> TestResult {
        for failure in [
            WorkerError::Preparation,
            WorkerError::Activation,
            WorkerError::Running,
        ] {
            let mut rig = Rig::start(Settings::default())?;
            {
                let mut script = rig.script.borrow_mut();
                match failure {
                    WorkerError::Preparation => script.preparation = Poll::Ready(Err(failure)),
                    WorkerError::Activation => {
                        script.preparation = Poll::Ready(Ok(()));
                        script.activation = Err(failure);
                    }
                    WorkerError::Running | WorkerError::Cleanup => {
                        script.preparation = Poll::Ready(Ok(()));
                        script.running = Err(failure);
                    }
                }
            }
            for _ in 0..2 {
                let _observation = rig.poll()?;
                if rig.owner()?.admission_closed() {
                    break;
                }
            }
            assert!(rig.owner()?.admission_closed());
            assert_eq!(rig.failure(), Some(&FixtureError::Worker(failure)));
            assert_eq!(rig.owner()?.outstanding_work(), 1);
            assert_eq!(rig.count(&Event::Cancel), 1);
            assert!(!rig.statuses().iter().any(|raw| raw[1] == 1));
            if failure == WorkerError::Preparation {
                assert_eq!(rig.count(&Event::Activate), 0);
                assert!(!rig.statuses().iter().any(|raw| raw[1] == 4));
            }
            assert!(matches!(rig.settle(Ok(())), Some(Err(_))));
            assert_eq!(rig.owner()?.outstanding_work(), 0);
            assert_eq!(rig.statuses().last(), Some(&[16, 1, 0, 1066, 1, 0, 0]));
        }
        Ok(())
    }

    /// Private admission closure survives a reversed clock before `StopPending` can
    /// change lifecycle state, and failed Stop reporting cannot reopen admission.
    ///
    /// # Panics
    /// Fails if Stop/time/report failure reopens admission or releases charged work.
    ///
    /// # Errors
    /// Propagate checked fixture construction or missing retained-owner errors.
    #[test]
    fn stop_fences_admission_before_time_or_status_failure() -> TestResult {
        for reverse_clock in [false, true] {
            let error = NativeError::new(NativeOperation::Status, 7433);
            let mut rig = Rig::start(Settings {
                report_failures: if reverse_clock {
                    Vec::new()
                } else {
                    vec![(3, error)]
                },
                ..Settings::default()
            })?;
            assert!(matches!(rig.poll()?, InvocationPoll::Pending(_)));
            if reverse_clock {
                *rig.clock.time.borrow_mut() = rig.earlier()?;
            }
            let _control = rig.notification.handle_control(1);
            let _observation = rig.poll()?;
            assert!(rig.owner()?.admission_closed());
            assert_eq!(rig.owner()?.outstanding_work(), 1);
            assert_eq!(rig.count(&Event::Cancel), 1);
            assert_eq!(rig.count(&Event::Activate), 0);
            assert!(!rig.statuses().iter().any(|raw| raw[1] == 1));
            let prepare_calls = rig.count(&Event::Prepare);
            let _observation = rig.poll()?;
            assert_eq!(rig.count(&Event::Prepare), prepare_calls);
            assert_eq!(rig.count(&Event::Cancel), 1);
            if reverse_clock {
                assert_eq!(rig.failure(), Some(&FixtureError::ClockReversed));
            } else {
                assert_eq!(
                    rig.failure(),
                    Some(&FixtureError::Lifecycle(LifecycleError::Report(error)))
                );
            }
            assert!(matches!(rig.settle(Ok(())), Some(Err(_))));
        }
        Ok(())
    }

    /// One primary worker failure survives bounded independent cleanup/report faults.
    ///
    /// # Panics
    /// Fails if cleanup replaces the primary cause, loses a secondary fact or repeats reporting.
    ///
    /// # Errors
    /// Propagate checked fixture construction or missing retained-owner errors.
    #[test]
    fn cleanup_and_reporting_failures_preserve_primary_and_all_bounded_facts() -> TestResult {
        let stop_error = NativeError::new(NativeOperation::Status, 7443);
        let terminal_error = NativeError::new(NativeOperation::Status, 7441);
        let mut rig = Rig::start(Settings {
            report_failures: vec![(3, stop_error), (1, terminal_error)],
            ..Settings::default()
        })?;
        rig.script.borrow_mut().preparation = Poll::Ready(Err(WorkerError::Preparation));
        let _observation = rig.poll()?;
        assert_eq!(
            rig.failure(),
            Some(&FixtureError::Worker(WorkerError::Preparation))
        );
        assert_eq!(rig.owner()?.outstanding_work(), 1);
        let result = rig.settle(Err(WorkerError::Cleanup));
        let Some(Err(FixtureError::Cleanup(failure))) = result else {
            return Err(
                "cleanup/report failures were not retained as bounded secondary facts".into(),
            );
        };
        assert_eq!(
            failure.primary(),
            &FixtureError::Worker(WorkerError::Preparation)
        );
        assert_eq!(failure.completion(), Some(WorkerError::Cleanup));
        assert_eq!(
            failure.stop_report(),
            Some(&LifecycleError::Report(stop_error))
        );
        assert_eq!(
            failure.terminal_report(),
            Some(&LifecycleError::Report(terminal_error))
        );
        assert_eq!(rig.owner()?.outstanding_work(), 0);
        assert_eq!(rig.statuses().iter().filter(|raw| raw[1] == 3).count(), 1);
        assert_eq!(rig.statuses().iter().filter(|raw| raw[1] == 1).count(), 1);
        assert!(matches!(rig.poll()?, InvocationPoll::Finished));
        assert_eq!(rig.statuses().iter().filter(|raw| raw[1] == 1).count(), 1);
        Ok(())
    }

    /// A cleanup failure after an ordinary Stop remains a failed terminal outcome.
    ///
    /// # Panics
    /// Fails if unresolved cleanup releases work or failed completion produces a clean result.
    ///
    /// # Errors
    /// Propagate checked fixture construction or missing retained-owner errors.
    #[test]
    fn explicit_finished_error_is_completion_but_never_clean() -> TestResult {
        let mut rig = Rig::start(Settings::default())?;
        rig.script.borrow_mut().preparation = Poll::Ready(Ok(()));
        let _observation = rig.poll()?;
        let _control = rig.notification.handle_control(1);
        let _observation = rig.poll()?;
        assert_eq!(rig.owner()?.outstanding_work(), 1);
        assert!(!rig.statuses().iter().any(|raw| raw[1] == 1));
        let result = rig.settle(Err(WorkerError::Cleanup));
        assert!(matches!(result, Some(Err(_))));
        assert_eq!(rig.owner()?.outstanding_work(), 0);
        assert_eq!(rig.statuses().last(), Some(&[16, 1, 0, 1066, 1, 0, 0]));
        Ok(())
    }

    /// An expired startup allowance remains the cause while the same invocation
    /// receives one separately bounded shutdown allowance. No worker wake is needed
    /// for a finite manual step to observe the exact shutdown deadline.
    ///
    /// # Panics
    /// Fails if startup expiry renews shutdown, releases work or fabricates terminal status.
    ///
    /// # Errors
    /// Propagate checked fixture construction or missing retained-owner errors.
    #[test]
    fn startup_expiry_starts_one_shutdown_allowance_then_retains_overdue_work() -> TestResult {
        let mut rig = Rig::start(Settings::default())?;
        assert!(matches!(rig.poll()?, InvocationPoll::Pending(_)));
        rig.advance(Duration::from_secs(10))?;
        assert!(matches!(
            rig.poll()?,
            InvocationPoll::Pending(duration) if duration == Duration::from_secs(10)
        ));
        assert_eq!(
            rig.failure(),
            Some(&FixtureError::Lifecycle(LifecycleError::DeadlineElapsed))
        );
        assert!(rig.owner()?.admission_closed());
        assert_eq!(rig.owner()?.outstanding_work(), 1);
        assert_eq!(rig.count(&Event::Cancel), 1);
        rig.advance(Duration::from_secs(9))?;
        assert!(matches!(
            rig.poll()?,
            InvocationPoll::Pending(duration) if duration == Duration::from_secs(1)
        ));
        let reports_before_expiry = rig.statuses();
        rig.advance(Duration::from_secs(1))?;
        assert!(matches!(rig.poll()?, InvocationPoll::Overdue));
        assert_eq!(rig.owner()?.outstanding_work(), 1);
        assert_eq!(rig.statuses(), reports_before_expiry);
        assert!(!rig.statuses().iter().any(|raw| raw[1] == 1));
        assert!(matches!(rig.settle(Ok(())), Some(Err(_))));
        assert_eq!(rig.owner()?.outstanding_work(), 0);
        assert_eq!(rig.statuses().last(), Some(&[16, 1, 0, 1066, 1, 0, 0]));
        Ok(())
    }

    /// Exact fixed pending deadlines and repeated Stop cannot release or replace work.
    ///
    /// # Panics
    /// Fails if overdue or repeated Stop destroys custody, renews a deadline or reports Clean.
    ///
    /// # Errors
    /// Propagate checked fixture construction or missing retained-owner errors.
    #[test]
    fn overdue_retains_same_owner_reporter_and_charge_until_late_completion() -> TestResult {
        let mut rig = Rig::start(Settings::default())?;
        assert!(matches!(rig.poll()?, InvocationPoll::Pending(_)));
        let _control = rig.notification.handle_control(1);
        let first_stop = rig.poll()?;
        assert!(matches!(first_stop, InvocationPoll::Pending(duration) if !duration.is_zero()));
        rig.advance(Duration::from_secs(10))?;
        assert!(matches!(rig.poll()?, InvocationPoll::Overdue));
        let reports = rig.statuses();
        for _ in 0..3 {
            let _control = rig.notification.handle_control(1);
            assert!(matches!(rig.poll()?, InvocationPoll::Overdue));
        }
        assert_eq!(rig.statuses(), reports);
        assert!(rig.owner()?.admission_closed());
        assert_eq!(rig.owner()?.outstanding_work(), 1);
        assert_eq!(rig.count(&Event::Prepare), 1);
        assert_eq!(rig.count(&Event::Cancel), 1);
        assert_eq!(rig.count(&Event::WorkerDropped), 0);
        assert_eq!(rig.count(&Event::ReporterDropped), 0);
        assert!(!rig.statuses().iter().any(|raw| raw[1] == 1));
        assert!(rig.take_result().is_none());
        assert!(matches!(rig.settle(Ok(())), Some(Err(_))));
        assert_eq!(rig.owner()?.outstanding_work(), 0);
        assert_eq!(rig.statuses().last(), Some(&[16, 1, 0, 1066, 1, 0, 0]));
        Ok(())
    }

    /// Stop deadline overflow fences a live job even when the portable lifecycle
    /// cannot enter its next phase. A representable neighboring deadline still
    /// reports `StopPending`; neither case releases work before explicit completion.
    ///
    /// # Panics
    /// Fails if an unrepresentable Stop deadline admits work, loses its original
    /// failure, drops custody or manufactures an acknowledged terminal status.
    ///
    /// # Errors
    /// Propagate checked fixture construction or an unsupported Instant boundary.
    #[test]
    fn stop_deadline_overflow_retains_live_work_and_its_original_failure() -> TestResult {
        let mut ceiling = Instant::now();
        // The existing representability fixture searches whole seconds this way.
        // These 64 checked additions read no service clock and perform no waits.
        for bit in (0..u64::BITS).rev() {
            let seconds = 1_u64.checked_shl(bit).ok_or("fixture shift exceeded 63")?;
            if let Some(later) = ceiling.checked_add(Duration::from_secs(seconds)) {
                ceiling = later;
            }
        }
        if ceiling.checked_add(Duration::from_secs(1)).is_some() {
            return Err("fixture search did not reach a whole-second ceiling".into());
        }
        let neighbor = ceiling
            .checked_sub(Duration::from_secs(30))
            .ok_or("fixture ceiling has no thirty-second predecessor")?;
        assert!(ceiling.checked_add(Duration::from_secs(10)).is_none());
        assert!(neighbor.checked_add(Duration::from_secs(10)).is_some());
        for overflow in [false, true] {
            let mut rig = Rig::start(Settings::default())?;
            let _observation = rig.poll()?;
            assert_eq!(rig.owner()?.outstanding_work(), 1);
            *rig.clock.time.borrow_mut() = if overflow { ceiling } else { neighbor };
            let _control = rig.notification.handle_control(1);
            let observation = rig.poll()?;
            if overflow {
                assert_eq!(observation, InvocationPoll::Overdue);
                assert_eq!(rig.owner()?.state(), Some(ServiceState::StartPending));
            } else {
                assert_eq!(
                    observation,
                    InvocationPoll::Pending(Duration::from_secs(10))
                );
                assert_eq!(rig.owner()?.state(), Some(ServiceState::StopPending));
            }
            assert!(rig.owner()?.admission_closed());
            assert_eq!(rig.owner()?.outstanding_work(), 1);
            assert_eq!(rig.count(&Event::Prepare), 1);
            assert_eq!(rig.count(&Event::Cancel), 1);
            assert_eq!(rig.count(&Event::Activate), 0);
            assert!(!rig.statuses().iter().any(|raw| raw[1] == 1));
            assert_eq!(
                rig.failure(),
                Some(&FixtureError::Lifecycle(LifecycleError::DeadlineElapsed))
            );
            let result = rig.settle(Ok(()));
            assert_eq!(rig.owner()?.outstanding_work(), 0);
            if overflow {
                let Some(Err(FixtureError::Cleanup(facts))) = result else {
                    return Err("overflow lost bounded stop failure facts".into());
                };
                assert_eq!(
                    facts.primary(),
                    &FixtureError::Lifecycle(LifecycleError::DeadlineElapsed)
                );
                assert_eq!(facts.stop_report(), Some(&LifecycleError::DeadlineOverflow));
                assert_eq!(
                    facts.terminal_report(),
                    Some(&LifecycleError::InvalidTransition)
                );
                assert!(facts.completion().is_none());
                assert!(!rig.statuses().iter().any(|raw| matches!(raw[1], 3 | 1)));
            } else {
                assert_eq!(
                    result,
                    Some(Err(FixtureError::Lifecycle(
                        LifecycleError::DeadlineElapsed
                    )))
                );
                assert_eq!(rig.statuses().last(), Some(&[16, 1, 0, 1066, 1, 0, 0]));
            }
        }
        Ok(())
    }

    /// The running allowance is absolute, and a neighboring pre-deadline Stop is clean.
    ///
    /// # Panics
    /// Fails if the absolute run boundary or its timely Stop control is misclassified.
    ///
    /// # Errors
    /// Propagate checked fixture construction or missing retained-owner errors.
    #[test]
    fn exact_running_limit_fails_while_timely_stop_succeeds() -> TestResult {
        for timely_stop in [false, true] {
            let mut rig = Rig::start(Settings::default())?;
            rig.script.borrow_mut().preparation = Poll::Ready(Ok(()));
            let _observation = rig.poll()?;
            rig.advance(Duration::from_secs(if timely_stop { 59 } else { 60 }))?;
            if timely_stop {
                let _control = rig.notification.handle_control(1);
            }
            let _observation = rig.poll()?;
            assert!(rig.owner()?.admission_closed());
            assert_eq!(rig.owner()?.outstanding_work(), 1);
            if timely_stop {
                assert!(rig.failure().is_none());
            } else {
                assert_eq!(rig.failure(), Some(&FixtureError::RunLimitElapsed));
            }
            let result = rig.settle(Ok(()));
            assert_eq!(matches!(result, Some(Ok(()))), timely_stop);
        }
        Ok(())
    }

    /// Completion belongs to its original owner even while another invocation owns
    /// independently charged work. It cannot settle or free that successor's charge.
    ///
    /// # Panics
    /// Fails if old completion changes successor custody, admission or status.
    ///
    /// # Errors
    /// Propagate checked fixture construction or missing retained-owner errors.
    #[test]
    fn late_completion_cannot_release_successor_invocation_work() -> TestResult {
        let mut old = Rig::start(Settings::default())?;
        let _observation = old.poll()?;
        let _control = old.notification.handle_control(1);
        let _observation = old.poll()?;
        old.advance(Duration::from_secs(10))?;
        assert!(matches!(old.poll()?, InvocationPoll::Overdue));

        let mut successor = Rig::start(Settings::default())?;
        successor.script.borrow_mut().preparation = Poll::Ready(Ok(()));
        let _observation = successor.poll()?;
        assert_eq!(successor.owner()?.outstanding_work(), 1);
        let successor_reports = successor.statuses();
        assert!(matches!(old.settle(Ok(())), Some(Err(_))));
        assert_eq!(old.owner()?.outstanding_work(), 0);
        assert_eq!(successor.owner()?.outstanding_work(), 1);
        assert_eq!(successor.owner()?.state(), Some(ServiceState::Running));
        assert_eq!(successor.statuses(), successor_reports);
        assert!(!successor.owner()?.admission_closed());
        assert_eq!(successor.settle(Ok(())), Some(Ok(())));
        Ok(())
    }

    /// Discarding a finite observation never consumes the retained owner or its job.
    ///
    /// # Panics
    /// Fails if discarding a poll consumes its owner, completes work or exposes a final result.
    ///
    /// # Errors
    /// Propagate checked fixture construction or missing retained-owner errors.
    #[test]
    fn dropping_poll_observation_keeps_unresolved_owner_available() -> TestResult {
        let mut rig = Rig::start(Settings::default())?;
        {
            let _observation = rig.poll()?;
        }
        assert_eq!(rig.owner()?.outstanding_work(), 1);
        assert_eq!(rig.count(&Event::WorkerDropped), 0);
        assert_eq!(rig.count(&Event::ReporterDropped), 0);
        assert!(matches!(rig.poll()?, InvocationPoll::Pending(_)));
        assert_eq!(rig.owner()?.outstanding_work(), 1);
        assert!(rig.take_result().is_none());
        assert!(!rig.statuses().iter().any(|raw| raw[1] == 1));
        assert_eq!(rig.settle(Ok(())), Some(Ok(())));
        assert!(rig.take_result().is_none());
        Ok(())
    }

    /// Exercise the retained standard-library wake on a fresh host-owner thread.
    /// A bounded channel hands the actual wake to the publisher; its atomic release
    /// is observed before parking, after notification. Benign SCM noise remains a
    /// useful control. This source control makes no native or scheduler latency claim.
    ///
    /// # Panics
    /// Fails if the controlled retained wake is lost or benign controls manufacture Stop.
    ///
    /// # Errors
    /// Propagate checked fixture, bounded channel or owner thread errors.
    #[test]
    fn worker_wake_before_park_is_retained_and_rechecks_real_stop() -> TestResult {
        use std::sync::{
            Arc,
            atomic::{AtomicBool, Ordering},
            mpsc,
        };

        let (wake_sender, wake_receiver) = mpsc::sync_channel(1);
        let published = Arc::new(AtomicBool::new(false));
        let owner_published = Arc::clone(&published);
        let owner_thread = std::thread::spawn(move || {
            let result = (|| -> TestResult {
                let mut rig = Rig::start(Settings::default())?;
                let _observation = rig.poll()?;
                let wake = rig
                    .script
                    .borrow()
                    .wake
                    .clone()
                    .ok_or("worker did not receive the invocation wake")?;
                assert_eq!(rig.notification.handle_control(4), ScmControl::Interrogate);
                assert_eq!(
                    rig.notification.handle_control(u32::MAX),
                    ScmControl::Unsupported
                );
                assert!(!rig.notification.stop_requested());
                // Consume any token predating this fresh owner's controlled publish.
                std::thread::park_timeout(Duration::ZERO);
                // No wake is handed out yet. A quiet neighboring wait may return
                // spuriously; either return must preserve actual pending state.
                std::thread::park_timeout(Duration::from_millis(30));
                assert!(matches!(rig.poll()?, InvocationPoll::Pending(_)));
                assert_eq!(rig.owner()?.outstanding_work(), 1);
                assert_eq!(rig.count(&Event::Activate), 0);
                assert!(!rig.notification.stop_requested());
                std::thread::park_timeout(Duration::ZERO);
                wake_sender.send(wake.clone())?;
                let handshake_started = Instant::now();
                while !owner_published.load(Ordering::Acquire) {
                    if handshake_started.elapsed() >= Duration::from_secs(5) {
                        return Err("wake publisher did not release the bounded handshake".into());
                    }
                    // A channel receive here could consume the unpark token itself.
                    std::thread::yield_now();
                }
                let before_park = Instant::now();
                std::thread::park_timeout(Duration::from_secs(2));
                assert!(before_park.elapsed() < Duration::from_secs(1));
                assert!(matches!(rig.poll()?, InvocationPoll::Pending(_)));
                assert_eq!(rig.owner()?.state(), Some(ServiceState::StartPending));
                assert_eq!(rig.count(&Event::Activate), 0);
                assert!(!rig.notification.stop_requested());

                let _control = rig.notification.handle_control(1);
                wake.notify();
                let _observation = rig.poll()?;
                assert!(rig.owner()?.admission_closed());
                assert_eq!(rig.count(&Event::Cancel), 1);
                assert_eq!(rig.owner()?.outstanding_work(), 1);
                assert_eq!(rig.settle(Ok(())), Some(Ok(())));
                Ok(())
            })();
            result.map_err(|error| error.to_string())
        });

        let publication = wake_receiver.recv_timeout(Duration::from_secs(5));
        if let Ok(wake) = &publication {
            wake.notify();
        }
        published.store(true, Ordering::Release);
        let owner_result = owner_thread
            .join()
            .map_err(|_panic| "retained-wake owner thread panicked")?;
        publication?;
        owner_result.map_err(Into::into)
    }

    /// Test-only child selection, absent from native executable composition.
    #[cfg(target_os = "linux")]
    const OWNER_DROP_CHILD_MODE: &str = "NEVROTICK_S31_OWNER_DROP_CHILD_MODE";

    /// Exercise the actual fatal custody guard in an externally bounded subprocess.
    /// Healthy destruction verifies that both destructor observers are reachable.
    /// The configured runner must bound child execution and disable core dumps.
    ///
    /// # Panics
    /// Fails if unresolved destruction returns, aborts incorrectly or runs owned destructors.
    ///
    /// # Errors
    /// Propagate checked fixture, process launch, child mode or output decoding errors.
    #[cfg(target_os = "linux")]
    #[test]
    fn unresolved_owner_drop_aborts_before_worker_or_reporter_destruction() -> TestResult {
        use std::{os::unix::process::ExitStatusExt, process::Command};

        match std::env::var(OWNER_DROP_CHILD_MODE) {
            Ok(mode) => {
                let mut rig = Rig::start(Settings::default())?;
                rig.trace.borrow_mut().destructor_markers = true;
                let _observation = rig.poll()?;
                match mode.as_str() {
                    "healthy" => {
                        assert_eq!(rig.settle(Ok(())), Some(Ok(())));
                        let invocation = rig.invocation.take().ok_or("missing healthy owner")?;
                        drop(invocation);
                    }
                    "unresolved" => {
                        let invocation = rig.invocation.take().ok_or("missing unresolved owner")?;
                        eprintln!("fixture-worker-unresolved-drop-entered");
                        drop(invocation);
                    }
                    _ => return Err("unknown worker-custody child mode".into()),
                }
                eprintln!("fixture-worker-owner-drop-returned");
                return Ok(());
            }
            Err(std::env::VarError::NotPresent) => {}
            Err(error) => return Err(Box::new(error)),
        }

        for mode in ["healthy", "unresolved"] {
            let output = Command::new(std::env::current_exe()?)
                .args([
                    "--exact",
                    "host::tests::worker_tests::unresolved_owner_drop_aborts_before_worker_or_reporter_destruction",
                    "--nocapture",
                    "--test-threads=1",
                ])
                .env(OWNER_DROP_CHILD_MODE, mode)
                .env("RUST_BACKTRACE", "0")
                .output()?;
            let stderr = std::str::from_utf8(&output.stderr)?;
            if mode == "healthy" {
                assert!(output.status.success());
                assert!(stderr.contains("fixture-worker-worker-dropped"));
                assert!(stderr.contains("fixture-worker-reporter-dropped"));
                assert!(stderr.contains("fixture-worker-owner-drop-returned"));
                assert!(!stderr.contains("fixture-worker-unresolved-drop-entered"));
            } else {
                assert_eq!(output.status.signal(), Some(6));
                assert!(stderr.contains("fixture-worker-unresolved-drop-entered"));
                assert!(!stderr.contains("fixture-worker-worker-dropped"));
                assert!(!stderr.contains("fixture-worker-reporter-dropped"));
                assert!(!stderr.contains("fixture-worker-owner-drop-returned"));
            }
        }
        Ok(())
    }
}

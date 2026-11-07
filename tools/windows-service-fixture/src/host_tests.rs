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
    /// Fails if a delivered Stop is not classified as Stop.
    fn park(&mut self, timeout: Duration) {
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
    ])
}

/// Construct only a complete successful zero-entry capture for healthy old traces.
///
/// # Errors
/// Propagate checked construction; failed/partial API output is never simulated as success.
fn empty_snapshot() -> Result<PrivilegeSnapshot, TokenObservationFailure> {
    fixture_privilege_ids()
        .and_then(|ids| PrivilegeSnapshot::decode(&[0; 4], ids))
        .map_err(|error| {
            TokenObservationFailure::new(
                TokenObservationCause::Capture(error),
                TokenCleanupOutcome::NotOpened,
                TokenCleanupOutcome::NotOpened,
            )
        })
}

/// Explicit healthy observer provided to every historical actual-host trace.
struct HealthyTokenObserver;
impl TokenObserver for HealthyTokenObserver {
    /// Supply a complete empty capture; no optional observer or bypass is selected.
    fn observe(&mut self) -> Result<PrivilegeSnapshot, TokenObservationFailure> {
        empty_snapshot()
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
        &mut HealthyTokenObserver,
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
        Err(FixtureError::Lifecycle(windows_platform::service::LifecycleError::Report(error)))
            if error == first
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
        &mut HealthyTokenObserver,
        clock,
        &mut wait,
        notification,
    )
}

/// Reversal during readiness, work admission, initial/wake loop observations or
/// either cleanup read is final. No separate loop-origin read renews the interval.
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
        ("readiness", 3, false, false),
        ("work admission", 4, true, false),
        ("initial loop observation", 5, true, false),
        ("wake observation", 6, true, true),
        ("stop bookkeeping", 7, true, true),
        ("terminal bookkeeping", 8, true, true),
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
        observations: vec![base; 8],
        reads: 0,
    };
    execute_scripted_clock(&mut clock, &observation, &notification)?;
    assert_eq!(clock.reads, 8);
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
                    ready,
                    earlier_fixture_observation(ready, Duration::from_nanos(1)),
                ]
            } else {
                vec![startup, startup, ready, ready, ready, ready, ready, ready]
            },
            reads: 0,
        };
        let result = execute_scripted_clock(&mut clock, &observation, &notification);
        let (win32_code, service_code) = if reversed {
            assert!(matches!(result, Err(FixtureError::ClockReversed)));
            assert_eq!(clock.reads, 4);
            assert_eq!(waits(&observation), []);
            (1066, 1)
        } else {
            result?;
            assert_eq!(clock.reads, 8);
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
/// Expired pre-work observations require six reads, proving no loop entry read.
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
    for delayed_read in [4, 5] {
        for elapsed_ms in [59_999, 60_000, 60_001] {
            let later = ready
                .checked_add(Duration::from_millis(elapsed_ms))
                .ok_or(FixtureError::ClockOverflow)?;
            let begin_work = if delayed_read == 4 { later } else { ready };
            let mut clock = ScriptedClock {
                observations: vec![
                    startup, startup, ready, begin_work, later, later, later, later, later,
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
                8
            } else if delayed_read == 4 {
                6
            } else {
                7
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
                    startup, startup, ready, ready, ready, ready, stopping, finished,
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
                8,
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
    result: Result<PrivilegeSnapshot, TokenObservationFailure>,
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
        result,
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
    low: u32,
    attributes: u32,
) -> Result<PrivilegeSnapshot, windows_platform::privilege_observation::PrivilegeCaptureError> {
    let mut raw = 1_u32.to_le_bytes().to_vec();
    raw.extend(low.to_le_bytes());
    raw.extend(0_i32.to_le_bytes());
    raw.extend(attributes.to_le_bytes());
    PrivilegeSnapshot::decode(&raw, fixture_privilege_ids()?)
}

/// Both real callers consume the snapshot: engine absence is mandatory; broker is descriptive.
///
/// # Errors
/// Propagate fixture construction failure and unexpected healthy-host failure.
/// # Panics
/// Fails if engine presence (even disabled/default-only) reaches Running, the
/// broker gains an engine-only refusal, or either role skips/repeats capture.
#[test]
fn real_host_engine_requires_absence_while_broker_observation_is_descriptive()
-> Result<(), Box<dyn Error>> {
    for (low, privilege) in [
        (17, ServicePrivilege::SystemTime),
        (23, ServicePrivilege::Debug),
        (41, ServicePrivilege::Impersonate),
    ] {
        for attributes in [0, 1, 2, 0x8000_0001] {
            for role in [LabRole::Engine, LabRole::ClockBroker] {
                let observation = Rc::new(RefCell::new(Observations::default()));
                let notification = StopNotification::new();
                let (result, calls) = execute_observation(
                    role,
                    0,
                    &notification,
                    Ok(present_snapshot(low, attributes)?),
                    CaptureAction::None,
                    &observation,
                );
                assert_eq!(calls, 1);
                if role == LabRole::Engine {
                    let Err(FixtureError::PrivilegeAdmission(error)) = result else {
                        return Err("engine admitted a present forbidden privilege".into());
                    };
                    assert_eq!(error.privilege(), privilege);
                    assert_eq!(
                        error.state(),
                        if attributes & 2 == 0 {
                            windows_platform::privilege_observation::PrivilegeState::PresentDisabled
                        } else {
                            windows_platform::privilege_observation::PrivilegeState::PresentEnabled
                        }
                    );
                    assert!(!statuses(&observation).iter().any(|status| status[1] == 4));
                    assert_eq!(waits(&observation), []);
                    assert_eq!(
                        statuses(&observation).last(),
                        Some(&[16, 1, 0, 1066, 1, 0, 0])
                    );
                } else {
                    result?;
                    assert!(statuses(&observation).iter().any(|status| status[1] == 4));
                    assert_eq!(waits(&observation), [Duration::from_secs(60)]);
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
            empty_snapshot(),
            CaptureAction::None,
            &observation,
        );
        result?;
        assert_eq!(calls, 1);
        assert_eq!(waits(&observation), [Duration::from_secs(60)]);
    }
    Ok(())
}

/// Unknown API/context/capture/cleanup facts refuse both real callers without retry.
///
/// # Errors
/// Propagate independent identity construction failure.
/// # Panics
/// Fails if first-cause or separate cleanup outcomes change, or any unknown reaches Running.
#[test]
fn real_host_refuses_unknown_context_and_retains_both_cleanup_failures()
-> Result<(), Box<dyn Error>> {
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
    for raw in [&[0; 3][..], &[1, 0, 0, 0][..]] {
        let error = PrivilegeSnapshot::decode(raw, fixture_privilege_ids()?)
            .err()
            .ok_or("malformed fixture capture admitted")?;
        failures.push(TokenObservationFailure::new(
            TokenObservationCause::Capture(error),
            TokenCleanupOutcome::Closed,
            TokenCleanupOutcome::NotOpened,
        ));
    }
    for failure in failures {
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
                empty_snapshot(),
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
                empty_snapshot(),
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
            let mut script = vec![startup, startup];
            script.extend(std::iter::repeat_n(ready, if failed { 3 } else { 6 }));
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
                &mut HealthyTokenObserver,
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
                if failed { 5 } else { 8 },
            ));
        }
    }
    assert_eq!(actual, expected);
    Ok(())
}

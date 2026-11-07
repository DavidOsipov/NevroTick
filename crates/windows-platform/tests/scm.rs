// SPDX-License-Identifier: GPL-2.0-only
//! Public-consumer facts for portable SCM projection and bounded notification.

use std::{
    cell::RefCell,
    convert::Infallible,
    error::Error,
    num::NonZeroU32,
    rc::Rc,
    sync::{Arc, Barrier},
    time::Instant,
};

use windows_platform::{
    scm::{ScmControl, ScmStatus, StopNotification},
    service::{
        PendingBudget, ServiceLifecycle, ServiceRole, ServiceStatus, StatusReporter, StopOutcome,
    },
};

/// Record the numeric output of every real lifecycle status-effect request.
struct Reporter(Rc<RefCell<Vec<ScmStatus>>>);

impl StatusReporter for Reporter {
    type Error = Infallible;

    /// Project only the immutable status received from the actual lifecycle owner.
    fn report(&mut self, status: ServiceStatus) -> Result<(), Infallible> {
        self.0.borrow_mut().push(ScmStatus::from_status(status));
        Ok(())
    }
}

/// Read every numeric field in the documented `SERVICE_STATUS` order.
fn fields(status: ScmStatus) -> [u32; 7] {
    [
        status.service_type(),
        status.current_state(),
        status.controls_accepted(),
        status.win32_exit_code(),
        status.service_specific_exit_code(),
        status.checkpoint(),
        status.wait_hint_ms(),
    ]
}

/// Compare all seven fields with independent Windows constants for both roles.
///
/// # Panics
/// Fails if any state, control, exit, progress or service-type field differs.
///
/// # Errors
/// Propagate unexpected checked-budget or lifecycle failures.
#[test]
fn every_lifecycle_phase_has_the_exact_seven_field_projection() -> Result<(), Box<dyn Error>> {
    for role in [ServiceRole::Engine, ServiceRole::ClockBroker] {
        let log = Rc::new(RefCell::new(Vec::new()));
        let mut service = ServiceLifecycle::new(
            role,
            Reporter(Rc::clone(&log)),
            PendingBudget::new(17, 200)?,
            PendingBudget::new(29, 300)?,
            NonZeroU32::MIN,
        );
        let now = Instant::now();
        service.start(now)?;
        service.progress(now)?;
        service.ready(now)?;
        service.request_stop(now)?;
        service.progress(now)?;
        service.finish_stop(now, StopOutcome::Clean)?;
        let actual: Vec<_> = log.borrow().iter().copied().map(fields).collect();
        assert_eq!(
            actual,
            [
                [0x10, 2, 0, 0, 0, 1, 17],
                [0x10, 2, 0, 0, 0, 2, 17],
                [0x10, 4, 1, 0, 0, 0, 0],
                [0x10, 3, 0, 0, 0, 1, 29],
                [0x10, 3, 0, 0, 0, 2, 29],
                [0x10, 1, 0, 0, 0, 0, 0],
            ]
        );
        assert!(actual.iter().all(|raw| raw[0] & 0x200 == 0));
    }
    Ok(())
}

/// Preserve every nonzero terminal-code boundary without mixing Win32 namespaces.
///
/// # Panics
/// Fails if a failed terminal status loses or truncates its service-specific code.
///
/// # Errors
/// Propagate unexpected checked-budget or lifecycle failures.
#[test]
fn failed_cleanup_maps_the_exact_service_specific_error_code() -> Result<(), Box<dyn Error>> {
    for raw_code in [1, 1066, 0x200, u32::MAX] {
        let Some(code) = NonZeroU32::new(raw_code) else {
            return Err("nonzero fixture code was rejected".into());
        };
        let log = Rc::new(RefCell::new(Vec::new()));
        let budget = PendingBudget::new(1, u32::MAX)?;
        let mut service = ServiceLifecycle::new(
            ServiceRole::ClockBroker,
            Reporter(Rc::clone(&log)),
            budget,
            budget,
            NonZeroU32::MIN,
        );
        let now = Instant::now();
        service.start(now)?;
        service.ready(now)?;
        service.request_stop(now)?;
        service.finish_stop(now, StopOutcome::Failed(code))?;
        let actual: Vec<_> = log.borrow().iter().copied().map(fields).collect();
        assert_eq!(
            actual,
            [
                [0x10, 2, 0, 0, 0, 1, 1],
                [0x10, 4, 1, 0, 0, 0, 0],
                [0x10, 3, 0, 0, 0, 1, 1],
                [0x10, 1, 0, 1066, raw_code, 0, 0],
            ]
        );
    }
    Ok(())
}

/// A legitimate early stop has no Running report and retains maximal wait hints.
///
/// # Panics
/// Fails if projection fabricates Running or changes a checked maximal wait hint.
///
/// # Errors
/// Propagate unexpected checked-budget or lifecycle failures.
#[test]
fn early_stop_preserves_pending_limits_without_fabricating_running() -> Result<(), Box<dyn Error>> {
    let log = Rc::new(RefCell::new(Vec::new()));
    let budget = PendingBudget::new(u32::MAX, u32::MAX)?;
    let mut service = ServiceLifecycle::new(
        ServiceRole::Engine,
        Reporter(Rc::clone(&log)),
        budget,
        budget,
        NonZeroU32::MIN,
    );
    let now = Instant::now();
    service.start(now)?;
    service.request_stop(now)?;
    service.finish_stop(now, StopOutcome::Clean)?;
    let actual: Vec<_> = log.borrow().iter().copied().map(fields).collect();
    assert_eq!(
        actual,
        [
            [0x10, 2, 0, 0, 0, 1, u32::MAX],
            [0x10, 3, 0, 0, 0, 1, u32::MAX],
            [0x10, 1, 0, 0, 0, 0, 0],
        ]
    );
    Ok(())
}

/// Check the complete small raw-control space plus large and boundary values.
///
/// # Panics
/// Fails if any unsupported control is accepted or has a successful return code.
#[test]
fn controls_decode_only_stop_and_interrogate() {
    for raw in (0..=256).chain([u32::MAX - 1, u32::MAX]) {
        let (expected, code) = match raw {
            1 => (ScmControl::Stop, 0),
            4 => (ScmControl::Interrogate, 0),
            _ => (ScmControl::Unsupported, 120),
        };
        let decoded = ScmControl::decode(raw);
        assert_eq!(decoded, expected);
        assert_eq!(decoded.return_code(), code);
    }
}

/// Interrogate and unsupported controls neither set nor clear the sticky stop bit.
///
/// # Panics
/// Fails if decoding any ordinary control mutates unrelated notification state.
#[test]
fn notification_is_coalesced_and_unrelated_controls_have_no_effect() {
    let notification = StopNotification::new();
    for raw in (0..=256).chain([u32::MAX]) {
        if raw != 1 {
            assert_eq!(notification.handle_control(raw), ScmControl::decode(raw));
            assert!(!notification.stop_requested());
        }
    }
    for _ in 0..1000 {
        assert_eq!(notification.handle_control(1), ScmControl::Stop);
        assert!(notification.stop_requested());
    }
    for raw in (0..=256).chain([u32::MAX]) {
        assert_eq!(notification.handle_control(raw), ScmControl::decode(raw));
        assert!(notification.stop_requested());
    }
    assert!(!StopNotification::default().stop_requested());
    assert!(!StopNotification::new().stop_requested());
}

/// A portable notification cannot mutate the actual lifecycle or call its reporter.
///
/// # Panics
/// Fails if notification changes any observed lifecycle state or reporter output.
///
/// # Errors
/// Propagate unexpected checked-budget or lifecycle failures.
#[test]
fn receiving_a_stop_is_separate_from_the_owners_authorized_transition() -> Result<(), Box<dyn Error>>
{
    let log = Rc::new(RefCell::new(Vec::new()));
    let budget = PendingBudget::new(1, 100)?;
    let mut service = ServiceLifecycle::new(
        ServiceRole::Engine,
        Reporter(Rc::clone(&log)),
        budget,
        budget,
        NonZeroU32::MIN,
    );
    let notification = StopNotification::new();
    assert_eq!(notification.handle_control(1), ScmControl::Stop);
    assert_eq!(service.state(), None);
    assert_eq!(log.borrow().len(), 0);
    let now = Instant::now();
    service.start(now)?;
    service.ready(now)?;
    let before = service.acknowledged();
    assert_eq!(notification.handle_control(1).return_code(), 0);
    assert_eq!(service.acknowledged(), before);
    assert_eq!(log.borrow().len(), 2);
    service.request_stop(now)?;
    service.finish_stop(now, StopOutcome::Clean)?;
    assert_eq!(log.borrow().len(), 4);
    Ok(())
}

/// Race real threads beside non-Stop traffic without allocating per notification.
///
/// # Panics
/// Fails if a worker observes a wrong reply or all Stop notifications are lost.
#[test]
fn concurrent_notifications_preserve_a_single_sticky_stop_request() {
    for send_stop in [false, true] {
        let notification = Arc::new(StopNotification::new());
        let barrier = Arc::new(Barrier::new(8));
        std::thread::scope(|scope| {
            for worker in 0..8 {
                let notification = Arc::clone(&notification);
                let barrier = Arc::clone(&barrier);
                let _sender = scope.spawn(move || {
                    barrier.wait();
                    for _ in 0..64 {
                        assert_eq!(notification.handle_control(4), ScmControl::Interrogate);
                        assert_eq!(notification.handle_control(u32::MAX).return_code(), 120);
                        if send_stop && worker % 2 == 0 {
                            assert_eq!(notification.handle_control(1), ScmControl::Stop);
                        }
                    }
                });
            }
        });
        assert_eq!(notification.stop_requested(), send_stop);
        assert_eq!(notification.handle_control(4).return_code(), 0);
        assert_eq!(notification.stop_requested(), send_stop);
    }
}

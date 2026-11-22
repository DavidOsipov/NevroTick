// SPDX-License-Identifier: GPL-2.0-only
//! Actual broker hosting transitions with a controlled reservation.

use std::{cell::Cell, task::Poll};

use super::{
    Factory, HealthyTokenObserver, Observations, Rc, RefCell, TestClock, TestWait, Time,
    WaitAction, statuses,
};
use crate::{
    FixtureError, LabRole, NativeError, NativeOperation,
    broker_clock_change::{BrokerClockChanges, NativeBrokerReader},
    broker_endpoint::{BrokerEndpointWorker, Reservation},
    error::WorkerError,
    host::{HostReceipt, Worker, WorkerFinish, WorkerWake, run_broker_service_with_worker},
};
use std::time::{Duration, Instant};
use windows_platform::{
    scm::{ScmControl, StopNotification},
    service::LifecycleError,
};

/// Observation made at the exact point of an acquisition or close call.
#[derive(Default)]
struct EndpointTrace {
    /// Last actual delivered SCM state before acquisition.
    acquired_after: Option<u32>,
    /// Last actual delivered SCM state before close.
    closed_after: Option<u32>,
    /// Total closure calls on the original owned resource.
    close_calls: usize,
}

/// A controlled handle whose explicit close observes status and can fail.
struct TestEndpoint<'a> {
    /// Original host's delivered status observations.
    observations: &'a Rc<RefCell<Observations>>,
    /// Independent exact calls, recorded when handling the original handle.
    endpoints: &'a RefCell<EndpointTrace>,
    /// Opt-in close refusal.
    failure: Option<NativeError>,
}

impl Reservation for TestEndpoint<'_> {
    /// Record release on the original handle, preserving any real fixture refusal.
    fn close(self) -> Result<(), NativeError> {
        let mut trace: std::cell::RefMut<'_, EndpointTrace> = self.endpoints.borrow_mut();
        trace.closed_after = statuses(self.observations).last().map(|status| status[1]);
        trace.close_calls += 1;
        self.failure.map_or(Ok(()), Err)
    }
}

/// Startup controls selected independently from the observed result.
#[derive(Clone, Copy)]
enum Scenario {
    /// Prepare, report Running, then stop while idle.
    Healthy,
    /// Preserve first-instance collision refusal.
    Collision,
    /// Receive Stop only after the endpoint was acquired.
    StopDuringPrepare,
    /// Finish acquisition at the original startup deadline.
    DeadlineDuringPrepare,
    /// Refuse the Running status even though acquisition succeeded.
    FailedRunningReport,
}

/// Observe the native-role driver with independent endpoint and SCM controls.
///
/// # Panics
/// Panics if the controlled host deviates from acquisition and release ordering.
#[test]
fn broker_endpoint_is_owned_before_running_until_physical_release() {
    static CHANGES: [BrokerClockChanges; 5] = [const { BrokerClockChanges::new() }; 5];
    let journeys: [Scenario; 5] = [
        Scenario::Healthy,
        Scenario::Collision,
        Scenario::StopDuringPrepare,
        Scenario::DeadlineDuringPrepare,
        Scenario::FailedRunningReport,
    ];
    for (scenario, changes) in journeys.into_iter().zip(CHANGES.iter()) {
        let observation: Rc<RefCell<Observations>> = Rc::new(RefCell::new(Observations::default()));
        let trace: RefCell<EndpointTrace> = RefCell::new(EndpointTrace::default());
        let stop: StopNotification = StopNotification::new();
        let time: Rc<RefCell<Time>> = Rc::new(RefCell::new(Time {
            current: Instant::now(),
            backwards_once: false,
            backwards_persistent: false,
        }));
        let collision: NativeError = NativeError::new(NativeOperation::BrokerEndpointCreate, 5);
        let report_failure: NativeError = NativeError::new(NativeOperation::Status, 13);
        if matches!(scenario, Scenario::FailedRunningReport) {
            observation
                .borrow_mut()
                .status_failures
                .push((2, report_failure));
        }
        let mut clock: TestClock = TestClock(Rc::clone(&time));
        let mut reporting: Factory = Factory {
            observations: Rc::clone(&observation),
            failure: None,
        };
        let mut wait: TestWait<'_> = TestWait {
            time: Rc::clone(&time),
            observations: Rc::clone(&observation),
            actions: vec![WaitAction::Stop(Duration::ZERO)],
            next: 0,
            notification: &stop,
        };
        let acquire = || -> Result<TestEndpoint<'_>, NativeError> {
            trace.borrow_mut().acquired_after =
                statuses(&observation).last().map(|status| status[1]);
            match scenario {
                Scenario::Healthy | Scenario::FailedRunningReport => {}
                Scenario::Collision => return Err(collision),
                Scenario::StopDuringPrepare => {
                    assert_eq!(stop.handle_control(1), ScmControl::Stop);
                }
                Scenario::DeadlineDuringPrepare => {
                    time.borrow_mut().current += Duration::from_secs(10);
                }
            }
            Ok(TestEndpoint {
                observations: &observation,
                endpoints: &trace,
                failure: None,
            })
        };
        #[cfg(windows)]
        let reader: NativeBrokerReader = crate::broker_clock_change::sampled_native_reader(changes);
        #[cfg(not(windows))]
        let reader: NativeBrokerReader = NativeBrokerReader::new(changes, || Ok(0));
        let receipt: HostReceipt = run_broker_service_with_worker(
            0,
            &mut reporting,
            &mut HealthyTokenObserver(LabRole::ClockBroker),
            &mut clock,
            &mut wait,
            &stop,
            (reader, BrokerEndpointWorker::new(acquire)),
        );
        let ordered: Vec<u32> = statuses(&observation)
            .iter()
            .map(|status| status[1])
            .collect();
        let recorded: std::cell::Ref<'_, EndpointTrace> = trace.borrow();
        verify_journey(scenario, &receipt, &ordered, &recorded, wait.next);
    }
}

/// Check actual effects against each independent scenario's expected outcome.
///
/// # Panics
/// Panics on a different terminal cause, early readiness, or out-of-order close.
fn verify_journey(
    scenario: Scenario,
    receipt: &HostReceipt,
    ordered: &[u32],
    recorded: &EndpointTrace,
    waits: usize,
) {
    assert_eq!(recorded.acquired_after, Some(2));
    assert_eq!(ordered.last(), Some(&1), "physical close precedes Stopped");
    let primary: Option<&FixtureError> = match receipt.result() {
        Ok(()) => None,
        Err(FixtureError::Cleanup(facts)) => Some(facts.primary()),
        Err(cause) => Some(cause),
    };
    match scenario {
        Scenario::Healthy => {
            assert!(primary.is_none());
            assert_eq!(ordered, [2, 4, 3, 1]);
        }
        Scenario::Collision => {
            let collision: NativeError = NativeError::new(NativeOperation::BrokerEndpointCreate, 5);
            assert_eq!(
                primary,
                Some(&FixtureError::Worker(WorkerError::EndpointPreparation(
                    collision
                )))
            );
            assert!(!ordered.contains(&4));
            assert_eq!(recorded.close_calls, 0);
            return;
        }
        Scenario::StopDuringPrepare => {
            assert!(primary.is_none());
            assert!(!ordered.contains(&4));
        }
        Scenario::DeadlineDuringPrepare => {
            assert_eq!(
                primary,
                Some(&FixtureError::Lifecycle(LifecycleError::DeadlineElapsed))
            );
            assert!(!ordered.contains(&4));
        }
        Scenario::FailedRunningReport => {
            let report: NativeError = NativeError::new(NativeOperation::Status, 13);
            assert_eq!(
                primary,
                Some(&FixtureError::Lifecycle(LifecycleError::Report(report)))
            );
            assert!(
                ordered.contains(&4),
                "the failed reporter was actually reached"
            );
            assert_eq!(
                waits, 0,
                "unacknowledged Running never reaches the work wait"
            );
        }
    }
    assert_eq!(recorded.close_calls, 1);
    assert_eq!(recorded.closed_after, Some(3));
}

/// A failed close retains the cleanup error and cannot acknowledge endpoint release.
///
/// # Panics
/// Panics if release is retried or if Stop produces invented completion.
#[test]
fn broker_worker_retains_unconfirmed_endpoint_release_without_retry() {
    let calls: Cell<usize> = Cell::new(0);
    let trace: RefCell<EndpointTrace> = RefCell::default();
    let observations: Rc<RefCell<Observations>> = Rc::new(RefCell::default());
    let raw: NativeError = NativeError::new(NativeOperation::BrokerEndpointClose, 6);
    let wake: WorkerWake = WorkerWake::current();
    let acquire = || {
        calls.set(calls.get() + 1);
        Ok(TestEndpoint {
            observations: &observations,
            endpoints: &trace,
            failure: Some(raw),
        })
    };
    let mut worker: BrokerEndpointWorker<_, TestEndpoint<'_>> = BrokerEndpointWorker::new(acquire);
    assert_eq!(worker.poll_finished(&wake), WorkerFinish::Pending);
    assert_eq!(worker.poll_prepare(&wake), Poll::Ready(Ok(())));
    assert_eq!(worker.activate(), Ok(()));
    worker.request_cancel(&wake);
    for _ in 0..2 {
        assert_eq!(
            worker.poll_finished(&wake),
            WorkerFinish::NativeDrainPending {
                cleanup_failure: Some(WorkerError::EndpointClose(raw)),
            }
        );
        worker.request_cancel(&wake);
    }
    assert_eq!(calls.get(), 1);
    assert_eq!(trace.borrow().close_calls, 1);
}

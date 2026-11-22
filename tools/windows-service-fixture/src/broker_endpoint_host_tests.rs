// SPDX-License-Identifier: GPL-2.0-only
//! Actual broker hosting transitions with a controlled reservation.

use std::{cell::Cell, task::Poll};

use super::{
    Event, Factory, HealthyTokenObserver, Observations, Rc, RefCell, TestClock, TestWait, Time,
    WaitAction, statuses,
};
use crate::{
    FixtureError, LabRole, NativeError, NativeOperation,
    broker_clock_change::{BrokerClockChanges, NativeBrokerReader},
    broker_endpoint::{AcceptCompletion, AcceptStart, BrokerEndpointWorker, Reservation},
    error::WorkerError,
    host::{
        HostReceipt, Worker, WorkerCompletion, WorkerFailure, WorkerFinish, WorkerWake,
        run_broker_service_with_worker,
    },
};
use std::time::{Duration, Instant};
use windows_platform::{
    scm::{ScmControl, StopNotification},
    service::LifecycleError,
};

/// Exact observed native call and last delivered SCM state before the call.
#[derive(Debug, Eq, PartialEq)]
enum EndpointCall {
    /// Begin the one overlapped accept.
    Begin(Option<u32>),
    /// Request cancellation on the submitted accept.
    Cancel(Option<u32>),
    /// Collect completion of the retained accept.
    Poll(Option<u32>),
    /// Release the original endpoint.
    Close(Option<u32>),
}

/// Observations made by the actual worker's endpoint.
#[derive(Default)]
struct EndpointTrace {
    /// Last actual delivered SCM state before acquisition.
    acquired_after: Option<u32>,
    /// Ordered calls on the original endpoint.
    calls: Vec<EndpointCall>,
}

/// Fixed scripted observations, independent of the worker's decisions.
struct AcceptScript<'a> {
    /// Submission disposition for the original accept.
    start: Result<AcceptStart, NativeError>,
    /// Cancellation request disposition, without any acknowledgement implied.
    cancel: Result<(), NativeError>,
    /// Native completion observations returned in sequence.
    polls: &'a [Result<Poll<AcceptCompletion>, NativeError>],
}

/// The accept queues and then reports terminal cancellation on Stop.
const CANCELLABLE_ACCEPT: AcceptScript<'static> = AcceptScript {
    start: Ok(AcceptStart::Queued),
    cancel: Ok(()),
    polls: &[Ok(Poll::Ready(AcceptCompletion::Cancelled))],
};

/// A controlled reservation whose native actions record SCM status.
struct TestEndpoint<'a> {
    /// Original host's delivered status observations.
    observations: &'a Rc<RefCell<Observations>>,
    /// Independent exact calls, recorded when handling the original handle.
    endpoints: &'a RefCell<EndpointTrace>,
    /// Opt-in close refusal.
    failure: Option<NativeError>,
    /// Reproducible accept history at the native-call seam.
    accept: AcceptScript<'a>,
    /// Number of completion observations already collected.
    polls: usize,
}

impl Reservation for TestEndpoint<'_> {
    /// Record which acknowledged host state permitted submission.
    fn begin_accept(&mut self) -> Result<AcceptStart, NativeError> {
        self.record(EndpointCall::Begin);
        self.accept.start
    }

    /// Record cancellation independently of its acceptance or terminal result.
    fn request_accept_cancel(&mut self) -> Result<(), NativeError> {
        self.record(EndpointCall::Cancel);
        self.accept.cancel
    }

    /// Return the next fixed observation. An extra poll remains pending.
    fn poll_accept(&mut self) -> Result<Poll<AcceptCompletion>, NativeError> {
        self.record(EndpointCall::Poll);
        let result: Result<Poll<AcceptCompletion>, NativeError> = self
            .accept
            .polls
            .get(self.polls)
            .copied()
            .unwrap_or(Ok(Poll::Pending));
        self.polls += 1;
        result
    }

    /// Record release on the original handle, preserving any real fixture refusal.
    fn close(self) -> Result<(), NativeError> {
        self.record(EndpointCall::Close);
        self.failure.map_or(Ok(()), Err)
    }
}

impl TestEndpoint<'_> {
    /// Record a call before returning its independently prescribed result.
    fn record(&self, call: fn(Option<u32>) -> EndpointCall) {
        let state: Option<u32> = statuses(self.observations).last().map(|status| status[1]);
        self.endpoints.borrow_mut().calls.push(call(state));
    }
}

/// Startup controls selected independently from the observed result.
#[derive(Clone, Copy)]
enum Scenario {
    /// Prepare, report Running, submit one accept and cancel on Stop.
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
                accept: CANCELLABLE_ACCEPT,
                polls: 0,
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
            assert_eq!(
                recorded.calls,
                [
                    EndpointCall::Begin(Some(4)),
                    EndpointCall::Cancel(Some(3)),
                    EndpointCall::Poll(Some(3)),
                    EndpointCall::Close(Some(3)),
                ]
            );
            return;
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
            let expected: [EndpointCall; 0] = [];
            assert_eq!(recorded.calls, expected);
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
    assert_eq!(recorded.calls, [EndpointCall::Close(Some(3))]);
}

/// The real host retains the queued accept through its original Stop deadline.
///
/// # Panics
/// Panics if polling renews Stop, discards the accept early or reports Clean.
#[test]
fn broker_accept_late_completion_remains_owned_past_stop_cutoff() {
    static CHANGES: BrokerClockChanges = BrokerClockChanges::new();
    let observation: Rc<RefCell<Observations>> = Rc::new(RefCell::new(Observations::default()));
    let trace: RefCell<EndpointTrace> = RefCell::default();
    let stop: StopNotification = StopNotification::new();
    let time: Rc<RefCell<Time>> = Rc::new(RefCell::new(Time {
        current: Instant::now(),
        backwards_once: false,
        backwards_persistent: false,
    }));
    let mut clock: TestClock = TestClock(Rc::clone(&time));
    let mut reporting: Factory = Factory {
        observations: Rc::clone(&observation),
        failure: None,
    };
    let mut wait: TestWait<'_> = TestWait {
        time,
        observations: Rc::clone(&observation),
        actions: vec![
            WaitAction::Stop(Duration::ZERO),
            WaitAction::Spurious(Duration::from_millis(9_999)),
            WaitAction::Spurious(Duration::from_millis(1)),
            WaitAction::Spurious(Duration::from_millis(20)),
        ],
        next: 0,
        notification: &stop,
    };
    let acquire = || -> Result<TestEndpoint<'_>, NativeError> {
        Ok(TestEndpoint {
            observations: &observation,
            endpoints: &trace,
            failure: None,
            accept: AcceptScript {
                polls: &[
                    Ok(Poll::Pending),
                    Ok(Poll::Pending),
                    Ok(Poll::Pending),
                    Ok(Poll::Ready(AcceptCompletion::Connected)),
                ],
                ..CANCELLABLE_ACCEPT
            },
            polls: 0,
        })
    };
    #[cfg(windows)]
    let reader: NativeBrokerReader = crate::broker_clock_change::sampled_native_reader(&CHANGES);
    #[cfg(not(windows))]
    let reader: NativeBrokerReader = NativeBrokerReader::new(&CHANGES, || Ok(0));
    let receipt: HostReceipt = run_broker_service_with_worker(
        0,
        &mut reporting,
        &mut HealthyTokenObserver(LabRole::ClockBroker),
        &mut clock,
        &mut wait,
        &stop,
        (reader, BrokerEndpointWorker::new(acquire)),
    );

    assert_eq!(
        receipt.result(),
        &Err(FixtureError::Lifecycle(LifecycleError::DeadlineElapsed))
    );
    let status_rows: Vec<[u32; 7]> = statuses(&observation);
    assert_eq!(
        status_rows,
        [
            [16, 2, 0, 0, 0, 1, 1000],
            [16, 4, 1, 0, 0, 0, 0],
            [16, 3, 0, 0, 0, 1, 1000],
            [16, 1, 0, 1066, 1, 0, 0],
        ]
    );
    let waits: Vec<Duration> = observation
        .borrow()
        .events
        .iter()
        .filter_map(|event: &Event| match event {
            Event::Wait(duration) => Some(*duration),
            _ => None,
        })
        .collect();
    assert!(matches!(
        waits.as_slice(),
        [_running, first, last_before_deadline, overdue]
            if *first == Duration::from_millis(20)
                && *last_before_deadline == Duration::from_millis(1)
                && *overdue == Duration::from_millis(20)
    ));
    assert_eq!(
        trace.borrow().calls,
        [
            EndpointCall::Begin(Some(4)),
            EndpointCall::Cancel(Some(3)),
            EndpointCall::Poll(Some(3)),
            EndpointCall::Poll(Some(3)),
            EndpointCall::Poll(Some(3)),
            EndpointCall::Poll(Some(3)),
            EndpointCall::Close(Some(3)),
        ]
    );
}

/// Refused or already-terminal accepts cannot be resubmitted or dispatched.
///
/// # Panics
/// Panics if the worker polls/cancels an accept without outstanding native work.
#[test]
fn broker_worker_fences_running_accept_refusal_and_unexpected_completion() {
    let raw: NativeError = NativeError::new(NativeOperation::BrokerEndpointAcceptSubmit, 5);
    let outcomes: [(Result<AcceptStart, NativeError>, WorkerError); 3] = [
        (Err(raw), WorkerError::EndpointAccept(raw)),
        (
            Ok(AcceptStart::Connected),
            WorkerError::EndpointUnexpectedCompletion,
        ),
        (
            Ok(AcceptStart::Queued),
            WorkerError::EndpointUnexpectedCompletion,
        ),
    ];
    for (start, expected) in outcomes {
        let trace: RefCell<EndpointTrace> = RefCell::default();
        let observations: Rc<RefCell<Observations>> = Rc::default();
        let wake: WorkerWake = WorkerWake::current();
        let acquire = || -> Result<TestEndpoint<'_>, NativeError> {
            Ok(TestEndpoint {
                observations: &observations,
                endpoints: &trace,
                failure: None,
                accept: AcceptScript {
                    start,
                    polls: &[Ok(Poll::Ready(AcceptCompletion::Connected))],
                    ..CANCELLABLE_ACCEPT
                },
                polls: 0,
            })
        };
        let mut worker: BrokerEndpointWorker<_, TestEndpoint<'_>> =
            BrokerEndpointWorker::new(acquire);
        assert_eq!(worker.poll_prepare(&wake), Poll::Ready(Ok(())));
        assert_eq!(
            worker.poll_running(&wake),
            Err(WorkerFailure::Operation(WorkerError::Running))
        );
        let empty: [EndpointCall; 0] = [];
        assert_eq!(trace.borrow().calls, empty);
        assert_eq!(worker.activate(), Ok(()));
        if start == Ok(AcceptStart::Queued) {
            assert_eq!(worker.poll_running(&wake), Ok(()));
        }
        assert_eq!(
            worker.poll_running(&wake),
            Err(WorkerFailure::Operation(expected))
        );
        assert_eq!(
            worker.poll_running(&wake),
            Err(WorkerFailure::Operation(WorkerError::Running))
        );
        worker.request_cancel(&wake);
        assert_eq!(
            worker.poll_finished(&wake),
            WorkerFinish::Finished(WorkerCompletion {
                operation_failure: None,
                cleanup_failure: None,
            })
        );
        let recorded: std::cell::Ref<'_, EndpointTrace> = trace.borrow();
        let expected: &[EndpointCall] = if start == Ok(AcceptStart::Queued) {
            &[
                EndpointCall::Begin(None),
                EndpointCall::Poll(None),
                EndpointCall::Close(None),
            ]
        } else {
            &[EndpointCall::Begin(None), EndpointCall::Close(None)]
        };
        assert_eq!(recorded.calls, expected);
    }
}

/// An indeterminate finish and a cancel refusal preserve the original slot.
///
/// # Panics
/// Panics if the first cleanup cause is replaced or a request becomes completion.
#[test]
fn broker_worker_preserves_accept_custody_and_first_cleanup_failure() {
    let running_failure: NativeError =
        NativeError::new(NativeOperation::BrokerEndpointAcceptCompletion, 6);
    let repeat_failure: NativeError =
        NativeError::new(NativeOperation::BrokerEndpointAcceptCompletion, 87);
    let cancel_failure: NativeError =
        NativeError::new(NativeOperation::BrokerEndpointAcceptCancel, 5);
    let close_failure: NativeError = NativeError::new(NativeOperation::BrokerEndpointClose, 6);
    let polls: [Result<Poll<AcceptCompletion>, NativeError>; 4] = [
        Err(running_failure),
        Err(repeat_failure),
        Ok(Poll::Pending),
        Ok(Poll::Ready(AcceptCompletion::Cancelled)),
    ];
    for (cancel, close) in [
        (Ok(()), None),
        (Err(cancel_failure), None),
        (Err(cancel_failure), Some(close_failure)),
    ] {
        let trace: RefCell<EndpointTrace> = RefCell::default();
        let observations: Rc<RefCell<Observations>> = Rc::default();
        let wake: WorkerWake = WorkerWake::current();
        let acquire = || -> Result<TestEndpoint<'_>, NativeError> {
            Ok(TestEndpoint {
                observations: &observations,
                endpoints: &trace,
                failure: close,
                accept: AcceptScript {
                    cancel,
                    polls: &polls,
                    ..CANCELLABLE_ACCEPT
                },
                polls: 0,
            })
        };
        let mut worker: BrokerEndpointWorker<_, TestEndpoint<'_>> =
            BrokerEndpointWorker::new(acquire);
        assert_eq!(worker.poll_prepare(&wake), Poll::Ready(Ok(())));
        assert_eq!(worker.activate(), Ok(()));
        assert_eq!(worker.poll_running(&wake), Ok(()));
        assert_eq!(
            worker.poll_running(&wake),
            Err(WorkerFailure::Operation(WorkerError::EndpointAccept(
                running_failure
            )))
        );
        assert_eq!(
            worker.poll_running(&wake),
            Err(WorkerFailure::Operation(WorkerError::Running))
        );
        worker.request_cancel(&wake);
        let cleanup: WorkerError =
            WorkerError::EndpointAcceptCleanup(cancel.err().unwrap_or(repeat_failure));
        let pending: WorkerFinish = WorkerFinish::NativeDrainPending {
            cleanup_failure: Some(cleanup),
        };
        for _ in 0..2 {
            assert_eq!(worker.poll_finished(&wake), pending);
            worker.request_cancel(&wake);
        }
        let finished: WorkerFinish = if close.is_none() {
            WorkerFinish::Finished(WorkerCompletion {
                operation_failure: None,
                cleanup_failure: Some(cleanup),
            })
        } else {
            pending
        };
        assert_eq!(worker.poll_finished(&wake), finished);
        assert_eq!(worker.poll_finished(&wake), finished);
        assert_eq!(
            trace.borrow().calls,
            [
                EndpointCall::Begin(None),
                EndpointCall::Poll(None),
                EndpointCall::Cancel(None),
                EndpointCall::Poll(None),
                EndpointCall::Poll(None),
                EndpointCall::Poll(None),
                EndpointCall::Close(None),
            ]
        );
    }
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
            accept: CANCELLABLE_ACCEPT,
            polls: 0,
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
    assert_eq!(trace.borrow().calls, [EndpointCall::Close(None)]);
}

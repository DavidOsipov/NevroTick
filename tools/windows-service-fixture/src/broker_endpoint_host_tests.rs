// SPDX-License-Identifier: GPL-2.0-only
//! Actual broker hosting transitions with a controlled reservation.

use std::{cell::Cell, mem::ManuallyDrop, task::Poll};

#[cfg(windows)]
use std::os::windows::io::BorrowedHandle;

#[cfg(windows)]
use crate::native_broker_wait::{BrokerControlWake, CompletionWakeState, PendingCompletionWake};

use super::{
    DiagnosticView, Event, Factory, HealthyTokenObserver, Observations, Rc, RefCell, Reporter,
    TestClock, TestWait, Time, WaitAction, startup_error, statuses,
};
use crate::{
    FixtureError, LabRole, NativeError, NativeOperation,
    broker_clock_change::{
        BrokerClockChanges, ClockRevision, ControlDisposition, NativeBrokerReader, ReplyRevision,
        deliver,
    },
    broker_endpoint::{
        AcceptCompletion, AcceptStart, BrokerEndpointWorker, FirstReadCompletion, FirstReadStart,
        FirstReplyCompletion, FirstReplyStart, Reservation,
    },
    error::WorkerError,
    host::{
        Clock, ConnectionEdge, ConnectionWait, HostReceipt, InvocationPoll, ObservedClock,
        PrefillRefusal, RunningConnection, ServiceInvocation, Worker, WorkerCompletion,
        WorkerFailure, WorkerFinish, WorkerWake, run_broker_service_with_worker,
        start_broker_worker, start_invocation,
    },
};
use std::time::{Duration, Instant};
use windows_platform::{
    scm::{ScmControl, StopNotification},
    service::{LifecycleError, ServiceState},
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
    /// Observe the retained peer before writing the prefill marker.
    ObservePrefillPeer(Option<u32>),
    /// Submit the prefill marker on the original endpoint.
    BeginPrefillMarker(Option<u32>),
    /// Collect the original prefill marker.
    PollPrefillMarker(Option<u32>),
    /// Submit the only read on the observed, connected endpoint.
    BeginRead(Option<u32>),
    /// Observe physical completion on that read.
    PollRead(Option<u32>),
    /// Request cancellation of the original read.
    CancelRead(Option<u32>),
    /// Submit the reply once on the original endpoint.
    BeginReply(Option<u32>),
    /// Observe the exact submitted reply.
    PollReply(Option<u32>),
    /// Request cancellation of the submitted reply.
    CancelReply(Option<u32>),
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
    /// The physical write results returned by the original endpoint, including drain.
    reply_results: Vec<Result<Poll<FirstReplyCompletion>, NativeError>>,
    /// Currentness observed on the original scripted peer at each reached guard.
    reply_peer_checks: Vec<bool>,
}

/// Fixed scripted observations, independent of the worker's decisions.
struct AcceptScript<'a> {
    /// Submission disposition for the original accept.
    start: Result<AcceptStart, NativeError>,
    /// Cancellation request disposition, without any acknowledgement implied.
    cancel: Result<(), NativeError>,
    /// Native completion observations returned in sequence.
    polls: &'a [Result<Poll<AcceptCompletion>, NativeError>],
    /// Optional independent event delivered during native submission.
    on_begin: Option<&'a dyn Fn()>,
    /// Optional independent event delivered during the first native observation.
    on_first_poll: Option<&'a dyn Fn()>,
}

/// Inject one wait failure, then let the real owner drive its fallback waits.
#[cfg(windows)]
struct FailedWait<'a> {
    /// Retains the existing scripted clock and observations.
    fallback: TestWait<'a>,
    /// The original adapter failure is already available to the owner.
    failed: bool,
}

#[cfg(windows)]
impl crate::host::Wait for FailedWait<'_> {
    /// Keep the original test wait on every fallback observation.
    fn park(&mut self, timeout: Duration) {
        crate::host::Wait::park(&mut self.fallback, timeout);
    }

    /// Refuse exactly one native wait, leaving actual completion to the worker.
    /// # Panics
    /// Panics if the host drops the charged worker's signal on either wait.
    fn park_with_completion(
        &mut self,
        timeout: Duration,
        completion: Option<PendingCompletionWake<'_>>,
    ) -> Result<(), NativeError> {
        assert!(completion.is_some());
        if self.failed {
            self.park(timeout);
            Ok(())
        } else {
            self.failed = true;
            Err(NativeError::new(NativeOperation::BrokerWakeWait, 6))
        }
    }
}

/// The accept queues and then reports terminal cancellation on Stop.
const CANCELLABLE_ACCEPT: AcceptScript<'static> = AcceptScript {
    start: Ok(AcceptStart::Queued),
    cancel: Ok(()),
    polls: &[Ok(Poll::Ready(AcceptCompletion::Cancelled))],
    on_begin: None,
    on_first_poll: None,
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
    /// Selected live signal for a test that checks handoff to the actual host wait.
    #[cfg(windows)]
    hint: Option<(BorrowedHandle<'a>, CompletionWakeState)>,
}

impl Reservation for TestEndpoint<'_> {
    /// Record which acknowledged host state permitted submission.
    fn begin_accept(&mut self) -> Result<AcceptStart, NativeError> {
        self.record(EndpointCall::Begin);
        if let Some(action) = self.accept.on_begin {
            action();
        }
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
        if self.polls == 0
            && let Some(action) = self.accept.on_first_poll
        {
            action();
        }
        let result: Result<Poll<AcceptCompletion>, NativeError> = self
            .accept
            .polls
            .get(self.polls)
            .copied()
            .unwrap_or(Ok(Poll::Pending));
        self.polls += 1;
        result
    }

    /// Lend the selected live signal and its scripted operation state together.
    #[cfg(windows)]
    fn completion_wake(&mut self) -> Option<PendingCompletionWake<'_>> {
        let (event, state): &mut (BorrowedHandle<'_>, CompletionWakeState) = self.hint.as_mut()?;
        Some(state.lend(*event))
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
                #[cfg(windows)]
                hint: None,
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

/// A broken wait reaches the running owner and retains custody through fallback.
///
/// # Errors
/// Returns an unexpected failure to create the private event for the handoff.
///
/// # Panics
/// Fails if the host bypasses the Windows wait, retires the accept on failure,
/// or loses the original operation before publishing its failed status.
#[cfg(windows)]
#[test]
fn broker_wait_failure_preserves_the_original_accept_until_fallback_completion()
-> Result<(), NativeError> {
    static CHANGES: BrokerClockChanges = BrokerClockChanges::new();
    let observation: Rc<RefCell<Observations>> = Rc::new(RefCell::new(Observations::default()));
    let trace: RefCell<EndpointTrace> = RefCell::default();
    let event: BrokerControlWake = BrokerControlWake::new()?;
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
    let mut wait: FailedWait<'_> = FailedWait {
        failed: false,
        fallback: TestWait {
            time,
            observations: Rc::clone(&observation),
            actions: vec![WaitAction::Spurious(Duration::from_millis(20))],
            next: 0,
            notification: &stop,
        },
    };
    let acquire = || -> Result<TestEndpoint<'_>, NativeError> {
        Ok(TestEndpoint {
            observations: &observation,
            endpoints: &trace,
            failure: None,
            accept: AcceptScript {
                polls: &[
                    Ok(Poll::Pending),
                    Ok(Poll::Ready(AcceptCompletion::Cancelled)),
                ],
                ..CANCELLABLE_ACCEPT
            },
            polls: 0,
            #[cfg(windows)]
            hint: Some((event.test_event(), CompletionWakeState::new())),
        })
    };
    let reader: NativeBrokerReader = crate::broker_clock_change::sampled_native_reader(&CHANGES);
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
        &Err(FixtureError::Native(NativeError::new(
            NativeOperation::BrokerWakeWait,
            6
        )))
    );
    assert_eq!(wait.fallback.next, 1);
    assert_eq!(
        trace.borrow().calls,
        [
            EndpointCall::Begin(Some(4)),
            EndpointCall::Cancel(Some(3)),
            EndpointCall::Poll(Some(3)),
            EndpointCall::Poll(Some(3)),
            EndpointCall::Close(Some(3)),
        ]
    );
    assert_eq!(
        statuses(&observation).last(),
        Some(&[16, 1, 0, 1066, 1, 0, 0])
    );
    Ok(())
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
            #[cfg(windows)]
            hint: None,
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

/// Supply a controlled monotonic clock to the real serialized host.
fn fixed_accept_clock() -> TestClock {
    TestClock(Rc::new(RefCell::new(Time {
        current: Instant::now(),
        backwards_once: false,
        backwards_persistent: false,
    })))
}

/// Give direct worker controls the same checked borrow as the invocation uses.
fn test_connection<'a>(
    observed: &'a mut ObservedClock,
    clock: &'a mut TestClock,
    notification: &'a StopNotification,
    cutoff: &'a mut Option<Instant>,
) -> RunningConnection<'a, TestClock> {
    RunningConnection {
        run_deadline: observed.last_trusted + Duration::from_secs(60),
        observed,
        clock,
        notification,
        cutoff,
        closed: false,
        failure: None,
        status_source: None,
        #[cfg(windows)]
        status_read: None,
        #[cfg(windows)]
        status_capture_observed: false,
        reply_written: false,
    }
}

/// The local cutoff retires one accept and preserves the service's original run.
///
/// # Errors
/// Returns the original startup refusal.
///
/// # Panics
/// Panics if the owner renews a cutoff, releases early, or fails on ordinary expiry.
#[test]
fn broker_accept_cutoff_retires_once_and_preserves_running() -> Result<(), FixtureError> {
    let observations: Rc<RefCell<Observations>> = Rc::default();
    let trace: RefCell<EndpointTrace> = RefCell::default();
    let stop: StopNotification = StopNotification::new();
    let mut clock: TestClock = fixed_accept_clock();
    let mut factory: Factory = Factory {
        observations: Rc::clone(&observations),
        failure: None,
    };
    let acquire = || -> Result<TestEndpoint<'_>, NativeError> {
        Ok(TestEndpoint {
            observations: &observations,
            endpoints: &trace,
            failure: None,
            accept: AcceptScript {
                polls: &[
                    Ok(Poll::Pending),
                    Ok(Poll::Pending),
                    Ok(Poll::Ready(AcceptCompletion::Connected)),
                ],
                ..CANCELLABLE_ACCEPT
            },
            polls: 0,
            #[cfg(windows)]
            hint: None,
        })
    };
    let mut invocation: ServiceInvocation<Reporter, BrokerEndpointWorker<_, TestEndpoint<'_>>> =
        start_invocation(
            LabRole::ClockBroker,
            0,
            &mut factory,
            &mut HealthyTokenObserver(LabRole::ClockBroker),
            &mut clock,
            &stop,
            BrokerEndpointWorker::new(acquire),
        )
        .map_err(startup_error)?;

    let steps: [(u64, u64); 5] = [(0, 5000), (4999, 1), (1, 20), (20, 20), (20, 54_960)];
    for (elapsed, next_wait) in steps {
        clock.0.borrow_mut().current += Duration::from_millis(elapsed);
        assert_eq!(
            invocation.poll(&mut clock, &stop),
            InvocationPoll::Pending(Duration::from_millis(next_wait))
        );
        assert_eq!(invocation.state(), Some(ServiceState::Running));
        assert_eq!(invocation.outstanding_work(), 1);
        assert!(!invocation.admission_closed());
        assert!(invocation.failure().is_none());
    }
    assert_eq!(
        trace.borrow().calls,
        [
            EndpointCall::Begin(Some(4)),
            EndpointCall::Poll(Some(4)),
            EndpointCall::Cancel(Some(4)),
            EndpointCall::Poll(Some(4)),
            EndpointCall::Poll(Some(4)),
        ]
    );
    clock.0.borrow_mut().current += Duration::from_millis(54_959);
    assert_eq!(
        invocation.poll(&mut clock, &stop),
        InvocationPoll::Pending(Duration::from_millis(1))
    );
    clock.0.borrow_mut().current += Duration::from_millis(1);
    assert_eq!(invocation.poll(&mut clock, &stop), InvocationPoll::Finished);
    assert_eq!(invocation.outstanding_work(), 0);
    assert_eq!(
        trace.borrow().calls.last(),
        Some(&EndpointCall::Close(Some(3)))
    );
    assert_eq!(trace.borrow().calls.len(), 6);
    assert_eq!(invocation.failure(), Some(&FixtureError::RunLimitElapsed));
    Ok(())
}

/// Conditions at the exact native completion observation.
#[derive(Clone, Copy)]
enum AcceptCompletionEdge {
    /// The deny-only reservation treats an eligible connection as unexpected.
    Timely,
    /// Trusted time reaches the original cutoff during the observation.
    Expired,
    /// Stop arrives during the observation.
    Stop,
}

/// Recheck cutoff and control after observing actual native completion.
///
/// # Errors
/// Returns the original startup refusal.
///
/// # Panics
/// Panics if physical completion alone determines current connection admission.
#[test]
fn broker_accept_rechecks_completion_at_cutoff_or_stop() -> Result<(), FixtureError> {
    for edge in [
        AcceptCompletionEdge::Timely,
        AcceptCompletionEdge::Expired,
        AcceptCompletionEdge::Stop,
    ] {
        let observations: Rc<RefCell<Observations>> = Rc::default();
        let trace: RefCell<EndpointTrace> = RefCell::default();
        let stop: StopNotification = StopNotification::new();
        let mut clock: TestClock = fixed_accept_clock();
        let time: Rc<RefCell<Time>> = Rc::clone(&clock.0);
        let mut factory: Factory = Factory {
            observations: Rc::clone(&observations),
            failure: None,
        };
        let during_observation = || match edge {
            AcceptCompletionEdge::Timely => {}
            AcceptCompletionEdge::Expired => {
                time.borrow_mut().current += Duration::from_millis(1);
            }
            AcceptCompletionEdge::Stop => {
                assert_eq!(stop.handle_control(1), ScmControl::Stop);
            }
        };
        let acquire = || -> Result<TestEndpoint<'_>, NativeError> {
            Ok(TestEndpoint {
                observations: &observations,
                endpoints: &trace,
                failure: None,
                accept: AcceptScript {
                    polls: &[Ok(Poll::Ready(AcceptCompletion::Connected))],
                    on_first_poll: Some(&during_observation),
                    ..CANCELLABLE_ACCEPT
                },
                polls: 0,
                #[cfg(windows)]
                hint: None,
            })
        };
        let mut invocation: ServiceInvocation<Reporter, BrokerEndpointWorker<_, TestEndpoint<'_>>> =
            start_invocation(
                LabRole::ClockBroker,
                0,
                &mut factory,
                &mut HealthyTokenObserver(LabRole::ClockBroker),
                &mut clock,
                &stop,
                BrokerEndpointWorker::new(acquire),
            )
            .map_err(startup_error)?;
        assert_eq!(
            invocation.poll(&mut clock, &stop),
            InvocationPoll::Pending(Duration::from_secs(5))
        );
        time.borrow_mut().current += Duration::from_millis(4999);
        let poll: InvocationPoll = invocation.poll(&mut clock, &stop);
        match edge {
            AcceptCompletionEdge::Timely => {
                assert_eq!(poll, InvocationPoll::Finished);
                assert_eq!(
                    invocation.failure(),
                    Some(&FixtureError::Worker(
                        WorkerError::EndpointUnexpectedCompletion
                    ))
                );
            }
            AcceptCompletionEdge::Expired => {
                assert_eq!(poll, InvocationPoll::Pending(Duration::from_secs(55)));
                assert_eq!(invocation.state(), Some(ServiceState::Running));
                assert_eq!(invocation.outstanding_work(), 1);
                assert_eq!(trace.borrow().calls.len(), 2);
                assert_eq!(stop.handle_control(1), ScmControl::Stop);
                assert_eq!(invocation.poll(&mut clock, &stop), InvocationPoll::Finished);
                assert!(invocation.failure().is_none());
            }
            AcceptCompletionEdge::Stop => {
                assert_eq!(poll, InvocationPoll::Finished);
                assert!(invocation.failure().is_none());
            }
        }
        assert_eq!(
            trace.borrow().calls,
            [
                EndpointCall::Begin(Some(4)),
                EndpointCall::Poll(Some(4)),
                EndpointCall::Close(Some(3)),
            ]
        );
        assert_eq!(invocation.outstanding_work(), 0);
    }
    Ok(())
}

/// A native submission may consume its entire window or observe concurrent Stop.
///
/// # Errors
/// Returns the original startup refusal.
///
/// # Panics
/// Panics if the owner loses a newly queued accept or cancels an already terminal one.
#[test]
fn broker_accept_rechecks_submission_at_cutoff_or_stop() -> Result<(), FixtureError> {
    for start in [AcceptStart::Queued, AcceptStart::Connected] {
        for edge in [AcceptCompletionEdge::Expired, AcceptCompletionEdge::Stop] {
            let observations: Rc<RefCell<Observations>> = Rc::default();
            let trace: RefCell<EndpointTrace> = RefCell::default();
            let stop: StopNotification = StopNotification::new();
            let mut clock: TestClock = fixed_accept_clock();
            let time: Rc<RefCell<Time>> = Rc::clone(&clock.0);
            let mut factory: Factory = Factory {
                observations: Rc::clone(&observations),
                failure: None,
            };
            let expires: bool = matches!(edge, AcceptCompletionEdge::Expired);
            let during_submission = || {
                if expires {
                    time.borrow_mut().current += Duration::from_secs(5);
                } else {
                    assert_eq!(stop.handle_control(1), ScmControl::Stop);
                }
            };
            let acquire = || -> Result<TestEndpoint<'_>, NativeError> {
                Ok(TestEndpoint {
                    observations: &observations,
                    endpoints: &trace,
                    failure: None,
                    accept: AcceptScript {
                        start: Ok(start),
                        on_begin: Some(&during_submission),
                        polls: &[Ok(Poll::Ready(AcceptCompletion::Connected))],
                        ..CANCELLABLE_ACCEPT
                    },
                    polls: 0,
                    #[cfg(windows)]
                    hint: None,
                })
            };
            let mut invocation: ServiceInvocation<
                Reporter,
                BrokerEndpointWorker<_, TestEndpoint<'_>>,
            > = start_invocation(
                LabRole::ClockBroker,
                0,
                &mut factory,
                &mut HealthyTokenObserver(LabRole::ClockBroker),
                &mut clock,
                &stop,
                BrokerEndpointWorker::new(acquire),
            )
            .map_err(startup_error)?;
            let poll: InvocationPoll = invocation.poll(&mut clock, &stop);
            let mut expected: Vec<EndpointCall> = vec![EndpointCall::Begin(Some(4))];
            if expires {
                let service_wait: InvocationPoll = InvocationPoll::Pending(Duration::from_secs(55));
                if start == AcceptStart::Queued {
                    assert_eq!(poll, InvocationPoll::Pending(Duration::from_millis(20)));
                    assert_eq!(invocation.poll(&mut clock, &stop), service_wait);
                    expected.push(EndpointCall::Cancel(Some(4)));
                    expected.push(EndpointCall::Poll(Some(4)));
                } else {
                    assert_eq!(poll, service_wait);
                }
                assert_eq!(invocation.outstanding_work(), 1);
                assert_eq!(trace.borrow().calls, expected);
                assert_eq!(stop.handle_control(1), ScmControl::Stop);
                assert_eq!(invocation.poll(&mut clock, &stop), InvocationPoll::Finished);
            } else {
                assert_eq!(poll, InvocationPoll::Finished);
                if start == AcceptStart::Queued {
                    expected.push(EndpointCall::Cancel(Some(3)));
                    expected.push(EndpointCall::Poll(Some(3)));
                }
            }
            expected.push(EndpointCall::Close(Some(3)));
            assert_eq!(trace.borrow().calls, expected);
            assert_eq!(invocation.outstanding_work(), 0);
            assert!(invocation.failure().is_none());
        }
    }
    Ok(())
}

/// Stop inherits local cancellation even when its native request was refused.
///
/// # Errors
/// Returns the original startup refusal.
///
/// # Panics
/// Panics if Stop cancels again, forgets the refusal, or falsely acknowledges cleanup.
#[test]
fn broker_stop_inherits_accept_cancellation_started_at_cutoff() -> Result<(), FixtureError> {
    let refusal: NativeError = NativeError::new(NativeOperation::BrokerEndpointAcceptCancel, 5);
    let later: NativeError = NativeError::new(NativeOperation::BrokerEndpointAcceptCompletion, 87);
    for cancel in [Ok(()), Err(refusal)] {
        let observations: Rc<RefCell<Observations>> = Rc::default();
        let trace: RefCell<EndpointTrace> = RefCell::default();
        let stop: StopNotification = StopNotification::new();
        let mut clock: TestClock = fixed_accept_clock();
        let mut factory: Factory = Factory {
            observations: Rc::clone(&observations),
            failure: None,
        };
        let polls: &[Result<Poll<AcceptCompletion>, NativeError>] = if cancel.is_ok() {
            &[
                Ok(Poll::Pending),
                Ok(Poll::Ready(AcceptCompletion::Connected)),
            ]
        } else {
            &[
                Ok(Poll::Pending),
                Err(later),
                Ok(Poll::Ready(AcceptCompletion::Connected)),
            ]
        };
        let acquire = || -> Result<TestEndpoint<'_>, NativeError> {
            Ok(TestEndpoint {
                observations: &observations,
                endpoints: &trace,
                failure: None,
                accept: AcceptScript {
                    cancel,
                    polls,
                    ..CANCELLABLE_ACCEPT
                },
                polls: 0,
                #[cfg(windows)]
                hint: None,
            })
        };
        let mut invocation: ServiceInvocation<Reporter, BrokerEndpointWorker<_, TestEndpoint<'_>>> =
            start_invocation(
                LabRole::ClockBroker,
                0,
                &mut factory,
                &mut HealthyTokenObserver(LabRole::ClockBroker),
                &mut clock,
                &stop,
                BrokerEndpointWorker::new(acquire),
            )
            .map_err(startup_error)?;
        assert_eq!(
            invocation.poll(&mut clock, &stop),
            InvocationPoll::Pending(Duration::from_secs(5))
        );
        clock.0.borrow_mut().current += Duration::from_secs(5);
        let poll: InvocationPoll = invocation.poll(&mut clock, &stop);
        let drain_wait: InvocationPoll = InvocationPoll::NativeDrain(Duration::from_millis(20));
        if cancel.is_ok() {
            assert_eq!(poll, InvocationPoll::Pending(Duration::from_millis(20)));
            assert_eq!(stop.handle_control(1), ScmControl::Stop);
            assert_eq!(invocation.poll(&mut clock, &stop), drain_wait);
        } else {
            assert_eq!(poll, drain_wait);
        }
        assert_eq!(invocation.state(), Some(ServiceState::StopPending));
        assert_eq!(invocation.outstanding_work(), 1);
        assert_eq!(trace.borrow().calls.len(), 3);
        let cleanup: Option<WorkerError> = cancel.err().map(WorkerError::EndpointAcceptCleanup);
        assert_eq!(invocation.pending_cleanup_failure, cleanup);
        let expected: Option<FixtureError> = cleanup.map(FixtureError::Worker);
        let mut calls: Vec<EndpointCall> = vec![
            EndpointCall::Begin(Some(4)),
            EndpointCall::Cancel(Some(4)),
            EndpointCall::Poll(Some(3)),
        ];
        if cancel.is_err() {
            assert_eq!(invocation.poll(&mut clock, &stop), drain_wait);
            assert_eq!(invocation.failure(), expected.as_ref());
            assert_eq!(invocation.outstanding_work(), 1);
            calls.push(EndpointCall::Poll(Some(3)));
        }
        assert_eq!(invocation.poll(&mut clock, &stop), InvocationPoll::Finished);
        assert_eq!(invocation.failure(), expected.as_ref());
        assert_eq!(invocation.outstanding_work(), 0);
        calls.push(EndpointCall::Poll(Some(3)));
        calls.push(EndpointCall::Close(Some(3)));
        assert_eq!(trace.borrow().calls, calls);
        let receipt: HostReceipt = invocation
            .take_receipt()
            .ok_or(FixtureError::OutcomeUnavailable)?;
        assert_eq!(receipt.result(), &expected.map_or(Ok(()), Err));
    }
    Ok(())
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
        let mut clock: TestClock = fixed_accept_clock();
        let mut observed: ObservedClock = ObservedClock::new(&mut clock);
        let notification: StopNotification = StopNotification::new();
        let mut cutoff: Option<Instant> = None;
        let mut connection: RunningConnection<'_, TestClock> =
            test_connection(&mut observed, &mut clock, &notification, &mut cutoff);
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
                #[cfg(windows)]
                hint: None,
            })
        };
        let mut worker: BrokerEndpointWorker<_, TestEndpoint<'_>> =
            BrokerEndpointWorker::new(acquire);
        assert_eq!(worker.poll_prepare(&wake), Poll::Ready(Ok(())));
        assert_eq!(
            worker.poll_running_with_connection(&wake, &mut connection),
            Err(WorkerFailure::Operation(WorkerError::Running))
        );
        let empty: [EndpointCall; 0] = [];
        assert_eq!(trace.borrow().calls, empty);
        assert_eq!(worker.activate(Instant::now()), Ok(()));
        if start == Ok(AcceptStart::Queued) {
            assert!(matches!(
                worker.poll_running_with_connection(&wake, &mut connection),
                Ok(ConnectionWait::Cutoff(_))
            ));
        }
        assert_eq!(
            worker.poll_running_with_connection(&wake, &mut connection),
            Err(WorkerFailure::Operation(expected))
        );
        assert_eq!(
            worker.poll_running_with_connection(&wake, &mut connection),
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
        let mut clock: TestClock = fixed_accept_clock();
        let mut observed: ObservedClock = ObservedClock::new(&mut clock);
        let notification: StopNotification = StopNotification::new();
        let mut cutoff: Option<Instant> = None;
        let mut connection: RunningConnection<'_, TestClock> =
            test_connection(&mut observed, &mut clock, &notification, &mut cutoff);
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
                #[cfg(windows)]
                hint: None,
            })
        };
        let mut worker: BrokerEndpointWorker<_, TestEndpoint<'_>> =
            BrokerEndpointWorker::new(acquire);
        assert_eq!(worker.poll_prepare(&wake), Poll::Ready(Ok(())));
        assert_eq!(worker.activate(Instant::now()), Ok(()));
        assert!(matches!(
            worker.poll_running_with_connection(&wake, &mut connection),
            Ok(ConnectionWait::Cutoff(_))
        ));
        assert_eq!(
            worker.poll_running_with_connection(&wake, &mut connection),
            Err(WorkerFailure::Operation(WorkerError::EndpointAccept(
                running_failure
            )))
        );
        assert_eq!(
            worker.poll_running_with_connection(&wake, &mut connection),
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

/// A refused status and unconfirmed close reach separate live diagnostic records.
///
/// # Errors
/// Returns the original failure if the controlled invocation cannot start.
///
/// # Panics
/// Panics if cleanup is hidden, repeated, or treated as successful retirement.
#[test]
fn broker_host_reports_pending_close_failure_without_acknowledging_retirement()
-> Result<(), FixtureError> {
    let report: NativeError = NativeError::new(NativeOperation::Status, 13);
    let close: NativeError = NativeError::new(NativeOperation::BrokerEndpointClose, 6);
    for (failed_status, retained_charge) in [(2, 0), (3, 1)] {
        let observations: Rc<RefCell<Observations>> = Rc::new(RefCell::new(Observations {
            status_failures: vec![(failed_status, report)],
            ..Observations::default()
        }));
        let trace: RefCell<EndpointTrace> = RefCell::default();
        let stop: StopNotification = StopNotification::new();
        let mut clock: TestClock = TestClock(Rc::new(RefCell::new(Time {
            current: Instant::now(),
            backwards_once: false,
            backwards_persistent: false,
        })));
        let mut factory: Factory = Factory {
            observations: Rc::clone(&observations),
            failure: None,
        };
        let acquire = || -> Result<TestEndpoint<'_>, NativeError> {
            Ok(TestEndpoint {
                observations: &observations,
                endpoints: &trace,
                failure: Some(close),
                accept: CANCELLABLE_ACCEPT,
                polls: 0,
                #[cfg(windows)]
                hint: None,
            })
        };
        let invocation: ServiceInvocation<Reporter, BrokerEndpointWorker<_, TestEndpoint<'_>>> =
            start_invocation(
                LabRole::ClockBroker,
                0,
                &mut factory,
                &mut HealthyTokenObserver(LabRole::ClockBroker),
                &mut clock,
                &stop,
                BrokerEndpointWorker::new(acquire),
            )
            .map_err(startup_error)?;
        // The scripted reservation holds no native resource. Its permanent
        // uncertainty must never be represented as resolved to run Drop.
        let mut invocation: ManuallyDrop<
            ServiceInvocation<Reporter, BrokerEndpointWorker<_, TestEndpoint<'_>>>,
        > = ManuallyDrop::new(invocation);
        if failed_status == 3 {
            assert!(matches!(
                invocation.poll(&mut clock, &stop),
                InvocationPoll::Pending(_)
            ));
            assert_eq!(stop.handle_control(1), ScmControl::Stop);
        }
        for _ in 0..6 {
            assert_eq!(
                invocation.poll(&mut clock, &stop),
                InvocationPoll::NativeDrain(Duration::from_millis(20))
            );
            assert_eq!(
                invocation.failure(),
                Some(&FixtureError::Lifecycle(LifecycleError::Report(report)))
            );
            assert_eq!(
                invocation.pending_cleanup_failure,
                Some(WorkerError::EndpointClose(close))
            );
            assert_eq!(invocation.outstanding_work(), retained_charge);
            assert!(invocation.admission_closed());
            assert!(invocation.take_receipt().is_none());
        }
        let records: super::DiagnosticRouteView = invocation.diagnostics.required();
        let observed: Vec<(u8, u8)> = records
            .records()
            .iter()
            .map(|record: &[u8; 9]| (record[2], record[4]))
            .collect();
        let expected: &[(u8, u8)] = if failed_status == 2 {
            &[(1, 0), (4, 10), (3, 0), (10, 23)]
        } else {
            &[(1, 0), (2, 0), (3, 0), (4, 10), (10, 23)]
        };
        assert_eq!(observed, expected);
        let status_codes: Vec<u32> = statuses(&observations)
            .iter()
            .map(|status: &[u32; 7]| status[1])
            .collect();
        assert_eq!(status_codes, [2, 4, 3]);
        let expected_calls: &[EndpointCall] = if failed_status == 2 {
            &[EndpointCall::Close(Some(3))]
        } else {
            &[
                EndpointCall::Begin(Some(4)),
                EndpointCall::Cancel(Some(3)),
                EndpointCall::Poll(Some(3)),
                EndpointCall::Close(Some(3)),
            ]
        };
        assert_eq!(trace.borrow().calls, expected_calls);
    }
    Ok(())
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
            #[cfg(windows)]
            hint: None,
        })
    };
    let mut worker: BrokerEndpointWorker<_, TestEndpoint<'_>> = BrokerEndpointWorker::new(acquire);
    assert_eq!(worker.poll_finished(&wake), WorkerFinish::Pending);
    assert_eq!(worker.poll_prepare(&wake), Poll::Ready(Ok(())));
    assert_eq!(worker.activate(Instant::now()), Ok(()));
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

/// Independent operation outcomes for a fixture endpoint with a read slot.
struct ArmedEndpoint<'a> {
    /// The existing reservation owns the accept, status observations and release.
    endpoint: TestEndpoint<'a>,
    /// Scripted result from the same submitted read.
    read_start: FirstReadStart,
    /// Enable prefill and script currentness of its retained peer.
    prefill_peer: Option<&'a dyn Fn() -> bool>,
    /// A deterministic change during marker preparation, before its first guard.
    before_prefill_guard: Option<&'a dyn Fn()>,
    /// A deterministic control or clock change while prefill is settling.
    before_read_submit: Option<&'a dyn Fn()>,
    /// A deterministic control or clock change during submission.
    on_read_begin: Option<&'a dyn Fn()>,
    /// Independent result of cancelling the original read.
    read_cancel: Result<(), NativeError>,
    /// Exact successive read observations, with surplus polls remaining charged.
    reads: &'a [Result<Poll<FirstReadCompletion>, NativeError>],
    /// Number of read observations already collected.
    read_polls: usize,
    /// Optional reply observations on this exact reservation.
    reply: Option<ReplyScript<'a>>,
}

/// Independent reply submission, cancellation and completion observations.
struct ReplyScript<'a> {
    /// Whether the write queues or reports inline completion.
    immediate: bool,
    /// Control delivered synchronously during submission.
    stop_on_submit: Option<&'a StopNotification>,
    /// Control delivered once after the selected peer observation samples current authority.
    stop_on_peer_check: Option<(&'a StopNotification, usize)>,
    /// A time-change notice delivered by the selected peer observation.
    change_on_peer_check: Option<(&'static BrokerClockChanges, &'a StopNotification, usize)>,
    /// Deliver a time-change notice during collection of the complete write count.
    change_on_write_completion: Option<(&'static BrokerClockChanges, &'a StopNotification)>,
    /// Deliver a time-change notice on the first pending write observation.
    change_on_pending_write: Option<(&'static BrokerClockChanges, &'a StopNotification)>,
    /// Authority of the original peer, independent of the mailbox, cutoff and pipe.
    peer_current: &'a Cell<bool>,
    /// Withdraw during the physical terminal observation, before publication.
    withdraw_on_completion: bool,
    /// Cancellation result, independent of physical completion.
    cancel: Result<(), NativeError>,
    /// Separate terminal observations for the original operation.
    polls: std::array::IntoIter<Result<Poll<FirstReplyCompletion>, NativeError>, 3>,
}

/// The portable worker receives bytes already admitted by the host seam.
const SCRIPTED_REPLY: &[u8] = b"admitted fixture reply";

impl<'a> ArmedEndpoint<'a> {
    /// Script a queued accept that connects before the independent read history.
    fn new(
        observations: &'a Rc<RefCell<Observations>>,
        trace: &'a RefCell<EndpointTrace>,
        read_start: FirstReadStart,
        read_cancel: Result<(), NativeError>,
        reads: &'a [Result<Poll<FirstReadCompletion>, NativeError>],
    ) -> Self {
        Self {
            endpoint: TestEndpoint {
                observations,
                endpoints: trace,
                failure: None,
                accept: AcceptScript {
                    polls: &[Ok(Poll::Ready(AcceptCompletion::Connected))],
                    ..CANCELLABLE_ACCEPT
                },
                polls: 0,
                #[cfg(windows)]
                hint: None,
            },
            read_start,
            prefill_peer: None,
            before_prefill_guard: None,
            before_read_submit: None,
            on_read_begin: None,
            read_cancel,
            reads,
            read_polls: 0,
            reply: None,
        }
    }
}

impl Reservation for ArmedEndpoint<'_> {
    /// Submit the original accept under the acknowledged Running permit.
    fn begin_accept(&mut self) -> Result<AcceptStart, NativeError> {
        self.endpoint.begin_accept()
    }

    /// Target cancellation at the accept.
    fn request_accept_cancel(&mut self) -> Result<(), NativeError> {
        self.endpoint.request_accept_cancel()
    }

    /// Retain the separately scripted accept result.
    fn poll_accept(&mut self) -> Result<Poll<AcceptCompletion>, NativeError> {
        self.endpoint.poll_accept()
    }

    /// Record the call under the actual current SCM state.
    fn begin_first_read<C: Clock>(
        &mut self,
        connection: &mut RunningConnection<'_, C>,
    ) -> Result<FirstReadStart, NativeError> {
        if let Some(observe_peer) = self.prefill_peer {
            if let Some(action) = self.before_prefill_guard {
                action();
            }
            match connection.admit_prefill_marker(|| {
                self.endpoint.record(EndpointCall::ObservePrefillPeer);
                observe_peer()
            }) {
                Ok(()) => {
                    self.endpoint.record(EndpointCall::BeginPrefillMarker);
                    self.endpoint.record(EndpointCall::PollPrefillMarker);
                }
                Err(PrefillRefusal::ConnectionClosed) => return Ok(FirstReadStart::Refused),
                Err(PrefillRefusal::ForeignPeer) => {
                    return Err(NativeError::new(
                        NativeOperation::BrokerEndpointReplySubmit,
                        87,
                    ));
                }
            }
        }
        if let Some(action) = self.before_read_submit {
            action();
        }
        if !connection.first_read_is_current() {
            return Ok(FirstReadStart::Refused);
        }
        self.endpoint.record(EndpointCall::BeginRead);
        if let Some(action) = self.on_read_begin {
            action();
        }
        Ok(self.read_start)
    }

    /// Record collection without reporting progress or advancing time.
    fn poll_first_read(&mut self) -> Result<Poll<FirstReadCompletion>, NativeError> {
        self.endpoint.record(EndpointCall::PollRead);
        let result: Result<Poll<FirstReadCompletion>, NativeError> = self
            .reads
            .get(self.read_polls)
            .copied()
            .unwrap_or(Ok(Poll::Pending));
        self.read_polls += 1;
        result
    }

    /// Keep cancellation and completion observable as separate events.
    fn request_first_read_cancel(&mut self) -> Result<(), NativeError> {
        self.endpoint.record(EndpointCall::CancelRead);
        self.read_cancel
    }

    /// Sample the same peer's authority independently of any pipe or service state.
    /// # Panics
    /// Fails if the selected Stop control is not recognized.
    fn reply_peer_current(&mut self) -> bool {
        let current: bool = self
            .reply
            .as_ref()
            .is_some_and(|reply: &ReplyScript<'_>| reply.peer_current.get());
        let prior_checks: usize = self.endpoint.endpoints.borrow().reply_peer_checks.len();
        if let Some((stop, _)) = self.reply.as_mut().and_then(|reply: &mut ReplyScript<'_>| {
            reply
                .stop_on_peer_check
                .take_if(|(_, previous): &mut (&StopNotification, usize)| *previous == prior_checks)
        }) {
            assert_eq!(stop.handle_control(1), ScmControl::Stop);
        }
        if let Some((changes, stop, _)) =
            self.reply.as_mut().and_then(|reply: &mut ReplyScript<'_>| {
                reply.change_on_peer_check.take_if(
                    |(_, _, previous): &mut (&BrokerClockChanges, &StopNotification, usize)| {
                        *previous == prior_checks
                    },
                )
            })
        {
            let disposition: ControlDisposition = deliver(Some(changes), stop, 0x10, 0);
            assert_eq!(disposition.status, 0);
            assert!(disposition.wake);
        }
        self.endpoint
            .endpoints
            .borrow_mut()
            .reply_peer_checks
            .push(current);
        current
    }

    /// Recheck after preparing the slot, then record the write and its exact bytes.
    ///
    /// # Panics
    /// Panics if the worker substitutes different reply bytes.
    fn begin_first_reply<C: Clock>(
        &mut self,
        bytes: &[u8],
        connection: &mut RunningConnection<'_, C>,
        cutoff: Instant,
        positive_revision: Option<ReplyRevision>,
    ) -> Result<FirstReplyStart, NativeError> {
        assert_eq!(bytes, SCRIPTED_REPLY);
        if !matches!(
            connection
                .reply_edge_with_revision(cutoff, positive_revision, || self.reply_peer_current()),
            ConnectionEdge::Open(_)
        ) {
            return Ok(FirstReplyStart::Refused);
        }
        self.endpoint.record(EndpointCall::BeginReply);
        let Some(reply) = &self.reply else {
            return Ok(FirstReplyStart::Unarmed);
        };
        if let Some(stop) = reply.stop_on_submit {
            assert_eq!(stop.handle_control(1), ScmControl::Stop);
        }
        Ok(if reply.immediate {
            FirstReplyStart::Immediate
        } else {
            FirstReplyStart::Queued
        })
    }

    /// Observe the one write without committing its result in the fixture.
    /// # Panics
    /// Fails if the selected time-change control is not recognized.
    fn poll_first_reply(&mut self) -> Result<Poll<FirstReplyCompletion>, NativeError> {
        self.endpoint.record(EndpointCall::PollReply);
        let Some(reply) = &mut self.reply else {
            return Err(REPLY_OBSERVATION_REFUSED);
        };
        let result: Result<Poll<FirstReplyCompletion>, NativeError> =
            reply.polls.next().unwrap_or(Ok(Poll::Pending));
        if reply.withdraw_on_completion && matches!(result, Ok(Poll::Ready(_))) {
            reply.peer_current.set(false);
        }
        let change: Option<(&BrokerClockChanges, &StopNotification)> = match result {
            Ok(Poll::Pending) => reply.change_on_pending_write.take(),
            Ok(Poll::Ready(FirstReplyCompletion::Written)) => {
                reply.change_on_write_completion.take()
            }
            _ => None,
        };
        if let Some((changes, stop)) = change {
            let disposition: ControlDisposition = deliver(Some(changes), stop, 0x10, 0);
            assert_eq!(disposition.status, 0);
            assert!(disposition.wake);
        }
        self.endpoint
            .endpoints
            .borrow_mut()
            .reply_results
            .push(result);
        result
    }

    /// Record cancellation without treating its acknowledgement as completion.
    fn request_first_reply_cancel(&mut self) -> Result<(), NativeError> {
        self.endpoint.record(EndpointCall::CancelReply);
        self.reply
            .as_ref()
            .map_or(Err(REPLY_CANCEL_REFUSED), |reply: &ReplyScript<'_>| {
                reply.cancel
            })
    }

    /// Release the original reservation after the worker settles.
    fn close(self) -> Result<(), NativeError> {
        self.endpoint.close()
    }
}

/// Observe the real host through both asynchronous and inline read submission.
///
/// # Errors
/// Returns the original startup refusal.
///
/// # Panics
/// Panics if accept and read do not each complete once under the same Running permit.
#[test]
fn broker_first_read_uses_the_original_running_permit() -> Result<(), FixtureError> {
    for start in [FirstReadStart::Queued, FirstReadStart::Immediate] {
        let observations: Rc<RefCell<Observations>> = Rc::default();
        let trace: RefCell<EndpointTrace> = RefCell::default();
        let stop: StopNotification = StopNotification::new();
        let mut clock: TestClock = fixed_accept_clock();
        let mut factory: Factory = Factory {
            observations: Rc::clone(&observations),
            failure: None,
        };
        let acquire = || -> Result<ArmedEndpoint<'_>, NativeError> {
            Ok(ArmedEndpoint::new(
                &observations,
                &trace,
                start,
                Ok(()),
                &[Ok(Poll::Ready(FirstReadCompletion::Settled))],
            ))
        };
        let mut invocation: ServiceInvocation<
            Reporter,
            BrokerEndpointWorker<_, ArmedEndpoint<'_>>,
        > = start_invocation(
            LabRole::ClockBroker,
            0,
            &mut factory,
            &mut HealthyTokenObserver(LabRole::ClockBroker),
            &mut clock,
            &stop,
            BrokerEndpointWorker::new(acquire),
        )
        .map_err(startup_error)?;
        let no_calls: [EndpointCall; 0] = [];
        assert_eq!(trace.borrow().calls, no_calls);
        assert_eq!(
            invocation.poll(&mut clock, &stop),
            InvocationPoll::Pending(Duration::from_secs(5))
        );
        clock.0.borrow_mut().current += Duration::from_secs(1);
        let accepted: InvocationPoll = invocation.poll(&mut clock, &stop);
        if start == FirstReadStart::Queued {
            assert_eq!(accepted, InvocationPoll::Pending(Duration::from_secs(4)));
            assert_eq!(trace.borrow().calls.len(), 3);
            assert_eq!(
                invocation.poll(&mut clock, &stop),
                InvocationPoll::Pending(Duration::from_secs(59))
            );
        } else {
            assert_eq!(accepted, InvocationPoll::Pending(Duration::from_secs(59)));
        }
        assert_eq!(invocation.outstanding_work(), 1);
        assert_eq!(stop.handle_control(1), ScmControl::Stop);
        assert_eq!(invocation.poll(&mut clock, &stop), InvocationPoll::Finished);
        assert!(invocation.failure().is_none());
        assert_eq!(invocation.outstanding_work(), 0);
        assert_eq!(
            trace.borrow().calls,
            [
                EndpointCall::Begin(Some(4)),
                EndpointCall::Poll(Some(4)),
                EndpointCall::BeginRead(Some(4)),
                EndpointCall::PollRead(Some(4)),
                EndpointCall::Close(Some(3)),
            ]
        );
    }
    Ok(())
}

/// Initiate cancellation at the original cutoff and keep the charge through Stop.
///
/// # Errors
/// Returns the original startup refusal.
///
/// # Panics
/// Panics if Stop retries cancellation, loses a failure or closes before read settlement.
#[test]
fn broker_first_read_cutoff_and_stop_retain_the_original_operation() -> Result<(), FixtureError> {
    let cancel_error: NativeError = NativeError::new(NativeOperation::BrokerEndpointReadCancel, 5);
    let poll_error: NativeError =
        NativeError::new(NativeOperation::BrokerEndpointReadCompletion, 6);
    for cancel in [Ok(()), Err(cancel_error)] {
        let observations: Rc<RefCell<Observations>> = Rc::default();
        let trace: RefCell<EndpointTrace> = RefCell::default();
        let stop: StopNotification = StopNotification::new();
        let mut clock: TestClock = fixed_accept_clock();
        let mut factory: Factory = Factory {
            observations: Rc::clone(&observations),
            failure: None,
        };
        let reads: [Result<Poll<FirstReadCompletion>, NativeError>; 2] = if cancel.is_ok() {
            [
                Ok(Poll::Pending),
                Ok(Poll::Ready(FirstReadCompletion::Settled)),
            ]
        } else {
            [
                Err(poll_error),
                Ok(Poll::Ready(FirstReadCompletion::Invalid(poll_error))),
            ]
        };
        let acquire = || -> Result<ArmedEndpoint<'_>, NativeError> {
            Ok(ArmedEndpoint::new(
                &observations,
                &trace,
                FirstReadStart::Queued,
                cancel,
                &reads,
            ))
        };
        let mut invocation: ServiceInvocation<
            Reporter,
            BrokerEndpointWorker<_, ArmedEndpoint<'_>>,
        > = start_invocation(
            LabRole::ClockBroker,
            0,
            &mut factory,
            &mut HealthyTokenObserver(LabRole::ClockBroker),
            &mut clock,
            &stop,
            BrokerEndpointWorker::new(acquire),
        )
        .map_err(startup_error)?;
        assert_eq!(
            invocation.poll(&mut clock, &stop),
            InvocationPoll::Pending(Duration::from_secs(5))
        );
        assert_eq!(
            invocation.poll(&mut clock, &stop),
            InvocationPoll::Pending(Duration::from_secs(5))
        );
        clock.0.borrow_mut().current += Duration::from_secs(5);
        let at_cutoff: InvocationPoll = invocation.poll(&mut clock, &stop);
        let drain: InvocationPoll = InvocationPoll::NativeDrain(Duration::from_millis(20));
        if cancel.is_ok() {
            assert_eq!(
                at_cutoff,
                InvocationPoll::Pending(Duration::from_millis(20))
            );
            assert_eq!(stop.handle_control(1), ScmControl::Stop);
            assert_eq!(invocation.poll(&mut clock, &stop), drain);
        } else {
            assert_eq!(at_cutoff, drain);
        }
        assert_eq!(invocation.state(), Some(ServiceState::StopPending));
        assert_eq!(invocation.outstanding_work(), 1);
        let expected: Option<WorkerError> = cancel.err().map(WorkerError::EndpointReadCleanup);
        assert_eq!(invocation.pending_cleanup_failure, expected);
        assert_eq!(invocation.poll(&mut clock, &stop), InvocationPoll::Finished);
        assert_eq!(invocation.outstanding_work(), 0);
        assert_eq!(
            invocation.failure(),
            expected.map(FixtureError::Worker).as_ref()
        );
        assert_eq!(
            trace.borrow().calls,
            [
                EndpointCall::Begin(Some(4)),
                EndpointCall::Poll(Some(4)),
                EndpointCall::BeginRead(Some(4)),
                EndpointCall::CancelRead(Some(4)),
                EndpointCall::PollRead(Some(3)),
                EndpointCall::PollRead(Some(3)),
                EndpointCall::Close(Some(3)),
            ]
        );
    }
    Ok(())
}

/// Inline success is collected once even if Stop or equality closes admission.
///
/// # Errors
/// Returns the original startup refusal.
///
/// # Panics
/// Panics if a completed read receives cancellation or early release.
#[test]
fn broker_inline_first_read_collects_after_stop_or_cutoff_without_cancelling()
-> Result<(), FixtureError> {
    inline_read_history(false, false)?;
    inline_read_history(true, false)?;
    inline_read_history(true, true)
}

/// Exercise the selected inline completion result at the original cutoff or during Stop.
/// # Errors
/// Returns a refusal from starting the service fixture.
/// # Panics
/// Panics if completion is cancelled or custody releases early.
fn inline_read_history(stop_during_read: bool, ambiguous: bool) -> Result<(), FixtureError> {
    let reads: &[Result<Poll<FirstReadCompletion>, NativeError>] =
        inline_read_observations(ambiguous);
    let observations: Rc<RefCell<Observations>> = Rc::default();
    let trace: RefCell<EndpointTrace> = RefCell::default();
    let stop: StopNotification = StopNotification::new();
    let mut clock: TestClock = fixed_accept_clock();
    let time: Rc<RefCell<Time>> = Rc::clone(&clock.0);
    let mut factory: Factory = Factory {
        observations: Rc::clone(&observations),
        failure: None,
    };
    let during_read = || {
        if stop_during_read {
            assert_eq!(stop.handle_control(1), ScmControl::Stop);
        } else {
            time.borrow_mut().current += Duration::from_secs(5);
        }
    };
    let acquire = || -> Result<ArmedEndpoint<'_>, NativeError> {
        Ok(ArmedEndpoint {
            on_read_begin: Some(&during_read),
            ..ArmedEndpoint::new(
                &observations,
                &trace,
                FirstReadStart::Immediate,
                Err(NativeError::new(
                    NativeOperation::BrokerEndpointReadCancel,
                    5,
                )),
                reads,
            )
        })
    };
    let mut invocation: ServiceInvocation<Reporter, BrokerEndpointWorker<_, ArmedEndpoint<'_>>> =
        start_invocation(
            LabRole::ClockBroker,
            0,
            &mut factory,
            &mut HealthyTokenObserver(LabRole::ClockBroker),
            &mut clock,
            &stop,
            BrokerEndpointWorker::new(acquire),
        )
        .map_err(startup_error)?;
    assert_eq!(
        invocation.poll(&mut clock, &stop),
        InvocationPoll::Pending(Duration::from_secs(5))
    );
    let settled: InvocationPoll = invocation.poll(&mut clock, &stop);
    let mut calls: Vec<EndpointCall> = vec![
        EndpointCall::Begin(Some(4)),
        EndpointCall::Poll(Some(4)),
        EndpointCall::BeginRead(Some(4)),
        EndpointCall::PollRead(Some(4)),
    ];
    let expected: Option<FixtureError> = if ambiguous {
        assert_eq!(
            settled,
            InvocationPoll::NativeDrain(Duration::from_millis(20))
        );
        assert_eq!(invocation.state(), Some(ServiceState::StopPending));
        assert_eq!(invocation.outstanding_work(), 1);
        let first_error: FixtureError =
            FixtureError::Worker(WorkerError::EndpointRead(INLINE_READ_FIRST_ERROR));
        assert_eq!(invocation.failure(), Some(&first_error));
        assert_eq!(
            invocation.pending_cleanup_failure,
            Some(WorkerError::EndpointReadCleanup(INLINE_READ_LATER_ERROR))
        );
        calls.push(EndpointCall::PollRead(Some(3)));
        assert_eq!(trace.borrow().calls, calls);
        assert_eq!(invocation.poll(&mut clock, &stop), InvocationPoll::Finished);
        calls.push(EndpointCall::PollRead(Some(3)));
        Some(first_error)
    } else {
        if stop_during_read {
            assert_eq!(settled, InvocationPoll::Finished);
        } else {
            assert_eq!(settled, InvocationPoll::Pending(Duration::from_secs(55)));
            assert_eq!(invocation.state(), Some(ServiceState::Running));
            assert_eq!(invocation.outstanding_work(), 1);
            assert_eq!(trace.borrow().calls, calls);
            assert_eq!(stop.handle_control(1), ScmControl::Stop);
            assert_eq!(invocation.poll(&mut clock, &stop), InvocationPoll::Finished);
        }
        None
    };
    assert_eq!(invocation.outstanding_work(), 0);
    assert_eq!(invocation.failure(), expected.as_ref());
    calls.push(EndpointCall::Close(Some(3)));
    assert_eq!(trace.borrow().calls, calls);
    Ok(())
}

/// Stop during prefill must leave the original read slot unused.
/// # Errors
/// Returns a refusal from starting the service fixture.
#[test]
fn broker_first_read_refuses_stop_after_prefill() -> Result<(), FixtureError> {
    refused_post_prefill_read(true)
}

/// The original cutoff during prefill must leave the original read slot unused.
/// # Errors
/// Returns a refusal from starting the service fixture.
#[test]
fn broker_first_read_refuses_original_cutoff_after_prefill() -> Result<(), FixtureError> {
    refused_post_prefill_read(false)
}

/// Compare Stop against connection expiry after prefill, before any read has started.
/// # Errors
/// Returns a refusal from starting the service fixture.
/// # Panics
/// Panics if a read starts or the reservation releases before the original Stop.
fn refused_post_prefill_read(stop_during_prefill: bool) -> Result<(), FixtureError> {
    let observations: Rc<RefCell<Observations>> = Rc::default();
    let trace: RefCell<EndpointTrace> = RefCell::default();
    let stop: StopNotification = StopNotification::new();
    let mut clock: TestClock = fixed_accept_clock();
    let time: Rc<RefCell<Time>> = Rc::clone(&clock.0);
    let prefill_observed: Cell<bool> = Cell::new(false);
    let mut factory: Factory = Factory {
        observations: Rc::clone(&observations),
        failure: None,
    };
    let prefill = || {
        assert!(!prefill_observed.replace(true));
        if stop_during_prefill {
            assert_eq!(stop.handle_control(1), ScmControl::Stop);
        } else {
            time.borrow_mut().current += Duration::from_secs(5);
        }
    };
    let acquire = || -> Result<ArmedEndpoint<'_>, NativeError> {
        Ok(ArmedEndpoint {
            before_read_submit: Some(&prefill),
            ..ArmedEndpoint::new(
                &observations,
                &trace,
                FirstReadStart::Immediate,
                Ok(()),
                &[Ok(Poll::Ready(FirstReadCompletion::Settled))],
            )
        })
    };
    let mut invocation: ServiceInvocation<Reporter, BrokerEndpointWorker<_, ArmedEndpoint<'_>>> =
        start_invocation(
            LabRole::ClockBroker,
            0,
            &mut factory,
            &mut HealthyTokenObserver(LabRole::ClockBroker),
            &mut clock,
            &stop,
            BrokerEndpointWorker::new(acquire),
        )
        .map_err(startup_error)?;
    let accept_queued: InvocationPoll = invocation.poll(&mut clock, &stop);
    let after_prefill: InvocationPoll = invocation.poll(&mut clock, &stop);
    if stop_during_prefill {
        assert_eq!(after_prefill, InvocationPoll::Finished);
    } else {
        let state: Option<ServiceState> = invocation.state();
        let work: u32 = invocation.outstanding_work();
        let delivered: ScmControl = stop.handle_control(1);
        let finished: InvocationPoll = invocation.poll(&mut clock, &stop);
        assert_eq!(finished, InvocationPoll::Finished);
        assert_eq!(delivered, ScmControl::Stop);
        assert_eq!(
            after_prefill,
            InvocationPoll::Pending(Duration::from_secs(55))
        );
        assert_eq!(state, Some(ServiceState::Running));
        assert_eq!(work, 1);
    }
    assert_eq!(
        accept_queued,
        InvocationPoll::Pending(Duration::from_secs(5))
    );
    assert!(prefill_observed.get());
    assert_eq!(invocation.outstanding_work(), 0);
    assert!(invocation.failure().is_none());
    assert_eq!(
        trace.borrow().calls,
        [
            EndpointCall::Begin(Some(4)),
            EndpointCall::Poll(Some(4)),
            EndpointCall::Close(Some(3)),
        ]
    );
    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum PrefillHistory {
    Current,
    StopBeforeGuard,
    OriginalCutoffBeforeGuard,
    Stop,
    OriginalCutoff,
    ForeignPeer,
}

/// Specify the invocation state after the marker admission attempt returns.
fn expected_prefill_outcome(
    history: PrefillHistory,
) -> (InvocationPoll, Option<ServiceState>, u32) {
    match history {
        PrefillHistory::Current => (
            InvocationPoll::Pending(Duration::from_secs(60)),
            Some(ServiceState::Running),
            1,
        ),
        PrefillHistory::StopBeforeGuard | PrefillHistory::Stop | PrefillHistory::ForeignPeer => {
            (InvocationPoll::Finished, Some(ServiceState::Stopped), 0)
        }
        PrefillHistory::OriginalCutoffBeforeGuard | PrefillHistory::OriginalCutoff => (
            InvocationPoll::Pending(Duration::from_secs(55)),
            Some(ServiceState::Running),
            1,
        ),
    }
}

/// Withdraw at the independently selected preparation or peer-observation edge.
fn withdraw_prefill_at(
    history: PrefillHistory,
    before_guard: bool,
    stop: &StopNotification,
    delivered_stop: &Cell<Option<ScmControl>>,
    time: &RefCell<Time>,
    cutoff: Option<Instant>,
) {
    match (history, before_guard) {
        (PrefillHistory::StopBeforeGuard, true) | (PrefillHistory::Stop, false) => {
            delivered_stop.set(Some(stop.handle_control(1)));
        }
        (PrefillHistory::OriginalCutoffBeforeGuard, true)
        | (PrefillHistory::OriginalCutoff, false) => {
            if let Some(cutoff) = cutoff {
                time.borrow_mut().current = cutoff;
            }
        }
        _ => {}
    }
}

/// Recheck Stop, the original cutoff and peer identity before submitting a marker.
///
/// # Errors
/// Returns a refusal from starting the service fixture.
///
/// # Panics
/// Panics if a refused marker starts, or the original worker or its permit is lost.
#[test]
fn broker_prefill_marker_rechecks_stop_and_cutoff_after_peer_observation()
-> Result<(), FixtureError> {
    for history in [
        PrefillHistory::Current,
        PrefillHistory::StopBeforeGuard,
        PrefillHistory::OriginalCutoffBeforeGuard,
        PrefillHistory::Stop,
        PrefillHistory::OriginalCutoff,
        PrefillHistory::ForeignPeer,
    ] {
        check_prefill_marker_history(history)?;
    }
    Ok(())
}

/// Exercise one marker admission history through the original worker and host.
///
/// # Errors
/// Returns a refusal from starting the service fixture.
///
/// # Panics
/// Panics if refused work starts or the original cutoff, permit or retirement is lost.
fn check_prefill_marker_history(history: PrefillHistory) -> Result<(), FixtureError> {
    let observations: Rc<RefCell<Observations>> = Rc::default();
    let trace: RefCell<EndpointTrace> = RefCell::default();
    let stop: StopNotification = StopNotification::new();
    let delivered_stop: Cell<Option<ScmControl>> = Cell::new(None);
    let mut clock: TestClock = fixed_accept_clock();
    let time: Rc<RefCell<Time>> = Rc::clone(&clock.0);
    let anchor: Instant = time.borrow().current;
    let original_cutoff: Cell<Option<Instant>> = Cell::new(None);
    let mut factory: Factory = Factory {
        observations: Rc::clone(&observations),
        failure: None,
    };
    let change_at_boundary = |before_guard: bool| {
        withdraw_prefill_at(
            history,
            before_guard,
            &stop,
            &delivered_stop,
            &time,
            original_cutoff.get(),
        );
    };
    let before_prefill_guard = || change_at_boundary(true);
    let observe_peer = || -> bool {
        change_at_boundary(false);
        !matches!(history, PrefillHistory::ForeignPeer)
    };
    let acquire = || -> Result<ArmedEndpoint<'_>, NativeError> {
        Ok(ArmedEndpoint {
            prefill_peer: Some(&observe_peer),
            before_prefill_guard: Some(&before_prefill_guard),
            ..ArmedEndpoint::new(
                &observations,
                &trace,
                FirstReadStart::Immediate,
                Ok(()),
                &[Ok(Poll::Ready(FirstReadCompletion::Settled))],
            )
        })
    };
    let mut invocation: ServiceInvocation<Reporter, BrokerEndpointWorker<_, ArmedEndpoint<'_>>> =
        start_invocation(
            LabRole::ClockBroker,
            0,
            &mut factory,
            &mut HealthyTokenObserver(LabRole::ClockBroker),
            &mut clock,
            &stop,
            BrokerEndpointWorker::new(acquire),
        )
        .map_err(startup_error)?;
    let accept_queued: InvocationPoll = invocation.poll(&mut clock, &stop);
    assert_eq!(
        invocation.state(),
        Some(ServiceState::Running),
        "{history:?}"
    );
    assert_eq!(invocation.outstanding_work(), 1, "{history:?}");
    original_cutoff.set(invocation.connection_cutoff);
    let after_admission: InvocationPoll = invocation.poll(&mut clock, &stop);
    let state: Option<ServiceState> = invocation.state();
    let work: u32 = invocation.outstanding_work();
    let cutoff_after_admission: Option<Instant> = invocation.connection_cutoff;
    let failure: Option<FixtureError> = matches!(history, PrefillHistory::ForeignPeer).then_some(
        FixtureError::Worker(WorkerError::EndpointRead(NativeError::new(
            NativeOperation::BrokerEndpointReplySubmit,
            87,
        ))),
    );
    assert_eq!(invocation.failure(), failure.as_ref(), "{history:?}");
    let delivered: ScmControl = delivered_stop
        .get()
        .unwrap_or_else(|| stop.handle_control(1));
    let retired: InvocationPoll = invocation.poll(&mut clock, &stop);

    assert_eq!(retired, InvocationPoll::Finished, "{history:?}");
    assert_eq!(delivered, ScmControl::Stop);
    assert_eq!(invocation.outstanding_work(), 0);
    assert_eq!(invocation.failure(), failure.as_ref(), "{history:?}");
    assert_eq!(
        (after_admission, state, work),
        expected_prefill_outcome(history),
        "{history:?}"
    );
    assert_eq!(
        accept_queued,
        InvocationPoll::Pending(Duration::from_secs(5))
    );
    assert_eq!(original_cutoff.get(), Some(anchor + Duration::from_secs(5)));
    assert_eq!(cutoff_after_admission, original_cutoff.get());

    let calls: Vec<EndpointCall> = prefill_marker_calls(history);
    assert_eq!(trace.borrow().calls, calls, "{history:?}");
    Ok(())
}

/// Expected owned operations from a current or refused prefill followed by Stop.
fn prefill_marker_calls(history: PrefillHistory) -> Vec<EndpointCall> {
    let mut calls: Vec<EndpointCall> =
        vec![EndpointCall::Begin(Some(4)), EndpointCall::Poll(Some(4))];
    if !matches!(
        history,
        PrefillHistory::StopBeforeGuard | PrefillHistory::OriginalCutoffBeforeGuard
    ) {
        calls.push(EndpointCall::ObservePrefillPeer(Some(4)));
    }
    if matches!(history, PrefillHistory::Current) {
        calls.extend([
            EndpointCall::BeginPrefillMarker(Some(4)),
            EndpointCall::PollPrefillMarker(Some(4)),
            EndpointCall::BeginRead(Some(4)),
            EndpointCall::PollRead(Some(4)),
        ]);
    }
    calls.push(EndpointCall::Close(Some(3)));
    calls
}

/// The first ambiguous result is observed while Running.
const INLINE_READ_FIRST_ERROR: NativeError =
    NativeError::new(NativeOperation::BrokerEndpointReadCompletion, 6);

/// A distinct later refusal is observed during physical retirement.
const INLINE_READ_LATER_ERROR: NativeError =
    NativeError::new(NativeOperation::BrokerEndpointReadCompletion, 13);

/// Supply known terminal output or two independently specified refusals before it.
fn inline_read_observations(
    ambiguous: bool,
) -> &'static [Result<Poll<FirstReadCompletion>, NativeError>] {
    if ambiguous {
        &[
            Err(INLINE_READ_FIRST_ERROR),
            Err(INLINE_READ_LATER_ERROR),
            Ok(Poll::Ready(FirstReadCompletion::Settled)),
        ]
    } else {
        &[Ok(Poll::Ready(FirstReadCompletion::Settled))]
    }
}

/// An independent refusal from requesting cancellation of the original write.
const REPLY_CANCEL_REFUSED: NativeError =
    NativeError::new(NativeOperation::BrokerEndpointReplyCancel, 5);

/// A distinct write observation failure that does not confirm completion.
const REPLY_OBSERVATION_REFUSED: NativeError =
    NativeError::new(NativeOperation::BrokerEndpointReplyCompletion, 6);

/// A fabricated terminal short count, distinct from unconfirmed completion.
const REPLY_INVALID_COUNT: NativeError =
    NativeError::new(NativeOperation::BrokerEndpointReplyCompletion, 87);

/// Boundary and native return histories selected before exercising the worker.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReplyHistory {
    /// Equality at the original cutoff fences submission.
    ExpiredBeforeSubmit,
    /// The original broker mailbox is retired before the write.
    MailboxClosedBeforeSubmit,
    /// Only the original peer withdraws before the already decided reply is submitted.
    PeerWithdrawnBeforeSubmit,
    /// The native write settled with an invalid complete count.
    InvalidInlineCount,
    /// A healthy inline result spends the one submission.
    LiveInline,
    /// A current peer lets the queued reply complete through the original Running owner.
    LiveQueued,
    /// A stable positive revision lets the queued reply confirm.
    PositiveQueued,
    /// A new revision arrives after the write queues, before the worker returns.
    RevisionChangedAfterQueue,
    /// A new revision arrives between two Running turns.
    RevisionChangedBetweenPolls,
    /// A new revision arrives during a pending write observation.
    RevisionChangedDuringPending,
    /// The completed queued write retires its original positive revision during collection.
    RevisionChangedDuringWriteCompletion,
    /// The final peer check after an inline write retires its original positive revision.
    RevisionChangedAfterInlineWrite,
    /// A time change during completion does not invalidate an unavailable reply.
    UnavailableDuringWriteCompletion,
    /// A Stop delivered during inline submission still permits collection.
    InlineStop,
    /// The peer returns current after delivering Stop, before any write is submitted.
    StopDuringPeerCheck,
    /// The initial peer observation succeeds and Stop arrives during final admission.
    StopDuringFinalPeerCheck,
    /// The mailbox stays live but the original positive revision retires before submission.
    RevisionChangedDuringFinalPeerCheck,
    /// Stop cancels a queued write without freeing the permit.
    QueuedStop,
    /// Windows confirms cancellation of the original write after Stop.
    QueuedCancellation,
    /// The original cutoff cancels once before Stop retires the write.
    QueuedExpiry,
    /// A cancellation refusal precedes an ambiguous completion observation.
    QueuedCancelFailure,
    /// Withdrawal while queued leads to a late physical write without confirmation.
    PeerWithdrawnQueued,
    /// The exact write completes but the peer has withdrawn before publication.
    PeerWithdrawnOnCompletion,
}

/// One Running observation, checked after the original invocation has stopped.
struct ReplyObservation {
    /// The observed wait and write confirmation.
    actual: (ConnectionWait, bool),
    /// The independently scripted wait and write confirmation.
    expected: (ConnectionWait, bool),
}

impl ReplyHistory {
    /// State the independent publication outcome for this history.
    const fn confirms(self) -> bool {
        matches!(
            self,
            Self::LiveInline
                | Self::LiveQueued
                | Self::PositiveQueued
                | Self::UnavailableDuringWriteCompletion
        )
    }

    /// Select histories that withdraw a positive revision before terminal collection.
    const fn retires_queued_revision(self) -> bool {
        matches!(
            self,
            Self::RevisionChangedAfterQueue
                | Self::RevisionChangedBetweenPolls
                | Self::RevisionChangedDuringPending
        )
    }

    /// Select the queued replies driven to completion while the service remains Running.
    const fn collects_while_running(self) -> bool {
        self.retires_queued_revision()
            || matches!(
                self,
                Self::LiveQueued
                    | Self::PositiveQueued
                    | Self::RevisionChangedDuringWriteCompletion
                    | Self::UnavailableDuringWriteCompletion
                    | Self::PeerWithdrawnQueued
                    | Self::PeerWithdrawnOnCompletion
            )
    }

    /// Supply physical outcomes independently of the worker's state.
    fn script<'a>(
        self,
        stop: &'a StopNotification,
        peer_current: &'a Cell<bool>,
        changes: &'static BrokerClockChanges,
    ) -> ReplyScript<'a> {
        let immediate: bool = matches!(
            self,
            Self::ExpiredBeforeSubmit
                | Self::MailboxClosedBeforeSubmit
                | Self::InvalidInlineCount
                | Self::LiveInline
                | Self::InlineStop
                | Self::StopDuringPeerCheck
                | Self::StopDuringFinalPeerCheck
                | Self::RevisionChangedDuringFinalPeerCheck
                | Self::RevisionChangedAfterInlineWrite
        );
        let first: Result<Poll<FirstReplyCompletion>, NativeError> = match self {
            Self::QueuedCancelFailure => Err(REPLY_OBSERVATION_REFUSED),
            Self::InvalidInlineCount => Ok(Poll::Ready(FirstReplyCompletion::Invalid(
                REPLY_INVALID_COUNT,
            ))),
            _ if immediate => Ok(Poll::Ready(FirstReplyCompletion::Written)),
            _ => Ok(Poll::Pending),
        };
        let terminal: FirstReplyCompletion = if self == Self::QueuedCancellation {
            FirstReplyCompletion::Abandoned
        } else {
            FirstReplyCompletion::Written
        };
        ReplyScript {
            immediate,
            stop_on_submit: (self == Self::InlineStop).then_some(stop),
            stop_on_peer_check: match self {
                Self::StopDuringPeerCheck => Some((stop, 0)),
                Self::StopDuringFinalPeerCheck => Some((stop, 1)),
                _ => None,
            },
            change_on_peer_check: match self {
                Self::RevisionChangedDuringFinalPeerCheck => Some((changes, stop, 1)),
                Self::RevisionChangedAfterInlineWrite | Self::RevisionChangedAfterQueue => {
                    Some((changes, stop, 2))
                }
                _ => None,
            },
            change_on_write_completion: matches!(
                self,
                Self::RevisionChangedDuringWriteCompletion | Self::UnavailableDuringWriteCompletion
            )
            .then_some((changes, stop)),
            change_on_pending_write: (self == Self::RevisionChangedDuringPending)
                .then_some((changes, stop)),
            peer_current,
            withdraw_on_completion: self == Self::PeerWithdrawnOnCompletion,
            cancel: if self == Self::QueuedCancelFailure {
                Err(REPLY_CANCEL_REFUSED)
            } else {
                Ok(())
            },
            polls: if self == Self::PeerWithdrawnQueued
                || matches!(
                    self,
                    Self::RevisionChangedBetweenPolls | Self::RevisionChangedDuringPending
                ) {
                [first, Ok(Poll::Pending), Ok(Poll::Ready(terminal))]
            } else {
                [first, Ok(Poll::Ready(terminal)), Ok(Poll::Pending)]
            }
            .into_iter(),
        }
    }
}

/// Submit and drain on the actual worker under the original host's Running permit.
///
/// # Errors
/// Returns a startup refusal or an absent original deadline.
///
/// # Panics
/// Panics if the scripted histories reveal a repeated effect or premature release.
#[test]
fn broker_reply_keeps_the_original_cutoff_and_drains_exactly_one_write() -> Result<(), FixtureError>
{
    static CHANGES: [BrokerClockChanges; 16] = [const { BrokerClockChanges::new() }; 16];
    let histories: [ReplyHistory; 16] = [
        ReplyHistory::ExpiredBeforeSubmit,
        ReplyHistory::MailboxClosedBeforeSubmit,
        ReplyHistory::InvalidInlineCount,
        ReplyHistory::LiveInline,
        ReplyHistory::InlineStop,
        ReplyHistory::QueuedStop,
        ReplyHistory::QueuedCancellation,
        ReplyHistory::QueuedExpiry,
        ReplyHistory::QueuedCancelFailure,
        ReplyHistory::LiveQueued,
        ReplyHistory::PeerWithdrawnBeforeSubmit,
        ReplyHistory::PeerWithdrawnQueued,
        ReplyHistory::PeerWithdrawnOnCompletion,
        ReplyHistory::StopDuringPeerCheck,
        ReplyHistory::StopDuringFinalPeerCheck,
        ReplyHistory::RevisionChangedDuringFinalPeerCheck,
    ];
    for (history, changes) in histories.into_iter().zip(CHANGES.iter()) {
        check_reply_history(history, changes)?;
    }
    Ok(())
}

/// Drive stale positive and valid neighboring replies through actual Running collection.
/// # Errors
/// Preserves fixture startup and terminal observation refusals.
#[test]
fn broker_confirms_only_current_positive_reply_revisions() -> Result<(), FixtureError> {
    static CHANGES: [BrokerClockChanges; 7] = [const { BrokerClockChanges::new() }; 7];
    let histories: [ReplyHistory; 7] = [
        ReplyHistory::PositiveQueued,
        ReplyHistory::UnavailableDuringWriteCompletion,
        ReplyHistory::RevisionChangedDuringWriteCompletion,
        ReplyHistory::RevisionChangedAfterInlineWrite,
        ReplyHistory::RevisionChangedAfterQueue,
        ReplyHistory::RevisionChangedBetweenPolls,
        ReplyHistory::RevisionChangedDuringPending,
    ];
    for (history, changes) in histories.into_iter().zip(CHANGES.iter()) {
        check_reply_history(history, changes)?;
    }
    Ok(())
}

/// Start one invocation and present the handoff in the read's terminal poll.
///
/// # Errors
/// Returns a startup refusal or an absent original deadline.
///
/// # Panics
/// Panics if accepted reply work escapes its original ownership or guard.
fn check_reply_history(
    history: ReplyHistory,
    changes: &'static BrokerClockChanges,
) -> Result<(), FixtureError> {
    let observations: Rc<RefCell<Observations>> = Rc::default();
    let trace: RefCell<EndpointTrace> = RefCell::default();
    let peer_current: Cell<bool> = Cell::new(true);
    let stop: StopNotification = StopNotification::new();
    let mut clock: TestClock = fixed_accept_clock();
    let anchor: Instant = clock.0.borrow().current;
    let mut factory: Factory = Factory {
        observations: Rc::clone(&observations),
        failure: None,
    };
    let acquire = || {
        Ok(ArmedEndpoint {
            reply: Some(history.script(&stop, &peer_current, changes)),
            ..ArmedEndpoint::new(
                &observations,
                &trace,
                FirstReadStart::Immediate,
                Ok(()),
                &[Ok(Poll::Ready(FirstReadCompletion::Settled))],
            )
        })
    };
    assert!(changes.arm());
    #[cfg(windows)]
    let reader: NativeBrokerReader = NativeBrokerReader::new(changes, || {
        Err(crate::broker_clock_change::BrokerClockSampleError::Adjustment(0))
    });
    #[cfg(not(windows))]
    let reader: NativeBrokerReader = NativeBrokerReader::new(changes, || Err(0));
    let mut invocation: ServiceInvocation<Reporter, BrokerEndpointWorker<_, ArmedEndpoint<'_>>> =
        start_broker_worker(
            0,
            &mut factory,
            &mut HealthyTokenObserver(LabRole::ClockBroker),
            &mut clock,
            &stop,
            reader,
            BrokerEndpointWorker::new(acquire),
        )
        .map_err(startup_error)?;
    for _ in 0..2 {
        assert!(matches!(
            invocation.poll(&mut clock, &stop),
            InvocationPoll::Pending(_)
        ));
    }
    let cutoff: Instant = invocation
        .connection_cutoff
        .ok_or(FixtureError::OutcomeUnavailable)?;
    assert_eq!(cutoff, anchor + Duration::from_secs(5));
    match history {
        ReplyHistory::ExpiredBeforeSubmit => clock.0.borrow_mut().current = cutoff,
        ReplyHistory::MailboxClosedBeforeSubmit => changes.close(),
        ReplyHistory::PeerWithdrawnBeforeSubmit => peer_current.set(false),
        _ => {}
    }
    let deadline: Instant = invocation
        .run_deadline
        .ok_or(FixtureError::OutcomeUnavailable)?;
    let mut reply_observations: Vec<ReplyObservation> = submit_scripted_reply(
        &mut invocation,
        &mut clock,
        &stop,
        deadline,
        cutoff,
        history,
        changes,
    )?;
    if history.collects_while_running() {
        reply_observations.extend(collect_scripted_reply(
            &mut invocation,
            &mut clock,
            &stop,
            deadline,
            cutoff,
            history,
            || {
                if history == ReplyHistory::PeerWithdrawnQueued {
                    peer_current.set(false);
                } else {
                    let disposition: ControlDisposition = deliver(Some(changes), &stop, 0x10, 0);
                    assert_eq!(disposition.status, 0);
                    assert!(disposition.wake);
                }
            },
        ));
    }
    let revision_before_stop: Option<ClockRevision<'static>> = changes.revision();
    stop_after_scripted_reply(&mut invocation, &mut clock, &stop, history, cutoff)?;
    for observation in reply_observations {
        assert_eq!(observation.actual, observation.expected, "{history:?}");
    }
    #[cfg(windows)]
    assert_eq!(invocation.native_status.reply_written, history.confirms());
    assert_eq!(invocation.connection_cutoff, Some(cutoff));
    check_reply_trace(history, &trace.borrow(), revision_before_stop.as_ref());
    Ok(())
}

/// Check the original endpoint calls and returned dispositions.
/// # Panics
/// Fails if the operation trace differs from the selected history.
fn check_reply_trace(
    history: ReplyHistory,
    trace: &EndpointTrace,
    revision_before_stop: Option<&ClockRevision<'_>>,
) {
    assert!(
        match_reply_calls(history, &trace.calls),
        "{history:?}: {:?}",
        trace.calls
    );
    check_peer_reply_observations(history, trace);
    if matches!(
        history,
        ReplyHistory::RevisionChangedDuringWriteCompletion
            | ReplyHistory::RevisionChangedAfterInlineWrite
            | ReplyHistory::UnavailableDuringWriteCompletion
    ) || history.retires_queued_revision()
    {
        assert!(
            revision_before_stop
                .is_some_and(|revision: &ClockRevision<'_>| { revision.number().get() == 2 })
        );
    }
}

/// Pass the same bytes twice through the host seam to observe the one-write bound.
/// # Errors
/// Returns an absent original or replacement fixture revision.
/// # Panics
/// Fails if the original admission yields an unexpected wait.
fn submit_scripted_reply<'a, A: FnOnce() -> Result<ArmedEndpoint<'a>, NativeError>>(
    invocation: &mut ServiceInvocation<Reporter, BrokerEndpointWorker<A, ArmedEndpoint<'a>>>,
    clock: &mut TestClock,
    stop: &StopNotification,
    deadline: Instant,
    cutoff: Instant,
    history: ReplyHistory,
    changes: &'static BrokerClockChanges,
) -> Result<Vec<ReplyObservation>, FixtureError> {
    let expected: ConnectionWait = match history {
        ReplyHistory::ExpiredBeforeSubmit
        | ReplyHistory::MailboxClosedBeforeSubmit
        | ReplyHistory::PeerWithdrawnBeforeSubmit
        | ReplyHistory::InvalidInlineCount
        | ReplyHistory::LiveInline
        | ReplyHistory::InlineStop
        | ReplyHistory::StopDuringPeerCheck
        | ReplyHistory::StopDuringFinalPeerCheck
        | ReplyHistory::RevisionChangedDuringFinalPeerCheck
        | ReplyHistory::RevisionChangedAfterInlineWrite => ConnectionWait::Service,
        ReplyHistory::RevisionChangedAfterQueue => ConnectionWait::Drain,
        _ => ConnectionWait::Cutoff(cutoff),
    };
    let positive_revision: Option<ClockRevision<'static>> = match history {
        ReplyHistory::LiveInline
        | ReplyHistory::PositiveQueued
        | ReplyHistory::RevisionChangedDuringFinalPeerCheck
        | ReplyHistory::RevisionChangedDuringWriteCompletion
        | ReplyHistory::RevisionChangedAfterInlineWrite
        | ReplyHistory::RevisionChangedAfterQueue
        | ReplyHistory::RevisionChangedBetweenPolls
        | ReplyHistory::RevisionChangedDuringPending => {
            Some(changes.revision().ok_or(FixtureError::OutcomeUnavailable)?)
        }
        _ => None,
    };
    let mut observations: Vec<ReplyObservation> = Vec::new();
    for expected in [
        (expected, history == ReplyHistory::LiveInline),
        (ConnectionWait::Service, false),
    ] {
        let mut confirmed: bool = false;
        let actual: ConnectionWait =
            invocation.drive_running_connection(clock, stop, deadline, |worker, _, connection| {
                let result: Result<ConnectionWait, WorkerFailure> = worker.begin_reply(
                    connection,
                    SCRIPTED_REPLY,
                    cutoff,
                    positive_revision
                        .as_ref()
                        .map(ClockRevision::for_status_reply),
                );
                confirmed = connection.reply_written;
                result
            });
        observations.push(ReplyObservation {
            actual: (actual, confirmed),
            expected,
        });
    }
    if history == ReplyHistory::RevisionChangedDuringFinalPeerCheck {
        let original: &ClockRevision<'static> = positive_revision
            .as_ref()
            .ok_or(FixtureError::OutcomeUnavailable)?;
        let current: ClockRevision<'static> =
            changes.revision().ok_or(FixtureError::OutcomeUnavailable)?;
        assert_eq!(original.number().get(), 1);
        assert_eq!(current.number().get(), 2);
        assert!(!original.admits());
        assert!(current.admits());
        assert!(!stop.stop_requested());
        assert!(clock.0.borrow().current < cutoff);
    }
    Ok(observations)
}

/// Drive queued progress and withdrawal through the actual Running owner.
/// # Panics
/// Fails if withdrawal renews authority, or if retirement releases the charge early.
fn collect_scripted_reply<P: Worker>(
    invocation: &mut ServiceInvocation<Reporter, P>,
    clock: &mut TestClock,
    stop: &StopNotification,
    deadline: Instant,
    cutoff: Instant,
    history: ReplyHistory,
    withdraw: impl FnOnce(),
) -> Vec<ReplyObservation> {
    let collect = |invocation: &mut ServiceInvocation<Reporter, P>, clock: &mut TestClock| {
        let mut confirmed: bool = false;
        let wait: ConnectionWait =
            invocation.drive_running_connection(clock, stop, deadline, |worker, wake, edge| {
                let result: Result<ConnectionWait, WorkerFailure> =
                    worker.poll_running_with_connection(wake, edge);
                confirmed = edge.reply_written;
                result
            });
        (wait, confirmed)
    };
    let first_wait: ConnectionWait = if matches!(
        history,
        ReplyHistory::RevisionChangedAfterQueue | ReplyHistory::RevisionChangedDuringPending
    ) {
        ConnectionWait::Drain
    } else {
        ConnectionWait::Cutoff(cutoff)
    };
    let mut observations: Vec<ReplyObservation> = vec![ReplyObservation {
        actual: collect(invocation, clock),
        expected: (first_wait, false),
    }];
    if history == ReplyHistory::PeerWithdrawnQueued {
        withdraw();
        for _ in 0..2 {
            assert_eq!(
                invocation.poll(clock, stop),
                InvocationPoll::Pending(Duration::from_millis(20))
            );
            assert_eq!(invocation.state(), Some(ServiceState::Running));
            assert_eq!(invocation.outstanding_work(), 1);
            assert!(invocation.take_receipt().is_none());
        }
    } else if history == ReplyHistory::RevisionChangedBetweenPolls {
        withdraw();
        observations.push(ReplyObservation {
            actual: collect(invocation, clock),
            expected: (ConnectionWait::Drain, false),
        });
    }
    if matches!(
        history,
        ReplyHistory::RevisionChangedBetweenPolls | ReplyHistory::RevisionChangedDuringPending
    ) {
        observations.push(ReplyObservation {
            actual: collect(invocation, clock),
            expected: (ConnectionWait::Drain, false),
        });
    }
    assert_eq!(invocation.state(), Some(ServiceState::Running));
    assert_eq!(invocation.outstanding_work(), 1);
    assert!(invocation.take_receipt().is_none());
    observations.push(ReplyObservation {
        actual: collect(invocation, clock),
        expected: (ConnectionWait::Service, history.confirms()),
    });
    assert_eq!(invocation.state(), Some(ServiceState::Running));
    assert_eq!(invocation.outstanding_work(), 1);
    assert!(invocation.take_receipt().is_none());
    assert!(invocation.failure().is_none());
    assert!(!stop.stop_requested());
    assert!(clock.0.borrow().current < cutoff);
    assert!(
        invocation
            .broker_clock_reader
            .as_ref()
            .is_some_and(NativeBrokerReader::status_source_open)
    );
    observations
}

/// Check the original backend's physical dispositions independently of publication.
/// # Panics
/// Fails if a guard did not reach the selected authority state or misobserved a write.
fn check_peer_reply_observations(history: ReplyHistory, trace: &EndpointTrace) {
    const NO_RESULTS: [Result<Poll<FirstReplyCompletion>, NativeError>; 0] = [];
    let pending: Result<Poll<FirstReplyCompletion>, NativeError> = Ok(Poll::Pending);
    let written: Result<Poll<FirstReplyCompletion>, NativeError> =
        Ok(Poll::Ready(FirstReplyCompletion::Written));
    match history {
        ReplyHistory::LiveQueued
        | ReplyHistory::PositiveQueued
        | ReplyHistory::RevisionChangedDuringWriteCompletion
        | ReplyHistory::UnavailableDuringWriteCompletion => {
            assert_eq!(trace.reply_peer_checks, [true; 7]);
            assert_eq!(trace.reply_results, [pending, written]);
        }
        ReplyHistory::RevisionChangedAfterInlineWrite => {
            assert_eq!(trace.reply_peer_checks, [true; 3]);
            assert_eq!(trace.reply_results, [written]);
        }
        ReplyHistory::PeerWithdrawnBeforeSubmit | ReplyHistory::StopDuringPeerCheck => {
            assert_eq!(
                trace.reply_peer_checks,
                [history == ReplyHistory::StopDuringPeerCheck]
            );
            assert_eq!(trace.reply_results, NO_RESULTS);
        }
        ReplyHistory::StopDuringFinalPeerCheck
        | ReplyHistory::RevisionChangedDuringFinalPeerCheck => {
            assert_eq!(trace.reply_peer_checks, [true, true]);
            assert_eq!(trace.reply_results, NO_RESULTS);
        }
        ReplyHistory::PeerWithdrawnQueued => {
            assert_eq!(
                trace.reply_peer_checks,
                [true, true, true, true, true, false]
            );
            assert_eq!(trace.reply_results, [pending, pending, written]);
        }
        ReplyHistory::RevisionChangedAfterQueue => {
            assert_eq!(trace.reply_peer_checks, [true; 3]);
            assert_eq!(trace.reply_results, [pending, written]);
        }
        ReplyHistory::RevisionChangedBetweenPolls | ReplyHistory::RevisionChangedDuringPending => {
            let checks: usize = if history == ReplyHistory::RevisionChangedBetweenPolls {
                6
            } else {
                5
            };
            assert_eq!(trace.reply_peer_checks.as_slice(), &[true; 6][..checks]);
            assert_eq!(trace.reply_results, [pending, pending, written]);
        }
        ReplyHistory::PeerWithdrawnOnCompletion => {
            assert_eq!(
                trace.reply_peer_checks,
                [true, true, true, true, true, true, false]
            );
            assert_eq!(trace.reply_results, [pending, written]);
        }
        _ => {}
    }
}

/// Retire the original reply work and verify its sealed diagnostic records.
///
/// # Errors
/// Returns an absent terminal receipt.
///
/// # Panics
/// Panics if cancellation is mistaken for completion or a diagnostic loses its cause.
fn stop_after_scripted_reply(
    invocation: &mut ServiceInvocation<Reporter, impl Worker>,
    clock: &mut TestClock,
    stop: &StopNotification,
    history: ReplyHistory,
    cutoff: Instant,
) -> Result<(), FixtureError> {
    if history == ReplyHistory::QueuedExpiry {
        clock.0.borrow_mut().current = cutoff;
        assert_eq!(
            invocation.poll(clock, stop),
            InvocationPoll::Pending(Duration::from_millis(20))
        );
        assert_eq!(invocation.state(), Some(ServiceState::Running));
    }
    assert_eq!(invocation.outstanding_work(), 1);
    if matches!(
        history,
        ReplyHistory::InlineStop
            | ReplyHistory::StopDuringPeerCheck
            | ReplyHistory::StopDuringFinalPeerCheck
    ) {
        assert!(stop.stop_requested());
    } else {
        assert_eq!(stop.handle_control(1), ScmControl::Stop);
    }
    let expected: Option<WorkerError> = (history == ReplyHistory::QueuedCancelFailure)
        .then_some(WorkerError::EndpointReplyCleanup(REPLY_CANCEL_REFUSED));
    if matches!(
        history,
        ReplyHistory::QueuedStop
            | ReplyHistory::QueuedCancellation
            | ReplyHistory::QueuedExpiry
            | ReplyHistory::QueuedCancelFailure
    ) {
        assert_eq!(
            invocation.poll(clock, stop),
            InvocationPoll::NativeDrain(Duration::from_millis(20))
        );
        assert_eq!(invocation.state(), Some(ServiceState::StopPending));
        assert_eq!(invocation.outstanding_work(), 1);
        assert!(invocation.take_receipt().is_none());
        assert_eq!(invocation.pending_cleanup_failure, expected);
    }
    assert_eq!(invocation.poll(clock, stop), InvocationPoll::Finished);
    assert_eq!(invocation.outstanding_work(), 0);
    if matches!(
        history,
        ReplyHistory::StopDuringPeerCheck | ReplyHistory::StopDuringFinalPeerCheck
    ) {
        assert_eq!(invocation.state(), Some(ServiceState::Stopped));
    }
    assert_eq!(
        invocation.failure(),
        if history == ReplyHistory::InvalidInlineCount {
            Some(FixtureError::Worker(WorkerError::EndpointReply(
                REPLY_INVALID_COUNT,
            )))
        } else {
            expected.map(FixtureError::Worker)
        }
        .as_ref()
    );
    let receipt: HostReceipt = invocation
        .take_receipt()
        .ok_or(FixtureError::OutcomeUnavailable)?;
    if matches!(
        history,
        ReplyHistory::StopDuringPeerCheck | ReplyHistory::StopDuringFinalPeerCheck
    ) {
        assert!(receipt.result().is_ok());
    }
    let records: super::DiagnosticRouteView =
        super::DiagnosticRouteView::from_route(receipt.diagnostics().required());
    let observed: Vec<(u8, u8)> = records
        .records()
        .iter()
        .filter(|record| matches!(record[2], 4 | 5 | 9 | 10))
        .map(|record: &[u8; 9]| (record[2], record[4]))
        .collect();
    let expected: &[(u8, u8)] = match history {
        ReplyHistory::InvalidInlineCount => &[(4, 22), (9, 22)],
        ReplyHistory::QueuedCancelFailure => &[(4, 23)],
        _ => &[],
    };
    assert_eq!(observed, expected, "{history:?}");
    Ok(())
}

/// Compare the exact accepted read and retirement calls on one reservation.
fn match_reply_calls(history: ReplyHistory, calls: &[EndpointCall]) -> bool {
    let [
        EndpointCall::Begin(Some(4)),
        EndpointCall::Poll(Some(4)),
        EndpointCall::BeginRead(Some(4)),
        EndpointCall::PollRead(Some(4)),
        reply @ ..,
        EndpointCall::Close(Some(3)),
    ] = calls
    else {
        return false;
    };
    match history {
        ReplyHistory::ExpiredBeforeSubmit
        | ReplyHistory::MailboxClosedBeforeSubmit
        | ReplyHistory::PeerWithdrawnBeforeSubmit
        | ReplyHistory::StopDuringPeerCheck
        | ReplyHistory::StopDuringFinalPeerCheck
        | ReplyHistory::RevisionChangedDuringFinalPeerCheck => reply.is_empty(),
        ReplyHistory::LiveInline
        | ReplyHistory::InvalidInlineCount
        | ReplyHistory::RevisionChangedAfterInlineWrite => matches!(
            reply,
            [
                EndpointCall::BeginReply(Some(4)),
                EndpointCall::PollReply(Some(4)),
            ]
        ),
        ReplyHistory::InlineStop => matches!(
            reply,
            [
                EndpointCall::BeginReply(Some(4)),
                EndpointCall::PollReply(Some(3 | 4)),
            ]
        ),
        ReplyHistory::QueuedExpiry => matches!(
            reply,
            [
                EndpointCall::BeginReply(Some(4)),
                EndpointCall::CancelReply(Some(4)),
                EndpointCall::PollReply(Some(3)),
                EndpointCall::PollReply(Some(3)),
            ]
        ),
        ReplyHistory::QueuedStop
        | ReplyHistory::QueuedCancellation
        | ReplyHistory::QueuedCancelFailure => matches!(
            reply,
            [
                EndpointCall::BeginReply(Some(4)),
                EndpointCall::CancelReply(Some(3)),
                EndpointCall::PollReply(Some(3)),
                EndpointCall::PollReply(Some(3)),
            ]
        ),
        ReplyHistory::LiveQueued
        | ReplyHistory::PositiveQueued
        | ReplyHistory::RevisionChangedDuringWriteCompletion
        | ReplyHistory::UnavailableDuringWriteCompletion
        | ReplyHistory::PeerWithdrawnOnCompletion => matches!(
            reply,
            [
                EndpointCall::BeginReply(Some(4)),
                EndpointCall::PollReply(Some(4)),
                EndpointCall::PollReply(Some(4)),
            ]
        ),
        ReplyHistory::RevisionChangedAfterQueue => matches!(
            reply,
            [
                EndpointCall::BeginReply(Some(4)),
                EndpointCall::CancelReply(Some(4)),
                EndpointCall::PollReply(Some(4)),
                EndpointCall::PollReply(Some(4)),
            ]
        ),
        ReplyHistory::PeerWithdrawnQueued
        | ReplyHistory::RevisionChangedBetweenPolls
        | ReplyHistory::RevisionChangedDuringPending => matches!(
            reply,
            [
                EndpointCall::BeginReply(Some(4)),
                EndpointCall::PollReply(Some(4)),
                EndpointCall::CancelReply(Some(4)),
                EndpointCall::PollReply(Some(4)),
                EndpointCall::PollReply(Some(4)),
            ]
        ),
    }
}

/// Run the actual broker worker on the original native accept and read events.
///
/// # Errors
/// Returns a fixture native or lifecycle refusal.
///
/// # Panics
/// Panics if a completion event is borrowed for the wrong operation or survives retirement.
#[cfg(windows)]
#[test]
fn native_broker_first_read_wakes_and_drains_on_the_original_event()
-> Result<(), Box<dyn std::error::Error>> {
    use crate::{
        native_broker_endpoint::NativeBrokerEndpoint,
        native_broker_wait::{BrokerWait, WakeSource},
        native_pipe_pair::{
            LabClientContext, LabPipeWriter, open_first_read_writer, prepare_first_read_fixture,
        },
    };

    for (case, stop_with_stalled_client) in [(0x4652_4544_u32, false), (0x4652_4545_u32, true)] {
        let mut factory: Factory = Factory {
            observations: Rc::default(),
            failure: None,
        };
        let mut clock: TestClock = fixed_accept_clock();
        let stop: StopNotification = StopNotification::new();
        let control: BrokerControlWake = BrokerControlWake::new()?;
        let mut wait: BrokerWait = BrokerWait::new();
        let acquire = || -> Result<NativeBrokerEndpoint, NativeError> {
            prepare_first_read_fixture(case, None)
        };
        let mut invocation: ServiceInvocation<
            Reporter,
            BrokerEndpointWorker<_, NativeBrokerEndpoint>,
        > = start_invocation(
            LabRole::ClockBroker,
            0,
            &mut factory,
            &mut HealthyTokenObserver(LabRole::ClockBroker),
            &mut clock,
            &stop,
            BrokerEndpointWorker::new(acquire),
        )
        .map_err(startup_error)?;
        assert_eq!(
            invocation.poll(&mut clock, &stop),
            InvocationPoll::Pending(Duration::from_secs(5))
        );
        check_broker_read_wake(
            &mut wait,
            &control,
            &mut invocation.worker,
            Duration::ZERO,
            WakeSource::Timeout,
        )?;
        let writer: LabPipeWriter = open_first_read_writer(case, LabClientContext::Identification)?;
        check_broker_read_wake(
            &mut wait,
            &control,
            &mut invocation.worker,
            Duration::from_secs(2),
            WakeSource::Completion,
        )?;
        assert_eq!(
            invocation.poll(&mut clock, &stop),
            InvocationPoll::Pending(Duration::from_secs(5))
        );
        assert!(invocation.worker.completion_wake().is_some());
        check_broker_read_wake(
            &mut wait,
            &control,
            &mut invocation.worker,
            Duration::ZERO,
            WakeSource::Timeout,
        )?;
        if !stop_with_stalled_client {
            writer.write(b"first broker read")?;
            check_broker_read_wake(
                &mut wait,
                &control,
                &mut invocation.worker,
                Duration::from_secs(2),
                WakeSource::Completion,
            )?;
            check_broker_read_wake(
                &mut wait,
                &control,
                &mut invocation.worker,
                Duration::ZERO,
                WakeSource::Timeout,
            )?;
            assert_eq!(
                invocation.poll(&mut clock, &stop),
                InvocationPoll::Pending(Duration::from_secs(60))
            );
            assert!(invocation.worker.completion_wake().is_none());
        }
        retire_native_first_read(
            &mut invocation,
            &mut clock,
            &stop,
            &mut wait,
            &control,
            stop_with_stalled_client,
        )?;
    }
    Ok(())
}

/// Enter Stop and require physical retirement before giving up the work permit.
/// # Errors
/// Returns a refused native wait.
/// # Panics
/// Panics if the original read is not settled or a clean Stop leaves work charged.
#[cfg(windows)]
fn retire_native_first_read(
    invocation: &mut ServiceInvocation<Reporter, impl Worker>,
    clock: &mut TestClock,
    stop: &StopNotification,
    wait: &mut crate::native_broker_wait::BrokerWait,
    control: &BrokerControlWake,
    stop_with_stalled_client: bool,
) -> Result<(), NativeError> {
    use crate::native_broker_wait::WakeSource;
    assert_eq!(invocation.outstanding_work(), 1);
    assert_eq!(stop.handle_control(1), ScmControl::Stop);
    let result: InvocationPoll = invocation.poll(clock, stop);
    if matches!(result, InvocationPoll::NativeDrain(_)) {
        assert!(stop_with_stalled_client);
        assert_eq!(invocation.state(), Some(ServiceState::StopPending));
        assert_eq!(invocation.outstanding_work(), 1);
        check_broker_read_wake(
            wait,
            control,
            &mut invocation.worker,
            Duration::from_secs(2),
            WakeSource::Completion,
        )?;
        assert_eq!(invocation.poll(clock, stop), InvocationPoll::Finished);
    } else {
        assert_eq!(result, InvocationPoll::Finished);
    }
    assert!(invocation.worker.completion_wake().is_none());
    assert!(invocation.failure().is_none());
    assert_eq!(invocation.outstanding_work(), 0);
    Ok(())
}

/// Check the actual wait and the event currently lent by the worker.
/// # Errors
/// Returns the original native wait failure.
/// # Panics
/// Panics if the wait observes another wake source.
#[cfg(windows)]
fn check_broker_read_wake(
    wait: &mut crate::native_broker_wait::BrokerWait,
    control: &BrokerControlWake,
    worker: &mut impl Worker,
    timeout: Duration,
    expected: crate::native_broker_wait::WakeSource,
) -> Result<(), NativeError> {
    assert_eq!(
        wait.observe(control, timeout, worker.completion_wake())?,
        expected
    );
    Ok(())
}

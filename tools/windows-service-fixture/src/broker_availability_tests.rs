// SPDX-License-Identifier: GPL-2.0-only
//! Exercise live availability before final receipt extraction from the real broker owner.

use std::{
    cell::RefCell,
    error::Error,
    sync::atomic::{AtomicBool, AtomicU32, Ordering::SeqCst},
    time::{Duration, Instant},
};

use windows_platform::{
    scm::StopNotification,
    service::{LifecycleError, ServiceState},
};

#[cfg(windows)]
use windows_platform_native::{LeapPresentation, LeapPresentationError};

use super::{
    BrokerClockChanges, BrokerClockRead, HealthyTokenObserver, HostReceipt, LabRole,
    NativeBrokerReader, Reporting, deliver,
};
use crate::{
    FixtureError, NativeError, NativeOperation,
    host::{
        BrokerAvailability, Clock, InertWorker, InvocationPoll, ServiceInvocation,
        start_broker_invocation,
    },
};

/// Separate registered callback state for each history in the single test.
static MAILBOXES: [BrokerClockChanges; 5] = [const { BrokerClockChanges::new() }; 5];
/// Actual raw getter calls, independent of the host's decision clock.
static GETTERS: AtomicU32 = AtomicU32::new(0);
/// The next scheduled sample returns the original native failure code zero.
static FAIL_SAMPLE: AtomicBool = AtomicBool::new(false);

/// Successful and failed samples from the real native getter.
#[cfg(windows)]
type Sample = Result<crate::BrokerClockSample, crate::BrokerClockSampleError>;
/// Successful and failed samples from the independent portable script.
#[cfg(not(windows))]
type Sample = Result<u64, u32>;

/// Return a monotone sample or the explicitly selected failure.
/// # Errors
/// Return the fixture's original failure code zero.
/// # Panics
/// On Windows, panics if the selected adjustment failure is not reached and preserved.
fn read() -> Sample {
    let earlier: u32 = GETTERS.fetch_add(1, SeqCst);
    sample(earlier, FAIL_SAMPLE.swap(false, SeqCst))
}

/// Return a real clock sample with the selected presentation refusal.
/// # Errors
/// Preserves an original coordinate or adjustment failure.
#[cfg(windows)]
fn read_with_presentation_failure(failure: LeapPresentationError) -> Sample {
    let _earlier: u32 = GETTERS.fetch_add(1, SeqCst);
    crate::BrokerClockSample::read_with_presentation_failure(failure)
}

/// Return a real read on the healthy native path and inject the staged failure.
/// # Errors
/// Returns the selected refusal or the original Windows getter failure.
/// # Panics
/// Panics if the coordinate getter fails before the scripted adjustment refusal
/// or the combined getter does not preserve its failure stage and code.
#[cfg(windows)]
fn sample(_earlier: u32, fail: bool) -> Sample {
    if fail {
        let result: Sample = crate::BrokerClockSample::read_with_adjustment(|| Err(0));
        assert_eq!(result, Err(crate::BrokerClockSampleError::Adjustment(0)));
        return result;
    }
    crate::BrokerClockSample::read()
}

/// Return the independent portable positive and refused getter results.
/// # Errors
/// Returns the literal scripted error code zero.
#[cfg(not(windows))]
fn sample(earlier: u32, fail: bool) -> Sample {
    if fail {
        return Err(0);
    }
    Ok(u64::from(earlier))
}

/// Deliver a control while the real owner observes the final decision time.
struct DecisionClock<'a> {
    /// Explicit monotonic time from the independent scenario.
    time: Instant,
    /// Optional control delivered during exactly the next owner read.
    next_control: Option<u32>,
    /// The original registered observer and Stop pair.
    mailbox: &'static BrokerClockChanges,
    /// Sticky Stop for the same invocation.
    stop: &'a StopNotification,
    /// Whether the selected control reached the actual callback branch.
    delivered_wake: bool,
}

impl Clock for DecisionClock<'_> {
    fn now(&mut self) -> Instant {
        if let Some(raw) = self.next_control.take() {
            self.delivered_wake = deliver(Some(self.mailbox), self.stop, raw, 0).wake;
        }
        self.time
    }
}

/// Independent sequence selected before creating the invocation.
#[derive(Clone, Copy, Eq, PartialEq)]
enum Scenario {
    /// A status attempt which the reporter refuses.
    RunningRefused,
    /// Failed sample, renewal and repeated reads through suspension.
    Renewal,
    /// Time change crossing the live edge after an original success.
    TimeChangeAtDecision,
    /// Stop crossing the live edge after an original success.
    StopAtDecision,
    /// Exact cutoff refusal beside its still-admitted predecessor.
    Deadline,
}

/// Select where the host sees a typed presentation failure on Windows.
#[cfg(windows)]
fn presentation_getter(scenario: Scenario) -> fn() -> Sample {
    match scenario {
        Scenario::TimeChangeAtDecision => {
            || read_with_presentation_failure(LeapPresentationError::QueryFailed(0))
        }
        Scenario::StopAtDecision => || {
            read_with_presentation_failure(LeapPresentationError::InvalidReply {
                flags: 2,
                reserved: 19,
            })
        },
        _ => read,
    }
}

/// Fixed observations from follow-up actions before service retirement.
struct Subsequent {
    /// Decision crossing the chosen control or deadline.
    crossed: BrokerAvailability,
    /// Status around the five selected failure and renewal edges.
    renewal: [BrokerAvailability; 5],
    /// Actual getter counts immediately before and after the suspended poll.
    calls: [u32; 2],
    /// The registered handler acknowledged the two scripted power events.
    power_wakes: [bool; 2],
}

/// Send the selected lifecycle events before taking any terminal receipt.
fn subsequent(
    scenario: Scenario,
    start: Instant,
    owner: &mut ServiceInvocation<Reporting<'_>, InertWorker>,
    clock: &mut DecisionClock<'_>,
    stop: &StopNotification,
) -> Subsequent {
    use BrokerAvailability::Unavailable;

    let mut crossed: BrokerAvailability = Unavailable;
    let mut calls: [u32; 2] = [0; 2];
    let mut renewal: [BrokerAvailability; 5] = [Unavailable; 5];
    let mut power_wakes: [bool; 2] = [false; 2];
    match scenario {
        Scenario::RunningRefused => (),
        Scenario::TimeChangeAtDecision | Scenario::StopAtDecision => {
            clock.next_control = Some(if scenario == Scenario::StopAtDecision {
                1
            } else {
                16
            });
            crossed = owner.broker_availability(clock, stop);
        }
        Scenario::Deadline => {
            clock.time = start + Duration::from_nanos(59_999_999_999);
            renewal[0] = owner.broker_availability(clock, stop);
            clock.time = start + Duration::from_secs(60);
            crossed = owner.broker_availability(clock, stop);
        }
        Scenario::Renewal => {
            FAIL_SAMPLE.store(true, SeqCst);
            clock.time = start + Duration::from_secs(1);
            let _observed_wait: InvocationPoll = owner.poll(clock, stop);
            renewal[0] = owner.broker_availability(clock, stop);
            clock.time = start + Duration::from_secs(2);
            let _observed_wait: InvocationPoll = owner.poll(clock, stop);
            renewal[1] = owner.broker_availability(clock, stop);
            calls[0] = GETTERS.load(SeqCst);
            power_wakes[0] = deliver(Some(clock.mailbox), stop, 0x0d, 0x04).wake;
            let _observed_wait: InvocationPoll = owner.poll(clock, stop);
            renewal[2] = owner.broker_availability(clock, stop);
            calls[1] = GETTERS.load(SeqCst);
            power_wakes[1] = deliver(Some(clock.mailbox), stop, 0x0d, 0x12).wake;
            renewal[3] = owner.broker_availability(clock, stop);
            let _observed_wait: InvocationPoll = owner.poll(clock, stop);
            renewal[4] = owner.broker_availability(clock, stop);
        }
    }
    Subsequent {
        crossed,
        renewal,
        calls,
        power_wakes,
    }
}

/// Only a successful sample from this awake Running context is observable.
/// # Errors
/// Returns an unexpected startup or incomplete inert cleanup.
/// # Panics
/// Fails if a stale or missed sample supplies availability, a status call starts
/// another getter, or the owning invocation loses actual history or failure.
#[test]
#[cfg_attr(
    windows,
    ignore = "requires an explicitly admitted read-only Windows clock fixture"
)]
fn broker_reports_only_current_availability_before_cleanup() -> Result<(), Box<dyn Error>> {
    let cases: [Scenario; 5] = [
        Scenario::RunningRefused,
        Scenario::Renewal,
        Scenario::TimeChangeAtDecision,
        Scenario::StopAtDecision,
        Scenario::Deadline,
    ];
    for (scenario, mailbox) in cases.into_iter().zip(&MAILBOXES) {
        check_scenario(scenario, mailbox)?;
    }
    Ok(())
}

/// Check one independent host from registration through receipt.
/// # Errors
/// Returns an unexpected startup or incomplete inert cleanup.
/// # Panics
/// Fails if the scenario's live decisions, custody or historical result differ.
fn check_scenario(
    scenario: Scenario,
    mailbox: &'static BrokerClockChanges,
) -> Result<(), Box<dyn Error>> {
    use BrokerAvailability::{Available, Unavailable};

    GETTERS.store(0, SeqCst);
    let start: Instant = Instant::now();
    let stop: StopNotification = StopNotification::new();
    let masks: RefCell<Vec<(ServiceState, u32)>> = RefCell::default();
    let mut clock: DecisionClock<'_> = DecisionClock {
        time: start,
        next_control: None,
        mailbox,
        stop: &stop,
        delivered_wake: false,
    };
    let mut reporter: Reporting<'_> = Reporting {
        changes: mailbox,
        stop: &stop,
        masks: &masks,
        accept_running: scenario != Scenario::RunningRefused,
        early_notice: None,
        reads: &GETTERS,
    };
    #[cfg(windows)]
    let getter: fn() -> Sample = presentation_getter(scenario);
    #[cfg(not(windows))]
    let getter: fn() -> Sample = read;
    let reader: NativeBrokerReader =
        NativeBrokerReader::new(mailbox, getter).with_sampling(|_, _| None);
    let mut owner: ServiceInvocation<Reporting<'_>, InertWorker> = start_broker_invocation(
        0,
        &mut reporter,
        &mut HealthyTokenObserver(LabRole::ClockBroker),
        &mut clock,
        &stop,
        reader,
    )
    .map_err(|_receipt: HostReceipt| "unexpected broker startup failure")?;
    let preparing: BrokerAvailability = owner.broker_availability(&mut clock, &stop);
    let first_poll: InvocationPoll = owner.poll(&mut clock, &stop);
    let initial: BrokerAvailability = owner.broker_availability(&mut clock, &stop);
    let Subsequent {
        crossed,
        renewal,
        calls,
        power_wakes,
    } = subsequent(scenario, start, &mut owner, &mut clock, &stop);
    // Take observations before Stop/cleanup can hide a wrong live decision.
    let decision_getters: u32 = GETTERS.load(SeqCst);
    let _close: super::ControlDisposition = deliver(Some(mailbox), &stop, 1, 0);
    let closed: BrokerAvailability = owner.broker_availability(&mut clock, &stop);
    let last_poll: InvocationPoll = owner.poll(&mut clock, &stop);
    let receipt: HostReceipt = owner
        .take_receipt()
        .ok_or("inert broker failed to finish")?;
    assert_eq!(last_poll, InvocationPoll::Finished);
    assert_eq!(closed, Unavailable);
    assert_eq!(preparing, Unavailable);
    assert_eq!(GETTERS.load(SeqCst), decision_getters);
    match scenario {
        Scenario::RunningRefused => {
            assert_eq!(first_poll, InvocationPoll::Finished);
            assert_eq!(initial, Unavailable);
            assert_eq!(decision_getters, 0);
            assert_eq!(receipt.broker_clock_read(), None);
            assert_eq!(
                receipt.result(),
                &Err(FixtureError::Lifecycle(LifecycleError::Report(
                    NativeError::new(NativeOperation::Status, 0)
                )))
            );
        }
        Scenario::Renewal => {
            assert_eq!(initial, Available);
            assert_eq!(
                renewal,
                [Unavailable, Available, Unavailable, Unavailable, Available]
            );
            assert_eq!(power_wakes, [true; 2]);
            assert_eq!(calls, [3; 2]);
            assert_eq!(decision_getters, 4);
            assert_eq!(receipt.result(), &Ok(()));
        }
        Scenario::TimeChangeAtDecision | Scenario::StopAtDecision | Scenario::Deadline => {
            assert_eq!(initial, Available);
            assert_eq!(crossed, Unavailable);
            assert_eq!(decision_getters, 1);
            check_terminal_receipt(scenario, &receipt, renewal[0], clock.delivered_wake);
        }
    }
    Ok(())
}

/// Check the completed sample and the original termination outcome.
/// # Panics
/// Fails if raw read history, live cutoff behavior or termination attribution differs.
fn check_terminal_receipt(
    scenario: Scenario,
    receipt: &HostReceipt,
    before_cutoff: BrokerAvailability,
    delivered_wake: bool,
) {
    let read: Option<crate::NativeBrokerClockRead> = receipt.broker_clock_read();
    assert!(matches!(read, Some(BrokerClockRead::Completed {
            revision, result: Ok(_), ..
        }) if revision.get() == 1));
    if scenario == Scenario::Deadline {
        assert_eq!(before_cutoff, BrokerAvailability::Available);
        assert_eq!(receipt.result(), &Err(FixtureError::RunLimitElapsed));
    } else {
        assert!(delivered_wake);
        assert_eq!(receipt.result(), &Ok(()));
    }
    #[cfg(windows)]
    if let Some(BrokerClockRead::Completed {
        result: Ok(sample), ..
    }) = read
    {
        check_leap_presentation(scenario, sample.leap_presentation());
    }
}

/// Check the original presentation observation from the finished broker invocation.
/// # Panics
/// Fails if the native default mode or exact scripted refusal is lost or changed.
#[cfg(windows)]
fn check_leap_presentation(
    scenario: Scenario,
    observed: Result<LeapPresentation, LeapPresentationError>,
) {
    let expected: Result<LeapPresentation, LeapPresentationError> = match scenario {
        Scenario::TimeChangeAtDecision => Err(LeapPresentationError::QueryFailed(0)),
        Scenario::StopAtDecision => Err(LeapPresentationError::InvalidReply {
            flags: 2,
            reserved: 19,
        }),
        Scenario::Deadline => {
            println!("broker_leap_presentation={observed:?}");
            Ok(LeapPresentation::Compatibility)
        }
        Scenario::RunningRefused | Scenario::Renewal => return,
    };
    assert_eq!(observed, expected);
}

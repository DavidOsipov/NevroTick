// SPDX-License-Identifier: GPL-2.0-only
//! Observe one broker read through the actual retained host invocation.

use std::{
    cell::{Cell, RefCell},
    error::Error,
    rc::Rc,
    time::{Duration, Instant},
};

use super::super::{
    Clock, HostReceipt, InertWorker, InvocationPoll, ServiceInvocation,
    broker_observation::{
        BrokerObservation, BrokerPowerGeneration, BrokerReadFailure, BrokerReading, RawAdjustment,
        Retirement,
    },
    start_invocation,
};
use super::{Factory, HealthyTokenObserver, Observations, Reporter, TestClock, Time, statuses};
use crate::{FixtureError, LabRole, NativeError, NativeOperation};
use windows_platform::scm::{ScmControl, StopNotification};

/// Setup and observation errors remain distinct from the literal test expectations.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
/// The real service owner with the existing inert worker and fixture status sink.
type BrokerInvocation = ServiceInvocation<Reporter, InertWorker>;

/// Supply a status sink and complete fixed-role privileges to the actual owner.
///
/// # Errors
/// Returns the retained invocation startup failure.
fn start(
    role: LabRole,
    observations: &Rc<RefCell<Observations>>,
    clock: &mut impl Clock,
    notification: &StopNotification,
) -> TestResult<BrokerInvocation> {
    let mut factory: Factory = Factory {
        observations: Rc::clone(observations),
        failure: None,
    };
    start_invocation(
        role,
        0,
        &mut factory,
        &mut HealthyTokenObserver(role),
        clock,
        notification,
        InertWorker::new(),
    )
    .map_err(|receipt: HostReceipt| super::startup_error(receipt).into())
}

/// Request genuine Stop and return the same owner's one terminal receipt.
///
/// # Errors
/// Returns an error if the bounded poll loop fails to obtain the terminal receipt.
///
/// # Panics
/// Panics if the owner refuses Stop decoding or publishes a second receipt.
fn finish(
    invocation: &mut BrokerInvocation,
    clock: &mut impl Clock,
    notification: &StopNotification,
) -> TestResult<HostReceipt> {
    assert_eq!(notification.handle_control(1), ScmControl::Stop);
    for _ in 0..8 {
        if matches!(
            invocation.poll(clock, notification),
            InvocationPoll::Finished
        ) {
            let receipt: HostReceipt = invocation.take_receipt().ok_or("missing broker receipt")?;
            assert!(invocation.take_receipt().is_none());
            return Ok(receipt);
        }
    }
    Err("inert broker invocation did not finish within eight polls".into())
}

/// Shared test inputs and independently recorded calls for one invocation.
struct Rig {
    /// Actual service lifecycle owner.
    invocation: BrokerInvocation,
    /// Current scripted monotonic input.
    clock: TestClock,
    /// Actual sticky control, shared with selected effect hooks.
    notification: Rc<StopNotification>,
    /// Independently recorded status attempts and fault selection.
    observations: Rc<RefCell<Observations>>,
}

impl Rig {
    /// Create a healthy fixed-role owner before any broker probe is selected.
    ///
    /// # Errors
    /// Returns the original startup failure.
    fn start(role: LabRole) -> TestResult<Self> {
        let observations: Rc<RefCell<Observations>> = Rc::new(RefCell::default());
        let notification: Rc<StopNotification> = Rc::new(StopNotification::new());
        let mut clock: TestClock = TestClock(Rc::new(RefCell::new(Time {
            current: Instant::now(),
            backwards_once: false,
            backwards_persistent: false,
        })));
        let invocation: BrokerInvocation = start(role, &observations, &mut clock, &notification)?;
        Ok(Self {
            invocation,
            clock,
            notification,
            observations,
        })
    }

    /// Advance the real host once without deriving a verdict from the input script.
    fn poll(&mut self) -> InvocationPoll {
        self.invocation.poll(&mut self.clock, &self.notification)
    }

    /// Resolve the original invocation through its actual terminal transition.
    ///
    /// # Errors
    /// Returns an error if the initial invocation does not publish a receipt.
    fn finish(&mut self) -> TestResult<HostReceipt> {
        finish(&mut self.invocation, &mut self.clock, &self.notification)
    }
}

/// Unequal words with distinct high bits and an independently chosen mode.
const fn sample(disabled: bool) -> RawAdjustment {
    RawAdjustment {
        adjustment: 0x1234_5678_9abc_def0,
        increment: 0xfedc_ba98_7654_3210,
        periodic_adjustment_disabled: disabled,
    }
}

/// The actual receiving gate orders retirement against the getter and its receipt.
///
/// # Errors
/// Returns the real fixture startup or finish failure.
///
/// # Panics
/// Fails if a retired generation calls early, publishes late, or loses native failure.
#[test]
fn broker_power_generation_orders_invalidation_against_observation() -> TestResult {
    for (before_read, during_read, result) in [
        (true, false, Ok(sample(true))),
        (false, true, Ok(sample(true))),
        (false, true, Err(0_u32)),
        (false, false, Ok(sample(false))),
    ] {
        let mut rig: Rig = Rig::start(LabRole::ClockBroker)?;
        let generation: BrokerPowerGeneration = BrokerPowerGeneration::new();
        let invalidate_in_getter = generation.invalidator();
        let calls: Rc<Cell<u32>> = Rc::new(Cell::new(0));
        let called: Rc<Cell<u32>> = Rc::clone(&calls);
        assert!(
            rig.invocation
                .select_power_scoped_probe(&generation, move || {
                    called.set(called.get() + 1);
                    if during_read {
                        invalidate_in_getter();
                    }
                    result
                })
        );
        if before_read {
            generation.invalidator()();
        }
        let before: Instant = rig.clock.0.borrow().current;
        let _poll: InvocationPoll = rig.poll();
        let calls_expected: u32 = u32::from(!before_read);
        assert_eq!(calls.get(), calls_expected);

        let other: BrokerPowerGeneration = BrokerPowerGeneration::new();
        assert!(
            !rig.invocation
                .select_power_scoped_probe(&other, || Ok(sample(true)))
        );
        generation.invalidator()();
        let receipt: HostReceipt = rig.finish()?;
        assert_eq!(receipt.result(), &Ok(()));
        assert_eq!(calls.get(), calls_expected);
        let expected: BrokerObservation = if before_read || during_read {
            BrokerObservation::Retired {
                reason: Retirement::PowerContext,
                native_failure: result.err().map(BrokerReadFailure::Adjustment),
            }
        } else {
            BrokerObservation::Observed {
                before,
                after: before,
                raw: BrokerReading::Adjustment(sample(false)),
            }
        };
        assert_eq!(receipt.broker_observation(), Some(expected));
    }
    Ok(())
}

/// Dropping the owner withdraws an otherwise pending selected generation.
///
/// # Errors
/// Returns the real fixture startup or finish failure.
///
/// # Panics
/// Fails if a new context reopens the old read or the getter runs after withdrawal.
#[test]
fn broker_power_generation_cannot_outlive_its_owner() -> TestResult {
    let mut rig: Rig = Rig::start(LabRole::ClockBroker)?;
    let generation: BrokerPowerGeneration = BrokerPowerGeneration::new();
    let calls: Rc<Cell<u32>> = Rc::new(Cell::new(0));
    let called: Rc<Cell<u32>> = Rc::clone(&calls);
    assert!(
        rig.invocation
            .select_power_scoped_probe(&generation, move || {
                called.set(called.get() + 1);
                Ok(sample(true))
            })
    );
    drop(generation);
    let new_generation: BrokerPowerGeneration = BrokerPowerGeneration::new();
    assert!(
        !rig.invocation
            .select_power_scoped_probe(&new_generation, || Ok(sample(false)))
    );
    let _poll: InvocationPoll = rig.poll();
    let receipt: HostReceipt = rig.finish()?;
    assert_eq!(calls.get(), 0);
    assert_eq!(receipt.result(), &Ok(()));
    assert_eq!(
        receipt.broker_observation(),
        Some(BrokerObservation::Retired {
            reason: Retirement::PowerContext,
            native_failure: None,
        })
    );
    Ok(())
}

/// Admit the broker only once, after Running, and preserve both full-width words.
///
/// # Errors
/// Returns a fixture setup, observation time, or completion error.
///
/// # Panics
/// Panics if readiness, single execution, the bracket, or retained values differ.
#[test]
fn broker_probe_keeps_one_post_running_raw_observation() -> TestResult {
    for disabled in [false, true] {
        let mut rig: Rig = Rig::start(LabRole::ClockBroker)?;
        let calls: Rc<Cell<u32>> = Rc::new(Cell::new(0));
        let called: Rc<Cell<u32>> = Rc::clone(&calls);
        let observed: Rc<RefCell<Observations>> = Rc::clone(&rig.observations);
        let states_at_call: Rc<RefCell<Vec<[u32; 7]>>> = Rc::new(RefCell::new(Vec::new()));
        let captured: Rc<RefCell<Vec<[u32; 7]>>> = Rc::clone(&states_at_call);
        let time: Rc<RefCell<Time>> = Rc::clone(&rig.clock.0);
        let before: Instant = time.borrow().current;
        let after: Instant = before
            .checked_add(Duration::from_nanos(7))
            .ok_or("test clock overflow")?;
        let selected: bool = rig.invocation.select_broker_probe(move || {
            called.set(called.get() + 1);
            *captured.borrow_mut() = statuses(&observed);
            time.borrow_mut().current = after;
            Ok(sample(disabled))
        });
        let replaced: bool = rig.invocation.select_broker_probe(|| Err(401));
        let first: InvocationPoll = rig.poll();
        let early_receipt: bool = rig.invocation.take_receipt().is_some();
        let _repeated_poll: InvocationPoll = rig.poll();
        let receipt: HostReceipt = rig.finish()?;
        assert!(selected);
        assert!(!replaced);
        assert!(matches!(first, InvocationPoll::Pending(_)));
        assert!(!early_receipt);
        assert_eq!(calls.get(), 1);
        assert_eq!(
            *states_at_call.borrow(),
            [[16, 2, 0, 0, 0, 1, 1000], [16, 4, 1, 0, 0, 0, 0]]
        );
        assert_eq!(
            receipt.broker_observation(),
            Some(BrokerObservation::Observed {
                before,
                after,
                raw: BrokerReading::Adjustment(sample(disabled)),
            })
        );
        assert_eq!(receipt.result(), &Ok(()));
    }
    Ok(())
}

/// Retain zero and nonzero native failures without inventing an observation or service error.
///
/// # Errors
/// Returns a fixture setup or completion error.
///
/// # Panics
/// Panics if the probe admission or retained outcome differs.
#[test]
fn broker_probe_preserves_native_failure_including_zero() -> TestResult {
    for code in [0_u32, 0xe112_6789] {
        let mut rig: Rig = Rig::start(LabRole::ClockBroker)?;
        let selected: bool = rig.invocation.select_broker_probe(move || Err(code));
        let _poll: InvocationPoll = rig.poll();
        let receipt: HostReceipt = rig.finish()?;
        assert!(selected);
        assert_eq!(
            receipt.broker_observation(),
            Some(BrokerObservation::NativeFailure(
                BrokerReadFailure::Adjustment(code)
            ))
        );
        assert_eq!(receipt.result(), &Ok(()));
    }
    Ok(())
}

/// Select one independently controlled guard before the first eligible effect.
#[derive(Clone, Copy)]
enum BeforeCall {
    /// The network-facing role cannot select the probe.
    Engine,
    /// Stop is published before the owner can enter the probe.
    Stop,
    /// Stop is published during the probe's final clock read before the getter.
    StopAtEdge,
    /// The original startup budget expires before worker readiness.
    Deadline,
    /// A real status attempt is refused before Running is acknowledged.
    RunningRefusal,
}

/// Deliver Stop during the fifth read of the initial poll, after preparation,
/// readiness, and both activation boundaries have observed quiet control.
struct StopAtProbeEdge<'a> {
    clock: &'a mut TestClock,
    notification: &'a StopNotification,
    reads: u8,
}

impl Clock for StopAtProbeEdge<'_> {
    fn now(&mut self) -> Instant {
        self.reads += 1;
        if self.reads == 5 {
            assert_eq!(self.notification.handle_control(1), ScmControl::Stop);
        }
        self.clock.now()
    }
}

/// Exercise role, Stop, exact deadline and reporter refusal through the host owner.
///
/// # Errors
/// Returns a fixture setup, deadline construction, or completion error.
///
/// # Panics
/// Panics if a refused scenario calls the getter or reports a different outcome.
#[test]
fn broker_probe_has_no_effect_after_pre_call_refusal() -> TestResult {
    for guard in [
        BeforeCall::Engine,
        BeforeCall::Stop,
        BeforeCall::StopAtEdge,
        BeforeCall::Deadline,
        BeforeCall::RunningRefusal,
    ] {
        let role: LabRole = if matches!(guard, BeforeCall::Engine) {
            LabRole::Engine
        } else {
            LabRole::ClockBroker
        };
        let mut rig: Rig = Rig::start(role)?;
        let called: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let reached: Rc<Cell<bool>> = Rc::clone(&called);
        let selected: bool = rig.invocation.select_broker_probe(move || {
            reached.set(true);
            Ok(sample(false))
        });
        match guard {
            BeforeCall::Engine | BeforeCall::StopAtEdge => {}
            BeforeCall::Stop => {
                let _stop: ScmControl = rig.notification.handle_control(1);
            }
            BeforeCall::Deadline => {
                let mut time: std::cell::RefMut<'_, Time> = rig.clock.0.borrow_mut();
                time.current = time
                    .current
                    .checked_add(Duration::from_secs(10))
                    .ok_or("test clock overflow")?;
            }
            BeforeCall::RunningRefusal => rig
                .observations
                .borrow_mut()
                .status_failures
                .push((2, NativeError::new(NativeOperation::Status, 7801))),
        }
        let _poll: InvocationPoll = if matches!(guard, BeforeCall::StopAtEdge) {
            let mut edge: StopAtProbeEdge<'_> = StopAtProbeEdge {
                clock: &mut rig.clock,
                notification: &rig.notification,
                reads: 0,
            };
            rig.invocation.poll(&mut edge, &rig.notification)
        } else {
            rig.poll()
        };
        let receipt: HostReceipt = rig.finish()?;
        assert_eq!(selected, !matches!(guard, BeforeCall::Engine));
        assert!(!called.get());
        assert_eq!(
            receipt.broker_observation(),
            matches!(guard, BeforeCall::StopAtEdge).then_some(BrokerObservation::Retired {
                reason: Retirement::Stop,
                native_failure: None,
            })
        );
        assert_eq!(
            receipt.result().is_err(),
            matches!(guard, BeforeCall::Deadline | BeforeCall::RunningRefusal)
        );
    }
    Ok(())
}

/// Stop arriving inside the selected operation retires returned words and keeps its error.
///
/// # Errors
/// Returns a fixture setup or completion error.
///
/// # Panics
/// Panics if Stop accepts the late reading, repeats the getter, or loses its error.
#[test]
fn broker_probe_retires_in_call_stop_without_losing_native_failure() -> TestResult {
    for returned in [Ok(sample(true)), Err(0_u32), Err(6093_u32)] {
        let mut rig: Rig = Rig::start(LabRole::ClockBroker)?;
        let calls: Rc<Cell<u32>> = Rc::new(Cell::new(0));
        let called: Rc<Cell<u32>> = Rc::clone(&calls);
        let stop: Rc<StopNotification> = Rc::clone(&rig.notification);
        let selected: bool = rig.invocation.select_broker_probe(move || {
            called.set(called.get() + 1);
            let _stop: ScmControl = stop.handle_control(1);
            returned
        });
        let _poll: InvocationPoll = rig.poll();
        let receipt: HostReceipt = rig.finish()?;
        assert!(selected);
        assert_eq!(calls.get(), 1);
        assert_eq!(
            receipt.broker_observation(),
            Some(BrokerObservation::Retired {
                reason: Retirement::Stop,
                native_failure: returned.err().map(BrokerReadFailure::Adjustment),
            })
        );
        assert_eq!(receipt.result(), &Ok(()));
    }
    Ok(())
}

/// A getter returning at the exact Running cutoff or after reversal yields no raw words.
///
/// # Errors
/// Returns a fixture setup, deadline construction, or completion error.
///
/// # Panics
/// Panics if the late read or original lifecycle failure is classified incorrectly.
#[test]
fn broker_probe_retires_at_in_call_deadline_or_clock_reversal() -> TestResult {
    for reason in [Retirement::Deadline, Retirement::Clock] {
        let mut rig: Rig = Rig::start(LabRole::ClockBroker)?;
        let time: Rc<RefCell<Time>> = Rc::clone(&rig.clock.0);
        let cutoff: Instant = time
            .borrow()
            .current
            .checked_add(Duration::from_secs(60))
            .ok_or("test clock overflow")?;
        let selected: bool = rig.invocation.select_broker_probe(move || {
            let mut time: std::cell::RefMut<'_, Time> = time.borrow_mut();
            if reason == Retirement::Deadline {
                time.current = cutoff;
            } else {
                time.backwards_once = true;
            }
            Ok(sample(false))
        });
        let _poll: InvocationPoll = rig.poll();
        let receipt: HostReceipt = rig.finish()?;
        assert!(selected);
        assert_eq!(
            receipt.broker_observation(),
            Some(BrokerObservation::Retired {
                reason,
                native_failure: None
            })
        );
        let expected: FixtureError = if reason == Retirement::Deadline {
            FixtureError::RunLimitElapsed
        } else {
            FixtureError::ClockReversed
        };
        assert_eq!(receipt.result(), &Err(expected));
    }
    Ok(())
}

/// Test input and independent system readback for the explicitly selected guest case.
#[cfg(windows)]
mod native {
    use super::super::super::broker_observation::{BrokerPowerWatch, read_native_adjustment};
    use super::{
        BrokerInvocation, BrokerObservation, BrokerReading, Clock, HostReceipt, Instant,
        InvocationPoll, LabRole, Observations, Rc, RefCell, StopNotification, TestResult, finish,
        start,
    };
    use std::time::{SystemTime, UNIX_EPOCH};
    use windows_platform_native::{
        ClockAdjustment, ClockCoordinates, ClockReadError, read_clock_adjustment,
        read_clock_coordinates,
    };

    /// The same monotonic source as the real Windows service composition.
    struct LiveClock;

    impl Clock for LiveClock {
        /// Observe host-independent monotonic order inside the guest process.
        fn now(&mut self) -> Instant {
            Instant::now()
        }
    }

    /// Compare to the existing Windows leaf without passing through the broker owner.
    ///
    /// # Errors
    /// Returns the raw Win32 code from the separate reference read.
    fn reference() -> Result<(u64, u64, bool), u32> {
        let state: ClockAdjustment = read_clock_adjustment()?;
        Ok((
            state.adjustment(),
            state.increment(),
            state.periodic_adjustment_disabled(),
        ))
    }

    /// The real host retains the Windows result and encloses it in monotonic time.
    ///
    /// # Errors
    /// Returns a startup, native reference, or receipt acquisition error.
    ///
    /// # Panics
    /// Panics if a stable reference differs from the retained read, or its bracket is invalid.
    #[test]
    #[ignore = "requires an explicitly admitted offline Windows clock-read fixture"]
    fn broker_native_adjustment_observation_matches_stable_reference() -> TestResult {
        let observed: Rc<RefCell<Observations>> = Rc::new(RefCell::default());
        let stop: StopNotification = StopNotification::new();
        let mut clock: LiveClock = LiveClock;
        let mut invocation: BrokerInvocation =
            start(LabRole::ClockBroker, &observed, &mut clock, &stop)?;
        let selected: bool = invocation.select_broker_probe(read_native_adjustment);
        let first: (u64, u64, bool) =
            reference().map_err(|code: u32| format!("first reference read failed: {code}"))?;
        let earliest: Instant = Instant::now();
        let _poll: InvocationPoll = invocation.poll(&mut clock, &stop);
        let latest: Instant = Instant::now();
        let last: Result<(u64, u64, bool), u32> = reference();
        let receipt: HostReceipt = finish(&mut invocation, &mut clock, &stop)?;
        let last: (u64, u64, bool) =
            last.map_err(|code: u32| format!("last reference read failed: {code}"))?;
        assert!(selected);
        let Some(BrokerObservation::Observed {
            before,
            after,
            raw: BrokerReading::Adjustment(raw),
        }) = receipt.broker_observation()
        else {
            return Err(format!(
                "expected successful raw Windows observation, got {:?}",
                receipt.broker_observation()
            )
            .into());
        };
        assert!(earliest <= before && before <= after && after <= latest);
        assert_eq!(receipt.result(), &Ok(()));
        if first == last {
            assert_eq!(
                (
                    raw.adjustment,
                    raw.increment,
                    raw.periodic_adjustment_disabled
                ),
                first
            );
            println!(
                "broker native comparison stable: adjustment={} increment={} disabled={}",
                raw.adjustment, raw.increment, raw.periodic_adjustment_disabled
            );
        } else {
            println!("broker native comparison inconclusive: independent reference changed");
        }
        Ok(())
    }

    /// Convert a live wall-clock comparison read into FILETIME ticks for this test.
    ///
    /// # Errors
    /// Returns a conversion failure if the observed guest date is before `UNIX_EPOCH`
    /// or beyond the raw FILETIME range.
    fn reference_filetime() -> TestResult<u64> {
        let nanos: u128 = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        // FILETIME starts at 1601-01-01. There are 134774 days to UNIX_EPOCH.
        let epoch_offset: u128 = 134_774 * 86_400 * 10_000_000;
        let ticks: u128 = nanos / 100 + epoch_offset;
        Ok(u64::try_from(ticks)?)
    }

    /// Capture one actual native QPC and FILETIME pair through the broker owner.
    ///
    /// # Errors
    /// Returns a fixture setup, reference, native observation, or completion failure.
    ///
    /// # Panics
    /// Panics if a read escapes the outer brackets, has a different frequency,
    /// or shares the one-shot slot with a second observation.
    #[test]
    #[ignore = "requires an explicitly admitted offline Windows clock-read fixture"]
    fn broker_native_coordinate_observation_matches_containing_reads() -> TestResult {
        let watch: BrokerPowerWatch = BrokerPowerWatch::subscribe()
            .map_err(|error| format!("broker power registration failed: {error:?}"))?;
        let observation: TestResult =
            observe_coordinates(&watch).and_then(|()| refuse_spent_generation(&watch));
        let closed: Result<(), windows_platform_native::ResumeUnregistrationError> = watch.close();
        match (observation, closed) {
            (Ok(()), Ok(())) => {
                println!("broker native power subscription closed");
                Ok(())
            }
            (Err(error), Ok(())) => Err(error),
            (Ok(()), Err(cleanup)) => Err(format!("broker power close failed: {cleanup:?}").into()),
            (Err(error), Err(cleanup)) => {
                Err(format!("{error}. Broker power close failed: {cleanup:?}").into())
            }
        }
    }

    /// Read and admit coordinates under an active subscribed generation.
    ///
    /// # Errors
    /// Returns the original fixture, reference read or observation failure.
    ///
    /// # Panics
    /// Panics if the selected read escapes its reference brackets or permits a duplicate.
    fn observe_coordinates(watch: &BrokerPowerWatch) -> TestResult {
        let observed: Rc<RefCell<Observations>> = Rc::new(RefCell::default());
        let stop: StopNotification = StopNotification::new();
        let mut clock: LiveClock = LiveClock;
        let mut invocation: BrokerInvocation =
            start(LabRole::ClockBroker, &observed, &mut clock, &stop)?;
        let selected: bool = invocation.select_clock_coordinates(watch);
        let duplicate: bool = invocation.select_broker_probe(read_native_adjustment);
        let qpc_lower: ClockCoordinates = read_clock_coordinates()
            .map_err(|error: ClockReadError| format!("lower QPC reference failed: {error:?}"))?;
        let wall_lower: u64 = reference_filetime()?;
        let earliest: Instant = Instant::now();
        let _poll: InvocationPoll = invocation.poll(&mut clock, &stop);
        let latest: Instant = Instant::now();
        let wall_upper_result: TestResult<u64> = reference_filetime();
        let qpc_upper_result: Result<ClockCoordinates, ClockReadError> = read_clock_coordinates();
        let receipt: HostReceipt = finish(&mut invocation, &mut clock, &stop)?;
        let wall_upper: u64 = wall_upper_result?;
        let qpc_upper: ClockCoordinates = qpc_upper_result
            .map_err(|error: ClockReadError| format!("upper QPC reference failed: {error:?}"))?;
        assert!(selected);
        assert!(!duplicate);
        let Some(BrokerObservation::Observed {
            before,
            after,
            raw: BrokerReading::ClockCoordinates(raw),
        }) = receipt.broker_observation()
        else {
            return Err(format!(
                "expected a bracketed native coordinate result, got {:?}",
                receipt.broker_observation()
            )
            .into());
        };
        assert_eq!(receipt.result(), &Ok(()));
        assert!(earliest <= before && before <= after && after <= latest);
        assert_eq!(raw.frequency_hz(), qpc_lower.frequency_hz());
        assert_eq!(raw.frequency_hz(), qpc_upper.frequency_hz());
        assert!(qpc_lower.qpc_after() <= raw.qpc_before());
        assert!(raw.qpc_before() <= raw.qpc_after());
        assert!(raw.qpc_after() <= qpc_upper.qpc_before());
        assert!((wall_lower..=wall_upper).contains(&raw.filetime_100ns()));
        println!(
            "broker native coordinates observed: hz={} qpc_lower={} qpc_before={} qpc_after={} qpc_upper={} filetime_lower={} filetime={} filetime_upper={}",
            raw.frequency_hz(),
            qpc_lower.qpc_after(),
            raw.qpc_before(),
            raw.qpc_after(),
            qpc_upper.qpc_before(),
            wall_lower,
            raw.filetime_100ns(),
            wall_upper,
        );
        Ok(())
    }

    /// Exercise the actual Windows coordinate selector with the already spent watch.
    ///
    /// # Errors
    /// Returns the original fixture setup or completion failure.
    ///
    /// # Panics
    /// Panics if the second invocation publishes coordinates from the spent generation.
    fn refuse_spent_generation(watch: &BrokerPowerWatch) -> TestResult {
        let observed: Rc<RefCell<Observations>> = Rc::new(RefCell::default());
        let stop: StopNotification = StopNotification::new();
        let mut clock: LiveClock = LiveClock;
        let mut invocation: BrokerInvocation =
            start(LabRole::ClockBroker, &observed, &mut clock, &stop)?;
        assert!(invocation.select_clock_coordinates(watch));
        let _poll: InvocationPoll = invocation.poll(&mut clock, &stop);
        let receipt: HostReceipt = finish(&mut invocation, &mut clock, &stop)?;
        assert_eq!(receipt.result(), &Ok(()));
        assert_eq!(
            receipt.broker_observation(),
            Some(BrokerObservation::Retired {
                reason: super::Retirement::PowerContext,
                native_failure: None,
            })
        );
        println!("broker native spent power generation refused");
        Ok(())
    }
}

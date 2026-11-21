// SPDX-License-Identifier: GPL-2.0-only
//! Test the selected broker composition through its real host driver.

use std::{
    cell::RefCell,
    error::Error,
    num::NonZeroU64,
    rc::Rc,
    sync::atomic::{AtomicU32, Ordering::SeqCst},
    time::{Duration, Instant},
};

use windows_platform::{
    scm::{ScmStatus, StopNotification},
    service::{LifecycleError, ServiceState, ServiceStatus, StatusReporter},
};

use super::{HealthyTokenObserver, TestClock, Time};
use crate::{
    BrokerClockRead, FixtureError, LabRole, NativeError, NativeOperation,
    broker_clock_change::{
        BrokerClockChanges, ControlDisposition, NativeBrokerReader, deliver, report_status,
    },
    host::{HostReceipt, StatusFactory, Wait, run_broker_service},
};

#[path = "broker_clock_sampling_tests.rs"]
mod sampling_tests;

#[path = "broker_availability_tests.rs"]
mod availability_tests;

/// Independent callback state for the acknowledged host history.
static HEALTHY: BrokerClockChanges = BrokerClockChanges::new();
/// Independent callback state for the refused Running acknowledgement.
static FAILED: BrokerClockChanges = BrokerClockChanges::new();
/// Independent callback state for the initial revision without an early notice.
static QUIET: BrokerClockChanges = BrokerClockChanges::new();
/// Count actual getter calls, independently reset between these fixtures.
static READS: AtomicU32 = AtomicU32::new(0);
/// Distinct last captured raw value.
#[cfg(not(windows))]
const RAW: u64 = 0x7abc_def0_1234_5678;
/// Fixed test-clock advance between the crossing read and the next poll.
const ADVANCE: Duration = Duration::from_millis(23);

/// Raw result from the selected Windows reader.
#[cfg(windows)]
type ReadResult =
    Result<windows_platform_native::ClockCoordinates, windows_platform_native::ClockReadError>;
/// Portable positive and zero-error test inputs to the same receiver.
#[cfg(not(windows))]
type ReadResult = Result<u64, u32>;

/// Read actual native coordinates at the selected invocation's read edge.
/// # Errors
/// Return the original staged native refusal.
#[cfg(windows)]
fn sample(_fail: bool) -> ReadResult {
    windows_platform_native::read_clock_coordinates()
}

/// Script either a useful sample or original native error zero.
/// # Errors
/// Return the selected zero failure.
#[cfg(not(windows))]
fn sample(fail: bool) -> ReadResult {
    if fail { Err(0) } else { Ok(RAW) }
}

/// Run the selected getter across the first notice, then allow a fresh read.
/// # Errors
/// Preserve the original selected getter failure.
/// # Panics
/// Panics if the second broker control cannot reach its running mailbox.
fn read() -> ReadResult {
    let earlier: u32 = READS.fetch_add(1, SeqCst);
    let result: ReadResult = sample(earlier == 0);
    if earlier == 0 {
        assert!(deliver(Some(&HEALTHY), &StopNotification::new(), 16, 0).wake);
    }
    result
}

/// Observe the unmodified initial generation with its marked call count.
/// # Errors
/// Return the original native refusal from the unchanged adapter.
fn read_quiet() -> ReadResult {
    let _previous: u32 = READS.fetch_add(1, SeqCst);
    sample(false)
}

/// The actual registration seam records acknowledgements separately from attempts.
struct Reporting<'a> {
    /// Original registered process-local mailbox.
    changes: &'static BrokerClockChanges,
    /// Both status attempts and delivered notices refer to this source's Stop bit.
    stop: &'a StopNotification,
    /// Independent accepted-control observations.
    masks: &'a RefCell<Vec<(ServiceState, u32)>>,
    /// Whether the scripted Running report acknowledges its attempt.
    accept_running: bool,
    /// Optional control and subtype delivered during the status attempt.
    early_notice: Option<(u32, u32)>,
    /// Actual getter count, observed before acknowledging Running.
    reads: &'static AtomicU32,
}

impl StatusFactory for Reporting<'_> {
    type Reporter = Self;

    /// Bind the reporter only for the broker's fixed role.
    fn register(&mut self, role: LabRole) -> Result<Self::Reporter, NativeError> {
        if role != LabRole::ClockBroker {
            return Err(NativeError::new(NativeOperation::Register, 5));
        }
        Ok(Self {
            changes: self.changes,
            stop: self.stop,
            masks: self.masks,
            accept_running: self.accept_running,
            early_notice: self.early_notice,
            reads: self.reads,
        })
    }
}

impl StatusReporter for Reporting<'_> {
    type Error = NativeError;

    /// Model delivery during `SetServiceStatus` using the actual shared adapter.
    ///
    /// # Errors
    /// Returns the selected Running refusal without manufacturing acknowledgement.
    /// # Panics
    /// Panics if the getter ran before acknowledged Running or the notice was dropped.
    fn report(&mut self, status: ServiceStatus) -> Result<(), NativeError> {
        report_status(Some(self.changes), status.state(), |extra: u32| {
            self.masks.borrow_mut().push((
                status.state(),
                ScmStatus::from_status(status).controls_accepted() | extra,
            ));
            if status.state() == ServiceState::Running {
                assert_eq!(self.reads.load(SeqCst), 0);
                if let Some((raw, event)) = self.early_notice {
                    assert!(deliver(Some(self.changes), self.stop, raw, event).wake);
                }
                if !self.accept_running {
                    return Err(NativeError::new(NativeOperation::Status, 0));
                }
            }
            Ok(())
        })
    }
}

/// Bound the actual host loop to one or two waiting observations.
struct TwoPolls<'a> {
    /// Same sticky Stop notification used by the actual host.
    stop: &'a StopNotification,
    /// Mailbox closed by the same callback branch as the broker handler.
    changes: &'static BrokerClockChanges,
    /// Actual waits observed after the two expected attempts.
    waits: u32,
    /// Number of actual waits this history allows.
    limit: u32,
    /// Shared time advanced only at the first renewal wake.
    time: Rc<RefCell<Time>>,
}

impl Wait for TwoPolls<'_> {
    /// First wait models the retained notice wake. The second delivers Stop.
    /// # Panics
    /// Panics on a renewed deadline, missing getter call, or an unexpected third wait.
    fn park(&mut self, timeout: Duration) {
        self.waits += 1;
        assert!(self.waits <= self.limit);
        let elapsed: Duration = if self.waits == 1 {
            Duration::ZERO
        } else {
            ADVANCE
        };
        assert_eq!(
            timeout.checked_add(elapsed),
            Some(Duration::from_millis(60_000))
        );
        assert_eq!(READS.load(SeqCst), self.waits);
        if self.waits == self.limit {
            assert!(deliver(Some(self.changes), self.stop, 1, 0).wake);
        } else {
            self.time.borrow_mut().current += ADVANCE;
        }
    }
}

/// Admit an early notice, retire crossed work, and require actual re-observation.
/// # Errors
/// Return a missing native observation or wall-clock reference in the Windows fixture.
/// # Panics
/// Panics if privileges/registration/status bypass the host, or capture/history or closure differ.
#[test]
#[cfg_attr(
    windows,
    ignore = "requires an explicitly admitted read-only Windows clock fixture"
)]
fn selected_broker_renews_after_crossing_and_refuses_failed_running() -> Result<(), Box<dyn Error>>
{
    for (changes, accept_running, early_notice) in [
        (&FAILED, false, true),
        (&QUIET, true, false),
        (&HEALTHY, true, true),
    ] {
        READS.store(0, SeqCst);
        let stop: StopNotification = StopNotification::new();
        let masks: RefCell<Vec<(ServiceState, u32)>> = RefCell::default();
        let now: Instant = Instant::now();
        let mut clock: TestClock = TestClock(Rc::new(RefCell::new(Time {
            current: now,
            backwards_once: false,
            backwards_persistent: false,
        })));
        let mut reporter: Reporting<'_> = Reporting {
            changes,
            stop: &stop,
            masks: &masks,
            accept_running,
            early_notice: early_notice.then_some((16, 0)),
            reads: &READS,
        };
        let limit: u32 = if early_notice { 2 } else { 1 };
        let mut wait: TwoPolls<'_> = TwoPolls {
            stop: &stop,
            changes,
            waits: 0,
            limit,
            time: Rc::clone(&clock.0),
        };
        #[cfg(windows)]
        let wall_lower: Option<u64> = reference_filetime();
        let receipt: HostReceipt = run_broker_service(
            0,
            &mut reporter,
            &mut HealthyTokenObserver(LabRole::ClockBroker),
            &mut clock,
            &mut wait,
            &stop,
            NativeBrokerReader::new(changes, if early_notice { read } else { read_quiet }),
        );
        assert!(!deliver(Some(changes), &stop, 16, 0).wake);
        assert_eq!(
            masks
                .borrow()
                .iter()
                .filter(|(state, _)| *state == ServiceState::Running)
                .count(),
            1
        );
        assert!(masks.borrow().iter().all(|(state, mask)| *mask
            == if *state == ServiceState::Running {
                0x241
            } else {
                0
            }));
        if accept_running {
            assert_eq!(receipt.result(), &Ok(()));
            assert_eq!(wait.waits, limit);
            let expected: Instant = if early_notice { now + ADVANCE } else { now };
            let number: Option<NonZeroU64> = NonZeroU64::new(if early_notice { 3 } else { 1 });
            let read: crate::NativeBrokerClockRead = receipt
                .broker_clock_read()
                .ok_or("broker read receipt unavailable")?;
            assert!(matches!(read, BrokerClockRead::Completed {
                revision, before, after, result: Ok(_),
            } if Some(revision) == number && before == expected && after == expected));
            #[cfg(not(windows))]
            assert!(matches!(
                read,
                BrokerClockRead::Completed {
                    result: Ok(RAW),
                    ..
                }
            ));
            #[cfg(windows)]
            if early_notice {
                observe_native(read, wall_lower)?;
            }
        } else {
            assert_eq!(
                receipt.result(),
                &Err(FixtureError::Lifecycle(LifecycleError::Report(
                    NativeError::new(NativeOperation::Status, 0)
                )))
            );
            assert_eq!(READS.load(SeqCst), 0);
            assert_eq!(receipt.broker_clock_read(), None);
        }
    }
    Ok(())
}

/// Separate process-like mailbox for the bounded power history.
static POWER_HISTORY: BrokerClockChanges = BrokerClockChanges::new();
/// A second mailbox for the terminal crossing-read control.
static POWER_CROSSING: BrokerClockChanges = BrokerClockChanges::new();
/// Count actual reads independently of the time-change test.
static POWER_READS: AtomicU32 = AtomicU32::new(0);

/// Observe one raw sample in the renewable fixture.
/// # Errors
/// Preserve the selected getter's original error.
fn read_power() -> ReadResult {
    let _previous: u32 = POWER_READS.fetch_add(1, SeqCst);
    sample(false)
}

/// Deliver a complete suspend/resume pair before the getter returns to its owner.
/// # Errors
/// Preserve the selected getter's original error.
/// # Panics
/// Panics if an injected notice does not reach the original mailbox.
fn read_power_crossing() -> ReadResult {
    let _previous: u32 = POWER_READS.fetch_add(1, SeqCst);
    let result: ReadResult = sample(true);
    let stop: StopNotification = StopNotification::new();
    assert!(deliver(Some(&POWER_CROSSING), &stop, 13, 4).wake);
    assert!(deliver(Some(&POWER_CROSSING), &stop, 13, 18).wake);
    result
}

/// Independent expectation at one real wait, then the next fixture event.
struct PowerStep {
    /// Exact calls issued by the owner before the wait.
    reads: u32,
    /// Raw control number and subtype delivered by the fixture.
    control: (u32, u32),
    /// Expected callback acknowledgement and wake decision.
    response: (u32, bool),
}

/// Drive finite power scenarios through the existing host waiter.
struct PowerWait<'a> {
    /// Exact event/read-count history independent of the implementation.
    steps: std::slice::Iter<'static, PowerStep>,
    /// Broker's registered signal and actual sticky Stop bit.
    changes: &'static BrokerClockChanges,
    /// Original host Stop notification.
    stop: &'a StopNotification,
    /// Fake monotonic source preserving one initial absolute deadline.
    time: Rc<RefCell<Time>>,
    /// Total elapsed in the script.
    elapsed: Duration,
}

impl Wait for PowerWait<'_> {
    /// Observe the fixed deadline before delivering each scripted notice.
    /// # Panics
    /// Panics on missing/extra reads, wrong callback, or a renewed deadline.
    fn park(&mut self, duration: Duration) {
        assert_eq!(
            duration.checked_add(self.elapsed),
            Some(Duration::from_secs(60))
        );
        let step: Option<&PowerStep> = self.steps.next();
        assert!(step.is_some(), "unexpected extra broker wait");
        let Some(step) = step else {
            return;
        };
        assert_eq!(POWER_READS.load(SeqCst), step.reads);
        let response: ControlDisposition = deliver(
            Some(self.changes),
            self.stop,
            step.control.0,
            step.control.1,
        );
        assert_eq!((response.status, response.wake), step.response);
        self.elapsed += ADVANCE;
        self.time.borrow_mut().current += ADVANCE;
    }
}

/// Two complete delivered power cycles followed by an unpaired automatic resume.
const HISTORY: &[PowerStep] = &[
    PowerStep {
        reads: 0,
        control: (16, 0),
        response: (0, true),
    },
    PowerStep {
        reads: 0,
        control: (13, 18),
        response: (0, true),
    },
    PowerStep {
        reads: 1,
        control: (13, 4),
        response: (0, true),
    },
    PowerStep {
        reads: 1,
        control: (13, 18),
        response: (0, true),
    },
    PowerStep {
        reads: 2,
        control: (13, 7),
        response: (120, false),
    },
    PowerStep {
        reads: 2,
        control: (13, 18),
        response: (0, true),
    },
    PowerStep {
        reads: 3,
        control: (1, 0),
        response: (0, true),
    },
];
/// Observe the crossed attempt before any fresh revision can replace its record.
const CROSSED: &[PowerStep] = &[PowerStep {
    reads: 1,
    control: (1, 0),
    response: (0, true),
}];

/// Run the original host through repeated sleep contexts without renewing its deadline.
/// # Errors
/// Return a missing read or independent native reference from the fixture.
/// # Panics
/// Panics if actual callback admission, observations or terminal retirement differ.
#[test]
#[cfg_attr(
    windows,
    ignore = "requires an explicitly admitted read-only Windows clock fixture"
)]
fn selected_broker_power_resumes_across_repeated_callbacks() -> Result<(), Box<dyn Error>> {
    for crossing in [true, false] {
        POWER_READS.store(0, SeqCst);
        let (changes, steps): (&'static BrokerClockChanges, &'static [PowerStep]) = if crossing {
            (&POWER_CROSSING, CROSSED)
        } else {
            (&POWER_HISTORY, HISTORY)
        };
        let stop: StopNotification = StopNotification::new();
        let masks: RefCell<Vec<(ServiceState, u32)>> = RefCell::default();
        let now: Instant = Instant::now();
        let mut clock: TestClock = TestClock(Rc::new(RefCell::new(Time {
            current: now,
            backwards_once: false,
            backwards_persistent: false,
        })));
        let mut reporter: Reporting<'_> = Reporting {
            changes,
            stop: &stop,
            masks: &masks,
            accept_running: true,
            early_notice: (!crossing).then_some((13, 4)),
            reads: &POWER_READS,
        };
        let mut wait: PowerWait<'_> = PowerWait {
            steps: steps.iter(),
            changes,
            stop: &stop,
            time: Rc::clone(&clock.0),
            elapsed: Duration::ZERO,
        };
        #[cfg(windows)]
        let wall_lower: Option<u64> = reference_filetime();
        let receipt: HostReceipt = run_broker_service(
            0,
            &mut reporter,
            &mut HealthyTokenObserver(LabRole::ClockBroker),
            &mut clock,
            &mut wait,
            &stop,
            NativeBrokerReader::new(
                changes,
                if crossing {
                    read_power_crossing
                } else {
                    read_power
                },
            ),
        );
        assert!(wait.steps.as_slice().is_empty());
        assert_eq!(receipt.result(), &Ok(()));
        assert_eq!(deliver(Some(changes), &stop, 13, 18).status, 120);
        let read: crate::NativeBrokerClockRead = receipt
            .broker_clock_read()
            .ok_or("missing power observation receipt")?;
        if crossing {
            #[cfg(not(windows))]
            let native_failure: Option<u32> = Some(0);
            #[cfg(windows)]
            let native_failure: Option<windows_platform_native::ClockReadError> = None;
            assert_eq!(
                read,
                BrokerClockRead::Retired {
                    revision: NonZeroU64::MIN,
                    reason: crate::ClockReadRetirement::Generation,
                    native_failure
                }
            );
        } else {
            let expected: Instant = now + ADVANCE * 6;
            assert!(
                matches!(read, BrokerClockRead::Completed { revision, before, after, result: Ok(_) }
                if revision.get() == 7 && before == expected && after == expected)
            );
            #[cfg(windows)]
            observe_native(read, wall_lower)?;
        }
    }
    Ok(())
}

/// Get a separate Windows wall-clock read in unsigned FILETIME ticks.
#[cfg(windows)]
fn reference_filetime() -> Option<u64> {
    let nanos: u128 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_nanos();
    let offset: u128 = 134_774 * 86_400 * 10_000_000;
    u64::try_from(nanos / 100 + offset).ok()
}

/// Read back the independently bounded final native bracket.
/// # Errors
/// Return an unavailable native bracket or absent wall reference.
/// # Panics
/// Panics if the retained native coordinates escape their wall references.
#[cfg(windows)]
fn observe_native(
    read: crate::NativeBrokerClockRead,
    lower: Option<u64>,
) -> Result<(), Box<dyn Error>> {
    let lower: u64 = lower.ok_or("lower wall reference unavailable")?;
    let upper: u64 = reference_filetime().ok_or("upper wall reference unavailable")?;
    let BrokerClockRead::Completed {
        result: Ok(raw), ..
    } = read
    else {
        return Err("native coordinate observation unavailable".into());
    };
    assert!((lower..=upper).contains(&raw.filetime_100ns()));
    println!(
        "broker renewal observed: hz={} qpc_before={} qpc_after={} filetime_lower={} filetime={} filetime_upper={}",
        raw.frequency_hz(),
        raw.qpc_before(),
        raw.qpc_after(),
        lower,
        raw.filetime_100ns(),
        upper
    );
    Ok(())
}

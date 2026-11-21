// SPDX-License-Identifier: GPL-2.0-only
//! Exercise the quiet sampler through the actual serialized broker host.

use std::{
    cell::RefCell,
    error::Error,
    rc::Rc,
    time::{Duration, Instant},
};
#[cfg(not(windows))]
use std::{num::NonZeroU64, sync::atomic::Ordering::SeqCst};

use super::{
    BrokerClockChanges, BrokerClockRead, HealthyTokenObserver, HostReceipt, LabRole, Reporting,
    StopNotification, TestClock, Time, Wait, deliver, run_broker_service,
};
#[cfg(windows)]
use super::{observe_native, reference_filetime};
#[cfg(windows)]
use crate::broker_clock_change::sampled_native_reader;
#[cfg(not(windows))]
use crate::{
    SampledClockBreak, SampledClockDiscontinuity, broker_clock_change::NativeBrokerReader,
};

#[cfg(not(windows))]
#[path = "broker_clock_sampling_cutoff_tests.rs"]
mod cutoff_tests;

/// Separate process-like receiver for the quiet sampler.
static CHANGES: BrokerClockChanges = BrokerClockChanges::new();
/// Scripted raw getter invocations on the GNU fixture.
static READS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// Produce a healthy baseline, a backward value, then a lower fresh anchor.
/// # Errors
/// Returns an unexpected-call marker outside the finite script.
#[cfg(not(windows))]
fn read() -> Result<u64, u32> {
    match READS.fetch_add(1, SeqCst) {
        0 => Ok(100),
        1 => Ok(90),
        2 => Ok(20),
        _ => Err(99),
    }
}

/// Bound the actual wait seam and advance controlled owner time.
struct SampleWait<'a> {
    /// The original host Stop signal.
    stop: &'a StopNotification,
    /// The same controlled clock observed by the serialized host.
    time: Rc<RefCell<Time>>,
    /// Number of waits already observed.
    next: usize,
}

impl Wait for SampleWait<'_> {
    /// A spurious wake, an exact due time, a missed period and Stop.
    /// # Panics
    /// Fails on drift from the checked cadence or an unexpected read or wait.
    fn park(&mut self, timeout: Duration) {
        const STEPS: [(u64, u64, u32); 4] = [
            (1_000, 250, 1),
            (750, 750, 1),
            (1_000, 5_000, 2),
            (1_000, 0, 3),
        ];
        let step: Option<&(u64, u64, u32)> = STEPS.get(self.next);
        assert!(step.is_some(), "unexpected extra sample wait");
        let Some(step) = step else {
            return;
        };
        assert_eq!(timeout, Duration::from_millis(step.0));
        #[cfg(not(windows))]
        assert_eq!(READS.load(SeqCst), step.2);
        self.next += 1;
        self.time.borrow_mut().current += Duration::from_millis(step.1);
        if self.next == STEPS.len() {
            assert!(deliver(Some(&CHANGES), self.stop, 1, 0).wake);
        }
    }
}

/// Reobserve quietly without a burst and retain the reason across a fresh anchor.
/// # Errors
/// Returns a missing native receipt or reference error.
/// # Panics
/// Fails if the host does extra work, loses a break, or mislabels a new revision.
#[test]
#[cfg_attr(
    windows,
    ignore = "requires an explicitly admitted read-only Windows clock fixture"
)]
fn selected_broker_samples_quietly_and_retains_observed_breaks() -> Result<(), Box<dyn Error>> {
    let start: Instant = Instant::now();
    let stop: StopNotification = StopNotification::new();
    let masks: RefCell<Vec<(super::ServiceState, u32)>> = RefCell::default();
    let time: Rc<RefCell<Time>> = Rc::new(RefCell::new(Time {
        current: start,
        backwards_once: false,
        backwards_persistent: false,
    }));
    let mut clock: TestClock = TestClock(Rc::clone(&time));
    let mut factory: Reporting<'_> = Reporting {
        changes: &CHANGES,
        stop: &stop,
        masks: &masks,
        accept_running: true,
        early_notice: None,
        reads: &READS,
    };
    let mut wait: SampleWait<'_> = SampleWait {
        stop: &stop,
        time,
        next: 0,
    };
    #[cfg(not(windows))]
    let reader: NativeBrokerReader =
        NativeBrokerReader::new(&CHANGES, read).with_sampling(|before: &u64, after: &u64| {
            (after < before).then_some(SampledClockDiscontinuity::FiletimeReversed)
        });
    #[cfg(windows)]
    let wall_lower: Option<u64> = reference_filetime();
    #[cfg(windows)]
    let reader = sampled_native_reader(&CHANGES);
    let receipt: HostReceipt = run_broker_service(
        0,
        &mut factory,
        &mut HealthyTokenObserver(LabRole::ClockBroker),
        &mut clock,
        &mut wait,
        &stop,
        reader,
    );
    assert_eq!(wait.next, 4);
    assert_eq!(receipt.result(), &Ok(()));
    let last: crate::NativeBrokerClockRead = receipt
        .broker_clock_read()
        .ok_or("missing quiet sample receipt")?;
    let expected: Instant = start + Duration::from_secs(6);
    assert!(
        matches!(last, BrokerClockRead::Completed { before, after, .. } if before == expected && after == expected)
    );
    #[cfg(not(windows))]
    {
        assert!(
            matches!(last, BrokerClockRead::Completed { revision, result: Ok(20), .. } if revision.get() == 2)
        );
        assert_eq!(
            receipt.broker_clock_break(),
            Some(SampledClockBreak {
                revision: NonZeroU64::MIN,
                observed_at: start + Duration::from_secs(1),
                reason: SampledClockDiscontinuity::FiletimeReversed
            })
        );
        assert_eq!(READS.load(SeqCst), 3);
    }
    #[cfg(windows)]
    {
        assert!(
            matches!(last, BrokerClockRead::Completed { revision, result: Ok(_), .. } if revision.get() == 1)
        );
        assert_eq!(receipt.broker_clock_break(), None);
        observe_native(last, wall_lower)?;
    }
    Ok(())
}

// SPDX-License-Identifier: GPL-2.0-only
//! Exercise the original Running cutoff on both edges of the actual quiet sampler.

use std::{
    cell::{Cell, RefCell},
    error::Error,
    rc::Rc,
    sync::atomic::{AtomicU32, Ordering::SeqCst},
    time::{Duration, Instant},
};

use windows_platform::{scm::StopNotification, service::ServiceState};

use super::{
    BrokerClockChanges, BrokerClockRead, HealthyTokenObserver, HostReceipt, LabRole,
    NativeBrokerReader, Reporting, Wait, run_broker_service,
};
use crate::{ClockReadRetirement, FixtureError, host::Clock};

/// Process-like mailbox for expiry before a due read.
static BEFORE: BrokerClockChanges = BrokerClockChanges::new();
/// Separate mailbox for expiry during a due read.
static AFTER: BrokerClockChanges = BrokerClockChanges::new();
/// Getter calls observed separately from the owner clock and receipt.
static READS: AtomicU32 = AtomicU32::new(0);

/// Supply the original successful anchor followed by an original failure code.
/// # Errors
/// Return the scripted failure code zero on the second call.
fn read() -> Result<u64, u32> {
    match READS.fetch_add(1, SeqCst) {
        0 => Ok(100),
        _ => Err(0),
    }
}

/// Restore explicit completion timestamps from a finite scripted getter sequence.
struct CutoffClock {
    /// Shared current owner time, advanced by the waiter or this source.
    time: Rc<Cell<Instant>>,
    /// Known timestamp one second before the immutable Running cutoff.
    penultimate: Instant,
    /// Exact original cutoff.
    cutoff: Instant,
    /// Select the pre-call coincidence or the in-call expiry.
    due_at_cutoff: bool,
}

impl Clock for CutoffClock {
    /// Simulate a slow first anchor or a second read returning at the cutoff.
    fn now(&mut self) -> Instant {
        match (self.due_at_cutoff, READS.load(SeqCst)) {
            (true, 1) if self.time.get() < self.penultimate => self.time.set(self.penultimate),
            (false, 2) => self.time.set(self.cutoff),
            _ => (),
        }
        self.time.get()
    }
}

/// Deliver exactly one timed wake through the real host loop.
struct DueWait {
    /// Shared owner clock state.
    time: Rc<Cell<Instant>>,
    /// Count actual requested parks.
    waits: u32,
}

impl Wait for DueWait {
    /// Advance to the next due timestamp without granting a new Running allowance.
    /// # Panics
    /// Fails if a second park or a longer park conceals the expired work.
    fn park(&mut self, duration: Duration) {
        assert_eq!(self.waits, 0);
        assert_eq!(duration, Duration::from_secs(1));
        self.waits += 1;
        self.time.set(self.time.get() + Duration::from_secs(1));
    }

    /// The inert shutdown should finish before it needs any indefinite wait.
    /// # Panics
    /// Fails promptly if an expired sample keeps the fixture waiting.
    #[expect(
        clippy::panic,
        reason = "an indefinite park is a failure of the inert cutoff control"
    )]
    fn park_until_notified(&mut self) {
        panic!("inert cutoff must finish without an overdue wait");
    }
}

/// The cutoff wins over a due sample and over completion returning at equality.
/// # Errors
/// Returns a missing receipt or historical anchor in either fixture.
/// # Panics
/// Fails on an extra read, late success, changed failure or missing terminal status.
#[test]
fn sampled_broker_refuses_work_at_the_original_cutoff() -> Result<(), Box<dyn Error>> {
    for (changes, due_at_cutoff) in [(&BEFORE, true), (&AFTER, false)] {
        READS.store(0, SeqCst);
        let start: Instant = Instant::now();
        let stop: StopNotification = StopNotification::new();
        let masks: RefCell<Vec<(ServiceState, u32)>> = RefCell::default();
        let time: Rc<Cell<Instant>> = Rc::new(Cell::new(start));
        let mut clock: CutoffClock = CutoffClock {
            time: Rc::clone(&time),
            penultimate: start + Duration::from_secs(59),
            cutoff: start + Duration::from_secs(60),
            due_at_cutoff,
        };
        let mut wait: DueWait = DueWait { time, waits: 0 };
        let mut reporting: Reporting<'_> = Reporting {
            changes,
            stop: &stop,
            masks: &masks,
            accept_running: true,
            early_notice: None,
            reads: &READS,
        };
        let receipt: HostReceipt = run_broker_service(
            0,
            &mut reporting,
            &mut HealthyTokenObserver(LabRole::ClockBroker),
            &mut clock,
            &mut wait,
            &stop,
            NativeBrokerReader::new(changes, read).with_sampling(|_, _| None),
        );
        assert_eq!(wait.waits, 1);
        assert_eq!(receipt.result(), &Err(FixtureError::RunLimitElapsed));
        assert_eq!(receipt.broker_clock_break(), None);
        let last: crate::NativeBrokerClockRead = receipt
            .broker_clock_read()
            .ok_or("missing cutoff receipt")?;
        if due_at_cutoff {
            assert_eq!(READS.load(SeqCst), 1);
            assert!(
                matches!(last, BrokerClockRead::Completed { before, after, result: Ok(100), .. }
                if before == start && after == clock.penultimate)
            );
        } else {
            assert_eq!(READS.load(SeqCst), 2);
            assert!(matches!(
                last,
                BrokerClockRead::Retired {
                    reason: ClockReadRetirement::Deadline,
                    native_failure: Some(0),
                    ..
                }
            ));
        }
    }
    Ok(())
}

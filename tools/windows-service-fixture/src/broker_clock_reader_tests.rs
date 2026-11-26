// SPDX-License-Identifier: GPL-2.0-only
//! Deterministic broker clock controls kept outside the measured implementation.

use std::{
    num::NonZeroU64,
    sync::atomic::{AtomicU32, Ordering::SeqCst},
    time::{Duration, Instant},
};

use windows_platform::{scm::StopNotification, service::ServiceState};

use super::{
    BrokerClockChanges, BrokerClockRead, BrokerClockReader, ClockNotice, ClockReadRetirement,
    ClockRevision, ControlDisposition, ReplyRevision, SAMPLE_INTERVAL, SampledClockDiscontinuity,
    SampledClockTransition, TIME_CHANGE, deliver, report_status,
};

/// A bound reporter advertises only its live mask and retains early delivery.
///
/// # Panics
/// Fails if an unsupported input is accepted, retirement reopens, or counters wrap.
#[test]
fn time_change_controls_are_role_and_lifecycle_bound() {
    let changes: BrokerClockChanges = BrokerClockChanges::new();
    let stop: StopNotification = StopNotification::new();
    assert_eq!(deliver(Some(&changes), &stop, 16, 0).status, 120);
    assert_eq!(deliver(None, &stop, 16, 0).status, 120);
    let start: Result<(), ()> =
        report_status(Some(&changes), ServiceState::StartPending, |extra: u32| {
            assert_eq!(extra, 0);
            assert!(!deliver(Some(&changes), &stop, 16, 0).wake);
            Ok(())
        });
    assert_eq!(start, Ok(()));
    let failed: Result<(), u32> =
        report_status(Some(&changes), ServiceState::Running, |extra: u32| {
            assert_eq!(extra, 0x240);
            assert!(deliver(Some(&changes), &stop, 16, 0).wake);
            assert_eq!(changes.revision().map(|r| r.number()), NonZeroU64::new(2));
            Err(0)
        });
    assert_eq!(failed, Err(0));
    assert!(changes.revision().is_none());
    assert!(!changes.arm());
    assert_eq!(deliver(Some(&changes), &stop, 16, 0).status, 120);
    let last: BrokerClockChanges = BrokerClockChanges::new();
    last.revision.store(u64::MAX - 1, SeqCst);
    assert!(last.notify(ClockNotice::TimeChange));
    assert!(last.revision().is_none());
    assert!(!last.notify(ClockNotice::TimeChange));
    assert!(!last.arm());
}

/// One independent fixture source dedicated to the read-failure control.
static CROSSING: BrokerClockChanges = BrokerClockChanges::new();

/// Return a typed zero error while a new notice crosses the original call.
///
/// # Errors
/// Always return the original scripted error code zero.
///
/// # Panics
/// Fails if the synthetic notification does not reach the original mailbox.
fn crossing_failure() -> Result<u64, u32> {
    assert!(CROSSING.notify(ClockNotice::TimeChange));
    Err(0)
}

/// A crossing read preserves its actual error and spent revision.
///
/// # Panics
/// Fails if read completion or history is invented after replacement or close.
#[test]
fn crossing_read_keeps_the_original_failure_under_its_revision() {
    let mut reader: BrokerClockReader<u64, u32> =
        BrokerClockReader::new(&CROSSING, crossing_failure);
    assert!(CROSSING.arm());
    let now: Instant = Instant::now();
    reader.poll(now, || Ok(now));
    let historical: Option<BrokerClockRead<u64, u32>> = Some(BrokerClockRead::Retired {
        revision: NonZeroU64::MIN,
        reason: ClockReadRetirement::Generation,
        native_failure: Some(0),
    });
    assert_eq!(reader.history(), historical);
    reader.close();
    reader.poll(now, || Ok(now));
    assert_eq!(reader.history(), historical);

    notice_before_getter_retires_without_a_read();
}

/// A separate mailbox for a notice delivered at the opening decision edge.
static BEFORE_READ: BrokerClockChanges = BrokerClockChanges::new();
/// Count any getter invocation under that fixture's retired revision.
static BEFORE_READ_CALLS: AtomicU32 = AtomicU32::new(0);

/// Return an observable successful sample if the opening fence is missed.
///
/// # Errors
/// Returns the unexpected-read marker after the first getter invocation.
fn read_at_opening_edge() -> Result<u64, u32> {
    match BEFORE_READ_CALLS.fetch_add(1, SeqCst) {
        0 => Ok(80),
        _ => Err(99),
    }
}

/// A successful edge timestamp cannot authorize a getter on the retired revision.
///
/// # Panics
/// Panics if the original read runs, the newer revision is displaced, or
/// the original attempt publishes a bracket instead of a generation refusal.
fn notice_before_getter_retires_without_a_read() {
    assert!(BEFORE_READ.arm());
    let now: Instant = Instant::now();
    let mut reader: BrokerClockReader<u64, u32> =
        BrokerClockReader::new(&BEFORE_READ, read_at_opening_edge).with_sampling(|_, _| None);
    let mut edges: u32 = 0;
    reader.poll(now, || {
        edges += 1;
        assert!(BEFORE_READ.notify(ClockNotice::TimeChange));
        Ok(now)
    });
    assert_eq!(edges, 1);
    assert_eq!(BEFORE_READ_CALLS.load(SeqCst), 0);
    assert_eq!(
        BEFORE_READ.revision().as_ref().map(ClockRevision::number),
        NonZeroU64::new(2)
    );
    assert_eq!(reader.sample_wait(now), Some(Duration::ZERO));
    assert_eq!(
        reader.history(),
        Some(BrokerClockRead::Retired {
            revision: NonZeroU64::MIN,
            reason: ClockReadRetirement::Generation,
            native_failure: None,
        })
    );
}

/// Separate registered state for a concurrent-notice fixture.
static NOTICE_RACE: BrokerClockChanges = BrokerClockChanges::new();
/// Mark the actual two native-like calls in the concurrently replaced revision.
static RACE_READS: AtomicU32 = AtomicU32::new(0);

/// Supply ordered synthetic wall ticks with an observed regression.
/// # Errors
/// Reject a read outside the two scripted getter calls.
fn race_read() -> Result<u64, u32> {
    match RACE_READS.fetch_add(1, SeqCst) {
        0 => Ok(80),
        1 => Ok(70),
        _ => Err(99),
    }
}

/// A concurrent real notice wins over a sampled invalidation at commit.
/// # Panics
/// Fails if the sampler replaces a delivered revision or records an uncommitted break.
#[test]
fn newer_notice_wins_over_a_sampled_contradiction() {
    assert!(NOTICE_RACE.arm());
    let now: Instant = Instant::now();
    let due: Instant = now + SAMPLE_INTERVAL;
    let mut reader: BrokerClockReader<u64, u32> = BrokerClockReader::new(&NOTICE_RACE, race_read)
        .with_sampling(|a: &u64, b: &u64| {
            (b < a).then_some(SampledClockTransition::Discontinuity(
                SampledClockDiscontinuity::FiletimeReversed,
            ))
        });
    reader.poll(now, || Ok(now));
    let mut edges: u32 = 0;
    reader.poll(due, || {
        edges += 1;
        if edges == 2 {
            assert!(NOTICE_RACE.notify(ClockNotice::TimeChange));
        }
        Ok(due)
    });
    assert_eq!(RACE_READS.load(SeqCst), 2);
    assert_eq!(
        NOTICE_RACE.revision().map(|revision| revision.number()),
        NonZeroU64::new(2)
    );
    assert_eq!(reader.last_discontinuity(), None);
    assert_eq!(reader.sample_wait(due), Some(Duration::ZERO));
    assert_eq!(
        reader.history(),
        Some(BrokerClockRead::Retired {
            revision: NonZeroU64::MIN,
            reason: ClockReadRetirement::Generation,
            native_failure: None
        })
    );
}

/// Separate live owner for the gap case.
static GAP: BrokerClockChanges = BrokerClockChanges::new();
/// Keep the marker sequence local to that owner.
static GAP_READS: AtomicU32 = AtomicU32::new(0);

/// A failure interrupts comparable captures even when later values are lower.
/// # Errors
/// Preserve the original zero from the second attempted call.
fn gap_read() -> Result<u64, u32> {
    match GAP_READS.fetch_add(1, SeqCst) {
        0 => Ok(100),
        1 => Err(0),
        2 => Err(83),
        3 => Ok(99),
        _ => Err(99),
    }
}

/// A failed getter revokes its baseline but retains the failed observation.
/// # Errors
/// Returns the missing-revision assertion if either healthy read is unavailable.
/// # Panics
/// Fails if the gap leaves a live revision, renews cadence, or loses raw evidence.
#[test]
fn native_failure_revokes_the_sample_pair() -> Result<(), &'static str> {
    assert!(GAP.arm());
    let now: Instant = Instant::now();
    let before: Instant = now + SAMPLE_INTERVAL;
    let after: Instant = before + Duration::from_millis(1);
    let retry: Instant = after + SAMPLE_INTERVAL;
    let fresh: Instant = retry + SAMPLE_INTERVAL;
    let mut reader: BrokerClockReader<u64, u32> = BrokerClockReader::new(&GAP, gap_read)
        .with_sampling(|a: &u64, b: &u64| {
            (b < a).then_some(SampledClockTransition::Discontinuity(
                SampledClockDiscontinuity::FiletimeReversed,
            ))
        });
    reader.poll(now, || Ok(now));
    let original: ClockRevision<'_> = reader.observed_awake_revision().ok_or("missing baseline")?;
    assert!(original.admits());
    let mut edges: std::array::IntoIter<Instant, 2> = [before, after].into_iter();
    reader.poll(before, || edges.next().ok_or(ClockReadRetirement::Clock));
    assert!(!original.admits());
    let failed: Option<BrokerClockRead<u64, u32>> = Some(BrokerClockRead::Completed {
        revision: original.number(),
        before,
        after,
        result: Err(0),
    });
    assert_eq!(reader.history(), failed);
    assert!(reader.observed_awake_revision().is_none());
    let replacement: ClockRevision<'_> = GAP.revision().ok_or("missing replacement")?;
    assert_eq!(Some(replacement.number()), NonZeroU64::new(2));

    let just_early: Instant = retry
        .checked_sub(Duration::from_nanos(1))
        .ok_or("missing pre-due instant")?;
    assert_eq!(
        reader.sample_wait(just_early),
        Some(Duration::from_nanos(1))
    );
    reader.poll(just_early, || Ok(just_early));
    assert_eq!(GAP_READS.load(SeqCst), 2);
    assert_eq!(reader.history(), failed);

    reader.poll(retry, || Ok(retry));
    assert_eq!(
        reader.history(),
        Some(BrokerClockRead::Completed {
            revision: replacement.number(),
            before: retry,
            after: retry,
            result: Err(83),
        })
    );
    assert!(replacement.admits());
    assert!(reader.observed_awake_revision().is_none());
    reader.poll(fresh, || Ok(fresh));
    assert_eq!(
        reader.history(),
        Some(BrokerClockRead::Completed {
            revision: replacement.number(),
            before: fresh,
            after: fresh,
            result: Ok(99),
        })
    );
    assert_eq!(reader.last_discontinuity(), None);
    assert_eq!(GAP_READS.load(SeqCst), 4);
    let observed: ClockRevision<'_> = reader
        .observed_awake_revision()
        .ok_or("missing fresh observation")?;
    assert_eq!(observed.number(), replacement.number());
    assert!(observed.admits());
    assert!(!original.admits());
    Ok(())
}

/// Count the actual getter calls for the sequential notice and Stop controls.
static GAP_RACE_READS: AtomicU32 = AtomicU32::new(0);

/// Supply a successful baseline followed by an original failed observation.
/// # Errors
/// Return code zero on the second getter call of each fixture history.
fn gap_race_read() -> Result<u64, u32> {
    match GAP_RACE_READS.fetch_add(1, SeqCst) {
        0 => Ok(100),
        1 => Err(0),
        _ => Err(99),
    }
}

/// An independent control can revoke the failed getter's baseline first.
/// # Panics
/// Fails if the sampler advances the winner's revision or loses the raw error.
#[test]
fn notice_or_stop_wins_over_a_sampled_getter_failure() {
    static NOTICE: BrokerClockChanges = BrokerClockChanges::new();
    static STOP: BrokerClockChanges = BrokerClockChanges::new();
    let cases: [(&BrokerClockChanges, u32, Option<NonZeroU64>); 2] =
        [(&NOTICE, TIME_CHANGE, NonZeroU64::new(2)), (&STOP, 1, None)];
    for (changes, control, expected) in cases {
        GAP_RACE_READS.store(0, SeqCst);
        assert!(changes.arm());
        let stop: StopNotification = StopNotification::new();
        let now: Instant = Instant::now();
        let due: Instant = now + SAMPLE_INTERVAL;
        let mut reader: BrokerClockReader<u64, u32> =
            BrokerClockReader::new(changes, gap_race_read).with_sampling(|a: &u64, b: &u64| {
                (b < a).then_some(SampledClockTransition::Discontinuity(
                    SampledClockDiscontinuity::FiletimeReversed,
                ))
            });
        reader.poll(now, || Ok(now));
        assert!(reader.observed_awake_revision().is_some());
        let mut edges: u32 = 0;
        reader.poll(due, || {
            edges += 1;
            if edges == 2 {
                let notice: ControlDisposition = deliver(Some(changes), &stop, control, 0);
                assert_eq!(notice.status, 0);
                assert!(notice.wake);
            }
            Ok(due)
        });
        assert_eq!(GAP_RACE_READS.load(SeqCst), 2);
        assert_eq!(
            changes
                .revision()
                .map(|revision: ClockRevision<'_>| revision.number()),
            expected
        );
        assert_eq!(stop.stop_requested(), control == 1);
        assert_eq!(
            reader.history(),
            Some(BrokerClockRead::Retired {
                revision: NonZeroU64::MIN,
                reason: ClockReadRetirement::Generation,
                native_failure: Some(0),
            })
        );
        assert!(reader.observed_awake_revision().is_none());
        assert_eq!(reader.last_discontinuity(), None);
    }
}

/// Count actual reads across the sequential terminal-edge controls.
static TERMINAL_EDGE_READS: AtomicU32 = AtomicU32::new(0);

/// Supply a valid raw baseline, then the original error from a closing read.
/// # Errors
/// Returns the literal native error code zero after the valid baseline.
fn terminal_edge_read() -> Result<u64, u32> {
    match TERMINAL_EDGE_READS.fetch_add(1, SeqCst) {
        0 => Ok(100),
        _ => Err(0),
    }
}

/// An invocation-terminal edge withdraws previous authority before host cleanup.
/// # Errors
/// Returns the missing-baseline assertion if the preceding healthy read was refused.
/// # Panics
/// Fails if retirement leaves a revision current, changes history or retries work.
#[test]
fn terminal_read_edge_withdraws_the_original_revision_immediately() -> Result<(), &'static str> {
    static STOP: BrokerClockChanges = BrokerClockChanges::new();
    static DEADLINE: BrokerClockChanges = BrokerClockChanges::new();
    static CLOCK: BrokerClockChanges = BrokerClockChanges::new();
    let cases: [(&BrokerClockChanges, ClockReadRetirement, Option<u32>); 3] = [
        (&STOP, ClockReadRetirement::Stop, None),
        (&DEADLINE, ClockReadRetirement::Deadline, Some(0)),
        (&CLOCK, ClockReadRetirement::Clock, Some(0)),
    ];
    for (changes, reason, native_failure) in cases {
        TERMINAL_EDGE_READS.store(0, SeqCst);
        assert!(changes.arm());
        let now: Instant = Instant::now();
        let due: Instant = now + SAMPLE_INTERVAL;
        let mut reader: BrokerClockReader<u64, u32> =
            BrokerClockReader::new(changes, terminal_edge_read).with_sampling(|_, _| None);
        reader.poll(now, || Ok(now));
        let original: ClockRevision<'_> =
            reader.observed_awake_revision().ok_or("missing baseline")?;
        let retained: ReplyRevision = original.for_status_reply();
        assert!(retained.is_current());

        let mut edges: u32 = 0;
        let mut replacement: Option<ClockRevision<'_>> = None;
        reader.poll(due, || {
            edges += 1;
            if native_failure.is_some() && edges == 1 {
                Ok(due)
            } else {
                if reason == ClockReadRetirement::Clock {
                    assert!(changes.notify(ClockNotice::TimeChange));
                    replacement = changes.revision();
                    assert!(replacement.as_ref().is_some_and(ClockRevision::admits));
                }
                Err(reason)
            }
        });
        let retired: Option<BrokerClockRead<u64, u32>> = Some(BrokerClockRead::Retired {
            revision: original.number(),
            reason,
            native_failure,
        });
        assert_eq!(reader.history(), retired);
        assert!(!retained.is_current());
        if let Some(newer) = replacement {
            assert_ne!(newer.number(), original.number());
            assert!(!newer.admits());
        }
        assert!(!changes.notify(ClockNotice::Resume));
        assert!(!changes.arm());
        assert_eq!(reader.sample_wait(due), None);
        let later: Instant = due + SAMPLE_INTERVAL;
        reader.poll(later, || Ok(later));
        assert_eq!(reader.history(), retired);
        assert_eq!(TERMINAL_EDGE_READS.load(SeqCst), edges);
    }
    Ok(())
}

/// The last valid awake revision can retire without wrapping.
/// # Errors
/// Returns the missing-fence assertion if the largest legal state is unavailable.
/// # Panics
/// Fails if the penultimate revision cannot advance or the final revision reopens.
#[test]
fn sampled_invalidation_closes_at_revision_exhaustion() -> Result<(), &'static str> {
    let changes: BrokerClockChanges = BrokerClockChanges::new();
    changes.revision.store(u64::MAX - 3, SeqCst);
    let previous = changes.revision().ok_or("missing penultimate revision")?;
    assert_eq!(previous.invalidate(), Ok(NonZeroU64::new(u64::MAX >> 1)));
    let last = changes.revision().ok_or("missing last revision")?;
    assert_eq!(last.invalidate(), Ok(None));
    assert!(!last.admits());
    assert!(!changes.arm());
    assert!(changes.revision().is_none());

    Ok(())
}

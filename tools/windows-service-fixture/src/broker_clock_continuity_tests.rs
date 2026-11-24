// SPDX-License-Identifier: GPL-2.0-only
//! Deterministic broker clock controls kept outside the measured implementation.

use std::{
    num::NonZeroU64,
    sync::atomic::{AtomicU32, Ordering::SeqCst},
    time::{Duration, Instant},
};

use super::{
    super::{
        BrokerClockChanges, BrokerClockRead, BrokerClockReader, ClockReadRetirement, ClockRevision,
        SampledClockBreak,
    },
    ClockSample, SampledClockDiscontinuity, SampledClockTransition, compare, compare_for_sampling,
};

/// Distinct raw words and a mode for the portable comparator controls.
#[derive(Clone, Copy, Eq, PartialEq)]
struct Adjustment {
    /// Scripted adjustment word.
    update_word: u64,
    /// Scripted increment word.
    increment: u64,
    /// Scripted disabled-mode flag.
    periodic_adjustment_disabled: bool,
}

/// Independently named process modes for portable comparator controls.
#[derive(Clone, Copy, Eq, PartialEq)]
enum Presentation {
    /// Ordinary compatibility presentation.
    Compatibility,
    /// Opt-in sixtieth-second presentation.
    SixtySecond,
}

/// One scripted observation through the same comparator and reader as the broker.
type ScriptedSample = ClockSample<Adjustment, Presentation>;

/// A valid earlier capture ending exactly at signed tick zero.
const PREVIOUS: ScriptedSample = ClockSample {
    frequency_hz: NonZeroU64::MIN,
    qpc_before: -2,
    qpc_after: 0,
    filetime_100ns: 80,
    adjustment: Adjustment {
        update_word: 17,
        increment: 29,
        periodic_adjustment_disabled: false,
    },
    presentation: Some(Presentation::Compatibility),
};

/// The same exact adjustment after the original bracket remains observationally compatible.
const NEXT: ScriptedSample = ClockSample {
    qpc_before: 0,
    qpc_after: 1,
    filetime_100ns: 81,
    ..PREVIOUS
};

/// Give frequency precedence even when counter and wall order both disagree.
///
/// # Panics
/// Panics if changed frequency is missed alone or in the presence of other conflicts.
#[test]
fn changed_frequency_precedes_other_conflicts() {
    let changed_frequency_hz: NonZeroU64 = NonZeroU64::MIN.saturating_add(1);
    let mut candidate: ScriptedSample = ClockSample {
        frequency_hz: changed_frequency_hz,
        qpc_before: 0,
        qpc_after: 1,
        filetime_100ns: 81,
        ..PREVIOUS
    };
    assert_eq!(
        compare(&PREVIOUS, &candidate),
        Some(SampledClockDiscontinuity::FrequencyChanged)
    );
    candidate.qpc_before = -1;
    candidate.filetime_100ns = 79;
    candidate.adjustment.increment = 30;
    assert_eq!(
        compare(&PREVIOUS, &candidate),
        Some(SampledClockDiscontinuity::FrequencyChanged)
    );
}

/// Exercise the first tick of the later bracket against the last earlier tick.
///
/// # Panics
/// Panics if a one-tick counter reversal is missed, equal ticks are refused, or
/// a wall reversal takes precedence over a counter reversal.
#[test]
fn counter_order_uses_the_two_inner_edges() {
    let mut candidate: ScriptedSample = ClockSample {
        qpc_before: -1,
        qpc_after: 2,
        filetime_100ns: 79,
        ..PREVIOUS
    };
    assert_eq!(
        compare(&PREVIOUS, &candidate),
        Some(SampledClockDiscontinuity::QpcReversed)
    );
    candidate.qpc_before = 0;
    candidate.adjustment.update_word = 18;
    assert_eq!(
        compare(&PREVIOUS, &candidate),
        Some(SampledClockDiscontinuity::FiletimeReversed)
    );
    candidate.adjustment = PREVIOUS.adjustment;
    candidate.filetime_100ns = 80;
    assert_eq!(compare(&PREVIOUS, &candidate), None);
    candidate.qpc_before = 1;
    assert_eq!(compare(&PREVIOUS, &candidate), None);
}

/// Check the wall-clock boundary with strictly increasing QPC ticks.
///
/// # Panics
/// Panics if a one-unit wall regression is missed or either neighbor is refused.
#[test]
fn filetime_order_refuses_only_strict_regression() {
    for (filetime_100ns, expected) in [
        (79, Some(SampledClockDiscontinuity::FiletimeReversed)),
        (80, None),
        (81, None),
    ] {
        let candidate: ScriptedSample = ClockSample {
            qpc_before: 1,
            qpc_after: 2,
            filetime_100ns,
            ..PREVIOUS
        };
        assert_eq!(compare(&PREVIOUS, &candidate), expected);
    }
}

/// Exercise both directions across the full signed tick and unsigned wall ranges.
///
/// # Panics
/// Panics if extremes are reordered, ignored, or refused on the healthy pair.
#[test]
fn signed_tick_and_unsigned_wall_extremes_remain_exact() {
    let low: ScriptedSample = ClockSample {
        qpc_before: i64::MIN,
        qpc_after: i64::MIN,
        filetime_100ns: 0,
        ..PREVIOUS
    };
    let high: ScriptedSample = ClockSample {
        qpc_before: i64::MAX,
        qpc_after: i64::MAX,
        filetime_100ns: u64::MAX,
        ..PREVIOUS
    };
    assert_eq!(compare(&low, &high), None);
    assert_eq!(
        compare(&high, &low),
        Some(SampledClockDiscontinuity::QpcReversed)
    );
    let wall_reversed: ScriptedSample = ClockSample {
        filetime_100ns: 0,
        ..high
    };
    assert_eq!(
        compare(&high, &wall_reversed),
        Some(SampledClockDiscontinuity::FiletimeReversed)
    );
}

/// Detect each independent field change while keeping the clock ordering valid.
///
/// # Panics
/// Panics if a field change is missed, or unchanged adjustment is taken as
/// evidence of a violation based on forward wall movement alone.
#[test]
fn compares_each_adjustment_field_exactly() {
    let changed: [ScriptedSample; 3] = [
        ClockSample {
            adjustment: Adjustment {
                update_word: 18,
                ..NEXT.adjustment
            },
            ..NEXT
        },
        ClockSample {
            adjustment: Adjustment {
                increment: 30,
                ..NEXT.adjustment
            },
            ..NEXT
        },
        ClockSample {
            adjustment: Adjustment {
                periodic_adjustment_disabled: true,
                ..NEXT.adjustment
            },
            ..NEXT
        },
    ];
    for candidate in &changed {
        assert_eq!(
            compare(&PREVIOUS, candidate),
            Some(SampledClockDiscontinuity::AdjustmentChanged)
        );
    }
    assert_eq!(compare(&PREVIOUS, &NEXT), None);
    let large_forward_jump: ScriptedSample = ClockSample {
        filetime_100ns: u64::MAX,
        ..NEXT
    };
    assert_eq!(compare(&PREVIOUS, &large_forward_jump), None);
}

/// Compare all known and unavailable mode pairs after the existing clock checks.
///
/// # Panics
/// Panics if a mode change or loss is missed, a stable pair withdraws the
/// revision, or presentation replaces an earlier adjustment cause.
#[test]
fn compares_only_known_presentation_changes_after_adjustment() {
    let modes: [Option<Presentation>; 3] = [
        Some(Presentation::Compatibility),
        Some(Presentation::SixtySecond),
        None,
    ];
    let changed: Option<SampledClockTransition> = Some(SampledClockTransition::Discontinuity(
        SampledClockDiscontinuity::LeapPresentationChanged,
    ));
    let gap: Option<SampledClockTransition> = Some(SampledClockTransition::PresentationGap);
    let expected: [[Option<SampledClockTransition>; 3]; 3] =
        [[None, changed, gap], [changed, None, gap], [None; 3]];
    for (before, row) in modes.into_iter().zip(expected) {
        let previous: ScriptedSample = ClockSample {
            presentation: before,
            ..PREVIOUS
        };
        for (after, outcome) in modes.into_iter().zip(row) {
            let candidate: ScriptedSample = ClockSample {
                presentation: after,
                ..NEXT
            };
            assert_eq!(compare_for_sampling(&previous, &candidate), outcome);
        }
    }
    let both_changed: ScriptedSample = ClockSample {
        adjustment: Adjustment {
            update_word: 18,
            ..NEXT.adjustment
        },
        presentation: Some(Presentation::SixtySecond),
        ..NEXT
    };
    assert_eq!(
        compare(&PREVIOUS, &both_changed),
        Some(SampledClockDiscontinuity::AdjustmentChanged)
    );
    let gap_and_conflict: ScriptedSample = ScriptedSample {
        presentation: None,
        ..both_changed
    };
    assert_eq!(
        compare_for_sampling(&PREVIOUS, &gap_and_conflict),
        Some(SampledClockTransition::Discontinuity(
            SampledClockDiscontinuity::AdjustmentChanged,
        ))
    );
}

/// Separate mailbox for the process-presentation transition.
static PRESENTATION_CHANGES: BrokerClockChanges = BrokerClockChanges::new();
/// Number of samples charged by that reader.
static PRESENTATION_READS: AtomicU32 = AtomicU32::new(0);

/// Supply a known change with otherwise compatible observations and a fresh successor.
///
/// # Errors
/// Returns the unexpected-read marker if the sampler exceeds three calls.
fn read_presentation_sequence() -> Result<ScriptedSample, u32> {
    match PRESENTATION_READS.fetch_add(1, SeqCst) {
        0 => Ok(PREVIOUS),
        1 => Ok(ClockSample {
            presentation: Some(Presentation::SixtySecond),
            ..NEXT
        }),
        2 => Ok(ClockSample {
            qpc_before: 1,
            qpc_after: 2,
            filetime_100ns: 82,
            presentation: Some(Presentation::SixtySecond),
            ..NEXT
        }),
        _ => Err(42),
    }
}

/// A sampled mode transition retires the old revision before the next due read.
///
/// # Panics
/// Panics if the old revision still admits work, recovery occurs early, or
/// the history misrepresents a known change or an unavailable observation.
#[test]
fn presentation_change_retires_the_original_observation() {
    assert!(PRESENTATION_CHANGES.arm());
    let initial: Instant = Instant::now();
    let changed: Instant = initial + Duration::from_secs(1);
    let fresh: Instant = changed + Duration::from_secs(1);
    let mut reader: BrokerClockReader<ScriptedSample, u32> =
        BrokerClockReader::new(&PRESENTATION_CHANGES, read_presentation_sequence)
            .with_sampling(compare_for_sampling);

    reader.poll(initial, || Ok(initial));
    let original: Option<ClockRevision<'static>> = reader.observed_awake_revision();
    assert_eq!(
        original.as_ref().map(ClockRevision::number),
        Some(NonZeroU64::MIN)
    );

    reader.poll(changed, || Ok(changed));
    assert!(original.is_some_and(|revision: ClockRevision<'static>| !revision.admits()));
    assert!(reader.observed_awake_revision().is_none());
    assert!(matches!(
        reader.history(),
        Some(BrokerClockRead::Retired {
            revision: NonZeroU64::MIN,
            reason: ClockReadRetirement::Discontinuity(
                SampledClockDiscontinuity::LeapPresentationChanged
            ),
            native_failure: None
        })
    ));
    let retained: Option<SampledClockBreak> = Some(SampledClockBreak {
        revision: NonZeroU64::MIN,
        observed_at: changed,
        reason: SampledClockDiscontinuity::LeapPresentationChanged,
    });
    assert_eq!(reader.last_discontinuity(), retained);

    let early: Instant = changed + Duration::from_nanos(999_999_999);
    reader.poll(early, || Ok(early));
    assert_eq!(PRESENTATION_READS.load(SeqCst), 2);
    assert!(reader.observed_awake_revision().is_none());

    reader.poll(fresh, || Ok(fresh));
    assert_eq!(
        reader
            .observed_awake_revision()
            .map(|revision: ClockRevision<'static>| revision.number()),
        NonZeroU64::new(2)
    );
    assert!(matches!(
        reader.history(),
        Some(BrokerClockRead::Completed { result: Ok(sample), .. })
            if sample.presentation == Some(Presentation::SixtySecond)
    ));
    assert_eq!(reader.last_discontinuity(), retained);
    assert_eq!(PRESENTATION_READS.load(SeqCst), 3);

    check_presentation_gap();
}

/// Independent mailbox for the loss of a known presentation observation.
static PRESENTATION_GAP: BrokerClockChanges = BrokerClockChanges::new();
/// Number of samples charged across the presentation gap.
static GAP_READS: AtomicU32 = AtomicU32::new(0);

/// Return a known mode, a gap, another unavailable mode and a restored mode.
///
/// # Errors
/// Returns the unexpected-read marker if the sampler exceeds four calls.
fn read_presentation_gap() -> Result<ScriptedSample, u32> {
    match GAP_READS.fetch_add(1, SeqCst) {
        0 => Ok(PREVIOUS),
        1 => Ok(ScriptedSample {
            filetime_100ns: 100,
            presentation: None,
            ..NEXT
        }),
        2 => Ok(ScriptedSample {
            qpc_before: 1,
            qpc_after: 2,
            filetime_100ns: 82,
            presentation: None,
            ..NEXT
        }),
        3 => Ok(ScriptedSample {
            qpc_before: 2,
            qpc_after: 3,
            filetime_100ns: 83,
            ..NEXT
        }),
        _ => Err(42),
    }
}

/// Keep the raw observation and retire its live revision when the mode is lost.
///
/// # Panics
/// Panics if the gap is admitted, loses its historical bracket, reports a
/// contradiction, changes the next due instant, or reuses the old revision.
fn check_presentation_gap() {
    assert!(PRESENTATION_GAP.arm());
    let initial: Instant = Instant::now();
    let loss_start: Instant = initial + Duration::from_secs(1);
    let loss_end: Instant = loss_start + Duration::from_millis(20);
    let mut reader: BrokerClockReader<ScriptedSample, u32> =
        BrokerClockReader::new(&PRESENTATION_GAP, read_presentation_gap)
            .with_sampling(compare_for_sampling);

    reader.poll(initial, || Ok(initial));
    let original: Option<ClockRevision<'static>> = reader.observed_awake_revision();
    assert_eq!(
        original.as_ref().map(ClockRevision::number),
        Some(NonZeroU64::MIN)
    );

    let mut edges: std::array::IntoIter<Instant, 2> = [loss_start, loss_end].into_iter();
    reader.poll(loss_start, || {
        edges.next().ok_or(ClockReadRetirement::Generation)
    });
    assert!(
        original
            .as_ref()
            .is_some_and(|revision: &ClockRevision<'_>| !revision.admits()),
        "unavailable presentation must retire the original awake revision"
    );
    assert!(reader.observed_awake_revision().is_none());
    assert!(matches!(
        reader.history(),
        Some(BrokerClockRead::Completed {
            revision: NonZeroU64::MIN, before, after, result: Ok(sample),
        }) if before == loss_start && after == loss_end
            && sample.filetime_100ns == 100 && sample.presentation.is_none()
    ));
    assert!(reader.last_discontinuity().is_none());

    let early: Instant = loss_end + Duration::from_nanos(999_999_999);
    assert_eq!(reader.sample_wait(early), Some(Duration::from_nanos(1)));
    reader.poll(early, || Ok(early));
    assert_eq!(GAP_READS.load(SeqCst), 2);
    assert!(reader.observed_awake_revision().is_none());

    let fresh: Instant = loss_end + Duration::from_secs(1);
    reader.poll(fresh, || Ok(fresh));
    assert_eq!(
        reader
            .observed_awake_revision()
            .as_ref()
            .map(ClockRevision::number),
        NonZeroU64::new(2)
    );
    let restored: Instant = fresh + Duration::from_secs(1);
    reader.poll(restored, || Ok(restored));
    assert_eq!(
        reader
            .observed_awake_revision()
            .as_ref()
            .map(ClockRevision::number),
        NonZeroU64::new(2)
    );
    assert!(matches!(
        reader.history(),
        Some(BrokerClockRead::Completed { result: Ok(sample), .. })
            if sample.presentation == Some(Presentation::Compatibility)
    ));
    assert!(original.is_some_and(|revision: ClockRevision<'static>| !revision.admits()));
    assert_eq!(GAP_READS.load(SeqCst), 4);
}

/// Independent process-local sampling context for the adjustment transition.
static CHANGES: BrokerClockChanges = BrokerClockChanges::new();
/// Number of samples charged by the real reader.
static READS: AtomicU32 = AtomicU32::new(0);

/// Return the original, changed, and freshly anchored observations.
///
/// # Errors
/// Returns the literal unexpected-read marker if the sampler exceeds three calls.
fn read_adjustment_sequence() -> Result<ScriptedSample, u32> {
    match READS.fetch_add(1, SeqCst) {
        0 => Ok(PREVIOUS),
        1 => Ok(ClockSample {
            adjustment: Adjustment {
                periodic_adjustment_disabled: true,
                ..NEXT.adjustment
            },
            ..NEXT
        }),
        2 => Ok(ClockSample {
            qpc_before: 1,
            qpc_after: 2,
            filetime_100ns: 82,
            adjustment: Adjustment {
                periodic_adjustment_disabled: true,
                ..NEXT.adjustment
            },
            ..NEXT
        }),
        _ => Err(42),
    }
}

/// Retire the changed observation through the real same-revision CAS and recover.
///
/// # Panics
/// Panics if the original observation is never available, remains available after
/// a change, or the next due observation fails to publish in a fresh revision.
#[test]
fn adjustment_change_retires_the_original_observation() {
    assert!(CHANGES.arm());
    let initial: Instant = Instant::now();
    let changed: Instant = initial + Duration::from_secs(1);
    let fresh: Instant = changed + Duration::from_secs(1);
    let mut reader: BrokerClockReader<ScriptedSample, u32> =
        BrokerClockReader::new(&CHANGES, read_adjustment_sequence)
            .with_sampling(compare_for_sampling);

    reader.poll(initial, || Ok(initial));
    assert_eq!(
        reader
            .observed_awake_revision()
            .map(|revision| revision.number()),
        Some(NonZeroU64::MIN)
    );

    reader.poll(changed, || Ok(changed));
    assert!(reader.observed_awake_revision().is_none());
    assert!(matches!(
        reader.history(),
        Some(BrokerClockRead::Retired {
            revision: NonZeroU64::MIN,
            reason: ClockReadRetirement::Discontinuity(
                SampledClockDiscontinuity::AdjustmentChanged
            ),
            native_failure: None
        })
    ));
    let retained: Option<SampledClockBreak> = Some(SampledClockBreak {
        revision: NonZeroU64::MIN,
        observed_at: changed,
        reason: SampledClockDiscontinuity::AdjustmentChanged,
    });
    assert_eq!(reader.last_discontinuity(), retained);
    assert_eq!(READS.load(SeqCst), 2);

    reader.poll(fresh, || Ok(fresh));
    assert_eq!(
        reader
            .observed_awake_revision()
            .map(|revision| revision.number()),
        NonZeroU64::new(2)
    );
    assert_eq!(reader.last_discontinuity(), retained);
    assert_eq!(READS.load(SeqCst), 3);
}

/// Preserve an exact coordinate error and skip both later reads.
///
/// # Panics
/// Panics if either later read runs or the original error is changed.
#[cfg(windows)]
#[test]
fn coordinate_failure_skips_the_adjustment_read() {
    use std::cell::Cell;

    use windows_platform_native::{ClockReadError, LeapPresentationError};

    use super::{BrokerClockSample, BrokerClockSampleError};

    let adjustment_read: Cell<bool> = Cell::new(false);
    let presentation_read: Cell<bool> = Cell::new(false);
    let result: Result<BrokerClockSample, BrokerClockSampleError> = BrokerClockSample::read_with(
        || Err(ClockReadError::Before(0)),
        || {
            adjustment_read.set(true);
            Err(1)
        },
        || {
            presentation_read.set(true);
            Err(LeapPresentationError::QueryFailed(2))
        },
    );
    assert_eq!(
        result,
        Err(BrokerClockSampleError::Coordinates(ClockReadError::Before(
            0
        )))
    );
    assert!(!adjustment_read.get());
    assert!(!presentation_read.get());
}

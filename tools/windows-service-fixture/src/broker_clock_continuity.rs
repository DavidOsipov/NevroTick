// SPDX-License-Identifier: GPL-2.0-only
//! Compare adjacent raw captures from one awake broker revision.

#![cfg(any(windows, test))]
#![forbid(unsafe_code)]

use std::num::NonZeroU64;

#[cfg(windows)]
use windows_platform_native::{
    ClockAdjustment, ClockCoordinates, ClockReadError, LeapPresentation, LeapPresentationError,
    read_clock_adjustment, read_clock_coordinates, read_leap_presentation,
};

/// One completed broker sample with separately observed Windows clock state.
#[cfg(windows)]
#[must_use = "retain the checked coordinates, adjustment, and leap presentation observation"]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BrokerClockSample {
    /// Original QPC bracket around the precise wall-clock read.
    coordinates: ClockCoordinates,
    /// One adjustment read immediately after that bracket.
    adjustment: ClockAdjustment,
    /// This broker process's separately queried presentation after adjustment.
    leap_presentation: Result<LeapPresentation, LeapPresentationError>,
}

#[cfg(windows)]
impl BrokerClockSample {
    /// Read coordinates and adjustment, then observe this process's leap presentation.
    ///
    /// The separate reads confer no mapping, continuity or clock authority.
    ///
    /// # Errors
    /// Return the first coordinate or adjustment refusal. The presentation query's
    /// outcome remains separate from the existing broker observation admission.
    pub(crate) fn read() -> Result<Self, BrokerClockSampleError> {
        Self::read_with(
            read_clock_coordinates,
            read_clock_adjustment,
            read_leap_presentation,
        )
    }

    /// Complete the original sample, then retain the subsequent presentation result.
    ///
    /// # Errors
    /// Stop at the first coordinate or adjustment failure before querying presentation.
    fn read_with(
        coordinates: impl FnOnce() -> Result<ClockCoordinates, ClockReadError>,
        adjustment: impl FnOnce() -> Result<ClockAdjustment, u32>,
        leap_presentation: impl FnOnce() -> Result<LeapPresentation, LeapPresentationError>,
    ) -> Result<Self, BrokerClockSampleError> {
        let coordinates: ClockCoordinates =
            coordinates().map_err(BrokerClockSampleError::Coordinates)?;
        let adjustment: ClockAdjustment =
            adjustment().map_err(BrokerClockSampleError::Adjustment)?;
        Ok(Self {
            coordinates,
            adjustment,
            leap_presentation: leap_presentation(),
        })
    }

    /// Use the actual coordinate getter and a controlled adjustment result in tests.
    ///
    /// # Errors
    /// Preserve the real coordinate failure or the scripted adjustment refusal.
    #[cfg(test)]
    pub(crate) fn read_with_adjustment(
        adjustment: impl FnOnce() -> Result<ClockAdjustment, u32>,
    ) -> Result<Self, BrokerClockSampleError> {
        Self::read_with(read_clock_coordinates, adjustment, read_leap_presentation)
    }

    /// Use actual clock coordinates and adjustment with a controlled presentation refusal.
    ///
    /// # Errors
    /// Preserve the original coordinate or adjustment failure.
    #[cfg(test)]
    pub(crate) fn read_with_presentation_failure(
        failure: LeapPresentationError,
    ) -> Result<Self, BrokerClockSampleError> {
        Self::read_with(read_clock_coordinates, read_clock_adjustment, || {
            Err(failure)
        })
    }

    /// Return the original checked bracket without further clock reads.
    #[must_use]
    pub const fn coordinates(&self) -> &ClockCoordinates {
        &self.coordinates
    }

    /// Return the raw adjustment state observed after the bracket.
    #[must_use]
    pub const fn adjustment(&self) -> &ClockAdjustment {
        &self.adjustment
    }

    /// Return this process's original presentation observation, including an exact refusal.
    /// The result describes one historical query after the clock reads.
    ///
    /// # Errors
    /// Returns the original presentation query refusal, when recorded.
    pub const fn leap_presentation(&self) -> Result<LeapPresentation, LeapPresentationError> {
        self.leap_presentation
    }
}

/// First stage that prevented a complete raw broker sample.
#[cfg(windows)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BrokerClockSampleError {
    /// The checked coordinate getter refused its own operation.
    Coordinates(ClockReadError),
    /// The adjustment getter failed with this original Win32 code, including zero.
    Adjustment(u32),
}

/// An observed conflict between two serialized broker clock captures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SampledClockDiscontinuity {
    /// The second capture reports a different QPC frequency.
    FrequencyChanged,
    /// The second capture's first QPC tick precedes the first capture's last tick.
    QpcReversed,
    /// The second capture reports a smaller raw FILETIME value.
    FiletimeReversed,
    /// At least one raw adjustment word or the reported mode differs.
    AdjustmentChanged,
    /// The two successful process presentation observations report different modes.
    LeapPresentationChanged,
}

/// The raw components needed to compare two completed observations.
#[cfg_attr(test, derive(Clone, Copy))]
struct ClockSample<Adjustment, Presentation> {
    /// Positive QPC frequency in ticks per second.
    frequency_hz: NonZeroU64,
    /// Signed QPC tick observed before FILETIME.
    qpc_before: i64,
    /// Signed QPC tick observed after FILETIME, at least `qpc_before`.
    qpc_after: i64,
    /// Raw wall coordinate in 100-nanosecond units.
    filetime_100ns: u64,
    /// Original complete adjustment state observed after this bracket.
    adjustment: Adjustment,
    /// Known process mode for comparison; the original query result stays in the raw sample.
    presentation: Option<Presentation>,
}

#[cfg(windows)]
impl ClockSample<ClockAdjustment, LeapPresentation> {
    /// Project the checked coordinates, adjustment and any known presentation mode.
    /// The immutable native sample retains the original presentation query error.
    fn from_native(sample: &BrokerClockSample) -> Self {
        let coordinates: &ClockCoordinates = sample.coordinates();
        Self {
            frequency_hz: coordinates.frequency_hz(),
            qpc_before: coordinates.qpc_before(),
            qpc_after: coordinates.qpc_after(),
            filetime_100ns: coordinates.filetime_100ns(),
            adjustment: *sample.adjustment(),
            presentation: sample.leap_presentation().ok(),
        }
    }
}

/// Compare completed native observations in the order the broker read them.
/// The owner supplies adjacent synchronous captures from the same thread and
/// awake revision. An empty result leaves wall-clock continuity unestablished.
#[cfg(windows)]
#[must_use]
pub(super) fn compare_native(
    previous: &BrokerClockSample,
    candidate: &BrokerClockSample,
) -> Option<SampledClockDiscontinuity> {
    compare(
        &ClockSample::from_native(previous),
        &ClockSample::from_native(candidate),
    )
}

/// Return the first conflict in frequency, counter order, wall order, adjustment
/// and observed leap presentation. Equal endpoints and wall readings satisfy their
/// ordering checks. Raw adjustment comparison uses exact equality. Presentation
/// comparison requires two successful observations, leaving a refused query's
/// original error in the raw sample. No result establishes a UTC mapping.
fn compare<Adjustment: Eq, Presentation: Eq>(
    previous: &ClockSample<Adjustment, Presentation>,
    candidate: &ClockSample<Adjustment, Presentation>,
) -> Option<SampledClockDiscontinuity> {
    if candidate.frequency_hz != previous.frequency_hz {
        return Some(SampledClockDiscontinuity::FrequencyChanged);
    }
    if candidate.qpc_before < previous.qpc_after {
        return Some(SampledClockDiscontinuity::QpcReversed);
    }
    if candidate.filetime_100ns < previous.filetime_100ns {
        return Some(SampledClockDiscontinuity::FiletimeReversed);
    }
    if candidate.adjustment != previous.adjustment {
        return Some(SampledClockDiscontinuity::AdjustmentChanged);
    }
    if previous
        .presentation
        .as_ref()
        .zip(candidate.presentation.as_ref())
        .is_some_and(|(before, after): (&Presentation, &Presentation)| before != after)
    {
        return Some(SampledClockDiscontinuity::LeapPresentationChanged);
    }
    None
}

#[cfg(test)]
mod tests {
    use std::{
        num::NonZeroU64,
        sync::atomic::{AtomicU32, Ordering::SeqCst},
        time::{Duration, Instant},
    };

    use super::{
        super::{
            BrokerClockChanges, BrokerClockRead, BrokerClockReader, ClockReadRetirement,
            ClockRevision, SampledClockBreak,
        },
        ClockSample, SampledClockDiscontinuity, compare,
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
    /// Panics if a confirmed mode change is missed, an unknown or matching pair
    /// withdraws the revision, or presentation replaces an earlier adjustment cause.
    #[test]
    fn compares_only_known_presentation_changes_after_adjustment() {
        let modes: [Option<Presentation>; 3] = [
            Some(Presentation::Compatibility),
            Some(Presentation::SixtySecond),
            None,
        ];
        let changed: Option<SampledClockDiscontinuity> =
            Some(SampledClockDiscontinuity::LeapPresentationChanged);
        let expected: [[Option<SampledClockDiscontinuity>; 3]; 3] =
            [[None, changed, None], [changed, None, None], [None; 3]];
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
                assert_eq!(compare(&previous, &candidate), outcome);
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
    /// a fresh observation loses the historical break.
    #[test]
    fn presentation_change_retires_the_original_observation() {
        assert!(PRESENTATION_CHANGES.arm());
        let initial: Instant = Instant::now();
        let changed: Instant = initial + Duration::from_secs(1);
        let fresh: Instant = changed + Duration::from_secs(1);
        let mut reader: BrokerClockReader<ScriptedSample, u32> =
            BrokerClockReader::new(&PRESENTATION_CHANGES, read_presentation_sequence)
                .with_sampling(compare);

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
            BrokerClockReader::new(&CHANGES, read_adjustment_sequence).with_sampling(compare);

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
        let result: Result<BrokerClockSample, BrokerClockSampleError> =
            BrokerClockSample::read_with(
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
}

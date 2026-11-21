// SPDX-License-Identifier: GPL-2.0-only
//! Compare adjacent raw captures from one awake broker revision.

#![cfg(any(windows, test))]
#![forbid(unsafe_code)]

use std::num::NonZeroU64;

#[cfg(windows)]
use windows_platform_native::ClockCoordinates;

/// An observed conflict between two serialized broker clock captures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SampledClockDiscontinuity {
    /// The second capture reports a different QPC frequency.
    FrequencyChanged,
    /// The second capture's first QPC tick precedes the first capture's last tick.
    QpcReversed,
    /// The second capture reports a smaller raw FILETIME value.
    FiletimeReversed,
}

/// The raw components needed to compare two already validated brackets.
struct ClockSample {
    /// Positive QPC frequency in ticks per second.
    frequency_hz: NonZeroU64,
    /// Signed QPC tick observed before FILETIME.
    qpc_before: i64,
    /// Signed QPC tick observed after FILETIME, at least `qpc_before`.
    qpc_after: i64,
    /// Raw wall coordinate in 100-nanosecond units.
    filetime_100ns: u64,
}

#[cfg(windows)]
impl ClockSample {
    /// Project the checked bracket without converting units or changing its values.
    fn from_native(sample: &ClockCoordinates) -> Self {
        Self {
            frequency_hz: sample.frequency_hz(),
            qpc_before: sample.qpc_before(),
            qpc_after: sample.qpc_after(),
            filetime_100ns: sample.filetime_100ns(),
        }
    }
}

/// Compare validated native brackets in the order the broker read them.
/// The owner supplies adjacent synchronous captures from the same thread and
/// awake revision. An empty result leaves wall-clock continuity unestablished.
#[cfg(windows)]
#[must_use]
pub(super) fn compare_native(
    previous: &ClockCoordinates,
    candidate: &ClockCoordinates,
) -> Option<SampledClockDiscontinuity> {
    compare(
        &ClockSample::from_native(previous),
        &ClockSample::from_native(candidate),
    )
}

/// Return the first observed conflict in frequency, counter order and wall order.
/// Equal endpoints or wall readings satisfy their respective ordering checks.
fn compare(previous: &ClockSample, candidate: &ClockSample) -> Option<SampledClockDiscontinuity> {
    if candidate.frequency_hz != previous.frequency_hz {
        return Some(SampledClockDiscontinuity::FrequencyChanged);
    }
    if candidate.qpc_before < previous.qpc_after {
        return Some(SampledClockDiscontinuity::QpcReversed);
    }
    if candidate.filetime_100ns < previous.filetime_100ns {
        return Some(SampledClockDiscontinuity::FiletimeReversed);
    }
    None
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU64;

    use super::{ClockSample, SampledClockDiscontinuity, compare};

    /// A valid earlier capture ending exactly at signed tick zero.
    const PREVIOUS: ClockSample = ClockSample {
        frequency_hz: NonZeroU64::MIN,
        qpc_before: -2,
        qpc_after: 0,
        filetime_100ns: 80,
    };

    /// Give frequency precedence even when counter and wall order both disagree.
    ///
    /// # Panics
    /// Panics if changed frequency is missed alone or in the presence of other conflicts.
    #[test]
    fn changed_frequency_precedes_other_conflicts() {
        let changed_frequency_hz: NonZeroU64 = NonZeroU64::MIN.saturating_add(1);
        let mut candidate: ClockSample = ClockSample {
            frequency_hz: changed_frequency_hz,
            qpc_before: 0,
            qpc_after: 1,
            filetime_100ns: 81,
        };
        assert_eq!(
            compare(&PREVIOUS, &candidate),
            Some(SampledClockDiscontinuity::FrequencyChanged)
        );
        candidate.qpc_before = -1;
        candidate.filetime_100ns = 79;
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
        let mut candidate: ClockSample = ClockSample {
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
        assert_eq!(
            compare(&PREVIOUS, &candidate),
            Some(SampledClockDiscontinuity::FiletimeReversed)
        );
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
            let candidate: ClockSample = ClockSample {
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
        let low: ClockSample = ClockSample {
            qpc_before: i64::MIN,
            qpc_after: i64::MIN,
            filetime_100ns: 0,
            ..PREVIOUS
        };
        let high: ClockSample = ClockSample {
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
        let wall_reversed: ClockSample = ClockSample {
            filetime_100ns: 0,
            ..high
        };
        assert_eq!(
            compare(&high, &wall_reversed),
            Some(SampledClockDiscontinuity::FiletimeReversed)
        );
    }
}

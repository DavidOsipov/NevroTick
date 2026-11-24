// SPDX-License-Identifier: GPL-2.0-only
//! Compare adjacent raw captures from one awake broker revision.

#![cfg(any(windows, test))]
#![forbid(unsafe_code)]

use std::num::NonZeroU64;

use super::SampledClockTransition;

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
    /// Return the first coordinate or adjustment refusal. Retain the presentation
    /// query result within the sample so the reader can compare it with prior evidence.
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
/// awake revision. Results identify withdrawal evidence and confer no UTC mapping.
#[cfg(windows)]
#[must_use]
pub(super) fn compare_native(
    previous: &BrokerClockSample,
    candidate: &BrokerClockSample,
) -> Option<SampledClockTransition> {
    compare_for_sampling(
        &ClockSample::from_native(previous),
        &ClockSample::from_native(candidate),
    )
}

/// Preserve contradiction precedence and distinguish loss of a known observation.
fn compare_for_sampling<Adjustment: Eq, Presentation: Eq>(
    previous: &ClockSample<Adjustment, Presentation>,
    candidate: &ClockSample<Adjustment, Presentation>,
) -> Option<SampledClockTransition> {
    if let Some(reason) = compare(previous, candidate) {
        return Some(SampledClockTransition::Discontinuity(reason));
    }
    (previous.presentation.is_some() && candidate.presentation.is_none())
        .then_some(SampledClockTransition::PresentationGap)
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
#[path = "broker_clock_continuity_tests.rs"]
mod tests;

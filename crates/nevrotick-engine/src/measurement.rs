// SPDX-License-Identifier: GPL-2.0-only
//! Conditional, ineligible arithmetic on explicit experimental software clocks.
//!
//! This module reads no clocks and performs no I/O. Its caller supplies actual
//! captures and an explicitly declared date/coherence model. Checks can reject
//! inconsistent wall/monotonic observations; passing them cannot exclude
//! unobserved clock jumps or network asymmetry.
//! Results describe nominal software-read placement, not physical packet times,
//! qualified UTC continuity, authentication, sample eligibility or clock authority.
//! The engine owner separately rechecks current identity, generations and deadline
//! before consuming the request and publishing its terminal outcome once.

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use ntp_wire::RawHeader;
use time_domain::{
    DomainError, EraWindow, ExchangeArithmetic, UtcCoordinateQ32, exchange_arithmetic, unfold_ntp,
};

use super::SoftwareClock;

/// Exact scale of an NTP fractional second.
const Q32_SCALE: i128 = 1_i128 << 32;
/// Exact decimal scale of the supplied system-wall representation.
const NANOS_PER_SECOND: i128 = 1_000_000_000;
/// Whole seconds from the NTP epoch to the Unix epoch; no date inference occurs.
const NTP_UNIX_OFFSET_SECONDS: i128 = 2_208_988_800;
/// Maximum declared coherence allowance, matching the bounded 30-second pilot.
/// This is an experiment-input bound, not an adopted clock-accuracy threshold.
const MAX_DECLARED_RESIDUAL_NS: u64 = 30_000_000_000;

/// Fixed redacted reasons why no experimental numerical measurement is present.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MeasurementRefusal {
    /// No explicit date/model profile was supplied.
    Unavailable,
    /// Date endpoints or the finite declared coherence allowance are invalid.
    InvalidProfile,
    /// Capture brackets, send/receive order or processing order are inconsistent.
    CaptureOrder,
    /// Observed wall/monotonic progress exceeds the declared experimental model.
    MappingDiscontinuity,
    /// A nominal local coordinate lies outside the independently supplied scope.
    DateScope,
    /// The supplied scope admits no era for a peer timestamp.
    EraNoCandidate,
    /// The supplied scope admits multiple eras for a peer timestamp.
    EraAmbiguous,
    /// A checked representation, conversion or arithmetic result is unavailable.
    Arithmetic,
    /// The inert header is not an ordinary, nonzero-timestamp time reply.
    Protocol,
}

impl MeasurementRefusal {
    /// Return a bounded allowlisted code, never rejected bytes or raw errors.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unavailable => "mapping_unavailable",
            Self::InvalidProfile => "invalid_profile",
            Self::CaptureOrder => "capture_order",
            Self::MappingDiscontinuity => "mapping_discontinuity",
            Self::DateScope => "date_scope",
            Self::EraNoCandidate => "era_no_candidate",
            Self::EraAmbiguous => "era_ambiguous",
            Self::Arithmetic => "arithmetic",
            Self::Protocol => "not_time_reply",
        }
    }
}

impl std::fmt::Display for MeasurementRefusal {
    /// Emit the same fixed code as the diagnostic projection, without raw data.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl std::error::Error for MeasurementRefusal {}

/// Explicit experimental assumptions, carrying no trusted-date or UTC guarantee.
///
/// The caller declares unsmeared nominal software UTC and local-clock coherence
/// over this one bounded run. Wall/monotonic observations only challenge that
/// declaration; they cannot prove uninterrupted continuity or physical accuracy.
/// The numeric date window is supplied independently of captured host dates.
/// All fractional values of both inclusive endpoint seconds are admitted.
///
/// The supported constructor admits explicit bounds and preserves the allowance:
/// ```
/// use nevrotick_engine::{ExperimentalProfile, MeasurementRefusal};
/// # fn main() -> Result<(), MeasurementRefusal> {
/// let profile = ExperimentalProfile::new(4_000_000_000, 4_000_000_001, 1_000_000)?;
/// assert_eq!(profile.earliest_ntp_seconds(), 4_000_000_000);
/// assert_eq!(profile.latest_ntp_seconds(), 4_000_000_001);
/// assert_eq!(profile.maximum_residual_ns(), 1_000_000);
/// # Ok(()) }
/// ```
/// Every raw field is supplied here; privacy rejects the unchecked allowance:
/// ```compile_fail,E0451
/// use nevrotick_engine::{ExperimentalProfile, MeasurementRefusal};
/// # fn main() -> Result<(), MeasurementRefusal> {
/// let checked = ExperimentalProfile::new(0, 0, 0)?;
/// let _bypass = ExperimentalProfile {
///     dates: checked.dates(),
///     earliest_ntp_seconds: 0,
///     latest_ntp_seconds: 0,
///     maximum_residual_ns: u64::MAX,
/// };
/// # Ok(()) }
/// ```
/// Checked assumptions cannot be changed through their private representation:
/// ```compile_fail,E0616
/// use nevrotick_engine::{ExperimentalProfile, MeasurementRefusal};
/// # fn main() -> Result<(), MeasurementRefusal> {
/// let mut checked = ExperimentalProfile::new(0, 0, 0)?;
/// checked.maximum_residual_ns = u64::MAX;
/// assert_eq!(checked.maximum_residual_ns(), u64::MAX);
/// # Ok(()) }
/// ```
/// No default can manufacture the required explicit date/model declaration:
/// ```compile_fail,E0599
/// use nevrotick_engine::ExperimentalProfile;
/// let _bypass = ExperimentalProfile::default();
/// ```
/// Raw tuples cannot bypass the checked constructor through an infallible trait:
/// ```compile_fail,E0277
/// use nevrotick_engine::ExperimentalProfile;
/// let _bypass: ExperimentalProfile = (0_u64, 0_u64, u64::MAX).into();
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExperimentalProfile {
    /// Garde-backed endpoint and ordering invariants are owned by `EraWindow`.
    dates: EraWindow,
    /// Original inclusive lower whole second, coupled to `dates` by construction.
    earliest_ntp_seconds: u64,
    /// Original inclusive upper whole second, coupled to `dates` by construction.
    latest_ntp_seconds: u64,
    /// Maximum permitted interval-gap residual, at most 30,000,000,000 ns.
    maximum_residual_ns: u64,
}

impl ExperimentalProfile {
    /// Check explicit inclusive whole-second bounds and a finite lab allowance.
    ///
    /// Mandatory `EraWindow` construction validates both Q32 endpoints through
    /// its shared Garde rules and checks their ordering. No host clock is read.
    /// The allowance may be zero and is not an accuracy or quality claim.
    ///
    /// # Errors
    /// Rejects inverted/out-of-envelope endpoints or allowances above 30 seconds.
    pub fn new(
        earliest_ntp_seconds: u64,
        latest_ntp_seconds: u64,
        maximum_residual_ns: u64,
    ) -> Result<Self, MeasurementRefusal> {
        if maximum_residual_ns > MAX_DECLARED_RESIDUAL_NS {
            return Err(MeasurementRefusal::InvalidProfile);
        }
        let earliest = i128::from(earliest_ntp_seconds)
            .checked_mul(Q32_SCALE)
            .ok_or(MeasurementRefusal::InvalidProfile)?;
        let latest = i128::from(latest_ntp_seconds)
            .checked_mul(Q32_SCALE)
            .and_then(|value| value.checked_add(Q32_SCALE - 1))
            .ok_or(MeasurementRefusal::InvalidProfile)?;
        let dates =
            EraWindow::new(earliest, latest).map_err(|_| MeasurementRefusal::InvalidProfile)?;
        Ok(Self {
            dates,
            earliest_ntp_seconds,
            latest_ntp_seconds,
            maximum_residual_ns,
        })
    }

    /// Return the exact immutable inclusive date window, with no truth assertion.
    #[must_use]
    pub const fn dates(self) -> EraWindow {
        self.dates
    }

    /// Return the caller's original inclusive lower whole NTP second.
    #[must_use]
    pub const fn earliest_ntp_seconds(self) -> u64 {
        self.earliest_ntp_seconds
    }

    /// Return the caller's original inclusive upper whole NTP second.
    #[must_use]
    pub const fn latest_ntp_seconds(self) -> u64 {
        self.latest_ntp_seconds
    }

    /// Return the declared coherence allowance; this is not physical uncertainty.
    #[must_use]
    pub const fn maximum_residual_ns(self) -> u64 {
        self.maximum_residual_ns
    }
}

/// Immutable numerical result conditional on the experimental software model.
///
/// Coordinates are nominal T1/T2/T3/T4 in the explicit NTP-epoch scale. T1 is
/// the real send-before wall read, never the randomized matching token. Exact
/// arithmetic applies to the floored Q32 local coordinates; the original
/// nanosecond representation is reconstructible from the retained residuals.
/// Capture placement and unobserved discontinuities remain unqualified, wire
/// capture error remains unknown, and this value confers no sample eligibility.
///
/// A caller may read the checked immutable result returned by the engine:
/// ```
/// use nevrotick_engine::ExperimentalMeasurement;
/// let _read: fn(ExperimentalMeasurement) -> [u32; 2] =
///     |measurement| measurement.local_fractional_remainders();
/// ```
/// Struct update cannot replace a private assumption on an existing result:
/// ```compile_fail,E0451
/// use nevrotick_engine::ExperimentalMeasurement;
/// let _bypass: fn(ExperimentalMeasurement) -> ExperimentalMeasurement = |original| {
///     ExperimentalMeasurement { declared_limit_ns: u64::MAX, ..original }
/// };
/// ```
/// Nor may a checked result be mutated after publication:
/// ```compile_fail,E0616
/// use nevrotick_engine::ExperimentalMeasurement;
/// let _bypass: fn(ExperimentalMeasurement) -> ExperimentalMeasurement = |mut original| {
///     original.declared_limit_ns = u64::MAX;
///     original
/// };
/// ```
/// An absent measurement has no default successful replacement:
/// ```compile_fail,E0599
/// use nevrotick_engine::ExperimentalMeasurement;
/// let _bypass = ExperimentalMeasurement::default();
/// ```
/// Raw coordinates cannot create measurement evidence through `From`:
/// ```compile_fail,E0277
/// use nevrotick_engine::ExperimentalMeasurement;
/// let _bypass: ExperimentalMeasurement = [0_i128; 4].into();
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExperimentalMeasurement {
    /// Checked nominal coordinates, ordered T1, T2, T3, T4.
    coordinates: [UtcCoordinateQ32; 4],
    /// Actual checked Q33 phase and signed Q32 raw round-trip arithmetic.
    arithmetic: ExchangeArithmetic,
    /// Remainders for T1/T4: exact local coordinate is floor + remainder/10^9.
    local_fractional_remainders: [u32; 2],
    /// Maximum minimum gap between observed wall progress and capture intervals.
    observed_residual_ns: u64,
    /// Explicit model allowance against which the observed gap was checked.
    declared_limit_ns: u64,
    /// Fresh processing age since receive capture completion, supplied by owner.
    acquisition_age: Duration,
    /// Nominal coordinate span including send-after, not a physical wire span.
    software_window: EraWindow,
}

impl ExperimentalMeasurement {
    /// Return the immutable nominal T1/T2/T3/T4 coordinates.
    #[must_use]
    pub const fn coordinates(self) -> [UtcCoordinateQ32; 4] {
        self.coordinates
    }

    /// Return exact arithmetic on Q32 inputs, without dropping signed raw RTT.
    #[must_use]
    pub const fn arithmetic(self) -> ExchangeArithmetic {
        self.arithmetic
    }

    /// Return T1/T4 conversion remainders, each strictly below 1,000,000,000.
    ///
    /// Each remainder has denominator 10^9 Q32 ticks. Relative to retained
    /// arithmetic, nominal phase's Q33 numerator has correction -(r1+r4)/10^9
    /// and nominal RTT's Q32 numerator has correction (r4-r1)/10^9.
    #[must_use]
    pub const fn local_fractional_remainders(self) -> [u32; 2] {
        self.local_fractional_remainders
    }

    /// Return the observed interval-gap residual; zero is no accuracy claim.
    #[must_use]
    pub const fn observed_residual_ns(self) -> u64 {
        self.observed_residual_ns
    }

    /// Return the declared model bound separately from its observed residual.
    #[must_use]
    pub const fn declared_limit_ns(self) -> u64 {
        self.declared_limit_ns
    }

    /// Return age from the chosen receive-capture completion reference instant.
    #[must_use]
    pub const fn acquisition_age(self) -> Duration {
        self.acquisition_age
    }

    /// Return nominal software-coordinate coverage, not packet-placement bounds.
    #[must_use]
    pub const fn software_window(self) -> EraWindow {
        self.software_window
    }

    /// Attach the actual checked fresh age at the owner's final guarded commit.
    ///
    /// The unique engine owner must derive `age` from fresh processing minus the
    /// same receive-capture completion and reject expired/stale state before this
    /// call. An immutable `Duration` is retained; no clock or state is read here.
    #[must_use]
    pub(crate) const fn with_acquisition_age(self, age: Duration) -> Self {
        Self {
            coordinates: self.coordinates,
            arithmetic: self.arithmetic,
            local_fractional_remainders: self.local_fractional_remainders,
            observed_residual_ns: self.observed_residual_ns,
            declared_limit_ns: self.declared_limit_ns,
            acquisition_age: age,
            software_window: self.software_window,
        }
    }
}

/// Exact decimal source coordinate plus its checked floor-Q32 representation.
#[derive(Clone, Copy)]
struct ConvertedWall {
    /// Exact nominal NTP-epoch nanoseconds before binary scaling.
    nanoseconds: i128,
    /// Floor of the exact nominal coordinate in Q32 ticks.
    coordinate: UtcCoordinateQ32,
    /// Euclidean remainder in 0..10^9, with denominator 10^9 Q32 ticks.
    remainder: u32,
}

/// Stage a fixed-size conditional measurement, without effects or clock reads.
///
/// Header structure is checked again at this numerical consumer. Current source,
/// socket, token, generation, cancellation and deadline ownership remain with the
/// unique caller and must be rechecked at final commit. Profile absence and any
/// refusal leave the supplied immutable inputs unchanged. Successful arithmetic
/// never asserts continuity, authentication, wire accuracy or statistical weight.
///
/// # Errors
/// Returns fixed refusal classes for missing context, invalid ordinary headers,
/// capture ordering, observed discontinuity, date scope, era or checked arithmetic.
pub(crate) fn measure(
    profile: Option<ExperimentalProfile>,
    send_before: SoftwareClock,
    send_after: SoftwareClock,
    read: SoftwareClock,
    processing: Instant,
    header: RawHeader,
) -> Result<ExperimentalMeasurement, MeasurementRefusal> {
    let profile = profile.ok_or(MeasurementRefusal::Unavailable)?;
    require_time_reply(header)?;
    check_capture_order(send_before, send_after, read, processing)?;
    let t1 = convert_wall(send_before.utc)?;
    let after = convert_wall(send_after.utc)?;
    let t4 = convert_wall(read.utc)?;
    let after_residual = coherence_residual(send_before, send_after, t1, after)?;
    let read_residual = coherence_residual(send_before, read, t1, t4)?;
    let observed_residual_ns = after_residual.max(read_residual);
    if observed_residual_ns > profile.maximum_residual_ns {
        return Err(MeasurementRefusal::MappingDiscontinuity);
    }
    require_in_scope(
        profile.dates,
        &[t1.coordinate, after.coordinate, t4.coordinate],
    )?;
    let receive = header.receive_timestamp();
    let transmit = header.transmit_timestamp();
    let t2 =
        unfold_ntp(receive.seconds(), receive.fraction(), &profile.dates).map_err(era_refusal)?;
    let t3 =
        unfold_ntp(transmit.seconds(), transmit.fraction(), &profile.dates).map_err(era_refusal)?;
    let arithmetic = exchange_arithmetic(t1.coordinate, t2, t3, t4.coordinate)
        .map_err(|_| MeasurementRefusal::Arithmetic)?;
    let earliest = t1
        .coordinate
        .min(after.coordinate)
        .min(t2)
        .min(t3)
        .min(t4.coordinate);
    let latest = t1
        .coordinate
        .max(after.coordinate)
        .max(t2)
        .max(t3)
        .max(t4.coordinate);
    let software_window = EraWindow::new(earliest.ticks(), latest.ticks())
        .map_err(|_| MeasurementRefusal::Arithmetic)?;
    let acquisition_age = processing
        .checked_duration_since(read.mono_after)
        .ok_or(MeasurementRefusal::CaptureOrder)?;
    Ok(ExperimentalMeasurement {
        coordinates: [t1.coordinate, t2, t3, t4.coordinate],
        arithmetic,
        local_fractional_remainders: [t1.remainder, t4.remainder],
        observed_residual_ns,
        declared_limit_ns: profile.maximum_residual_ns,
        acquisition_age,
        software_window,
    })
}

/// Admit only ordinary version-four server replies before numerical conversion.
///
/// # Errors
/// Rejects control/unsupported/unsynchronized headers and either zero timestamp.
fn require_time_reply(header: RawHeader) -> Result<(), MeasurementRefusal> {
    if header.version() != 4
        || header.mode() != 4
        || !(1..=15).contains(&header.stratum())
        || header.leap_indicator() == 3
        || header.receive_timestamp().bits() == 0
        || header.transmit_timestamp().bits() == 0
    {
        return Err(MeasurementRefusal::Protocol);
    }
    Ok(())
}

/// Require ordered nonoverlapping captures through the supplied processing read.
///
/// # Errors
/// Rejects a reversed capture, send/receive sequence or processing-before-read.
fn check_capture_order(
    send_before: SoftwareClock,
    send_after: SoftwareClock,
    read: SoftwareClock,
    processing: Instant,
) -> Result<(), MeasurementRefusal> {
    let ordered = [
        send_before.mono,
        send_before.mono_after,
        send_after.mono,
        send_after.mono_after,
        read.mono,
        read.mono_after,
        processing,
    ];
    if ordered
        .windows(2)
        .any(|pair| matches!(pair, [earlier, later] if earlier > later))
    {
        return Err(MeasurementRefusal::CaptureOrder);
    }
    Ok(())
}

/// Convert an actual supplied wall value, including pre-Unix values, without I/O.
///
/// # Errors
/// Rejects nonrepresentable signed decimal, scaled, coordinate or remainder data.
fn convert_wall(wall: SystemTime) -> Result<ConvertedWall, MeasurementRefusal> {
    let unix_nanoseconds = match wall.duration_since(UNIX_EPOCH) {
        Ok(elapsed) => signed_nanoseconds(elapsed)?,
        Err(earlier) => signed_nanoseconds(earlier.duration())?
            .checked_neg()
            .ok_or(MeasurementRefusal::Arithmetic)?,
    };
    let offset = NTP_UNIX_OFFSET_SECONDS
        .checked_mul(NANOS_PER_SECOND)
        .ok_or(MeasurementRefusal::Arithmetic)?;
    let nanoseconds = unix_nanoseconds
        .checked_add(offset)
        .ok_or(MeasurementRefusal::Arithmetic)?;
    let scaled = nanoseconds
        .checked_mul(Q32_SCALE)
        .ok_or(MeasurementRefusal::Arithmetic)?;
    let coordinate = UtcCoordinateQ32::new(scaled.div_euclid(NANOS_PER_SECOND))
        .map_err(|_| MeasurementRefusal::Arithmetic)?;
    let remainder = u32::try_from(scaled.rem_euclid(NANOS_PER_SECOND))
        .map_err(|_| MeasurementRefusal::Arithmetic)?;
    Ok(ConvertedWall {
        nanoseconds,
        coordinate,
        remainder,
    })
}

/// Convert a nonnegative duration to signed nanoseconds with checked narrowing.
///
/// # Errors
/// Rejects durations beyond the signed arithmetic envelope.
fn signed_nanoseconds(value: Duration) -> Result<i128, MeasurementRefusal> {
    i128::try_from(value.as_nanos()).map_err(|_| MeasurementRefusal::Arithmetic)
}

/// Calculate the minimum wall-progress gap from the possible read-time interval.
///
/// This value can refute a declared model, not prove physical clock continuity.
///
/// # Errors
/// Rejects crossed capture brackets or nonrepresentable interval/gap arithmetic.
fn coherence_residual(
    base: SoftwareClock,
    later: SoftwareClock,
    base_wall: ConvertedWall,
    later_wall: ConvertedWall,
) -> Result<u64, MeasurementRefusal> {
    let lower = signed_nanoseconds(
        later
            .mono
            .checked_duration_since(base.mono_after)
            .ok_or(MeasurementRefusal::CaptureOrder)?,
    )?;
    let upper = signed_nanoseconds(
        later
            .mono_after
            .checked_duration_since(base.mono)
            .ok_or(MeasurementRefusal::CaptureOrder)?,
    )?;
    let wall_delta = later_wall
        .nanoseconds
        .checked_sub(base_wall.nanoseconds)
        .ok_or(MeasurementRefusal::Arithmetic)?;
    let gap = if wall_delta < lower {
        lower
            .checked_sub(wall_delta)
            .ok_or(MeasurementRefusal::Arithmetic)?
    } else if wall_delta > upper {
        wall_delta
            .checked_sub(upper)
            .ok_or(MeasurementRefusal::Arithmetic)?
    } else {
        0
    };
    u64::try_from(gap).map_err(|_| MeasurementRefusal::Arithmetic)
}

/// Check every nominal local point against the supplied independent date scope.
///
/// # Errors
/// Rejects any point outside the immutable inclusive profile; never narrows it.
fn require_in_scope(
    dates: EraWindow,
    coordinates: &[UtcCoordinateQ32; 3],
) -> Result<(), MeasurementRefusal> {
    if coordinates
        .iter()
        .any(|value| *value < dates.earliest() || *value > dates.latest())
    {
        return Err(MeasurementRefusal::DateScope);
    }
    Ok(())
}

/// Preserve zero/one/multiple era outcomes; map other exact-domain failures.
fn era_refusal(error: DomainError) -> MeasurementRefusal {
    match error {
        DomainError::NoEraCandidate => MeasurementRefusal::EraNoCandidate,
        DomainError::AmbiguousEra => MeasurementRefusal::EraAmbiguous,
        DomainError::CoordinateOutOfRange
        | DomainError::InvalidEraWindow
        | DomainError::ArithmeticOverflow
        | DomainError::DurationOutOfOrder
        | DomainError::NanosecondsOutOfRange
        | DomainError::ResidualOutOfRange => MeasurementRefusal::Arithmetic,
    }
}

#[cfg(test)]
mod tests {
    //! Independent literal arithmetic and invalid/valid neighbors at the consumer.

    use std::time::{Duration, Instant, UNIX_EPOCH};

    use ntp_wire::{RawHeader, decode_fixed_header};

    use super::{
        ExperimentalProfile, MAX_DECLARED_RESIDUAL_NS, MeasurementRefusal, NANOS_PER_SECOND,
        NTP_UNIX_OFFSET_SECONDS, Q32_SCALE, SoftwareClock, convert_wall, measure,
    };

    /// Nonzero NTP-era-zero base, independent of the host's current UTC reading.
    const BASE: u64 = 4_000_000_000;
    /// Exact whole-second raw timestamp for the common fixture base.
    const BASE_BITS: u64 = BASE << 32;

    /// Build captures on the Windows-compatible 100 ns wall-representation grid.
    /// Fixture construction reads no wall clock and declares no physical accuracy.
    ///
    /// # Errors
    /// Rejects a noncanonical or non-100-ns-aligned fraction, or an unrepresentable
    /// checked monotonic/system-wall fixture time on the current platform.
    fn capture(
        base: Instant,
        mono_ns: u64,
        ntp_seconds: u64,
        nanos: u64,
    ) -> Result<SoftwareClock, MeasurementRefusal> {
        if nanos >= 1_000_000_000 || !nanos.is_multiple_of(100) {
            return Err(MeasurementRefusal::Arithmetic);
        }
        let mono = base
            .checked_add(Duration::from_nanos(mono_ns))
            .ok_or(MeasurementRefusal::Arithmetic)?;
        let wall = if let Some(seconds) = ntp_seconds.checked_sub(2_208_988_800) {
            UNIX_EPOCH.checked_add(Duration::from_secs(seconds))
        } else {
            UNIX_EPOCH.checked_sub(Duration::from_secs(2_208_988_800 - ntp_seconds))
        }
        .and_then(|value| value.checked_add(Duration::from_nanos(nanos)))
        .ok_or(MeasurementRefusal::Arithmetic)?;
        Ok(SoftwareClock {
            mono,
            utc: wall,
            mono_after: mono,
        })
    }

    /// Encode exact inert peer bits and an arbitrary independent matching token.
    ///
    /// # Errors
    /// Returns `Protocol` for an unexpected decoder rejection of the complete
    /// fixed-size 48-byte fixture.
    fn header(
        receive: u64,
        transmit: u64,
        wire_token: u64,
    ) -> Result<RawHeader, MeasurementRefusal> {
        let mut bytes = [0; 48];
        bytes[0] = 0x24;
        bytes[1] = 1;
        bytes[24..32].copy_from_slice(&wire_token.to_be_bytes());
        bytes[32..40].copy_from_slice(&receive.to_be_bytes());
        bytes[40..48].copy_from_slice(&transmit.to_be_bytes());
        decode_fixed_header(&bytes).map_err(|_| MeasurementRefusal::Protocol)
    }

    /// The supplied dyadic exchange has literal +1/64-second phase and 5/64 RTT.
    ///
    /// # Errors
    /// Propagates fixture/profile construction failures or an unexpected refusal
    /// of either healthy dyadic measurement.
    ///
    /// # Panics
    /// Panics if changing the token changes arithmetic, or literal phase, RTT,
    /// residual and acquisition-age expectations differ from the result.
    #[test]
    fn exact_dyadic_results_ignore_the_wire_token() -> Result<(), MeasurementRefusal> {
        let start = Instant::now();
        let before = capture(start, 0, BASE, 0)?;
        let after = capture(start, 1_000_000, BASE, 1_000_000)?;
        let read = capture(start, 85_937_500, BASE, 85_937_500)?;
        let processing = start
            .checked_add(Duration::from_millis(100))
            .ok_or(MeasurementRefusal::Arithmetic)?;
        let profile = Some(ExperimentalProfile::new(BASE, BASE + 1, 0)?);
        let first = measure(
            profile,
            before,
            after,
            read,
            processing,
            header(BASE_BITS + (7 << 25), BASE_BITS + (8 << 25), 1)?,
        )?;
        let second = measure(
            profile,
            before,
            after,
            read,
            processing,
            header(BASE_BITS + (7 << 25), BASE_BITS + (8 << 25), u64::MAX)?,
        )?;
        assert_eq!(first, second);
        assert_eq!(first.arithmetic().offset().numerator(), 134_217_728);
        assert_eq!(first.arithmetic().round_trip().numerator(), 335_544_320);
        assert_eq!(first.local_fractional_remainders(), [0, 0]);
        assert_eq!(first.observed_residual_ns(), 0);
        assert_eq!(first.acquisition_age(), Duration::from_nanos(14_062_500));
        Ok(())
    }

    /// Equal peer shifts reverse phase without altering the independent RTT.
    ///
    /// # Errors
    /// Propagates fixture/profile construction failures or an unexpected refusal
    /// of the shifted healthy measurement.
    ///
    /// # Panics
    /// Panics if the common peer shift fails to reverse phase or changes raw RTT.
    #[test]
    fn peer_shift_changes_sign_without_changing_rtt() -> Result<(), MeasurementRefusal> {
        let start = Instant::now();
        let before = capture(start, 0, BASE, 0)?;
        let after = capture(start, 0, BASE, 0)?;
        let read = capture(start, 85_937_500, BASE, 85_937_500)?;
        let result = measure(
            Some(ExperimentalProfile::new(BASE, BASE + 1, 0)?),
            before,
            after,
            read,
            read.mono_after,
            header(BASE_BITS + (3 << 25), BASE_BITS + (4 << 25), 7)?,
        )?;
        assert_eq!(result.arithmetic().offset().numerator(), -134_217_728);
        assert_eq!(result.arithmetic().round_trip().numerator(), 335_544_320);
        Ok(())
    }

    /// Short local elapsed time preserves a negative signed raw network RTT.
    ///
    /// # Errors
    /// Propagates fixture/profile construction failures or an unexpected refusal
    /// of the negative-RTT or half-tick measurement.
    ///
    /// # Panics
    /// Panics if negative raw RTT or the odd phase numerator is clamped or changed.
    #[test]
    fn negative_raw_rtt_and_half_q32_phase_are_preserved() -> Result<(), MeasurementRefusal> {
        let start = Instant::now();
        let before = capture(start, 0, BASE, 0)?;
        let read = capture(start, 7_812_500, BASE, 7_812_500)?;
        let profile = Some(ExperimentalProfile::new(BASE, BASE + 1, 0)?);
        let negative = measure(
            profile,
            before,
            before,
            read,
            read.mono_after,
            header(BASE_BITS + (7 << 25), BASE_BITS + (9 << 25), 8)?,
        )?;
        assert_eq!(negative.arithmetic().round_trip().numerator(), -33_554_432);
        let half = measure(
            profile,
            before,
            before,
            before,
            before.mono_after,
            header(BASE_BITS, BASE_BITS + 1, 8)?,
        )?;
        assert_eq!(half.arithmetic().offset().numerator(), 1);
        assert_eq!(half.arithmetic().round_trip().numerator(), -1);
        Ok(())
    }

    /// Portable 100/200 ns inputs retain local residuals beyond their Q32 floors.
    ///
    /// # Errors
    /// Propagates fixture/profile construction failures or an unexpected refusal
    /// of the nanosecond-input measurement.
    ///
    /// # Panics
    /// Panics if either remainder or exact integer reconstruction disagrees with
    /// the independently specified nanosecond values.
    #[test]
    fn fractional_conversion_residuals_reconstruct_nominal_values() -> Result<(), MeasurementRefusal>
    {
        let start = Instant::now();
        let before = capture(start, 0, BASE, 100)?;
        let read = capture(start, 100, BASE, 200)?;
        let result = measure(
            Some(ExperimentalProfile::new(BASE, BASE, 0)?),
            before,
            before,
            read,
            read.mono_after,
            header(BASE_BITS, BASE_BITS, 9)?,
        )?;
        let [r1, r4] = result.local_fractional_remainders();
        assert_eq!([r1, r4], [496_729_600, 993_459_200]);
        assert_eq!(result.arithmetic().offset().numerator(), -1_287);
        assert_eq!(result.arithmetic().round_trip().numerator(), 429);
        assert_eq!(
            result.arithmetic().offset().numerator() * NANOS_PER_SECOND
                - i128::from(r1)
                - i128::from(r4),
            -300 * Q32_SCALE
        );
        assert_eq!(
            result.arithmetic().round_trip().numerator() * NANOS_PER_SECOND + i128::from(r4)
                - i128::from(r1),
            100 * Q32_SCALE
        );
        Ok(())
    }

    /// Pre/post-Unix 100 ns wall neighbors retain their literal binary residuals.
    ///
    /// # Errors
    /// Propagates unavailable checked pre/post-epoch `SystemTime` construction
    /// or conversion of those otherwise healthy 100-nanosecond neighbors.
    ///
    /// # Panics
    /// Panics if either neighbor loses its literal Q32 floor or remainder.
    #[test]
    fn pre_unix_and_post_unix_100_ns_have_literal_residuals() -> Result<(), MeasurementRefusal> {
        let before = UNIX_EPOCH
            .checked_sub(Duration::from_nanos(100))
            .ok_or(MeasurementRefusal::Arithmetic)?;
        let after = UNIX_EPOCH
            .checked_add(Duration::from_nanos(100))
            .ok_or(MeasurementRefusal::Arithmetic)?;
        let old = convert_wall(before)?;
        let new = convert_wall(after)?;
        let epoch_ticks = NTP_UNIX_OFFSET_SECONDS * Q32_SCALE;
        assert_eq!(old.coordinate.ticks(), epoch_ticks - 430);
        assert_eq!(old.remainder, 503_270_400);
        assert_eq!(new.coordinate.ticks(), epoch_ticks + 429);
        assert_eq!(new.remainder, 496_729_600);
        Ok(())
    }

    /// Missing explicit assumptions preserve absence rather than supplying zero.
    ///
    /// # Errors
    /// Propagates failure to construct the supplied capture or nonzero peer header.
    ///
    /// # Panics
    /// Panics if absent context produces anything other than `Unavailable`.
    #[test]
    fn absent_profile_is_a_distinct_transport_only_reason() -> Result<(), MeasurementRefusal> {
        let now = Instant::now();
        let clock = capture(now, 0, BASE, 0)?;
        assert_eq!(
            measure(
                None,
                clock,
                clock,
                clock,
                now,
                header(BASE_BITS, BASE_BITS, 1)?
            ),
            Err(MeasurementRefusal::Unavailable)
        );
        Ok(())
    }

    /// Exact inclusive allowance succeeds while its immediate lower neighbor fails.
    ///
    /// # Errors
    /// Propagates fixture/profile construction failures or an unexpected refusal
    /// at the exact inclusive coherence allowance.
    ///
    /// # Panics
    /// Panics if the accepted residual/allowance differs or the lower allowance
    /// fails to produce `MappingDiscontinuity`.
    #[test]
    fn declared_coherence_limit_has_an_exact_healthy_neighbor() -> Result<(), MeasurementRefusal> {
        let start = Instant::now();
        let before = capture(start, 0, BASE, 0)?;
        let read = capture(start, 10_000_000, BASE, 20_000_000)?;
        let raw = header(BASE_BITS, BASE_BITS, 2)?;
        let accepted = measure(
            Some(ExperimentalProfile::new(BASE, BASE, 10_000_000)?),
            before,
            before,
            read,
            read.mono_after,
            raw,
        )?;
        assert_eq!(accepted.observed_residual_ns(), 10_000_000);
        assert_eq!(accepted.declared_limit_ns(), 10_000_000);
        assert_eq!(
            measure(
                Some(ExperimentalProfile::new(BASE, BASE, 9_999_999)?),
                before,
                before,
                read,
                read.mono_after,
                raw
            ),
            Err(MeasurementRefusal::MappingDiscontinuity)
        );
        Ok(())
    }

    /// Portable capture brackets admit elapsed wall progress in [600, 1600] ns.
    ///
    /// # Errors
    /// Propagates fixture/profile construction failures or refusal of a specified
    /// inside, exact-boundary or 100-nanosecond-allowance healthy measurement.
    ///
    /// # Panics
    /// Panics if endpoints or 100-nanosecond-grid neighbors fail their literal
    /// residuals, including the one-nanosecond allowance boundary at 99/100 ns.
    #[test]
    fn nonzero_capture_intervals_keep_inside_and_outside_neighbors()
    -> Result<(), MeasurementRefusal> {
        let start = Instant::now();
        let mut before = capture(start, 1_000, BASE, 0)?;
        before.mono_after = start
            .checked_add(Duration::from_nanos(1_400))
            .ok_or(MeasurementRefusal::Arithmetic)?;
        let mut read = capture(start, 3_000, BASE, 2_000)?;
        read.mono_after = start
            .checked_add(Duration::from_nanos(3_600))
            .ok_or(MeasurementRefusal::Arithmetic)?;
        let raw = header(BASE_BITS, BASE_BITS, 11)?;
        let strict = Some(ExperimentalProfile::new(BASE, BASE, 0)?);
        let just_below = Some(ExperimentalProfile::new(BASE, BASE, 99)?);
        let neighbor = Some(ExperimentalProfile::new(BASE, BASE, 100)?);
        for wall_delta in [600, 1_200, 1_600] {
            let mut after = capture(start, 2_000, BASE, wall_delta)?;
            after.mono_after = start
                .checked_add(Duration::from_nanos(2_600))
                .ok_or(MeasurementRefusal::Arithmetic)?;
            let accepted = measure(strict, before, after, read, read.mono_after, raw)?;
            assert_eq!(accepted.observed_residual_ns(), 0);
        }
        for wall_delta in [500, 1_700] {
            let mut after = capture(start, 2_000, BASE, wall_delta)?;
            after.mono_after = start
                .checked_add(Duration::from_nanos(2_600))
                .ok_or(MeasurementRefusal::Arithmetic)?;
            assert_eq!(
                measure(strict, before, after, read, read.mono_after, raw),
                Err(MeasurementRefusal::MappingDiscontinuity)
            );
            assert_eq!(
                measure(just_below, before, after, read, read.mono_after, raw),
                Err(MeasurementRefusal::MappingDiscontinuity)
            );
            let accepted = measure(neighbor, before, after, read, read.mono_after, raw)?;
            assert_eq!(accepted.observed_residual_ns(), 100);
            assert_eq!(accepted.declared_limit_ns(), 100);
        }
        Ok(())
    }

    /// Bad bracket/send/processing order rejects beside current ordered captures.
    ///
    /// # Errors
    /// Propagates failure to construct the capture, profile or header neighbors.
    ///
    /// # Panics
    /// Panics if reversed capture/processing order is accepted or the ordered
    /// healthy neighbor is refused.
    #[test]
    fn reversed_brackets_and_processing_are_not_ages() -> Result<(), MeasurementRefusal> {
        let start = Instant::now();
        let first = capture(start, 0, BASE, 0)?;
        let later = capture(start, 100, BASE, 100)?;
        let profile = Some(ExperimentalProfile::new(BASE, BASE, 0)?);
        let raw = header(BASE_BITS, BASE_BITS, 3)?;
        assert_eq!(
            measure(profile, first, later, first, later.mono_after, raw),
            Err(MeasurementRefusal::CaptureOrder)
        );
        assert_eq!(
            measure(profile, first, first, later, start, raw),
            Err(MeasurementRefusal::CaptureOrder)
        );
        assert!(measure(profile, first, first, later, later.mono_after, raw).is_ok());
        Ok(())
    }

    /// Host values cannot enlarge the independent date scope or select an era.
    ///
    /// # Errors
    /// Propagates failure to construct the independent scope or supplied fixture.
    ///
    /// # Panics
    /// Panics if the outside local value is rebased or yields the wrong refusal.
    #[test]
    fn out_of_scope_local_coordinate_is_not_rebased() -> Result<(), MeasurementRefusal> {
        let start = Instant::now();
        let clock = capture(start, 0, BASE - 1, 0)?;
        assert_eq!(
            measure(
                Some(ExperimentalProfile::new(BASE, BASE, 0)?),
                clock,
                clock,
                clock,
                start,
                header(BASE_BITS, BASE_BITS, 4)?
            ),
            Err(MeasurementRefusal::DateScope)
        );
        Ok(())
    }

    /// Send-after remains part of date admission even when nominal endpoints fit.
    ///
    /// # Errors
    /// Propagates capture/header construction or explicit profile setup failures.
    ///
    /// # Panics
    /// Panics if the send-after excursion disappears from date-scope admission.
    #[test]
    fn send_after_outside_scope_cannot_hide_between_valid_endpoints()
    -> Result<(), MeasurementRefusal> {
        let start = Instant::now();
        let before = capture(start, 0, BASE, 0)?;
        let after = capture(start, 100, BASE + 2, 0)?;
        let read = capture(start, 200, BASE, 200)?;
        assert_eq!(
            measure(
                Some(ExperimentalProfile::new(BASE, BASE, 3_000_000_000)?),
                before,
                after,
                read,
                read.mono_after,
                header(BASE_BITS, BASE_BITS, 5)?
            ),
            Err(MeasurementRefusal::DateScope)
        );
        Ok(())
    }

    /// Independent windows retain both absent and ambiguous peer-era outcomes.
    ///
    /// # Errors
    /// Propagates construction failures for the supplied captures, peer headers
    /// or independent narrow/wide date profiles.
    ///
    /// # Panics
    /// Panics if zero and multiple candidate outcomes merge or a date fallback
    /// replaces either required era refusal.
    #[test]
    fn era_zero_and_multiple_candidates_remain_distinct() -> Result<(), MeasurementRefusal> {
        let start = Instant::now();
        let clock = capture(start, 0, BASE, 0)?;
        assert_eq!(
            measure(
                Some(ExperimentalProfile::new(BASE, BASE, 0)?),
                clock,
                clock,
                clock,
                start,
                header((BASE + 2) << 32, (BASE + 2) << 32, 6)?
            ),
            Err(MeasurementRefusal::EraNoCandidate)
        );
        assert_eq!(
            measure(
                Some(ExperimentalProfile::new(0, 1_u64 << 32, 0)?),
                clock,
                clock,
                clock,
                start,
                header(1, 1, 6)?
            ),
            Err(MeasurementRefusal::EraAmbiguous)
        );
        Ok(())
    }

    /// Direct numerical callers cannot admit controls, bad modes or zero stamps.
    ///
    /// # Errors
    /// Propagates fixture decoding or capture/profile setup failures.
    ///
    /// # Panics
    /// Panics if a control, bad version/mode/LI or zero timestamp is measured,
    /// or if the ordinary healthy neighbor is refused.
    #[test]
    fn inert_header_does_not_bypass_the_time_reply_guard() -> Result<(), MeasurementRefusal> {
        let start = Instant::now();
        let clock = capture(start, 0, BASE, 0)?;
        let profile = Some(ExperimentalProfile::new(BASE, BASE, 0)?);
        for (flags, stratum, receive, transmit) in [
            (0x24, 0, BASE_BITS, BASE_BITS),
            (0x24, 16, BASE_BITS, BASE_BITS),
            (0xe4, 1, BASE_BITS, BASE_BITS),
            (0x1c, 1, BASE_BITS, BASE_BITS),
            (0x23, 1, BASE_BITS, BASE_BITS),
            (0x24, 1, 0, BASE_BITS),
            (0x24, 1, BASE_BITS, 0),
        ] {
            let raw = header(receive, transmit, 7)?;
            let mut bytes = [0; 48];
            bytes[0] = flags;
            bytes[1] = stratum;
            bytes[32..40].copy_from_slice(&raw.receive_timestamp().bits().to_be_bytes());
            bytes[40..48].copy_from_slice(&raw.transmit_timestamp().bits().to_be_bytes());
            let invalid = decode_fixed_header(&bytes).map_err(|_| MeasurementRefusal::Protocol)?;
            assert_eq!(
                measure(profile, clock, clock, clock, start, invalid),
                Err(MeasurementRefusal::Protocol)
            );
        }
        assert!(
            measure(
                profile,
                clock,
                clock,
                clock,
                start,
                header(BASE_BITS, BASE_BITS, 7)?
            )
            .is_ok()
        );
        Ok(())
    }

    /// Whole-second endpoints retain all fractions and exact configured bounds.
    ///
    /// # Errors
    /// Propagates an unexpected refusal of the legal maximum-allowance profile.
    ///
    /// # Panics
    /// Panics if original bounds/full fractions change or an invalid bound or
    /// excessive-allowance neighbor is admitted.
    #[test]
    fn profile_bounds_and_maximum_allowance_are_checked() -> Result<(), MeasurementRefusal> {
        let profile = ExperimentalProfile::new(BASE, BASE, MAX_DECLARED_RESIDUAL_NS)?;
        assert_eq!(profile.earliest_ntp_seconds(), BASE);
        assert_eq!(profile.latest_ntp_seconds(), BASE);
        assert_eq!(profile.maximum_residual_ns(), MAX_DECLARED_RESIDUAL_NS);
        assert_eq!(profile.dates().earliest().ticks(), i128::from(BASE_BITS));
        assert_eq!(
            profile.dates().latest().ticks(),
            i128::from(BASE_BITS) + Q32_SCALE - 1
        );
        assert_eq!(
            ExperimentalProfile::new(2, 1, 0),
            Err(MeasurementRefusal::InvalidProfile)
        );
        assert_eq!(
            ExperimentalProfile::new(0, 1, MAX_DECLARED_RESIDUAL_NS + 1),
            Err(MeasurementRefusal::InvalidProfile)
        );
        assert_eq!(
            ExperimentalProfile::new(i64::MAX.unsigned_abs() + 1, i64::MAX.unsigned_abs() + 1, 0),
            Err(MeasurementRefusal::InvalidProfile)
        );
        Ok(())
    }

    /// Final age replacement preserves the checked coordinates and model evidence.
    ///
    /// # Errors
    /// Propagates fixture/profile construction or unexpected refusal of the
    /// initial healthy measurement.
    ///
    /// # Panics
    /// Panics if the fresh age differs or stamping alters the checked coordinates,
    /// arithmetic or nominal software window.
    #[test]
    fn fresh_owner_age_stamp_preserves_the_actual_arithmetic() -> Result<(), MeasurementRefusal> {
        let start = Instant::now();
        let clock = capture(start, 0, BASE, 0)?;
        let staged = measure(
            Some(ExperimentalProfile::new(BASE, BASE, 0)?),
            clock,
            clock,
            clock,
            start,
            header(BASE_BITS, BASE_BITS, 1)?,
        )?;
        let committed = staged.with_acquisition_age(Duration::from_micros(25));
        assert_eq!(committed.acquisition_age(), Duration::from_micros(25));
        assert_eq!(committed.coordinates(), staged.coordinates());
        assert_eq!(committed.arithmetic(), staged.arithmetic());
        assert_eq!(committed.software_window(), staged.software_window());
        Ok(())
    }
}

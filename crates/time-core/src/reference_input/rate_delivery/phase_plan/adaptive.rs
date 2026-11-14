/*
Original reference.c notice for the adaptive correction-area rule:
Copyright (C) Richard P. Curnow  1997-2003
Copyright (C) Miroslav Lichvar  2009-2018, 2020, 2022
Copyright (C) Andy Fiddaman  2024

This program is free software; you can redistribute it and/or modify
it under the terms of version 2 of the GNU General Public License as
published by the Free Software Foundation.

This program is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public License along
with this program; if not, write to the Free Software Foundation, Inc.,
51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.
*/
// SPDX-License-Identifier: GPL-2.0-only
//! Checked adaptive correction-area and horizon policy inside the finite owner.
//!
//! David Osipov with AI assistance, 13 November 2026: adapted from the pinned
//! Chrony 4.9 reference.c get_correction_rate/REF_SetReference and sys_generic.c
//! update_slew. Extracted into this private owner on 14 November without changing
//! checked admission, arithmetic order, exact ceiling or integer-clamp semantics.
//! The parent retains actual accepted-reference custody, active timestamps,
//! obligation deadlines, issue/settle transitions and all permission decisions.
//! This computed active-time constant supplies no physical clock conversion,
//! native timer heuristic, shipped default, source eligibility or clock authority.
//! Source provenance remains in the crate's PROVENANCE.md and the existing
//! tasks/s1.3-source-selection-readiness.md chain. Failed Garde admission may
//! allocate its bounded temporary report; retained state and success are bounded.

use std::fmt;

use garde::Validate;
use time_domain::{ActiveAwakeDurationNs, ActiveAwakeNs};

use super::FixturePhaseError;
use crate::regression::{ArithmeticOperation, ArithmeticStage, FitError, PhaseDebt, arithmetic};

/// Largest integer nanosecond count converted exactly into binary64 by this owner.
const ADAPTIVE_MAX_NANOSECONDS: u64 = 1_u64 << 53;

/// Explicit checked adaptive correction parameters; these select no shipped policy.
///
/// Construction uses exact typed active durations and the mandatory checked API:
/// ```
/// use time_core::FixtureAdaptiveHorizonProfile;
/// use time_domain::ActiveAwakeNs;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let zero = ActiveAwakeNs::new(0);
/// let minimum = ActiveAwakeNs::new(1_000_000_000).duration_since(zero)?;
/// let maximum = ActiveAwakeNs::new(16_000_000_000).duration_since(zero)?;
/// let _policy = FixtureAdaptiveHorizonProfile::new(4.0, minimum, maximum, 0.0)?;
/// # Ok(()) }
/// ```
/// Raw fields cannot bypass profile admission:
/// ```compile_fail,E0451
/// use time_core::FixtureAdaptiveHorizonProfile;
/// use time_domain::ActiveAwakeDurationNs;
/// fn bypass(minimum: ActiveAwakeDurationNs, maximum: ActiveAwakeDurationNs)
///     -> FixtureAdaptiveHorizonProfile {
///     FixtureAdaptiveHorizonProfile {
///         correction_time_ratio: 4.0,
///         minimum,
///         maximum,
///         small_debt_threshold: 0.0,
///     }
/// }
/// ```
/// An admitted profile cannot be mutated into an unchecked one:
/// ```compile_fail,E0616
/// use time_core::FixtureAdaptiveHorizonProfile;
/// fn bypass(mut profile: FixtureAdaptiveHorizonProfile) {
///     profile.correction_time_ratio = 0.0;
/// }
/// ```
#[derive(Clone, Copy)]
pub struct FixtureAdaptiveHorizonProfile {
    /// Positive normal ratio multiplying the statistical phase deviation and interval.
    correction_time_ratio: f64,
    /// Positive exact active-time lower bound on the computed time constant.
    minimum: ActiveAwakeDurationNs,
    /// Positive exact upper bound, independent of the obligation's permission budget.
    maximum: ActiveAwakeDurationNs,
    /// Finite nonnegative comparison threshold; small debt is not silently zeroed.
    small_debt_threshold: f64,
}

/// Check the complete immutable profile without reading clocks or changing inputs.
pub(super) fn adaptive_profile_valid(value: &FixtureAdaptiveHorizonProfile) -> bool {
    value.correction_time_ratio.is_normal()
        && value.correction_time_ratio > 0.0
        && value.minimum.nanoseconds() > 0
        && value.minimum.nanoseconds() <= value.maximum.nanoseconds()
        && value.maximum.nanoseconds() <= ADAPTIVE_MAX_NANOSECONDS
        && value.small_debt_threshold.is_finite()
        && value.small_debt_threshold >= 0.0
}

#[derive(Validate)]
/// One mandatory shared rule covers every raw adaptive parameter.
struct RawAdaptiveProfile {
    #[garde(custom(|value: &FixtureAdaptiveHorizonProfile, (): &()| {
        if adaptive_profile_valid(value) { Ok(()) } else { Err(garde::Error::new("invalid adaptive horizon profile")) }
    }))]
    /// Whole scalar/duration relationship, including the supported exact integer bound.
    value: FixtureAdaptiveHorizonProfile,
}

impl FixtureAdaptiveHorizonProfile {
    /// Admit explicit parameters without constraining the original obligation lifetime.
    /// # Errors
    /// Reject an unsupported ratio, unordered/unsupported durations, threshold or validation.
    pub fn new(
        correction_time_ratio: f64,
        minimum: ActiveAwakeDurationNs,
        maximum: ActiveAwakeDurationNs,
        small_debt_threshold: f64,
    ) -> Result<Self, FixturePhaseError> {
        let raw = RawAdaptiveProfile {
            value: Self {
                correction_time_ratio,
                minimum,
                maximum,
                small_debt_threshold,
            },
        };
        let validated = raw.validate();
        if !adaptive_profile_valid(&raw.value) {
            return Err(FixturePhaseError::InvalidProfile);
        }
        validated.map_err(|_| FixturePhaseError::ValidationInvariant)?;
        Ok(raw.value)
    }
}

impl fmt::Debug for FixtureAdaptiveHorizonProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FixtureAdaptiveHorizonProfile { .. }")
    }
}

/// Closed adaptive-policy refusals; no raw values or input-selected text is retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureAdaptiveHorizonError {
    /// An accepted-reference active interval exceeds exact integer-conversion support.
    UnsupportedInterval,
    /// The supplied statistical deviation violates the normal-or-zero nonnegative rule.
    UnsupportedStatistic,
    /// A separately rounded primitive rejected its operands or result.
    Arithmetic {
        /// Original bounded numerical cause, without operands or computed raw values.
        source: FitError,
    },
    /// Exact ceiling or typed duration construction cannot represent the result.
    DurationConversion,
    /// Mandatory output admission disagreed with its shared invariant.
    ValidationInvariant,
}

impl fmt::Display for FixtureAdaptiveHorizonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::UnsupportedInterval => "adaptive horizon: unsupported reference interval",
            Self::UnsupportedStatistic => "adaptive horizon: unsupported phase deviation",
            Self::Arithmetic { .. } => "adaptive horizon: arithmetic rejected",
            Self::DurationConversion => "adaptive horizon: duration conversion rejected",
            Self::ValidationInvariant => "adaptive horizon: validation invariant",
        })
    }
}

impl std::error::Error for FixtureAdaptiveHorizonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Arithmetic { source } => Some(source),
            Self::UnsupportedInterval
            | Self::UnsupportedStatistic
            | Self::DurationConversion
            | Self::ValidationInvariant => None,
        }
    }
}

/// One immutable computed control area, not an uncertainty or physical-error bound.
#[derive(Clone, Copy)]
pub(super) struct AdaptiveArea {
    /// Nominal displacement seconds times active seconds, from the accepted reference.
    point: f64,
}

/// The complete scalar invariant for a newly computed controller decision value.
fn adaptive_area_valid(point: f64) -> bool {
    (point.is_normal() || point == 0.0) && point >= 0.0
}

#[derive(Validate)]
/// Mandatory checked construction prevents a raw unsupported area from being retained.
struct RawAdaptiveArea {
    #[garde(custom(|point: &f64, (): &()| {
        if adaptive_area_valid(*point) { Ok(()) } else { Err(garde::Error::new("invalid adaptive correction area")) }
    }))]
    /// The scalar is the complete immutable computed-area representation.
    point: f64,
}

/// Wrap a bounded adaptive refusal without flattening an arithmetic source.
pub(super) fn adaptive_error(source: FixtureAdaptiveHorizonError) -> FixturePhaseError {
    FixturePhaseError::AdaptiveHorizon { source }
}

/// Evaluate one separately rounded shared primitive; unsupported results never fall back.
/// # Errors
/// Preserve the shared primitive's exact bounded operand/result refusal.
fn adaptive_arithmetic(
    left: f64,
    right: f64,
    operation: ArithmeticOperation,
) -> Result<f64, FixturePhaseError> {
    arithmetic(left, right, operation, ArithmeticStage::Coefficients)
        .map_err(|source| adaptive_error(FixtureAdaptiveHorizonError::Arithmetic { source }))
}

/// Convert an exact supported active interval to computed seconds without clock inference.
/// # Errors
/// Reject unsupported intervals, impossible integer decomposition or arithmetic failure.
fn adaptive_duration_seconds(duration: ActiveAwakeDurationNs) -> Result<f64, FixturePhaseError> {
    let nanos = duration.nanoseconds();
    if nanos > ADAPTIVE_MAX_NANOSECONDS {
        return Err(adaptive_error(
            FixtureAdaptiveHorizonError::UnsupportedInterval,
        ));
    }
    let invariant = || adaptive_error(FixtureAdaptiveHorizonError::ValidationInvariant);
    let high = u32::try_from(nanos >> 32).map_err(|_| invariant())?;
    let low = u32::try_from(nanos & u64::from(u32::MAX)).map_err(|_| invariant())?;
    // Each u32 conversion is exact; the power-of-two product and sum are exact
    // because the whole supported integer is at most 2^53. No cast is required.
    let upper = adaptive_arithmetic(
        f64::from(high),
        4_294_967_296.0,
        ArithmeticOperation::Multiply,
    )?;
    let exact = adaptive_arithmetic(upper, f64::from(low), ArithmeticOperation::Add)?;
    adaptive_arithmetic(exact, 1_000_000_000.0, ArithmeticOperation::Divide)
}

/// Compute the exact integer ceiling of positive normal binary64 seconds in nanoseconds.
///
/// Decomposing the original bits avoids rounding seconds*1e9 down across an
/// integer boundary. Scratch/work is fixed; no floating-to-integer cast occurs.
/// # Errors
/// Reject unsupported seconds, checked integer overflow or a ceiling above 2^53.
fn adaptive_ceil_nanoseconds(seconds: f64) -> Result<u64, FixturePhaseError> {
    let failure = || adaptive_error(FixtureAdaptiveHorizonError::DurationConversion);
    if !seconds.is_normal() || seconds <= 0.0 {
        return Err(failure());
    }
    let bits = seconds.to_bits();
    let mantissa = (bits & ((1_u64 << 52) - 1)) | (1_u64 << 52);
    let exponent = i32::try_from((bits >> 52) & 0x7ff).map_err(|_| failure())? - 1023 - 52;
    let numerator = u128::from(mantissa)
        .checked_mul(1_000_000_000)
        .ok_or_else(failure)?;
    let ceiling = if exponent >= 0 {
        let shift = u32::try_from(exponent).map_err(|_| failure())?;
        let factor = 1_u128.checked_shl(shift).ok_or_else(failure)?;
        numerator.checked_mul(factor).ok_or_else(failure)?
    } else {
        let shift = u32::try_from(-exponent).map_err(|_| failure())?;
        if shift >= u128::BITS {
            // The numerator is below 2^83, so every such quotient is in (0,1).
            1
        } else {
            let denominator = 1_u128.checked_shl(shift).ok_or_else(failure)?;
            let whole = numerator / denominator;
            whole
                .checked_add(u128::from(numerator % denominator != 0))
                .ok_or_else(failure)?
        }
    };
    if ceiling == 0 || ceiling > u128::from(ADAPTIVE_MAX_NANOSECONDS) {
        return Err(failure());
    }
    u64::try_from(ceiling).map_err(|_| failure())
}

/// Construct an exact same-domain duration from a checked numerical nanosecond count.
/// # Errors
/// Reject zero, unsupported magnitude or typed-duration construction failure.
fn adaptive_duration(nanoseconds: u64) -> Result<ActiveAwakeDurationNs, FixturePhaseError> {
    if nanoseconds == 0 || nanoseconds > ADAPTIVE_MAX_NANOSECONDS {
        return Err(adaptive_error(
            FixtureAdaptiveHorizonError::DurationConversion,
        ));
    }
    ActiveAwakeNs::new(nanoseconds)
        .duration_since(ActiveAwakeNs::new(0))
        .map_err(|_| adaptive_error(FixtureAdaptiveHorizonError::DurationConversion))
}

impl AdaptiveArea {
    /// Admit one computed finite nominal area through its shared whole-output rule.
    /// # Errors
    /// Reject a nonfinite, negative or subnormal value, or validation disagreement.
    fn new(point: f64) -> Result<Self, FixturePhaseError> {
        let raw = RawAdaptiveArea { point };
        let validated = raw.validate();
        if !adaptive_area_valid(raw.point) {
            return Err(adaptive_error(
                FixtureAdaptiveHorizonError::ValidationInvariant,
            ));
        }
        validated.map_err(|_| adaptive_error(FixtureAdaptiveHorizonError::ValidationInvariant))?;
        Ok(Self { point: raw.point })
    }

    /// Form the accepted reference's control area once, preserving donor RN order.
    /// # Errors
    /// Reject unsupported interval/statistics and each unsupported intermediate.
    pub(super) fn from_reference(
        profile: FixtureAdaptiveHorizonProfile,
        deviation: f64,
        elapsed: ActiveAwakeDurationNs,
    ) -> Result<Self, FixturePhaseError> {
        if !(deviation.is_normal() || deviation == 0.0) || deviation < 0.0 {
            return Err(adaptive_error(
                FixtureAdaptiveHorizonError::UnsupportedStatistic,
            ));
        }
        if elapsed.nanoseconds() > ADAPTIVE_MAX_NANOSECONDS {
            return Err(adaptive_error(
                FixtureAdaptiveHorizonError::UnsupportedInterval,
            ));
        }
        if deviation == 0.0 || elapsed.nanoseconds() == 0 {
            return Self::new(0.0);
        }
        let half_ratio = adaptive_arithmetic(
            profile.correction_time_ratio,
            0.5,
            ArithmeticOperation::Multiply,
        )?;
        let scaled_deviation =
            adaptive_arithmetic(half_ratio, deviation, ArithmeticOperation::Multiply)?;
        let seconds = adaptive_duration_seconds(elapsed)?;
        Self::new(adaptive_arithmetic(
            scaled_deviation,
            seconds,
            ArithmeticOperation::Multiply,
        )?)
    }

    /// Select a computed time constant without shortening it to permission remaining.
    ///
    /// The exact selected duration, not an ideal nonlinear law, is passed to the
    /// existing proposal-error owner. Neutral debt never performs area/debt division.
    /// # Errors
    /// Preserve unsupported quotient, duration-conversion or arithmetic failures.
    pub(super) fn horizon(
        self,
        profile: FixtureAdaptiveHorizonProfile,
        debt: PhaseDebt,
    ) -> Result<ActiveAwakeDurationNs, FixturePhaseError> {
        let magnitude = debt.point().abs();
        if magnitude <= debt.error() || magnitude == 0.0 || magnitude < profile.small_debt_threshold
        {
            return Ok(profile.maximum);
        }
        if self.point == 0.0 {
            return Ok(profile.minimum);
        }
        let seconds = adaptive_arithmetic(self.point, magnitude, ArithmeticOperation::Divide)?;
        if seconds >= adaptive_duration_seconds(profile.maximum)? {
            return Ok(profile.maximum);
        }
        let nanoseconds = adaptive_ceil_nanoseconds(seconds)?
            .max(profile.minimum.nanoseconds())
            .min(profile.maximum.nanoseconds());
        adaptive_duration(nanoseconds)
    }
}

#[cfg(test)]
#[path = "../../../../tests/private/adaptive_horizon.rs"]
mod tests;

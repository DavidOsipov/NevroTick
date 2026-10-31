// SPDX-License-Identifier: GPL-2.0-only
//! Exact outward Q33 enclosure of an immutable checked fixture projection.
//!
//! The root bridge depends on the projection and interval owners; regression
//! does not depend on selection. It encloses the exact real values represented
//! by the supplied binary64 center and distance, without rounded floating-point
//! addition or subtraction. Fixed-size integer work admits signed zeros and raw
//! subnormal operands, but neither changes projection admission nor establishes
//! physical uncertainty, observation freshness, agreement or clock authority.
//! Only the original immutable projection and a completed checked interval are
//! published. No history, ambient time, policy or external effect is accessed.

use std::{cmp::Ordering, fmt};

use garde::Validate;

use crate::{FixtureAnchorProjection, FixtureInterval, IntervalError};

/// Exact binary64 representation of one Q33 quantum in seconds, `2^-33`.
const QUANTUM_SECONDS: f64 = f64::from_bits(0x3de0_0000_0000_0000);
/// Necessary exclusive positive operand bound in seconds, `2^30`.
const OPERAND_LIMIT: f64 = 1_073_741_824.0;
/// Largest admitted scaled operand magnitude, including the negative endpoint.
const MAX_SCALED_MAGNITUDE: u64 = 1_u64 << 63;

/// Whether one exact represented endpoint lies on the Q33 integer lattice.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureEndpointRounding {
    /// No fractional Q33 residual remained, including exact cancellation/carry.
    Exact,
    /// A nonzero exact residual required outward floor or ceiling.
    OutwardFraction,
}

/// Bounded enclosure failure without raw operands, labels or validation reports.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum FixtureEnclosureError {
    /// Invalid numerical domain or an endpoint outside the Q33 i64 lattice.
    /// No priority between lower and upper endpoint rejection is promised.
    Unrepresentable,
    /// An impossible checked internal representation contradicted its invariant.
    Invariant,
    /// The interval owner's actual typed construction failure.
    Interval {
        /// Preserved bounded interval cause; it contains no raw fixture input.
        source: IntervalError,
    },
}

impl fmt::Debug for FixtureEnclosureError {
    /// Format only a closed failure code; retained causes remain in `source()`.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Unrepresentable => "FixtureEnclosureError::Unrepresentable",
            Self::Invariant => "FixtureEnclosureError::Invariant",
            Self::Interval { .. } => "FixtureEnclosureError::Interval { .. }",
        })
    }
}

impl fmt::Display for FixtureEnclosureError {
    /// Format fixed redacted text without repeating the retained typed cause.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Unrepresentable => "fixture enclosure: unrepresentable",
            Self::Invariant => "fixture enclosure: invariant",
            Self::Interval { .. } => "fixture enclosure: interval construction",
        })
    }
}

impl std::error::Error for FixtureEnclosureError {
    /// Borrow the actual interval cause; numerical and invariant failures are leaves.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Interval { source } => Some(source),
            Self::Unrepresentable | Self::Invariant => None,
        }
    }
}

/// Immutable fixture-only provenance, interval and endpoint-rounding facts.
#[derive(Clone, Copy)]
pub struct FixtureProjectedInterval {
    /// Original checked projection, preserving all supplied fields and bits.
    projection: FixtureAnchorProjection,
    /// Completed checked Q33 enclosure, labelled only by the projection source.
    interval: FixtureInterval,
    /// Exact lower endpoint's relationship to the integer lattice.
    lower_rounding: FixtureEndpointRounding,
    /// Exact upper endpoint's relationship to the integer lattice.
    upper_rounding: FixtureEndpointRounding,
}

impl fmt::Debug for FixtureProjectedInterval {
    /// Reveal only the type, excluding projection, label, bounds and rounding data.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureProjectedInterval { .. }")
    }
}

impl FixtureProjectedInterval {
    /// Borrow the unchanged original projection without recomputation or effects.
    #[must_use]
    pub const fn projection(&self) -> &FixtureAnchorProjection {
        &self.projection
    }

    /// Borrow the checked closed Q33 enclosure without changing its asserted label.
    #[must_use]
    pub const fn interval(&self) -> &FixtureInterval {
        &self.interval
    }

    /// Copy lower endpoint rounding without any physical confidence assertion.
    #[must_use]
    pub const fn lower_rounding(&self) -> FixtureEndpointRounding {
        self.lower_rounding
    }

    /// Copy upper endpoint rounding without any physical confidence assertion.
    #[must_use]
    pub const fn upper_rounding(&self) -> FixtureEndpointRounding {
        self.upper_rounding
    }

    /// Return the exact routine fixture quantum in seconds, without conversion.
    #[must_use]
    pub const fn quantum_seconds(&self) -> f64 {
        QUANTUM_SECONDS
    }
}

/// Raw candidate with mandatory shared finite/range garde rules on both fields.
#[derive(Validate)]
struct RawEnclosureOperands {
    /// Represented signed center seconds; both zero signs and subnormals are legal.
    #[garde(custom(center_rule))]
    center_seconds: f64,
    /// Represented nonnegative radius seconds; negative zero is legal.
    #[garde(custom(distance_rule))]
    distance_seconds: f64,
}

/// Admit only finite center values within the necessary Q33 enclosure prebound.
///
/// This fixed-work predicate is the sole center rule owner; exact endpoint range
/// remains the integer completion owner's responsibility. Bitwise finiteness and
/// magnitude checks preserve subnormal classification independently of floating
/// input-flush behavior. It retains no input and performs no floating arithmetic.
fn center_admissible(value: f64) -> bool {
    let bits = value.to_bits();
    let magnitude = bits & 0x7fff_ffff_ffff_ffff;
    let limit = OPERAND_LIMIT.to_bits();
    magnitude >> 52 != 0x7ff && (magnitude < limit || (bits >> 63 != 0 && magnitude == limit))
}

/// Admit only finite nonnegative distances below the necessary operand prebound.
///
/// Both signed zeros and positive subnormals pass; this constant-work predicate
/// rejects negative subnormals by sign/magnitude bits even under input flushing.
/// It grants no uncertainty, freshness or selection policy and retains no input.
fn distance_admissible(value: f64) -> bool {
    let bits = value.to_bits();
    let magnitude = bits & 0x7fff_ffff_ffff_ffff;
    magnitude >> 52 != 0x7ff
        && magnitude < OPERAND_LIMIT.to_bits()
        && (bits >> 63 == 0 || magnitude == 0)
}

/// Apply the shared center predicate inside mandatory raw-candidate validation.
/// # Errors
/// Rejects unsupported raw centers with fixed text and no retained scalar.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rules require a borrowed field and borrowed unit context"
)]
fn center_rule(value: &f64, (): &()) -> garde::Result {
    if center_admissible(*value) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid fixture enclosure center"))
    }
}

/// Apply the shared distance predicate inside mandatory raw-candidate validation.
/// # Errors
/// Rejects unsupported raw distances with fixed text and no retained scalar.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rules require a borrowed field and borrowed unit context"
)]
fn distance_rule(value: &f64, (): &()) -> garde::Result {
    if distance_admissible(*value) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid fixture enclosure distance"))
    }
}

/// Canonical proper nonnegative dyadic fraction with at most 53 numerator bits.
#[derive(Clone, Copy)]
struct Fraction {
    /// Zero or a numerator strictly smaller than `2^shift` and `2^53`.
    numerator: u64,
    /// Zero exactly for zero numerator; otherwise an exponent in 1..=1041.
    shift: u32,
}

impl Fraction {
    /// Canonical exact zero; its fields satisfy the private representation directly.
    const ZERO: Self = Self {
        numerator: 0,
        shift: 0,
    };

    /// Check a transformed numerator/denominator and canonicalize exact zero.
    ///
    /// Finite fixed-size values are admitted without allocation or effects.
    /// # Errors
    /// Returns `Invariant` for an improper fraction or an unsupported bit/shift bound.
    fn new(numerator: u64, shift: u32) -> Result<Self, FixtureEnclosureError> {
        if shift > 1041 || numerator >= (1_u64 << 53) {
            return Err(FixtureEnclosureError::Invariant);
        }
        if numerator == 0 {
            return Ok(Self::ZERO);
        }
        if shift == 0 || (shift < 64 && numerator >= (1_u64 << shift)) {
            return Err(FixtureEnclosureError::Invariant);
        }
        Ok(Self { numerator, shift })
    }

    /// Compare two checked fractions exactly with at most one bounded alignment.
    ///
    /// Nonzero leading powers determine unequal magnitudes. Equal powers imply
    /// the aligned numerators have at most 53 bits, including unreduced fractions.
    /// Zero is handled before that calculation; no denominator is materialized.
    /// # Errors
    /// Reports an impossible checked shift/alignment contradiction as `Invariant`.
    fn compare(self, other: Self) -> Result<Ordering, FixtureEnclosureError> {
        if self.numerator == 0 || other.numerator == 0 {
            return Ok(self.numerator.cmp(&other.numerator));
        }
        let leading = i64::from(64 - self.numerator.leading_zeros()) - i64::from(self.shift);
        let other_leading =
            i64::from(64 - other.numerator.leading_zeros()) - i64::from(other.shift);
        match leading.cmp(&other_leading) {
            Ordering::Equal => {
                if self.shift < other.shift {
                    let aligned = self
                        .numerator
                        .checked_shl(other.shift - self.shift)
                        .ok_or(FixtureEnclosureError::Invariant)?;
                    Ok(aligned.cmp(&other.numerator))
                } else {
                    let aligned = other
                        .numerator
                        .checked_shl(self.shift - other.shift)
                        .ok_or(FixtureEnclosureError::Invariant)?;
                    Ok(self.numerator.cmp(&aligned))
                }
            }
            unequal => Ok(unequal),
        }
    }

    /// Form the exact complement of a checked proper fraction at least one half.
    ///
    /// Such a fraction has shift at most 53, so its denominator fits in u64.
    /// # Errors
    /// Rejects a smaller fraction or impossible denominator/subtraction invariant.
    fn complement(self) -> Result<Self, FixtureEnclosureError> {
        if self.compare(Self::new(1, 1)?)? == Ordering::Less || self.shift > 53 {
            return Err(FixtureEnclosureError::Invariant);
        }
        let denominator = 1_u64
            .checked_shl(self.shift)
            .ok_or(FixtureEnclosureError::Invariant)?;
        let numerator = denominator
            .checked_sub(self.numerator)
            .ok_or(FixtureEnclosureError::Invariant)?;
        Self::new(numerator, self.shift)
    }
}

/// Exact sign and magnitude of one admitted operand multiplied by `2^33`.
#[derive(Clone, Copy)]
struct ScaledValue {
    /// Original sign, retained even for zero; it never changes a zero endpoint.
    negative: bool,
    /// Integral magnitude no greater than `2^63`.
    whole: u64,
    /// Checked proper fractional magnitude, with no huge complemented numerator.
    fraction: Fraction,
}

impl ScaledValue {
    /// Decompose finite admitted bits and check the transformed representation.
    ///
    /// A normal exponent field E gives `significand * 2^(E-1042)`; exponent zero
    /// uses only the mantissa times `2^-1041`. Fixed-width work handles both zero
    /// signs and all subnormals without any floating-point arithmetic or effects.
    /// # Errors
    /// An exponent, shift or magnitude contradicting admitted operands is `Invariant`.
    fn from_bits(bits: u64) -> Result<Self, FixtureEnclosureError> {
        let exponent =
            u32::try_from((bits >> 52) & 0x7ff).map_err(|_| FixtureEnclosureError::Invariant)?;
        let mantissa = bits & ((1_u64 << 52) - 1);
        let (significand, scale) = if exponent == 0 {
            (mantissa, -1041_i64)
        } else {
            ((1_u64 << 52) | mantissa, i64::from(exponent) - 1042)
        };
        let (whole, fraction) = Self::split(significand, scale)?;
        if whole > MAX_SCALED_MAGNITUDE {
            return Err(FixtureEnclosureError::Invariant);
        }
        Ok(Self {
            negative: bits >> 63 != 0,
            whole,
            fraction,
        })
    }

    /// Split an IEEE significand and scale into checked whole and fraction parts.
    ///
    /// Shifts are checked before primitives; fractions never exceed 53 numerator
    /// bits. Nonnegative scales need no fraction and negative scales use no loop.
    /// # Errors
    /// Reports unsupported raw significand/scale or transformed overflow as `Invariant`.
    fn split(significand: u64, scale: i64) -> Result<(u64, Fraction), FixtureEnclosureError> {
        if significand >= (1_u64 << 53) || !(-1041..=11).contains(&scale) {
            return Err(FixtureEnclosureError::Invariant);
        }
        if scale >= 0 {
            let shift = u32::try_from(scale).map_err(|_| FixtureEnclosureError::Invariant)?;
            let whole = significand
                .checked_shl(shift)
                .ok_or(FixtureEnclosureError::Invariant)?;
            return Ok((whole, Fraction::ZERO));
        }
        let shift = u32::try_from(-scale).map_err(|_| FixtureEnclosureError::Invariant)?;
        if shift >= 53 {
            return Ok((0, Fraction::new(significand, shift)?));
        }
        let whole = significand >> shift;
        let numerator = significand & ((1_u64 << shift) - 1);
        Ok((whole, Fraction::new(numerator, shift)?))
    }

    /// Copy the exact signed integral component; u64 magnitude always fits i128.
    fn signed_whole(self) -> i128 {
        if self.negative {
            -i128::from(self.whole)
        } else {
            i128::from(self.whole)
        }
    }
}

/// Two immutable scaled operands admitted together by the mandatory garde path.
struct EnclosureOperands {
    /// Exact sign-magnitude center, including its proper fractional residual.
    center: ScaledValue,
    /// Nonnegative distance; the sign of a supplied zero remains harmless.
    distance: ScaledValue,
}

impl EnclosureOperands {
    /// Validate both raw scalar fields before exact fixed-size decomposition.
    ///
    /// Temporary garde reports can allocate for at most two fixed fields and are
    /// dropped on rejection. No raw candidate or report is published or retained.
    /// # Errors
    /// Domain/prebound failures are `Unrepresentable`; impossible checked transforms
    /// are `Invariant`. Exact endpoint representability is checked separately.
    fn new(center_seconds: f64, distance_seconds: f64) -> Result<Self, FixtureEnclosureError> {
        let raw = RawEnclosureOperands {
            center_seconds,
            distance_seconds,
        };
        raw.validate()
            .map_err(|_| FixtureEnclosureError::Unrepresentable)?;
        Ok(Self {
            center: ScaledValue::from_bits(raw.center_seconds.to_bits())?,
            distance: ScaledValue::from_bits(raw.distance_seconds.to_bits())?,
        })
    }
}

/// Integral carry and residual-presence facts for a sum of two proper fractions.
struct FractionSum {
    /// At most one integral unit is produced by two proper fractions.
    carry: bool,
    /// Whether a nonzero proper fraction remains after the optional carry.
    residual: bool,
}

/// Classify a magnitude-fraction sum without constructing its possibly huge denominator.
///
/// Two fractions below one half cannot carry. Otherwise the selected fraction
/// at least one half has an exact bounded complement; one comparison classifies
/// the carry and exactness. Constant work, fixed storage and no effects occur.
/// # Errors
/// Propagates only a checked fraction/alignment representation contradiction.
fn add_fractions(left: Fraction, right: Fraction) -> Result<FractionSum, FixtureEnclosureError> {
    let half = Fraction::new(1, 1)?;
    let chosen = if left.compare(half)? != Ordering::Less {
        Some((left, right))
    } else if right.compare(half)? != Ordering::Less {
        Some((right, left))
    } else {
        None
    };
    let Some((large, other)) = chosen else {
        return Ok(FractionSum {
            carry: false,
            residual: left.numerator != 0 || right.numerator != 0,
        });
    };
    let comparison = other.compare(large.complement()?)?;
    Ok(FractionSum {
        carry: comparison != Ordering::Less,
        residual: comparison != Ordering::Equal,
    })
}

/// Exact signed integer plus the sign of a residual strictly between -1 and 1.
struct EndpointParts {
    /// Sum of integral components and any same-sign fractional carry.
    whole: i128,
    /// Less/Equal/Greater means negative/zero/positive exact residual respectively.
    residual: Ordering,
}

impl EndpointParts {
    /// Combine exactly two checked sign-magnitude values, with the chosen radius sign.
    ///
    /// Same-sign fractions may carry once; opposite signs need only comparison.
    /// No actual residual numerator or floating-point endpoint is constructed.
    /// # Errors
    /// Reports checked integer overflow or a fraction invariant contradiction.
    fn new(
        center: ScaledValue,
        distance: ScaledValue,
        subtract_distance: bool,
    ) -> Result<Self, FixtureEnclosureError> {
        let distance_whole = if subtract_distance {
            -i128::from(distance.whole)
        } else {
            i128::from(distance.whole)
        };
        let mut whole = center
            .signed_whole()
            .checked_add(distance_whole)
            .ok_or(FixtureEnclosureError::Invariant)?;
        let residual = if center.negative == subtract_distance {
            let sum = add_fractions(center.fraction, distance.fraction)?;
            if sum.carry {
                whole = whole
                    .checked_add(if center.negative { -1 } else { 1 })
                    .ok_or(FixtureEnclosureError::Invariant)?;
            }
            if sum.residual {
                if center.negative {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            } else {
                Ordering::Equal
            }
        } else {
            let comparison = center.fraction.compare(distance.fraction)?;
            if center.negative {
                comparison.reverse()
            } else {
                comparison
            }
        };
        Ok(Self { whole, residual })
    }

    /// Complete outward floor or ceiling and check the final i64 representation.
    ///
    /// Only a negative residual lowers a floor, and only a positive residual
    /// raises a ceiling. Residual zero is the sole exactness criterion.
    /// # Errors
    /// Returns `Unrepresentable` for final range failure, or `Invariant` for i128 overflow.
    fn finish(self, lower: bool) -> Result<(i64, FixtureEndpointRounding), FixtureEnclosureError> {
        let adjustment = match (lower, self.residual) {
            (true, Ordering::Less) => -1,
            (false, Ordering::Greater) => 1,
            _ => 0,
        };
        let integer = self
            .whole
            .checked_add(adjustment)
            .ok_or(FixtureEnclosureError::Invariant)?;
        let integer = i64::try_from(integer).map_err(|_| FixtureEnclosureError::Unrepresentable)?;
        let rounding = if self.residual == Ordering::Equal {
            FixtureEndpointRounding::Exact
        } else {
            FixtureEndpointRounding::OutwardFraction
        };
        Ok((integer, rounding))
    }
}

/// Completed private endpoints, published only after both exact i64 checks.
struct EnclosureBounds {
    /// Inclusive exact outward lower lattice coordinate.
    lower: i64,
    /// Inclusive exact outward upper lattice coordinate.
    upper: i64,
    /// Exactness fact for the represented lower real endpoint.
    lower_rounding: FixtureEndpointRounding,
    /// Exactness fact for the represented upper real endpoint.
    upper_rounding: FixtureEndpointRounding,
}

/// Admit raw operands and compute their exact represented Q33 enclosure.
///
/// At most two decompositions and two endpoint combinations use fixed-size
/// storage and no loops, refits, state reads or effects. This shared raw helper
/// constructs no checked projection, history or interval.
/// # Errors
/// Rejects raw domain/final range as `Unrepresentable`, checked contradictions
/// as `Invariant`, and never returns a partial endpoint pair.
fn enclose_scalars(
    center_seconds: f64,
    distance_seconds: f64,
) -> Result<EnclosureBounds, FixtureEnclosureError> {
    let operands = EnclosureOperands::new(center_seconds, distance_seconds)?;
    let (lower, lower_rounding) =
        EndpointParts::new(operands.center, operands.distance, true)?.finish(true)?;
    let (upper, upper_rounding) =
        EndpointParts::new(operands.center, operands.distance, false)?.finish(false)?;
    if lower > upper {
        return Err(FixtureEnclosureError::Invariant);
    }
    Ok(EnclosureBounds {
        lower,
        upper,
        lower_rounding,
        upper_rounding,
    })
}

/// Enclose a checked fixture projection on the explicit Q33 integer lattice.
///
/// Computes `floor((c-r)*2^33)` and `ceil((c+r)*2^33)` from the exact real values
/// represented by the projection's binary64 center/distance, without first
/// rounding `c-r` or `c+r` in floating point. Both final endpoints must fit i64.
/// Label derivation uses only the projection's asserted source identifier.
/// The original projection is retained unchanged; the interval owner validates
/// the completed label and ordered bounds before the immutable wrapper appears.
///
/// Work and scratch are fixed; garde validation can allocate reports for two
/// raw fields and the existing interval rules. No refit, history mutation,
/// observation refresh, ambient clock read, freshness/policy decision, physical
/// agreement assertion or external effect occurs.
/// # Errors
/// Returns `Unrepresentable` for numerical domain/range failure without endpoint
/// priority, `Invariant` for an impossible checked contradiction, or the interval
/// owner's preserved `Interval` cause. No partial wrapper is published.
pub fn enclose_fixture_projection(
    projection: &FixtureAnchorProjection,
) -> Result<FixtureProjectedInterval, FixtureEnclosureError> {
    let bounds = enclose_scalars(projection.center_seconds(), projection.distance_seconds())?;
    let label = projection.context().source_id().get();
    let interval = FixtureInterval::new(label, bounds.lower, bounds.upper)
        .map_err(|source| FixtureEnclosureError::Interval { source })?;
    Ok(FixtureProjectedInterval {
        projection: *projection,
        interval,
        lower_rounding: bounds.lower_rounding,
        upper_rounding: bounds.upper_rounding,
    })
}

/// Raw guards live separately and forge no checked projection, interval or history.
#[cfg(test)]
#[path = "../tests/private/enclosure_guards.rs"]
mod enclosure_guards;

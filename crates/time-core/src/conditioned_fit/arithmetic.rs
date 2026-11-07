// SPDX-License-Identifier: GPL-2.0-only
//! Original finite point and directed closed-interval primitives for N2a.
//!
//! Point operations use separate binary64 primitives and admit finite subnormals,
//! but refuse nonzero multiplication/division rounded to zero. Intervals instead
//! widen underflow with adjacent subnormals. Their producer owns containment of
//! exact real operations; mandatory Garde construction owns finite, ordered,
//! canonical representations. No physical uncertainty, source authority, mutable
//! state, allocation on success, ambient reads or external effects are supplied.

use std::fmt;

use garde::Validate;

/// Payload-free local failure; the fit owner supplies operation, stage and index.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ArithmeticError {
    /// A raw point operand or separately rounded point result is not finite.
    NonFinite,
    /// A stored divisor or complete denominator interval lacks positive margin.
    NonPositiveDenominator,
    /// A nonzero point product or quotient was rounded to numerical zero.
    PointUnderflow,
    /// A raw interval/error representation violates its checked invariant.
    InvalidInterval,
    /// A directed endpoint or absolute error has no supported finite bound.
    ErrorRange,
}

impl fmt::Display for ArithmeticError {
    /// Format fixed causes without operands, endpoints or validator reports.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::NonFinite => "nonfinite conditioned point arithmetic",
            Self::NonPositiveDenominator => "conditioned denominator lacks positive margin",
            Self::PointUnderflow => "conditioned nonzero point underflow",
            Self::InvalidInterval => "invalid conditioned enclosure representation",
            Self::ErrorRange => "conditioned enclosure exceeds finite range",
        })
    }
}

impl std::error::Error for ArithmeticError {}

/// The complete separately rounded primitive family, without implicit FMA.
#[derive(Clone, Copy)]
enum Operation {
    /// One addition, preserving legal cancellation.
    Add,
    /// One subtraction, preserving legal cancellation.
    Subtract,
    /// One multiplication, with point-only nonzero-loss rejection.
    Multiply,
    /// One division whose stored denominator must be positive.
    Divide,
}

/// Direction of one exact-real endpoint's representable enclosure.
#[derive(Clone, Copy)]
enum Direction {
    /// Round toward negative infinity after the separately rounded primitive.
    Down,
    /// Round toward positive infinity after the separately rounded primitive.
    Up,
}

/// Canonicalize numerical zero only, without changing a nonzero finite value.
fn canonical(value: f64) -> f64 {
    if value == 0.0 { 0.0 } else { value }
}

/// Decide the maintained finite representation, including canonical positive zero.
fn valid_point(value: f64) -> bool {
    value.is_finite() && (value != 0.0 || !value.is_sign_negative())
}

/// Mandatory raw scalar admission; accepted finite subnormals are preserved.
#[derive(Validate)]
struct PointCandidate {
    /// Raw point representation, without a physical unit or authority claim.
    #[garde(custom(point_rule))]
    value: f64,
}

/// Adapt the sole scalar predicate to Garde's borrowed-field interface.
/// # Errors
/// Reject nonfinite or noncanonical representations with fixed report text.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde custom rules borrow raw fields and their unit context"
)]
fn point_rule(value: &f64, (): &()) -> garde::Result {
    if valid_point(*value) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid conditioned point"))
    }
}

/// The sole raw point producer, used for operands and transformed point results.
/// # Errors
/// Reject nonfinite values; discard bounded Garde reports without retaining data.
fn checked_point(value: f64) -> Result<f64, ArithmeticError> {
    let candidate = PointCandidate {
        value: canonical(value),
    };
    candidate
        .validate()
        .map_err(|_| ArithmeticError::NonFinite)?;
    Ok(candidate.value)
}

/// Evaluate precisely one primitive; its caller owns operand/result admission.
fn evaluate(left: f64, right: f64, operation: Operation) -> f64 {
    match operation {
        Operation::Add => left + right,
        Operation::Subtract => left - right,
        Operation::Multiply => left * right,
        Operation::Divide => left / right,
    }
}

/// Admit raw operands and the transformed output around one point primitive.
///
/// Division checks positive denominator before evaluating, even for zero numerator.
/// Every failure returns no value and preserves both copied input representations.
/// # Errors
/// Refuse nonfinite operands/results, nonpositive division and nonzero zero loss.
fn point_operation(left: f64, right: f64, operation: Operation) -> Result<f64, ArithmeticError> {
    let left = checked_point(left)?;
    let right = checked_point(right)?;
    if matches!(operation, Operation::Divide) && right <= 0.0 {
        return Err(ArithmeticError::NonPositiveDenominator);
    }
    let result = checked_point(evaluate(left, right, operation))?;
    let lost_nonzero = match operation {
        Operation::Multiply => left != 0.0 && right != 0.0 && result == 0.0,
        Operation::Divide => left != 0.0 && result == 0.0,
        Operation::Add | Operation::Subtract => false,
    };
    if lost_nonzero {
        Err(ArithmeticError::PointUnderflow)
    } else {
        Ok(result)
    }
}

/// Add finite raw point operands, admitting finite subnormals and cancellation.
/// # Errors
/// Refuse nonfinite operands or separately rounded result without any effect.
pub(super) fn point_add(left: f64, right: f64) -> Result<f64, ArithmeticError> {
    point_operation(left, right, Operation::Add)
}

/// Subtract finite raw point operands with canonical zero and no reassociation.
/// # Errors
/// Refuse nonfinite operands or separately rounded result without any effect.
pub(super) fn point_sub(left: f64, right: f64) -> Result<f64, ArithmeticError> {
    point_operation(left, right, Operation::Subtract)
}

/// Multiply finite raw point operands, retaining finite nonzero subnormal results.
/// # Errors
/// Refuse nonfinite operands/results or loss of a nonzero product to zero.
pub(super) fn point_mul(left: f64, right: f64) -> Result<f64, ArithmeticError> {
    point_operation(left, right, Operation::Multiply)
}

/// Divide finite raw points with a strictly positive stored denominator.
/// # Errors
/// Refuse nonfinite values, nonpositive denominator or nonzero quotient zero loss.
pub(super) fn point_div_positive(left: f64, right: f64) -> Result<f64, ArithmeticError> {
    point_operation(left, right, Operation::Divide)
}

/// Negate a finite raw point exactly, canonicalizing numerical zero.
/// # Errors
/// Refuse a nonfinite operand/output through mandatory scalar construction.
pub(super) fn point_neg(value: f64) -> Result<f64, ArithmeticError> {
    checked_point(-checked_point(value)?)
}

/// Decide finite canonical endpoints in closed ascending order.
fn valid_interval(endpoints: [f64; 2]) -> bool {
    let [lower, upper] = endpoints;
    valid_point(lower) && valid_point(upper) && lower <= upper
}

/// Mandatory raw interval representation, including the cross-endpoint order.
#[derive(Validate)]
struct IntervalCandidate {
    /// Lower and upper stored endpoints; one maintained rule covers both fields.
    #[garde(custom(interval_rule))]
    endpoints: [f64; 2],
}

/// Adapt finite, canonical and ordered endpoint admission to Garde.
/// # Errors
/// Reject invalid endpoint fields/order without formatting them.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde custom rules borrow the fixed raw endpoint array and context"
)]
fn interval_rule(endpoints: &[f64; 2], (): &()) -> garde::Result {
    if valid_interval(*endpoints) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid conditioned interval"))
    }
}

/// Immutable finite closed interval; only the private checked producer constructs it.
#[derive(Clone, Copy)]
pub(super) struct Interval {
    /// Canonical finite lower endpoint, no greater than the upper endpoint.
    lower: f64,
    /// Canonical finite upper endpoint, no smaller than the lower endpoint.
    upper: f64,
}

impl fmt::Debug for Interval {
    /// Reveal the domain type without endpoints or their signed bits.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Interval { .. }")
    }
}

/// Enclose one operation on finite stored endpoints, not a rounded point estimate.
///
/// Private callers supply endpoints from checked immutable intervals. Explicit
/// exact identities avoid unnecessary widening. Every other finite primitive
/// advances once in the requested direction, including zero/subnormal underflow.
/// # Errors
/// Refuse nonpositive division or an unavailable finite primitive/outward endpoint.
fn endpoint(
    left: f64,
    right: f64,
    operation: Operation,
    direction: Direction,
) -> Result<f64, ArithmeticError> {
    if matches!(operation, Operation::Divide) && right <= 0.0 {
        return Err(ArithmeticError::NonPositiveDenominator);
    }
    let exact = match operation {
        Operation::Add if left == 0.0 => Some(right),
        Operation::Add | Operation::Subtract if right == 0.0 => Some(left),
        Operation::Subtract if left.to_bits() == right.to_bits() => Some(0.0),
        Operation::Multiply if left == 0.0 || right == 0.0 => Some(0.0),
        Operation::Multiply if left.to_bits() == 1.0_f64.to_bits() => Some(right),
        Operation::Multiply | Operation::Divide if right.to_bits() == 1.0_f64.to_bits() => {
            Some(left)
        }
        Operation::Divide if left == 0.0 => Some(0.0),
        Operation::Divide if left.to_bits() == right.to_bits() => Some(1.0),
        Operation::Add | Operation::Subtract | Operation::Multiply | Operation::Divide => None,
    };
    if let Some(value) = exact {
        return Ok(canonical(value));
    }
    let rounded = evaluate(left, right, operation);
    if !rounded.is_finite() {
        return Err(ArithmeticError::ErrorRange);
    }
    let outward = match direction {
        Direction::Down => rounded.next_down(),
        Direction::Up => rounded.next_up(),
    };
    if outward.is_finite() {
        Ok(canonical(outward))
    } else {
        Err(ArithmeticError::ErrorRange)
    }
}

impl Interval {
    /// Sole raw interval producer; canonicalize zeros then mandatorily validate.
    /// # Errors
    /// Refuse nonfinite/reversed endpoints without publishing any interval.
    fn new(lower: f64, upper: f64) -> Result<Self, ArithmeticError> {
        let candidate = IntervalCandidate {
            endpoints: [canonical(lower), canonical(upper)],
        };
        candidate
            .validate()
            .map_err(|_| ArithmeticError::InvalidInterval)?;
        let [lower, upper] = candidate.endpoints;
        Ok(Self { lower, upper })
    }

    /// Admit a raw finite scalar as an exact singleton, including subnormals.
    /// # Errors
    /// Refuse nonfinite input through mandatory interval construction.
    pub(super) fn exact(value: f64) -> Result<Self, ArithmeticError> {
        Self::new(value, value)
    }

    /// Seed an exact-source coordinate from its actual rounded point and bound.
    ///
    /// The N1 producer owns the original exact-to-rounded relationship. A zero
    /// bound is an exact singleton; positive bounds are evaluated outward.
    /// # Errors
    /// Refuse nonfinite points/bounds, negative bounds or finite-range exhaustion.
    pub(super) fn from_error(point: f64, error: f64) -> Result<Self, ArithmeticError> {
        let point = checked_point(point)?;
        let error = checked_point(error)?;
        if error < 0.0 {
            return Err(ArithmeticError::InvalidInterval);
        }
        let point = Self::exact(point)?;
        let error = Self::exact(error)?;
        Self::new(point.sub(error)?.lower, point.add(error)?.upper)
    }

    /// Return the admitted finite lower endpoint without changing ownership.
    pub(super) const fn lower(self) -> f64 {
        self.lower
    }

    /// Return the admitted finite upper endpoint without changing ownership.
    pub(super) const fn upper(self) -> f64 {
        self.upper
    }

    /// Enclose the sum of all values from two immutable checked intervals.
    /// # Errors
    /// Refuse finite-range exhaustion; neither operand is modified.
    pub(super) fn add(self, right: Self) -> Result<Self, ArithmeticError> {
        Self::new(
            endpoint(self.lower, right.lower, Operation::Add, Direction::Down)?,
            endpoint(self.upper, right.upper, Operation::Add, Direction::Up)?,
        )
    }

    /// Enclose pairwise subtraction; a zero left interval uses exact negation.
    /// # Errors
    /// Refuse finite-range exhaustion; neither operand is modified.
    pub(super) fn sub(self, right: Self) -> Result<Self, ArithmeticError> {
        if self.lower == 0.0 && self.upper == 0.0 {
            return right.neg();
        }
        Self::new(
            endpoint(
                self.lower,
                right.upper,
                Operation::Subtract,
                Direction::Down,
            )?,
            endpoint(self.upper, right.lower, Operation::Subtract, Direction::Up)?,
        )
    }

    /// Enclose all four endpoint products; exact sign facts can strengthen zero.
    /// # Errors
    /// Refuse a nonfinite/exhausted endpoint; preserve both immutable operands.
    pub(super) fn mul(self, right: Self) -> Result<Self, ArithmeticError> {
        let mut result = self.four_endpoints(right, Operation::Multiply)?;
        if (self.lower >= 0.0 && right.lower >= 0.0) || (self.upper <= 0.0 && right.upper <= 0.0) {
            // Every exact product is nonnegative, independently of rounded signs.
            result = Self::new(result.lower.max(0.0), result.upper)?;
        }
        Ok(result)
    }

    /// Enclose all four endpoint quotients with strict denominator lower margin.
    /// # Errors
    /// Refuse nonpositive denominator lower endpoint or finite-range exhaustion.
    pub(super) fn div_positive(self, right: Self) -> Result<Self, ArithmeticError> {
        if right.lower <= 0.0 {
            return Err(ArithmeticError::NonPositiveDenominator);
        }
        let result = self.four_endpoints(right, Operation::Divide)?;
        if self.lower >= 0.0 {
            // A nonnegative numerator divided by a positive interval stays so.
            Self::new(result.lower.max(0.0), result.upper)
        } else {
            Ok(result)
        }
    }

    /// Bound the selected four endpoint combinations with fixed scalar scratch.
    ///
    /// Both operands carry immutable finite/order admission. The only division
    /// caller establishes strict positive lower margin before entering this helper.
    /// # Errors
    /// Refuse any unsupported endpoint without returning a partial interval.
    fn four_endpoints(self, right: Self, operation: Operation) -> Result<Self, ArithmeticError> {
        let mut lower = f64::MAX;
        let mut upper = -f64::MAX;
        for left in [self.lower, self.upper] {
            for right in [right.lower, right.upper] {
                lower = lower.min(endpoint(left, right, operation, Direction::Down)?);
                upper = upper.max(endpoint(left, right, operation, Direction::Up)?);
            }
        }
        Self::new(lower, upper)
    }

    /// Enclose the exact square, retaining lower zero whenever the input crosses it.
    ///
    /// This uses the mathematical nonnegative range, including when an endpoint
    /// square underflows. It never clamps a computed point estimate.
    /// # Errors
    /// Refuse endpoint-square overflow or exhausted finite outward bounds.
    pub(super) fn square(self) -> Result<Self, ArithmeticError> {
        let lower = if self.lower <= 0.0 && self.upper >= 0.0 {
            0.0
        } else {
            endpoint(self.lower, self.lower, Operation::Multiply, Direction::Down)?
                .min(endpoint(
                    self.upper,
                    self.upper,
                    Operation::Multiply,
                    Direction::Down,
                )?)
                .max(0.0)
        };
        let upper = endpoint(self.lower, self.lower, Operation::Multiply, Direction::Up)?.max(
            endpoint(self.upper, self.upper, Operation::Multiply, Direction::Up)?,
        );
        Self::new(lower, upper)
    }

    /// Exactly negate checked endpoints and reverse their order, preserving zero.
    /// # Errors
    /// Preserve mandatory transformed scalar/interval validation failure.
    pub(super) fn neg(self) -> Result<Self, ArithmeticError> {
        Self::new(point_neg(self.upper)?, point_neg(self.lower)?)
    }
}

/// Decide a final hull's representation and a necessary endpoint-distance check.
///
/// The producing directed arithmetic owns the exact error guarantee; this rule
/// rejects malformed fields, excluded points and visibly insufficient claimed
/// bounds. It does not certify an arbitrary caller's mathematical target.
fn valid_scalar(values: [f64; 4]) -> bool {
    let [point, lower, upper, error] = values;
    valid_point(point)
        && valid_interval([lower, upper])
        && valid_point(error)
        && error >= 0.0
        && lower <= point
        && point <= upper
        && error >= point - lower
        && error >= upper - point
}

/// Mandatory final scalar admission, covering every raw field and their relation.
#[derive(Clone, Copy, Validate)]
struct ScalarCandidate {
    /// Point, lower hull endpoint, upper hull endpoint and absolute error.
    #[garde(custom(scalar_rule))]
    values: [f64; 4],
}

/// Adapt final representation/containment rules to mandatory Garde construction.
/// # Errors
/// Reject malformed or insufficient fields with bounded payload-free text.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde custom rules borrow the fixed raw scalar array and context"
)]
fn scalar_rule(values: &[f64; 4], (): &()) -> garde::Result {
    if valid_scalar(*values) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid conditioned scalar enclosure"))
    }
}

/// One computed point and a finite hull enclosing it and its exact-real target.
#[derive(Clone, Copy)]
pub(super) struct ScalarEnclosure {
    /// Canonical finite computed point; never replaced or clamped by the bounds.
    value: f64,
    /// Lower closed hull endpoint, no greater than the point.
    lower: f64,
    /// Upper closed hull endpoint, no smaller than the point.
    upper: f64,
    /// Outward maximum distance from the computed point to either hull endpoint.
    absolute_error: f64,
}

impl fmt::Debug for ScalarEnclosure {
    /// Reveal only the scalar owner, withholding all points, endpoints and bounds.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ScalarEnclosure { .. }")
    }
}

/// Sole final raw publisher; private fault controls exercise this actual guard.
/// # Errors
/// Refuse invalid final representations without exposing a partially checked value.
fn publish_scalar(candidate: ScalarCandidate) -> Result<ScalarEnclosure, ArithmeticError> {
    candidate
        .validate()
        .map_err(|_| ArithmeticError::InvalidInterval)?;
    let [value, lower, upper, absolute_error] = candidate.values;
    Ok(ScalarEnclosure {
        value,
        lower,
        upper,
        absolute_error,
    })
}

impl ScalarEnclosure {
    /// Hull a raw computed point with its independently propagated target interval.
    ///
    /// The caller owns the target's actual N1/fit association. Directed endpoint
    /// subtraction bounds both distances, including cancellation and underflow.
    /// No caller-supplied error certificate or mutable attachment is accepted.
    /// # Errors
    /// Refuse nonfinite point, error-range exhaustion or mandatory output failure.
    pub(super) fn new(point: f64, target: Interval) -> Result<Self, ArithmeticError> {
        let point = checked_point(point)?;
        let lower = target.lower().min(point);
        let upper = target.upper().max(point);
        let left = endpoint(point, lower, Operation::Subtract, Direction::Up)?;
        let right = endpoint(upper, point, Operation::Subtract, Direction::Up)?;
        publish_scalar(ScalarCandidate {
            values: [point, lower, upper, left.max(right)],
        })
    }

    /// Return the unchanged computed point, retaining no authority beyond arithmetic.
    pub(super) const fn value(self) -> f64 {
        self.value
    }

    /// Return the finite hull's closed lower endpoint.
    pub(super) const fn lower(self) -> f64 {
        self.lower
    }

    /// Return the finite hull's closed upper endpoint.
    pub(super) const fn upper(self) -> f64 {
        self.upper
    }

    /// Return the outward absolute arithmetic-error bound in the caller's units.
    pub(super) const fn absolute_error(self) -> f64 {
        self.absolute_error
    }
}

/// Private representation and primitive controls, distinct from whole-fit oracles.
#[cfg(test)]
#[path = "../../tests/private/conditioned_arithmetic.rs"]
mod tests;

// SPDX-License-Identifier: GPL-2.0-only
//! Checked companions for separately rounded combination primitives.
//!
//! A companion admits one finite normal-or-zero point and a finite nonnegative
//! absolute error, including subnormal errors. The represented exact real lies
//! within that error of the saved point; only its producer can establish that
//! semantic relationship. Constructors validate representation, not the producer's
//! mathematical proof. Each transformation preserves a proved input relationship
//! with outward evaluation of every nonnegative bound primitive. No mutation,
//! logging, ambient reads, participant choice or lifecycle commit occurs here.

use std::fmt;

use garde::Validate;

use crate::regression::scalar_issue;
use crate::{ArithmeticIssue, ArithmeticOperation, FixtureChartBoundOperation, FixtureChartError};

use super::super::{FixtureCombinationError, FixtureCombinationStep, binary};

/// Exact binary64 unit roundoff, `2^-53`.
const UNIT_ROUNDOFF: f64 = f64::from_bits(0x3ca0_0000_0000_0000);
/// Existing conservative upper for positive underflow; never a point clamp.
const ETA_UPPER: f64 = f64::MIN_POSITIVE;

/// Closed representation, point-operation or proof-envelope failure.
#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum CompanionError {
    /// A proposed point is outside the shared normal-or-zero support envelope.
    InvalidPoint {
        /// Original scalar classification, without the rejected numeric value.
        issue: ArithmeticIssue,
    },
    /// A proposed absolute error is negative or nonfinite.
    InvalidError,
    /// Mandatory Garde validation disagreed with the shared raw predicates.
    ValidationInvariant,
    /// A strictly positive lower bound for the exact denominator is unavailable.
    UnprovedPositiveMargin,
    /// Preserve the actual separately rounded combination primitive failure.
    Value {
        /// Existing step and arithmetic cause; contains no proposed companion.
        source: FixtureCombinationError,
    },
    /// Preserve the actual nonnegative outward primitive failure.
    Bound {
        /// Existing operation and finite-envelope cause, without scalar payloads.
        source: FixtureChartError,
    },
}

impl fmt::Debug for CompanionError {
    /// Keep diagnostics bounded and independent of supplied numeric values.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("CompanionError { .. }")
    }
}

impl fmt::Display for CompanionError {
    /// Display one fixed error family without formatting nested numerical input.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidPoint { .. } => "combination companion: unsupported point",
            Self::InvalidError => "combination companion: unsupported error bound",
            Self::ValidationInvariant => "combination companion: validation disagreement",
            Self::UnprovedPositiveMargin => "combination companion: unproved denominator margin",
            Self::Value { .. } => "combination companion: point arithmetic rejected",
            Self::Bound { .. } => "combination companion: outward arithmetic rejected",
        })
    }
}

impl std::error::Error for CompanionError {
    /// Retain the real checked arithmetic cause; representation failures have none.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Value { source } => Some(source),
            Self::Bound { source } => Some(source),
            _ => None,
        }
    }
}

/// Admit every raw binary64 point to the shared support predicate without effects.
/// # Errors
/// Reject a nonfinite or nonzero subnormal value with one bounded static message.
fn point_rule(value: f64) -> garde::Result {
    if scalar_issue(value).is_none() {
        Ok(())
    } else {
        Err(garde::Error::new("unsupported combination point"))
    }
}

/// Recognize finite nonnegative absolute errors, including both zeros/subnormals.
fn error_valid(value: f64) -> bool {
    value.is_finite() && value >= 0.0
}

/// Bound raw-error validation messages without exposing the rejected scalar.
/// # Errors
/// Reject a nonfinite or negative error with one bounded static message.
fn error_rule(value: f64) -> garde::Result {
    if error_valid(value) {
        Ok(())
    } else {
        Err(garde::Error::new("unsupported combination error bound"))
    }
}

/// Mandatory raw-field validation before immutable private construction.
#[derive(Validate)]
struct RawCompanion {
    /// Normal-or-zero saved value, preserving the original signed-zero bits.
    #[garde(custom(|value: &f64, (): &()| point_rule(*value)))]
    value: f64,
    /// Finite nonnegative error; no upward normal clamp is applied on admission.
    #[garde(custom(|value: &f64, (): &()| error_rule(*value)))]
    error: f64,
}

impl RawCompanion {
    /// Project shared predicate failures in point-then-error order, without data.
    fn first_error(&self) -> Option<CompanionError> {
        if let Some(issue) = scalar_issue(self.value) {
            Some(CompanionError::InvalidPoint { issue })
        } else if !error_valid(self.error) {
            Some(CompanionError::InvalidError)
        } else {
            None
        }
    }
}

/// Immutable representation of one rounded point and a proved absolute error.
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct Companion {
    /// Supported rounded point; mathematical meaning/units belong to its caller.
    value: f64,
    /// Finite nonnegative absolute error in the same units as the represented point.
    error: f64,
}

impl fmt::Debug for Companion {
    /// Expose only the type, retaining no numeric or signed-zero diagnostic value.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Companion { .. }")
    }
}

impl Companion {
    /// Mandatorily validate representation before publishing a private companion.
    ///
    /// The caller owns proof that its exact reference is within `error` of
    /// `value`. All raw binary64 representations are accepted for validation;
    /// failure returns no companion and has no external or state effect.
    /// # Errors
    /// Reject unsupported points, invalid errors, then unexplained Garde failure.
    pub(crate) fn new(value: f64, error: f64) -> Result<Self, CompanionError> {
        let candidate = RawCompanion { value, error };
        let validation = candidate.validate();
        if let Some(error) = candidate.first_error() {
            return Err(error);
        }
        validation.map_err(|_| CompanionError::ValidationInvariant)?;
        Ok(Self { value, error })
    }

    /// Admit exact stored input bits with zero error and no arithmetic/effects.
    /// # Errors
    /// Preserve the mandatory constructor's supported-point rejection.
    pub(crate) fn exact(value: f64) -> Result<Self, CompanionError> {
        Self::new(value, 0.0)
    }

    /// Return the immutable rounded value without changing bits or authority.
    #[must_use]
    pub(crate) const fn value(self) -> f64 {
        self.value
    }

    /// Return the separate absolute error without claiming statistical uncertainty.
    #[must_use]
    pub(crate) const fn error(self) -> f64 {
        self.error
    }

    /// Add separately rounded points and propagate both errors strictly outward.
    /// # Errors
    /// Preserve actual point/upper failures; return no partially checked value.
    pub(crate) fn add(
        self,
        other: Self,
        step: FixtureCombinationStep,
    ) -> Result<Self, CompanionError> {
        let value = point(self.value, other.value, ArithmeticOperation::Add, step)?;
        Self::new(value, addition_error(self, other)?)
    }

    /// Subtract separately rounded points with the same absolute-error contract.
    /// # Errors
    /// Preserve actual point/upper failures without modifying either input.
    #[cfg(test)]
    pub(crate) fn subtract(
        self,
        other: Self,
        step: FixtureCombinationStep,
    ) -> Result<Self, CompanionError> {
        let value = point(self.value, other.value, ArithmeticOperation::Subtract, step)?;
        Self::new(value, addition_error(self, other)?)
    }

    /// Multiply separately rounded points, including the product of input errors.
    /// # Errors
    /// Reject point/upper overflow or unsupported point underflow; no input changes.
    pub(crate) fn multiply(
        self,
        other: Self,
        step: FixtureCombinationStep,
    ) -> Result<Self, CompanionError> {
        let value = point(self.value, other.value, ArithmeticOperation::Multiply, step)?;
        let left = multiply_up(self.value.abs(), other.error)?;
        let right = multiply_up(other.value.abs(), self.error)?;
        let cross = multiply_up(self.error, other.error)?;
        let magnitude = multiply_up(self.value.abs(), other.value.abs())?;
        let rounding = add_up(multiply_up(UNIT_ROUNDOFF, magnitude)?, ETA_UPPER)?;
        let error = add_up(add_up(add_up(left, right)?, cross)?, rounding)?;
        Self::new(value, error)
    }

    /// Divide by a denominator with a proved strictly positive exact margin.
    ///
    /// With `L <= b-Eb` and `Q >= abs(a)/b`, every primitive in
    /// `(Ea + Q*Eb)/L + u*Q + eta` evaluates outward. Point division is the
    /// original separately rounded primitive; no reciprocal reassociation occurs.
    /// # Errors
    /// Reject unavailable margin first, then point/upper failures. Inputs remain
    /// unchanged, and a bound failure never publishes an unbounded point.
    pub(crate) fn divide_positive(
        self,
        denominator: Self,
        step: FixtureCombinationStep,
    ) -> Result<Self, CompanionError> {
        let lower = lower_margin(denominator.value, denominator.error)?;
        let value = point(
            self.value,
            denominator.value,
            ArithmeticOperation::Divide,
            step,
        )?;
        let quotient = divide_up(self.value.abs(), denominator.value)?;
        let numerator = add_up(self.error, multiply_up(quotient, denominator.error)?)?;
        let propagated = divide_up(numerator, lower)?;
        let rounding = add_up(multiply_up(UNIT_ROUNDOFF, quotient)?, ETA_UPPER)?;
        Self::new(value, add_up(propagated, rounding)?)
    }
}

/// Execute one actual supported point operation, preserving its original step.
/// # Errors
/// Preserve the existing combination primitive's exact typed arithmetic cause.
fn point(
    left: f64,
    right: f64,
    operation: ArithmeticOperation,
    step: FixtureCombinationStep,
) -> Result<f64, CompanionError> {
    binary(left, right, operation, step).map_err(|source| CompanionError::Value { source })
}

/// Evaluate one bound through the unchanged shared nonnegative primitive owner.
/// # Errors
/// Preserve the actual invalid-operand/divisor/exhausted-upper cause, without data.
fn upper(
    operation: FixtureChartBoundOperation,
    left: f64,
    right: f64,
) -> Result<f64, CompanionError> {
    operation
        .evaluate_up(left, right)
        .map_err(|source| CompanionError::Bound { source })
}

/// Add finite nonnegative stored bounds outward, admitting subnormal operands.
/// # Errors
/// Preserve shared outward Add failures; no value is clamped or mutated.
fn add_up(left: f64, right: f64) -> Result<f64, CompanionError> {
    upper(FixtureChartBoundOperation::Add, left, right)
}

/// Multiply finite nonnegative stored bounds outward with the shared envelope.
/// # Errors
/// Preserve shared outward Multiply failures without any partial result.
fn multiply_up(left: f64, right: f64) -> Result<f64, CompanionError> {
    upper(FixtureChartBoundOperation::Multiply, left, right)
}

/// Divide a nonnegative bound by a finite positive exact/lower denominator.
/// # Errors
/// Preserve shared outward Divide failures, including exhausted finite successors.
fn divide_up(left: f64, right: f64) -> Result<f64, CompanionError> {
    upper(FixtureChartBoundOperation::Divide, left, right)
}

/// Bound one addition/subtraction from two immutable admitted companions.
/// # Errors
/// Reject the first exhausted outward primitive; no point or input changes.
fn addition_error(left: Companion, right: Companion) -> Result<f64, CompanionError> {
    let magnitude = add_up(left.value.abs(), right.value.abs())?;
    let rounding = add_up(multiply_up(UNIT_ROUNDOFF, magnitude)?, ETA_UPPER)?;
    add_up(add_up(left.error, right.error)?, rounding)
}

/// Produce finite `0 < L <= b-Eb` without ever rounding a lower bound upward.
///
/// All finite positive `b` and finite nonnegative `Eb`, including subnormals,
/// reach the explicit `b > Eb` guard. Their separate binary64 subtraction has
/// an exact result whenever its positive rounded result is subnormal. Otherwise
/// one downward successor is conservative under round-to-nearest. A flush to
/// zero or unsupported result rejects; no upward normal clamp is used.
/// # Errors
/// Reject invalid operands, nonpositive exact margin or unavailable positive
/// finite predecessor. The helper uses scalar scratch and changes no state.
fn lower_margin(value: f64, error: f64) -> Result<f64, CompanionError> {
    if !value.is_finite() || value <= 0.0 || !error_valid(error) || value <= error {
        return Err(CompanionError::UnprovedPositiveMargin);
    }
    let rounded = value - error;
    if !rounded.is_finite() || rounded <= 0.0 {
        return Err(CompanionError::UnprovedPositiveMargin);
    }
    if rounded.is_subnormal() {
        return Ok(rounded);
    }
    let lower = rounded.next_down();
    if lower.is_finite() && lower > 0.0 {
        Ok(lower)
    } else {
        Err(CompanionError::UnprovedPositiveMargin)
    }
}

/// Independent exact-bit and interval controls, with no fabricated product state.
#[cfg(test)]
#[path = "../../../tests/private/chart_combination_roundoff.rs"]
mod tests;

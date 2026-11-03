// SPDX-License-Identifier: GPL-2.0-only
//! Checked affine reexpression and separate outward arithmetic-error bounds.
//!
//! Authored by David Osipov with AI assistance, 3 November 2026. The independent
//! S154 `Fraction` reference owns the frozen equations; this module introduces no
//! fitted-statistic, achieved-effect or physical-clock uncertainty claim.
//!
//! Coordinate primitives reuse the existing normal-or-zero arithmetic owner.
//! Only the nonnegative bound primitives widen positive underflow; coordinate
//! values are never clamped. All work uses scalar scratch and fixed four-value
//! sums, without allocation, mutation, logging or ambient reads.

use time_domain::MonotonicNs;

use super::super::normalization::integer_to_f64;
use super::super::projection::FixtureAnchorProjection;
use super::super::{ArithmeticStage, NormalizationComponent, arithmetic};
use super::{
    ChartPhase, ChartScalars, FixtureAffineChart, FixtureChartBoundIssue,
    FixtureChartBoundOperation, FixtureChartError, FixtureChartStep,
};

/// Exact binary64 unit roundoff, `2^-53`, without a rounded `1 + u` constant.
const UNIT_ROUNDOFF: f64 = f64::from_bits(0x3ca0_0000_0000_0000);
/// Representable upper bound of the real half-subnormal `eta = 2^-1075`.
const ETA_UPPER: f64 = f64::MIN_POSITIVE;
/// Exact positive integer denominator converting nanoseconds into seconds.
const NANOSECONDS_PER_SECOND: f64 = 1_000_000_000.0;

/// One exact integer magnitude and its separately rounded signed seconds.
struct Elapsed {
    /// Exact `abs(query - reference)` after the inclusive `2^53` integer guard.
    nanoseconds: f64,
    /// `RN((query - reference) / 10^9)`, with numerical zero retained as `+0`.
    seconds: f64,
}

/// Absolute magnitude and arithmetic-error bounds for one shared displacement.
struct DisplacementBounds {
    /// Upper bound of the exact real magnitude `abs(D + F*q)`.
    magnitude: f64,
    /// Upper bound of `abs(saved_delta - (D + F*q))` for the stored input bits.
    error: f64,
}

/// Project one closed coordinate primitive through its maintained operation map.
///
/// Every raw operand pair is checked by the existing arithmetic owner. A failure
/// retains its actual response-stage cause and the calling chart step; no partial
/// value or state change escapes.
/// # Errors
/// Reject unsupported operands/results, nonpositive division or zero underflow.
fn value_arithmetic(
    left: f64,
    right: f64,
    step: FixtureChartStep,
) -> Result<f64, FixtureChartError> {
    arithmetic(left, right, step.operation(), ArithmeticStage::Response)
        .map_err(|source| FixtureChartError::Arithmetic { step, source })
}

/// Convert an exact signed timestamp difference without converting an absolute time.
///
/// All typed instants are accepted inputs. Unsigned `abs_diff` cannot overflow;
/// the shared integer owner independently rejects magnitudes above `2^53`.
/// Division is one separate checked primitive, followed only by exact negation.
/// Fixed scalar work preserves both input instants and reads no clock.
/// # Errors
/// Preserve exact integer-conversion or response-stage seconds-division causes.
fn elapsed(query: MonotonicNs, reference: MonotonicNs) -> Result<Elapsed, FixtureChartError> {
    let magnitude = query.nanoseconds().abs_diff(reference.nanoseconds());
    let nanoseconds = integer_to_f64(magnitude, NormalizationComponent::Time)
        .map_err(|source| FixtureChartError::Normalization { source })?;
    let positive = value_arithmetic(
        nanoseconds,
        NANOSECONDS_PER_SECOND,
        FixtureChartStep::SecondsDivide,
    )?;
    let seconds = if query < reference {
        -positive
    } else {
        positive
    };
    Ok(Elapsed {
        nanoseconds,
        seconds,
    })
}

/// Evaluate one nonnegative stored-operand bound operation strictly outward.
///
/// Unlike coordinate arithmetic, this helper admits finite subnormal operands.
/// Exact-zero operand cases return `+0`; any positive zero/subnormal result
/// widens to `MIN_POSITIVE`. A positive normal result advances by one binary64
/// value. Division bounds the quotient of these stored operands; recursive use
/// requires an exact denominator or a suitable positive lower bound.
/// No coordinate value, input or external state is changed.
/// # Errors
/// Reject nonfinite/negative operands first, nonpositive divisors second, then
/// nonfinite results or a result without a finite outward successor.
fn bound_arithmetic(
    left: f64,
    right: f64,
    operation: FixtureChartBoundOperation,
) -> Result<f64, FixtureChartError> {
    let failure = |issue| FixtureChartError::Bound { operation, issue };
    if !left.is_finite() || !right.is_finite() || left < 0.0 || right < 0.0 {
        return Err(failure(FixtureChartBoundIssue::InvalidOperand));
    }
    if operation == FixtureChartBoundOperation::Divide && right <= 0.0 {
        return Err(failure(FixtureChartBoundIssue::NonPositiveDivisor));
    }
    let exact_zero = match operation {
        FixtureChartBoundOperation::Add => left == 0.0 && right == 0.0,
        FixtureChartBoundOperation::Multiply => left == 0.0 || right == 0.0,
        FixtureChartBoundOperation::Divide => left == 0.0,
    };
    if exact_zero {
        return Ok(0.0);
    }
    let rounded = match operation {
        FixtureChartBoundOperation::Add => left + right,
        FixtureChartBoundOperation::Multiply => left * right,
        FixtureChartBoundOperation::Divide => left / right,
    };
    if !rounded.is_finite() {
        return Err(failure(FixtureChartBoundIssue::NonFiniteResult));
    }
    if rounded < f64::MIN_POSITIVE {
        return Ok(f64::MIN_POSITIVE);
    }
    let upper = rounded.next_up();
    if !upper.is_finite() {
        return Err(failure(FixtureChartBoundIssue::NonFiniteResult));
    }
    Ok(upper)
}

/// Add two raw finite nonnegative bound operands strictly outward.
///
/// This is the parent enclosure's sole radius-plus-arithmetic-error seam. Exact
/// zero returns `+0`; positive underflow widens only this bound. No point value,
/// statistical SD or input is changed, and work is constant.
/// # Errors
/// Preserve the bound owner's operand or finite-successor rejection for Add.
pub(super) fn add_up(left: f64, right: f64) -> Result<f64, FixtureChartError> {
    bound_arithmetic(left, right, FixtureChartBoundOperation::Add)
}

/// Multiply two nonnegative bound operands with the same outward/error contract.
/// # Errors
/// Preserve the bound owner's exact Multiply cause without a partial result.
fn multiply_up(left: f64, right: f64) -> Result<f64, FixtureChartError> {
    bound_arithmetic(left, right, FixtureChartBoundOperation::Multiply)
}

/// Divide stored nonnegative bound operands by a finite positive divisor.
/// # Errors
/// Preserve the bound owner's exact Divide cause without a partial result.
fn divide_up(left: f64, right: f64) -> Result<f64, FixtureChartError> {
    bound_arithmetic(left, right, FixtureChartBoundOperation::Divide)
}

/// Sum two nonnegative bounds using the reference's outward `0 + a + b` order.
/// # Errors
/// Preserve the first unsupported bound addition; no input changes.
fn sum_two_up(left: f64, right: f64) -> Result<f64, FixtureChartError> {
    let first = add_up(0.0, left)?;
    add_up(first, right)
}

/// Sum exactly four bounds in reference order, checking every intermediate.
/// # Errors
/// Preserve the first unsupported bound addition; no input changes.
fn sum_four_up(first: f64, second: f64, third: f64, fourth: f64) -> Result<f64, FixtureChartError> {
    let mut total = 0.0;
    for value in [first, second, third, fourth] {
        total = add_up(total, value)?;
    }
    Ok(total)
}

/// Bound the one saved affine displacement relative to exact stored coefficients.
///
/// `elapsed` comes only from the exact integer-conversion owner. The divisor
/// `10^9` is exact, so its outward quotient bounds `abs(q)`. Every subsequent
/// quantity is nonnegative and uses an outward operation. `E + u*E` replaces
/// the unrepresentable constant `1 + u`; eta's upper is always included.
/// # Errors
/// Reject the first invalid/overflowing bound operation, including a finite MAX
/// result whose next representable upper would be infinite. Fixed work has no
/// input, history, fit or coordinate effects.
fn displacement_bounds(
    elapsed: &Elapsed,
    phase: f64,
    rate: f64,
) -> Result<DisplacementBounds, FixtureChartError> {
    let q_magnitude = divide_up(elapsed.nanoseconds, NANOSECONDS_PER_SECOND)?;
    let product_magnitude = multiply_up(rate.abs(), q_magnitude)?;
    let magnitude = sum_two_up(phase.abs(), product_magnitude)?;
    let time_error = sum_two_up(multiply_up(UNIT_ROUNDOFF, q_magnitude)?, ETA_UPPER)?;
    let propagated_time_error = multiply_up(rate.abs(), time_error)?;
    let product_error = sum_four_up(
        propagated_time_error,
        multiply_up(UNIT_ROUNDOFF, propagated_time_error)?,
        multiply_up(UNIT_ROUNDOFF, product_magnitude)?,
        ETA_UPPER,
    )?;
    let error = sum_four_up(
        product_error,
        multiply_up(UNIT_ROUNDOFF, product_error)?,
        multiply_up(UNIT_ROUNDOFF, magnitude)?,
        ETA_UPPER,
    )?;
    Ok(DisplacementBounds { magnitude, error })
}

/// Bound one center subtraction using its original stored value and shared delta.
///
/// Triangle magnitudes conservatively replace exact cancellation. The result
/// covers new arithmetic only, excluding old-fit, composition and physical error.
/// # Errors
/// Preserve the first failed outward operation without publishing a center.
fn phase_error(original: f64, delta: &DisplacementBounds) -> Result<f64, FixtureChartError> {
    let magnitude = sum_two_up(original.abs(), delta.magnitude)?;
    sum_four_up(
        delta.error,
        multiply_up(UNIT_ROUNDOFF, delta.error)?,
        multiply_up(UNIT_ROUNDOFF, magnitude)?,
        ETA_UPPER,
    )
}

/// Bound rate subtraction independently of the saved phase displacement.
/// # Errors
/// Preserve the first failed outward operation without publishing a rate.
fn rate_error(original: f64, adjustment: f64) -> Result<f64, FixtureChartError> {
    let magnitude = sum_two_up(original.abs(), adjustment.abs())?;
    sum_two_up(multiply_up(UNIT_ROUNDOFF, magnitude)?, ETA_UPPER)
}

/// Reexpress one genuine complete original projection with separate error bounds.
///
/// The parent borrowed-view owner binds source context, full retained history and
/// descriptor provenance before this purely numerical helper. This helper still
/// rejects a query outside the descriptor domain. One saved delta is reused for
/// the original anchor center and optional original fitted phase; all coordinate
/// primitives precede bound construction. No old statistic, uncertainty, instant,
/// history, metadata or lifecycle state is changed. Only complete scalar scratch
/// escapes, for the parent's mandatory final mapped-value admission.
/// # Errors
/// Preserve query-domain, integer conversion, exact coordinate-step arithmetic,
/// then outward-bound failures in that order; no partial projection is published.
pub(super) fn transform(
    chart: &FixtureAffineChart,
    original: &FixtureAnchorProjection,
) -> Result<ChartScalars, FixtureChartError> {
    let query = original.projected_at();
    if !chart.domain.contains(query) {
        return Err(FixtureChartError::QueryOutsideDomain);
    }
    let elapsed = elapsed(query, chart.domain.reference)?;
    let product = value_arithmetic(
        chart.rate,
        elapsed.seconds,
        FixtureChartStep::RateTimesElapsed,
    )?;
    let delta = value_arithmetic(chart.phase, product, FixtureChartStep::Displacement)?;
    let original_center = original.center_seconds();
    let original_rate = original.phase_rate_seconds_per_second();
    let center = value_arithmetic(original_center, delta, FixtureChartStep::AnchorCenter)?;
    let tracking_values = if let Some(tracking) = original.tracking() {
        let source = tracking.phase_seconds();
        let mapped = value_arithmetic(source, delta, FixtureChartStep::TrackingPhase)?;
        Some((source, mapped))
    } else {
        None
    };
    let rate = value_arithmetic(original_rate, chart.rate, FixtureChartStep::Rate)?;
    let delta_bounds = displacement_bounds(&elapsed, chart.phase, chart.rate)?;
    let center_error = phase_error(original_center, &delta_bounds)?;
    let tracking = if let Some((source, value)) = tracking_values {
        Some(ChartPhase {
            value,
            error: phase_error(source, &delta_bounds)?,
        })
    } else {
        None
    };
    let rate_error = rate_error(original_rate, chart.rate)?;
    Ok(ChartScalars {
        center,
        rate,
        center_error,
        rate_error,
        tracking,
    })
}

/// Raw private arithmetic controls retain their original module identity.
#[cfg(test)]
#[path = "../../../tests/private/chart_roundoff.rs"]
mod tests;

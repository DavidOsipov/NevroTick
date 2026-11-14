// SPDX-License-Identifier: GPL-2.0-only
//! Private S154 arithmetic controls; no checked history or chart is fabricated.

use std::num::NonZeroU64;

use time_domain::{ActiveAwakeDurationNs, ActiveAwakeNs, MonotonicNs};

use super::super::super::{ArithmeticIssue, ArithmeticOperation, FitError, NormalizationError};
use super::super::{
    FixtureAffineChart, FixtureChartDomain, FixtureChartField, FixtureChartFrames,
    FixtureChartIssue,
};
use super::{
    ArithmeticStage, FixtureChartBoundIssue, FixtureChartBoundOperation, FixtureChartError,
    FixtureChartStep, NormalizationComponent, UNIT_ROUNDOFF, bound_arithmetic, elapsed,
    value_arithmetic,
};

/// A private arithmetic check returns its existing typed failure directly.
type TestResult = Result<(), FixtureChartError>;

/// Exact zeros and positive underflow are separate, independently fixed cases.
/// # Errors
/// Preserve any unexpected upper-operation rejection beside legal neighbors.
/// # Panics
/// Reject incorrect zero signs, silent positive-zero loss or a missing next-up.
#[test]
fn upper_operators_preserve_zero_and_underflow_neighbors() -> TestResult {
    for (operation, left, right) in [
        (FixtureChartBoundOperation::Add, -0.0, 0.0),
        (FixtureChartBoundOperation::Multiply, 0.0, f64::MAX),
        (FixtureChartBoundOperation::Divide, -0.0, 1.0),
    ] {
        assert_eq!(bound_arithmetic(left, right, operation)?.to_bits(), 0);
    }
    let smallest = f64::from_bits(1);
    let largest_subnormal = f64::from_bits(0x000f_ffff_ffff_ffff);
    let next_minimum = f64::from_bits(0x0010_0000_0000_0001);
    let below_one = f64::from_bits(0x3fef_ffff_ffff_ffff);
    for (operation, left, right, expected) in [
        (
            FixtureChartBoundOperation::Multiply,
            smallest,
            0.5,
            f64::MIN_POSITIVE,
        ),
        (
            FixtureChartBoundOperation::Divide,
            smallest,
            2.0,
            f64::MIN_POSITIVE,
        ),
        (
            FixtureChartBoundOperation::Add,
            smallest,
            smallest,
            f64::MIN_POSITIVE,
        ),
        (
            FixtureChartBoundOperation::Add,
            largest_subnormal,
            0.0,
            f64::MIN_POSITIVE,
        ),
        (
            FixtureChartBoundOperation::Add,
            f64::MIN_POSITIVE,
            0.0,
            next_minimum,
        ),
        (
            FixtureChartBoundOperation::Divide,
            1.0,
            2.0,
            f64::from_bits(0x3fe0_0000_0000_0001),
        ),
        (
            FixtureChartBoundOperation::Multiply,
            f64::MIN_POSITIVE,
            below_one,
            next_minimum,
        ),
    ] {
        assert_eq!(
            bound_arithmetic(left, right, operation)?.to_bits(),
            expected.to_bits()
        );
    }
    Ok(())
}

/// Reject the complete raw operand domain before exact-zero shortcuts.
/// # Panics
/// Reject negative/nonfinite acceptance, wrong precedence or a finite MAX cap.
#[test]
fn upper_operators_reject_invalid_and_exhausted_bounds() {
    for operation in [
        FixtureChartBoundOperation::Add,
        FixtureChartBoundOperation::Multiply,
        FixtureChartBoundOperation::Divide,
    ] {
        for invalid in [-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let expected = Err(FixtureChartError::Bound {
                operation,
                issue: FixtureChartBoundIssue::InvalidOperand,
            });
            assert_eq!(bound_arithmetic(invalid, 0.0, operation), expected);
            assert_eq!(bound_arithmetic(0.0, invalid, operation), expected);
        }
    }
    for divisor in [0.0, -0.0] {
        assert_eq!(
            bound_arithmetic(0.0, divisor, FixtureChartBoundOperation::Divide),
            Err(FixtureChartError::Bound {
                operation: FixtureChartBoundOperation::Divide,
                issue: FixtureChartBoundIssue::NonPositiveDivisor,
            })
        );
    }
    for (operation, right) in [
        (FixtureChartBoundOperation::Add, 0.0),
        (FixtureChartBoundOperation::Multiply, 2.0),
        (FixtureChartBoundOperation::Divide, 0.5),
    ] {
        assert_eq!(
            bound_arithmetic(f64::MAX, right, operation),
            Err(FixtureChartError::Bound {
                operation,
                issue: FixtureChartBoundIssue::NonFiniteResult,
            })
        );
    }
}

/// Bound widening never changes the coordinate owner's supported domain.
/// # Errors
/// Preserve any unexpected rejection of the signed-zero neighboring value.
/// # Panics
/// Reject a wrong primitive/stage/cause or clamped coordinate underflow.
#[test]
fn value_primitives_preserve_steps_and_reject_underflow() -> TestResult {
    assert_eq!(
        value_arithmetic(-0.0, 0.0, FixtureChartStep::AnchorCenter)?.to_bits(),
        (-0.0_f64).to_bits()
    );
    for (right, issue) in [
        (0.5, ArithmeticIssue::Subnormal),
        (UNIT_ROUNDOFF, ArithmeticIssue::ZeroUnderflow),
    ] {
        assert_eq!(
            value_arithmetic(f64::MIN_POSITIVE, right, FixtureChartStep::RateTimesElapsed),
            Err(FixtureChartError::Arithmetic {
                step: FixtureChartStep::RateTimesElapsed,
                source: FitError::Arithmetic {
                    stage: ArithmeticStage::Response,
                    operation: ArithmeticOperation::Multiply,
                    issue,
                },
            })
        );
        assert_eq!(
            bound_arithmetic(
                f64::MIN_POSITIVE,
                right,
                FixtureChartBoundOperation::Multiply
            )?
            .to_bits(),
            f64::MIN_POSITIVE.to_bits()
        );
    }
    for (step, left, right, operation, issue) in [
        (
            FixtureChartStep::SecondsDivide,
            1.0,
            0.0,
            ArithmeticOperation::Divide,
            ArithmeticIssue::NonPositiveDenominator,
        ),
        (
            FixtureChartStep::RateTimesElapsed,
            f64::MAX,
            2.0,
            ArithmeticOperation::Multiply,
            ArithmeticIssue::NonFinite,
        ),
        (
            FixtureChartStep::Displacement,
            f64::MAX,
            f64::MAX,
            ArithmeticOperation::Add,
            ArithmeticIssue::NonFinite,
        ),
        (
            FixtureChartStep::AnchorCenter,
            f64::MAX,
            -f64::MAX,
            ArithmeticOperation::Subtract,
            ArithmeticIssue::NonFinite,
        ),
        (
            FixtureChartStep::TrackingPhase,
            f64::MAX,
            -f64::MAX,
            ArithmeticOperation::Subtract,
            ArithmeticIssue::NonFinite,
        ),
        (
            FixtureChartStep::Rate,
            f64::MAX,
            -f64::MAX,
            ArithmeticOperation::Subtract,
            ArithmeticIssue::NonFinite,
        ),
    ] {
        assert_eq!(
            value_arithmetic(left, right, step),
            Err(FixtureChartError::Arithmetic {
                step,
                source: FitError::Arithmetic {
                    stage: ArithmeticStage::Response,
                    operation,
                    issue,
                },
            })
        );
    }
    Ok(())
}

/// Signed differences reuse exact integer limits rather than absolute casts.
/// # Errors
/// Preserve unexpected conversion rejection beside exact limit neighbors.
/// # Panics
/// Reject a lost sign, changed integer magnitude, overflow or weakened guard.
#[test]
fn signed_elapsed_preserves_exact_integer_limits() -> TestResult {
    for (query, reference, expected_seconds) in [
        (0, 0, 0),
        (1, 0, 0x3e11_2e0b_e826_d695),
        (0, 1, 0xbe11_2e0b_e826_d695),
    ] {
        let result = elapsed(MonotonicNs::new(query), MonotonicNs::new(reference))?;
        assert_eq!(result.seconds.to_bits(), expected_seconds);
    }
    let limit = 1_u64 << 53;
    let result = elapsed(MonotonicNs::new(limit), MonotonicNs::new(0))?;
    assert_eq!(result.nanoseconds.to_bits(), 0x4340_0000_0000_0000);
    for query in [limit + 1, u64::MAX] {
        assert!(matches!(
            elapsed(MonotonicNs::new(query), MonotonicNs::new(0)),
            Err(FixtureChartError::Normalization {
                source: NormalizationError::IntegerRange {
                    component: NormalizationComponent::Time,
                },
            })
        ));
    }
    Ok(())
}

/// Construct a genuine checked active duration and one independent chart domain.
/// # Errors
/// Preserve typed clock, frame, domain and settled arithmetic failures.
fn settled_effect(
    rate: f64,
    start: u64,
    end: u64,
) -> Result<FixtureAffineChart, FixtureChartError> {
    let elapsed: ActiveAwakeDurationNs = ActiveAwakeNs::new(end)
        .duration_since(ActiveAwakeNs::new(start))
        .map_err(|source| FixtureChartError::Domain { source })?;
    let frames = FixtureChartFrames::new(
        NonZeroU64::MIN,
        NonZeroU64::MIN,
        NonZeroU64::new(2).ok_or(FixtureChartError::Invariant)?,
        NonZeroU64::new(3).ok_or(FixtureChartError::Invariant)?,
    )?;
    let domain = FixtureChartDomain::new(
        MonotonicNs::new(0),
        MonotonicNs::new(1_000_000_000),
        MonotonicNs::new(2_000_000_000),
    )?;
    FixtureAffineChart::settled_rate_effect(frames, domain, rate, elapsed)
}

/// Compare with Q=(2^54-1)/(2^54*10^9) using integer cross-products only.
///
/// This Q follows from the exact stored 1/3 bits and three whole nanoseconds.
/// It retains a factor 5^9 in its denominator, so no binary64 point equals Q.
/// The admitted point's binade gives common denominator 2^82*5^9; canceling
/// powers of two keeps every actual comparison inside checked u128 arithmetic.
/// # Errors
/// Preserve an unexpected integer exponent-conversion failure.
/// # Panics
/// Reject zero/lost error, a wrong point binade or an enclosure excluding Q.
fn covers_stored_third_for_three_ns(point: f64, error: f64) -> TestResult {
    let bits = point.abs().to_bits();
    assert_eq!((bits >> 52) & 0x7ff, 0x3e1);
    assert!(error.is_normal() && error > 0.0);
    let significand = u128::from((bits & 0x000f_ffff_ffff_ffff) | (1_u64 << 52));
    let actual = significand * 1_953_125;
    let exact = ((1_u128 << 54) - 1) * (1_u128 << 19);
    let difference = actual.abs_diff(exact);
    assert!(difference > 0);
    let error_bits = error.to_bits();
    let error_significand = u128::from((error_bits & 0x000f_ffff_ffff_ffff) | (1_u64 << 52));
    let exponent = i32::try_from((error_bits >> 52) & 0x7ff)
        .map_err(|_| FixtureChartError::Invariant)?
        - 1023
        - 52
        + 82;
    let scaled_error = error_significand * 1_953_125;
    if exponent < 0 {
        let shift = exponent.unsigned_abs();
        let scaled_difference = 1_u128
            .checked_shl(shift)
            .and_then(|scale| difference.checked_mul(scale));
        assert!(scaled_difference.is_some_and(|value| value <= scaled_error));
    } else {
        let scaled_bound = u32::try_from(exponent)
            .ok()
            .and_then(|shift| 1_u128.checked_shl(shift))
            .and_then(|scale| scaled_error.checked_mul(scale));
        // Overflow here proves a positive bound exceeds the bounded difference.
        assert!(scaled_bound.is_none_or(|value| difference <= value));
    }
    Ok(())
}

/// General stored rate and inexact seconds require error before composition.
/// # Errors
/// Preserve unexpected constructor or checked arithmetic failure.
/// # Panics
/// Reject an exact-scalar factory, omitted integration error or wrong sign/units.
#[test]
fn settled_rate_preserves_general_binary64_integration_error() -> TestResult {
    let positive = f64::from_bits(0x3fd5_5555_5555_5555);
    for rate in [positive, -positive] {
        let effect = settled_effect(rate, 0, 3)?;
        let point = effect.phase_at_reference_seconds();
        assert_eq!(point.is_sign_negative(), rate.is_sign_negative());
        covers_stored_third_for_three_ns(point, effect.phase_coefficient_roundoff_seconds())?;
        assert_eq!(
            effect.rate_seconds_per_second().to_bits(),
            (-0.0_f64).to_bits()
        );
        assert_eq!(
            effect
                .rate_coefficient_roundoff_seconds_per_second()
                .to_bits(),
            0
        );
    }
    Ok(())
}

/// Exact dyadics and both zero signs remain supported without treating zero as no effect.
/// # Errors
/// Preserve unexpected checked construction failure.
/// # Panics
/// Reject changed zero signs, rate attachment or exact integer/dyadic results.
#[test]
fn settled_rate_keeps_dyadic_and_signed_zero_points() -> TestResult {
    for (rate, duration, expected) in [
        (1.0 / 1024.0, 2_000_000_000, 1.0 / 512.0),
        (-1.0 / 1024.0, 2_000_000_000, -1.0 / 512.0),
        (0.0, 0, 0.0),
        (-0.0, 0, -0.0),
        (0.0, 2_000_000_000, 0.0),
        (-0.0, 2_000_000_000, -0.0),
        (1.0 / 1024.0, 0, 0.0),
        (-1.0 / 1024.0, 0, -0.0),
    ] {
        let effect = settled_effect(rate, 0, duration)?;
        assert_eq!(
            effect.phase_at_reference_seconds().to_bits(),
            f64::to_bits(expected)
        );
        assert_eq!(
            effect.rate_seconds_per_second().to_bits(),
            (-0.0_f64).to_bits()
        );
        assert!(effect.phase_coefficient_roundoff_seconds().is_finite());
        assert_eq!(
            effect
                .rate_coefficient_roundoff_seconds_per_second()
                .to_bits(),
            0
        );
    }
    Ok(())
}

/// Exact active deltas, not absolute reading casts, own the inclusive 2^53 boundary.
/// # Errors
/// Preserve unexpected supported conversion failure.
/// # Panics
/// Reject origin-dependent results, rounded-over-limit admission or an altered error.
#[test]
fn settled_rate_uses_exact_active_duration_and_integer_neighbors() -> TestResult {
    let limit = 1_u64 << 53;
    for duration in [limit - 1, limit] {
        let low = settled_effect(1.0 / 1024.0, 0, duration)?;
        let high = settled_effect(1.0 / 1024.0, u64::MAX - duration, u64::MAX)?;
        assert_eq!(
            low.phase_at_reference_seconds().to_bits(),
            high.phase_at_reference_seconds().to_bits()
        );
        assert_eq!(
            low.phase_coefficient_roundoff_seconds().to_bits(),
            high.phase_coefficient_roundoff_seconds().to_bits()
        );
    }
    for duration in [limit + 1, u64::MAX] {
        assert!(matches!(
            settled_effect(1.0 / 1024.0, 0, duration),
            Err(FixtureChartError::Normalization {
                source: NormalizationError::IntegerRange {
                    component: NormalizationComponent::Time
                }
            })
        ));
    }
    Ok(())
}

/// Raw rate support and reachable finite arithmetic failures keep their actual owners.
/// # Errors
/// Preserve unexpected healthy-neighbor rejection.
/// # Panics
/// Reject a policy cap, malformed rate, value clamp or unrelated arithmetic cause.
#[test]
fn settled_rate_rejects_unsupported_rates_and_reachable_results() -> TestResult {
    for rate in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(
            FixtureAffineChart::check_delivery_rate(rate),
            Err(FixtureChartError::InvalidScalar {
                field: FixtureChartField::Rate,
                issue: FixtureChartIssue::NonFinite
            })
        );
    }
    for rate in [
        f64::from_bits(1),
        -f64::from_bits(1),
        f64::from_bits(0x000f_ffff_ffff_ffff),
        -f64::from_bits(0x000f_ffff_ffff_ffff),
    ] {
        assert_eq!(
            FixtureAffineChart::check_delivery_rate(rate),
            Err(FixtureChartError::InvalidScalar {
                field: FixtureChartField::Rate,
                issue: FixtureChartIssue::Subnormal
            })
        );
    }
    for rate in [
        0.0,
        -0.0,
        f64::MIN_POSITIVE,
        -f64::MIN_POSITIVE,
        f64::MAX,
        -f64::MAX,
    ] {
        FixtureAffineChart::check_delivery_rate(rate)?;
    }
    for (rate, duration, issue) in [
        (f64::MAX, 2_000_000_000, ArithmeticIssue::NonFinite),
        (f64::MIN_POSITIVE, 1, ArithmeticIssue::Subnormal),
    ] {
        assert!(
            matches!(settled_effect(rate, 0, duration), Err(FixtureChartError::Arithmetic {
            step: FixtureChartStep::RateTimesElapsed,
            source: FitError::Arithmetic { stage: ArithmeticStage::Response,
                operation: ArithmeticOperation::Multiply, issue: actual } }) if actual == issue)
        );
    }
    assert_eq!(
        settled_effect(f64::MIN_POSITIVE, 0, 1_000_000_000)?
            .phase_at_reference_seconds()
            .to_bits(),
        f64::MIN_POSITIVE.to_bits()
    );
    assert_eq!(
        settled_effect(f64::MAX, 0, 500_000_000)?
            .phase_at_reference_seconds()
            .to_bits(),
        (f64::MAX / 2.0).to_bits()
    );
    Ok(())
}

// SPDX-License-Identifier: GPL-2.0-only
//! Independent exact-bit and endpoint controls for the private companion owner.
//!
//! Numerical expectations below are fixed binary64/rational facts, not outputs
//! derived from the production propagation equations. No history, selection,
//! checked projection, participant or lifecycle state is fabricated here.

use std::error::Error;

use crate::{
    ArithmeticIssue, ArithmeticOperation, ArithmeticStage, FitError, FixtureChartBoundIssue,
    FixtureChartBoundOperation, FixtureChartError, FixtureCombinationError, FixtureCombinationStep,
};

use super::super::{Companion, CompanionError};
use super::{UNIT_ROUNDOFF, lower_margin};

/// Unexpected checked arithmetic failures remain actual typed test failures.
type TestResult = Result<(), CompanionError>;

/// Signed point zero and subnormal errors retain their exact admitted bits.
/// # Errors
/// Preserve any unexpected mandatory construction rejection.
/// # Panics
/// Reject representation widening, nonfinite acceptance or wrong field priority.
#[test]
fn companion_construction_preserves_supported_bits_and_rejects_invalid_fields() -> TestResult {
    for value in [0.0, -0.0, 1.0, -1.0, f64::MAX] {
        for error in [0.0, -0.0, f64::from_bits(1), f64::MIN_POSITIVE] {
            let admitted = Companion::new(value, error)?;
            assert_eq!(admitted.value().to_bits(), value.to_bits());
            assert_eq!(admitted.error().to_bits(), error.to_bits());
        }
    }
    for (value, issue) in [
        (f64::NAN, ArithmeticIssue::NonFinite),
        (f64::INFINITY, ArithmeticIssue::NonFinite),
        (f64::NEG_INFINITY, ArithmeticIssue::NonFinite),
        (f64::from_bits(1), ArithmeticIssue::Subnormal),
        (-f64::from_bits(1), ArithmeticIssue::Subnormal),
    ] {
        assert_eq!(
            Companion::new(value, f64::NAN).err(),
            Some(CompanionError::InvalidPoint { issue })
        );
    }
    for error in [-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(
            Companion::new(1.0, error).err(),
            Some(CompanionError::InvalidError)
        );
    }
    Ok(())
}

/// Rounded subtraction ties cannot become an upward denominator lower bound.
/// # Errors
/// Preserve any unexpected legal lower-margin rejection.
/// # Panics
/// Reject upward clamping, equality acceptance or loss of the smallest margin.
#[test]
fn positive_margin_keeps_ties_and_exact_subnormal_differences() -> TestResult {
    let half_below_one = f64::from_bits(0x3c90_0000_0000_0000);
    assert_eq!(
        lower_margin(1.0, half_below_one)?.to_bits(),
        0x3fef_ffff_ffff_ffff
    );
    let next_minimum = f64::from_bits(0x0010_0000_0000_0001);
    assert_eq!(lower_margin(next_minimum, f64::MIN_POSITIVE)?.to_bits(), 1);
    assert_eq!(lower_margin(f64::from_bits(1), 0.0)?.to_bits(), 1);
    assert_eq!(
        lower_margin(f64::from_bits(2), f64::from_bits(1))?.to_bits(),
        1
    );
    assert_eq!(
        lower_margin(f64::MIN_POSITIVE, 0.0)?.to_bits(),
        0x000f_ffff_ffff_ffff
    );
    let below_one = f64::from_bits(0x3fef_ffff_ffff_ffff);
    let lower = lower_margin(1.0, below_one)?;
    assert!(lower > 0.0 && lower <= UNIT_ROUNDOFF);
    for (value, error) in [
        (1.0, 1.0),
        (1.0, 2.0),
        (0.0, 0.0),
        (-0.0, 0.0),
        (-1.0, 0.0),
        (f64::NAN, 0.0),
        (f64::INFINITY, 0.0),
        (1.0, f64::NAN),
        (1.0, f64::INFINITY),
        (1.0, -1.0),
    ] {
        assert_eq!(
            lower_margin(value, error),
            Err(CompanionError::UnprovedPositiveMargin)
        );
    }
    Ok(())
}

/// Actual division consumes a positive subnormal margin without clamping it up.
/// # Errors
/// Preserve unexpected construction/division rejection beside the exact neighbor.
/// # Panics
/// Reject an underestimated endpoint error or a changed separately rounded point.
#[test]
fn positive_division_consumes_the_exact_smallest_subnormal_margin() -> TestResult {
    let numerator = Companion::exact(f64::MIN_POSITIVE)?;
    let point = f64::from_bits(0x0010_0000_0000_0001);
    let denominator = Companion::new(point, f64::MIN_POSITIVE)?;
    let quotient = numerator.divide_positive(denominator, FixtureCombinationStep::PhaseMean)?;
    assert_eq!(quotient.value().to_bits(), 0x3fef_ffff_ffff_fffe);
    // b-Eb is exactly2^-1074, so the represented denominator's lower endpoint
    // gives exact quotient2^52. The saved point is1-2^-52; its endpoint error
    // is2^52-1+2^-52, whose least binary64 upper is2^52-0.5.
    assert!(quotient.error().is_finite());
    assert!(quotient.error() >= f64::from_bits(0x432f_ffff_ffff_ffff));
    let healthy =
        numerator.divide_positive(Companion::exact(point)?, FixtureCombinationStep::PhaseMean)?;
    assert_eq!(healthy.value().to_bits(), quotient.value().to_bits());
    assert!(healthy.error() > 0.0 && healthy.error() < 1.0);
    Ok(())
}

/// Exact input intervals give independent endpoint errors for all four operators.
/// # Errors
/// Preserve any unexpected valid point/error rejection.
/// # Panics
/// Reject lost cross-error terms, point substitution or an underestimated bound.
#[test]
fn companion_operations_contain_independent_interval_endpoints() -> TestResult {
    let left = Companion::new(1.5, 0.5)?;
    let right = Companion::new(2.0, 0.25)?;
    let sum = left.add(right, FixtureCombinationStep::PhaseSum)?;
    let difference = left.subtract(right, FixtureCombinationStep::PrincipalDifference)?;
    let product = left.multiply(right, FixtureCombinationStep::WeightedPhase)?;
    let quotient = left.divide_positive(right, FixtureCombinationStep::PhaseMean)?;
    assert_eq!(sum.value().to_bits(), 3.5_f64.to_bits());
    assert_eq!(difference.value().to_bits(), (-0.5_f64).to_bits());
    assert_eq!(product.value().to_bits(), 3.0_f64.to_bits());
    assert_eq!(quotient.value().to_bits(), 0.75_f64.to_bits());
    assert!(sum.error() >= 0.75 && difference.error() >= 0.75);
    assert!(product.error() >= 1.5);
    // max |A/B - 3/4| over [1,2]/[7/4,9/4] is11/28.
    // 0x3fd924924924924a is the least binary64 upper for that exact rational.
    assert!(quotient.error() >= f64::from_bits(0x3fd9_2492_4924_924a));
    assert_eq!(left.value().to_bits(), 1.5_f64.to_bits());
    assert_eq!(right.error().to_bits(), 0.25_f64.to_bits());

    // The same endpoint widths apply when both stored inputs are negative.
    // Division keeps its denominator positive and negates only the numerator.
    let negative_left = Companion::new(-1.5, 0.5)?;
    let negative_right = Companion::new(-2.0, 0.25)?;
    let negative_sum = negative_left.add(negative_right, FixtureCombinationStep::PhaseSum)?;
    let negative_difference =
        negative_left.subtract(negative_right, FixtureCombinationStep::PrincipalDifference)?;
    let negative_product =
        negative_left.multiply(negative_right, FixtureCombinationStep::WeightedPhase)?;
    let negative_quotient =
        negative_left.divide_positive(right, FixtureCombinationStep::PhaseMean)?;
    assert_eq!(negative_sum.value().to_bits(), (-3.5_f64).to_bits());
    assert_eq!(negative_difference.value().to_bits(), 0.5_f64.to_bits());
    assert_eq!(negative_product.value().to_bits(), 3.0_f64.to_bits());
    assert_eq!(negative_quotient.value().to_bits(), (-0.75_f64).to_bits());
    assert!(negative_sum.error() >= 0.75 && negative_difference.error() >= 0.75);
    assert!(negative_product.error() >= 1.5);
    assert!(negative_quotient.error() >= f64::from_bits(0x3fd9_2492_4924_924a));
    Ok(())
}

/// Each separately rounded tie needs its own error, even from exact stored inputs.
/// # Errors
/// Preserve actual checked operation failures beside supported controls.
/// # Panics
/// Reject reassociation, lost signed zero or an absent new rounding allowance.
#[test]
fn exact_input_ties_and_cancellation_retain_separate_rounding() -> TestResult {
    let large = Companion::exact(9_007_199_254_740_992.0)?;
    let one = Companion::exact(1.0)?;
    let tied = large.add(one, FixtureCombinationStep::PhaseSum)?;
    assert_eq!(tied.value().to_bits(), large.value().to_bits());
    assert!(tied.error() >= 1.0);
    let cancelled = tied.subtract(large, FixtureCombinationStep::PrincipalDifference)?;
    assert_eq!(cancelled.value().to_bits(), 0);
    assert!(cancelled.error() >= 1.0);
    let negative_zero = Companion::exact(-0.0)?.multiply(
        Companion::exact(2.0)?,
        FixtureCombinationStep::WeightedPhase,
    )?;
    assert_eq!(negative_zero.value().to_bits(), (-0.0_f64).to_bits());
    assert!(negative_zero.error() > 0.0);
    Ok(())
}

/// Sixteen exact stored0.1 values have a different ascending rounded mean.
/// # Errors
/// Preserve unexpected weighted-product/sum/final-division failures.
/// # Panics
/// Reject a once-rounded reference, different reduction order or missing sum error.
#[test]
fn sixteen_stored_tenths_include_new_rounded_reduction_error() -> TestResult {
    let value = Companion::exact(f64::from_bits(0x3fb9_9999_9999_999a))?;
    let weight = Companion::exact(1.0)?;
    let mut weights = Companion::exact(0.0)?;
    let mut weighted = Companion::exact(0.0)?;
    for _ in 0..16 {
        weights = weights.add(weight, FixtureCombinationStep::PhaseWeightSum)?;
        let contribution = weight.multiply(value, FixtureCombinationStep::WeightedPhase)?;
        weighted = weighted.add(contribution, FixtureCombinationStep::PhaseSum)?;
    }
    let mean = weighted.divide_positive(weights, FixtureCombinationStep::PhaseMean)?;
    assert_eq!(mean.value().to_bits(), 0x3fb9_9999_9999_999b);
    // The exact mean of identical stored bits is0x...99a. The next stored point
    // differs by exactly2^-56, independent of all production bound equations.
    assert!(mean.error() >= f64::from_bits(0x3c70_0000_0000_0000));
    assert_eq!(value.error().to_bits(), 0);
    Ok(())
}

/// The reciprocal must carry the separately rounded SD square's error.
/// # Errors
/// Preserve unexpected actual variance/weight arithmetic failures.
/// # Panics
/// Reject a replaced weight formula or a reciprocal-only error allowance.
#[test]
fn reciprocal_rate_weight_retains_the_sd_square_error() -> TestResult {
    let deviation = Companion::exact(f64::from_bits(0x3ff6_6666_6666_6666))?;
    let variance = deviation.multiply(deviation, FixtureCombinationStep::RateVariance)?;
    let weight =
        Companion::exact(1.0)?.divide_positive(variance, FixtureCombinationStep::RateWeight)?;
    assert_eq!(variance.value().to_bits(), 0x3fff_5c28_f5c2_8f5b);
    assert_eq!(weight.value().to_bits(), 0x3fe0_5397_829c_bc16);
    // Exact stored-input reciprocal-weight error is the rational
    // 2578763506749997493828118774723 /
    // 44758487643258896190913158979468628915717668864.
    // Its least binary64 upper is0x3c909b3df290cb02. This exceeds the
    // reciprocal-only u*Q term (0x1.05397829cbc16p-54), detecting lost square error.
    assert!(weight.error() >= f64::from_bits(0x3c90_9b3d_f290_cb02));
    Ok(())
}

/// Unsupported points and unavailable bound margins retain distinct typed causes.
/// # Errors
/// Preserve construction failures in healthy controls.
/// # Panics
/// Reject overflow publication, zero/missing margins or mutated input companions.
#[test]
fn companion_failures_preserve_actual_point_and_bound_owners() -> TestResult {
    let largest = Companion::exact(f64::MAX)?;
    let doubled = largest.multiply(
        Companion::exact(2.0)?,
        FixtureCombinationStep::WeightedPhase,
    );
    let expected = CompanionError::Value {
        source: FixtureCombinationError::Arithmetic {
            step: FixtureCombinationStep::WeightedPhase,
            source: FitError::Arithmetic {
                stage: ArithmeticStage::Combination,
                operation: ArithmeticOperation::Multiply,
                issue: ArithmeticIssue::NonFinite,
            },
        },
    };
    assert_eq!(doubled.err(), Some(expected));
    assert!(expected.source().is_some());
    let no_margin = Companion::new(1.0, 1.0)?;
    assert_eq!(
        Companion::exact(1.0)?
            .divide_positive(no_margin, FixtureCombinationStep::PhaseMean)
            .err(),
        Some(CompanionError::UnprovedPositiveMargin)
    );
    let broad = Companion::new(1.0, f64::MAX)?;
    let upper = broad
        .add(Companion::exact(1.0)?, FixtureCombinationStep::PhaseSum)
        .err();
    let expected = CompanionError::Bound {
        source: FixtureChartError::Bound {
            operation: FixtureChartBoundOperation::Add,
            issue: FixtureChartBoundIssue::NonFiniteResult,
        },
    };
    assert_eq!(upper, Some(expected));
    assert!(expected.source().is_some());
    assert_eq!(largest.value().to_bits(), f64::MAX.to_bits());
    assert_eq!(broad.error().to_bits(), f64::MAX.to_bits());
    Ok(())
}

/// Diagnostics expose bounded families while preserving no raw companion scalar.
/// # Errors
/// Preserve unexpected legitimate constructor failure.
/// # Panics
/// Reject numeric formatting or a fabricated nested representation cause.
#[test]
fn companion_diagnostics_are_bounded_and_keep_real_source_chains() -> TestResult {
    let value = Companion::new(-3.5, 0.125)?;
    assert_eq!(format!("{value:?}"), "Companion { .. }");
    let error = CompanionError::InvalidError;
    assert_eq!(format!("{error:?}"), "CompanionError { .. }");
    assert_eq!(
        error.to_string(),
        "combination companion: unsupported error bound"
    );
    assert!(error.source().is_none());
    Ok(())
}

// SPDX-License-Identifier: GPL-2.0-only
//! Independent external controls for the frozen S1.2 whole-window divisor contract.
//!
//! Expectations are literal dyadic facts, hand-derived rational anchors and
//! explicitly identified IEEE binary64 rounding facts. No product helper supplies
//! expected divisors. Each call has at most 65 rows, including the rejected
//! one-over boundary; the largest invalid-input sweep has 448 calls. These pure
//! controls establish no sample eligibility, physical confidence or clock effect.
#![forbid(unsafe_code)]

use time_core::{WindowVarianceDivisors, WindowVarianceError, WindowVarianceStage};

/// Debug is an intentionally redacted projection, separate from numeric getters.
///
/// # Errors
/// Propagates rejection of the legal window.
///
/// # Panics
/// Requires useful divisors beside a debug view containing no raw numeric inputs.
#[test]
fn debug_projection_omits_distances_and_precision() -> TestResult {
    let distances = [1.0, 1.0, 2.0];
    let window = WindowVarianceDivisors::derive(&distances, 0.25)?;
    assert_eq!(window.divisors()[2].to_bits(), 16.0_f64.to_bits());
    assert_eq!(format!("{window:?}"), "WindowVarianceDivisors { .. }");
    Ok(())
}

/// Public error data formats without raw operands or nested validation reports.
/// These constructed errors exercise diagnostics, not arithmetic reachability.
///
/// # Panics
/// Requires bounded nonempty diagnostics for every error and arithmetic-step tag.
#[test]
fn error_diagnostics_are_bounded_without_validation_payloads() {
    let mut errors = vec![
        WindowVarianceError::CountOutOfRange { count: usize::MAX },
        WindowVarianceError::InvalidPrecision,
        WindowVarianceError::InvalidDistance { index: usize::MAX },
        WindowVarianceError::ValidationInvariant,
    ];
    errors.extend(
        [
            WindowVarianceStage::MedianSum,
            WindowVarianceStage::MedianAverage,
            WindowVarianceStage::MedianDifference,
            WindowVarianceStage::Spread,
            WindowVarianceStage::Threshold,
            WindowVarianceStage::Excess,
            WindowVarianceStage::Ratio,
            WindowVarianceStage::Increment,
            WindowVarianceStage::Square,
        ]
        .map(|stage| WindowVarianceError::Arithmetic { stage }),
    );
    for error in errors {
        let message = error.to_string();
        assert!(!message.is_empty() && message.len() <= 96);
        assert!(std::error::Error::source(&error).is_none());
    }
}

/// Preserve unexpected typed product failures separately from assertion failures.
type TestResult = Result<(), WindowVarianceError>;

/// Absolute comparison allowance 2^-48 for the moderate nondyadic rational facts.
const RATIONAL_TOLERANCE: f64 = 1.0 / 281_474_976_710_656.0;

/// Smallest positive subnormal, retained as an exact one-quantum input.
const QUANTUM: f64 = f64::from_bits(1);

/// Every rejected scalar class, including both zeros and a negative subnormal.
const INVALID_SCALARS: [f64; 7] = [
    0.0,
    -0.0,
    -1.0,
    f64::from_bits(0x8000_0000_0000_0001),
    f64::from_bits(0x7ff8_0000_0000_0042),
    f64::INFINITY,
    f64::NEG_INFINITY,
];

/// Check exact output bits plus unchanged borrowed input, precision and row order.
///
/// # Errors
/// Propagates an unexpected rejection of the supplied healthy case.
///
/// # Panics
/// Asserts the independently supplied divisors and all snapshot guarantees.
fn exact_case<const N: usize>(
    distances: &[f64; N],
    precision: f64,
    expected: [f64; N],
) -> TestResult {
    let before = (*distances).map(f64::to_bits);
    let result = WindowVarianceDivisors::derive(distances, precision)?;
    assert_eq!(result.divisors().len(), N);
    assert!(
        result
            .divisors()
            .iter()
            .map(|value| value.to_bits())
            .eq(expected.map(f64::to_bits))
    );
    assert!(std::ptr::eq(
        result.peer_distances_seconds(),
        distances.as_slice()
    ));
    assert_eq!(
        result.local_precision_seconds().to_bits(),
        precision.to_bits()
    );
    assert_eq!((*distances).map(f64::to_bits), before);
    Ok(())
}

/// Check predeclared rational anchors without reproducing the derivation.
///
/// # Errors
/// Propagates an unexpected rejection of the supplied healthy case.
///
/// # Panics
/// Asserts finite outputs within the fixed absolute allowance and snapshot custody.
fn rational_case<const N: usize>(
    distances: &[f64; N],
    precision: f64,
    expected: [f64; N],
) -> TestResult {
    let before = (*distances).map(f64::to_bits);
    let result = WindowVarianceDivisors::derive(distances, precision)?;
    assert_eq!(result.divisors().len(), N);
    for (actual, anchor) in result.divisors().iter().zip(expected) {
        assert!(actual.is_finite());
        assert!(
            (*actual - anchor).abs() <= RATIONAL_TOLERANCE,
            "actual={actual:?}, independent rational anchor={anchor:?}"
        );
    }
    assert!(std::ptr::eq(
        result.peer_distances_seconds(),
        distances.as_slice()
    ));
    assert_eq!(
        result.local_precision_seconds().to_bits(),
        precision.to_bits()
    );
    assert_eq!((*distances).map(f64::to_bits), before);
    Ok(())
}

/// Require one complete typed rejection while preserving every original input bit.
///
/// # Panics
/// Asserts the exact error, including precedence/index/stage, and input preservation.
fn rejected_case<const N: usize>(
    distances: &[f64; N],
    precision: f64,
    expected: WindowVarianceError,
) {
    let before = (*distances).map(f64::to_bits);
    assert_eq!(
        WindowVarianceDivisors::derive(distances, precision).err(),
        Some(expected)
    );
    assert_eq!((*distances).map(f64::to_bits), before);
}

/// Retain healthy unit controls before the feature's discriminating squared scale.
///
/// # Errors
/// Propagates an unexpected healthy-construction rejection.
///
/// # Panics
/// Requires exact whole-window divisors; its nonunit assertion is the RED anchor.
#[test]
fn whole_window_quality_uses_precision_and_squared_scale() -> TestResult {
    exact_case(&[1.0], 0.25, [1.0])?;
    exact_case(&[1.0; 4], 0.25, [1.0; 4])?;
    let distances = [1.0, 1.0, 2.0];
    let result = WindowVarianceDivisors::derive(&distances, 0.25)?;
    assert!(
        result
            .divisors()
            .iter()
            .map(|value| value.to_bits())
            .eq([1.0_f64, 1.0, 16.0].map(f64::to_bits)),
        "window quality did not derive the whole-window squared divisor"
    );
    Ok(())
}

/// An upper-clamped spread gives dyadic divisors with reduced high-distance weight.
///
/// # Errors
/// Propagates unexpected rejection.
///
/// # Panics
/// Requires the independently squared 7/4 and 15/4 scales.
#[test]
fn upper_clamped_spread_has_exact_dyadic_divisors() -> TestResult {
    exact_case(&[1.0, 2.0, 4.0], 0.25, [1.0, 49.0 / 16.0, 225.0 / 16.0])
}

/// The interior 0.7 coefficient gives rational scales 27/20 and 41/20.
///
/// # Errors
/// Propagates unexpected rejection.
///
/// # Panics
/// Requires rational squared-scale anchors within the declared allowance.
#[test]
fn interior_spread_uses_the_point_seven_coefficient() -> TestResult {
    rational_case(&[1.0, 1.5, 2.0], 0.25, [1.0, 729.0 / 400.0, 1681.0 / 400.0])
}

/// Precision wins when the source clamp's nominal bounds are inverted.
///
/// # Errors
/// Propagates unexpected rejection.
///
/// # Panics
/// Requires the legal precision-greater-than-minimum cases without a clamp panic.
#[test]
fn precision_above_minimum_is_legal_and_wins_the_clamp() -> TestResult {
    exact_case(&[1.0], 2.0, [1.0])?;
    exact_case(&[1.0, 2.0, 4.0], 2.0, [1.0, 1.0, 9.0 / 4.0])
}

/// A whole-window divisor can legitimately exceed the fixture receiver's cap 16.
///
/// # Errors
/// Propagates unexpected rejection.
///
/// # Panics
/// Requires the exact squared scale 12 squared, without clipping or reciprocal use.
#[test]
fn legal_divisors_are_not_capped_at_sixteen() -> TestResult {
    exact_case(&[1.0, 1.0, 4.0], 0.25, [1.0, 1.0, 144.0])
}

/// Unsorted even middles 5/4 and 7/4 define median 3/2 in scratch storage.
///
/// # Errors
/// Propagates unexpected rejection.
///
/// # Panics
/// Requires independently derived results in original order and unchanged input.
#[test]
fn even_median_preserves_original_order_and_storage() -> TestResult {
    rational_case(
        &[2.0, 1.0, 1.75, 1.25],
        0.25,
        [1681.0 / 400.0, 1.0, 289.0 / 100.0, 1.0],
    )
}

/// All six permutations move each exact divisor with its corresponding distance.
///
/// # Errors
/// Propagates unexpected rejection of any bounded permutation.
///
/// # Panics
/// Requires the six literal, independently paired output orders.
#[test]
fn permutations_preserve_distance_to_divisor_correspondence() -> TestResult {
    for (distances, expected) in [
        ([1.0, 2.0, 4.0], [1.0, 49.0 / 16.0, 225.0 / 16.0]),
        ([1.0, 4.0, 2.0], [1.0, 225.0 / 16.0, 49.0 / 16.0]),
        ([2.0, 1.0, 4.0], [49.0 / 16.0, 1.0, 225.0 / 16.0]),
        ([2.0, 4.0, 1.0], [49.0 / 16.0, 225.0 / 16.0, 1.0]),
        ([4.0, 1.0, 2.0], [225.0 / 16.0, 1.0, 49.0 / 16.0]),
        ([4.0, 2.0, 1.0], [225.0 / 16.0, 49.0 / 16.0, 1.0]),
    ] {
        exact_case(&distances, 0.25, expected)?;
    }
    Ok(())
}

/// Changing only later window rows changes an earlier retained nonunit divisor.
///
/// # Errors
/// Propagates unexpected rejection.
///
/// # Panics
/// Requires distance 2 to change from divisor 16 to 49/16.
#[test]
fn changing_the_window_reweights_retained_rows() -> TestResult {
    exact_case(&[1.0, 1.0, 2.0], 0.25, [1.0, 1.0, 16.0])?;
    exact_case(
        &[1.0, 1.0, 2.0, 3.0, 3.0],
        0.25,
        [1.0, 1.0, 49.0 / 16.0, 121.0 / 16.0, 121.0 / 16.0],
    )
}

/// Exact power-of-two unit changes stay inside a safe ordinary binary64 envelope.
///
/// # Errors
/// Propagates unexpected rejection at either scale or the original units.
///
/// # Panics
/// Requires the same literal dyadic divisors for scales 2^-20, 1 and 2^20.
#[test]
fn safe_power_of_two_unit_scales_preserve_divisor_bits() -> TestResult {
    for scale in [1.0 / 1_048_576.0, 1.0, 1_048_576.0] {
        let distances = [1.0, 2.0, 4.0].map(|value| value * scale);
        exact_case(&distances, 0.25 * scale, [1.0, 49.0 / 16.0, 225.0 / 16.0])?;
    }
    Ok(())
}

/// The first representable distance above threshold creates exactly eight unit ulps.
///
/// # Errors
/// Propagates unexpected rejection of any threshold neighbor.
///
/// # Panics
/// Requires exact outputs below, at and immediately above 5/4.
#[test]
fn excess_threshold_has_exact_neighbor_results() -> TestResult {
    exact_case(&[1.0, 1.0, 1.25_f64.next_down()], 0.25, [1.0; 3])?;
    exact_case(&[1.0, 1.0, 1.25], 0.25, [1.0; 3])?;
    exact_case(
        &[1.0, 1.0, 1.25_f64.next_up()],
        0.25,
        [1.0, 1.0, f64::from_bits(1.0_f64.to_bits() + 8)],
    )
}

/// At median 39/32 and precision 5/16, the rounded raw spread equals precision.
///
/// # Errors
/// Propagates unexpected rejection of a neighboring median.
///
/// # Panics
/// Requires the lower-clamp plateau and strict decrease immediately above it.
#[test]
fn lower_spread_clamp_has_a_plateau_and_valid_upper_neighbor() -> TestResult {
    let median = 39.0_f64 / 32.0;
    let below_distances = [1.0, median.next_down(), 2.0];
    let at_distances = [1.0, median, 2.0];
    let above_distances = [1.0, median.next_up(), 2.0];
    rational_case(&at_distances, 5.0 / 16.0, [1.0, 1.0, 256.0 / 25.0])?;
    let below = WindowVarianceDivisors::derive(&below_distances, 5.0 / 16.0)?;
    let at = WindowVarianceDivisors::derive(&at_distances, 5.0 / 16.0)?;
    let above = WindowVarianceDivisors::derive(&above_distances, 5.0 / 16.0)?;
    assert_eq!(below.divisors()[2].to_bits(), at.divisors()[2].to_bits());
    assert!(above.divisors()[2] < at.divisors()[2]);
    Ok(())
}

/// Literal 1.7 minus 1 has the coefficient's stored 0.7 bits, giving spread 1.
///
/// # Errors
/// Propagates unexpected rejection of a neighboring median.
///
/// # Panics
/// Requires an exact upper-clamp plateau and strict increase below it.
#[test]
fn upper_spread_clamp_has_a_plateau_and_valid_lower_neighbor() -> TestResult {
    let median = 1.7_f64;
    let below_distances = [1.0, median.next_down(), 2.0];
    let at_distances = [1.0, median, 2.0];
    let above_distances = [1.0, median.next_up(), 2.0];
    // The middle row also exceeds 5/4: (1 + (17/10 - 5/4))² = 841/400.
    rational_case(&at_distances, 0.25, [1.0, 841.0 / 400.0, 49.0 / 16.0])?;
    rational_case(&above_distances, 0.25, [1.0, 841.0 / 400.0, 49.0 / 16.0])?;
    let at = WindowVarianceDivisors::derive(&at_distances, 0.25)?;
    let above = WindowVarianceDivisors::derive(&above_distances, 0.25)?;
    assert_eq!(at.divisors()[2].to_bits(), (49.0_f64 / 16.0).to_bits());
    assert_eq!(above.divisors()[2].to_bits(), (49.0_f64 / 16.0).to_bits());
    let below = WindowVarianceDivisors::derive(&below_distances, 0.25)?;
    assert!(below.divisors()[2] > 49.0 / 16.0);
    Ok(())
}

/// Two rows form a legal weighting window independently of fit-readiness minima.
///
/// # Errors
/// Propagates unexpected rejection.
///
/// # Panics
/// Requires median 2 and the exact high-distance divisor 121/16.
#[test]
fn two_rows_are_a_legal_weighting_window() -> TestResult {
    exact_case(&[1.0, 3.0], 0.25, [1.0, 121.0 / 16.0])
}

/// The admitted maximum retains a nonunit final row without truncation.
///
/// # Errors
/// Propagates unexpected rejection of 64 valid rows.
///
/// # Panics
/// Requires every original row and the final divisor 16.
#[test]
fn sixty_four_rows_retain_the_last_distance_and_divisor() -> TestResult {
    let mut distances = [1.0; 64];
    distances[63] = 2.0;
    let mut expected = [1.0; 64];
    expected[63] = 16.0;
    exact_case(&distances, 0.25, expected)
}

/// The two invalid cardinalities reject complete otherwise ordinary windows.
///
/// # Panics
/// Requires exact count errors for zero and 65 rows.
#[test]
fn empty_and_sixty_five_row_windows_are_rejected() {
    rejected_case(&[], 0.25, WindowVarianceError::CountOutOfRange { count: 0 });
    rejected_case(
        &[1.0; 65],
        0.25,
        WindowVarianceError::CountOutOfRange { count: 65 },
    );
}

/// Cardinality precedes invalid precision and invalid row contents.
///
/// # Panics
/// Requires count errors before any scalar error for both invalid cardinalities.
#[test]
fn count_errors_precede_scalar_validation() {
    rejected_case(&[], 0.0, WindowVarianceError::CountOutOfRange { count: 0 });
    rejected_case(
        &[f64::NAN; 65],
        0.0,
        WindowVarianceError::CountOutOfRange { count: 65 },
    );
}

/// Precision rejects each invalid scalar class without changing healthy rows.
///
/// # Panics
/// Requires the exact precision error for all seven invalid values.
#[test]
fn every_invalid_precision_class_is_rejected() {
    for precision in INVALID_SCALARS {
        rejected_case(
            &[1.0, 1.0, 2.0],
            precision,
            WindowVarianceError::InvalidPrecision,
        );
    }
}

/// Every original row position receives full finite-positive validation.
///
/// # Panics
/// Requires the original distance index for each of seven invalid classes at 64 positions.
#[test]
fn every_invalid_distance_class_is_rejected_at_every_position() {
    for invalid in INVALID_SCALARS {
        for index in 0..64 {
            let mut distances = [1.0; 64];
            distances[index] = invalid;
            rejected_case(
                &distances,
                0.25,
                WindowVarianceError::InvalidDistance { index },
            );
        }
    }
}

/// Invalid precision precedes a distance error in an otherwise legal-size window.
///
/// # Panics
/// Requires precision error even when every supplied distance is invalid.
#[test]
fn precision_error_precedes_distance_validation() {
    rejected_case(&[f64::NAN; 3], -0.0, WindowVarianceError::InvalidPrecision);
}

/// The first invalid original row wins before sorting or arithmetic begins.
///
/// # Panics
/// Requires original index order and complete scalar validation before overflowing work.
#[test]
fn original_distance_index_precedes_later_errors_and_arithmetic() {
    rejected_case(
        &[1.0, f64::NAN, 0.0, f64::INFINITY],
        0.25,
        WindowVarianceError::InvalidDistance { index: 1 },
    );
    rejected_case(
        &[f64::MAX, f64::MAX, 0.0, f64::MAX],
        1.0,
        WindowVarianceError::InvalidDistance { index: 2 },
    );
}

/// Finite positive precision has no normal-only gate or artificial upper bound.
///
/// # Errors
/// Propagates rejection of any healthy precision endpoint.
///
/// # Panics
/// Requires a unit divisor at positive-subnormal, minimum-normal and maximum precision.
#[test]
fn positive_precision_extremes_are_accepted() -> TestResult {
    for precision in [QUANTUM, f64::MIN_POSITIVE, f64::MAX] {
        exact_case(&[1.0], precision, [1.0])?;
    }
    Ok(())
}

/// Finite positive distances include both positive-subnormal and maximum values.
///
/// # Errors
/// Propagates rejection of any healthy distance endpoint.
///
/// # Panics
/// Requires unchanged distance bits and the unit divisor at each endpoint.
#[test]
fn positive_distance_extremes_are_accepted() -> TestResult {
    for distance in [QUANTUM, f64::MIN_POSITIVE, f64::MAX] {
        exact_case(&[distance], QUANTUM, [1.0])?;
    }
    Ok(())
}

/// Subnormal physical-unit inputs still produce exact ordinary dimensionless weights.
///
/// # Errors
/// Propagates unexpected rejection of valid positive subnormals.
///
/// # Panics
/// Requires threshold two quanta and squared scale three squared.
#[test]
fn positive_subnormal_inputs_produce_exact_nonunit_divisors() -> TestResult {
    exact_case(
        &[QUANTUM, QUANTUM, f64::from_bits(4)],
        QUANTUM,
        [1.0, 1.0, 9.0],
    )
}

/// Source-order even averaging rounds 6.5 quanta to even six, before spread work.
///
/// # Errors
/// Propagates unexpected rejection of valid positive subnormals.
///
/// # Panics
/// Requires [1,1,9,64]; a safe-midpoint rewrite instead changes the median to seven.
#[test]
fn subnormal_even_median_preserves_sum_then_half_rounding() -> TestResult {
    exact_case(
        &[
            f64::from_bits(5),
            f64::from_bits(5),
            f64::from_bits(8),
            f64::from_bits(13),
        ],
        QUANTUM,
        [1.0, 1.0, 9.0, 64.0],
    )
}

/// Averaging seven quanta differs from summing individually rounded half-products.
///
/// # Errors
/// Propagates unexpected rejection of valid positive subnormals.
///
/// # Panics
/// Requires the hand-rounded spread three quanta and independently anchored outputs.
#[test]
fn subnormal_even_median_does_not_split_half_products() -> TestResult {
    let distances = [
        f64::from_bits(5),
        f64::from_bits(5),
        f64::from_bits(9),
        f64::from_bits(13),
    ];
    let result = WindowVarianceDivisors::derive(&distances, QUANTUM)?;
    assert_eq!(result.divisors().len(), 4);
    assert_eq!(result.divisors()[0].to_bits(), 1.0_f64.to_bits());
    assert_eq!(result.divisors()[1].to_bits(), 1.0_f64.to_bits());
    assert_eq!(result.divisors()[2].to_bits(), 4.0_f64.to_bits());
    assert!((result.divisors()[3] - 100.0 / 9.0).abs() <= 1.0 / 70_368_744_177_664.0);
    Ok(())
}

/// The source's overflowing even-median sum rejects even when a clamp could hide it.
///
/// # Errors
/// Propagates unexpected rejection of the finite-sum and odd-median controls.
///
/// # Panics
/// Requires the named median-sum failure and preserves all original bits.
#[test]
fn even_median_sum_overflow_is_rejected_with_healthy_controls() -> TestResult {
    exact_case(&[f64::MAX / 2.0; 2], 1.0, [1.0; 2])?;
    exact_case(&[f64::MAX], 1.0, [1.0])?;
    rejected_case(
        &[f64::MAX; 2],
        1.0,
        WindowVarianceError::Arithmetic {
            stage: WindowVarianceStage::MedianSum,
        },
    );
    Ok(())
}

/// Spread division must reject infinity before a later min/max can conceal it.
///
/// # Errors
/// Propagates unexpected rejection of the identical-distance high-precision control.
///
/// # Panics
/// Requires the named spread failure rather than an apparently valid all-unit result.
#[test]
fn spread_overflow_cannot_be_hidden_by_the_clamp() -> TestResult {
    exact_case(&[1.0; 3], f64::MAX, [1.0; 3])?;
    rejected_case(
        &[1.0, f64::MAX, f64::MAX],
        f64::MAX,
        WindowVarianceError::Arithmetic {
            stage: WindowVarianceStage::Spread,
        },
    );
    Ok(())
}

/// Threshold addition rejects overflow independently of a valid singleton median.
///
/// # Errors
/// Propagates unexpected rejection of the finite-threshold neighbor.
///
/// # Panics
/// Requires the threshold stage instead of hiding infinity behind comparison.
#[test]
fn threshold_overflow_is_rejected_with_a_finite_neighbor() -> TestResult {
    exact_case(&[f64::MAX / 2.0], f64::MAX / 2.0, [1.0])?;
    rejected_case(
        &[f64::MAX],
        f64::MAX,
        WindowVarianceError::Arithmetic {
            stage: WindowVarianceStage::Threshold,
        },
    );
    Ok(())
}

/// A positive subnormal denominator can overflow a later ratio despite valid fields.
///
/// # Errors
/// Propagates unexpected rejection of the small finite-ratio control.
///
/// # Panics
/// Requires the ratio stage with unchanged caller storage.
#[test]
fn ratio_overflow_is_rejected_with_a_subnormal_healthy_control() -> TestResult {
    exact_case(
        &[QUANTUM, QUANTUM, f64::from_bits(4)],
        QUANTUM,
        [1.0, 1.0, 9.0],
    )?;
    rejected_case(
        &[QUANTUM, QUANTUM, 1.0],
        QUANTUM,
        WindowVarianceError::Arithmetic {
            stage: WindowVarianceStage::Ratio,
        },
    );
    Ok(())
}

/// The square can overflow after its ratio and increment both remain finite.
///
/// # Errors
/// Propagates unexpected rejection of the finite squared-scale control.
///
/// # Panics
/// Requires the square stage at distance 2^512 and divisor 2^1022 at 2^511.
#[test]
fn square_overflow_is_rejected_with_a_finite_large_neighbor() -> TestResult {
    exact_case(
        &[1.0, 1.0, f64::from_bits(0x5fe0_0000_0000_0000)],
        1.0,
        [1.0, 1.0, f64::from_bits(0x7fd0_0000_0000_0000)],
    )?;
    rejected_case(
        &[1.0, 1.0, f64::from_bits(0x5ff0_0000_0000_0000)],
        1.0,
        WindowVarianceError::Arithmetic {
            stage: WindowVarianceStage::Square,
        },
    );
    Ok(())
}

/// A large valid divisor can have exactly the smallest positive normal reciprocal.
///
/// # Errors
/// Propagates unexpected rejection of the finite positive output.
///
/// # Panics
/// Requires divisor 2^1022 and reciprocal 2^-1022 with exact bits.
#[test]
fn minimum_normal_reciprocal_boundary_is_accepted() -> TestResult {
    let distances = [1.0, 1.0, f64::from_bits(0x5fe0_0000_0000_0000)];
    exact_case(
        &distances,
        1.0,
        [1.0, 1.0, f64::from_bits(0x7fd0_0000_0000_0000)],
    )?;
    let result = WindowVarianceDivisors::derive(&distances, 1.0)?;
    assert_eq!(
        (1.0 / result.divisors()[2]).to_bits(),
        f64::MIN_POSITIVE.to_bits()
    );
    Ok(())
}

/// A finite output is legal even when its inverse-divisor weight is subnormal.
///
/// # Errors
/// Propagates unexpected rejection of a divisor above the reciprocal-normal range.
///
/// # Panics
/// Requires divisor 9*2^1020 and a positive subnormal reciprocal without a new gate.
#[test]
fn positive_subnormal_reciprocals_do_not_narrow_output_admission() -> TestResult {
    let distances = [1.0, 1.0, f64::from_bits(0x5fe8_0000_0000_0000)];
    exact_case(
        &distances,
        1.0,
        [1.0, 1.0, f64::from_bits(0x7fe2_0000_0000_0000)],
    )?;
    let result = WindowVarianceDivisors::derive(&distances, 1.0)?;
    assert!((1.0 / result.divisors()[2]).is_subnormal());
    assert!(1.0 / result.divisors()[2] > 0.0);
    Ok(())
}

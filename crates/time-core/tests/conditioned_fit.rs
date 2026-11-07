// SPDX-License-Identifier: GPL-2.0-only
//! Independent N2a suffix, affine-fit and enclosure acceptance facts.
//!
//! Exact rational targets and prescribed rounded point bits were frozen before
//! candidate observation in `n2-oracles-v1/{exact_oracles,recipe_controls}.py`.
//! Neither oracle imports production code. Pairwise and three-row determinant
//! identities establish mathematical truth independently of the five fit passes.
//!
//! Each target's adjacent binary64 floor/ceiling is proved by integer arithmetic.
//! Since returned endpoints are binary64, `lower <= floor(target)` and
//! `upper >= ceiling(target)` are exactly equivalent to rational containment.
//! Given separately asserted point bits, an error at least the upward-rounded
//! exact point-to-target distance proves the promised absolute-error bound.
//! No fitted tolerance, dependency, physical-accuracy or native claim is involved.
#![forbid(unsafe_code)]

use std::{error::Error, num::NonZeroU64, ptr};

use time_core::{
    ConditionedBatchOutcome, ConditionedFit, ConditionedFitError, ConditionedFitIssue,
    ConditionedFitOperation, ConditionedFitStage, ConditionedNormalizedEstimate,
    ConditionedPhaseSeconds, ConditionedRateRatio, ConditionedSuffix, ConditionedSuffixOutcome,
    ConditionedWeightedSecondsSquared, DeclaredObservationContext, ExactObservationRow,
    PreparedConditionedBatch, fit_conditioned_suffix, prepare_conditioned_batch,
    prepare_conditioned_suffix,
};

/// Propagate real product failures separately from assertion/fixture failures.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;

/// Exact original corrected-phase denominator; no floating absolute conversion.
const DENOMINATOR: i128 = 8_589_934_592_000_000_000;
/// Exact original numerator for one millisecond.
const MILLISECOND: i128 = DENOMINATOR / 1000;
/// Greatest finite quality divisor reachable through the retained rounded square.
const LARGE_DIVISOR_BITS: u64 = 0x7fef_ffff_ffff_fffe;
/// Original distance producing that square with minimum and precision one.
const LARGE_DISTANCE: f64 = f64::from_bits(0x5fef_ffff_ffff_ffff);
/// Exact original newest seconds forcing full-population time exponent 35.
const GEOMETRY_NEWEST_SECONDS: u64 = (1_u64 << 34) + (1_u64 << 13);

/// Frozen scalar truth: point execution plus exact rational comparisons.
#[derive(Clone, Copy)]
struct ScalarFact {
    /// Independently rounded five-pass/lifting result bits.
    point: u64,
    /// Greatest binary64 value no larger than the exact rational target.
    target_floor: u64,
    /// Least binary64 value no smaller than the exact rational target.
    target_ceiling: u64,
    /// Least binary64 error enclosing the exact point-to-target distance.
    error_ceiling: u64,
}

/// Retain independently derived literal facts without product arithmetic.
const fn fact(
    point: u64,
    target_floor: u64,
    target_ceiling: u64,
    error_ceiling: u64,
) -> ScalarFact {
    ScalarFact {
        point,
        target_floor,
        target_ceiling,
        error_ceiling,
    }
}

/// Supply legal declarations without asserting actual source or clock authority.
const fn context() -> DeclaredObservationContext {
    DeclaredObservationContext::new(
        NonZeroU64::MIN,
        NonZeroU64::MIN,
        NonZeroU64::MIN,
        NonZeroU64::MIN,
        NonZeroU64::MIN,
    )
}

/// Construct actual checked rows from independently selected raw references.
/// # Errors
/// Propagates row rejection or an invalid fixture index/key conversion.
/// # Panics
/// Requires equal input lengths for this bounded fixture factory.
fn rows(
    references: &[(u64, bool)],
    phases: &[i128],
    distances: &[f64],
) -> TestResult<Vec<ExactObservationRow>> {
    assert_eq!(references.len(), phases.len());
    assert_eq!(references.len(), distances.len());
    references
        .iter()
        .zip(phases)
        .zip(distances)
        .enumerate()
        .map(|(index, ((reference, phase), distance))| {
            let key = NonZeroU64::new(u64::try_from(index)? + 1).ok_or("fixture key")?;
            Ok(ExactObservationRow::new(
                context(),
                key,
                reference.0,
                reference.1,
                *phase,
                *distance,
            )?)
        })
        .collect()
}

/// Construct whole-second reference fixtures through the checked raw row boundary.
/// # Errors
/// Rejects a seconds-to-nanoseconds fixture overflow or propagates row admission.
fn seconds_rows(
    seconds: &[u64],
    phases: &[i128],
    distances: &[f64],
) -> TestResult<Vec<ExactObservationRow>> {
    let references: Vec<_> = seconds
        .iter()
        .map(|value| {
            value
                .checked_mul(1_000_000_000)
                .map(|whole| (whole, false))
                .ok_or("fixture reference overflow")
        })
        .collect::<Result<_, _>>()?;
    rows(&references, phases, distances)
}

/// Select only the closed ready N1 variant, without constructing replacement data.
/// # Errors
/// Propagates preparation failure or rejects unexpectedly insufficient fixtures.
fn prepared(
    rows: &[ExactObservationRow],
    precision: f64,
) -> TestResult<PreparedConditionedBatch<'_>> {
    match prepare_conditioned_batch(context(), rows, precision)? {
        ConditionedBatchOutcome::Prepared(batch) => Ok(batch),
        ConditionedBatchOutcome::Insufficient(_) => Err("expected prepared batch".into()),
    }
}

/// Borrow an actual ready suffix through its only public producer.
/// # Errors
/// Propagates range failure or rejects an unexpectedly short selected suffix.
fn ready<'batch, 'rows>(
    parent: &'batch PreparedConditionedBatch<'rows>,
    start: usize,
) -> TestResult<ConditionedSuffix<'batch, 'rows>> {
    match prepare_conditioned_suffix(parent, start)? {
        ConditionedSuffixOutcome::Ready(suffix) => Ok(suffix),
        ConditionedSuffixOutcome::Insufficient(_) => Err("expected ready suffix".into()),
    }
}

/// Read one normalized scalar through the distinct public normalized type.
fn normalized(value: ConditionedNormalizedEstimate) -> [f64; 4] {
    [
        value.value(),
        value.lower(),
        value.upper(),
        value.absolute_error(),
    ]
}

/// Read the relative phase correction through its seconds-specific type.
fn phase(value: ConditionedPhaseSeconds) -> [f64; 4] {
    [
        value.value(),
        value.lower(),
        value.upper(),
        value.absolute_error(),
    ]
}

/// Read the frequency ratio through its distinct dimensionless type.
fn rate(value: ConditionedRateRatio) -> [f64; 4] {
    [
        value.value(),
        value.lower(),
        value.upper(),
        value.absolute_error(),
    ]
}

/// Read weighted residual units through their distinct seconds-squared type.
fn sse(value: ConditionedWeightedSecondsSquared) -> [f64; 4] {
    [
        value.value(),
        value.lower(),
        value.upper(),
        value.absolute_error(),
    ]
}

/// Observe every published scalar in fixed normalized-then-lifted order.
fn fields(fit: &ConditionedFit<'_, '_, '_>) -> [[f64; 4]; 6] {
    [
        normalized(fit.normalized_intercept()),
        normalized(fit.normalized_rate()),
        normalized(fit.normalized_weighted_sse()),
        phase(fit.phase_correction_seconds()),
        rate(fit.frequency_ratio()),
        sse(fit.weighted_seconds_squared()),
    ]
}

/// Compare actual bits and exact rational containment without epsilon arithmetic.
/// # Panics
/// Requires all four finite fields, point containment and independently proved bounds.
fn assert_scalar(actual: [f64; 4], expected: ScalarFact) {
    let [point, lower, upper, error] = actual;
    assert!(actual.iter().all(|value| value.is_finite()));
    assert!(error >= 0.0);
    assert!(lower <= point && point <= upper);
    assert_eq!(point.to_bits(), expected.point);
    assert!(lower <= f64::from_bits(expected.target_floor));
    assert!(upper >= f64::from_bits(expected.target_ceiling));
    assert!(error >= f64::from_bits(expected.error_ceiling));
}

/// Check the complete published fit against six independent unit-specific facts.
/// # Panics
/// Requires every scalar contract, actual suffix borrow and exact original anchor.
fn assert_fit(fit: &ConditionedFit<'_, '_, '_>, expected: [ScalarFact; 6]) {
    for (actual, truth) in fields(fit).into_iter().zip(expected) {
        assert_scalar(actual, truth);
    }
    assert!(ptr::eq(
        fit.newest_row(),
        fit.suffix().parent().newest_row()
    ));
    assert_eq!(
        fit.exact_phase_anchor_numerator(),
        fit.newest_row().corrected_phase_numerator()
    );
    assert_eq!(
        fit.exact_phase_anchor_denominator(),
        8_589_934_592_000_000_000
    );
}

/// Reflect a nonzero exact coefficient and its adjacent bounds through zero.
/// The frozen separate-operation recipe is odd under response sign reversal;
/// the SSE and positive absolute error stay unchanged.
const fn reflected(expected: ScalarFact) -> ScalarFact {
    const SIGN: u64 = 1_u64 << 63;
    fact(
        expected.point ^ SIGN,
        expected.target_ceiling ^ SIGN,
        expected.target_floor ^ SIGN,
        expected.error_ceiling,
    )
}

/// Record all publicly observable original row bits and declared context.
type RowSnapshot = (DeclaredObservationContext, NonZeroU64, u64, bool, i128, u64);

/// Snapshot caller data so success and late rejection must preserve its bits.
fn snapshot(rows: &[ExactObservationRow]) -> Vec<RowSnapshot> {
    rows.iter()
        .map(|row| {
            (
                row.context(),
                row.observation_key(),
                row.reference_whole_ns(),
                row.reference_half(),
                row.corrected_phase_numerator(),
                row.peer_distance_seconds().to_bits(),
            )
        })
        .collect()
}

/// Three-point nonunit truth includes the retained local delta, not an absolute float.
/// # Errors
/// Propagates any real checked constructor or fit failure.
/// # Panics
/// Requires exact point recipe bits and rational enclosure facts.
#[test]
fn three_point_nonunit_exact_facts_and_lifting() -> TestResult {
    let rows = seconds_rows(
        &[0, 1, 2],
        &[-MILLISECOND, 0, 3 * MILLISECOND],
        &[1.0, 1.0, 1.5],
    )?;
    let before = snapshot(&rows);
    let parent = prepared(&rows, 0.25)?;
    let suffix = ready(&parent, 0)?;
    let fit = fit_conditioned_suffix(&suffix)?;
    assert!(ptr::eq(fit.suffix(), &raw const suffix));
    assert_fit(&fit, THREE_POINT_NONUNIT);
    assert_eq!(snapshot(&rows), before);
    Ok(())
}

/// Negative response/rate is a healthy sign reversal, with the same weighted SSE.
/// # Errors
/// Propagates real preparation/suffix/fit failures.
/// # Panics
/// Requires reflected exact targets and independently frozen odd point behavior.
#[test]
fn negative_phase_and_rate_preserve_weighted_sse() -> TestResult {
    let rows = seconds_rows(
        &[0, 1, 2],
        &[MILLISECOND, 0, -3 * MILLISECOND],
        &[1.0, 1.0, 1.5],
    )?;
    let parent = prepared(&rows, 0.25)?;
    let suffix = ready(&parent, 0)?;
    assert_fit(
        &fit_conditioned_suffix(&suffix)?,
        [
            reflected(THREE_POINT_NONUNIT[0]),
            reflected(THREE_POINT_NONUNIT[1]),
            THREE_POINT_NONUNIT[2],
            reflected(THREE_POINT_NONUNIT[3]),
            reflected(THREE_POINT_NONUNIT[4]),
            THREE_POINT_NONUNIT[5],
        ],
    );
    Ok(())
}

/// Suffix quality must preserve the full-population factors and original scales.
/// # Errors
/// Propagates any real preparation/suffix/fit failure.
/// # Panics
/// Requires original pointer/weight association and the independent nonunit result.
#[test]
fn suffix_preserves_full_population_quality_chart_and_rows() -> TestResult {
    let rows = seconds_rows(
        &[0, 1, 2, 3, 4],
        &[0, 0, -MILLISECOND, 0, 3 * MILLISECOND],
        &[1.0, 1.0, 2.0, 2.0, 3.0],
    )?;
    let before = snapshot(&rows);
    let parent = prepared(&rows, 0.25)?;
    let suffix = ready(&parent, 2)?;
    assert!(ptr::eq(suffix.parent(), &raw const parent));
    assert_eq!(suffix.original_start(), 2);
    assert_eq!(suffix.sample_count(), 3);
    assert_eq!(suffix.parent().time_scale_exponent(), 2);
    assert_eq!(suffix.parent().phase_scale_exponent(), -7);
    for (local, expected) in [49.0_f64 / 16.0, 49.0 / 16.0, 121.0 / 16.0]
        .into_iter()
        .enumerate()
    {
        let point = suffix.point(local).ok_or("retained suffix point")?;
        assert!(ptr::eq(
            point,
            parent.point(local + 2).ok_or("parent point")?
        ));
        assert!(ptr::eq(point.original_row(), &raw const rows[local + 2]));
        assert!(ptr::eq(
            suffix.original_row(local).ok_or("original row")?,
            &raw const rows[local + 2]
        ));
        assert_eq!(point.variance_divisor().to_bits(), expected.to_bits());
    }
    let fit = fit_conditioned_suffix(&suffix)?;
    assert_fit(&fit, SUFFIX_RETAINS_FULL_CHART_AND_DIVISORS);
    assert_eq!(snapshot(&rows), before);
    Ok(())
}

/// True constant phases retain the exact huge anchor and canonical zero corrections.
/// # Errors
/// Propagates any real admission or fit failure.
/// # Panics
/// Requires every computed scalar to be positive zero without absolute phase conversion.
#[test]
fn huge_anchor_constant_phase_keeps_exact_anchor_and_zero_corrections() -> TestResult {
    for anchor in [i128::MIN, 0, i128::MAX] {
        let rows = seconds_rows(&[0, 1, 2], &[anchor; 3], &[1.0, 1.0, 1.5])?;
        let parent = prepared(&rows, 0.25)?;
        let suffix = ready(&parent, 0)?;
        let fit = fit_conditioned_suffix(&suffix)?;
        assert_fit(&fit, HUGE_ANCHOR_CONSTANT);
        assert_eq!(fit.exact_phase_anchor_numerator(), anchor);
        assert!(
            fields(&fit)
                .into_iter()
                .flatten()
                .all(|value| value.to_bits() == 0)
        );
        assert!(!format!("{fit:?}").contains(&anchor.to_string()) || anchor == 0);
    }
    Ok(())
}

/// A nonconstant response can have valid exact cancellation to zero covariance.
/// # Errors
/// Propagates any real preparation or fit failure.
/// # Panics
/// Requires the independent exact line/SSE enclosure and canonical zero rate.
#[test]
fn nonconstant_zero_covariance_is_not_point_underflow() -> TestResult {
    let rows = seconds_rows(&[0, 1, 2], &[0, DENOMINATOR, 0], &[1.0; 3])?;
    let parent = prepared(&rows, 0.25)?;
    let suffix = ready(&parent, 0)?;
    assert_fit(
        &fit_conditioned_suffix(&suffix)?,
        NONCONSTANT_ZERO_COVARIANCE,
    );
    Ok(())
}

/// Maximum population has an independently exact dyadic line and physical rate 1/2.
/// # Errors
/// Propagates checked fixture-index conversions or actual product errors.
/// # Panics
/// Requires all 64 rows and the six exact target/output facts.
#[test]
fn maximum_population_exact_dyadic_line() -> TestResult {
    let seconds: Vec<_> = (0_u64..64).collect();
    let phases: Vec<_> = (0_i128..64)
        .map(|index| (index - 63) * (DENOMINATOR / 2))
        .collect();
    let rows = seconds_rows(&seconds, &phases, &[1.0; 64])?;
    let parent = prepared(&rows, 0.25)?;
    let suffix = ready(&parent, 0)?;
    assert_eq!(suffix.sample_count(), 64);
    assert_fit(&fit_conditioned_suffix(&suffix)?, MAX_64_EXACT_DYADIC_LINE);
    Ok(())
}

/// A positive subnormal reciprocal is admitted while its zero centered product is exact.
/// # Errors
/// Propagates any real preparation/fit failure.
/// # Panics
/// Requires the actual uncapped divisor and the independent healthy exact line.
#[test]
fn one_legal_subnormal_precision_has_a_healthy_line() -> TestResult {
    let rows = seconds_rows(
        &[0, 1, 2],
        &[-2 * DENOMINATOR, -DENOMINATOR, 0],
        &[1.0, LARGE_DISTANCE, 1.0],
    )?;
    let parent = prepared(&rows, 1.0)?;
    assert_eq!(
        parent
            .point(1)
            .ok_or("large-divisor point")?
            .variance_divisor()
            .to_bits(),
        LARGE_DIVISOR_BITS
    );
    let suffix = ready(&parent, 0)?;
    assert_fit(
        &fit_conditioned_suffix(&suffix)?,
        ONE_LEGAL_SUBNORMAL_PRECISION_LINEAR,
    );
    Ok(())
}

/// All suffix weights may be subnormal; original weighted SSE must retain their scale.
/// # Errors
/// Propagates any real preparation/suffix/fit failure.
/// # Panics
/// Requires both the exact line and distinct nonzero SSE common-divisor facts.
#[test]
fn all_large_divisor_suffixes_keep_original_sse_scale() -> TestResult {
    let cases = [
        (
            [0, 0, 0, -MILLISECOND, 0, 3 * MILLISECOND],
            ALL_LARGE_SUFFIX_NONZERO_SSE,
        ),
        (
            [
                -5 * DENOMINATOR,
                -4 * DENOMINATOR,
                -3 * DENOMINATOR,
                -2 * DENOMINATOR,
                -DENOMINATOR,
                0,
            ],
            ALL_LARGE_SUFFIX_EXACT_LINE,
        ),
    ];
    for (phases, expected) in cases {
        let rows = seconds_rows(
            &[0, 1, 2, 3, 4, 5],
            &phases,
            &[
                1.0,
                1.0,
                1.0,
                LARGE_DISTANCE,
                LARGE_DISTANCE,
                LARGE_DISTANCE,
            ],
        )?;
        let before = snapshot(&rows);
        let parent = prepared(&rows, 1.0)?;
        let suffix = ready(&parent, 3)?;
        for local in 0..3 {
            assert_eq!(
                suffix
                    .point(local)
                    .ok_or("large-q point")?
                    .variance_divisor()
                    .to_bits(),
                LARGE_DIVISOR_BITS
            );
        }
        assert_fit(&fit_conditioned_suffix(&suffix)?, expected);
        assert_eq!(snapshot(&rows), before);
    }
    Ok(())
}

/// Adjacent exact half ticks and tiny corrections survive an enormous original anchor.
/// # Errors
/// Propagates real row/preparation/suffix/fit failures.
/// # Panics
/// Requires exact anchor/pointer preservation and independent relative correction facts.
#[test]
fn huge_anchor_adjacent_half_ticks_keep_small_relative_correction() -> TestResult {
    let rows = rows(
        &[
            (u64::MAX - 1, false),
            (u64::MAX - 1, true),
            (u64::MAX, false),
        ],
        &[i128::MAX - 4, i128::MAX - 3, i128::MAX],
        &[1.0, 1.0, 1.5],
    )?;
    let before = snapshot(&rows);
    let parent = prepared(&rows, 0.25)?;
    assert_eq!(parent.time_scale_exponent(), -29);
    assert_eq!(parent.phase_scale_exponent(), -60);
    let suffix = ready(&parent, 0)?;
    let fit = fit_conditioned_suffix(&suffix)?;
    assert_fit(&fit, HUGE_ANCHOR_ADJACENT_HALF_TICKS);
    assert_eq!(fit.exact_phase_anchor_numerator(), i128::MAX);
    assert_eq!(snapshot(&rows), before);
    Ok(())
}

/// N1 conversion can hide exact curvature; N2's interval must preserve that error.
/// # Errors
/// Propagates actual checked constructor and fit failures.
/// # Panics
/// Requires computed zeros to enclose independently positive exact intercept and SSE.
#[test]
fn rounded_perfect_line_keeps_original_rational_curvature_in_error_bounds() -> TestResult {
    let rows = seconds_rows(
        &[0, 1, 2],
        &[-DENOMINATOR, -DENOMINATOR / 2 + 1, 0],
        &[1.0; 3],
    )?;
    let parent = prepared(&rows, 0.25)?;
    assert_eq!(
        parent
            .point(1)
            .ok_or("middle point")?
            .normalized_phase()
            .to_bits(),
        (-0.5_f64).to_bits()
    );
    let suffix = ready(&parent, 0)?;
    let fit = fit_conditioned_suffix(&suffix)?;
    assert_fit(&fit, ROUNDED_LINE_WITH_EXACT_CURVATURE);
    assert!(fit.normalized_intercept().upper() > 0.0);
    assert!(fit.normalized_weighted_sse().upper() > 0.0);
    Ok(())
}

/// Every valid start retains the original parent, including empty/short suffixes.
/// # Errors
/// Propagates actual preparation/suffix admission errors.
/// # Panics
/// Requires exact counts, branch classification, actual row borrows and safe getters.
#[test]
fn suffix_range_short_results_and_all_getter_boundaries() -> TestResult {
    let rows = seconds_rows(&[0, 1, 2, 3, 4], &[0; 5], &[1.0; 5])?;
    let parent = prepared(&rows, 0.25)?;
    for start in 0..=5 {
        let outcome = prepare_conditioned_suffix(&parent, start)?;
        let (actual_parent, actual_start, count) = match &outcome {
            ConditionedSuffixOutcome::Ready(suffix) => {
                assert!(5 - start >= 3);
                for local in 0..suffix.sample_count() {
                    assert!(ptr::eq(
                        suffix.point(local).ok_or("ready point")?,
                        parent.point(start + local).ok_or("parent point")?
                    ));
                    assert!(ptr::eq(
                        suffix.original_row(local).ok_or("ready row")?,
                        &raw const rows[start + local]
                    ));
                }
                for index in [suffix.sample_count(), usize::MAX] {
                    assert!(suffix.point(index).is_none());
                    assert!(suffix.original_row(index).is_none());
                }
                (
                    suffix.parent(),
                    suffix.original_start(),
                    suffix.sample_count(),
                )
            }
            ConditionedSuffixOutcome::Insufficient(suffix) => {
                assert!(5 - start <= 2);
                for local in 0..suffix.sample_count() {
                    assert!(ptr::eq(
                        suffix.point(local).ok_or("short point")?,
                        parent.point(start + local).ok_or("parent point")?
                    ));
                    assert!(ptr::eq(
                        suffix.original_row(local).ok_or("short row")?,
                        &raw const rows[start + local]
                    ));
                }
                for index in [suffix.sample_count(), usize::MAX] {
                    assert!(suffix.point(index).is_none());
                    assert!(suffix.original_row(index).is_none());
                }
                (
                    suffix.parent(),
                    suffix.original_start(),
                    suffix.sample_count(),
                )
            }
        };
        assert!(ptr::eq(actual_parent, &raw const parent));
        assert_eq!(actual_start, start);
        assert_eq!(count, 5 - start);
        assert!(ptr::eq(actual_parent.newest_row(), &raw const rows[4]));
    }
    for start in [6, usize::MAX] {
        assert!(matches!(prepare_conditioned_suffix(&parent, start),
            Err(ConditionedFitError::SuffixRange { start: actual, count: 5 }) if actual == start));
    }
    Ok(())
}

/// Construct the independent tiny-geometry family using actual full-batch quality.
/// # Errors
/// Propagates checked raw row construction errors.
fn geometry_rows(tau: u64) -> TestResult<Vec<ExactObservationRow>> {
    seconds_rows(
        &[
            0,
            1,
            2,
            3,
            GEOMETRY_NEWEST_SECONDS - 2 * tau,
            GEOMETRY_NEWEST_SECONDS - tau,
            GEOMETRY_NEWEST_SECONDS,
        ],
        &[0; 7],
        &[
            1.0,
            1.0,
            1.0,
            1.0,
            LARGE_DISTANCE,
            LARGE_DISTANCE,
            LARGE_DISTANCE,
        ],
    )
}

/// A true arithmetic underflow, unresolved enclosure and healthy tiny neighbor differ.
/// # Errors
/// Propagates real raw/prepared/suffix errors or the healthy neighbor's fit error.
/// # Panics
/// Requires frozen stage/index/cause and no constant shortcut around geometry.
#[test]
fn frozen_tiny_geometry_underflow_unresolved_and_healthy_neighbors() -> TestResult {
    for tau in [512, 1024, 2048] {
        let rows = geometry_rows(tau)?;
        let before = snapshot(&rows);
        let parent = prepared(&rows, 1.0)?;
        assert_eq!(parent.time_scale_exponent(), 35);
        let suffix = ready(&parent, 4)?;
        match tau {
            512 => assert!(matches!(
                fit_conditioned_suffix(&suffix),
                Err(ConditionedFitError::Arithmetic {
                    stage: ConditionedFitStage::Geometry,
                    index: Some(4),
                    operation: ConditionedFitOperation::Multiply,
                    issue: ConditionedFitIssue::PointUnderflow,
                })
            )),
            1024 => assert!(matches!(
                fit_conditioned_suffix(&suffix),
                Err(ConditionedFitError::UnresolvedGeometry)
            )),
            _ => assert_fit(&fit_conditioned_suffix(&suffix)?, HUGE_ANCHOR_CONSTANT),
        }
        assert_eq!(snapshot(&rows), before);
    }
    Ok(())
}

/// A rounded common precision can induce a tiny nonzero centered product underflow.
/// # Errors
/// Propagates actual row/prepared/suffix admission errors.
/// # Panics
/// Requires the independently predicted geometry refusal before any response product.
#[test]
fn nondyadic_subnormal_precision_reports_original_geometry_index() -> TestResult {
    for phases in [[0; 7], [0, 0, 0, 0, 8 * DENOMINATOR, 0, 0]] {
        let rows = seconds_rows(
            &[0, 1, 2, 3, 4, 5, 6],
            &phases,
            &[1.0, 1.0, 1.0, 1.0, 1e154, 1e154, 1e154],
        )?;
        let before = snapshot(&rows);
        let parent = prepared(&rows, 1.0)?;
        let suffix = ready(&parent, 4)?;
        assert!(matches!(
            fit_conditioned_suffix(&suffix),
            Err(ConditionedFitError::Arithmetic {
                stage: ConditionedFitStage::Geometry,
                index: Some(5),
                operation: ConditionedFitOperation::Multiply,
                issue: ConditionedFitIssue::PointUnderflow,
            })
        ));
        assert_eq!(snapshot(&rows), before);
    }
    Ok(())
}

/// A late nonzero SSE lifting underflow must return no partial published fit.
/// # Errors
/// Propagates actual preparation and suffix failures.
/// # Panics
/// Requires the independently frozen late stage and unchanged original caller data.
#[test]
fn late_sse_lifting_underflow_is_atomic() -> TestResult {
    let rows = seconds_rows(
        &[0, 1, 2, 3, 4, 5],
        &[0, 0, 0, -1, 0, 3],
        &[
            1.0,
            1.0,
            1.0,
            LARGE_DISTANCE,
            LARGE_DISTANCE,
            LARGE_DISTANCE,
        ],
    )?;
    let before = snapshot(&rows);
    let parent = prepared(&rows, 1.0)?;
    assert_eq!(parent.phase_scale_exponent(), -60);
    let suffix = ready(&parent, 3)?;
    assert!(matches!(
        fit_conditioned_suffix(&suffix),
        Err(ConditionedFitError::Arithmetic {
            stage: ConditionedFitStage::SseLifting,
            index: None,
            operation: ConditionedFitOperation::Multiply,
            issue: ConditionedFitIssue::PointUnderflow,
        })
    ));
    assert_eq!(snapshot(&rows), before);
    Ok(())
}

// Frozen scalar tables. Six entries are normalized a/b/S followed by lifted a/b/S.
/// Independent exact target brackets and rounded bits: `three_point_nonunit`.
const THREE_POINT_NONUNIT: [ScalarFact; 6] = [
    fact(
        0xbfbd_208a_5a91_2e34,
        0xbfbd_208a_5a91_2e32,
        0xbfbd_208a_5a91_2e31,
        0x3c83_24a2_ee05_ea9d,
    ),
    fact(
        0x3fdb_4e81_b4e8_1b4f,
        0x3fdb_4e81_b4e8_1b4e,
        0x3fdb_4e81_b4e8_1b4f,
        0x3c7f_92c5_f92c_5f93,
    ),
    fact(
        0x3f7d_d37f_5698_c2c2,
        0x3f7d_d37f_5698_c2c2,
        0x3f7d_d37f_5698_c2c3,
        0x3c04_2eea_9f5d_876b,
    ),
    fact(
        0xbf4d_208a_5a91_2e34,
        0xbf4d_208a_5a91_2e32,
        0xbf4d_208a_5a91_2e31,
        0x3c13_24a2_ee05_ea9d,
    ),
    fact(
        0x3f5b_4e81_b4e8_1b4f,
        0x3f5b_4e81_b4e8_1b4e,
        0x3f5b_4e81_b4e8_1b4f,
        0x3bff_92c5_f92c_5f93,
    ),
    fact(
        0x3e9d_d37f_5698_c2c2,
        0x3e9d_d37f_5698_c2c2,
        0x3e9d_d37f_5698_c2c3,
        0x3b24_2eea_9f5d_876b,
    ),
];

/// Independent exact target brackets and rounded bits: `suffix_retains_full_chart_and_divisors`.
const SUFFIX_RETAINS_FULL_CHART_AND_DIVISORS: [ScalarFact; 6] = [
    fact(
        0xbfb5_aa90_e1cb_77e0,
        0xbfb5_aa90_e1cb_77df,
        0xbfb5_aa90_e1cb_77de,
        0x3c76_50bb_d584_9aef,
    ),
    fact(
        0x3fed_8b7f_bf89_d202,
        0x3fed_8b7f_bf89_d201,
        0x3fed_8b7f_bf89_d202,
        0x3c9f_c4e9_0080_ec5c,
    ),
    fact(
        0x3f67_7842_a078_3488,
        0x3f67_7842_a078_3487,
        0x3f67_7842_a078_3488,
        0x3bac_f583_35c0_51d2,
    ),
    fact(
        0xbf45_aa90_e1cb_77e0,
        0xbf45_aa90_e1cb_77df,
        0xbf45_aa90_e1cb_77de,
        0x3c06_50bb_d584_9aef,
    ),
    fact(
        0x3f5d_8b7f_bf89_d202,
        0x3f5d_8b7f_bf89_d201,
        0x3f5d_8b7f_bf89_d202,
        0x3c0f_c4e9_0080_ec5c,
    ),
    fact(
        0x3e87_7842_a078_3488,
        0x3e87_7842_a078_3487,
        0x3e87_7842_a078_3488,
        0x3acc_f583_35c0_51d2,
    ),
];

/// Independent exact target brackets and rounded bits: `nonconstant_zero_covariance`.
const NONCONSTANT_ZERO_COVARIANCE: [ScalarFact; 6] = [
    fact(
        0x3fd5_5555_5555_5555,
        0x3fd5_5555_5555_5555,
        0x3fd5_5555_5555_5556,
        0x3c75_5555_5555_5556,
    ),
    fact(
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
    fact(
        0x3fe5_5555_5555_5556,
        0x3fe5_5555_5555_5555,
        0x3fe5_5555_5555_5556,
        0x3c95_5555_5555_5556,
    ),
    fact(
        0x3fd5_5555_5555_5555,
        0x3fd5_5555_5555_5555,
        0x3fd5_5555_5555_5556,
        0x3c75_5555_5555_5556,
    ),
    fact(
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
    fact(
        0x3fe5_5555_5555_5556,
        0x3fe5_5555_5555_5555,
        0x3fe5_5555_5555_5556,
        0x3c95_5555_5555_5556,
    ),
];

/// Independent exact target brackets and rounded bits: `huge_anchor_constant`.
const HUGE_ANCHOR_CONSTANT: [ScalarFact; 6] = [
    fact(
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
    fact(
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
    fact(
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
    fact(
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
    fact(
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
    fact(
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
];

/// Independent exact target brackets and rounded bits: `max_64_exact_dyadic_line`.
const MAX_64_EXACT_DYADIC_LINE: [ScalarFact; 6] = [
    fact(
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
    fact(
        0x3ff0_0000_0000_0000,
        0x3ff0_0000_0000_0000,
        0x3ff0_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
    fact(
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
    fact(
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
    fact(
        0x3fe0_0000_0000_0000,
        0x3fe0_0000_0000_0000,
        0x3fe0_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
    fact(
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
];

/// Independent exact target brackets and rounded bits: `one_legal_subnormal_precision_linear`.
const ONE_LEGAL_SUBNORMAL_PRECISION_LINEAR: [ScalarFact; 6] = [
    fact(
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
    fact(
        0x3ff0_0000_0000_0000,
        0x3ff0_0000_0000_0000,
        0x3ff0_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
    fact(
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
    fact(
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
    fact(
        0x3ff0_0000_0000_0000,
        0x3ff0_0000_0000_0000,
        0x3ff0_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
    fact(
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
];

/// Independent exact target brackets and rounded bits: `all_large_suffix_nonzero_sse`.
const ALL_LARGE_SUFFIX_NONZERO_SSE: [ScalarFact; 6] = [
    fact(
        0xbfa5_d867_c3ec_e2a8,
        0xbfa5_d867_c3ec_e2a6,
        0xbfa5_d867_c3ec_e2a5,
        0x3c76_5b7a_3284_6ff6,
    ),
    fact(
        0x4000_624d_d2f1_aa00,
        0x4000_624d_d2f1_a9fb,
        0x4000_624d_d2f1_a9fc,
        0x3ce0_624d_d2f1_a9fc,
    ),
    fact(
        0x0000_0b2f_4fc0_7949,
        0x0000_0b2f_4fc0_7949,
        0x0000_0b2f_4fc0_794a,
        0x0000_0000_0000_0001,
    ),
    fact(
        0xbf35_d867_c3ec_e2a8,
        0xbf35_d867_c3ec_e2a6,
        0xbf35_d867_c3ec_e2a5,
        0x3c06_5b7a_3284_6ff6,
    ),
    fact(
        0x3f60_624d_d2f1_aa00,
        0x3f60_624d_d2f1_a9fb,
        0x3f60_624d_d2f1_a9fc,
        0x3c40_624d_d2f1_a9fc,
    ),
    fact(
        0x0000_0000_2cbd_3f02,
        0x0000_0000_2cbd_3f01,
        0x0000_0000_2cbd_3f02,
        0x0000_0000_0000_0001,
    ),
];

/// Independent exact target brackets and rounded bits: `all_large_suffix_exact_line`.
const ALL_LARGE_SUFFIX_EXACT_LINE: [ScalarFact; 6] = [
    fact(
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
    fact(
        0x3ff0_0000_0000_0000,
        0x3ff0_0000_0000_0000,
        0x3ff0_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
    fact(
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
    fact(
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
    fact(
        0x3ff0_0000_0000_0000,
        0x3ff0_0000_0000_0000,
        0x3ff0_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
    fact(
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
];

/// Independent exact target brackets and rounded bits: `huge_anchor_adjacent_half_ticks`.
const HUGE_ANCHOR_ADJACENT_HALF_TICKS: [ScalarFact; 6] = [
    fact(
        0xbfbe_8abf_d59a_6104,
        0xbfbe_8abf_d59a_6109,
        0xbfbe_8abf_d59a_6108,
        0x3c91_69f3_0725_cf16,
    ),
    fact(
        0x3fea_aaaa_aaaa_aaab,
        0x3fea_aaaa_aaaa_aaaa,
        0x3fea_aaaa_aaaa_aaab,
        0x3c85_5555_5555_5556,
    ),
    fact(
        0x3f80_65a8_ba82_0342,
        0x3f80_65a8_ba82_0343,
        0x3f80_65a8_ba82_0344,
        0x3c43_9b13_8cca_fd21,
    ),
    fact(
        0xbbfe_8abf_d59a_6104,
        0xbbfe_8abf_d59a_6109,
        0xbbfe_8abf_d59a_6108,
        0x38d1_69f3_0725_cf16,
    ),
    fact(
        0x3dfa_aaaa_aaaa_aaab,
        0x3dfa_aaaa_aaaa_aaaa,
        0x3dfa_aaaa_aaaa_aaab,
        0x3a95_5555_5555_5556,
    ),
    fact(
        0x3800_65a8_ba82_0342,
        0x3800_65a8_ba82_0343,
        0x3800_65a8_ba82_0344,
        0x34c3_9b13_8cca_fd21,
    ),
];

/// Independent exact target brackets and rounded bits: `rounded_line_with_exact_curvature`.
const ROUNDED_LINE_WITH_EXACT_CURVATURE: [ScalarFact; 6] = [
    fact(
        0x0000_0000_0000_0000,
        0x3be6_e80f_e033_c8c6,
        0x3be6_e80f_e033_c8c7,
        0x3be6_e80f_e033_c8c7,
    ),
    fact(
        0x3ff0_0000_0000_0000,
        0x3ff0_0000_0000_0000,
        0x3ff0_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
    fact(
        0x0000_0000_0000_0000,
        0x3808_987d_17c3_04e4,
        0x3808_987d_17c3_04e5,
        0x3808_987d_17c3_04e5,
    ),
    fact(
        0x0000_0000_0000_0000,
        0x3be6_e80f_e033_c8c6,
        0x3be6_e80f_e033_c8c7,
        0x3be6_e80f_e033_c8c7,
    ),
    fact(
        0x3fe0_0000_0000_0000,
        0x3fe0_0000_0000_0000,
        0x3fe0_0000_0000_0000,
        0x0000_0000_0000_0000,
    ),
    fact(
        0x0000_0000_0000_0000,
        0x3808_987d_17c3_04e4,
        0x3808_987d_17c3_04e5,
        0x3808_987d_17c3_04e5,
    ),
];

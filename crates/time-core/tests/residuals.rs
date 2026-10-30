// SPDX-License-Identifier: GPL-2.0-only
//! Independent external S1.2b report, arithmetic and privacy consumers.
//!
//! Scope: S12B-01..06, the S1.2 CHECK-7/CHECK-25 supplement. The frozen contract,
//! exact normal-equation facts and integer binary64 traces supply expectations;
//! no new implementation body, Rust arithmetic helper or observed output does.
//! The prospective decimal/64-point zero-line clarification has SHA-256
//! 7d860c2b58254f8f9c4b7c311fc8283e0a30a0ce2e6d937a21dba5c5bc1d56c6.
//!
//! A complete profile has nine test scopes, 169 statistics requests, zero direct
//! fit calls and 1,330 actual point-constructor attempts. Statistics requests
//! have a source-owned one-inner-fit upper bound; this file does not fabricate
//! internal-fit telemetry. Parent direct-fit accounting remains separate.
//! All loops/data are bounded, with no retries, native effects or global counts.
//! Each test emits its actual executed prefix, including assertion unwinding.
//! No Rust execution is claimed by authoring these tests.
#![forbid(unsafe_code)]

#[path = "support/residual_facts.rs"]
mod residual_facts;

use std::{
    cell::Cell,
    error::Error,
    io::{self, Write},
    mem,
    num::NonZeroU64,
};

use residual_facts::{EXACT_CASES, Expected, MODELED_CASES};
use time_core::{
    ArithmeticIssue, ArithmeticOperation, ArithmeticStage, FitError, FitInput, FitPoint,
    FixtureResidualStatistics, PointField, PointIssue, ResidualError, ResidualField, ResidualIssue,
    ResidualStep, residual_statistics,
};

/// Harness/setup failures remain distinct from product assertions.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
/// Positive-statistic relative tolerance, exactly 2^-38, with no absolute floor.
const STATISTIC_RELATIVE_TOLERANCE: f64 = 1.0 / 274_877_906_944.0;
/// Retained parent absolute intercept tolerance, 2^-36 seconds.
const PHASE_TOLERANCE: f64 = 1.0 / 68_719_476_736.0;
/// Retained parent absolute rate tolerance, 2^-32 seconds per second.
const RATE_TOLERANCE: f64 = 1.0 / 4_294_967_296.0;
/// Retained parent relative coefficient tolerance, 2^-40.
const COEFFICIENT_RELATIVE_TOLERANCE: f64 = 1.0 / 1_099_511_627_776.0;
/// Prospective named rounded-line SSE allowance, 2^-60 seconds squared.
const ZERO_SSE_ALLOWANCE: f64 = f64::from_bits(0x3c30_0000_0000_0000);
/// Prospective named rounded-line scale/variance allowance, 2^-64 seconds squared.
const ZERO_NORMALIZED_ALLOWANCE: f64 = f64::from_bits(0x3bf0_0000_0000_0000);

/// Per-test public attempts; immutable counters are never shared across tests.
struct StatsProfile {
    /// Actual statistics public-entry attempts, including rejected calls.
    statistics: Cell<u64>,
    /// Actual point-constructor attempts, including rejected calls.
    points: Cell<u64>,
}

impl StatsProfile {
    /// Start zero counts; total, no product calls, allocation or effects.
    const fn new() -> Self {
        Self {
            statistics: Cell::new(0),
            points: Cell::new(0),
        }
    }

    /// Charge before one actual public point construction; no retry or mutation.
    /// # Errors
    /// Returns a harness ceiling error or preserves the exact typed product result.
    /// # Resources
    /// Constant work; the scope ceiling is 65,536 attempts.
    fn point(&self, x: f64, y: f64, divisor: f64) -> TestResult<Result<FitPoint, FitError>> {
        charge(&self.points, 65_536)?;
        Ok(FitPoint::new(x, y, divisor))
    }

    /// Charge before one actual statistics request for an immutable admitted input.
    /// # Errors
    /// Returns a harness ceiling error or preserves the exact typed product result.
    /// # Resources
    /// Constant harness work; this scope permits at most 209 requests.
    fn statistics(
        &self,
        input: &FitInput,
    ) -> TestResult<Result<FixtureResidualStatistics, ResidualError>> {
        charge(&self.statistics, 209)?;
        Ok(residual_statistics(input))
    }
}

impl Drop for StatsProfile {
    /// Emit one assembled bounded record using one `write_all`, also on assertion unwind.
    /// A failed/missing best-effort emission remains incomplete supervisor evidence.
    /// No product calls occur; only the owned scope counts are observed.
    fn drop(&mut self) {
        let record = format!(
            "CORE_STATS_PROFILE_COUNTS stats={} points={}\n",
            self.statistics.get(),
            self.points.get()
        );
        let _emission_result = io::stderr().lock().write_all(record.as_bytes());
    }
}

/// Charge before the counted operation, with no effects when over the ceiling.
/// # Errors
/// Rejects `u64` overflow or a ceiling violation; never performs a product operation.
/// # Resources
/// Constant work/storage; no panic is required for rejection.
fn charge(counter: &Cell<u64>, ceiling: u64) -> TestResult {
    let next = counter
        .get()
        .checked_add(1)
        .ok_or_else(|| io::Error::other("statistics profile overflow"))?;
    if next > ceiling {
        return Err(io::Error::other("statistics profile ceiling").into());
    }
    counter.set(next);
    Ok(())
}

/// Admit an explicit nonzero fixture label without granting trust or native authority.
/// # Errors
/// Rejects zero without a fallback; constant work and no product operation.
fn frame(value: u64) -> TestResult<NonZeroU64> {
    Ok(NonZeroU64::new(value).ok_or_else(|| io::Error::other("zero fixture frame"))?)
}

/// Admit 3..64 raw rows exactly once each, preserving their chronological order.
/// # Errors
/// Rejects invalid helper size, a counter ceiling, or the real point constructor.
/// # Resources
/// At most 64 owned checked points and 64 actual public constructor attempts.
fn admit(profile: &StatsProfile, rows: &[[f64; 3]]) -> TestResult<Vec<FitPoint>> {
    if !(3..=64).contains(&rows.len()) {
        return Err(io::Error::other("unbounded residual fixture").into());
    }
    let mut points = Vec::with_capacity(rows.len());
    for row in rows {
        points.push(profile.point(row[0], row[1], row[2])??);
    }
    Ok(points)
}

/// Observe immutable raw point bits without arithmetic, mutation or product calls.
fn point_bits(point: &FitPoint) -> [u64; 3] {
    [
        point.relative_seconds().to_bits(),
        point.phase_seconds().to_bits(),
        point.variance_divisor().to_bits(),
    ]
}

/// Snapshot at most 64 checked points without changing values or invoking a consumer.
/// # Resources
/// One bounded owned vector; callers supply admitted fixture storage.
fn snapshot(points: &[FitPoint]) -> Vec<[u64; 3]> {
    points.iter().map(point_bits).collect()
}

/// Observe all published numeric report bits without changing the report.
fn report_bits(report: &FixtureResidualStatistics) -> [u64; 5] {
    [
        report.line().phase_seconds().to_bits(),
        report.line().phase_rate_seconds_per_second().to_bits(),
        report.weighted_sse_seconds_squared().to_bits(),
        report.residual_scale_seconds_squared().to_bits(),
        report.normalized_data_variance_seconds_squared().to_bits(),
    ]
}

/// Check fresh report metadata against the same input owner; no additional fit.
/// # Panics
/// Asserts frame, exact checked count and dof, including the contained line frame.
/// # Resources
/// Constant work with no mutation, allocation or product calls.
fn assert_metadata(report: &FixtureResidualStatistics, input: &FitInput) {
    assert_eq!(report.frame(), input.frame());
    assert_eq!(report.line().frame(), input.frame());
    assert_eq!(report.sample_count(), input.sample_count());
    assert_eq!(report.degrees_of_freedom(), input.sample_count() - 2);
}

/// Make one real request and observe immutable caller storage on every return path.
/// # Errors
/// Propagates setup/counter failures while preserving the statistics product result.
/// # Panics
/// Asserts unchanged source bits/frame/count and successful report association.
/// # Resources
/// At most 64 points, bounded snapshots, one input and exactly one statistics request.
fn statistics_rows(
    profile: &StatsProfile,
    rows: &[[f64; 3]],
    identity: u64,
) -> TestResult<Result<FixtureResidualStatistics, ResidualError>> {
    let points = admit(profile, rows)?;
    let before = snapshot(&points);
    let input = FitInput::new(frame(identity)?, &points)?;
    let result = profile.statistics(&input)?;
    assert_eq!(snapshot(&points), before);
    assert_eq!(input.frame(), frame(identity)?);
    assert_eq!(input.sample_count(), rows.len());
    if let Ok(report) = &result {
        assert_metadata(report, &input);
    }
    Ok(result)
}

/// Compare a strictly positive normal expected statistic with a relative-only bound.
/// Total for every `f64` input; zero, negative, subnormal and nonfinite outputs fail.
fn positive_statistic_matches(actual: f64, expected: f64) -> bool {
    actual.is_normal()
        && actual.is_sign_positive()
        && expected.is_normal()
        && expected.is_sign_positive()
        && (actual / expected - 1.0).abs() <= STATISTIC_RELATIVE_TOLERANCE
}

/// Accept only positive-signed normal-or-zero values inside a frozen zero allowance.
/// Total for every `f64` pair; unsupported subnormals cannot pass via a loose envelope.
/// Constant work, no allocation, mutation or product calls.
fn rounded_zero_statistic_matches(actual: f64, allowance: f64) -> bool {
    actual.is_finite() && actual.is_sign_positive() && !actual.is_subnormal() && actual <= allowance
}

/// Compare returned line coefficients against independent parent facts.
/// # Panics
/// Asserts finite coefficients within predeclared parent absolute/relative bounds.
/// # Resources
/// Constant work; no solver call or source-derived expectation.
fn assert_line(report: &FixtureResidualStatistics, phase: f64, rate: f64) {
    let line = report.line();
    let actual_phase = line.phase_seconds();
    let actual_rate = line.phase_rate_seconds_per_second();
    assert!(actual_phase.is_finite());
    assert!(actual_rate.is_finite());
    assert!(
        (actual_phase - phase).abs()
            <= PHASE_TOLERANCE + COEFFICIENT_RELATIVE_TOLERANCE * phase.abs()
    );
    assert!(
        (actual_rate - rate).abs() <= RATE_TOLERANCE + COEFFICIENT_RELATIVE_TOLERANCE * rate.abs()
    );
}

/// Apply frozen scalar expectations, permitting rounded zeros only for named lines.
/// # Panics
/// Rejects false-zero positive results and wrong normalization; exact controls need +0.
/// # Resources
/// Constant work; no new product call and no product arithmetic supplies truth.
fn assert_statistics(report: &FixtureResidualStatistics, expected: &Expected, rounded_zero: bool) {
    for (actual, wanted, allowance) in [
        (
            report.weighted_sse_seconds_squared(),
            expected.weighted_sse_seconds_squared,
            ZERO_SSE_ALLOWANCE,
        ),
        (
            report.residual_scale_seconds_squared(),
            expected.residual_scale_seconds_squared,
            ZERO_NORMALIZED_ALLOWANCE,
        ),
        (
            report.normalized_data_variance_seconds_squared(),
            expected.normalized_data_variance_seconds_squared,
            ZERO_NORMALIZED_ALLOWANCE,
        ),
    ] {
        if wanted.is_sign_positive() && wanted.is_normal() {
            assert!(
                positive_statistic_matches(actual, wanted),
                "positive statistic differs: actual={actual:e}, expected={wanted:e}"
            );
        } else if rounded_zero {
            assert!(rounded_zero_statistic_matches(actual, allowance));
        } else {
            assert_eq!(
                actual.to_bits(),
                0,
                "exact-zero statistic must be positive zero"
            );
        }
    }
}

/// Factory plus stable step context; no Clone/Copy trait is required from the public enum.
type StepCase = (fn() -> ResidualStep, &'static str);
/// Factory plus stable scalar context; each manual error owns its fresh public field.
type FieldCase = (fn() -> ResidualField, &'static str);

/// Construct each public step afresh with its independently named typed context.
/// Total, fixed nine factories; no product calls, mutation or allocation.
fn step_cases() -> [StepCase; 9] {
    [
        (|| ResidualStep::PhaseDifference, "phasedifference"),
        (|| ResidualStep::Trend, "trend"),
        (|| ResidualStep::Residual, "residual"),
        (|| ResidualStep::Square, "square"),
        (|| ResidualStep::Weight, "weight"),
        (|| ResidualStep::Accumulate, "accumulate"),
        (|| ResidualStep::Scale, "scale"),
        (|| ResidualStep::NormalizeMultiply, "normalizemultiply"),
        (|| ResidualStep::NormalizeDivide, "normalizedivide"),
    ]
}

/// Construct each public statistic field afresh with its independent context.
/// Total, fixed three factories; no product calls, mutation or allocation.
fn field_cases() -> [FieldCase; 3] {
    [
        (|| ResidualField::WeightedSse, "weightedsse"),
        (|| ResidualField::ResidualScale, "residualscale"),
        (
            || ResidualField::NormalizedDataVariance,
            "normalizeddatavariance",
        ),
    ]
}

/// Compare unit-only public enum variants without requiring an unadvertised `Eq` trait.
/// Total for same-typed values; no allocation, mutation or product operation.
fn same_variant<T>(actual: &T, expected: &T) -> bool {
    mem::discriminant(actual) == mem::discriminant(expected)
}

/// Retain only ASCII alphanumeric typed-context characters for formatting checks.
/// # Resources
/// One string bounded by the caller's checked <=512-byte display input.
fn compact(value: &str) -> String {
    value
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .flat_map(char::to_lowercase)
        .collect()
}

/// Assert bounded, useful typed formatting without embedding raw fixture values.
/// # Panics
/// Requires all expected context codes and a <=512-byte Display and Debug result.
/// # Resources
/// Bounded strings only; no product request or floating-point formatting oracle.
fn assert_error_context(error: &ResidualError, codes: &[&str]) {
    let display = error.to_string();
    let debug = format!("{error:?}");
    assert!(display.len() <= 512);
    assert!(debug.len() <= 512);
    let normalized = compact(&display);
    for code in codes {
        assert!(
            normalized.contains(code),
            "missing typed error context: {code}"
        );
    }
    for forbidden in [
        "317.25",
        "0.3125",
        "1.23456789",
        "garde::",
        "validation report",
    ] {
        assert!(!display.contains(forbidden));
        assert!(!debug.contains(forbidden));
    }
}

/// S12B-02..04: every named base/sign fact and every admitted common-divisor fact.
/// # Errors
/// Propagates actual admission/statistics failures; a `NotImplemented` stub fails.
/// # Panics
/// Asserts independent line/E/s/v facts, n/dof/frame and positive output >1 controls.
/// # Resources
/// Exactly 48 statistics requests / 632 points in a complete scope; no direct fit.
#[test]
fn all_named_exact_facts_and_admitted_transformations() -> TestResult {
    let profile = StatsProfile::new();
    for case in EXACT_CASES {
        for variant in 0..3 {
            if variant == 2 && !case.scaled_admitted {
                continue; // Retained algebraic fact only: transformed d exceeds 16.
            }
            let rows: Vec<[f64; 3]> = case
                .rows
                .iter()
                .map(|row| match variant {
                    1 => [row[0], -row[1], row[2]],
                    2 => [row[0], row[1], row[2] * 2.0],
                    _ => *row,
                })
                .collect();
            let expected = match variant {
                1 => &case.phase_reversed,
                2 => &case.divisors_times_two,
                _ => &case.base,
            };
            let report = statistics_rows(&profile, &rows, 101)??;
            assert_line(
                &report,
                expected.phase_seconds,
                expected.rate_seconds_per_second,
            );
            let rounded_zero = matches!(case.name, "linear" | "sixty_four_point_exact_line");
            assert_statistics(&report, expected, rounded_zero);
            if case.name == "endpoint_noisy_sixty_four" && variant != 2 {
                assert!(report.weighted_sse_seconds_squared() > 1.0);
                assert!(report.residual_scale_seconds_squared() > 1.0);
                assert!(report.normalized_data_variance_seconds_squared() > 1.0);
            }
        }
    }
    Ok(())
}

/// S12B-05: first-failure producer steps and exact independently modeled result bits.
/// # Errors
/// Propagates only setup/counter errors or an unexpected product error on a healthy row.
/// # Panics
/// Requires exact `Square`/`Weight`/`Scale`/`NormalizeDivide` failures and supported neighbors.
/// # Resources
/// Exactly 11 requests / 277 points; mixed signed zero must produce public +0 squares.
/// No additional fit is used to calculate a coefficient or expectation.
#[test]
fn modeled_primitive_boundaries_and_signed_zero_preserve_exact_bits() -> TestResult {
    let profile = StatsProfile::new();
    for case in MODELED_CASES {
        let rows: Vec<[f64; 3]> = case
            .row_bits
            .iter()
            .map(|row| row.map(f64::from_bits))
            .collect();
        let result = statistics_rows(&profile, &rows, 202)?;
        if let Some(expected) = &case.first_unsupported {
            let (step, source) = match result {
                Err(ResidualError::Arithmetic { step, source }) => (step, source),
                other => {
                    return Err(io::Error::other(format!(
                        "missing modeled first failure {}: {:?}",
                        case.name,
                        other.as_ref().err()
                    ))
                    .into());
                }
            };
            assert!(
                same_variant(&step, &expected.step),
                "wrong producer: {}",
                case.name
            );
            let typed = match source {
                FitError::Arithmetic {
                    stage,
                    operation,
                    issue,
                } => {
                    same_variant(&stage, &ArithmeticStage::Response)
                        && same_variant(&operation, &expected.operation)
                        && same_variant(&issue, &expected.issue)
                }
                _ => false,
            };
            assert!(typed, "lost actual shared arithmetic source: {}", case.name);
        } else {
            let report = result?;
            assert_eq!(report.line().phase_seconds().to_bits(), 0, "{}", case.name);
            assert_eq!(report.line().phase_rate_seconds_per_second().to_bits(), 0);
            assert_eq!(
                Some(report.weighted_sse_seconds_squared().to_bits()),
                case.weighted_sse
            );
            assert_eq!(
                Some(report.residual_scale_seconds_squared().to_bits()),
                case.residual_scale
            );
            assert_eq!(
                Some(report.normalized_data_variance_seconds_squared().to_bits()),
                case.normalized_data_variance
            );
        }
    }
    Ok(())
}

/// S12B-02/04: analytic four-point families have independently orthogonal residuals.
/// For x=(-3,-2,-1,0), r=(1,-1,-1,1), sum(r)=sum(x*r)=0.
/// Therefore y=A+B*x+c*r has coefficients A/B, E=4*c*c/d, s=2*c*c/d,
/// and v=2*c*c. All A/B/c/d and their admitted transforms are dyadic.
/// # Errors
/// Propagates checked admission or actual statistics errors.
/// # Panics
/// Asserts independent coefficients and distinct positive E/s/v for all transforms.
/// # Resources
/// 32 bounded sets * 3 transforms = exactly 96 requests / 384 points; no direct fit.
#[test]
fn bounded_32_analytic_sets_have_96_independent_transform_controls() -> TestResult {
    let profile = StatsProfile::new();
    for phase in [-1.0 / 8.0, 0.0, 1.0 / 8.0, 1.0 / 4.0] {
        for rate in [-1.0 / 32.0, 0.0, 1.0 / 64.0, 1.0 / 32.0] {
            for noise in [1.0 / 128.0, 1.0 / 64.0] {
                for (sign, divisor) in [(1.0, 1.0), (-1.0, 1.0), (1.0, 2.0)] {
                    let mut rows = [[0.0; 3]; 4];
                    for (row, (x, residual)) in
                        rows.iter_mut()
                            .zip([(-3.0, 1.0), (-2.0, -1.0), (-1.0, -1.0), (0.0, 1.0)])
                    {
                        *row = [x, sign * (phase + rate * x + noise * residual), divisor];
                    }
                    let expected = Expected {
                        phase_seconds: sign * phase,
                        rate_seconds_per_second: sign * rate,
                        weighted_sse_seconds_squared: 4.0 * noise * noise / divisor,
                        residual_scale_seconds_squared: 2.0 * noise * noise / divisor,
                        normalized_data_variance_seconds_squared: 2.0 * noise * noise,
                    };
                    let report = statistics_rows(&profile, &rows, 303)??;
                    assert_line(
                        &report,
                        expected.phase_seconds,
                        expected.rate_seconds_per_second,
                    );
                    assert_statistics(&report, &expected, false);
                }
            }
        }
    }
    Ok(())
}

/// S12B-04: constant normal extrema and signed zeros retain exact parent phase bits.
/// # Errors
/// Propagates real setup/statistics failures; no rejection can replace a positive control.
/// # Panics
/// Asserts exact first-phase bits, +0 rate and +0 E/s/v, including `MIN_POSITIVE`.
/// # Resources
/// Six requests / 18 points; the inherited constant shortcut may not be bypassed.
#[test]
fn constant_extrema_and_signed_zeros_have_exact_positive_zero_statistics() -> TestResult {
    let profile = StatsProfile::new();
    for phase in [f64::MIN_POSITIVE, -f64::MIN_POSITIVE, 0.0, -0.0, 1.0, -1.0] {
        let rows = [[-2.0, phase, 1.0], [-1.0, phase, 16.0], [0.0, phase, 16.0]];
        let report = statistics_rows(&profile, &rows, 404)??;
        assert_eq!(report.line().phase_seconds().to_bits(), phase.to_bits());
        assert_eq!(report.line().phase_rate_seconds_per_second().to_bits(), 0);
        assert_eq!(report.weighted_sse_seconds_squared().to_bits(), 0);
        assert_eq!(report.residual_scale_seconds_squared().to_bits(), 0);
        assert_eq!(
            report.normalized_data_variance_seconds_squared().to_bits(),
            0
        );
    }
    Ok(())
}

/// S12B-01: equal frames do not establish input association; caller storage is copied.
/// # Errors
/// Propagates actual constructors/statistics failures without a fallback report.
/// # Panics
/// Requires fresh 4/3 versus 7/6 line/SSE association and preserves held/owned reports.
/// # Resources
/// Three requests / four point attempts; no independently supplied line or direct fit.
#[test]
fn same_frame_changed_data_and_owned_input_have_fresh_associated_reports() -> TestResult {
    let profile = StatsProfile::new();
    let mut points = admit(
        &profile,
        &[[-2.0, -1.0, 1.0], [-1.0, 1.0, 1.0], [0.0, 1.0, 1.0]],
    )?;
    let first_input = FitInput::new(frame(u64::MAX)?, &points)?;
    let first = profile.statistics(&first_input)??;
    assert_metadata(&first, &first_input);
    let first_bits = report_bits(&first);
    *points
        .first_mut()
        .ok_or_else(|| io::Error::other("missing owned point"))? =
        profile.point(-2.0, 0.0, 1.0)??;
    let second_input = FitInput::new(frame(u64::MAX)?, &points)?;
    let before = snapshot(&points);
    let second = profile.statistics(&second_input)??;
    let repeated = profile.statistics(&first_input)??;
    assert_metadata(&second, &second_input);
    assert_metadata(&repeated, &first_input);
    assert_line(&first, 4.0 / 3.0, 1.0);
    assert_line(&second, 7.0 / 6.0, 1.0 / 2.0);
    assert!(positive_statistic_matches(
        first.weighted_sse_seconds_squared(),
        2.0 / 3.0
    ));
    assert!(positive_statistic_matches(
        second.weighted_sse_seconds_squared(),
        1.0 / 6.0
    ));
    assert_eq!(report_bits(&first), first_bits);
    assert_eq!(report_bits(&repeated), first_bits);
    assert_eq!(snapshot(&points), before);
    Ok(())
}

/// S12B-01/06: inherited fit failures keep their actual typed source before statistics.
/// # Errors
/// Propagates setup/counter failures and unexpected healthy-neighbor product failures.
/// # Panics
/// Distinguishes inherited `Subnormal`/`ZeroUnderflow` from residual-context errors.
/// # Resources
/// Two failure requests and two healthy neighbors: four requests / 12 points.
#[test]
fn parent_fit_errors_forward_before_residuals_with_healthy_neighbors() -> TestResult {
    let profile = StatsProfile::new();
    for (rows, issue) in [
        (
            [
                [-2.0, 0.0, 1.0],
                [-1.0, f64::MIN_POSITIVE, 16.0],
                [0.0, 0.0, 16.0],
            ],
            ArithmeticIssue::Subnormal,
        ),
        (
            [
                [-2.0, -0.5, 1.0],
                [
                    f64::from_bits(0xbfef_ffff_ffff_ffff),
                    f64::MIN_POSITIVE,
                    1.0,
                ],
                [0.0, 0.5, 1.0],
            ],
            ArithmeticIssue::ZeroUnderflow,
        ),
    ] {
        let result = statistics_rows(&profile, &rows, 505)?;
        let matches = match &result {
            Err(ResidualError::Fit(FitError::Arithmetic {
                stage: ArithmeticStage::Response,
                operation: ArithmeticOperation::Multiply,
                issue: actual,
            })) => same_variant(actual, &issue),
            _ => false,
        };
        assert!(
            matches,
            "inherited typed fit failure must precede residual work"
        );
        if let Err(error) = result {
            let source = error
                .source()
                .ok_or_else(|| io::Error::other("missing fit source"))?;
            assert!(source.downcast_ref::<FitError>().is_some());
        }
        let healthy = [
            [rows[0][0], 0.0, rows[0][2]],
            [rows[1][0], 0.0, rows[1][2]],
            [rows[2][0], 0.0, rows[2][2]],
        ];
        let report = statistics_rows(&profile, &healthy, 505)??;
        assert_eq!(report.weighted_sse_seconds_squared().to_bits(), 0);
    }
    Ok(())
}

/// S12B-06: report Debug permits only type/frame/count/dof, never coordinates/results.
/// # Errors
/// Propagates actual setup/statistics failures.
/// # Panics
/// Rejects any unapproved token, raw numeric output or report larger than 512 logical bytes.
/// # Resources
/// One request / three points; one <=256-byte successful Debug representation.
#[test]
fn report_debug_has_only_permitted_metadata_and_bounded_owned_size() -> TestResult {
    let profile = StatsProfile::new();
    let report = statistics_rows(
        &profile,
        &[[-2.0, 0.3125, 1.0], [-1.0, -0.125, 1.0], [0.0, 0.625, 1.0]],
        991,
    )??;
    assert!(size_of::<FixtureResidualStatistics>() <= 512);
    let debug = format!("{report:?}");
    assert!(debug.len() <= 256);
    for token in debug
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
        .filter(|token| !token.is_empty())
    {
        let token = token.to_ascii_lowercase();
        assert!(
            matches!(
                token.as_str(),
                "fixtureresidualstatistics"
                    | "frame"
                    | "sample_count"
                    | "count"
                    | "degrees_of_freedom"
                    | "dof"
                    | "nonzerou64"
                    | "991"
                    | "3"
                    | "1"
            ),
            "non-metadata report Debug token"
        );
    }
    Ok(())
}

/// S12B-06: every public error kind remains total and typed under manual construction.
/// # Panics
/// Requires bounded context, direct preserved `FitError` sources, and no fabricated sources.
/// # Resources
/// Zero product calls/points; finite 576 arithmetic + 9 atypical arithmetic + 13 scalar/count + 1 fit wrapper cases.
#[test]
fn manually_constructed_errors_preserve_total_bounded_context_and_sources() {
    let _profile = StatsProfile::new();
    for (make_step, step_code) in step_cases() {
        for (stage, stage_code) in [
            (ArithmeticStage::Geometry, "geometry"),
            (ArithmeticStage::Response, "response"),
            (ArithmeticStage::Coefficients, "coefficients"),
            (ArithmeticStage::Result, "result"),
        ] {
            for (operation, operation_code) in [
                (ArithmeticOperation::Add, "add"),
                (ArithmeticOperation::Subtract, "subtract"),
                (ArithmeticOperation::Multiply, "multiply"),
                (ArithmeticOperation::Divide, "divide"),
            ] {
                for (issue, issue_code) in [
                    (ArithmeticIssue::NonFinite, "nonfinite"),
                    (ArithmeticIssue::Subnormal, "subnormal"),
                    (ArithmeticIssue::ZeroUnderflow, "zerounderflow"),
                    (
                        ArithmeticIssue::NonPositiveDenominator,
                        "nonpositivedenominator",
                    ),
                ] {
                    let error = ResidualError::Arithmetic {
                        step: make_step(),
                        source: FitError::Arithmetic {
                            stage,
                            operation,
                            issue,
                        },
                    };
                    assert_error_context(
                        &error,
                        &[step_code, stage_code, operation_code, issue_code],
                    );
                    let source = error.source();
                    assert!(
                        source
                            .and_then(|source| source.downcast_ref::<FitError>())
                            .is_some()
                    );
                }
            }
        }
        // Public manual variants must remain total even for a non-arithmetic source.
        let error = ResidualError::Arithmetic {
            step: make_step(),
            source: FitError::CountOutOfRange,
        };
        assert_error_context(&error, &[step_code, "count"]);
        assert!(matches!(
            error
                .source()
                .and_then(|source| source.downcast_ref::<FitError>()),
            Some(FitError::CountOutOfRange)
        ));
    }
    for (make_field, field_code) in field_cases() {
        for (issue, issue_code) in [
            (ResidualIssue::NonFinite, "nonfinite"),
            (ResidualIssue::Subnormal, "subnormal"),
            (ResidualIssue::Negative, "negative"),
        ] {
            let error = ResidualError::InvalidStatistic {
                field: make_field(),
                issue,
            };
            assert_error_context(&error, &[field_code, issue_code]);
            assert!(error.source().is_none());
        }
        let error = ResidualError::ValidationInvariant {
            field: make_field(),
        };
        assert_error_context(&error, &[field_code, "validation", "invariant"]);
        assert!(error.source().is_none());
    }
    let error = ResidualError::CountInvariant;
    assert_error_context(&error, &["count", "invariant"]);
    assert!(error.source().is_none());
    let wrapped = ResidualError::Fit(FitError::Point {
        field: PointField::PhaseSeconds,
        issue: PointIssue::Subnormal,
    });
    assert_error_context(&wrapped, &["phase", "subnormal"]);
    assert!(matches!(
        wrapped
            .source()
            .and_then(|source| source.downcast_ref::<FitError>()),
        Some(FitError::Point {
            field: PointField::PhaseSeconds,
            issue: PointIssue::Subnormal
        })
    ));
}

/// Oracle health: positive tiny facts and rounded zeros retain their scalar domain.
/// # Panics
/// Accepts healthy normal/zero controls and rejects subnormal, negative, NaN and defects.
/// # Resources
/// Zero product calls/points; bounded pure comparison controls.
#[test]
fn positive_relative_oracle_rejects_false_zero_and_changed_values() {
    let _profile = StatsProfile::new();
    for expected in [1.0 / 3.0, f64::from_bits(0x0170_0000_0000_0000)] {
        assert!(positive_statistic_matches(expected, expected));
        assert!(!positive_statistic_matches(0.0, expected));
        assert!(!positive_statistic_matches(-expected, expected));
        assert!(!positive_statistic_matches(f64::NAN, expected));
        assert!(!positive_statistic_matches(expected * 1.25, expected));
    }
    for allowance in [ZERO_SSE_ALLOWANCE, ZERO_NORMALIZED_ALLOWANCE] {
        assert!(rounded_zero_statistic_matches(0.0, allowance));
        assert!(rounded_zero_statistic_matches(f64::MIN_POSITIVE, allowance));
        assert!(rounded_zero_statistic_matches(allowance, allowance));
        assert!(!rounded_zero_statistic_matches(
            f64::from_bits(1),
            allowance
        ));
        assert!(!rounded_zero_statistic_matches(
            f64::from_bits(0x000f_ffff_ffff_ffff),
            allowance
        ));
        assert!(!rounded_zero_statistic_matches(allowance * 2.0, allowance));
        assert!(!rounded_zero_statistic_matches(f64::NAN, allowance));
    }
}

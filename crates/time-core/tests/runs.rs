// SPDX-License-Identifier: GPL-2.0-only
//! Independent external consumers of the provisional S1.2 fixture-runs API.
//!
//! Expectations use the separately reviewed source-only runs contract and
//! binary64 analysis, not the implementation's own table or fit as an oracle.
//! The accepted numerical envelopes are conditional on the documented
//! separately rounded operation order and target semantics; they do not prove
//! successful computation or native/product qualification.
//!
//! The bounded profile has nine test scopes, 18 public selector attempts,
//! 18 input-constructor attempts and 159 point-constructor attempts if every
//! path reaches completion. This excludes source-owned internal suffix work.
//! Prefix counts emit even during assertion unwinding.
#![forbid(unsafe_code)]

use std::{
    cell::Cell,
    error::Error,
    io::{self, Write},
    num::NonZeroU64,
};

use time_core::{
    ArithmeticIssue, ArithmeticOperation, ArithmeticStage, FitError, FitInput, FitPoint,
    FixtureRunsError, FixtureRunsSelection, FixtureRunsStage, FixtureRunsStop, ResidualError,
    ResidualStep, select_fixture_runs,
};

/// Fixture setup and real product errors remain distinguishable from assertions.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;

/// Conditional absolute coefficient bound, in seconds or seconds/second.
const COEFFICIENT_TOLERANCE: f64 = 3e-9;
/// Conditional absolute weighted SSE bound, in seconds squared.
const SSE_TOLERANCE: f64 = 4e-6;
/// Conditional absolute residual-scale bound, in seconds squared.
const SCALE_TOLERANCE: f64 = 5e-6;
/// Conditional absolute normalized-variance bound, in seconds squared.
const VARIANCE_TOLERANCE: f64 = 3e-5;

/// One scope's actual public-entry attempts, including unfinished prefixes.
struct Profile {
    /// Attempted checked point constructions.
    points: Cell<usize>,
    /// Attempted checked batch constructions.
    inputs: Cell<usize>,
    /// Attempted fixture selector entries.
    selections: Cell<usize>,
}

impl Profile {
    /// Start one local profile without invoking a product owner.
    const fn new() -> Self {
        Self {
            points: Cell::new(0),
            inputs: Cell::new(0),
            selections: Cell::new(0),
        }
    }

    /// Charge the next bounded point attempt before its public constructor.
    /// # Errors
    /// Preserves count ceiling and actual checked-construction errors.
    fn point(&self, x: f64, y: f64, divisor: f64) -> TestResult<FitPoint> {
        charge(&self.points, 128)?;
        Ok(FitPoint::new(x, y, divisor)?)
    }

    /// Charge one bounded batch admission before the actual public call.
    /// # Errors
    /// Preserves count ceiling and actual checked-batch errors.
    fn input(&self, frame: NonZeroU64, points: &[FitPoint]) -> TestResult<FitInput> {
        charge(&self.inputs, 8)?;
        Ok(FitInput::new(frame, points)?)
    }

    /// Charge one bounded selector call before reaching the actual public entry.
    /// # Errors
    /// Preserves count ceiling and the actual selector error.
    fn select(
        &self,
        input: &FitInput,
        minimum: usize,
    ) -> TestResult<Result<FixtureRunsSelection, FixtureRunsError>> {
        charge(&self.selections, 8)?;
        Ok(select_fixture_runs(input, minimum))
    }
}

impl Drop for Profile {
    /// Report this scope's actual prefix without invoking any product owner.
    fn drop(&mut self) {
        let record = format!(
            "CORE_RUNS_PROFILE points={} inputs={} selections={}\n",
            self.points.get(),
            self.inputs.get(),
            self.selections.get()
        );
        let _emission_result = io::stderr().lock().write_all(record.as_bytes());
    }
}

/// Update an owned local counter before work, rejecting overflow or ceiling.
/// # Errors
/// An invalid next count prevents the associated public operation.
fn charge(counter: &Cell<usize>, ceiling: usize) -> TestResult {
    let next = counter
        .get()
        .checked_add(1)
        .ok_or_else(|| io::Error::other("runs profile overflow"))?;
    if next > ceiling {
        return Err(io::Error::other("runs profile ceiling").into());
    }
    counter.set(next);
    Ok(())
}

/// Retain immutable point bits, including the sign of zero.
fn point_bits(point: &FitPoint) -> [u64; 3] {
    [
        point.relative_seconds().to_bits(),
        point.phase_seconds().to_bits(),
        point.variance_divisor().to_bits(),
    ]
}

/// Build a fresh checked fixture and preserve observable input on either outcome.
/// # Errors
/// Propagates fixture-size, point, batch, or actual selection failures.
/// # Panics
/// Asserts the caller's point bits and input frame/count remain unchanged.
fn run_result(
    profile: &Profile,
    rows: &[[f64; 3]],
    minimum: usize,
) -> TestResult<Result<FixtureRunsSelection, FixtureRunsError>> {
    if !(3..=64).contains(&rows.len()) {
        return Err(io::Error::other("runs fixture size").into());
    }
    let frame = NonZeroU64::new(17).ok_or_else(|| io::Error::other("runs frame"))?;
    let mut points = Vec::with_capacity(rows.len());
    for [x, y, divisor] in rows {
        points.push(profile.point(*x, *y, *divisor)?);
    }
    let original: Vec<[u64; 3]> = points.iter().map(point_bits).collect();
    let input = profile.input(frame, &points)?;
    assert_eq!(input.frame(), frame);
    assert_eq!(input.sample_count(), rows.len());
    let selected = profile.select(&input, minimum)?;
    assert_eq!(input.frame(), frame);
    assert_eq!(input.sample_count(), rows.len());
    assert_eq!(points.iter().map(point_bits).collect::<Vec<_>>(), original);
    Ok(selected)
}

/// Return the actual successful selection from one explicitly charged run.
/// # Errors
/// Preserves fixture setup and actual public selection failures.
fn run(profile: &Profile, rows: &[[f64; 3]], minimum: usize) -> TestResult<FixtureRunsSelection> {
    Ok(run_result(profile, rows, minimum)??)
}

/// Independently declared result coordinates and exact-real numerical facts.
struct Expected {
    /// Selected original-array start.
    start: usize,
    /// Number of points in the selected suffix.
    count: usize,
    /// Start of the extended decision window.
    decision_start: usize,
    /// Number of points in the decision window.
    decision_count: usize,
    /// Expected decision-window runs.
    decision_runs: usize,
    /// Expected selected-suffix runs.
    fit_runs: usize,
    /// Whether the strict runs test passed before minimum fallback.
    passed: bool,
    /// Exact-real phase at relative zero, in seconds.
    phase: f64,
    /// Exact-real phase rate, in seconds/second.
    rate: f64,
    /// Exact-real weighted SSE, in seconds squared.
    sse: f64,
    /// Exact-real residual scale, in seconds squared.
    scale: f64,
    /// Exact-real normalized variance, in seconds squared.
    variance: f64,
}

/// Compare a finite scalar with a separately frozen conditional absolute bound.
/// # Panics
/// Rejects a nonfinite output or a value outside the declared bound.
fn near(actual: f64, expected: f64, tolerance: f64) {
    assert!(actual.is_finite());
    assert!((actual - expected).abs() <= tolerance);
}

/// Check the actual private result's metadata and same-associated statistics.
/// # Errors
/// Rejects the impossible zero literal frame in test setup.
/// # Panics
/// Rejects any mismatch in independently declared fixture facts.
fn check(selected: &FixtureRunsSelection, expected: &Expected) -> TestResult {
    let frame = NonZeroU64::new(17).ok_or_else(|| io::Error::other("runs frame"))?;
    assert_eq!(selected.start(), expected.start);
    assert_eq!(selected.sample_count(), expected.count);
    assert_eq!(selected.frame(), frame);
    assert_eq!(selected.decision_start(), expected.decision_start);
    assert_eq!(selected.decision_count(), expected.decision_count);
    assert_eq!(selected.decision_runs(), expected.decision_runs);
    assert_eq!(selected.fit_runs(), expected.fit_runs);
    if expected.passed {
        assert!(matches!(selected.stop(), FixtureRunsStop::RunsPassed));
    } else {
        assert!(matches!(selected.stop(), FixtureRunsStop::MinimumFallback));
    }
    let stats = selected.statistics();
    assert_eq!(stats.frame(), frame);
    assert_eq!(stats.line().frame(), frame);
    assert_eq!(stats.sample_count(), expected.count);
    assert_eq!(stats.degrees_of_freedom(), expected.count - 2);
    near(
        stats.line().phase_seconds(),
        expected.phase,
        COEFFICIENT_TOLERANCE,
    );
    near(
        stats.line().phase_rate_seconds_per_second(),
        expected.rate,
        COEFFICIENT_TOLERANCE,
    );
    near(
        stats.weighted_sse_seconds_squared(),
        expected.sse,
        SSE_TOLERANCE,
    );
    near(
        stats.residual_scale_seconds_squared(),
        expected.scale,
        SCALE_TOLERANCE,
    );
    near(
        stats.normalized_data_variance_seconds_squared(),
        expected.variance,
        VARIANCE_TOLERANCE,
    );
    Ok(())
}

/// Check the exact constant-phase branch, including all residual results.
/// # Errors
/// Propagates fixture or selector failure.
/// # Panics
/// Rejects an altered zero-run, frame, count, or constant-result fact.
#[test]
fn runs_constant_minimum_eight_and_maximum() -> TestResult {
    let profile = Profile::new();
    for count in [3_i32, 8, 64] {
        let rows: Vec<[f64; 3]> = ((1 - count)..=0)
            .map(|x| [f64::from(x), 0.25, 1.0])
            .collect();
        let selected = run(&profile, &rows, 3)?;
        let count = usize::try_from(count)?;
        check(
            &selected,
            &Expected {
                start: 0,
                count,
                decision_start: 0,
                decision_count: count,
                decision_runs: count,
                fit_runs: count,
                passed: true,
                phase: 0.25,
                rate: 0.0,
                sse: 0.0,
                scale: 0.0,
                variance: 0.0,
            },
        )?;
        let stats = selected.statistics();
        assert_eq!(
            stats.line().phase_seconds().to_bits(),
            0x3fd0_0000_0000_0000
        );
        assert_eq!(stats.line().phase_rate_seconds_per_second().to_bits(), 0);
        for value in [
            stats.weighted_sse_seconds_squared(),
            stats.residual_scale_seconds_squared(),
            stats.normalized_data_variance_seconds_squared(),
        ] {
            assert_eq!(value.to_bits() & 0x7fff_ffff_ffff_ffff, 0);
        }
    }
    Ok(())
}

/// Larger divisors contribute less weight, including at the minimum.
/// # Errors
/// Propagates fixture or selector failure.
/// # Panics
/// Rejects inverted weights, wrong selection or incorrect statistics.
#[test]
fn runs_unequal_divisors_use_inverse_weights() -> TestResult {
    let profile = Profile::new();
    let rows = [[-2.0, 0.0, 1.0], [-1.0, 0.5, 4.0], [0.0, 0.0, 1.0]];
    let selected = run(&profile, &rows, 3)?;
    check(
        &selected,
        &Expected {
            start: 0,
            count: 3,
            decision_start: 0,
            decision_count: 3,
            decision_runs: 3,
            fit_runs: 3,
            passed: true,
            phase: 1.0 / 18.0,
            rate: 0.0,
            sse: 1.0 / 18.0,
            scale: 1.0 / 18.0,
            variance: 2.0 / 27.0,
        },
    )
}

/// Independent 12-point x coordinates, relative to the newest observation.
const X12: [f64; 12] = [
    -11.0, -10.0, -9.0, -8.0, -7.0, -6.0, -5.0, -4.0, -3.0, -2.0, -1.0, 0.0,
];

/// Pair explicitly supplied phases with the frozen unit-divisor timeline.
/// # Panics
/// The fixed arrays share twelve indices; an out-of-range index is a test defect.
fn rows12(phases: [f64; 12]) -> [[f64; 3]; 12] {
    let mut rows = [[0.0; 3]; 12];
    for (index, row) in rows.iter_mut().enumerate() {
        *row = [X12[index], phases[index], 1.0];
    }
    rows
}

/// The extended decision window defeats a suffix-only early pass.
/// # Errors
/// Propagates fixture or selector failure.
/// # Panics
/// Rejects a non-strict equality decision or wrong fallback suffix.
#[test]
fn runs_extended_equality_reaches_minimum_fallback() -> TestResult {
    let profile = Profile::new();
    let rows = rows12([
        -21.0 / 32.0,
        -21.0 / 32.0,
        27.0 / 32.0,
        27.0 / 32.0,
        27.0 / 32.0,
        -27.0 / 32.0,
        -27.0 / 32.0,
        -27.0 / 32.0,
        -27.0 / 32.0,
        16.0 / 32.0,
        16.0 / 32.0,
        16.0 / 32.0,
    ]);
    let selected = run(&profile, &rows, 10)?;
    check(
        &selected,
        &Expected {
            start: 2,
            count: 10,
            decision_start: 0,
            decision_count: 12,
            decision_runs: 4,
            fit_runs: 3,
            passed: false,
            phase: -21.0 / 160.0,
            rate: -7.0 / 160.0,
            sse: 14_163.0 / 2_560.0,
            scale: 14_163.0 / 20_480.0,
            variance: 14_163.0 / 20_480.0,
        },
    )
}

/// A real strict one-above pass occurs after the first point is dropped.
/// # Errors
/// Propagates fixture or selector failure.
/// # Panics
/// Rejects a premature stop, fallback, or copied full-window run count.
#[test]
fn runs_strict_pass_after_one_drop() -> TestResult {
    let profile = Profile::new();
    let rows = rows12([
        1.0,
        -21.0 / 512.0,
        27.0 / 512.0,
        27.0 / 512.0,
        27.0 / 512.0,
        -27.0 / 512.0,
        -27.0 / 512.0,
        -27.0 / 512.0,
        -27.0 / 512.0,
        16.0 / 512.0,
        16.0 / 512.0,
        16.0 / 512.0,
    ]);
    let selected = run(&profile, &rows, 3)?;
    check(
        &selected,
        &Expected {
            start: 1,
            count: 11,
            decision_start: 0,
            decision_count: 12,
            decision_runs: 5,
            fit_runs: 4,
            passed: true,
            phase: 0.0,
            rate: 0.0,
            sse: 789.0 / 32_768.0,
            scale: 789.0 / 294_912.0,
            variance: 789.0 / 294_912.0,
        },
    )
}

/// This trajectory reaches the nonzero extended-window start.
/// # Errors
/// Propagates fixture or selector failure.
/// # Panics
/// Rejects an incorrect window boundary, run count, line or terminal summary.
#[test]
fn runs_reaches_nonzero_decision_window_start() -> TestResult {
    let profile = Profile::new();
    let epsilon = 1.0 / 65_536.0;
    let rows = rows12([
        121.0 / 128.0,
        100.0 / 128.0,
        81.0 / 128.0,
        64.0 / 128.0,
        49.0 / 128.0,
        36.0 / 128.0,
        25.0 / 128.0,
        16.0 / 128.0,
        9.0 / 128.0,
        4.0 / 128.0,
        1.0 / 128.0,
        1.0 / 65_536.0,
    ]);
    let selected = run(&profile, &rows, 3)?;
    let sse = 1.0 / 4096.0 + epsilon / 64.0 + 3.0 * epsilon * epsilon / 10.0;
    check(
        &selected,
        &Expected {
            start: 8,
            count: 4,
            decision_start: 4,
            decision_count: 8,
            decision_runs: 3,
            fit_runs: 3,
            passed: true,
            phase: -1.0 / 128.0 + 7.0 * epsilon / 10.0,
            rate: -3.0 / 128.0 + 3.0 * epsilon / 10.0,
            sse,
            scale: sse / 2.0,
            variance: sse / 2.0,
        },
    )
}

/// A normal-sized residual-square neighbor of the proposed terminal fault.
/// # Errors
/// Propagates fixture or selector failure.
/// # Panics
/// Rejects wrong run counts or an absent/nonfinite positive statistic.
#[test]
fn runs_small_normal_residual_square_neighbor() -> TestResult {
    let profile = Profile::new();
    let magnitude = f64::from_bits(0x20b0_0000_0000_0000);
    let rows = [
        [-2.0, magnitude, 1.0],
        [-1.0, 0.0, 1.0],
        [0.0, magnitude, 1.0],
    ];
    let selected = run(&profile, &rows, 3)?;
    let frame = NonZeroU64::new(17).ok_or_else(|| io::Error::other("runs frame"))?;
    assert_eq!(selected.start(), 0);
    assert_eq!(selected.sample_count(), 3);
    assert_eq!(selected.frame(), frame);
    assert_eq!(
        (selected.decision_start(), selected.decision_count()),
        (0, 3)
    );
    assert_eq!((selected.decision_runs(), selected.fit_runs()), (3, 3));
    assert!(matches!(selected.stop(), FixtureRunsStop::RunsPassed));
    let stats = selected.statistics();
    assert_eq!(stats.frame(), frame);
    assert_eq!(stats.line().frame(), frame);
    assert_eq!((stats.sample_count(), stats.degrees_of_freedom()), (3, 1));
    for value in [
        stats.weighted_sse_seconds_squared(),
        stats.residual_scale_seconds_squared(),
        stats.normalized_data_variance_seconds_squared(),
    ] {
        assert!(value.is_finite() && value.is_normal() && value > 0.0);
    }

    // These three residuals have zero sum and zero covariance for x=-8..=0.
    let tiny = f64::from_bits(0x2000_0000_0000_0000);
    let mut summary_rows = [
        [-8.0, tiny, 1.0],
        [-7.0, -2.0 * tiny, 1.0],
        [-6.0, tiny, 1.0],
        [-5.0, 0.0, 1.0],
        [-4.0, 0.0, 1.0],
        [-3.0, 0.0, 1.0],
        [-2.0, 0.0, 1.0],
        [-1.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
    ];
    let error = rejected(run_result(&profile, &summary_rows, 9)?)?;
    check_summary_rejection(&error, tiny)?;

    let neighbor = f64::from_bits(0x2010_0000_0000_0000);
    summary_rows[0][1] = neighbor;
    summary_rows[1][1] = -2.0 * neighbor;
    summary_rows[2][1] = neighbor;
    let selected = run(&profile, &summary_rows, 9)?;
    assert_eq!(
        (selected.start(), selected.sample_count(), selected.frame()),
        (0, 9, frame)
    );
    assert_eq!(
        (selected.decision_start(), selected.decision_count()),
        (0, 9)
    );
    assert_eq!((selected.decision_runs(), selected.fit_runs()), (9, 9));
    assert!(matches!(selected.stop(), FixtureRunsStop::RunsPassed));
    let stats = selected.statistics();
    assert_eq!(
        (
            stats.frame(),
            stats.sample_count(),
            stats.degrees_of_freedom()
        ),
        (frame, 9, 7)
    );
    assert_eq!(
        stats.line().phase_seconds().to_bits() & 0x7fff_ffff_ffff_ffff,
        0
    );
    assert_eq!(
        stats.line().phase_rate_seconds_per_second().to_bits() & 0x7fff_ffff_ffff_ffff,
        0
    );
    assert_eq!(
        stats.weighted_sse_seconds_squared().to_bits(),
        (24.0 * f64::MIN_POSITIVE).to_bits()
    );
    let scale = stats.residual_scale_seconds_squared();
    assert!(scale > 3.0 * f64::MIN_POSITIVE && scale < 4.0 * f64::MIN_POSITIVE);
    for value in [
        stats.weighted_sse_seconds_squared(),
        scale,
        stats.normalized_data_variance_seconds_squared(),
    ] {
        assert!(value.is_finite() && value.is_normal() && value > 0.0);
    }
    assert_eq!(
        (
            profile.points.get(),
            profile.inputs.get(),
            profile.selections.get()
        ),
        (21, 3, 3)
    );
    Ok(())
}

/// Extract the actual selector rejection without substituting an error value.
/// # Errors
/// Rejects unexpected success as a fixture assertion failure.
fn rejected(
    result: Result<FixtureRunsSelection, FixtureRunsError>,
) -> TestResult<FixtureRunsError> {
    let Err(error) = result else {
        return Err(io::Error::other("expected runs rejection").into());
    };
    Ok(error)
}

/// Check bounded redacted text for an actual returned error.
/// # Panics
/// Rejects excessive formatted text or the supplied raw-value representations.
fn check_error_redaction(error: &FixtureRunsError, forbidden: &[String]) {
    let rendered = [error.to_string(), format!("{error:?}")];
    for value in rendered {
        assert!(value.len() <= 512);
        for raw in forbidden {
            assert!(!value.contains(raw));
        }
    }
}

/// Assert the actual terminal scale failure and its retained nested cause.
/// # Errors
/// Returns an error if either expected source is absent.
/// # Panics
/// Rejects a wrong context, arithmetic cause or unredacted format.
fn check_summary_rejection(error: &FixtureRunsError, tiny: f64) -> TestResult {
    assert_eq!(error.stage(), FixtureRunsStage::StatisticsSummary);
    assert_eq!(
        (error.candidate_start(), error.point_index()),
        (Some(0), None)
    );
    assert_eq!(error.to_string(), "fixture runs: StatisticsSummary");
    let source = error
        .source()
        .ok_or_else(|| io::Error::other("missing summary source"))?;
    assert!(matches!(
        source.downcast_ref::<ResidualError>(),
        Some(ResidualError::Arithmetic {
            step: ResidualStep::Scale,
            source: FitError::Arithmetic {
                stage: ArithmeticStage::Response,
                operation: ArithmeticOperation::Divide,
                issue: ArithmeticIssue::Subnormal,
            },
        })
    ));
    let cause = source
        .source()
        .ok_or_else(|| io::Error::other("missing scale arithmetic source"))?;
    assert!(matches!(
        cause.downcast_ref::<FitError>(),
        Some(FitError::Arithmetic {
            stage: ArithmeticStage::Response,
            operation: ArithmeticOperation::Divide,
            issue: ArithmeticIssue::Subnormal,
        })
    ));
    assert!(cause.source().is_none());
    check_error_redaction(error, &[format!("{tiny:e}"), format!("{tiny:?}")]);
    Ok(())
}

/// Minimum validation occurs before candidate work, with a valid neighbor.
/// # Errors
/// Propagates fixture setup and unexpected selector success/failure.
/// # Panics
/// Rejects incorrect minimum-stage context, source or unredacted error output.
#[test]
fn runs_minimum_rejects_invalid_values_and_accepts_three() -> TestResult {
    let profile = Profile::new();
    let rows = [[-2.0, 0.25, 1.0], [-1.0, 0.25, 1.0], [0.0, 0.25, 1.0]];
    for minimum in [0, 2, 4, usize::MAX] {
        let error = rejected(run_result(&profile, &rows, minimum)?)?;
        assert!(matches!(error.stage(), FixtureRunsStage::Minimum));
        assert_eq!(error.candidate_start(), None);
        assert_eq!(error.point_index(), None);
        assert!(error.source().is_none());
        check_error_redaction(&error, &[String::from("0.25")]);
    }
    let selected = run(&profile, &rows, 3)?;
    check(
        &selected,
        &Expected {
            start: 0,
            count: 3,
            decision_start: 0,
            decision_count: 3,
            decision_runs: 3,
            fit_runs: 3,
            passed: true,
            phase: 0.25,
            rate: 0.0,
            sse: 0.0,
            scale: 0.0,
            variance: 0.0,
        },
    )
}

/// An early checked fit failure retains its source after minimum precedence.
/// # Errors
/// Propagates fixture setup and unexpected selector success.
/// # Panics
/// Rejects incorrect operation order, typed cause, location, or raw-value leakage.
#[test]
fn runs_early_fit_failure_retains_typed_source_and_position() -> TestResult {
    let profile = Profile::new();
    let tiny = f64::MIN_POSITIVE;
    let rows = [[-2.0, tiny, 16.0], [-1.0, 0.0, 1.0], [0.0, 0.0, 1.0]];
    let invalid_minimum = rejected(run_result(&profile, &rows, 2)?)?;
    assert!(matches!(invalid_minimum.stage(), FixtureRunsStage::Minimum));
    assert_eq!(invalid_minimum.candidate_start(), None);
    assert_eq!(invalid_minimum.point_index(), None);
    assert!(invalid_minimum.source().is_none());
    let error = rejected(run_result(&profile, &rows, 3)?)?;
    assert!(matches!(error.stage(), FixtureRunsStage::Fit));
    assert_eq!(error.candidate_start(), Some(0));
    assert_eq!(error.point_index(), None);
    let source = error
        .source()
        .ok_or_else(|| io::Error::other("missing fit source"))?;
    assert!(matches!(
        source.downcast_ref::<FitError>(),
        Some(FitError::Arithmetic {
            stage: ArithmeticStage::Response,
            operation: ArithmeticOperation::Multiply,
            issue: ArithmeticIssue::Subnormal,
        })
    ));
    assert!(source.source().is_none());
    check_error_redaction(&error, &[format!("{tiny:e}"), format!("{tiny:?}")]);
    Ok(())
}

/// A terminal square failure retains the actual original input row and cause.
/// # Errors
/// Propagates fixture setup and unexpected selector success.
/// # Panics
/// Rejects an incorrect statistics-row context, source chain, or leaked input.
#[test]
fn runs_terminal_statistics_failure_retains_row_and_typed_source() -> TestResult {
    let profile = Profile::new();
    let magnitude = f64::from_bits(0x1f70_0000_0000_0000);
    let rows = [
        [-2.0, magnitude, 1.0],
        [-1.0, 0.0, 1.0],
        [0.0, magnitude, 1.0],
    ];
    let error = rejected(run_result(&profile, &rows, 3)?)?;
    assert!(matches!(error.stage(), FixtureRunsStage::StatisticsRow));
    assert_eq!(error.candidate_start(), Some(0));
    assert_eq!(error.point_index(), Some(0));
    let source = error
        .source()
        .ok_or_else(|| io::Error::other("missing statistics source"))?;
    assert!(matches!(
        source.downcast_ref::<ResidualError>(),
        Some(ResidualError::Arithmetic {
            step: ResidualStep::Square,
            source: FitError::Arithmetic {
                stage: ArithmeticStage::Response,
                operation: ArithmeticOperation::Multiply,
                issue: ArithmeticIssue::Subnormal,
            },
        })
    ));
    let cause = source
        .source()
        .ok_or_else(|| io::Error::other("missing arithmetic source"))?;
    assert!(matches!(
        cause.downcast_ref::<FitError>(),
        Some(FitError::Arithmetic {
            stage: ArithmeticStage::Response,
            operation: ArithmeticOperation::Multiply,
            issue: ArithmeticIssue::Subnormal,
        })
    ));
    assert!(cause.source().is_none());
    check_error_redaction(
        &error,
        &[format!("{magnitude:e}"), format!("{magnitude:?}")],
    );
    Ok(())
}

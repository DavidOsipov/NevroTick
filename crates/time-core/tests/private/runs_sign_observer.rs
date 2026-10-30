// SPDX-License-Identifier: GPL-2.0-only
//! Bounded test-only observation of two actual candidate-cache residuals.
//!
//! The fixture and open intervals are frozen independently of the selector.
//! Captures are from the actual cache slot after a successful residual write;
//! no residual or fit is recomputed here. These tests observe a test artifact,
//! not production authentication, time authority, or native behavior.

use std::{
    error::Error,
    io::{self, Write},
    num::NonZeroU64,
};

use super::{
    FitError, FitInput, FixtureRunsError, FixtureRunsSelection, FixtureRunsStage, FixtureRunsStop,
    ResidualError, RunsObserver, select_fixture_runs, select_fixture_runs_observed,
};
use crate::FitPoint;

/// Setup failures remain distinct from selector results and assertions.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;

/// Fixed saturating capture state; a duplicate never overwrites first evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Seen {
    /// No matching event occurred.
    Zero,
    /// Exactly one matching event occurred.
    Once,
    /// More than one matching event occurred.
    Duplicate,
}

/// Two owned slots and fixed-size observer bookkeeping for one selector call.
#[derive(Debug)]
struct SignObserver {
    /// First actual residual bits at candidate 5, original index 6.
    positive: Option<u64>,
    /// First actual residual bits at candidate 5, original index 10.
    negative: Option<u64>,
    /// Saturating state for the first target.
    positive_seen: Seen,
    /// Saturating state for the second target.
    negative_seen: Seen,
    /// Saturating number of hook calls, for bounded evidence.
    calls: usize,
}

impl SignObserver {
    /// Start one empty, privately owned observer with no product effect.
    const fn new() -> Self {
        Self {
            positive: None,
            negative: None,
            positive_seen: Seen::Zero,
            negative_seen: Seen::Zero,
            calls: 0,
        }
    }
}

/// Save first observed bits and mark a duplicate without returning a decision.
fn capture(seen: &mut Seen, slot: &mut Option<u64>, residual: f64) {
    match *seen {
        Seen::Zero => {
            *slot = Some(residual.to_bits());
            *seen = Seen::Once;
        }
        Seen::Once => *seen = Seen::Duplicate,
        Seen::Duplicate => {}
    }
}

impl RunsObserver for SignObserver {
    /// Observe the actual immutable cache value, without changing the selector.
    fn observe(&mut self, start: usize, index: usize, residual: &f64) {
        self.calls = self.calls.saturating_add(1);
        if (start, index) == (5, 6) {
            capture(&mut self.positive_seen, &mut self.positive, *residual);
        } else if (start, index) == (5, 10) {
            capture(&mut self.negative_seen, &mut self.negative, *residual);
        }
    }
}

/// Actual per-test public constructor and selector attempt counts.
#[derive(Debug, Default)]
struct Counts {
    /// Number of point constructors entered.
    points: usize,
    /// Number of input constructors entered.
    inputs: usize,
    /// Number of selector calls entered.
    selectors: usize,
}

impl Drop for Counts {
    /// Emit the reached attempted prefix even on ordinary assertion unwinding.
    fn drop(&mut self) {
        let record = format!(
            "CORE_RUNS_SIGN_PROFILE points={} inputs={} selectors={}\n",
            self.points, self.inputs, self.selectors
        );
        let _emission_result = io::stderr().lock().write_all(record.as_bytes());
    }
}

/// Increment one bounded setup counter before invoking the named owner.
/// # Errors
/// Rejects an exhausted or overflowing local budget before the operation.
fn charge(value: &mut usize, ceiling: usize) -> TestResult {
    let next = value
        .checked_add(1)
        .ok_or_else(|| io::Error::other("sign observer count overflow"))?;
    if next > ceiling {
        return Err(io::Error::other("sign observer count ceiling").into());
    }
    *value = next;
    Ok(())
}

/// Twelve independently declared quadratic phases and newest-relative times.
const ROWS: [[f64; 3]; 12] = [
    [-11.0, 121.0 / 128.0, 1.0],
    [-10.0, 100.0 / 128.0, 1.0],
    [-9.0, 81.0 / 128.0, 1.0],
    [-8.0, 64.0 / 128.0, 1.0],
    [-7.0, 49.0 / 128.0, 1.0],
    [-6.0, 36.0 / 128.0, 1.0],
    [-5.0, 25.0 / 128.0, 1.0],
    [-4.0, 16.0 / 128.0, 1.0],
    [-3.0, 9.0 / 128.0, 1.0],
    [-2.0, 4.0 / 128.0, 1.0],
    [-1.0, 1.0 / 128.0, 1.0],
    [0.0, 1.0 / 65_536.0, 1.0],
];

/// Construct one independent checked fixture; no point attempt is repeated.
/// # Errors
/// Preserves local ceiling and actual point/input construction errors.
fn fixture(counts: &mut Counts) -> TestResult<(FitInput, [FitPoint; 12])> {
    charge(&mut counts.points, 24)?;
    let first = FitPoint::new(ROWS[0][0], ROWS[0][1], ROWS[0][2])?;
    let mut points = [first; 12];
    for (slot, row) in points.iter_mut().zip(ROWS).skip(1) {
        charge(&mut counts.points, 24)?;
        *slot = FitPoint::new(row[0], row[1], row[2])?;
    }
    let frame = NonZeroU64::new(17).ok_or_else(|| io::Error::other("sign fixture frame"))?;
    charge(&mut counts.inputs, 2)?;
    let input = FitInput::new(frame, &points)?;
    Ok((input, points))
}

/// Preserve every caller-owned point scalar bit.
fn point_bits(point: &FitPoint) -> [u64; 3] {
    [
        point.relative_seconds().to_bits(),
        point.phase_seconds().to_bits(),
        point.variance_divisor().to_bits(),
    ]
}

/// Project all 12 immutable caller-owned point records.
fn caller_bits(points: &[FitPoint; 12]) -> [[u64; 3]; 12] {
    points.map(|point| point_bits(&point))
}

/// Project the actual private admitted batch, without exposing it publicly.
fn input_bits(input: &FitInput) -> Option<[[u64; 3]; 12]> {
    let batch = input.batch();
    let points: &[FitPoint; 12] = batch.values.try_into().ok()?;
    Some(caller_bits(points))
}

/// Every public result field, including raw coefficient/statistic bits.
#[derive(Debug, Eq, PartialEq)]
struct SuccessFacts {
    /// Original selected suffix start.
    start: usize,
    /// Selected suffix count.
    count: usize,
    /// Asserted result frame.
    frame: NonZeroU64,
    /// Declared terminal reason.
    stop: FixtureRunsStop,
    /// Original decision-window start.
    decision_start: usize,
    /// Actual decision-window length.
    decision_count: usize,
    /// Actual decision-window runs.
    decision_runs: usize,
    /// Actual fitted-suffix runs.
    fit_runs: usize,
    /// Actual statistics frame.
    statistics_frame: NonZeroU64,
    /// Actual statistics population.
    statistics_count: usize,
    /// Actual statistics degrees of freedom.
    degrees: usize,
    /// Actual fitted-line frame.
    line_frame: NonZeroU64,
    /// Actual phase, rate, SSE, scale and variance bits, in that order.
    numeric_bits: [u64; 5],
}

/// Complete recognized nested source chain; unknown or excess sources are marked.
#[derive(Debug, Eq, PartialEq)]
enum SourceFacts {
    /// The selector error carries no source.
    None,
    /// A direct fit source and whether it unexpectedly has another source.
    Fit(FitError, bool),
    /// A residual source, its fit cause if any, and an unexpected chain marker.
    Residual(ResidualError, Option<FitError>, bool),
    /// A source or nested source was not one of the admitted error owners.
    Unknown,
}

/// Actual structured error context and complete recognized source chain.
#[derive(Debug, Eq, PartialEq)]
struct FailureFacts {
    /// Actual selector failure stage.
    stage: FixtureRunsStage,
    /// Actual optional original candidate start.
    candidate_start: Option<usize>,
    /// Actual optional original point index.
    point_index: Option<usize>,
    /// Actual typed nested error chain.
    source: SourceFacts,
}

/// One complete actual public outcome projection, never an expected result.
#[derive(Debug, Eq, PartialEq)]
enum OutcomeFacts {
    /// All public success facts.
    Success(SuccessFacts),
    /// All public failure facts and typed source chain.
    Failure(FailureFacts),
}

/// Read the actual source chain without guessing a cause from formatted text.
fn source_facts(error: &FixtureRunsError) -> SourceFacts {
    let Some(source) = error.source() else {
        return SourceFacts::None;
    };
    if let Some(fit) = source.downcast_ref::<FitError>() {
        return SourceFacts::Fit(*fit, source.source().is_some());
    }
    if let Some(residual) = source.downcast_ref::<ResidualError>() {
        return match source.source() {
            None => SourceFacts::Residual(*residual, None, false),
            Some(nested) => match nested.downcast_ref::<FitError>() {
                Some(fit) => {
                    SourceFacts::Residual(*residual, Some(*fit), nested.source().is_some())
                }
                None => SourceFacts::Unknown,
            },
        };
    }
    SourceFacts::Unknown
}

/// Project the real selector return without propagating or asserting it.
fn outcome_facts(result: &Result<FixtureRunsSelection, FixtureRunsError>) -> OutcomeFacts {
    match result {
        Ok(selected) => {
            let stats = selected.statistics();
            let line = stats.line();
            OutcomeFacts::Success(SuccessFacts {
                start: selected.start(),
                count: selected.sample_count(),
                frame: selected.frame(),
                stop: selected.stop(),
                decision_start: selected.decision_start(),
                decision_count: selected.decision_count(),
                decision_runs: selected.decision_runs(),
                fit_runs: selected.fit_runs(),
                statistics_frame: stats.frame(),
                statistics_count: stats.sample_count(),
                degrees: stats.degrees_of_freedom(),
                line_frame: line.frame(),
                numeric_bits: [
                    line.phase_seconds().to_bits(),
                    line.phase_rate_seconds_per_second().to_bits(),
                    stats.weighted_sse_seconds_squared().to_bits(),
                    stats.residual_scale_seconds_squared().to_bits(),
                    stats.normalized_data_variance_seconds_squared().to_bits(),
                ],
            })
        }
        Err(error) => OutcomeFacts::Failure(FailureFacts {
            stage: error.stage(),
            candidate_start: error.candidate_start(),
            point_index: error.point_index(),
            source: source_facts(error),
        }),
    }
}

/// Persist a bounded synthetic pre-assertion receipt through the root collector.
/// # Errors
/// Rejects an oversized record or preserves the actual stderr write failure.
fn receipt(
    counts: &Counts,
    capture_state: &SignObserver,
    observed: &OutcomeFacts,
    ordinary: &OutcomeFacts,
    preservation: [bool; 6],
    identities: [((NonZeroU64, usize), (NonZeroU64, usize)); 2],
) -> io::Result<()> {
    let record = format!(
        "CORE_RUNS_SIGN_RECEIPT counts={counts:?} observer={capture_state:?} observed={observed:?} ordinary={ordinary:?} preservation={preservation:?} identities={identities:?}\n"
    );
    if record.len() > 4096 {
        return Err(io::Error::other("sign observer receipt exceeds bound"));
    }
    io::stderr().lock().write_all(record.as_bytes())
}

/// Compare one finite actual output with an independently reviewed bound.
/// # Panics
/// Fails for nonfinite values or deviations exceeding the frozen tolerance.
fn near(bits: u64, expected: f64, tolerance: f64) {
    let value = f64::from_bits(bits);
    assert!(value.is_finite());
    assert!((value - expected).abs() <= tolerance);
}

/// Validate the separately derived terminal trajectory and numerical facts.
/// # Panics
/// Fails when actual selection, same-line report or numerical facts differ.
fn check_expected(actual: &SuccessFacts, frame: NonZeroU64) {
    assert_eq!((actual.start, actual.count, actual.frame), (8, 4, frame));
    assert_eq!(actual.stop, FixtureRunsStop::RunsPassed);
    assert_eq!(
        (
            actual.decision_start,
            actual.decision_count,
            actual.decision_runs,
            actual.fit_runs
        ),
        (4, 8, 3, 3)
    );
    assert_eq!(
        (
            actual.statistics_frame,
            actual.statistics_count,
            actual.degrees,
            actual.line_frame
        ),
        (frame, 4, 2, frame)
    );
    let epsilon = 1.0 / 65_536.0;
    let sse = 1.0 / 4096.0 + epsilon / 64.0 + 3.0 * epsilon * epsilon / 10.0;
    near(
        actual.numeric_bits[0],
        -1.0 / 128.0 + 7.0 * epsilon / 10.0,
        3e-9,
    );
    near(
        actual.numeric_bits[1],
        -3.0 / 128.0 + 3.0 * epsilon / 10.0,
        3e-9,
    );
    near(actual.numeric_bits[2], sse, 4e-6);
    near(actual.numeric_bits[3], sse / 2.0, 5e-6);
    near(actual.numeric_bits[4], sse / 2.0, 3e-5);
}

/// Read the two reached cache witnesses even when selection later rejects.
/// # Errors
/// Propagates setup, receipt, or unexpected healthy-selector failures.
/// # Panics
/// Fails on missing/duplicate/wrong-sign captures or any outcome/input mismatch.
#[test]
fn observed_cache_near_zero_signs_and_public_equivalence() -> TestResult {
    let mut counts = Counts::default();
    let (observed_input, observed_points) = fixture(&mut counts)?;
    let (ordinary_input, ordinary_points) = fixture(&mut counts)?;
    let observed_caller_before = caller_bits(&observed_points);
    let ordinary_caller_before = caller_bits(&ordinary_points);
    let observed_input_before = input_bits(&observed_input);
    let ordinary_input_before = input_bits(&ordinary_input);
    let observed_identity_before = (observed_input.frame(), observed_input.sample_count());
    let ordinary_identity_before = (ordinary_input.frame(), ordinary_input.sample_count());
    let mut capture_state = SignObserver::new();
    charge(&mut counts.selectors, 2)?;
    let observed_result = select_fixture_runs_observed(&observed_input, 3, &mut capture_state);
    charge(&mut counts.selectors, 2)?;
    let ordinary_result = select_fixture_runs(&ordinary_input, 3);
    let observed = outcome_facts(&observed_result);
    let ordinary = outcome_facts(&ordinary_result);
    let identities = [
        (
            observed_identity_before,
            (observed_input.frame(), observed_input.sample_count()),
        ),
        (
            ordinary_identity_before,
            (ordinary_input.frame(), ordinary_input.sample_count()),
        ),
    ];
    let preservation = [
        caller_bits(&observed_points) == observed_caller_before,
        caller_bits(&ordinary_points) == ordinary_caller_before,
        input_bits(&observed_input) == observed_input_before,
        input_bits(&ordinary_input) == ordinary_input_before,
        identities[0].0 == identities[0].1,
        identities[1].0 == identities[1].1,
    ];
    receipt(
        &counts,
        &capture_state,
        &observed,
        &ordinary,
        preservation,
        identities,
    )?;

    // No selector Result is propagated before both actual cache observations.
    assert_eq!(
        capture_state.positive_seen,
        Seen::Once,
        "runs_sign_positive_identity"
    );
    assert!(
        capture_state
            .positive
            .map(f64::from_bits)
            .is_some_and(|value| value.is_finite()
                && value.is_normal()
                && value > 1.0 / 1_048_576.0
                && value < 1.0 / 524_288.0),
        "runs_sign_positive_interval"
    );
    assert_eq!(
        capture_state.negative_seen,
        Seen::Once,
        "runs_sign_negative_identity"
    );
    assert!(
        capture_state
            .negative
            .map(f64::from_bits)
            .is_some_and(|value| value.is_finite()
                && value.is_normal()
                && value > -1.0 / 131_072.0
                && value < -1.0 / 262_144.0),
        "runs_sign_negative_interval"
    );
    assert_eq!((counts.points, counts.inputs, counts.selectors), (24, 2, 2));
    assert!(capture_state.calls <= 3098);
    assert_eq!(preservation, [true; 6]);
    assert_eq!(observed_input_before, Some(observed_caller_before));
    assert_eq!(ordinary_input_before, Some(ordinary_caller_before));
    assert_eq!(observed_caller_before, ordinary_caller_before);
    let frame = NonZeroU64::new(17).ok_or_else(|| io::Error::other("sign fixture frame"))?;
    assert_eq!(
        (observed_input.frame(), observed_input.sample_count()),
        (frame, 12)
    );
    assert_eq!(
        (ordinary_input.frame(), ordinary_input.sample_count()),
        (frame, 12)
    );
    if let OutcomeFacts::Failure(failure) = &observed {
        assert!(!matches!(failure.source, SourceFacts::Unknown));
    }
    if let OutcomeFacts::Failure(failure) = &ordinary {
        assert!(!matches!(failure.source, SourceFacts::Unknown));
    }
    assert_eq!(observed, ordinary);
    let OutcomeFacts::Success(actual) = &observed else {
        return Err(io::Error::other("healthy selector rejected").into());
    };
    check_expected(actual, frame);
    Ok(())
}

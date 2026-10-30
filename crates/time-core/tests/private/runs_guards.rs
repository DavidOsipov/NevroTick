// SPDX-License-Identifier: GPL-2.0-only
//! Separately bounded private runs-table, sign, error-context and window controls.
//!
//! Frozen literal expectations come from the independently transcribed Chrony
//! table and fixture contract; private synthetic helper cases do not claim a
//! public selector reached the same window or native/product behavior.

use std::{
    error::Error,
    io::{self, Write},
    num::NonZeroU64,
};

use super::{
    FitError, FitInput, FixtureRunsError, FixtureRunsStage, ResidualError, RunsObserver,
    TerminalCandidate, count_runs, critical_runs,
};
use crate::{ArithmeticIssue, ArithmeticOperation, ArithmeticStage, FitPoint, ResidualStep};

/// Setup failures remain distinct from assertions and real owner failures.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;

/// Actual per-scope attempted helper and constructor operations.
#[derive(Debug, Default)]
struct Counts {
    /// Critical-table lookup attempts.
    table: usize,
    /// Raw run-count attempts.
    runs: usize,
    /// Contextual error-construction attempts.
    contexts: usize,
    /// Public checked point-construction attempts.
    points: usize,
    /// Public checked input-construction attempts.
    inputs: usize,
    /// Private candidate-preparation attempts.
    prepares: usize,
    /// Direct fit calls made by these helper scopes (none).
    direct_fits: usize,
    /// Public selector calls made by these helper scopes (none).
    selectors: usize,
    /// Sum of lengths returned by successful candidate preparations.
    returned_window_rows: usize,
}

impl Drop for Counts {
    /// Emit only the actual reached prefix; a missing receipt is incomplete.
    fn drop(&mut self) {
        let record = format!(
            "CORE_RUNS_GUARDS table={} runs={} contexts={} points={} inputs={} prepares={} direct_fits={} selectors={} returned_window_rows={}\n",
            self.table,
            self.runs,
            self.contexts,
            self.points,
            self.inputs,
            self.prepares,
            self.direct_fits,
            self.selectors,
            self.returned_window_rows
        );
        let _emission_result = io::stderr().lock().write_all(record.as_bytes());
    }
}

/// Charge an attempted operation before reaching its actual owner.
/// # Errors
/// Rejects counter overflow or a scope-specific ceiling before that operation.
fn charge(counter: &mut usize, ceiling: usize) -> TestResult {
    let next = counter
        .checked_add(1)
        .ok_or_else(|| io::Error::other("runs guard counter overflow"))?;
    if next > ceiling {
        return Err(io::Error::other("runs guard counter ceiling").into());
    }
    *counter = next;
    Ok(())
}

/// Independently transcribed pinned values for actual window lengths 3..=64.
const EXPECTED_CRITICAL: [usize; 62] = [
    0, 0, 0, 0, 0, 2, 3, 3, 3, 4, 4, 5, 5, 5, 6, 6, 7, 7, 7, 8, 8, 9, 9, 9, 10, 10, 11, 11, 11, 12,
    12, 13, 13, 14, 14, 14, 15, 15, 16, 16, 17, 17, 18, 18, 18, 19, 19, 20, 20, 21, 21, 21, 22, 22,
    23, 23, 24, 24, 25, 25, 26, 26,
];

/// Compare every admitted entry and both out-of-range neighbors.
/// # Errors
/// Propagates a local accounting ceiling failure.
/// # Panics
/// Rejects any changed threshold, index mapping, or boundary admission.
#[test]
fn runs_critical_table_matches_independent_3_through_64() -> TestResult {
    let mut counts = Counts::default();
    for (length, expected) in (3_usize..=64).zip(EXPECTED_CRITICAL) {
        charge(&mut counts.table, 64)?;
        assert_eq!(
            critical_runs(length),
            Some(expected),
            "window length {length}"
        );
    }
    for invalid in [2, 65] {
        charge(&mut counts.table, 64)?;
        assert_eq!(critical_runs(invalid), None);
    }
    assert_eq!(counts.table, 64);
    Ok(())
}

/// Count signed zeros and strict threshold controls through the actual helper.
/// # Errors
/// Propagates a local accounting ceiling failure.
/// # Panics
/// Rejects any incorrect sign boundary, count, length guard, or strict decision.
#[test]
fn runs_sign_helper_counts_zero_and_strict_threshold_neighbors() -> TestResult {
    let mut counts = Counts::default();
    let negative_zero = f64::from_bits(0x8000_0000_0000_0000);
    let positive_zero = f64::from_bits(0);
    let cases: [(&[f64], usize); 11] = [
        (&[-1.0, -1.0, positive_zero, positive_zero, 1.0, -1.0], 5),
        (&[negative_zero, positive_zero], 2),
        (&[positive_zero, negative_zero, positive_zero], 3),
        (&[positive_zero, -1.0], 2),
        (&[negative_zero, -1.0], 2),
        (&[1.0, 1.0], 1),
        (&[-1.0, -1.0], 1),
        (&[1.0, -1.0], 2),
        (
            &[
                1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0,
            ],
            3,
        ),
        (
            &[
                1.0, 1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, -1.0, -1.0, -1.0,
            ],
            4,
        ),
        (
            &[
                1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0,
            ],
            5,
        ),
    ];
    for (values, expected) in cases {
        charge(&mut counts.runs, 13)?;
        assert_eq!(count_runs(values), Some(expected));
    }
    charge(&mut counts.runs, 13)?;
    assert_eq!(count_runs(&[]), None);
    charge(&mut counts.runs, 13)?;
    assert_eq!(count_runs(&[1.0; 65]), None);
    charge(&mut counts.table, 1)?;
    let threshold = critical_runs(12).ok_or_else(|| io::Error::other("missing threshold"))?;
    assert_eq!(threshold, 4);
    assert!(3 <= threshold);
    assert!(4 <= threshold);
    assert!(5 > threshold);
    assert_eq!((counts.runs, counts.table), (13, 1));
    Ok(())
}

/// A bounded, publicly constructible typed cause for context-guard tests.
fn fit_cause() -> FitError {
    FitError::Arithmetic {
        stage: ArithmeticStage::Response,
        operation: ArithmeticOperation::Multiply,
        issue: ArithmeticIssue::Subnormal,
    }
}

/// A bounded residual cause retains its actual nested typed fit cause.
fn residual_cause() -> ResidualError {
    ResidualError::Arithmetic {
        step: ResidualStep::Square,
        source: fit_cause(),
    }
}

/// Fixed public text for independently expected context-guard outcomes.
fn expected_context_display(stage: FixtureRunsStage, valid: bool) -> &'static str {
    match (stage, valid) {
        (FixtureRunsStage::Suffix, true) => "fixture runs: Suffix",
        (FixtureRunsStage::Fit, true) => "fixture runs: Fit",
        (FixtureRunsStage::DecisionResidual, true) => "fixture runs: DecisionResidual",
        (FixtureRunsStage::StatisticsRow, true) => "fixture runs: StatisticsRow",
        (FixtureRunsStage::StatisticsSummary, true) => "fixture runs: StatisticsSummary",
        _ => "fixture runs: Invariant",
    }
}

/// Exercise actual private context guards with explicit valid and invalid inputs.
/// # Errors
/// Propagates counter failures or a missing valid nested source.
/// # Panics
/// Rejects an incorrectly admitted context, altered cause, or forged position.
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "The finite 21-case contextual guard inventory remains together for review."
)]
fn runs_error_context_guards_preserve_sources_and_reject_invalid_positions() -> TestResult {
    let mut counts = Counts::default();
    let fit_cases = [
        (FixtureRunsStage::Suffix, 0, 3, true),
        (FixtureRunsStage::Fit, 61, 64, true),
        (FixtureRunsStage::Suffix, 0, 2, false),
        (FixtureRunsStage::Suffix, 0, 65, false),
        (FixtureRunsStage::Fit, 62, 64, false),
        (FixtureRunsStage::DecisionResidual, 0, 3, false),
    ];
    for (stage, start, n, valid) in fit_cases {
        charge(&mut counts.contexts, 21)?;
        let error = FixtureRunsError::fit(stage, start, n, fit_cause());
        assert_eq!(error.to_string(), expected_context_display(stage, valid));
        if valid {
            assert_eq!(error.stage(), stage);
            assert_eq!(error.candidate_start(), Some(start));
            assert_eq!(error.point_index(), None);
            let source = error
                .source()
                .ok_or_else(|| io::Error::other("fit source"))?;
            assert_eq!(source.downcast_ref::<FitError>(), Some(&fit_cause()));
            assert!(source.source().is_none());
        } else {
            assert_eq!(error.stage(), FixtureRunsStage::Invariant);
            assert_eq!((error.candidate_start(), error.point_index()), (None, None));
            assert!(error.source().is_none());
        }
    }
    let residual_cases = [
        (FixtureRunsStage::DecisionResidual, 33, 2, Some(2), 64, true),
        (
            FixtureRunsStage::DecisionResidual,
            33,
            2,
            Some(63),
            64,
            true,
        ),
        (FixtureRunsStage::StatisticsRow, 33, 2, Some(33), 64, true),
        (FixtureRunsStage::StatisticsRow, 33, 2, Some(63), 64, true),
        (FixtureRunsStage::StatisticsSummary, 33, 2, None, 64, true),
        (
            FixtureRunsStage::DecisionResidual,
            33,
            2,
            Some(1),
            64,
            false,
        ),
        (
            FixtureRunsStage::DecisionResidual,
            33,
            2,
            Some(64),
            64,
            false,
        ),
        (FixtureRunsStage::StatisticsRow, 33, 2, Some(32), 64, false),
        (FixtureRunsStage::StatisticsRow, 33, 2, Some(64), 64, false),
        (
            FixtureRunsStage::StatisticsSummary,
            33,
            2,
            Some(33),
            64,
            false,
        ),
        (
            FixtureRunsStage::DecisionResidual,
            33,
            34,
            Some(34),
            64,
            false,
        ),
        (
            FixtureRunsStage::DecisionResidual,
            62,
            2,
            Some(63),
            64,
            false,
        ),
        (FixtureRunsStage::DecisionResidual, 0, 0, Some(0), 2, false),
        (FixtureRunsStage::DecisionResidual, 0, 0, Some(0), 65, false),
        (FixtureRunsStage::Fit, 0, 0, Some(0), 64, false),
    ];
    for (stage, start, window, point, n, valid) in residual_cases {
        charge(&mut counts.contexts, 21)?;
        let error = FixtureRunsError::residual(stage, start, window, point, n, residual_cause());
        assert_eq!(error.to_string(), expected_context_display(stage, valid));
        if valid {
            assert_eq!(
                (error.stage(), error.candidate_start(), error.point_index()),
                (stage, Some(start), point)
            );
            let source = error
                .source()
                .ok_or_else(|| io::Error::other("residual source"))?;
            assert_eq!(
                source.downcast_ref::<ResidualError>(),
                Some(&residual_cause())
            );
            let nested = source
                .source()
                .ok_or_else(|| io::Error::other("nested fit source"))?;
            assert_eq!(nested.downcast_ref::<FitError>(), Some(&fit_cause()));
            assert!(nested.source().is_none());
        } else {
            assert_eq!(error.stage(), FixtureRunsStage::Invariant);
            assert_eq!((error.candidate_start(), error.point_index()), (None, None));
            assert!(error.source().is_none());
        }
    }
    assert_eq!(counts.contexts, 21);
    Ok(())
}

/// Raw immutable point bits, including signed-zero representations.
fn point_bits(point: &FitPoint) -> [u64; 3] {
    [
        point.relative_seconds().to_bits(),
        point.phase_seconds().to_bits(),
        point.variance_divisor().to_bits(),
    ]
}

/// Count actual guard-passage and residual hooks without changing their owner.
#[derive(Default)]
struct CandidateObserver {
    /// Number of guard-passage hooks reached by one preparation.
    before_calls: usize,
    /// The most recently observed original candidate start.
    before_start: Option<usize>,
    /// Number of actual residual-store hooks reached by one preparation.
    residual_calls: usize,
}

impl RunsObserver for CandidateObserver {
    /// Record the actual post-guard point without changing preparation.
    fn before_suffix(&mut self, start: usize) {
        self.before_calls = self.before_calls.saturating_add(1);
        self.before_start = Some(start);
    }

    /// Count the actual stored residual without changing its cache slot.
    fn observe(&mut self, _start: usize, _index: usize, _residual: &f64) {
        self.residual_calls = self.residual_calls.saturating_add(1);
    }
}

/// Check one invalid start against the same admitted 64-point input.
///
/// Charges the preparation attempt before its owner, observes the actual guard
/// boundary and retains the typed rejection. The borrowed input cannot change.
/// # Errors
/// Propagates an exhausted local preparation allowance or unexpected admission.
/// # Panics
/// Rejects an incorrect fixture, reached suffix work or changed error contract.
fn check_invalid_candidate_start(input: &FitInput, counts: &mut Counts) -> TestResult {
    assert_eq!(input.sample_count(), 64);
    charge(&mut counts.prepares, 5)?;
    let mut capture = CandidateObserver::default();
    let invalid = {
        let mut observer: Option<&mut dyn RunsObserver> = Some(&mut capture);
        TerminalCandidate::prepare(input, 62, &mut observer)
    };
    assert_eq!(
        (
            capture.before_calls,
            capture.before_start,
            capture.residual_calls
        ),
        (0, None, 0)
    );
    let Err(error) = invalid else {
        return Err(io::Error::other("invalid candidate start was admitted").into());
    };
    assert_eq!(error.stage(), FixtureRunsStage::Invariant);
    assert_eq!((error.candidate_start(), error.point_index()), (None, None));
    assert!(error.source().is_none());
    Ok(())
}

/// Prepare four valid candidates and one invalid start from a checked fixture.
/// # Errors
/// Preserves local accounting and actual public input/private preparation errors.
/// # Panics
/// Rejects an incorrect checked window, suffix alignment or mutation of input.
#[test]
fn runs_private_candidate_windows_observe_original_index_boundaries() -> TestResult {
    let mut counts = Counts::default();
    charge(&mut counts.points, 64)?;
    let first = FitPoint::new(-63.0, 0.25, 1.0)?;
    let mut points = [first; 64];
    for (slot, x) in points.iter_mut().skip(1).zip(-62_i32..=0) {
        charge(&mut counts.points, 64)?;
        *slot = FitPoint::new(f64::from(x), 0.25, 1.0)?;
    }
    let before = points.map(|point| point_bits(&point));
    let frame = NonZeroU64::new(17).ok_or_else(|| io::Error::other("window frame"))?;
    charge(&mut counts.inputs, 1)?;
    let input = FitInput::new(frame, &points)?;
    let cases = [
        (0, 64, 0, 64),
        (32, 32, 0, 64),
        (33, 31, 2, 62),
        (61, 3, 58, 6),
    ];
    for (start, q, r, length) in cases {
        charge(&mut counts.prepares, 5)?;
        let mut capture = CandidateObserver::default();
        let candidate = {
            let mut observer: Option<&mut dyn RunsObserver> = Some(&mut capture);
            TerminalCandidate::prepare(&input, start, &mut observer)?
        };
        assert_eq!(
            (capture.before_calls, capture.before_start),
            (1, Some(start))
        );
        assert_eq!(capture.residual_calls, length);
        assert_eq!(
            (
                candidate.start,
                candidate.suffix().sample_count(),
                candidate.window_start,
                candidate.window_len
            ),
            (start, q, r, length)
        );
        assert_eq!(candidate.suffix().frame(), frame);
        assert_eq!(candidate.line().frame(), frame);
        let window = candidate
            .window()
            .ok_or_else(|| io::Error::other("missing window"))?;
        let fit = candidate
            .fit_window()
            .ok_or_else(|| io::Error::other("missing fit window"))?;
        counts.returned_window_rows = counts
            .returned_window_rows
            .checked_add(window.len())
            .ok_or_else(|| io::Error::other("window count overflow"))?;
        assert_eq!((window.len(), fit.len()), (length, q));
        assert!(
            window
                .iter()
                .all(|value| value.to_bits().trailing_zeros() >= 63)
        );
        let (first_index, first_value) = candidate
            .suffix_residual(0)
            .ok_or_else(|| io::Error::other("missing first suffix residual"))?;
        let (last_index, last_value) = candidate
            .suffix_residual(q - 1)
            .ok_or_else(|| io::Error::other("missing last suffix residual"))?;
        assert_eq!((first_index, last_index), (start, 63));
        assert!(first_value.to_bits().trailing_zeros() >= 63);
        assert!(last_value.to_bits().trailing_zeros() >= 63);
        assert!(candidate.suffix_residual(q).is_none());
        assert!(candidate.suffix_residual(usize::MAX).is_none());
        let inconsistent = TerminalCandidate {
            window_len: 0,
            ..candidate
        };
        assert!(inconsistent.suffix_residual(0).is_none());
    }
    check_invalid_candidate_start(&input, &mut counts)?;
    assert_eq!((input.frame(), input.sample_count()), (frame, 64));
    assert_eq!(points.map(|point| point_bits(&point)), before);
    let stored: &[FitPoint; 64] = input.batch().values.try_into()?;
    assert_eq!(stored.map(|point| point_bits(&point)), before);
    assert_eq!(
        (
            counts.points,
            counts.inputs,
            counts.prepares,
            counts.direct_fits,
            counts.selectors,
            counts.returned_window_rows
        ),
        (64, 1, 5, 0, 0, 196)
    );
    assert_eq!((input.frame(), input.sample_count()), (frame, 64));
    assert_eq!(points.map(|point| point_bits(&point)), before);
    let stored: &[FitPoint; 64] = input.batch().values.try_into()?;
    assert_eq!(stored.map(|point| point_bits(&point)), before);
    Ok(())
}

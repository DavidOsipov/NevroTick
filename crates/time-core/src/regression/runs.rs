// SPDX-License-Identifier: GPL-2.0-only
/*
 chronyd/chronyc - Programs for keeping computer clocks accurate.

**********************************************************************
* Copyright (C) Richard P. Curnow  1997-2003
* Copyright (C) Miroslav Lichvar  2011, 2016-2017
*
* This program is free software; you can redistribute it and/or modify
* it under the terms of version 2 of the GNU General Public License as
* published by the Free Software Foundation.
*
* This program is distributed in the hope that it will be useful, but
* WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
* General Public License for more details.
*
* You should have received a copy of the GNU General Public License along
* with this program; if not, write to the Free Software Foundation, Inc.,
* 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
*
**********************************************************************
*/
//! Bounded fixture residual runs and suffix selection.
//!
//! Modified by David Osipov with AI assistance, 30 October 2026. This adapts
//! Chrony 4.9 `regress.c` table and selected runs/start-index statements at
//! revision 8df4f1263e206585e6c4e61352d042c2dd12916a. The m=0 fixture window
//! may include original points dropped from the fit. No production sample,
//! history pruning, physical uncertainty or clock authority is supplied.

use std::{fmt, num::NonZeroU64};

use super::{
    FitError, FitInput, FixtureLine, FixtureResidualStatistics, ResidualError, fit,
    residuals::{TerminalStatisticsError, checked_residual, statistics_from_terminal},
};

/// Critical runs indexed by window length; only 3..=64 is admitted for use.
const CRITICAL_RUNS: [u8; 65] = [
    0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 3, 3, 4, 4, 5, 5, 5, 6, 6, 7, 7, 7, 8, 8, 9, 9, 9, 10, 10, 11,
    11, 11, 12, 12, 13, 13, 14, 14, 14, 15, 15, 16, 16, 17, 17, 18, 18, 18, 19, 19, 20, 20, 21, 21,
    21, 22, 22, 23, 23, 24, 24, 25, 25, 26, 26,
];

/// Test-owned observer of candidate admission and stored residuals.
///
/// The qualified test implementation records bounded synthetic values only;
/// it cannot change the borrowed slot or return a selection decision.
#[cfg(test)]
trait RunsObserver {
    /// Observe passage of the early guard, before arithmetic or batch access.
    fn before_suffix(&mut self, _start: usize) {}

    /// Observe the actual stored slot at its candidate and original indices.
    #[expect(
        clippy::trivially_copy_pass_by_ref,
        reason = "the test observer borrows the actual checked cache slot before its consumer"
    )]
    fn observe(&mut self, start: usize, index: usize, residual: &f64);
}

/// Reason the fixture selector stopped on its returned suffix.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureRunsStop {
    /// The decision-window runs exceeded the applicable critical value.
    RunsPassed,
    /// The fixture minimum stopped a decision that did not pass.
    MinimumFallback,
}

/// Stage at which the fixture runs operation rejected a request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureRunsStage {
    /// The explicit fixture minimum was outside the input's supported range.
    Minimum,
    /// Checked suffix construction or geometry failed.
    Suffix,
    /// The candidate suffix's fit failed.
    Fit,
    /// A decision-window residual failed.
    DecisionResidual,
    /// A terminal weighted-residual row failed.
    StatisticsRow,
    /// A terminal aggregate or statistic admission failed.
    StatisticsSummary,
    /// An internal checked index or context invariant failed.
    Invariant,
}

/// Opaque, bounded fixture failure retaining actual typed causes and positions.
///
/// Private constructors enforce stage-specific positions in the original input;
/// no raw scalar, source endpoint or partial result is retained.
#[derive(Debug)]
pub struct FixtureRunsError {
    /// Closed stage of the failure.
    stage: FixtureRunsStage,
    /// Original-input candidate start, when established.
    candidate_start: Option<usize>,
    /// Original-input point index, when established for a row.
    point_index: Option<usize>,
    /// Actual checked fit cause, if one exists.
    fit_source: Option<FitError>,
    /// Actual checked residual cause, if one exists.
    residual_source: Option<ResidualError>,
}

impl FixtureRunsError {
    /// Construct a context-free stage; only minimum and invariant are accepted.
    ///
    /// Any other stage becomes invariant, without inventing a source or index.
    fn plain(stage: FixtureRunsStage) -> Self {
        let stage = match stage {
            FixtureRunsStage::Minimum => FixtureRunsStage::Minimum,
            _ => FixtureRunsStage::Invariant,
        };
        Self {
            stage,
            candidate_start: None,
            point_index: None,
            fit_source: None,
            residual_source: None,
        }
    }

    /// Preserve a fit cause only when the original candidate context is valid.
    ///
    /// Invalid stages or bounds produce a context-free invariant failure.
    fn fit(stage: FixtureRunsStage, start: usize, n: usize, source: FitError) -> Self {
        if !(3..=64).contains(&n)
            || start > n - 3
            || !matches!(stage, FixtureRunsStage::Suffix | FixtureRunsStage::Fit)
        {
            return Self::plain(FixtureRunsStage::Invariant);
        }
        Self {
            stage,
            candidate_start: Some(start),
            point_index: None,
            fit_source: Some(source),
            residual_source: None,
        }
    }

    /// Preserve a residual cause only with the appropriate bounded row context.
    ///
    /// Decision indices belong to the computed original-input window, terminal
    /// row indices to the selected suffix; aggregate failures have no row.
    fn residual(
        stage: FixtureRunsStage,
        start: usize,
        window_start: usize,
        point: Option<usize>,
        n: usize,
        source: ResidualError,
    ) -> Self {
        if !(3..=64).contains(&n) || start > n - 3 || window_start > start {
            return Self::plain(FixtureRunsStage::Invariant);
        }
        let valid = match (stage, point) {
            (FixtureRunsStage::DecisionResidual, Some(index)) => (window_start..n).contains(&index),
            (FixtureRunsStage::StatisticsRow, Some(index)) => (start..n).contains(&index),
            (FixtureRunsStage::StatisticsSummary, None) => true,
            _ => false,
        };
        if !valid {
            return Self::plain(FixtureRunsStage::Invariant);
        }
        Self {
            stage,
            candidate_start: Some(start),
            point_index: point,
            fit_source: None,
            residual_source: Some(source),
        }
    }

    /// Return the closed failure stage without effects.
    #[must_use]
    pub const fn stage(&self) -> FixtureRunsStage {
        self.stage
    }

    /// Return an established original-input candidate start, if any.
    #[must_use]
    pub const fn candidate_start(&self) -> Option<usize> {
        self.candidate_start
    }

    /// Return an established original-input point index, if any.
    #[must_use]
    pub const fn point_index(&self) -> Option<usize> {
        self.point_index
    }
}

impl fmt::Display for FixtureRunsError {
    /// Format only the closed stage, without numerical inputs or reports.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "fixture runs: {:?}", self.stage)
    }
}

impl std::error::Error for FixtureRunsError {
    /// Borrow the actual typed cause, including all public nested variants.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match (&self.fit_source, &self.residual_source) {
            (Some(source), _) => Some(source),
            (None, Some(source)) => Some(source),
            (None, None) => None,
        }
    }
}

/// Immutable selected fixture suffix and its same-fit residual diagnostics.
///
/// Private construction follows a completed decision and statistics. A fixture
/// frame is an assertion, not an absolute monotonic reference.
#[derive(Debug)]
pub struct FixtureRunsSelection {
    /// Selected suffix start in the original input.
    start: usize,
    /// Selected suffix size.
    sample_count: usize,
    /// Caller-asserted fixture frame.
    frame: NonZeroU64,
    /// Explicit terminal reason.
    stop: FixtureRunsStop,
    /// Original-input start of the decision window.
    decision_start: usize,
    /// Number of points in the decision window.
    decision_count: usize,
    /// Runs counted over the decision window.
    decision_runs: usize,
    /// Runs counted over the fitted suffix only.
    fit_runs: usize,
    /// Diagnostics belonging to the same fitted suffix and line.
    statistics: FixtureResidualStatistics,
}

impl FixtureRunsSelection {
    /// Return the selected start in the original input.
    #[must_use]
    pub const fn start(&self) -> usize {
        self.start
    }
    /// Return the selected suffix's sample count.
    #[must_use]
    pub const fn sample_count(&self) -> usize {
        self.sample_count
    }
    /// Return the unchanged asserted fixture frame.
    #[must_use]
    pub const fn frame(&self) -> NonZeroU64 {
        self.frame
    }
    /// Return the declared terminal reason.
    #[must_use]
    pub const fn stop(&self) -> FixtureRunsStop {
        self.stop
    }
    /// Return the decision window's start in the original input.
    #[must_use]
    pub const fn decision_start(&self) -> usize {
        self.decision_start
    }
    /// Return the number of points in the decision window.
    #[must_use]
    pub const fn decision_count(&self) -> usize {
        self.decision_count
    }
    /// Return the decision-window run count.
    #[must_use]
    pub const fn decision_runs(&self) -> usize {
        self.decision_runs
    }
    /// Return the fitted-suffix run count.
    #[must_use]
    pub const fn fit_runs(&self) -> usize {
        self.fit_runs
    }
    /// Borrow the selected suffix's associated fixture diagnostics.
    #[must_use]
    pub const fn statistics(&self) -> &FixtureResidualStatistics {
        &self.statistics
    }
}

/// One complete same-fit candidate with a fully evaluated decision-window cache.
///
/// Only this module constructs it from original checked input and a bounded start.
/// The suffix, line and completed cache cannot be rebound independently.
pub(super) struct TerminalCandidate {
    /// Admitted suffix owning the exact geometry and points used by the fit.
    suffix: FitInput,
    /// The unique line fitted to the suffix.
    line: FixtureLine,
    /// Original-input suffix start.
    start: usize,
    /// Original-input decision-window start.
    window_start: usize,
    /// Computed prefix length, 3..=64.
    window_len: usize,
    /// Only the computed prefix is observable; remaining slots are initialized.
    residuals: [f64; 64],
}

impl TerminalCandidate {
    /// Prepare a complete candidate without publishing a partially computed one.
    ///
    /// Input is immutable; bounded checks precede slicing and all arithmetic.
    /// At most one suffix construction, one fit and 64 residuals are performed.
    /// # Errors
    /// Preserves first actual suffix, fit or decision-row failure and context.
    fn prepare(
        input: &FitInput,
        start: usize,
        #[cfg(test)] observer: &mut Option<&mut dyn RunsObserver>,
    ) -> Result<Self, FixtureRunsError> {
        let n = input.sample_count();
        if !(3..=64).contains(&n) || start > n - 3 {
            return Err(FixtureRunsError::plain(FixtureRunsStage::Invariant));
        }
        #[cfg(test)]
        if let Some(observer) = observer.as_deref_mut() {
            observer.before_suffix(start);
        }
        let q = n
            .checked_sub(start)
            .ok_or_else(|| FixtureRunsError::plain(FixtureRunsStage::Invariant))?;
        let twice_q = q
            .checked_mul(2)
            .ok_or_else(|| FixtureRunsError::plain(FixtureRunsStage::Invariant))?;
        let window_start = n.saturating_sub(twice_q);
        let points = input.batch();
        let suffix_points = points
            .values
            .get(start..n)
            .ok_or_else(|| FixtureRunsError::plain(FixtureRunsStage::Invariant))?;
        let suffix = FitInput::new(input.frame(), suffix_points)
            .map_err(|source| FixtureRunsError::fit(FixtureRunsStage::Suffix, start, n, source))?;
        let line = fit(&suffix)
            .map_err(|source| FixtureRunsError::fit(FixtureRunsStage::Fit, start, n, source))?;
        let window = points
            .values
            .get(window_start..n)
            .ok_or_else(|| FixtureRunsError::plain(FixtureRunsStage::Invariant))?;
        let mut residuals = [0.0; 64];
        for (offset, point) in window.iter().enumerate() {
            let index = window_start
                .checked_add(offset)
                .ok_or_else(|| FixtureRunsError::plain(FixtureRunsStage::Invariant))?;
            let residual = checked_residual(point, &line).map_err(|source| {
                FixtureRunsError::residual(
                    FixtureRunsStage::DecisionResidual,
                    start,
                    window_start,
                    Some(index),
                    n,
                    source,
                )
            })?;
            let slot = residuals
                .get_mut(offset)
                .ok_or_else(|| FixtureRunsError::plain(FixtureRunsStage::Invariant))?;
            *slot = residual;
            #[cfg(test)]
            if let Some(observer) = observer.as_deref_mut() {
                observer.observe(start, index, slot);
            }
        }
        Ok(Self {
            suffix,
            line,
            start,
            window_start,
            window_len: window.len(),
            residuals,
        })
    }

    /// Borrow the admitted same-fit suffix without mutation.
    pub(super) const fn suffix(&self) -> &FitInput {
        &self.suffix
    }

    /// Copy the unique checked line used to populate this candidate's cache.
    pub(super) const fn line(&self) -> FixtureLine {
        self.line
    }

    /// Project one checked suffix offset to its original index and cached value.
    ///
    /// Unsupported offsets or an inconsistent window return None without effects.
    pub(super) fn suffix_residual(&self, offset: usize) -> Option<(usize, f64)> {
        if offset >= self.suffix.sample_count() {
            return None;
        }
        let index = self.start.checked_add(offset)?;
        let cache_index = index.checked_sub(self.window_start)?;
        if cache_index >= self.window_len {
            return None;
        }
        self.residuals
            .get(cache_index)
            .copied()
            .map(|value| (index, value))
    }

    /// Borrow the complete checked decision-window residual prefix.
    fn window(&self) -> Option<&[f64]> {
        self.residuals.get(..self.window_len)
    }

    /// Borrow the suffix-aligned portion of the same checked residual cache.
    fn fit_window(&self) -> Option<&[f64]> {
        let offset = self.start.checked_sub(self.window_start)?;
        self.residuals.get(offset..self.window_len)
    }
}

/// Count runs in a nonempty bounded checked residual view.
///
/// Each zero-involving adjacency starts a new run. The caller supplies a view
/// of checked residuals; this helper has no effects or numeric transformations.
fn count_runs(values: &[f64]) -> Option<usize> {
    if values.is_empty() || values.len() > 64 {
        return None;
    }
    let mut runs = 1usize;
    for pair in values.windows(2) {
        let same = (pair[0] < 0.0 && pair[1] < 0.0) || (pair[0] > 0.0 && pair[1] > 0.0);
        if !same {
            runs = runs.checked_add(1)?;
        }
    }
    Some(runs)
}

/// Retrieve the pinned threshold for an admitted fixture-window count.
fn critical_runs(count: usize) -> Option<usize> {
    if !(3..=64).contains(&count) {
        return None;
    }
    CRITICAL_RUNS.get(count).copied().map(usize::from)
}

/// Select a bounded fixture regression suffix with checked residual runs.
///
/// The input and its newest-relative zero remain unchanged. The explicit minimum
/// is a fixture value, not source-quality or history policy. Every candidate owns
/// one fit and a bounded ratio-two original-input residual window. Terminal
/// statistics use that same fit and cache; no history or external effect occurs.
/// # Errors
/// Rejects an invalid minimum first, then the first suffix, fit, residual,
/// statistics or internal-context failure, without publishing a partial result.
pub fn select_fixture_runs(
    input: &FitInput,
    min_samples: usize,
) -> Result<FixtureRunsSelection, FixtureRunsError> {
    #[cfg(test)]
    let mut observer = None;
    select_fixture_runs_inner(
        input,
        min_samples,
        #[cfg(test)]
        &mut observer,
    )
}

/// Invoke the same selector body with a private test-owned observer.
///
/// The observer receives only immutable cache slots after successful storage.
/// # Errors
/// Preserves the same selector failures and ordering as the ordinary wrapper.
#[cfg(test)]
fn select_fixture_runs_observed(
    input: &FitInput,
    min_samples: usize,
    observer: &mut impl RunsObserver,
) -> Result<FixtureRunsSelection, FixtureRunsError> {
    let mut observer: Option<&mut dyn RunsObserver> = Some(observer);
    select_fixture_runs_inner(input, min_samples, &mut observer)
}

/// Perform the common bounded selector, with test observation compiled out otherwise.
///
/// The input is immutable and only a completed same-fit result is returned.
/// # Errors
/// Preserves the first minimum, candidate, residual or result failure.
fn select_fixture_runs_inner(
    input: &FitInput,
    min_samples: usize,
    #[cfg(test)] observer: &mut Option<&mut dyn RunsObserver>,
) -> Result<FixtureRunsSelection, FixtureRunsError> {
    let n = input.sample_count();
    if !(3..=n).contains(&min_samples) {
        return Err(FixtureRunsError::plain(FixtureRunsStage::Minimum));
    }
    let mut start = 0usize;
    loop {
        let candidate = TerminalCandidate::prepare(
            input,
            start,
            #[cfg(test)]
            observer,
        )?;
        let window = candidate
            .window()
            .ok_or_else(|| FixtureRunsError::plain(FixtureRunsStage::Invariant))?;
        let decision_runs = count_runs(window)
            .ok_or_else(|| FixtureRunsError::plain(FixtureRunsStage::Invariant))?;
        let threshold = critical_runs(window.len())
            .ok_or_else(|| FixtureRunsError::plain(FixtureRunsStage::Invariant))?;
        let q = candidate.suffix.sample_count();
        let stop = if decision_runs > threshold {
            Some(FixtureRunsStop::RunsPassed)
        } else if q == min_samples {
            Some(FixtureRunsStop::MinimumFallback)
        } else {
            None
        };
        if let Some(stop) = stop {
            let fit_runs = candidate
                .fit_window()
                .and_then(count_runs)
                .ok_or_else(|| FixtureRunsError::plain(FixtureRunsStage::Invariant))?;
            let statistics = statistics_from_terminal(&candidate).map_err(|error| match error {
                TerminalStatisticsError::Row { index, source } => FixtureRunsError::residual(
                    FixtureRunsStage::StatisticsRow,
                    start,
                    candidate.window_start,
                    Some(index),
                    n,
                    source,
                ),
                TerminalStatisticsError::Summary(source) => FixtureRunsError::residual(
                    FixtureRunsStage::StatisticsSummary,
                    start,
                    candidate.window_start,
                    None,
                    n,
                    source,
                ),
                TerminalStatisticsError::Invariant => {
                    FixtureRunsError::plain(FixtureRunsStage::Invariant)
                }
            })?;
            return Ok(FixtureRunsSelection {
                start,
                sample_count: q,
                frame: input.frame(),
                stop,
                decision_start: candidate.window_start,
                decision_count: candidate.window_len,
                decision_runs,
                fit_runs,
                statistics,
            });
        }
        start = start
            .checked_add(1)
            .ok_or_else(|| FixtureRunsError::plain(FixtureRunsStage::Invariant))?;
    }
}

/// Private test-owned observer and independent synthetic assertions.
#[cfg(test)]
#[path = "../../tests/private/runs_sign_observer.rs"]
mod runs_sign_observer;

/// Private controls for bounded runs helpers and candidate windows.
#[cfg(test)]
#[path = "../../tests/private/runs_guards.rs"]
mod runs_guards;

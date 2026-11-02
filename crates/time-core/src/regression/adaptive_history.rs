// SPDX-License-Identifier: GPL-2.0-only
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2003
 * Copyright (C) Miroslav Lichvar  2011-2014, 2016-2018, 2021
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
//! One failure-atomic adaptive-history caller over the shared numerical owners.
//!
//! Modified by David Osipov with AI assistance, 2 November 2026. The
//! chronological append, successful pruning and retained-runs caller adapts
//! `sourcestats.c` 270-329 and 556-598, Chrony 4.9 revision
//! `8df4f1263e206585e6c4e61352d042c2dd12916a`. The additive ledger and
//! `../../PROVENANCE.md` retain scope, notices and combined-capacity deviations.
//! The existing normalizer and retained-prefix selector remain the numerical
//! owners. No ring indexing, source-quality weighting, physical uncertainty,
//! production eligibility, correction accounting or clock authority is adopted.
//!
//! Append checks context and exact time order before staging bounded eviction.
//! One normalization covers all auxiliary and fitted rows; one retained-prefix
//! selection supplies the actual terminal fit and statistics without refitting.
//! Only its complete decision window is retained. Complete transformed-state
//! checks precede one publication, preserving the old owner on every failure.
//! No smaller-window retry, observation refresh or external effect is provided.
//!
//! One retained 64-cell array coexists with one staged 64-cell array and the
//! existing normalization/selection scratch. Explicit observation writes comprise
//! 64 initialization slots, at most 63 retained copies and at most 63 compaction
//! copies. Value moves and physical compiler copies/stack/RSS are not measured.
//! Each append makes at most one normalization and one selector entry, whose
//! existing bound is 62 candidate fits. No direct fit or statistic call is added.

use std::{cmp::Ordering, fmt};

use garde::Validate;
use time_domain::MonotonicNs;

use super::{
    FixtureContext, FixtureNormalization, FixtureObservation, FixtureRunsError,
    FixtureRunsSelection, FixtureRunsStop, MAX_POINTS, MIN_POINTS, NormalizationError,
    prepare_fixture_input, select_fixture_runs_with_retained,
};

/// Closed construction or append failure without observation or scalar payloads.
#[derive(Debug)]
pub enum AdaptiveHistoryError {
    /// The explicit immutable fixture minimum lies outside three through 64.
    InvalidMinimum,
    /// Incoming labels differ from the immutable whole history context.
    ContextMismatch,
    /// The incoming exact monotonic time equals the newest committed time.
    DuplicateTime,
    /// The incoming exact monotonic time precedes the newest committed time.
    OutOfOrderTime,
    /// A private state or validation relationship failed before publication.
    StateInvariant,
    /// The complete staged combined input failed existing normalization.
    Normalization {
        /// The actual bounded preparation cause, with staged combined indices.
        source: NormalizationError,
    },
    /// The real retained-prefix selector rejected before publication.
    Runs {
        /// The actual bounded selection cause, with staged combined indices.
        source: FixtureRunsError,
    },
}

impl fmt::Display for AdaptiveHistoryError {
    /// Format fixed context and existing redacted causes without input payloads.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMinimum => formatter.write_str("adaptive history minimum"),
            Self::ContextMismatch => formatter.write_str("adaptive history context mismatch"),
            Self::DuplicateTime => formatter.write_str("adaptive history duplicate time"),
            Self::OutOfOrderTime => formatter.write_str("adaptive history out-of-order time"),
            Self::StateInvariant => formatter.write_str("adaptive history state invariant"),
            Self::Normalization { source } => {
                write!(formatter, "adaptive history normalization: {source}")
            }
            Self::Runs { source } => write!(formatter, "adaptive history runs: {source}"),
        }
    }
}

impl std::error::Error for AdaptiveHistoryError {
    /// Borrow only the actual nested cause; closed admission failures have none.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Normalization { source } => Some(source),
            Self::Runs { source } => Some(source),
            Self::InvalidMinimum
            | Self::ContextMismatch
            | Self::DuplicateTime
            | Self::OutOfOrderTime
            | Self::StateInvariant => None,
        }
    }
}

/// Test the explicit fixture minimum without normalization or state effects.
fn minimum_valid(value: usize) -> bool {
    (MIN_POINTS..=MAX_POINTS).contains(&value)
}

/// Invoke the same minimum predicate inside mandatory private validation.
/// # Errors
/// Reject unsupported values with fixed text and no retained raw payload.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rule signature borrows its scalar field and context"
)]
fn minimum_rule(value: &usize, (): &()) -> garde::Result {
    if minimum_valid(*value) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid adaptive history minimum"))
    }
}

/// The constructor's sole raw scalar requiring mandatory range admission.
#[derive(Validate)]
struct RawMinimum {
    /// Explicit caller-supplied fixture minimum; no product default is selected.
    #[garde(custom(minimum_rule))]
    value: usize,
}

/// Immutable association of actual preparation and its terminal runs selection.
///
/// Selection indices describe the combined input before successful pruning.
/// They are not indices into subsequently retained fitted or decision rows.
/// No public constructor, mutable field or separately replaceable statistic is
/// supplied; the owning history transaction publishes the whole report.
pub struct FixtureAdaptiveHistoryReport {
    /// Complete immutable context of the actual prepared combined input.
    context: FixtureContext,
    /// Exact newest observation used as the preparation reference.
    reference: MonotonicNs,
    /// The history's immutable explicit fixture pruning minimum.
    configured_minimum: usize,
    /// The actual current-row minimum used by the selector for this operation.
    effective_minimum: usize,
    /// Owned terminal selection, including its unchanged same-fit statistics.
    selection: FixtureRunsSelection,
}

impl fmt::Debug for FixtureAdaptiveHistoryReport {
    /// Expose only context and bounded count/status metadata, never fitted values.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FixtureAdaptiveHistoryReport")
            .field("context", &self.context)
            .field("configured_minimum", &self.configured_minimum)
            .field("effective_minimum", &self.effective_minimum)
            .field("sample_count", &self.selection.sample_count())
            .field("stop", &self.selection.stop())
            .finish_non_exhaustive()
    }
}

impl FixtureAdaptiveHistoryReport {
    /// Return the unchanged asserted context without adding provenance authority.
    #[must_use]
    pub const fn context(&self) -> FixtureContext {
        self.context
    }

    /// Return the actual exact preparation reference without refreshing evidence.
    #[must_use]
    pub const fn reference(&self) -> MonotonicNs {
        self.reference
    }

    /// Return the explicit immutable fixture pruning minimum, not eligibility.
    #[must_use]
    pub const fn configured_minimum(&self) -> usize {
        self.configured_minimum
    }

    /// Return the declared minimum used by this operation's actual selector.
    #[must_use]
    pub const fn effective_minimum(&self) -> usize {
        self.effective_minimum
    }

    /// Borrow the same owned terminal selection with its pre-prune input indices.
    #[must_use]
    pub const fn selection(&self) -> &FixtureRunsSelection {
        &self.selection
    }
}

/// One complete commit unit; auxiliary and fitted rows share one fixed array.
///
/// A freshly assembled private candidate has not yet established the following
/// relationships. `validate_complete` admits them before `try_append` publishes
/// the candidate; merely initializing this structure grants no checked status.
///
/// The admitted combined prefix is chronological and contains auxiliary rows
/// before current fitted rows. Its total is at most 64 and auxiliary count does
/// not exceed fitted count. Fewer than three fitted rows have no auxiliary rows
/// or report. Every sufficient state owns its same-operation terminal report.
struct CommittedAdaptiveState {
    /// Fixed combined storage; unused initialized slots are never observable.
    observations: [FixtureObservation; MAX_POINTS],
    /// Number of decision-only oldest rows before the fitted suffix.
    auxiliary_count: usize,
    /// Number of current fitted rows, excluding the auxiliary prefix.
    sample_count: usize,
    /// Whole same-operation report, absent before three current rows exist.
    report: Option<FixtureAdaptiveHistoryReport>,
}

/// Raw staged append rows before normalization, selection and output validation.
///
/// One fixed array owns the candidate combined prefix. Its context/order and
/// count relationships have passed append admission, but it has no report or
/// committed-history status until numerical completion and final validation.
struct StagedAppend {
    /// Initialized combined storage, with no independent history owner.
    observations: [FixtureObservation; MAX_POINTS],
    /// Auxiliary prefix remaining after any staged capacity eviction.
    auxiliary_count: usize,
    /// Current fitted suffix including the actual incoming observation.
    sample_count: usize,
}

/// Borrow only a nonempty bounded combined prefix with a valid fitted suffix.
///
/// Counts are checked before slicing; no row is inspected, copied or changed.
/// # Errors
/// Reject zero/oversized fitted counts, oversized auxiliary counts or bad sums.
fn combined_prefix(
    observations: &[FixtureObservation; MAX_POINTS],
    auxiliary_count: usize,
    sample_count: usize,
) -> Result<&[FixtureObservation], AdaptiveHistoryError> {
    if !(1..=MAX_POINTS).contains(&sample_count) || auxiliary_count > sample_count {
        return Err(AdaptiveHistoryError::StateInvariant);
    }
    let count = auxiliary_count
        .checked_add(sample_count)
        .ok_or(AdaptiveHistoryError::StateInvariant)?;
    observations
        .get(..count)
        .ok_or(AdaptiveHistoryError::StateInvariant)
}

/// Compact one admitted decision window in its existing staged array.
///
/// Forward copies preserve overlap because every source is at or after its
/// destination. A zero start performs no copies; other windows copy at most 63
/// observations. No live history or numeric result is changed.
/// # Errors
/// Reject invalid window/index relationships before any live-state publication.
fn retain_window(
    observations: &mut [FixtureObservation; MAX_POINTS],
    start: usize,
    end: usize,
) -> Result<(), AdaptiveHistoryError> {
    let count = observations
        .get(start..end)
        .ok_or(AdaptiveHistoryError::StateInvariant)?
        .len();
    if start == 0 {
        return Ok(());
    }
    for destination in 0..count {
        let source = start
            .checked_add(destination)
            .ok_or(AdaptiveHistoryError::StateInvariant)?;
        let observation = observations
            .get(source)
            .copied()
            .ok_or(AdaptiveHistoryError::StateInvariant)?;
        *observations
            .get_mut(destination)
            .ok_or(AdaptiveHistoryError::StateInvariant)? = observation;
    }
    Ok(())
}

/// Validate the transformed complete owner against its actual append provenance.
///
/// The private input may be an unpublished candidate whose count, report and
/// provenance relationships are not yet established. Reject disagreement before
/// publication; a valid observation or selection alone admits no whole history.
///
/// At most 64 contexts and 63 adjacent time relationships are inspected. Selection
/// indices retain their original combined-input meaning, while the stored prefix
/// must be exactly its complete decision window. Immutable statistics remain
/// owned by that same selection; no fit, copy, mutation or effect occurs here.
/// # Errors
/// Reject any count, context, order, reference, minimum or report disagreement.
fn validate_complete(
    state: &CommittedAdaptiveState,
    context: FixtureContext,
    configured_minimum: usize,
    original_count: usize,
    original_auxiliary: usize,
    newest: MonotonicNs,
) -> Result<(), AdaptiveHistoryError> {
    let rows = combined_prefix(
        &state.observations,
        state.auxiliary_count,
        state.sample_count,
    )?;
    if rows.iter().any(|row| row.context() != context)
        || rows
            .windows(2)
            .any(|pair| pair[0].observed_at() >= pair[1].observed_at())
        || rows.last().map(FixtureObservation::observed_at) != Some(newest)
    {
        return Err(AdaptiveHistoryError::StateInvariant);
    }
    let Some(report) = &state.report else {
        return if state.sample_count < MIN_POINTS
            && state.auxiliary_count == 0
            && original_auxiliary == 0
            && original_count == rows.len()
        {
            Ok(())
        } else {
            Err(AdaptiveHistoryError::StateInvariant)
        };
    };
    let original_current = original_count
        .checked_sub(original_auxiliary)
        .ok_or(AdaptiveHistoryError::StateInvariant)?;
    let effective_minimum = configured_minimum.min(original_current);
    let selection = &report.selection;
    let twice_count = state
        .sample_count
        .checked_mul(2)
        .ok_or(AdaptiveHistoryError::StateInvariant)?;
    if state.sample_count < MIN_POINTS
        || state.sample_count < effective_minimum
        || report.context != context
        || report.reference != newest
        || report.configured_minimum != configured_minimum
        || report.effective_minimum != effective_minimum
        || selection.frame() != context.frame()
        || selection.statistics().frame() != context.frame()
        || selection.sample_count() != state.sample_count
        || selection.statistics().sample_count() != state.sample_count
        || selection.start() < original_auxiliary
        || selection.start().checked_add(state.sample_count) != Some(original_count)
        || selection
            .decision_start()
            .checked_add(state.auxiliary_count)
            != Some(selection.start())
        || selection.decision_count() != rows.len()
        || selection.decision_start().checked_add(rows.len()) != Some(original_count)
        || selection.decision_start() != original_count.saturating_sub(twice_count)
        || (selection.stop() == FixtureRunsStop::MinimumFallback
            && state.sample_count != effective_minimum)
    {
        return Err(AdaptiveHistoryError::StateInvariant);
    }
    Ok(())
}

impl StagedAppend {
    /// Normalize the entire candidate once and retain its actual decision window.
    ///
    /// The immutable context/minimum come from the checked history owner. Warmup
    /// owns no fit/report. Otherwise one shared selector entry uses the
    /// explicit effective minimum and returns the sole owned terminal statistics.
    /// Compaction affects only this stage, followed by complete output validation.
    /// # Errors
    /// Preserve normalization or selection rejection without retrying a smaller
    /// window; reject every internal relationship before returning a commit unit.
    fn finish(
        mut self,
        context: FixtureContext,
        configured_minimum: usize,
    ) -> Result<CommittedAdaptiveState, AdaptiveHistoryError> {
        let rows = combined_prefix(&self.observations, self.auxiliary_count, self.sample_count)?;
        let original_count = rows.len();
        let original_auxiliary = self.auxiliary_count;
        let newest = rows
            .last()
            .ok_or(AdaptiveHistoryError::StateInvariant)?
            .observed_at();
        let report = match prepare_fixture_input(context, rows)
            .map_err(|source| AdaptiveHistoryError::Normalization { source })?
        {
            FixtureNormalization::Insufficient(insufficient) => {
                if self.sample_count >= MIN_POINTS
                    || self.auxiliary_count != 0
                    || insufficient.context() != context
                    || insufficient.sample_count() != original_count
                    || insufficient.reference() != Some(newest)
                {
                    return Err(AdaptiveHistoryError::StateInvariant);
                }
                None
            }
            FixtureNormalization::Prepared(prepared) => {
                if self.sample_count < MIN_POINTS
                    || prepared.context() != context
                    || prepared.sample_count() != original_count
                    || prepared.reference() != newest
                {
                    return Err(AdaptiveHistoryError::StateInvariant);
                }
                let effective_minimum = configured_minimum.min(self.sample_count);
                let selection = select_fixture_runs_with_retained(
                    prepared.input(),
                    self.auxiliary_count,
                    effective_minimum,
                )
                .map_err(|source| AdaptiveHistoryError::Runs { source })?;
                let first = selection.decision_start();
                if first.checked_add(selection.decision_count()) != Some(original_count) {
                    return Err(AdaptiveHistoryError::StateInvariant);
                }
                self.auxiliary_count = selection
                    .start()
                    .checked_sub(first)
                    .ok_or(AdaptiveHistoryError::StateInvariant)?;
                self.sample_count = selection.sample_count();
                retain_window(&mut self.observations, first, original_count)?;
                Some(FixtureAdaptiveHistoryReport {
                    context: prepared.context(),
                    reference: prepared.reference(),
                    configured_minimum,
                    effective_minimum,
                    selection,
                })
            }
        };
        let complete = CommittedAdaptiveState {
            observations: self.observations,
            auxiliary_count: self.auxiliary_count,
            sample_count: self.sample_count,
            report,
        };
        validate_complete(
            &complete,
            context,
            configured_minimum,
            original_count,
            original_auxiliary,
            newest,
        )?;
        Ok(complete)
    }
}

/// Checked fixture context and minimum with one bounded adaptive state owner.
///
/// Auxiliary decision rows are exposed separately from current fitted rows.
/// This type supplies no projection adapter, policy default, source eligibility,
/// persistence, ambient clock or authority to perform a correction.
///
/// The privately admitted minimum cannot be rewritten by an external consumer:
/// ```compile_fail
/// use time_core::FixtureAdaptiveHistory;
/// fn reset_minimum(mut history: FixtureAdaptiveHistory) {
///     history.configured_minimum = 0;
/// }
/// ```
/// A legitimate constructor and immutable accessor remain available:
/// ```
/// use std::num::NonZeroU64;
/// use time_core::{AdaptiveHistoryError, FixtureAdaptiveHistory, FixtureContext};
/// # fn main() -> Result<(), AdaptiveHistoryError> {
/// let context = FixtureContext::new(NonZeroU64::MIN, NonZeroU64::MIN, NonZeroU64::MIN);
/// let history = FixtureAdaptiveHistory::new(context, 3)?;
/// assert_eq!(history.context(), context);
/// assert_eq!(history.configured_minimum(), 3);
/// assert_eq!(history.sample_count(), 0);
/// # Ok(()) }
/// ```
pub struct FixtureAdaptiveHistory {
    /// Immutable caller-asserted context; no native continuity is inferred.
    context: FixtureContext,
    /// Mandatory privately validated fixture minimum in three through 64.
    configured_minimum: usize,
    /// None is the sole empty state; every mutable association commits together.
    state: Option<CommittedAdaptiveState>,
}

impl fmt::Debug for FixtureAdaptiveHistory {
    /// Format context/count/presence metadata without observations or fitted data.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FixtureAdaptiveHistory")
            .field("context", &self.context)
            .field("configured_minimum", &self.configured_minimum)
            .field("sample_count", &self.sample_count())
            .field("auxiliary_count", &self.auxiliary_count())
            .field("reference_present", &self.reference().is_some())
            .field("report_present", &self.report().is_some())
            .finish_non_exhaustive()
    }
}

impl FixtureAdaptiveHistory {
    /// Validate the explicit minimum, then construct one empty immutable context.
    ///
    /// Mandatory private Garde validation and the shared predicate must agree.
    /// No observation, normalization, selection, fit or external effect occurs;
    /// a bounded temporary validation report is discarded before returning.
    /// # Errors
    /// Reject unsupported minimum first; validation disagreement is an invariant
    /// failure. Rejection publishes no partially admitted history.
    pub fn new(
        context: FixtureContext,
        configured_minimum: usize,
    ) -> Result<Self, AdaptiveHistoryError> {
        let validation = RawMinimum {
            value: configured_minimum,
        }
        .validate();
        if !minimum_valid(configured_minimum) {
            return Err(AdaptiveHistoryError::InvalidMinimum);
        }
        validation.map_err(|_| AdaptiveHistoryError::StateInvariant)?;
        Ok(Self {
            context,
            configured_minimum,
            state: None,
        })
    }

    /// Return the unchanged asserted context without a current authority check.
    #[must_use]
    pub const fn context(&self) -> FixtureContext {
        self.context
    }

    /// Return the immutable explicit pruning minimum without any defaulting.
    #[must_use]
    pub const fn configured_minimum(&self) -> usize {
        self.configured_minimum
    }

    /// Return only the current fitted count, excluding decision-only older rows.
    #[must_use]
    pub const fn sample_count(&self) -> usize {
        match &self.state {
            Some(state) => state.sample_count,
            None => 0,
        }
    }

    /// Return the number of separately retained decision-only rows, without work.
    #[must_use]
    pub const fn auxiliary_count(&self) -> usize {
        match &self.state {
            Some(state) => state.auxiliary_count,
            None => 0,
        }
    }

    /// Return the exact newest held observation time; an empty owner has none.
    ///
    /// Bounded private indices are checked without changing or refreshing state.
    #[must_use]
    pub fn reference(&self) -> Option<MonotonicNs> {
        let state = self.state.as_ref()?;
        let index = state
            .auxiliary_count
            .checked_add(state.sample_count)?
            .checked_sub(1)?;
        state
            .observations
            .get(index)
            .map(FixtureObservation::observed_at)
    }

    /// Borrow a fitted row by its current fitted index, hiding auxiliary/padding.
    ///
    /// An out-of-range index returns None; no conversion, fitting or copy occurs.
    #[must_use]
    pub fn observation(&self, index: usize) -> Option<&FixtureObservation> {
        let state = self.state.as_ref()?;
        let end = state.auxiliary_count.checked_add(state.sample_count)?;
        state
            .observations
            .get(state.auxiliary_count..end)?
            .get(index)
    }

    /// Borrow a current combined decision row by its explicit combined index.
    ///
    /// The view includes auxiliary then fitted rows, hiding initialized padding.
    /// It grants no eligibility or permission to use an auxiliary row as an anchor.
    #[must_use]
    pub fn decision_observation(&self, index: usize) -> Option<&FixtureObservation> {
        let state = self.state.as_ref()?;
        let end = state.auxiliary_count.checked_add(state.sample_count)?;
        state.observations.get(..end)?.get(index)
    }

    /// Borrow the whole committed report without recalculation or relabelling.
    #[must_use]
    pub const fn report(&self) -> Option<&FixtureAdaptiveHistoryReport> {
        match &self.state {
            Some(state) => state.report.as_ref(),
            None => None,
        }
    }

    /// Stage one validated append, complete numerical work, then publish once.
    ///
    /// Context and strict newest-time admission precede capacity eviction. Full
    /// storage evicts its oldest auxiliary row first, or oldest fitted row when
    /// no auxiliary rows exist. The complete combined stage is normalized once;
    /// any auxiliary-row error rejects without retrying a shorter input. One
    /// shared selector supplies sufficient states' terminal fit and statistics.
    /// Complete postconditions precede the sole live-state assignment. There is
    /// no observation refresh, direct fit, external effect or fallible cleanup.
    /// # Errors
    /// Preserve context, duplicate/older time, normalization and selector failures
    /// in that order. Every failure leaves all old rows/counts/report/reference
    /// unchanged; invariant disagreement likewise publishes no partial state.
    pub fn try_append(
        &mut self,
        observation: FixtureObservation,
    ) -> Result<(), AdaptiveHistoryError> {
        let staged = self.stage_append(observation)?;
        let complete = staged.finish(self.context, self.configured_minimum)?;
        self.state = Some(complete);
        Ok(())
    }

    /// Admit changing input relationships before any staged eviction or copying.
    ///
    /// The old state is borrowed unchanged. One initialized fixed array receives
    /// at most 63 old rows; its next slot already holds the incoming observation.
    /// The resulting raw stage has bounded ordered rows but no numerical report.
    /// # Errors
    /// Context mismatch precedes duplicate/older time; impossible old counts or
    /// transformed bounds reject without live-state changes or numerical work.
    fn stage_append(
        &self,
        observation: FixtureObservation,
    ) -> Result<StagedAppend, AdaptiveHistoryError> {
        if observation.context() != self.context {
            return Err(AdaptiveHistoryError::ContextMismatch);
        }
        let (retained, auxiliary_count) = if let Some(state) = &self.state {
            let rows = combined_prefix(
                &state.observations,
                state.auxiliary_count,
                state.sample_count,
            )?;
            let newest = rows.last().ok_or(AdaptiveHistoryError::StateInvariant)?;
            match observation.observed_at().cmp(&newest.observed_at()) {
                Ordering::Equal => return Err(AdaptiveHistoryError::DuplicateTime),
                Ordering::Less => return Err(AdaptiveHistoryError::OutOfOrderTime),
                Ordering::Greater => {}
            }
            if rows.len() == MAX_POINTS {
                let auxiliary_count = if state.auxiliary_count > 0 {
                    state
                        .auxiliary_count
                        .checked_sub(1)
                        .ok_or(AdaptiveHistoryError::StateInvariant)?
                } else {
                    0
                };
                (
                    rows.get(1..).ok_or(AdaptiveHistoryError::StateInvariant)?,
                    auxiliary_count,
                )
            } else {
                (rows, state.auxiliary_count)
            }
        } else {
            (&[][..], 0)
        };
        let combined_count = retained
            .len()
            .checked_add(1)
            .ok_or(AdaptiveHistoryError::StateInvariant)?;
        let sample_count = combined_count
            .checked_sub(auxiliary_count)
            .ok_or(AdaptiveHistoryError::StateInvariant)?;
        let mut observations = [observation; MAX_POINTS];
        for (target, retained) in observations.iter_mut().zip(retained) {
            *target = *retained;
        }
        let _prefix = combined_prefix(&observations, auxiliary_count, sample_count)?;
        Ok(StagedAppend {
            observations,
            auxiliary_count,
            sample_count,
        })
    }
}

/// Direct semantic checks of the raw Garde rule without constructor masking.
#[cfg(test)]
#[path = "../../tests/private/adaptive_history_guards.rs"]
mod raw_minimum_guards;

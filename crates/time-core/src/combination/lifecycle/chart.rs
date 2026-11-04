// SPDX-License-Identifier: GPL-2.0-only
//! Chart-aware receiving and output over the existing combination transaction.
//!
//! The parent owns staging, original-source scoring, penalties, observation
//! credit, emission and the sole commit. This child retains the actual chart
//! decision/batch/entries and combines fitted chart coordinates with separate
//! new-arithmetic bounds. No chart view installs a source generation, refits a
//! history, authorizes a clock correction or grants physical confidence.

use std::{fmt, num::NonZeroU64, ptr};

use garde::Validate;
use time_domain::MonotonicNs;

use crate::combination::{
    FixtureCombinationError, FixtureTrackingCombinationError, MAX_SUPPORTERS,
    admission::chart_receiving::{
        AdmittedChartSupporters, ChartTrackingSupporters, admit_chart_source,
    },
    chart::{ChartQuantities, ChartReductionError, ChartSums, CompanionError},
};
use crate::{
    FixtureAffineChart, FixtureAnchorProjection, FixtureChartBatch, FixtureChartError,
    FixtureChartInterval, FixtureChartPrincipalDecision, FixtureChartPrincipalError,
    FixtureContext, FixturePrincipalStratum, chart_batch::chart_descriptor_mismatch,
    regression::scalar_issue,
};

use super::{
    CheckedEstimateView, CompletedOutcome, DecisionFacts, FixtureCombinationLifecycleError,
    FixtureCombinationSourceStatus, FixtureCombinationState, Inner, SharedOutcomeInput,
    SourceOutcome, emit_round, prepare_contributors, shared_outcome_valid, uncertainty_issue,
    with_staged,
};

/// Bounded chart, complete-metadata, lifecycle or numerical-bound rejection.
#[derive(PartialEq)]
pub enum FixtureChartCombinationError {
    /// Preserve the real expected-chart or original principal receiving failure.
    Principal {
        /// Actual bounded chart-principal cause without fabricated metadata.
        source: FixtureChartPrincipalError,
    },
    /// An original witness participant has no same-entry tracking attachment.
    MissingTracking {
        /// Canonical original witness ordinal, bounded below sixteen.
        index: usize,
    },
    /// Preserve the existing compatibility, point-reduction or output cause.
    Lifecycle {
        /// Actual bounded existing lifecycle failure, including exact point causes.
        source: FixtureCombinationLifecycleError,
    },
    /// Preserve an actual outward arithmetic-bound failure.
    Bound {
        /// Original checked chart bound operation and cause.
        source: FixtureChartError,
    },
    /// No proved strictly positive denominator margin exists for the bound.
    UnprovedPositiveMargin,
    /// Private checked representation, provenance or reduction order disagreed.
    Invariant,
}

impl fmt::Debug for FixtureChartCombinationError {
    /// Redact source identities, descriptors, numerical inputs and prior state.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureChartCombinationError { .. }")
    }
}

impl fmt::Display for FixtureChartCombinationError {
    /// Display only a bounded fixed family code without formatting supplied data.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Principal { .. } => "fixture chart combination: principal rejected",
            Self::MissingTracking { .. } => "fixture chart combination: tracking missing",
            Self::Lifecycle { .. } => "fixture chart combination: lifecycle rejected",
            Self::Bound { .. } => "fixture chart combination: bound arithmetic rejected",
            Self::UnprovedPositiveMargin => {
                "fixture chart combination: unproved denominator margin"
            }
            Self::Invariant => "fixture chart combination: invariant rejected",
        })
    }
}

impl std::error::Error for FixtureChartCombinationError {
    /// Borrow actual delegated causes; representation failures invent no cause.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Principal { source } => Some(source),
            Self::Lifecycle { source } => Some(source),
            Self::Bound { source } => Some(source),
            Self::MissingTracking { .. } | Self::UnprovedPositiveMargin | Self::Invariant => None,
        }
    }
}

impl From<FixtureChartPrincipalError> for FixtureChartCombinationError {
    /// Preserve the genuine chart-principal failure without changing state.
    fn from(source: FixtureChartPrincipalError) -> Self {
        Self::Principal { source }
    }
}

impl From<FixtureCombinationLifecycleError> for FixtureChartCombinationError {
    /// Retain the existing state, point-arithmetic or validation cause unchanged.
    fn from(source: FixtureCombinationLifecycleError) -> Self {
        Self::Lifecycle { source }
    }
}

impl From<FixtureCombinationError> for FixtureChartCombinationError {
    /// Keep the existing exact combination cause under the lifecycle family.
    fn from(source: FixtureCombinationError) -> Self {
        FixtureCombinationLifecycleError::Combination { source }.into()
    }
}

impl From<FixtureTrackingCombinationError> for FixtureChartCombinationError {
    /// Preserve a missing canonical attachment or the actual receiving cause.
    fn from(error: FixtureTrackingCombinationError) -> Self {
        match error {
            FixtureTrackingCombinationError::MissingTracking { index } => {
                Self::MissingTracking { index }
            }
            FixtureTrackingCombinationError::Combination { source } => source.into(),
        }
    }
}

impl From<ChartReductionError> for FixtureChartCombinationError {
    /// Project private companion failures without exposing private arithmetic types.
    fn from(error: ChartReductionError) -> Self {
        match error {
            ChartReductionError::Combination(source)
            | ChartReductionError::Companion(CompanionError::Value { source }) => source.into(),
            ChartReductionError::Companion(CompanionError::Bound { source }) => {
                Self::Bound { source }
            }
            ChartReductionError::Companion(CompanionError::UnprovedPositiveMargin) => {
                Self::UnprovedPositiveMargin
            }
            ChartReductionError::Companion(CompanionError::ValidationInvariant) => {
                FixtureCombinationLifecycleError::ValidationInvariant.into()
            }
            // Typed input/output owners already establish point/error representation.
            ChartReductionError::Invariant
            | ChartReductionError::Companion(
                CompanionError::InvalidPoint { .. } | CompanionError::InvalidError,
            ) => Self::Invariant,
        }
    }
}

/// Checked fitted chart coordinates with actual batch and original-source custody.
///
/// Raw source context/observation times remain separate from target coordinates.
/// The two mean bounds cover only new arithmetic over stored inputs, not source
/// fitting error, statistical deviation, physical uncertainty or clock effects.
/// The result has no public constructor or mutable numerical access.
/// ```compile_fail
/// use time_core::FixtureChartCombinedEstimate;
/// fn overwrite(value: &mut FixtureChartCombinedEstimate<'_, '_, '_>) {
///     value.quantities.phase_error = 0.0;
/// }
/// ```
pub struct FixtureChartCombinedEstimate<'batch, 'entries, 'chart> {
    /// The exact batch whose genuine decision supplied the original witness.
    batch: &'batch FixtureChartBatch<'entries, 'chart>,
    /// The exact witnessed principal wrapper, retaining its own descriptor.
    principal: &'entries FixtureChartInterval<'chart>,
    /// Canonical contributing prefix; inaccessible padding supplies no extra votes.
    labels: [NonZeroU64; MAX_SUPPORTERS],
    /// Contributing count, independent of the original agreement cardinality.
    count: usize,
    /// Checked five points and separate deterministic mean-error bounds.
    quantities: ChartQuantities,
}

impl fmt::Debug for FixtureChartCombinedEstimate<'_, '_, '_> {
    /// Reveal no descriptor, source identities, ages or numerical quantities.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureChartCombinedEstimate { .. }")
    }
}

impl<'batch, 'entries, 'chart> FixtureChartCombinedEstimate<'batch, 'entries, 'chart> {
    /// Borrow the actual admitted batch, without constructing a substitute view.
    #[must_use]
    pub const fn batch(&self) -> &'batch FixtureChartBatch<'entries, 'chart> {
        self.batch
    }
    /// Borrow the batch's actual declared descriptor, preserving its lifetime.
    #[must_use]
    pub const fn chart(&self) -> &'chart FixtureAffineChart {
        self.batch.chart()
    }
    /// Borrow the actual original principal wrapper with its own descriptor.
    #[must_use]
    pub const fn principal_entry(&self) -> &'entries FixtureChartInterval<'chart> {
        self.principal
    }
    /// Copy unchanged raw source assertions, not target coordinates or authority.
    #[must_use]
    pub const fn source_context(&self) -> FixtureContext {
        self.principal.projection().source_projection().context()
    }
    /// Copy mathematical target assertions while retaining the original source label.
    #[must_use]
    pub const fn context(&self) -> FixtureContext {
        self.principal.projection().context()
    }
    /// Copy the original asserted principal source label without fresh credit.
    #[must_use]
    pub const fn principal_label(&self) -> NonZeroU64 {
        self.source_context().source_id()
    }
    /// Copy the mathematical target frame, separate from `source_context()`.
    #[must_use]
    pub const fn frame(&self) -> NonZeroU64 {
        self.context().frame()
    }
    /// Copy mathematical target generation without installing a source generation.
    #[must_use]
    pub const fn mapping_generation(&self) -> NonZeroU64 {
        self.context().generation()
    }
    /// Copy the unchanged actual source anchor, without refreshing observation age.
    #[must_use]
    pub const fn anchor_observed_at(&self) -> MonotonicNs {
        self.principal
            .projection()
            .source_projection()
            .anchor_observed_at()
    }
    /// Copy the unchanged latest actual source observation instant.
    #[must_use]
    pub const fn latest_observed_at(&self) -> MonotonicNs {
        self.principal
            .projection()
            .source_projection()
            .latest_observed_at()
    }
    /// Copy the unchanged original fit reference instant.
    #[must_use]
    pub const fn fit_reference(&self) -> MonotonicNs {
        self.principal
            .projection()
            .source_projection()
            .fit_reference()
    }
    /// Copy the original common evaluation instant without an ambient clock read.
    #[must_use]
    pub const fn projected_at(&self) -> MonotonicNs {
        self.principal
            .projection()
            .source_projection()
            .projected_at()
    }
    /// Count mathematical contributors without changing the original quorum.
    #[must_use]
    pub const fn supporter_count(&self) -> usize {
        self.count
    }
    /// Iterate canonical contributors without exposing padding or extra votes.
    pub fn supporters(&self) -> impl ExactSizeIterator<Item = NonZeroU64> + '_ {
        self.labels.iter().take(self.count).copied()
    }
    /// Copy the transformed fitted-phase mean in seconds, positive behind.
    #[must_use]
    pub const fn phase_seconds(&self) -> f64 {
        self.quantities.points.phase_seconds
    }
    /// Copy the transformed fitted-rate mean in seconds/second.
    #[must_use]
    pub const fn phase_rate_seconds_per_second(&self) -> f64 {
        self.quantities.points.phase_rate_seconds_per_second
    }
    /// Copy statistical phase SD separately from numerical mean error.
    #[must_use]
    pub const fn phase_standard_deviation_seconds(&self) -> f64 {
        self.quantities.points.phase_standard_deviation_seconds
    }
    /// Copy statistical rate SD separately from numerical mean error.
    #[must_use]
    pub const fn rate_standard_deviation_seconds_per_second(&self) -> f64 {
        self.quantities
            .points
            .rate_standard_deviation_seconds_per_second
    }
    /// Copy combined source tracking skew, not projection-growth skew.
    #[must_use]
    pub const fn tracking_skew_seconds_per_second(&self) -> f64 {
        self.quantities.points.tracking_skew_seconds_per_second
    }
    /// Copy only the new-arithmetic fitted-phase mean error upper, in seconds.
    #[must_use]
    pub const fn phase_mean_roundoff_seconds(&self) -> f64 {
        self.quantities.phase_error
    }
    /// Copy only the new-arithmetic fitted-rate mean error upper, in seconds/second.
    #[must_use]
    pub const fn rate_mean_roundoff_seconds_per_second(&self) -> f64 {
        self.quantities.rate_error
    }
}

impl CheckedEstimateView for FixtureChartCombinedEstimate<'_, '_, '_> {
    /// Return only the completed mathematical contributor prefix length.
    fn supporter_count(&self) -> usize {
        FixtureChartCombinedEstimate::supporter_count(self)
    }
    /// Check the completed contributor labels without changing original agreement.
    fn contains_source(&self, label: NonZeroU64) -> bool {
        self.supporters().any(|actual| actual == label)
    }
    /// Retain exact raw principal identity and every original context/age assertion.
    fn matches_principal(&self, projection: &FixtureAnchorProjection) -> bool {
        ptr::eq(self.principal.projection().source_projection(), projection)
            && self.source_context() == projection.context()
            && self.anchor_observed_at() == projection.anchor_observed_at()
            && self.latest_observed_at() == projection.latest_observed_at()
            && self.fit_reference() == projection.fit_reference()
            && self.projected_at() == projection.projected_at()
    }
}

/// Genuine chart decision and newly emitted estimate from one retained owner.
pub struct FixtureChartCombinationOutcome<'batch, 'entries, 'chart> {
    /// Actual decision owns its chart selection and original batch lifetime.
    principal: FixtureChartPrincipalDecision<'batch, 'entries, 'chart>,
    /// New checked estimate only when actual reference credit was consumed.
    combined: Option<FixtureChartCombinedEstimate<'batch, 'entries, 'chart>>,
    /// New per-instance emission, never fabricated for reevaluation.
    emitted_round: Option<NonZeroU64>,
    /// Complete original roster status prefix after shared parent preparation.
    sources: [Option<SourceOutcome>; MAX_SUPPORTERS],
    /// Admitted roster length; inaccessible padding is never reported.
    len: usize,
}

impl fmt::Debug for FixtureChartCombinationOutcome<'_, '_, '_> {
    /// Reveal neither labels, descriptors, numerical outputs nor event credit.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureChartCombinationOutcome { .. }")
    }
}

impl<'batch, 'entries, 'chart> FixtureChartCombinationOutcome<'batch, 'entries, 'chart> {
    /// Borrow the actual same-batch decision without repeating its transition.
    #[must_use]
    pub const fn principal(&self) -> &FixtureChartPrincipalDecision<'batch, 'entries, 'chart> {
        &self.principal
    }
    /// Borrow the newly checked estimate only for a genuine ready emission.
    #[must_use]
    pub const fn combined(
        &self,
    ) -> Option<&FixtureChartCombinedEstimate<'batch, 'entries, 'chart>> {
        self.combined.as_ref()
    }
    /// Copy this owner's new round ordinal, absent for retained/unavailable results.
    #[must_use]
    pub const fn emitted_round(&self) -> Option<NonZeroU64> {
        self.emitted_round
    }
    /// Observe original witnessed population independently of filtering.
    #[must_use]
    pub fn witness_count(&self) -> Option<usize> {
        self.principal
            .selection()
            .witnessed()
            .map(|witness| witness.supporters().len())
    }
    /// Observe contributing population only when a new estimate exists.
    #[must_use]
    pub fn contributor_count(&self) -> Option<usize> {
        self.combined
            .as_ref()
            .map(FixtureChartCombinedEstimate::supporter_count)
    }
    /// Observe one registered source status without new eligibility or credit.
    #[must_use]
    pub fn source_status(&self, id: NonZeroU64) -> Option<FixtureCombinationSourceStatus> {
        self.sources
            .iter()
            .take(self.len)
            .flatten()
            .find(|source| source.source_id == id)
            .map(|source| source.status)
    }
}

/// Actual admitted original witness and its complete prepared contributor mask.
type ContributorInput<'value, 'input, 'entries, 'chart> = (
    &'value AdmittedChartSupporters<'input, 'entries, 'chart>,
    [bool; MAX_SUPPORTERS],
);

/// Validate the complete private contributor mask against original witness positions.
/// # Errors
/// Reject out-of-prefix flags, an empty set or omission of the original principal.
fn contributor_count(
    value: &ContributorInput<'_, '_, '_, '_>,
) -> Result<usize, FixtureCombinationError> {
    let (admitted, mask) = value;
    if admitted.count() == 0
        || admitted.count() > MAX_SUPPORTERS
        || mask.iter().skip(admitted.count()).any(|included| *included)
    {
        return Err(FixtureCombinationError::Invariant);
    }
    let principal = admitted.principal_original().context().source_id();
    let mut included = 0_usize;
    let mut principal_present = false;
    for (index, label) in admitted.labels().enumerate() {
        if mask
            .get(index)
            .copied()
            .ok_or(FixtureCombinationError::Invariant)?
        {
            included = included
                .checked_add(1)
                .ok_or(FixtureCombinationError::Invariant)?;
            principal_present |= label == principal;
        }
    }
    if included == 0 || !principal_present {
        return Err(FixtureCombinationError::Invariant);
    }
    Ok(included)
}

/// Mandatory relationship admission before private chart reduction.
#[derive(Validate)]
struct RawContributors<'value, 'input, 'entries, 'chart> {
    /// The complete mask and actual admitted witness share one bounded predicate.
    #[garde(custom(|value: &ContributorInput<'_, '_, '_, '_>, (): &()| {
        contributor_count(value).map(|_| ())
            .map_err(|_| garde::Error::new("invalid fixture chart contributor mask"))
    }))]
    value: ContributorInput<'value, 'input, 'entries, 'chart>,
}

/// Reduce checked actual entries, using singleton rules only for original population one.
/// # Errors
/// Preserve association, complete-mask, exact point or actual bound failures.
fn combine_chart_supporters<'batch, 'entries, 'chart>(
    decision: &FixtureChartPrincipalDecision<'batch, 'entries, 'chart>,
    admitted: &AdmittedChartSupporters<'_, 'entries, 'chart>,
    mask: [bool; MAX_SUPPORTERS],
) -> Result<FixtureChartCombinedEstimate<'batch, 'entries, 'chart>, FixtureChartCombinationError> {
    if !ptr::eq(admitted.selection(), decision.selection())
        || !decision
            .principal_entry()
            .is_some_and(|entry| ptr::eq(entry, admitted.principal()))
    {
        return Err(FixtureChartCombinationError::Invariant);
    }
    let raw = RawContributors {
        value: (admitted, mask),
    };
    let validated = raw.validate();
    let count = contributor_count(&raw.value)?;
    validated.map_err(|_| FixtureCombinationLifecycleError::ValidationInvariant)?;
    let principal_label = admitted.principal_original().context().source_id();
    let principal_phase = admitted.record(principal_label)?.tracking().phase_seconds();
    let mut labels = [principal_label; MAX_SUPPORTERS];
    let mut occupied = 0_usize;
    let mut sums = if admitted.count() > 1 {
        Some(ChartSums::empty()?)
    } else {
        None
    };
    for (index, label) in admitted.labels().enumerate() {
        if !mask
            .get(index)
            .copied()
            .ok_or(FixtureCombinationError::Invariant)?
        {
            continue;
        }
        *labels
            .get_mut(occupied)
            .ok_or(FixtureCombinationError::Invariant)? = label;
        occupied = occupied
            .checked_add(1)
            .ok_or(FixtureCombinationError::Invariant)?;
        if let Some(previous) = sums.as_ref() {
            sums = Some(previous.with_record(admitted.record(label)?, principal_phase, index)?);
        }
    }
    if occupied != count {
        return Err(FixtureChartCombinationError::Invariant);
    }
    let quantities = match sums {
        Some(sums) => sums.finish()?,
        None => ChartQuantities::singleton(admitted)?,
    };
    Ok(FixtureChartCombinedEstimate {
        batch: decision.batch(),
        principal: admitted.principal(),
        labels,
        count,
        quantities,
    })
}

/// Recognize checked point and separate error representations before publication.
fn quantities_valid(estimate: &FixtureChartCombinedEstimate<'_, '_, '_>) -> bool {
    let value = &estimate.quantities;
    [
        value.points.phase_seconds,
        value.points.phase_rate_seconds_per_second,
    ]
    .into_iter()
    .all(|point| scalar_issue(point).is_none())
        && [
            value.points.phase_standard_deviation_seconds,
            value.points.rate_standard_deviation_seconds_per_second,
            value.points.tracking_skew_seconds_per_second,
        ]
        .into_iter()
        .all(|sd| uncertainty_issue(sd).is_none())
        && [value.phase_error, value.rate_error]
            .into_iter()
            .all(|bound| bound.is_finite() && bound >= 0.0)
}

/// Check actual descriptor/batch custody, target/source separation and contributor prefix.
fn estimate_valid(
    estimate: &FixtureChartCombinedEstimate<'_, '_, '_>,
    decision: &FixtureChartPrincipalDecision<'_, '_, '_>,
) -> bool {
    let Some(principal) = decision.principal_entry() else {
        return false;
    };
    let Some(witness) = decision.selection().witnessed() else {
        return false;
    };
    let frames = decision.batch().chart().frames();
    if estimate.count == 0
        || estimate.count > MAX_SUPPORTERS
        || !ptr::eq(estimate.batch(), decision.batch())
        || !ptr::eq(estimate.principal_entry(), principal)
        || chart_descriptor_mismatch(estimate.chart(), principal.projection().chart()).is_some()
        || estimate.source_context().frame() != frames.source_frame()
        || estimate.source_context().generation() != frames.source_generation()
        || estimate.context().frame() != frames.target_frame()
        || estimate.context().generation() != frames.target_generation()
        || estimate.projected_at() != decision.batch().projected_at()
        || !quantities_valid(estimate)
    {
        return false;
    }
    let mut previous = None;
    let mut principal_present = false;
    for label in estimate.supporters() {
        if previous.is_some_and(|before| before >= label)
            || !witness.supporters().any(|actual| actual == label)
        {
            return false;
        }
        previous = Some(label);
        principal_present |= label == estimate.principal_label();
    }
    principal_present
}

/// Complete borrowed chart output and still-unpublished parent state relationship.
struct ChartOutcomeInput<'value, 'batch, 'entries, 'chart> {
    /// Genuine same-batch decision, never a converted legacy selection.
    principal: &'value FixtureChartPrincipalDecision<'batch, 'entries, 'chart>,
    /// Complete unpublished estimate under that exact decision.
    combined: Option<&'value FixtureChartCombinedEstimate<'batch, 'entries, 'chart>>,
    /// Prepared next emission identity, absent when no new estimate exists.
    round: Option<NonZeroU64>,
    /// Complete parent-prepared original roster statuses.
    statuses: &'value [Option<SourceOutcome>; MAX_SUPPORTERS],
    /// Unpublished whole next state; no reference escapes completion.
    inner: &'value Inner,
    /// Entry-state emission count, preserved on every nonready transition.
    previous_rounds: u64,
}

/// Apply chart-specific provenance/bounds and the unchanged shared lifecycle predicate.
fn chart_outcome_valid(value: &ChartOutcomeInput<'_, '_, '_, '_>) -> bool {
    if value
        .combined
        .is_some_and(|estimate| !estimate_valid(estimate, value.principal))
    {
        return false;
    }
    shared_outcome_valid(&SharedOutcomeInput {
        principal: DecisionFacts::chart(value.principal),
        combined: value.combined,
        round: value.round,
        statuses: value.statuses,
        inner: value.inner,
        previous_rounds: value.previous_rounds,
    })
}

/// Mandatory whole-output relationship admission before the parent's sole commit.
#[derive(Validate)]
struct RawChartOutcome<'value, 'batch, 'entries, 'chart> {
    /// Descriptor, bounds and shared state/output relations have one maintained rule.
    #[garde(custom(|value: &ChartOutcomeInput<'_, '_, '_, '_>, (): &()| {
        if chart_outcome_valid(value) { Ok(()) }
        else { Err(garde::Error::new("invalid fixture chart combination output")) }
    }))]
    value: ChartOutcomeInput<'value, 'batch, 'entries, 'chart>,
}

/// Complete the actual chart output only after Garde and every postcondition agree.
/// # Errors
/// Reject inconsistent provenance/state/bounds without a completed commit token.
fn complete_chart_outcome<'batch, 'entries, 'chart>(
    principal: FixtureChartPrincipalDecision<'batch, 'entries, 'chart>,
    combined: Option<FixtureChartCombinedEstimate<'batch, 'entries, 'chart>>,
    round: Option<NonZeroU64>,
    statuses: [Option<SourceOutcome>; MAX_SUPPORTERS],
    inner: &Inner,
    previous_rounds: u64,
) -> Result<
    CompletedOutcome<FixtureChartCombinationOutcome<'batch, 'entries, 'chart>>,
    FixtureChartCombinationError,
> {
    let raw = RawChartOutcome {
        value: ChartOutcomeInput {
            principal: &principal,
            combined: combined.as_ref(),
            round,
            statuses: &statuses,
            inner,
            previous_rounds,
        },
    };
    let validated = raw.validate();
    if !chart_outcome_valid(&raw.value) {
        return Err(FixtureCombinationLifecycleError::Invariant.into());
    }
    validated.map_err(|_| FixtureCombinationLifecycleError::ValidationInvariant)?;
    Ok(CompletedOutcome {
        output: FixtureChartCombinationOutcome {
            principal,
            combined,
            emitted_round: round,
            sources: statuses,
            len: inner.len,
        },
    })
}

impl FixtureCombinationState {
    /// Advance the same retained original-source owner through a genuine chart batch.
    ///
    /// Full witness tracking precedes shared scoring/filtering, including retained
    /// and later-excluded participants. Compatibility uses original rate/distance/
    /// skew; means use transformed fitted phase/rate with their separate arithmetic
    /// bounds. Only original witness size one permits denominator-free copying.
    /// The parent stages the whole owner and commits only a completed checked output.
    /// No view change, reevaluation or replay creates a source observation or credit.
    /// # Errors
    /// Preserve capacity, expected-chart, principal, metadata, compatibility, exact
    /// point and bound failures. Every failure preserves the complete prior owner.
    pub fn advance_chart<'batch, 'entries, 'chart>(
        &mut self,
        expected: &FixtureAffineChart,
        batch: &'batch FixtureChartBatch<'entries, 'chart>,
        strata: &[FixturePrincipalStratum],
    ) -> Result<
        FixtureChartCombinationOutcome<'batch, 'entries, 'chart>,
        FixtureChartCombinationError,
    > {
        if strata.len() > MAX_SUPPORTERS {
            return Err(FixtureCombinationLifecycleError::CapacityExceeded.into());
        }
        with_staged(&mut self.inner, |staged| {
            let previous_principal = staged.principal.principal();
            let previous_rounds = staged.emitted_rounds;
            let decision = staged.principal.advance_chart(expected, batch, strata)?;
            let (combined, round, statuses) = {
                let tracking = ChartTrackingSupporters::from_decision(&decision)?;
                let admitted = admit_chart_source(&decision, &tracking)?;
                let prepared = prepare_contributors(
                    staged,
                    DecisionFacts::chart(&decision),
                    previous_principal,
                    admitted.as_ref().map(AdmittedChartSupporters::originals),
                )?;
                if let Some(mask) = prepared.mask {
                    let admitted = admitted
                        .as_ref()
                        .ok_or(FixtureCombinationLifecycleError::Invariant)?;
                    let estimate = combine_chart_supporters(&decision, admitted, mask)?;
                    let round = emit_round(staged, &prepared)?;
                    (Some(estimate), Some(round), prepared.statuses)
                } else {
                    (None, None, prepared.statuses)
                }
            };
            complete_chart_outcome(decision, combined, round, statuses, staged, previous_rounds)
        })
    }
}

#[cfg(test)]
#[path = "../../../tests/private/chart_combination_lifecycle_guards.rs"]
mod guard_tests;

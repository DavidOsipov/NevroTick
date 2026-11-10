// SPDX-License-Identifier: GPL-2.0-only
//! Borrowed reference-input readiness for one explicitly bound fixture chart.
//!
//! The owner retains the exact checked chart instance and only monotone modeled
//! readiness. Admission consumes no observation or emission credit and changes
//! neither the owner nor the genuine chart-combination outcome. A token borrows
//! both values; its estimate access also shortens the estimate's inner lifetimes.
//! An exclusive modeled achieved-phase receiver admits its complete relationship
//! before invalidating this owner, then delegates replacement arithmetic to the
//! existing chart composer. An admitted arithmetic failure never restores the old
//! input. No installed clock mapping, actual correction, restoration or authority
//! is inferred from these fixture values. There is no new numerical formula.
//!

/// Local settled-rate accounting delegates all numerical work to the chart owner.
mod rate_delivery;

pub use rate_delivery::{
    FixtureBaselineCompletion, FixtureBaselineError, FixtureBaselineFeedback,
    FixtureBaselineProposal, FixtureBaselineRetirement, FixtureBaselineSettlement,
    FixtureControllerClockMap, FixtureControllerClockMapInput, FixtureFrequencyAdmission,
    FixtureFrequencyClockAssociation, FixtureFrequencyError, FixtureFrequencyHistory,
    FixtureFrequencyProfile, FixtureFrequencyProjection, FixturePhaseBaseline, FixturePhaseBlocked,
    FixturePhaseConstructionRejected, FixturePhaseError, FixturePhaseNow, FixturePhasePlanOwner,
    FixturePhaseProfile, FixturePhaseProposal, FixturePhaseProvenance,
    FixturePhaseReferenceAdmission, FixturePhaseResidualPermission, FixturePhaseRetirement,
    FixturePhaseRetirementReason, FixturePhaseSettlement, FixtureRateClock,
    FixtureRateClockContext, FixtureRateCursorStatus, FixtureRateDeliveryError,
    FixtureRateSettlement, FixtureRateSettlementCompletion, FixtureRateSettlementCursor,
    FixtureRateSettlementKind, FixtureRateSpan, FixtureSettledRateInterval,
};

use std::{fmt, num::NonZeroU64, ptr};

use garde::Validate;

use crate::{
    FixtureAffineChart, FixtureChartCombinationOutcome, FixtureChartCombinedEstimate,
    FixtureChartDomain, FixtureChartError, FixtureChartFrames,
};

/// Modeled usability of this owner's exact fixture chart, without clock authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureReferenceReadiness {
    /// No invalidating feedback has been received by this owner.
    Ready,
    /// The modeled coordinates changed; this owner cannot restore readiness.
    CoordinateChanged,
    /// The modeled effect is unknown; this state dominates every later feedback.
    Unknown,
}

/// Closed modeled feedback with no operation identity or quantitative effect.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureCorrectionFeedback {
    /// Explicit modeled proof of no effect; preserves every existing readiness.
    ProvenNoEffect,
    /// Invalidate the modeled view without constructing a replacement mapping.
    CoordinateChanged,
    /// Make the modeled effect unknown without treating it as zero or success.
    Unknown,
}

/// Bounded receiving rejection, retaining neither input values nor Garde reports.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum FixtureReferenceInputError {
    /// This owner has received invalidating modeled feedback.
    Unavailable {
        /// The unchanged closed readiness state at rejection.
        readiness: FixtureReferenceReadiness,
    },
    /// The actual outcome lacks `ReferenceReady`, a combined estimate or a round.
    NotReady,
    /// The actual estimate borrows a different chart instance, even if equal.
    ForeignChartView,
    /// A modeled achieved-phase relationship or scalar failed before admission.
    Chart {
        /// Exact bounded chart cause, retaining no chart, scalar or raw report.
        source: FixtureChartError,
    },
    /// Mandatory Garde disagreed with the shared typed relationship rule.
    ValidationInvariant,
}

impl fmt::Debug for FixtureReferenceInputError {
    /// Emit a fixed redacted family name without formatting rejected inputs.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureReferenceInputError { .. }")
    }
}

impl fmt::Display for FixtureReferenceInputError {
    /// Distinguish bounded rejection families without source labels or numerics.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Unavailable { .. } => "fixture reference input: view unavailable",
            Self::NotReady => "fixture reference input: outcome not ready",
            Self::ForeignChartView => "fixture reference input: foreign chart view",
            Self::Chart { .. } => "fixture reference input: achieved phase rejected",
            Self::ValidationInvariant => "fixture reference input: validation invariant",
        })
    }
}

impl std::error::Error for FixtureReferenceInputError {
    /// Borrow the exact bounded chart cause without formatting rejected values.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Chart { source } => Some(source),
            Self::Unavailable { .. }
            | Self::NotReady
            | Self::ForeignChartView
            | Self::ValidationInvariant => None,
        }
    }
}

/// Admitted modeled phase event, distinct from an unchanged-state rejection.
///
/// The old owner is already `CoordinateChanged`, even if the replacement failed.
/// This value neither restores that owner nor grants replay or clock authority.
/// Private fields prevent callers from forging an admitted completion:
///
/// ```compile_fail,E0451
/// use time_core::{FixtureAchievedPhaseCompletion, FixtureChartError};
/// let _forged = FixtureAchievedPhaseCompletion {
///     replacement: Err(FixtureChartError::Invariant),
/// };
/// ```
#[must_use]
pub struct FixtureAchievedPhaseCompletion {
    /// Sole replacement attempt after the owner has committed invalidation.
    replacement: Result<FixtureAffineChart, FixtureChartError>,
}

impl fmt::Debug for FixtureAchievedPhaseCompletion {
    /// Redact the replacement chart and bounded cause with constant output.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureAchievedPhaseCompletion { .. }")
    }
}

impl FixtureAchievedPhaseCompletion {
    /// Borrow the checked replacement or copy its exact bounded chart failure.
    ///
    /// Neither branch changes readiness or retries replacement arithmetic.
    /// # Errors
    /// Return the original composition failure after admitted invalidation.
    pub const fn replacement(&self) -> Result<&FixtureAffineChart, FixtureChartError> {
        match &self.replacement {
            Ok(chart) => Ok(chart),
            Err(error) => Err(*error),
        }
    }

    /// Consume this completion and transfer its replacement or exact chart cause.
    ///
    /// The returned chart does not restore the predecessor's readiness owner.
    /// # Errors
    /// Return the original composition failure; the old owner stays invalidated.
    pub fn into_replacement(self) -> Result<FixtureAffineChart, FixtureChartError> {
        self.replacement
    }
}

/// Monotone readiness bound to one immutable checked chart instance.
pub struct FixtureReferenceInputState<'chart> {
    /// Exact modeled view; equal descriptors elsewhere do not replace it.
    chart: &'chart FixtureAffineChart,
    /// Closed state changed only by monotone feedback or admitted phase reception.
    readiness: FixtureReferenceReadiness,
}

impl fmt::Debug for FixtureReferenceInputState<'_> {
    /// Redact the bound chart, labels and modeled readiness with fixed output.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureReferenceInputState { .. }")
    }
}

/// Actual receiving arguments whose shared rule owns the contextual relationship.
struct ReferenceInputValue<'value, 'batch, 'entries, 'chart> {
    /// Immutable owner borrow prevents feedback while an admitted token is used.
    owner: &'value FixtureReferenceInputState<'chart>,
    /// Genuine immutable output retains its existing checked provenance and credit.
    outcome: &'value FixtureChartCombinationOutcome<'batch, 'entries, 'chart>,
}

/// Privately checked parts borrowed from the actual admitted outcome.
struct ReferenceInputParts<'value, 'batch, 'entries, 'chart> {
    /// Actual combined field, never a reconstructed estimate or detached pair.
    combined: &'value FixtureChartCombinedEstimate<'batch, 'entries, 'chart>,
    /// Existing nonzero emission ordinal copied without consuming fresh credit.
    emitted_round: NonZeroU64,
}

/// Check readiness, genuine emission and exact chart identity without mutation.
/// # Errors
/// Reject unavailable readiness first, then incomplete emission, then foreign view.
fn reference_input_parts<'value, 'batch, 'entries, 'chart>(
    value: &ReferenceInputValue<'value, 'batch, 'entries, 'chart>,
) -> Result<ReferenceInputParts<'value, 'batch, 'entries, 'chart>, FixtureReferenceInputError> {
    if value.owner.readiness != FixtureReferenceReadiness::Ready {
        return Err(FixtureReferenceInputError::Unavailable {
            readiness: value.owner.readiness,
        });
    }
    if !value.outcome.principal().reference_ready() {
        return Err(FixtureReferenceInputError::NotReady);
    }
    let combined = value
        .outcome
        .combined()
        .ok_or(FixtureReferenceInputError::NotReady)?;
    let emitted_round = value
        .outcome
        .emitted_round()
        .ok_or(FixtureReferenceInputError::NotReady)?;
    if !ptr::eq(
        ptr::from_ref(combined.chart()),
        ptr::from_ref(value.owner.chart),
    ) {
        return Err(FixtureReferenceInputError::ForeignChartView);
    }
    Ok(ReferenceInputParts {
        combined,
        emitted_round,
    })
}

/// Mandatory Garde adaptation of the same bounded actual receiving relationship.
#[derive(Validate)]
struct RawReferenceInput<'value, 'batch, 'entries, 'chart> {
    /// The single shared rule owns readiness, emission and exact-instance binding.
    #[garde(custom(|value: &ReferenceInputValue<'_, '_, '_, '_>, (): &()| {
        reference_input_parts(value).map(|_| ())
            .map_err(|_| garde::Error::new("invalid fixture reference input relationship"))
    }))]
    value: ReferenceInputValue<'value, 'batch, 'entries, 'chart>,
}

/// Actual exclusive receiver context, borrowed only inside its mutable call.
struct AchievedPhaseValue<'value, 'chart> {
    /// Current readiness and exact predecessor binding, held under exclusivity.
    owner: &'value FixtureReferenceInputState<'chart>,
    /// Caller-supplied predecessor; structural equality cannot replace identity.
    predecessor: &'value FixtureAffineChart,
    /// Checked distinct B-to-C frame/generation tags, without numeric ordering.
    frames: FixtureChartFrames,
    /// Checked complete common time domain, including its displacement reference.
    domain: FixtureChartDomain,
    /// Only the modeled achieved displacement in seconds; no requested quantity.
    achieved_phase_seconds: f64,
}

/// Private checked effect; no owner borrow or reusable admission token escapes.
struct AchievedPhaseEffect {
    /// Owned B-to-C descriptor whose zero rate was admitted with its achieved phase.
    step: FixtureAffineChart,
}

/// Prepare one checked zero-rate effect through the existing invariant owners.
///
/// The shared Garde and typed path use this exact ordered relationship. Checked
/// frames already own B-to-C distinctness; the endpoint constructor owns A-to-C
/// distinctness. All work is constant and neither input nor owner is changed.
/// # Errors
/// Reject readiness first, exact identity, B mapping, domain, scalar, then A-to-C
/// endpoint validity. Preserve the exact bounded cause of every chart rejection.
fn achieved_phase_effect(
    value: &AchievedPhaseValue<'_, '_>,
) -> Result<AchievedPhaseEffect, FixtureReferenceInputError> {
    reference_chart_identity(value.owner, value.predecessor)?;
    reference_chart_mapping(value.predecessor, value.frames, value.domain)?;
    let predecessor_frames = value.predecessor.frames();
    let effect = FixtureAffineChart::new(
        value.frames,
        value.domain,
        value.achieved_phase_seconds,
        // Negative zero preserves even a predecessor rate's negative-zero bits.
        -0.0,
    )
    .map_err(|source| FixtureReferenceInputError::Chart { source })?;
    FixtureChartFrames::new(
        predecessor_frames.source_frame(),
        predecessor_frames.source_generation(),
        value.frames.target_frame(),
        value.frames.target_generation(),
    )
    .map_err(|source| FixtureReferenceInputError::Chart { source })?;
    Ok(AchievedPhaseEffect { step: effect })
}

/// Check current readiness before exact-instance identity, without mutation.
/// # Errors
/// Preserve unavailable state or foreign chart identity; equal values do not bind.
fn reference_chart_identity(
    owner: &FixtureReferenceInputState<'_>,
    predecessor: &FixtureAffineChart,
) -> Result<(), FixtureReferenceInputError> {
    if owner.readiness != FixtureReferenceReadiness::Ready {
        return Err(FixtureReferenceInputError::Unavailable {
            readiness: owner.readiness,
        });
    }
    if !ptr::eq(ptr::from_ref(predecessor), ptr::from_ref(owner.chart)) {
        return Err(FixtureReferenceInputError::ForeignChartView);
    }
    Ok(())
}

/// Reuse checked tags/domain while checking the complete composition relationship.
/// # Errors
/// Mapping mismatch precedes complete domain/reference mismatch; no effects occur.
fn reference_chart_mapping(
    predecessor: &FixtureAffineChart,
    frames: FixtureChartFrames,
    domain: FixtureChartDomain,
) -> Result<(), FixtureReferenceInputError> {
    let predecessor_frames = predecessor.frames();
    if predecessor_frames.target_frame() != frames.source_frame()
        || predecessor_frames.target_generation() != frames.source_generation()
    {
        return Err(FixtureReferenceInputError::Chart {
            source: FixtureChartError::CompositionMappingMismatch,
        });
    }
    if predecessor.domain() != domain {
        return Err(FixtureReferenceInputError::Chart {
            source: FixtureChartError::CompositionDomainMismatch,
        });
    }
    Ok(())
}

/// Mandatory Garde context used and consumed inside the exclusive receiver.
#[derive(Validate)]
struct RawAchievedPhase<'value, 'chart> {
    /// One maintained rule owns current state, identity, context and scalar rules.
    #[garde(custom(|value: &AchievedPhaseValue<'_, '_>, (): &()| {
        achieved_phase_effect(value).map(|_| ())
            .map_err(|_| garde::Error::new("invalid fixture achieved phase relationship"))
    }))]
    value: AchievedPhaseValue<'value, 'chart>,
}

impl<'chart> FixtureReferenceInputState<'chart> {
    /// Bind one immutable checked chart in Ready state without external effects.
    ///
    /// The caller supplies a modeled fixture view. Construction neither verifies
    /// an installed mapping nor establishes continuity with a different owner.
    #[must_use]
    pub const fn new(chart: &'chart FixtureAffineChart) -> Self {
        Self {
            chart,
            readiness: FixtureReferenceReadiness::Ready,
        }
    }

    /// Copy this owner's closed readiness without inspecting chart numerics.
    #[must_use]
    pub const fn readiness(&self) -> FixtureReferenceReadiness {
        self.readiness
    }

    /// Apply modeled feedback monotonically, with no restoration or clock effect.
    ///
    /// `ProvenNoEffect` is identity; `CoordinateChanged` cannot weaken `Unknown`.
    /// Repetition is idempotent and the returned state is the committed readiness.
    pub fn receive_feedback(
        &mut self,
        feedback: FixtureCorrectionFeedback,
    ) -> FixtureReferenceReadiness {
        self.readiness = match (self.readiness, feedback) {
            (_, FixtureCorrectionFeedback::Unknown) | (FixtureReferenceReadiness::Unknown, _) => {
                FixtureReferenceReadiness::Unknown
            }
            (_, FixtureCorrectionFeedback::CoordinateChanged)
            | (FixtureReferenceReadiness::CoordinateChanged, _) => {
                FixtureReferenceReadiness::CoordinateChanged
            }
            (FixtureReferenceReadiness::Ready, FixtureCorrectionFeedback::ProvenNoEffect) => {
                FixtureReferenceReadiness::Ready
            }
        };
        self.readiness
    }

    /// Receive one pure modeled achieved displacement under exclusive ownership.
    ///
    /// `predecessor` must be this owner's exact A-to-B chart instance. `frames`
    /// supplies the checked B-to-C mapping and `domain` must equal its complete
    /// domain/reference. The existing scalar constructor admits achieved seconds
    /// with zero effect rate; the existing composer alone calculates A-to-C.
    /// Positive achieved phase increases chart displacement and reduces projected
    /// offset. Signed zero with distinct tags is a coordinate change; the separate
    /// `ProvenNoEffect` feedback is the identity operation.
    ///
    /// Mandatory private Garde admission finishes before mutation. The call then
    /// commits `CoordinateChanged` before fallible composition and returns an
    /// admitted completion carrying either its replacement or exact chart error.
    /// No replacement restores this owner. Repeated calls reject locally; no
    /// global operation identity, durable replay protection or clock effect is
    /// introduced. Work/storage are constant and histories are never touched.
    ///
    /// The mutable receiver excludes all live reference-input/estimate borrows;
    /// no externally borrowed effect token is needed to call it.
    /// # Errors
    /// Outer rejection preserves readiness and both charts. Unavailable state
    /// dominates foreign identity, mapping/domain/scalar/endpoint rejection and
    /// Garde disagreement. An admitted composition failure is inside the returned
    /// completion and always leaves the old owner `CoordinateChanged`.
    pub fn receive_achieved_phase(
        &mut self,
        predecessor: &FixtureAffineChart,
        frames: FixtureChartFrames,
        domain: FixtureChartDomain,
        achieved_phase_seconds: f64,
    ) -> Result<FixtureAchievedPhaseCompletion, FixtureReferenceInputError> {
        let effect = {
            let raw = RawAchievedPhase {
                value: AchievedPhaseValue {
                    owner: self,
                    predecessor,
                    frames,
                    domain,
                    achieved_phase_seconds,
                },
            };
            let validated = raw.validate();
            let effect = achieved_phase_effect(&raw.value)?;
            validated.map_err(|_| FixtureReferenceInputError::ValidationInvariant)?;
            effect
        };
        self.readiness = FixtureReferenceReadiness::CoordinateChanged;
        Ok(FixtureAchievedPhaseCompletion {
            replacement: predecessor.compose(&effect.step),
        })
    }

    /// Admit the actual ready emission through mandatory shared Garde validation.
    ///
    /// The token borrows this owner and outcome; repeated admission is read-only
    /// reuse of the same emission, without new observation or proposal credit.
    /// # Errors
    /// Return unavailable readiness, incomplete emission, foreign chart instance
    /// or Garde disagreement. Rejection leaves both borrowed inputs unchanged.
    pub fn admit_reference_input<'value, 'batch, 'entries>(
        &'value self,
        outcome: &'value FixtureChartCombinationOutcome<'batch, 'entries, 'chart>,
    ) -> Result<FixtureReferenceInput<'value, 'batch, 'entries, 'chart>, FixtureReferenceInputError>
    {
        let raw = RawReferenceInput {
            value: ReferenceInputValue {
                owner: self,
                outcome,
            },
        };
        let validated = raw.validate();
        let parts = reference_input_parts(&raw.value)?;
        validated.map_err(|_| FixtureReferenceInputError::ValidationInvariant)?;
        Ok(FixtureReferenceInput {
            _owner: self,
            _outcome: outcome,
            combined: parts.combined,
            emitted_round: parts.emitted_round,
        })
    }
}

/// Non-Clone checked input borrowing both the readiness owner and actual outcome.
///
/// Private token fields prevent unchecked construction, even with checked inputs:
///
/// ```compile_fail,E0451
/// use std::num::NonZeroU64;
/// use time_core::{FixtureChartCombinationOutcome, FixtureChartCombinedEstimate,
///     FixtureReferenceInput, FixtureReferenceInputState};
/// fn forge<'value, 'batch, 'entries, 'chart>(
///     owner: &'value FixtureReferenceInputState<'chart>,
///     outcome: &'value FixtureChartCombinationOutcome<'batch, 'entries, 'chart>,
///     combined: &'value FixtureChartCombinedEstimate<'batch, 'entries, 'chart>,
/// ) -> FixtureReferenceInput<'value, 'batch, 'entries, 'chart> {
///     FixtureReferenceInput {
///         _owner: owner, _outcome: outcome, combined,
///         emitted_round: NonZeroU64::MIN,
///     }
/// }
/// # let _forge = forge;
/// ```
///
/// A nested chart borrow through a live token also excludes mutable feedback:
///
/// ```compile_fail,E0502
/// use time_core::{FixtureChartCombinationOutcome, FixtureCorrectionFeedback,
///     FixtureReferenceInputError, FixtureReferenceInputState};
/// fn overlap<'batch, 'entries, 'chart>(
///     owner: &mut FixtureReferenceInputState<'chart>,
///     outcome: &FixtureChartCombinationOutcome<'batch, 'entries, 'chart>,
/// ) -> Result<(), FixtureReferenceInputError> {
///     let input = owner.admit_reference_input(outcome)?;
///     let chart = input.combined().chart();
///     owner.receive_feedback(FixtureCorrectionFeedback::CoordinateChanged);
///     let _domain = chart.domain();
///     Ok(())
/// }
/// # let _overlap = overlap;
/// ```
///
/// The same live chart borrow also excludes achieved-phase reception:
///
/// ```compile_fail,E0502
/// use time_core::{FixtureAffineChart, FixtureChartCombinationOutcome,
///     FixtureChartDomain, FixtureChartFrames, FixtureReferenceInputError,
///     FixtureReferenceInputState};
/// fn overlap_phase<'batch, 'entries, 'chart>(
///     owner: &mut FixtureReferenceInputState<'chart>,
///     outcome: &FixtureChartCombinationOutcome<'batch, 'entries, 'chart>,
///     predecessor: &FixtureAffineChart, frames: FixtureChartFrames,
///     domain: FixtureChartDomain, achieved_phase_seconds: f64,
/// ) -> Result<(), FixtureReferenceInputError> {
///     let input = owner.admit_reference_input(outcome)?;
///     let chart = input.combined().chart();
///     let _completion = owner.receive_achieved_phase(
///         predecessor, frames, domain, achieved_phase_seconds)?;
///     let _domain = chart.domain();
///     Ok(())
/// }
/// # let _overlap = overlap_phase;
/// ```
///
/// Ending the input borrow before reception is a legal neighboring consumer:
///
/// ```
/// use time_core::{FixtureAchievedPhaseCompletion, FixtureAffineChart,
///     FixtureChartCombinationOutcome, FixtureChartDomain, FixtureChartFrames,
///     FixtureReferenceInputError, FixtureReferenceInputState};
/// fn scoped_phase<'batch, 'entries, 'chart>(
///     owner: &mut FixtureReferenceInputState<'chart>,
///     outcome: &FixtureChartCombinationOutcome<'batch, 'entries, 'chart>,
///     predecessor: &FixtureAffineChart, frames: FixtureChartFrames,
///     domain: FixtureChartDomain, achieved_phase_seconds: f64,
/// ) -> Result<FixtureAchievedPhaseCompletion, FixtureReferenceInputError> {
///     {
///         let input = owner.admit_reference_input(outcome)?;
///         let _phase = input.combined().phase_seconds();
///     }
///     owner.receive_achieved_phase(
///         predecessor, frames, domain, achieved_phase_seconds)
/// }
/// # let _scoped_phase = scoped_phase;
/// ```
///
/// A scoped token permits later feedback without creating another observation:
///
/// ```
/// use std::num::NonZeroU64;
/// use time_core::{FixtureChartCombinationOutcome, FixtureCorrectionFeedback,
///     FixtureReferenceInputError, FixtureReferenceInputState};
/// fn scoped<'batch, 'entries, 'chart>(
///     owner: &mut FixtureReferenceInputState<'chart>,
///     outcome: &FixtureChartCombinationOutcome<'batch, 'entries, 'chart>,
/// ) -> Result<NonZeroU64, FixtureReferenceInputError> {
///     let round = {
///         let input = owner.admit_reference_input(outcome)?;
///         let _phase = input.combined().phase_seconds();
///         input.emitted_round()
///     };
///     owner.receive_feedback(FixtureCorrectionFeedback::ProvenNoEffect);
///     Ok(round)
/// }
/// # let _scoped = scoped;
/// ```
pub struct FixtureReferenceInput<'value, 'batch, 'entries, 'chart> {
    /// This retained borrow prevents mutable feedback while the token is used.
    _owner: &'value FixtureReferenceInputState<'chart>,
    /// Retain the actual outcome without exposing a longer-lived outcome accessor.
    _outcome: &'value FixtureChartCombinationOutcome<'batch, 'entries, 'chart>,
    /// Checked actual field; access shortens every inner borrow to the token use.
    combined: &'value FixtureChartCombinedEstimate<'batch, 'entries, 'chart>,
    /// Actual existing round, with no new round allocation or arithmetic.
    emitted_round: NonZeroU64,
}

impl fmt::Debug for FixtureReferenceInput<'_, '_, '_, '_> {
    /// Redact the owner, estimate, source identities and emission ordinal.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureReferenceInput { .. }")
    }
}

impl FixtureReferenceInput<'_, '_, '_, '_> {
    /// Borrow the actual estimate only through this token's current short borrow.
    ///
    /// Inner batch, entry and chart references are shortened as well; extracting
    /// them cannot extend an estimate use beyond the token's owner borrow.
    #[must_use]
    pub const fn combined(&self) -> &FixtureChartCombinedEstimate<'_, '_, '_> {
        self.combined
    }

    /// Copy the original nonzero emitted round without creating further credit.
    #[must_use]
    pub const fn emitted_round(&self) -> NonZeroU64 {
        self.emitted_round
    }
}

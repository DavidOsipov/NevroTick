// SPDX-License-Identifier: GPL-2.0-only
//! Bounded numerical contracts without sample or clock authority.
//!
//! S1.2a supplies mandatory checked point construction, response-independent
//! geometry and a bounded inverse-divisor affine fit. Exact constant phases
//! retain the first admitted phase bits with zero rate; nonconstant response
//! arithmetic is evaluated in the prescribed chronological binary64 order.
//! S1.2b/c add fresh-line fixture residual diagnostics and exact-observation
//! normalization. S1.2d adds one bounded fixture-history transaction: context/order
//! admission, staged exact rows, one normalization and fresh same-input statistics
//! precede complete state replacement. Rejection preserves the prior rows and
//! report. These fixture values supply no physical uncertainty, eligibility,
//! source selection, network, persistence or clock authority.
//! `WindowVarianceDivisors` separately derives checked dimensionless divisors from
//! a complete bounded peer-distance window and explicit precision. Its borrowed
//! numeric snapshot establishes no physical quality or production sample admission.
//!
//! ```
//! # use std::num::NonZeroU64;
//! # use time_core::{FitError, FitInput, FitPoint};
//! # fn main() -> Result<(), FitError> {
//! let points = [
//!     FitPoint::new(-2.0, 0.001, 1.0)?,
//!     FitPoint::new(-1.0, 0.002, 1.0)?,
//!     FitPoint::new(0.0, 0.003, 1.0)?,
//! ];
//! let input = FitInput::new(NonZeroU64::MIN, &points)?;
//! assert_eq!(input.frame(), NonZeroU64::MIN);
//! assert_eq!(input.sample_count(), 3);
//! assert_eq!(points[0].relative_seconds(), -2.0);
//! # Ok(()) }
//! ```
#![forbid(unsafe_code)]

/// Sole checked owner for synthetic point, batch, geometry and result contracts.
mod regression;

/// Checked synthetic intervals and bounded, inert fixture-majority selection.
mod selection;

/// Exact Q33 enclosure bridge between checked fixture projection and interval owners.
mod projection_enclosure;

/// Chart-specific geometry consumes numerical bounds and retains chart provenance.
mod chart_enclosure;

/// Complete same-chart interval admission with provenance-retaining selection.
mod chart_batch;

/// Borrowed common-context admission and selection of fixture projection enclosures.
mod projected_batch;

/// Fixed-roster fixture principal contracts and checked parameter/stratum admission.
mod principal;

/// Checked common-instant uncertainty admission and inert supporter-combination kernel.
mod combination;

/// Checked source-quality window derivation and aligned variance divisors.
mod window_variance;

/// Exact row association and once-rounded, conditioned numerical preparation.
mod conditioned_window;

/// Borrowed conditioned suffixes with enclosed affine fitting and unit-safe lifting.
mod conditioned_fit;

pub use conditioned_fit::{
    ConditionedFit, ConditionedFitError, ConditionedFitIssue, ConditionedFitOperation,
    ConditionedFitStage, ConditionedNormalizedEstimate, ConditionedPhaseSeconds,
    ConditionedRateRatio, ConditionedSuffix, ConditionedSuffixOutcome,
    ConditionedWeightedSecondsSquared, InsufficientConditionedSuffix, fit_conditioned_suffix,
    prepare_conditioned_suffix,
};

pub use conditioned_window::{
    ConditionedBatchOutcome, ConditionedCoordinate, ConditionedWindowError,
    DeclaredObservationContext, ExactObservationRow, InsufficientConditionedBatch,
    PreparedConditionedBatch, PreparedConditionedPoint, prepare_conditioned_batch,
};

pub use window_variance::{WindowVarianceDivisors, WindowVarianceError, WindowVarianceStage};

pub use combination::{
    FixtureCombinationContextField, FixtureCombinationDenominatorField, FixtureCombinationError,
    FixtureCombinationKernelKind, FixtureCombinationKernelOutcome, FixtureCombinationScalarField,
    FixtureCombinationScalarIssue, FixtureCombinationStep, FixtureCombinedEstimate,
    FixtureSupporterUncertainty, FixtureTrackingCombinationError, combine_fixture_supporters,
    combine_fixture_tracking_supporters,
};

pub use combination::{
    FixtureChartCombinationError, FixtureChartCombinationOutcome, FixtureChartCombinedEstimate,
    FixtureCombinationLifecycleError, FixtureCombinationLifecycleStep, FixtureCombinationOutcome,
    FixtureCombinationParameterField, FixtureCombinationParameterIssue,
    FixtureCombinationParameters, FixtureCombinationSourceState, FixtureCombinationSourceStatus,
    FixtureCombinationState, FixtureReachabilityUpdate, FixtureTrackingLifecycleError,
};

pub use principal::{
    FixtureChartPrincipalDecision, FixtureChartPrincipalError, FixturePrincipalContextField,
    FixturePrincipalDecision, FixturePrincipalError, FixturePrincipalOutcomeKind,
    FixturePrincipalParameterField, FixturePrincipalParameters, FixturePrincipalScalarIssue,
    FixturePrincipalSourceState, FixturePrincipalState, FixturePrincipalStep,
    FixturePrincipalStratum,
};

pub use projected_batch::{
    FixtureProjectedBatch, FixtureProjectedBatchError, FixtureProjectionMismatchKind,
};

pub use projection_enclosure::{
    FixtureEnclosureError, FixtureEndpointRounding, FixtureProjectedInterval,
    enclose_fixture_projection,
};

pub use chart_enclosure::{FixtureChartInterval, enclose_fixture_chart_projection};

pub use chart_batch::{
    FixtureChartBatch, FixtureChartBatchError, FixtureChartMismatchKind, FixtureChartSelection,
};

pub use regression::{
    FixtureAdaptiveChartView, FixtureAffineChart, FixtureChartBoundIssue,
    FixtureChartBoundOperation, FixtureChartDomain, FixtureChartError, FixtureChartField,
    FixtureChartFrames, FixtureChartIssue, FixtureChartOutcome, FixtureChartProjection,
    FixtureChartStep, FixtureChartTracking,
};

pub use regression::{
    AdaptiveHistoryError, ArithmeticIssue, ArithmeticOperation, ArithmeticStage, FitError,
    FitInput, FitPoint, FixtureAdaptiveHistory, FixtureAdaptiveHistoryReport,
    FixtureAnchorProjection, FixtureContext, FixtureHistory, FixtureHistoryReport, FixtureLine,
    FixtureNormalization, FixtureObservation, FixtureProjectionError, FixtureProjectionField,
    FixtureProjectionIssue, FixtureProjectionStep, FixtureProjectionTerms,
    FixtureResidualStatistics, FixtureRunsError, FixtureRunsSelection, FixtureRunsStage,
    FixtureRunsStop, FixtureStandardError, FixtureStandardErrorField, FixtureStandardErrorStep,
    FixtureStandardErrors, FixtureTrackingEstimate, FixtureTrackingField,
    FixtureTrackingProjectionError, FixtureTrackingStep, HistoryError, InsufficientFixture,
    NormalizationComponent, NormalizationError, NormalizationStep, ObservationError, PointField,
    PointIssue, PreparedFixtureInput, ResidualError, ResidualField, ResidualIssue, ResidualStep,
    fit, prepare_fixture_input, project_fixture_adaptive_anchor, project_fixture_adaptive_tracking,
    project_fixture_anchor, residual_statistics, select_fixture_runs,
    select_fixture_runs_with_retained,
};

pub use selection::{
    BatchError, FixtureBatch, FixtureInterval, FixtureMinimum, FixtureOutcomeKind,
    FixtureSelection, FixtureWitness, IntervalError, MinimumError, SelectionError,
    SelectionInvariantStage, select_fixture,
};

/// Borrowed fixture input, monotone readiness and exclusive modeled phase feedback.
mod reference_input;

pub use reference_input::{
    FixtureAchievedPhaseCompletion, FixtureCorrectionFeedback, FixturePhaseBaseline,
    FixturePhaseBlocked, FixturePhaseConstructionRejected, FixturePhaseError, FixturePhaseNow,
    FixturePhasePlanOwner, FixturePhaseProfile, FixturePhaseProposal, FixturePhaseProvenance,
    FixturePhaseReferenceAdmission, FixturePhaseResidualPermission, FixturePhaseRetirement,
    FixturePhaseRetirementReason, FixturePhaseSettlement, FixtureRateClock,
    FixtureRateClockContext, FixtureRateCursorStatus, FixtureRateDeliveryError,
    FixtureRateSettlement, FixtureRateSettlementCompletion, FixtureRateSettlementCursor,
    FixtureRateSettlementKind, FixtureRateSpan, FixtureReferenceInput, FixtureReferenceInputError,
    FixtureReferenceInputState, FixtureReferenceReadiness, FixtureSettledRateInterval,
};

pub use regression::{
    FixturePhaseNumericError, FixturePhaseNumericField, FixturePhaseNumericIssue,
    FixturePhaseNumericStep,
};

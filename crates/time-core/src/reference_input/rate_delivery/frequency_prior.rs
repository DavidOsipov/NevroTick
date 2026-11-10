// SPDX-License-Identifier: GPL-2.0-only
//! Genuine-reference frequency state and conditional monotonic-chart prediction.
//!
//! The phase owner supplies the actual token, current-time admission and consumed
//! delivery descriptor. Private numeric constructors own transformed scalars;
//! this module owns clock association, model provenance and finite quality. No
//! model value acknowledges a baseline, creates evidence or authorizes an effect.

use std::{fmt, num::NonZeroU64};

use garde::Validate;
use time_domain::{MonotonicDurationNs, MonotonicNs};

use super::{
    FixtureRateClock, FixtureRateClockContext, FixtureRateCursorStatus,
    FixtureRateSettlementCursor, phase_plan::FixturePhaseRetirementReason,
};
use crate::{
    FixtureAffineChart, FixtureChartCombinedEstimate, FixtureChartDomain, FixtureChartError,
    FixtureChartFrames, FixtureContext, FixtureFrequencyNumericError,
    regression::{FrequencyAnchor, FrequencyPrediction, FrequencyPrior},
};

/// Exact relative nanoseconds supported by the common integer-to-float owner.
const MAX_EXACT_NANOSECONDS: u64 = 1_u64 << 53;

/// Closed failure families, without rejected values or caller-supplied text.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum FixtureFrequencyError {
    /// A raw profile violates the supported scalar or duration envelope.
    InvalidProfile,
    /// This owner was deliberately constructed without a frequency model.
    Disabled,
    /// Current clock-origin or generation assertions differ.
    ClockContextMismatch,
    /// Actual source/chart assertions do not match the explicit fixture binding.
    ClockAssociationMismatch,
    /// A supplied current reading predates a retained reading or endpoint.
    TimeRegression {
        /// The independently ordered domain that failed first.
        clock: FixtureRateClock,
    },
    /// An enabled constructor received a terminal or differently mapped cursor.
    CursorUnavailable,
    /// A query precedes the genuine model/evidence reference.
    QueryBeforeReference,
    /// A query is outside the actual current mathematical chart domain.
    QueryOutsideDomain,
    /// No genuine eligible reference has produced a model.
    NoModel,
    /// Original evidence age or the outward growth upper reached its limit.
    Expired,
    /// An issued operation has not supplied its actual terminal result.
    OutstandingOperation,
    /// Stop permanently retired useful model projection on this owner.
    Stopped,
    /// A generation change permanently retired this model's association.
    Generation,
    /// Unknown delivery preserves history without a usable current model.
    Unknown,
    /// The genuine measured skew exceeds the explicit profile threshold.
    MeasuredSkew,
    /// A known transition could not safely re-express the historical anchor.
    AnchorRebaseUnavailable,
    /// A checked numerical transformation could not produce its promised bound.
    Numeric {
        /// Exact bounded numerical cause, never formatted into the parent text.
        source: FixtureFrequencyNumericError,
    },
    /// Mandatory validation or actual once-only receipt invariants disagreed.
    ValidationInvariant,
}

impl fmt::Debug for FixtureFrequencyError {
    /// Keep even nested cause payloads outside the fixed diagnostic surface.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureFrequencyError { .. }")
    }
}

impl fmt::Display for FixtureFrequencyError {
    /// Report only the closed family, never rejected numeric or identity fields.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidProfile => "fixture frequency: invalid profile",
            Self::Disabled => "fixture frequency: disabled",
            Self::ClockContextMismatch => "fixture frequency: clock context mismatch",
            Self::ClockAssociationMismatch => "fixture frequency: clock association mismatch",
            Self::TimeRegression { .. } => "fixture frequency: time regression",
            Self::CursorUnavailable => "fixture frequency: cursor unavailable",
            Self::QueryBeforeReference => "fixture frequency: query before reference",
            Self::QueryOutsideDomain => "fixture frequency: query outside domain",
            Self::NoModel => "fixture frequency: no model",
            Self::Expired => "fixture frequency: expired",
            Self::OutstandingOperation => "fixture frequency: outstanding operation",
            Self::Stopped => "fixture frequency: stopped",
            Self::Generation => "fixture frequency: generation retired",
            Self::Unknown => "fixture frequency: unknown delivery",
            Self::MeasuredSkew => "fixture frequency: measured skew refused",
            Self::AnchorRebaseUnavailable => "fixture frequency: anchor rebase unavailable",
            Self::Numeric { .. } => "fixture frequency: numerical refusal",
            Self::ValidationInvariant => "fixture frequency: validation invariant",
        })
    }
}

impl std::error::Error for FixtureFrequencyError {
    /// Preserve the separately bounded numerical cause without flattening it.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Numeric { source } => Some(source),
            Self::InvalidProfile
            | Self::Disabled
            | Self::ClockContextMismatch
            | Self::ClockAssociationMismatch
            | Self::TimeRegression { .. }
            | Self::CursorUnavailable
            | Self::QueryBeforeReference
            | Self::QueryOutsideDomain
            | Self::NoModel
            | Self::Expired
            | Self::OutstandingOperation
            | Self::Stopped
            | Self::Generation
            | Self::Unknown
            | Self::MeasuredSkew
            | Self::AnchorRebaseUnavailable
            | Self::ValidationInvariant => None,
        }
    }
}

/// Immutable declared model limits, never shipping defaults or physical assurance.
///
/// Every public construction path invokes the maintained profile rule:
/// ```
/// use time_core::FixtureFrequencyProfile;
/// use time_domain::MonotonicNs;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let age = MonotonicNs::new(1).duration_since(MonotonicNs::new(0))?;
/// let profile = FixtureFrequencyProfile::new(0.0, 0.0, 1.0, age)?;
/// assert_eq!(profile.maximum_age().nanoseconds(), 1);
/// assert!(FixtureFrequencyProfile::new(f64::NAN, 0.0, 1.0, age).is_err());
/// # Ok(()) }
/// ```
/// Raw fields cannot bypass that construction:
/// ```compile_fail,E0451
/// use time_core::FixtureFrequencyProfile;
/// use time_domain::MonotonicNs;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let age = MonotonicNs::new(1).duration_since(MonotonicNs::new(0))?;
/// let _raw = FixtureFrequencyProfile {
///     max_skew: -1.0, clock_error: 0.0, growth_cap: 1.0,
///     max_age: age,
/// };
/// # Ok(()) }
/// ```
#[derive(Clone, Copy)]
pub struct FixtureFrequencyProfile {
    /// Inclusive maximum genuine measured tracking skew.
    max_skew: f64,
    /// Declared conditional clock-error growth per monotonic second.
    clock_error: f64,
    /// Exclusive outward added-growth limit in seconds.
    growth_cap: f64,
    /// Exclusive age limit measured from the actual latest observation.
    max_age: MonotonicDurationNs,
}

/// Whole raw profile checked together by the one maintained predicate.
type ProfileValues = (f64, f64, f64, MonotonicDurationNs);

/// Admit only nonnegative supported rates, positive growth and exact finite age.
fn profile_valid(values: &ProfileValues) -> bool {
    let (skew, clock_error, growth, age) = *values;
    [skew, clock_error]
        .into_iter()
        .all(|value| (value.is_normal() || value == 0.0) && value >= 0.0)
        && growth.is_normal()
        && growth > 0.0
        && age.nanoseconds() != 0
        && age.nanoseconds() <= MAX_EXACT_NANOSECONDS
}

/// Mandatory Garde adapter for the complete raw profile relation.
#[derive(Validate)]
struct RawProfile {
    /// One rule covers every raw scalar and the duration relationship.
    #[garde(custom(|value: &ProfileValues, (): &()| {
        if profile_valid(value) { Ok(()) } else { Err(garde::Error::new("invalid frequency profile")) }
    }))]
    value: ProfileValues,
}

impl FixtureFrequencyProfile {
    /// Admit explicit fixture limits through mandatory shared validation.
    /// # Errors
    /// Reject unsupported rates, nonpositive growth/age or validation disagreement.
    pub fn new(
        maximum_measured_skew: f64,
        maximum_clock_error_rate: f64,
        growth_cap: f64,
        maximum_age: MonotonicDurationNs,
    ) -> Result<Self, FixtureFrequencyError> {
        let raw = RawProfile {
            value: (
                maximum_measured_skew,
                maximum_clock_error_rate,
                growth_cap,
                maximum_age,
            ),
        };
        let validated = raw.validate();
        if !profile_valid(&raw.value) {
            return Err(FixtureFrequencyError::InvalidProfile);
        }
        validated.map_err(|_| FixtureFrequencyError::ValidationInvariant)?;
        Ok(Self {
            max_skew: maximum_measured_skew,
            clock_error: maximum_clock_error_rate,
            growth_cap,
            max_age: maximum_age,
        })
    }

    /// Copy the explicitly declared inclusive measured-skew threshold.
    #[must_use]
    pub const fn maximum_measured_skew(&self) -> f64 {
        self.max_skew
    }
    /// Copy the declared conditional clock-error rate, not a calibration.
    #[must_use]
    pub const fn maximum_clock_error_rate(&self) -> f64 {
        self.clock_error
    }
    /// Copy the exclusive outward added-growth cap in seconds.
    #[must_use]
    pub const fn growth_cap(&self) -> f64 {
        self.growth_cap
    }
    /// Copy the exclusive original-observation age limit.
    #[must_use]
    pub const fn maximum_age(&self) -> MonotonicDurationNs {
        self.max_age
    }
}

impl fmt::Debug for FixtureFrequencyProfile {
    /// Hide declared limits instead of exposing accepted or rejected scalars.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureFrequencyProfile { .. }")
    }
}

/// Explicit fixture source-to-monotonic association, without native provenance.
#[derive(Clone, Copy)]
pub struct FixtureFrequencyClockAssociation {
    /// Typed source-frame assertion, checked against the actual chart on use.
    source_frame: NonZeroU64,
    /// Typed original source-generation assertion, never numerically ordered.
    source_generation: NonZeroU64,
    /// Independent monotonic-origin assertion, not an active-time conversion.
    monotonic_origin: NonZeroU64,
    /// Shared fixture clock-generation assertion.
    clock_generation: NonZeroU64,
}

impl FixtureFrequencyClockAssociation {
    /// Retain nonzero labels; the actual relationship is mandatory at receivers.
    #[must_use]
    pub const fn new(
        source_frame: NonZeroU64,
        source_generation: NonZeroU64,
        monotonic_origin: NonZeroU64,
        clock_generation: NonZeroU64,
    ) -> Self {
        Self {
            source_frame,
            source_generation,
            monotonic_origin,
            clock_generation,
        }
    }
}

impl fmt::Debug for FixtureFrequencyClockAssociation {
    /// Preserve the fixed redaction contract for all fixture labels.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureFrequencyClockAssociation { .. }")
    }
}

/// One actual source-admission result; no variant itself grants current quality.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum FixtureFrequencyAdmission {
    /// This is deliberately the legacy finite-only owner.
    Disabled,
    /// Actual advancement supplied no admissible ready estimate.
    NoReady {
        /// Original producer round if that committed outcome emitted one.
        emitted_round: Option<NonZeroU64>,
    },
    /// The whole genuine model record was initialized or updated.
    Accepted {
        /// Actual consumed producer emission, never caller-created credit.
        emitted_round: NonZeroU64,
    },
    /// Actual emitted credit remains consumed despite model refusal.
    Refused {
        /// Original actual emitted round whose model processing failed.
        emitted_round: NonZeroU64,
        /// Exact bounded refusal, independent of the phase admission outcome.
        reason: FixtureFrequencyError,
    },
}

impl fmt::Debug for FixtureFrequencyAdmission {
    /// Hide actual round and bounded cause details from this opaque summary.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureFrequencyAdmission { .. }")
    }
}

/// Owned immutable identity of the exact mathematical basis used by a model.
#[derive(Clone, Copy)]
struct Basis {
    /// Checked frame identities supplied by the actual chart constructor.
    frames: FixtureChartFrames,
    /// Checked domain, preserved without independent raw reconstruction.
    domain: FixtureChartDomain,
    /// Original stored coefficient bits, used only for exact basis identity.
    phase: u64,
    /// Original stored rate bits, separate from model frequency.
    rate: u64,
    /// Original stored coefficient-error bits.
    phase_error: u64,
    /// Original stored rate-error bits.
    rate_error: u64,
}

impl Basis {
    /// Capture only immutable checked chart fields, without numerical conversion.
    fn from_chart(chart: &FixtureAffineChart) -> Self {
        Self {
            frames: chart.frames(),
            domain: chart.domain(),
            phase: chart.phase_at_reference_seconds().to_bits(),
            rate: chart.rate_seconds_per_second().to_bits(),
            phase_error: chart.phase_coefficient_roundoff_seconds().to_bits(),
            rate_error: chart
                .rate_coefficient_roundoff_seconds_per_second()
                .to_bits(),
        }
    }

    /// Match complete saved geometry, not merely equal frame tags.
    fn matches(&self, chart: &FixtureAffineChart) -> bool {
        self.frames == chart.frames()
            && same_domain(self.domain, chart.domain())
            && self.phase == chart.phase_at_reference_seconds().to_bits()
            && self.rate == chart.rate_seconds_per_second().to_bits()
            && self.phase_error == chart.phase_coefficient_roundoff_seconds().to_bits()
            && self.rate_error
                == chart
                    .rate_coefficient_roundoff_seconds_per_second()
                    .to_bits()
    }
}

/// Immutable whole model; private factories retain one actual reference record.
#[derive(Clone, Copy)]
struct Model {
    /// Numerically checked mean/skew/residual and their inherited bounds.
    prior: FrequencyPrior,
    /// Numerically checked phase anchor in the current mathematical basis.
    anchor: FrequencyAnchor,
    /// Current checked geometry; known re-expression alone may replace this.
    basis: Basis,
    /// Actual source context copied while its genuine token was borrowed.
    source: FixtureContext,
    /// Original reference's chart frames, distinct from the current basis.
    reference_frames: FixtureChartFrames,
    /// Actual producer emission that supplied this complete record.
    round: NonZeroU64,
    /// Original actual raw-anchor observation time.
    observed_anchor: MonotonicNs,
    /// Original latest observation, never replaced by intake/projection time.
    latest: MonotonicNs,
    /// Original same-owner fit reference.
    fit: MonotonicNs,
    /// Actual projected epoch of this model's genuine phase anchor.
    projected: MonotonicNs,
    /// Original source statistical phase SD, not posterior model variance.
    phase_sd: f64,
    /// Original source statistical rate SD, not the skew gain weight.
    rate_sd: f64,
}

/// Historical immutable model diagnostics, never a current-quality capability.
///
/// A consumer can read the saved mean without obtaining a mutation path:
/// ```
/// use time_core::FixtureFrequencyHistory;
/// fn mean(history: &FixtureFrequencyHistory) -> f64 {
///     history.mean_seconds_per_second()
/// }
/// ```
/// ```compile_fail,E0616
/// use time_core::FixtureFrequencyHistory;
/// fn replace(mut history: FixtureFrequencyHistory) {
///     history.model = history.model;
/// }
/// ```
#[derive(Clone, Copy)]
pub struct FixtureFrequencyHistory {
    /// Only the actual state owner can create this complete saved snapshot.
    model: Model,
}

impl FixtureFrequencyHistory {
    /// Copy conditional model rate per monotonic second.
    #[must_use]
    pub const fn mean_seconds_per_second(&self) -> f64 {
        self.model.prior.mean()
    }
    /// Copy only deterministic inherited/new rate error.
    #[must_use]
    pub const fn mean_roundoff_seconds_per_second(&self) -> f64 {
        self.model.prior.mean_error()
    }
    /// Copy learned conditional skew, not a physical confidence guarantee.
    #[must_use]
    pub const fn skew_seconds_per_second(&self) -> f64 {
        self.model.prior.skew()
    }
    /// Copy deterministic skew enclosure independently of the nominal skew.
    #[must_use]
    pub const fn skew_roundoff_seconds_per_second(&self) -> f64 {
        self.model.prior.skew_error()
    }
    /// Copy the actual last assimilation's conditional residual rate.
    #[must_use]
    pub const fn residual_seconds_per_second(&self) -> f64 {
        self.model.prior.residual()
    }
    /// Copy its separate deterministic error upper.
    #[must_use]
    pub const fn residual_roundoff_seconds_per_second(&self) -> f64 {
        self.model.prior.residual_error()
    }
    /// Copy the anchor in its current counterfactual chart.
    #[must_use]
    pub const fn anchor_phase_seconds(&self) -> f64 {
        self.model.anchor.phase()
    }
    /// Copy the propagated anchor numerical bound.
    #[must_use]
    pub const fn anchor_roundoff_seconds(&self) -> f64 {
        self.model.anchor.error()
    }
    /// Copy original fitted phase SD, not a learned posterior SD.
    #[must_use]
    pub const fn phase_standard_deviation_seconds(&self) -> f64 {
        self.model.phase_sd
    }
    /// Copy original fitted rate SD, not a gain weight.
    #[must_use]
    pub const fn rate_standard_deviation_seconds_per_second(&self) -> f64 {
        self.model.rate_sd
    }
    /// Copy the original actual model-producing emission.
    #[must_use]
    pub const fn emitted_round(&self) -> NonZeroU64 {
        self.model.round
    }
    /// Copy original source identity without changing its generation.
    #[must_use]
    pub const fn source_context(&self) -> FixtureContext {
        self.model.source
    }
    /// Copy current mathematical frame tags, separate from original source tags.
    #[must_use]
    pub const fn frames(&self) -> FixtureChartFrames {
        self.model.basis.frames
    }
    /// Copy the genuine reference's original chart tags.
    #[must_use]
    pub const fn reference_frames(&self) -> FixtureChartFrames {
        self.model.reference_frames
    }
    /// Copy the current mathematical support domain.
    #[must_use]
    pub const fn domain(&self) -> FixtureChartDomain {
        self.model.basis.domain
    }
    /// Copy original raw-anchor observation time.
    #[must_use]
    pub const fn anchor_observed_at(&self) -> MonotonicNs {
        self.model.observed_anchor
    }
    /// Copy original latest observation time, including after re-expression.
    #[must_use]
    pub const fn latest_observed_at(&self) -> MonotonicNs {
        self.model.latest
    }
    /// Copy the genuine original fit reference.
    #[must_use]
    pub const fn fit_reference(&self) -> MonotonicNs {
        self.model.fit
    }
    /// Copy the actual phase-anchor epoch, never a later receipt time.
    #[must_use]
    pub const fn projected_at(&self) -> MonotonicNs {
        self.model.projected
    }
}

impl fmt::Debug for FixtureFrequencyHistory {
    /// Keep historical values, labels and original times private in formatting.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureFrequencyHistory { .. }")
    }
}

/// Immutable bounded prediction at one admitted supplied monotonic instant.
#[derive(Clone, Copy)]
pub struct FixtureFrequencyProjection {
    /// Whole saved model/provenance, with no detached tuple attachment path.
    history: FixtureFrequencyHistory,
    /// Checked phase and conditional added growth from the numeric owner.
    prediction: FrequencyPrediction,
    /// Actual supplied query whose state and budget checks completed.
    evaluated: MonotonicNs,
    /// Exact age since the actual original latest observation.
    age: MonotonicDurationNs,
}

impl FixtureFrequencyProjection {
    /// Borrow the exact historical model used for this completed prediction.
    #[must_use]
    pub const fn history(&self) -> &FixtureFrequencyHistory {
        &self.history
    }
    /// Copy conditional nominal phase, positive behind, at the saved query.
    #[must_use]
    pub const fn nominal_phase_seconds(&self) -> f64 {
        self.prediction.phase()
    }
    /// Copy only its deterministic arithmetic enclosure.
    #[must_use]
    pub const fn phase_roundoff_seconds(&self) -> f64 {
        self.prediction.phase_error()
    }
    /// Copy conditional extra prediction growth, never total root dispersion.
    #[must_use]
    pub const fn added_growth_seconds(&self) -> f64 {
        self.prediction.growth()
    }
    /// Copy the outward upper used by the exclusive growth-cap check.
    #[must_use]
    pub const fn added_growth_upper_seconds(&self) -> f64 {
        self.prediction.growth_upper()
    }
    /// Copy the supplied monotonic query without an ambient read.
    #[must_use]
    pub const fn evaluated_at(&self) -> MonotonicNs {
        self.evaluated
    }
    /// Copy exact original-observation age at this query.
    #[must_use]
    pub const fn evidence_age(&self) -> MonotonicDurationNs {
        self.age
    }
}

impl fmt::Debug for FixtureFrequencyProjection {
    /// Keep prediction and historical provenance outside the debug surface.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureFrequencyProjection { .. }")
    }
}

/// Every field is already immutable checked data; this rule owns association.
struct Relationship<'value> {
    /// Explicit fixture assertion retained by the enabled owner.
    association: FixtureFrequencyClockAssociation,
    /// Actual current chart, never a caller-produced scalar substitute.
    chart: &'value FixtureAffineChart,
    /// Actual cursor/receiver clock identity.
    context: FixtureRateClockContext,
    /// Actual source context when a genuine reference is being assimilated.
    source: Option<FixtureContext>,
}

/// Match monotonic identity before actual chart/source frame relationships.
/// # Errors
/// Reject mismatches without modifying source, model, clock or cursor state.
fn relationship(value: &Relationship<'_>) -> Result<(), FixtureFrequencyError> {
    let association = value.association;
    if association.monotonic_origin != value.context.monotonic_origin
        || association.clock_generation != value.context.generation
    {
        return Err(FixtureFrequencyError::ClockContextMismatch);
    }
    let frames = value.chart.frames();
    if association.source_frame != frames.source_frame()
        || association.source_generation != frames.source_generation()
        || value.source.is_some_and(|source| {
            source.frame() != association.source_frame
                || source.generation() != association.source_generation
        })
    {
        return Err(FixtureFrequencyError::ClockAssociationMismatch);
    }
    Ok(())
}

/// Mandatory Garde adapter for actual current association, not labels alone.
#[derive(Validate)]
struct RawRelationship<'value> {
    /// Whole chart/clock/source relationship is checked by the maintained rule.
    #[garde(custom(|value: &Relationship<'_>, (): &()| {
        relationship(value).map_err(|_| garde::Error::new("invalid frequency association"))
    }))]
    value: Relationship<'value>,
}

/// Invoke mandatory validation and retain the precise shared relationship cause.
/// # Errors
/// Reject clock/source mismatch or a validation/rule disagreement atomically.
fn checked_relationship(value: Relationship<'_>) -> Result<(), FixtureFrequencyError> {
    let raw = RawRelationship { value };
    let validated = raw.validate();
    relationship(&raw.value)?;
    validated.map_err(|_| FixtureFrequencyError::ValidationInvariant)
}

/// Immutable enabled configuration; no reset or optional caller validation exists.
#[derive(Clone, Copy)]
struct Configuration {
    /// Privately admitted scalar/time profile.
    profile: FixtureFrequencyProfile,
    /// Explicit source/monotonic assertion checked at every changed context.
    association: FixtureFrequencyClockAssociation,
}

/// Sole model state moved with the actual producer/cursor continuation.
pub(super) struct FrequencyState {
    /// None is permanently the legacy finite-only state for this instance.
    configuration: Option<Configuration>,
    /// Complete historical model; quality refusal never discards its facts.
    model: Option<Model>,
    /// Actual consumed emission, including refused model processing.
    consumed: Option<NonZeroU64>,
    /// Last actual source-admission result, independent of phase admission.
    admission: FixtureFrequencyAdmission,
    /// Sticky model-only loss; new genuine eligible evidence alone may recover it.
    unavailable: Option<FixtureFrequencyError>,
    /// First Stop/Generation, independent of the phase owner's first reason.
    retired: Option<FixtureFrequencyError>,
    /// Actual Unknown is independently sticky even after another retirement.
    unknown: bool,
}

impl FrequencyState {
    /// Construct the immutable legacy opt-out without new evidence or policy.
    pub(super) const fn disabled() -> Self {
        Self {
            configuration: None,
            model: None,
            consumed: None,
            admission: FixtureFrequencyAdmission::Disabled,
            unavailable: None,
            retired: None,
            unknown: false,
        }
    }

    /// Bind an already checked profile to the actual chart and pending cursor.
    /// # Errors
    /// Reject actual association/cursor mismatch without taking any custody.
    pub(super) fn enabled(
        profile: FixtureFrequencyProfile,
        association: FixtureFrequencyClockAssociation,
        chart: &FixtureAffineChart,
        cursor: &FixtureRateSettlementCursor,
    ) -> Result<Self, FixtureFrequencyError> {
        checked_relationship(Relationship {
            association,
            chart,
            context: cursor.context,
            source: None,
        })?;
        let frames = chart.frames();
        if cursor.status() != FixtureRateCursorStatus::Pending
            || cursor.target_frame != frames.target_frame()
            || cursor.target_generation != frames.target_generation()
        {
            return Err(FixtureFrequencyError::CursorUnavailable);
        }
        Ok(Self {
            configuration: Some(Configuration {
                profile,
                association,
            }),
            admission: FixtureFrequencyAdmission::NoReady {
                emitted_round: None,
            },
            ..Self::disabled()
        })
    }

    /// Let the parent avoid advancing current-time facts on a Disabled query.
    pub(super) const fn is_enabled(&self) -> bool {
        self.configuration.is_some()
    }
    /// Copy the actual last source outcome, without current-quality inference.
    pub(super) const fn admission(&self) -> FixtureFrequencyAdmission {
        self.admission
    }
    /// Copy immutable historical data even when projection is unavailable.
    pub(super) fn history(&self) -> Option<FixtureFrequencyHistory> {
        self.model.map(|model| FixtureFrequencyHistory { model })
    }

    /// Consume actual emitted credit before any dependent fallible processing.
    /// # Errors
    /// Reject nonincreasing actual rounds without treating them as fresh credit.
    fn consume(&mut self, round: NonZeroU64) -> Result<(), FixtureFrequencyError> {
        if self.consumed.is_some_and(|previous| round <= previous) {
            return Err(FixtureFrequencyError::ValidationInvariant);
        }
        self.consumed = Some(round);
        Ok(())
    }

    /// Retain a committed source/readiness refusal without inventing a model.
    pub(super) fn no_ready(&mut self, round: Option<NonZeroU64>) {
        if !self.is_enabled() {
            return;
        }
        if let Some(round) = round
            && let Err(reason) = self.consume(round)
        {
            self.admission = FixtureFrequencyAdmission::Refused {
                emitted_round: round,
                reason,
            };
            return;
        }
        self.admission = FixtureFrequencyAdmission::NoReady {
            emitted_round: round,
        };
    }

    /// Assimilate only the actual estimate borrowed by the parent's real token.
    /// Source/phase commit has already happened; refusal retains the old record.
    pub(super) fn assimilate(
        &mut self,
        estimate: &FixtureChartCombinedEstimate<'_, '_, '_>,
        round: NonZeroU64,
        context: FixtureRateClockContext,
        now: MonotonicNs,
    ) {
        let Some(configuration) = self.configuration else {
            return;
        };
        let result = self.consume(round).and_then(|()| {
            self.blocked()?;
            checked_relationship(Relationship {
                association: configuration.association,
                chart: estimate.chart(),
                context,
                source: Some(estimate.source_context()),
            })?;
            query_domain(estimate.chart().domain(), estimate.projected_at(), now)?;
            if estimate.tracking_skew_seconds_per_second() > configuration.profile.max_skew {
                return Err(FixtureFrequencyError::MeasuredSkew);
            }
            self.refresh_previous(estimate.chart(), now, configuration.profile);
            self.prepare_model(estimate, round, now, configuration.profile)
        });
        match result {
            Ok(model) => {
                self.model = Some(model);
                self.unavailable = None;
                self.admission = FixtureFrequencyAdmission::Accepted {
                    emitted_round: round,
                };
            }
            Err(reason) => {
                self.admission = FixtureFrequencyAdmission::Refused {
                    emitted_round: round,
                    reason,
                }
            }
        }
    }

    /// Observe support for the saved old model before deciding update versus recovery.
    fn refresh_previous(
        &mut self,
        chart: &FixtureAffineChart,
        now: MonotonicNs,
        profile: FixtureFrequencyProfile,
    ) {
        if self.unavailable.is_some() {
            return;
        }
        if let Some(model) = self.model
            && let Err(error) = predict(&model, profile, chart, now)
        {
            self.unavailable = Some(match error {
                FixtureFrequencyError::QueryBeforeReference
                | FixtureFrequencyError::QueryOutsideDomain => {
                    FixtureFrequencyError::AnchorRebaseUnavailable
                }
                other => other,
            });
        }
    }

    /// Stage a whole fresh record and its current support without publishing fields.
    /// # Errors
    /// Preserve old model on unsupported initialization/update/anchor/support.
    fn prepare_model(
        &self,
        estimate: &FixtureChartCombinedEstimate<'_, '_, '_>,
        round: NonZeroU64,
        now: MonotonicNs,
        profile: FixtureFrequencyProfile,
    ) -> Result<Model, FixtureFrequencyError> {
        let previous = match self.unavailable {
            None => self.model,
            Some(
                FixtureFrequencyError::Expired
                | FixtureFrequencyError::AnchorRebaseUnavailable
                | FixtureFrequencyError::Numeric { .. },
            ) => None,
            Some(error) => return Err(error),
        };
        let rate = estimate.phase_rate_seconds_per_second();
        let error = estimate.rate_mean_roundoff_seconds_per_second();
        let skew = estimate.tracking_skew_seconds_per_second();
        let prior = match previous {
            Some(model) => model.prior.assimilate(rate, error, skew),
            None => FrequencyPrior::initialize(rate, error, skew),
        }
        .map_err(|source| FixtureFrequencyError::Numeric { source })?;
        let anchor = FrequencyAnchor::new(
            estimate.phase_seconds(),
            estimate.phase_mean_roundoff_seconds(),
        )
        .map_err(|source| FixtureFrequencyError::Numeric { source })?;
        let model = Model {
            prior,
            anchor,
            basis: Basis::from_chart(estimate.chart()),
            source: estimate.source_context(),
            reference_frames: estimate.chart().frames(),
            round,
            observed_anchor: estimate.anchor_observed_at(),
            latest: estimate.latest_observed_at(),
            fit: estimate.fit_reference(),
            projected: estimate.projected_at(),
            phase_sd: estimate.phase_standard_deviation_seconds(),
            rate_sd: estimate.rate_standard_deviation_seconds_per_second(),
        };
        predict(&model, profile, estimate.chart(), now)?;
        Ok(model)
    }

    /// Check independent irreversible model blocks before operation/model checks.
    /// # Errors
    /// Preserve first Stop/Generation, then Unknown without changing any history.
    fn blocked(&self) -> Result<(), FixtureFrequencyError> {
        if let Some(error) = self.retired {
            return Err(error);
        }
        if self.unknown {
            return Err(FixtureFrequencyError::Unknown);
        }
        Ok(())
    }

    /// Predict after the parent admits/commits supplied current-time high waters.
    /// # Errors
    /// Reject blocks/pending work/association/model/age/bounds in that order.
    /// Model-only numerical/expiry loss is retained for genuine fresh recovery.
    pub(super) fn project(
        &mut self,
        chart: &FixtureAffineChart,
        context: FixtureRateClockContext,
        now: MonotonicNs,
        outstanding: bool,
    ) -> Result<FixtureFrequencyProjection, FixtureFrequencyError> {
        let configuration = self.configuration.ok_or(FixtureFrequencyError::Disabled)?;
        self.blocked()?;
        if outstanding {
            return Err(FixtureFrequencyError::OutstandingOperation);
        }
        checked_relationship(Relationship {
            association: configuration.association,
            chart,
            context,
            source: self.model.map(|model| model.source),
        })?;
        let model = self.model.ok_or(FixtureFrequencyError::NoModel)?;
        if let Some(error) = self.unavailable {
            return Err(error);
        }
        match predict(&model, configuration.profile, chart, now) {
            Ok((prediction, age)) => Ok(FixtureFrequencyProjection {
                history: FixtureFrequencyHistory { model },
                prediction,
                evaluated: now,
                age,
            }),
            Err(error) => {
                if matches!(
                    error,
                    FixtureFrequencyError::Expired
                        | FixtureFrequencyError::Numeric { .. }
                        | FixtureFrequencyError::AnchorRebaseUnavailable
                ) {
                    self.unavailable = Some(error);
                }
                Err(error)
            }
        }
    }

    /// Latch from every actual reason, even after the phase owner first saw Conflict.
    pub(super) fn retire(&mut self, reason: FixturePhaseRetirementReason) {
        if !self.is_enabled() || self.retired.is_some() {
            return;
        }
        self.retired = match reason {
            FixturePhaseRetirementReason::Stop => Some(FixtureFrequencyError::Stopped),
            FixturePhaseRetirementReason::Generation => Some(FixtureFrequencyError::Generation),
            FixturePhaseRetirementReason::Conflict => None,
        };
    }

    /// Keep actual Unknown independent of another already latched retirement.
    pub(super) fn unknown(&mut self) {
        if self.is_enabled() {
            self.unknown = true;
        }
    }

    /// Obtain command numbers from this owner's actual supported model only.
    /// # Errors
    /// Preserve current quality, basis, clock and numeric refusals without credit.
    pub(super) fn baseline_request(
        &mut self,
        chart: &FixtureAffineChart,
        context: FixtureRateClockContext,
        now: MonotonicNs,
        operands: crate::regression::BaselineOperands,
    ) -> Result<(NonZeroU64, crate::regression::BaselineRequest), FixtureFrequencyError> {
        let _projection = self.project(chart, context, now, false)?;
        let model = self.model.ok_or(FixtureFrequencyError::NoModel)?;
        if !matches!(self.admission, FixtureFrequencyAdmission::Accepted { emitted_round } if emitted_round == model.round)
        {
            return Err(FixtureFrequencyError::NoModel);
        }
        let numbers = model
            .prior
            .baseline_request(operands)
            .map_err(|source| FixtureFrequencyError::Numeric { source })?;
        Ok((model.round, numbers))
    }

    /// Rebase a committed base effect and retain its exact model transform cause.
    /// # Errors
    /// Preserve original model on failure, latch the same cause and permit no refund.
    pub(super) fn rebase_baseline(
        &mut self,
        predecessor: &FixtureAffineChart,
        effect: &FixtureAffineChart,
        replacement: &FixtureAffineChart,
    ) -> Result<(), FixtureFrequencyError> {
        let model = self.model.ok_or(FixtureFrequencyError::NoModel)?;
        match rebase_model(&model, predecessor, Ok(Some(effect)), Ok(Some(replacement))) {
            Ok(next) => {
                self.model = Some(next);
                Ok(())
            }
            Err(error) => {
                self.unavailable = Some(error);
                Err(error)
            }
        }
    }

    /// Re-express only through the parent's actual admitted total-effect descriptor.
    /// Failure preserves the historical whole record and committed delivery facts.
    /// The legacy phase path retains its established bounded unavailable family.
    pub(super) fn rebase(
        &mut self,
        predecessor: &FixtureAffineChart,
        effect: Result<Option<&FixtureAffineChart>, FixtureChartError>,
        replacement: Result<Option<&FixtureAffineChart>, FixtureChartError>,
    ) {
        if !self.is_enabled() {
            return;
        }
        let Some(model) = self.model else {
            return;
        };
        match rebase_model(&model, predecessor, effect, replacement) {
            Ok(model) => self.model = Some(model),
            Err(_) => self.unavailable = Some(FixtureFrequencyError::AnchorRebaseUnavailable),
        }
    }
}

/// Compare actual checked domain geometry without allocating or re-parsing.
fn same_domain(left: FixtureChartDomain, right: FixtureChartDomain) -> bool {
    left.start() == right.start()
        && left.common_reference() == right.common_reference()
        && left.end() == right.end()
}

/// Use the checked domain's inclusive endpoints at this actual consumer.
fn contains(domain: FixtureChartDomain, at: MonotonicNs) -> bool {
    domain.start() <= at && at <= domain.end()
}

/// Require the genuine anchor and current query on one supported monotonic domain.
/// # Errors
/// Reject a query before the reference, then unsupported reference/query domain.
fn query_domain(
    domain: FixtureChartDomain,
    reference: MonotonicNs,
    now: MonotonicNs,
) -> Result<(), FixtureFrequencyError> {
    if now < reference {
        return Err(FixtureFrequencyError::QueryBeforeReference);
    }
    if !contains(domain, reference) || !contains(domain, now) {
        return Err(FixtureFrequencyError::QueryOutsideDomain);
    }
    Ok(())
}

/// Produce current model support from original epochs, never accumulated old growth.
/// # Errors
/// Reject changed basis/domain, exact original-observation age, then numeric/growth.
fn predict(
    model: &Model,
    profile: FixtureFrequencyProfile,
    chart: &FixtureAffineChart,
    now: MonotonicNs,
) -> Result<(FrequencyPrediction, MonotonicDurationNs), FixtureFrequencyError> {
    if !model.basis.matches(chart) {
        return Err(FixtureFrequencyError::AnchorRebaseUnavailable);
    }
    query_domain(chart.domain(), model.projected, now)?;
    let age = now
        .duration_since(model.latest)
        .map_err(|_| FixtureFrequencyError::QueryBeforeReference)?;
    if age.nanoseconds() >= profile.max_age.nanoseconds() {
        return Err(FixtureFrequencyError::Expired);
    }
    let elapsed = now
        .duration_since(model.projected)
        .map_err(|_| FixtureFrequencyError::QueryBeforeReference)?;
    let result = model
        .prior
        .project(&model.anchor, elapsed, profile.clock_error)
        .map_err(|source| FixtureFrequencyError::Numeric { source })?;
    if result.growth_upper() >= profile.growth_cap {
        return Err(FixtureFrequencyError::Expired);
    }
    Ok((result, age))
}

/// Stage a checked affine mean/anchor re-expression without fresh evidence.
/// # Errors
/// Preserve the whole model when complete composition/domain/numeric checks fail.
fn rebase_model(
    model: &Model,
    predecessor: &FixtureAffineChart,
    effect: Result<Option<&FixtureAffineChart>, FixtureChartError>,
    replacement: Result<Option<&FixtureAffineChart>, FixtureChartError>,
) -> Result<Model, FixtureFrequencyError> {
    let (Ok(Some(effect)), Ok(Some(replacement))) = (effect, replacement) else {
        return Err(FixtureFrequencyError::AnchorRebaseUnavailable);
    };
    let old = predecessor.frames();
    let step = effect.frames();
    let new = replacement.frames();
    if !model.basis.matches(predecessor)
        || step.source_frame() != old.target_frame()
        || step.source_generation() != old.target_generation()
        || new.source_frame() != old.source_frame()
        || new.source_generation() != old.source_generation()
        || new.target_frame() != step.target_frame()
        || new.target_generation() != step.target_generation()
        || !contains(replacement.domain(), model.projected)
    {
        return Err(FixtureFrequencyError::AnchorRebaseUnavailable);
    }
    // Recompute the exact checked composition, including all coefficient errors.
    // Equal endpoint tags alone do not establish a transformed model's basis.
    let expected = predecessor
        .compose_intersection(effect)
        .map_err(|_| FixtureFrequencyError::AnchorRebaseUnavailable)?;
    if !Basis::from_chart(&expected).matches(replacement) {
        return Err(FixtureFrequencyError::AnchorRebaseUnavailable);
    }
    let anchor = model
        .anchor
        .rebase_affine(effect, model.projected)
        .map_err(|source| FixtureFrequencyError::Numeric { source })?;
    let prior = model
        .prior
        .shift_rate(
            effect.rate_seconds_per_second(),
            effect.rate_coefficient_roundoff_seconds_per_second(),
        )
        .map_err(|source| FixtureFrequencyError::Numeric { source })?;

    Ok(Model {
        prior,
        anchor,
        basis: Basis::from_chart(replacement),
        ..*model
    })
}

#[cfg(test)]
#[path = "../../../tests/private/frequency_prior_state.rs"]
mod tests;

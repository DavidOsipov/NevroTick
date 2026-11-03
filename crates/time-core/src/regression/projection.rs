// SPDX-License-Identifier: GPL-2.0-only
/*
chronyd/chronyc - Programs for keeping computer clocks accurate.

Original sourcestats.c notice:
Copyright (C) Richard P. Curnow  1997-2003
Copyright (C) Miroslav Lichvar  2011-2014, 2016-2018, 2021

This program is free software; you can redistribute it and/or modify
it under the terms of version 2 of the GNU General Public License as
published by the Free Software Foundation.

This program is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public License along
with this program; if not, write to the Free Software Foundation, Inc.,
51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
*/
/*
chronyd/chronyc - Programs for keeping computer clocks accurate.

Original sources.c notice:
Copyright (C) Richard P. Curnow  1997-2003
Copyright (C) Miroslav Lichvar  2011-2016, 2018, 2020-2025

This program is free software; you can redistribute it and/or modify
it under the terms of version 2 of the GNU General Public License as
published by the Free Software Foundation.

This program is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public License along
with this program; if not, write to the Free Software Foundation, Inc.,
51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
*/
//! Pure fixture-anchor projection from one borrowed committed history.
//!
//! Modified by David Osipov with AI assistance, 3 November 2026: the adaptive
//! fitted-anchor adapter shares the original checked projection arithmetic.
//! S152 adds same-owner fitted tracking under the retained Chrony 4.9
//! `sourcestats.c` 709–714 and `sources.c` 922–924 contract. Tracking remains
//! separate from the original raw-anchor center and observation authority.
//!
//! The history alone owns the held observation, latest instant and fitted rate.
//! Asserted context and exact anchor time are checked before any arithmetic. Age
//! remains an exact monotonic duration; only its checked difference is converted
//! to positive seconds. Phase reuses the exact Q33 conversion owner. Every
//! binary64 primitive is separately evaluated and checked before another can hide
//! a rejected intermediate. No fused multiply-add or fitted-intercept substitute
//! is used for the anchor center. Supplied distance terms are fixture assertions, never confidence,
//! authentication, freshness policy, integer enclosure or correction authority.
//!
//! Work and scalar scratch are constant. Projection borrows rather than copies
//! history and performs no fit, normalization, append, allocation of owned input,
//! ambient read, I/O or state change. Terms admission uses a temporary garde
//! report over exactly three fixed fields; only bounded typed codes escape.

use std::fmt;

use garde::Validate;
use time_domain::{DomainError, MonotonicDurationNs, MonotonicNs};

use super::{
    ArithmeticOperation, ArithmeticStage, FitError, FixtureAdaptiveHistory, FixtureContext,
    FixtureHistory, FixtureLine, FixtureObservation, FixtureResidualStatistics,
    FixtureStandardError, NormalizationComponent, NormalizationError, NormalizationStep,
    arithmetic,
    normalization::{integer_to_f64, normalization_arithmetic, phase_seconds},
};

/// Supplied term role in the constructor's declared first-error order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureProjectionField {
    /// Asserted nonnegative half-delay in seconds.
    HalfDelaySeconds,
    /// Asserted nonnegative dispersion in seconds.
    DispersionSeconds,
    /// Asserted nonnegative skew in seconds per second.
    SkewSecondsPerSecond,
}

/// First violated term invariant; both signed zeros are supported.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureProjectionIssue {
    /// Raw value is NaN or either infinity.
    NonFinite,
    /// Raw nonzero value is subnormal, checked before its sign.
    Subnormal,
    /// Finite normal value is negative.
    Negative,
}

/// One separate ordinary binary64 primitive in projection order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureProjectionStep {
    /// Multiply the same-history fitted rate by positive anchor age.
    RateShift,
    /// Add the actual anchor phase to the checked rate shift.
    Center,
    /// Add the supplied half-delay to supplied dispersion.
    BaseDistance,
    /// Multiply positive anchor age by supplied skew.
    Growth,
    /// Add the checked base distance to checked growth.
    Distance,
}

impl FixtureProjectionStep {
    /// Select the step's sole primitive without arithmetic, mutation or effects.
    const fn operation(self) -> ArithmeticOperation {
        match self {
            Self::RateShift | Self::Growth => ArithmeticOperation::Multiply,
            Self::Center | Self::BaseDistance | Self::Distance => ArithmeticOperation::Add,
        }
    }
}

/// Closed projection rejection with no numeric payload or retained garde report.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureProjectionError {
    /// First invalid term in half-delay, dispersion, skew constructor order.
    InvalidTerm {
        /// Closed supplied term role, never a caller-selected path.
        field: FixtureProjectionField,
        /// First shared finite/subnormal/sign failure for that field.
        issue: FixtureProjectionIssue,
    },
    /// Mandatory garde disagreed with every shared scalar predicate.
    ValidationInvariant,
    /// Expected complete context differs from the history; checked first.
    ContextMismatch,
    /// The history has no committed fit report; checked before anchor lookup.
    MissingReport,
    /// The requested anchor index is not in the actual retained prefix.
    MissingAnchor,
    /// The held row at the index has a different exact expected anchor time.
    AnchorMismatch,
    /// Projection is earlier than the latest accepted observation.
    BackwardProjection,
    /// A checked exact same-domain duration operation failed without effects.
    Domain {
        /// Actual bounded time-domain cause, with no raw reading retained.
        source: DomainError,
    },
    /// Existing exact integer or phase conversion rejected its actual input.
    Conversion {
        /// Actual checked-conversion owner cause, preserved without relabelling.
        source: NormalizationError,
    },
    /// One separate projection primitive rejected its operands or result.
    Arithmetic {
        /// Actual primitive's closed projection role.
        step: FixtureProjectionStep,
        /// Actual shared arithmetic cause, without raw operands or output.
        source: FitError,
    },
}

impl fmt::Display for FixtureProjectionError {
    /// Format fixed codes and bounded typed causes for every public construction.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTerm { field, issue } => {
                write!(formatter, "fixture projection term {field:?}: {issue:?}")
            }
            Self::ValidationInvariant => formatter.write_str("projection validation invariant"),
            Self::ContextMismatch => formatter.write_str("projection context mismatch"),
            Self::MissingReport => formatter.write_str("projection missing report"),
            Self::MissingAnchor => formatter.write_str("projection missing anchor"),
            Self::AnchorMismatch => formatter.write_str("projection anchor time mismatch"),
            Self::BackwardProjection => formatter.write_str("projection before latest observation"),
            Self::Domain { source } => write!(formatter, "projection duration: {source}"),
            Self::Conversion { source } => write!(formatter, "projection conversion: {source}"),
            Self::Arithmetic { step, source } => {
                write!(formatter, "projection arithmetic {step:?}: {source}")
            }
        }
    }
}

impl std::error::Error for FixtureProjectionError {
    /// Borrow actual closed typed causes without allocation, parsing or effects.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Domain { source } => Some(source),
            Self::Conversion { source } => Some(source),
            Self::Arithmetic { source, .. } => Some(source),
            Self::InvalidTerm { .. }
            | Self::ValidationInvariant
            | Self::ContextMismatch
            | Self::MissingReport
            | Self::MissingAnchor
            | Self::AnchorMismatch
            | Self::BackwardProjection => None,
        }
    }
}

/// Scalar role in the tracking payload's declared final admission order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureTrackingField {
    /// Signed fitted phase at the common evaluation instant, in seconds.
    Phase,
    /// Nonnegative fitted phase standard deviation at that instant, in seconds.
    PhaseStandardDeviation,
    /// Nonnegative same-fit rate standard deviation, in seconds per second.
    RateStandardDeviation,
    /// Separately asserted nonnegative tracking skew, in seconds per second.
    TrackingSkew,
}

/// One separate checked primitive in the fitted tracking transformation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureTrackingStep {
    /// Multiply the same fitted rate by the fit-reference age.
    RateShift,
    /// Add the checked shift to the same fitted intercept.
    Phase,
    /// Multiply the same fitted rate SD by the fit-reference age.
    StandardDeviationGrowth,
    /// Add the checked growth to the fitted intercept SD.
    StandardDeviation,
}

impl FixtureTrackingStep {
    /// Select the sole actual operation for a closed step, without arithmetic.
    const fn operation(self) -> ArithmeticOperation {
        match self {
            Self::RateShift | Self::StandardDeviationGrowth => ArithmeticOperation::Multiply,
            Self::Phase | Self::StandardDeviation => ArithmeticOperation::Add,
        }
    }
}

/// Additive bounded tracking rejection, preserving the actual nested cause.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureTrackingProjectionError {
    /// Existing anchor projection, duration or exact conversion failed.
    Projection {
        /// Original projection cause, including its time-domain or conversion source.
        source: FixtureProjectionError,
    },
    /// The same selected report rejected its standard-error calculation.
    StandardErrors {
        /// Actual same-fit diagnostic failure, without an independently supplied report.
        source: FixtureStandardError,
    },
    /// Shared supported-domain or field-specific sign admission rejected a scalar.
    InvalidScalar {
        /// Actual scalar role; signed phase has no nonnegative restriction.
        field: FixtureTrackingField,
        /// First finite/subnormal/sign failure, without raw numeric payload.
        issue: FixtureProjectionIssue,
    },
    /// A separately evaluated fitted transformation failed before publication.
    Arithmetic {
        /// Closed role of the actual multiply or add operation.
        step: FixtureTrackingStep,
        /// Actual Response-stage shared arithmetic failure.
        source: FitError,
    },
    /// Mandatory admission disagreed with its maintained scalar predicate.
    ValidationInvariant,
}

impl fmt::Display for FixtureTrackingProjectionError {
    /// Emit fixed bounded codes and typed fields, never numeric values or reports.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Projection { .. } => formatter.write_str("tracking projection failure"),
            Self::StandardErrors { .. } => formatter.write_str("tracking standard-error failure"),
            Self::InvalidScalar { field, issue } => {
                write!(formatter, "tracking scalar {field:?}: {issue:?}")
            }
            Self::Arithmetic { step, .. } => write!(formatter, "tracking arithmetic {step:?}"),
            Self::ValidationInvariant => formatter.write_str("tracking validation invariant"),
        }
    }
}

impl std::error::Error for FixtureTrackingProjectionError {
    /// Borrow actual typed causes without allocation, relabelling or effects.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Projection { source } => Some(source),
            Self::StandardErrors { source } => Some(source),
            Self::Arithmetic { source, .. } => Some(source),
            Self::InvalidScalar { .. } | Self::ValidationInvariant => None,
        }
    }
}

/// Sole raw-term predicate; accepts both zero signs and preserves all input bits.
///
/// Constant-work checks admit finite normal nonnegative values or either zero,
/// with `NonFinite` then `Subnormal` then `Negative` precedence and no effects.
fn term_issue(value: f64) -> Option<FixtureProjectionIssue> {
    if !value.is_finite() {
        Some(FixtureProjectionIssue::NonFinite)
    } else if value.is_subnormal() {
        Some(FixtureProjectionIssue::Subnormal)
    } else if value < 0.0 {
        Some(FixtureProjectionIssue::Negative)
    } else {
        None
    }
}

/// Adapt the sole raw-term predicate to garde without formatting rejected input.
/// # Errors
/// Returns one fixed redacted failure for an unsupported term; no state changes.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rules borrow their raw field and context"
)]
fn term_rule(value: &f64, (): &()) -> garde::Result {
    if term_issue(*value).is_none() {
        Ok(())
    } else {
        Err(garde::Error::new("invalid fixture projection term"))
    }
}

/// Classify any raw tracking scalar, preserving the shared support precedence.
///
/// Phase alone is signed. Its negative normal branch remains admitted; no input
/// bits are changed, and SD/skew keep the same nonnegative term rule.
fn tracking_issue(value: f64, field: FixtureTrackingField) -> Option<FixtureProjectionIssue> {
    match term_issue(value) {
        Some(FixtureProjectionIssue::Negative) if field == FixtureTrackingField::Phase => None,
        other => other,
    }
}

/// Adapt the shared signed-phase predicate to mandatory borrowed Garde admission.
/// # Errors
/// Reject only unsupported raw phase with fixed text and no state change.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rules borrow their raw field and context"
)]
fn tracking_phase_rule(value: &f64, (): &()) -> garde::Result {
    if tracking_issue(*value, FixtureTrackingField::Phase).is_none() {
        Ok(())
    } else {
        Err(garde::Error::new("invalid fixture tracking phase"))
    }
}

/// One untrusted supplied tracking skew, separate from checked projection terms.
#[derive(Validate)]
struct TrackingSkewCandidate {
    /// Both signed zeros are legal; the shared rule rejects unsupported negatives.
    #[garde(custom(term_rule))]
    value: f64,
}

/// Mandatorily admit one supplied skew without publishing a tracking association.
/// # Errors
/// Preserve the shared scalar cause, or a Garde/predicate disagreement.
fn admit_tracking_skew(value: f64) -> Result<f64, FixtureTrackingProjectionError> {
    let raw = TrackingSkewCandidate { value };
    let validated = raw.validate();
    if let Some(issue) = term_issue(value) {
        return Err(FixtureTrackingProjectionError::InvalidScalar {
            field: FixtureTrackingField::TrackingSkew,
            issue,
        });
    }
    validated.map_err(|_| FixtureTrackingProjectionError::ValidationInvariant)?;
    Ok(value)
}

/// Four unpublished transformed scalars, with no report or projection authority.
#[derive(Validate)]
struct TrackingCandidate {
    /// Signed common-time phase retains the shared finite normal-or-zero domain.
    #[garde(custom(tracking_phase_rule))]
    phase: f64,
    /// Common-time phase SD retains the shared nonnegative supported domain.
    #[garde(custom(term_rule))]
    phase_sd: f64,
    /// Same-fit rate SD retains the shared nonnegative supported domain.
    #[garde(custom(term_rule))]
    rate_sd: f64,
    /// Supplied tracking skew retains the same nonnegative supported domain.
    #[garde(custom(term_rule))]
    skew: f64,
}

impl TrackingCandidate {
    /// Admit all four raw scalars without minting or attaching any public result.
    /// # Errors
    /// Return the first shared phase/phase-SD/rate-SD/skew cause or Garde disagreement.
    fn admit(&self) -> Result<(), FixtureTrackingProjectionError> {
        let validated = self.validate();
        for (value, field) in [
            (self.phase, FixtureTrackingField::Phase),
            (self.phase_sd, FixtureTrackingField::PhaseStandardDeviation),
            (self.rate_sd, FixtureTrackingField::RateStandardDeviation),
            (self.skew, FixtureTrackingField::TrackingSkew),
        ] {
            if let Some(issue) = tracking_issue(value, field) {
                return Err(FixtureTrackingProjectionError::InvalidScalar { field, issue });
            }
        }
        validated.map_err(|_| FixtureTrackingProjectionError::ValidationInvariant)
    }
}

/// Execute one unfused tracking primitive under its fixed operation and stage.
/// # Errors
/// Preserve the actual checked arithmetic cause without publishing partial data.
fn tracking_arithmetic(
    left: f64,
    right: f64,
    step: FixtureTrackingStep,
) -> Result<f64, FixtureTrackingProjectionError> {
    arithmetic(left, right, step.operation(), ArithmeticStage::Response)
        .map_err(|source| FixtureTrackingProjectionError::Arithmetic { step, source })
}

/// Three raw terms, each mandatorily covered by the same scalar rule owner.
#[derive(Validate)]
struct ProjectionCandidate {
    /// Raw asserted half-delay seconds, admitted before publication.
    #[garde(custom(term_rule))]
    half_delay_seconds: f64,
    /// Raw asserted dispersion seconds, admitted before publication.
    #[garde(custom(term_rule))]
    dispersion_seconds: f64,
    /// Raw asserted skew seconds per second, admitted before publication.
    #[garde(custom(term_rule))]
    skew_seconds_per_second: f64,
}

impl ProjectionCandidate {
    /// Project the first shared scalar failure in constructor order, without effects.
    fn first_error(&self) -> Option<FixtureProjectionError> {
        for (value, field) in [
            (
                self.half_delay_seconds,
                FixtureProjectionField::HalfDelaySeconds,
            ),
            (
                self.dispersion_seconds,
                FixtureProjectionField::DispersionSeconds,
            ),
            (
                self.skew_seconds_per_second,
                FixtureProjectionField::SkewSecondsPerSecond,
            ),
        ] {
            if let Some(issue) = term_issue(value) {
                return Some(FixtureProjectionError::InvalidTerm { field, issue });
            }
        }
        None
    }
}

/// Immutable checked fixture assertions; supplied signed-zero bits are retained.
///
/// No default, mutable view, builder or deserialization bypass is provided.
/// ```compile_fail,E0451
/// use time_core::FixtureProjectionTerms;
/// let _ = FixtureProjectionTerms {
///     half_delay_seconds: 0.0, dispersion_seconds: 0.0, skew_seconds_per_second: 0.0,
/// };
/// ```
/// ```
/// # use time_core::{FixtureProjectionError, FixtureProjectionTerms};
/// # fn main() -> Result<(), FixtureProjectionError> {
/// let terms = FixtureProjectionTerms::new(-0.0, 0.125, 0.0)?;
/// assert_eq!(terms.half_delay_seconds().to_bits(), (-0.0_f64).to_bits());
/// assert_eq!(terms.dispersion_seconds(), 0.125);
/// # Ok(()) }
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct FixtureProjectionTerms {
    /// Garde-admitted half-delay seconds, retaining the original bits.
    half_delay_seconds: f64,
    /// Garde-admitted dispersion seconds, retaining the original bits.
    dispersion_seconds: f64,
    /// Garde-admitted skew seconds per second, retaining the original bits.
    skew_seconds_per_second: f64,
}

impl fmt::Debug for FixtureProjectionTerms {
    /// Reveal only the type, omitting supplied numeric values and their signs.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureProjectionTerms { .. }")
    }
}

impl FixtureProjectionTerms {
    /// Mandatorily validate all three fields before immutable private construction.
    ///
    /// Both signed zeros are accepted unchanged. Validation's temporary report
    /// is bounded to three fixed fields and discarded without formatting. Work
    /// is constant; rejection has no output value, mutation or external effect.
    /// # Errors
    /// Rejects nonfinite, subnormal or negative terms in half-delay, dispersion,
    /// skew field order; otherwise unexplained garde failure is an invariant error.
    pub fn new(
        half_delay_seconds: f64,
        dispersion_seconds: f64,
        skew_seconds_per_second: f64,
    ) -> Result<Self, FixtureProjectionError> {
        let candidate = ProjectionCandidate {
            half_delay_seconds,
            dispersion_seconds,
            skew_seconds_per_second,
        };
        let validation = candidate.validate();
        if let Some(error) = candidate.first_error() {
            return Err(error);
        }
        validation.map_err(|_| FixtureProjectionError::ValidationInvariant)?;
        Ok(Self {
            half_delay_seconds,
            dispersion_seconds,
            skew_seconds_per_second,
        })
    }

    /// Return unchanged supplied half-delay seconds without effects or policy.
    #[must_use]
    pub const fn half_delay_seconds(&self) -> f64 {
        self.half_delay_seconds
    }

    /// Return unchanged supplied dispersion seconds without effects or policy.
    #[must_use]
    pub const fn dispersion_seconds(&self) -> f64 {
        self.dispersion_seconds
    }

    /// Return unchanged supplied skew seconds per second without effects or policy.
    #[must_use]
    pub const fn skew_seconds_per_second(&self) -> f64 {
        self.skew_seconds_per_second
    }
}

/// Opaque fitted tracking values associated only by their enclosing projection.
///
/// No public constructor, mutation or reattachment path exists. Context, fitted
/// rate and all instants remain owned by the original enclosing projection;
/// copying these four values cannot bind them to another entry or grant credit.
#[derive(Clone, Copy)]
pub struct FixtureTrackingEstimate {
    /// Checked signed fitted phase at the enclosing projection's evaluation time.
    phase_seconds: f64,
    /// Checked nonnegative same-fit phase SD aged to that same instant.
    phase_standard_deviation_seconds: f64,
    /// Checked nonnegative same-fit rate SD, unchanged by projection.
    rate_standard_deviation_seconds_per_second: f64,
    /// Separately checked supplied tracking skew, not projection growth skew.
    tracking_skew_seconds_per_second: f64,
}

impl fmt::Debug for FixtureTrackingEstimate {
    /// Reveal only the type without numeric payloads or contextual identifiers.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureTrackingEstimate { .. }")
    }
}

impl FixtureTrackingEstimate {
    /// Copy the same-fit phase at the enclosing evaluation instant without effects.
    #[must_use]
    pub const fn phase_seconds(&self) -> f64 {
        self.phase_seconds
    }

    /// Copy the checked same-fit phase SD at that instant without recomputation.
    #[must_use]
    pub const fn phase_standard_deviation_seconds(&self) -> f64 {
        self.phase_standard_deviation_seconds
    }

    /// Copy the original same-fit rate SD, retaining supported signed-zero bits.
    #[must_use]
    pub const fn rate_standard_deviation_seconds_per_second(&self) -> f64 {
        self.rate_standard_deviation_seconds_per_second
    }

    /// Copy the independent supplied tracking skew without calibration or effects.
    #[must_use]
    pub const fn tracking_skew_seconds_per_second(&self) -> f64 {
        self.tracking_skew_seconds_per_second
    }
}

/// Immutable result from one actual held anchor and its same-history fitted rate.
///
/// Observation and fit instants remain distinct from projection time; repeated
/// evaluation creates no fresh observation. The numeric distance is conditional
/// fixture arithmetic, without physical confidence or integer interval admission.
/// Only the checked producer below may construct it; no mutation is exposed.
/// ```compile_fail,E0616
/// use time_core::FixtureAnchorProjection;
/// fn alter(mut projection: FixtureAnchorProjection) {
///     projection.center_seconds = 0.0;
/// }
/// ```
#[derive(Clone, Copy)]
pub struct FixtureAnchorProjection {
    /// Complete asserted same-history labels; these grant no trust.
    context: FixtureContext,
    /// Exact actual retained anchor time, checked against the expected identity.
    anchor_observed_at: MonotonicNs,
    /// Exact latest accepted observation time, never refreshed by projection.
    latest_observed_at: MonotonicNs,
    /// Reference used by the history's actual committed fit.
    fit_reference: MonotonicNs,
    /// Explicit caller-supplied evaluation instant, at or after latest observation.
    projected_at: MonotonicNs,
    /// Exact same-domain difference from actual anchor to projection.
    anchor_elapsed: MonotonicDurationNs,
    /// Exact same-domain difference from latest observation to projection.
    latest_elapsed: MonotonicDurationNs,
    /// Checked positive anchor age seconds, with positive zero at zero duration.
    anchor_elapsed_seconds: f64,
    /// Checked actual anchor phase plus same-history rate times anchor age.
    center_seconds: f64,
    /// Exact supported same-history fitted phase derivative, in seconds per second.
    /// The immutable report's checked line owns its normal-or-zero invariant.
    phase_rate_seconds_per_second: f64,
    /// Original immutable supplied terms, including supplied signed-zero bits.
    terms: FixtureProjectionTerms,
    /// Checked separate anchor age times supplied skew, in seconds.
    growth_seconds: f64,
    /// Checked half-delay plus dispersion, then growth, in seconds.
    distance_seconds: f64,
    /// Optional same-owner fitted tracking; legacy producers leave it absent.
    tracking: Option<FixtureTrackingEstimate>,
}

impl fmt::Debug for FixtureAnchorProjection {
    /// Reveal only the type, without context, instants, durations or numeric data.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureAnchorProjection { .. }")
    }
}

impl FixtureAnchorProjection {
    /// Return unchanged complete asserted context without trust or effects.
    #[must_use]
    pub const fn context(&self) -> FixtureContext {
        self.context
    }

    /// Return actual retained anchor time without manufacturing an observation.
    #[must_use]
    pub const fn anchor_observed_at(&self) -> MonotonicNs {
        self.anchor_observed_at
    }

    /// Return latest accepted time without refreshing it or reading ambient time.
    #[must_use]
    pub const fn latest_observed_at(&self) -> MonotonicNs {
        self.latest_observed_at
    }

    /// Return the same committed fit's reference without fitting or effects.
    #[must_use]
    pub const fn fit_reference(&self) -> MonotonicNs {
        self.fit_reference
    }

    /// Return the supplied projection instant without observation or policy effects.
    #[must_use]
    pub const fn projected_at(&self) -> MonotonicNs {
        self.projected_at
    }

    /// Return the exact checked anchor age without conversion or effects.
    #[must_use]
    pub const fn anchor_elapsed(&self) -> MonotonicDurationNs {
        self.anchor_elapsed
    }

    /// Return the exact checked latest-observation age without effects.
    #[must_use]
    pub const fn latest_elapsed(&self) -> MonotonicDurationNs {
        self.latest_elapsed
    }

    /// Return checked nonnegative anchor age seconds without changing zero bits.
    #[must_use]
    pub const fn anchor_elapsed_seconds(&self) -> f64 {
        self.anchor_elapsed_seconds
    }

    /// Return checked positive-behind projected fixture phase seconds unchanged.
    #[must_use]
    pub const fn center_seconds(&self) -> f64 {
        self.center_seconds
    }

    /// Return the same fitted positive-behind phase derivative used for projection.
    ///
    /// This is seconds per monotonic second, separate from supplied skew or any
    /// clock-correction request. The owned bits stay fixed across later appends.
    #[must_use]
    pub const fn phase_rate_seconds_per_second(&self) -> f64 {
        self.phase_rate_seconds_per_second
    }

    /// Return the original immutable asserted terms without recomputation/effects.
    #[must_use]
    pub const fn terms(&self) -> FixtureProjectionTerms {
        self.terms
    }

    /// Return separate checked age-times-skew growth seconds without effects.
    #[must_use]
    pub const fn growth_seconds(&self) -> f64 {
        self.growth_seconds
    }

    /// Return checked fixture distance seconds without physical confidence claims.
    #[must_use]
    pub const fn distance_seconds(&self) -> f64 {
        self.distance_seconds
    }

    /// Borrow same-owner fitted tracking without supplying a rebinding operation.
    ///
    /// Absence identifies a legacy anchor-only projection, not zero uncertainty.
    #[must_use]
    pub const fn tracking(&self) -> Option<&FixtureTrackingEstimate> {
        self.tracking.as_ref()
    }
}

/// Convert only an exact checked elapsed difference, never an absolute reading.
///
/// The existing integer owner enforces inclusive `2^53` nanoseconds and exact
/// reconstruction; one separately guarded division by `1e9` gives nonnegative
/// seconds. Integer zero remains positive zero. Constant work has no effects.
/// # Errors
/// Retains the existing integer/conversion/division cause before any publication.
fn elapsed_seconds(elapsed: MonotonicDurationNs) -> Result<f64, FixtureProjectionError> {
    let exact = integer_to_f64(elapsed.nanoseconds(), NormalizationComponent::Time)
        .map_err(|source| FixtureProjectionError::Conversion { source })?;
    normalization_arithmetic(
        exact,
        1_000_000_000.0,
        NormalizationComponent::Time,
        NormalizationStep::SecondsDivide,
    )
    .map_err(|source| FixtureProjectionError::Conversion { source })
}

/// Run one step's sole primitive through the shared raw arithmetic guard owner.
///
/// Every raw scalar pair is independently checked. Legal zero signs and ordinary
/// cancellation are preserved; no clamp, fusion, reassociation or effect occurs.
/// # Errors
/// Retains unsupported operands/results and nonzero-to-zero underflow with the
/// exact step and actual Response-stage shared arithmetic cause.
fn projection_arithmetic(
    left: f64,
    right: f64,
    step: FixtureProjectionStep,
) -> Result<f64, FixtureProjectionError> {
    arithmetic(left, right, step.operation(), ArithmeticStage::Response)
        .map_err(|source| FixtureProjectionError::Arithmetic { step, source })
}

/// One borrowed, checked anchor/report association from a single history owner.
///
/// Only the two whole-history constructors below assemble this capsule. Their
/// current-context and exact-anchor guards precede publication; each borrows its
/// anchor and line from the same immutable owner. The line's private checked fit
/// construction owns its finite normal-or-zero rate. A shared lifetime alone is
/// not an association proof. No independent report/line/raw-parts factory exists.
struct ProjectionSource<'a> {
    /// Checked complete context copied from the requested same-owner identity.
    context: FixtureContext,
    /// Actual fitted anchor; adaptive auxiliary and padding rows are excluded.
    anchor: &'a FixtureObservation,
    /// Actual anchor time read and checked by the same-owner constructor.
    anchor_observed_at: MonotonicNs,
    /// Actual latest accepted row time, retained without refreshing evidence.
    latest_observed_at: MonotonicNs,
    /// Exact reference from this same owner's committed report.
    fit_reference: MonotonicNs,
    /// This same report's checked terminal line; never an independently supplied fit.
    line: &'a FixtureLine,
    /// Exact same report that owns the borrowed line, never a separately supplied fit.
    statistics: &'a FixtureResidualStatistics,
}

impl<'a> ProjectionSource<'a> {
    /// Borrow an exact fixed-history anchor and its own checked report in constant work.
    ///
    /// No arithmetic, fitting, mutation, allocation or ambient effects occur.
    /// The committed history invariant owns the report/prefix association.
    /// # Errors
    /// Rejects context, absent report, absent anchor, then mismatched anchor time;
    /// an absent same-owner reference retains the existing `MissingAnchor` code.
    fn fixed(
        history: &'a FixtureHistory,
        expected_context: FixtureContext,
        anchor_index: usize,
        expected_anchor_at: MonotonicNs,
    ) -> Result<Self, FixtureProjectionError> {
        if history.context() != expected_context {
            return Err(FixtureProjectionError::ContextMismatch);
        }
        let report = history
            .report()
            .ok_or(FixtureProjectionError::MissingReport)?;
        let anchor = history
            .observation(anchor_index)
            .ok_or(FixtureProjectionError::MissingAnchor)?;
        let anchor_observed_at = anchor.observed_at();
        if anchor_observed_at != expected_anchor_at {
            return Err(FixtureProjectionError::AnchorMismatch);
        }
        // A committed report guarantees a nonempty same-prefix history/reference.
        let latest_observed_at = history
            .reference()
            .ok_or(FixtureProjectionError::MissingAnchor)?;
        let statistics = report.statistics();
        Ok(Self {
            context: expected_context,
            anchor,
            anchor_observed_at,
            latest_observed_at,
            fit_reference: report.reference(),
            line: statistics.line(),
            statistics,
        })
    }

    /// Borrow only a current fitted adaptive row and that owner's terminal line.
    ///
    /// `observation` translates a current fitted index inside the checked owner;
    /// selection indices refer to an older combined input and are never reused.
    /// Auxiliary rows and the configured pruning minimum grant no eligibility.
    /// No scan, refit, mutation, copying of history or external effect occurs.
    /// # Errors
    /// Preserves the fixed constructor's context/report/anchor/time/reference
    /// error order, including warmup without a report and out-of-range indices.
    fn adaptive(
        history: &'a FixtureAdaptiveHistory,
        expected_context: FixtureContext,
        anchor_index: usize,
        expected_anchor_at: MonotonicNs,
    ) -> Result<Self, FixtureProjectionError> {
        if history.context() != expected_context {
            return Err(FixtureProjectionError::ContextMismatch);
        }
        let report = history
            .report()
            .ok_or(FixtureProjectionError::MissingReport)?;
        let anchor = history
            .observation(anchor_index)
            .ok_or(FixtureProjectionError::MissingAnchor)?;
        let anchor_observed_at = anchor.observed_at();
        if anchor_observed_at != expected_anchor_at {
            return Err(FixtureProjectionError::AnchorMismatch);
        }
        // A committed report guarantees a nonempty same-operation fitted suffix/reference.
        let latest_observed_at = history
            .reference()
            .ok_or(FixtureProjectionError::MissingAnchor)?;
        let statistics = report.selection().statistics();
        Ok(Self {
            context: expected_context,
            anchor,
            anchor_observed_at,
            latest_observed_at,
            fit_reference: report.reference(),
            line: statistics.line(),
            statistics,
        })
    }

    /// Project this checked association using immutable already-admitted terms.
    ///
    /// Performs two exact duration operations, one elapsed and one phase conversion,
    /// then the original five separate arithmetic primitives in their original order.
    /// Scalar scratch and work are constant. The returned owned result retains the
    /// real anchor/latest/fit instants; no observation or policy is refreshed.
    /// # Errors
    /// Rejects projection before latest time first, then preserves exact duration,
    /// conversion and primitive-specific causes before publishing any result.
    fn project(
        self,
        at: MonotonicNs,
        terms: FixtureProjectionTerms,
    ) -> Result<FixtureAnchorProjection, FixtureProjectionError> {
        let anchor_observed_at = self.anchor_observed_at;
        if at < self.latest_observed_at {
            return Err(FixtureProjectionError::BackwardProjection);
        }
        let anchor_elapsed = at
            .duration_since(anchor_observed_at)
            .map_err(|source| FixtureProjectionError::Domain { source })?;
        let latest_elapsed = at
            .duration_since(self.latest_observed_at)
            .map_err(|source| FixtureProjectionError::Domain { source })?;
        let anchor_elapsed_seconds = elapsed_seconds(anchor_elapsed)?;
        let phase = phase_seconds(self.anchor)
            .map_err(|source| FixtureProjectionError::Conversion { source })?;
        let rate = self.line.phase_rate_seconds_per_second();
        let shift = projection_arithmetic(
            rate,
            anchor_elapsed_seconds,
            FixtureProjectionStep::RateShift,
        )?;
        let center_seconds = projection_arithmetic(phase, shift, FixtureProjectionStep::Center)?;
        let base = projection_arithmetic(
            terms.half_delay_seconds,
            terms.dispersion_seconds,
            FixtureProjectionStep::BaseDistance,
        )?;
        let growth_seconds = projection_arithmetic(
            anchor_elapsed_seconds,
            terms.skew_seconds_per_second,
            FixtureProjectionStep::Growth,
        )?;
        let distance_seconds =
            projection_arithmetic(base, growth_seconds, FixtureProjectionStep::Distance)?;
        Ok(FixtureAnchorProjection {
            context: self.context,
            anchor_observed_at,
            latest_observed_at: self.latest_observed_at,
            fit_reference: self.fit_reference,
            projected_at: at,
            anchor_elapsed,
            latest_elapsed,
            anchor_elapsed_seconds,
            center_seconds,
            phase_rate_seconds_per_second: rate,
            terms,
            growth_seconds,
            distance_seconds,
            tracking: None,
        })
    }
}

/// Project one explicitly identified held fixture anchor using its current fit.
///
/// One shared history borrow freezes its actual context, report, anchor and
/// latest row throughout constant-work preparation. The history report's
/// producer owns the same-prefix rate/reference association; no independent
/// report can be paired here. Exact monotonic subtraction precedes conversion.
/// Anchor phase is Q33 numerator divided by `2^33`, never nanoseconds. Separate
/// checked operations evaluate rate*age, phase+shift, half-delay+dispersion,
/// age*skew, base+growth in that order. Each intermediate must be supported before
/// the next operation. No fitted intercept or residual statistic contributes.
///
/// The immutable result is published only after every check. No history mutation,
/// observation refresh, auto-anchor, fit, normalization, network, clock or other
/// external effect occurs. Numerical range is no adopted age/eligibility policy.
/// # Errors
/// Context mismatch precedes missing report, missing anchor, exact anchor-time
/// mismatch, projection before latest time, then exact duration/conversion and
/// separately checked arithmetic. Rejection returns no partial projection.
pub fn project_fixture_anchor(
    history: &FixtureHistory,
    expected_context: FixtureContext,
    anchor_index: usize,
    expected_anchor_at: MonotonicNs,
    at: MonotonicNs,
    terms: FixtureProjectionTerms,
) -> Result<FixtureAnchorProjection, FixtureProjectionError> {
    ProjectionSource::fixed(history, expected_context, anchor_index, expected_anchor_at)?
        .project(at, terms)
}

/// Project one explicitly identified current fitted adaptive-history anchor.
///
/// Borrows the adaptive owner's own committed report and current fitted row.
/// `anchor_index` is relative to `history.observation`, not the combined decision
/// window or the selector's pre-prune input. Auxiliary rows are never anchors.
/// The configured minimum is a pruning floor, not an added eligibility condition:
/// every actual committed report may supply its same-owner checked fitted rate.
///
/// Reuses the fixed-history projection's exact duration/phase conversions and five
/// separately checked arithmetic primitives. It does not fit, scan history, copy
/// owned rows, mutate state or refresh evidence; all instants keep their original
/// meaning. Supplied terms and the result grant no trust, freshness or correction
/// authority. Work and scalar scratch are constant and no allocation is required.
/// # Errors
/// Context mismatch precedes missing report, missing fitted anchor, exact anchor
/// time mismatch, projection before latest time, then exact duration/conversion
/// and separately checked arithmetic. No partial result is published on failure.
pub fn project_fixture_adaptive_anchor(
    history: &FixtureAdaptiveHistory,
    expected_context: FixtureContext,
    anchor_index: usize,
    expected_anchor_at: MonotonicNs,
    at: MonotonicNs,
    terms: FixtureProjectionTerms,
) -> Result<FixtureAnchorProjection, FixtureProjectionError> {
    ProjectionSource::adaptive(history, expected_context, anchor_index, expected_anchor_at)?
        .project(at, terms)
}

/// Prepare same-owner raw-anchor and fitted tracking output at one explicit time.
///
/// The single whole-history capsule binds anchor, line and selected statistics.
/// Complete original projection precedes supplied-skew admission, same-fit
/// standard errors, fit-reference age, and four separate tracking primitives.
/// Final mandatory scalar admission precedes the sole attachment publication.
/// No detached projection/report pairing helper exists. The owner and all real
/// observation instants stay unchanged; no fit, history scan or effect occurs.
/// # Errors
/// Preserve existing projection failures first, then skew, standard-error,
/// reference duration/conversion, arithmetic and final scalar causes in order.
pub fn project_fixture_adaptive_tracking(
    history: &FixtureAdaptiveHistory,
    expected_context: FixtureContext,
    anchor_index: usize,
    expected_anchor_at: MonotonicNs,
    at: MonotonicNs,
    projection_terms: FixtureProjectionTerms,
    tracking_skew_seconds_per_second: f64,
) -> Result<FixtureAnchorProjection, FixtureTrackingProjectionError> {
    let source =
        ProjectionSource::adaptive(history, expected_context, anchor_index, expected_anchor_at)
            .map_err(|source| FixtureTrackingProjectionError::Projection { source })?;
    let statistics = source.statistics;
    let mut projection = source
        .project(at, projection_terms)
        .map_err(|source| FixtureTrackingProjectionError::Projection { source })?;
    let skew = admit_tracking_skew(tracking_skew_seconds_per_second)?;
    let errors = statistics
        .standard_errors()
        .map_err(|source| FixtureTrackingProjectionError::StandardErrors { source })?;
    let elapsed = at
        .duration_since(projection.fit_reference)
        .map_err(|source| FixtureTrackingProjectionError::Projection {
            source: FixtureProjectionError::Domain { source },
        })?;
    let elapsed = elapsed_seconds(elapsed)
        .map_err(|source| FixtureTrackingProjectionError::Projection { source })?;
    let shift = tracking_arithmetic(
        projection.phase_rate_seconds_per_second,
        elapsed,
        FixtureTrackingStep::RateShift,
    )?;
    let phase = tracking_arithmetic(
        statistics.line().phase_seconds(),
        shift,
        FixtureTrackingStep::Phase,
    )?;
    let rate_sd = errors.rate_standard_error_seconds_per_second();
    let growth = tracking_arithmetic(
        rate_sd,
        elapsed,
        FixtureTrackingStep::StandardDeviationGrowth,
    )?;
    let phase_sd = tracking_arithmetic(
        errors.phase_standard_error_seconds(),
        growth,
        FixtureTrackingStep::StandardDeviation,
    )?;
    let candidate = TrackingCandidate {
        phase,
        phase_sd,
        rate_sd,
        skew,
    };
    candidate.admit()?;
    projection.tracking = Some(FixtureTrackingEstimate {
        phase_seconds: candidate.phase,
        phase_standard_deviation_seconds: candidate.phase_sd,
        rate_standard_deviation_seconds_per_second: candidate.rate_sd,
        tracking_skew_seconds_per_second: candidate.skew,
    });
    Ok(projection)
}

/// Raw source-derived guards construct no checked history, terms or projection.
#[cfg(test)]
#[path = "../../tests/private/projection_guards.rs"]
mod projection_guards;

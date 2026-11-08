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
//! Borrowed mathematical affine views of genuine adaptive fixture histories.
//!
//! Modified by David Osipov with AI assistance, 3 November 2026. The conceptual
//! history-wide coordinate obligation follows Chrony revision
//! `8df4f1263e206585e6c4e61352d042c2dd12916a`, `SST_SlewSamples` and
//! `slew_sources`, under the preceding source ledger. This is not a translation
//! of their wall-clock timestamp changes or rate denominator. Monotonic times,
//! original Q33 observations and the original fitted report remain unchanged.
//!
//! One checked descriptor defines a mathematical chart on an inclusive time
//! domain. A view borrows that descriptor and the whole history; its two real
//! projection paths produce original source values before applying the same
//! saved displacement to anchor and fitted centers. Separate numerical bounds
//! cover the newly evaluated binary64 operations and retained coefficient
//! composition error. No value grants an achieved-correction claim, current
//! publication authority or physical UTC
//! enclosure. No append, refit, allocation of history storage or external effect
//! occurs. Preparation scans at most 64 rows twice for mandatory Garde and typed
//! admission; projection uses constant scalar scratch and retains one descriptor
//! borrow plus one original immutable projection.

use std::{fmt, num::NonZeroU64};

use garde::Validate;
use time_domain::{ActiveAwakeDurationNs, DomainError, MonotonicNs};

use super::{
    ArithmeticOperation, FitError, FixtureAdaptiveHistory, FixtureAnchorProjection, FixtureContext,
    FixtureProjectionError, FixtureProjectionTerms, FixtureTrackingEstimate,
    FixtureTrackingProjectionError, MAX_POINTS, NormalizationError,
    project_fixture_adaptive_anchor, project_fixture_adaptive_tracking,
};

/// Sole numerical owner of separately evaluated values and outward error bounds.
mod roundoff;

pub use roundoff::{
    FixturePhaseNumericError, FixturePhaseNumericField, FixturePhaseNumericIssue,
    FixturePhaseNumericStep,
};
pub(crate) use roundoff::{PhaseDebt, PhaseRates};

/// Inclusive integer magnitude that has an exact binary64 representation.
const MAX_EXACT_NANOSECONDS: u64 = 1_u64 << 53;

/// Raw or transformed scalar role, in the applicable constructor's stated order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureChartField {
    /// Signed displacement in seconds at the common descriptor reference.
    Displacement,
    /// Signed phase rate in seconds per second.
    Rate,
    /// Signed transformed raw-anchor center in seconds.
    AnchorCenter,
    /// Signed transformed same-owner fitted phase in seconds.
    TrackingPhase,
    /// Nonnegative new-arithmetic error bound for the raw-anchor center.
    CenterRoundoff,
    /// Nonnegative new-arithmetic error bound for the fitted rate.
    RateRoundoff,
    /// Nonnegative new-arithmetic error bound for the fitted phase.
    TrackingRoundoff,
    /// Nonnegative displacement-coefficient error in seconds at the reference.
    PhaseCoefficientRoundoff,
    /// Nonnegative rate-coefficient error in seconds per second.
    RateCoefficientRoundoff,
}

/// First unsupported scalar property; signed zero is always supported.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureChartIssue {
    /// The input is NaN or either infinity.
    NonFinite,
    /// A nonzero subnormal occurs before any field-specific sign check.
    Subnormal,
    /// A nonnegative bound is negative.
    Negative,
}

/// One fixed unfused primitive in the numerical value transformation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureChartStep {
    /// Divide the exact signed nanosecond difference by one billion.
    SecondsDivide,
    /// Multiply the descriptor rate by the converted elapsed seconds.
    RateTimesElapsed,
    /// Add displacement at the common reference to the checked rate shift.
    Displacement,
    /// Subtract the saved displacement from the original raw-anchor center.
    AnchorCenter,
    /// Subtract that same saved displacement from the original fitted phase.
    TrackingPhase,
    /// Subtract descriptor rate from the original fitted rate.
    Rate,
    /// Add the two displacement coefficients at the identical common reference.
    ComposeDisplacement,
    /// Add the two rate coefficients over the unchanged monotonic time axis.
    ComposeRate,
}

impl FixtureChartStep {
    /// Select the sole primitive for this role without evaluating operands.
    pub(super) const fn operation(self) -> ArithmeticOperation {
        match self {
            Self::SecondsDivide => ArithmeticOperation::Divide,
            Self::RateTimesElapsed => ArithmeticOperation::Multiply,
            Self::Displacement | Self::ComposeDisplacement | Self::ComposeRate => {
                ArithmeticOperation::Add
            }
            Self::AnchorCenter | Self::TrackingPhase | Self::Rate => ArithmeticOperation::Subtract,
        }
    }
}

/// Positive outward-bound operation, separate from estimated-value arithmetic.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureChartBoundOperation {
    /// Outward addition of two nonnegative quantities.
    Add,
    /// Outward multiplication of two nonnegative quantities.
    Multiply,
    /// Outward division by a strictly positive denominator.
    Divide,
}

impl FixtureChartBoundOperation {
    /// Evaluate this nonnegative primitive through the sole outward-bound owner.
    ///
    /// This crate-private bridge shares arithmetic without exposing a raw chart
    /// constructor or changing coordinate values. Work is fixed and effect-free.
    /// # Errors
    /// Preserve invalid operands/divisors and exhausted finite-upper causes.
    pub(crate) fn evaluate_up(self, left: f64, right: f64) -> Result<f64, FixtureChartError> {
        roundoff::bound_arithmetic(left, right, self)
    }
}

/// Bounded cause from the outward-bound owner; contains no numeric input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureChartBoundIssue {
    /// A bound operand is negative or nonfinite.
    InvalidOperand,
    /// The bound denominator is zero or negative.
    NonPositiveDivisor,
    /// Evaluation or outward successor cannot produce a finite upper bound.
    NonFiniteResult,
}

/// Closed chart rejection; no history, descriptor or raw scalar is retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureChartError {
    /// Target frame or target generation equals its corresponding source tag.
    InvalidFrames,
    /// Domain order or its inclusive exact-integer span is unsupported.
    InvalidDomain,
    /// Expected raw or target context differs from its actual whole owner.
    ContextMismatch,
    /// Descriptor source frame/generation differs from the raw history context.
    SourceMappingMismatch,
    /// The first chart's target frame or generation differs from the next source.
    CompositionMappingMismatch,
    /// The two complete composition domains or common references differ.
    CompositionDomainMismatch,
    /// The genuine history has no sufficient fitted report.
    MissingReport,
    /// A chronological combined auxiliary/fitted row lies outside the domain.
    HistoryOutsideDomain {
        /// Bounded index in the actual combined history, never a caller path.
        index: usize,
    },
    /// The genuine report reference lies outside the descriptor domain.
    ReferenceOutsideDomain,
    /// Query time lies outside the descriptor's inclusive mathematical domain.
    QueryOutsideDomain,
    /// Unknown fixture outcome cannot prepare a chart view.
    UnknownOutcome,
    /// A scalar fails the shared supported-domain or nonnegative-bound rule.
    InvalidScalar {
        /// Actual closed role of the rejected scalar.
        field: FixtureChartField,
        /// First failed finite/subnormal/sign property.
        issue: FixtureChartIssue,
    },
    /// Mandatory Garde admission disagrees with its maintained typed predicate.
    ValidationInvariant,
    /// An immutable producer or private association invariant was violated.
    Invariant,
    /// Actual exact time-domain operation failed.
    Domain {
        /// Original bounded typed time-domain cause.
        source: DomainError,
    },
    /// Exact integer conversion rejected its supported representation.
    Normalization {
        /// Original bounded typed conversion cause.
        source: NormalizationError,
    },
    /// The unchanged original anchor producer rejected before chart arithmetic.
    Projection {
        /// Original bounded typed projection cause.
        source: FixtureProjectionError,
    },
    /// The unchanged original fitted tracking producer rejected first.
    Tracking {
        /// Original bounded typed tracking cause, including its nested producer.
        source: FixtureTrackingProjectionError,
    },
    /// A separate checked value primitive failed before result publication.
    Arithmetic {
        /// Exact primitive's closed role in the chart transformation.
        step: FixtureChartStep,
        /// Actual shared Response-stage arithmetic cause.
        source: FitError,
    },
    /// A positive outward arithmetic bound could not be represented.
    Bound {
        /// Exact positive-bound operation that failed.
        operation: FixtureChartBoundOperation,
        /// Closed cause without raw operand values.
        issue: FixtureChartBoundIssue,
    },
    /// The exact Q33 enclosure consumer rejected the transformed geometry.
    Enclosure {
        /// Original bounded typed enclosure cause.
        source: crate::FixtureEnclosureError,
    },
}

impl fmt::Display for FixtureChartError {
    /// Emit bounded fixed labels and closed fields without exposing numeric input.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFrames => formatter.write_str("chart frame relationship"),
            Self::InvalidDomain => formatter.write_str("chart domain relationship"),
            Self::ContextMismatch => formatter.write_str("chart context mismatch"),
            Self::SourceMappingMismatch => formatter.write_str("chart source mapping mismatch"),
            Self::CompositionMappingMismatch => {
                formatter.write_str("chart composition mapping mismatch")
            }
            Self::CompositionDomainMismatch => {
                formatter.write_str("chart composition domain mismatch")
            }
            Self::MissingReport => formatter.write_str("chart missing fitted report"),
            Self::HistoryOutsideDomain { index } => {
                write!(formatter, "chart history domain index {index}")
            }
            Self::ReferenceOutsideDomain => formatter.write_str("chart reference outside domain"),
            Self::QueryOutsideDomain => formatter.write_str("chart query outside domain"),
            Self::UnknownOutcome => formatter.write_str("chart outcome unknown"),
            Self::InvalidScalar { field, issue } => {
                write!(formatter, "chart scalar {field:?}: {issue:?}")
            }
            Self::ValidationInvariant => formatter.write_str("chart validation invariant"),
            Self::Invariant => formatter.write_str("chart producer invariant"),
            Self::Domain { .. } => formatter.write_str("chart time-domain failure"),
            Self::Normalization { .. } => formatter.write_str("chart conversion failure"),
            Self::Projection { .. } => formatter.write_str("chart original projection failure"),
            Self::Tracking { .. } => formatter.write_str("chart original tracking failure"),
            Self::Arithmetic { step, .. } => write!(formatter, "chart arithmetic {step:?}"),
            Self::Bound { operation, issue } => {
                write!(formatter, "chart bound {operation:?}: {issue:?}")
            }
            Self::Enclosure { .. } => formatter.write_str("chart enclosure failure"),
        }
    }
}

impl std::error::Error for FixtureChartError {
    /// Borrow each actual typed cause; local predicates have no synthetic cause.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Domain { source } => Some(source),
            Self::Normalization { source } => Some(source),
            Self::Projection { source } => Some(source),
            Self::Tracking { source } => Some(source),
            Self::Arithmetic { source, .. } => Some(source),
            Self::Enclosure { source } => Some(source),
            Self::InvalidFrames
            | Self::InvalidDomain
            | Self::ContextMismatch
            | Self::SourceMappingMismatch
            | Self::CompositionMappingMismatch
            | Self::CompositionDomainMismatch
            | Self::MissingReport
            | Self::HistoryOutsideDomain { .. }
            | Self::ReferenceOutsideDomain
            | Self::QueryOutsideDomain
            | Self::UnknownOutcome
            | Self::InvalidScalar { .. }
            | Self::ValidationInvariant
            | Self::Invariant
            | Self::Bound { .. } => None,
        }
    }
}

/// Four raw nonzero tags; the tuple's nonzero invariant belongs to `NonZeroU64`.
#[derive(Validate)]
struct FramesCandidate {
    /// Custom rule owns both changed-mapping relationship checks together.
    #[garde(custom(frames_rule))]
    labels: (NonZeroU64, NonZeroU64, NonZeroU64, NonZeroU64),
}

/// Check distinct frame and generation pairs, preserving all four tag bits.
fn frames_valid(labels: &(NonZeroU64, NonZeroU64, NonZeroU64, NonZeroU64)) -> bool {
    labels.0 != labels.2 && labels.1 != labels.3
}

/// Adapt the sole frame predicate to mandatory borrowed Garde admission.
/// # Errors
/// Reject equal source/target frame or generation with fixed text and no effects.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rule signature borrows its raw field and unit context"
)]
fn frames_rule(
    labels: &(NonZeroU64, NonZeroU64, NonZeroU64, NonZeroU64),
    (): &(),
) -> garde::Result {
    if frames_valid(labels) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid chart frame relationship"))
    }
}

/// Checked asserted mapping labels; these tags establish no freshness or authority.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct FixtureChartFrames {
    /// Source composite monotonic-origin and phase-mapping tag.
    source_frame: NonZeroU64,
    /// Source revision assertion, preserved as raw provenance.
    source_generation: NonZeroU64,
    /// Distinct target composite mapping tag, with unchanged numeric monotonic data.
    target_frame: NonZeroU64,
    /// Distinct target revision assertion, without a global epoch claim.
    target_generation: NonZeroU64,
}

impl fmt::Debug for FixtureChartFrames {
    /// Redact asserted mapping labels with fixed bounded metadata.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureChartFrames { checked: true, .. }")
    }
}

impl FixtureChartFrames {
    /// Admit distinct source/target frame and generation tags without mutation.
    /// # Errors
    /// Reject frame then generation relationship failures, or Garde disagreement.
    pub fn new(
        source_frame: NonZeroU64,
        source_generation: NonZeroU64,
        target_frame: NonZeroU64,
        target_generation: NonZeroU64,
    ) -> Result<Self, FixtureChartError> {
        let raw = FramesCandidate {
            labels: (
                source_frame,
                source_generation,
                target_frame,
                target_generation,
            ),
        };
        let validated = raw.validate();
        if !frames_valid(&raw.labels) {
            return Err(FixtureChartError::InvalidFrames);
        }
        validated.map_err(|_| FixtureChartError::ValidationInvariant)?;
        Ok(Self {
            source_frame,
            source_generation,
            target_frame,
            target_generation,
        })
    }

    /// Return the original asserted composite frame without authority or effects.
    #[must_use]
    pub const fn source_frame(&self) -> NonZeroU64 {
        self.source_frame
    }

    /// Return the original revision assertion without a current-state check.
    #[must_use]
    pub const fn source_generation(&self) -> NonZeroU64 {
        self.source_generation
    }

    /// Return the distinct target composite mapping tag without relabelling reports.
    #[must_use]
    pub const fn target_frame(&self) -> NonZeroU64 {
        self.target_frame
    }

    /// Return the target revision assertion without granting publication authority.
    #[must_use]
    pub const fn target_generation(&self) -> NonZeroU64 {
        self.target_generation
    }
}

/// Three raw immutable domain times with one mandatory relationship owner.
#[derive(Validate)]
struct DomainCandidate {
    /// `MonotonicNs` owns representation; this rule owns ordering and exact span.
    #[garde(custom(domain_rule))]
    times: (MonotonicNs, MonotonicNs, MonotonicNs),
}

/// Admit inclusive order and a total span at most `2^53` ns using integer work.
fn domain_valid(times: &(MonotonicNs, MonotonicNs, MonotonicNs)) -> bool {
    let (start, reference, end) = *times;
    start <= reference
        && reference <= end
        && end
            .nanoseconds()
            .checked_sub(start.nanoseconds())
            .is_some_and(|span| span <= MAX_EXACT_NANOSECONDS)
}

/// Adapt the same domain relationship to mandatory Garde admission.
/// # Errors
/// Reject reversed times or an inexact-integer span without retaining input.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rule signature borrows its raw field and unit context"
)]
fn domain_rule(times: &(MonotonicNs, MonotonicNs, MonotonicNs), (): &()) -> garde::Result {
    if domain_valid(times) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid chart domain"))
    }
}

/// Checked inclusive mathematical domain and one common displacement reference.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct FixtureChartDomain {
    /// Inclusive earliest supported instant, not a policy age cutoff.
    start: MonotonicNs,
    /// One descriptor-wide reference, independent of each history's fitted origin.
    reference: MonotonicNs,
    /// Inclusive latest supported instant, not evidence of actuator delivery.
    end: MonotonicNs,
}

impl fmt::Debug for FixtureChartDomain {
    /// Redact numerical domain instants with fixed bounded metadata.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureChartDomain { checked: true, .. }")
    }
}

impl FixtureChartDomain {
    /// Admit inclusive order and exact representability of every relative integer.
    /// # Errors
    /// Reject unsupported domain relationships or mandatory-validation disagreement.
    pub fn new(
        start: MonotonicNs,
        common_reference: MonotonicNs,
        end: MonotonicNs,
    ) -> Result<Self, FixtureChartError> {
        let raw = DomainCandidate {
            times: (start, common_reference, end),
        };
        let validated = raw.validate();
        if !domain_valid(&raw.times) {
            return Err(FixtureChartError::InvalidDomain);
        }
        validated.map_err(|_| FixtureChartError::ValidationInvariant)?;
        Ok(Self {
            start,
            reference: common_reference,
            end,
        })
    }

    /// Return the unchanged inclusive domain start without effects.
    #[must_use]
    pub const fn start(&self) -> MonotonicNs {
        self.start
    }

    /// Return the single common reference used for displacement in every history.
    #[must_use]
    pub const fn common_reference(&self) -> MonotonicNs {
        self.reference
    }

    /// Return the unchanged inclusive mathematical endpoint without authority.
    #[must_use]
    pub const fn end(&self) -> MonotonicNs {
        self.end
    }

    /// Test an already represented monotonic time against inclusive domain limits.
    fn contains(&self, at: MonotonicNs) -> bool {
        self.start <= at && at <= self.end
    }
}

/// Classify raw scalar support before applying the bound-only nonnegative rule.
fn scalar_issue(value: f64, field: FixtureChartField) -> Option<FixtureChartIssue> {
    if !value.is_finite() {
        Some(FixtureChartIssue::NonFinite)
    } else if value.is_subnormal() {
        Some(FixtureChartIssue::Subnormal)
    } else if value < 0.0
        && matches!(
            field,
            FixtureChartField::CenterRoundoff
                | FixtureChartField::RateRoundoff
                | FixtureChartField::TrackingRoundoff
                | FixtureChartField::PhaseCoefficientRoundoff
                | FixtureChartField::RateCoefficientRoundoff
        )
    {
        Some(FixtureChartIssue::Negative)
    } else {
        None
    }
}

/// Adapt signed finite-normal-or-zero scalar support without changing value bits.
/// # Errors
/// Reject unsupported floating values with fixed text and no effects.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rules borrow raw fields and context"
)]
fn signed_rule(value: &f64, (): &()) -> garde::Result {
    if scalar_issue(*value, FixtureChartField::Displacement).is_none() {
        Ok(())
    } else {
        Err(garde::Error::new("invalid chart signed scalar"))
    }
}

/// Adapt the same scalar support with the nonnegative numerical-bound invariant.
/// # Errors
/// Reject unsupported or negative bounds without normalizing their values.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rules borrow raw fields and context"
)]
fn bound_rule(value: &f64, (): &()) -> garde::Result {
    if scalar_issue(*value, FixtureChartField::CenterRoundoff).is_none() {
        Ok(())
    } else {
        Err(garde::Error::new("invalid chart bound scalar"))
    }
}

/// Descriptor point/error scalars; checked frame/domain groups retain their owners.
#[derive(Validate)]
struct AffineCandidate {
    /// Signed seconds at the one common reference.
    #[garde(custom(signed_rule))]
    phase: f64,
    /// Signed seconds per second; no clock-policy range is inferred.
    #[garde(custom(signed_rule))]
    rate: f64,
    /// Nonnegative displacement-coefficient error, separate from physical error.
    #[garde(custom(bound_rule))]
    phase_error: f64,
    /// Nonnegative rate-coefficient error, separate from statistical deviation.
    #[garde(custom(bound_rule))]
    rate_error: f64,
}

impl AffineCandidate {
    /// Consume mandatory validation and preserve the first ordered scalar failure.
    /// # Errors
    /// Reject phase, rate, their respective bounds or validation disagreement.
    fn admit(&self) -> Result<(), FixtureChartError> {
        let validated = self.validate();
        for (value, field) in [
            (self.phase, FixtureChartField::Displacement),
            (self.rate, FixtureChartField::Rate),
            (
                self.phase_error,
                FixtureChartField::PhaseCoefficientRoundoff,
            ),
            (self.rate_error, FixtureChartField::RateCoefficientRoundoff),
        ] {
            if let Some(issue) = scalar_issue(value, field) {
                return Err(FixtureChartError::InvalidScalar { field, issue });
            }
        }
        validated.map_err(|_| FixtureChartError::ValidationInvariant)
    }
}

/// Check the unchanged time axis and the complete intermediate/end-point chain.
///
/// Mapping mismatch precedes domain mismatch, then the existing frame rules
/// reject an endpoint frame or generation cycle. Neither input is modified.
fn composition_error(
    first: &FixtureAffineChart,
    next: &FixtureAffineChart,
) -> Option<FixtureChartError> {
    if first.frames.target_frame != next.frames.source_frame
        || first.frames.target_generation != next.frames.source_generation
    {
        Some(FixtureChartError::CompositionMappingMismatch)
    } else if first.domain != next.domain {
        Some(FixtureChartError::CompositionDomainMismatch)
    } else if !frames_valid(&(
        first.frames.source_frame,
        first.frames.source_generation,
        next.frames.target_frame,
        next.frames.target_generation,
    )) {
        Some(FixtureChartError::InvalidFrames)
    } else {
        None
    }
}

/// One mandatory whole relationship over the actual immutable composition inputs.
#[derive(Validate)]
struct CompositionCandidate<'chart> {
    /// Checked descriptors own scalar invariants; this rule owns their association.
    #[garde(custom(|charts: &(&FixtureAffineChart, &FixtureAffineChart), (): &()| {
        if composition_error(charts.0, charts.1).is_none() {
            Ok(())
        } else {
            Err(garde::Error::new("invalid chart composition relationship"))
        }
    }))]
    charts: (&'chart FixtureAffineChart, &'chart FixtureAffineChart),
}

/// Immutable mathematical displacement descriptor; no achieved-effect authority.
///
/// Checked labels and domain have a legal direct constructor path:
/// ```
/// use std::num::NonZeroU64;
/// use time_core::{FixtureAffineChart, FixtureChartDomain, FixtureChartFrames};
/// use time_domain::MonotonicNs;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let frames = FixtureChartFrames::new(
///     NonZeroU64::new(7).ok_or("zero label")?,
///     NonZeroU64::new(11).ok_or("zero label")?,
///     NonZeroU64::new(8).ok_or("zero label")?,
///     NonZeroU64::new(12).ok_or("zero label")?,
/// )?;
/// let domain = FixtureChartDomain::new(
///     MonotonicNs::new(0), MonotonicNs::new(1), MonotonicNs::new(2),
/// )?;
/// let chart = FixtureAffineChart::new(frames, domain, 0.0, 0.0)?;
/// assert_eq!(chart.frames(), frames);
/// assert_eq!(chart.domain(), domain);
/// # Ok(())
/// # }
/// ```
pub struct FixtureAffineChart {
    /// Constructor-owned distinct source/target tags, reused without raw validation.
    frames: FixtureChartFrames,
    /// Constructor-owned inclusive exact-integer domain.
    domain: FixtureChartDomain,
    /// Admitted signed displacement seconds at the domain's common reference.
    phase: f64,
    /// Admitted signed displacement rate in seconds per second.
    rate: f64,
    /// Absolute numerical error of the displacement coefficient at the reference.
    phase_error: f64,
    /// Absolute numerical error of the rate coefficient over the unchanged axis.
    rate_error: f64,
}

impl fmt::Debug for FixtureAffineChart {
    /// Redact chart numeric values and fixture labels with constant bounded output.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureAffineChart { checked: true, .. }")
    }
}

impl FixtureAffineChart {
    /// Check the shared raw rate envelope without asserting delivery or policy.
    ///
    /// Signed zeros and every finite normal magnitude are admitted unchanged.
    /// This bridge owns no current-event, clock, lifecycle or authority decision.
    /// # Errors
    /// Reject nonfinite then subnormal rates through the existing scalar rule.
    pub(crate) fn check_delivery_rate(rate: f64) -> Result<(), FixtureChartError> {
        if let Some(issue) = scalar_issue(rate, FixtureChartField::Rate) {
            return Err(FixtureChartError::InvalidScalar {
                field: FixtureChartField::Rate,
                issue,
            });
        }
        Ok(())
    }

    /// Build one checked zero-rate effect from delivered rate and exact awake time.
    ///
    /// The supplied duration retains its active-awake owner; no monotonic instant
    /// is fabricated or changed. The sole roundoff owner supplies the separately
    /// rounded phase and its error against exact stored rate bits times exact
    /// nanoseconds. Mandatory private admission retains both before publication.
    /// This numerical descriptor grants no delivery, readiness or clock authority.
    /// # Errors
    /// Reject raw rate support first, then exact duration/point/bound arithmetic
    /// and final scalar admission; no descriptor or input is changed on failure.
    pub(crate) fn settled_rate_effect(
        frames: FixtureChartFrames,
        domain: FixtureChartDomain,
        rate: f64,
        elapsed: ActiveAwakeDurationNs,
    ) -> Result<Self, FixtureChartError> {
        Self::check_delivery_rate(rate)?;
        let coefficients = roundoff::settled_rate_coefficients(rate, elapsed)?;
        coefficients.admit()?;
        Ok(Self {
            frames,
            domain,
            phase: coefficients.phase,
            rate: coefficients.rate,
            phase_error: coefficients.phase_error,
            rate_error: coefficients.rate_error,
        })
    }

    /// Admit exact stored-bit coefficients with zero coefficient-error bounds.
    /// # Errors
    /// Reject displacement before rate, or mandatory Garde disagreement; no effects.
    pub fn new(
        frames: FixtureChartFrames,
        domain: FixtureChartDomain,
        phase_at_reference_seconds: f64,
        rate_seconds_per_second: f64,
    ) -> Result<Self, FixtureChartError> {
        let raw = AffineCandidate {
            phase: phase_at_reference_seconds,
            rate: rate_seconds_per_second,
            phase_error: 0.0,
            rate_error: 0.0,
        };
        raw.admit()?;
        Ok(Self {
            frames,
            domain,
            phase: raw.phase,
            rate: raw.rate,
            phase_error: raw.phase_error,
            rate_error: raw.rate_error,
        })
    }

    /// Compose A-to-B with B-to-C over one identical complete monotonic domain.
    ///
    /// The result subtracts the sum of both displacements, retaining A-to-C
    /// provenance and inherited errors from both operands. Point sums are
    /// separately rounded before either coefficient bound is constructed.
    /// Work/storage are constant; neither descriptor nor any history is mutated.
    /// This mathematical result acknowledges no clock effect or readiness change.
    /// # Errors
    /// Reject mapping, domain and endpoint cycles before arithmetic; preserve
    /// phase/rate point failures, phase/rate bound failures and final validation.
    pub fn compose(&self, next: &Self) -> Result<Self, FixtureChartError> {
        let raw = CompositionCandidate {
            charts: (self, next),
        };
        let validated = raw.validate();
        if let Some(error) = composition_error(raw.charts.0, raw.charts.1) {
            return Err(error);
        }
        validated.map_err(|_| FixtureChartError::ValidationInvariant)?;
        let frames = FixtureChartFrames::new(
            self.frames.source_frame,
            self.frames.source_generation,
            next.frames.target_frame,
            next.frames.target_generation,
        )?;
        let coefficients = roundoff::compose_coefficients(self, next)?;
        coefficients.admit()?;
        Ok(Self {
            frames,
            domain: self.domain,
            phase: coefficients.phase,
            rate: coefficients.rate,
            phase_error: coefficients.phase_error,
            rate_error: coefficients.rate_error,
        })
    }

    /// Return checked mapping labels without granting common-chart identity.
    #[must_use]
    pub const fn frames(&self) -> FixtureChartFrames {
        self.frames
    }

    /// Return the checked numerical domain without an actuator-delivery assertion.
    #[must_use]
    pub const fn domain(&self) -> FixtureChartDomain {
        self.domain
    }

    /// Return the unchanged admitted displacement at the common reference in seconds.
    #[must_use]
    pub const fn phase_at_reference_seconds(&self) -> f64 {
        self.phase
    }

    /// Return the unchanged admitted mathematical displacement rate in seconds/second.
    #[must_use]
    pub const fn rate_seconds_per_second(&self) -> f64 {
        self.rate
    }

    /// Return only displacement-coefficient arithmetic error at the reference.
    ///
    /// This bounds composition over exact stored input bits, not physical error,
    /// source-fit uncertainty or the later projection's own evaluation error.
    #[must_use]
    pub const fn phase_coefficient_roundoff_seconds(&self) -> f64 {
        self.phase_error
    }

    /// Return only rate-coefficient arithmetic error in seconds per second.
    ///
    /// Direct descriptors return zero; composition preserves both input bounds.
    /// No statistical uncertainty, current mapping or achieved effect is implied.
    #[must_use]
    pub const fn rate_coefficient_roundoff_seconds_per_second(&self) -> f64 {
        self.rate_error
    }
}

/// Fixture-only preparation outcome; no variant acknowledges a physical effect.
#[derive(Clone, Copy, Debug)]
pub enum FixtureChartOutcome<'chart> {
    /// Borrow this exact checked mathematical descriptor, not just matching labels.
    Known(&'chart FixtureAffineChart),
    /// Prepare no new view; the checked raw history remains unchanged.
    Refused,
    /// Reject this preparation without revoking any historical legacy result.
    Unknown,
}

/// Check real raw mapping/report and every retained row against one descriptor.
///
/// Both inputs are immutable checked whole owners. At most 64 combined rows are
/// inspected. Missing/replaced observations are invariants, never synthetic data.
fn history_error(
    history: &FixtureAdaptiveHistory,
    chart: &FixtureAffineChart,
) -> Option<FixtureChartError> {
    let context = history.context();
    if context.frame() != chart.frames.source_frame
        || context.generation() != chart.frames.source_generation
    {
        return Some(FixtureChartError::SourceMappingMismatch);
    }
    let Some(report) = history.report() else {
        return Some(FixtureChartError::MissingReport);
    };
    let Some(count) = history
        .sample_count()
        .checked_add(history.auxiliary_count())
    else {
        return Some(FixtureChartError::Invariant);
    };
    if count > MAX_POINTS {
        return Some(FixtureChartError::Invariant);
    }
    for index in 0..count {
        let Some(row) = history.decision_observation(index) else {
            return Some(FixtureChartError::Invariant);
        };
        if !chart.domain.contains(row.observed_at()) {
            return Some(FixtureChartError::HistoryOutsideDomain { index });
        }
    }
    if !chart.domain.contains(report.reference()) {
        return Some(FixtureChartError::ReferenceOutsideDomain);
    }
    None
}

/// Borrowed raw relationship carrier; individual owner types retain their invariants.
#[derive(Validate)]
struct HistoryCandidate<'history, 'chart> {
    /// The single rule checks every newly asserted mapping/domain association.
    #[garde(custom(history_rule))]
    owners: (&'history FixtureAdaptiveHistory, &'chart FixtureAffineChart),
}

/// Adapt the whole-owner relationship predicate without copying history or report.
/// # Errors
/// Reject mapping, sufficiency or domain failures with fixed text and no mutation.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rule signature borrows its raw field and unit context"
)]
fn history_rule(owners: &(&FixtureAdaptiveHistory, &FixtureAffineChart), (): &()) -> garde::Result {
    if history_error(owners.0, owners.1).is_none() {
        Ok(())
    } else {
        Err(garde::Error::new("invalid chart history association"))
    }
}

/// Immutable whole-history view; its borrow prevents append while it is used.
///
/// A live view cannot observe a history changed through a concurrent mutable borrow:
/// ```compile_fail
/// use time_core::{FixtureAdaptiveHistory, FixtureAffineChart, FixtureChartError,
///     FixtureChartOutcome, FixtureObservation};
/// fn change_while_viewed(history: &mut FixtureAdaptiveHistory,
///     chart: &FixtureAffineChart, row: FixtureObservation) -> Result<(), FixtureChartError> {
///     let Some(view) = history.prepare_chart(history.context(), FixtureChartOutcome::Known(chart))?
///         else { return Ok(()); };
///     let _ = history.try_append(row);
///     let _ = view.context();
///     Ok(())
/// }
/// ```
pub struct FixtureAdaptiveChartView<'history, 'chart> {
    /// Genuine checked source owner; raw rows and report are never relabelled.
    history: &'history FixtureAdaptiveHistory,
    /// Exact checked common-reference descriptor used for every projection.
    chart: &'chart FixtureAffineChart,
}

impl fmt::Debug for FixtureAdaptiveChartView<'_, '_> {
    /// Emit fixed bounded view metadata without raw observations or chart values.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureAdaptiveChartView { checked: true, .. }")
    }
}

impl FixtureAdaptiveHistory {
    /// Prepare a borrowed mathematical chart without changing any source state.
    ///
    /// Raw context is checked first. Refusal needs no fitted report and yields no
    /// view. Known descriptors then require source labels, a genuine report, all
    /// combined auxiliary/fitted rows and its reference inside the domain. No
    /// target-chart append, live acknowledgement or generation migration occurs.
    /// # Errors
    /// Reject context, unknown outcome, mapping/report/domain, then Garde mismatch.
    pub fn prepare_chart<'history, 'chart>(
        &'history self,
        expected_raw_context: FixtureContext,
        outcome: FixtureChartOutcome<'chart>,
    ) -> Result<Option<FixtureAdaptiveChartView<'history, 'chart>>, FixtureChartError> {
        if self.context() != expected_raw_context {
            return Err(FixtureChartError::ContextMismatch);
        }
        let chart = match outcome {
            FixtureChartOutcome::Refused => return Ok(None),
            FixtureChartOutcome::Unknown => return Err(FixtureChartError::UnknownOutcome),
            FixtureChartOutcome::Known(chart) => chart,
        };
        let raw = HistoryCandidate {
            owners: (self, chart),
        };
        let validated = raw.validate();
        if let Some(error) = history_error(self, chart) {
            return Err(error);
        }
        validated.map_err(|_| FixtureChartError::ValidationInvariant)?;
        Ok(Some(FixtureAdaptiveChartView {
            history: self,
            chart,
        }))
    }
}

/// Raw typed request grouping; the real producer owns anchor/time/terms admission.
#[derive(Clone, Copy)]
struct ChartRequest {
    /// Caller assertion checked against the view's target context first.
    expected_context: FixtureContext,
    /// Current fitted-row index, never an auxiliary/selection-array coordinate.
    anchor_index: usize,
    /// Exact asserted source anchor time admitted by the original producer.
    expected_anchor_at: MonotonicNs,
    /// Common query time, checked by original producer before chart domain.
    at: MonotonicNs,
    /// Immutable terms admitted by their existing mandatory constructor.
    terms: FixtureProjectionTerms,
}

/// Closed dispatch to genuine whole-history producers; no detached result input.
#[derive(Clone, Copy)]
enum ChartProjectionKind {
    /// Preserve original raw-anchor-only producer semantics.
    Anchor,
    /// Preserve original tracking producer semantics including raw-skew admission.
    Tracking {
        /// Untrusted skew, admitted only by the unchanged original producer.
        skew: f64,
    },
}

impl<'chart> FixtureAdaptiveChartView<'_, 'chart> {
    /// Return the real source context without substituting target frame labels.
    #[must_use]
    pub const fn source_context(&self) -> FixtureContext {
        self.history.context()
    }

    /// Return target mapping assertions with the unchanged actual source identity.
    #[must_use]
    pub const fn context(&self) -> FixtureContext {
        FixtureContext::new(
            self.history.context().source_id(),
            self.chart.frames.target_frame,
            self.chart.frames.target_generation,
        )
    }

    /// Borrow the exact immutable descriptor, preserving common-chart identity.
    #[must_use]
    pub const fn chart(&self) -> &'chart FixtureAffineChart {
        self.chart
    }

    /// Project one genuine fitted anchor and carry its new arithmetic error bound.
    /// # Errors
    /// Check target context, original producer, query domain, then new arithmetic.
    /// No partial projection, report change or observation credit is published.
    pub fn project_anchor(
        &self,
        expected_target_context: FixtureContext,
        anchor_index: usize,
        expected_anchor_at: MonotonicNs,
        at: MonotonicNs,
        terms: FixtureProjectionTerms,
    ) -> Result<FixtureChartProjection<'chart>, FixtureChartError> {
        self.project(
            ChartRequest {
                expected_context: expected_target_context,
                anchor_index,
                expected_anchor_at,
                at,
                terms,
            },
            ChartProjectionKind::Anchor,
        )
    }

    /// Project real raw-anchor and fitted tracking centers through one saved delta.
    /// # Errors
    /// Preserve target-context and complete original-tracking errors before chart
    /// domain/arithmetic errors; all source rows, SD/skew and report remain intact.
    pub fn project_tracking(
        &self,
        expected_target_context: FixtureContext,
        anchor_index: usize,
        expected_anchor_at: MonotonicNs,
        at: MonotonicNs,
        terms: FixtureProjectionTerms,
        tracking_skew: f64,
    ) -> Result<FixtureChartProjection<'chart>, FixtureChartError> {
        self.project(
            ChartRequest {
                expected_context: expected_target_context,
                anchor_index,
                expected_anchor_at,
                at,
                terms,
            },
            ChartProjectionKind::Tracking {
                skew: tracking_skew,
            },
        )
    }

    /// Run the actual same-owner producer, transform, admit and publish once.
    ///
    /// Only raw request fields enter this private helper. No caller can pair a
    /// detached checked projection/report with a different descriptor or history.
    /// # Errors
    /// Preserve each original cause and reject unsupported derived values atomically.
    fn project(
        &self,
        request: ChartRequest,
        kind: ChartProjectionKind,
    ) -> Result<FixtureChartProjection<'chart>, FixtureChartError> {
        if request.expected_context != self.context() {
            return Err(FixtureChartError::ContextMismatch);
        }
        let original = match kind {
            ChartProjectionKind::Anchor => project_fixture_adaptive_anchor(
                self.history,
                self.history.context(),
                request.anchor_index,
                request.expected_anchor_at,
                request.at,
                request.terms,
            )
            .map_err(|source| FixtureChartError::Projection { source })?,
            ChartProjectionKind::Tracking { skew } => project_fixture_adaptive_tracking(
                self.history,
                self.history.context(),
                request.anchor_index,
                request.expected_anchor_at,
                request.at,
                request.terms,
                skew,
            )
            .map_err(|source| FixtureChartError::Tracking { source })?,
        };
        if !self.chart.domain.contains(request.at) {
            return Err(FixtureChartError::QueryOutsideDomain);
        }
        let scalars = roundoff::transform(self.chart, &original)?;
        let raw = ProjectionCandidate {
            center: scalars.center,
            rate: scalars.rate,
            center_error: scalars.center_error,
            rate_error: scalars.rate_error,
        };
        raw.admit()?;
        let tracking = match (original.tracking(), scalars.tracking) {
            (None, None) => None,
            (Some(source), Some(phase)) => {
                let raw_phase = TrackingPhaseCandidate {
                    value: phase.value,
                    error: phase.error,
                };
                raw_phase.admit()?;
                Some(FixtureChartTracking {
                    source: *source,
                    phase: phase.value,
                    phase_error: phase.error,
                })
            }
            (None, Some(_)) | (Some(_), None) => return Err(FixtureChartError::Invariant),
        };
        Ok(FixtureChartProjection {
            chart: self.chart,
            source_projection: original,
            center: scalars.center,
            rate: scalars.rate,
            center_error: scalars.center_error,
            rate_error: scalars.rate_error,
            tracking,
        })
    }
}

/// Unpublished arithmetic results from the one numerical child, not an owner token.
pub(super) struct ChartScalars {
    /// Signed transformed raw-anchor center seconds.
    pub(super) center: f64,
    /// Signed transformed fitted rate seconds per second.
    pub(super) rate: f64,
    /// Nonnegative anchor transformation roundoff bound in seconds.
    pub(super) center_error: f64,
    /// Nonnegative rate transformation roundoff bound in seconds per second.
    pub(super) rate_error: f64,
    /// Optional transformed fitted phase exactly matching original tracking presence.
    pub(super) tracking: Option<ChartPhase>,
}

/// Unpublished same-delta fitted-phase result and its separate arithmetic bound.
pub(super) struct ChartPhase {
    /// Signed fitted phase at the query time after the common chart displacement.
    pub(super) value: f64,
    /// Nonnegative deterministic new-arithmetic error in seconds.
    pub(super) error: f64,
}

/// All four newly represented base scalars, mandatorily admitted before publication.
#[derive(Validate)]
struct ProjectionCandidate {
    /// Signed center remains finite normal or signed zero.
    #[garde(custom(signed_rule))]
    center: f64,
    /// Signed fitted rate retains the same supported floating representation.
    #[garde(custom(signed_rule))]
    rate: f64,
    /// Outward error must be nonnegative, normal or zero.
    #[garde(custom(bound_rule))]
    center_error: f64,
    /// Rate error has the same nonnegative supported domain.
    #[garde(custom(bound_rule))]
    rate_error: f64,
}

impl ProjectionCandidate {
    /// Invoke mandatory validation and return the first typed scalar cause.
    /// # Errors
    /// Reject center, rate, center error, rate error or Garde disagreement in order.
    fn admit(&self) -> Result<(), FixtureChartError> {
        let validated = self.validate();
        for (value, field) in [
            (self.center, FixtureChartField::AnchorCenter),
            (self.rate, FixtureChartField::Rate),
            (self.center_error, FixtureChartField::CenterRoundoff),
            (self.rate_error, FixtureChartField::RateRoundoff),
        ] {
            if let Some(issue) = scalar_issue(value, field) {
                return Err(FixtureChartError::InvalidScalar { field, issue });
            }
        }
        validated.map_err(|_| FixtureChartError::ValidationInvariant)
    }
}

/// The optional new phase representation; inherited SD/skew keep their old owner.
#[derive(Validate)]
struct TrackingPhaseCandidate {
    /// Signed transformed phase in seconds.
    #[garde(custom(signed_rule))]
    value: f64,
    /// Nonnegative roundoff seconds, distinct from original statistical deviation.
    #[garde(custom(bound_rule))]
    error: f64,
}

impl TrackingPhaseCandidate {
    /// Admit signed phase before its nonnegative arithmetic-bound scalar.
    /// # Errors
    /// Preserve the first typed scalar failure or Garde disagreement, without effects.
    fn admit(&self) -> Result<(), FixtureChartError> {
        let validated = self.validate();
        for (value, field) in [
            (self.value, FixtureChartField::TrackingPhase),
            (self.error, FixtureChartField::TrackingRoundoff),
        ] {
            if let Some(issue) = scalar_issue(value, field) {
                return Err(FixtureChartError::InvalidScalar { field, issue });
            }
        }
        validated.map_err(|_| FixtureChartError::ValidationInvariant)
    }
}

/// Immutable transformed fitted phase with unchanged same-owner SD/skew provenance.
#[derive(Clone, Copy)]
pub struct FixtureChartTracking {
    /// Genuine whole-producer attachment; no caller can separately supply its SD.
    source: FixtureTrackingEstimate,
    /// Newly admitted signed fitted phase at the chart query time.
    phase: f64,
    /// Separate nonnegative numerical bound, never merged into statistical SD.
    phase_error: f64,
}

impl fmt::Debug for FixtureChartTracking {
    /// Redact all fitted values and numerical bounds with fixed bounded text.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureChartTracking { checked: true, .. }")
    }
}

impl FixtureChartTracking {
    /// Return transformed signed fitted phase in seconds without effects.
    #[must_use]
    pub const fn phase_seconds(&self) -> f64 {
        self.phase
    }

    /// Return only the new-arithmetic phase error upper, not physical uncertainty.
    #[must_use]
    pub const fn phase_roundoff_seconds(&self) -> f64 {
        self.phase_error
    }

    /// Return unchanged same-owner statistical phase SD from the original producer.
    #[must_use]
    pub const fn phase_standard_deviation_seconds(&self) -> f64 {
        self.source.phase_standard_deviation_seconds()
    }

    /// Return unchanged same-fit statistical rate SD; no chart uncertainty is added.
    #[must_use]
    pub const fn rate_standard_deviation_seconds_per_second(&self) -> f64 {
        self.source.rate_standard_deviation_seconds_per_second()
    }

    /// Return the original explicitly supplied checked tracking skew unchanged.
    #[must_use]
    pub const fn tracking_skew_seconds_per_second(&self) -> f64 {
        self.source.tracking_skew_seconds_per_second()
    }
}

/// Opaque chart result retaining original provenance and exact descriptor identity.
///
/// Copying preserves the descriptor lifetime and all bounds. There is no legacy
/// conversion: consumers must explicitly use this mathematical chart's geometry.
/// Private output mutation cannot discard its checked numerical relationship:
/// ```compile_fail
/// use time_core::FixtureChartProjection;
/// fn overwrite(mut projection: FixtureChartProjection<'_>) {
///     projection.center = 0.0;
/// }
/// ```
/// A result cannot escape its actual common-reference descriptor:
/// ```compile_fail
/// use time_core::{FixtureAdaptiveHistory, FixtureAffineChart, FixtureChartDomain,
///     FixtureChartError, FixtureChartFrames, FixtureChartOutcome,
///     FixtureChartProjection, FixtureProjectionTerms};
/// use time_domain::MonotonicNs;
/// fn escape<'history>(history: &'history FixtureAdaptiveHistory,
///     frames: FixtureChartFrames, domain: FixtureChartDomain,
///     at: MonotonicNs, terms: FixtureProjectionTerms)
///     -> Result<FixtureChartProjection<'history>, FixtureChartError> {
///     let chart = FixtureAffineChart::new(frames, domain, 0.0, 0.0)?;
///     let Some(view) = history.prepare_chart(history.context(), FixtureChartOutcome::Known(&chart))?
///         else { return Err(FixtureChartError::Invariant); };
///     view.project_anchor(view.context(), 0, at, at, terms)
/// }
/// ```
#[derive(Clone, Copy)]
pub struct FixtureChartProjection<'chart> {
    /// Exact immutable common-reference chart object, retained beyond the view.
    chart: &'chart FixtureAffineChart,
    /// Complete genuine original projection with raw source frame/time provenance.
    source_projection: FixtureAnchorProjection,
    /// Admitted transformed raw-anchor center in seconds.
    center: f64,
    /// Admitted transformed fitted rate in seconds per second.
    rate: f64,
    /// Nonnegative new-arithmetic anchor error in seconds.
    center_error: f64,
    /// Nonnegative new-arithmetic rate error in seconds per second.
    rate_error: f64,
    /// Same-delta fitted phase and unchanged original statistical metadata.
    tracking: Option<FixtureChartTracking>,
}

impl fmt::Debug for FixtureChartProjection<'_> {
    /// Redact source observations, chart values and transformed estimates.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureChartProjection { checked: true, .. }")
    }
}

impl<'chart> FixtureChartProjection<'chart> {
    /// Borrow unchanged original projection provenance, without a target relabel.
    #[must_use]
    pub const fn source_projection(&self) -> &FixtureAnchorProjection {
        &self.source_projection
    }

    /// Borrow the exact descriptor, preserving actual mathematical-map identity.
    #[must_use]
    pub const fn chart(&self) -> &'chart FixtureAffineChart {
        self.chart
    }

    /// Return target mapping assertions with the actual original source identity.
    #[must_use]
    pub const fn context(&self) -> FixtureContext {
        FixtureContext::new(
            self.source_projection.context().source_id(),
            self.chart.frames.target_frame,
            self.chart.frames.target_generation,
        )
    }

    /// Return the admitted signed transformed raw-anchor center in seconds.
    #[must_use]
    pub const fn center_seconds(&self) -> f64 {
        self.center
    }

    /// Return the admitted transformed fitted rate in seconds per second.
    #[must_use]
    pub const fn phase_rate_seconds_per_second(&self) -> f64 {
        self.rate
    }

    /// Return the separate deterministic new-arithmetic center error upper.
    #[must_use]
    pub const fn center_roundoff_seconds(&self) -> f64 {
        self.center_error
    }

    /// Return the separate deterministic new-arithmetic rate error upper.
    #[must_use]
    pub const fn rate_roundoff_seconds_per_second(&self) -> f64 {
        self.rate_error
    }

    /// Borrow same-delta fitted tracking when the actual original producer supplied it.
    #[must_use]
    pub const fn tracking(&self) -> Option<&FixtureChartTracking> {
        self.tracking.as_ref()
    }

    /// Add the numerical anchor error outward to original geometry for Q33 enclosure.
    /// # Errors
    /// Reject an unrepresentable finite upper bound; never reduce or clamp radius.
    pub(crate) fn enclosure_distance_seconds(&self) -> Result<f64, FixtureChartError> {
        roundoff::add_up(self.source_projection.distance_seconds(), self.center_error)
    }
}

/// Independent raw-carrier and closed-error controls, outside production coverage.
#[cfg(test)]
#[path = "../../tests/private/chart_guards.rs"]
mod chart_guards;

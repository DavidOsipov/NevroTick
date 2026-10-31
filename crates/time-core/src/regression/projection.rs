// SPDX-License-Identifier: GPL-2.0-only
//! Pure fixture-anchor projection from one borrowed committed history.
//!
//! The history alone owns the held observation, latest instant and fitted rate.
//! Asserted context and exact anchor time are checked before any arithmetic. Age
//! remains an exact monotonic duration; only its checked difference is converted
//! to positive seconds. Phase reuses the exact Q33 conversion owner. Every
//! binary64 primitive is separately evaluated and checked before another can hide
//! a rejected intermediate. No fused multiply-add or fitted-intercept substitute
//! is used. Supplied distance terms are fixture assertions, never confidence,
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
    ArithmeticOperation, ArithmeticStage, FitError, FixtureContext, FixtureHistory,
    NormalizationComponent, NormalizationError, NormalizationStep, arithmetic,
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
    /// Original immutable supplied terms, including supplied signed-zero bits.
    terms: FixtureProjectionTerms,
    /// Checked separate anchor age times supplied skew, in seconds.
    growth_seconds: f64,
    /// Checked half-delay plus dispersion, then growth, in seconds.
    distance_seconds: f64,
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
    if at < latest_observed_at {
        return Err(FixtureProjectionError::BackwardProjection);
    }
    let anchor_elapsed = at
        .duration_since(anchor_observed_at)
        .map_err(|source| FixtureProjectionError::Domain { source })?;
    let latest_elapsed = at
        .duration_since(latest_observed_at)
        .map_err(|source| FixtureProjectionError::Domain { source })?;
    let anchor_elapsed_seconds = elapsed_seconds(anchor_elapsed)?;
    let phase =
        phase_seconds(anchor).map_err(|source| FixtureProjectionError::Conversion { source })?;
    let rate = report.statistics().line().phase_rate_seconds_per_second();
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
        context: expected_context,
        anchor_observed_at,
        latest_observed_at,
        fit_reference: report.reference(),
        projected_at: at,
        anchor_elapsed,
        latest_elapsed,
        anchor_elapsed_seconds,
        center_seconds,
        terms,
        growth_seconds,
        distance_seconds,
    })
}

/// Raw source-derived guards construct no checked history, terms or projection.
#[cfg(test)]
#[path = "../../tests/private/projection_guards.rs"]
mod projection_guards;

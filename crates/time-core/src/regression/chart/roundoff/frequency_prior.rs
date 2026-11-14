// SPDX-License-Identifier: GPL-2.0-only
/*
 chronyd/chronyc - Programs for keeping computer clocks accurate.

**********************************************************************
* Copyright (C) Richard P. Curnow  1997-2003
* Copyright (C) Miroslav Lichvar  2009-2018, 2020, 2022
* Copyright (C) Andy Fiddaman  2024
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
//! Conditional same-chart frequency fusion and separate arithmetic enclosures.
//!
//! Adapted by David Osipov with AI assistance, 8 November 2026, from Chrony 4.9
//! `reference.c:get_clock_estimates`, revision
//! `8df4f1263e206585e6c4e61352d042c2dd12916a`. The S1.4 Revision 2 contract
//! replaces the assumed already-applied zero residual with an explicit prior,
//! normalizes weights, fixes separate RN operations, and refuses indeterminate
//! weights. These checked numbers grant no observation, clock or baseline right.
//!
//! Points use the existing finite normal-or-zero arithmetic policy. Private
//! scalar endpoint calculations may use subnormals solely for outward bounds;
//! exposed error bounds widen positive subnormals to `MIN_POSITIVE`. All scratch
//! and work are fixed-size scalars, without heap retention, I/O or ambient reads.

use std::fmt;

use garde::Validate;
use time_domain::{ActiveAwakeDurationNs, MonotonicDurationNs, MonotonicNs};

use super::{NANOSECONDS_PER_SECOND, bound_arithmetic, coefficient_addition_error};
use crate::regression::normalization::integer_to_f64;
use crate::regression::{
    ArithmeticIssue, ArithmeticOperation, ArithmeticStage, FitError, FixtureAffineChart,
    FixtureChartBoundOperation, FixtureChartError, NormalizationComponent, NormalizationError,
    arithmetic, checked_square_root, scalar_issue,
};

/// Closed scalar role in frequency-model numerical admission.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureFrequencyNumericField {
    /// A rate mean in positive-behind seconds per monotonic second.
    Mean,
    /// The nonnegative absolute arithmetic error on that rate mean.
    MeanError,
    /// A nonnegative conditional tracking skew, separate from rate SD.
    Skew,
    /// The nonnegative inherited arithmetic error on model skew.
    SkewError,
    /// The new fitted rate minus the fused mean, in rate units.
    Residual,
    /// The nonnegative absolute arithmetic error on the residual.
    ResidualError,
    /// A same-chart positive-behind phase anchor, in seconds.
    Phase,
    /// The nonnegative absolute arithmetic error on phase.
    PhaseError,
    /// Explicit conditional maximum clock-error rate, in rate units.
    MaximumClockError,
    /// Nonnegative nominal added prediction extent, in seconds.
    Growth,
    /// Finite nonnegative outward upper of the exact added extent.
    GrowthUpper,
    /// Current chart displacement gradient, in seconds per monotonic second.
    ChartRate,
    /// Absolute error on the current chart gradient.
    ChartRateError,
    /// Known actual-clock displacement gradient in the same monotonic basis.
    ActualRate,
    /// Absolute error on the actual-clock gradient.
    ActualRateError,
    /// Positive affine active-awake seconds per monotonic second.
    ActiveRatio,
    /// Absolute error on the declared active/monotonic ratio.
    ActiveRatioError,
    /// Known achieved baseline in seconds per active-awake second.
    Baseline,
    /// Actual newly achieved baseline, distinct from a requested value.
    AchievedBaseline,
    /// Explicit positive requested aggregate-rate point cap.
    AggregateCap,
    /// Unclipped desired stable compensation point.
    DesiredBaseline,
    /// Absolute arithmetic/map error on the unclipped desired point.
    DesiredBaselineError,
    /// Policy-clipped requested point, never an achieved baseline.
    RequestedBaseline,
    /// Known displacement at the numeric map's reference instant.
    MapPhase,
    /// Absolute error on that displacement.
    MapPhaseError,
    /// Known gradient removed from a retained model mean.
    RateShift,
    /// Absolute error on the removed gradient.
    RateShiftError,
    /// Constructed affine effect displacement at its common reference.
    EffectPhase,
    /// Absolute error on the affine displacement coefficient.
    EffectPhaseError,
    /// Constructed affine effect gradient.
    EffectRate,
    /// Absolute error on the affine gradient coefficient.
    EffectRateError,
}

/// Closed scalar or enclosure failure, without any rejected value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureFrequencyNumericIssue {
    /// A scalar or endpoint is nonfinite or has no finite outward successor.
    NonFinite,
    /// A nonzero point/error scalar has unsupported subnormal representation.
    Subnormal,
    /// A nonnegative quantity or square-root endpoint was negative.
    Negative,
    /// An interval denominator cannot be bounded strictly above zero.
    NonPositive,
}

/// Actual frequency operation, distinct from phase-debt and chart steps.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureFrequencyNumericStep {
    /// Normalize both skews by their positive maximum.
    Normalize,
    /// Form the normalized quadratic weight and its complement.
    Gain,
    /// Subtract the retained mean from the genuinely new rate.
    Innovation,
    /// Fuse the prior and measured means in the prescribed point order.
    Mean,
    /// Subtract the fused mean from the new rate.
    Residual,
    /// Form the nonnegative scaled skew mixture.
    Mixture,
    /// Form the innovation-based square-root disagreement term.
    Disagreement,
    /// Add mixture and disagreement, carrying inherited skew error.
    Skew,
    /// Convert exact monotonic elapsed nanoseconds to seconds.
    Elapsed,
    /// Project the current mean from its genuine same-chart phase anchor.
    Prediction,
    /// Form conditional added prediction extent and its outward upper.
    Growth,
    /// Subtract one checked actual-total constant phase effect.
    AnchorRebase,
    /// Add chart gradient then subtract the known actual-clock gradient.
    BaselineResidual,
    /// Divide the residual by the independently declared positive active ratio.
    BaselineIncrement,
    /// Add the desired active-rate increment to the old achieved baseline.
    BaselineRequest,
    /// Shift a model mean by one known affine coordinate gradient.
    RateRebase,
    /// Compute the actual baseline change and its monotonic gradient.
    AffineEffectRate,
    /// Compute an affine effect's phase at a separately supplied epoch.
    AffineEffectPhase,
    /// Evaluate and subtract an affine effect at the actual model anchor epoch.
    AffineAnchorRebase,
    /// Check a supplied clock ratio against exact same-segment integer spans.
    ActiveRatioSpan,
    /// Advance the actual-clock displacement to a known acknowledgement epoch.
    MapAdvance,
}

/// Bounded numerical refusal with no rates, phases, timestamps or reports retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureFrequencyNumericError {
    /// A raw or transformed scalar violates its maintained invariant.
    Input {
        /// Fixed role of the rejected quantity.
        field: FixtureFrequencyNumericField,
        /// Unsupported scalar property.
        issue: FixtureFrequencyNumericIssue,
    },
    /// A separately rounded point primitive was unsupported.
    Arithmetic {
        /// Actual point operation being evaluated.
        step: FixtureFrequencyNumericStep,
        /// Existing bounded checked-arithmetic cause.
        source: FitError,
    },
    /// An existing outward coefficient bound was unavailable.
    Bound {
        /// Actual model operation needing the bound.
        step: FixtureFrequencyNumericStep,
        /// Existing bounded coefficient-error cause.
        source: FixtureChartError,
    },
    /// A private outward endpoint cannot represent the required enclosure.
    Enclosure {
        /// Exact model operation whose enclosure was unavailable.
        step: FixtureFrequencyNumericStep,
        /// Closed endpoint property, without a payload.
        issue: FixtureFrequencyNumericIssue,
    },
    /// Exact elapsed nanoseconds exceed the inclusive `2^53` conversion envelope.
    Duration {
        /// Existing bounded integer-conversion cause.
        source: NormalizationError,
    },
    /// Both stored tracking-skew points are zero; no precision is invented.
    IndeterminateWeights,
    /// Inherited skew intervals prevent a strictly positive weight denominator.
    SingularWeights,
    /// Exact integer spans contradict the supplied affine clock-ratio enclosure.
    ActiveRatioMismatch,
    /// A checked chart-domain or transformed-chart invariant refused the effect.
    Chart {
        /// Existing bounded chart cause, without raw numeric values.
        source: FixtureChartError,
    },
    /// Mandatory validation disagreed with the complete maintained rule.
    ValidationInvariant,
}

impl fmt::Display for FixtureFrequencyNumericError {
    /// Format only fixed roles and reason families, never rejected values.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input { field, issue } => {
                write!(formatter, "frequency numeric input: {field:?}/{issue:?}")
            }
            Self::Arithmetic { step, .. } => {
                write!(formatter, "frequency numeric arithmetic: {step:?}")
            }
            Self::Bound { step, .. } | Self::Enclosure { step, .. } => {
                write!(formatter, "frequency numeric enclosure: {step:?}")
            }
            Self::Duration { .. } => formatter.write_str("frequency numeric duration"),
            Self::IndeterminateWeights => formatter.write_str("frequency numeric zero weights"),
            Self::SingularWeights => formatter.write_str("frequency numeric singular enclosure"),
            Self::ActiveRatioMismatch => {
                formatter.write_str("frequency numeric clock-ratio mismatch")
            }
            Self::Chart { .. } => formatter.write_str("frequency numeric chart refusal"),
            Self::ValidationInvariant => {
                formatter.write_str("frequency numeric validation invariant")
            }
        }
    }
}

impl std::error::Error for FixtureFrequencyNumericError {
    /// Expose only the retained bounded arithmetic/conversion cause.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Arithmetic { source, .. } => Some(source),
            Self::Bound { source, .. } | Self::Chart { source } => Some(source),
            Self::Duration { source } => Some(source),
            Self::Input { .. }
            | Self::Enclosure { .. }
            | Self::IndeterminateWeights
            | Self::SingularWeights
            | Self::ActiveRatioMismatch
            | Self::ValidationInvariant => None,
        }
    }
}

/// Immutable numeric model; its owner separately supplies chart and credit custody.
#[derive(Clone, Copy)]
pub(crate) struct FrequencyPrior {
    /// Stored mean, seconds per monotonic second, positive behind.
    mean: f64,
    /// Absolute numerical error on the carried mean.
    mean_error: f64,
    /// Conditional model skew, not a statistical SD or UTC promise.
    skew: f64,
    /// Absolute numerical error on the carried skew.
    skew_error: f64,
    /// Last assimilated fitted rate minus the model mean.
    residual: f64,
    /// Absolute numerical error on the carried residual.
    residual_error: f64,
}

impl fmt::Debug for FrequencyPrior {
    /// Redact all numeric model data, including alternate formatting.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FrequencyPrior { checked: true, .. }")
    }
}

/// Checked same-chart phase point; it owns no source time or chart authority.
#[derive(Clone, Copy)]
pub(crate) struct FrequencyAnchor {
    /// Positive-behind phase seconds at the lifecycle-owned projected instant.
    phase: f64,
    /// Absolute numerical error on the anchor.
    error: f64,
}

impl fmt::Debug for FrequencyAnchor {
    /// Redact the phase and its bound.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FrequencyAnchor { checked: true, .. }")
    }
}

/// Checked conditional prediction with arithmetic and model growth kept distinct.
#[derive(Clone, Copy)]
pub(crate) struct FrequencyPrediction {
    /// Prescribed separately rounded phase point.
    phase: f64,
    /// Absolute deterministic error on that phase point.
    phase_error: f64,
    /// Prescribed nominal added conditional prediction extent.
    growth: f64,
    /// Outward upper enclosing exact conditional growth and the nominal point.
    growth_upper: f64,
}

/// Raw scalar request inputs; the numerical receiving method validates all fields.
#[derive(Clone, Copy)]
pub(crate) struct BaselineOperands {
    /// Current chart gradient in displacement seconds per monotonic second.
    pub(crate) chart_rate: f64,
    /// Absolute arithmetic error on the chart gradient.
    pub(crate) chart_rate_error: f64,
    /// Known actual-clock gradient in the same source/monotonic basis.
    pub(crate) actual_rate: f64,
    /// Absolute error on the known actual-clock gradient.
    pub(crate) actual_rate_error: f64,
    /// Declared active-awake seconds per monotonic second.
    pub(crate) active_ratio: f64,
    /// Absolute error on the declared ratio.
    pub(crate) active_ratio_error: f64,
    /// Old achieved baseline, never the previous requested point.
    pub(crate) baseline: f64,
    /// Explicit existing aggregate point cap; no default is selected.
    pub(crate) cap: f64,
}

/// Checked desired/requested numbers; this carries no acknowledgement authority.
#[derive(Clone, Copy)]
pub(crate) struct BaselineRequest {
    /// Separately rounded unclipped desired rate.
    desired: f64,
    /// Absolute bound on the unclipped mathematical desire.
    desired_error: f64,
    /// Desired point after the declared symmetric policy clamp.
    requested: f64,
    /// Whether the point, rather than its arithmetic enclosure, was clipped.
    clipped: bool,
    /// Positive cap retained for complete transformed-output validation.
    cap: f64,
}

impl fmt::Debug for BaselineRequest {
    /// Keep all model and policy scalars outside diagnostics.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("BaselineRequest { checked: true, .. }")
    }
}

/// Checked coefficients of one declared actual-clock map at a separate epoch.
#[derive(Clone, Copy)]
pub(crate) struct BaselineMapNumbers {
    /// Positive declared active/monotonic ratio.
    ratio: f64,
    /// Absolute arithmetic/map error on that ratio.
    ratio_error: f64,
    /// Absolute actual-clock displacement at the lifecycle-owned reference.
    phase: f64,
    /// Absolute error on that displacement.
    phase_error: f64,
    /// Actual-clock gradient over the admitted affine segment.
    rate: f64,
    /// Absolute error on that gradient.
    rate_error: f64,
}

impl fmt::Debug for BaselineMapNumbers {
    /// Do not expose accepted map values or imply physical calibration.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("BaselineMapNumbers { checked: true, .. }")
    }
}

/// Computed affine effect coefficients; only the checked factory constructs them.
#[derive(Clone, Copy)]
pub(crate) struct AffineEffectCoefficients {
    /// Displacement seconds at the chart common reference.
    phase: f64,
    /// Absolute bound against the exact declared map/rate inputs.
    phase_error: f64,
    /// Displacement seconds per monotonic second.
    rate: f64,
    /// Absolute bound on the exact declared rate change.
    rate_error: f64,
}

/// Raw achieved-change inputs, checked before arithmetic and without retained state.
#[derive(Clone, Copy)]
struct BaselineChange {
    /// Declared active/monotonic ratio.
    ratio: f64,
    /// Absolute error on the ratio.
    error: f64,
    /// Previous known achieved baseline.
    old: f64,
    /// Newly known achieved baseline.
    actual: f64,
}

/// One already checked point/range pair for a known baseline-gradient change.
struct RateChange {
    /// Prescribed separately rounded gradient point.
    point: f64,
    /// Outward enclosure of its exact declared operands.
    range: Interval,
}

impl fmt::Debug for FrequencyPrediction {
    /// Redact prediction quantities without implying physical accuracy.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FrequencyPrediction { checked: true, .. }")
    }
}

/// One fresh rate and its distinct inherited error/skew inputs.
#[derive(Clone, Copy)]
struct Sample {
    /// Nominal rate mean from the genuine received estimate.
    rate: f64,
    /// Inherited deterministic error on that mean.
    error: f64,
    /// Nominal conditional tracking skew, not rate SD.
    skew: f64,
}

/// Check complete scalar support without retaining the rejected value.
/// # Errors
/// Return the named field's nonfinite/subnormal rejection.
fn supported(
    value: f64,
    field: FixtureFrequencyNumericField,
) -> Result<(), FixtureFrequencyNumericError> {
    let issue = match scalar_issue(value) {
        None => return Ok(()),
        Some(ArithmeticIssue::NonFinite) => FixtureFrequencyNumericIssue::NonFinite,
        Some(ArithmeticIssue::Subnormal) => FixtureFrequencyNumericIssue::Subnormal,
        Some(_) => return Err(FixtureFrequencyNumericError::ValidationInvariant),
    };
    Err(FixtureFrequencyNumericError::Input { field, issue })
}

/// Admit a supported nonnegative scalar, retaining either zero's input bits.
/// # Errors
/// Reject scalar support first, then a negative normal quantity.
fn nonnegative(
    value: f64,
    field: FixtureFrequencyNumericField,
) -> Result<(), FixtureFrequencyNumericError> {
    supported(value, field)?;
    if value < 0.0 {
        return Err(FixtureFrequencyNumericError::Input {
            field,
            issue: FixtureFrequencyNumericIssue::Negative,
        });
    }
    Ok(())
}

/// Check every fresh sample field in constructor argument order.
/// # Errors
/// Preserve the first rate/error/skew invariant violation.
fn sample_rule(value: &Sample) -> Result<(), FixtureFrequencyNumericError> {
    supported(value.rate, FixtureFrequencyNumericField::Mean)?;
    nonnegative(value.error, FixtureFrequencyNumericField::MeanError)?;
    nonnegative(value.skew, FixtureFrequencyNumericField::Skew)
}

/// Check all carried model point/error fields before transformed publication.
/// # Errors
/// Preserve the first complete-field invariant rejection.
fn prior_rule(value: &FrequencyPrior) -> Result<(), FixtureFrequencyNumericError> {
    supported(value.mean, FixtureFrequencyNumericField::Mean)?;
    nonnegative(value.mean_error, FixtureFrequencyNumericField::MeanError)?;
    nonnegative(value.skew, FixtureFrequencyNumericField::Skew)?;
    nonnegative(value.skew_error, FixtureFrequencyNumericField::SkewError)?;
    supported(value.residual, FixtureFrequencyNumericField::Residual)?;
    nonnegative(
        value.residual_error,
        FixtureFrequencyNumericField::ResidualError,
    )
}

/// Check both anchor fields; identity/time relationships belong to its owner.
/// # Errors
/// Reject unsupported phase before an unsupported or negative error.
fn anchor_rule(value: &FrequencyAnchor) -> Result<(), FixtureFrequencyNumericError> {
    supported(value.phase, FixtureFrequencyNumericField::Phase)?;
    nonnegative(value.error, FixtureFrequencyNumericField::PhaseError)
}

/// Check every prediction field and its point/upper relationship.
/// # Errors
/// Reject unsupported fields or a purported upper below its nominal point.
fn prediction_rule(value: &FrequencyPrediction) -> Result<(), FixtureFrequencyNumericError> {
    supported(value.phase, FixtureFrequencyNumericField::Phase)?;
    nonnegative(value.phase_error, FixtureFrequencyNumericField::PhaseError)?;
    nonnegative(value.growth, FixtureFrequencyNumericField::Growth)?;
    nonnegative(
        value.growth_upper,
        FixtureFrequencyNumericField::GrowthUpper,
    )?;
    if value.growth_upper < value.growth {
        return Err(FixtureFrequencyNumericError::ValidationInvariant);
    }
    Ok(())
}

/// Check a declared ratio and require its complete enclosure to be positive.
/// # Errors
/// Preserve scalar/error support before rejecting a singular ratio interval.
fn ratio_rule((ratio, error): &(f64, f64)) -> Result<(), FixtureFrequencyNumericError> {
    supported(*ratio, FixtureFrequencyNumericField::ActiveRatio)?;
    nonnegative(*error, FixtureFrequencyNumericField::ActiveRatioError)?;
    if *ratio <= 0.0 {
        return Err(FixtureFrequencyNumericError::Input {
            field: FixtureFrequencyNumericField::ActiveRatio,
            issue: FixtureFrequencyNumericIssue::NonPositive,
        });
    }
    if Interval::around(
        *ratio,
        *error,
        FixtureFrequencyNumericStep::BaselineIncrement,
    )?
    .low <= 0.0
    {
        return Err(enclosure(
            FixtureFrequencyNumericStep::BaselineIncrement,
            FixtureFrequencyNumericIssue::NonPositive,
        ));
    }
    Ok(())
}

/// Check every raw request scalar; clipping policy is separate from model error.
/// # Errors
/// Preserve the first rate/error/ratio/baseline/cap refusal.
fn baseline_operands_rule(value: &BaselineOperands) -> Result<(), FixtureFrequencyNumericError> {
    supported(value.chart_rate, FixtureFrequencyNumericField::ChartRate)?;
    nonnegative(
        value.chart_rate_error,
        FixtureFrequencyNumericField::ChartRateError,
    )?;
    supported(value.actual_rate, FixtureFrequencyNumericField::ActualRate)?;
    nonnegative(
        value.actual_rate_error,
        FixtureFrequencyNumericField::ActualRateError,
    )?;
    ratio_rule(&(value.active_ratio, value.active_ratio_error))?;
    supported(value.baseline, FixtureFrequencyNumericField::Baseline)?;
    supported(value.cap, FixtureFrequencyNumericField::AggregateCap)?;
    if value.cap <= 0.0 {
        return Err(FixtureFrequencyNumericError::Input {
            field: FixtureFrequencyNumericField::AggregateCap,
            issue: FixtureFrequencyNumericIssue::NonPositive,
        });
    }
    Ok(())
}

/// Check the desired error and exact point-policy relationship before exposure.
/// # Errors
/// Reject unsupported output fields or a changed clamp/credit projection.
fn baseline_request_rule(value: &BaselineRequest) -> Result<(), FixtureFrequencyNumericError> {
    supported(value.desired, FixtureFrequencyNumericField::DesiredBaseline)?;
    nonnegative(
        value.desired_error,
        FixtureFrequencyNumericField::DesiredBaselineError,
    )?;
    supported(
        value.requested,
        FixtureFrequencyNumericField::RequestedBaseline,
    )?;
    supported(value.cap, FixtureFrequencyNumericField::AggregateCap)?;
    if value.cap <= 0.0
        || value.clipped != (value.desired < -value.cap || value.desired > value.cap)
        || value.requested.to_bits() != value.desired.clamp(-value.cap, value.cap).to_bits()
    {
        return Err(FixtureFrequencyNumericError::ValidationInvariant);
    }
    Ok(())
}

/// Check all retained declared map coefficients and their separate errors.
/// # Errors
/// Preserve positive-ratio, phase, rate and complete-bound refusals.
fn map_rule(value: &BaselineMapNumbers) -> Result<(), FixtureFrequencyNumericError> {
    ratio_rule(&(value.ratio, value.ratio_error))?;
    supported(value.phase, FixtureFrequencyNumericField::MapPhase)?;
    nonnegative(
        value.phase_error,
        FixtureFrequencyNumericField::MapPhaseError,
    )?;
    supported(value.rate, FixtureFrequencyNumericField::ActualRate)?;
    nonnegative(
        value.rate_error,
        FixtureFrequencyNumericField::ActualRateError,
    )
}

/// Check one known gradient shift; actual effect identity belongs to its receiver.
/// # Errors
/// Preserve unsupported delta/error before any prior or map transformation.
fn shift_rule((delta, error): &(f64, f64)) -> Result<(), FixtureFrequencyNumericError> {
    supported(*delta, FixtureFrequencyNumericField::RateShift)?;
    nonnegative(*error, FixtureFrequencyNumericField::RateShiftError)
}

/// Check the ratio and both achieved points without interpreting a desired rate.
/// # Errors
/// Preserve ratio support and the old/new achieved-baseline scalar causes.
fn change_rule(value: &BaselineChange) -> Result<(), FixtureFrequencyNumericError> {
    ratio_rule(&(value.ratio, value.error))?;
    supported(value.old, FixtureFrequencyNumericField::Baseline)?;
    supported(value.actual, FixtureFrequencyNumericField::AchievedBaseline)
}

/// Check all computed coefficient points and nonnegative errors together.
/// # Errors
/// Reject unsupported affine point/error outputs before a chart can retain them.
fn affine_effect_rule(
    value: &AffineEffectCoefficients,
) -> Result<(), FixtureFrequencyNumericError> {
    supported(value.phase, FixtureFrequencyNumericField::EffectPhase)?;
    nonnegative(
        value.phase_error,
        FixtureFrequencyNumericField::EffectPhaseError,
    )?;
    supported(value.rate, FixtureFrequencyNumericField::EffectRate)?;
    nonnegative(
        value.rate_error,
        FixtureFrequencyNumericField::EffectRateError,
    )
}

#[derive(Validate)]
/// Mandatory complete raw baseline-request input admission.
struct RawBaselineOperands {
    #[garde(custom(|value: &BaselineOperands, (): &()| baseline_operands_rule(value).map_err(|_| garde::Error::new("invalid baseline operands"))))]
    /// One maintained rule covers every raw scalar and ratio relation.
    value: BaselineOperands,
}

#[derive(Validate)]
/// Mandatory desired/requested output admission, including the policy projection.
struct RawBaselineRequest {
    #[garde(custom(|value: &BaselineRequest, (): &()| baseline_request_rule(value).map_err(|_| garde::Error::new("invalid baseline request"))))]
    /// One maintained rule owns all numeric fields, clipping and cap.
    value: BaselineRequest,
}

#[derive(Validate)]
/// Mandatory declared-clock-map admission before construction or advancement.
struct RawBaselineMap {
    #[garde(custom(|value: &BaselineMapNumbers, (): &()| map_rule(value).map_err(|_| garde::Error::new("invalid baseline map"))))]
    /// Typed numeric relationship; this grants no physical mapping authority.
    value: BaselineMapNumbers,
}

#[derive(Validate)]
/// Mandatory raw ratio admission for exact-span consistency checks.
struct RawRatio {
    #[garde(custom(|value: &(f64, f64), (): &()| ratio_rule(value).map_err(|_| garde::Error::new("invalid active ratio"))))]
    /// Point/error pair checked together, including a positive lower bound.
    value: (f64, f64),
}

#[derive(Validate)]
/// Mandatory known gradient-shift input admission.
struct RawShift {
    #[garde(custom(|value: &(f64, f64), (): &()| shift_rule(value).map_err(|_| garde::Error::new("invalid rate shift"))))]
    /// Separate immutable delta point and error.
    value: (f64, f64),
}

#[derive(Validate)]
/// Mandatory actual baseline-change admission shared by map and chart effects.
struct RawBaselineChange {
    #[garde(custom(|value: &BaselineChange, (): &()| change_rule(value).map_err(|_| garde::Error::new("invalid achieved baseline change"))))]
    /// The rule covers both achieved rates and the full ratio enclosure.
    value: BaselineChange,
}

#[derive(Validate)]
/// Mandatory computed affine coefficient admission before returning any getter.
struct RawAffineEffect {
    #[garde(custom(|value: &AffineEffectCoefficients, (): &()| affine_effect_rule(value).map_err(|_| garde::Error::new("invalid affine rate effect"))))]
    /// Complete point/error coefficient pair; no bound is reset to zero.
    value: AffineEffectCoefficients,
}

#[derive(Validate)]
/// Mandatory complete-field sample admission; no field is skipped.
struct RawSample {
    #[garde(custom(|value: &Sample, (): &()| sample_rule(value).map_err(|_| garde::Error::new("invalid frequency sample"))))]
    /// The maintained rule owns all three sample fields.
    value: Sample,
}

#[derive(Validate)]
/// Mandatory complete-field model admission after construction or transformation.
struct RawPrior {
    #[garde(custom(|value: &FrequencyPrior, (): &()| prior_rule(value).map_err(|_| garde::Error::new("invalid frequency prior"))))]
    /// The maintained rule owns all six immutable model fields.
    value: FrequencyPrior,
}

#[derive(Validate)]
/// Mandatory phase-anchor input and transformed-output admission.
struct RawAnchor {
    #[garde(custom(|value: &FrequencyAnchor, (): &()| anchor_rule(value).map_err(|_| garde::Error::new("invalid frequency anchor"))))]
    /// The maintained rule owns both anchor fields.
    value: FrequencyAnchor,
}

#[derive(Validate)]
/// Mandatory complete-field prediction admission before any getter is exposed.
struct RawPrediction {
    #[garde(custom(|value: &FrequencyPrediction, (): &()| prediction_rule(value).map_err(|_| garde::Error::new("invalid frequency prediction"))))]
    /// The maintained rule also owns the point/upper relationship.
    value: FrequencyPrediction,
}

#[derive(Validate)]
/// Mandatory admission of the separately supplied conditional clock-error term.
struct RawMaximum {
    #[garde(custom(|value: &f64, (): &()| nonnegative(*value, FixtureFrequencyNumericField::MaximumClockError).map_err(|_| garde::Error::new("invalid clock error rate"))))]
    /// Explicit model input, without a default or calibration claim.
    value: f64,
}

impl Sample {
    /// Admit a raw fresh sample with mandatory shared validation.
    /// # Errors
    /// Preserve its exact scalar cause or validator disagreement.
    fn new(rate: f64, error: f64, skew: f64) -> Result<Self, FixtureFrequencyNumericError> {
        let raw = RawSample {
            value: Self { rate, error, skew },
        };
        let validated = raw.validate();
        sample_rule(&raw.value)?;
        validated.map_err(|_| FixtureFrequencyNumericError::ValidationInvariant)?;
        Ok(raw.value)
    }
}

impl FrequencyPrior {
    /// Initialize from one received rate, its inherited error and nominal skew.
    /// The lifecycle owns genuine intake/eligibility. Correlated `f-f` is exactly
    /// zero; its error is zero even when the same fitted mean has inherited error.
    /// # Errors
    /// Reject raw support or mandatory transformed-output admission.
    pub(crate) fn initialize(
        rate: f64,
        rate_error: f64,
        skew: f64,
    ) -> Result<Self, FixtureFrequencyNumericError> {
        let sample = Sample::new(rate, rate_error, skew)?;
        let residual = point(
            sample.rate,
            sample.rate,
            ArithmeticOperation::Subtract,
            FixtureFrequencyNumericStep::Residual,
        )?;
        Self {
            mean: sample.rate,
            mean_error: sample.error,
            skew: sample.skew,
            skew_error: 0.0,
            residual,
            residual_error: 0.0,
        }
        .admit()
    }

    /// Fuse one fresh rate in the exact normalized RN order, without mutation.
    /// The lifecycle must gate skew, chart, evidence credit and current state.
    /// Inherited mean/skew intervals and the fresh mean error remain enclosed.
    /// # Errors
    /// Reject fresh fields, zero weights, unsupported points or enclosures;
    /// this immutable prior and all caller-owned committed facts stay unchanged.
    pub(crate) fn assimilate(
        &self,
        rate: f64,
        rate_error: f64,
        skew: f64,
    ) -> Result<Self, FixtureFrequencyNumericError> {
        let sample = Sample::new(rate, rate_error, skew)?;
        if self.skew == 0.0 && sample.skew == 0.0 {
            return Err(FixtureFrequencyNumericError::IndeterminateWeights);
        }
        let points = update_points(self, sample)?;
        let ranges = update_intervals(self, sample)?;
        Self {
            mean: points.mean,
            mean_error: ranges
                .mean
                .error(points.mean, FixtureFrequencyNumericStep::Mean)?,
            skew: points.skew,
            skew_error: ranges
                .skew
                .error(points.skew, FixtureFrequencyNumericStep::Skew)?,
            residual: points.residual,
            residual_error: ranges
                .residual
                .error(points.residual, FixtureFrequencyNumericStep::Residual)?,
        }
        .admit()
    }

    /// Predict from the current genuine anchor using typed monotonic duration.
    /// Added growth is conditional extra extent, never a total UTC error bound.
    /// # Errors
    /// Reject the explicit error-rate term, exact-duration conversion, points,
    /// inherited enclosures or transformed outputs; no input is mutated.
    pub(crate) fn project(
        &self,
        anchor: &FrequencyAnchor,
        elapsed: MonotonicDurationNs,
        maximum_clock_error_rate: f64,
    ) -> Result<FrequencyPrediction, FixtureFrequencyNumericError> {
        let raw = RawMaximum {
            value: maximum_clock_error_rate,
        };
        let validated = raw.validate();
        nonnegative(raw.value, FixtureFrequencyNumericField::MaximumClockError)?;
        validated.map_err(|_| FixtureFrequencyNumericError::ValidationInvariant)?;
        let (seconds, time) = seconds_interval(elapsed)?;
        let phase = point(
            anchor.phase,
            point(
                self.mean,
                seconds,
                ArithmeticOperation::Multiply,
                FixtureFrequencyNumericStep::Prediction,
            )?,
            ArithmeticOperation::Add,
            FixtureFrequencyNumericStep::Prediction,
        )?;
        let growth_rate = point(
            point(
                self.skew,
                self.residual.abs(),
                ArithmeticOperation::Add,
                FixtureFrequencyNumericStep::Growth,
            )?,
            raw.value,
            ArithmeticOperation::Add,
            FixtureFrequencyNumericStep::Growth,
        )?;
        let growth = point(
            seconds,
            growth_rate,
            ArithmeticOperation::Multiply,
            FixtureFrequencyNumericStep::Growth,
        )?;
        let phase_range = Interval::around(
            anchor.phase,
            anchor.error,
            FixtureFrequencyNumericStep::Prediction,
        )?
        .add(
            Interval::around(
                self.mean,
                self.mean_error,
                FixtureFrequencyNumericStep::Prediction,
            )?
            .multiply(time, FixtureFrequencyNumericStep::Prediction)?,
            FixtureFrequencyNumericStep::Prediction,
        )?;
        let growth_range = growth_interval(self, raw.value, time)?;
        let phase_error = if elapsed.nanoseconds() == 0 {
            anchor.error
        } else {
            phase_range.error(phase, FixtureFrequencyNumericStep::Prediction)?
        };
        FrequencyPrediction {
            phase,
            phase_error,
            growth,
            growth_upper: upper_nonnegative(
                growth_range.high.max(growth),
                FixtureFrequencyNumericStep::Growth,
            )?,
        }
        .admit()
    }

    /// Require the maintained complete invariant and its mandatory validator.
    /// # Errors
    /// Preserve transformed-field rejection before a validator disagreement.
    fn admit(self) -> Result<Self, FixtureFrequencyNumericError> {
        let raw = RawPrior { value: self };
        let validated = raw.validate();
        prior_rule(&raw.value)?;
        validated.map_err(|_| FixtureFrequencyNumericError::ValidationInvariant)?;
        Ok(raw.value)
    }

    /// Copy the conditional rate mean in monotonic-chart units.
    #[must_use]
    pub(crate) const fn mean(&self) -> f64 {
        self.mean
    }
    /// Copy its inherited and new absolute arithmetic error.
    #[must_use]
    pub(crate) const fn mean_error(&self) -> f64 {
        self.mean_error
    }
    /// Copy conditional skew separately from statistical rate SD.
    #[must_use]
    pub(crate) const fn skew(&self) -> f64 {
        self.skew
    }
    /// Copy the carried absolute arithmetic error on skew.
    #[must_use]
    pub(crate) const fn skew_error(&self) -> f64 {
        self.skew_error
    }
    /// Copy the last assimilated fitted rate minus the fused mean.
    #[must_use]
    pub(crate) const fn residual(&self) -> f64 {
        self.residual
    }
    /// Copy the carried absolute arithmetic error on residual.
    #[must_use]
    pub(crate) const fn residual_error(&self) -> f64 {
        self.residual_error
    }
}

impl FrequencyAnchor {
    /// Admit a same-chart phase point and its inherited numerical error.
    /// # Errors
    /// Reject either field or mandatory-validation disagreement.
    pub(crate) fn new(phase: f64, error: f64) -> Result<Self, FixtureFrequencyNumericError> {
        Self { phase, error }.admit()
    }

    /// Subtract one actual-total constant phase effect and carry both errors.
    /// The lifecycle must establish actual effect identity and zero rate slope.
    /// # Errors
    /// Reject effect support, subtraction, bound overflow or output admission;
    /// the previous anchor and caller-owned delivery facts remain unchanged.
    pub(crate) fn rebase(
        &self,
        effect_phase: f64,
        effect_error: f64,
    ) -> Result<Self, FixtureFrequencyNumericError> {
        let effect = Self::new(effect_phase, effect_error)?;
        let phase = point(
            self.phase,
            effect.phase,
            ArithmeticOperation::Subtract,
            FixtureFrequencyNumericStep::AnchorRebase,
        )?;
        let error = if effect.phase == 0.0 && effect.error == 0.0 {
            self.error
        } else {
            coefficient_addition_error(self.phase, self.error, -effect.phase, effect.error)
                .map_err(|source| FixtureFrequencyNumericError::Bound {
                    step: FixtureFrequencyNumericStep::AnchorRebase,
                    source,
                })?
        };
        Self { phase, error }.admit()
    }

    /// Check both transformed fields through their mandatory shared owner.
    /// # Errors
    /// Preserve the scalar cause or validator disagreement.
    fn admit(self) -> Result<Self, FixtureFrequencyNumericError> {
        let raw = RawAnchor { value: self };
        let validated = raw.validate();
        anchor_rule(&raw.value)?;
        validated.map_err(|_| FixtureFrequencyNumericError::ValidationInvariant)?;
        Ok(raw.value)
    }

    /// Copy the checked phase point, in positive-behind seconds.
    #[must_use]
    pub(crate) const fn phase(&self) -> f64 {
        self.phase
    }
    /// Copy its nonnegative absolute arithmetic error.
    #[must_use]
    pub(crate) const fn error(&self) -> f64 {
        self.error
    }
}

impl FrequencyPrediction {
    /// Check all derived fields and the upper/point relationship.
    /// # Errors
    /// Reject transformed scalars or mandatory-validation disagreement.
    fn admit(self) -> Result<Self, FixtureFrequencyNumericError> {
        let raw = RawPrediction { value: self };
        let validated = raw.validate();
        prediction_rule(&raw.value)?;
        validated.map_err(|_| FixtureFrequencyNumericError::ValidationInvariant)?;
        Ok(raw.value)
    }

    /// Copy nominal phase, distinct from model extent or statistical SD.
    #[must_use]
    pub(crate) const fn phase(&self) -> f64 {
        self.phase
    }
    /// Copy deterministic phase arithmetic error, including inherited inputs.
    #[must_use]
    pub(crate) const fn phase_error(&self) -> f64 {
        self.phase_error
    }
    /// Copy nominal conditional added prediction extent.
    #[must_use]
    pub(crate) const fn growth(&self) -> f64 {
        self.growth
    }
    /// Copy its outward upper, used for the lifecycle's explicit growth cap.
    #[must_use]
    pub(crate) const fn growth_upper(&self) -> f64 {
        self.growth_upper
    }
}

/// Unpublished point-tree results; every component uses checked scalar arithmetic.
struct PriorPoints {
    /// Prescribed fused mean point.
    mean: f64,
    /// Prescribed mixture-plus-disagreement point.
    skew: f64,
    /// Prescribed new rate minus fused mean.
    residual: f64,
}

/// Checked normalized point-tree operands, with no reference/clock authority.
struct PointWeights {
    /// Positive maximum of the two stored skews.
    scale: f64,
    /// Separately rounded prior skew divided by that maximum.
    old: f64,
    /// Separately rounded new skew divided by that maximum.
    fresh: f64,
    /// Separately rounded normalized quadratic gain.
    gain: f64,
    /// Separately rounded one minus that gain.
    complement: f64,
}

/// Evaluate the exact chosen normalized gain point subtree.
/// # Errors
/// Reject unsupported division/quadratics instead of inventing zero precision.
fn point_weights(old: f64, fresh: f64) -> Result<PointWeights, FixtureFrequencyNumericError> {
    let scale = old.max(fresh);
    let x = point(
        old,
        scale,
        ArithmeticOperation::Divide,
        FixtureFrequencyNumericStep::Normalize,
    )?;
    let y = point(
        fresh,
        scale,
        ArithmeticOperation::Divide,
        FixtureFrequencyNumericStep::Normalize,
    )?;
    let a = point(
        3.0,
        point(
            x,
            x,
            ArithmeticOperation::Multiply,
            FixtureFrequencyNumericStep::Gain,
        )?,
        ArithmeticOperation::Multiply,
        FixtureFrequencyNumericStep::Gain,
    )?;
    let d = point(
        a,
        point(
            y,
            y,
            ArithmeticOperation::Multiply,
            FixtureFrequencyNumericStep::Gain,
        )?,
        ArithmeticOperation::Add,
        FixtureFrequencyNumericStep::Gain,
    )?;
    let gain = point(
        a,
        d,
        ArithmeticOperation::Divide,
        FixtureFrequencyNumericStep::Gain,
    )?;
    let complement = point(
        1.0,
        gain,
        ArithmeticOperation::Subtract,
        FixtureFrequencyNumericStep::Gain,
    )?;
    Ok(PointWeights {
        scale,
        old: x,
        fresh: y,
        gain,
        complement,
    })
}

/// Evaluate exactly the accepted normalized point tree, with no fused operations.
/// # Errors
/// Preserve the first unsupported normalized/mean/skew point primitive.
fn update_points(
    prior: &FrequencyPrior,
    sample: Sample,
) -> Result<PriorPoints, FixtureFrequencyNumericError> {
    let weights = point_weights(prior.skew, sample.skew)?;
    let g = weights.gain;
    let c = weights.complement;
    let delta = point(
        sample.rate,
        prior.mean,
        ArithmeticOperation::Subtract,
        FixtureFrequencyNumericStep::Innovation,
    )?;
    let mean = point(
        prior.mean,
        point(
            g,
            delta,
            ArithmeticOperation::Multiply,
            FixtureFrequencyNumericStep::Mean,
        )?,
        ArithmeticOperation::Add,
        FixtureFrequencyNumericStep::Mean,
    )?;
    let residual = point(
        sample.rate,
        mean,
        ArithmeticOperation::Subtract,
        FixtureFrequencyNumericStep::Residual,
    )?;
    let mixture = point(
        point(
            c,
            weights.old,
            ArithmeticOperation::Multiply,
            FixtureFrequencyNumericStep::Mixture,
        )?,
        point(
            g,
            weights.fresh,
            ArithmeticOperation::Multiply,
            FixtureFrequencyNumericStep::Mixture,
        )?,
        ArithmeticOperation::Add,
        FixtureFrequencyNumericStep::Mixture,
    )?;
    let base = point(
        weights.scale,
        mixture,
        ArithmeticOperation::Multiply,
        FixtureFrequencyNumericStep::Mixture,
    )?;
    let radicand = point(
        g,
        c,
        ArithmeticOperation::Multiply,
        FixtureFrequencyNumericStep::Disagreement,
    )?;
    let root = checked_square_root(radicand, ArithmeticStage::Coefficients).map_err(|source| {
        FixtureFrequencyNumericError::Arithmetic {
            step: FixtureFrequencyNumericStep::Disagreement,
            source,
        }
    })?;
    let extra = point(
        delta.abs(),
        root,
        ArithmeticOperation::Multiply,
        FixtureFrequencyNumericStep::Disagreement,
    )?;
    let skew = point(
        base,
        extra,
        ArithmeticOperation::Add,
        FixtureFrequencyNumericStep::Skew,
    )?;
    Ok(PriorPoints {
        mean,
        skew,
        residual,
    })
}

/// Three closed enclosures of the exact real fusion law, not of point intermediates.
struct PriorIntervals {
    /// Enclosure of the exact fused mean under inherited input intervals.
    mean: Interval,
    /// Enclosure of the exact conditional mixture and disagreement.
    skew: Interval,
    /// Enclosure of the exact last fitted rate minus fused mean.
    residual: Interval,
}

/// Enclose the inherited real model law using a fixed positive normalization scale.
/// # Errors
/// Refuse singular weight support or unavailable finite outward endpoints.
fn update_intervals(
    prior: &FrequencyPrior,
    sample: Sample,
) -> Result<PriorIntervals, FixtureFrequencyNumericError> {
    let scale = Interval::point(prior.skew.max(sample.skew));
    let old_skew = Interval::around(
        prior.skew,
        prior.skew_error,
        FixtureFrequencyNumericStep::Normalize,
    )?
    .nonnegative();
    let fresh_skew = Interval::point(sample.skew);
    let g = gain_interval(old_skew, fresh_skew, scale)?;
    let c = Interval::ONE
        .subtract(g, FixtureFrequencyNumericStep::Gain)?
        .unit();
    let p = Interval::around(
        prior.mean,
        prior.mean_error,
        FixtureFrequencyNumericStep::Mean,
    )?;
    let f = Interval::around(sample.rate, sample.error, FixtureFrequencyNumericStep::Mean)?;
    let delta = f.subtract(p, FixtureFrequencyNumericStep::Innovation)?;
    let mean = p.add(
        g.multiply(delta, FixtureFrequencyNumericStep::Mean)?,
        FixtureFrequencyNumericStep::Mean,
    )?;
    let residual = f.subtract(mean, FixtureFrequencyNumericStep::Residual)?;
    let base = c
        .multiply(old_skew, FixtureFrequencyNumericStep::Mixture)?
        .add(
            g.multiply(fresh_skew, FixtureFrequencyNumericStep::Mixture)?,
            FixtureFrequencyNumericStep::Mixture,
        )?
        .nonnegative();
    let root = g
        .multiply(c, FixtureFrequencyNumericStep::Disagreement)?
        .nonnegative()
        .square_root(FixtureFrequencyNumericStep::Disagreement)?;
    let extra = delta
        .absolute()
        .multiply(root, FixtureFrequencyNumericStep::Disagreement)?
        .nonnegative();
    let skew = base
        .add(extra, FixtureFrequencyNumericStep::Skew)?
        .nonnegative();
    Ok(PriorIntervals {
        mean,
        skew,
        residual,
    })
}

/// Bound gain for nonnegative inherited skews; exact one-zero cases stay exact.
/// # Errors
/// Refuse an interval denominator with no positive lower or endpoint overflow.
fn gain_interval(
    old: Interval,
    fresh: Interval,
    scale: Interval,
) -> Result<Interval, FixtureFrequencyNumericError> {
    if old.is_zero() && fresh.low > 0.0 {
        return Ok(Interval::ZERO);
    }
    if fresh.is_zero() {
        if old.low <= 0.0 {
            return Err(FixtureFrequencyNumericError::SingularWeights);
        }
        return Ok(Interval::ONE);
    }
    let x = old
        .divide(scale, FixtureFrequencyNumericStep::Normalize)?
        .nonnegative();
    let y = fresh
        .divide(scale, FixtureFrequencyNumericStep::Normalize)?
        .nonnegative();
    let a = Interval::point(3.0)
        .multiply(
            x.multiply(x, FixtureFrequencyNumericStep::Gain)?,
            FixtureFrequencyNumericStep::Gain,
        )?
        .nonnegative();
    let d = a
        .add(
            y.multiply(y, FixtureFrequencyNumericStep::Gain)?,
            FixtureFrequencyNumericStep::Gain,
        )?
        .nonnegative();
    if d.low <= 0.0 {
        return Err(FixtureFrequencyNumericError::SingularWeights);
    }
    Ok(a.divide(d, FixtureFrequencyNumericStep::Gain)?.unit())
}

/// Convert exact monotonic elapsed nanoseconds, enclosing the rational division.
/// # Errors
/// Reject the inclusive integer envelope, scalar division or finite interval.
fn seconds_interval(
    duration: MonotonicDurationNs,
) -> Result<(f64, Interval), FixtureFrequencyNumericError> {
    let nanos = integer_to_f64(duration.nanoseconds(), NormalizationComponent::Time)
        .map_err(|source| FixtureFrequencyNumericError::Duration { source })?;
    let seconds = point(
        nanos,
        NANOSECONDS_PER_SECOND,
        ArithmeticOperation::Divide,
        FixtureFrequencyNumericStep::Elapsed,
    )?;
    let range = Interval::point(nanos)
        .divide(
            Interval::point(NANOSECONDS_PER_SECOND),
            FixtureFrequencyNumericStep::Elapsed,
        )?
        .nonnegative();
    Ok((seconds, range))
}

/// Enclose conditional growth, carrying skew/residual and exact-duration errors.
/// # Errors
/// Reject any unavailable finite interval while leaving all model facts intact.
fn growth_interval(
    prior: &FrequencyPrior,
    maximum: f64,
    time: Interval,
) -> Result<Interval, FixtureFrequencyNumericError> {
    let skew = Interval::around(
        prior.skew,
        prior.skew_error,
        FixtureFrequencyNumericStep::Growth,
    )?
    .nonnegative();
    let residual = Interval::around(
        prior.residual,
        prior.residual_error,
        FixtureFrequencyNumericStep::Growth,
    )?
    .absolute();
    let rate = skew
        .add(residual, FixtureFrequencyNumericStep::Growth)?
        .add(
            Interval::point(maximum),
            FixtureFrequencyNumericStep::Growth,
        )?
        .nonnegative();
    Ok(time
        .multiply(rate, FixtureFrequencyNumericStep::Growth)?
        .nonnegative())
}

/// Evaluate one separately rounded point through the existing checked owner.
/// # Errors
/// Preserve its exact scalar-support/arithmetic cause at the actual new step.
fn point(
    left: f64,
    right: f64,
    operation: ArithmeticOperation,
    step: FixtureFrequencyNumericStep,
) -> Result<f64, FixtureFrequencyNumericError> {
    arithmetic(left, right, operation, ArithmeticStage::Coefficients)
        .map_err(|source| FixtureFrequencyNumericError::Arithmetic { step, source })
}

/// Construct a closed endpoint refusal without storing the offending endpoint.
const fn enclosure(
    step: FixtureFrequencyNumericStep,
    issue: FixtureFrequencyNumericIssue,
) -> FixtureFrequencyNumericError {
    FixtureFrequencyNumericError::Enclosure { step, issue }
}

/// Round a finite endpoint strictly outward, permitting bound-only subnormals.
/// # Errors
/// Refuse nonfinite input or a missing finite outward successor.
fn outward(
    value: f64,
    upward: bool,
    step: FixtureFrequencyNumericStep,
) -> Result<f64, FixtureFrequencyNumericError> {
    if !value.is_finite() {
        return Err(enclosure(step, FixtureFrequencyNumericIssue::NonFinite));
    }
    let result = if upward {
        value.next_up()
    } else {
        value.next_down()
    };
    if !result.is_finite() {
        return Err(enclosure(step, FixtureFrequencyNumericIssue::NonFinite));
    }
    Ok(result)
}

/// Publish a supported nonnegative outward error/upper, preserving exact zero.
/// # Errors
/// Refuse a negative/nonfinite upper or one without a finite successor.
fn upper_nonnegative(
    value: f64,
    step: FixtureFrequencyNumericStep,
) -> Result<f64, FixtureFrequencyNumericError> {
    bound_arithmetic(value, 0.0, FixtureChartBoundOperation::Add)
        .map_err(|source| FixtureFrequencyNumericError::Bound { step, source })
}

/// Private fixed-scalar enclosure; endpoints are finite and ordered by construction.
#[derive(Clone, Copy)]
struct Interval {
    /// Finite lower endpoint of the closed exact-real enclosure.
    low: f64,
    /// Finite upper endpoint, never below the lower endpoint.
    high: f64,
}

impl Interval {
    /// Exact zero interval for mathematically exact zero identities only.
    const ZERO: Self = Self {
        low: 0.0,
        high: 0.0,
    };
    /// Exact one interval, used for weight complements.
    const ONE: Self = Self {
        low: 1.0,
        high: 1.0,
    };

    /// Wrap already supported stored operand bits as an exact real parameter.
    const fn point(value: f64) -> Self {
        Self {
            low: value,
            high: value,
        }
    }

    /// Form outward endpoints from an admitted point and nonnegative error.
    /// # Errors
    /// Refuse endpoint overflow; zero error preserves the exact stored point.
    fn around(
        value: f64,
        error: f64,
        step: FixtureFrequencyNumericStep,
    ) -> Result<Self, FixtureFrequencyNumericError> {
        if error == 0.0 {
            return Ok(Self::point(value));
        }
        Ok(Self {
            low: outward(value - error, false, step)?,
            high: outward(value + error, true, step)?,
        })
    }

    /// Recognize only a genuinely exact zero enclosure.
    fn is_zero(self) -> bool {
        self.low == 0.0 && self.high == 0.0
    }

    /// Intersect a mathematically nonnegative quantity with its known domain.
    fn nonnegative(self) -> Self {
        Self {
            low: self.low.max(0.0),
            high: self.high.max(0.0),
        }
    }

    /// Intersect the proven weight/complement range with the closed unit interval.
    fn unit(self) -> Self {
        Self {
            low: self.low.clamp(0.0, 1.0),
            high: self.high.clamp(0.0, 1.0),
        }
    }

    /// Add closed intervals outward; exact zero operands preserve their peer.
    /// # Errors
    /// Refuse endpoint overflow rather than replace an unavailable bound.
    fn add(
        self,
        other: Self,
        step: FixtureFrequencyNumericStep,
    ) -> Result<Self, FixtureFrequencyNumericError> {
        if self.is_zero() {
            return Ok(other);
        }
        if other.is_zero() {
            return Ok(self);
        }
        Ok(Self {
            low: outward(self.low + other.low, false, step)?,
            high: outward(self.high + other.high, true, step)?,
        })
    }

    /// Subtract outward; identical exact stored operands cancel exactly.
    /// # Errors
    /// Refuse endpoint overflow, leaving both immutable operands unchanged.
    #[expect(
        clippy::float_cmp,
        reason = "This branch recognizes exact singleton identity, including signed zeros; an approximate comparison could falsely erase a nonzero mathematical difference."
    )]
    fn subtract(
        self,
        other: Self,
        step: FixtureFrequencyNumericStep,
    ) -> Result<Self, FixtureFrequencyNumericError> {
        if self.low == self.high && other.low == other.high && self.low == other.low {
            return Ok(Self::ZERO);
        }
        self.add(
            Self {
                low: -other.high,
                high: -other.low,
            },
            step,
        )
    }

    /// Multiply four endpoint pairs in fixed scalar work; exact zero stays zero.
    /// # Errors
    /// Refuse nonfinite products or unrepresentable outward endpoints.
    fn multiply(
        self,
        other: Self,
        step: FixtureFrequencyNumericStep,
    ) -> Result<Self, FixtureFrequencyNumericError> {
        if self.is_zero() || other.is_zero() {
            return Ok(Self::ZERO);
        }
        let pairs = [
            (self.low, other.low),
            (self.low, other.high),
            (self.high, other.low),
            (self.high, other.high),
        ];
        let mut low = f64::MAX;
        let mut high = -f64::MAX;
        for (left, right) in pairs {
            let (lower, upper) = if left == 0.0 || right == 0.0 {
                (0.0, 0.0)
            } else {
                let product = left * right;
                (
                    outward(product, false, step)?,
                    outward(product, true, step)?,
                )
            };
            low = low.min(lower);
            high = high.max(upper);
        }
        Ok(Self { low, high })
    }

    /// Divide endpoint pairs by a strictly positive denominator interval.
    /// # Errors
    /// Refuse a singular denominator or unavailable finite outward quotient.
    fn divide(
        self,
        other: Self,
        step: FixtureFrequencyNumericStep,
    ) -> Result<Self, FixtureFrequencyNumericError> {
        if other.low <= 0.0 {
            return Err(enclosure(step, FixtureFrequencyNumericIssue::NonPositive));
        }
        if self.is_zero() {
            return Ok(Self::ZERO);
        }
        let pairs = [
            (self.low, other.low),
            (self.low, other.high),
            (self.high, other.low),
            (self.high, other.high),
        ];
        let mut low = f64::MAX;
        let mut high = -f64::MAX;
        for (left, right) in pairs {
            let (lower, upper) = if left == 0.0 {
                (0.0, 0.0)
            } else {
                let quotient = left / right;
                (
                    outward(quotient, false, step)?,
                    outward(quotient, true, step)?,
                )
            };
            low = low.min(lower);
            high = high.max(upper);
        }
        Ok(Self { low, high })
    }

    /// Map a closed interval through exact absolute value without arithmetic loss.
    fn absolute(self) -> Self {
        if self.low >= 0.0 {
            self
        } else if self.high <= 0.0 {
            Self {
                low: -self.high,
                high: -self.low,
            }
        } else {
            Self {
                low: 0.0,
                high: (-self.low).max(self.high),
            }
        }
    }

    /// Bound the correctly rounded root monotonically, including exact zero.
    /// # Errors
    /// Reject negative endpoints or unavailable finite outward roots.
    fn square_root(
        self,
        step: FixtureFrequencyNumericStep,
    ) -> Result<Self, FixtureFrequencyNumericError> {
        if self.low < 0.0 {
            return Err(enclosure(step, FixtureFrequencyNumericIssue::Negative));
        }
        let low = if self.low == 0.0 {
            0.0
        } else {
            outward(self.low.sqrt(), false, step)?
        };
        let high = if self.high == 0.0 {
            0.0
        } else {
            outward(self.high.sqrt(), true, step)?
        };
        Ok(Self { low, high })
    }

    /// Enclose distance from a nominal point to every exact member of this range.
    /// # Errors
    /// Refuse magnitude overflow or an unsupported final nonnegative upper.
    fn error(
        self,
        point: f64,
        step: FixtureFrequencyNumericStep,
    ) -> Result<f64, FixtureFrequencyNumericError> {
        upper_nonnegative(
            (point - self.low).abs().max((self.high - point).abs()),
            step,
        )
    }
}

impl FrequencyPrior {
    /// Convert this actual retained model through an explicitly supplied basis.
    /// No request acknowledges a baseline, refreshes evidence or grants authority.
    /// # Errors
    /// Reject raw ratios/scalars, arithmetic/enclosure failure or output admission.
    pub(crate) fn baseline_request(
        &self,
        operands: BaselineOperands,
    ) -> Result<BaselineRequest, FixtureFrequencyNumericError> {
        let raw = RawBaselineOperands { value: operands };
        let validated = raw.validate();
        baseline_operands_rule(&raw.value)?;
        validated.map_err(|_| FixtureFrequencyNumericError::ValidationInvariant)?;
        let operands = raw.value;
        let residual = point(
            point(
                self.mean,
                operands.chart_rate,
                ArithmeticOperation::Add,
                FixtureFrequencyNumericStep::BaselineResidual,
            )?,
            operands.actual_rate,
            ArithmeticOperation::Subtract,
            FixtureFrequencyNumericStep::BaselineResidual,
        )?;
        let increment = point(
            residual,
            operands.active_ratio,
            ArithmeticOperation::Divide,
            FixtureFrequencyNumericStep::BaselineIncrement,
        )?;
        let desired = point(
            operands.baseline,
            increment,
            ArithmeticOperation::Add,
            FixtureFrequencyNumericStep::BaselineRequest,
        )?;
        let desired_error = self
            .baseline_range(operands)?
            .error(desired, FixtureFrequencyNumericStep::BaselineRequest)?;
        BaselineRequest {
            desired,
            desired_error,
            requested: desired.clamp(-operands.cap, operands.cap),
            clipped: desired < -operands.cap || desired > operands.cap,
            cap: operands.cap,
        }
        .admit()
    }

    /// Enclose the exact desired expression over admitted model/map inputs.
    /// # Errors
    /// Reject unavailable finite endpoints or a nonpositive ratio denominator.
    fn baseline_range(
        &self,
        operands: BaselineOperands,
    ) -> Result<Interval, FixtureFrequencyNumericError> {
        let step = FixtureFrequencyNumericStep::BaselineResidual;
        Interval::around(self.mean, self.mean_error, step)?
            .add(
                Interval::around(operands.chart_rate, operands.chart_rate_error, step)?,
                step,
            )?
            .subtract(
                Interval::around(operands.actual_rate, operands.actual_rate_error, step)?,
                step,
            )?
            .divide(
                Interval::around(operands.active_ratio, operands.active_ratio_error, step)?,
                FixtureFrequencyNumericStep::BaselineIncrement,
            )?
            .add(
                Interval::point(operands.baseline),
                FixtureFrequencyNumericStep::BaselineRequest,
            )
    }

    /// Re-express the mean under one known gradient shift, without new learning.
    /// Residual/skew facts share that exact shift and retain their original bits.
    /// # Errors
    /// Reject delta/error input, unsupported subtraction or transformed bounds.
    pub(crate) fn shift_rate(
        &self,
        delta: f64,
        error: f64,
    ) -> Result<Self, FixtureFrequencyNumericError> {
        let raw = RawShift {
            value: (delta, error),
        };
        let validated = raw.validate();
        shift_rule(&raw.value)?;
        validated.map_err(|_| FixtureFrequencyNumericError::ValidationInvariant)?;
        let (delta, error) = raw.value;
        if delta == 0.0 && error == 0.0 {
            return Ok(*self);
        }
        let step = FixtureFrequencyNumericStep::RateRebase;
        let mean = point(self.mean, delta, ArithmeticOperation::Subtract, step)?;
        let mean_error = Interval::around(self.mean, self.mean_error, step)?
            .subtract(Interval::around(delta, error, step)?, step)?
            .error(mean, step)?;
        Self {
            mean,
            mean_error,
            ..*self
        }
        .admit()
    }
}

impl BaselineRequest {
    /// Validate every transformed numeric field and its clipping relationship.
    /// # Errors
    /// Preserve scalar, bound or mandatory-validation disagreement.
    fn admit(self) -> Result<Self, FixtureFrequencyNumericError> {
        let raw = RawBaselineRequest { value: self };
        let validated = raw.validate();
        baseline_request_rule(&raw.value)?;
        validated.map_err(|_| FixtureFrequencyNumericError::ValidationInvariant)?;
        Ok(raw.value)
    }

    /// Copy the unclipped desired active-rate point.
    #[must_use]
    pub(crate) const fn desired(&self) -> f64 {
        self.desired
    }
    /// Copy its separate arithmetic/map error, without policy clipping.
    #[must_use]
    pub(crate) const fn desired_error(&self) -> f64 {
        self.desired_error
    }
    /// Copy the policy-clipped requested point, never an acknowledgement.
    #[must_use]
    pub(crate) const fn requested(&self) -> f64 {
        self.requested
    }
    /// Report point clipping separately from numerical uncertainty.
    #[must_use]
    pub(crate) const fn clipped(&self) -> bool {
        self.clipped
    }
}

impl BaselineMapNumbers {
    /// Admit complete declared numeric map coefficients without physical assurance.
    /// # Errors
    /// Reject unsupported points/errors or a nonpositive ratio enclosure.
    pub(crate) fn new(
        ratio: f64,
        ratio_error: f64,
        phase: f64,
        phase_error: f64,
        rate: f64,
        rate_error: f64,
    ) -> Result<Self, FixtureFrequencyNumericError> {
        Self {
            ratio,
            ratio_error,
            phase,
            phase_error,
            rate,
            rate_error,
        }
        .admit()
    }

    /// Advance absolute Cold to the known acknowledgement before changing its slope.
    /// No old-rate constant chart rebase is added to this absolute map again.
    /// # Errors
    /// Reject actual-rate inputs, exact duration conversion or transformed output.
    pub(crate) fn advance(
        &self,
        old_baseline: f64,
        actual_baseline: f64,
        elapsed: MonotonicDurationNs,
    ) -> Result<Self, FixtureFrequencyNumericError> {
        let change =
            checked_rate_change(self.ratio, self.ratio_error, old_baseline, actual_baseline)?;
        let (seconds, time) = seconds_interval(elapsed)?;
        let step = FixtureFrequencyNumericStep::MapAdvance;
        let phase = if elapsed.nanoseconds() == 0 {
            self.phase
        } else {
            point(
                self.phase,
                point(self.rate, seconds, ArithmeticOperation::Multiply, step)?,
                ArithmeticOperation::Add,
                step,
            )?
        };
        let phase_error = if elapsed.nanoseconds() == 0 {
            self.phase_error
        } else {
            Interval::around(self.phase, self.phase_error, step)?
                .add(
                    Interval::around(self.rate, self.rate_error, step)?.multiply(time, step)?,
                    step,
                )?
                .error(phase, step)?
        };
        let rate = if change.range.is_zero() {
            self.rate
        } else {
            point(self.rate, change.point, ArithmeticOperation::Add, step)?
        };
        let rate_error = if change.range.is_zero() {
            self.rate_error
        } else {
            Interval::around(self.rate, self.rate_error, step)?
                .add(change.range, step)?
                .error(rate, step)?
        };
        Self {
            phase,
            phase_error,
            rate,
            rate_error,
            ..*self
        }
        .admit()
    }

    /// Require the complete numeric map rule before publishing any fields.
    /// # Errors
    /// Preserve exact field failure before validator disagreement.
    fn admit(self) -> Result<Self, FixtureFrequencyNumericError> {
        let raw = RawBaselineMap { value: self };
        let validated = raw.validate();
        map_rule(&raw.value)?;
        validated.map_err(|_| FixtureFrequencyNumericError::ValidationInvariant)?;
        Ok(raw.value)
    }

    /// Copy the declared active/monotonic ratio.
    #[must_use]
    pub(crate) const fn ratio(&self) -> f64 {
        self.ratio
    }
    /// Copy its absolute error, not a calibration guarantee.
    #[must_use]
    pub(crate) const fn ratio_error(&self) -> f64 {
        self.ratio_error
    }
    /// Copy absolute Cold at the lifecycle-owned reference.
    #[must_use]
    pub(crate) const fn phase(&self) -> f64 {
        self.phase
    }
    /// Copy its carried absolute error for component checks.
    #[cfg(test)]
    #[must_use]
    pub(crate) const fn phase_error(&self) -> f64 {
        self.phase_error
    }
    /// Copy the known actual-clock displacement gradient.
    #[must_use]
    pub(crate) const fn rate(&self) -> f64 {
        self.rate
    }
    /// Copy its separately carried absolute error.
    #[must_use]
    pub(crate) const fn rate_error(&self) -> f64 {
        self.rate_error
    }
}

/// Admit and compute one achieved active-rate change in monotonic-gradient units.
/// # Errors
/// Preserve actual-input, subtraction, product and outward-enclosure causes.
fn checked_rate_change(
    ratio: f64,
    error: f64,
    old: f64,
    actual: f64,
) -> Result<RateChange, FixtureFrequencyNumericError> {
    let raw = RawBaselineChange {
        value: BaselineChange {
            ratio,
            error,
            old,
            actual,
        },
    };
    let validated = raw.validate();
    change_rule(&raw.value)?;
    validated.map_err(|_| FixtureFrequencyNumericError::ValidationInvariant)?;
    let BaselineChange {
        ratio,
        error,
        old,
        actual,
    } = raw.value;
    let step = FixtureFrequencyNumericStep::AffineEffectRate;
    let delta = point(actual, old, ArithmeticOperation::Subtract, step)?;
    let result = point(ratio, delta, ArithmeticOperation::Multiply, step)?;
    let range = Interval::around(ratio, error, step)?.multiply(
        Interval::point(actual).subtract(Interval::point(old), step)?,
        step,
    )?;
    Ok(RateChange {
        point: result,
        range,
    })
}

/// Convert a signed exact integer difference, never either absolute timestamp.
/// # Errors
/// Preserve the inclusive 2^53 integer guard and checked seconds/enclosure cause.
fn signed_seconds_interval(
    at: MonotonicNs,
    reference: MonotonicNs,
) -> Result<(f64, Interval), FixtureFrequencyNumericError> {
    let nanos = integer_to_f64(
        at.nanoseconds().abs_diff(reference.nanoseconds()),
        NormalizationComponent::Time,
    )
    .map_err(|source| FixtureFrequencyNumericError::Duration { source })?;
    let step = FixtureFrequencyNumericStep::AffineEffectPhase;
    let seconds = point(
        nanos,
        NANOSECONDS_PER_SECOND,
        ArithmeticOperation::Divide,
        step,
    )?;
    let range = Interval::point(nanos).divide(Interval::point(NANOSECONDS_PER_SECOND), step)?;
    if at < reference {
        Ok((
            -seconds,
            Interval {
                low: -range.high,
                high: -range.low,
            },
        ))
    } else {
        Ok((seconds, range))
    }
}

/// Derive the continuous affine effect g*(M-Ma), retaining its computed bounds.
/// # Errors
/// Reject raw achieved/ratio inputs, unsupported integer difference or coefficients.
pub(crate) fn baseline_effect_coefficients(
    reference: MonotonicNs,
    at: MonotonicNs,
    ratio: f64,
    ratio_error: f64,
    old: f64,
    actual: f64,
) -> Result<AffineEffectCoefficients, FixtureFrequencyNumericError> {
    let change = checked_rate_change(ratio, ratio_error, old, actual)?;
    let (seconds, time) = signed_seconds_interval(reference, at)?;
    let step = FixtureFrequencyNumericStep::AffineEffectPhase;
    let phase = point(change.point, seconds, ArithmeticOperation::Multiply, step)?;
    let value = AffineEffectCoefficients {
        phase,
        phase_error: change.range.multiply(time, step)?.error(phase, step)?,
        rate: change.point,
        rate_error: change
            .range
            .error(change.point, FixtureFrequencyNumericStep::AffineEffectRate)?,
    };
    let raw = RawAffineEffect { value };
    let validated = raw.validate();
    affine_effect_rule(&raw.value)?;
    validated.map_err(|_| FixtureFrequencyNumericError::ValidationInvariant)?;
    Ok(raw.value)
}

impl AffineEffectCoefficients {
    /// Copy the computed displacement coefficient.
    #[must_use]
    pub(crate) const fn phase(&self) -> f64 {
        self.phase
    }
    /// Copy its inherited/new error without resetting it at chart construction.
    #[must_use]
    pub(crate) const fn phase_error(&self) -> f64 {
        self.phase_error
    }
    /// Copy the actual achieved-change gradient.
    #[must_use]
    pub(crate) const fn rate(&self) -> f64 {
        self.rate
    }
    /// Copy its separate ratio/arithmetic error.
    #[must_use]
    pub(crate) const fn rate_error(&self) -> f64 {
        self.rate_error
    }
}

impl FrequencyAnchor {
    /// Subtract the actual affine effect at this model's original projected epoch.
    /// # Errors
    /// Reject outside-domain queries, unsupported evaluation or unavailable bounds.
    pub(crate) fn rebase_affine(
        &self,
        effect: &FixtureAffineChart,
        at: MonotonicNs,
    ) -> Result<Self, FixtureFrequencyNumericError> {
        let domain = effect.domain();
        if at < domain.start() || at > domain.end() {
            return Err(FixtureFrequencyNumericError::Chart {
                source: FixtureChartError::QueryOutsideDomain,
            });
        }
        if effect.rate_seconds_per_second() == 0.0
            && effect.rate_coefficient_roundoff_seconds_per_second() == 0.0
        {
            return self.rebase(
                effect.phase_at_reference_seconds(),
                effect.phase_coefficient_roundoff_seconds(),
            );
        }
        let (seconds, time) = signed_seconds_interval(at, domain.common_reference())?;
        let step = FixtureFrequencyNumericStep::AffineAnchorRebase;
        let displacement = point(
            effect.phase_at_reference_seconds(),
            point(
                effect.rate_seconds_per_second(),
                seconds,
                ArithmeticOperation::Multiply,
                step,
            )?,
            ArithmeticOperation::Add,
            step,
        )?;
        let range = Interval::around(
            effect.phase_at_reference_seconds(),
            effect.phase_coefficient_roundoff_seconds(),
            step,
        )?
        .add(
            Interval::around(
                effect.rate_seconds_per_second(),
                effect.rate_coefficient_roundoff_seconds_per_second(),
                step,
            )?
            .multiply(time, step)?,
            step,
        )?;
        if displacement == 0.0 && range.is_zero() {
            return Ok(*self);
        }
        let phase = point(
            self.phase,
            displacement,
            ArithmeticOperation::Subtract,
            step,
        )?;
        let error = Interval::around(self.phase, self.error, step)?
            .subtract(range, step)?
            .error(phase, step)?;
        Self { phase, error }.admit()
    }
}

/// Check exact same-segment integer spans against a declared positive ratio.
/// Zero/zero is consistency only; this never calibrates a map from timestamps.
/// # Errors
/// Reject raw ratio/error, integer envelope or a contradictory span relation.
pub(crate) fn check_active_ratio_span(
    active: ActiveAwakeDurationNs,
    monotonic: MonotonicDurationNs,
    ratio: f64,
    error: f64,
) -> Result<(), FixtureFrequencyNumericError> {
    let raw = RawRatio {
        value: (ratio, error),
    };
    let validated = raw.validate();
    ratio_rule(&raw.value)?;
    validated.map_err(|_| FixtureFrequencyNumericError::ValidationInvariant)?;
    let (ratio, error) = raw.value;
    let active = integer_to_f64(active.nanoseconds(), NormalizationComponent::Time)
        .map_err(|source| FixtureFrequencyNumericError::Duration { source })?;
    let mono = integer_to_f64(monotonic.nanoseconds(), NormalizationComponent::Time)
        .map_err(|source| FixtureFrequencyNumericError::Duration { source })?;
    let step = FixtureFrequencyNumericStep::ActiveRatioSpan;
    let range = Interval::around(ratio, error, step)?.multiply(Interval::point(mono), step)?;
    if active < range.low || active > range.high {
        return Err(FixtureFrequencyNumericError::ActiveRatioMismatch);
    }
    Ok(())
}

/// Private controls retain their actual compiled numerical-module identity.
#[cfg(test)]
#[path = "../../../../tests/private/frequency_prior_numeric.rs"]
mod tests;

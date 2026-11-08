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
use time_domain::MonotonicDurationNs;

use super::{NANOSECONDS_PER_SECOND, bound_arithmetic, coefficient_addition_error};
use crate::regression::normalization::integer_to_f64;
use crate::regression::{
    ArithmeticIssue, ArithmeticOperation, ArithmeticStage, FitError, FixtureChartBoundOperation,
    FixtureChartError, NormalizationComponent, NormalizationError, arithmetic, checked_square_root,
    scalar_issue,
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
            Self::Bound { source, .. } => Some(source),
            Self::Duration { source } => Some(source),
            Self::Input { .. }
            | Self::Enclosure { .. }
            | Self::IndeterminateWeights
            | Self::SingularWeights
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

/// Private controls retain their actual compiled numerical-module identity.
#[cfg(test)]
#[path = "../../../../tests/private/frequency_prior_numeric.rs"]
mod tests;

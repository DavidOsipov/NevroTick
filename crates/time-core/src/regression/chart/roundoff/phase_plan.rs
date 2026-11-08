// SPDX-License-Identifier: GPL-2.0-only
//! Checked baseline-relative phase debt and finite requested-rate numerics.
//!
//! The lifecycle owner supplies actual modeled rates and typed active durations.
//! This owner carries numerical error through separate RN primitives; it grants
//! no source freshness, delivery, operation identity or clock authority. Bounds
//! concern exact stored inputs and exact integer durations, never physical error.

use std::fmt;

use garde::Validate;
use time_domain::ActiveAwakeDurationNs;

use super::{
    ETA_UPPER, NANOSECONDS_PER_SECOND, UNIT_ROUNDOFF, bound_arithmetic, coefficient_addition_error,
    sum_four_up,
};
use crate::regression::normalization::integer_to_f64;
use crate::regression::{
    ArithmeticIssue, ArithmeticOperation, ArithmeticStage, FitError, FixtureChartBoundOperation,
    FixtureChartError, NormalizationComponent, NormalizationError, arithmetic, scalar_issue,
};

/// Closed scalar role for phase-plan numerical admission; no input value is kept.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixturePhaseNumericField {
    /// Carried baseline-relative phase obligation, in displacement seconds.
    Debt,
    /// Nonnegative absolute numerical error on the carried phase obligation.
    DebtRoundoff,
    /// Known aggregate applied rate, in displacement seconds per active second.
    AggregateRate,
    /// Declared known effective baseline in the same rate units and interval.
    BaselineRate,
    /// Strictly positive maximum magnitude of a requested transient rate.
    TransientCap,
    /// Strictly positive maximum magnitude of the requested aggregate rate.
    AggregateCap,
    /// Requested transient-rate point after the explicit symmetric clamp.
    TransientRate,
    /// Nonnegative numerical error relative to the ideal clamped quotient.
    TransientRoundoff,
    /// Nonnegative numerical error of the requested aggregate-rate point.
    AggregateRoundoff,
    /// Positive conservative lower bound on exact remaining active seconds.
    RemainingSeconds,
}

/// Unsupported scalar property in the maintained normal-or-zero value envelope.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixturePhaseNumericIssue {
    /// A scalar is NaN or infinite.
    NonFinite,
    /// A nonzero scalar is subnormal.
    Subnormal,
    /// A numerical error bound is negative.
    Negative,
    /// A cap or denominator lower bound is zero or negative.
    NonPositive,
}

/// Actual phase-plan numerical operation, separate from chart projection steps.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixturePhaseNumericStep {
    /// Convert exact delivered active nanoseconds to separately rounded seconds.
    ElapsedSeconds,
    /// Multiply the actual aggregate rate by rounded delivered active seconds.
    AggregateDelivery,
    /// Multiply the known baseline by the same rounded active seconds.
    BaselineDelivery,
    /// Subtract this interval's aggregate contribution from the carried debt.
    DebtSubtract,
    /// Restore this interval's baseline contribution to that intermediate.
    DebtAdd,
    /// Convert exact remaining active nanoseconds to rounded seconds.
    RemainingSeconds,
    /// Form a strictly positive lower bound on exact remaining active seconds.
    HorizonLower,
    /// Divide carried debt by the separately rounded remaining duration.
    TransientDivide,
    /// Add the requested clamped transient point to the unchanged baseline.
    AggregateAdd,
    /// Bound the duration conversion relative to exact integer nanoseconds.
    DurationBound,
    /// Bound a delivered rate product relative to the exact active duration.
    DeliveryBound,
    /// Carry inherited and newly evaluated phase-debt arithmetic error.
    DebtBound,
    /// Bound numerator, denominator-conversion and quotient-rounding error.
    TransientBound,
    /// Bound aggregate proposal addition and inherited transient error.
    AggregateBound,
}

/// Fixed numerical refusal without raw phase, rate, timestamps or validation reports.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixturePhaseNumericError {
    /// A raw or transformed scalar violates its named invariant.
    Input {
        /// Closed role of the rejected scalar.
        field: FixturePhaseNumericField,
        /// Closed unsupported property, without the rejected value.
        issue: FixturePhaseNumericIssue,
    },
    /// Exact integer conversion rejected the existing inclusive `2^53` envelope.
    Duration {
        /// Delivered or remaining duration conversion being evaluated.
        step: FixturePhaseNumericStep,
        /// Existing bounded exact-conversion cause.
        source: NormalizationError,
    },
    /// A separately rounded value primitive is unsupported.
    Arithmetic {
        /// Actual new numerical operation.
        step: FixturePhaseNumericStep,
        /// Shared checked primitive's bounded cause.
        source: FitError,
    },
    /// An outward arithmetic bound cannot be represented finitely.
    Bound {
        /// Actual phase-plan bound being evaluated.
        step: FixturePhaseNumericStep,
        /// Existing bounded outward-arithmetic cause.
        source: FixtureChartError,
    },
    /// Remaining active duration is exactly zero; no useful division is possible.
    EmptyHorizon,
    /// The requested aggregate point exceeds its explicit magnitude cap.
    AggregateLimit,
    /// Mandatory validation disagrees with the shared complete invariant rule.
    ValidationInvariant,
}

impl fmt::Display for FixturePhaseNumericError {
    /// Format only fixed numerical roles/codes, retaining no rejected values.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input { field, issue } => {
                write!(formatter, "phase numeric input: {field:?}/{issue:?}")
            }
            Self::Duration { step, .. } => write!(formatter, "phase numeric duration: {step:?}"),
            Self::Arithmetic { step, .. } => {
                write!(formatter, "phase numeric arithmetic: {step:?}")
            }
            Self::Bound { step, .. } => write!(formatter, "phase numeric bound: {step:?}"),
            Self::EmptyHorizon => formatter.write_str("phase numeric: empty active horizon"),
            Self::AggregateLimit => formatter.write_str("phase numeric: aggregate cap exceeded"),
            Self::ValidationInvariant => formatter.write_str("phase numeric: validation invariant"),
        }
    }
}

impl std::error::Error for FixturePhaseNumericError {
    /// Expose only the already bounded typed arithmetic/conversion cause.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Duration { source, .. } => Some(source),
            Self::Arithmetic { source, .. } => Some(source),
            Self::Bound { source, .. } => Some(source),
            Self::Input { .. }
            | Self::EmptyHorizon
            | Self::AggregateLimit
            | Self::ValidationInvariant => None,
        }
    }
}

/// Immutable carried numerical obligation, without lifecycle or delivery authority.
#[derive(Clone, Copy)]
pub(crate) struct PhaseDebt {
    /// Separately rounded current debt in baseline-relative displacement seconds.
    point: f64,
    /// Absolute error enclosing all inherited and new numerical operations.
    error: f64,
}

impl fmt::Debug for PhaseDebt {
    /// Redact the numerical obligation and its bound with constant output.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PhaseDebt { checked: true, .. }")
    }
}

/// One complete debt invariant shared by raw construction and transformed output.
/// # Errors
/// Reject unsupported point, then unsupported/negative error; no effects occur.
fn debt_rule(value: &PhaseDebt) -> Result<(), FixturePhaseNumericError> {
    supported(value.point, FixturePhaseNumericField::Debt)?;
    nonnegative(value.error, FixturePhaseNumericField::DebtRoundoff)
}

/// Mandatory complete-field admission for immutable phase-debt construction.
#[derive(Validate)]
struct RawDebt {
    /// One rule owns both the point and its independently meaningful error bound.
    #[garde(custom(|value: &PhaseDebt, (): &()| {
        debt_rule(value).map_err(|_| garde::Error::new("invalid phase debt"))
    }))]
    value: PhaseDebt,
}

/// One known modeled interval's immutable scalar and typed-duration inputs.
#[derive(Clone, Copy)]
struct SettlementInputs {
    /// Actual aggregate rate; no requested rate is retained here.
    aggregate: f64,
    /// Declared known effective baseline for this same active interval.
    baseline: f64,
    /// Existing typed duration owns nonnegative active-time representation.
    duration: ActiveAwakeDurationNs,
}

/// Shared settlement-input rule; exact conversion remains the numerical transform.
/// # Errors
/// Reject unsupported aggregate then baseline; the typed duration is not converted.
fn settlement_rule(value: &SettlementInputs) -> Result<(), FixturePhaseNumericError> {
    supported(value.aggregate, FixturePhaseNumericField::AggregateRate)?;
    supported(value.baseline, FixturePhaseNumericField::BaselineRate)?;
    // ActiveAwakeDurationNs privately owns the complete nonnegative u64 value.
    // Its exact-to-binary64 support guard belongs to duration_seconds below.
    Ok(())
}

/// Unavoidable input admission before a debt transformation reads raw rates.
#[derive(Validate)]
struct RawSettlement {
    /// The shared rule covers both scalars; the typed duration retains its owner.
    #[garde(custom(|value: &SettlementInputs, (): &()| {
        settlement_rule(value).map_err(|_| garde::Error::new("invalid phase settlement numerics"))
    }))]
    value: SettlementInputs,
}

/// Immutable explicit proposal inputs, with no adopted or implicit defaults.
#[derive(Clone, Copy)]
struct ProposalInputs {
    /// Stored known baseline, whose signed-zero bits must remain meaningful.
    baseline: f64,
    /// Existing active duration; zero is rejected before quotient evaluation.
    remaining: ActiveAwakeDurationNs,
    /// Strictly positive symmetric transient point cap.
    transient_cap: f64,
    /// Strictly positive aggregate point cap.
    aggregate_cap: f64,
}

/// Shared proposal-input rule, including the baseline-only output's cap.
/// # Errors
/// Reject baseline, transient cap, aggregate cap, zero horizon, then baseline cap.
fn proposal_rule(value: &ProposalInputs) -> Result<(), FixturePhaseNumericError> {
    supported(value.baseline, FixturePhaseNumericField::BaselineRate)?;
    positive(value.transient_cap, FixturePhaseNumericField::TransientCap)?;
    positive(value.aggregate_cap, FixturePhaseNumericField::AggregateCap)?;
    if value.remaining.nanoseconds() == 0 {
        return Err(FixturePhaseNumericError::EmptyHorizon);
    }
    if value.baseline.abs() > value.aggregate_cap {
        return Err(FixturePhaseNumericError::AggregateLimit);
    }
    Ok(())
}

/// Mandatory whole-input rule for every requested-rate transformation.
#[derive(Validate)]
struct RawProposal {
    /// The rule owns raw caps/baseline and checks the typed duration's zero case.
    #[garde(custom(|value: &ProposalInputs, (): &()| {
        proposal_rule(value).map_err(|_| garde::Error::new("invalid phase proposal numerics"))
    }))]
    value: ProposalInputs,
}

/// Checked requested rates; neither their points nor bounds assert application.
///
/// With no established debt sign, useful is false: the request is exactly no
/// transient plus the stored baseline. Its zero request errors do not declare
/// the uncertain debt or an unavailable ideal phase correction to be zero.
#[derive(Clone, Copy)]
pub(crate) struct PhaseRates {
    /// Requested transient point, or positive zero in the no-transient branch.
    transient: f64,
    /// Error relative to the ideal clamped quotient when useful is true.
    transient_error: f64,
    /// Separately rounded aggregate request, or unchanged baseline bits.
    aggregate: f64,
    /// Aggregate-addition plus inherited transient error when useful is true.
    aggregate_error: f64,
    /// A strict numerical enclosure establishes the debt's nonzero sign.
    useful: bool,
}

impl fmt::Debug for PhaseRates {
    /// Redact all numerical requests and bounds with constant output.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PhaseRates { checked: true, .. }")
    }
}

/// Complete checked result together with its unchanged admitted cap context.
struct RateCandidate {
    /// Transformed points, numerical bounds and their explicit branch meaning.
    rates: PhaseRates,
    /// Immutable already-admitted context supplies exact cap/baseline invariants.
    inputs: ProposalInputs,
}

/// Shared complete output rule; every field has a scalar or contextual owner.
/// # Errors
/// Reject unsupported points/bounds, cap excess, then an inconsistent branch.
fn rates_rule(value: &RateCandidate) -> Result<(), FixturePhaseNumericError> {
    let rates = &value.rates;
    supported(rates.transient, FixturePhaseNumericField::TransientRate)?;
    nonnegative(
        rates.transient_error,
        FixturePhaseNumericField::TransientRoundoff,
    )?;
    supported(rates.aggregate, FixturePhaseNumericField::AggregateRate)?;
    nonnegative(
        rates.aggregate_error,
        FixturePhaseNumericField::AggregateRoundoff,
    )?;
    if rates.aggregate.abs() > value.inputs.aggregate_cap {
        return Err(FixturePhaseNumericError::AggregateLimit);
    }
    if rates.useful {
        if rates.transient == 0.0 || rates.transient.abs() > value.inputs.transient_cap {
            return Err(FixturePhaseNumericError::ValidationInvariant);
        }
    } else if rates.transient.to_bits() != 0
        || rates.transient_error != 0.0
        || rates.aggregate.to_bits() != value.inputs.baseline.to_bits()
        || rates.aggregate_error != 0.0
    {
        return Err(FixturePhaseNumericError::ValidationInvariant);
    }
    Ok(())
}

/// Mandatory post-transformation admission before requested rates are exposed.
#[derive(Validate)]
struct RawRates {
    /// One complete rule checks results against their unchanged admitted context.
    #[garde(custom(|value: &RateCandidate, (): &()| {
        rates_rule(value).map_err(|_| garde::Error::new("invalid phase rate result"))
    }))]
    value: RateCandidate,
}

impl PhaseDebt {
    /// Admit one stored debt point and its separate nonnegative absolute error.
    /// # Errors
    /// Mandatory complete-field Garde/typed admission publishes no partial debt.
    pub(crate) fn new(point: f64, error: f64) -> Result<Self, FixturePhaseNumericError> {
        let raw = RawDebt {
            value: Self { point, error },
        };
        let validated = raw.validate();
        debt_rule(&raw.value)?;
        validated.map_err(|_| FixturePhaseNumericError::ValidationInvariant)?;
        Ok(raw.value)
    }

    /// Copy the current checked debt point, without restoring lifecycle authority.
    #[must_use]
    pub(crate) const fn point(&self) -> f64 {
        self.point
    }

    /// Copy its numerical error, separate from source statistics/physical error.
    #[must_use]
    pub(crate) const fn error(&self) -> f64 {
        self.error
    }

    /// Carry `P_k - aggregate*N/1e9 + baseline*N/1e9` through separate RN steps.
    ///
    /// The lifecycle must already have committed the matching actual event. This
    /// immutable numerical transformation neither performs nor refunds that commit.
    /// Shared conversion/bound primitives use fixed scratch and no ambient state.
    /// # Errors
    /// Refuse unsupported inputs, integer/point/bound arithmetic or final admission;
    /// this value and the typed interval remain unchanged on every failure.
    pub(crate) fn settled(
        self,
        total_rate: f64,
        baseline: f64,
        elapsed: ActiveAwakeDurationNs,
    ) -> Result<Self, FixturePhaseNumericError> {
        let raw = RawSettlement {
            value: SettlementInputs {
                aggregate: total_rate,
                baseline,
                duration: elapsed,
            },
        };
        let validated = raw.validate();
        settlement_rule(&raw.value)?;
        validated.map_err(|_| FixturePhaseNumericError::ValidationInvariant)?;
        let inputs = raw.value;
        let (seconds, time_error) =
            duration_seconds(inputs.duration, FixturePhaseNumericStep::ElapsedSeconds)?;
        let total = point(
            inputs.aggregate,
            seconds,
            ArithmeticOperation::Multiply,
            FixturePhaseNumericStep::AggregateDelivery,
        )?;
        let base = point(
            inputs.baseline,
            seconds,
            ArithmeticOperation::Multiply,
            FixturePhaseNumericStep::BaselineDelivery,
        )?;
        let reduced = point(
            self.point,
            total,
            ArithmeticOperation::Subtract,
            FixturePhaseNumericStep::DebtSubtract,
        )?;
        let result = point(
            reduced,
            base,
            ArithmeticOperation::Add,
            FixturePhaseNumericStep::DebtAdd,
        )?;
        let total_error = delivery_error(inputs.aggregate, seconds, time_error)?;
        let base_error = delivery_error(inputs.baseline, seconds, time_error)?;
        let reduced_error = addition_error(
            self.point,
            self.error,
            -total,
            total_error,
            FixturePhaseNumericStep::DebtBound,
        )?;
        let error = addition_error(
            reduced,
            reduced_error,
            base,
            base_error,
            FixturePhaseNumericStep::DebtBound,
        )?;
        Self::new(result, error)
    }

    /// Request a capped transient over exact remaining active nanoseconds.
    ///
    /// The useful branch's bounds enclose the ideal clamped quotient of the
    /// carried debt enclosure by exact remaining seconds, then baseline addition.
    /// Clamp is nonexpansive, so it cannot increase the quotient error. An unknown
    /// debt sign requests no transient and preserves the baseline's exact bits.
    /// # Errors
    /// Reject bad raw caps/baseline/horizon, unsupported numerical operations,
    /// aggregate-cap excess or mandatory final result admission. No state changes.
    pub(crate) fn proposal(
        self,
        baseline: f64,
        remaining: ActiveAwakeDurationNs,
        transient_cap: f64,
        aggregate_cap: f64,
    ) -> Result<PhaseRates, FixturePhaseNumericError> {
        let raw = RawProposal {
            value: ProposalInputs {
                baseline,
                remaining,
                transient_cap,
                aggregate_cap,
            },
        };
        let validated = raw.validate();
        proposal_rule(&raw.value)?;
        validated.map_err(|_| FixturePhaseNumericError::ValidationInvariant)?;
        let inputs = raw.value;
        if self.point.abs() <= self.error {
            return PhaseRates::admit(
                PhaseRates {
                    transient: 0.0,
                    transient_error: 0.0,
                    aggregate: inputs.baseline,
                    aggregate_error: 0.0,
                    useful: false,
                },
                inputs,
            );
        }
        let (seconds, time_error) =
            duration_seconds(inputs.remaining, FixturePhaseNumericStep::RemainingSeconds)?;
        let quotient = point(
            self.point,
            seconds,
            ArithmeticOperation::Divide,
            FixturePhaseNumericStep::TransientDivide,
        )?;
        let transient = if quotient > inputs.transient_cap {
            inputs.transient_cap
        } else if quotient < -inputs.transient_cap {
            -inputs.transient_cap
        } else {
            quotient
        };
        let aggregate = point(
            inputs.baseline,
            transient,
            ArithmeticOperation::Add,
            FixturePhaseNumericStep::AggregateAdd,
        )?;
        if aggregate.abs() > inputs.aggregate_cap {
            return Err(FixturePhaseNumericError::AggregateLimit);
        }
        let quotient_error = quotient_error(self, seconds, time_error)?;
        let aggregate_error = addition_error(
            inputs.baseline,
            0.0,
            transient,
            quotient_error,
            FixturePhaseNumericStep::AggregateBound,
        )?;
        PhaseRates::admit(
            PhaseRates {
                transient,
                transient_error: quotient_error,
                aggregate,
                aggregate_error,
                useful: true,
            },
            inputs,
        )
    }
}

impl PhaseRates {
    /// Admit every transformed field against its unchanged immutable cap context.
    /// # Errors
    /// Reject a failed numerical/result invariant before publishing any request.
    fn admit(rates: Self, inputs: ProposalInputs) -> Result<Self, FixturePhaseNumericError> {
        let raw = RawRates {
            value: RateCandidate { rates, inputs },
        };
        let validated = raw.validate();
        rates_rule(&raw.value)?;
        validated.map_err(|_| FixturePhaseNumericError::ValidationInvariant)?;
        Ok(raw.value.rates)
    }

    /// Copy the requested transient, positive zero when no transient is useful.
    #[must_use]
    pub(crate) const fn transient_rate(&self) -> f64 {
        self.transient
    }

    /// Copy its numerical error; no-transient zero describes the request only.
    #[must_use]
    pub(crate) const fn transient_error(&self) -> f64 {
        self.transient_error
    }

    /// Copy the requested aggregate rate, retaining baseline bits when not useful.
    #[must_use]
    pub(crate) const fn aggregate_rate(&self) -> f64 {
        self.aggregate
    }

    /// Copy its numerical error, separate from actual actuator quantization.
    #[must_use]
    pub(crate) const fn aggregate_error(&self) -> f64 {
        self.aggregate_error
    }

    /// Report the strict numerical-sign branch without asserting delivery/readiness.
    #[must_use]
    pub(crate) const fn has_useful_transient(&self) -> bool {
        self.useful
    }
}

/// Shared normal-or-zero scalar admission, preserving both signed-zero bits.
/// # Errors
/// Reject nonfinite before subnormal values using the existing numerical owner.
fn supported(value: f64, field: FixturePhaseNumericField) -> Result<(), FixturePhaseNumericError> {
    let issue = match scalar_issue(value) {
        Some(ArithmeticIssue::NonFinite) => FixturePhaseNumericIssue::NonFinite,
        Some(ArithmeticIssue::Subnormal) => FixturePhaseNumericIssue::Subnormal,
        Some(_) => return Err(FixturePhaseNumericError::ValidationInvariant),
        None => return Ok(()),
    };
    Err(FixturePhaseNumericError::Input { field, issue })
}

/// Check a numerical error's normal-or-zero representation and nonnegative value.
/// # Errors
/// Reject scalar support first, then a negative bound without normalizing zeros.
fn nonnegative(
    value: f64,
    field: FixturePhaseNumericField,
) -> Result<(), FixturePhaseNumericError> {
    supported(value, field)?;
    if value < 0.0 {
        return Err(FixturePhaseNumericError::Input {
            field,
            issue: FixturePhaseNumericIssue::Negative,
        });
    }
    Ok(())
}

/// Check a strictly positive normal cap or denominator without changing its bits.
/// # Errors
/// Reject scalar support first, then either zero or a negative value.
fn positive(value: f64, field: FixturePhaseNumericField) -> Result<(), FixturePhaseNumericError> {
    supported(value, field)?;
    if value <= 0.0 {
        return Err(FixturePhaseNumericError::Input {
            field,
            issue: FixturePhaseNumericIssue::NonPositive,
        });
    }
    Ok(())
}

/// Evaluate exactly one RN primitive through the existing checked value owner.
/// # Errors
/// Preserve its exact bounded cause under the phase-plan caller's actual step.
fn point(
    left: f64,
    right: f64,
    operation: ArithmeticOperation,
    step: FixturePhaseNumericStep,
) -> Result<f64, FixturePhaseNumericError> {
    arithmetic(left, right, operation, ArithmeticStage::Coefficients)
        .map_err(|source| FixturePhaseNumericError::Arithmetic { step, source })
}

/// Evaluate one outward nonnegative bound using the existing shared primitive.
/// # Errors
/// Preserve its actual bounded cause without exposing any floating operands.
fn bound(
    left: f64,
    right: f64,
    operation: FixtureChartBoundOperation,
    step: FixturePhaseNumericStep,
) -> Result<f64, FixturePhaseNumericError> {
    bound_arithmetic(left, right, operation)
        .map_err(|source| FixturePhaseNumericError::Bound { step, source })
}

/// Convert exact active nanoseconds and bound RN conversion against the rational.
///
/// The integer owner rejects magnitudes above `2^53`; zero conversion is exact.
/// Nonzero error uses `u*upper(N/1e9)+eta`, always evaluated strictly outward.
/// # Errors
/// Preserve exact-conversion, point-division then outward-bound failures.
fn duration_seconds(
    duration: ActiveAwakeDurationNs,
    step: FixturePhaseNumericStep,
) -> Result<(f64, f64), FixturePhaseNumericError> {
    let nanos = integer_to_f64(duration.nanoseconds(), NormalizationComponent::Time)
        .map_err(|source| FixturePhaseNumericError::Duration { step, source })?;
    let seconds = point(
        nanos,
        NANOSECONDS_PER_SECOND,
        ArithmeticOperation::Divide,
        step,
    )?;
    if nanos == 0.0 {
        return Ok((seconds, 0.0));
    }
    let upper = bound(
        nanos,
        NANOSECONDS_PER_SECOND,
        FixtureChartBoundOperation::Divide,
        FixturePhaseNumericStep::DurationBound,
    )?;
    let relative = bound(
        UNIT_ROUNDOFF,
        upper,
        FixtureChartBoundOperation::Multiply,
        FixturePhaseNumericStep::DurationBound,
    )?;
    let error = bound(
        relative,
        ETA_UPPER,
        FixtureChartBoundOperation::Add,
        FixturePhaseNumericStep::DurationBound,
    )?;
    Ok((seconds, error))
}

/// Bound `RN(rate*s)` against exact `rate*N/1e9` using the stored-seconds error.
///
/// Exact zero products have zero error. Otherwise the absolute error is at most
/// `abs(rate)*E_s + u*abs(rate*s) + eta`, with all magnitudes bounded outward.
/// # Errors
/// Reject the first unsupported finite outward upper; inputs remain unchanged.
fn delivery_error(
    rate: f64,
    seconds: f64,
    time_error: f64,
) -> Result<f64, FixturePhaseNumericError> {
    if rate == 0.0 || seconds == 0.0 {
        return Ok(0.0);
    }
    let step = FixturePhaseNumericStep::DeliveryBound;
    let inherited = bound(
        rate.abs(),
        time_error,
        FixtureChartBoundOperation::Multiply,
        step,
    )?;
    let magnitude = bound(
        rate.abs(),
        seconds,
        FixtureChartBoundOperation::Multiply,
        step,
    )?;
    let rounding = bound(
        UNIT_ROUNDOFF,
        magnitude,
        FixtureChartBoundOperation::Multiply,
        step,
    )?;
    sum_four_up(inherited, rounding, ETA_UPPER, 0.0)
        .map_err(|source| FixturePhaseNumericError::Bound { step, source })
}

/// Reuse the parent addition enclosure for addition or subtraction via negation.
/// # Errors
/// Preserve a bound failure; no point value or prior numerical bound is changed.
fn addition_error(
    left: f64,
    left_error: f64,
    right: f64,
    right_error: f64,
    step: FixturePhaseNumericStep,
) -> Result<f64, FixturePhaseNumericError> {
    coefficient_addition_error(left, left_error, right, right_error)
        .map_err(|source| FixturePhaseNumericError::Bound { step, source })
}

/// Bound the computed quotient against debt enclosure divided by exact seconds.
///
/// Let stored seconds be `s`, exact seconds `T`, and `L<=T` be positive. The
/// discrepancy before quotient rounding is bounded by
/// `E_P/L + (abs(P)/s)*E_s/L`. Add `u*abs(P/s)+eta` for RN division. Stored `s`
/// is an exact bound-operation divisor; only `L`, never RN seconds, bounds `T`.
/// Symmetric clamping is nonexpansive and preserves this absolute error upper.
/// # Errors
/// Reject a failed lower bound or outward operation without publishing a result.
fn quotient_error(
    debt: PhaseDebt,
    seconds: f64,
    time_error: f64,
) -> Result<f64, FixturePhaseNumericError> {
    let lower_point = point(
        seconds,
        time_error,
        ArithmeticOperation::Subtract,
        FixturePhaseNumericStep::HorizonLower,
    )?;
    let lower = lower_point.next_down();
    positive(lower, FixturePhaseNumericField::RemainingSeconds)?;
    let step = FixturePhaseNumericStep::TransientBound;
    let numerator = bound(debt.error, lower, FixtureChartBoundOperation::Divide, step)?;
    let magnitude = bound(
        debt.point.abs(),
        seconds,
        FixtureChartBoundOperation::Divide,
        step,
    )?;
    let time_product = bound(
        magnitude,
        time_error,
        FixtureChartBoundOperation::Multiply,
        step,
    )?;
    let denominator = bound(
        time_product,
        lower,
        FixtureChartBoundOperation::Divide,
        step,
    )?;
    let rounding = bound(
        UNIT_ROUNDOFF,
        magnitude,
        FixtureChartBoundOperation::Multiply,
        step,
    )?;
    sum_four_up(numerator, denominator, rounding, ETA_UPPER)
        .map_err(|source| FixturePhaseNumericError::Bound { step, source })
}

#[cfg(test)]
#[path = "../../../../tests/private/phase_plan_numeric.rs"]
mod tests;

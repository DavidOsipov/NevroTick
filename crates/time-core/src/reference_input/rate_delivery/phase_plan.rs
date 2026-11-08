/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Miroslav Lichvar  2014-2015
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

  =======================================================================

*/
// SPDX-License-Identifier: GPL-2.0-only
//! Owning finite modeled phase obligations, adapted from Chrony 4.9 accounting.
//!
//! David Osipov with AI assistance, 8 November 2026: the achieved-total-minus-base
//! accounting order is adapted from `sys_generic.c`. This is a modified pure Rust
//! fixture, not a clean-room claim or a translation of the native controller.
//! One exclusive producer lease and persistent cursor prevent local duplicate
//! issuance. Known delivery commits before any replacement failure. No retained heap state,
//! clocks, I/O, actual acknowledgement or physical mapping is supplied here.
//! Failed Garde admission may allocate its bounded temporary validation report.

use std::{fmt, num::NonZeroU64};

use garde::Validate;
use time_domain::{ActiveAwakeDurationNs, ActiveAwakeNs, MonotonicDurationNs, MonotonicNs};

use super::{
    FixtureFrequencyAdmission, FixtureFrequencyClockAssociation, FixtureFrequencyError,
    FixtureFrequencyHistory, FixtureFrequencyProfile, FixtureFrequencyProjection, FixtureRateClock,
    FixtureRateClockContext, FixtureRateCursorStatus, FixtureRateDeliveryError,
    FixtureRateSettlement, FixtureRateSettlementCursor, FixtureRateSettlementKind,
    FixtureReferenceInputState, frequency_prior::FrequencyState,
};
use crate::{
    FixtureAffineChart, FixtureChartBatch, FixtureChartCombinationError, FixtureChartError,
    FixtureChartFrames, FixtureCombinationState, FixtureContext, FixturePrincipalStratum,
    FixtureReferenceInputError,
    regression::{FixturePhaseNumericError, PhaseDebt, PhaseRates},
};

/// Typed fixture readings; their context is asserted, never a clock supplier.
#[derive(Clone, Copy)]
pub struct FixturePhaseNow {
    /// Caller-asserted association shared by these two independent clock readings.
    context: FixtureRateClockContext,
    /// Supplied active-awake nanoseconds; only same-domain ordering is checked.
    active: ActiveAwakeNs,
    /// Supplied monotonic nanoseconds, never substituted for elapsed delivery.
    monotonic: MonotonicNs,
}

impl FixturePhaseNow {
    /// Retain already unit-tagged readings; current ordering belongs to receivers.
    #[must_use]
    pub const fn new(
        context: FixtureRateClockContext,
        active: ActiveAwakeNs,
        monotonic: MonotonicNs,
    ) -> Self {
        Self {
            context,
            active,
            monotonic,
        }
    }

    /// Copy the original active-awake reading without clock conversion.
    #[must_use]
    pub const fn active(&self) -> ActiveAwakeNs {
        self.active
    }

    /// Copy the original independent monotonic reading.
    #[must_use]
    pub const fn monotonic(&self) -> MonotonicNs {
        self.monotonic
    }
}

impl fmt::Debug for FixturePhaseNow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FixturePhaseNow { .. }")
    }
}

/// Known modeled effective baseline, not a requested or native frequency.
///
/// Checked construction retains signed zero and rejects unsupported raw values:
///
/// ```
/// use time_core::FixturePhaseBaseline;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let baseline = FixturePhaseBaseline::new(-0.0)?;
/// assert_eq!(baseline.rate().to_bits(), (-0.0_f64).to_bits());
/// assert!(FixturePhaseBaseline::new(f64::NAN).is_err());
/// assert_eq!(FixturePhaseBaseline::new(1.0 / 1024.0)?.rate(), 1.0 / 1024.0);
/// # Ok(()) }
/// ```
#[derive(Clone, Copy)]
pub struct FixturePhaseBaseline {
    /// Checked finite normal-or-zero effective baseline in seconds per active second.
    rate: f64,
}

#[derive(Validate)]
/// Mandatory scalar admission for otherwise untrusted effective-baseline bits.
struct RawBaseline {
    #[garde(custom(|value: &f64, (): &()| {
        FixtureAffineChart::check_delivery_rate(*value)
            .map_err(|_| garde::Error::new("unsupported fixture baseline"))
    }))]
    /// Untrusted stored baseline bits checked by the shared delivery-rate rule.
    rate: f64,
}

impl FixturePhaseBaseline {
    /// Admit supported stored rate bits through the existing scalar owner.
    /// # Errors
    /// Preserve unsupported rate or mandatory-validation disagreement.
    pub fn new(rate: f64) -> Result<Self, FixturePhaseError> {
        let raw = RawBaseline { rate };
        let validated = raw.validate();
        FixtureAffineChart::check_delivery_rate(rate)
            .map_err(|source| FixturePhaseError::Chart { source })?;
        validated.map_err(|_| FixturePhaseError::ValidationInvariant)?;
        Ok(Self { rate })
    }

    /// Copy modeled seconds per active-awake second, including signed-zero bits.
    #[must_use]
    pub const fn rate(&self) -> f64 {
        self.rate
    }
}

impl fmt::Debug for FixturePhaseBaseline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FixturePhaseBaseline { .. }")
    }
}

/// Explicit finite fixture parameters; no shipping default is selected.
#[derive(Clone, Copy)]
pub struct FixturePhaseProfile {
    /// Positive finite active allowance and initial proposal denominator.
    active_horizon: ActiveAwakeDurationNs,
    /// Positive independent monotonic expiry interval; no clock conversion is implied.
    monotonic_horizon: MonotonicDurationNs,
    /// Explicit positive cap on the magnitude of a useful phase-debt point.
    phase_cap: f64,
    /// Explicit positive cap on the requested transient rate alone.
    transient_cap: f64,
    /// Explicit positive cap on the aggregate baseline-plus-transient proposal.
    aggregate_cap: f64,
}

/// Check the explicit positive finite profile without mutation.
fn profile_valid(value: &FixturePhaseProfile) -> bool {
    value.active_horizon.nanoseconds() > 0
        && value.active_horizon.nanoseconds() <= (1_u64 << 53)
        && value.monotonic_horizon.nanoseconds() > 0
        && [value.phase_cap, value.transient_cap, value.aggregate_cap]
            .iter()
            .all(|v| v.is_normal() && *v > 0.0)
}

#[derive(Validate)]
/// Mandatory shared admission of the complete explicit finite profile.
struct RawProfile {
    #[garde(custom(|value: &FixturePhaseProfile, (): &()| {
        if profile_valid(value) { Ok(()) } else { Err(garde::Error::new("invalid finite phase profile")) }
    }))]
    /// Complete profile group whose positivity and supported horizon share one rule.
    value: FixturePhaseProfile,
}

impl FixturePhaseProfile {
    /// Admit positive horizons and finite positive normal point/rate caps.
    /// # Errors
    /// Reject zero/unsupported horizons, invalid caps or validation disagreement.
    pub fn new(
        active_horizon: ActiveAwakeDurationNs,
        monotonic_horizon: MonotonicDurationNs,
        phase_cap: f64,
        transient_cap: f64,
        aggregate_cap: f64,
    ) -> Result<Self, FixturePhaseError> {
        let raw = RawProfile {
            value: Self {
                active_horizon,
                monotonic_horizon,
                phase_cap,
                transient_cap,
                aggregate_cap,
            },
        };
        let validated = raw.validate();
        if !profile_valid(&raw.value) {
            return Err(FixturePhaseError::InvalidProfile);
        }
        validated.map_err(|_| FixturePhaseError::ValidationInvariant)?;
        Ok(raw.value)
    }
}

impl fmt::Debug for FixturePhaseProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FixturePhaseProfile { .. }")
    }
}

/// Explicit conditional permission to continue old evidence without fresh credit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixturePhaseResidualPermission {
    /// Explicit fixture permission; no native authority is inferred.
    Allow,
    /// No continuation from old evidence may be issued.
    Deny,
}

/// Terminal proposal-authority retirement; historical settlement remains available.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixturePhaseRetirementReason {
    /// Explicit stop permanently retires useful proposals.
    Stop,
    /// Conflicting current evidence retires useful proposals.
    Conflict,
    /// Current generation no longer permits useful proposals.
    Generation,
}

/// Bounded lifecycle refusal, retaining typed causes rather than inputs.
#[derive(PartialEq)]
pub enum FixturePhaseError {
    /// The enabled model rejected its explicit clock/profile association.
    Frequency {
        /// Bounded conditional-model cause with no raw labels or input values.
        source: FixtureFrequencyError,
    },
    /// An explicit horizon or numerical cap is unsupported.
    InvalidProfile,
    /// Mandatory validation disagreed with its shared typed rule.
    ValidationInvariant,
    /// The supplied fixture clock association differs.
    ClockContextMismatch,
    /// An independently ordered clock moved behind known readings.
    TimeRegression {
        /// Independently ordered clock whose current value was rejected.
        clock: FixtureRateClock,
    },
    /// Adding an explicit finite horizon overflowed its own clock.
    ClockOverflow,
    /// The local settlement cursor is unresolved or exhausted.
    CursorUnavailable,
    /// The actual chart does not match the cursor target tags.
    CursorMappingMismatch,
    /// One issued operation must settle before another action.
    OutstandingOperation,
    /// No issued operation owns this terminal result.
    NoOutstandingOperation,
    /// No usable admitted phase debt exists.
    NoReference,
    /// An original active or monotonic deadline was reached.
    Expired,
    /// Useful proposal authority was explicitly retired.
    Retired {
        /// Original closed retirement reason.
        reason: FixturePhaseRetirementReason,
    },
    /// Conditional permission to reuse old evidence is absent.
    ResidualPermission,
    /// No known effective baseline accompanies actual delivery.
    BaselineUnavailable,
    /// The declared effective baseline changed during this obligation.
    BaselineChanged,
    /// The signed point exceeds the explicit phase cap.
    PhaseEnvelope,
    /// Known delivered aggregate exceeds the explicit cap.
    DeliveredOutOfEnvelope,
    /// Known transient delivery opposes the original debt direction.
    DeliveredOppositeDirection,
    /// Known transient delivery crossed the debt sign.
    DeliveredOvershoot,
    /// The producer and cursor already left this retirement.
    TransferConsumed,
    /// The existing chart owner rejected a checked transformation.
    Chart {
        /// Original bounded typed cause without input payloads.
        source: FixtureChartError,
    },
    /// The actual reference receiver rejected an outcome.
    Reference {
        /// Original bounded typed cause without input payloads.
        source: FixtureReferenceInputError,
    },
    /// The actual settled-rate receiver rejected the event.
    Delivery {
        /// Original bounded typed cause without input payloads.
        source: FixtureRateDeliveryError,
    },
    /// The leased producer rejected its actual batch transition.
    Combination {
        /// Original bounded typed cause without input payloads.
        source: FixtureChartCombinationError,
    },
    /// The numerical owner rejected debt or proposal arithmetic.
    Numeric {
        /// Original bounded typed cause without input payloads.
        source: FixturePhaseNumericError,
    },
}

impl fmt::Debug for FixturePhaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FixturePhaseError { .. }")
    }
}

impl fmt::Display for FixturePhaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Frequency { .. } => "fixture phase: frequency association rejected",
            Self::InvalidProfile => "fixture phase: invalid profile",
            Self::ValidationInvariant => "fixture phase: validation invariant",
            Self::ClockContextMismatch => "fixture phase: clock context mismatch",
            Self::TimeRegression { .. } => "fixture phase: clock moved backwards",
            Self::ClockOverflow => "fixture phase: deadline overflow",
            Self::CursorUnavailable => "fixture phase: cursor unavailable",
            Self::CursorMappingMismatch => "fixture phase: cursor mapping mismatch",
            Self::OutstandingOperation => "fixture phase: operation outstanding",
            Self::NoOutstandingOperation => "fixture phase: no operation outstanding",
            Self::NoReference => "fixture phase: no admitted reference",
            Self::Expired => "fixture phase: deadline reached",
            Self::Retired { .. } => "fixture phase: proposal authority retired",
            Self::ResidualPermission => "fixture phase: residual permission absent",
            Self::BaselineUnavailable => "fixture phase: baseline unavailable",
            Self::BaselineChanged => "fixture phase: baseline changed",
            Self::PhaseEnvelope => "fixture phase: phase envelope exceeded",
            Self::DeliveredOutOfEnvelope => "fixture phase: delivered rate exceeded envelope",
            Self::DeliveredOppositeDirection => "fixture phase: opposite transient delivered",
            Self::DeliveredOvershoot => "fixture phase: transient overshot",
            Self::TransferConsumed => "fixture phase: continuation already transferred",
            Self::Chart { .. } => "fixture phase: chart rejected",
            Self::Reference { .. } => "fixture phase: reference rejected",
            Self::Delivery { .. } => "fixture phase: delivery rejected",
            Self::Combination { .. } => "fixture phase: producer rejected",
            Self::Numeric { .. } => "fixture phase: arithmetic rejected",
        })
    }
}

impl std::error::Error for FixturePhaseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Frequency { source } => Some(source),
            Self::Chart { source } => Some(source),
            Self::Reference { source } => Some(source),
            Self::Delivery { source } => Some(source),
            Self::Combination { source } => Some(source),
            Self::Numeric { source } => Some(source),
            _ => None,
        }
    }
}

/// Original checked emission metadata, preserved without fresh credit or ages.
#[derive(Clone, Copy)]
pub struct FixturePhaseProvenance {
    /// Original admitted positive-behind phase in seconds, not the current remainder.
    phase: f64,
    /// Original numerical mean-roundoff bound, distinct from source statistics.
    error: f64,
    /// Actual committed local producer emission, never caller-supplied issuance credit.
    round: NonZeroU64,
    /// Original source assertions retained independently of mathematical remapping.
    context: FixtureContext,
    /// Checked mathematical frame and generation tags at reference acceptance.
    frames: FixtureChartFrames,
    /// Original anchor observation time, unchanged by plan or delivery transitions.
    anchor: MonotonicNs,
    /// Original latest source observation time; continuation cannot refresh its age.
    latest: MonotonicNs,
    /// Original fitted reference instant associated with this genuine estimate.
    fit_reference: MonotonicNs,
    /// Actual common evaluation instant from the admitted producer outcome.
    projected_at: MonotonicNs,
}

impl FixturePhaseProvenance {
    /// Original admitted fitted phase in seconds, not the mutable remainder.
    #[must_use]
    pub const fn phase_seconds(&self) -> f64 {
        self.phase
    }
    /// Original numerical mean error, separate from statistics.
    #[must_use]
    pub const fn phase_error_seconds(&self) -> f64 {
        self.error
    }
    /// Actual local emission ordinal of the exclusively leased producer.
    #[must_use]
    pub const fn emitted_round(&self) -> NonZeroU64 {
        self.round
    }
    /// Original source context without remapping or source authority.
    #[must_use]
    pub const fn source_context(&self) -> FixtureContext {
        self.context
    }
    /// Mathematical descriptor tags at acceptance.
    #[must_use]
    pub const fn frames(&self) -> FixtureChartFrames {
        self.frames
    }
    /// Original anchor reading, never refreshed by delivery.
    #[must_use]
    pub const fn anchor_observed_at(&self) -> MonotonicNs {
        self.anchor
    }
    /// Original latest observation reading.
    #[must_use]
    pub const fn latest_observed_at(&self) -> MonotonicNs {
        self.latest
    }
    /// Original fitted reference reading.
    #[must_use]
    pub const fn fit_reference(&self) -> MonotonicNs {
        self.fit_reference
    }
    /// Actual common projection evaluation reading.
    #[must_use]
    pub const fn projected_at(&self) -> MonotonicNs {
        self.projected_at
    }
}

impl fmt::Debug for FixturePhaseProvenance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FixturePhaseProvenance { .. }")
    }
}

#[derive(Clone, Copy)]
/// One carried numerical debt and its immutable original deadline budget.
struct Obligation {
    /// Current carried debt point and its accumulated deterministic numerical bound.
    debt: PhaseDebt,
    /// Original checked active deadline, retained across partial delivery and resume.
    active_deadline: ActiveAwakeNs,
    /// Original independent monotonic expiry, never restarted by continuation.
    monotonic_deadline: MonotonicNs,
    /// Whether another issue requires explicit permission to reuse old evidence.
    residual: bool,
    /// Latched expiry of this obligation; valid later observations cannot rewind it.
    expired: bool,
}

#[derive(Clone, Copy)]
/// One checked local proposal, distinct from any modeled applied interval.
struct Issued {
    /// Exact pending cursor ordinal assigned to this one issued operation.
    ordinal: NonZeroU64,
    /// Original typed issuance readings used to reject predating terminal spans.
    started: FixturePhaseNow,
    /// Checked requested rates and bounds; these values never acknowledge delivery.
    rates: PhaseRates,
}

/// An immutable view of one actual issued operation; it borrows its owner.
pub struct FixturePhaseProposal<'view> {
    /// Borrow of the actual immutable issued record owned by the live lifecycle.
    issued: &'view Issued,
    /// Borrow of its original finite obligation and unchanged deadline metadata.
    obligation: &'view Obligation,
}

impl FixturePhaseProposal<'_> {
    /// Existing cursor identity reserved for this issued operation.
    #[must_use]
    pub const fn ordinal(&self) -> NonZeroU64 {
        self.issued.ordinal
    }
    /// Requested transient, never a delivery acknowledgement.
    #[must_use]
    pub fn transient_rate(&self) -> f64 {
        self.issued.rates.transient_rate()
    }
    /// Numerical error of the transient proposal alone.
    #[must_use]
    pub fn transient_error(&self) -> f64 {
        self.issued.rates.transient_error()
    }
    /// Aggregate proposal including the separately retained baseline.
    #[must_use]
    pub fn aggregate_rate(&self) -> f64 {
        self.issued.rates.aggregate_rate()
    }
    /// Numerical aggregate addition/proposal bound.
    #[must_use]
    pub fn aggregate_error(&self) -> f64 {
        self.issued.rates.aggregate_error()
    }
    /// Whether this view proposes a numerically sign-established transient.
    #[must_use]
    pub fn has_useful_transient(&self) -> bool {
        self.issued.rates.has_useful_transient()
    }
    /// Original active deadline, unchanged by cancellation or continuation.
    #[must_use]
    pub const fn active_deadline(&self) -> ActiveAwakeNs {
        self.obligation.active_deadline
    }
    /// Original independent monotonic expiry.
    #[must_use]
    pub const fn monotonic_deadline(&self) -> MonotonicNs {
        self.obligation.monotonic_deadline
    }
}

impl fmt::Debug for FixturePhaseProposal<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FixturePhaseProposal { .. }")
    }
}

/// Producer admission committed independently of useful phase-plan admission.
pub enum FixturePhaseReferenceAdmission {
    /// A genuine new emission supplied a checked finite obligation.
    Accepted {
        /// Actual emitted producer round, not caller-supplied credit.
        emitted_round: NonZeroU64,
    },
    /// Producer state committed, while no useful finite plan was admitted.
    AdmittedNoPlan {
        /// Actual producer emission if this committed result emitted one.
        emitted_round: Option<NonZeroU64>,
        /// Bounded phase admission failure after the producer commit.
        reason: FixturePhaseError,
    },
}

impl fmt::Debug for FixturePhaseReferenceAdmission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FixturePhaseReferenceAdmission { .. }")
    }
}

/// Failed construction returns the supplied persistent cursor without resetting it.
pub struct FixturePhaseConstructionRejected {
    /// The supplied persistent cursor, returned intact rather than reconstructed.
    cursor: FixtureRateSettlementCursor,
    /// Compact construction-only refusal that retains no chart or producer payload.
    error: ConstructionFailure,
}

#[derive(Clone, Copy)]
/// Construction-only refusal codes keep the cursor-returning error bounded.
/// No branch owns producer state, charts, reference payloads or validation reports.
enum ConstructionFailure {
    /// Enabled model construction rejected the actual monotonic clock assertion.
    FrequencyClock,
    /// Enabled model construction rejected the original source/chart assertion.
    FrequencyAssociation,
    /// Enabled model construction rejected the actual cursor status/mapping.
    FrequencyCursor,
    /// Enabled model construction disagreed with its mandatory invariant rule.
    FrequencyInvariant,
    /// Supplied clock association differs from the actual cursor context.
    ClockContext,
    /// Active time precedes the cursor's last known active endpoint.
    ActiveOrder,
    /// Monotonic time precedes its independently known endpoint.
    MonotonicOrder,
    /// An explicit initial deadline cannot be represented in its clock domain.
    Overflow,
    /// The supplied cursor is unresolved or exhausted.
    Cursor,
    /// Supplied chart target tags differ from the actual cursor target.
    Mapping,
    /// Known effective baseline exceeds the explicit aggregate point cap.
    Baseline,
    /// Mandatory validation or an unexpected construction-only family disagreed.
    Invariant,
}

impl ConstructionFailure {
    /// Keep only the four actual enabled-construction causes beside the cursor.
    /// Numerical and quality errors cannot arise before any model exists; an
    /// unexpected such result is a construction invariant disagreement.
    fn from_frequency_error(error: FixtureFrequencyError) -> Self {
        match error {
            FixtureFrequencyError::ClockContextMismatch => Self::FrequencyClock,
            FixtureFrequencyError::ClockAssociationMismatch => Self::FrequencyAssociation,
            FixtureFrequencyError::CursorUnavailable => Self::FrequencyCursor,
            _ => Self::FrequencyInvariant,
        }
    }

    /// Project a construction-time refusal into its bounded stored family.
    /// The borrowed cause is neither mutated nor retained.
    fn from_error(error: &FixturePhaseError) -> Self {
        match error {
            FixturePhaseError::Frequency { source } => Self::from_frequency_error(*source),
            FixturePhaseError::ClockContextMismatch => Self::ClockContext,
            FixturePhaseError::TimeRegression {
                clock: FixtureRateClock::ActiveAwake,
            } => Self::ActiveOrder,
            FixturePhaseError::TimeRegression {
                clock: FixtureRateClock::Monotonic,
            } => Self::MonotonicOrder,
            FixturePhaseError::ClockOverflow => Self::Overflow,
            FixturePhaseError::CursorUnavailable => Self::Cursor,
            FixturePhaseError::CursorMappingMismatch => Self::Mapping,
            FixturePhaseError::DeliveredOutOfEnvelope => Self::Baseline,
            _ => Self::Invariant,
        }
    }
    /// Expand the stored construction refusal back to its exact lifecycle family.
    /// This changes no cursor, producer, clock reading or useful-plan state.
    fn into_error(self) -> FixturePhaseError {
        match self {
            Self::FrequencyClock => FixturePhaseError::Frequency {
                source: FixtureFrequencyError::ClockContextMismatch,
            },
            Self::FrequencyAssociation => FixturePhaseError::Frequency {
                source: FixtureFrequencyError::ClockAssociationMismatch,
            },
            Self::FrequencyCursor => FixturePhaseError::Frequency {
                source: FixtureFrequencyError::CursorUnavailable,
            },
            Self::FrequencyInvariant => FixturePhaseError::Frequency {
                source: FixtureFrequencyError::ValidationInvariant,
            },
            Self::ClockContext => FixturePhaseError::ClockContextMismatch,
            Self::ActiveOrder => FixturePhaseError::TimeRegression {
                clock: FixtureRateClock::ActiveAwake,
            },
            Self::MonotonicOrder => FixturePhaseError::TimeRegression {
                clock: FixtureRateClock::Monotonic,
            },
            Self::Overflow => FixturePhaseError::ClockOverflow,
            Self::Cursor => FixturePhaseError::CursorUnavailable,
            Self::Mapping => FixturePhaseError::CursorMappingMismatch,
            Self::Baseline => FixturePhaseError::DeliveredOutOfEnvelope,
            Self::Invariant => FixturePhaseError::ValidationInvariant,
        }
    }
}

impl FixturePhaseConstructionRejected {
    /// Recover the unchanged owned cursor and exact bounded rejection.
    #[must_use]
    pub fn into_parts(self) -> (FixtureRateSettlementCursor, FixturePhaseError) {
        (self.cursor, self.error.into_error())
    }
}

impl fmt::Debug for FixturePhaseConstructionRejected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FixturePhaseConstructionRejected { .. }")
    }
}
impl fmt::Display for FixturePhaseConstructionRejected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("fixture phase: construction rejected")
    }
}
impl std::error::Error for FixturePhaseConstructionRejected {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

/// One current mathematical chart and exclusive local producer/settlement custody.
///
/// The leased producer cannot be independently inspected or advanced while the
/// owner remains live:
///
/// ```compile_fail,E0502
/// use time_core::{FixtureAffineChart, FixtureCombinationState, FixturePhaseBaseline,
///     FixturePhaseNow, FixturePhasePlanOwner, FixturePhaseProfile,
///     FixtureRateSettlementCursor};
/// fn overlap(producer: &mut FixtureCombinationState, chart: &FixtureAffineChart,
///     cursor: FixtureRateSettlementCursor, now: FixturePhaseNow,
///     profile: FixturePhaseProfile, baseline: FixturePhaseBaseline)
///     -> Result<(), Box<dyn std::error::Error>> {
///     let owner = FixturePhasePlanOwner::new(producer, chart, cursor, now, profile, baseline)?;
///     std::hint::black_box(producer.emitted_rounds());
///     std::hint::black_box(owner);
///     Ok(())
/// }
/// ```
///
/// A proposal view cannot survive the consuming terminal transition:
///
/// ```compile_fail,E0505
/// use time_core::{FixturePhaseBaseline, FixturePhaseError, FixturePhaseNow,
///     FixturePhasePlanOwner, FixturePhaseResidualPermission, FixtureRateSettlement};
/// fn stale_view(mut owner: FixturePhasePlanOwner<'_, '_>, event: FixtureRateSettlement,
///     baseline: FixturePhaseBaseline, now: FixturePhaseNow) -> Result<(), FixturePhaseError> {
///     let proposal = owner.issue(now, FixturePhaseResidualPermission::Allow)?;
///     let settled = owner.settle(event, Some(baseline), now);
///     std::hint::black_box(proposal.ordinal());
///     std::hint::black_box(settled);
///     Ok(())
/// }
/// ```
pub struct FixturePhasePlanOwner<'producer, 'chart> {
    /// Same-owner optional feature mode; enabled assimilation cannot be bypassed.
    frequency: FrequencyState,
    /// Exclusive actual producer lease preventing concurrent emission-credit changes.
    producer: &'producer mut FixtureCombinationState,
    /// Local readiness bound to the exact currently borrowed mathematical chart.
    readiness: FixtureReferenceInputState<'chart>,
    /// Persistent operation ordinal, target labels and known endpoint high-water marks.
    cursor: FixtureRateSettlementCursor,
    /// Checked finite fixture caps and horizon lengths, without shipping defaults.
    profile: FixturePhaseProfile,
    /// Known modeled effective baseline used for this obligation and its proposals.
    baseline: FixturePhaseBaseline,
    /// Last admitted clock readings; valid observations survive later plan refusal.
    last_now: FixturePhaseNow,
    /// Monotone stop, conflict or generation retirement of useful proposal authority.
    authority: Option<FixturePhaseRetirementReason>,
    /// Optional useful debt and original deadlines; absent for a context-only owner.
    obligation: Option<Obligation>,
    /// Original genuine reference metadata retained without creating another emission.
    provenance: Option<FixturePhaseProvenance>,
    /// Last known numerical debt, retained as history when useful admission fails.
    historical_debt: Option<PhaseDebt>,
    /// At most one issued operation, retained until its matching terminal admission.
    issued: Option<Issued>,
}

impl fmt::Debug for FixturePhasePlanOwner<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FixturePhasePlanOwner { .. }")
    }
}

/// Borrowed current-clock admission context; checking changes neither owner.
struct NowRelationship<'value> {
    /// Last admitted current readings supplying independent clock-order lower bounds.
    previous: FixturePhaseNow,
    /// Actual cursor supplying immutable context and known terminal endpoint bounds.
    cursor: &'value FixtureRateSettlementCursor,
    /// New supplied typed readings whose relationship is being checked.
    now: FixturePhaseNow,
}

/// Check both current clock orders and context without converting clocks.
/// # Errors
/// Reject foreign context first, then active or monotonic regression against
/// the prior readings and independently known cursor endpoints. No mutation.
fn now_relationship(value: &NowRelationship<'_>) -> Result<(), FixturePhaseError> {
    if value.now.context != value.cursor.context || value.now.context != value.previous.context {
        return Err(FixturePhaseError::ClockContextMismatch);
    }
    if value.now.active < value.previous.active
        || value
            .cursor
            .last_active
            .is_some_and(|last| value.now.active < last)
    {
        return Err(FixturePhaseError::TimeRegression {
            clock: FixtureRateClock::ActiveAwake,
        });
    }
    if value.now.monotonic < value.previous.monotonic
        || value
            .cursor
            .last_monotonic
            .is_some_and(|last| value.now.monotonic < last)
    {
        return Err(FixturePhaseError::TimeRegression {
            clock: FixtureRateClock::Monotonic,
        });
    }
    Ok(())
}

#[derive(Validate)]
/// Mandatory Garde adapter for the same independent-clock relationship.
struct RawNow<'value> {
    #[garde(custom(|value: &NowRelationship<'_>, (): &()| {
        now_relationship(value).map_err(|_| garde::Error::new("invalid current fixture clocks"))
    }))]
    /// Whole current-clock relationship validated by the same rule as the typed path.
    value: NowRelationship<'value>,
}

/// Consume mandatory Garde and the same current-clock relationship.
/// # Errors
/// Preserve the shared context/order refusal, or report mandatory Garde
/// disagreement. Both clock domains and the borrowed cursor remain unchanged.
fn check_now(
    previous: FixturePhaseNow,
    cursor: &FixtureRateSettlementCursor,
    now: FixturePhaseNow,
) -> Result<(), FixturePhaseError> {
    let raw = RawNow {
        value: NowRelationship {
            previous,
            cursor,
            now,
        },
    };
    let validated = raw.validate();
    now_relationship(&raw.value)?;
    validated.map_err(|_| FixturePhaseError::ValidationInvariant)
}

/// Compute two independent checked deadlines without state mutation.
/// # Errors
/// Reject checked addition overflow in active time before monotonic time.
/// No deadline, owner or input reading is published or changed on failure.
fn deadlines(
    profile: FixturePhaseProfile,
    now: FixturePhaseNow,
) -> Result<(ActiveAwakeNs, MonotonicNs), FixturePhaseError> {
    let active = now
        .active
        .checked_add(profile.active_horizon)
        .map_err(|_| FixturePhaseError::ClockOverflow)?;
    let monotonic = now
        .monotonic
        .checked_add(profile.monotonic_horizon)
        .map_err(|_| FixturePhaseError::ClockOverflow)?;
    Ok((active, monotonic))
}

/// Require a pending cursor bound to the actual mathematical target.
/// # Errors
/// Reject an unresolved/exhausted cursor before unequal mathematical target
/// frame or generation tags; no ordering between generation labels is inferred.
fn cursor_mapping(
    cursor: &FixtureRateSettlementCursor,
    chart: &FixtureAffineChart,
) -> Result<(), FixturePhaseError> {
    if cursor.status() != FixtureRateCursorStatus::Pending {
        return Err(FixturePhaseError::CursorUnavailable);
    }
    let frames = chart.frames();
    if cursor.target_frame != frames.target_frame()
        || cursor.target_generation != frames.target_generation()
    {
        return Err(FixturePhaseError::CursorMappingMismatch);
    }
    Ok(())
}

impl<'producer, 'chart> FixturePhasePlanOwner<'producer, 'chart> {
    /// Lease the actual producer and retain an unchanged local settlement cursor.
    /// # Errors
    /// Return the supplied cursor on invalid clock/mapping/baseline/deadline input.
    pub fn new(
        producer: &'producer mut FixtureCombinationState,
        chart: &'chart FixtureAffineChart,
        cursor: FixtureRateSettlementCursor,
        now: FixturePhaseNow,
        profile: FixturePhaseProfile,
        baseline: FixturePhaseBaseline,
    ) -> Result<Self, FixturePhaseConstructionRejected> {
        let admitted = check_now(now, &cursor, now)
            .and_then(|()| cursor_mapping(&cursor, chart))
            .and_then(|()| deadlines(profile, now).map(|_| ()))
            .and_then(|()| {
                if baseline.rate.abs() <= profile.aggregate_cap {
                    Ok(())
                } else {
                    Err(FixturePhaseError::DeliveredOutOfEnvelope)
                }
            });
        if let Err(error) = admitted {
            return Err(FixturePhaseConstructionRejected {
                cursor,
                error: ConstructionFailure::from_error(&error),
            });
        }
        Ok(Self {
            frequency: FrequencyState::disabled(),
            producer,
            readiness: FixtureReferenceInputState::new(chart),
            cursor,
            profile,
            baseline,
            last_now: now,
            authority: None,
            obligation: None,
            provenance: None,
            historical_debt: None,
            issued: None,
        })
    }

    /// Construct the same exclusive owner with compulsory frequency assimilation.
    ///
    /// The explicit immutable association is a fixture assertion, not a native
    /// clock mapping. Both constructor paths retain the same cursor/producer
    /// custody. There is no later public enable or raw model setter.
    /// # Errors
    /// Preserve the supplied cursor on association or existing phase admission
    /// failure; no producer advancement or model observation occurs.
    #[expect(
        clippy::too_many_arguments,
        reason = "Both independent profiles and the explicit clock association are mandatory inputs; no ambient/default policy is selected."
    )]
    pub fn new_with_frequency(
        producer: &'producer mut FixtureCombinationState,
        chart: &'chart FixtureAffineChart,
        cursor: FixtureRateSettlementCursor,
        now: FixturePhaseNow,
        profile: FixturePhaseProfile,
        baseline: FixturePhaseBaseline,
        frequency_profile: FixtureFrequencyProfile,
        association: FixtureFrequencyClockAssociation,
    ) -> Result<Self, FixturePhaseConstructionRejected> {
        let frequency =
            match FrequencyState::enabled(frequency_profile, association, chart, &cursor) {
                Ok(frequency) => frequency,
                Err(source) => {
                    return Err(FixturePhaseConstructionRejected {
                        cursor,
                        error: ConstructionFailure::from_frequency_error(source),
                    });
                }
            };
        let mut owner = Self::new(producer, chart, cursor, now, profile, baseline)?;
        owner.frequency = frequency;
        Ok(owner)
    }

    /// Last actual source-admission outcome, separate from finite phase admission.
    #[must_use]
    pub fn frequency_admission(&self) -> FixtureFrequencyAdmission {
        self.frequency.admission()
    }

    /// Copy historical diagnostics without claiming current model availability.
    #[must_use]
    pub fn frequency_history(&self) -> Option<FixtureFrequencyHistory> {
        self.frequency.history()
    }

    /// Project the conditional model on the explicitly associated monotonic axis.
    ///
    /// Enabled valid current readings commit before later quality/pending/model
    /// rejection, so an earlier query cannot revive quality. This creates no
    /// fresh source, phase obligation, baseline acknowledgement or clock effect.
    /// # Errors
    /// Disabled/context/order failures change no state. Later refusals retain
    /// the valid time high-water and all issued-operation/history custody.
    ///
    /// Prediction is not a fresh-reference admission path:
    /// ```compile_fail,E0308
    /// use time_core::{FixtureFrequencyProjection, FixturePhaseNow,
    ///     FixturePhasePlanOwner, FixturePrincipalStratum};
    /// fn substitute(owner: &mut FixturePhasePlanOwner<'_, '_>,
    ///     prediction: FixtureFrequencyProjection, strata: &[FixturePrincipalStratum],
    ///     now: FixturePhaseNow) {
    ///     let _result = owner.receive_reference(&prediction, strata, now);
    /// }
    /// ```
    /// Nor can callers bypass genuine intake with a scalar learning method:
    /// ```compile_fail,E0599
    /// use time_core::FixturePhasePlanOwner;
    /// fn bypass(owner: &mut FixturePhasePlanOwner<'_, '_>) {
    ///     owner.learn(0.0, 0.0);
    /// }
    /// ```
    pub fn project_model(
        &mut self,
        now: FixturePhaseNow,
    ) -> Result<FixtureFrequencyProjection, FixtureFrequencyError> {
        if !self.frequency.is_enabled() {
            return Err(FixtureFrequencyError::Disabled);
        }
        check_now(self.last_now, &self.cursor, now).map_err(|error| match error {
            FixturePhaseError::ClockContextMismatch => FixtureFrequencyError::ClockContextMismatch,
            FixturePhaseError::TimeRegression { clock } => {
                FixtureFrequencyError::TimeRegression { clock }
            }
            _ => FixtureFrequencyError::ValidationInvariant,
        })?;
        self.last_now = now;
        self.frequency.project(
            self.readiness.chart,
            self.cursor.context,
            now.monotonic,
            self.issued.is_some(),
        )
    }

    /// Borrow the actual chart with its existing lifetime for genuine new batches.
    #[must_use]
    pub const fn chart(&self) -> &'chart FixtureAffineChart {
        self.readiness.chart
    }

    /// Borrow the persistent local cursor without a mutation/reset path.
    #[must_use]
    pub const fn cursor(&self) -> &FixtureRateSettlementCursor {
        &self.cursor
    }

    /// Last admitted current readings, independently ordered in their own domains.
    #[must_use]
    pub const fn current_time(&self) -> FixturePhaseNow {
        self.last_now
    }

    /// Original accepted reference metadata, never a manufactured new estimate.
    #[must_use]
    pub const fn provenance(&self) -> Option<&FixturePhaseProvenance> {
        self.provenance.as_ref()
    }

    /// Copy the useful current debt, absent in a context-only owner.
    #[must_use]
    pub fn remaining_phase_seconds(&self) -> Option<f64> {
        self.obligation.map(|v| v.debt.point())
    }

    /// Separate numerical enclosure of the current debt.
    #[must_use]
    pub fn remaining_phase_error_seconds(&self) -> Option<f64> {
        self.obligation.map(|v| v.debt.error())
    }

    /// Last known historical debt remains inspectable after useful admission fails.
    #[must_use]
    pub fn last_known_phase_seconds(&self) -> Option<f64> {
        self.historical_debt.map(|v| v.point())
    }

    /// Numerical bound of the historical debt, never a new source observation.
    #[must_use]
    pub fn last_known_phase_error_seconds(&self) -> Option<f64> {
        self.historical_debt.map(|v| v.error())
    }

    /// Original active deadline, absent before a useful reference obligation.
    #[must_use]
    pub fn active_deadline(&self) -> Option<ActiveAwakeNs> {
        self.obligation.map(|v| v.active_deadline)
    }

    /// Original independent monotonic expiry.
    #[must_use]
    pub fn monotonic_deadline(&self) -> Option<MonotonicNs> {
        self.obligation.map(|v| v.monotonic_deadline)
    }

    /// Retire proposal authority monotonically while retaining a late receiver.
    pub fn retire(&mut self, reason: FixturePhaseRetirementReason) {
        self.frequency.retire(reason);
        if self.authority.is_none() {
            self.authority = Some(reason);
        }
    }

    /// Check monotone proposal retirement without changing historical reception.
    /// # Errors
    /// Return the original stop, conflict or generation reason; never restore authority.
    fn authority_ready(&self) -> Result<(), FixturePhaseError> {
        if let Some(reason) = self.authority {
            return Err(FixturePhaseError::Retired { reason });
        }
        Ok(())
    }

    /// Commit genuine producer advancement, then separately admit one finite debt.
    /// # Errors
    /// Pre-admission rejects preserve the producer and owner. Post-advance phase
    /// refusal is `AdmittedNoPlan` and retains the producer's actual consumed credit.
    pub fn receive_reference<'batch, 'entries>(
        &mut self,
        batch: &'batch FixtureChartBatch<'entries, 'chart>,
        strata: &[FixturePrincipalStratum],
        now: FixturePhaseNow,
    ) -> Result<FixturePhaseReferenceAdmission, FixturePhaseError> {
        self.authority_ready()?;
        if self.issued.is_some() {
            return Err(FixturePhaseError::OutstandingOperation);
        }
        cursor_mapping(&self.cursor, self.chart())?;
        if !std::ptr::eq(self.chart(), batch.chart()) {
            return Err(FixturePhaseError::Reference {
                source: FixtureReferenceInputError::ForeignChartView,
            });
        }
        check_now(self.last_now, &self.cursor, now)?;
        let (active_deadline, monotonic_deadline) = deadlines(self.profile, now)?;
        let outcome = self
            .producer
            .advance_chart(self.readiness.chart, batch, strata)
            .map_err(|source| FixturePhaseError::Combination { source })?;
        self.last_now = now;
        self.obligation = None;
        let round = outcome.emitted_round();
        let original = {
            let token = match self.readiness.admit_reference_input(&outcome) {
                Ok(token) => token,
                Err(source) => {
                    self.frequency.no_ready(round);
                    return Ok(FixturePhaseReferenceAdmission::AdmittedNoPlan {
                        emitted_round: round,
                        reason: FixturePhaseError::Reference { source },
                    });
                }
            };
            let estimate = token.combined();
            self.frequency.assimilate(
                estimate,
                token.emitted_round(),
                self.cursor.context,
                now.monotonic,
            );
            FixturePhaseProvenance {
                phase: estimate.phase_seconds(),
                error: estimate.phase_mean_roundoff_seconds(),
                round: token.emitted_round(),
                context: estimate.source_context(),
                frames: estimate.chart().frames(),
                anchor: estimate.anchor_observed_at(),
                latest: estimate.latest_observed_at(),
                fit_reference: estimate.fit_reference(),
                projected_at: estimate.projected_at(),
            }
        };
        self.provenance = Some(original);
        let debt = match PhaseDebt::new(original.phase, original.error) {
            Ok(debt) => debt,
            Err(source) => {
                return Ok(FixturePhaseReferenceAdmission::AdmittedNoPlan {
                    emitted_round: round,
                    reason: FixturePhaseError::Numeric { source },
                });
            }
        };
        self.historical_debt = Some(debt);
        if debt.point().abs() > self.profile.phase_cap {
            return Ok(FixturePhaseReferenceAdmission::AdmittedNoPlan {
                emitted_round: round,
                reason: FixturePhaseError::PhaseEnvelope,
            });
        }
        self.obligation = Some(Obligation {
            debt,
            active_deadline,
            monotonic_deadline,
            residual: false,
            expired: false,
        });
        Ok(FixturePhaseReferenceAdmission::Accepted {
            emitted_round: original.round,
        })
    }

    /// Issue at most one operation through the original remaining active allowance.
    /// # Errors
    /// Reject absent/retired/expired debt, missing residual permission, duplicate
    /// issuance, changed clocks or numerical failure. Valid time is never rewound.
    pub fn issue(
        &mut self,
        now: FixturePhaseNow,
        permission: FixturePhaseResidualPermission,
    ) -> Result<FixturePhaseProposal<'_>, FixturePhaseError> {
        self.authority_ready()?;
        if self.issued.is_some() {
            return Err(FixturePhaseError::OutstandingOperation);
        }
        check_now(self.last_now, &self.cursor, now)?;
        self.last_now = now;
        let obligation = self
            .obligation
            .as_mut()
            .ok_or(FixturePhaseError::NoReference)?;
        if obligation.expired
            || now.active >= obligation.active_deadline
            || now.monotonic >= obligation.monotonic_deadline
        {
            obligation.expired = true;
            return Err(FixturePhaseError::Expired);
        }
        if obligation.residual && permission != FixturePhaseResidualPermission::Allow {
            return Err(FixturePhaseError::ResidualPermission);
        }
        let ordinal = self
            .cursor
            .next_ordinal()
            .ok_or(FixturePhaseError::CursorUnavailable)?;
        let remaining = obligation
            .active_deadline
            .duration_since(now.active)
            .map_err(|_| FixturePhaseError::Expired)?;
        let rates = obligation
            .debt
            .proposal(
                self.baseline.rate,
                remaining,
                self.profile.transient_cap,
                self.profile.aggregate_cap,
            )
            .map_err(|source| FixturePhaseError::Numeric { source })?;
        self.issued = Some(Issued {
            ordinal,
            started: now,
            rates,
        });
        self.proposal()
            .ok_or(FixturePhaseError::ValidationInvariant)
    }

    /// Borrow an already-issued proposal without minting another operation.
    #[must_use]
    pub fn proposal(&self) -> Option<FixturePhaseProposal<'_>> {
        if self.authority.is_some() || self.obligation.is_some_and(|v| v.expired) {
            return None;
        }
        Some(FixturePhaseProposal {
            issued: self.issued.as_ref()?,
            obligation: self.obligation.as_ref()?,
        })
    }
}

/// One consuming terminal path; only outer rejection returns an unchanged owner.
#[must_use]
#[expect(
    clippy::large_enum_variant,
    reason = "The bounded inline result retains exclusive late-result custody and immutable model history without introducing a success-path heap allocation; the largest variant is constant-size, not input-sized."
)]
pub enum FixturePhaseSettlement<'producer, 'chart> {
    /// The event was not admitted and ownership is returned intact.
    OuterRejected {
        /// Original owner, including the outstanding operation and producer lease.
        owner: FixturePhasePlanOwner<'producer, 'chart>,
        /// Exact pre-consumption refusal.
        error: FixturePhaseError,
    },
    /// The ordinal was consumed, with no invented numerical effect or endpoints.
    ProvenNoEffect(FixturePhasePlanOwner<'producer, 'chart>),
    /// Known delivery was committed; no predecessor chart lifetime is retained.
    Known(FixturePhaseRetirement<'producer>),
    /// Current Unknown consumed and blocked the local operation permanently.
    Unknown(FixturePhaseBlocked<'producer>),
}

impl fmt::Debug for FixturePhaseSettlement<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FixturePhaseSettlement { .. }")
    }
}

/// Owned transfer custody without any predecessor-chart borrow or useful proposal.
struct Continuation<'producer> {
    /// The same model/profile/clock association moves through the sole custody slot.
    frequency: FrequencyState,
    /// Exclusively leased producer transferred once, never cloned or reconstructed.
    producer: &'producer mut FixtureCombinationState,
    /// Actual advanced cursor retaining consumption and historical endpoint marks.
    cursor: FixtureRateSettlementCursor,
    /// Checked fixture limits preserved for residual or genuinely fresh admission.
    profile: FixturePhaseProfile,
    /// Prior known baseline; fresh admission must explicitly supply its current base.
    baseline: FixturePhaseBaseline,
    /// Last admitted current readings, including valid rejected-resume observations.
    last_now: FixturePhaseNow,
    /// Monotone retirement that neither transfer path may silently revive.
    authority: Option<FixturePhaseRetirementReason>,
}

/// Owned committed settlement, with one optional transfer slot and actual chart.
///
/// The first returned owner borrows the actual replacement, so another live
/// transfer cannot borrow the same retirement concurrently:
///
/// ```compile_fail,E0499
/// use time_core::{FixturePhaseBaseline, FixturePhaseError, FixturePhaseNow,
///     FixturePhaseResidualPermission, FixturePhaseRetirement};
/// fn concurrent(retired: &mut FixturePhaseRetirement<'_>, now: FixturePhaseNow,
///     baseline: FixturePhaseBaseline) -> Result<(), FixturePhaseError> {
///     let first = retired.resume(now, FixturePhaseResidualPermission::Allow)?;
///     let second = retired.resume_fresh(now, baseline)?;
///     std::hint::black_box((first, second));
///     Ok(())
/// }
/// ```
#[must_use]
pub struct FixturePhaseRetirement<'producer> {
    /// Historical model diagnostics survive consumption of the continuation slot.
    frequency_history: Option<FixtureFrequencyHistory>,
    /// Sole producer-and-cursor transfer slot; taking either path empties it.
    continuation: Option<Continuation<'producer>>,
    /// Original typed known terminal event, retained independently of arithmetic.
    event: FixtureRateSettlement,
    /// Actual owned replacement chart or the original post-consumption chart error.
    replacement: Result<FixtureAffineChart, FixtureChartError>,
    /// New accounted debt and bound, or its exact baseline/numerical failure.
    debt: Result<PhaseDebt, FixturePhaseError>,
    /// Post-accounting policy refusal without erasing a computed signed remainder.
    refusal: Option<FixturePhaseError>,
    /// Prior carried debt and deadlines, preserving history when new accounting fails.
    original: Option<Obligation>,
    /// Original accepted source evidence, independent of transfer-slot consumption.
    provenance: Option<FixturePhaseProvenance>,
}

impl fmt::Debug for FixturePhaseRetirement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FixturePhaseRetirement { .. }")
    }
}

/// Unknown terminal custody; no chart or useful continuation is manufactured.
#[must_use]
pub struct FixturePhaseBlocked<'producer> {
    /// Owned unresolved cursor and producer lease, with no recovery or issuance path.
    continuation: Continuation<'producer>,
    /// Actual current Unknown event whose ordinal was consumed and blocked.
    event: FixtureRateSettlement,
    /// Prior genuine source evidence retained strictly as historical metadata.
    provenance: Option<FixturePhaseProvenance>,
    /// Last known pre-Unknown debt; this cannot stand for unknown delivered phase.
    historical_debt: Option<PhaseDebt>,
}

impl FixturePhaseBlocked<'_> {
    /// Copy historical diagnostics; Unknown grants no useful model projection.
    #[must_use]
    pub fn frequency_history(&self) -> Option<FixtureFrequencyHistory> {
        self.continuation.frequency.history()
    }
    /// Borrow the actually consumed unresolved cursor without a reset path.
    #[must_use]
    pub const fn cursor(&self) -> &FixtureRateSettlementCursor {
        &self.continuation.cursor
    }
    /// Borrow the original typed Unknown event.
    #[must_use]
    pub const fn event(&self) -> &FixtureRateSettlement {
        &self.event
    }
    /// Retain prior accepted source metadata as historical evidence only.
    #[must_use]
    pub const fn provenance(&self) -> Option<&FixturePhaseProvenance> {
        self.provenance.as_ref()
    }
    /// Historical pre-Unknown debt is not a new current residual.
    #[must_use]
    pub fn last_known_phase_seconds(&self) -> Option<f64> {
        self.historical_debt.map(|v| v.point())
    }
    /// Historical numerical bound; Unknown supplies no new debt calculation.
    #[must_use]
    pub fn last_known_phase_error_seconds(&self) -> Option<f64> {
        self.historical_debt.map(|v| v.error())
    }
}

impl fmt::Debug for FixturePhaseBlocked<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FixturePhaseBlocked { .. }")
    }
}

/// Bind known terminal spans to the issued operation and supplied current readings.
/// # Errors
/// Reject a known span predating issuance or ending after supplied current
/// readings, active first. No-effect and Unknown invent no endpoints.
fn terminal_now(
    issued: Issued,
    event: FixtureRateSettlement,
    now: FixturePhaseNow,
) -> Result<(), FixturePhaseError> {
    if let FixtureRateSettlement::Completed(interval) | FixtureRateSettlement::Cancelled(interval) =
        event
    {
        if interval.span.active.0 < issued.started.active || now.active < interval.span.active.1 {
            return Err(FixturePhaseError::TimeRegression {
                clock: FixtureRateClock::ActiveAwake,
            });
        }
        if interval.span.effective.0 < issued.started.monotonic
            || now.monotonic < interval.span.effective.1
        {
            return Err(FixturePhaseError::TimeRegression {
                clock: FixtureRateClock::Monotonic,
            });
        }
    }
    Ok(())
}

/// Immutable issued-operation and terminal-span relationship before consumption.
struct TerminalRelationship {
    /// Actual issued record binding known spans to the local operation start.
    issued: Issued,
    /// Immutable typed terminal event; no requested rate or duration is substituted.
    event: FixtureRateSettlement,
    /// Supplied current readings required to contain each known same-domain end.
    now: FixturePhaseNow,
}

#[derive(Validate)]
/// Mandatory Garde adapter for the same issued-span admission rule.
struct RawTerminal {
    #[garde(custom(|value: &TerminalRelationship, (): &()| {
        terminal_now(value.issued, value.event, value.now)
            .map_err(|_| garde::Error::new("invalid issued terminal relationship"))
    }))]
    /// Complete issued-span relationship checked before any terminal consumption.
    value: TerminalRelationship,
}

/// Consume the shared issued-span relationship and its mandatory validation.
/// # Errors
/// Preserve the shared issued-span refusal or mandatory-validation
/// disagreement; no event, ordinal or endpoint has yet been consumed.
fn check_terminal(
    issued: Issued,
    event: FixtureRateSettlement,
    now: FixturePhaseNow,
) -> Result<(), FixturePhaseError> {
    let raw = RawTerminal {
        value: TerminalRelationship { issued, event, now },
    };
    let validated = raw.validate();
    terminal_now(issued, event, now)?;
    validated.map_err(|_| FixturePhaseError::ValidationInvariant)
}

/// Account only a known matching baseline and the newly delivered typed interval.
/// # Errors
/// Reject unavailable/changed baseline or missing carried debt/known interval;
/// otherwise preserve the numerical owner's exact post-settlement cause.
/// The caller already retains committed total delivery independently.
fn settled_debt(
    obligation: Option<Obligation>,
    event: FixtureRateSettlement,
    baseline: FixturePhaseBaseline,
    observed: Option<FixturePhaseBaseline>,
) -> Result<PhaseDebt, FixturePhaseError> {
    let known = observed.ok_or(FixturePhaseError::BaselineUnavailable)?;
    if known.rate.to_bits() != baseline.rate.to_bits() {
        return Err(FixturePhaseError::BaselineChanged);
    }
    let old = obligation.ok_or(FixturePhaseError::NoReference)?;
    let (FixtureRateSettlement::Completed(interval) | FixtureRateSettlement::Cancelled(interval)) =
        event
    else {
        return Err(FixturePhaseError::NoOutstandingOperation);
    };
    old.debt
        .settled(interval.rate, known.rate, interval.span.active_elapsed)
        .map_err(|source| FixturePhaseError::Numeric { source })
}

/// Classify post-accounting policy retirement without erasing its signed result.
fn continuation_refusal(
    profile: FixturePhaseProfile,
    baseline: FixturePhaseBaseline,
    obligation: Option<Obligation>,
    event: FixtureRateSettlement,
    debt: &Result<PhaseDebt, FixturePhaseError>,
    now: FixturePhaseNow,
) -> Option<FixturePhaseError> {
    let (Some(old), Ok(next)) = (obligation, debt) else {
        return None;
    };
    let (FixtureRateSettlement::Completed(interval) | FixtureRateSettlement::Cancelled(interval)) =
        event
    else {
        return None;
    };
    if interval.rate.abs() > profile.aggregate_cap {
        return Some(FixturePhaseError::DeliveredOutOfEnvelope);
    }
    if (old.debt.point() > 0.0 && interval.rate < baseline.rate)
        || (old.debt.point() < 0.0 && interval.rate > baseline.rate)
    {
        return Some(FixturePhaseError::DeliveredOppositeDirection);
    }
    if (old.debt.point() > 0.0 && next.point() < 0.0)
        || (old.debt.point() < 0.0 && next.point() > 0.0)
    {
        return Some(FixturePhaseError::DeliveredOvershoot);
    }
    if next.point().abs() > profile.phase_cap {
        return Some(FixturePhaseError::PhaseEnvelope);
    }
    if old.expired || now.active >= old.active_deadline || now.monotonic >= old.monotonic_deadline {
        return Some(FixturePhaseError::Expired);
    }
    None
}

impl<'producer, 'chart> FixturePhasePlanOwner<'producer, 'chart> {
    /// Move the actual producer lease and cursor into chart-independent custody.
    /// The prior readiness borrow and issued view do not escape this transfer.
    fn into_continuation(self) -> Continuation<'producer> {
        Continuation {
            frequency: self.frequency,
            producer: self.producer,
            cursor: self.cursor,
            profile: self.profile,
            baseline: self.baseline,
            last_now: self.last_now,
            authority: self.authority,
        }
    }

    /// Consume one issued operation through the existing total-rate receiver once.
    ///
    /// Stop/expiry does not prevent historical admission of a matching terminal.
    /// Only structural/identity/order rejection returns the original owner. Known
    /// numerical or policy failure remains inside the committed retirement.
    /// # Panics
    /// None: invalid optional state is an explicit outer refusal.
    pub fn settle(
        mut self,
        event: FixtureRateSettlement,
        observed_baseline: Option<FixturePhaseBaseline>,
        now: FixturePhaseNow,
    ) -> FixturePhaseSettlement<'producer, 'chart> {
        let Some(issued) = self.issued else {
            return FixturePhaseSettlement::OuterRejected {
                owner: self,
                error: FixturePhaseError::NoOutstandingOperation,
            };
        };
        let preflight = check_now(self.last_now, &self.cursor, now)
            .and_then(|()| check_terminal(issued, event, now));
        if let Err(error) = preflight {
            return FixturePhaseSettlement::OuterRejected { owner: self, error };
        }
        let predecessor = self.readiness.chart;
        let completion =
            match self
                .readiness
                .receive_rate_settlement(&mut self.cursor, predecessor, event)
            {
                Ok(completion) => completion,
                Err(source) => {
                    return FixturePhaseSettlement::OuterRejected {
                        owner: self,
                        error: FixturePhaseError::Delivery { source },
                    };
                }
            };
        self.last_now = now;
        self.issued = None;
        match completion.kind() {
            FixtureRateSettlementKind::ProvenNoEffect => {
                if let Some(old) = self.obligation.as_mut() {
                    old.residual = true;
                    old.expired |= now.active >= old.active_deadline
                        || now.monotonic >= old.monotonic_deadline;
                }
                FixturePhaseSettlement::ProvenNoEffect(self)
            }
            FixtureRateSettlementKind::Unknown => {
                self.frequency.unknown();
                let provenance = self.provenance;
                let historical_debt = self.historical_debt;
                FixturePhaseSettlement::Unknown(FixturePhaseBlocked {
                    continuation: self.into_continuation(),
                    event,
                    provenance,
                    historical_debt,
                })
            }
            FixtureRateSettlementKind::Completed | FixtureRateSettlementKind::Cancelled => {
                self.frequency
                    .rebase(predecessor, completion.effect(), completion.replacement());
                let debt = settled_debt(self.obligation, event, self.baseline, observed_baseline);
                let refusal = continuation_refusal(
                    self.profile,
                    self.baseline,
                    self.obligation,
                    event,
                    &debt,
                    now,
                );
                let replacement = match completion.into_replacement() {
                    Ok(Some(chart)) => Ok(chart),
                    Ok(None) => Err(FixtureChartError::Invariant),
                    Err(error) => Err(error),
                };
                let original = self.obligation;
                let provenance = self.provenance;
                if let Ok(value) = &debt {
                    self.historical_debt = Some(*value);
                }
                let frequency_history = self.frequency.history();
                FixturePhaseSettlement::Known(FixturePhaseRetirement {
                    frequency_history,
                    continuation: Some(self.into_continuation()),
                    event,
                    replacement,
                    debt,
                    refusal,
                    original,
                    provenance,
                })
            }
        }
    }
}

impl<'producer> FixturePhaseRetirement<'producer> {
    /// Copy the historical rebase result without supplying a usable projection.
    /// The snapshot survives a successful take-once transfer.
    #[must_use]
    pub const fn frequency_history(&self) -> Option<FixtureFrequencyHistory> {
        self.frequency_history
    }
    /// Borrow the actual replacement or preserve its original composition error.
    /// # Errors
    /// Return the original post-consumption chart failure.
    pub const fn replacement(&self) -> Result<&FixtureAffineChart, FixtureChartError> {
        match &self.replacement {
            Ok(chart) => Ok(chart),
            Err(error) => Err(*error),
        }
    }

    /// Borrow the original typed known terminal event, including its delivered span.
    #[must_use]
    pub const fn event(&self) -> &FixtureRateSettlement {
        &self.event
    }
    /// Borrow the real cursor only until the take-once transfer consumes it.
    #[must_use]
    pub fn cursor(&self) -> Option<&FixtureRateSettlementCursor> {
        self.continuation.as_ref().map(|v| &v.cursor)
    }
    /// Historical original source metadata survives a successful transfer.
    #[must_use]
    pub const fn provenance(&self) -> Option<&FixturePhaseProvenance> {
        self.provenance.as_ref()
    }
    /// Copy the historical accounted remainder, even if policy prohibits its use.
    #[must_use]
    pub fn remaining_phase_seconds(&self) -> Option<f64> {
        self.debt.as_ref().ok().map(PhaseDebt::point)
    }
    /// Copy the separately retained historical numerical bound.
    #[must_use]
    pub fn remaining_phase_error_seconds(&self) -> Option<f64> {
        self.debt.as_ref().ok().map(PhaseDebt::error)
    }
    /// Last known signed debt survives failed baseline or numerical accounting.
    /// A prior value is historical, not an assertion of the unavailable remainder.
    #[must_use]
    pub fn last_known_phase_seconds(&self) -> Option<f64> {
        self.debt
            .as_ref()
            .ok()
            .copied()
            .or_else(|| self.original.map(|v| v.debt))
            .map(|v| v.point())
    }
    /// Bound paired with the same historical debt selection, including prior error.
    #[must_use]
    pub fn last_known_phase_error_seconds(&self) -> Option<f64> {
        self.debt
            .as_ref()
            .ok()
            .copied()
            .or_else(|| self.original.map(|v| v.debt))
            .map(|v| v.error())
    }
    /// Borrow the original debt failure or subsequent residual-policy refusal.
    #[must_use]
    pub fn debt_error(&self) -> Option<&FixturePhaseError> {
        match &self.debt {
            Err(error) => Some(error),
            Ok(_) => self.refusal.as_ref(),
        }
    }
    /// Original active deadline is historical, never reset by resume.
    #[must_use]
    pub fn active_deadline(&self) -> Option<ActiveAwakeNs> {
        self.original.map(|v| v.active_deadline)
    }
    /// Original independent monotonic deadline.
    #[must_use]
    pub fn monotonic_deadline(&self) -> Option<MonotonicNs> {
        self.original.map(|v| v.monotonic_deadline)
    }
    /// Last current readings remain in the actual transfer slot until consumed.
    #[must_use]
    pub fn current_time(&self) -> Option<FixturePhaseNow> {
        self.continuation.as_ref().map(|v| v.last_now)
    }

    /// Admit current transfer eligibility before taking producer or cursor custody.
    /// Valid current readings and expiry facts are retained even when a later
    /// resume condition rejects, preventing a caller from rewinding those facts.
    /// # Errors
    /// Reject chart failure, consumed custody, retired authority, terminal cursor
    /// or invalid context/order. Invalid context or regressed clocks mutate nothing.
    fn admit_transfer(&mut self, now: FixturePhaseNow) -> Result<(), FixturePhaseError> {
        self.replacement()
            .map_err(|source| FixturePhaseError::Chart { source })?;
        let value = self
            .continuation
            .as_mut()
            .ok_or(FixturePhaseError::TransferConsumed)?;
        if let Some(reason) = value.authority {
            return Err(FixturePhaseError::Retired { reason });
        }
        if value.cursor.status() != FixtureRateCursorStatus::Pending {
            return Err(FixturePhaseError::CursorUnavailable);
        }
        check_now(value.last_now, &value.cursor, now)?;
        value.last_now = now;
        if let Some(old) = self.original.as_mut() {
            old.expired |=
                now.active >= old.active_deadline || now.monotonic >= old.monotonic_deadline;
        }
        Ok(())
    }

    /// Take the sole admitted custody slot and borrow its actual replacement chart.
    /// Only a completed transfer constructs the next local readiness owner.
    /// # Errors
    /// Reject a failed replacement or already consumed slot before moving custody.
    fn take_owner<'new>(
        &'new mut self,
        obligation: Option<Obligation>,
        baseline: FixturePhaseBaseline,
    ) -> Result<FixturePhasePlanOwner<'producer, 'new>, FixturePhaseError> {
        let chart = self
            .replacement
            .as_ref()
            .map_err(|source| FixturePhaseError::Chart { source: *source })?;
        let value = self
            .continuation
            .take()
            .ok_or(FixturePhaseError::TransferConsumed)?;
        Ok(FixturePhasePlanOwner {
            frequency: value.frequency,
            producer: value.producer,
            readiness: FixtureReferenceInputState::new(chart),
            cursor: value.cursor,
            profile: value.profile,
            baseline,
            last_now: value.last_now,
            authority: value.authority,
            obligation,
            provenance: self.provenance,
            historical_debt: self
                .debt
                .as_ref()
                .ok()
                .copied()
                .or_else(|| self.original.map(|v| v.debt)),
            issued: None,
        })
    }

    /// Transfer one permitted residual context without issuing an operation.
    /// # Errors
    /// Reject consumed/retired/unknown/exhausted/expired/failed debt or missing
    /// permission before taking the producer/cursor slot. Historical facts remain.
    pub fn resume<'new>(
        &'new mut self,
        now: FixturePhaseNow,
        permission: FixturePhaseResidualPermission,
    ) -> Result<FixturePhasePlanOwner<'producer, 'new>, FixturePhaseError> {
        self.admit_transfer(now)?;
        if permission != FixturePhaseResidualPermission::Allow {
            return Err(FixturePhaseError::ResidualPermission);
        }
        let debt = *self
            .debt
            .as_ref()
            .map_err(|_| FixturePhaseError::NoReference)?;
        if let Some(error) = &self.refusal {
            return Err(match error {
                FixturePhaseError::Expired => FixturePhaseError::Expired,
                FixturePhaseError::DeliveredOutOfEnvelope => {
                    FixturePhaseError::DeliveredOutOfEnvelope
                }
                FixturePhaseError::DeliveredOppositeDirection => {
                    FixturePhaseError::DeliveredOppositeDirection
                }
                FixturePhaseError::DeliveredOvershoot => FixturePhaseError::DeliveredOvershoot,
                FixturePhaseError::PhaseEnvelope => FixturePhaseError::PhaseEnvelope,
                _ => FixturePhaseError::ValidationInvariant,
            });
        }
        let value = self
            .continuation
            .as_ref()
            .ok_or(FixturePhaseError::TransferConsumed)?;
        let mut old = self.original.ok_or(FixturePhaseError::NoReference)?;
        if old.expired {
            return Err(FixturePhaseError::Expired);
        }
        old.debt = debt;
        old.residual = true;
        let baseline = value.baseline;
        self.take_owner(Some(old), baseline)
    }

    /// Transfer a context-only owner for genuinely new evidence and a known base.
    ///
    /// Old expiry/debt failure is not inherited as new-source authority; stop,
    /// conflict, generation retirement and cursor terminal state still block.
    /// # Errors
    /// Reject invalid current clocks/base or an unavailable take-once slot first.
    pub fn resume_fresh<'new>(
        &'new mut self,
        now: FixturePhaseNow,
        known_current_baseline: FixturePhaseBaseline,
    ) -> Result<FixturePhasePlanOwner<'producer, 'new>, FixturePhaseError> {
        self.admit_transfer(now)?;
        let value = self
            .continuation
            .as_ref()
            .ok_or(FixturePhaseError::TransferConsumed)?;
        if known_current_baseline.rate.abs() > value.profile.aggregate_cap {
            return Err(FixturePhaseError::DeliveredOutOfEnvelope);
        }
        self.take_owner(None, known_current_baseline)
    }
}

#[cfg(test)]
#[path = "../../../tests/private/phase_plan_lifecycle.rs"]
mod tests;

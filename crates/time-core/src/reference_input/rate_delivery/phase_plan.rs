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
/*
Original reference.c notice for the adaptive correction-area rule:
Copyright (C) Richard P. Curnow  1997-2003
Copyright (C) Miroslav Lichvar  2009-2018, 2020, 2022
Copyright (C) Andy Fiddaman  2024

This program is free software; you can redistribute it and/or modify
it under the terms of version 2 of the GNU General Public License as
published by the Free Software Foundation.

This program is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public License along
with this program; if not, write to the Free Software Foundation, Inc.,
51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.
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
//!
//! 10 November 2026: stable-base requests extend this same exclusive owner. The
//! reference/local/sys_generic requested-to-achieved feedback chain remains the
//! Chrony 4.9 provenance; the explicit affine conversion is displacement-rate
//! algebra in Core units, not a copy of upstream ppm or incremental dfreq units.
//! A request alone never changes the known baseline, chart, estimator or clock.
//!
//! 13 November 2026: the opt-in adaptive horizon follows reference.c's
//! get_correction_rate/REF_SetReference and sys_generic.c's update_slew at the
//! pinned Chrony 4.9 revision. Its explicit active-time adaptation selects a
//! computed correction time constant independently of original permission
//! deadlines. It supplies no native timer heuristics, defaults or clock mapping.

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
    FixtureAffineChart, FixtureChartBatch, FixtureChartCombinationError, FixtureChartDomain,
    FixtureChartError, FixtureChartFrames, FixtureCombinationState, FixtureContext,
    FixtureFrequencyNumericError, FixturePrincipalStratum, FixtureReferenceInputError,
    regression::{
        BaselineMapNumbers, BaselineOperands, BaselineRequest, FixturePhaseNumericError, PhaseDebt,
        PhaseRates, check_active_ratio_span,
    },
};

/// Checked adaptive-horizon parameters and numerical policy share one private owner.
mod adaptive;

use adaptive::{AdaptiveArea, adaptive_error, adaptive_profile_valid};
pub use adaptive::{FixtureAdaptiveHorizonError, FixtureAdaptiveHorizonProfile};

/// Matched finite step operations share this owner's operation and transfer custody.
mod step;

pub use step::{
    FixtureStepCompletion, FixtureStepDirections, FixtureStepError, FixtureStepFeedback,
    FixtureStepLimits, FixtureStepProposal, FixtureStepRetirement, FixtureStepSettlement,
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
    /// Optional immutable controller law; fixed mode retains the original denominator.
    adaptive: Option<FixtureAdaptiveHorizonProfile>,
}

/// Check the explicit positive finite profile without mutation.
fn profile_valid(value: &FixturePhaseProfile) -> bool {
    value.active_horizon.nanoseconds() > 0
        && value.active_horizon.nanoseconds() <= (1_u64 << 53)
        && value.monotonic_horizon.nanoseconds() > 0
        && [value.phase_cap, value.transient_cap, value.aggregate_cap]
            .iter()
            .all(|v| v.is_normal() && *v > 0.0)
        && value.adaptive.as_ref().is_none_or(adaptive_profile_valid)
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
                adaptive: None,
            },
        };
        let validated = raw.validate();
        if !profile_valid(&raw.value) {
            return Err(FixturePhaseError::InvalidProfile);
        }
        validated.map_err(|_| FixturePhaseError::ValidationInvariant)?;
        Ok(raw.value)
    }

    /// Opt into checked adaptive selection without changing either original lifetime.
    /// # Errors
    /// Preserve complete shared phase-profile admission; no owner or debt is changed.
    pub fn with_adaptive_horizon(
        self,
        adaptive: FixtureAdaptiveHorizonProfile,
    ) -> Result<Self, FixturePhaseError> {
        let raw = RawProfile {
            value: Self {
                adaptive: Some(adaptive),
                ..self
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
    /// The opt-in adaptive policy could not produce a supported checked decision.
    AdaptiveHorizon {
        /// Original bounded policy/arithmetic cause, with no raw source values.
        source: FixtureAdaptiveHorizonError,
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
            Self::AdaptiveHorizon { .. } => "fixture phase: adaptive horizon rejected",
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
            Self::AdaptiveHorizon { source } => Some(source),
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
    /// Accepted-reference control area; only adaptive mode carries this checked value.
    adaptive_area: Option<AdaptiveArea>,
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
    /// Exact selected rate denominator, independent of original permission remaining.
    correction_horizon: ActiveAwakeDurationNs,
    /// Requested active span, capped only after rate selection by original permission.
    requested_active_duration: ActiveAwakeDurationNs,
    /// Same-domain end of the requested span; neither expiry nor actual completion.
    requested_active_deadline: ActiveAwakeNs,
}

/// One shared issued slot; a base request cannot coexist with phase work.
#[derive(Clone, Copy)]
enum PendingOperation {
    /// Existing finite phase proposal and its real terminal identity.
    Phase(Issued),
    /// Model-derived stable-base request sharing the same operation slot.
    Baseline(BaselineIssued),
    /// Explicit finite step request sharing the same cursor and original obligation.
    Step(step::StepIssued),
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
    /// Exact active-time constant used by the requested-rate arithmetic.
    /// This may exceed the original remaining permission and proves no delivery.
    #[must_use]
    pub const fn correction_horizon(&self) -> ActiveAwakeDurationNs {
        self.issued.correction_horizon
    }
    /// Requested active duration, at most the original allowance remaining at issue.
    /// This is independent of actual acknowledged completion and monotonic expiry.
    #[must_use]
    pub const fn requested_active_duration(&self) -> ActiveAwakeDurationNs {
        self.issued.requested_active_duration
    }
    /// Issuance active reading plus the requested span; no clock conversion occurs.
    #[must_use]
    pub const fn requested_active_deadline(&self) -> ActiveAwakeNs {
        self.issued.requested_active_deadline
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
    /// Previous actually Accepted adaptive reference, never settlement/projection time.
    last_accepted_reference_active: Option<ActiveAwakeNs>,
    /// Monotone stop, conflict or generation retirement of useful proposal authority.
    authority: Option<FixturePhaseRetirementReason>,
    /// Optional useful debt and original deadlines; absent for a context-only owner.
    obligation: Option<Obligation>,
    /// Original genuine reference metadata retained without creating another emission.
    provenance: Option<FixturePhaseProvenance>,
    /// Last known numerical debt, retained as history when useful admission fails.
    historical_debt: Option<PhaseDebt>,
    /// At most one issued operation, retained until its matching terminal admission.
    issued: Option<PendingOperation>,
    /// Last actual accepted frequency emission reserved for a base request.
    baseline_round: Option<NonZeroU64>,
    /// Same-context exact affine clock relation, independent of model/base credit.
    clock_relation: Option<ControllerClockRelation>,
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
            last_accepted_reference_active: None,
            authority: None,
            obligation: None,
            provenance: None,
            historical_debt: None,
            issued: None,
            baseline_round: None,
            clock_relation: None,
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

    /// Prepare the actual reference's area without installing debt or advancing time.
    ///
    /// The receiving boundary has admitted this same owner's active reading and
    /// obtains deviation from its genuine token. Fixed mode performs no new work.
    /// Only successful obligation installation commits the accepted-reference stamp.
    /// # Errors
    /// Preserve interval/statistical/arithmetic refusal without changing owner or credit.
    fn prepare_reference_area(
        &self,
        deviation: f64,
        active: ActiveAwakeNs,
    ) -> Result<Option<AdaptiveArea>, FixturePhaseError> {
        let Some(profile) = self.profile.adaptive else {
            return Ok(None);
        };
        let previous = self.last_accepted_reference_active.unwrap_or(active);
        let elapsed = active
            .duration_since(previous)
            .map_err(|_| adaptive_error(FixtureAdaptiveHorizonError::ValidationInvariant))?;
        AdaptiveArea::from_reference(profile, deviation, elapsed).map(Some)
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
        let (original, deviation) = {
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
            let provenance = FixturePhaseProvenance {
                phase: estimate.phase_seconds(),
                error: estimate.phase_mean_roundoff_seconds(),
                round: token.emitted_round(),
                context: estimate.source_context(),
                frames: estimate.chart().frames(),
                anchor: estimate.anchor_observed_at(),
                latest: estimate.latest_observed_at(),
                fit_reference: estimate.fit_reference(),
                projected_at: estimate.projected_at(),
            };
            (provenance, estimate.phase_standard_deviation_seconds())
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
        let adaptive_area = match self.prepare_reference_area(deviation, now.active) {
            Ok(area) => area,
            Err(reason) => {
                return Ok(FixturePhaseReferenceAdmission::AdmittedNoPlan {
                    emitted_round: round,
                    reason,
                });
            }
        };
        self.obligation = Some(Obligation {
            debt,
            active_deadline,
            monotonic_deadline,
            residual: false,
            expired: false,
            adaptive_area,
        });
        if self.profile.adaptive.is_some() {
            self.last_accepted_reference_active = Some(now.active);
        }
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
        let correction_horizon = match (self.profile.adaptive, obligation.adaptive_area) {
            (Some(profile), Some(area)) => area.horizon(profile, obligation.debt)?,
            (None, None) => remaining,
            _ => return Err(FixturePhaseError::ValidationInvariant),
        };
        let requested_active_duration =
            if correction_horizon.nanoseconds() < remaining.nanoseconds() {
                correction_horizon
            } else {
                remaining
            };
        let requested_active_deadline = now
            .active
            .checked_add(requested_active_duration)
            .map_err(|_| FixturePhaseError::ClockOverflow)?;
        if requested_active_deadline > obligation.active_deadline {
            return Err(FixturePhaseError::ValidationInvariant);
        }
        let rates = obligation
            .debt
            .proposal(
                self.baseline.rate,
                correction_horizon,
                self.profile.transient_cap,
                self.profile.aggregate_cap,
            )
            .map_err(|source| FixturePhaseError::Numeric { source })?;
        self.issued = Some(PendingOperation::Phase(Issued {
            ordinal,
            started: now,
            rates,
            correction_horizon,
            requested_active_duration,
            requested_active_deadline,
        }));
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
            issued: match self.issued.as_ref()? {
                PendingOperation::Phase(issued) => issued,
                PendingOperation::Baseline(_) | PendingOperation::Step(_) => return None,
            },
            obligation: self.obligation.as_ref()?,
        })
    }
}

/// One consuming terminal path; only outer rejection returns an unchanged owner.
#[must_use]
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
    /// Reserved source-emission credit survives every chart/owner transfer.
    baseline_round: Option<NonZeroU64>,
    /// Same exact clock relation moves through all take-once transfer paths.
    clock_relation: Option<ControllerClockRelation>,
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
    /// Only accepted-reference time survives; terminal observations cannot replace it.
    last_accepted_reference_active: Option<ActiveAwakeNs>,
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
            baseline_round: self.baseline_round,
            clock_relation: self.clock_relation,
            producer: self.producer,
            cursor: self.cursor,
            profile: self.profile,
            baseline: self.baseline,
            last_now: self.last_now,
            last_accepted_reference_active: self.last_accepted_reference_active,
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
        let Some(PendingOperation::Phase(issued)) = self.issued else {
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
            baseline_round: value.baseline_round,
            clock_relation: value.clock_relation,
            producer: value.producer,
            readiness: FixtureReferenceInputState::new(chart),
            cursor: value.cursor,
            profile: value.profile,
            baseline,
            last_now: value.last_now,
            last_accepted_reference_active: value.last_accepted_reference_active,
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

/// Bounded stable-compensation refusal; no rejected clocks, rates or maps escape.
pub enum FixtureBaselineError {
    /// Existing phase/cursor/context guard rejected this operation.
    Phase {
        /// Exact existing phase/cursor cause without rejected payloads.
        source: FixturePhaseError,
    },
    /// Existing supported-model consumer refused this request.
    Frequency {
        /// Exact existing supported-model cause.
        source: FixtureFrequencyError,
    },
    /// Checked conversion or affine feedback failed.
    Numeric {
        /// Original checked conversion/transform cause.
        source: FixtureFrequencyNumericError,
    },
    /// The declared map does not match the actual basis, baseline or clock window.
    Map,
    /// This actual emission already reserved its one stable-base operation.
    ReusedEmission,
    /// The received event does not match this issued base operation.
    Receipt,
}
impl fmt::Debug for FixtureBaselineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FixtureBaselineError { .. }")
    }
}
impl fmt::Display for FixtureBaselineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Phase { .. } => "fixture baseline: phase owner refused",
            Self::Frequency { .. } => "fixture baseline: model refused",
            Self::Numeric { .. } => "fixture baseline: numeric refusal",
            Self::Map => "fixture baseline: clock map refused",
            Self::ReusedEmission => "fixture baseline: emission already used",
            Self::Receipt => "fixture baseline: unmatched receipt",
        })
    }
}
impl std::error::Error for FixtureBaselineError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Phase { source } => Some(source),
            Self::Frequency { source } => Some(source),
            Self::Numeric { source } => Some(source),
            _ => None,
        }
    }
}

/// Untrusted explicit coefficients for one declared no-sleep mathematical segment.
/// Constructing this input creates neither a map nor physical clock authority.
/// Raw input cannot substitute for the checked map at the proposal consumer:
///
/// ```compile_fail,E0308
/// use time_core::{FixtureControllerClockMapInput, FixturePhaseNow, FixturePhasePlanOwner};
/// fn bypass(mut owner: FixturePhasePlanOwner<'_, '_>, input: FixtureControllerClockMapInput,
///     now: FixturePhaseNow) {
///     let _ = owner.issue_baseline(input, now);
/// }
/// ```
#[derive(Clone, Copy)]
pub struct FixtureControllerClockMapInput {
    /// Independently tagged actual readings at the coefficient reference.
    pub reference: FixturePhaseNow,
    /// Inclusive monotonic end of this segment; no automatic extension exists.
    pub valid_until: MonotonicNs,
    /// Declared dA/dM; no unit ratio is assumed.
    pub active_ratio: f64,
    /// Absolute inherited uncertainty of the declared ratio.
    pub active_ratio_error: f64,
    /// Absolute source-to-actual displacement at the reference, not Cold-D.
    pub actual_phase: f64,
    /// Absolute inherited bound on the actual displacement.
    pub actual_phase_error: f64,
    /// Source-to-actual displacement gradient k in seconds per monotonic second.
    pub actual_rate: f64,
    /// Absolute inherited bound on k.
    pub actual_rate_error: f64,
}
impl fmt::Debug for FixtureControllerClockMapInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FixtureControllerClockMapInput { .. }")
    }
}

/// Full immutable basis equality; no tag-only numerical substitution is admitted.
#[derive(Clone, Copy)]
struct ControllerBasis {
    /// Full checked source/target equality labels.
    frames: FixtureChartFrames,
    /// Exact supported geometry including its coefficient epoch.
    domain: FixtureChartDomain,
    /// Phase/rate and their independent error bits, in fixed order.
    coefficients: [u64; 4],
}
impl ControllerBasis {
    /// Capture already checked immutable coefficients and their error bits.
    fn from_chart(chart: &FixtureAffineChart) -> Self {
        Self {
            frames: chart.frames(),
            domain: chart.domain(),
            coefficients: [
                chart.phase_at_reference_seconds().to_bits(),
                chart.rate_seconds_per_second().to_bits(),
                chart.phase_coefficient_roundoff_seconds().to_bits(),
                chart
                    .rate_coefficient_roundoff_seconds_per_second()
                    .to_bits(),
            ],
        }
    }
    /// Compare complete geometry without granting any native identity.
    fn matches(self, chart: &FixtureAffineChart) -> bool {
        let other = Self::from_chart(chart);
        self.frames == other.frames
            && self.coefficients == other.coefficients
            && self.domain.start() == other.domain.start()
            && self.domain.common_reference() == other.domain.common_reference()
            && self.domain.end() == other.domain.end()
    }
}

/// Exact positive rational clock-span witness; this never supplies authority.
/// The first admitted nonzero span fixes one affine ratio for later observations.
#[derive(Clone, Copy)]
struct ControllerRatioWitness {
    /// Exact active-awake duration in the first admitted nonzero span.
    active: ActiveAwakeDurationNs,
    /// Exact positive monotonic duration in that same span.
    monotonic: MonotonicDurationNs,
}
impl ControllerRatioWitness {
    /// Compare exact ratios with checked integers, without floating division.
    /// # Errors
    /// Refuse a contradictory span or unsupported product without any mutation.
    fn admits(
        self,
        active: ActiveAwakeDurationNs,
        monotonic: MonotonicDurationNs,
    ) -> Result<(), FixtureBaselineError> {
        let left = u128::from(self.active.nanoseconds())
            .checked_mul(u128::from(monotonic.nanoseconds()))
            .ok_or(FixtureBaselineError::Map)?;
        let right = u128::from(active.nanoseconds())
            .checked_mul(u128::from(self.monotonic.nanoseconds()))
            .ok_or(FixtureBaselineError::Map)?;
        if left != right {
            return Err(FixtureBaselineError::Map);
        }
        Ok(())
    }
}

/// Persistent clock-only relation custody; it contains no model or base authority.
#[derive(Clone, Copy)]
struct ControllerClockRelation {
    /// Original exact clock pair, retained when a map's coefficient epoch moves.
    reference: FixturePhaseNow,
    /// Compatible exact ratio learned only from admitted nonzero spans.
    witness: Option<ControllerRatioWitness>,
}
impl ControllerClockRelation {
    /// Check an observation against the same fixed origin and coefficient bounds.
    /// # Errors
    /// Refuse context/order, ratio-enclosure or exact affine inconsistency.
    fn observe(
        mut self,
        now: FixturePhaseNow,
        ratio: f64,
        error: f64,
    ) -> Result<Self, FixtureBaselineError> {
        if now.context != self.reference.context
            || now.active < self.reference.active
            || now.monotonic < self.reference.monotonic
        {
            return Err(FixtureBaselineError::Map);
        }
        let active = now
            .active
            .duration_since(self.reference.active)
            .map_err(|_| FixtureBaselineError::Map)?;
        let mono = now
            .monotonic
            .duration_since(self.reference.monotonic)
            .map_err(|_| FixtureBaselineError::Map)?;
        check_active_ratio_span(active, mono, ratio, error)
            .map_err(|source| FixtureBaselineError::Numeric { source })?;
        if let Some(witness) = self.witness {
            check_active_ratio_span(witness.active, witness.monotonic, ratio, error)
                .map_err(|source| FixtureBaselineError::Numeric { source })?;
            witness.admits(active, mono)?;
        } else if mono.nanoseconds() != 0 {
            self.witness = Some(ControllerRatioWitness {
                active,
                monotonic: mono,
            });
        }
        Ok(self)
    }
    /// Merge a changed chart/map declaration without losing original clock custody.
    /// # Errors
    /// Refuse a new intercept or incompatible ratio before any issuance credit.
    fn admit_map(
        self,
        mut map: FixtureControllerClockMap,
        now: FixturePhaseNow,
    ) -> Result<(Self, FixtureControllerClockMap), FixtureBaselineError> {
        let observed = self
            .observe(
                map.reference,
                map.numbers.ratio(),
                map.numbers.ratio_error(),
            )?
            .observe(now, map.numbers.ratio(), map.numbers.ratio_error())?;
        if let (Some(old), Some(new)) = (observed.witness, map.witness) {
            old.admits(new.active, new.monotonic)?;
        }
        map.witness = observed.witness;
        Ok((observed, map.observe(now)?))
    }
}

/// Checked, immutable declared map; it supplies no production physical issuer.
///
/// ```compile_fail,E0616
/// use time_core::FixtureControllerClockMap;
/// fn replace(map: &mut FixtureControllerClockMap) { map.baseline = 0; }
/// ```
/// ```compile_fail,E0599
/// use time_core::FixtureControllerClockMap;
/// let _ = FixtureControllerClockMap::default();
/// ```
#[derive(Clone, Copy)]
pub struct FixtureControllerClockMap {
    /// Actual complete chart basis to which this declaration belongs.
    basis: ControllerBasis,
    /// Exact clocks at the absolute Cold coefficient epoch.
    reference: FixturePhaseNow,
    /// Explicit segment end, never extended by a query or acknowledgement.
    valid_until: MonotonicNs,
    /// Known achieved baseline identity, including signed zero.
    baseline: u64,
    /// Privately checked affine coefficients and inherited errors.
    numbers: BaselineMapNumbers,
    /// Exact ratio custody survives rebinding the coefficient epoch to an ack.
    witness: Option<ControllerRatioWitness>,
}
impl fmt::Debug for FixtureControllerClockMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FixtureControllerClockMap { .. }")
    }
}

/// Mandatory whole-group map/domain admission; numeric fields already have their owner.
#[derive(Validate)]
struct RawControllerMap<'a> {
    #[garde(custom(|value: &(&FixtureAffineChart, &FixtureControllerClockMap), (): &()| {
        controller_map_rule(value.0, value.1).map_err(|_| garde::Error::new("invalid controller clock map"))
    }))]
    /// Actual immutable chart/map group; optional caller validation cannot bypass it.
    value: (&'a FixtureAffineChart, &'a FixtureControllerClockMap),
}
/// Validate the whole immutable map at its real owner boundary.
/// # Errors
/// Refuse unsupported domain or a substituted chart without any effect.
fn controller_map_rule(
    chart: &FixtureAffineChart,
    map: &FixtureControllerClockMap,
) -> Result<(), FixtureBaselineError> {
    if !map.basis.matches(chart)
        || map.reference.monotonic < chart.domain().start()
        || map.valid_until > chart.domain().end()
        || map.valid_until <= map.reference.monotonic
    {
        return Err(FixtureBaselineError::Map);
    }
    Ok(())
}
impl FixtureControllerClockMap {
    /// Admit explicit coefficients; current issue/receipt guards still recheck context.
    /// # Errors
    /// Refuse numeric support, domain or mandatory-validation disagreement.
    pub fn new(
        chart: &FixtureAffineChart,
        baseline: FixturePhaseBaseline,
        input: FixtureControllerClockMapInput,
    ) -> Result<Self, FixtureBaselineError> {
        let numbers = BaselineMapNumbers::new(
            input.active_ratio,
            input.active_ratio_error,
            input.actual_phase,
            input.actual_phase_error,
            input.actual_rate,
            input.actual_rate_error,
        )
        .map_err(|source| FixtureBaselineError::Numeric { source })?;
        let map = Self {
            basis: ControllerBasis::from_chart(chart),
            reference: input.reference,
            valid_until: input.valid_until,
            baseline: baseline.rate.to_bits(),
            numbers,
            witness: None,
        };
        let raw = RawControllerMap {
            value: (chart, &map),
        };
        let validated = raw.validate();
        controller_map_rule(chart, &map)?;
        validated.map_err(|_| FixtureBaselineError::Phase {
            source: FixturePhaseError::ValidationInvariant,
        })?;
        Ok(map)
    }
    /// Declared ratio; this is a mathematical assertion, not native calibration.
    #[must_use]
    pub const fn active_ratio(&self) -> f64 {
        self.numbers.ratio()
    }
    /// Absolute source-to-actual displacement at the retained reference epoch.
    #[must_use]
    pub const fn actual_phase(&self) -> f64 {
        self.numbers.phase()
    }
    /// Absolute source-to-actual gradient at the retained reference epoch.
    #[must_use]
    pub const fn actual_rate(&self) -> f64 {
        self.numbers.rate()
    }
    /// Epoch of the actual displacement coefficients in the admitted segment.
    #[must_use]
    pub const fn reference(&self) -> FixturePhaseNow {
        self.reference
    }
    /// Recheck both supplied readings against the declared affine segment.
    /// # Errors
    /// Refuse context, expiry, regression or non-unit clock inconsistency.
    fn at(self, now: FixturePhaseNow) -> Result<(), FixtureBaselineError> {
        self.observe(now).map(|_| ())
    }
    /// Admit one exact observation and retain one compatible fixed affine ratio.
    /// # Errors
    /// Refuse current context/domain/order, coefficient enclosure or joint ratio.
    fn observe(mut self, now: FixturePhaseNow) -> Result<Self, FixtureBaselineError> {
        if now.context != self.reference.context
            || now.monotonic < self.reference.monotonic
            || now.monotonic > self.valid_until
            || now.active < self.reference.active
        {
            return Err(FixtureBaselineError::Map);
        }
        let active = now
            .active
            .duration_since(self.reference.active)
            .map_err(|_| FixtureBaselineError::Map)?;
        let monotonic = now
            .monotonic
            .duration_since(self.reference.monotonic)
            .map_err(|_| FixtureBaselineError::Map)?;
        check_active_ratio_span(
            active,
            monotonic,
            self.numbers.ratio(),
            self.numbers.ratio_error(),
        )
        .map_err(|source| FixtureBaselineError::Numeric { source })?;
        if monotonic.nanoseconds() == 0 {
            if active.nanoseconds() != 0 {
                return Err(FixtureBaselineError::Map);
            }
        } else if let Some(witness) = self.witness {
            witness.admits(active, monotonic)?;
        } else {
            self.witness = Some(ControllerRatioWitness { active, monotonic });
        }
        Ok(self)
    }
}

/// Actual one-slot baseline command retained by the existing exclusive owner.
#[derive(Clone, Copy)]
struct BaselineIssued {
    /// Current shared persistent cursor identity reserved by this operation.
    ordinal: NonZeroU64,
    /// Exact clocks at request admission, bounding the known old-rate span.
    started: FixturePhaseNow,
    /// Original active request cutoff; no acknowledgement renews it.
    active_deadline: ActiveAwakeNs,
    /// Original monotonic cutoff bounded by the declared map segment.
    monotonic_deadline: MonotonicNs,
    /// Genuine accepted frequency emission consumed by this request.
    round: NonZeroU64,
    /// Checked desired/clipped request and its arithmetic enclosure.
    numbers: BaselineRequest,
    /// Immutable full-basis map admitted at issue time.
    map: FixtureControllerClockMap,
    /// Known old achieved rate; requested rate never substitutes for it.
    old_baseline: FixturePhaseBaseline,
}
/// Borrowed requested compensation; no method converts it into an achieved baseline.
///
/// ```compile_fail,E0505
/// use time_core::{FixtureBaselineError, FixtureBaselineFeedback, FixtureControllerClockMap,
///     FixturePhaseNow, FixturePhasePlanOwner};
/// fn stale(mut owner: FixturePhasePlanOwner<'_, '_>, map: FixtureControllerClockMap,
///     now: FixturePhaseNow, event: FixtureBaselineFeedback) -> Result<(), FixtureBaselineError> {
///     let proposal = owner.issue_baseline(map, now)?;
///     let settled = owner.settle_baseline(event, now);
///     std::hint::black_box(proposal.ordinal());
///     std::hint::black_box(settled);
///     Ok(())
/// }
/// ```
pub struct FixtureBaselineProposal<'a> {
    /// Borrow of the actual one pending operation, never an owned issuer token.
    issued: &'a BaselineIssued,
}
impl FixtureBaselineProposal<'_> {
    /// Sole shared-cursor operation ordinal.
    #[must_use]
    pub const fn ordinal(&self) -> NonZeroU64 {
        self.issued.ordinal
    }
    /// Actual accepted frequency emission reserved for this request.
    #[must_use]
    pub const fn emitted_round(&self) -> NonZeroU64 {
        self.issued.round
    }
    /// Requested additive stable displacement per active second.
    #[must_use]
    pub fn requested_rate(&self) -> f64 {
        self.issued.numbers.requested()
    }
    /// Unclipped desire, separate from policy clipping and achievement.
    #[must_use]
    pub fn desired_rate(&self) -> f64 {
        self.issued.numbers.desired()
    }
    /// Arithmetic/map error on the unclipped desire, not statistical uncertainty.
    #[must_use]
    pub fn desired_error(&self) -> f64 {
        self.issued.numbers.desired_error()
    }
    /// Whether the explicit aggregate cap clipped the numerical desire.
    #[must_use]
    pub fn clipped(&self) -> bool {
        self.issued.numbers.clipped()
    }
    /// Original independent monotonic request deadline.
    #[must_use]
    pub const fn monotonic_deadline(&self) -> MonotonicNs {
        self.issued.monotonic_deadline
    }
    /// Original active-awake request deadline.
    #[must_use]
    pub const fn active_deadline(&self) -> ActiveAwakeNs {
        self.issued.active_deadline
    }
}
impl fmt::Debug for FixtureBaselineProposal<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FixtureBaselineProposal { .. }")
    }
}

/// Checked Known receipt, distinct from a requested rate and native authority.
///
/// ```compile_fail,E0451
/// use time_core::FixtureBaselineCompletion;
/// fn bypass(base: FixtureBaselineCompletion) {
///     let _ = FixtureBaselineCompletion { achieved: todo!(), ..base };
/// }
/// ```
#[derive(Clone, Copy)]
pub struct FixtureBaselineCompletion {
    /// Known old total rate and exact effective boundary on both clocks.
    delivery: super::FixtureSettledRateInterval,
    /// Separately known achieved new base at the interval endpoint.
    achieved: FixturePhaseBaseline,
    /// Rate-effect chart step after the constant-delivery intermediate target.
    final_frames: FixtureChartFrames,
}
impl FixtureBaselineCompletion {
    /// Retain an explicitly known old-rate interval and achieved rate at its endpoint.
    /// Current identity, exact start, actual old rate and clocks belong to the receiver.
    /// # Errors
    /// Reject a broken two-step frame chain without constructing a receipt.
    pub fn new(
        delivery: super::FixtureSettledRateInterval,
        achieved: FixturePhaseBaseline,
        final_frames: FixtureChartFrames,
    ) -> Result<Self, FixtureBaselineError> {
        let raw = RawBaselineCompletion {
            value: Self {
                delivery,
                achieved,
                final_frames,
            },
        };
        let validated = raw.validate();
        baseline_completion_rule(&raw.value)?;
        validated.map_err(|_| FixtureBaselineError::Phase {
            source: FixturePhaseError::ValidationInvariant,
        })?;
        Ok(raw.value)
    }
}
/// Check the complete immutable two-step descriptor before publishing a receipt.
/// # Errors
/// Preserve mapping mismatch and the checked full endpoint-cycle refusal.
fn baseline_completion_rule(value: &FixtureBaselineCompletion) -> Result<(), FixtureBaselineError> {
    let delivery = value.delivery;
    let final_frames = value.final_frames;
    if final_frames.source_frame() != delivery.frames.target_frame()
        || final_frames.source_generation() != delivery.frames.target_generation()
    {
        return Err(FixtureBaselineError::Receipt);
    }
    FixtureChartFrames::new(
        delivery.frames.source_frame(),
        delivery.frames.source_generation(),
        final_frames.target_frame(),
        final_frames.target_generation(),
    )
    .map_err(|source| FixtureBaselineError::Phase {
        source: FixturePhaseError::Chart { source },
    })?;
    Ok(())
}
#[derive(Validate)]
/// Mandatory whole immutable receipt-construction group.
struct RawBaselineCompletion {
    #[garde(custom(|value: &FixtureBaselineCompletion, (): &()| {
        baseline_completion_rule(value).map_err(|_| garde::Error::new("invalid baseline receipt descriptor"))
    }))]
    /// Existing immutable typed operands plus their changed frame relationship.
    value: FixtureBaselineCompletion,
}
impl fmt::Debug for FixtureBaselineCompletion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FixtureBaselineCompletion { .. }")
    }
}
/// Actual modeled terminal observation; equal rates never imply `ProvenNoEffect`.
#[derive(Clone, Copy)]
pub enum FixtureBaselineFeedback {
    /// Known old-rate delivery and explicitly achieved new stable base.
    Known(FixtureBaselineCompletion),
    /// Explicit absence of a base change; no endpoint or delivered rate is invented.
    ProvenNoEffect {
        /// Actual clock association of the matching operation.
        context: FixtureRateClockContext,
        /// Exact pending shared cursor identity.
        ordinal: NonZeroU64,
    },
    /// Uncertain effect or effective boundary, irreversibly blocking this cursor.
    Unknown {
        /// Actual uncertain operation's clock association.
        context: FixtureRateClockContext,
        /// Shared ordinal consumed into unresolved custody.
        ordinal: NonZeroU64,
    },
}
impl FixtureBaselineFeedback {
    /// Retain the exact modeled terminal identity; no endpoint is invented.
    fn terminal(self) -> FixtureRateSettlement {
        match self {
            Self::Known(receipt) => FixtureRateSettlement::Completed(receipt.delivery),
            Self::ProvenNoEffect { context, ordinal } => {
                FixtureRateSettlement::ProvenNoEffect { context, ordinal }
            }
            Self::Unknown { context, ordinal } => {
                FixtureRateSettlement::Unknown { context, ordinal }
            }
        }
    }
}
impl fmt::Debug for FixtureBaselineFeedback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FixtureBaselineFeedback { .. }")
    }
}

/// Consume-once result of the same owner's baseline operation.
#[must_use]
#[expect(
    clippy::large_enum_variant,
    reason = "The bounded inline result returns the original owner on rejection and retains known producer/cursor custody and model history without a success-path heap allocation; no variant is input-sized."
)]
pub enum FixtureBaselineSettlement<'producer, 'chart> {
    /// Admission failed without consuming the original issued owner.
    OuterRejected {
        /// Original actual owner, still retaining its pending request and credit.
        owner: FixturePhasePlanOwner<'producer, 'chart>,
        /// Exact bounded structural/current-context refusal.
        error: FixtureBaselineError,
    },
    /// The request/ordinal was consumed without an invented correction.
    ProvenNoEffect(FixturePhasePlanOwner<'producer, 'chart>),
    /// Known effect custody, including post-consumption numerical failure.
    Known(FixtureBaselineRetirement<'producer>),
    /// Uncertain effect consumes/blocks the shared cursor.
    Unknown(FixturePhaseBlocked<'producer>),
}
impl fmt::Debug for FixtureBaselineSettlement<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FixtureBaselineSettlement { .. }")
    }
}
/// Known achieved-base history plus the same take-once producer/cursor continuation.
pub struct FixtureBaselineRetirement<'producer> {
    /// Existing take-once producer/cursor custody and committed chart/debt history.
    inner: FixturePhaseRetirement<'producer>,
    /// Known actual new base, retained independently of all later failures.
    achieved: FixturePhaseBaseline,
    /// Absolute Cold rebound to the actual final basis, or its original refusal.
    map: Result<FixtureControllerClockMap, FixtureBaselineError>,
    /// Bit-level baseline change, independently of a rounded zero gradient.
    changed: bool,
    /// Original post-consumption transform refusal, retained independently of custody.
    failure: Option<FixtureBaselineError>,
    /// Base-operation deadline/cap refusal, distinct from retired old phase debt.
    operation_refusal: Option<FixturePhaseError>,
}
impl fmt::Debug for FixtureBaselineRetirement<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FixtureBaselineRetirement { .. }")
    }
}
impl<'producer> FixtureBaselineRetirement<'producer> {
    /// Borrow the actual replacement or retained post-consumption chart failure.
    /// # Errors
    /// Return the exact retained descriptor/composition failure without a refund.
    pub fn replacement(&self) -> Result<&FixtureAffineChart, &FixtureBaselineError> {
        self.inner
            .replacement
            .as_ref()
            .map_err(|_| self.failure.as_ref().unwrap_or(&INTERNAL_BASELINE_REFUSAL))
    }
    /// Exact retained transform failure after the matching event was consumed.
    #[must_use]
    pub fn feedback_error(&self) -> Option<&FixtureBaselineError> {
        self.failure.as_ref()
    }
    /// Original finite obligation deadlines remain historical across the base change.
    #[must_use]
    pub fn original_phase_deadlines(&self) -> (Option<ActiveAwakeNs>, Option<MonotonicNs>) {
        (
            self.inner.active_deadline(),
            self.inner.monotonic_deadline(),
        )
    }
    /// Actual cap/deadline refusal after the achieved fact was retained.
    #[must_use]
    pub fn continuation_error(&self) -> Option<&FixturePhaseError> {
        self.operation_refusal.as_ref()
    }
    /// Borrow the exact committed old finite-delivery event independently of feedback.
    #[must_use]
    pub const fn event(&self) -> &FixtureRateSettlement {
        self.inner.event()
    }
    /// The shared consumed cursor remains observable until custody is transferred.
    #[must_use]
    pub fn cursor(&self) -> Option<&FixtureRateSettlementCursor> {
        self.inner.cursor()
    }
    /// Known achieved fact survives numeric/profile failure and continuation consumption.
    #[must_use]
    pub const fn achieved_baseline(&self) -> FixturePhaseBaseline {
        self.achieved
    }
    /// Borrow model history without granting current projection or issuance.
    #[must_use]
    pub const fn frequency_history(&self) -> Option<FixtureFrequencyHistory> {
        self.inner.frequency_history()
    }
    /// Return the rebound current map or original post-consumption cause.
    /// # Errors
    /// Borrow the exact original map-transform refusal; no replacement is invented.
    pub fn clock_map(&self) -> Result<&FixtureControllerClockMap, &FixtureBaselineError> {
        self.map.as_ref()
    }
    /// Resume at most once; changed baseline admits context only, not fresh debt.
    /// # Errors
    /// Preserve unavailable map, profile/authority/cursor and consumed-transfer causes.
    pub fn resume<'a>(
        &'a mut self,
        now: FixturePhaseNow,
        permission: FixturePhaseResidualPermission,
    ) -> Result<FixturePhasePlanOwner<'producer, 'a>, FixtureBaselineError> {
        self.transfer_ready()?;
        let owner = if self.changed || self.inner.original.is_none() {
            self.inner.resume_fresh(now, self.achieved)
        } else {
            self.inner.resume(now, permission)
        };
        owner.map_err(|source| FixtureBaselineError::Phase { source })
    }
    /// Transfer context only, retaining history without creating fresh phase debt.
    /// The known baseline is the private actual achieved fact, never a new input.
    /// # Errors
    /// Preserve consumed/retired/cursor/profile failures after admitting current clocks.
    pub fn resume_fresh<'a>(
        &'a mut self,
        now: FixturePhaseNow,
    ) -> Result<FixturePhasePlanOwner<'producer, 'a>, FixtureBaselineError> {
        self.transfer_ready()?;
        self.inner
            .resume_fresh(now, self.achieved)
            .map_err(|source| FixtureBaselineError::Phase { source })
    }
    /// Require all post-consumption numeric/map/base-operation gates before transfer.
    /// # Errors
    /// Refuse useful continuation while retaining exact historical failure getters.
    fn transfer_ready(&self) -> Result<(), FixtureBaselineError> {
        if self.failure.is_some() || self.map.is_err() || self.operation_refusal.is_some() {
            return Err(FixtureBaselineError::Receipt);
        }
        Ok(())
    }
}

impl FixturePhasePlanOwner<'_, '_> {
    /// Reserve one model-derived requested base, never an acknowledged baseline.
    /// The actual supported model, complete map/baseline and shared cursor are checked here.
    /// # Errors
    /// Refuse pending, retired, stale/reused emission, map/time or arithmetic failures.
    pub fn issue_baseline(
        &mut self,
        map: FixtureControllerClockMap,
        now: FixturePhaseNow,
    ) -> Result<FixtureBaselineProposal<'_>, FixtureBaselineError> {
        self.authority_ready()
            .map_err(|source| FixtureBaselineError::Phase { source })?;
        if self.issued.is_some() {
            return Err(FixtureBaselineError::Phase {
                source: FixturePhaseError::OutstandingOperation,
            });
        }
        cursor_mapping(&self.cursor, self.chart())
            .map_err(|source| FixtureBaselineError::Phase { source })?;
        check_now(self.last_now, &self.cursor, now)
            .map_err(|source| FixtureBaselineError::Phase { source })?;
        if !map.basis.matches(self.chart())
            || map.baseline != self.baseline.rate.to_bits()
            || now.monotonic >= map.valid_until
        {
            return Err(FixtureBaselineError::Map);
        }
        let relation = self.clock_relation.unwrap_or(ControllerClockRelation {
            reference: map.reference,
            witness: map.witness,
        });
        // A newly declared segment cannot contradict already admitted current
        // readings that lie inside it, even before the first base request.
        let relation = if self.last_now.monotonic >= map.reference.monotonic
            && self.last_now.monotonic <= map.valid_until
        {
            relation.observe(
                self.last_now,
                map.numbers.ratio(),
                map.numbers.ratio_error(),
            )?
        } else {
            relation
        };
        let (relation, map) = relation.admit_map(map, now)?;
        self.clock_relation = Some(relation);
        self.last_now = now;
        let operands = BaselineOperands {
            chart_rate: self.chart().rate_seconds_per_second(),
            chart_rate_error: self.chart().rate_coefficient_roundoff_seconds_per_second(),
            actual_rate: map.numbers.rate(),
            actual_rate_error: map.numbers.rate_error(),
            active_ratio: map.numbers.ratio(),
            active_ratio_error: map.numbers.ratio_error(),
            baseline: self.baseline.rate,
            cap: self.profile.aggregate_cap,
        };
        let (round, numbers) = self
            .frequency
            .baseline_request(
                self.readiness.chart,
                self.cursor.context,
                now.monotonic,
                operands,
            )
            .map_err(|source| FixtureBaselineError::Frequency { source })?;
        if self.baseline_round.is_some_and(|old| round <= old) {
            return Err(FixtureBaselineError::ReusedEmission);
        }
        let (active_deadline, mono) = deadlines(self.profile, now)
            .map_err(|source| FixtureBaselineError::Phase { source })?;
        let ordinal = self
            .cursor
            .next_ordinal()
            .ok_or(FixtureBaselineError::Phase {
                source: FixturePhaseError::CursorUnavailable,
            })?;
        self.baseline_round = Some(round);
        self.issued = Some(PendingOperation::Baseline(BaselineIssued {
            ordinal,
            started: now,
            active_deadline,
            monotonic_deadline: mono.min(map.valid_until),
            round,
            numbers,
            map,
            old_baseline: self.baseline,
        }));
        self.baseline_proposal()
            .ok_or(FixtureBaselineError::Receipt)
    }
    /// Borrow the existing base proposal without another credit or clock read.
    #[must_use]
    pub fn baseline_proposal(&self) -> Option<FixtureBaselineProposal<'_>> {
        if self.authority.is_some() {
            return None;
        }
        match self.issued.as_ref()? {
            PendingOperation::Baseline(issued) => Some(FixtureBaselineProposal { issued }),
            PendingOperation::Phase(_) | PendingOperation::Step(_) => None,
        }
    }
}

/// This sentinel is unreachable after a successful affine feedback construction;
/// exact failure diagnostics are retained in `failure` before any useful resume.
const INTERNAL_BASELINE_REFUSAL: FixtureBaselineError = FixtureBaselineError::Receipt;

/// Current pending request and exact immutable terminal relationship.
struct BaselineTerminal {
    /// Actual pending operation retained by the exclusive owner.
    issued: BaselineIssued,
    /// Supplied immutable event whose current relationship is checked here.
    event: FixtureBaselineFeedback,
    /// Supplied current clocks containing every admitted known endpoint.
    now: FixturePhaseNow,
}
/// Check all caller-declared receipt relationships before consuming any ordinal.
/// # Errors
/// Refuse context/identity, wrong old baseline, endpoint or affine segment mismatch.
fn baseline_terminal(value: &BaselineTerminal) -> Result<(), FixtureBaselineError> {
    let (context, ordinal) = match value.event {
        FixtureBaselineFeedback::Known(receipt) => {
            (receipt.delivery.context, receipt.delivery.ordinal)
        }
        FixtureBaselineFeedback::ProvenNoEffect { context, ordinal }
        | FixtureBaselineFeedback::Unknown { context, ordinal } => (context, ordinal),
    };
    if context != value.issued.started.context || ordinal != value.issued.ordinal {
        return Err(FixtureBaselineError::Receipt);
    }
    if let FixtureBaselineFeedback::Known(receipt) = value.event {
        let span = receipt.delivery.span;
        if receipt.delivery.rate.to_bits() != value.issued.old_baseline.rate.to_bits()
            || span.active.0 != value.issued.started.active
            || span.effective.0 != value.issued.started.monotonic
            || span.active.1 > value.now.active
            || span.effective.1 > value.now.monotonic
        {
            return Err(FixtureBaselineError::Receipt);
        }
        if span.effective.1 <= value.issued.map.valid_until {
            value.issued.map.at(FixturePhaseNow::new(
                context,
                span.active.1,
                span.effective.1,
            ))?;
            // Inside the admitted domain, the actual operation must also fit
            // the same bounds; expired maps authorize no extrapolated guard.
            let mono = span
                .effective
                .1
                .duration_since(span.effective.0)
                .map_err(|_| FixtureBaselineError::Map)?;
            check_active_ratio_span(
                span.active_elapsed,
                mono,
                value.issued.map.numbers.ratio(),
                value.issued.map.numbers.ratio_error(),
            )
            .map_err(|source| FixtureBaselineError::Numeric { source })?;
        }
    }
    Ok(())
}
#[derive(Validate)]
/// Mandatory current pending-operation/terminal relationship adapter.
struct RawBaselineTerminal {
    #[garde(custom(|value: &BaselineTerminal, (): &()| {
        baseline_terminal(value).map_err(|_| garde::Error::new("invalid matched baseline receipt"))
    }))]
    /// Complete actual relationship, validated before cursor consumption.
    value: BaselineTerminal,
}
/// Mandatory whole-group validation of the same real pending-operation rule.
/// # Errors
/// Preserve the original relationship cause or report validation disagreement.
fn check_baseline_terminal(value: BaselineTerminal) -> Result<(), FixtureBaselineError> {
    let raw = RawBaselineTerminal { value };
    let validated = raw.validate();
    baseline_terminal(&raw.value)?;
    validated.map_err(|_| FixtureBaselineError::Phase {
        source: FixturePhaseError::ValidationInvariant,
    })
}

/// Compose the already committed finite delivery with the continuous rate effect.
/// # Errors
/// Retain chart or numeric failure after commitment; no cursor is refunded.
fn baseline_feedback_charts(
    predecessor: &FixtureAffineChart,
    completion: &super::FixtureRateSettlementCompletion,
    issued: &BaselineIssued,
    receipt: FixtureBaselineCompletion,
) -> Result<(FixtureAffineChart, FixtureAffineChart), FixtureBaselineError> {
    let constant = completion
        .effect()
        .map_err(|source| FixtureBaselineError::Phase {
            source: FixturePhaseError::Chart { source },
        })?
        .ok_or(FixtureBaselineError::Receipt)?;
    // The constant receiver has already committed and constructed its actual
    // intermediate replacement; preserve its original failure before extending.
    completion
        .replacement()
        .map_err(|source| FixtureBaselineError::Phase {
            source: FixturePhaseError::Chart { source },
        })?
        .ok_or(FixtureBaselineError::Receipt)?;
    if receipt.delivery.span.effective.1 > issued.map.valid_until {
        return Err(FixtureBaselineError::Map);
    }
    let affine = FixtureAffineChart::baseline_rate_effect(
        receipt.final_frames,
        predecessor.domain(),
        issued.map.numbers.ratio(),
        issued.map.numbers.ratio_error(),
        issued.old_baseline.rate,
        receipt.achieved.rate,
        receipt.delivery.span.effective.1,
    )
    .map_err(|source| FixtureBaselineError::Numeric { source })?;
    // Fixed RN association is predecessor.compose(constant.compose(affine));
    // sequential predecessor composition can have different point bits.
    let effect = constant
        .compose(&affine)
        .map_err(|source| FixtureBaselineError::Phase {
            source: FixturePhaseError::Chart { source },
        })?;
    let replacement =
        predecessor
            .compose(&effect)
            .map_err(|source| FixtureBaselineError::Phase {
                source: FixturePhaseError::Chart { source },
            })?;
    Ok((effect, replacement))
}

/// Rebind absolute Cold to the post-settlement basis without charging it twice.
/// # Errors
/// Preserve elapsed-time/numeric refusal after cursor consumption; no map extension.
fn rebound_baseline_map(
    issued: &BaselineIssued,
    receipt: FixtureBaselineCompletion,
    replacement: &FixtureAffineChart,
) -> Result<FixtureControllerClockMap, FixtureBaselineError> {
    let observed = issued.map.observe(FixturePhaseNow::new(
        receipt.delivery.context,
        receipt.delivery.span.active.1,
        receipt.delivery.span.effective.1,
    ))?;
    // A boundary acknowledgement is inside the effect's inclusive domain but
    // leaves no positive future segment for a newly rebound checked map.
    if receipt.delivery.span.effective.1 >= issued.map.valid_until {
        return Err(FixtureBaselineError::Map);
    }
    let elapsed = receipt
        .delivery
        .span
        .effective
        .1
        .duration_since(issued.map.reference.monotonic)
        .map_err(|_| FixtureBaselineError::Map)?;
    let numbers = issued
        .map
        .numbers
        .advance(issued.old_baseline.rate, receipt.achieved.rate, elapsed)
        .map_err(|source| FixtureBaselineError::Numeric { source })?;
    Ok(FixtureControllerClockMap {
        basis: ControllerBasis::from_chart(replacement),
        reference: FixturePhaseNow::new(
            receipt.delivery.context,
            receipt.delivery.span.active.1,
            receipt.delivery.span.effective.1,
        ),
        valid_until: issued.map.valid_until,
        baseline: receipt.achieved.rate.to_bits(),
        numbers,
        witness: observed.witness,
    })
}

impl<'producer, 'chart> FixturePhasePlanOwner<'producer, 'chart> {
    /// Consume one matched stable-base receipt through the existing shared cursor.
    /// Known achieved facts survive arithmetic, cap, stop and deadline refusal.
    /// # Panics
    /// None: absent pending state and malformed receipts are explicit outer refusals.
    pub fn settle_baseline(
        mut self,
        event: FixtureBaselineFeedback,
        now: FixturePhaseNow,
    ) -> FixtureBaselineSettlement<'producer, 'chart> {
        let Some(PendingOperation::Baseline(issued)) = self.issued else {
            return FixtureBaselineSettlement::OuterRejected {
                owner: self,
                error: FixtureBaselineError::Phase {
                    source: FixturePhaseError::NoOutstandingOperation,
                },
            };
        };
        let preflight = check_now(self.last_now, &self.cursor, now)
            .map_err(|source| FixtureBaselineError::Phase { source })
            .and_then(|()| check_baseline_terminal(BaselineTerminal { issued, event, now }));
        if let Err(error) = preflight {
            return FixtureBaselineSettlement::OuterRejected { owner: self, error };
        }
        let relation = match self.baseline_terminal_relation(&issued, event, now) {
            Ok(value) => value,
            Err(error) => return FixtureBaselineSettlement::OuterRejected { owner: self, error },
        };
        let predecessor = self.readiness.chart;
        let terminal = event.terminal();
        let consumed = match event {
            FixtureBaselineFeedback::Known(receipt) => self.readiness.receive_baseline_settlement(
                &mut self.cursor,
                predecessor,
                receipt.delivery,
                receipt.final_frames,
            ),
            _ => self
                .readiness
                .receive_rate_settlement(&mut self.cursor, predecessor, terminal),
        };
        let completion = match consumed {
            Ok(value) => value,
            Err(source) => {
                return FixtureBaselineSettlement::OuterRejected {
                    owner: self,
                    error: FixtureBaselineError::Phase {
                        source: FixturePhaseError::Delivery { source },
                    },
                };
            }
        };
        self.last_now = now;
        self.clock_relation = relation;
        self.issued = None;
        match event {
            FixtureBaselineFeedback::ProvenNoEffect { .. } => {
                if let Some(old) = self.obligation.as_mut() {
                    old.residual = true;
                    old.expired |= now.active >= old.active_deadline
                        || now.monotonic >= old.monotonic_deadline;
                }
                FixtureBaselineSettlement::ProvenNoEffect(self)
            }
            FixtureBaselineFeedback::Unknown { .. } => {
                self.frequency.unknown();
                let provenance = self.provenance;
                let historical_debt = self.historical_debt;
                FixtureBaselineSettlement::Unknown(FixturePhaseBlocked {
                    continuation: self.into_continuation(),
                    event: terminal,
                    provenance,
                    historical_debt,
                })
            }
            FixtureBaselineFeedback::Known(receipt) => {
                self.finish_baseline(predecessor, &completion, &issued, receipt, terminal, now)
            }
        }
    }

    /// Retain only clock observations inside the admitted map segment.
    /// `ProvenNoEffect` changes neither baseline/model nor source credit; `Unknown` blocks.
    /// # Errors
    /// Refuse an inconsistent known/in-segment clock relation before consumption.
    fn baseline_terminal_relation(
        &self,
        issued: &BaselineIssued,
        event: FixtureBaselineFeedback,
        now: FixturePhaseNow,
    ) -> Result<Option<ControllerClockRelation>, FixtureBaselineError> {
        let observed = match event {
            FixtureBaselineFeedback::Known(receipt)
                if receipt.delivery.span.effective.1 <= issued.map.valid_until =>
            {
                Some(FixturePhaseNow::new(
                    receipt.delivery.context,
                    receipt.delivery.span.active.1,
                    receipt.delivery.span.effective.1,
                ))
            }
            FixtureBaselineFeedback::ProvenNoEffect { .. }
                if now.monotonic <= issued.map.valid_until =>
            {
                Some(now)
            }
            _ => None,
        };
        match (self.clock_relation, observed) {
            (Some(relation), Some(now)) => relation
                .observe(
                    now,
                    issued.map.numbers.ratio(),
                    issued.map.numbers.ratio_error(),
                )
                .map(Some),
            _ => Ok(self.clock_relation),
        }
    }

    /// Finish post-consumption arithmetic; no outer rejection or refund is possible.
    fn finish_baseline(
        mut self,
        predecessor: &FixtureAffineChart,
        completion: &super::FixtureRateSettlementCompletion,
        issued: &BaselineIssued,
        receipt: FixtureBaselineCompletion,
        terminal: FixtureRateSettlement,
        now: FixturePhaseNow,
    ) -> FixtureBaselineSettlement<'producer, 'chart> {
        let charts = baseline_feedback_charts(predecessor, completion, issued, receipt);
        let changed = receipt.achieved.rate.to_bits() != issued.old_baseline.rate.to_bits();
        let mut failure = None;
        let mut rebound = Err(FixtureBaselineError::Receipt);
        let replacement = match charts {
            Ok((effect, replacement)) => {
                if let Err(source) =
                    self.frequency
                        .rebase_baseline(predecessor, &effect, &replacement)
                {
                    failure = Some(FixtureBaselineError::Frequency { source });
                }
                rebound = rebound_baseline_map(issued, receipt, &replacement);
                Ok(replacement)
            }
            Err(error) => {
                self.frequency.rebase(
                    predecessor,
                    Err(FixtureChartError::Invariant),
                    Err(FixtureChartError::Invariant),
                );
                if matches!(&error, FixtureBaselineError::Map) {
                    rebound = Err(FixtureBaselineError::Map);
                }
                failure = Some(error);
                Err(FixtureChartError::Invariant)
            }
        };
        let debt = settled_debt(
            self.obligation,
            terminal,
            self.baseline,
            Some(self.baseline),
        );
        let refusal = continuation_refusal(
            self.profile,
            self.baseline,
            self.obligation,
            terminal,
            &debt,
            now,
        );
        let operation_refusal = if receipt.achieved.rate.abs() > self.profile.aggregate_cap {
            Some(FixturePhaseError::DeliveredOutOfEnvelope)
        } else if now.active >= issued.active_deadline || now.monotonic >= issued.monotonic_deadline
        {
            Some(FixturePhaseError::Expired)
        } else {
            None
        };
        if changed {
            self.baseline = receipt.achieved;
        }
        let original = self.obligation;
        let provenance = self.provenance;
        if let Ok(value) = &debt {
            self.historical_debt = Some(*value);
        }
        let frequency_history = self.frequency.history();
        let inner = FixturePhaseRetirement {
            frequency_history,
            continuation: Some(self.into_continuation()),
            event: terminal,
            replacement,
            debt,
            refusal,
            original,
            provenance,
        };
        FixtureBaselineSettlement::Known(FixtureBaselineRetirement {
            inner,
            achieved: receipt.achieved,
            map: rebound,
            changed,
            failure,
            operation_refusal,
        })
    }
}

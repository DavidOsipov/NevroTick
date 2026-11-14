/*
Chrony reference.c / local.c provenance for the modified finite request/actual
feedback boundary:
Copyright (C) Richard P. Curnow  1997-2003
Copyright (C) Miroslav Lichvar  2009-2018, 2020, 2022, 2025
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
//! Explicit matched constant steps inside the existing finite phase owner.
//!
//! This is the pure request/actual-feedback boundary of the Chrony-derived
//! controller, with positive displacement meaning local time is behind. It does
//! not adopt upstream automatic threshold/update-count policy or failure fallback.
//! A known actual aggregate interval and constant step share one cursor commit;
//! neither the requested step nor a boolean driver result acknowledges delivery.
//! Constant charts reexpress retained evidence over its unchanged domain, not the
//! physical wall-clock trajectory. No native mapping generation is supplied.
//! David Osipov with AI assistance, 13 November 2026: this is a modified Rust
//! fixture adaptation of the pinned Chrony 4.9 REF_SetReference -> local-clock
//! acknowledgement chain, with explicit actual-effect custody rather than its
//! boolean driver interface. Upstream threshold/count/fallback are not ported.

use std::{fmt, num::NonZeroU64};

use garde::Validate;
use time_domain::{ActiveAwakeNs, MonotonicNs};

use super::super::{FixtureRateSettlementCompletion, FixtureSettledRateInterval};
use super::{
    FixtureAffineChart, FixtureChartError, FixtureChartFrames, FixtureFrequencyHistory,
    FixturePhaseBaseline, FixturePhaseBlocked, FixturePhaseError, FixturePhaseNow,
    FixturePhasePlanOwner, FixturePhaseProvenance, FixturePhaseResidualPermission,
    FixturePhaseRetirement, FixtureRateClockContext, FixtureRateSettlement,
    FixtureRateSettlementCursor, PendingOperation, PhaseDebt, check_now, cursor_mapping,
    settled_debt,
};

/// Explicit allowed sign of one modeled step; no production policy is implied.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureStepDirections {
    /// Only positive local-clock advancement may be requested or acknowledged.
    Positive,
    /// Only negative local-clock displacement may be requested or acknowledged.
    Negative,
    /// Either established sign is allowed by this explicit fixture limit.
    Both,
}

impl FixtureStepDirections {
    /// Test supported stored bits without interpreting zero as a useful request.
    fn permits(self, displacement: f64) -> bool {
        match self {
            Self::Positive => displacement >= 0.0,
            Self::Negative => displacement <= 0.0,
            Self::Both => true,
        }
    }
}

/// Bounded refusal retaining no raw displacement, clock, identifier or Garde report.
#[derive(PartialEq)]
pub enum FixtureStepError {
    /// The original finite owner or checked numerical/chart consumer refused.
    Phase {
        /// Original bounded cause, independently of matched known-effect custody.
        source: FixturePhaseError,
    },
    /// The explicit per-operation magnitude is not a supported positive value.
    InvalidLimits,
    /// No established nonzero debt sign permits a useful constant request.
    NoUsefulStep,
    /// A requested or known step exceeds an explicit or original phase magnitude.
    Magnitude,
    /// A requested or known step has a prohibited or opposing sign.
    Direction,
    /// A receipt's complete immutable chain or current identity does not match.
    Receipt,
}

impl fmt::Debug for FixtureStepError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureStepError { .. }")
    }
}

impl fmt::Display for FixtureStepError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Phase { .. } => "fixture step: phase owner refused",
            Self::InvalidLimits => "fixture step: invalid explicit limits",
            Self::NoUsefulStep => "fixture step: no established nonzero debt",
            Self::Magnitude => "fixture step: magnitude exceeded",
            Self::Direction => "fixture step: direction refused",
            Self::Receipt => "fixture step: unmatched receipt",
        })
    }
}

impl std::error::Error for FixtureStepError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Phase { source } => Some(source),
            Self::InvalidLimits
            | Self::NoUsefulStep
            | Self::Magnitude
            | Self::Direction
            | Self::Receipt => None,
        }
    }
}

/// Wrap an existing exact bounded owner failure without discarding its cause.
fn phase_error(source: FixturePhaseError) -> FixtureStepError {
    FixtureStepError::Phase { source }
}

/// Explicit private per-request fixture limits, without any shipped default.
///
/// ```
/// use time_core::{FixtureStepDirections, FixtureStepLimits};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let limits = FixtureStepLimits::new(0.125, FixtureStepDirections::Both)?;
/// assert_eq!(limits.maximum_abs_seconds(), 0.125);
/// assert!(FixtureStepLimits::new(f64::NAN, FixtureStepDirections::Both).is_err());
/// # Ok(()) }
/// ```
///
/// ```compile_fail,E0451
/// use time_core::{FixtureStepDirections, FixtureStepLimits};
/// let _ = FixtureStepLimits { maximum: 1.0, directions: FixtureStepDirections::Both };
/// ```
/// ```compile_fail,E0616
/// use time_core::FixtureStepLimits;
/// fn bypass(limits: &mut FixtureStepLimits) { limits.maximum = f64::INFINITY; }
/// ```
#[derive(Clone, Copy)]
pub struct FixtureStepLimits {
    /// Strictly positive normal displacement cap for this one modeled operation.
    maximum: f64,
    /// Explicit closed sign permission, separate from current debt direction.
    directions: FixtureStepDirections,
}

/// Check all raw limit fields; the closed enum owns its own representation.
/// # Errors
/// Refuse unsupported or nonpositive magnitude without constructing any limit.
fn limits_rule(value: &FixtureStepLimits) -> Result<(), FixtureStepError> {
    if !value.maximum.is_normal() || value.maximum <= 0.0 {
        return Err(FixtureStepError::InvalidLimits);
    }
    match value.directions {
        FixtureStepDirections::Positive
        | FixtureStepDirections::Negative
        | FixtureStepDirections::Both => Ok(()),
    }
}

#[derive(Validate)]
/// Mandatory shared whole-group admission for otherwise untrusted limit inputs.
struct RawLimits {
    /// The rule owns magnitude and the explicit closed direction representation.
    #[garde(custom(|value: &FixtureStepLimits, (): &()| {
        limits_rule(value).map_err(|_| garde::Error::new("invalid explicit step limits"))
    }))]
    value: FixtureStepLimits,
}

impl FixtureStepLimits {
    /// Admit one explicit per-operation magnitude/direction group without effects.
    /// # Errors
    /// Reject unsupported limits or mandatory-validation disagreement unchanged.
    pub fn new(
        maximum_abs_seconds: f64,
        directions: FixtureStepDirections,
    ) -> Result<Self, FixtureStepError> {
        let raw = RawLimits {
            value: Self {
                maximum: maximum_abs_seconds,
                directions,
            },
        };
        let validated = raw.validate();
        limits_rule(&raw.value)?;
        validated.map_err(|_| phase_error(FixturePhaseError::ValidationInvariant))?;
        Ok(raw.value)
    }

    /// Copy the explicit cap without granting a physical correction allowance.
    #[must_use]
    pub const fn maximum_abs_seconds(&self) -> f64 {
        self.maximum
    }

    /// Copy the explicit sign permission; current debt sign is checked separately.
    #[must_use]
    pub const fn directions(&self) -> FixtureStepDirections {
        self.directions
    }
}

impl fmt::Debug for FixtureStepLimits {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureStepLimits { .. }")
    }
}

/// Immutable pending metadata inside the same Phase/Baseline/Step exclusive slot.
#[derive(Clone, Copy)]
pub(super) struct StepIssued {
    /// Existing persistent cursor ordinal; a request creates no second cursor.
    ordinal: NonZeroU64,
    /// Exact start readings to which the known full aggregate interval must bind.
    started: FixturePhaseNow,
    /// Requested current debt point/error, independently of actual delivery.
    requested: PhaseDebt,
    /// Explicit per-operation cap and direction, without new global policy.
    limits: FixtureStepLimits,
    /// Original unchanged active cutoff of the accepted finite obligation.
    active_deadline: ActiveAwakeNs,
    /// Original unchanged independent monotonic expiry.
    monotonic_deadline: MonotonicNs,
    /// Known effective aggregate baseline at request issuance.
    baseline: FixturePhaseBaseline,
    /// Original real accepted emission, never reconstructed from caller labels.
    provenance: FixturePhaseProvenance,
}

/// One complete already checked debt/limits group at the actual issue boundary.
struct StepRequest {
    /// Privately admitted numerical debt and inherited enclosure.
    debt: PhaseDebt,
    /// Immutable checked per-operation limit.
    limits: FixtureStepLimits,
    /// Original accepted owner's checked phase cap, never widened by this request.
    phase_cap: f64,
}

/// Enforce established sign and both actual issue-time magnitude authorities.
/// # Errors
/// Reject neutral debt, unsupported magnitude or prohibited sign; never clip.
fn request_rule(value: &StepRequest) -> Result<(), FixtureStepError> {
    let magnitude = value.debt.point().abs();
    if magnitude <= value.debt.error() {
        return Err(FixtureStepError::NoUsefulStep);
    }
    if magnitude > value.limits.maximum || magnitude > value.phase_cap {
        return Err(FixtureStepError::Magnitude);
    }
    if !value.limits.directions.permits(value.debt.point()) {
        return Err(FixtureStepError::Direction);
    }
    Ok(())
}

#[derive(Validate)]
/// Shared whole-group current request rule; each immutable operand keeps its owner.
struct RawStepRequest {
    #[garde(custom(|value: &StepRequest, (): &()| {
        request_rule(value).map_err(|_| garde::Error::new("invalid finite step request"))
    }))]
    /// Cross-field current sign/cap relationship checked before publication.
    value: StepRequest,
}

/// Borrowed actual request metadata; no conversion into an acknowledgement exists.
///
/// ```compile_fail,E0505
/// use time_core::{FixturePhaseNow, FixturePhasePlanOwner, FixturePhaseResidualPermission,
///     FixtureStepError, FixtureStepFeedback, FixtureStepLimits};
/// fn stale(mut owner: FixturePhasePlanOwner<'_, '_>, now: FixturePhaseNow,
///     limits: FixtureStepLimits, feedback: FixtureStepFeedback) -> Result<(), FixtureStepError> {
///     let proposal = owner.issue_step(now, FixturePhaseResidualPermission::Allow, limits)?;
///     let result = owner.settle_step(feedback, now);
///     std::hint::black_box(proposal.ordinal());
///     std::hint::black_box(result);
///     Ok(())
/// }
/// ```
pub struct FixtureStepProposal<'view> {
    /// Borrow of the actual metadata in the exclusive current pending slot.
    issued: &'view StepIssued,
}

impl FixtureStepProposal<'_> {
    /// Exact shared cursor ordinal of this issued request.
    #[must_use]
    pub const fn ordinal(&self) -> NonZeroU64 {
        self.issued.ordinal
    }
    /// Requested current displacement in seconds; never an applied effect.
    #[must_use]
    pub fn requested_phase_seconds(&self) -> f64 {
        self.issued.requested.point()
    }
    /// Inherited numerical debt enclosure; not statistical or physical error.
    #[must_use]
    pub fn requested_phase_error_seconds(&self) -> f64 {
        self.issued.requested.error()
    }
    /// Known modeled baseline accompanying the request, independently of actual rate.
    #[must_use]
    pub const fn known_baseline(&self) -> FixturePhaseBaseline {
        self.issued.baseline
    }
    /// Original accepted finite active expiry, not a renewed request lifetime.
    #[must_use]
    pub const fn active_deadline(&self) -> ActiveAwakeNs {
        self.issued.active_deadline
    }
    /// Original independent monotonic expiry.
    #[must_use]
    pub const fn monotonic_deadline(&self) -> MonotonicNs {
        self.issued.monotonic_deadline
    }
    /// Original real observation/emission facts, without newly issued source credit.
    #[must_use]
    pub const fn provenance(&self) -> &FixturePhaseProvenance {
        &self.issued.provenance
    }
}

impl fmt::Debug for FixtureStepProposal<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureStepProposal { .. }")
    }
}

/// Private checked known-rate-plus-step descriptor, never a native success token.
///
/// ```compile_fail,E0616
/// use time_core::FixtureStepCompletion;
/// fn bypass(receipt: &mut FixtureStepCompletion) { receipt.achieved = f64::NAN; }
/// ```
/// ```compile_fail,E0451
/// use time_core::FixtureStepCompletion;
/// fn forge(receipt: FixtureStepCompletion) {
///     let _ = FixtureStepCompletion { achieved: 0.0, ..receipt };
/// }
/// ```
#[derive(Clone, Copy)]
pub struct FixtureStepCompletion {
    /// Actual full aggregate interval, sharing this operation's context and ordinal.
    delivery: FixtureSettledRateInterval,
    /// Actual known constant displacement; both signed zero values remain Known.
    achieved: f64,
    /// Intermediate-to-final chart frames, sharing the interval's unchanged domain.
    final_frames: FixtureChartFrames,
}

/// Check supported step bits and the complete noncyclic two-link frame chain.
/// # Errors
/// Refuse raw scalar support or inconsistent frame endpoints without any mutation.
fn completion_rule(value: &FixtureStepCompletion) -> Result<(), FixtureStepError> {
    FixtureAffineChart::check_delivery_rate(value.achieved)
        .map_err(|source| phase_error(FixturePhaseError::Chart { source }))?;
    if value.final_frames.source_frame() != value.delivery.frames.target_frame()
        || value.final_frames.source_generation() != value.delivery.frames.target_generation()
    {
        return Err(FixtureStepError::Receipt);
    }
    FixtureChartFrames::new(
        value.delivery.frames.source_frame(),
        value.delivery.frames.source_generation(),
        value.final_frames.target_frame(),
        value.final_frames.target_generation(),
    )
    .map_err(|source| phase_error(FixturePhaseError::Chart { source }))?;
    Ok(())
}

#[derive(Validate)]
/// Mandatory scalar/representation/frame-chain admission before receipt publication.
struct RawStepCompletion {
    #[garde(custom(|value: &FixtureStepCompletion, (): &()| {
        completion_rule(value).map_err(|_| garde::Error::new("invalid known step descriptor"))
    }))]
    /// Immutable typed interval/frames plus raw actual displacement bits.
    value: FixtureStepCompletion,
}

impl FixtureStepCompletion {
    /// Retain an actual aggregate interval and independent actual constant step.
    /// # Errors
    /// Reject unsupported step, broken/cyclic frame chain or validation disagreement.
    pub fn new(
        delivery: FixtureSettledRateInterval,
        achieved_step_seconds: f64,
        final_frames: FixtureChartFrames,
    ) -> Result<Self, FixtureStepError> {
        let raw = RawStepCompletion {
            value: Self {
                delivery,
                achieved: achieved_step_seconds,
                final_frames,
            },
        };
        let validated = raw.validate();
        completion_rule(&raw.value)?;
        validated.map_err(|_| phase_error(FixturePhaseError::ValidationInvariant))?;
        Ok(raw.value)
    }
    /// Copy the exact original aggregate interval independently of all later failures.
    #[must_use]
    pub const fn delivery(&self) -> FixtureSettledRateInterval {
        self.delivery
    }
    /// Copy actual achieved displacement, never the requested point.
    #[must_use]
    pub const fn achieved_step_seconds(&self) -> f64 {
        self.achieved
    }
    /// Copy actual final modeled target tags, without native mapping validity.
    #[must_use]
    pub const fn final_frames(&self) -> FixtureChartFrames {
        self.final_frames
    }
}

impl fmt::Debug for FixtureStepCompletion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureStepCompletion { .. }")
    }
}

/// Actual modeled terminal facts; zero step and rate equality do not imply no effect.
#[derive(Clone, Copy)]
pub enum FixtureStepFeedback {
    /// Known full aggregate interval and separately known constant displacement.
    Known(FixtureStepCompletion),
    /// Explicit whole-operation absence; continuing baseline is not invented here.
    ProvenNoEffect {
        /// Original elapsed-clock association of this one operation.
        context: FixtureRateClockContext,
        /// Shared cursor ordinal consumed without a fabricated delivered interval.
        ordinal: NonZeroU64,
    },
    /// Unknown aggregate/step outcome; normal ownership remains permanently blocked.
    Unknown {
        /// Original elapsed-clock association of this one operation.
        context: FixtureRateClockContext,
        /// Shared ordinal consumed into unresolved custody, never retried as fresh.
        ordinal: NonZeroU64,
    },
}

impl FixtureStepFeedback {
    /// Reuse the original aggregate event's identity without acknowledging the step twice.
    fn terminal(self) -> FixtureRateSettlement {
        match self {
            Self::Known(value) => FixtureRateSettlement::Completed(value.delivery),
            Self::ProvenNoEffect { context, ordinal } => {
                FixtureRateSettlement::ProvenNoEffect { context, ordinal }
            }
            Self::Unknown { context, ordinal } => {
                FixtureRateSettlement::Unknown { context, ordinal }
            }
        }
    }
}

impl fmt::Debug for FixtureStepFeedback {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureStepFeedback { .. }")
    }
}

impl FixturePhasePlanOwner<'_, '_> {
    /// Reserve one explicit established-sign step through the same pending slot.
    ///
    /// The unchanged carried point is requested, never silently clipped. Limits
    /// are explicit caller assertions, not automatic selection or clock authority.
    /// Valid current readings and latched expiry survive later request refusal.
    /// # Errors
    /// Reject retired/pending/mismapped state, time, absent/expired/reused debt,
    /// neutral sign or unsupported requested magnitude/direction without issuance.
    pub fn issue_step(
        &mut self,
        now: FixturePhaseNow,
        permission: FixturePhaseResidualPermission,
        limits: FixtureStepLimits,
    ) -> Result<FixtureStepProposal<'_>, FixtureStepError> {
        self.authority_ready().map_err(phase_error)?;
        if self.issued.is_some() {
            return Err(phase_error(FixturePhaseError::OutstandingOperation));
        }
        cursor_mapping(&self.cursor, self.chart()).map_err(phase_error)?;
        check_now(self.last_now, &self.cursor, now).map_err(phase_error)?;
        self.last_now = now;
        let old = self
            .obligation
            .as_mut()
            .ok_or_else(|| phase_error(FixturePhaseError::NoReference))?;
        if old.expired
            || now.active >= old.active_deadline
            || now.monotonic >= old.monotonic_deadline
        {
            old.expired = true;
            return Err(phase_error(FixturePhaseError::Expired));
        }
        if old.residual && permission != FixturePhaseResidualPermission::Allow {
            return Err(phase_error(FixturePhaseError::ResidualPermission));
        }
        let raw = RawStepRequest {
            value: StepRequest {
                debt: old.debt,
                limits,
                phase_cap: self.profile.phase_cap,
            },
        };
        let validated = raw.validate();
        request_rule(&raw.value)?;
        validated.map_err(|_| phase_error(FixturePhaseError::ValidationInvariant))?;
        let ordinal = self
            .cursor
            .next_ordinal()
            .ok_or_else(|| phase_error(FixturePhaseError::CursorUnavailable))?;
        self.issued = Some(PendingOperation::Step(StepIssued {
            ordinal,
            started: now,
            requested: old.debt,
            limits,
            active_deadline: old.active_deadline,
            monotonic_deadline: old.monotonic_deadline,
            baseline: self.baseline,
            provenance: self
                .provenance
                .ok_or_else(|| phase_error(FixturePhaseError::ValidationInvariant))?,
        }));
        self.step_proposal()
            .ok_or_else(|| phase_error(FixturePhaseError::ValidationInvariant))
    }

    /// Borrow the existing step request without another ordinal, emission or clock read.
    #[must_use]
    pub fn step_proposal(&self) -> Option<FixtureStepProposal<'_>> {
        if self.authority.is_some() || self.obligation.is_some_and(|old| old.expired) {
            return None;
        }
        match self.issued.as_ref()? {
            PendingOperation::Step(issued) => Some(FixtureStepProposal { issued }),
            PendingOperation::Phase(_) | PendingOperation::Baseline(_) => None,
        }
    }
}

/// One complete pending/event/now relationship before any historical consumption.
struct StepTerminal {
    /// Immutable actual issued metadata owned by the exclusive lifecycle.
    issued: StepIssued,
    /// Supplied terminal descriptor, with no authority inferred from construction.
    event: FixtureStepFeedback,
    /// Actual supplied upper bounds in the independently ordered elapsed clocks.
    now: FixturePhaseNow,
}

/// Require exact issued identity and full ordered known interval before consumption.
/// # Errors
/// Refuse wrong context/ordinal or endpoints outside issue/current readings unchanged.
fn terminal_rule(value: &StepTerminal) -> Result<(), FixtureStepError> {
    let (context, ordinal) = value.event.terminal().identity();
    if context != value.issued.started.context || ordinal != value.issued.ordinal {
        return Err(FixtureStepError::Receipt);
    }
    if let FixtureStepFeedback::Known(receipt) = value.event {
        let span = receipt.delivery.span;
        if span.active.0 != value.issued.started.active
            || span.effective.0 != value.issued.started.monotonic
            || span.active.1 > value.now.active
            || span.effective.1 > value.now.monotonic
        {
            return Err(FixtureStepError::Receipt);
        }
    }
    Ok(())
}

#[derive(Validate)]
/// Mandatory actual pending-to-terminal relationship admission.
struct RawStepTerminal {
    #[garde(custom(|value: &StepTerminal, (): &()| {
        terminal_rule(value).map_err(|_| garde::Error::new("invalid matched step terminal"))
    }))]
    /// Whole unchanged relationship checked before the shared cursor commit.
    value: StepTerminal,
}

/// Admit the same current whole-group rule through mandatory Garde and typed errors.
/// # Errors
/// Preserve exact relationship refusal or validation disagreement without mutation.
fn check_terminal(value: StepTerminal) -> Result<(), FixtureStepError> {
    let raw = RawStepTerminal { value };
    let validated = raw.validate();
    terminal_rule(&raw.value)?;
    validated.map_err(|_| phase_error(FixturePhaseError::ValidationInvariant))
}

/// Result of one consuming step transition, preserving the actual owner on rejection.
#[must_use]
#[expect(
    clippy::large_enum_variant,
    reason = "The bounded Known variant retains the actual requested step, achieved step, original aggregate interval, partial chart/debt results and sole producer/cursor transfer inline; boxing these historical facts would add a success-path heap allocation and change the established finite-owner custody contract."
)]
pub enum FixtureStepSettlement<'producer, 'chart> {
    /// Structural/current-state admission failed before consumption.
    OuterRejected {
        /// Original owner, including its unchanged pending step and producer lease.
        owner: FixturePhasePlanOwner<'producer, 'chart>,
        /// Original bounded relationship or current-state failure.
        error: FixtureStepError,
    },
    /// Whole-operation absence consumed its ordinal, with original allowance retained.
    ProvenNoEffect(FixturePhasePlanOwner<'producer, 'chart>),
    /// Known facts consumed once, independently of all later arithmetic/policy results.
    Known(FixtureStepRetirement<'producer>),
    /// Actual unresolved event consumes and permanently blocks the common cursor.
    Unknown(FixturePhaseBlocked<'producer>),
}

impl fmt::Debug for FixtureStepSettlement<'_, '_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureStepSettlement { .. }")
    }
}

/// Known constant-step facts plus the original take-once producer/cursor custody.
pub struct FixtureStepRetirement<'producer> {
    /// Shared historical debt/chart/producer custody, without a public residual resume.
    inner: FixturePhaseRetirement<'producer>,
    /// Original actual requested value/error/identity survives every later failure.
    issued: StepIssued,
    /// Independent known S and original interval/final chain survive every failure.
    receipt: FixtureStepCompletion,
    /// Actual first-link chart results survive a later constant/composition failure.
    rate_completion: FixtureRateSettlementCompletion,
    /// Successful rate-only debt survives a later constant subtraction failure.
    pre_step_debt: Option<PhaseDebt>,
    /// Original post-consumption complete chart-construction failure, if any.
    failure: Option<FixtureStepError>,
    /// Post-accounting policy/authority refusal, never substituted for known facts.
    operation_refusal: Option<FixtureStepError>,
}

impl fmt::Debug for FixtureStepRetirement<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureStepRetirement { .. }")
    }
}

/// Compose the actual aggregate effect with the independent constant step once.
/// # Errors
/// Preserve the first rate/composition failure or later checked chart cause exactly.
fn feedback_charts(
    predecessor: &FixtureAffineChart,
    completion: &FixtureRateSettlementCompletion,
    receipt: FixtureStepCompletion,
) -> Result<(FixtureAffineChart, FixtureAffineChart), FixtureChartError> {
    let rate = completion.effect()?.ok_or(FixtureChartError::Invariant)?;
    completion
        .replacement()?
        .ok_or(FixtureChartError::Invariant)?;
    let constant = FixtureAffineChart::new(
        receipt.final_frames,
        predecessor.domain(),
        receipt.achieved,
        -0.0,
    )?;
    // Keep RN association predecessor.compose(rate.compose(constant)); neither
    // regrouping the rate/debt arithmetic nor a fabricated one-second span occurs.
    let effect = rate.compose(&constant)?;
    let replacement = predecessor.compose(&effect)?;
    Ok((effect, replacement))
}

/// Classify known out-of-contract effects only after actual debt/chart accounting.
/// This pure classifier never changes or refunds any historical fact.
fn operation_refusal(
    owner: &FixturePhasePlanOwner<'_, '_>,
    issued: StepIssued,
    receipt: FixtureStepCompletion,
    debt: &Result<PhaseDebt, FixturePhaseError>,
    now: FixturePhaseNow,
) -> Option<FixtureStepError> {
    if receipt.delivery.rate.to_bits() != issued.baseline.rate.to_bits() {
        return Some(phase_error(FixturePhaseError::BaselineChanged));
    }
    if receipt.delivery.rate.abs() > owner.profile.aggregate_cap {
        return Some(phase_error(FixturePhaseError::DeliveredOutOfEnvelope));
    }
    if receipt.achieved.abs() > issued.limits.maximum
        || receipt.achieved.abs() > owner.profile.phase_cap
    {
        return Some(FixtureStepError::Magnitude);
    }
    if !issued.limits.directions.permits(receipt.achieved)
        || (issued.requested.point() > 0.0 && receipt.achieved < 0.0)
        || (issued.requested.point() < 0.0 && receipt.achieved > 0.0)
    {
        return Some(FixtureStepError::Direction);
    }
    if let Ok(next) = debt {
        if (issued.requested.point() > 0.0 && next.point() < 0.0)
            || (issued.requested.point() < 0.0 && next.point() > 0.0)
        {
            return Some(phase_error(FixturePhaseError::DeliveredOvershoot));
        }
        if next.point().abs() > owner.profile.phase_cap {
            return Some(phase_error(FixturePhaseError::PhaseEnvelope));
        }
    }
    if let Some(reason) = owner.authority {
        return Some(phase_error(FixturePhaseError::Retired { reason }));
    }
    if owner.obligation.is_some_and(|old| old.expired)
        || now.active >= issued.active_deadline
        || now.monotonic >= issued.monotonic_deadline
    {
        return Some(phase_error(FixturePhaseError::Expired));
    }
    None
}

impl<'producer, 'chart> FixturePhasePlanOwner<'producer, 'chart> {
    /// Consume one matched step event through the existing shared cursor once.
    ///
    /// Stop/expiry or an unexpected actual aggregate never erases known history.
    /// Only outer structural/context/order rejection returns an unchanged owner.
    /// Known zero remains Known; uncertain completion blocks without retry credit.
    pub fn settle_step(
        mut self,
        event: FixtureStepFeedback,
        now: FixturePhaseNow,
    ) -> FixtureStepSettlement<'producer, 'chart> {
        let Some(PendingOperation::Step(issued)) = self.issued else {
            return FixtureStepSettlement::OuterRejected {
                owner: self,
                error: phase_error(FixturePhaseError::NoOutstandingOperation),
            };
        };
        let preflight = check_now(self.last_now, &self.cursor, now)
            .map_err(phase_error)
            .and_then(|()| check_terminal(StepTerminal { issued, event, now }));
        if let Err(error) = preflight {
            return FixtureStepSettlement::OuterRejected { owner: self, error };
        }
        let predecessor = self.readiness.chart;
        let terminal = event.terminal();
        let consumed = match event {
            FixtureStepFeedback::Known(receipt) => self.readiness.receive_step_settlement(
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
            Ok(completion) => completion,
            Err(source) => {
                return FixtureStepSettlement::OuterRejected {
                    owner: self,
                    error: phase_error(FixturePhaseError::Delivery { source }),
                };
            }
        };
        self.last_now = now;
        self.issued = None;
        match event {
            FixtureStepFeedback::ProvenNoEffect { .. } => {
                if let Some(old) = self.obligation.as_mut() {
                    old.residual = true;
                    old.expired |= now.active >= old.active_deadline
                        || now.monotonic >= old.monotonic_deadline;
                }
                FixtureStepSettlement::ProvenNoEffect(self)
            }
            FixtureStepFeedback::Unknown { .. } => {
                self.frequency.unknown();
                let provenance = self.provenance;
                let historical_debt = self.historical_debt;
                FixtureStepSettlement::Unknown(FixturePhaseBlocked {
                    continuation: self.into_continuation(),
                    event: terminal,
                    provenance,
                    historical_debt,
                })
            }
            FixtureStepFeedback::Known(receipt) => {
                self.finish_step(predecessor, completion, issued, receipt, terminal, now)
            }
        }
    }

    /// Retain and compute all post-consumption facts; no ordinal can be refunded.
    fn finish_step(
        mut self,
        predecessor: &FixtureAffineChart,
        completion: FixtureRateSettlementCompletion,
        issued: StepIssued,
        receipt: FixtureStepCompletion,
        terminal: FixtureRateSettlement,
        now: FixturePhaseNow,
    ) -> FixtureStepSettlement<'producer, 'chart> {
        let charts = feedback_charts(predecessor, &completion, receipt);
        let mut failure = None;
        let replacement = match charts {
            Ok((effect, replacement)) => {
                self.frequency
                    .rebase(predecessor, Ok(Some(&effect)), Ok(Some(&replacement)));
                Ok(replacement)
            }
            Err(source) => {
                self.frequency.rebase(predecessor, Err(source), Err(source));
                failure = Some(phase_error(FixturePhaseError::Chart { source }));
                Err(source)
            }
        };
        let (pre_step_debt, debt) = match settled_debt(
            self.obligation,
            terminal,
            issued.baseline,
            Some(issued.baseline),
        ) {
            Ok(before) => (
                Some(before),
                before
                    .stepped(receipt.achieved)
                    .map_err(|source| FixturePhaseError::Numeric { source }),
            ),
            Err(error) => (None, Err(error)),
        };
        let operation_refusal = operation_refusal(&self, issued, receipt, &debt, now);
        let original = self.obligation;
        let provenance = self.provenance;
        if let Ok(next) = &debt {
            self.historical_debt = Some(*next);
        } else if let Some(before) = pre_step_debt {
            self.historical_debt = Some(before);
        }
        // This exact active/monotonic relation contains no UTC displacement.
        // Keep it and the accepted-reference active stamp across sole transfer.
        // The old absolute map belongs to the old full chart basis and cannot
        // match the newly targeted replacement; no map is silently rebound here.
        let frequency_history = self.frequency.history();
        let inner = FixturePhaseRetirement {
            frequency_history,
            continuation: Some(self.into_continuation()),
            event: terminal,
            replacement,
            debt,
            refusal: None,
            original,
            provenance,
        };
        FixtureStepSettlement::Known(FixtureStepRetirement {
            inner,
            issued,
            receipt,
            rate_completion: completion,
            pre_step_debt,
            failure,
            operation_refusal,
        })
    }
}

impl<'producer> FixtureStepRetirement<'producer> {
    /// Original consumed shared identity remains inspectable after sole transfer.
    #[must_use]
    pub const fn ordinal(&self) -> NonZeroU64 {
        self.issued.ordinal
    }
    /// Borrow all independently known original completion facts without recomputing them.
    #[must_use]
    pub const fn completion(&self) -> &FixtureStepCompletion {
        &self.receipt
    }
    /// Borrow the full actual replacement or retain its exact post-commit chart error.
    /// # Errors
    /// Return the original arithmetic/composition failure without restoring credit.
    pub fn replacement(&self) -> Result<&FixtureAffineChart, FixtureChartError> {
        self.inner.replacement()
    }
    /// Original full-chart failure, independent of both known rate interval and S.
    #[must_use]
    pub fn feedback_error(&self) -> Option<&FixtureStepError> {
        self.failure.as_ref()
    }
    /// Actual policy/authority refusal, after all supported actual accounting.
    #[must_use]
    pub fn continuation_error(&self) -> Option<&FixtureStepError> {
        self.operation_refusal.as_ref()
    }
    /// Exact original aggregate event; its Known S is separately retained below.
    #[must_use]
    pub const fn event(&self) -> &FixtureRateSettlement {
        self.inner.event()
    }
    /// Known actual constant displacement survives arithmetic failure and transfer.
    #[must_use]
    pub const fn achieved_step_seconds(&self) -> f64 {
        self.receipt.achieved
    }
    /// Original requested point survives transfer; it is never applied feedback.
    #[must_use]
    pub fn requested_phase_seconds(&self) -> f64 {
        self.issued.requested.point()
    }
    /// Original requested numerical enclosure, independently of actual S.
    #[must_use]
    pub fn requested_phase_error_seconds(&self) -> f64 {
        self.issued.requested.error()
    }
    /// Final historical accounted debt when both actual numerical stages succeeded.
    #[must_use]
    pub fn remaining_phase_seconds(&self) -> Option<f64> {
        self.inner.remaining_phase_seconds()
    }
    /// Numerical enclosure of the same final historical debt.
    #[must_use]
    pub fn remaining_phase_error_seconds(&self) -> Option<f64> {
        self.inner.remaining_phase_error_seconds()
    }
    /// Partial actual rate-only debt survives a later constant subtraction failure.
    #[must_use]
    pub fn pre_step_phase_seconds(&self) -> Option<f64> {
        self.pre_step_debt.map(|value| value.point())
    }
    /// Numerical enclosure paired with that same partial actual debt.
    #[must_use]
    pub fn pre_step_phase_error_seconds(&self) -> Option<f64> {
        self.pre_step_debt.map(|value| value.error())
    }
    /// Original debt failure, independently of a chart failure or later policy refusal.
    #[must_use]
    pub fn debt_error(&self) -> Option<&FixturePhaseError> {
        self.inner.debt_error()
    }
    /// Actual shared cursor remains inspectable until the sole transfer is consumed.
    #[must_use]
    pub fn cursor(&self) -> Option<&FixtureRateSettlementCursor> {
        self.inner.cursor()
    }
    /// Genuine old source metadata remains historical after context-only recovery.
    #[must_use]
    pub const fn provenance(&self) -> Option<&FixturePhaseProvenance> {
        self.inner.provenance()
    }
    /// Original historical active deadline; never a new phase allowance.
    #[must_use]
    pub const fn active_deadline(&self) -> ActiveAwakeNs {
        self.issued.active_deadline
    }
    /// Original historical monotonic deadline.
    #[must_use]
    pub const fn monotonic_deadline(&self) -> MonotonicNs {
        self.issued.monotonic_deadline
    }
    /// Reexpressed model history, without restoring observation or phase credit.
    #[must_use]
    pub const fn frequency_history(&self) -> Option<FixtureFrequencyHistory> {
        self.inner.frequency_history()
    }
    /// Borrow the admitted intermediate chart independently of later step failure.
    /// # Errors
    /// Return the exact original actual aggregate chart/composition cause.
    pub fn rate_replacement(&self) -> Result<Option<&FixtureAffineChart>, FixtureChartError> {
        self.rate_completion.replacement()
    }

    /// Take context-only custody once, requiring genuinely new reference admission.
    ///
    /// No old debt/adaptive area is restored. The independently retained elapsed
    /// counter relation and prior accepted-reference stamp remain. A baseline
    /// operation requires a new map on the actual replacement basis; this method
    /// grants no production UTC mapping validity or native generation continuity.
    /// # Errors
    /// Refuse known policy failure, failed replacement, retired/terminal custody,
    /// invalid current readings or an already consumed transfer; never mint credit.
    pub fn resume_fresh<'new>(
        &'new mut self,
        now: FixturePhaseNow,
    ) -> Result<FixturePhasePlanOwner<'producer, 'new>, FixtureStepError> {
        if self.operation_refusal.is_some() {
            return Err(FixtureStepError::Receipt);
        }
        self.inner
            .resume_fresh(now, self.issued.baseline)
            .map_err(phase_error)
    }
}

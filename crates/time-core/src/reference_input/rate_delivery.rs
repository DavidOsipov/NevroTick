// SPDX-License-Identifier: GPL-2.0-only
//! Local, inert accounting of settled active-time rate contributions.
//!
//! Known delivery reexpresses the same retained evidence in a counterfactual
//! coordinate frame; it does not reconstruct a physical ramp. A cursor survives
//! replacement chart owners, but its reconstruction grants no global replay,
//! native acknowledgement, durable settlement or clock authority. All storage
//! and receiving work are constant. Histories and observation credit are untouched.

/// Owning finite phase obligations layered on the existing terminal receiver.
mod phase_plan;

pub use phase_plan::{
    FixturePhaseBaseline, FixturePhaseBlocked, FixturePhaseConstructionRejected, FixturePhaseError,
    FixturePhaseNow, FixturePhasePlanOwner, FixturePhaseProfile, FixturePhaseProposal,
    FixturePhaseProvenance, FixturePhaseReferenceAdmission, FixturePhaseResidualPermission,
    FixturePhaseRetirement, FixturePhaseRetirementReason, FixturePhaseSettlement,
};

use std::{fmt, num::NonZeroU64};

use garde::Validate;
use time_domain::{ActiveAwakeDurationNs, ActiveAwakeNs, MonotonicNs};

use super::{
    FixtureAffineChart, FixtureChartDomain, FixtureChartError, FixtureChartFrames,
    FixtureCorrectionFeedback, FixtureReferenceInputError, FixtureReferenceInputState,
    reference_chart_identity, reference_chart_mapping,
};

/// Independently ordered clock domain; no conversion between these units exists.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureRateClock {
    /// Elapsed active-awake nanoseconds supply integrated delivery.
    ActiveAwake,
    /// Monotonic endpoints supply ordering/effective-event provenance only.
    Monotonic,
}

/// Caller-asserted clock labels, without authentication or native continuity.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct FixtureRateClockContext {
    /// Nonzero equality tag for one monotonic fixture origin.
    monotonic_origin: NonZeroU64,
    /// Nonzero equality tag for one independent active-awake fixture origin.
    active_origin: NonZeroU64,
    /// Shared caller assertion of the modeled clock-generation association.
    generation: NonZeroU64,
}

impl FixtureRateClockContext {
    /// Retain already nonzero equality tags without inferring clock correspondence.
    #[must_use]
    pub const fn new(
        monotonic_origin: NonZeroU64,
        active_origin: NonZeroU64,
        generation: NonZeroU64,
    ) -> Self {
        Self {
            monotonic_origin,
            active_origin,
            generation,
        }
    }
}

impl fmt::Debug for FixtureRateClockContext {
    /// Redact all caller-asserted labels with fixed output.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureRateClockContext { .. }")
    }
}

/// Closed state of one local settlement cursor, not a broker operation journal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureRateCursorStatus {
    /// Exactly one next ordinal is admissible under current relationship guards.
    Pending,
    /// An unknown outcome blocks further normal settlement on this cursor.
    Unresolved,
    /// The maximum ordinal was consumed; no counter wrap or reuse is possible.
    Exhausted,
}

/// Bounded admission failure; rejected payloads and their numeric values are absent.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum FixtureRateDeliveryError {
    /// Current readiness, exact identity or composition relationship was rejected.
    Reference {
        /// Exact bounded reference-owner cause.
        source: FixtureReferenceInputError,
    },
    /// This cursor cannot accept another current event.
    CursorUnavailable {
        /// The unchanged terminal local state.
        status: FixtureRateCursorStatus,
    },
    /// Caller-asserted clock origins/generation differ from the cursor's tags.
    ClockContextMismatch,
    /// The event is not the cursor's exact next local operation ordinal.
    OrdinalMismatch,
    /// A replacement owner uses old/different mathematical target coordinates.
    CursorMappingMismatch,
    /// A constructor received a reversed pair in the stated clock domain.
    InvalidSpan {
        /// The independent domain whose pair was reversed.
        clock: FixtureRateClock,
    },
    /// A known active span strictly overlaps a previously consumed known span.
    ActiveOverlap,
    /// A known monotonic span strictly overlaps its previous effective span.
    MonotonicOverlap,
    /// A raw rate or prospective endpoint chain failed its maintained owner.
    Chart {
        /// Exact bounded chart cause.
        source: FixtureChartError,
    },
    /// Mandatory Garde and the shared typed rule disagreed before mutation.
    ValidationInvariant,
}

impl fmt::Debug for FixtureRateDeliveryError {
    /// Emit a fixed redacted family without formatting rejected records.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureRateDeliveryError { .. }")
    }
}

impl fmt::Display for FixtureRateDeliveryError {
    /// Preserve bounded failure families without source tags, ordinals or times.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Reference { .. } => "fixture rate delivery: reference rejected",
            Self::CursorUnavailable { .. } => "fixture rate delivery: cursor unavailable",
            Self::ClockContextMismatch => "fixture rate delivery: clock context mismatch",
            Self::OrdinalMismatch => "fixture rate delivery: ordinal mismatch",
            Self::CursorMappingMismatch => "fixture rate delivery: cursor mapping mismatch",
            Self::InvalidSpan { .. } => "fixture rate delivery: reversed clock span",
            Self::ActiveOverlap => "fixture rate delivery: active span overlaps",
            Self::MonotonicOverlap => "fixture rate delivery: monotonic span overlaps",
            Self::Chart { .. } => "fixture rate delivery: chart rejected",
            Self::ValidationInvariant => "fixture rate delivery: validation invariant",
        })
    }
}

impl std::error::Error for FixtureRateDeliveryError {
    /// Borrow only bounded typed causes; formatting remains independently redacted.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Reference { source } => Some(source),
            Self::Chart { source } => Some(source),
            _ => None,
        }
    }
}

/// Ordered pairs before construction, retaining their independent unit tags.
struct SpanInput {
    /// Supplied active-awake start/end, never inferred from the effective pair.
    active: (ActiveAwakeNs, ActiveAwakeNs),
    /// Supplied monotonic start/end, never used as elapsed delivery.
    effective: (MonotonicNs, MonotonicNs),
}

/// Check both pairs and return the exact typed active duration, without effects.
/// # Errors
/// Active reversal precedes monotonic reversal. All ordered u64 spans are allowed;
/// the numerical support envelope is deliberately applied only after settlement.
fn span_duration(value: &SpanInput) -> Result<ActiveAwakeDurationNs, FixtureRateDeliveryError> {
    let duration = value.active.1.duration_since(value.active.0).map_err(|_| {
        FixtureRateDeliveryError::InvalidSpan {
            clock: FixtureRateClock::ActiveAwake,
        }
    })?;
    value
        .effective
        .1
        .duration_since(value.effective.0)
        .map_err(|_| FixtureRateDeliveryError::InvalidSpan {
            clock: FixtureRateClock::Monotonic,
        })?;
    Ok(duration)
}

/// Mandatory shared order rule inside actual span construction.
#[derive(Validate)]
struct RawSpan {
    /// Typed endpoints own their units; this rule owns both pair orderings.
    #[garde(custom(|value: &SpanInput, (): &()| {
        span_duration(value).map(|_| ())
            .map_err(|_| garde::Error::new("invalid fixture rate span"))
    }))]
    value: SpanInput,
}

/// Immutable ordered half-open delivery/effective spans, including zero width.
///
/// Clock domains cannot be substituted for each other:
/// ```compile_fail,E0308
/// use time_core::FixtureRateSpan;
/// use time_domain::MonotonicNs;
/// let pair = (MonotonicNs::new(0), MonotonicNs::new(1));
/// let _ = FixtureRateSpan::new(pair, pair);
/// ```
/// Constructor-owned ordering cannot be bypassed by writing a retained pair:
/// ```compile_fail,E0616
/// use time_core::FixtureRateSpan;
/// use time_domain::ActiveAwakeNs;
/// fn bypass(span: &mut FixtureRateSpan) {
///     span.active = (ActiveAwakeNs::new(2), ActiveAwakeNs::new(1));
/// }
/// ```
#[derive(Clone, Copy)]
pub struct FixtureRateSpan {
    /// Original independently checked active endpoints.
    active: (ActiveAwakeNs, ActiveAwakeNs),
    /// Original independently checked effective monotonic endpoints.
    effective: (MonotonicNs, MonotonicNs),
    /// Exact same-domain difference, without absolute timestamp conversion.
    active_elapsed: ActiveAwakeDurationNs,
}

impl FixtureRateSpan {
    /// Admit both exact endpoint orderings through mandatory Garde/shared rules.
    /// # Errors
    /// Preserve the first reversed clock pair or validation disagreement.
    pub fn new(
        active: (ActiveAwakeNs, ActiveAwakeNs),
        effective: (MonotonicNs, MonotonicNs),
    ) -> Result<Self, FixtureRateDeliveryError> {
        let raw = RawSpan {
            value: SpanInput { active, effective },
        };
        let validated = raw.validate();
        let active_elapsed = span_duration(&raw.value)?;
        validated.map_err(|_| FixtureRateDeliveryError::ValidationInvariant)?;
        Ok(Self {
            active,
            effective,
            active_elapsed,
        })
    }

    /// Return the checked elapsed value without converting clock domains.
    #[must_use]
    pub const fn active_elapsed(&self) -> ActiveAwakeDurationNs {
        self.active_elapsed
    }
}

impl fmt::Debug for FixtureRateSpan {
    /// Redact all captured readings and durations with fixed output.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureRateSpan { .. }")
    }
}

/// Known delivered portion; stored-bit rate validation is the only new scalar rule.
///
/// The admitted rate cannot be replaced by an unchecked scalar:
/// ```compile_fail,E0616
/// use time_core::FixtureSettledRateInterval;
/// fn bypass(interval: &mut FixtureSettledRateInterval) {
///     interval.rate = f64::NAN;
/// }
/// ```
#[derive(Clone, Copy)]
pub struct FixtureSettledRateInterval {
    /// Caller-asserted immutable clock-origin/generation association.
    context: FixtureRateClockContext,
    /// One local operation identity, checked against the actual mutable cursor.
    ordinal: NonZeroU64,
    /// Constructor-owned ordered typed endpoints and exact active duration.
    span: FixtureRateSpan,
    /// Supported stored binary64 applied rate, not a requested quantity.
    rate: f64,
    /// Constructor-owned distinct prospective B-to-C mapping tags.
    frames: FixtureChartFrames,
    /// Complete checked mathematical domain/reference, not the delivery window.
    domain: FixtureChartDomain,
}

/// Mandatory rate rule over otherwise immutable constructor-owned groups.
#[derive(Validate)]
struct RawInterval {
    /// The shared numerical owner checks rate; typed fields retain other invariants.
    #[garde(custom(|value: &FixtureSettledRateInterval, (): &()| {
        FixtureAffineChart::check_delivery_rate(value.rate)
            .map_err(|_| garde::Error::new("invalid fixture applied rate"))
    }))]
    value: FixtureSettledRateInterval,
}

impl FixtureSettledRateInterval {
    /// Admit the known modeled portion with mandatory rate validation, no effects.
    ///
    /// Current context, ordinal, mapping and overlap belong to the receiver.
    /// Numerical duration/product support is checked after terminal admission.
    /// # Errors
    /// Preserve the exact unsupported stored-rate cause or Garde disagreement.
    pub fn new(
        context: FixtureRateClockContext,
        ordinal: NonZeroU64,
        span: FixtureRateSpan,
        rate: f64,
        frames: FixtureChartFrames,
        domain: FixtureChartDomain,
    ) -> Result<Self, FixtureRateDeliveryError> {
        let raw = RawInterval {
            value: Self {
                context,
                ordinal,
                span,
                rate,
                frames,
                domain,
            },
        };
        let validated = raw.validate();
        FixtureAffineChart::check_delivery_rate(raw.value.rate)
            .map_err(|source| FixtureRateDeliveryError::Chart { source })?;
        validated.map_err(|_| FixtureRateDeliveryError::ValidationInvariant)?;
        Ok(raw.value)
    }
}

impl fmt::Debug for FixtureSettledRateInterval {
    /// Redact rate, labels, ordinal and timestamps with fixed output.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureSettledRateInterval { .. }")
    }
}

/// Closed modeled terminal observation, without native acknowledgement authority.
#[derive(Clone, Copy)]
pub enum FixtureRateSettlement {
    /// Known cessation with its actual modeled delivered interval/rate.
    Completed(FixtureSettledRateInterval),
    /// Known cancellation retaining the actually delivered portion, not a refund.
    Cancelled(FixtureSettledRateInterval),
    /// Explicit modeled proof of no effect, with no invented endpoint interval.
    ProvenNoEffect {
        /// Caller-asserted clock association.
        context: FixtureRateClockContext,
        /// Local ordinal being settled.
        ordinal: NonZeroU64,
    },
    /// Unknown effect; consumes/blocks without inventing a phase or endpoint.
    Unknown {
        /// Caller-asserted clock association.
        context: FixtureRateClockContext,
        /// Local ordinal being settled.
        ordinal: NonZeroU64,
    },
}

impl fmt::Debug for FixtureRateSettlement {
    /// Redact all event payloads, including local operation ordinals.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureRateSettlement { .. }")
    }
}

impl FixtureRateSettlement {
    /// Copy immutable receipt identity without treating it as authenticated proof.
    const fn identity(self) -> (FixtureRateClockContext, NonZeroU64) {
        match self {
            Self::Completed(interval) | Self::Cancelled(interval) => {
                (interval.context, interval.ordinal)
            }
            Self::ProvenNoEffect { context, ordinal } | Self::Unknown { context, ordinal } => {
                (context, ordinal)
            }
        }
    }
}

/// Private state encodes exactly one next ordinal or a terminal local condition.
enum CursorProgress {
    /// Current expected local operation identity.
    Pending(NonZeroU64),
    /// No further normal settlement can reconcile an unknown effect.
    Unresolved,
    /// Ordinal exhaustion is explicit, never wrapped or refunded.
    Exhausted,
}

/// Persistent local settlement cursor; replacements cannot erase its high-water marks.
///
/// The cursor is intentionally non-Clone and has no reset/recovery constructor.
/// A new independent fixture instance is outside its at-most-once guarantee.
/// ```compile_fail,E0277
/// use time_core::FixtureRateSettlementCursor;
/// fn duplicate<T: Clone>(_: T) {}
/// fn copy_cursor(cursor: FixtureRateSettlementCursor) {
///     duplicate(cursor);
/// }
/// ```
pub struct FixtureRateSettlementCursor {
    /// Immutable caller-asserted clock context.
    context: FixtureRateClockContext,
    /// Current usable local operation state.
    progress: CursorProgress,
    /// Expected current mathematical target frame, updated after known admission.
    target_frame: NonZeroU64,
    /// Expected current target generation, not numerically ordered.
    target_generation: NonZeroU64,
    /// Last known consumed active endpoint; Unknown supplies no new endpoint.
    last_active: Option<ActiveAwakeNs>,
    /// Last known consumed effective endpoint in its independent clock domain.
    last_monotonic: Option<MonotonicNs>,
}

impl FixtureRateSettlementCursor {
    /// Start one explicit local fixture sequence bound to checked target labels.
    ///
    /// Supplied labels/ordinal establish neither prior settlement nor continuity
    /// with another cursor, durable replay protection or installed clock state.
    #[must_use]
    pub const fn new(
        predecessor: &FixtureAffineChart,
        context: FixtureRateClockContext,
        next_ordinal: NonZeroU64,
    ) -> Self {
        let frames = predecessor.frames();
        Self {
            context,
            progress: CursorProgress::Pending(next_ordinal),
            target_frame: frames.target_frame(),
            target_generation: frames.target_generation(),
            last_active: None,
            last_monotonic: None,
        }
    }

    /// Return the closed local state without granting current event admission.
    #[must_use]
    pub const fn status(&self) -> FixtureRateCursorStatus {
        match self.progress {
            CursorProgress::Pending(_) => FixtureRateCursorStatus::Pending,
            CursorProgress::Unresolved => FixtureRateCursorStatus::Unresolved,
            CursorProgress::Exhausted => FixtureRateCursorStatus::Exhausted,
        }
    }

    /// Copy the sole admissible local identity, absent after either terminal state.
    #[must_use]
    pub const fn next_ordinal(&self) -> Option<NonZeroU64> {
        match self.progress {
            CursorProgress::Pending(value) => Some(value),
            _ => None,
        }
    }

    /// Return the last known active endpoint without inventing Unknown delivery.
    #[must_use]
    pub const fn last_active_end(&self) -> Option<ActiveAwakeNs> {
        self.last_active
    }

    /// Return the independently ordered last known effective endpoint.
    #[must_use]
    pub const fn last_monotonic_end(&self) -> Option<MonotonicNs> {
        self.last_monotonic
    }
}

impl fmt::Debug for FixtureRateSettlementCursor {
    /// Redact cursor identity, progress and endpoints with fixed output.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureRateSettlementCursor { .. }")
    }
}

/// Disposition of an admitted modeled event; not an authority or success token.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureRateSettlementKind {
    /// Completed known delivery, including an admitted arithmetic failure.
    Completed,
    /// Cancelled known delivered portion, including an arithmetic failure.
    Cancelled,
    /// Admitted explicit no-effect event; no replacement exists.
    ProvenNoEffect,
    /// Admitted unknown event; no inferred replacement exists.
    Unknown,
}

/// Opaque, non-Clone admitted result with separately retained arithmetic failure.
///
/// A caller cannot manufacture an admitted event or replacement absence:
/// ```compile_fail,E0451
/// use time_core::{FixtureRateSettlementCompletion, FixtureRateSettlementKind};
/// let _forged = FixtureRateSettlementCompletion {
///     kind: FixtureRateSettlementKind::Completed,
///     replacement: Ok(None),
/// };
/// ```
#[must_use]
pub struct FixtureRateSettlementCompletion {
    /// Actual closed event kind retained even when replacement arithmetic failed.
    kind: FixtureRateSettlementKind,
    /// None is only known no-effect/unknown absence; Err retains its exact cause.
    replacement: Result<Option<FixtureAffineChart>, FixtureChartError>,
}

impl FixtureRateSettlementCompletion {
    /// Copy the admitted kind without inferring numerical or native success.
    #[must_use]
    pub const fn kind(&self) -> FixtureRateSettlementKind {
        self.kind
    }

    /// Borrow the replacement, explicit absence or original bounded failure.
    /// # Errors
    /// Preserve post-admission integration/bound/composition failure; no retry.
    pub const fn replacement(&self) -> Result<Option<&FixtureAffineChart>, FixtureChartError> {
        match &self.replacement {
            Ok(Some(chart)) => Ok(Some(chart)),
            Ok(None) => Ok(None),
            Err(error) => Err(*error),
        }
    }

    /// Consume this result and transfer the same checked replacement or failure.
    /// # Errors
    /// Preserve the original admitted failure without restoring either owner.
    pub fn into_replacement(self) -> Result<Option<FixtureAffineChart>, FixtureChartError> {
        self.replacement
    }
}

impl fmt::Debug for FixtureRateSettlementCompletion {
    /// Redact all payloads and numeric results with bounded fixed output.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("FixtureRateSettlementCompletion { .. }")
    }
}

/// Actual current receiver/cursor/receipt relationship before mutation.
struct SettlementValue<'value, 'chart> {
    /// Exclusively borrowed at the API; admission itself is read-only.
    owner: &'value FixtureReferenceInputState<'chart>,
    /// Actual persistent cursor, not caller-supplied high-water snapshots.
    cursor: &'value FixtureRateSettlementCursor,
    /// Supplied exact chart instance, checked against the readiness owner.
    predecessor: &'value FixtureAffineChart,
    /// Immutable typed raw event; current eligibility still belongs to this rule.
    event: FixtureRateSettlement,
}

/// Owned admitted parts; no borrowed validation context escapes to commit.
struct AdmittedSettlement {
    /// Checked next progress after consumption, including ordinal exhaustion.
    next: CursorProgress,
    /// Actual immutable admitted event, never a requested amount.
    event: FixtureRateSettlement,
}

/// Check known mapping/endpoint/overlap relations without integrating delivery.
/// # Errors
/// Preserve context relationship, endpoint cycle, active overlap, then monotonic
/// overlap. All numerical integration and its support failures happen after commit.
fn known_relationship(
    predecessor: &FixtureAffineChart,
    cursor: &FixtureRateSettlementCursor,
    interval: FixtureSettledRateInterval,
) -> Result<(), FixtureRateDeliveryError> {
    reference_chart_mapping(predecessor, interval.frames, interval.domain)
        .map_err(|source| FixtureRateDeliveryError::Reference { source })?;
    let first = predecessor.frames();
    FixtureChartFrames::new(
        first.source_frame(),
        first.source_generation(),
        interval.frames.target_frame(),
        interval.frames.target_generation(),
    )
    .map_err(|source| FixtureRateDeliveryError::Chart { source })?;
    if let Some(last) = cursor.last_active
        && interval.span.active.0 < last
    {
        return Err(FixtureRateDeliveryError::ActiveOverlap);
    }
    if let Some(last) = cursor.last_monotonic
        && interval.span.effective.0 < last
    {
        return Err(FixtureRateDeliveryError::MonotonicOverlap);
    }
    Ok(())
}

/// Admit the whole live relationship and calculate checked nonwrapping progress.
/// # Errors
/// Readiness/identity precede cursor state, context, ordinal, cursor mapping and
/// known-event relationships. Rejection preserves both owners and every input.
fn settlement_parts(
    value: &SettlementValue<'_, '_>,
) -> Result<AdmittedSettlement, FixtureRateDeliveryError> {
    reference_chart_identity(value.owner, value.predecessor)
        .map_err(|source| FixtureRateDeliveryError::Reference { source })?;
    let CursorProgress::Pending(expected) = value.cursor.progress else {
        return Err(FixtureRateDeliveryError::CursorUnavailable {
            status: value.cursor.status(),
        });
    };
    let (context, ordinal) = value.event.identity();
    if context != value.cursor.context {
        return Err(FixtureRateDeliveryError::ClockContextMismatch);
    }
    if ordinal != expected {
        return Err(FixtureRateDeliveryError::OrdinalMismatch);
    }
    let frames = value.predecessor.frames();
    if frames.target_frame() != value.cursor.target_frame
        || frames.target_generation() != value.cursor.target_generation
    {
        return Err(FixtureRateDeliveryError::CursorMappingMismatch);
    }
    if let FixtureRateSettlement::Completed(interval) | FixtureRateSettlement::Cancelled(interval) =
        value.event
    {
        known_relationship(value.predecessor, value.cursor, interval)?;
    }
    let next = match ordinal.get().checked_add(1).and_then(NonZeroU64::new) {
        Some(ordinal) => CursorProgress::Pending(ordinal),
        None => CursorProgress::Exhausted,
    };
    Ok(AdmittedSettlement {
        next,
        event: value.event,
    })
}

/// Mandatory shared current-state rule inside the actual exclusive receiver.
#[derive(Validate)]
struct RawSettlement<'value, 'chart> {
    /// One rule owns the complete owner/cursor/event association before effects.
    #[garde(custom(|value: &SettlementValue<'_, '_>, (): &()| {
        settlement_parts(value).map(|_| ())
            .map_err(|_| garde::Error::new("invalid fixture rate settlement relationship"))
    }))]
    value: SettlementValue<'value, 'chart>,
}

impl FixtureReferenceInputState<'_> {
    /// Admit and account one settled modeled event, then attempt its replacement.
    ///
    /// Current Ready/exact identity and the actual cursor are mandatory. Known
    /// intervals consume/advance labels/high-water marks and invalidate BEFORE
    /// conversion, checked multiplication, outward bounds or composition. The
    /// replacement is a counterfactual same-domain translation, never a physical
    /// ramp, new observation or native acknowledgement. Unknown blocks the cursor;
    /// no-effect consumes only the ordinal. Work/storage are constant.
    /// # Errors
    /// Outer admission rejection changes neither owner. Later arithmetic failure
    /// is INSIDE the admitted completion and cannot refund consumption or Ready.
    ///
    /// A live admitted estimate excludes a simultaneous settlement:
    /// ```compile_fail,E0502
    /// use time_core::{FixtureAffineChart, FixtureChartCombinationOutcome,
    ///     FixtureRateSettlement, FixtureRateSettlementCursor,
    ///     FixtureReferenceInputState};
    /// fn overlap<'batch, 'entries, 'chart>(
    ///     owner: &mut FixtureReferenceInputState<'chart>,
    ///     outcome: &FixtureChartCombinationOutcome<'batch, 'entries, 'chart>,
    ///     cursor: &mut FixtureRateSettlementCursor,
    ///     predecessor: &FixtureAffineChart, event: FixtureRateSettlement,
    /// ) -> Result<(), Box<dyn std::error::Error>> {
    ///     let input = owner.admit_reference_input(outcome)?;
    ///     owner.receive_rate_settlement(cursor, predecessor, event)?;
    ///     let _phase = input.combined().phase_seconds();
    ///     Ok(())
    /// }
    /// ```
    pub fn receive_rate_settlement(
        &mut self,
        cursor: &mut FixtureRateSettlementCursor,
        predecessor: &FixtureAffineChart,
        event: FixtureRateSettlement,
    ) -> Result<FixtureRateSettlementCompletion, FixtureRateDeliveryError> {
        let admitted = {
            let raw = RawSettlement {
                value: SettlementValue {
                    owner: self,
                    cursor,
                    predecessor,
                    event,
                },
            };
            let validated = raw.validate();
            let admitted = settlement_parts(&raw.value)?;
            validated.map_err(|_| FixtureRateDeliveryError::ValidationInvariant)?;
            admitted
        };
        cursor.progress = admitted.next;
        let (kind, replacement) = match admitted.event {
            FixtureRateSettlement::ProvenNoEffect { .. } => {
                (FixtureRateSettlementKind::ProvenNoEffect, Ok(None))
            }
            FixtureRateSettlement::Unknown { .. } => {
                cursor.progress = CursorProgress::Unresolved;
                self.receive_feedback(FixtureCorrectionFeedback::Unknown);
                (FixtureRateSettlementKind::Unknown, Ok(None))
            }
            FixtureRateSettlement::Completed(interval)
            | FixtureRateSettlement::Cancelled(interval) => {
                cursor.last_active = Some(interval.span.active.1);
                cursor.last_monotonic = Some(interval.span.effective.1);
                cursor.target_frame = interval.frames.target_frame();
                cursor.target_generation = interval.frames.target_generation();
                self.receive_feedback(FixtureCorrectionFeedback::CoordinateChanged);
                let replacement = FixtureAffineChart::settled_rate_effect(
                    interval.frames,
                    interval.domain,
                    interval.rate,
                    interval.span.active_elapsed,
                )
                .and_then(|effect| predecessor.compose(&effect))
                .map(Some);
                let kind = if matches!(admitted.event, FixtureRateSettlement::Completed(_)) {
                    FixtureRateSettlementKind::Completed
                } else {
                    FixtureRateSettlementKind::Cancelled
                };
                (kind, replacement)
            }
        };
        Ok(FixtureRateSettlementCompletion { kind, replacement })
    }
}

// SPDX-License-Identifier: GPL-2.0-only
//! Exclusive fixture lifecycle owner and the sole owner of completed results.
//!
//! Completed-result fields and construction remain in this module. No external
//! consumer or sibling module can manufacture a control or time observation.
//! Current generations are owner-held facts, never response claims. All failures
//! preserve the slot and sequence. Successful completion is staged before the
//! exclusive owner consumes its pending slot; no packet or token enters a result.

use std::{fmt, num::NonZeroU64};

use ntp_wire::{FrameError, FrameLimits, RawHeader, frame_no_legacy_mac_datagram};
use time_domain::{
    DomainError, MonotonicNs, PhaseOffsetQ33, SignedRoundTripQ32, UtcCoordinateQ32,
    exchange_arithmetic, unfold_ntp,
};

use crate::{
    AssociationContext, Binding, ClockGeneration, Generations, PeerEraContext, ReceiptFacts,
    RequestId, RequestTiming, WireToken,
};

/// Fixed-size state admitted atomically by the sole association owner.
///
/// Every input is immutable and checked in its own domain. Context is captured
/// at arm; receipt admission rechecks it against the current owner. No packet,
/// allocation, native resource or cloneable pending owner is retained.
#[derive(Debug)]
struct PendingRequest {
    /// Context captured from the trusted fixture lifecycle owner at admission.
    context: AssociationContext,
    /// Once-only identity, distinct from the opaque wire correlation token.
    id: RequestId,
    /// Separately saved real send coordinate on the asserted fixture mapping.
    saved_real_t1: UtcCoordinateQ32,
    /// Checked nonzero correlation bits; Debug redacts their representation.
    token: WireToken,
    /// Checked positive bounded monotonic interval.
    timing: RequestTiming,
    /// Explicit peer coordinate windows, without anchor trust.
    peer_eras: PeerEraContext,
}

impl PendingRequest {
    /// Check captured generations against the current exclusive owner.
    ///
    /// # Errors
    /// Fixed ordered stale-generation failures; no state change or effects.
    fn check_current(&self, current: AssociationContext) -> Result<(), ReceiveError> {
        let captured = self.context.generations();
        let current = current.generations();
        if captured.configuration() != current.configuration() {
            return Err(ReceiveError::StaleConfiguration);
        }
        if captured.association() != current.association() {
            return Err(ReceiveError::StaleAssociation);
        }
        if captured.clock() != current.clock() {
            return Err(ReceiveError::StaleClock);
        }
        Ok(())
    }

    /// Admit binding, current context, nonzero origin and monotonic ordering.
    ///
    /// The caller already framed the complete header and checked v4/server mode.
    /// # Errors
    /// Returns the first fixed rejection, preserving all request/owner state.
    fn check_receipt(
        &self,
        current: AssociationContext,
        header: &RawHeader,
        receipt: ReceiptFacts,
    ) -> Result<(), ReceiveError> {
        let binding = self.context.binding();
        if receipt.binding().source_id() != binding.source_id() {
            return Err(ReceiveError::SourceMismatch);
        }
        if receipt.binding().socket_id() != binding.socket_id() {
            return Err(ReceiveError::SocketMismatch);
        }
        if receipt.binding().endpoint() != binding.endpoint() {
            return Err(ReceiveError::EndpointMismatch);
        }
        self.check_current(current)?;
        if receipt.mapping_generation() != current.generations().clock() {
            return Err(ReceiveError::MappingGenerationMismatch);
        }
        let origin = WireToken::new(header.origin_timestamp().bits())
            .map_err(|_| ReceiveError::ZeroOrigin)?;
        if origin != self.token {
            return Err(ReceiveError::OriginMismatch);
        }
        if receipt.arrival() < self.timing.send() {
            return Err(ReceiveError::ArrivalBeforeSend);
        }
        if receipt.processing_now() < receipt.arrival() {
            return Err(ReceiveError::ProcessingBeforeArrival);
        }
        if receipt.processing_now() >= self.timing.deadline() {
            return Err(ReceiveError::DeadlineReached);
        }
        Ok(())
    }

    /// Stage one complete result only after this method's admission succeeds.
    ///
    /// Complete framing and v4/server checks are the caller's preconditions.
    /// Work and retained output have fixed bounds; the pending owner is unchanged.
    /// # Errors
    /// Propagates ordered receipt and time-only failures before any completion.
    fn stage_event(
        &self,
        current: AssociationContext,
        header: &RawHeader,
        receipt: ReceiptFacts,
    ) -> Result<FixtureEvent, ReceiveError> {
        self.check_receipt(current, header, receipt)?;
        let facts = CompletionFacts {
            binding: self.context.binding(),
            request_id: self.id,
            generations: self.context.generations(),
            mapping_generation: receipt.mapping_generation(),
            send: self.timing.send(),
            arrival: receipt.arrival(),
            processing_now: receipt.processing_now(),
            deadline: self.timing.deadline(),
        };
        if header.stratum() == 0 {
            return Ok(FixtureEvent::Control(OpaqueControl {
                facts,
                code_bits: header.reference_id(),
            }));
        }
        if header.leap_indicator() == 3 {
            return Err(ReceiveError::UnsynchronizedLeap);
        }
        if header.stratum() > 15 {
            return Err(ReceiveError::InvalidStratum);
        }
        let receive = header.receive_timestamp();
        let transmit = header.transmit_timestamp();
        if receive.bits() == 0 {
            return Err(ReceiveError::ZeroReceiveTimestamp);
        }
        if transmit.bits() == 0 {
            return Err(ReceiveError::ZeroTransmitTimestamp);
        }
        let peer_t2 = unfold_ntp(
            receive.seconds(),
            receive.fraction(),
            &self.peer_eras.receive_window(),
        )
        .map_err(ReceiveError::ReceiveEra)?;
        let peer_t3 = unfold_ntp(
            transmit.seconds(),
            transmit.fraction(),
            &self.peer_eras.transmit_window(),
        )
        .map_err(ReceiveError::TransmitEra)?;
        let arithmetic =
            exchange_arithmetic(self.saved_real_t1, peer_t2, peer_t3, receipt.local_t4())
                .map_err(ReceiveError::Arithmetic)?;
        Ok(FixtureEvent::Time(UnauthenticatedFixtureObservation {
            facts,
            saved_real_t1: self.saved_real_t1,
            peer_t2,
            peer_t3,
            local_t4: receipt.local_t4(),
            offset: arithmetic.offset(),
            signed_round_trip: arithmetic.round_trip(),
            leap_indicator: header.leap_indicator(),
            stratum: header.stratum(),
            poll: header.poll(),
            precision: header.precision(),
            root_delay_bits: header.root_delay_bits(),
            root_dispersion_bits: header.root_dispersion_bits(),
            reference_id: header.reference_id(),
            reference_timestamp_bits: header.reference_timestamp().bits(),
        }))
    }
}

/// Exclusive owner for the named unauthenticated plain-header fixture profile.
///
/// No clone/default route may duplicate its once-only state:
/// ```compile_fail,E0599
/// use ntp_exchange::Association;
/// fn duplicate(owner: Association) {
///     let _ = owner.clone();
/// }
/// ```
/// ```compile_fail,E0277
/// use ntp_exchange::Association;
/// let _: Association = Default::default();
/// ```
/// Legal construction requires the complete explicit context and sequence:
/// ```
/// use std::{net::SocketAddr, num::NonZeroU64};
/// use ntp_exchange::{Association, AssociationContext, AssociationGeneration,
///     Binding, ClockGeneration, ConfigurationGeneration, Generations,
///     RequestId, SocketId, SourceId};
/// let source = SourceId::new(NonZeroU64::MIN);
/// let socket = SocketId::new(NonZeroU64::MIN);
/// let binding = Binding::new(source, socket, SocketAddr::from(([192, 0, 2, 1], 123)));
/// let generations = Generations::new(ConfigurationGeneration::new(1),
///     AssociationGeneration::new(1), ClockGeneration::new(1));
/// let context = AssociationContext::new(binding, generations);
/// let first = RequestId::new(NonZeroU64::MIN);
/// let owner = Association::new_plain_header_fixture(context, first);
/// assert_eq!(owner.current_context(), context);
/// assert_eq!(owner.next_request_id(), Some(first));
/// assert_eq!(owner.pending_request_id(), None);
/// ```
#[derive(Debug)]
pub struct Association {
    /// Current immutable binding/generations owned exclusively by this instance.
    context: AssociationContext,
    /// Next sequence value; exhaustion is explicit and never replenished.
    next_request: Option<RequestId>,
    /// At most one fully initialized request, with no native resource custody.
    pending: Option<PendingRequest>,
}

impl Association {
    /// Create an empty explicit 48-byte fixture owner with the supplied sequence.
    ///
    /// There is no production default or token issuer. The maximum request ID is
    /// usable once, then exhaustion survives completion and lifecycle changes.
    #[must_use]
    pub const fn new_plain_header_fixture(
        context: AssociationContext,
        initial_request: RequestId,
    ) -> Self {
        Self {
            context,
            next_request: Some(initial_request),
            pending: None,
        }
    }

    /// Atomically admit one request with separately saved real T1.
    ///
    /// Inputs are immutable checked values on the driver's asserted mapping.
    /// Stage the complete request and successor before changing either field.
    /// Token entropy/freshness and native send provenance remain unqualified.
    /// # Errors
    /// Busy precedes exhaustion; every rejection preserves the entire owner.
    ///
    /// A raw integer cannot bypass token construction:
    /// ```compile_fail,E0308
    /// use ntp_exchange::{Association, PeerEraContext, RequestTiming};
    /// use time_domain::UtcCoordinateQ32;
    /// fn bypass(owner: &mut Association, t1: UtcCoordinateQ32,
    ///     timing: RequestTiming, eras: PeerEraContext) {
    ///     let _ = owner.arm(t1, 1_u64, timing, eras);
    /// }
    /// ```
    pub fn arm(
        &mut self,
        saved_real_t1: UtcCoordinateQ32,
        token: WireToken,
        timing: RequestTiming,
        peer_eras: PeerEraContext,
    ) -> Result<RequestId, ArmError> {
        if self.pending.is_some() {
            return Err(ArmError::Busy);
        }
        let id = self.next_request.ok_or(ArmError::SequenceExhausted)?;
        let next = id
            .value()
            .get()
            .checked_add(1)
            .and_then(NonZeroU64::new)
            .map(RequestId::new);
        let pending = PendingRequest {
            context: self.context,
            id,
            saved_real_t1,
            token,
            timing,
            peer_eras,
        };
        self.pending = Some(pending);
        self.next_request = next;
        Ok(id)
    }

    /// Admit complete packet bytes and consume a current request exactly once.
    ///
    /// Ordered checks: complete 48-byte framing, v4/server, pending state,
    /// binding/current generations/mapping, nonzero origin/token and monotonic
    /// ordering; then control classification or exact time-only processing.
    /// Stage the entire result before the final identity/context guard and slot
    /// clear. Exclusive borrowing admits no callback, await or concurrent update.
    /// # Errors
    /// Fixed redacted rejection preserves pending data, context and sequence.
    pub fn receive(
        &mut self,
        packet: &[u8],
        receipt: ReceiptFacts,
    ) -> Result<FixtureEvent, ReceiveError> {
        let limits = FrameLimits::new(48, 0).map_err(ReceiveError::Framing)?;
        let frame = frame_no_legacy_mac_datagram(packet, limits).map_err(ReceiveError::Framing)?;
        let header = frame.header();
        if header.version() != 4 {
            return Err(ReceiveError::UnsupportedVersion);
        }
        if header.mode() != 4 {
            return Err(ReceiveError::UnsupportedMode);
        }
        let pending = self.pending.as_ref().ok_or(ReceiveError::NoPending)?;
        let expected_id = pending.id;
        let event = pending.stage_event(self.context, header, receipt)?;
        let current = self.pending.as_ref().ok_or(ReceiveError::NoPending)?;
        if current.id != expected_id {
            return Err(ReceiveError::NoPending);
        }
        current.check_current(self.context)?;
        self.pending = None;
        Ok(event)
    }

    /// Retire precisely the request named by a trusted fixture lifecycle call.
    /// # Errors
    /// No pending request or a stale identity leaves all state unchanged.
    pub fn cancel(&mut self, expected: RequestId) -> Result<RequestId, CancelError> {
        let pending = self.pending.as_ref().ok_or(CancelError::NoPending)?;
        if pending.id != expected {
            return Err(CancelError::RequestMismatch);
        }
        let retired = pending.id;
        self.pending = None;
        Ok(retired)
    }

    /// Retire a pending request only at or beyond its monotonic deadline.
    ///
    /// Packet rejection never invokes this trusted lifecycle operation.
    /// # Errors
    /// Missing request or an early reading preserves the entire owner.
    pub fn expire(&mut self, now: MonotonicNs) -> Result<RequestId, ExpireError> {
        let pending = self.pending.as_ref().ok_or(ExpireError::NoPending)?;
        if now < pending.timing.deadline() {
            return Err(ExpireError::NotExpired);
        }
        let retired = pending.id;
        self.pending = None;
        Ok(retired)
    }

    /// Atomically update context and retire old pending work, if present.
    ///
    /// Generations may jump forward but never regress; at least one advances.
    /// A changed binding requires association-generation advance. Sequence state
    /// survives all accepted updates, including exhaustion.
    /// # Errors
    /// Check configuration/association/clock regression, no advance, then binding
    /// change. Every failed candidate preserves context, pending and sequence.
    pub fn advance_context(
        &mut self,
        next: AssociationContext,
    ) -> Result<Option<RequestId>, ContextError> {
        let old = self.context.generations();
        let generations = next.generations();
        if generations.configuration() < old.configuration() {
            return Err(ContextError::ConfigurationRegression);
        }
        if generations.association() < old.association() {
            return Err(ContextError::AssociationRegression);
        }
        if generations.clock() < old.clock() {
            return Err(ContextError::ClockRegression);
        }
        if generations == old {
            return Err(ContextError::NoGenerationAdvance);
        }
        if next.binding() != self.context.binding()
            && generations.association() == old.association()
        {
            return Err(ContextError::BindingChangedWithoutAssociationAdvance);
        }
        let retired = self.pending.as_ref().map(|pending| pending.id);
        self.pending = None;
        self.context = next;
        Ok(retired)
    }

    /// Return the immutable current context without granting native authority.
    #[must_use]
    pub const fn current_context(&self) -> AssociationContext {
        self.context
    }

    /// Return the pending identity without exposing the token or saved request.
    #[must_use]
    pub const fn pending_request_id(&self) -> Option<RequestId> {
        match &self.pending {
            Some(pending) => Some(pending.id),
            None => None,
        }
    }

    /// Return the next sequence identity without changing or replenishing it.
    #[must_use]
    pub const fn next_request_id(&self) -> Option<RequestId> {
        self.next_request
    }
}

/// Bounded once-only result, constructible only from private completed payloads.
#[derive(Debug, Eq, PartialEq)]
pub enum FixtureEvent {
    /// An unauthenticated exact observation, with no statistical admission.
    Time(UnauthenticatedFixtureObservation),
    /// An opaque stratum-zero control, with no scheduler or time effect.
    Control(OpaqueControl),
}

/// Exact bounded context and monotonic facts retained after a completed request.
///
/// Private fields and no factory prevent consumer-manufactured completion:
/// ```compile_fail,E0277
/// use ntp_exchange::CompletionFacts;
/// let _: CompletionFacts = Default::default();
/// ```
#[derive(Debug, Eq, PartialEq)]
pub struct CompletionFacts {
    /// Committed source/socket/endpoint binding of the completed request.
    binding: Binding,
    /// Committed request identity, without wire-freshness guarantees.
    request_id: RequestId,
    /// Captured and current generation tuple checked before completion.
    generations: Generations,
    /// Receipt mapping generation checked against the current clock generation.
    mapping_generation: ClockGeneration,
    /// Saved exact monotonic send reading.
    send: MonotonicNs,
    /// Checked same-domain monotonic arrival reading.
    arrival: MonotonicNs,
    /// Checked same-domain monotonic processing reading before the deadline.
    processing_now: MonotonicNs,
    /// Saved exclusive response-processing deadline.
    deadline: MonotonicNs,
}

impl CompletionFacts {
    /// Return the exact committed binding without native-identity claims.
    #[must_use]
    pub const fn binding(&self) -> Binding {
        self.binding
    }

    /// Return the committed request identity without token disclosure.
    #[must_use]
    pub const fn request_id(&self) -> RequestId {
        self.request_id
    }

    /// Return the committed configuration, association and clock tuple.
    #[must_use]
    pub const fn generations(&self) -> Generations {
        self.generations
    }

    /// Return the independently supplied and checked receipt mapping generation.
    #[must_use]
    pub const fn mapping_generation(&self) -> ClockGeneration {
        self.mapping_generation
    }

    /// Return the saved exact monotonic send reading.
    #[must_use]
    pub const fn send(&self) -> MonotonicNs {
        self.send
    }

    /// Return the checked monotonic arrival reading.
    #[must_use]
    pub const fn arrival(&self) -> MonotonicNs {
        self.arrival
    }

    /// Return the checked monotonic processing reading.
    #[must_use]
    pub const fn processing_now(&self) -> MonotonicNs {
        self.processing_now
    }

    /// Return the exact exclusive processing deadline.
    #[must_use]
    pub const fn deadline(&self) -> MonotonicNs {
        self.deadline
    }
}

/// Exact unauthenticated fixture facts, never an eligible sample or source vote.
///
/// Raw quality claims remain unqualified; the future statistical owner must admit
/// their meaning and complete current policy independently. No packet, header,
/// token or rounded nanosecond result is retained.
///
/// ```compile_fail,E0616
/// use ntp_exchange::UnauthenticatedFixtureObservation;
/// fn change(observation: &mut UnauthenticatedFixtureObservation) {
///     observation.stratum = 1;
/// }
/// ```
#[derive(Debug, Eq, PartialEq)]
pub struct UnauthenticatedFixtureObservation {
    /// Bounded facts constructed only after current-request admission.
    facts: CompletionFacts,
    /// Separately saved real local send coordinate, never replaced by the token.
    saved_real_t1: UtcCoordinateQ32,
    /// Peer receive coordinate unfolded through its explicit fixture window.
    peer_t2: UtcCoordinateQ32,
    /// Peer transmit coordinate unfolded through its explicit fixture window.
    peer_t3: UtcCoordinateQ32,
    /// Local receipt coordinate on the asserted common mapping.
    local_t4: UtcCoordinateQ32,
    /// Exact signed numerator over 2^33 seconds; positive means local is behind.
    offset: PhaseOffsetQ33,
    /// Exact signed raw RTT numerator over 2^32 seconds, including negatives.
    signed_round_trip: SignedRoundTripQ32,
    /// Admitted raw time-candidate LI bits, without leap-policy interpretation.
    leap_indicator: u8,
    /// Admitted raw time-candidate stratum, without sample eligibility.
    stratum: u8,
    /// Exact signed raw poll exponent, without scheduling policy.
    poll: i8,
    /// Exact signed raw precision exponent, without uncertainty policy.
    precision: i8,
    /// Exact signed-format wire root-delay bits, with no quality threshold.
    root_delay_bits: u32,
    /// Exact unsigned-format wire root-dispersion bits, with no quality threshold.
    root_dispersion_bits: u32,
    /// Exact opaque raw reference identifier, not an authenticated identity.
    reference_id: u32,
    /// Exact raw reference timestamp, with no era or age inference.
    reference_timestamp_bits: u64,
}

impl UnauthenticatedFixtureObservation {
    /// Borrow immutable completion facts without creating another event.
    #[must_use]
    pub const fn facts(&self) -> &CompletionFacts {
        &self.facts
    }

    /// Return the separately saved real T1 in exact Q32 coordinate units.
    #[must_use]
    pub const fn saved_real_t1(&self) -> UtcCoordinateQ32 {
        self.saved_real_t1
    }

    /// Return the explicitly unfolded T2 in exact Q32 coordinate units.
    #[must_use]
    pub const fn peer_t2(&self) -> UtcCoordinateQ32 {
        self.peer_t2
    }

    /// Return the explicitly unfolded T3 in exact Q32 coordinate units.
    #[must_use]
    pub const fn peer_t3(&self) -> UtcCoordinateQ32 {
        self.peer_t3
    }

    /// Return the supplied local T4 in exact Q32 coordinate units.
    #[must_use]
    pub const fn local_t4(&self) -> UtcCoordinateQ32 {
        self.local_t4
    }

    /// Return the exact signed Q33 phase without rounding or eligibility claims.
    #[must_use]
    pub const fn offset(&self) -> PhaseOffsetQ33 {
        self.offset
    }

    /// Return exact signed Q32 RTT without clamping negative values.
    #[must_use]
    pub const fn signed_round_trip(&self) -> SignedRoundTripQ32 {
        self.signed_round_trip
    }

    /// Return the exact raw LI claim without interpreting leap policy.
    #[must_use]
    pub const fn leap_indicator(&self) -> u8 {
        self.leap_indicator
    }

    /// Return the exact raw stratum claim without sample eligibility.
    #[must_use]
    pub const fn stratum(&self) -> u8 {
        self.stratum
    }

    /// Return the exact signed poll exponent without a scheduling action.
    #[must_use]
    pub const fn poll(&self) -> i8 {
        self.poll
    }

    /// Return the exact signed precision exponent without uncertainty inference.
    #[must_use]
    pub const fn precision(&self) -> i8 {
        self.precision
    }

    /// Return all root-delay wire bits without conversion or quality policy.
    #[must_use]
    pub const fn root_delay_bits(&self) -> u32 {
        self.root_delay_bits
    }

    /// Return all root-dispersion wire bits without conversion or quality policy.
    #[must_use]
    pub const fn root_dispersion_bits(&self) -> u32 {
        self.root_dispersion_bits
    }

    /// Return the exact opaque reference identifier.
    #[must_use]
    pub const fn reference_id(&self) -> u32 {
        self.reference_id
    }

    /// Return reference timestamp bits without era or reference-age inference.
    #[must_use]
    pub const fn reference_timestamp_bits(&self) -> u64 {
        self.reference_timestamp_bits
    }
}

/// Terminal opaque stratum-zero control, with no arithmetic or time-quality facts.
///
/// Every LI value and arbitrary peer timestamp bits have the same control meaning
/// after complete current correlation. This type supplies no poll/rearm action.
///
/// ```compile_fail,E0451
/// use ntp_exchange::{CompletionFacts, OpaqueControl};
/// fn fabricate(facts: CompletionFacts) -> OpaqueControl {
///     OpaqueControl { facts, code_bits: 0 }
/// }
/// ```
#[derive(Debug, Eq, PartialEq)]
pub struct OpaqueControl {
    /// Only bounded completion identity/generation/monotonic facts are retained.
    facts: CompletionFacts,
    /// Opaque reference-ID bits, including unknown control codes.
    code_bits: u32,
}

impl OpaqueControl {
    /// Borrow immutable completion facts without creating another event.
    #[must_use]
    pub const fn facts(&self) -> &CompletionFacts {
        &self.facts
    }

    /// Return the opaque code without backoff, blacklist or scheduler meaning.
    #[must_use]
    pub const fn code_bits(&self) -> u32 {
        self.code_bits
    }
}

/// Fixed request-admission failures, with no retained token or caller payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArmError {
    /// An existing pending request already owns the only slot.
    Busy,
    /// No unused nonzero sequence identity remains.
    SequenceExhausted,
}

impl fmt::Display for ArmError {
    /// Write a fixed bounded message without input values.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Busy => "request slot busy",
            Self::SequenceExhausted => "request sequence exhausted",
        })
    }
}

impl std::error::Error for ArmError {}

/// Fixed trusted-cancellation failures; rejection preserves a successor request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CancelError {
    /// No pending request exists to retire.
    NoPending,
    /// The supplied expected ID differs from the actual pending identity.
    RequestMismatch,
}

impl fmt::Display for CancelError {
    /// Write a fixed bounded message without request values.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::NoPending => "no pending request",
            Self::RequestMismatch => "request identity mismatch",
        })
    }
}

impl std::error::Error for CancelError {}

/// Fixed explicit-expiry failures; packet rejection never implicitly retires.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExpireError {
    /// No pending request exists to expire.
    NoPending,
    /// The supplied same-domain monotonic instant precedes the deadline.
    NotExpired,
}

impl fmt::Display for ExpireError {
    /// Write a fixed bounded message without timing values.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::NoPending => "no pending request",
            Self::NotExpired => "request not expired",
        })
    }
}

impl std::error::Error for ExpireError {}

/// Fixed atomic lifecycle-update failures; no rejected context is retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContextError {
    /// A supplied configuration generation moves backward.
    ConfigurationRegression,
    /// A supplied association generation moves backward.
    AssociationRegression,
    /// A supplied clock generation moves backward.
    ClockRegression,
    /// No component of the generation tuple advances.
    NoGenerationAdvance,
    /// Binding changed without a strictly newer association generation.
    BindingChangedWithoutAssociationAdvance,
}

impl fmt::Display for ContextError {
    /// Write a fixed bounded message without identity or generation values.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::ConfigurationRegression => "configuration generation regressed",
            Self::AssociationRegression => "association generation regressed",
            Self::ClockRegression => "clock generation regressed",
            Self::NoGenerationAdvance => "no generation advanced",
            Self::BindingChangedWithoutAssociationAdvance => {
                "binding changed without association advance"
            }
        })
    }
}

impl std::error::Error for ContextError {}

/// Fixed bounded receive failures; no variant retains packet or token values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReceiveError {
    /// Complete framing rejected the packet under the named 48-byte profile.
    Framing(FrameError),
    /// The fixed header is not NTP version 4.
    UnsupportedVersion,
    /// The fixed header is not server mode 4.
    UnsupportedMode,
    /// The owner has no pending request to complete.
    NoPending,
    /// The actual fixture receipt source differs from the pending binding.
    SourceMismatch,
    /// The actual fixture receipt socket differs from the pending binding.
    SocketMismatch,
    /// The exact fixture receipt endpoint differs from the pending binding.
    EndpointMismatch,
    /// Captured configuration no longer matches the owner's current context.
    StaleConfiguration,
    /// Captured association no longer matches the owner's current context.
    StaleAssociation,
    /// Captured clock generation no longer matches the owner's current context.
    StaleClock,
    /// The receipt T4 mapping is not the owner's current clock generation.
    MappingGenerationMismatch,
    /// Raw origin zero cannot construct a checked correlation token.
    ZeroOrigin,
    /// A checked nonzero origin differs from the pending request token.
    OriginMismatch,
    /// The receipt arrived before the saved send instant.
    ArrivalBeforeSend,
    /// Processing is asserted before arrival in the same monotonic domain.
    ProcessingBeforeArrival,
    /// Processing occurs at or beyond the exclusive request deadline.
    DeadlineReached,
    /// A time candidate reports LI=3; controls do not use this time-only rule.
    UnsynchronizedLeap,
    /// A noncontrol candidate reports stratum outside 1 through 15.
    InvalidStratum,
    /// A time candidate has zero receive timestamp bits.
    ZeroReceiveTimestamp,
    /// A time candidate has zero transmit timestamp bits.
    ZeroTransmitTimestamp,
    /// Explicit receive-window unfolding failed before completion.
    ReceiveEra(DomainError),
    /// Explicit transmit-window unfolding failed before completion.
    TransmitEra(DomainError),
    /// Exact checked arithmetic failed before completion.
    Arithmetic(DomainError),
}

impl fmt::Display for ReceiveError {
    /// Write a fixed bounded rejection message without packet or token content.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Framing(_) => "packet framing rejected",
            Self::UnsupportedVersion => "unsupported packet version",
            Self::UnsupportedMode => "unsupported packet mode",
            Self::NoPending => "no pending request",
            Self::SourceMismatch => "receipt source mismatch",
            Self::SocketMismatch => "receipt socket mismatch",
            Self::EndpointMismatch => "receipt endpoint mismatch",
            Self::StaleConfiguration => "stale configuration generation",
            Self::StaleAssociation => "stale association generation",
            Self::StaleClock => "stale clock generation",
            Self::MappingGenerationMismatch => "receipt mapping generation mismatch",
            Self::ZeroOrigin => "zero response origin",
            Self::OriginMismatch => "response origin mismatch",
            Self::ArrivalBeforeSend => "arrival before send",
            Self::ProcessingBeforeArrival => "processing before arrival",
            Self::DeadlineReached => "request deadline reached",
            Self::UnsynchronizedLeap => "unsynchronized time candidate",
            Self::InvalidStratum => "invalid time-candidate stratum",
            Self::ZeroReceiveTimestamp => "zero receive timestamp",
            Self::ZeroTransmitTimestamp => "zero transmit timestamp",
            Self::ReceiveEra(_) => "receive era rejected",
            Self::TransmitEra(_) => "transmit era rejected",
            Self::Arithmetic(_) => "exchange arithmetic rejected",
        })
    }
}

impl std::error::Error for ReceiveError {}

#[cfg(test)]
/// Direct helper-domain checks using only valid immutable fixture values.
mod tests {
    use std::{error::Error, net::SocketAddr, num::NonZeroU64};

    use time_domain::{EraWindow, MonotonicNs, UtcCoordinateQ32};

    use super::{PendingRequest, ReceiveError};
    use crate::{
        AssociationContext, AssociationGeneration, Binding, ClockGeneration,
        ConfigurationGeneration, Generations, PeerEraContext, RequestId, RequestTiming, SocketId,
        SourceId, WireToken,
    };

    /// Check every stale-generation branch and its documented rejection order.
    ///
    /// The helper accepts any typed current context. Public lifecycle retirement
    /// does not narrow that domain to matching generations. Every pending field
    /// below uses its legitimate checked representation; no invalid state or
    /// consumer construction escape is introduced.
    ///
    /// # Errors
    /// Propagates unexpected rejection of the fixed valid fixture values.
    ///
    /// # Panics
    /// Fails if a mismatched generation passes, precedence changes or the healthy
    /// captured context stops matching after a rejected helper call.
    #[test]
    fn pending_current_checks_each_generation_and_precedence() -> Result<(), Box<dyn Error>> {
        let binding = Binding::new(
            SourceId::new(NonZeroU64::MIN),
            SocketId::new(NonZeroU64::MIN),
            SocketAddr::from(([192, 0, 2, 1], 123)),
        );
        let context = AssociationContext::new(
            binding,
            Generations::new(
                ConfigurationGeneration::new(7),
                AssociationGeneration::new(11),
                ClockGeneration::new(13),
            ),
        );
        let pending = PendingRequest {
            context,
            id: RequestId::new(NonZeroU64::MIN),
            saved_real_t1: UtcCoordinateQ32::new(31)?,
            token: WireToken::new(37)?,
            timing: RequestTiming::new(MonotonicNs::new(41), MonotonicNs::new(43))?,
            peer_eras: PeerEraContext::new(EraWindow::new(47, 53)?, EraWindow::new(59, 61)?),
        };
        assert_eq!(pending.check_current(context), Ok(()));

        for ((configuration, association, clock), expected) in [
            ((6, 11, 13), ReceiveError::StaleConfiguration),
            ((8, 11, 13), ReceiveError::StaleConfiguration),
            ((7, 10, 13), ReceiveError::StaleAssociation),
            ((7, 12, 13), ReceiveError::StaleAssociation),
            ((7, 11, 12), ReceiveError::StaleClock),
            ((7, 11, 14), ReceiveError::StaleClock),
            ((8, 12, 14), ReceiveError::StaleConfiguration),
            ((7, 12, 14), ReceiveError::StaleAssociation),
        ] {
            let current = AssociationContext::new(
                binding,
                Generations::new(
                    ConfigurationGeneration::new(configuration),
                    AssociationGeneration::new(association),
                    ClockGeneration::new(clock),
                ),
            );
            assert_eq!(pending.check_current(current), Err(expected));
            assert_eq!(pending.context, context);
            assert_eq!(pending.check_current(context), Ok(()));
        }
        Ok(())
    }
}

// SPDX-License-Identifier: GPL-2.0-only
//! Immutable checked fixture inputs, distinct identity roles and timing bounds.
//!
//! Scalar representations and construction belong here; changing currentness
//! belongs to the association. Supplied IDs, addresses, coordinates and clock
//! generations are assertions by the fixture driver, not native provenance.

use std::{fmt, net::SocketAddr, num::NonZeroU64};

use time_domain::{EraWindow, MonotonicNs, UtcCoordinateQ32};

/// Experimental request-lifetime ceiling, not a production response timeout.
pub const FIXTURE_MAX_LIFETIME_NS: u64 = 1_000_000_000;

/// Nonzero fixture source identity; does not establish source approval.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourceId {
    /// Representation checked by `NonZeroU64`, with no cross-process uniqueness claim.
    value: NonZeroU64,
}

impl SourceId {
    /// Retain an already checked nonzero identity without effects.
    #[must_use]
    pub const fn new(value: NonZeroU64) -> Self {
        Self { value }
    }

    /// Return the immutable fixture identity without asserting authority.
    #[must_use]
    pub const fn value(self) -> NonZeroU64 {
        self.value
    }
}

/// Nonzero fixture socket identity; does not represent a native socket handle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SocketId {
    /// Representation checked by `NonZeroU64`, separate from source identity.
    value: NonZeroU64,
}

impl SocketId {
    /// Retain an already checked fixture socket identity without effects.
    #[must_use]
    pub const fn new(value: NonZeroU64) -> Self {
        Self { value }
    }

    /// Return the immutable fixture socket identity without native claims.
    #[must_use]
    pub const fn value(self) -> NonZeroU64 {
        self.value
    }
}

/// Nonzero sequence identity belonging to one association owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RequestId {
    /// Checked nonzero sequence value; advancement must never wrap or reset.
    value: NonZeroU64,
}

impl RequestId {
    /// Retain a checked sequence value, including `u64::MAX`, without effects.
    #[must_use]
    pub const fn new(value: NonZeroU64) -> Self {
        Self { value }
    }

    /// Return the immutable sequence identity without any freshness claim.
    #[must_use]
    pub const fn value(self) -> NonZeroU64 {
        self.value
    }
}

/// Fixture configuration generation; every `u64` representation is admitted.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ConfigurationGeneration {
    /// Immutable counter whose changes must be checked by the current owner.
    value: u64,
}

impl ConfigurationGeneration {
    /// Tag a numeric generation without certifying currentness or authority.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self { value }
    }

    /// Return the exact generation without effects.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.value
    }

    /// Produce a successor without changing this immutable generation.
    ///
    /// # Errors
    /// Returns exhaustion at `u64::MAX`; never wraps or resets.
    pub fn checked_next(self) -> Result<Self, GenerationError> {
        self.value
            .checked_add(1)
            .map(Self::new)
            .ok_or(GenerationError::GenerationExhausted)
    }
}

/// Fixture association generation, distinct from configuration and clock roles.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AssociationGeneration {
    /// Immutable counter whose contextual update belongs to the association.
    value: u64,
}

impl AssociationGeneration {
    /// Tag a numeric generation without certifying currentness or authority.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self { value }
    }

    /// Return the exact generation without effects.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.value
    }

    /// Produce a checked successor without changing this generation.
    ///
    /// # Errors
    /// Returns exhaustion at `u64::MAX`; never wraps or resets.
    pub fn checked_next(self) -> Result<Self, GenerationError> {
        self.value
            .checked_add(1)
            .map(Self::new)
            .ok_or(GenerationError::GenerationExhausted)
    }
}

/// Fixture clock-mapping generation, with no assertion of native continuity.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ClockGeneration {
    /// Immutable mapping counter, separate from configuration and association.
    value: u64,
}

impl ClockGeneration {
    /// Tag a numeric mapping generation without certifying its provenance.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self { value }
    }

    /// Return the exact mapping generation without effects.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.value
    }

    /// Produce a checked successor without mutating this generation.
    ///
    /// # Errors
    /// Returns exhaustion at `u64::MAX`; never wraps or resets.
    pub fn checked_next(self) -> Result<Self, GenerationError> {
        self.value
            .checked_add(1)
            .map(Self::new)
            .ok_or(GenerationError::GenerationExhausted)
    }
}

/// Fixed generation failure containing no rejected value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GenerationError {
    /// A successor cannot be represented without wrapping.
    GenerationExhausted,
}

impl fmt::Display for GenerationError {
    /// Write a fixed bounded message without input or side effects.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("generation exhausted")
    }
}

impl std::error::Error for GenerationError {}

/// Immutable generation tuple; currentness is checked by the association owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Generations {
    /// Configuration revision role, never inferred from response bytes.
    configuration: ConfigurationGeneration,
    /// Association lifecycle role, never inferred from response bytes.
    association: AssociationGeneration,
    /// Local clock-mapping role supplied by the fixture lifecycle driver.
    clock: ClockGeneration,
}

impl Generations {
    /// Join already typed generations without granting authority or currentness.
    #[must_use]
    pub const fn new(
        configuration: ConfigurationGeneration,
        association: AssociationGeneration,
        clock: ClockGeneration,
    ) -> Self {
        Self {
            configuration,
            association,
            clock,
        }
    }

    /// Return the immutable configuration generation.
    #[must_use]
    pub const fn configuration(self) -> ConfigurationGeneration {
        self.configuration
    }

    /// Return the immutable association generation.
    #[must_use]
    pub const fn association(self) -> AssociationGeneration {
        self.association
    }

    /// Return the immutable clock generation.
    #[must_use]
    pub const fn clock(self) -> ClockGeneration {
        self.clock
    }
}

/// Exact fixture source, socket and endpoint with distinct identity roles.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Binding {
    /// Asserted logical source identity.
    source_id: SourceId,
    /// Asserted receiving socket identity.
    socket_id: SocketId,
    /// Exact address and port; no resolution or destination-policy claim.
    endpoint: SocketAddr,
}

impl Binding {
    /// Join checked identities and an exact address without lookup or effects.
    #[must_use]
    pub const fn new(source_id: SourceId, socket_id: SocketId, endpoint: SocketAddr) -> Self {
        Self {
            source_id,
            socket_id,
            endpoint,
        }
    }

    /// Return the immutable fixture source role.
    #[must_use]
    pub const fn source_id(self) -> SourceId {
        self.source_id
    }

    /// Return the immutable fixture socket role.
    #[must_use]
    pub const fn socket_id(self) -> SocketId {
        self.socket_id
    }

    /// Return the exact endpoint without normalization or DNS work.
    #[must_use]
    pub const fn endpoint(self) -> SocketAddr {
        self.endpoint
    }
}

/// Immutable lifecycle candidate; only an association can admit it as current.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AssociationContext {
    /// Explicit fixture source, socket and address binding.
    binding: Binding,
    /// Explicit typed configuration, association and clock generations.
    generations: Generations,
}

impl AssociationContext {
    /// Join immutable inputs without checking an existing owner's lifecycle.
    #[must_use]
    pub const fn new(binding: Binding, generations: Generations) -> Self {
        Self {
            binding,
            generations,
        }
    }

    /// Return the immutable binding without granting identity authority.
    #[must_use]
    pub const fn binding(self) -> Binding {
        self.binding
    }

    /// Return the immutable tuple without asserting its currentness.
    #[must_use]
    pub const fn generations(self) -> Generations {
        self.generations
    }
}

/// Checked nonzero wire correlation bits, with no entropy or freshness claim.
///
/// ```
/// use ntp_exchange::{TokenError, WireToken};
/// assert!(matches!(WireToken::new(0), Err(TokenError::Zero)));
/// assert!(WireToken::new(1).is_ok());
/// ```
/// Private construction and default bypasses are rejected:
/// ```compile_fail,E0451
/// use std::num::NonZeroU64;
/// use ntp_exchange::WireToken;
/// let _ = WireToken { value: NonZeroU64::MIN };
/// ```
/// ```compile_fail,E0277
/// use ntp_exchange::WireToken;
/// let _: WireToken = Default::default();
/// ```
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct WireToken {
    /// Sole checked nonzero representation; never expose through raw projections.
    value: NonZeroU64,
}

impl WireToken {
    /// Admit every nonzero wire value without normalization or token generation.
    ///
    /// # Errors
    /// Rejects zero with a fixed error and retains no rejected input.
    pub fn new(value: u64) -> Result<Self, TokenError> {
        NonZeroU64::new(value)
            .map(|value| Self { value })
            .ok_or(TokenError::Zero)
    }
}

impl fmt::Debug for WireToken {
    /// Format only a fixed redaction, never the correlation value.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("WireToken(<redacted>)")
    }
}

/// Fixed nonzero-token failure; rejected wire values are not retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenError {
    /// Zero cannot be a request token or matching response origin.
    Zero,
}

impl fmt::Display for TokenError {
    /// Write a fixed bounded error without exposing input.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("zero wire token")
    }
}

impl std::error::Error for TokenError {}

/// Positive monotonic request lifetime bounded by the named fixture ceiling.
///
/// ```
/// # use ntp_exchange::{RequestTiming, TimingError};
/// # use time_domain::MonotonicNs;
/// # fn main() -> Result<(), TimingError> {
/// let timing = RequestTiming::new(MonotonicNs::new(5), MonotonicNs::new(6))?;
/// assert_eq!(timing.deadline().nanoseconds(), 6);
/// # Ok(()) }
/// ```
/// An active-awake reading cannot substitute for a monotonic deadline:
/// ```compile_fail,E0308
/// use ntp_exchange::RequestTiming;
/// use time_domain::{ActiveAwakeNs, MonotonicNs};
/// let _ = RequestTiming::new(MonotonicNs::new(5), ActiveAwakeNs::new(6));
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RequestTiming {
    /// Supplied monotonic send instant, on the asserted shared fixture mapping.
    send: MonotonicNs,
    /// Exclusive response-processing deadline, after send and within budget.
    deadline: MonotonicNs,
}

impl RequestTiming {
    /// Check an exact positive same-domain interval before exposing timing.
    ///
    /// # Errors
    /// Rejects equal or reversed readings and lifetimes above one second.
    /// Checked subtraction never wraps; the inputs are unchanged on failure.
    pub fn new(send: MonotonicNs, deadline: MonotonicNs) -> Result<Self, TimingError> {
        let lifetime = deadline
            .duration_since(send)
            .map_err(|_| TimingError::NonPositiveLifetime)?
            .nanoseconds();
        if lifetime == 0 {
            return Err(TimingError::NonPositiveLifetime);
        }
        if lifetime > FIXTURE_MAX_LIFETIME_NS {
            return Err(TimingError::LifetimeExceeded);
        }
        Ok(Self { send, deadline })
    }

    /// Return the immutable same-domain send reading.
    #[must_use]
    pub const fn send(self) -> MonotonicNs {
        self.send
    }

    /// Return the immutable exclusive processing deadline.
    #[must_use]
    pub const fn deadline(self) -> MonotonicNs {
        self.deadline
    }
}

/// Fixed lifetime failures; no rejected timing data is retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimingError {
    /// Equal or reversed endpoints cannot define a positive lifetime.
    NonPositiveLifetime,
    /// The positive interval exceeds the explicit experiment ceiling.
    LifetimeExceeded,
}

impl fmt::Display for TimingError {
    /// Write a fixed bounded message without input values.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::NonPositiveLifetime => "nonpositive request lifetime",
            Self::LifetimeExceeded => "request lifetime exceeded",
        })
    }
}

impl std::error::Error for TimingError {}

/// Explicit numeric windows for the two peer coordinates, without anchor trust.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PeerEraContext {
    /// Inclusive checked window for peer receive time T2.
    receive: EraWindow,
    /// Inclusive checked window for peer transmit time T3.
    transmit: EraWindow,
}

impl PeerEraContext {
    /// Retain two already checked windows on the asserted common fixture scale.
    ///
    /// Both windows are mandatory. Wide windows remain valid inputs; unfolding
    /// may later reject an ambiguous era rather than choosing a date.
    #[must_use]
    pub const fn new(receive: EraWindow, transmit: EraWindow) -> Self {
        Self { receive, transmit }
    }

    /// Return the immutable peer receive window without a trust assertion.
    #[must_use]
    pub const fn receive_window(self) -> EraWindow {
        self.receive
    }

    /// Return the immutable peer transmit window without a trust assertion.
    #[must_use]
    pub const fn transmit_window(self) -> EraWindow {
        self.transmit
    }
}

/// Immutable asserted receipt facts; the association checks all relationships.
///
/// No caller-supplied currentness boolean or configuration/association generation
/// can replace the owner's current context and captured pending context.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReceiptFacts {
    /// Actual fixture driver source, socket and endpoint for this receipt.
    binding: Binding,
    /// Asserted mapping generation used to obtain local T4.
    mapping_generation: ClockGeneration,
    /// Asserted monotonic arrival instant in the request's clock domain.
    arrival: MonotonicNs,
    /// Asserted monotonic processing instant; receive must check the deadline.
    processing_now: MonotonicNs,
    /// Checked local receipt coordinate on the supplied common fixture scale.
    local_t4: UtcCoordinateQ32,
}

impl ReceiptFacts {
    /// Join typed supplied facts without asserting their ordering or currentness.
    #[must_use]
    pub const fn new(
        binding: Binding,
        mapping_generation: ClockGeneration,
        arrival: MonotonicNs,
        processing_now: MonotonicNs,
        local_t4: UtcCoordinateQ32,
    ) -> Self {
        Self {
            binding,
            mapping_generation,
            arrival,
            processing_now,
            local_t4,
        }
    }

    /// Return the asserted receipt binding without native-identity claims.
    #[must_use]
    pub const fn binding(self) -> Binding {
        self.binding
    }

    /// Return the asserted T4 mapping generation without a currentness claim.
    #[must_use]
    pub const fn mapping_generation(self) -> ClockGeneration {
        self.mapping_generation
    }

    /// Return the asserted same-domain monotonic arrival reading.
    #[must_use]
    pub const fn arrival(self) -> MonotonicNs {
        self.arrival
    }

    /// Return the asserted monotonic processing reading.
    #[must_use]
    pub const fn processing_now(self) -> MonotonicNs {
        self.processing_now
    }

    /// Return the supplied checked T4 coordinate without source/truth claims.
    #[must_use]
    pub const fn local_t4(self) -> UtcCoordinateQ32 {
        self.local_t4
    }
}

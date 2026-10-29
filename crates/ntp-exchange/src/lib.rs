// SPDX-License-Identifier: GPL-2.0-only
//! Pure single-slot association for an explicit unauthenticated 48-byte fixture.
//!
//! Typed fixture facts convey no transport identity, clock truth, authentication,
//! statistical eligibility or authority. The association owns its current
//! context and exclusively serializes completion and retirement.
//! No API performs network, clock, persistence, logging or native work.
//!
//! Only the explicit fixture profile is available. Exact observations remain
//! unauthenticated and carry no statistical, transport or clock authority.
#![forbid(unsafe_code)]

mod association;
mod context;

pub use association::{
    ArmError, Association, CancelError, CompletionFacts, ContextError, ExpireError, FixtureEvent,
    OpaqueControl, ReceiveError, UnauthenticatedFixtureObservation,
};
pub use context::{
    AssociationContext, AssociationGeneration, Binding, ClockGeneration, ConfigurationGeneration,
    FIXTURE_MAX_LIFETIME_NS, GenerationError, Generations, PeerEraContext, ReceiptFacts, RequestId,
    RequestTiming, SocketId, SourceId, TimingError, TokenError, WireToken,
};

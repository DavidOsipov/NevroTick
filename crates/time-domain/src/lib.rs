//! Pure checked time coordinates and exact relative arithmetic.
//!
//! Coordinates use an explicitly supplied common NTP-epoch fixture time scale.
//! None of these values proves UTC truth, authentication, caller provenance,
//! clock continuity or authority. There is no ambient clock, I/O or state effect.
#![forbid(unsafe_code)]

mod clock;
mod coordinate;
mod era;
mod exchange;
mod rounding;

pub use clock::{ActiveAwakeDurationNs, ActiveAwakeNs, MonotonicDurationNs, MonotonicNs};
pub use coordinate::{MAX_COORDINATE_TICKS, MIN_COORDINATE_TICKS, UtcCoordinateQ32};
pub use era::{EraWindow, unfold_ntp};
pub use exchange::{ExchangeArithmetic, PhaseOffsetQ33, SignedRoundTripQ32, exchange_arithmetic};
pub use rounding::{RoundedPhaseOffsetNs, RoundedRoundTripNs};

/// Fixed redacted domain failures; rejected input is never retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DomainError {
    /// A coordinate lies outside the signed-i64-seconds Q32 envelope.
    CoordinateOutOfRange,
    /// Era endpoints are out of range or inverted.
    InvalidEraWindow,
    /// No unfolding of the supplied raw timestamp belongs to the window.
    NoEraCandidate,
    /// More than one unfolding belongs to the inclusive window.
    AmbiguousEra,
    /// A checked integer operation cannot represent its result.
    ArithmeticOverflow,
    /// Elapsed time was requested in reverse order.
    DurationOutOfOrder,
    /// Rounded nanoseconds cannot be represented by a signed i64.
    NanosecondsOutOfRange,
    /// A rounded result would violate its exact residual invariant.
    ResidualOutOfRange,
}

impl std::fmt::Display for DomainError {
    /// Write a fixed message without formatting rejected input or garde reports.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::CoordinateOutOfRange => "coordinate out of range",
            Self::InvalidEraWindow => "invalid era window",
            Self::NoEraCandidate => "no era candidate",
            Self::AmbiguousEra => "ambiguous era",
            Self::ArithmeticOverflow => "arithmetic overflow",
            Self::DurationOutOfOrder => "duration out of order",
            Self::NanosecondsOutOfRange => "nanoseconds out of range",
            Self::ResidualOutOfRange => "residual out of range",
        })
    }
}

impl std::error::Error for DomainError {}

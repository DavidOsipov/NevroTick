//! Exact inert four-coordinate arithmetic. Saved-real-T1 provenance, shared
//! time scale and exchange association are caller obligations; there is no
//! authentication, chronology/quality policy, state admission or clock effect.

use crate::{DomainError, RoundedPhaseOffsetNs, RoundedRoundTripNs, UtcCoordinateQ32, rounding};

/// Two differences of admitted Q32 coordinates cannot exceed this magnitude.
const MAX_RELATIVE_NUMERATOR: i128 = (1_i128 << 97) - 2;

/// Enforce the exact transformed-output envelope without allocation or effects.
fn relative_in_range(value: i128) -> bool {
    (-MAX_RELATIVE_NUMERATOR..=MAX_RELATIVE_NUMERATOR).contains(&value)
}

/// Exact offset numerator over 2^33 seconds; positive means local time is behind.
///
/// ```compile_fail,E0451
/// use time_domain::PhaseOffsetQ33;
/// let _ = PhaseOffsetQ33 { numerator: i128::MAX };
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PhaseOffsetQ33 {
    /// Checked sum of two Q32 coordinate differences; retains odd half-ticks.
    numerator: i128,
}

/// Exact signed raw RTT numerator over 2^32 seconds, including negative values.
///
/// ```compile_fail,E0451
/// use time_domain::SignedRoundTripQ32;
/// let _ = SignedRoundTripQ32 { numerator: i128::MIN };
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SignedRoundTripQ32 {
    /// Checked difference of local and peer intervals without precision clamp.
    numerator: i128,
}

/// Exact relative arithmetic results, conveying no accepted-exchange authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExchangeArithmetic {
    /// Checked exact phase result in Q33 units.
    offset: PhaseOffsetQ33,
    /// Checked signed raw delay result in Q32 units.
    round_trip: SignedRoundTripQ32,
}

impl ExchangeArithmetic {
    /// Return the exact immutable offset, preserving its half-Q32-tick units.
    #[must_use]
    pub const fn offset(self) -> PhaseOffsetQ33 {
        self.offset
    }

    /// Return signed raw RTT without introducing sample or precision policy.
    #[must_use]
    pub const fn round_trip(self) -> SignedRoundTripQ32 {
        self.round_trip
    }
}

impl PhaseOffsetQ33 {
    /// Return the immutable signed numerator over 2^33 seconds.
    #[must_use]
    pub const fn numerator(self) -> i128 {
        self.numerator
    }

    /// Round to signed-i64 nanoseconds, retaining the exact Q33 residual.
    ///
    /// # Errors
    /// Rejects checked integer/destination overflow or an invalid residual.
    pub fn round_nanoseconds_ties_even(self) -> Result<RoundedPhaseOffsetNs, DomainError> {
        rounding::round_offset(self.numerator)
    }
}

impl SignedRoundTripQ32 {
    /// Return the immutable signed numerator over 2^32 seconds.
    #[must_use]
    pub const fn numerator(self) -> i128 {
        self.numerator
    }

    /// Round to signed-i64 nanoseconds, retaining the exact Q32 residual.
    ///
    /// # Errors
    /// Rejects checked integer/destination overflow or an invalid residual.
    pub fn round_nanoseconds_ties_even(self) -> Result<RoundedRoundTripNs, DomainError> {
        rounding::round_round_trip(self.numerator)
    }
}

/// Compute n=(T2-T1)+(T3-T4) and d=(T4-T1)-(T3-T2) exactly.
///
/// Inputs must share the caller's coordinate mapping/time scale. The caller must
/// supply its saved real T1; this ordinary coordinate type cannot prove that
/// provenance. Result magnitudes are at most 2^97-2. Work is constant, allocation
/// free and inert; negative raw RTT remains a mathematical result.
///
/// Raw numbers cannot substitute for checked coordinates:
/// ```compile_fail,E0308
/// use time_domain::exchange_arithmetic;
/// let _ = exchange_arithmetic(0, 1, 2, 3);
/// ```
/// A legal external consumer can calculate and explicitly round checked values:
/// ```
/// # use time_domain::{DomainError, UtcCoordinateQ32, exchange_arithmetic};
/// # fn main() -> Result<(), DomainError> {
/// let zero = UtcCoordinateQ32::new(0)?;
/// let one_tick = UtcCoordinateQ32::new(1)?;
/// let exact = exchange_arithmetic(zero, one_tick, zero, zero)?;
/// assert_eq!(exact.offset().numerator(), 1);
/// let rounded = exact.offset().round_nanoseconds_ties_even()?;
/// assert_eq!(rounded.nanoseconds(), 0);
/// assert_eq!(rounded.residual_numerator(), 1_000_000_000);
/// # Ok(()) }
/// ```
///
/// # Errors
/// Rejects checked arithmetic or transformed-output invariant failures.
pub fn exchange_arithmetic(
    real_t1: UtcCoordinateQ32,
    peer_t2: UtcCoordinateQ32,
    peer_t3: UtcCoordinateQ32,
    local_t4: UtcCoordinateQ32,
) -> Result<ExchangeArithmetic, DomainError> {
    let receive_delta = peer_t2
        .ticks()
        .checked_sub(real_t1.ticks())
        .ok_or(DomainError::ArithmeticOverflow)?;
    let transmit_delta = peer_t3
        .ticks()
        .checked_sub(local_t4.ticks())
        .ok_or(DomainError::ArithmeticOverflow)?;
    let offset_numerator = receive_delta
        .checked_add(transmit_delta)
        .ok_or(DomainError::ArithmeticOverflow)?;
    let local_elapsed = local_t4
        .ticks()
        .checked_sub(real_t1.ticks())
        .ok_or(DomainError::ArithmeticOverflow)?;
    let peer_elapsed = peer_t3
        .ticks()
        .checked_sub(peer_t2.ticks())
        .ok_or(DomainError::ArithmeticOverflow)?;
    let round_trip_numerator = local_elapsed
        .checked_sub(peer_elapsed)
        .ok_or(DomainError::ArithmeticOverflow)?;
    if !relative_in_range(offset_numerator) || !relative_in_range(round_trip_numerator) {
        return Err(DomainError::ArithmeticOverflow);
    }
    Ok(ExchangeArithmetic {
        offset: PhaseOffsetQ33 {
            numerator: offset_numerator,
        },
        round_trip: SignedRoundTripQ32 {
            numerator: round_trip_numerator,
        },
    })
}

#[cfg(test)]
/// Independent raw-predicate boundaries, without bypassing domain construction.
mod tests {
    use super::relative_in_range;

    /// Challenge the raw helper contract without constructing invalid domain values.
    ///
    /// # Panics
    /// Panics if a valid boundary is rejected or an out-of-range input is admitted.
    #[test]
    fn relative_envelope_includes_endpoints_and_rejects_neighbors() {
        // Independently calculated from twice the distance between the documented
        // coordinate endpoints, rather than importing the production constant.
        const LIMIT: i128 = 158_456_325_028_528_675_187_087_900_670;
        for admitted in [-LIMIT, -LIMIT + 1, 0, LIMIT - 1, LIMIT] {
            assert!(relative_in_range(admitted), "valid boundary: {admitted}");
        }
        for rejected in [
            i128::MIN,
            -LIMIT - 2,
            -LIMIT - 1,
            LIMIT + 1,
            LIMIT + 2,
            i128::MAX,
        ] {
            assert!(!relative_in_range(rejected), "outside envelope: {rejected}");
        }
    }
}

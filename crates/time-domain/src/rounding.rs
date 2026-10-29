//! Explicit nearest-ties-even integer-nanosecond conversion with exact signed
//! quantization residuals. Fixed-size arithmetic introduces no correction policy,
//! allocation, clock effects or hidden floating-point conversion.

use crate::DomainError;

/// Only these fixed, nonzero binary scales may reach integer division.
#[derive(Clone, Copy)]
enum Scale {
    /// Exact phase numerator uses a denominator of 2^33 seconds.
    Phase,
    /// Exact signed RTT numerator uses a denominator of 2^32 seconds.
    RoundTrip,
}

impl Scale {
    /// Return the exact positive denominator owned by the source unit.
    const fn denominator(self) -> i128 {
        match self {
            Self::Phase => 1_i128 << 33,
            Self::RoundTrip => 1_i128 << 32,
        }
    }
}

/// Round any signed numerator at a fixed scale, checking every transformed result.
///
/// Returns i64 nanoseconds and the exact bounded signed residual without
/// allocation or effects.
///
/// # Errors
/// Rejects arithmetic overflow, an out-of-i64 destination or an invalid residual.
fn round(numerator: i128, scale: Scale) -> Result<(i64, i128), DomainError> {
    let denominator = scale.denominator();
    let scaled = numerator
        .checked_mul(1_000_000_000)
        .ok_or(DomainError::ArithmeticOverflow)?;
    // The scale supplies a positive power of two; Euclidean division gives a
    // floor and nonnegative remainder for negative numerators as well.
    let floor = scaled.div_euclid(denominator);
    let remainder = scaled.rem_euclid(denominator);
    let halfway = denominator / 2;
    let round_up = remainder > halfway || (remainder == halfway && floor % 2 != 0);
    let rounded = if round_up {
        floor
            .checked_add(1)
            .ok_or(DomainError::ArithmeticOverflow)?
    } else {
        floor
    };
    let nanoseconds = i64::try_from(rounded).map_err(|_| DomainError::NanosecondsOutOfRange)?;
    let reconstructed = rounded
        .checked_mul(denominator)
        .ok_or(DomainError::ArithmeticOverflow)?;
    let residual = scaled
        .checked_sub(reconstructed)
        .ok_or(DomainError::ArithmeticOverflow)?;
    if !(-halfway..=halfway).contains(&residual)
        || ((residual == -halfway || residual == halfway) && nanoseconds % 2 != 0)
    {
        return Err(DomainError::ResidualOutOfRange);
    }
    Ok((nanoseconds, residual))
}

/// Rounded phase nanoseconds plus residual numerator over 2^33.
///
/// ```compile_fail,E0451
/// use time_domain::RoundedPhaseOffsetNs;
/// let _ = RoundedPhaseOffsetNs { nanoseconds: 1, residual_numerator: i128::MAX };
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RoundedPhaseOffsetNs {
    /// Checked rounded signed-i64 nanosecond destination.
    nanoseconds: i64,
    /// Exact residual in [-2^32, 2^32], with even destination at ties.
    residual_numerator: i128,
}

/// Rounded signed raw RTT nanoseconds plus residual numerator over 2^32.
///
/// ```compile_fail,E0451
/// use time_domain::RoundedRoundTripNs;
/// let _ = RoundedRoundTripNs { nanoseconds: 1, residual_numerator: i128::MIN };
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RoundedRoundTripNs {
    /// Checked rounded signed-i64 nanosecond destination.
    nanoseconds: i64,
    /// Exact residual in [-2^31, 2^31], with even destination at ties.
    residual_numerator: i128,
}

impl RoundedPhaseOffsetNs {
    /// Return the immutable checked rounded nanoseconds.
    #[must_use]
    pub const fn nanoseconds(self) -> i64 {
        self.nanoseconds
    }

    /// Return exact signed residual numerator in nanoseconds divided by 2^33.
    #[must_use]
    pub const fn residual_numerator(self) -> i128 {
        self.residual_numerator
    }

    /// Return the fixed denominator 2^33, making reconstruction explicit.
    #[must_use]
    pub const fn residual_denominator(self) -> i128 {
        1_i128 << 33
    }
}

impl RoundedRoundTripNs {
    /// Return the immutable checked rounded signed nanoseconds.
    #[must_use]
    pub const fn nanoseconds(self) -> i64 {
        self.nanoseconds
    }

    /// Return exact signed residual numerator in nanoseconds divided by 2^32.
    #[must_use]
    pub const fn residual_numerator(self) -> i128 {
        self.residual_numerator
    }

    /// Return the fixed denominator 2^32, making reconstruction explicit.
    #[must_use]
    pub const fn residual_denominator(self) -> i128 {
        1_i128 << 32
    }
}

/// Convert an operation-produced Q33 numerator and validate destination/residual.
///
/// # Errors
/// Rejects checked overflow, an out-of-i64 rounded result or invalid residual.
pub(crate) fn round_offset(numerator: i128) -> Result<RoundedPhaseOffsetNs, DomainError> {
    let (nanoseconds, residual_numerator) = round(numerator, Scale::Phase)?;
    Ok(RoundedPhaseOffsetNs {
        nanoseconds,
        residual_numerator,
    })
}

/// Convert an operation-produced Q32 numerator and validate destination/residual.
///
/// # Errors
/// Rejects checked overflow, an out-of-i64 rounded result or invalid residual.
pub(crate) fn round_round_trip(numerator: i128) -> Result<RoundedRoundTripNs, DomainError> {
    let (nanoseconds, residual_numerator) = round(numerator, Scale::RoundTrip)?;
    Ok(RoundedRoundTripNs {
        nanoseconds,
        residual_numerator,
    })
}

#[cfg(test)]
/// Raw-helper boundary controls without constructing invalid public results.
mod tests {
    use super::{DomainError, Scale, round};

    /// Distinguish scaling overflow from a representable out-of-i64 destination.
    ///
    /// # Panics
    /// Panics if either scale loses the documented raw-input error distinction.
    #[test]
    fn scaling_overflow_has_representable_neighbors() {
        // Independently calculated floor(i128::MAX / 1_000_000_000); the negative
        // endpoint has the same last-fitting magnitude despite signed asymmetry.
        const LAST_FITTING: i128 = 170_141_183_460_469_231_731_687_303_715;
        for scale in [Scale::Phase, Scale::RoundTrip] {
            assert_eq!(round(0, scale), Ok((0, 0)));
            for fitting in [-LAST_FITTING, LAST_FITTING] {
                assert_eq!(
                    round(fitting, scale),
                    Err(DomainError::NanosecondsOutOfRange)
                );
            }
            for overflowing in [i128::MIN, -LAST_FITTING - 1, LAST_FITTING + 1, i128::MAX] {
                assert_eq!(
                    round(overflowing, scale),
                    Err(DomainError::ArithmeticOverflow)
                );
            }
        }
    }
}

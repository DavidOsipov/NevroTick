// SPDX-License-Identifier: GPL-2.0-only
//! Once-rounded exact rational seconds with a separate representation-error bound.
//!
//! The exact signed-magnitude tuple denotes seconds. It establishes no source
//! identity, physical accuracy, freshness, clock mapping or permission. All work
//! uses bounded scalar storage, with no ambient reads, I/O or state mutation.

use std::{fmt, num::NonZeroU64};

use garde::Validate;

/// Exact binary64 upper magnitude, including a carry from the largest input.
const MAX_SECONDS: f64 = f64::from_bits(0x47f0_0000_0000_0000);
/// Exact lower magnitude, including rounding down from the smallest input ratio.
const MIN_SECONDS: f64 = f64::from_bits(0x3bf0_0000_0000_0000);
/// Smallest nonzero representation-error bound, exactly `2^-117` seconds.
const MIN_ROUNDOFF_SECONDS: f64 = f64::from_bits(0x38a0_0000_0000_0000);
/// Largest representation-error bound, exactly `2^74` seconds.
const MAX_ROUNDOFF_SECONDS: f64 = f64::from_bits(0x4490_0000_0000_0000);

/// Sign of the exact rational seconds; a zero numerator canonicalizes to positive.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RationalSign {
    /// A nonnegative exact value.
    Positive,
    /// A negative exact value when the numerator magnitude is nonzero.
    Negative,
}

/// Fixed, payload-free failures from checked arithmetic or output admission.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RationalSecondsError {
    /// A checked internal arithmetic operation contradicted its bounded contract.
    ArithmeticInvariant,
    /// A transformed result failed its mandatory postcondition validation.
    OutputInvariant,
}

impl fmt::Display for RationalSecondsError {
    /// Write only a fixed failure message, never operands or a validation report.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::ArithmeticInvariant => "rational seconds arithmetic invariant",
            Self::OutputInvariant => "rational seconds output invariant",
        })
    }
}

impl std::error::Error for RationalSecondsError {}

/// Immutable exact seconds, their nearest binary64 value and arithmetic error bound.
///
/// The retained tuple denotes `sign * numerator_magnitude / denominator` seconds.
/// Unreduced tuples can denote equal numbers; this representation intentionally
/// provides no numeric equality, hashing or ordering traits. Its error bound is
/// representation error only, never clock or capture uncertainty.
///
/// A legal external consumer retains the exact source and canonical zero:
/// ```
/// # use std::num::NonZeroU64;
/// # use time_domain::{RationalSecondsError, RationalSign, RoundedRationalSeconds};
/// # fn main() -> Result<(), RationalSecondsError> {
/// let value = RoundedRationalSeconds::new(RationalSign::Negative, 0, NonZeroU64::MIN)?;
/// assert_eq!(value.sign(), RationalSign::Positive);
/// assert_eq!(value.numerator_magnitude(), 0);
/// assert_eq!(value.denominator(), NonZeroU64::MIN);
/// assert_eq!(value.seconds().to_bits(), 0);
/// assert_eq!(value.absolute_roundoff_seconds().to_bits(), 0);
/// # Ok(()) }
/// ```
/// Raw fields cannot bypass construction:
/// ```compile_fail,E0451
/// use std::num::NonZeroU64;
/// use time_domain::{RationalSign, RoundedRationalSeconds};
/// let _ = RoundedRationalSeconds {
///     sign: RationalSign::Positive,
///     numerator_magnitude: 1,
///     denominator: NonZeroU64::MIN,
///     seconds: 0.0,
///     absolute_roundoff_seconds: 0.0,
/// };
/// ```
/// Existing values cannot be mutated:
/// ```compile_fail,E0616
/// use time_domain::RoundedRationalSeconds;
/// let _ = |mut value: RoundedRationalSeconds| { value.seconds = 0.0; };
/// ```
/// A primitive cannot supply an unchecked conversion:
/// ```compile_fail,E0277
/// use time_domain::RoundedRationalSeconds;
/// let _: RoundedRationalSeconds = 1_u128.into();
/// ```
/// Missing results cannot receive a default replacement:
/// ```compile_fail,E0599
/// use time_domain::RoundedRationalSeconds;
/// let _ = RoundedRationalSeconds::default();
/// ```
#[derive(Clone, Copy)]
pub struct RoundedRationalSeconds {
    /// Canonical sign; a zero numerator always retains `Positive`.
    sign: RationalSign,
    /// Exact immutable numerator magnitude in seconds times the denominator.
    numerator_magnitude: u128,
    /// Exact positive denominator, with nonzero admission owned by its type.
    denominator: NonZeroU64,
    /// Checked nearest binary64 seconds, with numerical zero canonicalized to +0.
    seconds: f64,
    /// Zero for exact representation or the original binade's half-ULP bound.
    absolute_roundoff_seconds: f64,
}

impl fmt::Debug for RoundedRationalSeconds {
    /// Expose the type without formatting the exact or floating numeric payload.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("RoundedRationalSeconds { .. }")
    }
}

/// Transformed output with mandatory scalar validation and typed exact inputs.
#[derive(Validate)]
struct OutputCandidate {
    /// The closed sign enum and `new`'s zero canonicalization own this invariant.
    #[garde(skip)]
    sign: RationalSign,
    /// The primitive `u128` type owns the complete admitted magnitude envelope.
    #[garde(skip)]
    numerator_magnitude: u128,
    /// `NonZeroU64` construction owns the complete positive denominator envelope.
    #[garde(skip)]
    denominator: NonZeroU64,
    /// One shared postcondition rule admits only supported seconds representations.
    #[garde(custom(seconds_rule))]
    seconds: f64,
    /// One shared postcondition rule admits only supported arithmetic bounds.
    #[garde(custom(roundoff_rule))]
    absolute_roundoff_seconds: f64,
}

/// Admit normal finite values or the unique positive-zero representation.
fn normal_or_positive_zero(value: f64) -> bool {
    value.is_normal() || value.to_bits() == 0
}

/// Validate transformed seconds without retaining or formatting a rejected value.
/// # Errors
/// Reject all except positive zero or signed normals in `[2^-64,2^128]` magnitude.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rules require borrowed values and context"
)]
fn seconds_rule(value: &f64, (): &()) -> garde::Result {
    if normal_or_positive_zero(*value)
        && (value.to_bits() == 0 || (MIN_SECONDS..=MAX_SECONDS).contains(&value.abs()))
    {
        Ok(())
    } else {
        Err(garde::Error::new("invalid rational seconds"))
    }
}

/// Validate a transformed error bound without retaining a rejected value.
/// # Errors
/// Reject all except positive zero or a binary power in `[2^-117,2^74]`.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde custom rules require borrowed values and context"
)]
fn roundoff_rule(value: &f64, (): &()) -> garde::Result {
    if value.to_bits() == 0
        || (value.is_normal()
            && value.to_bits() & ((1_u64 << 52) - 1) == 0
            && (MIN_ROUNDOFF_SECONDS..=MAX_ROUNDOFF_SECONDS).contains(value))
    {
        Ok(())
    } else {
        Err(garde::Error::new("invalid rational seconds roundoff"))
    }
}

/// Checked normalized quotient and its exact proper remainder.
///
/// `normalized_ratio` alone constructs these fields. The significand is in
/// `[2^52,2^53)`, the remainder is below its positive denominator, and the exact
/// input's binade exponent is in `[-64,127]`. The scaled denominator is below
/// `2^76`, so doubling its proper remainder fits `u128`.
struct NormalizedRatio {
    /// Truncated 53-bit significand before nearest/ties-even rounding.
    significand: u64,
    /// Exact nonnegative remainder from the one scaled integer division.
    remainder: u128,
    /// Positive scaled denominator of the proper remainder.
    denominator: u128,
    /// Original exact-ratio binade exponent, retained across a rounding carry.
    original_exponent: i32,
}

/// Private unsigned conversion result and the derivation facts used at publication.
///
/// `round_magnitude` owns the exactness flag and original exponent. Publication
/// checks their association with the floating result and bound; no caller may
/// substitute a floating estimate or a physical uncertainty for these fields.
#[derive(Clone, Copy)]
struct MagnitudeRounding {
    /// Nonnegative nearest/ties-even value before the exact sign application.
    seconds: f64,
    /// Zero iff exact, otherwise the original binade's exact half-ULP bound.
    absolute_roundoff_seconds: f64,
    /// Original binade in `[-64,127]`, or zero for exact numeric zero.
    original_exponent: i32,
    /// Whether the scaled integer division has an exact zero remainder.
    exact: bool,
}

/// Multiply by an exact binary power, rejecting shift or product overflow.
///
/// All raw `u128` values and `u32` shifts are accepted. Materializing the power
/// checks the shift count; checked multiplication also detects high-bit loss,
/// which `checked_shl` alone would not detect. Work is fixed and effect-free.
/// # Errors
/// Return a fixed invariant failure for a shift at least 128 or product overflow.
fn scale_integer(value: u128, shift: u32) -> Result<u128, RationalSecondsError> {
    let power = 1_u128
        .checked_shl(shift)
        .ok_or(RationalSecondsError::ArithmeticInvariant)?;
    value
        .checked_mul(power)
        .ok_or(RationalSecondsError::ArithmeticInvariant)
}

/// Find the exact nonzero ratio's binade using bit lengths and one comparison.
///
/// The magnitude must be nonzero; the denominator's type owns its positive-u64
/// invariant. The bit-length difference is in `[-63,127]`. Aligning its shorter
/// operand uses at most 128 bits and an exact comparison decides whether to
/// decrement it. The checked result is in `[-64,127]`, without floating work.
/// # Errors
/// Reject zero magnitude or an impossible checked arithmetic/range disagreement.
fn original_binade(magnitude: u128, denominator: NonZeroU64) -> Result<i32, RationalSecondsError> {
    if magnitude == 0 {
        return Err(RationalSecondsError::ArithmeticInvariant);
    }
    let numerator_bits = i32::try_from(magnitude.bit_width())
        .map_err(|_| RationalSecondsError::ArithmeticInvariant)?;
    let denominator_bits = i32::try_from(denominator.get().bit_width())
        .map_err(|_| RationalSecondsError::ArithmeticInvariant)?;
    let candidate = numerator_bits
        .checked_sub(denominator_bits)
        .ok_or(RationalSecondsError::ArithmeticInvariant)?;
    let below_candidate = if candidate >= 0 {
        let shift =
            u32::try_from(candidate).map_err(|_| RationalSecondsError::ArithmeticInvariant)?;
        magnitude < scale_integer(u128::from(denominator.get()), shift)?
    } else {
        let shift = candidate
            .checked_neg()
            .and_then(|value| u32::try_from(value).ok())
            .ok_or(RationalSecondsError::ArithmeticInvariant)?;
        scale_integer(magnitude, shift)? < u128::from(denominator.get())
    };
    let exponent = if below_candidate {
        candidate
            .checked_sub(1)
            .ok_or(RationalSecondsError::ArithmeticInvariant)?
    } else {
        candidate
    };
    if !(-64..=127).contains(&exponent) {
        return Err(RationalSecondsError::ArithmeticInvariant);
    }
    Ok(exponent)
}

/// Normalize a nonzero exact ratio through one scaled quotient and remainder.
///
/// The binade owner establishes the scaling direction and exponent. For
/// `E<=52`, `N*2^(52-E) < D*2^53 < 2^117`; otherwise the scaled denominator
/// `D*2^(E-52)` has at most 76 bits. Checked multiplication protects both facts.
/// Output significand and proper remainder are checked before publication. Work
/// uses fixed scalar storage, without loops, allocation or external effects.
/// # Errors
/// Reject zero or any checked scaling, division, conversion or output disagreement.
fn normalized_ratio(
    magnitude: u128,
    denominator: NonZeroU64,
) -> Result<NormalizedRatio, RationalSecondsError> {
    let exponent = original_binade(magnitude, denominator)?;
    let shift = 52_i32
        .checked_sub(exponent)
        .ok_or(RationalSecondsError::ArithmeticInvariant)?;
    let denominator = u128::from(denominator.get());
    let (numerator, denominator) = if shift >= 0 {
        let shift = u32::try_from(shift).map_err(|_| RationalSecondsError::ArithmeticInvariant)?;
        (scale_integer(magnitude, shift)?, denominator)
    } else {
        let shift = shift
            .checked_neg()
            .and_then(|value| u32::try_from(value).ok())
            .ok_or(RationalSecondsError::ArithmeticInvariant)?;
        (magnitude, scale_integer(denominator, shift)?)
    };
    let quotient = numerator
        .checked_div(denominator)
        .ok_or(RationalSecondsError::ArithmeticInvariant)?;
    let remainder = numerator
        .checked_rem(denominator)
        .ok_or(RationalSecondsError::ArithmeticInvariant)?;
    let significand =
        u64::try_from(quotient).map_err(|_| RationalSecondsError::ArithmeticInvariant)?;
    if !((1_u64 << 52)..(1_u64 << 53)).contains(&significand)
        || remainder >= denominator
        || denominator >= (1_u128 << 76)
    {
        return Err(RationalSecondsError::ArithmeticInvariant);
    }
    Ok(NormalizedRatio {
        significand,
        remainder,
        denominator,
        original_exponent: exponent,
    })
}

/// Construct an exact normal binary64 power of two from its unbiased exponent.
///
/// All `i32` arguments are accepted; only `[-1022,1023]` produce a normal value.
/// Integer bias/range/shift checks precede bit construction, without arithmetic
/// on floating values, allocation or external effects.
/// # Errors
/// Reject an unsupported exponent or an impossible checked encoding disagreement.
fn binary_power(exponent: i32) -> Result<f64, RationalSecondsError> {
    let biased = exponent
        .checked_add(1023)
        .and_then(|value| u64::try_from(value).ok())
        .ok_or(RationalSecondsError::ArithmeticInvariant)?;
    if !(1..=2046).contains(&biased) {
        return Err(RationalSecondsError::ArithmeticInvariant);
    }
    let bits = biased
        .checked_shl(52)
        .ok_or(RationalSecondsError::ArithmeticInvariant)?;
    Ok(f64::from_bits(bits))
}

/// Round one unsigned exact seconds ratio to nearest, resolving ties to even.
///
/// Zero returns exact positive zero. For nonzero input the normalized owner
/// supplies 53 significand bits and an exact proper remainder. Comparing twice
/// that remainder chooses rounding without a floating intermediate. A carry
/// increments only the output exponent; error uses the original binade. The
/// retained exactness flag is true precisely for a zero division remainder.
/// Work is fixed-size, without allocation, I/O, time lookup or state mutation.
/// # Errors
/// Preserve typed normalization, checked rounding or bit-encoding disagreement.
fn round_magnitude(
    magnitude: u128,
    denominator: NonZeroU64,
) -> Result<MagnitudeRounding, RationalSecondsError> {
    if magnitude == 0 {
        return Ok(MagnitudeRounding {
            seconds: 0.0,
            absolute_roundoff_seconds: 0.0,
            original_exponent: 0,
            exact: true,
        });
    }
    let normalized = normalized_ratio(magnitude, denominator)?;
    let doubled_remainder = normalized
        .remainder
        .checked_mul(2)
        .ok_or(RationalSecondsError::ArithmeticInvariant)?;
    let round_up = doubled_remainder > normalized.denominator
        || (doubled_remainder == normalized.denominator && normalized.significand & 1 != 0);
    let rounded = normalized
        .significand
        .checked_add(u64::from(round_up))
        .ok_or(RationalSecondsError::ArithmeticInvariant)?;
    let (significand, output_exponent) = if rounded == 1_u64 << 53 {
        let exponent = normalized
            .original_exponent
            .checked_add(1)
            .ok_or(RationalSecondsError::ArithmeticInvariant)?;
        (1_u64 << 52, exponent)
    } else {
        (rounded, normalized.original_exponent)
    };
    let fraction = significand
        .checked_sub(1_u64 << 52)
        .ok_or(RationalSecondsError::ArithmeticInvariant)?;
    let seconds = f64::from_bits(binary_power(output_exponent)?.to_bits() | fraction);
    let exact = normalized.remainder == 0;
    let absolute_roundoff_seconds = if exact {
        0.0
    } else {
        let exponent = normalized
            .original_exponent
            .checked_sub(53)
            .ok_or(RationalSecondsError::ArithmeticInvariant)?;
        binary_power(exponent)?
    };
    Ok(MagnitudeRounding {
        seconds,
        absolute_roundoff_seconds,
        original_exponent: normalized.original_exponent,
        exact,
    })
}

/// Check representation associations and mandatorily admit both floating fields.
///
/// Typed inputs retain their full envelopes; `new` owns zero canonicalization.
/// The private conversion owns exactness/binade facts. This owner checks positive
/// unsigned magnitude, canonical zero, sign and original-binade error association.
/// Both shared garde rules execute before constructing a public result.
/// It does not repeat the rational algorithm or infer physical/source authority.
/// No partial result escapes; one bounded temporary garde report may allocate.
/// # Errors
/// Reject scalar or association disagreement using a fixed payload-free error.
fn publish(
    sign: RationalSign,
    magnitude: u128,
    denominator: NonZeroU64,
    rounded: MagnitudeRounding,
) -> Result<RoundedRationalSeconds, RationalSecondsError> {
    // Admit the raw unsigned magnitude before applying its requested sign. This
    // includes negative zero, and does not rely on today's checked producer.
    if rounded.seconds.to_bits() >> 63 != 0 {
        return Err(RationalSecondsError::OutputInvariant);
    }
    let sign_bit = match sign {
        RationalSign::Positive => 0,
        RationalSign::Negative => 1_u64 << 63,
    };
    let candidate = OutputCandidate {
        sign,
        numerator_magnitude: magnitude,
        denominator,
        seconds: f64::from_bits(rounded.seconds.to_bits() | sign_bit),
        absolute_roundoff_seconds: rounded.absolute_roundoff_seconds,
    };
    candidate
        .validate()
        .map_err(|_| RationalSecondsError::OutputInvariant)?;
    if magnitude == 0 {
        if sign != RationalSign::Positive
            || candidate.seconds.to_bits() != 0
            || candidate.absolute_roundoff_seconds.to_bits() != 0
            || rounded.original_exponent != 0
            || !rounded.exact
        {
            return Err(RationalSecondsError::OutputInvariant);
        }
    } else {
        if !rounded.seconds.is_normal() || !(-64..=127).contains(&rounded.original_exponent) {
            return Err(RationalSecondsError::OutputInvariant);
        }
        let expected_bound = if rounded.exact {
            0.0
        } else {
            let exponent = rounded
                .original_exponent
                .checked_sub(53)
                .ok_or(RationalSecondsError::OutputInvariant)?;
            binary_power(exponent).map_err(|_| RationalSecondsError::OutputInvariant)?
        };
        if candidate.absolute_roundoff_seconds.to_bits() != expected_bound.to_bits()
            || candidate.seconds.to_bits() >> 63 != sign_bit >> 63
        {
            return Err(RationalSecondsError::OutputInvariant);
        }
    }
    Ok(RoundedRationalSeconds {
        sign: candidate.sign,
        numerator_magnitude: candidate.numerator_magnitude,
        denominator: candidate.denominator,
        seconds: candidate.seconds,
        absolute_roundoff_seconds: candidate.absolute_roundoff_seconds,
    })
}

impl RoundedRationalSeconds {
    /// Convert exact rational seconds once, retaining their immutable input tuple.
    ///
    /// All `u128` magnitudes and nonzero `u64` denominators are admitted. Zero
    /// canonicalizes to a positive sign and positive floating zero. Nonzero exact
    /// ratios lie strictly between `2^-64` and `2^128`, so nearest/ties-even output
    /// is finite normal. The absolute error bound is zero iff representation is
    /// exact; otherwise it is `2^(E-53)`, using the exact input's binade exponent.
    /// Successful work uses bounded scalar storage without allocation or effects;
    /// rejection may allocate one bounded temporary garde report, then discards it.
    /// # Errors
    /// Reject checked arithmetic or mandatory transformed-output disagreements.
    pub fn new(
        sign: RationalSign,
        magnitude: u128,
        denominator: NonZeroU64,
    ) -> Result<Self, RationalSecondsError> {
        let sign = if magnitude == 0 {
            RationalSign::Positive
        } else {
            sign
        };
        let rounded = round_magnitude(magnitude, denominator)?;
        publish(sign, magnitude, denominator, rounded)
    }

    /// Return the immutable exact sign, canonicalized to positive for zero.
    #[must_use]
    pub const fn sign(self) -> RationalSign {
        self.sign
    }

    /// Return the exact retained numerator magnitude without conversion.
    #[must_use]
    pub const fn numerator_magnitude(self) -> u128 {
        self.numerator_magnitude
    }

    /// Return the exact retained positive denominator in the seconds ratio.
    #[must_use]
    pub const fn denominator(self) -> NonZeroU64 {
        self.denominator
    }

    /// Return the once-rounded nearest/ties-even binary64 seconds.
    #[must_use]
    pub const fn seconds(self) -> f64 {
        self.seconds
    }

    /// Return the arithmetic absolute-error bound in seconds, without new rounding.
    #[must_use]
    pub const fn absolute_roundoff_seconds(self) -> f64 {
        self.absolute_roundoff_seconds
    }
}

#[cfg(test)]
#[path = "../tests/private/rational_seconds.rs"]
mod tests;

//! Independent public exact-ratio conversion facts, frozen before feature RED.
//!
//! Literal answers come from exact rational comparisons with adjacent binary64
//! midpoints. Shifted controls use exact power-of-two scaling of known dyadics
//! and thirds; they do not reproduce the production division/conversion path.
#![forbid(unsafe_code)]

use std::{error::Error, io, num::NonZeroU64};

use time_domain::{RationalSign, RoundedRationalSeconds};

/// Public construction failures and impossible fixture-denominator failures.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;

/// Named fixture: label, sign, magnitude, denominator, seconds bits and bound bits.
type LiteralRatio = (&'static str, RationalSign, u128, u64, u64, u64);

/// Positive fixture: magnitude, denominator, seconds bits and bound bits.
type MidpointNeighbor = (u128, u64, u64, u64);

/// Fifteen frozen named source tuples, rounded seconds bits and bound bits.
const LITERAL_RATIOS: [LiteralRatio; 15] = [
    (
        "negative zero",
        RationalSign::Negative,
        0_u128,
        u64::MAX,
        0_u64,
        0_u64,
    ),
    (
        "exact unit",
        RationalSign::Positive,
        1,
        1,
        0x3ff0_0000_0000_0000,
        0,
    ),
    (
        "smallest positive ratio",
        RationalSign::Positive,
        1,
        u64::MAX,
        0x3bf0_0000_0000_0000,
        0x38a0_0000_0000_0000,
    ),
    (
        "i128 maximum magnitude",
        RationalSign::Positive,
        170_141_183_460_469_231_731_687_303_715_884_105_727,
        1,
        0x47e0_0000_0000_0000,
        0x4480_0000_0000_0000,
    ),
    (
        "u128 maximum positive",
        RationalSign::Positive,
        u128::MAX,
        1,
        0x47f0_0000_0000_0000,
        0x4490_0000_0000_0000,
    ),
    (
        "u128 maximum negative",
        RationalSign::Negative,
        u128::MAX,
        1,
        0xc7f0_0000_0000_0000,
        0x4490_0000_0000_0000,
    ),
    (
        "both unsigned maxima",
        RationalSign::Positive,
        u128::MAX,
        u64::MAX,
        0x43f0_0000_0000_0000,
        0x40a0_0000_0000_0000,
    ),
    (
        "i128 minimum unsigned magnitude",
        RationalSign::Negative,
        170_141_183_460_469_231_731_687_303_715_884_105_728,
        1,
        0xc7e0_0000_0000_0000,
        0,
    ),
    (
        "even lower positive tie",
        RationalSign::Positive,
        9_007_199_254_740_993,
        9_007_199_254_740_992,
        0x3ff0_0000_0000_0000,
        0x3ca0_0000_0000_0000,
    ),
    (
        "odd lower positive tie",
        RationalSign::Positive,
        9_007_199_254_740_995,
        9_007_199_254_740_992,
        0x3ff0_0000_0000_0002,
        0x3ca0_0000_0000_0000,
    ),
    (
        "even lower negative tie",
        RationalSign::Negative,
        9_007_199_254_740_993,
        9_007_199_254_740_992,
        0xbff0_0000_0000_0000,
        0x3ca0_0000_0000_0000,
    ),
    (
        "odd lower negative tie",
        RationalSign::Negative,
        9_007_199_254_740_995,
        9_007_199_254_740_992,
        0xbff0_0000_0000_0002,
        0x3ca0_0000_0000_0000,
    ),
    (
        "tie carries to next binade",
        RationalSign::Positive,
        18_014_398_509_481_983,
        9_007_199_254_740_992,
        0x4000_0000_0000_0000,
        0x3ca0_0000_0000_0000,
    ),
    (
        "numerator cast counterexample",
        RationalSign::Positive,
        9_007_199_254_740_993,
        3,
        0x4325_5555_5555_5556,
        0,
    ),
    (
        "denominator cast counterexample",
        RationalSign::Positive,
        1,
        9_007_199_254_740_993,
        0x3c9f_ffff_ffff_ffff,
        0x3940_0000_0000_0000,
    ),
];

/// Fifteen exact midpoint-neighbor source tuples, rounded bits and bound bits.
const ADJACENT_RATIONAL_NEIGHBORS: [MidpointNeighbor; 15] = [
    (
        36_028_797_018_963_965_u128,
        36_028_797_018_963_968_u64,
        0x3fef_ffff_ffff_ffff_u64,
        0x3c90_0000_0000_0000_u64,
    ),
    (
        36_028_797_018_963_966,
        36_028_797_018_963_968,
        0x3ff0_0000_0000_0000,
        0x3c90_0000_0000_0000,
    ),
    (
        36_028_797_018_963_967,
        36_028_797_018_963_968,
        0x3ff0_0000_0000_0000,
        0x3c90_0000_0000_0000,
    ),
    (
        18_014_398_509_481_985,
        18_014_398_509_481_984,
        0x3ff0_0000_0000_0000,
        0x3ca0_0000_0000_0000,
    ),
    (
        18_014_398_509_481_986,
        18_014_398_509_481_984,
        0x3ff0_0000_0000_0000,
        0x3ca0_0000_0000_0000,
    ),
    (
        18_014_398_509_481_987,
        18_014_398_509_481_984,
        0x3ff0_0000_0000_0001,
        0x3ca0_0000_0000_0000,
    ),
    (
        18_014_398_509_481_989,
        18_014_398_509_481_984,
        0x3ff0_0000_0000_0001,
        0x3ca0_0000_0000_0000,
    ),
    (
        18_014_398_509_481_990,
        18_014_398_509_481_984,
        0x3ff0_0000_0000_0002,
        0x3ca0_0000_0000_0000,
    ),
    (
        18_014_398_509_481_991,
        18_014_398_509_481_984,
        0x3ff0_0000_0000_0002,
        0x3ca0_0000_0000_0000,
    ),
    (
        36_028_797_018_963_965,
        18_014_398_509_481_984,
        0x3fff_ffff_ffff_ffff,
        0x3ca0_0000_0000_0000,
    ),
    (
        36_028_797_018_963_966,
        18_014_398_509_481_984,
        0x4000_0000_0000_0000,
        0x3ca0_0000_0000_0000,
    ),
    (
        36_028_797_018_963_967,
        18_014_398_509_481_984,
        0x4000_0000_0000_0000,
        0x3ca0_0000_0000_0000,
    ),
    (
        36_028_797_018_963_971,
        18_014_398_509_481_984,
        0x4000_0000_0000_0000,
        0x3cb0_0000_0000_0000,
    ),
    (
        36_028_797_018_963_972,
        18_014_398_509_481_984,
        0x4000_0000_0000_0000,
        0x3cb0_0000_0000_0000,
    ),
    (
        36_028_797_018_963_973,
        18_014_398_509_481_984,
        0x4000_0000_0000_0001,
        0x3cb0_0000_0000_0000,
    ),
];

/// Admit a literal nonzero denominator and call the actual public constructor.
/// # Errors
/// Propagates a zero fixture denominator or unexpected production rejection.
fn convert(
    sign: RationalSign,
    magnitude: u128,
    denominator: u64,
) -> TestResult<RoundedRationalSeconds> {
    let denominator = NonZeroU64::new(denominator)
        .ok_or_else(|| io::Error::other("zero rational-seconds fixture denominator"))?;
    Ok(RoundedRationalSeconds::new(sign, magnitude, denominator)?)
}

/// Compare independently fixed output bits and the retained exact source tuple.
/// # Errors
/// Propagates an unexpected rejection of the literal admitted input.
/// # Panics
/// Fails on changed source, noncanonical zero, wrong rounding or wrong error bound.
fn check(
    sign: RationalSign,
    magnitude: u128,
    denominator: u64,
    seconds_bits: u64,
    bound_bits: u64,
    name: &str,
) -> TestResult {
    let value = convert(sign, magnitude, denominator)?;
    let retained_sign = if magnitude == 0 {
        RationalSign::Positive
    } else {
        sign
    };
    assert_eq!(value.sign(), retained_sign, "source sign: {name}");
    assert_eq!(
        value.numerator_magnitude(),
        magnitude,
        "source magnitude: {name}"
    );
    assert_eq!(
        value.denominator().get(),
        denominator,
        "source denominator: {name}"
    );
    assert_eq!(value.seconds().to_bits(), seconds_bits, "seconds: {name}");
    assert_eq!(
        value.absolute_roundoff_seconds().to_bits(),
        bound_bits,
        "rounding bound: {name}"
    );
    assert!(
        value.seconds().is_normal() || value.seconds().to_bits() == 0,
        "result must be normal or canonical positive zero: {name}"
    );
    Ok(())
}

/// Healthy zero/unit controls precede an exact representable quotient whose
/// separately rounded numerator produces a different result.
/// # Errors
/// Propagates unexpected public construction rejection.
/// # Panics
/// Fails when conversion rounds either integer operand before the exact ratio.
#[test]
fn rounds_exact_ratio_once_without_operand_double_rounding() -> TestResult {
    check(RationalSign::Positive, 0, 1, 0, 0, "healthy zero")?;
    check(
        RationalSign::Positive,
        1,
        1,
        0x3ff0_0000_0000_0000,
        0,
        "healthy unit",
    )?;

    // 9_007_199_254_740_993 = 3 * 3_002_399_751_580_331. The quotient
    // is exact binary64; first rounding the numerator loses one integer unit.
    let value = convert(RationalSign::Positive, 9_007_199_254_740_993, 3)?;
    assert_eq!(
        value.seconds().to_bits(),
        0x4325_5555_5555_5556,
        "rational seconds must round the combined exact ratio once"
    );
    assert_eq!(value.absolute_roundoff_seconds().to_bits(), 0);
    assert_eq!(value.numerator_magnitude(), 9_007_199_254_740_993);
    assert_eq!(value.denominator().get(), 3);
    Ok(())
}

/// Fifteen frozen anchors cover signs, zero, integer endpoints, both tie
/// parities, exponent carry and numerator/denominator operand-rounding defects.
/// # Errors
/// Propagates unexpected public construction rejection.
/// # Panics
/// Fails on any changed literal bit pattern, retained tuple or error bound.
#[test]
fn literal_ratios_cover_full_integer_envelope_and_tie_parities() -> TestResult {
    for (name, sign, magnitude, denominator, seconds_bits, bound_bits) in LITERAL_RATIOS {
        check(sign, magnitude, denominator, seconds_bits, bound_bits, name)?;
    }
    Ok(())
}

/// Adjacent rational numerator literals discriminate each side of ties below
/// one, within one binade and across the unequal spacings around two.
/// # Errors
/// Propagates unexpected public construction rejection.
/// # Panics
/// Fails on a wrong tie decision, carry, sign or original-binade error bound.
#[test]
fn adjacent_rational_neighbors_preserve_ties_and_original_binade_bound() -> TestResult {
    for (magnitude, denominator, positive_bits, bound_bits) in ADJACENT_RATIONAL_NEIGHBORS {
        for (sign, sign_bit) in [
            (RationalSign::Positive, 0_u64),
            (RationalSign::Negative, 0x8000_0000_0000_0000),
        ] {
            check(
                sign,
                magnitude,
                denominator,
                positive_bits | sign_bit,
                bound_bits,
                "adjacent rational midpoint neighbor",
            )?;
        }
    }
    Ok(())
}

/// Equal mathematical values keep their distinct unreduced input tuples through
/// public getters; no numeric equality trait is needed to observe either fact.
/// # Errors
/// Propagates unexpected public construction rejection.
/// # Panics
/// Fails on changed bits, lost unreduced source tuples or incorrect exactness.
#[test]
fn equivalent_unreduced_ratios_retain_original_tuples() -> TestResult {
    for (magnitude, denominator, positive_bits, bound_bits) in [
        (
            1_u128,
            3_u64,
            0x3fd5_5555_5555_5555_u64,
            0x3c80_0000_0000_0000_u64,
        ),
        (2, 6, 0x3fd5_5555_5555_5555, 0x3c80_0000_0000_0000),
        (3, 9, 0x3fd5_5555_5555_5555, 0x3c80_0000_0000_0000),
        (1, 2, 0x3fe0_0000_0000_0000, 0),
        (2, 4, 0x3fe0_0000_0000_0000, 0),
        (3, 6, 0x3fe0_0000_0000_0000, 0),
        (u128::from(u64::MAX), u64::MAX, 0x3ff0_0000_0000_0000, 0),
    ] {
        for (sign, sign_bit) in [
            (RationalSign::Positive, 0_u64),
            (RationalSign::Negative, 0x8000_0000_0000_0000),
        ] {
            check(
                sign,
                magnitude,
                denominator,
                positive_bits | sign_bit,
                bound_bits,
                "unreduced source tuple",
            )?;
        }
    }
    Ok(())
}

/// Literal engine scales convert rational nanoseconds directly to seconds.
/// These pure numeric controls do not claim actual engine capture/admission.
/// # Errors
/// Propagates unexpected public construction rejection.
/// # Panics
/// Fails on a unit/scaling mistake, extra rounding or changed retained denominator.
#[test]
fn engine_denominator_literals_preserve_seconds_without_nanosecond_rounding() -> TestResult {
    for (magnitude, denominator, positive_bits, bound_bits) in [
        (
            1_u128,
            8_589_934_592_000_000_000_u64,
            0x3c01_2e0b_e826_d695_u64,
            0x38b0_0000_0000_0000_u64,
        ),
        (
            1,
            4_294_967_296_000_000_000,
            0x3c11_2e0b_e826_d695,
            0x38c0_0000_0000_0000,
        ),
        (
            134_217_728_000_000_000,
            8_589_934_592_000_000_000,
            0x3f90_0000_0000_0000,
            0,
        ),
        (
            335_544_320_000_000_000,
            4_294_967_296_000_000_000,
            0x3fb4_0000_0000_0000,
            0,
        ),
        (
            858_993_459_200,
            8_589_934_592_000_000_000,
            0x3e7a_d7f2_9abc_af48,
            0x3b20_0000_0000_0000,
        ),
        (
            429_496_729_600,
            4_294_967_296_000_000_000,
            0x3e7a_d7f2_9abc_af48,
            0x3b20_0000_0000_0000,
        ),
    ] {
        for (sign, sign_bit) in [
            (RationalSign::Positive, 0_u64),
            (RationalSign::Negative, 0x8000_0000_0000_0000),
        ] {
            check(
                sign,
                magnitude,
                denominator,
                positive_bits | sign_bit,
                bound_bits,
                "engine seconds denominator",
            )?;
        }
    }
    Ok(())
}

/// Exact binary scaling gives 640 independently counted controls over all
/// numerator power shifts, all denominator power shifts and inexact thirds.
/// # Errors
/// Propagates unexpected public construction rejection.
/// # Panics
/// Fails on an incorrect scaled bit pattern, source retention, bound or count.
#[test]
fn counted_binary_scalings_span_full_unsigned_shift_domain() -> TestResult {
    let mut count = 0_usize;
    for (sign, sign_bit) in [
        (RationalSign::Positive, 0_u64),
        (RationalSign::Negative, 0x8000_0000_0000_0000),
    ] {
        for shift in 0_u32..128 {
            let exponent_bits = u64::from(shift) << 52;
            check(
                sign,
                1_u128 << shift,
                1,
                (0x3ff0_0000_0000_0000 + exponent_bits) | sign_bit,
                0,
                "exact shifted integer",
            )?;
            count += 1;
            // RN(1/3) = 0x3fd5555555555555, with binade -2 and bound
            // 2^-55. Exact binary scaling preserves significand and rounding.
            check(
                sign,
                1_u128 << shift,
                3,
                (0x3fd5_5555_5555_5555 + exponent_bits) | sign_bit,
                0x3c80_0000_0000_0000 + exponent_bits,
                "inexact shifted third",
            )?;
            count += 1;
        }
        for shift in 0_u32..64 {
            check(
                sign,
                1,
                1_u64 << shift,
                (0x3ff0_0000_0000_0000 - (u64::from(shift) << 52)) | sign_bit,
                0,
                "exact reciprocal power",
            )?;
            count += 1;
        }
    }
    assert_eq!(count, 640, "complete fixed binary-scaling inventory");
    Ok(())
}

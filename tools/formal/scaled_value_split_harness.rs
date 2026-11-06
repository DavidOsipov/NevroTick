// Full raw (u64 significand, i64 scale) contract of the actual ScaledValue::split.
// The denominator-cutoff-64 reconstruction oracle differs from source cutoff 53.
// Raw rejection is checked before oracle negation; no caller assumption narrows it.

const MAX_SPLIT_SIGNIFICAND: u64 = 9_007_199_254_740_991;

/// Check complete outcomes and exact quotient/remainder on the admitted domain.
fn scaled_value_split_contract(
    significand: u64,
    scale: i64,
) -> Result<(u64, Fraction), FixtureEnclosureError> {
    let observed = ScaledValue::split(significand, scale);
    let required_success = significand <= MAX_SPLIT_SIGNIFICAND && scale >= -1041 && scale <= 11;
    assert_eq!(observed.is_ok(), required_success, "split required success");
    assert_eq!(
        matches!(observed, Err(FixtureEnclosureError::Invariant)),
        !required_success,
        "split rejection classification"
    );
    // Invalid raw scales, including i64::MIN, never reach oracle negation.
    if required_success {
        if let Ok((whole, fraction)) = observed {
            if scale >= 0 {
                let exact = u128::from(significand) << (scale as u32);
                assert_eq!(u128::from(whole), exact, "split exact nonnegative whole");
                assert!(
                    fraction.numerator == 0 && fraction.shift == 0,
                    "split nonnegative zero fraction"
                );
            } else {
                let denominator_shift = (-scale) as u32;
                if denominator_shift < 64 {
                    // For arbitrary u64 output fields and k<=63 this sum is at
                    // most 2^127+2^63-1, strictly below the u128 limit.
                    let reconstructed = (u128::from(whole) << denominator_shift)
                        + u128::from(fraction.numerator);
                    assert_eq!(
                        reconstructed,
                        u128::from(significand),
                        "split exact quotient remainder"
                    );
                    assert!(
                        u128::from(fraction.numerator) < (1_u128 << denominator_shift),
                        "split strict remainder bound"
                    );
                } else {
                    assert_eq!(whole, 0, "split large denominator whole zero");
                    assert_eq!(
                        fraction.numerator, significand,
                        "split large denominator numerator retained"
                    );
                }
                if fraction.numerator != 0 {
                    assert_eq!(fraction.shift, denominator_shift, "split nonzero shift retained");
                }
            }
            if fraction.numerator == 0 {
                assert_eq!(fraction.shift, 0, "split zero canonicalization");
            }
            let output_bits = 64 - fraction.numerator.leading_zeros();
            assert!(output_bits <= 53, "split output numerator bound");
            assert!(fraction.shift <= 1041, "split output shift bound");
            assert!(
                fraction.numerator == 0 || (fraction.shift >= 1 && output_bits <= fraction.shift),
                "split output proper dyadic"
            );
        }
    }
    observed
}

#[kani::proof]
fn scaled_value_split_full_domain() {
    let significand = kani::any::<u64>();
    let scale = kani::any::<i64>();
    let observed = scaled_value_split_contract(significand, scale);
    kani::cover!(
        significand == 0 && scale == 0
            && matches!(observed, Ok((0, Fraction { numerator: 0, shift: 0 }))),
        "split zero at scale zero"
    );
    kani::cover!(
        significand == 0 && scale == -1041
            && matches!(observed, Ok((0, Fraction { numerator: 0, shift: 0 }))),
        "split zero at minimum scale"
    );
    kani::cover!(
        significand == 0 && scale == -64
            && matches!(observed, Ok((0, Fraction { numerator: 0, shift: 0 }))),
        "split zero at negative sixty four"
    );
    kani::cover!(
        significand == 1 && scale == 0
            && matches!(observed, Ok((1, Fraction { numerator: 0, shift: 0 }))),
        "split integral unit"
    );
    kani::cover!(
        significand == MAX_SPLIT_SIGNIFICAND && scale == 11
            && matches!(observed, Ok((18_446_744_073_709_549_568, Fraction { numerator: 0, shift: 0 }))),
        "split maximum raw whole"
    );
    kani::cover!(
        significand == 2 && scale == -1
            && matches!(observed, Ok((1, Fraction { numerator: 0, shift: 0 }))),
        "split exact negative scale"
    );
    kani::cover!(
        significand == 3 && scale == -1
            && matches!(observed, Ok((1, Fraction { numerator: 1, shift: 1 }))),
        "split fractional negative scale"
    );
    kani::cover!(
        significand == MAX_SPLIT_SIGNIFICAND && scale == -52
            && matches!(observed, Ok((1, Fraction { numerator: 4_503_599_627_370_495, shift: 52 }))),
        "split below source cutoff"
    );
    kani::cover!(
        significand == MAX_SPLIT_SIGNIFICAND && scale == -53
            && matches!(observed, Ok((0, Fraction { numerator: MAX_SPLIT_SIGNIFICAND, shift: 53 }))),
        "split at source cutoff"
    );
    kani::cover!(
        significand == MAX_SPLIT_SIGNIFICAND && scale == -63
            && matches!(observed, Ok((0, Fraction { numerator: MAX_SPLIT_SIGNIFICAND, shift: 63 }))),
        "split below oracle cutoff"
    );
    kani::cover!(
        significand == MAX_SPLIT_SIGNIFICAND && scale == -64
            && matches!(observed, Ok((0, Fraction { numerator: MAX_SPLIT_SIGNIFICAND, shift: 64 }))),
        "split at oracle cutoff"
    );
    kani::cover!(
        significand == 1 && scale == -1041
            && matches!(observed, Ok((0, Fraction { numerator: 1, shift: 1041 }))),
        "split maximum fraction shift"
    );
    kani::cover!(
        significand == 9_007_199_254_740_992 && scale == -54
            && matches!(observed, Err(FixtureEnclosureError::Invariant)),
        "split oversized proper significand rejected"
    );
    kani::cover!(
        significand == u64::MAX && scale == -1041
            && matches!(observed, Err(FixtureEnclosureError::Invariant)),
        "split maximum raw significand rejected"
    );
    kani::cover!(
        significand == 0 && scale == 12 && matches!(observed, Err(FixtureEnclosureError::Invariant)),
        "split zero beyond upper scale rejected"
    );
    kani::cover!(
        significand == 1 && scale == 12 && matches!(observed, Err(FixtureEnclosureError::Invariant)),
        "split nonzero beyond upper scale rejected"
    );
    kani::cover!(
        significand == 0 && scale == -1042 && matches!(observed, Err(FixtureEnclosureError::Invariant)),
        "split zero beyond lower scale rejected"
    );
    kani::cover!(
        significand == 1 && scale == -1042 && matches!(observed, Err(FixtureEnclosureError::Invariant)),
        "split nonzero beyond lower scale rejected"
    );
    kani::cover!(
        significand == 0 && scale == i64::MIN && matches!(observed, Err(FixtureEnclosureError::Invariant)),
        "split minimum raw scale rejected"
    );
    kani::cover!(
        significand == 0 && scale == i64::MAX && matches!(observed, Err(FixtureEnclosureError::Invariant)),
        "split maximum raw scale rejected"
    );
}

/// Reject-all loses an ordinary exact integral unit.
#[kani::proof]
fn scaled_value_split_control_success() {
    let observed = ScaledValue::split(1, 0);
    assert!(observed.is_ok(), "split required success");
}

/// A zero residual is valid but cannot represent the independent exact value 3/2.
#[kani::proof]
fn scaled_value_split_control_remainder() {
    let observed = ScaledValue::split(3, -1);
    assert!(
        matches!(observed, Ok((1, Fraction { numerator: 1, shift: 1 }))),
        "split exact quotient remainder"
    );
}

/// Unsupported raw scales are rejected even for an exact zero significand.
#[kani::proof]
fn scaled_value_split_control_error() {
    let observed = ScaledValue::split(0, 12);
    assert!(
        matches!(observed, Err(FixtureEnclosureError::Invariant)),
        "split rejection classification"
    );
}

// Actual representation methods only; source correspondence is prepared separately.
// IEEE quotient/remainder observations use no floating-point arithmetic or assumptions.

const RAW_SIGNED_WHOLE_LIMIT: i128 = 18_446_744_073_709_551_615;

/// Check all raw primitive fields, a stronger domain than product construction.
/// This private proof extension grants no unchecked construction API to callers.
fn signed_whole_contract(whole: u64, negative: bool, fraction: Fraction) -> i128 {
    let observed = ScaledValue { negative, whole, fraction }.signed_whole();
    let zero_fraction = ScaledValue { negative, whole, fraction: Fraction::ZERO }.signed_whole();
    assert_eq!(observed.unsigned_abs(), u128::from(whole), "signed whole exact magnitude");
    assert_eq!(observed < 0, negative && whole != 0, "signed whole strict sign");
    assert_eq!(observed == 0, whole == 0, "signed whole zero equivalence");
    assert!(observed >= -RAW_SIGNED_WHOLE_LIMIT && observed <= RAW_SIGNED_WHOLE_LIMIT,
            "signed whole raw envelope");
    assert_eq!(observed, zero_fraction, "signed whole fraction independence");
    observed
}

#[kani::proof]
fn signed_whole_full_domain() {
    let whole = kani::any::<u64>();
    let negative = kani::any::<bool>();
    let numerator = kani::any::<u64>();
    let shift = kani::any::<u32>();
    let fraction = Fraction { numerator, shift };
    let observed = signed_whole_contract(whole, negative, fraction);
    let zero_fraction = numerator == 0 && shift == 0;
    kani::cover!(
        whole == 0 && !negative && zero_fraction && observed == 0_i128,
        "signed whole positive zero"
    );
    kani::cover!(
        whole == 0 && negative && zero_fraction && observed == 0_i128,
        "signed whole negative zero"
    );
    kani::cover!(
        whole == 1 && !negative && zero_fraction && observed == 1_i128,
        "signed whole positive one"
    );
    kani::cover!(
        whole == 1 && negative && zero_fraction && observed == -1_i128,
        "signed whole negative one"
    );
    kani::cover!(
        whole == 2 && !negative && zero_fraction && observed == 2_i128,
        "signed whole positive two"
    );
    kani::cover!(
        whole == 2 && negative && zero_fraction && observed == -2_i128,
        "signed whole negative two"
    );
    kani::cover!(
        whole == 9_223_372_036_854_775_807 && !negative && zero_fraction && observed == 9_223_372_036_854_775_807_i128,
        "signed whole positive i64 maximum"
    );
    kani::cover!(
        whole == 9_223_372_036_854_775_807 && negative && zero_fraction && observed == -9_223_372_036_854_775_807_i128,
        "signed whole negative i64 maximum"
    );
    kani::cover!(
        whole == 9_223_372_036_854_775_808 && !negative && zero_fraction && observed == 9_223_372_036_854_775_808_i128,
        "signed whole positive two to sixty three"
    );
    kani::cover!(
        whole == 9_223_372_036_854_775_808 && negative && zero_fraction && observed == -9_223_372_036_854_775_808_i128,
        "signed whole negative two to sixty three"
    );
    kani::cover!(
        whole == 9_223_372_036_854_775_809 && !negative && zero_fraction && observed == 9_223_372_036_854_775_809_i128,
        "signed whole positive above two to sixty three"
    );
    kani::cover!(
        whole == 9_223_372_036_854_775_809 && negative && zero_fraction && observed == -9_223_372_036_854_775_809_i128,
        "signed whole negative above two to sixty three"
    );
    kani::cover!(
        whole == 18_446_744_073_709_551_614 && !negative && zero_fraction
            && observed == 18_446_744_073_709_551_614_i128,
        "signed whole positive below u64 maximum"
    );
    kani::cover!(
        whole == 18_446_744_073_709_551_614 && negative && zero_fraction
            && observed == -18_446_744_073_709_551_614_i128,
        "signed whole negative below u64 maximum"
    );
    kani::cover!(
        whole == 18_446_744_073_709_551_615 && !negative && zero_fraction
            && observed == 18_446_744_073_709_551_615_i128,
        "signed whole positive u64 maximum"
    );
    kani::cover!(
        whole == 18_446_744_073_709_551_615 && negative && zero_fraction
            && observed == -18_446_744_073_709_551_615_i128,
        "signed whole negative u64 maximum"
    );
    kani::cover!(
        whole == 0 && !negative && numerator == 1 && shift == 1 && observed == 0,
        "signed whole positive fractional zero"
    );
    kani::cover!(
        whole == 0 && negative && numerator == 1 && shift == 1 && observed == 0,
        "signed whole negative fractional zero"
    );
    kani::cover!(
        whole == 1 && !negative && numerator == u64::MAX && shift == 0 && observed == 1,
        "signed whole ignores raw oversized fraction"
    );
    kani::cover!(
        whole == 1 && negative && numerator == 0 && shift == u32::MAX && observed == -1,
        "signed whole ignores raw maximum fraction shift"
    );
}

/// Observe original IEEE magnitude through exact integer format definitions.
/// All potentially large shifts are gated by independent raw-bit admission.
fn from_bits_contract(bits: u64) -> Result<ScaledValue, FixtureEnclosureError> {
    let observed = ScaledValue::from_bits(bits);
    let magnitude = bits & 0x7fff_ffff_ffff_ffff;
    let required_success = magnitude <= 0x41d0_0000_0000_0000;
    assert_eq!(observed.is_ok(), required_success, "from bits required success");
    assert_eq!(matches!(observed, Err(FixtureEnclosureError::Invariant)), !required_success,
               "from bits rejection classification");
    if required_success {
        if let Ok(value) = observed {
            assert_eq!(value.negative, bits & 0x8000_0000_0000_0000 != 0,
                       "from bits original sign retained");
            // Binary64 uses a 52-bit mantissa and biased exponent 1023. Q33
            // scaling gives 1042 = 1023+52-33. Division/remainder extract the
            // format fields independently of the source's masks and shifts.
            let exponent = (magnitude / 4_503_599_627_370_496) as i64;
            let mantissa = magnitude % 4_503_599_627_370_496;
            let numerator = u128::from(mantissa)
                + if exponent == 0 { 0 } else { 4_503_599_627_370_496_u128 };
            let effective_exponent = if exponent == 0 { 1 } else { exponent };
            let k = 1042_i64 - effective_exponent;
            if k <= 0 {
                let exact = numerator << ((-k) as u32);
                assert_eq!(u128::from(value.whole), exact, "from bits exact integral magnitude");
                assert!(value.fraction.numerator == 0 && value.fraction.shift == 0,
                        "from bits integral zero fraction");
            } else if k < 64 {
                // With arbitrary u64 output fields and 1<=k<=63, this sum is
                // at most 2^127+2^63-1 and cannot overflow u128.
                let reconstructed = (u128::from(value.whole) << (k as u32))
                    + u128::from(value.fraction.numerator);
                assert_eq!(reconstructed, numerator, "from bits exact quotient remainder");
                assert!(u128::from(value.fraction.numerator) < (1_u128 << (k as u32)),
                        "from bits strict remainder bound");
            } else {
                assert_eq!(value.whole, 0, "from bits large denominator whole zero");
                assert_eq!(u128::from(value.fraction.numerator), numerator,
                           "from bits large denominator numerator retained");
            }
            if value.fraction.numerator == 0 {
                assert_eq!(value.fraction.shift, 0, "from bits zero canonicalization");
            } else {
                assert!(k > 0 && i64::from(value.fraction.shift) == k,
                        "from bits nonzero shift retained");
            }
            assert!(value.whole <= 9_223_372_036_854_775_808, "from bits whole envelope");
            let output_bits = 64 - value.fraction.numerator.leading_zeros();
            assert!(output_bits <= 53, "from bits output numerator bound");
            assert!(value.fraction.shift <= 1041, "from bits output shift bound");
            assert!(value.fraction.numerator == 0
                    || (value.fraction.shift >= 1 && output_bits <= value.fraction.shift),
                    "from bits output proper dyadic");
        }
    }
    observed
}

#[kani::proof]
fn from_bits_full_domain() {
    let bits = kani::any::<u64>();
    let observed = from_bits_contract(bits);
    kani::cover!(
        bits == 0x0000000000000000
            && matches!(observed, Ok(ScaledValue { negative: false, whole: 0,
                fraction: Fraction { numerator: 0, shift: 0 } })),
        "from bits positive zero"
    );
    kani::cover!(
        bits == 0x8000000000000000
            && matches!(observed, Ok(ScaledValue { negative: true, whole: 0,
                fraction: Fraction { numerator: 0, shift: 0 } })),
        "from bits negative zero"
    );
    kani::cover!(
        bits == 0x0000000000000001
            && matches!(observed, Ok(ScaledValue { negative: false, whole: 0,
                fraction: Fraction { numerator: 1, shift: 1041 } })),
        "from bits positive minimum subnormal"
    );
    kani::cover!(
        bits == 0x8000000000000001
            && matches!(observed, Ok(ScaledValue { negative: true, whole: 0,
                fraction: Fraction { numerator: 1, shift: 1041 } })),
        "from bits negative minimum subnormal"
    );
    kani::cover!(
        bits == 0x000fffffffffffff
            && matches!(observed, Ok(ScaledValue { negative: false, whole: 0,
                fraction: Fraction { numerator: 4_503_599_627_370_495, shift: 1041 } })),
        "from bits maximum subnormal"
    );
    kani::cover!(
        bits == 0x0010000000000000
            && matches!(observed, Ok(ScaledValue { negative: false, whole: 0,
                fraction: Fraction { numerator: 4_503_599_627_370_496, shift: 1041 } })),
        "from bits minimum normal"
    );
    kani::cover!(
        bits == 0x3dd0000000000000
            && matches!(observed, Ok(ScaledValue { negative: false, whole: 0,
                fraction: Fraction { numerator: 4_503_599_627_370_496, shift: 53 } })),
        "from bits half quantum"
    );
    kani::cover!(
        bits == 0x3ddfffffffffffff
            && matches!(observed, Ok(ScaledValue { negative: false, whole: 0,
                fraction: Fraction { numerator: 9_007_199_254_740_991, shift: 53 } })),
        "from bits below quantum"
    );
    kani::cover!(
        bits == 0x3de0000000000000
            && matches!(observed, Ok(ScaledValue { negative: false, whole: 1,
                fraction: Fraction { numerator: 0, shift: 0 } })),
        "from bits integral quantum"
    );
    kani::cover!(
        bits == 0x3de0000000000001
            && matches!(observed, Ok(ScaledValue { negative: false, whole: 1,
                fraction: Fraction { numerator: 1, shift: 52 } })),
        "from bits above quantum"
    );
    kani::cover!(
        bits == 0x3d30000000000000
            && matches!(observed, Ok(ScaledValue { negative: false, whole: 0,
                fraction: Fraction { numerator: 4_503_599_627_370_496, shift: 63 } })),
        "from bits denominator sixty three"
    );
    kani::cover!(
        bits == 0x3d20000000000000
            && matches!(observed, Ok(ScaledValue { negative: false, whole: 0,
                fraction: Fraction { numerator: 4_503_599_627_370_496, shift: 64 } })),
        "from bits denominator sixty four"
    );
    kani::cover!(
        bits == 0x41d0000000000000
            && matches!(observed, Ok(ScaledValue { negative: false, whole: 9_223_372_036_854_775_808,
                fraction: Fraction { numerator: 0, shift: 0 } })),
        "from bits positive magnitude boundary"
    );
    kani::cover!(
        bits == 0xc1d0000000000000
            && matches!(observed, Ok(ScaledValue { negative: true, whole: 9_223_372_036_854_775_808,
                fraction: Fraction { numerator: 0, shift: 0 } })),
        "from bits negative magnitude boundary"
    );
    kani::cover!(
        bits == 0x41cfffffffffffff
            && matches!(observed, Ok(ScaledValue { negative: false, whole: 9_223_372_036_854_774_784,
                fraction: Fraction { numerator: 0, shift: 0 } })),
        "from bits below magnitude boundary"
    );
    kani::cover!(
        bits == 0x41d0000000000001 && matches!(observed, Err(FixtureEnclosureError::Invariant)),
        "from bits above magnitude boundary rejected"
    );
    kani::cover!(
        bits == 0x7ff0000000000000 && matches!(observed, Err(FixtureEnclosureError::Invariant)),
        "from bits positive infinity rejected"
    );
    kani::cover!(
        bits == 0xfff0000000000000 && matches!(observed, Err(FixtureEnclosureError::Invariant)),
        "from bits negative infinity rejected"
    );
    kani::cover!(
        bits == 0x7ff8000000000001 && matches!(observed, Err(FixtureEnclosureError::Invariant)),
        "from bits quiet NaN rejected"
    );
    kani::cover!(
        bits == 0xfff0000000000001 && matches!(observed, Err(FixtureEnclosureError::Invariant)),
        "from bits negative signalling NaN rejected"
    );
}

/// Omitting negation preserves magnitude but loses the promised nonzero sign.
#[kani::proof]
fn signed_whole_control_sign() {
    let observed = ScaledValue { negative: true, whole: 1, fraction: Fraction::ZERO }.signed_whole();
    assert!(observed < 0, "signed whole strict sign");
}

/// The raw-helper domain includes whole magnitudes beyond i64::MAX.
#[kani::proof]
fn signed_whole_control_magnitude() {
    let observed = ScaledValue { negative: false, whole: u64::MAX, fraction: Fraction::ZERO }.signed_whole();
    assert_eq!(observed.unsigned_abs(), u128::from(u64::MAX), "signed whole exact magnitude");
}

/// Equal whole/sign values must ignore a valid half-fraction's numerator.
#[kani::proof]
fn signed_whole_control_fraction() {
    let zero = ScaledValue { negative: false, whole: 0, fraction: Fraction::ZERO }.signed_whole();
    let half = ScaledValue { negative: false, whole: 0,
                           fraction: Fraction { numerator: 1, shift: 1 } }.signed_whole();
    assert_eq!(half, zero, "signed whole fraction independence");
}

/// The smallest positive binary64 subnormal is exactly one over 2^1041 Q33 units.
#[kani::proof]
fn from_bits_control_subnormal() {
    let observed = ScaledValue::from_bits(1);
    assert!(matches!(observed, Ok(ScaledValue {
        negative: false, whole: 0, fraction: Fraction { numerator: 1, shift: 1041 }
    })), "from bits nonzero shift retained");
}

/// Original negative-zero metadata is retained even though its magnitude is zero.
#[kani::proof]
fn from_bits_control_sign() {
    let observed = ScaledValue::from_bits(0x8000_0000_0000_0000);
    assert!(matches!(observed, Ok(ScaledValue { negative: true, .. })),
            "from bits original sign retained");
}

/// The immediate representable neighbor above 2^30 reaches the whole-bound guard.
#[kani::proof]
fn from_bits_control_error() {
    let observed = ScaledValue::from_bits(0x41d0_0000_0000_0001);
    assert!(matches!(observed, Err(FixtureEnclosureError::Invariant)),
            "from bits rejection classification");
}

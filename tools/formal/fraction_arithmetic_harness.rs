// All checked fractions and pairs, reached through complete actual raw factories.
// No Kani assumptions, callee stubs or unchecked private field assembly.

const HALF_NUMERATOR: u64 = 4_503_599_627_370_496;
const MAX_NUMERATOR: u64 = 9_007_199_254_740_991;

fn arithmetic_raw_admitted(numerator: u64, shift: u32) -> bool {
    let bits = 64 - numerator.leading_zeros();
    shift <= 1041 && bits <= 53 && (numerator == 0 || bits <= shift)
}

/// Independent half threshold, guarded before forming the finite denominator.
fn complement_contract(n: u64, s: u32) -> Option<(u64, u32)> {
    let input = Fraction::new(n, s);
    assert_eq!(
        input.is_ok(),
        arithmetic_raw_admitted(n, s),
        "complement factory completeness"
    );
    let Ok(input) = input else {
        return None;
    };
    let denominator = if 1 <= input.shift && input.shift <= 53 {
        Some(1_u128 << input.shift)
    } else {
        None
    };
    let required_success = match denominator {
        Some(value) => 2 * u128::from(input.numerator) >= value,
        None => false,
    };
    let observed = input.complement();
    assert_eq!(
        observed.is_ok(),
        required_success,
        "complement required success"
    );
    assert_eq!(
        matches!(observed, Err(FixtureEnclosureError::Invariant)),
        !required_success,
        "complement rejection classification"
    );
    if let Ok(value) = observed {
        // An erroneous success outside this domain is already rejected above;
        // keeping the power guarded makes the independent oracle total.
        if let Some(denominator) = denominator {
            assert_eq!(
                u128::from(value.numerator) + u128::from(input.numerator),
                denominator,
                "complement exact numerator relation"
            );
            assert!(
                u128::from(value.numerator) < denominator,
                "complement proper output"
            );
        }
        assert_eq!(value.shift, input.shift, "complement shift retained");
        assert!(
            1 <= value.numerator && value.numerator <= HALF_NUMERATOR,
            "complement positive output bound"
        );
        Some((value.numerator, value.shift))
    } else {
        None
    }
}

/// Typed support for the reviewed global bound f <= 1-2^-53. At small
/// denominators the exact numerator at denominator 2^53 is at most 2^53-1;
/// at larger denominators the same numerator bound suffices mathematically.
fn addition_global_fraction_maximum(value: Fraction) {
    if value.shift <= 53 {
        let scaled = u128::from(value.numerator) << (53 - value.shift);
        assert!(
            scaled <= u128::from(MAX_NUMERATOR),
            "addition global fraction maximum"
        );
    } else {
        assert!(
            value.numerator <= MAX_NUMERATOR,
            "addition global fraction maximum"
        );
    }
}

/// Independent exact common-denominator oracle, with a reviewed large-M branch.
/// Every shift here is read from the actual canonical constructed fraction.
fn addition_rational_oracle(left: Fraction, right: Fraction) -> (bool, bool) {
    let maximum = left.shift.max(right.shift);
    if maximum < 128 {
        let denominator = 1_u128 << maximum;
        let left_gap = maximum - left.shift;
        let right_gap = maximum - right.shift;
        let left_scaled = u128::from(left.numerator) << left_gap;
        let right_scaled = u128::from(right.numerator) << right_gap;
        assert!(
            left_scaled >> left_gap == u128::from(left.numerator)
                && right_scaled >> right_gap == u128::from(right.numerator),
            "addition oracle alignment exact"
        );
        assert!(
            left_scaled < denominator && right_scaled < denominator,
            "addition oracle proper numerator bounds"
        );
        // Both values are below 2^M, M<=127, so even their sum fits u128.
        // This explicit check precedes the ordinary addition and its safety check.
        assert!(
            left_scaled <= u128::MAX - right_scaled,
            "addition oracle sum representable"
        );
        let sum = left_scaled + right_scaled;
        (sum >= denominator, sum != 0 && sum != denominator)
    } else {
        let high = if left.shift >= 128 { left } else { right };
        assert!(
            high.numerator > 0 && 128 <= high.shift && high.shift <= 1041,
            "addition high denominator positive"
        );
        assert!(
            high.numerator < (1_u64 << 53),
            "addition high numerator bound"
        );
        addition_global_fraction_maximum(left);
        addition_global_fraction_maximum(right);
        // Trusted reviewed specification correspondence: the positive high term
        // is <2^53/2^128 = 2^-75, while every other checked fraction is at most
        // 1-2^-53. Thus 0<sum<1. No oversized power or callee oracle is used.
        (false, true)
    }
}

fn addition_contract(n: u64, s: u32, m: u64, t: u32) -> Option<(bool, bool)> {
    let left = Fraction::new(n, s);
    let right = Fraction::new(m, t);
    assert_eq!(
        left.is_ok(),
        arithmetic_raw_admitted(n, s),
        "addition left factory completeness"
    );
    assert_eq!(
        right.is_ok(),
        arithmetic_raw_admitted(m, t),
        "addition right factory completeness"
    );
    match (left, right) {
        (Ok(left), Ok(right)) => {
            let expected = addition_rational_oracle(left, right);
            let observed = add_fractions(left, right);
            assert!(observed.is_ok(), "addition required success");
            if let Ok(value) = observed {
                assert_eq!(value.carry, expected.0, "addition exact carry");
                assert_eq!(value.residual, expected.1, "addition exact residual");
                Some((value.carry, value.residual))
            } else {
                None
            }
        }
        _ => None,
    }
}

#[kani::proof]
fn fraction_complement_full_domain() {
    let n = kani::any::<u64>();
    let s = kani::any::<u32>();
    let inputs = (n, s);
    let observed = complement_contract(n, s);
    kani::cover!(
        inputs == (0, 0) && observed.is_none(),
        "complement zero rejected"
    );
    kani::cover!(
        inputs == (1, 2) && observed.is_none(),
        "complement quarter rejected"
    );
    kani::cover!(
        inputs == (HALF_NUMERATOR - 1, 53) && observed.is_none(),
        "complement below half neighbor rejected"
    );
    kani::cover!(
        inputs == (MAX_NUMERATOR, 54) && observed.is_none(),
        "complement wide below half rejected"
    );
    kani::cover!(
        inputs == (1, 1041) && observed.is_none(),
        "complement minimum positive rejected"
    );
    kani::cover!(
        inputs == (MAX_NUMERATOR, 1041) && observed.is_none(),
        "complement maximum shift numerator rejected"
    );
    kani::cover!(
        inputs == (1, 1) && observed == Some((1, 1)),
        "complement half succeeds"
    );
    kani::cover!(
        inputs == (2, 2) && observed == Some((2, 2)),
        "complement unreduced half succeeds"
    );
    kani::cover!(
        inputs == (HALF_NUMERATOR, 53) && observed == Some((HALF_NUMERATOR, 53)),
        "complement maximum half representation succeeds"
    );
    kani::cover!(
        inputs == (HALF_NUMERATOR + 1, 53) && observed == Some((HALF_NUMERATOR - 1, 53)),
        "complement above half neighbor succeeds"
    );
    kani::cover!(
        inputs == (MAX_NUMERATOR, 53) && observed == Some((1, 53)),
        "complement near one succeeds"
    );
    kani::cover!(
        inputs == (3, 2) && observed == Some((1, 2)),
        "complement three quarters succeeds"
    );
    kani::cover!(
        inputs == (5, 3) && observed == Some((3, 3)),
        "complement five eighths succeeds"
    );
}

#[kani::proof]
fn fraction_addition_full_domain() {
    let n = kani::any::<u64>();
    let s = kani::any::<u32>();
    let m = kani::any::<u64>();
    let t = kani::any::<u32>();
    let inputs = (n, s, m, t);
    let observed = addition_contract(n, s, m, t);
    kani::cover!(
        inputs == (0, 0, 0, 0) && observed == Some((false, false)),
        "addition zero sum"
    );
    kani::cover!(
        inputs == (1, 2, 0, 0) && observed == Some((false, true)),
        "addition quarter plus zero"
    );
    kani::cover!(
        inputs == (0, 0, 1, 2) && observed == Some((false, true)),
        "addition zero plus quarter"
    );
    kani::cover!(
        inputs == (1, 2, 1, 2) && observed == Some((false, true)),
        "addition two quarters"
    );
    kani::cover!(
        inputs == (1, 1, 1, 1) && observed == Some((true, false)),
        "addition exact halves"
    );
    kani::cover!(
        inputs == (1, 2, 3, 2) && observed == Some((true, false)),
        "addition quarter plus three quarters"
    );
    kani::cover!(
        inputs == (3, 2, 1, 2) && observed == Some((true, false)),
        "addition three quarters plus quarter"
    );
    kani::cover!(
        inputs == (3, 2, 1, 1) && observed == Some((true, true)),
        "addition three quarters plus half"
    );
    kani::cover!(
        inputs == (1, 1, 3, 2) && observed == Some((true, true)),
        "addition half plus three quarters"
    );
    kani::cover!(
        inputs == (2, 2, 2, 2) && observed == Some((true, false)),
        "addition unreduced halves"
    );
    kani::cover!(
        inputs == (1, 1, HALF_NUMERATOR - 1, 53) && observed == Some((false, true)),
        "addition just below unit"
    );
    kani::cover!(
        inputs == (1, 1, HALF_NUMERATOR + 1, 53) && observed == Some((true, true)),
        "addition just above unit"
    );
    kani::cover!(
        inputs == (MAX_NUMERATOR, 53, MAX_NUMERATOR, 53) && observed == Some((true, true)),
        "addition maximum numerator sum"
    );
    kani::cover!(
        inputs == (MAX_NUMERATOR, 53, MAX_NUMERATOR, 127) && observed == Some((false, true)),
        "addition u128 edge left"
    );
    kani::cover!(
        inputs == (MAX_NUMERATOR, 127, MAX_NUMERATOR, 53) && observed == Some((false, true)),
        "addition u128 edge right"
    );
    kani::cover!(
        inputs == (MAX_NUMERATOR, 53, MAX_NUMERATOR, 128) && observed == Some((false, true)),
        "addition high denominator left"
    );
    kani::cover!(
        inputs == (MAX_NUMERATOR, 128, MAX_NUMERATOR, 53) && observed == Some((false, true)),
        "addition high denominator right"
    );
    kani::cover!(
        inputs == (1, 1041, 0, 0) && observed == Some((false, true)),
        "addition minimum positive plus zero"
    );
    kani::cover!(
        inputs == (0, 0, 1, 1041) && observed == Some((false, true)),
        "addition zero plus minimum positive"
    );
    kani::cover!(
        inputs == (1, 1041, 1, 1041) && observed == Some((false, true)),
        "addition two minimum positives"
    );
    kani::cover!(
        inputs == (0, 1041, 0, 1041) && observed == Some((false, false)),
        "addition raw zeros canonicalize"
    );
    kani::cover!(
        inputs == (1, 1, 1, 2) && observed == Some((false, true)),
        "addition half plus quarter"
    );
    kani::cover!(
        inputs == (1, 2, 1, 1) && observed == Some((false, true)),
        "addition quarter plus half"
    );
}

#[kani::proof]
fn fraction_complement_control_success() {
    match Fraction::new(1, 1) {
        Ok(value) => assert!(value.complement().is_ok(), "complement required success"),
        Err(_) => assert!(false, "arithmetic control factory setup"),
    }
}

#[kani::proof]
fn fraction_complement_control_numerator() {
    match Fraction::new(5, 3) {
        Ok(value) => assert!(
            matches!(value.complement(), Ok(result) if u128::from(result.numerator) + 5 == 8 && result.shift == 3),
            "complement exact numerator relation"
        ),
        Err(_) => assert!(false, "arithmetic control factory setup"),
    }
}

#[kani::proof]
fn fraction_complement_control_error() {
    match Fraction::new(1, 2) {
        Ok(value) => assert!(
            matches!(value.complement(), Err(FixtureEnclosureError::Invariant)),
            "complement rejection classification"
        ),
        Err(_) => assert!(false, "arithmetic control factory setup"),
    }
}

#[kani::proof]
fn fraction_addition_control_success() {
    match (Fraction::new(0, 0), Fraction::new(0, 0)) {
        (Ok(left), Ok(right)) => assert!(
            add_fractions(left, right).is_ok(),
            "addition required success"
        ),
        _ => assert!(false, "arithmetic control factory setup"),
    }
}

#[kani::proof]
fn fraction_addition_control_carry() {
    match (Fraction::new(1, 1), Fraction::new(1, 1)) {
        (Ok(left), Ok(right)) => match add_fractions(left, right) {
            Ok(value) => assert!(value.carry, "addition exact carry"),
            Err(_) => assert!(false, "arithmetic control operation setup"),
        },
        _ => assert!(false, "arithmetic control factory setup"),
    }
}

#[kani::proof]
fn fraction_addition_control_residual() {
    match (Fraction::new(1, 1), Fraction::new(1, 2)) {
        (Ok(left), Ok(right)) => match add_fractions(left, right) {
            Ok(value) => assert!(value.residual, "addition exact residual"),
            Err(_) => assert!(false, "arithmetic control operation setup"),
        },
        _ => assert!(false, "arithmetic control factory setup"),
    }
}

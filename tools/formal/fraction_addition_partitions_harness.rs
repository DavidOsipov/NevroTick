// Six explicit raw-domain guards. Membership is the sole leaf assumption.
// Actual Fraction constructors and arithmetic remain unchanged and inline.

fn partition_raw_admitted(numerator: u64, shift: u32) -> bool {
    let bits = 64 - numerator.leading_zeros();
    shift <= 1041 && bits <= 53 && (numerator == 0 || bits <= shift)
}

fn partition_large(numerator: u64, canonical_shift: u32) -> bool {
    1 <= canonical_shift
        && canonical_shift <= 53
        && 2 * u128::from(numerator) >= (1_u128 << canonical_shift)
}

fn partition_guards(n: u64, s: u32, m: u64, t: u32) -> [bool; 6] {
    // Raw admission precedes classification-only normalization.
    if !partition_raw_admitted(n, s) || !partition_raw_admitted(m, t) {
        return [true, false, false, false, false, false];
    }
    let left_shift = if n == 0 { 0 } else { s };
    let right_shift = if m == 0 { 0 } else { t };
    let left_large = partition_large(n, left_shift);
    let right_large = partition_large(m, right_shift);
    [
        false,
        !left_large && !right_large,
        left_large && right_shift >= 106,
        left_large && right_shift < 106,
        !left_large && right_large && left_shift >= 106,
        !left_large && right_large && left_shift < 106,
    ]
}

fn partition_raw_inputs() -> (u64, u32, u64, u32) {
    (
        kani::any::<u64>(),
        kani::any::<u32>(),
        kani::any::<u64>(),
        kani::any::<u32>(),
    )
}

#[kani::proof]
fn fraction_addition_partition_union() {
    let (n, s, m, t) = partition_raw_inputs();
    let inputs = (n, s, m, t);
    let guards = partition_guards(n, s, m, t);
    let count = u8::from(guards[0])
        + u8::from(guards[1])
        + u8::from(guards[2])
        + u8::from(guards[3])
        + u8::from(guards[4])
        + u8::from(guards[5]);
    assert_eq!(count, 1, "addition partition exactly one guard");
    let left = Fraction::new(n, s);
    let right = Fraction::new(m, t);
    assert_eq!(
        guards[0],
        left.is_err() || right.is_err(),
        "addition partition raw rejection equivalence"
    );
    assert_eq!(
        guards[1] || guards[2] || guards[3] || guards[4] || guards[5],
        left.is_ok() && right.is_ok(),
        "addition partition raw success equivalence"
    );
    if let Ok(value) = left {
        assert_eq!(value.numerator, n, "addition partition left numerator exact");
        assert_eq!(
            value.shift,
            if n == 0 { 0 } else { s },
            "addition partition left canonical shift exact"
        );
    }
    if let Ok(value) = right {
        assert_eq!(value.numerator, m, "addition partition right numerator exact");
        assert_eq!(
            value.shift,
            if m == 0 { 0 } else { t },
            "addition partition right canonical shift exact"
        );
    }
    kani::cover!(
        inputs == (1, 0, 0, 0) && guards[0] && left.is_err() && right.is_ok(),
        "addition partition guard 0 reachable"
    );
    kani::cover!(
        inputs == (1, 2, 0, 0) && guards[1] && left.is_ok() && right.is_ok(),
        "addition partition guard 1 reachable"
    );
    kani::cover!(
        inputs == (1, 1, 1, 1041) && guards[2] && left.is_ok() && right.is_ok(),
        "addition partition guard 2 reachable"
    );
    kani::cover!(
        inputs == (1, 1, 1, 1) && guards[3] && left.is_ok() && right.is_ok(),
        "addition partition guard 3 reachable"
    );
    kani::cover!(
        inputs == (1, 1041, 1, 1) && guards[4] && left.is_ok() && right.is_ok(),
        "addition partition guard 4 reachable"
    );
    kani::cover!(
        inputs == (1, 2, 1, 1) && guards[5] && left.is_ok() && right.is_ok(),
        "addition partition guard 5 reachable"
    );
    kani::cover!(
        inputs == (1, 0, 0, 0) && left.is_err() && right.is_ok(),
        "addition partition raw left rejection neighbor"
    );
    kani::cover!(
        inputs == (1, 1, 0, 0) && left.is_ok() && right.is_ok(),
        "addition partition raw left acceptance neighbor"
    );
    kani::cover!(
        inputs == (0, 0, 1, 0) && left.is_ok() && right.is_err(),
        "addition partition raw right rejection neighbor"
    );
    kani::cover!(
        inputs == (0, 0, 1, 1) && left.is_ok() && right.is_ok(),
        "addition partition raw right acceptance neighbor"
    );
}

#[kani::proof]
fn fraction_addition_partition_0() {
    let (n, s, m, t) = partition_raw_inputs();
    let inputs = (n, s, m, t);
    let guards = partition_guards(n, s, m, t);
    kani::assume(guards[0]);
    let observed = addition_contract(n, s, m, t);
    kani::cover!(
        inputs == (1, 0, 0, 0) && observed.is_none(),
        "addition partition invalid left factory"
    );
    kani::cover!(
        inputs == (0, 0, 1, 0) && observed.is_none(),
        "addition partition invalid right factory"
    );
}

#[kani::proof]
fn fraction_addition_partition_1() {
    let (n, s, m, t) = partition_raw_inputs();
    let inputs = (n, s, m, t);
    let guards = partition_guards(n, s, m, t);
    kani::assume(guards[1]);
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
}

#[kani::proof]
fn fraction_addition_partition_2() {
    let (n, s, m, t) = partition_raw_inputs();
    let inputs = (n, s, m, t);
    let guards = partition_guards(n, s, m, t);
    kani::assume(guards[2]);
    let observed = addition_contract(n, s, m, t);
    kani::cover!(
        inputs == (MAX_NUMERATOR, 53, MAX_NUMERATOR, 127) && observed == Some((false, true)),
        "addition u128 edge left"
    );
    kani::cover!(
        inputs == (MAX_NUMERATOR, 53, MAX_NUMERATOR, 128) && observed == Some((false, true)),
        "addition high denominator left"
    );
}

#[kani::proof]
fn fraction_addition_partition_3() {
    let (n, s, m, t) = partition_raw_inputs();
    let inputs = (n, s, m, t);
    let guards = partition_guards(n, s, m, t);
    kani::assume(guards[3]);
    let observed = addition_contract(n, s, m, t);
    kani::cover!(
        inputs == (1, 1, 1, 1) && observed == Some((true, false)),
        "addition exact halves"
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
        inputs == (1, 1, 1, 2) && observed == Some((false, true)),
        "addition half plus quarter"
    );
}

#[kani::proof]
fn fraction_addition_partition_4() {
    let (n, s, m, t) = partition_raw_inputs();
    let inputs = (n, s, m, t);
    let guards = partition_guards(n, s, m, t);
    kani::assume(guards[4]);
    let observed = addition_contract(n, s, m, t);
    kani::cover!(
        inputs == (MAX_NUMERATOR, 127, MAX_NUMERATOR, 53) && observed == Some((false, true)),
        "addition u128 edge right"
    );
    kani::cover!(
        inputs == (MAX_NUMERATOR, 128, MAX_NUMERATOR, 53) && observed == Some((false, true)),
        "addition high denominator right"
    );
}

#[kani::proof]
fn fraction_addition_partition_5() {
    let (n, s, m, t) = partition_raw_inputs();
    let inputs = (n, s, m, t);
    let guards = partition_guards(n, s, m, t);
    kani::assume(guards[5]);
    let observed = addition_contract(n, s, m, t);
    kani::cover!(
        inputs == (1, 2, 3, 2) && observed == Some((true, false)),
        "addition quarter plus three quarters"
    );
    kani::cover!(
        inputs == (1, 2, 1, 1) && observed == Some((false, true)),
        "addition quarter plus half"
    );
}

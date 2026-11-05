// Full checked-fraction comparison domain, reached through four raw inputs and
// the actual complete constructors. No assumptions or private field assembly.

const ALIGNMENT_NUMERATOR: u64 = 4_503_599_627_370_496;
const MAX_NUMERATOR: u64 = 9_007_199_254_740_991;

/// Independent bit-length admission contract already proved for actual new.
fn comparison_raw_admitted(numerator: u64, shift: u32) -> bool {
    let bits = 64 - numerator.leading_zeros();
    shift <= 1041 && bits <= 53 && (numerator == 0 || bits <= shift)
}

/// Independent exact rational ordering by bounded cross scaling, never by the
/// implementation's leading exponent. Inputs have actual constructor invariants.
fn comparison_rational_oracle(left: Fraction, right: Fraction) -> Ordering {
    if left.numerator == 0 {
        return if right.numerator == 0 {
            Ordering::Equal
        } else {
            Ordering::Less
        };
    }
    if right.numerator == 0 {
        return Ordering::Greater;
    }
    if left.shift == right.shift {
        return left.numerator.cmp(&right.numerator);
    }
    if left.shift < right.shift {
        let gap = right.shift - left.shift;
        assert!(gap <= 1040, "comparison oracle gap bound");
        if gap < 64 {
            let scaled = u128::from(left.numerator) << gap;
            assert!(scaled < (1_u128 << 116), "comparison oracle scaled width");
            scaled.cmp(&u128::from(right.numerator))
        } else {
            // Cross scaling a positive integer by at least 64 exceeds every
            // admitted numerator (at most 53 bits), without forming 2^gap.
            Ordering::Greater
        }
    } else {
        let gap = left.shift - right.shift;
        assert!(gap <= 1040, "comparison oracle gap bound");
        if gap < 64 {
            let scaled = u128::from(right.numerator) << gap;
            assert!(scaled < (1_u128 << 116), "comparison oracle scaled width");
            u128::from(left.numerator).cmp(&scaled)
        } else {
            Ordering::Less
        }
    }
}

/// Check source-equivalent guard formulas as machine lemmas, independently of
/// the rational oracle. This neither replaces nor instruments actual compare.
fn comparison_source_machine_bounds(left: Fraction, right: Fraction, exact: Ordering) {
    if left.numerator == 0 || right.numerator == 0 {
        return;
    }
    let bits = 64 - left.numerator.leading_zeros();
    let other_bits = 64 - right.numerator.leading_zeros();
    assert!(
        1 <= bits && bits <= 53 && 1 <= other_bits && other_bits <= 53,
        "comparison positive numerator bit bounds"
    );
    // These are exactly the source's i64 leading-exponent formulas. All
    // conversions, subtractions and shifts retain default machine checks.
    let leading = i64::from(bits) - i64::from(left.shift);
    let other_leading = i64::from(other_bits) - i64::from(right.shift);
    assert!(
        -1040 <= leading && leading <= 0 && -1040 <= other_leading && other_leading <= 0,
        "comparison leading exponent bounds"
    );
    if leading == other_leading {
        if left.shift < right.shift {
            let gap = right.shift - left.shift;
            assert_eq!(
                i64::from(gap),
                i64::from(other_bits) - i64::from(bits),
                "comparison equal power alignment relation"
            );
            assert!(gap <= 52, "comparison alignment count bound");
            let mathematical = u128::from(left.numerator) << gap;
            let machine = left.numerator.checked_shl(gap);
            assert!(
                mathematical < (1_u128 << 53) && machine.map(u128::from) == Some(mathematical),
                "comparison alignment retains numerator bits"
            );
        } else {
            let gap = left.shift - right.shift;
            assert_eq!(
                i64::from(gap),
                i64::from(bits) - i64::from(other_bits),
                "comparison equal power alignment relation"
            );
            assert!(gap <= 52, "comparison alignment count bound");
            let mathematical = u128::from(right.numerator) << gap;
            let machine = right.numerator.checked_shl(gap);
            assert!(
                mathematical < (1_u128 << 53) && machine.map(u128::from) == Some(mathematical),
                "comparison alignment retains numerator bits"
            );
        }
    } else {
        assert!(
            (leading < other_leading && exact == Ordering::Less)
                || (leading > other_leading && exact == Ordering::Greater),
            "comparison unequal powers strict order"
        );
    }
}

/// Constructor completeness prevents restricting the theorem by accidental
/// rejection. Its existing exact retention/surjectivity proof remains reused.
fn fraction_comparison_contract(n: u64, s: u32, m: u64, t: u32) -> Option<Ordering> {
    let left = Fraction::new(n, s);
    let right = Fraction::new(m, t);
    assert_eq!(
        left.is_ok(),
        comparison_raw_admitted(n, s),
        "comparison left factory completeness"
    );
    assert_eq!(
        right.is_ok(),
        comparison_raw_admitted(m, t),
        "comparison right factory completeness"
    );
    match (left, right) {
        (Ok(left), Ok(right)) => {
            let exact = comparison_rational_oracle(left, right);
            comparison_source_machine_bounds(left, right, exact);
            let observed = left.compare(right);
            assert!(observed.is_ok(), "comparison required success");
            if let Ok(order) = observed {
                assert_eq!(order, exact, "comparison exact rational ordering");
                Some(order)
            } else {
                None
            }
        }
        _ => None,
    }
}

#[kani::proof]
fn fraction_comparison_full_domain() {
    let n = kani::any::<u64>();
    let s = kani::any::<u32>();
    let m = kani::any::<u64>();
    let t = kani::any::<u32>();
    let inputs = (n, s, m, t);
    let observed = fraction_comparison_contract(n, s, m, t);
    kani::cover!(
        inputs == (0, 0, 0, 0) && observed == Some(Ordering::Equal),
        "comparison zero equality"
    );
    kani::cover!(
        inputs == (0, 0, 1, 1041) && observed == Some(Ordering::Less),
        "comparison zero below minimum positive"
    );
    kani::cover!(
        inputs == (1, 1041, 0, 0) && observed == Some(Ordering::Greater),
        "comparison minimum positive above zero"
    );
    kani::cover!(
        inputs == (1, 1, 1, 1) && observed == Some(Ordering::Equal),
        "comparison identical one half"
    );
    kani::cover!(
        inputs == (1, 2, 1, 1) && observed == Some(Ordering::Less),
        "comparison quarter below half"
    );
    kani::cover!(
        inputs == (1, 1, 1, 2) && observed == Some(Ordering::Greater),
        "comparison half above quarter"
    );
    kani::cover!(
        inputs == (2, 3, 3, 3) && observed == Some(Ordering::Less),
        "comparison same shift less"
    );
    kani::cover!(
        inputs == (3, 3, 2, 3) && observed == Some(Ordering::Greater),
        "comparison same shift greater"
    );
    kani::cover!(
        inputs == (1, 1, 2, 2) && observed == Some(Ordering::Equal),
        "comparison unreduced equality left"
    );
    kani::cover!(
        inputs == (2, 2, 1, 1) && observed == Some(Ordering::Equal),
        "comparison unreduced equality right"
    );
    kani::cover!(
        inputs == (1, 1, ALIGNMENT_NUMERATOR, 53) && observed == Some(Ordering::Equal),
        "comparison maximum alignment left"
    );
    kani::cover!(
        inputs == (ALIGNMENT_NUMERATOR, 53, 1, 1) && observed == Some(Ordering::Equal),
        "comparison maximum alignment right"
    );
    kani::cover!(
        inputs == (1, 1, ALIGNMENT_NUMERATOR + 1, 53) && observed == Some(Ordering::Less),
        "comparison maximum alignment less"
    );
    kani::cover!(
        inputs == (ALIGNMENT_NUMERATOR + 1, 53, 1, 1) && observed == Some(Ordering::Greater),
        "comparison maximum alignment greater"
    );
    kani::cover!(
        inputs == (ALIGNMENT_NUMERATOR, 53, ALIGNMENT_NUMERATOR, 116)
            && observed == Some(Ordering::Greater),
        "comparison gap63 greater"
    );
    kani::cover!(
        inputs == (ALIGNMENT_NUMERATOR, 116, ALIGNMENT_NUMERATOR, 53)
            && observed == Some(Ordering::Less),
        "comparison gap63 less"
    );
    kani::cover!(
        inputs == (ALIGNMENT_NUMERATOR, 53, ALIGNMENT_NUMERATOR, 117)
            && observed == Some(Ordering::Greater),
        "comparison gap64 greater"
    );
    kani::cover!(
        inputs == (ALIGNMENT_NUMERATOR, 117, ALIGNMENT_NUMERATOR, 53)
            && observed == Some(Ordering::Less),
        "comparison gap64 less"
    );
    kani::cover!(
        inputs == (MAX_NUMERATOR, 1041, MAX_NUMERATOR, 53) && observed == Some(Ordering::Less),
        "comparison maximum numerator wide gap"
    );
    kani::cover!(
        inputs == (1, 1, 1, 1041) && observed == Some(Ordering::Greater),
        "comparison maximum shift gap"
    );
}

/// Ordinary identical checked values must compare successfully.
#[kani::proof]
fn fraction_comparison_control_success() {
    match (Fraction::new(1, 1), Fraction::new(1, 1)) {
        (Ok(left), Ok(right)) => {
            assert!(left.compare(right).is_ok(), "comparison required success")
        }
        _ => assert!(false, "comparison control factory setup"),
    }
}

/// Reversing unequal leading powers preserves Ordering's codomain but loses Less.
#[kani::proof]
fn fraction_comparison_control_unequal() {
    match (Fraction::new(1, 2), Fraction::new(1, 1)) {
        (Ok(left), Ok(right)) => assert!(
            matches!(left.compare(right), Ok(Ordering::Less)),
            "comparison strict unequal order"
        ),
        _ => assert!(false, "comparison control factory setup"),
    }
}

/// Distinct unreduced representations must remain exactly equal.
#[kani::proof]
fn fraction_comparison_control_alignment() {
    match (Fraction::new(1, 1), Fraction::new(2, 2)) {
        (Ok(left), Ok(right)) => assert!(
            matches!(left.compare(right), Ok(Ordering::Equal)),
            "comparison unreduced equality"
        ),
        _ => assert!(false, "comparison control factory setup"),
    }
}

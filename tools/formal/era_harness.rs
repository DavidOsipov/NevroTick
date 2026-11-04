//! Full admitted-window resolver theorem with a modular-distance oracle.

/// Every valid 96-bit signed coordinate has one high-i32/low-u64 representation.
fn endpoint() -> i128 {
    (i128::from(kani::any::<i32>()) << 64) + i128::from(kani::any::<u64>())
}

/// Check all three outcomes using the least matching residue at or above lower.
/// The oracle uses modular distance, not the subject's signed division formula.
fn resolver_contract(lower: i128, upper: i128, seconds: u32, fraction: u32) {
    let window = era::proof_window(lower, upper);
    let observed = era::unfold_ntp(seconds, fraction, &window).map(|value| value.ticks());
    let raw = (u64::from(seconds) << 32) | u64::from(fraction);
    // This cast deliberately retains the endpoint's low 64 bits, including
    // negative endpoints. Wrapping subtraction gives a distance in [0,2^64).
    let distance = raw.wrapping_sub(lower as u64);
    let first = lower + i128::from(distance);
    let next = first + (1_i128 << 64);
    if first > upper {
        assert_eq!(
            observed,
            Err(DomainError::NoEraCandidate),
            "absence classification"
        );
    } else if next <= upper {
        assert_eq!(
            observed,
            Err(DomainError::AmbiguousEra),
            "ambiguity classification"
        );
    } else {
        let ticks = match observed {
            Ok(value) => value,
            Err(_) => {
                assert!(false, "required unique success");
                return;
            }
        };
        assert!(
            ticks == first && lower <= ticks && ticks <= upper && ticks as u64 == raw,
            "exact unique result"
        );
    }
}

#[kani::proof]
fn era_full_domain() {
    let lower = endpoint();
    let upper = endpoint();
    // The sole assumption is the immutable EraWindow ordering invariant.
    kani::assume(lower <= upper);
    let seconds = kani::any();
    let fraction = kani::any();
    resolver_contract(lower, upper, seconds, fraction);
    kani::cover!(
        lower == 0 && upper == 0 && seconds == 0 && fraction == 0,
        "unique point witness"
    );
    kani::cover!(
        lower == 1 && upper == (1_i128 << 64) - 1 && seconds == 0 && fraction == 0,
        "absent witness"
    );
    kani::cover!(
        lower == -(1_i128 << 64) && upper == 0 && seconds == 0 && fraction == 0,
        "multiple-era witness"
    );
}

#[kani::proof]
fn era_control_unique() {
    resolver_contract(0, 0, 0, 0);
}

#[kani::proof]
fn era_control_coordinate() {
    resolver_contract(0, 100, 0, 0);
}

#[kani::proof]
fn era_control_absent() {
    resolver_contract(1, (1_i128 << 64) - 1, 0, 0);
}

// Full raw (u64 numerator, u32 shift) contract of the actual Fraction::new.
// Leading-zero bit length specifies admission without the constructor's shifts.
// No constructor precondition or reachable-caller assumption narrows the domain.

const MAX_PROPER_NUMERATOR: u64 = 9_007_199_254_740_991;

/// Check required success, exact retained representation and complete errors.
fn fraction_constructor_contract(
    numerator: u64,
    shift: u32,
) -> Result<Fraction, FixtureEnclosureError> {
    let observed = Fraction::new(numerator, shift);
    let bits = 64 - numerator.leading_zeros();
    let required_success = shift <= 1041 && bits <= 53 && (numerator == 0 || bits <= shift);
    assert_eq!(
        observed.is_ok(),
        required_success,
        "fraction required success"
    );
    assert_eq!(
        matches!(observed, Err(FixtureEnclosureError::Invariant)),
        !required_success,
        "fraction rejection classification"
    );
    if let Ok(value) = observed {
        assert_eq!(
            value.numerator, numerator,
            "fraction exact numerator retained"
        );
        if numerator == 0 {
            assert!(
                value.numerator == 0 && value.shift == 0,
                "fraction zero canonicalization"
            );
        } else {
            assert_eq!(value.shift, shift, "fraction nonzero shift retained");
        }
        let output_bits = 64 - value.numerator.leading_zeros();
        assert!(output_bits <= 53, "fraction output numerator bound");
        assert!(value.shift <= 1041, "fraction output shift bound");
        assert!(
            value.numerator == 0 || (value.shift >= 1 && output_bits <= value.shift),
            "fraction output proper dyadic"
        );
    }
    observed
}

#[kani::proof]
fn fraction_constructor_full_domain() {
    let numerator = kani::any::<u64>();
    let shift = kani::any::<u32>();
    let observed = fraction_constructor_contract(numerator, shift);
    kani::cover!(
        numerator == 0
            && shift == 0
            && matches!(
                observed,
                Ok(Fraction {
                    numerator: 0,
                    shift: 0
                })
            ),
        "fraction zero at shift zero is canonical"
    );
    kani::cover!(
        numerator == 0
            && shift == 1041
            && matches!(
                observed,
                Ok(Fraction {
                    numerator: 0,
                    shift: 0
                })
            ),
        "fraction zero at maximum shift is canonical"
    );
    kani::cover!(
        numerator == 1 && shift == 1 && observed.is_ok(),
        "fraction one half succeeds"
    );
    kani::cover!(
        numerator == MAX_PROPER_NUMERATOR && shift == 53 && observed.is_ok(),
        "fraction maximum numerator at shift 53 succeeds"
    );
    kani::cover!(
        numerator == 1 && shift == 1041 && observed.is_ok(),
        "fraction unit numerator at maximum shift succeeds"
    );
    kani::cover!(
        numerator == 0
            && shift == 1042
            && matches!(observed, Err(FixtureEnclosureError::Invariant)),
        "fraction zero beyond maximum shift is rejected"
    );
    kani::cover!(
        numerator == 0
            && shift == u32::MAX
            && matches!(observed, Err(FixtureEnclosureError::Invariant)),
        "fraction zero at raw maximum shift is rejected"
    );
    kani::cover!(
        numerator == 9_007_199_254_740_992
            && shift == 54
            && matches!(observed, Err(FixtureEnclosureError::Invariant)),
        "fraction oversized proper numerator is rejected"
    );
    kani::cover!(
        numerator == u64::MAX
            && shift == 1041
            && matches!(observed, Err(FixtureEnclosureError::Invariant)),
        "fraction raw maximum numerator is rejected"
    );
    kani::cover!(
        numerator == 1 && shift == 0 && matches!(observed, Err(FixtureEnclosureError::Invariant)),
        "fraction nonzero numerator at zero shift is rejected"
    );
    kani::cover!(
        numerator == 4 && shift == 2 && matches!(observed, Err(FixtureEnclosureError::Invariant)),
        "fraction unit boundary is rejected"
    );
    kani::cover!(
        numerator == 3 && shift == 2 && observed.is_ok(),
        "fraction proper neighbor succeeds"
    );
    kani::cover!(
        numerator == MAX_PROPER_NUMERATOR && shift == 63 && observed.is_ok(),
        "fraction maximum numerator at shift 63 succeeds"
    );
    kani::cover!(
        numerator == MAX_PROPER_NUMERATOR && shift == 64 && observed.is_ok(),
        "fraction maximum numerator at shift 64 succeeds"
    );
}

/// Reject-all loses the ordinary proper dyadic one half.
#[kani::proof]
fn fraction_constructor_control_success() {
    let observed = Fraction::new(1, 1);
    assert!(observed.is_ok(), "fraction required success");
}

/// Decrementing three to two preserves validity but changes the supplied value.
#[kani::proof]
fn fraction_constructor_control_numerator() {
    let observed = Fraction::new(3, 2);
    assert!(
        matches!(
            observed,
            Ok(Fraction {
                numerator: 3,
                shift: 2
            })
        ),
        "fraction exact numerator retained"
    );
}

/// The shift limit precedes zero canonicalization in the actual raw contract.
#[kani::proof]
fn fraction_constructor_control_error() {
    let observed = Fraction::new(0, 1042);
    assert!(
        matches!(observed, Err(FixtureEnclosureError::Invariant)),
        "fraction rejection classification"
    );
}

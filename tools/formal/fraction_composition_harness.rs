// Proof-only models and controls, pasted lexically into proof.rs by preparation.
// All replacement target paths are generated as fully qualified unique paths.

fn composition_model_new(numerator: u64, shift: u32) -> Result<Fraction, FixtureEnclosureError> {
    // Constructor has the complete raw domain: no input assumption/precondition.
    let seed = CompositionFractionSeed {
        tag: kani::any::<u8>(),
        numerator: kani::any::<u64>(),
        shift: kani::any::<u32>(),
    };
    let result = composition_decode_fraction(seed);
    kani::assume(composition_constructor_post(numerator, shift, &result));
    result
}

fn composition_model_compare(
    left: Fraction,
    right: Fraction,
) -> Result<Ordering, FixtureEnclosureError> {
    kani::assert(
        composition_checked(left) && composition_checked(right),
        "composition comparison checked precondition",
    );
    let result = composition_decode_order(kani::any::<u8>());
    kani::assume(composition_comparison_post(left, right, &result));
    result
}

fn composition_model_complement(input: Fraction) -> Result<Fraction, FixtureEnclosureError> {
    kani::assert(
        composition_checked(input),
        "composition complement checked precondition",
    );
    let seed = CompositionFractionSeed {
        tag: kani::any::<u8>(),
        numerator: kani::any::<u64>(),
        shift: kani::any::<u32>(),
    };
    let result = composition_decode_fraction(seed);
    kani::assume(composition_complement_post(input, &result));
    result
}

fn composition_activation_new(
    _numerator: u64,
    _shift: u32,
) -> Result<Fraction, FixtureEnclosureError> {
    Err(FixtureEnclosureError::Unrepresentable)
}
fn composition_activation_compare(
    _left: Fraction,
    _right: Fraction,
) -> Result<Ordering, FixtureEnclosureError> {
    Ok(Ordering::Greater)
}
fn composition_activation_complement(_input: Fraction) -> Result<Fraction, FixtureEnclosureError> {
    Err(FixtureEnclosureError::Interval {
        source: IntervalError::ValidationInvariant,
    })
}

#[kani::proof]
fn composition_mechanism_full_domain() {
    let activated_new = matches!(
        Fraction::new(3, 2),
        Err(FixtureEnclosureError::Unrepresentable)
    );
    let activated_compare = matches!(
        Fraction {
            numerator: 1,
            shift: 2
        }
        .compare(Fraction {
            numerator: 1,
            shift: 1
        }),
        Ok(Ordering::Greater)
    );
    let activated_complement = matches!(
        Fraction {
            numerator: 1,
            shift: 2
        }
        .complement(),
        Err(FixtureEnclosureError::Interval {
            source: IntervalError::ValidationInvariant
        })
    );
    assert!(activated_new, "composition constructor replacement active");
    assert!(
        activated_compare,
        "composition comparison replacement active"
    );
    assert!(
        activated_complement,
        "composition complement replacement active"
    );
    kani::cover!(
        activated_new && activated_compare && activated_complement,
        "composition all three replacements reached"
    );

    let seed = CompositionFractionSeed {
        tag: kani::any::<u8>(),
        numerator: kani::any::<u64>(),
        shift: kani::any::<u32>(),
    };
    let raw_fraction = composition_decode_fraction(seed);
    assert!(
        composition_fraction_result_equal(
            raw_fraction,
            composition_decode_fraction(composition_encode_fraction(raw_fraction))
        ),
        "composition complete fraction result codec"
    );
    kani::cover!(
        matches!(raw_fraction, Ok(_)),
        "composition codec fraction Ok"
    );
    kani::cover!(
        matches!(raw_fraction, Err(FixtureEnclosureError::Unrepresentable)),
        "composition codec fraction Unrepresentable"
    );
    kani::cover!(
        matches!(raw_fraction, Err(FixtureEnclosureError::Invariant)),
        "composition codec fraction Invariant"
    );
    kani::cover!(
        matches!(
            raw_fraction,
            Err(FixtureEnclosureError::Interval {
                source: IntervalError::ZeroLabel
            })
        ),
        "composition codec fraction ZeroLabel"
    );
    kani::cover!(
        matches!(
            raw_fraction,
            Err(FixtureEnclosureError::Interval {
                source: IntervalError::ReversedBounds
            })
        ),
        "composition codec fraction ReversedBounds"
    );
    kani::cover!(
        matches!(
            raw_fraction,
            Err(FixtureEnclosureError::Interval {
                source: IntervalError::ValidationInvariant
            })
        ),
        "composition codec fraction ValidationInvariant"
    );
    let raw_order = composition_decode_order(kani::any::<u8>());
    assert!(
        composition_order_result_equal(
            raw_order,
            composition_decode_order(composition_encode_order(raw_order))
        ),
        "composition complete native Ordering result codec"
    );
    kani::cover!(
        matches!(raw_order, Ok(Ordering::Less)),
        "composition codec Ordering Less"
    );
    kani::cover!(
        matches!(raw_order, Ok(Ordering::Equal)),
        "composition codec Ordering Equal"
    );
    kani::cover!(
        matches!(raw_order, Ok(Ordering::Greater)),
        "composition codec Ordering Greater"
    );
    kani::cover!(
        matches!(raw_order, Err(FixtureEnclosureError::Unrepresentable)),
        "composition codec Ordering Unrepresentable"
    );
    kani::cover!(
        matches!(raw_order, Err(FixtureEnclosureError::Invariant)),
        "composition codec Ordering Invariant"
    );
    kani::cover!(
        matches!(
            raw_order,
            Err(FixtureEnclosureError::Interval {
                source: IntervalError::ZeroLabel
            })
        ),
        "composition codec Ordering ZeroLabel"
    );
    kani::cover!(
        matches!(
            raw_order,
            Err(FixtureEnclosureError::Interval {
                source: IntervalError::ReversedBounds
            })
        ),
        "composition codec Ordering ReversedBounds"
    );
    kani::cover!(
        matches!(
            raw_order,
            Err(FixtureEnclosureError::Interval {
                source: IntervalError::ValidationInvariant
            })
        ),
        "composition codec Ordering ValidationInvariant"
    );

    // Unrestricted raw input bits and unrestricted output seeds challenge totality.
    let numerator = kani::any::<u64>();
    let shift = kani::any::<u32>();
    let raw_left = Fraction {
        numerator: kani::any::<u64>(),
        shift: kani::any::<u32>(),
    };
    let raw_right = Fraction {
        numerator: kani::any::<u64>(),
        shift: kani::any::<u32>(),
    };
    let new_post = composition_constructor_post(numerator, shift, &raw_fraction);
    let compare_post = composition_comparison_post(raw_left, raw_right, &raw_order);
    let complement_post = composition_complement_post(raw_left, &raw_fraction);
    assert!(
        !new_post || composition_constructor_admitted(numerator, shift) == raw_fraction.is_ok(),
        "composition constructor total predicate relation"
    );
    assert!(
        !compare_post
            || (composition_checked(raw_left)
                && composition_checked(raw_right)
                && raw_order.is_ok()),
        "composition comparison total predicate relation"
    );
    assert!(
        !complement_post || composition_checked(raw_left),
        "composition complement total predicate relation"
    );
    kani::cover!(new_post, "composition raw constructor Post true");
    kani::cover!(!new_post, "composition raw constructor Post false");
    kani::cover!(compare_post, "composition raw comparison Post true");
    kani::cover!(!compare_post, "composition raw comparison Post false");
    kani::cover!(complement_post, "composition raw complement Post true");
    kani::cover!(!complement_post, "composition raw complement Post false");

    let new_valid = composition_model_new(2, 2);
    let new_invalid = composition_model_new(0, 1042);
    let compare_valid = composition_model_compare(
        Fraction {
            numerator: 1,
            shift: 1,
        },
        Fraction {
            numerator: 2,
            shift: 2,
        },
    );
    let complement_valid = composition_model_complement(Fraction {
        numerator: 2,
        shift: 2,
    });
    let complement_reject = composition_model_complement(Fraction {
        numerator: 1,
        shift: 2,
    });
    let neighbors = matches!(
        new_valid,
        Ok(Fraction {
            numerator: 2,
            shift: 2
        })
    ) && matches!(new_invalid, Err(FixtureEnclosureError::Invariant))
        && matches!(compare_valid, Ok(Ordering::Equal))
        && matches!(
            complement_valid,
            Ok(Fraction {
                numerator: 2,
                shift: 2
            })
        )
        && matches!(complement_reject, Err(FixtureEnclosureError::Invariant));
    assert!(neighbors, "composition real model valid neighbors");
    kani::cover!(neighbors, "composition real model neighbors reached");
}

#[kani::proof]
fn composition_assert_then_assume() {
    let condition = kani::any::<bool>();
    kani::assert(
        condition,
        "composition earlier assertion survives false assumption",
    );
    kani::assume(condition);
}

#[kani::proof]
fn composition_constructor_bad_post() {
    let result = Fraction::new(3, 2);
    assert!(
        composition_constructor_post(3, 2, &result),
        "composition constructor actual postcondition"
    );
}
#[kani::proof]
fn composition_constructor_wrong_seed() {
    let result = Fraction::new(3, 2);
    assert!(
        composition_fraction_result_equal(
            result,
            composition_decode_fraction(composition_encode_fraction(result))
        ),
        "composition constructor actual return seed"
    );
}
#[kani::proof]
fn composition_comparison_bad_post() {
    if let (Ok(left), Ok(right)) = (Fraction::new(1, 2), Fraction::new(1, 1)) {
        let result = left.compare(right);
        assert!(
            composition_comparison_post(left, right, &result),
            "composition comparison actual postcondition"
        );
    }
}
#[kani::proof]
fn composition_comparison_wrong_seed() {
    if let (Ok(left), Ok(right)) = (Fraction::new(1, 2), Fraction::new(1, 1)) {
        let result = left.compare(right);
        assert!(
            composition_order_result_equal(
                result,
                composition_decode_order(composition_encode_order(result))
            ),
            "composition comparison actual return seed"
        );
    }
}
#[kani::proof]
fn composition_complement_bad_post() {
    if let Ok(input) = Fraction::new(5, 3) {
        let result = input.complement();
        assert!(
            composition_complement_post(input, &result),
            "composition complement actual postcondition"
        );
    }
}
#[kani::proof]
fn composition_complement_wrong_seed() {
    if let Ok(input) = Fraction::new(5, 3) {
        let result = input.complement();
        assert!(
            composition_fraction_result_equal(
                result,
                composition_decode_fraction(composition_encode_fraction(result))
            ),
            "composition complement actual return seed"
        );
    }
}

#[kani::proof]
fn composition_comparison_bad_precondition() {
    let _result = Fraction {
        numerator: 0,
        shift: 1,
    }
    .compare(Fraction::ZERO);
}
#[kani::proof]
fn composition_complement_bad_precondition() {
    let _result = Fraction {
        numerator: 0,
        shift: 1,
    }
    .complement();
}

// All raw u8/u8/u64 tuples are admitted; the specification widens independently.
// Actual From and Ord::min remain in the selected product predicate.

#[kani::proof]
fn lifecycle_source_fields_full_domain() {
    let maturity = kani::any::<u8>();
    let penalty = kani::any::<u8>();
    let ordinal = kani::any::<u64>();
    let input = (maturity, penalty, ordinal);
    let retained_input = input;
    let maturity_relation =
        (ordinal < 8 && maturity as u128 == ordinal as u128) || (ordinal >= 8 && maturity == 8);
    let penalty_relation = penalty <= 32;
    let required = penalty_relation && maturity_relation;
    let observed = source_fields_valid(input);
    assert_eq!(observed, required, "source fields exact classification");
    if maturity_relation {
        assert_eq!(
            observed, penalty_relation,
            "source fields penalty classification"
        );
    }
    if penalty_relation {
        assert_eq!(
            observed, maturity_relation,
            "source fields maturity classification"
        );
    }
    if !required {
        assert!(!observed, "source fields required rejection");
    }
    assert_eq!(input, retained_input, "source fields input retained");
    kani::cover!(
        input == (0, 0, 0) && observed,
        "source fields zero admitted"
    );
    kani::cover!(
        input == (7, 31, 7) && observed,
        "source fields last immature admitted"
    );
    kani::cover!(
        input == (8, 32, 8) && observed,
        "source fields exact maturity and penalty admitted"
    );
    kani::cover!(
        input == (8, 0, 9) && observed,
        "source fields after maturity admitted"
    );
    kani::cover!(
        input == (8, 32, u64::MAX) && observed,
        "source fields maximum ordinal admitted"
    );
    kani::cover!(
        input == (7, 32, 8) && !observed,
        "source fields low mature value rejected"
    );
    kani::cover!(
        input == (8, 33, 8) && !observed,
        "source fields first excess penalty rejected"
    );
    kani::cover!(
        input == (8, 255, 8) && !observed,
        "source fields maximum penalty rejected"
    );
    kani::cover!(
        input == (9, 0, 9) && !observed,
        "source fields first excess maturity rejected"
    );
    kani::cover!(
        input == (255, 0, u64::MAX) && !observed,
        "source fields maximum maturity rejected"
    );
    kani::cover!(
        input == (0, 0, 1) && !observed,
        "source fields low relationship rejected"
    );
    kani::cover!(
        input == (1, 0, 0) && !observed,
        "source fields high relationship rejected"
    );
}

#[kani::proof]
fn lifecycle_source_fields_control_strict_penalty() {
    assert!(
        source_fields_valid((8, 32, 8)),
        "source fields penalty classification"
    );
}

#[kani::proof]
fn lifecycle_source_fields_control_wrong_maturity() {
    assert!(
        source_fields_valid((8, 0, 8)),
        "source fields maturity classification"
    );
}

#[kani::proof]
fn lifecycle_source_fields_control_loose_relationship() {
    assert!(
        !source_fields_valid((0, 0, 1)),
        "source fields required rejection"
    );
}

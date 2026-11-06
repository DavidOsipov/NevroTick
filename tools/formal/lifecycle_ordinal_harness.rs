// Every raw predecessor is admitted; actual checked_next owns checked construction.
// The independent successor oracle widens before adding, without source helpers.

#[kani::proof]
fn lifecycle_ordinal_full_domain() {
    let previous = kani::any::<u64>();
    let retained_previous = previous;
    let mathematical_successor = (previous as u128) + 1;
    let observed = checked_next(previous);
    let exact_exhaustion = matches!(
        &observed,
        Err(FixtureCombinationLifecycleError::CounterExhausted)
    );
    assert_eq!(
        observed.is_ok(),
        mathematical_successor <= u64::MAX as u128,
        "ordinal success iff bound"
    );
    if let Ok(value) = &observed {
        assert_eq!(
            value.get() as u128,
            mathematical_successor,
            "ordinal exact successor"
        );
        assert!(value.get() > 0, "ordinal nonzero output");
    }
    if previous == u64::MAX {
        assert!(exact_exhaustion, "ordinal exact exhaustion error");
    }
    assert_eq!(
        exact_exhaustion,
        previous == u64::MAX,
        "ordinal exhaustion iff max"
    );
    assert_eq!(previous, retained_previous, "ordinal input retained");
    kani::cover!(
        previous == 0 && matches!(&observed, Ok(value) if value.get() == 1),
        "ordinal zero reaches one"
    );
    kani::cover!(
        previous == 1 && matches!(&observed, Ok(value) if value.get() == 2),
        "ordinal one reaches two"
    );
    kani::cover!(
        previous == (1_u64 << 63)
            && matches!(&observed, Ok(value) if value.get() == (1_u64 << 63) + 1),
        "ordinal high bit successor"
    );
    kani::cover!(
        previous == u64::MAX - 1 && matches!(&observed, Ok(value) if value.get() == u64::MAX),
        "ordinal final successor"
    );
    kani::cover!(
        previous == u64::MAX && exact_exhaustion,
        "ordinal maximum exhausts"
    );
}

#[kani::proof]
fn lifecycle_ordinal_control_always_error() {
    let observed = checked_next(0);
    assert!(observed.is_ok(), "ordinal success iff bound");
}

#[kani::proof]
fn lifecycle_ordinal_control_wrong_increment() {
    let observed = checked_next(0);
    assert!(
        matches!(observed, Ok(value) if value.get() as u128 == 1_u128),
        "ordinal exact successor"
    );
}

#[kani::proof]
fn lifecycle_ordinal_control_wrong_error() {
    let observed = checked_next(u64::MAX);
    assert!(
        matches!(
            observed,
            Err(FixtureCombinationLifecycleError::CounterExhausted)
        ),
        "ordinal exact exhaustion error"
    );
}

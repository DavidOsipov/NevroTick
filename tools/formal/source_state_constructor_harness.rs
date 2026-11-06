// Full raw typed constructor domain. Counters are never filtered or assumed.
// Actual validation retains Report/Path/Error allocation and normal drop behavior.
// Allocator exhaustion, target features and unwinding remain unqualified.

#[kani::proof]
fn source_state_constructor_full_domain() {
    let raw_label = kani::any::<u64>();
    let maturity = kani::any::<u8>();
    let penalty = kani::any::<u8>();
    let ordinal = kani::any::<u64>();
    let Some(label) = NonZeroU64::new(raw_label) else {
        return; // Raw zero is outside the actual NonZeroU64 input domain only.
    };
    let input = (raw_label, maturity, penalty, ordinal);
    let retained = input;
    let required = penalty <= 32
        && ((ordinal < 8 && maturity as u128 == ordinal as u128)
            || (ordinal >= 8 && maturity == 8));
    let counters = (maturity, penalty, ordinal);
    let validation = RawSourceFields { counters }.validate();
    assert_eq!(
        validation.is_ok(),
        required,
        "actual derived validation iff relation"
    );
    let callback = source_fields_rule(counters);
    assert_eq!(
        callback.is_err(),
        !required,
        "actual callback rejects iff invalid"
    );
    // Observe real report/error cleanup before entering the separate constructor.
    drop(validation);
    drop(callback);
    let result = FixtureCombinationSourceState::new(label, maturity, penalty, ordinal);
    assert_eq!(result.is_ok(), required, "constructor success iff relation");
    if let Ok(state) = &result {
        assert_eq!(state.source_id(), label, "constructor exact source label");
        assert_eq!(
            state.reachability_updates(),
            maturity,
            "constructor exact maturity"
        );
        assert_eq!(state.penalty(), penalty, "constructor exact penalty");
        assert_eq!(
            state.last_reachability_ordinal(),
            ordinal,
            "constructor exact ordinal"
        );
    }
    if let Err(error) = &result {
        assert!(
            matches!(error, FixtureCombinationLifecycleError::Invariant),
            "constructor every rejection is Invariant"
        );
    }
    assert!(
        !matches!(
            &result,
            Err(FixtureCombinationLifecycleError::ValidationInvariant)
        ),
        "constructor ValidationInvariant unreachable"
    );
    assert_eq!(input, retained, "constructor raw inputs unchanged");
    kani::cover!(
        raw_label == 1 && counters == (0, 0, 0) && result.is_ok(),
        "label one zero admitted"
    );
    kani::cover!(
        raw_label == u64::MAX && counters == (8, 32, u64::MAX) && result.is_ok(),
        "maximum label and ordinal admitted"
    );
    kani::cover!(
        counters == (7, 31, 7) && result.is_ok(),
        "last immature admitted"
    );
    kani::cover!(
        counters == (8, 32, 8) && result.is_ok(),
        "exact maturity and penalty admitted"
    );
    kani::cover!(
        counters == (8, 0, 9) && result.is_ok(),
        "post maturity admitted"
    );
    kani::cover!(
        counters == (7, 32, 8) && result.is_err(),
        "low mature rejected"
    );
    kani::cover!(
        counters == (8, 33, 8) && result.is_err(),
        "first excess penalty rejected"
    );
    kani::cover!(
        counters == (8, 255, 8) && result.is_err(),
        "maximum penalty rejected"
    );
    kani::cover!(
        counters == (9, 0, 9) && result.is_err(),
        "first excess maturity rejected"
    );
    kani::cover!(
        counters == (255, 0, u64::MAX) && result.is_err(),
        "maximum maturity rejected"
    );
    kani::cover!(
        counters == (0, 0, 1) && result.is_err(),
        "low relationship rejected"
    );
    kani::cover!(
        counters == (1, 0, 0) && result.is_err(),
        "high relationship rejected"
    );
}

#[kani::proof]
fn source_state_constructor_control_penalty() {
    let label = NonZeroU64::new(1).expect("fixed nonzero label");
    let result = FixtureCombinationSourceState::new(label, 8, 32, 8);
    assert!(result.is_ok(), "constructor boundary success");
}

#[kani::proof]
fn source_state_constructor_control_retention() {
    let label = NonZeroU64::new(1).expect("fixed nonzero label");
    let result = FixtureCombinationSourceState::new(label, 8, 32, 8);
    assert!(
        matches!(&result, Ok(state) if state.penalty() == 32),
        "constructor exact retained penalty"
    );
}

#[kani::proof]
fn source_state_constructor_control_error() {
    let label = NonZeroU64::new(1).expect("fixed nonzero label");
    let result = FixtureCombinationSourceState::new(label, 8, 33, 8);
    assert!(
        matches!(&result, Err(FixtureCombinationLifecycleError::Invariant)),
        "constructor exact invalid error"
    );
}

#[kani::proof]
fn source_state_constructor_control_validation() {
    // The direct callback remains healthy under the actual derived-closure bypass.
    // Keep the intended assertion focused on the independent validation obligation.
    let invalid = RawSourceFields {
        counters: (8, 33, 8),
    };
    let validation = invalid.validate();
    let direct_callback = source_fields_rule((8, 33, 8));
    assert!(
        validation.is_err() && direct_callback.is_err(),
        "actual derived validation rejects invalid tuple"
    );
}

//! Complete raw-u64 clock arithmetic contracts; origin and native semantics are separate.

fn monotonic_duration_contract(a: u64, b: u64) {
    let observed = clock::MonotonicNs::new(a).duration_since(clock::MonotonicNs::new(b));
    if a >= b {
        let expected = u128::from(a) - u128::from(b);
        match observed {
            Ok(elapsed) => assert_eq!(
                u128::from(elapsed.nanoseconds()),
                expected,
                "monotonic duration exact result"
            ),
            Err(_) => assert!(false, "monotonic duration required success"),
        }
    } else {
        assert_eq!(
            observed,
            Err(DomainError::DurationOutOfOrder),
            "monotonic reverse classification"
        );
    }
}

fn monotonic_addition_contract(a: u64, delta: u64) {
    // Admission uses only the real checked API; no proof-only private assembly.
    let elapsed = match clock::MonotonicNs::new(delta).duration_since(clock::MonotonicNs::new(0)) {
        Ok(value) => value,
        Err(_) => {
            assert!(false, "monotonic elapsed admission required success");
            return;
        }
    };
    assert_eq!(
        elapsed.nanoseconds(),
        delta,
        "monotonic elapsed admission exact"
    );
    let observed = clock::MonotonicNs::new(a)
        .checked_add(elapsed)
        .map(|value| value.nanoseconds());
    let sum = u128::from(a) + u128::from(delta);
    if sum <= u128::from(u64::MAX) {
        assert_eq!(observed, Ok(sum as u64), "monotonic addition exact success");
    } else {
        assert_eq!(
            observed,
            Err(DomainError::ArithmeticOverflow),
            "monotonic addition overflow classification"
        );
    }
}

fn active_awake_duration_contract(a: u64, b: u64) {
    let observed = clock::ActiveAwakeNs::new(a).duration_since(clock::ActiveAwakeNs::new(b));
    if a >= b {
        let expected = u128::from(a) - u128::from(b);
        match observed {
            Ok(elapsed) => assert_eq!(
                u128::from(elapsed.nanoseconds()),
                expected,
                "active-awake duration exact result"
            ),
            Err(_) => assert!(false, "active-awake duration required success"),
        }
    } else {
        assert_eq!(
            observed,
            Err(DomainError::DurationOutOfOrder),
            "active-awake reverse classification"
        );
    }
}

fn active_awake_addition_contract(a: u64, delta: u64) {
    let elapsed =
        match clock::ActiveAwakeNs::new(delta).duration_since(clock::ActiveAwakeNs::new(0)) {
            Ok(value) => value,
            Err(_) => {
                assert!(false, "active-awake elapsed admission required success");
                return;
            }
        };
    assert_eq!(
        elapsed.nanoseconds(),
        delta,
        "active-awake elapsed admission exact"
    );
    let observed = clock::ActiveAwakeNs::new(a)
        .checked_add(elapsed)
        .map(|value| value.nanoseconds());
    let sum = u128::from(a) + u128::from(delta);
    if sum <= u128::from(u64::MAX) {
        assert_eq!(
            observed,
            Ok(sum as u64),
            "active-awake addition exact success"
        );
    } else {
        assert_eq!(
            observed,
            Err(DomainError::ArithmeticOverflow),
            "active-awake addition overflow classification"
        );
    }
}

#[kani::proof]
fn monotonic_full_domain() {
    let a = kani::any::<u64>();
    let b = kani::any::<u64>();
    let delta = kani::any::<u64>();
    assert_eq!(
        clock::MonotonicNs::new(a).nanoseconds(),
        a,
        "monotonic reading exact"
    );
    monotonic_duration_contract(a, b);
    monotonic_addition_contract(a, delta);
    kani::cover!(a == 0 && b == 0 && delta == 0, "monotonic zero endpoints");
    kani::cover!(
        a == u64::MAX && b == 0 && delta == 0,
        "monotonic maximum duration"
    );
    kani::cover!(
        a == 0 && b == u64::MAX && delta == 0,
        "monotonic reversed endpoints"
    );
    kani::cover!(
        a == u64::MAX && b == u64::MAX && delta == 0,
        "monotonic maximum plus zero"
    );
    kani::cover!(
        a == 0 && b == 0 && delta == u64::MAX,
        "monotonic full-span addition"
    );
    kani::cover!(
        a == u64::MAX && b == 0 && delta == 1,
        "monotonic first addition overflow"
    );
}

#[kani::proof]
fn active_awake_full_domain() {
    let a = kani::any::<u64>();
    let b = kani::any::<u64>();
    let delta = kani::any::<u64>();
    assert_eq!(
        clock::ActiveAwakeNs::new(a).nanoseconds(),
        a,
        "active-awake reading exact"
    );
    active_awake_duration_contract(a, b);
    active_awake_addition_contract(a, delta);
    kani::cover!(
        a == 0 && b == 0 && delta == 0,
        "active-awake zero endpoints"
    );
    kani::cover!(
        a == u64::MAX && b == 0 && delta == 0,
        "active-awake maximum duration"
    );
    kani::cover!(
        a == 0 && b == u64::MAX && delta == 0,
        "active-awake reversed endpoints"
    );
    kani::cover!(
        a == u64::MAX && b == u64::MAX && delta == 0,
        "active-awake maximum plus zero"
    );
    kani::cover!(
        a == 0 && b == 0 && delta == u64::MAX,
        "active-awake full-span addition"
    );
    kani::cover!(
        a == u64::MAX && b == 0 && delta == 1,
        "active-awake first addition overflow"
    );
}

#[kani::proof]
fn monotonic_control_zero() {
    monotonic_duration_contract(0, 0);
}

#[kani::proof]
fn monotonic_control_duration() {
    monotonic_duration_contract(5, 3);
}

#[kani::proof]
fn monotonic_control_overflow() {
    monotonic_addition_contract(u64::MAX, 1);
}

#[kani::proof]
fn active_awake_control_zero() {
    active_awake_duration_contract(0, 0);
}

#[kani::proof]
fn active_awake_control_duration() {
    active_awake_duration_contract(5, 3);
}

#[kani::proof]
fn active_awake_control_overflow() {
    active_awake_addition_contract(u64::MAX, 1);
}

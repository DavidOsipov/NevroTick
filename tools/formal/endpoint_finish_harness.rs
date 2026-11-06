// Full raw-domain contract of the actual private EndpointParts::finish method.
// No constructor, Garde, fraction machinery or reachable-state claim is imported.
// A residual sign denotes every real residual in (-1, 0), {0}, or (0, 1).

const MIN_INTEGER: i128 = i64::MIN as i128;
const MAX_INTEGER: i128 = i64::MAX as i128;

/// Classify required outcomes independently, then relate the actual integer to
/// floor/ceiling inequalities. No checked-add algorithm is copied into the oracle.
fn endpoint_finish_contract(
    whole: i128,
    residual: Ordering,
    lower: bool,
) -> Result<(i64, FixtureEndpointRounding), FixtureEnclosureError> {
    let observed = EndpointParts { whole, residual }.finish(lower);
    let negative_floor = lower && residual == Ordering::Less;
    let positive_ceiling = !lower && residual == Ordering::Greater;
    let invariant =
        (negative_floor && whole == i128::MIN) || (positive_ceiling && whole == i128::MAX);
    let required_success = if negative_floor {
        MIN_INTEGER + 1 <= whole && whole <= MAX_INTEGER + 1
    } else if positive_ceiling {
        MIN_INTEGER - 1 <= whole && whole <= MAX_INTEGER - 1
    } else {
        MIN_INTEGER <= whole && whole <= MAX_INTEGER
    };
    assert_eq!(
        observed.is_ok(),
        required_success,
        "endpoint required success"
    );
    assert_eq!(
        matches!(observed, Err(FixtureEnclosureError::Invariant)),
        invariant,
        "endpoint checked overflow classification"
    );
    assert_eq!(
        matches!(observed, Err(FixtureEnclosureError::Unrepresentable)),
        !required_success && !invariant,
        "endpoint range rejection classification"
    );
    assert!(
        !matches!(observed, Err(FixtureEnclosureError::Interval { .. })),
        "endpoint never interval error"
    );

    if let Ok((integer, rounding)) = observed {
        // These bounds are a conclusion of the independently specified success
        // intervals. The explicit branch also keeps the oracle total on faulty
        // implementations returning spurious Ok for an arbitrary raw i128.
        if required_success {
            assert!(
                MIN_INTEGER - 1 <= whole && whole <= MAX_INTEGER + 1,
                "endpoint oracle success envelope"
            );
            let sign: i128 = match residual {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            };
            // Half-integer representatives have the same integer floor/ceiling
            // as every residual in their sign class. This restricts no input.
            let twice_value = whole * 2 + sign;
            let twice_integer = i128::from(integer) * 2;
            if lower {
                assert!(
                    twice_integer <= twice_value && twice_value < twice_integer + 2,
                    "endpoint exact floor relation"
                );
            } else {
                assert!(
                    twice_integer - 2 < twice_value && twice_value <= twice_integer,
                    "endpoint exact ceiling relation"
                );
            }
        }
        let expected_rounding = if residual == Ordering::Equal {
            FixtureEndpointRounding::Exact
        } else {
            FixtureEndpointRounding::OutwardFraction
        };
        assert_eq!(
            rounding, expected_rounding,
            "endpoint residual rounding classification"
        );
    }
    observed
}

#[kani::proof]
fn endpoint_finish_full_domain() {
    let whole = kani::any::<i128>();
    // Every actual Ordering variant is reachable from two unconstrained bools;
    // no enum representation assumption or raw-domain restriction is required.
    let residual = if kani::any::<bool>() {
        Ordering::Less
    } else if kani::any::<bool>() {
        Ordering::Equal
    } else {
        Ordering::Greater
    };
    let lower = kani::any::<bool>();
    let observed = endpoint_finish_contract(whole, residual, lower);
    kani::cover!(
        whole == i128::MIN
            && residual == Ordering::Less
            && lower
            && matches!(observed, Err(FixtureEnclosureError::Invariant)),
        "endpoint raw minimum negative floor is invariant"
    );
    kani::cover!(
        whole == i128::MAX
            && residual == Ordering::Greater
            && !lower
            && matches!(observed, Err(FixtureEnclosureError::Invariant)),
        "endpoint raw maximum positive ceiling is invariant"
    );
    kani::cover!(
        whole == MIN_INTEGER
            && residual == Ordering::Equal
            && lower
            && matches!(observed, Ok((i64::MIN, FixtureEndpointRounding::Exact))),
        "endpoint exact minimum succeeds"
    );
    kani::cover!(
        whole == MAX_INTEGER
            && residual == Ordering::Equal
            && !lower
            && matches!(observed, Ok((i64::MAX, FixtureEndpointRounding::Exact))),
        "endpoint exact maximum succeeds"
    );
    kani::cover!(
        whole == MAX_INTEGER + 1
            && residual == Ordering::Less
            && lower
            && matches!(
                observed,
                Ok((i64::MAX, FixtureEndpointRounding::OutwardFraction))
            ),
        "endpoint negative floor rounds into range"
    );
    kani::cover!(
        whole == MIN_INTEGER - 1
            && residual == Ordering::Greater
            && !lower
            && matches!(
                observed,
                Ok((i64::MIN, FixtureEndpointRounding::OutwardFraction))
            ),
        "endpoint positive ceiling rounds into range"
    );
    kani::cover!(
        whole == MIN_INTEGER - 1
            && residual == Ordering::Equal
            && lower
            && matches!(observed, Err(FixtureEnclosureError::Unrepresentable)),
        "endpoint exact lower neighbor is unrepresentable"
    );
    kani::cover!(
        whole == MAX_INTEGER + 1
            && residual == Ordering::Equal
            && !lower
            && matches!(observed, Err(FixtureEnclosureError::Unrepresentable)),
        "endpoint exact upper neighbor is unrepresentable"
    );
    kani::cover!(
        whole == 0
            && residual == Ordering::Less
            && !lower
            && matches!(observed, Ok((0, FixtureEndpointRounding::OutwardFraction))),
        "endpoint negative ceiling needs no adjustment"
    );
    kani::cover!(
        whole == 0
            && residual == Ordering::Greater
            && lower
            && matches!(observed, Ok((0, FixtureEndpointRounding::OutwardFraction))),
        "endpoint positive floor needs no adjustment"
    );
}

/// Ordinary exact zero must succeed; reject-all is a semantic defect.
#[kani::proof]
fn endpoint_finish_control_success() {
    let observed = EndpointParts {
        whole: 0,
        residual: Ordering::Equal,
    }
    .finish(true);
    assert!(
        matches!(observed, Ok((0, FixtureEndpointRounding::Exact))),
        "endpoint required success"
    );
}

/// Returning zero preserves the integer range but violates the exact floor.
#[kani::proof]
fn endpoint_finish_control_negative_floor() {
    let observed = EndpointParts {
        whole: 0,
        residual: Ordering::Less,
    }
    .finish(true);
    assert!(
        matches!(observed, Ok((-1, FixtureEndpointRounding::OutwardFraction))),
        "endpoint exact negative floor"
    );
}

/// M+1 is reached by the upper endpoint at center = distance = 2^29 seconds;
/// conversion failure is a range rejection, rather than an internal invariant.
#[kani::proof]
fn endpoint_finish_control_range_error() {
    let observed = EndpointParts {
        whole: MAX_INTEGER + 1,
        residual: Ordering::Equal,
    }
    .finish(false);
    assert!(
        matches!(observed, Err(FixtureEnclosureError::Unrepresentable)),
        "endpoint range rejection classification"
    );
}

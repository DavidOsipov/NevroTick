//! Full raw-i128 admission/rejection for the actual private relative predicate.
//!
//! This complements the producer theorem: arbitrary raw values include values
//! no valid four-coordinate exchange can produce. No input assumption is used.

/// Twice the independently specified signed-96-bit coordinate distance.
const RELATIVE_LIMIT: i128 = 158_456_325_028_528_675_187_087_900_670;

/// Specify both inclusive accepted endpoints and all rejected raw values.
fn relative_contract(value: i128) {
    let observed = exchange::proof_relative_in_range(value);
    if value < -RELATIVE_LIMIT {
        assert!(!observed, "lower relative rejection");
    } else if value > RELATIVE_LIMIT {
        assert!(!observed, "upper relative rejection");
    } else {
        assert!(observed, "required relative success");
    }
}

#[kani::proof]
fn relative_full_domain() {
    let value = kani::any::<i128>();
    relative_contract(value);
    let observed = exchange::proof_relative_in_range(value);
    kani::cover!(value == 0 && observed, "relative zero is admitted");
    kani::cover!(
        value == -RELATIVE_LIMIT && observed,
        "lower relative endpoint is admitted"
    );
    kani::cover!(
        value == RELATIVE_LIMIT && observed,
        "upper relative endpoint is admitted"
    );
    kani::cover!(
        value == -RELATIVE_LIMIT - 1 && !observed,
        "lower relative neighbor is rejected"
    );
    kani::cover!(
        value == RELATIVE_LIMIT + 1 && !observed,
        "upper relative neighbor is rejected"
    );
    kani::cover!(
        value == i128::MIN && !observed,
        "relative raw minimum is rejected"
    );
    kani::cover!(
        value == i128::MAX && !observed,
        "relative raw maximum is rejected"
    );
}

/// Reject-all violates required successful admission at ordinary zero.
#[kani::proof]
fn relative_control_success() {
    relative_contract(0);
}

/// Accept-all violates rejection of a representable raw upper-limit value.
#[kani::proof]
fn relative_control_upper() {
    relative_contract(i128::MAX);
}

/// Widening only the lower edge violates rejection of its first neighbor.
#[kani::proof]
fn relative_control_lower() {
    relative_contract(-RELATIVE_LIMIT - 1);
}

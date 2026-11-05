//! Full-domain exchange arithmetic split into independently universal conjuncts.
//!
//! All three healthy theorems are required together. No theorem is imported as
//! an assumption, and no native, chronology, authentication or origin fact follows.

const COORDINATE_LOWER: i128 = -39_614_081_257_132_168_796_771_975_168;
const COORDINATE_UPPER: i128 = 39_614_081_257_132_168_796_771_975_167;
const RELATIVE_LIMIT: i128 = 158_456_325_028_528_675_187_087_900_670;

/// This representation spans every signed-96-bit coordinate, independently.
fn admitted_ticks() -> i128 {
    (i128::from(kani::any::<i32>()) << 64) + i128::from(kani::any::<u64>())
}

/// Only the actual public constructor assembles product coordinate values.
fn admitted_coordinate(raw: i128) -> UtcCoordinateQ32 {
    match UtcCoordinateQ32::new(raw) {
        Ok(value) => value,
        Err(_) => {
            assert!(false, "encoded coordinate requires construction");
            panic!("coordinate construction required")
        }
    }
}

fn actual_exchange(
    t1: i128,
    t2: i128,
    t3: i128,
    t4: i128,
) -> Result<exchange::ExchangeArithmetic, DomainError> {
    exchange::exchange_arithmetic(
        admitted_coordinate(t1),
        admitted_coordinate(t2),
        admitted_coordinate(t3),
        admitted_coordinate(t4),
    )
}

/// The grouping oracle differs from the implementation's difference-first order.
fn phase_contract(t1: i128, t2: i128, t3: i128, t4: i128) {
    let observed = actual_exchange(t1, t2, t3, t4);
    assert!(observed.is_ok(), "phase required success");
    if let Ok(arithmetic) = observed {
        let phase = arithmetic.offset().numerator();
        assert_eq!(phase, (t2 + t3) - (t1 + t4), "exact phase numerator");
        assert!(
            phase >= -RELATIVE_LIMIT && phase <= RELATIVE_LIMIT,
            "phase producer envelope"
        );
    }
}

fn round_trip_contract(t1: i128, t2: i128, t3: i128, t4: i128) {
    let observed = actual_exchange(t1, t2, t3, t4);
    assert!(observed.is_ok(), "round trip required success");
    if let Ok(arithmetic) = observed {
        let round_trip = arithmetic.round_trip().numerator();
        assert_eq!(round_trip, (t2 + t4) - (t1 + t3), "exact signed round trip");
        assert!(
            round_trip >= -RELATIVE_LIMIT && round_trip <= RELATIVE_LIMIT,
            "round trip producer envelope"
        );
    }
}

/// This is a guarded universal implication, not an assumed producer bound.
/// The two producer theorems must establish its premise for both actual outputs.
fn scaling_contract(numerator: i128) {
    if numerator >= -RELATIVE_LIMIT && numerator <= RELATIVE_LIMIT {
        assert!(
            rounding::proof_nanosecond_scale(numerator).is_some(),
            "bounded scaling succeeds"
        );
    }
}

/// Literal witnesses still traverse actual checked construction and arithmetic.
fn concrete_observation(t1: i128, t2: i128, t3: i128, t4: i128) -> (i128, i128) {
    match actual_exchange(t1, t2, t3, t4) {
        Ok(arithmetic) => (
            arithmetic.offset().numerator(),
            arithmetic.round_trip().numerator(),
        ),
        Err(_) => {
            assert!(false, "concrete exchange witness requires success");
            panic!("concrete exchange observation required")
        }
    }
}

#[kani::proof]
fn exchange_phase_full_domain() {
    let t1 = admitted_ticks();
    let t2 = admitted_ticks();
    let t3 = admitted_ticks();
    let t4 = admitted_ticks();
    phase_contract(t1, t2, t3, t4);
    kani::cover!(
        concrete_observation(0, 0, 0, 0) == (0, 0),
        "phase zero exchange"
    );
    kani::cover!(
        concrete_observation(
            COORDINATE_LOWER,
            COORDINATE_UPPER,
            COORDINATE_UPPER,
            COORDINATE_LOWER
        ) == (RELATIVE_LIMIT, 0),
        "phase positive endpoint"
    );
    kani::cover!(
        concrete_observation(
            COORDINATE_UPPER,
            COORDINATE_LOWER,
            COORDINATE_LOWER,
            COORDINATE_UPPER
        ) == (-RELATIVE_LIMIT, 0),
        "phase negative endpoint"
    );
    kani::cover!(
        concrete_observation(0, 0, 1, 0) == (1, -1),
        "phase odd half-tick"
    );
}

#[kani::proof]
fn exchange_rtt_full_domain() {
    let t1 = admitted_ticks();
    let t2 = admitted_ticks();
    let t3 = admitted_ticks();
    let t4 = admitted_ticks();
    round_trip_contract(t1, t2, t3, t4);
    kani::cover!(
        concrete_observation(0, 0, 0, 0) == (0, 0),
        "round trip zero exchange"
    );
    kani::cover!(
        concrete_observation(
            COORDINATE_LOWER,
            COORDINATE_UPPER,
            COORDINATE_LOWER,
            COORDINATE_UPPER
        ) == (0, RELATIVE_LIMIT),
        "round trip positive endpoint"
    );
    kani::cover!(
        concrete_observation(
            COORDINATE_UPPER,
            COORDINATE_LOWER,
            COORDINATE_UPPER,
            COORDINATE_LOWER
        ) == (0, -RELATIVE_LIMIT),
        "round trip negative endpoint"
    );
    kani::cover!(
        concrete_observation(0, 0, 1, 0) == (1, -1),
        "round trip negative tick"
    );
}

#[kani::proof]
fn exchange_scaling_full_domain() {
    scaling_contract(kani::any::<i128>());
    kani::cover!(
        rounding::proof_nanosecond_scale(0) == Some(0),
        "scaling zero succeeds"
    );
    kani::cover!(
        rounding::proof_nanosecond_scale(RELATIVE_LIMIT) == Some(RELATIVE_LIMIT * 1_000_000_000),
        "scaling positive caller endpoint"
    );
    kani::cover!(
        rounding::proof_nanosecond_scale(-RELATIVE_LIMIT) == Some(-RELATIVE_LIMIT * 1_000_000_000),
        "scaling negative caller endpoint"
    );
    kani::cover!(
        rounding::proof_nanosecond_scale(1) == Some(1_000_000_000),
        "scaling interior succeeds"
    );
}

#[kani::proof]
fn exchange_composition_control_success() {
    phase_contract(0, 0, 0, 0);
}

#[kani::proof]
fn exchange_composition_control_phase() {
    phase_contract(0, 0, 1, 0);
}

#[kani::proof]
fn exchange_composition_control_rtt() {
    round_trip_contract(0, 0, 1, 0);
}

#[kani::proof]
fn exchange_composition_control_scaling() {
    scaling_contract(RELATIVE_LIMIT);
}

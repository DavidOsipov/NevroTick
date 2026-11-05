//! Source-bound coordinate admission and exact four-coordinate arithmetic.
//!
//! These contracts prove numeric behavior, not a common physical time scale,
//! saved-transmit provenance, association, authentication or clock authority.

/// Literal independent signed-96-bit endpoints in Q32 ticks.
const COORDINATE_LOWER: i128 = -39_614_081_257_132_168_796_771_975_168;
const COORDINATE_UPPER: i128 = 39_614_081_257_132_168_796_771_975_167;

/// Twice the distance between the independent coordinate endpoints: 2^97 - 2.
const RELATIVE_LIMIT: i128 = 158_456_325_028_528_675_187_087_900_670;

/// Complete admission, exact retention and redacted rejection over raw i128.
/// No implementation constants or range predicate supply the expected result.
fn coordinate_contract(raw: i128) {
    let observed = UtcCoordinateQ32::new(raw);
    if raw < COORDINATE_LOWER || raw > COORDINATE_UPPER {
        assert_eq!(
            observed,
            Err(DomainError::CoordinateOutOfRange),
            "coordinate rejection classification"
        );
    } else {
        let coordinate = match observed {
            Ok(value) => value,
            Err(_) => {
                assert!(false, "required coordinate success");
                return;
            }
        };
        assert_eq!(coordinate.ticks(), raw, "exact retained coordinate");
    }
}

#[kani::proof]
fn coordinate_full_domain() {
    let raw = kani::any::<i128>();
    coordinate_contract(raw);
    kani::cover!(raw == 0, "zero coordinate succeeds");
    kani::cover!(raw == COORDINATE_LOWER, "minimum coordinate succeeds");
    kani::cover!(raw == COORDINATE_UPPER, "maximum coordinate succeeds");
    kani::cover!(
        raw == COORDINATE_LOWER - 1,
        "lower coordinate neighbor is rejected"
    );
    kani::cover!(
        raw == COORDINATE_UPPER + 1,
        "upper coordinate neighbor is rejected"
    );
    kani::cover!(raw == i128::MIN, "raw signed minimum is rejected");
    kani::cover!(raw == i128::MAX, "raw signed maximum is rejected");
}

/// Reject-all must violate required success, even at an ordinary valid input.
#[kani::proof]
fn coordinate_control_zero() {
    coordinate_contract(0);
}

/// Perturbing a valid stored coordinate must violate exact retention.
#[kani::proof]
fn coordinate_control_retained() {
    coordinate_contract(0);
}

/// The first invalid upper neighbor must retain its exact rejection category.
#[kani::proof]
fn coordinate_control_rejection() {
    coordinate_contract(COORDINATE_UPPER + 1);
}

/// Every signed-96-bit value has one high-i32/low-u64 representation.
/// The representation covers the entire admitted domain without assumptions.
fn admitted_ticks() -> i128 {
    (i128::from(kani::any::<i32>()) << 64) + i128::from(kani::any::<u64>())
}

/// Exercise actual coordinate admission; no private aggregate is assembled.
fn admitted_coordinate(raw: i128) -> UtcCoordinateQ32 {
    match UtcCoordinateQ32::new(raw) {
        Ok(value) => value,
        Err(_) => {
            assert!(false, "encoded coordinate must be admitted");
            panic!("coordinate construction required")
        }
    }
}

/// Complete exact exchange arithmetic over four independent valid coordinates.
///
/// The oracle adds like-signed role groups before subtraction; production first
/// forms differences. Every oracle intermediate fits i128 for signed-96-bit
/// inputs. No chronology or nonnegative-RTT precondition narrows the domain.
fn exchange_contract(t1: i128, t2: i128, t3: i128, t4: i128) {
    let observed = exchange::exchange_arithmetic(
        admitted_coordinate(t1),
        admitted_coordinate(t2),
        admitted_coordinate(t3),
        admitted_coordinate(t4),
    );
    let arithmetic = match observed {
        Ok(value) => value,
        Err(_) => {
            assert!(false, "required exchange success");
            return;
        }
    };
    let offset = arithmetic.offset().numerator();
    let round_trip = arithmetic.round_trip().numerator();
    assert_eq!(offset, (t2 + t3) - (t1 + t4), "exact phase numerator");
    assert_eq!(round_trip, (t2 + t4) - (t1 + t3), "exact signed round trip");
    assert!(
        offset >= -RELATIVE_LIMIT && offset <= RELATIVE_LIMIT,
        "phase producer envelope"
    );
    assert!(
        round_trip >= -RELATIVE_LIMIT && round_trip <= RELATIVE_LIMIT,
        "round trip producer envelope"
    );
    assert!(
        offset.checked_mul(1_000_000_000).is_some(),
        "phase nanosecond scaling fits"
    );
    assert!(
        round_trip.checked_mul(1_000_000_000).is_some(),
        "round trip nanosecond scaling fits"
    );
}

/// Observe one concrete witness through the actual checked public APIs.
/// Error paths fail explicitly and return no fabricated observation.
fn concrete_exchange_observation(t1: i128, t2: i128, t3: i128, t4: i128) -> (i128, i128) {
    let arithmetic = match exchange::exchange_arithmetic(
        admitted_coordinate(t1),
        admitted_coordinate(t2),
        admitted_coordinate(t3),
        admitted_coordinate(t4),
    ) {
        Ok(value) => value,
        Err(_) => {
            assert!(false, "concrete exchange witness requires success");
            panic!("concrete exchange observation required")
        }
    };
    (
        arithmetic.offset().numerator(),
        arithmetic.round_trip().numerator(),
    )
}

#[kani::proof]
fn exchange_full_domain() {
    let t1 = admitted_ticks();
    let t2 = admitted_ticks();
    let t3 = admitted_ticks();
    let t4 = admitted_ticks();
    exchange_contract(t1, t2, t3, t4);

    // The universal contract above still ranges over four independent complete
    // coordinates. Concrete real calls establish reachable controls without
    // asking six SAT searches to discover the same literal boundary inputs.
    let zero = concrete_exchange_observation(0, 0, 0, 0);
    kani::cover!(zero == (0, 0), "zero exchange");
    let positive_phase = concrete_exchange_observation(
        COORDINATE_LOWER,
        COORDINATE_UPPER,
        COORDINATE_UPPER,
        COORDINATE_LOWER,
    );
    kani::cover!(
        positive_phase == (RELATIVE_LIMIT, 0),
        "positive phase endpoint"
    );
    let negative_phase = concrete_exchange_observation(
        COORDINATE_UPPER,
        COORDINATE_LOWER,
        COORDINATE_LOWER,
        COORDINATE_UPPER,
    );
    kani::cover!(
        negative_phase == (-RELATIVE_LIMIT, 0),
        "negative phase endpoint"
    );
    let positive_round_trip = concrete_exchange_observation(
        COORDINATE_LOWER,
        COORDINATE_UPPER,
        COORDINATE_LOWER,
        COORDINATE_UPPER,
    );
    kani::cover!(
        positive_round_trip == (0, RELATIVE_LIMIT),
        "positive round trip endpoint"
    );
    let negative_round_trip = concrete_exchange_observation(
        COORDINATE_UPPER,
        COORDINATE_LOWER,
        COORDINATE_UPPER,
        COORDINATE_LOWER,
    );
    kani::cover!(
        negative_round_trip == (0, -RELATIVE_LIMIT),
        "negative round trip endpoint"
    );
    let odd_phase = concrete_exchange_observation(0, 0, 1, 0);
    kani::cover!(odd_phase == (1, -1), "odd phase and negative round trip");
}

/// Reject-all violates required success without involving extreme arithmetic.
#[kani::proof]
fn exchange_control_zero() {
    exchange_contract(0, 0, 0, 0);
}

/// Reversing the transmit contribution changes +1 phase to -1.
#[kani::proof]
fn exchange_control_phase() {
    exchange_contract(0, 0, 1, 0);
}

/// Reversing peer-elapsed subtraction changes a legitimate -1 RTT to +1.
#[kani::proof]
fn exchange_control_round_trip() {
    exchange_contract(0, 0, 1, 0);
}

//! Exact public four-coordinate arithmetic from literal independently reviewed
//! exchange-vector facts. No future live exchange/sample outcome is claimed.
#![forbid(unsafe_code)]

use time_domain::{DomainError, ExchangeArithmetic, UtcCoordinateQ32, exchange_arithmetic};

/// Admit four explicit fixture coordinates without manufacturing provenance.
/// # Errors
/// Propagates unexpected coordinate or arithmetic rejection.
fn calculate(ticks: [i128; 4]) -> Result<ExchangeArithmetic, DomainError> {
    exchange_arithmetic(
        UtcCoordinateQ32::new(ticks[0])?,
        UtcCoordinateQ32::new(ticks[1])?,
        UtcCoordinateQ32::new(ticks[2])?,
        UtcCoordinateQ32::new(ticks[3])?,
    )
}

/// The four reviewed arithmetic vectors use fixed raw coordinates and numerators.
/// Expectations are copied from the reviewed JSON specification, not generated
/// by production arithmetic. All readings use the common non-leap fixture scale.
/// # Errors
/// Propagates unexpected checked construction/arithmetic failure.
/// # Panics
/// Fails if exact offset/RTT, half-ticks, rollover or positive-behind sign differs.
#[test]
fn four_reviewed_literal_exchange_oracles() -> Result<(), DomainError> {
    for (name, ticks, offset, round_trip) in [
        (
            "ordinary-symmetric",
            [
                0xe000_0000_0000_0000_i128,
                0xe000_0000_11eb_851e,
                0xe000_0000_147a_e147,
                0xe000_0000_1c28_f5c2,
            ],
            171_798_691,
            429_496_729,
        ),
        (
            "randomized-wire-token",
            [
                0xe000_0000_0000_0000_i128,
                0xe000_0000_11eb_851e,
                0xe000_0000_147a_e147,
                0xe000_0000_1c28_f5c2,
            ],
            171_798_691,
            429_496_729,
        ),
        (
            "asymmetric-path",
            [
                0xe000_0000_0000_0000_i128,
                0xe000_0000_0ccc_cccc,
                0xe000_0000_0f5c_28f5,
                0xe000_0000_1c28_f5c2,
            ],
            -1,
            429_496_729,
        ),
        (
            "era-rollover-explicit",
            [
                18_446_744_073_575_333_888_i128,
                18_446_744_073_843_769_344,
                18_446_744_074_112_204_800,
                18_446_744_074_380_640_256,
            ],
            0,
            536_870_912,
        ),
    ] {
        let result = calculate(ticks)?;
        assert_eq!(result.offset().numerator(), offset, "{name}");
        assert_eq!(result.round_trip().numerator(), round_trip, "{name}");
    }
    Ok(())
}

/// A randomized packet origin is deliberately unrelated to saved real T1.
/// The ordinary coordinate API cannot certify provenance, but substitution must
/// mathematically discriminate against the independently specified saved-T1 facts.
/// # Errors
/// Propagates unexpected checked coordinate/arithmetic failure.
/// # Panics
/// Fails if using the unrelated token accidentally preserves the healthy oracle.
#[test]
fn saved_real_t1_discriminates_unrelated_wire_token() -> Result<(), DomainError> {
    let saved = calculate([
        0xe000_0000_0000_0000,
        0xe000_0000_11eb_851e,
        0xe000_0000_147a_e147,
        0xe000_0000_1c28_f5c2,
    ])?;
    let substituted = calculate([
        0x0102_0304_0506_0708,
        0xe000_0000_11eb_851e,
        0xe000_0000_147a_e147,
        0xe000_0000_1c28_f5c2,
    ])?;
    assert_eq!(saved.offset().numerator(), 171_798_691);
    assert_eq!(saved.round_trip().numerator(), 429_496_729);
    assert_ne!(substituted.offset().numerator(), 171_798_691);
    assert_ne!(substituted.round_trip().numerator(), 429_496_729);
    Ok(())
}

/// Odd signs retain exact Q33 half-ticks and negative raw delays remain inert data.
/// # Errors
/// Propagates unexpected checked input/arithmetic rejection.
/// # Panics
/// Fails if half-ticks truncate or an unconditional delay policy is introduced.
#[test]
fn odd_offset_signs_and_negative_rtt_are_preserved() -> Result<(), DomainError> {
    for (ticks, offset, round_trip) in [
        ([0, 1, 0, 0], 1, 1),
        ([0, -1, 0, 0], -1, -1),
        ([0, 0, 3, 1], 2, -2),
        ([0, 0, 0, 0], 0, 0),
    ] {
        let result = calculate(ticks)?;
        assert_eq!(result.offset().numerator(), offset);
        assert_eq!(result.round_trip().numerator(), round_trip);
    }
    Ok(())
}

/// Endpoint differences reach exact 2^97-2 output bounds without primitive wrap.
/// # Errors
/// Propagates unexpected rejection of legal extreme-coordinate arithmetic.
/// # Panics
/// Fails when intermediate differences or result signs/ranges are narrowed.
#[test]
fn extreme_coordinates_preserve_full_result_envelope() -> Result<(), DomainError> {
    let low = -(1_i128 << 95);
    let high = (1_i128 << 95) - 1;
    let bound = (1_i128 << 97) - 2;
    assert_eq!(
        calculate([low, high, high, low])?.offset().numerator(),
        bound
    );
    assert_eq!(
        calculate([high, low, low, high])?.offset().numerator(),
        -bound
    );
    assert_eq!(
        calculate([low, high, low, high])?.round_trip().numerator(),
        bound
    );
    assert_eq!(
        calculate([high, low, high, low])?.round_trip().numerator(),
        -bound
    );
    Ok(())
}

/// Analytic integer paths produce known sign/delay; common translation changes
/// no relative result. One Q32 second and binary fractions are exactly represented.
/// # Errors
/// Propagates unexpected valid-coordinate or arithmetic rejection.
/// # Panics
/// Fails when physical fixture facts or common-translation invariance differ.
#[test]
fn known_binary_paths_and_common_translation() -> Result<(), DomainError> {
    // +1/4 second peer offset, 1/8 second each path, 1/16 second processing.
    let trajectory = [0_i128, 1_610_612_736, 1_879_048_192, 1_342_177_280];
    for translation in [-(1_i128 << 90), -1, 0, 1, 1_i128 << 90] {
        let shifted = trajectory.map(|value| value + translation);
        let result = calculate(shifted)?;
        assert_eq!(result.offset().numerator(), 2_147_483_648);
        assert_eq!(result.round_trip().numerator(), 1_073_741_824);
    }
    Ok(())
}

/// A finite known-process model supplies 1,225 independent offset/delay pairs.
/// Inputs are built from truth, path delays and processing rather than applying
/// the production four-coordinate formulas as an expected-value oracle.
/// # Errors
/// Propagates unexpected checked construction/arithmetic failure.
/// # Panics
/// Fails when exact model offset/path bias, delay or translation differs.
#[test]
fn bounded_generated_known_process_trajectories() -> Result<(), DomainError> {
    for truth in -2_i128..=2 {
        for forward in 0_i128..=6 {
            for reverse in 0_i128..=6 {
                for processing in 0_i128..=4 {
                    let t1 = -1_000;
                    let t2 = t1 + forward + truth;
                    let t3 = t2 + processing;
                    let t4 = t1 + forward + processing + reverse;
                    let expected_phase = 2 * truth + forward - reverse;
                    let expected_delay = forward + reverse;
                    let result = calculate([t1, t2, t3, t4])?;
                    assert_eq!(result.offset().numerator(), expected_phase);
                    assert_eq!(result.round_trip().numerator(), expected_delay);
                }
            }
        }
    }
    Ok(())
}

// SPDX-License-Identifier: GPL-2.0-only
//! Independent Q1 dyadic facts and direct checked-boundary controls.
//!
//! These synthetic internal facts use the real checked measurement constructor.
//! Real UDP acquisition evidence belongs to the separate external consumer tests.

use std::{
    net::SocketAddr,
    ptr,
    sync::Arc,
    time::{Duration, Instant, UNIX_EPOCH},
};

use ntp_wire::decode_fixed_header;

use super::*;
use crate::{ExperimentalProfile, RunClockContext, SoftwareClock, measurement::measure};

/// Literal era-zero time base, independent of the host wall clock.
const BASE: u64 = 4_000_000_000;
/// One eighth second in exact decimal nanoseconds.
const LOCAL_NS: u64 = 125_000_000;
/// One sixteenth second in exact Q32 ticks.
const SERVER_TICKS: i128 = 268_435_456;

/// Assert a reviewed pure product success without formatting its error payload.
/// # Errors
/// Preserve the original typed result for the existing propagation path.
/// # Panics
/// Reject only the independently known-valid product result named by the static fact.
fn product_ok<T, E>(result: Result<T, E>, fact: &'static str) -> Result<T, E> {
    assert!(result.is_ok(), "{fact}");
    result
}

/// Build original checked nominal facts on the common 100 ns wall-clock grid.
///
/// The host supplies only a monotonic fixture anchor, never a date or accuracy
/// claim. Capture brackets have zero fixture width; they are not physical bounds.
/// # Errors
/// Preserve unsupported host times, raw header or real measurement construction.
fn facts(
    local_ns: u64,
    server_ticks: i128,
    precision: i8,
    roots: [u32; 2],
) -> Result<TransportFacts, Box<dyn Error>> {
    facts_with_wall(
        local_ns,
        i64::try_from(local_ns)?,
        0,
        server_ticks,
        precision,
        roots,
    )
}

/// Keep actual monotonic ordering distinct from explicitly declared wall progress.
///
/// Negative wall progress is conditional input under the admitted consistency
/// allowance; it supplies no production continuity or physical-accuracy evidence.
/// # Errors
/// Preserve unsupported host instants, header or checked measurement admission.
fn facts_with_wall(
    local_ns: u64,
    wall_delta_ns: i64,
    maximum_residual_ns: u64,
    server_ticks: i128,
    precision: i8,
    roots: [u32; 2],
) -> Result<TransportFacts, Box<dyn Error>> {
    let origin = Instant::now();
    let wall = UNIX_EPOCH
        .checked_add(Duration::from_secs(BASE - 2_208_988_800))
        .ok_or("wall fixture")?;
    let before = SoftwareClock {
        mono: origin,
        utc: wall,
        mono_after: origin,
    };
    let read_mono = origin
        .checked_add(Duration::from_nanos(local_ns))
        .ok_or("read fixture")?;
    let read = SoftwareClock {
        mono: read_mono,
        utc: if wall_delta_ns < 0 {
            wall.checked_sub(Duration::from_nanos(wall_delta_ns.unsigned_abs()))
        } else {
            wall.checked_add(Duration::from_nanos(wall_delta_ns.unsigned_abs()))
        }
        .ok_or("read wall fixture")?,
        mono_after: read_mono,
    };
    let processing = read_mono
        .checked_add(Duration::from_millis(7))
        .ok_or("processing fixture")?;
    let receive_bits = (BASE << 32) + (1 << 30);
    let transmit_bits = u64::try_from(
        i128::from(receive_bits)
            .checked_add(server_ticks)
            .ok_or("server fixture")?,
    )?;
    let mut bytes = [0_u8; 48];
    bytes[0] = 0x24;
    bytes[1] = 2;
    bytes[3] = precision.to_be_bytes()[0];
    bytes[4..8].copy_from_slice(&roots[0].to_be_bytes());
    bytes[8..12].copy_from_slice(&roots[1].to_be_bytes());
    bytes[32..40].copy_from_slice(&receive_bits.to_be_bytes());
    bytes[40..48].copy_from_slice(&transmit_bits.to_be_bytes());
    let header = decode_fixed_header(&bytes).map_err(|_| "header fixture")?;
    let profile = Some(product_ok(
        ExperimentalProfile::new(BASE - 1, BASE + 20, maximum_residual_ns),
        "known valid quality fixture profile was refused",
    )?);
    let measurement = product_ok(
        measure(profile, before, before, read, processing, header),
        "known valid synthetic quality measurement was refused",
    )?;
    let peer: SocketAddr = "127.0.0.1:8123".parse()?;
    let local: SocketAddr = "127.0.0.1:18123".parse()?;
    Ok(TransportFacts {
        send_before: before,
        send_after: before,
        read_return: read,
        processing,
        peer_receive_bits: receive_bits,
        peer_transmit_bits: transmit_bits,
        header,
        peer,
        local,
        identity: Arc::new(()),
        run_context: Arc::new(RunClockContext { origin }),
        measurement: Ok(measurement),
        profile,
    })
}

/// Construct all explicit dyadic assumptions through the actual checked path.
/// # Errors
/// Propagate unexpected rejection of the independently valid declarations.
fn assumptions(limits: [f64; 3]) -> Result<DeclaredQualityAssumptions, QualityRefusal> {
    DeclaredQualityAssumptions::new(
        1.0 / 1024.0,
        1.0 / 256.0,
        1.0 / 512.0,
        DeclaredFrequencyInterval::new(-1.0 / 64.0, 1.0 / 64.0)?,
        DeclaredQualityLimits::new(limits[0], limits[1], limits[2])?,
    )
}

/// Compare a computed result against a literal exact binary64 specification fact.
/// # Panics
/// Panic if the original numeric value or the canonical zero sign differs.
fn exact(actual: f64, expected: f64) {
    assert_eq!(actual.to_bits(), expected.to_bits());
}

/// The documented dyadic calculation must preserve source association and age.
/// # Errors
/// Preserve genuine checked setup or evaluator refusal.
/// # Panics
/// Panic on changed independent quantities, identity, exact tuple or original age.
#[test]
fn dyadic_calculation_borrows_original_facts_and_assumptions() -> Result<(), Box<dyn Error>> {
    for (remote, sign) in [
        (SERVER_TICKS, RationalSign::Positive),
        (3 * SERVER_TICKS, RationalSign::Negative),
    ] {
        let facts = facts(LOCAL_NS, remote, -10, [8192, 256])?;
        let declared = assumptions([1.0, 1.0, 1.0])?;
        let result = product_ok(
            evaluate_conditional_quality(&facts, &declared),
            "known valid conditional quality evaluation was refused",
        )?;
        assert!(ptr::eq(
            ptr::from_ref(result.original_facts()),
            ptr::from_ref(&facts)
        ));
        assert!(ptr::eq(
            ptr::from_ref(result.declared_assumptions()),
            ptr::from_ref(&declared)
        ));
        exact(
            result
                .declared_assumptions()
                .transmit_capture_allowance_seconds(),
            1.0 / 256.0,
        );
        exact(
            result
                .declared_assumptions()
                .receive_capture_allowance_seconds(),
            1.0 / 512.0,
        );
        assert_eq!(
            result.local_interval().numerator_magnitude(),
            536_870_912_000_000_000
        );
        assert_eq!(
            result.local_interval().denominator().get(),
            4_294_967_296_000_000_000
        );
        assert_eq!(
            result.raw_round_trip().numerator_magnitude(),
            268_435_456_000_000_000
        );
        assert_eq!(result.raw_round_trip().sign(), sign);
        exact(result.local_interval().seconds(), 1.0 / 8.0);
        exact(result.local_interval().absolute_roundoff_seconds(), 0.0);
        exact(
            result.server_interval().seconds(),
            if remote == SERVER_TICKS {
                1.0 / 16.0
            } else {
                3.0 / 16.0
            },
        );
        exact(result.peer_precision_seconds(), 1.0 / 1024.0);
        exact(result.combined_precision_seconds(), 1.0 / 512.0);
        exact(result.adjusted_delay_seconds(), 1.0 / 16.0);
        exact(result.frequency_half_width(), 1.0 / 64.0);
        exact(result.peer_dispersion_seconds(), 3.0 / 512.0);
        exact(result.peer_distance_seconds(), 19.0 / 512.0);
        exact(result.root_delay_seconds(), 3.0 / 16.0);
        exact(result.root_dispersion_seconds(), 5.0 / 512.0);
        exact(result.root_distance_seconds(), 53.0 / 512.0);
        exact(result.advertised_root_delay_seconds(), 1.0 / 8.0);
        exact(result.advertised_root_dispersion_seconds(), 1.0 / 256.0);
        exact(result.advertised_distance_seconds(), 17.0 / 256.0);
        exact(result.delay_minus_dispersion_seconds(), 29.0 / 512.0);
        assert_eq!(
            product_ok(
                result.original_facts().measurement(),
                "known successful retained quality measurement was lost"
            )?
            .acquisition_age(),
            Duration::from_millis(7)
        );
        assert_eq!(format!("{result:?}"), "ConditionalQuality { .. }");
    }
    Ok(())
}

/// Negative peer intervals use absolute response time without a new ordering rule.
/// # Errors
/// Preserve setup or evaluator refusal.
/// # Panics
/// Panic if signed R is lost or if the computed quality differs from exact facts.
#[test]
fn signed_server_interval_and_unsigned_root_words_are_retained() -> Result<(), Box<dyn Error>> {
    let negative = facts(LOCAL_NS, -SERVER_TICKS, -10, [0, 0])?;
    let declared = assumptions([1.0, 1.0, 1.0])?;
    let result = product_ok(
        evaluate_conditional_quality(&negative, &declared),
        "known valid conditional quality evaluation was refused",
    )?;
    exact(result.server_interval().seconds(), -1.0 / 16.0);
    exact(result.raw_round_trip().seconds(), 3.0 / 16.0);
    exact(result.peer_distance_seconds(), 51.0 / 512.0);
    let high = facts(LOCAL_NS, SERVER_TICKS, -10, [0x8000_0000, 0xffff_0000])?;
    let broad = assumptions([1_000_000.0; 3])?;
    let high_result = product_ok(
        evaluate_conditional_quality(&high, &broad),
        "known valid conditional quality evaluation was refused",
    )?;
    exact(high_result.advertised_root_delay_seconds(), 32_768.0);
    exact(high_result.advertised_root_dispersion_seconds(), 65_535.0);
    exact(high_result.advertised_distance_seconds(), 81_919.0);
    exact(high_result.peer_distance_seconds(), 19.0 / 512.0);
    Ok(())
}

/// Negative R is compared by its absolute magnitude at the inclusive limit.
///
/// The fixture has exact R=-1/16 s, raw RTT=3/16 s and peer distance=51/512 s.
/// A signed comparison would wrongly accept the positive predecessor of 1/16.
/// # Errors
/// Preserve checked fixture or accepted-boundary evaluation failures.
/// # Panics
/// Panic if the predecessor passes, equality fails or original facts/age change.
#[test]
fn negative_server_interval_processing_limit_uses_absolute_magnitude() -> Result<(), Box<dyn Error>>
{
    let facts = facts(LOCAL_NS, -SERVER_TICKS, -10, [0, 0])?;
    let original_age = product_ok(
        facts.measurement(),
        "known successful quality fixture measurement was lost",
    )?
    .acquisition_age();
    let original_header = facts.header();
    let processing = 1.0_f64 / 16.0;
    let below = assumptions([1.0, 1.0, processing.next_down()])?;
    assert!(matches!(
        evaluate_conditional_quality(&facts, &below),
        Err(QualityRefusal::ServerProcessing)
    ));
    assert_eq!(
        product_ok(
            facts.measurement(),
            "known successful quality fixture measurement was lost"
        )?
        .acquisition_age(),
        original_age
    );
    for maximum in [processing, processing.next_up()] {
        let declared = assumptions([1.0, 1.0, maximum])?;
        let result = evaluate_conditional_quality(&facts, &declared);
        assert!(result.is_ok());
        let result = result?;
        exact(result.server_interval().seconds(), -1.0 / 16.0);
        exact(result.raw_round_trip().seconds(), 3.0 / 16.0);
        exact(result.peer_distance_seconds(), 51.0 / 512.0);
        assert!(ptr::eq(
            ptr::from_ref(result.original_facts()),
            ptr::from_ref(&facts)
        ));
        assert_eq!(result.original_facts().header(), original_header);
        assert_eq!(
            product_ok(
                result.original_facts().measurement(),
                "known successful retained quality measurement was lost"
            )?
            .acquisition_age(),
            original_age
        );
    }
    Ok(())
}

/// The frequency-dispersion term uses absolute wall progress under its declaration.
/// # Errors
/// Preserve checked capture/consistency admission or conditional evaluation refusal.
/// # Panics
/// Panic if negative local progress subtracts uncertainty or changes original age.
#[test]
fn negative_local_wall_interval_grows_declared_frequency_dispersion() -> Result<(), Box<dyn Error>>
{
    let facts = facts_with_wall(
        LOCAL_NS,
        -125_000_000,
        250_000_000,
        SERVER_TICKS,
        -10,
        [0, 0],
    )?;
    let declared = assumptions([1.0, 1.0, 1.0])?;
    let result = product_ok(
        evaluate_conditional_quality(&facts, &declared),
        "known valid conditional quality evaluation was refused",
    )?;
    exact(result.local_interval().seconds(), -1.0 / 8.0);
    exact(result.raw_round_trip().seconds(), -3.0 / 16.0);
    exact(result.adjusted_delay_seconds(), 3.0 / 16.0);
    exact(result.peer_dispersion_seconds(), 3.0 / 512.0);
    exact(result.peer_distance_seconds(), 51.0 / 512.0);
    assert_eq!(
        product_ok(
            result.original_facts().measurement(),
            "known successful retained quality measurement was lost",
        )?
        .observed_residual_ns(),
        250_000_000
    );
    assert_eq!(
        product_ok(
            result.original_facts().measurement(),
            "known successful retained quality measurement was lost"
        )?
        .acquisition_age(),
        Duration::from_millis(7)
    );
    Ok(())
}

/// Zero nominal RTT still receives the explicit combined-precision delay floor.
/// # Errors
/// Preserve checked capture/measurement construction or evaluator refusal.
/// # Panics
/// Panic if a zero RTT removes the independent precision floor or distance terms.
#[test]
fn zero_raw_rtt_uses_combined_precision_floor() -> Result<(), Box<dyn Error>> {
    let facts = facts(LOCAL_NS, 2 * SERVER_TICKS, -10, [0, 0])?;
    let declared = assumptions([1.0, 1.0, 1.0])?;
    let result = product_ok(
        evaluate_conditional_quality(&facts, &declared),
        "known valid conditional quality evaluation was refused",
    )?;
    assert_eq!(result.raw_round_trip().numerator_magnitude(), 0);
    exact(result.raw_round_trip().seconds(), 0.0);
    exact(result.combined_precision_seconds(), 1.0 / 512.0);
    exact(result.adjusted_delay_seconds(), 1.0 / 512.0);
    exact(result.peer_dispersion_seconds(), 3.0 / 512.0);
    exact(result.peer_distance_seconds(), 7.0 / 1024.0);
    exact(result.root_distance_seconds(), 7.0 / 1024.0);
    Ok(())
}

/// Precision boundary facts independently specify clamped binary powers.
/// # Errors
/// Preserve construction or evaluation failures.
/// # Panics
/// Panic if either clamp or its immediate interior neighbor changes.
#[test]
fn precision_clamps_have_exact_boundary_and_neighbor_values() -> Result<(), Box<dyn Error>> {
    let declared = assumptions([1_000_000_000_000.0; 3])?;
    for (precision, expected) in [
        (-128, 1.0 / 2_147_483_648.0),
        (-32, 1.0 / 2_147_483_648.0),
        (-31, 1.0 / 2_147_483_648.0),
        (-30, 1.0 / 1_073_741_824.0),
        (30, 1_073_741_824.0),
        (31, 2_147_483_648.0),
        (32, 2_147_483_648.0),
        (127, 2_147_483_648.0),
    ] {
        let facts = facts(LOCAL_NS, SERVER_TICKS, precision, [0, 0])?;
        exact(
            product_ok(
                evaluate_conditional_quality(&facts, &declared),
                "known valid conditional quality evaluation was refused",
            )?
            .peer_precision_seconds(),
            expected,
        );
    }
    Ok(())
}

/// Strict advertised and inclusive delay/response limits have different neighbors.
/// # Errors
/// Preserve checked fixture or evaluator failures for legal neighbors.
/// # Panics
/// Panic if an equality is admitted/refused with the wrong comparison semantics.
#[test]
fn computed_policy_boundaries_preserve_strict_and_inclusive_comparisons()
-> Result<(), Box<dyn Error>> {
    let facts = facts(LOCAL_NS, SERVER_TICKS, -10, [8192, 256])?;
    let advertised = 17.0_f64 / 256.0;
    for maximum in [advertised.next_down(), advertised] {
        let declared = assumptions([maximum, 1.0, 1.0])?;
        assert!(matches!(
            evaluate_conditional_quality(&facts, &declared),
            Err(QualityRefusal::AdvertisedDistance)
        ));
    }
    let declared = assumptions([advertised.next_up(), 1.0, 1.0])?;
    let _accepted = product_ok(
        evaluate_conditional_quality(&facts, &declared),
        "known valid conditional quality evaluation was refused",
    )?;
    let adjusted = 29.0_f64 / 512.0;
    let below = assumptions([1.0, adjusted.next_down(), 1.0])?;
    assert!(matches!(
        evaluate_conditional_quality(&facts, &below),
        Err(QualityRefusal::AdjustedDelay)
    ));
    for maximum in [adjusted, adjusted.next_up()] {
        let declared = assumptions([1.0, maximum, 1.0])?;
        assert!(evaluate_conditional_quality(&facts, &declared).is_ok());
    }
    let response = 1.0_f64 / 16.0;
    let below = assumptions([1.0, 1.0, response.next_down()])?;
    assert!(matches!(
        evaluate_conditional_quality(&facts, &below),
        Err(QualityRefusal::ServerProcessing)
    ));
    for maximum in [response, response.next_up()] {
        let declared = assumptions([1.0, 1.0, maximum])?;
        assert!(evaluate_conditional_quality(&facts, &declared).is_ok());
    }
    // A large capture allowance makes D-E negative, isolating P's own boundary.
    let precision = 1.0_f64 / 512.0;
    for (maximum, accepted) in [
        (precision.next_down(), false),
        (precision, true),
        (precision.next_up(), true),
    ] {
        let declared = DeclaredQualityAssumptions::new(
            1.0 / 1024.0,
            1.0,
            0.0,
            DeclaredFrequencyInterval::new(0.0, 0.0)?,
            DeclaredQualityLimits::new(2.0, maximum, 1.0)?,
        )?;
        let result = evaluate_conditional_quality(&facts, &declared);
        if accepted {
            assert!(result.is_ok());
        } else {
            assert!(matches!(result, Err(QualityRefusal::PrecisionDelay)));
        }
    }
    Ok(())
}

/// Raw assumptions must enter the mandatory constructor, including signed zeros.
/// # Errors
/// Preserve unexpected rejection of accepted neighboring assumptions.
/// # Panics
/// Panic if raw validation/constructor paths disagree or original bits are changed.
#[test]
fn mandatory_assumption_admission_preserves_bits_and_rejects_invalid_fields()
-> Result<(), Box<dyn Error>> {
    let frequency = DeclaredFrequencyInterval::new(-0.0, 0.0);
    assert!(frequency.is_ok());
    let frequency = frequency?;
    let limits = DeclaredQualityLimits::new(1.0, 2.0, 4.0);
    assert!(limits.is_ok());
    let limits = limits?;
    let legal = DeclaredQualityAssumptions::new(f64::from_bits(1), -0.0, 0.0, frequency, limits);
    assert!(legal.is_ok());
    for invalid in [
        0.0,
        -0.0,
        -1.0,
        -f64::from_bits(1),
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ] {
        assert!(positive_rule(&invalid, &()).is_err());
        assert!(matches!(
            DeclaredQualityAssumptions::new(invalid, 0.0, 0.0, frequency, limits),
            Err(QualityRefusal::InvalidAssumption(
                QualityAssumption::HistoricalQuantum
            ))
        ));
        for (values, expected) in [
            (
                [invalid, 1.0, 1.0],
                QualityAssumption::AdvertisedDistanceLimit,
            ),
            ([1.0, invalid, 1.0], QualityAssumption::DelayLimit),
            (
                [1.0, 1.0, invalid],
                QualityAssumption::ServerProcessingLimit,
            ),
        ] {
            assert!(
                matches!(DeclaredQualityLimits::new(values[0], values[1], values[2]),
                Err(QualityRefusal::InvalidAssumption(actual)) if actual == expected)
            );
        }
    }
    for invalid in [
        -1.0,
        -f64::from_bits(1),
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ] {
        assert!(matches!(
            DeclaredQualityAssumptions::new(1.0, invalid, 0.0, frequency, limits),
            Err(QualityRefusal::InvalidAssumption(
                QualityAssumption::TransmitCaptureAllowance
            ))
        ));
        assert!(matches!(
            DeclaredQualityAssumptions::new(1.0, 0.0, invalid, frequency, limits),
            Err(QualityRefusal::InvalidAssumption(
                QualityAssumption::ReceiveCaptureAllowance
            ))
        ));
    }
    for endpoints in [
        [1.0, -1.0],
        [f64::NAN, 0.0],
        [0.0, f64::NAN],
        [f64::NEG_INFINITY, 0.0],
        [0.0, f64::INFINITY],
    ] {
        assert!(frequency_rule(&endpoints, &()).is_err());
        assert!(DeclaredFrequencyInterval { endpoints }.validate().is_err());
        assert!(matches!(
            DeclaredFrequencyInterval::new(endpoints[0], endpoints[1]),
            Err(QualityRefusal::InvalidAssumption(
                QualityAssumption::FrequencyInterval
            ))
        ));
    }
    Ok(())
}

/// Admitted finite operands can still overflow; each failure retains its actual stage.
/// # Errors
/// Preserve checked setup failures.
/// # Panics
/// Panic if a nonfinite computed intermediate reaches max, policy or publication.
#[test]
fn finite_input_overflow_is_refused_at_the_first_computed_stage() -> Result<(), Box<dyn Error>> {
    let ordinary = facts(LOCAL_NS, SERVER_TICKS, -10, [0, 0])?;
    let limits = DeclaredQualityLimits::new(f64::MAX, f64::MAX, f64::MAX)?;
    let frequency_width = DeclaredQualityAssumptions::new(
        1.0,
        0.0,
        0.0,
        DeclaredFrequencyInterval::new(-f64::MAX, f64::MAX)?,
        limits,
    )?;
    assert!(matches!(
        evaluate_conditional_quality(&ordinary, &frequency_width),
        Err(QualityRefusal::NonFinite(QualityStage::FrequencyWidth))
    ));
    let peer_distance = DeclaredQualityAssumptions::new(
        f64::MAX,
        0.0,
        0.0,
        DeclaredFrequencyInterval::new(0.0, 0.0)?,
        limits,
    )?;
    assert!(matches!(
        evaluate_conditional_quality(&ordinary, &peer_distance),
        Err(QualityRefusal::NonFinite(QualityStage::PeerDistance))
    ));
    let long = facts(8_000_000_000, SERVER_TICKS, -10, [0, 0])?;
    let product = DeclaredQualityAssumptions::new(
        1.0,
        0.0,
        0.0,
        DeclaredFrequencyInterval::new(0.0, f64::MAX)?,
        limits,
    )?;
    assert!(matches!(
        evaluate_conditional_quality(&long, &product),
        Err(QualityRefusal::NonFinite(QualityStage::FrequencyDispersion))
    ));
    let two_seconds = facts(2_000_000_000, SERVER_TICKS, -10, [0, 0])?;
    let addition = DeclaredQualityAssumptions::new(
        1.0,
        f64::MAX,
        0.0,
        DeclaredFrequencyInterval::new(0.0, f64::MAX)?,
        limits,
    )?;
    assert!(matches!(
        evaluate_conditional_quality(&two_seconds, &addition),
        Err(QualityRefusal::NonFinite(QualityStage::PeerDispersion))
    ));
    Ok(())
}

/// Gradual underflow follows computed values while original nonzero declarations survive.
/// # Errors
/// Preserve checked construction or evaluation failure.
/// # Panics
/// Panic if subnormal inputs are rejected or computed zeros acquire a negative sign.
#[test]
fn gradual_underflow_does_not_claim_exact_zero_uncertainty() -> Result<(), Box<dyn Error>> {
    let facts = facts(LOCAL_NS, SERVER_TICKS, -31, [0, 0])?;
    let limits = DeclaredQualityLimits::new(1.0, 1.0, 1.0)?;
    for (upper_bits, expected_half_bits) in [(1, 0), (2, 1)] {
        let declared = DeclaredQualityAssumptions::new(
            f64::from_bits(1),
            -0.0,
            0.0,
            DeclaredFrequencyInterval::new(0.0, f64::from_bits(upper_bits))?,
            limits,
        )?;
        let result = product_ok(
            evaluate_conditional_quality(&facts, &declared),
            "known valid conditional quality evaluation was refused",
        )?;
        assert_eq!(result.frequency_half_width().to_bits(), expected_half_bits);
        assert_eq!(
            result
                .declared_assumptions()
                .frequency_interval()
                .upper()
                .to_bits(),
            upper_bits
        );
        exact(result.peer_dispersion_seconds(), 1.0 / 2_147_483_648.0);
    }
    exact(finite(QualityStage::AdjustedDelay, -0.0)?, 0.0);
    assert!(finite_output_rule(&0.0, &()).is_ok());
    assert!(finite_output_rule(&-0.0, &()).is_err());
    Ok(())
}

/// Measurement refusal propagates before quality arithmetic without payload-bearing errors.
/// # Errors
/// Preserve checked fixture setup or healthy neighboring evaluation failures.
/// # Panics
/// Panic if a measurement refusal is replaced, hidden, or formatted with input data.
#[test]
fn original_measurement_refusal_and_redacted_diagnostics_are_preserved()
-> Result<(), Box<dyn Error>> {
    let mut facts = facts(LOCAL_NS, SERVER_TICKS, -10, [8192, 256])?;
    let declared = assumptions([1.0; 3])?;
    let _healthy = product_ok(
        evaluate_conditional_quality(&facts, &declared),
        "known valid conditional quality evaluation was refused",
    )?;
    facts.measurement = Err(MeasurementRefusal::MappingDiscontinuity);
    let result = evaluate_conditional_quality(&facts, &declared);
    assert!(matches!(
        result,
        Err(QualityRefusal::Measurement(
            MeasurementRefusal::MappingDiscontinuity
        ))
    ));
    let error = QualityRefusal::Measurement(MeasurementRefusal::MappingDiscontinuity);
    assert_eq!(
        error
            .source()
            .and_then(|cause| cause.downcast_ref::<MeasurementRefusal>()),
        Some(&MeasurementRefusal::MappingDiscontinuity)
    );
    assert_eq!(
        error.to_string(),
        "conditional quality measurement unavailable"
    );
    assert_eq!(format!("{declared:?}"), "DeclaredQualityAssumptions { .. }");
    assert_eq!(
        format!("{:?}", declared.frequency_interval()),
        "DeclaredFrequencyInterval { .. }"
    );
    assert_eq!(
        format!("{:?}", declared.limits()),
        "DeclaredQualityLimits { .. }"
    );
    Ok(())
}

/// Exact RTT must survive cancellation of separately rounded local/server intervals.
///
/// Independent Fraction arithmetic gives L=1/10, R=429496730/2^32 and
/// L-R=-1/10737418240. Nearest binary64 is 0xbdd999999999999a; subtraction
/// after separately rounding L and R instead gives 0xbdd9999980000000.
/// # Errors
/// Preserve checked fixture construction and evaluation failures.
/// # Panics
/// Panic if the exact corrected numerator or once-rounded cancellation fact changes.
#[test]
fn raw_round_trip_is_converted_once_before_cancellation() -> Result<(), Box<dyn Error>> {
    let facts = facts(100_000_000, 429_496_730, -10, [0, 0])?;
    let declared = assumptions([1.0; 3])?;
    let result = product_ok(
        evaluate_conditional_quality(&facts, &declared),
        "known valid conditional quality evaluation was refused",
    )?;
    assert_eq!(result.raw_round_trip().sign(), RationalSign::Negative);
    assert_eq!(result.raw_round_trip().numerator_magnitude(), 400_000_000);
    assert_eq!(
        result.raw_round_trip().denominator().get(),
        4_294_967_296_000_000_000
    );
    assert_eq!(
        result.raw_round_trip().seconds().to_bits(),
        0xbdd9_9999_9999_999a
    );
    assert_eq!(
        (result.local_interval().seconds() - result.server_interval().seconds()).to_bits(),
        0xbdd9_9999_8000_0000
    );
    assert!(result.raw_round_trip().absolute_roundoff_seconds() > 0.0);
    Ok(())
}

/// Scalar helpers admit their full raw domains independently of checked callers.
/// # Errors
/// Preserve unexpected refusal of supported finite values or exact ratios.
/// # Panics
/// Panic on nonfinite admission, signed-zero changes or lost exact ratio metadata.
#[test]
fn raw_scalar_and_interval_helpers_preserve_full_admitted_domains() -> Result<(), Box<dyn Error>> {
    for (value, signed, nonnegative) in [
        (0.0, true, true),
        (-0.0, false, false),
        (f64::from_bits(1), true, true),
        (-f64::from_bits(1), true, false),
        (1.0, true, true),
        (-1.0, true, false),
        (f64::MAX, true, true),
        (-f64::MAX, true, false),
        (f64::NAN, false, false),
        (f64::INFINITY, false, false),
        (f64::NEG_INFINITY, false, false),
    ] {
        assert_eq!(finite_output_rule(&value, &()).is_ok(), signed);
        assert_eq!(nonnegative_output_rule(&value, &()).is_ok(), nonnegative);
        let actual = finite(QualityStage::AdjustedDelay, value);
        if value.is_finite() {
            exact(actual?, if value == 0.0 { 0.0 } else { value });
        } else {
            assert!(matches!(
                actual,
                Err(QualityRefusal::NonFinite(QualityStage::AdjustedDelay))
            ));
        }
    }
    assert!(matches!(
        interval_seconds(1, 0, QualityInterval::Server),
        Err(QualityRefusal::ExactArithmetic(QualityInterval::Server))
    ));
    for numerator in [i128::MIN, -3, 0, 3, i128::MAX] {
        let result = interval_seconds(numerator, 1, QualityInterval::Local)?;
        assert_eq!(result.numerator_magnitude(), numerator.unsigned_abs());
        assert_eq!(result.denominator().get(), 1);
        assert_eq!(
            result.sign(),
            if numerator < 0 {
                RationalSign::Negative
            } else {
                RationalSign::Positive
            }
        );
        assert!(result.seconds().is_finite());
    }
    Ok(())
}

/// Exercise generated field/dive dispatch, separately from public admission.
///
/// Deliberately altered private raw candidates are macro-validation controls;
/// they are never published as checked assumptions or quality results.
/// # Errors
/// Preserve healthy checked fixture or computation failures.
/// # Panics
/// Panic if a raw field bypasses its specified generated rule or nested dispatch.
#[test]
fn generated_garde_dispatch_covers_nested_and_computed_fields() -> Result<(), Box<dyn Error>> {
    let facts = facts(LOCAL_NS, SERVER_TICKS, -10, [8192, 256])?;
    let declared = assumptions([1.0; 3])?;
    assert!(declared.validate().is_ok());
    let mut bad_frequency = declared;
    bad_frequency.frequency.endpoints = [0.0, f64::NAN];
    assert!(bad_frequency.validate().is_err());
    let mut bad_limits = declared;
    bad_limits.limits.delay = f64::NAN;
    assert!(bad_limits.validate().is_err());
    let intervals = product_ok(
        exact_intervals(&facts),
        "known valid quality interval extraction was refused",
    )?;
    for (value, positive, nonnegative, signed) in [
        (1.0, true, true, true),
        (f64::from_bits(1), true, true, true),
        (0.0, false, true, true),
        (-0.0, false, false, false),
        (-f64::from_bits(1), false, false, true),
        (f64::NAN, false, false, false),
        (f64::INFINITY, false, false, false),
        (f64::NEG_INFINITY, false, false, false),
    ] {
        for index in 0..13 {
            let mut candidate = product_ok(
                compute_quality(&facts, &declared, &intervals),
                "known valid quality computation was refused",
            )?;
            assert!(candidate.validate().is_ok());
            let (field, expected) = match index {
                0 => (&mut candidate.peer_precision, positive),
                1 => (&mut candidate.combined_precision, positive),
                2 => (&mut candidate.adjusted_delay, positive),
                3 => (&mut candidate.frequency_half_width, nonnegative),
                4 => (&mut candidate.peer_dispersion, positive),
                5 => (&mut candidate.advertised_root_delay, nonnegative),
                6 => (&mut candidate.advertised_root_dispersion, nonnegative),
                7 => (&mut candidate.advertised_distance, nonnegative),
                8 => (&mut candidate.root_delay, positive),
                9 => (&mut candidate.root_dispersion, positive),
                10 => (&mut candidate.peer_distance, positive),
                11 => (&mut candidate.root_distance, positive),
                12 => (&mut candidate.delay_minus_dispersion, signed),
                _ => return Err("unknown computed field in bounded test".into()),
            };
            *field = value;
            assert_eq!(
                candidate.validate().is_ok(),
                expected,
                "computed field {index}"
            );
        }
    }
    Ok(())
}

/// Competing failures retain the specified causal order and original observation.
/// # Errors
/// Preserve checked setup or the healthy neighboring evaluation.
/// # Panics
/// Panic if lower-priority arithmetic/policy failures hide the first refusal.
#[test]
fn evaluator_failure_precedence_preserves_original_observation() -> Result<(), Box<dyn Error>> {
    let facts = facts(LOCAL_NS, SERVER_TICKS, -10, [8192, 256])?;
    let original_age = product_ok(
        facts.measurement(),
        "known successful quality fixture measurement was lost",
    )?
    .acquisition_age();
    let overflow = DeclaredQualityAssumptions::new(
        1.0 / 1024.0,
        0.0,
        0.0,
        DeclaredFrequencyInterval::new(-f64::MAX, f64::MAX)?,
        DeclaredQualityLimits::new(1.0 / 1024.0, 1.0 / 1024.0, 1.0 / 128.0)?,
    )?;
    assert!(matches!(
        evaluate_conditional_quality(&facts, &overflow),
        Err(QualityRefusal::NonFinite(QualityStage::FrequencyWidth))
    ));
    for (limits, first) in [
        (
            [17.0 / 256.0, 1.0 / 1024.0, 1.0 / 128.0],
            QualityRefusal::AdvertisedDistance,
        ),
        (
            [1.0, 1.0 / 1024.0, 1.0 / 128.0],
            QualityRefusal::PrecisionDelay,
        ),
        (
            [1.0, 1.0 / 64.0, 1.0 / 128.0],
            QualityRefusal::AdjustedDelay,
        ),
        ([1.0, 1.0, 1.0 / 128.0], QualityRefusal::ServerProcessing),
    ] {
        let declared = assumptions(limits)?;
        assert!(
            matches!(evaluate_conditional_quality(&facts, &declared), Err(actual) if actual == first)
        );
    }
    let legal = assumptions([1.0; 3])?;
    let _accepted = product_ok(
        evaluate_conditional_quality(&facts, &legal),
        "known valid conditional quality evaluation was refused",
    )?;
    assert_eq!(
        product_ok(
            facts.measurement(),
            "known successful quality fixture measurement was lost"
        )?
        .acquisition_age(),
        original_age
    );
    let mut unavailable = facts;
    unavailable.profile = None;
    unavailable.measurement = measure(
        None,
        unavailable.send_before,
        unavailable.send_after,
        unavailable.read_return,
        unavailable.processing,
        unavailable.header,
    );
    assert!(matches!(
        evaluate_conditional_quality(&unavailable, &overflow),
        Err(QualityRefusal::Measurement(MeasurementRefusal::Unavailable))
    ));
    Ok(())
}

/// Direct error values qualify diagnostics only, not evaluator reachability.
/// # Panics
/// Panic if fixed bounded messages gain payloads or actual cause types are lost.
#[test]
fn refusal_families_keep_bounded_diagnostics_and_typed_sources() {
    let rational = QualityRefusal::Rational {
        interval: QualityInterval::Local,
        source: RationalSecondsError::ArithmeticInvariant,
    };
    assert_eq!(
        rational
            .source()
            .and_then(|cause| cause.downcast_ref::<RationalSecondsError>()),
        Some(&RationalSecondsError::ArithmeticInvariant)
    );
    for error in [
        QualityRefusal::InvalidAssumption(QualityAssumption::HistoricalQuantum),
        QualityRefusal::Measurement(MeasurementRefusal::Unavailable),
        QualityRefusal::ExactArithmetic(QualityInterval::Local),
        rational,
        QualityRefusal::NonFinite(QualityStage::FrequencyWidth),
        QualityRefusal::AdvertisedDistance,
        QualityRefusal::PrecisionDelay,
        QualityRefusal::AdjustedDelay,
        QualityRefusal::ServerProcessing,
        QualityRefusal::OutputInvariant,
        QualityRefusal::ValidationInvariant,
    ] {
        let text = error.to_string();
        assert!(text.starts_with("conditional quality ") && text.len() <= 80 && text.is_ascii());
        assert!(!text.bytes().any(|byte| byte.is_ascii_control()));
        assert_eq!(
            error.source().is_some(),
            matches!(
                error,
                QualityRefusal::Measurement(_) | QualityRefusal::Rational { .. }
            )
        );
    }
}

/// Preserve public declaration getters and deterministic first-invalid precedence.
/// # Errors
/// Preserve unexpected rejection of explicit valid declaration values.
/// # Panics
/// Panic if original bits or the first invalid field identity changes.
#[test]
fn declared_getters_and_constructor_precedence_preserve_original_bits() -> Result<(), Box<dyn Error>>
{
    let frequency = DeclaredFrequencyInterval::new(-0.0, 0.0)?;
    let limits = DeclaredQualityLimits::new(1.0, 2.0, 4.0)?;
    let legal = DeclaredQualityAssumptions::new(f64::from_bits(1), -0.0, 0.0, frequency, limits)?;
    assert_eq!(legal.historical_local_quantum_seconds().to_bits(), 1);
    assert_eq!(
        legal.transmit_capture_allowance_seconds().to_bits(),
        (-0.0_f64).to_bits()
    );
    assert_eq!(legal.receive_capture_allowance_seconds().to_bits(), 0);
    assert_eq!(
        legal.frequency_interval().lower().to_bits(),
        (-0.0_f64).to_bits()
    );
    assert_eq!(legal.frequency_interval().upper().to_bits(), 0);
    assert_eq!(
        legal
            .limits()
            .maximum_advertised_distance_seconds()
            .to_bits(),
        1.0_f64.to_bits()
    );
    assert_eq!(
        legal.limits().maximum_delay_seconds().to_bits(),
        2.0_f64.to_bits()
    );
    assert_eq!(
        legal.limits().maximum_server_processing_seconds().to_bits(),
        4.0_f64.to_bits()
    );
    let distinct_limits = DeclaredQualityLimits::new(8.0, 2.0, 4.0)?;
    assert_eq!(
        distinct_limits
            .maximum_advertised_distance_seconds()
            .to_bits(),
        8.0_f64.to_bits()
    );
    for (quantum, transmit, receive, first) in [
        (0.0, -1.0, -1.0, QualityAssumption::HistoricalQuantum),
        (1.0, -1.0, -1.0, QualityAssumption::TransmitCaptureAllowance),
    ] {
        assert!(matches!(
            DeclaredQualityAssumptions::new(quantum, transmit, receive, frequency, limits),
            Err(QualityRefusal::InvalidAssumption(actual)) if actual == first
        ));
    }
    for (values, first) in [
        ([0.0, 0.0, 0.0], QualityAssumption::AdvertisedDistanceLimit),
        ([1.0, 0.0, 0.0], QualityAssumption::DelayLimit),
    ] {
        assert!(matches!(
            DeclaredQualityLimits::new(values[0], values[1], values[2]),
            Err(QualityRefusal::InvalidAssumption(actual)) if actual == first
        ));
    }
    Ok(())
}

//! Independent S1.1d public-consumer tests for CHECK-6/7/25.
//!
//! Packet literals, exact arithmetic facts and the transition table come from
//! tasks/s1.1-exchange-association.md and s1.1-exchange-vectors.json. No product
//! reducer, parser or arithmetic helper supplies the expected results. These
//! fixture-only tests establish no transport, authentication or native behavior.
#![forbid(unsafe_code)]

use std::{error::Error, io, net::SocketAddr, num::NonZeroU64};

use ntp_exchange::{
    ArmError, Association, AssociationContext, AssociationGeneration, Binding, CancelError,
    ClockGeneration, CompletionFacts, ConfigurationGeneration, ContextError, ExpireError,
    FixtureEvent, GenerationError, Generations, PeerEraContext, ReceiptFacts, ReceiveError,
    RequestId, RequestTiming, SocketId, SourceId, TimingError, TokenError,
    UnauthenticatedFixtureObservation, WireToken,
};
use time_domain::{DomainError, EraWindow, MonotonicNs, UtcCoordinateQ32};

#[path = "support/vectors.rs"]
mod vectors;

/// Unexpected fixture construction or consumer-variant failures propagate.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;

/// Exact default saved T1, independently specified as era-zero Q32 bits.
const SAVED_T1: i128 = 0xe000_0000_0000_0000;
/// Exact default local receive coordinate, independent of monotonic time.
const LOCAL_T4: i128 = 0xe000_0000_1c28_f5c2;
/// Nonrandom default correlation bits; not an entropy claim.
const ORIGIN: u64 = 0xe000_0000_0000_0000;
/// JSON fixture's one-million microseconds, exactly converted to nanoseconds.
const SEND: u64 = 1_000_000_000;
/// JSON fixture's 1,110,000 microseconds, exactly converted to nanoseconds.
const ARRIVAL: u64 = 1_110_000_000;
/// JSON fixture's two-million microseconds, exactly converted to nanoseconds.
const DEADLINE: u64 = 2_000_000_000;

/// Construct a positive fixture identifier without unchecked conversion.
/// # Errors
/// Rejects a zero fixture identifier rather than hiding a setup failure.
fn nonzero(value: u64) -> TestResult<NonZeroU64> {
    Ok(NonZeroU64::new(value).ok_or_else(|| io::Error::other("zero fixture ID"))?)
}

/// Name a nonzero request in the public identity domain.
/// # Errors
/// Propagates an invalid zero fixture ID.
fn request(value: u64) -> TestResult<RequestId> {
    Ok(RequestId::new(nonzero(value)?))
}

/// Map fixture-A/socket-1 to explicit numeric identities and a documentation IP.
/// # Errors
/// Propagates an invalid fixed endpoint or identifier.
fn binding(source: u64, socket: u64, endpoint: &str) -> TestResult<Binding> {
    Ok(Binding::new(
        SourceId::new(nonzero(source)?),
        SocketId::new(nonzero(socket)?),
        endpoint.parse::<SocketAddr>()?,
    ))
}

/// Preserve three independently named generation components without derivation.
fn generations(configuration: u64, association: u64, clock: u64) -> Generations {
    Generations::new(
        ConfigurationGeneration::new(configuration),
        AssociationGeneration::new(association),
        ClockGeneration::new(clock),
    )
}

/// Establish the default fixture binding and literal generation tuple.
/// # Errors
/// Propagates invalid fixture data.
fn context() -> TestResult<AssociationContext> {
    Ok(AssociationContext::new(
        binding(1, 1, "192.0.2.1:123")?,
        generations(1, 1, 1),
    ))
}

/// Supply the explicit era-zero fixture interval, not an ambient-date pivot.
/// # Errors
/// Propagates invalid numeric fixture windows.
fn eras() -> TestResult<PeerEraContext> {
    let window = EraWindow::new(0, 18_446_744_073_709_551_615)?;
    Ok(PeerEraContext::new(window, window))
}

/// Construct the fixture's exact one-second monotonic lifetime.
/// # Errors
/// Propagates unexpected timing rejection.
fn timing() -> TestResult<RequestTiming> {
    Ok(RequestTiming::new(
        MonotonicNs::new(SEND),
        MonotonicNs::new(DEADLINE),
    )?)
}

/// Supply the complete explicit default receipt facts for the given context.
/// # Errors
/// Propagates unexpected coordinate rejection.
fn receipt(current: AssociationContext) -> TestResult<ReceiptFacts> {
    Ok(ReceiptFacts::new(
        current.binding(),
        current.generations().clock(),
        MonotonicNs::new(ARRIVAL),
        MonotonicNs::new(ARRIVAL),
        UtcCoordinateQ32::new(LOCAL_T4)?,
    ))
}

/// Start the real public association with one explicit saved-T1 request.
/// # Errors
/// Propagates unexpected construction or arm rejection.
/// # Panics
/// Fails if the first admitted request identity differs from the fixture ID.
fn armed(token: u64) -> TestResult<Association> {
    let mut owner = Association::new_plain_header_fixture(context()?, request(1)?);
    let allocated = owner.arm(
        UtcCoordinateQ32::new(SAVED_T1)?,
        WireToken::new(token)?,
        timing()?,
        eras()?,
    )?;
    assert_eq!(allocated, request(1)?);
    Ok(owner)
}

/// Consume the actual result variant without inventing a completed observation.
/// # Errors
/// Reports a control event where the independent scenario requires time.
fn time(event: FixtureEvent) -> TestResult<UnauthenticatedFixtureObservation> {
    match event {
        FixtureEvent::Time(observation) => Ok(observation),
        FixtureEvent::Control(_) => Err(io::Error::other("expected time event").into()),
    }
}

/// Check all committed source/generation/timing facts against fixture truth.
/// # Errors
/// Propagates unexpected fixed fixture construction failure.
/// # Panics
/// Fails for omitted, swapped or normalized completion facts.
fn assert_facts(facts: &CompletionFacts, expected_request: u64) -> TestResult {
    assert_eq!(facts.binding(), context()?.binding());
    assert_eq!(facts.request_id(), request(expected_request)?);
    assert_eq!(facts.generations(), generations(1, 1, 1));
    assert_eq!(facts.mapping_generation(), ClockGeneration::new(1));
    assert_eq!(facts.send().nanoseconds(), SEND);
    assert_eq!(facts.arrival().nanoseconds(), ARRIVAL);
    assert_eq!(facts.processing_now().nanoseconds(), ARRIVAL);
    assert_eq!(facts.deadline().nanoseconds(), DEADLINE);
    Ok(())
}

/// Assert the default raw quality claims without treating them as admission.
/// # Panics
/// Fails when any named wire-quality field is omitted or transformed.
fn assert_default_quality(observation: &UnauthenticatedFixtureObservation) {
    assert_eq!(observation.leap_indicator(), 0);
    assert_eq!(observation.stratum(), 2);
    assert_eq!(observation.poll(), 6);
    assert_eq!(observation.precision(), -20);
    assert_eq!(observation.root_delay_bits(), 0);
    assert_eq!(observation.root_dispersion_bits(), 0);
    assert_eq!(observation.reference_id(), 0x4750_5300);
    assert_eq!(
        observation.reference_timestamp_bits(),
        0xdfff_ffff_0000_0000
    );
}

/// Prove a rejection preserves the exact pending identity and sequence, then
/// execute a healthy literal through the same owner to exclude reject-all.
/// # Errors
/// Propagates unexpected fixture or receive failure.
/// # Panics
/// Fails for changed context/slot/sequence, incorrect error or healthy arithmetic.
fn reject_then_healthy(packet: &[u8], expected: ReceiveError) -> TestResult {
    let mut owner = armed(ORIGIN)?;
    let before_context = owner.current_context();
    let before_next = owner.next_request_id();
    assert_eq!(owner.receive(packet, receipt(context()?)?), Err(expected));
    assert_eq!(owner.current_context(), before_context);
    assert_eq!(owner.pending_request_id(), Some(request(1)?));
    assert_eq!(owner.next_request_id(), before_next);
    let observed = time(owner.receive(&vectors::ORDINARY_SYMMETRIC, receipt(context()?)?)?)?;
    assert_eq!(observed.offset().numerator(), 171_798_691);
    assert_eq!(observed.signed_round_trip().numerator(), 429_496_729);
    assert_eq!(owner.pending_request_id(), None);
    Ok(())
}

/// Public checked constructors preserve role identity and exact immutable facts.
/// # Errors
/// Propagates unexpected fixture construction failure.
/// # Panics
/// Fails on any getter swap, forbidden normalization or missing range rejection.
#[test]
fn legal_consumer_preserves_distinct_input_roles() -> TestResult {
    let named = binding(7, 11, "[2001:db8::7]:9123")?;
    assert_eq!(named.source_id().value().get(), 7);
    assert_eq!(named.socket_id().value().get(), 11);
    assert_eq!(
        named.endpoint(),
        "[2001:db8::7]:9123".parse::<SocketAddr>()?
    );
    assert_eq!(request(13)?.value().get(), 13);
    let versions = generations(17, 19, 23);
    assert_eq!(versions.configuration().value(), 17);
    assert_eq!(versions.association().value(), 19);
    assert_eq!(versions.clock().value(), 23);
    let current = AssociationContext::new(named, versions);
    assert_eq!(current.binding(), named);
    assert_eq!(current.generations(), versions);
    let sent = MonotonicNs::new(29);
    let deadline = MonotonicNs::new(31);
    let lifetime = RequestTiming::new(sent, deadline)?;
    assert_eq!(lifetime.send(), sent);
    assert_eq!(lifetime.deadline(), deadline);
    let receive_window = EraWindow::new(37, 41)?;
    let transmit_window = EraWindow::new(43, 47)?;
    let windows = PeerEraContext::new(receive_window, transmit_window);
    assert_eq!(windows.receive_window(), receive_window);
    assert_eq!(windows.transmit_window(), transmit_window);
    let facts = ReceiptFacts::new(
        named,
        ClockGeneration::new(53),
        MonotonicNs::new(59),
        MonotonicNs::new(61),
        UtcCoordinateQ32::new(67)?,
    );
    assert_eq!(facts.binding(), named);
    assert_eq!(facts.mapping_generation().value(), 53);
    assert_eq!(facts.arrival().nanoseconds(), 59);
    assert_eq!(facts.processing_now().nanoseconds(), 61);
    assert_eq!(facts.local_t4().ticks(), 67);
    Ok(())
}

/// Zero and saturated generations are legal values; increments never wrap.
/// # Errors
/// Propagates unexpected valid generation increment rejection.
/// # Panics
/// Fails for off-by-one increments or wrap at MAX.
#[test]
fn checked_generation_endpoints_do_not_wrap() -> TestResult {
    for value in [0, 1, u64::MAX - 1] {
        assert_eq!(
            ConfigurationGeneration::new(value).checked_next()?.value(),
            value + 1
        );
        assert_eq!(
            AssociationGeneration::new(value).checked_next()?.value(),
            value + 1
        );
        assert_eq!(
            ClockGeneration::new(value).checked_next()?.value(),
            value + 1
        );
    }
    assert_eq!(
        ConfigurationGeneration::new(u64::MAX).checked_next(),
        Err(GenerationError::GenerationExhausted)
    );
    assert_eq!(
        AssociationGeneration::new(u64::MAX).checked_next(),
        Err(GenerationError::GenerationExhausted)
    );
    assert_eq!(
        ClockGeneration::new(u64::MAX).checked_next(),
        Err(GenerationError::GenerationExhausted)
    );
    Ok(())
}

/// Timing constructor expectations use the independently fixed 1e9 ns ceiling.
/// # Errors
/// Propagates unexpected positive, in-budget lifetime rejection.
/// # Panics
/// Fails for equal/reverse/over-budget acceptance or legal endpoint rejection.
#[test]
fn request_lifetime_exact_neighbors() -> TestResult {
    for (send, deadline) in [
        (0, 1),
        (0, 999_999_999),
        (0, 1_000_000_000),
        (u64::MAX - 1, u64::MAX),
    ] {
        let value = RequestTiming::new(MonotonicNs::new(send), MonotonicNs::new(deadline))?;
        assert_eq!(value.send().nanoseconds(), send);
        assert_eq!(value.deadline().nanoseconds(), deadline);
    }
    for (send, deadline) in [(0, 0), (1, 1), (1, 0), (u64::MAX, 0)] {
        assert_eq!(
            RequestTiming::new(MonotonicNs::new(send), MonotonicNs::new(deadline)),
            Err(TimingError::NonPositiveLifetime)
        );
    }
    for (send, deadline) in [(0, 1_000_000_001), (0, u64::MAX)] {
        assert_eq!(
            RequestTiming::new(MonotonicNs::new(send), MonotonicNs::new(deadline)),
            Err(TimingError::LifetimeExceeded)
        );
    }
    Ok(())
}

/// Exact independent arithmetic survives unrelated origin correlation bits.
/// # Errors
/// Propagates unexpected valid request/packet rejection.
/// # Panics
/// Fails for wrong coordinates, arithmetic, quality, facts or repeated completion.
#[test]
fn reviewed_time_literals_preserve_saved_t1_and_exact_outputs() -> TestResult {
    for (packet, token, t2, t3, offset) in [
        (
            vectors::ORDINARY_SYMMETRIC,
            ORIGIN,
            0xe000_0000_11eb_851e_i128,
            0xe000_0000_147a_e147_i128,
            171_798_691,
        ),
        (
            vectors::RANDOMIZED_WIRE_TOKEN,
            0x0102_0304_0506_0708,
            0xe000_0000_11eb_851e,
            0xe000_0000_147a_e147,
            171_798_691,
        ),
        (
            vectors::ASYMMETRIC_PATH,
            ORIGIN,
            0xe000_0000_0ccc_cccc,
            0xe000_0000_0f5c_28f5,
            -1,
        ),
    ] {
        let mut owner = armed(token)?;
        let observed = time(owner.receive(&packet, receipt(context()?)?)?)?;
        assert_facts(observed.facts(), 1)?;
        assert_default_quality(&observed);
        assert_eq!(observed.saved_real_t1().ticks(), SAVED_T1);
        assert_eq!(observed.peer_t2().ticks(), t2);
        assert_eq!(observed.peer_t3().ticks(), t3);
        assert_eq!(observed.local_t4().ticks(), LOCAL_T4);
        assert_eq!(observed.offset().numerator(), offset);
        assert_eq!(observed.signed_round_trip().numerator(), 429_496_729);
        assert_eq!(owner.pending_request_id(), None);
        assert_eq!(owner.next_request_id(), Some(request(2)?));
    }
    Ok(())
}

/// All eight unchanged rejected literals retain a useful current request.
/// # Errors
/// Propagates unexpected construction or healthy-neighbor rejection.
/// # Panics
/// Fails when a raw rejection reason, slot preservation or neighbor differs.
#[test]
fn reviewed_rejection_literals_preserve_pending_request() -> TestResult {
    use ntp_wire::FrameError;
    for (packet, expected) in [
        (
            vectors::TRUNCATED_47.as_slice(),
            ReceiveError::Framing(FrameError::HeaderTooShort),
        ),
        (
            vectors::HEADER_ONLY_OVER_49.as_slice(),
            ReceiveError::Framing(FrameError::ByteLimit),
        ),
        (
            vectors::CLIENT_MODE.as_slice(),
            ReceiveError::UnsupportedMode,
        ),
        (
            vectors::OLD_VERSION.as_slice(),
            ReceiveError::UnsupportedVersion,
        ),
        (
            vectors::UNSYNCHRONIZED_LEAP.as_slice(),
            ReceiveError::UnsynchronizedLeap,
        ),
        (vectors::STRATUM_16.as_slice(), ReceiveError::InvalidStratum),
        (
            vectors::ZERO_RECEIVE.as_slice(),
            ReceiveError::ZeroReceiveTimestamp,
        ),
        (
            vectors::WRONG_ORIGIN.as_slice(),
            ReceiveError::OriginMismatch,
        ),
    ] {
        reject_then_healthy(packet, expected)?;
    }
    Ok(())
}

/// Complete framing rejects every short length and bounded long neighbors.
/// # Errors
/// Propagates unexpected setup or healthy receive failure.
/// # Panics
/// Fails if framing admits an unsupported complete packet or consumes a slot.
#[test]
fn complete_packet_length_boundaries() -> TestResult {
    for length in 0..48 {
        reject_then_healthy(
            &vectors::ORDINARY_SYMMETRIC[..length],
            ReceiveError::Framing(ntp_wire::FrameError::HeaderTooShort),
        )?;
    }
    for length in [49, 64, 4096] {
        let mut oversized = vec![0_u8; length];
        oversized[..48].copy_from_slice(&vectors::ORDINARY_SYMMETRIC);
        reject_then_healthy(
            &oversized,
            ReceiveError::Framing(ntp_wire::FrameError::ByteLimit),
        )?;
    }
    Ok(())
}

/// Replaying the exact literal requires real completion, never a fixture flag.
/// # Errors
/// Propagates unexpected valid packet rejection.
/// # Panics
/// Fails on an event after completed state or a reset request sequence.
#[test]
fn second_response_literal_follows_real_completion() -> TestResult {
    let mut owner = armed(ORIGIN)?;
    let first = time(owner.receive(&vectors::SECOND_RESPONSE_SAME_REQUEST, receipt(context()?)?)?)?;
    assert_eq!(first.offset().numerator(), 171_798_691);
    assert_eq!(
        owner.receive(&vectors::SECOND_RESPONSE_SAME_REQUEST, receipt(context()?)?),
        Err(ReceiveError::NoPending)
    );
    assert_eq!(owner.next_request_id(), Some(request(2)?));
    assert_eq!(owner.pending_request_id(), None);
    Ok(())
}

/// Checked zero/one token neighbors are challenged through the complete receive.
/// # Errors
/// Propagates unexpected valid token/request/packet rejection.
/// # Panics
/// Fails on zero construction, zero-origin completion or substituted wire T1.
#[test]
fn zero_origin_rejects_and_token_one_preserves_saved_t1() -> TestResult {
    assert_eq!(WireToken::new(0), Err(TokenError::Zero));
    let mut owner = armed(1)?;
    let mut packet = vectors::ORDINARY_SYMMETRIC;
    packet[24..32].copy_from_slice(&0_u64.to_be_bytes());
    assert_eq!(
        owner.receive(&packet, receipt(context()?)?),
        Err(ReceiveError::ZeroOrigin)
    );
    assert_eq!(owner.pending_request_id(), Some(request(1)?));
    packet[24..32].copy_from_slice(&1_u64.to_be_bytes());
    let observed = time(owner.receive(&packet, receipt(context()?)?)?)?;
    assert_eq!(observed.saved_real_t1().ticks(), SAVED_T1);
    assert_eq!(observed.offset().numerator(), 171_798_691);
    assert_eq!(observed.signed_round_trip().numerator(), 429_496_729);
    assert_eq!(
        owner.receive(&packet, receipt(context()?)?),
        Err(ReceiveError::NoPending)
    );
    Ok(())
}

/// Every nonzero representation endpoint remains correlation data only.
/// # Errors
/// Propagates unexpected checked-token or healthy receive rejection.
/// # Panics
/// Fails for token normalization or altered numerical truth.
#[test]
fn nonzero_token_extremes_are_admitted_without_normalization() -> TestResult {
    for token in [1, 2, u64::MAX - 1, u64::MAX] {
        let mut owner = armed(token)?;
        let mut packet = vectors::ORDINARY_SYMMETRIC;
        packet[24..32].copy_from_slice(&token.to_be_bytes());
        let observed = time(owner.receive(&packet, receipt(context()?)?)?)?;
        assert_eq!(observed.offset().numerator(), 171_798_691);
        assert_eq!(observed.signed_round_trip().numerator(), 429_496_729);
    }
    Ok(())
}

/// Actual receipt identities reject individually while leaving a healthy slot.
/// # Errors
/// Propagates unexpected fixture or healthy-neighbor failure.
/// # Panics
/// Fails if a source/socket/address/port/mapping mismatch completes or retires.
#[test]
fn receipt_binding_and_mapping_are_checked_independently() -> TestResult {
    for (received, mapping, expected) in [
        (
            binding(2, 1, "192.0.2.1:123")?,
            1,
            ReceiveError::SourceMismatch,
        ),
        (
            binding(1, 2, "192.0.2.1:123")?,
            1,
            ReceiveError::SocketMismatch,
        ),
        (
            binding(1, 1, "192.0.2.2:123")?,
            1,
            ReceiveError::EndpointMismatch,
        ),
        (
            binding(1, 1, "192.0.2.1:124")?,
            1,
            ReceiveError::EndpointMismatch,
        ),
        (
            binding(1, 1, "[2001:db8::1]:123")?,
            1,
            ReceiveError::EndpointMismatch,
        ),
        (
            context()?.binding(),
            0,
            ReceiveError::MappingGenerationMismatch,
        ),
        (
            context()?.binding(),
            2,
            ReceiveError::MappingGenerationMismatch,
        ),
    ] {
        let mut owner = armed(ORIGIN)?;
        let wrong = ReceiptFacts::new(
            received,
            ClockGeneration::new(mapping),
            MonotonicNs::new(ARRIVAL),
            MonotonicNs::new(ARRIVAL),
            UtcCoordinateQ32::new(LOCAL_T4)?,
        );
        assert_eq!(
            owner.receive(&vectors::WRONG_ENDPOINT, wrong),
            Err(expected)
        );
        assert_eq!(owner.pending_request_id(), Some(request(1)?));
        assert_eq!(owner.next_request_id(), Some(request(2)?));
        assert_facts(
            time(owner.receive(&vectors::ORDINARY_SYMMETRIC, receipt(context()?)?)?)?.facts(),
            1,
        )?;
    }
    Ok(())
}

/// Boundary truth is send <= arrival <= processing < deadline, in nanoseconds.
/// # Errors
/// Propagates unexpected valid-neighbor rejection.
/// # Panics
/// Fails on an inclusivity error or any rejected-packet retirement.
#[test]
fn receive_timing_exact_neighbors() -> TestResult {
    for (arrival, processing, expected) in [
        (SEND - 1, SEND, Some(ReceiveError::ArrivalBeforeSend)),
        (SEND, SEND, None),
        (SEND + 1, SEND + 1, None),
        (
            ARRIVAL,
            ARRIVAL - 1,
            Some(ReceiveError::ProcessingBeforeArrival),
        ),
        (ARRIVAL, ARRIVAL, None),
        (ARRIVAL, ARRIVAL + 1, None),
        (DEADLINE - 1, DEADLINE - 1, None),
        (DEADLINE - 1, DEADLINE, Some(ReceiveError::DeadlineReached)),
        (
            DEADLINE - 1,
            DEADLINE + 1,
            Some(ReceiveError::DeadlineReached),
        ),
        (DEADLINE, DEADLINE, Some(ReceiveError::DeadlineReached)),
    ] {
        let mut owner = armed(ORIGIN)?;
        let facts = ReceiptFacts::new(
            context()?.binding(),
            ClockGeneration::new(1),
            MonotonicNs::new(arrival),
            MonotonicNs::new(processing),
            UtcCoordinateQ32::new(LOCAL_T4)?,
        );
        let result = owner.receive(&vectors::ORDINARY_SYMMETRIC, facts);
        if let Some(error) = expected {
            assert_eq!(result, Err(error));
            assert_eq!(owner.pending_request_id(), Some(request(1)?));
            assert!(
                owner
                    .receive(&vectors::ORDINARY_SYMMETRIC, receipt(context()?)?)
                    .is_ok()
            );
        } else {
            let observed = time(result?)?;
            assert_eq!(observed.facts().arrival().nanoseconds(), arrival);
            assert_eq!(observed.facts().processing_now().nanoseconds(), processing);
            assert_eq!(owner.pending_request_id(), None);
        }
    }
    Ok(())
}

/// LI 0..2 and strata 1..15 remain raw claims, not authenticated quality.
/// # Errors
/// Propagates unexpected legal-time rejection.
/// # Panics
/// Fails on boundary rejection, quality normalization or zero transmit acceptance.
#[test]
fn time_quality_boundaries_and_zero_transmit() -> TestResult {
    for leap in 0_u8..=2 {
        for stratum in [1, 2, 15] {
            let mut owner = armed(ORIGIN)?;
            let mut packet = vectors::ORDINARY_SYMMETRIC;
            packet[0] = (leap << 6) | 0x24;
            packet[1] = stratum;
            let observed = time(owner.receive(&packet, receipt(context()?)?)?)?;
            assert_eq!(observed.leap_indicator(), leap);
            assert_eq!(observed.stratum(), stratum);
            assert_eq!(observed.offset().numerator(), 171_798_691);
        }
    }
    for stratum in [16, 17, 255] {
        let mut packet = vectors::ORDINARY_SYMMETRIC;
        packet[1] = stratum;
        reject_then_healthy(&packet, ReceiveError::InvalidStratum)?;
    }
    let mut zero_transmit = vectors::ORDINARY_SYMMETRIC;
    zero_transmit[40..48].copy_from_slice(&[0; 8]);
    reject_then_healthy(&zero_transmit, ReceiveError::ZeroTransmitTimestamp)?;
    Ok(())
}

/// Distinct signed/raw claims discriminate every public quality-field projection.
/// # Errors
/// Propagates unexpected valid-time rejection.
/// # Panics
/// Fails for omitted, swapped or normalized raw quality bits.
#[test]
fn observation_retains_exact_nondefault_quality_fields() -> TestResult {
    for (poll, precision) in [(i8::MIN, i8::MAX), (i8::MAX, i8::MIN)] {
        let mut packet = vectors::ORDINARY_SYMMETRIC;
        packet[0] = 0xa4;
        packet[1] = 15;
        packet[2] = poll.to_be_bytes()[0];
        packet[3] = precision.to_be_bytes()[0];
        packet[4..8].copy_from_slice(&0x8000_0001_u32.to_be_bytes());
        packet[8..12].copy_from_slice(&0xffff_fffe_u32.to_be_bytes());
        packet[12..16].copy_from_slice(&0xa1b2_c3d4_u32.to_be_bytes());
        packet[16..24].copy_from_slice(&0x1020_3040_5060_7080_u64.to_be_bytes());
        let mut owner = armed(ORIGIN)?;
        let observed = time(owner.receive(&packet, receipt(context()?)?)?)?;
        assert_eq!(observed.leap_indicator(), 2);
        assert_eq!(observed.stratum(), 15);
        assert_eq!(observed.poll(), poll);
        assert_eq!(observed.precision(), precision);
        assert_eq!(observed.root_delay_bits(), 0x8000_0001);
        assert_eq!(observed.root_dispersion_bits(), 0xffff_fffe);
        assert_eq!(observed.reference_id(), 0xa1b2_c3d4);
        assert_eq!(observed.reference_timestamp_bits(), 0x1020_3040_5060_7080);
        assert_eq!(observed.offset().numerator(), 171_798_691);
    }
    Ok(())
}

/// The unchanged LI=0 RATE literal completes only an opaque once-only control.
/// # Errors
/// Propagates unexpected control or fresh-request failure.
/// # Panics
/// Fails for a time event, wrong code/facts, replay event or automatic rearm.
#[test]
fn kod_rate_literal_is_terminal_control_then_fresh_request_works() -> TestResult {
    let mut owner = armed(ORIGIN)?;
    let event = owner.receive(&vectors::KOD_RATE, receipt(context()?)?)?;
    let FixtureEvent::Control(control) = event else {
        return Err(io::Error::other("RATE emitted a time observation").into());
    };
    assert_eq!(control.code_bits(), 0x5241_5445);
    assert_facts(control.facts(), 1)?;
    assert_eq!(owner.pending_request_id(), None);
    assert_eq!(
        owner.receive(&vectors::KOD_RATE, receipt(context()?)?),
        Err(ReceiveError::NoPending)
    );
    assert_eq!(
        owner.arm(
            UtcCoordinateQ32::new(SAVED_T1)?,
            WireToken::new(1)?,
            timing()?,
            eras()?
        )?,
        request(2)?
    );
    let mut fresh = vectors::ORDINARY_SYMMETRIC;
    fresh[24..32].copy_from_slice(&1_u64.to_be_bytes());
    assert_facts(
        time(owner.receive(&fresh, receipt(context()?)?)?)?.facts(),
        2,
    )?;
    Ok(())
}

/// All four LI values discard undefined control timestamps, including zero.
/// # Errors
/// Propagates unexpected correlated-control rejection.
/// # Panics
/// Fails for time classification, altered code/facts, automatic rearm or replay.
#[test]
fn control_ignores_all_li_and_peer_timestamp_bit_families() -> TestResult {
    for leap in 0_u8..=3 {
        for (t2, t3) in [
            (0_u64, 0_u64),
            (0, u64::MAX),
            (u64::MAX, 0),
            (1, 1),
            (0xdead_beef_0000_0001, 0x1234_5678_9abc_def0),
        ] {
            for code in [0, 0x5241_5445, 0x4445_4e59, u32::MAX] {
                let mut owner = armed(ORIGIN)?;
                let mut packet = vectors::KOD_RATE;
                packet[0] = (leap << 6) | 0x24;
                packet[12..16].copy_from_slice(&code.to_be_bytes());
                packet[32..40].copy_from_slice(&t2.to_be_bytes());
                packet[40..48].copy_from_slice(&t3.to_be_bytes());
                let event = owner.receive(&packet, receipt(context()?)?)?;
                let FixtureEvent::Control(control) = event else {
                    return Err(io::Error::other("stratum zero emitted time").into());
                };
                assert_eq!(control.code_bits(), code);
                assert_facts(control.facts(), 1)?;
                assert_eq!(owner.pending_request_id(), None);
                assert_eq!(owner.next_request_id(), Some(request(2)?));
                assert_eq!(
                    owner.receive(&packet, receipt(context()?)?),
                    Err(ReceiveError::NoPending)
                );
            }
        }
    }
    Ok(())
}

/// Undefined control timestamps cannot bypass correlation, mapping or deadline.
/// # Errors
/// Propagates unexpected valid-control rejection.
/// # Panics
/// Fails when any forged control consumes the healthy pending request.
#[test]
fn control_forgery_neighbors_preserve_pending_for_every_li() -> TestResult {
    for leap in 0_u8..=3 {
        let mut legal = vectors::KOD_RATE;
        legal[0] = (leap << 6) | 0x24;
        legal[32..48].fill(0);
        for (origin, received, mapping, processing, expected) in [
            (
                0,
                context()?.binding(),
                1,
                ARRIVAL,
                ReceiveError::ZeroOrigin,
            ),
            (
                1,
                context()?.binding(),
                1,
                ARRIVAL,
                ReceiveError::OriginMismatch,
            ),
            (
                ORIGIN,
                binding(1, 1, "192.0.2.2:123")?,
                1,
                ARRIVAL,
                ReceiveError::EndpointMismatch,
            ),
            (
                ORIGIN,
                context()?.binding(),
                2,
                ARRIVAL,
                ReceiveError::MappingGenerationMismatch,
            ),
            (
                ORIGIN,
                context()?.binding(),
                1,
                DEADLINE,
                ReceiveError::DeadlineReached,
            ),
        ] {
            let mut owner = armed(ORIGIN)?;
            let mut forged = legal;
            forged[24..32].copy_from_slice(&origin.to_be_bytes());
            let facts = ReceiptFacts::new(
                received,
                ClockGeneration::new(mapping),
                MonotonicNs::new(ARRIVAL),
                MonotonicNs::new(processing),
                UtcCoordinateQ32::new(LOCAL_T4)?,
            );
            assert_eq!(owner.receive(&forged, facts), Err(expected));
            assert_eq!(owner.pending_request_id(), Some(request(1)?));
            assert!(matches!(
                owner.receive(&legal, receipt(context()?)?)?,
                FixtureEvent::Control(_)
            ));
            assert_eq!(
                owner.receive(&legal, receipt(context()?)?),
                Err(ReceiveError::NoPending)
            );
        }
    }
    Ok(())
}

/// Explicit era-one fixture windows yield the independently known rollover truth.
/// # Errors
/// Propagates unexpected valid-window/packet rejection.
/// # Panics
/// Fails if rollover guesses an era, loses exact values or alters the 1/8s delay.
#[test]
fn explicit_rollover_literal_is_only_fixture_numeric_evidence() -> TestResult {
    let mut owner = Association::new_plain_header_fixture(context()?, request(1)?);
    let window = EraWindow::new(18_446_744_073_709_551_616, 36_893_488_147_419_103_231)?;
    let allocated = owner.arm(
        UtcCoordinateQ32::new(18_446_744_073_575_333_888)?,
        WireToken::new(0xffff_ffff_f800_0000)?,
        timing()?,
        PeerEraContext::new(window, window),
    )?;
    assert_eq!(allocated, request(1)?);
    let facts = ReceiptFacts::new(
        context()?.binding(),
        ClockGeneration::new(1),
        MonotonicNs::new(1_187_500_000),
        MonotonicNs::new(1_187_500_000),
        UtcCoordinateQ32::new(18_446_744_074_380_640_256)?,
    );
    let observed = time(owner.receive(&vectors::ERA_ROLLOVER_EXPLICIT, facts)?)?;
    assert_eq!(observed.saved_real_t1().ticks(), 18_446_744_073_575_333_888);
    assert_eq!(observed.peer_t2().ticks(), 18_446_744_073_843_769_344);
    assert_eq!(observed.peer_t3().ticks(), 18_446_744_074_112_204_800);
    assert_eq!(observed.local_t4().ticks(), 18_446_744_074_380_640_256);
    assert_eq!(observed.offset().numerator(), 0);
    assert_eq!(observed.signed_round_trip().numerator(), 536_870_912);
    assert_eq!(observed.facts().arrival().nanoseconds(), 1_187_500_000);
    Ok(())
}

/// Missing-era context is a compile boundary; this case reaches actual ambiguity.
/// # Errors
/// Propagates unexpected valid numeric fixture construction or control rejection.
/// # Panics
/// Fails if ambiguous unfolding completes time or destroys pending state.
#[test]
fn ambiguous_rollover_literal_reaches_numeric_failure_atomically() -> TestResult {
    let mut owner = Association::new_plain_header_fixture(context()?, request(1)?);
    let wide = EraWindow::new(0, 36_893_488_147_419_103_231)?;
    let allocated = owner.arm(
        UtcCoordinateQ32::new(18_446_744_073_575_333_888)?,
        WireToken::new(0xffff_ffff_f800_0000)?,
        timing()?,
        PeerEraContext::new(wide, wide),
    )?;
    assert_eq!(allocated, request(1)?);
    assert_eq!(
        owner.receive(&vectors::ERA_ROLLOVER_AMBIGUOUS, receipt(context()?)?),
        Err(ReceiveError::ReceiveEra(DomainError::AmbiguousEra))
    );
    assert_eq!(owner.pending_request_id(), Some(request(1)?));
    assert_eq!(owner.next_request_id(), Some(request(2)?));
    let mut control = vectors::ERA_ROLLOVER_AMBIGUOUS;
    control[1] = 0;
    assert!(matches!(
        owner.receive(&control, receipt(context()?)?)?,
        FixtureEvent::Control(_)
    ));
    Ok(())
}

/// Independent windows isolate receive versus transmit no-candidate/ambiguity.
/// # Errors
/// Propagates unexpected fixture construction or healthy control rejection.
/// # Panics
/// Fails if numerical rejection mutates state or controls inspect unused eras.
#[test]
fn peer_era_failures_are_atomic_on_each_coordinate() -> TestResult {
    let good = EraWindow::new(0, 18_446_744_073_709_551_615)?;
    let absent = EraWindow::new(0, 1)?;
    let ambiguous = EraWindow::new(0, 36_893_488_147_419_103_231)?;
    for (t2, t3, expected) in [
        (
            absent,
            good,
            ReceiveError::ReceiveEra(DomainError::NoEraCandidate),
        ),
        (
            good,
            absent,
            ReceiveError::TransmitEra(DomainError::NoEraCandidate),
        ),
        (
            ambiguous,
            good,
            ReceiveError::ReceiveEra(DomainError::AmbiguousEra),
        ),
        (
            good,
            ambiguous,
            ReceiveError::TransmitEra(DomainError::AmbiguousEra),
        ),
    ] {
        let mut owner = Association::new_plain_header_fixture(context()?, request(1)?);
        let allocated = owner.arm(
            UtcCoordinateQ32::new(SAVED_T1)?,
            WireToken::new(ORIGIN)?,
            timing()?,
            PeerEraContext::new(t2, t3),
        )?;
        assert_eq!(allocated, request(1)?);
        assert_eq!(
            owner.receive(&vectors::ORDINARY_SYMMETRIC, receipt(context()?)?),
            Err(expected)
        );
        assert_eq!(owner.pending_request_id(), Some(request(1)?));
        assert_eq!(owner.current_context(), context()?);
        assert!(matches!(
            owner.receive(&vectors::KOD_RATE, receipt(context()?)?)?,
            FixtureEvent::Control(_)
        ));
    }
    Ok(())
}

/// Exact one-tick odd offsets and a negative delay remain inert numerical facts.
/// # Errors
/// Propagates unexpected fixture construction or time rejection.
/// # Panics
/// Fails if arithmetic rounds, drops sign or adds statistical rejection policy.
#[test]
fn odd_half_ticks_and_negative_signed_rtt_survive_receive() -> TestResult {
    for (t1, t2, t3, t4, offset, rtt) in [
        (0_i128, 1_u64, 1_u64, 1_i128, 1, 1),
        (2, 1, 1, 1, -1, -1),
        (0, 1, 4, 1, 4, -2),
    ] {
        let mut owner = Association::new_plain_header_fixture(context()?, request(1)?);
        let allocated = owner.arm(
            UtcCoordinateQ32::new(t1)?,
            WireToken::new(ORIGIN)?,
            timing()?,
            eras()?,
        )?;
        assert_eq!(allocated, request(1)?);
        let mut packet = vectors::ORDINARY_SYMMETRIC;
        packet[32..40].copy_from_slice(&t2.to_be_bytes());
        packet[40..48].copy_from_slice(&t3.to_be_bytes());
        let facts = ReceiptFacts::new(
            context()?.binding(),
            ClockGeneration::new(1),
            MonotonicNs::new(ARRIVAL),
            MonotonicNs::new(ARRIVAL),
            UtcCoordinateQ32::new(t4)?,
        );
        let observed = time(owner.receive(&packet, facts)?)?;
        assert_eq!(observed.offset().numerator(), offset);
        assert_eq!(observed.signed_round_trip().numerator(), rtt);
    }
    Ok(())
}

/// Arm the next request with a caller-selected nonzero token and fixture truth.
/// # Errors
/// Propagates fixture construction or actual arm rejection.
fn arm_next(owner: &mut Association, token: u64) -> TestResult<RequestId> {
    Ok(owner.arm(
        UtcCoordinateQ32::new(SAVED_T1)?,
        WireToken::new(token)?,
        timing()?,
        eras()?,
    )?)
}

/// One slot rejects overwrite, and cancellation names the actual pending ID.
/// # Errors
/// Propagates unexpected valid transition rejection.
/// # Panics
/// Fails on busy overwrite, stale cancellation or sequence reset.
#[test]
fn busy_and_cancel_transitions_preserve_exact_request_identity() -> TestResult {
    let mut owner = armed(ORIGIN)?;
    assert_eq!(
        owner.arm(
            UtcCoordinateQ32::new(0)?,
            WireToken::new(1)?,
            timing()?,
            eras()?
        ),
        Err(ArmError::Busy)
    );
    assert_eq!(owner.pending_request_id(), Some(request(1)?));
    assert_eq!(owner.next_request_id(), Some(request(2)?));
    assert_eq!(owner.cancel(request(2)?), Err(CancelError::RequestMismatch));
    assert_eq!(owner.cancel(request(1)?)?, request(1)?);
    assert_eq!(owner.cancel(request(1)?), Err(CancelError::NoPending));
    assert_eq!(arm_next(&mut owner, 1)?, request(2)?);
    assert_eq!(owner.cancel(request(1)?), Err(CancelError::RequestMismatch));
    assert_eq!(
        owner.receive(&vectors::ORDINARY_SYMMETRIC, receipt(context()?)?),
        Err(ReceiveError::OriginMismatch)
    );
    let mut fresh = vectors::ORDINARY_SYMMETRIC;
    fresh[24..32].copy_from_slice(&1_u64.to_be_bytes());
    assert_facts(
        time(owner.receive(&fresh, receipt(context()?)?)?)?.facts(),
        2,
    )?;
    Ok(())
}

/// Expiry belongs to an explicit trusted lifecycle call, including equality.
/// # Errors
/// Propagates unexpected valid expiry/rearm rejection.
/// # Panics
/// Fails on early retirement, deadline inclusivity or successor corruption.
#[test]
fn expiry_exact_neighbors_and_late_traffic() -> TestResult {
    for now in [DEADLINE, DEADLINE + 1, u64::MAX] {
        let mut owner = armed(ORIGIN)?;
        assert_eq!(
            owner.expire(MonotonicNs::new(DEADLINE - 1)),
            Err(ExpireError::NotExpired)
        );
        assert_eq!(owner.pending_request_id(), Some(request(1)?));
        assert_eq!(owner.expire(MonotonicNs::new(now))?, request(1)?);
        assert_eq!(
            owner.expire(MonotonicNs::new(now)),
            Err(ExpireError::NoPending)
        );
        assert_eq!(
            owner.receive(&vectors::ORDINARY_SYMMETRIC, receipt(context()?)?),
            Err(ReceiveError::NoPending)
        );
        assert_eq!(arm_next(&mut owner, 1)?, request(2)?);
        assert_eq!(
            owner.receive(&vectors::ORDINARY_SYMMETRIC, receipt(context()?)?),
            Err(ReceiveError::OriginMismatch)
        );
        let mut fresh = vectors::ORDINARY_SYMMETRIC;
        fresh[24..32].copy_from_slice(&1_u64.to_be_bytes());
        assert!(owner.receive(&fresh, receipt(context()?)?).is_ok());
    }
    Ok(())
}

/// The final positive ID is usable once; busy precedes exhaustion and never wraps.
/// # Errors
/// Propagates unexpected legal final-ID arm/retirement rejection.
/// # Panics
/// Fails for early exhaustion, wrapped IDs or counter restoration on retirement.
#[test]
fn request_id_maximum_completes_once_and_exhaustion_survives_retirement() -> TestResult {
    for retire_by in 0..4 {
        let mut owner = Association::new_plain_header_fixture(context()?, request(u64::MAX)?);
        assert_eq!(arm_next(&mut owner, ORIGIN)?, request(u64::MAX)?);
        assert_eq!(owner.next_request_id(), None);
        assert_eq!(
            owner.arm(
                UtcCoordinateQ32::new(0)?,
                WireToken::new(1)?,
                timing()?,
                eras()?
            ),
            Err(ArmError::Busy)
        );
        match retire_by {
            0 => {
                assert!(
                    owner
                        .receive(&vectors::ORDINARY_SYMMETRIC, receipt(context()?)?)
                        .is_ok()
                );
            }
            1 => {
                assert_eq!(owner.cancel(request(u64::MAX)?)?, request(u64::MAX)?);
            }
            2 => {
                assert_eq!(
                    owner.expire(MonotonicNs::new(DEADLINE))?,
                    request(u64::MAX)?
                );
            }
            _ => {
                let next = AssociationContext::new(context()?.binding(), generations(2, 1, 1));
                assert_eq!(owner.advance_context(next)?, Some(request(u64::MAX)?));
            }
        }
        assert_eq!(owner.pending_request_id(), None);
        assert_eq!(owner.next_request_id(), None);
        assert_eq!(
            owner.arm(
                UtcCoordinateQ32::new(0)?,
                WireToken::new(1)?,
                timing()?,
                eras()?
            ),
            Err(ArmError::SequenceExhausted)
        );
        assert_eq!(owner.pending_request_id(), None);
        assert_eq!(owner.next_request_id(), None);
    }
    Ok(())
}

/// Trusted configuration/association/clock advances retire real pending work.
/// # Errors
/// Propagates unexpected valid update, successor arm or healthy receive rejection.
/// # Panics
/// Fails if old literals complete/retire a successor or output facts stay stale.
#[test]
fn generation_literal_families_execute_real_context_transitions() -> TestResult {
    for (next_generations, old_packet) in [
        (generations(2, 1, 1), vectors::ORDINARY_SYMMETRIC),
        (generations(1, 2, 1), vectors::STALE_ASSOCIATION),
        (generations(1, 1, 2), vectors::CLOCK_DISCONTINUITY),
    ] {
        let mut owner = armed(ORIGIN)?;
        let next = AssociationContext::new(context()?.binding(), next_generations);
        assert_eq!(owner.advance_context(next)?, Some(request(1)?));
        assert_eq!(
            owner.receive(&old_packet, receipt(context()?)?),
            Err(ReceiveError::NoPending)
        );
        assert_eq!(arm_next(&mut owner, 1)?, request(2)?);
        let expected = if next_generations.clock() == ClockGeneration::new(2) {
            ReceiveError::MappingGenerationMismatch
        } else {
            ReceiveError::OriginMismatch
        };
        assert_eq!(
            owner.receive(&old_packet, receipt(context()?)?),
            Err(expected)
        );
        assert_eq!(owner.pending_request_id(), Some(request(2)?));
        let mut fresh = vectors::ORDINARY_SYMMETRIC;
        fresh[24..32].copy_from_slice(&1_u64.to_be_bytes());
        let observed = time(owner.receive(&fresh, receipt(next)?)?)?;
        assert_eq!(observed.facts().generations(), next_generations);
        assert_eq!(
            observed.facts().mapping_generation(),
            next_generations.clock()
        );
        assert_eq!(observed.facts().request_id(), request(2)?);
    }
    Ok(())
}

/// Invalid trusted updates cannot partially replace context, slot or sequence.
/// # Errors
/// Propagates unexpected fixed fixture or healthy-neighbor failure.
/// # Panics
/// Fails on backward/equal generation acceptance or incomplete atomic rejection.
#[test]
fn context_regression_and_no_advance_are_atomic() -> TestResult {
    for (next, expected) in [
        (generations(0, 2, 2), ContextError::ConfigurationRegression),
        (generations(2, 0, 2), ContextError::AssociationRegression),
        (generations(2, 2, 0), ContextError::ClockRegression),
        (generations(1, 1, 1), ContextError::NoGenerationAdvance),
    ] {
        let mut owner = armed(ORIGIN)?;
        assert_eq!(
            owner.advance_context(AssociationContext::new(context()?.binding(), next)),
            Err(expected)
        );
        assert_eq!(owner.current_context(), context()?);
        assert_eq!(owner.pending_request_id(), Some(request(1)?));
        assert_eq!(owner.next_request_id(), Some(request(2)?));
        assert_facts(
            time(owner.receive(&vectors::ORDINARY_SYMMETRIC, receipt(context()?)?)?)?.facts(),
            1,
        )?;
    }
    Ok(())
}

/// Every binding component requires a real association-generation advance.
/// # Errors
/// Propagates unexpected fixture or valid context-update failure.
/// # Panics
/// Fails if binding swaps bypass generation admission or valid successors fail.
#[test]
fn every_binding_change_requires_association_advance() -> TestResult {
    for changed in [
        binding(2, 1, "192.0.2.1:123")?,
        binding(1, 2, "192.0.2.1:123")?,
        binding(1, 1, "192.0.2.2:123")?,
        binding(1, 1, "192.0.2.1:124")?,
    ] {
        let mut owner = armed(ORIGIN)?;
        let invalid = AssociationContext::new(changed, generations(2, 1, 2));
        assert_eq!(
            owner.advance_context(invalid),
            Err(ContextError::BindingChangedWithoutAssociationAdvance)
        );
        assert_eq!(owner.current_context(), context()?);
        assert_eq!(owner.pending_request_id(), Some(request(1)?));
        let valid = AssociationContext::new(changed, generations(2, 7, 2));
        assert_eq!(owner.advance_context(valid)?, Some(request(1)?));
        assert_eq!(owner.current_context(), valid);
        assert_eq!(arm_next(&mut owner, 1)?, request(2)?);
        let mut fresh = vectors::ORDINARY_SYMMETRIC;
        fresh[24..32].copy_from_slice(&1_u64.to_be_bytes());
        let observed = time(owner.receive(&fresh, receipt(valid)?)?)?;
        assert_eq!(observed.facts().binding(), changed);
        assert_eq!(observed.facts().generations(), generations(2, 7, 2));
    }
    Ok(())
}

/// Forward jumps and zero are admitted; a saturated component may remain equal.
/// # Errors
/// Propagates unexpected valid generation-context update rejection.
/// # Panics
/// Fails on hidden one-step limits, saturation wrap or invalid all-MAX advance.
#[test]
fn zero_forward_jumps_and_saturated_generation_contexts() -> TestResult {
    let zero = AssociationContext::new(context()?.binding(), generations(0, 0, 0));
    let mut owner = Association::new_plain_header_fixture(zero, request(1)?);
    assert_eq!(owner.current_context(), zero);
    let jump = AssociationContext::new(context()?.binding(), generations(u64::MAX, 7, 13));
    assert_eq!(owner.advance_context(jump)?, None);
    let saturation = AssociationContext::new(
        context()?.binding(),
        generations(u64::MAX, u64::MAX, u64::MAX),
    );
    assert_eq!(owner.advance_context(saturation)?, None);
    assert_eq!(
        owner.advance_context(saturation),
        Err(ContextError::NoGenerationAdvance)
    );
    assert_eq!(owner.current_context(), saturation);
    assert_eq!(owner.next_request_id(), Some(request(1)?));
    assert_eq!(arm_next(&mut owner, ORIGIN)?, request(1)?);
    let observed = time(owner.receive(&vectors::ORDINARY_SYMMETRIC, receipt(saturation)?)?)?;
    assert_eq!(
        observed.facts().generations(),
        generations(u64::MAX, u64::MAX, u64::MAX)
    );
    Ok(())
}

/// Release-active raw rejection is paired with the matching accepted neighbor.
/// # Errors
/// Propagates unexpected valid token/request/packet rejection.
/// # Panics
/// Fails if zero-origin admission depends on debug assertions or loses the slot.
#[test]
fn release_build_preserves_exchange_admission() -> TestResult {
    assert_eq!(WireToken::new(0), Err(TokenError::Zero));
    let mut owner = armed(1)?;
    let mut packet = vectors::ORDINARY_SYMMETRIC;
    packet[24..32].fill(0);
    assert_eq!(
        owner.receive(&packet, receipt(context()?)?),
        Err(ReceiveError::ZeroOrigin)
    );
    assert_eq!(owner.pending_request_id(), Some(request(1)?));
    packet[24..32].copy_from_slice(&1_u64.to_be_bytes());
    assert_eq!(
        time(owner.receive(&packet, receipt(context()?)?)?)?
            .offset()
            .numerator(),
        171_798_691
    );
    assert_eq!(owner.pending_request_id(), None);
    Ok(())
}

/// Ordered guards distinguish malformed profile/context/correlation/timing input.
/// # Errors
/// Propagates unexpected setup or healthy request rejection.
/// # Panics
/// Fails if a later guard replaces an earlier documented rejection or mutates state.
#[test]
fn receive_guard_precedence_is_observable_at_the_raw_entry() -> TestResult {
    let mut empty = Association::new_plain_header_fixture(context()?, request(1)?);
    assert_eq!(
        empty.receive(&[], receipt(context()?)?),
        Err(ReceiveError::Framing(ntp_wire::FrameError::HeaderTooShort))
    );
    assert_eq!(
        empty.receive(&vectors::OLD_VERSION, receipt(context()?)?),
        Err(ReceiveError::UnsupportedVersion)
    );
    assert_eq!(
        empty.receive(&vectors::CLIENT_MODE, receipt(context()?)?),
        Err(ReceiveError::UnsupportedMode)
    );
    assert_eq!(
        empty.receive(&vectors::ORDINARY_SYMMETRIC, receipt(context()?)?),
        Err(ReceiveError::NoPending)
    );
    let mut owner = armed(ORIGIN)?;
    let mut packet = vectors::ORDINARY_SYMMETRIC;
    packet[24..32].fill(0);
    let wrong = ReceiptFacts::new(
        binding(2, 2, "192.0.2.2:124")?,
        ClockGeneration::new(2),
        MonotonicNs::new(SEND - 1),
        MonotonicNs::new(DEADLINE),
        UtcCoordinateQ32::new(0)?,
    );
    assert_eq!(
        owner.receive(&packet, wrong),
        Err(ReceiveError::SourceMismatch)
    );
    assert_eq!(
        owner.receive(&packet, receipt(context()?)?),
        Err(ReceiveError::ZeroOrigin)
    );
    packet[24..32].copy_from_slice(&ORIGIN.to_be_bytes());
    packet[0] = 0xe4;
    packet[1] = 16;
    packet[32..48].fill(0);
    assert_eq!(
        owner.receive(&packet, receipt(context()?)?),
        Err(ReceiveError::UnsynchronizedLeap)
    );
    packet[0] = 0x24;
    assert_eq!(
        owner.receive(&packet, receipt(context()?)?),
        Err(ReceiveError::InvalidStratum)
    );
    packet[1] = 1;
    assert_eq!(
        owner.receive(&packet, receipt(context()?)?),
        Err(ReceiveError::ZeroReceiveTimestamp)
    );
    assert_eq!(owner.pending_request_id(), Some(request(1)?));
    assert!(
        owner
            .receive(&vectors::ORDINARY_SYMMETRIC, receipt(context()?)?)
            .is_ok()
    );
    Ok(())
}

/// A requirement-derived sequential model; no production reducer is consulted.
///
/// Transition table: arm empty -> pending(next), next++; arm pending -> Busy.
/// Time/control pending -> empty, event++; empty -> `NoPending`. Malformed input
/// always rejects and preserves state. Correct cancel/late expiry retire pending;
/// wrong cancel/early expiry preserve it. Forward context update retires pending;
/// equal context rejects. Every transition preserves the next request sequence.
struct Model {
    /// Independently tracked current configuration generation.
    configuration: u64,
    /// Current pending request, or an empty completed/retired slot.
    pending: Option<u64>,
    /// Next never-reused request identity in this bounded trace.
    next: u64,
    /// Expected exactly-once events emitted by successful time/control receives.
    events: u64,
}

/// Execute one bounded generated command against the independent table.
/// # Errors
/// Propagates invalid harness facts or an unexpected public transition failure.
/// # Panics
/// Fails at the first table/application outcome mismatch.
fn model_step(owner: &mut Association, model: &mut Model, action: usize) -> TestResult<u64> {
    let current =
        AssociationContext::new(context()?.binding(), generations(model.configuration, 1, 1));
    match action {
        0 => {
            let actual = owner.arm(
                UtcCoordinateQ32::new(SAVED_T1)?,
                WireToken::new(ORIGIN)?,
                timing()?,
                eras()?,
            );
            if model.pending.is_some() {
                assert_eq!(actual, Err(ArmError::Busy));
            } else {
                assert_eq!(actual?, request(model.next)?);
                model.pending = Some(model.next);
                model.next += 1;
            }
        }
        1 | 2 => {
            let packet = if action == 1 {
                &vectors::ORDINARY_SYMMETRIC
            } else {
                &vectors::KOD_RATE
            };
            let actual = owner.receive(packet, receipt(current)?);
            if let Some(pending) = model.pending {
                match (&actual?, action) {
                    (FixtureEvent::Time(observation), 1) => {
                        assert_eq!(observation.facts().request_id(), request(pending)?);
                        assert_eq!(observation.offset().numerator(), 171_798_691);
                    }
                    (FixtureEvent::Control(control), 2) => {
                        assert_eq!(control.facts().request_id(), request(pending)?);
                        assert_eq!(control.code_bits(), 0x5241_5445);
                    }
                    _ => return Err(io::Error::other("model event class mismatch").into()),
                }
                model.pending = None;
                model.events += 1;
                return Ok(1);
            }
            assert_eq!(actual, Err(ReceiveError::NoPending));
        }
        3 => assert_eq!(
            owner.receive(&vectors::TRUNCATED_47, receipt(current)?),
            Err(ReceiveError::Framing(ntp_wire::FrameError::HeaderTooShort))
        ),
        4 => {
            if let Some(pending) = model.pending {
                assert_eq!(owner.cancel(request(pending)?)?, request(pending)?);
                model.pending = None;
            } else {
                assert_eq!(owner.cancel(request(1)?), Err(CancelError::NoPending));
            }
        }
        5 => {
            let expected = if model.pending.is_some() {
                CancelError::RequestMismatch
            } else {
                CancelError::NoPending
            };
            assert_eq!(owner.cancel(request(u64::MAX)?), Err(expected));
        }
        6 => {
            let expected = if model.pending.is_some() {
                ExpireError::NotExpired
            } else {
                ExpireError::NoPending
            };
            assert_eq!(owner.expire(MonotonicNs::new(DEADLINE - 1)), Err(expected));
        }
        7 => {
            if let Some(pending) = model.pending {
                assert_eq!(owner.expire(MonotonicNs::new(DEADLINE))?, request(pending)?);
                model.pending = None;
            } else {
                assert_eq!(
                    owner.expire(MonotonicNs::new(DEADLINE)),
                    Err(ExpireError::NoPending)
                );
            }
        }
        8 => {
            model.configuration += 1;
            let next = AssociationContext::new(
                context()?.binding(),
                generations(model.configuration, 1, 1),
            );
            assert_eq!(
                owner.advance_context(next)?,
                model.pending.map(request).transpose()?
            );
            model.pending = None;
        }
        _ => assert_eq!(
            owner.advance_context(current),
            Err(ContextError::NoGenerationAdvance)
        ),
    }
    Ok(0)
}

/// Deterministic xorshift selects 2,048 commands from the independent table.
/// Seed and command count make every failure reproducible from empty state.
/// # Errors
/// Propagates unexpected fixture/transition failures.
/// # Panics
/// Fails on any state/event mismatch, empty action family or no healthy event.
#[test]
fn bounded_generated_trace_matches_independent_state_model() -> TestResult {
    let mut owner = Association::new_plain_header_fixture(context()?, request(1)?);
    let mut model = Model {
        configuration: 1,
        pending: None,
        next: 1,
        events: 0,
    };
    let mut seed = 0x511d_cafe_babe_2026_u64;
    let mut executed = [0_u64; 10];
    let mut actual_events = 0;
    for step in 0..2048 {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        let action = usize::try_from(seed % 10)?;
        executed[action] += 1;
        actual_events += model_step(&mut owner, &mut model, action)?;
        assert_eq!(
            owner.pending_request_id(),
            model.pending.map(request).transpose()?,
            "step {step}"
        );
        assert_eq!(
            owner.next_request_id(),
            Some(request(model.next)?),
            "step {step}"
        );
        assert_eq!(
            owner.current_context(),
            AssociationContext::new(context()?.binding(), generations(model.configuration, 1, 1)),
            "step {step}"
        );
        assert_eq!(actual_events, model.events, "step {step}");
    }
    assert_eq!(executed, [201, 203, 222, 198, 209, 200, 182, 204, 203, 226]);
    assert_eq!(executed.iter().sum::<u64>(), 2048);
    assert_eq!(actual_events, 61);
    assert_eq!(model.configuration, 204);
    assert_eq!(model.next, 165);
    assert_eq!(model.pending, None);
    Ok(())
}

/// Fixed workload executes 4,096 hostile packets and 256 retire/rearm recoveries.
/// It observes real cleanup before teardown; allocation instrumentation is separate.
/// # Errors
/// Propagates unexpected setup, retirement or healthy successor failure.
/// # Panics
/// Fails for rejection mutation, stale completion or final retained pending work.
#[test]
fn bounded_hostile_workload_and_retire_rearm_recovery() -> TestResult {
    let mut owner = Association::new_plain_header_fixture(context()?, request(1)?);
    let mut rejected = 0;
    let mut recovered = 0;
    for cycle in 1_u64..=256 {
        let id = arm_next(&mut owner, cycle)?;
        for _ in 0..16 {
            let mut hostile = vectors::ORDINARY_SYMMETRIC;
            hostile[24..32].fill(0);
            assert_eq!(
                owner.receive(&hostile, receipt(context()?)?),
                Err(ReceiveError::ZeroOrigin)
            );
            assert_eq!(owner.pending_request_id(), Some(id));
            rejected += 1;
        }
        if cycle % 2 == 0 {
            assert_eq!(owner.cancel(id)?, id);
        } else {
            assert_eq!(owner.expire(MonotonicNs::new(DEADLINE))?, id);
        }
        assert_eq!(owner.pending_request_id(), None);
        let successor = arm_next(&mut owner, cycle + 1000)?;
        assert_eq!(owner.cancel(id), Err(CancelError::RequestMismatch));
        let mut fresh = vectors::ORDINARY_SYMMETRIC;
        fresh[24..32].copy_from_slice(&(cycle + 1000).to_be_bytes());
        let observed = time(owner.receive(&fresh, receipt(context()?)?)?)?;
        assert_eq!(observed.facts().request_id(), successor);
        assert_eq!(observed.offset().numerator(), 171_798_691);
        assert_eq!(owner.pending_request_id(), None);
        recovered += 1;
    }
    assert_eq!(rejected, 4096);
    assert_eq!(recovered, 256);
    assert_eq!(owner.next_request_id(), Some(request(513)?));
    assert_eq!(owner.pending_request_id(), None);
    Ok(())
}

/// The two final IDs are distinct; after both complete no wrap/reuse is possible.
/// # Errors
/// Propagates unexpected final-ID arm or healthy completion failure.
/// # Panics
/// Fails if the sequence skips MAX, repeats an ID or wraps to a legal identity.
#[test]
fn final_two_request_sequence_values_are_each_usable_once() -> TestResult {
    let mut owner = Association::new_plain_header_fixture(context()?, request(u64::MAX - 1)?);
    for expected in [u64::MAX - 1, u64::MAX] {
        assert_eq!(arm_next(&mut owner, ORIGIN)?, request(expected)?);
        let observed = time(owner.receive(&vectors::ORDINARY_SYMMETRIC, receipt(context()?)?)?)?;
        assert_eq!(observed.facts().request_id(), request(expected)?);
        assert_eq!(owner.pending_request_id(), None);
    }
    assert_eq!(owner.next_request_id(), None);
    assert_eq!(
        owner.arm(
            UtcCoordinateQ32::new(SAVED_T1)?,
            WireToken::new(ORIGIN)?,
            timing()?,
            eras()?
        ),
        Err(ArmError::SequenceExhausted)
    );
    Ok(())
}

/// Token formatting and completed-result formatting reveal no origin-token bits.
/// # Errors
/// Propagates unexpected valid token, request or healthy packet rejection.
/// # Panics
/// Fails if checked tokens or committed results expose their secret representation.
#[test]
fn token_debug_and_completed_observation_do_not_retain_origin_bits() -> TestResult {
    let token = 0x0f1e_2d3c_4b5a_6978_u64;
    let redacted = format!("{:?}", WireToken::new(token)?);
    for representation in [
        token.to_string(),
        format!("{token:x}"),
        format!("{token:X}"),
    ] {
        assert!(!redacted.contains(&representation));
    }
    let mut owner = armed(token)?;
    let mut packet = vectors::ORDINARY_SYMMETRIC;
    packet[24..32].copy_from_slice(&token.to_be_bytes());
    let observation = time(owner.receive(&packet, receipt(context()?)?)?)?;
    let rendered = format!("{observation:?}");
    for representation in [
        token.to_string(),
        format!("{token:x}"),
        format!("{token:X}"),
    ] {
        assert!(!rendered.contains(&representation));
    }
    assert_eq!(observation.saved_real_t1().ticks(), SAVED_T1);
    assert_eq!(owner.pending_request_id(), None);
    Ok(())
}

/// A rejected busy arm preserves every private input of the original request.
/// # Errors
/// Propagates unexpected fixture construction or original-request rejection.
/// # Panics
/// Fails if a rejected replacement changes saved time, token, timing or eras.
#[test]
fn busy_rejection_preserves_all_private_pending_inputs() -> TestResult {
    let mut owner = armed(ORIGIN)?;
    let replacement_timing = RequestTiming::new(MonotonicNs::new(17), MonotonicNs::new(19))?;
    let replacement_eras = PeerEraContext::new(EraWindow::new(37, 41)?, EraWindow::new(43, 47)?);
    assert_eq!(
        owner.arm(
            UtcCoordinateQ32::new(7)?,
            WireToken::new(1)?,
            replacement_timing,
            replacement_eras,
        ),
        Err(ArmError::Busy)
    );
    assert_eq!(owner.current_context(), context()?);
    assert_eq!(owner.pending_request_id(), Some(request(1)?));
    assert_eq!(owner.next_request_id(), Some(request(2)?));

    let observed = time(owner.receive(&vectors::ORDINARY_SYMMETRIC, receipt(context()?)?)?)?;
    assert_facts(observed.facts(), 1)?;
    assert_default_quality(&observed);
    assert_eq!(observed.saved_real_t1().ticks(), SAVED_T1);
    assert_eq!(observed.peer_t2().ticks(), 0xe000_0000_11eb_851e);
    assert_eq!(observed.peer_t3().ticks(), 0xe000_0000_147a_e147);
    assert_eq!(observed.local_t4().ticks(), LOCAL_T4);
    assert_eq!(observed.offset().numerator(), 171_798_691);
    assert_eq!(observed.signed_round_trip().numerator(), 429_496_729);
    assert_eq!(owner.current_context(), context()?);
    assert_eq!(owner.pending_request_id(), None);
    assert_eq!(owner.next_request_id(), Some(request(2)?));
    assert_eq!(
        owner.receive(&vectors::ORDINARY_SYMMETRIC, receipt(context()?)?),
        Err(ReceiveError::NoPending)
    );
    Ok(())
}

/// Check fixed error formatting independently of any complete expected wording.
/// # Panics
/// Fails for empty, excessive or input-bearing public diagnostics.
fn assert_bounded_errors<T: Error>(errors: &[T]) {
    for error in errors {
        let message = error.to_string();
        assert_ne!(message, "");
        assert!(message.len() <= 80);
        assert!(message.is_ascii());
        assert!(!message.contains("192.0.2.1"));
        assert!(!message.contains("16140901064495857664"));
        assert!(format!("{error:?}").len() <= 128);
    }
}

/// Every public failure has a useful fixed diagnostic without rejected input.
/// # Panics
/// Fails for empty/unbounded formatting or diagnostic disclosure.
#[test]
fn all_public_failures_have_bounded_redacted_formatting() {
    assert_bounded_errors(&[TokenError::Zero]);
    assert_bounded_errors(&[GenerationError::GenerationExhausted]);
    assert_bounded_errors(&[
        TimingError::NonPositiveLifetime,
        TimingError::LifetimeExceeded,
    ]);
    assert_bounded_errors(&[ArmError::Busy, ArmError::SequenceExhausted]);
    assert_bounded_errors(&[CancelError::NoPending, CancelError::RequestMismatch]);
    assert_bounded_errors(&[ExpireError::NoPending, ExpireError::NotExpired]);
    assert_bounded_errors(&[
        ContextError::ConfigurationRegression,
        ContextError::AssociationRegression,
        ContextError::ClockRegression,
        ContextError::NoGenerationAdvance,
        ContextError::BindingChangedWithoutAssociationAdvance,
    ]);
    assert_bounded_errors(&[
        ReceiveError::Framing(ntp_wire::FrameError::ByteLimit),
        ReceiveError::UnsupportedVersion,
        ReceiveError::UnsupportedMode,
        ReceiveError::NoPending,
        ReceiveError::SourceMismatch,
        ReceiveError::SocketMismatch,
        ReceiveError::EndpointMismatch,
        ReceiveError::StaleConfiguration,
        ReceiveError::StaleAssociation,
        ReceiveError::StaleClock,
        ReceiveError::MappingGenerationMismatch,
        ReceiveError::ZeroOrigin,
        ReceiveError::OriginMismatch,
        ReceiveError::ArrivalBeforeSend,
        ReceiveError::ProcessingBeforeArrival,
        ReceiveError::DeadlineReached,
        ReceiveError::UnsynchronizedLeap,
        ReceiveError::InvalidStratum,
        ReceiveError::ZeroReceiveTimestamp,
        ReceiveError::ZeroTransmitTimestamp,
        ReceiveError::ReceiveEra(DomainError::NoEraCandidate),
        ReceiveError::TransmitEra(DomainError::AmbiguousEra),
        ReceiveError::Arithmetic(DomainError::ArithmeticOverflow),
    ]);
}

/// Bounded deterministic input producer and actual public-association driver.
#[path = "support/scenario_driver.rs"]
mod scenario_driver;

/// Independently authored literal observations and scenario conformance checks.
#[path = "support/scenario_expectations.rs"]
mod scenario_expectations;

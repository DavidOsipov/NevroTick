//! RFC 8915 section 5.1 literal exporter metadata; REQ-8/BND-2/CHECK-6/25.
//! No exporter call, key length, provider support, TLS identity or authority claim.
#![forbid(unsafe_code)]

// Agreed API: NtsKeExporterDirection::{ClientToServer,ServerToClient}; private
// owned NtsKeExporterContext; pending.exporter_context(direction) is infallible;
// context.label() -> &'static str; context.context_bytes() -> &[u8;5].
// The context has no public raw factory, lifetime, Default or Debug route.
// Proposed maintained API checks (comments are NOT executed rustdoc evidence):
// ```compile_fail,E0451
// use ntp_wire::NtsKeExporterContext;
// let _ = NtsKeExporterContext { bytes: [0; 5] };
// ```
// ```compile_fail,E0616
// use ntp_wire::NtsKeExporterContext;
// fn corrupt(mut context: NtsKeExporterContext) { context.bytes[4] = 7; }
// ```
// ```compile_fail,E0594
// use ntp_wire::NtsKeExporterContext;
// fn corrupt(context: &NtsKeExporterContext) { context.context_bytes()[4] = 7; }
// ```
// ```compile_fail,E0277
// use ntp_wire::NtsKeExporterContext;
// let _: NtsKeExporterContext = Default::default();
// ```
// Legal neighboring use is exercised below after all negotiation buffers expire.

use ntp_wire::{
    NtsKeAccumulator, NtsKeExporterContext, NtsKeExporterDirection, NtsKeFrameLimits,
    NtsKeNegotiationError, NtsKeNtpv4Negotiation, NtsKeProgress, encode_nts_ke_ntpv4_request,
    frame_nts_ke_message, negotiate_nts_ke_ntpv4, read_nts_ke_ntpv4_offer,
};

/// Independent single-AEAD15 request with critical NPN/AEAD/EOM records.
static REQUEST15: [u8; 16] = [0x80, 1, 0, 2, 0, 0, 0x80, 4, 0, 2, 0, 15, 0x80, 0, 0, 0];
/// Independent successful response: offered15, one opaque byte cookie, final EOM.
static RESPONSE15: [u8; 21] = [
    0x80, 1, 0, 2, 0, 0, 0, 4, 0, 2, 0, 15, 0, 5, 0, 1, 0xaa, 0x80, 0, 0, 0,
];

/// Admit the same explicit existing experiment budgets for all small literals.
/// # Errors
/// Preserve unexpected rejection rather than manufacturing parser defaults.
fn limits() -> Result<NtsKeFrameLimits, String> {
    NtsKeFrameLimits::new(64, 8).map_err(|error| format!("limits: {error:?}"))
}

/// Construct pending negotiation through actual old owners and independent bytes.
/// # Errors
/// Preserve unexpected literal/request/response negotiation rejection.
fn pending(word: [u8; 2], response: &[u8]) -> Result<NtsKeNtpv4Negotiation<'_>, String> {
    let request = [
        0x80, 1, 0, 2, 0, 0, 0x80, 4, 0, 2, word[0], word[1], 0x80, 0, 0, 0,
    ];
    let raw = frame_nts_ke_message(&request, limits()?).map_err(|e| format!("request: {e:?}"))?;
    let offer = read_nts_ke_ntpv4_offer(&raw).map_err(|e| format!("offer: {e:?}"))?;
    let response =
        frame_nts_ke_message(response, limits()?).map_err(|e| format!("response: {e:?}"))?;
    negotiate_nts_ke_ntpv4(&offer, response).map_err(|e| format!("negotiate: {e:?}"))
}

/// Check only public, immutable metadata against independently declared literals.
/// # Panics
/// Require the exact RFC label and five context bytes, without inferring key size.
fn assert_context(context: &NtsKeExporterContext, expected: [u8; 5]) {
    assert_eq!(context.label(), "EXPORTER-network-time-security");
    assert_eq!(context.context_bytes(), &expected);
}

/// Existing literal negotiation remains healthy without invoking the new method.
/// # Errors
/// Preserve any pre-existing framing or semantic-consumer failure.
/// # Panics
/// Require exact selected algorithm, cookie and response bytes as a RED neighbor.
#[test]
fn exporter_context_existing_literal_negotiation_control() -> Result<(), String> {
    let negotiated = pending([0, 15], &RESPONSE15)?;
    assert_eq!(negotiated.aead_id(), 15);
    assert_eq!(negotiated.bytes(), RESPONSE15);
    assert_eq!(negotiated.cookie_count(), 1);
    assert_eq!(negotiated.cookies().next(), Some(&[0xaa][..]));
    assert_eq!(negotiated.server_hint(), None);
    assert_eq!(negotiated.port_hint(), None);
    Ok(())
}

/// RFC 8915 section5.1 specifies protocol0, algorithm15 and distinct direction bits.
/// # Errors
/// Preserve rejection by an already existing negotiation owner.
/// # Panics
/// Require both independent context literals, the exact label and direction split.
#[test]
fn exporter_context_algorithm15_has_literal_directional_bytes() -> Result<(), String> {
    let negotiated = pending([0, 15], &RESPONSE15)?;
    let c2s = negotiated.exporter_context(NtsKeExporterDirection::ClientToServer);
    let s2c = negotiated.exporter_context(NtsKeExporterDirection::ServerToClient);
    assert_eq!(c2s.label(), "EXPORTER-network-time-security");
    assert_eq!(s2c.label(), "EXPORTER-network-time-security");
    assert_eq!(
        c2s.context_bytes(),
        &[0, 0, 0, 15, 0],
        "independent C2S exporter context"
    );
    assert_eq!(
        s2c.context_bytes(),
        &[0, 0, 0, 15, 1],
        "independent S2C exporter context"
    );
    assert_ne!(c2s.context_bytes(), s2c.context_bytes());
    Ok(())
}

/// The selected generic u16 representation is retained; no provider is selected.
/// # Errors
/// Preserve rejection of independently offered full-width algorithm identifiers.
/// # Panics
/// Require asymmetric byte order, both endpoint values and both direction octets.
#[test]
fn exporter_context_preserves_full_u16_selected_algorithm() -> Result<(), String> {
    for (identifier, word) in [
        (0x1234_u16, [0x12, 0x34]),
        (u16::MAX, [0xff, 0xff]),
        (0, [0, 0]),
    ] {
        let response = [
            0x80, 1, 0, 2, 0, 0, 0, 4, 0, 2, word[0], word[1], 0, 5, 0, 0, 0x80, 0, 0, 0,
        ];
        let negotiated = pending(word, &response)?;
        assert_eq!(negotiated.aead_id(), identifier);
        assert_context(
            &negotiated.exporter_context(NtsKeExporterDirection::ClientToServer),
            [0, 0, word[0], word[1], 0],
        );
        assert_context(
            &negotiated.exporter_context(NtsKeExporterDirection::ServerToClient),
            [0, 0, word[0], word[1], 1],
        );
    }
    Ok(())
}

/// The real encoder and fragmented stream owners reach negotiation and metadata.
/// # Errors
/// Preserve actual encoder, incremental framing or semantic-admission failure.
/// # Panics
/// Require literal wire bytes and contexts that outlive every source buffer.
#[test]
fn exporter_context_follows_encoded_stream_negotiation_and_owns_metadata() -> Result<(), String> {
    let (c2s, s2c) = {
        let identifiers = [0x1234, 15, 0xffff, 0];
        let expected = [
            0x80, 1, 0, 2, 0, 0, 0x80, 4, 0, 8, 0x12, 0x34, 0, 15, 0xff, 0xff, 0, 0, 0x80, 0, 0, 0,
        ];
        let mut encoded = [0xa5; 64];
        let offered = encode_nts_ke_ntpv4_request(&identifiers, &mut encoded, limits()?)
            .map_err(|e| format!("encode: {e:?}"))?;
        assert_eq!(offered.request_bytes(), expected);
        let mut request_storage = [0xa5; 64];
        let mut request = NtsKeAccumulator::new(&mut request_storage, limits()?)
            .map_err(|e| format!("request new: {e:?}"))?;
        assert_eq!(
            request.push(&offered.request_bytes()[..7]),
            Ok(NtsKeProgress::NeedMore)
        );
        assert_eq!(
            request.push(&offered.request_bytes()[7..19]),
            Ok(NtsKeProgress::NeedMore)
        );
        assert_eq!(
            request.push(&offered.request_bytes()[19..]),
            Ok(NtsKeProgress::EndOfMessage)
        );
        let raw_request = request
            .finish()
            .map_err(|e| format!("request finish: {e:?}"))?;
        assert_eq!(raw_request.bytes(), expected);
        let checked =
            read_nts_ke_ntpv4_offer(&raw_request).map_err(|e| format!("stream offer: {e:?}"))?;
        let reply = [
            0x80, 1, 0, 2, 0, 0, 0, 4, 0, 2, 0x12, 0x34, 0, 5, 0, 1, 0xaa, 0x80, 0, 0, 0,
        ];
        let mut response_storage = [0xa5; 64];
        let mut response = NtsKeAccumulator::new(&mut response_storage, limits()?)
            .map_err(|e| format!("response new: {e:?}"))?;
        assert_eq!(response.push(&reply[..5]), Ok(NtsKeProgress::NeedMore));
        assert_eq!(response.push(&reply[5..10]), Ok(NtsKeProgress::NeedMore));
        assert_eq!(response.push(&reply[10..]), Ok(NtsKeProgress::EndOfMessage));
        let negotiated = negotiate_nts_ke_ntpv4(
            &checked,
            response
                .finish()
                .map_err(|e| format!("response finish: {e:?}"))?,
        )
        .map_err(|e| format!("stream negotiate: {e:?}"))?;
        assert_eq!(negotiated.aead_id(), 0x1234);
        assert_eq!(negotiated.cookie_count(), 1);
        (
            negotiated.exporter_context(NtsKeExporterDirection::ClientToServer),
            negotiated.exporter_context(NtsKeExporterDirection::ServerToClient),
        )
    };
    assert_context(&c2s, [0, 0, 0x12, 0x34, 0]);
    assert_context(&s2c, [0, 0, 0x12, 0x34, 1]);
    Ok(())
}

/// Rejected responses cannot reach the context consumer through a successful value.
/// # Errors
/// Preserve independent request/response structural setup failures.
/// # Panics
/// Require the exact rejection and zero context-consumer calls for each case.
#[test]
fn exporter_context_requires_successful_offered_negotiation() -> Result<(), String> {
    let raw = frame_nts_ke_message(&REQUEST15, limits()?).map_err(|e| format!("request: {e:?}"))?;
    let offer = read_nts_ke_ntpv4_offer(&raw).map_err(|e| format!("offer: {e:?}"))?;
    for (response, expected) in [
        (
            &[
                0x80, 1, 0, 2, 0, 0, 0, 4, 0, 2, 0, 16, 0, 5, 0, 0, 0x80, 0, 0, 0,
            ][..],
            NtsKeNegotiationError::UnofferedAead,
        ),
        (
            &[0x80, 2, 0, 2, 0, 1, 0x80, 0, 0, 0][..],
            NtsKeNegotiationError::PeerError { code: 1 },
        ),
        (
            &[0x80, 3, 0, 2, 0, 7, 0x80, 0, 0, 0][..],
            NtsKeNegotiationError::UnsupportedWarning { code: 7 },
        ),
    ] {
        let framed = frame_nts_ke_message(response, limits()?)
            .map_err(|e| format!("negative frame: {e:?}"))?;
        let mut consumers = 0;
        let result = negotiate_nts_ke_ntpv4(&offer, framed).map(|negotiated| {
            consumers += 1;
            negotiated.exporter_context(NtsKeExporterDirection::ClientToServer)
        });
        assert_eq!(result.err(), Some(expected));
        assert_eq!(consumers, 0);
    }
    Ok(())
}

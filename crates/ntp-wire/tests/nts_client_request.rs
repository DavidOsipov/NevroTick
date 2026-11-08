//! Independent RFC 8915 request-layout facts for the inert two-stage formatter.
//! REQ-8, BND-2, CHECK-6/17/25; no provider, random-source or send authority.
#![forbid(unsafe_code)]

use ntp_wire::{
    FrameError, FrameLimits, NtsRequestEncodeError, NtsRequestShape, RawHeader,
    decode_fixed_header, frame_no_legacy_mac_datagram, prepare_nts_client_request,
    read_nts_authenticator_parts,
};

const HEADER: [u8; 48] = [
    0x23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
const UID: [u8; 32] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31,
];
const COOKIE: [u8; 12] = [
    0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab,
];

// Full independently specified bytes: 48-byte header, 36-byte UID, 16-byte
// Cookie, 28-byte final Authenticator. No placeholder or algorithm is implicit.
const EMPTY_COMPONENT_PACKET: [u8; 128] = [
    0x23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 4, 0, 36, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
    10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 2, 4,
    0, 16, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 4, 4, 0, 28, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

// Two 16-byte placeholders, a three-byte nonce, five-byte ciphertext and twelve
// bytes of Additional Padding (N_REQ=16, padded nonce=4); the final field is32.
const PADDED_TWO_PLACEHOLDER_PACKET: [u8; 164] = [
    0x23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 4, 0, 36, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
    10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 2, 4,
    0, 16, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 3, 4, 0, 16, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 0, 32,
    0, 3, 0, 5, 1, 2, 3, 0, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

/// Decode an exact header without treating raw construction as request admission.
/// # Errors
/// Preserve an unexpected fixed-header decoding failure.
fn header(bytes: &[u8; 48]) -> Result<RawHeader, String> {
    decode_fixed_header(bytes).map_err(|error| format!("header: {error:?}"))
}

/// Supply explicit existing packet budgets; no default profile is selected.
/// # Errors
/// Preserve an unexpected limit rejection.
fn limits(bytes: usize, fields: usize) -> Result<FrameLimits, String> {
    FrameLimits::new(bytes, fields).map_err(|error| format!("limits: {error:?}"))
}

/// Admit one declared structural shape without an algorithm-binding claim.
/// # Errors
/// Preserve an unexpected shape rejection.
fn shape(nonce: usize, ciphertext: usize, maximum: usize) -> Result<NtsRequestShape, String> {
    NtsRequestShape::new(nonce, ciphertext, maximum).map_err(|error| format!("shape: {error:?}"))
}

/// One independent packet literal and its AAD, field-count and component facts.
type LiteralReaderCase<'a> = (&'a [u8], usize, usize, &'a [u8], &'a [u8]);

/// Independently admit both complete byte literals without invoking the formatter.
/// # Errors
/// Preserve unexpected existing frame or authenticator-reader refusal.
/// # Panics
/// Require each literal's exact field count, AAD boundary and declared components.
#[test]
fn existing_readers_accept_both_independent_full_literals_without_a_formatter() -> Result<(), String>
{
    let cases: [LiteralReaderCase<'_>; 2] = [
        (&EMPTY_COMPONENT_PACKET, 100, 3, &[], &[]),
        (
            &PADDED_TWO_PLACEHOLDER_PACKET,
            132,
            5,
            &[1, 2, 3],
            &[4, 5, 6, 7, 8],
        ),
    ];
    for (packet, aad_end, field_count, nonce, ciphertext) in cases {
        let framed = frame_no_legacy_mac_datagram(packet, limits(packet.len(), field_count)?)
            .map_err(|error| format!("literal frame: {error:?}"))?;
        assert_eq!(framed.extension_count(), field_count);
        assert_eq!(framed.header().version(), 4);
        assert_eq!(framed.header().mode(), 3);
        let parts = read_nts_authenticator_parts(packet, limits(packet.len(), field_count)?)
            .map_err(|error| format!("literal authenticator: {error:?}"))?;
        assert_eq!(parts.associated_data(), &packet[..aad_end]);
        assert_eq!(parts.nonce(), nonce);
        assert_eq!(parts.ciphertext(), ciphertext);
    }
    Ok(())
}

/// Observe every completed byte and the exact candidate AAD's original pointer.
/// # Errors
/// Preserve unexpected constructor, formatter or structural-reader refusal.
/// # Panics
/// Require the independent whole-packet literal, exact AAD and untouched suffix.
#[test]
fn empty_components_match_the_complete_literal_without_placeholders() -> Result<(), String> {
    let mut output = [0xa5; 136];
    let original = output.as_ptr();
    let prepared = prepare_nts_client_request(
        &header(&HEADER)?,
        &UID,
        &COOKIE,
        0,
        shape(0, 0, 0)?,
        &mut output,
        limits(128, 3)?,
    );
    assert!(
        prepared.is_ok(),
        "the independently specified 128-byte request fits its exact byte, field and capacity budgets"
    );
    let plan = prepared.map_err(|error| format!("prepare: {error:?}"))?;
    assert_eq!(plan.associated_data(), &EMPTY_COMPONENT_PACKET[..100]);
    assert!(core::ptr::eq(plan.associated_data().as_ptr(), original));
    let complete = plan
        .finish(&[], &[])
        .map_err(|error| format!("finish: {error:?}"))?;
    assert_eq!(complete.packet_bytes(), EMPTY_COMPONENT_PACKET);
    assert!(core::ptr::eq(complete.packet_bytes().as_ptr(), original));
    let parts = read_nts_authenticator_parts(complete.packet_bytes(), limits(128, 3)?)
        .map_err(|error| format!("secondary reader: {error:?}"))?;
    assert_eq!(parts.associated_data(), &EMPTY_COMPONENT_PACKET[..100]);
    assert_eq!(parts.nonce(), []);
    assert_eq!(parts.ciphertext(), []);
    assert_eq!(&output[128..], &[0xa5; 8]);
    Ok(())
}

/// Check complete independent bytes where both alignment and NMAX padding apply.
/// # Errors
/// Preserve unexpected constructor, formatter or reader refusal.
/// # Panics
/// Require exact placeholders, AAD, lengths, component padding and Additional Padding.
#[test]
fn multiple_placeholders_and_nmax_padding_match_the_complete_literal() -> Result<(), String> {
    let mut output = [0xa5; 168];
    let plan = prepare_nts_client_request(
        &header(&HEADER)?,
        &UID,
        &COOKIE,
        2,
        shape(3, 5, 16)?,
        &mut output,
        limits(164, 5)?,
    )
    .map_err(|error| format!("prepare: {error:?}"))?;
    assert_eq!(
        plan.associated_data(),
        &PADDED_TWO_PLACEHOLDER_PACKET[..132]
    );
    let complete = plan
        .finish(&[1, 2, 3], &[4, 5, 6, 7, 8])
        .map_err(|error| format!("finish: {error:?}"))?;
    assert_eq!(complete.packet_bytes(), PADDED_TWO_PLACEHOLDER_PACKET);
    let framed = frame_no_legacy_mac_datagram(complete.packet_bytes(), limits(164, 5)?)
        .map_err(|error| format!("secondary frame: {error:?}"))?;
    assert_eq!(framed.extension_count(), 5);
    let fields: Vec<_> = framed.extensions().collect();
    assert_eq!(fields[0].body(), UID);
    assert_eq!(fields[1].body(), COOKIE);
    for placeholder in &fields[2..4] {
        assert_eq!(placeholder.field_type(), 0x0304);
        assert_eq!(placeholder.body(), &[0; 12]);
    }
    let parts = read_nts_authenticator_parts(complete.packet_bytes(), limits(164, 5)?)
        .map_err(|error| format!("secondary envelope: {error:?}"))?;
    assert_eq!(
        parts.associated_data(),
        &PADDED_TWO_PLACEHOLDER_PACKET[..132]
    );
    assert_eq!(parts.nonce(), &[1, 2, 3]);
    assert_eq!(parts.ciphertext(), &[4, 5, 6, 7, 8]);
    assert_eq!(&output[164..], &[0xa5; 4]);
    Ok(())
}

/// Distinguish padded nonce length, min(16,NMAX), and the final-field minimum.
/// # Errors
/// Preserve any unexpected shape, preparation or finalization refusal.
/// # Panics
/// Require independently tabulated final spans, zero padding and exact data bytes.
#[test]
fn nonce_allowance_and_word_padding_have_independent_boundary_answers() -> Result<(), String> {
    let cases = [
        (0, 0, 0, 28),
        (1, 0, 1, 28),
        (1, 0, 5, 28),
        (1, 17, 5, 36),
        (3, 5, 16, 32),
        (4, 5, 16, 32),
        (13, 5, 16, 32),
        (16, 5, 16, 32),
        (17, 5, 17, 36),
        (3, 5, usize::MAX, 32),
    ];
    for (nonce_length, ciphertext_length, maximum, final_length) in cases {
        let mut output = [0xa5; 256];
        let nonce = vec![0x91; nonce_length];
        let ciphertext = vec![0xe7; ciphertext_length];
        let plan = prepare_nts_client_request(
            &header(&HEADER)?,
            &UID,
            &COOKIE,
            0,
            shape(nonce_length, ciphertext_length, maximum)?,
            &mut output,
            limits(256, 3)?,
        )
        .map_err(|error| format!("prepare: {error:?}"))?;
        let complete = plan
            .finish(&nonce, &ciphertext)
            .map_err(|error| format!("finish: {error:?}"))?;
        let packet = complete.packet_bytes();
        assert_eq!(packet.len(), 100 + final_length);
        assert_eq!(
            &packet[100..104],
            &[4, 4, 0, u8::try_from(final_length).map_err(|_| "span")?]
        );
        assert_eq!(
            &packet[104..106],
            u16::try_from(nonce_length)
                .map_err(|_| "nonce length")?
                .to_be_bytes()
        );
        assert_eq!(
            &packet[106..108],
            u16::try_from(ciphertext_length)
                .map_err(|_| "ciphertext length")?
                .to_be_bytes()
        );
        assert_eq!(&packet[108..108 + nonce_length], nonce);
        let ciphertext_start = 108 + nonce_length.next_multiple_of(4);
        assert!(
            packet[108 + nonce_length..ciphertext_start]
                .iter()
                .all(|byte| *byte == 0)
        );
        assert_eq!(
            &packet[ciphertext_start..ciphertext_start + ciphertext_length],
            ciphertext
        );
        assert!(
            packet[ciphertext_start + ciphertext_length..]
                .iter()
                .all(|byte| *byte == 0)
        );
    }
    Ok(())
}

/// Challenge every first-byte bit pattern through the actual preparation entry.
/// # Errors
/// Preserve unexpected header/limit/shape/finish failures for valid neighbors.
/// # Panics
/// Only version4/mode3 succeeds, independently of LI; refusals preserve all bytes.
#[test]
fn every_version_mode_and_leap_combination_reaches_request_header_admission() -> Result<(), String>
{
    for first in 0..=u8::MAX {
        let mut bytes = HEADER;
        bytes[0] = first;
        for (index, octet) in bytes[1..].iter_mut().enumerate() {
            *octet = u8::try_from(index + 1).map_err(|_| "nonzero header pattern")?;
        }
        let mut output = [0xa5; 128];
        let result = prepare_nts_client_request(
            &header(&bytes)?,
            &UID,
            &COOKIE,
            0,
            shape(0, 0, 0)?,
            &mut output,
            limits(128, 3)?,
        );
        if first & 0x3f == 0x23 {
            let plan = result.map_err(|error| format!("valid header: {error:?}"))?;
            let complete = plan
                .finish(&[], &[])
                .map_err(|error| format!("finish: {error:?}"))?;
            assert_eq!(&complete.packet_bytes()[..48], bytes);
        } else {
            assert!(matches!(
                result,
                Err(NtsRequestEncodeError::InvalidClientHeader)
            ));
            assert_eq!(output, [0xa5; 128]);
        }
    }
    Ok(())
}

/// Exercise exact-copy UID and opaque-cookie neighbors without changing KE rules.
/// # Errors
/// Preserve unexpected valid-neighbor construction or formatting refusal.
/// # Panics
/// Reject unsupported shapes atomically and retain every supported input octet.
#[test]
fn uid_and_cookie_neighbors_are_admitted_without_implicit_padding() -> Result<(), String> {
    for uid_length in [0, 28, 31, 32, 33, 35, 36] {
        for cookie_length in [0, 8, 11, 12, 13, 15, 16] {
            let uid = vec![0xd3; uid_length];
            let cookie = vec![0xb7; cookie_length];
            let mut output = [0xa5; 256];
            let result = prepare_nts_client_request(
                &header(&HEADER)?,
                &uid,
                &cookie,
                0,
                shape(0, 0, 0)?,
                &mut output,
                limits(256, 3)?,
            );
            if uid_length < 32 || uid_length % 4 != 0 {
                assert!(matches!(
                    result,
                    Err(NtsRequestEncodeError::UnsupportedUniqueIdentifier)
                ));
                assert_eq!(output, [0xa5; 256]);
            } else if cookie_length < 12 || cookie_length % 4 != 0 {
                assert!(matches!(
                    result,
                    Err(NtsRequestEncodeError::UnsupportedCookie)
                ));
                assert_eq!(output, [0xa5; 256]);
            } else {
                let plan = result.map_err(|error| format!("valid shape: {error:?}"))?;
                assert_eq!(&plan.associated_data()[52..52 + uid_length], uid);
                assert_eq!(&plan.associated_data()[56 + uid_length..], cookie);
                let complete = plan
                    .finish(&[], &[])
                    .map_err(|error| format!("finish: {error:?}"))?;
                assert_eq!(
                    complete.packet_bytes().len(),
                    84 + uid_length + cookie_length
                );
            }
        }
    }
    Ok(())
}

/// Check declared component representability independently of packet budgets.
/// # Panics
/// Require exact redacted error classes and valid neighboring shapes.
#[test]
fn shape_rejects_component_and_authenticator_overflow_without_an_nmax_default() {
    assert!(matches!(
        NtsRequestShape::new(4, 0, 3),
        Err(NtsRequestEncodeError::InvalidNonceLength)
    ));
    assert!(matches!(
        NtsRequestShape::new(65_536, 0, usize::MAX),
        Err(NtsRequestEncodeError::InvalidNonceLength)
    ));
    assert!(matches!(
        NtsRequestShape::new(0, 65_536, 0),
        Err(NtsRequestEncodeError::InvalidCiphertextLength)
    ));
    assert!(matches!(
        NtsRequestShape::new(usize::MAX, 0, usize::MAX),
        Err(NtsRequestEncodeError::InvalidNonceLength)
    ));
    assert!(matches!(
        NtsRequestShape::new(0, usize::MAX, 0),
        Err(NtsRequestEncodeError::InvalidCiphertextLength)
    ));
    assert!(matches!(
        NtsRequestShape::new(0, 65_525, 0),
        Err(NtsRequestEncodeError::AuthenticatorTooLong)
    ));
    assert!(matches!(
        NtsRequestShape::new(65_525, 0, usize::MAX),
        Err(NtsRequestEncodeError::AuthenticatorTooLong)
    ));
    assert!(NtsRequestShape::new(0, 65_524, 0).is_ok());
    assert!(NtsRequestShape::new(65_524, 0, usize::MAX).is_ok());
    assert!(NtsRequestShape::new(0, 0, usize::MAX).is_ok());
}

/// Apply separate whole-packet byte, output-capacity and extension-count budgets.
/// # Errors
/// Preserve unexpected constructor or valid-neighbor formatter failures.
/// # Panics
/// One-below refuses atomically; exact/one-above capacity and count remain useful.
#[test]
fn preparation_preflights_each_packet_budget_before_writing() -> Result<(), String> {
    for ceiling in [127, 128, 129] {
        let mut output = [0xa5; 136];
        let result = prepare_nts_client_request(
            &header(&HEADER)?,
            &UID,
            &COOKIE,
            0,
            shape(0, 0, 0)?,
            &mut output,
            limits(ceiling, 3)?,
        );
        if ceiling == 127 {
            assert!(matches!(
                result,
                Err(NtsRequestEncodeError::FrameLimit(FrameError::ByteLimit))
            ));
            assert_eq!(output, [0xa5; 136]);
        } else {
            let complete = result
                .map_err(|error| format!("byte neighbor: {error:?}"))?
                .finish(&[], &[])
                .map_err(|error| format!("finish: {error:?}"))?;
            assert_eq!(complete.packet_bytes(), EMPTY_COMPONENT_PACKET);
        }
    }
    for capacity in [127, 128, 129] {
        let mut output = vec![0xa5; capacity];
        let result = prepare_nts_client_request(
            &header(&HEADER)?,
            &UID,
            &COOKIE,
            0,
            shape(0, 0, 0)?,
            &mut output,
            limits(128, 3)?,
        );
        if capacity == 127 {
            assert!(matches!(result, Err(NtsRequestEncodeError::OutputTooSmall)));
            assert!(output.iter().all(|byte| *byte == 0xa5));
        } else {
            let complete = result
                .map_err(|error| format!("capacity neighbor: {error:?}"))?
                .finish(&[], &[])
                .map_err(|error| format!("finish: {error:?}"))?;
            assert_eq!(complete.packet_bytes(), EMPTY_COMPONENT_PACKET);
            assert!(output[128..].iter().all(|byte| *byte == 0xa5));
        }
    }
    for fields in [2, 3, 4] {
        let mut output = [0xa5; 128];
        let result = prepare_nts_client_request(
            &header(&HEADER)?,
            &UID,
            &COOKIE,
            0,
            shape(0, 0, 0)?,
            &mut output,
            limits(128, fields)?,
        );
        if fields == 2 {
            assert!(matches!(
                result,
                Err(NtsRequestEncodeError::FrameLimit(
                    FrameError::TooManyExtensions
                ))
            ));
            assert_eq!(output, [0xa5; 128]);
        } else {
            let complete = result
                .map_err(|error| format!("count neighbor: {error:?}"))?
                .finish(&[], &[])
                .map_err(|error| format!("finish: {error:?}"))?;
            assert_eq!(complete.packet_bytes(), EMPTY_COMPONENT_PACKET);
        }
    }
    Ok(())
}

/// Distinguish RFC placeholder guidance from this existing64-field resource ceiling.
/// # Errors
/// Preserve unexpected constructor or supported-neighbor formatter refusal.
/// # Panics
/// Sixty-one placeholders fit; sixty-two/`usize::MAX` refuse without mutation.
#[test]
fn placeholder_count_uses_the_existing_field_budget_including_all_three_fixed_fields()
-> Result<(), String> {
    for count in [0, 7, 8, 60, 61, 62, usize::MAX] {
        let mut output = [0xa5; 4096];
        let result = prepare_nts_client_request(
            &header(&HEADER)?,
            &UID,
            &COOKIE,
            count,
            shape(0, 0, 0)?,
            &mut output,
            limits(4096, 64)?,
        );
        if count > 61 {
            assert!(result.is_err());
            assert_eq!(output, [0xa5; 4096]);
        } else {
            let complete = result
                .map_err(|error| format!("count: {error:?}"))?
                .finish(&[], &[])
                .map_err(|error| format!("finish: {error:?}"))?;
            assert_eq!(complete.packet_bytes().len(), 128 + 16 * count);
            for placeholder in complete.packet_bytes()[100..100 + 16 * count]
                .as_chunks::<16>()
                .0
            {
                assert_eq!(&placeholder[..4], &[3, 4, 0, 16]);
                assert_eq!(&placeholder[4..], &[0; 12]);
            }
        }
    }
    Ok(())
}

/// A placeholder takes its exact body size from the admitted opaque cookie.
/// # Errors
/// Preserve unexpected constructor or supported-neighbor formatter refusal.
/// # Panics
/// Every varying-size placeholder has the required header, length and zero body.
#[test]
fn placeholder_body_length_tracks_each_supported_cookie_size() -> Result<(), String> {
    for cookie_length in [12, 16, 20] {
        for count in [1, 2] {
            let cookie = vec![0xb7; cookie_length];
            let mut output = [0xa5; 256];
            let plan = prepare_nts_client_request(
                &header(&HEADER)?,
                &UID,
                &cookie,
                count,
                shape(0, 0, 0)?,
                &mut output,
                limits(256, 5)?,
            )
            .map_err(|error| format!("prepare: {error:?}"))?;
            let field_length = 4 + cookie_length;
            let aad_length = 84 + (1 + count) * field_length;
            assert_eq!(plan.associated_data().len(), aad_length);
            assert_eq!(&plan.associated_data()[88..88 + cookie_length], cookie);
            let length = u16::try_from(field_length)
                .map_err(|_| "placeholder length")?
                .to_be_bytes();
            for placeholder in
                plan.associated_data()[84 + field_length..].chunks_exact(field_length)
            {
                assert_eq!(&placeholder[..4], &[3, 4, length[0], length[1]]);
                assert!(placeholder[4..].iter().all(|byte| *byte == 0));
            }
            let complete = plan
                .finish(&[], &[])
                .map_err(|error| format!("finish: {error:?}"))?;
            assert_eq!(complete.packet_bytes().len(), aad_length + 28);
        }
    }
    Ok(())
}

/// Reach the current4096-byte packet maximum with an exact-copy opaque cookie.
/// # Errors
/// Preserve unexpected constructor or exact-boundary formatter refusal.
/// # Panics
/// Cookie3980 fills4096; its aligned next neighbor refuses atomically.
#[test]
fn maximum_packet_bytes_is_independent_of_opaque_cookie_representability() -> Result<(), String> {
    for cookie_length in [3976, 3980, 3984] {
        let cookie = vec![0xb7; cookie_length];
        let mut output = [0xa5; 4104];
        let result = prepare_nts_client_request(
            &header(&HEADER)?,
            &UID,
            &cookie,
            0,
            shape(0, 0, 0)?,
            &mut output,
            limits(4096, 3)?,
        );
        if cookie_length == 3984 {
            assert!(matches!(
                result,
                Err(NtsRequestEncodeError::FrameLimit(FrameError::ByteLimit))
            ));
            assert_eq!(output, [0xa5; 4104]);
        } else {
            let complete = result
                .map_err(|error| format!("byte boundary: {error:?}"))?
                .finish(&[], &[])
                .map_err(|error| format!("finish: {error:?}"))?;
            assert_eq!(complete.packet_bytes().len(), 116 + cookie_length);
            assert_eq!(&complete.packet_bytes()[88..88 + cookie_length], cookie);
        }
    }
    Ok(())
}

/// Separate an encodable field that exceeds the packet budget from wire overflow.
/// # Errors
/// Preserve unexpected header, limit or declared-shape construction failures.
/// # Panics
/// Aligned65528-byte bodies reach `ByteLimit`; aligned65532-byte bodies are unsupported.
#[test]
fn uid_and_cookie_wire_length_neighbors_preserve_typed_refusals_and_storage() -> Result<(), String>
{
    for body_length in [65_528, 65_532] {
        let oversized = vec![0xb7; body_length];
        for oversized_uid in [true, false] {
            let uid = if oversized_uid {
                &oversized[..]
            } else {
                &UID[..]
            };
            let cookie = if oversized_uid {
                &COOKIE[..]
            } else {
                &oversized[..]
            };
            let mut output = [0xa5; 4096];
            let result = prepare_nts_client_request(
                &header(&HEADER)?,
                uid,
                cookie,
                0,
                shape(0, 0, 0)?,
                &mut output,
                limits(4096, 3)?,
            );
            if body_length == 65_528 {
                assert!(matches!(
                    result,
                    Err(NtsRequestEncodeError::FrameLimit(FrameError::ByteLimit))
                ));
            } else if oversized_uid {
                assert!(matches!(
                    result,
                    Err(NtsRequestEncodeError::UnsupportedUniqueIdentifier)
                ));
            } else {
                assert!(matches!(
                    result,
                    Err(NtsRequestEncodeError::UnsupportedCookie)
                ));
            }
            assert_eq!(output, [0xa5; 4096]);
        }
    }
    Ok(())
}

/// Observe finalization's own unchanged input, including the prepared prefix.
/// # Errors
/// Preserve unexpected valid preparation failures.
/// # Panics
/// Each mismatched component refuses before any tail or suffix write.
#[test]
fn finalization_length_refusals_preserve_the_complete_prepared_stage() -> Result<(), String> {
    for (nonce_length, ciphertext_length, nonce_mismatch) in [
        (2, 5, true),
        (2, 4, true),
        (4, 5, true),
        (3, 4, false),
        (3, 6, false),
    ] {
        let mut output = [0xa5; 168];
        let mut stage = output;
        stage[..132].copy_from_slice(&PADDED_TWO_PLACEHOLDER_PACKET[..132]);
        let plan = prepare_nts_client_request(
            &header(&HEADER)?,
            &UID,
            &COOKIE,
            2,
            shape(3, 5, 16)?,
            &mut output,
            limits(164, 5)?,
        )
        .map_err(|error| format!("prepare: {error:?}"))?;
        let result = plan.finish(&vec![1; nonce_length], &vec![2; ciphertext_length]);
        if nonce_mismatch {
            assert!(matches!(
                result,
                Err(NtsRequestEncodeError::NonceLengthMismatch)
            ));
        } else {
            assert!(matches!(
                result,
                Err(NtsRequestEncodeError::CiphertextLengthMismatch)
            ));
        }
        assert_eq!(output, stage);
    }
    Ok(())
}

/// Abandoning a plan exposes no completed view and initializes no tail bytes.
/// # Errors
/// Preserve unexpected constructor or preparation refusal.
/// # Panics
/// Only the complete checked AAD prefix is written before finalization.
#[test]
fn abandonment_writes_only_the_prepared_prefix() -> Result<(), String> {
    let mut output = [0xa5; 168];
    {
        let plan = prepare_nts_client_request(
            &header(&HEADER)?,
            &UID,
            &COOKIE,
            2,
            shape(3, 5, 16)?,
            &mut output,
            limits(164, 5)?,
        )
        .map_err(|error| format!("prepare: {error:?}"))?;
        assert_eq!(
            plan.associated_data(),
            &PADDED_TWO_PLACEHOLDER_PACKET[..132]
        );
    }
    assert_eq!(&output[..132], &PADDED_TWO_PLACEHOLDER_PACKET[..132]);
    assert_eq!(&output[132..], &[0xa5; 36]);
    Ok(())
}

/// Header, UID and cookie are copied; only destination lifetime owns the plan.
/// # Errors
/// Preserve unexpected constructor, preparation or finalization refusal.
/// # Panics
/// Temporary source inputs may expire before finalization without changing bytes.
#[test]
fn source_inputs_may_expire_after_preparation() -> Result<(), String> {
    let mut output = [0xa5; 128];
    let plan = {
        let local_header = header(&HEADER)?;
        let local_uid = UID;
        let local_cookie = COOKIE;
        prepare_nts_client_request(
            &local_header,
            &local_uid,
            &local_cookie,
            0,
            shape(0, 0, 0)?,
            &mut output,
            limits(128, 3)?,
        )
        .map_err(|error| format!("prepare: {error:?}"))?
    };
    assert_eq!(plan.associated_data(), &EMPTY_COMPONENT_PACKET[..100]);
    let complete = plan
        .finish(&[], &[])
        .map_err(|error| format!("finish: {error:?}"))?;
    assert_eq!(complete.packet_bytes(), EMPTY_COMPONENT_PACKET);
    Ok(())
}

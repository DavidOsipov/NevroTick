//! Literal RFC 8915 section 4/4.1.1/4.1.2/4.1.5 request facts.
//! REQ-8, BND-2, CHECK-6/25; no provider, transmission or TLS proof.
#![forbid(unsafe_code)]

use ntp_wire::{
    NtsKeFrameError, NtsKeFrameLimits, NtsKeNegotiationError, NtsKeRequestEncodeError,
    encode_nts_ke_ntpv4_request, frame_nts_ke_message, negotiate_nts_ke_ntpv4,
    read_nts_ke_ntpv4_offer,
};

/// Admit only explicit existing experiment limits.
/// # Errors
/// Preserve any unexpected limit rejection.
fn limits(bytes: usize, records: usize) -> Result<NtsKeFrameLimits, String> {
    NtsKeFrameLimits::new(bytes, records).map_err(|error| format!("limits: {error:?}"))
}

/// Admit the independent literal through the existing framing/offer consumers.
///
/// # Errors
/// Reject an invalid literal rather than attributing it to the new encoder.
///
/// # Panics
/// Require the exact original offered identifier and complete request bytes.
#[test]
fn request_encoder_literal_control_is_a_valid_existing_offer() -> Result<(), String> {
    let bytes = [0x80, 1, 0, 2, 0, 0, 0x80, 4, 0, 2, 0, 15, 0x80, 0, 0, 0];
    let raw = frame_nts_ke_message(&bytes, limits(16, 3)?)
        .map_err(|error| format!("literal frame: {error:?}"))?;
    let offer =
        read_nts_ke_ntpv4_offer(&raw).map_err(|error| format!("literal offer: {error:?}"))?;
    assert_eq!(offer.request_bytes(), bytes);
    assert_eq!(offer.offered_aead_bytes(), [0, 15]);
    Ok(())
}

/// Literal bytes, actual borrowed spans and real downstream consumers agree.
/// # Errors
/// Preserve any unexpected encoder, framing or semantic rejection.
/// # Panics
/// Require exact independent wire bytes, spans, success and peer-error result.
#[test]
fn request_encoder_returns_original_offer_and_literal_single_request() -> Result<(), String> {
    let expected = [0x80, 1, 0, 2, 0, 0, 0x80, 4, 0, 2, 0, 15, 0x80, 0, 0, 0];
    let mut storage = [0xa5; 24];
    let original = storage.as_ptr();
    let aead_original = storage[10..12].as_ptr();
    let cap = limits(16, 3)?;
    let offer = encode_nts_ke_ntpv4_request(&[15], &mut storage, cap)
        .map_err(|error| format!("encode: {error:?}"))?;
    assert_eq!(offer.request_bytes(), expected);
    assert!(core::ptr::eq(offer.request_bytes().as_ptr(), original));
    assert!(core::ptr::eq(
        offer.offered_aead_bytes().as_ptr(),
        aead_original
    ));
    assert_eq!(offer.offered_aead_bytes(), [0, 15]);
    let raw = frame_nts_ke_message(offer.request_bytes(), cap)
        .map_err(|error| format!("frame: {error:?}"))?;
    assert_eq!(
        raw.records()
            .map(|record| record.record_type())
            .collect::<Vec<_>>(),
        [1, 4, 0]
    );
    let checked = read_nts_ke_ntpv4_offer(&raw).map_err(|error| format!("offer: {error:?}"))?;
    assert_eq!(checked.request_bytes(), expected);
    let healthy = [
        0x80, 1, 0, 2, 0, 0, 0, 4, 0, 2, 0, 15, 0, 5, 0, 0, 0x80, 0, 0, 0,
    ];
    let peer_error = [0x80, 2, 0, 2, 0, 1, 0x80, 0, 0, 0];
    let response = frame_nts_ke_message(&healthy, limits(20, 4)?)
        .map_err(|error| format!("response: {error:?}"))?;
    assert_eq!(
        negotiate_nts_ke_ntpv4(&offer, response)
            .map_err(|error| format!("negotiate: {error:?}"))?
            .aead_id(),
        15
    );
    let response = frame_nts_ke_message(&peer_error, limits(10, 2)?)
        .map_err(|error| format!("error response: {error:?}"))?;
    assert_eq!(
        negotiate_nts_ke_ntpv4(&offer, response).err(),
        Some(NtsKeNegotiationError::PeerError { code: 1 })
    );
    assert_eq!(&storage[16..], &[0xa5; 8]);
    Ok(())
}

/// Supplied order, duplicates and the full u16 identifier representation survive.
/// # Errors
/// Preserve rejection of any independently valid two-identifier offer.
/// # Panics
/// Require exact 18-byte messages and the original AEAD body span.
#[test]
fn request_encoder_preserves_order_duplicates_and_generic_identifiers() -> Result<(), String> {
    for (ids, body) in [
        ([0x1234_u16, 15], [0x12_u8, 0x34, 0, 15]),
        ([15, 15], [0, 15, 0, 15]),
        ([0, u16::MAX], [0, 0, 0xff, 0xff]),
    ] {
        let expected = [
            0x80, 1, 0, 2, 0, 0, 0x80, 4, 0, 4, body[0], body[1], body[2], body[3], 0x80, 0, 0, 0,
        ];
        let mut storage = [0xa5; 18];
        let offer = encode_nts_ke_ntpv4_request(&ids, &mut storage, limits(18, 3)?)
            .map_err(|error| format!("encode: {error:?}"))?;
        assert_eq!(offer.request_bytes(), expected);
        assert_eq!(offer.offered_aead_bytes(), body);
        assert!(core::ptr::eq(
            offer.offered_aead_bytes().as_ptr(),
            offer.request_bytes()[10..14].as_ptr()
        ));
    }
    Ok(())
}

/// Every applicable admission failure precedes the first destination write.
/// # Errors
/// Preserve only an unexpected fixture-limit rejection.
/// # Panics
/// Require specific errors and unchanged destination, then an accepted neighbor.
#[test]
fn request_encoder_rejections_leave_the_entire_destination_unchanged() -> Result<(), String> {
    use NtsKeRequestEncodeError::{AeadOfferTooLong, EmptyAeadOffer, FrameLimit, OutputTooSmall};
    let overflow = vec![15; 32_768];
    for (ids, cap, available, expected) in [
        (&[][..], limits(16, 3)?, 16, EmptyAeadOffer),
        (
            overflow.as_slice(),
            limits(131_072, 3)?,
            16,
            AeadOfferTooLong,
        ),
        (
            &[15][..],
            limits(15, 3)?,
            16,
            FrameLimit(NtsKeFrameError::ByteLimit),
        ),
        (
            &[15][..],
            limits(16, 2)?,
            16,
            FrameLimit(NtsKeFrameError::TooManyRecords),
        ),
        (&[15][..], limits(16, 3)?, 15, OutputTooSmall),
    ] {
        let mut storage = [0xa5; 24];
        assert_eq!(
            encode_nts_ke_ntpv4_request(ids, &mut storage[..available], cap).err(),
            Some(expected)
        );
        assert_eq!(storage, [0xa5; 24]);
        let offer = encode_nts_ke_ntpv4_request(&[15], &mut storage, limits(16, 3)?)
            .map_err(|error| format!("healthy neighbor: {error:?}"))?;
        assert_eq!(offer.request_bytes().len(), 16);
        assert_eq!(&storage[16..], &[0xa5; 8]);
    }
    Ok(())
}

/// The largest even u16 body is 65,534 bytes: 32,767 identifiers.
/// # Errors
/// Preserve rejection of the exact representable-body boundary or its consumer.
/// # Panics
/// Require the independent 65,548-byte message, exact body and preserved tail.
#[test]
fn request_encoder_admits_the_exact_body_length_boundary() -> Result<(), String> {
    let ids = vec![15; 32_767];
    let mut storage = vec![0xa5; 65_556];
    let cap = limits(65_548, 3)?;
    let offer = encode_nts_ke_ntpv4_request(&ids, &mut storage, cap)
        .map_err(|error| format!("maximum encode: {error:?}"))?;
    assert_eq!(offer.request_bytes().len(), 65_548);
    assert_eq!(
        &offer.request_bytes()[..10],
        &[0x80, 1, 0, 2, 0, 0, 0x80, 4, 0xff, 0xfe]
    );
    assert_eq!(offer.offered_aead_bytes().len(), 65_534);
    assert!(
        offer
            .offered_aead_bytes()
            .as_chunks::<2>()
            .0
            .iter()
            .all(|pair| *pair == [0, 15])
    );
    assert_eq!(&offer.request_bytes()[65_544..], &[0x80, 0, 0, 0]);
    let raw = frame_nts_ke_message(offer.request_bytes(), cap)
        .map_err(|error| format!("maximum frame: {error:?}"))?;
    assert_eq!(raw.record_count(), 3);
    assert_eq!(
        read_nts_ke_ntpv4_offer(&raw)
            .map_err(|error| format!("maximum offer: {error:?}"))?
            .offered_aead_bytes()
            .len(),
        65_534
    );
    assert_eq!(&storage[65_548..], &[0xa5; 8]);
    Ok(())
}

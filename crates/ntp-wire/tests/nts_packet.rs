//! Independent inert RFC 8915 envelope facts; REQ-8/BND-2/CHECK-6/25.
#![forbid(unsafe_code)]

use ntp_wire::{
    FrameError, FrameLimits, NtsAuthenticatorError, NtsAuthenticatorParts,
    frame_no_legacy_mac_datagram, read_nts_authenticator_parts,
};

/// Nonzero raw header; this owner deliberately admits no mode/version semantics.
const HEADER: [u8; 48] = [0xa5; 48];
/// One opaque preceding field; UID-specific length policy is deliberately absent.
const PREFIX: [u8; 16] = [
    1, 4, 0, 16, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b,
];
/// Nonce1/ciphertext3, nonzero alignment bytes and twelve opaque extra bytes.
const AUTHENTICATOR: [u8; 28] = [
    4, 4, 0, 28, 0, 1, 0, 3, 0x11, 0xaa, 0xbb, 0xcc, 0x21, 0x22, 0x23, 0xdd, 0x91, 0x92, 0x93,
    0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c,
];

/// Serialize independently declared literals, with no production encoder oracle.
fn literal() -> Vec<u8> {
    let mut packet = HEADER.to_vec();
    packet.extend_from_slice(&PREFIX);
    packet.extend_from_slice(&AUTHENTICATOR);
    packet
}

/// Admit explicit existing experiment bounds, without a product-default claim.
/// # Errors
/// Preserve unexpected rejection by the existing checked limits owner.
fn limits() -> Result<FrameLimits, String> {
    FrameLimits::new(4096, 64).map_err(|error| format!("limits: {error:?}"))
}

/// Exercise the real public reader and preserve its actual redacted error.
/// # Errors
/// Propagate limit admission and actual envelope-reader rejection.
fn parts(input: &[u8]) -> Result<NtsAuthenticatorParts<'_>, String> {
    read_nts_authenticator_parts(input, limits()?).map_err(|error| format!("envelope: {error:?}"))
}

/// A healthy pre-existing framer control does not call the new envelope reader.
/// # Errors
/// Preserve any existing limits or framing failure.
/// # Panics
/// Require independently known field count, type and original body spans.
#[test]
fn nts_authenticator_literal_control_frames_existing_owner() -> Result<(), String> {
    let packet = literal();
    let raw = frame_no_legacy_mac_datagram(&packet, limits()?)
        .map_err(|error| format!("frame: {error:?}"))?;
    assert_eq!(raw.extension_count(), 2);
    let mut fields = raw.extensions();
    assert_eq!(fields.next().map(|field| field.bytes()), Some(&PREFIX[..]));
    assert_eq!(
        fields.next().map(|field| field.bytes()),
        Some(&AUTHENTICATOR[..])
    );
    assert!(fields.next().is_none());
    Ok(())
}

/// Literal byte ranges distinguish payload, both padding regions and exact AAD.
/// # Errors
/// Preserve actual reader rejection, including the intended rejecting-stub RED.
/// # Panics
/// Require original-pointer identity, independent payloads and view-drop survival.
#[test]
fn nts_authenticator_literal_parts_preserve_original_storage() -> Result<(), String> {
    let packet = literal();
    let (aad, nonce, ciphertext) = {
        let view = parts(&packet)?;
        (view.associated_data(), view.nonce(), view.ciphertext())
    };
    assert_eq!(aad.len(), 64, "independent original associated-data prefix");
    assert_eq!(&aad[..48], HEADER);
    assert_eq!(&aad[48..], PREFIX);
    assert_eq!(nonce, &[0x11]);
    assert_eq!(ciphertext, &[0x21, 0x22, 0x23]);
    assert_eq!(aad.as_ptr(), packet.as_ptr());
    assert_eq!(nonce.as_ptr(), packet[72..].as_ptr());
    assert_eq!(ciphertext.as_ptr(), packet[76..].as_ptr());
    Ok(())
}

/// Independently supplied padded widths avoid using the reader's rounding oracle.
/// # Panics
/// Each fixed recipe must fit the explicitly chosen 28-byte field.
fn small_packet(nonce_len: u8, nonce_width: usize, cipher_len: u8, cipher_width: usize) -> Vec<u8> {
    let mut packet = HEADER.to_vec();
    packet.extend_from_slice(&[4, 4, 0, 28, 0, nonce_len, 0, cipher_len]);
    packet.extend_from_slice(&[0x11, 0x12, 0x13, 0x14][..usize::from(nonce_len)]);
    packet.resize(56 + nonce_width, 0xa7);
    packet.extend_from_slice(&[0x21, 0x22, 0x23, 0x24][..usize::from(cipher_len)]);
    packet.resize(56 + nonce_width + cipher_width, 0xb8);
    assert!(packet.len() <= 76);
    packet.resize(76, 0xc9);
    packet
}

/// Explicit 0/1/3/4 neighbors include nonzero pad bytes and opaque empty payloads.
/// # Errors
/// Preserve an unexpected structural rejection at any fixed neighboring shape.
/// # Panics
/// Require declared bytes only; no algorithm/tag/nonce validity is inferred.
#[test]
fn nts_authenticator_declared_lengths_and_padding_neighbors() -> Result<(), String> {
    for (nonce_len, nonce_width) in [(0, 0), (1, 4), (3, 4), (4, 4)] {
        for (cipher_len, cipher_width) in [(0, 0), (1, 4), (3, 4), (4, 4)] {
            let packet = small_packet(nonce_len, nonce_width, cipher_len, cipher_width);
            let view = parts(&packet)?;
            assert_eq!(view.associated_data(), HEADER);
            assert_eq!(
                view.nonce(),
                &[0x11, 0x12, 0x13, 0x14][..usize::from(nonce_len)]
            );
            assert_eq!(
                view.ciphertext(),
                &[0x21, 0x22, 0x23, 0x24][..usize::from(cipher_len)]
            );
        }
    }
    Ok(())
}

/// Additional aligned padding is opaque regardless of value or chosen length.
/// # Errors
/// Preserve rejection of any otherwise structurally valid padding alternative.
/// # Panics
/// Require identical candidate ranges for three literal additional-padding sizes.
#[test]
fn nts_authenticator_additional_padding_has_no_semantic_effect() -> Result<(), String> {
    for (length, extra) in [(28, 0), (32, 4), (36, 8)] {
        let mut packet = small_packet(1, 4, 3, 4);
        packet[51] = length;
        packet.extend_from_slice(&[0xee; 8][..extra]);
        let view = parts(&packet)?;
        assert_eq!(view.associated_data(), HEADER);
        assert_eq!(view.nonce(), &[0x11]);
        assert_eq!(view.ciphertext(), &[0x21, 0x22, 0x23]);
    }
    Ok(())
}

/// The literal rounded spans 12+8 fill a 28-byte field after its eight-byte head.
/// # Errors
/// Preserve rejection of either independently admitted neighboring layout.
/// # Panics
/// Require payload-only projections and reject the 16+8 span needing 32 bytes.
#[test]
fn nts_authenticator_padded_components_fill_exact_field() -> Result<(), String> {
    for (nonce_len, nonce_width) in [(3, 4), (9, 12)] {
        let mut packet = HEADER.to_vec();
        packet.extend_from_slice(&[4, 4, 0, 28, 0, nonce_len, 0, 5]);
        packet.extend_from_slice(
            &[0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19][..usize::from(nonce_len)],
        );
        packet.resize(56 + nonce_width, 0xa7);
        packet.extend_from_slice(&[0x21, 0x22, 0x23, 0x24, 0x25]);
        packet.resize(56 + nonce_width + 8, 0xb8);
        packet.resize(76, 0xc9);
        let view = parts(&packet)?;
        assert_eq!(
            view.nonce(),
            &[0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19][..usize::from(nonce_len)]
        );
        assert_eq!(view.ciphertext(), &[0x21, 0x22, 0x23, 0x24, 0x25]);
        packet[53] = 13;
        assert_eq!(
            read_nts_authenticator_parts(&packet, limits()?).err(),
            Some(NtsAuthenticatorError::InvalidAuthenticatorLength)
        );
    }
    Ok(())
}

/// The largest frame admits 4040 nonce bytes; ciphertext keeps high-byte lengths.
/// # Errors
/// Preserve the healthy exact-4096-byte neighbor's actual reader result.
/// # Panics
/// Reject the next rounded range and full-u16 declarations with no panic.
#[test]
fn nts_authenticator_range_and_u16_extremes_are_checked() -> Result<(), String> {
    let mut packet = vec![0xa6; 4096];
    packet[48..56].copy_from_slice(&[4, 4, 0x0f, 0xd0, 0x0f, 0xc8, 0, 0]);
    assert_eq!(parts(&packet)?.nonce(), &[0xa6; 4040]);
    for (cipher_len, field_word, cipher_word, packet_len) in
        [(256, [1, 8], [1, 0], 312), (257, [1, 12], [1, 1], 316)]
    {
        let mut wide = vec![0xbc; packet_len];
        wide[48..56].copy_from_slice(&[
            4,
            4,
            field_word[0],
            field_word[1],
            0,
            0,
            cipher_word[0],
            cipher_word[1],
        ]);
        let view = parts(&wide)?;
        assert_eq!(view.nonce(), []);
        assert_eq!(view.ciphertext().len(), cipher_len);
        assert_eq!(view.ciphertext(), &wide[56..56 + cipher_len]);
        assert_eq!(view.ciphertext().as_ptr(), wide[56..].as_ptr());
    }
    for lengths in [[0x0f, 0xc9, 0, 0], [0xff, 0xff, 0, 0], [0, 0, 0xff, 0xff]] {
        packet[52..56].copy_from_slice(&lengths);
        assert_eq!(
            read_nts_authenticator_parts(&packet, limits()?).err(),
            Some(NtsAuthenticatorError::InvalidAuthenticatorLength)
        );
    }
    Ok(())
}

/// Nonfinal minimum fields exercise exact-fit and rounded-nonce/cipher overflow.
/// # Errors
/// Preserve the healthy 4+4 payload shape in an outer-valid minimum field.
/// # Panics
/// Reject either declared padded span exceeding the actual selected field.
#[test]
fn nts_authenticator_small_field_ranges_do_not_cross_the_suffix() -> Result<(), String> {
    let mut packet = HEADER.to_vec();
    packet.extend_from_slice(&[4, 4, 0, 16, 0, 4, 0, 4, 1, 2, 3, 4, 5, 6, 7, 8]);
    packet.extend_from_slice(&[0xff; 28]);
    packet[66..68].copy_from_slice(&[0, 28]);
    assert_eq!(parts(&packet)?.ciphertext(), &[5, 6, 7, 8]);
    for lengths in [[0, 9, 0, 0], [0, 5, 0, 1]] {
        packet[52..56].copy_from_slice(&lengths);
        assert_eq!(
            read_nts_authenticator_parts(&packet, limits()?).err(),
            Some(NtsAuthenticatorError::InvalidAuthenticatorLength)
        );
    }
    Ok(())
}

/// Even another authenticator with impossible inner lengths is discarded suffix.
/// # Errors
/// Preserve acceptance independently of each outer-valid trailing field's type.
/// # Panics
/// Require that suffix bytes cannot change or replace the first envelope result.
#[test]
fn nts_authenticator_well_framed_suffix_is_semantically_discarded() -> Result<(), String> {
    for field_type in [[4, 4], [1, 4], [2, 4], [0xff, 0xff]] {
        let mut packet = literal();
        packet.extend_from_slice(&[0xff; 28]);
        packet[92..96].copy_from_slice(&[field_type[0], field_type[1], 0, 28]);
        let view = parts(&packet)?;
        assert_eq!(view.associated_data(), &packet[..64]);
        assert_eq!(view.nonce(), &[0x11]);
        assert_eq!(view.ciphertext(), &[0x21, 0x22, 0x23]);
    }
    let mut packet = literal();
    packet[68..72].copy_from_slice(&[0xff, 0xff, 0, 3]);
    packet.extend_from_slice(&AUTHENTICATOR);
    assert_eq!(
        read_nts_authenticator_parts(&packet, limits()?).err(),
        Some(NtsAuthenticatorError::InvalidAuthenticatorLength)
    );
    Ok(())
}

/// Complete outer admission precedes even a healthy first authenticator result.
/// # Errors
/// Preserve checked limits construction.
/// # Panics
/// Require exact framing causes for malformed suffixes, byte/count excess and EOF.
#[test]
fn nts_authenticator_outer_failures_precede_envelope_projection() -> Result<(), String> {
    let mut packet = literal();
    packet.extend_from_slice(&[4, 4, 0, 28]);
    assert_eq!(
        read_nts_authenticator_parts(&packet, limits()?).err(),
        Some(NtsAuthenticatorError::Frame(FrameError::TruncatedExtension))
    );
    packet[94..96].copy_from_slice(&[0, 12]);
    assert_eq!(
        read_nts_authenticator_parts(&packet, limits()?).err(),
        Some(NtsAuthenticatorError::Frame(
            FrameError::InvalidExtensionLength
        ))
    );
    assert_eq!(
        read_nts_authenticator_parts(
            &literal(),
            FrameLimits::new(91, 64).map_err(|e| format!("limits: {e:?}"))?
        )
        .err(),
        Some(NtsAuthenticatorError::Frame(FrameError::ByteLimit))
    );
    assert_eq!(
        read_nts_authenticator_parts(
            &literal(),
            FrameLimits::new(4096, 1).map_err(|e| format!("limits: {e:?}"))?
        )
        .err(),
        Some(NtsAuthenticatorError::Frame(FrameError::TooManyExtensions))
    );
    assert_eq!(
        read_nts_authenticator_parts(&HEADER[..47], limits()?).err(),
        Some(NtsAuthenticatorError::Frame(FrameError::HeaderTooShort))
    );
    assert_eq!(
        read_nts_authenticator_parts(&HEADER, limits()?).err(),
        Some(NtsAuthenticatorError::MissingAuthenticator)
    );
    Ok(())
}

// SPDX-License-Identifier: GPL-2.0-only
//! Public component controls; supplied keys and entropy simulate no TLS success.

use super::{DirectionalKeys, RequestError};
use ntp_wire::{FrameError, NtsRequestEncodeError};
use zeroize::Zeroizing;

/// Existing complete packet from the separately admitted independent oracle.
/// Inputs and provenance remain in the AES witness fixture README.
const PACKET: &[u8; 140] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tools/crypto-aes-siv-witness/tests/fixtures/sealing-zero.bin"
));
/// Public C2S fixture key, whose first half affects this empty-plaintext tag.
const C2S: [u8; 32] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31,
];
/// Public unmodified cookie from the same independent complete-packet fixture.
const COOKIE: [u8; 12] = [
    0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab,
];
/// Public UID32 followed by the independent fixture's nonce16.
const ENTROPY: [u8; 48] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
    0x1c, 0x1d, 0x1e, 0x1f,
];

/// Construct only public component inputs; confer no exporter provenance.
fn pair(c2s: [u8; 32], s2c: [u8; 32]) -> DirectionalKeys {
    DirectionalKeys::new(Zeroizing::new(c2s), Zeroizing::new(s2c))
}

/// The actual helper reproduces the full independent packet, not only its count.
///
/// # Errors
/// Preserve any unexpected component refusal of the public healthy fixture.
///
/// # Panics
/// The actual component differs from the independent packet or refusal contract.
#[test]
fn complete_request_matches_existing_independent_answer() -> Result<(), RequestError> {
    let mut storage = [0xa5; 256];
    {
        let keys = pair(C2S, [0x55; 32]);
        let DirectionalKeys { s2c_key, .. } = &keys;
        assert_eq!(*keys.c2s_key, C2S);
        assert_eq!(**s2c_key, [0x55; 32]);
        let request = keys.seal_request(&COOKIE, &ENTROPY, &mut storage)?;
        assert_eq!(request.packet_bytes(), PACKET);
        assert_eq!(*keys.c2s_key, C2S);
        assert_eq!(**s2c_key, [0x55; 32]);
    }
    assert!(storage[140..].iter().all(|byte| *byte == 0xa5));
    Ok(())
}

/// Changing the paired response key cannot change this outgoing request.
///
/// # Errors
/// Preserve refusal of either otherwise healthy directional fixture.
///
/// # Panics
/// The actual component differs from the independent packet or refusal contract.
#[test]
fn s2c_changes_do_not_select_the_outgoing_key() -> Result<(), RequestError> {
    for s2c in [[0x55; 32], [0xaa; 32]] {
        let mut storage = [0; 140];
        let request = pair(C2S, s2c).seal_request(&COOKIE, &ENTROPY, &mut storage)?;
        assert_eq!(request.packet_bytes(), PACKET);
    }
    Ok(())
}

/// Wrong direction and a changed C2S CMAC half break the independent tag answer.
///
/// # Errors
/// Preserve an unexpected component refusal; changed keys are still valid widths.
///
/// # Panics
/// The actual component differs from the independent packet or refusal contract.
#[test]
fn swapped_or_changed_c2s_cannot_match_the_independent_answer() -> Result<(), RequestError> {
    let mut altered = C2S;
    altered[0] ^= 1;
    for (c2s, s2c) in [([0x55; 32], C2S), (altered, [0x55; 32])] {
        let mut storage = [0; 140];
        let request = pair(c2s, s2c).seal_request(&COOKIE, &ENTROPY, &mut storage)?;
        assert_eq!(&request.packet_bytes()[..124], &PACKET[..124]);
        assert_ne!(&request.packet_bytes()[124..], &PACKET[124..]);
    }
    Ok(())
}

/// UID and nonce use their exact disjoint portions of the supplied entropy block.
///
/// # Errors
/// Preserve refusal of either otherwise healthy fixed-width entropy fixture.
///
/// # Panics
/// The actual component differs from the independent packet or refusal contract.
#[test]
fn uid_and_nonce_changes_reach_their_actual_packet_fields() -> Result<(), RequestError> {
    for (entropy_offset, packet_offset) in [(0, 52), (32, 108)] {
        let mut entropy = ENTROPY;
        entropy[entropy_offset] ^= 1;
        let mut storage = [0; 140];
        let request = pair(C2S, [0x55; 32]).seal_request(&COOKIE, &entropy, &mut storage)?;
        assert_eq!(
            request.packet_bytes()[packet_offset],
            entropy[entropy_offset]
        );
        assert_ne!(&request.packet_bytes()[124..], &PACKET[124..]);
    }
    Ok(())
}

/// Empty, short or unaligned cookies are refused without rewriting storage.
///
/// # Panics
/// The actual component differs from the independent packet or refusal contract.
#[test]
fn cookie_representability_refusals_preserve_the_destination() {
    for cookie in [&[][..], &[0xa0; 11][..], &[0xa0; 13][..]] {
        let mut storage = [0xa5; 256];
        let result = pair(C2S, [0x55; 32]).seal_request(cookie, &ENTROPY, &mut storage);
        assert_eq!(
            result.err(),
            Some(RequestError::Format(
                NtsRequestEncodeError::UnsupportedCookie
            ))
        );
        assert_eq!(storage, [0xa5; 256]);
    }
}

/// Exact packet capacity succeeds; its next aligned cookie exceeds the byte cap.
///
/// # Errors
/// Preserve an unexpected refusal at the independently calculated exact limit.
///
/// # Panics
/// The actual component differs from the independent packet or refusal contract.
#[test]
fn maximum_cookie_fits_and_next_aligned_cookie_is_refused() -> Result<(), RequestError> {
    let mut storage = [0xa5; 4096];
    let request = pair(C2S, [0x55; 32]).seal_request(&[0xab; 3968], &ENTROPY, &mut storage)?;
    assert_eq!(request.packet_bytes().len(), 4096);
    let mut larger_storage = [0xa5; 4100];
    let result = pair(C2S, [0x55; 32]).seal_request(&[0xab; 3972], &ENTROPY, &mut larger_storage);
    assert_eq!(
        result.err(),
        Some(RequestError::Format(NtsRequestEncodeError::FrameLimit(
            FrameError::ByteLimit
        )))
    );
    assert_eq!(larger_storage, [0xa5; 4100]);
    Ok(())
}

/// A one-byte capacity deficit does not publish or partially prepare a packet.
///
/// # Panics
/// The actual component differs from the independent packet or refusal contract.
#[test]
fn capacity_refusal_preserves_the_entire_destination() {
    let mut storage = [0xa5; 139];
    let result = pair(C2S, [0x55; 32]).seal_request(&COOKIE, &ENTROPY, &mut storage);
    assert_eq!(
        result.err(),
        Some(RequestError::Format(NtsRequestEncodeError::OutputTooSmall))
    );
    assert_eq!(storage, [0xa5; 139]);
}

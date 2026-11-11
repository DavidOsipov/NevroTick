// SPDX-License-Identifier: GPL-2.0-only
//! Actual S2C component controls with independent public valid-tag packets.
//! These fixed keys and packets establish no TLS/exporter/native provenance.

use nevrotick_s115_aes_siv_fixture::staged::StagedDecryptError;
use ntp_wire::{
    FrameError, FrameLimits, NtsPlaintextError, NtsPlaintextLimits, read_nts_authenticator_parts,
};
use zeroize::Zeroizing;

use super::fixtures;
use super::{DirectionalKeys, ReplyError, stage_reply};

/// Public C2S input; its two halves both differ from the response key.
const C2S: [u8; 32] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31,
];
/// Public actual response-fixture key, never a TLS exporter result.
const S2C: [u8; 32] = [
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f,
    0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f,
];
/// The exact public UID echoed by every healthy packet.
const UID: [u8; 32] = C2S;
/// Saved wire token, deliberately unrelated to any real local T1.
const ORIGIN: u64 = 0x1122_3344_5566_7788;
/// Exact independent server receive bits, with no era or quality claim.
const RECEIVE: u64 = 0x2233_4455_6677_8899;
/// Exact independent server transmit bits, distinct from receive.
const TRANSMIT: u64 = 0x3344_5566_7788_99aa;
/// Exact public opaque replacement body, not regenerated through the receiver.
const COOKIE: [u8; 12] = [
    0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab,
];

/// Supply only fixed public material to the actual directional owner.
fn keys(c2s: [u8; 32], s2c: [u8; 32]) -> DirectionalKeys {
    DirectionalKeys::new(Zeroizing::new(c2s), Zeroizing::new(s2c))
}

/// Prove semantic-negative fixture tags reach the real opening consumer first.
/// # Errors
/// Preserve actual bounded framing/opening failures instead of relabelling them.
/// # Panics
/// Fail if a supposed valid-tag fixture never enters its semantic consumer.
fn assert_valid_tag(raw: &[u8]) -> Result<(), ReplyError> {
    let limits = FrameLimits::new(4096, 64).map_err(ReplyError::Frame)?;
    let parts = read_nts_authenticator_parts(raw, limits).map_err(ReplyError::Envelope)?;
    let inner = NtsPlaintextLimits::new(4096, 64)
        .map_err(|error| ReplyError::Open(StagedDecryptError::Plaintext(error)))?;
    let mut scratch = [0xa5; 4096];
    let entered = keys(C2S, S2C)
        .open_response(&parts, &mut scratch, inner, |_| true)
        .map_err(ReplyError::Open)?;
    assert!(entered);
    assert!(scratch.iter().all(|byte| *byte == 0));
    Ok(())
}

/// Healthy admission preserves all independent coordinates and exact cookie bytes.
/// # Errors
/// Preserve unexpected refusal of the independent valid-tag neighbor.
/// # Panics
/// Fail on wrong key direction, transformed bytes or lost raw coordinates.
#[test]
fn healthy_response_stages_exact_coordinates_and_cookie_once() -> Result<(), ReplyError> {
    assert_eq!(fixtures::HEALTHY.len(), 140);
    let staged = stage_reply(&keys(C2S, S2C), fixtures::HEALTHY, &UID, ORIGIN, 1)?;
    assert_eq!(staged.cookie_count(), 1);
    assert_eq!(staged.cookie_bytes(), 12);
    let (fields, cookies) = staged.into_parts();
    let expected_header =
        ntp_wire::decode_fixed_header(&fixtures::HEALTHY[..48]).map_err(|_| ReplyError::Header)?;
    assert_eq!(fields.header(), expected_header);
    assert_eq!(fields.origin().bits(), ORIGIN);
    assert_eq!(fields.receive().bits(), RECEIVE);
    assert_eq!(fields.transmit().bits(), TRANSMIT);
    assert_eq!(cookies.len(), 1);
    assert_eq!(cookies[0].as_slice(), COOKIE);
    Ok(())
}

/// Only the paired S2C direction opens replies; changing C2S has no influence.
/// # Errors
/// Preserve an unexpected healthy-neighbor refusal.
/// # Panics
/// Fail if a wrong or modified response key publishes staged material.
#[test]
fn s2c_direction_is_required_and_c2s_is_not_used_to_open() -> Result<(), ReplyError> {
    stage_reply(&keys([0xee; 32], S2C), fixtures::HEALTHY, &UID, ORIGIN, 1)?;
    let mut changed_first = S2C;
    changed_first[0] ^= 1;
    let mut changed_second = S2C;
    changed_second[16] ^= 1;
    for response_key in [C2S, changed_first, changed_second] {
        assert!(matches!(
            stage_reply(&keys(S2C, response_key), fixtures::HEALTHY, &UID, ORIGIN, 1),
            Err(ReplyError::Open(StagedDecryptError::Authentication))
        ));
    }
    Ok(())
}

/// Fresh valid tags distinguish semantic refusals from generic AEAD tampering.
/// # Errors
/// Preserve a claimed fixture's unexpected tag/framing failure.
/// # Panics
/// Fail if any semantic-negative fixture enters staging or has a wrong category.
#[test]
fn valid_tag_header_uid_origin_and_reserved_placement_refuse() -> Result<(), ReplyError> {
    for (raw, expected) in [
        (fixtures::WRONG_VERSION, ReplyError::Header),
        (fixtures::WRONG_MODE, ReplyError::Header),
        (fixtures::UNSYNCHRONIZED, ReplyError::Header),
        (fixtures::STRATUM_ZERO, ReplyError::Header),
        (fixtures::STRATUM_SIXTEEN, ReplyError::Header),
        (fixtures::WRONG_ORIGIN, ReplyError::Origin),
        (fixtures::WRONG_UID, ReplyError::UniqueIdentifier),
        (fixtures::MISSING_UID, ReplyError::UniqueIdentifier),
        (fixtures::SHORT_UID, ReplyError::UniqueIdentifier),
        (fixtures::DUPLICATE_UID, ReplyError::UniqueIdentifier),
        (fixtures::CLEAR_COOKIE, ReplyError::Placement),
        (fixtures::CLEAR_PLACEHOLDER, ReplyError::Placement),
        (fixtures::INNER_UID, ReplyError::Placement),
        (fixtures::INNER_AUTH, ReplyError::Placement),
        (fixtures::INNER_PLACEHOLDER, ReplyError::Placement),
    ] {
        assert_valid_tag(raw)?;
        assert!(matches!(
            stage_reply(&keys(C2S, S2C), raw, &UID, ORIGIN, 1),
            Err(error) if error == expected
        ));
    }
    stage_reply(&keys(C2S, S2C), fixtures::HEALTHY, &UID, ORIGIN, 1)?;
    Ok(())
}

/// Zero fields are a local policy refusal; an encrypted empty body remains a field.
/// The ledger separately proves that repeated refusals retain spent work/deadline.
/// # Errors
/// Preserve unexpected failure of either actual opening or the empty-body neighbor.
/// # Panics
/// Fail if zero fields publish or if opaque empty bodies are silently discarded.
#[test]
fn authenticated_zero_cookie_count_differs_from_one_empty_cookie() -> Result<(), ReplyError> {
    assert_eq!(fixtures::ZERO_COOKIES.len(), 124);
    assert_valid_tag(fixtures::ZERO_COOKIES)?;
    assert!(matches!(
        stage_reply(&keys(C2S, S2C), fixtures::ZERO_COOKIES, &UID, ORIGIN, 1),
        Err(ReplyError::Cookies)
    ));
    let staged = stage_reply(&keys(C2S, S2C), fixtures::EMPTY_COOKIE, &UID, ORIGIN, 1)?;
    assert_eq!(staged.cookie_count(), 1);
    assert_eq!(staged.cookie_bytes(), 0);
    let (_, cookies) = staged.into_parts();
    assert!(cookies[0].is_empty());
    Ok(())
}

/// Count every cookie field, including repeated identical opaque bodies.
/// # Errors
/// Preserve unexpected refusal of an admitted replacement-count neighbor.
/// # Panics
/// Fail if duplicates are deduplicated, over-allowance cookies admitted or eight lost.
#[test]
fn replacement_count_respects_saved_placeholder_allowance_and_eight_cap() -> Result<(), ReplyError>
{
    assert_valid_tag(fixtures::TWO_COOKIES)?;
    assert!(matches!(
        stage_reply(&keys(C2S, S2C), fixtures::TWO_COOKIES, &UID, ORIGIN, 1),
        Err(ReplyError::Cookies)
    ));
    let two = stage_reply(&keys(C2S, S2C), fixtures::TWO_COOKIES, &UID, ORIGIN, 2)?;
    assert_eq!(two.cookie_count(), 2);
    assert_eq!(two.cookie_bytes(), 24);
    let (_, copies) = two.into_parts();
    assert_eq!(copies[0].as_slice(), COOKIE);
    assert_eq!(copies[1].as_slice(), COOKIE);
    let eight = stage_reply(&keys(C2S, S2C), fixtures::EIGHT_COOKIES, &UID, ORIGIN, 8)?;
    assert_eq!(eight.cookie_count(), 8);
    assert_eq!(eight.cookie_bytes(), 96);
    assert_valid_tag(fixtures::NINE_COOKIES)?;
    assert!(matches!(
        stage_reply(&keys(C2S, S2C), fixtures::NINE_COOKIES, &UID, ORIGIN, 8),
        Err(ReplyError::Cookies)
    ));
    Ok(())
}

/// Unknown fields are inert; even reserved-looking unauthenticated suffixes discard.
/// # Errors
/// Preserve refusal of otherwise valid protected prefixes.
/// # Panics
/// Fail if an ignored field changes authenticated coordinates/cookies.
#[test]
fn unknown_fields_and_unauthenticated_suffix_cannot_change_staged_material()
-> Result<(), ReplyError> {
    for raw in [
        fixtures::UNKNOWN_FIELDS_AND_SUFFIX,
        fixtures::SUFFIX_DUPLICATE_UID_AUTH,
    ] {
        let staged = stage_reply(&keys(C2S, S2C), raw, &UID, ORIGIN, 1)?;
        assert_eq!(staged.cookie_count(), 1);
        assert_eq!(staged.cookie_bytes(), 12);
        let (fields, cookies) = staged.into_parts();
        assert_eq!(fields.origin().bits(), ORIGIN);
        assert_eq!(fields.receive().bits(), RECEIVE);
        assert_eq!(fields.transmit().bits(), TRANSMIT);
        assert_eq!(cookies[0].as_slice(), COOKIE);
    }
    Ok(())
}

/// Work bounds retain valid exact neighbors rather than merely rejecting all input.
/// # Errors
/// Preserve unexpected rejection of exact parser ceilings.
/// # Panics
/// Fail if an over-bound complete packet/field sequence reaches staged output.
#[test]
fn exact_packet_outer_and_inner_limits_have_rejecting_upper_neighbors() -> Result<(), ReplyError> {
    assert_eq!(fixtures::MAX_PACKET_BYTES.len(), 4096);
    assert_eq!(fixtures::TOO_MANY_PACKET_BYTES.len(), 4100);
    for raw in [
        fixtures::MAX_PACKET_BYTES,
        fixtures::MAX_OUTER_FIELDS,
        fixtures::MAX_INNER_FIELDS,
    ] {
        stage_reply(&keys(C2S, S2C), raw, &UID, ORIGIN, 1)?;
    }
    assert!(matches!(
        stage_reply(
            &keys(C2S, S2C),
            fixtures::TOO_MANY_PACKET_BYTES,
            &UID,
            ORIGIN,
            1
        ),
        Err(ReplyError::Frame(FrameError::ByteLimit))
    ));
    assert!(matches!(
        stage_reply(&keys(C2S, S2C), fixtures::TOO_MANY_OUTER, &UID, ORIGIN, 1),
        Err(ReplyError::Frame(FrameError::TooManyExtensions))
    ));
    assert!(matches!(
        stage_reply(&keys(C2S, S2C), fixtures::TOO_MANY_INNER, &UID, ORIGIN, 1),
        Err(ReplyError::Open(StagedDecryptError::Plaintext(
            NtsPlaintextError::TooManyFields
        )))
    ));
    assert!(matches!(
        stage_reply(&keys(C2S, S2C), fixtures::MALFORMED_INNER, &UID, ORIGIN, 1),
        Err(ReplyError::Open(StagedDecryptError::Plaintext(
            NtsPlaintextError::InvalidFieldLength
        )))
    ));
    Ok(())
}

/// Raw invalid allowances fail before parsing, beside a real admitted neighbor.
/// # Errors
/// Preserve the otherwise valid allowance's unexpected refusal.
/// # Panics
/// Fail if a caller bypasses the one-through-eight replacement bound.
#[test]
fn replacement_allowance_requires_checked_nonzero_bounded_input() -> Result<(), ReplyError> {
    for maximum in [0, 9, usize::MAX] {
        assert!(matches!(
            stage_reply(&keys(C2S, S2C), &[], &UID, ORIGIN, maximum),
            Err(ReplyError::Limits)
        ));
    }
    stage_reply(&keys(C2S, S2C), fixtures::HEALTHY, &UID, ORIGIN, 1)?;
    Ok(())
}

/// Algorithm15 requires at least one nonce octet, not a fixed sixteen on receive.
/// These valid-tag neighbors distinguish semantic admission from AEAD refusal.
/// # Errors
/// Preserve unexpected framing/opening or nonempty-nonce neighbor refusal.
/// # Panics
/// Fail if an empty nonce stages, or if valid one/twelve-octet nonces are rejected.
#[test]
fn selected_algorithm_nonce_minimum_preserves_short_valid_neighbors() -> Result<(), ReplyError> {
    for (raw, length) in [
        (fixtures::NONCE_ONE, 1),
        (fixtures::NONCE_TWELVE, 12),
        (fixtures::NONCE_ZERO, 0),
    ] {
        let limits = FrameLimits::new(4096, 64).map_err(ReplyError::Frame)?;
        let parts = read_nts_authenticator_parts(raw, limits).map_err(ReplyError::Envelope)?;
        assert_eq!(parts.nonce().len(), length);
        assert_valid_tag(raw)?;
        let result = stage_reply(&keys(C2S, S2C), raw, &UID, ORIGIN, 1);
        if length == 0 {
            assert!(result.is_err());
        } else {
            let staged = result?;
            assert_eq!(staged.cookie_count(), 1);
            assert_eq!(staged.cookie_bytes(), 12);
            let (_, cookies) = staged.into_parts();
            assert_eq!(cookies[0].as_slice(), COOKIE);
        }
    }
    Ok(())
}

/// Mutate bytes that remain semantically plausible so actual AEAD must refuse.
/// # Panics
/// Fail if altered authenticated header, nonce, tag or body publishes material.
#[test]
fn authenticated_header_nonce_tag_and_body_tampering_refuse() {
    // Literal HEALTHY layout: AAD84, auth header8, nonce16, tag16, inner body16.
    for offset in [12, 92, 108, 124] {
        let mut packet = fixtures::HEALTHY.to_vec();
        packet[offset] ^= 1;
        assert!(matches!(
            stage_reply(&keys(C2S, S2C), &packet, &UID, ORIGIN, 1),
            Err(ReplyError::Open(StagedDecryptError::Authentication))
        ));
    }
}

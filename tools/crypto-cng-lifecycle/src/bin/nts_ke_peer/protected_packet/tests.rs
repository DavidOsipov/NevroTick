// SPDX-License-Identifier: GPL-2.0-only
//! Literal wire expectations for the protected fixture codec.

use aes_siv::{KeyInit, Tag, siv::Aes128Siv};
use zeroize::Zeroizing;

use super::{PacketFailure, admit_and_seal};

/// Public test-only inputs, distinct in every cryptographic role.
const C2S: [u8; 32] = [0x3a; 32];
/// Public S2C key. It must not authenticate the input.
const S2C: [u8; 32] = [0xc9; 32];
/// Request fixture nonce, distinct from the reply nonce.
const CLIENT_NONCE: [u8; 16] = [0x11; 16];
/// The reply's explicitly supplied test nonce.
const SERVER_NONCE: [u8; 16] = [0x44; 16];
/// Original correlation from the request transmit field.
const WIRE: [u8; 8] = [0x53, 0x46, 0xf8, 0x05, 0x35, 0x77, 0xaa, 0x02];
/// Exact UID supplied in the public test packet.
const UID: [u8; 32] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31,
];

/// Stamp a literal client packet from source-independent field offsets.
///
/// # Errors
/// Refuse AEAD setup failure rather than returning an unauthenticated fixture.
fn request() -> Result<[u8; 252], &'static str> {
    let mut packet: [u8; 252] = [0; 252];
    packet[0] = 0x23;
    packet[40..48].copy_from_slice(&WIRE);
    packet[48..52].copy_from_slice(&[0x01, 0x04, 0, 36]);
    packet[52..84].copy_from_slice(&UID);
    packet[84..100].copy_from_slice(&[
        0x02, 0x04, 0, 16, 0xaa, 0xbb, 0xcc, 0xdd, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80,
    ]);
    let placeholders: &mut [u8] = &mut packet[100..212];
    let (fields, remainder): (&mut [[u8; 16]], &mut [u8]) = placeholders.as_chunks_mut();
    if !remainder.is_empty() {
        return Err("invalid literal placeholder extent");
    }
    for field in fields {
        field[..4].copy_from_slice(&[0x03, 0x04, 0, 16]);
    }
    packet[212..220].copy_from_slice(&[0x04, 0x04, 0, 40, 0, 16, 0, 16]);
    packet[220..236].copy_from_slice(&CLIENT_NONCE);
    stamp(&mut packet)?;
    Ok(packet)
}

/// Restamp selected changed public fields without using the client encoder.
///
/// # Errors
/// Preserve provider refusal, leaving the previous tag intact.
fn stamp(packet: &mut [u8; 252]) -> Result<(), &'static str> {
    let mut empty: [u8; 0] = [];
    let tag: Tag = Aes128Siv::new((&C2S).into())
        .encrypt_inout_detached([&packet[..212], &packet[220..236]], (&mut empty[..]).into())
        .map_err(|_| "fixture-tag")?;
    packet[236..252].copy_from_slice(tag.as_slice());
    Ok(())
}

/// Open the literal server ciphertext with the supplied key and exact wire AAD.
///
/// # Errors
/// Refuse unauthenticated output without exposing plaintext.
fn open_reply(reply: &[u8; 140], key: &[u8; 32]) -> Result<Zeroizing<[u8; 16]>, &'static str> {
    let raw_tag: &[u8; 16] = reply[108..124].try_into().map_err(|_| "reply-tag")?;
    let mut plaintext: Zeroizing<[u8; 16]> =
        Zeroizing::new(reply[124..140].try_into().map_err(|_| "reply-body")?);
    Aes128Siv::new(key.into())
        .decrypt_inout_detached(
            [&reply[..84], &reply[92..108]],
            (&mut plaintext[..]).into(),
            raw_tag.into(),
        )
        .map_err(|_| "reply-authentication")?;
    Ok(plaintext)
}

/// Compare original correlation, exact extension bytes and maintained AEAD output.
///
/// # Errors
/// Preserve unexpected fixture creation, admission or opening failure.
/// # Panics
/// Fail if header, UID, nonce, encrypted content or direction has changed.
#[test]
fn authenticated_packet_returns_one_correlated_encrypted_cookie() -> Result<(), &'static str> {
    let input: [u8; 252] = request()?;
    let reply: Zeroizing<[u8; 140]> =
        admit_and_seal(&input, &C2S, &S2C, &SERVER_NONCE).map_err(|_| "valid-request")?;
    assert_eq!(&reply[..2], &[0x24, 2]);
    assert_eq!(&reply[2..16], &[0; 14]);
    assert_eq!(&reply[16..24], &[1, 0x23, 0x45, 0x67, 0x89, 0xab, 0xc0, 0]);
    assert_eq!(&reply[24..32], &WIRE);
    assert_eq!(
        &reply[32..48],
        &[
            1, 0x23, 0x45, 0x67, 0x89, 0xab, 0xd0, 0, 1, 0x23, 0x45, 0x67, 0x89, 0xab, 0xe0, 0
        ]
    );
    assert_eq!(&reply[48..52], &[1, 4, 0, 36]);
    assert_eq!(&reply[52..84], &UID);
    assert_eq!(&reply[84..92], &[4, 4, 0, 56, 0, 16, 0, 32]);
    assert_eq!(&reply[92..108], &SERVER_NONCE);
    let cookie_field: [u8; 16] = [
        2, 4, 0, 16, 0x83, 0x73, 0x63, 0x53, 0x43, 0x33, 0x23, 0x13, 0x91, 0x92, 0x93, 0x94,
    ];
    assert_ne!(reply[124..140], cookie_field);
    assert_eq!(*open_reply(&reply, &S2C)?, cookie_field);
    assert!(open_reply(&reply, &C2S).is_err());
    Ok(())
}

/// Authenticate a second legal UID and origin and challenge changes after sealing.
///
/// # Errors
/// Preserve fixture construction or independent healthy opening failure.
/// # Panics
/// Fail if valid correlation is ignored or modified correlation/ciphertext opens.
#[test]
fn exact_authenticated_originals_bind_uid_origin_and_encrypted_cookie() -> Result<(), &'static str>
{
    let mut input: [u8; 252] = request()?;
    input[47] = 0x99;
    input[52] = 0x42;
    stamp(&mut input)?;
    let reply: Zeroizing<[u8; 140]> =
        admit_and_seal(&input, &C2S, &S2C, &SERVER_NONCE).map_err(|_| "valid-neighbor")?;
    assert_eq!(reply[31], 0x99);
    assert_eq!(reply[52], 0x42);
    drop(open_reply(&reply, &S2C)?);
    for index in [31, 52, 92, 108, 124] {
        let mut altered: [u8; 140] = *reply;
        altered[index] ^= 1;
        assert!(open_reply(&altered, &S2C).is_err());
    }
    Ok(())
}

/// Distinguish length/header/shape refusals even when the modified AAD is signed.
///
/// # Errors
/// Preserve failure to authenticate the independently prepared inputs.
/// # Panics
/// Fail if any invalid selected shape receives a sealed reply.
#[test]
fn fixed_shape_checks_precede_authentication_success() -> Result<(), &'static str> {
    let valid: [u8; 252] = request()?;
    assert_eq!(
        admit_and_seal(&valid[..251], &C2S, &S2C, &SERVER_NONCE).err(),
        Some(PacketFailure::Length)
    );
    let mut longer: [u8; 253] = [0; 253];
    longer[..252].copy_from_slice(&valid);
    assert_eq!(
        admit_and_seal(&longer, &C2S, &S2C, &SERVER_NONCE).err(),
        Some(PacketFailure::Length)
    );
    let mut zero_origin: [u8; 252] = valid;
    zero_origin[40..48].fill(0);
    stamp(&mut zero_origin)?;
    assert_eq!(
        admit_and_seal(&zero_origin, &C2S, &S2C, &SERVER_NONCE).err(),
        Some(PacketFailure::Header)
    );
    for header in [0x1b, 0x24] {
        let mut invalid: [u8; 252] = valid;
        invalid[0] = header;
        stamp(&mut invalid)?;
        assert_eq!(
            admit_and_seal(&invalid, &C2S, &S2C, &SERVER_NONCE).err(),
            Some(PacketFailure::Header)
        );
    }
    for index in [84, 88, 101, 104, 217, 219] {
        let mut invalid: [u8; 252] = valid;
        invalid[index] ^= 1;
        stamp(&mut invalid)?;
        assert_eq!(
            admit_and_seal(&invalid, &C2S, &S2C, &SERVER_NONCE).err(),
            Some(PacketFailure::Shape)
        );
    }
    Ok(())
}

/// Unknown keys or tampered authenticated bytes never produce a reply.
///
/// # Errors
/// Preserve independent request construction failure.
/// # Panics
/// Fail if a swapped direction, altered accepted-shape AAD, nonce or tag opens.
#[test]
fn request_authentication_refuses_wrong_direction_and_tampering() -> Result<(), &'static str> {
    let valid: [u8; 252] = request()?;
    assert_eq!(
        admit_and_seal(&valid, &S2C, &C2S, &SERVER_NONCE).err(),
        Some(PacketFailure::Authentication)
    );
    for index in [47, 52, 220, 236] {
        let mut invalid: [u8; 252] = valid;
        invalid[index] ^= 1;
        assert_eq!(
            admit_and_seal(&invalid, &C2S, &S2C, &SERVER_NONCE).err(),
            Some(PacketFailure::Authentication)
        );
    }
    Ok(())
}

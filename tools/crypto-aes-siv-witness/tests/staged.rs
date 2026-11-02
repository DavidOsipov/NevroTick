// SPDX-License-Identifier: GPL-2.0-only
//! Fixture-only adapter checks; no NTS interoperability or trust claim.
#![forbid(unsafe_code)]

use aes_siv::{KeyInit, siv::Aes128Siv};
use ntp_wire::{FrameLimits, NtsPlaintextLimits, read_nts_authenticator_parts};

// Public test data; the RFC 5297 KAT in lib.rs independently tests the primitive.
const KEY: [u8; 32] = [0x42; 32];
const HEADER: [u8; 48] = [0x23; 48];
const PREFIX: [u8; 16] = [
    0x01, 0x04, 0, 16, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b,
];
const NONCE: [u8; 12] = [
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
];
const PLAIN: [u8; 12] = [0x12, 0x34, 0, 4, 0x02, 0x04, 0, 8, 0xde, 0xad, 0xbe, 0xef];
type TestResult<T = ()> = Result<T, &'static str>;

/// Construct the wire format independently of the adapter and its parser.
/// The ciphertext format is the RFC 5297 SIV tag followed by encrypted bytes.
fn packet(plaintext: &[u8]) -> TestResult<Vec<u8>> {
    let mut aad = HEADER.to_vec();
    aad.extend_from_slice(&PREFIX);
    let mut body = plaintext.to_vec();
    let mut cipher = Aes128Siv::new_from_slice(&KEY).map_err(|_| "fixture-key")?;
    let tag = cipher
        .encrypt_inout_detached([aad.as_slice(), &NONCE[..]], (&mut body[..]).into())
        .map_err(|_| "fixture-encryption")?;
    let ciphertext_len = 16 + body.len();
    let field_len = 8 + NONCE.len() + ciphertext_len;
    let field_len = u16::try_from(field_len).map_err(|_| "fixture-length")?;
    let ciphertext_len = u16::try_from(ciphertext_len).map_err(|_| "fixture-length")?;
    let mut output = aad;
    output.extend_from_slice(&[0x04, 0x04]);
    output.extend_from_slice(&field_len.to_be_bytes());
    output.extend_from_slice(&[0, 12]);
    output.extend_from_slice(&ciphertext_len.to_be_bytes());
    output.extend_from_slice(&NONCE);
    output.extend_from_slice(tag.as_slice());
    output.extend_from_slice(&body);
    Ok(output)
}

fn outer_limits() -> TestResult<FrameLimits> {
    FrameLimits::new(4096, 64).map_err(|_| "outer-limits")
}

fn inner_limits(bytes: usize, fields: usize) -> TestResult<NtsPlaintextLimits> {
    NtsPlaintextLimits::new(bytes, fields).map_err(|_| "inner-limits")
}

use nevrotick_s115_aes_siv_fixture::staged::{StagedDecryptError, with_decrypted_frame};
use ntp_wire::NtsPlaintextError;
use std::{
    cell::Cell,
    panic::{AssertUnwindSafe, catch_unwind},
};

/// Authentication and full framing publish exactly the literal field sequence.
#[test]
fn authenticated_fields_reach_consumer_once_and_clear_all_scratch() -> TestResult {
    let wire = packet(&PLAIN)?;
    let parts = read_nts_authenticator_parts(&wire, outer_limits()?).map_err(|_| "outer-frame")?;
    assert_eq!(parts.associated_data(), [&HEADER[..], &PREFIX[..]].concat());
    assert_eq!(parts.nonce(), NONCE);
    let mut scratch = [0xa5; 4096];
    let calls = Cell::new(0);
    let result = with_decrypted_frame(&parts, &KEY, &mut scratch, inner_limits(12, 2)?, |frame| {
        calls.set(calls.get() + 1);
        assert_eq!(frame.field_count(), 2);
        let fields: Vec<_> = frame.fields().collect();
        assert_eq!(fields[0].field_type(), 0x1234);
        assert_eq!(fields[0].bytes(), &[0x12, 0x34, 0, 4]);
        assert_eq!(fields[0].body(), &[]);
        assert_eq!(fields[1].field_type(), 0x0204);
        assert_eq!(
            fields[1].bytes(),
            &[0x02, 0x04, 0, 8, 0xde, 0xad, 0xbe, 0xef]
        );
        assert_eq!(fields[1].body(), &[0xde, 0xad, 0xbe, 0xef]);
        73
    });
    assert_eq!(result, Ok(73));
    assert_eq!(calls.get(), 1);
    assert!(scratch.iter().all(|b| *b == 0));
    Ok(())
}

/// An empty authenticated sequence is valid and is consumed once.
#[test]
fn authenticated_empty_plaintext_is_valid() -> TestResult {
    let wire = packet(&[])?;
    let parts = read_nts_authenticator_parts(&wire, outer_limits()?).map_err(|_| "outer-frame")?;
    let mut scratch = [0xa5; 4096];
    let calls = Cell::new(0);
    let result = with_decrypted_frame(&parts, &KEY, &mut scratch, inner_limits(0, 0)?, |frame| {
        calls.set(calls.get() + 1);
        assert_eq!(frame.field_count(), 0);
        assert!(frame.fields().next().is_none());
    });
    assert_eq!(result, Ok(()));
    assert_eq!(calls.get(), 1);
    assert!(scratch.iter().all(|b| *b == 0));
    Ok(())
}

/// Every tamper remains outer-frameable but cannot reach the consumer.
#[test]
fn modified_aad_nonce_tag_body_and_short_tag_never_publish() -> TestResult {
    let original = packet(&PLAIN)?;
    // Literal offsets follow the fixed 48-byte header and 16-byte prefix:
    // auth header 64..72, nonce 72..84, tag 84..100, body 100..112.
    for (offset, expected) in [
        (0, StagedDecryptError::Authentication),
        (52, StagedDecryptError::Authentication),
        (72, StagedDecryptError::Authentication),
        (84, StagedDecryptError::Authentication),
        (100, StagedDecryptError::Authentication),
        (70, StagedDecryptError::ShortTag),
    ] {
        let mut wire = original.clone();
        if offset == 70 {
            wire[70..72].copy_from_slice(&15_u16.to_be_bytes());
        } else {
            wire[offset] ^= 1;
        }
        let parts = read_nts_authenticator_parts(&wire, outer_limits()?)
            .map_err(|_| "mutated-outer-frame")?;
        let mut scratch = [0xa5; 4096];
        let calls = Cell::new(0);
        let result = with_decrypted_frame(&parts, &KEY, &mut scratch, inner_limits(12, 2)?, |_| {
            calls.set(calls.get() + 1);
        });
        assert_eq!(calls.get(), 0);
        assert_eq!(result, Err(expected), "fixed mutation offset {offset}");
        assert!(scratch.iter().all(|b| *b == 0));
    }
    let parts = read_nts_authenticator_parts(&original, outer_limits()?)
        .map_err(|_| "original-outer-frame")?;
    let mut wrong_key = KEY;
    wrong_key[0] ^= 1;
    let mut scratch = [0xa5; 4096];
    let calls = Cell::new(0);
    let result = with_decrypted_frame(
        &parts,
        &wrong_key,
        &mut scratch,
        inner_limits(12, 2)?,
        |_| {
            calls.set(calls.get() + 1);
        },
    );
    assert_eq!(calls.get(), 0);
    assert_eq!(result, Err(StagedDecryptError::Authentication));
    assert!(scratch.iter().all(|b| *b == 0));
    Ok(())
}

/// Authenticate malformed sequences before testing complete framing and budgets.
#[test]
fn authenticated_invalid_inner_fields_and_limits_never_publish() -> TestResult {
    let truncated = [0x12, 0x34, 0, 4, 0x02, 0x04, 0, 12, 0xde, 0xad, 0xbe, 0xef];
    let short = [0x12, 0x34, 0, 4, 0x02, 0x04, 0, 2, 0xde, 0xad, 0xbe, 0xef];
    let unaligned = [0x12, 0x34, 0, 4, 0x02, 0x04, 0, 7, 0xde, 0xad, 0xbe, 0xef];
    for (plain, max_bytes, max_fields, expected) in [
        (&truncated[..], 12, 2, NtsPlaintextError::TruncatedField),
        (&short[..], 12, 2, NtsPlaintextError::InvalidFieldLength),
        (&unaligned[..], 12, 2, NtsPlaintextError::InvalidFieldLength),
        (&PLAIN[..], 11, 2, NtsPlaintextError::ByteLimit),
        (&PLAIN[..], 12, 1, NtsPlaintextError::TooManyFields),
    ] {
        let wire = packet(plain)?;
        let parts =
            read_nts_authenticator_parts(&wire, outer_limits()?).map_err(|_| "outer-frame")?;
        let mut scratch = [0xa5; 4096];
        let calls = Cell::new(0);
        let result = with_decrypted_frame(
            &parts,
            &KEY,
            &mut scratch,
            inner_limits(max_bytes, max_fields)?,
            |_| calls.set(calls.get() + 1),
        );
        assert_eq!(calls.get(), 0);
        assert_eq!(result, Err(StagedDecryptError::Plaintext(expected)));
        assert!(scratch.iter().all(|b| *b == 0));
    }
    Ok(())
}

/// Existing outer admission makes adapter-capacity failure unconstructible.
/// The largest reachable packet instead exercises the real bound without forging parts.
#[test]
fn maximum_outer_packet_authenticates_without_capacity_failure() -> TestResult {
    let plain = [0x12, 0x34, 0, 4].repeat(999);
    let wire = packet(&plain)?;
    assert_eq!(wire.len(), 4096);
    let parts =
        read_nts_authenticator_parts(&wire, outer_limits()?).map_err(|_| "maximum-outer-frame")?;
    let mut scratch = [0xa5; 4096];
    let calls = Cell::new(0);
    let result = with_decrypted_frame(
        &parts,
        &KEY,
        &mut scratch,
        inner_limits(3996, 999)?,
        |frame| {
            calls.set(calls.get() + 1);
            assert_eq!(frame.field_count(), 999);
            assert!(
                frame
                    .fields()
                    .all(|field| field.bytes() == [0x12, 0x34, 0, 4])
            );
        },
    );
    assert_eq!(result, Ok(()));
    assert_eq!(calls.get(), 1);
    assert!(scratch.iter().all(|b| *b == 0));
    Ok(())
}

/// A consumer panic still drops the scratch owner and clears its full extent.
#[test]
fn consumer_unwind_clears_all_scratch() -> TestResult {
    let wire = packet(&PLAIN)?;
    let parts = read_nts_authenticator_parts(&wire, outer_limits()?).map_err(|_| "outer-frame")?;
    let mut scratch = [0xa5; 4096];
    let calls = Cell::new(0);
    let limits = inner_limits(12, 2)?;
    let result = catch_unwind(AssertUnwindSafe(|| {
        let _ = with_decrypted_frame(&parts, &KEY, &mut scratch, limits, |_| {
            calls.set(calls.get() + 1);
            panic!("fixed-consumer-unwind-control")
        });
    }));
    assert!(result.is_err());
    assert_eq!(calls.get(), 1);
    assert!(scratch.iter().all(|b| *b == 0));
    Ok(())
}

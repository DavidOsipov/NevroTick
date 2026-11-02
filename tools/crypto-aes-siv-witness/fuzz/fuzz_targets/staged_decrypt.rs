// SPDX-License-Identifier: GPL-2.0-only
//! Standalone, bounded staged-AES semantic fuzz oracle; no product adoption.
//!
//! Input: lane, corruption, two BE byte-budget bytes, two BE field-budget bytes,
//! then independently selected payload. Raw mode selects at most3996 bytes;
//! constructed mode places at most3988 payload bytes into a guaranteed-valid
//! two-field sequence. Budgets never select or truncate payload. Six corruption
//! modes are healthy/tag/AAD/nonce/body/wrong-key. An empty body has no body byte
//! to corrupt, so that particular mode explicitly falls back to corrupting tag.
//! Encryption constructs reachable states using the maintained primitive; it
//! does not replace the separate independent RFC known-answer tests.
//! All keys/seeds are public fixture data. Assertions are semantic checks, not
//! ASan detector controls. No Windows, PowerShell, network or clock effect occurs.
#![no_main]
#![forbid(unsafe_code)]

use aes_siv::{KeyInit, siv::Aes128Siv};
use libfuzzer_sys::fuzz_target;
use nevrotick_s115_aes_siv_fixture::staged::{StagedDecryptError, with_decrypted_frame};
use ntp_wire::{
    FrameLimits, NtsPlaintextError, NtsPlaintextLimits, RawNtsPlaintext,
    read_nts_authenticator_parts,
};
use std::sync::atomic::{AtomicBool, Ordering};

/// Fixed public fixture key; never an application or negotiated secret.
const KEY: [u8; 32] = [0x42; 32];
/// Fixed public nonce; this experiment makes no general nonce-policy claim.
const NONCE: [u8; 12] = [
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
];
/// Select the nonempty literal valid sequence with exact12-byte/two-field limits.
const HEALTHY_SEED: &[u8] = &[1, 0, 0, 12, 0, 2, 0xde, 0xad, 0xbe, 0xef];
/// With64 AAD,12 nonce and16 tag bytes, this produces at most4096 outer bytes.
const MAX_INNER: usize = 3996;
/// A fixed comparison failure, without dumping candidate plaintext or key bytes.
const ORACLE: &str = "independent staged AES semantic oracle";
/// A process-local output bound only; this flag never changes test semantics.
static WITNESS_PRINTED: AtomicBool = AtomicBool::new(false);

/// Missing selector bytes are zero; selectors are disjoint from the payload.
fn selector(data: &[u8], index: usize) -> u8 {
    data.get(index).copied().unwrap_or(0)
}

/// Decode exactly two bounded selector bytes independently of production code.
fn budget_word(data: &[u8], start: usize) -> usize {
    usize::from(selector(data, start)) * 256 + usize::from(selector(data, start + 1))
}

/// Select original payload, then independently construct or preserve its bytes.
fn plaintext(data: &[u8]) -> Vec<u8> {
    let payload = data.get(6..).unwrap_or(&[]);
    if selector(data, 0) % 2 == 0 {
        return payload[..payload.len().min(MAX_INNER)].to_vec();
    }
    let payload = &payload[..payload.len().min(MAX_INNER - 8)];
    let padded = payload.len().div_ceil(4) * 4;
    let length = u16::try_from(4 + padded).expect("bounded constructed field length");
    let mut result = Vec::with_capacity(8 + padded);
    result.extend_from_slice(&[0x12, 0x34, 0, 4, 0x02, 0x04]);
    result.extend_from_slice(&length.to_be_bytes());
    result.extend_from_slice(payload);
    result.resize(8 + padded, 0);
    result
}

/// Bind every selected original payload byte to the independently known grammar.
///
/// This check is outside the constructor: dropping/changing payload in that
/// helper cannot change the expected input silently. Payload and budget selectors
/// remain disjoint, including the nonempty fixed forwarding witness.
fn check_selection(data: &[u8], original: &[u8]) {
    let payload = data.get(6..).unwrap_or(&[]);
    if selector(data, 0) % 2 == 0 {
        assert!(original == &payload[..payload.len().min(3996)], "{ORACLE}");
    } else {
        let kept = payload.len().min(3988);
        let words = (kept + 3) / 4;
        let padded = 4 * words;
        assert_eq!(original.len(), 8 + padded, "{ORACLE}");
        assert!(original[..6] == [0x12, 0x34, 0, 4, 0x02, 0x04], "{ORACLE}");
        assert_eq!(
            usize::from(original[6]) * 256 + usize::from(original[7]),
            4 + padded,
            "{ORACLE}"
        );
        assert!(original[8..8 + kept] == payload[..kept], "{ORACLE}");
        assert!(
            original[8 + kept..].iter().all(|byte| *byte == 0),
            "{ORACLE}"
        );
    }
}

/// Model complete inner framing using a four-byte-word countdown, not its parser.
///
/// Byte rejection precedes count rejection, then prefix/length/extent checks.
/// Successful word consumption is bounded by the3996-byte original plaintext;
/// a declared oversized extent stops after the first unavailable word.
fn reference(
    input: &[u8],
    byte_limit: usize,
    field_limit: usize,
) -> Result<usize, NtsPlaintextError> {
    if input.len() > byte_limit {
        return Err(NtsPlaintextError::ByteLimit);
    }
    let mut at = 0;
    let mut count = 0;
    while at < input.len() {
        if count == field_limit {
            return Err(NtsPlaintextError::TooManyFields);
        }
        if input.len() - at < 4 {
            return Err(NtsPlaintextError::InvalidFieldLength);
        }
        let length = usize::from(input[at + 2]) * 256 + usize::from(input[at + 3]);
        if length < 4 || length % 4 != 0 {
            return Err(NtsPlaintextError::InvalidFieldLength);
        }
        let mut words = length / 4;
        while words != 0 {
            if input.len() - at < 4 {
                return Err(NtsPlaintextError::TruncatedField);
            }
            at += 4;
            words -= 1;
        }
        count += 1;
    }
    Ok(count)
}

/// Encrypt selected bytes directly under literal ordered AAD/nonce components.
///
/// A64-byte original prefix contains a48-byte header plus a16-byte extension.
/// The aligned authenticator carries nonce12, tag16, ciphertext and zero padding.
/// The returned body starts at byte100 and the tag at byte84, independently of
/// any production formatting or inner-framing helper. Every Vec length/capacity
/// requested here is at most4096 bytes.
fn packet(plain: &[u8]) -> Vec<u8> {
    assert!(plain.len() <= MAX_INNER, "{ORACLE}");
    let mut aad = [0x23; 64];
    aad[48..64].copy_from_slice(&[
        0x01, 0x04, 0, 16, 0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b,
    ]);
    let mut body = plain.to_vec();
    let mut cipher = Aes128Siv::new((&KEY).into());
    let tag = cipher
        .encrypt_inout_detached([&aad[..], &NONCE[..]], (&mut body[..]).into())
        .expect("bounded two-component public fixture encryption");
    let cipher_length = 16 + body.len();
    let padded = cipher_length.div_ceil(4) * 4;
    let field_length = 8 + NONCE.len() + padded;
    let mut result = Vec::with_capacity(64 + field_length);
    result.extend_from_slice(&aad);
    result.extend_from_slice(&[4, 4]);
    result.extend_from_slice(
        &u16::try_from(field_length)
            .expect("bounded field")
            .to_be_bytes(),
    );
    result.extend_from_slice(&[0, 12]);
    result.extend_from_slice(
        &u16::try_from(cipher_length)
            .expect("bounded ciphertext")
            .to_be_bytes(),
    );
    result.extend_from_slice(&NONCE);
    result.extend_from_slice(tag.as_slice());
    result.extend_from_slice(&body);
    result.resize(64 + field_length, 0);
    result
}

/// Compare the consumer's original field views to the original modeled bytes.
///
/// Reference success proves all indexed prefixes/extents. The scratch pointer is
/// used only for equality/offset checks, never dereferenced. Empty field bodies
/// must retain their actual one-past-prefix original scratch address too.
fn compare_fields(frame: &RawNtsPlaintext<'_>, original: &[u8], count: usize, base: *const u8) {
    assert_eq!(frame.field_count(), count, "{ORACLE}");
    let mut fields = frame.fields();
    let mut at = 0;
    for _ in 0..count {
        let field = fields.next().expect(ORACLE);
        let length = usize::from(original[at + 2]) * 256 + usize::from(original[at + 3]);
        let kind = u16::from(original[at]) * 256 + u16::from(original[at + 1]);
        let end = at + length;
        assert_eq!(field.field_type(), kind, "{ORACLE}");
        assert!(field.bytes() == &original[at..end], "{ORACLE}");
        assert!(field.body() == &original[at + 4..end], "{ORACLE}");
        assert!(field.bytes().as_ptr() == base.wrapping_add(at), "{ORACLE}");
        assert!(
            field.body().as_ptr() == base.wrapping_add(at + 4),
            "{ORACLE}"
        );
        at = end;
    }
    assert!(fields.next().is_none() && at == original.len(), "{ORACLE}");
}

/// Execute one modeled input; no unrelated fixed positive call is hidden here.
fn check(data: &[u8]) {
    // This deliberate harness defect has no internal comparison. Its finite
    // control MUST be rejected externally for a missing healthy witness, even
    // when libFuzzer itself exits normally. It proves no sanitizer detection.
    if cfg!(feature = "seeded-staged-no-oracle") {
        return;
    }
    let original = plaintext(data);
    check_selection(data, &original);
    let byte_limit = budget_word(data, 2) % 4001;
    let field_limit = budget_word(data, 4) % 1001;
    let modeled = reference(&original, byte_limit, field_limit);
    let corruption = selector(data, 1) % 6;
    let expected = if corruption == 0 {
        modeled.map_err(StagedDecryptError::Plaintext)
    } else {
        Err(StagedDecryptError::Authentication)
    };

    let mut forwarded = original.clone();
    if cfg!(feature = "seeded-staged-empty-forwarding") {
        forwarded.clear();
    } else if cfg!(feature = "seeded-staged-alter-forwarding") && forwarded.len() > 8 {
        // The nonempty healthy witness changes a BODY byte, preserving framing.
        forwarded[8] ^= 1;
    }
    let mut wire = packet(&forwarded);
    let mut key = KEY;
    match corruption {
        0 => {}
        1 => wire[84] ^= 1,
        2 => wire[0] ^= 1,
        3 => wire[72] ^= 1,
        4 => {
            let offset = if forwarded.is_empty() { 84 } else { 100 };
            wire[offset] ^= 1;
        }
        5 => key[0] ^= 1,
        _ => unreachable!("modulo-six selector"),
    }
    let parts = read_nts_authenticator_parts(
        &wire,
        FrameLimits::new(4096, 2).expect("fixed outer limits"),
    )
    .expect("independently constructed bounded outer packet");
    assert!(parts.associated_data() == &wire[..64], "{ORACLE}");
    assert!(
        parts.associated_data().as_ptr() == wire.as_ptr(),
        "{ORACLE}"
    );
    assert!(parts.nonce() == &wire[72..84], "{ORACLE}");
    assert!(parts.nonce().as_ptr() == wire[72..84].as_ptr(), "{ORACLE}");
    assert!(
        parts.ciphertext() == &wire[84..100 + forwarded.len()],
        "{ORACLE}"
    );

    let supplied_bytes = if cfg!(feature = "seeded-staged-wrong-budget") {
        MAX_INNER
    } else {
        byte_limit
    };
    let limits =
        NtsPlaintextLimits::new(supplied_bytes, field_limit).expect("bounded inner limits");
    let mut scratch = [0xa5; 4096];
    let base = scratch.as_ptr();
    let mut calls = 0;
    let (actual_call, observed) = if cfg!(feature = "seeded-staged-reject-all") {
        (false, Err(StagedDecryptError::Authentication))
    } else {
        (
            true,
            with_decrypted_frame(&parts, &key, &mut scratch, limits, |frame| {
                calls += 1;
                assert!(expected.is_ok(), "{ORACLE}");
                let count = expected.expect("modeled successful callback");
                compare_fields(&frame, &original, count, base);
                if data == HEALTHY_SEED {
                    // Literal expectations from the selected seed, not its builder.
                    let mut fields = frame.fields();
                    assert!(
                        fields.next().expect(ORACLE).bytes() == [0x12, 0x34, 0, 4],
                        "{ORACLE}"
                    );
                    assert!(
                        fields.next().expect(ORACLE).bytes()
                            == [0x02, 0x04, 0, 8, 0xde, 0xad, 0xbe, 0xef],
                        "{ORACLE}"
                    );
                    assert!(fields.next().is_none(), "{ORACLE}");
                }
                count
            }),
        )
    };
    assert!(
        actual_call,
        "independent staged AES target forwarding oracle"
    );
    assert_eq!(calls, usize::from(expected.is_ok()), "{ORACLE}");
    assert_eq!(observed, expected, "{ORACLE}");
    assert!(scratch.iter().all(|byte| *byte == 0), "{ORACLE}");

    // Exact selected input, actual consumer observations, result and full wipe
    // have all been checked above. No other input or fixed background call can
    // emit this marker. At most one line is emitted per process.
    if data == HEALTHY_SEED && !WITNESS_PRINTED.swap(true, Ordering::Relaxed) {
        assert_eq!(observed, Ok(2), "{ORACLE}");
        eprintln!("STAGED_AES_HEALTHY_WITNESS fields=2 payload=deadbeef callbacks=1 scratch=zero");
    }
}

fuzz_target!(|data: &[u8]| {
    check(data);
});

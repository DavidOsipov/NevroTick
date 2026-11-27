// SPDX-License-Identifier: GPL-2.0-only
//! Loopback peer signing with the independently encoded two-leaf fixture layout.

use ed25519_dalek::{Signature, Signer, SigningKey};
use sha2::{Digest, Sha512};
use std::error::Error;

/// Independently encoded OpenSSL-signed frame used as the fixture template.
const LEFT_RESPONSE: &[u8; 476] = include_bytes!("tree-left-response.bin");

/// Certificate domain separator with its RFC terminator.
const DELEGATION_CONTEXT: &[u8] = b"Roughtime v1 delegation signature\0";
/// Response domain separator with its RFC terminator.
const RESPONSE_CONTEXT: &[u8] = b"Roughtime v1 response signature\0";

/// Sign the exact observed request and a fixture-selected date.
///
/// # Errors
/// Refuses a midpoint outside the representable fixture delegation span.
pub(super) fn sign_reply(
    trust_anchor: &SigningKey,
    response_key: &SigningKey,
    request: &[u8],
    nonce: [u8; 32],
    midpoint: u64,
    radius: u32,
) -> Result<[u8; 476], Box<dyn Error>> {
    let mut response: [u8; 476] = *LEFT_RESPONSE;
    response[132..164].copy_from_slice(&nonce);
    response[256..260].copy_from_slice(&radius.to_le_bytes());
    response[260..268].copy_from_slice(&midpoint.to_le_bytes());
    let leaf: [u8; 32] = digest(&[&[0], request]);
    let merkle_root: [u8; 32] = digest(&[&[1], &leaf, &response[168..200]]);
    response[276..308].copy_from_slice(&merkle_root);

    let earliest: u64 = midpoint
        .checked_sub(100)
        .ok_or("fixture delegation starts too early")?;
    let latest: u64 = midpoint
        .checked_add(100)
        .ok_or("fixture delegation ends too late")?;
    response[424..456].copy_from_slice(response_key.verifying_key().as_bytes());
    response[456..464].copy_from_slice(&earliest.to_le_bytes());
    response[464..472].copy_from_slice(&latest.to_le_bytes());
    let delegation: Signature =
        trust_anchor.sign(&[DELEGATION_CONTEXT, &response[388..472]].concat());
    response[324..388].copy_from_slice(&delegation.to_bytes());
    let signature: Signature = response_key.sign(&[RESPONSE_CONTEXT, &response[200..308]].concat());
    response[68..132].copy_from_slice(&signature.to_bytes());
    Ok(response)
}

/// Independently hash the unmodified transmitted or received bytes.
pub(super) fn digest(parts: &[&[u8]]) -> [u8; 32] {
    let mut state: Sha512 = Sha512::new();
    for part in parts {
        state.update(part);
    }
    let bytes: [u8; 64] = state.finalize().into();
    let mut truncated: [u8; 32] = [0; 32];
    truncated.copy_from_slice(&bytes[..32]);
    truncated
}

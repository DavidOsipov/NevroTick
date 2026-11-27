// SPDX-License-Identifier: GPL-2.0-only
//! Encode the fixed RFC 10049 v1 UDP request profile.

use super::{RootKey, truncated_hash};

/// The 12-byte frame header followed by the 1,024-byte padded message.
pub(super) const REQUEST_BYTES: usize = 12 + 1_024;

/// Magic, body length, tag count, value starts, ordered tags, and `VER = [1]`.
/// The four explicit starts are 4, 36, 68, and 72 bytes into the values section.
const PREFIX: &[u8; 56] = b"ROUGHTIM\
    \0\x04\0\0\
    \x05\0\0\0\
    \x04\0\0\0\x24\0\0\0\x44\0\0\0\x48\0\0\0\
    VER\0SRV\0NONCTYPEZZZZ\
    \x01\0\0\0";

/// Offset of the 32-byte `SRV` value in the complete frame.
const SERVER_START: usize = PREFIX.len();
/// Offset of the 32-byte `NONC` value in the complete frame.
const NONCE_START: usize = SERVER_START + 32;
/// Offset of the zero `TYPE` word before the zero `ZZZZ` padding.
const TYPE_START: usize = NONCE_START + 32;

/// Build one padded request for the selected root and the caller's exact nonce.
///
/// The caller owns entropy, nonce custody and the sending policy. This pure
/// encoder performs one bounded hash and returns the complete frame by value.
#[must_use]
pub(super) fn encode(root: &RootKey, nonce: &[u8; 32]) -> [u8; REQUEST_BYTES] {
    let commitment: [u8; 32] = truncated_hash(&[&[0xff], root.0.as_bytes()]);
    let mut request: [u8; REQUEST_BYTES] = [0; REQUEST_BYTES];
    request[..SERVER_START].copy_from_slice(PREFIX);
    request[SERVER_START..NONCE_START].copy_from_slice(&commitment);
    request[NONCE_START..TYPE_START].copy_from_slice(nonce);
    request
}

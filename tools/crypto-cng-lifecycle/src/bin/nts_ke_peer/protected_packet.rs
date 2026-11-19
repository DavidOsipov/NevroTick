// SPDX-License-Identifier: GPL-2.0-only
//! Exact first-packet profile of the standalone loopback NTS peer.
//!
//! The caller owns one exchange and supplies the same-session directional
//! exporters. This codec supplies no socket, deadline, entropy or cookie store.

use aes_siv::{KeyInit, siv::Aes128Siv};
use nevrotick_s115_aes_siv_fixture::staged::with_decrypted_frame;
use ntp_wire::{
    ExtensionFields, FrameLimits, NtsAuthenticatorParts, NtsPlaintextLimits, RawDatagram,
    RawExtension, RawNtsPlaintext, frame_no_legacy_mac_datagram,
};
use zeroize::Zeroizing;

/// Client header, UID32, first cookie, seven placeholders and final authenticator.
pub(super) const REQUEST_BYTES: usize = 252;
/// Server header and UID32 followed by its sole final authenticator.
const REPLY_BYTES: usize = 140;
/// Exact NTS associated-data extent for the server header and UID field.
const REPLY_PREFIX_BYTES: usize = 84;
/// Public fixture cookie, distinct from the cookie sent in the KE response.
const REPLACEMENT_COOKIE: [u8; 12] = [
    0x83, 0x73, 0x63, 0x53, 0x43, 0x33, 0x23, 0x13, 0x91, 0x92, 0x93, 0x94,
];

/// Redacted packet outcomes, containing no rejected input or secret bytes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum PacketFailure {
    /// The complete UDP payload has a different extent from the selected request.
    Length,
    /// The request lacks a v4 client header with a nonzero wire transmit token.
    Header,
    /// Complete framing or the selected ordered extension shape was refused.
    Shape,
    /// The bounded request did not authenticate with the original C2S exporter.
    Authentication,
    /// The maintained provider refused to seal the reply.
    Seal,
}

/// Authenticate the fixed C2S request and seal one correlated S2C reply.
///
/// On success the reply echoes the authenticated transmit token as origin and
/// echoes the sole UID. The supplied response nonce is used unchanged. It is
/// chosen and owned by the calling one-shot fixture. The fixture returns one
/// synthetic replacement cookie, with no subsequent-request capability.
///
/// # Errors
/// Refuse the exact packet-length, header, ordered-field or authentication
/// mismatch before sealing. Preserve provider refusal without returning bytes.
pub(super) fn admit_and_seal(
    request: &[u8],
    c2s_key: &[u8; 32],
    s2c_key: &[u8; 32],
    response_nonce: &[u8; 16],
) -> Result<Zeroizing<[u8; REPLY_BYTES]>, PacketFailure> {
    if request.len() != REQUEST_BYTES {
        return Err(PacketFailure::Length);
    }
    let limits: FrameLimits =
        FrameLimits::new(REQUEST_BYTES, 10).map_err(|_| PacketFailure::Shape)?;
    let frame: RawDatagram<'_> =
        frame_no_legacy_mac_datagram(request, limits).map_err(|_| PacketFailure::Shape)?;
    let wire_transmit: u64 = frame.header().transmit_timestamp().bits();
    if frame.header().version() != 4 || frame.header().mode() != 3 || wire_transmit == 0 {
        return Err(PacketFailure::Header);
    }
    let uid: &[u8; 32] = request_uid(&frame)?;
    let parts: NtsAuthenticatorParts<'_> = frame
        .nts_authenticator_parts()
        .map_err(|_| PacketFailure::Shape)?;
    if parts.nonce().len() != 16 || parts.ciphertext().len() != 16 {
        return Err(PacketFailure::Shape);
    }
    let plaintext_limits: NtsPlaintextLimits =
        NtsPlaintextLimits::new(0, 0).map_err(|_| PacketFailure::Shape)?;
    let mut scratch: [u8; 4096] = [0; 4096];
    with_decrypted_frame(
        &parts,
        c2s_key,
        &mut scratch,
        plaintext_limits,
        |_: RawNtsPlaintext<'_>| (),
    )
    .map_err(|_| PacketFailure::Authentication)?;
    seal_reply(wire_transmit, uid, s2c_key, response_nonce)
}

/// Validate the complete ordered field sequence and borrow the exact UID body.
///
/// # Errors
/// Refuse absent, misplaced or unequal fields, including a trailing extension.
fn request_uid<'a>(frame: &RawDatagram<'a>) -> Result<&'a [u8; 32], PacketFailure> {
    let mut fields: ExtensionFields<'a> = frame.extensions();
    let uid_field: RawExtension<'a> = fields
        .next()
        .filter(|field: &RawExtension<'a>| field.field_type() == 0x0104)
        .ok_or(PacketFailure::Shape)?;
    let uid: &[u8; 32] = uid_field
        .body()
        .try_into()
        .map_err(|_| PacketFailure::Shape)?;
    let initial_cookie: &[u8] = super::RESPONSE.get(16..28).ok_or(PacketFailure::Shape)?;
    if !fields.next().is_some_and(|field: RawExtension<'a>| {
        field.field_type() == 0x0204 && field.body() == initial_cookie
    }) {
        return Err(PacketFailure::Shape);
    }
    for _ in 0..7 {
        if !fields.next().is_some_and(|field: RawExtension<'a>| {
            field.field_type() == 0x0304 && field.body() == [0; 12]
        }) {
            return Err(PacketFailure::Shape);
        }
    }
    if !fields.next().is_some_and(|field: RawExtension<'a>| {
        field.field_type() == 0x0404 && field.bytes().len() == 40
    }) || fields.next().is_some()
    {
        return Err(PacketFailure::Shape);
    }
    Ok(uid)
}

/// Seal an authenticated request's correlated reply in its own wiping output.
///
/// The application timestamps are public opaque fixture coordinates. They
/// establish no UTC or clock-mapping claim. The body is encrypted in place.
///
/// # Errors
/// A provider failure drops the entire candidate and publishes no reply bytes.
fn seal_reply(
    wire_transmit: u64,
    uid: &[u8; 32],
    s2c_key: &[u8; 32],
    nonce: &[u8; 16],
) -> Result<Zeroizing<[u8; REPLY_BYTES]>, PacketFailure> {
    let mut reply: Zeroizing<[u8; REPLY_BYTES]> = Zeroizing::new([0; REPLY_BYTES]);
    let bytes: &mut [u8] = reply.as_mut();
    let (aad, authenticator): (&mut [u8], &mut [u8]) = bytes.split_at_mut(REPLY_PREFIX_BYTES);
    aad[..2].copy_from_slice(&[0x24, 2]);
    aad[16..24].copy_from_slice(&0x0123_4567_89ab_c000_u64.to_be_bytes());
    aad[24..32].copy_from_slice(&wire_transmit.to_be_bytes());
    aad[32..40].copy_from_slice(&0x0123_4567_89ab_d000_u64.to_be_bytes());
    aad[40..48].copy_from_slice(&0x0123_4567_89ab_e000_u64.to_be_bytes());
    aad[48..52].copy_from_slice(&[1, 4, 0, 36]);
    aad[52..84].copy_from_slice(uid);

    let (envelope, body): (&mut [u8], &mut [u8]) = authenticator.split_at_mut(40);
    envelope[..8].copy_from_slice(&[4, 4, 0, 56, 0, 16, 0, 32]);
    envelope[8..24].copy_from_slice(nonce);
    body[..4].copy_from_slice(&[2, 4, 0, 16]);
    body[4..16].copy_from_slice(&REPLACEMENT_COOKIE);
    let tag: aes_siv::Tag = Aes128Siv::new(s2c_key.into())
        .encrypt_inout_detached([&aad[..], nonce.as_slice()], body.into())
        .map_err(|_| PacketFailure::Seal)?;
    envelope[24..40].copy_from_slice(tag.as_slice());
    Ok(reply)
}

#[cfg(test)]
mod tests;

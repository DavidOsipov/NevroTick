// SPDX-License-Identifier: GPL-2.0-only
//! Exact protected packet profiles of the standalone loopback NTS peer.
//!
//! The caller owns the bounded exchanges and supplies the same-session directional
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
pub(super) const REPLY_BYTES: usize = 140;
/// Server header, UID32 and final authenticator with no plaintext fields.
pub(super) const DENY_BYTES: usize = 124;
/// Exact NTS associated-data extent for the server header and UID field.
const REPLY_PREFIX_BYTES: usize = 84;
/// Correlation-only NTSN contains only the NTP header and UID field.
pub(super) const NAK_BYTES: usize = REPLY_PREFIX_BYTES;
/// Public fixture cookie, distinct from the cookie sent in the KE response.
const REPLACEMENT_COOKIE: [u8; 12] = [
    0x83, 0x73, 0x63, 0x53, 0x43, 0x33, 0x23, 0x13, 0x91, 0x92, 0x93, 0x94,
];
/// Public terminal cookie, distinct from both earlier cookies in the exchange.
const SUCCESSOR_COOKIE: [u8; 12] = [
    0x12, 0x24, 0x36, 0x48, 0x5a, 0x6c, 0x7e, 0x90, 0xa2, 0xb4, 0xc6, 0xd8,
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
    /// An authenticated successor reused either initial correlation value.
    Replay,
    /// The maintained provider refused to seal the reply.
    Seal,
}

/// Authenticated request correlation needed to reject a reused UID or transmit token.
pub(super) struct Correlation {
    /// The request's original nonzero wire transmit token.
    wire_transmit: u64,
    /// The sole request UID authenticated together with that token.
    uid: [u8; 32],
}

/// Correlation authenticated with the issued cookie and checked against the first leg.
pub(super) struct SuccessorCorrelation {
    /// The current request's UID and transmit token.
    current: Correlation,
}

/// Authenticate the fixed C2S request and seal one correlated S2C reply.
///
/// On success the reply echoes the authenticated transmit token as origin and
/// echoes the sole UID. The supplied response nonce is used unchanged. It is
/// chosen and owned by the calling fixture. The reply contains a synthetic
/// replacement cookie. The original correlation is retained for the optional
/// successor exchange.
///
/// # Errors
/// Refuse the exact packet-length, header, ordered-field or authentication
/// mismatch before sealing. Preserve provider refusal without returning bytes.
pub(super) fn admit_and_seal(
    request: &[u8],
    c2s_key: &[u8; 32],
    s2c_key: &[u8; 32],
    response_nonce: &[u8; 16],
) -> Result<(Correlation, Zeroizing<[u8; REPLY_BYTES]>), PacketFailure> {
    let cookie: &[u8] = super::RESPONSE.get(16..28).ok_or(PacketFailure::Shape)?;
    let correlation: Correlation = authenticate(request, c2s_key, cookie)?;
    let reply: Zeroizing<[u8; REPLY_BYTES]> =
        seal_reply(&correlation, s2c_key, response_nonce, &REPLACEMENT_COOKIE)?;
    Ok((correlation, reply))
}

/// Admit the one successor using the issued cookie and fresh request correlation.
///
/// # Errors
/// Refuse invalid framing, header, cookie, authentication or reused correlation.
pub(super) fn admit_successor(
    request: &[u8],
    c2s_key: &[u8; 32],
    initial: &Correlation,
) -> Result<SuccessorCorrelation, PacketFailure> {
    let successor: Correlation = authenticate(request, c2s_key, &REPLACEMENT_COOKIE)?;
    if successor.wire_transmit == initial.wire_transmit || successor.uid == initial.uid {
        return Err(PacketFailure::Replay);
    }
    Ok(SuccessorCorrelation { current: successor })
}

/// Authenticate a complete fixed-profile request and own its correlation.
///
/// # Errors
/// Refuse a length, framing, header, cookie, field order or authentication mismatch.
fn authenticate(
    request: &[u8],
    c2s_key: &[u8; 32],
    expected_cookie: &[u8],
) -> Result<Correlation, PacketFailure> {
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
    let uid: &[u8; 32] = request_uid(&frame, expected_cookie)?;
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
    Ok(Correlation {
        wire_transmit,
        uid: *uid,
    })
}

/// Validate the complete ordered field sequence and borrow the exact UID body.
///
/// # Errors
/// Refuse absent, misplaced or unequal fields, including a trailing extension.
fn request_uid<'a>(
    frame: &RawDatagram<'a>,
    expected_cookie: &[u8],
) -> Result<&'a [u8; 32], PacketFailure> {
    let mut fields: ExtensionFields<'a> = frame.extensions();
    let uid_field: RawExtension<'a> = fields
        .next()
        .filter(|field: &RawExtension<'a>| field.field_type() == 0x0104)
        .ok_or(PacketFailure::Shape)?;
    let uid: &[u8; 32] = uid_field
        .body()
        .try_into()
        .map_err(|_| PacketFailure::Shape)?;
    if !fields.next().is_some_and(|field: RawExtension<'a>| {
        field.field_type() == 0x0204 && field.body() == expected_cookie
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
    correlation: &Correlation,
    s2c_key: &[u8; 32],
    nonce: &[u8; 16],
    cookie: &[u8; 12],
) -> Result<Zeroizing<[u8; REPLY_BYTES]>, PacketFailure> {
    let mut reply: Zeroizing<[u8; REPLY_BYTES]> = Zeroizing::new([0; REPLY_BYTES]);
    let bytes: &mut [u8] = reply.as_mut();
    let (aad, authenticator): (&mut [u8], &mut [u8]) = bytes.split_at_mut(REPLY_PREFIX_BYTES);
    aad[..2].copy_from_slice(&[0x24, 2]);
    aad[16..24].copy_from_slice(&0x0123_4567_89ab_c000_u64.to_be_bytes());
    aad[24..32].copy_from_slice(&correlation.wire_transmit.to_be_bytes());
    aad[32..40].copy_from_slice(&0x0123_4567_89ab_d000_u64.to_be_bytes());
    aad[40..48].copy_from_slice(&0x0123_4567_89ab_e000_u64.to_be_bytes());
    aad[48..52].copy_from_slice(&[1, 4, 0, 36]);
    aad[52..84].copy_from_slice(&correlation.uid);

    let (envelope, body): (&mut [u8], &mut [u8]) = authenticator.split_at_mut(40);
    envelope[..8].copy_from_slice(&[4, 4, 0, 56, 0, 16, 0, 32]);
    body[..4].copy_from_slice(&[2, 4, 0, 16]);
    body[4..16].copy_from_slice(cookie);
    seal_authenticator(aad, envelope, body, s2c_key, nonce)?;
    Ok(reply)
}

/// Encode the correlation-only NTSN allowed before the authenticated successor reply.
///
/// Its only field echoes the UID from the admitted second request. The same
/// request's transmit token supplies the origin. There are no timestamps used
/// for time, cookie, authenticator, or other extensions.
pub(super) fn correlated_nak(correlation: &SuccessorCorrelation) -> [u8; NAK_BYTES] {
    let current: &Correlation = &correlation.current;
    let mut reply: [u8; NAK_BYTES] = [0; NAK_BYTES];
    reply[0] = 0x24;
    reply[12..16].copy_from_slice(b"NTSN");
    reply[24..32].copy_from_slice(&current.wire_transmit.to_be_bytes());
    reply[48..52].copy_from_slice(&[1, 4, 0, 36]);
    reply[52..84].copy_from_slice(&current.uid);
    reply
}

/// Consume the authenticated successor correlation to seal a reply with a new cookie.
///
/// # Errors
/// Refuse a provider failure without returning reply bytes.
pub(super) fn seal_ordinary_successor(
    correlation: SuccessorCorrelation,
    s2c_key: &[u8; 32],
    nonce: &[u8; 16],
) -> Result<Zeroizing<[u8; REPLY_BYTES]>, PacketFailure> {
    let SuccessorCorrelation { current } = correlation;
    seal_reply(&current, s2c_key, nonce, &SUCCESSOR_COOKIE)
}

/// Consume the authenticated successor correlation to seal a DENY with no cookies.
///
/// # Errors
/// Refuse a provider failure without returning reply bytes.
pub(super) fn seal_deny(
    correlation: SuccessorCorrelation,
    s2c_key: &[u8; 32],
    nonce: &[u8; 16],
) -> Result<Zeroizing<[u8; DENY_BYTES]>, PacketFailure> {
    let SuccessorCorrelation { current } = correlation;
    let mut reply: Zeroizing<[u8; DENY_BYTES]> = Zeroizing::new([0; DENY_BYTES]);
    let (aad, envelope): (&mut [u8], &mut [u8]) = reply.as_mut().split_at_mut(REPLY_PREFIX_BYTES);
    aad[0] = 0x24;
    aad[12..16].copy_from_slice(b"DENY");
    aad[24..32].copy_from_slice(&current.wire_transmit.to_be_bytes());
    aad[48..52].copy_from_slice(&[1, 4, 0, 36]);
    aad[52..84].copy_from_slice(&current.uid);
    envelope[..8].copy_from_slice(&[4, 4, 0, 40, 0, 16, 0, 16]);
    seal_authenticator(aad, envelope, &mut [], s2c_key, nonce)?;
    Ok(reply)
}

/// Fill the fixed authenticator nonce and seal its body using the response direction.
///
/// # Errors
/// Propagate a sealing failure so the caller cannot send a partial candidate.
fn seal_authenticator(
    aad: &[u8],
    envelope: &mut [u8],
    body: &mut [u8],
    s2c_key: &[u8; 32],
    nonce: &[u8; 16],
) -> Result<(), PacketFailure> {
    envelope[8..24].copy_from_slice(nonce);
    let tag: aes_siv::Tag = Aes128Siv::new(s2c_key.into())
        .encrypt_inout_detached([aad, nonce.as_slice()], body.into())
        .map_err(|_| PacketFailure::Seal)?;
    envelope[24..40].copy_from_slice(tag.as_slice());
    Ok(())
}

#[cfg(test)]
mod tests;

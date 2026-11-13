// SPDX-License-Identifier: GPL-2.0-only
//! Bounded S2C packet admission into private, unpublished replacement material.
//!
//! The caller owns request/endpoint/currentness, packet-work charging and final
//! serialized publication. This component changes no ledger or freshness state.
//! Its only normal key supplier is the private same-connection association owner.

use nevrotick_s115_aes_siv_fixture::staged::StagedDecryptError;
use ntp_wire::{
    FrameError, FrameLimits, NtpTimestamp, NtsAuthenticatorError, NtsPlaintextLimits, RawHeader,
    RawNtsPlaintext, frame_no_legacy_mac_datagram, read_nts_authenticator_parts,
};
use zeroize::Zeroizing;

use super::request::DirectionalKeys;

#[cfg(test)]
pub(super) mod fixtures;
#[cfg(test)]
mod tests;

/// Maximum complete packet and private opening scratch, not a transport default.
const PACKET_LIMIT: usize = 4096;
/// Separate bounded outer and inner traversals, including ignored extensions.
const FIELD_LIMIT: usize = 64;
/// Maximum replacement population admitted by the frozen association prototype.
const COOKIE_LIMIT: usize = 8;

/// Closed failures retaining no key, packet, UID, cookie or plaintext material.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ReplyError {
    /// The request's admitted replacement allowance is outside one through eight.
    Limits,
    /// The complete outer datagram failed bounded framing.
    Frame(FrameError),
    /// The first authenticator's declared components failed bounded projection.
    Envelope(NtsAuthenticatorError),
    /// Selected algorithm 15 requires at least one nonce octet.
    Nonce,
    /// Version, mode, leap state or stratum cannot represent ordinary time data.
    Header,
    /// The origin differs from the saved wire token, not the separate real T1.
    Origin,
    /// Exactly one matching cleartext UID32 was not present before authentication.
    UniqueIdentifier,
    /// A reserved NTS field appeared in a forbidden cleartext/encrypted position.
    Placement,
    /// No encrypted cookie, excess count or aggregate arithmetic refusal.
    Cookies,
    /// Fallible bounded staging allocation failed before publication.
    Allocation,
    /// Actual S2C authentication or complete decrypted framing failed.
    Open(StagedDecryptError),
}

/// Raw authenticated coordinates, carrying no era, eligibility or clock authority.
#[derive(Clone, Copy, Eq, PartialEq)]
pub(super) struct ReplyFields {
    /// Complete header from the actual authenticated AAD, not reconstructed scalars.
    header: RawHeader,
}

impl ReplyFields {
    /// Retain every authenticated header bit without physical/era/eligibility authority.
    pub(super) const fn header(&self) -> RawHeader {
        self.header
    }

    /// Return the checked wire origin without substituting it for real local T1.
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "EP2 numerical-consumer integration remains held.")
    )]
    pub(super) const fn origin(&self) -> NtpTimestamp {
        self.header.origin_timestamp()
    }

    /// Return server receive bits without unfolding an era or admitting a sample.
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "EP2 numerical-consumer integration remains held.")
    )]
    pub(super) const fn receive(&self) -> NtpTimestamp {
        self.header.receive_timestamp()
    }

    /// Return server transmit bits without an uncertainty or freshness claim.
    #[cfg_attr(
        not(test),
        expect(dead_code, reason = "EP2 numerical-consumer integration remains held.")
    )]
    pub(super) const fn transmit(&self) -> NtpTimestamp {
        self.header.transmit_timestamp()
    }
}

/// Owned candidate returned only after complete packet/crypto/semantic admission.
/// Its private fields prevent unverified material from becoming staged output.
pub(super) struct StagedReply {
    /// Exact checked raw coordinates, independently current only at final commit.
    fields: ReplyFields,
    /// Exact opaque encrypted-cookie bodies, zeroized on every ordinary drop.
    cookies: Vec<Zeroizing<Vec<u8>>>,
    /// Checked total payload, including zero-length cookie bodies as zero bytes.
    cookie_bytes: usize,
}

impl StagedReply {
    /// Count encrypted cookie fields, not their nonempty/representable subset.
    pub(super) fn cookie_count(&self) -> usize {
        self.cookies.len()
    }

    /// Return the bounded total opaque payload; empty fields still consume slots.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "EP2 staged-population inspection has no admitted product consumer."
        )
    )]
    pub(super) const fn cookie_bytes(&self) -> usize {
        self.cookie_bytes
    }

    /// Move the complete candidate once into the caller's final currentness guard.
    pub(super) fn into_parts(self) -> (ReplyFields, Vec<Zeroizing<Vec<u8>>>) {
        (self.fields, self.cookies)
    }
}

/// Stage one bounded response under the actual association's implicit S2C key.
///
/// The caller must charge work before entry and preserve that charge on every
/// rejection. It also supplies the current request's UID, exact wire transmit
/// token and one-plus-placeholder allowance. Endpoint/time/currentness and actual
/// send completion remain mandatory caller guards, not packet assertions.
/// Complete outer framing includes any unauthenticated suffix, whose semantic
/// contents are then discarded. Only the first authenticator bounds the AAD.
/// Allocations and cookie copies occur after successful S2C authentication and
/// complete inner framing; refusal publishes no partially copied population.
/// # Errors
/// Preserve bounded framing/opening failures and closed semantic/allocation
/// categories. A zero encrypted-cookie count is distinct from an empty body.
pub(super) fn stage_reply(
    keys: &DirectionalKeys,
    raw: &[u8],
    expected_uid: &[u8; 32],
    wire_transmit: u64,
    maximum_cookies: usize,
) -> Result<StagedReply, ReplyError> {
    if !(1..=COOKIE_LIMIT).contains(&maximum_cookies) {
        return Err(ReplyError::Limits);
    }
    let limits = FrameLimits::new(PACKET_LIMIT, FIELD_LIMIT).map_err(ReplyError::Frame)?;
    let datagram = frame_no_legacy_mac_datagram(raw, limits).map_err(ReplyError::Frame)?;
    let header = datagram.header();
    if header.version() != 4
        || header.mode() != 4
        || header.leap_indicator() == 3
        || !(1..=15).contains(&header.stratum())
    {
        return Err(ReplyError::Header);
    }
    if header.origin_timestamp().bits() != wire_transmit {
        return Err(ReplyError::Origin);
    }
    let mut uid_seen = false;
    for field in datagram.extensions() {
        match field.field_type() {
            0x0404 => break,
            0x0104 => {
                if uid_seen || field.body() != expected_uid {
                    return Err(ReplyError::UniqueIdentifier);
                }
                uid_seen = true;
            }
            0x0204 | 0x0304 => return Err(ReplyError::Placement),
            _ => {}
        }
    }
    if !uid_seen {
        return Err(ReplyError::UniqueIdentifier);
    }
    let fields = ReplyFields { header: *header };
    let parts = read_nts_authenticator_parts(raw, limits).map_err(ReplyError::Envelope)?;
    if parts.nonce().is_empty() {
        return Err(ReplyError::Nonce);
    }
    let plaintext_limits = NtsPlaintextLimits::new(PACKET_LIMIT, FIELD_LIMIT)
        .map_err(|error| ReplyError::Open(StagedDecryptError::Plaintext(error)))?;
    let mut scratch = [0_u8; PACKET_LIMIT];
    keys.open_response(&parts, &mut scratch, plaintext_limits, |frame| {
        stage_cookies(&frame, fields, maximum_cookies)
    })
    .map_err(ReplyError::Open)?
}

/// Admit the full authenticated field sequence before allocating exact copies.
/// Unknown non-NTS fields have no effect. Duplicate opaque cookie bodies remain
/// distinct fields; no unselected deduplication or payload normalization occurs.
/// # Errors
/// Refuse reserved-field placement, zero/excess cookies, checked sum failure or
/// fallible allocation. Partial staging drops its zeroizing owned cookie bodies.
fn stage_cookies(
    frame: &RawNtsPlaintext<'_>,
    fields: ReplyFields,
    maximum_cookies: usize,
) -> Result<StagedReply, ReplyError> {
    let mut count = 0_usize;
    let mut cookie_bytes = 0_usize;
    for field in frame.fields() {
        match field.field_type() {
            0x0104 | 0x0304 | 0x0404 => return Err(ReplyError::Placement),
            0x0204 => {
                count += 1;
                if count > maximum_cookies {
                    return Err(ReplyError::Cookies);
                }
                cookie_bytes = cookie_bytes
                    .checked_add(field.body().len())
                    .ok_or(ReplyError::Cookies)?;
            }
            _ => {}
        }
    }
    if count == 0 {
        return Err(ReplyError::Cookies);
    }
    let mut cookies = Vec::new();
    cookies
        .try_reserve_exact(count)
        .map_err(|_| ReplyError::Allocation)?;
    for field in frame.fields().filter(|field| field.field_type() == 0x0204) {
        let mut copy = Zeroizing::new(Vec::new());
        copy.try_reserve_exact(field.body().len())
            .map_err(|_| ReplyError::Allocation)?;
        copy.extend_from_slice(field.body());
        cookies.push(copy);
    }
    Ok(StagedReply {
        fields,
        cookies,
        cookie_bytes,
    })
}

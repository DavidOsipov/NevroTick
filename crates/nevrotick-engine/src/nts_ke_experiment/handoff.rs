// SPDX-License-Identifier: GPL-2.0-only
//! One private untransmitted request, bound to its original publication owner.
//!
//! This component accepts owned laboratory material, not authenticated evidence.
//! Only the actual Windows producer supplies its same-connection candidate.
//! Packet construction never establishes permission to send or admit a reply.

use zeroize::Zeroizing;

use super::request::DirectionalKeys;
use super::{COOKIE_BYTES_LIMIT, COOKIE_LIMIT, Control, Failure, IoPhase, Summary};

#[cfg(test)]
mod tests;

/// Maximum private packet storage; independent of cookie and TLS buffers.
const PACKET_BYTES: usize = 4096;

/// Complete private packet awaiting the same invocation's final commit.
///
/// There is no packet accessor, alternate control argument, clone or debug view.
/// Dropping this value retires its packet without manufacturing completion.
pub(super) struct PreparedRequest<'a> {
    /// Original cancellation, expiry and consume-once authority.
    control: &'a Control,
    /// Complete packet, wiped on every ordinary return or unwind.
    packet: Zeroizing<[u8; PACKET_BYTES]>,
    /// Exact formatter-owned packet length, bounded by private storage.
    length: usize,
    /// Complete admitted initial cookie population; no reuse rule follows.
    cookie_count: usize,
    /// Checked aggregate payload, including all retained initial cookies.
    cookie_bytes: usize,
}

impl PreparedRequest<'_> {
    /// Commit under the original serialized control/time checkpoint, then wipe.
    ///
    /// # Errors
    /// Refuse cancellation, expiry or unavailable/consumed control. The private
    /// packet is destroyed on either outcome; no transmission occurs.
    pub(super) fn finish(self) -> Result<Summary, Failure> {
        let Self {
            control,
            packet,
            length,
            cookie_count,
            cookie_bytes,
        } = self;
        control.commit()?;
        drop(packet);
        Ok(Summary {
            cookies: cookie_count,
            cookie_bytes,
            packet_bytes: length,
            transmitted: false,
        })
    }
}

/// Acquire bounded entropy and seal once without committing historical success.
///
/// The actual producer supplies CNG entropy; component controls use public fixed
/// bytes. No caller label or entropy callback grants TLS or send authority.
/// Entropy and the entire packet retire on refusal; the association retains keys.
/// Cookie bytes remain
/// borrowed from the candidate and are never rewritten, copied or substituted.
/// Check the complete population before selecting its first exact body.
///
/// # Errors
/// Refuse original currentness before/after entropy and sealing, actual entropy
/// failure or the real formatter/sealer's rejection. Return no completed packet
/// or final summary on refusal. After successful entropy, refuse retirement
/// before entering the formatter/sealer. An entropy failure retains its error
/// even if that same call also retires the invocation.
pub(super) fn prepare<'a>(
    control: &'a Control,
    keys: &DirectionalKeys,
    cookies: &[Zeroizing<Vec<u8>>],
    fill: impl FnOnce(&mut [u8; 48]) -> Result<(), Failure>,
) -> Result<PreparedRequest<'a>, Failure> {
    control.remaining(IoPhase::Exchange)?;
    if cookies.is_empty() || cookies.len() > COOKIE_LIMIT {
        return Err(Failure::Cookies);
    }
    let cookie_bytes = cookies.iter().try_fold(0_usize, |total, cookie| {
        total.checked_add(cookie.len()).ok_or(Failure::Cookies)
    })?;
    if cookie_bytes > COOKIE_BYTES_LIMIT {
        return Err(Failure::Cookies);
    }
    let first_cookie = cookies.first().ok_or(Failure::Cookies)?;
    let mut entropy = Zeroizing::new([0_u8; 48]);
    fill(&mut entropy)?;
    control.remaining(IoPhase::Exchange)?;
    let mut packet = Zeroizing::new([0_u8; PACKET_BYTES]);
    let length = {
        let request = keys
            .seal_request(first_cookie, &entropy, packet.as_mut())
            .map_err(Failure::Packet)?;
        request.packet_bytes().len()
    };
    control.remaining(IoPhase::Exchange)?;
    drop(entropy);
    Ok(PreparedRequest {
        control,
        packet,
        length,
        cookie_count: cookies.len(),
        cookie_bytes,
    })
}

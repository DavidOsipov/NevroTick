//! One directional, untransmitted laboratory request through the existing sealer.
//!
//! Fixed key widths and consuming ownership establish no TLS, entropy, cookie,
//! association, currentness or send authority. The actual Windows producer and
//! handoff owner supply those separate checks. No network, allocation, RNG,
//! clock, logging or Windows operation occurs in this component.

use nevrotick_s115_aes_siv_fixture::sealing::{RequestSealError, prepare_algorithm15_request};
use ntp_wire::{
    FrameError, FrameLimits, HeaderError, NtsClientRequest, NtsRequestEncodeError,
    decode_fixed_header,
};
use zeroize::Zeroizing;

#[cfg(test)]
#[path = "request_tests.rs"]
mod tests;

/// Redacted component failures; no variant contains secret or packet material.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum RequestError {
    /// The fixed laboratory header did not enter the existing raw decoder.
    Header(HeaderError),
    /// The explicit laboratory byte/count bounds did not enter the wire owner.
    Limits(FrameError),
    /// The borrowed UID/nonce split did not retain its fixed typed dimensions.
    EntropyWidth,
    /// The actual formatter refused the unmodified cookie or destination shape.
    Format(NtsRequestEncodeError),
    /// The existing algorithm-15 sealer refused encryption or finalization.
    Seal(RequestSealError),
}

/// Exclusive directional key custody; deliberately no clone, debug or raw getter.
///
/// Construction accepts all fixed-width owned inputs. Only the actual producer
/// can establish exporter provenance; this component cannot authenticate it.
pub(super) struct DirectionalKeys {
    /// Only this direction may authenticate the outgoing laboratory request.
    c2s_key: Zeroizing<[u8; 32]>,
    /// Retained paired direction, wiped alongside C2S after sealing or refusal.
    s2c_key: Zeroizing<[u8; 32]>,
}

impl DirectionalKeys {
    /// Take both fixed-width owners without cloning or selecting a direction.
    pub(super) fn new(c2s_key: Zeroizing<[u8; 32]>, s2c_key: Zeroizing<[u8; 32]>) -> Self {
        Self { c2s_key, s2c_key }
    }

    /// Seal exactly one inert request using C2S, then wipe both owned keys.
    ///
    /// The fixed v4/client header has zero transmit time and must never be sent.
    /// Borrow the first 32 entropy bytes as UID and the final 16 as nonce; no
    /// randomness claim is made here. The caller selects one exact cookie;
    /// empty, unaligned or otherwise unrepresentable bodies refuse unchanged.
    /// Three fields and 4096 packet bytes bound the actual formatter, with zero
    /// placeholders and no encrypted extension fields. No cookie clone occurs
    /// before the formatter's necessary packet copy.
    ///
    /// The returned immutable view borrows only caller storage. Both keys are
    /// wiped on every ordinary return, including refusal. Provider/OS copies,
    /// abort cleanup and the caller's entropy/storage lifetime remain separate.
    /// Preparation refusal preserves storage; later sealer refusal may leave
    /// its prepared prefix, without publishing completed bytes.
    ///
    /// # Errors
    /// Preserve fixed-header/limit construction, defensive entropy conversion,
    /// actual formatter and actual sealer failures as closed redacted variants.
    pub(super) fn seal_request<'storage>(
        self,
        cookie: &[u8],
        entropy: &[u8; 48],
        storage: &'storage mut [u8],
    ) -> Result<NtsClientRequest<'storage>, RequestError> {
        let Self { c2s_key, s2c_key } = self;
        let mut raw_header = [0_u8; 48];
        raw_header[0] = 0x23;
        let header = decode_fixed_header(&raw_header).map_err(RequestError::Header)?;
        let limits = FrameLimits::new(4096, 3).map_err(RequestError::Limits)?;
        // The input type fixes 48 bytes, so the constant split is in bounds.
        let (uid, nonce_bytes) = entropy.split_at(32);
        let nonce: &[u8; 16] = nonce_bytes
            .try_into()
            .map_err(|_| RequestError::EntropyWidth)?;
        let result = prepare_algorithm15_request(&header, uid, cookie, 0, storage, limits)
            .map_err(RequestError::Format)
            .and_then(|prepared| prepared.seal(&c2s_key, nonce).map_err(RequestError::Seal));
        drop((c2s_key, s2c_key));
        result
    }
}

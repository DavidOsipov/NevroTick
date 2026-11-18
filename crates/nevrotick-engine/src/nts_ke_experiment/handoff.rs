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
    /// Actual association-bound mode; only the real owner can attach managed custody.
    origin: PreviewOrigin<'a>,
}

/// A managed packet cannot take the diagnostic final-commit path.
enum PreviewOrigin<'a> {
    /// Raw sealed storage carries no association or finalization permission.
    Unbound,
    /// The actual diagnostic association attached its same-Control owner once.
    Diagnostic,
    /// Original same-owner configuration/control/endpoint and checked population.
    Managed(super::association::PreviewOwner<'a>),
}

impl PreparedRequest<'_> {
    /// Commit under the original serialized control/time checkpoint, then wipe.
    ///
    /// # Errors
    /// Refuse unbound/managed packets, cancellation, expiry or unavailable/consumed
    /// control. The private
    /// packet is destroyed on either outcome; no transmission occurs.
    pub(super) fn finish(self) -> Result<Summary, Failure> {
        let Self {
            control,
            packet,
            length,
            cookie_count,
            cookie_bytes,
            origin,
        }: Self = self;
        if !matches!(origin, PreviewOrigin::Diagnostic) {
            return Err(Failure::Control);
        }
        control.commit()?;
        drop(packet);
        Ok(Summary {
            cookies: cookie_count,
            cookie_bytes,
            packet_bytes: length,
            transmitted: false,
        })
    }

    /// Consume this actual packet through its published association's sealed binding.
    /// # Errors
    /// Preserve foreign owner, source replacement, restriction, stop or expiry refusal.
    pub(super) fn finish_managed(self) -> Result<ManagedPreviewHistory, Failure> {
        managed::finish(self)
    }
}

impl<'a> PreparedRequest<'a> {
    /// Bind the request to the actual publishing owner before it can leave the ledger.
    /// # Errors
    /// Refuse foreign packet/control or a second origin attachment without completion.
    pub(super) fn bind_owner(
        mut self,
        owner: super::association::PreviewOwner<'a>,
    ) -> Result<Self, Failure> {
        if !matches!(&self.origin, PreviewOrigin::Unbound) || !owner.matches_control(self.control) {
            return Err(Failure::Control);
        }
        self.origin = if owner.is_managed() {
            PreviewOrigin::Managed(owner)
        } else {
            PreviewOrigin::Diagnostic
        };
        Ok(self)
    }
}

/// Keep actual managed completion construction below the ordinary handoff boundary.
mod managed {
    use super::{Failure, PreparedRequest, PreviewOrigin, Summary};
    use crate::SourceConfigurationSnapshot;

    /// Historical never-send completion minted only by the real packet commit.
    #[must_use = "Retain or inspect the historical Preview result; dropping it grants no authority"]
    pub(in super::super) struct ManagedPreviewHistory {
        /// Original immutable nonsensitive packet/cookie counts.
        summary: Summary,
        /// Actual installed source definition checked by the same final commit.
        configuration: SourceConfigurationSnapshot,
    }

    impl ManagedPreviewHistory {
        /// Borrow the original historical counts, with no new effect or current grant.
        pub(in super::super) const fn summary(&self) -> &Summary {
            &self.summary
        }

        /// Borrow the original issued revision without reading replacement state.
        pub(in super::super) const fn configuration(&self) -> &SourceConfigurationSnapshot {
            &self.configuration
        }
    }

    /// Consume the actual packet and association token at one final current checkpoint.
    /// # Errors
    /// Preserve every owner/currentness refusal without a completion or refunded work.
    pub(super) fn finish(prepared: PreparedRequest<'_>) -> Result<ManagedPreviewHistory, Failure> {
        let PreparedRequest {
            control,
            packet,
            length,
            // The handoff owns the selected cookie; the token retains the genuine
            // complete pre-operation population, as on the diagnostic Preview path.
            cookie_count: _,
            cookie_bytes: _,
            origin,
        }: PreparedRequest<'_> = prepared;
        let PreviewOrigin::Managed(owner): PreviewOrigin<'_> = origin else {
            return Err(Failure::Control);
        };
        let (configuration, population): (
            SourceConfigurationSnapshot,
            super::super::association::CheckedPopulation,
        ) = owner.commit(control)?;
        let (cookie_count, cookie_bytes): (usize, usize) = population.counts();
        drop(packet);
        Ok(ManagedPreviewHistory {
            summary: Summary {
                cookies: cookie_count,
                cookie_bytes,
                packet_bytes: length,
                transmitted: false,
            },
            configuration,
        })
    }
}

pub(super) use managed::ManagedPreviewHistory;

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
    let cookie_bytes: usize =
        cookies
            .iter()
            .try_fold(0_usize, |total: usize, cookie: &Zeroizing<Vec<u8>>| {
                total.checked_add(cookie.len()).ok_or(Failure::Cookies)
            })?;
    if cookie_bytes > COOKIE_BYTES_LIMIT {
        return Err(Failure::Cookies);
    }
    let first_cookie: &Zeroizing<Vec<u8>> = cookies.first().ok_or(Failure::Cookies)?;
    let mut entropy: Zeroizing<[u8; 48]> = Zeroizing::new([0_u8; 48]);
    fill(&mut entropy)?;
    control.remaining(IoPhase::Exchange)?;
    let mut packet: Zeroizing<[u8; PACKET_BYTES]> = Zeroizing::new([0_u8; PACKET_BYTES]);
    let length: usize = {
        let request: ntp_wire::NtsClientRequest<'_> = keys
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
        origin: PreviewOrigin::Unbound,
    })
}

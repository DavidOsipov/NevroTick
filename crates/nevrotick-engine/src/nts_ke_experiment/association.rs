// SPDX-License-Identifier: GPL-2.0-only
//! Exclusive Engine custody of one actual, ephemeral KE completion.
//!
//! Only the Windows transport supplies normal completion material. Component
//! tests use public bytes and establish no TLS/provider success. Publication is
//! atomic; no public constructor, secret accessor, UDP sender or Core edge exists.

use std::time::Instant;
use zeroize::Zeroizing;

use super::handoff::PreparedRequest;
use super::process::ProviderScope;
use super::profile::StrictProfile;
use super::request::DirectionalKeys;
use super::{COOKIE_BYTES_LIMIT, COOKIE_LIMIT, Config, Control, Failure, IoPhase};

pub(super) mod ledger;

/// Complete same-invocation transfer, with no clone or partially public fields.
pub(super) struct AssociationCandidate<'a> {
    /// Original checked identity and numeric loopback mapping.
    config: &'a Config,
    /// Actual issuer's still-live cancellation and original deadline owner.
    control: &'a Control,
    /// Keep actual global provider shutdown outside all dependent ownership.
    provider: &'a ProviderScope<'a>,
    /// Admitted prospective UDP endpoint; EP1 never opens a UDP socket.
    udp: std::net::SocketAddr,
    /// Original KE publication/ephemeral association cap, never renewed.
    expiry: Instant,
    /// Actual accepted TLS profile, preserved without an asserted-auth flag.
    _profile: StrictProfile,
    /// Both owned exporters from that same verified connection.
    keys: DirectionalKeys,
    /// Exact opaque bodies moved once out of the complete response.
    cookies: Vec<Zeroizing<Vec<u8>>>,
}

impl<'a> AssociationCandidate<'a> {
    /// Take the complete private producer transfer without cloning secret bytes.
    /// Actual TLS/exporter provenance belongs to the sole Windows caller.
    pub(super) fn new(
        config: &'a Config,
        control: &'a Control,
        provider: &'a ProviderScope<'a>,
        udp: std::net::SocketAddr,
        profile: StrictProfile,
        keys: DirectionalKeys,
        cookies: Vec<Zeroizing<Vec<u8>>>,
    ) -> Self {
        Self {
            config,
            control,
            provider,
            udp,
            expiry: control.expiry,
            _profile: profile,
            keys,
            cookies,
        }
    }
}

/// Checked population of one actual association at a receiving boundary.
/// Only that association can mint this opaque immutable snapshot.
#[derive(Clone, Copy)]
pub(super) struct CheckedPopulation {
    /// Checked entry count, including each admitted empty opaque cookie.
    cookies: usize,
    /// Checked aggregate payload without retaining another secret copy.
    cookie_bytes: usize,
}

impl CheckedPopulation {
    /// Project the captured checked counts without granting pool availability.
    pub(super) const fn counts(self) -> (usize, usize) {
        (self.cookies, self.cookie_bytes)
    }
}

/// One published Engine association and its one never-send preview allowance.
pub(super) struct AssociationOwner<'a> {
    /// Complete immutable custody from the genuine consuming transfer.
    material: AssociationCandidate<'a>,
    /// Charged before entropy; rejection/drop never replenishes this allowance.
    preview_spent: bool,
    /// One same-context request owner and its nonrenewable aggregate budgets.
    ledger: ledger::LedgerState<'a>,
}

impl<'a> AssociationOwner<'a> {
    /// Publish the entire same-owner candidate under a fresh serialized guard.
    /// # Errors
    /// Refuse foreign issuer/provider, endpoint, payload bounds, duplicate KE,
    /// cancellation or original expiry. No partially live owner is published.
    pub(super) fn receive(
        config: &'a Config,
        control: &'a Control,
        provider: &'a ProviderScope<'a>,
        candidate: AssociationCandidate<'a>,
    ) -> Result<Self, Failure> {
        if !std::ptr::eq(config, candidate.config)
            || !std::ptr::eq(control, candidate.control)
            || !std::ptr::eq(provider, candidate.provider)
            || candidate.expiry != control.expiry
        {
            return Err(Failure::Control);
        }
        if candidate.udp.ip() != config.peer.ip() || candidate.udp.port() == 0 {
            return Err(Failure::Endpoint);
        }
        if candidate.cookies.is_empty() || candidate.cookies.len() > COOKIE_LIMIT {
            return Err(Failure::Cookies);
        }
        let bytes = candidate.cookies.iter().try_fold(0_usize, |total, body| {
            total.checked_add(body.len()).ok_or(Failure::Cookies)
        })?;
        if bytes > COOKIE_BYTES_LIMIT {
            return Err(Failure::Cookies);
        }
        let ledger = ledger::LedgerState::new()?;
        control.publish_ke()?;
        Ok(Self {
            material: candidate,
            preview_spent: false,
            ledger,
        })
    }

    /// Capture this same actual association's checked pre-operation population.
    /// Identity alone is checked here; ordinary operation guards retain currentness.
    /// # Errors
    /// Refuse foreign Control or inconsistent counts without altering the pool.
    pub(super) fn receiving_population(
        &self,
        control: &Control,
    ) -> Result<CheckedPopulation, Failure> {
        if !std::ptr::eq(control, self.material.control) {
            return Err(Failure::Control);
        }
        let (cookies, cookie_bytes) = self.cookie_population()?;
        Ok(CheckedPopulation {
            cookies,
            cookie_bytes,
        })
    }

    /// Return only the bounded current cookie population's nonsensitive counts.
    /// # Errors
    /// Refuse an inconsistent count or aggregate rather than emit false metadata.
    pub(super) fn cookie_population(&self) -> Result<(usize, usize), Failure> {
        if self.material.cookies.len() > COOKIE_LIMIT {
            return Err(Failure::Cookies);
        }
        let bytes = self
            .material
            .cookies
            .iter()
            .try_fold(0_usize, |total, body| {
                total.checked_add(body.len()).ok_or(Failure::Cookies)
            })?;
        if bytes > COOKIE_BYTES_LIMIT {
            return Err(Failure::Cookies);
        }
        Ok((self.material.cookies.len(), bytes))
    }

    /// Borrow C2S for the existing private preview while retaining paired S2C.
    /// # Errors
    /// Refuse stale/cancelled association or spent allowance before entropy.
    /// Charge once before entering entropy/sealing; no refusal resets credit.
    pub(super) fn prepare_preview(
        &mut self,
        fill: impl FnOnce(&mut [u8; 48]) -> Result<(), Failure>,
    ) -> Result<PreparedRequest<'_>, Failure> {
        self.material.control.remaining(IoPhase::Exchange)?;
        if self.preview_spent {
            return Err(Failure::PreviewSpent);
        }
        self.preview_spent = true;
        self.ledger_preview(fill)
    }
}

#[cfg(test)]
mod tests;

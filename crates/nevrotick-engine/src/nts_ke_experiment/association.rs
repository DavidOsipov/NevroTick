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
use super::profile::{CertificateTimeProvenance, FullAuthenticationBracket, StrictProfile};
use super::request::DirectionalKeys;
use super::{COOKIE_BYTES_LIMIT, COOKIE_LIMIT, Config, Control, Failure};

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
    profile: StrictProfile,
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
            profile,
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
    /// Exact source-issued negotiated UDP lease, present only for managed protection.
    protected_endpoint: Option<crate::source_policy::ProtectedEndpoint>,
    /// Charged before entropy; rejection/drop never replenishes this allowance.
    preview_spent: bool,
    /// One same-context request owner and its nonrenewable aggregate budgets.
    ledger: ledger::LedgerState<'a>,
}

/// Original Preview custody projected only by an actual published association.
/// Fields remain private so a caller cannot supply an unrelated context or endpoint.
pub(super) struct PreviewOwner<'a> {
    /// Original immutable connection and closed operation selection.
    config: &'a Config,
    /// Actual same-invocation cancellation and phase owner.
    control: &'a Control,
    /// Actual admitted endpoint received with this same association.
    udp: std::net::SocketAddr,
    /// Genuine checked population captured before this Preview spends its cookie.
    population: CheckedPopulation,
}

impl PreviewOwner<'_> {
    /// Check exact packet Control before attaching this actual owner's origin.
    pub(super) fn matches_control(&self, control: &Control) -> bool {
        std::ptr::eq(self.control, control)
    }

    /// Distinguish the sealed source-owned selection from both diagnostic modes.
    pub(super) fn is_managed(&self) -> bool {
        matches!(&self.config.mode, super::ExchangeMode::ManagedPreview(_))
    }

    /// Commit only the original packet/control and return its historical definition.
    /// Clone the immutable observation before acquiring combined current custody.
    /// # Errors
    /// Refuse diagnostic/foreign packets, replacement, restrictions, stop or expiry.
    pub(super) fn commit(
        self,
        packet_control: &Control,
    ) -> Result<(crate::SourceConfigurationSnapshot, CheckedPopulation), Failure> {
        if !std::ptr::eq(self.control, packet_control) {
            return Err(Failure::Control);
        }
        let super::ExchangeMode::ManagedPreview(context) = &self.config.mode else {
            return Err(Failure::Control);
        };
        let original: crate::SourceConfigurationSnapshot = context.snapshot().clone();
        self.config
            .guard(self.control, Some(self.udp))?
            .commit(Instant::now())?;
        Ok((original, self.population))
    }
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
        if candidate.udp.ip() != config.peer.ip() || !crate::lab_peer_is_valid(candidate.udp) {
            return Err(Failure::Endpoint);
        }
        if candidate.cookies.is_empty() || candidate.cookies.len() > COOKIE_LIMIT {
            return Err(Failure::Cookies);
        }
        let bytes: usize = candidate.cookies.iter().try_fold(
            0_usize,
            |total: usize, body: &Zeroizing<Vec<u8>>| {
                total.checked_add(body.len()).ok_or(Failure::Cookies)
            },
        )?;
        if bytes > COOKIE_BYTES_LIMIT {
            return Err(Failure::Cookies);
        }
        let ledger: ledger::LedgerState<'a> = ledger::LedgerState::new()?;
        let protected_endpoint: Option<crate::source_policy::ProtectedEndpoint> =
            config.publish_ke(control, candidate.udp)?;
        Ok(Self {
            material: candidate,
            protected_endpoint,
            preview_spent: false,
            ledger,
        })
    }

    /// Read the original profile's historical certificate time provenance.
    pub(super) const fn certificate_time_provenance(&self) -> CertificateTimeProvenance {
        self.material.profile.certificate_time_provenance()
    }

    /// Preserve the original KE profile's process-local authentication bracket.
    pub(super) const fn full_authentication_bracket(&self) -> Option<FullAuthenticationBracket> {
        self.material.profile.full_authentication_bracket()
    }

    /// Check the closed operation selection and borrow its original negotiated lease.
    /// # Errors
    /// Refuse Preview's no-send selection and any detached mode or destination.
    pub(super) fn protected_endpoint(
        &self,
    ) -> Result<Option<&crate::source_policy::ProtectedEndpoint>, Failure> {
        match (&self.material.config.mode, &self.protected_endpoint) {
            (super::ExchangeMode::ManagedProtected(_), Some(endpoint)) => Ok(Some(endpoint)),
            (super::ExchangeMode::Preview | super::ExchangeMode::ProtectedDatagram, None) => {
                Ok(None)
            }
            _ => Err(Failure::Control),
        }
    }

    /// Mint a same-owner final-Preview binding without copying keys or current authority.
    /// # Errors
    /// Refuse protected authority or inconsistent original population.
    fn preview_owner(&self) -> Result<PreviewOwner<'a>, Failure> {
        self.check_preview_mode()?;
        Ok(PreviewOwner {
            config: self.material.config,
            control: self.material.control,
            udp: self.material.udp,
            population: self.receiving_population(self.material.control)?,
        })
    }

    /// Managed protected custody is valid only for the ordinary protected path.
    /// # Errors
    /// Refuse conversion into a weaker Preview completion.
    fn check_preview_mode(&self) -> Result<(), Failure> {
        if matches!(
            &self.material.config.mode,
            super::ExchangeMode::ManagedProtected(_)
        ) {
            return Err(Failure::Control);
        }
        Ok(())
    }

    /// Reach the same private token issuer only for duplicate-binding controls.
    /// # Errors
    /// Preserve the real same-owner population refusal without a replacement token.
    #[cfg(test)]
    pub(super) fn preview_owner_for_test(&self) -> Result<PreviewOwner<'a>, Failure> {
        self.preview_owner()
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
    ) -> Result<PreparedRequest<'a>, Failure> {
        self.check_preview_mode()?;
        {
            let guard: super::ConfigurationGuard<'_> = self
                .material
                .config
                .guard(self.material.control, Some(self.material.udp))?;
            guard.control.published(Instant::now())?;
            if self.preview_spent {
                return Err(Failure::PreviewSpent);
            }
            self.preview_spent = true;
        }
        self.ledger_preview(fill)
    }
}

#[cfg(test)]
mod tests;

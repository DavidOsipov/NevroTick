// SPDX-License-Identifier: GPL-2.0-only
//! Consuming retirement and one exact predecessor's private request history.
//!
//! A retirement fact proves only the named original-context ledger transition.
//! Its receiving projection rechecks actual owners and current authority. Neither
//! it nor the owned history authorizes a KE attempt, physical-completion claim,
//! protected send, cookie return, observation or deadline renewal.

use std::time::Instant;

use zeroize::Zeroizing;

use super::{
    ATTEMPTS, ExhaustedResponse, LedgerError, RecoveryIntent, check_receive_times, refused,
    was_used,
};
use crate::nts_ke_experiment::association::AssociationOwner;
use crate::nts_ke_experiment::process::ProviderScope;
use crate::nts_ke_experiment::{COOKIE_BYTES_LIMIT, Config, Control, ExchangeMode, Failure};

/// One predecessor's exact possibly-sent cookies and already issued UIDs.
/// Shared actual source/provider borrows remain; no old Control or key is kept.
pub(in crate::nts_ke_experiment) struct PastRequests<'scope> {
    /// The receiving projection checked the actual immutable source owner.
    config: &'scope Config,
    /// History cannot be installed under another provider shutdown scope.
    provider: &'scope ProviderScope<'scope>,
    /// Exact moved cookie bodies, with ordinary zeroization and no clone.
    cookies: Vec<Zeroizing<Vec<u8>>>,
    /// Preserve the original fixed storage without copying the issued prefix.
    uids: Zeroizing<[[u8; 32]; ATTEMPTS]>,
    /// Only this checked prefix represents actual issued identifiers.
    uid_count: usize,
}

/// Check raw history dimensions without creating archive or issuer authority.
/// Immutable actual ledger ownership is required separately at consuming retirement.
/// # Errors
/// Refuse empty, inconsistent or over-capacity raw data; no checked archive leaves.
#[cfg(any(windows, test))]
fn check_history_bounds(cookies: &[Zeroizing<Vec<u8>>], uid_count: usize) -> Result<(), Failure> {
    if cookies.is_empty() || uid_count > ATTEMPTS || cookies.len() > uid_count {
        return Err(refused(LedgerError::Capacity));
    }
    let bytes = cookies
        .iter()
        .try_fold(0_usize, |total, body| total.checked_add(body.len()))
        .ok_or(refused(LedgerError::Capacity))?;
    if bytes > COOKIE_BYTES_LIMIT {
        return Err(refused(LedgerError::Capacity));
    }
    Ok(())
}

impl PastRequests<'_> {
    /// Test exact opaque membership without releasing any cookie bytes.
    pub(super) fn cookie_was_used(&self, body: &[u8]) -> bool {
        was_used(&self.cookies, body)
    }

    /// Inspect only the checked issued prefix; no new UID becomes authorized.
    pub(super) fn uid_was_issued(&self, uid: &[u8; 32]) -> bool {
        self.uids.iter().take(self.uid_count).any(|old| old == uid)
    }
}

/// Move-only actual original-context exit; its local Control cannot escape.
#[cfg(any(windows, test))]
pub(in crate::nts_ke_experiment) struct RetiredContext<'context> {
    /// Original complete source/provider/Control binding, never numeric identity.
    config: &'context Config,
    /// Retain the original Control only until the receiving projection.
    control: &'context Control,
    /// The original global shutdown scope remains borrowed through retirement.
    provider: &'context ProviderScope<'context>,
    /// Actual owned coalesced intent, never accepted from a caller.
    intent: RecoveryIntent,
    /// Moved bounded archive; old keys, pool and active storage have retired.
    history: PastRequests<'context>,
}

#[cfg(any(windows, test))]
impl RetiredContext<'_> {
    /// Check actual custody and current Published authority before projection.
    /// The checked outer references bind history beyond the local Control's life;
    /// the returned unchanged time is inert and grants no scheduling permission.
    /// # Errors
    /// Refuse foreign source/provider/Control, cancellation or original expiry.
    pub(in crate::nts_ke_experiment) fn receive<'scope>(
        self,
        config: &'scope Config,
        control: &Control,
        provider: &'scope ProviderScope<'scope>,
    ) -> Result<(Instant, PastRequests<'scope>), Failure> {
        if !std::ptr::eq(config, self.config)
            || !std::ptr::eq(control, self.control)
            || !std::ptr::eq(provider, self.provider)
        {
            return Err(Failure::Control);
        }
        control.with_published(|_| Ok(()))?;
        let PastRequests {
            cookies,
            uids,
            uid_count,
            ..
        } = self.history;
        let history = PastRequests {
            config,
            provider,
            cookies,
            uids,
            uid_count,
        };
        Ok((self.intent.not_before, history))
    }
}

#[cfg(any(windows, test))]
impl<'a> AssociationOwner<'a> {
    /// Consume one genuine final count-refusal exhaustion and retire its secrets.
    /// Actual backend/runtime retirement is a prerequisite of the sole normal
    /// composition caller, not a fact inferred from this logical state transfer.
    /// # Errors
    /// Refuse foreign/stale owners, another reason, prior receipt/predecessor,
    /// active custody, incomplete frame exhaustion or invalid receive chronology.
    pub(in crate::nts_ke_experiment) fn retire_for_recovery(
        mut self,
        config: &Config,
        control: &Control,
        provider: &ProviderScope<'_>,
    ) -> Result<RetiredContext<'a>, Failure> {
        if !std::ptr::eq(config, self.material.config)
            || !std::ptr::eq(control, self.material.control)
            || !std::ptr::eq(provider, self.material.provider)
            || config.mode != ExchangeMode::ProtectedDatagram
            || self.material.expiry != control.expiry
        {
            return Err(Failure::Control);
        }
        control.with_published(|now| {
            if self.ledger.previous.is_some()
                || self.ledger.active.is_some()
                || self.ledger.last.is_some()
                || self.ledger.frames != 0
            {
                return Err(refused(LedgerError::Phase));
            }
            // Owner-only creation fixes original expiry. The genuine final marker
            // and irreversible frame exhaustion force the first work intent.
            if self.ledger.recovery.is_none() {
                return Err(refused(LedgerError::Phase));
            }
            let Some(ExhaustedResponse::Cookies { sent_at, arrival }) =
                self.ledger.exhausted_response
            else {
                return Err(refused(LedgerError::Phase));
            };
            check_receive_times(sent_at, arrival, now)
        })?;
        let intent = self
            .ledger
            .recovery
            .take()
            .ok_or(refused(LedgerError::Phase))?;
        // Move only this consumed actual ledger's complete archives. Bounds alone
        // never mint a PastRequests value or attest absence of omitted requests.
        let cookies = self.ledger.used;
        let uids = self.ledger.uids;
        let uid_count = self.ledger.uid_count;
        check_history_bounds(&cookies, uid_count)?;
        let history = PastRequests {
            config: self.material.config,
            provider: self.material.provider,
            cookies,
            uids,
            uid_count,
        };
        let retired = RetiredContext {
            config: self.material.config,
            control: self.material.control,
            provider: self.material.provider,
            intent,
            history,
        };
        // The returned fact retains no old pool, directional key or active packet.
        drop(self.material);
        Ok(retired)
    }

    /// Consume a pristine actual replacement and its one checked predecessor.
    /// Before mode/entropy/send admission, remove known possibly-sent values.
    /// Failure consumes both owners and grants no restoration or later retry.
    /// # Errors
    /// Refuse foreign source/provider/Control, nonpristine or stale replacement,
    /// another predecessor or an empty remaining fresh-cookie population.
    pub(in crate::nts_ke_experiment) fn install_previous(
        mut self,
        config: &Config,
        control: &Control,
        provider: &ProviderScope<'_>,
        history: PastRequests<'a>,
    ) -> Result<Self, Failure> {
        if !std::ptr::eq(config, self.material.config)
            || !std::ptr::eq(control, self.material.control)
            || !std::ptr::eq(provider, self.material.provider)
            || !std::ptr::eq(config, history.config)
            || !std::ptr::eq(provider, history.provider)
            || self.material.expiry != control.expiry
        {
            return Err(Failure::Control);
        }
        control.with_published(|_| {
            // Every local slot, UID, send, receive, receipt and recovery state
            // requires a charged reservation. Return, failure and Drop never
            // replenish attempts; preview/predecessor have separate owners.
            // A new state writer or any refund/reset invalidates this reduction
            // and requires reviewing the unused-owner admission contract.
            if self.preview_spent
                || self.ledger.previous.is_some()
                || self.ledger.attempts != ATTEMPTS
            {
                return Err(refused(LedgerError::Phase));
            }
            self.material
                .cookies
                .retain(|body| !history.cookie_was_used(body));
            if self.material.cookies.is_empty() {
                return Err(refused(LedgerError::Cookies));
            }
            self.ledger.previous = Some(history);
            Ok(())
        })?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests;

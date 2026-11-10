// SPDX-License-Identifier: GPL-2.0-only
//! Private, allocation-free progress through one admitted laboratory address set.
//!
//! The parent configuration's mandatory Garde constructor owns the immutable
//! one-to-three distinct loopback peers and positive finite timeout limits.
//! This owner preserves their first-admitted order and the original deadline.
//! It performs no I/O or clock reads and supplies no source, DNS, path, mapping,
//! authentication, generation, statistical or clock authority.
//!
//! A claim exclusively borrows its cursor. Only consuming that exact claim with
//! its matching index releases the active position; dropping it does not. The
//! parent retains actual socket/report custody and calls `finish` only after
//! actual attempt completion. A copied tuple is diagnostic data, not a claim.

use std::{net::SocketAddr, time::Instant};

use super::AddressRoundConfig;
use crate::PilotError;

/// One finite ordered round; private state has no reset, replacement or clone.
pub(super) struct Progress<'config> {
    /// Already checked immutable input; no raw constructor or second rule.
    config: &'config AddressRoundConfig,
    /// Original whole-round cutoff; issuing/completing work never renews it.
    deadline: Instant,
    /// Next never-issued index, bounded by the admitted three-address capacity.
    next: u8,
    /// Issued position whose exact claim has not acknowledged completion.
    active: Option<u8>,
}

impl<'config> Progress<'config> {
    /// Borrow the parent's checked configuration and retain its original cutoff.
    /// An already reached cutoff is legal and permits no claim. This constructs
    /// only empty progress state; the parent owns deadline creation and current
    /// clock observations. No allocation, native work or permission is created.
    #[must_use]
    pub(super) const fn new(config: &'config AddressRoundConfig, deadline: Instant) -> Self {
        Self {
            config,
            deadline,
            next: 0,
            active: None,
        }
    }

    /// Exclusively claim the next original peer and its bounded attempt cutoff.
    /// `now` is the parent's current monotonic observation. Exhaustion and the
    /// inclusive whole-round cutoff return no claim without changing state.
    /// A claim consumes its position exactly once; dropping or mis-completing
    /// that claim cannot restore it.
    ///
    /// # Errors
    /// Active ownership or contradictory indices return `InternalState`;
    /// checked cutoff arithmetic returns `DeadlineOverflow`. Every refusal
    /// preserves the prior state and cannot look like ordinary exhaustion.
    pub(super) fn claim_next(
        &mut self,
        now: Instant,
    ) -> Result<Option<Claim<'_, 'config>>, PilotError> {
        if self.active.is_some() {
            return Err(PilotError::InternalState);
        }
        if now >= self.deadline || usize::from(self.next) == self.config.peers.len() {
            return Ok(None);
        }
        let peer = *self
            .config
            .peers
            .get(usize::from(self.next))
            .ok_or(PilotError::InternalState)?;
        let remaining = self
            .deadline
            .checked_duration_since(now)
            .ok_or(PilotError::DeadlineOverflow)?;
        let deadline = now
            .checked_add(self.config.request_timeout.min(remaining))
            .ok_or(PilotError::DeadlineOverflow)?;
        let next = self.next.checked_add(1).ok_or(PilotError::InternalState)?;
        let index = self.next;
        self.next = next;
        self.active = Some(index);
        Ok(Some(Claim {
            cursor: self,
            index,
            peer,
            deadline,
        }))
    }
}

/// Non-Clone, non-Copy exclusive claim tied to the cursor that issued it.
/// An abandoned claim leaves that cursor occupied; no destructor admits work.
pub(super) struct Claim<'cursor, 'config> {
    /// Actual issuing owner, enforced by the exclusive borrow.
    cursor: &'cursor mut Progress<'config>,
    /// Original admitted position, never an externally assigned request ID.
    index: u8,
    /// Exact checked endpoint at that original position.
    peer: SocketAddr,
    /// Earlier of per-attempt and original whole-round cutoffs.
    deadline: Instant,
}

impl Claim<'_, '_> {
    /// Copy the original position, endpoint and cutoff without releasing custody.
    #[must_use]
    pub(super) const fn parts(&self) -> (u8, SocketAddr, Instant) {
        (self.index, self.peer, self.deadline)
    }

    /// Consume the exact active claim after the parent has completed its work.
    /// This acknowledges cursor ownership only, not native teardown or success.
    ///
    /// # Errors
    /// A mismatched completion index or internal owner contradiction consumes
    /// this claim and leaves the cursor occupied, admitting no successor.
    pub(super) fn finish(self, index: u8) -> Result<(), PilotError> {
        if index != self.index || self.cursor.active != Some(self.index) {
            return Err(PilotError::InternalState);
        }
        self.cursor.active = None;
        Ok(())
    }
}

#[cfg(test)]
#[path = "../../tests/private/address_progress.rs"]
mod tests;

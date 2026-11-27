// SPDX-License-Identifier: GPL-2.0-only
//! Controlled mapping revisions for exercises of the actual joined receiving owner.

use std::num::NonZeroU64;
use std::ops::RangeInclusive;
use std::sync::{Arc, Mutex, MutexGuard, TryLockError};
use std::time::{Duration, Instant};

use super::super::datagram::ProtectedHistory;
use super::ReceivedOrdinaryReceipt;
use super::{ManagedProtectedHistory, ManagedReceivingContext, ProtectedReceiptAdmission};
use crate::{SoftwareClock, SourceConfigurationSnapshot};

/// The independent mapping and admission limits selected for a controlled exchange.
pub(in super::super) struct FixtureMapping {
    /// The actual issued definition selected by this supplier.
    source: SourceConfigurationSnapshot,
    /// Inclusive interval covering the outer software-capture bounds.
    validity: RangeInclusive<Instant>,
    /// Error supplied by the controlled mapping.
    uncertainty: Duration,
    /// Independent receiving limit for this fixture.
    uncertainty_limit: Duration,
    /// Limit on age from original packet acceptance in this fixture.
    age_limit: Duration,
}

impl FixtureMapping {
    /// Keep the fixture inputs distinct until the actual receiving decision.
    pub(in super::super) const fn new(
        source: SourceConfigurationSnapshot,
        validity: RangeInclusive<Instant>,
        uncertainty: Duration,
        uncertainty_limit: Duration,
        age_limit: Duration,
    ) -> Self {
        Self {
            source,
            validity,
            uncertainty,
            uncertainty_limit,
            age_limit,
        }
    }

    /// Evaluate controlled inputs against the original exchange, under its lease.
    fn evaluate(
        &self,
        original: &ManagedProtectedHistory,
        now: Instant,
    ) -> ProtectedReceiptAdmission {
        if !original.configuration().same_configuration(&self.source) {
            return ProtectedReceiptAdmission::ForeignMapping;
        }
        let history: &ProtectedHistory = original.history();
        let captures: [SoftwareClock; 3] = history.captures();
        if !covers_captures(&self.validity, &captures) {
            return ProtectedReceiptAdmission::MappingValidity;
        }
        if self.uncertainty > self.uncertainty_limit {
            return ProtectedReceiptAdmission::MappingUncertainty;
        }
        let acceptance: Instant = history.receipt().accepted_at();
        if now < acceptance
            || now
                .checked_duration_since(acceptance)
                .is_none_or(|age: Duration| age > self.age_limit)
        {
            return ProtectedReceiptAdmission::ReceiptAge;
        }
        ProtectedReceiptAdmission::FixtureHandoff
    }
}

/// Require the declared interval to contain both edges of every original bracket.
fn covers_captures(validity: &RangeInclusive<Instant>, captures: &[SoftwareClock; 3]) -> bool {
    captures.iter().all(|capture: &SoftwareClock| {
        validity.contains(&capture.monotonic()) && validity.contains(&capture.monotonic_after())
    })
}

/// Isolate each outer edge so an inner timestamp cannot stand in for the whole span.
/// # Errors
/// Refuse an unrepresentable test coordinate.
/// # Panics
/// Fail if a touching interval refuses or a truncated interval covers the span.
#[test]
fn controlled_validity_covers_both_outer_capture_edges() -> Result<(), ProtectedReceiptAdmission> {
    use std::time::UNIX_EPOCH;

    let start: Instant = Instant::now();
    let shift = |offset: u64| -> Result<Instant, ProtectedReceiptAdmission> {
        start
            .checked_add(Duration::from_millis(offset))
            .ok_or(ProtectedReceiptAdmission::MappingValidity)
    };
    let captures: [SoftwareClock; 3] = [
        SoftwareClock {
            mono: start,
            utc: UNIX_EPOCH,
            mono_after: shift(1)?,
        },
        SoftwareClock {
            mono: shift(2)?,
            utc: UNIX_EPOCH,
            mono_after: shift(3)?,
        },
        SoftwareClock {
            mono: shift(4)?,
            utc: UNIX_EPOCH,
            mono_after: shift(5)?,
        },
    ];
    assert!(covers_captures(&(start..=shift(5)?), &captures));
    assert!(!covers_captures(&(shift(1)?..=shift(5)?), &captures));
    assert!(!covers_captures(&(start..=shift(4)?), &captures));
    Ok(())
}

/// One current revision at an independently controlled issuer.
struct FixtureMappingState {
    /// No revision remains after checked exhaustion.
    revision: Option<NonZeroU64>,
    /// Withdrawal removes evidence even when the revision can advance.
    mapping: Option<FixtureMapping>,
}

/// The actual issuing and withdrawing owner for joined source controls.
pub(in super::super) struct FixtureMappingOwner {
    /// Carries current validity through each receiving transfer.
    state: Mutex<FixtureMappingState>,
}

impl FixtureMappingOwner {
    /// Construct the selected controlled authority before binding any acquisition.
    pub(in super::super) const fn new(revision: NonZeroU64, mapping: FixtureMapping) -> Self {
        Self {
            state: Mutex::new(FixtureMappingState {
                revision: Some(revision),
                mapping: Some(mapping),
            }),
        }
    }

    /// Bind this actual currently issued revision, without allowing later rebinding.
    pub(super) fn lease(self: &Arc<Self>) -> Option<FixtureMappingLease> {
        let state: MutexGuard<'_, FixtureMappingState> = self.state.try_lock().ok()?;
        state.mapping.as_ref()?;
        Some(FixtureMappingLease {
            owner: Arc::clone(self),
            revision: state.revision?,
        })
    }

    /// Replace or withdraw evidence. Exhaustion irrevocably disables issuance.
    /// # Errors
    /// Refuse unavailable custody or checked revision exhaustion.
    pub(in super::super) fn replace(
        &self,
        mapping: Option<FixtureMapping>,
    ) -> Result<(), ProtectedReceiptAdmission> {
        let mut state: MutexGuard<'_, FixtureMappingState> = self
            .state
            .try_lock()
            .map_err(|_| ProtectedReceiptAdmission::MappingWithdrawn)?;
        state.revision = state
            .revision
            .and_then(|revision: NonZeroU64| revision.checked_add(1));
        state.mapping = None;
        if state.revision.is_none() {
            return Err(ProtectedReceiptAdmission::MappingWithdrawn);
        }
        state.mapping = mapping;
        Ok(())
    }
}

/// The original input, bound before acquisition to one issuer and revision.
pub(super) struct FixtureMappingLease {
    /// Retain the exact current issuer through its synchronous receiving decision.
    owner: Arc<FixtureMappingOwner>,
    /// The original revision; fresh issuance permits a genuinely new receipt only.
    revision: NonZeroU64,
}

impl FixtureMappingLease {
    /// Consume the exact history under this issuer's gate and the caller's live gates.
    pub(super) fn receive(
        &self,
        receiving: &ManagedReceivingContext,
        history: Arc<ManagedProtectedHistory>,
    ) -> ReceivedOrdinaryReceipt {
        let Ok(state): Result<
            MutexGuard<'_, FixtureMappingState>,
            TryLockError<MutexGuard<'_, FixtureMappingState>>,
        > = self.owner.state.try_lock() else {
            return ReceivedOrdinaryReceipt {
                history,
                admission: ProtectedReceiptAdmission::MappingWithdrawn,
            };
        };
        let admission: ProtectedReceiptAdmission = if state.revision == Some(self.revision) {
            match &state.mapping {
                Some(mapping) => match receiving.receiving_time(&history) {
                    Ok(now) => mapping.evaluate(&history, now),
                    Err(refusal) => refusal,
                },
                None => ProtectedReceiptAdmission::MappingWithdrawn,
            }
        } else {
            ProtectedReceiptAdmission::MappingWithdrawn
        };
        ReceivedOrdinaryReceipt { history, admission }
    }
}

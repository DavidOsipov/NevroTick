// SPDX-License-Identifier: GPL-2.0-only
//! Consume original, normally joined ordinary receipts before historical projection.

use std::sync::Arc;
use std::time::Instant;

use super::datagram::{ManagedProtectedHistory, PendingJoinedOrdinary};
use super::{Control, ScopeGuard, StopScope};
use crate::SourcePolicyError;
use crate::source_policy::{ConfiguredKePreview, KePreviewPolicyGuard, ProtectedReceiptPolicy};

#[cfg(test)]
mod fixture;
#[cfg(test)]
pub(super) use fixture::{FixtureMapping, FixtureMappingOwner};

/// The receiving decision for an original, joined ordinary protected receipt.
///
/// This disposition is separate from authenticated packet and cookie history.
#[non_exhaustive]
#[must_use = "retain the receiving decision beside the original receipt"]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProtectedReceiptAdmission {
    /// Historical facts reached a diagnostic or abnormal route outside the managed join.
    NotEvaluated,
    /// The actual acquisition scope stopped or lost its control gate.
    ControlUnavailable,
    /// The receipt does not carry this invocation's original authority.
    ForeignReceipt,
    /// The original acceptance, commit and final read fail the lifetime bound.
    Deadline,
    /// The live source or endpoint policy refused receiving.
    SourcePolicy(SourcePolicyError),
    /// No qualified read-side mapping was available.
    MappingUnavailable,
    /// The controlled supplier withdrew the original revision before receipt transfer.
    #[cfg(test)]
    MappingWithdrawn,
    /// The controlled supplier issued evidence for a different source definition.
    #[cfg(test)]
    ForeignMapping,
    /// The original capture span exceeds the controlled validity interval.
    #[cfg(test)]
    MappingValidity,
    /// The controlled error exceeds the independent allowance.
    #[cfg(test)]
    MappingUncertainty,
    /// The original acceptance is too old under the controlled lifetime policy.
    #[cfg(test)]
    ReceiptAge,
    /// The controlled fixture received this original receipt exactly once.
    #[cfg(test)]
    FixtureHandoff,
}

/// One original ordinary history moved through the joined receiving decision.
#[must_use = "retain the joined receipt together with its receiving decision"]
pub(super) struct ReceivedOrdinaryReceipt {
    /// Original immutable facts, with neither a live policy owner nor a socket.
    history: Arc<ManagedProtectedHistory>,
    /// Disposition evaluated exactly once during transfer at joined take.
    admission: ProtectedReceiptAdmission,
}

impl ReceivedOrdinaryReceipt {
    /// Consume the completed receiving decision for historical report projection.
    pub(super) fn into_parts(self) -> (Arc<ManagedProtectedHistory>, ProtectedReceiptAdmission) {
        (self.history, self.admission)
    }

    /// Inspect original facts in the joined component assertions.
    #[cfg(test)]
    pub(super) fn history(&self) -> &ManagedProtectedHistory {
        &self.history
    }

    /// Consume a genuine receipt when the collector has no original authority.
    pub(super) fn without_context(receipt: PendingJoinedOrdinary) -> Self {
        Self {
            history: receipt.into_history(),
            admission: ProtectedReceiptAdmission::ControlUnavailable,
        }
    }
}

/// Live guard context retained only by the collector of the original prepared job.
pub(super) struct ManagedReceivingContext {
    /// Exact definition as issued before the job entered the worker.
    definition: ConfiguredKePreview,
    /// Original association identity to which the committed facts must belong.
    association: Arc<()>,
    /// Actual stop authority shared with the current observer and worker.
    scope: Arc<StopScope>,
    /// Full-job exclusive cutoff, already clamped to the owner's original cutoff.
    deadline: Instant,
    /// Controlled issuer bound before acquisition. This input has no native route.
    #[cfg(test)]
    fixture: Option<fixture::FixtureMappingLease>,
    /// Independent monotonic observation consumed only at the final guarded point.
    #[cfg(test)]
    observed_at: Option<Instant>,
}

/// Keep both gate owners through the decision. Drop Stop before policy releases.
struct ReceivingGuards<'a> {
    /// Actual current stop latch.
    scope: ScopeGuard<'a>,
    /// Actual source and endpoint policy custody.
    policy: KePreviewPolicyGuard<'a>,
}

impl ManagedReceivingContext {
    /// Retain original receiving authority while the checked job is still owned.
    pub(super) fn from_prepared(definition: &ConfiguredKePreview, control: &Control) -> Self {
        Self {
            definition: definition.receiving_definition(),
            association: Arc::clone(&control.first_receipt_identity),
            scope: Arc::clone(&control.stop_scope),
            deadline: control.expiry,
            #[cfg(test)]
            fixture: None,
            #[cfg(test)]
            observed_at: None,
        }
    }

    /// Move the receipt once under the original scope and source policy gates.
    /// A refusal neither changes the original facts nor restores spent work.
    pub(super) fn receive(&self, receipt: PendingJoinedOrdinary) -> ReceivedOrdinaryReceipt {
        let guards: ReceivingGuards<'_> = match self.lock() {
            Ok(guards) => guards,
            Err(admission) => {
                return ReceivedOrdinaryReceipt {
                    history: receipt.into_history(),
                    admission,
                };
            }
        };
        let history: Arc<ManagedProtectedHistory> = receipt.into_history();
        if let Err(admission) = self.check_current(&guards, &history) {
            return ReceivedOrdinaryReceipt { history, admission };
        }
        #[cfg(test)]
        if let Some(fixture) = &self.fixture {
            return fixture.receive(self, history);
        }
        let admission: ProtectedReceiptAdmission = match self.receiving_time(&history) {
            Ok(_) => ProtectedReceiptAdmission::MappingUnavailable,
            Err(refusal) => refusal,
        };
        ReceivedOrdinaryReceipt { history, admission }
    }

    /// Bind only an actual current fixture revision to this prepared receipt owner.
    #[cfg(test)]
    pub(super) fn with_fixture_mapping(mut self, owner: &Arc<FixtureMappingOwner>) -> Self {
        self.fixture = owner.lease();
        self
    }

    /// Supply the final monotonic observation for one controlled joined invocation.
    #[cfg(test)]
    pub(super) const fn with_observed_at(mut self, observed_at: Instant) -> Self {
        self.observed_at = Some(observed_at);
        self
    }

    /// Acquire original custody in Stop then policy order.
    /// # Errors
    /// Preserve unavailable Stop or policy custody as a receiving refusal.
    fn lock(&self) -> Result<ReceivingGuards<'_>, ProtectedReceiptAdmission> {
        let scope: ScopeGuard<'_> = self
            .scope
            .lock()
            .map_err(|_| ProtectedReceiptAdmission::ControlUnavailable)?;
        let policy: KePreviewPolicyGuard<'_> = self
            .definition
            .lock()
            .map_err(ProtectedReceiptAdmission::SourcePolicy)?;
        Ok(ReceivingGuards { scope, policy })
    }

    /// Check the original facts and current ownership under both retained gates.
    /// # Errors
    /// Refuse closed control, foreign receipt, or changed source or endpoint policy.
    fn check_current(
        &self,
        guards: &ReceivingGuards<'_>,
        original: &ManagedProtectedHistory,
    ) -> Result<(), ProtectedReceiptAdmission> {
        use super::datagram::ProtectedHistory;

        if guards.scope.is_stopped() {
            return Err(ProtectedReceiptAdmission::ControlUnavailable);
        }
        let history: &ProtectedHistory = original.history();
        if !history.belongs_to_association(&self.association)
            || !original
                .configuration()
                .same_configuration(self.definition.snapshot())
        {
            return Err(ProtectedReceiptAdmission::ForeignReceipt);
        }
        let Some(policy): Option<&ProtectedReceiptPolicy> = original.receipt_policy() else {
            return Err(ProtectedReceiptAdmission::ForeignReceipt);
        };
        guards
            .policy
            .check_receipt(history.endpoints().1, policy)
            .map_err(ProtectedReceiptAdmission::SourcePolicy)
    }

    /// Observe the final time under current receiving custody.
    /// # Errors
    /// Refuse before the original commit or at/after the original cutoff.
    fn receiving_time(
        &self,
        original: &ManagedProtectedHistory,
    ) -> Result<Instant, ProtectedReceiptAdmission> {
        #[cfg(not(test))]
        let now: Instant = Instant::now();
        #[cfg(test)]
        let now: Instant = self.observed_at.unwrap_or_else(Instant::now);
        if now < original.history().committed_at() || now >= self.deadline {
            return Err(ProtectedReceiptAdmission::Deadline);
        }
        Ok(now)
    }
}

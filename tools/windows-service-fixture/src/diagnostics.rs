// SPDX-License-Identifier: GPL-2.0-only
//! Fixture-private portable diagnostic policy, not a native or durable logger.
//!
//! Only closed typed events reach encoding. Each route owns sixteen nine-byte
//! records and saturating full/unavailable counters. Admission has no I/O, lock,
//! callback, allocation, retry or flush. Optional output cannot consume required
//! capacity. Sequence exhaustion refuses new records instead of wrapping. The
//! v1 byte order is version, role, event, severity, cause, then four little-endian
//! sequence bytes. Codes belong only to this fixture contract.
//! The sequence belongs only to one retained invocation; it asserts no process/restart
//! identity, generation, UTC truth, acknowledged status or work completion.
//! Native providers, output draining/flush and authorized debug sessions remain
//! with their existing owners. No arbitrary text or error chain is admitted.

use windows_platform::service::LifecycleError;

use crate::{FixtureError, LabRole, NativeError, error::WorkerError};

/// Fixture-only memory allocation per route, independent of product disk budgets.
const CAPACITY: usize = 16;
/// Exact v1 record size; no Rust enum or struct layout is serialized.
const RECORD_BYTES: usize = 9;

/// Reviewed bounded cause; only the exhaustive typed projections construct it.
#[derive(Clone, Copy)]
pub(super) struct FailureCode(u8);

impl FailureCode {
    /// Preserve useful categories without traversing or formatting retained causes.
    pub(super) fn from_fixture(error: &FixtureError) -> Self {
        Self(match error {
            FixtureError::WindowsOnly => 1,
            FixtureError::AlreadyDispatched => 2,
            FixtureError::OwnerAlreadySet => 3,
            FixtureError::UnsupportedArguments => 4,
            FixtureError::ClockReversed => 5,
            FixtureError::ClockOverflow => 6,
            FixtureError::RunLimitElapsed => 7,
            FixtureError::Budget(_) => 8,
            FixtureError::Native(_) => 9,
            FixtureError::Lifecycle(error) => return Self::from_lifecycle(error),
            FixtureError::Worker(error) => return Self::from_worker(*error),
            FixtureError::TokenObservation(_) => 24,
            FixtureError::PrivilegeAdmission(_) => 25,
            FixtureError::ServiceFailed => 26,
            FixtureError::OutcomeUnavailable => 27,
            // Arbitrary nested public errors cannot grow diagnostic traversal.
            FixtureError::Cleanup(_) => 28,
        })
    }

    /// Map every lifecycle refusal distinctly; native error text/code stays typed.
    pub(super) const fn from_lifecycle(error: &LifecycleError<NativeError>) -> Self {
        Self(match error {
            LifecycleError::Report(_) => 10,
            LifecycleError::DeadlineElapsed => 11,
            LifecycleError::InvalidTransition => 12,
            LifecycleError::ReportingFailed => 13,
            LifecycleError::TimeReversed => 14,
            LifecycleError::DeadlineOverflow => 15,
            LifecycleError::CheckpointOverflow => 16,
            LifecycleError::OutstandingWork => 17,
            LifecycleError::WorkLimit => 18,
            LifecycleError::FailureOutcomeRequired => 19,
        })
    }

    /// Keep initialization, activation, health and completed cleanup distinct.
    pub(super) const fn from_worker(error: WorkerError) -> Self {
        Self(match error {
            WorkerError::Preparation => 20,
            WorkerError::Activation => 21,
            WorkerError::Running => 22,
            WorkerError::Cleanup => 23,
        })
    }
}

/// Decision-owner facts; no caller-supplied text, field name or raw payload.
#[derive(Clone, Copy)]
pub(super) enum Event {
    /// The invocation exists; this is not a successful startup status claim.
    StartObserved,
    /// Dormant readiness and the Running status were actually acknowledged.
    RunningAcknowledged,
    /// The owner fenced new admission before fallible Stop work.
    AdmissionClosed,
    /// The existing earliest failure, not an additional error chain.
    PrimaryFailure(FailureCode),
    /// A later actual completion acknowledged failed cleanup.
    CompletionFailure(FailureCode),
    /// A later `StopPending` report was refused or unacknowledged.
    StopReportFailure(FailureCode),
    /// A later terminal report was refused or unacknowledged.
    TerminalReportFailure(FailureCode),
    /// Custody resolved and terminal reporting attempted, including failed reports.
    /// This explicitly does not mean successful Stopped or a Clean result.
    Finished,
}

impl Event {
    /// Stable explicit (event, severity, cause) mapping: INFO=1, ERROR=2.
    const fn fields(self) -> (u8, u8, u8) {
        match self {
            Self::StartObserved => (1, 1, 0),
            Self::RunningAcknowledged => (2, 1, 0),
            Self::AdmissionClosed => (3, 1, 0),
            Self::PrimaryFailure(code) => (4, 2, code.0),
            Self::CompletionFailure(code) => (5, 2, code.0),
            Self::StopReportFailure(code) => (6, 2, code.0),
            Self::TerminalReportFailure(code) => (7, 2, code.0),
            Self::Finished => (8, 1, 0),
        }
    }
}

/// Sole encoding producer; private bytes prevent arbitrary sink payload admission.
#[derive(Clone, Copy)]
struct EncodedRecord([u8; RECORD_BYTES]);

/// Concrete finite sink. Unavailable admission is a bounded injected sink failure,
/// not a native channel observation. No callback can stall the service owner.
pub(super) struct MemorySink {
    /// Fixed encoded storage; only the initialized prefix is observable.
    records: [[u8; RECORD_BYTES]; CAPACITY],
    /// Always in 0..=CAPACITY, changed only after obtaining a free slot.
    len: usize,
    /// Fixed route availability; it changes no service policy or authority.
    available: bool,
    /// Saturating count of admissions refused by a full route.
    full: u32,
    /// Saturating count of admissions refused by an unavailable route.
    unavailable: u32,
}

impl MemorySink {
    /// Construct empty fixed storage with explicit route availability.
    pub(super) const fn new(available: bool) -> Self {
        Self {
            records: [[0; RECORD_BYTES]; CAPACITY],
            len: 0,
            available,
            full: 0,
            unavailable: 0,
        }
    }

    /// Attempt exactly once, retaining the first records and independent loss.
    /// A full/unavailable sink neither retries nor recursively emits a failure.
    fn try_emit(&mut self, record: EncodedRecord) {
        if !self.available {
            self.unavailable = self.unavailable.saturating_add(1);
            return;
        }
        if let Some(slot) = self.records.get_mut(self.len) {
            *slot = record.0;
            self.len += 1;
        } else {
            self.full = self.full.saturating_add(1);
        }
    }

    /// Observe only the admitted prefix, without draining or acknowledging work.
    #[cfg(test)]
    pub(super) fn records(&self) -> &[[u8; RECORD_BYTES]] {
        self.records.get(..self.len).unwrap_or(&[])
    }

    /// Read (full, unavailable) loss without emitting a recursive health event.
    #[cfg(test)]
    pub(super) const fn loss(&self) -> (u32, u32) {
        (self.full, self.unavailable)
    }

    /// Inject preoccupied storage; sentinel bytes are a test fixture, never emitted
    /// by a production encoder or a claimed observed service transition.
    #[cfg(test)]
    pub(super) const fn full() -> Self {
        Self {
            records: [[0; RECORD_BYTES]; CAPACITY],
            len: CAPACITY,
            available: true,
            full: 0,
            unavailable: 0,
        }
    }
}

/// Process composition of separate routes. None means optional output is off;
/// no user filter string, environment directive or extra authority is accepted.
pub(super) struct Sinks {
    /// Independent required route, including INFO events.
    required: MemorySink,
    /// Optional duplicate context cannot consume the required allocation.
    optional: Option<MemorySink>,
}

impl Sinks {
    /// Bind finite concrete routes without executing external work.
    pub(super) const fn new(required: MemorySink, optional: Option<MemorySink>) -> Self {
        Self { required, optional }
    }
}

/// One invocation's projection and routing owner, holding no service capability.
pub(super) struct Diagnostics {
    /// Fixed composition role, bound inside actual invocation construction.
    role: LabRole,
    /// Next local sequence; None permanently refuses sequence reuse.
    next_sequence: Option<u32>,
    /// Saturating loss after sequence exhaustion, separately visible from sinks.
    sequence_loss: u32,
    /// Two independent fixed resource allocations.
    sinks: Sinks,
}

impl Diagnostics {
    /// Begin at local sequence one; no cross-process identity is asserted.
    pub(super) const fn new(role: LabRole, sinks: Sinks) -> Self {
        Self {
            role,
            next_sequence: Some(1),
            sequence_loss: 0,
            sinks,
        }
    }

    /// Project before queuing, attempt required first, then the optional route.
    /// Return no service decision or success/failure signal to the caller.
    pub(super) fn emit(&mut self, event: Event) {
        let Some(sequence) = self.next_sequence else {
            self.sequence_loss = self.sequence_loss.saturating_add(1);
            return;
        };
        self.next_sequence = sequence.checked_add(1);
        let role = match self.role {
            LabRole::Engine => 1,
            LabRole::ClockBroker => 2,
        };
        let (event, severity, cause) = event.fields();
        let [a, b, c, d] = sequence.to_le_bytes();
        let record = EncodedRecord([1, role, event, severity, cause, a, b, c, d]);
        self.sinks.required.try_emit(record);
        if let Some(optional) = &mut self.sinks.optional {
            optional.try_emit(record);
        }
    }

    /// Observe the actual required output without modifying owner decisions.
    #[cfg(test)]
    pub(super) const fn required(&self) -> &MemorySink {
        &self.sinks.required
    }

    /// Observe optional output, preserving explicit Off as absence.
    #[cfg(test)]
    pub(super) const fn optional(&self) -> Option<&MemorySink> {
        self.sinks.optional.as_ref()
    }
}

#[cfg(test)]
#[path = "diagnostics_tests.rs"]
mod tests;

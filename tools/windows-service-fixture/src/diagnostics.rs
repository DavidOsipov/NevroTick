// SPDX-License-Identifier: GPL-2.0-only
//! Fixture-owned typed error projection into the sole shared diagnostic policy.
//!
//! This adapter never formats or traverses error chains. The shared router owns
//! catalogue semantics, validation, encoding, finite queues, consumption and health.
//! It has no service, worker, native completion or clock authority.

use std::{convert::Infallible, task::Poll, time::Instant};

use windows_platform::{
    diagnostics::{
        Component, DiagnosticObservation, DiagnosticSnapshot, ImportanceMask, LifecycleCause,
        MemoryConsumer, MemoryRoutes, OptionalProfile, PrimaryCause, Router, WorkerCause,
        WriterRecord,
    },
    service::LifecycleError,
};

pub(super) use windows_platform::diagnostics::ServiceEvent as Event;

use crate::{FixtureError, LabRole, NativeError, error::WorkerError, host::WorkerWake};

/// Project the complete original error once without inspecting nested payloads.
pub(super) const fn primary_cause(error: &FixtureError) -> PrimaryCause {
    match error {
        FixtureError::WindowsOnly => PrimaryCause::WindowsOnly,
        FixtureError::AlreadyDispatched => PrimaryCause::AlreadyDispatched,
        FixtureError::OwnerAlreadySet => PrimaryCause::OwnerAlreadySet,
        FixtureError::UnsupportedArguments => PrimaryCause::UnsupportedArguments,
        FixtureError::ClockReversed => PrimaryCause::ClockReversed,
        FixtureError::ClockOverflow => PrimaryCause::ClockOverflow,
        FixtureError::RunLimitElapsed => PrimaryCause::RunLimitElapsed,
        FixtureError::Budget(_) => PrimaryCause::Budget,
        FixtureError::Native(_) => PrimaryCause::Native,
        FixtureError::Lifecycle(error) => match error {
            LifecycleError::Report(_) => PrimaryCause::LifecycleReport,
            LifecycleError::DeadlineElapsed => PrimaryCause::LifecycleDeadlineElapsed,
            LifecycleError::InvalidTransition => PrimaryCause::LifecycleInvalidTransition,
            LifecycleError::ReportingFailed => PrimaryCause::LifecycleReportingFailed,
            LifecycleError::TimeReversed => PrimaryCause::LifecycleTimeReversed,
            LifecycleError::DeadlineOverflow => PrimaryCause::LifecycleDeadlineOverflow,
            LifecycleError::CheckpointOverflow => PrimaryCause::LifecycleCheckpointOverflow,
            LifecycleError::OutstandingWork => PrimaryCause::LifecycleOutstandingWork,
            LifecycleError::WorkLimit => PrimaryCause::LifecycleWorkLimit,
            LifecycleError::FailureOutcomeRequired => PrimaryCause::LifecycleFailureOutcomeRequired,
        },
        FixtureError::Worker(error) => match error {
            WorkerError::Preparation => PrimaryCause::WorkerPreparation,
            WorkerError::Activation => PrimaryCause::WorkerActivation,
            WorkerError::Running => PrimaryCause::WorkerRunning,
            WorkerError::Cleanup => PrimaryCause::WorkerCleanup,
        },
        FixtureError::TokenObservation(_) => PrimaryCause::TokenObservation,
        FixtureError::PrivilegeAdmission(_) => PrimaryCause::PrivilegeAdmission,
        FixtureError::ServiceFailed => PrimaryCause::ServiceFailed,
        FixtureError::OutcomeUnavailable => PrimaryCause::OutcomeUnavailable,
        FixtureError::Cleanup(_) => PrimaryCause::NestedCleanupNotTraversed,
    }
}

/// Preserve each lifecycle refusal while retaining native payloads at their owner.
pub(super) const fn lifecycle_cause(error: &LifecycleError<NativeError>) -> LifecycleCause {
    match error {
        LifecycleError::Report(_) => LifecycleCause::Report,
        LifecycleError::DeadlineElapsed => LifecycleCause::DeadlineElapsed,
        LifecycleError::InvalidTransition => LifecycleCause::InvalidTransition,
        LifecycleError::ReportingFailed => LifecycleCause::ReportingFailed,
        LifecycleError::TimeReversed => LifecycleCause::TimeReversed,
        LifecycleError::DeadlineOverflow => LifecycleCause::DeadlineOverflow,
        LifecycleError::CheckpointOverflow => LifecycleCause::CheckpointOverflow,
        LifecycleError::OutstandingWork => LifecycleCause::OutstandingWork,
        LifecycleError::WorkLimit => LifecycleCause::WorkLimit,
        LifecycleError::FailureOutcomeRequired => LifecycleCause::FailureOutcomeRequired,
    }
}

/// Keep preparation, activation, running and cleanup causes distinct.
pub(super) const fn worker_cause(error: WorkerError) -> WorkerCause {
    match error {
        WorkerError::Preparation => WorkerCause::Preparation,
        WorkerError::Activation => WorkerCause::Activation,
        WorkerError::Running => WorkerCause::Running,
        WorkerError::Cleanup => WorkerCause::Cleanup,
    }
}

/// Finite executable composition; optional absence selects the shared Off profile.
/// Consumers carry no callbacks, paths or authority over the service owner.
pub(super) struct Sinks {
    /// Independent fixed routes, always present even when optional output is Off.
    routes: MemoryRoutes,
    /// Optional duplicate eligibility, owned by the shared router.
    optional: OptionalProfile,
}

impl Sinks {
    /// Bind both finite consumers without starting work or accepting raw bytes.
    pub(super) fn new(required: MemoryConsumer, optional: Option<MemoryConsumer>) -> Self {
        let (optional, profile): (MemoryConsumer, OptionalProfile) = match optional {
            Some(consumer) => (consumer, OptionalProfile::Info),
            None => (
                MemoryConsumer::new(windows_platform::diagnostics::ConsumerState::Available),
                OptionalProfile::Off,
            ),
        };
        Self {
            routes: MemoryRoutes::new(required, optional),
            optional: profile,
        }
    }
}

/// One role-bound invocation's shared routing owner; no semantic table lives here.
pub(super) struct Diagnostics {
    /// Sole reusable producer/queue/consumer owner, retained through final receipt.
    router: Router,
}

impl Diagnostics {
    /// Bind a fixed role once, with no positive metadata issuer or ambient read.
    pub(super) fn new(role: LabRole, sinks: Sinks) -> Self {
        let component: Component = match role {
            LabRole::Engine => Component::Engine,
            LabRole::ClockBroker => Component::ClockBroker,
        };
        Self {
            router: Router::new(
                component,
                sinks.optional,
                ImportanceMask::all(),
                sinks.routes,
            ),
        }
    }

    /// Emit only an actual closed decision and its frozen lifecycle observation.
    pub(super) fn emit(&mut self, event: Event, observation: DiagnosticObservation) {
        self.router.emit(event, observation);
    }

    /// Emit through the same routes, transferring only an admitted live writer record.
    pub(super) fn emit_for_writer(
        &mut self,
        event: Event,
        observation: DiagnosticObservation,
    ) -> Option<WriterRecord> {
        self.router.emit_for_writer(event, observation)
    }

    /// Attempt at most one original per route, without reading time or service state.
    pub(super) fn pump_one(&mut self) {
        self.router.pump_one();
    }

    /// Observe actual consumption and health without advancing either owner.
    #[cfg(test)]
    pub(super) fn snapshot(&self) -> DiagnosticSnapshot {
        self.router.snapshot()
    }

    /// Seal production and consume at most sixteen remaining originals per route.
    pub(super) fn finish(&mut self) -> DiagnosticSnapshot {
        self.router.finish()
    }
}

/// Write allowance on the original service shutdown clock.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum CloseMode {
    /// Admit new writer effects only before this original absolute deadline.
    Drain(Instant),
    /// Discard work that never started and retain any outstanding operation.
    Retire,
}

/// Writer work and cleanup custody have resolved, with an independent loss fact.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum WriterClose {
    /// Local writer completion without observed loss. This gives no history receipt.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "the portable writer is exercised by the fixture tests before native writer composition"
        )
    )]
    Drained,
    /// Local refusal or loss with cleanup custody actually resolved.
    Loss,
}

/// The remaining observation needed while the same writer is still owned.
#[derive(Clone, Copy)]
pub(super) enum ClosePending {
    /// Work or teardown remains outstanding. Its owner will publish and wake.
    Wake,
    /// The writer has published its final marker and will send no further wake.
    /// Only physical completion can now discharge its custody.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "the writer join tail is exercised by fixture tests before native writer composition"
        )
    )]
    JoinTail,
}

/// One prompt, bounded adapter. Construction admits no work or live resource.
pub(super) trait EventWriter {
    /// Attempt one live record. True accepts ownership, without promising delivery.
    /// False is a final local refusal. Never wait or perform sink I/O here.
    fn offer(&mut self, record: WriterRecord, wake: &WorkerWake) -> bool;

    /// Pause unstarted backend work at the host's Stop fence, before any status call.
    fn fence(&mut self);

    /// Close admission and observe actual writer and cleanup completion.
    /// Check the actual start of a drain write against the absolute deadline.
    /// Retire prohibits new writes and supplies no completion by itself.
    /// Publish progress before waking this same owner.
    /// An observed final marker is sticky until physical completion.
    ///
    /// # Errors
    /// Return the pending observation while the original writer remains owned.
    fn poll_close(
        &mut self,
        mode: CloseMode,
        wake: &WorkerWake,
    ) -> Result<WriterClose, ClosePending>;
}

impl EventWriter for Infallible {
    /// The default fixture has no writer value and never reaches this operation.
    fn offer(&mut self, _record: WriterRecord, _wake: &WorkerWake) -> bool {
        match *self {}
    }

    /// The default fixture has no writer value to fence.
    fn fence(&mut self) {
        match *self {}
    }

    /// The default fixture has no writer value and never reaches this operation.
    fn poll_close(
        &mut self,
        _mode: CloseMode,
        _wake: &WorkerWake,
    ) -> Result<WriterClose, ClosePending> {
        match *self {}
    }
}

/// Progress of the same writer across every service-owner poll.
enum Phase {
    /// Live records may be offered. An unused adapter has no custody.
    Open {
        /// A reached offer can leave writer resources even after a refusal.
        attempted: bool,
    },
    /// No admission. The writer is retained until its completion is observed.
    Closing(ClosePending),
    /// Actual local close disposition is latched and never re-polled.
    Closed(WriterClose),
}

/// Own the exact selected adapter through expiry, wake and terminal retirement.
pub(super) struct Delivery<W> {
    /// Adapter supplied before any service operation.
    writer: W,
    /// Finite owner state, distinct from the status/timing decisions.
    phase: Phase,
    /// Sticky refusal, even when every accepted record drains normally.
    refused: bool,
}

impl<W: EventWriter> Delivery<W> {
    /// Take a dormant adapter without starting it.
    pub(super) const fn new(writer: W) -> Self {
        Self {
            writer,
            phase: Phase::Open { attempted: false },
            refused: false,
        }
    }

    /// Check the admission fence before projecting a writer record.
    pub(super) const fn is_open(&self) -> bool {
        matches!(self.phase, Phase::Open { .. })
    }

    /// A previously observed actual close needs no further adapter poll.
    pub(super) const fn is_closed(&self) -> bool {
        matches!(self.phase, Phase::Closed(_))
    }

    /// A final wake can precede physical completion. Keep observing the join tail.
    pub(super) const fn needs_join_tail(&self) -> bool {
        matches!(self.phase, Phase::Closing(ClosePending::JoinTail))
    }

    /// Transfer one original while open, retaining refusal through close.
    pub(super) fn offer(&mut self, record: WriterRecord, wake: &WorkerWake) {
        if let Phase::Open { attempted } = &mut self.phase {
            *attempted = true;
            self.refused |= !self.writer.offer(record, wake);
        }
    }

    /// Close admission before any subsequent time observation or terminal decision.
    pub(super) fn fence(&mut self) {
        if self.is_open() {
            self.phase = Phase::Closing(ClosePending::Wake);
            self.writer.fence();
        }
    }

    /// Retain the result of the original close; an expired allowance is never a join.
    pub(super) fn poll_close(&mut self, mode: CloseMode, wake: &WorkerWake) -> Poll<WriterClose> {
        self.fence();
        if let Phase::Closed(outcome) = self.phase {
            return Poll::Ready(outcome);
        }
        let progress: Result<WriterClose, ClosePending> = self.writer.poll_close(mode, wake);
        let outcome: WriterClose = match progress {
            Ok(outcome) => outcome,
            Err(pending) => {
                if !self.needs_join_tail() {
                    self.phase = Phase::Closing(pending);
                }
                return Poll::Pending;
            }
        };
        let outcome: WriterClose = if self.refused {
            WriterClose::Loss
        } else {
            outcome
        };
        self.phase = Phase::Closed(outcome);
        Poll::Ready(outcome)
    }

    /// Does destruction still risk discarding the adapter's actual custody?
    pub(super) const fn unresolved(&self) -> bool {
        matches!(
            self.phase,
            Phase::Open { attempted: true } | Phase::Closing(_)
        )
    }

    /// Expose the final local disposition to the same invocation's receipt.
    #[cfg(test)]
    pub(super) const fn outcome(&self) -> Option<WriterClose> {
        match self.phase {
            Phase::Closed(outcome) => Some(outcome),
            Phase::Open { .. } | Phase::Closing(_) => None,
        }
    }
}

#[cfg(test)]
#[path = "diagnostics_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "diagnostics_thread.rs"]
pub(super) mod threaded;

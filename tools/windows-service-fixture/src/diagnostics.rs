// SPDX-License-Identifier: GPL-2.0-only
//! Fixture-owned typed error projection into the sole shared diagnostic policy.
//!
//! This adapter never formats or traverses error chains. The shared router owns
//! catalogue semantics, validation, encoding, finite queues, consumption and health.
//! It has no service, worker, native completion or clock authority.

use windows_platform::{
    diagnostics::{
        Component, DiagnosticObservation, DiagnosticSnapshot, ImportanceMask, LifecycleCause,
        MemoryConsumer, MemoryRoutes, OptionalProfile, PrimaryCause, Router, WorkerCause,
    },
    service::LifecycleError,
};

pub(super) use windows_platform::diagnostics::ServiceEvent as Event;

use crate::{FixtureError, LabRole, NativeError, error::WorkerError};

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

#[cfg(test)]
#[path = "diagnostics_tests.rs"]
mod tests;

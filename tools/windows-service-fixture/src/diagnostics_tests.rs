// SPDX-License-Identifier: GPL-2.0-only
//! Actual adapter -> shared queue -> concrete consumer observations.

use windows_platform::{
    diagnostics::{ConsumerState, DecodedRecord, DiagnosticObservation, MemoryConsumer},
    service::{BudgetError, LifecycleError},
};

use super::{Diagnostics, Event, Sinks, lifecycle_cause, primary_cause, worker_cause};
use crate::{FixtureError, LabRole, NativeError, NativeOperation, error::WorkerError};

/// The typed projection keeps its independently fixed cause without formatting.
/// # Errors
/// Propagate only checked fixture construction failures.
/// # Panics
/// Fails if a cause loses its fixed category or retains a raw payload.
#[test]
fn sensitive_observation_and_nested_cleanup_have_fixed_nonrecursive_categories()
-> Result<(), Box<dyn std::error::Error>> {
    use crate::{
        TokenCleanupOutcome, TokenObservationCause, TokenObservationFailure, error::ServiceFailure,
    };
    use windows_platform::privilege_observation::{
        PrivilegeLuid, PrivilegeSnapshot, ServicePrivilegeIds,
    };

    let failure = FixtureError::TokenObservation(TokenObservationFailure::new(
        TokenObservationCause::Native(NativeError::new(
            NativeOperation::PrivilegeQuery,
            0x5345_4352,
        )),
        TokenCleanupOutcome::Failed(NativeError::new(NativeOperation::TokenClose, u32::MAX)),
        TokenCleanupOutcome::NotOpened,
    ));
    assert_eq!(primary_cause(&failure).code(), 24);
    let ids = ServicePrivilegeIds::new([
        PrivilegeLuid::new(17, 0),
        PrivilegeLuid::new(23, 0),
        PrivilegeLuid::new(41, 0),
        PrivilegeLuid::new(59, 0),
    ])?;
    let snapshot =
        PrivilegeSnapshot::decode(&[1, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0], ids)?;
    let admission = snapshot
        .require_role(windows_platform::service::ServiceRole::Engine)
        .err()
        .ok_or("disabled privilege admitted")?;
    assert_eq!(
        primary_cause(&FixtureError::PrivilegeAdmission(admission)).code(),
        25
    );
    let nested = FixtureError::Cleanup(ServiceFailure::new(
        FixtureError::Cleanup(ServiceFailure::new(failure, None, None, None, None)),
        Some(WorkerError::Running),
        Some(WorkerError::Cleanup),
        None,
        None,
    ));
    assert_eq!(primary_cause(&nested).code(), 28);
    Ok(())
}

/// The typed projection keeps its independently fixed cause without formatting.
/// # Errors
/// Propagate only checked fixture construction failures.
/// # Panics
/// Fails if a cause loses its fixed category or retains a raw payload.
#[test]
fn every_lifecycle_and_worker_category_has_a_fixed_cause() {
    for (failure, expected) in [
        (
            LifecycleError::Report(NativeError::new(NativeOperation::Status, u32::MAX)),
            10,
        ),
        (LifecycleError::DeadlineElapsed, 11),
        (LifecycleError::InvalidTransition, 12),
        (LifecycleError::ReportingFailed, 13),
        (LifecycleError::TimeReversed, 14),
        (LifecycleError::DeadlineOverflow, 15),
        (LifecycleError::CheckpointOverflow, 16),
        (LifecycleError::OutstandingWork, 17),
        (LifecycleError::WorkLimit, 18),
        (LifecycleError::FailureOutcomeRequired, 19),
    ] {
        assert_eq!(lifecycle_cause(&failure).code(), expected);
        assert_eq!(
            primary_cause(&FixtureError::Lifecycle(failure)).code(),
            expected
        );
    }
    for (failure, expected) in [
        (WorkerError::Preparation, 20),
        (WorkerError::Activation, 21),
        (WorkerError::Running, 22),
        (WorkerError::Cleanup, 23),
    ] {
        assert_eq!(worker_cause(failure).code(), expected);
        assert_eq!(
            primary_cause(&FixtureError::Worker(failure)).code(),
            expected
        );
    }
}

/// The typed projection keeps its independently fixed cause without formatting.
/// # Errors
/// Propagate only checked fixture construction failures.
/// # Panics
/// Fails if a cause loses its fixed category or retains a raw payload.
#[test]
fn fixture_categories_do_not_include_arbitrary_native_codes() {
    for (failure, expected) in [
        (FixtureError::WindowsOnly, 1),
        (FixtureError::AlreadyDispatched, 2),
        (FixtureError::OwnerAlreadySet, 3),
        (FixtureError::UnsupportedArguments, 4),
        (FixtureError::ClockReversed, 5),
        (FixtureError::ClockOverflow, 6),
        (FixtureError::RunLimitElapsed, 7),
        (FixtureError::Budget(BudgetError), 8),
        (
            FixtureError::Native(NativeError::new(NativeOperation::Status, 0x5345_4352)),
            9,
        ),
        (FixtureError::ServiceFailed, 26),
        (FixtureError::OutcomeUnavailable, 27),
    ] {
        assert_eq!(primary_cause(&failure).code(), expected);
    }
}

/// Independently construct the complete v2 literal with unavailable live issuers.
/// Actual lifecycle zero is represented by kind1 and a zero coordinate.
fn literal(
    role: u8,
    id: u8,
    level: u8,
    importance: u8,
    cause: u8,
    sequence: u8,
    observed: bool,
) -> [u8; 128] {
    let mut bytes: [u8; 128] = [0; 128];
    bytes[..8].copy_from_slice(&[2, 1, role, id, level, importance, 1, cause]);
    bytes[8] = sequence;
    bytes[16] = u8::from(observed);
    bytes
}

/// Both routes consume the exact originals, including independent level/importance.
/// # Panics
/// Fails if any complete byte, role, order, cause, availability or route differs.
#[test]
fn complete_v2_literals_reach_actual_consumers() {
    for (role, code) in [(LabRole::Engine, 1), (LabRole::ClockBroker, 2)] {
        let mut diagnostics: Diagnostics = Diagnostics::new(
            role,
            Sinks::new(
                MemoryConsumer::new(ConsumerState::Available),
                Some(MemoryConsumer::new(ConsumerState::Available)),
            ),
        );
        let events: [(Event, u8, u8, u8, u8); 9] = [
            (Event::StartObserved, 1, 3, 5, 0),
            (Event::RunningAcknowledged, 2, 3, 5, 0),
            (Event::AdmissionClosed, 3, 3, 5, 0),
            (
                Event::PrimaryFailure(primary_cause(&FixtureError::Worker(
                    WorkerError::Preparation,
                ))),
                4,
                1,
                2,
                20,
            ),
            (
                Event::CompletionFailure(worker_cause(WorkerError::Cleanup)),
                5,
                1,
                2,
                23,
            ),
            (
                Event::StopReportFailure(lifecycle_cause(&LifecycleError::Report(
                    NativeError::new(NativeOperation::Status, 771),
                ))),
                6,
                1,
                2,
                10,
            ),
            (
                Event::TerminalReportFailure(lifecycle_cause(&LifecycleError::ReportingFailed)),
                7,
                1,
                2,
                13,
            ),
            (Event::Finished, 8, 3, 5, 0),
            (
                Event::OperationFailure(worker_cause(WorkerError::Running)),
                9,
                1,
                2,
                22,
            ),
        ];
        let mut expected: Vec<[u8; 128]> = Vec::new();
        for (sequence, (event, id, level, importance, cause)) in (1_u8..).zip(events) {
            diagnostics.emit(event, DiagnosticObservation::unavailable());
            expected.push(literal(code, id, level, importance, cause, sequence, false));
        }
        let queued: windows_platform::diagnostics::DiagnosticSnapshot = diagnostics.snapshot();
        assert_eq!(queued.required().pending(), 9);
        assert!(queued.required().history().is_empty());
        let snapshot: windows_platform::diagnostics::DiagnosticSnapshot = diagnostics.finish();
        assert!(snapshot.sealed());
        for route in [snapshot.required(), snapshot.optional()] {
            let actual: Vec<[u8; 128]> = route.history().records().copied().collect();
            assert_eq!(actual, expected);
            assert_eq!(route.health().queued().value(), 9);
            assert_eq!(route.health().consumer_attempts().value(), 9);
            assert_eq!(route.health().accepted().value(), 9);
            assert_eq!(route.pending(), 0);
            assert!(route.history_complete());
        }
    }
}

/// Available queues and failing consumers are independently observed; a successful
/// enqueue never implies acceptance, completeness or native/durable delivery.
/// # Panics
/// Fails if any selected failure is hidden or prevents the healthy route consuming.
#[test]
fn queue_admission_is_independent_from_every_consumer_outcome() {
    for state in [
        ConsumerState::Available,
        ConsumerState::Full,
        ConsumerState::Reject,
        ConsumerState::Unavailable,
    ] {
        let mut diagnostics: Diagnostics = Diagnostics::new(
            LabRole::Engine,
            Sinks::new(
                MemoryConsumer::new(state),
                Some(MemoryConsumer::new(ConsumerState::Available)),
            ),
        );
        diagnostics.emit(
            Event::PrimaryFailure(primary_cause(&FixtureError::Native(NativeError::new(
                NativeOperation::Register,
                0x5345_4352,
            )))),
            DiagnosticObservation::unavailable(),
        );
        let queued: windows_platform::diagnostics::DiagnosticSnapshot = diagnostics.snapshot();
        assert_eq!(queued.required().health().queued().value(), 1);
        assert_eq!(queued.required().health().consumer_attempts().value(), 0);
        diagnostics.pump_one();
        let snapshot: windows_platform::diagnostics::DiagnosticSnapshot = diagnostics.finish();
        let health: &windows_platform::diagnostics::RouteHealth = snapshot.required().health();
        assert_eq!(health.queued().value(), 1);
        assert_eq!(health.consumer_attempts().value(), 1);
        assert_eq!(
            health.accepted().value(),
            u64::from(state == ConsumerState::Available)
        );
        assert_eq!(
            health.consumer_full().value(),
            u64::from(state == ConsumerState::Full)
        );
        assert_eq!(
            health.consumer_rejected().value(),
            u64::from(state == ConsumerState::Reject)
        );
        assert_eq!(
            health.consumer_unavailable().value(),
            u64::from(state == ConsumerState::Unavailable)
        );
        assert_eq!(
            snapshot.required().history_complete(),
            state == ConsumerState::Available
        );
        assert_eq!(snapshot.optional().health().accepted().value(), 1);
        assert_eq!(
            snapshot.optional().history().get(0),
            Some(&literal(1, 4, 1, 2, 9, 1, false))
        );
    }
}

/// Historical v1 bytes remain an explicit prior contract, rejected by the v2 reader.
/// # Panics
/// Fails if the current passive decoder silently reinterprets historical bytes.
#[test]
fn historical_v1_literal_is_not_a_supported_v2_record() {
    let historical: [u8; 9] = [1, 1, 4, 2, 9, 1, 0, 0, 0];
    assert!(DecodedRecord::decode(&historical).is_err());
}

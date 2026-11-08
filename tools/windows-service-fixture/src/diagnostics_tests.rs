// SPDX-License-Identifier: GPL-2.0-only
//! Independent wire expectations and finite sink boundary observations.

use windows_platform::service::{BudgetError, LifecycleError};

use super::{Diagnostics, Event, FailureCode, MemorySink, Sinks};
use crate::{FixtureError, LabRole, NativeError, NativeOperation, error::WorkerError};

/// Token/privilege/cleanup categories preserve useful meaning without walking
/// nested failures or formatting native payloads.
/// # Errors
/// Propagate construction failure of the independently encoded privilege fixture.
/// # Panics
/// Fails if private-token or bounded cleanup details enter the output category.
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
    assert_eq!(FailureCode::from_fixture(&failure).0, 24);
    let ids = ServicePrivilegeIds::new([
        PrivilegeLuid::new(17, 0),
        PrivilegeLuid::new(23, 0),
        PrivilegeLuid::new(41, 0),
    ])?;
    let snapshot =
        PrivilegeSnapshot::decode(&[1, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0], ids)?;
    let admission = snapshot
        .require_engine_absence()
        .err()
        .ok_or("disabled privilege admitted")?;
    assert_eq!(
        FailureCode::from_fixture(&FixtureError::PrivilegeAdmission(admission)).0,
        25
    );
    let nested = FixtureError::Cleanup(ServiceFailure::new(
        FixtureError::Cleanup(ServiceFailure::new(failure, None, None, None)),
        Some(WorkerError::Cleanup),
        None,
        None,
    ));
    assert_eq!(FailureCode::from_fixture(&nested).0, 28);
    Ok(())
}

/// Every event has an independently fixed wire meaning, honest severity and order.
/// # Panics
/// Fails if the encoder changes a field, role, sequence or optional route.
#[test]
fn literal_events_reach_both_routes_with_explicit_version_and_role() {
    for (role, role_code) in [(LabRole::Engine, 1), (LabRole::ClockBroker, 2)] {
        let mut diagnostics = Diagnostics::new(
            role,
            Sinks::new(MemorySink::new(true), Some(MemorySink::new(true))),
        );
        for event in [
            Event::StartObserved,
            Event::RunningAcknowledged,
            Event::AdmissionClosed,
            Event::PrimaryFailure(FailureCode::from_worker(WorkerError::Preparation)),
            Event::CompletionFailure(FailureCode::from_worker(WorkerError::Cleanup)),
            Event::StopReportFailure(FailureCode::from_lifecycle(&LifecycleError::Report(
                NativeError::new(NativeOperation::Status, 771),
            ))),
            Event::TerminalReportFailure(FailureCode::from_lifecycle(
                &LifecycleError::ReportingFailed,
            )),
            Event::Finished,
        ] {
            diagnostics.emit(event);
        }
        let expected = [
            [1, role_code, 1, 1, 0, 1, 0, 0, 0],
            [1, role_code, 2, 1, 0, 2, 0, 0, 0],
            [1, role_code, 3, 1, 0, 3, 0, 0, 0],
            [1, role_code, 4, 2, 20, 4, 0, 0, 0],
            [1, role_code, 5, 2, 23, 5, 0, 0, 0],
            [1, role_code, 6, 2, 10, 6, 0, 0, 0],
            [1, role_code, 7, 2, 13, 7, 0, 0, 0],
            [1, role_code, 8, 1, 0, 8, 0, 0, 0],
        ];
        assert_eq!(diagnostics.required().records(), expected);
        assert_eq!(
            diagnostics.optional().map(MemorySink::records),
            Some(expected.as_slice())
        );
        assert_eq!(diagnostics.required().loss(), (0, 0));
    }
}

/// Cause projection is useful but never traverses arbitrary source/error text.
/// # Panics
/// Fails if two distinct lifecycle or worker causes collapse or change wire code.
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
        assert_eq!(FailureCode::from_lifecycle(&failure).0, expected);
        assert_eq!(
            FailureCode::from_fixture(&FixtureError::Lifecycle(failure)).0,
            expected
        );
    }
    for (failure, expected) in [
        (WorkerError::Preparation, 20),
        (WorkerError::Activation, 21),
        (WorkerError::Running, 22),
        (WorkerError::Cleanup, 23),
    ] {
        assert_eq!(FailureCode::from_worker(failure).0, expected);
        assert_eq!(
            FailureCode::from_fixture(&FixtureError::Worker(failure)).0,
            expected
        );
    }
}

/// Ordinary invocation failures retain separate stable categories.
/// # Panics
/// Fails if mapping copies a numeric native payload or erases useful causes.
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
        assert_eq!(FailureCode::from_fixture(&failure).0, expected);
    }
}

/// Each route preserves its first sixteen records; later traffic is bounded loss.
/// # Panics
/// Fails if overflow replaces earlier records, grows memory, wraps or retries.
#[test]
fn full_and_unavailable_routes_have_independent_saturating_loss() {
    let mut diagnostics = Diagnostics::new(
        LabRole::Engine,
        Sinks::new(MemorySink::new(true), Some(MemorySink::new(false))),
    );
    for _ in 0..18 {
        diagnostics.emit(Event::StartObserved);
    }
    assert_eq!(diagnostics.required().records().len(), 16);
    assert_eq!(
        diagnostics.required().records().first(),
        Some(&[1, 1, 1, 1, 0, 1, 0, 0, 0])
    );
    assert_eq!(
        diagnostics.required().records().last(),
        Some(&[1, 1, 1, 1, 0, 16, 0, 0, 0])
    );
    assert_eq!(diagnostics.required().loss(), (2, 0));
    assert_eq!(diagnostics.optional().map(MemorySink::loss), Some((0, 18)));
    diagnostics.sinks.required.full = u32::MAX;
    if let Some(optional) = &mut diagnostics.sinks.optional {
        optional.unavailable = u32::MAX;
    }
    diagnostics.emit(Event::Finished);
    assert_eq!(diagnostics.required().loss(), (u32::MAX, 0));
    assert_eq!(
        diagnostics.optional().map(MemorySink::loss),
        Some((0, u32::MAX))
    );
}

/// Required INFO remains attempted when optional is Off, full or unavailable.
/// # Panics
/// Fails if required routing depends on an optional route or optional loss.
#[test]
fn optional_output_cannot_suppress_required_info() {
    for optional in [None, Some(MemorySink::full()), Some(MemorySink::new(false))] {
        let mut diagnostics = Diagnostics::new(
            LabRole::ClockBroker,
            Sinks::new(MemorySink::new(true), optional),
        );
        diagnostics.emit(Event::RunningAcknowledged);
        assert_eq!(
            diagnostics.required().records(),
            [[1, 2, 2, 1, 0, 1, 0, 0, 0]]
        );
        assert_eq!(diagnostics.required().loss(), (0, 0));
    }
}

/// Lost required delivery cannot block a healthy optional route or create a retry.
/// # Panics
/// Fails if required failure suppresses optional delivery or records false receipt.
#[test]
fn unavailable_required_still_attempts_optional_exactly_once() {
    let mut diagnostics = Diagnostics::new(
        LabRole::Engine,
        Sinks::new(MemorySink::new(false), Some(MemorySink::new(true))),
    );
    diagnostics.emit(Event::Finished);
    let empty: &[[u8; 9]] = &[];
    assert_eq!(diagnostics.required().records(), empty);
    assert_eq!(diagnostics.required().loss(), (0, 1));
    assert_eq!(
        diagnostics.optional().map(MemorySink::records),
        Some([[1, 1, 8, 1, 0, 1, 0, 0, 0]].as_slice())
    );
}

/// The final sequence remains usable; no later record reuses or wraps its identity.
/// # Panics
/// Fails if exact exhaustion loses the last valid event, wraps or retries.
#[test]
fn sequence_exhaustion_preserves_the_last_record_and_reports_loss() {
    let mut diagnostics = Diagnostics::new(
        LabRole::Engine,
        Sinks::new(MemorySink::new(true), Some(MemorySink::new(true))),
    );
    diagnostics.next_sequence = Some(u32::MAX - 1);
    diagnostics.emit(Event::StartObserved);
    diagnostics.emit(Event::Finished);
    diagnostics.emit(Event::StartObserved);
    assert_eq!(
        diagnostics.required().records(),
        [
            [1, 1, 1, 1, 0, 254, 255, 255, 255],
            [1, 1, 8, 1, 0, 255, 255, 255, 255],
        ]
    );
    assert_eq!(diagnostics.sequence_loss, 1);
    assert_eq!(diagnostics.required().loss(), (0, 0));
    diagnostics.sequence_loss = u32::MAX;
    diagnostics.emit(Event::Finished);
    assert_eq!(diagnostics.sequence_loss, u32::MAX);
    assert_eq!(
        diagnostics.optional().map(MemorySink::records),
        Some(diagnostics.required().records())
    );
}

// SPDX-License-Identifier: GPL-2.0-only
//! Contracts for fixed diagnostic categories and separately inspectable causes.
//!
//! Typed-value tests establish no native, hook, export or executable behavior.

use std::{collections::BTreeSet, error::Error};

use windows_platform::service::{BudgetError, LifecycleError, PendingBudget};

use super::{FixtureError, NativeError, NativeOperation, ServiceFailure, WorkerError};

/// Fixture-only category ceiling, separate from product logging/output policy.
const MAX_CATEGORY_BYTES: usize = 64;
/// Exact native failure boundaries, including zero after a reported API failure.
const RAW_CODES: [u32; 3] = [0, 1, u32::MAX];
/// Complete currently selected native-effect classes.
const OPERATIONS: [NativeOperation; 8] = [
    NativeOperation::Dispatch,
    NativeOperation::Register,
    NativeOperation::Status,
    NativeOperation::ThreadToken,
    NativeOperation::ProcessToken,
    NativeOperation::PrivilegeLookup,
    NativeOperation::PrivilegeQuery,
    NativeOperation::TokenClose,
];

/// Observe only ordinary Display output, leaving typed causes separately accessible.
///
/// # Panics
/// Fails if a category is blank, oversized, multiline, non-ASCII or includes a
/// control byte. This helper does not recursively format error sources or Debug.
fn category(error: &dyn Error) -> String {
    let message = error.to_string();
    assert_ne!(message, "");
    assert!(message.len() <= MAX_CATEGORY_BYTES);
    assert!(
        message
            .bytes()
            .all(|byte| byte.is_ascii_graphic() || byte == b' ')
    );
    assert!(message.bytes().any(|byte| byte.is_ascii_alphabetic()));
    message
}

/// Formatting depends on operation class, while exact numeric cause stays typed.
///
/// # Panics
/// Fails if codes change the category, operations collapse, a getter normalizes
/// an original code or an unrelated/system-text cause is introduced.
#[test]
fn native_categories_are_code_independent_and_operation_specific() {
    let mut classes = BTreeSet::new();
    for operation in OPERATIONS {
        let baseline = category(&NativeError::new(operation, 0));
        let semantic_token = match operation {
            NativeOperation::Dispatch => "dispatcher",
            NativeOperation::Register => "registration",
            NativeOperation::Status => "status",
            NativeOperation::ThreadToken => "thread token",
            NativeOperation::ProcessToken => "process token",
            NativeOperation::PrivilegeLookup => "privilege lookup",
            NativeOperation::PrivilegeQuery => "privilege capture",
            NativeOperation::TokenClose => "handle close",
        };
        assert!(baseline.contains(semantic_token));
        for code in RAW_CODES {
            let native = NativeError::new(operation, code);
            assert_eq!(native.operation(), operation);
            assert_eq!(native.code(), code);
            assert!(native.source().is_none());
            assert_eq!(category(&native), baseline);
        }
        assert!(classes.insert(baseline));
    }
    assert_eq!(classes.len(), OPERATIONS.len());
}

/// Every current fixture error category is bounded and distinct at its own layer.
/// Cause-free variants expose no synthetic nested failure; wrapped causes remain
/// independently inspectable in the following typed-source tests.
///
/// # Panics
/// Fails if a category collapses into another, exceeds its budget or loses/adds
/// a cause contrary to that variant's actual type contract.
#[test]
fn fixture_variant_categories_are_distinct_and_have_the_declared_source_presence() {
    let cases = [
        (FixtureError::WindowsOnly, false),
        (FixtureError::AlreadyDispatched, false),
        (FixtureError::OwnerAlreadySet, false),
        (FixtureError::UnsupportedArguments, false),
        (FixtureError::ClockReversed, false),
        (FixtureError::ClockOverflow, false),
        (FixtureError::RunLimitElapsed, false),
        (FixtureError::Budget(BudgetError), true),
        (
            FixtureError::Lifecycle(LifecycleError::DeadlineElapsed),
            true,
        ),
        (
            FixtureError::Native(NativeError::new(NativeOperation::Dispatch, u32::MAX)),
            true,
        ),
        (FixtureError::ServiceFailed, false),
        (FixtureError::OutcomeUnavailable, false),
        (FixtureError::Worker(WorkerError::Preparation), true),
        (
            FixtureError::Cleanup(ServiceFailure::new(
                FixtureError::Worker(WorkerError::Preparation),
                Some(WorkerError::Cleanup),
                None,
                None,
            )),
            true,
        ),
    ];
    let expected_classes = cases.len();
    let mut classes = BTreeSet::new();
    for (error, has_source) in cases {
        assert_eq!(error.source().is_some(), has_source);
        assert!(classes.insert(category(&error)));
    }
    assert_eq!(classes.len(), expected_classes);
}

/// Worker and bounded cleanup failures preserve typed causes without formatting
/// numeric reporter codes or recursively joining their diagnostic messages.
///
/// # Errors
/// Propagate missing or changed typed-source hops.
///
/// # Panics
/// Fail if categories collapse, secondary slots disappear or the primary changes.
#[test]
fn worker_cleanup_facts_retain_the_primary_and_each_bounded_secondary() -> Result<(), Box<dyn Error>>
{
    let mut categories = BTreeSet::new();
    for cause in [
        WorkerError::Preparation,
        WorkerError::Activation,
        WorkerError::Running,
        WorkerError::Cleanup,
    ] {
        assert!(categories.insert(category(&cause)));
        let wrapped = FixtureError::Worker(cause);
        assert_eq!(
            wrapped
                .source()
                .and_then(|source| source.downcast_ref::<WorkerError>()),
            Some(&cause),
        );
        category(&wrapped);
    }
    let first = NativeError::new(NativeOperation::Status, 717);
    let stop = NativeError::new(NativeOperation::Status, 718);
    let terminal = NativeError::new(NativeOperation::Status, 719);
    let facts = ServiceFailure::new(
        FixtureError::Lifecycle(LifecycleError::Report(first)),
        Some(WorkerError::Cleanup),
        Some(LifecycleError::Report(stop)),
        Some(LifecycleError::Report(terminal)),
    );
    assert_eq!(
        facts.primary(),
        &FixtureError::Lifecycle(LifecycleError::Report(first)),
    );
    assert_eq!(facts.completion(), Some(WorkerError::Cleanup));
    assert_eq!(facts.stop_report(), Some(&LifecycleError::Report(stop)));
    assert_eq!(
        facts.terminal_report(),
        Some(&LifecycleError::Report(terminal)),
    );
    category(&facts);
    let wrapped = FixtureError::Cleanup(facts);
    category(&wrapped);
    let source = wrapped.source().ok_or("bounded cleanup source missing")?;
    let facts = source
        .downcast_ref::<ServiceFailure>()
        .ok_or("cleanup source changed")?;
    assert_eq!(
        source
            .source()
            .and_then(|cause| cause.downcast_ref::<FixtureError>()),
        Some(facts.primary()),
    );
    Ok(())
}

/// An actual rejected budget retains its typed cause through the fixture layer.
///
/// # Panics
/// Fails if the source is replaced by text, gains an unrelated nested source or
/// changes its original typed identity.
///
/// # Errors
/// Fail this fixture if the independently invalid zero-hint budget is accepted.
#[test]
fn rejected_budget_preserves_the_exact_typed_source() -> Result<(), Box<dyn Error>> {
    let budget = PendingBudget::new(0, 1)
        .err()
        .ok_or("invalid zero-hint fixture budget unexpectedly admitted")?;
    let wrapped = FixtureError::Budget(budget);
    let source = wrapped.source().ok_or("budget cause missing")?;
    assert_eq!(source.downcast_ref::<BudgetError>(), Some(&budget));
    assert!(source.source().is_none());
    category(&wrapped);
    Ok(())
}

/// Native and lifecycle wrappers retain exact codes without displaying payloads.
/// Different codes/operations remain one category at each wrapper layer; only
/// explicit typed source traversal exposes their original native identity.
///
/// # Panics
/// Fails if native codes or operations change, category text depends on them,
/// an intermediate lifecycle type is lost or source-chain termination changes.
///
/// # Errors
/// Fail this fixture if any expected typed source is absent.
#[test]
fn wrapped_native_causes_preserve_each_typed_hop_without_recursive_formatting()
-> Result<(), Box<dyn Error>> {
    let native_category = category(&FixtureError::Native(NativeError::new(
        NativeOperation::Dispatch,
        0,
    )));
    let lifecycle_category = category(&FixtureError::Lifecycle(LifecycleError::Report(
        NativeError::new(NativeOperation::Dispatch, 0),
    )));
    assert_ne!(native_category, lifecycle_category);
    for operation in OPERATIONS {
        for code in RAW_CODES {
            let native = NativeError::new(operation, code);
            let wrapped = FixtureError::Native(native);
            assert_eq!(category(&wrapped), native_category);
            let source = wrapped.source().ok_or("native cause missing")?;
            let recovered = source
                .downcast_ref::<NativeError>()
                .ok_or("native cause replaced by another type")?;
            assert_eq!(recovered.operation(), operation);
            assert_eq!(recovered.code(), code);
            assert!(source.source().is_none());

            let wrapped = FixtureError::Lifecycle(LifecycleError::Report(native));
            assert_eq!(category(&wrapped), lifecycle_category);
            let lifecycle = wrapped.source().ok_or("lifecycle cause missing")?;
            assert_eq!(
                lifecycle.downcast_ref::<LifecycleError<NativeError>>(),
                Some(&LifecycleError::Report(native))
            );
            let source = lifecycle.source().ok_or("reporter cause missing")?;
            let recovered = source
                .downcast_ref::<NativeError>()
                .ok_or("reporter cause replaced by another type")?;
            assert_eq!(recovered.operation(), operation);
            assert_eq!(recovered.code(), code);
            assert!(source.source().is_none());
        }
    }
    Ok(())
}

/// Non-report lifecycle failures retain their semantic cause under one safe category.
///
/// # Panics
/// Fails if a lifecycle refusal loses its typed identity, recursively alters the
/// wrapper category or acquires an invented native cause.
///
/// # Errors
/// Fail this fixture if a declared lifecycle source is absent.
#[test]
fn semantic_lifecycle_causes_remain_typed_and_do_not_inherit_native_sources()
-> Result<(), Box<dyn Error>> {
    let baseline = category(&FixtureError::Lifecycle(LifecycleError::DeadlineElapsed));
    for cause in [
        LifecycleError::InvalidTransition,
        LifecycleError::ReportingFailed,
        LifecycleError::TimeReversed,
        LifecycleError::DeadlineElapsed,
        LifecycleError::DeadlineOverflow,
        LifecycleError::CheckpointOverflow,
        LifecycleError::OutstandingWork,
        LifecycleError::WorkLimit,
        LifecycleError::FailureOutcomeRequired,
    ] {
        let wrapped = FixtureError::Lifecycle(cause);
        assert_eq!(category(&wrapped), baseline);
        let source = wrapped.source().ok_or("semantic lifecycle cause missing")?;
        let FixtureError::Lifecycle(original) = &wrapped else {
            return Err("fixture did not retain its selected lifecycle variant".into());
        };
        assert_eq!(
            source.downcast_ref::<LifecycleError<NativeError>>(),
            Some(original)
        );
        assert!(source.source().is_none());
    }
    Ok(())
}

/// New token diagnostics retain primary cause and both explicit cleanup outcomes.
///
/// # Errors
/// Propagate checked fixture construction and missing typed-source failures.
/// # Panics
/// Fails if cleanup replaces first cause, disabled presence loses identity, or
/// bounded wrapper categories collapse or recursively expose raw native codes.
#[test]
fn token_observation_and_privilege_admission_preserve_typed_cause_custody()
-> Result<(), Box<dyn Error>> {
    use super::{TokenCleanupOutcome, TokenObservationCause, TokenObservationFailure};
    use windows_platform::privilege_observation::{
        PrivilegeLuid, PrivilegeSnapshot, PrivilegeState, ServicePrivilege, ServicePrivilegeError,
        ServicePrivilegeIds,
    };
    let first = NativeError::new(NativeOperation::PrivilegeQuery, 0);
    let primary = TokenCleanupOutcome::Failed(NativeError::new(NativeOperation::TokenClose, 6));
    let thread = TokenCleanupOutcome::Failed(NativeError::new(NativeOperation::TokenClose, 7));
    let failure =
        TokenObservationFailure::new(TokenObservationCause::Native(first), primary, thread);
    assert_eq!(failure.cause(), TokenObservationCause::Native(first));
    assert_eq!(failure.primary_cleanup(), primary);
    assert_eq!(failure.thread_cleanup(), thread);
    let wrapped = FixtureError::TokenObservation(failure);
    assert_eq!(
        wrapped
            .source()
            .and_then(|error| error.downcast_ref::<TokenObservationFailure>()),
        Some(&failure)
    );
    assert_eq!(
        wrapped
            .source()
            .and_then(Error::source)
            .and_then(|error| error.downcast_ref::<TokenObservationCause>()),
        Some(&TokenObservationCause::Native(first))
    );
    assert_eq!(
        wrapped
            .source()
            .and_then(Error::source)
            .and_then(Error::source)
            .and_then(|error| error.downcast_ref::<NativeError>()),
        Some(&first)
    );
    category(&failure);
    category(&failure.cause());
    let observation_category = category(&wrapped);
    let ids = ServicePrivilegeIds::new([
        PrivilegeLuid::new(17, 0),
        PrivilegeLuid::new(23, 0),
        PrivilegeLuid::new(41, 0),
        PrivilegeLuid::new(59, 0),
    ])?;
    let raw = [1, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0];
    let snapshot = PrivilegeSnapshot::decode(&raw, ids)?;
    let admission = snapshot
        .require_role(windows_platform::service::ServiceRole::Engine)
        .err()
        .ok_or("disabled privilege admitted")?;
    let ServicePrivilegeError::Forbidden(facts) = admission else {
        return Err("known forbidden privilege lost its typed facts".into());
    };
    assert_eq!(facts.privilege(), ServicePrivilege::SystemTime);
    assert_eq!(facts.state(), PrivilegeState::PresentDisabled);
    let wrapped = FixtureError::PrivilegeAdmission(admission);
    assert_eq!(
        wrapped
            .source()
            .and_then(|error| error.downcast_ref::<ServicePrivilegeError>()),
        Some(&admission)
    );
    assert_eq!(
        wrapped
            .source()
            .and_then(Error::source)
            .and_then(|error| error
                .downcast_ref::<windows_platform::privilege_observation::EnginePrivilegeError>()),
        Some(&facts)
    );
    assert_ne!(category(&wrapped), observation_category);
    for cause in [
        TokenObservationCause::UnexpectedThreadToken,
        TokenObservationCause::InvalidHandle,
        TokenObservationCause::Capture(
            windows_platform::privilege_observation::PrivilegeCaptureError::Length,
        ),
    ] {
        category(&cause);
        let failure = TokenObservationFailure::new(
            cause,
            TokenCleanupOutcome::NotOpened,
            TokenCleanupOutcome::Closed,
        );
        assert_eq!(failure.cause(), cause);
        assert_eq!(
            category(&FixtureError::TokenObservation(failure)),
            observation_category
        );
        assert_eq!(
            cause.source().is_some(),
            matches!(cause, TokenObservationCause::Capture(_))
        );
    }
    Ok(())
}

/// Role-profile errors retain typed causes while outer formatting excludes every
/// raw unknown privilege identity and accepted capture attribute.
///
/// # Errors
/// Propagate checked identity/raw construction or absence of the expected refusal.
/// # Panics
/// Fails if Debug, Display or the typed source projects arbitrary token contents.
#[test]
fn role_profile_error_formatting_excludes_raw_unknown_token_data() -> Result<(), Box<dyn Error>> {
    use windows_platform::privilege_observation::{
        PrivilegeLuid, PrivilegeSnapshot, ServicePrivilegeError, ServicePrivilegeIds,
    };
    let ids = ServicePrivilegeIds::new([
        PrivilegeLuid::new(17, 0),
        PrivilegeLuid::new(23, 0),
        PrivilegeLuid::new(41, 0),
        PrivilegeLuid::new(59, 0),
    ])?;
    let mut raw = 2_u32.to_le_bytes().to_vec();
    for (low, high, attributes) in [
        (59_u32, 0_i32, 2_u32),
        (0x5345_4352, -123_456_789, 0x8000_0001),
    ] {
        raw.extend(low.to_le_bytes());
        raw.extend(high.to_le_bytes());
        raw.extend(attributes.to_le_bytes());
    }
    let snapshot = PrivilegeSnapshot::decode(&raw, ids)?;
    let refusal = snapshot
        .require_role(windows_platform::service::ServiceRole::Engine)
        .err()
        .ok_or("unknown extra admitted")?;
    assert_eq!(refusal, ServicePrivilegeError::UnexpectedPrivilege);
    let error = FixtureError::PrivilegeAdmission(refusal);
    assert_eq!(
        format!("{error:?}"),
        "PrivilegeAdmission(UnexpectedPrivilege)"
    );
    assert_eq!(
        category(&error),
        "laboratory service privilege admission failed"
    );
    assert_eq!(
        error
            .source()
            .and_then(|cause| cause.downcast_ref::<ServicePrivilegeError>()),
        Some(&ServicePrivilegeError::UnexpectedPrivilege)
    );
    Ok(())
}

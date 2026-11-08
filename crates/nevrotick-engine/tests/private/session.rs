// SPDX-License-Identifier: GPL-2.0-only
//! Independent lifecycle guards; modeled execution cases create no native work.

use super::*;
use std::{net::SocketAddr, panic::AssertUnwindSafe, time::Duration};

/// Preserve actual fixture and preparation errors without semantic conversion.
type Check<T> = Result<T, Box<dyn Error>>;

/// Assert a reviewed pure product success without formatting its error payload.
/// # Errors
/// Keep the original typed result for the existing propagation path.
/// # Panics
/// Reject only the independently known-valid product result named by the static fact.
fn product_ok<T, E>(result: Result<T, E>, fact: &'static str) -> Result<T, E> {
    assert!(result.is_ok(), "{fact}");
    result
}

/// Independent reuse expectations; synthetic causes do not claim native observations.
const COHERENCE_REUSE_CASES: [(SeriesCoherence, bool); 13] = [
    (SeriesCoherence::ObservedCompatible, true),
    (SeriesCoherence::ObservedInconsistent, false),
    (
        SeriesCoherence::Unavailable(CoherenceUnavailable::NoProfile),
        true,
    ),
    (
        SeriesCoherence::Unavailable(CoherenceUnavailable::NoFacts),
        true,
    ),
    (
        SeriesCoherence::Unavailable(CoherenceUnavailable::Capture(
            MeasurementRefusal::Unavailable,
        )),
        false,
    ),
    (
        SeriesCoherence::Unavailable(CoherenceUnavailable::Capture(
            MeasurementRefusal::InvalidProfile,
        )),
        false,
    ),
    (
        SeriesCoherence::Unavailable(CoherenceUnavailable::Capture(
            MeasurementRefusal::CaptureOrder,
        )),
        false,
    ),
    (
        SeriesCoherence::Unavailable(CoherenceUnavailable::Capture(
            MeasurementRefusal::MappingDiscontinuity,
        )),
        false,
    ),
    (
        SeriesCoherence::Unavailable(CoherenceUnavailable::Capture(MeasurementRefusal::DateScope)),
        false,
    ),
    (
        SeriesCoherence::Unavailable(CoherenceUnavailable::Capture(
            MeasurementRefusal::EraNoCandidate,
        )),
        false,
    ),
    (
        SeriesCoherence::Unavailable(CoherenceUnavailable::Capture(
            MeasurementRefusal::EraAmbiguous,
        )),
        false,
    ),
    (
        SeriesCoherence::Unavailable(CoherenceUnavailable::Capture(
            MeasurementRefusal::Arithmetic,
        )),
        false,
    ),
    (
        SeriesCoherence::Unavailable(CoherenceUnavailable::Capture(MeasurementRefusal::Protocol)),
        false,
    ),
];

/// Supply a checked laboratory configuration; no socket is opened here.
/// # Errors
/// Preserve the original configuration constructor refusal.
fn config() -> Result<SeriesConfig, PilotError> {
    product_ok(
        SeriesConfig::new(
            SocketAddr::from(([127, 0, 0, 1], 41001)),
            Duration::from_millis(100),
            Duration::from_secs(1),
        ),
        "known valid private session configuration was refused",
    )
}

/// Prepare a known Idle owner, retaining genuine entropy/driver errors as errors.
/// # Errors
/// Preserve original driver preparation failure, including entropy failure.
/// # Panics
/// Reject a structural/session refusal of this independently known Idle owner.
fn idle_plan(owner: &mut AcquisitionSession) -> Check<(PreparedAcquisition<'_>, ControlHandle)> {
    let result = owner.prepare();
    assert!(
        !matches!(
            &result,
            Err(AcquisitionError::Busy
                | AcquisitionError::Retired
                | AcquisitionError::Invariant
                | AcquisitionError::Driver(PilotError::InternalState))
        ),
        "known idle session refused actual preparation"
    );
    Ok(result?)
}

/// Drop an unused actual preparation before permitting a new preparation.
/// # Errors
/// Preserve checked configuration and actual preparation failures.
/// # Panics
/// Require release of the original watch receiver and reuse of the same chart.
#[test]
fn unused_plan_drop_releases_capacity_without_replacing_the_chart() -> Check<()> {
    let mut owner = AcquisitionSession::new(config()?);
    let chart = Arc::clone(&owner.chart);
    let (plan, control) = idle_plan(&mut owner)?;
    assert_eq!(plan.session.state, SessionState::Prepared);
    assert_eq!(control.sender.receiver_count(), 1);
    assert!(Arc::ptr_eq(&plan.chart, &chart));
    drop(plan);
    assert_eq!(control.sender.receiver_count(), 0);
    assert_eq!(owner.state, SessionState::Idle);
    control.stop();
    control.invalidate()?;
    let (next, next_control) = idle_plan(&mut owner)?;
    assert!(Arc::ptr_eq(&next.chart, &chart));
    assert!(!next_control.sender.borrow().stop);
    assert_eq!(next_control.sender.borrow().epoch, 0);
    drop(next);
    assert_eq!(owner.state, SessionState::Idle);
    Ok(())
}

/// Deliberate forgetting is a fault control, not an ordinary resource strategy.
/// # Errors
/// Preserve actual configuration and entropy errors.
/// # Panics
/// Require Busy to remain until terminal retirement; retirement never restores Idle.
#[test]
#[expect(
    clippy::mem_forget,
    reason = "deliberately challenge forgotten prepared-guard capacity"
)]
fn forgotten_plan_keeps_busy_until_idempotent_terminal_retirement() -> Check<()> {
    let mut owner = AcquisitionSession::new(config()?);
    let (plan, _control) = idle_plan(&mut owner)?;
    std::mem::forget(plan);
    assert_eq!(owner.state, SessionState::Prepared);
    assert!(matches!(owner.prepare(), Err(AcquisitionError::Busy)));
    owner.retire();
    owner.retire();
    assert_eq!(owner.state, SessionState::Retired);
    assert!(matches!(owner.prepare(), Err(AcquisitionError::Retired)));
    let mut fresh = AcquisitionSession::new(config()?);
    let (neighbor, _control) = idle_plan(&mut fresh)?;
    drop(neighbor);
    assert_eq!(fresh.state, SessionState::Idle);
    Ok(())
}

/// An entered-runtime refusal occurs before preparation's entropy operation.
/// # Errors
/// Preserve actual runtime/configuration/healthy preparation errors.
/// # Panics
/// Require failed prepare to remain Idle and failed consuming execute to retire.
#[test]
fn entered_runtime_refusals_preserve_prepare_rollback_and_retire_consumed_execution() -> Check<()> {
    let runtime = tokio::runtime::Builder::new_current_thread().build()?;
    let mut owner = AcquisitionSession::new(config()?);
    {
        let _entered = runtime.enter();
        assert!(matches!(
            owner.prepare(),
            Err(AcquisitionError::Driver(PilotError::EnteredRuntime))
        ));
    }
    assert_eq!(owner.state, SessionState::Idle);
    let (plan, control) = idle_plan(&mut owner)?;
    let schedule = ControlSchedule::new(None, None)?;
    {
        let _entered = runtime.enter();
        assert!(matches!(
            plan.execute(&schedule),
            Err(AcquisitionError::Driver(PilotError::EnteredRuntime))
        ));
    }
    assert_eq!(control.sender.receiver_count(), 0);
    assert_eq!(owner.state, SessionState::Retired);
    assert!(matches!(owner.prepare(), Err(AcquisitionError::Retired)));
    Ok(())
}

/// Modeled busy states challenge the real preparation entrypoint before delegation.
/// No live executing acquisition or extra mutable alias is manufactured here.
/// # Errors
/// Preserve actual runtime/configuration errors.
/// # Panics
/// Require Busy/Retired to precede the underlying entered-runtime refusal.
#[test]
fn private_busy_and_retired_states_reject_before_driver_preparation() -> Check<()> {
    let runtime = tokio::runtime::Builder::new_current_thread().build()?;
    for state in [
        SessionState::Prepared,
        SessionState::Executing,
        SessionState::Retired,
    ] {
        let mut owner = AcquisitionSession::new(config()?);
        owner.state = state;
        let _entered = runtime.enter();
        let result = owner.prepare();
        match state {
            SessionState::Prepared | SessionState::Executing => {
                assert!(matches!(result, Err(AcquisitionError::Busy)));
            }
            SessionState::Retired => assert!(matches!(result, Err(AcquisitionError::Retired))),
            SessionState::Idle => {
                return Err(std::io::Error::other("fixture selected an undeclared state").into());
            }
        }
    }
    Ok(())
}

/// Genuine preparation plus synthetic private contradictions must fail closed.
/// No report is fabricated, attached to a chart or returned by a substitute runner.
/// # Errors
/// Preserve actual configuration/preparation failures.
/// # Panics
/// Require missing plans, foreign equal-origin charts and wrong states to retire.
#[test]
fn private_plan_state_chart_and_missing_inner_contradictions_retire() -> Check<()> {
    for fault in 0..5 {
        let mut owner = AcquisitionSession::new(config()?);
        let (mut plan, control) = idle_plan(&mut owner)?;
        match fault {
            0 => drop(plan.prepared.take()),
            1 => {
                plan.chart = Arc::new(SessionChart {
                    origin: plan.chart.origin,
                });
            }
            2 => plan.session.state = SessionState::Idle,
            3 => plan.session.state = SessionState::Executing,
            4 => plan.session.state = SessionState::Retired,
            _ => {
                return Err(
                    std::io::Error::other("fixture selected an undeclared contradiction").into(),
                );
            }
        }
        let result = plan.begin_execution();
        assert!(matches!(&result, Err(AcquisitionError::Invariant)));
        drop(result);
        drop(plan);
        assert_eq!(control.sender.receiver_count(), 0);
        assert_eq!(owner.state, SessionState::Retired);
        assert!(matches!(owner.prepare(), Err(AcquisitionError::Retired)));
    }
    Ok(())
}

/// Challenge Drop directly, without letting `begin_execution` retire faults first.
/// Private chart/state changes are synthetic; every inner preparation is genuine.
/// # Errors
/// Preserve actual configuration/preparation failures.
/// # Panics
/// Require contradictory unused guards to release resources and retire permanently.
#[test]
fn private_drop_contradictions_retire_without_entering_execution() -> Check<()> {
    let mut healthy = AcquisitionSession::new(config()?);
    let (neighbor, neighbor_control) = idle_plan(&mut healthy)?;
    drop(neighbor);
    assert_eq!(neighbor_control.sender.receiver_count(), 0);
    assert_eq!(healthy.state, SessionState::Idle);
    let (next, _control) = idle_plan(&mut healthy)?;
    drop(next);
    for fault in 0..3 {
        let mut owner = AcquisitionSession::new(config()?);
        let (mut plan, control) = idle_plan(&mut owner)?;
        assert_eq!(plan.session.state, SessionState::Prepared);
        assert_eq!(control.sender.receiver_count(), 1);
        match fault {
            0 => {
                plan.chart = Arc::new(SessionChart {
                    origin: plan.chart.origin,
                });
            }
            1 => drop(plan.prepared.take()),
            2 => {
                plan.session.state = SessionState::Idle;
            }
            _ => {
                return Err(
                    std::io::Error::other("fixture selected an undeclared Drop fault").into(),
                );
            }
        }
        drop(plan);
        assert_eq!(control.sender.receiver_count(), 0);
        assert_eq!(owner.state, SessionState::Retired);
        assert!(matches!(owner.prepare(), Err(AcquisitionError::Retired)));
    }
    Ok(())
}

/// Equal numeric origins are a synthetic metadata challenge, not shared identity.
/// Both wrappers still come only from genuine, pre-stopped consuming executions.
/// # Errors
/// Preserve actual socket/configuration/preparation/driver and cleanup failures.
/// # Panics
/// Require equal-origin foreign allocations to remain distinct report memberships.
#[test]
fn equal_origin_foreign_owners_produce_distinct_genuine_report_memberships() -> Check<()> {
    let peer = std::net::UdpSocket::bind(SocketAddr::from(([127, 0, 0, 1], 0)))?;
    let address = peer.local_addr()?;
    let mut first = AcquisitionSession::new(product_ok(
        SeriesConfig::new(address, Duration::from_millis(100), Duration::from_secs(1)),
        "known valid first equal-origin session configuration was refused",
    )?);
    let mut second = AcquisitionSession::new(product_ok(
        SeriesConfig::new(address, Duration::from_millis(100), Duration::from_secs(1)),
        "known valid second equal-origin session configuration was refused",
    )?);
    // Change only the second unique chart's origin, before either preparation.
    Arc::get_mut(&mut second.chart)
        .ok_or_else(|| std::io::Error::other("fixture chart unexpectedly has another owner"))?
        .origin = first.chart.origin;
    assert!(!Arc::ptr_eq(&first.chart, &second.chart));
    assert_eq!(first.chart.origin, second.chart.origin);
    let schedule = ControlSchedule::new(None, None)?;
    let reports = [&mut first, &mut second].map(|owner| -> Check<AcquisitionReport> {
        let (plan, control) = idle_plan(owner)?;
        control.stop();
        let result = plan.execute(&schedule);
        assert!(
            !matches!(
                &result,
                Err(AcquisitionError::Busy
                    | AcquisitionError::Retired
                    | AcquisitionError::Invariant
                    | AcquisitionError::Driver(PilotError::InternalState))
            ),
            "genuine prepared session refused pre-stopped execution"
        );
        let report = result?;
        let series = report.series();
        if series.stop_cleanup_failure().is_some()
            || series.invalidate_cleanup_failure().is_some()
            || series.primary_failure().is_some_and(|cause| {
                cause.native_cause().is_some() || cause.code() != "internal_state"
            })
            || series.attempts().iter().any(|attempt| {
                attempt.report().is_some_and(|record| {
                    record.status() == crate::Status::Io || record.io_cause().is_some()
                })
            })
        {
            return Err(
                std::io::Error::other("actual pre-stopped driver or cleanup failed").into(),
            );
        }
        assert!(
            series.primary_failure().is_none(),
            "genuine pre-stopped session returned an internal invariant failure"
        );
        assert_eq!(series.end(), SeriesEnd::Stopped);
        assert_eq!(control.sender.receiver_count(), 0);
        Ok(report)
    });
    let [first_result, second_result] = reports;
    let first_report = first_result?;
    let second_report = second_result?;
    assert!(matches!(
        first_report.reference_at(0),
        Err(AcquisitionReferenceRefusal::FactsUnavailable)
    ));
    assert!(matches!(
        second_report.reference_at(0),
        Err(AcquisitionReferenceRefusal::FactsUnavailable)
    ));
    assert_eq!(first_report.chart_origin(), second_report.chart_origin());
    assert!(!first_report.same_session(&second_report));
    assert!(!second_report.same_session(&first_report));
    assert!(first_report.same_session(&first_report));
    assert!(second_report.same_session(&second_report));
    assert_eq!(first.state, SessionState::Idle);
    assert_eq!(second.state, SessionState::Idle);
    Ok(())
}

/// Starting then abandoning the private guard models uncertain execution only.
/// It does not claim an observed native failure or native cleanup qualification.
/// # Errors
/// Preserve actual configuration/preparation errors.
/// # Panics
/// Require Executing Drop to retire instead of reopening the slot.
#[test]
fn synthetic_started_guard_drop_retires_without_reopening_capacity() -> Check<()> {
    let mut owner = AcquisitionSession::new(config()?);
    let (mut plan, control) = idle_plan(&mut owner)?;
    let begun = plan.begin_execution();
    assert!(
        begun.is_ok(),
        "genuine prepared guard refused entering execution"
    );
    let inner = begun?;
    assert_eq!(plan.session.state, SessionState::Executing);
    drop(inner);
    drop(plan);
    assert_eq!(control.sender.receiver_count(), 0);
    assert_eq!(owner.state, SessionState::Retired);
    Ok(())
}

/// Inject an unwind after the actual guard entered Executing, before native work.
/// # Errors
/// Preserve actual configuration/preparation errors.
/// # Panics
/// Catch only the deliberately injected panic and require fail-closed retirement.
#[test]
#[expect(
    clippy::panic,
    reason = "deliberate caught unwind verifies executing-guard retirement"
)]
fn synthetic_execution_unwind_retires_and_releases_prepared_resources() -> Check<()> {
    let mut owner = AcquisitionSession::new(config()?);
    let (mut plan, control) = idle_plan(&mut owner)?;
    let begun = plan.begin_execution();
    assert!(
        begun.is_ok(),
        "genuine prepared guard refused entering execution"
    );
    let inner = begun?;
    let observed = std::panic::catch_unwind(AssertUnwindSafe(move || {
        let _plan = plan;
        let _inner = inner;
        panic!("deliberate session lifecycle unwind");
    }));
    assert!(observed.is_err());
    assert_eq!(control.sender.receiver_count(), 0);
    assert_eq!(owner.state, SessionState::Retired);
    assert!(matches!(owner.prepare(), Err(AcquisitionError::Retired)));
    Ok(())
}

/// Enumerate all labels, eight cause masks and thirteen independent coherence cases.
/// These are pure outcome-classifier cases, not injected or observed native errors.
/// # Panics
/// Only clean normal labels without a coherence veto may return Idle.
#[test]
fn completion_classification_covers_all_end_and_failure_combinations() {
    for (end, clean) in [
        (SeriesEnd::Completed, SessionState::Idle),
        (SeriesEnd::Stopped, SessionState::Idle),
        (SeriesEnd::SeriesDeadline, SessionState::Idle),
        (SeriesEnd::Stale, SessionState::Retired),
        (SeriesEnd::Failure, SessionState::Retired),
    ] {
        for (coherence, permits_reuse) in COHERENCE_REUSE_CASES {
            for flags in 0_u8..8 {
                let expected = if flags == 0 && permits_reuse {
                    clean
                } else {
                    SessionState::Retired
                };
                assert_eq!(
                    completion_state(
                        end,
                        flags & 1 != 0,
                        flags & 2 != 0,
                        flags & 4 != 0,
                        coherence,
                    ),
                    expected
                );
            }
        }
    }
}

/// Metadata diagnostics contain only the declared fixed scalar status/cause names.
/// These are synthetic enum values, not observed contradictory-clock execution.
/// # Panics
/// Require the exact bounded Debug projection for every classifier control.
#[test]
fn series_coherence_debug_contains_only_fixed_scalar_causes() {
    for ((coherence, _permits_reuse), expected) in COHERENCE_REUSE_CASES.into_iter().zip([
        "ObservedCompatible",
        "ObservedInconsistent",
        "Unavailable(NoProfile)",
        "Unavailable(NoFacts)",
        "Unavailable(Capture(Unavailable))",
        "Unavailable(Capture(InvalidProfile))",
        "Unavailable(Capture(CaptureOrder))",
        "Unavailable(Capture(MappingDiscontinuity))",
        "Unavailable(Capture(DateScope))",
        "Unavailable(Capture(EraNoCandidate))",
        "Unavailable(Capture(EraAmbiguous))",
        "Unavailable(Capture(Arithmetic))",
        "Unavailable(Capture(Protocol))",
    ]) {
        let debug_fixed = format!("{coherence:?}") == expected;
        assert!(
            debug_fixed,
            "coherence Debug must contain only fixed scalar metadata"
        );
    }
}

/// Error projection retains a typed driver source without exposing chart values.
/// # Errors
/// Preserve configuration and genuine preparation errors.
/// # Panics
/// Require fixed Debug/Display text and the original typed entered-runtime cause.
#[test]
fn session_debug_and_driver_error_sources_are_fixed_and_typed() -> Check<()> {
    let mut owner = AcquisitionSession::new(config()?);
    let owner_redacted = format!("{owner:?}") == "AcquisitionSession { .. }";
    assert!(
        owner_redacted,
        "session Debug must remain fixed and redacted"
    );
    let (plan, _control) = idle_plan(&mut owner)?;
    let plan_redacted = format!("{plan:?}") == "PreparedAcquisition { .. }";
    assert!(
        plan_redacted,
        "prepared Debug must remain fixed and redacted"
    );
    drop(plan);
    for (error, expected_display) in [
        (AcquisitionError::Busy, "acquisition session busy"),
        (AcquisitionError::Retired, "acquisition session retired"),
        (AcquisitionError::Invariant, "acquisition session invariant"),
        (
            AcquisitionError::Driver(PilotError::EnteredRuntime),
            "acquisition driver failed",
        ),
    ] {
        let error_redacted = format!("{error:?}") == "AcquisitionError { .. }";
        assert!(
            error_redacted,
            "acquisition error Debug must remain fixed and redacted"
        );
        let display_fixed = error.to_string() == expected_display;
        assert!(
            display_fixed,
            "acquisition error Display must remain fixed and redacted"
        );
        match &error {
            AcquisitionError::Driver(_) => assert!(error.source().is_some_and(|source| matches!(
                source.downcast_ref::<PilotError>(),
                Some(PilotError::EnteredRuntime)
            ))),
            AcquisitionError::Busy | AcquisitionError::Retired | AcquisitionError::Invariant => {
                assert!(error.source().is_none());
            }
        }
    }
    Ok(())
}

/// Diagnostic projection never formats a retained view, capture or original cause.
/// These are pure error-value controls, not observed backend or arithmetic failures.
/// # Panics
/// Require four fixed diagnostic families and exact typed wrapped sources.
#[test]
fn reference_refusal_diagnostics_and_exact_typed_sources_are_fixed() {
    for (error, expected_display) in [
        (
            AcquisitionReferenceRefusal::AttemptPosition,
            "session reference attempt position unavailable",
        ),
        (
            AcquisitionReferenceRefusal::FactsUnavailable,
            "session reference facts unavailable",
        ),
        (
            AcquisitionReferenceRefusal::Measurement(MeasurementRefusal::Unavailable),
            "session reference measurement unavailable",
        ),
        (
            AcquisitionReferenceRefusal::Reference(RelativeMeasurementRefusal::ReferenceRange),
            "session reference midpoint unavailable",
        ),
    ] {
        let debug_fixed = format!("{error:?}") == "AcquisitionReferenceRefusal { .. }";
        assert!(
            debug_fixed,
            "session reference refusal Debug must remain redacted"
        );
        let display_fixed = error.to_string() == expected_display;
        assert!(
            display_fixed,
            "session reference refusal Display must remain fixed"
        );
        match error {
            AcquisitionReferenceRefusal::AttemptPosition
            | AcquisitionReferenceRefusal::FactsUnavailable => {
                assert!(error.source().is_none());
            }
            AcquisitionReferenceRefusal::Measurement(expected) => {
                assert!(error.source().is_some_and(|source| {
                    source.downcast_ref::<MeasurementRefusal>() == Some(&expected)
                }));
            }
            AcquisitionReferenceRefusal::Reference(expected) => {
                assert!(
                    error
                        .source()
                        .is_some_and(|source| source.downcast_ref::<RelativeMeasurementRefusal>()
                            == Some(&expected))
                );
            }
        }
    }
    for expected in [
        MeasurementRefusal::Unavailable,
        MeasurementRefusal::InvalidProfile,
        MeasurementRefusal::CaptureOrder,
        MeasurementRefusal::MappingDiscontinuity,
        MeasurementRefusal::DateScope,
        MeasurementRefusal::EraNoCandidate,
        MeasurementRefusal::EraAmbiguous,
        MeasurementRefusal::Arithmetic,
        MeasurementRefusal::Protocol,
    ] {
        let error = AcquisitionReferenceRefusal::Measurement(expected);
        assert!(
            error.source().is_some_and(
                |source| source.downcast_ref::<MeasurementRefusal>() == Some(&expected)
            )
        );
    }
    for expected in [
        RelativeMeasurementRefusal::ReferenceBeforeOrigin,
        RelativeMeasurementRefusal::ReferenceOrder,
        RelativeMeasurementRefusal::ReferenceRange,
        RelativeMeasurementRefusal::Arithmetic,
        RelativeMeasurementRefusal::OutputInvariant,
    ] {
        let error = AcquisitionReferenceRefusal::Reference(expected);
        assert!(error.source().is_some_and(|source| {
            source.downcast_ref::<RelativeMeasurementRefusal>() == Some(&expected)
        }));
    }
}

/// Literal synthetic brackets reach threshold selection without creating a report.
/// The unrelated date window demonstrates that capture assessment does not redate facts.
/// # Errors
/// Preserve checked profile/time fixture construction failures without semantic credit.
/// # Panics
/// Require inclusive limit selection, checked-failure precedence and absent-facts refusal.
#[test]
fn synthetic_capture_coherence_preserves_nonzero_limit_and_checked_failure_precedence() -> Check<()>
{
    let profile = product_ok(
        ExperimentalProfile::new(0, 0, 10),
        "known valid synthetic coherence profile was refused",
    )?;
    let origin = Instant::now();
    let capture = |before: u64, after: u64, wall: u64| -> Check<crate::SoftwareClock> {
        Ok(crate::SoftwareClock {
            mono: origin
                .checked_add(Duration::from_nanos(before))
                .ok_or_else(|| std::io::Error::other("synthetic capture start unavailable"))?,
            utc: std::time::UNIX_EPOCH
                .checked_add(Duration::from_nanos(wall))
                .ok_or_else(|| std::io::Error::other("synthetic capture wall time unavailable"))?,
            mono_after: origin
                .checked_add(Duration::from_nanos(after))
                .ok_or_else(|| std::io::Error::other("synthetic capture end unavailable"))?,
        })
    };
    let baseline = capture(0, 100, 0)?;
    let middle = capture(200, 300, 200)?;
    // Wall readings stay on the 100 ns Windows grid. The last wall delta is 500,
    // while its possible monotonic interval ends at 490 ns: the exact gap is 10.
    // Moving only that monotonic endpoint to 489 ns makes the gap exactly 11.
    for (last_after, expected) in [
        (490, SeriesCoherence::ObservedCompatible),
        (489, SeriesCoherence::ObservedInconsistent),
    ] {
        assert_eq!(
            capture_coherence(
                profile,
                &[
                    Some([baseline, middle, capture(400, last_after, 500)?]),
                    None,
                    None
                ],
            ),
            expected
        );
    }
    assert_eq!(
        capture_coherence(
            profile,
            &[
                Some([
                    baseline,
                    capture(200, 300, 2_000)?,
                    capture(400, 300, 2_100)?
                ]),
                None,
                None
            ],
        ),
        SeriesCoherence::Unavailable(CoherenceUnavailable::Capture(
            MeasurementRefusal::CaptureOrder
        ))
    );
    let mut wide_wall = capture(400, 500, 0)?;
    wide_wall.utc = std::time::UNIX_EPOCH
        .checked_add(Duration::from_secs(20_000_000_000))
        .ok_or_else(|| std::io::Error::other("synthetic wide wall time unavailable"))?;
    assert_eq!(
        capture_coherence(
            profile,
            &[
                Some([baseline, capture(200, 300, 2_000)?, wide_wall]),
                None,
                None
            ],
        ),
        SeriesCoherence::Unavailable(CoherenceUnavailable::Capture(
            MeasurementRefusal::Arithmetic
        ))
    );
    assert_eq!(
        capture_coherence(profile, &[None, None, None]),
        SeriesCoherence::Unavailable(CoherenceUnavailable::NoFacts)
    );
    Ok(())
}

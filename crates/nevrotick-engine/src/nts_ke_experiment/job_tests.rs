// SPDX-License-Identifier: GPL-2.0-only
//! Supplemental actual job-owner guards; fixed callback failures make no protocol claim.

use std::sync::OnceLock;
use std::sync::atomic::AtomicUsize;
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::sync::{Arc, LazyLock, Weak};
use std::time::Duration;

use super::{
    AtomicBool, CollectError, Config, Control, Facts, Failed, Failure, Hooks, Instant,
    InvocationCompletion, IoPhase, Observation, Operation, Ordering, Phase, PreparedInvocation,
    ProcessAdmission, ProcessClaim, ProviderScope, SubmitError, Termination, WorkerSlot,
};
use crate::nts_ke_experiment::process::native_scope_with;
use crate::nts_ke_experiment::{RunFailed, Summary, run_synchronously_with};

/// Finite component watchdog, not a bound on a product/native operation or explicit join.
const WAIT: Duration = Duration::from_secs(2);

/// Complete fixed supplemental callback failure, independent of any TLS/protocol truth.
const fn primary() -> Failed {
    Failed {
        primary: Failure::Entropy,
        cleanup: Some(Failure::Shutdown(std::io::ErrorKind::BrokenPipe)),
        admission_cleanup: Some(Failure::Control),
    }
}

/// Create the existing component provider with the actual late-bound claim lifetime.
/// # Errors
/// This inert adapter always succeeds within the actual worker's fallible factory type.
#[expect(
    clippy::unnecessary_wraps,
    reason = "The shared worker requires its fallible provider-factory signature."
)]
fn component_provider(claim: ProcessClaim<'_>) -> Result<ProviderScope<'_>, Failure> {
    Ok(ProviderScope::component(claim))
}

/// Admit genuine checked input and original Control before submitting a component job.
/// # Errors
/// Preserve actual Config/Control construction refusal; create no substituted owner.
fn prepared() -> Result<(PreparedInvocation, Weak<()>), Failure> {
    let config = Config::new("127.0.0.1:4460".into(), "Time.Example".into())?;
    let witness = Arc::downgrade(&config.lifetime);
    let (prepared, _stop) =
        PreparedInvocation::prepare(config).map_err(|failure| failure.primary)?;
    Ok((prepared, witness))
}

/// Supply a fixed typed component failure only after the original currentness guard.
/// # Errors
/// Retain actual control refusal or the complete selected supplemental failure.
fn component_failure<'a>(
    _config: &'a Config,
    _provider: &'a ProviderScope<'a>,
    control: Control,
) -> Operation {
    control
        .remaining(IoPhase::Handshake)
        .map_err(Failed::from)?;
    drop(control);
    Err(primary())
}

/// Controller-only observation copies a closed failure, never an owning completion.
/// Missing, moved or successful facts supply no failure.
/// # Errors
/// Refuse poisoned observation without healing or consuming the actual payload.
fn recorded_failure(facts: &Facts) -> Result<Option<Failed>, Failure> {
    let Some(cell) = facts.operation.get() else {
        return Ok(None);
    };
    let operation = cell.lock().map_err(|_| Failure::Control)?;
    Ok(operation_failure(operation.as_ref()))
}

/// Borrow only the closed failure fields while preserving any owning completion in place.
fn operation_failure(operation: Option<&Operation>) -> Option<Failed> {
    match operation {
        Some(Err(failure)) => Some(*failure),
        Some(Ok(_)) | None => None,
    }
}

/// Run a fixed exact test in a finite child so the parent never claims global admission.
/// Only child stderr is inherited for causal assertions; its libtest records stay private.
/// # Errors
/// Preserve child setup, unsuccessful status and watchdog/cleanup failures as fixture refusal.
fn isolated_native_case(
    name: &str,
    check: impl FnOnce() -> Result<(), Failure>,
) -> Result<(), Failure> {
    /// Child-only selector; Command sets it without changing the parent environment.
    const CHILD_CASE: &str = "NEVROTICK_PROVIDER_SCOPE_CHILD";
    /// Prove the selected body completed, excluding libtest's zero-test success status.
    const CHILD_COMPLETED: i32 = 86;
    if std::env::var_os(CHILD_CASE).as_deref() == Some(std::ffi::OsStr::new(name)) {
        check()?;
        #[expect(
            clippy::exit,
            reason = "only the isolated test child exits after its body and explicit custody checks finish"
        )]
        std::process::exit(CHILD_COMPLETED);
    }
    let executable = std::env::current_exe().map_err(|_| Failure::Control)?;
    let mut child = std::process::Command::new(executable)
        .args(["--exact", name, "--nocapture", "--test-threads=1"])
        .env(CHILD_CASE, name)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .map_err(|_| Failure::Control)?;
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                return if status.code() == Some(CHILD_COMPLETED) {
                    Ok(())
                } else {
                    Err(Failure::Control)
                };
            }
            Ok(None) if Instant::now() < deadline => {
                std::thread::sleep(Duration::from_millis(10));
            }
            Ok(None) | Err(_) => break,
        }
    }
    // Always attempt reap, including when kill races with natural child exit.
    let killed = child.kill();
    let reaped = child.wait();
    killed.map_err(|_| Failure::Control)?;
    reaped.map_err(|_| Failure::Control)?;
    Err(Failure::Control)
}

/// Two genuine foreign claims must refuse at the Native owner before either inert effect.
/// # Errors
/// Preserve genuine claim admission and finite isolated-child setup failures.
/// # Panics
/// Fail after dropping any unexpectedly accepted scopes if either foreign origin is accepted.
#[test]
fn provider_native_foreign_origins_refuse_before_effect_factory() -> Result<(), Failure> {
    isolated_native_case(
        "nts_ke_experiment::job::tests::provider_native_foreign_origins_refuse_before_effect_factory",
        || {
            let first_admission = ProcessAdmission::new();
            let second_admission = ProcessAdmission::new();
            let first_claim = first_admission.claim()?;
            let second_claim = second_admission.claim()?;
            let effects = AtomicUsize::new(0);
            let first = native_scope_with(first_claim, |claim| {
                effects.fetch_add(1, Ordering::AcqRel);
                ProviderScope::component(claim)
            });
            let second = native_scope_with(second_claim, |claim| {
                effects.fetch_add(1, Ordering::AcqRel);
                ProviderScope::component(claim)
            });
            let first_refused = matches!(&first, Err(Failure::Control));
            let second_refused = matches!(&second, Err(Failure::Control));
            drop(first);
            drop(second);
            assert_eq!(
                effects.load(Ordering::Acquire),
                0,
                "KE-PROV-01-foreign-before-effect-factory"
            );
            assert!(
                first_refused && second_refused,
                "KE-PROV-01-foreign-control-refusal"
            );
            assert!(matches!(first_admission.claim(), Err(Failure::ProcessUsed)));
            assert!(matches!(
                second_admission.claim(),
                Err(Failure::ProcessUsed)
            ));
            Ok(())
        },
    )
}

/// The fixed authoritative latch admits one inert scope and stays spent after its drop.
/// # Errors
/// Preserve real authoritative claim, shared admission and finite child setup failures.
/// # Panics
/// Fail if the genuine origin is refused, its factory repeats or admission reopens.
#[test]
fn provider_native_authoritative_once_preserves_permanent_admission() -> Result<(), Failure> {
    isolated_native_case(
        "nts_ke_experiment::job::tests::provider_native_authoritative_once_preserves_permanent_admission",
        || {
            let admission = &crate::nts_ke_experiment::PROCESS_ADMISSION;
            let claim = admission.claim()?;
            let effects = AtomicUsize::new(0);
            let provider = native_scope_with(claim, |claim| {
                effects.fetch_add(1, Ordering::AcqRel);
                ProviderScope::component(claim)
            })?;
            let lifetime = provider.retirement_witness();
            let refused_while_live = matches!(admission.claim(), Err(Failure::ProcessUsed));
            let foreign_live_admission = ProcessAdmission::new();
            let foreign_live = native_scope_with(foreign_live_admission.claim()?, |claim| {
                effects.fetch_add(1, Ordering::AcqRel);
                ProviderScope::component(claim)
            });
            let foreign_refused_while_live = matches!(&foreign_live, Err(Failure::Control));
            drop(foreign_live);
            drop(provider);
            let refused_after_drop = matches!(admission.claim(), Err(Failure::ProcessUsed));
            let foreign_retired_admission = ProcessAdmission::new();
            let foreign_retired = native_scope_with(foreign_retired_admission.claim()?, |claim| {
                effects.fetch_add(1, Ordering::AcqRel);
                ProviderScope::component(claim)
            });
            let foreign_refused_after_drop = matches!(&foreign_retired, Err(Failure::Control));
            drop(foreign_retired);
            assert!(
                foreign_refused_while_live && foreign_refused_after_drop,
                "KE-PROV-01-origin-is-identity-after-native-claim"
            );
            assert_eq!(
                effects.load(Ordering::Acquire),
                1,
                "KE-PROV-01-authoritative-factory-once"
            );
            assert_eq!(lifetime.strong_count(), 0);
            assert!(
                refused_while_live && refused_after_drop,
                "KE-PROV-01-authoritative-admission-never-reopens"
            );
            Ok(())
        },
    )
}

/// A real whole-job foreign claim must record local refusal before actual Config retirement.
/// Every unexpectedly accepted inert provider/job is released and joined before assertions.
/// # Errors
/// Preserve checked input, actual submission, gate/watchdog and collection failures.
/// # Panics
/// Fail if factory/producer executes, facts change, capacity escapes or Config outlives retirement.
#[test]
fn job_native_foreign_origin_records_control_before_actual_retirement() -> Result<(), Failure> {
    isolated_native_case(
        "nts_ke_experiment::job::tests::job_native_foreign_origin_records_control_before_actual_retirement",
        || {
            /// Permanent actual job custody for this isolated component case.
            static SLOT: WorkerSlot = WorkerSlot::new();
            /// Genuine foreign latch, never an injected expected authoritative origin.
            static ADMISSION: ProcessAdmission = ProcessAdmission::new();
            /// Actual shared-owner factory entries, counted only inside its effect body.
            static EFFECTS: AtomicUsize = AtomicUsize::new(0);
            /// Actual whole-job producer entries, independent of factory observations.
            static PRODUCERS: AtomicUsize = AtomicUsize::new(0);

            /// Reach the same Native admission owner with an inert effect factory.
            /// # Errors
            /// Preserve the actual shared owner's local refusal without producer effects.
            fn observed_provider(claim: ProcessClaim<'_>) -> Result<ProviderScope<'_>, Failure> {
                native_scope_with(claim, |claim| {
                    EFFECTS.fetch_add(1, Ordering::AcqRel);
                    ProviderScope::component(claim)
                })
            }

            let (first, lifetime) = prepared()?;
            let (second, second_lifetime) = prepared()?;
            let (hold, gate) = gate();
            let watchdog = Arc::clone(&gate.failed);
            let mut hooks = Hooks::none();
            hooks.before_retire = Some(hold);
            SLOT.submit_with(
                first,
                &ADMISSION,
                observed_provider,
                |config, provider, control| {
                    PRODUCERS.fetch_add(1, Ordering::AcqRel);
                    component_failure(config, provider, control)
                },
                hooks,
            )
            .map_err(|_| Failure::Control)?;
            let entered = gate.wait();
            let recorded_before = recorded_failure(&SLOT.facts)?;
            let retired_before = SLOT.facts.retired.load(Ordering::Acquire);
            let live_config_before = lifetime.strong_count();
            let observed_before = SLOT.observer().poll_until(Instant::now() + WAIT);
            let withheld = SLOT.take_result();
            let repeated = SLOT.submit_with(
                second,
                &ADMISSION,
                component_provider,
                component_failure,
                Hooks::none(),
            );
            let released = gate.release();
            let withheld_after_release = SLOT.take_result();
            let joined = collect_available(&SLOT);
            let known = SLOT.take_result();
            entered?;
            released?;
            joined.map_err(|_| Failure::Control)?;
            if watchdog.load(Ordering::Acquire) {
                return Err(Failure::Control);
            }
            let outcome = known.map_err(|_| Failure::Control)?;
            assert_eq!(
                EFFECTS.load(Ordering::Acquire),
                0,
                "KE-PROV-03-refusal-before-effect-factory"
            );
            assert_eq!(
                PRODUCERS.load(Ordering::Acquire),
                0,
                "KE-PROV-03-refusal-before-producer"
            );
            let expected = Some(Failed::from(Failure::Control));
            assert_eq!(
                recorded_before, expected,
                "KE-PROV-03-local-primary-before-retirement"
            );
            assert!(!retired_before);
            assert_eq!(live_config_before, 1);
            assert_eq!(observed_before, Observation::Running(false));
            assert!(matches!(withheld, Err(CollectError::NotReady)));
            assert!(matches!(
                withheld_after_release,
                Err(CollectError::NotReady)
            ));
            assert_eq!(repeated, Err(SubmitError::Used));
            assert_eq!(second_lifetime.strong_count(), 0);
            assert!(
                matches!(&outcome.operation, Some(Err(failure)) if Some(*failure) == expected),
                "KE-PROV-03-local-primary-after-join"
            );
            assert_eq!(outcome.termination, Termination::Returned);
            assert!(outcome.retired, "KE-PROV-03-retirement-after-config-drop");
            assert_eq!(lifetime.strong_count(), 0);
            assert!(matches!(ADMISSION.claim(), Err(Failure::ProcessUsed)));
            Ok(())
        },
    )
}

/// Default worker retirement preserves Native-origin refusal without any observation hook.
/// The genuine foreign latch never claims or changes the authoritative process admission.
/// # Errors
/// Preserve checked preparation, actual submission and collection failures.
/// # Panics
/// Assert after actual joins that no inert factory/producer effect or new admission occurred.
#[test]
fn job_native_foreign_origin_retires_without_hooks() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    static EFFECTS: AtomicUsize = AtomicUsize::new(0);
    static PRODUCERS: AtomicUsize = AtomicUsize::new(0);

    /// Reach the actual origin guard while keeping even unexpected acceptance inert.
    /// # Errors
    /// Preserve the shared owner's refusal before its component-only factory executes.
    fn observed_provider(claim: ProcessClaim<'_>) -> Result<ProviderScope<'_>, Failure> {
        native_scope_with(claim, |claim| {
            EFFECTS.fetch_add(1, Ordering::AcqRel);
            ProviderScope::component(claim)
        })
    }

    let (first, lifetime) = prepared()?;
    SLOT.submit_with(
        first,
        &ADMISSION,
        observed_provider,
        |config, provider, control| {
            PRODUCERS.fetch_add(1, Ordering::AcqRel);
            component_failure(config, provider, control)
        },
        Hooks::none(),
    )
    .map_err(|_| Failure::Control)?;
    SLOT.collect().map_err(|_| Failure::Control)?;
    let outcome = SLOT.take_result().map_err(|_| Failure::Control)?;
    let (second, second_lifetime) = prepared()?;
    let repeated = SLOT.submit_with(
        second,
        &ADMISSION,
        component_provider,
        component_failure,
        Hooks::none(),
    );
    if repeated.is_ok() {
        // Retire any unexpectedly accepted inert work before behavioral assertions.
        SLOT.collect().map_err(|_| Failure::Control)?;
        let _unexpected_outcome = SLOT.take_result().map_err(|_| Failure::Control)?;
    }

    assert_eq!(EFFECTS.load(Ordering::Acquire), 0);
    assert_eq!(PRODUCERS.load(Ordering::Acquire), 0);
    assert!(
        matches!(&outcome.operation, Some(Err(failure)) if *failure == Failed::from(Failure::Control)),
        "KE-PROV-03-default-retirement-preserves-local-refusal"
    );
    assert_eq!(outcome.termination, Termination::Returned);
    assert!(
        outcome.retired,
        "KE-PROV-03-default-retirement-actually-completes"
    );
    assert_eq!(lifetime.strong_count(), 0);
    assert_eq!(second_lifetime.strong_count(), 0);
    assert_eq!(repeated, Err(SubmitError::Used));
    assert!(matches!(ADMISSION.claim(), Err(Failure::ProcessUsed)));
    Ok(())
}

/// An actual Linux Builder refusal disposes owned input without a worker or completion.
/// The fixed invalid request is capability-checked in the admitted bounded GNU profile.
/// # Errors
/// Preserve checked preparation and cleanup failures for any unexpectedly accepted inert work.
/// # Panics
/// Assert after actual cleanup that the typed refusal, terminal custody and unused admission survive.
#[cfg(all(target_os = "linux", target_env = "gnu", target_arch = "x86_64"))]
#[test]
fn job_actual_builder_refusal_preserves_unsubmitted_custody() -> Result<(), Failure> {
    isolated_native_case(
        "nts_ke_experiment::job::tests::job_actual_builder_refusal_preserves_unsubmitted_custody",
        || {
            static SLOT: WorkerSlot = WorkerSlot::new();
            static ADMISSION: ProcessAdmission = ProcessAdmission::new();
            static PROVIDERS: AtomicUsize = AtomicUsize::new(0);
            static PRODUCERS: AtomicUsize = AtomicUsize::new(0);

            /// Observe actual provider entry while keeping unexpected worker acceptance inert.
            /// # Errors
            /// Preserve the shared component adapter's typed result.
            fn observed_provider(claim: ProcessClaim<'_>) -> Result<ProviderScope<'_>, Failure> {
                PROVIDERS.fetch_add(1, Ordering::AcqRel);
                component_provider(claim)
            }

            let (first, lifetime) = prepared()?;
            let mut hooks = Hooks::none();
            hooks.stack_request = super::StackRequest::InvalidAligned;
            let submitted = SLOT.submit_with(
                first,
                &ADMISSION,
                observed_provider,
                |config, provider, control| {
                    PRODUCERS.fetch_add(1, Ordering::AcqRel);
                    component_failure(config, provider, control)
                },
                hooks,
            );
            if submitted.is_ok() {
                SLOT.collect().map_err(|_| Failure::Control)?;
                let _unexpected_outcome = SLOT.take_result().map_err(|_| Failure::Control)?;
            }
            let observed = SLOT.observer().poll_until(Instant::now() + WAIT);
            let collected = SLOT.collect();
            let taken = SLOT.take_result();
            let operation_missing = SLOT.facts.operation.get().is_none();
            let retired = SLOT.facts.retired.load(Ordering::Acquire);
            let sealed = SLOT.sealed_count();
            let original_live = lifetime.strong_count();
            let providers = PROVIDERS.load(Ordering::Acquire);
            let producers = PRODUCERS.load(Ordering::Acquire);
            let (second, second_lifetime) = prepared()?;
            let repeated = SLOT.submit_with(
                second,
                &ADMISSION,
                component_provider,
                component_failure,
                Hooks::none(),
            );
            if repeated.is_ok() {
                SLOT.collect().map_err(|_| Failure::Control)?;
                let _unexpected_outcome = SLOT.take_result().map_err(|_| Failure::Control)?;
            }
            // This genuine postcondition probe is a test action, never worker execution.
            // Its temporary claim ends in this block; the permanent latch is not reset.
            let admission_unused = {
                let postcondition_claim = ADMISSION.claim();
                postcondition_claim.is_ok()
            };

            let expected = SubmitError::Spawn(std::io::ErrorKind::WouldBlock);
            assert_eq!(submitted, Err(expected));
            assert_eq!(
                observed,
                Observation::NotSubmitted(expected),
                "KE-JOB-03-real-builder-refusal-terminal-phase"
            );
            assert_eq!(providers, 0);
            assert_eq!(producers, 0);
            assert_eq!(original_live, 0);
            assert!(operation_missing);
            assert!(!retired);
            assert_eq!(sealed, Ok(0));
            assert_eq!(collected, Err(CollectError::NotReady));
            assert!(matches!(taken, Err(CollectError::NotReady)));
            assert_eq!(repeated, Err(SubmitError::Used));
            assert_eq!(second_lifetime.strong_count(), 0);
            assert!(
                admission_unused,
                "KE-JOB-03-real-builder-refusal-unused-admission"
            );
            Ok(())
        },
    )
}

/// Poison this actual slot by unwinding through its acquired guard without field mutation.
/// The caught payload is this helper's fixed unit, never a product unwind payload.
/// # Errors
/// Refuse failed acquisition or absent actual poison before behavioral observations.
fn poison(slot: &WorkerSlot) -> Result<(), Failure> {
    let caught = std::panic::catch_unwind(|| -> Result<(), Failure> {
        let _guard = slot.custody.lock().map_err(|_| Failure::Control)?;
        std::panic::resume_unwind(Box::new(()));
    });
    match caught {
        Err(payload) => {
            drop(payload);
            if slot.custody.is_poisoned() {
                Ok(())
            } else {
                Err(Failure::Control)
            }
        }
        Ok(_) => Err(Failure::Control),
    }
}

/// Poison only the actual recorded payload mutex without replacing or moving its value.
/// The caught payload is this controller's fixed unit, never a product unwind payload.
/// # Errors
/// Refuse missing facts, failed acquisition or absent actual poison.
fn poison_recorded_facts(slot: &WorkerSlot) -> Result<(), Failure> {
    let cell = slot.facts.operation.get().ok_or(Failure::Control)?;
    let caught = std::panic::catch_unwind(|| -> Result<(), Failure> {
        let _guard = cell.lock().map_err(|_| Failure::Control)?;
        std::panic::resume_unwind(Box::new(()));
    });
    match caught {
        Err(payload) => {
            drop(payload);
            if cell.is_poisoned() {
                Ok(())
            } else {
                Err(Failure::Control)
            }
        }
        Ok(_) => Err(Failure::Control),
    }
}

/// Actual held custody refuses all competing APIs, then permits one genuine job.
/// Runtime construction retains the same process-lifetime static owner.
/// # Errors
/// Preserve actual setup, preparation, submission and joined-result failures.
/// # Panics
/// Fail if held custody admits work, terminal collection repeats or admission reopens.
#[test]
fn job_actual_held_custody_refuses_then_preserves_one_terminal_result() -> Result<(), Failure> {
    static SLOT: LazyLock<WorkerSlot> = LazyLock::new(WorkerSlot::new);
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let slot: &'static WorkerSlot = &SLOT;
    let unused = slot.observer().poll_until(Instant::now() + WAIT);
    let unsubmitted_collect = slot.collect();
    let (first, first_lifetime) = prepared()?;
    let held = slot.custody.lock().map_err(|_| Failure::Control)?;
    let busy_submit = slot.submit_with(
        first,
        &ADMISSION,
        component_provider,
        component_failure,
        Hooks::none(),
    );
    let busy_observe = slot.observer().poll_until(Instant::now() + WAIT);
    let busy_collect = slot.collect();
    let busy_take = slot.take_result();
    drop(held);
    let still_unused = slot.observer().poll_until(Instant::now() + WAIT);
    let (next, next_lifetime) = prepared()?;
    slot.submit_with(
        next,
        &ADMISSION,
        component_provider,
        component_failure,
        Hooks::none(),
    )
    .map_err(|_| Failure::Control)?;
    slot.collect().map_err(|_| Failure::Control)?;
    let duplicate_before_take = slot.collect();
    let outcome = slot.take_result().map_err(|_| Failure::Control)?;
    let duplicate_after_take = slot.collect();
    let duplicate_take = slot.take_result();
    let (again, _) = prepared()?;
    let resubmitted = slot.submit_with(
        again,
        &ADMISSION,
        component_provider,
        component_failure,
        Hooks::none(),
    );

    assert_eq!(unused, Observation::Unused);
    assert_eq!(unsubmitted_collect, Err(CollectError::NotReady));
    assert_eq!(
        busy_submit,
        Err(SubmitError::Busy),
        "KE-JOB-03-held-custody-refuses-submit"
    );
    assert_eq!(
        busy_observe,
        Observation::Busy,
        "KE-JOB-04-held-custody-observation-unknown"
    );
    assert_eq!(
        busy_collect,
        Err(CollectError::Busy),
        "KE-JOB-04-held-custody-refuses-collector"
    );
    assert!(
        matches!(busy_take, Err(CollectError::Busy)),
        "KE-JOB-04-held-custody-withholds-result"
    );
    assert_eq!(still_unused, Observation::Unused);
    assert_eq!(first_lifetime.strong_count(), 0);
    assert!(matches!(&outcome.operation, Some(Err(failure)) if *failure == primary()));
    assert_eq!(outcome.termination, Termination::Returned);
    assert!(outcome.retired);
    assert_eq!(next_lifetime.strong_count(), 0);
    assert_eq!(duplicate_before_take, Err(CollectError::NotReady));
    assert_eq!(duplicate_after_take, Err(CollectError::NotReady));
    assert!(matches!(duplicate_take, Err(CollectError::NotReady)));
    assert_eq!(
        resubmitted,
        Err(SubmitError::Used),
        "KE-JOB-03-terminal-slot-never-reopens"
    );
    assert!(matches!(ADMISSION.claim(), Err(Failure::ProcessUsed)));
    // Check the selected Builder request, independently of the OS-granted stack.
    assert_eq!(
        super::STACK_BYTES,
        2_097_152,
        "KE-JOB-01-selected-requested-stack-bytes"
    );
    Ok(())
}

/// Actual guard poison refuses every API without starting or claiming a job.
/// # Errors
/// Preserve real poison setup and checked preparation failures.
/// # Panics
/// Fail if poisoned custody is accepted, healed or represented as available.
#[test]
fn job_actual_unused_custody_poison_refuses_without_process_use() -> Result<(), Failure> {
    static SLOT: LazyLock<WorkerSlot> = LazyLock::new(WorkerSlot::new);
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let slot: &'static WorkerSlot = &SLOT;
    poison(slot)?;
    let (prepared, lifetime) = prepared()?;
    let submitted = slot.submit_with(
        prepared,
        &ADMISSION,
        component_provider,
        component_failure,
        Hooks::none(),
    );
    let observed = slot.observer().poll_until(Instant::now() + WAIT);
    let collected = slot.collect();
    let taken = slot.take_result();
    assert_eq!(
        submitted,
        Err(SubmitError::Control),
        "KE-JOB-03-poisoned-custody-refuses-submit"
    );
    assert_eq!(
        observed,
        Observation::Control,
        "KE-JOB-04-poisoned-observation-refuses"
    );
    assert_eq!(
        collected,
        Err(CollectError::Control),
        "KE-JOB-04-poisoned-collection-refuses"
    );
    assert!(
        matches!(taken, Err(CollectError::Control)),
        "KE-JOB-04-poisoned-result-refuses"
    );
    assert!(slot.custody.is_poisoned());
    assert_eq!(lifetime.strong_count(), 0);
    assert!(ADMISSION.claim().is_ok());
    Ok(())
}

/// The real once-written fact cell rejects replacement and retains every original cause.
/// # Errors
/// Preserve unexpected first-record refusal.
/// # Panics
/// Fail if duplicate recording overwrites facts or asserts retirement.
#[test]
fn job_actual_fact_cell_duplicate_preserves_complete_original_failure() -> Result<(), Failure> {
    let facts = Facts::new();
    facts.record(Err(primary()))?;
    let duplicate = facts.record(Err(Failed::from(Failure::Cancelled)));
    assert_eq!(
        duplicate,
        Err(Failure::Control),
        "KE-JOB-05-facts-record-once"
    );
    assert_eq!(
        recorded_failure(&facts)?,
        Some(primary()),
        "KE-JOB-05-duplicate-facts-preserve-original"
    );
    assert!(!facts.retired.load(Ordering::Acquire));
    Ok(())
}

/// One actual worker moves its preview through join while its fact latch stays permanently set.
/// Preview counts create no protected history or authentication fact.
/// # Errors
/// Preserve checked input, finite callback setup/release and actual collection failures.
/// # Panics
/// Fail if pre-join take succeeds, preview changes, a second take succeeds or recording reopens.
#[test]
fn job_actual_preview_moves_once_and_record_after_take_refuses() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let (invocation, lifetime) = prepared()?;
    let expected = Summary {
        cookies: 1,
        cookie_bytes: 12,
        packet_bytes: 128,
        transmitted: false,
    };
    let (hold, gate) = gate();
    let watchdog = Arc::clone(&gate.failed);
    let mut hooks = Hooks::none();
    hooks.before_retire = Some(hold);
    SLOT.submit_with(
        invocation,
        &ADMISSION,
        component_provider,
        move |_config, _provider, control| {
            control
                .remaining(IoPhase::Handshake)
                .map_err(Failed::from)?;
            drop(control);
            Ok(InvocationCompletion::Preview(expected))
        },
        hooks,
    )
    .map_err(|_| Failure::Control)?;
    let entered = gate.wait();
    let premature = SLOT.take_result();
    let recorded_before = SLOT.facts.operation.get().is_some();
    let retired_before = SLOT.facts.retired.load(Ordering::Acquire);
    let released = gate.release();
    let collected = collect_available(&SLOT);
    entered?;
    released?;
    collected.map_err(|_| Failure::Control)?;
    if watchdog.load(Ordering::Acquire) {
        return Err(Failure::Control);
    }
    let outcome = SLOT.take_result().map_err(|_| Failure::Control)?;
    let duplicate = SLOT.take_result();
    let record_after_take = SLOT.facts.record(Err(Failed::from(Failure::Cancelled)));
    let cell = SLOT.facts.operation.get().ok_or(Failure::Control)?;
    let payload_moved = cell.lock().map_err(|_| Failure::Control)?.is_none();

    assert!(
        matches!(premature, Err(CollectError::NotReady)),
        "KE-HIST-03-prejoin-take-refuses"
    );
    assert!(recorded_before && !retired_before);
    assert!(
        matches!(&outcome.operation, Some(Ok(InvocationCompletion::Preview(summary))) if *summary == expected),
        "KE-HIST-02-preview-retains-counts-without-protected-history"
    );
    assert_eq!(outcome.termination, Termination::Returned);
    assert!(outcome.retired);
    assert_eq!(lifetime.strong_count(), 0);
    assert!(
        matches!(duplicate, Err(CollectError::NotReady)),
        "KE-HIST-03-duplicate-take-refuses"
    );
    assert_eq!(
        record_after_take,
        Err(Failure::Control),
        "KE-HIST-03-record-after-take-refuses"
    );
    assert!(
        payload_moved && !cell.is_poisoned(),
        "KE-HIST-03-outer-latch-retains-empty-payload"
    );
    assert_eq!(
        SLOT.observer().poll_until(Instant::now() + WAIT),
        Observation::Taken
    );
    assert!(matches!(ADMISSION.claim(), Err(Failure::ProcessUsed)));
    Ok(())
}

/// Inner fact poison preserves a real worker's complete failure but forbids normal return.
/// # Errors
/// Preserve genuine preparation, callback setup/release, poison and actual collection failures.
/// # Panics
/// Assert after actual join that original facts survive without healing poison or reopening.
#[test]
fn job_actual_inner_fact_poison_preserves_failure_and_refuses_normal_claim() -> Result<(), Failure>
{
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let (invocation, lifetime) = prepared()?;
    let (hold, gate) = gate();
    let watchdog = Arc::clone(&gate.failed);
    let mut hooks = Hooks::none();
    hooks.before_retire = Some(hold);
    SLOT.submit_with(
        invocation,
        &ADMISSION,
        component_provider,
        component_failure,
        hooks,
    )
    .map_err(|_| Failure::Control)?;
    let entered = gate.wait();
    let poisoned = if entered.is_ok() {
        poison_recorded_facts(&SLOT)
    } else {
        Err(Failure::Control)
    };
    let premature = SLOT.take_result();
    let released = gate.release();
    let collected = collect_available(&SLOT);
    entered?;
    released?;
    poisoned?;
    collected.map_err(|_| Failure::Control)?;
    if watchdog.load(Ordering::Acquire) {
        return Err(Failure::Control);
    }
    let outcome = SLOT.take_result().map_err(|_| Failure::Control)?;
    let duplicate = SLOT.take_result();
    let record_after_take = SLOT.facts.record(Err(Failed::from(Failure::Cancelled)));
    let cell = SLOT.facts.operation.get().ok_or(Failure::Control)?;
    let Err(poisoned) = cell.lock() else {
        return Err(Failure::Control);
    };
    let payload_moved = poisoned.into_inner().is_none();

    assert!(matches!(premature, Err(CollectError::NotReady)));
    assert!(
        matches!(&outcome.operation, Some(Err(failure)) if *failure == primary()),
        "KE-HIST-05-inner-poison-preserves-original-failure"
    );
    assert_eq!(
        outcome.termination,
        Termination::Contract(Failure::Control),
        "KE-HIST-05-inner-poison-is-not-normal"
    );
    assert!(outcome.retired && payload_moved);
    assert!(
        cell.is_poisoned() && !SLOT.custody.is_poisoned(),
        "KE-HIST-05-inner-poison-never-healed"
    );
    assert_eq!(SLOT.sealed_count().map_err(|_| Failure::Control)?, 0);
    assert!(matches!(duplicate, Err(CollectError::NotReady)));
    assert_eq!(record_after_take, Err(Failure::Control));
    assert_eq!(lifetime.strong_count(), 0);
    assert!(matches!(ADMISSION.claim(), Err(Failure::ProcessUsed)));
    Ok(())
}

/// A preexisting internal fact must not become a normal-return or retirement acknowledgement.
/// This synthetic invariant fault reaches the actual submitted worker and collector.
/// # Errors
/// Preserve checked preparation, real submission, collection and first-record failures.
/// # Panics
/// Assert after actual join that the original fact survives and real owned resources dropped.
#[test]
fn job_actual_duplicate_record_preserves_joined_contract() -> Result<(), Failure> {
    static SLOT: LazyLock<WorkerSlot> = LazyLock::new(WorkerSlot::new);
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let slot: &'static WorkerSlot = &SLOT;
    let original = Failed {
        primary: Failure::Cancelled,
        ..primary()
    };
    slot.facts.record(Err(original))?;
    let (prepared, config_lifetime) = prepared()?;
    let provider_lifetime = Arc::new(OnceLock::<Weak<()>>::new());
    let observed_provider = Arc::clone(&provider_lifetime);
    let produced_operation = Arc::new(OnceLock::<Failed>::new());
    let observed_operation = Arc::clone(&produced_operation);
    slot.submit_with(
        prepared,
        &ADMISSION,
        component_provider,
        move |config, provider, control| {
            observed_provider
                .set(provider.retirement_witness())
                .map_err(|_| Failed::from(Failure::Control))?;
            let operation = component_failure(config, provider, control);
            let Err(observed_failure) = &operation else {
                return Err(Failed::from(Failure::Control));
            };
            observed_operation
                .set(*observed_failure)
                .map_err(|_| Failed::from(Failure::Control))?;
            operation
        },
        Hooks::none(),
    )
    .map_err(|_| Failure::Control)?;
    slot.collect().map_err(|_| Failure::Control)?;
    let outcome = slot.take_result().map_err(|_| Failure::Control)?;

    assert_eq!(produced_operation.get().copied(), Some(primary()));
    assert!(
        matches!(&outcome.operation, Some(Err(failure)) if *failure == original),
        "KE-JOB-05-worker-duplicate-preserves-original"
    );
    assert_eq!(
        outcome.termination,
        Termination::Contract(Failure::Control),
        "KE-JOB-05-actual-join-retains-record-refusal"
    );
    assert!(
        !outcome.retired,
        "KE-JOB-05-record-refusal-cannot-acknowledge-retirement"
    );
    assert_eq!(config_lifetime.strong_count(), 0);
    assert_eq!(provider_lifetime.get().map(Weak::strong_count), Some(0));
    assert!(matches!(ADMISSION.claim(), Err(Failure::ProcessUsed)));
    Ok(())
}

/// Controller of a real worker retirement callback, with nonblocking early-exit release.
struct Gate {
    /// Actual callback-entry observation; not a completion receipt.
    entered: Receiver<()>,
    /// One bounded release message, never a capacity/completion acknowledgement.
    release: SyncSender<()>,
    /// At most one release attempt by the actual controller.
    released: bool,
    /// Callback watchdog failures are setup failures, never product behavioral success.
    failed: Arc<AtomicBool>,
}

impl Gate {
    /// Require actual callback entry within its finite setup watchdog.
    /// # Errors
    /// Preserve missing entry as setup failure, without behavioral assertions.
    fn wait(&self) -> Result<(), Failure> {
        self.entered
            .recv_timeout(WAIT)
            .map_err(|_| Failure::Control)
    }

    /// Release the held actual callback without claiming its return or join.
    /// # Errors
    /// Refuse an unavailable release channel.
    fn release(mut self) -> Result<(), Failure> {
        self.released = true;
        self.release.try_send(()).map_err(|_| Failure::Control)
    }
}

impl Drop for Gate {
    /// Nonblocking cleanup precedes implicit scoped collector join on error/unwind.
    fn drop(&mut self) {
        if !self.released {
            self.released = true;
            let _release_attempt = self.release.try_send(());
        }
    }
}

/// Hold a genuine before-retirement hook without panicking on watchdog failure.
fn gate() -> (Box<dyn FnOnce() + Send>, Gate) {
    let (entered_tx, entered_rx) = sync_channel(1);
    let (release_tx, release_rx) = sync_channel(1);
    let failed = Arc::new(AtomicBool::new(false));
    let callback_failed = Arc::clone(&failed);
    let hold = Box::new(move || {
        if entered_tx.try_send(()).is_err() {
            callback_failed.store(true, Ordering::Release);
        }
        if release_rx.recv_timeout(WAIT).is_err() {
            callback_failed.store(true, Ordering::Release);
        }
    });
    (
        hold,
        Gate {
            entered: entered_rx,
            release: release_tx,
            released: false,
            failed,
        },
    )
}

/// Retry temporary observer contention only, without renewing a job or its Control.
/// # Errors
/// Preserve every non-Busy refusal and a Busy result after the existing watchdog.
fn collect_available(slot: &'static WorkerSlot) -> Result<(), CollectError> {
    let deadline = Instant::now() + WAIT;
    loop {
        match slot.collect() {
            Err(CollectError::Busy) if Instant::now() < deadline => std::thread::yield_now(),
            result => return result,
        }
    }
}

/// Require the real collector's installed Joining state before injecting custody poison.
/// # Errors
/// Refuse absent/late setup; never write a desired phase or fake a handle.
fn joining(slot: &'static WorkerSlot) -> Result<(), Failure> {
    let deadline = Instant::now() + Duration::from_secs(1);
    loop {
        match slot.observer().poll_until(deadline) {
            Observation::Joining => return Ok(()),
            Observation::Running(_) | Observation::Busy => std::thread::yield_now(),
            _ => return Err(Failure::Control),
        }
    }
}

/// Actual post-join poison retains the primary while refusing a normal termination claim.
/// # Errors
/// Preserve actual gated-worker, retained collector, poison setup and release failures.
/// # Panics
/// Assert only after release/join and successful setup watchdog checks.
#[test]
fn job_actual_postjoin_poison_preserves_primary_and_refuses_normal_claim() -> Result<(), Failure> {
    static SLOT: LazyLock<WorkerSlot> = LazyLock::new(WorkerSlot::new);
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let slot: &'static WorkerSlot = &SLOT;
    std::thread::scope(|threads| -> Result<(), Failure> {
        let (hold, gate) = gate();
        let watchdog = Arc::clone(&gate.failed);
        let (prepared, lifetime) = prepared()?;
        let mut hooks = Hooks::none();
        hooks.before_retire = Some(hold);
        slot.submit_with(
            prepared,
            &ADMISSION,
            component_provider,
            component_failure,
            hooks,
        )
        .map_err(|_| Failure::Control)?;
        let collector = threads.spawn(move || collect_available(slot));
        gate.wait()?;
        joining(slot)?;
        let competing = slot.collect();
        let withheld = slot.take_result();
        poison(slot)?;
        let released = gate.release();
        let collected = collector.join().map_err(|_| Failure::Control)?;
        released?;
        if watchdog.load(Ordering::Acquire) {
            return Err(Failure::Control);
        }
        let custody = match slot.custody.lock() {
            Err(poisoned) => poisoned.into_inner(),
            Ok(_) => return Err(Failure::Control),
        };
        let (operation, termination, retired) = match &custody.phase {
            Phase::Joined(outcome) => (
                operation_failure(outcome.operation.as_ref()),
                outcome.termination,
                outcome.retired,
            ),
            _ => return Err(Failure::Control),
        };
        let sealed_count = usize::from(custody.sealed_panic.is_some());
        drop(custody);
        let refused_take = slot.take_result();
        assert_eq!(
            competing,
            Err(CollectError::Busy),
            "KE-JOB-04-competing-collector-retains-joining"
        );
        assert!(matches!(withheld, Err(CollectError::NotReady)));
        assert_eq!(collected, Ok(()));
        assert_eq!(
            operation,
            Some(primary()),
            "KE-JOB-05-postjoin-poison-preserves-primary"
        );
        assert_eq!(
            termination,
            Termination::Contract(Failure::Control),
            "KE-JOB-05-postjoin-poison-is-not-normal"
        );
        assert!(retired);
        assert_eq!(sealed_count, 0);
        assert_eq!(lifetime.strong_count(), 0);
        assert!(matches!(refused_take, Err(CollectError::Control)));
        assert!(slot.custody.is_poisoned());
        Ok(())
    })
}

/// Fixed opaque product-job payload used only to observe sealed destructor custody.
struct Payload {
    /// The permanent slot must prevent this destructor from running during collection.
    drops: &'static AtomicUsize,
}

impl Drop for Payload {
    /// Count destruction without output; expected zero while the static slot retains it.
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::AcqRel);
    }
}

/// Actual post-join poison cannot discard a worker panic or its already-recorded primary.
/// # Errors
/// Preserve actual worker/collector, poison setup and finite release failures.
/// # Panics
/// Assert only after actual join and successful watchdog setup; never inspect the payload.
#[test]
fn job_actual_postjoin_poison_seals_panic_and_preserves_recorded_failure() -> Result<(), Failure> {
    static SLOT: LazyLock<WorkerSlot> = LazyLock::new(WorkerSlot::new);
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    static DROPS: AtomicUsize = AtomicUsize::new(0);
    let slot: &'static WorkerSlot = &SLOT;
    std::thread::scope(|threads| -> Result<(), Failure> {
        let (hold, gate) = gate();
        let watchdog = Arc::clone(&gate.failed);
        let (prepared, lifetime) = prepared()?;
        let mut hooks = Hooks::none();
        hooks.before_retire = Some(Box::new(move || {
            hold();
            std::panic::resume_unwind(Box::new(Payload { drops: &DROPS }));
        }));
        slot.submit_with(
            prepared,
            &ADMISSION,
            component_provider,
            component_failure,
            hooks,
        )
        .map_err(|_| Failure::Control)?;
        let collector = threads.spawn(move || collect_available(slot));
        gate.wait()?;
        joining(slot)?;
        let competing = slot.collect();
        let withheld = slot.take_result();
        poison(slot)?;
        let released = gate.release();
        let collected = collector.join().map_err(|_| Failure::Control)?;
        released?;
        if watchdog.load(Ordering::Acquire) {
            return Err(Failure::Control);
        }
        let custody = match slot.custody.lock() {
            Err(poisoned) => poisoned.into_inner(),
            Ok(_) => return Err(Failure::Control),
        };
        let (operation, termination, retired) = match &custody.phase {
            Phase::Joined(outcome) => (
                operation_failure(outcome.operation.as_ref()),
                outcome.termination,
                outcome.retired,
            ),
            _ => return Err(Failure::Control),
        };
        let sealed_count = usize::from(custody.sealed_panic.is_some());
        drop(custody);
        let refused_take = slot.take_result();
        assert_eq!(competing, Err(CollectError::Busy));
        assert!(matches!(withheld, Err(CollectError::NotReady)));
        assert_eq!(collected, Ok(()));
        assert_eq!(
            operation,
            Some(primary()),
            "KE-JOB-05-poisoned-panic-preserves-primary"
        );
        assert_eq!(
            termination,
            Termination::Panicked,
            "KE-JOB-05-poisoned-join-retains-panic"
        );
        assert!(!retired);
        assert_eq!(sealed_count, 1, "KE-JOB-05-poisoned-join-seals-payload");
        assert_eq!(DROPS.load(Ordering::Acquire), 0);
        assert_eq!(lifetime.strong_count(), 0);
        assert!(matches!(refused_take, Err(CollectError::Control)));
        assert!(slot.custody.is_poisoned());
        Ok(())
    })
}

/// A genuine submitted facade job remains owned when real custody contention refuses collection.
/// # Errors
/// Preserve checked setup, actual retirement watchdog, release and joined-result failures.
/// # Panics
/// Release the actual guard/gate and join admitted work before behavioral assertions.
#[test]
fn synchronous_facade_collection_contention_retains_unconfirmed_actual_job() -> Result<(), Failure>
{
    static SLOT: LazyLock<WorkerSlot> = LazyLock::new(WorkerSlot::new);
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let slot: &'static WorkerSlot = &SLOT;
    let config = Config::new("127.0.0.1:4460".into(), "Time.Example".into())?;
    let config_lifetime = Arc::downgrade(&config.lifetime);
    let (second, _) = prepared()?;
    let provider_lifetime = Arc::new(OnceLock::<Weak<()>>::new());
    let observed_provider = Arc::clone(&provider_lifetime);
    let (hold, gate) = gate();
    let watchdog = Arc::clone(&gate.failed);
    let mut hooks = Hooks::none();
    hooks.before_retire = Some(hold);
    let mut custody_guard = None;
    let mut setup = Ok(());
    let facade = run_synchronously_with(config, slot, None, |actual_slot, prepared| {
        actual_slot.submit_with(
            prepared,
            &ADMISSION,
            component_provider,
            move |config, provider, control| {
                observed_provider
                    .set(provider.retirement_witness())
                    .map_err(|_| Failed::from(Failure::Control))?;
                component_failure(config, provider, control)
            },
            hooks,
        )?;
        // Submission already succeeded. Subsequent fixture failures are retained as
        // setup facts; they must not be misreported as a NotSubmitted disposition.
        setup = gate.wait();
        if setup.is_ok() {
            match actual_slot.custody.try_lock() {
                Ok(guard) => custody_guard = Some(guard),
                Err(_) => setup = Err(Failure::Control),
            }
        }
        Ok(())
    });
    let live_config = config_lifetime.strong_count();
    let live_provider = provider_lifetime.get().map(Weak::strong_count);
    let retained_running_handle = custody_guard
        .as_ref()
        .is_some_and(|guard| matches!(&guard.phase, Phase::Running(_)));
    let observation = slot.observer().poll_until(Instant::now() + WAIT);
    let second_submission = slot.submit_with(
        second,
        &ADMISSION,
        component_provider,
        component_failure,
        Hooks::none(),
    );
    drop(custody_guard);
    let released = gate.release();
    let state_after_release = slot.observer().poll_until(Instant::now() + WAIT);
    let joined = match state_after_release {
        Observation::Running(_) => collect_available(slot),
        Observation::Joined | Observation::Taken => Ok(()),
        _ => Err(CollectError::Control),
    };
    let known = slot.take_result();
    released?;
    setup?;
    if watchdog.load(Ordering::Acquire) {
        return Err(Failure::Control);
    }
    assert!(
        matches!(facade, Err(RunFailed::Collection(CollectError::Busy))),
        "KE-FACADE-01-collection-refusal-is-unconfirmed"
    );
    assert!(
        retained_running_handle,
        "KE-FACADE-01-collection-refusal-retains-actual-handle"
    );
    assert_eq!((live_config, live_provider), (1, Some(1)));
    assert_eq!(observation, Observation::Busy);
    assert_eq!(second_submission, Err(SubmitError::Busy));
    assert_eq!(joined, Ok(()));
    let outcome = known.map_err(|_| Failure::Control)?;
    assert!(matches!(&outcome.operation, Some(Err(failure)) if *failure == primary()));
    assert_eq!(outcome.termination, Termination::Returned);
    assert!(outcome.retired);
    assert_eq!(config_lifetime.strong_count(), 0);
    assert_eq!(provider_lifetime.get().map(Weak::strong_count), Some(0));
    assert!(matches!(ADMISSION.claim(), Err(Failure::ProcessUsed)));
    Ok(())
}

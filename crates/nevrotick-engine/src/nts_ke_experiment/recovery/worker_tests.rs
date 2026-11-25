// SPDX-License-Identifier: GPL-2.0-only
//! Whole-call controls reuse the actual Script producer, receiving path and static slot.

use std::cell::{Cell, RefCell};
#[cfg(any(windows, target_os = "linux"))]
use std::net::Ipv4Addr;
use std::num::NonZeroU32;
use std::sync::OnceLock;
use std::sync::atomic::AtomicUsize;

#[cfg(windows)]
use windows_platform_native::{
    ClockCoordinates, ClockReadError, LeapPresentation, LeapPresentationError,
    read_clock_coordinates,
};

/// Actual joined protected-history adaptation, with independent numerical expectations.
#[path = "numerics_tests.rs"]
mod numerics_tests;

use super::{
    Arc, AtomicBool, Config, Control, Duration, ExchangeCase, ExchangeMode, Failed, Failure,
    Instant, Ordering, ProcessAdmission, ProviderScope, Receiver, ScriptProducer, StopDisposition,
    StopHandle, SyncSender, Weak, failed, runtime, sync_channel,
};
#[cfg(windows)]
use crate::ProtectedNativeClockObservation;
use crate::nts_ke_experiment::association::ledger::AppliedProtectedControl;
#[cfg(any(windows, target_os = "linux"))]
use crate::nts_ke_experiment::datagram::tests::{LoopbackPeerLeg, serve_managed_loopback};
use crate::nts_ke_experiment::datagram::{
    ReceivedInvocation, receive_joined, tests::managed_attempt_config,
};
use crate::nts_ke_experiment::job::{
    CollectError, Hooks, JoinedOutcome, Observation, Operation, PreparedInvocation,
    SubmissionReservation, SubmitError, Termination, WorkerSlot,
};
#[cfg(any(windows, target_os = "linux"))]
use crate::nts_ke_experiment::managed_preview::{
    ManagedPowerObservation, ManagedProtectedPairReport,
};
#[cfg(any(windows, target_os = "linux"))]
use crate::nts_ke_experiment::power_target::{PowerEvent, PowerTarget};
use crate::nts_ke_experiment::process::ProcessClaim;
#[cfg(any(windows, target_os = "linux"))]
use crate::nts_ke_experiment::profile::CertificateTimeProvenance;
use crate::nts_ke_experiment::{
    InvocationCompletion, ObservedInvocation, ObserverStartError, ProtectedControlKind, RunFailed,
    Summary, drive_invocation_with, drive_successor_fixture, output, run_joined_with,
    run_observing, run_observing_with_cutoff, run_reserved_observing, run_synchronously_with,
    write_observed_run,
};
#[cfg(any(windows, target_os = "linux"))]
use crate::{
    ConditionalQuality, DeclaredFrequencyInterval, DeclaredQualityAssumptions,
    DeclaredQualityLimits, ExperimentalMeasurement, ExperimentalProfile, ExperimentalReferenceNs,
    MeasurementRefusal, QualityRefusal, RelativeMeasurementRefusal, SourceScale,
};
use crate::{
    ManagedProtectedReport, RunClockContext, SoftwareClock, SourceProtectedProbeFailure,
    SourceProtectedProbeReport,
};

/// Finite component-handshake allowance, not a product/native-operation deadline.
const WAIT: Duration = Duration::from_secs(2);

/// Adapt the existing component constructor with the actual late-bound claim lifetime.
/// This creates only its genuine test scope and consumes the original checked claim.
/// # Errors
/// This inert adapter always succeeds within the actual worker's fallible factory type.
#[expect(
    clippy::unnecessary_wraps,
    reason = "The shared worker requires its fallible provider-factory signature."
)]
fn component_provider(claim: ProcessClaim<'_>) -> Result<ProviderScope<'_>, Failure> {
    Ok(ProviderScope::component(claim))
}

/// Actual checked input with independently retained stop and non-owning lifetime observation.
struct Input {
    /// Actual immutable Config and original checked Control, not replacements.
    prepared: PreparedInvocation,
    /// Stop endpoint derived before submission from that original scope.
    stop: StopHandle,
    /// Weak witness attached by the actual checked Config constructor.
    config_lifetime: Weak<()>,
}

/// Admit fixed loopback arguments through the real constructor before job preparation.
/// # Errors
/// Preserve actual argument/Control admission failure.
fn input(mode: &ExchangeMode) -> Result<Input, Failure> {
    let mut arguments: Vec<std::ffi::OsString> =
        vec!["127.0.0.1:4460".into(), "Time.Example".into()];
    if matches!(mode, ExchangeMode::ProtectedDatagram) {
        arguments.push("--protected-datagram".into());
    }
    let config: Config = Config::from_arguments(arguments.into_iter())?;
    let config_lifetime: Weak<()> = Arc::downgrade(&config.lifetime);
    let (prepared, stop): (PreparedInvocation, StopHandle) =
        PreparedInvocation::prepare(config).map_err(|failure: Failed| failure.primary)?;
    Ok(Input {
        prepared,
        stop,
        config_lifetime,
    })
}

/// Outside controller of one capacity-one component callback handshake.
struct Gate {
    /// Receipt proves callback entry, never operation completion.
    entered: Receiver<()>,
    /// One release lets the actual callback return.
    release: SyncSender<()>,
    /// This controller makes at most one release attempt, including early-exit cleanup.
    released: bool,
}

impl Gate {
    /// Observe actual entry within the independent component watchdog.
    /// # Errors
    /// Refuse missing entry rather than accepting a timeout as application behavior.
    /// # Panics
    /// Fail at the actual installed-state assertion before a missing worker can time out.
    fn wait(&self, slot: &'static WorkerSlot) -> Result<(), Failure> {
        assert_eq!(
            slot.observer().poll_until(Instant::now() + WAIT),
            Observation::Running(false),
            "KE-JOB-03-installed-running-before-barrier"
        );
        self.entered
            .recv_timeout(WAIT)
            .map_err(|_| Failure::Control)?;
        assert_eq!(
            slot.observer().poll_until(Instant::now() + WAIT),
            Observation::Running(false),
            "KE-JOB-04-running-at-actual-barrier"
        );
        Ok(())
    }

    /// Release the held callback once without claiming any subsequent retirement.
    /// # Errors
    /// Refuse an already-lost callback receiver.
    fn release(mut self) -> Result<(), Failure> {
        self.released = true;
        self.release.try_send(()).map_err(|_| Failure::Control)
    }
}

impl Drop for Gate {
    /// Release an owned callback nonblockingly on assertion failure or early return.
    /// Full/disconnected cleanup is not a completion acknowledgement; the actual static
    /// slot retains its worker and any eventual joined result.
    fn drop(&mut self) {
        if !self.released {
            self.released = true;
            let _release_attempt = self.release.try_send(());
        }
    }
}

/// Construct only bounded channel endpoints; no application operation is simulated here.
fn producer_gate() -> ((SyncSender<()>, Receiver<()>), Gate) {
    let (entered_tx, entered_rx) = sync_channel(1);
    let (release_tx, release_rx) = sync_channel(1);
    (
        (entered_tx, release_rx),
        Gate {
            entered: entered_rx,
            release: release_tx,
            released: false,
        },
    )
}

/// Hold an actual whole-owner seam outside all authority/custody gates.
/// # Panics
/// Fail on missing release; its finite timeout prevents a broken control from hanging.
fn owner_gate() -> (Box<dyn FnOnce() + Send>, Gate) {
    let ((entered, release), controller) = producer_gate();
    (
        Box::new(move || {
            assert!(entered.send(()).is_ok());
            assert!(release.recv_timeout(WAIT).is_ok());
        }),
        controller,
    )
}

/// Retry only temporary fixture-observer contention within its existing watchdog.
/// # Errors
/// Preserve every non-Busy refusal and a Busy result after the watchdog expires.
fn collect_available(slot: &'static WorkerSlot) -> Result<(), CollectError> {
    let deadline = Instant::now() + WAIT;
    loop {
        match slot.collect() {
            Err(CollectError::Busy) if Instant::now() < deadline => std::thread::yield_now(),
            result => return result,
        }
    }
}

/// Obtain actual joined facts once, retaining sealed payload custody across consumption.
/// # Errors
/// Preserve actual collection/take refusal as a component failure.
/// # Panics
/// Fail if early hints replace join or a second result can be consumed.
fn collect(slot: &'static WorkerSlot) -> Result<JoinedOutcome, Failure> {
    collect_available(slot).map_err(|_| Failure::Control)?;
    assert_eq!(
        slot.observer().poll_until(Instant::now() + WAIT),
        Observation::Joined
    );
    let outcome = slot.take_fixture_result().map_err(|_| Failure::Control)?;
    assert!(matches!(
        slot.take_fixture_result(),
        Err(CollectError::NotReady)
    ));
    assert_eq!(
        slot.observer().poll_until(Instant::now() + WAIT),
        Observation::Taken
    );
    Ok(outcome)
}

/// Original stop is available while actual startup is held and before permanent admission.
/// # Errors
/// Preserve real preparation, handshake, stop and joined-result failures.
/// # Panics
/// Fail if startup creates a new Control, enters Producer or spends process admission.
#[test]
fn whole_job_prestart_stop_preserves_original_control_and_no_producer() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let Input {
        prepared,
        stop,
        config_lifetime,
    } = input(&ExchangeMode::Preview)?;
    let original = prepared.original_times();
    let entered_producer = Arc::new(AtomicBool::new(false));
    let entered = Arc::clone(&entered_producer);
    let (start, gate) = owner_gate();
    let mut hooks = Hooks::none();
    hooks.before_start = Some(start);
    SLOT.submit_fixture(
        prepared,
        &ADMISSION,
        component_provider,
        move |config, provider, control| {
            entered.store(true, Ordering::Release);
            assert_eq!(
                (control.origin, control.handshake_deadline, control.expiry),
                original
            );
            let mut producer =
                ScriptProducer::new(config, provider, vec![Ok(())], ExchangeCase::Healthy);
            drive_invocation_with(config, provider, control, &mut producer)
        },
        hooks,
    )
    .map_err(|_| Failure::Control)?;
    gate.wait(&SLOT)?;
    let stopped = stop.request_stop()?;
    let pending = SLOT.take_fixture_result();
    gate.release()?;
    let outcome = collect(&SLOT)?;
    assert_eq!(stopped, StopDisposition::Latched);
    assert!(matches!(pending, Err(CollectError::NotReady)));
    assert!(
        matches!(&outcome.operation, Some(Err(actual)) if *actual == failed(Failure::Cancelled))
    );
    assert_eq!(outcome.termination, Termination::Returned);
    assert!(outcome.retired && !entered_producer.load(Ordering::Acquire));
    assert_eq!(config_lifetime.strong_count(), 0);
    assert!(
        ADMISSION.claim().is_ok(),
        "pre-start stop must precede the permanent claim"
    );
    Ok(())
}

/// A real checked expired Control reaches startup without any renewed origin or claim.
/// # Errors
/// Preserve checked fixture construction, spawning and actual joined return.
/// # Panics
/// Fail if expired startup reaches Producer or claims process use.
#[test]
fn whole_job_expired_original_refuses_actual_startup() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let config = Config::new("127.0.0.1:4460".into(), "Time.Example".into())?;
    let origin = Instant::now().checked_sub(WAIT).ok_or(Failure::Deadline)?;
    let control = Control::clamped(origin, origin + Duration::from_millis(1))?;
    let prepared = PreparedInvocation::checked_fixture(config, control);
    let entered = Arc::new(AtomicBool::new(false));
    let observed = Arc::clone(&entered);
    SLOT.submit_fixture(
        prepared,
        &ADMISSION,
        component_provider,
        move |config, provider, control| {
            observed.store(true, Ordering::Release);
            let mut producer =
                ScriptProducer::new(config, provider, vec![Ok(())], ExchangeCase::Healthy);
            drive_invocation_with(config, provider, control, &mut producer)
        },
        Hooks::none(),
    )
    .map_err(|_| Failure::Control)?;
    let outcome = collect(&SLOT)?;
    assert!(
        matches!(&outcome.operation, Some(Err(actual)) if *actual == failed(Failure::Deadline)),
        "KE-JOB-02-original-startup-deadline"
    );
    assert!(outcome.retired && !entered.load(Ordering::Acquire));
    assert!(ADMISSION.claim().is_ok());
    Ok(())
}

/// Abandoned observation cannot release a held actual Producer or its permanent slot.
/// Entered Tokio collection refuses without moving its real handle.
/// # Errors
/// Preserve actual callback, runtime, stop and whole-call collection failures.
/// # Panics
/// Fail if timeout/drop/stop detach work, lose secondary facts or admit a second job.
#[test]
fn whole_job_callback_abandonment_keeps_custody_and_failure_facts() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let Input {
        prepared,
        stop,
        config_lifetime,
    } = input(&ExchangeMode::Preview)?;
    let original = prepared.original_times();
    let primary = Failed {
        primary: Failure::Connect(std::io::ErrorKind::TimedOut),
        cleanup: Some(Failure::Shutdown(std::io::ErrorKind::BrokenPipe)),
        admission_cleanup: Some(Failure::Control),
    };
    let (producer_hold, gate) = producer_gate();
    SLOT.submit_fixture(
        prepared,
        &ADMISSION,
        component_provider,
        move |config, provider, control| {
            assert_eq!(
                (control.origin, control.handshake_deadline, control.expiry),
                original
            );
            let mut producer =
                ScriptProducer::new(config, provider, vec![Err(primary)], ExchangeCase::Healthy);
            producer.establish_gate = Some(producer_hold);
            let result = drive_invocation_with(config, provider, control, &mut producer);
            assert_eq!(
                (
                    producer.establish_calls,
                    producer.exchange_calls,
                    producer.preview_calls
                ),
                (1, 0, 0)
            );
            result
        },
        Hooks::none(),
    )
    .map_err(|_| Failure::Control)?;
    gate.wait(&SLOT)?;
    let timed_out = {
        let observer = SLOT.observer();
        observer.poll_until(Instant::now())
    };
    let entered_refusal = runtime()?.block_on(async { SLOT.collect() });
    let stopped = stop.request_stop()?;
    let still_running = SLOT.observer().poll_until(Instant::now() + WAIT);
    let second: Input = input(&ExchangeMode::Preview)?;
    let refused = SLOT.submit_fixture(
        second.prepared,
        &ADMISSION,
        component_provider,
        |config, provider, control| {
            let mut producer =
                ScriptProducer::new(config, provider, vec![Ok(())], ExchangeCase::Healthy);
            drive_invocation_with(config, provider, control, &mut producer)
        },
        Hooks::none(),
    );
    let premature = SLOT.take_fixture_result();
    gate.release()?;
    let outcome = collect(&SLOT)?;
    assert_eq!(timed_out, Observation::TimedOut);
    assert_eq!(entered_refusal, Err(CollectError::EnteredRuntime));
    assert_eq!(stopped, StopDisposition::Latched);
    assert_eq!(still_running, Observation::Running(false));
    assert_eq!(refused, Err(SubmitError::Used), "KE-JOB-04-occupied-slot");
    assert!(matches!(premature, Err(CollectError::NotReady)));
    assert!(matches!(&outcome.operation, Some(Err(actual)) if *actual == primary));
    assert_eq!(outcome.termination, Termination::Returned);
    assert!(outcome.retired);
    assert_eq!(config_lifetime.strong_count(), 0);
    assert!(matches!(ADMISSION.claim(), Err(Failure::ProcessUsed)));
    Ok(())
}

/// Genuine protected completion stays withheld until provider-before-Config retirement.
/// Late stop preserves the already committed historical result.
/// # Errors
/// Preserve actual protected Script, barrier, stop and joined-result failures.
/// # Panics
/// Fail if a hint exposes results early, actual retirement reverses or late stop erases commit.
#[test]
fn whole_job_protected_commit_waits_for_actual_outer_retirement() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let Input {
        prepared,
        stop,
        config_lifetime,
    } = input(&ExchangeMode::ProtectedDatagram)?;
    let original = prepared.original_times();
    let provider_witness = Arc::new(OnceLock::<Weak<()>>::new());
    let observed_provider = Arc::clone(&provider_witness);
    let retired_provider = Arc::clone(&provider_witness);
    let config_at_provider = config_lifetime.clone();
    let ordered = Arc::new(AtomicBool::new(false));
    let observed_order = Arc::clone(&ordered);
    let (retire, gate) = owner_gate();
    let mut hooks = Hooks::none();
    hooks.before_retire = Some(retire);
    hooks.after_provider = Some(Box::new(move || {
        assert_eq!(retired_provider.get().map(Weak::strong_count), Some(0));
        assert_eq!(config_at_provider.strong_count(), 1);
        observed_order.store(true, Ordering::Release);
    }));
    SLOT.submit_fixture(
        prepared,
        &ADMISSION,
        component_provider,
        move |config, provider, control| {
            assert!(observed_provider.set(provider.retirement_witness()).is_ok());
            let mut producer =
                ScriptProducer::new(config, provider, vec![Ok(())], ExchangeCase::Healthy);
            let result = drive_invocation_with(config, provider, control, &mut producer);
            assert_eq!(
                (
                    producer.establish_calls,
                    producer.exchange_calls,
                    producer.entropy_calls
                ),
                (1, 1, 1)
            );
            assert_eq!(producer.controls.len(), 1);
            let times = producer
                .controls
                .first()
                .ok_or_else(|| failed(Failure::Control))?;
            assert_eq!((times.origin, times.handshake, times.expiry), original);
            assert_eq!(
                producer.backend_lifetime.as_ref().map(Weak::strong_count),
                Some(0)
            );
            assert!(tokio::runtime::Handle::try_current().is_err());
            result
        },
        hooks,
    )
    .map_err(|_| Failure::Control)?;
    gate.wait(&SLOT)?;
    let live_config = config_lifetime.strong_count();
    let live_provider = provider_witness.get().map(Weak::strong_count);
    let withheld = SLOT.take_fixture_result();
    let hint = SLOT.observer().poll_until(Instant::now() + WAIT);
    let stopped = stop.request_stop()?;
    gate.release()?;
    let outcome = collect(&SLOT)?;
    assert_eq!((live_config, live_provider), (1, Some(1)));
    assert!(
        matches!(withheld, Err(CollectError::NotReady)),
        "KE-JOB-04-result-before-retirement"
    );
    assert_eq!(hint, Observation::Running(false));
    assert_eq!(stopped, StopDisposition::Latched);
    let completion = outcome
        .operation
        .ok_or(Failure::Control)?
        .map_err(|failure| failure.primary)?;
    let summary: &Summary = completion.summary().ok_or(Failure::Control)?;
    assert!(summary.transmitted && summary.packet_bytes > 48);
    assert_eq!((summary.cookies, summary.cookie_bytes), (1, 12));
    assert!(outcome.retired && ordered.load(Ordering::Acquire));
    assert_eq!(outcome.termination, Termination::Returned);
    assert_eq!(provider_witness.get().map(Weak::strong_count), Some(0));
    assert_eq!(config_lifetime.strong_count(), 0);
    assert!(matches!(ADMISSION.claim(), Err(Failure::ProcessUsed)));
    Ok(())
}

/// Stop during the genuine protected entropy callback prevents commit and preserves charges.
/// # Errors
/// Preserve actual protected callback, stop and joined-result failures.
/// # Panics
/// Fail if stopping refunds the operation, sends a packet or produces a success fact.
#[test]
fn whole_job_stop_before_protected_commit_cannot_publish() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let Input { prepared, stop, .. } = input(&ExchangeMode::ProtectedDatagram)?;
    let (hold, gate) = producer_gate();
    SLOT.submit_fixture(
        prepared,
        &ADMISSION,
        component_provider,
        move |config, provider, control| {
            let mut producer =
                ScriptProducer::new(config, provider, vec![Ok(())], ExchangeCase::Healthy);
            producer.protected_gate = Some(hold);
            let result = drive_invocation_with(config, provider, control, &mut producer);
            assert_eq!(
                (
                    producer.establish_calls,
                    producer.exchange_calls,
                    producer.entropy_calls
                ),
                (1, 1, 1)
            );
            let observation = producer
                .last_exchange
                .ok_or_else(|| failed(Failure::Control))?;
            assert!(!observation.has_receipt);
            assert_eq!(observation.after.attempts, observation.before.attempts - 1);
            result
        },
        Hooks::none(),
    )
    .map_err(|_| Failure::Control)?;
    gate.wait(&SLOT)?;
    let stopped = stop.request_stop()?;
    gate.release()?;
    let outcome = collect(&SLOT)?;
    assert_eq!(stopped, StopDisposition::Latched);
    assert!(
        matches!(&outcome.operation, Some(Err(actual)) if *actual == failed(Failure::Cancelled))
    );
    assert_eq!(outcome.termination, Termination::Returned);
    assert!(outcome.retired);
    Ok(())
}

/// Trusted pre-Builder refusal disposes unsubmitted input without claim or completion.
/// # Errors
/// Preserve actual checked preparation and observation errors.
/// # Panics
/// Fail if refusal starts a closure, claims a process or creates collectible completion.
#[test]
fn whole_job_prespawn_refusal_is_not_completion_or_process_use() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let Input {
        prepared,
        config_lifetime,
        ..
    } = input(&ExchangeMode::Preview)?;
    let entered = Arc::new(AtomicBool::new(false));
    let observed = Arc::clone(&entered);
    let mut hooks = Hooks::none();
    hooks.spawn_refusal = Some(std::io::ErrorKind::OutOfMemory);
    let submission = SLOT.submit_fixture(
        prepared,
        &ADMISSION,
        component_provider,
        move |config, provider, control| {
            observed.store(true, Ordering::Release);
            let mut producer =
                ScriptProducer::new(config, provider, vec![Ok(())], ExchangeCase::Healthy);
            drive_invocation_with(config, provider, control, &mut producer)
        },
        hooks,
    );
    let expected = SubmitError::Spawn(std::io::ErrorKind::OutOfMemory);
    assert_eq!(submission, Err(expected));
    assert_eq!(
        SLOT.observer().poll_until(Instant::now() + WAIT),
        Observation::NotSubmitted(expected)
    );
    assert!(matches!(SLOT.collect(), Err(CollectError::NotReady)));
    assert!(matches!(
        SLOT.take_fixture_result(),
        Err(CollectError::NotReady)
    ));
    assert!(!entered.load(Ordering::Acquire));
    assert_eq!(config_lifetime.strong_count(), 0);
    assert!(ADMISSION.claim().is_ok());
    Ok(())
}

/// Fixed opaque test payload; ordinary result collection must not execute this destructor.
struct Payload {
    /// Observe destructor execution without formatting or exposing payload contents.
    drops: &'static AtomicUsize,
}

impl Drop for Payload {
    /// Count destruction only; the sealed-payload contract requires this to remain zero.
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::AcqRel);
    }
}

/// Actual Producer unwinding before any result remains abnormal and unconfirmed.
/// # Errors
/// Preserve actual submission/collection and sealed-count observation failures.
/// # Panics
/// Fail if panic invents operation facts or its opaque payload is dropped by collection.
#[test]
fn whole_job_panic_before_result_seals_payload_without_completion() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    static DROPS: AtomicUsize = AtomicUsize::new(0);
    let Input {
        prepared,
        config_lifetime,
        ..
    } = input(&ExchangeMode::Preview)?;
    SLOT.submit_fixture(
        prepared,
        &ADMISSION,
        component_provider,
        |config, provider, control| {
            let mut producer =
                ScriptProducer::new(config, provider, vec![Ok(())], ExchangeCase::Healthy);
            producer.establish_panic = Some(Box::new(Payload { drops: &DROPS }));
            drive_invocation_with(config, provider, control, &mut producer)
        },
        Hooks::none(),
    )
    .map_err(|_| Failure::Control)?;
    let outcome = collect(&SLOT)?;
    assert!(outcome.operation.is_none());
    assert_eq!(outcome.termination, Termination::Panicked);
    assert!(!outcome.retired);
    assert_eq!(config_lifetime.strong_count(), 0);
    assert_eq!(SLOT.sealed_count().map_err(|_| Failure::Control)?, 1);
    assert_eq!(DROPS.load(Ordering::Acquire), 0);
    assert!(matches!(ADMISSION.claim(), Err(Failure::ProcessUsed)));
    Ok(())
}

/// Genuine recorded failure and its cleanup facts survive a later outer-retirement panic.
/// # Errors
/// Preserve actual submission/collection and sealed-count observation failures.
/// # Panics
/// Fail if retirement panic overwrites the primary or exposes/drops its sealed payload.
#[test]
fn whole_job_retirement_panic_preserves_recorded_primary_and_secondaries() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    static DROPS: AtomicUsize = AtomicUsize::new(0);
    /// Opaque synthetic payload whose bytes must never enter either failure row.
    const PANIC_SECRET: &str = "synthetic-retirement-panic-secret-7461";
    let Input {
        prepared,
        config_lifetime,
        ..
    } = input(&ExchangeMode::Preview)?;
    let primary = Failed {
        primary: Failure::Handshake(std::io::ErrorKind::ConnectionReset),
        cleanup: Some(Failure::Shutdown(std::io::ErrorKind::BrokenPipe)),
        admission_cleanup: Some(Failure::Control),
    };
    let mut hooks = Hooks::none();
    hooks.before_retire = Some(Box::new(|| {
        std::panic::resume_unwind(Box::new((Payload { drops: &DROPS }, PANIC_SECRET)))
    }));
    SLOT.submit_fixture(
        prepared,
        &ADMISSION,
        component_provider,
        move |config, provider, control| {
            let mut producer =
                ScriptProducer::new(config, provider, vec![Err(primary)], ExchangeCase::Healthy);
            drive_invocation_with(config, provider, control, &mut producer)
        },
        hooks,
    )
    .map_err(|_| Failure::Control)?;
    let failure = RunFailed::Abnormal(collect(&SLOT)?);
    let mut emitted = Vec::new();
    assert!(output::write_run_failure(&failure, &mut emitted).is_ok());
    let expected = concat!(
        "scope=engine-ke-prototype eligible=0 fresh_sync=0 stage=worker-termination cause=panicked retirement_confirmed=0\n",
        "scope=engine-ke-prototype eligible=0 fresh_sync=0 stage=handshake cause=io cleanup_stage=shutdown cleanup_cause=io admission_stage=currentness admission_cause=control\n"
    );
    assert_eq!(emitted, expected.as_bytes());
    assert!(
        !emitted
            .windows(PANIC_SECRET.len())
            .any(|bytes| bytes == PANIC_SECRET.as_bytes())
    );
    assert!(
        !emitted
            .windows(b"completed=1".len())
            .any(|bytes| bytes == b"completed=1")
    );

    let mut full = std::io::Cursor::new([0_u8; 32]);
    let result = output::write_run_failure(&failure, &mut full);
    assert!(result.is_err(), "full failure sink must refuse");
    let error = result.err().ok_or(Failure::Control)?;
    assert_eq!(error.kind(), std::io::ErrorKind::WriteZero);
    assert_eq!(full.position(), 32);
    assert_eq!(full.get_ref(), &expected.as_bytes()[..32]);
    let mut empty = std::io::Cursor::new([]);
    let result = output::write_run_failure(&failure, &mut empty);
    assert!(result.is_err(), "empty failure sink must refuse");
    let error = result.err().ok_or(Failure::Control)?;
    assert_eq!(error.kind(), std::io::ErrorKind::WriteZero);
    assert_eq!(empty.position(), 0);

    let RunFailed::Abnormal(outcome) = &failure else {
        return Err(Failure::Control);
    };
    assert!(
        matches!(&outcome.operation, Some(Err(actual)) if *actual == primary),
        "KE-JOB-05-recorded-failure-survives"
    );
    assert_eq!(outcome.termination, Termination::Panicked);
    assert!(!outcome.retired);
    assert_eq!(config_lifetime.strong_count(), 0);
    assert_eq!(SLOT.sealed_count().map_err(|_| Failure::Control)?, 1);
    assert_eq!(DROPS.load(Ordering::Acquire), 0);
    Ok(())
}

/// The actual worker cannot consume/detach its own handle; outside collection stays healthy.
/// # Errors
/// Preserve real startup, Script, channel and joined-result failures.
/// # Panics
/// Fail if self-collection mutates custody or the outside collector cannot consume once.
#[test]
fn whole_job_self_join_refuses_without_moving_the_actual_handle() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let Input { prepared, .. } = input(&ExchangeMode::Preview)?;
    let (start, gate) = owner_gate();
    let mut hooks = Hooks::none();
    hooks.before_start = Some(start);
    let (observed_tx, observed_rx) = sync_channel(1);
    SLOT.submit_fixture(
        prepared,
        &ADMISSION,
        component_provider,
        move |config, provider, control| {
            observed_tx
                .send(SLOT.collect())
                .map_err(|_| failed(Failure::Control))?;
            let mut producer =
                ScriptProducer::new(config, provider, vec![Ok(())], ExchangeCase::Healthy);
            drive_invocation_with(config, provider, control, &mut producer)
        },
        hooks,
    )
    .map_err(|_| Failure::Control)?;
    gate.wait(&SLOT)?;
    gate.release()?;
    let refused = observed_rx
        .recv_timeout(WAIT)
        .map_err(|_| Failure::Control)?;
    let outcome = collect(&SLOT)?;
    assert_eq!(refused, Err(CollectError::SelfJoin));
    assert_eq!(outcome.termination, Termination::Returned);
    assert!(outcome.retired);
    assert!(
        matches!(outcome.operation, Some(Ok(completion)) if completion.summary().is_some_and(|summary: &Summary| !summary.transmitted))
    );
    assert!(matches!(ADMISSION.claim(), Err(Failure::ProcessUsed)));
    Ok(())
}

thread_local! {
    /// Actual worker-thread-local destruction, later than whole-call normal return.
    static TLS_TAIL: RefCell<Option<TlsTail>> = const { RefCell::new(None) };
}

/// One bounded actual TLS destructor handshake; its destructor never asserts or panics.
struct TlsTail {
    /// Reports entry into the actual worker thread's TLS destruction tail.
    entered: SyncSender<()>,
    /// Finite controller release, owned inside the retained collector's scope.
    release: Receiver<()>,
    /// Failed entry/release observations stay typed as an atomic watchdog failure.
    watchdog_failed: Arc<AtomicBool>,
    /// Set at the end of this actual destructor, independently of joined slot state.
    finished: Arc<AtomicBool>,
}

impl TlsTail {
    /// Install this sentinel only in the actual current Script worker's TLS slot.
    /// # Errors
    /// Refuse conflicting TLS ownership instead of replacing a prior sentinel.
    fn install(self) -> Result<(), Failure> {
        TLS_TAIL.with(|slot| {
            let mut current = slot.try_borrow_mut().map_err(|_| Failure::Control)?;
            if current.is_some() {
                return Err(Failure::Control);
            }
            *current = Some(self);
            Ok(())
        })
    }
}

impl Drop for TlsTail {
    /// Observe bounded entry/release failures without unwinding from TLS destruction.
    /// No assertion, formatting, operation fact or completion acknowledgement occurs here.
    fn drop(&mut self) {
        if self.entered.try_send(()).is_err() {
            self.watchdog_failed.store(true, Ordering::Release);
        }
        if self.release.recv_timeout(WAIT).is_err() {
            self.watchdog_failed.store(true, Ordering::Release);
        }
        self.finished.store(true, Ordering::Release);
    }
}

/// Logical return and retired Config cannot substitute for actual joined TLS termination.
/// This tests the reached portable standard-library lifetime, not a native stop deadline.
/// # Errors
/// Preserve actual Script, finite handshake, retained collector and final result failures.
/// # Panics
/// Assert only after the tail is released and the scoped collector explicitly joined.
#[test]
fn whole_job_actual_join_waits_for_thread_local_destruction() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let Input {
        prepared,
        config_lifetime,
        ..
    } = input(&ExchangeMode::Preview)?;
    let finished = Arc::new(AtomicBool::new(false));
    let watchdog_failed = Arc::new(AtomicBool::new(false));
    std::thread::scope(|threads| -> Result<(), Failure> {
        // On error/unwind this gate drops before scope performs any implicit join.
        let ((entered, release), gate) = producer_gate();
        let tail = TlsTail {
            entered,
            release,
            watchdog_failed: Arc::clone(&watchdog_failed),
            finished: Arc::clone(&finished),
        };
        SLOT.submit_fixture(
            prepared,
            &ADMISSION,
            component_provider,
            move |config, provider, control| {
                tail.install().map_err(failed)?;
                let mut producer =
                    ScriptProducer::new(config, provider, vec![Ok(())], ExchangeCase::Healthy);
                drive_invocation_with(config, provider, control, &mut producer)
            },
            Hooks::none(),
        )
        .map_err(|_| Failure::Control)?;
        gate.entered
            .recv_timeout(WAIT)
            .map_err(|_| Failure::Control)?;
        let config_dropped_at_tail = config_lifetime.strong_count();
        let (collected_tx, collected_rx) = sync_channel(1);
        let finished_at_collection = Arc::clone(&finished);
        let collector = threads.spawn(move || {
            let collected = collect_available(&SLOT);
            // Capture immediately after collect returns, before any further handshake.
            let tail_finished = finished_at_collection.load(Ordering::Acquire);
            let receipt = (collected, tail_finished);
            let delivered = collected_tx.try_send(receipt).is_ok();
            (receipt, delivered)
        });
        let observation_deadline = Instant::now() + Duration::from_secs(1);
        let phase_while_held = loop {
            let observed = SLOT.observer().poll_until(observation_deadline);
            match observed {
                Observation::Running(_) | Observation::Busy => std::thread::yield_now(),
                _ => break observed,
            }
        };
        let take_while_held = SLOT.take_fixture_result();
        let receipt_while_held = collected_rx.recv_timeout(Duration::from_millis(50));
        let finished_before_release = finished.load(Ordering::Acquire);
        let released = gate.release();
        let collector_return = collector.join().map_err(|_| Failure::Control)?;
        let final_result = SLOT.take_fixture_result();

        // Reject fixture failures before any marker that can establish behavioral RED.
        released?;
        assert!(
            !watchdog_failed.load(Ordering::Acquire),
            "KE-JOB-SETUP-TLS-watchdog"
        );
        let ((collected, tail_finished_at_return), delivered) = collector_return;
        assert!(delivered, "KE-JOB-SETUP-collector-delivery");
        assert_eq!(collected, Ok(()), "KE-JOB-SETUP-collector-refusal");

        // Every behavioral assertion follows release and actual collector join.
        assert_eq!(
            phase_while_held,
            Observation::Joining,
            "KE-JOB-06-actual-join-required"
        );
        assert!(
            matches!(take_while_held, Err(CollectError::NotReady)),
            "KE-JOB-06-actual-join-required"
        );
        assert!(
            matches!(
                receipt_while_held,
                Err(std::sync::mpsc::RecvTimeoutError::Timeout)
            ),
            "KE-JOB-06-actual-join-required"
        );
        assert!(!finished_before_release, "KE-JOB-06-actual-join-required");
        assert!(tail_finished_at_return, "KE-JOB-06-actual-join-required");
        assert!(finished.load(Ordering::Acquire));
        assert_eq!(config_dropped_at_tail, 0);
        assert_eq!(config_lifetime.strong_count(), 0);
        let outcome = final_result.map_err(|_| Failure::Control)?;
        assert_eq!(outcome.termination, Termination::Returned);
        assert!(outcome.retired);
        assert!(
            matches!(outcome.operation, Some(Ok(completion)) if completion.summary().is_some_and(|summary: &Summary| !summary.transmitted))
        );
        assert!(matches!(
            SLOT.take_fixture_result(),
            Err(CollectError::NotReady)
        ));
        assert!(matches!(ADMISSION.claim(), Err(Failure::ProcessUsed)));
        Ok(())
    })
}

/// Submit only the actual component job while observing genuine adapter/producer entry.
/// # Errors
/// Preserve actual slot refusal; supply no collector, result or retirement substitute.
fn submit_facade_script(
    slot: &'static WorkerSlot,
    prepared: PreparedInvocation,
    admission: &'static ProcessAdmission,
    submissions: &AtomicUsize,
    producers: Arc<AtomicUsize>,
) -> Result<(), SubmitError> {
    submit_reserved_facade_script(
        slot.try_reserve(prepared),
        admission,
        submissions,
        producers,
    )
}

/// Run a permitted component submission while recording adapter and producer entry.
///
/// # Errors
/// Forward a slot or actual spawn refusal without creating a substitute worker.
fn submit_reserved_facade_script(
    reservation: Result<SubmissionReservation, SubmitError>,
    admission: &'static ProcessAdmission,
    submissions: &AtomicUsize,
    producers: Arc<AtomicUsize>,
) -> Result<(), SubmitError> {
    submissions.fetch_add(1, Ordering::AcqRel);
    reservation?.submit_fixture(
        admission,
        component_provider,
        move |config, provider, control| {
            producers.fetch_add(1, Ordering::AcqRel);
            let mut producer =
                ScriptProducer::new(config, provider, vec![Ok(())], ExchangeCase::Healthy);
            drive_invocation_with(config, provider, control, &mut producer)
        },
        Hooks::none(),
    )
}

/// Drain unexpected work in this exclusively owned case slot outside entered Tokio.
/// # Errors
/// Preserve unavailable/unknown custody and actual join/take refusal as setup failures.
/// # Panics
/// The existing actual collector checks joined one-take custody after it returns.
fn drain_facade_observation(
    slot: &'static WorkerSlot,
    observation: Observation,
) -> Result<(), Failure> {
    match observation {
        Observation::Running(_) => {
            collect(slot)?;
        }
        Observation::Joined => {
            slot.take_fixture_result().map_err(|_| Failure::Control)?;
        }
        Observation::Unused | Observation::Taken | Observation::NotSubmitted(_) => {}
        Observation::Busy | Observation::Joining | Observation::Control | Observation::TimedOut => {
            return Err(Failure::Control);
        }
    }
    Ok(())
}

/// The actual synchronous facade refuses entered context before any adapter work.
/// Its same slot/admission then completes a genuine successful protected Script neighbor.
/// # Errors
/// Preserve checked input, runtime, actual unexpected-work drain and genuine Script failures.
/// # Panics
/// Drain unexpected work outside Tokio before asserting the no-submission boundary.
#[test]
fn synchronous_facade_refuses_entered_context_before_submission_then_same_slot_succeeds()
-> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let config = Config::new("127.0.0.1:4460".into(), "Time.Example".into())?;
    let refused_config = Arc::downgrade(&config.lifetime);
    let submissions = Arc::new(AtomicUsize::new(0));
    let producers = Arc::new(AtomicUsize::new(0));
    let submitted = Arc::clone(&submissions);
    let produced = Arc::clone(&producers);
    let refused = runtime()?.block_on(async {
        run_synchronously_with(config, &SLOT, None, move |slot, prepared| {
            submit_facade_script(slot, prepared, &ADMISSION, &submitted, produced)
        })
    });
    let after_refusal = SLOT.observer().poll_until(Instant::now() + WAIT);
    // A moved/removed facade guard may have admitted real work. Drain outside Tokio
    // before the semantic assertion, without substituting a result or resetting custody.
    drain_facade_observation(&SLOT, after_refusal)?;
    assert_eq!(
        submissions.load(Ordering::Acquire),
        0,
        "KE-FACADE-01-no-submission-before-context-refusal"
    );
    assert_eq!(
        producers.load(Ordering::Acquire),
        0,
        "KE-FACADE-01-no-producer-before-context-refusal"
    );
    assert!(
        matches!(refused, Err(RunFailed::CallerContext)),
        "KE-FACADE-01-distinct-caller-context-refusal"
    );
    assert_eq!(after_refusal, Observation::Unused);
    assert_eq!(refused_config.strong_count(), 0);

    let config = Config::from_arguments(
        vec![
            "127.0.0.1:4460".into(),
            "Time.Example".into(),
            "--protected-datagram".into(),
        ]
        .into_iter(),
    )?;
    let completed_config = Arc::downgrade(&config.lifetime);
    let submitted = Arc::clone(&submissions);
    let produced = Arc::clone(&producers);
    let completed = run_synchronously_with(config, &SLOT, None, move |slot, prepared| {
        submit_facade_script(slot, prepared, &ADMISSION, &submitted, produced)
    });
    let after_completion = SLOT.observer().poll_until(Instant::now() + WAIT);
    // Keep cleanup explicit if a faulty facade returns before its actual job joins.
    drain_facade_observation(&SLOT, after_completion)?;
    let completion = completed.map_err(|_| Failure::Control)?;
    let summary: &Summary = completion.summary();
    assert!(summary.transmitted);
    assert_eq!((summary.cookies, summary.cookie_bytes), (1, 12));
    assert_eq!(submissions.load(Ordering::Acquire), 1);
    assert_eq!(producers.load(Ordering::Acquire), 1);
    assert_eq!(after_completion, Observation::Taken);
    assert_eq!(completed_config.strong_count(), 0);
    assert!(matches!(ADMISSION.claim(), Err(Failure::ProcessUsed)));
    Ok(())
}

/// Project a pre-start refusal before any operation or observer cleanup can occur.
/// # Errors
/// Refuse a successful projection in a pre-start failure specimen.
/// # Panics
/// Fail if the original outer label is obscured or cleanup/history is invented.
fn assert_prestart_report(
    observed: Result<ObservedInvocation, ObserverStartError>,
    expected: (&str, &str),
) -> Result<(), Failure> {
    let Err(failure) = SourceProtectedProbeReport::from_observed(observed) else {
        return Err(Failure::Control);
    };
    assert_eq!(failure.labels(), expected);
    assert_eq!(
        (
            failure.primary_labels(),
            failure.cleanup_labels(),
            failure.admission_cleanup_labels()
        ),
        (None, None, None)
    );
    assert!(failure.source_policy_error().is_none());
    assert!(failure.retained_configuration().is_none());
    assert!(!failure.power_stop_failed());
    assert_eq!(failure.power_unregistration_error(), None);
    Ok(())
}

/// A failed power subscription refuses the actual submission adapter.
/// # Errors
/// Preserve checked setup and explicit draining if the facade unexpectedly submits.
/// # Panics
/// Fail if refusal invokes the adapter or spends the original process admission.
#[test]
fn observed_facade_registration_failure_refuses_submission() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let submissions: AtomicUsize = AtomicUsize::new(0);
    let producers: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let registrations: Cell<usize> = Cell::new(0);
    let entered_config: Config = Config::new("127.0.0.1:4460".into(), "Time.Example".into())?;
    let entered: Result<ObservedInvocation, ObserverStartError> = runtime()?.block_on(async {
        run_observing(
            entered_config,
            &SLOT,
            |_: StopHandle| {
                registrations.set(1);
                Err::<fn() -> Result<(), NonZeroU32>, ()>(())
            },
            |reservation| {
                submit_reserved_facade_script(
                    Ok(reservation),
                    &ADMISSION,
                    &submissions,
                    Arc::clone(&producers),
                )
            },
        )
    });
    assert!(matches!(
        entered,
        Err(ObserverStartError::Preparation(RunFailed::CallerContext))
    ));
    assert_prestart_report(entered, ("caller-context", "entered-runtime"))?;
    assert_eq!(registrations.get(), 0);
    let config: Config = Config::new("127.0.0.1:4460".into(), "Time.Example".into())?;
    let competing_config: Config = Config::new("127.0.0.1:4460".into(), "Time.Example".into())?;
    let result: Result<ObservedInvocation, ObserverStartError> = run_observing(
        config,
        &SLOT,
        |_: StopHandle| {
            assert_eq!(SLOT.sealed_count(), Ok(0));
            assert_eq!(
                SLOT.observer().poll_until(Instant::now() + WAIT),
                Observation::Busy
            );
            let competing: Result<ObservedInvocation, ObserverStartError> = run_observing(
                competing_config,
                &SLOT,
                |_: StopHandle| {
                    registrations.set(2);
                    Err::<fn() -> Result<(), NonZeroU32>, ()>(())
                },
                |_| Err(SubmitError::Control),
            );
            assert!(matches!(
                competing,
                Err(ObserverStartError::Preparation(RunFailed::Submission(
                    SubmitError::Busy
                )))
            ));
            assert!(assert_prestart_report(competing, ("worker-submission", "busy")).is_ok());
            assert_eq!(registrations.get(), 0);
            assert_eq!(SLOT.collect(), Err(CollectError::Busy));
            assert_eq!(submissions.load(Ordering::Acquire), 0);
            Err::<fn() -> Result<(), NonZeroU32>, ()>(())
        },
        |reservation| {
            submit_reserved_facade_script(Ok(reservation), &ADMISSION, &submissions, producers)
        },
    );
    let state: Observation = SLOT.observer().poll_until(Instant::now() + WAIT);
    drain_facade_observation(&SLOT, state)?;
    assert!(matches!(result, Err(ObserverStartError::Registration)));
    assert_prestart_report(
        Err(ObserverStartError::Registration),
        ("power-registration", "unavailable"),
    )?;
    assert_managed_registration_refusal(result)?;
    let mut diagnostic: Vec<u8> = Vec::new();
    output::write_power_registration_failure(&mut diagnostic).map_err(|_| Failure::Control)?;
    assert_eq!(diagnostic, b"scope=engine-ke-prototype eligible=0 fresh_sync=0 stage=power-registration cause=unavailable retirement_confirmed=0\n");
    assert_eq!(submissions.load(Ordering::Acquire), 0);
    assert_eq!(state, Observation::Unused);
    assert!(ADMISSION.claim().is_ok());
    Ok(())
}

/// Preserve the actual observer refusal at the successor consumer boundary.
/// # Errors
/// Report a false successful projection of a failed registration.
/// # Panics
/// Fail if the projection invents operation, cleanup or retained reply facts.
fn assert_managed_registration_refusal(
    observed: Result<ObservedInvocation, ObserverStartError>,
) -> Result<(), Failure> {
    let Err(failure): Result<ManagedProtectedReport, SourceProtectedProbeFailure> =
        ManagedProtectedReport::from_observed(observed)
    else {
        return Err(Failure::Control);
    };
    assert_eq!(failure.labels(), ("power-registration", "unavailable"));
    assert_eq!(
        (
            failure.primary_labels(),
            failure.cleanup_labels(),
            failure.admission_cleanup_labels()
        ),
        (None, None, None)
    );
    assert!(failure.source_policy_error().is_none());
    assert!(failure.retained_configuration().is_none());
    assert_eq!(failure.power_unregistration_error(), None);
    assert!(!failure.power_stop_failed());
    assert!(failure.retained_first().is_none());
    assert_eq!(failure.retained_first_nak_at(), None);
    Ok(())
}

/// An already elapsed owner cutoff refuses before the slot or observer is used.
/// # Errors
/// Preserve checked setup and a missing refusal at the actual report boundary.
/// # Panics
/// Fail if an expired plan reaches reservation, registration or submission.
#[test]
fn observed_managed_cutoff_refuses_preparation_before_reservation() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    let now: Instant = Instant::now();
    let earlier: Instant = now
        .checked_sub(Duration::from_millis(50))
        .ok_or(Failure::Deadline)?;
    let (config, _source, _authority): (Config, crate::SourceHandle, crate::SourcePolicyAuthority) =
        managed_attempt_config(earlier, Duration::from_millis(4), Duration::from_millis(8))?;
    let lifetime: Weak<()> = Arc::downgrade(&config.lifetime);
    let registered: Cell<bool> = Cell::new(false);
    let submitted: Cell<bool> = Cell::new(false);
    let observed: Result<ObservedInvocation, ObserverStartError> = run_observing_with_cutoff(
        config,
        Some(now),
        &SLOT,
        |_: StopHandle| {
            registered.set(true);
            Ok(|| Ok(()))
        },
        |_: SubmissionReservation| {
            submitted.set(true);
            Err(SubmitError::Control)
        },
    );
    assert!(
        !registered.get() && !submitted.get(),
        "expired owner entered the observer path"
    );
    assert!(matches!(
        &observed,
        Err(ObserverStartError::Preparation(RunFailed::Preparation(actual)))
            if *actual == failed(Failure::Deadline)
    ));
    let Err(failure) = ManagedProtectedReport::from_observed(observed) else {
        return Err(Failure::Control);
    };
    assert_eq!(failure.labels(), ("currentness", "deadline"));
    assert_eq!(failure.power_unregistration_error(), None);
    assert!(!failure.power_stop_failed());
    assert_eq!(
        SLOT.observer().poll_until(Instant::now() + WAIT),
        Observation::Unused
    );
    assert_eq!(lifetime.strong_count(), 0);
    Ok(())
}

/// Refuse a once-valid plan that reaches registration after its original cutoff.
/// # Errors
/// Preserve fixture admission and unexpected report failures.
/// # Panics
/// Fail if expired work registers or submits, or its reservation blocks a healthy invocation.
#[cfg(any(windows, target_os = "linux"))]
#[test]
fn observed_managed_cutoff_refuses_expired_reservation_before_registration() -> Result<(), Failure>
{
    use crate::ManagedProtectedTerminal;
    use crate::nts_ke_experiment::IoPhase;

    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let cutoff: Instant = Instant::now();
    let origin: Instant = cutoff.checked_sub(WAIT).ok_or(Failure::Deadline)?;
    let (config, _, _) =
        managed_attempt_config(origin, Duration::from_millis(4), Duration::from_millis(8))?;
    let control: Control = Control::clamped(origin, cutoff)?;
    assert!(
        control
            .guard()?
            .remaining(IoPhase::Handshake, origin)
            .is_ok()
    );
    let prepared: PreparedInvocation = PreparedInvocation::checked_fixture(config, control);
    let reservation: SubmissionReservation =
        SLOT.try_reserve(prepared).map_err(|_| Failure::Control)?;
    assert_eq!(SLOT.collect(), Err(CollectError::Busy));

    let registrations: Cell<usize> = Cell::new(0);
    let submissions: Cell<usize> = Cell::new(0);
    let observed: Result<ObservedInvocation, ObserverStartError> = run_reserved_observing(
        reservation,
        |_: StopHandle| {
            registrations.set(registrations.get() + 1);
            Ok(|| Ok(()))
        },
        |_: SubmissionReservation| {
            submissions.set(submissions.get() + 1);
            Err(SubmitError::Control)
        },
    );
    assert_eq!(
        registrations.get(),
        0,
        "expired reservation registered its observer"
    );
    assert_eq!(submissions.get(), 0, "expired reservation submitted work");
    assert!(matches!(
        observed,
        Err(ObserverStartError::Preparation(RunFailed::Preparation(actual)))
            if actual == failed(Failure::Deadline)
    ));
    assert_eq!(
        SLOT.observer().poll_until(Instant::now() + WAIT),
        Observation::Unused
    );

    let (config, _, _) =
        managed_attempt_config(origin, Duration::from_millis(4), Duration::from_millis(8))?;
    let cutoff: Instant = Instant::now().checked_add(WAIT).ok_or(Failure::Deadline)?;
    let closes: Cell<usize> = Cell::new(0);
    let entries: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let observed: ObservedInvocation = run_observing_with_cutoff(
        config,
        Some(cutoff),
        &SLOT,
        |_: StopHandle| {
            registrations.set(registrations.get() + 1);
            Ok(|| {
                closes.set(closes.get() + 1);
                Ok(())
            })
        },
        |reservation: SubmissionReservation| {
            submissions.set(submissions.get() + 1);
            submit_managed_cutoff_worker(&SLOT, reservation, &ADMISSION, Arc::clone(&entries), None)
        },
    )
    .map_err(|_| Failure::Control)?;
    let report: ManagedProtectedReport =
        ManagedProtectedReport::from_observed(Ok(observed)).map_err(|_| Failure::Control)?;
    assert!(
        matches!(report.terminal(), ManagedProtectedTerminal::Restriction(restriction)
        if restriction.code() == "DENY" && restriction.authenticated())
    );
    assert_eq!(
        SLOT.observer().poll_until(Instant::now() + WAIT),
        Observation::Taken
    );
    assert_eq!(registrations.get(), 1);
    assert_eq!(submissions.get(), 1);
    assert_eq!(entries.load(Ordering::Acquire), 1);
    assert_eq!(closes.get(), 1);
    Ok(())
}

/// Carry the cutoff through submission, actual startup and the observer close.
/// The timely neighbor applies a genuine managed restriction in the same worker route.
/// # Errors
/// Preserve setup, submission and unexpected joined-result failures.
/// # Panics
/// Fail if delayed startup admits a producer, loses its join or reports a policy effect.
#[cfg(any(windows, target_os = "linux"))]
#[test]
fn observed_managed_cutoff_survives_worker_start_and_join() -> Result<(), Failure> {
    use crate::{
        ManagedProtectedTerminal, SourceHandle, SourcePolicyAuthority, SourcePolicySnapshot,
    };
    use std::net::SocketAddr;

    static SLOTS: [WorkerSlot; 2] = [const { WorkerSlot::new() }; 2];
    static ADMISSIONS: [ProcessAdmission; 2] = [const { ProcessAdmission::new() }; 2];
    for ((slot, admission), delayed) in SLOTS.iter().zip(&ADMISSIONS).zip([true, false]) {
        let earlier: Instant = Instant::now()
            .checked_sub(Duration::from_millis(50))
            .ok_or(Failure::Deadline)?;
        let (config, source, _authority): (Config, SourceHandle, SourcePolicyAuthority) =
            managed_attempt_config(earlier, Duration::from_millis(4), Duration::from_millis(8))?;
        let remote: SocketAddr = SocketAddr::new(config.peer.ip(), 123);
        let before: SourcePolicySnapshot = source
            .policy_snapshot(remote)
            .map_err(Failure::SourcePolicy)?;
        let lifetime: Weak<()> = Arc::downgrade(&config.lifetime);
        let producer_entries: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
        let entries: Arc<AtomicUsize> = Arc::clone(&producer_entries);
        let closes: Cell<usize> = Cell::new(0);
        let window: Duration = if delayed {
            Duration::from_millis(500)
        } else {
            WAIT
        };
        let cutoff: Instant = Instant::now()
            .checked_add(window)
            .ok_or(Failure::Deadline)?;
        let observed: Result<ObservedInvocation, ObserverStartError> = run_observing_with_cutoff(
            config,
            Some(cutoff),
            slot,
            |stop: StopHandle| {
                let closes: &Cell<usize> = &closes;
                Ok(move || {
                    assert_eq!(
                        slot.observer().poll_until(Instant::now() + WAIT),
                        Observation::Taken
                    );
                    assert_eq!(stop.request_stop(), Ok(StopDisposition::Latched));
                    assert_eq!(stop.request_stop(), Ok(StopDisposition::AlreadyLatched));
                    closes.set(closes.get() + 1);
                    Err(NonZeroU32::MIN)
                })
            },
            |reservation: SubmissionReservation| {
                submit_managed_cutoff_worker(
                    slot,
                    reservation,
                    admission,
                    entries,
                    delayed.then_some(cutoff),
                )
            },
        );
        let state: Observation = slot.observer().poll_until(Instant::now() + WAIT);
        drain_facade_observation(slot, state)?;
        assert_eq!(state, Observation::Taken);
        assert_eq!(closes.get(), 1);
        assert_eq!(lifetime.strong_count(), 0);
        assert_eq!(
            producer_entries.load(Ordering::Acquire),
            usize::from(!delayed)
        );
        let after: SourcePolicySnapshot = source
            .policy_snapshot(remote)
            .map_err(Failure::SourcePolicy)?;
        let report: ManagedProtectedReport =
            ManagedProtectedReport::from_observed(observed).map_err(|_| Failure::Control)?;
        if delayed {
            confirm_unstarted_deadline(&report)?;
            assert_eq!(after.source().revision(), before.source().revision());
            assert_eq!(after.endpoint().revision(), before.endpoint().revision());
            assert!(admission.claim().is_ok());
        } else {
            assert!(
                matches!(report.terminal(), ManagedProtectedTerminal::Restriction(restriction)
                if restriction.code() == "DENY" && restriction.authenticated())
            );
            assert_eq!(report.power_unregistration_error(), Some(NonZeroU32::MIN));
            assert!(after.source().refused() && after.endpoint().refused());
            assert!(matches!(admission.claim(), Err(Failure::ProcessUsed)));
        }
    }
    Ok(())
}

/// Confirm the refused terminal report of the actual joined, unstarted worker.
/// # Errors
/// Refuse a result that does not carry the original deadline disposition.
/// # Panics
/// Fail if expiry loses observer cleanup or invents protected work or history.
#[cfg(any(windows, target_os = "linux"))]
fn confirm_unstarted_deadline(report: &ManagedProtectedReport) -> Result<(), Failure> {
    let crate::ManagedProtectedTerminal::Refused(failure) = report.terminal() else {
        return Err(Failure::Control);
    };
    assert_eq!(failure.labels(), ("currentness", "deadline"));
    assert_eq!(failure.primary_labels(), Some(("currentness", "deadline")));
    assert_eq!(failure.cleanup_labels(), None);
    assert_eq!(failure.admission_cleanup_labels(), None);
    assert!(!failure.power_stop_failed());
    assert_eq!(failure.power_unregistration_error(), Some(NonZeroU32::MIN));
    assert_eq!(report.power_unregistration_error(), Some(NonZeroU32::MIN));
    assert!(report.first().is_none() && report.first_nak_at().is_none());
    assert_eq!(report.remaining_association_work(), None);
    assert_eq!(
        (
            report.attempted_sends(),
            report.fully_submitted_sends(),
            report.remaining_driver_work()
        ),
        (0, 0, 64)
    );
    Ok(())
}

/// Spawn the actual managed worker, optionally holding its startup through expiry.
/// # Errors
/// Preserve submission refusal. Report a missing worker entry or lost release as fixture failure.
/// # Panics
/// Fail if the held worker exposes a joined result before release.
#[cfg(any(windows, target_os = "linux"))]
fn submit_managed_cutoff_worker(
    slot: &'static WorkerSlot,
    reservation: SubmissionReservation,
    admission: &'static ProcessAdmission,
    entries: Arc<AtomicUsize>,
    hold_until: Option<Instant>,
) -> Result<(), SubmitError> {
    let mut hooks: Hooks = Hooks::none();
    let hold: Option<(Instant, Gate)> = hold_until.map(|cutoff: Instant| {
        let (pause, gate): (Box<dyn FnOnce() + Send>, Gate) = owner_gate();
        hooks.before_start = Some(pause);
        (cutoff, gate)
    });
    reservation.submit_bounded_successor_fixture(
        admission,
        component_provider,
        move |config, provider, control, writer| {
            entries.fetch_add(1, Ordering::AcqRel);
            let mut producer: ScriptProducer<'_> =
                ScriptProducer::new(config, provider, vec![Ok(())], ExchangeCase::Healthy);
            producer.managed_initial_control = Some(ProtectedControlKind::Deny);
            drive_successor_fixture(config, provider, control, &mut producer, writer)
        },
        hooks,
    )?;
    if let Some((cutoff, gate)) = hold {
        gate.wait(slot).map_err(|_| SubmitError::Control)?;
        assert!(matches!(
            slot.take_for_receiving(),
            Err(CollectError::NotReady)
        ));
        std::thread::sleep(cutoff.saturating_duration_since(Instant::now()));
        assert!(Instant::now() >= cutoff);
        assert_eq!(
            slot.observer().poll_until(Instant::now() + WAIT),
            Observation::Running(false)
        );
        gate.release().map_err(|_| SubmitError::Control)?;
    }
    Ok(())
}

/// Deliver an event during subscription through the real facade's stop endpoint.
/// # Errors
/// Preserve checked setup and an unexpected facade failure.
/// # Panics
/// Fail if event delivery enters a producer, spends admission or loses Cancelled.
#[test]
fn observed_facade_early_event_cancels_the_original_worker() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let config: Config = Config::new("127.0.0.1:4460".into(), "Time.Example".into())?;
    let submissions: AtomicUsize = AtomicUsize::new(0);
    let producers: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let observer_closed: Cell<bool> = Cell::new(false);
    let observed: ObservedInvocation = run_observing(
        config,
        &SLOT,
        |stop| {
            assert_eq!(stop.request_stop(), Ok(StopDisposition::Latched));
            Ok(|| {
                observer_closed.set(true);
                Err(NonZeroU32::MIN)
            })
        },
        |reservation| {
            submit_reserved_facade_script(
                Ok(reservation),
                &ADMISSION,
                &submissions,
                Arc::clone(&producers),
            )
        },
    )
    .map_err(|_| Failure::Control)?;
    assert!(
        matches!(&observed.result, Err(RunFailed::Operation(actual)) if *actual == failed(Failure::Cancelled))
    );
    assert!(observer_closed.get() && !observed.cleanup.stop_failed);
    assert_eq!(observed.cleanup.unregistration_error, Some(NonZeroU32::MIN));
    let mut diagnostic: Vec<u8> = Vec::new();
    let mut completed: Vec<u8> = Vec::new();
    assert!(!write_observed_run(
        observed,
        &mut completed,
        &mut diagnostic
    ));
    assert_eq!(completed, [0_u8; 0]);
    assert_eq!(diagnostic, concat!(
        "scope=engine-ke-prototype eligible=0 fresh_sync=0 stage=currentness cause=cancelled cleanup_stage=none cleanup_cause=none admission_stage=none admission_cause=none\n",
        "scope=engine-ke-prototype eligible=0 fresh_sync=0 stage=power-cleanup stop_failed=0 unregistration_failed=1\n"
    ).as_bytes());
    assert_eq!(submissions.load(Ordering::Acquire), 1);
    assert_eq!(producers.load(Ordering::Acquire), 0);
    assert_eq!(
        SLOT.observer().poll_until(Instant::now() + WAIT),
        Observation::Taken
    );
    assert!(ADMISSION.claim().is_ok());
    Ok(())
}

/// A normal protected result remains historical through late stop and failed close.
/// # Errors
/// Preserve checked setup, facade and observed late-stop failure.
/// # Panics
/// Fail if close precedes the actual joined take or changes the original history.
#[test]
fn observed_facade_late_event_preserves_joined_protected_result() -> Result<(), Failure> {
    /// Consumer of the same genuinely joined probe result.
    enum Consumer {
        /// Output to an accepting diagnostic and result writer.
        Output,
        /// Output to a writer that cannot retain the result bytes.
        UnwritableOutput,
        /// Present the probe to the distinct public successor report.
        ManagedSuccessor,
    }

    static SLOTS: [WorkerSlot; 4] = [const { WorkerSlot::new() }; 4];
    static ADMISSIONS: [ProcessAdmission; 4] = [const { ProcessAdmission::new() }; 4];
    let cases: [(bool, Consumer); 4] = [
        (false, Consumer::Output),
        (true, Consumer::Output),
        (false, Consumer::UnwritableOutput),
        (true, Consumer::ManagedSuccessor),
    ];
    for ((slot, admission), (late_event, consumer)) in SLOTS.iter().zip(&ADMISSIONS).zip(cases) {
        let config: Config = Config::from_arguments(
            [
                "127.0.0.1:4460".into(),
                "Time.Example".into(),
                "--protected-datagram".into(),
            ]
            .into_iter(),
        )?;
        let submissions: AtomicUsize = AtomicUsize::new(0);
        let producers: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
        let observed: ObservedInvocation = run_observing(
            config,
            slot,
            |stop| {
                Ok(move || {
                    assert_eq!(
                        slot.observer().poll_until(Instant::now() + WAIT),
                        Observation::Taken
                    );
                    if late_event {
                        assert_eq!(stop.request_stop(), Ok(StopDisposition::Latched));
                        Err(NonZeroU32::MIN)
                    } else {
                        Ok(())
                    }
                })
            },
            |reservation| {
                submit_reserved_facade_script(
                    Ok(reservation),
                    admission,
                    &submissions,
                    Arc::clone(&producers),
                )
            },
        )
        .map_err(|_| Failure::Control)?;
        let Ok(ReceivedInvocation::Probe(completion)) = &observed.result else {
            return Err(Failure::Control);
        };
        let summary: &Summary = completion.summary();
        assert!(summary.transmitted);
        assert_eq!((summary.cookies, summary.cookie_bytes), (1, 12));
        assert_eq!(submissions.load(Ordering::Acquire), 1);
        assert_eq!(producers.load(Ordering::Acquire), 1);
        assert_eq!(
            observed.cleanup.unregistration_error,
            late_event.then_some(NonZeroU32::MIN)
        );
        assert!(!observed.cleanup.stop_failed);
        if matches!(consumer, Consumer::ManagedSuccessor) {
            assert_wrong_managed_mode(observed)?;
            continue;
        }
        let mut diagnostic: Vec<u8> = Vec::new();
        if matches!(consumer, Consumer::UnwritableOutput) {
            let mut rejected: [u8; 0] = [];
            let mut sink = std::io::Cursor::new(rejected.as_mut_slice());
            assert!(!write_observed_run(observed, &mut sink, &mut diagnostic));
        } else {
            let mut completed: Vec<u8> = Vec::new();
            assert_eq!(
                write_observed_run(observed, &mut completed, &mut diagnostic),
                !late_event
            );
            assert!(completed.starts_with(b"scope=engine-ke-prototype eligible=0 fresh_sync=0 completed=1 cookies=1 cookie_bytes=12 "));
        }
        if late_event {
            assert_eq!(diagnostic, b"scope=engine-ke-prototype eligible=0 fresh_sync=0 stage=power-cleanup stop_failed=0 unregistration_failed=1\n");
        } else {
            assert_eq!(diagnostic, [0_u8; 0]);
        }
    }
    Ok(())
}

/// The successor projection refuses a joined probe and preserves its close failure.
/// # Errors
/// Report unexpected successful projection of another operation mode.
/// # Panics
/// Fail if refusal obscures the exact close status or invents successor history.
fn assert_wrong_managed_mode(observed: ObservedInvocation) -> Result<(), Failure> {
    let Err(failure): Result<ManagedProtectedReport, SourceProtectedProbeFailure> =
        ManagedProtectedReport::from_observed(Ok(observed))
    else {
        return Err(Failure::Control);
    };
    assert_eq!(failure.labels(), ("currentness", "control"));
    assert_eq!(failure.primary_labels(), Some(("currentness", "control")));
    assert_eq!(failure.power_unregistration_error(), Some(NonZeroU32::MIN));
    assert!(!failure.power_stop_failed());
    assert!(failure.retained_first().is_none());
    assert_eq!(failure.retained_first_nak_at(), None);
    Ok(())
}

/// Drive both real UDP legs under the actual checked config and one worker.
/// # Errors
/// Preserve the actual operation and cleanup outcome.
/// # Panics
/// Fail if the actual invocation initiates another KE, first leg or preview.
#[cfg(any(windows, target_os = "linux"))]
fn loopback_control_operation<'scope>(
    config: &'scope Config,
    provider: &'scope ProviderScope<'scope>,
    control: Control,
    first_receipt: crate::nts_ke_experiment::job::FirstReceiptWriter<'scope>,
    remote: std::net::SocketAddr,
    before_commit: super::LoopbackBeforeCommit,
) -> Operation {
    let original_expiry: Instant = control.expiry;
    let mut producer: ScriptProducer<'_> =
        ScriptProducer::new(config, provider, vec![Ok(())], ExchangeCase::Healthy);
    producer.managed_loopback = Some(remote);
    producer.before_loopback_commit = Some(before_commit);
    let result: Operation =
        drive_successor_fixture(config, provider, control, &mut producer, first_receipt);
    assert_eq!(
        (
            producer.establish_calls,
            producer.exchange_calls,
            producer.preview_calls
        ),
        (1, 1, 0)
    );
    assert!(matches!(
        producer.controls.as_slice(),
        [observed] if observed.expiry == original_expiry
    ));
    result
}

/// Prepare one checked window across both loopback legs.
///
/// A refused first reply can leave the request Pending while it waits for
/// another datagram. This fixture bounds that wait under its original deadline.
/// # Errors
/// Preserve deadline arithmetic and Control construction refusals.
#[cfg(any(windows, target_os = "linux"))]
fn prepare_managed_loopback(config: Config) -> Result<PreparedInvocation, Failure> {
    let origin: Instant = Instant::now();
    let window: Duration = WAIT.checked_mul(2).ok_or(Failure::Deadline)?;
    let cutoff: Instant = origin.checked_add(window).ok_or(Failure::Deadline)?;
    let control: Control = Control::clamped(origin, cutoff)?;
    Ok(PreparedInvocation::checked_fixture(config, control))
}

/// Submit the actual bounded first request and observe the sealed result after join.
/// # Errors
/// Preserve source admission, fixture, worker and joined operation refusal.
/// # Panics
/// Fail if the invocation admits another producer operation or loses its original facts.
#[cfg(any(windows, target_os = "linux"))]
fn run_initial_restriction(
    slot: &'static WorkerSlot,
    admission: &'static ProcessAdmission,
    kind: ProtectedControlKind,
    replace_configuration: bool,
) -> Result<(), Failure> {
    use std::net::SocketAddr;

    use crate::{
        SourceConfigurationSnapshot, SourceHandle, SourcePolicyAuthority, SourcePolicySnapshot,
        SourceScale,
    };

    let earlier: Instant = Instant::now()
        .checked_sub(Duration::from_millis(50))
        .ok_or(Failure::Deadline)?;
    let (config, source, mut authority): (Config, SourceHandle, SourcePolicyAuthority) =
        managed_attempt_config(earlier, Duration::from_millis(4), Duration::from_millis(16))?;
    let original: SourceConfigurationSnapshot = source
        .configuration_snapshot()
        .map_err(Failure::SourcePolicy)?
        .ok_or(Failure::Control)?;
    let remote: SocketAddr = SocketAddr::new(config.peer.ip(), 123);
    let expect_denial: bool = kind != ProtectedControlKind::Rate && !replace_configuration;
    let denial_scope: Option<InitialDenialScope> = expect_denial
        .then(|| InitialDenialScope::capture(&authority, remote))
        .transpose()?;
    let (observe_retired, retired_rx): (
        Option<super::LoopbackBeforeCommit>,
        Receiver<SourcePolicySnapshot>,
    ) = initial_retirement_observer(&source, remote, expect_denial);
    let before: SourcePolicySnapshot = source
        .policy_snapshot(remote)
        .map_err(Failure::SourcePolicy)?;
    let config_lifetime: Weak<()> = Arc::downgrade(&config.lifetime);
    let expected: SourceConfigurationSnapshot = original.clone();
    let observed: Result<ReceivedInvocation, RunFailed> =
        run_joined_with(config, slot, None, |slot, prepared| {
            slot.try_reserve(prepared)?
                .submit_bounded_successor_fixture(
                    admission,
                    component_provider,
                    move |config, provider, control, writer| {
                        let mut producer: ScriptProducer<'_> = ScriptProducer::new(
                            config,
                            provider,
                            vec![Ok(())],
                            ExchangeCase::Healthy,
                        );
                        producer.managed_initial_control = Some(kind);
                        if replace_configuration {
                            producer.before_initial_control_commit = Some(Box::new(move || {
                                let replacement: SourceConfigurationSnapshot = authority
                                    .replace_source_scale(&expected, SourceScale::UnsmearedUtc)
                                    .map_err(Failure::SourcePolicy)?;
                                assert!(!replacement.same_configuration(&expected));
                                Ok(())
                            }));
                        } else {
                            producer.before_initial_control_commit = observe_retired;
                        }
                        let outcome: Operation = drive_successor_fixture(
                            config,
                            provider,
                            control,
                            &mut producer,
                            writer,
                        );
                        assert_eq!((producer.establish_calls, producer.exchange_calls), (1, 1));
                        outcome
                    },
                    Hooks::none(),
                )
        });
    let report: ManagedProtectedReport =
        ManagedProtectedReport::from_observed(Ok(ObservedInvocation {
            result: observed,
            cleanup: crate::nts_ke_experiment::PowerObserverCleanup::default(),
        }))
        .map_err(|_| Failure::Control)?;
    let after: SourcePolicySnapshot = source
        .policy_snapshot(remote)
        .map_err(Failure::SourcePolicy)?;
    assert_initial_restriction_policy(
        &report,
        &before,
        &after,
        &original,
        remote,
        kind,
        replace_configuration,
    )?;
    if let Some(denial_scope) = denial_scope {
        let retired: SourcePolicySnapshot = retired_rx.try_recv().map_err(|_| Failure::Control)?;
        denial_scope.verify(remote, &retired, &after)?;
    }
    assert_eq!(config_lifetime.strong_count(), 0);
    assert_eq!(
        slot.observer().poll_until(Instant::now() + WAIT),
        Observation::Taken
    );
    Ok(())
}

/// Capture one retired send before a selected denial reaches the policy owner.
#[cfg(any(windows, target_os = "linux"))]
fn initial_retirement_observer(
    source: &crate::SourceHandle,
    remote: std::net::SocketAddr,
    observe: bool,
) -> (
    Option<super::LoopbackBeforeCommit>,
    Receiver<crate::SourcePolicySnapshot>,
) {
    use crate::{SourceHandle, SourcePolicySnapshot};

    let (tx, rx): (
        SyncSender<SourcePolicySnapshot>,
        Receiver<SourcePolicySnapshot>,
    ) = sync_channel(1);
    let source: SourceHandle = source.clone();
    let publish: super::LoopbackBeforeCommit = Box::new(move || {
        let retired: SourcePolicySnapshot = source
            .policy_snapshot(remote)
            .map_err(Failure::SourcePolicy)?;
        tx.try_send(retired).map_err(|_| Failure::Control)
    });
    (observe.then_some(publish), rx)
}

/// Compare the real policy records and original joined decision.
/// # Errors
/// Preserve an unexpected result path or an unrepresentable observed floor.
/// # Panics
/// Fail if the joined history, receive debits or live restriction differ.
#[cfg(any(windows, target_os = "linux"))]
fn assert_initial_restriction_policy(
    report: &ManagedProtectedReport,
    before: &crate::SourcePolicySnapshot,
    after: &crate::SourcePolicySnapshot,
    original: &crate::SourceConfigurationSnapshot,
    remote: std::net::SocketAddr,
    kind: ProtectedControlKind,
    replaced: bool,
) -> Result<(), Failure> {
    use crate::ManagedProtectedTerminal;

    assert!(report.first().is_none());
    assert!(report.first_nak_at().is_none());
    assert_eq!(
        (report.attempted_sends(), report.fully_submitted_sends()),
        (1, 1)
    );
    assert_eq!(
        report.remaining_association_work(),
        Some((31, 28, 131_072 - 84 - 84 - 124 - 124))
    );
    match (replaced, report.terminal()) {
        (true, ManagedProtectedTerminal::Refused(failure)) => {
            assert_eq!(
                failure.source_policy_error(),
                Some(crate::SourcePolicyError::ConfigurationChanged)
            );
            assert_eq!(after.source().interval(), Duration::from_millis(4));
            assert_eq!(after.endpoint().interval(), Duration::from_millis(8));
        }
        (false, ManagedProtectedTerminal::Restriction(control)) => {
            let expected_code: &str = match kind {
                ProtectedControlKind::Rate => "RATE",
                ProtectedControlKind::Deny => "DENY",
                ProtectedControlKind::Restrict => "RSTR",
            };
            assert_eq!(control.code(), expected_code);
            assert!(control.authenticated());
            assert_eq!((control.remote(), control.serial()), (remote, 1));
            assert!(control.configuration().same_configuration(original));
            if kind == ProtectedControlKind::Rate {
                let source_interval: Duration = Duration::from_millis(8);
                let endpoint_interval: Duration = Duration::from_millis(16);
                assert_eq!(after.source().interval(), source_interval);
                assert_eq!(after.endpoint().interval(), endpoint_interval);
                assert_eq!(
                    after.source().floor(),
                    control.decided_at().checked_add(source_interval)
                );
                assert_eq!(
                    after.endpoint().floor(),
                    control.decided_at().checked_add(endpoint_interval)
                );
            } else {
                assert_eq!(after.source().interval(), Duration::from_millis(4));
                assert_eq!(after.endpoint().interval(), Duration::from_millis(8));
            }
        }
        _ => return Err(Failure::Control),
    }
    let expected_revision_change: u64 = if replaced { 1 } else { 2 };
    for (before, after) in [
        (before.source(), after.source()),
        (before.endpoint(), after.endpoint()),
    ] {
        assert_eq!(
            after.refused(),
            !replaced && kind != ProtectedControlKind::Rate
        );
        assert!(!after.unavailable());
        assert!(after.floor() > before.floor());
        assert_eq!(
            after.revision().checked_sub(before.revision()),
            Some(expected_revision_change)
        );
    }
    Ok(())
}

/// Retain the records that must survive the original send and initial denial.
#[cfg(any(windows, target_os = "linux"))]
struct InitialDenialScope {
    /// Actual second source of the same policy owner.
    alias: crate::SourceHandle,
    /// Already-bound endpoint on the same IP with a different port.
    other_remote: std::net::SocketAddr,
    /// Alias source record before worker entry.
    alias_source: crate::SourceRestrictionSnapshot,
    /// Distinct endpoint record before worker entry.
    other_endpoint: crate::SourceRestrictionSnapshot,
}

#[cfg(any(windows, target_os = "linux"))]
impl InitialDenialScope {
    /// Bind the untouched neighbor and capture both original records.
    /// # Errors
    /// Preserve genuine binding and snapshot failure.
    fn capture(
        authority: &crate::SourcePolicyAuthority,
        remote: std::net::SocketAddr,
    ) -> Result<Self, Failure> {
        use crate::{SourceHandle, SourcePolicySnapshot};

        let other_remote: std::net::SocketAddr = std::net::SocketAddr::new(remote.ip(), 124);
        let alias: SourceHandle = authority.source(1).map_err(Failure::SourcePolicy)?;
        let _other_binding: crate::source_policy::SourceBinding =
            alias.bind(other_remote).map_err(Failure::SourcePolicy)?;
        let shared: SourcePolicySnapshot = alias
            .policy_snapshot(remote)
            .map_err(Failure::SourcePolicy)?;
        let other: SourcePolicySnapshot = alias
            .policy_snapshot(other_remote)
            .map_err(Failure::SourcePolicy)?;
        Ok(Self {
            alias,
            other_remote,
            alias_source: *shared.source(),
            other_endpoint: *other.endpoint(),
        })
    }

    /// Compare retired send floors and exact shared and untouched records.
    /// # Errors
    /// Preserve readback refusal from the original policy cell.
    /// # Panics
    /// Fail if a denial changes either floor or spreads beyond direct records.
    fn verify(
        &self,
        remote: std::net::SocketAddr,
        retired: &crate::SourcePolicySnapshot,
        applied: &crate::SourcePolicySnapshot,
    ) -> Result<(), Failure> {
        use crate::SourcePolicySnapshot;

        assert!(!retired.source().refused() && !retired.source().unavailable());
        assert!(!retired.endpoint().refused() && !retired.endpoint().unavailable());
        assert_eq!(applied.source().floor(), retired.source().floor());
        assert_eq!(applied.endpoint().floor(), retired.endpoint().floor());
        let shared: SourcePolicySnapshot = self
            .alias
            .policy_snapshot(remote)
            .map_err(Failure::SourcePolicy)?;
        let other: SourcePolicySnapshot = self
            .alias
            .policy_snapshot(self.other_remote)
            .map_err(Failure::SourcePolicy)?;
        assert_same_initial_record(shared.source(), &self.alias_source);
        assert_same_initial_record(shared.endpoint(), applied.endpoint());
        assert_same_initial_record(other.endpoint(), &self.other_endpoint);
        Ok(())
    }
}

/// Compare stored facts independently of the time of each readback.
/// # Panics
/// Fail if any stable restriction property differs.
#[cfg(any(windows, target_os = "linux"))]
fn assert_same_initial_record(
    actual: &crate::SourceRestrictionSnapshot,
    expected: &crate::SourceRestrictionSnapshot,
) {
    assert_eq!(actual.refused(), expected.refused());
    assert_eq!(actual.unavailable(), expected.unavailable());
    assert_eq!(actual.interval(), expected.interval());
    assert_eq!(actual.maximum_interval(), expected.maximum_interval());
    assert_eq!(actual.floor(), expected.floor());
    assert_eq!(actual.revision(), expected.revision());
}

/// Correlated-only and invalid controls spend receive work before initial authenticated RATE.
/// # Errors
/// Preserve actual fixture, operation or joined outcome refusal.
#[cfg(any(windows, target_os = "linux"))]
#[test]
fn joined_initial_authenticated_rate_commits_without_successor() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    run_initial_restriction(&SLOT, &ADMISSION, ProtectedControlKind::Rate, false)
}

/// Apply an initial authenticated DENY to the exact source and endpoint.
/// # Errors
/// Preserve actual fixture, operation or joined outcome refusal.
#[cfg(any(windows, target_os = "linux"))]
#[test]
fn joined_initial_authenticated_deny_commits_without_successor() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    run_initial_restriction(&SLOT, &ADMISSION, ProtectedControlKind::Deny, false)
}

/// Apply an initial authenticated RSTR to the exact source and endpoint.
/// # Errors
/// Preserve actual fixture, operation or joined outcome refusal.
#[cfg(any(windows, target_os = "linux"))]
#[test]
fn joined_initial_authenticated_restrict_commits_without_successor() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    run_initial_restriction(&SLOT, &ADMISSION, ProtectedControlKind::Restrict, false)
}

/// A genuine first control cannot apply after its original policy revision is replaced.
/// # Errors
/// Preserve actual fixture, operation or joined outcome refusal.
#[cfg(any(windows, target_os = "linux"))]
#[test]
fn joined_initial_authenticated_rate_refuses_replaced_definition() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    run_initial_restriction(&SLOT, &ADMISSION, ProtectedControlKind::Rate, true)
}

/// Terminal actions at the retired second reply's final policy boundary.
#[cfg(any(windows, target_os = "linux"))]
#[derive(Clone, Copy)]
enum LoopbackSuccessor {
    /// A later ordinary authenticated reply completes the same pending request.
    ApplyOrdinary,
    /// Let the genuine retired DENY commit under the original authority.
    ApplyDeny,
    /// Replace the actual source revision before the DENY can commit.
    ReplaceConfiguration,
    /// Unwind the real worker before it records any terminal operation result.
    Unwind,
}

#[cfg(any(windows, target_os = "linux"))]
impl LoopbackSuccessor {
    /// Select the independently sealed first response sent by the loopback peer.
    const fn first_reply(self) -> &'static [u8] {
        use crate::nts_ke_experiment::response::fixtures;

        match self {
            Self::ApplyOrdinary => fixtures::LOOPBACK_FIRST_QUALITY_REPLY,
            Self::ApplyDeny => fixtures::LEAP_ANNOUNCED_REPLY,
            Self::ReplaceConfiguration | Self::Unwind => fixtures::LOOPBACK_DELETION_QUALITY_REPLY,
        }
    }
}

/// Count destruction of the opaque payload sealed by the actual unwind case.
#[cfg(any(windows, target_os = "linux"))]
static SUCCESSOR_UNWIND_DROPS: AtomicUsize = AtomicUsize::new(0);

/// Hold the actual terminal boundary and deliver the selected outside event.
/// The component controller observes and releases only this one boundary.
/// # Panics
/// The returned hook resumes the selected fixture unwind or fails if the
/// replacement retains the original revision.
#[cfg(any(windows, target_os = "linux"))]
fn loopback_successor_hook(
    successor: LoopbackSuccessor,
    mut authority: crate::SourcePolicyAuthority,
    original: crate::SourceConfigurationSnapshot,
    target: crate::KePreviewTarget,
) -> (super::LoopbackBeforeCommit, Gate) {
    let ((entered, release), gate) = producer_gate();
    let before_commit: super::LoopbackBeforeCommit = Box::new(move || {
        entered.send(()).map_err(|_| Failure::Control)?;
        release.recv_timeout(WAIT).map_err(|_| Failure::Control)?;
        match successor {
            LoopbackSuccessor::ApplyOrdinary | LoopbackSuccessor::ApplyDeny => {}
            LoopbackSuccessor::ReplaceConfiguration => {
                let replacement: crate::SourceConfigurationSnapshot = authority
                    .replace_ke_preview_configuration(
                        &original,
                        original.addresses().clone(),
                        target,
                    )
                    .map_err(Failure::SourcePolicy)?;
                assert_eq!(replacement.declared_scale(), original.declared_scale());
                assert!(!replacement.same_configuration(&original));
            }
            LoopbackSuccessor::Unwind => std::panic::resume_unwind(Box::new(Payload {
                drops: &SUCCESSOR_UNWIND_DROPS,
            })),
        }
        Ok(())
    });
    (before_commit, gate)
}

/// Wait for the actual finished hint without collecting or taking the worker.
/// # Errors
/// Refuse a watchdog timeout or any inapplicable custody phase.
#[cfg(any(windows, target_os = "linux"))]
fn wait_for_unjoined_successor(slot: &'static WorkerSlot) -> Result<(), Failure> {
    let deadline: Instant = Instant::now().checked_add(WAIT).ok_or(Failure::Deadline)?;
    loop {
        match slot.observer().poll_until(deadline) {
            Observation::Running(true) => return Ok(()),
            Observation::Running(false) | Observation::Busy => std::thread::yield_now(),
            _ => return Err(Failure::Control),
        }
    }
}

/// Observe the first history across normally and abnormally joined successors.
/// Both UDP legs share one original finite fixture window.
///
/// # Errors
/// Preserve checked setup and worker failure ahead of a separate peer failure.
/// # Panics
/// Fail if sent work, committed history, or the exact policy effect is lost.
#[cfg(any(windows, target_os = "linux"))]
fn run_managed_loopback_control(
    slot: &'static WorkerSlot,
    admission: &'static ProcessAdmission,
    successor: LoopbackSuccessor,
) -> Result<(), Failure> {
    let mut point: u8 = 0;
    let measurement: Option<LoopbackMeasurement> = match successor {
        LoopbackSuccessor::ApplyDeny | LoopbackSuccessor::ReplaceConfiguration => {
            Some(LoopbackMeasurement {
                profile: valid_loopback_measurement_profile()?,
                // Keep the original scale equal to the replacement's default.
                scale: SourceScale::Unknown,
            })
        }
        LoopbackSuccessor::ApplyOrdinary | LoopbackSuccessor::Unwind => None,
    };
    run_managed_loopback_observing(
        slot,
        admission,
        successor,
        true,
        measurement,
        move || {
            point = point.checked_add(1).ok_or(Failure::Control)?;
            Ok(point)
        },
        |_, [first, successor]: [LoopbackPeerLeg<u8>; 2]| {
            assert_eq!(
                [
                    first.received,
                    first.sending,
                    successor.received,
                    successor.sending
                ],
                [1, 2, 3, 4]
            );
            Ok(())
        },
    )
}

/// Bind the fixture to an ephemeral IPv4 loopback port with finite I/O deadlines.
/// # Errors
/// Preserve bind refusal and refuse a missing timeout or local address.
#[cfg(any(windows, target_os = "linux"))]
fn bind_managed_loopback_peer() -> Result<(std::net::UdpSocket, std::net::SocketAddr), Failure> {
    use crate::nts_ke_experiment::datagram::DatagramError;

    let peer: std::net::UdpSocket = std::net::UdpSocket::bind((Ipv4Addr::LOCALHOST, 0))
        .map_err(|error: std::io::Error| Failure::Datagram(DatagramError::Bind(error.kind())))?;
    peer.set_read_timeout(Some(WAIT))
        .map_err(|_| Failure::Control)?;
    peer.set_write_timeout(Some(WAIT))
        .map_err(|_| Failure::Control)?;
    let remote: std::net::SocketAddr = peer.local_addr().map_err(|_| Failure::Endpoint)?;
    Ok((peer, remote))
}

/// Explicit checked assumptions for one component observation.
#[cfg(any(windows, target_os = "linux"))]
struct LoopbackMeasurement {
    profile: ExperimentalProfile,
    scale: SourceScale,
}

/// Finite coherence allowance for the experimental loopback calculation.
#[cfg(any(windows, target_os = "linux"))]
const LOOPBACK_MEASUREMENT_ALLOWANCE_NS: u64 = 30_000_000_000;

/// Construct the same fixture model for ordinary and control successors.
/// # Errors
/// Refuse a date window or allowance rejected by the checked constructor.
#[cfg(any(windows, target_os = "linux"))]
fn valid_loopback_measurement_profile() -> Result<ExperimentalProfile, Failure> {
    ExperimentalProfile::new(0, u64::from(u32::MAX), LOOPBACK_MEASUREMENT_ALLOWANCE_NS)
        .map_err(|_| Failure::Control)
}

/// The installed source and its alias under one fixture policy authority.
#[cfg(any(windows, target_os = "linux"))]
struct LoopbackSource {
    authority: crate::SourcePolicyAuthority,
    source: crate::SourceHandle,
    alias: crate::SourceHandle,
    expected: crate::SourceConfigurationSnapshot,
    target: crate::KePreviewTarget,
}

/// Install the exact loopback configuration with an optional scale declaration.
/// # Errors
/// Preserve source, address, target and configuration refusals.
#[cfg(any(windows, target_os = "linux"))]
fn prepare_loopback_source(
    remote: std::net::SocketAddr,
    scale: Option<SourceScale>,
) -> Result<LoopbackSource, Failure> {
    use crate::{
        KePreviewTarget, SourceAddressSet, SourceConfigurationSnapshot, SourceHandle,
        SourcePolicyAuthority, SourcePolicyLimits,
    };

    let limits: SourcePolicyLimits =
        SourcePolicyLimits::new(2, 2, Duration::from_millis(1), Duration::from_millis(4))
            .map_err(Failure::SourcePolicy)?;
    let mut authority: SourcePolicyAuthority =
        crate::source_policy::isolated_authority(limits).map_err(Failure::SourcePolicy)?;
    let source: SourceHandle = authority.source(0).map_err(Failure::SourcePolicy)?;
    let alias: SourceHandle = authority.source(1).map_err(Failure::SourcePolicy)?;
    let target: KePreviewTarget = KePreviewTarget::new(
        std::net::SocketAddr::from((Ipv4Addr::LOCALHOST, 4460)),
        "Time.Example",
    )
    .map_err(|_| Failure::Arguments)?;
    let addresses: SourceAddressSet =
        SourceAddressSet::new(&[remote]).map_err(Failure::SourcePolicy)?;
    let expected: SourceConfigurationSnapshot = authority
        .install_ke_preview_configuration(&source, addresses, target.clone())
        .map_err(Failure::SourcePolicy)?;
    let expected: SourceConfigurationSnapshot = if let Some(scale) = scale {
        authority
            .replace_source_scale(&expected, scale)
            .map_err(Failure::SourcePolicy)?
    } else {
        expected
    };
    Ok(LoopbackSource {
        authority,
        source,
        alias,
        expected,
        target,
    })
}

/// Run the bounded worker while a separate peer owns its socket observations.
/// Pass actual peer and Engine observations to the verifier after both joins.
/// # Errors
/// Preserve fixture setup, worker, peer and verifier failures.
/// # Panics
/// Fail if the actual worker or policy state differs from the selected scenario.
#[cfg(any(windows, target_os = "linux"))]
fn run_managed_loopback_observing<T: Send>(
    slot: &'static WorkerSlot,
    admission: &'static ProcessAdmission,
    successor: LoopbackSuccessor,
    sends_nak: bool,
    measurement: Option<LoopbackMeasurement>,
    peer_clock: impl FnMut() -> Result<T, Failure> + Send,
    verify: impl FnOnce(Option<&ManagedProtectedReport>, [LoopbackPeerLeg<T>; 2]) -> Result<(), Failure>,
) -> Result<(), Failure> {
    use crate::nts_ke_experiment::receive_submitted;
    let (peer, remote): (std::net::UdpSocket, std::net::SocketAddr) = bind_managed_loopback_peer()?;
    let scale: Option<SourceScale> = measurement
        .as_ref()
        .map(|input: &LoopbackMeasurement| input.scale);
    let profile: Option<ExperimentalProfile> =
        measurement.map(|input: LoopbackMeasurement| input.profile);
    let LoopbackSource {
        authority,
        source,
        alias,
        expected,
        target,
    }: LoopbackSource = prepare_loopback_source(remote, scale)?;
    let config: Config = Config::from_managed_protected(
        authority
            .authorize_protected_ke(&expected)
            .map_err(Failure::SourcePolicy)?,
    );
    let config_lifetime: Weak<()> = Arc::downgrade(&config.lifetime);

    let (before_commit, gate): (super::LoopbackBeforeCommit, Gate) =
        loopback_successor_hook(successor, authority, expected.clone(), target);
    let prepared: PreparedInvocation = prepare_managed_loopback(config)?;
    let original_origin: Instant = prepared.original_times().0;
    std::thread::scope(|scope| {
        let peer_socket: &std::net::UdpSocket = &peer;
        let peer_worker: std::thread::ScopedJoinHandle<
            '_,
            Result<[LoopbackPeerLeg<T>; 2], Failure>,
        > = std::thread::Builder::new()
            .spawn_scoped(scope, move || {
                serve_managed_loopback(
                    peer_socket,
                    successor.first_reply(),
                    matches!(successor, LoopbackSuccessor::ApplyOrdinary),
                    sends_nak,
                    peer_clock,
                )
            })
            .map_err(|_| Failure::Control)?;
        let provider_lifetime: Arc<OnceLock<Weak<()>>> =
            submit_managed_loopback_worker(slot, admission, prepared, remote, before_commit)?;
        gate.wait(slot)?;
        let before_successor_commit: Instant = Instant::now();
        assert!(matches!(
            slot.take_for_receiving(),
            Err(CollectError::NotReady)
        ));
        assert_loopback_policy(&source, &alias, remote, false)?;
        gate.release()?;
        wait_for_unjoined_successor(slot)?;
        assert!(matches!(
            slot.take_for_receiving(),
            Err(CollectError::NotReady)
        ));
        let provider_before_join: Option<usize> = provider_lifetime.get().map(Weak::strong_count);
        let result: Result<ReceivedInvocation, RunFailed> = receive_submitted(slot, profile);
        let peer_result: Result<[LoopbackPeerLeg<T>; 2], Failure> =
            peer_worker.join().unwrap_or(Err(Failure::Control));
        assert_eq!(
            provider_before_join,
            Some(1),
            "the actual collector must retain the managed provider until join"
        );
        assert_eq!(provider_lifetime.get().map(Weak::strong_count), Some(0));
        let first_endpoints: Result<(std::net::SocketAddr, std::net::SocketAddr), Failure> =
            peer_result
                .as_ref()
                .map(|[first, _]: &[LoopbackPeerLeg<T>; 2]| {
                    (first.client_endpoint, first.peer_endpoint)
                })
                .map_err(|failure: &Failure| *failure);
        let report: Option<ManagedProtectedReport> = assert_joined_loopback_successor(
            result,
            successor,
            sends_nak,
            remote,
            &expected,
            &(original_origin..=before_successor_commit),
            first_endpoints,
        )?;
        verify(report.as_ref(), peer_result?)
    })?;
    let deny_applied: bool = matches!(successor, LoopbackSuccessor::ApplyDeny);
    assert_loopback_policy(&source, &alias, remote, deny_applied)?;
    assert_loopback_custody(slot, &peer, &config_lifetime, successor)
}

/// Submit the actual component worker and permit observation of its provider lifetime.
/// # Errors
/// Preserve reservation and submission refusal as a fixture control failure.
#[cfg(any(windows, target_os = "linux"))]
fn submit_managed_loopback_worker(
    slot: &'static WorkerSlot,
    admission: &'static ProcessAdmission,
    prepared: PreparedInvocation,
    remote: std::net::SocketAddr,
    before_commit: super::LoopbackBeforeCommit,
) -> Result<Arc<OnceLock<Weak<()>>>, Failure> {
    let provider_lifetime: Arc<OnceLock<Weak<()>>> = Arc::new(OnceLock::new());
    let worker_provider_lifetime: Arc<OnceLock<Weak<()>>> = Arc::clone(&provider_lifetime);
    slot.try_reserve(prepared)
        .and_then(|reservation: SubmissionReservation| {
            reservation.submit_bounded_successor_fixture(
                admission,
                component_provider,
                move |config, provider, control, first_receipt| {
                    worker_provider_lifetime
                        .set(provider.retirement_witness())
                        .map_err(|_: Weak<()>| Failed::from(Failure::Control))?;
                    loopback_control_operation(
                        config,
                        provider,
                        control,
                        first_receipt,
                        remote,
                        before_commit,
                    )
                },
                Hooks::none(),
            )
        })
        .map_err(|_: SubmitError| Failure::Control)?;
    Ok(provider_lifetime)
}

#[cfg(any(windows, target_os = "linux"))]
/// One installed source and its separately observed UDP endpoint.
struct PairLoopbackPeer {
    /// Exact source revision that must appear in both of the worker's receipts.
    expected: crate::SourceConfigurationSnapshot,
    /// Independent loopback peer, held through its actual scoped join.
    socket: std::net::UdpSocket,
    /// Bound destination used by the actual component transport.
    remote: std::net::SocketAddr,
}

/// Install the private bounded policy root for the managed pair fixture.
/// # Errors
/// Preserve the actual policy-limit and root admission refusal.
#[cfg(any(windows, target_os = "linux"))]
fn pair_loopback_authority() -> Result<crate::SourcePolicyAuthority, Failure> {
    let limits: crate::SourcePolicyLimits =
        crate::SourcePolicyLimits::new(2, 2, Duration::from_millis(1), Duration::from_millis(4))
            .map_err(Failure::SourcePolicy)?;
    crate::source_policy::isolated_authority(limits).map_err(Failure::SourcePolicy)
}

/// Capture the first installed source through the same issuer as the native facade.
/// # Errors
/// Preserve the actual source authority refusal.
#[cfg(any(windows, target_os = "linux"))]
fn pair_loopback_config(
    authority: &crate::SourcePolicyAuthority,
    expected: &crate::SourceConfigurationSnapshot,
) -> Result<Config, Failure> {
    authority
        .authorize_protected_ke(expected)
        .map(Config::from_managed_protected)
        .map_err(Failure::SourcePolicy)
}

/// Install one of the pair's sources using the genuine policy owner.
/// # Errors
/// Preserve socket, source, address and target refusal.
#[cfg(any(windows, target_os = "linux"))]
fn pair_loopback_peer(
    authority: &mut crate::SourcePolicyAuthority,
    index: usize,
) -> Result<PairLoopbackPeer, Failure> {
    let (socket, remote): (std::net::UdpSocket, std::net::SocketAddr) =
        bind_managed_loopback_peer()?;
    let expected: crate::SourceConfigurationSnapshot =
        install_pair_loopback_source(authority, index, &[remote])?;
    Ok(PairLoopbackPeer {
        expected,
        socket,
        remote,
    })
}

/// Issue a checked definition for the selected bound peers.
/// # Errors
/// Preserve the actual source, address, target and installation refusals.
#[cfg(any(windows, target_os = "linux"))]
fn install_pair_loopback_source(
    authority: &mut crate::SourcePolicyAuthority,
    index: usize,
    peers: &[std::net::SocketAddr],
) -> Result<crate::SourceConfigurationSnapshot, Failure> {
    let source: crate::SourceHandle = authority.source(index).map_err(Failure::SourcePolicy)?;
    let target: crate::KePreviewTarget = crate::KePreviewTarget::new(
        std::net::SocketAddr::from((Ipv4Addr::LOCALHOST, 4460)),
        "Time.Example",
    )
    .map_err(|_| Failure::Arguments)?;
    let addresses: crate::SourceAddressSet =
        crate::SourceAddressSet::new(peers).map_err(Failure::SourcePolicy)?;
    authority
        .install_ke_preview_configuration(&source, addresses, target)
        .map_err(Failure::SourcePolicy)
}

/// Choose whether both jobs use a single installed source definition.
#[cfg(any(windows, target_os = "linux"))]
#[derive(Clone, Copy)]
enum PairConfigurations {
    /// Two peer sockets selected from one original configuration.
    Shared,
    /// Each independently bound peer has its own source configuration.
    Distinct,
}

/// Select the single active resume delivery for the first joined-pair job.
#[cfg(any(windows, target_os = "linux"))]
#[derive(Clone, Copy)]
enum PairResumeDelivery {
    /// Neither active job receives an event.
    None,
    /// Deliver after the worker records the operation, before it retires.
    BeforeWorkerRetirement,
    /// Deliver inside the presentation reader after the joined take.
    #[cfg(windows)]
    DuringClosingRead,
}

/// Bind two independent peers and install their selected source configuration(s).
/// # Errors
/// Preserve peer binding and source installation refusals.
#[cfg(any(windows, target_os = "linux"))]
fn pair_loopback_peers(
    authority: &mut crate::SourcePolicyAuthority,
    configurations: PairConfigurations,
) -> Result<[PairLoopbackPeer; 2], Failure> {
    if matches!(configurations, PairConfigurations::Distinct) {
        return Ok([
            pair_loopback_peer(authority, 0)?,
            pair_loopback_peer(authority, 1)?,
        ]);
    }
    let (first_socket, first_remote): (std::net::UdpSocket, std::net::SocketAddr) =
        bind_managed_loopback_peer()?;
    let (second_socket, second_remote): (std::net::UdpSocket, std::net::SocketAddr) =
        bind_managed_loopback_peer()?;
    let expected: crate::SourceConfigurationSnapshot =
        install_pair_loopback_source(authority, 0, &[first_remote, second_remote])?;
    Ok([
        PairLoopbackPeer {
            expected: expected.clone(),
            socket: first_socket,
            remote: first_remote,
        },
        PairLoopbackPeer {
            expected,
            socket: second_socket,
            remote: second_remote,
        },
    ])
}

/// Observe one pair job's original first and terminal receipts.
/// # Errors
/// Refuse a missing receipt or a nonordinary terminal.
/// # Panics
/// Fail if source custody, exchange identity, observed endpoints, cutoff or completed work differs.
#[cfg(any(windows, target_os = "linux"))]
fn assert_pair_loopback_report(
    report: &ManagedProtectedReport,
    expected: &crate::SourceConfigurationSnapshot,
    cutoff: Instant,
    peer: [LoopbackPeerLeg<Instant>; 2],
) -> Result<(), Failure> {
    let first: &crate::ManagedProtectedReceipt = report.first().ok_or(Failure::Control)?;
    let crate::ManagedProtectedTerminal::Ordinary(terminal) = report.terminal() else {
        return Err(Failure::Control);
    };
    assert!(first.same_invocation(terminal));
    assert!(first.same_ke_association(terminal));
    assert!(!first.same_exchange(terminal));
    assert_eq!(
        first.certificate_time_provenance(),
        CertificateTimeProvenance::Component
    );
    assert_eq!(
        terminal.certificate_time_provenance(),
        CertificateTimeProvenance::Component
    );
    assert!(first.configuration().same_configuration(expected));
    assert!(terminal.configuration().same_configuration(expected));
    assert!(first.committed_at() <= terminal.committed_at());
    assert!(terminal.committed_at() < cutoff);
    for (leg, receipt) in peer.into_iter().zip([first, terminal]) {
        assert_eq!(receipt.local_endpoint(), leg.client_endpoint);
        assert_eq!(receipt.peer_endpoint(), leg.peer_endpoint);
        assert!(leg.received <= leg.sending);
        assert!(leg.sending <= receipt.received_at());
    }
    assert_loopback_work(report, true, false);
    assert_eq!(report.power_unregistration_error(), None);
    Ok(())
}

/// Submit the real bounded job and inspect its provider before the worker uses it.
/// # Errors
/// Preserve the actual reserved submission refusal.
/// # Panics
/// Fail if the worker substitutes its provider or renews the caller's cutoff.
#[cfg(any(windows, target_os = "linux"))]
fn submit_pair_loopback(
    reservation: SubmissionReservation,
    admission: &'static ProcessAdmission,
    provider_witness: Arc<OnceLock<Weak<()>>>,
    cutoff: Instant,
    remote: std::net::SocketAddr,
    hooks: Hooks,
) -> Result<crate::nts_ke_experiment::job::SubmittedJob, SubmitError> {
    reservation.submit_pair_successor_fixture(
        admission,
        component_provider,
        move |config, provider, control, writer| {
            let current: Weak<()> = provider.retirement_witness();
            let first: &Weak<()> = provider_witness.get_or_init(|| Weak::clone(&current));
            assert!(Weak::ptr_eq(first, &current));
            assert_eq!(current.strong_count(), 1);
            assert_eq!(control.expiry, cutoff);
            loopback_control_operation(
                config,
                provider,
                control,
                writer,
                remote,
                Box::new(|| Ok(())),
            )
        },
        hooks,
    )
}

/// Deliver resume after the original operation is recorded, when selected.
/// # Errors
/// Refuse a missing original target if a late event was requested.
/// # Panics
/// Fail if the original target has no active binding or refuses the event.
#[cfg(any(windows, target_os = "linux"))]
fn pair_retirement_hooks(
    observer: Option<&Weak<PowerTarget>>,
    index: usize,
    delivery: PairResumeDelivery,
) -> Result<Hooks, SubmitError> {
    let mut hooks: Hooks = Hooks::none();
    if index == 0 && matches!(delivery, PairResumeDelivery::BeforeWorkerRetirement) {
        let target: Arc<PowerTarget> = observer
            .and_then(Weak::upgrade)
            .ok_or(SubmitError::Control)?;
        hooks.before_retire = Some(Box::new(move || assert!(deliver_pair_resume(&target))));
    }
    Ok(hooks)
}

/// Deliver resume to the active binding and report whether it accepted delivery.
/// # Panics
/// Fail if the original target is idle before delivery.
#[cfg(any(windows, target_os = "linux"))]
fn deliver_pair_resume(target: &PowerTarget) -> bool {
    assert!(matches!(target.prepare(), Err(Failure::Control)));
    target.deliver(PowerEvent::AutomaticResume).is_ok()
}

/// Actual synthetically authenticated UDP peer result for one managed job.
#[cfg(any(windows, target_os = "linux"))]
type PairPeerJoin<'scope> =
    std::thread::ScopedJoinHandle<'scope, Result<[LoopbackPeerLeg<Instant>; 2], Failure>>;

/// Start one independent two-leg peer within the test's scoped threads.
/// # Errors
/// Preserve the fixture thread submission refusal.
#[cfg(any(windows, target_os = "linux"))]
fn spawn_pair_loopback_peer<'scope, 'env>(
    scope: &'scope std::thread::Scope<'scope, 'env>,
    socket: &'env std::net::UdpSocket,
    successor: LoopbackSuccessor,
) -> Result<PairPeerJoin<'scope>, Failure> {
    std::thread::Builder::new()
        .spawn_scoped(scope, move || {
            serve_managed_loopback(
                socket,
                successor.first_reply(),
                matches!(successor, LoopbackSuccessor::ApplyOrdinary),
                false,
                || Ok(Instant::now()),
            )
        })
        .map_err(|_| Failure::Control)
}

/// Observe the independently scripted mode around each actual worker take.
/// # Errors
/// This successful script uses the real native result type for production dispatch.
/// # Panics
/// Fail if a reading moves across the actual submission or joined take.
#[cfg(windows)]
#[expect(
    clippy::unnecessary_wraps,
    reason = "The shared reader requires its exact fallible native query signature."
)]
fn pair_presentation_read(
    reads: &Cell<usize>,
    submissions: &Cell<usize>,
    slot: &'static WorkerSlot,
    cutoff: Instant,
    observer: Option<&Weak<PowerTarget>>,
    resume_delivery: PairResumeDelivery,
) -> crate::nts_ke_experiment::ProcessLeapPresentationRead {
    let index: usize = reads.get();
    reads.set(index + 1);
    let job: usize = index / 2;
    if index.is_multiple_of(2) {
        assert_eq!(submissions.get(), job);
    } else {
        assert_eq!(submissions.get(), job + 1);
        assert_eq!(slot.observer().poll_until(cutoff), Observation::Taken);
    }
    if index == 1 && matches!(resume_delivery, PairResumeDelivery::DuringClosingRead) {
        assert!(
            observer
                .and_then(Weak::upgrade)
                .is_some_and(|target: Arc<PowerTarget>| deliver_pair_resume(&target)),
            "the first closing read must deliver to the original target"
        );
    }
    if index == 3 {
        Ok(LeapPresentation::SixtySecond)
    } else {
        Ok(LeapPresentation::Compatibility)
    }
}

/// Check unmodified mode outcomes carried to their own job's report.
/// # Panics
/// Fail if the two jobs share readings or lose an observed mode change.
#[cfg(windows)]
fn assert_pair_presentation(
    first: &ManagedProtectedReport,
    second: &ManagedProtectedReport,
    reads: usize,
) {
    assert_eq!(reads, 4);
    assert_eq!(
        first.process_leap_presentation(),
        Some([Ok(LeapPresentation::Compatibility); 2])
    );
    assert_eq!(
        second.process_leap_presentation(),
        Some([
            Ok(LeapPresentation::Compatibility),
            Ok(LeapPresentation::SixtySecond),
        ])
    );
}

/// Compare the original first receipts across the two actual joined jobs.
///
/// # Errors
/// Refuse if either job lost its first ordinary receipt.
///
/// # Panics
/// Fail if the second job's receive precedes the first commit or the jobs share identity.
#[cfg(any(windows, target_os = "linux"))]
fn assert_pair_invocation_order(
    first: &ManagedProtectedReport,
    second: &ManagedProtectedReport,
    configurations: PairConfigurations,
) -> Result<(), Failure> {
    let first_receipt: &crate::ManagedProtectedReceipt = first.first().ok_or(Failure::Control)?;
    let second_receipt: &crate::ManagedProtectedReceipt = second.first().ok_or(Failure::Control)?;
    assert!(first_receipt.committed_at() <= second_receipt.received_at());
    assert_eq!(
        first_receipt
            .configuration()
            .same_configuration(second_receipt.configuration()),
        matches!(configurations, PairConfigurations::Shared)
    );
    assert!(!first_receipt.same_invocation(second_receipt));
    assert!(!first_receipt.same_ke_association(second_receipt));
    assert!(!first_receipt.same_exchange(second_receipt));
    Ok(())
}

/// Match the two actual joined reports to the independent peer and power histories.
/// # Errors
/// Preserve a missing report, peer failure or inconsistent receipt.
/// # Panics
/// Fail if original work, identity, chronology or the scope of power delivery differs.
#[cfg(any(windows, target_os = "linux"))]
fn assert_pair_loopback_reports(
    pair: &ManagedProtectedPairReport,
    peers: [(PairPeerJoin<'_>, &crate::SourceConfigurationSnapshot); 2],
    cutoff: Instant,
    configurations: PairConfigurations,
    resume_delivery: PairResumeDelivery,
    #[cfg(windows)] presentation_reads: usize,
) -> Result<(), Failure> {
    let [(first_peer, first_expected), (second_peer, second_expected)] = peers;
    let first: &ManagedProtectedReport = pair.first();
    let second: &ManagedProtectedReport = pair
        .second()
        .ok_or(Failure::Control)?
        .map_err(|_| Failure::Control)?;
    let first_power: ManagedPowerObservation =
        if matches!(resume_delivery, PairResumeDelivery::None) {
            ManagedPowerObservation::NoEventObserved
        } else {
            ManagedPowerObservation::Invalidated
        };
    for (report, peer, expected, power) in [
        (first, first_peer, first_expected, first_power),
        (
            second,
            second_peer,
            second_expected,
            ManagedPowerObservation::NoEventObserved,
        ),
    ] {
        let observed: [LoopbackPeerLeg<Instant>; 2] =
            peer.join().map_err(|_| Failure::Control)??;
        assert_pair_loopback_report(report, expected, cutoff, observed)?;
        assert_eq!(report.power_observation(), Some(power));
    }
    #[cfg(windows)]
    assert_pair_presentation(first, second, presentation_reads);
    assert_pair_invocation_order(first, second, configurations)
}

/// Retain one provider, observer and configuration across both loopback jobs.
/// # Errors
/// Preserve admission, worker result and independent peer refusal.
/// # Panics
/// Fail if the pair overlaps worker ownership, loses original history, resets
/// its cutoff, duplicates a subscription, or closes before provider retirement.
#[cfg(any(windows, target_os = "linux"))]
#[test]
fn managed_pair_reuses_provider_across_two_joined_loopback_jobs() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    assert_managed_pair_reissue(
        &SLOT,
        &ADMISSION,
        PairConfigurations::Shared,
        PairResumeDelivery::None,
        |_| Ok(()),
    )
}

/// Deliver automatic resume after the first terminal commit and before its join.
/// # Errors
/// Preserve admission, worker result and independent peer refusal.
/// # Panics
/// Fail if a late delivery changes original work or history, if the first
/// report misses the invalidation, or if the new invocation inherits it.
#[cfg(any(windows, target_os = "linux"))]
#[test]
fn managed_pair_late_power_event_invalidates_only_first_joined_observation() -> Result<(), Failure>
{
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    assert_managed_pair_reissue(
        &SLOT,
        &ADMISSION,
        PairConfigurations::Shared,
        PairResumeDelivery::BeforeWorkerRetirement,
        |_| Ok(()),
    )
}

/// Deliver resume during the first closing read, after its joined result was taken.
/// # Errors
/// Preserve admission, worker result and independent peer refusal.
/// # Panics
/// Fail on early binding retirement, discarded receipt facts or inherited invalidation.
#[cfg(windows)]
#[test]
fn managed_pair_closing_read_resume_invalidates_only_first_joined_observation()
-> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    assert_managed_pair_reissue(
        &SLOT,
        &ADMISSION,
        PairConfigurations::Shared,
        PairResumeDelivery::DuringClosingRead,
        |_| Ok(()),
    )
}

/// Reissue the actual joined pair after delivered suspend and automatic resume.
/// # Errors
/// Preserve delivery, admission, worker result and independent peer refusal.
/// # Panics
/// Fail if suspension admits preparation or resume fails to restore the second
/// job's fresh admission under the original provider, observer and cutoff.
#[cfg(any(windows, target_os = "linux"))]
#[test]
fn managed_pair_reissues_after_delivered_suspend_and_automatic_resume() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    assert_managed_pair_reissue(
        &SLOT,
        &ADMISSION,
        PairConfigurations::Distinct,
        PairResumeDelivery::None,
        |target: &PowerTarget| {
            target.deliver(PowerEvent::Suspending)?;
            assert!(matches!(target.prepare(), Err(Failure::Control)));
            target.deliver(PowerEvent::AutomaticResume)
        },
    )
}

/// Preserve a transition or source refusal in the second preparation outcome.
#[cfg(any(windows, target_os = "linux"))]
fn pair_preparation_refusal(cause: Failure) -> SourceProtectedProbeFailure {
    SourceProtectedProbeFailure::from_run_failure(RunFailed::Preparation(Failed::from(cause)))
}

/// Check that the first job has relinquished its original resources before reissue.
/// # Panics
/// Fail on missing collection, overlapping ownership or a replaced provider.
#[cfg(any(windows, target_os = "linux"))]
fn pair_first_taken(
    slot: &'static WorkerSlot,
    cutoff: Instant,
    submissions: usize,
    first_lifetime: &Weak<()>,
    provider: &OnceLock<Weak<()>>,
) {
    assert_eq!(submissions, 1);
    assert_eq!(slot.observer().poll_until(cutoff), Observation::Taken);
    assert_eq!(first_lifetime.strong_count(), 0);
    assert_eq!(provider.get().map(Weak::strong_count), Some(1));
}

/// Join both original worker jobs and observe selected power delivery boundaries.
/// # Errors
/// Preserve the transition, admission, worker result and independent peer refusal.
/// # Panics
/// Fail if a job overlaps original custody, loses receipt identity, resets the
/// cutoff, substitutes the provider, or closes before actual worker retirement.
#[cfg(any(windows, target_os = "linux"))]
fn assert_managed_pair_reissue(
    slot: &'static WorkerSlot,
    admission: &'static ProcessAdmission,
    configurations: PairConfigurations,
    delivery: PairResumeDelivery,
    after_first_take: impl FnOnce(&PowerTarget) -> Result<(), Failure>,
) -> Result<(), Failure> {
    use std::cell::OnceCell;

    use crate::nts_ke_experiment::managed_preview::run_managed_pair_with;

    let mut authority: crate::SourcePolicyAuthority = pair_loopback_authority()?;
    let [first, second]: [PairLoopbackPeer; 2] =
        pair_loopback_peers(&mut authority, configurations)?;
    let config: Config = pair_loopback_config(&authority, &first.expected)?;
    let first_lifetime: Weak<()> = Arc::downgrade(&config.lifetime);
    let observer: OnceCell<Weak<PowerTarget>> = OnceCell::new();
    let provider: Arc<OnceLock<Weak<()>>> = Arc::new(OnceLock::new());
    let submissions: Cell<usize> = Cell::new(0);
    let closes: Cell<usize> = Cell::new(0);
    #[cfg(windows)]
    let presentation_reads: Cell<usize> = Cell::new(0);

    let pair: ManagedProtectedPairReport =
        std::thread::scope(|scope: &std::thread::Scope<'_, '_>| {
            let first_peer: PairPeerJoin<'_> =
                spawn_pair_loopback_peer(scope, &first.socket, LoopbackSuccessor::ApplyOrdinary)?;
            let second_peer: PairPeerJoin<'_> =
                spawn_pair_loopback_peer(scope, &second.socket, LoopbackSuccessor::ApplyOrdinary)?;
            let cutoff: Instant = Instant::now()
                .checked_add(WAIT.checked_mul(2).ok_or(Failure::Deadline)?)
                .ok_or(Failure::Deadline)?;
            let pair: ManagedProtectedPairReport = run_managed_pair_with(
                config,
                || {
                    pair_first_taken(slot, cutoff, submissions.get(), &first_lifetime, &provider);
                    let target: Arc<PowerTarget> = observer
                        .get()
                        .and_then(Weak::upgrade)
                        .ok_or(Failure::Control)
                        .map_err(pair_preparation_refusal)?;
                    assert!(target.prepare().is_ok());
                    after_first_take(&target).map_err(pair_preparation_refusal)?;
                    pair_loopback_config(&authority, &second.expected)
                        .map_err(pair_preparation_refusal)
                },
                cutoff,
                slot,
                |target: Arc<PowerTarget>| {
                    observer.set(Arc::downgrade(&target)).map_err(|_| ())?;
                    let provider: &OnceLock<Weak<()>> = &provider;
                    let closes: &Cell<usize> = &closes;
                    Ok(move || {
                        assert_eq!(slot.observer().poll_until(cutoff), Observation::Taken);
                        assert_eq!(provider.get().map(Weak::strong_count), Some(0));
                        closes.set(closes.get() + 1);
                        drop(target);
                        Err(NonZeroU32::MIN)
                    })
                },
                |reservation: SubmissionReservation| {
                    let index: usize = submissions.get();
                    let remote: std::net::SocketAddr = [first.remote, second.remote]
                        .get(index)
                        .copied()
                        .ok_or(SubmitError::Control)?;
                    let hooks: Hooks = pair_retirement_hooks(observer.get(), index, delivery)?;
                    submissions.set(index + 1);
                    submit_pair_loopback(
                        reservation,
                        admission,
                        Arc::clone(&provider),
                        cutoff,
                        remote,
                        hooks,
                    )
                },
                #[cfg(windows)]
                Some(&mut || {
                    pair_presentation_read(
                        &presentation_reads,
                        &submissions,
                        slot,
                        cutoff,
                        observer.get(),
                        delivery,
                    )
                }),
            )
            .map_err(|_| Failure::Control)?;
            assert_pair_loopback_reports(
                &pair,
                [
                    (first_peer, &first.expected),
                    (second_peer, &second.expected),
                ],
                cutoff,
                configurations,
                delivery,
                #[cfg(windows)]
                presentation_reads.get(),
            )?;
            Ok::<ManagedProtectedPairReport, Failure>(pair)
        })?;

    assert_eq!((submissions.get(), closes.get()), (2, 1));
    assert_eq!(observer.get().map(Weak::strong_count), Some(0));
    assert_eq!(pair.power_unregistration_error(), Some(NonZeroU32::MIN));
    assert!(matches!(admission.claim(), Err(Failure::ProcessUsed)));
    Ok(())
}

/// Keep an already joined first job and refuse the second after a delivered suspend.
/// # Errors
/// Preserve the actual first job and independent peer refusals.
/// # Panics
/// Fail if the suspended attempt reserves or submits another worker or loses first history.
#[cfg(any(windows, target_os = "linux"))]
#[test]
fn managed_pair_delivered_suspend_between_jobs_refuses_second_submission() -> Result<(), Failure> {
    use std::cell::OnceCell;

    use crate::nts_ke_experiment::managed_preview::{
        ManagedProtectedPairReport, run_managed_pair_with,
    };
    use crate::nts_ke_experiment::power_target::{PowerEvent, PowerTarget};

    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let mut authority: crate::SourcePolicyAuthority = pair_loopback_authority()?;
    let first: PairLoopbackPeer = pair_loopback_peer(&mut authority, 0)?;
    let second: PairLoopbackPeer = pair_loopback_peer(&mut authority, 1)?;
    let config: Config = pair_loopback_config(&authority, &first.expected)?;
    let provider: Arc<OnceLock<Weak<()>>> = Arc::new(OnceLock::new());
    let observer: OnceCell<Arc<PowerTarget>> = OnceCell::new();
    let submissions: Cell<usize> = Cell::new(0);
    let cutoff: Instant = Instant::now().checked_add(WAIT).ok_or(Failure::Deadline)?;
    let preparation_refusal = |cause: Failure| {
        SourceProtectedProbeFailure::from_run_failure(RunFailed::Preparation(Failed::from(cause)))
    };

    let report: ManagedProtectedPairReport =
        std::thread::scope(|scope: &std::thread::Scope<'_, '_>| {
            let peer: PairPeerJoin<'_> =
                spawn_pair_loopback_peer(scope, &first.socket, LoopbackSuccessor::ApplyOrdinary)?;
            let report: ManagedProtectedPairReport = run_managed_pair_with(
                config,
                || {
                    assert_eq!(SLOT.observer().poll_until(cutoff), Observation::Taken);
                    let target: &Arc<PowerTarget> = observer
                        .get()
                        .ok_or_else(|| preparation_refusal(Failure::Control))?;
                    target
                        .deliver(PowerEvent::Suspending)
                        .map_err(preparation_refusal)?;
                    pair_loopback_config(&authority, &second.expected).map_err(preparation_refusal)
                },
                cutoff,
                &SLOT,
                |target: Arc<PowerTarget>| {
                    observer.set(Arc::clone(&target)).map_err(|_| ())?;
                    Ok(move || {
                        assert_eq!(SLOT.observer().poll_until(cutoff), Observation::Taken);
                        drop(target);
                        Ok(())
                    })
                },
                |reservation: SubmissionReservation| {
                    let index: usize = submissions.replace(submissions.get() + 1);
                    if index != 0 {
                        return Err(SubmitError::Control);
                    }
                    submit_pair_loopback(
                        reservation,
                        &ADMISSION,
                        Arc::clone(&provider),
                        cutoff,
                        first.remote,
                        Hooks::none(),
                    )
                },
                #[cfg(windows)]
                None,
            )
            .map_err(|_| Failure::Control)?;
            let observations: [LoopbackPeerLeg<Instant>; 2] =
                peer.join().map_err(|_| Failure::Control)??;
            assert_pair_loopback_report(report.first(), &first.expected, cutoff, observations)?;
            Ok::<ManagedProtectedPairReport, Failure>(report)
        })?;

    let refused: &SourceProtectedProbeFailure = report
        .second()
        .ok_or(Failure::Control)?
        .err()
        .ok_or(Failure::Control)?;
    assert_eq!(refused.labels(), ("currentness", "control"));
    assert_eq!(submissions.get(), 1);
    assert_eq!(provider.get().map(Weak::strong_count), Some(0));
    assert_eq!(report.power_unregistration_error(), None);
    assert!(matches!(ADMISSION.claim(), Err(Failure::ProcessUsed)));
    Ok(())
}

/// Keep the actual first commit when the first job's successor authenticates DENY.
/// # Errors
/// Preserve fixture setup, worker, or independent peer failures.
/// # Panics
/// Fail if the genuine restriction still permits second source authorization or submission.
#[cfg(any(windows, target_os = "linux"))]
#[test]
fn managed_pair_terminal_deny_suppresses_second_authorization() -> Result<(), Failure> {
    use crate::nts_ke_experiment::managed_preview::{
        ManagedProtectedPairReport, run_managed_pair_with,
    };
    use crate::nts_ke_experiment::power_target::PowerTarget;

    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let mut authority: crate::SourcePolicyAuthority = pair_loopback_authority()?;
    let first: PairLoopbackPeer = pair_loopback_peer(&mut authority, 0)?;
    let config: Config = pair_loopback_config(&authority, &first.expected)?;
    let provider: Arc<OnceLock<Weak<()>>> = Arc::new(OnceLock::new());
    let second_preparations: Cell<usize> = Cell::new(0);
    let submissions: Cell<usize> = Cell::new(0);
    let cutoff: Instant = Instant::now().checked_add(WAIT).ok_or(Failure::Deadline)?;
    let report: ManagedProtectedPairReport = std::thread::scope(|scope| {
        let peer: PairPeerJoin<'_> =
            spawn_pair_loopback_peer(scope, &first.socket, LoopbackSuccessor::ApplyDeny)?;
        let report: ManagedProtectedPairReport = run_managed_pair_with(
            config,
            || {
                second_preparations.set(second_preparations.get() + 1);
                Err(SourceProtectedProbeFailure::from_run_failure(
                    RunFailed::Operation(Failed::from(Failure::Control)),
                ))
            },
            cutoff,
            &SLOT,
            |target: Arc<PowerTarget>| {
                Ok(move || {
                    drop(target);
                    Ok(())
                })
            },
            |reservation: SubmissionReservation| {
                submissions.set(submissions.get() + 1);
                submit_pair_loopback(
                    reservation,
                    &ADMISSION,
                    Arc::clone(&provider),
                    cutoff,
                    first.remote,
                    Hooks::none(),
                )
            },
            #[cfg(windows)]
            Some(&mut || {
                if submissions.get() == 0 {
                    Err(LeapPresentationError::QueryFailed(5))
                } else {
                    Err(LeapPresentationError::InvalidReply {
                        flags: 1,
                        reserved: 4,
                    })
                }
            }),
        )
        .map_err(|_| Failure::Control)?;
        #[cfg(windows)]
        assert_eq!(
            report.first().process_leap_presentation(),
            Some([
                Err(LeapPresentationError::QueryFailed(5)),
                Err(LeapPresentationError::InvalidReply {
                    flags: 1,
                    reserved: 4
                }),
            ])
        );
        let [ordinary, deny]: [LoopbackPeerLeg<Instant>; 2] =
            peer.join().map_err(|_| Failure::Control)??;
        let receipt: &crate::ManagedProtectedReceipt =
            report.first().first().ok_or(Failure::Control)?;
        let crate::ManagedProtectedTerminal::Restriction(restriction) = report.first().terminal()
        else {
            return Err(Failure::Control);
        };
        assert!(ordinary.sending <= receipt.received_at());
        assert!(receipt.committed_at() <= deny.received);
        assert!(deny.sending <= restriction.decided_at() && restriction.decided_at() < cutoff);
        assert!(receipt.configuration().same_configuration(&first.expected));
        assert!(
            restriction
                .configuration()
                .same_configuration(&first.expected)
        );
        assert_eq!(restriction.code(), "DENY");
        assert!(restriction.authenticated());
        assert_eq!(restriction.remote(), first.remote);
        assert_loopback_work(report.first(), false, false);
        Ok::<ManagedProtectedPairReport, Failure>(report)
    })?;

    assert_eq!(submissions.get(), 1);
    assert_eq!(
        second_preparations.get(),
        0,
        "an authenticated terminal DENY must veto authorization even with a genuine first receipt"
    );
    assert!(report.second().is_none());
    assert_eq!(report.power_unregistration_error(), None);
    assert_eq!(provider.get().map(Weak::strong_count), Some(0));
    Ok(())
}

/// Stop the original active binding during subscription and collect its refusal.
/// # Errors
/// Preserve preparation, collection, or an unexpected terminal disposition.
/// # Panics
/// Fail if the joined Stop refusal spends provider admission, reaches a producer
/// or permits the second job, or if cleanup loses its exact nonzero status.
#[cfg(any(windows, target_os = "linux"))]
#[test]
fn managed_pair_joined_stop_refusal_suppresses_second_job() -> Result<(), Failure> {
    use crate::nts_ke_experiment::managed_preview::{
        ManagedProtectedPairReport, run_managed_pair_with,
    };
    use crate::nts_ke_experiment::power_target::PowerTarget;

    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let (config, _source, _authority): (Config, crate::SourceHandle, crate::SourcePolicyAuthority) =
        managed_attempt_config(
            Instant::now(),
            Duration::from_millis(4),
            Duration::from_millis(8),
        )?;
    let cutoff: Instant = Instant::now().checked_add(WAIT).ok_or(Failure::Deadline)?;
    let submissions: Cell<usize> = Cell::new(0);
    let second_preparations: Cell<usize> = Cell::new(0);
    let closes: Cell<usize> = Cell::new(0);
    let producer_entries: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let report: ManagedProtectedPairReport = run_managed_pair_with(
        config,
        || {
            second_preparations.set(second_preparations.get() + 1);
            Err(SourceProtectedProbeFailure::from_run_failure(
                RunFailed::Operation(Failed::from(Failure::Control)),
            ))
        },
        cutoff,
        &SLOT,
        |target: Arc<PowerTarget>| {
            target
                .deliver(PowerEvent::AutomaticResume)
                .map_err(|_| ())?;
            let closes: &Cell<usize> = &closes;
            Ok(move || {
                assert_eq!(SLOT.observer().poll_until(cutoff), Observation::Taken);
                closes.set(closes.get() + 1);
                drop(target);
                Err(NonZeroU32::MIN)
            })
        },
        |reservation: SubmissionReservation| {
            submissions.set(submissions.get() + 1);
            let entries: Arc<AtomicUsize> = Arc::clone(&producer_entries);
            reservation.submit_pair_successor_fixture(
                &ADMISSION,
                component_provider,
                move |_, _, _, _| {
                    entries.fetch_add(1, Ordering::AcqRel);
                    Err(Failed::from(Failure::Control))
                },
                Hooks::none(),
            )
        },
        #[cfg(windows)]
        None,
    )
    .map_err(|_| Failure::Control)?;

    #[cfg(windows)]
    assert_eq!(report.first().process_leap_presentation(), None);
    let crate::ManagedProtectedTerminal::Refused(failure) = report.first().terminal() else {
        return Err(Failure::Control);
    };
    assert_eq!(failure.labels(), ("currentness", "cancelled"));
    assert!(report.first().first().is_none());
    assert_eq!(
        (
            report.first().attempted_sends(),
            report.first().fully_submitted_sends()
        ),
        (0, 0)
    );
    assert!(report.second().is_none());
    assert_eq!(
        (submissions.get(), second_preparations.get(), closes.get()),
        (1, 0, 1)
    );
    assert_eq!(producer_entries.load(Ordering::Acquire), 0);
    assert_eq!(report.power_unregistration_error(), Some(NonZeroU32::MIN));
    assert!(ADMISSION.claim().is_ok());
    Ok(())
}

/// Retain the genuine abnormal join without manufacturing a closing process read.
/// # Errors
/// Preserve preparation and reject an unexpectedly normal result.
/// # Panics
/// Fail if closing observations or second work follow abnormal retirement.
#[cfg(windows)]
#[test]
fn managed_pair_abnormal_join_omits_closing_process_observation() -> Result<(), Failure> {
    use crate::nts_ke_experiment::managed_preview::{
        ManagedProtectedPairReport, run_managed_pair_with,
    };
    use crate::nts_ke_experiment::power_target::PowerTarget;

    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let mut authority: crate::SourcePolicyAuthority = pair_loopback_authority()?;
    let source: PairLoopbackPeer = pair_loopback_peer(&mut authority, 0)?;
    let config: Config = pair_loopback_config(&authority, &source.expected)?;
    let cutoff: Instant = Instant::now().checked_add(WAIT).ok_or(Failure::Deadline)?;
    let reads: Cell<usize> = Cell::new(0);
    let second_preparations: Cell<usize> = Cell::new(0);

    let result: Result<ManagedProtectedPairReport, SourceProtectedProbeFailure> =
        run_managed_pair_with(
            config,
            || {
                second_preparations.set(second_preparations.get() + 1);
                Err(SourceProtectedProbeFailure::from_run_failure(
                    RunFailed::Operation(Failed::from(Failure::Control)),
                ))
            },
            cutoff,
            &SLOT,
            |target: Arc<PowerTarget>| {
                Ok(move || {
                    assert_eq!(SLOT.observer().poll_until(cutoff), Observation::Taken);
                    drop(target);
                    Ok(())
                })
            },
            |reservation: SubmissionReservation| {
                let mut hooks: Hooks = Hooks::none();
                hooks.before_retire = Some(Box::new(|| {
                    std::panic::resume_unwind(Box::new(()));
                }));
                reservation.submit_pair_successor_fixture(
                    &ADMISSION,
                    component_provider,
                    |_, _, _, _| Err(Failed::from(Failure::Control)),
                    hooks,
                )
            },
            Some(&mut || {
                reads.set(reads.get() + 1);
                Ok(LeapPresentation::Compatibility)
            }),
        );
    let Err(failure) = result else {
        return Err(Failure::Control);
    };
    assert_eq!(failure.labels(), ("worker-termination", "panicked"));
    assert_eq!(reads.get(), 1);
    assert_eq!(second_preparations.get(), 0);
    Ok(())
}

/// Inspect worker ownership and peer work before the fixture destroys its resources.
/// # Errors
/// Preserve clock arithmetic and the peer inspection setup refusal.
/// # Panics
/// Fail if the joined result can be taken twice, a panic payload escapes, or
/// any extra datagram or original input owner remains outstanding.
#[cfg(any(windows, target_os = "linux"))]
fn assert_loopback_custody(
    slot: &'static WorkerSlot,
    peer: &std::net::UdpSocket,
    config_lifetime: &Weak<()>,
    successor: LoopbackSuccessor,
) -> Result<(), Failure> {
    assert_eq!(
        slot.sealed_count(),
        Ok(usize::from(matches!(successor, LoopbackSuccessor::Unwind)))
    );
    assert_eq!(config_lifetime.strong_count(), 0);
    let inspect_until: Instant = Instant::now().checked_add(WAIT).ok_or(Failure::Deadline)?;
    assert_eq!(
        slot.observer().poll_until(inspect_until),
        Observation::Taken
    );
    assert!(matches!(
        slot.take_for_receiving(),
        Err(CollectError::NotReady)
    ));

    peer.set_nonblocking(true).map_err(|_| Failure::Control)?;
    assert!(matches!(
        peer.recv_from(&mut [0_u8; 1]),
        Err(error) if error.kind() == std::io::ErrorKind::WouldBlock
    ));
    Ok(())
}

/// Link the two real send debits and optional DENY to their direct policy records.
/// # Errors
/// Preserve readback failures from the actual original policy cell.
/// # Panics
/// Fail for missing attempt floors, a lost restriction or alias propagation.
#[cfg(any(windows, target_os = "linux"))]
fn assert_loopback_policy(
    source: &crate::SourceHandle,
    alias: &crate::SourceHandle,
    remote: std::net::SocketAddr,
    deny_applied: bool,
) -> Result<(), Failure> {
    use crate::SourcePolicySnapshot;

    let after: SourcePolicySnapshot = source
        .policy_snapshot(remote)
        .map_err(Failure::SourcePolicy)?;
    let alias_after: SourcePolicySnapshot = alias
        .policy_snapshot(remote)
        .map_err(Failure::SourcePolicy)?;
    assert_eq!(after.source().refused(), deny_applied);
    assert_eq!(after.endpoint().refused(), deny_applied);
    assert!(!alias_after.source().refused());
    assert_eq!(alias_after.endpoint().refused(), deny_applied);
    let expected_revision: u64 = if deny_applied { 4 } else { 3 };
    assert_eq!(after.source().revision(), expected_revision);
    assert_eq!(after.endpoint().revision(), expected_revision);
    assert!(after.source().floor().is_some());
    assert!(after.endpoint().floor().is_some());
    Ok(())
}

/// Original receive and transmit bits shared by the independently sealed first replies.
#[cfg(any(windows, target_os = "linux"))]
const FIRST_PEER_TIMESTAMPS: [u64; 2] = [0x2233_4455_6677_8899, 0x3344_5566_7788_99aa];

/// Compare both exact wire values without interpreting either timestamp.
/// # Panics
/// Fail if the joined receipt changes or swaps the two values.
#[cfg(any(windows, target_os = "linux"))]
fn assert_peer_timestamps(receipt: &crate::ManagedProtectedReceipt, expected: [u64; 2]) {
    assert_eq!(
        receipt.peer_timestamps().map(|timestamp| timestamp.bits()),
        expected
    );
}

/// Compare the four advertised peer values against the independently selected wire facts.
/// # Panics
/// Fail if the joined receipt changes, drops or swaps a peer quality field.
#[cfg(any(windows, target_os = "linux"))]
fn assert_peer_quality(receipt: &crate::ManagedProtectedReceipt, expected: (u8, i8, u32, u32)) {
    assert_eq!(
        (
            receipt.peer_stratum(),
            receipt.peer_precision_exponent(),
            receipt.peer_root_delay_bits(),
            receipt.peer_root_dispersion_bits(),
        ),
        expected
    );
}

/// Expected independently sealed fields for the first receipt in the LI=2 histories.
#[cfg(any(windows, target_os = "linux"))]
const FIRST_DELETION_QUALITY: (u8, i8, u32, u32) = (6, -25, 0x9a12_3456, 0xe0b0_c0d0);

/// Inspect the first genuine receipt beside the independently expected terminal facts.
/// # Errors
/// Preserve unexpected operation failures and refuse absent or mismatched custody.
/// # Panics
/// Fail if the first history or the second-leg decision is overwritten or fabricated.
#[cfg(any(windows, target_os = "linux"))]
fn assert_joined_loopback_successor(
    result: Result<ReceivedInvocation, RunFailed>,
    successor: LoopbackSuccessor,
    sends_nak: bool,
    remote: std::net::SocketAddr,
    expected: &crate::SourceConfigurationSnapshot,
    first_commit_window: &std::ops::RangeInclusive<Instant>,
    first_endpoints: Result<(std::net::SocketAddr, std::net::SocketAddr), Failure>,
) -> Result<Option<ManagedProtectedReport>, Failure> {
    match (successor, result) {
        (
            LoopbackSuccessor::ApplyOrdinary
            | LoopbackSuccessor::ApplyDeny
            | LoopbackSuccessor::ReplaceConfiguration,
            Ok(ReceivedInvocation::ManagedSuccessor(joined)),
        ) => {
            use crate::ManagedProtectedTerminal;
            use crate::nts_ke_experiment::datagram::ManagedProtectedHistory;

            let first: &ManagedProtectedHistory = joined.first_history().ok_or(Failure::Control)?;
            let ordinary: bool = matches!(successor, LoopbackSuccessor::ApplyOrdinary);
            assert_loopback_first_receipt(first, remote, expected, first_commit_window, ordinary);
            let first_timing: ReceiptTiming = original_receipt_timing(first)?;
            let terminal_timing: Option<ReceiptTiming> = joined
                .terminal_history()
                .map(original_receipt_timing)
                .transpose()?;
            let report: ManagedProtectedReport =
                ManagedProtectedReport::from_observed(Ok(ObservedInvocation {
                    result: Ok(ReceivedInvocation::ManagedSuccessor(joined)),
                    cleanup: crate::nts_ke_experiment::PowerObserverCleanup::default(),
                }))
                .map_err(|_| Failure::Control)?;
            let projected: &crate::ManagedProtectedReceipt = assert_first_loopback_projection(
                &report,
                &first_timing,
                expected,
                successor,
                sends_nak,
                first_commit_window,
            )?;
            let nak: Option<Instant> = report.first_nak_at();
            assert_loopback_work(&report, ordinary, sends_nak);
            assert_eq!(
                report.recovery_unavailable(),
                matches!(successor, LoopbackSuccessor::ReplaceConfiguration),
            );
            assert_eq!(report.power_unregistration_error(), None);
            if !ordinary {
                assert_eq!(report.terminal_measurement(), None);
            }
            match (successor, report.terminal()) {
                (LoopbackSuccessor::ApplyOrdinary, ManagedProtectedTerminal::Ordinary(receipt)) => {
                    let timing: ReceiptTiming = terminal_timing.ok_or(Failure::Control)?;
                    assert_ordinary_loopback_receipt(
                        projected,
                        receipt,
                        expected,
                        nak.unwrap_or(first_timing.committed_at),
                        &timing,
                        *first_commit_window.end(),
                        report.terminal_measurement(),
                    )?;
                }
                (LoopbackSuccessor::ApplyDeny, ManagedProtectedTerminal::Restriction(history)) => {
                    assert_eq!(projected.leap_indicator(), 1);
                    assert_eq!(history.code(), "DENY");
                    assert!(history.authenticated());
                    assert_eq!(history.remote(), remote);
                    assert_eq!(history.serial(), 2);
                    assert!(history.configuration().same_configuration(expected));
                    assert!(history.decided_at() >= *first_commit_window.end());
                    assert_joined_restriction_debug(&report);
                }
                (
                    LoopbackSuccessor::ReplaceConfiguration,
                    ManagedProtectedTerminal::Refused(failure),
                ) => {
                    assert_eq!(projected.leap_indicator(), 2);
                    assert_eq!(
                        failure.source_policy_error(),
                        Some(crate::SourcePolicyError::ConfigurationChanged),
                    );
                }
                _ => return Err(Failure::Control),
            }
            Ok(Some(report))
        }
        (LoopbackSuccessor::Unwind, Err(RunFailed::Abnormal(joined))) => {
            assert_abnormal_loopback_successor(
                joined,
                remote,
                expected,
                first_commit_window,
                first_endpoints,
            )
            .map(|()| None)
        }
        (_, Err(RunFailed::Operation(failure))) => Err(failure.primary),
        _ => Err(Failure::Control),
    }
}

/// Inspect the original first receipt independently of the terminal disposition.
/// # Errors
/// Refuse a missing first receipt or failed independent measurement oracle.
/// # Panics
/// Fail if receipt custody, original numerics or first NAK chronology changes.
#[cfg(any(windows, target_os = "linux"))]
fn assert_first_loopback_projection<'a>(
    report: &'a ManagedProtectedReport,
    first_timing: &ReceiptTiming,
    expected: &crate::SourceConfigurationSnapshot,
    successor: LoopbackSuccessor,
    sends_nak: bool,
    first_commit_window: &std::ops::RangeInclusive<Instant>,
) -> Result<&'a crate::ManagedProtectedReceipt, Failure> {
    let committed_at: Instant = first_timing.committed_at;
    let projected: &crate::ManagedProtectedReceipt = report.first().ok_or(Failure::Control)?;
    let nak: Option<Instant> = report.first_nak_at();
    assert_eq!(nak.is_some(), sends_nak);
    assert_eq!(projected.committed_at(), committed_at);
    assert_eq!(projected.packet_bytes(), 252);
    assert_eq!(projected.initial_cookie_population(), (1, 12));
    assert_peer_timestamps(projected, FIRST_PEER_TIMESTAMPS);
    let first_quality: (u8, i8, u32, u32) = match successor {
        LoopbackSuccessor::ApplyOrdinary => (5, -23, 0x8123_4567, 0xf102_0304),
        LoopbackSuccessor::ApplyDeny => (2, 0, 0, 0),
        LoopbackSuccessor::ReplaceConfiguration | LoopbackSuccessor::Unwind => {
            FIRST_DELETION_QUALITY
        }
    };
    assert_peer_quality(projected, first_quality);
    first_timing.assert_projection(projected);
    let first_measurement: Option<Result<ExperimentalMeasurement, MeasurementRefusal>> =
        report.first_measurement();
    if matches!(
        successor,
        LoopbackSuccessor::ApplyDeny | LoopbackSuccessor::ReplaceConfiguration
    ) {
        assert!(
            matches!(first_measurement, Some(Ok(measurement)) if
                measurement.declared_limit_ns() == LOOPBACK_MEASUREMENT_ALLOWANCE_NS),
            "first receipt lost its original measurement: {first_measurement:?}"
        );
        let assumptions: DeclaredQualityAssumptions = loopback_quality_assumptions(131_072.0)?;
        let quality: ConditionalQuality<'_, crate::ManagedProtectedReceipt> = report
            .first_quality(&assumptions)
            .ok_or(Failure::Control)?
            .map_err(|_| Failure::Control)?;
        assert_joined_quality(
            &quality,
            projected,
            &assumptions,
            first_quality,
            FIRST_PEER_TIMESTAMPS,
        )?;
        assert!(report.terminal_quality(&assumptions).is_none());
    }
    if let Some(Ok(measurement)) = first_measurement {
        first_timing.assert_measurement(measurement, FIRST_PEER_TIMESTAMPS)?;
    }
    assert!(projected.configuration().same_configuration(expected));
    assert!(projected.received_at() <= committed_at);
    if let Some(nak) = nak {
        assert!(committed_at <= nak && first_commit_window.contains(&nak));
    }
    Ok(projected)
}

/// Debit exactly two real requests and each prescribed peer reply.
/// # Panics
/// Fail on a missing full send or a refunded receive frame, byte or readiness debit.
#[cfg(any(windows, target_os = "linux"))]
fn assert_loopback_work(report: &ManagedProtectedReport, ordinary: bool, sends_nak: bool) {
    assert_eq!(
        (report.attempted_sends(), report.fully_submitted_sends()),
        (2, 2)
    );
    // Each leg needs write and read readiness. Each datagram needs a receive attempt.
    let nak_count: usize = usize::from(sends_nak);
    assert!(report.remaining_driver_work() <= 58 - nak_count);
    let (first_bytes, last_bytes): (usize, usize) = if ordinary { (156, 140) } else { (140, 124) };
    assert_eq!(
        report.remaining_association_work(),
        Some((
            30,
            30 - nak_count,
            131_072 - first_bytes - last_bytes - 84 * nak_count
        )),
    );
}

/// Coordinates of each original application wall-read bracket.
#[cfg(any(windows, target_os = "linux"))]
type CaptureCoordinates = [(Instant, std::time::SystemTime, Instant); 3];

/// Original coordinates and independently derived elapsed bounds.
#[cfg(any(windows, target_os = "linux"))]
struct ReceiptTiming {
    /// Local monotonic and wall coordinates from the original history.
    captures: CaptureCoordinates,
    /// The allocation retained by the actual joined history.
    run_context: Arc<RunClockContext>,
    /// Exact midpoint in half nanoseconds from the original chart.
    reference_half_ns: u128,
    /// Independently paired bounds from the original local brackets.
    bounds: [Duration; 2],
    /// Original authenticated acceptance, before the final commit and join.
    accepted_at: Instant,
    /// Original guard observation, distinct from the receive and midpoint references.
    committed_at: Instant,
    /// Exact original native dispositions, retained before projecting the receipt.
    #[cfg(windows)]
    native: [Option<ProtectedNativeClockObservation>; 3],
}

#[cfg(any(windows, target_os = "linux"))]
impl ReceiptTiming {
    /// Compare with independent fixed-point arithmetic over original captures and peer bits.
    /// # Errors
    /// Preserve a wall conversion or original receipt order failure.
    /// # Panics
    /// Fail if a joined measurement uses the wrong leg, bracket, age, or units.
    fn assert_measurement(
        &self,
        measurement: ExperimentalMeasurement,
        [receive, transmit]: [u64; 2],
    ) -> Result<(), Failure> {
        const NANOSECONDS: i128 = 1_000_000_000;
        const Q32: i128 = 1_i128 << 32;
        let t1: i128 = numerics_tests::ntp_nanoseconds(self.captures[0].1)?;
        let send_return_wall: i128 = numerics_tests::ntp_nanoseconds(self.captures[1].1)?;
        let t4: i128 = numerics_tests::ntp_nanoseconds(self.captures[2].1)?;
        let (t2, t3): (i128, i128) = (i128::from(receive), i128::from(transmit));
        let q1: i128 = (t1 * Q32) / NANOSECONDS;
        let q4: i128 = (t4 * Q32) / NANOSECONDS;
        let send_return_q32: i128 = (send_return_wall * Q32) / NANOSECONDS;
        assert_eq!(
            measurement
                .coordinates()
                .map(time_domain::UtcCoordinateQ32::ticks),
            [q1, t2, t3, q4],
        );
        let coordinates: [i128; 5] = [q1, send_return_q32, t2, t3, q4];
        let window: time_domain::EraWindow = measurement.software_window();
        assert_eq!(Some(&window.earliest().ticks()), coordinates.iter().min());
        assert_eq!(Some(&window.latest().ticks()), coordinates.iter().max());
        assert_eq!(
            measurement.nominal_phase().numerator(),
            NANOSECONDS * (t2 + t3) - Q32 * (t1 + t4),
        );
        assert_eq!(
            measurement.nominal_round_trip().numerator(),
            Q32 * (t4 - t1) - NANOSECONDS * (t3 - t2),
        );
        let age: Duration = self
            .accepted_at
            .checked_duration_since(self.captures[2].2)
            .ok_or(Failure::Control)?;
        assert_eq!(measurement.acquisition_age(), age);
        Ok(())
    }

    /// Require public receipt accessors to preserve the original timing facts.
    /// # Panics
    /// Fail if the projection changes or discards an original timing value.
    fn assert_projection(&self, receipt: &crate::ManagedProtectedReceipt) {
        assert_eq!(
            capture_coordinates(receipt.application_clock_captures()),
            self.captures
        );
        assert_eq!(receipt.application_wall_elapsed_bounds(), Some(self.bounds));
        assert_eq!(receipt.received_at(), self.captures[2].2);
        assert_eq!(receipt.committed_at(), self.committed_at);
        assert!(receipt.retains_run_context(&self.run_context));
        let reference: Result<u128, RelativeMeasurementRefusal> =
            receipt
                .nominal_reference()
                .map(|value: ExperimentalReferenceNs| {
                    2 * u128::from(value.whole_nanoseconds())
                        + u128::from(value.has_half_nanosecond())
                });
        assert_eq!(reference, Ok(self.reference_half_ns));
        #[cfg(windows)]
        assert!(
            receipt.application_native_clock_captures() == self.native,
            "joined receipt changed the original native clock observations"
        );
    }
}

/// Keep the leading monotonic read, wall value and trailing monotonic read.
#[cfg(any(windows, target_os = "linux"))]
fn capture_coordinates(captures: [SoftwareClock; 3]) -> CaptureCoordinates {
    captures.map(|capture: SoftwareClock| {
        (
            capture.monotonic(),
            capture.utc(),
            capture.monotonic_after(),
        )
    })
}

/// Capture all original readings and independently pair their elapsed endpoints.
/// # Errors
/// Preserve an unexpected reversal in the genuine joined capture sequence.
/// # Panics
/// Fail if the retained captures or original commit are out of order.
#[cfg(any(windows, target_os = "linux"))]
fn original_receipt_timing(
    original: &crate::nts_ke_experiment::datagram::ManagedProtectedHistory,
) -> Result<ReceiptTiming, Failure> {
    let captures: [SoftwareClock; 3] = original.history().captures();
    let [before, after, read]: [SoftwareClock; 3] = captures;
    assert!(before.monotonic() <= before.monotonic_after());
    assert!(before.monotonic_after() <= after.monotonic());
    assert!(after.monotonic() <= after.monotonic_after());
    assert!(after.monotonic_after() <= read.monotonic());
    assert!(read.monotonic() <= read.monotonic_after());
    assert!(read.monotonic_after() <= original.history().committed_at());
    let minimum: Duration = read
        .monotonic()
        .checked_duration_since(before.monotonic_after())
        .ok_or(Failure::Control)?;
    let maximum: Duration = read
        .monotonic_after()
        .checked_duration_since(before.monotonic())
        .ok_or(Failure::Control)?;
    let run_context: &Arc<RunClockContext> = original.history().run_context();
    let origin: Instant = run_context.origin;
    assert!(origin <= before.monotonic());
    let send_offset_ns: u128 = before.monotonic().duration_since(origin).as_nanos();
    let read_offset_ns: u128 = read.monotonic().duration_since(origin).as_nanos();
    let reference_half_ns: u128 = send_offset_ns
        .checked_add(read_offset_ns)
        .ok_or(Failure::Control)?;
    Ok(ReceiptTiming {
        captures: capture_coordinates(captures),
        run_context: Arc::clone(run_context),
        reference_half_ns,
        bounds: [minimum, maximum],
        accepted_at: original.history().receipt().accepted_at(),
        committed_at: original.history().committed_at(),
        #[cfg(windows)]
        native: original.history().native_captures(),
    })
}

/// Compare the ordinary terminal receipt with its original history and first receipt.
/// # Errors
/// Preserve independent numerical and original age check failures.
/// # Panics
/// Fail if either receipt loses invocation, timing, configuration or packet facts.
#[cfg(any(windows, target_os = "linux"))]
fn assert_ordinary_loopback_receipt(
    first: &crate::ManagedProtectedReceipt,
    receipt: &crate::ManagedProtectedReceipt,
    expected: &crate::SourceConfigurationSnapshot,
    earliest_arrival: Instant,
    timing: &ReceiptTiming,
    before_commit: Instant,
    measurement: Option<Result<ExperimentalMeasurement, MeasurementRefusal>>,
) -> Result<(), Failure> {
    assert_eq!(first.leap_indicator(), 0);
    assert!(first.same_invocation(receipt));
    assert!(receipt.configuration().same_configuration(expected));
    assert!(earliest_arrival <= receipt.received_at());
    assert!(receipt.received_at() <= receipt.committed_at());
    assert_eq!(receipt.packet_bytes(), 236);
    assert_eq!(receipt.initial_cookie_population(), (2, 24));
    assert_eq!(receipt.leap_indicator(), 0);
    assert_peer_timestamps(receipt, [0x4455_6677_8899_aabb, 0x5566_7788_99aa_bbcc]);
    assert_peer_quality(receipt, (7, -19, 0xcafe_4321, 0xa0b0_9080));
    timing.assert_projection(receipt);
    if let Some(Ok(measurement)) = measurement {
        assert!(timing.accepted_at <= before_commit);
        assert!(before_commit < timing.committed_at);
        timing.assert_measurement(measurement, [0x4455_6677_8899_aabb, 0x5566_7788_99aa_bbcc])?;
    }
    Ok(())
}

/// Compare joined DENY diagnostics with the permitted summary fields.
/// # Panics
/// Fail if formatting leaks source configuration, endpoint, packet or timing data.
#[cfg(any(windows, target_os = "linux"))]
fn assert_joined_restriction_debug(report: &ManagedProtectedReport) {
    assert_eq!(
        format!("{report:?}"),
        format!(
            concat!(
                "ManagedProtectedReport {{ first: Some(ManagedProtectedReceipt ",
                "{{ packet_bytes: 252, leap_indicator: 1, .. }}), ",
                "terminal: Restriction(ManagedProtectedRestriction ",
                "{{ code: \"DENY\", authenticated: true, .. }}), ",
                "attempted_sends: 2, fully_submitted_sends: 2, ",
                "remaining_driver_work: {}, .. }}",
            ),
            report.remaining_driver_work()
        ),
        "joined diagnostics disclose only the selected summary fields"
    );
}

/// Compare the public failure with genuine first-reply custody after abnormal join.
/// # Errors
/// Preserve wrong-mode refusal, missing history or independent peer failure.
/// # Panics
/// Fail if retirement changes the first receipt or manufactures terminal history.
#[cfg(any(windows, target_os = "linux"))]
fn assert_abnormal_loopback_successor(
    joined: JoinedOutcome,
    remote: std::net::SocketAddr,
    expected: &crate::SourceConfigurationSnapshot,
    first_commit_window: &std::ops::RangeInclusive<Instant>,
    first_endpoints: Result<(std::net::SocketAddr, std::net::SocketAddr), Failure>,
) -> Result<(), Failure> {
    assert_eq!(joined.termination, Termination::Panicked);
    assert!(!joined.retired);
    assert!(joined.operation.is_none());
    let first: &crate::nts_ke_experiment::datagram::ManagedProtectedHistory =
        joined.first_receipt().ok_or(Failure::Control)?;
    assert_loopback_first_receipt(first, remote, expected, first_commit_window, false);
    let committed_at: Instant = first.history().committed_at();
    let first_timing: ReceiptTiming = original_receipt_timing(first)?;
    let Err(failure): Result<ManagedProtectedReport, SourceProtectedProbeFailure> =
        ManagedProtectedReport::from_observed(Ok(ObservedInvocation {
            result: Err(RunFailed::Abnormal(joined)),
            cleanup: crate::nts_ke_experiment::PowerObserverCleanup::default(),
        }))
    else {
        return Err(Failure::Control);
    };
    assert_eq!(failure.labels(), ("worker-termination", "panicked"));
    assert!(failure.primary_labels().is_none());
    assert!(failure.retained_configuration().is_none());
    assert_eq!(failure.power_unregistration_error(), None);
    let retained: crate::ManagedProtectedReceipt =
        failure.retained_first().ok_or(Failure::Control)?;
    assert_eq!(
        retained.certificate_time_provenance(),
        CertificateTimeProvenance::Component
    );
    let nak_at: Instant = failure.retained_first_nak_at().ok_or(Failure::Control)?;
    assert!(retained.configuration().same_configuration(expected));
    assert_eq!(retained.committed_at(), committed_at);
    assert_eq!(retained.initial_cookie_population(), (1, 12));
    assert_eq!(retained.packet_bytes(), 252);
    assert_eq!(retained.leap_indicator(), 2);
    assert_peer_timestamps(&retained, FIRST_PEER_TIMESTAMPS);
    assert_peer_quality(&retained, FIRST_DELETION_QUALITY);
    first_timing.assert_projection(&retained);
    let projected_again: crate::ManagedProtectedReceipt =
        failure.retained_first().ok_or(Failure::Control)?;
    assert!(retained.same_exchange(&projected_again));
    assert!(retained.same_ke_association(&projected_again));
    let (client, peer): (std::net::SocketAddr, std::net::SocketAddr) = first_endpoints?;
    assert_eq!(retained.local_endpoint(), client);
    assert_eq!(retained.peer_endpoint(), peer);
    assert!(committed_at <= nak_at && first_commit_window.contains(&nak_at));
    Ok(())
}

/// Match the original wire vector, pre-checkout population and receiving context.
/// # Panics
/// Fail if later work replaces or renews any of the retained original facts.
#[cfg(any(windows, target_os = "linux"))]
fn assert_loopback_first_receipt(
    first: &crate::nts_ke_experiment::datagram::ManagedProtectedHistory,
    remote: std::net::SocketAddr,
    expected: &crate::SourceConfigurationSnapshot,
    first_commit_window: &std::ops::RangeInclusive<Instant>,
    two_replacements: bool,
) {
    let history: &crate::nts_ke_experiment::datagram::ProtectedHistory = first.history();
    let receipt: crate::nts_ke_experiment::association::ledger::Receipt = history.receipt();
    assert!(first.configuration().same_configuration(expected));
    assert_eq!(history.endpoints().1, remote);
    assert_eq!(history.run_context().origin, *first_commit_window.start());
    assert_eq!(receipt.serial(), 1);
    assert_eq!(
        receipt.fields().header().origin_timestamp().bits(),
        0x1122_3344_5566_7788
    );
    let replacements: usize = if two_replacements { 2 } else { 1 };
    assert_eq!(receipt.cookie_counts(), (replacements, replacements));
    assert_eq!(history.population(), (replacements, 12 * replacements));
    assert_eq!(
        first.summary(),
        &Summary {
            cookies: 1,
            cookie_bytes: 12,
            packet_bytes: 252,
            transmitted: true,
        }
    );
    assert!(first_commit_window.contains(&receipt.accepted_at()));
    assert!(first_commit_window.contains(&history.committed_at()));
    assert!(receipt.accepted_at() <= history.committed_at());
}

/// Compare the genuine applied control with the independent fixed peer's decision.
/// # Panics
/// Fail if protection, policy scope or the successor ordinal is lost.
#[cfg(windows)]
fn assert_loopback_deny(
    history: &AppliedProtectedControl,
    remote: std::net::SocketAddr,
    expected: &crate::SourceConfigurationSnapshot,
) {
    use crate::nts_ke_experiment::response::ControlProtection;

    assert_eq!(history.kind(), ProtectedControlKind::Deny);
    assert_eq!(history.protection(), ControlProtection::Authenticated);
    assert_eq!(history.remote(), remote);
    assert_eq!(history.serial(), 2);
    assert!(history.configuration().same_configuration(expected));
}

/// Require the real normally joined native fixture result.
/// # Errors
/// Preserve original unexpected operation failures and refuse the wrong disposition.
/// # Panics
/// Fail if the original control differs from the fixed peer's decision.
#[cfg(windows)]
fn assert_native_loopback_control(
    result: Result<ReceivedInvocation, RunFailed>,
    remote: std::net::SocketAddr,
    expected: &crate::SourceConfigurationSnapshot,
) -> Result<(), Failure> {
    match result {
        Ok(ReceivedInvocation::ProtectedControl(joined)) => {
            let history: AppliedProtectedControl = joined.into_history();
            assert_loopback_deny(&history, remote, expected);
        }
        Err(RunFailed::Operation(failure)) => return Err(failure.primary),
        _ => return Err(Failure::Control),
    }
    Ok(())
}

/// Refuse a different Control even when its timing and actual Stop scope match.
/// # Errors
/// Preserve checked fixture setup and actual joined receiving refusals.
/// # Panics
/// Fail if attaching the original writer admits the sibling's producer or history.
#[test]
fn successor_writer_refuses_equal_value_foreign_control_before_producer() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let earlier: Instant = Instant::now()
        .checked_sub(Duration::from_millis(20))
        .ok_or(Failure::Deadline)?;
    let (config, _source, _authority): (Config, crate::SourceHandle, crate::SourcePolicyAuthority) =
        managed_attempt_config(earlier, Duration::from_millis(4), Duration::from_millis(8))?;
    let result: Result<ReceivedInvocation, RunFailed> = run_joined_with(
        config,
        &SLOT,
        None,
        |slot: &'static WorkerSlot, prepared: PreparedInvocation| {
            slot.submit_successor_fixture(
                prepared,
                &ADMISSION,
                component_provider,
                |config, provider, original, writer| {
                    let sibling: Control = Control::clamped_in(
                        original.origin,
                        original.expiry,
                        Arc::clone(&original.stop_scope),
                    )?;
                    assert_eq!(sibling.origin, original.origin);
                    assert_eq!(sibling.expiry, original.expiry);
                    assert_eq!(sibling.handshake_deadline, original.handshake_deadline);
                    assert!(Arc::ptr_eq(&sibling.stop_scope, &original.stop_scope));
                    let mut producer: ScriptProducer<'_> =
                        ScriptProducer::new(config, provider, vec![Ok(())], ExchangeCase::Healthy);
                    producer.managed_control = Some(ProtectedControlKind::Deny);
                    let outcome: Operation =
                        drive_successor_fixture(config, provider, sibling, &mut producer, writer);
                    assert_eq!((producer.establish_calls, producer.exchange_calls), (0, 0));
                    outcome
                },
                Hooks::none(),
            )
        },
    );
    assert!(
        matches!(result, Err(RunFailed::Operation(actual)) if actual == failed(Failure::Control))
    );
    assert_eq!(
        SLOT.observer().poll_until(Instant::now() + WAIT),
        Observation::Taken
    );
    Ok(())
}

/// Joined exact DENY preserves the first real-socket receipt's measurement.
/// # Errors
/// Preserve fixture or actual worker failure.
#[cfg(any(windows, target_os = "linux"))]
#[test]
fn managed_loopback_successor_commits_exact_deny_after_normal_join() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    run_managed_loopback_control(&SLOT, &ADMISSION, LoopbackSuccessor::ApplyDeny)
}

/// An admitted NAK leaves the original pending opportunity for an ordinary reply.
/// # Errors
/// Preserve fixture or actual worker failure.
#[cfg(any(windows, target_os = "linux"))]
#[test]
fn managed_loopback_successor_preserves_nak_and_ordinary_reply_after_join() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    run_managed_loopback_control(&SLOT, &ADMISSION, LoopbackSuccessor::ApplyOrdinary)
}

/// Compare both original ordinary exchanges through the joined numerical consumer.
/// # Errors
/// Preserve checked fixture, worker and numerical failures.
/// # Panics
/// Fail on dropped declarations, numerical fabrication or a lost original refusal.
#[cfg(any(windows, target_os = "linux"))]
#[test]
fn managed_loopback_measures_both_joined_ordinary_receipts() -> Result<(), Failure> {
    static SLOTS: [WorkerSlot; 4] = [const { WorkerSlot::new() }; 4];
    static ADMISSIONS: [ProcessAdmission; 4] = [const { ProcessAdmission::new() }; 4];
    struct Scenario {
        measurement: Option<LoopbackMeasurement>,
        refusal: Option<MeasurementRefusal>,
    }
    let valid: ExperimentalProfile = valid_loopback_measurement_profile()?;
    let wrong_date: ExperimentalProfile =
        ExperimentalProfile::new(0, 0, LOOPBACK_MEASUREMENT_ALLOWANCE_NS)
            .map_err(|_| Failure::Control)?;
    let scenarios: [Scenario; 4] = [
        Scenario {
            measurement: None,
            refusal: Some(MeasurementRefusal::Unavailable),
        },
        Scenario {
            measurement: Some(LoopbackMeasurement {
                profile: wrong_date,
                scale: SourceScale::UnsmearedUtc,
            }),
            refusal: Some(MeasurementRefusal::DateScope),
        },
        Scenario {
            measurement: Some(LoopbackMeasurement {
                profile: valid,
                scale: SourceScale::Smeared,
            }),
            refusal: Some(MeasurementRefusal::SourceScaleConflict),
        },
        Scenario {
            measurement: Some(LoopbackMeasurement {
                profile: valid,
                scale: SourceScale::UnsmearedUtc,
            }),
            refusal: None,
        },
    ];
    for (
        (slot, admission),
        Scenario {
            measurement,
            refusal,
        },
    ) in SLOTS.iter().zip(&ADMISSIONS).zip(scenarios)
    {
        run_managed_loopback_observing(
            slot,
            admission,
            LoopbackSuccessor::ApplyOrdinary,
            true,
            measurement,
            || Ok(()),
            |report, _: [LoopbackPeerLeg<()>; 2]| {
                let report: &ManagedProtectedReport = report.ok_or(Failure::Control)?;
                for result in [report.first_measurement(), report.terminal_measurement()] {
                    let result: Result<ExperimentalMeasurement, MeasurementRefusal> =
                        result.ok_or(Failure::Control)?;
                    if let Some(expected) = refusal {
                        assert_eq!(result, Err(expected));
                    } else {
                        let measurement: ExperimentalMeasurement =
                            result.map_err(|_| Failure::Control)?;
                        assert_eq!(
                            measurement.declared_limit_ns(),
                            LOOPBACK_MEASUREMENT_ALLOWANCE_NS
                        );
                    }
                }
                assert_ordinary_joined_quality(report, refusal)
            },
        )?;
    }
    Ok(())
}

/// Declare broad test-only time limits so the literal peer timestamps reach the Q1 checks.
/// # Errors
/// Preserve a refusal from the real checked quality declaration.
#[cfg(any(windows, target_os = "linux"))]
fn loopback_quality_assumptions(
    maximum_advertised_distance: f64,
) -> Result<DeclaredQualityAssumptions, Failure> {
    let frequency: DeclaredFrequencyInterval =
        DeclaredFrequencyInterval::new(-1.0 / 64.0, 1.0 / 64.0).map_err(|_| Failure::Control)?;
    let limits: DeclaredQualityLimits = DeclaredQualityLimits::new(
        maximum_advertised_distance,
        1_073_741_824.0,
        1_073_741_824.0,
    )
    .map_err(|_| Failure::Control)?;
    DeclaredQualityAssumptions::new(1.0 / 1_024.0, 1.0 / 256.0, 1.0 / 512.0, frequency, limits)
        .map_err(|_| Failure::Control)
}

/// Check both actual joined receipts and each original measurement refusal.
/// # Errors
/// Preserve missing receipt, numeric refusal and independent timing-oracle failure.
/// # Panics
/// Fail on a wrong measurement error or a root-limit comparison that is not strict.
#[cfg(any(windows, target_os = "linux"))]
fn assert_ordinary_joined_quality(
    report: &ManagedProtectedReport,
    refusal: Option<MeasurementRefusal>,
) -> Result<(), Failure> {
    use crate::ManagedProtectedTerminal;

    let assumptions: DeclaredQualityAssumptions = loopback_quality_assumptions(131_072.0)?;
    let first: &crate::ManagedProtectedReceipt = report.first().ok_or(Failure::Control)?;
    let ManagedProtectedTerminal::Ordinary(terminal) = report.terminal() else {
        return Err(Failure::Control);
    };
    for (receipt, outcome, claims, timestamps) in [
        (
            first,
            report.first_quality(&assumptions),
            (5, -23, 0x8123_4567, 0xf102_0304),
            FIRST_PEER_TIMESTAMPS,
        ),
        (
            terminal,
            report.terminal_quality(&assumptions),
            (7, -19, 0xcafe_4321, 0xa0b0_9080),
            [0x4455_6677_8899_aabb, 0x5566_7788_99aa_bbcc],
        ),
    ] {
        let outcome: Result<
            ConditionalQuality<'_, crate::ManagedProtectedReceipt>,
            QualityRefusal,
        > = outcome.ok_or(Failure::Control)?;
        if let Some(expected) = refusal {
            assert!(
                matches!(outcome, Err(QualityRefusal::Measurement(actual)) if actual == expected)
            );
        } else {
            let quality: ConditionalQuality<'_, crate::ManagedProtectedReceipt> =
                outcome.map_err(|_| Failure::Control)?;
            assert_joined_quality(&quality, receipt, &assumptions, claims, timestamps)?;
        }
    }
    if refusal.is_none() {
        // First distance equals the limit exactly. The terminal uses smaller claims.
        let boundary: f64 =
            f64::from(0x8123_4567_u32) / 131_072.0 + f64::from(0xf102_0304_u32) / 65_536.0;
        let assumptions: DeclaredQualityAssumptions = loopback_quality_assumptions(boundary)?;
        assert!(matches!(
            report.first_quality(&assumptions),
            Some(Err(QualityRefusal::AdvertisedDistance))
        ));
        assert!(matches!(report.terminal_quality(&assumptions), Some(Ok(_))));
    }
    Ok(())
}

/// Compare receipt identity and computed quality with literal wire fields and brackets.
/// # Errors
/// Preserve a failed independent wall conversion from the original joined receipt.
/// # Panics
/// Fail on cross-leg substitution, rounded-first RTT or changed signed wire interpretation.
#[cfg(any(windows, target_os = "linux"))]
fn assert_joined_quality(
    quality: &ConditionalQuality<'_, crate::ManagedProtectedReceipt>,
    receipt: &crate::ManagedProtectedReceipt,
    assumptions: &DeclaredQualityAssumptions,
    (_, precision, root_delay_bits, root_dispersion_bits): (u8, i8, u32, u32),
    [receive, transmit]: [u64; 2],
) -> Result<(), Failure> {
    assert!(std::ptr::eq(quality.original_receipt(), receipt));
    assert!(std::ptr::eq(quality.declared_assumptions(), assumptions));
    assert_eq!(
        quality.peer_precision_seconds().to_bits(),
        2.0_f64.powi(i32::from(precision)).to_bits()
    );
    assert_eq!(
        quality.advertised_root_delay_seconds().to_bits(),
        (f64::from(root_delay_bits) / 65_536.0).to_bits()
    );
    assert_eq!(
        quality.advertised_root_dispersion_seconds().to_bits(),
        (f64::from(root_dispersion_bits) / 65_536.0).to_bits()
    );

    let [send, _, read]: [SoftwareClock; 3] = receipt.application_clock_captures();
    let local_ns: i128 =
        numerics_tests::ntp_nanoseconds(read.utc())? - numerics_tests::ntp_nanoseconds(send.utc())?;
    let server_q32: i128 = i128::from(transmit) - i128::from(receive);
    let raw_numerator: i128 = (local_ns << 32) - 1_000_000_000 * server_q32;
    let raw: time_domain::RoundedRationalSeconds = quality.raw_round_trip();
    assert!(raw_numerator < 0);
    assert_eq!(raw.sign(), time_domain::RationalSign::Negative);
    assert_eq!(raw.numerator_magnitude(), raw_numerator.unsigned_abs());
    assert_eq!(raw.denominator().get(), 4_294_967_296_000_000_000);
    Ok(())
}

/// Pair the genuine joined application captures with an independently timed peer.
/// Both authenticated replies cross the real Windows UDP adapter.
/// # Errors
/// Preserve the fixture and actual worker failures or refuse absent clocks.
#[cfg(windows)]
#[test]
fn managed_loopback_ordinary_legs_preserve_native_capture_placement() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    run_managed_loopback_observing(
        &SLOT,
        &ADMISSION,
        LoopbackSuccessor::ApplyOrdinary,
        false,
        None,
        || {
            read_clock_coordinates().map_err(|error: ClockReadError| {
                eprintln!("native loopback peer clock read failed: {error:?}");
                Failure::Control
            })
        },
        verify_native_loopback_timing,
    )
}

/// Require both ordinary receipts from the actual join and pair each with its peer leg.
/// # Errors
/// Refuse absent first/ordinary terminal history or a missing native observation.
/// # Panics
/// Fail if either leg lacks independent placement evidence, repeats the first
/// exchange, or overlaps the other leg.
#[cfg(windows)]
fn verify_native_loopback_timing(
    report: Option<&ManagedProtectedReport>,
    [peer_first, peer_second]: [LoopbackPeerLeg<ClockCoordinates>; 2],
) -> Result<(), Failure> {
    let report: &ManagedProtectedReport = report.ok_or(Failure::Control)?;
    let first: &crate::ManagedProtectedReceipt = report.first().ok_or(Failure::Control)?;
    let crate::ManagedProtectedTerminal::Ordinary(second) = report.terminal() else {
        return Err(Failure::Control);
    };
    assert!(first.same_invocation(second));
    assert!(!first.same_exchange(second));
    let first_points: [ClockCoordinates; 3] =
        verify_native_loopback_leg("first", first, peer_first)?;
    let second_points: [ClockCoordinates; 3] =
        verify_native_loopback_leg("successor", second, peer_second)?;
    assert_eq!(
        first_points[2].frequency_hz(),
        second_points[0].frequency_hz()
    );
    assert!(
        first_points[2].qpc_after() <= second_points[0].qpc_before(),
        "first receive-return must complete before the worker's successor pre-send"
    );
    Ok(())
}

/// Compare one leg's original Engine readings with observations on the actual peer.
/// # Errors
/// Refuse absent, refused or unrepresentable native history at any Engine point.
/// # Panics
/// Fail on different socket endpoints or frequencies, reversed same-thread
/// intervals, or an ambiguous or reversed comparison between the two threads.
#[cfg(windows)]
fn verify_native_loopback_leg(
    leg: &str,
    receipt: &crate::ManagedProtectedReceipt,
    peer: LoopbackPeerLeg<ClockCoordinates>,
) -> Result<[ClockCoordinates; 3], Failure> {
    use crate::ProtectedNativeClockObservation::Recorded;

    let [
        Some(Recorded(pre_send)),
        Some(Recorded(send_return)),
        Some(Recorded(receive_return)),
    ] = receipt.application_native_clock_captures()
    else {
        return Err(Failure::Control);
    };
    let span: crate::ProtectedNativeCaptureSpan = receipt
        .native_capture_span()
        .map_err(|_| Failure::Control)?;
    assert!(
        span.captures() == [pre_send, send_return, receive_return],
        "{leg}"
    );
    assert_eq!(span.frequency_hz(), pre_send.frequency_hz(), "{leg}");
    assert_eq!(
        span.qpc_bounds(),
        [pre_send.qpc_before(), receive_return.qpc_after()],
        "{leg}",
    );
    assert_eq!(receipt.local_endpoint(), peer.client_endpoint, "{leg}");
    assert_eq!(receipt.peer_endpoint(), peer.peer_endpoint, "{leg}");
    let points: [(&str, ClockCoordinates); 5] = [
        ("engine-pre-send", pre_send),
        ("engine-send-return", send_return),
        ("peer-receive-return", peer.received),
        ("peer-pre-send", peer.sending),
        ("engine-receive-return", receive_return),
    ];
    for (point, coordinates) in points {
        assert_eq!(
            coordinates.frequency_hz(),
            pre_send.frequency_hz(),
            "{leg} {point}: different QPC frequency"
        );
        assert!(
            coordinates.qpc_before() <= coordinates.qpc_after(),
            "{leg} {point}: reversed same-thread bracket"
        );
    }

    assert!(pre_send.qpc_after() <= send_return.qpc_before(), "{leg}");
    assert!(
        send_return.qpc_after() <= receive_return.qpc_before(),
        "{leg}"
    );
    assert!(
        peer.received.qpc_after() <= peer.sending.qpc_before(),
        "{leg}"
    );
    assert_cross_thread_qpc_order(
        leg,
        "Engine pre-send to peer receive-return",
        pre_send.qpc_after(),
        peer.received.qpc_before(),
    );
    assert_cross_thread_qpc_order(
        leg,
        "peer pre-send to Engine receive-return",
        peer.sending.qpc_after(),
        receive_return.qpc_before(),
    );

    for (point, coordinates) in points {
        println!(
            "loopback-native leg={leg} point={point} frequency_hz={} qpc_before={} qpc_after={} filetime_100ns={}",
            coordinates.frequency_hz(),
            coordinates.qpc_before(),
            coordinates.qpc_after(),
            coordinates.filetime_100ns()
        );
    }
    Ok([pre_send, send_return, receive_return])
}

/// Require an observed cross-thread QPC separation larger than one tick.
/// # Panics
/// Fail separately on ambiguous or demonstrably reversed readings.
#[cfg(windows)]
fn assert_cross_thread_qpc_order(leg: &str, interval: &str, earlier: i64, later: i64) {
    let ticks: i128 = i128::from(later) - i128::from(earlier);
    assert!(
        !(-1..=1).contains(&ticks),
        "{leg} {interval}: ambiguous cross-thread QPC order ({ticks} ticks)"
    );
    assert!(
        ticks >= 2,
        "{leg} {interval}: reversed cross-thread QPC order ({ticks} ticks)"
    );
}

/// Equal-value replacement refuses the retired control and preserves first measurement.
/// # Errors
/// Preserve fixture failure or unexpected typed refusal.
#[cfg(any(windows, target_os = "linux"))]
#[test]
fn managed_loopback_successor_refuses_same_values_replacement_before_commit() -> Result<(), Failure>
{
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    run_managed_loopback_control(&SLOT, &ADMISSION, LoopbackSuccessor::ReplaceConfiguration)
}

/// Unwinding before the second decision retains only the genuine first receipt.
/// # Errors
/// Preserve fixture or actual worker receiving failure.
/// # Panics
/// Fail if joining the worker destroys the original opaque unwind payload.
#[cfg(any(windows, target_os = "linux"))]
#[test]
fn managed_loopback_successor_unwind_preserves_first_receipt_after_join() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    run_managed_loopback_control(&SLOT, &ADMISSION, LoopbackSuccessor::Unwind)?;
    assert_eq!(SUCCESSOR_UNWIND_DROPS.load(Ordering::Acquire), 0);
    Ok(())
}

/// Admit only the fixed owned fixture's numeric IPv4 loopback endpoint.
/// # Errors
/// Refuse missing, non-Unicode, oversized, nonloopback or zero-port input before a
/// native observation/provider/worker can start.
#[cfg(windows)]
fn native_successor_endpoint(name: &str) -> Result<std::net::SocketAddr, Failure> {
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

    let value: std::ffi::OsString = std::env::var_os(name).ok_or(Failure::Arguments)?;
    if value.len() > 32 {
        return Err(Failure::Arguments);
    }
    let endpoint: SocketAddrV4 = value
        .to_str()
        .ok_or(Failure::Arguments)?
        .parse()
        .map_err(|_| Failure::Arguments)?;
    if endpoint.ip() != &Ipv4Addr::LOCALHOST || endpoint.port() == 0 {
        return Err(Failure::Endpoint);
    }
    Ok(SocketAddr::V4(endpoint))
}

/// Verify full native establishment, authenticated UDP continuation and joined DENY.
/// The explicitly selected peer and trust fixture must already be owned by the
/// disposable native test controller. No test clock or verifier is substituted.
/// # Errors
/// Preserve setup, normal worker operation and bounded output refusals.
/// # Panics
/// Fail if the worker or observer is not retired or the exact policy effect differs.
#[cfg(windows)]
#[test]
#[ignore = "requires the owned offline Windows TLS and UDP successor peer"]
fn native_provider_managed_loopback_successor_commits_exact_deny_after_normal_join()
-> Result<(), Failure> {
    use std::io::{self, Write};
    use std::net::SocketAddr;

    use crate::nts_ke_experiment::{report_run_failure, subscribe_power_observer};
    use crate::{
        KePreviewTarget, SourceAddressSet, SourceConfigurationSnapshot, SourceHandle,
        SourcePolicyAuthority, SourcePolicyLimits,
    };

    static SLOT: WorkerSlot = WorkerSlot::new();
    let ke: SocketAddr = native_successor_endpoint("NV_NTS_SUCCESSOR_KE_PEER")?;
    let udp: SocketAddr = native_successor_endpoint("NV_NTS_SUCCESSOR_UDP_PEER")?;
    let target: KePreviewTarget =
        KePreviewTarget::new(ke, "ke-a7e40ec4eade477dbaba6ca9e9f975a4.invalid")
            .map_err(|_| Failure::Arguments)?;
    let limits: SourcePolicyLimits =
        SourcePolicyLimits::new(2, 2, Duration::from_millis(1), Duration::from_millis(4))
            .map_err(Failure::SourcePolicy)?;
    let mut authority: SourcePolicyAuthority =
        crate::source_policy::isolated_authority(limits).map_err(Failure::SourcePolicy)?;
    let source: SourceHandle = authority.source(0).map_err(Failure::SourcePolicy)?;
    let alias: SourceHandle = authority.source(1).map_err(Failure::SourcePolicy)?;
    let addresses: SourceAddressSet =
        SourceAddressSet::new(&[udp]).map_err(Failure::SourcePolicy)?;
    let expected: SourceConfigurationSnapshot = authority
        .install_ke_preview_configuration(&source, addresses, target)
        .map_err(Failure::SourcePolicy)?;
    let config: Config = Config::from_managed_protected(
        authority
            .authorize_protected_ke(&expected)
            .map_err(Failure::SourcePolicy)?,
    );
    let config_lifetime: Weak<()> = Arc::downgrade(&config.lifetime);
    let observed: ObservedInvocation = run_observing(
        config,
        &SLOT,
        subscribe_power_observer,
        SubmissionReservation::submit_native_successor_fixture,
    )
    .map_err(|error: ObserverStartError| {
        let failure: RunFailed = match error {
            ObserverStartError::Preparation(failure) => failure,
            ObserverStartError::Registration => RunFailed::PowerRegistration,
        };
        let _reported: std::process::ExitCode = report_run_failure(&failure);
        Failure::Control
    })?;
    if let Err(failure) = &observed.result {
        let _reported: std::process::ExitCode = report_run_failure(failure);
    }
    assert_native_loopback_control(observed.result, udp, &expected)?;
    assert_loopback_policy(&source, &alias, udp, true)?;
    assert!(!observed.cleanup.stop_failed);
    assert_eq!(observed.cleanup.unregistration_error, None);
    assert_eq!(config_lifetime.strong_count(), 0);
    let inspect_until: Instant = Instant::now().checked_add(WAIT).ok_or(Failure::Deadline)?;
    assert_eq!(
        SLOT.observer().poll_until(inspect_until),
        Observation::Taken
    );
    assert_eq!(SLOT.sealed_count(), Ok(0));

    writeln!(
        io::stdout().lock(),
        "scope=engine-ke-prototype eligible=0 fresh_sync=0 native_successor=deny requests=2 replies=2 retained_replacements=1 worker_joined=1"
    )
    .map_err(|_| Failure::Control)
}

/// Closed actual-owner retirement choices; neither simulates native cleanup.
#[derive(Clone, Copy)]
enum HistoryRetirement {
    Ordinary,
    Panics,
}

/// Drive one actual establishment and the selected same-association managed step.
/// # Errors
/// Preserve the existing invocation owner's operation and cleanup facts.
/// # Panics
/// Fail if the scripted managed step enters another establishment or a preview.
fn scripted_managed_control_operation<'scope>(
    config: &'scope Config,
    provider: &'scope ProviderScope<'scope>,
    control: Control,
    kind: ProtectedControlKind,
) -> Operation {
    let original_expiry: Instant = control.expiry;
    let mut producer: ScriptProducer<'_> =
        ScriptProducer::new(config, provider, vec![Ok(())], ExchangeCase::Healthy);
    producer.managed_control = Some(kind);
    let result: Operation = drive_invocation_with(config, provider, control, &mut producer);
    assert_eq!(
        (
            producer.establish_calls,
            producer.exchange_calls,
            producer.preview_calls
        ),
        (1, 1, 0)
    );
    assert!(matches!(
        producer.controls.as_slice(),
        [observed] if observed.expiry == original_expiry
    ));
    result
}

/// Exercise the actual managed two-request step through submission and joined receiving.
/// # Errors
/// Preserve source, assertion fixture, worker or receiving refusal.
/// # Panics
/// Fail if the real retired control loses its history or enters the wrong result path.
fn run_managed_control_history(
    slot: &'static WorkerSlot,
    admission: &'static ProcessAdmission,
    retirement: HistoryRetirement,
    expected_kind: ProtectedControlKind,
) -> Result<(), Failure> {
    use std::net::SocketAddr;

    use crate::{
        SourceConfigurationSnapshot, SourceHandle, SourcePolicyAuthority, SourcePolicySnapshot,
    };

    let earlier: Instant = Instant::now()
        .checked_sub(Duration::from_millis(20))
        .ok_or(Failure::Deadline)?;
    let (config, source, _authority): (Config, SourceHandle, SourcePolicyAuthority) =
        managed_attempt_config(earlier, Duration::from_millis(4), Duration::from_millis(8))?;
    let definition: SourceConfigurationSnapshot = source
        .configuration_snapshot()
        .map_err(Failure::SourcePolicy)?
        .ok_or(Failure::Control)?;
    let remote: SocketAddr = SocketAddr::new(config.peer.ip(), 123);
    let before: SourcePolicySnapshot = source
        .policy_snapshot(remote)
        .map_err(Failure::SourcePolicy)?;
    let config_lifetime: Weak<()> = Arc::downgrade(&config.lifetime);
    let mut hooks: Hooks = Hooks::none();
    if matches!(retirement, HistoryRetirement::Panics) {
        hooks.before_retire = Some(Box::new(|| {
            std::panic::resume_unwind(Box::new(()));
        }));
    }
    let started: Instant = Instant::now();
    let result: Result<ReceivedInvocation, RunFailed> = run_joined_with(
        config,
        slot,
        None,
        |slot: &'static WorkerSlot, prepared: PreparedInvocation| {
            slot.submit_fixture(
                prepared,
                admission,
                component_provider,
                move |config, provider, control| {
                    scripted_managed_control_operation(config, provider, control, expected_kind)
                },
                hooks,
            )
        },
    );
    let history: AppliedProtectedControl = match (retirement, result) {
        (HistoryRetirement::Ordinary, Ok(ReceivedInvocation::ProtectedControl(joined))) => {
            joined.into_history()
        }
        (HistoryRetirement::Panics, Err(RunFailed::Abnormal(joined))) => {
            assert_eq!(joined.termination, Termination::Panicked);
            assert!(!joined.retired);
            assert_eq!(slot.sealed_count().map_err(|_| Failure::Control)?, 1);
            let completion: InvocationCompletion = joined
                .operation
                .ok_or(Failure::Control)?
                .map_err(|failure: Failed| failure.primary)?;
            assert!(completion.summary().is_none());
            let InvocationCompletion::ManagedProtectedControl(history) = completion else {
                return Err(Failure::Control);
            };
            history
        }
        _ => return Err(Failure::Control),
    };
    assert_eq!(history.kind(), expected_kind);
    assert!(history.configuration().same_configuration(&definition));
    assert_eq!(history.remote(), remote);
    assert_eq!(history.serial(), 2);
    assert!((started..=Instant::now()).contains(&history.decided_at()));
    let after: SourcePolicySnapshot = source
        .policy_snapshot(remote)
        .map_err(Failure::SourcePolicy)?;
    let refused: bool = expected_kind != ProtectedControlKind::Rate;
    assert_eq!(
        (after.source().refused(), after.endpoint().refused()),
        (refused, refused)
    );
    assert_eq!(
        after
            .source()
            .revision()
            .checked_sub(before.source().revision()),
        Some(3)
    );
    assert_eq!(
        after
            .endpoint()
            .revision()
            .checked_sub(before.endpoint().revision()),
        Some(3)
    );
    assert_eq!(config_lifetime.strong_count(), 0);
    let inspect_until: Instant = Instant::now().checked_add(WAIT).ok_or(Failure::Deadline)?;
    assert_eq!(
        slot.observer().poll_until(inspect_until),
        Observation::Taken
    );
    Ok(())
}

/// Preserve the three exact control histories through normal joins and DENY during unwind.
/// # Errors
/// Preserve any failure from the real joined component worker.
#[test]
fn managed_deny_worker_retains_exact_history_without_numerical_projection() -> Result<(), Failure> {
    static NORMAL: WorkerSlot = WorkerSlot::new();
    static ABNORMAL: WorkerSlot = WorkerSlot::new();
    static NORMAL_ADMISSION: ProcessAdmission = ProcessAdmission::new();
    static ABNORMAL_ADMISSION: ProcessAdmission = ProcessAdmission::new();
    static RESTRICTED: WorkerSlot = WorkerSlot::new();
    static RESTRICTED_ADMISSION: ProcessAdmission = ProcessAdmission::new();
    static THROTTLED: WorkerSlot = WorkerSlot::new();
    static THROTTLED_ADMISSION: ProcessAdmission = ProcessAdmission::new();
    for (slot, admission, retirement, kind) in [
        (
            &NORMAL,
            &NORMAL_ADMISSION,
            HistoryRetirement::Ordinary,
            ProtectedControlKind::Deny,
        ),
        (
            &ABNORMAL,
            &ABNORMAL_ADMISSION,
            HistoryRetirement::Panics,
            ProtectedControlKind::Deny,
        ),
        (
            &RESTRICTED,
            &RESTRICTED_ADMISSION,
            HistoryRetirement::Ordinary,
            ProtectedControlKind::Restrict,
        ),
        (
            &THROTTLED,
            &THROTTLED_ADMISSION,
            HistoryRetirement::Ordinary,
            ProtectedControlKind::Rate,
        ),
    ] {
        run_managed_control_history(slot, admission, retirement, kind)?;
    }
    Ok(())
}

/// A Stop after the recorded effect cannot undo it or expose it before actual join.
/// # Errors
/// Preserve source setup, real worker, Stop and receiving refusal.
/// # Panics
/// Fail if a held or already-taken slot exposes a result, or the late Stop erases
/// the historical denial or changes the policy already applied by that denial.
#[test]
fn managed_deny_worker_post_record_stop_preserves_only_once_joined_history() -> Result<(), Failure>
{
    use std::net::SocketAddr;

    use crate::nts_ke_experiment::job::ReceivingOutcome;
    use crate::{
        ExperimentalProfile, SourceConfigurationSnapshot, SourceHandle, SourcePolicyAuthority,
        SourcePolicySnapshot,
    };

    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let earlier: Instant = Instant::now()
        .checked_sub(Duration::from_millis(20))
        .ok_or(Failure::Deadline)?;
    let (config, source, _authority): (Config, SourceHandle, SourcePolicyAuthority) =
        managed_attempt_config(earlier, Duration::from_millis(4), Duration::from_millis(8))?;
    let definition: SourceConfigurationSnapshot = source
        .configuration_snapshot()
        .map_err(Failure::SourcePolicy)?
        .ok_or(Failure::Control)?;
    let remote: SocketAddr = SocketAddr::new(config.peer.ip(), 123);
    let config_lifetime: Weak<()> = Arc::downgrade(&config.lifetime);
    let (prepared, stop): (PreparedInvocation, StopHandle) =
        PreparedInvocation::prepare(config).map_err(|failure: Failed| failure.primary)?;
    let (hold, gate): (Box<dyn FnOnce() + Send>, Gate) = owner_gate();
    let mut hooks: Hooks = Hooks::none();
    hooks.before_retire = Some(hold);
    SLOT.submit_fixture(
        prepared,
        &ADMISSION,
        component_provider,
        |config, provider, control| {
            scripted_managed_control_operation(
                config,
                provider,
                control,
                ProtectedControlKind::Deny,
            )
        },
        hooks,
    )
    .map_err(|_| Failure::Control)?;
    gate.wait(&SLOT)?;
    assert_eq!(config_lifetime.strong_count(), 1);
    assert!(matches!(
        SLOT.take_for_receiving(),
        Err(CollectError::NotReady)
    ));
    let recorded: SourcePolicySnapshot = source
        .policy_snapshot(remote)
        .map_err(Failure::SourcePolicy)?;
    assert!(recorded.source().refused() && recorded.endpoint().refused());
    assert_eq!(stop.request_stop()?, StopDisposition::Latched);
    gate.release()?;
    collect_available(&SLOT).map_err(|_| Failure::Control)?;
    assert_eq!(
        SLOT.observer().poll_until(Instant::now() + WAIT),
        Observation::Joined
    );
    let joined: ReceivingOutcome = SLOT.take_for_receiving().map_err(|_| Failure::Control)?;
    let profile: ExperimentalProfile =
        ExperimentalProfile::new(0, u64::from(u32::MAX), 0).map_err(|_| Failure::Control)?;
    let ReceivedInvocation::ProtectedControl(received) =
        receive_joined(joined, Some(profile)).map_err(|_| Failure::Control)?
    else {
        return Err(Failure::Control);
    };
    let history: &AppliedProtectedControl = received.history();
    assert_eq!(history.kind(), ProtectedControlKind::Deny);
    assert!(history.configuration().same_configuration(&definition));
    assert_eq!(history.remote(), remote);
    assert_eq!(history.serial(), 2);
    assert!(history.decided_at() <= recorded.captured_at());
    let after: SourcePolicySnapshot = source
        .policy_snapshot(remote)
        .map_err(Failure::SourcePolicy)?;
    assert!(after.source().refused() && after.endpoint().refused());
    assert_eq!(after.source().revision(), recorded.source().revision());
    assert_eq!(after.endpoint().revision(), recorded.endpoint().revision());
    assert_eq!(config_lifetime.strong_count(), 0);
    assert!(matches!(
        SLOT.take_for_receiving(),
        Err(CollectError::NotReady)
    ));
    assert_eq!(SLOT.collect(), Err(CollectError::NotReady));
    Ok(())
}

/// Compare the actual retained payload with its pre-record observation and literals.
/// No raw constructor, fresh timestamp or result publication is performed here.
/// # Panics
/// Fail if original bytes, captures, ordinal, time or identities change during transfer.
fn assert_retained_history(
    history: &crate::nts_ke_experiment::datagram::ProtectedHistory,
    expected: &super::ProtectedWitness,
    original_chart: &Weak<RunClockContext>,
    original_origin: Instant,
    peer_times: (u64, u64),
) {
    let receipt = history.receipt();
    assert_eq!(
        history.population(),
        expected.population,
        "KE-HIST-01-original-pool-population"
    );
    assert_eq!(
        receipt.fields().header(),
        expected.receipt.fields().header(),
        "KE-HIST-01-full-header"
    );
    assert_eq!(
        (
            receipt.fields().receive().bits(),
            receipt.fields().transmit().bits()
        ),
        peer_times,
        "KE-HIST-07-exact-peer-bits"
    );
    assert_eq!(receipt.serial(), expected.receipt.serial());
    assert_eq!(
        receipt.accepted_at(),
        expected.receipt.accepted_at(),
        "KE-HIST-01-original-packet-time"
    );
    assert_eq!(receipt.local_t1(), None);
    let captures: [SoftwareClock; 3] = history.captures();
    for (actual, expected) in captures.into_iter().zip(expected.captures) {
        assert_eq!(
            (actual.monotonic(), actual.utc(), actual.monotonic_after()),
            (
                expected.monotonic(),
                expected.utc(),
                expected.monotonic_after()
            ),
            "KE-HIST-01-original-capture"
        );
    }
    assert_eq!(history.endpoints(), expected.endpoints);
    assert!(
        Weak::ptr_eq(&Arc::downgrade(history.identity()), &expected.identity),
        "KE-HIST-01-socket-identity"
    );
    assert!(
        Weak::ptr_eq(&Arc::downgrade(history.run_context()), original_chart),
        "KE-HIST-01-chart-identity"
    );
    assert_eq!(history.run_context().origin, original_origin);
    assert_eq!(expected.identity.strong_count(), 1);
}

/// Observe actual component-resource retirement independently of retained history.
/// # Panics
/// Fail on a missing witness or any backend/provider/Config still owned after join.
fn assert_retired_resources(
    config: &Weak<()>,
    provider: Option<&Weak<()>>,
    backend: Option<&Weak<()>>,
) {
    assert_eq!(
        backend.map(Weak::strong_count),
        Some(0),
        "KE-HIST-01-backend-retired-while-history-lives"
    );
    assert_eq!(provider.map(Weak::strong_count), Some(0));
    assert_eq!(config.strong_count(), 0);
}

/// Confirm that the selected wall-step fixture keeps ordered monotonic brackets.
/// # Panics
/// Fail if the selected fixture loses its genuine stepped-wall placement.
fn assert_original_wall_step(
    history: &crate::nts_ke_experiment::datagram::ProtectedHistory,
    case: ExchangeCase,
) {
    if matches!(case, ExchangeCase::AfterWallStep) {
        let [before, after, read]: [SoftwareClock; 3] = history.captures();
        assert!(before.monotonic_after() <= after.monotonic());
        assert!(after.monotonic_after() <= read.monotonic());
        assert!(after.utc() > read.utc());
    }
}

/// Carry genuine Script facts through the real driver, worker, join and final take.
/// The witness observes actual retired operation output; it cannot issue a result.
/// # Errors
/// Preserve checked input, actual Script/worker and missing-observation failures.
/// # Panics
/// Fail on changed history, refreshed time/identity, false retirement or duplicate take.
fn check_joined_protected_history(
    slot: &'static WorkerSlot,
    admission: &'static ProcessAdmission,
    case: ExchangeCase,
    peer_times: (u64, u64),
    retirement: HistoryRetirement,
) -> Result<(), Failure> {
    static PANIC_DROPS: AtomicUsize = AtomicUsize::new(0);
    let Input {
        prepared,
        config_lifetime,
        ..
    } = input(&ExchangeMode::ProtectedDatagram)?;
    let original_origin = prepared.original_times().0;
    let witness = Arc::new(OnceLock::<super::ProtectedWitness>::new());
    let observed = Arc::clone(&witness);
    let chart: Arc<OnceLock<Weak<RunClockContext>>> = Arc::new(OnceLock::new());
    let observed_chart = Arc::clone(&chart);
    let resource = Arc::new(OnceLock::<Weak<()>>::new());
    let observed_resource = Arc::clone(&resource);
    let provider = Arc::new(OnceLock::<Weak<()>>::new());
    let observed_provider = Arc::clone(&provider);
    let mut hooks = Hooks::none();
    if matches!(retirement, HistoryRetirement::Panics) {
        hooks.before_retire = Some(Box::new(|| {
            std::panic::resume_unwind(Box::new(Payload {
                drops: &PANIC_DROPS,
            }))
        }));
    }
    slot.submit_fixture(
        prepared,
        admission,
        component_provider,
        move |config, provider, control| {
            assert!(observed_provider.set(provider.retirement_witness()).is_ok());
            let mut producer = ScriptProducer::new(config, provider, vec![Ok(())], case);
            let completion = drive_invocation_with(config, provider, control, &mut producer);
            if let Ok(InvocationCompletion::Protected(history)) = &completion {
                assert_eq!(history.run_context().origin, original_origin);
                assert!(
                    observed_chart
                        .set(Arc::downgrade(history.run_context()))
                        .is_ok()
                );
            }
            if let Some(history) = producer.last_protected.take() {
                assert!(observed.set(history).is_ok());
            }
            if let Some(lifetime) = producer.backend_lifetime.take() {
                assert!(observed_resource.set(lifetime).is_ok());
            }
            assert_eq!((producer.establish_calls, producer.exchange_calls), (1, 1));
            completion
        },
        hooks,
    )
    .map_err(|_| Failure::Control)?;
    let outcome = collect(slot)?;
    let expected = witness.get().ok_or(Failure::Control)?;
    let original_chart = chart.get().ok_or(Failure::Control)?;
    let completion = outcome
        .operation
        .ok_or(Failure::Control)?
        .map_err(|failure| failure.primary)?;
    let InvocationCompletion::Protected(history) = &completion else {
        return Err(Failure::Control);
    };
    assert_retained_history(
        history,
        expected,
        original_chart,
        original_origin,
        peer_times,
    );
    assert_original_wall_step(history, case);
    assert_retired_resources(&config_lifetime, provider.get(), resource.get());
    let summary: &Summary = completion.summary().ok_or(Failure::Control)?;
    assert!(summary.transmitted && summary.packet_bytes > 48);
    assert_eq!((summary.cookies, summary.cookie_bytes), (1, 12));
    match retirement {
        HistoryRetirement::Ordinary => {
            assert_eq!(outcome.termination, Termination::Returned);
            assert!(outcome.retired);
        }
        HistoryRetirement::Panics => {
            assert_eq!(
                outcome.termination,
                Termination::Panicked,
                "KE-HIST-04-retirement-panic"
            );
            assert!(!outcome.retired);
            assert_eq!(slot.sealed_count().map_err(|_| Failure::Control)?, 1);
            assert_eq!(PANIC_DROPS.load(Ordering::Acquire), 0);
        }
    }
    assert!(matches!(
        slot.take_fixture_result(),
        Err(CollectError::NotReady)
    ));
    assert!(matches!(admission.claim(), Err(Failure::ProcessUsed)));
    drop(completion);
    assert_eq!(expected.identity.strong_count(), 0);
    assert_eq!(original_chart.strong_count(), 0);
    Ok(())
}

/// Valid-tag zero T2/T3 values and a scripted local wall step remain inert history.
/// # Errors
/// Preserve actual joined Script failures; no numerical/sample constructor is invoked.
/// # Panics
/// Fail if any original bits/captures/identity are lost or cleanup is falsely inferred.
#[test]
fn whole_job_protected_history_retains_nonzero_and_zero_timestamps() -> Result<(), Failure> {
    static SLOTS: [WorkerSlot; 6] = [const { WorkerSlot::new() }; 6];
    static ADMISSIONS: [ProcessAdmission; 6] = [const { ProcessAdmission::new() }; 6];
    let cases: [(ExchangeCase, (u64, u64)); 6] = [
        (
            ExchangeCase::Healthy,
            (0x2233_4455_6677_8899, 0x3344_5566_7788_99aa),
        ),
        (
            ExchangeCase::AfterWallStep,
            (0x2233_4455_6677_8899, 0x3344_5566_7788_99aa),
        ),
        (ExchangeCase::ZeroReceive, (0, 0x3344_5566_7788_99aa)),
        (ExchangeCase::ZeroTransmit, (0x2233_4455_6677_8899, 0)),
        (ExchangeCase::ZeroTimestamps, (0, 0)),
        (
            ExchangeCase::MultipleCookies,
            (0x2233_4455_6677_8899, 0x3344_5566_7788_99aa),
        ),
    ];
    for ((slot, admission), (case, peer_times)) in SLOTS.iter().zip(&ADMISSIONS).zip(cases) {
        check_joined_protected_history(
            slot,
            admission,
            case,
            peer_times,
            HistoryRetirement::Ordinary,
        )?;
    }
    Ok(())
}

/// Later actual worker unwind keeps the original owning protected result available.
/// # Errors
/// Preserve actual checked input, Script and collection failures.
/// # Panics
/// Fail if panic loses recorded history, claims normal retirement or drops sealed data.
#[test]
fn whole_job_protected_history_survives_later_retirement_panic() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    check_joined_protected_history(
        &SLOT,
        &ADMISSION,
        ExchangeCase::Healthy,
        (0x2233_4455_6677_8899, 0x3344_5566_7788_99aa),
        HistoryRetirement::Panics,
    )
}

/// A second actual facade call receives the permanent slot refusal after healthy join.
/// # Errors
/// Preserve actual checked input, Script and synchronous facade failures.
/// # Panics
/// Fail if refusal loses its cause, re-enters Producer or retains unsubmitted input.
#[test]
fn facade_permanent_submission_refusal_preserves_cause_after_healthy_join() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let submissions = AtomicUsize::new(0);
    let producers = Arc::new(AtomicUsize::new(0));
    let config = Config::new("127.0.0.1:4460".into(), "Time.Example".into())?;
    let produced = Arc::clone(&producers);
    let completed = run_synchronously_with(config, &SLOT, None, |slot, prepared| {
        submit_facade_script(slot, prepared, &ADMISSION, &submissions, produced)
    })
    .map_err(|_| Failure::Control)?;
    assert!(completed.protected().is_none());
    assert_eq!(completed.projection().numerical(), "not_attempted");
    assert_eq!(completed.projection().numerical_reason(), "none");
    drop(completed);
    let config = Config::new("127.0.0.1:4460".into(), "Time.Example".into())?;
    let unsubmitted = Arc::downgrade(&config.lifetime);
    let produced = Arc::clone(&producers);
    let refused = run_synchronously_with(config, &SLOT, None, |slot, prepared| {
        submit_facade_script(slot, prepared, &ADMISSION, &submissions, produced)
    });
    let Err(RunFailed::Submission(cause)) = refused else {
        return Err(Failure::Control);
    };
    assert_eq!(
        cause,
        SubmitError::Used,
        "KE-HIST-06-facade-original-submission"
    );
    let registrations: Cell<usize> = Cell::new(0);
    let observed_refusal: Result<ObservedInvocation, ObserverStartError> = run_observing(
        Config::new("127.0.0.1:4460".into(), "Time.Example".into())?,
        &SLOT,
        |_stop: StopHandle| {
            registrations.set(registrations.get() + 1);
            Ok(|| Ok(()))
        },
        |reservation: SubmissionReservation| {
            submit_reserved_facade_script(
                Ok(reservation),
                &ADMISSION,
                &submissions,
                Arc::clone(&producers),
            )
        },
    );
    assert!(matches!(
        observed_refusal,
        Err(ObserverStartError::Preparation(RunFailed::Submission(
            SubmitError::Used
        )))
    ));
    assert_prestart_report(observed_refusal, ("worker-submission", "used"))?;
    assert_eq!(registrations.get(), 0);
    assert_eq!(submissions.load(Ordering::Acquire), 2);
    assert_eq!(producers.load(Ordering::Acquire), 1);
    assert_eq!(unsubmitted.strong_count(), 0);
    assert_eq!(
        SLOT.observer().poll_until(Instant::now() + WAIT),
        Observation::Taken
    );
    assert!(matches!(ADMISSION.claim(), Err(Failure::ProcessUsed)));
    Ok(())
}

/// Normally joined failure retains the original primary and all three secondary facts.
/// # Errors
/// Preserve actual checked construction and genuine whole-call refusal.
/// # Panics
/// Fail if the facade replaces a cause or claims a submission/abnormal disposition.
#[test]
fn facade_actual_operation_failure_preserves_all_original_causes() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let primary: Failed = Failed {
        primary: Failure::Handshake(std::io::ErrorKind::ConnectionReset),
        cleanup: Some(Failure::Shutdown(std::io::ErrorKind::BrokenPipe)),
        admission_cleanup: Some(Failure::Control),
    };
    let config: Config = Config::new("127.0.0.1:4460".into(), "Time.Example".into())?;
    let lifetime: Weak<()> = Arc::downgrade(&config.lifetime);
    let close_code: NonZeroU32 = NonZeroU32::new(5).ok_or(Failure::Control)?;
    let closes: Cell<usize> = Cell::new(0);
    let observed: ObservedInvocation = run_observing(
        config,
        &SLOT,
        |_: StopHandle| {
            Ok(|| {
                closes.set(closes.get() + 1);
                Err(close_code)
            })
        },
        |reservation: SubmissionReservation| {
            reservation.submit_fixture(
                &ADMISSION,
                component_provider,
                move |config: &Config, provider: &ProviderScope<'_>, control: Control| {
                    let mut producer: ScriptProducer<'_> = ScriptProducer::new(
                        config,
                        provider,
                        vec![Err(primary)],
                        ExchangeCase::Healthy,
                    );
                    drive_invocation_with(config, provider, control, &mut producer)
                },
                Hooks::none(),
            )
        },
    )
    .map_err(|_| Failure::Control)?;
    let Err(RunFailed::Operation(actual)) = &observed.result else {
        return Err(Failure::Control);
    };
    assert_eq!(*actual, primary, "KE-HIST-06-facade-original-operation");
    let Err(failure) = SourceProtectedProbeReport::from_observed(Ok(observed)) else {
        return Err(Failure::Control);
    };
    assert_joined_failure_report(&failure, close_code);
    assert_eq!(closes.get(), 1);
    assert_eq!(lifetime.strong_count(), 0);
    assert_eq!(
        SLOT.observer().poll_until(Instant::now() + WAIT),
        Observation::Taken
    );
    Ok(())
}

/// Assert the specific joined failure and allowlisted output after the actual close.
/// # Panics
/// Fail if any disposition, cleanup status or redacted rendering changes.
fn assert_joined_failure_report(failure: &SourceProtectedProbeFailure, close_code: NonZeroU32) {
    assert_eq!(failure.labels(), ("handshake", "io"));
    assert_eq!(failure.primary_labels(), Some(("handshake", "io")));
    assert_eq!(failure.cleanup_labels(), Some(("shutdown", "io")));
    assert_eq!(
        failure.admission_cleanup_labels(),
        Some(("currentness", "control"))
    );
    assert_eq!(failure.power_unregistration_error(), Some(close_code));
    assert!(!failure.power_stop_failed());
    assert!(failure.source_policy_error().is_none());
    assert!(failure.retained_configuration().is_none());
    assert_eq!(
        failure.to_string(),
        "source protected probe refused: handshake/io"
    );
    assert_eq!(
        format!("{failure:?}"),
        r#"SourceProtectedProbeFailure { stage: "handshake", cause: "io", .. }"#
    );
}

/// The facade moves the complete abnormal outcome, including its genuine history.
/// # Errors
/// Preserve actual protected Script/worker failures and missing joined facts.
/// # Panics
/// Fail if partial movement loses history or changes the actual retirement disposition.
#[test]
fn facade_actual_retirement_panic_retains_owning_protected_history() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    static DROPS: AtomicUsize = AtomicUsize::new(0);
    let config = Config::from_arguments(
        vec![
            "127.0.0.1:4460".into(),
            "Time.Example".into(),
            "--protected-datagram".into(),
        ]
        .into_iter(),
    )?;
    let lifetime = Arc::downgrade(&config.lifetime);
    let result = run_synchronously_with(config, &SLOT, None, |slot, prepared| {
        let mut hooks = Hooks::none();
        hooks.before_retire = Some(Box::new(|| {
            std::panic::resume_unwind(Box::new(Payload { drops: &DROPS }))
        }));
        slot.submit_fixture(
            prepared,
            &ADMISSION,
            component_provider,
            |config, provider, control| {
                let mut producer =
                    ScriptProducer::new(config, provider, vec![Ok(())], ExchangeCase::Healthy);
                drive_invocation_with(config, provider, control, &mut producer)
            },
            hooks,
        )
    });
    let Err(RunFailed::Abnormal(outcome)) = result else {
        return Err(Failure::Control);
    };
    assert_eq!(outcome.termination, Termination::Panicked);
    assert!(!outcome.retired);
    let completion = outcome
        .operation
        .ok_or(Failure::Control)?
        .map_err(|failure| failure.primary)?;
    let InvocationCompletion::Protected(history) = &completion else {
        return Err(Failure::Control);
    };
    assert_eq!(
        (
            history.receipt().fields().receive().bits(),
            history.receipt().fields().transmit().bits()
        ),
        (0x2233_4455_6677_8899, 0x3344_5566_7788_99aa)
    );
    assert!(completion.summary().ok_or(Failure::Control)?.transmitted);
    assert_eq!(lifetime.strong_count(), 0);
    assert_eq!(SLOT.sealed_count().map_err(|_| Failure::Control)?, 1);
    assert_eq!(DROPS.load(Ordering::Acquire), 0);
    assert_eq!(
        SLOT.observer().poll_until(Instant::now() + WAIT),
        Observation::Taken
    );
    Ok(())
}

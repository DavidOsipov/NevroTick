// SPDX-License-Identifier: GPL-2.0-only
//! Whole-call controls reuse the actual Script producer, receiving path and static slot.

use std::cell::RefCell;
use std::sync::OnceLock;
use std::sync::atomic::AtomicUsize;

/// Actual joined protected-history adaptation, with independent numerical expectations.
#[path = "numerics_tests.rs"]
mod numerics_tests;

use super::{
    Arc, AtomicBool, Config, Control, Duration, ExchangeCase, ExchangeMode, Failed, Failure,
    Instant, Ordering, ProcessAdmission, ProviderScope, Receiver, ScriptProducer, StopDisposition,
    StopHandle, SyncSender, Weak, failed, runtime, sync_channel,
};
use crate::nts_ke_experiment::job::{
    CollectError, Hooks, JoinedOutcome, Observation, PreparedInvocation, SubmitError, Termination,
    WorkerSlot,
};
use crate::nts_ke_experiment::process::ProcessClaim;
use crate::nts_ke_experiment::{
    InvocationCompletion, RunFailed, drive_invocation_with, output, run_synchronously_with,
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
fn input(mode: ExchangeMode) -> Result<Input, Failure> {
    let mut arguments = vec!["127.0.0.1:4460".into(), "Time.Example".into()];
    if mode == ExchangeMode::ProtectedDatagram {
        arguments.push("--protected-datagram".into());
    }
    let config = Config::from_arguments(arguments.into_iter())?;
    let config_lifetime = Arc::downgrade(&config.lifetime);
    let (prepared, stop) =
        PreparedInvocation::prepare(config).map_err(|failure| failure.primary)?;
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
    } = input(ExchangeMode::Preview)?;
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
    } = input(ExchangeMode::Preview)?;
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
    let second = input(ExchangeMode::Preview)?;
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
    } = input(ExchangeMode::ProtectedDatagram)?;
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
    let summary = completion.summary();
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
    let Input { prepared, stop, .. } = input(ExchangeMode::ProtectedDatagram)?;
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
    } = input(ExchangeMode::Preview)?;
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
    } = input(ExchangeMode::Preview)?;
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
    } = input(ExchangeMode::Preview)?;
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
    let Input { prepared, .. } = input(ExchangeMode::Preview)?;
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
    assert!(matches!(outcome.operation, Some(Ok(completion)) if !completion.summary().transmitted));
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
    } = input(ExchangeMode::Preview)?;
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
            matches!(outcome.operation, Some(Ok(completion)) if !completion.summary().transmitted)
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
    submissions.fetch_add(1, Ordering::AcqRel);
    slot.submit_fixture(
        prepared,
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
    let summary = completion.summary();
    assert!(summary.transmitted);
    assert_eq!((summary.cookies, summary.cookie_bytes), (1, 12));
    assert_eq!(submissions.load(Ordering::Acquire), 1);
    assert_eq!(producers.load(Ordering::Acquire), 1);
    assert_eq!(after_completion, Observation::Taken);
    assert_eq!(completed_config.strong_count(), 0);
    assert!(matches!(ADMISSION.claim(), Err(Failure::ProcessUsed)));
    Ok(())
}

/// Closed actual-owner retirement choices; neither simulates native cleanup.
#[derive(Clone, Copy)]
enum HistoryRetirement {
    Ordinary,
    Panics,
}

/// Compare the actual retained payload with its pre-record observation and literals.
/// No raw constructor, fresh timestamp or result publication is performed here.
/// # Panics
/// Fail if original bytes, captures, ordinal, time or identities change during transfer.
fn assert_retained_history(
    history: &crate::nts_ke_experiment::datagram::ProtectedHistory,
    expected: &super::ProtectedWitness,
    original_chart: &Weak<crate::RunClockContext>,
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
    for (actual, expected) in history.captures().into_iter().zip(expected.captures) {
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
    } = input(ExchangeMode::ProtectedDatagram)?;
    let original_origin = prepared.original_times().0;
    let witness = Arc::new(OnceLock::<super::ProtectedWitness>::new());
    let observed = Arc::clone(&witness);
    let chart = Arc::new(OnceLock::<Weak<crate::RunClockContext>>::new());
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
    assert_retired_resources(&config_lifetime, provider.get(), resource.get());
    let summary = completion.summary();
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

/// Valid-tag zero T2/T3 values remain inert history, alongside the nonzero neighbor.
/// # Errors
/// Preserve actual joined Script failures; no numerical/sample constructor is invoked.
/// # Panics
/// Fail if any original bits/captures/identity are lost or cleanup is falsely inferred.
#[test]
fn whole_job_protected_history_retains_nonzero_and_zero_timestamps() -> Result<(), Failure> {
    static SLOTS: [WorkerSlot; 5] = [const { WorkerSlot::new() }; 5];
    static ADMISSIONS: [ProcessAdmission; 5] = [const { ProcessAdmission::new() }; 5];
    let cases = [
        (
            ExchangeCase::Healthy,
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

/// Normally joined failure retains the original primary and both secondary facts.
/// # Errors
/// Preserve actual checked construction and genuine whole-call refusal.
/// # Panics
/// Fail if the facade replaces a cause or claims a submission/abnormal disposition.
#[test]
fn facade_actual_operation_failure_preserves_all_original_causes() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let primary = Failed {
        primary: Failure::Handshake(std::io::ErrorKind::ConnectionReset),
        cleanup: Some(Failure::Shutdown(std::io::ErrorKind::BrokenPipe)),
        admission_cleanup: Some(Failure::Control),
    };
    let config = Config::new("127.0.0.1:4460".into(), "Time.Example".into())?;
    let lifetime = Arc::downgrade(&config.lifetime);
    let result = run_synchronously_with(config, &SLOT, None, |slot, prepared| {
        slot.submit_fixture(
            prepared,
            &ADMISSION,
            component_provider,
            move |config, provider, control| {
                let mut producer = ScriptProducer::new(
                    config,
                    provider,
                    vec![Err(primary)],
                    ExchangeCase::Healthy,
                );
                drive_invocation_with(config, provider, control, &mut producer)
            },
            Hooks::none(),
        )
    });
    let Err(RunFailed::Operation(actual)) = result else {
        return Err(Failure::Control);
    };
    assert_eq!(actual, primary, "KE-HIST-06-facade-original-operation");
    assert_eq!(lifetime.strong_count(), 0);
    assert_eq!(
        SLOT.observer().poll_until(Instant::now() + WAIT),
        Observation::Taken
    );
    Ok(())
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
    assert!(completion.summary().transmitted);
    assert_eq!(lifetime.strong_count(), 0);
    assert_eq!(SLOT.sealed_count().map_err(|_| Failure::Control)?, 1);
    assert_eq!(DROPS.load(Ordering::Acquire), 0);
    assert_eq!(
        SLOT.observer().poll_until(Instant::now() + WAIT),
        Observation::Taken
    );
    Ok(())
}

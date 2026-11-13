// SPDX-License-Identifier: GPL-2.0-only
//! Conditional numerical receiving through the actual Script, worker and joined facade.
//! The fixed Era0 declaration is a model, never a healthy-time or eligibility claim.

use std::sync::mpsc::TryRecvError;
use std::time::{SystemTime, UNIX_EPOCH};

use super::{
    Arc, AtomicUsize, CollectError, Config, Duration, ExchangeCase, ExchangeMode, Failed, Failure,
    Hooks, Instant, InvocationCompletion, Observation, OnceLock, Ordering, Payload,
    ProcessAdmission, RunFailed, ScriptProducer, SubmitError, Termination, WAIT, Weak, WorkerSlot,
    component_provider, drive_invocation_with, failed, owner_gate, run_synchronously_with,
    sync_channel,
};
use crate::nts_ke_experiment::datagram::ReceivedCompletion;
use crate::nts_ke_experiment::job::PreparedInvocation;
use crate::{ExperimentalProfile, MeasurementRefusal};

/// Literal authenticated receive coordinate in the maintained healthy fixture.
const RECEIVE_BITS: u64 = 0x2233_4455_6677_8899;
/// Literal authenticated transmit coordinate in that same fixture.
const TRANSMIT_BITS: u64 = 0x3344_5566_7788_99aa;
/// Independent number of binary fractional ticks in one second.
const Q32: i128 = 4_294_967_296;
/// Independent decimal nanoseconds per second.
const NANOS: i128 = 1_000_000_000;

/// Pre-adapter observations borrow only genuine completed operation facts.
struct BeforeJoin {
    /// Original actual producer facts before numerical adaptation.
    protected: super::super::ProtectedWitness,
    /// Non-owning identity of the original invocation chart.
    chart: Weak<crate::RunClockContext>,
    /// Original checked invocation origin, observed before its operation.
    origin: Instant,
    /// Actual provider scope's independent destruction witness.
    provider: Weak<()>,
    /// Actual Script backend's independent destruction witness.
    backend: Weak<()>,
}

/// Own the actual facade result beside non-owning pre-adapter observations.
struct Observed {
    /// Genuine owning result returned only through the synchronous facade.
    completion: ReceivedCompletion,
    /// Once-written actual producer witness; no observer constructs completion.
    before: Arc<OnceLock<BeforeJoin>>,
    /// Independent Config destruction witness.
    config: Weak<()>,
}

/// Admit the unchanged closed loopback configuration through its real constructor.
/// # Errors
/// Preserve actual argument admission refusal.
fn config(mode: ExchangeMode) -> Result<Config, Failure> {
    let mut arguments = vec!["127.0.0.1:4460".into(), "Time.Example".into()];
    if mode == ExchangeMode::ProtectedDatagram {
        arguments.push("--protected-datagram".into());
    }
    Config::from_arguments(arguments.into_iter())
}

/// Declare exactly Era0 and a finite consistency allowance, without date inference.
/// # Errors
/// Preserve checked model construction refusal.
fn declaration() -> Result<ExperimentalProfile, Failure> {
    ExperimentalProfile::new(0, u64::from(u32::MAX), 30_000_000_000).map_err(|_| Failure::Control)
}

/// Submit the real whole-call owner; observers neither construct nor commit history.
/// # Errors
/// Preserve actual slot submission refusal.
/// # Panics
/// Fail if actual producer counts or write-once pre-adapter custody changes.
fn submit_case(
    slot: &'static WorkerSlot,
    prepared: PreparedInvocation,
    admission: &'static ProcessAdmission,
    case: ExchangeCase,
    before: Arc<OnceLock<BeforeJoin>>,
    hooks: Hooks,
) -> Result<(), SubmitError> {
    slot.submit_with(
        prepared,
        admission,
        component_provider,
        move |config, provider, control| {
            let origin = control.origin;
            let provider_witness = provider.retirement_witness();
            let mut producer = ScriptProducer::new(config, provider, vec![Ok(())], case);
            let completion = drive_invocation_with(config, provider, control, &mut producer);
            assert_eq!(producer.establish_calls, 1);
            if let Ok(InvocationCompletion::Protected(history)) = &completion {
                let protected = producer
                    .last_protected
                    .take()
                    .ok_or_else(|| failed(Failure::Control))?;
                let backend = producer
                    .backend_lifetime
                    .take()
                    .ok_or_else(|| failed(Failure::Control))?;
                assert!(
                    before
                        .set(BeforeJoin {
                            protected,
                            chart: Arc::downgrade(history.run_context()),
                            origin,
                            provider: provider_witness,
                            backend,
                        })
                        .is_ok()
                );
                assert_eq!((producer.exchange_calls, producer.entropy_calls), (1, 1));
            }
            completion
        },
        hooks,
    )
}

/// Run the actual facade once, retaining its original result and immutable witness.
/// # Errors
/// Preserve fixture admission, actual worker/facade failure or missing history.
/// # Panics
/// Fail if actual joined custody, permanent process admission or one-take behavior changes.
fn protected_case(
    slot: &'static WorkerSlot,
    admission: &'static ProcessAdmission,
    case: ExchangeCase,
    profile: Option<ExperimentalProfile>,
) -> Result<Observed, Failure> {
    let config = config(ExchangeMode::ProtectedDatagram)?;
    let lifetime = Arc::downgrade(&config.lifetime);
    let before = Arc::new(OnceLock::new());
    let observed = Arc::clone(&before);
    let completion = run_synchronously_with(config, slot, profile, move |slot, prepared| {
        submit_case(slot, prepared, admission, case, observed, Hooks::none())
    })
    .map_err(|_| Failure::Control)?;
    assert_eq!(
        slot.observer().poll_until(Instant::now() + WAIT),
        Observation::Taken
    );
    assert!(matches!(slot.take_result(), Err(CollectError::NotReady)));
    assert!(matches!(admission.claim(), Err(Failure::ProcessUsed)));
    Ok(Observed {
        completion,
        before,
        config: lifetime,
    })
}

/// Compare the post-join carrier against actual pre-adapter facts and fixed peer bits.
/// # Errors
/// Refuse absent protected custody or a missing actual producer witness.
/// # Panics
/// Fail if adaptation changes custody, timestamps, identities, population or CLI counts.
fn assert_original(observed: &Observed, peer_times: (u64, u64)) -> Result<(), Failure> {
    let ReceivedCompletion::Protected(carrier) = &observed.completion else {
        return Err(Failure::Control);
    };
    let before = observed.before.get().ok_or(Failure::Control)?;
    super::assert_retained_history(
        carrier.history(),
        &before.protected,
        &before.chart,
        before.origin,
        peer_times,
    );
    super::assert_retired_resources(
        &observed.config,
        Some(&before.provider),
        Some(&before.backend),
    );
    let summary = carrier.history().summary();
    assert!(summary.transmitted && summary.packet_bytes > 48);
    assert_eq!((summary.cookies, summary.cookie_bytes), (1, 12));
    Ok(())
}

/// Independently convert the supplied decimal wall reading to NTP-epoch nanoseconds.
/// # Errors
/// Refuse nonrepresentable fixture arithmetic rather than weakening the model.
/// # Panics
/// Refuse a host date outside this explicitly bounded Era0 control.
fn ntp_nanoseconds(wall: SystemTime) -> Result<i128, Failure> {
    let unix = match wall.duration_since(UNIX_EPOCH) {
        Ok(value) => i128::try_from(value.as_nanos()).map_err(|_| Failure::Control)?,
        Err(value) => -i128::try_from(value.duration().as_nanos()).map_err(|_| Failure::Control)?,
    };
    let ntp = unix + 2_208_988_800 * NANOS;
    assert!(
        (0..4_294_967_296 * NANOS).contains(&ntp),
        "fixed declared-model control requires actual local reads in Era0"
    );
    Ok(ntp)
}

/// Calculate expected exact quantities directly from literal peer bits and decimal clocks.
/// No production conversion, unfolding, exchange arithmetic or measure call is an oracle.
/// # Errors
/// Preserve missing conditional success or independent fixture arithmetic refusal.
/// # Panics
/// Fail if any coordinate, signed arithmetic, nominal ratio, window or original age differs.
fn assert_independent_math(observed: &Observed) -> Result<(), Failure> {
    let ReceivedCompletion::Protected(carrier) = &observed.completion else {
        return Err(Failure::Control);
    };
    let measurement = carrier
        .measurement()
        .as_ref()
        .map_err(|_| Failure::Control)?;
    let [send, after, read] = carrier.history().captures();
    let u1 = ntp_nanoseconds(send.utc())?;
    let ua = ntp_nanoseconds(after.utc())?;
    let u4 = ntp_nanoseconds(read.utc())?;
    let q1 = (u1 * Q32).div_euclid(NANOS);
    let qa = (ua * Q32).div_euclid(NANOS);
    let q4 = (u4 * Q32).div_euclid(NANOS);
    let a = 2_464_388_554_683_811_993_i128;
    let b = 3_694_171_492_931_115_434_i128;
    assert_eq!(
        measurement
            .coordinates()
            .map(time_domain::UtcCoordinateQ32::ticks),
        [q1, a, b, q4]
    );
    assert_eq!(
        measurement.arithmetic().offset().numerator(),
        a + b - q1 - q4
    );
    assert_eq!(
        measurement.arithmetic().round_trip().numerator(),
        q4 - q1 - (b - a)
    );
    assert!(measurement.arithmetic().round_trip().numerator() < 0);
    assert_eq!(
        measurement.nominal_phase().numerator(),
        NANOS * (a + b) - Q32 * (u1 + u4)
    );
    assert_eq!(measurement.nominal_phase().denominator(), 8_589_934_592);
    assert_eq!(
        measurement.nominal_round_trip().numerator(),
        Q32 * (u4 - u1) - NANOS * (b - a)
    );
    assert_eq!(
        measurement.nominal_round_trip().denominator(),
        4_294_967_296
    );
    let coordinates = [q1, qa, a, b, q4];
    assert_eq!(
        measurement.software_window().earliest().ticks(),
        *coordinates.iter().min().ok_or(Failure::Control)?
    );
    assert_eq!(
        measurement.software_window().latest().ticks(),
        *coordinates.iter().max().ok_or(Failure::Control)?
    );
    let expected_age = carrier
        .history()
        .receipt()
        .accepted_at()
        .checked_duration_since(read.monotonic_after())
        .ok_or(Failure::Control)?;
    assert_eq!(measurement.acquisition_age(), expected_age);
    assert_eq!(measurement.declared_limit_ns(), 30_000_000_000);
    Ok(())
}

/// Missing declaration preserves genuine joined history without numerical authority.
/// # Errors
/// Preserve actual protected acquisition and joined receiving failure.
/// # Panics
/// Fail if a missing declaration gains numerics or changes original history.

#[test]
fn facade_protected_numerics_none_retains_original_history() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let observed = protected_case(&SLOT, &ADMISSION, ExchangeCase::Healthy, None)?;
    assert_original(&observed, (RECEIVE_BITS, TRANSMIT_BITS))?;
    let ReceivedCompletion::Protected(carrier) = &observed.completion else {
        return Err(Failure::Control);
    };
    assert!(carrier.profile().is_none());
    assert!(matches!(
        carrier.measurement(),
        Err(MeasurementRefusal::Unavailable)
    ));
    Ok(())
}

/// A fixed explicit declaration permits inert signed arithmetic on actual captures.
/// # Errors
/// Preserve actual acquisition, checked declaration and independent arithmetic refusal.
/// # Panics
/// Fail if the exact declaration or independently calculated result changes.

#[test]
fn facade_protected_numerics_declared_era_zero_has_independent_math() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let profile = declaration()?;
    let observed = protected_case(&SLOT, &ADMISSION, ExchangeCase::Healthy, Some(profile))?;
    assert_original(&observed, (RECEIVE_BITS, TRANSMIT_BITS))?;
    let ReceivedCompletion::Protected(carrier) = &observed.completion else {
        return Err(Failure::Control);
    };
    assert_eq!(carrier.profile(), Some(&profile));
    assert_independent_math(&observed)
}

/// Incompatible checked dates remain a numerical refusal beside unchanged history.
/// # Errors
/// Preserve actual acquisition and checked scope construction refusal.
/// # Panics
/// Fail if incompatible dates are widened or history is discarded.

#[test]
fn facade_protected_numerics_exact_declared_dates_refuse_without_losing_history()
-> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let profile = ExperimentalProfile::new(0, 0, 30_000_000_000).map_err(|_| Failure::Control)?;
    let observed = protected_case(&SLOT, &ADMISSION, ExchangeCase::Healthy, Some(profile))?;
    assert_original(&observed, (RECEIVE_BITS, TRANSMIT_BITS))?;
    let ReceivedCompletion::Protected(carrier) = &observed.completion else {
        return Err(Failure::Control);
    };
    assert_eq!(carrier.profile(), Some(&profile));
    assert!(matches!(
        carrier.measurement(),
        Err(MeasurementRefusal::DateScope)
    ));
    Ok(())
}

/// Authentic zero peer coordinates retain exact bits and the missing-profile precedence.
/// # Errors
/// Preserve each actual protected exchange and checked declared-model construction.
/// # Panics
/// Fail if authentic zero bits or profile/protocol refusal precedence changes.

#[test]
fn facade_protected_numerics_authentic_zero_bits_keep_distinct_refusals() -> Result<(), Failure> {
    static SLOTS: [WorkerSlot; 6] = [const { WorkerSlot::new() }; 6];
    static ADMISSIONS: [ProcessAdmission; 6] = [const { ProcessAdmission::new() }; 6];
    let profile = declaration()?;
    let cases = [
        (ExchangeCase::ZeroReceive, (0, TRANSMIT_BITS)),
        (ExchangeCase::ZeroTransmit, (RECEIVE_BITS, 0)),
        (ExchangeCase::ZeroTimestamps, (0, 0)),
    ];
    for (index, (case, bits)) in cases.into_iter().enumerate() {
        for (declaration_index, declared) in [None, Some(profile)].into_iter().enumerate() {
            let at = 2 * index + declaration_index;
            let observed = protected_case(&SLOTS[at], &ADMISSIONS[at], case, declared)?;
            assert_original(&observed, bits)?;
            let ReceivedCompletion::Protected(carrier) = &observed.completion else {
                return Err(Failure::Control);
            };
            assert_eq!(carrier.profile(), declared.as_ref());
            let expected = if declared.is_some() {
                MeasurementRefusal::Protocol
            } else {
                MeasurementRefusal::Unavailable
            };
            assert!(matches!(carrier.measurement(), Err(actual) if *actual == expected));
        }
    }
    Ok(())
}

/// The actual facade cannot publish before retirement/join or refresh historical age.
/// # Errors
/// Preserve finite channel/gate cleanup and actual facade/worker failure.
/// # Panics
/// Fail if a carrier escapes before join or later time replaces original publication age.

#[test]
fn facade_protected_numerics_waits_for_join_and_keeps_original_age() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let config = config(ExchangeMode::ProtectedDatagram)?;
    let lifetime = Arc::downgrade(&config.lifetime);
    let before = Arc::new(OnceLock::new());
    let observed_before = Arc::clone(&before);
    let (retire, gate) = owner_gate();
    let mut hooks = Hooks::none();
    hooks.before_retire = Some(retire);
    let profile = declaration()?;
    let (sent, received) = sync_channel(1);
    let completion = std::thread::scope(|scope| {
        let facade = scope.spawn(move || {
            let result = run_synchronously_with(config, &SLOT, Some(profile), |slot, prepared| {
                submit_case(
                    slot,
                    prepared,
                    &ADMISSION,
                    ExchangeCase::Healthy,
                    observed_before,
                    hooks,
                )
            });
            sent.send(result).map_err(|_| Failure::Control)
        });
        gate.entered
            .recv_timeout(WAIT)
            .map_err(|_| Failure::Control)?;
        let deadline = Instant::now() + WAIT;
        while SLOT.observer().poll_until(deadline) != Observation::Joining {
            if Instant::now() >= deadline {
                return Err(Failure::Control);
            }
            std::thread::yield_now();
        }
        let unavailable = received.try_recv();
        let was_unavailable = matches!(&unavailable, Err(TryRecvError::Empty));
        let withheld = SLOT.take_result();
        let original = before.get().ok_or(Failure::Control)?;
        let read = original.protected.captures[2];
        let expected_age = original
            .protected
            .receipt
            .accepted_at()
            .checked_duration_since(read.monotonic_after())
            .ok_or(Failure::Control)?;
        std::thread::sleep(Duration::from_millis(5));
        let later_age = Instant::now()
            .checked_duration_since(read.monotonic_after())
            .ok_or(Failure::Control)?;
        gate.release()?;
        let result = match unavailable {
            Ok(result) => result,
            Err(TryRecvError::Empty) => {
                received.recv_timeout(WAIT).map_err(|_| Failure::Control)?
            }
            Err(TryRecvError::Disconnected) => return Err(Failure::Control),
        };
        facade.join().map_err(|_| Failure::Control)??;
        assert!(matches!(withheld, Err(CollectError::NotReady)));
        assert!(later_age > expected_age);
        // A faulty early facade is drained only after the real hold has been released.
        super::drain_facade_observation(&SLOT, SLOT.observer().poll_until(Instant::now() + WAIT))?;
        assert!(was_unavailable, "no carrier before actual join");
        result.map_err(|_| Failure::Control)
    })?;
    let observed = Observed {
        completion,
        before,
        config: lifetime,
    };
    assert_original(&observed, (RECEIVE_BITS, TRANSMIT_BITS))?;
    assert_independent_math(&observed)
}

/// Preview, failed operations and later retirement panic bypass protected adaptation.
/// # Errors
/// Preserve actual Script, failure, preview and abnormal joined-result observations.
/// # Panics
/// Fail if adaptation changes preview, original failure or abnormal-retirement custody.

#[test]
fn facade_protected_numerics_bypasses_preview_failure_and_retirement_panic() -> Result<(), Failure>
{
    static SLOTS: [WorkerSlot; 3] = [const { WorkerSlot::new() }; 3];
    static ADMISSIONS: [ProcessAdmission; 3] = [const { ProcessAdmission::new() }; 3];
    static DROPS: AtomicUsize = AtomicUsize::new(0);
    let profile = declaration()?;
    let unused = Arc::new(OnceLock::new());
    let preview = run_synchronously_with(
        config(ExchangeMode::Preview)?,
        &SLOTS[0],
        Some(profile),
        |slot, prepared| {
            submit_case(
                slot,
                prepared,
                &ADMISSIONS[0],
                ExchangeCase::Healthy,
                Arc::clone(&unused),
                Hooks::none(),
            )
        },
    )
    .map_err(|_| Failure::Control)?;
    assert!(matches!(preview, ReceivedCompletion::Preview(summary) if !summary.transmitted));
    assert!(unused.get().is_none());
    let primary = Failed {
        primary: Failure::Handshake(std::io::ErrorKind::ConnectionReset),
        cleanup: Some(Failure::Shutdown(std::io::ErrorKind::BrokenPipe)),
        admission_cleanup: Some(Failure::Control),
    };
    let failure = run_synchronously_with(
        config(ExchangeMode::ProtectedDatagram)?,
        &SLOTS[1],
        Some(profile),
        |slot, prepared| {
            slot.submit_with(
                prepared,
                &ADMISSIONS[1],
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
        },
    );
    assert!(matches!(failure, Err(RunFailed::Operation(actual)) if actual == primary));
    let before = Arc::new(OnceLock::new());
    let mut hooks = Hooks::none();
    hooks.before_retire = Some(Box::new(|| {
        std::panic::resume_unwind(Box::new(Payload { drops: &DROPS }))
    }));
    let abnormal = run_synchronously_with(
        config(ExchangeMode::ProtectedDatagram)?,
        &SLOTS[2],
        Some(profile),
        |slot, prepared| {
            submit_case(
                slot,
                prepared,
                &ADMISSIONS[2],
                ExchangeCase::Healthy,
                Arc::clone(&before),
                hooks,
            )
        },
    );
    let Err(RunFailed::Abnormal(outcome)) = abnormal else {
        return Err(Failure::Control);
    };
    assert_eq!(outcome.termination, Termination::Panicked);
    assert!(!outcome.retired);
    let Some(Ok(InvocationCompletion::Protected(history))) = outcome.operation else {
        return Err(Failure::Control);
    };
    assert_eq!(
        (
            history.receipt().fields().receive().bits(),
            history.receipt().fields().transmit().bits()
        ),
        (RECEIVE_BITS, TRANSMIT_BITS)
    );
    assert!(before.get().is_some());
    assert_eq!(SLOTS[2].sealed_count().map_err(|_| Failure::Control)?, 1);
    assert_eq!(DROPS.load(Ordering::Acquire), 0);
    Ok(())
}

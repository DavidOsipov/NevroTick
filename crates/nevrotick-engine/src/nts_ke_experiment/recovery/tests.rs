// SPDX-License-Identifier: GPL-2.0-only
//! Actual shared-step component controls; fixed public material is not TLS evidence.

use std::collections::VecDeque;
use std::io;
use std::net::SocketAddr;
use std::sync::{Arc, Weak};
use std::time::{Duration, Instant};

use zeroize::Zeroizing;

use super::{
    AssociationCandidate, AssociationOwner, C2S, Config, Control, DirectionalKeys, Failure,
    LedgerError, ProcessAdmission, ProviderScope, RequestInput, S2C, Script, ScriptRead,
    ScriptSend, StrictProfile, backend, entropy, exchange_with, fixtures, runtime,
};
use crate::nts_ke_experiment::association::ledger::{RequestPhase, Status};
use crate::nts_ke_experiment::datagram::ProtectedCompletion;
use crate::nts_ke_experiment::recovery::{
    Backoff, ContextStage, End, Invocation, Limits, Plan, Producer, Snapshot, Step, step_with,
};
use crate::nts_ke_experiment::{EXCHANGE_TIME, ExchangeMode, Failed, HANDSHAKE_TIME};

/// Metadata observed inside the actual producer call, never a retained Control identity.
#[derive(Clone, Copy, Debug)]
struct ControlTimes {
    origin: Instant,
    handshake: Instant,
    expiry: Instant,
}

/// One closed selected protected-operation fixture; no callback fabricates a fact.
#[derive(Clone, Copy, Debug)]
enum ExchangeCase {
    Healthy,
    ReceiveReset,
    ZeroCookies,
    LastAllowanceHealthy,
    StaleReturned,
    MixedCountRefusal,
    MalformedOnly,
    UnknownExtent,
    SendUnknown,
    EntropyFailure,
    CancelZeroCookies,
}

/// A second declared component context; these are not real TLS exporter facts.
#[derive(Clone, Copy, Debug)]
enum SuccessorCase {
    Healthy,
    OldCookie,
    MixedCookie,
    RepeatedUid,
    OldS2cOnly,
    OldPacketsThenHealthy,
    ZeroCookies,
    StaleReturned,
}

const SUCCESSOR_C2S: [u8; 32] = [
    0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f,
    0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f,
];
const SUCCESSOR_S2C: [u8; 32] = [
    0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce, 0xcf,
    0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd, 0xde, 0xdf,
];
const SUCCESSOR_UID: [u8; 32] = [
    0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f,
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x3e, 0x3f,
];
const SUCCESSOR_WIRE: u64 = 0x8899_aabb_ccdd_eeff;
const MALFORMED_RESPONSE: &[u8] = &[0];

/// Observe only secret-free state around the actual retired exchange wrapper.
#[derive(Clone, Copy)]
struct ExchangeObservation {
    before: Status,
    after: Status,
    expiry_before: Instant,
    expiry_after: Instant,
    has_receipt: bool,
}

/// Replace external KE/entropy facts only; admission and the receiving owner stay real.
struct ScriptProducer<'fixture> {
    config: &'fixture Config,
    provider: &'fixture ProviderScope<'fixture>,
    establishment: VecDeque<Result<(), Failed>>,
    exchange_case: ExchangeCase,
    establish_calls: usize,
    exchange_calls: usize,
    preview_calls: usize,
    preview_failure: Option<Failure>,
    controls: Vec<ControlTimes>,
    establish_delay: Duration,
    /// Publish genuine transaction cancellation before the selected failure returns.
    cancel_establish_failure: bool,
    draws: VecDeque<Result<Duration, Failure>>,
    draw_bounds: Vec<Duration>,
    draw_delay: Duration,
    backend_lifetime: Option<Weak<()>>,
    last_exchange: Option<ExchangeObservation>,
    /// Select declared successor material by successful context, not KE call count.
    successor: Option<SuccessorCase>,
    established_contexts: usize,
    entropy_calls: usize,
}

/// Select bounded, public packet facts for one already selected component case.
/// This helper creates no owner, retirement fact or response admission result.
fn script_reads(
    selected: ExchangeCase,
    successor: Option<SuccessorCase>,
    remote: SocketAddr,
) -> Vec<ScriptRead> {
    if let Some(case) = successor {
        return match case {
            SuccessorCase::OldS2cOnly => (0..32)
                .map(|_| ScriptRead::Packet(fixtures::SUCCESSOR_OLD_S2C, remote))
                .collect(),
            SuccessorCase::OldPacketsThenHealthy => vec![
                ScriptRead::Packet(fixtures::HEALTHY, remote),
                ScriptRead::Packet(fixtures::SUCCESSOR_OLD_S2C, remote),
                ScriptRead::Packet(fixtures::SUCCESSOR_HEALTHY, remote),
            ],
            SuccessorCase::ZeroCookies => (0..32)
                .map(|_| ScriptRead::Packet(fixtures::SUCCESSOR_ZERO_COOKIES, remote))
                .collect(),
            _ => vec![ScriptRead::Packet(fixtures::SUCCESSOR_HEALTHY, remote)],
        };
    }
    match selected {
        ExchangeCase::ReceiveReset => {
            vec![ScriptRead::Failed(io::ErrorKind::ConnectionReset)]
        }
        ExchangeCase::ZeroCookies | ExchangeCase::CancelZeroCookies => (0..32)
            .map(|_| ScriptRead::Packet(fixtures::ZERO_COOKIES, remote))
            .collect(),
        ExchangeCase::LastAllowanceHealthy => {
            let mut reads: Vec<_> = (0..31)
                .map(|_| ScriptRead::Packet(fixtures::ZERO_COOKIES, remote))
                .collect();
            reads.push(ScriptRead::Packet(fixtures::HEALTHY, remote));
            reads
        }
        ExchangeCase::MixedCountRefusal => {
            let mut reads: Vec<_> = (0..31)
                .map(|_| ScriptRead::Packet(MALFORMED_RESPONSE, remote))
                .collect();
            reads.push(ScriptRead::Packet(fixtures::NINE_COOKIES, remote));
            reads
        }
        ExchangeCase::MalformedOnly => (0..32)
            .map(|_| ScriptRead::Packet(MALFORMED_RESPONSE, remote))
            .collect(),
        ExchangeCase::UnknownExtent => vec![ScriptRead::UnknownExtent],
        ExchangeCase::Healthy
        | ExchangeCase::StaleReturned
        | ExchangeCase::SendUnknown
        | ExchangeCase::EntropyFailure => {
            vec![ScriptRead::Packet(fixtures::HEALTHY, remote)]
        }
    }
}

impl Producer for ScriptProducer<'_> {
    fn establish<'tx>(
        &mut self,
        config: &'tx Config,
        control: &'tx Control,
        provider: &'tx ProviderScope<'tx>,
    ) -> Result<AssociationOwner<'tx>, Failed> {
        assert!(std::ptr::eq(config, self.config));
        assert!(std::ptr::eq(provider, self.provider));
        self.establish_calls += 1;
        self.controls.push(ControlTimes {
            origin: control.origin,
            handshake: control.handshake_deadline,
            expiry: control.expiry,
        });
        if !self.establish_delay.is_zero() {
            std::thread::sleep(self.establish_delay);
        }
        let establishment = self
            .establishment
            .pop_front()
            .unwrap_or_else(|| Err(failed(Failure::Control)));
        if self.cancel_establish_failure && establishment.is_err() {
            assert_eq!(
                control.cancel().map_err(failed)?,
                crate::nts_ke_experiment::CancelOutcome::Cancelled
            );
        }
        establishment?;
        let is_successor = self.successor.is_some() && self.established_contexts == 1;
        assert!(
            self.established_contexts < 2,
            "fixture received a third established context"
        );
        let keys = if is_successor {
            DirectionalKeys::new(Zeroizing::new(SUCCESSOR_C2S), Zeroizing::new(SUCCESSOR_S2C))
        } else {
            DirectionalKeys::new(Zeroizing::new(C2S), Zeroizing::new(S2C))
        };
        let cookies = if is_successor {
            match self.successor {
                Some(SuccessorCase::OldCookie) => vec![Zeroizing::new(vec![0x11; 12])],
                Some(SuccessorCase::MixedCookie) => vec![
                    Zeroizing::new(vec![0x11; 12]),
                    Zeroizing::new(vec![0x22; 12]),
                ],
                _ => vec![Zeroizing::new(vec![0x22; 12])],
            }
        } else {
            vec![Zeroizing::new(vec![0x11; 12])]
        };
        let remote = SocketAddr::new(config.peer.ip(), 123);
        let candidate = AssociationCandidate::new(
            config,
            control,
            provider,
            remote,
            StrictProfile::component(),
            keys,
            cookies,
        );
        let owner =
            AssociationOwner::receive(config, control, provider, candidate).map_err(failed)?;
        self.established_contexts += 1;
        Ok(owner)
    }

    fn exchange<'tx>(
        &mut self,
        association: &mut AssociationOwner<'tx>,
        control: &'tx Control,
    ) -> Result<ProtectedCompletion<'tx>, Failure> {
        self.exchange_calls += 1;
        let selected = self.exchange_case;
        let successor = if self.established_contexts == 2 {
            self.successor
        } else {
            None
        };
        let before = association.status();
        let expiry_before = control.expiry;
        let mut lifetime = None;
        let mut entropy_calls = 0;
        let result = exchange_with(
            association,
            control,
            |output| {
                entropy_calls += 1;
                if matches!(selected, ExchangeCase::EntropyFailure) && successor.is_none() {
                    return Err(Failure::Entropy);
                }
                if let Some(case) = successor {
                    output[..8].copy_from_slice(&SUCCESSOR_WIRE.to_be_bytes());
                    output[8..40].copy_from_slice(if matches!(case, SuccessorCase::RepeatedUid) {
                        &C2S
                    } else {
                        &SUCCESSOR_UID
                    });
                    output[40..].fill(0x99);
                    Ok(())
                } else {
                    entropy(output)
                }
            },
            |remote| {
                let reads = script_reads(selected, successor, remote);
                let send = if matches!(selected, ExchangeCase::SendUnknown) && successor.is_none() {
                    ScriptSend::Unknown
                } else {
                    ScriptSend::Full
                };
                let backend = backend(remote, Script::new(send, reads));
                lifetime = Some(Arc::downgrade(&backend.identity));
                Ok(backend)
            },
        );
        self.entropy_calls += entropy_calls;
        self.backend_lifetime = lifetime;
        self.last_exchange = Some(ExchangeObservation {
            before,
            after: association.status(),
            expiry_before,
            expiry_after: control.expiry,
            has_receipt: association.last_receipt().is_some(),
        });
        if matches!(selected, ExchangeCase::CancelZeroCookies) && successor.is_none() {
            assert_eq!(
                control.cancel()?,
                crate::nts_ke_experiment::CancelOutcome::Cancelled
            );
        }
        let completion = result?;
        if matches!(selected, ExchangeCase::StaleReturned)
            || matches!(successor, Some(SuccessorCase::StaleReturned))
        {
            let ticket = association.reserve(RequestInput::Observed)?;
            association.return_unused(ticket)?;
        }
        Ok(completion)
    }

    fn preview_entropy(&mut self, output: &mut [u8; 48]) -> Result<(), Failure> {
        self.preview_calls += 1;
        if let Some(failure) = self.preview_failure {
            return Err(failure);
        }
        output[..32].copy_from_slice(&C2S);
        output[32..].fill(0x66);
        Ok(())
    }

    fn jitter(&mut self, upper: Duration) -> Result<Duration, Failure> {
        self.draw_bounds.push(upper);
        if !self.draw_delay.is_zero() {
            std::thread::sleep(self.draw_delay);
        }
        self.draws.pop_front().unwrap_or(Ok(Duration::ZERO))
    }
}

/// Explicit injected component failure, not a native transport provenance claim.
const fn failed(primary: Failure) -> Failed {
    Failed {
        primary,
        cleanup: None,
        admission_cleanup: None,
    }
}

/// Small explicit component inputs; these are not public retry defaults.
/// # Errors
/// Preserve the actual checked delay/limit constructors' refusal.
fn experiment(transactions: u8, refusals: u8, observations: u8) -> Result<Limits, Failure> {
    let backoff = Backoff::new(
        Duration::from_millis(40),
        Duration::from_millis(160),
        2,
        Duration::from_millis(20),
    )?;
    Limits::new(
        Duration::from_secs(5),
        transactions,
        refusals,
        observations,
        backoff,
    )
}

/// Keep one actual Config, permanent process claim and component provider across steps.
/// # Errors
/// Preserve checked construction and the actual shared-step exercise's failure.
/// # Panics
/// Fail if component retirement permits a second process claim.
fn with_fixture(
    mode: ExchangeMode,
    limits: Option<Limits>,
    establishment: Vec<Result<(), Failed>>,
    exchange_case: ExchangeCase,
    exercise: impl FnOnce(
        &mut Invocation<'_>,
        &ProviderScope<'_>,
        &mut ScriptProducer<'_>,
    ) -> Result<(), Failure>,
) -> Result<(), Failure> {
    let mut arguments = vec!["127.0.0.1:4460".into(), "Time.Example".into()];
    if matches!(mode, ExchangeMode::ProtectedDatagram) {
        arguments.push("--protected-datagram".into());
    }
    let config = Config::from_arguments(arguments.into_iter())?;
    let original_control = if limits.is_none() {
        Some(Control::new(Instant::now())?)
    } else {
        None
    };
    let admission = ProcessAdmission::new();
    let claim = admission.claim()?;
    {
        let provider = ProviderScope::component(&claim);
        let mut invocation = match limits {
            Some(limits) => Invocation::experimental(&config, &provider, limits)?,
            None => Invocation::one_shot(
                &config,
                &provider,
                original_control.ok_or(Failure::Control)?,
            )?,
        };
        let mut producer = ScriptProducer {
            config: &config,
            provider: &provider,
            establishment: establishment.into(),
            exchange_case,
            establish_calls: 0,
            exchange_calls: 0,
            preview_calls: 0,
            preview_failure: None,
            controls: Vec::new(),
            establish_delay: Duration::ZERO,
            cancel_establish_failure: false,
            draws: VecDeque::new(),
            draw_bounds: Vec::new(),
            draw_delay: Duration::ZERO,
            backend_lifetime: None,
            last_exchange: None,
            successor: None,
            established_contexts: 0,
            entropy_calls: 0,
        };
        exercise(&mut invocation, &provider, &mut producer)?;
    }
    assert!(matches!(admission.claim(), Err(Failure::ProcessUsed)));
    Ok(())
}

/// Park until the already-recorded monotonic boundary without polling/replanning.
/// # Errors
/// Refuse fixture runtime failure or the finite three-second watchdog.
fn park_until(boundary: Instant) -> Result<(), Failure> {
    runtime()?
        .block_on(async {
            tokio::time::timeout(
                Duration::from_secs(3),
                tokio::time::sleep_until(tokio::time::Instant::from_std(boundary)),
            )
            .await
        })
        .map_err(|_| Failure::Control)
}

/// Read only the actual shared result; unexpected terminal/error results fail the fixture.
/// # Errors
/// Preserve the original operation failure; unexpected success/closure is Control.
fn deferred(result: &Result<Step, Failed>) -> Result<Instant, Failure> {
    match result {
        Ok(Step::Deferred(boundary)) => Ok(*boundary),
        Ok(Step::Complete(_) | Step::Closed(_)) => Err(Failure::Control),
        Err(failure) => Err(failure.primary),
    }
}

/// Extract the original failure only after the actual shared step has returned.
/// # Errors
/// Refuse if a selected failed operation publishes counts or schedules another step.
fn failed_step(result: &Result<Step, Failed>) -> Result<Failed, Failure> {
    match result {
        Err(failure) => Ok(*failure),
        Ok(_) => Err(Failure::Control),
    }
}

/// Observe all three existing closed failure facts without discarding cleanup.
/// # Panics
/// Fail if primary, TLS close or admission cleanup is replaced or erased.
fn assert_failure(
    failure: &Failed,
    primary: Failure,
    cleanup: Option<Failure>,
    admission: Option<Failure>,
) {
    assert_eq!(failure.primary, primary);
    assert_eq!(failure.cleanup, cleanup);
    assert_eq!(failure.admission_cleanup, admission);
}

/// Experimental constructors enforce the hard envelope before an invocation exists.
/// # Errors
/// Preserve the admitted exact-boundary neighbors' construction failures.
/// # Panics
/// Fail if zero/over-cap ceilings or contradictory delay bounds are admitted.
#[test]
fn checked_experiment_caps_and_delay_relations_have_exact_neighbors() -> Result<(), Failure> {
    let minimum = Duration::from_nanos(1);
    let backoff = Backoff::new(minimum, EXCHANGE_TIME, 1, Duration::ZERO)?;
    let limits = Limits::new(EXCHANGE_TIME, 4, 8, 16, backoff)?;
    limits.with_context_lifetime(minimum)?;
    limits.with_context_lifetime(EXCHANGE_TIME)?;
    for lifetime in [Duration::ZERO, EXCHANGE_TIME + minimum] {
        assert!(matches!(
            limits.with_context_lifetime(lifetime),
            Err(Failure::Limits)
        ));
    }
    for (window, n, r, d) in [
        (Duration::ZERO, 4, 8, 16),
        (EXCHANGE_TIME + minimum, 4, 8, 16),
        (EXCHANGE_TIME, 0, 8, 16),
        (EXCHANGE_TIME, 5, 8, 16),
        (EXCHANGE_TIME, 4, 0, 16),
        (EXCHANGE_TIME, 4, 9, 16),
        (EXCHANGE_TIME, 4, 8, 1),
        (EXCHANGE_TIME, 4, 8, 17),
        (
            EXCHANGE_TIME
                .checked_sub(minimum)
                .ok_or(Failure::Deadline)?,
            4,
            8,
            16,
        ),
    ] {
        assert!(matches!(
            Limits::new(window, n, r, d, backoff),
            Err(Failure::Limits)
        ));
    }
    for (low, high, growth, jitter) in [
        (Duration::ZERO, minimum, 1, Duration::ZERO),
        (minimum + minimum, minimum, 1, Duration::ZERO),
        (minimum, minimum, 0, Duration::ZERO),
        (minimum, minimum, 1, minimum),
    ] {
        assert!(matches!(
            Backoff::new(low, high, growth, jitter),
            Err(Failure::Limits)
        ));
    }
    Ok(())
}

/// The pure helpers reject their own invalid domain, not only bad caller construction.
/// # Errors
/// Preserve checked policy and valid neighboring calculations.
/// # Panics
/// Fail if growth/jitter wraps, invalid prior bases pass, or equality is rejected.
#[test]
fn pure_backoff_growth_headroom_and_overflow_are_independent() -> Result<(), Failure> {
    let low = Duration::from_millis(100);
    let high = Duration::from_millis(400);
    let jitter = Duration::from_millis(30);
    let policy = Backoff::new(low, high, 2, jitter)?;
    assert_eq!(policy.base_after(None)?, low);
    assert_eq!(policy.base_after(Some(low))?, Duration::from_millis(200));
    assert_eq!(policy.base_after(Some(Duration::from_millis(200)))?, high);
    assert_eq!(policy.base_after(Some(high))?, high);
    assert_eq!(policy.draw_upper(low)?, jitter);
    assert_eq!(policy.delay(low, jitter)?, Duration::from_millis(130));
    assert_eq!(policy.draw_upper(high)?, Duration::ZERO);
    assert_eq!(policy.delay(high, Duration::ZERO)?, high);
    let tick = Duration::from_nanos(1);
    for base in [
        Duration::ZERO,
        low.checked_sub(tick).ok_or(Failure::Deadline)?,
        high + tick,
    ] {
        assert!(policy.base_after(Some(base)).is_err());
        assert!(policy.draw_upper(base).is_err());
        assert!(policy.delay(base, Duration::ZERO).is_err());
    }
    assert!(policy.delay(low, jitter + tick).is_err());
    assert!(policy.delay(high, tick).is_err());
    let huge = Duration::from_secs(u64::MAX / 2 + 1);
    let overflow = Backoff::new(huge, Duration::MAX, 2, Duration::ZERO)?;
    assert_eq!(overflow.base_after(None)?, huge);
    assert!(matches!(
        overflow.base_after(Some(huge)),
        Err(Failure::Deadline)
    ));
    // A checked maximum base plus an extra tick refuses before addition can wrap.
    let addition = Backoff::new(tick, Duration::MAX, 1, tick)?;
    assert!(addition.delay(Duration::MAX, tick).is_err());
    Ok(())
}

/// Public one-shot policy closes before retry timing or entropy, retaining run origins.
/// # Errors
/// Preserve checked fixture construction and the original currentness assertions.
/// # Panics
/// Fail if the first 20/60-second bounds renew, or one-shot requests any retry draw.
#[test]
fn one_shot_preserves_first_origins_and_stops_before_retry_draw() -> Result<(), Failure> {
    let cause = Failure::Connect(io::ErrorKind::ConnectionReset);
    with_fixture(
        ExchangeMode::Preview,
        None,
        vec![Err(failed(cause))],
        ExchangeCase::Healthy,
        |invocation, _provider, producer| {
            let original = invocation.snapshot();
            std::thread::sleep(Duration::from_millis(5));
            let failure = failed_step(&step_with(invocation, producer))?;
            assert_failure(&failure, cause, None, None);
            let observed = producer.controls.first().ok_or(Failure::Control)?;
            assert_eq!(observed.origin, original.origin);
            assert_eq!(
                observed.handshake,
                original.origin + Duration::from_secs(20)
            );
            assert_eq!(observed.expiry, original.origin + Duration::from_secs(60));
            assert_eq!(HANDSHAKE_TIME, Duration::from_secs(20));
            assert_eq!(EXCHANGE_TIME, Duration::from_secs(60));
            assert_eq!(producer.establish_calls, 1);
            assert_eq!(producer.draw_bounds, [Duration::ZERO; 0]);
            let closed = invocation.snapshot();
            assert_eq!(
                (closed.transactions, closed.refusals, closed.observations),
                (1, 0, 2)
            );
            assert_eq!(closed.end, Some(End::Failure));
            assert!(matches!(
                step_with(invocation, producer),
                Ok(Step::Closed(End::Failure))
            ));
            assert_eq!(invocation.snapshot(), closed);
            assert_eq!(producer.establish_calls, 1);
            Ok(())
        },
    )
}

/// Every admitted native kind is interpreted only at the direct establish receiver.
/// # Errors
/// Preserve explicit checked policies and actual scheduling refusal.
/// # Panics
/// Fail if a reviewed stage/kind loses its bounded reprobe or mutates the original cause.
#[test]
fn establishment_stage_and_native_kind_table_reaches_the_real_step() -> Result<(), Failure> {
    for kind in [
        io::ErrorKind::TimedOut,
        io::ErrorKind::ConnectionRefused,
        io::ErrorKind::ConnectionReset,
        io::ErrorKind::ConnectionAborted,
    ] {
        for cause in [
            Failure::Connect(kind),
            Failure::Handshake(kind),
            Failure::Request(kind),
            Failure::Response(kind),
        ] {
            with_fixture(
                ExchangeMode::Preview,
                Some(experiment(4, 8, 16)?),
                vec![Err(failed(cause))],
                ExchangeCase::Healthy,
                |invocation, _provider, producer| {
                    let boundary = deferred(&step_with(invocation, producer))?;
                    let state = invocation.snapshot();
                    assert_eq!(
                        (state.transactions, state.refusals, state.observations),
                        (1, 0, 2)
                    );
                    assert_eq!(state.base, Some(Duration::from_millis(40)));
                    assert_eq!(state.not_before, boundary);
                    assert!(boundary < state.cutoff);
                    assert_eq!(state.end, None);
                    assert_eq!(producer.draw_bounds, [Duration::from_millis(20)]);
                    assert_failure(
                        invocation.last_failure().ok_or(Failure::Control)?,
                        cause,
                        None,
                        None,
                    );
                    Ok(())
                },
            )?;
        }
    }
    Ok(())
}

/// A cancelled transaction cannot authorize KE recovery from its transient failure.
/// The uncancelled neighbor first demonstrates the same failure's healthy reprobe.
/// # Errors
/// Preserve checked fixture construction, original failure and finite parked waits.
/// # Panics
/// Fail if cancellation permits another producer, jitter, charge or renewed window.
#[test]
fn cancelled_establishment_failure_cannot_admit_recovery() -> Result<(), Failure> {
    let cause = Failure::Connect(io::ErrorKind::TimedOut);
    for cancelled in [false, true] {
        with_fixture(
            ExchangeMode::Preview,
            Some(experiment(4, 8, 16)?),
            vec![Err(failed(cause)), Ok(())],
            ExchangeCase::Healthy,
            |invocation, _provider, producer| {
                producer.cancel_establish_failure = cancelled;
                let original = invocation.snapshot();
                let result = step_with(invocation, producer);
                let first = invocation.snapshot();
                assert_eq!(
                    (first.transactions, first.refusals, first.observations),
                    (1, 0, 2)
                );
                assert_eq!(
                    (first.origin, first.cutoff),
                    (original.origin, original.cutoff)
                );
                assert_failure(
                    invocation.last_failure().ok_or(Failure::Control)?,
                    cause,
                    None,
                    None,
                );
                if cancelled {
                    assert!(matches!(&result, Err(failure) if failure.primary == cause));
                    assert_failure(&failed_step(&result)?, cause, None, None);
                    let closed = first;
                    assert_eq!(closed.end, Some(End::Failure));
                    assert_eq!(closed.base, None);
                    for _ in 0..2 {
                        assert!(matches!(
                            step_with(invocation, producer),
                            Ok(Step::Closed(End::Failure))
                        ));
                        assert_eq!(invocation.snapshot(), closed);
                    }
                    assert_eq!(
                        (
                            producer.establish_calls,
                            producer.preview_calls,
                            producer.exchange_calls
                        ),
                        (1, 0, 0)
                    );
                    assert_eq!(producer.draw_bounds, [Duration::ZERO; 0]);
                } else {
                    let boundary = deferred(&result)?;
                    assert_eq!(producer.draw_bounds, [Duration::from_millis(20)]);
                    park_until(boundary)?;
                    let Step::Complete(summary) =
                        step_with(invocation, producer).map_err(|failure| failure.primary)?
                    else {
                        return Err(Failure::Control);
                    };
                    assert!(!summary.transmitted);
                    assert_eq!((summary.cookies, summary.cookie_bytes), (1, 12));
                    let completed = invocation.snapshot();
                    assert_eq!(
                        (
                            completed.transactions,
                            completed.refusals,
                            completed.observations
                        ),
                        (2, 0, 4)
                    );
                    assert_eq!(completed.end, Some(End::Completed));
                    assert_eq!(completed.cutoff, original.cutoff);
                    assert!(matches!(
                        step_with(invocation, producer),
                        Ok(Step::Closed(End::Completed))
                    ));
                    assert_eq!(invocation.snapshot(), completed);
                    assert_eq!(
                        (
                            producer.establish_calls,
                            producer.preview_calls,
                            producer.exchange_calls
                        ),
                        (2, 1, 0)
                    );
                    assert_eq!(producer.draw_bounds, [Duration::from_millis(20)]);
                }
                Ok(())
            },
        )?;
    }
    Ok(())
}

/// Bare invalid data, local/control/UDP and shutdown causes grant no KE retry.
/// # Errors
/// Preserve checked fixture construction and actual original-failure inspection.
/// # Panics
/// Fail if a shared I/O kind bypasses the selected establishment-stage table.
#[test]
fn unmapped_or_wrong_stage_establishment_causes_are_terminal() -> Result<(), Failure> {
    for cause in [
        Failure::Shutdown(io::ErrorKind::ConnectionReset),
        Failure::Response(io::ErrorKind::InvalidData),
        Failure::Connect(io::ErrorKind::PermissionDenied),
        Failure::Allocation,
        Failure::Entropy,
        Failure::Control,
        Failure::Deadline,
        Failure::Cancelled,
        Failure::Datagram(super::DatagramError::Receive(
            io::ErrorKind::ConnectionReset,
        )),
    ] {
        with_fixture(
            ExchangeMode::Preview,
            Some(experiment(4, 8, 16)?),
            vec![Err(failed(cause))],
            ExchangeCase::Healthy,
            |invocation, _provider, producer| {
                let failure = failed_step(&step_with(invocation, producer))?;
                assert_failure(&failure, cause, None, None);
                assert_eq!(invocation.snapshot().end, Some(End::Failure));
                assert_eq!(
                    (
                        producer.establish_calls,
                        producer.exchange_calls,
                        producer.preview_calls
                    ),
                    (1, 0, 0)
                );
                assert_eq!(producer.draw_bounds, [Duration::ZERO; 0]);
                let closed = invocation.snapshot();
                assert!(matches!(
                    step_with(invocation, producer),
                    Ok(Step::Closed(End::Failure))
                ));
                assert_eq!(invocation.snapshot(), closed);
                Ok(())
            },
        )?;
    }
    Ok(())
}

/// Original establishment and independently supplied cleanup failures remain distinct.
/// No cancellation-count observation is claimed by this fixture.
/// # Errors
/// Preserve checked construction and retained last-failure access.
/// # Panics
/// Fail if cleanup is erased, replaces the primary, or admits another producer/draw.
#[test]
fn cleanup_facts_are_preserved_and_forbid_establishment_reprobe() -> Result<(), Failure> {
    let primary = Failure::Connect(io::ErrorKind::ConnectionReset);
    for (cleanup, admission) in [
        (Some(Failure::Shutdown(io::ErrorKind::BrokenPipe)), None),
        (None, Some(Failure::Entropy)),
        (
            Some(Failure::Shutdown(io::ErrorKind::BrokenPipe)),
            Some(Failure::Control),
        ),
    ] {
        let injected = Failed {
            primary,
            cleanup,
            admission_cleanup: admission,
        };
        with_fixture(
            ExchangeMode::Preview,
            Some(experiment(4, 8, 16)?),
            vec![Err(injected)],
            ExchangeCase::Healthy,
            |invocation, _provider, producer| {
                let actual = failed_step(&step_with(invocation, producer))?;
                assert_failure(&actual, primary, cleanup, admission);
                assert_failure(
                    invocation.last_failure().ok_or(Failure::Control)?,
                    primary,
                    cleanup,
                    admission,
                );
                assert_eq!(invocation.snapshot().end, Some(End::Failure));
                assert_eq!(
                    (
                        invocation.snapshot().transactions,
                        invocation.snapshot().observations
                    ),
                    (1, 2)
                );
                assert_eq!(producer.draw_bounds, [Duration::ZERO; 0]);
                assert!(matches!(
                    step_with(invocation, producer),
                    Ok(Step::Closed(End::Failure))
                ));
                assert_eq!(producer.establish_calls, 1);
                Ok(())
            },
        )?;
    }
    Ok(())
}

/// Inert future plans and tightened constraints cannot authorize a current producer call.
/// # Errors
/// Preserve checked fixture/time construction and actual scheduling outcome.
/// # Panics
/// Fail if pure planning charges work, shorter constraints weaken waits, or equality refuses.
#[test]
fn pure_plan_and_independent_earliest_boundaries_never_mint_admission() -> Result<(), Failure> {
    with_fixture(
        ExchangeMode::Preview,
        Some(experiment(4, 8, 16)?),
        vec![Ok(())],
        ExchangeCase::Healthy,
        |invocation, _provider, producer| {
            let initial = invocation.snapshot();
            // The original lower boundary is inclusive; this remains an inert plan.
            assert_eq!(invocation.plan_at(initial.origin), Plan::Due);
            assert_eq!(invocation.snapshot(), initial);
            let source = initial.origin + Duration::from_secs(1);
            let global = initial.origin + Duration::from_secs(2);
            invocation.constrain_source_until(source);
            invocation.constrain_global_until(global);
            invocation.constrain_source_until(initial.origin);
            invocation.constrain_global_until(initial.origin);
            let before = invocation.snapshot();
            assert_eq!(
                (before.source_not_before, before.global_not_before),
                (source, global)
            );
            assert_eq!(
                invocation.plan_at(
                    global
                        .checked_sub(Duration::from_nanos(1))
                        .ok_or(Failure::Deadline)?
                ),
                Plan::Deferred(global)
            );
            assert_eq!(invocation.plan_at(global), Plan::Due);
            assert_eq!(invocation.plan_at(before.cutoff), Plan::Closed(End::Window));
            assert_eq!(
                invocation.plan_at(
                    before
                        .origin
                        .checked_sub(Duration::from_nanos(1))
                        .ok_or(Failure::Deadline)?
                ),
                Plan::Closed(End::Window)
            );
            assert_eq!(invocation.snapshot(), before);
            assert!(Instant::now() < global);
            assert_eq!(deferred(&step_with(invocation, producer))?, global);
            let denied = invocation.snapshot();
            assert_eq!(
                (denied.transactions, denied.refusals, denied.observations),
                (0, 1, 1)
            );
            assert_eq!(
                (
                    producer.establish_calls,
                    producer.exchange_calls,
                    producer.preview_calls
                ),
                (0, 0, 0)
            );
            assert_eq!(producer.draw_bounds, [Duration::ZERO; 0]);
            Ok(())
        },
    )
}

/// The eighth no-producer refusal closes in its existing observation, without redraw.
/// # Errors
/// Preserve checked fixture and actual early-refusal access.
/// # Panics
/// Fail if repeated early requests exceed R8/D16, enter KE or reopen terminal state.
#[test]
fn eight_refusals_close_without_extra_observation_or_producer_work() -> Result<(), Failure> {
    with_fixture(
        ExchangeMode::Preview,
        Some(experiment(4, 8, 16)?),
        vec![Ok(())],
        ExchangeCase::Healthy,
        |invocation, _provider, producer| {
            let boundary = invocation.snapshot().origin + Duration::from_secs(2);
            invocation.constrain_source_until(boundary);
            for count in 1..=8 {
                assert!(Instant::now() < boundary);
                let result = step_with(invocation, producer);
                if count == 8 {
                    assert!(matches!(result, Ok(Step::Closed(End::Refusals))));
                } else {
                    assert_eq!(deferred(&result)?, boundary);
                }
                let state = invocation.snapshot();
                assert_eq!(
                    (state.transactions, state.refusals, state.observations),
                    (0, count, count)
                );
            }
            let closed = invocation.snapshot();
            assert_eq!(closed.end, Some(End::Refusals));
            for _ in 0..3 {
                assert!(matches!(
                    step_with(invocation, producer),
                    Ok(Step::Closed(End::Refusals))
                ));
                assert_eq!(invocation.snapshot(), closed);
            }
            assert_eq!(producer.establish_calls, 0);
            assert_eq!(producer.draw_bounds, [Duration::ZERO; 0]);
            Ok(())
        },
    )
}

/// Four charged establishment calls share one window and never obtain a fifth entry.
/// # Errors
/// Preserve checked fixture, actual deferred results and bounded parking failures.
/// # Panics
/// Fail if attempts/observations renew, growth restarts, or a retry extends original cutoff.
#[test]
fn four_submissions_preserve_growth_cutoff_and_irreversible_charges() -> Result<(), Failure> {
    let cause = Failure::Connect(io::ErrorKind::ConnectionRefused);
    let failures = (0..4).map(|_| Err(failed(cause))).collect();
    with_fixture(
        ExchangeMode::Preview,
        Some(experiment(4, 8, 16)?),
        failures,
        ExchangeCase::Healthy,
        |invocation, _provider, producer| {
            let original = invocation.snapshot();
            for (index, base) in [40_u64, 80, 160].into_iter().enumerate() {
                let boundary = deferred(&step_with(invocation, producer))?;
                let state = invocation.snapshot();
                assert_eq!(state.base, Some(Duration::from_millis(base)));
                assert_eq!(usize::from(state.transactions), index + 1);
                assert_eq!(usize::from(state.observations), 2 * (index + 1));
                assert_eq!(state.cutoff, original.cutoff);
                park_until(boundary)?;
            }
            let failure = failed_step(&step_with(invocation, producer))?;
            assert_failure(&failure, cause, None, None);
            let terminal = invocation.snapshot();
            assert_eq!(
                (
                    terminal.transactions,
                    terminal.refusals,
                    terminal.observations
                ),
                (4, 0, 8)
            );
            assert_eq!(terminal.end, Some(End::Transactions));
            assert_eq!(producer.establish_calls, 4);
            // The third base equals the maximum, so no draw is requested there;
            // the final failed submission cannot draw for an impossible fifth.
            assert_eq!(producer.draw_bounds, [Duration::from_millis(20); 2]);
            for observed in &producer.controls {
                assert_eq!(observed.expiry, original.cutoff);
                assert_eq!(observed.handshake, original.cutoff);
            }
            assert!(producer.controls[1].origin > producer.controls[0].origin);
            assert!(matches!(
                step_with(invocation, producer),
                Ok(Step::Closed(End::Transactions))
            ));
            assert_eq!(invocation.snapshot(), terminal);
            assert_eq!(producer.establish_calls, 4);
            Ok(())
        },
    )
}

/// A complete outcome reservation must fit; exact capacity admits its healthy neighbor.
/// # Errors
/// Preserve explicit checked fixture construction and original-failure inspection.
/// # Panics
/// Fail if observation exhaustion charges another transaction or redraws its failure.
#[test]
fn insufficient_observation_pair_closes_without_partial_new_charges() -> Result<(), Failure> {
    let cause = Failure::Handshake(io::ErrorKind::TimedOut);
    with_fixture(
        ExchangeMode::Preview,
        Some(experiment(4, 8, 3)?),
        vec![Err(failed(cause)), Ok(())],
        ExchangeCase::Healthy,
        |invocation, _provider, producer| {
            let failure = failed_step(&step_with(invocation, producer))?;
            assert_failure(&failure, cause, None, None);
            let closed = invocation.snapshot();
            assert_eq!(
                (closed.transactions, closed.refusals, closed.observations),
                (1, 0, 2)
            );
            assert_eq!(closed.end, Some(End::Observations));
            assert_eq!(closed.base, None);
            assert_eq!(producer.draw_bounds, [Duration::ZERO; 0]);
            assert!(matches!(
                step_with(invocation, producer),
                Ok(Step::Closed(End::Observations))
            ));
            assert_eq!(invocation.snapshot(), closed);
            assert_eq!(producer.establish_calls, 1);
            Ok(())
        },
    )?;
    with_fixture(
        ExchangeMode::Preview,
        Some(experiment(4, 8, 4)?),
        vec![Err(failed(cause)), Ok(())],
        ExchangeCase::Healthy,
        |invocation, _provider, producer| {
            // Two charged observations leave exactly the next complete pair.
            let result = step_with(invocation, producer);
            assert!(matches!(&result, Ok(Step::Deferred(_))));
            let boundary = deferred(&result)?;
            let first = invocation.snapshot();
            assert_eq!(
                (first.transactions, first.refusals, first.observations),
                (1, 0, 2)
            );
            assert_eq!(first.base, Some(Duration::from_millis(40)));
            assert_eq!(first.end, None);
            assert_failure(
                invocation.last_failure().ok_or(Failure::Control)?,
                cause,
                None,
                None,
            );
            park_until(boundary)?;
            let Step::Complete(summary) =
                step_with(invocation, producer).map_err(|failure| failure.primary)?
            else {
                return Err(Failure::Control);
            };
            assert!(!summary.transmitted);
            assert_eq!((summary.cookies, summary.cookie_bytes), (1, 12));
            let completed = invocation.snapshot();
            assert_eq!(
                (
                    completed.transactions,
                    completed.refusals,
                    completed.observations
                ),
                (2, 0, 4)
            );
            assert_eq!(completed.end, Some(End::Completed));
            assert_eq!(
                (completed.cutoff, completed.base),
                (first.cutoff, first.base)
            );
            assert!(matches!(
                step_with(invocation, producer),
                Ok(Step::Closed(End::Completed))
            ));
            assert_eq!(invocation.snapshot(), completed);
            assert_eq!(
                (
                    producer.establish_calls,
                    producer.preview_calls,
                    producer.exchange_calls
                ),
                (2, 1, 0)
            );
            assert_eq!(producer.draw_bounds, [Duration::from_millis(20)]);
            Ok(())
        },
    )
}

/// One healthy same-invocation reprobe consumes a real retired fact then closes permanently.
/// # Errors
/// Preserve actual checked setup, finite parking and genuine protected completion.
/// # Panics
/// Fail if success loses initial counts, resets external guards or admits another context.
#[test]
fn current_protected_success_resets_only_terminal_route_backoff() -> Result<(), Failure> {
    let cause = Failure::Request(io::ErrorKind::ConnectionAborted);
    with_fixture(
        ExchangeMode::ProtectedDatagram,
        Some(experiment(4, 8, 16)?),
        vec![Err(failed(cause)), Ok(()), Ok(())],
        ExchangeCase::Healthy,
        |invocation, _provider, producer| {
            let boundary = deferred(&step_with(invocation, producer))?;
            let failed_state = invocation.snapshot();
            assert_eq!(failed_state.base, Some(Duration::from_millis(40)));
            park_until(boundary)?;
            let result = step_with(invocation, producer).map_err(|failure| failure.primary)?;
            let Step::Complete(summary) = result else {
                return Err(Failure::Control);
            };
            assert!(summary.transmitted);
            assert_eq!((summary.cookies, summary.cookie_bytes), (1, 12));
            let completed = invocation.snapshot();
            assert_eq!(
                (
                    completed.transactions,
                    completed.refusals,
                    completed.observations
                ),
                (2, 0, 4)
            );
            assert_eq!(completed.base, None);
            assert_eq!(completed.end, Some(End::Completed));
            assert_eq!(completed.cutoff, failed_state.cutoff);
            assert_eq!(completed.source_not_before, failed_state.source_not_before);
            assert_eq!(completed.global_not_before, failed_state.global_not_before);
            assert_eq!(
                (
                    producer.establish_calls,
                    producer.exchange_calls,
                    producer.preview_calls
                ),
                (2, 1, 0)
            );
            assert_eq!(
                producer
                    .backend_lifetime
                    .as_ref()
                    .ok_or(Failure::Control)?
                    .strong_count(),
                0
            );
            assert!(matches!(
                step_with(invocation, producer),
                Ok(Step::Closed(End::Completed))
            ));
            assert_eq!(invocation.snapshot(), completed);
            assert_eq!(producer.establish_calls, 2);
            assert_eq!(producer.draw_bounds.len(), 1);
            Ok(())
        },
    )
}

/// Preview success and entropy refusal each terminate without a protected reset fact.
/// # Errors
/// Preserve checked fixture, bounded parking and expected original failure access.
/// # Panics
/// Fail if a preview sends, clears retry history, or enables another successful context.
#[test]
fn preview_result_is_terminal_without_protected_backoff_reset() -> Result<(), Failure> {
    let cause = Failure::Connect(io::ErrorKind::ConnectionReset);
    for entropy_fails in [false, true] {
        with_fixture(
            ExchangeMode::Preview,
            Some(experiment(4, 8, 16)?),
            vec![Err(failed(cause)), Ok(()), Ok(())],
            ExchangeCase::Healthy,
            |invocation, _provider, producer| {
                let boundary = deferred(&step_with(invocation, producer))?;
                let prior = invocation.snapshot();
                park_until(boundary)?;
                producer.preview_failure = entropy_fails.then_some(Failure::Entropy);
                let result = step_with(invocation, producer);
                let end = if entropy_fails {
                    assert_failure(&failed_step(&result)?, Failure::Entropy, None, None);
                    End::Failure
                } else {
                    let Step::Complete(summary) = result.map_err(|failure| failure.primary)? else {
                        return Err(Failure::Control);
                    };
                    assert!(!summary.transmitted);
                    assert_eq!((summary.cookies, summary.cookie_bytes), (1, 12));
                    End::Completed
                };
                let terminal = invocation.snapshot();
                assert_eq!(terminal.base, prior.base);
                assert_eq!(terminal.end, Some(end));
                assert_eq!((terminal.transactions, terminal.observations), (2, 4));
                assert_eq!((producer.exchange_calls, producer.preview_calls), (0, 1));
                assert!(
                    matches!(step_with(invocation, producer), Ok(Step::Closed(actual)) if actual == end)
                );
                assert_eq!(invocation.snapshot(), terminal);
                assert_eq!(producer.establish_calls, 2);
                assert_eq!(producer.draw_bounds.len(), 1);
                Ok(())
            },
        )?;
    }
    Ok(())
}

/// Real UDP refusal, exhausted zero-cookie work and stale history never open a KE route.
/// # Errors
/// Preserve checked fixture/parking and actual returned operation-failure inspection.
/// # Panics
/// Fail if protected failure resets backoff, drops original intent or retries across contexts.
#[test]
fn protected_failure_and_stale_history_preserve_route_and_original_intent() -> Result<(), Failure> {
    for (selected, expected) in [
        (
            ExchangeCase::ReceiveReset,
            Failure::Datagram(super::DatagramError::Receive(
                io::ErrorKind::ConnectionReset,
            )),
        ),
        (
            ExchangeCase::ZeroCookies,
            Failure::Ledger(LedgerError::Reply(super::ReplyError::Cookies)),
        ),
        (
            ExchangeCase::StaleReturned,
            Failure::Ledger(LedgerError::Ticket),
        ),
    ] {
        with_fixture(
            ExchangeMode::ProtectedDatagram,
            Some(experiment(4, 8, 16)?),
            vec![
                Err(failed(Failure::Connect(io::ErrorKind::ConnectionReset))),
                Ok(()),
                Ok(()),
            ],
            selected,
            |invocation, _provider, producer| {
                let boundary = deferred(&step_with(invocation, producer))?;
                let prior = invocation.snapshot();
                park_until(boundary)?;
                let failure = failed_step(&step_with(invocation, producer))?;
                assert_failure(&failure, expected, None, None);
                let terminal = invocation.snapshot();
                assert_eq!(terminal.base, prior.base);
                assert_eq!((terminal.transactions, terminal.observations), (2, 4));
                assert_eq!(terminal.end, Some(End::Failure));
                if matches!(selected, ExchangeCase::ZeroCookies) {
                    assert_eq!(terminal.intent_not_before, Some(prior.cutoff));
                } else {
                    assert_eq!(terminal.intent_not_before, None);
                }
                assert_eq!(
                    producer
                        .backend_lifetime
                        .as_ref()
                        .ok_or(Failure::Control)?
                        .strong_count(),
                    0
                );
                assert!(matches!(
                    step_with(invocation, producer),
                    Ok(Step::Closed(End::Failure))
                ));
                assert_eq!(invocation.snapshot(), terminal);
                assert_eq!((producer.establish_calls, producer.exchange_calls), (2, 1));
                assert_eq!(producer.draw_bounds.len(), 1);
                Ok(())
            },
        )?;
    }
    Ok(())
}

/// Notifications and pure plan probes retain one committed draw and independent minima.
/// # Errors
/// Preserve checked construction and the actual first/early deferred results.
/// # Panics
/// Fail if an early probe redraws, grows retry base, resets counters or weakens a boundary.
#[test]
fn early_probes_do_not_redraw_or_replenish_route_state() -> Result<(), Failure> {
    let cause = Failure::Response(io::ErrorKind::ConnectionReset);
    with_fixture(
        ExchangeMode::Preview,
        Some(experiment(4, 8, 16)?),
        vec![Err(failed(cause)), Ok(())],
        ExchangeCase::Healthy,
        |invocation, _provider, producer| {
            producer.draws.push_back(Ok(Duration::from_millis(10)));
            let route = deferred(&step_with(invocation, producer))?;
            let prior = invocation.snapshot();
            let independent = prior.origin + Duration::from_secs(2);
            invocation.constrain_global_until(independent);
            producer.draws.push_back(Err(Failure::Entropy));
            assert_eq!(invocation.plan_at(independent), Plan::Due);
            assert!(Instant::now() < independent);
            assert_eq!(deferred(&step_with(invocation, producer))?, independent);
            let early = invocation.snapshot();
            assert!(route < independent);
            assert_eq!(early.base, prior.base);
            assert_eq!(early.cutoff, prior.cutoff);
            assert_eq!(
                (early.transactions, early.refusals, early.observations),
                (1, 1, 3)
            );
            assert_eq!(producer.draw_bounds, [Duration::from_millis(20)]);
            assert_eq!(producer.draws.len(), 1);
            assert_eq!(producer.establish_calls, 1);
            assert_failure(
                invocation.last_failure().ok_or(Failure::Control)?,
                cause,
                None,
                None,
            );
            Ok(())
        },
    )
}

/// A real elapsed invocation cannot consume an inert future plan as permission.
/// # Errors
/// Preserve checked explicit limits and the bounded actual wait.
/// # Panics
/// Fail if original cutoff renews, charges work or invokes a producer/draw.
#[test]
fn real_cutoff_refuses_without_producer_or_partial_accounting() -> Result<(), Failure> {
    let backoff = Backoff::new(
        Duration::from_millis(20),
        Duration::from_millis(40),
        1,
        Duration::from_millis(10),
    )?;
    let limits = Limits::new(Duration::from_millis(200), 4, 8, 16, backoff)?;
    with_fixture(
        ExchangeMode::Preview,
        Some(limits),
        vec![Ok(())],
        ExchangeCase::Healthy,
        |invocation, _provider, producer| {
            let original = invocation.snapshot();
            assert_eq!(
                invocation.plan_at(
                    original
                        .cutoff
                        .checked_sub(Duration::from_nanos(1))
                        .ok_or(Failure::Deadline)?
                ),
                Plan::Due
            );
            assert_eq!(
                invocation.plan_at(original.cutoff),
                Plan::Closed(End::Window)
            );
            assert_eq!(invocation.snapshot(), original);
            park_until(original.cutoff)?;
            assert!(Instant::now() >= original.cutoff);
            assert!(matches!(
                step_with(invocation, producer),
                Ok(Step::Closed(End::Window))
            ));
            let closed = invocation.snapshot();
            assert_eq!(
                (closed.transactions, closed.refusals, closed.observations),
                (0, 0, 0)
            );
            assert_eq!(closed.cutoff, original.cutoff);
            assert_eq!(producer.establish_calls, 0);
            assert_eq!(producer.draw_bounds, [Duration::ZERO; 0]);
            assert!(matches!(
                step_with(invocation, producer),
                Ok(Step::Closed(End::Window))
            ));
            assert_eq!(invocation.snapshot(), closed);
            Ok(())
        },
    )
}

/// Nominal and single-draw cutoff failures terminate without shortening or redrawing.
/// # Errors
/// Preserve explicit checked construction and original failure inspection.
/// # Panics
/// Fail if a no-headroom retry draws, a drawn deadline wraps, or either path retries.
#[test]
fn retry_cutoff_checks_cover_before_draw_and_after_draw() -> Result<(), Failure> {
    let cause = Failure::Handshake(io::ErrorKind::ConnectionAborted);
    for delayed_establish in [true, false] {
        let policy = Backoff::new(
            Duration::from_millis(100),
            Duration::from_millis(200),
            1,
            Duration::from_millis(100),
        )?;
        let limits = Limits::new(Duration::from_millis(200), 4, 8, 16, policy)?;
        with_fixture(
            ExchangeMode::Preview,
            Some(limits),
            vec![Err(failed(cause)), Ok(())],
            ExchangeCase::Healthy,
            |invocation, _provider, producer| {
                assert!(
                    invocation
                        .snapshot()
                        .cutoff
                        .saturating_duration_since(Instant::now())
                        >= Duration::from_millis(150),
                    "cutoff fixture lost setup headroom before its first step"
                );
                if delayed_establish {
                    producer.establish_delay = Duration::from_millis(120);
                } else {
                    producer.draws.push_back(Ok(Duration::from_millis(100)));
                }
                let failure = failed_step(&step_with(invocation, producer))?;
                assert_failure(&failure, cause, None, None);
                let closed = invocation.snapshot();
                assert_eq!(closed.end, Some(End::Window));
                assert_eq!((closed.transactions, closed.observations), (1, 2));
                assert_eq!(closed.base, None);
                assert_eq!(producer.draw_bounds.len(), usize::from(!delayed_establish));
                assert!(matches!(
                    step_with(invocation, producer),
                    Ok(Step::Closed(End::Window))
                ));
                assert_eq!(invocation.snapshot(), closed);
                assert_eq!(producer.establish_calls, 1);
                Ok(())
            },
        )?;
    }
    Ok(())
}

/// A bounded draw source that returns after cutoff cannot publish its earlier plan.
/// # Errors
/// Preserve checked fixture and original failure inspection.
/// # Panics
/// Fail if a slow draw renews cutoff, publishes a retry or overwrites its KE cause.
#[test]
fn draw_return_rechecks_actual_time_and_preserves_original_failure() -> Result<(), Failure> {
    let cause = Failure::Connect(io::ErrorKind::ConnectionRefused);
    let policy = Backoff::new(
        Duration::from_millis(40),
        Duration::from_millis(80),
        1,
        Duration::from_millis(20),
    )?;
    let limits = Limits::new(Duration::from_millis(200), 4, 8, 16, policy)?;
    with_fixture(
        ExchangeMode::Preview,
        Some(limits),
        vec![Err(failed(cause))],
        ExchangeCase::Healthy,
        |invocation, _provider, producer| {
            assert!(
                invocation
                    .snapshot()
                    .cutoff
                    .saturating_duration_since(Instant::now())
                    >= Duration::from_millis(150),
                "cutoff fixture lost setup headroom before its first step"
            );
            producer.draw_delay = Duration::from_millis(250);
            let cutoff = invocation.snapshot().cutoff;
            let failure = failed_step(&step_with(invocation, producer))?;
            assert_failure(&failure, cause, None, None);
            assert!(Instant::now() >= cutoff);
            let closed = invocation.snapshot();
            assert_eq!(closed.end, Some(End::Window));
            assert_eq!(closed.base, None);
            assert_eq!(producer.draw_bounds, [Duration::from_millis(20)]);
            assert!(matches!(
                step_with(invocation, producer),
                Ok(Step::Closed(End::Window))
            ));
            assert_eq!(invocation.snapshot(), closed);
            Ok(())
        },
    )
}

/// Failed or out-of-range draws cannot erase the separate actual KE failure.
/// # Errors
/// Preserve checked fixture construction and last-failure access.
/// # Panics
/// Fail if draw failure/rejection schedules another attempt or replaces primary facts.
#[test]
fn jitter_failure_and_excess_draw_keep_original_ke_cause() -> Result<(), Failure> {
    let cause = Failure::Connect(io::ErrorKind::ConnectionReset);
    for draw_fails in [true, false] {
        with_fixture(
            ExchangeMode::Preview,
            Some(experiment(4, 8, 16)?),
            vec![Err(failed(cause)), Ok(())],
            ExchangeCase::Healthy,
            |invocation, _provider, producer| {
                producer.draws.push_back(if draw_fails {
                    Err(Failure::Entropy)
                } else {
                    Ok(Duration::from_millis(20) + Duration::from_nanos(1))
                });
                let failure = failed_step(&step_with(invocation, producer))?;
                assert_failure(&failure, cause, None, None);
                assert_failure(
                    invocation.last_failure().ok_or(Failure::Control)?,
                    cause,
                    None,
                    None,
                );
                let expected = if draw_fails {
                    End::Jitter(Failure::Entropy)
                } else {
                    End::Arithmetic
                };
                let closed = invocation.snapshot();
                assert_eq!(closed.end, Some(expected));
                assert_eq!(
                    (closed.transactions, closed.refusals, closed.observations),
                    (1, 0, 2)
                );
                assert_eq!(closed.base, None);
                assert_eq!(producer.draw_bounds, [Duration::from_millis(20)]);
                assert!(
                    matches!(step_with(invocation, producer), Ok(Step::Closed(actual)) if actual == expected)
                );
                assert_eq!(invocation.snapshot(), closed);
                assert_eq!(producer.establish_calls, 1);
                Ok(())
            },
        )?;
    }
    Ok(())
}

/// Newly GNU-visible native-stage labels retain the existing fixed CLI contract.
/// # Panics
/// Fail if a typed cause exposes payloads or changes its established stage/kind label.
#[test]
fn native_stage_failure_labels_remain_fixed_without_native_execution() {
    use crate::nts_ke_experiment::failure_labels;

    for (failure, stage) in [
        (
            Failure::Connect(io::ErrorKind::ConnectionRefused),
            "connect",
        ),
        (
            Failure::Handshake(io::ErrorKind::ConnectionRefused),
            "handshake",
        ),
        (
            Failure::Request(io::ErrorKind::ConnectionRefused),
            "request",
        ),
        (
            Failure::Shutdown(io::ErrorKind::ConnectionRefused),
            "shutdown",
        ),
    ] {
        assert_eq!(failure_labels(failure), (stage, "refused"));
    }
}

/// Cutoff consistency is enforced at its own checked constructor boundary.
/// # Errors
/// Preserve checked owners, limits, control and healthy-neighbor construction.
/// # Panics
/// Fail if either isolated cutoff violation is admitted or an inclusive endpoint moves.
fn assert_original_control_cutoff_bounds<'scope>(
    config: &'scope Config,
    provider: &'scope ProviderScope<'scope>,
) -> Result<(), Failure> {
    let window = Duration::from_secs(5);
    let policy = Backoff::new(
        Duration::from_millis(40),
        Duration::from_millis(160),
        2,
        Duration::from_millis(20),
    )?;
    let limits = Limits::new(window, 4, 8, 16, policy)?;
    let origin = Instant::now();
    let expiry = origin
        .checked_add(Duration::from_secs(1))
        .ok_or(Failure::Deadline)?;
    let maximum = origin.checked_add(window).ok_or(Failure::Deadline)?;
    let tick = Duration::from_nanos(1);
    for cutoff in [
        expiry.checked_sub(tick).ok_or(Failure::Deadline)?,
        maximum.checked_add(tick).ok_or(Failure::Deadline)?,
    ] {
        let control = Control::clamped(origin, expiry)?;
        assert_eq!((control.origin, control.expiry), (origin, expiry));
        assert!(matches!(
            Invocation::checked_control_fixture(config, provider, limits, control, cutoff),
            Err(Failure::Deadline)
        ));
    }
    for cutoff in [expiry, maximum] {
        let control = Control::clamped(origin, expiry)?;
        assert_eq!((control.origin, control.expiry), (origin, expiry));
        let invocation =
            Invocation::checked_control_fixture(config, provider, limits, control, cutoff)?;
        let original = invocation.snapshot();
        assert_eq!((original.origin, original.cutoff), (origin, cutoff));
        assert_eq!(
            (
                original.transactions,
                original.refusals,
                original.observations
            ),
            (0, 0, 0)
        );
        assert_eq!(invocation.plan_at(origin), Plan::Due);
        assert_eq!(invocation.snapshot(), original);
    }
    Ok(())
}

/// Checked timing construction does not turn future, empty or stale inputs into work.
/// # Errors
/// Preserve legitimate checked controls, Config and component provider construction.
/// # Panics
/// Fail if clamping renews authority or one-shot rebases its original control.
#[test]
fn timing_constructors_preserve_future_guards_and_original_stale_refusal() -> Result<(), Failure> {
    use crate::nts_ke_experiment::IoPhase;

    let now = Instant::now();
    let future = now + Duration::from_secs(2);
    let cutoff = future + Duration::from_secs(1);
    let control = Control::clamped(future, cutoff)?;
    assert_eq!(control.origin, future);
    assert_eq!(
        (control.handshake_deadline, control.expiry),
        (cutoff, cutoff)
    );
    assert!(matches!(
        control.remaining(IoPhase::Handshake),
        Err(Failure::Deadline)
    ));
    assert!(matches!(
        control.remaining(IoPhase::Exchange),
        Err(Failure::Deadline)
    ));
    assert!(matches!(Control::clamped(now, now), Err(Failure::Deadline)));
    assert!(matches!(
        Control::clamped(future, now),
        Err(Failure::Deadline)
    ));
    let config = Config::new("127.0.0.1:4460".into(), "Time.Example".into())?;
    let admission = ProcessAdmission::new();
    let claim = admission.claim()?;
    let provider = ProviderScope::component(&claim);
    assert_original_control_cutoff_bounds(&config, &provider)?;
    // A legitimately shorter original Control remains accepted without renewal.
    let short_origin = Instant::now();
    let short_expiry = short_origin
        .checked_add(Duration::from_secs(5))
        .ok_or(Failure::Deadline)?;
    let shorter = Control::clamped(short_origin, short_expiry)?;
    let invocation = Invocation::one_shot(&config, &provider, shorter)?;
    let bounded = invocation.snapshot();
    assert_eq!(
        (bounded.origin, bounded.cutoff),
        (short_origin, short_expiry)
    );
    assert_eq!(
        (bounded.transactions, bounded.refusals, bounded.observations),
        (0, 0, 0)
    );
    assert_eq!(bounded.base, None);
    assert_eq!(invocation.plan_at(short_origin), Plan::Due);
    assert_eq!(invocation.snapshot(), bounded);
    let stale = Control::new(
        Instant::now()
            .checked_sub(Duration::from_secs(21))
            .ok_or(Failure::Deadline)?,
    )?;
    assert!(matches!(
        Invocation::one_shot(&config, &provider, stale),
        Err(Failure::Deadline)
    ));
    let future_control = Control::new(future)?;
    assert!(matches!(
        Invocation::one_shot(&config, &provider, future_control),
        Err(Failure::Deadline)
    ));
    let cancelled = Control::new(Instant::now())?;
    cancelled.cancel()?;
    assert!(matches!(
        Invocation::one_shot(&config, &provider, cancelled),
        Err(Failure::Cancelled)
    ));
    Ok(())
}

/// The receiving fact's direct guard accepts its committed Control and refuses another.
/// This does not fabricate an `ActiveReceiver` or claim an impossible cross-step borrow.
/// # Errors
/// Preserve genuine checked association, packet completion and guard failures.
/// # Panics
/// Fail if wrong-context checking mutates history or makes committed history unpublished.
#[test]
fn genuine_committed_fact_control_guard_refuses_foreign_checked_owner() -> Result<(), Failure> {
    super::with_owner(S2C, |owner, control, _remote| {
        let completion = exchange_with(owner, control, entropy, |remote| {
            Ok(backend(
                remote,
                Script::new(
                    ScriptSend::Full,
                    vec![ScriptRead::Packet(fixtures::HEALTHY, remote)],
                ),
            ))
        })?;
        let committed = completion.commit(owner)?;
        let original = owner.status();
        let receipt = owner.last_receipt().map(super::Receipt::serial);
        // Its own Control is already Committed: this guard must not require Published.
        committed.check_control(control)?;
        super::with_owner(S2C, |foreign, foreign_control, _foreign_remote| {
            let before = foreign.status();
            assert!(matches!(
                committed.check_control(foreign_control),
                Err(Failure::Control)
            ));
            assert_eq!(foreign.status(), before);
            assert!(foreign.last_receipt().is_none());
            foreign_control.remaining(crate::nts_ke_experiment::IoPhase::Exchange)?;
            assert_eq!(owner.status(), original);
            assert_eq!(owner.last_receipt().map(super::Receipt::serial), receipt);
            committed.check_control(control)?;
            let summary = committed.into_summary();
            assert!(summary.transmitted);
            assert_eq!((summary.cookies, summary.cookie_bytes), (1, 12));
            assert_eq!(
                control.cancel()?,
                crate::nts_ke_experiment::CancelOutcome::AlreadyCommitted
            );
            Ok(())
        })
    })
}

/// A pre-producer refusal closes admission without replacing the latest real failure.
/// The helper is exercised directly; this does not claim deterministic OS preemption.
/// # Errors
/// Preserve checked fixture creation and genuine first-step results.
/// # Panics
/// Fail if a no-work refusal overwrites primary/cleanup, charges work or draws again.
#[test]
fn admission_refusal_preserves_prior_failure_and_has_no_prior_neighbor() -> Result<(), Failure> {
    let cause = Failure::Connect(io::ErrorKind::ConnectionReset);
    for secondary in [None, Some(Failure::Shutdown(io::ErrorKind::BrokenPipe))] {
        let actual = Failed {
            primary: cause,
            cleanup: secondary,
            admission_cleanup: None,
        };
        with_fixture(
            ExchangeMode::Preview,
            Some(experiment(4, 8, 16)?),
            vec![Err(actual), Ok(())],
            ExchangeCase::Healthy,
            |invocation, _provider, producer| {
                if secondary.is_none() {
                    deferred(&step_with(invocation, producer))?;
                } else {
                    assert_failure(
                        &failed_step(&step_with(invocation, producer))?,
                        cause,
                        secondary,
                        None,
                    );
                }
                let before = invocation.snapshot();
                let draws = producer.draw_bounds.clone();
                let refused = invocation.admission_failed(Failure::Deadline, End::Window);
                assert_failure(&refused, cause, secondary, None);
                assert_failure(
                    invocation.last_failure().ok_or(Failure::Control)?,
                    cause,
                    secondary,
                    None,
                );
                let after = invocation.snapshot();
                assert_eq!(after.end, Some(End::Window));
                assert_eq!(
                    (after.transactions, after.refusals, after.observations),
                    (before.transactions, before.refusals, before.observations)
                );
                assert_eq!(
                    (after.origin, after.cutoff, after.not_before, after.base),
                    (before.origin, before.cutoff, before.not_before, before.base)
                );
                assert_eq!(
                    (
                        after.source_not_before,
                        after.global_not_before,
                        after.intent_not_before
                    ),
                    (
                        before.source_not_before,
                        before.global_not_before,
                        before.intent_not_before
                    )
                );
                assert!(matches!(
                    step_with(invocation, producer),
                    Ok(Step::Closed(End::Window))
                ));
                assert_eq!(invocation.snapshot(), after);
                assert_eq!(producer.establish_calls, 1);
                assert_eq!(producer.draw_bounds, draws);
                Ok(())
            },
        )?;
    }
    with_fixture(
        ExchangeMode::Preview,
        Some(experiment(4, 8, 16)?),
        vec![Ok(())],
        ExchangeCase::Healthy,
        |invocation, _provider, producer| {
            let before = invocation.snapshot();
            assert!(invocation.last_failure().is_none());
            let refused = invocation.admission_failed(Failure::Deadline, End::Window);
            assert_failure(&refused, Failure::Deadline, None, None);
            assert_failure(
                invocation.last_failure().ok_or(Failure::Control)?,
                Failure::Deadline,
                None,
                None,
            );
            let after = invocation.snapshot();
            assert_eq!(
                (after.transactions, after.refusals, after.observations),
                (0, 0, 0)
            );
            assert_eq!((after.origin, after.cutoff), (before.origin, before.cutoff));
            assert!(matches!(
                step_with(invocation, producer),
                Ok(Step::Closed(End::Window))
            ));
            assert_eq!(invocation.snapshot(), after);
            assert_eq!(producer.establish_calls, 0);
            assert_eq!(producer.draw_bounds, [Duration::ZERO; 0]);
            Ok(())
        },
    )
}

/// The final receive allowance can still complete the original protected request.
/// # Errors
/// Preserve checked context construction and the actual shared protected operation.
/// # Panics
/// Fail if rejected replies renew work/deadline or trigger a replacement context.
#[test]
fn last_allowance_healthy_reply_stays_in_original_context() -> Result<(), Failure> {
    let context_lifetime = Duration::from_secs(1);
    let limits = experiment(4, 8, 16)?.with_context_lifetime(context_lifetime)?;
    with_fixture(
        ExchangeMode::ProtectedDatagram,
        Some(limits),
        vec![Ok(()), Ok(())],
        ExchangeCase::LastAllowanceHealthy,
        |invocation, _provider, producer| {
            let original = invocation.snapshot();
            let context_expiry = original
                .origin
                .checked_add(context_lifetime)
                .ok_or(Failure::Deadline)?;
            assert!(
                context_expiry.saturating_duration_since(Instant::now())
                    >= Duration::from_millis(750),
                "exhaustion fixture lost setup headroom before its first step"
            );
            let result = step_with(invocation, producer).map_err(|failure| failure.primary)?;
            let Step::Complete(summary) = result else {
                return Err(Failure::Control);
            };
            assert!(summary.transmitted);
            assert_eq!((summary.cookies, summary.cookie_bytes), (1, 12));
            let observed = producer.last_exchange.as_ref().ok_or(Failure::Control)?;
            assert_eq!(
                (
                    observed.before.attempts,
                    observed.before.frames,
                    observed.before.bytes
                ),
                (32, 32, 131_072)
            );
            assert_eq!(
                (
                    observed.after.attempts,
                    observed.after.frames,
                    observed.after.used,
                    observed.after.uids
                ),
                (31, 0, 1, 1)
            );
            let spent = fixtures::ZERO_COOKIES
                .len()
                .checked_mul(31)
                .and_then(|bytes| bytes.checked_add(fixtures::HEALTHY.len()))
                .ok_or(Failure::Control)?;
            assert_eq!(
                observed.after.bytes,
                131_072_usize.checked_sub(spent).ok_or(Failure::Control)?
            );
            assert_eq!(observed.after.phase, RequestPhase::Idle);
            assert!(observed.has_receipt);
            assert_eq!(
                (observed.expiry_before, observed.expiry_after),
                (context_expiry, context_expiry)
            );
            assert_eq!(producer.controls[0].expiry, context_expiry);
            assert!(context_expiry < original.cutoff);
            assert_eq!(
                producer
                    .backend_lifetime
                    .as_ref()
                    .ok_or(Failure::Control)?
                    .strong_count(),
                0
            );
            let completed = invocation.snapshot();
            assert_eq!(
                (completed.origin, completed.cutoff),
                (original.origin, original.cutoff)
            );
            assert_eq!(
                (
                    completed.transactions,
                    completed.refusals,
                    completed.observations
                ),
                (1, 0, 2)
            );
            assert_eq!(completed.intent_not_before, None);
            assert_eq!(completed.end, Some(End::Completed));
            assert_eq!(
                (
                    producer.establish_calls,
                    producer.exchange_calls,
                    producer.preview_calls
                ),
                (1, 1, 0)
            );
            assert_eq!(producer.draw_bounds, [Duration::ZERO; 0]);
            assert!(matches!(
                step_with(invocation, producer),
                Ok(Step::Closed(End::Completed))
            ));
            assert_eq!(invocation.snapshot(), completed);
            assert_eq!(producer.establish_calls, 1);
            Ok(())
        },
    )
}

/// Check only metadata captured around the actual thirty-two rejected replies.
/// No observation here changes the owner or supplies a recovery authorization.
/// # Errors
/// Refuse missing actual exchange or resource observations and checked byte arithmetic.
/// # Panics
/// Fail if the original operation refunds work, retains a resource or renews expiry.
fn assert_spent_original_context(
    producer: &ScriptProducer<'_>,
    context_expiry: Instant,
    cutoff: Instant,
) -> Result<(), Failure> {
    let observed = producer.last_exchange.as_ref().ok_or(Failure::Control)?;
    assert_eq!(
        (
            observed.before.attempts,
            observed.before.frames,
            observed.before.bytes
        ),
        (32, 32, 131_072)
    );
    assert_eq!(
        (
            observed.after.attempts,
            observed.after.frames,
            observed.after.used,
            observed.after.uids
        ),
        (31, 0, 1, 1)
    );
    let spent = fixtures::ZERO_COOKIES
        .len()
        .checked_mul(32)
        .ok_or(Failure::Control)?;
    assert_eq!(
        observed.after.bytes,
        131_072_usize.checked_sub(spent).ok_or(Failure::Control)?
    );
    assert!(observed.after.bytes > 0);
    assert_eq!(
        (observed.after.phase, observed.after.available),
        (RequestPhase::Idle, 0)
    );
    assert!(!observed.has_receipt);
    assert_eq!(
        (observed.expiry_before, observed.expiry_after),
        (context_expiry, context_expiry)
    );
    assert_eq!(producer.controls[0].expiry, context_expiry);
    assert!(context_expiry < cutoff);
    assert_eq!(
        producer
            .backend_lifetime
            .as_ref()
            .ok_or(Failure::Control)?
            .strong_count(),
        0
    );
    Ok(())
}

/// Real authenticated count refusals exhaust one context before any replacement.
/// # Errors
/// Preserve checked setup and actual shared-step scheduling/refusal observations.
/// # Panics
/// Fail if exhaustion is terminal, refunds work or admits early KE/redraw.
#[test]
fn receive_work_exhaustion_defers_replacement_without_renewing_context() -> Result<(), Failure> {
    let context_lifetime = Duration::from_secs(1);
    let limits = experiment(4, 8, 16)?.with_context_lifetime(context_lifetime)?;
    let cause = Failure::Ledger(LedgerError::Reply(super::ReplyError::Cookies));
    with_fixture(
        ExchangeMode::ProtectedDatagram,
        Some(limits),
        vec![Ok(()), Ok(())],
        ExchangeCase::ZeroCookies,
        |invocation, _provider, producer| {
            let original = invocation.snapshot();
            let context_expiry = original
                .origin
                .checked_add(context_lifetime)
                .ok_or(Failure::Deadline)?;
            assert!(
                context_expiry.saturating_duration_since(Instant::now())
                    >= Duration::from_millis(750),
                "exhaustion fixture lost setup headroom before its first step"
            );
            let result = step_with(invocation, producer);
            assert_spent_original_context(producer, context_expiry, original.cutoff)?;
            let exhausted = invocation.snapshot();
            assert_eq!(
                (exhausted.origin, exhausted.cutoff),
                (original.origin, original.cutoff)
            );
            assert_eq!(
                (
                    exhausted.transactions,
                    exhausted.refusals,
                    exhausted.observations
                ),
                (1, 0, 2)
            );
            assert_eq!(exhausted.intent_not_before, Some(context_expiry));
            assert_failure(
                invocation.last_failure().ok_or(Failure::Control)?,
                cause,
                None,
                None,
            );
            if let Err(failure) = &result {
                assert_failure(failure, cause, None, None);
            }
            // The old receiver has already produced every required genuine fact above.
            assert!(
                matches!(&result, Ok(Step::Deferred(_))),
                "exhausted original context must defer replacement"
            );
            let boundary = deferred(&result)?;
            assert!(boundary >= context_expiry && boundary < original.cutoff);
            assert_eq!(exhausted.not_before, boundary);
            assert_eq!(exhausted.end, None);
            assert_eq!(exhausted.base, Some(Duration::from_millis(40)));
            assert_eq!(
                (
                    producer.establish_calls,
                    producer.exchange_calls,
                    producer.preview_calls
                ),
                (1, 1, 0)
            );
            assert_eq!(producer.draw_bounds, [Duration::from_millis(20)]);
            assert!(
                Instant::now() < boundary,
                "exhaustion fixture lost its early-probe boundary"
            );
            assert_eq!(deferred(&step_with(invocation, producer))?, boundary);
            let early = invocation.snapshot();
            assert_eq!(
                (early.transactions, early.refusals, early.observations),
                (1, 1, 3)
            );
            assert_eq!(
                (early.origin, early.cutoff, early.not_before, early.base),
                (exhausted.origin, exhausted.cutoff, boundary, exhausted.base)
            );
            assert_eq!(early.intent_not_before, Some(context_expiry));
            assert_eq!(
                (
                    producer.establish_calls,
                    producer.exchange_calls,
                    producer.preview_calls
                ),
                (1, 1, 0)
            );
            assert_eq!(producer.draw_bounds, [Duration::from_millis(20)]);
            assert_failure(
                invocation.last_failure().ok_or(Failure::Control)?,
                cause,
                None,
                None,
            );
            Ok(())
        },
    )
}

/// Explicit component limits; these select no public lifetime or retry default.
/// # Errors
/// Preserve both mandatory checked limit constructors.
fn replacement_limits() -> Result<Limits, Failure> {
    experiment(4, 8, 16)?.with_context_lifetime(Duration::from_secs(1))
}

/// Invoke the actual first operation and observe its retained, genuine exhaustion.
/// This helper acts through production; it does not mint a retirement fact.
/// # Errors
/// Preserve setup, operation and scheduling refusals.
/// # Panics
/// Fail if the actual first context loses custody, budgets or its original cause.
fn exhaust_first_context(
    invocation: &mut Invocation<'_>,
    producer: &mut ScriptProducer<'_>,
) -> Result<(Instant, Snapshot), Failure> {
    let original = invocation.snapshot();
    let result = step_with(invocation, producer);
    assert!(
        matches!(&result, Ok(Step::Deferred(_))),
        "first genuine exhaustion did not defer"
    );
    let boundary = deferred(&result)?;
    let state = invocation.snapshot();
    let observed = producer.last_exchange.as_ref().ok_or(Failure::Control)?;
    assert_eq!(
        (
            observed.after.frames,
            observed.after.phase,
            observed.has_receipt
        ),
        (0, RequestPhase::Idle, false)
    );
    assert_eq!(observed.expiry_before, observed.expiry_after);
    assert_eq!(state.intent_not_before, Some(observed.expiry_before));
    assert!(boundary >= observed.expiry_before && boundary < original.cutoff);
    assert_eq!(
        (state.origin, state.cutoff),
        (original.origin, original.cutoff)
    );
    assert_eq!(
        (
            state.transactions,
            state.refusals,
            state.observations,
            state.contexts
        ),
        (1, 0, 2, 1)
    );
    assert!(state.has_previous);
    assert_eq!(state.base, Some(Duration::from_millis(40)));
    assert_eq!(producer.entropy_calls, 1);
    assert_eq!(
        producer
            .backend_lifetime
            .as_ref()
            .ok_or(Failure::Control)?
            .strong_count(),
        0
    );
    assert_failure(
        invocation.last_failure().ok_or(Failure::Control)?,
        Failure::Ledger(LedgerError::Reply(super::ReplyError::Cookies)),
        None,
        None,
    );
    Ok((boundary, state))
}

/// The one admitted successor reaches a genuine commit with distinct public material.
/// # Errors
/// Preserve checked construction, actual exhaustion/parking and protected completion.
/// # Panics
/// Fail if a context, original cutoff, exact observation pair or terminal commit renews.
#[test]
fn exhausted_context_replacement_commits_once_at_the_exact_observation_limit() -> Result<(), Failure>
{
    let limits = experiment(4, 8, 4)?.with_context_lifetime(Duration::from_secs(1))?;
    with_fixture(
        ExchangeMode::ProtectedDatagram,
        Some(limits),
        vec![Ok(()), Ok(()), Ok(())],
        ExchangeCase::ZeroCookies,
        |invocation, _provider, producer| {
            producer.successor = Some(SuccessorCase::Healthy);
            let (boundary, retired) = exhaust_first_context(invocation, producer)?;
            park_until(boundary)?;
            let Step::Complete(summary) =
                step_with(invocation, producer).map_err(|failure| failure.primary)?
            else {
                return Err(Failure::Control);
            };
            assert!(summary.transmitted);
            let complete = invocation.snapshot();
            assert_eq!(
                (complete.origin, complete.cutoff),
                (retired.origin, retired.cutoff)
            );
            assert_eq!(
                (
                    complete.transactions,
                    complete.refusals,
                    complete.observations,
                    complete.contexts
                ),
                (2, 0, 4, 2)
            );
            assert!(!complete.has_previous);
            assert_eq!((complete.base, complete.end), (None, Some(End::Completed)));
            assert_eq!(
                (
                    producer.establish_calls,
                    producer.exchange_calls,
                    producer.entropy_calls
                ),
                (2, 2, 2)
            );
            assert_eq!(producer.controls.len(), 2);
            assert!(producer.controls[1].origin >= boundary);
            assert!(producer.controls[1].expiry <= retired.cutoff);
            let observed = producer.last_exchange.as_ref().ok_or(Failure::Control)?;
            assert!(observed.has_receipt);
            assert_eq!(
                (observed.expiry_before, observed.expiry_after),
                (producer.controls[1].expiry, producer.controls[1].expiry)
            );
            assert_eq!(observed.after.frames, 31);
            assert_eq!(
                producer
                    .backend_lifetime
                    .as_ref()
                    .ok_or(Failure::Control)?
                    .strong_count(),
                0
            );
            assert!(matches!(
                step_with(invocation, producer),
                Ok(Step::Closed(End::Completed))
            ));
            assert_eq!(invocation.snapshot(), complete);
            assert_eq!((producer.establish_calls, producer.exchange_calls), (2, 2));
            Ok(())
        },
    )
}

/// Earlier rejects need not be authenticated; the final count error owns the marker.
/// # Errors
/// Preserve checked setup and the actual shared exhaustion path.
/// # Panics
/// Fail if recovery assumes thirty-two zero-cookie replies or refunds earlier bytes.
#[test]
fn mixed_rejections_and_final_authenticated_excess_count_can_retire() -> Result<(), Failure> {
    with_fixture(
        ExchangeMode::ProtectedDatagram,
        Some(replacement_limits()?),
        vec![Ok(())],
        ExchangeCase::MixedCountRefusal,
        |invocation, _provider, producer| {
            let (_boundary, state) = exhaust_first_context(invocation, producer)?;
            let observed = producer.last_exchange.as_ref().ok_or(Failure::Control)?;
            let spent = MALFORMED_RESPONSE
                .len()
                .checked_mul(31)
                .and_then(|n| n.checked_add(fixtures::NINE_COOKIES.len()))
                .ok_or(Failure::Control)?;
            assert_eq!(
                observed.after.bytes,
                131_072_usize.checked_sub(spent).ok_or(Failure::Control)?
            );
            assert_eq!((producer.establish_calls, producer.exchange_calls), (1, 1));
            assert!(state.has_previous);
            assert_eq!(producer.draw_bounds, [Duration::from_millis(20)]);
            Ok(())
        },
    )
}

/// An intervening KE failure cannot discard history or replace a later attach cause.
/// # Errors
/// Preserve genuine first exhaustion, finite waits and checked failure observations.
/// # Panics
/// Fail if retained history disappears, old cookies send, or attach replaces primary.
#[test]
fn predecessor_history_survives_failed_ke_and_preserves_latest_attach_primary()
-> Result<(), Failure> {
    let cause = Failure::Connect(io::ErrorKind::ConnectionReset);
    with_fixture(
        ExchangeMode::ProtectedDatagram,
        Some(replacement_limits()?),
        vec![Ok(()), Err(failed(cause)), Ok(())],
        ExchangeCase::ZeroCookies,
        |invocation, _provider, producer| {
            producer.successor = Some(SuccessorCase::OldCookie);
            let (first_boundary, original) = exhaust_first_context(invocation, producer)?;
            park_until(first_boundary)?;
            let second_boundary = deferred(&step_with(invocation, producer))?;
            let failed_ke = invocation.snapshot();
            assert_eq!(
                (
                    failed_ke.transactions,
                    failed_ke.observations,
                    failed_ke.contexts
                ),
                (2, 4, 1)
            );
            assert!(failed_ke.has_previous);
            assert_eq!(failed_ke.base, Some(Duration::from_millis(80)));
            assert_eq!(
                (failed_ke.origin, failed_ke.cutoff),
                (original.origin, original.cutoff)
            );
            assert_failure(
                invocation.last_failure().ok_or(Failure::Control)?,
                cause,
                None,
                None,
            );
            park_until(second_boundary)?;
            let failure = failed_step(&step_with(invocation, producer))?;
            assert_failure(&failure, cause, None, None);
            let terminal = invocation.snapshot();
            assert_eq!(
                terminal.end,
                Some(End::Context(
                    ContextStage::Attach,
                    Failure::Ledger(LedgerError::Cookies)
                ))
            );
            assert_eq!(
                (
                    terminal.transactions,
                    terminal.observations,
                    terminal.contexts
                ),
                (3, 6, 2)
            );
            assert_eq!(
                (
                    producer.establish_calls,
                    producer.exchange_calls,
                    producer.entropy_calls
                ),
                (3, 1, 1)
            );
            assert_eq!(producer.draw_bounds, [Duration::from_millis(20); 2]);
            assert!(matches!(
                step_with(invocation, producer),
                Ok(Step::Closed(End::Context(ContextStage::Attach, _)))
            ));
            assert_eq!(invocation.snapshot(), terminal);
            Ok(())
        },
    )
}

/// Check two discarded old packets followed by one accepted fresh packet.
/// These metadata checks cannot perform admission or replenish the real pending request.
/// # Panics
/// Fail if either old packet commits, any budget is refunded or expiry is renewed.
fn assert_old_packets_then_healthy(observed: &ExchangeObservation) {
    assert_eq!(fixtures::HEALTHY.len(), 140);
    assert_eq!(fixtures::SUCCESSOR_OLD_S2C.len(), 140);
    assert_eq!(fixtures::SUCCESSOR_HEALTHY.len(), 140);
    assert_eq!(
        (observed.before.frames, observed.before.bytes),
        (32, 131_072)
    );
    assert_eq!((observed.after.frames, observed.after.bytes), (29, 130_652));
    assert_eq!(
        (
            observed.after.attempts,
            observed.after.used,
            observed.after.uids
        ),
        (31, 1, 1)
    );
    assert_eq!(observed.after.phase, RequestPhase::Idle);
    assert_eq!(observed.expiry_before, observed.expiry_after);
    assert!(observed.has_receipt);
}

/// Inspect the actual selected successor result against independently fixed expectations.
/// Only the driver can have produced the completion or its observed refusal.
/// # Errors
/// Preserve unexpected results and refuse cases outside this closed test group.
/// # Panics
/// Fail if old material commits or a fresh neighboring reply fails to complete.
fn assert_successor_outcome(
    selected: SuccessorCase,
    result: &Result<Step, Failed>,
    observed: &ExchangeObservation,
) -> Result<End, Failure> {
    let end = match selected {
        SuccessorCase::MixedCookie | SuccessorCase::OldPacketsThenHealthy => {
            let Step::Complete(summary) = result.as_ref().map_err(|failure| failure.primary)?
            else {
                return Err(Failure::Control);
            };
            assert!(summary.transmitted);
            assert_eq!((summary.cookies, summary.cookie_bytes), (1, 12));
            assert_eq!(observed.before.available, 1);
            assert!(observed.has_receipt);
            if matches!(selected, SuccessorCase::OldPacketsThenHealthy) {
                assert_old_packets_then_healthy(observed);
            }
            End::Completed
        }
        SuccessorCase::RepeatedUid => {
            assert_failure(
                &failed_step(result)?,
                Failure::Ledger(LedgerError::RepeatedUid),
                None,
                None,
            );
            assert_eq!(observed.after.used, observed.before.used);
            assert!(!observed.has_receipt);
            End::Failure
        }
        SuccessorCase::OldS2cOnly => {
            let failure = failed_step(result)?;
            assert!(matches!(
                failure.primary,
                Failure::Ledger(LedgerError::Reply(super::ReplyError::Open(_)))
            ));
            assert_eq!(observed.after.frames, 0);
            assert!(!observed.has_receipt);
            End::Failure
        }
        SuccessorCase::StaleReturned => {
            assert_failure(
                &failed_step(result)?,
                Failure::Ledger(LedgerError::Ticket),
                None,
                None,
            );
            End::Failure
        }
        SuccessorCase::ZeroCookies => {
            assert_failure(
                &failed_step(result)?,
                Failure::Ledger(LedgerError::Reply(super::ReplyError::Cookies)),
                None,
                None,
            );
            assert_eq!(observed.after.frames, 0);
            End::Failure
        }
        _ => return Err(Failure::Control),
    };
    Ok(end)
}

/// Current material succeeds beside inherited-cookie, UID, key and receipt refusals.
/// # Errors
/// Preserve actual component setup, parking and operation-result observation.
/// # Panics
/// Fail if old facts send/commit, or a second failed context admits a third KE.
#[test]
fn successor_history_and_old_key_or_stale_results_have_fresh_neighbors() -> Result<(), Failure> {
    for selected in [
        SuccessorCase::MixedCookie,
        SuccessorCase::RepeatedUid,
        SuccessorCase::OldS2cOnly,
        SuccessorCase::OldPacketsThenHealthy,
        SuccessorCase::StaleReturned,
        SuccessorCase::ZeroCookies,
    ] {
        with_fixture(
            ExchangeMode::ProtectedDatagram,
            Some(replacement_limits()?),
            vec![Ok(()), Ok(()), Ok(())],
            ExchangeCase::ZeroCookies,
            |invocation, _provider, producer| {
                producer.successor = Some(selected);
                let (boundary, original) = exhaust_first_context(invocation, producer)?;
                park_until(boundary)?;
                let result = step_with(invocation, producer);
                let observed = producer.last_exchange.as_ref().ok_or(Failure::Control)?;
                let end = assert_successor_outcome(selected, &result, observed)?;
                let terminal = invocation.snapshot();
                assert_eq!(
                    (
                        terminal.transactions,
                        terminal.observations,
                        terminal.contexts
                    ),
                    (2, 4, 2)
                );
                assert_eq!(
                    (terminal.origin, terminal.cutoff),
                    (original.origin, original.cutoff)
                );
                assert_eq!(terminal.end, Some(end));
                assert_eq!(
                    (
                        producer.establish_calls,
                        producer.exchange_calls,
                        producer.entropy_calls
                    ),
                    (2, 2, 2)
                );
                assert_eq!(
                    producer
                        .backend_lifetime
                        .as_ref()
                        .ok_or(Failure::Control)?
                        .strong_count(),
                    0
                );
                assert!(
                    matches!(step_with(invocation, producer), Ok(Step::Closed(actual)) if actual == end)
                );
                assert_eq!(invocation.snapshot(), terminal);
                assert_eq!(producer.establish_calls, 2);
                Ok(())
            },
        )?;
    }
    Ok(())
}

/// Tightened independent waits and refused probes retain the one committed draw.
/// # Errors
/// Preserve checked setup and actual shared refusal results.
/// # Panics
/// Fail if probes renew admission, history, cutoff or the finite refusal allowance.
#[test]
fn source_global_and_refusal_guards_keep_exhausted_context_history() -> Result<(), Failure> {
    with_fixture(
        ExchangeMode::ProtectedDatagram,
        Some(replacement_limits()?),
        vec![Ok(())],
        ExchangeCase::ZeroCookies,
        |invocation, _provider, producer| {
            let (_boundary, retired) = exhaust_first_context(invocation, producer)?;
            let source = retired
                .origin
                .checked_add(Duration::from_millis(1500))
                .ok_or(Failure::Deadline)?;
            let global = retired
                .origin
                .checked_add(Duration::from_secs(2))
                .ok_or(Failure::Deadline)?;
            invocation.constrain_source_until(source);
            invocation.constrain_global_until(global);
            assert_eq!(invocation.plan_at(global), Plan::Due);
            for count in 1..=8 {
                assert!(
                    Instant::now() < global,
                    "exhaustion fixture lost its refusal boundary"
                );
                let result = step_with(invocation, producer);
                if count == 8 {
                    assert!(matches!(result, Ok(Step::Closed(End::Refusals))));
                } else {
                    assert_eq!(deferred(&result)?, global);
                }
                let state = invocation.snapshot();
                assert_eq!(
                    (
                        state.transactions,
                        state.refusals,
                        state.observations,
                        state.contexts
                    ),
                    (1, count, 2 + count, 1)
                );
                assert!(state.has_previous);
                assert_eq!(
                    (state.source_not_before, state.global_not_before),
                    (source, global)
                );
                assert_eq!(
                    (state.origin, state.cutoff, state.base),
                    (retired.origin, retired.cutoff, retired.base)
                );
                assert_eq!(state.intent_not_before, retired.intent_not_before);
            }
            assert_eq!(
                (
                    producer.establish_calls,
                    producer.exchange_calls,
                    producer.entropy_calls
                ),
                (1, 1, 1)
            );
            assert_eq!(producer.draw_bounds, [Duration::from_millis(20)]);
            Ok(())
        },
    )
}

/// Exhaustion cannot draw for an impossible producer, observation pair or window.
/// # Errors
/// Preserve explicit checked inputs and actual original failure observation.
/// # Panics
/// Fail if a refusal refunds the original operation or schedules impossible work.
#[test]
fn replacement_allowances_and_outer_cutoff_close_without_refund() -> Result<(), Failure> {
    for (transactions, observations, lifetime, expected) in [
        (1, 16, Duration::from_secs(1), End::Transactions),
        (4, 3, Duration::from_secs(1), End::Observations),
        (4, 16, Duration::from_secs(5), End::Window),
    ] {
        let limits = experiment(transactions, 8, observations)?.with_context_lifetime(lifetime)?;
        with_fixture(
            ExchangeMode::ProtectedDatagram,
            Some(limits),
            vec![Ok(()), Ok(())],
            ExchangeCase::ZeroCookies,
            |invocation, _provider, producer| {
                let original = invocation.snapshot();
                let failure = failed_step(&step_with(invocation, producer))?;
                assert_failure(
                    &failure,
                    Failure::Ledger(LedgerError::Reply(super::ReplyError::Cookies)),
                    None,
                    None,
                );
                let closed = invocation.snapshot();
                assert_eq!(closed.end, Some(expected));
                assert_eq!(
                    (
                        closed.transactions,
                        closed.refusals,
                        closed.observations,
                        closed.contexts
                    ),
                    (1, 0, 2, 1)
                );
                assert_eq!(
                    (closed.origin, closed.cutoff),
                    (original.origin, original.cutoff)
                );
                assert_eq!((producer.establish_calls, producer.exchange_calls), (1, 1));
                assert_eq!(producer.draw_bounds, [Duration::ZERO; 0]);
                assert!(
                    matches!(step_with(invocation, producer), Ok(Step::Closed(end)) if end == expected)
                );
                assert_eq!(invocation.snapshot(), closed);
                Ok(())
            },
        )?;
    }
    Ok(())
}

/// The marker's neighboring local/control/parser/unknown errors grant no KE route.
/// # Errors
/// Preserve setup and the actual operation's original failure.
/// # Panics
/// Fail if a copied error/intent grants history, work or fresh synchronization.
#[test]
fn cancelled_retirement_and_unsafe_mode_errors_never_open_recovery() -> Result<(), Failure> {
    for selected in [
        ExchangeCase::CancelZeroCookies,
        ExchangeCase::UnknownExtent,
        ExchangeCase::MalformedOnly,
        ExchangeCase::ReceiveReset,
        ExchangeCase::SendUnknown,
        ExchangeCase::EntropyFailure,
    ] {
        with_fixture(
            ExchangeMode::ProtectedDatagram,
            Some(replacement_limits()?),
            vec![Ok(()), Ok(())],
            selected,
            |invocation, _provider, producer| {
                let failure = failed_step(&step_with(invocation, producer))?;
                let expected_end = match selected {
                    ExchangeCase::CancelZeroCookies => {
                        assert_failure(
                            &failure,
                            Failure::Ledger(LedgerError::Reply(super::ReplyError::Cookies)),
                            None,
                            None,
                        );
                        End::Context(ContextStage::Retire, Failure::Cancelled)
                    }
                    ExchangeCase::UnknownExtent => {
                        assert_failure(
                            &failure,
                            Failure::Ledger(LedgerError::ReceiveWork),
                            None,
                            None,
                        );
                        End::Failure
                    }
                    ExchangeCase::MalformedOnly => {
                        assert!(matches!(
                            failure.primary,
                            Failure::Ledger(LedgerError::Reply(super::ReplyError::Frame(_)))
                        ));
                        End::Failure
                    }
                    ExchangeCase::ReceiveReset => {
                        assert_failure(
                            &failure,
                            Failure::Datagram(super::DatagramError::Receive(
                                io::ErrorKind::ConnectionReset,
                            )),
                            None,
                            None,
                        );
                        End::Failure
                    }
                    ExchangeCase::SendUnknown => {
                        assert_failure(&failure, Failure::Ledger(LedgerError::Unknown), None, None);
                        End::Failure
                    }
                    ExchangeCase::EntropyFailure => {
                        assert_failure(&failure, Failure::Entropy, None, None);
                        End::Failure
                    }
                    _ => return Err(Failure::Control),
                };
                let terminal = invocation.snapshot();
                assert_eq!(
                    (
                        terminal.transactions,
                        terminal.observations,
                        terminal.contexts
                    ),
                    (1, 2, 1)
                );
                assert!(!terminal.has_previous);
                assert_eq!(terminal.end, Some(expected_end));
                assert_eq!((producer.establish_calls, producer.exchange_calls), (1, 1));
                assert_eq!(producer.draw_bounds, [Duration::ZERO; 0]);
                assert!(
                    matches!(step_with(invocation, producer), Ok(Step::Closed(end)) if end == expected_end)
                );
                assert_eq!(invocation.snapshot(), terminal);
                Ok(())
            },
        )?;
    }
    Ok(())
}

/// Stop/cleanup during replacement establishment preserves history and original facts.
/// # Errors
/// Preserve genuine first exhaustion, bounded wait and subsequent KE failure.
/// # Panics
/// Fail if cancellation/cleanup retries or loses the newly admitted operation's cause.
#[test]
fn cancelled_or_failed_replacement_cleanup_cannot_reopen_history() -> Result<(), Failure> {
    let primary = Failure::Connect(io::ErrorKind::TimedOut);
    for (cancel, cleanup, admission) in [
        (true, None, None),
        (
            false,
            Some(Failure::Shutdown(io::ErrorKind::BrokenPipe)),
            None,
        ),
        (false, None, Some(Failure::Control)),
    ] {
        let injected = Failed {
            primary,
            cleanup,
            admission_cleanup: admission,
        };
        with_fixture(
            ExchangeMode::ProtectedDatagram,
            Some(replacement_limits()?),
            vec![Ok(()), Err(injected), Ok(())],
            ExchangeCase::ZeroCookies,
            |invocation, _provider, producer| {
                producer.successor = Some(SuccessorCase::Healthy);
                let (boundary, original) = exhaust_first_context(invocation, producer)?;
                producer.cancel_establish_failure = cancel;
                park_until(boundary)?;
                let failure = failed_step(&step_with(invocation, producer))?;
                assert_failure(&failure, primary, cleanup, admission);
                let terminal = invocation.snapshot();
                assert_eq!(terminal.end, Some(End::Failure));
                assert_eq!(
                    (
                        terminal.transactions,
                        terminal.observations,
                        terminal.contexts
                    ),
                    (2, 4, 1)
                );
                assert!(terminal.has_previous);
                assert_eq!(
                    (terminal.origin, terminal.cutoff),
                    (original.origin, original.cutoff)
                );
                assert_eq!(
                    (
                        producer.establish_calls,
                        producer.exchange_calls,
                        producer.entropy_calls
                    ),
                    (2, 1, 1)
                );
                assert_eq!(producer.draw_bounds, [Duration::from_millis(20)]);
                assert!(matches!(
                    step_with(invocation, producer),
                    Ok(Step::Closed(End::Failure))
                ));
                assert_eq!(invocation.snapshot(), terminal);
                Ok(())
            },
        )?;
    }
    Ok(())
}

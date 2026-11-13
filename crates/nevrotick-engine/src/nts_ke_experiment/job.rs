// SPDX-License-Identifier: GPL-2.0-only
//! Permanent custody of one complete synchronous laboratory invocation.
//!
//! Only static slots are constructed. Observers borrow them and own no thread or
//! completion resource. Explicit collection joins outside every gate; it promises
//! neither finite join latency nor a Windows service-stop arrangement.

use std::any::Any;
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock, TryLockError};
use std::thread::{self, JoinHandle};
use std::time::Instant;

use super::process::{ProcessAdmission, ProcessClaim, ProviderScope};
use super::{Config, Control, Failed, Failure, InvocationCompletion, IoPhase, StopHandle};

#[cfg(test)]
#[path = "job_tests.rs"]
mod tests;

/// Requested stack size only; OS rounding and actual native usage remain unqualified.
const STACK_BYTES: usize = 2 * 1024 * 1024;

/// Adapt the real native constructor with a late-bound claim/scope lifetime.
/// The returned scope owns this actual claim and retains its original admission lifetime.
#[cfg(windows)]
/// # Errors
/// Preserve local Native admission refusal before any shutdown handle is constructed.
fn native_provider(claim: ProcessClaim<'_>) -> Result<ProviderScope<'_>, Failure> {
    ProviderScope::new(claim)
}

/// One owning historical completion or closed failure; no secret/native owner survives.
pub(super) type Operation = Result<InvocationCompletion, Failed>;

/// Checked original input and Control, owned before any thread submission.
pub(super) struct PreparedInvocation {
    /// Actual immutable checked input, consumed by the one whole-call owner.
    config: Config,
    /// Original checked deadlines and stop scope, never renewed by startup.
    control: Control,
}

impl PreparedInvocation {
    /// Capture the origin once and derive the actual stop endpoint before submission.
    /// # Errors
    /// Refuse deadline arithmetic without creating a submitted job.
    pub(super) fn prepare(config: Config) -> Result<(Self, StopHandle), Failed> {
        let control = Control::new(Instant::now()).map_err(Failed::from)?;
        let stop = StopHandle {
            scope: std::sync::Arc::clone(&control.stop_scope),
        };
        Ok((Self { config, control }, stop))
    }

    /// Retain a genuinely checked expired Control for the actual startup refusal probe.
    /// This test seam grants no caller-selected clock to normal preparation.
    #[cfg(test)]
    pub(super) fn checked_fixture(config: Config, control: Control) -> Self {
        Self { config, control }
    }

    /// Observe original checked timestamps without transferring Control authority.
    #[cfg(test)]
    pub(super) fn original_times(&self) -> (Instant, Instant, Instant) {
        (
            self.control.origin,
            self.control.handshake_deadline,
            self.control.expiry,
        )
    }
}

/// Once-written facts remain private until actual join; retirement is a separate fact.
struct Facts {
    /// Permanently set before retirement; only the actual joined collector moves its payload.
    operation: OnceLock<Mutex<Option<Operation>>>,
    /// True only after the actual provider and Config retirement boundary returned.
    retired: AtomicBool,
}

impl Facts {
    /// Reserve the sole typed result location without submitting work.
    const fn new() -> Self {
        Self {
            operation: OnceLock::new(),
            retired: AtomicBool::new(false),
        }
    }

    /// Store the original closed result once, preserving every secondary cause.
    /// # Errors
    /// Duplicate recording refuses without overwriting or reopening the first fact,
    /// including after the joined collector has moved its payload. The worker never
    /// locks the inner cell.
    fn record(&self, operation: Operation) -> Result<(), Failure> {
        self.operation
            .set(Mutex::new(Some(operation)))
            .map_err(|_| Failure::Control)
    }
}

/// Closed submission failure; no variant acknowledges a completed operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum SubmitError {
    /// The slot gate is temporarily held; no second job is queued.
    Busy,
    /// This permanent slot has already attempted submission.
    Used,
    /// The slot gate cannot establish current custody.
    Control,
    /// Builder refused before executing the closure; retain only its typed kind.
    Spawn(io::ErrorKind),
}

/// Explicit synchronous collector refusal, distinct from operation/submission failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum CollectError {
    /// A submission or another collector currently holds custody.
    Busy,
    /// No actual submitted handle is available to join or result to take.
    NotReady,
    /// The worker cannot collect its own thread.
    SelfJoin,
    /// An entered Tokio context cannot become the blocking synchronous collector.
    EnteredRuntime,
    /// The slot gate is unavailable.
    Control,
}

/// Actual joined termination, independently interpreted from historical operation facts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Termination {
    /// The shared whole-call owner returned after confirmed ordinary retirement.
    Returned,
    /// Join observed unwinding; no normal completion or cleanup is asserted.
    Panicked,
    /// The whole-call owner refused its internal once-recorded result contract.
    Contract(Failure),
}

/// Only actual joined return exposes operation facts and its separate retirement marker.
pub(super) struct JoinedOutcome {
    /// Historical result, if genuinely recorded before termination/retirement.
    pub(super) operation: Option<Operation>,
    /// Actual joined termination, never inferred from a timer or stop acknowledgement.
    pub(super) termination: Termination,
    /// Ordinary retirement actually crossed; false remains unconfirmed.
    pub(super) retired: bool,
}

/// Secret-free observation; a finished hint alone conveys no joined result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the retained observer awaits the native controller; portable controls exercise it"
    )
)]
pub(super) enum Observation {
    /// Observation deadline passed; slot and capacity are untouched.
    TimedOut,
    /// The slot gate is held; its state is deliberately unknown to this observer.
    Busy,
    /// The actual gate cannot establish an observation.
    Control,
    /// No submission has been attempted.
    Unused,
    /// One worker is physically outstanding; the bool is only an `is_finished` hint.
    Running(bool),
    /// The designated collector retains the actual handle outside all gates.
    Joining,
    /// Actual join completed, but result facts remain withheld until take.
    Joined,
    /// The result was already consumed; submission remains permanently closed.
    Taken,
    /// Submission failed without a job or physical-completion fact.
    NotSubmitted(SubmitError),
}

/// Irreversible slot phase; handles exist only here or in the explicit collector.
enum Phase {
    /// No submission has been attempted.
    Unused,
    /// Handle/result capacity reserved while Builder submission is serialized.
    Submitting,
    /// The one actual handle remains in permanent custody.
    Running(JoinHandle<Result<(), Failure>>),
    /// The designated collector owns that handle on its synchronous call stack.
    Joining,
    /// Actual join returned; one owned history or closed failure may be consumed.
    Joined(JoinedOutcome),
    /// Final typed facts were consumed, without reopening admission.
    Taken,
    /// Terminal pre-submission refusal; no completion is invented.
    NotSubmitted(
        #[cfg_attr(
            not(test),
            expect(
                dead_code,
                reason = "the stored refusal awaits the native observation controller; portable controls read it"
            )
        )]
        SubmitError,
    ),
}

/// Slot gate protects phase plus one sealed unwind payload, never exposed or dropped.
struct Custody {
    /// The original one-way phase owner.
    phase: Phase,
    /// At most one opaque unwind payload; normal collection never runs its destructor.
    /// Permanent static custody bounds count, not bytes, privacy or zeroization.
    sealed_panic: Option<Box<dyn Any + Send + 'static>>,
}

/// One process-lifetime fixed owner; local owners and destructor-based completion are absent.
pub(super) struct WorkerSlot {
    /// Separate from Control scope/phase gates, never held for join/provider work.
    custody: Mutex<Custody>,
    /// Fixed typed facts reserved before any thread submission.
    facts: Facts,
}

/// Closed Linux test requests; no caller-selected size enters ordinary submission.
#[cfg(all(test, target_os = "linux", target_env = "gnu", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub(super) enum StackRequest {
    /// Preserve the shared production request and its actual healthy worker.
    Selected,
    /// Exercise Builder refusal with one fixed aligned request outside user address space.
    InvalidAligned,
}

/// Test-only controls; callbacks execute outside slot and authority gates.
/// Pure submission-fault selection remains inside the custody transaction.
#[cfg_attr(not(test), derive(Clone, Copy))]
pub(super) struct Hooks {
    /// Trusted pre-Builder refusal; it is not an OS exhaustion observation.
    #[cfg(test)]
    pub(super) spawn_refusal: Option<io::ErrorKind>,
    /// Select only the capability-checked Linux Builder fault, without bypassing spawn.
    #[cfg(all(test, target_os = "linux", target_env = "gnu", target_arch = "x86_64"))]
    pub(super) stack_request: StackRequest,
    /// Hold actual startup before its mandatory fresh guard and permanent claim.
    #[cfg(test)]
    pub(super) before_start: Option<Box<dyn FnOnce() + Send>>,
    /// Hold or fault after actual operation or factory-refusal recording, before retirement.
    #[cfg(test)]
    pub(super) before_retire: Option<Box<dyn FnOnce() + Send>>,
    /// Observe the genuine provider-before-Config destruction boundary.
    #[cfg(test)]
    pub(super) after_provider: Option<Box<dyn FnOnce() + Send>>,
}

impl Hooks {
    /// Normal execution injects no callbacks, alternate provider or spawn failure.
    pub(super) const fn none() -> Self {
        Self {
            #[cfg(test)]
            spawn_refusal: None,
            #[cfg(all(test, target_os = "linux", target_env = "gnu", target_arch = "x86_64"))]
            stack_request: StackRequest::Selected,
            #[cfg(test)]
            before_start: None,
            #[cfg(test)]
            before_retire: None,
            #[cfg(test)]
            after_provider: None,
        }
    }
}

impl WorkerSlot {
    /// Construct only a permanent static slot with one unused result/handle location.
    pub(super) const fn new() -> Self {
        Self {
            custody: Mutex::new(Custody {
                phase: Phase::Unused,
                sealed_panic: None,
            }),
            facts: Facts::new(),
        }
    }

    /// Submit exactly one real Native invocation; no entered Tokio context is inherited.
    /// # Errors
    /// Refuse slot contention/reuse or typed pre-spawn failure without creating a job.
    #[cfg(windows)]
    pub(super) fn try_submit(
        &'static self,
        prepared: PreparedInvocation,
    ) -> Result<(), SubmitError> {
        self.submit_with(
            prepared,
            &super::PROCESS_ADMISSION,
            native_provider,
            |config, provider, control| {
                super::drive_invocation_with(
                    config,
                    provider,
                    control,
                    &mut super::recovery::Native,
                )
            },
            Hooks::none(),
        )
    }

    /// Reserve fixed custody before the actual Builder spawn/handle-install transaction.
    /// The selected provider and operation execute only on the one ordinary worker.
    /// # Errors
    /// Refuse contention, reuse, poisoned custody or spawn without a queued alternative.
    pub(super) fn submit_with<F>(
        &'static self,
        prepared: PreparedInvocation,
        admission: &'static ProcessAdmission,
        make_provider: for<'a> fn(ProcessClaim<'a>) -> Result<ProviderScope<'a>, Failure>,
        execute: F,
        hooks: Hooks,
    ) -> Result<(), SubmitError>
    where
        F: for<'a> FnOnce(&'a Config, &'a ProviderScope<'a>, Control) -> Operation + Send + 'static,
    {
        let mut custody = match self.custody.try_lock() {
            Ok(custody) => custody,
            Err(TryLockError::WouldBlock) => return Err(SubmitError::Busy),
            Err(TryLockError::Poisoned(_)) => return Err(SubmitError::Control),
        };
        if !matches!(custody.phase, Phase::Unused) {
            return Err(SubmitError::Used);
        }
        custody.phase = Phase::Submitting;
        #[cfg(test)]
        if let Some(kind) = hooks.spawn_refusal {
            let failure = SubmitError::Spawn(kind);
            custody.phase = Phase::NotSubmitted(failure);
            return Err(failure);
        }
        let builder = thread::Builder::new().stack_size(STACK_BYTES);
        #[cfg(all(test, target_os = "linux", target_env = "gnu", target_arch = "x86_64"))]
        let builder = match hooks.stack_request {
            StackRequest::Selected => builder,
            StackRequest::InvalidAligned => builder.stack_size(1usize << 63),
        };
        let submitted = builder.spawn(move || {
            run_prepared_with(
                prepared,
                admission,
                make_provider,
                execute,
                &self.facts,
                hooks,
            )
        });
        match submitted {
            Ok(handle) => {
                custody.phase = Phase::Running(handle);
                Ok(())
            }
            Err(error) => {
                let failure = SubmitError::Spawn(error.kind());
                custody.phase = Phase::NotSubmitted(failure);
                Err(failure)
            }
        }
    }

    /// Borrow ordinary observation without transferring any thread/result custody.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "the retained observer awaits the native controller; portable controls exercise it"
        )
    )]
    pub(super) fn observer(&'static self) -> Observer {
        Observer { slot: self }
    }

    /// Join only on the designated synchronous collector, outside every gate.
    /// This explicit operation may wait for native/thread-local destruction indefinitely.
    /// # Errors
    /// Refuse self-join, no actual handle, competing collection or unavailable custody.
    pub(super) fn collect(&self) -> Result<(), CollectError> {
        if tokio::runtime::Handle::try_current().is_ok() {
            return Err(CollectError::EnteredRuntime);
        }
        let handle = {
            let mut custody = match self.custody.try_lock() {
                Ok(custody) => custody,
                Err(TryLockError::WouldBlock) => return Err(CollectError::Busy),
                Err(TryLockError::Poisoned(_)) => return Err(CollectError::Control),
            };
            // Keep the possibly panicking std current-thread lookup before handle transfer.
            if matches!(
                &custody.phase,
                Phase::Running(handle) if handle.thread().id() == thread::current().id()
            ) {
                return Err(CollectError::SelfJoin);
            }
            match std::mem::replace(&mut custody.phase, Phase::Joining) {
                Phase::Running(handle) => handle,
                phase @ (Phase::Submitting | Phase::Joining) => {
                    custody.phase = phase;
                    return Err(CollectError::Busy);
                }
                phase @ (Phase::Unused
                | Phase::Joined(_)
                | Phase::Taken
                | Phase::NotSubmitted(_)) => {
                    custody.phase = phase;
                    return Err(CollectError::NotReady);
                }
            }
        };
        let joined = handle.join();
        // No worker locks this cell, and no lock spans the actual join or teardown.
        // Recover poison solely to preserve the original owned operation; never heal it.
        let (operation, facts_poisoned) = match self.facts.operation.get() {
            Some(cell) => match cell.lock() {
                Ok(mut operation) => (operation.take(), false),
                Err(poisoned) => {
                    let mut operation = poisoned.into_inner();
                    (operation.take(), true)
                }
            },
            None => (None, false),
        };
        let retired = self.facts.retired.load(Ordering::Acquire);
        // No other operation mutates Joining, and observers never hold this gate.
        // Even an unavailable gate must retain, rather than destroy, an unwind payload.
        // Poison is not repaired into a normal-return claim.
        let (mut custody, custody_poisoned) = match self.custody.lock() {
            Ok(custody) => (custody, false),
            Err(poisoned) => (poisoned.into_inner(), true),
        };
        let termination = match joined {
            Ok(Ok(()))
                if operation.is_some() && retired && !facts_poisoned && !custody_poisoned =>
            {
                Termination::Returned
            }
            Ok(Ok(())) => Termination::Contract(Failure::Control),
            Ok(Err(failure)) => Termination::Contract(failure),
            Err(payload) => {
                custody.sealed_panic = Some(payload);
                Termination::Panicked
            }
        };
        custody.phase = Phase::Joined(JoinedOutcome {
            operation,
            termination,
            retired,
        });
        Ok(())
    }

    /// Consume final typed facts once after actual join, retaining any sealed payload.
    /// # Errors
    /// Refuse a premature/duplicate take, contention or unavailable custody.
    pub(super) fn take_result(&self) -> Result<JoinedOutcome, CollectError> {
        let mut custody = match self.custody.try_lock() {
            Ok(custody) => custody,
            Err(TryLockError::WouldBlock) => return Err(CollectError::Busy),
            Err(TryLockError::Poisoned(_)) => return Err(CollectError::Control),
        };
        match std::mem::replace(&mut custody.phase, Phase::Taken) {
            Phase::Joined(outcome) => Ok(outcome),
            phase @ (Phase::Unused
            | Phase::Submitting
            | Phase::Running(_)
            | Phase::Joining
            | Phase::Taken
            | Phase::NotSubmitted(_)) => {
                custody.phase = phase;
                Err(CollectError::NotReady)
            }
        }
    }

    /// Observe only the count of sealed payloads, without borrowing or formatting one.
    /// # Errors
    /// Refuse unavailable custody without inspecting or disposing any payload.
    #[cfg(test)]
    pub(super) fn sealed_count(&self) -> Result<usize, CollectError> {
        self.custody
            .try_lock()
            .map(|custody| usize::from(custody.sealed_panic.is_some()))
            .map_err(|_| CollectError::Busy)
    }
}

/// Ordinary observation can expire or be abandoned without dropping an actual owner.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the retained observer awaits the native controller; portable controls exercise it"
    )
)]
pub(super) struct Observer {
    /// Only a borrow of permanent custody; no handle, permit or result is transferred.
    slot: &'static WorkerSlot,
}

impl Observer {
    /// Inspect nonblockingly until this observer's own deadline, without state changes.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "the retained observer awaits the native controller; portable controls exercise it"
        )
    )]
    pub(super) fn poll_until(&self, deadline: Instant) -> Observation {
        if Instant::now() >= deadline {
            return Observation::TimedOut;
        }
        let custody = match self.slot.custody.try_lock() {
            Ok(custody) => custody,
            Err(TryLockError::WouldBlock) => return Observation::Busy,
            Err(TryLockError::Poisoned(_)) => return Observation::Control,
        };
        match &custody.phase {
            Phase::Unused => Observation::Unused,
            Phase::Submitting => Observation::Busy,
            Phase::Running(handle) => Observation::Running(handle.is_finished()),
            Phase::Joining => Observation::Joining,
            Phase::Joined(_) => Observation::Joined,
            Phase::Taken => Observation::Taken,
            Phase::NotSubmitted(failure) => Observation::NotSubmitted(*failure),
        }
    }
}

/// Own the actual original input/claim/provider through their complete ordinary retirement.
/// Startup currentness and permanent process admission serialize under the same Control
/// scope/phase guards. Those guards are released before provider or Producer work.
/// # Errors
/// Only a violated once-recording contract escapes; ordinary failures are recorded intact.
fn run_prepared_with<F>(
    prepared: PreparedInvocation,
    admission: &ProcessAdmission,
    make_provider: for<'a> fn(ProcessClaim<'a>) -> Result<ProviderScope<'a>, Failure>,
    execute: F,
    facts: &Facts,
    hooks: Hooks,
) -> Result<(), Failure>
where
    F: for<'a> FnOnce(&'a Config, &'a ProviderScope<'a>, Control) -> Operation,
{
    #[cfg(test)]
    let mut hooks = hooks;
    #[cfg(not(test))]
    let Hooks {} = hooks;
    #[cfg(test)]
    if let Some(start) = hooks.before_start.take() {
        start();
    }
    let PreparedInvocation { config, control } = prepared;
    let admitted = (|| {
        let guard = control.guard()?;
        guard.remaining(IoPhase::Handshake, Instant::now())?;
        admission.claim()
    })();
    match admitted {
        Ok(claim) => {
            match make_provider(claim) {
                Ok(provider) => {
                    let operation = execute(&config, &provider, control);
                    facts.record(operation)?;
                    #[cfg(test)]
                    if let Some(retire) = hooks.before_retire.take() {
                        retire();
                    }
                    drop(provider);
                    #[cfg(test)]
                    if let Some(after_provider) = hooks.after_provider.take() {
                        after_provider();
                    }
                }
                Err(primary) => {
                    facts.record(Err(Failed::from(primary)))?;
                    #[cfg(test)]
                    if let Some(retire) = hooks.before_retire.take() {
                        retire();
                    }
                    drop(control);
                }
            }
            drop(config);
        }
        Err(primary) => {
            facts.record(Err(Failed::from(primary)))?;
            drop(control);
            drop(config);
        }
    }
    facts.retired.store(true, Ordering::Release);
    Ok(())
}

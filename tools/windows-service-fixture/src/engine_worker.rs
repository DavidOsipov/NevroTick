// SPDX-License-Identifier: GPL-2.0-only
//! One real managed loopback acquisition behind the fixture's private Worker contract.
//!
//! Preparation retains an actual dormant Engine session on the host thread. The
//! host owns service status, the admission fence and the work permit. The one
//! acquisition thread retains its original session, report and cooperative Stop
//! handle. Returning a report or waking the host cannot complete that permit.

use std::{
    mem,
    panic::AssertUnwindSafe,
    sync::{
        Arc, Mutex, MutexGuard, OnceLock,
        atomic::{AtomicBool, Ordering},
    },
    task::Poll,
    thread::{Builder, JoinHandle},
    time::Instant,
};

use nevrotick_engine::{
    AcquisitionError, AcquisitionReport, AcquisitionSession, AddressRoundConfig, AddressRoundEnd,
    AddressRoundReport, BoundedAcquisitionError, ConfiguredAddressRound, ControlHandle,
    ControlSchedule, PilotError, PreparedAcquisition, PreparedAddressRound, SeriesConfig,
    SeriesEnd, SourceConfigurationSnapshot, SourceHandle, execute_address_round,
};

use crate::{
    error::WorkerError,
    host::{Worker, WorkerCompletion, WorkerFailure, WorkerFinish, WorkerWake, guard_service_main},
};

/// An original Engine result, retained without reconstructing its errors or report.
type AcquisitionResult = Result<AcquisitionReport, BoundedAcquisitionError>;

/// One original address-round result, including failures before a report exists.
type AddressRoundResult = Result<AddressRoundReport, BoundedAcquisitionError>;

/// Exactly one producer can publish the invocation's retained result.
#[must_use]
enum WorkerResult {
    /// The unchanged fixed-peer series path.
    Series(AcquisitionResult),
    /// The configured, original-cutoff address-round path.
    AddressRound(AddressRoundResult),
}

/// Move one prepared owner into the common acquisition thread exactly once.
enum PreparedWork {
    /// Existing fixed-peer session, still dormant until Engine preparation.
    Series(Box<AcquisitionSession>),
    /// Checked source/revision plan, without entropy, runtime or socket work.
    AddressRound(Box<ConfiguredAddressRound>),
}

/// The exclusive start and join authorizations belong to this worker instance.
enum State {
    /// Checked inputs for one acquisition, before any session or thread starts.
    Dormant {
        /// The original logical source, with live policy custody.
        source: SourceHandle,
        /// Its selected installed revision, retained through startup.
        configuration: SourceConfigurationSnapshot,
        /// Checked peer and series limits.
        config: SeriesConfig,
        /// Existing one-shot control schedule.
        schedule: ControlSchedule,
    },
    /// Checked address-round inputs, before source binding or Engine work.
    DormantRound {
        /// Original source and selected installed revision.
        source: SourceHandle,
        /// No same-address replacement can silently change this startup selection.
        configuration: SourceConfigurationSnapshot,
        /// Ordered candidates and the existing finite request/round limits.
        config: AddressRoundConfig,
        /// Existing one-shot control schedule, without automatic retries.
        schedule: ControlSchedule,
    },
    /// The selected session or round plan is ready; the host has not activated it.
    Prepared(PreparedWork, ControlSchedule, WorkerWake),
    /// The only owned acquisition thread. Its return marker alone is insufficient.
    Active(JoinHandle<()>),
    /// This invocation has no remaining preparation, acquisition or join work.
    Retired,
}

/// One sticky Stop decision serialized with publication of the original handle.
#[derive(Default)]
struct ControlGate {
    /// Once set, the cancellation decision cannot be cleared.
    cancelled: bool,
    /// Sole independently usable handle from the actual Engine preparation.
    control: Option<ControlHandle>,
}

/// Retained operation facts and an independent level-triggered return marker.
#[derive(Default)]
struct Shared {
    /// Serializes the Stop decision and publication of the exact original handle.
    gate: Mutex<ControlGate>,
    /// The original preparation or acquisition result, assigned once if reached.
    result: OnceLock<WorkerResult>,
    /// Failure at a host-side boundary before the acquisition thread owns work.
    inactive_failure: OnceLock<WorkerError>,
    /// A result cannot be replaced; even a refused publication must wake the host.
    publication_failed: AtomicBool,
    /// The thread reached its return tail, without claiming an acknowledged join.
    returned: AtomicBool,
}

impl Shared {
    /// Latch Stop and signal the exact available handle, even when the gate is poisoned.
    fn stop(&self) {
        let mut gate: MutexGuard<'_, ControlGate> = self
            .gate
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        gate.cancelled = true;
        if let Some(control) = &gate.control {
            control.stop();
        }
    }

    /// Install the actual new control after applying any Stop latched before publication.
    /// Refuse a poisoned or duplicate publication, and only cancel in those cases.
    fn publish_control(&self, control: ControlHandle) -> bool {
        match self.gate.lock() {
            Ok(mut gate) if gate.control.is_none() => {
                if gate.cancelled {
                    control.stop();
                }
                gate.control = Some(control);
                true
            }
            Ok(mut gate) => {
                gate.cancelled = true;
                if let Some(original) = &gate.control {
                    original.stop();
                }
                control.stop();
                self.publication_failed.store(true, Ordering::Release);
                false
            }
            Err(poisoned) => {
                let mut gate: MutexGuard<'_, ControlGate> = poisoned.into_inner();
                gate.cancelled = true;
                control.stop();
                false
            }
        }
    }

    /// Resolve ownership of the original handle only after there is no thread to join.
    fn retire_control(&self) {
        let mut gate: MutexGuard<'_, ControlGate> = self
            .gate
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        gate.cancelled = true;
        gate.control = None;
    }

    /// Keep exactly the original result; an impossible second publication is a worker failure.
    fn publish_result(&self, result: AcquisitionResult) {
        self.publish(WorkerResult::Series(result));
    }

    /// Preserve the actual address-round result in the same sole publication slot.
    fn publish_round_result(&self, result: AddressRoundResult) {
        self.publish(WorkerResult::AddressRound(result));
    }

    /// A duplicate producer cannot overwrite either retained result kind.
    fn publish(&self, result: WorkerResult) {
        if self.result.set(result).is_err() {
            self.publication_failed.store(true, Ordering::Release);
        }
    }

    /// Borrow only the original fixed-peer result; never reinterpret a round report.
    fn series_result(&self) -> Option<&AcquisitionResult> {
        match self.result.get()? {
            WorkerResult::Series(result) => Some(result),
            WorkerResult::AddressRound(_) => None,
        }
    }

    /// Borrow only the original configured-round result after actual publication.
    fn round_result(&self) -> Option<&AddressRoundResult> {
        match self.result.get()? {
            WorkerResult::AddressRound(result) => Some(result),
            WorkerResult::Series(_) => None,
        }
    }

    /// Record the result of the attempted startup/activation boundary once.
    fn refuse_inactive(&self, error: WorkerError) {
        if self.inactive_failure.set(error).is_err() {
            self.publication_failed.store(true, Ordering::Release);
        }
    }

    /// Project fixed health/cleanup facts while retaining the complete original result.
    fn completion(&self) -> WorkerCompletion {
        let (operation_failed, cleanup_failed): (bool, bool) = match self.result.get() {
            None => (false, false),
            Some(WorkerResult::Series(Err(_)) | WorkerResult::AddressRound(Err(_))) => {
                (true, false)
            }
            Some(WorkerResult::Series(Ok(report))) => {
                let series: &nevrotick_engine::SeriesReport = report.series();
                (
                    series_operation_failed(series.collection_end()),
                    any_cleanup_failure(
                        series.stop_cleanup_failure().is_some(),
                        series.invalidate_cleanup_failure().is_some(),
                    ),
                )
            }
            Some(WorkerResult::AddressRound(Ok(report))) => (
                round_operation_failed(report.round_end()),
                report.cleanup_errors().iter().any(Option::is_some),
            ),
        };
        WorkerCompletion {
            operation_failure: self.inactive_failure.get().copied().or_else(|| {
                (operation_failed
                    || self.gate.is_poisoned()
                    || self.publication_failed.load(Ordering::Acquire))
                .then_some(WorkerError::Running)
            }),
            cleanup_failure: cleanup_failed.then_some(WorkerError::Cleanup),
        }
    }
}

/// Either independently observed cleanup failure prevents a clean worker outcome.
/// This boolean policy does not establish either actual Engine cleanup producer.
fn any_cleanup_failure(stop_failed: bool, invalidate_failed: bool) -> bool {
    stop_failed || invalidate_failed
}

/// Classify operation failure from the Engine's validated collection decision.
/// Engine finish already forces primary failures to Failure; independent cleanup
/// cannot erase Stale or introduce an additional operation failure.
/// It does not establish either actual Engine cleanup producer or report transfer.
fn series_operation_failed(end: SeriesEnd) -> bool {
    matches!(end, SeriesEnd::Stale | SeriesEnd::Failure)
}

/// Keep a valid round's ordinary bounded dispositions separate from actual failures.
/// Timer cleanup is classified independently from the original acquisition decision.
fn round_operation_failed(end: AddressRoundEnd) -> bool {
    matches!(end, AddressRoundEnd::Stale | AddressRoundEnd::Failure)
}

/// The test composition can pause at these boundaries without replacing an Engine result.
#[derive(Default)]
struct TestHooks {
    /// After real Engine prepare, immediately before its control is published.
    before_control: Option<Box<dyn FnOnce() + Send>>,
    /// After the result and return wake, while the original thread is still owned.
    after_return: Option<Box<dyn FnOnce() + Send>>,
}

/// One private worker for the explicit portable fixture composition.
pub(super) struct EngineWorker {
    /// The unique prepared session or join handle.
    state: State,
    /// Bounded state shared with the one acquisition thread.
    shared: Arc<Shared>,
    /// Optional fixture-only interleaving controls; never an alternate Engine.
    hooks: TestHooks,
}

impl EngineWorker {
    /// Retain one selected revision and its checked inputs for dormant startup.
    /// The startup poll checks that the same revision is still current.
    ///
    /// # Errors
    /// Preserve refusal by the Engine's checked control schedule constructor.
    pub(super) fn new(
        source: SourceHandle,
        configuration: SourceConfigurationSnapshot,
        config: SeriesConfig,
    ) -> Result<Self, PilotError> {
        let schedule: ControlSchedule = ControlSchedule::new(None, None)?;
        Ok(Self {
            state: State::Dormant {
                source,
                configuration,
                config,
                schedule,
            },
            shared: Arc::new(Shared::default()),
            hooks: TestHooks::default(),
        })
    }

    /// Retain one ordered managed round without source binding, entropy or I/O.
    /// Startup binds the exact selected revision before admitting Running.
    /// # Errors
    /// Preserve refusal by the existing checked control schedule constructor.
    pub(super) fn new_address_round(
        source: SourceHandle,
        configuration: SourceConfigurationSnapshot,
        config: AddressRoundConfig,
    ) -> Result<Self, PilotError> {
        let schedule: ControlSchedule = ControlSchedule::new(None, None)?;
        Ok(Self {
            state: State::DormantRound {
                source,
                configuration,
                config,
                schedule,
            },
            shared: Arc::new(Shared::default()),
            hooks: TestHooks::default(),
        })
    }
}

impl Worker for EngineWorker {
    /// Construct the real dormant session under the host's startup permit, without socket or runtime.
    fn poll_prepare(&mut self, wake: &WorkerWake) -> Poll<Result<(), WorkerError>> {
        let prior: State = mem::replace(&mut self.state, State::Retired);
        match prior {
            State::Dormant {
                source,
                configuration,
                config,
                schedule,
            } => match source.session_for_configuration(&configuration, config) {
                Ok(session) => {
                    self.state = State::Prepared(
                        PreparedWork::Series(Box::new(session)),
                        schedule,
                        wake.clone(),
                    );
                    Poll::Ready(Ok(()))
                }
                Err(error) => {
                    self.shared.refuse_inactive(WorkerError::Preparation);
                    self.shared
                        .publish_result(Err(BoundedAcquisitionError::Acquisition(
                            AcquisitionError::SourcePolicy(error),
                        )));
                    Poll::Ready(Err(WorkerError::Preparation))
                }
            },
            State::DormantRound {
                source,
                configuration,
                config,
                schedule,
            } => match source.address_round_for_configuration(&configuration, config) {
                Ok(plan) => {
                    self.state = State::Prepared(
                        PreparedWork::AddressRound(Box::new(plan)),
                        schedule,
                        wake.clone(),
                    );
                    Poll::Ready(Ok(()))
                }
                Err(error) => {
                    self.shared.refuse_inactive(WorkerError::Preparation);
                    self.shared
                        .publish_round_result(Err(BoundedAcquisitionError::Acquisition(
                            AcquisitionError::SourcePolicy(error),
                        )));
                    Poll::Ready(Err(WorkerError::Preparation))
                }
            },
            prepared @ State::Prepared(..) => {
                self.state = prepared;
                Poll::Ready(Ok(()))
            }
            other => {
                self.state = other;
                Poll::Ready(Err(WorkerError::Preparation))
            }
        }
    }

    /// Move the same session and acknowledged cutoff to its sole acquisition thread.
    ///
    /// # Errors
    /// Refuse an absent, cancelled or poisoned preparation, or a failed thread spawn.
    fn activate(&mut self, cutoff: Instant) -> Result<(), WorkerError> {
        let prior: State = mem::replace(&mut self.state, State::Retired);
        let (work, schedule, wake): (PreparedWork, ControlSchedule, WorkerWake) = match prior {
            State::Prepared(work, schedule, wake) => (work, schedule, wake),
            other => {
                self.state = other;
                return Err(WorkerError::Activation);
            }
        };
        if !self.shared.gate.lock().is_ok_and(|gate| !gate.cancelled) {
            self.shared.refuse_inactive(WorkerError::Activation);
            return Err(WorkerError::Activation);
        }

        let shared: Arc<Shared> = Arc::clone(&self.shared);
        let hooks: TestHooks = mem::take(&mut self.hooks);
        if let Ok(thread) = Builder::new()
            .spawn(move || guarded_acquisition(work, schedule, cutoff, &shared, &wake, hooks))
        {
            self.state = State::Active(thread);
            Ok(())
        } else {
            self.shared.refuse_inactive(WorkerError::Activation);
            Err(WorkerError::Activation)
        }
    }

    /// Read already-published failure health without consuming the retained result.
    ///
    /// # Errors
    /// Preserve an observed operation or cleanup failure as its separate kind.
    fn poll_running(&mut self, _wake: &WorkerWake) -> Result<(), WorkerFailure> {
        let completion: WorkerCompletion = self.shared.completion();
        if let Some(error) = completion.operation_failure {
            Err(WorkerFailure::Operation(error))
        } else if let Some(error) = completion.cleanup_failure {
            Err(WorkerFailure::Cleanup(error))
        } else {
            Ok(())
        }
    }

    /// Resolve dormant work or request Stop through the exact active Engine control.
    fn request_cancel(&mut self, _wake: &WorkerWake) {
        self.shared.stop();
        if !matches!(self.state, State::Active(_)) {
            self.state = State::Retired;
        }
    }

    /// Distinguish an active operation, its return tail and an acknowledged join.
    fn poll_finished(&mut self, _wake: &WorkerWake) -> WorkerFinish {
        let prior: State = mem::replace(&mut self.state, State::Retired);
        if let State::Active(thread) = prior {
            if !thread.is_finished() {
                self.state = State::Active(thread);
                return if self.shared.returned.load(Ordering::Acquire) {
                    WorkerFinish::JoinTailPending
                } else {
                    WorkerFinish::Pending
                };
            }
            if thread.join().is_err() {
                // The acquisition guard already treats unexpected unwinds as fatal.
                // A join panic outside that guard must never acknowledge Finished.
                std::process::abort();
            }
        }
        self.shared.retire_control();
        WorkerFinish::Finished(self.shared.completion())
    }
}

/// Run the owned thread and always publish return readiness on ordinary paths.
/// The guard aborts on unexpected unwind before a Finished acknowledgement is possible.
fn guarded_acquisition(
    work: PreparedWork,
    schedule: ControlSchedule,
    cutoff: Instant,
    shared: &Shared,
    wake: &WorkerWake,
    hooks: TestHooks,
) {
    // This boundary never resumes an unwound session, hook or poisoned result.
    guard_service_main(AssertUnwindSafe(move || {
        let TestHooks {
            before_control,
            after_return,
        } = hooks;
        match work {
            PreparedWork::Series(session) => {
                run_acquisition(*session, &schedule, cutoff, shared, before_control);
            }
            PreparedWork::AddressRound(plan) => {
                run_address_round(*plan, &schedule, cutoff, shared, before_control);
            }
        }
        shared.returned.store(true, Ordering::Release);
        wake.notify();
        if let Some(pause) = after_return {
            pause();
        }
    }));
}

/// Execute at most one acquisition under the original Running cutoff.
/// Retain the typed result. A poisoned control publication refuses execution.
fn run_acquisition(
    mut session: AcquisitionSession,
    schedule: &ControlSchedule,
    cutoff: Instant,
    shared: &Shared,
    before_control: Option<Box<dyn FnOnce() + Send>>,
) {
    let prepared: Result<(PreparedAcquisition<'_>, ControlHandle), BoundedAcquisitionError> =
        session.prepare_before(cutoff);
    let result: AcquisitionResult = match prepared {
        Ok((prepared, control)) => {
            if let Some(pause) = before_control {
                pause();
            }
            if !shared.publish_control(control) {
                return;
            }
            prepared
                .execute(schedule)
                .map_err(BoundedAcquisitionError::Acquisition)
        }
        Err(error) => Err(error),
    };
    shared.publish_result(result);
}

/// Consume the configured round using the original acknowledged service cutoff.
/// Publish no successful report for expired preparation or a typed Engine failure.
fn run_address_round(
    plan: ConfiguredAddressRound,
    schedule: &ControlSchedule,
    cutoff: Instant,
    shared: &Shared,
    before_control: Option<Box<dyn FnOnce() + Send>>,
) {
    let prepared: Result<Option<(PreparedAddressRound, ControlHandle)>, PilotError> =
        plan.prepare_before(cutoff);
    let result: AddressRoundResult = match prepared {
        Ok(Some((prepared, control))) => {
            if let Some(pause) = before_control {
                pause();
            }
            if !shared.publish_control(control) {
                return;
            }
            execute_address_round(prepared, schedule).map_err(|error| {
                BoundedAcquisitionError::Acquisition(AcquisitionError::Driver(error))
            })
        }
        Ok(None) => Err(BoundedAcquisitionError::OwnerDeadlineElapsed),
        Err(error) => Err(BoundedAcquisitionError::Acquisition(
            AcquisitionError::Driver(error),
        )),
    };
    shared.publish_round_result(result);
}

#[cfg(test)]
#[path = "engine_worker_tests.rs"]
mod tests;

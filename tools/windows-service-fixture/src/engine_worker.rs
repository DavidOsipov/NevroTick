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
};

use nevrotick_engine::{
    AcquisitionError, AcquisitionReport, AcquisitionSession, ControlHandle, ControlSchedule,
    PilotError, PreparedAcquisition, SeriesConfig, SeriesEnd, SourceHandle,
};

use crate::{
    error::WorkerError,
    host::{Worker, WorkerCompletion, WorkerFailure, WorkerFinish, WorkerWake, guard_service_main},
};

/// An original Engine result, retained without reconstructing its errors or report.
type AcquisitionResult = Result<AcquisitionReport, AcquisitionError>;

/// The exclusive start and join authorizations belong to this worker instance.
enum State {
    /// Checked source and series plan, with no thread or operation started.
    Dormant(SourceHandle, SeriesConfig, ControlSchedule),
    /// Actual session initialization completed; the host has not activated it.
    Prepared(AcquisitionSession, ControlSchedule, WorkerWake),
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
    result: OnceLock<AcquisitionResult>,
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
        if self.result.set(result).is_err() {
            self.publication_failed.store(true, Ordering::Release);
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
        let result: Option<&AcquisitionResult> = self.result.get();
        let cleanup_failed: bool = result.is_some_and(|result| {
            result.as_ref().is_ok_and(|report| {
                let stop_failed: bool = report.series().stop_cleanup_failure().is_some();
                let invalidate_failed: bool =
                    report.series().invalidate_cleanup_failure().is_some();
                any_cleanup_failure(stop_failed, invalidate_failed)
            })
        });
        let operation_failed: bool = result.is_some_and(|result| match result {
            Err(_) => true,
            Ok(report) => {
                let series: &nevrotick_engine::SeriesReport = report.series();
                series_operation_failed(
                    series.end(),
                    series.primary_failure().is_some(),
                    cleanup_failed,
                )
            }
        });
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

/// Classify operation failure from the original series end and recorded cause presence.
/// Every enum/boolean combination is accepted; this total, effect-free policy
/// preserves primary and stale failures without duplicating a cleanup-only cause.
/// It does not establish either actual Engine cleanup producer or report transfer.
fn series_operation_failed(end: SeriesEnd, primary_failed: bool, cleanup_failed: bool) -> bool {
    primary_failed || end == SeriesEnd::Stale || (end == SeriesEnd::Failure && !cleanup_failed)
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
    /// Retain admitted, effect-free inputs for one dormant worker.
    ///
    /// # Errors
    /// Preserve refusal by the Engine's checked control schedule constructor.
    pub(super) fn new(source: SourceHandle, config: SeriesConfig) -> Result<Self, PilotError> {
        let schedule: ControlSchedule = ControlSchedule::new(None, None)?;
        Ok(Self {
            state: State::Dormant(source, config, schedule),
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
            State::Dormant(source, config, schedule) => match source.session(config) {
                Ok(session) => {
                    self.state = State::Prepared(session, schedule, wake.clone());
                    Poll::Ready(Ok(()))
                }
                Err(error) => {
                    self.shared.refuse_inactive(WorkerError::Preparation);
                    self.shared
                        .publish_result(Err(AcquisitionError::SourcePolicy(error)));
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

    /// Move the same prepared session once to its original Engine acquisition thread.
    ///
    /// # Errors
    /// Refuse an absent, cancelled or poisoned preparation, or a failed thread spawn.
    fn activate(&mut self) -> Result<(), WorkerError> {
        let prior: State = mem::replace(&mut self.state, State::Retired);
        let (session, schedule, wake): (AcquisitionSession, ControlSchedule, WorkerWake) =
            match prior {
                State::Prepared(session, schedule, wake) => (session, schedule, wake),
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
            .spawn(move || guarded_acquisition(session, schedule, &shared, &wake, hooks))
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
    session: AcquisitionSession,
    schedule: ControlSchedule,
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
        run_acquisition(session, &schedule, shared, before_control);
        shared.returned.store(true, Ordering::Release);
        wake.notify();
        if let Some(pause) = after_return {
            pause();
        }
    }));
}

/// Execute at most one prepared operation and retain its exact typed result.
/// A poisoned control publication independently refuses execution.
fn run_acquisition(
    mut session: AcquisitionSession,
    schedule: &ControlSchedule,
    shared: &Shared,
    before_control: Option<Box<dyn FnOnce() + Send>>,
) {
    let prepared: Result<(PreparedAcquisition<'_>, ControlHandle), AcquisitionError> =
        session.prepare();
    let result: AcquisitionResult = match prepared {
        Ok((prepared, control)) => {
            if let Some(pause) = before_control {
                pause();
            }
            if !shared.publish_control(control) {
                return;
            }
            prepared.execute(schedule)
        }
        Err(error) => Err(error),
    };
    shared.publish_result(result);
}

#[cfg(test)]
#[path = "engine_worker_tests.rs"]
mod tests;

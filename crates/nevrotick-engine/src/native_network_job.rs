// SPDX-License-Identifier: GPL-2.0-only
//! Process custody shared by the blocking resolver and controlled TCP connector.
//!
//! This lane admits one worker and retains its thread through actual join. The
//! separately admitted KE worker keeps the other process slot.

use std::{
    io,
    sync::{
        Arc, Mutex, MutexGuard, TryLockError,
        atomic::{AtomicU8, Ordering},
    },
    task::Poll,
    thread::{self, JoinHandle},
};
#[cfg(feature = "roughtime-verification")]
use std::{sync::OnceLock, time::Instant};
use tokio::sync::oneshot::{Receiver, Sender, channel};

/// Worker has yet to receive native dispatch admission.
const RESERVED: u8 = 0;
/// Worker won its one native dispatch admission.
const DISPATCHED: u8 = 1;
/// The original authority revoked dispatch before a worker claimed it.
const CANCELLED: u8 = 2;
/// Receiver closed and the original owner relinquished the result.
const RETIRED: u8 = 3;
/// The process owner observed actual worker join.
const JOINED: u8 = 4;

/// Failure of worker admission or the original typed handoff.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum NativeJobError {
    /// The shared lane is occupied or its owner is busy settling it.
    Busy,
    /// The shared lane, thread spawn, join or typed handoff failed.
    Failed,
}

/// Exact dispatch and completion state shared with one admitted invocation.
struct JobState {
    /// Monotonic lifecycle of this invocation's native worker.
    phase: AtomicU8,
    /// Time captured by the worker that won native dispatch admission.
    #[cfg(feature = "roughtime-verification")]
    submitted_at: OnceLock<Instant>,
    /// Observed once the same worker actually joined.
    #[cfg(feature = "roughtime-verification")]
    settlement: OnceLock<Settlement>,
}

impl JobState {
    /// Revoke a still-reserved dispatch while the typed result remains owned.
    fn cancel_dispatch(&self) {
        let _previous: Result<u8, u8> =
            self.phase
                .compare_exchange(RESERVED, CANCELLED, Ordering::AcqRel, Ordering::Acquire);
    }
}

/// Actual thread custody and identity of the original invocation.
struct Submission {
    /// Exact state shared with the typed invocation and dispatch admission.
    state: Arc<JobState>,
    /// Held until observed exit and actual join, including after cancellation.
    thread: JoinHandle<()>,
}

/// One process slot shared between DNS and TCP with no submission backlog.
static SUBMISSION: Mutex<Option<Submission>> = Mutex::new(None);

/// Serialize the two libtest fixtures that exercise the same process slot.
#[cfg(all(test, feature = "roughtime-verification"))]
pub(crate) static CUSTODY_FIXTURE: Mutex<()> = Mutex::new(());

/// One dispatch opportunity for the original process reservation.
pub(crate) struct DispatchAdmission {
    /// State belonging to this same worker through retirement.
    state: Arc<JobState>,
}

impl DispatchAdmission {
    /// Permit the first dispatch while the original reservation remains live.
    #[cfg(any(windows, test))]
    pub(crate) fn admit(&self) -> bool {
        self.claim()
    }

    /// Permit the first native dispatch and retain its worker clock sample.
    #[cfg(feature = "roughtime-verification")]
    pub(crate) fn admit_at(&self, now: Instant) -> bool {
        if !self.claim() {
            return false;
        }
        let _first: Result<(), Instant> = self.state.submitted_at.set(now);
        true
    }

    /// Report whether the original owner has retired this active worker.
    #[cfg(any(windows, test))]
    pub(crate) fn retirement_requested(&self) -> bool {
        self.state.phase.load(Ordering::Acquire) == RETIRED
    }

    /// Serialize the one dispatch with withdrawal or owner retirement.
    fn claim(&self) -> bool {
        self.state
            .phase
            .compare_exchange(RESERVED, DISPATCHED, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }
}

/// Observed completion of one exact worker.
#[cfg(feature = "roughtime-verification")]
#[derive(Clone, Copy)]
pub(crate) struct Settlement {
    /// A clock sample from the native entry admission, when it was charged.
    pub(crate) submitted_at: Option<Instant>,
    /// Clock sample taken after joining the actual worker.
    pub(crate) joined_at: Instant,
}

/// Original dispatch revocation and observation without typed result ownership.
#[cfg(feature = "roughtime-verification")]
#[derive(Clone)]
pub(crate) struct JoinWitness {
    /// Identity of the single worker this witness may observe.
    state: Arc<JobState>,
}

#[cfg(feature = "roughtime-verification")]
impl JoinWitness {
    /// Revoke a still-reserved dispatch without taking the original receiver.
    pub(crate) fn cancel_dispatch(&self) {
        self.state.cancel_dispatch();
    }

    /// Observe actual join, reaping only a retired worker with this identity.
    pub(crate) fn settled(&self) -> Option<Settlement> {
        if self.state.settlement.get().is_none() {
            reap_exact_retired(&self.state);
        }
        self.state.settlement.get().copied()
    }
}

/// Exclusive typed receiver owned by the original native invocation.
#[must_use = "keep the native job until join or explicitly retire its result"]
pub(crate) struct NativeJob<T> {
    /// Exact identity used to collect or retire this one worker.
    state: Arc<JobState>,
    /// Closed before a retired worker can release the shared lane.
    receiver: Option<Receiver<T>>,
}

impl<T: Send + 'static> NativeJob<T> {
    /// Reserve the shared process slot and start one worker with one result.
    ///
    /// # Errors
    /// Refuses busy or unavailable custody, an invalid thread name, or spawn failure.
    pub(crate) fn start(
        thread_name: &str,
        operation: impl FnOnce(DispatchAdmission) -> T + Send + 'static,
    ) -> Result<Self, NativeJobError> {
        if thread_name.as_bytes().contains(&0) {
            return Err(NativeJobError::Failed);
        }
        let mut slot: MutexGuard<'_, Option<Submission>> = match SUBMISSION.try_lock() {
            Ok(slot) => slot,
            Err(TryLockError::WouldBlock) => return Err(NativeJobError::Busy),
            Err(TryLockError::Poisoned(_)) => return Err(NativeJobError::Failed),
        };
        let discarded: Option<thread::Result<()>> = reap_retired(&mut slot);
        if slot.is_some() {
            return Err(NativeJobError::Busy);
        }
        let state: Arc<JobState> = Arc::new(JobState {
            phase: AtomicU8::new(RESERVED),
            #[cfg(feature = "roughtime-verification")]
            submitted_at: OnceLock::new(),
            #[cfg(feature = "roughtime-verification")]
            settlement: OnceLock::new(),
        });
        let admission: DispatchAdmission = DispatchAdmission {
            state: Arc::clone(&state),
        };
        let (sender, receiver): (Sender<T>, Receiver<T>) = channel();
        let spawned: io::Result<JoinHandle<()>> = thread::Builder::new()
            .name(String::from(thread_name))
            .spawn(move || drop(sender.send(operation(admission))));
        let result: Result<Self, NativeJobError> = spawned
            .map(|thread| {
                *slot = Some(Submission {
                    state: Arc::clone(&state),
                    thread,
                });
                Self {
                    state,
                    receiver: Some(receiver),
                }
            })
            .map_err(|_| NativeJobError::Failed);
        drop(slot);
        drop(discarded);
        result
    }

    /// Obtain settlement identity independently of the typed result receiver.
    #[cfg(feature = "roughtime-verification")]
    pub(crate) fn witness(&self) -> JoinWitness {
        JoinWitness {
            state: Arc::clone(&self.state),
        }
    }

    /// Consume the result only after observed worker exit and actual join.
    ///
    /// # Errors
    /// Reports failed custody, join or result handoff.
    pub(crate) fn try_join(&mut self) -> Poll<Result<T, NativeJobError>> {
        let mut slot: MutexGuard<'_, Option<Submission>> = match SUBMISSION.try_lock() {
            Ok(slot) => slot,
            Err(TryLockError::WouldBlock) => return Poll::Pending,
            Err(TryLockError::Poisoned(_)) => return Poll::Ready(Err(NativeJobError::Failed)),
        };
        if !owns(slot.as_ref(), &self.state) {
            return Poll::Ready(Err(NativeJobError::Failed));
        }
        let joined: Option<thread::Result<()>> = join_finished(&mut slot);
        drop(slot);
        match joined {
            Some(Ok(())) => Poll::Ready(
                self.receiver
                    .take()
                    .and_then(|mut receiver| receiver.try_recv().ok())
                    .ok_or(NativeJobError::Failed),
            ),
            Some(Err(_)) => {
                drop(self.receiver.take());
                Poll::Ready(Err(NativeJobError::Failed))
            }
            None => Poll::Pending,
        }
    }

    /// Discard the original result and report whether worker custody remains.
    #[cfg(any(windows, test))]
    pub(crate) fn retire(mut self) -> bool {
        self.mark_retired();
        reap_exact_retired(&self.state);
        self.state.phase.load(Ordering::Acquire) != JOINED
    }
}

impl<T> NativeJob<T> {
    /// Close the typed handoff before making the retired thread available to reap.
    fn mark_retired(&mut self) {
        self.state.cancel_dispatch();
        drop(self.receiver.take());
        let _previous: u8 = self.state.phase.fetch_max(RETIRED, Ordering::AcqRel);
    }
}

impl<T> Drop for NativeJob<T> {
    fn drop(&mut self) {
        self.mark_retired();
    }
}

/// Require the exact invocation so an observer cannot reap a successor.
fn owns(slot: Option<&Submission>, expected: &Arc<JobState>) -> bool {
    slot.is_some_and(|owner| Arc::ptr_eq(&owner.state, expected))
}

/// Attempt a nonblocking reap restricted to the exact retired invocation.
fn reap_exact_retired(expected: &Arc<JobState>) {
    let reaped: Option<thread::Result<()>> = if let Ok(mut slot) = SUBMISSION.try_lock()
        && owns(slot.as_ref(), expected)
    {
        reap_retired(&mut slot)
    } else {
        None
    };
    drop(reaped);
}

/// Allow any admission to reap a worker whose original receiver closed.
fn reap_retired(slot: &mut Option<Submission>) -> Option<thread::Result<()>> {
    if slot
        .as_ref()
        .is_none_or(|owner| owner.state.phase.load(Ordering::Acquire) != RETIRED)
    {
        return None;
    }
    join_finished(slot)
}

/// Keep the process lane occupied until the retained actual thread joins.
fn join_finished(slot: &mut Option<Submission>) -> Option<thread::Result<()>> {
    if !slot
        .as_ref()
        .is_some_and(|owner| thread_exited(&owner.thread))
    {
        return None;
    }
    let owner: Submission = slot.take()?;
    let joined: thread::Result<()> = owner.thread.join();
    #[cfg(feature = "roughtime-verification")]
    {
        let _first: Result<(), Settlement> = owner.state.settlement.set(Settlement {
            submitted_at: owner.state.submitted_at.get().copied(),
            joined_at: Instant::now(),
        });
    }
    owner.state.phase.store(JOINED, Ordering::Release);
    Some(joined)
}

/// Check Windows thread exit, including teardown after the function returns.
#[cfg(windows)]
fn thread_exited(thread: &JoinHandle<()>) -> bool {
    windows_platform_native::dns_worker_thread_exited(thread)
}

/// Observe completion of the portable worker before joining.
#[cfg(not(windows))]
fn thread_exited(thread: &JoinHandle<()>) -> bool {
    thread.is_finished()
}

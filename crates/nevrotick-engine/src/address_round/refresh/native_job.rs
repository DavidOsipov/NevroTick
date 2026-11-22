// SPDX-License-Identifier: GPL-2.0-only
//! Process custody of one actual blocking resolver thread.

use std::{
    sync::{
        Arc, Mutex, MutexGuard, TryLockError,
        atomic::{AtomicU8, Ordering},
    },
    task::Poll,
    thread::{self, JoinHandle},
    time::Duration,
};

use super::{AddressRoundRefreshOutcome, RefreshPlan};
use crate::PilotError;

/// Worker has not yet been authorized to enter the blocking adapter.
const RESERVED: u8 = 0;
/// Worker was authorized to enter the blocking adapter once.
const DISPATCHED: u8 = 1;
/// The invocation retired its reservation or stopped waiting.
const RETIRED: u8 = 2;
/// The process owner joined the actual worker.
const JOINED: u8 = 3;
/// Timer interval for checking actual completion without blocking the Engine.
const COMPLETION_CHECK: Duration = Duration::from_millis(5);

/// One outcome produced by the selected resolver operation.
type Answer = Result<RefreshPlan, AddressRoundRefreshOutcome>;

/// Actual thread custody and identity of its original invocation.
struct Submission {
    /// Shared only with the invocation and its one dispatch admission.
    state: Arc<AtomicU8>,
    /// Kept here until the process owner can join it.
    thread: JoinHandle<Answer>,
}

/// The sole actual blocking resolver slot, including retired threads.
static SUBMISSION: Mutex<Option<Submission>> = Mutex::new(None);

/// One dispatch decision belonging to the original invocation's reservation.
pub(super) struct DispatchAdmission {
    /// Monotonic state of the same submission whose thread remains in custody.
    state: Arc<AtomicU8>,
}

impl DispatchAdmission {
    /// Only the first dispatch admitted before retirement can enter the adapter.
    pub(super) fn admit(self) -> bool {
        self.state
            .compare_exchange(RESERVED, DISPATCHED, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }
}

/// Invocation-side ownership of one actual blocking resolver thread.
pub(super) struct NativeResolverJob {
    /// Identifies the submission and prevents dispatch after retirement.
    state: Arc<AtomicU8>,
}

impl NativeResolverJob {
    /// Reserve the process slot and hand it the actual thread.
    ///
    /// # Errors
    /// Report occupied custody or a failure to start the owned thread.
    pub(super) fn start(
        operation: impl FnOnce(DispatchAdmission) -> Answer + Send + 'static,
    ) -> Result<Self, PilotError> {
        let mut slot: MutexGuard<'_, Option<Submission>> = match SUBMISSION.try_lock() {
            Ok(slot) => slot,
            Err(TryLockError::WouldBlock) => return Err(PilotError::ResolutionBusy),
            Err(TryLockError::Poisoned(_)) => return Err(PilotError::ResolverTask),
        };
        let discarded: Option<thread::Result<Answer>> = slot
            .as_ref()
            .is_some_and(|owner| owner.state.load(Ordering::Acquire) == RETIRED)
            .then(|| join_finished(&mut slot))
            .flatten();
        if slot.is_some() {
            return Err(PilotError::ResolutionBusy);
        }
        let state: Arc<AtomicU8> = Arc::new(AtomicU8::new(RESERVED));
        let admission: DispatchAdmission = DispatchAdmission {
            state: Arc::clone(&state),
        };
        let thread: JoinHandle<Answer> = thread::Builder::new()
            .name(String::from("nevrotick-dns"))
            .spawn(move || operation(admission))
            .map_err(|_| PilotError::ResolverTask)?;
        *slot = Some(Submission {
            state: Arc::clone(&state),
            thread,
        });
        let job: Self = Self { state };
        drop(slot);
        drop(discarded);
        Ok(job)
    }

    /// Await and consume the result from this actual thread. Dropping the
    /// wait future leaves custody with the job until its owner retires it.
    ///
    /// # Errors
    /// Return the resolver disposition, or report failed or absent custody.
    pub(super) async fn wait(&mut self) -> Answer {
        loop {
            if let Poll::Ready(outcome) = self.try_collect() {
                return outcome;
            }
            tokio::time::sleep(COMPLETION_CHECK).await;
        }
    }

    /// Retire this invocation and report whether actual thread custody remains.
    pub(super) fn retire(self) -> bool {
        self.mark_retired();
        if let Ok(mut slot) = SUBMISSION.try_lock()
            && self.owns(slot.as_ref())
        {
            let discarded: Option<thread::Result<Answer>> = join_finished(&mut slot);
            drop(slot);
            drop(discarded);
        }
        self.state.load(Ordering::Acquire) != JOINED
    }

    /// Attempt to transfer only this invocation's actually joined result.
    fn try_collect(&self) -> Poll<Answer> {
        let mut slot: MutexGuard<'_, Option<Submission>> = match SUBMISSION.try_lock() {
            Ok(slot) => slot,
            Err(TryLockError::WouldBlock) => return Poll::Pending,
            Err(TryLockError::Poisoned(_)) => return Poll::Ready(Err(resolver_task())),
        };
        if !self.owns(slot.as_ref()) {
            return Poll::Ready(Err(resolver_task()));
        }
        let joined: Option<thread::Result<Answer>> = join_finished(&mut slot);
        drop(slot);
        match joined {
            Some(Ok(outcome)) => Poll::Ready(outcome),
            Some(Err(_)) => Poll::Ready(Err(resolver_task())),
            None => Poll::Pending,
        }
    }

    /// Match actual invocation identity even if a later thread occupies the slot.
    fn owns(&self, slot: Option<&Submission>) -> bool {
        slot.is_some_and(|owner| Arc::ptr_eq(&self.state, &owner.state))
    }

    /// Close dispatch admission and preserve the latched join state, if any.
    fn mark_retired(&self) {
        let _previous: u8 = self.state.fetch_max(RETIRED, Ordering::AcqRel);
    }
}

impl Drop for NativeResolverJob {
    fn drop(&mut self) {
        self.mark_retired();
    }
}

/// Remove custody only after confirmed exit and join. The slot remains locked
/// through the join and the joined result is transferred once.
fn join_finished(slot: &mut Option<Submission>) -> Option<thread::Result<Answer>> {
    if !slot
        .as_ref()
        .is_some_and(|owner| thread_exited(&owner.thread))
    {
        return None;
    }
    let owner: Submission = slot.take()?;
    let joined: thread::Result<Answer> = owner.thread.join();
    owner.state.store(JOINED, Ordering::Release);
    Some(joined)
}

/// Require Windows thread exit, including teardown after the function returns.
#[cfg(windows)]
fn thread_exited(thread: &JoinHandle<Answer>) -> bool {
    windows_platform_native::dns_worker_thread_exited(thread)
}

/// Observe the idle test worker using the portable Rust completion indication.
#[cfg(not(windows))]
fn thread_exited(thread: &JoinHandle<Answer>) -> bool {
    thread.is_finished()
}

/// Report a custody failure without exposing the worker's private panic payload.
fn resolver_task() -> AddressRoundRefreshOutcome {
    AddressRoundRefreshOutcome::Failed(PilotError::ResolverTask)
}

#[cfg(test)]
mod tests {
    use std::{
        error::Error,
        net::SocketAddr,
        sync::mpsc::{self, Receiver, Sender},
    };

    use super::super::{AddressRoundConfig, series_packets};
    use super::{
        AddressRoundRefreshOutcome, COMPLETION_CHECK, Duration, NativeResolverJob, PilotError,
        RefreshPlan, resolver_task,
    };

    /// Finite watchdog for both coordinator-side and worker-side waits.
    const WATCHDOG: Duration = Duration::from_secs(5);

    /// Prepare a checked result whose peer identifies the actual producing thread.
    /// # Errors
    /// Return a configuration or fixture-packet refusal.
    fn answer(peer: SocketAddr) -> Result<RefreshPlan, Box<dyn Error>> {
        let config: AddressRoundConfig =
            AddressRoundConfig::new(&[peer], Duration::from_millis(1), Duration::from_secs(1))?;
        let entropy: [u8; 24] = [
            1, 0, 0, 0, 0, 0, 0, 1, 2, 0, 0, 0, 0, 0, 0, 2, 3, 0, 0, 0, 0, 0, 0, 3,
        ];
        let packets = series_packets(&entropy)?;
        Ok(RefreshPlan { config, packets })
    }

    /// Reap a retired thread through normal admission and hold the next admitted dispatch.
    /// # Errors
    /// Return a fixture preparation or worker start failure.
    async fn start_successor(
        peer: SocketAddr,
    ) -> Result<(NativeResolverJob, Sender<()>, Receiver<bool>), Box<dyn Error>> {
        loop {
            let prepared: RefreshPlan = answer(peer)?;
            let (dispatched, observed): (Sender<bool>, Receiver<bool>) = mpsc::channel();
            let (release, released): (Sender<()>, Receiver<()>) = mpsc::channel();
            match NativeResolverJob::start(move |admission| {
                let admitted: bool = admission.admit();
                if dispatched.send(admitted).is_err()
                    || !admitted
                    || released.recv_timeout(WATCHDOG).is_err()
                {
                    return Err(resolver_task());
                }
                Ok(prepared)
            }) {
                Ok(job) => return Ok((job, release, observed)),
                Err(PilotError::ResolutionBusy) => tokio::time::sleep(COMPLETION_CHECK).await,
                Err(error) => return Err(error.into()),
            }
        }
    }

    /// Exercise the one process slot serially with actual blocking threads.
    ///
    /// # Errors
    /// Return failed fixture preparation or a bounded thread/watchdog failure.
    /// # Panics
    /// Detect invalid dispatch, premature slot release, lost or reused results.
    #[test]
    fn retirement_closes_dispatch_and_preserves_actual_thread_custody() -> Result<(), Box<dyn Error>>
    {
        let old_peer: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 41001));
        let current_peer: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 41002));
        let old_answer: RefreshPlan = answer(old_peer)?;
        let (entered, started): (Sender<()>, Receiver<()>) = mpsc::channel();
        let (release, released): (Sender<()>, Receiver<()>) = mpsc::channel();
        let (dispatched, observed): (Sender<bool>, Receiver<bool>) = mpsc::channel();
        let original: NativeResolverJob = NativeResolverJob::start(move |admission| {
            if entered.send(()).is_err() || released.recv_timeout(WATCHDOG).is_err() {
                return Err(resolver_task());
            }
            let _reported = dispatched.send(admission.admit());
            // Offer a late answer anyway to ensure successor custody discards it.
            Ok(old_answer)
        })?;
        started.recv_timeout(WATCHDOG)?;
        assert!(original.retire());
        assert!(matches!(
            NativeResolverJob::start(|_| Err(resolver_task())),
            Err(PilotError::ResolutionBusy)
        ));
        release.send(())?;
        assert!(!observed.recv_timeout(WATCHDOG)?);

        let runtime: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .build()?;
        let (mut current, release_current, observed_current) = runtime.block_on(async {
            tokio::time::timeout(WATCHDOG, start_successor(current_peer)).await?
        })?;
        assert!(observed_current.recv_timeout(WATCHDOG)?);
        assert!(
            runtime
                .block_on(async { tokio::time::timeout(Duration::ZERO, current.wait()).await })
                .is_err()
        );
        assert!(matches!(
            NativeResolverJob::start(|_| Err(resolver_task())),
            Err(PilotError::ResolutionBusy)
        ));
        release_current.send(())?;
        let joined =
            runtime.block_on(async { tokio::time::timeout(WATCHDOG, current.wait()).await })?;
        let Ok(joined) = joined else {
            return Err(PilotError::ResolverTask.into());
        };
        assert_eq!(joined.config.peers, [current_peer]);
        assert!(matches!(
            runtime.block_on(async { tokio::time::timeout(WATCHDOG, current.wait()).await })?,
            Err(AddressRoundRefreshOutcome::Failed(PilotError::ResolverTask))
        ));
        assert!(!current.retire());
        Ok(())
    }
}

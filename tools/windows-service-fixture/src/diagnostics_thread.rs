// SPDX-License-Identifier: GPL-2.0-only
//! Portable fixture writer with bounded admission and a physically joined owner.

use std::{
    sync::{
        Arc, OnceLock,
        atomic::{AtomicBool, AtomicU8, Ordering},
        mpsc::{Receiver, SyncSender, TryRecvError, sync_channel},
    },
    thread::{self, JoinHandle},
    time::Instant,
};

use windows_platform::diagnostics::WriterRecord;

use super::{CloseMode, ClosePending, EventWriter, WriterClose};
use crate::host::WorkerWake;

/// The host still accepts offers and the writer can start a record.
const OPEN: u8 = 0;
/// The host closed admission before attempting a status call.
const FENCED: u8 = 1;
/// The host gave the writer one original drain deadline.
const DRAINING: u8 = 2;
/// The writer can resolve only effects already in custody.
const RETIRED: u8 = 3;

/// A dormant portable backend. Only its owned writer thread performs effects.
/// Dormant destruction has no external effects. All backend resource cleanup
/// occurs in its closure, including on unwind. The portable backend and its
/// hook exclude blocking TLS teardown and custom panic-payload destructors.
pub(crate) trait WriterBackend: Send + 'static {
    /// Consume one admitted record. False observes local loss.
    fn write(&mut self, record: WriterRecord) -> bool;

    /// Release all local resources before returning. False observes local loss.
    /// Backend resources must also resolve when the thread unwinds.
    fn close(self) -> bool;
}

/// A fixture barrier after the final marker and before physical exit.
type ExitTail = Box<dyn FnOnce() + Send>;

/// A single writer with one queued record and at most one writer-owned record.
pub(crate) struct ThreadedWriter<B> {
    /// One unstarted backend and its prospective exit observation.
    dormant: Option<(B, ExitTail)>,
    /// The same thread stays owned until its actual completion is joined.
    job: Option<Job>,
}

/// All host-side controls for the single original thread.
struct Job {
    /// One bounded slot of accepted work.
    sender: SyncSender<WriterRecord>,
    /// Monotonic gate and return observation shared with this thread.
    shared: Arc<Shared>,
    /// The only completion owner of the original job.
    thread: JoinHandle<WriterClose>,
}

/// Control values observed without a host lock across backend effects.
struct Shared {
    /// Only the host changes mode. The worker makes same-mode start claims.
    mode: AtomicU8,
    /// The original deadline is published before entering drain mode.
    deadline: OnceLock<Instant>,
    /// The worker has finished its effect and cleanup path and is returning.
    returning: AtomicBool,
}

/// The return marker permits a join observation after the last wake.
/// Drop publishes it on unwind, after the inner worker resources were dropped.
struct ReturnMarker {
    /// The marker belongs to the same thread and invocation.
    shared: Arc<Shared>,
    /// Publish to the original service owner and retain its wake token.
    wake: WorkerWake,
}

impl ReturnMarker {
    /// Publish exactly once, before waking the owner.
    fn publish(&self) {
        if !self.shared.returning.swap(true, Ordering::AcqRel) {
            self.wake.notify();
        }
    }
}

impl Drop for ReturnMarker {
    /// Preserve the final wake on unwind and across the closure return.
    fn drop(&mut self) {
        self.publish();
    }
}

impl<B: WriterBackend> ThreadedWriter<B> {
    /// Defer spawning until the host offers its first selected live record.
    /// The tail holds no backend resources and runs after the final notification.
    /// A backend and tail used here must satisfy the portable cleanup contract.
    pub(crate) fn with_exit_tail(backend: B, tail: impl FnOnce() + Send + 'static) -> Self {
        Self {
            dormant: Some((backend, Box::new(tail))),
            job: None,
        }
    }

    /// Start at most one worker. Spawn refusal consumes the dormant adapter.
    fn start(&mut self, wake: &WorkerWake) {
        let Some((backend, tail)) = self.dormant.take() else {
            return;
        };
        let (sender, receiver): (SyncSender<WriterRecord>, Receiver<WriterRecord>) =
            sync_channel(1);
        let shared: Arc<Shared> = Arc::new(Shared {
            mode: AtomicU8::new(OPEN),
            deadline: OnceLock::new(),
            returning: AtomicBool::new(false),
        });
        let marker: ReturnMarker = ReturnMarker {
            shared: Arc::clone(&shared),
            wake: wake.clone(),
        };
        let worker: std::io::Result<JoinHandle<WriterClose>> =
            thread::Builder::new().spawn(move || {
                let outcome: WriterClose = run(backend, &marker.shared, receiver);
                marker.publish();
                tail();
                outcome
            });
        if let Ok(thread) = worker {
            self.job = Some(Job {
                sender,
                shared,
                thread,
            });
        }
    }
}

impl<B: WriterBackend> EventWriter for ThreadedWriter<B> {
    /// Use the original service thread as the sole producer. A full queue refuses.
    fn offer(&mut self, record: WriterRecord, wake: &WorkerWake) -> bool {
        if self.job.is_none() {
            self.start(wake);
        }
        self.job.as_ref().is_some_and(|job: &Job| {
            if job.shared.mode.load(Ordering::Acquire) == OPEN
                && job.sender.try_send(record).is_ok()
            {
                job.thread.thread().unpark();
                return true;
            }
            false
        })
    }

    /// Publish the pause before the host can enter `StopPending` reporting.
    fn fence(&mut self) {
        self.dormant = None;
        if let Some(job) = &self.job
            && job.shared.mode.load(Ordering::Acquire) == OPEN
        {
            job.shared.mode.store(FENCED, Ordering::Release);
            job.thread.thread().unpark();
        }
    }

    /// The first drain deadline binds all later polls. Retire is sticky.
    /// A terminal marker alone leaves the join handle with the service owner.
    fn poll_close(
        &mut self,
        mode: CloseMode,
        _wake: &WorkerWake,
    ) -> Result<WriterClose, ClosePending> {
        self.fence();
        let Some(job) = self.job.take() else {
            return Ok(WriterClose::Drained);
        };
        match mode {
            CloseMode::Drain(deadline) => {
                if job.shared.mode.load(Ordering::Acquire) == FENCED {
                    let _deadline: &Instant = job.shared.deadline.get_or_init(|| deadline);
                    job.shared.mode.store(DRAINING, Ordering::Release);
                }
            }
            CloseMode::Retire => job.shared.mode.store(RETIRED, Ordering::Release),
        }
        job.thread.thread().unpark();
        if !job.thread.is_finished() {
            let pending: ClosePending = if job.shared.returning.load(Ordering::Acquire) {
                ClosePending::JoinTail
            } else {
                ClosePending::Wake
            };
            self.job = Some(job);
            return Err(pending);
        }
        // Only joining consumes the original handle. The selected portable
        // backend keeps blocking work within the observed closure.
        Ok(job.thread.join().unwrap_or(WriterClose::Loss))
    }
}

/// Claim a logical start against the same atomic gate used by the host.
/// Recheck time and mode at the portable effect boundary. A native sink may
/// need its own admission check at the actual submission boundary.
fn start_write(shared: &Shared, mode: u8) -> bool {
    shared
        .mode
        .compare_exchange(mode, mode, Ordering::AcqRel, Ordering::Acquire)
        .is_ok()
        && (mode == OPEN
            || mode == DRAINING
                && shared
                    .deadline
                    .get()
                    .is_some_and(|deadline: &Instant| Instant::now() < *deadline))
        && shared.mode.load(Ordering::Acquire) == mode
}

/// Consume only the two bounded locations. The host never holds a sink lock.
fn run<B: WriterBackend>(
    mut backend: B,
    shared: &Shared,
    receiver: Receiver<WriterRecord>,
) -> WriterClose {
    let mut pending: Option<WriterRecord> = None;
    let mut loss: bool = false;
    loop {
        let mode: u8 = shared.mode.load(Ordering::Acquire);
        if mode == FENCED {
            thread::park();
            continue;
        }
        if pending.is_none() {
            match receiver.try_recv() {
                Ok(record) => pending = Some(record),
                Err(TryRecvError::Disconnected) => {
                    loss = true;
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }
        }
        if mode == RETIRED {
            loss |= pending.is_some();
            break;
        }
        let Some(record) = pending.take() else {
            if mode == DRAINING {
                break;
            }
            thread::park();
            continue;
        };
        if start_write(shared, mode) {
            loss |= !backend.write(record);
        } else if mode == DRAINING && shared.mode.load(Ordering::Acquire) == DRAINING {
            // The mode remained current, so this refusal was the original cutoff.
            loss = true;
            break;
        } else {
            pending = Some(record);
        }
    }
    drop(receiver);
    loss |= !backend.close();
    if loss {
        WriterClose::Loss
    } else {
        WriterClose::Drained
    }
}

/// Real producer, bounded writer queue, backend effects and joined local close.
#[path = "diagnostics_thread_tests.rs"]
mod queue_control;

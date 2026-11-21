// SPDX-License-Identifier: GPL-2.0-only
//! Portable fixture writer with bounded admission and a physically joined owner.

use std::{
    marker::PhantomData,
    rc::Rc,
    sync::{
        Arc, OnceLock,
        atomic::{AtomicBool, AtomicU8, Ordering},
        mpsc::{Receiver, SyncSender, TryRecvError, sync_channel},
    },
    thread::{self, JoinHandle},
    time::Instant,
};

use windows_platform::diagnostics::WriterRecord;

use super::{
    CloseMode, ClosePending, EventWriter, ReleaseUnconfirmed, UnconfirmedWriter, WriterClose,
};
use crate::host::WorkerWake;

mod event_payload {
    //! Submission layout of the two events in the compiled service fixture.

    use windows_platform::diagnostics::{LifecycleObservationClaim, WriterRecord};

    /// Descriptor values read back from the compiled fixture manifest.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub(crate) struct EventDescriptor {
        /// Catalogue event identifier.
        pub(crate) id: u16,
        /// Event layout version.
        pub(crate) version: u8,
        /// Fixture Operational channel index.
        pub(crate) channel: u8,
        /// Event severity level.
        pub(crate) level: u8,
        /// Manifest opcode.
        pub(crate) opcode: u8,
        /// Manifest task.
        pub(crate) task: u16,
        /// Manifest keyword mask including the channel bit.
        pub(crate) keyword: u64,
    }

    /// One live router record in the exact layout expected by the fixture backend.
    #[must_use]
    pub(crate) struct PreparedEvent {
        /// Fixed descriptor selected by the admitted event.
        descriptor: EventDescriptor,
        /// Original emitting service role.
        component: [u8; 1],
        /// Original invocation-local sequence, in little-endian order.
        sequence: [u8; 8],
        /// Original observation kind.
        observation_kind: [u8; 1],
        /// Original elapsed coordinate, in little-endian order.
        observed_elapsed_ns: [u8; 8],
        /// Original permitted cause or canonical zero.
        cause: [u8; 1],
    }

    impl PreparedEvent {
        /// Project only a live routed record. An event outside the compiled pair
        /// has no submission layout. Projection performs no I/O or clock reads.
        pub(super) fn from_record(record: &WriterRecord) -> Option<Self> {
            let (id, level): (u16, u8) = match record.event_id() {
                2 => (2, 4),
                4 => (4, 2),
                _ => return None,
            };
            let (observation_kind, observed_elapsed_ns): (u8, u64) = match record.observation() {
                LifecycleObservationClaim::Unavailable => (0, 0),
                LifecycleObservationClaim::LastAccepted(elapsed_ns) => (1, elapsed_ns),
                LifecycleObservationClaim::HistoricalAfterClockFault(elapsed_ns) => (2, elapsed_ns),
            };
            Some(Self {
                descriptor: EventDescriptor {
                    id,
                    version: 1,
                    channel: 16,
                    level,
                    opcode: 0,
                    task: 0,
                    keyword: 0x8000_0000_0000_0001,
                },
                component: [record.component().code()],
                sequence: record.sequence().to_le_bytes(),
                observation_kind: [observation_kind],
                observed_elapsed_ns: observed_elapsed_ns.to_le_bytes(),
                cause: [record.cause_code()],
            })
        }

        /// Borrow the descriptor tied to these fields.
        pub(crate) const fn descriptor(&self) -> &EventDescriptor {
            &self.descriptor
        }

        /// Borrow the five fields in manifest order, with no padding between bytes
        /// within each field.
        pub(crate) fn fields(&self) -> [&[u8]; 5] {
            [
                &self.component,
                &self.sequence,
                &self.observation_kind,
                &self.observed_elapsed_ns,
                &self.cause,
            ]
        }
    }
}
pub(crate) use event_payload::{EventDescriptor, PreparedEvent};

/// Admission of the original writer's one provider registration attempt.
pub(crate) struct RegisterOnce<'a> {
    /// Original writer fence and cutoff.
    shared: &'a Shared,
    /// Mode in which registration was selected.
    mode: u8,
    /// The synchronous registration belongs to this writer thread.
    thread_bound: PhantomData<Rc<()>>,
}

impl RegisterOnce<'_> {
    /// Enter the synchronous registration only under its own current claim.
    pub(crate) fn register(self, effect: impl FnOnce() -> bool) -> RegistrationAttempt {
        let attempt: Attempt = if start_write(self.shared, self.mode) {
            Attempt::Returned(effect())
        } else {
            Attempt::Unstarted
        };
        RegistrationAttempt(attempt)
    }
}

/// Outcome produced by consuming the registration claim.
#[must_use]
pub(crate) struct RegistrationAttempt(Attempt);

/// One prepared event and its current admission boundary on the writer thread.
pub(crate) struct SubmitOnce<'a> {
    /// The same packet remains in place through preparation and actual return.
    event: &'a PreparedEvent,
    /// The original writer gate, never a backend-supplied clock or deadline.
    shared: &'a Shared,
    /// The mode which selected this record for preparation.
    mode: u8,
    /// Prevent transfer to another thread, including a scoped thread.
    thread_bound: PhantomData<Rc<()>>,
}

impl SubmitOnce<'_> {
    /// Inspect the exact packet while preparing the backend call.
    pub(crate) const fn event(&self) -> &PreparedEvent {
        self.event
    }

    /// Invoke one synchronous effect after preparation and a fresh gate check.
    /// The backend enters the effect here and retains custody through its return.
    pub(crate) fn submit(self, effect: impl FnOnce() -> bool) -> WriteAttempt {
        if start_write(self.shared, self.mode) {
            WriteAttempt(Attempt::Returned(effect()))
        } else {
            WriteAttempt(Attempt::Unstarted)
        }
    }
}

/// A backend can return only an outcome produced by its own submission gate.
#[must_use]
pub(crate) struct WriteAttempt(Attempt);

/// Interpret the result without allowing another effect for the same attempt.
enum Attempt {
    /// The backend returned a local success or loss.
    Returned(bool),
    /// The current gate refused to start the effect.
    Unstarted,
}

/// Observe completed local cleanup or preserve the original provider uncertainty.
pub(crate) type BackendClose = Result<WriterClose, ReleaseUnconfirmed>;

/// Keep write observations when the original backend leaves its release uncertain.
type JoinedWriter = Result<WriterClose, UnconfirmedWriter>;

/// The host still accepts offers and the writer can start a record.
const OPEN: u8 = 0;
/// The host closed admission before attempting a status call.
const FENCED: u8 = 1;
/// The host gave the writer one original drain deadline.
const DRAINING: u8 = 2;
/// The writer can resolve only effects already in custody.
const RETIRED: u8 = 3;

/// A dormant portable backend. Only its owned writer thread performs effects.
/// Dormant destruction has no external effects. Resource release is attempted
/// on this same thread. An unwind cannot supply an unobserved release result.
/// The backend and its hook exclude custom panic-payload destructors.
pub(crate) trait WriterBackend: Send + 'static {
    /// Prepare any fixed registration input and take a current registration claim.
    /// The portable default performs no native registration.
    fn register(&mut self, registration: RegisterOnce<'_>) -> RegistrationAttempt {
        registration.register(|| true)
    }

    /// Prepare one packet without effects, then consume its gate immediately before
    /// entering the synchronous write. Provider registration needs its own admission.
    fn write(&mut self, submission: SubmitOnce<'_>) -> WriteAttempt;

    /// Attempt actual release. A completed thread alone cannot confirm release.
    /// Retain the original registration and status when release is unconfirmed.
    ///
    /// # Errors
    /// Returns the uncertainty observed by this resource owner.
    fn close(self) -> BackendClose;
}

/// A fixture barrier after the final marker and before physical exit.
type ExitTail = Box<dyn FnOnce() + Send>;

/// A single writer with one queued record and at most one writer-owned record.
pub(crate) struct ThreadedWriter<B> {
    /// One unstarted backend and its prospective exit observation.
    dormant: Option<(B, ExitTail)>,
    /// The same thread stays owned until its actual completion is joined.
    job: Option<Job>,
    /// A joined outcome remains observable without rejoining or retrying release.
    joined: Option<JoinedWriter>,
}

/// All host-side controls for the single original thread.
struct Job {
    /// One bounded slot of accepted work.
    sender: SyncSender<WriterRecord>,
    /// Monotonic gate and return observation shared with this thread.
    shared: Arc<Shared>,
    /// The only completion owner of the original job.
    thread: JoinHandle<JoinedWriter>,
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
            joined: None,
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
        let worker: std::io::Result<JoinHandle<JoinedWriter>> =
            thread::Builder::new().spawn(move || {
                let outcome: JoinedWriter = run(backend, &marker.shared, receiver);
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
        if let Some(joined) = self.joined {
            return joined.map_err(ClosePending::ReleaseUnconfirmed);
        }
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
        // Physical join can succeed without a confirmed provider release.
        let joined: JoinedWriter = job.thread.join().unwrap_or(Err(UnconfirmedWriter {
            release: ReleaseUnconfirmed::WriterPanicked,
            loss_observed: false,
        }));
        self.joined = Some(joined);
        joined.map_err(ClosePending::ReleaseUnconfirmed)
    }
}

/// Claim a logical start against the current host mode and original cutoff.
/// One claimed synchronous call remains in custody if Stop follows the claim.
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
///
/// # Errors
/// Preserve an explicit backend release uncertainty even after local loss.
fn run<B: WriterBackend>(
    mut backend: B,
    shared: &Shared,
    receiver: Receiver<WriterRecord>,
) -> JoinedWriter {
    let mut pending: Option<WriterRecord> = None;
    let mut loss: bool = false;
    let mut registered: bool = false;
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
        let attempt: Attempt = if registered {
            let Some(event): Option<PreparedEvent> = PreparedEvent::from_record(&record) else {
                loss = true;
                continue;
            };
            let submission: SubmitOnce<'_> = SubmitOnce {
                event: &event,
                shared,
                mode,
                thread_bound: PhantomData,
            };
            backend.write(submission).0
        } else {
            let registration: RegisterOnce<'_> = RegisterOnce {
                shared,
                mode,
                thread_bound: PhantomData,
            };
            match backend.register(registration).0 {
                Attempt::Returned(success) => {
                    if !success {
                        loss = true;
                        break;
                    }
                    // Keep the original, then take a fresh write claim after registration.
                    registered = true;
                    pending = Some(record);
                    continue;
                }
                Attempt::Unstarted => Attempt::Unstarted,
            }
        };
        match attempt {
            Attempt::Returned(success) => loss |= !success,
            Attempt::Unstarted
                if mode == DRAINING && shared.mode.load(Ordering::Acquire) == DRAINING =>
            {
                // The mode remained current, so the original cutoff refused this call.
                loss = true;
                break;
            }
            Attempt::Unstarted => pending = Some(record),
        }
    }
    drop(receiver);
    let closed: WriterClose =
        backend
            .close()
            .map_err(|release: ReleaseUnconfirmed| UnconfirmedWriter {
                release,
                loss_observed: loss,
            })?;
    if loss {
        Ok(WriterClose::Loss)
    } else {
        Ok(closed)
    }
}

/// Real producer, bounded writer queue, backend effects and joined local close.
#[path = "diagnostics_thread_tests.rs"]
mod queue_control;

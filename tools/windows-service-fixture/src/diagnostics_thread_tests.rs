// SPDX-License-Identifier: GPL-2.0-only
//! Real producer, bounded writer queue, backend effects and joined local close.

use std::{
    error::Error,
    num::{NonZeroU32, NonZeroU64},
    sync::mpsc::{Receiver, RecvTimeoutError, SyncSender, sync_channel},
    thread,
    time::{Duration, Instant},
};

use windows_platform::diagnostics::{
    ConsumerState, DiagnosticObservation, MemoryConsumer, PrimaryCause, WriterRecord,
};

use super::{
    BackendClose, CloseMode, ClosePending, EventDescriptor, EventWriter, JoinedWriter,
    RegisterOnce, RegistrationAttempt, ReleaseUnconfirmed, SubmitOnce, ThreadedWriter,
    UnconfirmedWriter, WriteAttempt, WriterBackend, WriterClose,
};
use crate::{
    LabRole,
    diagnostics::{Diagnostics, Event, Sinks},
    host::WorkerWake,
};

/// Bound the independent test-side observation and backend barrier waits.
const WATCHDOG: Duration = Duration::from_secs(5);

/// Exact submissions and local close observed at the backend boundary.
#[derive(Debug, Eq, PartialEq)]
enum Effect {
    /// The original owned registration was invoked.
    Register(Option<NonZeroU64>),
    /// The backend received this descriptor and its five field byte sequences.
    Write(EventDescriptor, [Vec<u8>; 5]),
    /// The backend consumed its local close operation.
    Close,
    /// Release of the exact token returned by the scripted registration.
    CloseRegistered(NonZeroU64),
}

/// Finite backend effects visible regardless of the writer's close verdict.
struct Backend {
    /// Enough slots to observe all three proposed writes and one close.
    effects: SyncSender<Effect>,
    /// The first call owns this barrier until the test releases it.
    first_release: Option<Receiver<()>>,
    /// An optional barrier after preparation and before admission of the effect.
    prepared: Option<(SyncSender<()>, Receiver<()>)>,
    /// Independent disposition of the held backend resource.
    release_unconfirmed: Option<ReleaseUnconfirmed>,
}

impl WriterBackend for Backend {
    /// Acknowledge backend entry before waiting on the first record.
    fn write(&mut self, submission: SubmitOnce<'_>) -> WriteAttempt {
        let prepared: Effect = Effect::Write(
            *submission.event().descriptor(),
            submission.event().fields().map(<[u8]>::to_vec),
        );
        if let Some((notice, release)) = self.prepared.take()
            && (notice.try_send(()).is_err() || release.recv_timeout(WATCHDOG).is_err())
        {
            return submission.submit(|| false);
        }
        submission.submit(|| {
            let observed: bool = self.effects.try_send(prepared).is_ok();
            if let Some(release) = self.first_release.take() {
                return observed
                    && matches!(
                        release.recv_timeout(WATCHDOG),
                        Err(RecvTimeoutError::Disconnected)
                    );
            }
            observed
        })
    }

    /// Observe actual backend close without supplying join or writer success.
    fn close(self) -> BackendClose {
        let observed: bool = self.effects.try_send(Effect::Close).is_ok();
        if let Some(release) = self.release_unconfirmed {
            Err(release)
        } else if observed {
            Ok(WriterClose::Drained)
        } else {
            Ok(WriterClose::Loss)
        }
    }
}

/// Observe close through the real adapter. A second bounded window lets the
/// original thread retire after the assertion watchdog expires.
/// # Errors
/// Preserve a writer that did not expose physical completion before the watchdog.
fn joined_close<B: WriterBackend>(
    writer: &mut ThreadedWriter<B>,
    deadline: Instant,
    wake: &WorkerWake,
) -> Result<JoinedWriter, Box<dyn Error>> {
    let start: Instant = Instant::now();
    while start.elapsed() < WATCHDOG {
        match writer.poll_close(CloseMode::Drain(deadline), wake) {
            Ok(close) => return Ok(Ok(close)),
            Err(ClosePending::ReleaseUnconfirmed(release)) => return Ok(Err(release)),
            Err(ClosePending::Wake | ClosePending::JoinTail) => {}
        }
        thread::park_timeout(Duration::from_millis(1));
    }
    let retirement: Instant = Instant::now();
    while retirement.elapsed() < WATCHDOG {
        if !matches!(
            writer.poll_close(CloseMode::Retire, wake),
            Err(ClosePending::Wake | ClosePending::JoinTail)
        ) {
            return Err("writer did not finish by the original watchdog".into());
        }
        thread::park_timeout(Duration::from_millis(1));
    }
    eprintln!("portable writer test could not retire its original thread");
    std::process::abort();
}

/// Admit one original through the fixture's genuine selected writer route.
/// # Errors
/// Preserve a routing failure before any writer thread starts.
fn emit(
    diagnostics: &mut Diagnostics,
    event: Event,
    observation: DiagnosticObservation,
) -> Result<WriterRecord, Box<dyn Error>> {
    diagnostics
        .emit_for_writer(event, observation)
        .ok_or_else(|| "eligible event did not yield a writer record".into())
}

/// Independently frozen layout for the first and second broker records.
fn expected_write(id: u16, level: u8, fields: [Vec<u8>; 5]) -> Effect {
    Effect::Write(
        EventDescriptor {
            id,
            version: 1,
            channel: 16,
            level,
            opcode: 0,
            task: 0,
            keyword: 0x8000_0000_0000_0001,
        },
        fields,
    )
}

/// Stage at a selected side of the actual registration claim.
#[derive(Clone, Copy, Eq, PartialEq)]
enum RegistrationBarrier {
    /// Make the original drain deadline expire before trying registration.
    BeforeClaim,
    /// Complete an already-started registration after the drain cutoff.
    InFlight,
    /// Registration actually returns failure while still timely.
    Refused,
}

/// Finite recorder of actual registration, write and close entry.
struct RegistrationBackend {
    /// Original selected registration schedule.
    barrier: RegistrationBarrier,
    /// Handshake with the cutoff observer.
    ready: SyncSender<()>,
    /// The test publishes its cutoff before returning this gate.
    release: Receiver<()>,
    /// Exact bounded effect observations.
    observed: SyncSender<Effect>,
    /// Token returned by the original successful registration.
    token: Option<NonZeroU64>,
}

impl RegistrationBackend {
    /// Notify the driver before waiting for the original deadline transition.
    fn hold(&self) -> bool {
        self.ready.try_send(()).is_ok() && self.release.recv_timeout(WATCHDOG).is_ok()
    }
}

impl WriterBackend for RegistrationBackend {
    /// Observe one real registration attempt on the selected side of a hold.
    fn register(&mut self, registration: RegisterOnce<'_>) -> RegistrationAttempt {
        if self.barrier == RegistrationBarrier::BeforeClaim && !self.hold() {
            return registration.register(|| false);
        }
        registration.register(|| {
            self.token = (self.barrier != RegistrationBarrier::Refused).then_some(NonZeroU64::MIN);
            self.observed.try_send(Effect::Register(self.token)).is_ok()
                && (self.barrier == RegistrationBarrier::BeforeClaim || self.hold())
                && self.token.is_some()
        })
    }

    /// Capture an unexpected write under the real submission claim.
    fn write(&mut self, submission: SubmitOnce<'_>) -> WriteAttempt {
        let event: Effect = Effect::Write(
            *submission.event().descriptor(),
            submission.event().fields().map(<[u8]>::to_vec),
        );
        submission.submit(|| self.observed.try_send(event).is_ok())
    }

    /// Record actual cleanup of the same dormant or registered backend.
    fn close(self) -> BackendClose {
        let close: Effect = self.token.map_or(Effect::Close, Effect::CloseRegistered);
        if self.observed.try_send(close).is_ok() {
            Ok(WriterClose::Drained)
        } else {
            Ok(WriterClose::Loss)
        }
    }
}

/// Challenge the separate registration claim and the fresh write after return.
/// # Errors
/// Propagate real input admission, local timing and thread observation errors.
/// # Panics
/// Fails if a held registration or a late write crosses the cutoff.
fn registration_crossing_cutoff(barrier: RegistrationBarrier) -> Result<(), Box<dyn Error>> {
    let mut diagnostics: Diagnostics = Diagnostics::new(
        LabRole::Engine,
        Sinks::new(MemoryConsumer::new(ConsumerState::Available), None),
    );
    let record: WriterRecord = emit(
        &mut diagnostics,
        Event::RunningAcknowledged,
        DiagnosticObservation::unavailable(),
    )?;
    let (observed, effects): (SyncSender<Effect>, Receiver<Effect>) = sync_channel(4);
    let (notice, ready): (SyncSender<()>, Receiver<()>) = sync_channel(1);
    let (resume, release): (SyncSender<()>, Receiver<()>) = sync_channel(1);
    let backend: RegistrationBackend = RegistrationBackend {
        barrier,
        ready: notice,
        release,
        observed,
        token: None,
    };
    let mut writer: ThreadedWriter<RegistrationBackend> =
        ThreadedWriter::with_exit_tail(backend, || {});
    let wake: WorkerWake = WorkerWake::current();
    let accepted: bool = writer.offer(record, &wake);
    let waiting: Result<(), RecvTimeoutError> = ready.recv_timeout(WATCHDOG);
    let now: Instant = Instant::now();
    let cutoff: Instant = if barrier == RegistrationBarrier::Refused {
        now.checked_add(Duration::from_secs(30))
            .ok_or("healthy registration deadline overflowed")?
    } else {
        now
    };
    let held: Result<WriterClose, ClosePending> =
        writer.poll_close(CloseMode::Drain(cutoff), &wake);
    let released: bool = resume.try_send(()).is_ok();
    drop(resume);
    assert_eq!(
        joined_close(&mut writer, cutoff, &wake)?,
        Ok(WriterClose::Loss)
    );
    assert!(accepted && released);
    waiting?;
    assert!(matches!(held, Err(ClosePending::Wake)));
    let expected: &[Effect] = match barrier {
        RegistrationBarrier::BeforeClaim => &[Effect::Close],
        RegistrationBarrier::InFlight => &[
            Effect::Register(Some(NonZeroU64::MIN)),
            Effect::CloseRegistered(NonZeroU64::MIN),
        ],
        RegistrationBarrier::Refused => &[Effect::Register(None), Effect::Close],
    };
    assert_eq!(effects.try_iter().collect::<Vec<Effect>>(), expected);
    Ok(())
}

/// Cross the fence after preparation and challenge both original drain deadlines.
/// # Errors
/// Preserves timing, routing and thread observation failures.
/// # Panics
/// Fails if the effect starts after its cutoff or loses a timely original.
fn cutoff_after_preparation(
    expired: bool,
    release_unconfirmed: Option<ReleaseUnconfirmed>,
) -> Result<(), Box<dyn Error>> {
    let mut diagnostics: Diagnostics = Diagnostics::new(
        LabRole::Engine,
        Sinks::new(MemoryConsumer::new(ConsumerState::Available), None),
    );
    let record: WriterRecord = emit(
        &mut diagnostics,
        Event::RunningAcknowledged,
        DiagnosticObservation::unavailable(),
    )?;
    let future: Instant = Instant::now()
        .checked_add(Duration::from_secs(30))
        .ok_or("drain deadline overflowed")?;
    let (effects, observed): (SyncSender<Effect>, Receiver<Effect>) = sync_channel(3);
    let (notice, prepared): (SyncSender<()>, Receiver<()>) = sync_channel(1);
    let (resume, release): (SyncSender<()>, Receiver<()>) = sync_channel(1);
    let backend: Backend = Backend {
        effects,
        first_release: None,
        prepared: Some((notice, release)),
        release_unconfirmed,
    };
    let mut writer: ThreadedWriter<Backend> = ThreadedWriter::with_exit_tail(backend, || {});
    let wake: WorkerWake = WorkerWake::current();
    let accepted: bool = writer.offer(record, &wake);
    let ready: Result<(), RecvTimeoutError> = prepared.recv_timeout(WATCHDOG);
    writer.fence();
    let deadline: Instant = if expired { Instant::now() } else { future };
    let held: Result<WriterClose, ClosePending> =
        writer.poll_close(CloseMode::Drain(deadline), &wake);
    let released: bool = resume.try_send(()).is_ok();
    drop(resume);
    let joined: JoinedWriter = joined_close(&mut writer, future, &wake)?;
    let actual: Vec<Effect> = observed.try_iter().collect();
    assert!(accepted && released);
    ready?;
    assert!(matches!(held, Err(ClosePending::Wake)));
    if expired {
        assert_eq!(actual, [Effect::Close], "write began after its cutoff");
    } else {
        assert_eq!(
            actual,
            [
                expected_write(
                    2,
                    4,
                    [
                        vec![1],
                        vec![1, 0, 0, 0, 0, 0, 0, 0],
                        vec![0],
                        vec![0; 8],
                        vec![0]
                    ]
                ),
                Effect::Close,
            ]
        );
    }
    let expected: JoinedWriter = release_unconfirmed.map_or_else(
        || {
            Ok(if expired {
                WriterClose::Loss
            } else {
                WriterClose::Drained
            })
        },
        |release: ReleaseUnconfirmed| {
            Err(UnconfirmedWriter {
                release,
                loss_observed: expired,
            })
        },
    );
    assert_eq!(joined, expected);
    assert_eq!(joined_close(&mut writer, future, &wake)?, expected);
    Ok(())
}

/// An in-flight original permits one queued original and refuses a third.
/// At the original cutoff only the already-started write may complete.
/// # Errors
/// Preserve producer admission, checked time, observation and real join failures.
/// # Panics
/// Fails on queue, sequence, backend roster, cutoff or local close divergence.
#[test]
fn one_queued_original_obeys_the_drain_cutoff() -> Result<(), Box<dyn Error>> {
    let cases: &[(bool, &[Effect], WriterClose)] = &[
        (
            false,
            &[
                expected_write(
                    4,
                    2,
                    [
                        vec![2],
                        vec![2, 0, 0, 0, 0, 0, 0, 0],
                        vec![2],
                        vec![0x34, 0x12, 0, 0, 0, 0, 0, 0],
                        vec![10],
                    ],
                ),
                Effect::Close,
            ],
            WriterClose::Drained,
        ),
        (true, &[Effect::Close], WriterClose::Loss),
    ];
    for &(expired, expected_tail, expected_close) in cases {
        let mut diagnostics: Diagnostics = Diagnostics::new(
            LabRole::ClockBroker,
            Sinks::new(MemoryConsumer::new(ConsumerState::Available), None),
        );
        let origin: Instant = Instant::now();
        let last_accepted: Instant = origin
            .checked_add(Duration::from_nanos(0x1234))
            .ok_or("historical observation overflowed")?;
        let first: WriterRecord = emit(
            &mut diagnostics,
            Event::RunningAcknowledged,
            DiagnosticObservation::unavailable(),
        )?;
        let second: WriterRecord = emit(
            &mut diagnostics,
            Event::PrimaryFailure(PrimaryCause::LifecycleReport),
            DiagnosticObservation::from_lifecycle(origin, last_accepted, true),
        )?;
        let third: WriterRecord = emit(
            &mut diagnostics,
            Event::RunningAcknowledged,
            DiagnosticObservation::unavailable(),
        )?;
        let sequences: [u64; 3] = [first.sequence(), second.sequence(), third.sequence()];

        let (effects, observed): (SyncSender<Effect>, Receiver<Effect>) = sync_channel(4);
        let (release, first_release): (SyncSender<()>, Receiver<()>) = sync_channel(1);
        let backend: Backend = Backend {
            effects,
            first_release: Some(first_release),
            prepared: None,
            release_unconfirmed: None,
        };
        let mut writer: ThreadedWriter<Backend> = ThreadedWriter::with_exit_tail(backend, || {});
        let wake: WorkerWake = WorkerWake::current();
        let future: Instant = Instant::now()
            .checked_add(Duration::from_secs(30))
            .ok_or("healthy drain deadline overflowed")?;

        let first_accepted: bool = writer.offer(first, &wake);
        let first_effect: Result<Effect, RecvTimeoutError> = observed.recv_timeout(WATCHDOG);
        let second_accepted: bool = writer.offer(second, &wake);
        let third_accepted: bool = writer.offer(third, &wake);
        let deadline: Instant = if expired { Instant::now() } else { future };
        let while_held: Result<WriterClose, ClosePending> =
            writer.poll_close(CloseMode::Drain(deadline), &wake);

        // Drop the only sender before any assertion or fallible return.
        drop(release);
        // Repeated drain requests must preserve the first cutoff.
        let closed: JoinedWriter = joined_close(&mut writer, future, &wake)?;
        let tail: Vec<Effect> = observed.try_iter().collect();
        assert_eq!(sequences, [1, 2, 3]);
        assert!(first_accepted);
        assert_eq!(
            first_effect?,
            expected_write(
                2,
                4,
                [
                    vec![2],
                    vec![1, 0, 0, 0, 0, 0, 0, 0],
                    vec![0],
                    vec![0; 8],
                    vec![0],
                ],
            )
        );
        assert!(second_accepted);
        assert!(!third_accepted);
        assert!(matches!(while_held, Err(ClosePending::Wake)));
        assert_eq!(tail.as_slice(), expected_tail, "expired={expired}");
        assert_eq!(closed, Ok(expected_close), "expired={expired}");
    }
    admission_and_release_controls()
}

/// Exercise both native effect claims and the independently latched release result.
/// # Errors
/// Preserve routing, thread completion and observation failures from the controls.
fn admission_and_release_controls() -> Result<(), Box<dyn Error>> {
    cutoff_after_preparation(false, None)?;
    cutoff_after_preparation(true, None)?;
    let release: ReleaseUnconfirmed = ReleaseUnconfirmed::Provider {
        registration: NonZeroU64::MIN,
        status: NonZeroU32::MIN,
    };
    cutoff_after_preparation(false, Some(release))?;
    cutoff_after_preparation(true, Some(release))?;
    registration_crossing_cutoff(RegistrationBarrier::BeforeClaim)?;
    registration_crossing_cutoff(RegistrationBarrier::InFlight)?;
    registration_crossing_cutoff(RegistrationBarrier::Refused)?;
    Ok(())
}

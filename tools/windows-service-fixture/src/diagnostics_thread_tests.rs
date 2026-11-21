// SPDX-License-Identifier: GPL-2.0-only
//! Real producer, bounded writer queue, backend effects and joined local close.

use std::{
    error::Error,
    sync::mpsc::{Receiver, RecvTimeoutError, SyncSender, sync_channel},
    thread,
    time::{Duration, Instant},
};

use windows_platform::diagnostics::{
    ConsumerState, DiagnosticObservation, MemoryConsumer, WriterRecord,
};

use super::{CloseMode, ClosePending, EventWriter, ThreadedWriter, WriterBackend, WriterClose};
use crate::{
    LabRole,
    diagnostics::{Diagnostics, Event, Sinks},
    host::WorkerWake,
};

/// Bound the independent test-side observation and backend barrier waits.
const WATCHDOG: Duration = Duration::from_secs(5);

/// The real backend effect roster contains only sequence IDs and local close.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Effect {
    /// The backend received this original record for writing.
    Write(u64),
    /// The backend consumed its local close operation.
    Close,
}

/// Finite backend effects visible regardless of the writer's close verdict.
struct Backend {
    /// Enough slots to observe all three proposed writes and one close.
    effects: SyncSender<Effect>,
    /// The first call owns this barrier until the test releases it.
    first_release: Option<Receiver<()>>,
}

impl WriterBackend for Backend {
    /// Acknowledge backend entry before waiting on the first record.
    fn write(&mut self, record: WriterRecord) -> bool {
        let observed: bool = self
            .effects
            .try_send(Effect::Write(record.sequence()))
            .is_ok();
        if let Some(release) = self.first_release.take() {
            return observed
                && matches!(
                    release.recv_timeout(WATCHDOG),
                    Err(RecvTimeoutError::Disconnected)
                );
        }
        observed
    }

    /// Observe actual backend close without supplying join or writer success.
    fn close(self) -> bool {
        self.effects.try_send(Effect::Close).is_ok()
    }
}

/// Observe close through the real adapter. A second bounded window lets the
/// original thread retire after the assertion watchdog expires.
/// # Errors
/// Preserve a writer that did not expose physical completion before the watchdog.
fn joined_close(
    writer: &mut ThreadedWriter<Backend>,
    deadline: Instant,
    wake: &WorkerWake,
) -> Result<WriterClose, Box<dyn Error>> {
    let start: Instant = Instant::now();
    while start.elapsed() < WATCHDOG {
        if let Ok(close) = writer.poll_close(CloseMode::Drain(deadline), wake) {
            return Ok(close);
        }
        thread::park_timeout(Duration::from_millis(1));
    }
    let retirement: Instant = Instant::now();
    while retirement.elapsed() < WATCHDOG {
        if writer.poll_close(CloseMode::Retire, wake).is_ok() {
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
fn emit(diagnostics: &mut Diagnostics) -> Result<WriterRecord, Box<dyn Error>> {
    diagnostics
        .emit_for_writer(
            Event::RunningAcknowledged,
            DiagnosticObservation::unavailable(),
        )
        .ok_or_else(|| "eligible event did not yield a writer record".into())
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
            &[Effect::Write(2), Effect::Close],
            WriterClose::Drained,
        ),
        (true, &[Effect::Close], WriterClose::Loss),
    ];
    for &(expired, expected_tail, expected_close) in cases {
        let mut diagnostics: Diagnostics = Diagnostics::new(
            LabRole::Engine,
            Sinks::new(MemoryConsumer::new(ConsumerState::Available), None),
        );
        let first: WriterRecord = emit(&mut diagnostics)?;
        let second: WriterRecord = emit(&mut diagnostics)?;
        let third: WriterRecord = emit(&mut diagnostics)?;
        let sequences: [u64; 3] = [first.sequence(), second.sequence(), third.sequence()];

        let (effects, observed): (SyncSender<Effect>, Receiver<Effect>) = sync_channel(4);
        let (release, first_release): (SyncSender<()>, Receiver<()>) = sync_channel(1);
        let backend: Backend = Backend {
            effects,
            first_release: Some(first_release),
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
        let closed: WriterClose = joined_close(&mut writer, future, &wake)?;
        let tail: Vec<Effect> = observed.try_iter().collect();
        assert_eq!(sequences, [1, 2, 3]);
        assert!(first_accepted);
        assert_eq!(first_effect?, Effect::Write(1));
        assert!(second_accepted);
        assert!(!third_accepted);
        assert!(matches!(while_held, Err(ClosePending::Wake)));
        assert_eq!(tail.as_slice(), expected_tail, "expired={expired}");
        assert_eq!(closed, expected_close, "expired={expired}");
    }
    Ok(())
}

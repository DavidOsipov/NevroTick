// SPDX-License-Identifier: GPL-2.0-only
//! Windows library-test worker for native requests and replies on owned pipes.

use std::task::Poll;

use crate::{
    error::WorkerError,
    host::{Worker, WorkerCompletion, WorkerFailure, WorkerFinish, WorkerWake},
    native_pipe_identity::PipeIdentity,
    native_pipe_pair::{
        LabDuplexPipe, LabIdentificationPipe, LabOverlappedWriter, LabPipeError, LabPipeReader,
    },
    native_pipe_read::{
        IdentifiedReadError, IdentifiedRequestRead, NativeCancel, NativePipeRead, NativeReadBytes,
        NativeReadError, NativeReadPoll, NativeReadStart, RetiredRead,
    },
    native_pipe_write::{NativePipeWrite, NativeWriteError, NativeWritePoll, NativeWriteStart},
};

/// A direct Running-phase pipe call was refused.
#[derive(Debug, Eq, PartialEq)]
pub(super) enum NativePipeWorkerError {
    /// The host has yet to activate this worker or has closed admission.
    Inactive,
    /// The owned slot refused the request or reported a native failure.
    Native(NativeWriteError),
    /// The identified request slot could not be created.
    Read(NativeReadError),
}

/// The host grants activation once and cancellation permanently closes admission.
#[derive(Clone, Copy, Eq, PartialEq)]
enum Phase {
    /// Owns an unused slot before the host admits preparation.
    Dormant,
    /// Preparation completed and the host can activate the unused slot.
    Prepared,
    /// The acknowledged Running owner can submit on the occupied slots.
    Active,
    /// A known drain error must reach the host before later completion facts.
    ReportCleanup,
    /// Submission and direct result retrieval are permanently fenced.
    Draining,
}

/// Active ingress operations without mutable access to replace the owned slot.
pub(super) struct ActiveRequestRead<'a> {
    /// Exclusive borrow of the original request slot while admission remains open.
    read: &'a mut IdentifiedRequestRead,
}

impl ActiveRequestRead<'_> {
    /// Queue a bounded fragment on the original read slot.
    /// # Errors
    /// Preserves identification, slot and native submission refusals.
    pub(super) fn begin(&mut self, span: usize) -> Result<NativeReadStart, IdentifiedReadError> {
        self.read.begin(span)
    }

    /// Collect a Running fragment after successful native identification.
    /// # Errors
    /// Preserves any original completion or identity refusal.
    pub(super) fn poll(&mut self) -> Result<NativeReadPoll, IdentifiedReadError> {
        self.read.poll()
    }

    /// Observe the original one-use paired submission extent.
    pub(super) const fn client_write_span(&self) -> Option<u32> {
        self.read.client_write_span()
    }

    /// Recheck the two original paired handles.
    /// # Errors
    /// Refuses foreign or failed native peer observation.
    pub(super) fn check_peers(&self) -> Result<(), LabPipeError> {
        self.read.check_peers()
    }

    /// Submit the admitted fragment from the original client endpoint.
    /// # Errors
    /// Preserves slot, extent and native submission failures.
    pub(super) fn write_from_client(&mut self, bytes: &[u8]) -> Result<(), LabPipeError> {
        self.read.write_from_client(bytes)
    }

    /// Observe the original operation's event without retrieving its completion.
    /// # Errors
    /// Preserves an idle slot, timeout or native wait failure.
    pub(super) fn wait_for_read_signal(&self) -> std::io::Result<()> {
        self.read.wait_for_read_signal()
    }

    /// Whether the original operation still owns its native storage.
    pub(super) const fn is_pending(&self) -> bool {
        self.read.is_pending()
    }
}

/// Read retirement shared by session expiry and the host's Stop path.
#[derive(Default)]
pub(super) struct ReadRetirement {
    /// First cancellation request on this irreversibly retired ingress.
    cancellation: Option<Result<NativeCancel, NativeReadError>>,
    /// Latched physical result, containing no frame data.
    terminal: Option<Result<RetiredRead, NativeReadError>>,
    /// First failed completion observation while the operation still owned its storage.
    completion_error: Option<u32>,
}

impl ReadRetirement {
    /// Retain the first native cancellation outcome across both retirement paths.
    /// # Errors
    /// Preserves a refused cancellation request without freeing the slot.
    fn request(
        &mut self,
        cancel: impl FnOnce() -> Result<NativeCancel, NativeReadError>,
    ) -> Result<NativeCancel, NativeReadError> {
        *self.cancellation.get_or_insert_with(cancel)
    }

    /// Collect physical completion after retiring the session ingress.
    /// # Errors
    /// Preserves the current completion refusal and retains the first unconfirmed failure.
    fn poll(
        &mut self,
        read: &mut IdentifiedRequestRead,
    ) -> Result<Poll<RetiredRead>, NativeReadError> {
        if let Some(terminal) = self.terminal {
            return terminal.map(Poll::Ready);
        }
        if self.cancellation.is_none() {
            return Err(NativeReadError::Idle);
        }
        let observation: Result<Poll<RetiredRead>, NativeReadError> = read.poll_retirement();
        match observation {
            Ok(Poll::Pending) => {}
            Ok(Poll::Ready(terminal)) => self.terminal = Some(Ok(terminal)),
            Err(NativeReadError::Completion(code)) => {
                self.completion_error.get_or_insert(code);
            }
            Err(error) => self.terminal = Some(Err(error)),
        }
        observation
    }

    /// Inspect the original cancellation request and the physical result independently.
    pub(super) const fn cancellation(&self) -> Option<Result<NativeCancel, NativeReadError>> {
        self.cancellation
    }

    /// Inspect the physical completion after retirement.
    pub(super) const fn terminal(&self) -> Option<Result<RetiredRead, NativeReadError>> {
        self.terminal
    }
}

/// Exclusive retirement access from the active session, independent of admission.
pub(super) struct RetiringRequestRead<'a> {
    /// Original request slot retained by the worker.
    read: &'a mut IdentifiedRequestRead,
    /// Same observations the worker will consult during Stop.
    retirement: &'a mut ReadRetirement,
}

impl RetiringRequestRead<'_> {
    /// Observe whether the same native operation still owns its storage.
    pub(super) const fn is_pending(&self) -> bool {
        self.read.is_pending()
    }

    /// Close ingress, requesting cancellation at most once.
    /// # Errors
    /// Returns the original native refusal if cancellation failed.
    pub(super) fn retire(&mut self) -> Result<NativeCancel, NativeReadError> {
        self.retirement.request(|| self.read.retire())
    }

    /// Observe physical completion with no frame data or identity capture.
    /// # Errors
    /// Preserves native completion refusal while custody remains occupied.
    pub(super) fn poll_retirement(&mut self) -> Result<Poll<RetiredRead>, NativeReadError> {
        self.retirement.poll(self.read)
    }
}

/// Allocated but unsubmitted slots for the exact connected duplex transport.
pub(super) struct PreparedDuplex {
    /// Engine-side read on the original client handle.
    client_read: NativePipeRead,
    /// Broker-side identified read on the original server handle.
    broker_read: IdentifiedRequestRead,
    /// Broker-side write on that same server handle.
    broker_write: NativePipeWrite,
}

impl PreparedDuplex {
    /// Allocate all idle slots before the receiving owner rechecks currentness.
    /// # Errors
    /// Retains the allocation failure. No native read or write has been submitted.
    pub(super) fn new(
        pipe: LabDuplexPipe,
        expected: PipeIdentity,
    ) -> Result<Self, NativePipeWorkerError> {
        let (ingress, reader, writer): (LabIdentificationPipe, LabPipeReader, LabOverlappedWriter) =
            pipe.into_transport();
        let client_read: NativePipeRead =
            NativePipeRead::new(reader).map_err(NativePipeWorkerError::Read)?;
        let broker_read: IdentifiedRequestRead =
            IdentifiedRequestRead::new(ingress, expected).map_err(NativePipeWorkerError::Read)?;
        let broker_write: NativePipeWrite =
            NativePipeWrite::new(writer).map_err(NativePipeWorkerError::Native)?;
        Ok(Self {
            client_read,
            broker_read,
            broker_write,
        })
    }
}

/// One serialized fixture worker whose pipe resources follow native completion.
pub(super) struct NativePipeWorker {
    /// Host-owned activation and irreversible Stop fence.
    phase: Phase,
    /// Identified request ingress, whose original operation follows Stop custody.
    read: Option<IdentifiedRequestRead>,
    /// Original pipe, event and write storage, released after terminal observation.
    write: Option<NativePipeWrite>,
    /// Exact result of the single request, including cancellation racing completion.
    cancellation: Option<Result<NativeCancel, NativeWriteError>>,
    /// Original physical result collected after the Stop fence.
    terminal: Option<Result<NativeWritePoll, NativeWriteError>>,
    /// First unconfirmed completion error, retained even after a later terminal result.
    completion_error: Option<u32>,
    /// Original request retirement from session expiry through broker Stop.
    read_retirement: ReadRetirement,
}

impl NativePipeWorker {
    /// Check the actual reader against the worker's retained reply endpoint.
    /// # Errors
    /// Refuses missing write custody, a foreign pair or failed native peer observations.
    pub(super) fn check_reply_reader(&self, reader: &NativePipeRead) -> Result<(), LabPipeError> {
        self.reply_write()?.check_reply_reader(reader)
    }

    /// Check where the actual completed read came from before frame assembly.
    /// # Errors
    /// Refuses missing write custody or bytes from a different physical pair.
    pub(super) fn check_reply_completion(
        &self,
        read: &NativeReadBytes,
    ) -> Result<(), LabPipeError> {
        self.reply_write()?.check_reply_completion(read)
    }

    /// Borrow the original slot solely for reply checks.
    /// # Errors
    /// Refuses a worker that no longer owns the writer.
    fn reply_write(&self) -> Result<&NativePipeWrite, LabPipeError> {
        self.write
            .as_ref()
            .ok_or(LabPipeError::MissingResource("broker reply writer"))
    }

    /// Acquire an initially idle slot for the already-connected fixture endpoint.
    /// # Errors
    /// Preserves a failure to create the private completion event.
    pub(super) fn new(writer: LabOverlappedWriter) -> Result<Self, NativeWriteError> {
        Ok(Self::dormant(Some(NativePipeWrite::new(writer)?)))
    }

    /// Prepare the duplex worker without exposing a request or reply slot.
    pub(super) fn awaiting_connection() -> Self {
        Self::dormant(None)
    }

    /// Keep the same retirement state before any Running permit can be issued.
    fn dormant(write: Option<NativePipeWrite>) -> Self {
        Self {
            phase: Phase::Dormant,
            read: None,
            write,
            cancellation: None,
            terminal: None,
            completion_error: None,
            read_retirement: ReadRetirement::default(),
        }
    }

    /// Bind the original request reader before Running admits any pipe operation.
    /// # Errors
    /// Preserves a failure to create either private completion event.
    pub(super) fn with_request_read(
        writer: LabOverlappedWriter,
        pipe: LabIdentificationPipe,
        expected: PipeIdentity,
    ) -> Result<Self, NativePipeWorkerError> {
        let mut worker: Self = Self::new(writer).map_err(NativePipeWorkerError::Native)?;
        worker.read =
            Some(IdentifiedRequestRead::new(pipe, expected).map_err(NativePipeWorkerError::Read)?);
        Ok(worker)
    }

    /// Move prepared, unsubmitted slots into this worker without further native calls.
    /// # Errors
    /// Refuses replacing existing custody or binding outside Running.
    pub(super) fn attach_duplex(
        &mut self,
        ready: PreparedDuplex,
    ) -> Result<NativePipeRead, NativePipeWorkerError> {
        if self.phase != Phase::Active || self.read.is_some() || self.write.is_some() {
            return Err(NativePipeWorkerError::Inactive);
        }
        let PreparedDuplex {
            client_read,
            broker_read,
            broker_write,
        } = ready;
        self.read = Some(broker_read);
        self.write = Some(broker_write);
        Ok(client_read)
    }

    /// Lend the original ingress slot only during acknowledged Running.
    pub(super) fn request_read(&mut self) -> Option<ActiveRequestRead<'_>> {
        if self.phase == Phase::Active {
            self.read.as_mut().map(|read| ActiveRequestRead { read })
        } else {
            None
        }
    }

    /// Keep session retirement available even after its receive admission closes.
    pub(super) fn retiring_request_read(&mut self) -> Option<RetiringRequestRead<'_>> {
        if self.phase != Phase::Active {
            return None;
        }
        Some(RetiringRequestRead {
            read: self.read.as_mut()?,
            retirement: &mut self.read_retirement,
        })
    }

    /// Observe the original retirement request and result.
    pub(super) const fn read_retirement(&self) -> &ReadRetirement {
        &self.read_retirement
    }

    /// Submit one bounded write under the host's current Running admission.
    /// # Errors
    /// Refuses inactive admission, an occupied slot, invalid size or native failure.
    pub(super) fn begin(
        &mut self,
        bytes: &[u8],
    ) -> Result<NativeWriteStart, NativePipeWorkerError> {
        self.active_write()?
            .begin(bytes)
            .map_err(NativePipeWorkerError::Native)
    }

    /// Observe a write during Running. A pending result leaves the slot occupied.
    /// # Errors
    /// Refuses inactive admission, missing submission or native failure.
    pub(super) fn poll_write(&mut self) -> Result<NativeWritePoll, NativePipeWorkerError> {
        self.active_write()?
            .poll()
            .map_err(NativePipeWorkerError::Native)
    }

    /// Read the exact drain result, independently of the cancellation request.
    pub(super) const fn terminal(&self) -> Option<&Result<NativeWritePoll, NativeWriteError>> {
        self.terminal.as_ref()
    }

    /// Read the single request outcome if Stop found a write to cancel.
    pub(super) const fn cancellation(&self) -> Option<&Result<NativeCancel, NativeWriteError>> {
        self.cancellation.as_ref()
    }

    /// Lend the slot only while the serialized host admits Running work.
    /// # Errors
    /// Refuses any call outside activation and before the Stop fence.
    fn active_write(&mut self) -> Result<&mut NativePipeWrite, NativePipeWorkerError> {
        match (self.phase, &mut self.write) {
            (Phase::Active, Some(write)) => Ok(write),
            _ => Err(NativePipeWorkerError::Inactive),
        }
    }

    /// Project bounded cleanup health while retaining both original native causes.
    fn cleanup_failure(&self) -> Option<WorkerError> {
        (self.cancellation.as_ref().is_some_and(Result::is_err)
            || self.completion_error.is_some()
            || self
                .read_retirement
                .cancellation
                .as_ref()
                .is_some_and(Result::is_err)
            || self.read_retirement.completion_error.is_some())
        .then_some(WorkerError::Cleanup)
    }
}

impl Worker for NativePipeWorker {
    /// Confirm the already-owned slot is ready without a submitted operation.
    fn poll_prepare(&mut self, _wake: &WorkerWake) -> Poll<Result<(), WorkerError>> {
        if matches!(self.phase, Phase::Dormant | Phase::Prepared) {
            self.phase = Phase::Prepared;
            Poll::Ready(Ok(()))
        } else {
            Poll::Ready(Err(WorkerError::Preparation))
        }
    }

    /// Enable submission after the host acknowledges Running and admits its permit.
    /// # Errors
    /// Refuses an unprepared, already-active or cancelled worker.
    fn activate(&mut self) -> Result<(), WorkerError> {
        if self.phase != Phase::Prepared {
            return Err(WorkerError::Activation);
        }
        self.phase = Phase::Active;
        Ok(())
    }

    /// Confirm activation. The Running fixture caller owns direct result collection.
    /// # Errors
    /// Refuses a health observation outside the active phase.
    fn poll_running(&mut self, _wake: &WorkerWake) -> Result<(), WorkerFailure> {
        if self.phase == Phase::Active {
            Ok(())
        } else {
            Err(WorkerFailure::Operation(WorkerError::Running))
        }
    }

    /// Close admission before the one targeted cancel request. Retain all custody.
    fn request_cancel(&mut self, _wake: &WorkerWake) {
        if matches!(self.phase, Phase::ReportCleanup | Phase::Draining) {
            return;
        }
        self.phase = Phase::Draining;
        if let Some(read) = &mut self.read
            && read.is_pending()
        {
            let _outcome: Result<NativeCancel, NativeReadError> =
                self.read_retirement.request(|| read.retire());
        }
        if let Some(write) = &mut self.write
            && write.is_pending()
        {
            let outcome: Result<NativeCancel, NativeWriteError> = write.request_cancel();
            self.cancellation = Some(outcome);
        }
        // A session expiry can already have observed failed read cleanup.
        // Preserve that observation even if the first Stop poll settles the read.
        if self.cleanup_failure().is_some() {
            self.phase = Phase::ReportCleanup;
        }
    }

    /// Release the exact slot only after its native operation is terminal.
    fn poll_finished(&mut self, _wake: &WorkerWake) -> WorkerFinish {
        if self.phase == Phase::ReportCleanup {
            // The host records this before a later terminal poll can supply another cause.
            self.phase = Phase::Draining;
            return WorkerFinish::NativeDrainPending {
                cleanup_failure: Some(WorkerError::Cleanup),
            };
        }
        if self.phase != Phase::Draining {
            return WorkerFinish::Pending;
        }
        if let Some(read) = &mut self.read
            && read.is_pending()
        {
            let _observation: Result<Poll<RetiredRead>, NativeReadError> =
                self.read_retirement.poll(read);
        }
        if let Some(write) = &mut self.write
            && write.is_pending()
        {
            match write.poll() {
                Ok(NativeWritePoll::Pending) => {}
                Err(NativeWriteError::Completion(code)) => {
                    self.completion_error.get_or_insert(code);
                }
                terminal => self.terminal = Some(terminal),
            }
        }
        if self
            .read
            .as_ref()
            .is_some_and(IdentifiedRequestRead::is_pending)
            || self.write.as_ref().is_some_and(NativePipeWrite::is_pending)
        {
            return WorkerFinish::NativeDrainPending {
                cleanup_failure: self.cleanup_failure(),
            };
        }
        drop(self.read.take());
        drop(self.write.take());
        let operation_failed: bool = self
            .terminal
            .as_ref()
            .is_some_and(|result| matches!(result, Err(_) | Ok(NativeWritePoll::Disconnected)))
            || self
                .read_retirement
                .terminal
                .as_ref()
                .is_some_and(|result| matches!(result, Err(_) | Ok(RetiredRead::Disconnected)));
        WorkerFinish::Finished(WorkerCompletion {
            operation_failure: operation_failed.then_some(WorkerError::Running),
            cleanup_failure: self.cleanup_failure(),
        })
    }
}

/// Prove that the two retirement paths cannot repeat or replace the native request.
/// # Panics
/// Fails if a later retirement invokes the request again or replaces the first result.
#[test]
fn repeated_read_retirement_calls_cancel_once_and_preserves_failure() {
    for first in [Ok(NativeCancel::Requested), Err(NativeReadError::Cancel(7))] {
        let mut requests: u8 = 0;
        let mut retirement: ReadRetirement = ReadRetirement::default();
        let original: Result<NativeCancel, NativeReadError> = retirement.request(|| {
            requests += 1;
            first
        });
        let repeated: Result<NativeCancel, NativeReadError> = retirement.request(|| {
            requests += 1;
            Ok(NativeCancel::NotFound)
        });
        assert_eq!((requests, original, repeated), (1, first, first));
        assert_eq!(retirement.cancellation(), Some(first));
        assert_eq!(retirement.terminal(), None);
    }
}

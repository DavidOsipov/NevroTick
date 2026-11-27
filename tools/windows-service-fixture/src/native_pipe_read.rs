// SPDX-License-Identifier: GPL-2.0-only
//! One event-backed overlapped read on an owned, process-local laboratory pipe.

use std::{
    cell::UnsafeCell,
    marker::PhantomPinned,
    os::windows::io::{AsHandle, AsRawHandle, BorrowedHandle, FromRawHandle, OwnedHandle},
    pin::Pin,
    ptr::{null, null_mut},
    task::Poll,
};

use windows_sys::Win32::{
    Foundation::{
        ERROR_BROKEN_PIPE, ERROR_IO_INCOMPLETE, ERROR_IO_PENDING, ERROR_NOT_FOUND,
        ERROR_OPERATION_ABORTED, GetLastError, HANDLE, WAIT_EVENT, WAIT_FAILED, WAIT_OBJECT_0,
        WAIT_TIMEOUT,
    },
    Storage::FileSystem::ReadFile,
    System::{
        IO::{CancelIoEx, GetOverlappedResult, OVERLAPPED},
        Threading::{CreateEventW, WaitForSingleObject},
    },
};

use crate::{
    native_broker_wait::{CompletionWakeState, PendingCompletionWake},
    native_pipe_identity::{
        CapturedRequestToken, FixtureClientError, PipeIdentity, PipeIdentityError,
        RetainedFixtureClient, capture,
    },
    native_pipe_pair::{
        LabIdentificationPipe, LabOverlappedWriter, LabPipeError, LabPipeReader, LabRequestWriter,
        LabUnconnectedDuplexPipe, ReplyPipePair, check_request_peers,
    },
    private_clock_wire::MAX_FRAME_LEN,
};

/// Local custody or native call refusal, retaining the original Win32 code.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum NativeReadError {
    /// The private completion event could not be acquired.
    Event(u32),
    /// A prior read still owns this slot.
    Busy,
    /// Read length is outside the fixed buffer.
    Size,
    /// No submitted operation can produce a result.
    Idle,
    /// The requested read could not start.
    Submit(u32),
    /// Completion could not be established. The slot still owns the operation.
    Completion(u32),
    /// The completed byte count exceeds its submitted extent.
    Count,
    /// The cancellation request failed. The operation remains occupied.
    Cancel(u32),
}

/// The actual native submission result. Both paths use the completion observer.
#[derive(Debug, Eq, PartialEq)]
#[must_use = "collect the submitted read through its original record"]
pub(super) enum NativeReadStart {
    /// Win32 completed the operation inline.
    Immediate,
    /// Win32 retained the supplied operation storage.
    Queued,
}

/// A successful exact prefix readable after native completion.
pub(super) struct NativeReadBytes {
    /// Fully initialized storage containing the completed prefix.
    bytes: [u8; MAX_FRAME_LEN],
    /// Checked actual transfer count.
    len: usize,
    /// Pair of the physical reader that produced this completion.
    reply_pair: Option<ReplyPipePair>,
}

impl NativeReadBytes {
    /// Return only the bytes acknowledged by the completed read.
    pub(super) fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }

    /// Return the pair of the actual source, when this was a native reply.
    pub(super) const fn reply_pair(&self) -> Option<&ReplyPipePair> {
        self.reply_pair.as_ref()
    }
}

/// A first read kept together with the server that actually produced it.
pub(super) struct CompletedFirstRead<'a> {
    /// No next read or disconnect can replace this read's attribution context.
    origin: &'a NativePipeRead,
    /// Exact completed prefix, before checking the fixture identity.
    bytes: NativeReadBytes,
}

impl CompletedFirstRead<'_> {
    /// Consume the exact read after the caller checks its current cutoff.
    ///
    /// # Errors
    /// Refuses empty completion or a different or unavailable native client identity.
    pub(super) fn attribute(
        self,
        expected: &PipeIdentity,
    ) -> Result<AttributedStatusRead, IdentifiedReadError> {
        let token: CapturedRequestToken = self.origin.require_client(&self.bytes, expected)?;
        Ok(AttributedStatusRead {
            bytes: self.bytes,
            token,
            #[cfg(test)]
            change_generation_after_dispatch: false,
        })
    }

    /// Match the read to the independently retained fixture client's token and component.
    ///
    /// # Errors
    /// Refuse unsuitable read attribution, changed continuity or a different component.
    pub(super) fn attribute_peer(
        self,
        peer: &mut RetainedFixtureClient,
    ) -> Result<AttributedStatusRead, IdentifiedReadError> {
        if !peer.observe_pipe(self.origin.pipe()) {
            return Err(IdentifiedReadError::Unattributed);
        }
        let token: CapturedRequestToken =
            self.origin.require_client(&self.bytes, peer.identity())?;
        peer.check_component(self.origin.pipe())
            .map_err(|cause: FixtureClientError| IdentifiedReadError::Component {
                cause,
                received: self.bytes.as_bytes().len(),
            })?;
        Ok(AttributedStatusRead {
            bytes: self.bytes,
            token,
            #[cfg(test)]
            change_generation_after_dispatch: false,
        })
    }
}

/// The completed prefix and query-only token from the original read.
pub(super) struct AttributedStatusRead {
    /// Only the native operation's exact completed bytes may reach dispatch.
    bytes: NativeReadBytes,
    /// The same identification token retained through dispatch admission.
    token: CapturedRequestToken,
    /// Directed fault applied after the fixture computes status, before it can publish a reply.
    #[cfg(test)]
    change_generation_after_dispatch: bool,
}

/// Which observation must refuse the original real token in a directed control.
#[cfg(test)]
#[derive(Clone, Copy)]
pub(super) enum RequestGenerationFault {
    /// Refuse before the one-shot status receiver is issued.
    BeforeDispatch,
    /// Refuse after the status decision, before the reply can escape the receiver.
    AfterDispatch,
}

impl AttributedStatusRead {
    /// Borrow the completed prefix while its captured token is still current.
    ///
    /// # Errors
    /// Preserve the token refusal and completed extent without exposing the bytes.
    pub(super) fn current_bytes(&self) -> Result<&[u8], IdentifiedReadError> {
        self.require_current()?;
        Ok(self.bytes.as_bytes())
    }

    /// Require the same token generation before releasing the read-only status result.
    ///
    /// # Errors
    /// Preserve the token refusal and completed extent without releasing any reply.
    pub(super) fn finish_dispatch(&mut self) -> Result<(), IdentifiedReadError> {
        #[cfg(test)]
        if std::mem::take(&mut self.change_generation_after_dispatch) {
            self.token.change_expected_generation_for_test();
        }
        self.require_current()
    }

    /// Check the original token and preserve the byte count on refusal.
    ///
    /// # Errors
    /// Preserve the first reobservation failure.
    fn require_current(&self) -> Result<(), IdentifiedReadError> {
        self.token
            .reobserve()
            .map_err(|cause: PipeIdentityError| IdentifiedReadError::Identity {
                cause,
                received: self.bytes.as_bytes().len(),
            })
    }

    /// Challenge a receiving guard while retaining the real completed read.
    #[cfg(test)]
    pub(super) fn fault_generation_for_test(&mut self, fault: RequestGenerationFault) {
        match fault {
            RequestGenerationFault::BeforeDispatch => {
                self.token.change_expected_generation_for_test();
            }
            RequestGenerationFault::AfterDispatch => {
                self.change_generation_after_dispatch = true;
            }
        }
    }
}

/// A nonwaiting observation of the submitted native operation.
pub(super) enum NativeReadPoll {
    /// Windows can still access the original operation storage.
    Pending,
    /// Windows completed the read with an observed transfer count.
    Data(NativeReadBytes),
    /// Windows reports that cancellation completed the operation.
    Cancelled,
    /// Windows reports the peer closed the pipe without a read result.
    Disconnected,
}

/// A terminal read after the owner closed admission. No frame bytes escape.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum RetiredRead {
    /// The operation won the cancellation race. Its bytes were discarded.
    Discarded {
        /// Exact transfer count established by the retired native operation.
        len: usize,
    },
    /// Cancellation ended the operation.
    Cancelled,
    /// The peer closed the pipe.
    Disconnected,
}

/// Result of requesting cancellation of the occupied slot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum NativeCancel {
    /// Windows accepted the cancellation request.
    Requested,
    /// Windows found no matching operation to cancel.
    NotFound,
}

/// Refusal at the actual native identification edge.
#[derive(Debug)]
pub(super) enum IdentifiedReadError {
    /// The operation itself did not produce a trusted terminal read.
    Transport(NativeReadError),
    /// The native client token could not be obtained or validated.
    Identity {
        /// Original token capture or validation failure.
        cause: PipeIdentityError,
        /// Exact completed length. No bytes escape this refusal.
        received: usize,
    },
    /// Component observation on the original fixture client was refused.
    Component {
        /// Original component or continuity refusal.
        cause: FixtureClientError,
        /// Exact completed length. No bytes escape this refusal.
        received: usize,
    },
    /// The consumed bytes and their captured context are inadmissible.
    Unattributed,
    /// An earlier identity refusal permanently closed this reader.
    Retired,
}

impl std::fmt::Display for IdentifiedReadError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transport(cause) => write!(formatter, "native read refused: {cause:?}"),
            Self::Identity { cause, received } => {
                write!(
                    formatter,
                    "pipe identity refused after {received} bytes: {cause:?}"
                )
            }
            Self::Component { cause, received } => {
                write!(
                    formatter,
                    "fixture component refused after {received} bytes: {cause}"
                )
            }
            Self::Unattributed => {
                formatter.write_str("pipe fragment has no matching client identity")
            }
            Self::Retired => formatter.write_str("identified pipe reader is retired"),
        }
    }
}

/// One inbound reader that releases bytes only after capturing their client context.
pub(super) struct IdentifiedRequestRead {
    /// Exact ingress server and original native completion owner.
    read: NativePipeRead,
    /// The original request client paired at native construction.
    writer: LabRequestWriter,
    /// At most one client write can follow each admitted, queued read.
    client_write_available: bool,
    /// Required local fixture principal and logon identity.
    expected: PipeIdentity,
    /// Permanent refusal after admission or attribution is lost.
    retired: bool,
}

impl IdentifiedRequestRead {
    /// Create an idle reader with the required identity selected by fixture composition.
    /// # Errors
    /// Refuses failure to acquire the original completion event.
    pub(super) fn new(
        pipe: LabIdentificationPipe,
        expected: PipeIdentity,
    ) -> Result<Self, NativeReadError> {
        let (reader, writer): (LabPipeReader, LabRequestWriter) = pipe.into_transport();
        Ok(Self {
            read: NativePipeRead::new(reader)?,
            writer,
            client_write_available: false,
            expected,
            retired: false,
        })
    }

    /// Verify both original handles before the logical submission guard.
    /// # Errors
    /// Refuses unavailable or foreign same-process peer observations.
    pub(super) fn check_peers(&self) -> Result<(), LabPipeError> {
        let ReadPipe::Paired(reader) = &self.read.reader else {
            return Err(LabPipeError::ForeignPeer);
        };
        check_request_peers(reader, &self.writer)
    }

    /// Return the extent that may still accept one paired client write.
    pub(super) const fn client_write_span(&self) -> Option<u32> {
        if self.client_write_available {
            self.read.record.outstanding
        } else {
            None
        }
    }

    /// Write from the paired client. The fixture caller owns current authorization.
    /// The anonymous control deliberately uses this raw entry to exercise refusal.
    /// # Errors
    /// Refuses a used slot or wrong extent. Once admitted, preserves native or short-write failure.
    pub(super) fn write_from_client(&mut self, bytes: &[u8]) -> Result<(), LabPipeError> {
        let count: u32 = u32::try_from(bytes.len()).map_err(|_error| LabPipeError::WriteSlot)?;
        if self.client_write_span() != Some(count) {
            return Err(LabPipeError::WriteSlot);
        }
        self.client_write_available = false;
        self.writer.write(bytes)
    }

    /// Admit one bounded native fragment while the reader can still authenticate it.
    /// # Errors
    /// Refuses a retired reader, invalid extent, occupied slot or native submission.
    pub(super) fn begin(&mut self, span: usize) -> Result<NativeReadStart, IdentifiedReadError> {
        if self.retired {
            return Err(IdentifiedReadError::Retired);
        }
        let started: NativeReadStart = self
            .read
            .begin(span)
            .map_err(IdentifiedReadError::Transport)?;
        self.client_write_available = started == NativeReadStart::Queued;
        Ok(started)
    }

    /// Capture context of newly completed bytes before releasing them or reading again.
    /// # Errors
    /// Refuses native failure, empty completion or incomplete or mismatched identity.
    pub(super) fn poll(&mut self) -> Result<NativeReadPoll, IdentifiedReadError> {
        if self.retired {
            return Err(IdentifiedReadError::Retired);
        }
        let completion: NativeReadPoll =
            self.read.poll().map_err(IdentifiedReadError::Transport)?;
        match completion {
            NativeReadPoll::Pending => Ok(completion),
            NativeReadPoll::Data(ref completed) => {
                // Retire by default until this exact fragment's identity is established.
                self.retired = true;
                drop(self.read.require_client(completed, &self.expected)?);
                self.retired = false;
                Ok(completion)
            }
            NativeReadPoll::Cancelled | NativeReadPoll::Disconnected => {
                self.retired = true;
                Ok(completion)
            }
        }
    }

    /// Close admission before requesting cancellation of the original operation.
    /// # Errors
    /// Preserves the native cancellation error and retains occupied custody.
    pub(super) fn retire(&mut self) -> Result<NativeCancel, NativeReadError> {
        self.retired = true;
        self.client_write_available = false;
        self.read.request_cancel()
    }

    /// Collect physical completion after the Stop fence, without client impersonation.
    /// # Errors
    /// Preserves completion refusal and leaves an unconfirmed operation occupied.
    pub(super) fn poll_retirement(&mut self) -> Result<Poll<RetiredRead>, NativeReadError> {
        self.retired = true;
        self.client_write_available = false;
        Ok(match self.read.poll()? {
            NativeReadPoll::Pending => Poll::Pending,
            NativeReadPoll::Data(completed) => Poll::Ready(RetiredRead::Discarded {
                len: completed.as_bytes().len(),
            }),
            NativeReadPoll::Cancelled => Poll::Ready(RetiredRead::Cancelled),
            NativeReadPoll::Disconnected => Poll::Ready(RetiredRead::Disconnected),
        })
    }

    /// Wait for the original operation to signal while leaving its result uncollected.
    /// # Errors
    /// Refuses an idle slot and preserves timeout or native wait failure.
    #[expect(
        unsafe_code,
        reason = "Observe the original owned read event without collecting bytes"
    )]
    pub(super) fn wait_for_read_signal(&self) -> std::io::Result<()> {
        if !self.read.is_pending() {
            return Err(std::io::Error::other("no outstanding native read"));
        }
        // SAFETY: The borrow keeps the private manual-reset event alive for this
        // finite wait. Waiting does not access the kernel-owned read storage.
        let result: WAIT_EVENT =
            unsafe { WaitForSingleObject(self.read.event.as_raw_handle(), 2_000) };
        match result {
            WAIT_OBJECT_0 => Ok(()),
            WAIT_TIMEOUT => Err(std::io::ErrorKind::TimedOut.into()),
            WAIT_FAILED => Err(std::io::Error::last_os_error()),
            code => Err(std::io::Error::other(format!(
                "unexpected native read wait result: {code}"
            ))),
        }
    }

    /// Observe whether completion still needs to be collected.
    pub(super) const fn is_pending(&self) -> bool {
        self.read.is_pending()
    }
}

/// Stable storage accessed by Win32 from submission through actual completion.
struct ReadStorage {
    /// Kernel-written operation record.
    overlapped: UnsafeCell<OVERLAPPED>,
    /// Kernel-written read buffer with a fixed initialized extent.
    bytes: UnsafeCell<[u8; MAX_FRAME_LEN]>,
    /// Keep the address stable even if the owner itself moves.
    _pin: PhantomPinned,
}

/// Exact handle owner selected at construction and kept through completion.
enum ReadPipe {
    /// The existing request or reply fixture, with optional paired identity.
    Paired(LabPipeReader),
    /// The broker worker's original accepted server after fixture-only arming.
    FirstRead(OwnedHandle),
}

impl AsHandle for ReadPipe {
    /// Borrow the same handle at every call while the slot is occupied.
    fn as_handle(&self) -> BorrowedHandle<'_> {
        match self {
            Self::Paired(pipe) => pipe.as_handle(),
            Self::FirstRead(pipe) => pipe.as_handle(),
        }
    }
}

/// State and memory whose destructor must run before the pipe and event.
struct ReadRecord {
    /// Pinned allocation reused only after a terminal result.
    storage: Pin<Box<ReadStorage>>,
    /// Requested extent of the operation with uncollected completion.
    outstanding: Option<u32>,
    /// Scheduling hint attached to this exact submission.
    wake: CompletionWakeState,
}

/// One laboratory read slot owning the pipe, completion event and storage.
pub(super) struct NativePipeRead {
    /// Outstanding pointers must be fenced before field destruction releases handles.
    record: ReadRecord,
    /// The privately selected overlapped endpoint.
    reader: ReadPipe,
    /// Noninherited manual-reset event bound to this slot only.
    event: OwnedHandle,
}

impl AsHandle for NativePipeRead {
    /// Borrow the original handle without changing its read or write custody.
    fn as_handle(&self) -> BorrowedHandle<'_> {
        self.pipe()
    }
}

impl NativePipeRead {
    /// Match the retained writer to this reader before starting or publishing a reply.
    /// # Errors
    /// Refuses a foreign pair or unavailable or foreign native peer identity.
    pub(super) fn check_reply_writer(
        &self,
        writer: &LabOverlappedWriter,
    ) -> Result<(), LabPipeError> {
        match &self.reader {
            ReadPipe::Paired(reader) => writer.check_reply_reader(reader),
            ReadPipe::FirstRead(_) => Err(LabPipeError::ForeignReplyPair),
        }
    }

    /// Retain the exact connected paired-fixture reader.
    ///
    /// # Errors
    /// Returns event creation failure and drops the idle endpoint.
    pub(super) fn new(reader: LabPipeReader) -> Result<Self, NativeReadError> {
        Self::prepare(ReadPipe::Paired(reader))
    }

    /// Keep the fixture's only server handle in the prepared read slot through accept.
    ///
    /// # Errors
    /// Returns event creation failure and drops the unsubmitted endpoint.
    pub(super) fn for_first_read(pipe: LabUnconnectedDuplexPipe) -> Result<Self, NativeReadError> {
        Self::prepare(ReadPipe::FirstRead(pipe.into_handle()))
    }

    /// Collect the original read and lend the successful prefix with its origin.
    ///
    /// # Errors
    /// An unconfirmed result retains the native operation. An invalid terminal
    /// count retires that operation without lending any bytes.
    pub(super) fn collect_first_read(
        &mut self,
        data: impl FnOnce(CompletedFirstRead<'_>),
    ) -> Result<Poll<()>, NativeReadError> {
        match self.poll()? {
            NativeReadPoll::Pending => Ok(Poll::Pending),
            NativeReadPoll::Data(bytes) => {
                data(CompletedFirstRead {
                    origin: self,
                    bytes,
                });
                Ok(Poll::Ready(()))
            }
            NativeReadPoll::Cancelled | NativeReadPoll::Disconnected => Ok(Poll::Ready(())),
        }
    }

    /// Compare the client from this exact nonempty read after successful reversion.
    ///
    /// # Errors
    /// Retains the first native identity refusal and the actual received count.
    fn require_client(
        &self,
        completed: &NativeReadBytes,
        expected: &PipeIdentity,
    ) -> Result<CapturedRequestToken, IdentifiedReadError> {
        let received: usize = completed.as_bytes().len();
        if received == 0 {
            return Err(IdentifiedReadError::Unattributed);
        }
        let token: CapturedRequestToken =
            capture(self.reader.as_handle()).map_err(|cause: PipeIdentityError| {
                IdentifiedReadError::Identity { cause, received }
            })?;
        if token.identity() != expected {
            return Err(IdentifiedReadError::Unattributed);
        }
        Ok(token)
    }

    /// Borrow the original handle for the accept and eventual read.
    pub(super) fn pipe(&self) -> BorrowedHandle<'_> {
        self.reader.as_handle()
    }

    /// Consume settled, armed resources for explicit fallible endpoint release.
    ///
    /// A charged record aborts before either handle can be released.
    pub(super) fn into_first_read_handles(self) -> Option<(OwnedHandle, OwnedHandle)> {
        let Self {
            record,
            reader,
            event,
        } = self;
        drop(record);
        match reader {
            ReadPipe::FirstRead(pipe) => Some((pipe, event)),
            ReadPipe::Paired(_) => None,
        }
    }

    /// Lend this operation's exact event, preserving its one-shot hint through cancellation.
    pub(super) fn completion_wake(&mut self) -> Option<PendingCompletionWake<'_>> {
        self.record.outstanding?;
        Some(self.record.wake.lend(self.event.as_handle()))
    }

    /// Create an initially idle slot for the exact owned pipe.
    /// # Errors
    /// Returns the event creation error and drops the unused endpoint.
    #[expect(
        unsafe_code,
        reason = "Create and own a private Win32 completion event"
    )]
    fn prepare(reader: ReadPipe) -> Result<Self, NativeReadError> {
        // SAFETY: NULL security attributes and name select a noninheritable,
        // unnamed event. The other arguments select manual reset and unsignaled.
        let handle: HANDLE = unsafe { CreateEventW(null(), 1, 0, null()) };
        if handle.is_null() {
            return Err(NativeReadError::Event(last_error()));
        }
        // SAFETY: CreateEventW returned one unaliased, caller-owned handle.
        let event: OwnedHandle = unsafe { OwnedHandle::from_raw_handle(handle) };
        Ok(Self {
            reader,
            event,
            record: ReadRecord {
                storage: Box::pin(ReadStorage {
                    overlapped: UnsafeCell::new(OVERLAPPED::default()),
                    bytes: UnsafeCell::new([0; MAX_FRAME_LEN]),
                    _pin: PhantomPinned,
                }),
                outstanding: None,
                wake: CompletionWakeState::new(),
            },
        })
    }

    /// Issue one nonempty read within the fixed capacity using this slot.
    /// # Errors
    /// Refuses invalid extent, occupied custody or a native submission failure.
    pub(super) fn begin(&mut self, span: usize) -> Result<NativeReadStart, NativeReadError> {
        self.begin_guarded(span, || Ok(()))
    }

    /// Check the owner after preparing the stable record and before submitting the read.
    /// # Errors
    /// A guard refusal leaves the prepared record idle. Other errors retain the native cause.
    #[expect(
        unsafe_code,
        reason = "Submit a bounded read with stable owned buffers and event"
    )]
    pub(super) fn begin_guarded<E: From<NativeReadError>>(
        &mut self,
        span: usize,
        guard: impl FnOnce() -> Result<(), E>,
    ) -> Result<NativeReadStart, E> {
        if self.record.outstanding.is_some() {
            return Err(NativeReadError::Busy.into());
        }
        if !(1..=MAX_FRAME_LEN).contains(&span) {
            return Err(NativeReadError::Size.into());
        }
        let span: u32 = u32::try_from(span).map_err(|_| NativeReadError::Size)?;
        let slot: &ReadStorage = self.record.storage.as_ref().get_ref();
        // SAFETY: The prior operation has completed or the slot is unused.
        // This record is uniquely writable and the event outlives submission.
        unsafe {
            slot.overlapped.get().write(OVERLAPPED {
                hEvent: self.event.as_raw_handle(),
                ..OVERLAPPED::default()
            });
        };
        self.record.wake = CompletionWakeState::new();
        let pipe: BorrowedHandle<'_> = self.reader.as_handle();
        let buffer: *mut u8 = slot.bytes.get().cast::<u8>();
        let overlapped: *mut OVERLAPPED = slot.overlapped.get();
        guard()?;
        self.record.outstanding = Some(span);
        // SAFETY: The private reader uses FILE_FLAG_OVERLAPPED. The
        // kernel-written pointers remain live and pinned. The fixed buffer
        // holds span writable bytes and the count output is collected by poll.
        let result: i32 =
            unsafe { ReadFile(pipe.as_raw_handle(), buffer, span, null_mut(), overlapped) };
        if result != 0 {
            return Ok(NativeReadStart::Immediate);
        }
        let code: u32 = last_error();
        if code == ERROR_IO_PENDING {
            return Ok(NativeReadStart::Queued);
        }
        // Byte-mode submission has no partial-message failure result.
        self.record.outstanding = None;
        Err(NativeReadError::Submit(code).into())
    }

    /// Observe native completion without waiting or releasing pending storage.
    /// # Errors
    /// Refuses missing submission, unconfirmed completion or an invalid count.
    #[expect(
        unsafe_code,
        reason = "Observe exact Win32 read completion before accessing its output"
    )]
    pub(super) fn poll(&mut self) -> Result<NativeReadPoll, NativeReadError> {
        let span: u32 = self.record.outstanding.ok_or(NativeReadError::Idle)?;
        let slot: &ReadStorage = self.record.storage.as_ref().get_ref();
        let mut actual: u32 = 0;
        // SAFETY: The occupied slot retains the exact pipe, record and event
        // from ReadFile. The count output is an exclusive local. FALSE is nonwaiting.
        let complete: i32 = unsafe {
            GetOverlappedResult(
                self.reader.as_handle().as_raw_handle(),
                slot.overlapped.get(),
                &raw mut actual,
                0,
            )
        };
        if complete != 0 {
            self.record.outstanding = None;
            if actual > span {
                return Err(NativeReadError::Count);
            }
            let len: usize = usize::try_from(actual).map_err(|_| NativeReadError::Count)?;
            // SAFETY: Windows reported successful completion before this
            // copy. The whole buffer was initialized at allocation.
            let bytes: [u8; MAX_FRAME_LEN] = unsafe { *slot.bytes.get() };
            return Ok(NativeReadPoll::Data(NativeReadBytes {
                bytes,
                len,
                reply_pair: match &self.reader {
                    ReadPipe::Paired(reader) => reader.reply_pair().cloned(),
                    ReadPipe::FirstRead(_) => None,
                },
            }));
        }
        match last_error() {
            ERROR_IO_INCOMPLETE => Ok(NativeReadPoll::Pending),
            ERROR_OPERATION_ABORTED => {
                self.record.outstanding = None;
                Ok(NativeReadPoll::Cancelled)
            }
            ERROR_BROKEN_PIPE => {
                self.record.outstanding = None;
                Ok(NativeReadPoll::Disconnected)
            }
            code => Err(NativeReadError::Completion(code)),
        }
    }

    /// Request cancellation of the exact submitted operation and retain custody.
    /// # Errors
    /// Refuses missing submission or reports a cancellation request failure.
    #[expect(
        unsafe_code,
        reason = "Target cancellation at the exact still-occupied overlapped record"
    )]
    pub(super) fn request_cancel(&mut self) -> Result<NativeCancel, NativeReadError> {
        if self.record.outstanding.is_none() {
            return Err(NativeReadError::Idle);
        }
        let slot: &ReadStorage = self.record.storage.as_ref().get_ref();
        // SAFETY: This is the exact record submitted on the still-owned pipe.
        // Cancellation preserves ownership of every associated resource.
        let accepted: i32 = unsafe {
            CancelIoEx(
                self.reader.as_handle().as_raw_handle(),
                slot.overlapped.get(),
            )
        };
        if accepted != 0 {
            return Ok(NativeCancel::Requested);
        }
        match last_error() {
            ERROR_NOT_FOUND => Ok(NativeCancel::NotFound),
            code => Err(NativeReadError::Cancel(code)),
        }
    }

    /// Whether terminal completion still needs to be collected.
    pub(super) const fn is_pending(&self) -> bool {
        self.record.outstanding.is_some()
    }
}

impl Drop for ReadRecord {
    /// Protect pending pointers even when a laboratory check fails or unwinds.
    fn drop(&mut self) {
        if self.outstanding.is_some() {
            std::process::abort();
        }
    }
}

/// Read the thread-local Win32 error immediately after a failing native call.
#[expect(unsafe_code, reason = "Read this thread's native failure code")]
fn last_error() -> u32 {
    // SAFETY: GetLastError takes no arguments and has thread-local semantics.
    unsafe { GetLastError() }
}

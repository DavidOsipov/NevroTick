// SPDX-License-Identifier: GPL-2.0-only
//! One event-backed overlapped write on an owned, process-local laboratory pipe.

use std::{
    cell::{Cell, UnsafeCell},
    marker::PhantomPinned,
    os::windows::io::{AsHandle, AsRawHandle, BorrowedHandle, FromRawHandle, OwnedHandle},
    pin::Pin,
    ptr::null,
    task::Poll,
    time::{Duration, Instant},
};

use windows_sys::Win32::{
    Foundation::{
        ERROR_BROKEN_PIPE, ERROR_IO_INCOMPLETE, ERROR_IO_PENDING, ERROR_NO_DATA, ERROR_NOT_FOUND,
        ERROR_OPERATION_ABORTED, ERROR_PIPE_NOT_CONNECTED, GetLastError, HANDLE,
    },
    Storage::FileSystem::WriteFile,
    System::{
        IO::{CancelIoEx, GetOverlappedResult, OVERLAPPED},
        Threading::CreateEventW,
    },
};

use crate::{
    native_broker_wait::{CompletionWakeState, PendingCompletionWake},
    native_pipe_pair::{LabFirstExchangeWriter, LabOverlappedWriter, LabPipeError, LabPipeReader},
    native_pipe_read::{
        CompletedFirstRead, NativeCancel, NativePipeRead, NativeReadBytes, NativeReadError,
        NativeReadStart,
    },
    private_clock_wire::MAX_FRAME_LEN,
};

/// Real-time bound on each fixture write observation and its cancellation drain.
const FIXTURE_WRITE_WAIT: Duration = Duration::from_secs(2);

/// Local custody or native call refusal, retaining the original Win32 code.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum NativeWriteError {
    /// The private completion event could not be acquired.
    Event(u32),
    /// A prior write still owns this slot.
    Busy,
    /// Write length is outside the fixed nonempty extent.
    Size,
    /// No submitted operation can produce a result.
    Idle,
    /// The requested write could not start.
    Submit(u32),
    /// Completion could not be established. The slot still owns the operation.
    Completion(u32),
    /// The completed count is zero or exceeds the submitted extent.
    Count,
    /// The cancellation request failed. The operation remains occupied.
    Cancel(u32),
}

/// Native submission outcome. Both paths require the completion observer.
#[must_use]
#[derive(Debug, Eq, PartialEq)]
pub(super) enum NativeWriteStart {
    /// Win32 completed the operation inline.
    Immediate,
    /// Win32 retained the supplied operation storage.
    Queued,
}

/// A nonwaiting observation of the submitted native operation.
#[must_use]
#[derive(Debug, Eq, PartialEq)]
pub(super) enum NativeWritePoll {
    /// Windows can still access the original operation storage.
    Pending,
    /// Windows completed the operation and reported this positive byte count.
    Written(usize),
    /// Windows reported terminal cancellation without a successful transfer count.
    Cancelled,
    /// Windows reported terminal pipe closure without a successful transfer count.
    Disconnected,
}

/// Stable storage accessed by Win32 from submission through actual completion.
struct WriteStorage {
    /// Kernel-written operation record.
    overlapped: UnsafeCell<OVERLAPPED>,
    /// Fixed input buffer. Only an idle slot may replace its contents.
    bytes: Cell<[u8; MAX_FRAME_LEN]>,
    /// Keep the address stable even if the owner itself moves.
    _pin: PhantomPinned,
}

/// The pinned record and its charge are fenced before handle destruction.
struct WriteRecord {
    /// Pinned allocation reused only after a terminal result.
    storage: Pin<Box<WriteStorage>>,
    /// Submitted byte count with uncollected completion.
    outstanding: Option<u32>,
    /// One scheduling hint for this same submission through cancellation.
    wake: CompletionWakeState,
}

/// One laboratory write slot owning the pipe, completion event and storage.
///
/// An unresolved drop terminates the supervised test process before Rust
/// can release memory still accessible to Windows.
pub(super) struct NativePipeWrite<W = LabOverlappedWriter> {
    /// The charge is checked before either native handle can be released.
    record: WriteRecord,
    /// The writer never changes while the slot owns a submission.
    writer: W,
    /// Noninherited manual-reset event bound to this slot only.
    event: OwnedHandle,
}

impl NativePipeWrite {
    /// Check the peer of the retained writer without replacing its native slot.
    /// # Errors
    /// Refuses a different pair or failed native peer observations.
    pub(super) fn check_reply_reader(&self, reader: &NativePipeRead) -> Result<(), LabPipeError> {
        reader.check_reply_writer(&self.writer)
    }

    /// Check the request reader on the opposite side of this exact duplex writer.
    /// # Errors
    /// Refuses a foreign direction or failed current native peer query.
    pub(super) fn check_pipe_reader(&self, reader: &LabPipeReader) -> Result<(), LabPipeError> {
        self.writer.check_reply_reader(reader)
    }

    /// Match bytes from a completed physical read to the retained writer.
    /// # Errors
    /// Refuses completed bytes from a foreign or non-reply pipe.
    pub(super) fn check_reply_completion(
        &self,
        read: &NativeReadBytes,
    ) -> Result<(), LabPipeError> {
        self.writer.require_reply_pair(read.reply_pair())
    }

    /// Create an idle slot for the original paired fixture writer.
    /// # Errors
    /// Returns the event creation error and drops the unused endpoint.
    pub(super) fn new(writer: LabOverlappedWriter) -> Result<Self, NativeWriteError> {
        Self::prepare(writer)
    }
}

impl NativePipeWrite<LabFirstExchangeWriter> {
    /// Create the write slot for the first exchange's original fixture client.
    /// # Errors
    /// Returns event creation failure without submitting native work.
    pub(super) fn for_first_exchange_client(
        client: LabFirstExchangeWriter,
    ) -> Result<Self, NativeWriteError> {
        Self::prepare(client)
    }

    /// Borrow the original first-exchange client for reply attribution.
    pub(super) fn client(&self) -> BorrowedHandle<'_> {
        self.writer.as_handle()
    }
}

impl NativePipeWrite<NativePipeRead> {
    /// Keep the original server in its read owner throughout the first exchange.
    /// # Errors
    /// Returns the event creation error and drops the idle read owner.
    pub(super) fn for_first_reply(reader: NativePipeRead) -> Result<Self, NativeWriteError> {
        Self::prepare(reader)
    }

    /// Borrow the original server handle through its single owner.
    pub(super) fn pipe(&self) -> BorrowedHandle<'_> {
        self.writer.pipe()
    }

    /// Submit a read on the original server before a reply occupies the slot.
    /// # Errors
    /// Preserve the read refusal or refuse an outstanding write.
    pub(super) fn begin_first_read(
        &mut self,
        extent: usize,
    ) -> Result<NativeReadStart, NativeReadError> {
        self.idle_reader()?.begin(extent)
    }

    /// Collect and lend the first read without transferring its pipe owner.
    /// # Errors
    /// Preserve the read refusal or refuse an outstanding write.
    pub(super) fn collect_first_read(
        &mut self,
        data: impl FnOnce(CompletedFirstRead<'_>),
    ) -> Result<Poll<()>, NativeReadError> {
        self.idle_reader()?.collect_first_read(data)
    }

    /// Target the first read's original operation on the retained server.
    /// # Errors
    /// Preserve the cancellation refusal or refuse an outstanding write.
    pub(super) fn request_first_read_cancel(&mut self) -> Result<NativeCancel, NativeReadError> {
        self.idle_reader()?.request_cancel()
    }

    /// Borrow the read slot privately so a caller cannot replace its original pipe.
    /// # Errors
    /// Refuses mutable access while this write still owns its native record.
    fn idle_reader(&mut self) -> Result<&mut NativePipeRead, NativeReadError> {
        if self.is_pending() {
            return Err(NativeReadError::Busy);
        }
        Ok(&mut self.writer)
    }

    /// Lend only the charged operation's original completion hint.
    pub(super) fn completion_wake(&mut self) -> Option<PendingCompletionWake<'_>> {
        if self.record.outstanding.is_some() {
            return Some(self.record.wake.lend(self.event.as_handle()));
        }
        self.writer.completion_wake()
    }

    /// Verify the write charge before returning the original read owner and write event.
    pub(super) fn into_first_reply_parts(self) -> (NativePipeRead, OwnedHandle) {
        let Self {
            record,
            writer,
            event,
        } = self;
        drop(record);
        (writer, event)
    }
}

impl<W: AsHandle> NativePipeWrite<W> {
    /// Create an initially idle slot around a privately selected pipe owner.
    /// # Errors
    /// Returns the event creation error and drops the unused endpoint.
    #[expect(
        unsafe_code,
        reason = "Create and own a private Win32 completion event"
    )]
    fn prepare(writer: W) -> Result<Self, NativeWriteError> {
        // SAFETY: NULL security attributes and name select a noninheritable,
        // unnamed event. The other arguments select manual reset and unsignaled.
        let handle: HANDLE = unsafe { CreateEventW(null(), 1, 0, null()) };
        if handle.is_null() {
            return Err(NativeWriteError::Event(last_error()));
        }
        // SAFETY: CreateEventW returned one unaliased, caller-owned handle.
        let event: OwnedHandle = unsafe { OwnedHandle::from_raw_handle(handle) };
        Ok(Self {
            record: WriteRecord {
                storage: Box::pin(WriteStorage {
                    overlapped: UnsafeCell::new(OVERLAPPED::default()),
                    bytes: Cell::new([0; MAX_FRAME_LEN]),
                    _pin: PhantomPinned,
                }),
                outstanding: None,
                wake: CompletionWakeState::new(),
            },
            writer,
            event,
        })
    }

    /// Copy and submit one nonempty write within the fixed capacity.
    /// The caller's original bytes are free for reuse after return.
    /// # Errors
    /// Refuses invalid extent, occupied custody or a native submission failure.
    pub(super) fn begin(&mut self, bytes: &[u8]) -> Result<NativeWriteStart, NativeWriteError> {
        self.begin_guarded(bytes, |_| Ok(()))
    }

    /// Revalidate the exact borrowed writer after preparing its stable slot.
    /// # Errors
    /// Retains extent, custody, guard and native failures. A guard refusal leaves the slot idle.
    #[expect(
        unsafe_code,
        reason = "Submit a bounded write with stable owned input, record and event"
    )]
    pub(super) fn begin_guarded<E: From<NativeWriteError>>(
        &mut self,
        bytes: &[u8],
        guard: impl FnOnce(BorrowedHandle<'_>) -> Result<(), E>,
    ) -> Result<NativeWriteStart, E> {
        if self.record.outstanding.is_some() {
            return Err(NativeWriteError::Busy.into());
        }
        if !(1..=MAX_FRAME_LEN).contains(&bytes.len()) {
            return Err(NativeWriteError::Size.into());
        }
        let span: u32 = u32::try_from(bytes.len()).map_err(|_| NativeWriteError::Size)?;
        let mut owned_bytes: [u8; MAX_FRAME_LEN] = [0; MAX_FRAME_LEN];
        owned_bytes[..bytes.len()].copy_from_slice(bytes);
        let slot: &WriteStorage = self.record.storage.as_ref().get_ref();
        slot.bytes.set(owned_bytes);
        // SAFETY: The prior operation has completed or the slot is unused.
        // This record is uniquely writable and the event outlives submission.
        unsafe {
            slot.overlapped.get().write(OVERLAPPED {
                hEvent: self.event.as_raw_handle(),
                ..OVERLAPPED::default()
            });
        };
        self.record.wake = CompletionWakeState::new();
        let pipe: BorrowedHandle<'_> = self.writer.as_handle();
        let buffer: *const u8 = slot.bytes.as_ptr().cast_const().cast::<u8>();
        let overlapped: *mut OVERLAPPED = slot.overlapped.get();
        guard(pipe)?;
        self.record.outstanding = Some(span);
        // SAFETY: The private constructors own an overlapped pipe. The slot
        // pins its initialized input and operation record and holds the exact
        // event and pipe owner. begin cannot replace occupied storage.
        let result: i32 = unsafe {
            WriteFile(
                pipe.as_raw_handle(),
                buffer,
                span,
                std::ptr::null_mut(),
                overlapped,
            )
        };
        if result != 0 {
            return Ok(NativeWriteStart::Immediate);
        }
        let code: u32 = last_error();
        if code == ERROR_IO_PENDING {
            return Ok(NativeWriteStart::Queued);
        }
        self.record.outstanding = None;
        Err(NativeWriteError::Submit(code).into())
    }

    /// Observe native completion without waiting or releasing pending storage.
    /// # Errors
    /// Refuses missing submission, unconfirmed completion or an invalid count.
    #[expect(
        unsafe_code,
        reason = "Observe exact Win32 write completion and physical byte count"
    )]
    pub(super) fn poll(&mut self) -> Result<NativeWritePoll, NativeWriteError> {
        let span: u32 = self.record.outstanding.ok_or(NativeWriteError::Idle)?;
        let slot: &WriteStorage = self.record.storage.as_ref().get_ref();
        let mut actual: u32 = 0;
        // SAFETY: The occupied slot retains the exact pipe, record and event
        // from WriteFile. The count output is an exclusive local. FALSE is nonwaiting.
        let complete: i32 = unsafe {
            GetOverlappedResult(
                self.writer.as_handle().as_raw_handle(),
                slot.overlapped.get(),
                &raw mut actual,
                0,
            )
        };
        if complete != 0 {
            return self.finish_count(span, actual);
        }
        match last_error() {
            ERROR_IO_INCOMPLETE => Ok(NativeWritePoll::Pending),
            ERROR_OPERATION_ABORTED => {
                self.record.outstanding = None;
                Ok(NativeWritePoll::Cancelled)
            }
            ERROR_BROKEN_PIPE | ERROR_NO_DATA | ERROR_PIPE_NOT_CONNECTED => {
                self.record.outstanding = None;
                Ok(NativeWritePoll::Disconnected)
            }
            code => Err(NativeWriteError::Completion(code)),
        }
    }

    /// Release the terminal slot and expose only a bounded physical count.
    /// # Errors
    /// A zero, oversized or unrepresentable count cannot establish delivery.
    fn finish_count(
        &mut self,
        span: u32,
        actual: u32,
    ) -> Result<NativeWritePoll, NativeWriteError> {
        self.record.outstanding = None;
        if !(1..=span).contains(&actual) {
            return Err(NativeWriteError::Count);
        }
        let written: usize = usize::try_from(actual).map_err(|_| NativeWriteError::Count)?;
        Ok(NativeWritePoll::Written(written))
    }

    /// Request cancellation of the exact submitted operation and retain custody.
    /// # Errors
    /// Refuses missing submission or reports a cancellation request failure.
    #[expect(
        unsafe_code,
        reason = "Target cancellation at the exact still-occupied overlapped record"
    )]
    pub(super) fn request_cancel(&mut self) -> Result<NativeCancel, NativeWriteError> {
        if self.record.outstanding.is_none() {
            return Err(NativeWriteError::Idle);
        }
        let slot: &WriteStorage = self.record.storage.as_ref().get_ref();
        // SAFETY: This is the exact record submitted on the still-owned pipe.
        // Cancellation preserves ownership of every associated resource.
        let accepted: i32 = unsafe {
            CancelIoEx(
                self.writer.as_handle().as_raw_handle(),
                slot.overlapped.get(),
            )
        };
        if accepted != 0 {
            return Ok(NativeCancel::Requested);
        }
        match last_error() {
            ERROR_NOT_FOUND => Ok(NativeCancel::NotFound),
            code => Err(NativeWriteError::Cancel(code)),
        }
    }

    /// Whether terminal completion still needs to be collected.
    pub(super) const fn is_pending(&self) -> bool {
        self.record.outstanding.is_some()
    }

    /// Satisfy one test sender's bounded write while its peer reads.
    /// # Errors
    /// Return the original failure after collecting the terminal result. An
    /// unresolved drain aborts the isolated fixture while it still owns the slot.
    pub(super) fn write_fixture_fragment(&mut self, bytes: &[u8]) -> Result<(), LabPipeError> {
        self.write_fixture_fragment_checked(bytes, None, |_| Ok(()))
    }

    /// Recheck the retained peer at submission and bound normal collection by the cutoff.
    /// # Errors
    /// Retains the typed guard, write or observation refusal through the separate drain.
    pub(super) fn write_fixture_fragment_before(
        &mut self,
        bytes: &[u8],
        cutoff: Instant,
        guard: impl FnOnce(BorrowedHandle<'_>) -> Result<(), LabPipeError>,
    ) -> Result<(), LabPipeError> {
        self.write_fixture_fragment_checked(bytes, Some(cutoff), guard)
    }

    /// Collect one fixture submission, draining its original slot after a timeout.
    /// # Errors
    /// Preserves the first refusal. Terminal cleanup remains separately bounded.
    fn write_fixture_fragment_checked(
        &mut self,
        bytes: &[u8],
        cutoff: Option<Instant>,
        guard: impl FnOnce(BorrowedHandle<'_>) -> Result<(), LabPipeError>,
    ) -> Result<(), LabPipeError> {
        let expected: u32 = u32::try_from(bytes.len()).map_err(|_| LabPipeError::WriteSize)?;
        let _submitted: NativeWriteStart = self.begin_guarded(bytes, guard)?;
        let terminal: NativeWritePoll = match self.wait_for_fixture_write(cutoff) {
            Ok(outcome) => outcome,
            Err(error) => {
                if self.is_pending() {
                    let cancellation: Result<NativeCancel, NativeWriteError> =
                        self.request_cancel();
                    let drained: Result<NativeWritePoll, LabPipeError> =
                        self.wait_for_fixture_write(None);
                    if self.is_pending() {
                        use std::io::Write;
                        let _diagnostic: std::io::Result<()> = writeln!(
                            std::io::stderr().lock(),
                            "fixture write still pending after {error}; cancel {cancellation:?}; drain {drained:?}",
                        );
                        std::process::abort();
                    }
                }
                return Err(error);
            }
        };
        confirm_fixture_write(terminal, expected, cutoff, Instant::now)
    }

    /// Collect the exact slot using the tighter absolute or relative observation bound.
    /// # Errors
    /// Preserve a native observation failure or expiry with the slot still owned.
    fn wait_for_fixture_write(
        &mut self,
        cutoff: Option<Instant>,
    ) -> Result<NativeWritePoll, LabPipeError> {
        let started: Instant = Instant::now();
        loop {
            let now: Instant = Instant::now();
            let mut remaining: Duration =
                FIXTURE_WRITE_WAIT.saturating_sub(now.saturating_duration_since(started));
            if let Some(cutoff) = cutoff {
                remaining = remaining.min(cutoff.saturating_duration_since(now));
            }
            if remaining.is_zero() {
                return Err(LabPipeError::WriteTimeout);
            }
            let outcome: NativeWritePoll = self.poll().map_err(LabPipeError::OverlappedWrite)?;
            if outcome != NativeWritePoll::Pending {
                return Ok(outcome);
            }
            std::thread::sleep(remaining.min(Duration::from_millis(1)));
        }
    }
}

/// Check physical count before admitting timely completion.
/// # Errors
/// Keeps terminal byte-count/transfer failures, or refuses a late exact count.
pub(super) fn confirm_fixture_write(
    terminal: NativeWritePoll,
    expected: u32,
    cutoff: Option<Instant>,
    now: impl FnOnce() -> Instant,
) -> Result<(), LabPipeError> {
    let NativeWritePoll::Written(count) = terminal else {
        return Err(LabPipeError::WriteTerminated(terminal));
    };
    let written: u32 = u32::try_from(count).map_err(|_| LabPipeError::WriteSize)?;
    if written != expected {
        return Err(LabPipeError::ShortWrite { expected, written });
    }
    if cutoff.is_some_and(|cutoff: Instant| now() >= cutoff) {
        return Err(LabPipeError::WriteTimeout);
    }
    Ok(())
}

impl Drop for WriteRecord {
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

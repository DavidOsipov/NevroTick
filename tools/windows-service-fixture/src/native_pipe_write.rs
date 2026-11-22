// SPDX-License-Identifier: GPL-2.0-only
//! One event-backed overlapped write on an owned, process-local laboratory pipe.

use std::{
    cell::{Cell, UnsafeCell},
    marker::PhantomPinned,
    os::windows::io::{AsHandle, AsRawHandle, FromRawHandle, OwnedHandle},
    pin::Pin,
    ptr::null,
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
    native_pipe_pair::{LabOverlappedWriter, LabPipeError},
    native_pipe_read::{NativeCancel, NativePipeRead, NativeReadBytes},
    private_clock_wire::MAX_FRAME_LEN,
};

/// Local custody or native call refusal, retaining the original Win32 code.
#[derive(Debug, Eq, PartialEq)]
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
    /// Only inspected when `WriteFile` reports inline success.
    inline_count: Cell<u32>,
    /// Fixed input buffer. Only an idle slot may replace its contents.
    bytes: Cell<[u8; MAX_FRAME_LEN]>,
    /// Keep the address stable even if the owner itself moves.
    _pin: PhantomPinned,
}

/// The occupied slot's uncollected result or queued physical extent.
#[derive(Clone, Copy)]
enum OutstandingWrite {
    /// `WriteFile` succeeded inline and supplied the transfer count.
    Inline { span: u32, actual: u32 },
    /// `WriteFile` returned `ERROR_IO_PENDING` for this extent.
    Queued { span: u32 },
}

/// One laboratory write slot owning the pipe, completion event and storage.
///
/// An unresolved drop terminates the supervised test process before Rust
/// can release memory still accessible to Windows.
pub(super) struct NativePipeWrite {
    /// The privately connected overlapped writer endpoint.
    writer: LabOverlappedWriter,
    /// Noninherited manual-reset event bound to this slot only.
    event: OwnedHandle,
    /// Pinned allocation reused only after a terminal result.
    storage: Pin<Box<WriteStorage>>,
    /// Submitted write with uncollected completion.
    outstanding: Option<OutstandingWrite>,
}

impl NativePipeWrite {
    /// Check the peer of the retained writer without replacing its native slot.
    /// # Errors
    /// Refuses a different pair or failed native peer observations.
    pub(super) fn check_reply_reader(&self, reader: &NativePipeRead) -> Result<(), LabPipeError> {
        reader.check_reply_writer(&self.writer)
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

    /// Create an initially idle slot for the exact owned pipe.
    /// # Errors
    /// Returns the event creation error and drops the unused endpoint.
    #[expect(
        unsafe_code,
        reason = "Create and own a private Win32 completion event"
    )]
    pub(super) fn new(writer: LabOverlappedWriter) -> Result<Self, NativeWriteError> {
        // SAFETY: NULL security attributes and name select a noninheritable,
        // unnamed event. The other arguments select manual reset and unsignaled.
        let handle: HANDLE = unsafe { CreateEventW(null(), 1, 0, null()) };
        if handle.is_null() {
            return Err(NativeWriteError::Event(last_error()));
        }
        // SAFETY: CreateEventW returned one unaliased, caller-owned handle.
        let event: OwnedHandle = unsafe { OwnedHandle::from_raw_handle(handle) };
        Ok(Self {
            writer,
            event,
            storage: Box::pin(WriteStorage {
                overlapped: UnsafeCell::new(OVERLAPPED::default()),
                inline_count: Cell::new(0),
                bytes: Cell::new([0; MAX_FRAME_LEN]),
                _pin: PhantomPinned,
            }),
            outstanding: None,
        })
    }

    /// Copy and submit one nonempty write within the fixed capacity.
    /// The caller's original bytes are free for reuse after return.
    /// # Errors
    /// Refuses invalid extent, occupied custody or a native submission failure.
    #[expect(
        unsafe_code,
        reason = "Submit a bounded write with stable owned input, record and event"
    )]
    pub(super) fn begin(&mut self, bytes: &[u8]) -> Result<NativeWriteStart, NativeWriteError> {
        if self.outstanding.is_some() {
            return Err(NativeWriteError::Busy);
        }
        if !(1..=MAX_FRAME_LEN).contains(&bytes.len()) {
            return Err(NativeWriteError::Size);
        }
        let span: u32 = u32::try_from(bytes.len()).map_err(|_| NativeWriteError::Size)?;
        let mut owned_bytes: [u8; MAX_FRAME_LEN] = [0; MAX_FRAME_LEN];
        owned_bytes[..bytes.len()].copy_from_slice(bytes);
        let slot: &WriteStorage = self.storage.as_ref().get_ref();
        slot.bytes.set(owned_bytes);
        slot.inline_count.set(0);
        // SAFETY: The prior operation has completed or the slot is unused.
        // This record is uniquely writable and the event outlives submission.
        unsafe {
            slot.overlapped.get().write(OVERLAPPED {
                hEvent: self.event.as_raw_handle(),
                ..OVERLAPPED::default()
            });
        };
        self.outstanding = Some(OutstandingWrite::Queued { span });
        // SAFETY: The private writer uses FILE_FLAG_OVERLAPPED. The slot pins
        // its initialized input, count output and operation record, and retains
        // the exact event and handle. begin cannot replace occupied storage.
        let result: i32 = unsafe {
            WriteFile(
                self.writer.as_handle().as_raw_handle(),
                slot.bytes.as_ptr().cast_const().cast(),
                span,
                slot.inline_count.as_ptr(),
                slot.overlapped.get(),
            )
        };
        if result != 0 {
            // The named-pipe API supplies this count on inline success. A queued
            // write instead obtains its count from GetOverlappedResult.
            self.outstanding = Some(OutstandingWrite::Inline {
                span,
                actual: slot.inline_count.get(),
            });
            return Ok(NativeWriteStart::Immediate);
        }
        let code: u32 = last_error();
        if code == ERROR_IO_PENDING {
            return Ok(NativeWriteStart::Queued);
        }
        self.outstanding = None;
        Err(NativeWriteError::Submit(code))
    }

    /// Observe native completion without waiting or releasing pending storage.
    /// # Errors
    /// Refuses missing submission, unconfirmed completion or an invalid count.
    #[expect(
        unsafe_code,
        reason = "Observe exact Win32 write completion and physical byte count"
    )]
    pub(super) fn poll(&mut self) -> Result<NativeWritePoll, NativeWriteError> {
        let span: u32 = match self.outstanding.ok_or(NativeWriteError::Idle)? {
            OutstandingWrite::Inline { span, actual } => return self.finish_count(span, actual),
            OutstandingWrite::Queued { span } => span,
        };
        let slot: &WriteStorage = self.storage.as_ref().get_ref();
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
                self.outstanding = None;
                Ok(NativeWritePoll::Cancelled)
            }
            ERROR_BROKEN_PIPE | ERROR_NO_DATA | ERROR_PIPE_NOT_CONNECTED => {
                self.outstanding = None;
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
        self.outstanding = None;
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
        if self.outstanding.is_none() {
            return Err(NativeWriteError::Idle);
        }
        let slot: &WriteStorage = self.storage.as_ref().get_ref();
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
        self.outstanding.is_some()
    }
}

impl Drop for NativePipeWrite {
    /// Protect pending pointers even when a laboratory check fails or unwinds.
    fn drop(&mut self) {
        if self.is_pending() {
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

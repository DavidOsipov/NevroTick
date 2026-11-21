// SPDX-License-Identifier: GPL-2.0-only
//! One event-backed overlapped read on an owned, process-local laboratory pipe.

use std::{
    cell::UnsafeCell,
    marker::PhantomPinned,
    os::windows::io::{AsHandle, AsRawHandle, FromRawHandle, OwnedHandle},
    pin::Pin,
    ptr::{null, null_mut},
};

use windows_sys::Win32::{
    Foundation::{
        ERROR_BROKEN_PIPE, ERROR_IO_INCOMPLETE, ERROR_IO_PENDING, ERROR_NOT_FOUND,
        ERROR_OPERATION_ABORTED, GetLastError, HANDLE,
    },
    Storage::FileSystem::ReadFile,
    System::{
        IO::{CancelIoEx, GetOverlappedResult, OVERLAPPED},
        Threading::CreateEventW,
    },
};

use crate::{native_pipe_pair::LabPipeReader, private_clock_wire::MAX_FRAME_LEN};

/// Local custody or native call refusal, retaining the original Win32 code.
#[derive(Debug, Eq, PartialEq)]
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
}

impl NativeReadBytes {
    /// Return only the bytes acknowledged by the completed read.
    pub(super) fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
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

/// Result of requesting cancellation of the occupied slot.
#[derive(Debug, Eq, PartialEq)]
pub(super) enum NativeCancel {
    /// Windows accepted the cancellation request.
    Requested,
    /// Windows found no matching operation to cancel.
    NotFound,
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

/// One laboratory read slot owning the pipe, completion event and storage.
///
/// An unresolved drop terminates the supervised test process before Rust
/// can release memory still accessible to Windows.
pub(super) struct NativePipeRead {
    /// The privately connected, read-only overlapped endpoint.
    reader: LabPipeReader,
    /// Noninherited manual-reset event bound to this slot only.
    event: OwnedHandle,
    /// Pinned allocation reused only after a terminal result.
    storage: Pin<Box<ReadStorage>>,
    /// Requested extent of the operation with uncollected completion.
    outstanding: Option<u32>,
}

impl NativePipeRead {
    /// Create an initially idle slot for the exact owned pipe.
    /// # Errors
    /// Returns the event creation error and drops the unused endpoint.
    #[expect(
        unsafe_code,
        reason = "Create and own a private Win32 completion event"
    )]
    pub(super) fn new(reader: LabPipeReader) -> Result<Self, NativeReadError> {
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
            storage: Box::pin(ReadStorage {
                overlapped: UnsafeCell::new(OVERLAPPED::default()),
                bytes: UnsafeCell::new([0; MAX_FRAME_LEN]),
                _pin: PhantomPinned,
            }),
            outstanding: None,
        })
    }

    /// Issue one nonempty read within the fixed capacity using this slot.
    /// # Errors
    /// Refuses invalid extent, occupied custody or a native submission failure.
    #[expect(
        unsafe_code,
        reason = "Submit a bounded read with stable owned buffers and event"
    )]
    pub(super) fn begin(&mut self, span: usize) -> Result<NativeReadStart, NativeReadError> {
        if self.outstanding.is_some() {
            return Err(NativeReadError::Busy);
        }
        if !(1..=MAX_FRAME_LEN).contains(&span) {
            return Err(NativeReadError::Size);
        }
        let span: u32 = u32::try_from(span).map_err(|_| NativeReadError::Size)?;
        let slot: &ReadStorage = self.storage.as_ref().get_ref();
        // SAFETY: The prior operation has completed or the slot is unused.
        // This record is uniquely writable and the event outlives submission.
        unsafe {
            slot.overlapped.get().write(OVERLAPPED {
                hEvent: self.event.as_raw_handle(),
                ..OVERLAPPED::default()
            });
        };
        self.outstanding = Some(span);
        // SAFETY: The private reader uses FILE_FLAG_OVERLAPPED. The
        // kernel-written pointers remain live and pinned. The fixed buffer
        // holds span writable bytes and the count output is collected by poll.
        let result: i32 = unsafe {
            ReadFile(
                self.reader.as_handle().as_raw_handle(),
                slot.bytes.get().cast(),
                span,
                null_mut(),
                slot.overlapped.get(),
            )
        };
        if result != 0 {
            return Ok(NativeReadStart::Immediate);
        }
        let code: u32 = last_error();
        if code == ERROR_IO_PENDING {
            return Ok(NativeReadStart::Queued);
        }
        // Byte-mode submission has no partial-message failure result.
        self.outstanding = None;
        Err(NativeReadError::Submit(code))
    }

    /// Observe native completion without waiting or releasing pending storage.
    /// # Errors
    /// Refuses missing submission, unconfirmed completion or an invalid count.
    #[expect(
        unsafe_code,
        reason = "Observe exact Win32 read completion before accessing its output"
    )]
    pub(super) fn poll(&mut self) -> Result<NativeReadPoll, NativeReadError> {
        let span: u32 = self.outstanding.ok_or(NativeReadError::Idle)?;
        let slot: &ReadStorage = self.storage.as_ref().get_ref();
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
            self.outstanding = None;
            if actual > span {
                return Err(NativeReadError::Count);
            }
            let len: usize = usize::try_from(actual).map_err(|_| NativeReadError::Count)?;
            // SAFETY: Windows reported successful completion before this
            // copy. The whole buffer was initialized at allocation.
            let bytes: [u8; MAX_FRAME_LEN] = unsafe { *slot.bytes.get() };
            return Ok(NativeReadPoll::Data(NativeReadBytes { bytes, len }));
        }
        match last_error() {
            ERROR_IO_INCOMPLETE => Ok(NativeReadPoll::Pending),
            ERROR_OPERATION_ABORTED => {
                self.outstanding = None;
                Ok(NativeReadPoll::Cancelled)
            }
            ERROR_BROKEN_PIPE => {
                self.outstanding = None;
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
        if self.outstanding.is_none() {
            return Err(NativeReadError::Idle);
        }
        let slot: &ReadStorage = self.storage.as_ref().get_ref();
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
        self.outstanding.is_some()
    }
}

impl Drop for NativePipeRead {
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

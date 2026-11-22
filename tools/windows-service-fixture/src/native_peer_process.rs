// SPDX-License-Identifier: GPL-2.0-only
//! Retain a candidate process object for the Windows connected-peer fixture.
//!
//! A checked PID associates the lookup with the retained handle. The caller owns
//! the pipe, token, service observation, and cutoff checks for each use.

use std::fmt;
use std::num::NonZeroU32;
use std::os::windows::io::{AsHandle, AsRawHandle, BorrowedHandle, FromRawHandle, OwnedHandle};

use windows_sys::Win32::Foundation::{
    CompareObjectHandles, GetLastError, HANDLE, WAIT_EVENT, WAIT_FAILED, WAIT_OBJECT_0,
    WAIT_TIMEOUT, WIN32_ERROR,
};
use windows_sys::Win32::System::Threading::{
    GetProcessId, OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_SYNCHRONIZE,
    WaitForSingleObject,
};
use windows_sys::core::BOOL;

/// Bounded failures while acquiring or observing the candidate process.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum PeerProcessError {
    /// Opening the queried process failed with this native error code.
    Open(WIN32_ERROR),
    /// Querying the retained handle's PID failed with this native error code.
    QueryId(WIN32_ERROR),
    /// The retained handle reported a different PID.
    WrongPid,
    /// The retained process object signaled termination.
    Exited,
    /// The zero-time wait failed with this native error code.
    Wait(WIN32_ERROR),
    /// The zero-time wait returned a result outside the process wait contract.
    UnexpectedWait(WAIT_EVENT),
}

impl fmt::Display for PeerProcessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open(code) => write!(formatter, "OpenProcess failed ({code})"),
            Self::QueryId(code) => write!(formatter, "GetProcessId failed ({code})"),
            Self::WrongPid => formatter.write_str("opened process reported a different PID"),
            Self::Exited => formatter.write_str("retained process exited"),
            Self::Wait(code) => write!(formatter, "WaitForSingleObject failed ({code})"),
            Self::UnexpectedWait(result) => write!(formatter, "unexpected process wait ({result})"),
        }
    }
}

impl std::error::Error for PeerProcessError {}

/// Own one query-and-synchronize process handle and its checked PID.
#[derive(Debug)]
pub(super) struct PeerProcess {
    /// The noninherited `OpenProcess` result, closed once by its owner on drop.
    handle: OwnedHandle,
    /// The requested nonzero PID, confirmed on the held handle.
    pid: NonZeroU32,
}

impl PeerProcess {
    /// Open and retain the candidate with query and synchronization rights.
    ///
    /// # Errors
    /// Preserve native open and PID-query failures. Refuse an unexpected PID.
    pub(super) fn open(pid: NonZeroU32) -> Result<Self, PeerProcessError> {
        #[expect(
            unsafe_code,
            reason = "retain a process with fixed query and wait rights"
        )]
        // SAFETY: The call accepts scalar access flags and a PID. Zero disables
        // inheritance. A nonnull result transfers one CloseHandle-managed owner.
        let raw: HANDLE = unsafe {
            OpenProcess(
                PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_SYNCHRONIZE,
                0,
                pid.get(),
            )
        };
        if raw.is_null() {
            return Err(PeerProcessError::Open(last_error()));
        }

        #[expect(unsafe_code, reason = "adopt the newly opened process handle")]
        // SAFETY: The nonnull OpenProcess result is uniquely owned and the API
        // returns a real handle that must be released with CloseHandle.
        let handle: OwnedHandle = unsafe { OwnedHandle::from_raw_handle(raw) };

        #[expect(unsafe_code, reason = "verify the PID from the owned process handle")]
        // SAFETY: The owned handle remains open and has query access.
        let observed: u32 = unsafe { GetProcessId(handle.as_raw_handle()) };
        if observed == 0 {
            return Err(PeerProcessError::QueryId(last_error()));
        }
        if observed != pid.get() {
            return Err(PeerProcessError::WrongPid);
        }
        Ok(Self { handle, pid })
    }

    /// Observe liveness of the retained object at a single instant.
    ///
    /// # Errors
    /// Report exit, the original native wait failure, or an unexpected result.
    pub(super) fn check_live(&self) -> Result<(), PeerProcessError> {
        #[expect(unsafe_code, reason = "poll the retained process without blocking")]
        // SAFETY: The borrow keeps the process handle open with synchronization
        // access. Zero milliseconds requests an immediate observation.
        let result: WAIT_EVENT = unsafe { WaitForSingleObject(self.handle.as_raw_handle(), 0) };
        match result {
            WAIT_TIMEOUT => Ok(()),
            WAIT_OBJECT_0 => Err(PeerProcessError::Exited),
            WAIT_FAILED => Err(PeerProcessError::Wait(last_error())),
            other => Err(PeerProcessError::UnexpectedWait(other)),
        }
    }

    /// Read the PID confirmed when the retained handle was opened.
    pub(super) const fn pid(&self) -> NonZeroU32 {
        self.pid
    }

    /// Compare against the independent process handle supplied by the fixture.
    pub(super) fn refers_to(&self, expected: BorrowedHandle<'_>) -> bool {
        same_kernel_object(self.as_handle(), expected)
    }
}

impl AsHandle for PeerProcess {
    fn as_handle(&self) -> BorrowedHandle<'_> {
        self.handle.as_handle()
    }
}

/// Observe equality of two borrowed kernel objects. Zero supplies no match.
pub(super) fn same_kernel_object(left: BorrowedHandle<'_>, right: BorrowedHandle<'_>) -> bool {
    #[expect(
        unsafe_code,
        reason = "compare two borrowed handles by their kernel objects"
    )]
    // SAFETY: Both handles stay borrowed through the call, which neither closes
    // the handles nor retains their raw values.
    let same: BOOL = unsafe { CompareObjectHandles(left.as_raw_handle(), right.as_raw_handle()) };
    same != 0
}

/// Capture a documented native failure before the process handle can close.
fn last_error() -> WIN32_ERROR {
    #[expect(unsafe_code, reason = "retain the failing operation's Win32 error")]
    // SAFETY: GetLastError takes no arguments and reads the calling thread's code.
    unsafe {
        GetLastError()
    }
}

// SPDX-License-Identifier: GPL-2.0-only
//! Retained process custody for native connected-peer observations.
//!
//! This adapter opens only limited query and synchronize access. Pipe, token,
//! service identity and the absolute admission cutoff belong to the caller.

use std::ffi::c_void;
use std::fmt;
use std::num::NonZeroU32;
use std::os::windows::io::{AsHandle, AsRawHandle, BorrowedHandle, FromRawHandle, OwnedHandle};

use crate::package_full_name::{
    PACKAGE_NAME_BUFFER, UNWRITTEN_NAME_UNIT, decode_package_full_name,
};
use crate::process_image_name::{PROCESS_IMAGE_NAME_BUFFER, read_process_image_name};
use crate::{
    PackageFullNameError, ProcessImageNameError, ReportedPackageFullName, ReportedProcessImageName,
};

/// The Win32 process right needed to observe process metadata and query the primary token.
const PROCESS_QUERY_LIMITED_INFORMATION: u32 = 0x1000;
/// The Win32 process right needed to poll the retained object for exit.
const PROCESS_SYNCHRONIZE: u32 = 0x0010_0000;
/// The process object became signaled.
const WAIT_OBJECT_0: u32 = 0;
/// The process object was unsignaled at the immediate poll.
const WAIT_TIMEOUT: u32 = 258;
/// The wait failed and supplied a thread-local Win32 error.
const WAIT_FAILED: u32 = u32::MAX;

#[link(name = "kernel32")]
#[expect(unsafe_code, reason = "exact Win32 process-query and wait ABIs")]
unsafe extern "system" {
    /// Acquire one process handle with the requested rights and inheritance flag.
    fn OpenProcess(access: u32, inherit_handle: i32, process_id: u32) -> *mut c_void;
    /// Read the process identifier from a queryable process handle.
    fn GetProcessId(process: *mut c_void) -> u32;
    /// Return the package full name of the process, including its terminator.
    fn GetPackageFullName(process: *mut c_void, length: *mut u32, name: *mut u16) -> i32;
    /// Read the full executable name with a wide-unit capacity and output count.
    fn QueryFullProcessImageNameW(
        process: *mut c_void,
        flags: u32,
        name: *mut u16,
        count: *mut u32,
    ) -> i32;
    /// Read the object's signal state with a timeout in milliseconds.
    fn WaitForSingleObject(handle: *mut c_void, milliseconds: u32) -> u32;
    /// Obtain the immediate native error on the calling thread.
    fn GetLastError() -> u32;
}

/// Bounded failures while acquiring or observing the candidate process.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PeerProcessError {
    /// Opening the queried process failed with this native error code.
    Open(u32),
    /// Querying the retained handle's PID failed with this native error code.
    QueryId(u32),
    /// The retained handle reported a different PID.
    WrongPid,
    /// The retained process object signaled termination.
    Exited,
    /// The zero-time wait failed with this native error code.
    Wait(u32),
    /// The zero-time wait returned a result outside the process wait contract.
    UnexpectedWait(u32),
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

/// Own one noninherited process handle with limited query and synchronize rights.
///
/// The confirmed PID identifies the opened handle's observation. The caller
/// must establish the expected peer independently, including any race before
/// acquisition, and recheck liveness where the observation is used.
#[must_use = "retain the process owner through the observation that depends on it"]
pub struct PeerProcess {
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
    pub fn open(pid: NonZeroU32) -> Result<Self, PeerProcessError> {
        #[expect(
            unsafe_code,
            reason = "retain a process with fixed query and wait rights"
        )]
        // SAFETY: The call accepts scalar access flags and a PID. Zero disables
        // inheritance. A nonnull result transfers one CloseHandle-managed owner.
        let raw: *mut c_void = unsafe {
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

    /// Poll the retained process without waiting for exit.
    ///
    /// Success means the process was unsignaled during this call. It may exit
    /// before a subsequent operation.
    ///
    /// # Errors
    /// Report exit, the original native wait failure, or an unexpected result.
    pub fn check_live(&self) -> Result<(), PeerProcessError> {
        #[expect(unsafe_code, reason = "poll the retained process without blocking")]
        // SAFETY: The borrow keeps the process handle open with synchronization
        // access. Zero milliseconds requests an immediate observation.
        let result: u32 = unsafe { WaitForSingleObject(self.handle.as_raw_handle(), 0) };
        match result {
            WAIT_TIMEOUT => Ok(()),
            WAIT_OBJECT_0 => Err(PeerProcessError::Exited),
            WAIT_FAILED => Err(PeerProcessError::Wait(last_error())),
            other => Err(PeerProcessError::UnexpectedWait(other)),
        }
    }

    /// Observe the package full name on the original retained process object.
    ///
    /// An unpackaged process returns `None`. The receiver separately checks
    /// liveness, its cutoff and the exact approved release before any effect.
    ///
    /// # Errors
    /// Retain a native refusal or reject an invalid returned name.
    pub fn package_full_name(
        &self,
    ) -> Result<Option<ReportedPackageFullName>, PackageFullNameError> {
        let mut units: [u16; PACKAGE_NAME_BUFFER] = [UNWRITTEN_NAME_UNIT; PACKAGE_NAME_BUFFER];
        let mut count: u32 =
            u32::try_from(units.len()).map_err(|_| PackageFullNameError::Malformed)?;
        #[expect(
            unsafe_code,
            reason = "observe package identity on the retained process"
        )]
        // SAFETY: The retained handle grants limited query access. Count is in
        // wide units and the initialized array owns that writable extent.
        let status: i32 = unsafe {
            GetPackageFullName(
                self.handle.as_raw_handle(),
                &raw mut count,
                units.as_mut_ptr(),
            )
        };
        decode_package_full_name(status, count, units)
    }

    /// Observe the Win32 executable image name on the original retained process.
    ///
    /// The fixture query uses one buffer capped at 1,024 wide units including
    /// the terminator. The caller checks the independently expected path and
    /// the original peer's liveness and deadline where it uses this observation.
    ///
    /// # Errors
    /// Retain the immediate Win32 error, including insufficient capacity, or
    /// reject a malformed successful result.
    pub fn image_name(&self) -> Result<ReportedProcessImageName, ProcessImageNameError> {
        read_process_image_name(
            |units: &mut [u16; PROCESS_IMAGE_NAME_BUFFER], count: &mut u32| {
                #[expect(
                    unsafe_code,
                    reason = "observe the retained process's Win32 image name"
                )]
                // SAFETY: The retained handle stays open with limited query access.
                // Flags zero request Win32 path format. The writable array covers the
                // input count in wide units and does not alias the writable DWORD.
                let status: i32 = unsafe {
                    QueryFullProcessImageNameW(
                        self.handle.as_raw_handle(),
                        0,
                        units.as_mut_ptr(),
                        &raw mut *count,
                    )
                };
                if status == 0 {
                    return Err(last_error());
                }
                Ok(())
            },
        )
    }

    /// Read the PID confirmed when the retained handle was opened.
    #[must_use]
    pub const fn pid(&self) -> NonZeroU32 {
        self.pid
    }
}

impl AsHandle for PeerProcess {
    /// Borrow the original query-and-synchronize handle for a native observation.
    fn as_handle(&self) -> BorrowedHandle<'_> {
        self.handle.as_handle()
    }
}

/// Capture a documented native failure before the process handle can close.
fn last_error() -> u32 {
    #[expect(unsafe_code, reason = "retain the failing operation's Win32 error")]
    // SAFETY: GetLastError takes no arguments and reads the calling thread's code.
    unsafe {
        GetLastError()
    }
}

// SPDX-License-Identifier: GPL-2.0-only
//! Retained process custody for native connected-peer observations.
//!
//! This adapter opens only limited process query, synchronize and token query
//! access. Pipe and service identity, the token profile and the absolute
//! admission cutoff belong to the caller.

use std::ffi::c_void;
use std::fmt;
use std::num::NonZeroU32;
use std::os::windows::io::{AsHandle, AsRawHandle, BorrowedHandle, FromRawHandle, OwnedHandle};
use std::ptr;

use crate::package_full_name::{
    PACKAGE_NAME_BUFFER, UNWRITTEN_NAME_UNIT, decode_package_full_name,
};
use crate::process_image_name::{PROCESS_IMAGE_NAME_BUFFER, read_process_image_name};
use crate::service_role_sid::{SidList, TokenSidInformation, observe_role_sid};
use crate::{
    PackageFullNameError, ProcessImageNameError, ReportedPackageFullName, ReportedProcessImageName,
    ServiceRoleSid, ServiceRoleSidObservation, ServiceRoleSidObservationError,
};

/// The Win32 process right needed to observe process metadata and query the primary token.
const PROCESS_QUERY_LIMITED_INFORMATION: u32 = 0x1000;
/// The Win32 process right needed to poll the retained object for exit.
const PROCESS_SYNCHRONIZE: u32 = 0x0010_0000;
/// The only requested access to primary and thread tokens.
const TOKEN_QUERY: u32 = 0x0008;
/// Token information class for normal groups.
const TOKEN_GROUPS_CLASS: i32 = 2;
/// Token information class containing the modification identifier.
const TOKEN_STATISTICS_CLASS: i32 = 10;
/// Token information class for the list of restricting SIDs.
const TOKEN_RESTRICTED_SIDS_CLASS: i32 = 11;
/// The thread has no impersonation token.
const ERROR_NO_TOKEN: u32 = 1008;
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
    /// Read the server process identifier reported for the original pipe instance.
    fn GetNamedPipeServerProcessId(pipe: *mut c_void, pid: *mut u32) -> i32;
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
    /// Return the calling thread's borrowed pseudohandle.
    fn GetCurrentThread() -> *mut c_void;
    /// Obtain the immediate native error on the calling thread.
    fn GetLastError() -> u32;
}

#[link(name = "advapi32")]
#[expect(unsafe_code, reason = "exact Win32 query-only token acquisition ABIs")]
unsafe extern "system" {
    /// Open the primary token of the retained process.
    fn OpenProcessToken(process: *mut c_void, access: u32, token: *mut *mut c_void) -> i32;
    /// Fill a caller-owned buffer with the selected token information.
    fn GetTokenInformation(
        token: *mut c_void,
        class: i32,
        output: *mut c_void,
        capacity: u32,
        written: *mut u32,
    ) -> i32;
    /// Observe the calling thread's impersonation token with a process-context access check.
    fn OpenThreadToken(
        thread: *mut c_void,
        access: u32,
        open_as_self: i32,
        token: *mut *mut c_void,
    ) -> i32;
}

#[link(
    name = "api-ms-win-core-handle-l1-1-0.dll",
    kind = "raw-dylib",
    modifiers = "+verbatim"
)]
#[expect(unsafe_code, reason = "exact Win32 kernel object comparison ABI")]
unsafe extern "system" {
    /// Determine whether two handles refer to the same underlying object.
    fn CompareObjectHandles(first: *mut c_void, second: *mut c_void) -> i32;
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

/// Refusal while querying the reported server PID of a connected pipe.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PipeServerPidError {
    /// The native query failed with this immediate Win32 error code.
    Native(u32),
    /// A successful query supplied no nonzero process identifier.
    ZeroPid,
}

impl fmt::Display for PipeServerPidError {
    /// Describe the failed observation without disclosing a process identifier.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Native(code) => write!(formatter, "GetNamedPipeServerProcessId failed ({code})"),
            Self::ZeroPid => formatter.write_str("pipe server query returned a zero PID"),
        }
    }
}

impl std::error::Error for PipeServerPidError {}

/// Read the native server PID from the caller's original connected client handle.
///
/// The caller owns connection selection, endpoint direction and the cutoff. The
/// reported PID supplies a comparison input for further process observation.
///
/// # Errors
/// Preserve native query failure or refuse a zero PID.
pub fn read_connected_pipe_server_pid(
    client: BorrowedHandle<'_>,
) -> Result<NonZeroU32, PipeServerPidError> {
    let mut pid: u32 = 0;
    #[expect(
        unsafe_code,
        reason = "query the reported server PID without driving the pipe"
    )]
    // SAFETY: The borrow keeps the supplied handle live for this call. The
    // output points to one initialized, writable DWORD. Win32 checks the handle.
    let found: i32 = unsafe { GetNamedPipeServerProcessId(client.as_raw_handle(), &raw mut pid) };
    if found == 0 {
        return Err(PipeServerPidError::Native(last_error()));
    }
    NonZeroU32::new(pid).ok_or(PipeServerPidError::ZeroPid)
}

/// Bounded refusal while acquiring a query-only primary token.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PeerPrimaryTokenError {
    /// The thread token query failed without establishing its absence.
    ThreadQuery(u32),
    /// The caller's thread is impersonating.
    AmbientThreadToken,
    /// Acquiring query access to the process token failed.
    ProcessTokenQuery(u32),
    /// A successful token query did not return a valid owned handle.
    InvalidHandle,
}

impl fmt::Display for PeerPrimaryTokenError {
    /// Report the failed operation without disclosing any token data.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ThreadQuery(code) => write!(formatter, "OpenThreadToken failed ({code})"),
            Self::AmbientThreadToken => formatter.write_str("thread already has a token"),
            Self::ProcessTokenQuery(code) => write!(formatter, "OpenProcessToken failed ({code})"),
            Self::InvalidHandle => formatter.write_str("invalid acquired token handle"),
        }
    }
}

impl std::error::Error for PeerPrimaryTokenError {}

/// Retain one noninherited, query-only primary-token handle.
///
/// Matching objects establish token continuity for the observations compared.
/// The receiving caller checks token attributes, role, process liveness and cutoff.
#[must_use = "retain the token object through the comparison that depends on it"]
pub struct PeerPrimaryToken {
    /// The original process-token query handle, closed once by its owner.
    handle: OwnedHandle,
}

impl PeerPrimaryToken {
    /// Compare against an independently observed token through its live handle.
    ///
    /// True requires the same kernel object. A false comparison supplies no match.
    /// Compare only references whose process and identity provenance the caller owns.
    #[must_use]
    pub fn refers_to(&self, reference: BorrowedHandle<'_>) -> bool {
        #[expect(unsafe_code, reason = "compare without disclosing the retained handle")]
        // SAFETY: Both handles remain borrowed throughout the synchronous comparison.
        // The call neither retains nor closes them and requires no extra object rights.
        let same: i32 =
            unsafe { CompareObjectHandles(self.handle.as_raw_handle(), reference.as_raw_handle()) };
        same != 0
    }

    /// Observe the expected SID within one generation of this query-only token.
    ///
    /// # Errors
    /// Preserve a native query failure or refuse malformed output or a changed token.
    pub(crate) fn observe_role_sid<'sid>(
        &self,
        expected: ServiceRoleSid<'sid>,
    ) -> Result<ServiceRoleSidObservation<'sid>, ServiceRoleSidObservationError> {
        observe_role_sid(
            expected,
            |information: TokenSidInformation, output: &mut [u8], capacity: u32| {
                let class: i32 = match information {
                    TokenSidInformation::Statistics => TOKEN_STATISTICS_CLASS,
                    TokenSidInformation::List(SidList::Groups) => TOKEN_GROUPS_CLASS,
                    TokenSidInformation::List(SidList::Restricting) => TOKEN_RESTRICTED_SIDS_CLASS,
                };
                let mut written: u32 = 0;
                #[expect(
                    unsafe_code,
                    reason = "query the retained token's statistics and SID lists"
                )]
                // SAFETY: This token handle remains live with TOKEN_QUERY. The slice
                // is the initialized, SDK-aligned output owned by the bounded
                // observer. The separate writable DWORD receives the result extent.
                let found: i32 = unsafe {
                    GetTokenInformation(
                        self.handle.as_raw_handle(),
                        class,
                        output.as_mut_ptr().cast(),
                        capacity,
                        &raw mut written,
                    )
                };
                if found == 0 {
                    return Err(last_error());
                }
                Ok(written)
            },
        )
    }

    /// Compare two retained observations without exposing either token handle.
    #[must_use]
    pub(crate) fn same_object(&self, other: &Self) -> bool {
        self.refers_to(other.handle.as_handle())
    }
}

/// Load the address of an initialized native PSID field without accessing its pointee.
pub(super) fn sid_address(entry: &[u8]) -> Option<usize> {
    let bytes: &[u8; size_of::<*const c_void>()] = entry.first_chunk()?;
    #[expect(unsafe_code, reason = "load the original SDK pointer representation")]
    // SAFETY: The fixed-size reference covers the initialized pointer field.
    // Any pointer bit pattern is admissible here. Only its numeric address is used.
    let pointer: *const c_void = unsafe { bytes.as_ptr().cast::<*const c_void>().read_unaligned() };
    Some(pointer.addr())
}

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

    /// Retain the primary token queried from the original process object.
    ///
    /// This synchronous observation checks the calling thread before opening the
    /// token. The caller separately retains its process and checks liveness and
    /// its original deadline around the dependent observations.
    ///
    /// # Errors
    /// Reject an ambient thread token, native query failure or invalid handle.
    pub fn retain_primary_token(&self) -> Result<PeerPrimaryToken, PeerPrimaryTokenError> {
        require_no_thread_token()?;
        let mut raw: *mut c_void = ptr::null_mut();
        #[expect(
            unsafe_code,
            reason = "open only query rights on the original process token"
        )]
        // SAFETY: The original process handle remains owned and has limited query
        // access. The output is initialized and writable for one handle.
        let opened: i32 =
            unsafe { OpenProcessToken(self.handle.as_raw_handle(), TOKEN_QUERY, &raw mut raw) };
        if opened == 0 {
            return Err(PeerPrimaryTokenError::ProcessTokenQuery(last_error()));
        }
        Ok(PeerPrimaryToken {
            handle: take_queried_token(raw)?,
        })
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

/// Require definite absence of an ambient impersonation token.
///
/// # Errors
/// Reject a present token, invalid successful output or an unexpected native failure.
fn require_no_thread_token() -> Result<(), PeerPrimaryTokenError> {
    #[expect(
        unsafe_code,
        reason = "borrow the calling thread without acquiring rights"
    )]
    // SAFETY: The call takes no inputs and returns the current thread's pseudohandle.
    let thread: *mut c_void = unsafe { GetCurrentThread() };
    let mut raw: *mut c_void = ptr::null_mut();
    #[expect(
        unsafe_code,
        reason = "observe only the calling thread's ambient token"
    )]
    // SAFETY: The thread pseudohandle is valid and raw is writable for one handle.
    // OpenAsSelf selects the access-check context and does not change the token.
    let opened: i32 = unsafe { OpenThreadToken(thread, TOKEN_QUERY, 1, &raw mut raw) };
    if opened == 0 {
        let code: u32 = last_error();
        return if code == ERROR_NO_TOKEN {
            Ok(())
        } else {
            Err(PeerPrimaryTokenError::ThreadQuery(code))
        };
    }
    let _ambient: OwnedHandle = take_queried_token(raw)?;
    Err(PeerPrimaryTokenError::AmbientThreadToken)
}

/// Take sole close custody of the output of a successful Win32 token open.
///
/// # Errors
/// Reject null or `INVALID_HANDLE_VALUE` output.
fn take_queried_token(raw: *mut c_void) -> Result<OwnedHandle, PeerPrimaryTokenError> {
    if raw.is_null() || raw.addr() == usize::MAX {
        return Err(PeerPrimaryTokenError::InvalidHandle);
    }
    #[expect(
        unsafe_code,
        reason = "transfer the successful token output into its close owner"
    )]
    // SAFETY: The only callers pass successful token open outputs. The check
    // excludes invalid sentinels and no other owner closes the new handle.
    let handle: OwnedHandle = unsafe { OwnedHandle::from_raw_handle(raw) };
    Ok(handle)
}

/// Capture a documented native failure before the process handle can close.
fn last_error() -> u32 {
    #[expect(unsafe_code, reason = "retain the failing operation's Win32 error")]
    // SAFETY: GetLastError takes no arguments and reads the calling thread's code.
    unsafe {
        GetLastError()
    }
}

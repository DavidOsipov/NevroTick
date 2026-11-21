// SPDX-License-Identifier: GPL-2.0-only
//! Same-process named pipe endpoints for bounded Windows completion controls.

use std::fmt;
use std::mem::size_of;
use std::os::windows::io::{AsHandle, AsRawHandle, BorrowedHandle, FromRawHandle, OwnedHandle};
use std::ptr;

use windows_sys::Win32::Foundation::{
    ERROR_PIPE_CONNECTED, GENERIC_READ, GetLastError, HANDLE, INVALID_HANDLE_VALUE, LocalFree,
};
use windows_sys::Win32::Security::Authorization::{
    ConvertStringSecurityDescriptorToSecurityDescriptorW, SDDL_REVISION_1,
};
use windows_sys::Win32::Security::{PSECURITY_DESCRIPTOR, SECURITY_ATTRIBUTES};
use windows_sys::Win32::Storage::FileSystem::{
    CreateFileW, FILE_FLAG_FIRST_PIPE_INSTANCE, FILE_FLAG_OVERLAPPED, OPEN_EXISTING,
    PIPE_ACCESS_OUTBOUND, SECURITY_IDENTIFICATION, SECURITY_SQOS_PRESENT, WriteFile,
};
use windows_sys::Win32::System::Pipes::{
    ConnectNamedPipe, CreateNamedPipeW, GetNamedPipeClientProcessId, GetNamedPipeServerProcessId,
    PIPE_REJECT_REMOTE_CLIENTS,
};
use windows_sys::Win32::System::Threading::GetCurrentProcessId;

/// Largest inert write and configured server buffer size, in bytes.
const MAX_WRITE_BYTES: u32 = 80;

/// Private, read-only client endpoint opened for overlapped I/O.
pub(super) struct LabPipeReader(OwnedHandle);

impl AsHandle for LabPipeReader {
    fn as_handle(&self) -> BorrowedHandle<'_> {
        self.0.as_handle()
    }
}

/// Private, synchronous server endpoint used only to send inert fixture bytes.
pub(super) struct LabPipeWriter(OwnedHandle);

impl LabPipeWriter {
    /// Write one nonempty, bounded byte sequence to the connected fixture reader.
    ///
    /// The child supervisor owns the absolute timeout for this synchronous call.
    ///
    /// # Errors
    /// Reject empty or oversized input before writing. Surface native and short
    /// writes without retrying an operation that may have produced a partial effect.
    pub(super) fn write(&self, bytes: &[u8]) -> Result<(), LabPipeError> {
        let count: u32 = u32::try_from(bytes.len()).map_err(|_| LabPipeError::WriteSize)?;
        if count == 0 || count > MAX_WRITE_BYTES {
            return Err(LabPipeError::WriteSize);
        }

        let mut written: u32 = 0;
        #[expect(
            unsafe_code,
            reason = "write bounded bytes on the fixture-owned synchronous pipe"
        )]
        // SAFETY: The owned server is synchronous. The input and output pointers
        // are valid for the call; the null OVERLAPPED requests no retained storage.
        let wrote: i32 = unsafe {
            WriteFile(
                self.0.as_raw_handle(),
                bytes.as_ptr(),
                count,
                &raw mut written,
                ptr::null_mut(),
            )
        };
        if wrote == 0 {
            return Err(last_error("WriteFile"));
        }
        if written != count {
            return Err(LabPipeError::ShortWrite {
                expected: count,
                written,
            });
        }
        Ok(())
    }
}

/// Fixture setup or write failure. Native errors retain the exact called operation.
#[derive(Debug)]
pub(super) enum LabPipeError {
    /// The named Win32 call failed and supplied this last-error code.
    Native { operation: &'static str, code: u32 },
    /// A successful native result omitted its required handle or descriptor.
    MissingResource(&'static str),
    /// The fixture could not represent the security attributes size.
    SecurityAttributesSize,
    /// At least one connected endpoint reported another process.
    ForeignPeer,
    /// The proposed write was empty or exceeded the fixture's byte cap.
    WriteSize,
    /// The native call reported fewer or more than the expected number of bytes.
    ShortWrite { expected: u32, written: u32 },
}

impl fmt::Display for LabPipeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Native { operation, code } => write!(formatter, "{operation} failed ({code})"),
            Self::MissingResource(operation) => {
                write!(formatter, "{operation} returned no resource")
            }
            Self::SecurityAttributesSize => {
                formatter.write_str("security attributes size unavailable")
            }
            Self::ForeignPeer => {
                formatter.write_str("fixture pipe endpoint belongs to another process")
            }
            Self::WriteSize => {
                formatter.write_str("fixture pipe write must contain 1 through 80 bytes")
            }
            Self::ShortWrite { expected, written } => {
                write!(
                    formatter,
                    "fixture pipe wrote {written} of {expected} bytes"
                )
            }
        }
    }
}

impl std::error::Error for LabPipeError {}

/// Open one same-process fixture pair for a fixed-width case number.
///
/// Both owned endpoints are checked before the writer can escape. The relative
/// pipe name is never returned and this fixture checks no service identity.
///
/// # Errors
/// Return exact setup failures or a foreign-peer refusal. Both handles close on refusal.
pub(super) fn open_lab_pipe(case: u32) -> Result<(LabPipeReader, LabPipeWriter), LabPipeError> {
    #[expect(unsafe_code, reason = "read the fixture's own process identifier")]
    // SAFETY: This argumentless API returns only the current PID.
    let process: u32 = unsafe { GetCurrentProcessId() };
    let name: Vec<u16> = format!(r"\\.\pipe\nevrotick-lab-private-pipe-{process:08x}-{case:08x}")
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    let server: OwnedHandle = create_server(&name)?;
    #[expect(
        unsafe_code,
        reason = "open the exact local pipe as a read-only overlapped client"
    )]
    // SAFETY: The fixed-size UTF-16 name is NUL-terminated. Null optional
    // parameters disable inheritance and pass no template handle.
    let client: HANDLE = unsafe {
        CreateFileW(
            name.as_ptr(),
            GENERIC_READ,
            0,
            ptr::null(),
            OPEN_EXISTING,
            FILE_FLAG_OVERLAPPED | SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION,
            ptr::null_mut(),
        )
    };
    let client: OwnedHandle = take_handle(client, "CreateFileW")?;
    #[expect(
        unsafe_code,
        reason = "confirm connection on the fixture-owned synchronous server"
    )]
    // SAFETY: The owned server was opened without FILE_FLAG_OVERLAPPED; the live
    // client is already connected and this call retains no storage.
    let connected: i32 = unsafe { ConnectNamedPipe(server.as_raw_handle(), ptr::null_mut()) };
    if connected == 0 {
        let error: LabPipeError = last_error("ConnectNamedPipe");
        if !matches!(
            error,
            LabPipeError::Native {
                code: ERROR_PIPE_CONNECTED,
                ..
            }
        ) {
            return Err(error);
        }
    }
    check_peers(&server, &client, process)?;
    Ok((LabPipeReader(client), LabPipeWriter(server)))
}

/// Create a single noninherited local-only writer instance with an explicit DACL.
///
/// # Errors
/// Preserve the first failed Win32 operation. Close a created handle if descriptor
/// release fails. Reject a missing native resource.
fn create_server(name: &[u16]) -> Result<OwnedHandle, LabPipeError> {
    let length: u32 = u32::try_from(size_of::<SECURITY_ATTRIBUTES>())
        .map_err(|_| LabPipeError::SecurityAttributesSize)?;
    let sddl: Vec<u16> = "D:P(A;;GA;;;OW)"
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    let mut descriptor: PSECURITY_DESCRIPTOR = ptr::null_mut();
    #[expect(
        unsafe_code,
        reason = "allocate the fixture's protected owner-only DACL"
    )]
    // SAFETY: The fixed SDDL is NUL-terminated. descriptor is owned writable
    // output; no size output is needed and the result must be freed with LocalFree.
    let converted: i32 = unsafe {
        ConvertStringSecurityDescriptorToSecurityDescriptorW(
            sddl.as_ptr(),
            SDDL_REVISION_1,
            &raw mut descriptor,
            ptr::null_mut(),
        )
    };
    if converted == 0 {
        return Err(last_error(
            "ConvertStringSecurityDescriptorToSecurityDescriptorW",
        ));
    }
    if descriptor.is_null() {
        return Err(LabPipeError::MissingResource(
            "ConvertStringSecurityDescriptorToSecurityDescriptorW",
        ));
    }
    let attributes: SECURITY_ATTRIBUTES = SECURITY_ATTRIBUTES {
        nLength: length,
        lpSecurityDescriptor: descriptor,
        bInheritHandle: 0,
    };

    #[expect(
        unsafe_code,
        reason = "create the only writer for this fixed local fixture name"
    )]
    // SAFETY: The only caller constructs a NUL-terminated UTF-16 name. The
    // attributes and its descriptor remain valid throughout the call.
    let server: HANDLE = unsafe {
        CreateNamedPipeW(
            name.as_ptr(),
            PIPE_ACCESS_OUTBOUND | FILE_FLAG_FIRST_PIPE_INSTANCE,
            // Byte type, byte read mode and blocking server mode are zero-valued.
            PIPE_REJECT_REMOTE_CLIENTS,
            1,
            MAX_WRITE_BYTES,
            0,
            0,
            &raw const attributes,
        )
    };
    let server: Result<OwnedHandle, LabPipeError> = take_handle(server, "CreateNamedPipeW");
    #[expect(
        unsafe_code,
        reason = "free the exact SDDL descriptor supplied to CreateNamedPipeW"
    )]
    // SAFETY: The descriptor was allocated by the successful conversion and has
    // not been freed. The Win32 create call has returned and no longer borrows it.
    let remaining: PSECURITY_DESCRIPTOR = unsafe { LocalFree(descriptor) };
    if remaining.is_null() {
        return server;
    }
    let release_error: LabPipeError = last_error("LocalFree");
    let _server: OwnedHandle = server?;
    Err(release_error)
}

/// Relate both connected endpoints to the fixture process before any write.
///
/// # Errors
/// Preserve failed native PID observations. Reject any different process.
fn check_peers(
    server: &OwnedHandle,
    client: &OwnedHandle,
    process: u32,
) -> Result<(), LabPipeError> {
    let mut peer_client: u32 = 0;
    #[expect(
        unsafe_code,
        reason = "observe the client attached to the owned server pipe"
    )]
    // SAFETY: The owned server handle stays open and peer_client is writable.
    let found: i32 =
        unsafe { GetNamedPipeClientProcessId(server.as_raw_handle(), &raw mut peer_client) };
    if found == 0 {
        return Err(last_error("GetNamedPipeClientProcessId"));
    }
    let mut peer_server: u32 = 0;
    #[expect(
        unsafe_code,
        reason = "observe the server attached to the owned client pipe"
    )]
    // SAFETY: The owned client handle stays open and peer_server is writable.
    let found: i32 =
        unsafe { GetNamedPipeServerProcessId(client.as_raw_handle(), &raw mut peer_server) };
    if found == 0 {
        return Err(last_error("GetNamedPipeServerProcessId"));
    }
    if peer_client != process || peer_server != process {
        return Err(LabPipeError::ForeignPeer);
    }
    Ok(())
}

/// Own the result of a fixture handle-creation call using `INVALID_HANDLE_VALUE`.
///
/// # Errors
/// Retain the failed call's last error. Refuse null success output.
fn take_handle(raw: HANDLE, operation: &'static str) -> Result<OwnedHandle, LabPipeError> {
    if raw == INVALID_HANDLE_VALUE {
        return Err(last_error(operation));
    }
    if raw.is_null() {
        return Err(LabPipeError::MissingResource(operation));
    }
    #[expect(
        unsafe_code,
        reason = "adopt a newly opened non-sentinel handle exactly once"
    )]
    // SAFETY: Callers transfer their unique CreateFileW or CreateNamedPipeW result.
    // Both APIs return CloseHandle-managed handles and neither creates a pseudo-handle.
    let owned: OwnedHandle = unsafe { OwnedHandle::from_raw_handle(raw) };
    Ok(owned)
}

/// Capture the current thread's last error immediately following a failing call.
fn last_error(operation: &'static str) -> LabPipeError {
    #[expect(
        unsafe_code,
        reason = "read the current thread's failing operation code"
    )]
    // SAFETY: GetLastError takes no arguments and returns only the thread-local code.
    let code: u32 = unsafe { GetLastError() };
    LabPipeError::Native { operation, code }
}

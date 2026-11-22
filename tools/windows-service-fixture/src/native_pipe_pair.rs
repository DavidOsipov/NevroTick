// SPDX-License-Identifier: GPL-2.0-only
//! Named pipe endpoints for bounded Windows completion and peer observations.

use std::fmt;
use std::mem::size_of;
use std::os::windows::io::{AsHandle, AsRawHandle, BorrowedHandle, FromRawHandle, OwnedHandle};
use std::ptr;
use std::sync::Arc;

use windows_sys::Win32::Foundation::{
    ERROR_PIPE_CONNECTED, GENERIC_READ, GetLastError, HANDLE, INVALID_HANDLE_VALUE, LocalFree,
};
use windows_sys::Win32::Security::Authorization::{
    ConvertStringSecurityDescriptorToSecurityDescriptorW, SDDL_REVISION_1,
};
use windows_sys::Win32::Security::{PSECURITY_DESCRIPTOR, SECURITY_ATTRIBUTES};
use windows_sys::Win32::Storage::FileSystem::{
    CreateFileW, FILE_FLAG_FIRST_PIPE_INSTANCE, FILE_FLAG_OVERLAPPED, FILE_WRITE_DATA,
    OPEN_EXISTING, PIPE_ACCESS_INBOUND, PIPE_ACCESS_OUTBOUND, SECURITY_ANONYMOUS,
    SECURITY_IDENTIFICATION, SECURITY_SQOS_PRESENT, SYNCHRONIZE, WriteFile,
};
use windows_sys::Win32::System::Pipes::{
    ConnectNamedPipe, CreateNamedPipeW, GetNamedPipeClientProcessId, GetNamedPipeServerProcessId,
    PIPE_REJECT_REMOTE_CLIENTS,
};
use windows_sys::Win32::System::Threading::GetCurrentProcessId;

/// Largest inert write and configured server buffer size, in bytes.
const MAX_WRITE_BYTES: u32 = 80;

/// A fixed local fixture name with its terminating NUL.
struct LabPipeName(Vec<u16>);

impl LabPipeName {
    /// Derive the local name from an OS process identifier and fixture case.
    fn for_process(process: u32, case: u32) -> Self {
        Self(
            format!(r"\\.\pipe\nevrotick-lab-private-pipe-{process:08x}-{case:08x}")
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect(),
        )
    }
}

/// Private, read-only endpoint opened for overlapped I/O.
pub(super) struct LabPipeReader {
    handle: OwnedHandle,
    reply_pair: Option<ReplyPipePair>,
}

/// Identity minted only when the native reply pipe's two handles are opened.
#[derive(Clone)]
pub(super) struct ReplyPipePair(Arc<()>);

impl LabPipeReader {
    /// Return the identity of the owned reply endpoint, if this is one.
    pub(super) const fn reply_pair(&self) -> Option<&ReplyPipePair> {
        self.reply_pair.as_ref()
    }
}

impl AsHandle for LabPipeReader {
    fn as_handle(&self) -> BorrowedHandle<'_> {
        self.handle.as_handle()
    }
}

/// Private synchronous endpoint used only to send inert fixture bytes.
pub(super) struct LabPipeWriter(OwnedHandle);

/// Both original endpoints of an inbound pipe with client identity capture.
pub(super) struct LabIdentificationPipe {
    reader: LabPipeReader,
    writer: LabPipeWriter,
}

impl LabIdentificationPipe {
    /// Transfer both endpoints to their single request-transport owner.
    pub(super) fn into_transport(self) -> (LabPipeReader, LabPipeWriter) {
        (self.reader, self.writer)
    }
}

/// The actual client security context selected by a fixture control.
#[derive(Clone, Copy)]
pub(super) enum LabClientContext {
    /// The selected static identification-level client.
    Identification,
    /// A separate native refusal control with no queryable client identity.
    Anonymous,
}

/// Private server endpoint opened for overlapped reply writes.
pub(super) struct LabOverlappedWriter {
    handle: OwnedHandle,
    reply_pair: ReplyPipePair,
}

impl LabOverlappedWriter {
    /// Match the privately issued pair identity, separately from a native PID observation.
    /// # Errors
    /// Refuses an absent or different reply pair even in the same process.
    pub(super) fn require_reply_pair(
        &self,
        pair: Option<&ReplyPipePair>,
    ) -> Result<(), LabPipeError> {
        if pair.is_none_or(|pair: &ReplyPipePair| !Arc::ptr_eq(&self.reply_pair.0, &pair.0)) {
            return Err(LabPipeError::ForeignReplyPair);
        }
        Ok(())
    }

    /// Verify the exact reader and the current peer observations on both retained handles.
    /// # Errors
    /// Refuses a foreign pair or missing or different native peer identity.
    pub(super) fn check_reply_reader(&self, reader: &LabPipeReader) -> Result<(), LabPipeError> {
        self.require_reply_pair(reader.reply_pair())?;
        check_peers(&self.handle, &reader.handle, std::process::id())
    }
}

impl AsHandle for LabOverlappedWriter {
    fn as_handle(&self) -> BorrowedHandle<'_> {
        self.handle.as_handle()
    }
}

/// Select the only admitted pair roles before acquiring either handle.
#[derive(Clone, Copy)]
enum ServerMode {
    /// Existing synchronous test peer.
    Synchronous,
    /// Broker reply writer.
    Overlapped,
    /// Server reads an inbound request and captures the just-read client context.
    ReadIdentity(LabClientContext),
}

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
        // SAFETY: The owned writer is synchronous. The input and output pointers
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
    /// The physical reply belongs to a different laboratory connection.
    ForeignReplyPair,
    /// The proposed write was empty or exceeded the fixture's byte cap.
    WriteSize,
    /// The paired reader has no unused admission for this exact write size.
    WriteSlot,
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
            Self::ForeignReplyPair => {
                formatter.write_str("fixture reply belongs to a different pipe pair")
            }
            Self::WriteSize => {
                formatter.write_str("fixture pipe write must contain 1 through 80 bytes")
            }
            Self::WriteSlot => {
                formatter.write_str("fixture request write has no matching read slot")
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
    let (server, client): (OwnedHandle, OwnedHandle) = open_pair(case, ServerMode::Synchronous)?;
    Ok((
        LabPipeReader {
            handle: client,
            reply_pair: None,
        },
        LabPipeWriter(server),
    ))
}

/// Open a same-process pair with a native overlapped reply writer.
/// # Errors
/// Return exact setup failures or a foreign-peer refusal before exposing either handle.
pub(super) fn open_lab_reply_pipe(
    case: u32,
) -> Result<(LabPipeReader, LabOverlappedWriter), LabPipeError> {
    let (server, client): (OwnedHandle, OwnedHandle) = open_pair(case, ServerMode::Overlapped)?;
    let pair: ReplyPipePair = ReplyPipePair(Arc::new(()));
    Ok((
        LabPipeReader {
            handle: client,
            reply_pair: Some(pair.clone()),
        },
        LabOverlappedWriter {
            handle: server,
            reply_pair: pair,
        },
    ))
}

/// Open an inbound overlapped server and a synchronous write-data client.
/// # Errors
/// Return exact setup failures or a foreign-peer refusal before exposing either handle.
pub(super) fn open_lab_identification_pipe(
    case: u32,
    context: LabClientContext,
) -> Result<LabIdentificationPipe, LabPipeError> {
    let (server, client): (OwnedHandle, OwnedHandle) =
        open_pair(case, ServerMode::ReadIdentity(context))?;
    Ok(LabIdentificationPipe {
        reader: LabPipeReader {
            handle: server,
            reply_pair: None,
        },
        writer: LabPipeWriter(client),
    })
}

/// Recheck the exact paired request handles against the running fixture process.
/// # Errors
/// Refuses unavailable or foreign native peer identity.
pub(super) fn check_request_peers(
    reader: &LabPipeReader,
    writer: &LabPipeWriter,
) -> Result<(), LabPipeError> {
    check_peers(&reader.handle, &writer.0, std::process::id())
}

/// Open and admit both ends of one local pipe for the selected mode.
/// # Errors
/// Close both handles on native setup or peer refusal.
fn open_pair(case: u32, mode: ServerMode) -> Result<(OwnedHandle, OwnedHandle), LabPipeError> {
    #[expect(unsafe_code, reason = "read the fixture's own process identifier")]
    // SAFETY: This argumentless API returns only the current PID.
    let process: u32 = unsafe { GetCurrentProcessId() };
    let name: LabPipeName = LabPipeName::for_process(process, case);
    let server: OwnedHandle = create_server(&name, mode)?;
    let client: OwnedHandle = open_client(&name, mode)?;
    if matches!(mode, ServerMode::Synchronous) {
        confirm_synchronous_connection(&server)?;
    }
    // CreateFile has already connected this live client. Verify the two
    // endpoints without submitting any overlapped connection operation.
    check_peers(&server, &client, process)?;
    Ok((server, client))
}

/// Open a noninherited client endpoint using the selected fixture profile.
/// # Errors
/// Retain the native open failure without exposing a client handle.
fn open_client(name: &LabPipeName, mode: ServerMode) -> Result<OwnedHandle, LabPipeError> {
    let (access, flags): (u32, u32) = match mode {
        ServerMode::ReadIdentity(context) => {
            let level: u32 = match context {
                LabClientContext::Identification => SECURITY_IDENTIFICATION,
                LabClientContext::Anonymous => SECURITY_ANONYMOUS,
            };
            // Omit dynamic tracking and effective-only. A capture still grants no effect rights.
            (FILE_WRITE_DATA | SYNCHRONIZE, SECURITY_SQOS_PRESENT | level)
        }
        ServerMode::Synchronous | ServerMode::Overlapped => (
            GENERIC_READ,
            FILE_FLAG_OVERLAPPED | SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION,
        ),
    };
    #[expect(
        unsafe_code,
        reason = "open the exact local pipe with the fixed role-specific client profile"
    )]
    // SAFETY: The fixed-size UTF-16 name is NUL-terminated. Null optional
    // parameters disable inheritance and pass no template handle.
    let client: HANDLE = unsafe {
        CreateFileW(
            name.0.as_ptr(),
            access,
            0,
            ptr::null(),
            OPEN_EXISTING,
            flags,
            ptr::null_mut(),
        )
    };
    take_handle(client, "CreateFileW")
}

/// Confirm the established connection on a synchronous server handle.
/// # Errors
/// Return any native failure other than the already connected result.
fn confirm_synchronous_connection(server: &OwnedHandle) -> Result<(), LabPipeError> {
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
    Ok(())
}

/// Create a single noninherited local-only pipe instance with an explicit DACL.
///
/// # Errors
/// Preserve the first failed Win32 operation. Close a created handle if descriptor
/// release fails. Reject a missing native resource.
fn create_server(name: &LabPipeName, mode: ServerMode) -> Result<OwnedHandle, LabPipeError> {
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
    let (access, outbound, inbound): (u32, u32, u32) = match mode {
        ServerMode::Synchronous => (PIPE_ACCESS_OUTBOUND, MAX_WRITE_BYTES, 0),
        ServerMode::Overlapped => (
            PIPE_ACCESS_OUTBOUND | FILE_FLAG_OVERLAPPED,
            MAX_WRITE_BYTES,
            0,
        ),
        ServerMode::ReadIdentity(_) => (
            PIPE_ACCESS_INBOUND | FILE_FLAG_OVERLAPPED,
            0,
            MAX_WRITE_BYTES,
        ),
    };

    #[expect(
        unsafe_code,
        reason = "create the only server for this fixed local fixture name"
    )]
    // SAFETY: LabPipeName contains a NUL-terminated UTF-16 name. The
    // attributes and its descriptor remain valid throughout the call.
    let server: HANDLE = unsafe {
        CreateNamedPipeW(
            name.0.as_ptr(),
            access | FILE_FLAG_FIRST_PIPE_INSTANCE,
            // Byte type, byte read mode and blocking server mode are zero-valued.
            PIPE_REJECT_REMOTE_CLIENTS,
            1,
            outbound,
            inbound,
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
    let peer_server: u32 = server_process_id(client)?;
    if peer_client != process || peer_server != process {
        return Err(LabPipeError::ForeignPeer);
    }
    Ok(())
}

/// Query the server PID reported for a connected client handle.
///
/// # Errors
/// Retain the native query failure. A returned PID carries no service authority.
fn server_process_id(client: &OwnedHandle) -> Result<u32, LabPipeError> {
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
    Ok(peer_server)
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

#[cfg(test)]
mod tests {
    //! Observe the retained process and token behind a read-only client endpoint.

    use std::ffi::OsStr;
    use std::io::{self, Read};
    use std::num::NonZeroU32;
    use std::os::windows::io::AsHandle;
    use std::process::{self, Child, Command, ExitStatus, Stdio};
    use std::thread;
    use std::time::{Duration, Instant};

    use windows_platform_native::{PeerProcess, PeerProcessError};
    use windows_sys::Win32::Foundation::ERROR_FILE_NOT_FOUND;

    use super::{
        LabPipeError, LabPipeName, OwnedHandle, ServerMode, create_server, open_client, open_pair,
        server_process_id,
    };
    use crate::native_peer_process::same_kernel_object;
    use crate::native_pipe_identity::RetainedPrimaryToken;

    /// Exact selector for the supervised fixture server.
    const PID_PROBE_CHILD_TEST: &str = "native_pipe_pair::tests::hold_pid_probe_server";
    /// Require explicit selection of the child role without passing a pipe name or PID.
    const PID_PROBE_CHILD: &str = "NEVROTICK_S02_CLIENT_PID_CHILD";
    /// Case reserved for this child, whose OS PID distinguishes concurrent probes.
    const PID_PROBE_CASE: u32 = 0x5350_4944;
    /// Distinct case for the original-pipe mismatch control.
    const FOREIGN_PID_CASE: u32 = 0x4650_4944;
    /// Shared absolute budget for acquiring the endpoint and releasing the child.
    const PROBE_WAIT: Duration = Duration::from_secs(5);
    /// Separate cleanup budget if orderly release fails.
    const REAP_WAIT: Duration = Duration::from_secs(2);
    /// Upper bound on the interval between immediate observations.
    const POLL_INTERVAL: Duration = Duration::from_millis(10);
    /// `AppModel`'s direct result for a process with no installed package identity.
    const NO_PACKAGE: i32 = 15_700;

    #[link(name = "kernel32")]
    #[expect(unsafe_code, reason = "independent exact-child AppModel query")]
    unsafe extern "system" {
        /// Read the package name for the calling fixture process.
        fn GetCurrentPackageFullName(length: *mut u32, name: *mut u16) -> i32;
    }

    /// Failures retain their source, or both observation and cleanup outcomes.
    type Check<T> = Result<T, Box<dyn std::error::Error>>;

    /// Own the exact fixture endpoint, observed process and primary-token handles.
    struct RetainedFixturePeer {
        /// The client handle originally opened for the spawned fixture.
        client: OwnedHandle,
        /// The same separately opened process object used for all observations.
        process: PeerProcess,
        /// Query-only handle to the primary token captured while the process was live.
        token: RetainedPrimaryToken,
    }

    impl RetainedFixturePeer {
        /// Reobserve identity through the original pipe and retained process and token.
        /// # Errors
        /// Preserve exit, native failures, mismatch or cutoff without reacquiring a process.
        fn check_current(&self, started: Instant) -> Check<()> {
            self.process.check_live()?;
            if started.elapsed() >= PROBE_WAIT {
                return Err("PID probe observation timed out".into());
            }
            check_connected_process(&self.client, &self.process)?;
            let current: RetainedPrimaryToken =
                RetainedPrimaryToken::open(self.process.as_handle())?;
            if !self.token.refers_to(&current) {
                return Err("fixture primary token object changed".into());
            }
            if self.process.package_full_name()?.is_some() {
                return Err(
                    "the copied-out fixture process has unexpected package identity".into(),
                );
            }
            self.process.check_live()?;
            if started.elapsed() >= PROBE_WAIT {
                return Err("PID probe observation timed out".into());
            }
            Ok(())
        }
    }

    /// Bind the child process and token objects to the PID reported by the client end.
    ///
    /// The API reference describes a server-created input handle. Exercise the
    /// client-opened handle directly, with no request, reply or service effects.
    ///
    /// # Errors
    /// Fail on setup, mismatched observation, exit or unresolved child cleanup.
    #[test]
    #[ignore = "requires the admitted distinct-process Windows fixture supervisor"]
    fn read_only_client_reports_live_child_pid() -> Check<()> {
        let mut child: Child = Command::new(std::env::current_exe()?)
            .args([
                "--exact",
                PID_PROBE_CHILD_TEST,
                "--ignored",
                "--nocapture",
                "--test-threads=1",
            ])
            .env_clear()
            .env(PID_PROBE_CHILD, "1")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::inherit())
            .spawn()?;
        let started: Instant = Instant::now();
        let observed: Check<RetainedFixturePeer> = observe_server_peer(&child, started);
        drop(child.stdin.take());
        match reap_child(&mut child, started, PROBE_WAIT) {
            Ok(status) if status.success() => {
                let released_after: Duration = started.elapsed();
                let peer: RetainedFixturePeer = observed?;
                check_exited_peer(&peer, started, released_after)?;
                if !matches!(
                    check_exited_peer(&peer, started, PROBE_WAIT),
                    Err(error) if error.downcast_ref::<io::Error>()
                        .is_some_and(|cause: &io::Error| cause.kind() == io::ErrorKind::TimedOut)
                ) {
                    return Err("child reap at the exact cutoff was accepted".into());
                }
                println!(
                    "peer-observation process=matched token=matched foreign-process=refused foreign-pipe=refused exited=refused cutoff=refused package=absent"
                );
                Ok(())
            }
            Ok(status) => Err(format!(
                "PID probe child exit {status}, observation {:?}",
                observed.err()
            )
            .into()),
            Err(release_error) => {
                let killed: io::Result<()> = child.kill();
                let reaped: Check<ExitStatus> = reap_child(&mut child, Instant::now(), REAP_WAIT);
                Err(format!(
                    "PID probe child release failed: {release_error}; observation {:?}; kill {killed:?}; reap {reaped:?}", observed.err()
                )
                .into())
            }
        }
    }

    /// Hold one local first instance until the supervising parent's stdin closes.
    ///
    /// Ordinary libtest invocations leave the child role dormant.
    ///
    /// # Errors
    /// Retain server creation and stdin failures. Reject unexpected stdin input.
    #[test]
    #[ignore = "selected only by the Windows peer PID supervisor"]
    fn hold_pid_probe_server() -> Check<()> {
        if std::env::var_os(PID_PROBE_CHILD).as_deref() != Some(OsStr::new("1")) {
            return Ok(());
        }
        let mut length: u32 = 0;
        #[expect(
            unsafe_code,
            reason = "independently check the exact copied-out child identity"
        )]
        // SAFETY: This documented sizing query takes a writable length and a
        // null buffer. An unpackaged process returns its direct absence code.
        let observed: i32 =
            unsafe { GetCurrentPackageFullName(&raw mut length, core::ptr::null_mut()) };
        if observed != NO_PACKAGE {
            return Err(format!("child did not observe package absence ({observed})").into());
        }
        let name: LabPipeName = LabPipeName::for_process(process::id(), PID_PROBE_CASE);
        let _server: OwnedHandle = create_server(&name, ServerMode::Overlapped)?;
        let count: usize = io::stdin().lock().read(&mut [0_u8; 1])?;
        if count != 0 {
            return Err("PID probe release must be private stdin EOF".into());
        }
        Ok(())
    }

    /// Connect only to the fixed case and retain the compared native objects.
    ///
    /// # Errors
    /// Retry only an absent pipe. Reject a failed query, mismatched PID, exited child
    /// or expired observation budget. Transfer the handles only after all checks.
    fn observe_server_peer(child: &Child, started: Instant) -> Check<RetainedFixturePeer> {
        let expected: NonZeroU32 =
            NonZeroU32::new(child.id()).ok_or("PID probe child had no PID")?;
        let parent_pid: NonZeroU32 =
            NonZeroU32::new(process::id()).ok_or("PID probe parent had no PID")?;
        if expected == parent_pid {
            return Err("PID probe requires a distinct child process".into());
        }
        let process: PeerProcess = PeerProcess::open(expected)?;
        if !same_kernel_object(process.as_handle(), child.as_handle()) {
            return Err("opened process differs from independent spawn handle".into());
        }
        let parent: PeerProcess = PeerProcess::open(parent_pid)?;
        if same_kernel_object(process.as_handle(), parent.as_handle()) {
            return Err("process object comparison accepted the live parent as the child".into());
        }
        let name: LabPipeName = LabPipeName::for_process(expected.get(), PID_PROBE_CASE);
        while started.elapsed() < PROBE_WAIT {
            process.check_live()?;
            let client: OwnedHandle = match open_client(&name, ServerMode::Overlapped) {
                Ok(client) => client,
                Err(LabPipeError::Native {
                    code: ERROR_FILE_NOT_FOUND,
                    ..
                }) => {
                    thread::sleep(POLL_INTERVAL);
                    continue;
                }
                Err(error) => return Err(error.into()),
            };
            check_connected_process(&client, &process)?;
            let (_foreign_server, foreign_client): (OwnedHandle, OwnedHandle) =
                open_pair(FOREIGN_PID_CASE, ServerMode::Overlapped)?;
            if !matches!(
                check_connected_process(&foreign_client, &process),
                Err(LabPipeError::ForeignPeer)
            ) {
                return Err(
                    "the same-account foreign pipe did not reach the process mismatch".into(),
                );
            }
            let token: RetainedPrimaryToken = RetainedPrimaryToken::open(process.as_handle())?;
            let expected_token: RetainedPrimaryToken =
                RetainedPrimaryToken::open(child.as_handle())?;
            if !token.refers_to(&expected_token) {
                return Err(
                    "opened token differs from query through independent spawn handle".into(),
                );
            }
            let peer: RetainedFixturePeer = RetainedFixturePeer {
                client,
                process,
                token,
            };
            peer.check_current(started)?;
            return Ok(peer);
        }
        Err("PID probe observation timed out".into())
    }

    /// Require the live native peer observation to match the retained process PID.
    /// # Errors
    /// Preserve the pipe query error or refuse a different process.
    fn check_connected_process(
        client: &OwnedHandle,
        process: &PeerProcess,
    ) -> Result<(), LabPipeError> {
        if server_process_id(client)? != process.pid().get() {
            return Err(LabPipeError::ForeignPeer);
        }
        Ok(())
    }

    /// Recheck the original peer after the supervisor observed actual child exit.
    /// # Errors
    /// Refuse a late reap, apparent success or an error masking the process exit.
    fn check_exited_peer(
        peer: &RetainedFixturePeer,
        started: Instant,
        released_after: Duration,
    ) -> Check<()> {
        if released_after >= PROBE_WAIT {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "PID probe child release observed at or after the shared cutoff",
            )
            .into());
        }
        match peer.check_current(started) {
            Err(error)
                if error.downcast_ref::<PeerProcessError>() == Some(&PeerProcessError::Exited) =>
            {
                Ok(())
            }
            Err(error) => Err(error),
            Ok(()) => Err("fixture accepted a peer after observed process exit".into()),
        }
    }

    /// Collect termination from the retained child without an unbounded wait.
    ///
    /// # Errors
    /// Retain a wait failure or expiry without claiming completion.
    fn reap_child(child: &mut Child, started: Instant, budget: Duration) -> Check<ExitStatus> {
        loop {
            if let Some(status) = child.try_wait()? {
                return Ok(status);
            }
            let remaining: Duration = budget.saturating_sub(started.elapsed());
            if remaining.is_zero() {
                return Err("PID probe child reap timed out".into());
            }
            thread::sleep(POLL_INTERVAL.min(remaining));
        }
    }
}

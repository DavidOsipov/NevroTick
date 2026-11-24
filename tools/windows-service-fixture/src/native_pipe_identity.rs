// SPDX-License-Identifier: GPL-2.0-only
//! Query-only caller and primary-token identity evidence for the native pipe fixture.

use std::fmt;
use std::marker::PhantomData;
use std::mem::{align_of, offset_of, size_of};
use std::os::windows::io::{AsHandle, AsRawHandle, BorrowedHandle, FromRawHandle, OwnedHandle};
use std::process;
use std::ptr;
use std::rc::Rc;

use windows_platform_native::{PeerPrimaryToken, PeerPrimaryTokenError, PeerProcess};
use windows_sys::Win32::Foundation::{
    ERROR_NO_TOKEN, GetLastError, HANDLE, INVALID_HANDLE_VALUE, TRUE,
};
use windows_sys::Win32::Security::{
    GetTokenInformation, IsWellKnownSid, RevertToSelf, SID, SecurityIdentification, TOKEN_QUERY,
    TOKEN_STATISTICS, TOKEN_TYPE, TOKEN_USER, TokenImpersonation, TokenPrimary, TokenStatistics,
    TokenUser, WinAnonymousSid,
};
use windows_sys::Win32::System::Pipes::ImpersonateNamedPipeClient;
use windows_sys::Win32::System::Threading::{
    GetCurrentProcess, GetCurrentThread, OpenProcessToken, OpenThreadToken,
};

#[cfg(test)]
mod acquisition_tests;

/// Maximum binary SID size from the Win32 SDK's `SECURITY_MAX_SID_SIZE`.
const MAX_SID_BYTES: usize = 68;
/// Length of the fixed portion of a binary SID before its subauthorities.
const SID_HEADER_BYTES: usize = offset_of!(SID, SubAuthority);
/// Number of subauthorities that fit within the Windows SID limit.
const MAX_SUBAUTHORITIES: usize = (MAX_SID_BYTES - SID_HEADER_BYTES) / size_of::<u32>();

/// Checked user SID and logon-session identifier. This relation supplies test evidence only.
#[derive(Eq, PartialEq)]
pub(super) struct PipeIdentity {
    /// Canonical binary SID followed by zero padding, with its length encoded in the SID.
    user_sid: [u8; MAX_SID_BYTES],
    /// Both halves of the token's logon-session LUID.
    authentication_id: (i32, u32),
}

impl fmt::Debug for PipeIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PipeIdentity")
            .finish_non_exhaustive()
    }
}

/// Failure to establish bounded, query-only token evidence.
#[derive(Debug)]
pub(super) enum PipeIdentityError {
    /// A documented Win32 call failed and supplied this last-error code.
    Native {
        /// Win32 operation reporting the failure.
        operation: &'static str,
        /// Thread-local Win32 last error captured at the failure.
        code: u32,
    },
    /// This thread already has an impersonation token.
    AmbientThreadToken,
    /// Token acquisition reported success without a valid owned handle.
    InvalidTokenHandle,
    /// Statistics did not supply the requested token type.
    UnexpectedTokenType,
    /// The captured impersonation token did not have identification-only quality of service.
    UnexpectedImpersonationLevel,
    /// A native output exceeded the supplied buffer or omitted required bytes.
    InvalidTokenInformation,
    /// The native output did not contain a complete and supported user SID.
    InvalidSid,
    /// The user SID denotes anonymous logon.
    AnonymousUser,
    /// A primary token changed during the identity read.
    ChangedPrimaryToken,
}

impl fmt::Display for PipeIdentityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Native { operation, code } => write!(formatter, "{operation} failed ({code})"),
            Self::AmbientThreadToken => {
                formatter.write_str("thread already has an impersonation token")
            }
            Self::InvalidTokenHandle => formatter.write_str("invalid acquired token handle"),
            Self::UnexpectedTokenType => formatter.write_str("unexpected token type"),
            Self::UnexpectedImpersonationLevel => {
                formatter.write_str("unexpected impersonation level")
            }
            Self::InvalidTokenInformation => {
                formatter.write_str("invalid token information extent")
            }
            Self::InvalidSid => formatter.write_str("invalid token user SID"),
            Self::AnonymousUser => formatter.write_str("anonymous token user"),
            Self::ChangedPrimaryToken => {
                formatter.write_str("primary token changed during identity observation")
            }
        }
    }
}

impl std::error::Error for PipeIdentityError {}

/// Capture the native identity associated with the server's most recent completed read.
///
/// The fixture read owner must serialize this call with reads and disconnects of
/// this connected server instance. Token inspection starts only after reversion.
///
/// # Errors
/// Reject an ambient thread token, failed capture, or unsuitable token evidence.
/// A failed reversion aborts the supervised fixture process immediately.
pub(super) fn capture(pipe: BorrowedHandle<'_>) -> Result<PipeIdentity, PipeIdentityError> {
    let thread: HANDLE = current_thread();
    let identification: PipeIdentification = PipeIdentification::begin(pipe)?;
    let opened_token: Result<HANDLE, PipeIdentityError> = open_thread_token(thread);
    drop(identification);

    let token: OwnedHandle = take_opened_token(opened_token?)?;
    query_identity(token.as_handle(), TokenImpersonation)
}

/// Revert an established pipe identification on its originating thread.
#[must_use = "retain the guard until the fixed identity observation has finished"]
struct PipeIdentification(PhantomData<Rc<()>>);

impl PipeIdentification {
    /// Enter only the identity context attributed to the server's latest completed read.
    ///
    /// # Errors
    /// Refuse an ambient thread token or a native impersonation failure.
    fn begin(pipe: BorrowedHandle<'_>) -> Result<Self, PipeIdentityError> {
        ensure_unimpersonated()?;
        #[expect(
            unsafe_code,
            reason = "begin pipe caller capture on the current thread"
        )]
        // SAFETY: The pipe borrow keeps the handle live for this synchronous call.
        // Windows rejects a handle that does not support server impersonation.
        let impersonated: i32 = unsafe { ImpersonateNamedPipeClient(pipe.as_raw_handle()) };
        if impersonated == 0 {
            return Err(last_error("ImpersonateNamedPipeClient"));
        }
        Ok(Self(PhantomData))
    }
}

impl Drop for PipeIdentification {
    /// Revert before returning any observation, including on unwinding.
    fn drop(&mut self) {
        #[expect(unsafe_code, reason = "end pipe impersonation on its original thread")]
        // SAFETY: Only successful impersonation constructs the guard. Its
        // thread-bound marker keeps reversion on the originating thread.
        let reverted: i32 = unsafe { RevertToSelf() };
        if reverted == 0 {
            process::abort();
        }
    }
}

/// Exercise the native process adapter during a controlled pipe identity scope.
///
/// The caller retains the connected server after its nonempty read and
/// excludes intervening reads or disconnects. Only the fixed observation
/// runs in that scope. Both outcomes reach the caller after reversion.
///
/// # Errors
/// Preserve failure to enter the identity scope and the adapter's own result
/// separately. A failed reversion aborts the fixture.
pub(super) fn observe_peer_token_during_identification(
    pipe: BorrowedHandle<'_>,
    peer: &PeerProcess,
) -> Result<Result<PeerPrimaryToken, PeerPrimaryTokenError>, PipeIdentityError> {
    let identification: PipeIdentification = PipeIdentification::begin(pipe)?;
    let observed: Result<PeerPrimaryToken, PeerPrimaryTokenError> = peer.retain_primary_token();
    drop(identification);
    Ok(observed)
}

/// Confirm that the current fixture thread has no impersonation token.
///
/// # Errors
/// Reject a present token or any result other than the native no-token status.
pub(super) fn ensure_unimpersonated() -> Result<(), PipeIdentityError> {
    match open_thread_token(current_thread()) {
        Err(PipeIdentityError::Native {
            code: ERROR_NO_TOKEN,
            ..
        }) => Ok(()),
        Ok(raw_token) => {
            let _ambient: OwnedHandle = take_opened_token(raw_token)?;
            Err(PipeIdentityError::AmbientThreadToken)
        }
        Err(error) => Err(error),
    }
}

/// Borrow the current thread pseudohandle for same-thread token queries.
fn current_thread() -> HANDLE {
    #[expect(unsafe_code, reason = "obtain the calling thread's pseudohandle")]
    // SAFETY: This result is a pseudohandle for the calling thread and isn't closed.
    unsafe {
        GetCurrentThread()
    }
}

/// Capture the fixture process's primary-token identity as the independent control.
///
/// # Errors
/// Reject failure to acquire or inspect the query-only primary-token handle.
pub(super) fn own_process_identity() -> Result<PipeIdentity, PipeIdentityError> {
    #[expect(unsafe_code, reason = "obtain the fixture process's pseudohandle")]
    // SAFETY: This call returns a borrowed pseudohandle for the current process.
    let process_handle: HANDLE = unsafe { GetCurrentProcess() };
    let token: OwnedHandle = open_primary_token(process_handle)?;
    query_identity(token.as_handle(), TokenPrimary)
}

/// Both halves of the Windows token's modification LUID.
#[derive(Clone, Copy, Eq, PartialEq)]
struct PrimaryTokenGeneration(i32, u32);

impl PrimaryTokenGeneration {
    /// Project the generation from a checked native statistics sample.
    const fn from_statistics(statistics: &TOKEN_STATISTICS) -> Self {
        Self(
            statistics.ModifiedId.HighPart,
            statistics.ModifiedId.LowPart,
        )
    }
}

/// A query-only primary token retained for object and generation comparison.
pub(super) struct RetainedPrimaryToken {
    /// Noninherited handle to the original token object.
    handle: OwnedHandle,
    /// Identity read from this same token before admission.
    identity: PipeIdentity,
    /// Generation shared by the statistics samples bracketing the SID read.
    generation: PrimaryTokenGeneration,
}

impl RetainedPrimaryToken {
    /// Observe a primary token through an already retained process object.
    /// # Errors
    /// Refuse impersonation, denied queries, unsuitable data or a changing token.
    pub(super) fn open(process: BorrowedHandle<'_>) -> Result<Self, PipeIdentityError> {
        ensure_unimpersonated()?;
        let token: OwnedHandle = open_primary_token(process.as_raw_handle())?;
        Self::from_token(token, read_statistics, sample_identity)
    }

    /// Retain the original handle only after an identity read and matching statistics.
    ///
    /// Both readers borrow the same query-only token. A refusal drops the owned
    /// handle before returning and supplies no retained identity.
    /// # Errors
    /// Preserve the first read or validation error, or refuse a changed generation.
    fn from_token(
        token: OwnedHandle,
        mut read: impl FnMut(BorrowedHandle<'_>) -> Result<(TOKEN_STATISTICS, u32), PipeIdentityError>,
        read_identity: impl FnOnce(
            BorrowedHandle<'_>,
            &TOKEN_STATISTICS,
        ) -> Result<PipeIdentity, PipeIdentityError>,
    ) -> Result<Self, PipeIdentityError> {
        let original: TOKEN_STATISTICS =
            validate_statistics(read(token.as_handle())?, TokenPrimary)?;
        let generation: PrimaryTokenGeneration = PrimaryTokenGeneration::from_statistics(&original);
        let identity: PipeIdentity = read_identity(token.as_handle(), &original)?;
        let confirmed: TOKEN_STATISTICS =
            validate_statistics(read(token.as_handle())?, TokenPrimary)?;
        if PrimaryTokenGeneration::from_statistics(&confirmed) != generation {
            return Err(PipeIdentityError::ChangedPrimaryToken);
        }
        Ok(Self {
            handle: token,
            identity,
            generation,
        })
    }

    /// Compare this checked identity token with the maintained process observation.
    pub(super) fn matches_observation(&self, observed: &PeerPrimaryToken) -> bool {
        observed.refers_to(self.handle.as_handle())
    }

    /// Require the same object, identity and modification generation at this sample.
    fn matches_current(&self, current: &Self) -> bool {
        self.identity == current.identity
            && self.generation == current.generation
            && crate::native_peer_process::same_kernel_object(
                self.handle.as_handle(),
                current.handle.as_handle(),
            )
    }
}

/// Own the independently selected fixture process and its original token.
/// The first exchange retains this owner through actual native write settlement.
pub(super) struct RetainedFixtureClient {
    /// Original noninherited process owner supplied by the fixture.
    process: PeerProcess,
    /// Query-only handle to that process's original primary token.
    token: RetainedPrimaryToken,
    /// One-way gate after process, pipe or token observation is refused.
    current: bool,
}

impl RetainedFixtureClient {
    /// Bind the fixture actor before accepting a pipe connection.
    ///
    /// # Errors
    /// Preserve the query-only primary-token acquisition refusal.
    pub(super) fn new(process: PeerProcess) -> Result<Self, PipeIdentityError> {
        let token: RetainedPrimaryToken = RetainedPrimaryToken::open(process.as_handle())?;
        Ok(Self {
            process,
            token,
            current: true,
        })
    }

    /// Borrow the identity from the exact primary token retained for this peer.
    pub(super) const fn identity(&self) -> &PipeIdentity {
        &self.token.identity
    }

    /// Require the original live process on this pipe before any identification.
    pub(super) fn observe_pipe(&mut self, server: BorrowedHandle<'_>) -> bool {
        self.current = self.current
            && self.process.check_live().is_ok()
            && crate::native_pipe_pair::read_connected_pipe_client_pid(server)
                .is_ok_and(|pid: u32| pid == self.process.pid().get());
        self.current
    }

    /// Observe the original process, token and generation through the same server.
    ///
    /// Any refusal irreversibly closes this fixture's reply authority. The
    /// process and token handles remain owned while an earlier write is charged.
    pub(super) fn observe(&mut self, server: BorrowedHandle<'_>) -> bool {
        self.current = self.observe_pipe(server)
            && RetainedPrimaryToken::open(self.process.as_handle())
                .is_ok_and(|current: RetainedPrimaryToken| self.token.matches_current(&current))
            && self.process.check_live().is_ok();
        self.current
    }
}

/// Open a query-only primary token while the caller keeps its process handle live.
/// # Errors
/// Preserve the native failure or refuse an invalid returned token handle.
fn open_primary_token(process_handle: HANDLE) -> Result<OwnedHandle, PipeIdentityError> {
    let mut raw_token: HANDLE = ptr::null_mut();
    #[expect(unsafe_code, reason = "open only a query handle to the primary token")]
    // SAFETY: The private callers retain the process handle during the call.
    // The output is live and writable for one HANDLE.
    let opened: i32 = unsafe { OpenProcessToken(process_handle, TOKEN_QUERY, &raw mut raw_token) };
    if opened == 0 {
        return Err(last_error("OpenProcessToken"));
    }
    take_opened_token(raw_token)
}

/// Acquire a query-only handle using the process-level access check.
///
/// The caller must immediately transfer a successful raw handle to an owner
/// after any required reversion.
///
/// # Errors
/// Return the native last error when Windows refuses to open the thread token.
fn open_thread_token(thread: HANDLE) -> Result<HANDLE, PipeIdentityError> {
    let mut raw_token: HANDLE = ptr::null_mut();
    #[expect(
        unsafe_code,
        reason = "capture only query rights on the current thread token"
    )]
    // SAFETY: The caller supplies its current-thread pseudohandle. The handle
    // output is live and writable. OpenAsSelf admits identification-level queries.
    let opened: i32 = unsafe { OpenThreadToken(thread, TOKEN_QUERY, TRUE, &raw mut raw_token) };
    if opened == 0 {
        return Err(last_error("OpenThreadToken"));
    }
    Ok(raw_token)
}

/// Transfer a successful token acquisition's handle to its sole closing owner.
///
/// # Errors
/// Reject a null or invalid result handle.
fn take_opened_token(raw_token: HANDLE) -> Result<OwnedHandle, PipeIdentityError> {
    if raw_token.is_null() || raw_token == INVALID_HANDLE_VALUE {
        return Err(PipeIdentityError::InvalidTokenHandle);
    }
    #[expect(
        unsafe_code,
        reason = "transfer confirmed Win32 token ownership to Rust"
    )]
    // SAFETY: The callers pass only the returned handle from a successful open.
    // The sentinel checks exclude invalid handles and no other owner closes it.
    let token: OwnedHandle = unsafe { OwnedHandle::from_raw_handle(raw_token) };
    Ok(token)
}

/// Snapshot a documented last error immediately after its failed native call.
fn last_error(operation: &'static str) -> PipeIdentityError {
    #[expect(unsafe_code, reason = "read the calling thread's Win32 last error")]
    // SAFETY: This function reads only the current thread's last-error slot.
    let code: u32 = unsafe { GetLastError() };
    PipeIdentityError::Native { operation, code }
}

/// Validate the token type and impersonation level before extracting identity.
///
/// # Errors
/// Reject a failed query, unexpected token type or level, or invalid user SID.
fn query_identity(
    token: BorrowedHandle<'_>,
    expected_type: TOKEN_TYPE,
) -> Result<PipeIdentity, PipeIdentityError> {
    let statistics: TOKEN_STATISTICS = validate_statistics(read_statistics(token)?, expected_type)?;
    sample_identity(token, &statistics)
}

/// Obtain the raw native statistics output and its reported extent.
/// # Errors
/// Preserve the native failure or refuse an unrepresentable buffer size.
fn read_statistics(
    token: BorrowedHandle<'_>,
) -> Result<(TOKEN_STATISTICS, u32), PipeIdentityError> {
    let mut statistics: TOKEN_STATISTICS = TOKEN_STATISTICS::default();
    let capacity: u32 = u32::try_from(size_of::<TOKEN_STATISTICS>())
        .map_err(|_| PipeIdentityError::InvalidTokenInformation)?;
    let mut written: u32 = 0;
    #[expect(unsafe_code, reason = "read the fixed TOKEN_STATISTICS Win32 layout")]
    // SAFETY: The handle remains live. Both outputs are writable and the ABI
    // structure occupies the supplied byte count with all fields initialized.
    let success: i32 = unsafe {
        GetTokenInformation(
            token.as_raw_handle(),
            TokenStatistics,
            (&raw mut statistics).cast(),
            capacity,
            &raw mut written,
        )
    };
    if success == 0 {
        return Err(last_error("GetTokenInformation(TokenStatistics)"));
    }
    Ok((statistics, written))
}

/// Require a complete statistics record for the requested token type and level.
/// # Errors
/// Refuse an invalid extent, unexpected token type or identification level.
fn validate_statistics(
    (statistics, written): (TOKEN_STATISTICS, u32),
    expected_type: TOKEN_TYPE,
) -> Result<TOKEN_STATISTICS, PipeIdentityError> {
    if usize::try_from(written).ok() != Some(size_of::<TOKEN_STATISTICS>()) {
        return Err(PipeIdentityError::InvalidTokenInformation);
    }
    if statistics.TokenType != expected_type {
        return Err(PipeIdentityError::UnexpectedTokenType);
    }
    if expected_type == TokenImpersonation
        && statistics.ImpersonationLevel != SecurityIdentification
    {
        return Err(PipeIdentityError::UnexpectedImpersonationLevel);
    }
    Ok(statistics)
}

/// Read the user SID on the same token that supplied the validated statistics.
/// # Errors
/// Preserve a native query or SID refusal.
fn sample_identity(
    token: BorrowedHandle<'_>,
    statistics: &TOKEN_STATISTICS,
) -> Result<PipeIdentity, PipeIdentityError> {
    Ok(PipeIdentity {
        user_sid: query_user_sid(token)?,
        authentication_id: (
            statistics.AuthenticationId.HighPart,
            statistics.AuthenticationId.LowPart,
        ),
    })
}

/// Aligned caller-owned `TokenUser` output, with inline storage for the largest SID.
#[repr(C)]
struct TokenUserBuffer {
    /// Native header with the pointer to the returned SID.
    header: TOKEN_USER,
    /// Inline bytes from which a checked SID may be copied.
    sid_storage: [u8; MAX_SID_BYTES],
}

/// Read a complete supported SID only from the native call's own output buffer.
///
/// # Errors
/// Reject a failed query, an incomplete or malformed SID, or anonymous logon.
fn query_user_sid(token: BorrowedHandle<'_>) -> Result<[u8; MAX_SID_BYTES], PipeIdentityError> {
    let mut output: TokenUserBuffer = TokenUserBuffer {
        header: TOKEN_USER::default(),
        sid_storage: [0; MAX_SID_BYTES],
    };
    let capacity: u32 = u32::try_from(size_of::<TokenUserBuffer>())
        .map_err(|_| PipeIdentityError::InvalidTokenInformation)?;
    let mut written: u32 = 0;
    #[expect(
        unsafe_code,
        reason = "read TOKEN_USER and inline SID bytes into bounded storage"
    )]
    // SAFETY: The borrowed token remains live and the caller owns both outputs.
    // The repr(C) buffer starts with an aligned TOKEN_USER and fits the SID.
    let success: i32 = unsafe {
        GetTokenInformation(
            token.as_raw_handle(),
            TokenUser,
            (&raw mut output).cast(),
            capacity,
            &raw mut written,
        )
    };
    if success == 0 {
        return Err(last_error("GetTokenInformation(TokenUser)"));
    }

    let written: usize =
        usize::try_from(written).map_err(|_| PipeIdentityError::InvalidTokenInformation)?;
    if written > size_of::<TokenUserBuffer>() {
        return Err(PipeIdentityError::InvalidTokenInformation);
    }
    let written_sid_bytes: usize = written
        .checked_sub(offset_of!(TokenUserBuffer, sid_storage))
        .ok_or(PipeIdentityError::InvalidTokenInformation)?;
    let written_sid_bytes: usize = written_sid_bytes.min(MAX_SID_BYTES);
    let sid_address: usize = output.header.User.Sid.addr();
    let sid_offset: usize = sid_address
        .checked_sub(output.sid_storage.as_ptr().addr())
        .ok_or(PipeIdentityError::InvalidSid)?;
    if !sid_address.is_multiple_of(align_of::<SID>()) {
        return Err(PipeIdentityError::InvalidSid);
    }
    let sid_bytes: &[u8] = output
        .sid_storage
        .get(sid_offset..written_sid_bytes)
        .ok_or(PipeIdentityError::InvalidSid)?;
    let [1, subauthority_count, ..] = sid_bytes else {
        return Err(PipeIdentityError::InvalidSid);
    };
    let count: usize = usize::from(*subauthority_count);
    if count > MAX_SUBAUTHORITIES {
        return Err(PipeIdentityError::InvalidSid);
    }
    let length: usize = SID_HEADER_BYTES + count * size_of::<u32>();
    let sid: &[u8] = sid_bytes
        .get(..length)
        .ok_or(PipeIdentityError::InvalidSid)?;
    #[expect(
        unsafe_code,
        reason = "reject the Windows well-known anonymous logon SID"
    )]
    // SAFETY: The slice starts at the validated SID alignment and contains the
    // complete checked header and all declared subauthorities. The call reads it.
    let anonymous: i32 = unsafe { IsWellKnownSid(sid.as_ptr().cast_mut().cast(), WinAnonymousSid) };
    if anonymous != 0 {
        return Err(PipeIdentityError::AnonymousUser);
    }

    let mut identity: [u8; MAX_SID_BYTES] = [0; MAX_SID_BYTES];
    identity
        .get_mut(..length)
        .ok_or(PipeIdentityError::InvalidSid)?
        .copy_from_slice(sid);
    Ok(identity)
}

#[cfg(test)]
mod tests {
    //! Exercise current-token checks on the original connected fixture pipe.

    use std::num::NonZeroU32;

    use super::{AsHandle, PeerProcess, PrimaryTokenGeneration, RetainedFixtureClient};
    use crate::native_pipe_pair::{
        LabClientContext, LabPipeReader, LabRequestWriter, open_lab_identification_pipe,
    };

    /// The same live pipe, process and token must refuse a stale generation.
    /// # Errors
    /// Preserve live fixture setup or native token query refusal.
    #[test]
    #[ignore = "requires the admitted offline Windows broker identity fixture"]
    fn native_retained_client_rejects_a_different_primary_token_generation()
    -> Result<(), Box<dyn std::error::Error>> {
        let (reader, _writer): (LabPipeReader, LabRequestWriter) =
            open_lab_identification_pipe(0x4745_4e31, LabClientContext::Identification)?
                .into_transport();
        let pid: NonZeroU32 = NonZeroU32::new(std::process::id()).ok_or("fixture PID was zero")?;
        for (high, low) in [(1_i32, 0_u32), (0, 1)] {
            let mut client: RetainedFixtureClient =
                RetainedFixtureClient::new(PeerProcess::open(pid)?)?;
            let original: PrimaryTokenGeneration = client.token.generation;
            if !client.observe(reader.as_handle()) {
                return Err("the original client generation was refused".into());
            }

            // Change only the saved observation. The kernel token, principal,
            // process and connected server remain the healthy originals.
            client.token.generation = PrimaryTokenGeneration(original.0 ^ high, original.1 ^ low);
            if client.observe(reader.as_handle()) {
                return Err("a different generation on the same token was accepted".into());
            }
            client.token.generation = original;
            if client.observe(reader.as_handle()) {
                return Err("restoring the sample reopened a refused client".into());
            }
        }
        Ok(())
    }
}

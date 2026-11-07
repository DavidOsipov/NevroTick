// SPDX-License-Identifier: GPL-2.0-only
//! Private fixed self-token observation; no privilege or impersonation mutation.
//!
//! Only the synchronous owner thread calls this adapter. It invokes no callback
//! or application closure between its thread-token checks. Those checks reject
//! observed impersonation; they prove no continuous or future process-wide state.
//! Failed close may retain a handle until termination: no retry and no Ready.

use std::{
    ffi::c_void,
    mem,
    ptr::{self, NonNull},
};

use windows_platform::privilege_observation::{
    MAX_CAPTURE_BYTES, PrivilegeCaptureError, PrivilegeLuid, PrivilegeSnapshot, ServicePrivilegeIds,
};
use windows_sys::Win32::{
    Foundation::{CloseHandle, ERROR_NO_TOKEN, GetLastError, LUID},
    Security::{
        GetTokenInformation, LUID_AND_ATTRIBUTES, LookupPrivilegeValueW, SE_DEBUG_NAME,
        SE_IMPERSONATE_NAME, SE_SYSTEMTIME_NAME, TOKEN_PRIVILEGES, TOKEN_QUERY, TokenPrivileges,
    },
    System::Threading::{GetCurrentProcess, GetCurrentThread, OpenProcessToken, OpenThreadToken},
};

use crate::{
    NativeError, NativeOperation, TokenCleanupOutcome, TokenObservationCause,
    TokenObservationFailure, host::TokenObserver,
};

// Bind byte decoding and storage alignment to the actual selected generated ABI.
const _: () = assert!(size_of::<LUID>() == 8);
const _: () = assert!(size_of::<LUID_AND_ATTRIBUTES>() == 12);
const _: () = assert!(align_of::<TOKEN_PRIVILEGES>() == align_of::<u32>());
const _: () = assert!(mem::offset_of!(TOKEN_PRIVILEGES, Privileges) == 4);
const _: () = assert!(MAX_CAPTURE_BYTES == 385 * size_of::<u32>());

/// Fixed native producer consumed by both genuine laboratory role compositions.
pub(super) struct NativeTokenObserver;

impl TokenObserver for NativeTokenObserver {
    /// Return facts only after all queries, context checks and explicit closes succeed.
    fn observe(&mut self) -> Result<PrivilegeSnapshot, TokenObservationFailure> {
        let mut primary = None;
        let mut unexpected_thread = None;
        let capture = (|| {
            require_no_thread_token(&mut unexpected_thread)?;
            primary = Some(open_primary()?);
            let ids = resolve_ids()?;
            let token = primary
                .as_ref()
                .ok_or(TokenObservationCause::InvalidHandle)?;
            let snapshot = capture_privileges(token, ids)?;
            require_no_thread_token(&mut unexpected_thread)?;
            Ok(snapshot)
        })();
        // Both explicit closes run even after a capture/context failure. Taking
        // custody first prevents Drop or subsequent code from retrying a failure.
        let thread_cleanup = finish(&mut unexpected_thread);
        let primary_cleanup = finish(&mut primary);
        let cause = match capture {
            Err(cause) => Some(cause),
            Ok(_) => match (thread_cleanup, primary_cleanup) {
                (TokenCleanupOutcome::Failed(error), _)
                | (_, TokenCleanupOutcome::Failed(error)) => {
                    Some(TokenObservationCause::Native(error))
                }
                _ => None,
            },
        };
        match (capture, cause) {
            (_, Some(cause)) | (Err(cause), None) => Err(TokenObservationFailure::new(
                cause,
                primary_cleanup,
                thread_cleanup,
            )),
            (Ok(snapshot), None) => Ok(snapshot),
        }
    }
}

/// Unique close custody for a non-null real handle returned by a successful open.
struct QueryToken {
    /// Taken before every close attempt; pseudo-handles are never stored here.
    handle: Option<NonNull<c_void>>,
}

impl QueryToken {
    /// Accept only successful API output, refusing a null success representation.
    ///
    /// # Errors
    /// A null success output establishes no owned handle and is indeterminate.
    fn new(handle: *mut c_void) -> Result<Self, TokenObservationCause> {
        let handle = NonNull::new(handle).ok_or(TokenObservationCause::InvalidHandle)?;
        Ok(Self {
            handle: Some(handle),
        })
    }

    /// Borrow the still-owned token handle for a synchronous query only.
    ///
    /// # Errors
    /// Refuse a slot whose unique close custody has already been taken.
    fn pointer(&self) -> Result<*mut c_void, TokenObservationCause> {
        self.handle
            .map(NonNull::as_ptr)
            .ok_or(TokenObservationCause::InvalidHandle)
    }

    /// Attempt close once. Failure expressly does not establish native release.
    fn finish(&mut self) -> TokenCleanupOutcome {
        let Some(handle) = self.handle.take() else {
            return TokenCleanupOutcome::NotOpened;
        };
        #[expect(
            unsafe_code,
            reason = "release the uniquely owned real token handle once"
        )]
        // SAFETY: successful open supplied this real non-null handle, still
        // exclusively owned; taking it above prevents every repeated close.
        let closed = unsafe { CloseHandle(handle.as_ptr()) };
        if closed == 0 {
            TokenCleanupOutcome::Failed(last_error(NativeOperation::TokenClose))
        } else {
            TokenCleanupOutcome::Closed
        }
    }
}

impl Drop for QueryToken {
    /// Best-effort unwind cleanup only; explicit failed closes are never retried.
    /// Panic/abort/native fault is not a successful observation or release claim.
    fn drop(&mut self) {
        let _cleanup = self.finish();
    }
}

/// Explicitly close an opened slot once, preserving absence of any opened handle.
fn finish(slot: &mut Option<QueryToken>) -> TokenCleanupOutcome {
    slot.take()
        .map_or(TokenCleanupOutcome::NotOpened, |mut token| token.finish())
}

/// Capture the failing API's current thread error immediately, even when zero.
fn last_error(operation: NativeOperation) -> NativeError {
    #[expect(
        unsafe_code,
        reason = "read the immediately failing native call's thread error"
    )]
    // SAFETY: GetLastError has no pointer or lifetime preconditions.
    let code = unsafe { GetLastError() };
    NativeError::new(operation, code)
}

/// Accept only definite absence; an opened thread token remains owned for cleanup.
///
/// # Errors
/// Reject an opened token, invalid success output or every native error except
/// `ERROR_NO_TOKEN`. Failed queries never establish absent impersonation.
fn require_no_thread_token(slot: &mut Option<QueryToken>) -> Result<(), TokenObservationCause> {
    let mut raw = ptr::null_mut();
    #[expect(
        unsafe_code,
        reason = "obtain only the current synchronous owner's pseudo-handle"
    )]
    // SAFETY: GetCurrentThread takes no arguments and grants no owned resource.
    let thread = unsafe { GetCurrentThread() };
    #[expect(
        unsafe_code,
        reason = "query the fixed current thread token without changing it"
    )]
    // SAFETY: current-thread pseudo-handle is valid; raw is writable output.
    // TOKEN_QUERY is the only requested right; OpenAsSelf changes access-check
    // context, never the token. No borrowed pointer survives this call.
    let opened = unsafe { OpenThreadToken(thread, TOKEN_QUERY, 1, &raw mut raw) };
    if opened == 0 {
        let error = last_error(NativeOperation::ThreadToken);
        if error.code() == ERROR_NO_TOKEN {
            return Ok(());
        }
        return Err(TokenObservationCause::Native(error));
    }
    *slot = Some(QueryToken::new(raw)?);
    Err(TokenObservationCause::UnexpectedThreadToken)
}

/// Open only this process primary token, with query rights and no caller handle.
///
/// # Errors
/// Preserve the immediately observed native failure or refuse null success output.
fn open_primary() -> Result<QueryToken, TokenObservationCause> {
    let mut raw = ptr::null_mut();
    #[expect(unsafe_code, reason = "obtain only this process pseudo-handle")]
    // SAFETY: GetCurrentProcess takes no arguments and creates no owned handle.
    let process = unsafe { GetCurrentProcess() };
    #[expect(unsafe_code, reason = "open only the current primary token for query")]
    // SAFETY: the fixed pseudo-handle is valid and raw is writable local output.
    let opened = unsafe { OpenProcessToken(process, TOKEN_QUERY, &raw mut raw) };
    if opened == 0 {
        return Err(TokenObservationCause::Native(last_error(
            NativeOperation::ProcessToken,
        )));
    }
    QueryToken::new(raw)
}

/// Resolve every fixed local name; partial or duplicate lookup results prove nothing.
///
/// # Errors
/// Reject any failed fixed-name lookup or collision among the three complete LUIDs.
fn resolve_ids() -> Result<ServicePrivilegeIds, TokenObservationCause> {
    let mut values = [PrivilegeLuid::new(0, 0); 3];
    for (slot, name) in [SE_SYSTEMTIME_NAME, SE_DEBUG_NAME, SE_IMPERSONATE_NAME]
        .into_iter()
        .enumerate()
    {
        let mut luid = LUID::default();
        #[expect(
            unsafe_code,
            reason = "resolve only one of three static local privilege names"
        )]
        // SAFETY: null selects this machine; generated constants are immutable
        // NUL-terminated UTF-16 names; luid is writable correctly aligned output.
        let found = unsafe { LookupPrivilegeValueW(ptr::null(), name, &raw mut luid) };
        if found == 0 {
            return Err(TokenObservationCause::Native(last_error(
                NativeOperation::PrivilegeLookup,
            )));
        }
        values[slot] = PrivilegeLuid::new(luid.LowPart, luid.HighPart);
    }
    ServicePrivilegeIds::new(values).map_err(TokenObservationCause::Capture)
}

/// Capture once into initialized, correctly aligned fixed storage; never retry.
/// Failed API output, including a zeroed buffer, never reaches the pure decoder.
///
/// # Errors
/// Reject missing handle custody, native query failure (including insufficient
/// fixed storage), excessive returned length and every checked-decoder refusal.
fn capture_privileges(
    token: &QueryToken,
    ids: ServicePrivilegeIds,
) -> Result<PrivilegeSnapshot, TokenObservationCause> {
    let mut words = [0_u32; 385];
    let mut returned = 0_u32;
    let handle = token.pointer()?;
    #[expect(
        unsafe_code,
        reason = "read bounded privileges from the uniquely held primary token"
    )]
    // SAFETY: words provides 1540 initialized writable bytes with ABI-asserted
    // alignment, the queried real handle remains owned, and returned is writable.
    // The synchronous API retains none of these pointers after return.
    let captured = unsafe {
        GetTokenInformation(
            handle,
            TokenPrivileges,
            words.as_mut_ptr().cast(),
            1540,
            &raw mut returned,
        )
    };
    if captured == 0 {
        return Err(TokenObservationCause::Native(last_error(
            NativeOperation::PrivilegeQuery,
        )));
    }
    let mut bytes = [0_u8; MAX_CAPTURE_BYTES];
    for (chunk, word) in bytes.as_chunks_mut::<4>().0.iter_mut().zip(words) {
        chunk.copy_from_slice(&word.to_le_bytes());
    }
    let returned = usize::try_from(returned)
        .map_err(|_| TokenObservationCause::Capture(PrivilegeCaptureError::Bounds))?;
    let complete = bytes.get(..returned).ok_or(TokenObservationCause::Capture(
        PrivilegeCaptureError::Bounds,
    ))?;
    PrivilegeSnapshot::decode(complete, ids).map_err(TokenObservationCause::Capture)
}

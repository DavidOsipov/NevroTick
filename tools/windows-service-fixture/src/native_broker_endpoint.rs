// SPDX-License-Identifier: GPL-2.0-only
//! Fixed laboratory broker reservation and one accept with retained native completion.

use std::{
    cell::UnsafeCell,
    marker::PhantomPinned,
    os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle, OwnedHandle},
    pin::Pin,
    ptr,
    task::Poll,
};

use windows_sys::{
    Win32::{
        Foundation::{
            CloseHandle, ERROR_INVALID_HANDLE, ERROR_INVALID_PARAMETER, ERROR_IO_INCOMPLETE,
            ERROR_IO_PENDING, ERROR_NOT_FOUND, ERROR_OPERATION_ABORTED, ERROR_PIPE_CONNECTED,
            GetLastError, HANDLE, INVALID_HANDLE_VALUE,
        },
        Security::{
            ACL, ACL_REVISION, InitializeAcl, InitializeSecurityDescriptor, PSECURITY_DESCRIPTOR,
            SE_DACL_PROTECTED, SECURITY_ATTRIBUTES, SECURITY_DESCRIPTOR,
            SetSecurityDescriptorControl, SetSecurityDescriptorDacl,
        },
        Storage::FileSystem::{
            FILE_FLAG_FIRST_PIPE_INSTANCE, FILE_FLAG_OVERLAPPED, PIPE_ACCESS_DUPLEX,
        },
        System::{
            IO::{CancelIoEx, GetOverlappedResult, OVERLAPPED},
            Pipes::{ConnectNamedPipe, CreateNamedPipeW, PIPE_REJECT_REMOTE_CLIENTS},
            Threading::CreateEventW,
        },
    },
    core::BOOL,
};

use crate::{
    NativeError, NativeOperation,
    broker_endpoint::{AcceptCompletion, AcceptStart, Reservation},
};

/// UTF-16 projection of `\\.\pipe\NevroTick-Lab-S02-Broker-Reservation` and its terminator.
const RESERVATION_NAME: [u16; 46] = [
    92, 92, 46, 92, 112, 105, 112, 101, 92, 78, 101, 118, 114, 111, 84, 105, 99, 107, 45, 76, 97,
    98, 45, 83, 48, 50, 45, 66, 114, 111, 107, 101, 114, 45, 82, 101, 115, 101, 114, 118, 97, 116,
    105, 111, 110, 0,
];

/// The SDK descriptor revision used by `InitializeSecurityDescriptor`.
const DESCRIPTOR_REVISION: u32 = 1;
/// A descriptor storage length outside the Win32 argument range.
const INVALID_DESCRIPTOR_SIZE: NativeError = NativeError::new(
    NativeOperation::BrokerEndpointDescriptor,
    ERROR_INVALID_PARAMETER,
);

/// DWORD-aligned storage for the empty ACL referenced by the absolute descriptor.
#[repr(C, align(4))]
struct EmptyAcl(ACL);

const _: () = assert!(size_of::<EmptyAcl>() == 8);
const _: () = assert!(size_of::<SECURITY_ATTRIBUTES>() == 24);

/// Submission and cancellation history of the sole exact native record.
#[derive(Clone, Copy, Eq, PartialEq)]
enum AcceptState {
    /// The event and record are initialized; no accept has been attempted.
    Ready,
    /// An accept may still be outstanding on the original pipe.
    Queued,
    /// The one cancellation request has been attempted; completion is still owed.
    Cancelling,
    /// Submission did not queue or a terminal result was observed.
    Retired,
}

/// Pinned, single-use storage for the only kernel-written record.
struct AcceptStorage {
    /// Interior mutability permits native writes while the Rust owner is retained.
    overlapped: UnsafeCell<OVERLAPPED>,
    /// Prevent moving the submitted storage out of its allocation.
    _pin: PhantomPinned,
}

/// Own the allocation independently of the Rust state used to decide release.
struct AcceptRecord {
    /// Keep the original allocation alive and still during all pending work.
    storage: Pin<Box<AcceptStorage>>,
    /// Rust-only state independent of the record written by Windows.
    state: AcceptState,
}

impl AcceptRecord {
    /// Borrow the exact pointer for submission, cancellation, or observation.
    fn pointer(&self) -> *mut OVERLAPPED {
        self.storage.as_ref().get_ref().overlapped.get()
    }
}

impl Drop for AcceptRecord {
    /// Fail fast on invalid destruction while the kernel may still write.
    fn drop(&mut self) {
        if matches!(self.state, AcceptState::Queued | AcceptState::Cancelling) {
            // The service owner retains this allocation during ordinary Stop.
            // An invalid drop cannot release pending storage or claim Stopped.
            std::process::abort();
        }
    }
}

/// Unique custody of the pipe and its sole accept record and event.
#[must_use = "retain the original reservation and any queued accept through explicit release"]
pub(super) struct NativeBrokerEndpoint {
    /// Drop the record first so invalid destruction cannot release its handles.
    accept: AcceptRecord,
    /// The noninherited manual-reset completion event belongs to this accept.
    event: OwnedHandle,
    /// The creation result, with best-effort release if explicit close is bypassed.
    handle: OwnedHandle,
}

impl NativeBrokerEndpoint {
    /// Prepare a local overlapped instance and its independent completion event.
    ///
    /// # Errors
    /// Reject an unrepresentable storage size and preserve the first descriptor
    /// or creation failure. A collision leaves the existing object untouched,
    /// and the idle provisional event drops without a submitted operation.
    pub(super) fn acquire() -> Result<Self, NativeError> {
        let acl_bytes: u32 =
            u32::try_from(size_of::<EmptyAcl>()).map_err(|_| INVALID_DESCRIPTOR_SIZE)?;
        let attributes_bytes: u32 =
            u32::try_from(size_of::<SECURITY_ATTRIBUTES>()).map_err(|_| INVALID_DESCRIPTOR_SIZE)?;
        let mut acl: EmptyAcl = EmptyAcl(ACL::default());
        let mut descriptor: SECURITY_DESCRIPTOR = SECURITY_DESCRIPTOR::default();
        let descriptor_ptr: PSECURITY_DESCRIPTOR = (&raw mut descriptor).cast();

        #[expect(unsafe_code, reason = "initialize one aligned empty ACL header")]
        // SAFETY: The aligned field provides a writable DWORD-sized ACL header
        // buffer. No ACE storage is needed, and the buffer outlives the descriptor.
        let initialized: BOOL = unsafe { InitializeAcl(&raw mut acl.0, acl_bytes, ACL_REVISION) };
        if initialized == 0 {
            return Err(last_error(NativeOperation::BrokerEndpointDescriptor));
        }

        #[expect(unsafe_code, reason = "initialize one stack-owned absolute descriptor")]
        // SAFETY: The pointer addresses a writable, aligned SECURITY_DESCRIPTOR.
        let initialized: BOOL =
            unsafe { InitializeSecurityDescriptor(descriptor_ptr, DESCRIPTOR_REVISION) };
        if initialized == 0 {
            return Err(last_error(NativeOperation::BrokerEndpointDescriptor));
        }

        #[expect(unsafe_code, reason = "install only the present empty DACL")]
        // SAFETY: The descriptor is an initialized absolute descriptor, and the
        // referenced ACL remains initialized and unmoved through creation.
        let assigned: BOOL =
            unsafe { SetSecurityDescriptorDacl(descriptor_ptr, 1, &raw const acl.0, 0) };
        if assigned == 0 {
            return Err(last_error(NativeOperation::BrokerEndpointDescriptor));
        }

        #[expect(unsafe_code, reason = "prevent inherited ACEs on the reservation")]
        // SAFETY: The descriptor is initialized and SE_DACL_PROTECTED is a
        // supported control mask for this operation.
        let protected: BOOL = unsafe {
            SetSecurityDescriptorControl(descriptor_ptr, SE_DACL_PROTECTED, SE_DACL_PROTECTED)
        };
        if protected == 0 {
            return Err(last_error(NativeOperation::BrokerEndpointDescriptor));
        }

        let attributes: SECURITY_ATTRIBUTES = SECURITY_ATTRIBUTES {
            nLength: attributes_bytes,
            lpSecurityDescriptor: descriptor_ptr,
            bInheritHandle: 0,
        };
        let event: OwnedHandle = create_accept_event()?;
        let accept: AcceptRecord = AcceptRecord {
            storage: Box::pin(AcceptStorage {
                overlapped: UnsafeCell::new(OVERLAPPED {
                    hEvent: event.as_raw_handle(),
                    ..OVERLAPPED::default()
                }),
                _pin: PhantomPinned,
            }),
            state: AcceptState::Ready,
        };
        #[expect(unsafe_code, reason = "reserve only the fixed local lab pipe name")]
        // SAFETY: The name has a terminating NUL. The initialized attributes,
        // descriptor and ACL remain valid through this synchronous call.
        let handle: HANDLE = unsafe {
            CreateNamedPipeW(
                RESERVATION_NAME.as_ptr(),
                PIPE_ACCESS_DUPLEX | FILE_FLAG_FIRST_PIPE_INSTANCE | FILE_FLAG_OVERLAPPED,
                PIPE_REJECT_REMOTE_CLIENTS,
                1,
                0,
                0,
                0,
                &raw const attributes,
            )
        };
        if handle == INVALID_HANDLE_VALUE {
            return Err(last_error(NativeOperation::BrokerEndpointCreate));
        }
        if handle.is_null() {
            return Err(NativeError::new(
                NativeOperation::BrokerEndpointCreate,
                ERROR_INVALID_HANDLE,
            ));
        }

        #[expect(unsafe_code, reason = "adopt the private native server handle")]
        // SAFETY: A successful creation returns a uniquely owned real handle
        // that must be released with CloseHandle.
        let handle: OwnedHandle = unsafe { OwnedHandle::from_raw_handle(handle) };
        Ok(Self {
            accept,
            event,
            handle,
        })
    }

    /// Release both original handles once, with no outstanding native I/O.
    ///
    /// # Errors
    /// Preserve the first close failure with release unconfirmed. Consuming the
    /// handles before the calls prevents repeating an uncertain release.
    pub(super) fn close(self) -> Result<(), NativeError> {
        let Self {
            accept,
            event,
            handle,
        } = self;
        drop(accept);
        let pipe_result: Result<(), NativeError> = close_handle(handle);
        let event_result: Result<(), NativeError> = close_handle(event);
        pipe_result.and(event_result)
    }
}

impl Reservation for NativeBrokerEndpoint {
    /// Submit once using the prepared pipe and event. Keep storage on pending.
    ///
    /// # Errors
    /// Refuses a repeated submission and preserves any native submission error.
    fn begin_accept(&mut self) -> Result<AcceptStart, NativeError> {
        let operation: NativeOperation = NativeOperation::BrokerEndpointAcceptSubmit;
        if self.accept.state != AcceptState::Ready {
            return Err(NativeError::new(operation, ERROR_INVALID_PARAMETER));
        }
        self.accept.state = AcceptState::Queued;
        #[expect(
            unsafe_code,
            reason = "submit one accept with the owned overlapped record"
        )]
        // SAFETY: The pipe was opened overlapped. The record is pinned, is
        // exclusively reserved for this operation and retains its manual-reset
        // event with the original pipe until observed completion.
        let connected: BOOL =
            unsafe { ConnectNamedPipe(self.handle.as_raw_handle(), self.accept.pointer()) };
        if connected != 0 {
            self.accept.state = AcceptState::Retired;
            return Ok(AcceptStart::Connected);
        }
        let error: NativeError = last_error(operation);
        if error.code() == ERROR_IO_PENDING {
            return Ok(AcceptStart::Queued);
        }
        self.accept.state = AcceptState::Retired;
        if error.code() == ERROR_PIPE_CONNECTED {
            return Ok(AcceptStart::Connected);
        }
        Err(error)
    }

    /// Request cancellation once for the exact queued accept. Retain custody.
    ///
    /// # Errors
    /// Refuses an ineligible request or reports its original native error.
    fn request_accept_cancel(&mut self) -> Result<(), NativeError> {
        let operation: NativeOperation = NativeOperation::BrokerEndpointAcceptCancel;
        if self.accept.state != AcceptState::Queued {
            return Err(NativeError::new(operation, ERROR_INVALID_PARAMETER));
        }
        self.accept.state = AcceptState::Cancelling;
        #[expect(
            unsafe_code,
            reason = "target the exact original accept without freeing it"
        )]
        // SAFETY: This original pipe and nonnull record remain owned and pinned.
        // Cancellation requests no other operation and does not acknowledge release.
        let requested: BOOL =
            unsafe { CancelIoEx(self.handle.as_raw_handle(), self.accept.pointer()) };
        if requested != 0 {
            return Ok(());
        }
        let error: NativeError = last_error(operation);
        if error.code() == ERROR_NOT_FOUND {
            return Ok(());
        }
        Err(error)
    }

    /// Observe the exact queued accept without waiting or reading its byte count.
    ///
    /// # Errors
    /// Refuses an unqueued observation and preserves custody on unknown failures.
    fn poll_accept(&mut self) -> Result<Poll<AcceptCompletion>, NativeError> {
        let operation: NativeOperation = NativeOperation::BrokerEndpointAcceptCompletion;
        if !matches!(
            self.accept.state,
            AcceptState::Queued | AcceptState::Cancelling
        ) {
            return Err(NativeError::new(operation, ERROR_INVALID_PARAMETER));
        }
        let mut ignored: u32 = 0;
        #[expect(
            unsafe_code,
            reason = "observe only the exact pending accept without waiting"
        )]
        // SAFETY: The same handle, event and pinned record from submission are
        // held. The scratch count is writable for this synchronous observation.
        let complete: BOOL = unsafe {
            GetOverlappedResult(
                self.handle.as_raw_handle(),
                self.accept.pointer(),
                &raw mut ignored,
                0,
            )
        };
        if complete != 0 {
            self.accept.state = AcceptState::Retired;
            return Ok(Poll::Ready(AcceptCompletion::Connected));
        }
        let error: NativeError = last_error(operation);
        match error.code() {
            ERROR_IO_INCOMPLETE => Ok(Poll::Pending),
            ERROR_OPERATION_ABORTED => {
                self.accept.state = AcceptState::Retired;
                Ok(Poll::Ready(AcceptCompletion::Cancelled))
            }
            _ => Err(error),
        }
    }

    /// Preserve the actual release result under the serialized host.
    fn close(self) -> Result<(), NativeError> {
        Self::close(self)
    }
}

/// Create the independent noninherited manual-reset event for one accept.
///
/// # Errors
/// Preserve the original failure and acquire no event on refusal.
fn create_accept_event() -> Result<OwnedHandle, NativeError> {
    #[expect(unsafe_code, reason = "create an unnamed private completion event")]
    // SAFETY: Null attributes and name select noninheritance and an unnamed
    // event. The other arguments select manual reset and initially unsignaled.
    let handle: HANDLE = unsafe { CreateEventW(ptr::null(), 1, 0, ptr::null()) };
    if handle.is_null() {
        return Err(last_error(NativeOperation::BrokerEndpointAcceptEvent));
    }
    #[expect(unsafe_code, reason = "adopt the successful event creation handle")]
    // SAFETY: Successful creation returns a unique handle released by CloseHandle.
    let event: OwnedHandle = unsafe { OwnedHandle::from_raw_handle(handle) };
    Ok(event)
}

/// Attempt one explicit native release after there is no possible pending use.
///
/// # Errors
/// Preserve the exact error. A failed attempt cannot establish release or retry.
fn close_handle(owner: OwnedHandle) -> Result<(), NativeError> {
    let handle: HANDLE = owner.into_raw_handle();
    #[expect(
        unsafe_code,
        reason = "observe explicit release of each original handle"
    )]
    // SAFETY: The private caller already retired all native work involving this
    // real handle. Consuming the sole owner leaves exactly one close attempt.
    let closed: BOOL = unsafe { CloseHandle(handle) };
    if closed == 0 {
        return Err(last_error(NativeOperation::BrokerEndpointClose));
    }
    Ok(())
}

/// Capture the immediately preceding Win32 failure before intervening calls.
fn last_error(operation: NativeOperation) -> NativeError {
    #[expect(
        unsafe_code,
        reason = "retain the native failure on the calling thread"
    )]
    // SAFETY: GetLastError has no pointer or lifetime preconditions.
    let code: u32 = unsafe { GetLastError() };
    NativeError::new(operation, code)
}

#[cfg(test)]
mod tests {
    use std::{
        error::Error,
        os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle},
        ptr,
        sync::{Mutex, MutexGuard},
        task::Poll,
    };

    use windows_sys::Win32::{
        Foundation::{
            ERROR_ACCESS_DENIED, GetLastError, HANDLE, INVALID_HANDLE_VALUE, WAIT_OBJECT_0,
        },
        Storage::FileSystem::{
            CreateFileW, FILE_READ_DATA, FILE_WRITE_DATA, OPEN_EXISTING, SECURITY_IDENTIFICATION,
            SECURITY_SQOS_PRESENT, SYNCHRONIZE,
        },
        System::Threading::WaitForSingleObject,
    };

    use super::NativeBrokerEndpoint;
    use crate::{
        NativeError, NativeOperation,
        broker_endpoint::{AcceptCompletion, AcceptStart, Reservation},
    };

    /// Serialize the only tests using the same process's fixed reservation name.
    static RESERVATION_TEST: Mutex<()> = Mutex::new(());

    /// Observe denial on the actual reserved name, then explicitly retire and reacquire it.
    ///
    /// # Errors
    /// Propagate failed acquisition or close, and refuse unexpected client access.
    /// # Panics
    /// Panics on any error code other than the expected access or collision refusal.
    #[test]
    fn reservation_denies_data_and_releases_the_fixed_name() -> Result<(), Box<dyn Error>> {
        let _name: MutexGuard<'_, ()> = RESERVATION_TEST
            .lock()
            .map_err(|_| "reservation test lock poisoned")?;
        let endpoint: NativeBrokerEndpoint = NativeBrokerEndpoint::acquire()?;
        let name: Vec<u16> = r"\\.\pipe\NevroTick-Lab-S02-Broker-Reservation"
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();

        for access in [FILE_READ_DATA, FILE_WRITE_DATA] {
            #[expect(unsafe_code, reason = "probe only data access on the owned lab object")]
            // SAFETY: The independent local name is NUL-terminated and valid
            // throughout the call. No handle inheritance or data I/O is requested.
            let opened: HANDLE = unsafe {
                CreateFileW(
                    name.as_ptr(),
                    access | SYNCHRONIZE,
                    0,
                    ptr::null(),
                    OPEN_EXISTING,
                    SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION,
                    ptr::null_mut(),
                )
            };
            if opened != INVALID_HANDLE_VALUE {
                if !opened.is_null() {
                    #[expect(unsafe_code, reason = "release any unexpectedly granted probe handle")]
                    // SAFETY: The successful call returned a unique real handle.
                    let unexpected: OwnedHandle = unsafe { OwnedHandle::from_raw_handle(opened) };
                    drop(unexpected);
                }
                return Err("reservation allowed a client data open".into());
            }
            #[expect(unsafe_code, reason = "observe the direct client refusal")]
            // SAFETY: GetLastError takes no arguments and follows the failed call.
            let refused: u32 = unsafe { GetLastError() };
            assert_eq!(refused, ERROR_ACCESS_DENIED);
        }

        assert_eq!(
            NativeBrokerEndpoint::acquire().err(),
            Some(NativeError::new(
                NativeOperation::BrokerEndpointCreate,
                ERROR_ACCESS_DENIED,
            )),
        );

        endpoint.close()?;
        let successor: NativeBrokerEndpoint = NativeBrokerEndpoint::acquire()?;
        successor.close()?;
        Ok(())
    }

    /// Observe a real pending and cancelled accept, then release and reacquire.
    ///
    /// # Errors
    /// Propagates native failures. If completion is still unconfirmed, the
    /// fixture's invalid-drop safeguard terminates the isolated test process.
    /// # Panics
    /// Fails on unexpected readiness, missing event signal or a wrong outcome.
    #[test]
    fn queued_accept_cancels_to_completion_before_release() -> Result<(), Box<dyn Error>> {
        let _name: MutexGuard<'_, ()> = RESERVATION_TEST
            .lock()
            .map_err(|_| "reservation test lock poisoned")?;
        let mut endpoint: NativeBrokerEndpoint = NativeBrokerEndpoint::acquire()?;
        assert!(matches!(endpoint.begin_accept()?, AcceptStart::Queued));
        assert!(matches!(endpoint.poll_accept()?, Poll::Pending));
        endpoint.request_accept_cancel()?;

        #[expect(
            unsafe_code,
            reason = "observe the owned accept event within the test bound"
        )]
        // SAFETY: The event handle is live and private. This finite wait only
        // observes the kernel signal. The exact result still comes from poll.
        let observed: u32 = unsafe { WaitForSingleObject(endpoint.event.as_raw_handle(), 2_000) };
        let completion: Poll<AcceptCompletion> = endpoint.poll_accept()?;
        assert!(
            completion.is_ready(),
            "accept still pending after event wait status {observed}"
        );
        endpoint.close()?;
        assert_eq!(observed, WAIT_OBJECT_0);
        assert!(matches!(
            completion,
            Poll::Ready(AcceptCompletion::Cancelled)
        ));

        NativeBrokerEndpoint::acquire()?.close()?;
        Ok(())
    }
}

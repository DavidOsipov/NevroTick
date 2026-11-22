// SPDX-License-Identifier: GPL-2.0-only
//! Fixed laboratory broker name reservation with no connection or data operations.

use std::os::windows::io::{FromRawHandle, IntoRawHandle, OwnedHandle};

use windows_sys::{
    Win32::{
        Foundation::{
            CloseHandle, ERROR_INVALID_HANDLE, ERROR_INVALID_PARAMETER, GetLastError, HANDLE,
            INVALID_HANDLE_VALUE,
        },
        Security::{
            ACL, ACL_REVISION, InitializeAcl, InitializeSecurityDescriptor, PSECURITY_DESCRIPTOR,
            SE_DACL_PROTECTED, SECURITY_ATTRIBUTES, SECURITY_DESCRIPTOR,
            SetSecurityDescriptorControl, SetSecurityDescriptorDacl,
        },
    },
    core::BOOL,
};

use crate::{NativeError, NativeOperation, broker_endpoint::Reservation};

/// Open the server side for duplex data, which remains inaccessible to clients.
const PIPE_ACCESS_DUPLEX: u32 = 0x0000_0003;
/// Refuse a colliding first instance without retry or adoption.
const FILE_FLAG_FIRST_PIPE_INSTANCE: u32 = 0x0008_0000;
/// Exclude remote clients regardless of the explicit access list.
const PIPE_REJECT_REMOTE_CLIENTS: u32 = 0x0000_0008;

#[link(name = "kernel32")]
#[expect(
    unsafe_code,
    reason = "one pinned x64 Win32 pipe acquisition declaration"
)]
unsafe extern "system" {
    /// Create one non-inherited server handle using synchronous input storage.
    fn CreateNamedPipeW(
        name: *const u16,
        open_mode: u32,
        pipe_mode: u32,
        max_instances: u32,
        out_buffer_size: u32,
        in_buffer_size: u32,
        default_timeout: u32,
        attributes: *const SECURITY_ATTRIBUTES,
    ) -> HANDLE;
}

// Compare this narrow normal-build ABI and flags to the existing dev bindings.
#[cfg(test)]
const _: unsafe extern "system" fn(
    *const u16,
    u32,
    u32,
    u32,
    u32,
    u32,
    u32,
    *const SECURITY_ATTRIBUTES,
) -> HANDLE = windows_sys::Win32::System::Pipes::CreateNamedPipeW;

#[cfg(test)]
const _: () = {
    assert!(PIPE_ACCESS_DUPLEX == windows_sys::Win32::Storage::FileSystem::PIPE_ACCESS_DUPLEX);
    assert!(
        FILE_FLAG_FIRST_PIPE_INSTANCE
            == windows_sys::Win32::Storage::FileSystem::FILE_FLAG_FIRST_PIPE_INSTANCE
    );
    assert!(
        PIPE_REJECT_REMOTE_CLIENTS == windows_sys::Win32::System::Pipes::PIPE_REJECT_REMOTE_CLIENTS
    );
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

/// Unique custody of the original noninherited first-instance reservation.
#[must_use = "retain the original reservation until explicit release"]
pub(super) struct NativeBrokerEndpoint {
    /// The creation result, with best-effort release if explicit close is bypassed.
    handle: OwnedHandle,
}

impl NativeBrokerEndpoint {
    /// Create one local server instance with a present protected DACL and no ACEs.
    ///
    /// # Errors
    /// Reject an unrepresentable storage size and preserve the first descriptor
    /// or creation failure. A collision leaves the existing object untouched
    /// and issues no reservation.
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
        #[expect(unsafe_code, reason = "reserve only the fixed local lab pipe name")]
        // SAFETY: The name has a terminating NUL. The initialized attributes,
        // descriptor and ACL remain valid through this synchronous call.
        let handle: HANDLE = unsafe {
            CreateNamedPipeW(
                RESERVATION_NAME.as_ptr(),
                PIPE_ACCESS_DUPLEX | FILE_FLAG_FIRST_PIPE_INSTANCE,
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
        Ok(Self { handle })
    }

    /// Consume the owner and acknowledge the original `CloseHandle` result.
    ///
    /// # Errors
    /// Preserve the exact close failure with release unconfirmed. Consuming the
    /// handle before the call prevents repeating an uncertain release.
    pub(super) fn close(self) -> Result<(), NativeError> {
        let handle: HANDLE = self.handle.into_raw_handle();
        #[expect(
            unsafe_code,
            reason = "observe explicit release of the original reservation"
        )]
        // SAFETY: This real handle came from successful creation. Its unique
        // Rust owner has been consumed and there is no outstanding native I/O.
        let closed: BOOL = unsafe { CloseHandle(handle) };
        if closed == 0 {
            return Err(last_error(NativeOperation::BrokerEndpointClose));
        }
        Ok(())
    }
}

impl Reservation for NativeBrokerEndpoint {
    /// Preserve the actual release result under the serialized host.
    fn close(self) -> Result<(), NativeError> {
        Self::close(self)
    }
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
        os::windows::io::{FromRawHandle, OwnedHandle},
        ptr,
    };

    use windows_sys::Win32::{
        Foundation::{ERROR_ACCESS_DENIED, GetLastError, HANDLE, INVALID_HANDLE_VALUE},
        Storage::FileSystem::{
            CreateFileW, FILE_READ_DATA, FILE_WRITE_DATA, OPEN_EXISTING, SECURITY_IDENTIFICATION,
            SECURITY_SQOS_PRESENT, SYNCHRONIZE,
        },
    };

    use super::NativeBrokerEndpoint;
    use crate::{NativeError, NativeOperation};

    /// Observe denial on the actual reserved name, then explicitly retire and reacquire it.
    ///
    /// # Errors
    /// Propagate failed acquisition or close, and refuse unexpected client access.
    /// # Panics
    /// Panics on any error code other than the expected access or collision refusal.
    #[test]
    fn reservation_denies_data_and_releases_the_fixed_name() -> Result<(), Box<dyn Error>> {
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
}

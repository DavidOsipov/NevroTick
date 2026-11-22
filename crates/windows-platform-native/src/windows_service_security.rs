// SPDX-License-Identifier: GPL-2.0-only
//! Query only the owner and discretionary ACL of a local SCM service object.

use core::ffi::c_void;

use crate::service_security::{ReportedServiceSecurity, SERVICE_SECURITY_BYTES};
use crate::service_status::ServiceKey;
use crate::windows_service_status::{OwnedScHandle, ServiceQueryError, last_error};

/// Fixed Windows request scope. No group or system audit ACL is requested.
const SECURITY_INFORMATION: u32 = 0x0000_0001 | 0x0000_0004;

/// Aligned writable storage for the self-relative descriptor from SCM.
#[repr(C, align(8))]
struct DescriptorBuffer([u8; SERVICE_SECURITY_BYTES]);

const _: [(); 8192] = [(); size_of::<DescriptorBuffer>()];

#[link(name = "advapi32")]
#[expect(unsafe_code, reason = "exact winsvc.h service security query ABI")]
unsafe extern "system" {
    /// Retrieve the selected security information using the caller's fixed buffer.
    fn QueryServiceObjectSecurity(
        service: *mut c_void,
        information: u32,
        descriptor: *mut c_void,
        buffer_size: u32,
        bytes_needed: *mut u32,
    ) -> i32;
}

/// Own a local service handle opened solely with `READ_CONTROL`.
#[must_use = "retain the security query handle and close it to verify retirement"]
pub struct ServiceSecurityQuery {
    /// The retained service object opened for the selected security observation.
    service: OwnedScHandle,
}

impl ServiceSecurityQuery {
    /// Open the checked service key in the local SCM database.
    ///
    /// The caller owns the synchronous operation budget and service selection.
    /// # Errors
    /// Preserve the original native open or manager close failure.
    pub fn open_local(key: ServiceKey<'_>) -> Result<Self, ServiceQueryError> {
        Ok(Self {
            service: OwnedScHandle::open_security(key)?,
        })
    }

    /// Read the owner and DACL envelopes with one bounded native query.
    ///
    /// The report describes that service object at the observed instant.
    /// # Errors
    /// Preserve native read failure, including an oversized required buffer,
    /// or refuse a malformed descriptor envelope.
    #[expect(
        unsafe_code,
        reason = "read only owner and DACL from the retained SCM object"
    )]
    pub fn query(&self) -> Result<ReportedServiceSecurity, ServiceQueryError> {
        let mut buffer: DescriptorBuffer = DescriptorBuffer([0; SERVICE_SECURITY_BYTES]);
        let mut bytes_needed: u32 = 0;
        // SAFETY: The owned handle stays live. The aligned 8192-byte buffer and
        // independent DWORD output are initialized and writable until return.
        let result: i32 = unsafe {
            QueryServiceObjectSecurity(
                self.service.as_raw(),
                SECURITY_INFORMATION,
                (&raw mut buffer).cast::<c_void>(),
                8192,
                &raw mut bytes_needed,
            )
        };
        if result == 0 {
            return Err(ServiceQueryError::QuerySecurity(last_error()));
        }
        ReportedServiceSecurity::from_snapshot(&buffer.0)
            .map_err(ServiceQueryError::InvalidSecurity)
    }

    /// Retire the query handle and retain native close acknowledgement.
    /// # Errors
    /// Preserve the original `CloseServiceHandle` failure.
    pub fn close(self) -> Result<(), ServiceQueryError> {
        self.service
            .close()
            .map_err(ServiceQueryError::CloseService)
    }
}

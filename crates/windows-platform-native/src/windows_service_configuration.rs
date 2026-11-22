// SPDX-License-Identifier: GPL-2.0-only
//! Observe stored startup, identity and privilege settings for one local service.

use core::ffi::c_void;

use crate::service_configuration::{configured_multistring, configured_string};
use crate::service_status::ServiceKey;
use crate::windows_service_status::{OwnedScHandle, ServiceQueryError, last_error};

/// Maximum byte count accepted by the two SCM configuration query APIs.
const CONFIGURATION_BUFFER_BYTES: u32 = 8192;
/// Number of wide units following the fixed SDK header in the query buffer.
const CONFIGURATION_TRAILING_UNITS: usize = (8192 - size_of::<QueryServiceConfig>()) / 2;
/// Win32 startup mode for a service that cannot be started.
const SERVICE_DISABLED: u32 = 4;
/// Configuration level for the SDK's single-DWORD `SERVICE_SID_INFO`.
const SERVICE_CONFIG_SERVICE_SID_INFO: u32 = 5;
/// `SERVICE_SID_INFO` contains exactly one DWORD, with no trailing data.
const SERVICE_SID_INFO_BYTES: u32 = 4;
/// Configuration level for the SDK's variable-length required-privilege list.
const SERVICE_CONFIG_REQUIRED_PRIVILEGES_INFO: u32 = 6;
/// Number of wide code units after the required-privileges pointer.
const REQUIRED_PRIVILEGES_TRAILING_UNITS: usize = (8192 - size_of::<*mut u16>()) / size_of::<u16>();

/// The Windows SDK `QUERY_SERVICE_CONFIGW` record.
#[repr(C)]
#[derive(Default)]
struct QueryServiceConfig {
    /// Raw `dwServiceType` bits.
    service_type: u32,
    /// Configured `dwStartType` value.
    start_type: u32,
    /// Configured `dwErrorControl` value.
    error_control: u32,
    /// SDK `lpBinaryPathName` field, used only as a checked buffer address.
    binary_path: *mut u16,
    /// SDK `lpLoadOrderGroup` field, kept opaque.
    load_order_group: *mut u16,
    /// SDK `dwTagId` field, omitted from the report.
    tag_id: u32,
    /// SDK `lpDependencies` field, kept opaque.
    dependencies: *mut u16,
    /// SDK `lpServiceStartName` field, used only as a checked buffer address.
    start_name: *mut u16,
    /// SDK `lpDisplayName` field, kept opaque.
    display_name: *mut u16,
}

/// Stack storage for the fixed header and variable-length output in one buffer.
#[repr(C)]
struct ServiceConfigurationBuffer {
    /// First bytes written by the API, aligned for the SDK record.
    header: QueryServiceConfig,
    /// Initialized storage for the API's strings. Read only through checked slices.
    strings: [u16; CONFIGURATION_TRAILING_UNITS],
}

const _: [(); 64] = [(); size_of::<QueryServiceConfig>()];
const _: [(); 8] = [(); align_of::<QueryServiceConfig>()];
const _: [(); 0] = [(); core::mem::offset_of!(QueryServiceConfig, service_type)];
const _: [(); 4] = [(); core::mem::offset_of!(QueryServiceConfig, start_type)];
const _: [(); 8] = [(); core::mem::offset_of!(QueryServiceConfig, error_control)];
const _: [(); 16] = [(); core::mem::offset_of!(QueryServiceConfig, binary_path)];
const _: [(); 48] = [(); core::mem::offset_of!(QueryServiceConfig, start_name)];
const _: [(); 64] = [(); core::mem::offset_of!(ServiceConfigurationBuffer, strings)];
const _: [(); 8192] = [(); size_of::<ServiceConfigurationBuffer>()];

/// Stack storage for the SDK's `SERVICE_REQUIRED_PRIVILEGES_INFOW` and its list.
#[repr(C)]
struct RequiredPrivilegesBuffer {
    /// The SDK's sole pointer field, used only as a checked buffer address.
    names: *mut u16,
    /// Initialized storage for the variable-length list.
    strings: [u16; REQUIRED_PRIVILEGES_TRAILING_UNITS],
}

const _: [(); 0] = [(); core::mem::offset_of!(RequiredPrivilegesBuffer, names)];
const _: [(); 8] = [(); core::mem::offset_of!(RequiredPrivilegesBuffer, strings)];
const _: [(); 8] = [(); align_of::<RequiredPrivilegesBuffer>()];
const _: [(); 8192] = [(); size_of::<RequiredPrivilegesBuffer>()];

#[link(name = "advapi32")]
#[expect(unsafe_code, reason = "Exact winsvc.h configuration query ABI")]
unsafe extern "system" {
    /// Read the named SCM object's configuration into the caller's buffer.
    fn QueryServiceConfigW(
        service: *mut c_void,
        configuration: *mut QueryServiceConfig,
        buffer_size: u32,
        bytes_needed: *mut u32,
    ) -> i32;
    /// Read one documented optional configuration level into the caller's buffer.
    fn QueryServiceConfig2W(
        service: *mut c_void,
        level: u32,
        configuration: *mut u8,
        buffer_size: u32,
        bytes_needed: *mut u32,
    ) -> i32;
}

/// Stored startup settings, binary path and start name reported by the SCM.
///
/// These values describe stored configuration, which can change independently
/// of the state or identity of a running process.
#[must_use = "retain the actual service configuration report"]
pub struct ReportedServiceConfiguration {
    /// Raw Win32 `dwServiceType` bits, including unrecognized bits.
    service_type: u32,
    /// Raw Win32 `dwStartType` value.
    start_type: u32,
    /// Raw Win32 `dwErrorControl` value.
    error_control: u32,
    /// Literal binary path and any arguments, without the terminating NUL.
    binary_path: Option<Box<[u16]>>,
    /// Literal account name or driver object, without the terminating NUL.
    start_name: Option<Box<[u16]>>,
}

impl ReportedServiceConfiguration {
    /// Return the configured Win32 service type bits.
    #[must_use]
    pub const fn service_type(&self) -> u32 {
        self.service_type
    }

    /// Return the configured Win32 startup mode.
    #[must_use]
    pub const fn start_type(&self) -> u32 {
        self.start_type
    }

    /// Return the configured Win32 startup error policy.
    #[must_use]
    pub const fn error_control(&self) -> u32 {
        self.error_control
    }

    /// Check whether the configured startup mode is `SERVICE_DISABLED`.
    #[must_use]
    pub const fn starts_disabled(&self) -> bool {
        self.start_type == SERVICE_DISABLED
    }

    /// Borrow the literal configured path and any arguments. A null field is absent.
    /// The value identifies stored configuration. It carries no executable identity.
    #[must_use]
    pub fn binary_path(&self) -> Option<&[u16]> {
        self.binary_path.as_deref()
    }

    /// Borrow the literal configured account or driver object. A null field is absent.
    /// Observe the actual token separately when checking effective identity.
    #[must_use]
    pub fn start_name(&self) -> Option<&[u16]> {
        self.start_name.as_deref()
    }
}

/// Own a configuration-query handle to one service in the local active database.
#[must_use = "keep the query handle through observation and close it to verify release"]
pub struct ServiceConfigurationQuery {
    /// The retained service object opened with `SERVICE_QUERY_CONFIG` access.
    service: OwnedScHandle,
}

impl ServiceConfigurationQuery {
    /// Open the local SCM and the exact checked service key for configuration query.
    ///
    /// The caller owns the synchronous operation budget and service selection.
    ///
    /// # Errors
    /// Preserves the first native open failure or a failed manager close.
    pub fn open_local(key: ServiceKey<'_>) -> Result<Self, ServiceQueryError> {
        let service: OwnedScHandle = OwnedScHandle::open_configuration(key)?;
        Ok(Self { service })
    }

    /// Read the startup and identity fields with one synchronous, fixed-size query.
    ///
    /// # Errors
    /// Preserve the native failure or a field-specific returned-string refusal.
    #[expect(unsafe_code, reason = "Read one bounded SDK configuration record")]
    pub fn query(&self) -> Result<ReportedServiceConfiguration, ServiceQueryError> {
        let mut buffer: ServiceConfigurationBuffer = ServiceConfigurationBuffer {
            header: QueryServiceConfig::default(),
            // Unwritten storage must never supply a synthetic string terminator.
            strings: [u16::MAX; CONFIGURATION_TRAILING_UNITS],
        };
        let mut bytes_needed: u32 = 0;
        // SAFETY: The service owner retains a live query handle. The pointer is
        // derived from the full writable buffer, whose initial header has the SDK
        // layout. The DWORD output is a separate writable local. Both outputs
        // remain live until the call returns.
        let success: i32 = unsafe {
            QueryServiceConfigW(
                self.service.as_raw(),
                (&raw mut buffer).cast::<QueryServiceConfig>(),
                CONFIGURATION_BUFFER_BYTES,
                &raw mut bytes_needed,
            )
        };
        if success == 0 {
            return Err(ServiceQueryError::QueryConfiguration(last_error()));
        }

        // Use the native pointers only as addresses. Read via the original
        // initialized array so no borrowed bytes can come from another object.
        let strings_address: usize = buffer.strings.as_ptr().addr();
        let binary_path: Option<&[u16]> = configured_string(
            &buffer.strings,
            strings_address,
            buffer.header.binary_path.addr(),
        )
        .map_err(ServiceQueryError::InvalidBinaryPath)?;
        let start_name: Option<&[u16]> = configured_string(
            &buffer.strings,
            strings_address,
            buffer.header.start_name.addr(),
        )
        .map_err(ServiceQueryError::InvalidStartName)?;
        Ok(ReportedServiceConfiguration {
            service_type: buffer.header.service_type,
            start_type: buffer.header.start_type,
            error_control: buffer.header.error_control,
            binary_path: binary_path.map(Box::from),
            start_name: start_name.map(Box::from),
        })
    }

    /// Read the explicit required-privilege list stored for this service.
    ///
    /// Return literal UTF-16 units in reported order, including the list's
    /// terminating empty element. A null pointer yields `None`. A present empty
    /// list yields one zero unit. This is configuration for a subsequent start.
    /// Observe the effective token and apply the selected role policy separately.
    ///
    /// # Errors
    /// Preserve the native query failure or refuse an invalid returned list.
    #[expect(unsafe_code, reason = "Read one bounded required-privileges record")]
    pub fn query_required_privileges(&self) -> Result<Option<Box<[u16]>>, ServiceQueryError> {
        let mut buffer: RequiredPrivilegesBuffer = RequiredPrivilegesBuffer {
            names: core::ptr::null_mut(),
            // Unwritten storage must never complete an unfinished list.
            strings: [u16::MAX; REQUIRED_PRIVILEGES_TRAILING_UNITS],
        };
        let mut bytes_needed: u32 = 0;
        // SAFETY: The service owner retains query access. The aligned record
        // and disjoint DWORD are initialized, writable and live for the call.
        let success: i32 = unsafe {
            QueryServiceConfig2W(
                self.service.as_raw(),
                SERVICE_CONFIG_REQUIRED_PRIVILEGES_INFO,
                (&raw mut buffer).cast::<u8>(),
                CONFIGURATION_BUFFER_BYTES,
                &raw mut bytes_needed,
            )
        };
        if success == 0 {
            return Err(ServiceQueryError::QueryRequiredPrivileges(last_error()));
        }

        // Use the returned address only as a checked index into the original buffer.
        let names: Option<&[u16]> = configured_multistring(
            &buffer.strings,
            buffer.strings.as_ptr().addr(),
            buffer.names.addr(),
        )
        .map_err(ServiceQueryError::InvalidRequiredPrivileges)?;
        Ok(names.map(Box::from))
    }

    /// Read the stored service-SID type as the exact `dwServiceSidType` DWORD.
    ///
    /// Unknown values remain visible. Changes take effect at the next system
    /// start. Observe the effective token independently.
    ///
    /// # Errors
    /// Preserve the native failure from the bounded SID-type query.
    #[expect(unsafe_code, reason = "Read one documented DWORD service SID record")]
    pub fn query_service_sid_type(&self) -> Result<u32, ServiceQueryError> {
        // The initializer cannot stand in for any of the known policy values.
        let mut sid_type: u32 = u32::MAX;
        let mut bytes_needed: u32 = 0;
        // SAFETY: SERVICE_SID_INFO contains one DWORD. Both disjoint, aligned
        // outputs are initialized and writable for the synchronous call.
        let success: i32 = unsafe {
            QueryServiceConfig2W(
                self.service.as_raw(),
                SERVICE_CONFIG_SERVICE_SID_INFO,
                (&raw mut sid_type).cast::<u8>(),
                SERVICE_SID_INFO_BYTES,
                &raw mut bytes_needed,
            )
        };
        if success == 0 {
            return Err(ServiceQueryError::QueryServiceSidType(last_error()));
        }
        Ok(sid_type)
    }

    /// Consume the handle and check the native retirement result.
    ///
    /// # Errors
    /// Preserves the native `CloseServiceHandle` failure.
    pub fn close(self) -> Result<(), ServiceQueryError> {
        self.service
            .close()
            .map_err(ServiceQueryError::CloseService)
    }
}

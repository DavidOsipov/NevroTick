// SPDX-License-Identifier: GPL-2.0-only
//! Read recorded service status and share narrow query-handle custody for the local SCM.

use core::{ffi::c_void, mem::ManuallyDrop, ptr::NonNull};

use crate::service_configuration::ConfigurationStringError;
use crate::service_security::ServiceSecurityError;
use crate::service_status::{ReportedServiceStatus, ServiceKey};

/// Manager right required to open a service by key.
const SC_MANAGER_CONNECT: u32 = 0x0001;
/// The separate access right for reading a service's configured values.
const SERVICE_QUERY_CONFIG: u32 = 0x0001;
/// The sole access right requested for the service handle.
const SERVICE_QUERY_STATUS: u32 = 0x0004;
/// The separate right to read owner and discretionary authorization information.
const READ_CONTROL: u32 = 0x0002_0000;
/// Query level for the `SERVICE_STATUS_PROCESS` record.
const SC_STATUS_PROCESS_INFO: i32 = 0;
/// Size of the query record reported to the ABI.
const STATUS_BYTES: u32 = 36;

/// The SDK's `SERVICE_STATUS_PROCESS` record consists of nine DWORDs.
#[repr(C)]
#[derive(Default)]
struct ServiceStatusProcess {
    /// Reported service type bits.
    service_type: u32,
    /// Last state reported to the SCM.
    current_state: u32,
    /// Reported control acceptance bits.
    controls_accepted: u32,
    /// Win32 exit code from the status record.
    win32_exit_code: u32,
    /// Service-defined exit code from the status record.
    service_specific_exit_code: u32,
    /// Progress checkpoint from the status record.
    check_point: u32,
    /// Estimated duration of a pending operation in milliseconds.
    wait_hint: u32,
    /// Recorded process ID, whose meaning depends on service state.
    process_id: u32,
    /// Additional service flags from the status record.
    service_flags: u32,
}

const _: [(); 36] = [(); size_of::<ServiceStatusProcess>()];
const _: [(); 4] = [(); align_of::<ServiceStatusProcess>()];
const _: [(); 28] = [(); core::mem::offset_of!(ServiceStatusProcess, process_id)];

#[link(name = "advapi32")]
#[expect(
    unsafe_code,
    reason = "Exact winsvc.h query-only Service Control Manager ABIs"
)]
unsafe extern "system" {
    /// Open the selected service manager database with the requested access.
    fn OpenSCManagerW(machine: *const u16, database: *const u16, access: u32) -> *mut c_void;
    /// Open the exact service key with the requested access.
    fn OpenServiceW(manager: *mut c_void, service: *const u16, access: u32) -> *mut c_void;
    /// Read one status record at the supplied query level.
    fn QueryServiceStatusEx(
        service: *mut c_void,
        level: i32,
        buffer: *mut u8,
        buffer_size: u32,
        bytes_needed: *mut u32,
    ) -> i32;
    /// Release an SCM or service handle.
    fn CloseServiceHandle(handle: *mut c_void) -> i32;
}

#[link(name = "kernel32")]
#[expect(
    unsafe_code,
    reason = "Capture the calling thread's Win32 failure immediately"
)]
unsafe extern "system" {
    /// Read the calling thread's last error code.
    fn GetLastError() -> u32;
}

/// A refusal at a specific native operation, retaining the original system code.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServiceQueryError {
    /// Opening the local active database with connect access failed.
    OpenManager(u32),
    /// Opening the named service with the observer's single query right failed.
    OpenService(u32),
    /// The service refused the open, and the manager did not acknowledge closure.
    OpenServiceAndCloseManager {
        /// Original error captured from the service open.
        open_service: u32,
        /// Additional error while closing the manager.
        close_manager: u32,
    },
    /// The manager handle did not acknowledge closure.
    CloseManager(u32),
    /// A manager close refusal prevented publication and service closure also failed.
    CloseManagerAndCloseService {
        /// Original error while closing the manager.
        close_manager: u32,
        /// Additional error while closing the service.
        close_service: u32,
    },
    /// Reading the recorded process status failed.
    Query(u32),
    /// Reading the configured service values failed.
    QueryConfiguration(u32),
    /// The returned binary path or command line falls outside its buffer contract.
    InvalidBinaryPath(ConfigurationStringError),
    /// The returned start account or driver object falls outside its buffer contract.
    InvalidStartName(ConfigurationStringError),
    /// Reading the configured required privileges failed.
    QueryRequiredPrivileges(u32),
    /// The returned required-privilege list falls outside its buffer contract.
    InvalidRequiredPrivileges(ConfigurationStringError),
    /// Reading the configured service-SID type failed.
    QueryServiceSidType(u32),
    /// Reading the service object's security descriptor failed.
    QuerySecurity(u32),
    /// The returned owner or discretionary ACL envelope was not interpretable.
    InvalidSecurity(ServiceSecurityError),
    /// The service handle did not acknowledge closure.
    CloseService(u32),
}

impl core::fmt::Display for ServiceQueryError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let (operation, code): (&str, u32) = match *self {
            Self::OpenManager(code) => ("open local service manager", code),
            Self::OpenService(code) => ("open service for query", code),
            Self::OpenServiceAndCloseManager {
                open_service,
                close_manager,
            } => {
                return write!(
                    formatter,
                    "open service for query failed ({open_service}), close service manager failed ({close_manager})"
                );
            }
            Self::CloseManager(code) => ("close service manager", code),
            Self::CloseManagerAndCloseService {
                close_manager,
                close_service,
            } => {
                return write!(
                    formatter,
                    "close service manager failed ({close_manager}), close service query handle failed ({close_service})"
                );
            }
            Self::Query(code) => ("query service status", code),
            Self::QueryConfiguration(code) => ("query service configuration", code),
            Self::InvalidBinaryPath(error) => {
                return write!(
                    formatter,
                    "invalid configured service binary path ({error})"
                );
            }
            Self::InvalidStartName(error) => {
                return write!(formatter, "invalid configured service start name ({error})");
            }
            Self::QueryRequiredPrivileges(code) => ("query configured required privileges", code),
            Self::InvalidRequiredPrivileges(error) => {
                return write!(
                    formatter,
                    "invalid configured required privileges ({error})"
                );
            }
            Self::QueryServiceSidType(code) => ("query configured service SID type", code),
            Self::QuerySecurity(code) => ("query service object security", code),
            Self::InvalidSecurity(error) => {
                return write!(formatter, "invalid service security observation ({error})");
            }
            Self::CloseService(code) => ("close service query handle", code),
        };
        write!(formatter, "{operation} failed ({code})")
    }
}

impl std::error::Error for ServiceQueryError {}

/// One native SCM object, closed with its own close function exactly once.
pub(super) struct OwnedScHandle(NonNull<c_void>);

impl OwnedScHandle {
    /// Open only the configuration of the named service in the local database.
    /// # Errors
    /// Preserve an open or manager close failure.
    pub(super) fn open_configuration(key: ServiceKey<'_>) -> Result<Self, ServiceQueryError> {
        Self::open_local(key, SERVICE_QUERY_CONFIG)
    }

    /// Open only the security observation right on the named local service.
    /// # Errors
    /// Preserve an open or manager close failure.
    pub(super) fn open_security(key: ServiceKey<'_>) -> Result<Self, ServiceQueryError> {
        Self::open_local(key, READ_CONTROL)
    }

    /// Open the named local service under one of the private query masks.
    /// # Errors
    /// Preserve an open or manager close failure.
    #[expect(
        unsafe_code,
        reason = "open local SCM objects with fixed separate query rights"
    )]
    fn open_local(key: ServiceKey<'_>, access: u32) -> Result<Self, ServiceQueryError> {
        // SAFETY: Both null pointers select the local machine and active database.
        let manager: *mut c_void =
            unsafe { OpenSCManagerW(core::ptr::null(), core::ptr::null(), SC_MANAGER_CONNECT) };
        let manager: Self = Self(
            NonNull::new(manager).ok_or_else(|| ServiceQueryError::OpenManager(last_error()))?,
        );
        // SAFETY: The retained manager and checked NUL-terminated key remain live.
        let service: *mut c_void =
            unsafe { OpenServiceW(manager.as_raw(), key.as_wide_with_nul().as_ptr(), access) };
        let opened: Result<Self, u32> = NonNull::new(service).map(Self).ok_or_else(last_error);
        finish_service_open(opened, manager.close(), Self::close)
    }

    /// Borrow the native handle for a synchronous query in this adapter family.
    pub(super) const fn as_raw(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Consume the handle and observe native retirement without retrying on error.
    /// # Errors
    /// Return the original `CloseServiceHandle` failure.
    pub(super) fn close(self) -> Result<(), u32> {
        close_handle(ManuallyDrop::new(self).0)
    }
}

/// Publish an opened service only after the manager's close is acknowledged.
/// # Errors
/// Keep the first open or close failure and any later discarded-handle close error.
fn finish_service_open<H>(
    opened: Result<H, u32>,
    manager_close: Result<(), u32>,
    close_service: impl FnOnce(H) -> Result<(), u32>,
) -> Result<H, ServiceQueryError> {
    match (opened, manager_close) {
        (Ok(service), Ok(())) => Ok(service),
        (Err(code), Ok(())) => Err(ServiceQueryError::OpenService(code)),
        (Err(open_service), Err(close_manager)) => {
            Err(ServiceQueryError::OpenServiceAndCloseManager {
                open_service,
                close_manager,
            })
        }
        (Ok(service), Err(close_manager)) => Err(match close_service(service) {
            Ok(()) => ServiceQueryError::CloseManager(close_manager),
            Err(close_service) => ServiceQueryError::CloseManagerAndCloseService {
                close_manager,
                close_service,
            },
        }),
    }
}

/// Exercise closure transfer and both paired-failure observations.
/// # Panics
/// Fails if incomplete closure publishes a handle or loses an original code.
#[test]
fn service_open_retires_handles_and_preserves_paired_failures() {
    use core::cell::Cell;

    let count: Cell<u8> = Cell::new(0);
    let close_ok = |handle: u8| {
        assert_eq!(handle, 42);
        count.set(count.get() + 1);
        Ok(())
    };
    assert_eq!(finish_service_open(Ok(42), Ok(()), close_ok), Ok(42));
    assert_eq!(
        finish_service_open(Err(1060), Ok(()), close_ok),
        Err(ServiceQueryError::OpenService(1060))
    );
    assert_eq!(
        finish_service_open(Err(5), Err(6), close_ok),
        Err(ServiceQueryError::OpenServiceAndCloseManager {
            open_service: 5,
            close_manager: 6
        })
    );
    assert_eq!(count.get(), 0);
    assert_eq!(
        finish_service_open(Ok(42), Err(6), close_ok),
        Err(ServiceQueryError::CloseManager(6))
    );
    assert_eq!(
        finish_service_open(Ok(42), Err(6), |handle: u8| {
            close_ok(handle)?;
            Err(7)
        }),
        Err(ServiceQueryError::CloseManagerAndCloseService {
            close_manager: 6,
            close_service: 7
        })
    );
    assert_eq!(count.get(), 2);
    assert_eq!(
        ServiceQueryError::OpenServiceAndCloseManager {
            open_service: 5,
            close_manager: 6
        }
        .to_string(),
        "open service for query failed (5), close service manager failed (6)"
    );
    assert_eq!(
        ServiceQueryError::CloseManagerAndCloseService {
            close_manager: 6,
            close_service: 7
        }
        .to_string(),
        "close service manager failed (6), close service query handle failed (7)"
    );
}

impl Drop for OwnedScHandle {
    fn drop(&mut self) {
        let _closed: Result<(), u32> = close_handle(self.0);
    }
}

/// Release one consumed SCM handle and preserve any native close error.
/// # Errors
/// Return the calling thread's native error code if closing the handle fails.
#[expect(
    unsafe_code,
    reason = "Close only the SCM objects returned by the query adapter"
)]
fn close_handle(handle: NonNull<c_void>) -> Result<(), u32> {
    // SAFETY: The private owner calls this exactly once for a live SCM handle.
    if unsafe { CloseServiceHandle(handle.as_ptr()) } == 0 {
        return Err(last_error());
    }
    Ok(())
}

/// Capture the calling thread's last error immediately after an SCM failure.
#[expect(
    unsafe_code,
    reason = "capture the failing operation's thread-local error"
)]
pub(super) fn last_error() -> u32 {
    // SAFETY: The getter reads only the current thread's native error slot.
    unsafe { GetLastError() }
}

/// Retain a status-only handle to one service in the local active database.
///
/// This synchronous adapter returns SCM reports. Process and token identity,
/// connected endpoint ownership and current authority are separate checks.
#[must_use = "keep the query handle for follow-up observations and close it to verify release"]
pub struct ServiceStatusQuery {
    /// The retained service object opened with status-only access.
    service: OwnedScHandle,
}

impl ServiceStatusQuery {
    /// Open the local SCM and the exact checked service key for status query.
    ///
    /// The caller owns the synchronous operation budget and the expected service
    /// selection. No service-control or configuration access is requested.
    ///
    /// # Errors
    /// Preserves the first native open failure or a failed manager close.
    pub fn open_local(key: ServiceKey<'_>) -> Result<Self, ServiceQueryError> {
        Ok(Self {
            service: OwnedScHandle::open_local(key, SERVICE_QUERY_STATUS)?,
        })
    }

    /// Read the last report for the same retained service object.
    ///
    /// # Errors
    /// Preserves the original failure from `QueryServiceStatusEx`.
    #[expect(unsafe_code, reason = "Read one fixed SDK process-status record")]
    pub fn query(&self) -> Result<ReportedServiceStatus, ServiceQueryError> {
        let mut status: ServiceStatusProcess = ServiceStatusProcess::default();
        let mut bytes_needed: u32 = 0;
        // SAFETY: The service handle is retained. The two disjoint, initialized
        // outputs are aligned and writable for the specified level and size.
        let success: i32 = unsafe {
            QueryServiceStatusEx(
                self.service.as_raw(),
                SC_STATUS_PROCESS_INFO,
                (&raw mut status).cast::<u8>(),
                STATUS_BYTES,
                &raw mut bytes_needed,
            )
        };
        if success == 0 {
            return Err(ServiceQueryError::Query(last_error()));
        }
        Ok(ReportedServiceStatus::new(
            status.service_type,
            status.current_state,
            status.process_id,
        ))
    }

    /// Consume this service handle and check the native retirement result.
    /// # Errors
    /// Preserves the native `CloseServiceHandle` failure.
    pub fn close(self) -> Result<(), ServiceQueryError> {
        self.service
            .close()
            .map_err(ServiceQueryError::CloseService)
    }
}

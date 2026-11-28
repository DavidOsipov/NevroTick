// SPDX-License-Identifier: GPL-3.0-only
//! Fixed desktop UI startup policies before Slint/backend work.
//!
//! The mitigation guard checks effective flags after any tightening. The
//! search policy sets the process default for later DLL loads. Pre-entry
//! images and searches with explicit loader flags are outside that default.

use std::fmt;

/// Required dynamic-code and legacy extension-point policy readback.
mod restricted_execution;
pub use restricted_execution::{RestrictedExecutionPolicy, RestrictedExecutionPolicyError};
/// One mandatory transition before the actual native backend factory.
mod admission;
#[cfg(windows)]
pub(super) use admission::StartupAdmissionError;
#[cfg(windows)]
pub(super) use admission::before_backend;

/// Permanent strict-handle admission has its own fixed policy and outcome.
mod strict_handles;
pub use strict_handles::StrictHandlePolicyError;

/// Required `PROCESS_MITIGATION_IMAGE_LOAD_POLICY` bits from `winnt.h`.
const REQUIRED_FLAGS: u32 = 0x01 | 0x02 | 0x04;
/// The three enforcement and two audit bits from the same policy layout.
const KNOWN_FLAGS: u32 = REQUIRED_FLAGS | 0x08 | 0x10;
/// Windows SDK's `LOAD_LIBRARY_SEARCH_APPLICATION_DIR | LOAD_LIBRARY_SEARCH_SYSTEM32`.
const DEFAULT_DLL_SEARCH_FLAGS: u32 = 0x0200 | 0x0800;

/// Failure to configure the current process's fixed default DLL search.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use]
pub struct DllSearchPolicyError {
    /// Original Win32 error code captured when the setter failed.
    code: u32,
}

impl DllSearchPolicyError {
    /// Return the exact Win32 error without decoding native text.
    #[must_use]
    pub const fn win32_code(self) -> u32 {
        self.code
    }
}

impl fmt::Display for DllSearchPolicyError {
    /// Format the fixed failure message and the native error code.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Cannot configure the default DLL search (Win32 {}).",
            self.code
        )
    }
}

impl std::error::Error for DllSearchPolicyError {}

/// Set the fixed default DLL search once through the supplied process setter.
///
/// # Errors
/// Preserve the setter's error code without retrying under weaker settings.
fn configure_default_dll_search_with(
    set: impl FnOnce(u32) -> Result<(), u32>,
) -> Result<(), DllSearchPolicyError> {
    set(DEFAULT_DLL_SEARCH_FLAGS).map_err(|code| DllSearchPolicyError { code })
}

/// Set the ordinary UI process's DLL search before backend selection.
///
/// # Errors
/// Return the original Win32 failure so the caller can refuse startup.
#[cfg(windows)]
pub(super) fn configure_default_dll_search() -> Result<(), DllSearchPolicyError> {
    configure_default_dll_search_with(windows::set_default_dll_search)
}

/// Bounded reasons the Windows UI cannot start under its required image policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use]
pub enum ImageLoadPolicyError {
    /// The initial query failed with this Win32 error code.
    ReadInitial(u32),
    /// The initial flags contain reserved or unrecognized bits.
    UnexpectedInitialFlags(u32),
    /// Applying the required and inherited flags failed with this Win32 code.
    Apply(u32),
    /// The post-apply query failed with this Win32 error code.
    ReadEffective(u32),
    /// The post-apply flags contain reserved or unrecognized bits.
    UnexpectedEffectiveFlags(u32),
    /// The effective flags omit requested enforcement or inherited audit bits.
    MissingEffectiveFlags {
        /// The union of required bits and the initial recognized flags.
        requested: u32,
        /// The exact flags returned by the post-apply Win32 query.
        observed: u32,
    },
}

impl fmt::Display for ImageLoadPolicyError {
    /// Format the fixed stage and native code or flags without system text.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadInitial(code) => write!(
                formatter,
                "Cannot read the startup image-load policy (Win32 {code})."
            ),
            Self::UnexpectedInitialFlags(flags) => write!(
                formatter,
                "Unexpected startup image-load policy flags ({flags:#010x})."
            ),
            Self::Apply(code) => write!(
                formatter,
                "Cannot set the startup image-load policy (Win32 {code})."
            ),
            Self::ReadEffective(code) => write!(
                formatter,
                "Cannot verify the effective image-load policy (Win32 {code})."
            ),
            Self::UnexpectedEffectiveFlags(flags) => write!(
                formatter,
                "Unexpected effective image-load policy flags ({flags:#010x})."
            ),
            Self::MissingEffectiveFlags {
                requested,
                observed,
            } => write!(
                formatter,
                "Incomplete effective image-load policy ({observed:#010x}, required {requested:#010x})."
            ),
        }
    }
}

impl std::error::Error for ImageLoadPolicyError {}

/// Synchronous reads and writes of one policy on the current process.
trait PolicyCalls {
    /// Return the exact flags for this adapter's one fixed policy kind.
    ///
    /// # Errors
    /// Return the original native error if the query fails.
    fn read(&mut self) -> Result<u32, u32>;

    /// Apply the supplied recognized flags.
    ///
    /// # Errors
    /// Return the original native error if the setter fails.
    fn apply(&mut self, flags: u32) -> Result<(), u32>;
}

/// Require the three enforcement bits and preserve every inherited known bit.
///
/// # Errors
/// Refuse startup on a failed native call, unrecognized flags or incomplete
/// readback.
fn enforce_with(calls: &mut impl PolicyCalls) -> Result<(), ImageLoadPolicyError> {
    let initial: u32 = calls.read().map_err(ImageLoadPolicyError::ReadInitial)?;
    if initial & !KNOWN_FLAGS != 0 {
        return Err(ImageLoadPolicyError::UnexpectedInitialFlags(initial));
    }
    let requested: u32 = initial | REQUIRED_FLAGS;
    if initial == requested {
        return Ok(());
    }

    calls
        .apply(requested)
        .map_err(ImageLoadPolicyError::Apply)?;
    let observed: u32 = calls.read().map_err(ImageLoadPolicyError::ReadEffective)?;
    if observed & !KNOWN_FLAGS != 0 {
        return Err(ImageLoadPolicyError::UnexpectedEffectiveFlags(observed));
    }
    if observed & requested != requested {
        return Err(ImageLoadPolicyError::MissingEffectiveFlags {
            requested,
            observed,
        });
    }
    Ok(())
}

/// Apply and verify the UI's fixed process policy before any Slint startup.
///
/// # Errors
/// Return the bounded native failure or rejected effective state. A prior
/// successful tightening is never rolled back.
#[cfg(windows)]
pub(super) fn enforce() -> Result<(), ImageLoadPolicyError> {
    enforce_with(&mut windows::SystemPolicyCalls)
}

/// Require permanent strict handles on this UI process before Slint starts.
///
/// # Errors
/// Return the first native error or refused observation. A successful setting
/// is permanent; later refusal does not roll back or retry under weaker flags.
#[cfg(windows)]
pub(super) fn enforce_strict_handles() -> Result<(), StrictHandlePolicyError> {
    strict_handles::enforce_with(&mut windows::StrictHandlePolicyCalls)
}

/// Enforce one of the two fixed restricted-execution policies on this process.
///
/// # Errors
/// Preserve the first query, tightening or effective-state refusal. This does
/// not qualify native dependency compatibility or complete role readiness.
#[cfg(windows)]
fn enforce_restricted(
    policy: RestrictedExecutionPolicy,
) -> Result<(), RestrictedExecutionPolicyError> {
    restricted_execution::enforce_with(&mut windows::RestrictedExecutionPolicyCalls::new(policy))
}

/// Narrow Kernel32 adapters for the UI process's startup policies.
#[cfg(windows)]
mod windows {
    use super::PolicyCalls;
    use super::restricted_execution::{PolicyCalls as RestrictedCalls, RestrictedExecutionPolicy};
    use std::{ffi::c_void, mem::size_of};
    use windows_sys::Win32::Foundation::{GetLastError, HANDLE};
    use windows_sys::Win32::System::LibraryLoader::{LOAD_LIBRARY_FLAGS, SetDefaultDllDirectories};

    /// Apply the startup owner's fixed flags to the calling process.
    ///
    /// # Errors
    /// Return the original Win32 code when the setter returns zero.
    #[expect(
        unsafe_code,
        reason = "Set the process default using the existing Windows binding and capture its immediate error"
    )]
    pub(super) fn set_default_dll_search(flags: LOAD_LIBRARY_FLAGS) -> Result<(), u32> {
        // SAFETY: The synchronous setter takes only a scalar flag word.
        let status: i32 = unsafe { SetDefaultDllDirectories(flags) };
        if status == 0 {
            // SAFETY: Read the failed setter's last error on the same thread.
            return Err(unsafe { GetLastError() });
        }
        Ok(())
    }

    /// `PROCESS_MITIGATION_POLICY::ProcessImageLoadPolicy` in the Windows SDK.
    const PROCESS_IMAGE_LOAD_POLICY: i32 = 10;

    /// `PROCESS_MITIGATION_POLICY::ProcessStrictHandleCheckPolicy` in `winnt.h`.
    const PROCESS_STRICT_HANDLE_CHECK_POLICY: i32 = 3;

    /// The single DWORD union in `PROCESS_MITIGATION_IMAGE_LOAD_POLICY`.
    #[repr(C)]
    struct NativeImageLoadPolicy {
        /// Flags, including the documented enforcement and audit bitfields.
        flags: u32,
    }

    /// The DWORD union in `PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY`.
    #[repr(C)]
    struct NativeStrictHandlePolicy {
        /// Both documented one-bit fields, with the upper 30 bits reserved.
        flags: u32,
    }

    // The pinned windows 0.62.2 Threading and SystemServices bindings expose
    // these exact ABIs. Keep this leaf private so the UI feature graph stays
    // unchanged and no caller can choose a handle or policy kind.
    #[link(name = "kernel32")]
    #[expect(
        unsafe_code,
        reason = "Exact Win32 process policy and pseudohandle ABIs"
    )]
    unsafe extern "system" {
        /// Obtain the borrowed pseudo handle for the calling process.
        fn GetCurrentProcess() -> HANDLE;
        /// Read exactly one typed policy into the supplied writable buffer.
        fn GetProcessMitigationPolicy(
            process: HANDLE,
            policy: i32,
            buffer: *mut c_void,
            length: usize,
        ) -> i32;
        /// Set exactly one typed policy on the calling process.
        fn SetProcessMitigationPolicy(policy: i32, buffer: *const c_void, length: usize) -> i32;
    }

    /// Win32 calls for the UI's own current process.
    pub(super) struct SystemPolicyCalls;

    impl PolicyCalls for SystemPolicyCalls {
        /// Read the complete flags from the borrowed current-process handle.
        ///
        /// # Errors
        /// Return the original Win32 code if the query fails.
        #[expect(
            unsafe_code,
            reason = "Use a current-process pseudohandle and fixed output buffer"
        )]
        fn read(&mut self) -> Result<u32, u32> {
            let mut policy: NativeImageLoadPolicy = NativeImageLoadPolicy { flags: 0 };
            // SAFETY: This API takes no arguments and the returned pseudo handle
            // is valid for a current-process query without closing it.
            let process: HANDLE = unsafe { GetCurrentProcess() };
            // SAFETY: The local output is writable and aligned for the exact
            // policy layout and remains live for this synchronous call.
            let status: i32 = unsafe {
                GetProcessMitigationPolicy(
                    process,
                    PROCESS_IMAGE_LOAD_POLICY,
                    (&raw mut policy).cast::<c_void>(),
                    size_of::<NativeImageLoadPolicy>(),
                )
            };
            if status == 0 {
                // SAFETY: This takes no arguments and immediately reads the
                // error for the failed query on the same thread.
                return Err(unsafe { GetLastError() });
            }
            Ok(policy.flags)
        }

        /// Set the checked flags with the exact read-only policy buffer.
        ///
        /// # Errors
        /// Return the original Win32 code if the setter fails.
        #[expect(
            unsafe_code,
            reason = "Pass the exact initialized policy buffer to Win32"
        )]
        fn apply(&mut self, flags: u32) -> Result<(), u32> {
            let policy: NativeImageLoadPolicy = NativeImageLoadPolicy { flags };
            // SAFETY: The local input is initialized and aligned for this
            // policy kind and remains live for this synchronous call.
            let status: i32 = unsafe {
                SetProcessMitigationPolicy(
                    PROCESS_IMAGE_LOAD_POLICY,
                    (&raw const policy).cast::<c_void>(),
                    size_of::<NativeImageLoadPolicy>(),
                )
            };
            if status == 0 {
                // SAFETY: This takes no arguments and immediately reads the
                // error for the failed setter on the same thread.
                return Err(unsafe { GetLastError() });
            }
            Ok(())
        }
    }

    /// Fixed strict-handle calls for the UI's own current process only.
    pub(super) struct StrictHandlePolicyCalls;

    impl PolicyCalls for StrictHandlePolicyCalls {
        /// Read the exact DWORD from the borrowed current-process pseudohandle.
        ///
        /// # Errors
        /// Preserve the query's immediate Win32 code on its original thread.
        #[expect(
            unsafe_code,
            reason = "Query the fixed strict-handle policy into its exact DWORD layout"
        )]
        fn read(&mut self) -> Result<u32, u32> {
            let mut policy: NativeStrictHandlePolicy = NativeStrictHandlePolicy { flags: 0 };
            // SAFETY: This argument-free API returns the borrowed pseudohandle
            // for this process. It needs no close and outlives the query.
            let process: HANDLE = unsafe { GetCurrentProcess() };
            // SAFETY: The initialized, aligned DWORD output has the exact SDK
            // policy extent and remains writable through this synchronous call.
            let status: i32 = unsafe {
                GetProcessMitigationPolicy(
                    process,
                    PROCESS_STRICT_HANDLE_CHECK_POLICY,
                    (&raw mut policy).cast::<c_void>(),
                    size_of::<NativeStrictHandlePolicy>(),
                )
            };
            if status == 0 {
                // SAFETY: Read the failed query's error before another call.
                return Err(unsafe { GetLastError() });
            }
            Ok(policy.flags)
        }

        /// Set the checked matching exception/permanent fields exactly once.
        ///
        /// # Errors
        /// Preserve the setter's immediate Win32 code without another setting.
        #[expect(
            unsafe_code,
            reason = "Set only the fixed current-process strict-handle DWORD policy"
        )]
        fn apply(&mut self, flags: u32) -> Result<(), u32> {
            let policy: NativeStrictHandlePolicy = NativeStrictHandlePolicy { flags };
            // SAFETY: The private caller supplies the checked matching fields.
            // This exact initialized DWORD input lives through the synchronous
            // call, which sets only this process's strict-handle policy.
            let status: i32 = unsafe {
                SetProcessMitigationPolicy(
                    PROCESS_STRICT_HANDLE_CHECK_POLICY,
                    (&raw const policy).cast::<c_void>(),
                    size_of::<NativeStrictHandlePolicy>(),
                )
            };
            if status == 0 {
                // SAFETY: Read the failed setter's error on this same thread.
                return Err(unsafe { GetLastError() });
            }
            Ok(())
        }
    }

    /// Exact DWORD union layout shared by the two selected execution policies.
    #[repr(C)]
    struct NativeRestrictedExecutionPolicy {
        /// SDK bitfields are checked by the fixed policy owner after query.
        flags: u32,
    }

    /// Closed policy choice for current-process calls; never a caller handle or ID.
    pub(super) struct RestrictedExecutionPolicyCalls {
        /// One code-owned policy retained through query, setter and readback.
        policy: RestrictedExecutionPolicy,
    }

    impl RestrictedExecutionPolicyCalls {
        /// Select only one of the UI role's two declared policy kinds.
        pub(super) const fn new(policy: RestrictedExecutionPolicy) -> Self {
            Self { policy }
        }

        /// Return the SDK enumeration value, not an untrusted numeric selector.
        const fn native_kind(&self) -> i32 {
            match self.policy {
                RestrictedExecutionPolicy::DynamicCode => 2,
                RestrictedExecutionPolicy::ExtensionPoints => 6,
            }
        }
    }

    impl RestrictedCalls for RestrictedExecutionPolicyCalls {
        /// Preserve the same policy identity in admission and all native calls.
        fn policy(&self) -> RestrictedExecutionPolicy {
            self.policy
        }

        /// Read one initialized DWORD from this process using its exact policy kind.
        ///
        /// # Errors
        /// Return the query's original same-thread Win32 error.
        #[expect(
            unsafe_code,
            reason = "Query a fixed current-process policy with its exact DWORD buffer"
        )]
        fn read(&mut self) -> Result<u32, u32> {
            let mut policy: NativeRestrictedExecutionPolicy =
                NativeRestrictedExecutionPolicy { flags: 0 };
            // SAFETY: The returned current-process pseudohandle is borrowed,
            // valid for this synchronous query and must not be closed.
            let process: HANDLE = unsafe { GetCurrentProcess() };
            // SAFETY: The closed policy kind selects exactly this initialized,
            // aligned DWORD layout. The writable output lives through the call.
            let status: i32 = unsafe {
                GetProcessMitigationPolicy(
                    process,
                    self.native_kind(),
                    (&raw mut policy).cast::<c_void>(),
                    size_of::<NativeRestrictedExecutionPolicy>(),
                )
            };
            if status == 0 {
                // SAFETY: Capture the failed query's error before another API.
                return Err(unsafe { GetLastError() });
            }
            Ok(policy.flags)
        }

        /// Apply the fixed owner's checked flags to this process only.
        ///
        /// # Errors
        /// Return the original setter failure without a weaker retry.
        #[expect(
            unsafe_code,
            reason = "Set a fixed current-process policy with its exact DWORD buffer"
        )]
        fn apply(&mut self, flags: u32) -> Result<(), u32> {
            let policy: NativeRestrictedExecutionPolicy = NativeRestrictedExecutionPolicy { flags };
            // SAFETY: The closed kind and initialized DWORD layout agree; this
            // read-only input remains live throughout the synchronous setter.
            let status: i32 = unsafe {
                SetProcessMitigationPolicy(
                    self.native_kind(),
                    (&raw const policy).cast::<c_void>(),
                    size_of::<NativeRestrictedExecutionPolicy>(),
                )
            };
            if status == 0 {
                // SAFETY: Capture the failed setter's error on this same thread.
                return Err(unsafe { GetLastError() });
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests;

/// Portable controls for the fixed flags and native failure propagation.
#[cfg(test)]
mod dll_search_tests {
    use super::{DllSearchPolicyError, configure_default_dll_search_with};

    /// An accepted setter must receive exactly application and System32 search.
    ///
    /// # Panics
    /// The setter is skipped, sees different flags or its success is discarded.
    #[test]
    fn sets_the_fixed_default_dll_search() {
        let mut requested: Option<u32> = None;
        let result: Result<(), DllSearchPolicyError> = configure_default_dll_search_with(|flags| {
            requested = Some(flags);
            Ok(())
        });
        assert_eq!(requested, Some(0x0a00));
        assert_eq!(result, Ok(()));
    }

    /// A failed setter must refuse startup with its original native code.
    ///
    /// # Panics
    /// The setter sees different flags or its original error is lost.
    #[test]
    fn preserves_the_failed_dll_search_setter_code() {
        let result: Result<(), DllSearchPolicyError> = configure_default_dll_search_with(|flags| {
            assert_eq!(flags, 0x0a00);
            Err(87)
        });
        assert_eq!(result.map_err(DllSearchPolicyError::win32_code), Err(87));
    }
}

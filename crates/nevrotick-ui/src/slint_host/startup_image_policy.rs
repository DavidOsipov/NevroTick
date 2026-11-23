// SPDX-License-Identifier: GPL-3.0-only
//! Desktop UI image-load policy for the current process before Slint starts.
//!
//! This guard reads and, if necessary, tightens the process policy once, then
//! checks the effective result. It does not cover images already loaded by
//! Windows before application startup.

use std::fmt;

/// Required `PROCESS_MITIGATION_IMAGE_LOAD_POLICY` bits from `winnt.h`.
const REQUIRED_FLAGS: u32 = 0x01 | 0x02 | 0x04;
/// The three enforcement and two audit bits from the same policy layout.
const KNOWN_FLAGS: u32 = REQUIRED_FLAGS | 0x08 | 0x10;

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
    /// Return the exact image-load flags.
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

/// Narrow binding to the calling process's image-load policy in Kernel32.
#[cfg(windows)]
mod windows {
    use super::PolicyCalls;
    use std::{ffi::c_void, mem::size_of};
    use windows_sys::Win32::Foundation::{GetLastError, HANDLE};

    /// `PROCESS_MITIGATION_POLICY::ProcessImageLoadPolicy` in the Windows SDK.
    const PROCESS_IMAGE_LOAD_POLICY: i32 = 10;

    /// The single DWORD union in `PROCESS_MITIGATION_IMAGE_LOAD_POLICY`.
    #[repr(C)]
    struct NativeImageLoadPolicy {
        /// Flags, including the documented enforcement and audit bitfields.
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
}

#[cfg(test)]
mod tests;

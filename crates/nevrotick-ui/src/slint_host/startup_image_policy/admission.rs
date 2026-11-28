// SPDX-License-Identifier: GPL-3.0-only
//! Admit the fixed startup protections before constructing any native backend.
//!
//! Each guard retains its own typed refusal. A failure stops the sequence without
//! another guard or backend attempt; protections already applied remain in force.
//! Backend construction owns its result, which this transition never interprets.

/// First protection that refused native backend construction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use = "A startup admission refusal must prevent backend construction"]
pub(crate) enum StartupAdmissionError {
    /// The image-load guard refused before the DLL search policy was attempted.
    ImageLoad(super::ImageLoadPolicyError),
    /// The process default DLL search could not be configured.
    DllSearch(super::DllSearchPolicyError),
    /// Permanent strict handles could not be admitted.
    StrictHandle(super::StrictHandlePolicyError),
    /// The dynamic-code or extension-point guard refused with its original policy.
    RestrictedExecution(super::RestrictedExecutionPolicyError),
}

/// Synchronous guards for this process's fixed pre-backend startup sequence.
trait StartupCalls {
    /// Admit the image-load protection before later startup work.
    ///
    /// # Errors
    /// Return the image-load owner's original refusal without retrying it.
    fn image_load(&mut self) -> Result<(), super::ImageLoadPolicyError>;

    /// Configure the fixed process default for subsequent DLL searches.
    ///
    /// # Errors
    /// Return the DLL-search owner's original setter refusal.
    fn dll_search(&mut self) -> Result<(), super::DllSearchPolicyError>;

    /// Admit permanent strict handles before constructing backend resources.
    ///
    /// # Errors
    /// Return the strict-handle owner's original refusal.
    fn strict_handles(&mut self) -> Result<(), super::StrictHandlePolicyError>;

    /// Admit the fixed dynamic-code policy on the current process.
    ///
    /// # Errors
    /// Return the restricted-execution owner's original dynamic-code refusal.
    fn dynamic_code(&mut self) -> Result<(), super::RestrictedExecutionPolicyError>;

    /// Admit the fixed extension-point policy on the current process.
    ///
    /// # Errors
    /// Return the restricted-execution owner's original extension-point refusal.
    fn extension_points(&mut self) -> Result<(), super::RestrictedExecutionPolicyError>;
}

/// Run all five guards in order, then consume the backend factory exactly once.
///
/// An accepted guard does not authorize another policy or undo earlier effects.
/// The factory's value, including an independently fallible result, is returned
/// unchanged and is never retried or converted into a protection failure.
///
/// # Errors
/// Preserve the first typed guard refusal; no later guard or factory is invoked.
fn before_backend_with<T>(
    calls: &mut impl StartupCalls,
    factory: impl FnOnce() -> T,
) -> Result<T, StartupAdmissionError> {
    calls
        .image_load()
        .map_err(StartupAdmissionError::ImageLoad)?;
    calls
        .dll_search()
        .map_err(StartupAdmissionError::DllSearch)?;
    calls
        .strict_handles()
        .map_err(StartupAdmissionError::StrictHandle)?;
    calls
        .dynamic_code()
        .map_err(StartupAdmissionError::RestrictedExecution)?;
    calls
        .extension_points()
        .map_err(StartupAdmissionError::RestrictedExecution)?;
    Ok(factory())
}

/// Native guards restricted to the current ordinary UI process.
#[cfg(windows)]
struct NativeCalls;

#[cfg(windows)]
impl StartupCalls for NativeCalls {
    /// Apply the image-load owner's fixed policy and effective-state checks.
    fn image_load(&mut self) -> Result<(), super::ImageLoadPolicyError> {
        super::enforce()
    }

    /// Apply the DLL-search owner's fixed process default.
    fn dll_search(&mut self) -> Result<(), super::DllSearchPolicyError> {
        super::configure_default_dll_search()
    }

    /// Apply the strict-handle owner's permanent policy.
    fn strict_handles(&mut self) -> Result<(), super::StrictHandlePolicyError> {
        super::enforce_strict_handles()
    }

    /// Apply and read back the fixed dynamic-code policy.
    fn dynamic_code(&mut self) -> Result<(), super::RestrictedExecutionPolicyError> {
        super::enforce_restricted(super::RestrictedExecutionPolicy::DynamicCode)
    }

    /// Apply and read back the fixed extension-point policy.
    fn extension_points(&mut self) -> Result<(), super::RestrictedExecutionPolicyError> {
        super::enforce_restricted(super::RestrictedExecutionPolicy::ExtensionPoints)
    }
}

/// Admit every required startup protection before consuming the backend factory.
///
/// # Errors
/// Preserve the first native protection refusal without constructing the backend.
#[cfg(windows)]
pub(crate) fn before_backend<T>(factory: impl FnOnce() -> T) -> Result<T, StartupAdmissionError> {
    before_backend_with(&mut NativeCalls, factory)
}

#[cfg(test)]
mod tests;

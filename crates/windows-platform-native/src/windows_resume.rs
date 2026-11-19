// SPDX-License-Identifier: GPL-2.0-only
//! `PowrProf` registration and matching explicit retirement for one observer.

use std::ffi::c_void;
use std::num::NonZeroU32;
use std::ptr::{NonNull, null_mut};

use crate::resume_slot::{Action, ResumeSlot};

/// Register a function callback through the exact `PowrProf` selector.
const DEVICE_NOTIFY_CALLBACK: u32 = 2;
/// Win32 success for registration, retirement and callback return.
const ERROR_SUCCESS: u32 = 0;
/// Win32 refusal when the already owned action cannot deliver.
const ERROR_GEN_FAILURE: u32 = 31;

/// Process-lifetime storage prevents a late callback from finding recycled state.
static SLOT: ResumeSlot = ResumeSlot::new();

/// C layout of `DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS` with an always-null context.
#[repr(C)]
struct SubscribeParameters {
    /// System calling convention and Win32 ULONG status.
    callback: extern "system" fn(*const c_void, u32, *const c_void) -> u32,
    /// A nullable reference has pointer layout and never contains a reference here.
    context: Option<&'static u8>,
}

/// Permanent input storage because unregistration does not promise callback drainage.
static PARAMETERS: SubscribeParameters = SubscribeParameters {
    callback: notify,
    context: None,
};

#[link(name = "powrprof")]
#[expect(unsafe_code, reason = "Exact Win32 powerbase.h import ABI")]
unsafe extern "system" {
    fn PowerRegisterSuspendResumeNotification(
        flags: u32,
        recipient: *const c_void,
        registration: *mut *mut c_void,
    ) -> u32;
    fn PowerUnregisterSuspendResumeNotification(registration: *mut c_void) -> u32;
}

/// Dispatch only the event ID. The non-unwinding system ABI aborts on a panic.
extern "system" fn notify(_context: *const c_void, event: u32, _setting: *const c_void) -> u32 {
    match SLOT.notify(event) {
        Ok(()) => ERROR_SUCCESS,
        Err(()) => ERROR_GEN_FAILURE,
    }
}

/// Why the process could not establish its resume observer.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[non_exhaustive]
pub enum ResumeRegistrationError {
    /// A previous registration attempt consumed the process's slot.
    AlreadyUsed,
    /// Exact nonzero status returned by `PowrProf`.
    Native(NonZeroU32),
    /// `PowrProf` returned success without an owned registration handle.
    MissingHandle,
}

/// Exact nonzero `PowrProf` retirement status, after Rust-side disarm.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ResumeUnregistrationError(NonZeroU32);

impl ResumeUnregistrationError {
    /// Preserve the nonzero Windows error from the attempted native release.
    #[must_use]
    pub const fn code(self) -> NonZeroU32 {
        self.0
    }
}

/// The one owned handle, kept on the subscribing thread.
#[must_use = "retain the watch through observation and close it to check native retirement"]
pub struct ResumeWatch {
    /// Taken once before the matching Win32 call.
    handle: Option<NonNull<c_void>>,
}

impl ResumeWatch {
    /// Register a one-shot action for delivered suspend or automatic-resume events.
    ///
    /// The action can arrive during registration. It must be bounded,
    /// non-panicking and independent of retirement of this watch. The Engine
    /// uses its existing synchronous invalidation request. No caller-supplied
    /// context pointer crosses into Windows.
    ///
    /// # Errors
    /// Refuse repeated attempts, nonzero native status or a missing handle.
    #[expect(
        unsafe_code,
        reason = "Pass the permanent descriptor and a writable handle out parameter"
    )]
    pub fn subscribe(
        action: impl FnOnce() -> Result<(), ()> + Send + 'static,
    ) -> Result<Self, ResumeRegistrationError> {
        if !SLOT.bind(Box::new(action)) {
            return Err(ResumeRegistrationError::AlreadyUsed);
        }
        let mut handle: *mut c_void = null_mut();
        // SAFETY: The [in] descriptor has the C layout and static lifetime
        // required by powerbase.h. The output slot is live and writable.
        let status: u32 = unsafe {
            PowerRegisterSuspendResumeNotification(
                DEVICE_NOTIFY_CALLBACK,
                (&raw const PARAMETERS).cast(),
                &raw mut handle,
            )
        };
        if let Some(error) = NonZeroU32::new(status) {
            drop(SLOT.disarm());
            return Err(ResumeRegistrationError::Native(error));
        }
        if let Some(handle) = NonNull::new(handle) {
            return Ok(Self {
                handle: Some(handle),
            });
        }
        drop(SLOT.disarm());
        Err(ResumeRegistrationError::MissingHandle)
    }

    /// Disarm delivery and attempt exactly one native unregistration.
    ///
    /// # Errors
    /// Return the original nonzero native status. The callback remains disarmed.
    pub fn close(mut self) -> Result<(), ResumeUnregistrationError> {
        self.release()
    }

    /// Retire the handle after serialized disarm. No slot lock spans the FFI call.
    ///
    /// # Errors
    /// Preserve a nonzero native retirement status.
    #[expect(
        unsafe_code,
        reason = "Retire only the unique handle returned by PowrProf"
    )]
    fn release(&mut self) -> Result<(), ResumeUnregistrationError> {
        let Some(handle) = self.handle.take() else {
            return Ok(());
        };
        // Keep arbitrary capture destructors out of native retirement.
        let _retired: Option<Action> = SLOT.disarm();
        // SAFETY: Handle came from successful registration and is consumed
        // here. The static callback and descriptor outlive delayed notifications.
        let status: u32 = unsafe { PowerUnregisterSuspendResumeNotification(handle.as_ptr()) };
        NonZeroU32::new(status).map_or(Ok(()), |error| Err(ResumeUnregistrationError(error)))
    }
}

impl Drop for ResumeWatch {
    fn drop(&mut self) {
        // Ordinary callers close explicitly. Unwinding still disarms the slot.
        let _released: Result<(), ResumeUnregistrationError> = self.release();
    }
}

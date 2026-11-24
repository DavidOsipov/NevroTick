// SPDX-License-Identifier: GPL-2.0-only
//! `PowrProf` registration and matching explicit retirement for one observer.

use std::ffi::c_void;
use std::num::NonZeroU32;
use std::ptr::{NonNull, null_mut};

use crate::resume_slot::{Action, ResumeEvent, ResumeSlot};

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

/// Accept the registering owner's output only on native success with a handle.
/// The slot must have consumed its one registration attempt.
///
/// # Errors
/// Preserve a nonzero native status or a missing handle and disarm delivery.
fn complete_registration<H>(
    slot: &ResumeSlot,
    status: u32,
    handle: Option<H>,
) -> Result<H, ResumeRegistrationError> {
    let result: Result<H, ResumeRegistrationError> = match NonZeroU32::new(status) {
        Some(error) => Err(ResumeRegistrationError::Native(error)),
        None => handle.ok_or(ResumeRegistrationError::MissingHandle),
    };
    if result.is_err() {
        drop(slot.disarm());
    }
    result
}

/// Consume the matching handle before the single unregister attempt.
/// Disarm first and retain the retired capture until the attempt returns.
///
/// # Errors
/// Preserve the nonzero status without restoring the handle or action.
fn retire_registration<H>(
    slot: &ResumeSlot,
    handle: &mut Option<H>,
    unregister: impl FnOnce(H) -> u32,
) -> Result<(), ResumeUnregistrationError> {
    let Some(handle) = handle.take() else {
        return Ok(());
    };
    let _retired: Option<Action> = slot.disarm();
    let status: u32 = unregister(handle);
    NonZeroU32::new(status).map_or(Ok(()), |error| Err(ResumeUnregistrationError(error)))
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
    pub fn subscribe(
        action: impl FnOnce() -> Result<(), ()> + Send + 'static,
    ) -> Result<Self, ResumeRegistrationError> {
        Self::register(Action::once(action))
    }

    /// Deliver each suspend or automatic-resume callback until the original watch is disarmed.
    ///
    /// Delivery preserves the event kind, is serialized and may start during registration.
    /// The action must be bounded, non-panicking and independent of this watch's retirement.
    /// An action failure is reported to Windows for that delivery, and the action remains
    /// available for a later notification.
    ///
    /// # Errors
    /// Refuse repeated registration attempts, a nonzero native status or a missing handle.
    pub fn subscribe_persistent(
        action: impl FnMut(ResumeEvent) -> Result<(), ()> + Send + 'static,
    ) -> Result<Self, ResumeRegistrationError> {
        Self::register(Action::Persistent(Box::new(action)))
    }

    /// Bind the action and request the one native registration for the process.
    ///
    /// # Errors
    /// Refuse a spent slot, a nonzero native status or a missing handle.
    #[expect(
        unsafe_code,
        reason = "Pass the permanent descriptor and a writable handle out parameter"
    )]
    fn register(action: Action) -> Result<Self, ResumeRegistrationError> {
        if !SLOT.bind(action) {
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
        let handle: NonNull<c_void> = complete_registration(&SLOT, status, NonNull::new(handle))?;
        Ok(Self {
            handle: Some(handle),
        })
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
        retire_registration(&SLOT, &mut self.handle, |handle: NonNull<c_void>| {
            // SAFETY: Handle came from successful registration and is consumed
            // here. The static callback and descriptor outlive delayed notifications.
            unsafe { PowerUnregisterSuspendResumeNotification(handle.as_ptr()) }
        })
    }
}

impl Drop for ResumeWatch {
    fn drop(&mut self) {
        // Ordinary callers close explicitly. Unwinding still disarms the slot.
        let _released: Result<(), ResumeUnregistrationError> = self.release();
    }
}

#[cfg(test)]
#[path = "windows_resume_tests.rs"]
mod fault_tests;

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::io;
    use std::ptr::null;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Weak};

    use super::{
        Action, PARAMETERS, ResumeEvent, ResumeRegistrationError, ResumeUnregistrationError,
        ResumeWatch, SLOT,
    };

    /// Observe native registration, active capture and explicit close in a fresh process.
    ///
    /// Direct calls to the registered callback exercise its Rust dispatch.
    /// The inert action records only direct calls on the test thread so ambient
    /// notifications cannot supply the expected event classes.
    ///
    /// # Errors
    /// Preserve the registration or unregistration refusal, or report a lost capture.
    ///
    /// # Panics
    /// Fail if dispatch loses an event class, close retains the capture or the slot
    /// permits a second registration.
    #[test]
    #[ignore = "requires an admitted ordinary Windows host and a fresh exact-selector process"]
    fn native_persistent_close_disarms_and_releases_capture() -> io::Result<()> {
        thread_local! {
            static DIRECT_CALL: Cell<bool> = const { Cell::new(false) };
        }
        const SUSPEND_SEEN: usize = 1;
        const AUTOMATIC_RESUME_SEEN: usize = 2;

        let seen: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
        let retained: Weak<AtomicUsize> = Arc::downgrade(&seen);
        let watch: ResumeWatch = ResumeWatch::subscribe_persistent(move |event: ResumeEvent| {
            if !DIRECT_CALL.try_with(Cell::get).unwrap_or(false) {
                return Ok(());
            }
            let bit: usize = match event {
                ResumeEvent::Suspending => SUSPEND_SEEN,
                ResumeEvent::AutomaticResume => AUTOMATIC_RESUME_SEEN,
            };
            let _previous: usize = seen.fetch_or(bit, Ordering::Relaxed);
            Ok(())
        })
        .map_err(|error: ResumeRegistrationError| {
            io::Error::other(format!("native persistent registration refused: {error:?}"))
        })?;
        assert!(matches!(
            ResumeWatch::subscribe(|| Ok(())),
            Err(ResumeRegistrationError::AlreadyUsed)
        ));
        {
            let seen: Arc<AtomicUsize> = retained
                .upgrade()
                .ok_or_else(|| io::Error::other("live watch lost its callback capture"))?;
            DIRECT_CALL.with(|direct: &Cell<bool>| {
                direct.set(true);
                assert_eq!((PARAMETERS.callback)(null(), 4, null()), 0);
                assert_eq!(seen.load(Ordering::Relaxed), SUSPEND_SEEN);
                assert_eq!((PARAMETERS.callback)(null(), 18, null()), 0);
                assert_eq!(
                    seen.load(Ordering::Relaxed),
                    SUSPEND_SEEN | AUTOMATIC_RESUME_SEEN
                );
                direct.set(false);
            });
        }
        watch.close().map_err(|error: ResumeUnregistrationError| {
            io::Error::other(format!(
                "native persistent unregistration refused: {error:?}"
            ))
        })?;
        assert!(
            retained.upgrade().is_none(),
            "closed watch retained its capture"
        );
        assert_eq!((PARAMETERS.callback)(null(), 4, null()), 0);
        assert_eq!((PARAMETERS.callback)(null(), 18, null()), 0);
        assert!(matches!(
            ResumeWatch::subscribe_persistent(|_| Ok(())),
            Err(ResumeRegistrationError::AlreadyUsed)
        ));
        Ok(())
    }

    /// Exercise the production callback in a fresh process without OS registration.
    ///
    /// # Panics
    /// Fail if unrelated or late events invoke the action, if delivery fails to
    /// report its Windows error code, or if failure leaves the action armed.
    #[test]
    fn callback_reports_failure_and_consumes_its_action() {
        let deliveries: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
        let observed: Arc<AtomicUsize> = Arc::clone(&deliveries);
        assert!(SLOT.bind(Action::once(move || {
            let _previous: usize = observed.fetch_add(1, Ordering::Relaxed);
            Err(())
        })));

        assert_eq!((PARAMETERS.callback)(null(), 7, null()), 0);
        assert_eq!(deliveries.load(Ordering::Relaxed), 0);
        assert_eq!((PARAMETERS.callback)(null(), 4, null()), 31);
        assert_eq!(deliveries.load(Ordering::Relaxed), 1);
        assert_eq!((PARAMETERS.callback)(null(), 18, null()), 0);
        assert!(SLOT.disarm().is_none());
        assert_eq!((PARAMETERS.callback)(null(), 4, null()), 0);
        assert_eq!(deliveries.load(Ordering::Relaxed), 1);
        assert!(matches!(
            ResumeWatch::subscribe_persistent(|_| Ok(())),
            Err(ResumeRegistrationError::AlreadyUsed)
        ));
    }
}

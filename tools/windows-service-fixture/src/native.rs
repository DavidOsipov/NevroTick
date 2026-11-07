// SPDX-License-Identifier: GPL-2.0-only
//! Private Win32 hosting for the two inert fixed laboratory compositions.
//!
//! A process attempts one dispatcher. Its `ServiceMain` publishes its existing
//! Rust thread handle before handler registration. The callback touches only
//! static notification state and that initialized handle; no context allocation,
//! event pointer, closure or dynamic string crosses the callback lifetime.
//! Expected errors remain fallible. An unexpected `ServiceMain` panic aborts this
//! inert process; this selects no shipping privileged-process fault policy.

use std::{
    ffi::c_void,
    ptr::{self, NonNull},
    sync::{OnceLock, atomic::AtomicBool},
    thread::{self, Thread},
    time::{Duration, Instant},
};

use windows_platform::{
    scm::{ScmControl, ScmStatus, StopNotification},
    service::{ServiceState, ServiceStatus, StatusReporter},
};
use windows_sys::Win32::{
    Foundation::GetLastError,
    System::Services::{
        RegisterServiceCtrlHandlerExW, SERVICE_STATUS, SERVICE_TABLE_ENTRYW, SetServiceStatus,
        StartServiceCtrlDispatcherW,
    },
};

use crate::{
    FixtureError, LabRole, NativeError, NativeOperation,
    host::{self, Clock, Dispatcher, StatusFactory, Wait},
};

/// Fixed self-token capture, isolated from the SCM callback surface.
#[path = "native_token.rs"]
mod token;

/// One irreversible dispatcher-attempt flag per executable process.
static DISPATCH_ATTEMPTED: AtomicBool = AtomicBool::new(false);
/// Static bounded Stop custody remains valid through process termination.
static STOP: StopNotification = StopNotification::new();
/// Exactly one initialized existing `ServiceMain` thread handle, never reset/taken.
static OWNER: OnceLock<Thread> = OnceLock::new();
/// Immutable actual owner result; no successful status is published before acknowledgement.
static OUTCOME: OnceLock<Result<(), FixtureError>> = OnceLock::new();
/// Separately owned original dispatcher failure; never replaces the owner receipt.
/// Initially empty, written only after this process's sole dispatch attempt fails.
static DISPATCH_FAILURE: OnceLock<FixtureError> = OnceLock::new();

/// Real effect adapter, containing no caller-selected name or raw callback input.
struct NativeDispatcher;

impl Dispatcher for NativeDispatcher {
    /// Connect the process using one fixed entry and the mandatory null terminator.
    ///
    /// # Errors
    /// Capture dispatcher failure immediately, without a console-mode retry.
    fn dispatch(&mut self, role: LabRole) -> Result<(), NativeError> {
        let service_main = match role {
            LabRole::Engine => engine_service_main,
            LabRole::ClockBroker => broker_service_main,
        };
        let table = [
            SERVICE_TABLE_ENTRYW {
                lpServiceName: role.wide_name().as_ptr().cast_mut(),
                lpServiceProc: Some(service_main),
            },
            SERVICE_TABLE_ENTRYW {
                lpServiceName: ptr::null_mut(),
                lpServiceProc: None,
            },
        ];
        #[expect(
            unsafe_code,
            reason = "one reviewed SCM dispatcher call with a fixed table"
        )]
        // SAFETY: The two initialized entries and static UTF-16 name outlive this
        // blocking call. The final entry is null; its callback matches the ABI.
        // The process-wide dispatch_once guard prevents a second invocation.
        let connected = unsafe { StartServiceCtrlDispatcherW(table.as_ptr()) };
        if connected == 0 {
            #[expect(unsafe_code, reason = "capture the failing dispatcher's thread error")]
            // SAFETY: GetLastError has no pointer preconditions and is read on the
            // failing thread immediately before any cleanup or other native call.
            let code = unsafe { GetLastError() };
            return Err(NativeError::new(NativeOperation::Dispatch, code));
        }
        Ok(())
    }
}

/// Per-`ServiceMain` registration facade; raw tokens never escape this module.
struct NativeFactory;

impl StatusFactory for NativeFactory {
    type Reporter = NativeReporter;

    /// Register only the static role handler after its owner handle is initialized.
    ///
    /// # Errors
    /// Preserve registration's immediate native error; construct no token on failure.
    fn register(&mut self, role: LabRole) -> Result<NativeReporter, NativeError> {
        #[expect(
            unsafe_code,
            reason = "register the fixed inert role and static callback"
        )]
        // SAFETY: The role's static name is NUL-terminated. The callback has the
        // exact system ABI and retains no event/context pointer. OWNER and STOP
        // are initialized/static before registration and outlive every callback.
        let handle = unsafe {
            RegisterServiceCtrlHandlerExW(
                role.wide_name().as_ptr(),
                Some(control_handler),
                ptr::null(),
            )
        };
        let Some(handle) = NonNull::new(handle) else {
            #[expect(unsafe_code, reason = "retain the exact registration failure")]
            // SAFETY: Capture the current thread's error immediately after the
            // failing call, before diagnostics, destruction or another API call.
            let code = unsafe { GetLastError() };
            let error = NativeError::new(NativeOperation::Register, code);
            return Err(error);
        };
        Ok(NativeReporter {
            token: Some(StatusToken(handle)),
        })
    }
}

/// Registration-produced status token; not a `CloseHandle` resource and never cloned.
struct StatusToken(NonNull<c_void>);

/// Sole reporting owner; consumes its token before the single terminal attempt.
struct NativeReporter {
    /// A registered token until terminal reporting, then permanently None.
    token: Option<StatusToken>,
}

impl StatusReporter for NativeReporter {
    type Error = NativeError;

    /// Populate every field and preserve actual `SetServiceStatus` acknowledgement.
    ///
    /// # Errors
    /// Return the original native error, or reject a terminal-token reuse locally.
    fn report(&mut self, status: ServiceStatus) -> Result<(), NativeError> {
        let terminal = status.state() == ServiceState::Stopped;
        let pointer = if terminal {
            self.token.take().map(|token| token.0)
        } else {
            self.token.as_ref().map(|token| token.0)
        };
        let Some(pointer) = pointer else {
            return Err(NativeError::new(NativeOperation::Status, 6));
        };
        let projected = ScmStatus::from_status(status);
        let raw = SERVICE_STATUS {
            dwServiceType: projected.service_type(),
            dwCurrentState: projected.current_state(),
            dwControlsAccepted: projected.controls_accepted(),
            dwWin32ExitCode: projected.win32_exit_code(),
            dwServiceSpecificExitCode: projected.service_specific_exit_code(),
            dwCheckPoint: projected.checkpoint(),
            dwWaitHint: projected.wait_hint_ms(),
        };
        #[expect(
            unsafe_code,
            reason = "one checked status report with registration-owned custody"
        )]
        // SAFETY: The private non-null token came from successful registration;
        // raw fully initializes the seven fields from checked portable status.
        // The stack value remains live for this synchronous call.
        // Terminal custody was removed before this attempt, so it cannot repeat.
        let reported = unsafe { SetServiceStatus(pointer.as_ptr(), &raw const raw) };
        if reported == 0 {
            #[expect(
                unsafe_code,
                reason = "capture the status failure before cleanup can alter it"
            )]
            // SAFETY: No native call, formatter or destructor has intervened since
            // SetServiceStatus failed on this thread.
            let code = unsafe { GetLastError() };
            let error = NativeError::new(NativeOperation::Status, code);
            return Err(error);
        }
        Ok(())
    }
}

/// Actual monotonic observer; performs no wall-clock read or adjustment.
struct NativeClock;

impl Clock for NativeClock {
    /// Obtain the next monotonic Instant on the serialized owner thread.
    fn now(&mut self) -> Instant {
        Instant::now()
    }
}

/// Finite waiting on the existing owner thread; owns no timer/event/native worker.
struct NativeWait;

impl Wait for NativeWait {
    /// Spurious wakes are allowed; the shared core rechecks Stop and the deadline.
    fn park(&mut self, timeout: Duration) {
        thread::park_timeout(timeout);
    }
}

/// Fixed engine `ServiceMain` entry; SCM argument pointers are never read or retained.
extern "system" fn engine_service_main(argc: u32, _argv: *mut *mut u16) {
    service_main(LabRole::Engine, argc);
}

/// Fixed broker `ServiceMain` entry; SCM argument pointers are never read or retained.
extern "system" fn broker_service_main(argc: u32, _argv: *mut *mut u16) {
    service_main(LabRole::ClockBroker, argc);
}

/// Execute fallible inert hosting without permitting Rust unwind across the ABI.
/// An unexpected panic terminates this fixture instead of resuming its work.
fn service_main(role: LabRole, argc: u32) {
    host::guard_service_main(|| {
        let result = if OWNER.set(thread::current()).is_err() {
            Err(FixtureError::OwnerAlreadySet)
        } else {
            host::run_service(
                role,
                argc,
                &mut NativeFactory,
                &mut token::NativeTokenObserver,
                &mut NativeClock,
                &mut NativeWait,
                &STOP,
            )
        };
        // This is only an immutable acknowledgement receipt: no cleanup, new
        // operation, logging or status-context reuse follows terminal reporting.
        // A dispatcher-return race without a receipt is explicitly unavailable.
        let _earlier_outcome = OUTCOME.set(result);
    });
}

/// Decode only supported controls and wake the pre-initialized owner on Stop.
/// This path allocates nothing, takes no Rust lock, reads no event pointer and
/// invokes no application closure. Static custody lasts until process termination.
extern "system" fn control_handler(
    control: u32,
    _event_type: u32,
    _event_data: *mut c_void,
    _context: *mut c_void,
) -> u32 {
    let decoded = STOP.handle_control(control);
    if decoded == ScmControl::Stop
        && let Some(owner) = OWNER.get()
    {
        owner.unpark();
    }
    decoded.return_code()
}

/// Enter one actual fixed-role dispatcher with no install/start-service operations.
///
/// # Errors
/// Preserve direct dispatcher/registration/status failure or a failed fixture exit.
pub(super) fn entry(role: LabRole) -> Result<(), &'static FixtureError> {
    host::dispatch_with_outcome(
        role,
        &DISPATCH_ATTEMPTED,
        &mut NativeDispatcher,
        &OUTCOME,
        &DISPATCH_FAILURE,
    )
}

// SPDX-License-Identifier: GPL-2.0-only
//! Private Win32 hosting for the two fixed laboratory compositions.
//!
//! A process attempts one dispatcher. Its `ServiceMain` publishes its existing
//! Rust thread handle before handler registration. The callback uses only
//! process-owned notification state and handles.
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
    scm::{ScmStatus, StopNotification},
    service::{ServiceState, ServiceStatus, StatusReporter},
};
use windows_sys::Win32::{
    Foundation::{ERROR_ALREADY_EXISTS, GetLastError},
    System::Services::{
        LPHANDLER_FUNCTION_EX, RegisterServiceCtrlHandlerExW, SERVICE_STATUS, SERVICE_TABLE_ENTRYW,
        SetServiceStatus, StartServiceCtrlDispatcherW,
    },
};

use crate::{
    FixtureError, LabRole, NativeBrokerClockRead, NativeError, NativeOperation, SampledClockBreak,
    broker_clock_change::{self, BrokerClockChanges, ControlDisposition, sampled_native_reader},
    host::{self, Clock, Dispatcher, HostReceipt, StatusFactory, Wait},
    native_broker_wait::{BrokerControlWake, BrokerWait, PendingCompletionWake, WakeSource},
};

/// Fixed self-token capture, isolated from the SCM callback surface.
#[path = "native_token.rs"]
mod token;

/// Compare the retained peer's decoded capture with a separate native self capture.
/// # Errors
/// Preserve the decoder or independent token observer's original refusal.
#[cfg(test)]
pub(super) fn retained_privileges_match_self(
    observed: &windows_platform_native::TokenPrivilegeObservation,
) -> Result<bool, Box<dyn std::error::Error>> {
    use crate::host::TokenObserver as _;
    use windows_platform::privilege_observation::PrivilegeSnapshot;

    let retained: PrivilegeSnapshot = token::decode_retained_privileges(observed)?;
    let independent: PrivilegeSnapshot = token::NativeTokenObserver.observe()?;
    Ok(retained == independent)
}

/// One irreversible dispatcher-attempt flag per executable process.
static DISPATCH_ATTEMPTED: AtomicBool = AtomicBool::new(false);
/// Static bounded Stop custody remains valid through process termination.
static STOP: StopNotification = StopNotification::new();
/// This broker-only process-lifetime receiver never owns an event payload.
static CLOCK_CHANGES: BrokerClockChanges = BrokerClockChanges::new();
/// The broker callback can outlive the invocation. Its one unnamed event stays
/// owned for the process lifetime because SCM provides no handler-unregister
/// acknowledgement. The invocation owns and retires its separate accept event.
static BROKER_WAKE: OnceLock<BrokerControlWake> = OnceLock::new();
/// Exactly one initialized existing `ServiceMain` thread handle, never reset/taken.
static OWNER: OnceLock<Thread> = OnceLock::new();
/// Immutable actual owner result; no successful status is published before acknowledgement.
static OUTCOME: OnceLock<HostReceipt> = OnceLock::new();
/// Separately owned original dispatcher failure; never replaces the owner receipt.
/// Initially empty, written only after this process's sole dispatch attempt fails.
static DISPATCH_FAILURE: OnceLock<FixtureError> = OnceLock::new();

/// Real effect adapter, containing no caller-selected name or raw callback input.
struct NativeDispatcher;

impl Dispatcher for NativeDispatcher {
    /// Connect the process using one fixed entry and the mandatory null terminator.
    ///
    /// # Errors
    /// Require the image-load, critical-error and extension-point policies before connecting to SCM.
    /// Preserve an admission or dispatcher failure without a console-mode retry.
    fn dispatch(&mut self, role: LabRole) -> Result<(), FixtureError> {
        windows_platform_native::enforce_current_image_load_policy()
            .map_err(FixtureError::ImageLoadPolicy)?;
        windows_platform_native::enforce_service_critical_error_mode()
            .map_err(FixtureError::ServiceErrorMode)?;
        windows_platform_native::enforce_current_extension_point_policy()
            .map_err(FixtureError::ExtensionPointPolicy)?;
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
            return Err(FixtureError::Native(NativeError::new(
                NativeOperation::Dispatch,
                code,
            )));
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
    /// Preserve wake preparation or registration failure before constructing a token.
    fn register(&mut self, role: LabRole) -> Result<NativeReporter, NativeError> {
        let (handler, changes): (LPHANDLER_FUNCTION_EX, Option<&'static BrokerClockChanges>) =
            match role {
                LabRole::Engine => (Some(engine_control_handler), None),
                LabRole::ClockBroker => {
                    let wake: BrokerControlWake = BrokerControlWake::new()?;
                    if BROKER_WAKE.set(wake).is_err() {
                        return Err(NativeError::new(
                            NativeOperation::BrokerWakePrepare,
                            ERROR_ALREADY_EXISTS,
                        ));
                    }
                    (Some(broker_control_handler), Some(&CLOCK_CHANGES))
                }
            };
        #[expect(
            unsafe_code,
            reason = "register the fixed inert role and static callback"
        )]
        // SAFETY: The role's static name is NUL-terminated. The callback has the
        // exact system ABI and receives no context pointer. Applicable callback
        // state is initialized before registration and outlives every callback.
        let handle = unsafe {
            RegisterServiceCtrlHandlerExW(role.wide_name().as_ptr(), handler, ptr::null())
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
            changes,
        })
    }
}

/// Registration-produced status token; not a `CloseHandle` resource and never cloned.
struct StatusToken(NonNull<c_void>);

/// Sole reporting owner; consumes its token before the single terminal attempt.
struct NativeReporter {
    /// A registered token until terminal reporting, then permanently None.
    token: Option<StatusToken>,
    /// Original broker callback receiver issued by this registration path.
    changes: Option<&'static BrokerClockChanges>,
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
        broker_clock_change::report_status(self.changes, status.state(), |extra: u32| {
            let raw: SERVICE_STATUS = SERVICE_STATUS {
                dwServiceType: projected.service_type(),
                dwCurrentState: projected.current_state(),
                dwControlsAccepted: projected.controls_accepted() | extra,
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
        })
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

impl Wait for BrokerWait {
    /// Keep the existing thread park available for bounded fallback.
    fn park(&mut self, timeout: Duration) {
        thread::park_timeout(timeout);
    }

    /// Observe only scheduling hints on the same owner, or fall back after error.
    fn park_with_completion(
        &mut self,
        timeout: Duration,
        completion: Option<PendingCompletionWake<'_>>,
    ) -> Result<(), NativeError> {
        if let Some(control) = BROKER_WAKE.get()
            && self.observe(control, timeout, completion)? != WakeSource::Fallback
        {
            return Ok(());
        }
        self.park(timeout);
        Ok(())
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

/// Execute fallible hosting without permitting Rust unwind across the ABI.
/// An unexpected panic terminates this fixture instead of resuming its work.
fn service_main(role: LabRole, argc: u32) {
    host::guard_service_main(|| {
        let receipt: HostReceipt = if OWNER.set(thread::current()).is_err() {
            HostReceipt::owner_already_set(role)
        } else {
            match role {
                LabRole::Engine => host::run_service(
                    role,
                    argc,
                    &mut NativeFactory,
                    &mut token::NativeTokenObserver,
                    &mut NativeClock,
                    &mut NativeWait,
                    &STOP,
                ),
                LabRole::ClockBroker => host::run_broker_service_with_worker(
                    argc,
                    &mut NativeFactory,
                    &mut token::NativeTokenObserver,
                    &mut NativeClock,
                    &mut BrokerWait::new(),
                    &STOP,
                    (
                        sampled_native_reader(&CLOCK_CHANGES),
                        crate::broker_endpoint::BrokerEndpointWorker::new(
                            crate::native_broker_endpoint::NativeBrokerEndpoint::acquire,
                        ),
                    ),
                ),
            }
        };
        // This is only an immutable acknowledgement receipt: no cleanup, new
        // operation, logging or status-context reuse follows terminal reporting.
        // A dispatcher-return race without a receipt is explicitly unavailable.
        let _earlier_outcome = OUTCOME.set(receipt);
    });
}

/// The Engine's static callback admits only its original controls.
extern "system" fn engine_control_handler(
    control: u32,
    _event_type: u32,
    _event_data: *mut c_void,
    _context: *mut c_void,
) -> u32 {
    handle_control(control, 0, None)
}

/// The broker also accepts clock and power notices on its live receiver.
/// Decode the control/subtype without reading or retaining the event pointer.
extern "system" fn broker_control_handler(
    control: u32,
    event_type: u32,
    _event_data: *mut c_void,
    _context: *mut c_void,
) -> u32 {
    handle_control(control, event_type, Some(&CLOCK_CHANGES))
}

/// Publish the bounded owning wake after the control has updated its mailbox.
fn handle_control(control: u32, event_type: u32, changes: Option<&BrokerClockChanges>) -> u32 {
    let disposition: ControlDisposition =
        broker_clock_change::deliver(changes, &STOP, control, event_type);
    if disposition.wake {
        if changes.is_some()
            && let Some(wake) = BROKER_WAKE.get()
        {
            wake.signal();
        }
        // Retain the Rust notification for Engine and broker fallback waits.
        if let Some(owner) = OWNER.get() {
            owner.unpark();
        }
    }
    disposition.status
}

/// Borrow the immutable snapshot from the same once-published actual result.
/// Before a receipt exists this returns None; observation never polls, consumes,
/// drains, reports status or acknowledges worker/native/durable completion.
pub(super) fn retained_diagnostics()
-> Option<&'static windows_platform::diagnostics::DiagnosticSnapshot> {
    OUTCOME.get().map(HostReceipt::diagnostics)
}

/// Read the actual broker receipt's last historical attempt, when available.
pub(super) fn retained_broker_clock_read() -> Option<NativeBrokerClockRead> {
    OUTCOME.get().and_then(HostReceipt::broker_clock_read)
}

/// Read the last sampled break from the original broker's terminal receipt.
pub(super) fn retained_broker_clock_break() -> Option<SampledClockBreak> {
    OUTCOME.get().and_then(HostReceipt::broker_clock_break)
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

#[cfg(test)]
#[path = "native_image_load_tests.rs"]
mod image_load_tests;

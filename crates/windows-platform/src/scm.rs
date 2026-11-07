// SPDX-License-Identifier: GPL-2.0-only
//! Portable SCM status projection and one bounded, sticky Stop notification.
//!
//! The lifecycle owner supplies immutable checked [`ServiceStatus`] values. Their
//! projection has the seven numeric `SERVICE_STATUS` fields in their documented
//! namespaces; it is not a C-layout structure, acknowledgement or installed-state
//! observation. Every report uses own-process type `0x10`, never the packaged
//! `0x200` flag. A native adapter must separately preserve raw queried type flags.
//!
//! Raw controls are decoded without event pointers, payloads, allocation or I/O.
//! [`StopNotification`] retains one atomic bit, suitable for process-lifetime
//! static custody. Handling a control cannot report status or mutate a lifecycle:
//! the current owner still decides the authorized transition and actual cleanup.
//! No notification authenticates its caller or confers clock authority. This
//! module selects no native callback, wake mechanism, name, account or budget.
//!
//! A notification can be statically retained without resetting a prior Stop:
//! ```
//! use windows_platform::scm::{ScmControl, StopNotification};
//! static NOTIFICATION: StopNotification = StopNotification::new();
//! assert_eq!(NOTIFICATION.handle_control(4), ScmControl::Interrogate);
//! assert!(!NOTIFICATION.stop_requested());
//! assert_eq!(NOTIFICATION.handle_control(1).return_code(), 0);
//! assert!(NOTIFICATION.stop_requested());
//! assert_eq!(NOTIFICATION.handle_control(255).return_code(), 120);
//! assert!(NOTIFICATION.stop_requested());
//! ```
//!
//! Raw output cannot bypass lifecycle-owned construction:
//! ```compile_fail,E0451
//! use windows_platform::scm::ScmStatus;
//! let forged = ScmStatus { service_type: 0x200, current_state: 4,
//!     controls_accepted: 1, win32_exit_code: 0, service_specific_exit_code: 0,
//!     checkpoint: 0, wait_hint_ms: 0 };
//! ```
//!
//! The notification's atomic state is private and has no clearing operation:
//! ```compile_fail,E0451
//! use std::sync::atomic::AtomicBool;
//! use windows_platform::scm::StopNotification;
//! let forged = StopNotification { stop: AtomicBool::new(false) };
//! ```

use std::sync::atomic::{AtomicBool, Ordering};

use crate::service::{ServiceState, ServiceStatus, StopOutcome};

/// Immutable numeric projection, constructible only from a lifecycle-owned status.
/// This is a portable value; a native adapter must explicitly populate its FFI DTO.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ScmStatus {
    /// Fixed own-process type, without installed package or interactive flags.
    service_type: u32,
    /// Stopped, pending startup, pending stop or Running in the SCM namespace.
    current_state: u32,
    /// Stop only while Running; zero in every other admitted state.
    controls_accepted: u32,
    /// Zero for success; service-specific-error marker for a failed terminal exit.
    win32_exit_code: u32,
    /// Nonzero failure code only beside the service-specific-error marker.
    service_specific_exit_code: u32,
    /// Checked pending progress; zero in stable states.
    checkpoint: u32,
    /// Checked pending milliseconds; zero in stable states.
    wait_hint_ms: u32,
}

impl ScmStatus {
    /// Project one immutable checked status without performing or acknowledging I/O.
    /// Input invariants belong to the private lifecycle construction; every field
    /// retains its units and namespace, without a second raw admission path.
    #[must_use]
    pub const fn from_status(status: ServiceStatus) -> Self {
        let current_state = match status.state() {
            ServiceState::Stopped => 1,
            ServiceState::StartPending => 2,
            ServiceState::StopPending => 3,
            ServiceState::Running => 4,
        };
        let (win32_exit_code, service_specific_exit_code) = match status.outcome() {
            Some(StopOutcome::Failed(code)) => (1066, code.get()),
            None | Some(StopOutcome::Clean) => (0, 0),
        };
        Self {
            service_type: 0x10,
            current_state,
            controls_accepted: if status.accepts_stop() { 1 } else { 0 },
            win32_exit_code,
            service_specific_exit_code,
            checkpoint: status.checkpoint(),
            wait_hint_ms: status.wait_hint_ms(),
        }
    }

    /// Read fixed own-process type, without inferring actual deployment state.
    #[must_use]
    pub const fn service_type(self) -> u32 {
        self.service_type
    }

    /// Read the SCM state value for the lifecycle-selected status.
    #[must_use]
    pub const fn current_state(self) -> u32 {
        self.current_state
    }

    /// Read the accepted-control bitmask; this grants no caller authority.
    #[must_use]
    pub const fn controls_accepted(self) -> u32 {
        self.controls_accepted
    }

    /// Read the Win32 exit code, keeping application failures in their own field.
    #[must_use]
    pub const fn win32_exit_code(self) -> u32 {
        self.win32_exit_code
    }

    /// Read the exact nonzero application failure or zero for non-failed statuses.
    #[must_use]
    pub const fn service_specific_exit_code(self) -> u32 {
        self.service_specific_exit_code
    }

    /// Read actual pending progress supplied by the serialized lifecycle owner.
    #[must_use]
    pub const fn checkpoint(self) -> u32 {
        self.checkpoint
    }

    /// Read the checked millisecond hint, without changing the fixed owner deadline.
    #[must_use]
    pub const fn wait_hint_ms(self) -> u32 {
        self.wait_hint_ms
    }
}

/// Bounded control classification, without retaining raw input or event data.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScmControl {
    /// Notify the owner of Stop; notification alone changes no lifecycle state.
    Stop,
    /// Succeed without re-reporting status or changing state.
    Interrogate,
    /// All other controls, including custom commands and unimplemented events.
    Unsupported,
}

impl ScmControl {
    /// Admit only Stop and Interrogate; every other raw value is unsupported.
    /// No value selects protected management or enables an additional event type.
    #[must_use]
    pub const fn decode(raw: u32) -> Self {
        match raw {
            1 => Self::Stop,
            4 => Self::Interrogate,
            _ => Self::Unsupported,
        }
    }

    /// Read the `HandlerEx` result: success or `ERROR_CALL_NOT_IMPLEMENTED`.
    /// Success acknowledges this classification, never cleanup or an effect.
    #[must_use]
    pub const fn return_code(self) -> u32 {
        match self {
            Self::Stop | Self::Interrogate => 0,
            Self::Unsupported => 120,
        }
    }
}

/// One process-lifetime-compatible Stop bit; repeated notifications coalesce.
/// It owns no queue, callback, event, pointer or lifecycle and exposes no reset.
#[derive(Debug, Default)]
pub struct StopNotification {
    /// Release-published Stop observation, acquired by the serialized owner.
    stop: AtomicBool,
}

impl StopNotification {
    /// Construct an independent clear bit, without resetting any existing instance.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            stop: AtomicBool::new(false),
        }
    }

    /// Classify a raw control and publish at most one sticky Stop observation.
    /// Work is bounded to classification and a single atomic store: no allocation,
    /// blocking, user code, reporting or lifecycle transition occurs here.
    #[must_use]
    pub fn handle_control(&self, raw: u32) -> ScmControl {
        let control = ScmControl::decode(raw);
        if control == ScmControl::Stop {
            self.stop.store(true, Ordering::Release);
        }
        control
    }

    /// Acquire the sticky observation, without consuming it or authorizing an effect.
    #[must_use]
    pub fn stop_requested(&self) -> bool {
        self.stop.load(Ordering::Acquire)
    }
}

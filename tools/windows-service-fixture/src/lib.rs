// SPDX-License-Identifier: GPL-2.0-only
//! Fixed-role laboratory service hosts, separate from the product workspace.
//!
//! The two public entries select fixed laboratory roles. They accept no service
//! names, handles, accounts, commands or function pointers. Windows hosting is
//! source preparation only; ordinary GNU entry points return `WindowsOnly`.
//! Running means that the admitted control loop is available, never that a
//! product service, authenticated time, provisioning or clock authority exists.
#![deny(unsafe_code)]

#[cfg(any(windows, test))]
#[forbid(unsafe_code)]
mod broker_clock_change;
#[cfg(any(windows, test))]
#[forbid(unsafe_code)]
mod broker_endpoint;
#[cfg(any(windows, test))]
#[forbid(unsafe_code)]
mod diagnostics;
#[cfg(test)]
#[forbid(unsafe_code)]
mod engine_worker;
mod error;
#[cfg(any(windows, test))]
#[forbid(unsafe_code)]
mod host;
#[cfg(windows)]
mod native;
#[cfg(windows)]
mod native_broker_endpoint;
#[cfg(windows)]
mod native_broker_wait;
#[cfg(all(windows, test))]
mod native_etw_capture;
#[cfg(all(windows, test))]
mod native_etw_writer;
#[cfg(all(windows, test))]
mod native_peer_process;
#[cfg(all(windows, test))]
mod native_pipe_identity;
#[cfg(all(windows, test))]
mod native_pipe_pair;
#[cfg(all(windows, test))]
mod native_pipe_read;
#[cfg(all(windows, test))]
mod native_pipe_worker;
#[cfg(all(windows, test))]
mod native_pipe_write;
#[cfg(test)]
mod private_clock_wire;
#[cfg(test)]
mod scm_access_probe;
#[cfg(all(windows, test))]
mod scm_observation_tests;

pub use error::{
    FixtureError, NativeError, NativeOperation, TokenCleanupOutcome, TokenObservationCause,
    TokenObservationFailure,
};

/// Bounded detail from the original dispatcher or pre-SCM policy refusal.
///
/// The variant and its typed payload identify the failing stage and the values
/// available at that stage. Fields absent from that payload were not recorded.
/// A native code of zero is still an observed code.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DispatchFailureDetail {
    /// The original native operation and its captured error code.
    Native(NativeError),
    /// DLL search or image-load refusal, with its original stage and known facts.
    #[cfg(windows)]
    ImageLoad(windows_platform_native::ImageLoadPolicyError),
    /// Critical-error mode refusal, with the exact pair of reported mode words.
    #[cfg(windows)]
    CriticalErrorMode(windows_platform_native::ServiceErrorModeError),
    /// Extension-point refusal, with its original stage and known facts.
    #[cfg(windows)]
    ExtensionPoint(windows_platform_native::ExtensionPointPolicyError),
    /// Permanent handle-check refusal, with its original stage and observed word.
    #[cfg(windows)]
    StrictHandle(windows_platform_native::StrictHandlePolicyError),
    /// The dispatcher returned a different cause with no startup-policy detail.
    Other(windows_platform::diagnostics::PrimaryCause),
}

/// The first retained dispatch refusal's fixed role, detail and local routing result.
/// This historical view grants no service, policy or diagnostic delivery authority.
#[must_use = "the dispatch diagnostic is the historical result of the first refusal"]
pub struct DispatchDiagnostic {
    /// Role that selected the original dispatcher and required diagnostic route.
    role: windows_platform::service::ServiceRole,
    /// Bounded fields taken from that same original error.
    detail: DispatchFailureDetail,
    /// The original sealed required route and its independently bounded health.
    snapshot: windows_platform::diagnostics::DiagnosticSnapshot,
}

impl DispatchDiagnostic {
    /// Read the original fixed role, without granting any of its privileges.
    #[must_use]
    pub const fn role(&self) -> windows_platform::service::ServiceRole {
        self.role
    }

    /// Read the exact refusal fields available at the original failing stage.
    #[must_use]
    pub const fn detail(&self) -> DispatchFailureDetail {
        self.detail
    }

    /// Borrow the same sealed local route returned by `retained_dispatch_diagnostics`.
    #[must_use]
    pub const fn snapshot(&self) -> &windows_platform::diagnostics::DiagnosticSnapshot {
        &self.snapshot
    }
}

#[cfg(any(windows, test))]
pub use broker_clock_change::{
    BrokerClockRead, ClockReadRetirement, SampledClockBreak, SampledClockDiscontinuity,
};

#[cfg(windows)]
pub use broker_clock_change::{BrokerClockSample, BrokerClockSampleError};

/// Historical last broker read from the native service fixture.
#[cfg(windows)]
pub type NativeBrokerClockRead = BrokerClockRead<BrokerClockSample, BrokerClockSampleError>;

/// Host-only scripted observation of the same broker receiver.
#[cfg(all(not(windows), test))]
type NativeBrokerClockRead = BrokerClockRead<u64, u32>;

/// Fixed role selected by its executable composition, never by SCM arguments.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum LabRole {
    /// Inert laboratory acquisition-role host, with no networking or clock calls.
    Engine,
    /// Read-only laboratory broker-role host, with no privileged effects.
    ClockBroker,
}

impl LabRole {
    /// Read the exact lab-only identity; final product identities remain pending.
    #[cfg(test)]
    const fn name(self) -> &'static str {
        match self {
            Self::Engine => "NevroTick-Lab-S02-Engine",
            Self::ClockBroker => "NevroTick-Lab-S02-Broker",
        }
    }

    /// Select the matching portable owner without granting operating-system rights.
    #[cfg(any(windows, test))]
    const fn service_role(self) -> windows_platform::service::ServiceRole {
        match self {
            Self::Engine => windows_platform::service::ServiceRole::Engine,
            Self::ClockBroker => windows_platform::service::ServiceRole::ClockBroker,
        }
    }

    /// Read the static NUL-terminated UTF-16 projection of the exact lab identity.
    /// This sealed lookup allocates nothing and accepts no external name input.
    #[cfg(any(windows, test))]
    const fn wide_name(self) -> &'static [u16] {
        match self {
            Self::Engine => &[
                78, 101, 118, 114, 111, 84, 105, 99, 107, 45, 76, 97, 98, 45, 83, 48, 50, 45, 69,
                110, 103, 105, 110, 101, 0,
            ],
            Self::ClockBroker => &[
                78, 101, 118, 114, 111, 84, 105, 99, 107, 45, 76, 97, 98, 45, 83, 48, 50, 45, 66,
                114, 111, 107, 101, 114, 0,
            ],
        }
    }
}

/// Enter only the fixed inert laboratory engine host.
///
/// # Errors
/// Returns `WindowsOnly` on other hosts, or the actual bounded fixture failure.
pub fn engine_entry() -> Result<(), &'static FixtureError> {
    entry(LabRole::Engine)
}

/// Enter only the fixed read-only laboratory broker host.
///
/// # Errors
/// Returns `WindowsOnly` on other hosts, or the actual bounded fixture failure.
pub fn broker_entry() -> Result<(), &'static FixtureError> {
    entry(LabRole::ClockBroker)
}

/// Read the broker's last raw observation from this process's terminal receipt.
/// This historical value carries no current mapping or correction authority.
#[cfg(windows)]
#[must_use]
pub fn retained_broker_clock_read() -> Option<NativeBrokerClockRead> {
    native::retained_broker_clock_read()
}

/// Read the broker's latest sampled invalidation after the invocation returns.
/// The record remains historical through any subsequent re-observation.
#[cfg(windows)]
#[must_use]
pub fn retained_broker_clock_break() -> Option<SampledClockBreak> {
    native::retained_broker_clock_break()
}

/// Borrow the bounded historical snapshot from this process's actual host receipt.
///
/// Returns `None` before a receipt exists, after dispatch without an owner receipt,
/// and on non-Windows hosts. Observation neither polls nor drains diagnostics and
/// cannot acknowledge work, cleanup, native delivery, service identity or UTC truth.
/// The snapshot and original operation result belong to the same retained receipt.
#[must_use]
pub fn retained_diagnostics() -> Option<&'static windows_platform::diagnostics::DiagnosticSnapshot>
{
    #[cfg(windows)]
    {
        native::retained_diagnostics()
    }
    #[cfg(not(windows))]
    {
        None
    }
}

/// Borrow the original dispatch failure diagnostic for either fixed role.
///
/// Returns `None` before a failure is retained, after successful dispatch, and
/// on non-Windows hosts. A duplicate entry cannot replace the first record.
/// This in-process snapshot is separate from `retained_diagnostics` and adds
/// no native delivery, lifecycle acknowledgement or protected effects.
#[must_use]
pub fn retained_dispatch_diagnostics()
-> Option<&'static windows_platform::diagnostics::DiagnosticSnapshot> {
    retained_dispatch_diagnostic().map(DispatchDiagnostic::snapshot)
}

/// Borrow the first dispatch refusal's typed detail and its original local route.
/// Returns `None` on non-Windows hosts or when no dispatcher failure was retained.
/// This observation neither reroutes the event nor reads the `ServiceMain` receipt.
#[must_use]
pub fn retained_dispatch_diagnostic() -> Option<&'static DispatchDiagnostic> {
    #[cfg(windows)]
    {
        native::retained_dispatch_diagnostic()
    }
    #[cfg(not(windows))]
    {
        None
    }
}

/// Select a native fixture without adding a console/debug fallback.
///
/// # Errors
/// Propagate native hosting failure; reject every non-Windows invocation.
fn entry(role: LabRole) -> Result<(), &'static FixtureError> {
    #[cfg(windows)]
    {
        native::entry(role)
    }
    #[cfg(not(windows))]
    {
        let _ = role;
        Err(&FixtureError::WindowsOnly)
    }
}

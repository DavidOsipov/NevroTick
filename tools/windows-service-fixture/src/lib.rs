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
#[cfg(all(windows, test))]
mod native_etw_capture;
#[cfg(all(windows, test))]
mod native_etw_writer;
#[cfg(all(windows, test))]
mod native_pipe_pair;
#[cfg(all(windows, test))]
mod native_pipe_read;
#[cfg(test)]
mod private_clock_wire;

pub use error::{
    FixtureError, NativeError, NativeOperation, TokenCleanupOutcome, TokenObservationCause,
    TokenObservationFailure,
};

#[cfg(any(windows, test))]
pub use broker_clock_change::{
    BrokerClockRead, ClockReadRetirement, SampledClockBreak, SampledClockDiscontinuity,
};

/// Historical last broker read from the native service fixture.
#[cfg(windows)]
pub type NativeBrokerClockRead = BrokerClockRead<
    windows_platform_native::ClockCoordinates,
    windows_platform_native::ClockReadError,
>;

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

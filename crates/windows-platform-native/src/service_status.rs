// SPDX-License-Identifier: GPL-2.0-only
//! Checked input and a historical report for a local SCM status query.

use std::num::NonZeroU32;

/// Maximum service key length in wide code units, excluding the terminator.
const MAX_KEY_UNITS: usize = 256;
/// Win32 service type for a dedicated process.
const SERVICE_WIN32_OWN_PROCESS: u32 = 0x10;
/// Win32 state reported after the service starts running.
const SERVICE_RUNNING: u32 = 4;

/// An immutable, NUL-terminated wide service key, suitable for `OpenServiceW`.
/// The caller selects an SCM key, as distinct from a localized display name.
#[derive(Clone, Copy, Debug)]
pub struct ServiceKey<'a> {
    /// Original checked units, including exactly one final NUL.
    wide: &'a [u16],
}

impl<'a> ServiceKey<'a> {
    /// Check the length, terminator, and excluded units without allocation.
    ///
    /// # Errors
    /// Return [`InvalidServiceKey`] for empty or overlong content, a missing or
    /// embedded NUL, or a slash or backslash in the content.
    pub fn new(wide: &'a [u16]) -> Result<Self, InvalidServiceKey> {
        let Some((&0, content)) = wide.split_last() else {
            return Err(InvalidServiceKey);
        };
        if !(1..=MAX_KEY_UNITS).contains(&content.len())
            || content
                .iter()
                .any(|&unit| matches!(unit, 0 | 0x002f | 0x005c))
        {
            return Err(InvalidServiceKey);
        }
        Ok(Self { wide })
    }

    /// Return the same checked units with the terminator still present.
    #[must_use]
    pub const fn as_wide_with_nul(self) -> &'a [u16] {
        self.wide
    }
}

/// The wide units do not meet the service key representation contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InvalidServiceKey;

/// Service type, state, and process ID from the last status reported to SCM.
///
/// The report can lag service transitions. A caller authenticating a broker must
/// independently bind the connected handle to the current service, process, and token.
#[must_use = "retain the actual service status report"]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReportedServiceStatus {
    /// Win32 `dwServiceType` from the report.
    service_type: u32,
    /// Win32 `dwCurrentState` from the report.
    state: u32,
    /// Win32 `dwProcessId` from the same report, possibly absent or stale.
    process_id: u32,
}

impl ReportedServiceStatus {
    /// Preserve the selected fields from a successful status query.
    pub(crate) const fn new(service_type: u32, state: u32, process_id: u32) -> Self {
        Self {
            service_type,
            state,
            process_id,
        }
    }

    /// Return the reported Win32 service type bits.
    #[must_use]
    pub const fn service_type(self) -> u32 {
        self.service_type
    }

    /// Return the reported Win32 service state value.
    #[must_use]
    pub const fn state(self) -> u32 {
        self.state
    }

    /// Return the reported PID of an own-process service reported as running.
    /// The PID is a lead for further observation and can be stale or reused.
    ///
    /// # Errors
    /// Refuse any other service type, a non-running state, or a zero PID.
    pub fn reported_running_process(self) -> Result<NonZeroU32, RunningServiceRefusal> {
        if self.service_type != SERVICE_WIN32_OWN_PROCESS {
            return Err(RunningServiceRefusal::NotOwnProcess);
        }
        if self.state != SERVICE_RUNNING {
            return Err(RunningServiceRefusal::NotRunning);
        }
        NonZeroU32::new(self.process_id).ok_or(RunningServiceRefusal::MissingProcessId)
    }
}

/// Reason the report cannot supply a candidate running service PID.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RunningServiceRefusal {
    /// The reported service type differs from an own-process Win32 service.
    NotOwnProcess,
    /// The reported state differs from `SERVICE_RUNNING`.
    NotRunning,
    /// The otherwise eligible report contains a zero PID.
    MissingProcessId,
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use super::{InvalidServiceKey, ReportedServiceStatus, RunningServiceRefusal, ServiceKey};

    /// Check exact key boundaries and preserve a surrogate pair without normalization.
    /// # Panics
    /// Require every valid key to preserve its full original sequence of wide units.
    #[test]
    fn service_key_preserves_valid_wide_units() {
        let shortest: [u16; 2] = [0x0041, 0];
        let multilingual: [u16; 4] = [0x0410, 0xd83d, 0xde80, 0];
        let mut longest: [u16; 257] = [0x0041; 257];
        longest[256] = 0;
        let valid: [&[u16]; 3] = [&shortest, &multilingual, &longest];
        for wide in valid {
            assert_eq!(
                ServiceKey::new(wide).map(ServiceKey::as_wide_with_nul),
                Ok(wide)
            );
        }
    }

    /// Refuse each representation error independently of service lookup.
    /// # Panics
    /// Require every malformed key to return the representation error.
    #[test]
    fn service_key_refuses_malformed_wide_units() {
        let mut too_long: [u16; 258] = [0x0041; 258];
        too_long[257] = 0;
        let invalid: [&[u16]; 7] = [
            &[],
            &[0],
            &[0x0041],
            &[0x0041, 0, 0],
            &[0x0041, 0x002f, 0],
            &[0x0041, 0x005c, 0],
            &too_long,
        ];
        for wide in invalid {
            assert!(matches!(ServiceKey::new(wide), Err(InvalidServiceKey)));
        }
    }

    /// A documented own-process running report preserves the type, state, and PID.
    /// # Panics
    /// Require the reported service type, Running state and nonzero process ID.
    #[test]
    fn status_reports_nonzero_running_own_process() {
        let status: ReportedServiceStatus = ReportedServiceStatus::new(0x10, 4, 42);
        assert_eq!(status.service_type(), 0x10);
        assert_eq!(status.state(), 4);
        assert_eq!(
            status.reported_running_process().map(NonZeroU32::get),
            Ok(42)
        );
    }

    /// Distinguish the adjacent shared-service, absent-PID, and state refusals.
    /// # Panics
    /// Require the exact refusal when the inputs cannot establish a usable Running PID.
    #[test]
    fn status_refuses_unusable_running_process_reports() {
        assert_eq!(
            ReportedServiceStatus::new(0x20, 4, 42).reported_running_process(),
            Err(RunningServiceRefusal::NotOwnProcess)
        );
        let mixed_type: ReportedServiceStatus = ReportedServiceStatus::new(0x110, 4, 42);
        assert_eq!(mixed_type.service_type(), 0x110);
        assert_eq!(
            mixed_type.reported_running_process(),
            Err(RunningServiceRefusal::NotOwnProcess)
        );
        for state in [1_u32, 2, 3, 5, 6, 7, u32::MAX] {
            assert_eq!(
                ReportedServiceStatus::new(0x10, state, 42).reported_running_process(),
                Err(RunningServiceRefusal::NotRunning)
            );
        }
        assert_eq!(
            ReportedServiceStatus::new(0x10, 4, 0).reported_running_process(),
            Err(RunningServiceRefusal::MissingProcessId)
        );
    }
}

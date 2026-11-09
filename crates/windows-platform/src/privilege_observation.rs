// SPDX-License-Identifier: GPL-2.0-only
//! Bounded, inert decoding of one successful self-token privilege capture.
//!
//! This parser establishes representation facts only. The native caller must
//! establish successful API completion, primary-token provenance and cleanup.
//! A snapshot is an observation, never a persistent authority grant.
//!
//! Snapshot fields cannot bypass checked decoding:
//! ```compile_fail,E0616
//! use windows_platform::privilege_observation::{PrivilegeLuid, PrivilegeSnapshot, PrivilegeState, ServicePrivilegeIds};
//! let ids = ServicePrivilegeIds::new([
//!     PrivilegeLuid::new(1, 0), PrivilegeLuid::new(2, 0),
//!     PrivilegeLuid::new(3, 0), PrivilegeLuid::new(4, 0),
//! ]).unwrap();
//! let mut snapshot = PrivilegeSnapshot::decode(&[0; 4], ids).unwrap();
//! snapshot.states[0] = PrivilegeState::PresentEnabled;
//! ```
//! Fixed lookup identities cannot bypass duplicate checks:
//! ```compile_fail,E0616
//! use windows_platform::privilege_observation::{PrivilegeLuid, ServicePrivilegeIds};
//! let mut ids = ServicePrivilegeIds::new([
//!     PrivilegeLuid::new(1, 0), PrivilegeLuid::new(2, 0),
//!     PrivilegeLuid::new(3, 0), PrivilegeLuid::new(4, 0),
//! ]).unwrap();
//! ids.values[1] = PrivilegeLuid::new(1, 0);
//! ```

use garde::Validate;
use std::{error::Error, fmt};

use crate::service::ServiceRole;

/// Maximum entries accepted from one complete token capture.
pub const MAX_CAPTURE_PRIVILEGES: usize = 128;
/// Exact upper bound: a four-byte count followed by 128 twelve-byte entries.
pub const MAX_CAPTURE_BYTES: usize = 1540;

/// Fixed locally resolved privileges used by role admission and named observations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServicePrivilege {
    /// Permission to change system time.
    SystemTime,
    /// Permission to debug other processes.
    Debug,
    /// Permission to impersonate a client.
    Impersonate,
    /// Traverse-notification privilege explicitly retained by both service roles.
    ChangeNotify,
}

impl ServicePrivilege {
    /// Return the stable slot shared by the fixed native lookup and decoder.
    const fn index(self) -> usize {
        match self {
            Self::SystemTime => 0,
            Self::Debug => 1,
            Self::Impersonate => 2,
            Self::ChangeNotify => 3,
        }
    }
}

/// Opaque complete Windows LUID value, with all 64 identity bits preserved.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrivilegeLuid {
    /// Low unsigned half of the identifier.
    low: u32,
    /// High signed half of the identifier.
    high: i32,
}

impl PrivilegeLuid {
    /// Preserve an operating-system lookup result without narrowing either half.
    #[must_use]
    pub const fn new(low: u32, high: i32) -> Self {
        Self { low, high }
    }
}

/// Four successfully resolved, pairwise distinct fixed privilege identities.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ServicePrivilegeIds {
    /// Identities in `SystemTime`, `Debug`, `Impersonate`, `ChangeNotify` order.
    values: [PrivilegeLuid; 4],
}

impl ServicePrivilegeIds {
    /// Bind all four successful lookups; duplicate identities are indeterminate.
    ///
    /// # Errors
    /// Returns `DuplicateTargetIds` if any pair denotes the same LUID.
    pub fn new(values: [PrivilegeLuid; 4]) -> Result<Self, PrivilegeCaptureError> {
        for (index, value) in values.iter().enumerate() {
            if values[..index].contains(value) {
                return Err(PrivilegeCaptureError::DuplicateTargetIds);
            }
        }
        Ok(Self { values })
    }
}

/// Presence and current enabled state, independent of default/used metadata.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrivilegeState {
    /// The complete successful capture contains no matching entry.
    Absent,
    /// A matching entry exists without `SE_PRIVILEGE_ENABLED`.
    PresentDisabled,
    /// A matching entry has `SE_PRIVILEGE_ENABLED`.
    PresentEnabled,
}

/// Bounded reason why raw capture cannot establish a complete known snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrivilegeCaptureError {
    /// Capture count or byte length exceeds the admitted finite limits.
    Bounds,
    /// Count and complete byte length disagree, or a header is truncated.
    Length,
    /// A capture contains duplicate LUID entries.
    DuplicatePrivilege,
    /// Fixed name lookups returned colliding identities.
    DuplicateTargetIds,
    /// An entry has an attribute outside the documented accepted set.
    UnsupportedAttributes,
    /// An entry claims removal rather than a supported observed privilege state.
    RemovedEntry,
}

impl fmt::Display for PrivilegeCaptureError {
    /// Project a fixed bounded rejection category without raw capture bytes.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Bounds => "privilege capture exceeds its fixed bounds",
            Self::Length => "privilege capture has an inconsistent length",
            Self::DuplicatePrivilege => "privilege capture has duplicate identities",
            Self::DuplicateTargetIds => "fixed privilege lookups returned duplicate identities",
            Self::UnsupportedAttributes => "privilege capture has unsupported attributes",
            Self::RemovedEntry => "privilege capture contains a removed entry",
        })
    }
}
impl Error for PrivilegeCaptureError {}

/// Mandatory shared validation for the only raw capture construction path.
#[derive(Validate)]
struct CaptureBounds {
    /// Count is independently bounded before scanning any entries.
    #[garde(range(max = 128))]
    count: u32,
    /// Complete successful returned byte count, including the count header.
    #[garde(range(min = 4, max = 1540))]
    bytes: usize,
}

/// One private present privilege, retaining its complete accepted attribute bits.
#[derive(Clone, Copy, Eq, PartialEq)]
struct PresentPrivilege {
    /// Full observed identity; unknown identities must remain present.
    luid: PrivilegeLuid,
    /// Exact accepted native bits, never reduced to enabled state alone.
    attributes: u32,
}

/// Complete bounded representation facts; native provenance remains caller-owned.
#[derive(Eq, PartialEq)]
pub struct PrivilegeSnapshot {
    /// Named presence projections retained beside the complete original entries.
    states: [PrivilegeState; 4],
    /// Locally resolved name identities, not portable numeric constants.
    ids: ServicePrivilegeIds,
    /// Initialized fixed storage; only the checked prefix denotes present entries.
    entries: [PresentPrivilege; MAX_CAPTURE_PRIVILEGES],
    /// Checked occupied prefix, always at most `MAX_CAPTURE_PRIVILEGES`.
    count: usize,
}

impl fmt::Debug for PrivilegeSnapshot {
    /// Expose bounded named facts and count, never raw unknown identities or bytes.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PrivilegeSnapshot")
            .field("states", &self.states)
            .field("count", &self.count)
            .finish_non_exhaustive()
    }
}

impl PrivilegeSnapshot {
    /// Decode exactly one complete successful little-endian Windows x64 capture.
    /// Every entry, including unrelated privileges, must satisfy the same rules.
    ///
    /// # Errors
    /// Rejects excessive/truncated/trailing data, duplicate identities, removed
    /// entries and unsupported attributes. Failure constructs no snapshot.
    pub fn decode(bytes: &[u8], ids: ServicePrivilegeIds) -> Result<Self, PrivilegeCaptureError> {
        let header = bytes.get(..4).ok_or(PrivilegeCaptureError::Length)?;
        let count = u32::from_le_bytes([header[0], header[1], header[2], header[3]]);
        CaptureBounds {
            count,
            bytes: bytes.len(),
        }
        .validate()
        .map_err(|_| PrivilegeCaptureError::Bounds)?;
        let raw_entries = bytes.get(4..).ok_or(PrivilegeCaptureError::Length)?;
        let count = usize::try_from(count).map_err(|_| PrivilegeCaptureError::Bounds)?;
        if count.checked_mul(12) != Some(raw_entries.len()) {
            return Err(PrivilegeCaptureError::Length);
        }
        let mut states = [PrivilegeState::Absent; 4];
        let mut entries = [PresentPrivilege {
            luid: PrivilegeLuid::new(0, 0),
            attributes: 0,
        }; MAX_CAPTURE_PRIVILEGES];
        for (index, entry) in raw_entries.as_chunks::<12>().0.iter().enumerate() {
            let luid = PrivilegeLuid::new(
                u32::from_le_bytes([entry[0], entry[1], entry[2], entry[3]]),
                i32::from_le_bytes([entry[4], entry[5], entry[6], entry[7]]),
            );
            if entries[..index].iter().any(|present| present.luid == luid) {
                return Err(PrivilegeCaptureError::DuplicatePrivilege);
            }
            let attributes = u32::from_le_bytes([entry[8], entry[9], entry[10], entry[11]]);
            if attributes & 4 != 0 {
                return Err(PrivilegeCaptureError::RemovedEntry);
            }
            if attributes & !0x8000_0003 != 0 {
                return Err(PrivilegeCaptureError::UnsupportedAttributes);
            }
            entries[index] = PresentPrivilege { luid, attributes };
            for (slot, target) in ids.values.iter().enumerate() {
                if *target == luid {
                    states[slot] = if attributes & 2 == 0 {
                        PrivilegeState::PresentDisabled
                    } else {
                        PrivilegeState::PresentEnabled
                    };
                }
            }
        }
        Ok(Self {
            states,
            ids,
            entries,
            count,
        })
    }

    /// Read one fixed privilege fact without implying current effect authority.
    #[must_use]
    pub const fn state(&self, privilege: ServicePrivilege) -> PrivilegeState {
        self.states[privilege.index()]
    }

    /// Read exact accepted bits of a named privilege without exposing unknown IDs.
    #[must_use]
    pub fn attributes(&self, privilege: ServicePrivilege) -> Option<u32> {
        let luid = self.ids.values[privilege.index()];
        self.entries[..self.count]
            .iter()
            .find_map(|entry| (entry.luid == luid).then_some(entry.attributes))
    }

    /// Require all three engine-forbidden privileges to be absent, even disabled.
    ///
    /// # Errors
    /// Returns the first present privilege in the fixed documented lookup order.
    pub fn require_engine_absence(&self) -> Result<(), EnginePrivilegeError> {
        for privilege in [
            ServicePrivilege::SystemTime,
            ServicePrivilege::Debug,
            ServicePrivilege::Impersonate,
        ] {
            let state = self.state(privilege);
            if state != PrivilegeState::Absent {
                return Err(EnginePrivilegeError { privilege, state });
            }
        }
        Ok(())
    }

    /// Compare the complete present set with the fixed role's startup profile.
    /// This establishes representation compatibility only, never live authority.
    ///
    /// # Errors
    /// Reject every missing or extra privilege, including unknown/disabled extras,
    /// and an enabled broker clock privilege. `ChangeNotify` may be enabled or disabled.
    pub fn require_role(&self, role: ServiceRole) -> Result<(), ServicePrivilegeError> {
        let forbidden: &[ServicePrivilege] = match role {
            ServiceRole::Engine => &[
                ServicePrivilege::SystemTime,
                ServicePrivilege::Debug,
                ServicePrivilege::Impersonate,
            ],
            ServiceRole::ClockBroker => &[ServicePrivilege::Debug, ServicePrivilege::Impersonate],
        };
        for &privilege in forbidden {
            let state = self.state(privilege);
            if state != PrivilegeState::Absent {
                return Err(ServicePrivilegeError::Forbidden(EnginePrivilegeError {
                    privilege,
                    state,
                }));
            }
        }
        if self.state(ServicePrivilege::ChangeNotify) == PrivilegeState::Absent {
            return Err(ServicePrivilegeError::MissingRequired(
                ServicePrivilege::ChangeNotify,
            ));
        }
        if role == ServiceRole::ClockBroker {
            match self.state(ServicePrivilege::SystemTime) {
                PrivilegeState::Absent => {
                    return Err(ServicePrivilegeError::MissingRequired(
                        ServicePrivilege::SystemTime,
                    ));
                }
                PrivilegeState::PresentEnabled => {
                    return Err(ServicePrivilegeError::BrokerClockEnabled);
                }
                PrivilegeState::PresentDisabled => {}
            }
        }
        let change_notify = self.ids.values[ServicePrivilege::ChangeNotify.index()];
        let system_time = self.ids.values[ServicePrivilege::SystemTime.index()];
        for entry in &self.entries[..self.count] {
            if entry.luid != change_notify
                && !(role == ServiceRole::ClockBroker && entry.luid == system_time)
            {
                return Err(ServicePrivilegeError::UnexpectedPrivilege);
            }
        }
        Ok(())
    }
}

/// Bounded role-profile refusal, containing no raw unknown LUID or captured data.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServicePrivilegeError {
    /// A formerly observed fixed forbidden privilege retains its typed facts.
    Forbidden(EnginePrivilegeError),
    /// One explicitly required named privilege is absent.
    MissingRequired(ServicePrivilege),
    /// A present entry lies outside the exact fixed-role set.
    UnexpectedPrivilege,
    /// Ordinary broker startup observed its clock privilege enabled.
    BrokerClockEnabled,
}

impl fmt::Display for ServicePrivilegeError {
    /// Format only reviewed categories and fixed named facts.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Forbidden(error) => fmt::Display::fmt(error, formatter),
            Self::MissingRequired(privilege) => {
                write!(
                    formatter,
                    "required service privilege {privilege:?} is absent"
                )
            }
            Self::UnexpectedPrivilege => {
                formatter.write_str("unexpected service privilege is present")
            }
            Self::BrokerClockEnabled => formatter.write_str("broker clock privilege is enabled"),
        }
    }
}

impl Error for ServicePrivilegeError {
    /// Preserve the original typed known-privilege facts without raw capture data.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Forbidden(error) => Some(error),
            _ => None,
        }
    }
}

/// Fixed engine admission refusal, preserving absence versus disabled/enabled.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EnginePrivilegeError {
    /// First forbidden privilege present in the capture.
    privilege: ServicePrivilege,
    /// Actual present state; absent can never construct this error.
    state: PrivilegeState,
}
impl EnginePrivilegeError {
    /// Read the fixed privilege that refused engine admission.
    #[must_use]
    pub const fn privilege(self) -> ServicePrivilege {
        self.privilege
    }
    /// Read its observed present state.
    #[must_use]
    pub const fn state(self) -> PrivilegeState {
        self.state
    }
}
impl fmt::Display for EnginePrivilegeError {
    /// Format only fixed privilege and presence enums, with no raw token identity.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "forbidden service privilege {:?} is {:?}",
            self.privilege, self.state
        )
    }
}
impl Error for EnginePrivilegeError {}

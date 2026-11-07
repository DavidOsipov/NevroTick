// SPDX-License-Identifier: GPL-2.0-only
//! Bounded, inert decoding of one successful self-token privilege capture.
//!
//! This parser establishes representation facts only. The native caller must
//! establish successful API completion, primary-token provenance and cleanup.
//! A snapshot is an observation, never a persistent authority grant.
//!
//! Snapshot fields cannot bypass checked decoding:
//! ```compile_fail,E0451
//! use windows_platform::privilege_observation::{PrivilegeSnapshot, PrivilegeState};
//! let snapshot = PrivilegeSnapshot { states: [PrivilegeState::Absent; 3] };
//! ```
//! Fixed lookup identities cannot bypass duplicate checks:
//! ```compile_fail,E0451
//! use windows_platform::privilege_observation::{PrivilegeLuid, ServicePrivilegeIds};
//! let ids = ServicePrivilegeIds { values: [PrivilegeLuid::new(0, 0); 3] };
//! ```

use garde::Validate;
use std::{error::Error, fmt};

/// Maximum entries accepted from one complete token capture.
pub const MAX_CAPTURE_PRIVILEGES: usize = 128;
/// Exact upper bound: a four-byte count followed by 128 twelve-byte entries.
pub const MAX_CAPTURE_BYTES: usize = 1540;

/// The three fixed privileges examined by the laboratory observer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServicePrivilege {
    /// Permission to change system time.
    SystemTime,
    /// Permission to debug other processes.
    Debug,
    /// Permission to impersonate a client.
    Impersonate,
}

impl ServicePrivilege {
    /// Return the stable slot shared by the fixed native lookup and decoder.
    const fn index(self) -> usize {
        match self {
            Self::SystemTime => 0,
            Self::Debug => 1,
            Self::Impersonate => 2,
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

/// Three successfully resolved, pairwise distinct fixed privilege identities.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ServicePrivilegeIds {
    /// Identities in `SystemTime`, `Debug`, `Impersonate` order.
    values: [PrivilegeLuid; 3],
}

impl ServicePrivilegeIds {
    /// Bind all three successful lookups; duplicate identities are indeterminate.
    ///
    /// # Errors
    /// Returns `DuplicateTargetIds` if any pair denotes the same LUID.
    pub fn new(values: [PrivilegeLuid; 3]) -> Result<Self, PrivilegeCaptureError> {
        if values[0] == values[1] || values[0] == values[2] || values[1] == values[2] {
            return Err(PrivilegeCaptureError::DuplicateTargetIds);
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

/// Complete structurally checked facts for the three fixed privileges.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrivilegeSnapshot {
    /// Complete checked representation only; native success remains caller-owned.
    states: [PrivilegeState; 3],
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
        let entries = bytes.get(4..).ok_or(PrivilegeCaptureError::Length)?;
        let count = usize::try_from(count).map_err(|_| PrivilegeCaptureError::Bounds)?;
        if count.checked_mul(12) != Some(entries.len()) {
            return Err(PrivilegeCaptureError::Length);
        }
        let mut states = [PrivilegeState::Absent; 3];
        let mut seen = [PrivilegeLuid::new(0, 0); MAX_CAPTURE_PRIVILEGES];
        for (index, entry) in entries.as_chunks::<12>().0.iter().enumerate() {
            let luid = PrivilegeLuid::new(
                u32::from_le_bytes([entry[0], entry[1], entry[2], entry[3]]),
                i32::from_le_bytes([entry[4], entry[5], entry[6], entry[7]]),
            );
            if seen[..index].contains(&luid) {
                return Err(PrivilegeCaptureError::DuplicatePrivilege);
            }
            seen[index] = luid;
            let attributes = u32::from_le_bytes([entry[8], entry[9], entry[10], entry[11]]);
            if attributes & 4 != 0 {
                return Err(PrivilegeCaptureError::RemovedEntry);
            }
            if attributes & !0x8000_0003 != 0 {
                return Err(PrivilegeCaptureError::UnsupportedAttributes);
            }
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
        Ok(Self { states })
    }

    /// Read one fixed privilege fact without implying current effect authority.
    #[must_use]
    pub const fn state(self, privilege: ServicePrivilege) -> PrivilegeState {
        self.states[privilege.index()]
    }

    /// Require all three engine-forbidden privileges to be absent, even disabled.
    ///
    /// # Errors
    /// Returns the first present privilege in the fixed documented lookup order.
    pub fn require_engine_absence(self) -> Result<(), EnginePrivilegeError> {
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
            "engine forbidden privilege {:?} is {:?}",
            self.privilege, self.state
        )
    }
}
impl Error for EnginePrivilegeError {}

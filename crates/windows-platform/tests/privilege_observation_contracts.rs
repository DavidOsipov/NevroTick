// SPDX-License-Identifier: GPL-2.0-only
//! Independent raw-byte contracts for the laboratory privilege decoder.
//!
//! These tests supply representation facts. A successful empty capture is not
//! evidence that a native API succeeded, that cleanup completed or that a
//! process currently has these privileges. Native callers own those checks.

use std::{error::Error, num::TryFromIntError};

use windows_platform::privilege_observation::{
    PrivilegeCaptureError, PrivilegeLuid, PrivilegeSnapshot, PrivilegeState, ServicePrivilege,
    ServicePrivilegeIds,
};

/// Use four independent fixed IDs in `SystemTime`, Debug, Impersonate, Notify order.
///
/// # Errors
/// Propagate unexpected rejection of this pairwise-distinct fixture mapping.
fn fixed_ids() -> Result<ServicePrivilegeIds, PrivilegeCaptureError> {
    ServicePrivilegeIds::new([
        PrivilegeLuid::new(17, -1),
        PrivilegeLuid::new(29, 3),
        PrivilegeLuid::new(41, 0),
        PrivilegeLuid::new(59, -7),
    ])
}

/// Write native fields directly, without calling any production encoder/parser.
/// The explicit cases below supply at most 129 entries.
///
/// # Errors
/// Reject an entry count that cannot be represented by the raw u32 header.
fn raw_capture(entries: &[(u32, i32, u32)]) -> Result<Vec<u8>, TryFromIntError> {
    let count = u32::try_from(entries.len())?;
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&count.to_le_bytes());
    for (low, high, attributes) in entries {
        bytes.extend_from_slice(&low.to_le_bytes());
        bytes.extend_from_slice(&high.to_le_bytes());
        bytes.extend_from_slice(&attributes.to_le_bytes());
    }
    Ok(bytes)
}

/// Supply distinct unrelated IDs; all callers use counts in the 0..=129 range.
fn unrelated_entries(count: u32) -> Vec<(u32, i32, u32)> {
    (0..count).map(|number| (1000 + number, 7, 0)).collect()
}

/// Observe all three states and the receiving engine check on a decoded value.
///
/// # Panics
/// Fail if a complete unrelated capture invents presence or refuses absence.
fn assert_all_absent(snapshot: &PrivilegeSnapshot) {
    for privilege in [
        ServicePrivilege::SystemTime,
        ServicePrivilege::Debug,
        ServicePrivilege::Impersonate,
    ] {
        assert_eq!(snapshot.state(privilege), PrivilegeState::Absent);
    }
    assert!(snapshot.require_engine_absence().is_ok());
}

/// Each isolated collision must be rejected when all other identities differ.
///
/// # Panics
/// Fail if any one of the six pairwise duplicate guards can be bypassed.
///
/// # Errors
/// Propagate unexpected failure of the distinct mapping and empty neighbor.
#[test]
fn each_isolated_lookup_collision_is_rejected() -> Result<(), Box<dyn Error>> {
    let a = PrivilegeLuid::new(7, 0);
    let b = PrivilegeLuid::new(8, -1);
    let c = PrivilegeLuid::new(9, i32::MIN);
    let d = PrivilegeLuid::new(10, i32::MAX);
    for values in [
        [a, a, c, d],
        [a, b, a, d],
        [a, b, c, a],
        [a, b, b, d],
        [a, b, c, b],
        [a, b, c, c],
    ] {
        assert_eq!(
            ServicePrivilegeIds::new(values),
            Err(PrivilegeCaptureError::DuplicateTargetIds)
        );
    }
    let ids = ServicePrivilegeIds::new([a, b, c, d])?;
    assert_all_absent(&PrivilegeSnapshot::decode(&[0, 0, 0, 0], ids)?);
    Ok(())
}

/// Equal low halves do not alias distinct signed high halves.
///
/// # Panics
/// Fail if the raw high bits are narrowed, ignored or assigned to another slot.
///
/// # Errors
/// Propagate unexpected failure of valid high-bit neighbors.
#[test]
fn equal_low_halves_preserve_all_signed_high_bits() -> Result<(), Box<dyn Error>> {
    let ids = ServicePrivilegeIds::new([
        PrivilegeLuid::new(10, i32::MIN),
        PrivilegeLuid::new(10, -1),
        PrivilegeLuid::new(10, i32::MAX),
        PrivilegeLuid::new(10, 0),
    ])?;
    let bytes = raw_capture(&[(10, i32::MAX, 2), (10, 0, 1), (10, -1, 0)])?;
    let snapshot = PrivilegeSnapshot::decode(&bytes, ids)?;
    assert_eq!(
        snapshot.state(ServicePrivilege::SystemTime),
        PrivilegeState::Absent
    );
    assert_eq!(
        snapshot.state(ServicePrivilege::Debug),
        PrivilegeState::PresentDisabled
    );
    assert_eq!(
        snapshot.state(ServicePrivilege::Impersonate),
        PrivilegeState::PresentEnabled
    );
    assert_eq!(
        snapshot.state(ServicePrivilege::ChangeNotify),
        PrivilegeState::PresentDisabled
    );
    assert_eq!(snapshot.attributes(ServicePrivilege::ChangeNotify), Some(1));
    let refusal = snapshot.require_engine_absence();
    assert!(refusal.is_err());
    if let Err(error) = refusal {
        assert_eq!(error.privilege(), ServicePrivilege::Debug);
        assert_eq!(error.state(), PrivilegeState::PresentDisabled);
    }
    Ok(())
}

/// A one-entry capture is exactly sixteen bytes, including its header.
///
/// # Panics
/// Fail if truncation or an extra byte becomes a complete absent snapshot.
///
/// # Errors
/// Propagate unexpected rejection of the exact valid neighboring capture.
#[test]
fn one_entry_requires_exactly_sixteen_bytes() -> Result<(), Box<dyn Error>> {
    let complete = raw_capture(&[(17, -1, 0)])?;
    assert_eq!(complete.len(), 16);
    for length in [15, 17] {
        let mut bytes = complete.clone();
        bytes.resize(length, 0);
        assert_eq!(
            PrivilegeSnapshot::decode(&bytes, fixed_ids()?),
            Err(PrivilegeCaptureError::Length)
        );
    }
    let snapshot = PrivilegeSnapshot::decode(&complete, fixed_ids()?)?;
    assert_eq!(
        snapshot.state(ServicePrivilege::SystemTime),
        PrivilegeState::PresentDisabled
    );
    assert!(snapshot.require_engine_absence().is_err());
    Ok(())
}

/// The laboratory count limit is independently fixed at 128, not a Windows max.
///
/// # Panics
/// Fail if 127/128 complete entries are refused, 129 is admitted, or a header
/// with a missing payload is normalized into a zero-count successful capture.
///
/// # Errors
/// Propagate unexpected failure of complete valid neighboring captures.
#[test]
fn count_neighbors_and_header_only_counts_remain_distinct() -> Result<(), Box<dyn Error>> {
    for count in [127, 128] {
        let mut entries = unrelated_entries(count);
        let bytes = raw_capture(&entries)?;
        let snapshot = PrivilegeSnapshot::decode(&bytes, fixed_ids()?)?;
        assert_all_absent(&snapshot);
        entries
            .last_mut()
            .ok_or("missing independently selected tail")?
            .2 = 0x8000_0001;
        let metadata_neighbor = PrivilegeSnapshot::decode(&raw_capture(&entries)?, fixed_ids()?)?;
        assert_ne!(snapshot, metadata_neighbor);
        entries
            .last_mut()
            .ok_or("missing independently selected tail")?
            .1 = -7;
        let identity_neighbor = PrivilegeSnapshot::decode(&raw_capture(&entries)?, fixed_ids()?)?;
        assert_ne!(metadata_neighbor, identity_neighbor);
    }
    let excessive = raw_capture(&unrelated_entries(129))?;
    assert_eq!(
        PrivilegeSnapshot::decode(&excessive, fixed_ids()?),
        Err(PrivilegeCaptureError::Bounds)
    );
    for count in [1_u32, 127, 128] {
        assert_eq!(
            PrivilegeSnapshot::decode(&count.to_le_bytes(), fixed_ids()?),
            Err(PrivilegeCaptureError::Length)
        );
    }
    for count in [129_u32, u32::MAX] {
        assert_eq!(
            PrivilegeSnapshot::decode(&count.to_le_bytes(), fixed_ids()?),
            Err(PrivilegeCaptureError::Bounds)
        );
    }
    Ok(())
}

/// The last admitted entry is still checked and cannot hide a disabled target.
///
/// # Panics
/// Fail if a decoder stops before entry 127/128, loses target identity or treats
/// present-but-disabled as absent. Truncated/trailing maximum captures must fail.
///
/// # Errors
/// Propagate unexpected failure of valid exact-limit captures.
#[test]
fn final_slot_targets_and_maximum_byte_neighbors_are_checked() -> Result<(), Box<dyn Error>> {
    let targets = [
        (17, -1, ServicePrivilege::SystemTime),
        (29, 3, ServicePrivilege::Debug),
        (41, 0, ServicePrivilege::Impersonate),
    ];
    for count in [127, 128] {
        for (low, high, privilege) in targets {
            let mut entries = unrelated_entries(count - 1);
            entries.push((low, high, 1));
            let bytes = raw_capture(&entries)?;
            let snapshot = PrivilegeSnapshot::decode(&bytes, fixed_ids()?)?;
            assert_eq!(snapshot.state(privilege), PrivilegeState::PresentDisabled);
            let refusal = snapshot.require_engine_absence();
            assert!(refusal.is_err());
            if let Err(error) = refusal {
                assert_eq!(error.privilege(), privilege);
                assert_eq!(error.state(), PrivilegeState::PresentDisabled);
            }
        }
    }
    let mut entries = unrelated_entries(127);
    entries.push((41, 0, 1));
    let complete = raw_capture(&entries)?;
    assert_eq!(complete.len(), 1540);
    for (length, error) in [
        (1539, PrivilegeCaptureError::Length),
        (1541, PrivilegeCaptureError::Bounds),
    ] {
        let mut bytes = complete.clone();
        bytes.resize(length, 0);
        assert_eq!(PrivilegeSnapshot::decode(&bytes, fixed_ids()?), Err(error));
    }
    assert_eq!(
        PrivilegeSnapshot::decode(&complete, fixed_ids()?)?.state(ServicePrivilege::Impersonate),
        PrivilegeState::PresentDisabled
    );
    Ok(())
}

/// Unknown or removed attributes on unrelated IDs invalidate the whole capture.
///
/// # Panics
/// Fail if filtering to three targets discards malformed original evidence.
///
/// # Errors
/// Propagate unexpected rejection of a known-metadata, unrelated neighbor.
#[test]
fn unrelated_attributes_cannot_be_filtered_into_absence() -> Result<(), Box<dyn Error>> {
    let good = raw_capture(&[(500, 5, 0x8000_0001)])?;
    assert_all_absent(&PrivilegeSnapshot::decode(&good, fixed_ids()?)?);
    for attributes in [8, 16, 0x4000_0000] {
        let bytes = raw_capture(&[(500, 5, attributes)])?;
        assert_eq!(
            PrivilegeSnapshot::decode(&bytes, fixed_ids()?),
            Err(PrivilegeCaptureError::UnsupportedAttributes)
        );
    }
    for attributes in [4, 5, 6, 7, 0x8000_0004] {
        let bytes = raw_capture(&[(500, 5, attributes)])?;
        assert_eq!(
            PrivilegeSnapshot::decode(&bytes, fixed_ids()?),
            Err(PrivilegeCaptureError::RemovedEntry)
        );
    }
    Ok(())
}

/// Mixed target ordering does not change slots or deterministic refusal order.
///
/// # Panics
/// Fail if raw record position replaces target identity or default metadata is
/// confused with current enablement.
///
/// # Errors
/// Propagate unexpected failure of the complete mixed-state capture.
#[test]
fn mixed_record_order_preserves_presence_and_current_enablement() -> Result<(), Box<dyn Error>> {
    let bytes = raw_capture(&[(29, 3, 2), (41, 0, 0), (17, -1, 0x8000_0001)])?;
    let snapshot = PrivilegeSnapshot::decode(&bytes, fixed_ids()?)?;
    assert_eq!(
        snapshot.state(ServicePrivilege::SystemTime),
        PrivilegeState::PresentDisabled
    );
    assert_eq!(
        snapshot.state(ServicePrivilege::Debug),
        PrivilegeState::PresentEnabled
    );
    assert_eq!(
        snapshot.state(ServicePrivilege::Impersonate),
        PrivilegeState::PresentDisabled
    );
    let refusal = snapshot.require_engine_absence();
    assert!(refusal.is_err());
    if let Err(error) = refusal {
        assert_eq!(error.privilege(), ServicePrivilege::SystemTime);
        assert_eq!(error.state(), PrivilegeState::PresentDisabled);
    }
    Ok(())
}

/// Duplicate original records cannot be normalized before admission.
///
/// # Panics
/// Fail if duplicate targets or unrelated IDs become one accepted record.
///
/// # Errors
/// Propagate an invalid fixture mapping or raw header conversion.
#[test]
fn duplicate_target_and_unrelated_records_both_reject() -> Result<(), Box<dyn Error>> {
    for entries in [
        [(17, -1, 0), (17, -1, 2)],
        [(500, 5, 0), (500, 5, 0x8000_0001)],
    ] {
        assert_eq!(
            PrivilegeSnapshot::decode(&raw_capture(&entries)?, fixed_ids()?),
            Err(PrivilegeCaptureError::DuplicatePrivilege)
        );
    }
    Ok(())
}

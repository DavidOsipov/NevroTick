// SPDX-License-Identifier: GPL-2.0-only
//! Public decoder and engine-admission facts independent of native token effects.

use windows_platform::privilege_observation::{
    PrivilegeCaptureError, PrivilegeLuid, PrivilegeSnapshot, PrivilegeState, ServicePrivilege,
    ServicePrivilegeIds,
};

/// Construct three fixed, distinct fixture identities.
///
/// # Errors
/// Propagate an unexpected checked-identity failure.
fn ids() -> Result<ServicePrivilegeIds, PrivilegeCaptureError> {
    ServicePrivilegeIds::new([
        PrivilegeLuid::new(17, 0),
        PrivilegeLuid::new(23, 0),
        PrivilegeLuid::new(41, 0),
    ])
}

/// Encode complete independent count and LUID/attribute bytes.
///
/// # Errors
/// Reject a fixture count that cannot be represented in the native header.
fn capture(entries: &[(u32, i32, u32)]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut bytes = u32::try_from(entries.len())?.to_le_bytes().to_vec();
    for (low, high, attributes) in entries {
        bytes.extend(low.to_le_bytes());
        bytes.extend(high.to_le_bytes());
        bytes.extend(attributes.to_le_bytes());
    }
    Ok(bytes)
}

/// Complete successful zero and unrelated captures prove only fixed target absence.
///
/// # Errors
/// Propagate fixture construction or checked decoding failure.
/// # Panics
/// Fails if unrelated privileges are mistaken for a target or absence is refused.
#[test]
fn complete_zero_and_unrelated_captures_allow_the_engine() -> Result<(), Box<dyn std::error::Error>>
{
    for entries in [vec![], vec![(999, -1, 0x8000_0003)]] {
        let snapshot = PrivilegeSnapshot::decode(&capture(&entries)?, ids()?)?;
        for target in [
            ServicePrivilege::SystemTime,
            ServicePrivilege::Debug,
            ServicePrivilege::Impersonate,
        ] {
            assert_eq!(snapshot.state(target), PrivilegeState::Absent);
        }
        snapshot.require_engine_absence()?;
    }
    Ok(())
}

/// Default/used metadata does not imply enabled, and disabled still forbids engine.
///
/// # Errors
/// Propagate fixture encoding and decoder errors.
/// # Panics
/// Fails if any fixed privilege is ignored, normalized away or misclassified.
#[test]
fn every_target_presence_and_known_attribute_combination_refuses_engine()
-> Result<(), Box<dyn std::error::Error>> {
    for (low, privilege) in [
        (17, ServicePrivilege::SystemTime),
        (23, ServicePrivilege::Debug),
        (41, ServicePrivilege::Impersonate),
    ] {
        for attributes in [
            0,
            1,
            2,
            3,
            0x8000_0000,
            0x8000_0001,
            0x8000_0002,
            0x8000_0003,
        ] {
            let snapshot = PrivilegeSnapshot::decode(&capture(&[(low, 0, attributes)])?, ids()?)?;
            let state = if attributes & 2 == 0 {
                PrivilegeState::PresentDisabled
            } else {
                PrivilegeState::PresentEnabled
            };
            assert_eq!(snapshot.state(privilege), state);
            let refusal = snapshot
                .require_engine_absence()
                .err()
                .ok_or("present privilege admitted")?;
            assert_eq!(refusal.privilege(), privilege);
            assert_eq!(refusal.state(), state);
        }
    }
    Ok(())
}

/// Whole captures reject malformed unrelated entries as strictly as target entries.
///
/// # Errors
/// Propagate fixture encoding and identity errors.
/// # Panics
/// Fails if duplicate, removed, unsupported or trailing raw bytes form a snapshot.
#[test]
fn raw_capture_failures_never_become_absence() -> Result<(), Box<dyn std::error::Error>> {
    for (entries, expected) in [
        (
            vec![(999, 0, 0), (999, 0, 2)],
            PrivilegeCaptureError::DuplicatePrivilege,
        ),
        (
            vec![(17, 0, 0), (17, 0, 0)],
            PrivilegeCaptureError::DuplicatePrivilege,
        ),
        (vec![(999, 0, 4)], PrivilegeCaptureError::RemovedEntry),
        (
            vec![(999, 0, 8)],
            PrivilegeCaptureError::UnsupportedAttributes,
        ),
    ] {
        assert_eq!(
            PrivilegeSnapshot::decode(&capture(&entries)?, ids()?),
            Err(expected)
        );
    }
    for bytes in [&[0; 3][..], &[0; 5][..], &[1, 0, 0, 0][..], &[255; 4][..]] {
        assert!(PrivilegeSnapshot::decode(bytes, ids()?).is_err());
    }
    Ok(())
}

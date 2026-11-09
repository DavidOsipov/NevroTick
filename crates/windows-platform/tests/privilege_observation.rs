// SPDX-License-Identifier: GPL-2.0-only
//! Public decoder and engine-admission facts independent of native token effects.

use windows_platform::privilege_observation::{
    PrivilegeCaptureError, PrivilegeLuid, PrivilegeSnapshot, PrivilegeState, ServicePrivilege,
    ServicePrivilegeError, ServicePrivilegeIds,
};
use windows_platform::service::ServiceRole;

/// Construct four fixed, distinct fixture identities.
///
/// # Errors
/// Propagate an unexpected checked-identity failure.
fn ids() -> Result<ServicePrivilegeIds, PrivilegeCaptureError> {
    ServicePrivilegeIds::new([
        PrivilegeLuid::new(17, 0),
        PrivilegeLuid::new(23, 0),
        PrivilegeLuid::new(41, 0),
        PrivilegeLuid::new(59, 0),
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
fn complete_zero_and_unrelated_captures_preserve_named_absence_only()
-> Result<(), Box<dyn std::error::Error>> {
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

/// The selected sets, not the production comparison, supply these expected facts.
/// Accepted metadata bits survive without imposing enabledness on `ChangeNotify`.
///
/// # Errors
/// Propagate independent encoding or checked decoding failure.
/// # Panics
/// Fails if an exact role set is refused or accepted bits disappear or change.
#[test]
fn exact_profiles_preserve_all_accepted_named_attributes() -> Result<(), Box<dyn std::error::Error>>
{
    for notify_attributes in [
        0,
        1,
        2,
        3,
        0x8000_0000,
        0x8000_0001,
        0x8000_0002,
        0x8000_0003,
    ] {
        let engine = PrivilegeSnapshot::decode(&capture(&[(59, 0, notify_attributes)])?, ids()?)?;
        assert_eq!(engine.require_role(ServiceRole::Engine), Ok(()));
        assert_eq!(
            engine.attributes(ServicePrivilege::ChangeNotify),
            Some(notify_attributes)
        );
        assert_eq!(engine.attributes(ServicePrivilege::SystemTime), None);
        for clock_attributes in [0, 1, 0x8000_0000, 0x8000_0001] {
            let broker = PrivilegeSnapshot::decode(
                &capture(&[(17, 0, clock_attributes), (59, 0, notify_attributes)])?,
                ids()?,
            )?;
            assert_eq!(broker.require_role(ServiceRole::ClockBroker), Ok(()));
            assert_eq!(
                broker.attributes(ServicePrivilege::SystemTime),
                Some(clock_attributes)
            );
            assert_eq!(
                broker.attributes(ServicePrivilege::ChangeNotify),
                Some(notify_attributes)
            );
        }
    }
    Ok(())
}

/// Empty/partial sets and extra full identities cannot become admitted roles.
///
/// # Errors
/// Propagate independently encoded fixture or checked decoding failure.
/// # Panics
/// Fails if absence, high-half identity, disabled extras or clock enablement vanish.
#[test]
fn exact_profiles_reject_missing_extras_and_enabled_clock() -> Result<(), Box<dyn std::error::Error>>
{
    for (role, entries, expected) in [
        (
            ServiceRole::Engine,
            vec![],
            ServicePrivilegeError::MissingRequired(ServicePrivilege::ChangeNotify),
        ),
        (
            ServiceRole::ClockBroker,
            vec![],
            ServicePrivilegeError::MissingRequired(ServicePrivilege::ChangeNotify),
        ),
        (
            ServiceRole::ClockBroker,
            vec![(17, 0, 0)],
            ServicePrivilegeError::MissingRequired(ServicePrivilege::ChangeNotify),
        ),
        (
            ServiceRole::ClockBroker,
            vec![(59, 0, 0)],
            ServicePrivilegeError::MissingRequired(ServicePrivilege::SystemTime),
        ),
    ] {
        let snapshot = PrivilegeSnapshot::decode(&capture(&entries)?, ids()?)?;
        assert_eq!(snapshot.require_role(role), Err(expected));
    }
    for role in [ServiceRole::Engine, ServiceRole::ClockBroker] {
        for extra in [(73, 0, 0), (73, 0, 2), (59, 1, 0)] {
            let mut entries = vec![(59, 0, 0)];
            if role == ServiceRole::ClockBroker {
                entries.push((17, 0, 0));
            }
            entries.push(extra);
            let snapshot = PrivilegeSnapshot::decode(&capture(&entries)?, ids()?)?;
            assert_eq!(
                snapshot.require_role(role),
                Err(ServicePrivilegeError::UnexpectedPrivilege)
            );
        }
    }
    for attributes in [2, 3, 0x8000_0002, 0x8000_0003] {
        let broker =
            PrivilegeSnapshot::decode(&capture(&[(59, 0, 0), (17, 0, attributes)])?, ids()?)?;
        assert_eq!(
            broker.require_role(ServiceRole::ClockBroker),
            Err(ServicePrivilegeError::BrokerClockEnabled)
        );
    }
    Ok(())
}

/// Unknown captured identity values never become Debug or Display payloads.
///
/// # Errors
/// Propagate independent fixture construction or decoding failure.
/// # Panics
/// Fails if snapshot/error formatting exposes raw captured identities or arrays.
#[test]
fn snapshot_and_role_refusals_keep_unknown_capture_data_private()
-> Result<(), Box<dyn std::error::Error>> {
    let snapshot = PrivilegeSnapshot::decode(
        &capture(&[(59, 0, 2), (0x5345_4352, -123_456_789, 0x8000_0001)])?,
        ids()?,
    )?;
    let error = snapshot
        .require_role(ServiceRole::Engine)
        .err()
        .ok_or("unknown extra admitted")?;
    assert_eq!(error, ServicePrivilegeError::UnexpectedPrivilege);
    assert_eq!(format!("{error:?}"), "UnexpectedPrivilege");
    assert_eq!(error.to_string(), "unexpected service privilege is present");
    let shown = format!("{snapshot:?}");
    assert_ne!(shown, "");
    assert!(shown.contains("PrivilegeSnapshot"));
    assert!(shown.contains("states: [Absent, Absent, Absent, PresentEnabled]"));
    assert!(shown.contains("count: 2"));
    let neighbor = PrivilegeSnapshot::decode(
        &capture(&[(59, 0, 2), (0x5345_4352, -123_456_789, 1)])?,
        ids()?,
    )?;
    assert_ne!(snapshot, neighbor);
    assert!(shown.len() <= 160);
    for raw in [
        "1397048146",
        "-123456789",
        "2147483649",
        "PresentPrivilege",
        "PrivilegeLuid",
    ] {
        assert!(!shown.contains(raw));
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
            let refusal = snapshot.require_engine_absence();
            assert!(refusal.is_err());
            let refusal = refusal.err().ok_or("present privilege admitted")?;
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

/// Real malformed captures and colliding lookups produce useful distinct failure
/// categories without including arbitrary captured identity values or byte arrays.
///
/// # Errors
/// Propagate independent fixture construction or an unexpectedly absent refusal.
/// # Panics
/// Fails if failure meaning disappears, output is unbounded or raw data escapes.
#[test]
fn produced_capture_refusals_have_bounded_useful_redacted_messages()
-> Result<(), Box<dyn std::error::Error>> {
    use std::{collections::BTreeSet, error::Error};

    let collision = ServicePrivilegeIds::new([
        PrivilegeLuid::new(17, 0),
        PrivilegeLuid::new(17, 0),
        PrivilegeLuid::new(41, 0),
        PrivilegeLuid::new(59, 0),
    ])
    .err();
    let mut messages = BTreeSet::new();
    for (produced, expected, words) in [
        (
            PrivilegeSnapshot::decode(&[129, 0, 0, 0], ids()?).err(),
            PrivilegeCaptureError::Bounds,
            ["capture", "bounds"],
        ),
        (
            PrivilegeSnapshot::decode(&[1, 0, 0, 0], ids()?).err(),
            PrivilegeCaptureError::Length,
            ["capture", "length"],
        ),
        (
            PrivilegeSnapshot::decode(
                &capture(&[
                    (0x5345_4352, -123_456_789, 0),
                    (0x5345_4352, -123_456_789, 2),
                ])?,
                ids()?,
            )
            .err(),
            PrivilegeCaptureError::DuplicatePrivilege,
            ["capture", "duplicate"],
        ),
        (
            collision,
            PrivilegeCaptureError::DuplicateTargetIds,
            ["lookups", "duplicate"],
        ),
        (
            PrivilegeSnapshot::decode(&capture(&[(0x5345_4352, -123_456_789, 8)])?, ids()?).err(),
            PrivilegeCaptureError::UnsupportedAttributes,
            ["attributes", "unsupported"],
        ),
        (
            PrivilegeSnapshot::decode(&capture(&[(0x5345_4352, -123_456_789, 4)])?, ids()?).err(),
            PrivilegeCaptureError::RemovedEntry,
            ["capture", "removed"],
        ),
    ] {
        let error = produced.ok_or("malformed capture or colliding lookup admitted")?;
        assert_eq!(error, expected);
        let message = error.to_string();
        assert_ne!(message, "");
        assert!(message.len() <= 64);
        assert!(
            message
                .bytes()
                .all(|byte| byte.is_ascii_alphabetic() || byte == b' ')
        );
        for word in words {
            assert!(message.contains(word));
        }
        assert!(error.source().is_none());
        assert!(messages.insert(message));
    }
    assert_eq!(messages.len(), 6);
    Ok(())
}

/// Forbidden profile sources preserve the actual named privilege and current state.
///
/// # Errors
/// Propagate checked raw fixtures, unexpected admission or a missing typed source.
/// # Panics
/// Fails if useful role/state meaning, bounded redaction or source custody changes.
#[test]
fn produced_forbidden_refusals_preserve_named_sources_and_bounded_messages()
-> Result<(), Box<dyn std::error::Error>> {
    use std::error::Error;
    use windows_platform::privilege_observation::EnginePrivilegeError;

    for (role, low, privilege, name) in [
        (
            ServiceRole::Engine,
            17,
            ServicePrivilege::SystemTime,
            "SystemTime",
        ),
        (ServiceRole::Engine, 23, ServicePrivilege::Debug, "Debug"),
        (
            ServiceRole::Engine,
            41,
            ServicePrivilege::Impersonate,
            "Impersonate",
        ),
        (
            ServiceRole::ClockBroker,
            23,
            ServicePrivilege::Debug,
            "Debug",
        ),
        (
            ServiceRole::ClockBroker,
            41,
            ServicePrivilege::Impersonate,
            "Impersonate",
        ),
    ] {
        for (attributes, state, state_name) in [
            (0, PrivilegeState::PresentDisabled, "PresentDisabled"),
            (2, PrivilegeState::PresentEnabled, "PresentEnabled"),
        ] {
            let mut entries = vec![(59, 0, 0), (low, 0, attributes)];
            if role == ServiceRole::ClockBroker {
                entries.push((17, 0, 0));
            }
            let snapshot = PrivilegeSnapshot::decode(&capture(&entries)?, ids()?)?;
            let error = snapshot
                .require_role(role)
                .err()
                .ok_or("forbidden profile admitted")?;
            let ServicePrivilegeError::Forbidden(facts) = error else {
                return Err("known forbidden privilege lost its original facts".into());
            };
            assert_eq!(facts.privilege(), privilege);
            assert_eq!(facts.state(), state);
            let source = error.source();
            assert!(source.is_some());
            let source = source.ok_or("known refusal lost its original source")?;
            assert_eq!(source.downcast_ref::<EnginePrivilegeError>(), Some(&facts));
            assert!(source.source().is_none());
            let message = error.to_string();
            assert!(message.len() <= 64);
            assert!(
                message
                    .bytes()
                    .all(|byte| byte.is_ascii_alphabetic() || byte == b' ')
            );
            for word in ["forbidden", name, state_name] {
                assert!(message.contains(word));
            }
        }
    }
    Ok(())
}

/// Missing, extra and enabled-clock refusals have useful bounded categories and
/// cannot fabricate a lower-level failure; exact neighboring profiles stay valid.
///
/// # Errors
/// Propagate checked fixtures or absence of an expected actual role refusal.
/// # Panics
/// Fails if meaning/redaction changes, a spurious cause appears or healthy sets fail.
#[test]
fn produced_profile_refusals_without_sources_keep_useful_bounded_messages()
-> Result<(), Box<dyn std::error::Error>> {
    use std::error::Error;

    for (role, entries, expected, words) in [
        (
            ServiceRole::Engine,
            vec![],
            ServicePrivilegeError::MissingRequired(ServicePrivilege::ChangeNotify),
            ["required", "ChangeNotify", "absent"],
        ),
        (
            ServiceRole::ClockBroker,
            vec![(59, 0, 0)],
            ServicePrivilegeError::MissingRequired(ServicePrivilege::SystemTime),
            ["required", "SystemTime", "absent"],
        ),
        (
            ServiceRole::Engine,
            vec![(59, 0, 0), (0x5345_4352, -123_456_789, 0x8000_0001)],
            ServicePrivilegeError::UnexpectedPrivilege,
            ["unexpected", "privilege", "present"],
        ),
        (
            ServiceRole::ClockBroker,
            vec![(59, 0, 0), (17, 0, 2)],
            ServicePrivilegeError::BrokerClockEnabled,
            ["broker", "clock", "enabled"],
        ),
    ] {
        let snapshot = PrivilegeSnapshot::decode(&capture(&entries)?, ids()?)?;
        let error = snapshot
            .require_role(role)
            .err()
            .ok_or("invalid profile admitted")?;
        assert_eq!(error, expected);
        assert!(error.source().is_none());
        let message = error.to_string();
        assert_ne!(message, "");
        assert!(message.len() <= 64);
        assert!(
            message
                .bytes()
                .all(|byte| byte.is_ascii_alphabetic() || byte == b' ')
        );
        for word in words {
            assert!(message.contains(word));
        }
    }
    for (role, entries) in [
        (ServiceRole::Engine, vec![(59, 0, 0)]),
        (ServiceRole::ClockBroker, vec![(59, 0, 0), (17, 0, 0)]),
    ] {
        assert_eq!(
            PrivilegeSnapshot::decode(&capture(&entries)?, ids()?)?.require_role(role),
            Ok(())
        );
    }
    Ok(())
}

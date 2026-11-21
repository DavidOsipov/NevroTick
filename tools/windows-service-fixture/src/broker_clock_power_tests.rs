// SPDX-License-Identifier: GPL-2.0-only
//! Exercise delivered SCM power histories on the broker mailbox.

use std::sync::atomic::Ordering::SeqCst;

use windows_platform::{scm::StopNotification, service::ServiceState};

use super::{BrokerClockChanges, ClockRevision, ControlDisposition, deliver, report_status};

/// Read the callback acknowledgement separately from its owner wake decision.
///
/// # Panics
/// Panics if the actual acknowledgement or wake differs from the expected pair.
fn assert_delivery(
    changes: Option<&BrokerClockChanges>,
    stop: &StopNotification,
    control: u32,
    event: u32,
    expected: (u32, bool),
) {
    let disposition: ControlDisposition = deliver(changes, stop, control, event);
    assert_eq!(
        (disposition.status, disposition.wake),
        expected,
        "SCM control {control:#x}, event {event:#x}"
    );
}

/// Read the published ordinal only when the broker can admit a capture.
fn capture_ordinal(changes: &BrokerClockChanges) -> Option<u64> {
    changes
        .revision()
        .map(|revision: ClockRevision<'_>| revision.number().get())
}

/// Only a live, selected broker handles the supported notification types.
///
/// # Panics
/// Fails if the role, status, subtype or failed acknowledgement admits a wrong effect.
#[test]
fn power_delivery_is_role_lifecycle_and_event_scoped() {
    let changes: BrokerClockChanges = BrokerClockChanges::new();
    let stop: StopNotification = StopNotification::new();
    for event in [4, 18] {
        assert_delivery(Some(&changes), &stop, 0x0d, event, (120, false));
        assert_delivery(None, &stop, 0x0d, event, (120, false));
    }

    let engine: Result<(), u32> = report_status(None, ServiceState::Running, |extra: u32| {
        assert_eq!(extra, 0);
        Ok(())
    });
    assert_eq!(engine, Ok(()));
    let pending: Result<(), u32> =
        report_status(Some(&changes), ServiceState::StartPending, |extra: u32| {
            assert_eq!(extra, 0);
            assert_delivery(Some(&changes), &stop, 0x0d, 18, (120, false));
            assert_eq!(capture_ordinal(&changes), None);
            Ok(())
        });
    assert_eq!(pending, Ok(()));
    let denied: Result<(), u32> =
        report_status(Some(&changes), ServiceState::Running, |extra: u32| {
            assert_eq!(extra, 0x240);
            assert_eq!(capture_ordinal(&changes), Some(1));
            assert_delivery(Some(&changes), &stop, 0x0d, 7, (120, false));
            assert_delivery(Some(&changes), &stop, 0x0d, u32::MAX, (120, false));
            assert_delivery(Some(&changes), &stop, 0x0e, 18, (120, false));
            assert_delivery(Some(&changes), &stop, 4, 18, (0, false));
            assert_eq!(capture_ordinal(&changes), Some(1));
            assert_delivery(Some(&changes), &stop, 0x0d, 4, (0, true));
            assert_eq!(capture_ordinal(&changes), None);
            Err(0)
        });
    assert_eq!(denied, Err(0));
    assert!(!changes.arm());
    assert!(!stop.stop_requested());
    assert_delivery(Some(&changes), &stop, 0x0d, 18, (120, false));
    assert_eq!(capture_ordinal(&changes), None);
}

/// Repeated power events renew awake capture without reusing an issued fence.
///
/// # Errors
/// Returns the missing-fence assertion if the initial awake state cannot issue one.
///
/// # Panics
/// Fails if a power or Stop decision misclassifies the original or renewed fence.
#[test]
fn suspend_blocks_capture_until_automatic_resume_begins_a_fresh_ordinal() -> Result<(), &'static str>
{
    let changes: BrokerClockChanges = BrokerClockChanges::new();
    let stop: StopNotification = StopNotification::new();
    assert!(changes.arm());
    let original: ClockRevision<'_> = changes.revision().ok_or("initial awake fence")?;
    assert_eq!(original.number().get(), 1);
    assert!(original.admits());

    // An automatic resume renews even if no suspend was delivered.
    assert_delivery(Some(&changes), &stop, 0x0d, 18, (0, true));
    assert_eq!(capture_ordinal(&changes), Some(2));
    assert!(!original.admits());
    let awake: ClockRevision<'_> = changes.revision().ok_or("renewed awake fence")?;
    assert!(awake.admits());

    assert_delivery(Some(&changes), &stop, 0x0d, 4, (0, true));
    assert_eq!(capture_ordinal(&changes), None);
    assert!(!awake.admits());
    let still_running: Result<(), u32> =
        report_status(Some(&changes), ServiceState::Running, |extra: u32| {
            assert_eq!(extra, 0x240);
            Ok(())
        });
    assert_eq!(still_running, Ok(()));
    assert_delivery(Some(&changes), &stop, 0x10, 18, (0, true));
    assert_eq!(capture_ordinal(&changes), None);
    assert_delivery(Some(&changes), &stop, 0x0d, 4, (0, true));
    assert_eq!(capture_ordinal(&changes), None);

    // The two suspends and the time change each counted while capture was blocked.
    assert_delivery(Some(&changes), &stop, 0x0d, 18, (0, true));
    assert_eq!(capture_ordinal(&changes), Some(6));
    assert!(!awake.admits());
    assert_delivery(Some(&changes), &stop, 0x0d, 4, (0, true));
    assert_eq!(capture_ordinal(&changes), None);
    assert_delivery(Some(&changes), &stop, 0x0d, 18, (0, true));
    assert_eq!(capture_ordinal(&changes), Some(8));

    let before_stop: ClockRevision<'_> = changes.revision().ok_or("pre-Stop awake fence")?;
    assert_delivery(Some(&changes), &stop, 1, 18, (0, true));
    assert!(stop.stop_requested());
    assert!(!before_stop.admits());
    assert!(!changes.arm());
    assert_delivery(Some(&changes), &stop, 0x0d, 18, (120, false));
    assert_eq!(capture_ordinal(&changes), None);
    Ok(())
}

/// The largest encoded suspended state is valid until further invalidation.
///
/// # Errors
/// Returns a missing-fence assertion if the last awake neighbor is refused.
///
/// # Panics
/// Fails if the largest suspended state or subsequent retirement can reopen.
#[test]
fn suspended_ordinal_exhaustion_is_irreversible() -> Result<(), &'static str> {
    let changes: BrokerClockChanges = BrokerClockChanges::new();
    let stop: StopNotification = StopNotification::new();
    let last_ordinal: u64 = u64::MAX / 2;
    changes.revision.store((last_ordinal - 1) << 1, SeqCst);
    let awake: ClockRevision<'_> = changes.revision().ok_or("last awake neighbor")?;
    assert_eq!(awake.number().get(), last_ordinal - 1);
    assert!(awake.admits());

    assert_delivery(Some(&changes), &stop, 0x0d, 4, (0, true));
    assert_eq!(changes.revision.load(SeqCst), u64::MAX);
    assert_eq!(capture_ordinal(&changes), None);
    assert!(!awake.admits());

    assert_delivery(Some(&changes), &stop, 0x10, 0, (0, true));
    assert!(!changes.arm());
    assert!(!stop.stop_requested());
    assert_delivery(Some(&changes), &stop, 0x0d, 18, (120, false));
    assert_delivery(Some(&changes), &stop, 0x0d, 4, (120, false));
    assert_delivery(Some(&changes), &stop, 0x10, 0, (120, false));
    assert_eq!(capture_ordinal(&changes), None);
    Ok(())
}

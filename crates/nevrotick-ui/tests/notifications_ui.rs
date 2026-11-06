//! Contract checks for inert notification requests and sample-only activation.
#![forbid(unsafe_code)]

use std::error::Error;

use nevrotick_ui::notifications_ui::{
    FixtureNotice, FixtureNoticeContext, FixtureNotificationPreview, NotificationBlock,
    ObserverPresence, UnavailableNotificationPort, WindowsNotificationSetting,
};
use nevrotick_ui::preferences::UiPreferences;
use nevrotick_ui::shell::{AdapterAvailability, Observation, SettingsSection, ShellRoute};

/// No accepted preference or observation makes the missing native adapter ready.
///
/// # Panics
/// Fails if a native request invents success or changes accepted preferences.
#[test]
fn native_test_request_never_claims_submission() {
    let preferences = UiPreferences::default();
    let before = preferences.clone();
    assert!(matches!(
        UnavailableNotificationPort::availability(),
        AdapterAvailability::Unavailable
    ));
    assert_eq!(
        UnavailableNotificationPort::request_test(
            &preferences,
            Observation::Current(WindowsNotificationSetting::Allowed),
            Observation::Current(ObserverPresence::Running),
        ),
        NotificationBlock::NativeAdapterUnavailable,
    );
    assert_eq!(preferences, before);
}

/// Accepted preferences and current observation guards are separate blockers.
///
/// # Panics
/// Fails if a disabled preference, unavailable observer or Windows block is ignored.
#[test]
fn test_request_checks_preference_observer_and_windows_separately() {
    let mut preferences = UiPreferences::default();
    let allowed = Observation::Current(WindowsNotificationSetting::Allowed);
    let running = Observation::Current(ObserverPresence::Running);

    preferences.set_notifications_enabled(false);
    assert_eq!(
        UnavailableNotificationPort::request_test(&preferences, allowed, running),
        NotificationBlock::PreferenceOff,
    );
    preferences.set_notifications_enabled(true);
    for observer in [
        Observation::Unknown,
        Observation::Current(ObserverPresence::Stopped),
        Observation::Stale(ObserverPresence::Running),
    ] {
        assert_eq!(
            UnavailableNotificationPort::request_test(&preferences, allowed, observer),
            NotificationBlock::ObserverUnavailable,
        );
    }
    for windows in [
        Observation::Unknown,
        Observation::Stale(WindowsNotificationSetting::Allowed),
        Observation::Current(WindowsNotificationSetting::Unsupported),
    ] {
        assert_eq!(
            UnavailableNotificationPort::request_test(&preferences, windows, running),
            NotificationBlock::WindowsUnavailable,
        );
    }
    for setting in [
        WindowsNotificationSetting::BlockedForUser,
        WindowsNotificationSetting::BlockedByPolicy,
    ] {
        assert_eq!(
            UnavailableNotificationPort::request_test(
                &preferences,
                Observation::Current(setting),
                running,
            ),
            NotificationBlock::WindowsBlocked,
        );
    }
}

/// Dismissal and unrelated local changes do not create another sample incident.
///
/// # Errors
/// Propagates unexpected rejection of a maintained nonzero fixture identity.
///
/// # Panics
/// Fails if identity or coalescing is lost through dismissal or preference changes.
#[test]
fn dismissing_a_preview_retains_identity_and_coalescing() -> Result<(), Box<dyn Error>> {
    let notice = FixtureNotice::new(7, FixtureNoticeContext::ServiceNeedsReview)
        .ok_or("nonzero fixture notice should be admitted")?;
    let mut preview = FixtureNotificationPreview::new(notice);
    let mut preferences = UiPreferences::default();
    assert!(!preview.visible());
    assert!(preview.show_example());
    assert!(preview.visible());
    assert!(!preview.show_example());
    preview.dismiss();
    assert!(!preview.visible());
    assert_eq!(preview.notice(), notice);
    assert!(!preview.show_example());

    preferences.set_notifications_enabled(false);
    preferences.set_notifications_enabled(true);
    // Service dialog cancellation has no notification-state operation to call.
    let before = preview.notice();
    let result = UnavailableNotificationPort::request_test(
        &preferences,
        Observation::Current(WindowsNotificationSetting::Allowed),
        Observation::Current(ObserverPresence::Running),
    );
    assert_eq!(result, NotificationBlock::NativeAdapterUnavailable);
    assert_eq!(preview.notice(), before);
    assert!(!preview.show_example());
    Ok(())
}

/// Activation admits a stored matching current context, otherwise opening Status.
///
/// # Errors
/// Propagates unexpected rejection of maintained nonzero fixture identities.
///
/// # Panics
/// Fails if arbitrary/stale identities can select a past context or issue an action.
#[test]
fn activation_is_current_typed_navigation_only() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        FixtureNotice::new(0, FixtureNoticeContext::StatusUnavailable),
        None
    );
    for (context, expected) in [
        (
            FixtureNoticeContext::StatusUnavailable,
            ShellRoute::Diagnostics,
        ),
        (
            FixtureNoticeContext::ServiceNeedsReview,
            ShellRoute::Settings(SettingsSection::BackgroundService),
        ),
        (
            FixtureNoticeContext::TestExample,
            ShellRoute::Settings(SettingsSection::Notifications),
        ),
    ] {
        let notice =
            FixtureNotice::new(7, context).ok_or("nonzero fixture notice should be admitted")?;
        let mut preview = FixtureNotificationPreview::new(notice);
        assert_eq!(notice.id(), 7);
        assert_eq!(notice.context(), context);
        assert_eq!(
            preview.activate(7, Observation::Current(notice)),
            ShellRoute::Status
        );
        assert!(preview.show_example());
        preview.dismiss();
        assert_eq!(preview.activate(7, Observation::Current(notice)), expected);
        for current in [Observation::Unknown, Observation::Stale(notice)] {
            assert_eq!(preview.activate(7, current), ShellRoute::Status);
        }
        for id in [0, 6, 8, u64::MAX] {
            assert_eq!(
                preview.activate(id, Observation::Current(notice)),
                ShellRoute::Status
            );
        }
        let successor =
            FixtureNotice::new(8, context).ok_or("nonzero successor should be admitted")?;
        assert_eq!(
            preview.activate(7, Observation::Current(successor)),
            ShellRoute::Status
        );
        let changed_context = match context {
            FixtureNoticeContext::StatusUnavailable => FixtureNoticeContext::ServiceNeedsReview,
            FixtureNoticeContext::ServiceNeedsReview | FixtureNoticeContext::TestExample => {
                FixtureNoticeContext::StatusUnavailable
            }
        };
        let changed = FixtureNotice::new(7, changed_context)
            .ok_or("same ID with another context should be constructible")?;
        assert_eq!(
            preview.activate(7, Observation::Current(changed)),
            ShellRoute::Status
        );
    }
    Ok(())
}

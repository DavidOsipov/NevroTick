//! Independent accepted-state and hostile-input checks for GUI preferences.
#![forbid(unsafe_code)]

use nevrotick_ui::preferences::{
    CheckInterval, CustomMinutes, MAX_CUSTOM_MINUTES_BYTES, PreferenceError, UiPreferences,
};

/// A fresh GUI starts with the v0.4 defaults and no OS/observer state requirement.
///
/// # Panics
/// Fails when a fresh preference differs from the independently specified default.
#[test]
fn defaults_are_automatic_one_custom_minute_notifications_on_and_tray_off() {
    let preferences = UiPreferences::default();
    assert_eq!(preferences.interval(), CheckInterval::Automatic);
    assert_eq!(preferences.last_custom_minutes().get(), 1);
    assert!(preferences.notifications_enabled());
    assert!(!preferences.tray_enabled());
    assert!(!preferences.sign_in_enabled());
    assert_eq!(CustomMinutes::default().get(), 1);
    assert_eq!(CheckInterval::default(), CheckInterval::Automatic);
}

/// Legal neighbors span the finite whole-minute domain and canonical output.
///
/// # Errors
/// Propagates unexpected rejection of a specified legal custom interval.
///
/// # Panics
/// Fails when an accepted representation loses its exact integer or UI identity.
#[test]
fn decimal_minutes_accept_neighbors_and_normalize_only_accepted_values()
-> Result<(), PreferenceError> {
    assert_eq!(MAX_CUSTOM_MINUTES_BYTES, 20);
    let mut preferences = UiPreferences::default();
    for (raw, expected, canonical) in [
        ("1", 1, "1"),
        ("00000000000000000001", 1, "1"),
        ("0002", 2, "2"),
        ("5", 5, "5"),
        ("15", 15, "15"),
        (
            "18446744073709551614",
            18_446_744_073_709_551_614,
            "18446744073709551614",
        ),
        ("18446744073709551615", u64::MAX, "18446744073709551615"),
    ] {
        let minutes = preferences.set_custom_minutes(raw)?;
        assert_eq!(minutes.get(), expected);
        assert_eq!(minutes.to_string(), canonical);
        assert_eq!(preferences.interval(), CheckInterval::Custom(minutes));
        assert_eq!(preferences.last_custom_minutes(), minutes);
    }
    preferences.set_interval(CheckInterval::FiveMinutes);
    assert_eq!(preferences.interval(), CheckInterval::FiveMinutes);
    let minutes = preferences.set_custom_minutes("5")?;
    assert_eq!(preferences.interval(), CheckInterval::Custom(minutes));
    assert_ne!(preferences.interval(), CheckInterval::FiveMinutes);
    Ok(())
}

/// Raw constructors and direct setters reject hostile values before publication.
///
/// # Errors
/// Propagates unexpected rejection of healthy initialization and neighbor input.
///
/// # Panics
/// Fails when invalid text enters accepted state or any field changes on rejection.
#[test]
fn hostile_direct_updates_preserve_every_accepted_preference() -> Result<(), PreferenceError> {
    let mut preferences = UiPreferences::default();
    let accepted = preferences.set_custom_minutes("37")?;
    preferences.set_notifications_enabled(false);
    preferences.set_tray_enabled(true);
    preferences.set_sign_in_enabled(true)?;
    let before = preferences.clone();
    let oversize = "7".repeat(4096);
    for raw in [
        "",
        "0",
        "00000000000000000000",
        "000000000000000000001",
        "18446744073709551616",
        "99999999999999999999",
        "+1",
        "-1",
        " 1",
        "1 ",
        "1\n",
        "1\t",
        "1\0",
        "1.0",
        "1,0",
        "1e1",
        "1E1",
        "NaN",
        "inf",
        "Infinity",
        "0x1",
        "1_000",
        "١",
        "１",
        "secret\nvalue",
        oversize.as_str(),
    ] {
        assert_eq!(
            CustomMinutes::parse(raw),
            Err(PreferenceError::InvalidCustomMinutes)
        );
        assert_eq!(
            preferences.set_custom_minutes(raw),
            Err(PreferenceError::InvalidCustomMinutes)
        );
        assert_eq!(preferences, before);
        assert_eq!(preferences.last_custom_minutes(), accepted);
    }
    let healthy = preferences.set_custom_minutes("38")?;
    assert_eq!(healthy.get(), 38);
    assert_eq!(preferences.interval(), CheckInterval::Custom(healthy));
    assert!(!preferences.notifications_enabled());
    assert!(preferences.tray_enabled());
    assert!(preferences.sign_in_enabled());
    Ok(())
}

/// Automatic and every preset retain the previous accepted custom interval.
///
/// # Errors
/// Propagates unexpected rejection of legal typed custom construction.
///
/// # Panics
/// Fails when changing a typed choice loses the retained custom value.
#[test]
fn typed_interval_changes_retain_last_custom_across_all_presets() -> Result<(), PreferenceError> {
    let mut preferences = UiPreferences::default();
    let custom = CustomMinutes::parse("00042")?;
    preferences.set_interval(CheckInterval::Custom(custom));
    for interval in [
        CheckInterval::Automatic,
        CheckInterval::TwoMinutes,
        CheckInterval::FiveMinutes,
        CheckInterval::FifteenMinutes,
        CheckInterval::Automatic,
    ] {
        preferences.set_interval(interval);
        assert_eq!(preferences.interval(), interval);
        assert_eq!(preferences.last_custom_minutes().get(), 42);
        assert!(preferences.notifications_enabled());
        assert!(!preferences.tray_enabled());
        assert!(!preferences.sign_in_enabled());
    }
    preferences.set_interval(CheckInterval::Custom(preferences.last_custom_minutes()));
    assert_eq!(preferences.interval(), CheckInterval::Custom(custom));
    Ok(())
}

/// Cancel is a view draft reset from accepted projections, not a model mutation.
///
/// # Errors
/// Propagates unexpected rejection of legal custom initialization.
///
/// # Panics
/// Fails when projections lose an accepted choice or retained custom value.
#[test]
fn cancel_can_restore_view_drafts_from_accepted_values_without_mutation()
-> Result<(), PreferenceError> {
    let mut preferences = UiPreferences::default();
    let accepted_custom = preferences.set_custom_minutes("00042")?;
    preferences.set_interval(CheckInterval::TwoMinutes);
    let before = preferences.clone();
    let mut draft_interval = CheckInterval::Automatic;
    let mut draft_custom = String::from("secret unsaved draft");
    assert_ne!(draft_interval, preferences.interval());
    assert_ne!(draft_custom, preferences.last_custom_minutes().to_string());
    assert_eq!(
        preferences.set_custom_minutes(&draft_custom),
        Err(PreferenceError::InvalidCustomMinutes)
    );
    draft_interval = preferences.interval();
    draft_custom = preferences.last_custom_minutes().to_string();
    assert_eq!(draft_interval, CheckInterval::TwoMinutes);
    assert_eq!(draft_custom, "42");
    assert_eq!(preferences.last_custom_minutes(), accepted_custom);
    assert_eq!(preferences, before);
    Ok(())
}

/// Notification preference changes preserve interval, tray and sign-in choices.
///
/// # Errors
/// Propagates unexpected rejection of legal custom and sign-in initialization.
///
/// # Panics
/// Fails when notification preference changes affect other accepted preferences.
#[test]
fn notifications_are_independent_of_tray_and_sign_in_preferences() -> Result<(), PreferenceError> {
    let mut preferences = UiPreferences::default();
    preferences.set_notifications_enabled(false);
    assert!(!preferences.notifications_enabled());
    assert!(!preferences.tray_enabled());
    let minutes = preferences.set_custom_minutes("19")?;
    preferences.set_tray_enabled(true);
    preferences.set_sign_in_enabled(true)?;
    for enabled in [true, false, false, true] {
        preferences.set_notifications_enabled(enabled);
        assert_eq!(preferences.notifications_enabled(), enabled);
        assert_eq!(preferences.interval(), CheckInterval::Custom(minutes));
        assert_eq!(preferences.last_custom_minutes(), minutes);
        assert!(preferences.tray_enabled());
        assert!(preferences.sign_in_enabled());
    }
    preferences.set_tray_enabled(false);
    assert!(preferences.notifications_enabled());
    assert!(!preferences.sign_in_enabled());
    assert_eq!(preferences.interval(), CheckInterval::Custom(minutes));
    Ok(())
}

/// Direct sign-in bypasses preserve state, while disabling tray clears sign-in.
///
/// # Errors
/// Propagates unexpected rejection of allowed sign-in and custom updates.
///
/// # Panics
/// Fails when a direct call bypasses the tray requirement or modifies other fields.
#[test]
fn sign_in_requires_current_tray_and_disabling_tray_clears_it() -> Result<(), PreferenceError> {
    let mut preferences = UiPreferences::default();
    preferences.set_notifications_enabled(false);
    let minutes = preferences.set_custom_minutes("7")?;
    let before = preferences.clone();
    assert_eq!(
        preferences.set_sign_in_enabled(true),
        Err(PreferenceError::TrayRequired)
    );
    assert_eq!(preferences, before);
    preferences.set_sign_in_enabled(false)?;
    assert_eq!(preferences, before);
    preferences.set_tray_enabled(true);
    assert!(!preferences.sign_in_enabled());
    preferences.set_sign_in_enabled(true)?;
    assert!(preferences.sign_in_enabled());
    preferences.set_tray_enabled(true);
    assert!(preferences.sign_in_enabled());
    preferences.set_sign_in_enabled(false)?;
    assert!(preferences.tray_enabled());
    preferences.set_sign_in_enabled(true)?;
    preferences.set_tray_enabled(false);
    assert!(!preferences.tray_enabled());
    assert!(!preferences.sign_in_enabled());
    assert!(!preferences.notifications_enabled());
    assert_eq!(preferences.interval(), CheckInterval::Custom(minutes));
    assert_eq!(preferences.last_custom_minutes(), minutes);
    let disabled = preferences.clone();
    assert_eq!(
        preferences.set_sign_in_enabled(true),
        Err(PreferenceError::TrayRequired)
    );
    assert_eq!(preferences, disabled);
    preferences.set_tray_enabled(true);
    assert!(!preferences.sign_in_enabled());
    Ok(())
}

/// Every error projection is fixed and contains no submitted custom text.
///
/// # Panics
/// Fails when rejected input leaks through public feedback, Debug or error source.
#[test]
fn preference_errors_expose_only_fixed_feedback() {
    let mut preferences = UiPreferences::default();
    let error = preferences.set_custom_minutes("secret\nvalue").err();
    assert_eq!(error, Some(PreferenceError::InvalidCustomMinutes));
    assert_eq!(
        PreferenceError::InvalidCustomMinutes.text(),
        "Enter whole minutes of at least 1, using digits only. For a very large value, try a smaller number."
    );
    assert_eq!(
        PreferenceError::InvalidCustomMinutes.to_string(),
        PreferenceError::InvalidCustomMinutes.text()
    );
    assert_eq!(
        format!("{:?}", PreferenceError::InvalidCustomMinutes),
        "InvalidCustomMinutes"
    );
    assert!(std::error::Error::source(&PreferenceError::InvalidCustomMinutes).is_none());
    assert_eq!(
        PreferenceError::TrayRequired.text(),
        "Enable the tray preference before opening at sign-in."
    );
    assert_eq!(
        PreferenceError::TrayRequired.to_string(),
        PreferenceError::TrayRequired.text()
    );
    assert!(std::error::Error::source(&PreferenceError::TrayRequired).is_none());
}

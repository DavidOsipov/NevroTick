//! Valid preference values cannot mutate hidden Settings workflows.
#![forbid(unsafe_code)]

use std::{error::Error, rc::Rc};

use nevrotick_ui::slint_host::{self, AppWindow, Page};
use slint::ComponentHandle;
use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{Platform, PlatformError, WindowAdapter};

/// Public software platform; no native registration, service or clock adapter.
struct Headless;

impl Platform for Headless {
    /// Create a hidden UI-thread-only software window, without an event loop.
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer))
    }
}

/// Fresh hosts challenge valid wrong-context values and later useful legal ones.
///
/// # Errors
/// Propagates platform/fixture setup failure rather than reporting absent checks.
///
/// # Panics
/// Fails on offscreen mutation, unrelated field-error loss or reject-everything.
#[test]
fn preference_receivers_require_their_current_named_settings_context() -> Result<(), Box<dyn Error>>
{
    slint::platform::set_platform(Box::new(Headless))?;
    let checks: [fn(&AppWindow); 3] = [
        check_valid_offscreen_payloads_and_spoofed_page_are_inert,
        check_other_settings_sections_and_error_retention,
        check_shared_preferences_preserve_a_retained_source_edit,
    ];
    for check in checks {
        let host = slint_host::create()?;
        let window = host.window();
        assert!(!window.window().is_visible());
        check(window);
        assert!(!window.window().is_visible());
        let weak = window.as_weak();
        drop(host);
        assert!(weak.upgrade().is_none());
    }
    Ok(())
}

/// Navigate through the same actual closed destination used by product controls.
fn section(window: &AppWindow, choice: i32) {
    window.invoke_settings_section_requested(choice);
}

/// Compare original raw interval fields without normalizing a rejected draft.
///
/// # Panics
/// Fails if a hidden Save or Cancel replaces raw interval text/selection.
fn assert_raw_interval(window: &AppWindow, choice: i32, raw: &str) {
    assert_eq!(window.get_draft_interval_choice(), choice);
    assert_eq!(window.get_draft_custom_minutes(), raw);
}

/// Field-valid input does not supply a trusted page or named-section context.
///
/// # Panics
/// Fails on valid wrong-context mutation or spoofed presentation authority.
fn check_valid_offscreen_payloads_and_spoofed_page_are_inert(window: &AppWindow) {
    assert_eq!(window.get_current_page(), Page::Status);
    section(window, 0);
    assert_eq!(window.invoke_raw_choice_edited(0, 4), 4);
    assert_eq!(
        window.invoke_raw_text_edited(4, "preserve raw 1.5".into()),
        "preserve raw 1.5"
    );
    window.invoke_navigate(Page::Status);
    for spoof in [false, true] {
        if spoof {
            window.set_current_page(Page::Settings);
            window.set_settings_section(4);
            window.set_power_mode(true);
        }
        window.invoke_interval_save_requested(4, "7".into());
        window.invoke_interval_cancel_requested();
        window.invoke_notifications_requested(false);
        window.invoke_tray_requested(true);
        window.invoke_sign_in_requested(true);
        assert_eq!(window.get_accepted_interval_label(), "Automatic");
        assert!(window.get_notifications_enabled());
        assert!(!window.get_tray_enabled());
        assert!(!window.get_sign_in_enabled());
        assert_eq!(window.get_interval_error(), "");
        assert_eq!(window.get_interval_feedback(), "");
        assert_eq!(window.get_preference_error(), "");
        assert_eq!(window.get_preference_feedback(), "");
        assert_raw_interval(window, 4, "preserve raw 1.5");
    }
    // A correct named destination admits the same value after both rejections.
    section(window, 0);
    assert_eq!(window.invoke_raw_text_edited(4, "7".into()), "7");
    window.invoke_interval_save_requested(4, "7".into());
    assert_eq!(
        window.get_accepted_interval_label(),
        "Every 7 minutes (custom)"
    );
    assert_raw_interval(window, 4, "7");
    // This accepted re-projection also observes the private preference owner.
    assert!(window.get_notifications_enabled());
    assert!(!window.get_tray_enabled());
    assert!(!window.get_sign_in_enabled());
    section(window, 4);
    window.invoke_tray_requested(true);
    window.invoke_sign_in_requested(true);
    assert!(window.get_tray_enabled());
    assert!(window.get_sign_in_enabled());
}

/// Other valid Settings groups cannot clear an interval draft or change its peers.
///
/// # Panics
/// Fails when a generic Settings page or valid wrong section bypasses admission.
fn check_other_settings_sections_and_error_retention(window: &AppWindow) {
    section(window, 0);
    assert_eq!(window.invoke_raw_choice_edited(0, 4), 4);
    assert_eq!(window.invoke_raw_text_edited(4, "0".into()), "0");
    section(window, 3);
    assert!(window.get_power_mode());
    for wrong_section in [1, 2, 3, 4] {
        section(window, wrong_section);
        window.invoke_interval_save_requested(4, "7".into());
        window.invoke_interval_cancel_requested();
        assert_eq!(window.get_accepted_interval_label(), "Automatic");
        assert_raw_interval(window, 4, "0");
    }
    section(window, 2);
    window.invoke_notifications_requested(false);
    assert!(!window.get_notifications_enabled());
    window.invoke_tray_requested(true);
    window.invoke_sign_in_requested(true);
    assert!(!window.get_tray_enabled());
    assert!(!window.get_sign_in_enabled());
    section(window, 4);
    window.invoke_notifications_requested(true);
    assert!(!window.get_notifications_enabled());
    window.invoke_tray_requested(true);
    window.invoke_sign_in_requested(true);
    assert!(window.get_tray_enabled());
    assert!(window.get_sign_in_enabled());
    assert!(!window.get_notifications_enabled());
    section(window, 0);
    window.invoke_interval_save_requested(4, "0".into());
    let error = window.get_interval_error();
    assert!(!error.is_empty());
    section(window, 2);
    window.invoke_interval_cancel_requested();
    window.invoke_notifications_requested(true);
    assert_eq!(window.get_interval_error(), error);
    assert_raw_interval(window, 4, "0");
    section(window, 0);
    assert_eq!(window.get_interval_error(), error);
    assert_eq!(window.invoke_raw_text_edited(4, "1".into()), "1");
    window.invoke_interval_save_requested(4, "1".into());
    assert_eq!(
        window.get_accepted_interval_label(),
        "Every 1 minute (custom)"
    );
    assert_eq!(window.get_interval_error(), "");
}

/// Correct shared settings remain usable while an independent source draft sleeps.
///
/// # Panics
/// Fails if new context checks turn Simple into privilege or discard source input.
fn check_shared_preferences_preserve_a_retained_source_edit(window: &AppWindow) {
    window.invoke_mode_requested(true);
    window.invoke_navigate(Page::Sources);
    window.invoke_edit_requested();
    assert_eq!(
        window.invoke_raw_text_edited(0, "Retained source draft".into()),
        "Retained source draft"
    );
    assert_eq!(
        window.invoke_raw_text_edited(1, " raw source host ".into()),
        " raw source host "
    );
    assert_eq!(
        window.invoke_raw_text_edited(2, "bad port".into()),
        "bad port"
    );
    section(window, 0);
    window.invoke_mode_requested(false);
    assert!(!window.get_power_mode());
    assert_eq!(window.invoke_raw_choice_edited(0, 4), 4);
    assert_eq!(window.invoke_raw_text_edited(4, "7".into()), "7");
    window.invoke_interval_save_requested(4, "7".into());
    section(window, 2);
    window.invoke_notifications_requested(false);
    section(window, 4);
    window.invoke_tray_requested(true);
    window.invoke_sign_in_requested(true);
    assert_eq!(
        window.get_accepted_interval_label(),
        "Every 7 minutes (custom)"
    );
    assert!(!window.get_notifications_enabled());
    assert!(window.get_tray_enabled());
    assert!(window.get_sign_in_enabled());
    assert!(window.get_editor_open());
    assert_eq!(window.get_draft_label(), "Retained source draft");
    assert_eq!(window.get_draft_host(), " raw source host ");
    assert_eq!(window.get_draft_port(), "bad port");
    window.invoke_mode_requested(true);
    window.invoke_navigate(Page::Sources);
    window.invoke_cancel_requested();
    assert!(!window.get_editor_open());
    assert_eq!(window.get_draft_host(), "north.example.invalid");
    assert_eq!(
        window.get_accepted_interval_label(),
        "Every 7 minutes (custom)"
    );
}

//! Real callbacks preserve mode/section ownership through Settings grouping.
#![forbid(unsafe_code)]

use std::{error::Error, rc::Rc};

use nevrotick_ui::slint_host::{self, AppWindow, Page};
use slint::ComponentHandle;
use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{Platform, PlatformError, WindowAdapter, WindowEvent};

/// Thread-owned software platform; no native window, service or external effects.
struct Headless;

impl Platform for Headless {
    /// Return a software adapter without adding native or worker-thread lifetime.
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer))
    }
}

/// Fresh real hosts exercise section, mode and modal state with useful neighbors.
///
/// # Errors
/// Propagates platform/window setup and cleanup failures explicitly.
///
/// # Panics
/// Fails when grouping loses raw previews, trusts view flags or redirects an intent.
#[test]
fn actual_settings_sections_preserve_context_and_local_previews() -> Result<(), Box<dyn Error>> {
    slint::platform::set_platform(Box::new(Headless))?;
    let checks: [fn(&AppWindow); 4] = [
        check_closed_sections_and_mode_reconciliation,
        check_dirty_recovery_holds_exact_section_and_mode,
        check_policy_context_and_quit_modal_bypasses,
        check_preference_feedback_keeps_its_group,
    ];
    for check in checks {
        let host = slint_host::create()?;
        let window = host.window();
        window
            .window()
            .set_size(slint::LogicalSize::new(640.0, 600.0));
        window.show()?;
        window
            .window()
            .dispatch_event(WindowEvent::WindowActiveChanged(true));
        check(window);
        window.hide()?;
        let weak = window.as_weak();
        drop(host);
        assert!(weak.upgrade().is_none());
    }
    Ok(())
}

/// Compare the reached section and mode, with the page checked independently.
///
/// # Panics
/// Fails when a requested advanced destination is only partially committed.
fn assert_settings(window: &AppWindow, section: i32, power: bool) {
    assert_eq!(window.get_current_page(), Page::Settings);
    assert_eq!(window.get_settings_section(), section);
    assert_eq!(window.get_power_mode(), power);
}

/// Shared sections keep mode; advanced profile entry and mode exit commit once.
///
/// # Panics
/// Fails on unknown-choice mutation or discarded accepted/raw policy state.
fn check_closed_sections_and_mode_reconciliation(window: &AppWindow) {
    window.invoke_navigate(Page::Settings);
    assert_eq!(window.invoke_raw_choice_edited(0, 4), 4);
    assert_eq!(
        window.invoke_raw_text_edited(4, "raw 1.5".into()),
        "raw 1.5"
    );
    for section in [0, 1, 2, 4] {
        window.invoke_settings_section_requested(section);
        assert_settings(window, section, false);
        assert!(window.get_settings_section_focused());
    }
    for invalid in [i32::MIN, -1, 5, i32::MAX] {
        window.invoke_settings_section_requested(invalid);
        assert_settings(window, 4, false);
        assert_eq!(window.get_draft_custom_minutes(), "raw 1.5");
    }
    window.invoke_settings_section_requested(3);
    assert_settings(window, 3, true);
    assert!(window.get_settings_section_focused());
    window.invoke_policy_edit_requested();
    assert_eq!(window.invoke_raw_choice_edited(3, 2), 2);
    window.invoke_mode_requested(false);
    assert_settings(window, 0, false);
    assert_eq!(window.get_policy_phase(), 1);
    assert_eq!(window.get_draft_policy_choice(), 2);
    assert!(!window.get_policy_has_preview());
    window.set_settings_section(3);
    window.set_power_mode(true);
    window.invoke_policy_review_requested(2);
    assert!(!window.get_policy_has_preview());
    assert_eq!(window.get_policy_phase(), 1);
    assert_eq!(window.get_draft_policy_choice(), 2);
    window.invoke_settings_section_requested(3);
    assert_settings(window, 3, true);
    window.invoke_policy_review_requested(2);
    assert_eq!(window.get_policy_phase(), 2);
    assert_eq!(window.get_policy_accepted_choice(), 2);
    assert!(window.get_policy_has_preview());
    assert!(window.get_settings_section_focused());
    window.invoke_policy_keep_requested();
    window.invoke_settings_section_requested(4);
    window.invoke_mode_requested(false);
    assert_settings(window, 4, false);
    assert!(window.get_policy_has_preview());
    assert_eq!(window.get_policy_accepted_choice(), 2);
    assert_eq!(window.get_draft_custom_minutes(), "raw 1.5");
}

/// First dirty-helper intent owns both section and mode until explicitly resolved.
///
/// # Panics
/// Fails when denied/Stay routes alter section or a second request redirects Keep.
fn check_dirty_recovery_holds_exact_section_and_mode(window: &AppWindow) {
    window.invoke_navigate(Page::Settings);
    window.invoke_recovery_open_requested();
    assert_eq!(window.invoke_raw_choice_edited(1, 1), 1);
    assert_eq!(
        window.invoke_raw_text_edited(5, "helper.example.invalid".into()),
        "helper.example.invalid"
    );
    window.invoke_settings_section_requested(3);
    assert!(window.get_recovery_leave_open());
    assert_eq!(window.get_current_page(), Page::Recovery);
    assert_eq!(window.get_settings_section(), 0);
    assert!(window.get_power_mode());
    window.invoke_settings_section_requested(4);
    window.invoke_mode_requested(false);
    window.invoke_recovery_leave_stay_requested();
    assert_eq!(window.get_current_page(), Page::Recovery);
    assert_eq!(window.get_settings_section(), 0);
    assert_eq!(window.get_draft_recovery_first(), "helper.example.invalid");
    window.invoke_settings_section_requested(3);
    window.set_recovery_leave_open(false);
    window.invoke_settings_section_requested(4);
    window.invoke_recovery_leave_keep_requested();
    assert_settings(window, 3, true);
    assert_eq!(window.get_draft_recovery_first(), "helper.example.invalid");
    assert!(window.get_settings_section_focused());
    window.invoke_mode_requested(false);
    assert_settings(window, 0, false);
    window.invoke_recovery_open_requested();
    window.invoke_settings_section_requested(4);
    assert!(window.get_recovery_leave_open());
    window.invoke_recovery_leave_discard_requested();
    assert_settings(window, 4, true);
    assert_eq!(window.get_draft_recovery_count(), 0);
    assert_eq!(window.get_draft_recovery_first(), "");
    window.invoke_mode_requested(false);
    assert_settings(window, 4, false);
}

/// Wrong-section and real quit guards reject even apparently valid UI properties.
///
/// # Panics
/// Fails if hidden policy actions acquire an owner or bypass a modal transition.
fn check_policy_context_and_quit_modal_bypasses(window: &AppWindow) {
    window.invoke_navigate(Page::Settings);
    window.set_settings_section(3);
    window.set_power_mode(true);
    window.invoke_policy_edit_requested();
    assert_eq!(window.get_policy_phase(), 0);
    window.invoke_settings_section_requested(3);
    assert_settings(window, 3, true);
    window.invoke_policy_edit_requested();
    assert_eq!(window.invoke_raw_choice_edited(3, 1), 1);
    window.invoke_quit_requested();
    assert!(window.get_quit_warning_open());
    window.set_quit_warning_open(false);
    window.invoke_settings_section_requested(4);
    window.invoke_mode_requested(false);
    window.invoke_policy_review_requested(1);
    assert_eq!(window.get_settings_section(), 3);
    assert!(window.get_power_mode());
    assert_eq!(window.get_policy_phase(), 1);
    assert!(!window.get_policy_has_preview());
    assert_eq!(window.get_draft_policy_choice(), 1);
    window.invoke_quit_cancel_requested();
    window.invoke_policy_review_requested(1);
    assert!(window.get_policy_has_preview());
    assert_eq!(window.get_policy_accepted_choice(), 1);
    assert!(window.get_settings_section_focused());
    window.invoke_policy_keep_requested();
    window.invoke_settings_section_requested(2);
    window.invoke_policy_edit_requested();
    assert_eq!(window.get_policy_phase(), 0);
    assert_eq!(window.get_policy_accepted_choice(), 1);
    window.invoke_settings_section_requested(3);
    window.invoke_policy_edit_requested();
    assert_eq!(window.get_policy_phase(), 1);
    assert_eq!(window.get_draft_policy_choice(), 1);
}

/// Preference feedback follows its originating group rather than the next tab.
///
/// # Panics
/// Fails if section navigation relabels/clears an existing validation message.
fn check_preference_feedback_keeps_its_group(window: &AppWindow) {
    window.invoke_navigate(Page::Settings);
    window.invoke_settings_section_requested(4);
    window.invoke_sign_in_requested(true);
    assert_eq!(window.get_preference_feedback_section(), 4);
    let sign_in_error = window.get_preference_error();
    assert!(!sign_in_error.is_empty());
    window.invoke_settings_section_requested(2);
    assert_eq!(window.get_preference_feedback_section(), 4);
    assert_eq!(window.get_preference_error(), sign_in_error);
    window.invoke_notifications_requested(false);
    assert_eq!(window.get_preference_feedback_section(), 2);
    assert_eq!(window.get_preference_error(), "");
    let notification_feedback = window.get_preference_feedback();
    assert!(notification_feedback.contains("Notification"));
    window.invoke_settings_section_requested(4);
    assert_eq!(window.get_preference_feedback_section(), 2);
    assert_eq!(window.get_preference_feedback(), notification_feedback);
    window.invoke_tray_requested(true);
    assert_eq!(window.get_preference_feedback_section(), 4);
    assert!(window.get_preference_feedback().contains("Tray"));
    window.invoke_sign_in_requested(true);
    assert!(window.get_sign_in_enabled());
    assert_eq!(window.get_preference_feedback_section(), 4);
    assert_eq!(window.get_preference_error(), "");
}

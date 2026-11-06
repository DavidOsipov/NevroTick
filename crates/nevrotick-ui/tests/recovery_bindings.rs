//! Actual generated recovery callbacks against independent fixture expectations.
#![forbid(unsafe_code)]

use std::{error::Error, rc::Rc};

use nevrotick_ui::model::ModelError;
use nevrotick_ui::presentation::{NavigationState, Page as SessionPage, UiError};
use nevrotick_ui::recovery::RecoveryError;
use nevrotick_ui::slint_host::{self, AppWindow, Page};
use nevrotick_ui::source_drafts::SourceDraftError;
use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{Key, Platform, PlatformError, WindowAdapter, WindowEvent};
use slint::{ComponentHandle, Model};

/// Public software window for generated receiving/model checks without rendering.
struct Headless;

impl Platform for Headless {
    /// Construct no native window, timer or application event loop.
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer))
    }
}

/// Named callback and its expected deliberate display lifetime.
struct RecoveryCase {
    /// Stable scenario name used by teardown diagnostics.
    name: &'static str,
    /// Actual generated callback scenario on one fresh host.
    check: fn(&AppWindow),
    /// True only for the case that intentionally leaves its warning window shown.
    expected_shown: bool,
}

/// Each case receives a fresh checked root; all callbacks own only weak handles.
///
/// # Errors
/// Propagates platform/fixture setup failure rather than hiding an absent check.
///
/// # Panics
/// Fails if a recovery receiving, navigation, atomicity or lifetime rule breaks.
#[test]
fn generated_recovery_callbacks_preserve_context_and_independent_state()
-> Result<(), Box<dyn Error>> {
    slint::platform::set_platform(Box::new(Headless))?;
    let checks = [
        RecoveryCase {
            name: "contextual entry and back",
            check: check_contextual_entry_and_back,
            expected_shown: false,
        },
        RecoveryCase {
            name: "direct admission and atomicity",
            check: check_direct_admission_and_atomicity,
            expected_shown: false,
        },
        RecoveryCase {
            name: "review cancel discard and spoofed phase",
            check: check_review_cancel_discard_and_spoofed_phase,
            expected_shown: false,
        },
        RecoveryCase {
            name: "dirty leave resolution and mode",
            check: check_dirty_leave_resolution_and_mode,
            expected_shown: false,
        },
        RecoveryCase {
            name: "source and preference independence",
            check: check_source_and_preference_independence,
            expected_shown: false,
        },
        RecoveryCase {
            name: "page scroll reset and back restore",
            check: check_page_scroll_reset_and_back_restore,
            expected_shown: false,
        },
        RecoveryCase {
            name: "modal keyboard containment",
            check: check_modal_keyboard_containment,
            expected_shown: false,
        },
        // Quit on this initially unshown root deliberately shows its warning.
        // Manage then keeps that observer open at the accepted service settings.
        RecoveryCase {
            name: "peer service route waits for leave acceptance",
            check: check_peer_service_route_waits_for_leave_acceptance,
            expected_shown: true,
        },
    ];
    for RecoveryCase {
        name,
        check,
        expected_shown,
    } in checks
    {
        let host = slint_host::create()?;
        let window = host.window();
        window
            .window()
            .set_size(slint::LogicalSize::new(1080.0, 780.0));
        check(window);
        assert_eq!(window.window().is_visible(), expected_shown, "{name}");
        if expected_shown {
            // This fixture ends an intentionally still-running observer. The
            // owned-loop shell test separately verifies successful real Close.
            window.hide()?;
        }
        let weak = window.as_weak();
        drop(host);
        assert!(weak.upgrade().is_none(), "{name}");
    }
    let restarted = slint_host::create()?;
    assert_eq!(preview(restarted.window()), Vec::<String>::new());
    assert!(!restarted.window().get_recovery_review_open());
    assert!(!restarted.window().get_recovery_leave_open());
    assert_raw(restarted.window(), 0, "", "");
    Ok(())
}

/// Read only the accepted literal helper identity projection.
fn preview(window: &AppWindow) -> Vec<String> {
    window
        .get_recovery_preview_hosts()
        .iter()
        .map(|host| host.to_string())
        .collect()
}

/// Enter bounded raw text through visible fields before selecting the wanted count.
/// Temporary count2 lets unused text originate in a legitimate editable field.
///
/// # Panics
/// Fails when ordinary fixture input is rejected before its Submit action.
fn set_raw(window: &AppWindow, count: i32, first: &str, second: &str) {
    assert_eq!(window.invoke_raw_choice_edited(1, 2), 2);
    assert_eq!(window.invoke_raw_text_edited(5, first.into()), first);
    assert_eq!(window.invoke_raw_text_edited(6, second.into()), second);
    assert_eq!(window.invoke_raw_choice_edited(1, count), count);
}

/// Compare all raw fields, including unused text retained only by the view.
///
/// # Panics
/// Fails if navigation, rejection or review normalizes/loses a draft.
fn assert_raw(window: &AppWindow, count: i32, first: &str, second: &str) {
    assert_eq!(window.get_draft_recovery_count(), count);
    assert_eq!(window.get_draft_recovery_first(), first);
    assert_eq!(window.get_draft_recovery_second(), second);
}

/// Submit a legitimately entered bounded raw draft to its independent receiver.
///
/// # Panics
/// Fails when the fixture cannot enter its requested raw editor values.
fn submit(window: &AppWindow, count: i32, first: &str, second: &str) {
    set_raw(window, count, first, second);
    window.invoke_recovery_update_requested(count, first.into(), second.into());
}

/// Explicit details enter Power; Back restores shared origin mode/page exactly.
///
/// # Panics
/// Fails when generic hidden-page bypasses Simple, or Back loses shared context.
fn check_contextual_entry_and_back(window: &AppWindow) {
    assert!(SessionPage::Recovery.is_power_only());
    let mut navigation = NavigationState::new();
    let before = navigation.clone();
    assert_eq!(
        navigation.navigate(SessionPage::Recovery),
        Err(UiError::PageUnavailable)
    );
    assert_eq!(navigation, before);
    window.invoke_navigate(Page::Recovery);
    assert_eq!(window.get_current_page(), Page::Status);
    assert!(!window.get_power_mode());
    assert_eq!(window.get_editor_error(), UiError::PageUnavailable.text());
    for shared in [Page::Settings, Page::About, Page::Status] {
        window.invoke_navigate(shared);
        window.invoke_recovery_open_requested();
        assert_eq!(window.get_current_page(), Page::Recovery);
        assert!(window.get_power_mode());
        assert!(!window.get_recovery_back_label().is_empty());
        window.invoke_recovery_back_requested();
        assert_eq!(window.get_current_page(), shared);
        assert!(!window.get_power_mode());
    }
    window.invoke_mode_requested(true);
    window.invoke_navigate(Page::Sources);
    window.invoke_mode_requested(false);
    window.invoke_recovery_open_requested();
    window.invoke_recovery_back_requested();
    assert_eq!(window.get_current_page(), Page::Status);
    assert!(!window.get_power_mode());
    window.invoke_mode_requested(true);
    assert_eq!(window.get_current_page(), Page::Sources);
    window.invoke_source_selected(3);
    window.invoke_navigate(Page::Recovery);
    assert_eq!(window.get_recovery_back_label(), "Back to Sources");
    window.invoke_recovery_back_requested();
    assert_eq!(window.get_current_page(), Page::Sources);
    assert!(window.get_power_mode());
    assert_eq!(window.get_selected_source(), 3);
    window.invoke_recovery_update_requested(1, "outside.example.invalid".into(), "".into());
    assert_eq!(preview(window), Vec::<String>::new());
    assert_eq!(window.get_current_page(), Page::Sources);
}

/// Malformed signed counts/used identities fail atomically through real bindings.
///
/// # Panics
/// Fails when a rejection replaces accepted helpers, normalizes raw text or leaks it.
fn check_direct_admission_and_atomicity(window: &AppWindow) {
    window.invoke_recovery_open_requested();
    submit(window, 1, "first.example.invalid", "unused invalid host");
    assert_eq!(preview(window), ["first.example.invalid"]);
    assert_raw(window, 1, "first.example.invalid", "unused invalid host");
    assert!(window.get_recovery_review_open());
    window.invoke_recovery_keep_editing_requested();
    check_invalid_helper_choices(window);
    check_invalid_helper_hosts(window);
    set_raw(
        window,
        1,
        "retained.example.invalid",
        "unused bounded draft",
    );
    window.invoke_recovery_update_requested(
        1,
        "other-valid.example.invalid".into(),
        "inert".into(),
    );
    assert_eq!(
        window.get_recovery_error(),
        "The submitted values do not match the current draft. Review the draft and try again."
    );
    assert_eq!(preview(window), ["first.example.invalid"]);
    assert_raw(
        window,
        1,
        "retained.example.invalid",
        "unused bounded draft",
    );
    window.invoke_recovery_update_requested(
        2,
        "retained.example.invalid".into(),
        "other-valid.example.invalid".into(),
    );
    assert_eq!(
        window.get_recovery_error(),
        "The submitted values do not match the current draft. Review the draft and try again."
    );
    assert_raw(
        window,
        1,
        "retained.example.invalid",
        "unused bounded draft",
    );
    let oversized = format!(
        "{}.{}.{}.{}.",
        "a".repeat(63),
        "b".repeat(63),
        "c".repeat(63),
        "d".repeat(61)
    );
    assert_eq!(oversized.len(), 254);
    submit(window, 1, &oversized, "");
    assert_eq!(preview(window), ["first.example.invalid"]);
    assert_raw(window, 1, &oversized, "");
    for (first, second) in [
        ("CLOCK.example.invalid", "clock.EXAMPLE.invalid."),
        ("2001:db8::1", "2001:0DB8:0:0:0:0:0:1"),
    ] {
        submit(window, 2, first, second);
        assert_eq!(preview(window), ["first.example.invalid"]);
        assert_eq!(
            window.get_recovery_error(),
            RecoveryError::Source(SourceDraftError::DuplicateIdentity).to_string()
        );
        assert_raw(window, 2, first, second);
    }
    submit(window, 2, "192.0.2.1", "2001:db8::2");
    assert_eq!(preview(window), ["192.0.2.1", "2001:db8::2"]);
    assert!(window.get_recovery_review_open());
    assert_eq!(window.get_recovery_error(), "");
}

/// Reject raw selector forgeries and independently malformed Submit choices.
/// Caller owns the Editing phase with one accepted first helper.
///
/// # Panics
/// Fails on signed-choice fallback, raw replacement or accepted-preview mutation.
fn check_invalid_helper_choices(window: &AppWindow) {
    for count in [i32::MIN, -1, 3, i32::MAX] {
        let prior_count = window.get_draft_recovery_count();
        let prior_first = window.get_draft_recovery_first();
        let prior_second = window.get_draft_recovery_second();
        assert_eq!(window.invoke_raw_choice_edited(1, count), prior_count);
        // The direct Submit receiver still challenges its own signed admission.
        window.invoke_recovery_update_requested(
            count,
            " raw host ".into(),
            "another raw host".into(),
        );
        assert_eq!(preview(window), ["first.example.invalid"]);
        assert!(!window.get_recovery_review_open());
        assert_eq!(
            window.get_recovery_error(),
            "The submitted values do not match the current draft. Review the draft and try again."
        );
        assert_raw(window, prior_count, &prior_first, &prior_second);
    }
}

/// Keep bounded ASCII format errors but reject unsupported raw characters early.
/// Caller owns the same Editing phase and retained first-helper preview.
///
/// # Panics
/// Fails if raw admission bypasses the repertoire or Submit bypasses host checks.
fn check_invalid_helper_hosts(window: &AppWindow) {
    for host in [
        "",
        "bad host",
        "https://wrong.invalid",
        "999.999.999.999",
        "192.000.2.1",
        "name.invalid..",
    ] {
        submit(window, 2, "first.example.invalid", host);
        assert_eq!(preview(window), ["first.example.invalid"]);
        assert_eq!(
            window.get_recovery_error(),
            RecoveryError::Source(SourceDraftError::Field(ModelError::InvalidHost)).to_string()
        );
        assert_raw(window, 2, "first.example.invalid", host);
        assert!(!window.get_recovery_error().contains(host) || host.is_empty());
    }
    // Rejected repertoire cannot replace the raw owner; its mismatched Submit
    // fails before the helper/domain boundary.
    let prior_second = window.get_draft_recovery_second();
    assert_eq!(
        window.invoke_raw_text_edited(6, "éxample.invalid".into()),
        prior_second
    );
    window.invoke_recovery_update_requested(
        2,
        "first.example.invalid".into(),
        "éxample.invalid".into(),
    );
    assert_eq!(preview(window), ["first.example.invalid"]);
    assert_eq!(
        window.get_recovery_error(),
        "The submitted values do not match the current draft. Review the draft and try again."
    );
    assert_raw(window, 2, "first.example.invalid", &prior_second);
}

/// UI phase flags never authorize commands, and Cancel differs from Discard.
///
/// # Panics
/// Fails when a forged review flag changes helpers or erases unsubmitted drafts.
fn check_review_cancel_discard_and_spoofed_phase(window: &AppWindow) {
    window.invoke_recovery_open_requested();
    window.set_recovery_review_open(true);
    window.invoke_recovery_discard_requested();
    assert!(!window.get_recovery_review_open());
    assert_eq!(preview(window), Vec::<String>::new());
    submit(window, 1, "accepted.example.invalid", "unused bad host");
    let accepted = preview(window);
    window.set_recovery_review_open(false);
    window.invoke_recovery_update_requested(0, "".into(), "".into());
    assert!(window.get_recovery_review_open());
    assert_eq!(preview(window), accepted);
    assert_raw(window, 1, "accepted.example.invalid", "unused bad host");
    window.set_recovery_review_open(false);
    window.invoke_recovery_cancel_requested();
    assert!(window.get_recovery_review_open());
    assert_eq!(preview(window), accepted);
    window.invoke_recovery_keep_editing_requested();
    assert_raw(window, 1, "accepted.example.invalid", "unused bad host");
    set_raw(window, 2, " unsubmitted ", "also invalid");
    window.invoke_recovery_cancel_requested();
    assert_raw(window, 1, "accepted.example.invalid", "");
    assert_eq!(preview(window), accepted);
    assert!(!window.get_recovery_review_open());
    submit(window, 2, "192.0.2.1", "::ffff:192.0.2.1");
    assert_eq!(preview(window), ["192.0.2.1", "::ffff:192.0.2.1"]);
    window.invoke_recovery_discard_requested();
    assert_eq!(preview(window), Vec::<String>::new());
    assert_raw(window, 0, "", "");
    assert!(!window.get_recovery_review_open());
    submit(window, 0, "unused invalid host", "also unused / host");
    assert_eq!(preview(window), Vec::<String>::new());
    assert_raw(window, 0, "unused invalid host", "also unused / host");
    assert!(window.get_recovery_review_open());
    window.invoke_recovery_discard_requested();
    assert_raw(window, 0, "", "");
}

/// First leave intent wins; Keep/Discard/Stay preserve the right local state.
///
/// # Panics
/// Fails when navigation bypasses dirty fields, redirects intent or loses context.
fn check_dirty_leave_resolution_and_mode(window: &AppWindow) {
    window.invoke_navigate(Page::Settings);
    window.invoke_recovery_open_requested();
    set_raw(window, 1, " raw invalid host ", "unused");
    window.invoke_recovery_back_requested();
    assert!(window.get_recovery_leave_open());
    assert_eq!(window.get_current_page(), Page::Recovery);
    window.invoke_notifications_requested(false);
    assert!(window.get_notifications_enabled());
    window.invoke_remove_requested(1);
    assert_eq!(window.get_sources().row_count(), 3);
    window.set_recovery_leave_open(false);
    window.invoke_recovery_update_requested(1, "new.example.invalid".into(), "".into());
    assert_eq!(preview(window), Vec::<String>::new());
    assert_raw(window, 1, " raw invalid host ", "unused");
    window.invoke_navigate(Page::About);
    window.invoke_recovery_leave_stay_requested();
    assert!(!window.get_recovery_leave_open());
    assert_eq!(window.get_current_page(), Page::Recovery);
    assert_raw(window, 1, " raw invalid host ", "unused");
    window.invoke_recovery_back_requested();
    window.invoke_navigate(Page::About);
    window.invoke_recovery_leave_keep_requested();
    assert_eq!(window.get_current_page(), Page::Settings);
    assert!(!window.get_power_mode());
    assert_raw(window, 1, " raw invalid host ", "unused");
    window.invoke_recovery_open_requested();
    window.invoke_navigate(Page::About);
    assert!(window.get_recovery_leave_open());
    window.invoke_recovery_leave_discard_requested();
    assert_eq!(window.get_current_page(), Page::About);
    assert!(window.get_power_mode());
    assert_raw(window, 0, "", "");
    assert_eq!(preview(window), Vec::<String>::new());
    window.invoke_recovery_open_requested();
    set_raw(window, 2, "first.example.invalid", "second.example.invalid");
    window.invoke_mode_requested(false);
    assert_eq!(window.get_current_page(), Page::Recovery);
    assert!(window.get_power_mode());
    assert!(window.get_recovery_leave_open());
    window.invoke_recovery_cancel_requested();
    assert_raw(window, 2, "first.example.invalid", "second.example.invalid");
    window.invoke_recovery_leave_keep_requested();
    assert!(!window.get_power_mode());
    assert_eq!(window.get_current_page(), Page::Status);
    window.invoke_mode_requested(true);
    assert_eq!(window.get_current_page(), Page::Recovery);
    window.invoke_recovery_back_requested();
    window.invoke_recovery_leave_discard_requested();
    assert_eq!(window.get_current_page(), Page::Status);
    assert!(!window.get_power_mode());
    assert_raw(window, 0, "", "");
    window.set_recovery_leave_open(true);
    window.invoke_recovery_leave_keep_requested();
    assert!(!window.get_recovery_leave_open());
    assert_eq!(window.get_current_page(), Page::Status);
}

/// Helper preview/Back does not edit ordinary sources, source raw drafts or prefs.
///
/// # Panics
/// Fails when the recovery child becomes a second source/preference owner.
fn check_source_and_preference_independence(window: &AppWindow) {
    window.invoke_mode_requested(true);
    window.invoke_navigate(Page::Sources);
    window.invoke_source_selected(3);
    window.invoke_edit_requested();
    assert_eq!(
        window.invoke_raw_text_edited(0, "Keep source draft".into()),
        "Keep source draft"
    );
    assert_eq!(
        window.invoke_raw_text_edited(1, " source host raw ".into()),
        " source host raw "
    );
    assert_eq!(window.invoke_raw_text_edited(2, "bad".into()), "bad");
    let sources = window.get_sources().iter().collect::<Vec<_>>();
    let changes = window.get_source_changes().iter().collect::<Vec<_>>();
    window.invoke_settings_section_requested(0);
    assert_eq!(window.invoke_raw_choice_edited(0, 4), 4);
    assert_eq!(window.invoke_raw_text_edited(4, "7".into()), "7");
    window.invoke_interval_save_requested(4, "7".into());
    assert_eq!(
        window.invoke_raw_text_edited(4, "fraction 1.5".into()),
        "fraction 1.5"
    );
    window.invoke_settings_section_requested(2);
    window.invoke_notifications_requested(false);
    window.invoke_settings_section_requested(4);
    window.invoke_tray_requested(true);
    window.invoke_sign_in_requested(true);
    window.invoke_navigate(Page::Settings);
    window.invoke_mode_requested(false);
    window.invoke_recovery_open_requested();
    submit(window, 2, "first.example.invalid", "second.example.invalid");
    window.invoke_recovery_back_requested();
    assert_eq!(window.get_current_page(), Page::Settings);
    assert!(!window.get_power_mode());
    assert_eq!(window.get_sources().iter().collect::<Vec<_>>(), sources);
    assert_eq!(
        window.get_source_changes().iter().collect::<Vec<_>>(),
        changes
    );
    assert_eq!(window.get_selected_source(), 3);
    assert!(window.get_editor_open());
    assert_eq!(window.get_draft_label(), "Keep source draft");
    assert_eq!(window.get_draft_host(), " source host raw ");
    assert_eq!(window.get_draft_port(), "bad");
    assert_eq!(
        window.get_accepted_interval_label(),
        "Every 7 minutes (custom)"
    );
    assert_eq!(window.get_draft_custom_minutes(), "fraction 1.5");
    assert!(!window.get_notifications_enabled());
    assert!(window.get_tray_enabled());
    assert!(window.get_sign_in_enabled());
}

/// Actual generated page-scroll setters and navigation restore bounded context.
///
/// # Panics
/// Fails when a new page inherits prior scroll or Back loses its origin scroll.
fn check_page_scroll_reset_and_back_restore(window: &AppWindow) {
    window.invoke_mode_requested(true);
    window.invoke_navigate(Page::Sources);
    window.set_page_scroll_y(-230.0);
    window.invoke_navigate(Page::Settings);
    assert_scroll(window, 0.0);
    window.set_page_scroll_y(-420.0);
    window.invoke_recovery_open_requested();
    assert_eq!(window.get_current_page(), Page::Recovery);
    assert_scroll(window, 0.0);
    window.set_page_scroll_y(-75.0);
    window.invoke_recovery_back_requested();
    assert_eq!(window.get_current_page(), Page::Settings);
    assert_scroll(window, -420.0);
    window.set_page_scroll_y(-90.0);
    window.invoke_navigate(Page::Settings);
    assert_scroll(window, -90.0);
    window.invoke_mode_requested(false);
    assert_eq!(window.get_current_page(), Page::Settings);
    assert_scroll(window, -90.0);
    window.invoke_recovery_open_requested();
    assert_scroll(window, 0.0);
    window.invoke_recovery_back_requested();
    assert!(!window.get_power_mode());
    assert_eq!(window.get_current_page(), Page::Settings);
    assert_scroll(window, -90.0);
}

/// Scroll fixtures are exactly representable logical pixels, compared by bits.
///
/// # Panics
/// Fails when the actual generated property differs from its specified position.
fn assert_scroll(window: &AppWindow, expected: f32) {
    assert_eq!(window.get_page_scroll_y().to_bits(), expected.to_bits());
}

/// Dispatch through the public software platform, never a native input injector.
fn key(window: &AppWindow, key: Key) {
    let text: slint::SharedString = key.into();
    window
        .window()
        .dispatch_event(WindowEvent::KeyPressed { text: text.clone() });
    window
        .window()
        .dispatch_event(WindowEvent::KeyReleased { text });
}

/// The actual modal controls contain Tab and map Escape/Return to safe Stay.
///
/// # Panics
/// Fails if focus escapes the active leave dialog or keyboard changes raw drafts.
fn check_modal_keyboard_containment(window: &AppWindow) {
    window
        .window()
        .dispatch_event(WindowEvent::WindowActiveChanged(true));
    window.invoke_recovery_open_requested();
    set_raw(window, 1, "pending.example.invalid", "unused raw");
    window.set_page_scroll_y(-80.0);
    let recovery_scroll = window.get_page_scroll_y();
    window.invoke_recovery_back_requested();
    assert!(window.get_recovery_leave_open());
    window.invoke_focus_recovery_leave_safe();
    assert!(window.get_recovery_leave_safe_focused());
    assert!(window.get_recovery_leave_focus_contained());
    for _ in 0..4 {
        key(window, Key::Tab);
        assert!(window.get_recovery_leave_focus_contained());
    }
    window.window().dispatch_event(WindowEvent::KeyPressed {
        text: Key::Shift.into(),
    });
    for _ in 0..3 {
        key(window, Key::Tab);
        assert!(window.get_recovery_leave_focus_contained());
    }
    window.window().dispatch_event(WindowEvent::KeyReleased {
        text: Key::Shift.into(),
    });
    key(window, Key::Escape);
    assert!(!window.get_recovery_leave_open());
    assert_eq!(window.get_current_page(), Page::Recovery);
    assert!(window.get_recovery_content_focused());
    assert_scroll(window, recovery_scroll);
    assert_raw(window, 1, "pending.example.invalid", "unused raw");
    window.invoke_recovery_back_requested();
    window.invoke_focus_recovery_leave_safe();
    key(window, Key::Return);
    assert!(!window.get_recovery_leave_open());
    assert_eq!(window.get_current_page(), Page::Recovery);
    assert!(window.get_recovery_content_focused());
    assert_scroll(window, recovery_scroll);
    assert_raw(window, 1, "pending.example.invalid", "unused raw");
}

/// A peer service destination commits section/focus only after draft resolution.
///
/// # Panics
/// Fails if Manage service bypasses raw recovery edits or Stay commits its route.
fn check_peer_service_route_waits_for_leave_acceptance(window: &AppWindow) {
    window
        .window()
        .dispatch_event(WindowEvent::WindowActiveChanged(true));
    window.invoke_recovery_open_requested();
    set_raw(window, 1, "pending.example.invalid", "unused");
    window.invoke_quit_requested();
    assert!(window.get_quit_warning_open());
    window.invoke_manage_service_requested();
    assert!(!window.get_quit_warning_open());
    assert!(window.get_recovery_leave_open());
    assert_eq!(window.get_current_page(), Page::Recovery);
    assert_eq!(window.get_settings_section(), 0);
    assert!(!window.get_service_section_focused());
    window.invoke_recovery_leave_stay_requested();
    assert_eq!(window.get_current_page(), Page::Recovery);
    assert_eq!(window.get_settings_section(), 0);
    assert_raw(window, 1, "pending.example.invalid", "unused");
    window.invoke_quit_requested();
    window.invoke_manage_service_requested();
    assert!(window.get_recovery_leave_open());
    window.invoke_recovery_leave_keep_requested();
    assert!(!window.get_recovery_leave_open());
    assert_eq!(window.get_current_page(), Page::Settings);
    assert_eq!(window.get_settings_section(), 1);
    assert!(window.get_service_section_focused());
    assert_raw(window, 1, "pending.example.invalid", "unused");
    assert_eq!(preview(window), Vec::<String>::new());
}

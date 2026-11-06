//! Real generated callback checks on Slint's public software-window platform.
//!
//! These tests exercise Rust bindings and observable models without a display,
//! event loop or native Windows behavior. No internal Slint testing API is used.
#![forbid(unsafe_code)]

use std::{
    cell::{Cell, RefCell},
    error::Error,
    rc::Rc,
};

use nevrotick_ui::model::ModelError;
use nevrotick_ui::presentation::UiError;
use nevrotick_ui::slint_host::{self, AppWindow, Page, SourceRow};
use nevrotick_ui::source_drafts::SourceDraftError;
use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{Platform, PlatformError, WindowAdapter};
use slint::{ComponentHandle, FilterModel, Model};

/// Fixed code-owned stale-submission reason; never reflects submitted bytes.
const RAW_STALE: &str =
    "The submitted values do not match the current draft. Review the draft and try again.";

/// Public software adapter for property/callback checks without native windows.
struct Headless {
    /// Bounded by the two code-owned repository invocations in this test.
    urls: Rc<RefCell<Vec<String>>>,
    /// Controlled platform response; neither response launches a browser.
    browser_available: Rc<Cell<bool>>,
}

impl Platform for Headless {
    /// Create a UI-thread adapter; tests neither draw it nor start an event loop.
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer))
    }

    /// Observe the actual declarative fixed target with no network/native effect.
    fn open_url(&self, url: &str) -> Result<(), PlatformError> {
        self.urls.borrow_mut().push(url.to_owned());
        if self.browser_available.get() {
            Ok(())
        } else {
            Err(PlatformError::from("Test platform browser is unavailable"))
        }
    }
}

/// Real callbacks preserve checked data, drafts, observable updates and teardown.
///
/// A single test owns platform initialization on its test thread, as Slint permits
/// one platform per thread. Each helper uses the same generated fixture root.
///
/// # Errors
/// Propagates platform initialization or checked host creation failure.
///
/// # Panics
/// Fails when the independently specified callback/lifecycle contract is broken.
#[test]
fn generated_callbacks_preserve_checked_state_drafts_and_lifetime() -> Result<(), Box<dyn Error>> {
    let urls = Rc::new(RefCell::new(Vec::new()));
    let browser_available = Rc::new(Cell::new(false));
    slint::platform::set_platform(Box::new(Headless {
        urls: Rc::clone(&urls),
        browser_available: Rc::clone(&browser_available),
    }))?;
    let host = slint_host::create()?;
    let window = host.window();

    check_initial_state_and_closed_editor_guard(window);
    check_mode_navigation_and_active_draft_guard(window);
    check_signed_selection_and_cancel(window);
    check_accepted_update_notifies_observers(window);
    check_rejected_updates_and_healthy_neighbor(window);
    check_reentrant_model_notification(window);
    check_preferences_and_draft_retention(window);
    check_source_workflow_callbacks(window);
    // A fixed URL still needs the actual About context before either outcome.
    let requests_before = urls.borrow().len();
    assert!(!window.invoke_repository_requested());
    assert_eq!(urls.borrow().len(), requests_before);
    window.invoke_navigate(Page::About);
    // Both outcomes traverse the real Slint action. Only this inert platform
    // records a request; true is not browser/page-view evidence.
    assert!(!window.invoke_repository_requested());
    browser_available.set(true);
    assert!(window.invoke_repository_requested());
    assert_eq!(
        urls.borrow().as_slice(),
        [
            "https://github.com/DavidOsipov/NevroTick",
            "https://github.com/DavidOsipov/NevroTick",
        ]
    );

    let weak = window.as_weak();
    drop(host);
    assert!(weak.upgrade().is_none());

    let restarted = slint_host::create()?;
    let rows = accepted_rows(restarted.window());
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].label, "Sample North");
    assert_eq!(rows[0].host, "north.example.invalid");
    assert_eq!(rows[0].port, "4460");
    assert_eq!(restarted.window().get_selected_source(), 1);
    assert!(!restarted.window().get_editor_open());
    assert_eq!(
        restarted.window().get_accepted_interval_label(),
        "Automatic"
    );
    assert!(!restarted.window().get_tray_enabled());
    assert!(!restarted.window().get_sign_in_enabled());
    assert!(restarted.window().get_notifications_enabled());
    let weak = restarted.window().as_weak();
    drop(restarted);
    assert!(weak.upgrade().is_none());
    Ok(())
}

/// Preference callbacks validate receiving state, preserve drafts and reset locally.
///
/// # Panics
/// Fails if malformed input mutates accepted values, navigation loses raw text,
/// or a spoofed view tray flag bypasses the accepted sign-in prerequisite.
fn check_preferences_and_draft_retention(window: &AppWindow) {
    window.invoke_navigate(Page::Settings);
    assert_eq!(window.get_accepted_interval_label(), "Automatic");
    edit_choice(window, 0, 4);
    edit_text(window, 4, "0007");
    for page in [Page::About, Page::Sources, Page::Settings] {
        window.invoke_navigate(page);
        assert_eq!(window.get_draft_custom_minutes(), "0007");
    }
    window.invoke_mode_requested(false);
    window.invoke_mode_requested(true);
    assert_eq!(window.get_draft_custom_minutes(), "0007");
    window.invoke_interval_save_requested(4, "0007".into());
    assert_eq!(
        window.get_accepted_interval_label(),
        "Every 7 minutes (custom)"
    );
    assert_eq!(window.get_draft_custom_minutes(), "7");
    for raw in ["0", "-1", "1.5", " 2 ", "1e2", "18446744073709551616"] {
        edit_text(window, 4, raw);
        window.invoke_interval_save_requested(4, raw.into());
        assert_eq!(
            window.get_accepted_interval_label(),
            "Every 7 minutes (custom)"
        );
        assert_eq!(window.get_draft_custom_minutes(), raw);
        assert!(!window.get_interval_error().is_empty());
        let error = window.get_interval_error();
        window.invoke_settings_section_requested(2);
        window.invoke_notifications_requested(false);
        assert_eq!(
            window.get_preference_feedback(),
            "Notification preference updated for this preview session only."
        );
        assert_eq!(window.get_interval_error(), error);
        assert_eq!(window.get_draft_custom_minutes(), raw);
        window.invoke_settings_section_requested(0);
    }
    // Forbidden repertoire never replaces the bounded raw draft; the direct
    // Submit receiver also requires the exact authoritative raw snapshot.
    let prior_raw = window.get_draft_custom_minutes();
    assert_eq!(
        window.invoke_raw_text_edited(4, "\u{0661}".into()),
        prior_raw
    );
    window.invoke_interval_save_requested(4, "\u{0661}".into());
    assert_eq!(window.get_draft_custom_minutes(), prior_raw);
    assert_eq!(
        window.get_accepted_interval_label(),
        "Every 7 minutes (custom)"
    );
    assert_eq!(window.get_interval_error(), RAW_STALE);
    for choice in [i32::MIN, -1, 5, i32::MAX] {
        window.invoke_interval_save_requested(choice, "1".into());
        assert_eq!(
            window.get_accepted_interval_label(),
            "Every 7 minutes (custom)"
        );
        assert_eq!(window.get_interval_error(), RAW_STALE);
    }
    // Well-formed values still cannot submit a different current snapshot.
    edit_text(window, 4, "8");
    window.invoke_interval_save_requested(4, "9".into());
    assert_eq!(window.get_interval_error(), RAW_STALE);
    assert_eq!(window.get_draft_custom_minutes(), "8");
    assert_eq!(
        window.get_accepted_interval_label(),
        "Every 7 minutes (custom)"
    );
    window.invoke_interval_cancel_requested();
    assert_eq!(window.get_draft_interval_choice(), 4);
    assert_eq!(window.get_draft_custom_minutes(), "7");
    edit_choice(window, 0, 2);
    window.invoke_interval_save_requested(2, "ignored for preset".into());
    assert_eq!(window.get_accepted_interval_label(), "Every 5 minutes");
    assert_eq!(window.get_draft_custom_minutes(), "7");
    edit_choice(window, 0, 4);
    edit_text(window, 4, "1");
    window.invoke_interval_save_requested(4, "1".into());
    assert_eq!(
        window.get_accepted_interval_label(),
        "Every 1 minute (custom)"
    );
    assert_eq!(window.get_interval_error(), "");

    check_tray_and_notification_preferences(window);
}

/// Shared preferences keep the accepted tray prerequisite and independent notices.
///
/// # Panics
/// Fails on view-property authority, lost sign-in prerequisite or coupled notices.
fn check_tray_and_notification_preferences(window: &AppWindow) {
    window.invoke_settings_section_requested(4);
    // Setting an affordance directly never changes the Rust-owned prerequisite.
    window.set_tray_enabled(true);
    window.invoke_sign_in_requested(true);
    assert!(!window.get_tray_enabled());
    assert!(!window.get_sign_in_enabled());
    assert!(!window.get_preference_error().is_empty());
    window.invoke_tray_requested(true);
    assert_eq!(
        window.get_preference_feedback(),
        "Tray preference updated for this preview session only."
    );
    window.invoke_sign_in_requested(true);
    assert_eq!(
        window.get_preference_feedback(),
        "Sign-in preference updated for this preview session only."
    );
    assert!(window.get_tray_enabled());
    assert!(window.get_sign_in_enabled());
    window.invoke_settings_section_requested(2);
    window.invoke_notifications_requested(false);
    assert!(!window.get_notifications_enabled());
    assert!(window.get_sign_in_enabled());
    window.invoke_settings_section_requested(4);
    window.invoke_tray_requested(false);
    assert!(!window.get_tray_enabled());
    assert!(!window.get_sign_in_enabled());
    assert!(!window.get_notifications_enabled());
}

/// A synchronous observable-model consumer cannot reopen an in-flight editor.
///
/// # Panics
/// Fails when a nested Edit leaves private ownership out of sync with the view.
fn check_reentrant_model_notification(window: &AppWindow) {
    let armed = Rc::new(Cell::new(false));
    let attempted = Rc::new(Cell::new(false));
    let trigger = armed.clone();
    let callback_attempted = attempted.clone();
    let weak = window.as_weak();
    let observer = FilterModel::new(window.get_sources(), move |_: &SourceRow| {
        if trigger.replace(false) {
            callback_attempted.set(true);
            if let Some(window) = weak.upgrade() {
                window.invoke_edit_requested();
            }
        }
        true
    });
    assert_eq!(observer.row_count(), 3);
    window.invoke_source_selected(1);
    window.invoke_edit_requested();
    armed.set(true);
    source_input(window, "Nested edit control", "192.0.2.22", 0, "4460");
    window.invoke_update_requested(
        "Nested edit control".into(),
        "192.0.2.22".into(),
        0,
        "4460".into(),
    );
    assert!(attempted.get());
    assert!(!window.get_editor_open());
    window.invoke_source_selected(2);
    assert_eq!(window.get_selected_source(), 2);
    assert_eq!(window.get_editor_error(), "");
    window.invoke_edit_requested();
    assert!(window.get_editor_open());
    window.invoke_cancel_requested();
    assert!(!window.get_editor_open());
}

/// Read presentation rows through the generated model, without accepting edits.
fn accepted_rows(window: &AppWindow) -> Vec<SourceRow> {
    window.get_sources().iter().collect()
}

/// Send ordinary bounded raw input through its actual receiving callback.
///
/// # Panics
/// Fails if this fixture setup is rejected or mutates its requested bytes.
fn edit_text(window: &AppWindow, field: i32, value: &str) {
    assert_eq!(window.invoke_raw_text_edited(field, value.into()), value);
}

/// Select one code-owned raw option without trusting a writable view property.
///
/// # Panics
/// Fails if this fixture setup is rejected or silently selects another option.
fn edit_choice(window: &AppWindow, field: i32, value: i32) {
    assert_eq!(window.invoke_raw_choice_edited(field, value), value);
}

/// Enter current source-editor raw fields without changing accepted sources.
///
/// # Panics
/// Fails when this fixture's source editor/context is not actually active.
fn set_drafts(window: &AppWindow, label: &str, host: &str, port: &str) {
    edit_text(window, 0, label);
    edit_text(window, 1, host);
    edit_text(window, 2, port);
}

/// Prepare a visible source editor through the real raw receivers.
///
/// # Panics
/// Fails if ordinary fixture bytes or its explicit protocol are not admitted.
fn source_input(window: &AppWindow, label: &str, host: &str, protocol: i32, port: &str) {
    edit_choice(window, 2, protocol);
    set_drafts(window, label, host, port);
}

/// Compare all raw draft fields without normalizing their original bytes.
///
/// # Panics
/// Fails when any current raw draft differs from the expected exact string.
fn assert_drafts(window: &AppWindow, label: &str, host: &str, port: &str) {
    assert_eq!(window.get_draft_label(), label);
    assert_eq!(window.get_draft_host(), host);
    assert_eq!(window.get_draft_port(), port);
}

/// A ready fixture rejects updates unless its private editor lifecycle is active.
///
/// # Panics
/// Fails when initialization, the editor guard or raw-draft retention is broken.
fn check_initial_state_and_closed_editor_guard(window: &AppWindow) {
    assert!(window.get_host_ready());
    assert_eq!(window.get_current_page(), Page::Status);
    assert!(!window.get_power_mode());
    assert!(!window.get_editor_open());
    assert_eq!(window.get_selected_source(), 1);
    let before = accepted_rows(window);
    assert_eq!(before.len(), 3);
    assert_eq!(before[0].label, "Sample North");
    assert_eq!(before[1].host, "south.example.invalid");
    assert_eq!(before[2].port, "4460");
    assert_drafts(window, "Sample North", "north.example.invalid", "4460");

    // Direct view setters are intentionally hostile; they own no raw buffer.
    window.set_draft_label("Unsubmitted".into());
    window.set_draft_host("draft.example.invalid".into());
    window.set_draft_port("1234".into());
    assert_eq!(
        window.invoke_raw_text_edited(0, "Unsubmitted".into()),
        "Sample North"
    );
    // Directly setting the view affordance cannot activate the receiving guard.
    window.set_editor_open(true);
    window.invoke_update_requested("Bypass".into(), "192.0.2.1".into(), 0, "1234".into());
    assert_eq!(accepted_rows(window), before);
    assert_eq!(
        window.get_editor_error(),
        "Open Sources in Power User mode before changing source previews."
    );
    assert_drafts(window, "Sample North", "north.example.invalid", "4460");
    window.set_editor_open(false);

    window.invoke_navigate(Page::Sources);
    assert_eq!(window.get_current_page(), Page::Status);
    assert_eq!(window.get_editor_error(), UiError::PageUnavailable.text());
    assert_drafts(window, "Sample North", "north.example.invalid", "4460");

    // At the reached route the independent trusted editor phase still rejects.
    window.invoke_mode_requested(true);
    window.invoke_navigate(Page::Sources);
    window.set_editor_open(true);
    window.invoke_update_requested("Bypass".into(), "192.0.2.1".into(), 0, "1234".into());
    assert_eq!(accepted_rows(window), before);
    assert_eq!(
        window.get_editor_error(),
        "Open the sample source editor before updating its preview."
    );
    assert_drafts(window, "Sample North", "north.example.invalid", "4460");
    window.set_editor_open(false);
    window.invoke_navigate(Page::Status);
    window.invoke_mode_requested(false);
}

/// Navigation/mode switches retain the inline edit, and selection cannot erase it.
///
/// # Panics
/// Fails when navigation, mode transitions or selection lose active editor data.
fn check_mode_navigation_and_active_draft_guard(window: &AppWindow) {
    window.invoke_mode_requested(true);
    window.invoke_navigate(Page::Sources);
    window.invoke_edit_requested();
    assert!(window.get_editor_open());
    assert_drafts(window, "Sample North", "north.example.invalid", "4460");
    set_drafts(window, "Raw draft", "draft.example.invalid", "bad port");
    let before = accepted_rows(window);

    window.invoke_edit_requested();
    for page in [
        Page::Statistics,
        Page::Diagnostics,
        Page::Settings,
        Page::About,
        Page::Sources,
    ] {
        window.invoke_navigate(page);
        assert_eq!(window.get_current_page(), page);
        assert_drafts(window, "Raw draft", "draft.example.invalid", "bad port");
    }
    window.invoke_mode_requested(false);
    assert_eq!(window.get_current_page(), Page::Status);
    assert!(!window.get_power_mode());
    window.invoke_mode_requested(true);
    assert_eq!(window.get_current_page(), Page::Sources);
    assert!(window.get_power_mode());
    assert!(window.get_editor_open());

    // Hiding the view's editor property cannot release the Rust lifecycle guard.
    window.set_editor_open(false);
    for view_id in [2, 3, -1, i32::MAX] {
        window.invoke_source_selected(view_id);
        assert_eq!(window.get_selected_source(), 1);
        assert_eq!(accepted_rows(window), before);
        assert_drafts(window, "Raw draft", "draft.example.invalid", "bad port");
    }
    window.invoke_cancel_requested();
    assert!(!window.get_editor_open());
    assert_drafts(window, "Sample North", "north.example.invalid", "4460");
    assert_eq!(window.get_editor_error(), "");

    for shared in [Page::Settings, Page::About] {
        window.invoke_navigate(shared);
        window.invoke_mode_requested(false);
        assert_eq!(window.get_current_page(), shared);
        window.invoke_mode_requested(true);
        assert_eq!(window.get_current_page(), shared);
    }
    window.invoke_navigate(Page::Sources);
}

/// Invalid signed stable IDs reject atomically; valid selection/Cancel reset.
/// Display row positions remain unrelated to callback source authority.
///
/// # Panics
/// Fails when selection rejection mutates state or Cancel fails to restore it.
fn check_signed_selection_and_cancel(window: &AppWindow) {
    let before = accepted_rows(window);
    assert_eq!(
        window.invoke_raw_text_edited(0, "Hidden draft".into()),
        "Sample North"
    );
    for view_id in [0, -1, i32::MIN, 4, i32::MAX] {
        window.invoke_source_selected(view_id);
        assert_eq!(window.get_selected_source(), 1);
        assert_eq!(accepted_rows(window), before);
        assert_eq!(
            window.get_editor_error(),
            SourceDraftError::UnknownSource.to_string()
        );
        assert_drafts(window, "Sample North", "north.example.invalid", "4460");
    }

    window.invoke_source_selected(3);
    assert_eq!(window.get_selected_source(), 3);
    assert_eq!(window.get_editor_error(), "");
    assert_drafts(window, "Sample West", "west.example.invalid", "4460");
    window.invoke_edit_requested();
    set_drafts(window, "Discard me", "https://unaccepted.invalid", "0");
    window.invoke_cancel_requested();
    assert!(!window.get_editor_open());
    assert_eq!(window.get_selected_source(), 3);
    assert_eq!(accepted_rows(window), before);
    assert_drafts(window, "Sample West", "west.example.invalid", "4460");
    assert_eq!(
        window.get_editor_feedback(),
        "Unaccepted editor changes discarded."
    );
}

/// A successful real callback changes only its checked row and notifies observers.
///
/// # Panics
/// Fails when an accepted update is missing, changes a neighbor or is unobserved.
fn check_accepted_update_notifies_observers(window: &AppWindow) {
    window.invoke_source_selected(1);
    let before = accepted_rows(window);
    let observer = FilterModel::new(window.get_sources(), |row: &SourceRow| {
        row.label == "Sample North"
    });
    assert_eq!(observer.row_count(), 1);
    window.invoke_edit_requested();
    set_drafts(window, "Accepted edit", "192.0.2.25", "1234");
    window.invoke_update_requested(
        "Accepted edit".into(),
        "192.0.2.25".into(),
        0,
        "1234".into(),
    );

    let after = accepted_rows(window);
    assert_eq!(after[0].label, "Accepted edit");
    assert_eq!(after[0].host, "192.0.2.25");
    assert_eq!(after[0].port, "1234");
    assert_eq!(after[1..], before[1..]);
    // FilterModel caches its row mapping and changes it through notifications.
    assert_eq!(observer.row_count(), 0);
    assert_eq!(window.get_selected_source(), 1);
    assert!(!window.get_editor_open());
    assert_eq!(window.get_editor_error(), "");
    assert_drafts(window, "Accepted edit", "192.0.2.25", "1234");
    assert_eq!(
        window.get_editor_feedback(),
        "Preview updated in memory only. The active fixture and service configuration are unchanged."
    );
}

/// Malformed real callback input retains rows, selection and raw drafts exactly.
///
/// # Panics
/// Fails when malformed input changes data or the healthy neighboring edit fails.
fn check_rejected_updates_and_healthy_neighbor(window: &AppWindow) {
    let before = accepted_rows(window);
    window.invoke_edit_requested();
    check_untrusted_source_inputs(window, &before);
    let changes = window.get_source_changes().iter().collect::<Vec<_>>();
    source_input(window, "Raw owner", "raw-owner.example.invalid", 0, "4460");
    window.invoke_update_requested(
        "Different valid".into(),
        "different.example.invalid".into(),
        0,
        "4460".into(),
    );
    assert_eq!(window.get_editor_error(), RAW_STALE);
    assert_eq!(
        window.get_source_changes().iter().collect::<Vec<_>>(),
        changes
    );
    assert_eq!(accepted_rows(window), before);
    assert_drafts(window, "Raw owner", "raw-owner.example.invalid", "4460");

    let observer = FilterModel::new(window.get_sources(), |row: &SourceRow| {
        row.label == "Sample North"
    });
    assert_eq!(observer.row_count(), 0);
    source_input(window, "Sample North", "2001:db8::1", 0, "65535");
    window.invoke_update_requested(
        "Sample North".into(),
        "2001:db8::1".into(),
        0,
        "65535".into(),
    );
    let after = accepted_rows(window);
    assert_eq!(after[0].label, "Sample North");
    assert_eq!(after[0].host, "2001:db8::1");
    assert_eq!(after[0].port, "65535");
    assert_eq!(after[1..], before[1..]);
    assert_eq!(observer.row_count(), 1);
    assert!(!window.get_editor_open());
    assert_drafts(window, "Sample North", "2001:db8::1", "65535");

    window.invoke_edit_requested();
    set_drafts(window, "Discard after success", "bad host", "0");
    window.invoke_cancel_requested();
    assert_eq!(accepted_rows(window), after);
    assert_drafts(window, "Sample North", "2001:db8::1", "65535");
    assert!(!window.get_editor_open());
}

/// Challenge bounded/raw-invalid inputs and the direct Submit boundary separately.
/// Caller retains a real active Edit target and the unchanged accepted rows.
///
/// # Panics
/// Fails if raw rejection is confused with direct domain admission or leaks data.
fn check_untrusted_source_inputs(window: &AppWindow, before: &[SourceRow]) {
    let oversize = "private".repeat(1000);
    for (label, host, port, error, raw_label_allowed) in [
        (
            "private\nlabel",
            "192.0.2.25",
            "1234",
            ModelError::InvalidLabel,
            false,
        ),
        (
            "Accepted edit",
            "https://private.invalid",
            "1234",
            ModelError::InvalidHost,
            true,
        ),
        (
            "Accepted edit",
            "192.0.2.25",
            "65536",
            ModelError::InvalidPort,
            true,
        ),
        (
            "Accepted edit",
            "192.0.2.25",
            "+123",
            ModelError::InvalidPort,
            true,
        ),
        (
            oversize.as_str(),
            "192.0.2.25",
            "1234",
            ModelError::InvalidLabel,
            false,
        ),
    ] {
        let prior_label = window.get_draft_label();
        let expected_label = if raw_label_allowed {
            edit_text(window, 0, label);
            label.into()
        } else {
            assert_eq!(window.invoke_raw_text_edited(0, label.into()), prior_label);
            prior_label
        };
        edit_text(window, 1, host);
        edit_text(window, 2, port);
        window.invoke_update_requested(label.into(), host.into(), 0, port.into());
        assert_eq!(accepted_rows(window), before);
        assert_eq!(window.get_selected_source(), 1);
        assert!(window.get_editor_open());
        let expected = if raw_label_allowed {
            SourceDraftError::Field(error).to_string()
        } else {
            RAW_STALE.to_owned()
        };
        assert_eq!(window.get_editor_error(), expected);
        assert!(!window.get_editor_error().contains("private"));
        assert_eq!(window.get_editor_feedback(), "");
        assert_drafts(window, &expected_label, host, port);
    }
}

/// Open the visible review before discarding this fixture's pending source deltas.
///
/// # Panics
/// Fails if a nonempty fixture cannot reach review or discard its own local deltas.
fn discard_previews(window: &AppWindow) {
    if window.get_source_changes().row_count() > 0 {
        window.invoke_review_requested();
        assert!(window.get_review_open());
        window.invoke_discard_requested();
        assert_eq!(window.get_source_changes().row_count(), 0);
    }
}

/// New source callbacks preserve fixture/personal-state boundaries end to end.
///
/// # Panics
/// Fails when draft workflows change preferences, lose IDs or claim active policy.
fn check_source_workflow_callbacks(window: &AppWindow) {
    window.invoke_navigate(Page::Sources);
    let interval = window.get_accepted_interval_label();
    let custom = window.get_draft_custom_minutes();
    let notifications = window.get_notifications_enabled();
    let tray = window.get_tray_enabled();
    let sign_in = window.get_sign_in_enabled();
    discard_previews(window);
    let baseline = accepted_rows(window);
    assert_eq!(baseline.len(), 3);
    assert_eq!(
        baseline.iter().map(|row| row.view_id).collect::<Vec<_>>(),
        [1, 2, 3]
    );
    assert!(
        baseline
            .iter()
            .all(|row| row.protocol_choice == 0 && !row.show_draft_badge)
    );
    assert_eq!(window.get_source_changes().row_count(), 0);
    check_explicit_protocol_and_port_provenance(window);
    check_duplicate_and_protocol_receivers(window);
    check_stable_ids_remove_undo_and_review(window);
    check_active_editor_blocks_source_mutations(window);
    check_editor_target_cannot_be_spoofed_by_view_flags(window);
    check_undo_identity_conflict_and_capacity(window);
    check_empty_sources_and_stale_addition_ids(window);
    assert_eq!(window.get_accepted_interval_label(), interval);
    assert_eq!(window.get_draft_custom_minutes(), custom);
    assert_eq!(window.get_notifications_enabled(), notifications);
    assert_eq!(window.get_tray_enabled(), tray);
    assert_eq!(window.get_sign_in_enabled(), sign_in);
    assert_eq!(accepted_rows(window), baseline);
}

/// Port provenance belongs to the host, including a manually invalid raw draft.
///
/// # Panics
/// Fails when protocol changes erase manual text or submit infers protocol/port.
fn check_explicit_protocol_and_port_provenance(window: &AppWindow) {
    let before = accepted_rows(window);
    window.invoke_add_requested();
    assert!(window.get_editor_open());
    assert!(window.get_editor_adding());
    assert_eq!(window.get_draft_protocol_choice(), 0);
    assert_eq!(window.get_draft_port(), "4460");
    window.invoke_protocol_requested(1);
    assert_eq!(window.get_draft_protocol_choice(), 1);
    assert_eq!(window.get_draft_port(), "123");
    window.invoke_protocol_requested(0);
    assert_eq!(window.get_draft_protocol_choice(), 0);
    assert_eq!(window.get_draft_port(), "4460");
    set_drafts(window, "Added NTP", "added.example.invalid", "invalid");
    window.invoke_port_edited();
    for invalid in [i32::MIN, -1, 2, i32::MAX] {
        window.invoke_protocol_requested(invalid);
        assert_eq!(window.get_draft_protocol_choice(), 0);
        assert_drafts(window, "Added NTP", "added.example.invalid", "invalid");
        assert_eq!(
            window.get_editor_error(),
            "Choose an available editor or option. No draft was changed."
        );
    }
    window.invoke_protocol_requested(1);
    assert_eq!(window.get_draft_protocol_choice(), 1);
    assert_drafts(window, "Added NTP", "added.example.invalid", "invalid");
    window.invoke_update_requested(
        "Added NTP".into(),
        "added.example.invalid".into(),
        1,
        "invalid".into(),
    );
    assert_eq!(accepted_rows(window), before);
    assert_eq!(window.get_source_changes().row_count(), 0);
    assert!(window.get_editor_open());
    assert_eq!(
        window.get_editor_error(),
        SourceDraftError::Field(ModelError::InvalidPort).to_string()
    );
    edit_text(window, 2, "4460");
    window.invoke_port_edited();
    window.invoke_protocol_requested(0);
    window.invoke_protocol_requested(1);
    assert_eq!(window.get_draft_port(), "4460");
    window.invoke_update_requested(
        "Added NTP".into(),
        "added.example.invalid".into(),
        1,
        "4460".into(),
    );
    let after = accepted_rows(window);
    assert_eq!(after.len(), 4);
    assert_eq!(after[..3], before[..]);
    assert!(after[3].view_id > 3);
    assert_eq!(after[3].protocol_choice, 1);
    assert_eq!(after[3].protocol_label, "Plain NTP preview");
    assert_eq!(after[3].port_label, "NTP port");
    assert_eq!(after[3].port, "4460");
    assert!(after[3].show_draft_badge);
    assert_eq!(after[3].draft_kind, "Add");
    assert!(!window.get_editor_open());
    assert_eq!(window.get_source_changes().row_count(), 1);
    assert_eq!(
        window.get_editor_feedback(),
        "Preview updated in memory only. The active fixture and service configuration are unchanged."
    );
}

/// Direct raw protocol/identity checks cannot be replaced by widget validation.
///
/// # Panics
/// Fails when a duplicate spelling, raw invalid protocol or port changes old data.
fn check_duplicate_and_protocol_receivers(window: &AppWindow) {
    let before = accepted_rows(window);
    let changes = window.get_source_changes().iter().collect::<Vec<_>>();
    window.invoke_add_requested();
    for (label, host, choice, port, expected) in [
        (
            "Other label",
            "ADDED.EXAMPLE.INVALID.",
            1,
            "123",
            SourceDraftError::DuplicateIdentity,
        ),
        (
            "NTS duplicate",
            "NORTH.EXAMPLE.INVALID.",
            0,
            "1234",
            SourceDraftError::DuplicateIdentity,
        ),
        (
            "Unknown",
            "unique.example.invalid",
            -1,
            "4460",
            SourceDraftError::InvalidProtocol,
        ),
        (
            "Unknown",
            "unique.example.invalid",
            2,
            "4460",
            SourceDraftError::InvalidProtocol,
        ),
        (
            "Unknown",
            "unique.example.invalid",
            i32::MAX,
            "4460",
            SourceDraftError::InvalidProtocol,
        ),
        (
            "Numeric",
            "999.999.999.999",
            0,
            "4460",
            SourceDraftError::Field(ModelError::InvalidHost),
        ),
        (
            "Numeric",
            "192.000.2.1",
            0,
            "4460",
            SourceDraftError::Field(ModelError::InvalidHost),
        ),
        (
            "Repeated root",
            "unique.example.invalid..",
            0,
            "4460",
            SourceDraftError::Field(ModelError::InvalidHost),
        ),
    ] {
        set_drafts(window, label, host, port);
        let prior_protocol = window.get_draft_protocol_choice();
        window.set_draft_protocol_choice(choice);
        let returned_protocol = window.invoke_raw_choice_edited(2, choice);
        if matches!(choice, 0 | 1) {
            assert_eq!(returned_protocol, choice);
        } else {
            assert_eq!(returned_protocol, prior_protocol);
        }
        window.invoke_update_requested(label.into(), host.into(), choice, port.into());
        assert_eq!(accepted_rows(window), before);
        assert_eq!(
            window.get_source_changes().iter().collect::<Vec<_>>(),
            changes
        );
        assert!(window.get_editor_open());
        let expected = if matches!(choice, 0 | 1) {
            expected.to_string()
        } else {
            RAW_STALE.to_owned()
        };
        assert_eq!(window.get_editor_error(), expected);
        assert_drafts(window, label, host, port);
        assert_eq!(window.get_draft_protocol_choice(), returned_protocol);
    }
    check_raw_length_and_ipv6_receivers(window, &before);
}

/// Continue an active Add draft through length/IPv6 rejection and valid neighbors.
///
/// # Panics
/// Fails when normalization bypasses raw bounds or IP byte equality is ignored.
fn check_raw_length_and_ipv6_receivers(window: &AppWindow, before: &[SourceRow]) {
    let changes = window.get_source_changes().iter().collect::<Vec<_>>();
    let maximal_host = format!(
        "{}.{}.{}.{}.",
        "a".repeat(63),
        "b".repeat(63),
        "c".repeat(63),
        "d".repeat(61)
    );
    assert_eq!(maximal_host.len(), 254);
    set_drafts(window, "Length bypass", &maximal_host, "4460");
    window.invoke_update_requested(
        "Length bypass".into(),
        maximal_host.clone().into(),
        0,
        "4460".into(),
    );
    assert_eq!(accepted_rows(window), before);
    assert_eq!(
        window.get_source_changes().iter().collect::<Vec<_>>(),
        changes
    );
    assert_eq!(
        window.get_editor_error(),
        SourceDraftError::Field(ModelError::InvalidHost).to_string()
    );
    assert_drafts(window, "Length bypass", &maximal_host, "4460");
    source_input(window, "IPv6 neighbor", "2001:db8::1", 0, "4460");
    window.invoke_update_requested(
        "IPv6 neighbor".into(),
        "2001:db8::1".into(),
        0,
        "4460".into(),
    );
    let ipv6 = accepted_rows(window);
    assert_eq!(ipv6.len(), before.len() + 1);
    window.invoke_add_requested();
    set_drafts(window, "Equivalent IPv6", "2001:0DB8:0:0:0:0:0:1", "9999");
    window.invoke_update_requested(
        "Equivalent IPv6".into(),
        "2001:0DB8:0:0:0:0:0:1".into(),
        0,
        "9999".into(),
    );
    assert_eq!(accepted_rows(window), ipv6);
    assert_eq!(
        window.get_editor_error(),
        SourceDraftError::DuplicateIdentity.to_string()
    );
    assert_drafts(window, "Equivalent IPv6", "2001:0DB8:0:0:0:0:0:1", "9999");
    // The same DNS identity can be an explicitly chosen NTP preview; no downgrade
    // or authentication fact is inferred from the NTS source beside it.
    source_input(
        window,
        "Explicit NTP neighbor",
        "north.example.invalid",
        1,
        "123",
    );
    window.invoke_update_requested(
        "Explicit NTP neighbor".into(),
        "north.example.invalid".into(),
        1,
        "123".into(),
    );
    let after = accepted_rows(window);
    assert_eq!(after.len(), ipv6.len() + 1);
    assert_eq!(after[after.len() - 1].protocol_choice, 1);
    assert!(after[after.len() - 1].show_draft_badge);
    assert_eq!(after[after.len() - 1].draft_kind, "Add");
    assert!(!window.get_editor_open());
}

/// Structural changes use source IDs, not current row indices, for authority.
///
/// # Panics
/// Fails when selection aliases a shifted row or review/Undo loses source history.
fn check_stable_ids_remove_undo_and_review(window: &AppWindow) {
    let before = accepted_rows(window);
    let added_id = before[3].view_id;
    window.invoke_remove_requested(2);
    let removed = accepted_rows(window);
    assert_eq!(removed.len(), before.len() - 1);
    assert_eq!(removed[1].view_id, 3);
    window.invoke_source_selected(3);
    assert_eq!(window.get_selected_source(), 3);
    assert_eq!(window.get_draft_label(), "Sample West");
    let changes = window.get_source_changes().iter().collect::<Vec<_>>();
    assert!(
        changes
            .iter()
            .any(|change| change.view_id == 2 && change.kind == "Remove")
    );
    window.invoke_review_requested();
    assert!(window.get_review_open());
    assert_eq!(accepted_rows(window), removed);
    assert_eq!(
        window.get_source_changes().iter().collect::<Vec<_>>(),
        changes
    );
    window.invoke_review_close_requested();
    assert!(!window.get_review_open());
    window.invoke_review_requested();
    assert!(window.get_review_open());
    window.invoke_undo_requested(2);
    assert_eq!(accepted_rows(window), before);
    assert_eq!(window.get_selected_source(), 3);
    let restored = accepted_rows(window);
    assert_eq!(restored[1].view_id, 2);
    assert!(!restored[1].show_draft_badge);
    assert_eq!(restored[1].draft_kind, "Baseline");
    let before = accepted_rows(window);
    window.invoke_undo_requested(i32::MAX);
    assert_eq!(accepted_rows(window), before);
    assert_eq!(
        window.get_editor_error(),
        SourceDraftError::UnknownSource.to_string()
    );
    window.invoke_review_close_requested();
    window.invoke_remove_requested(added_id);
    let removed_add = accepted_rows(window);
    assert_eq!(removed_add.len(), before.len() - 1);
    window.invoke_source_selected(added_id);
    assert_eq!(accepted_rows(window), removed_add);
    assert_eq!(
        window.get_editor_error(),
        SourceDraftError::UnknownSource.to_string()
    );
    window.invoke_add_requested();
    source_input(window, "New ID", "fresh-id.example.invalid", 0, "4460");
    window.invoke_update_requested(
        "New ID".into(),
        "fresh-id.example.invalid".into(),
        0,
        "4460".into(),
    );
    let after = accepted_rows(window);
    let newest = after[after.len() - 1].view_id;
    assert!(newest > added_id);
    assert!(newest > before[before.len() - 1].view_id);
    window.invoke_review_close_requested();
    assert!(!window.get_review_open());
}

/// Hiding the editor property never releases the receiver's active target guard.
///
/// # Panics
/// Fails when a direct mutation erases an unsubmitted draft or source review.
fn check_active_editor_blocks_source_mutations(window: &AppWindow) {
    window.invoke_source_selected(1);
    window.invoke_edit_requested();
    set_drafts(window, "Keep pending raw", " raw host ", "bad port");
    window.set_draft_protocol_choice(-1);
    assert_eq!(window.invoke_raw_choice_edited(2, -1), 0);
    let before = accepted_rows(window);
    let changes = window.get_source_changes().iter().collect::<Vec<_>>();
    let probes: [fn(&AppWindow); 5] = [
        |window| window.invoke_remove_requested(1),
        |window| window.invoke_undo_requested(1),
        AppWindow::invoke_discard_requested,
        AppWindow::invoke_review_requested,
        AppWindow::invoke_add_requested,
    ];
    for probe in probes {
        window.set_editor_open(false);
        probe(window);
        assert_eq!(accepted_rows(window), before);
        assert_eq!(
            window.get_source_changes().iter().collect::<Vec<_>>(),
            changes
        );
        assert_eq!(window.get_selected_source(), 1);
        assert_drafts(window, "Keep pending raw", " raw host ", "bad port");
        assert_eq!(window.get_draft_protocol_choice(), 0);
        assert!(!window.get_review_open());
    }
    window.invoke_cancel_requested();
    assert!(!window.get_editor_open());
    assert_drafts(window, "Sample North", "north.example.invalid", "4460");
    assert_eq!(window.get_draft_protocol_choice(), 0);
    assert_eq!(accepted_rows(window), before);
}

/// View flags and a selected-ID projection cannot change the private edit target.
///
/// # Panics
/// Fails when a direct property write redirects Edit to another source or Add.
fn check_editor_target_cannot_be_spoofed_by_view_flags(window: &AppWindow) {
    window.invoke_source_selected(1);
    let before = accepted_rows(window);
    window.invoke_edit_requested();
    source_input(
        window,
        "Trusted target",
        "trusted-target.example.invalid",
        0,
        "4460",
    );
    window.set_editor_adding(true);
    window.set_selected_source(2);
    window.invoke_update_requested(
        "Trusted target".into(),
        "trusted-target.example.invalid".into(),
        0,
        "4460".into(),
    );
    let edited = accepted_rows(window);
    assert_eq!(edited.len(), before.len());
    assert_eq!(edited[0].view_id, 1);
    assert_eq!(edited[0].label, "Trusted target");
    assert_eq!(edited[1..], before[1..]);
    assert_eq!(window.get_selected_source(), 1);
    assert!(!window.get_editor_open());
    window.invoke_add_requested();
    source_input(
        window,
        "Trusted Add",
        "trusted-add.example.invalid",
        0,
        "4460",
    );
    window.set_editor_adding(false);
    window.set_selected_source(1);
    window.invoke_update_requested(
        "Trusted Add".into(),
        "trusted-add.example.invalid".into(),
        0,
        "4460".into(),
    );
    let added = accepted_rows(window);
    assert_eq!(added.len(), edited.len() + 1);
    assert_eq!(added[..edited.len()], edited[..]);
    assert_eq!(added[added.len() - 1].label, "Trusted Add");
    assert!(added[added.len() - 1].view_id > edited[edited.len() - 1].view_id);
    assert_eq!(window.get_selected_source(), added[added.len() - 1].view_id);
    assert!(!window.get_editor_open());
}

/// Undo conflict/capacity feedback preserves the current preview and local deltas.
///
/// # Panics
/// Fails when Undo overwrites a newer identity or direct Add bypasses source cap.
fn check_undo_identity_conflict_and_capacity(window: &AppWindow) {
    discard_previews(window);
    window.invoke_source_selected(1);
    window.invoke_edit_requested();
    source_input(window, "Moved North", "moved.example.invalid", 0, "4460");
    window.invoke_update_requested(
        "Moved North".into(),
        "moved.example.invalid".into(),
        0,
        "4460".into(),
    );
    window.invoke_add_requested();
    source_input(window, "Reused North", "NORTH.EXAMPLE.INVALID.", 0, "4460");
    window.invoke_update_requested(
        "Reused North".into(),
        "NORTH.EXAMPLE.INVALID.".into(),
        0,
        "4460".into(),
    );
    let before = accepted_rows(window);
    let changes = window.get_source_changes().iter().collect::<Vec<_>>();
    window.invoke_review_requested();
    assert!(window.get_review_open());
    window.invoke_undo_requested(1);
    assert_eq!(accepted_rows(window), before);
    assert_eq!(
        window.get_source_changes().iter().collect::<Vec<_>>(),
        changes
    );
    assert_eq!(
        window.get_editor_error(),
        SourceDraftError::DuplicateIdentity.to_string()
    );
    let added_id = before[before.len() - 1].view_id;
    window.invoke_undo_requested(added_id);
    window.invoke_undo_requested(1);
    assert_eq!(accepted_rows(window).len(), 3);
    assert_eq!(window.get_source_changes().row_count(), 0);
    window.invoke_review_close_requested();
    for index in 0..13 {
        let host = format!("bounded-{index}.example.invalid");
        window.invoke_add_requested();
        source_input(window, "Bounded source", &host, 0, "4460");
        window.invoke_update_requested("Bounded source".into(), host.into(), 0, "4460".into());
    }
    assert_eq!(accepted_rows(window).len(), 16);
    let before = accepted_rows(window);
    let changes = window.get_source_changes().iter().collect::<Vec<_>>();
    window.invoke_add_requested();
    source_input(window, "Overflow", "overflow.example.invalid", 0, "4460");
    window.invoke_update_requested(
        "Overflow".into(),
        "overflow.example.invalid".into(),
        0,
        "4460".into(),
    );
    assert_eq!(accepted_rows(window), before);
    assert_eq!(
        window.get_source_changes().iter().collect::<Vec<_>>(),
        changes
    );
    assert_eq!(
        window.get_editor_error(),
        SourceDraftError::Capacity.to_string()
    );
    window.invoke_cancel_requested();
    discard_previews(window);
    assert_eq!(accepted_rows(window).len(), 3);
}

/// Empty effective previews are safe, and old addition IDs never rebind to rows.
///
/// # Panics
/// Fails when empty selection aliases a row or discarded IDs are later reused.
fn check_empty_sources_and_stale_addition_ids(window: &AppWindow) {
    for view_id in [1, 2, 3] {
        window.invoke_remove_requested(view_id);
    }
    assert_eq!(accepted_rows(window), Vec::<SourceRow>::new());
    assert_eq!(window.get_selected_source(), 0);
    window.invoke_edit_requested();
    assert!(!window.get_editor_open());
    assert_eq!(
        window.get_editor_error(),
        SourceDraftError::UnknownSource.to_string()
    );
    window.invoke_add_requested();
    assert!(window.get_editor_open());
    source_input(window, "Only preview", "only.example.invalid", 1, "123");
    window.invoke_update_requested(
        "Only preview".into(),
        "only.example.invalid".into(),
        1,
        "123".into(),
    );
    let only = accepted_rows(window);
    assert_eq!(only.len(), 1);
    assert!(only[0].view_id > 3);
    let stale = only[0].view_id;
    assert_eq!(window.get_selected_source(), stale);
    discard_previews(window);
    assert_eq!(accepted_rows(window).len(), 3);
    assert_eq!(window.get_source_changes().row_count(), 0);
    let before = accepted_rows(window);
    let selected = window.get_selected_source();
    for invalid in [0, -1, i32::MIN, stale, i32::MAX] {
        window.invoke_source_selected(invalid);
        assert_eq!(accepted_rows(window), before);
        assert_eq!(window.get_selected_source(), selected);
        window.invoke_remove_requested(invalid);
        window.invoke_undo_requested(invalid);
        assert_eq!(accepted_rows(window), before);
        assert_eq!(window.get_source_changes().row_count(), 0);
    }
}

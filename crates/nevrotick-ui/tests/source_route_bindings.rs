//! Actual source callbacks must retain draft owners while their route is hidden.
#![forbid(unsafe_code)]

use std::{error::Error, rc::Rc};

use nevrotick_ui::slint_host::{self, AppWindow, Page, SourceChangeRow, SourceRow};
use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{Platform, PlatformError, WindowAdapter};
use slint::{ComponentHandle, Model};

/// Public software platform; this suite exercises no native/network/service APIs.
struct Headless;

impl Platform for Headless {
    /// Allocate only a thread-owned software adapter without showing a window.
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer))
    }
}

/// Named callback scenario with an explicit expected manual display lifetime.
struct CallbackCase {
    /// Independent case identity makes failures attributable before cleanup.
    name: &'static str,
    /// Actual generated callback scenario, sharing this fresh checked host only.
    run: fn(&AppWindow),
    /// True only for the case that shows Quit review and chooses Keep open.
    warning_remains_shown: bool,
}

/// New receiving checks distinguish real route admission from writable view flags.
///
/// # Errors
/// Propagates setup failure instead of reporting a zero-behavior successful run.
///
/// # Panics
/// Fails on offscreen source mutation, draft loss or blocked legal neighboring use.
#[test]
fn actual_source_route_preserves_hidden_draft_owners() -> Result<(), Box<dyn Error>> {
    slint::platform::set_platform(Box::new(Headless))?;
    let checks = [
        CallbackCase {
            name: "check_closed_editor_hidden_requests_preserve_all_source_state",
            run: check_closed_editor_hidden_requests_preserve_all_source_state,
            warning_remains_shown: false,
        },
        CallbackCase {
            name: "check_active_editor_navigation_preserves_input_and_port_provenance",
            run: check_active_editor_navigation_preserves_input_and_port_provenance,
            warning_remains_shown: false,
        },
        CallbackCase {
            name: "check_owned_modals_reject_spoofed_source_requests",
            run: check_owned_modals_reject_spoofed_source_requests,
            warning_remains_shown: true,
        },
        CallbackCase {
            name: "check_review_reveals_its_own_context",
            run: check_review_reveals_its_own_context,
            warning_remains_shown: false,
        },
        CallbackCase {
            name: "check_review_undo_conflict_feedback_and_healthy_recovery",
            run: check_review_undo_conflict_feedback_and_healthy_recovery,
            warning_remains_shown: false,
        },
    ];
    for case in checks {
        let host = slint_host::create()?;
        let window = host.window();
        assert!(!window.window().is_visible(), "{} started shown", case.name);
        (case.run)(window);
        assert_eq!(
            window.window().is_visible(),
            case.warning_remains_shown,
            "{} has an unexpected manual display lifetime",
            case.name,
        );
        if case.warning_remains_shown {
            assert!(
                !window.get_quit_warning_open(),
                "{} left Quit review open",
                case.name
            );
            assert!(
                !window.get_ui_exit_requested(),
                "{} requested app exit",
                case.name
            );
            // This named case chose Keep open after showing Quit review. Hiding
            // its manually displayed fixture is harness cleanup, not app teardown.
            window.hide()?;
            assert!(
                !window.window().is_visible(),
                "{} hide did not complete",
                case.name
            );
        }
        let weak = window.as_weak();
        drop(host);
        assert!(
            weak.upgrade().is_none(),
            "{} retained its hidden host",
            case.name
        );
    }
    Ok(())
}

/// Enter through actual receiving navigation, never just the presentation flags.
fn sources(window: &AppWindow) {
    window.invoke_mode_requested(true);
    window.invoke_navigate(Page::Sources);
}

/// Review entered from a scrolled list must reveal its heading and controls.
///
/// # Panics
/// Fails if review/close preserve an unrelated scroll offset or lose the preview.
fn check_review_reveals_its_own_context(window: &AppWindow) {
    sources(window);
    window
        .window()
        .dispatch_event(slint::platform::WindowEvent::WindowActiveChanged(true));
    window.invoke_add_requested();
    assert_eq!(
        window.invoke_raw_text_edited(0, "Review sample".into()),
        "Review sample"
    );
    assert_eq!(
        window.invoke_raw_text_edited(1, "review.example.invalid".into()),
        "review.example.invalid"
    );
    assert_eq!(window.invoke_raw_text_edited(2, "4460".into()), "4460");
    window.invoke_update_requested(
        "Review sample".into(),
        "review.example.invalid".into(),
        0,
        "4460".into(),
    );
    assert_eq!(window.get_source_changes().row_count(), 1);
    window.set_page_scroll_y(-180.0);
    window.invoke_review_requested();
    assert!(window.get_review_open());
    assert!(window.get_source_inspection_focused());
    assert_eq!(window.get_page_scroll_y().to_bits(), 0.0_f32.to_bits());
    window.set_page_scroll_y(-80.0);
    window.invoke_review_close_requested();
    assert!(!window.get_review_open());
    assert!(window.get_source_inspection_focused());
    assert_eq!(window.get_page_scroll_y().to_bits(), 0.0_f32.to_bits());
    assert_eq!(window.get_source_changes().row_count(), 1);
}

/// A legal removed-identity replacement makes Undo fail atomically inside review.
///
/// # Panics
/// Fails if a duplicate replaces accepted rows, loses the error reason or the
/// review phase or cannot recover once the conflicting Add is explicitly undone.
fn check_review_undo_conflict_feedback_and_healthy_recovery(window: &AppWindow) {
    sources(window);
    window
        .window()
        .dispatch_event(slint::platform::WindowEvent::WindowActiveChanged(true));
    let baseline = rows(window);
    window.invoke_remove_requested(1);
    window.invoke_add_requested();
    assert_eq!(
        window.invoke_raw_text_edited(0, "Reuse removed North".into()),
        "Reuse removed North"
    );
    assert_eq!(
        window.invoke_raw_text_edited(1, "NORTH.EXAMPLE.INVALID.".into()),
        "NORTH.EXAMPLE.INVALID."
    );
    assert_eq!(window.invoke_raw_text_edited(2, "4460".into()), "4460");
    window.invoke_update_requested(
        "Reuse removed North".into(),
        "NORTH.EXAMPLE.INVALID.".into(),
        0,
        "4460".into(),
    );
    assert_eq!(window.get_selected_source(), 4);
    assert_eq!(changes(window).len(), 2);
    window.invoke_review_requested();
    let before_rows = rows(window);
    let before_changes = changes(window);
    window.set_page_scroll_y(-240.0);
    window.invoke_undo_requested(1);
    assert!(window.get_review_open());
    assert_eq!(rows(window), before_rows);
    assert_eq!(changes(window), before_changes);
    assert!(window.get_source_inspection_focused());
    assert_eq!(window.get_page_scroll_y().to_bits(), 0.0_f32.to_bits());
    assert_eq!(
        window.get_editor_error(),
        "This protocol and server identity are already in the preview."
    );
    // The successful neighboring path clears the prior error through the existing
    // checked EditorProjection reset; no redundant production clear is added.
    window.invoke_undo_requested(4);
    assert!(window.get_review_open());
    assert!(window.get_editor_error().is_empty());
    assert_eq!(changes(window).len(), 1);
    window.invoke_undo_requested(1);
    assert!(window.get_review_open());
    assert!(window.get_editor_error().is_empty());
    assert_eq!(changes(window), Vec::<SourceChangeRow>::new());
    assert_eq!(rows(window), baseline);
    window.invoke_review_close_requested();
    assert!(!window.get_review_open());
}

/// Snapshot all checked effective source rows, independent of their visible filter.
fn rows(window: &AppWindow) -> Vec<SourceRow> {
    window.get_sources().iter().collect()
}

/// Snapshot checked pending before/after rows, retaining a nonempty failure oracle.
fn changes(window: &AppWindow) -> Vec<SourceChangeRow> {
    window.get_source_changes().iter().collect()
}

/// Enter source text through the actual active raw editor receiver.
///
/// # Panics
/// Fails when ordinary fixture setup cannot reach its bounded draft buffer.
fn raw(window: &AppWindow, label: &str, host: &str, port: &str) {
    assert_eq!(window.invoke_raw_text_edited(0, label.into()), label);
    assert_eq!(window.invoke_raw_text_edited(1, host.into()), host);
    assert_eq!(window.invoke_raw_text_edited(2, port.into()), port);
}

/// Check raw fields and explicit protocol without assuming an accepted candidate.
///
/// # Panics
/// Fails if a hidden handler admits, normalizes or discards another page's drafts.
fn assert_raw(window: &AppWindow, label: &str, host: &str, port: &str, protocol: i32) {
    assert_eq!(window.get_draft_label(), label);
    assert_eq!(window.get_draft_host(), host);
    assert_eq!(window.get_draft_port(), port);
    assert_eq!(window.get_draft_protocol_choice(), protocol);
}

/// Exercise each old receiver with useful legal values against an offscreen owner.
///
/// # Panics
/// Fails if any receiver changes a source, selection, pending delta or review state.
fn check_closed_editor_hidden_requests_preserve_all_source_state(window: &AppWindow) {
    sources(window);
    window.invoke_add_requested();
    raw(window, "Pending", "pending.example.invalid", "4460");
    window.invoke_update_requested(
        "Pending".into(),
        "pending.example.invalid".into(),
        0,
        "4460".into(),
    );
    window.invoke_review_requested();
    assert!(window.get_review_open());
    assert_eq!(changes(window).len(), 1);
    assert_eq!(window.get_selected_source(), 4);
    let accepted = rows(window);
    let pending = changes(window);
    // The closed source editor rejects raw replacement as well as later actions.
    assert_eq!(
        window.invoke_raw_text_edited(0, "Keep raw".into()),
        "Pending"
    );
    window.invoke_navigate(Page::About);
    // These setters can change appearance only, not the receiving route owner.
    window.set_current_page(Page::Sources);
    window.set_power_mode(true);
    let requests: [fn(&AppWindow); 10] = [
        |window| window.invoke_source_selected(1),
        AppWindow::invoke_add_requested,
        AppWindow::invoke_edit_requested,
        |window| {
            window.invoke_update_requested(
                "Hidden".into(),
                "hidden.example.invalid".into(),
                0,
                "4460".into(),
            );
        },
        AppWindow::invoke_cancel_requested,
        |window| window.invoke_remove_requested(1),
        |window| window.invoke_undo_requested(4),
        AppWindow::invoke_review_requested,
        AppWindow::invoke_review_close_requested,
        AppWindow::invoke_discard_requested,
    ];
    for request in requests {
        request(window);
        assert_eq!(rows(window), accepted);
        assert_eq!(changes(window), pending);
        assert_eq!(window.get_selected_source(), 4);
        assert!(window.get_review_open());
        assert!(!window.get_editor_open());
        assert_raw(window, "Pending", "pending.example.invalid", "4460", 0);
        assert_eq!(
            window.get_editor_error(),
            "Open Sources in Power User mode before changing source previews."
        );
    }
    // The protocol compatibility receiver uses the guarded raw-choice path.
    window.invoke_protocol_requested(1);
    assert_eq!(
        window.get_editor_error(),
        "This editor is not active. Your draft and accepted settings were kept."
    );
    assert_eq!(rows(window), accepted);
    assert_eq!(changes(window), pending);
    assert_raw(window, "Pending", "pending.example.invalid", "4460", 0);
    // The dormant compatibility callback is inert, including feedback.
    let feedback = window.get_editor_error();
    window.invoke_port_edited();
    assert_eq!(window.get_editor_error(), feedback);
    assert_eq!(rows(window), accepted);
    assert_eq!(changes(window), pending);
    // A legal neighboring request still succeeds once the actual route returns.
    window.invoke_navigate(Page::Sources);
    window.invoke_review_close_requested();
    window.invoke_source_selected(1);
    assert_eq!(window.get_selected_source(), 1);
    window.invoke_review_requested();
    window.invoke_undo_requested(4);
    assert_eq!(rows(window).len(), 3);
    assert_eq!(changes(window), Vec::<SourceChangeRow>::new());
    window.invoke_review_close_requested();
    assert!(!window.get_review_open());
}

/// Hidden `PortEdited` must not change provenance; default switching remains legal.
///
/// # Panics
/// Fails on hidden admission/cancel or a manufactured manual-port provenance flag.
fn check_active_editor_navigation_preserves_input_and_port_provenance(window: &AppWindow) {
    sources(window);
    window.invoke_add_requested();
    // Preserve untouched default-port provenance: editing field2 is a manual edit.
    assert_eq!(
        window.invoke_raw_text_edited(0, "Unsaved".into()),
        "Unsaved"
    );
    assert_eq!(
        window.invoke_raw_text_edited(1, "draft.example.invalid".into()),
        "draft.example.invalid"
    );
    let accepted = rows(window);
    window.invoke_navigate(Page::About);
    window.invoke_mode_requested(false);
    window.set_current_page(Page::Sources);
    window.set_power_mode(true);
    window.invoke_port_edited();
    window.invoke_protocol_requested(1);
    window.invoke_update_requested(
        "Hidden accepted".into(),
        "hidden.example.invalid".into(),
        1,
        "123".into(),
    );
    window.invoke_cancel_requested();
    window.invoke_add_requested();
    window.invoke_edit_requested();
    assert!(window.get_editor_open());
    assert_eq!(rows(window), accepted);
    assert_eq!(changes(window), Vec::<SourceChangeRow>::new());
    assert_raw(window, "Unsaved", "draft.example.invalid", "4460", 0);
    sources(window);
    assert_raw(window, "Unsaved", "draft.example.invalid", "4460", 0);
    window.invoke_protocol_requested(1);
    assert_eq!(window.get_draft_port(), "123");
    assert_eq!(window.get_draft_protocol_choice(), 1);
    raw(window, "Visible", "visible.example.invalid", "123");
    window.invoke_update_requested(
        "Visible".into(),
        "visible.example.invalid".into(),
        1,
        "123".into(),
    );
    assert_eq!(rows(window).len(), 4);
    assert_eq!(changes(window).len(), 1);
    assert!(!window.get_editor_open());
}

/// Quit and recovery-leave ownership supersede even spoofed route/modal flags.
///
/// # Panics
/// Fails if a hidden current modal loses source drafts or accepted candidates.
fn check_owned_modals_reject_spoofed_source_requests(window: &AppWindow) {
    sources(window);
    window.invoke_add_requested();
    // Leave the default port untouched so the later protocol switch may replace it.
    assert_eq!(
        window.invoke_raw_text_edited(0, "Modal draft".into()),
        "Modal draft"
    );
    assert_eq!(
        window.invoke_raw_text_edited(1, "draft.example.invalid".into()),
        "draft.example.invalid"
    );
    let accepted = rows(window);
    window.invoke_quit_requested();
    assert!(window.get_quit_warning_open());
    window.set_quit_warning_open(false);
    window.invoke_cancel_requested();
    window.invoke_protocol_requested(1);
    window.invoke_port_edited();
    window.invoke_update_requested(
        "Hidden".into(),
        "hidden.example.invalid".into(),
        0,
        "4460".into(),
    );
    assert_eq!(rows(window), accepted);
    assert_eq!(changes(window), Vec::<SourceChangeRow>::new());
    assert!(window.get_editor_open());
    assert_raw(window, "Modal draft", "draft.example.invalid", "4460", 0);
    window.invoke_quit_cancel_requested();
    window.invoke_recovery_open_requested();
    assert_eq!(window.invoke_raw_choice_edited(1, 1), 1);
    assert_eq!(
        window.invoke_raw_text_edited(5, "helper.example.invalid".into()),
        "helper.example.invalid"
    );
    window.invoke_navigate(Page::About);
    assert!(window.get_recovery_leave_open());
    window.set_recovery_leave_open(false);
    window.set_current_page(Page::Sources);
    window.set_power_mode(true);
    window.invoke_cancel_requested();
    window.invoke_port_edited();
    window.invoke_protocol_requested(1);
    window.invoke_update_requested(
        "Hidden".into(),
        "hidden.example.invalid".into(),
        0,
        "4460".into(),
    );
    assert_eq!(rows(window), accepted);
    assert_eq!(changes(window), Vec::<SourceChangeRow>::new());
    assert!(window.get_editor_open());
    assert_raw(window, "Modal draft", "draft.example.invalid", "4460", 0);
    window.invoke_recovery_leave_stay_requested();
    window.invoke_recovery_cancel_requested();
    window.invoke_recovery_back_requested();
    assert_eq!(window.get_current_page(), Page::Sources);
    window.invoke_protocol_requested(1);
    assert_eq!(window.get_draft_port(), "123");
    window.invoke_cancel_requested();
    assert!(!window.get_editor_open());
    assert_eq!(rows(window), accepted);
}

//! Real generated receiving checks for local profile/query/detail workflows.
#![forbid(unsafe_code)]

use std::{error::Error, rc::Rc};

use nevrotick_ui::slint_host::{self, AppWindow, Page};
use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{Platform, PlatformError, WindowAdapter};
use slint::{ComponentHandle, Model};

/// Public software adapter; no service, network, filesystem or native UI effects.
struct Headless;

impl Platform for Headless {
    /// Construct an event-loop-thread-only software window without showing it.
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

/// Each helper gets a fresh sole source/policy owner and its real generated view.
///
/// # Errors
/// Propagates setup failure instead of reporting unexecuted assertions as passes.
///
/// # Panics
/// Fails when actual receiving phase/membership/query guards lose valid state.
#[test]
fn generated_policy_query_details_preserve_separate_owners() -> Result<(), Box<dyn Error>> {
    slint::platform::set_platform(Box::new(Headless))?;
    let checks = [
        CallbackCase {
            name: "check_profile_phases_and_invalid_direct_requests",
            run: check_profile_phases_and_invalid_direct_requests,
            warning_remains_shown: false,
        },
        CallbackCase {
            name: "check_profile_drafts_survive_navigation_and_keep_sources",
            run: check_profile_drafts_survive_navigation_and_keep_sources,
            warning_remains_shown: false,
        },
        CallbackCase {
            name: "check_query_atomicity_and_separate_models",
            run: check_query_atomicity_and_separate_models,
            warning_remains_shown: false,
        },
        CallbackCase {
            name: "check_hidden_selection_and_live_detail_membership",
            run: check_hidden_selection_and_live_detail_membership,
            warning_remains_shown: false,
        },
        CallbackCase {
            name: "check_modal_and_route_receiving_guards",
            run: check_modal_and_route_receiving_guards,
            warning_remains_shown: true,
        },
    ];
    for case in checks {
        let host = slint_host::create()?;
        let window = host.window();
        assert!(!window.window().is_visible(), "{} started shown", case.name);
        window
            .window()
            .set_size(slint::LogicalSize::new(640.0, 600.0));
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
    let fresh = slint_host::create()?;
    assert!(!fresh.window().get_policy_has_preview());
    assert_eq!(fresh.window().get_accepted_source_query(), "");
    assert!(!fresh.window().get_inspection_detail_open());
    Ok(())
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

/// Snapshot all-source IDs; this is the Statistics input, never a filtered list.
fn all_ids(window: &AppWindow) -> Vec<i32> {
    window
        .get_sources()
        .iter()
        .map(|source| source.view_id)
        .collect()
}

/// Snapshot the separate visible list using stable IDs rather than row indices.
fn visible_ids(window: &AppWindow) -> Vec<i32> {
    window
        .get_visible_sources()
        .iter()
        .map(|source| source.view_id)
        .collect()
}

/// Enter the actual Power-only source route before requests reach its receiver.
fn sources(window: &AppWindow) {
    window.invoke_mode_requested(true);
    window.invoke_navigate(Page::Sources);
}

/// Enter a current list-query draft, then invoke its independent Submit receiver.
///
/// # Panics
/// Fails when this fixture's visible search context rejects ordinary raw input.
fn query(window: &AppWindow, raw: &str) {
    assert!(window.invoke_source_search_requested(true));
    edit_text(window, 3, raw);
    window.invoke_source_query_requested(raw.into());
}

/// All three choices are explicit; spoofed view phase never supplies authority.
///
/// # Panics
/// Fails on raw-choice fallback, phase bypass or loss of accepted candidate.
fn check_profile_phases_and_invalid_direct_requests(window: &AppWindow) {
    window.invoke_navigate(Page::Settings);
    window.invoke_settings_section_requested(3);
    assert_eq!(window.get_policy_phase(), 0);
    assert!(!window.get_policy_has_preview());
    window.set_policy_phase(1);
    window.invoke_policy_review_requested(1);
    assert_eq!(window.get_policy_phase(), 0);
    assert!(!window.get_policy_has_preview());
    assert!(!window.get_policy_error().is_empty());
    window.invoke_policy_edit_requested();
    assert_eq!(window.get_policy_phase(), 1);
    for invalid in [i32::MIN, -1, 3, i32::MAX] {
        let prior_choice = window.get_draft_policy_choice();
        window.set_draft_policy_choice(invalid);
        assert_eq!(window.invoke_raw_choice_edited(3, invalid), prior_choice);
        window.invoke_policy_review_requested(invalid);
        assert_eq!(window.get_policy_phase(), 1);
        assert_eq!(window.get_draft_policy_choice(), prior_choice);
        assert!(!window.get_policy_has_preview());
        assert_eq!(
            window.get_policy_error(),
            "The submitted values do not match the current draft. Review the draft and try again."
        );
    }
    // A valid option also needs the exact currently entered draft snapshot.
    edit_choice(window, 3, 1);
    window.invoke_policy_review_requested(2);
    assert_eq!(
        window.get_policy_error(),
        "The submitted values do not match the current draft. Review the draft and try again."
    );
    assert_eq!(window.get_policy_phase(), 1);
    assert_eq!(window.get_draft_policy_choice(), 1);
    assert!(!window.get_policy_has_preview());
    for choice in [0, 1, 2] {
        edit_choice(window, 3, choice);
        window.invoke_policy_review_requested(choice);
        assert_eq!(window.get_policy_phase(), 2);
        assert_eq!(window.get_policy_accepted_choice(), choice);
        assert!(window.get_policy_has_preview());
        assert_eq!(window.get_policy_error(), "");
        window.invoke_policy_keep_requested();
        assert_eq!(window.get_policy_phase(), 0);
        window.invoke_policy_edit_requested();
    }
    edit_choice(window, 3, 0);
    window.invoke_policy_cancel_requested();
    assert_eq!(window.get_draft_policy_choice(), 2);
    assert_eq!(window.get_policy_accepted_choice(), 2);
    window.invoke_policy_open_review_requested();
    window.invoke_policy_discard_requested();
    assert_eq!(window.get_policy_phase(), 0);
    assert!(!window.get_policy_has_preview());
    assert_eq!(window.get_policy_accepted_choice(), 0);
}

/// Root raw selectors survive view recreation; profile preview edits no sources.
///
/// # Panics
/// Fails when navigation resets draft, source protocols/ports or another owner.
fn check_profile_drafts_survive_navigation_and_keep_sources(window: &AppWindow) {
    let baseline: Vec<_> = window.get_sources().iter().collect();
    window.invoke_navigate(Page::Settings);
    window.invoke_settings_section_requested(3);
    window.invoke_policy_edit_requested();
    edit_choice(window, 3, 1);
    window.invoke_navigate(Page::About);
    window.invoke_mode_requested(true);
    window.invoke_navigate(Page::Settings);
    assert_eq!(window.get_policy_phase(), 1);
    assert_eq!(window.get_draft_policy_choice(), 1);
    window.invoke_policy_review_requested(1);
    window.invoke_policy_keep_requested();
    assert!(window.get_policy_has_preview());
    assert_eq!(window.get_policy_accepted_choice(), 1);
    assert_eq!(window.get_sources().iter().collect::<Vec<_>>(), baseline);
    window.invoke_policy_edit_requested();
    edit_choice(window, 3, 2);
    window.invoke_policy_review_requested(2);
    assert_eq!(window.get_sources().iter().collect::<Vec<_>>(), baseline);
    assert!(
        !window
            .get_sources()
            .iter()
            .any(|source| source.protocol_choice == 1)
    );
    assert!(
        !window
            .get_sources()
            .iter()
            .any(|source| source.port == "123")
    );
}

/// Query accepts ASCII casefold/spaces and rejects raw forbidden neighbors atomically.
///
/// # Panics
/// Fails if all-source/Statistics rows mutate or invalid raw search loses state.
fn check_query_atomicity_and_separate_models(window: &AppWindow) {
    sources(window);
    assert_eq!(all_ids(window), [1, 2, 3]);
    query(window, "SOUTH");
    assert_eq!(visible_ids(window), [2]);
    assert_eq!(all_ids(window), [1, 2, 3]);
    assert_eq!(window.get_accepted_source_query(), "SOUTH");
    // Active well-formed mismatches preserve both accepted and raw queries.
    assert!(window.invoke_source_search_requested(true));
    window.invoke_source_query_requested("WEST".into());
    assert_eq!(
        window.get_source_query_error(),
        "The submitted values do not match the current draft. Review the draft and try again."
    );
    assert_eq!(window.get_draft_source_query(), "SOUTH");
    assert_eq!(window.get_accepted_source_query(), "SOUTH");
    assert_eq!(visible_ids(window), [2]);
    window.invoke_source_query_requested("SOUTH".into());
    assert_eq!(window.get_source_query_error(), "");
    assert!(!window.get_source_search_open());
    assert!(window.invoke_source_search_requested(true));
    for raw in ["\0", "tab\t", "line\n", "é", "🙂"] {
        assert_eq!(window.invoke_raw_text_edited(3, raw.into()), "SOUTH");
        window.invoke_source_query_requested(raw.into());
        assert_eq!(window.get_draft_source_query(), "SOUTH");
        assert_eq!(window.get_accepted_source_query(), "SOUTH");
        assert_eq!(visible_ids(window), [2]);
        assert_eq!(
            window.get_source_query_error(),
            "The submitted values do not match the current draft. Review the draft and try again."
        );
        assert!(window.get_source_search_open());
    }
    // A domain-invalid but bounded ASCII draft remains visible for correction.
    let too_long = "a".repeat(254);
    query(window, &too_long);
    assert_eq!(window.get_draft_source_query(), too_long.as_str());
    assert_eq!(window.get_accepted_source_query(), "SOUTH");
    assert_eq!(visible_ids(window), [2]);
    assert_eq!(
        window.get_source_query_error(),
        "Use up to 253 printable ASCII search characters. An empty search shows all sources."
    );
    assert!(window.get_source_search_open());
    for filter in [i32::MIN, -1, 1, 2, 3, i32::MAX] {
        window.invoke_source_filter_requested(filter);
        assert_eq!(visible_ids(window), [2]);
        assert_eq!(window.get_accepted_source_query(), "SOUTH");
        assert!(!window.get_source_query_error().is_empty());
    }
    window.invoke_source_filter_requested(0);
    assert_eq!(window.get_source_query_error(), "");
    query(window, " south ");
    assert_eq!(window.get_accepted_source_query(), " south ");
    assert_eq!(visible_ids(window), Vec::<i32>::new());
    assert_eq!(all_ids(window), [1, 2, 3]);
    window.invoke_source_query_clear_requested();
    assert_eq!(window.get_draft_source_query(), "");
    assert_eq!(visible_ids(window), [1, 2, 3]);
}

/// Hidden/stale stable IDs cannot become another row; details resolve after mutation.
///
/// # Panics
/// Fails on hidden identity edit/removal, stale aliasing or cached detail values.
fn check_hidden_selection_and_live_detail_membership(window: &AppWindow) {
    sources(window);
    // Make the genuine query editor present before Details takes ownership.
    assert!(window.invoke_source_search_requested(true));
    edit_text(window, 3, "south");
    window.set_page_scroll_y(-500.0);
    window.invoke_source_details_requested(1);
    assert_eq!(window.get_page_scroll_y().to_bits(), 0.0_f32.to_bits());
    assert!(window.get_source_inspection_focused());
    assert!(window.get_inspection_detail_open());
    assert_eq!(window.get_inspection_detail_source().view_id, 1);
    // The search editor/list are hidden by Details; valid raw and Submit
    // callbacks cannot mutate that hidden context.
    assert_eq!(window.invoke_raw_text_edited(3, "west".into()), "south");
    window.invoke_source_query_requested("south".into());
    assert_eq!(window.get_draft_source_query(), "south");
    assert_eq!(window.get_accepted_source_query(), "");
    assert_eq!(all_ids(window), [1, 2, 3]);
    window.invoke_source_filter_requested(0);
    assert!(!window.get_source_query_error().is_empty());
    window.invoke_source_details_requested(2);
    assert_eq!(window.get_inspection_detail_source().view_id, 1);
    window.invoke_add_requested();
    window.invoke_edit_requested();
    window.invoke_remove_requested(1);
    assert!(!window.get_editor_open());
    assert_eq!(all_ids(window), [1, 2, 3]);
    window.invoke_source_details_close_requested();
    query(window, "south");
    assert!(!window.get_selected_source_visible());
    window.set_selected_source_visible(true);
    window.invoke_edit_requested();
    assert!(!window.get_editor_open());
    assert_eq!(window.get_selected_source(), 1);
    window.invoke_remove_requested(1);
    assert_eq!(all_ids(window), [1, 2, 3]);
    window.invoke_source_selected(1);
    assert_eq!(window.get_selected_source(), 1);
    window.invoke_source_details_requested(1);
    assert!(!window.get_inspection_detail_open());
    assert!(!window.get_source_query_error().is_empty());
    window.invoke_source_selected(2);
    assert_eq!(window.get_selected_source(), 2);
    assert!(window.get_selected_source_visible());
    window.invoke_source_query_clear_requested();
    window.invoke_edit_requested();
    edit_text(window, 0, "Edited South");
    edit_text(window, 1, "changed.example.invalid");
    edit_text(window, 2, "4460");
    window.invoke_update_requested(
        "Edited South".into(),
        "changed.example.invalid".into(),
        0,
        "4460".into(),
    );
    window.invoke_source_details_requested(2);
    assert_eq!(window.get_inspection_detail_source().label, "Edited South");
    assert_eq!(
        window.get_inspection_detail_source().host,
        "changed.example.invalid"
    );
    window.invoke_remove_requested(2);
    assert_eq!(all_ids(window), [1, 2, 3]);
    assert!(window.get_inspection_detail_open());
    window.invoke_source_details_close_requested();
    window.invoke_remove_requested(2);
    assert!(!window.get_inspection_detail_open());
    assert_eq!(all_ids(window), [1, 3]);
    window.invoke_source_details_requested(2);
    assert!(!window.get_inspection_detail_open());
    window.invoke_add_requested();
    edit_choice(window, 2, 1);
    edit_text(window, 0, "New");
    edit_text(window, 1, "new.example.invalid");
    edit_text(window, 2, "123");
    window.invoke_update_requested("New".into(), "new.example.invalid".into(), 1, "123".into());
    assert_eq!(all_ids(window), [1, 3, 4]);
    window.invoke_source_details_requested(2);
    assert!(!window.get_inspection_detail_open());
    window.invoke_source_details_requested(4);
    assert!(window.get_inspection_detail_open());
    assert_eq!(window.get_inspection_detail_source().view_id, 4);
    assert_eq!(window.get_inspection_detail_source().protocol_choice, 1);
    window.set_page_scroll_y(-200.0);
    window.invoke_source_details_close_requested();
    assert_eq!(window.get_page_scroll_y().to_bits(), 0.0_f32.to_bits());
    assert!(window.get_source_inspection_focused());
    assert!(!window.get_inspection_detail_open());
}

/// Actual route/editor/modal state guards receivers even when view properties lie.
///
/// # Panics
/// Fails when hidden route or quit confirmation accepts another workflow mutation.
fn check_modal_and_route_receiving_guards(window: &AppWindow) {
    window.invoke_policy_edit_requested();
    assert_eq!(window.get_policy_phase(), 0);
    assert_eq!(window.invoke_raw_text_edited(3, "south".into()), "");
    window.invoke_source_query_requested("south".into());
    assert_eq!(window.get_accepted_source_query(), "");
    window.invoke_source_details_requested(1);
    assert!(!window.get_inspection_detail_open());
    sources(window);
    window.invoke_edit_requested();
    assert_eq!(window.invoke_raw_text_edited(3, "south".into()), "");
    window.invoke_source_query_requested("south".into());
    assert_eq!(window.get_accepted_source_query(), "");
    assert!(window.get_editor_open());
    window.invoke_navigate(Page::Settings);
    window.invoke_policy_edit_requested();
    assert_eq!(window.get_policy_phase(), 0);
    assert!(window.get_policy_error().contains("source preview edit"));
    assert!(!window.get_policy_error().contains("Open Settings"));
    window.invoke_navigate(Page::Sources);
    window.invoke_cancel_requested();
    window.invoke_quit_requested();
    assert!(window.get_quit_warning_open());
    assert_eq!(window.invoke_raw_text_edited(3, "south".into()), "");
    window.invoke_source_query_requested("south".into());
    assert_eq!(window.get_accepted_source_query(), "");
    window.invoke_source_details_requested(1);
    assert!(!window.get_inspection_detail_open());
    window.invoke_quit_cancel_requested();
    query(window, "south");
    assert_eq!(visible_ids(window), [2]);
    window.invoke_navigate(Page::Settings);
    window.invoke_settings_section_requested(1);
    window.invoke_policy_edit_requested();
    assert_eq!(window.get_policy_phase(), 0);
    window.invoke_settings_section_requested(0);
    window.invoke_policy_edit_requested();
    assert_eq!(window.get_policy_phase(), 0);
    window.invoke_settings_section_requested(3);
    window.invoke_policy_edit_requested();
    assert_eq!(window.get_policy_phase(), 1);
}

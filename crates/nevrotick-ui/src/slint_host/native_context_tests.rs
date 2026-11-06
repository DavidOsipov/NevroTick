// SPDX-License-Identifier: GPL-3.0-only
//! Portable real-owner counterexamples for current inline/native interaction.
#![forbid(unsafe_code)]

use std::{error::Error, rc::Rc};

use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{Platform, PlatformError, WindowAdapter};
use slint::{ComponentHandle, Model};

use super::{EditorTarget, InlineOwner, Page, SlintHost, create, recovery_workflow};
use crate::tray_ui::TrayInteraction;

/// Public software fixture; no native host, timer, service or OS port is installed.
struct Headless;
impl Platform for Headless {
    /// Supply the same public software adapter used by existing owner controls.
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer))
    }
}

/// Construct the ordinary effect-free owner on this test's UI thread.
///
/// # Errors
/// Propagates an unexpected software platform or checked fixture failure.
fn fixture() -> Result<SlintHost, Box<dyn Error>> {
    slint::platform::set_platform(Box::new(Headless))?;
    Ok(create()?)
}

/// Retained editor drafts on shared pages are not an invisible active dialog.
///
/// # Errors
/// Propagates unexpected software fixture construction failures.
///
/// # Panics
/// Fails if read-only details block, current editing is free, or navigation erases drafts.
#[test]
fn retained_source_editor_is_exclusive_only_on_current_power_sources() -> Result<(), Box<dyn Error>>
{
    let host = fixture()?;
    let window = host.window();
    window.invoke_mode_requested(true);
    window.invoke_navigate(Page::Sources);
    window.invoke_source_details_requested(1);
    assert_eq!(host.state.borrow().current_inline_owner(), None);
    assert_eq!(
        host.state.borrow().current_native_interaction(),
        TrayInteraction::Free
    );
    window.invoke_source_details_close_requested();
    window.invoke_source_selected(1);
    window.invoke_edit_requested();
    assert_eq!(
        window.invoke_raw_text_edited(0, "Retained source draft".into()),
        "Retained source draft"
    );
    assert_eq!(
        host.state.borrow().current_inline_owner(),
        Some(InlineOwner::Sources)
    );
    assert_eq!(
        host.state.borrow().current_native_interaction(),
        TrayInteraction::DialogOwned
    );
    for page in [Page::Settings, Page::About] {
        window.invoke_navigate(page);
        let owner = host.state.borrow();
        assert_ne!(owner.editor, EditorTarget::Closed);
        assert_eq!(owner.current_inline_owner(), None);
        assert_eq!(owner.current_native_interaction(), TrayInteraction::Free);
        drop(owner);
        assert_eq!(window.get_draft_label(), "Retained source draft");
    }
    window.invoke_navigate(Page::Sources);
    assert_eq!(
        host.state.borrow().current_inline_owner(),
        Some(InlineOwner::Sources)
    );
    assert_eq!(
        host.state.borrow().current_native_interaction(),
        TrayInteraction::DialogOwned
    );
    assert_eq!(window.get_draft_label(), "Retained source draft");
    Ok(())
}

/// A retained source review has the same route scope and preserves its candidate.
///
/// # Errors
/// Propagates unexpected software fixture construction failures.
///
/// # Panics
/// Fails if review becomes global or its accepted candidate is erased on navigation.
#[test]
fn retained_source_review_does_not_own_settings_or_about() -> Result<(), Box<dyn Error>> {
    let host = fixture()?;
    let window = host.window();
    window.invoke_mode_requested(true);
    window.invoke_navigate(Page::Sources);
    window.invoke_remove_requested(1);
    window.invoke_review_requested();
    assert!(host.state.borrow().review_open);
    let retained = window.get_source_changes().row_count();
    assert!(retained > 0);
    assert_eq!(
        host.state.borrow().current_native_interaction(),
        TrayInteraction::DialogOwned
    );
    for page in [Page::Settings, Page::About] {
        window.invoke_navigate(page);
        assert!(host.state.borrow().review_open);
        assert_eq!(host.state.borrow().current_inline_owner(), None);
        assert_eq!(
            host.state.borrow().current_native_interaction(),
            TrayInteraction::Free
        );
        assert_eq!(window.get_source_changes().row_count(), retained);
    }
    window.invoke_navigate(Page::Sources);
    assert_eq!(
        host.state.borrow().current_inline_owner(),
        Some(InlineOwner::Sources)
    );
    assert_eq!(
        host.state.borrow().current_native_interaction(),
        TrayInteraction::DialogOwned
    );
    Ok(())
}

/// Recovery Back retains its accepted review without inventing off-page ownership.
///
/// # Errors
/// Propagates unexpected software fixture construction failures.
///
/// # Panics
/// Fails if Back erases the preview, retains invisible modal ownership or frees current review.
#[test]
fn retained_recovery_review_is_not_active_after_back() -> Result<(), Box<dyn Error>> {
    let host = fixture()?;
    let window = host.window();
    window.invoke_recovery_open_requested();
    assert_eq!(window.invoke_raw_choice_edited(1, 1), 1);
    assert_eq!(
        window.invoke_raw_text_edited(5, "retained.example.invalid".into()),
        "retained.example.invalid"
    );
    window.invoke_recovery_update_requested(1, "retained.example.invalid".into(), "".into());
    assert!(matches!(
        host.state.borrow().recovery_phase,
        recovery_workflow::HelperPhase::Review
    ));
    assert_eq!(
        host.state.borrow().current_inline_owner(),
        Some(InlineOwner::Recovery)
    );
    assert_eq!(
        host.state.borrow().current_native_interaction(),
        TrayInteraction::DialogOwned
    );
    let retained = window.get_recovery_preview_hosts().row_count();
    assert_eq!(retained, 1);
    window.invoke_recovery_back_requested();
    assert!(matches!(
        host.state.borrow().recovery_phase,
        recovery_workflow::HelperPhase::Review
    ));
    assert_eq!(host.state.borrow().current_inline_owner(), None);
    assert_eq!(
        host.state.borrow().current_native_interaction(),
        TrayInteraction::Free
    );
    assert_eq!(window.get_recovery_preview_hosts().row_count(), retained);
    window.invoke_recovery_open_requested();
    assert_eq!(
        host.state.borrow().current_inline_owner(),
        Some(InlineOwner::Recovery)
    );
    assert_eq!(
        host.state.borrow().current_native_interaction(),
        TrayInteraction::DialogOwned
    );
    assert_eq!(window.get_recovery_preview_hosts().row_count(), retained);
    Ok(())
}

/// Global Quit and `RecoveryLeave` remain exclusive independently of current page.
///
/// # Errors
/// Propagates unexpected software fixture construction or typed navigation failure.
///
/// # Panics
/// Fails if the inline route restriction accidentally weakens a global modal guard.
#[test]
fn global_quit_and_recovery_leave_still_block_every_page() -> Result<(), Box<dyn Error>> {
    let host = fixture()?;
    let window = host.window();
    window.invoke_mode_requested(true);
    let pages = [
        Page::Status,
        Page::Sources,
        Page::Statistics,
        Page::Diagnostics,
        Page::Settings,
        Page::About,
        Page::Recovery,
    ];
    for page in pages {
        window.invoke_navigate(page);
        window.invoke_quit_requested();
        assert!(window.get_quit_warning_open());
        assert!(window.window().is_visible());
        assert_eq!(
            host.state.borrow().current_native_interaction(),
            TrayInteraction::DialogOwned
        );
        window.invoke_quit_cancel_requested();
        assert!(!window.get_quit_warning_open());
        assert!(window.window().is_visible());
    }
    host.state.borrow_mut().recovery_leave = Some(recovery_workflow::RecoveryLeave::Page(
        crate::presentation::Page::Status,
    ));
    for page in [
        crate::presentation::Page::Status,
        crate::presentation::Page::Sources,
        crate::presentation::Page::Statistics,
        crate::presentation::Page::Diagnostics,
        crate::presentation::Page::Settings,
        crate::presentation::Page::About,
        crate::presentation::Page::Recovery,
    ] {
        // Private typed context variation proves this modal stays global even
        // if a future caller changes navigation; no view flag supplies authority.
        host.state.borrow_mut().navigation.navigate(page)?;
        assert_eq!(
            host.state.borrow().current_native_interaction(),
            TrayInteraction::DialogOwned
        );
    }
    host.state.borrow_mut().recovery_leave = None;
    window.hide()?;
    let weak = window.as_weak();
    drop(host);
    assert!(weak.upgrade().is_none());
    Ok(())
}

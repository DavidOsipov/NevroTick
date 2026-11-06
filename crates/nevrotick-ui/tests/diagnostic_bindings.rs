//! Reached diagnostic choices preserve unrelated state and cannot start effects.
#![forbid(unsafe_code)]

use std::{error::Error, rc::Rc};

use nevrotick_ui::slint_host::{self, Page};
use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{Platform, PlatformError, WindowAdapter};
use slint::{ComponentHandle, Model};

/// Public software platform, with no OS clipboard, file or collection adapter.
struct Headless;

impl Platform for Headless {
    /// Create only an offscreen window owned by the UI thread.
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer))
    }
}

/// Challenge callbacks before entry, during quit review and after leaving Diagnostics.
///
/// # Errors
/// Propagates real platform/host setup failure.
///
/// # Panics
/// Fails if closed-choice admission, view ownership or unrelated state changes.
#[test]
fn generated_diagnostic_callbacks_enforce_the_reached_surface() -> Result<(), Box<dyn Error>> {
    slint::platform::set_platform(Box::new(Headless))?;
    let host = slint_host::create()?;
    let window = host.window();
    assert!(!window.window().is_visible());
    window.invoke_diagnostic_view_requested(2);
    assert_eq!(window.get_diagnostic_view(), 0);
    window.invoke_mode_requested(true);
    window.invoke_navigate(Page::Diagnostics);
    window.invoke_diagnostic_collection_requested(2, 2);
    assert_eq!(window.get_diagnostic_scope(), 0);
    assert_eq!(window.get_diagnostic_duration(), 0);
    window.invoke_diagnostic_view_requested(2);
    assert_eq!(window.get_diagnostic_view(), 2);
    window.invoke_diagnostic_collection_requested(2, 1);
    assert_eq!(window.get_diagnostic_scope(), 2);
    assert_eq!(window.get_diagnostic_duration(), 1);
    for (scope, duration) in [(-1, 0), (3, 0), (0, -1), (0, 3), (i32::MAX, i32::MIN)] {
        window.invoke_diagnostic_collection_requested(scope, duration);
        assert_eq!(window.get_diagnostic_scope(), 2);
        assert_eq!(window.get_diagnostic_duration(), 1);
        assert!(!window.get_diagnostic_feedback().is_empty());
    }
    window.invoke_diagnostic_view_requested(1);
    for choice in [i32::MIN, -1, 4, i32::MAX] {
        window.invoke_diagnostic_view_requested(choice);
        assert_eq!(window.get_diagnostic_view(), 1);
    }
    window.invoke_diagnostic_action_requested(0);
    assert!(window.get_diagnostic_feedback().contains("unavailable"));
    let before = window.get_diagnostic_feedback();
    window.invoke_quit_requested();
    assert!(window.get_quit_warning_open());
    assert!(window.window().is_visible());
    window.invoke_diagnostic_view_requested(3);
    window.invoke_diagnostic_collection_requested(0, 0);
    window.invoke_diagnostic_action_requested(i32::MAX);
    assert_eq!(window.get_diagnostic_view(), 1);
    assert_eq!(window.get_diagnostic_feedback(), before);
    window.invoke_quit_cancel_requested();
    assert!(!window.get_quit_warning_open());
    assert!(window.window().is_visible());
    assert!(!window.get_ui_exit_requested());
    window.invoke_diagnostic_view_requested(3);
    assert_eq!(window.get_diagnostic_view(), 3);
    window.invoke_navigate(Page::Sources);
    window.invoke_diagnostic_view_requested(0);
    assert_eq!(window.get_diagnostic_view(), 3);
    assert_eq!(window.get_sources().row_count(), 3);
    assert_eq!(window.get_source_changes().row_count(), 0);
    assert_eq!(window.get_accepted_interval_label(), "Automatic");
    // The Quit request intentionally showed this previously hidden fixture.
    // Cancel keeps the app open; this harness owns its one manual display cleanup.
    assert!(window.window().is_visible());
    window.hide()?;
    assert!(!window.window().is_visible());
    let weak = window.as_weak();
    drop(host);
    assert!(weak.upgrade().is_none());
    Ok(())
}

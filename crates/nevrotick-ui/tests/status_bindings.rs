//! Actual generated status callbacks against the fixed disconnected input port.
#![forbid(unsafe_code)]

use std::{error::Error, rc::Rc};

use nevrotick_ui::slint_host::{self, Page};
use nevrotick_ui::status_ui::StatusText;
use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{Platform, PlatformError, WindowAdapter, WindowEvent};
use slint::{ComponentHandle, Model};

/// No native window, event loop, service, measurement or clock operation exists.
struct Headless;

impl Platform for Headless {
    /// Supply the public software window for actual generated binding checks.
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer))
    }
}

/// Status connects its checked DTO and contextual remedy in both modes.
///
/// # Errors
/// Propagates platform/fixture/show/hide failure rather than claiming missing checks.
///
/// # Panics
/// Fails on forged view authority, wrong destination, false positive state or lost drafts.
#[test]
fn generated_status_actions_use_the_fixed_port_and_preserve_shared_state()
-> Result<(), Box<dyn Error>> {
    slint::platform::set_platform(Box::new(Headless))?;
    let host = slint_host::create()?;
    let window = host.window();
    window.show()?;
    window
        .window()
        .dispatch_event(WindowEvent::WindowActiveChanged(true));
    assert!(!window.get_power_mode());
    assert_eq!(window.get_current_page(), Page::Status);
    assert_eq!(
        window.get_status_headline(),
        StatusText::NoConnection.english()
    );
    assert_eq!(
        window.get_status_authentication(),
        StatusText::NotEstablished.english()
    );
    assert_eq!(
        window.get_status_agreement(),
        StatusText::NotEvaluated.english()
    );
    assert_eq!(
        window.get_status_freshness(),
        StatusText::NotAvailable.english()
    );
    assert_eq!(
        window.get_status_profile(),
        StatusText::ProfileUnknown.english()
    );
    // Direct invocation cannot turn the disabled measurement control into an effect.
    window.invoke_status_check_requested();
    assert_eq!(
        window.get_status_feedback(),
        StatusText::CheckUnavailable.english()
    );
    assert_eq!(
        window.get_status_headline(),
        StatusText::NoConnection.english()
    );
    window.invoke_status_remedy_requested();
    assert_eq!(window.get_current_page(), Page::Diagnostics);
    assert!(window.get_power_mode());
    // Calling a Status-only callback from a different page is rejected by its owner.
    window.invoke_status_remedy_requested();
    assert_eq!(
        window.get_status_feedback(),
        StatusText::ActionRequired.english()
    );
    assert_eq!(window.get_current_page(), Page::Diagnostics);
    window.invoke_navigate(Page::Sources);
    window.invoke_edit_requested();
    assert_eq!(
        window.invoke_raw_text_edited(1, "keep-status-route.example.invalid".into()),
        "keep-status-route.example.invalid"
    );
    window.invoke_navigate(Page::Settings);
    window.invoke_settings_section_requested(0);
    assert_eq!(window.invoke_raw_choice_edited(0, 4), 4);
    assert_eq!(window.invoke_raw_text_edited(4, "0".into()), "0");
    let sources = window.get_sources().iter().collect::<Vec<_>>();
    window.invoke_navigate(Page::Status);
    window.invoke_quit_requested();
    assert!(window.get_quit_warning_open());
    let feedback = window.get_status_feedback();
    window.set_quit_warning_open(false);
    window.set_fixture_status_remedy_label("Use Windows Time".into());
    window.invoke_status_remedy_requested();
    window.invoke_status_check_requested();
    assert_eq!(window.get_current_page(), Page::Status);
    assert_eq!(window.get_status_feedback(), feedback);
    window.invoke_quit_cancel_requested();
    window.invoke_status_remedy_requested();
    assert_eq!(window.get_current_page(), Page::Diagnostics);
    assert!(window.get_editor_open());
    assert_eq!(window.get_draft_host(), "keep-status-route.example.invalid");
    assert_eq!(window.get_draft_custom_minutes(), "0");
    assert_eq!(window.get_sources().iter().collect::<Vec<_>>(), sources);
    window.hide()?;
    let weak = window.as_weak();
    drop(host);
    assert!(weak.upgrade().is_none());
    Ok(())
}

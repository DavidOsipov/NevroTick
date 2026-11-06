//! Actual raw/Submit receiving controls for the Recovery form's bounded reveal.
#![forbid(unsafe_code)]

use std::{error::Error, io, rc::Rc};

use nevrotick_ui::slint_host::{self, AppWindow, Page};
use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{Platform, PlatformError, WindowAdapter, WindowEvent};
use slint::{ComponentHandle, Model, PhysicalSize, Rgb8Pixel};

/// One owned software window; no native display, service, event loop or clock.
struct Offscreen(Rc<MinimalSoftwareWindow>);

impl Platform for Offscreen {
    /// Supply the single adapter whose actual software layout the test paints.
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(self.0.clone())
    }
}

/// Exercise receiving/focus rules, not an assertion that every form pixel fits.
///
/// # Errors
/// Propagates platform, root, frame or intentional fixture-close failures.
///
/// # Panics
/// Fails if a real-context reveal is absent or a forbidden request moves context.
#[test]
fn recovery_reveal_uses_actual_editing_context_and_preserves_raw_state()
-> Result<(), Box<dyn Error>> {
    let adapter = MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer);
    slint::platform::set_platform(Box::new(Offscreen(Rc::clone(&adapter))))?;
    let host = slint_host::create()?;
    let window = host.window();
    adapter.set_size(PhysicalSize::new(640, 600));
    window.show()?;
    window
        .window()
        .dispatch_event(WindowEvent::WindowActiveChanged(true));
    paint(&adapter)?;
    let sources = window.get_sources().row_count();
    let selected = window.get_selected_source();
    let custom = window.get_draft_custom_minutes();
    let notifications = window.get_notifications_enabled();
    window.invoke_recovery_open_requested();
    assert_eq!(window.get_current_page(), Page::Recovery);
    assert!(window.get_power_mode());
    assert_eq!(window.invoke_raw_choice_edited(1, 1), 1);
    assert_eq!(
        window.invoke_raw_text_edited(5, "https://invalid.invalid".into()),
        "https://invalid.invalid"
    );
    paint(&adapter)?;
    check_submit_and_raw_rejections(window, &adapter)?;
    check_review_and_forbidden_contexts(window, &adapter)?;
    assert_eq!(window.get_sources().row_count(), sources);
    assert_eq!(window.get_selected_source(), selected);
    assert_eq!(window.get_draft_custom_minutes(), custom);
    assert_eq!(window.get_notifications_enabled(), notifications);
    assert!(window.window().is_visible());
    // This test intentionally retains its observer throughout. End that fixture
    // lifetime only after the application contracts above have been observed.
    window.hide()?;
    let weak = window.as_weak();
    drop(host);
    assert!(weak.upgrade().is_none());
    Ok(())
}

/// Realize fixed640x600 geometry; a painted frame is required, never assumed.
///
/// # Errors
/// Rejects a missing draw or unchanged sentinel buffer.
fn paint(adapter: &MinimalSoftwareWindow) -> Result<(), io::Error> {
    let sentinel = Rgb8Pixel {
        r: 255,
        g: 0,
        b: 255,
    };
    let mut frame = vec![sentinel; 640 * 600];
    adapter.window().request_redraw();
    let mut painted = false;
    adapter.draw_if_needed(|renderer| {
        renderer.render(&mut frame, 640);
        painted = true;
    });
    if !painted || frame.iter().all(|pixel| *pixel == sentinel) {
        return Err(io::Error::other("Recovery software frame was not painted"));
    }
    Ok(())
}

/// Observe the actual body displacement/focus after a real receiving callback.
///
/// # Errors
/// Propagates missing actual software layout.
///
/// # Panics
/// Fails if the last form is not revealed, the reveal is unstable, or focus is lost.
fn assert_revealed(window: &AppWindow, adapter: &MinimalSoftwareWindow) -> Result<(), io::Error> {
    paint(adapter)?;
    let scroll = window.get_page_scroll_y();
    assert!(scroll.is_finite() && scroll < 0.0);
    assert!(window.get_recovery_content_focused());
    window.invoke_reveal_recovery_form();
    paint(adapter)?;
    assert_eq!(window.get_page_scroll_y().to_bits(), scroll.to_bits());
    Ok(())
}

/// Domain rejection, stale Submit and proper raw failures reveal the same form.
///
/// # Errors
/// Propagates actual-frame failure.
///
/// # Panics
/// Fails when rejection mutates raw/accepted data, or an absent field causes reveal.
fn check_submit_and_raw_rejections(
    window: &AppWindow,
    adapter: &MinimalSoftwareWindow,
) -> Result<(), io::Error> {
    window.set_page_scroll_y(0.0);
    window.invoke_recovery_update_requested(1, window.get_draft_recovery_first(), "".into());
    assert!(!window.get_recovery_error().is_empty());
    assert!(!window.get_recovery_review_open());
    assert_eq!(window.get_recovery_preview_hosts().row_count(), 0);
    assert_eq!(window.get_draft_recovery_first(), "https://invalid.invalid");
    assert_revealed(window, adapter)?;
    let oversized = "x".repeat(1025);
    window.set_page_scroll_y(0.0);
    assert_eq!(
        window.invoke_raw_text_edited(5, oversized.clone().into()),
        "https://invalid.invalid"
    );
    assert_revealed(window, adapter)?;
    window.set_page_scroll_y(0.0);
    assert_eq!(window.invoke_raw_choice_edited(1, 3), 1);
    assert_revealed(window, adapter)?;
    window.set_page_scroll_y(0.0);
    assert_eq!(window.invoke_raw_text_edited(6, oversized.into()), "");
    paint(adapter)?;
    assert_eq!(
        window.get_page_scroll_y().abs().to_bits(),
        0.0_f32.to_bits()
    );
    window.invoke_recovery_update_requested(1, "forged.example.invalid".into(), "".into());
    assert_eq!(window.get_draft_recovery_first(), "https://invalid.invalid");
    assert_eq!(window.get_recovery_preview_hosts().row_count(), 0);
    assert_revealed(window, adapter)?;
    Ok(())
}

/// Keep valid review nearby while wrong phase/page/modal/exit cannot steal scroll.
///
/// # Errors
/// Propagates actual-frame failure.
///
/// # Panics
/// Fails if ordinary editing jumps, a valid review fails, or forbidden calls reveal.
fn check_review_and_forbidden_contexts(
    window: &AppWindow,
    adapter: &MinimalSoftwareWindow,
) -> Result<(), io::Error> {
    window.set_page_scroll_y(0.0);
    assert_eq!(
        window.invoke_raw_text_edited(5, "helper.example.invalid".into()),
        "helper.example.invalid"
    );
    assert_eq!(
        window.get_page_scroll_y().abs().to_bits(),
        0.0_f32.to_bits()
    );
    window.invoke_recovery_update_requested(1, window.get_draft_recovery_first(), "".into());
    assert!(window.get_recovery_review_open());
    assert_eq!(window.get_recovery_preview_hosts().row_count(), 1);
    assert_revealed(window, adapter)?;
    window.set_page_scroll_y(0.0);
    window.invoke_recovery_update_requested(1, window.get_draft_recovery_first(), "".into());
    paint(adapter)?;
    assert_eq!(
        window.get_page_scroll_y().abs().to_bits(),
        0.0_f32.to_bits()
    );
    window.set_ui_exit_requested(true);
    window.invoke_reveal_recovery_form();
    assert_eq!(
        window.get_page_scroll_y().abs().to_bits(),
        0.0_f32.to_bits()
    );
    window.set_ui_exit_requested(false);
    window.invoke_recovery_keep_editing_requested();
    assert_eq!(
        window.invoke_raw_text_edited(5, " raw helper draft ".into()),
        " raw helper draft "
    );
    window.invoke_navigate(Page::Settings);
    assert!(window.get_recovery_leave_open());
    // Process the toolkit's deferred changed-open focus before challenging the
    // forbidden reveal. The observer never chooses or focuses a modal action.
    slint::platform::update_timers_and_animations();
    paint(adapter)?;
    assert!(window.get_recovery_leave_safe_focused());
    window.set_page_scroll_y(0.0);
    window.invoke_reveal_recovery_form();
    window.invoke_recovery_update_requested(1, " raw helper draft ".into(), "".into());
    assert_eq!(
        window.get_page_scroll_y().abs().to_bits(),
        0.0_f32.to_bits()
    );
    assert!(window.get_recovery_leave_open());
    assert!(window.get_recovery_leave_focus_contained());
    assert_eq!(window.get_draft_recovery_first(), " raw helper draft ");
    window.invoke_recovery_leave_stay_requested();
    window.invoke_recovery_cancel_requested();
    window.invoke_recovery_back_requested();
    assert_eq!(window.get_current_page(), Page::Status);
    window.set_page_scroll_y(0.0);
    window.invoke_reveal_recovery_form();
    window.invoke_recovery_update_requested(1, "outside.example.invalid".into(), "".into());
    assert_eq!(
        window.invoke_raw_text_edited(5, "outside.invalid".into()),
        "helper.example.invalid"
    );
    paint(adapter)?;
    assert_eq!(
        window.get_page_scroll_y().abs().to_bits(),
        0.0_f32.to_bits()
    );
    assert_eq!(window.get_current_page(), Page::Status);
    assert_eq!(window.get_recovery_preview_hosts().row_count(), 1);
    Ok(())
}

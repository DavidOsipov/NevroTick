//! Actual generated shell callbacks and public software-window Close lifecycle.
//!
//! No OS window, service, tray, notification, native event loop or network is used.
#![forbid(unsafe_code)]

use std::{
    cell::RefCell,
    error::Error,
    rc::{Rc, Weak},
    sync::{Arc, atomic::Ordering},
};

use nevrotick_ui::slint_host::{self, AppWindow, Page, SourceRow};
use slint::platform::{EventLoopProxy, Key, Platform, PlatformError, WindowAdapter, WindowEvent};
use slint::{ComponentHandle, Model, PhysicalSize, Rgb8Pixel};

/// Bounded public event-loop proxy; no real event loop.
#[path = "private/shell_loop.rs"]
mod loop_fixture;
/// Public fault-injecting software window; no native backend.
#[path = "private/shell_window.rs"]
mod window_fixture;
use loop_fixture::{LoopControls, RecordingProxy};
use window_fixture::{VisibilityControls, VisibilityWindow};

/// Bounded long text used only by renderer tests and the capture example.
#[path = "private/quit_layout.rs"]
mod quit_layout;

/// Public software adapter; URL requests are recorded without launching a browser.
struct Headless {
    /// Fixed code-owned targets observed through Slint's public platform API.
    urls: Rc<RefCell<Vec<String>>>,
    /// Independent public adapter visibility failures and observations.
    visibility: Rc<VisibilityControls>,
    /// Queued callbacks and explicit quit requests, without a running loop.
    loops: Arc<LoopControls>,
    /// The latest public adapter is observed weakly, never retained by the harness.
    adapter: Rc<RefCell<Weak<VisibilityWindow>>>,
}

impl Platform for Headless {
    /// Supply one independently owned software window per fixture.
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        let adapter = VisibilityWindow::new(Rc::clone(&self.visibility));
        *self.adapter.borrow_mut() = Rc::downgrade(&adapter);
        Ok(adapter)
    }

    /// Drain bounded queued work and return; this does not run a native loop.
    ///
    /// # Errors
    /// Propagates the bounded queue's unavailable/termination error.
    fn run_event_loop(&self) -> Result<(), PlatformError> {
        self.loops
            .drain()
            .map(|_| ())
            .map_err(|error| PlatformError::from(error.to_string()))
    }

    /// Supply the public proxy needed to observe intended versus automatic quit.
    fn new_event_loop_proxy(&self) -> Option<Box<dyn EventLoopProxy>> {
        Some(Box::new(RecordingProxy::new(Arc::clone(&self.loops))))
    }

    /// Record an inert URI request for the dialog-permission regression.
    fn open_url(&self, url: &str) -> Result<(), PlatformError> {
        self.urls.borrow_mut().push(url.to_owned());
        Ok(())
    }
}

/// Real callbacks preserve private modal ownership, drafts and unavailable results.
///
/// # Errors
/// Propagates software-platform, checked fixture or show/hide failures.
///
/// # Panics
/// Fails if forged view flags bypass the receiver or shared service routing drifts.
#[test]
fn generated_shell_callbacks_and_close_lifecycle_preserve_the_fixture_contract()
-> Result<(), Box<dyn Error>> {
    let urls = Rc::new(RefCell::new(Vec::new()));
    let visibility = Rc::new(VisibilityControls::default());
    let loops = Arc::new(LoopControls::default());
    let adapter = Rc::new(RefCell::new(Weak::new()));
    slint::platform::set_platform(Box::new(Headless {
        urls: Rc::clone(&urls),
        visibility: Rc::clone(&visibility),
        loops: Arc::clone(&loops),
        adapter: Rc::clone(&adapter),
    }))?;
    // Public owned-loop mode installs the same keepalive sentinel as production.
    // This platform's run implementation only drains queued work, then returns.
    slint::run_event_loop_until_quit()?;
    check_private_quit_owner_and_draft_retention(&urls)?;
    check_service_destination_and_unavailable_ports()?;
    check_actual_quit_keyboard_containment(&loops)?;
    check_quit_body_keyboard_scrolling(&visibility, &adapter, &loops)?;
    check_public_close_and_confirmed_quit()?;
    check_visibility_failure_recovery(&visibility, &loops)?;
    check_restore_and_quit_failure_feedback(&visibility, &loops)?;
    check_failed_first_show_releases_custody(&visibility, &loops)?;
    Ok(())
}

/// Read accepted source projections without interpreting view rows as authority.
fn accepted_rows(window: &AppWindow) -> Vec<SourceRow> {
    window.get_sources().iter().collect()
}

/// Challenge direct callbacks while spoofing the visible modal and exit flags.
///
/// # Errors
/// Propagates fixture/window setup failure.
///
/// # Panics
/// Fails on bypassed confirmation, lost drafts, changed accepted values or URI effect.
fn check_private_quit_owner_and_draft_retention(
    urls: &Rc<RefCell<Vec<String>>>,
) -> Result<(), Box<dyn Error>> {
    let host = slint_host::create()?;
    let window = host.window();
    window.show()?;
    window
        .window()
        .dispatch_event(WindowEvent::WindowActiveChanged(true));
    window.set_quit_warning_open(true);
    window.invoke_quit_confirm_requested();
    assert!(window.window().is_visible());
    assert!(!window.get_ui_exit_requested());
    assert!(!window.get_quit_warning_open());
    assert!(window.get_shell_feedback().contains("review its warning"));

    window.invoke_mode_requested(true);
    window.invoke_navigate(Page::Sources);
    window.invoke_edit_requested();
    assert_eq!(
        window.invoke_raw_text_edited(0, "Pending local edit".into()),
        "Pending local edit"
    );
    assert_eq!(
        window.invoke_raw_text_edited(1, "unsubmitted.example.invalid".into()),
        "unsubmitted.example.invalid"
    );
    assert_eq!(window.invoke_raw_text_edited(2, "1234".into()), "1234");
    let before = accepted_rows(window);
    let interval = window.get_accepted_interval_label();
    let feedback = window.get_service_feedback();
    window.invoke_navigate(Page::About);
    assert!(window.invoke_repository_requested());
    window.invoke_navigate(Page::Sources);
    let requests_before = urls.borrow().len();
    window.invoke_quit_requested();
    assert!(window.get_quit_warning_open());
    assert!(window.get_quit_headline().contains("unconfirmed"));
    assert!(window.get_quit_keep_open_focused());
    // View flags are projections; this cannot release the privately owned modal.
    window.set_quit_warning_open(false);
    window.set_ui_exit_requested(true);
    window.invoke_navigate(Page::About);
    window.invoke_mode_requested(false);
    window.invoke_update_requested(
        "Rejected during modal".into(),
        "modal.example.invalid".into(),
        0,
        "4460".into(),
    );
    window.invoke_interval_save_requested(1, "2".into());
    window.invoke_notifications_requested(false);
    window.invoke_tray_requested(true);
    window.invoke_settings_section_requested(1);
    window.invoke_service_command_requested(0);
    window.invoke_notification_test_requested();
    assert!(!window.invoke_repository_requested());
    assert_eq!(urls.borrow().len(), requests_before);
    assert_eq!(window.get_current_page(), Page::Sources);
    assert!(window.get_power_mode());
    assert_eq!(accepted_rows(window), before);
    assert_eq!(window.get_accepted_interval_label(), interval);
    assert!(window.get_notifications_enabled());
    assert!(!window.get_tray_enabled());
    assert_eq!(window.get_service_feedback(), feedback);
    assert_eq!(window.get_draft_host(), "unsubmitted.example.invalid");

    window.invoke_quit_cancel_requested();
    assert!(!window.get_quit_warning_open());
    assert!(!window.get_ui_exit_requested());
    assert!(window.get_editor_open());
    assert_eq!(window.get_draft_label(), "Pending local edit");
    assert_eq!(window.get_draft_host(), "unsubmitted.example.invalid");
    assert_eq!(window.get_draft_port(), "1234");
    window.invoke_navigate(Page::About);
    assert!(window.invoke_repository_requested());
    window.invoke_navigate(Page::Sources);
    window.invoke_quit_requested();
    window.invoke_manage_service_requested();
    assert_eq!(window.get_current_page(), Page::Settings);
    assert_eq!(window.get_settings_section(), 1);
    assert!(window.get_service_section_focused());
    assert!(!window.get_quit_warning_open());
    assert!(window.get_editor_open());
    assert_eq!(window.get_draft_host(), "unsubmitted.example.invalid");
    window.hide()?;
    let weak = window.as_weak();
    drop(host);
    assert!(weak.upgrade().is_none());
    Ok(())
}

/// Shared service destination retains Simple, while deeper detail/diagnosis is explicit.
///
/// # Errors
/// Propagates fixture/window setup failure.
///
/// # Panics
/// Fails on silently forced Power, invalid section admission or fabricated effect success.
fn check_service_destination_and_unavailable_ports() -> Result<(), Box<dyn Error>> {
    let host = slint_host::create()?;
    let window = host.window();
    window.show()?;
    window
        .window()
        .dispatch_event(WindowEvent::WindowActiveChanged(true));
    window.invoke_quit_requested();
    window.invoke_manage_service_requested();
    assert!(!window.get_power_mode());
    assert_eq!(window.get_current_page(), Page::Settings);
    assert_eq!(window.get_settings_section(), 1);
    assert!(window.get_service_section_focused());
    assert!(!window.get_ui_exit_requested());
    for invalid in [i32::MIN, -1, 5, i32::MAX] {
        window.invoke_settings_section_requested(invalid);
        assert_eq!(window.get_settings_section(), 1);
        assert!(!window.get_power_mode());
    }
    for command in [i32::MIN, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, i32::MAX] {
        window.invoke_service_command_requested(command);
        assert!(
            window
                .get_service_feedback()
                .contains("No request was submitted")
        );
        assert!(!window.get_ui_exit_requested());
        assert_eq!(window.get_current_page(), Page::Settings);
    }
    let prior_notification = window.get_notification_feedback();
    window.invoke_notification_test_requested();
    assert_eq!(window.get_notification_feedback(), prior_notification);
    window.invoke_settings_section_requested(2);
    window.invoke_notification_test_requested();
    assert!(
        window
            .get_notification_feedback()
            .contains("No Windows notification was submitted")
    );
    window.invoke_notifications_requested(false);
    window.invoke_notification_test_requested();
    assert!(
        window
            .get_notification_feedback()
            .contains("off in NevroTick")
    );
    assert!(!window.get_notifications_enabled());
    window.invoke_service_power_requested();
    assert!(window.get_power_mode());
    assert_eq!(window.get_settings_section(), 1);
    window.invoke_diagnose_service_requested();
    assert_eq!(window.get_current_page(), Page::Diagnostics);
    assert!(window.get_power_mode());
    window.hide()?;
    Ok(())
}

/// Public Close dispatch and explicit confirmation use the same private owner.
///
/// # Errors
/// Propagates fixture/window setup failure.
///
/// # Panics
/// Fails if a saved/forged tray flag hides an inaccessible UI or skips its warning.
fn check_public_close_and_confirmed_quit() -> Result<(), Box<dyn Error>> {
    let normal = slint_host::create()?;
    normal.window().show()?;
    // A forged projection cannot change the accepted default tray preference.
    normal.window().set_tray_enabled(true);
    normal
        .window()
        .window()
        .dispatch_event(WindowEvent::CloseRequested);
    assert!(!normal.window().window().is_visible());
    assert!(normal.window().get_ui_exit_requested());
    assert!(!normal.window().get_quit_warning_open());
    normal.window().set_ui_exit_requested(false);
    assert!(!normal.window().invoke_repository_requested());
    let weak = normal.window().as_weak();
    drop(normal);
    assert!(weak.upgrade().is_none());

    let tray = slint_host::create()?;
    tray.window().show()?;
    tray.window().invoke_settings_section_requested(4);
    tray.window().invoke_tray_requested(true);
    tray.window().set_tray_enabled(false);
    tray.window()
        .window()
        .dispatch_event(WindowEvent::CloseRequested);
    assert!(tray.window().window().is_visible());
    assert!(!tray.window().get_ui_exit_requested());
    assert!(
        tray.window()
            .get_shell_feedback()
            .contains("usable tray icon")
    );
    tray.window().invoke_quit_requested();
    assert!(tray.window().get_quit_warning_open());
    tray.window()
        .window()
        .dispatch_event(WindowEvent::CloseRequested);
    assert!(tray.window().window().is_visible());
    assert!(tray.window().get_quit_warning_open());
    tray.window().invoke_quit_confirm_requested();
    assert!(!tray.window().window().is_visible());
    assert!(tray.window().get_ui_exit_requested());
    assert!(!tray.window().get_quit_warning_open());
    let weak = tray.window().as_weak();
    drop(tray);
    assert!(weak.upgrade().is_none());
    Ok(())
}

/// Dispatch a complete key through Slint's public window-event boundary.
///
/// # Errors
/// Propagates bounded queued callback failure before or after dispatch.
fn press_key(
    window: &AppWindow,
    key: Key,
    loops: &LoopControls,
) -> Result<(), slint::EventLoopError> {
    slint::platform::update_timers_and_animations();
    let _before = loops.drain()?;
    let text: slint::SharedString = key.into();
    window
        .window()
        .dispatch_event(WindowEvent::KeyPressed { text: text.clone() });
    window
        .window()
        .dispatch_event(WindowEvent::KeyReleased { text });
    slint::platform::update_timers_and_animations();
    let _after = loops.drain()?;
    Ok(())
}

/// Check initial safe activation, Escape and actual modal focus containment.
///
/// # Errors
/// Propagates fixture/window setup failure.
///
/// # Panics
/// Fails if keyboard traversal can activate an underlay checkbox or unsafe default.
fn check_actual_quit_keyboard_containment(loops: &LoopControls) -> Result<(), Box<dyn Error>> {
    let host = slint_host::create()?;
    let window = host.window();
    window.show()?;
    window
        .window()
        .dispatch_event(WindowEvent::WindowActiveChanged(true));
    window.invoke_navigate(Page::Settings);
    window.invoke_settings_section_requested(2);
    let notifications = window.get_notifications_enabled();
    let tray = window.get_tray_enabled();
    window.invoke_quit_requested();
    assert!(window.get_quit_keep_open_focused());
    press_key(window, Key::Return, loops)?;
    assert!(window.get_quit_trigger_focused());
    assert!(!window.get_quit_warning_open());
    assert!(!window.get_ui_exit_requested());
    assert!(window.window().is_visible());

    window.invoke_quit_requested();
    press_key(window, Key::Escape, loops)?;
    assert!(window.get_quit_trigger_focused());
    assert!(!window.get_quit_warning_open());
    assert!(!window.get_ui_exit_requested());

    window.invoke_quit_requested();
    for _ in 0..12 {
        press_key(window, Key::Tab, loops)?;
        assert!(window.get_quit_focus_contained());
        assert!(window.get_quit_warning_open());
        assert_eq!(window.get_notifications_enabled(), notifications);
        assert_eq!(window.get_tray_enabled(), tray);
    }
    assert!(window.get_quit_keep_open_focused());
    window.window().dispatch_event(WindowEvent::KeyPressed {
        text: Key::Shift.into(),
    });
    for _ in 0..3 {
        press_key(window, Key::Tab, loops)?;
        assert!(window.get_quit_focus_contained());
    }
    window.window().dispatch_event(WindowEvent::KeyReleased {
        text: Key::Shift.into(),
    });
    assert!(window.get_quit_keep_open_focused());
    press_key(window, Key::Space, loops)?;
    assert!(!window.get_quit_warning_open());
    assert!(!window.get_ui_exit_requested());
    assert_eq!(window.get_notifications_enabled(), notifications);
    assert_eq!(window.get_tray_enabled(), tray);
    window.hide()?;
    Ok(())
}

/// Realize the actual fixed minimum software viewport before reading geometry.
///
/// The allocation is fixed to640*600pixels. The weak link cannot retain a root;
/// drawing supplies layout/font metrics without a native display or event loop.
///
/// # Errors
/// Rejects an absent live adapter or a buffer that was not painted.
fn render_minimum_frame(
    visibility: &VisibilityControls,
    weak: &RefCell<Weak<VisibilityWindow>>,
) -> Result<(), PlatformError> {
    let adapter = weak
        .borrow()
        .upgrade()
        .ok_or_else(|| PlatformError::from("Fixture adapter is unavailable"))?;
    visibility.viewport.set(Some(PhysicalSize::new(640, 600)));
    adapter.window().dispatch_event(WindowEvent::Resized {
        size: slint::LogicalSize::new(640.0, 600.0),
    });
    adapter.window().request_redraw();
    let sentinel = Rgb8Pixel {
        r: 255,
        g: 0,
        b: 255,
    };
    let mut frame = vec![sentinel; 640 * 600];
    let _painted = adapter.renderer.render(&mut frame, 640);
    if frame.iter().all(|pixel| *pixel == sentinel) {
        return Err(PlatformError::from(
            "Fixture software frame was not rendered",
        ));
    }
    Ok(())
}

/// Preserve the independently observed English fit as a healthy neighboring case.
///
/// # Errors
/// Propagates bounded queue/key-dispatch failure.
///
/// # Panics
/// Fails when ordinary non-overflowing keys move the body or scroll the underlay.
fn assert_english_quit_warning_fits(
    window: &AppWindow,
    loops: &LoopControls,
    underlay_scroll: f32,
) -> Result<(), Box<dyn Error>> {
    assert_eq!(
        window.get_quit_body_scroll_limit().to_bits(),
        0.0_f32.to_bits()
    );
    for key in [Key::PageDown, Key::End, Key::PageUp, Key::Home] {
        press_key(window, key, loops)?;
        assert_eq!(
            window.get_quit_body_scroll_y().abs().to_bits(),
            0.0_f32.to_bits()
        );
        assert_eq!(
            window.get_page_scroll_y().to_bits(),
            underlay_scroll.to_bits()
        );
        assert!(window.get_quit_focus_contained());
    }
    Ok(())
}

/// Prepare a second raw draft through its actual visible editor, then return.
///
/// # Panics
/// Fails if ordinary interval setup is rejected before the modal scenario.
fn prepare_hidden_interval_draft(window: &AppWindow) {
    window.invoke_settings_section_requested(0);
    assert_eq!(window.invoke_raw_choice_edited(0, 4), 4);
    assert_eq!(window.invoke_raw_text_edited(4, "0".into()), "0");
    window.invoke_navigate(Page::Sources);
}

/// A bounded overflowing layout fixture is fully readable with the keyboard.
///
/// # Errors
/// Propagates fixture/window setup and bounded event-queue failure.
///
/// # Panics
/// Fails when modal scrolling escapes its bounds, moves the underlay or loses drafts.
fn check_quit_body_keyboard_scrolling(
    visibility: &VisibilityControls,
    adapter: &RefCell<Weak<VisibilityWindow>>,
    loops: &LoopControls,
) -> Result<(), Box<dyn Error>> {
    let host = slint_host::create()?;
    let window = host.window();
    window.show()?;
    window.window().dispatch_event(WindowEvent::Resized {
        size: slint::LogicalSize::new(640.0, 600.0),
    });
    window
        .window()
        .dispatch_event(WindowEvent::WindowActiveChanged(true));
    window.invoke_mode_requested(true);
    window.invoke_navigate(Page::Sources);
    window.invoke_edit_requested();
    assert_eq!(
        window.invoke_raw_text_edited(1, "retain-unsubmitted.example.invalid".into()),
        "retain-unsubmitted.example.invalid"
    );
    prepare_hidden_interval_draft(window);
    window.set_page_scroll_y(-60.0);
    // Establish the underlay's actual layout/clamped scroll before the modal.
    render_minimum_frame(visibility, adapter)?;
    let underlay_scroll = window.get_page_scroll_y();
    let accepted = accepted_rows(window);
    window.invoke_quit_requested();
    assert!(window.get_quit_keep_open_focused());
    // Resized alone is not a paint/layout pass on the custom public adapter.
    render_minimum_frame(visibility, adapter)?;
    // The actual maintained English warning fits this pinned viewport/font.
    assert_english_quit_warning_fits(window, loops, underlay_scroll)?;
    quit_layout::expand(window);
    render_minimum_frame(visibility, adapter)?;
    let limit = window.get_quit_body_scroll_limit();
    assert!(limit.is_finite() && limit > 0.0);
    assert_eq!(
        window.get_quit_body_scroll_y().abs().to_bits(),
        0.0_f32.to_bits()
    );
    press_key(window, Key::PageDown, loops)?;
    assert!(window.get_quit_body_scroll_y() < 0.0);
    assert!(window.get_quit_body_scroll_y() >= -limit);
    for _ in 0..4 {
        press_key(window, Key::End, loops)?;
        assert_eq!(
            window.get_quit_body_scroll_y().to_bits(),
            (-limit).to_bits()
        );
    }
    press_key(window, Key::PageUp, loops)?;
    assert!(window.get_quit_body_scroll_y() > -limit);
    assert!(window.get_quit_body_scroll_y() <= 0.0);
    for _ in 0..4 {
        press_key(window, Key::Home, loops)?;
        assert_eq!(
            window.get_quit_body_scroll_y().abs().to_bits(),
            0.0_f32.to_bits()
        );
    }
    for _ in 0..3 {
        press_key(window, Key::Tab, loops)?;
        assert!(window.get_quit_focus_contained());
    }
    assert!(window.get_quit_keep_open_focused());
    assert_eq!(
        window.get_page_scroll_y().to_bits(),
        underlay_scroll.to_bits()
    );
    assert_eq!(
        window.get_draft_host(),
        "retain-unsubmitted.example.invalid"
    );
    assert_eq!(window.get_draft_custom_minutes(), "0");
    assert_eq!(accepted_rows(window), accepted);
    // Reopen the same long presentation, without a controller projection that
    // would replace it with the shorter English copy and hide a reset defect.
    press_key(window, Key::End, loops)?;
    assert!(window.get_quit_body_scroll_y() < 0.0);
    window.set_quit_warning_open(false);
    let _hidden_jobs = loops.drain()?;
    render_minimum_frame(visibility, adapter)?;
    window.set_quit_warning_open(true);
    let _shown_jobs = loops.drain()?;
    render_minimum_frame(visibility, adapter)?;
    assert_eq!(
        window.get_quit_body_scroll_y().abs().to_bits(),
        0.0_f32.to_bits()
    );
    assert!(window.get_quit_keep_open_focused());
    press_key(window, Key::Escape, loops)?;
    assert!(!window.get_quit_warning_open());
    assert!(window.get_quit_trigger_focused());
    assert!(!window.get_ui_exit_requested());
    window.invoke_quit_requested();
    assert_eq!(
        window.get_quit_body_scroll_y().abs().to_bits(),
        0.0_f32.to_bits()
    );
    press_key(window, Key::Escape, loops)?;
    window.hide()?;
    visibility.viewport.set(None);
    Ok(())
}

/// Failed hide restores lifecycle custody without an automatic or explicit exit.
///
/// # Errors
/// Propagates fixture, window or bounded proxy-queue failures.
///
/// # Panics
/// Fails if failed hide queues quit, drops custody or loses the healthy retry.
fn check_visibility_failure_recovery(
    visibility: &VisibilityControls,
    loops: &LoopControls,
) -> Result<(), Box<dyn Error>> {
    let host = slint_host::create()?;
    let window = host.window();
    window.show()?;
    let shows = visibility.show_attempts.get();
    visibility.show_fails.set(true);
    // A visible root must not redundantly show while opening its quit overlay.
    window.invoke_quit_requested();
    assert!(window.get_quit_warning_open());
    assert_eq!(visibility.show_attempts.get(), shows);
    window.invoke_quit_cancel_requested();
    visibility.show_fails.set(false);

    let hides = visibility.hide_attempts.get();
    let quits = loops.quit_requests.load(Ordering::SeqCst);
    visibility.hide_fails.set(true);
    window.window().dispatch_event(WindowEvent::CloseRequested);
    assert_eq!(
        visibility.hide_attempts.get(),
        hides.checked_add(1).ok_or("hide counter exhausted")?
    );
    assert_eq!(
        visibility.show_attempts.get(),
        shows.checked_add(1).ok_or("show counter exhausted")?
    );
    assert_eq!(loops.quit_requests.load(Ordering::SeqCst), quits);
    assert!(window.window().is_visible());
    assert!(visibility.observed_visible.get());
    assert!(!window.get_ui_exit_requested());
    assert!(window.get_shell_feedback().contains("window was restored"));
    // A new explicit user request is a healthy neighbor, not an automatic retry.
    visibility.hide_fails.set(false);
    window.window().dispatch_event(WindowEvent::CloseRequested);
    assert!(!window.window().is_visible());
    assert!(window.get_ui_exit_requested());
    assert_eq!(
        loops.quit_requests.load(Ordering::SeqCst),
        quits.checked_add(1).ok_or("quit counter exhausted")?
    );
    let weak = window.as_weak();
    drop(host);
    let _drained = loops.drain()?;
    assert!(weak.upgrade().is_none());
    Ok(())
}

/// Failed restoration and rejected explicit quit never become close success.
///
/// # Errors
/// Propagates fixture, window or bounded proxy-queue failures.
///
/// # Panics
/// Fails if an uncertain window result is described as restored or completed.
fn check_restore_and_quit_failure_feedback(
    visibility: &VisibilityControls,
    loops: &LoopControls,
) -> Result<(), Box<dyn Error>> {
    let host = slint_host::create()?;
    let window = host.window();
    window.show()?;
    visibility.hide_fails.set(true);
    visibility.show_fails.set(true);
    let quits = loops.quit_requests.load(Ordering::SeqCst);
    window.window().dispatch_event(WindowEvent::CloseRequested);
    assert_eq!(loops.quit_requests.load(Ordering::SeqCst), quits);
    assert!(!window.get_ui_exit_requested());
    assert!(
        window
            .get_shell_feedback()
            .contains("Window state is unconfirmed")
    );
    // Core show bookkeeping is not evidence that the failing adapter restored.
    visibility.hide_fails.set(false);
    visibility.show_fails.set(false);
    window.show()?;
    loops.quit_fails.store(true, Ordering::SeqCst);
    window.window().dispatch_event(WindowEvent::CloseRequested);
    assert!(!window.get_ui_exit_requested());
    assert!(window.window().is_visible());
    assert!(window.get_shell_feedback().contains("exit request failed"));
    assert!(window.get_shell_feedback().contains("window was restored"));
    assert_eq!(
        loops.quit_requests.load(Ordering::SeqCst),
        quits.checked_add(1).ok_or("quit counter exhausted")?
    );
    loops.quit_fails.store(false, Ordering::SeqCst);
    window.window().dispatch_event(WindowEvent::CloseRequested);
    assert!(window.get_ui_exit_requested());
    assert!(!window.window().is_visible());
    let weak = window.as_weak();
    drop(host);
    let _drained = loops.drain()?;
    assert!(weak.upgrade().is_none());
    Ok(())
}

/// An unsuccessful first show must release its acquired strong component reference.
///
/// # Errors
/// Propagates fixture or bounded proxy-queue failures.
///
/// # Panics
/// Fails if an unshown warning leaks custody or opens an unresolvable modal.
fn check_failed_first_show_releases_custody(
    visibility: &VisibilityControls,
    loops: &LoopControls,
) -> Result<(), Box<dyn Error>> {
    let host = slint_host::create()?;
    let window = host.window();
    let weak = window.as_weak();
    visibility.show_fails.set(true);
    let hides = visibility.hide_attempts.get();
    window.invoke_quit_requested();
    assert!(!window.get_quit_warning_open());
    assert!(!window.get_ui_exit_requested());
    assert!(window.get_shell_feedback().contains("could not be shown"));
    assert_eq!(
        visibility.hide_attempts.get(),
        hides.checked_add(1).ok_or("hide counter exhausted")?
    );
    drop(host);
    let _drained = loops.drain()?;
    assert!(weak.upgrade().is_none());

    let failing_start = slint_host::create()?;
    let weak = failing_start.window().as_weak();
    assert!(failing_start.run().is_err());
    let _drained = loops.drain()?;
    assert!(weak.upgrade().is_none());
    visibility.hide_fails.set(true);
    let both_failed = slint_host::create()?;
    let weak = both_failed.window().as_weak();
    let error = both_failed
        .run()
        .err()
        .ok_or("Expected startup/cleanup failure")?;
    assert!(
        error
            .to_string()
            .contains("startup and window cleanup failed")
    );
    let _drained = loops.drain()?;
    assert!(weak.upgrade().is_none());
    visibility.hide_fails.set(false);
    visibility.show_fails.set(false);
    Ok(())
}

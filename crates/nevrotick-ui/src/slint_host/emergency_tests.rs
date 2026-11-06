//! Emergency consent is private, cancellation preserves raw drafts, and exit works.
#![forbid(unsafe_code)]

use std::{
    error::Error,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
};

use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{EventLoopProxy, Key, Platform, PlatformError, WindowAdapter, WindowEvent};
use slint::{ComponentHandle, EventLoopError, SharedString};

use crate::slint_host::{self, AppWindow, Page};

/// No native loop exists; only explicit public quit requests are counted.
struct ExitProxy(
    /// Shared bounded explicit-quit observation, without a window reference.
    Arc<AtomicU32>,
);

impl EventLoopProxy for ExitProxy {
    /// Record one checked explicit request, without pretending native termination.
    ///
    /// # Errors
    /// Rejects an exhausted counter without wrapping it.
    fn quit_event_loop(&self) -> Result<(), EventLoopError> {
        self.0
            .try_update(Ordering::SeqCst, Ordering::SeqCst, |n| n.checked_add(1))
            .map(|_| ())
            .map_err(|_| EventLoopError::EventLoopTerminated)
    }

    /// This zero-capacity fixture rejects arbitrary posted work without running it.
    ///
    /// # Errors
    /// Always returns explicit termination; no successful post is manufactured.
    fn invoke_from_event_loop(
        &self,
        _event: Box<dyn FnOnce() + Send>,
    ) -> Result<(), EventLoopError> {
        Err(EventLoopError::EventLoopTerminated)
    }
}

/// Public software-only platform; no service, clock, tray icon or native loop.
struct Headless(
    /// Shared explicit-quit count, without a retained component or native loop.
    Arc<AtomicU32>,
);

impl Platform for Headless {
    /// Supply an actual public software adapter without an OS window.
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer))
    }

    /// Return immediately, exercising only public owned-loop configuration.
    ///
    /// # Errors
    /// This fixed software fixture returns no event-loop error.
    fn run_event_loop(&self) -> Result<(), PlatformError> {
        Ok(())
    }

    /// Observe explicit quit without creating a native event loop.
    fn new_event_loop_proxy(&self) -> Option<Box<dyn EventLoopProxy>> {
        Some(Box::new(ExitProxy(Arc::clone(&self.0))))
    }
}

/// Dispatch a real public key pair; no arbitrary queued work is executed here.
fn key(window: &AppWindow, key: Key) {
    let text: SharedString = key.into();
    window
        .window()
        .dispatch_event(WindowEvent::KeyPressed { text: text.clone() });
    window
        .window()
        .dispatch_event(WindowEvent::KeyReleased { text });
    slint::platform::update_timers_and_animations();
}

/// Internal terminal-failure producer setup, separate from ordinary invalid input.
#[derive(Clone, Copy)]
enum FailureKind {
    /// A private catalog-render invariant failed, not an external bad catalog.
    Copy,
    /// A private checked raw-reset invariant failed, not a rejected user edit.
    Raw,
}

/// Private terminal markers, rather than forged view flags, own emergency exit.
///
/// # Errors
/// Propagates platform, checked host, owned-loop setup or window visibility errors.
///
/// # Panics
/// Fails if fatal tray mode traps an observer, unsafe resolution changes raw drafts,
/// a forged view arms consent, or explicit confirmation submits anything but UI exit.
#[test]
fn terminal_ui_failure_with_tray_enabled_preserves_drafts_and_exits_after_fixed_consent()
-> Result<(), Box<dyn Error>> {
    let exits = Arc::new(AtomicU32::new(0));
    slint::platform::set_platform(Box::new(Headless(Arc::clone(&exits))))?;
    slint::run_event_loop_until_quit()?;
    for failure in [FailureKind::Copy, FailureKind::Raw] {
        check_terminal_failure(failure, &exits)?;
    }
    Ok(())
}

/// A fresh real root exercises each admitted terminal receiving precondition.
///
/// # Errors
/// Propagates checked host creation and window visibility failure.
///
/// # Panics
/// Fails when consent, raw retention, focus or exact explicit exit count changes.
fn check_terminal_failure(failure: FailureKind, exits: &AtomicU32) -> Result<(), Box<dyn Error>> {
    let exits_before = exits.load(Ordering::SeqCst);
    let host = slint_host::create()?;
    let window = host.window();
    window.show()?;
    window
        .window()
        .dispatch_event(WindowEvent::WindowActiveChanged(true));
    window.invoke_navigate(Page::Settings);
    window.invoke_settings_section_requested(4);
    assert!(window.invoke_tray_requested(true));
    assert!(window.get_tray_enabled());
    // Adapter unavailability alone is not a programmer copy invariant.
    assert!(!window.invoke_window_close_requested());
    assert!(!host.state.borrow().shell.controller.emergency_armed());
    assert!(!window.get_quit_warning_open());
    window.invoke_settings_section_requested(0);
    assert_eq!(window.invoke_raw_choice_edited(0, 4), 4);
    assert_eq!(window.invoke_raw_text_edited(4, "0".into()), "0");
    window.invoke_recovery_open_requested();
    assert_eq!(window.get_current_page(), Page::Recovery);
    assert_eq!(window.invoke_raw_choice_edited(1, 1), 1);
    assert_eq!(
        window.invoke_raw_text_edited(5, " raw recovery draft ".into()),
        " raw recovery draft "
    );
    window.invoke_recovery_back_requested();
    assert!(window.get_recovery_leave_open());
    match failure {
        FailureKind::Copy => super::super::catalog_workflow::invariant_failed(window, &host.state),
        // This injects only the receiving failure precondition; it does not
        // exercise the host's checked-reset producer or its terminal latch.
        FailureKind::Raw => {
            host.state.borrow_mut().raw_admission =
                super::super::raw_workflow::RawAdmission::Failed;
        }
    }
    window.invoke_recovery_leave_keep_requested();
    window.invoke_recovery_leave_discard_requested();
    assert!(host.state.borrow().recovery_leave.is_some());
    assert!(!window.invoke_window_close_requested());
    assert_eq!(window.get_current_page(), Page::Recovery);
    assert!(!window.get_recovery_leave_open());
    assert!(host.state.borrow().recovery_leave.is_none());
    assert!(window.get_emergency_quit_warning());
    assert!(window.get_quit_warning_open());
    assert!(host.state.borrow().shell.controller.emergency_armed());
    assert_eq!(exits.load(Ordering::SeqCst), exits_before);
    assert!(window.window().is_visible());
    assert!(!window.get_quit_manage_focused());
    window.invoke_manage_service_requested();
    assert_eq!(window.get_current_page(), Page::Recovery);
    window.invoke_quit_cancel_requested();
    assert!(!host.state.borrow().shell.controller.emergency_armed());
    assert!(!window.get_quit_warning_open());
    assert_eq!(window.get_draft_recovery_first(), " raw recovery draft ");
    assert_eq!(window.get_draft_custom_minutes(), "0");
    check_forged_emergency_cannot_arm_consent(&host);
    assert!(!window.invoke_window_close_requested());
    assert!(window.get_quit_keep_open_focused());
    key(window, Key::Tab);
    assert!(window.get_quit_confirm_focused());
    assert!(!window.get_quit_manage_focused());
    key(window, Key::Return);
    assert!(window.get_ui_exit_requested());
    assert!(!window.window().is_visible());
    assert!(host.state.borrow().preferences.tray_enabled());
    assert_eq!(exits.load(Ordering::SeqCst), exits_before + 1);
    let weak = window.as_weak();
    drop(host);
    assert!(weak.upgrade().is_none());
    Ok(())
}

/// Raw visibility projections cannot authorize ordinary confirmation after failure.
///
/// # Panics
/// Fails if a projection arms the private controller, exits or selects management.
fn check_forged_emergency_cannot_arm_consent(host: &slint_host::SlintHost) {
    let window = host.window();
    window.set_emergency_quit_warning(true);
    window.set_quit_warning_open(true);
    window.invoke_quit_confirm_requested();
    window.invoke_manage_service_requested();
    window.invoke_quit_requested();
    assert!(!host.state.borrow().shell.controller.emergency_armed());
    assert!(!host.state.borrow().shell.controller.exit_requested());
    assert!(window.window().is_visible());
    assert_eq!(window.get_current_page(), Page::Recovery);
    window.invoke_quit_cancel_requested();
    assert!(!window.get_emergency_quit_warning());
    assert!(!window.get_quit_warning_open());
}

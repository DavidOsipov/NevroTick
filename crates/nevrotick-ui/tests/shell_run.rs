//! Actual host-run ownership checked with a public software platform and proxy.
//!
//! The platform dispatches one Close event and returns; no native loop is run.
#![forbid(unsafe_code)]

use std::{
    cell::RefCell,
    error::Error,
    rc::Rc,
    sync::{Arc, atomic::Ordering},
};

use nevrotick_ui::slint_host::{self, AppWindow};
use slint::ComponentHandle;
use slint::platform::{EventLoopProxy, Platform, PlatformError, WindowAdapter, WindowEvent};

/// Public recording proxy; requests do not terminate a native event loop.
#[path = "private/shell_loop.rs"]
mod loop_fixture;
/// Public bounded visibility-failure adapter shared with the callback tests.
#[path = "private/shell_window.rs"]
mod window_fixture;
use loop_fixture::{LoopControls, RecordingProxy};
use window_fixture::{VisibilityControls, VisibilityWindow};

/// One owned run dispatches its Close request through the actual host bridge.
struct RunPlatform {
    /// Weak root installed by the test after checked construction.
    root: Rc<RefCell<Option<slint::Weak<AppWindow>>>>,
    /// Public visibility controls; normal operations are accepted in this test.
    visibility: Rc<VisibilityControls>,
    /// Counts explicit and last-visible-window quit requests independently.
    loops: Arc<LoopControls>,
}

impl Platform for RunPlatform {
    /// Create an owned public software window with fixed test geometry.
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(VisibilityWindow::new(Rc::clone(&self.visibility)))
    }

    /// Dispatch one Close event then return, without a native/event polling loop.
    ///
    /// # Errors
    /// Rejects missing root, failed close admission or bounded queued work.
    fn run_event_loop(&self) -> Result<(), PlatformError> {
        let weak = self
            .root
            .borrow()
            .clone()
            .ok_or_else(|| PlatformError::from("Missing software root"))?;
        let window = weak
            .upgrade()
            .ok_or_else(|| PlatformError::from("Software root already released"))?;
        window.window().dispatch_event(WindowEvent::CloseRequested);
        if !window.get_ui_exit_requested() || window.window().is_visible() {
            return Err(PlatformError::from("Owned software Close was not accepted"));
        }
        self.loops
            .drain()
            .map(|_| ())
            .map_err(|error| PlatformError::from(error.to_string()))
    }

    /// Record quit requests without exiting or launching any native process.
    fn new_event_loop_proxy(&self) -> Option<Box<dyn EventLoopProxy>> {
        Some(Box::new(RecordingProxy::new(Arc::clone(&self.loops))))
    }
}

/// Host run owns its loop, so intended Close queues exactly one explicit quit.
///
/// With the old `Window::run`/auto-close owner, hide first queues automatic quit and
/// the new explicit close handler queues another; the observed count would be 2.
///
/// # Errors
/// Propagates platform, checked host-run or bounded queue failures.
///
/// # Panics
/// Fails if the actual host entry regresses to automatic last-window loop exit.
#[test]
fn host_run_owns_close_and_queues_only_one_explicit_quit() -> Result<(), Box<dyn Error>> {
    let root = Rc::new(RefCell::new(None));
    let loops = Arc::new(LoopControls::default());
    let visibility = Rc::new(VisibilityControls::default());
    slint::platform::set_platform(Box::new(RunPlatform {
        root: Rc::clone(&root),
        visibility: Rc::clone(&visibility),
        loops: Arc::clone(&loops),
    }))?;
    let host = slint_host::create()?;
    let weak = host.window().as_weak();
    *root.borrow_mut() = Some(weak.clone());
    host.run()?;
    let _drained = loops.drain()?;
    assert_eq!(loops.quit_requests.load(Ordering::SeqCst), 1);
    assert!(!visibility.observed_visible.get());
    assert!(weak.upgrade().is_none());
    Ok(())
}

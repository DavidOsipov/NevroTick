//! Public software-window adapter with bounded show/hide failure controls.
//!
//! This fixture owns no OS window, event loop, thread or native resource. Slint's
//! real Window owns visibility/keepalive bookkeeping; the adapter can reject the
//! requested backend operation so tests challenge its error path independently.
#![forbid(unsafe_code)]

use std::{
    cell::Cell,
    rc::{Rc, Weak},
};

use slint::platform::software_renderer::{RepaintBufferType, SoftwareRenderer};
use slint::platform::{PlatformError, Renderer, WindowAdapter};
use slint::{PhysicalSize, Window};

/// Shared observations and explicit failure switches for one sequential test run.
#[derive(Default)]
pub(crate) struct VisibilityControls {
    /// Reject hide requests while true; no backend visibility change is recorded.
    pub(crate) hide_fails: Cell<bool>,
    /// Reject show requests while true; no backend visibility change is recorded.
    pub(crate) show_fails: Cell<bool>,
    /// Count every backend hide attempt, including a deliberately rejected one.
    pub(crate) hide_attempts: Cell<u32>,
    /// Count every backend show attempt, including a deliberately rejected one.
    pub(crate) show_attempts: Cell<u32>,
    /// Last successfully requested adapter visibility, separate from Window state.
    pub(crate) observed_visible: Cell<bool>,
    /// Explicit software-frame viewport; None retains the prior fixture default.
    pub(crate) viewport: Cell<Option<PhysicalSize>>,
}

/// A public `WindowAdapter` implementation, not a wrapper around another adapter.
pub(crate) struct VisibilityWindow {
    /// Window whose weak adapter link points to this exact owner.
    window: Window,
    /// Public software renderer retained for the entire adapter lifetime.
    pub(crate) renderer: SoftwareRenderer,
    /// Bounded test observations, without retaining any component or window cycle.
    controls: Rc<VisibilityControls>,
}

impl VisibilityWindow {
    /// Create a correctly linked adapter using safe `Rc`/`Weak` unsizing coercion.
    #[must_use]
    pub(crate) fn new(controls: Rc<VisibilityControls>) -> Rc<Self> {
        Rc::new_cyclic(|weak: &Weak<Self>| {
            let adapter: Weak<dyn WindowAdapter> = weak.clone();
            Self {
                window: Window::new(adapter),
                renderer: SoftwareRenderer::new_with_repaint_buffer_type(
                    RepaintBufferType::NewBuffer,
                ),
                controls,
            }
        })
    }
}

impl WindowAdapter for VisibilityWindow {
    /// Return the public Window linked to this owner.
    fn window(&self) -> &Window {
        &self.window
    }

    /// Record one bounded attempt and independently accept or reject the request.
    ///
    /// # Errors
    /// Returns a fixed error for armed failure or exhausted observation count.
    /// Failure preserves the last successfully observed adapter visibility.
    fn set_visible(&self, visible: bool) -> Result<(), PlatformError> {
        let (attempts, fails, message) = if visible {
            (
                &self.controls.show_attempts,
                &self.controls.show_fails,
                "Fixture show request rejected",
            )
        } else {
            (
                &self.controls.hide_attempts,
                &self.controls.hide_fails,
                "Fixture hide request rejected",
            )
        };
        let next = attempts
            .get()
            .checked_add(1)
            .ok_or_else(|| PlatformError::from("Fixture visibility counter exhausted"))?;
        attempts.set(next);
        if fails.get() {
            return Err(PlatformError::from(message));
        }
        self.controls.observed_visible.set(visible);
        Ok(())
    }

    /// Supply the fixed physical test viewport without host display assumptions.
    fn size(&self) -> PhysicalSize {
        self.controls
            .viewport
            .get()
            .unwrap_or(PhysicalSize::new(640, 780))
    }

    /// Use Slint's public supplied renderer; no internal `Renderer` is implemented.
    fn renderer(&self) -> &dyn Renderer {
        &self.renderer
    }
}

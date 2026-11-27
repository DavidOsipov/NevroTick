// SPDX-License-Identifier: GPL-3.0-only
//! Exercise updater receiving guards and preview transitions through the generated callbacks.
#![forbid(unsafe_code)]

use std::{cell::Cell, error::Error, rc::Rc, time::Duration};

use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{Platform, PlatformError, WindowAdapter};

use super::generated::UpdateStage;
use super::{AppWindow, Page, SlintHost, create, update_workflow};

/// Headless software platform for the ordinary portable Slint host.
struct Headless(Rc<Cell<Duration>>);

impl Platform for Headless {
    /// Supply an adapter without an installed native window or event loop.
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer))
    }

    /// Supply a deterministic clock to the real Slint timer dispatcher.
    fn duration_since_start(&self) -> Duration {
        self.0.get()
    }
}

/// A real Slint host and manually advanced preview ticks.
struct TestSession(SlintHost, Rc<Cell<Duration>>);

impl TestSession {
    /// Construct the portable host on this test's UI thread.
    ///
    /// # Errors
    ///
    /// Propagates a software platform or fixture construction failure.
    fn new() -> Result<Self, Box<dyn Error>> {
        let clock: Rc<Cell<Duration>> = Rc::new(Cell::new(Duration::ZERO));
        slint::platform::set_platform(Box::new(Headless(Rc::clone(&clock))))?;
        Ok(Self(create()?, clock))
    }

    /// Return the generated component exposed to the user-facing controls.
    const fn window(&self) -> &AppWindow {
        self.0.window()
    }

    /// Deliver one preview tick carrying the given operation generation.
    fn advance(&self, generation: i32) {
        update_workflow::advance_for_test(&self.0.state, self.window(), generation);
    }

    /// Run the toolkit dispatcher at the specified elapsed time.
    fn dispatch_at(&self, millis: u64) {
        self.1.set(Duration::from_millis(millis));
        slint::platform::update_timers_and_animations();
    }

    /// Check both the projected stage and the operation it belongs to.
    ///
    /// # Panics
    ///
    /// The projection must match the expected stage and generation.
    fn assert_step(&self, expected_stage: UpdateStage, expected_generation: i32) {
        let preview = self.window().get_update_preview();
        assert_eq!(preview.stage, expected_stage);
        assert_eq!(preview.generation, expected_generation);
    }

    /// Start a check on About and deliver its first tick.
    ///
    /// # Panics
    ///
    /// The check must open an offer on a positive generation.
    fn open_offer(&self) -> i32 {
        let window: &AppWindow = self.window();
        window.invoke_preview_update_check();
        let generation: i32 = window.get_update_preview().generation;
        assert!(generation > 0);
        self.assert_step(UpdateStage::Checking, generation);
        self.advance(generation);
        self.assert_step(UpdateStage::Offered, generation);
        generation
    }

    /// Compare the visible package facts against the frozen preview fixture.
    ///
    /// # Panics
    ///
    /// The current projection must show the expected versions, channel, and size.
    fn assert_package_facts(&self) {
        let preview = self.window().get_update_preview();
        assert_eq!(preview.installed.as_str(), "1.0.0");
        assert_eq!(preview.target.as_str(), "1.1.0");
        assert_eq!(preview.channel.as_str(), "Stable");
        assert_eq!(preview.size.as_str(), "12582912 B");
    }
}

/// Only the current offer generation on About may receive a choice.
///
/// # Errors
///
/// Propagates a software fixture construction failure.
///
/// # Panics
///
/// Idle and Checking must reject choices. An offer must reject a wrong generation.
#[test]
fn preview_update_callbacks_require_the_current_offer_on_about() -> Result<(), Box<dyn Error>> {
    let test: TestSession = TestSession::new()?;
    let window: &AppWindow = test.window();
    assert_eq!(window.get_current_page(), Page::Status);
    let initial_generation: i32 = window.get_update_preview().generation;
    test.assert_step(UpdateStage::Idle, initial_generation);

    window.invoke_preview_update_check();
    window.invoke_preview_update_yes(1);
    window.invoke_preview_update_later(1);
    test.advance(1);
    test.assert_step(UpdateStage::Idle, initial_generation);

    window.invoke_navigate(Page::About);
    assert_eq!(window.get_current_page(), Page::About);
    window.invoke_preview_update_yes(1);
    window.invoke_preview_update_later(1);
    test.assert_step(UpdateStage::Idle, initial_generation);

    window.invoke_preview_update_check();
    let generation: i32 = window.get_update_preview().generation;
    assert!(generation > 0);
    test.assert_step(UpdateStage::Checking, generation);
    window.invoke_preview_update_check();
    test.assert_step(UpdateStage::Checking, generation);
    window.invoke_preview_update_yes(generation);
    test.assert_step(UpdateStage::Checking, generation);
    window.invoke_preview_update_later(generation);
    test.assert_step(UpdateStage::Checking, generation);
    test.advance(-1);
    test.assert_step(UpdateStage::Checking, generation);

    test.advance(generation);
    test.assert_step(UpdateStage::Offered, generation);
    test.assert_package_facts();
    let other_generation: i32 = if generation == 1 { 2 } else { 1 };
    for wrong_generation in [0_i32, -1, other_generation] {
        window.invoke_preview_update_yes(wrong_generation);
        test.assert_step(UpdateStage::Offered, generation);
        window.invoke_preview_update_later(wrong_generation);
        test.assert_step(UpdateStage::Offered, generation);
    }
    Ok(())
}

/// Later dismisses the offer and a new check requires a fresh choice.
///
/// # Errors
///
/// Propagates a software fixture construction failure.
///
/// # Panics
///
/// Declined consent and ticks must have no effect on the next offer.
#[test]
fn preview_update_decline_invalidates_the_previous_offer() -> Result<(), Box<dyn Error>> {
    let test: TestSession = TestSession::new()?;
    let window: &AppWindow = test.window();
    window.invoke_navigate(Page::About);
    let declined_generation: i32 = test.open_offer();

    window.invoke_preview_update_later(declined_generation);
    let idle_generation: i32 = window.get_update_preview().generation;
    test.assert_step(UpdateStage::Idle, idle_generation);
    window.invoke_preview_update_yes(declined_generation);
    window.invoke_preview_update_later(declined_generation);
    test.advance(declined_generation);
    test.assert_step(UpdateStage::Idle, idle_generation);

    let current_generation: i32 = test.open_offer();
    assert_ne!(declined_generation, current_generation);
    window.invoke_preview_update_yes(declined_generation);
    test.assert_step(UpdateStage::Offered, current_generation);
    window.invoke_preview_update_later(declined_generation);
    test.assert_step(UpdateStage::Offered, current_generation);
    window.invoke_preview_update_yes(current_generation);
    test.assert_step(UpdateStage::Downloading, current_generation);
    Ok(())
}

/// Leaving About abandons both a pending check and an unconfirmed offer.
///
/// # Errors
///
/// Propagates a software fixture construction failure.
///
/// # Panics
///
/// An abandoned generation must be unable to advance a later check or reopen an offer.
#[test]
fn preview_update_navigation_discards_unconfirmed_generations() -> Result<(), Box<dyn Error>> {
    let test: TestSession = TestSession::new()?;
    let window: &AppWindow = test.window();
    window.invoke_navigate(Page::About);
    window.invoke_preview_update_check();
    let abandoned_check: i32 = window.get_update_preview().generation;
    test.assert_step(UpdateStage::Checking, abandoned_check);

    window.invoke_navigate(Page::Status);
    assert_eq!(window.get_current_page(), Page::Status);
    let idle_generation: i32 = window.get_update_preview().generation;
    test.assert_step(UpdateStage::Idle, idle_generation);
    test.advance(abandoned_check);
    test.assert_step(UpdateStage::Idle, idle_generation);

    window.invoke_navigate(Page::About);
    window.invoke_preview_update_check();
    let abandoned_offer: i32 = window.get_update_preview().generation;
    assert_ne!(abandoned_check, abandoned_offer);
    test.advance(abandoned_check);
    test.assert_step(UpdateStage::Checking, abandoned_offer);
    test.advance(abandoned_offer);
    test.assert_step(UpdateStage::Offered, abandoned_offer);

    window.invoke_navigate(Page::Status);
    let idle_generation: i32 = window.get_update_preview().generation;
    test.assert_step(UpdateStage::Idle, idle_generation);
    window.invoke_preview_update_yes(abandoned_offer);
    window.invoke_preview_update_later(abandoned_offer);
    test.advance(abandoned_offer);
    test.assert_step(UpdateStage::Idle, idle_generation);

    window.invoke_navigate(Page::About);
    test.assert_step(UpdateStage::Idle, idle_generation);
    let current_generation: i32 = test.open_offer();
    assert_ne!(abandoned_offer, current_generation);
    window.invoke_preview_update_yes(abandoned_offer);
    test.assert_step(UpdateStage::Offered, current_generation);
    Ok(())
}

/// One accepted Yes advances the preview through its three progress stages.
///
/// # Errors
///
/// Propagates a software fixture construction failure.
///
/// # Panics
///
/// The projected sequence, package facts, and terminal state must match the preview.
#[test]
fn preview_update_one_yes_runs_all_progress_stages() -> Result<(), Box<dyn Error>> {
    let test: TestSession = TestSession::new()?;
    let window: &AppWindow = test.window();
    window.invoke_navigate(Page::About);
    let generation: i32 = test.open_offer();
    test.assert_package_facts();

    window.invoke_preview_update_yes(generation);
    test.assert_step(UpdateStage::Downloading, generation);
    test.assert_package_facts();
    window.invoke_preview_update_yes(generation);
    test.assert_step(UpdateStage::Downloading, generation);
    window.invoke_preview_update_later(generation);
    test.assert_step(UpdateStage::Downloading, generation);
    test.advance(generation ^ 1);
    test.assert_step(UpdateStage::Downloading, generation);

    window.invoke_navigate(Page::Status);
    test.advance(generation);
    test.assert_step(UpdateStage::Verifying, generation);
    test.assert_package_facts();
    test.advance(generation);
    test.assert_step(UpdateStage::Installing, generation);
    test.assert_package_facts();
    test.advance(generation);
    test.assert_step(UpdateStage::Complete, generation);
    window.invoke_navigate(Page::About);
    test.advance(generation);
    window.invoke_preview_update_yes(generation);
    window.invoke_preview_update_later(generation);
    test.assert_step(UpdateStage::Complete, generation);
    assert_eq!(window.get_current_page(), Page::About);
    Ok(())
}

/// Only a scheduled timer can make progress when the toolkit clock advances.
///
/// # Errors
/// Propagates a software fixture construction failure.
///
/// # Panics
/// A missing, repeated or prematurely delivered timer changes the sequence.
#[test]
fn preview_update_uses_the_toolkit_timer_for_each_stage() -> Result<(), Box<dyn Error>> {
    let test: TestSession = TestSession::new()?;
    let window: &AppWindow = test.window();
    window.invoke_navigate(Page::About);
    window.invoke_preview_update_check();
    let generation: i32 = window.get_update_preview().generation;
    test.dispatch_at(399);
    test.assert_step(UpdateStage::Checking, generation);
    test.dispatch_at(400);
    test.assert_step(UpdateStage::Offered, generation);
    test.dispatch_at(800);
    test.assert_step(UpdateStage::Offered, generation);
    window.invoke_preview_update_yes(generation);
    test.dispatch_at(1199);
    test.assert_step(UpdateStage::Downloading, generation);
    test.dispatch_at(1200);
    test.assert_step(UpdateStage::Verifying, generation);
    test.dispatch_at(1600);
    test.assert_step(UpdateStage::Installing, generation);
    test.dispatch_at(2000);
    test.assert_step(UpdateStage::Complete, generation);
    test.dispatch_at(2400);
    test.assert_step(UpdateStage::Complete, generation);
    Ok(())
}

/// Cancel each pre-install phase through the generated receiver and retire its timer.
///
/// # Errors
/// Propagates a software fixture construction failure.
/// # Panics
/// A stale, hidden, repeated or post-install cancellation must not change the owner.
#[test]
fn preview_update_cancel_retires_only_the_current_preinstall_operation()
-> Result<(), Box<dyn Error>> {
    let test: TestSession = TestSession::new()?;
    let window: &AppWindow = test.window();
    window.invoke_navigate(Page::About);
    window.invoke_preview_update_check();
    let checking: i32 = window.get_update_preview().generation;
    window.invoke_preview_update_cancel(checking ^ 1);
    test.assert_step(UpdateStage::Checking, checking);
    window.invoke_preview_update_cancel(checking);
    test.assert_step(UpdateStage::Cancelled, checking);
    window.invoke_preview_update_cancel(checking);
    test.advance(checking);
    test.dispatch_at(500);
    test.assert_step(UpdateStage::Cancelled, checking);

    window.invoke_preview_update_check();
    let download: i32 = window.get_update_preview().generation;
    assert_ne!(download, checking);
    test.assert_step(UpdateStage::Checking, download);
    test.advance(checking);
    window.invoke_preview_update_cancel(checking);
    test.assert_step(UpdateStage::Checking, download);
    test.advance(download);
    window.invoke_preview_update_cancel(checking);
    test.assert_step(UpdateStage::Offered, download);
    window.invoke_preview_update_yes(download);
    test.assert_step(UpdateStage::Downloading, download);
    window.invoke_navigate(Page::Status);
    window.invoke_preview_update_cancel(download);
    test.assert_step(UpdateStage::Downloading, download);
    window.invoke_navigate(Page::About);
    window.invoke_preview_update_cancel(download);
    test.assert_step(UpdateStage::Cancelled, download);
    test.advance(download);
    test.dispatch_at(1000);
    test.assert_step(UpdateStage::Cancelled, download);

    let verify: i32 = test.open_offer();
    window.invoke_preview_update_yes(verify);
    test.advance(download);
    test.assert_step(UpdateStage::Downloading, verify);
    test.advance(verify);
    test.assert_step(UpdateStage::Verifying, verify);
    window.invoke_preview_update_cancel(download);
    test.assert_step(UpdateStage::Verifying, verify);
    window.invoke_preview_update_cancel(verify);
    test.assert_step(UpdateStage::Cancelled, verify);
    test.advance(verify);
    test.dispatch_at(1500);
    test.assert_step(UpdateStage::Cancelled, verify);

    let install: i32 = test.open_offer();
    window.invoke_preview_update_yes(install);
    test.advance(install);
    test.advance(install);
    test.assert_step(UpdateStage::Installing, install);
    window.invoke_preview_update_cancel(install);
    test.assert_step(UpdateStage::Installing, install);
    test.advance(install);
    test.assert_step(UpdateStage::Complete, install);
    Ok(())
}

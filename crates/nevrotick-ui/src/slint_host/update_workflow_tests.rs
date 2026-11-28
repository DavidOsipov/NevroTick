// SPDX-License-Identifier: GPL-3.0-only
//! Exercise updater receiving guards and preview transitions through the generated callbacks.
#![forbid(unsafe_code)]

use std::{cell::Cell, error::Error, rc::Rc, time::Duration};

use slint::ComponentHandle;
use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::platform::{Platform, PlatformError, WindowAdapter};

use super::generated::{FixtureCopy, UpdateCheckScenario, UpdatePreview, UpdateStage};
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
        let preview: UpdatePreview = self.window().get_update_preview();
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
        let preview: UpdatePreview = self.window().get_update_preview();
        assert_eq!(preview.installed.as_str(), "1.0.0");
        assert_eq!(preview.target.as_str(), "1.1.0");
        assert_eq!(preview.channel.as_str(), "Stable");
        assert_eq!(preview.size.as_str(), "12582912 B");
    }

    /// Confirm a check-only result retains neither offer facts nor timer custody.
    ///
    /// # Panics
    /// Any package facts or retained timer would permit a misleading result.
    fn assert_check_retired(&self) {
        let preview: UpdatePreview = self.window().get_update_preview();
        assert!(preview.installed.is_empty());
        assert!(preview.target.is_empty());
        assert!(preview.channel.is_empty());
        assert!(preview.size.is_empty());
        assert!(!update_workflow::has_timer_for_test(&self.0.state));
    }
}

/// Each unavailable eligibility cause remains distinct from a successful no-update check.
///
/// # Errors
/// Propagates a software fixture construction failure.
/// # Panics
/// Closed raw choices must yield their literal expected result, with no offer or consent.
#[test]
fn preview_update_check_results_are_distinct_and_offer_no_package() -> Result<(), Box<dyn Error>> {
    let test: TestSession = TestSession::new()?;
    let window: &AppWindow = test.window();
    assert_eq!(
        window.global::<FixtureCopy>().get_window_title().as_str(),
        "NevroTick — sample preview"
    );
    window.invoke_navigate(Page::About);
    let cases: [(i32, UpdateCheckScenario, UpdateStage); 6] = [
        (1, UpdateCheckScenario::NoUpdate, UpdateStage::NoUpdate),
        (2, UpdateCheckScenario::Offline, UpdateStage::Offline),
        (
            3,
            UpdateCheckScenario::MetadataExpired,
            UpdateStage::MetadataExpired,
        ),
        (
            4,
            UpdateCheckScenario::TimeUnavailable,
            UpdateStage::TimeUnavailable,
        ),
        (
            5,
            UpdateCheckScenario::IdentityUnknown,
            UpdateStage::IdentityUnknown,
        ),
        (
            6,
            UpdateCheckScenario::ExternallyManaged,
            UpdateStage::ExternallyManaged,
        ),
    ];
    let mut prior_generation: i32 = 0;
    for (choice, scenario, expected) in cases {
        window.invoke_preview_update_scenario(choice);
        assert_eq!(window.get_update_preview().scenario, scenario);
        window.invoke_preview_update_check();
        let generation: i32 = window.get_update_preview().generation;
        assert!(generation > prior_generation);
        test.assert_step(UpdateStage::Checking, generation);
        assert!(update_workflow::has_timer_for_test(&test.0.state));
        test.advance(generation);
        test.assert_step(expected, generation);
        test.assert_check_retired();
        // Ordinary fixture startup has no active catalog. Read the same copy
        // global as About so regeneration cannot silently blank its results.
        let (rendered_copy, expected_text): (slint::SharedString, &str) = match expected {
            UpdateStage::NoUpdate => (
                window.global::<FixtureCopy>().get_update_no_update(),
                "Sample check complete: no eligible update was found.",
            ),
            UpdateStage::Offline => (
                window.global::<FixtureCopy>().get_update_offline(),
                "Sample check unavailable: offline. Update eligibility was not established.",
            ),
            UpdateStage::MetadataExpired => (
                window.global::<FixtureCopy>().get_update_metadata_expired(),
                "Sample check unavailable: update metadata has expired. Update eligibility was not established.",
            ),
            UpdateStage::TimeUnavailable => (
                window.global::<FixtureCopy>().get_update_time_unavailable(),
                "Sample check unavailable: trusted time is unavailable. Update eligibility was not established.",
            ),
            UpdateStage::IdentityUnknown => (
                window.global::<FixtureCopy>().get_update_identity_unknown(),
                "Sample check unavailable: the installed product identity is unknown. Update eligibility was not established.",
            ),
            UpdateStage::ExternallyManaged => (
                window
                    .global::<FixtureCopy>()
                    .get_update_externally_managed(),
                "Sample updates are managed externally. No package was offered by this check.",
            ),
            _ => return Err("Unexpected sample result in the independent case table.".into()),
        };
        assert_eq!(rendered_copy.as_str(), expected_text);
        window.invoke_preview_update_yes(generation);
        window.invoke_preview_update_later(generation);
        window.invoke_preview_update_cancel(generation);
        test.advance(generation);
        test.assert_step(expected, generation);
        test.assert_check_retired();
        prior_generation = generation;
    }
    window.invoke_preview_update_scenario(0);
    let healthy_generation: i32 = test.open_offer();
    assert!(healthy_generation > prior_generation);
    test.assert_package_facts();
    Ok(())
}

/// Raw selections cannot bypass closed admission or the accepted About destination.
///
/// # Errors
/// Propagates a software fixture construction failure.
/// # Panics
/// Invalid and wrong-page selections must preserve the accepted scenario and generation.
#[test]
fn preview_update_scenario_rejects_invalid_and_wrong_page_choices() -> Result<(), Box<dyn Error>> {
    let test: TestSession = TestSession::new()?;
    let window: &AppWindow = test.window();
    window.invoke_preview_update_scenario(2);
    assert_eq!(
        window.get_update_preview().scenario,
        UpdateCheckScenario::Available
    );
    test.assert_step(UpdateStage::Idle, 0);
    window.invoke_navigate(Page::About);
    window.invoke_preview_update_scenario(3);
    for invalid in [i32::MIN, -1, 7, i32::MAX] {
        window.invoke_preview_update_scenario(invalid);
        assert_eq!(
            window.get_update_preview().scenario,
            UpdateCheckScenario::MetadataExpired
        );
        test.assert_step(UpdateStage::Idle, 0);
    }
    test.assert_check_retired();
    window.invoke_quit_requested();
    assert!(window.get_quit_warning_open());
    window.invoke_preview_update_scenario(0);
    window.invoke_preview_update_check();
    assert_eq!(
        window.get_update_preview().scenario,
        UpdateCheckScenario::MetadataExpired
    );
    test.assert_step(UpdateStage::Idle, 0);
    test.assert_check_retired();
    window.invoke_quit_cancel_requested();
    assert!(!window.get_quit_warning_open());
    window.invoke_preview_update_check();
    let generation: i32 = window.get_update_preview().generation;
    test.advance(generation);
    test.assert_step(UpdateStage::MetadataExpired, generation);
    window.invoke_navigate(Page::Status);
    window.invoke_preview_update_scenario(0);
    assert_eq!(
        window.get_update_preview().scenario,
        UpdateCheckScenario::MetadataExpired
    );
    test.assert_step(UpdateStage::MetadataExpired, generation);
    Ok(())
}

/// An operation freezes its selected fixture before any timer delivery.
///
/// # Errors
/// Propagates a software fixture construction failure.
/// # Panics
/// Busy selections must not replace pending checks, offers or consented progress.
#[test]
fn preview_update_scenario_is_frozen_through_active_work() -> Result<(), Box<dyn Error>> {
    let test: TestSession = TestSession::new()?;
    let window: &AppWindow = test.window();
    window.invoke_navigate(Page::About);
    window.invoke_preview_update_scenario(2);
    window.invoke_preview_update_check();
    let unavailable: i32 = window.get_update_preview().generation;
    window.invoke_preview_update_scenario(0);
    assert_eq!(
        window.get_update_preview().scenario,
        UpdateCheckScenario::Offline
    );
    test.advance(unavailable);
    test.assert_step(UpdateStage::Offline, unavailable);
    window.invoke_preview_update_scenario(0);
    window.invoke_preview_update_check();
    let generation: i32 = window.get_update_preview().generation;
    for expected in [
        UpdateStage::Checking,
        UpdateStage::Offered,
        UpdateStage::Downloading,
        UpdateStage::Verifying,
        UpdateStage::Installing,
    ] {
        test.assert_step(expected, generation);
        window.invoke_preview_update_scenario(6);
        assert_eq!(
            window.get_update_preview().scenario,
            UpdateCheckScenario::Available
        );
        test.assert_step(expected, generation);
        if expected == UpdateStage::Offered {
            window.invoke_preview_update_yes(generation);
        } else {
            test.advance(generation);
        }
    }
    test.assert_step(UpdateStage::Complete, generation);
    window.invoke_preview_update_scenario(6);
    assert_eq!(
        window.get_update_preview().scenario,
        UpdateCheckScenario::ExternallyManaged
    );
    test.assert_step(UpdateStage::Idle, generation);
    test.assert_check_retired();
    test.advance(generation);
    test.assert_step(UpdateStage::Idle, generation);
    Ok(())
}

/// An old cancelled check or offer cannot replace a fresh non-offer result.
///
/// # Errors
/// Propagates a software fixture construction failure.
/// # Panics
/// Late callbacks must leave the replacement result and its retired timer unchanged.
#[test]
fn preview_update_old_deliveries_preserve_replacement_check_result() -> Result<(), Box<dyn Error>> {
    let test: TestSession = TestSession::new()?;
    let window: &AppWindow = test.window();
    window.invoke_navigate(Page::About);
    let old_offer: i32 = test.open_offer();
    window.invoke_preview_update_later(old_offer);
    window.invoke_preview_update_check();
    let old_check: i32 = window.get_update_preview().generation;
    window.invoke_preview_update_cancel(old_check);
    window.invoke_preview_update_scenario(5);
    window.invoke_preview_update_check();
    let generation: i32 = window.get_update_preview().generation;
    for old in [old_offer, old_check] {
        test.advance(old);
        window.invoke_preview_update_yes(old);
        window.invoke_preview_update_later(old);
        window.invoke_preview_update_cancel(old);
        test.assert_step(UpdateStage::Checking, generation);
    }
    test.advance(generation);
    for old in [old_offer, old_check] {
        test.advance(old);
        window.invoke_preview_update_yes(old);
        window.invoke_preview_update_later(old);
        window.invoke_preview_update_cancel(old);
        test.assert_step(UpdateStage::IdentityUnknown, generation);
        test.assert_check_retired();
    }
    Ok(())
}

/// The actual toolkit timer retires every check-only outcome before a healthy retry.
///
/// # Errors
/// Propagates a software fixture construction failure.
/// # Panics
/// Check-only results must arrive once at their deadline and release timer custody.
#[test]
fn preview_update_check_result_timers_retire_before_retry() -> Result<(), Box<dyn Error>> {
    let test: TestSession = TestSession::new()?;
    let window: &AppWindow = test.window();
    window.invoke_navigate(Page::About);
    let cases: [(i32, UpdateStage); 6] = [
        (1, UpdateStage::NoUpdate),
        (2, UpdateStage::Offline),
        (3, UpdateStage::MetadataExpired),
        (4, UpdateStage::TimeUnavailable),
        (5, UpdateStage::IdentityUnknown),
        (6, UpdateStage::ExternallyManaged),
    ];
    let mut elapsed: u64 = 0;
    for (choice, expected) in cases {
        window.invoke_preview_update_scenario(choice);
        window.invoke_preview_update_check();
        let generation: i32 = window.get_update_preview().generation;
        test.dispatch_at(elapsed + 399);
        test.assert_step(UpdateStage::Checking, generation);
        test.dispatch_at(elapsed + 400);
        test.assert_step(expected, generation);
        test.assert_check_retired();
        test.dispatch_at(elapsed + 800);
        test.assert_step(expected, generation);
        test.assert_check_retired();
        elapsed += 800;
    }
    window.invoke_preview_update_scenario(0);
    window.invoke_preview_update_check();
    let generation: i32 = window.get_update_preview().generation;
    test.dispatch_at(elapsed + 400);
    test.assert_step(UpdateStage::Offered, generation);
    test.assert_package_facts();
    assert!(!update_workflow::has_timer_for_test(&test.0.state));
    Ok(())
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

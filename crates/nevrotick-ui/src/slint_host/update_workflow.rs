// SPDX-License-Identifier: GPL-3.0-only
//! UI-thread ownership of the bounded About update simulation.

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    time::Duration,
};

use slint::{ComponentHandle, Timer, TimerMode};

use super::{
    AppWindow, BindingState, ProjectionGuard, SessionPage,
    generated::{UpdatePreview, UpdateStage},
};

/// One fixed sample release version with three numeric components.
#[derive(Clone, Copy)]
struct SampleVersion([u16; 3]);

impl SampleVersion {
    /// Render the exact sample identity as a display value.
    fn display(self) -> slint::SharedString {
        let [major, minor, patch] = self.0;
        format!("{major}.{minor}.{patch}").into()
    }
}

/// Channel identity for the approved fixture.
#[derive(Clone, Copy)]
enum SampleChannel {
    /// The fixed Stable channel.
    Stable,
}

impl SampleChannel {
    /// Return the display name of the sample channel.
    const fn display(self) -> &'static str {
        match self {
            Self::Stable => "Stable",
        }
    }
}

/// The conditions described by the single reviewed offer message.
#[derive(Clone, Copy)]
enum SampleTerms {
    /// An unmetered, non-roaming connection without external management.
    UnmeteredNonRoamingUnmanaged,
}

/// The material fixture terms are fixed by the code and the dedicated offer copy.
#[derive(Clone, Copy)]
struct SampleOffer {
    /// Currently installed sample version.
    installed: SampleVersion,
    /// Complete package version offered once for this generation.
    target: SampleVersion,
    /// Release channel attached to this target.
    channel: SampleChannel,
    /// Exact sample package size in bytes.
    bytes: u32,
    /// Material conditions bound to the offered copy and decision.
    terms: SampleTerms,
}

impl SampleOffer {
    /// Immutable package facts and conditions for the checked fixture.
    const CHECKED: Self = Self {
        installed: SampleVersion([1, 0, 0]),
        target: SampleVersion([1, 1, 0]),
        channel: SampleChannel::Stable,
        bytes: 12_582_912,
        terms: SampleTerms::UnmeteredNonRoamingUnmanaged,
    };

    /// Select the offer view whose text discloses these exact conditions.
    const fn offer_stage(self) -> UpdateStage {
        match self.terms {
            SampleTerms::UnmeteredNonRoamingUnmanaged => UpdateStage::Offered,
        }
    }
}

/// Consent consumes the offer and carries its identity through progress.
#[derive(Default)]
enum Phase {
    /// Ready for a fresh check with no outstanding offer.
    #[default]
    Idle,
    /// One pending check for the active About visit.
    Checking,
    /// Unconsumed material terms for the current visit.
    Offered(SampleOffer),
    /// Consent has been consumed and simulated download is active.
    Downloading(SampleOffer),
    /// Simulated verification of the same package is active.
    Verifying(SampleOffer),
    /// Simulated installation of the same package is active.
    Installing(SampleOffer),
    /// All simulated steps completed and the timer has retired.
    Complete,
    /// A pre-install operation was cancelled and its timer retired.
    Cancelled,
}

/// Identify which arm owns a tick. The check arm cannot advance consented work.
#[derive(Clone, Copy)]
enum Arm {
    /// Opens the unconfirmed offer in the active visit.
    Check,
    /// Advances steps only after consent.
    Progress,
}

/// The accepted About-only state and sole pending timer for one UI session.
#[derive(Default)]
pub(super) struct UpdatePreviewState {
    /// Monotonic operation identity. Zero is reserved for session construction.
    generation: i32,
    /// Sole accepted phase and current offer custody.
    phase: Phase,
    /// At most one scheduled arm, with weak references to the UI owner.
    timer: Option<Timer>,
}

impl UpdatePreviewState {
    /// Project current facts from the same offer value that consent consumes.
    fn projection(&self) -> UpdatePreview {
        let (stage, offer): (UpdateStage, Option<SampleOffer>) = match self.phase {
            Phase::Idle => (UpdateStage::Idle, None),
            Phase::Checking => (UpdateStage::Checking, None),
            Phase::Offered(offer) => (offer.offer_stage(), Some(offer)),
            Phase::Downloading(offer) => (UpdateStage::Downloading, Some(offer)),
            Phase::Verifying(offer) => (UpdateStage::Verifying, Some(offer)),
            Phase::Installing(offer) => (UpdateStage::Installing, Some(offer)),
            Phase::Complete => (UpdateStage::Complete, None),
            Phase::Cancelled => (UpdateStage::Cancelled, None),
        };
        let mut preview: UpdatePreview = UpdatePreview {
            generation: self.generation,
            stage,
            ..UpdatePreview::default()
        };
        if let Some(offer) = offer {
            preview.installed = offer.installed.display();
            preview.target = offer.target.display();
            preview.channel = offer.channel.display().into();
            preview.size = format!("{} B", offer.bytes).into();
        }
        preview
    }
}

/// Install receiver bindings with weak window and owner handles.
pub(super) fn install(window: &AppWindow, state: &Rc<RefCell<BindingState>>) {
    let weak: slint::Weak<AppWindow> = window.as_weak();
    let owner: Weak<RefCell<BindingState>> = Rc::downgrade(state);
    window.on_preview_update_check(move || {
        if let (Some(window), Some(state)) = (weak.upgrade(), owner.upgrade()) {
            check(&window, &state);
        }
    });
    let weak: slint::Weak<AppWindow> = window.as_weak();
    let owner: Weak<RefCell<BindingState>> = Rc::downgrade(state);
    window.on_preview_update_yes(move |generation| {
        if let (Some(window), Some(state)) = (weak.upgrade(), owner.upgrade()) {
            resolve(&window, &state, generation, Decision::Accept);
        }
    });
    let weak: slint::Weak<AppWindow> = window.as_weak();
    let owner: Weak<RefCell<BindingState>> = Rc::downgrade(state);
    window.on_preview_update_later(move |generation| {
        if let (Some(window), Some(state)) = (weak.upgrade(), owner.upgrade()) {
            resolve(&window, &state, generation, Decision::Dismiss);
        }
    });
    let weak: slint::Weak<AppWindow> = window.as_weak();
    let owner: Weak<RefCell<BindingState>> = Rc::downgrade(state);
    window.on_preview_update_cancel(move |generation| {
        if let (Some(window), Some(state)) = (weak.upgrade(), owner.upgrade()) {
            cancel(&window, &state, generation);
        }
    });
}

/// Cancel only the current pre-install sample and retire its timer.
fn cancel(window: &AppWindow, state: &Rc<RefCell<BindingState>>, generation: i32) {
    let Some(_guard) = ProjectionGuard::enter(state) else {
        return;
    };
    if !offer_visible(window) {
        return;
    }
    let (preview, timer): (UpdatePreview, Option<Timer>) = {
        let mut owner: std::cell::RefMut<'_, BindingState> = state.borrow_mut();
        if owner.navigation.page() != SessionPage::About
            || owner.updates.generation != generation
            || !matches!(
                owner.updates.phase,
                Phase::Checking | Phase::Downloading(_) | Phase::Verifying(_)
            )
        {
            return;
        }
        owner.updates.phase = Phase::Cancelled;
        (owner.updates.projection(), owner.updates.timer.take())
    };
    drop(timer);
    apply(window, preview);
}

/// Invalidate an unconfirmed check or offer when About leaves view.
pub(super) fn leave_about(window: &AppWindow, state: &RefCell<BindingState>) {
    let retired: Option<(Option<Timer>, UpdatePreview)> = {
        let mut owner: std::cell::RefMut<'_, BindingState> = state.borrow_mut();
        let updates: &mut UpdatePreviewState = &mut owner.updates;
        if matches!(updates.phase, Phase::Checking | Phase::Offered(_)) {
            updates.phase = Phase::Idle;
            Some((updates.timer.take(), updates.projection()))
        } else {
            None
        }
    };
    if let Some((timer, preview)) = retired {
        drop(timer);
        apply(window, preview);
    }
}

/// The Windows receiver also requires the actual window to be visible.
fn offer_visible(window: &AppWindow) -> bool {
    #[cfg(windows)]
    {
        window.window().is_visible()
    }
    #[cfg(not(windows))]
    {
        let _ = window;
        true
    }
}

/// Start one fresh check while About is the accepted navigation destination.
fn check(window: &AppWindow, state: &Rc<RefCell<BindingState>>) {
    let Some(_guard) = ProjectionGuard::enter(state) else {
        return;
    };
    if !offer_visible(window) {
        return;
    }
    let preview: UpdatePreview = {
        let mut owner: std::cell::RefMut<'_, BindingState> = state.borrow_mut();
        if owner.navigation.page() != SessionPage::About
            || !matches!(
                owner.updates.phase,
                Phase::Idle | Phase::Complete | Phase::Cancelled
            )
        {
            return;
        }
        let Some(generation) = owner.updates.generation.checked_add(1) else {
            return;
        };
        owner.updates.generation = generation;
        owner.updates.phase = Phase::Checking;
        owner.updates.projection()
    };
    arm(window, state, preview.generation, Arm::Check);
    apply(window, preview);
}

/// The two permitted transitions from an active, unconsumed offer.
#[derive(Clone, Copy)]
enum Decision {
    /// Consume the current offer and start simulated download.
    Accept,
    /// Close the current offer without consent.
    Dismiss,
}

/// Reject stale or wrong-page decisions before changing the accepted state.
fn resolve(
    window: &AppWindow,
    state: &Rc<RefCell<BindingState>>,
    generation: i32,
    decision: Decision,
) {
    let Some(_guard) = ProjectionGuard::enter(state) else {
        return;
    };
    if !offer_visible(window) {
        return;
    }
    let preview: UpdatePreview = {
        let mut owner: std::cell::RefMut<'_, BindingState> = state.borrow_mut();
        if owner.navigation.page() != SessionPage::About || owner.updates.generation != generation {
            return;
        }
        let Phase::Offered(offer) = owner.updates.phase else {
            return;
        };
        owner.updates.phase = match decision {
            Decision::Accept => Phase::Downloading(offer),
            Decision::Dismiss => Phase::Idle,
        };
        owner.updates.projection()
    };
    if matches!(decision, Decision::Accept) {
        arm(window, state, generation, Arm::Progress);
    }
    apply(window, preview);
}

/// Schedule the admitted check or progress arm. The caller holds the guard.
fn arm(window: &AppWindow, state: &Rc<RefCell<BindingState>>, generation: i32, arm: Arm) {
    const STEP: Duration = Duration::from_millis(400);
    let weak: slint::Weak<AppWindow> = window.as_weak();
    let owner: Weak<RefCell<BindingState>> = Rc::downgrade(state);
    let timer: Timer = Timer::default();
    timer.start(TimerMode::Repeated, STEP, move || {
        if let (Some(window), Some(state)) = (weak.upgrade(), owner.upgrade()) {
            tick(&state, &window, generation, arm);
        }
    });
    state.borrow_mut().updates.timer = Some(timer);
}

/// Publish an offer only in view. Already-consented progress survives navigation.
fn tick(state: &RefCell<BindingState>, window: &AppWindow, generation: i32, arm: Arm) {
    let Some(_guard) = ProjectionGuard::enter(state) else {
        return;
    };
    let check_arm: bool = matches!(arm, Arm::Check);
    let visible: bool = !check_arm || offer_visible(window);
    let (preview, timer): (UpdatePreview, Option<Timer>) = {
        let mut owner: std::cell::RefMut<'_, BindingState> = state.borrow_mut();
        if owner.updates.generation != generation
            || (check_arm && owner.navigation.page() != SessionPage::About)
        {
            return;
        }
        let updates: &mut UpdatePreviewState = &mut owner.updates;
        updates.phase = match (&updates.phase, arm) {
            (Phase::Checking, Arm::Check) if visible => Phase::Offered(SampleOffer::CHECKED),
            (Phase::Checking, Arm::Check) => Phase::Idle,
            (Phase::Downloading(offer), Arm::Progress) => Phase::Verifying(*offer),
            (Phase::Verifying(offer), Arm::Progress) => Phase::Installing(*offer),
            (Phase::Installing(_), Arm::Progress) => Phase::Complete,
            _ => return,
        };
        let timer: Option<Timer> = if matches!(
            updates.phase,
            Phase::Idle | Phase::Offered(_) | Phase::Complete
        ) {
            updates.timer.take()
        } else {
            None
        };
        (updates.projection(), timer)
    };
    drop(timer);
    apply(window, preview);
}

/// Publish one complete About projection after releasing all state borrows.
fn apply(window: &AppWindow, preview: UpdatePreview) {
    window.set_update_preview(preview);
}

/// Drive the same receiver without wall-clock waiting in headless tests.
#[cfg(all(test, not(windows)))]
pub(super) fn advance_for_test(
    state: &Rc<RefCell<BindingState>>,
    window: &AppWindow,
    generation: i32,
) {
    let arm: Arm = if matches!(state.borrow().updates.phase, Phase::Checking) {
        Arm::Check
    } else {
        Arm::Progress
    };
    tick(state, window, generation, arm);
}

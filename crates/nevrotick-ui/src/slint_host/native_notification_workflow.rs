// SPDX-License-Identifier: GPL-3.0-only
//! Private UI-thread receiver for explicit local native test notifications.
//!
//! The ordinary fixture host leaves this owner dormant. Only maintained native
//! startup may install it with an explicit identity/content mode. Neither this
//! owner nor the fixture status feed can produce a real problem notification.
//! Native callbacks retain only the adapter's bounded post-only bridge. The
//! existing root guard, shell controller and recovery navigator keep authority.
#![forbid(unsafe_code)]

use std::cell::RefCell;

use super::catalog_workflow::{FeedbackSlot, write};
use super::{AppWindow, BindingState, ProjectionGuard};
use crate::localization::UiText;
use crate::notifications_ui::{
    NotificationBlock, ObserverPresence, UnavailableNotificationPort, WindowsNotificationSetting,
};
use crate::presentation::Page;
use crate::shell::{Observation, SettingsSection};

#[cfg(any(windows, test))]
use crate::notifications_ui::FixtureNotice;
#[cfg(windows)]
use crate::notifications_ui::FixtureNoticeContext;
#[cfg(any(windows, test))]
use crate::notifications_ui::native_delivery::{NoticeError, PreparedTestNotice};

#[cfg(windows)]
use crate::localization::Snapshot;
#[cfg(windows)]
use crate::native_notifications_windows::{
    CloseOutcome, IdentityMode, NativeFailure, RequestOutcome, WindowsToastPort,
};
#[cfg(windows)]
use crate::native_tray::UiWake;
#[cfg(windows)]
use crate::notifications_ui::native_delivery::{ActivationResolution, Submission};
#[cfg(windows)]
use crate::shell::AdapterAvailability;
#[cfg(windows)]
use std::{rc::Rc, sync::Arc};

/// Native content selection is maintained startup code, never user/catalog data.
#[cfg(any(windows, test))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) enum NoticeContent {
    /// A real active approved snapshot is required; absence cannot use English.
    #[default]
    ApprovedCatalogOnly,
    /// Explicitly labelled engineering startup may use fixed sample words only
    /// when no active catalog exists. A failed active catalog never falls back.
    EngineeringExample,
}

/// One-way native lifetime. Preference changes cannot recreate/reset this owner.
#[cfg(any(windows, test))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum Lifetime {
    /// Ordinary `create()` has no native initialization or effects.
    #[default]
    Dormant,
    /// One startup attempt failed; GUI use continues without native delivery.
    Unavailable,
    /// One retained UI-thread adapter may accept explicit test requests.
    Running,
    /// A close attempt left exact retained cleanup obligations uncertain.
    Partial,
    /// Explicit close confirmed all owned adapter cleanup.
    Stopped,
}

/// No window/component or shell controller is retained by this child owner.
pub(super) struct NotificationBindingState {
    /// Adapter observations are private; view properties cannot replace them.
    windows: Observation<WindowsNotificationSetting>,
    /// Running means an actual UI/tray observer, independently of preferences.
    observer: Observation<ObserverPresence>,
    /// Only explicit native startup can construct a real platform owner.
    #[cfg(windows)]
    platform: NativeState,
}

impl Default for NotificationBindingState {
    /// A fixture has unknown native observations and performs no OS operation.
    fn default() -> Self {
        Self {
            windows: Observation::Unknown,
            observer: Observation::Unknown,
            #[cfg(windows)]
            platform: NativeState::default(),
        }
    }
}

/// UI-thread-only port custody; callbacks never own this state or its `RefCell`.
#[cfg(windows)]
#[derive(Default)]
struct NativeState {
    /// Presence is independent of the native tray's availability.
    port: Option<WindowsToastPort>,
    /// One fixed checked test identity, retained across toggles and dismissals.
    notice: Option<FixtureNotice>,
    /// Catalog absence requires an explicitly selected engineering mode.
    content: NoticeContent,
    /// Partial cleanup denies requests and a normal successful UI exit.
    lifetime: Lifetime,
    /// A consumed ID remains consumed across toggles, cleanup and repeated input.
    attempt: Attempt,
}

/// Sticky sample attempt meaning, independent of button enablement and wording.
#[cfg(any(windows, test))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum Attempt {
    /// No actual Show has consumed this session's fixed test identity.
    #[default]
    None,
    /// Windows accepted Show; display and reading remain unconfirmed.
    Accepted,
    /// Show or its later failure left the request outcome unconfirmed.
    Indeterminate,
    /// A duplicate preserves the earlier accepted result and consumes nothing.
    CoalescedAccepted,
    /// A duplicate preserves the earlier uncertain result and consumes nothing.
    CoalescedIndeterminate,
}

#[cfg(any(windows, test))]
impl Attempt {
    /// Preserve the first attempt's certainty when a repeated ID is coalesced.
    const fn coalesced(self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Accepted | Self::CoalescedAccepted => Self::CoalescedAccepted,
            Self::Indeterminate | Self::CoalescedIndeterminate => Self::CoalescedIndeterminate,
        }
    }

    /// Return closed factory selectors and exact corresponding fixture wording.
    const fn text(self) -> Option<(bool, bool, &'static str)> {
        match self {
            Self::None => None,
            Self::Accepted => Some((
                true,
                false,
                "Windows accepted the test request. A banner or reading is not confirmed.",
            )),
            Self::Indeterminate => Some((
                false,
                false,
                "The test request outcome is unconfirmed. A banner or reading is not confirmed.",
            )),
            Self::CoalescedAccepted => Some((
                true,
                true,
                "Windows accepted the test request. A banner or reading is not confirmed. This test will not be submitted again during this app session.",
            )),
            Self::CoalescedIndeterminate => Some((
                false,
                true,
                "The test request outcome is unconfirmed. A banner or reading is not confirmed. This test will not be submitted again during this app session.",
            )),
        }
    }
}

/// Fixed integration error; no external/native detail is retained or displayed.
#[cfg(windows)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum WorkflowError {
    /// Only one initialized idle owner may attempt native installation once.
    Busy,
    /// A code-owned nonzero test identity could not be constructed.
    Identity,
    /// Native open failed; absence remains explicit rather than simulated Ready.
    Native(NativeFailure),
    /// Active/required catalog text could not meet the actual native sink rules.
    Copy(NoticeError),
}

/// Fresh private receiver facts; every axis is independent of view properties.
#[derive(Clone, Copy)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "Immutable facts are independent receiving axes; no mutable workflow state or mutually exclusive variants are encoded"
)]
struct ReceiverFacts {
    /// Full callback installation has completed.
    initialized: bool,
    /// No existing synchronous projection owns the receiving transition.
    idle: bool,
    /// Catalog and raw-admission invariants have not failed.
    healthy: bool,
    /// No private shell/recovery/source/review modal currently owns interaction.
    modal_free: bool,
    /// The actual navigation owner is on shared Settings.
    settings: bool,
    /// The actual named destination is Notifications, not a forged signed view.
    notifications: bool,
}

impl ReceiverFacts {
    /// Capture only code-owned state before taking the receiving projection guard.
    fn from_owner(owner: &BindingState) -> Self {
        Self {
            initialized: owner.initialized,
            idle: !owner.projecting,
            healthy: !owner.copy.failed() && !owner.raw_failed(),
            modal_free: !owner.shell.blocks_other_requests()
                && owner.recovery_leave.is_none()
                && owner.current_inline_owner().is_none(),
            settings: owner.navigation.page() == Page::Settings,
            notifications: owner.shell.settings_section() == SettingsSection::Notifications,
        }
    }

    /// All independent private guards must agree before even preparing a test.
    const fn admitted(self) -> bool {
        self.initialized
            && self.idle
            && self.healthy
            && self.modal_free
            && self.settings
            && self.notifications
    }
}

/// Select an admitted active snapshot, or one explicit absent-catalog fixture.
///
/// # Errors
/// An active preparation error is returned unchanged. No rejected/incomplete
/// external catalog can make fixed engineering English acceptable by fallback.
#[cfg(any(windows, test))]
fn choose_prepared(
    active: Result<Option<PreparedTestNotice>, NoticeError>,
    notice: FixtureNotice,
    content: NoticeContent,
) -> Result<PreparedTestNotice, NoticeError> {
    match active? {
        Some(prepared) => Ok(prepared),
        None if content == NoticeContent::EngineeringExample => {
            PreparedTestNotice::engineering_example(notice)
        }
        None => Err(NoticeError::Catalog(crate::localization::Error::Schema)),
    }
}

/// Common callback receiver for actual button clicks and direct invocations.
///
/// The fixture path remains effect-free. Native requests separately re-observe
/// OS permission and actual observer presence before the adapter's own guards.
pub(super) fn request_test(weak: &slint::Weak<AppWindow>, state: &RefCell<BindingState>) {
    let Some(window) = weak.upgrade() else {
        return;
    };
    if !ReceiverFacts::from_owner(&state.borrow()).admitted() {
        return;
    }
    let Some(_projection) = ProjectionGuard::enter(state) else {
        return;
    };
    #[cfg(windows)]
    {
        if state.borrow().notifications.platform.lifetime == Lifetime::Running {
            request_native(&window, state);
            return;
        }
        if state.borrow().notifications.platform.attempt != Attempt::None {
            attempt_feedback(&window, state);
            return;
        }
    }
    let block = {
        let owner = state.borrow();
        UnavailableNotificationPort::request_test(
            &owner.preferences,
            owner.notifications.windows,
            owner.notifications.observer,
        )
    };
    blocked(&window, state, block);
}

/// Render only an established no-submission rejection through typed feedback.
fn blocked(window: &AppWindow, state: &RefCell<BindingState>, block: NotificationBlock) {
    let fixture = if block == NotificationBlock::NativeAdapterUnavailable {
        block.text().to_owned()
    } else {
        format!("{} No Windows notification was submitted.", block.text())
    };
    write(
        window,
        state,
        FeedbackSlot::NotificationFeedback,
        Some(UiText::notification_feedback(block)),
        fixture.into(),
    );
}

/// Install at most once after host initialization, with no state borrow in `WinRT`.
///
/// # Errors
/// Installation failure leaves ordinary GUI use available and native unavailable.
/// The caller must not reinterpret it as tray failure or overall startup success.
#[cfg(windows)]
pub(super) fn install(
    window: &AppWindow,
    state: &Rc<RefCell<BindingState>>,
    identity: IdentityMode,
    content: NoticeContent,
    wake: Arc<dyn UiWake>,
) -> Result<(), WorkflowError> {
    {
        let owner = state.borrow();
        if !owner.initialized
            || owner.projecting
            || owner.copy.failed()
            || owner.raw_failed()
            || owner.shell.blocks_other_requests()
            || owner.notifications.platform.lifetime != Lifetime::Dormant
        {
            return Err(WorkflowError::Busy);
        }
    }
    let _projection = ProjectionGuard::enter_native(state).ok_or(WorkflowError::Busy)?;
    let notice =
        FixtureNotice::new(1, FixtureNoticeContext::TestExample).ok_or(WorkflowError::Identity)?;
    // Mark the one startup attempt before entering native code. A callback can
    // enqueue only a later wake; it cannot see an in-progress Ready platform.
    {
        let mut owner = state.borrow_mut();
        owner.notifications.platform.lifetime = Lifetime::Unavailable;
        owner.notifications.platform.content = content;
        owner.notifications.platform.notice = Some(notice);
    }
    let preparation = {
        let owner = state.borrow();
        choose_prepared(
            owner.copy.prepare_native_notification(notice),
            notice,
            content,
        )
    };
    if let Err(error) = preparation {
        refresh(window, state);
        return Err(WorkflowError::Copy(error));
    }
    let opened = WindowsToastPort::open(identity, wake);
    let result = match opened {
        Ok(port) => {
            let mut owner = state.borrow_mut();
            owner.notifications.platform.port = Some(port);
            owner.notifications.platform.lifetime = Lifetime::Running;
            Ok(())
        }
        Err(error) => Err(WorkflowError::Native(error)),
    };
    refresh(window, state);
    result
}

/// Validate a complete candidate catalog before its shared owner commits it.
///
/// # Errors
/// A running native owner requires all title/body/action fields to render from
/// this same snapshot within their real native bounds. Rejection preserves the
/// previous active catalog, view and retained notification; it performs no COM.
#[cfg(windows)]
pub(super) fn prepare_catalog(
    owner: &BindingState,
    snapshot: &Snapshot<'_>,
) -> Result<(), NoticeError> {
    if owner.notifications.platform.lifetime != Lifetime::Running {
        return Ok(());
    }
    let notice = owner
        .notifications
        .platform
        .notice
        .ok_or(NoticeError::Catalog(crate::localization::Error::Schema))?;
    PreparedTestNotice::from_snapshot(snapshot, notice).map(|_| ())
}

/// Move adapter custody out of the `RefCell` before any COM/native call.
#[cfg(windows)]
fn take_port(state: &RefCell<BindingState>) -> Option<WindowsToastPort> {
    state.borrow_mut().notifications.platform.port.take()
}

/// Restore unique UI-thread custody; no native callback can fill this slot.
#[cfg(windows)]
fn restore_port(state: &RefCell<BindingState>, port: WindowsToastPort) {
    state.borrow_mut().notifications.platform.port = Some(port);
}

/// Observe separate native availability, Windows permission and observer axes.
///
/// Called under the host's native projection guard, except once during explicit
/// startup before ordinary interaction. Each native call runs without a borrow.
#[cfg(windows)]
pub(super) fn refresh(window: &AppWindow, state: &RefCell<BindingState>) {
    let port = take_port(state);
    let (availability, windows) = port.as_ref().map_or(
        (AdapterAvailability::Unavailable, Observation::Unknown),
        |port| {
            let availability = port.availability();
            let windows = if availability == AdapterAvailability::Ready {
                port.windows_setting()
            } else {
                Observation::Unknown
            };
            (availability, windows)
        },
    );
    if let Some(port) = port {
        restore_port(state, port);
    }
    let observer = super::native_workflow::observer_presence(window, state);
    let test_available = {
        let mut owner = state.borrow_mut();
        owner.notifications.windows = windows;
        owner.notifications.observer = observer;
        let context = ReceiverFacts::from_owner(&owner);
        context.initialized
            && context.healthy
            && context.modal_free
            && context.settings
            && context.notifications
            && availability == AdapterAvailability::Ready
            && owner.notifications.platform.lifetime == Lifetime::Running
            && owner.notifications.platform.attempt == Attempt::None
            && owner.preferences.notifications_enabled()
            && matches!(
                windows,
                Observation::Current(WindowsNotificationSetting::Allowed)
            )
            && matches!(observer, Observation::Current(ObserverPresence::Running))
    };
    super::shell_workflow::observe_native_notifications(
        window,
        state,
        availability,
        windows,
        observer,
        test_available,
    );
}

/// Prepare and submit only this owner's stable explicit sample under its guard.
#[cfg(windows)]
fn request_native(window: &AppWindow, state: &RefCell<BindingState>) {
    // The caller holds ProjectionGuard and just rechecked private page/modals.
    let observer = super::native_workflow::observer_presence(window, state);
    let prepared = {
        let owner = state.borrow();
        let Some(notice) = owner.notifications.platform.notice else {
            return;
        };
        choose_prepared(
            owner.copy.prepare_native_notification(notice),
            notice,
            owner.notifications.platform.content,
        )
    };
    let Ok(prepared) = prepared else {
        // Installation and atomic catalog publication already preflighted this
        // same native sink. A later private active-copy mismatch is an internal
        // invariant failure, never a Windows-permission error or English fallback.
        super::catalog_workflow::invariant_failed(window, state);
        return;
    };
    let preferences = state.borrow().preferences.clone();
    let Some(mut port) = take_port(state) else {
        no_new_attempt(window, state, NotificationBlock::NativeAdapterUnavailable);
        return;
    };
    let outcome = port.request_test(&prepared, &preferences, observer);
    restore_port(state, port);
    match outcome {
        RequestOutcome::Blocked(block) => no_new_attempt(window, state, block),
        RequestOutcome::NotSubmitted(_) => {
            no_new_attempt(window, state, NotificationBlock::WindowsUnavailable);
        }
        RequestOutcome::Attempted(Submission::Indeterminate) => {
            state.borrow_mut().notifications.platform.attempt = Attempt::Indeterminate;
            attempt_feedback(window, state);
        }
        RequestOutcome::Attempted(Submission::AcceptedByOs) => {
            state.borrow_mut().notifications.platform.attempt = Attempt::Accepted;
            attempt_feedback(window, state);
        }
        RequestOutcome::Coalesced => {
            let mut owner = state.borrow_mut();
            owner.notifications.platform.attempt = owner.notifications.platform.attempt.coalesced();
            drop(owner);
            attempt_feedback(window, state);
        }
    }
    refresh(window, state);
}

/// A later refusal cannot erase or contradict a previously consumed attempt.
#[cfg(windows)]
fn no_new_attempt(window: &AppWindow, state: &RefCell<BindingState>, block: NotificationBlock) {
    if state.borrow().notifications.platform.attempt == Attempt::None {
        blocked(window, state, block);
    } else {
        attempt_feedback(window, state);
    }
}

/// Keep one typed sticky meaning so catalog replay never reads displayed words.
#[cfg(windows)]
fn attempt_feedback(window: &AppWindow, state: &RefCell<BindingState>) {
    let Some((accepted, coalesced, fixture)) = state.borrow().notifications.platform.attempt.text()
    else {
        return;
    };
    let Ok(message) = UiText::notification_test_feedback(accepted, coalesced) else {
        super::catalog_workflow::invariant_failed(window, state);
        return;
    };
    write(
        window,
        state,
        FeedbackSlot::NotificationFeedback,
        Some(message),
        fixture.into(),
    );
}

/// Drain at most one bounded activation under the native transition guard.
///
/// The caller must release that guard, then recheck current root ownership and
/// use its existing shell/recovery dispatcher for this intention. No route is
/// committed here; queued activations cannot bypass modals or an exit barrier.
#[cfg(windows)]
pub(super) fn drain(
    window: &AppWindow,
    state: &RefCell<BindingState>,
) -> Option<ActivationResolution> {
    let (current, interaction) = {
        let owner = state.borrow();
        let current = match owner.notifications.platform.lifetime {
            Lifetime::Running => owner
                .notifications
                .platform
                .notice
                .map_or(Observation::Unknown, Observation::Current),
            Lifetime::Dormant | Lifetime::Unavailable | Lifetime::Partial | Lifetime::Stopped => {
                Observation::Unknown
            }
        };
        (current, super::native_workflow::interaction(&owner))
    };
    let mut port = take_port(state)?;
    let events = port.poll();
    let resolved = events
        .activation
        .map(|token| port.resolve_activation(token, current, interaction));
    restore_port(state, port);
    // Failed/wake-fault state changes affect only native availability. They do
    // not promote a fixture to a production incident or submit another toast.
    if events.failed_after_attempt || events.observer_fault {
        if matches!(
            state.borrow().notifications.platform.attempt,
            Attempt::Accepted | Attempt::CoalescedAccepted
        ) {
            state.borrow_mut().notifications.platform.attempt = Attempt::Indeterminate;
            attempt_feedback(window, state);
        }
        refresh(window, state);
    }
    resolved
}

/// Stop requests first, then attempt exact owned native cleanup once.
///
/// Partial is not asynchronous success. The shared exit owner keeps/restores
/// the ordinary window and reports exit-not-completed, rather than quitting or
/// spinning timer retries. A renewed explicit user Quit may call `retry_stop`.
#[cfg(windows)]
pub(super) fn begin_stop(window: &AppWindow, state: &RefCell<BindingState>) -> CloseOutcome {
    if state.borrow().notifications.platform.lifetime == Lifetime::Partial {
        return CloseOutcome::Partial;
    }
    close_once(window, state)
}

/// Retry retained cleanup once only for a renewed explicit user exit attempt.
#[cfg(windows)]
pub(super) fn retry_stop(window: &AppWindow, state: &RefCell<BindingState>) -> CloseOutcome {
    close_once(window, state)
}

/// Native teardown has no timer loop and retains every uncertain obligation.
#[cfg(windows)]
fn close_once(window: &AppWindow, state: &RefCell<BindingState>) -> CloseOutcome {
    // Deny requests before extracting the port; no callback can undo this.
    state.borrow_mut().notifications.platform.lifetime = Lifetime::Stopped;
    let result = if let Some(mut port) = take_port(state) {
        let outcome = port.close();
        if outcome == CloseOutcome::Partial {
            restore_port(state, port);
        }
        outcome
    } else {
        CloseOutcome::Complete
    };
    if result == CloseOutcome::Partial {
        state.borrow_mut().notifications.platform.lifetime = Lifetime::Partial;
    }
    refresh(window, state);
    result
}

/// Only an observed complete close/no installed owner can release normal exit.
#[cfg(windows)]
pub(super) fn stopped(state: &RefCell<BindingState>) -> bool {
    let owner = state.borrow();
    exit_ready(
        owner.notifications.platform.lifetime,
        owner.notifications.platform.port.is_some(),
    )
}

/// A missing temporary port is insufficient while a running/partial owner exists.
#[cfg(any(windows, test))]
const fn exit_ready(lifetime: Lifetime, port_present: bool) -> bool {
    !port_present
        && matches!(
            lifetime,
            Lifetime::Dormant | Lifetime::Unavailable | Lifetime::Stopped
        )
}

#[cfg(test)]
mod tests {
    //! Portable admission controls execute no native code or environment effect.

    use super::{Attempt, Lifetime, NoticeContent, ReceiverFacts, choose_prepared, exit_ready};
    use crate::notifications_ui::native_delivery::{NoticeError, PreparedTestNotice};
    use crate::notifications_ui::{FixtureNotice, FixtureNoticeContext};
    use slint::Model;

    /// Direct receiver calls use private navigation/modal state, never forged UI.
    ///
    /// # Errors
    /// Propagates software platform/host setup failure rather than counting it as
    /// successful receiver rejection. This control performs no native operation.
    ///
    /// # Panics
    /// Fails on a private-context bypass, wrong feedback or shown fixture leak.
    #[test]
    fn direct_receiver_ignores_forged_context_and_respects_private_modal()
    -> Result<(), Box<dyn std::error::Error>> {
        use std::rc::Rc;

        use slint::ComponentHandle;
        use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
        use slint::platform::{Platform, PlatformError, WindowAdapter};

        /// Public software adapter for one manually displayed warning fixture.
        struct Headless;
        impl Platform for Headless {
            /// Create no native window, observer or worker.
            fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
                Ok(MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer))
            }
        }
        slint::platform::set_platform(Box::new(Headless))?;
        let host = crate::slint_host::create()?;
        let window = host.window();
        window.set_current_page(crate::slint_host::Page::Settings);
        window.set_settings_section(2);
        super::request_test(&window.as_weak(), &host.state);
        assert!(window.get_notification_feedback().is_empty());
        window.invoke_navigate(crate::slint_host::Page::Settings);
        super::request_test(&window.as_weak(), &host.state);
        assert!(window.get_notification_feedback().is_empty());
        window.invoke_settings_section_requested(2);
        super::request_test(&window.as_weak(), &host.state);
        let accepted_neighbor = window.get_notification_feedback();
        assert_eq!(
            accepted_neighbor,
            "A running NevroTick window or tray observer is not confirmed. No Windows notification was submitted."
        );
        window.invoke_quit_requested();
        assert!(window.get_quit_warning_open());
        assert!(window.window().is_visible());
        window.set_quit_warning_open(false);
        super::request_test(&window.as_weak(), &host.state);
        assert_eq!(window.get_notification_feedback(), accepted_neighbor);
        window.invoke_quit_cancel_requested();
        assert!(!window.get_quit_warning_open());
        assert!(!window.get_ui_exit_requested());
        window.invoke_navigate(crate::slint_host::Page::Status);
        assert_eq!(window.get_current_page(), crate::slint_host::Page::Status);
        assert!(window.window().is_visible());
        // Only this fixture's genuine Quit warning displayed the window. Cancel
        // keeps it shown; this one manual teardown does not claim app cleanup.
        window.hide()?;
        assert!(!window.window().is_visible());
        let weak = window.as_weak();
        drop(host);
        assert!(weak.upgrade().is_none());
        retained_inline_drafts_do_not_block_settings()?;
        Ok(())
    }

    /// Fixed legal inline contexts retained by ordinary navigation.
    #[derive(Clone, Copy)]
    enum RetainedInline {
        /// An unsubmitted source editor with bounded raw fields.
        SourceEdit,
        /// A local source change review with an accepted preview delta.
        SourceReview,
        /// A local helper preview in its accepted review phase.
        RecoveryReview,
    }

    /// Reach each actual inline owner through healthy callbacks, not view setters.
    ///
    /// # Panics
    /// Fails if a healthy setup is rejected before its ownership test.
    fn open_inline(window: &crate::slint_host::AppWindow, case: RetainedInline) {
        match case {
            RetainedInline::SourceEdit => {
                window.invoke_mode_requested(true);
                window.invoke_navigate(crate::slint_host::Page::Sources);
                window.invoke_edit_requested();
                assert!(window.get_editor_open());
                assert_eq!(
                    window.invoke_raw_text_edited(1, "retained.example.invalid".into()),
                    "retained.example.invalid"
                );
            }
            RetainedInline::SourceReview => {
                window.invoke_mode_requested(true);
                window.invoke_navigate(crate::slint_host::Page::Sources);
                window.invoke_remove_requested(1);
                window.invoke_review_requested();
                assert!(window.get_review_open());
                assert_eq!(window.get_source_changes().row_count(), 1);
            }
            RetainedInline::RecoveryReview => {
                window.invoke_recovery_open_requested();
                assert_eq!(window.invoke_raw_choice_edited(1, 1), 1);
                assert_eq!(
                    window.invoke_raw_text_edited(5, "helper.example.invalid".into()),
                    "helper.example.invalid"
                );
                window.invoke_recovery_update_requested(
                    1,
                    window.get_draft_recovery_first(),
                    window.get_draft_recovery_second(),
                );
                assert!(window.get_recovery_review_open());
            }
        }
    }

    /// Current inline ownership rejects; an off-page retained draft is inert.
    ///
    /// # Errors
    /// Propagates a software host failure without treating it as guard success.
    ///
    /// # Panics
    /// Fails on hidden draft loss, off-page modal ownership or accepted wrong page.
    fn retained_inline_drafts_do_not_block_settings() -> Result<(), Box<dyn std::error::Error>> {
        use slint::ComponentHandle;

        for case in [
            RetainedInline::SourceEdit,
            RetainedInline::SourceReview,
            RetainedInline::RecoveryReview,
        ] {
            let host = crate::slint_host::create()?;
            let window = host.window();
            open_inline(window, case);
            assert!(host.state.borrow().current_inline_owner().is_some());
            assert!(!ReceiverFacts::from_owner(&host.state.borrow()).modal_free);
            let (editor, review, recovery, raw) = {
                let owner = host.state.borrow();
                (
                    owner.editor,
                    owner.review_open,
                    owner.recovery_phase,
                    owner.raw.clone(),
                )
            };
            // A well-formed view forgery does not change the actual inline owner.
            window.set_current_page(crate::slint_host::Page::Settings);
            window.set_settings_section(2);
            super::request_test(&window.as_weak(), &host.state);
            assert!(window.get_notification_feedback().is_empty());
            window.invoke_navigate(crate::slint_host::Page::Settings);
            window.invoke_settings_section_requested(2);
            window.invoke_mode_requested(false);
            assert!(host.state.borrow().current_inline_owner().is_none());
            assert!(ReceiverFacts::from_owner(&host.state.borrow()).admitted());
            super::request_test(&window.as_weak(), &host.state);
            assert_eq!(
                window.get_notification_feedback(),
                "A running NevroTick window or tray observer is not confirmed. No Windows notification was submitted."
            );
            {
                let owner = host.state.borrow();
                assert_eq!(owner.editor, editor);
                assert_eq!(owner.review_open, review);
                assert!(owner.recovery_phase == recovery);
                assert_eq!(owner.raw, raw);
            }
            assert!(!window.window().is_visible());
            let weak = window.as_weak();
            drop(host);
            assert!(weak.upgrade().is_none());
        }
        Ok(())
    }

    /// Every individual private guard rejects while the valid neighbor succeeds.
    ///
    /// # Panics
    /// Fails if a missing independent fact admits the request.
    #[test]
    fn receiver_requires_every_private_axis() {
        let valid = ReceiverFacts {
            initialized: true,
            idle: true,
            healthy: true,
            modal_free: true,
            settings: true,
            notifications: true,
        };
        assert!(valid.admitted());
        for rejected in [
            ReceiverFacts {
                initialized: false,
                ..valid
            },
            ReceiverFacts {
                idle: false,
                ..valid
            },
            ReceiverFacts {
                healthy: false,
                ..valid
            },
            ReceiverFacts {
                modal_free: false,
                ..valid
            },
            ReceiverFacts {
                settings: false,
                ..valid
            },
            ReceiverFacts {
                notifications: false,
                ..valid
            },
        ] {
            assert!(!rejected.admitted());
        }
    }

    /// Catalog absence and failure remain distinct for explicit fixture choice.
    ///
    /// # Errors
    /// Propagates an impossible failure to construct this fixed valid notice.
    ///
    /// # Panics
    /// Fails if rejected active text selects engineering fallback.
    #[test]
    fn rejected_catalog_never_selects_fixed_english() -> Result<(), Box<dyn std::error::Error>> {
        let notice = FixtureNotice::new(1, FixtureNoticeContext::TestExample)
            .ok_or("fixed test identity rejected")?;
        let expected = NoticeError::Catalog(crate::localization::Error::Schema);
        assert_eq!(
            choose_prepared(Err(expected), notice, NoticeContent::EngineeringExample).err(),
            Some(expected),
        );
        assert_eq!(
            choose_prepared(Ok(None), notice, NoticeContent::ApprovedCatalogOnly).err(),
            Some(expected),
        );
        let fixture = choose_prepared(Ok(None), notice, NoticeContent::EngineeringExample)?;
        assert_eq!(fixture.notice(), notice);
        assert_eq!(fixture.title(), "NevroTick test notification");
        assert_eq!(
            fixture.body(),
            "This is an example. No clock or service status was checked."
        );
        Ok(())
    }

    /// An existing preparation keeps its exact identity even in engineering mode.
    ///
    /// # Errors
    /// Propagates fixed notice/preparation failure rather than accepting a dummy.
    ///
    /// # Panics
    /// Fails if an already prepared identity is silently replaced.
    #[test]
    fn selected_preparation_is_not_replaced_or_reidentified()
    -> Result<(), Box<dyn std::error::Error>> {
        let retained = FixtureNotice::new(8, FixtureNoticeContext::TestExample)
            .ok_or("fixed retained identity rejected")?;
        let ignored = FixtureNotice::new(9, FixtureNoticeContext::TestExample)
            .ok_or("fixed neighboring identity rejected")?;
        let prepared = PreparedTestNotice::engineering_example(retained)?;
        let selected = choose_prepared(
            Ok(Some(prepared)),
            ignored,
            NoticeContent::EngineeringExample,
        )?;
        assert_eq!(selected.notice(), retained);
        Ok(())
    }

    /// Repeated input retains each original uncertainty class without new state.
    ///
    /// # Panics
    /// Fails if coalescing erases the earlier request certainty.
    #[test]
    fn coalescing_preserves_attempt_certainty_and_fixed_copy() {
        assert_eq!(Attempt::None.coalesced(), Attempt::None);
        assert_eq!(Attempt::Accepted.coalesced(), Attempt::CoalescedAccepted);
        assert_eq!(
            Attempt::Indeterminate.coalesced(),
            Attempt::CoalescedIndeterminate
        );
        assert_eq!(
            Attempt::CoalescedAccepted.coalesced(),
            Attempt::CoalescedAccepted
        );
        assert_eq!(
            Attempt::CoalescedIndeterminate.coalesced(),
            Attempt::CoalescedIndeterminate
        );
        assert!(Attempt::None.text().is_none());
        assert_eq!(
            Attempt::Accepted.text().map(|value| (value.0, value.1)),
            Some((true, false))
        );
        assert_eq!(
            Attempt::Indeterminate
                .text()
                .map(|value| (value.0, value.1)),
            Some((false, false))
        );
        assert_eq!(
            Attempt::CoalescedAccepted
                .text()
                .map(|value| (value.0, value.1)),
            Some((true, true))
        );
        assert_eq!(
            Attempt::CoalescedIndeterminate
                .text()
                .map(|value| (value.0, value.1)),
            Some((false, true))
        );
    }

    /// A partial/running native owner cannot be treated as a successful exit.
    ///
    /// # Panics
    /// Fails if unresolved ownership is treated as complete cleanup.
    #[test]
    fn normal_exit_requires_explicit_complete_cleanup() {
        for lifetime in [
            Lifetime::Dormant,
            Lifetime::Unavailable,
            Lifetime::Running,
            Lifetime::Partial,
            Lifetime::Stopped,
        ] {
            assert!(!exit_ready(lifetime, true));
        }
        assert!(!exit_ready(Lifetime::Running, false));
        assert!(!exit_ready(Lifetime::Partial, false));
        assert!(exit_ready(Lifetime::Dormant, false));
        assert!(exit_ready(Lifetime::Unavailable, false));
        assert!(exit_ready(Lifetime::Stopped, false));
    }
}

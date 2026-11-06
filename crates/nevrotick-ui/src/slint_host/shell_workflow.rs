//! Guarded UI-only quit confirmation and shared Settings/service destinations.
//!
//! One root owner retains accepted preferences and source drafts. This child
//! admits native observer intentions through the same controller. Service,
//! network, elevation and clock effects remain unavailable.
#![forbid(unsafe_code)]

use std::{cell::RefCell, rc::Rc};

use slint::{CloseRequestResponse, ComponentHandle};

use super::catalog_workflow::{FeedbackSlot, write, write_many};
use super::{AppWindow, BindingState, ProjectionGuard, project_navigation};
use crate::localization::{MessageId, UiText};
#[cfg(windows)]
use crate::notifications_ui::{ObserverPresence, WindowsNotificationSetting};
use crate::presentation::{Mode, Page, UiError};
#[cfg(windows)]
use crate::shell::Observation;
use crate::shell::{
    AdapterAvailability, FixtureServicePort, InteractionGuard, KeepOpenReason, QuitNotice,
    ServiceCommand, ServiceManagementPort, ServiceRequestOutcome, SettingsSection, ShellController,
    ShellOutcome, ShellRequest, ShellRoute,
};
#[cfg(any(windows, test))]
use crate::tray_ui::TrayInteraction;

/// Private receiving state; no view flag supplies dialog ownership or readiness.
pub(super) struct ShellBindingState {
    /// Sole owner of this window's quit confirmation and requested exit.
    controller: ShellController,
    /// No transport or service manager exists behind this fixture port.
    services: FixtureServicePort,
    /// Shared named Settings destination, independent of presentation mode.
    section: SettingsSection,
    /// Current Windows notification facility is not connected or observed.
    #[cfg(windows)]
    notification_windows: Observation<WindowsNotificationSetting>,
    /// Native observer availability is not inferred from a saved preference.
    #[cfg(windows)]
    notification_observer: Observation<ObserverPresence>,
}

impl ShellBindingState {
    /// Create local controller state with honest disconnected observations.
    pub(super) fn new() -> Self {
        Self {
            controller: ShellController::default(),
            services: FixtureServicePort,
            section: SettingsSection::Overview,
            #[cfg(windows)]
            notification_windows: Observation::Unknown,
            #[cfg(windows)]
            notification_observer: Observation::Unknown,
        }
    }

    /// Current sole shell/modal ownership; no generated property supplies it.
    #[cfg(any(windows, test))]
    pub(super) const fn interaction(&self) -> TrayInteraction {
        if self.controller.exit_requested() {
            TrayInteraction::ExitRequested
        } else if self.controller.quit_confirmation_open() {
            TrayInteraction::DialogOwned
        } else {
            TrayInteraction::Free
        }
    }

    /// Return the trusted named Settings destination for peer receiving guards.
    pub(super) const fn settings_section(&self) -> SettingsSection {
        self.section
    }

    /// Reconcile advanced Settings context when an accepted mode becomes Simple.
    /// Local profile/raw drafts are separate owners and remain untouched.
    pub(super) fn reconcile_mode(&mut self, mode: Mode) {
        if mode == Mode::Simple && self.section == SettingsSection::TimePolicy {
            self.section = SettingsSection::Overview;
        }
    }

    /// Block other receiving workflows while this modal or UI exit owns the root.
    pub(super) const fn blocks_other_requests(&self) -> bool {
        self.controller.quit_confirmation_open() || self.controller.exit_requested()
    }
}

/// Owned projection releases every state borrow before invoking toolkit setters.
struct ShellProjection {
    /// Rust-owned confirmation visibility; a forged view flag grants nothing.
    quit_open: bool,
    /// Fixed emergency wording is selected only by the private shell controller.
    emergency_open: bool,
    /// Closed service-continuity meaning, without raw service/host details.
    notice: QuitNotice,
    /// Checked shared section represented by a closed small signed choice.
    section: i32,
    /// Requested exit only, never a claim that the process terminated.
    exit_requested: bool,
}

impl ShellProjection {
    /// Borrow fixed unavailable observations without changing any axis.
    fn from_owner(owner: &BindingState) -> Self {
        let shell = &owner.shell;
        let notice = ShellController::quit_notice(shell.services.observations().runtime());
        Self {
            quit_open: shell.controller.quit_confirmation_open(),
            emergency_open: shell.controller.emergency_armed(),
            notice,
            section: section_choice(shell.section),
            exit_requested: shell.controller.exit_requested(),
        }
    }

    /// Project the owner; focused controls are requested only by explicit routes.
    fn apply(self, window: &AppWindow, state: &RefCell<BindingState>) {
        window.set_emergency_quit_warning(self.emergency_open);
        window.set_quit_warning_open(self.quit_open);
        write_many(
            window,
            state,
            &[
                (
                    FeedbackSlot::QuitHeadline,
                    Some(UiText::quit_headline(self.notice.service_state())),
                    self.notice.headline().into(),
                ),
                (
                    FeedbackSlot::QuitConsequence,
                    Some(UiText::quit_consequence(self.notice.service_state())),
                    self.notice.consequence().into(),
                ),
            ],
        );
        window.set_settings_section(self.section);
        window.set_ui_exit_requested(self.exit_requested);
    }
}

/// Initialize presentation before publishing host readiness or accepting requests.
pub(super) fn initialize(window: &AppWindow, state: &RefCell<BindingState>) {
    let projection = ShellProjection::from_owner(&state.borrow());
    projection.apply(window, state);
    write_many(
        window,
        state,
        &[
            (FeedbackSlot::ShellFeedback, None, "".into()),
            (FeedbackSlot::ServiceFeedback, None, "".into()),
            (FeedbackSlot::NotificationFeedback, None, "".into()),
        ],
    );
}

/// Install weak, narrow callbacks and the public native-close decision bridge.
pub(super) fn install_callbacks(window: &AppWindow, state: &Rc<RefCell<BindingState>>) {
    install_quit_callbacks(window, state);
    install_settings_callbacks(window, state);
    let owner = Rc::clone(state);
    window.on_repository_permitted(move || {
        owner.try_borrow().is_ok_and(|owner| {
            owner.initialized
                && !owner.projecting
                && !owner.copy.failed()
                && !owner.raw_failed()
                && owner.navigation.page() == Page::About
                && !owner.shell.blocks_other_requests()
                && owner.recovery_leave.is_none()
        })
    });
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_window_close_requested(move || close_allowed(&weak, &owner));
    let weak = window.as_weak();
    window.window().on_close_requested(move || {
        if let Some(window) = weak.upgrade() {
            // Our receiving handler owns the single fallible hide attempt and
            // can recover its pending state. Do not hide again in the toolkit.
            let _close_completed = window.invoke_window_close_requested();
            CloseRequestResponse::KeepWindowShown
        } else {
            CloseRequestResponse::HideWindow
        }
    });
}

/// Keep explicit Quit separate from caption Close; only this owner resolves it.
fn install_quit_callbacks(window: &AppWindow, state: &Rc<RefCell<BindingState>>) {
    for (kind, bind) in [
        (ShellRequest::QuitUi, 0),
        (ShellRequest::CancelQuit, 1),
        (ShellRequest::ConfirmQuit, 2),
    ] {
        let weak = window.as_weak();
        let owner = Rc::clone(state);
        match bind {
            0 => window.on_quit_requested(move || {
                shell_request(&weak, &owner, kind);
            }),
            1 => window.on_quit_cancel_requested(move || {
                shell_request(&weak, &owner, kind);
            }),
            _ => window.on_quit_confirm_requested(move || {
                shell_request(&weak, &owner, kind);
            }),
        }
    }
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_manage_service_requested(move || {
        shell_request(&weak, &owner, ShellRequest::ManageService);
    });
}

/// Bind closed section choices and explicit navigation versus unavailable effects.
fn install_settings_callbacks(window: &AppWindow, state: &Rc<RefCell<BindingState>>) {
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_settings_section_requested(move |choice| {
        select_section(&weak, &owner, choice, false);
    });
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_service_power_requested(move || {
        select_section(&weak, &owner, 1, true);
    });
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_diagnose_service_requested(move || {
        diagnose(&weak, &owner);
    });
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_service_command_requested(move |choice| {
        unavailable_service(&weak, &owner, choice);
    });
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_notification_test_requested(move || {
        super::native_notification_workflow::request_test(&weak, &owner);
    });
}

/// Use the same private shell admission for actual and directly invoked Close.
fn close_allowed(weak: &slint::Weak<AppWindow>, state: &RefCell<BindingState>) -> bool {
    let Some(window) = weak.upgrade() else {
        return false;
    };
    let Some(_projection) = ProjectionGuard::enter_shell(state) else {
        return false;
    };
    let failed_presentation = {
        let owner = state.borrow();
        owner.copy.failed() || owner.raw_failed()
    };
    if failed_presentation {
        // A terminal UI invariant cannot strand a raw Recovery leave modal.
        // This helper drops only its pending destination, exactly like Stay.
        super::recovery_workflow::stay_for_emergency(&window, state);
    }
    let tray = tray_availability(state);
    let (outcome, projection) = {
        let mut owner = state.borrow_mut();
        let outcome = if failed_presentation {
            owner
                .shell
                .controller
                .request_emergency_close(InteractionGuard::Free)
        } else {
            let preferences = owner.preferences.clone();
            let runtime = owner.shell.services.observations().runtime();
            owner.shell.controller.request(
                ShellRequest::CloseWindow,
                &preferences,
                tray,
                runtime,
                InteractionGuard::Free,
            )
        };
        (outcome, ShellProjection::from_owner(&owner))
    };
    projection.apply(&window, state);
    match outcome {
        ShellOutcome::ExitUiRequested => complete_exit_request(&window, state),
        ShellOutcome::HideToTrayRequested => hide_to_tray(&window, state),
        ShellOutcome::ShowQuitConfirmation(_) => {
            show_quit_confirmation(&window, state);
            false
        }
        _ => {
            show_keep_reason(&window, state, outcome);
            false
        }
    }
}

/// Dispatch explicit UI intents, without retaining borrows across callbacks.
fn shell_request(
    weak: &slint::Weak<AppWindow>,
    state: &RefCell<BindingState>,
    request: ShellRequest,
) {
    let Some(window) = weak.upgrade() else {
        return;
    };
    let Some(projection_guard) = ProjectionGuard::enter_shell(state) else {
        return;
    };
    let tray = tray_availability(state);
    let (outcome, projection) = {
        let mut owner = state.borrow_mut();
        if (owner.copy.failed() || owner.raw_failed())
            && !matches!(request, ShellRequest::CancelQuit)
            && !(request == ShellRequest::ConfirmQuit
                && owner.shell.controller.emergency_armed()
                && owner.shell.controller.quit_confirmation_open())
        {
            return;
        }
        let preferences = owner.preferences.clone();
        let runtime = owner.shell.services.observations().runtime();
        // Current source editor/review are inline, not separate modal owners.
        let outcome = owner.shell.controller.request(
            request,
            &preferences,
            tray,
            runtime,
            InteractionGuard::Free,
        );
        (outcome, ShellProjection::from_owner(&owner))
    };
    projection.apply(&window, state);
    match outcome {
        ShellOutcome::ShowQuitConfirmation(_) => show_quit_confirmation(&window, state),
        ShellOutcome::Navigate(route) => {
            write(&window, state, FeedbackSlot::ShellFeedback, None, "".into());
            // Quit is already closed. Release our projection owner before the
            // recovery navigator checks dirty drafts or opens its own modal.
            drop(projection_guard);
            super::recovery_workflow::request_shell_route(&window.as_weak(), state, route, false);
        }
        ShellOutcome::ExitUiRequested => {
            let _exit_completed = complete_exit_request(&window, state);
        }
        ShellOutcome::HideToTrayRequested => {
            let _hidden = hide_to_tray(&window, state);
        }
        _ => show_keep_reason(&window, state, outcome),
    }
}

/// Show the one owned normal/emergency warning and recover a failed show once.
fn show_quit_confirmation(window: &AppWindow, state: &RefCell<BindingState>) {
    let tray = tray_availability(state);
    // A shown root already has lifecycle custody. Avoid a redundant
    // backend show that could fail. For a hidden root, failed show
    // still acquires Slint keepalive and needs one bounded hide cleanup.
    if !window.window().is_visible() && window.show().is_err() {
        let cleanup_failed = window.hide().is_err();
        let projection = {
            let mut owner = state.borrow_mut();
            let preferences = owner.preferences.clone();
            let runtime = owner.shell.services.observations().runtime();
            let _cancelled = owner.shell.controller.request(
                ShellRequest::CancelQuit,
                &preferences,
                tray,
                runtime,
                InteractionGuard::Free,
            );
            ShellProjection::from_owner(&owner)
        };
        projection.apply(window, state);
        let (message, feedback) = if cleanup_failed {
            (
                MessageId::RUST_QUIT_SHOW_CLEANUP_FAILED,
                "The quit warning could not be shown and window cleanup failed. Window state is unconfirmed; no UI exit was confirmed.",
            )
        } else {
            (
                MessageId::RUST_QUIT_SHOW_FAILED,
                "The quit warning could not be shown. No UI exit was requested; try again.",
            )
        };
        write(
            window,
            state,
            FeedbackSlot::ShellFeedback,
            Some(UiText::fixed(message)),
            feedback.into(),
        );
    } else {
        write(window, state, FeedbackSlot::ShellFeedback, None, "".into());
        window.invoke_focus_quit_warning();
    }
}

/// Own one fallible public hide request rather than inferring process termination.
fn complete_exit_request(window: &AppWindow, state: &RefCell<BindingState>) -> bool {
    #[cfg(windows)]
    if super::native_workflow::begin_exit(window, state) {
        return false;
    }
    finish_exit_request(window, state)
}

/// Final public toolkit exit, reached only after native cleanup acknowledgement.
fn finish_exit_request(window: &AppWindow, state: &RefCell<BindingState>) -> bool {
    let hidden = window.hide().is_ok();
    if hidden && slint::quit_event_loop().is_ok() {
        return true;
    }
    // Slint 1.18.1 releases its strong component/keepalive even when hide
    // fails. One public show attempt reacquires lifecycle custody before we
    // expose a retry; its failure remains an unconfirmed window outcome.
    let restored = window.show().is_ok();
    #[cfg(windows)]
    super::native_workflow::exit_not_completed(state);
    let projection = {
        let mut owner = state.borrow_mut();
        let _outcome = owner.shell.controller.exit_not_completed();
        ShellProjection::from_owner(&owner)
    };
    projection.apply(window, state);
    let (message, feedback) = match (hidden, restored) {
        (false, true) => (
            MessageId::RUST_CLOSE_RESTORED,
            "App close failed. The window was restored; preview drafts are retained. Service state was not changed.",
        ),
        (false, false) => (
            MessageId::RUST_CLOSE_RESTORE_FAILED,
            "App close failed and the window could not be restored. Window state is unconfirmed; no service change was requested.",
        ),
        (true, true) => (
            MessageId::RUST_EXIT_RESTORED,
            "The app exit request failed. The window was restored; preview drafts are retained. No service change was requested.",
        ),
        (true, false) => (
            MessageId::RUST_EXIT_RESTORE_FAILED,
            "The app exit request failed and the window could not be restored. Window state is unconfirmed; no service change was requested.",
        ),
    };
    write(
        window,
        state,
        FeedbackSlot::ShellFeedback,
        Some(UiText::fixed(message)),
        feedback.into(),
    );
    false
}

/// Commit a typed, already guarded route only after the recovery leave decision.
///
/// The caller holds the root projection guard. Candidate navigation is checked
/// before either accepted navigation or Settings destination changes.
///
/// # Errors
/// Returns the checked navigation rejection without a partial destination change.
pub(super) fn commit_route(
    owner: &mut BindingState,
    route: ShellRoute,
    power: bool,
) -> Result<(), UiError> {
    let mut navigation = owner.navigation.clone();
    if power
        || matches!(
            route,
            ShellRoute::Diagnostics
                | ShellRoute::Sources
                | ShellRoute::CorrectionReview
                | ShellRoute::Settings(SettingsSection::TimePolicy)
        )
    {
        navigation.set_mode(Mode::Power);
    }
    let page = match route {
        ShellRoute::Status => Page::Status,
        ShellRoute::About => Page::About,
        ShellRoute::Diagnostics | ShellRoute::CorrectionReview => Page::Diagnostics,
        ShellRoute::Sources => Page::Sources,
        ShellRoute::Settings(_) => Page::Settings,
    };
    navigation.navigate(page)?;
    owner.navigation = navigation;
    if route == ShellRoute::CorrectionReview {
        owner.diagnostics.open_correction_review();
    }
    if let ShellRoute::Settings(section) = route {
        owner.shell.section = section;
    }
    owner.shell.reconcile_mode(owner.navigation.mode());
    Ok(())
}

/// Project only the trusted section after generic navigation/mode reconciliation.
/// This does not clear feedback, move focus or override a restored Back scroll.
pub(super) fn project_section(window: &AppWindow, state: &RefCell<BindingState>) {
    let section = section_choice(state.borrow().shell.section);
    window.set_settings_section(section);
}

/// Project an accepted shell route after its owner borrow ends, without effects.
///
/// The recovery navigator invokes service focus only after this projection for
/// an accepted `BackgroundService` destination. Stay never calls this function.
pub(super) fn project_route(window: &AppWindow, state: &RefCell<BindingState>) {
    let (mode, page, projection) = {
        let owner = state.borrow();
        (
            owner.navigation.mode(),
            owner.navigation.page(),
            ShellProjection::from_owner(&owner),
        )
    };
    project_navigation(window, mode, page);
    projection.apply(window, state);
    super::diagnostic_workflow::project(window, state);
    window.set_page_scroll_y(0.0);
}

/// Render a blocked close/forged confirmation without treating it as exit success.
fn show_keep_reason(window: &AppWindow, state: &RefCell<BindingState>, outcome: ShellOutcome) {
    let feedback = match outcome {
        ShellOutcome::KeepOpen(KeepOpenReason::TrayUnavailable)
        | ShellOutcome::HideToTrayRequested => Some((
            MessageId::RUST_TRAY_UNAVAILABLE,
            "A usable tray icon is not connected. Keep this window open, or choose Quit UI to close only the app.",
        )),
        ShellOutcome::KeepOpen(KeepOpenReason::NoQuitConfirmation) => Some((
            MessageId::RUST_QUIT_REVIEW_REQUIRED,
            "Open Quit UI and review its warning before confirming exit.",
        )),
        ShellOutcome::KeepOpen(KeepOpenReason::QuitCancelled) => Some((
            MessageId::RUST_QUIT_CANCELLED,
            "Quit cancelled. The app remains open; preview drafts are retained.",
        )),
        _ => None,
    };
    if let Some((message, text)) = feedback {
        write(
            window,
            state,
            FeedbackSlot::ShellFeedback,
            Some(UiText::fixed(message)),
            text.into(),
        );
    }
    if outcome == ShellOutcome::KeepOpen(KeepOpenReason::QuitCancelled) {
        window.invoke_focus_after_quit();
    }
}

/// Accept only closed section choices; service detail Power transition is explicit.
fn select_section(
    weak: &slint::Weak<AppWindow>,
    state: &RefCell<BindingState>,
    choice: i32,
    power: bool,
) {
    let Some(window) = weak.upgrade() else {
        return;
    };
    let Some(projection_guard) = ProjectionGuard::enter(state) else {
        return;
    };
    let section = match choice {
        0 => SettingsSection::Overview,
        1 => SettingsSection::BackgroundService,
        2 => SettingsSection::Notifications,
        3 => SettingsSection::TimePolicy,
        4 => SettingsSection::PersonalPreferences,
        _ => {
            write(
                &window,
                state,
                FeedbackSlot::ShellFeedback,
                Some(UiText::fixed(MessageId::RUST_SETTINGS_SECTION)),
                "Choose a listed Settings section.".into(),
            );
            return;
        }
    };
    drop(projection_guard);
    super::recovery_workflow::request_shell_route(
        weak,
        state,
        ShellRoute::Settings(section),
        power,
    );
}

/// Diagnose routes through the same dirty-draft guard; commit explicitly selects Power.
fn diagnose(weak: &slint::Weak<AppWindow>, state: &RefCell<BindingState>) {
    super::recovery_workflow::request_shell_route(weak, state, ShellRoute::Diagnostics, false);
}

/// Direct disabled-control calls still hit the disconnected typed fixture port.
fn unavailable_service(weak: &slint::Weak<AppWindow>, state: &RefCell<BindingState>, choice: i32) {
    let Some(window) = weak.upgrade() else {
        return;
    };
    let Some(_projection_guard) = ProjectionGuard::enter(state) else {
        return;
    };
    let command = match choice {
        0 => ServiceCommand::Start,
        1 => ServiceCommand::Stop,
        2 => ServiceCommand::Restart,
        3 => ServiceCommand::SetAutomaticStart(true),
        4 => ServiceCommand::SetAutomaticStart(false),
        5 => ServiceCommand::DisableNevroTick,
        6 => ServiceCommand::UseWindowsTime,
        7 => ServiceCommand::UseNevroTick,
        _ => {
            write(
                &window,
                state,
                FeedbackSlot::ServiceFeedback,
                Some(UiText::fixed(MessageId::RUST_SERVICE_ACTION)),
                "Choose a listed service action. No request was submitted.".into(),
            );
            return;
        }
    };
    let attempt = state.borrow().shell.services.request(command);
    match attempt.outcome() {
        ServiceRequestOutcome::AdapterUnavailable => write(&window, state, FeedbackSlot::ServiceFeedback, Some(UiText::fixed(MessageId::RUST_SERVICE_UNAVAILABLE)), "Service management is unavailable in this preview. No request was submitted; runtime, startup and clock ownership were not changed.".into()),
    }
}

/// Closed view representation; no enum cast or unknown fallback is used.
const fn section_choice(section: SettingsSection) -> i32 {
    match section {
        SettingsSection::Overview => 0,
        SettingsSection::BackgroundService => 1,
        SettingsSection::Notifications => 2,
        SettingsSection::TimePolicy => 3,
        SettingsSection::PersonalPreferences => 4,
    }
}

/// Fixture availability stays unavailable; only the installed native owner is queried.
fn tray_availability(state: &RefCell<BindingState>) -> AdapterAvailability {
    #[cfg(windows)]
    {
        super::native_workflow::availability(state)
    }
    #[cfg(not(windows))]
    {
        let _ = state;
        AdapterAvailability::Unavailable
    }
}

/// Hide only after another current actual readiness check; keep the explicit UI loop alive.
fn hide_to_tray(window: &AppWindow, state: &RefCell<BindingState>) -> bool {
    if tray_availability(state) != AdapterAvailability::Ready {
        show_keep_reason(
            window,
            state,
            ShellOutcome::KeepOpen(KeepOpenReason::TrayUnavailable),
        );
        return false;
    }
    if window.hide().is_ok() {
        return false;
    }
    let restored = window.show().is_ok();
    let (message, text) = if restored {
        (
            MessageId::RUST_CLOSE_RESTORED,
            "App close failed. The window was restored; preview drafts are retained. Service state was not changed.",
        )
    } else {
        (
            MessageId::RUST_CLOSE_RESTORE_FAILED,
            "App close failed and the window could not be restored. Window state is unconfirmed; no service change was requested.",
        )
    };
    write(
        window,
        state,
        FeedbackSlot::ShellFeedback,
        Some(UiText::fixed(message)),
        text.into(),
    );
    false
}

/// Current native facility observations project after releasing their private owner.
#[cfg(windows)]
pub(super) fn observe_native_notifications(
    window: &AppWindow,
    state: &RefCell<BindingState>,
    availability: AdapterAvailability,
    windows: Observation<WindowsNotificationSetting>,
    observer: Observation<ObserverPresence>,
    test_available: bool,
) {
    let (windows, observer) = {
        let mut owner = state.borrow_mut();
        owner.shell.notification_windows = windows;
        owner.shell.notification_observer = observer;
        (
            owner.shell.notification_windows,
            owner.shell.notification_observer,
        )
    };
    let windows = match windows {
        Observation::Current(WindowsNotificationSetting::Allowed) => 1,
        Observation::Current(WindowsNotificationSetting::BlockedForUser) => 2,
        Observation::Current(WindowsNotificationSetting::BlockedByPolicy) => 3,
        Observation::Current(WindowsNotificationSetting::Unsupported) => 4,
        Observation::Unknown | Observation::Stale(_) => 0,
    };
    let observer = match observer {
        Observation::Current(ObserverPresence::Running) => 1,
        Observation::Current(ObserverPresence::Stopped) => 2,
        Observation::Unknown | Observation::Stale(_) => 0,
    };
    window.set_native_notifications_available(availability == AdapterAvailability::Ready);
    window.set_native_notification_test_available(
        test_available && availability == AdapterAvailability::Ready,
    );
    window.set_windows_notification_setting(windows);
    window.set_notification_observer_state(observer);
}

/// Native intentions still pass through the current typed menu and sole controller.
#[cfg(windows)]
pub(super) fn receive_native_action(
    window: &AppWindow,
    state: &RefCell<BindingState>,
    id: crate::tray_ui::TrayItemId,
) {
    use crate::{
        status_ui::StatusText,
        tray_ui::{TrayItemId, TrayRejection},
    };
    if !super::native_workflow::restore_window(window) {
        return;
    }
    let Some(guard) = ProjectionGuard::enter_native(state) else {
        return;
    };
    let failed = {
        let owner = state.borrow();
        owner.copy.failed() || owner.raw_failed()
    };
    if failed {
        drop(guard);
        if id == TrayItemId::QuitUi {
            let _close_requested = close_allowed(&window.as_weak(), state);
        }
        return;
    }
    let request = {
        let owner = state.borrow();
        super::native_workflow::current_menu(&owner)
            .request(id.view_id(), super::native_workflow::interaction(&owner))
    };
    match request {
        Ok(request) => {
            drop(guard);
            shell_request(&window.as_weak(), state, request);
        }
        Err(TrayRejection::CheckUnavailable(_)) => {
            let text = StatusText::CheckUnavailable;
            if let Ok(message) = UiText::key(text.catalog_id()) {
                write(
                    window,
                    state,
                    FeedbackSlot::ShellFeedback,
                    Some(message),
                    text.english().into(),
                );
            }
        }
        Err(
            TrayRejection::DialogOwned | TrayRejection::ExitRequested | TrayRejection::InvalidItem,
        ) => {
            drop(guard);
            super::native_workflow::focus_owned(window, state);
        }
    }
}

/// Actual stop timeout/failure wording has one protected dynamic feedback sink.
#[cfg(windows)]
pub(super) fn native_stop_feedback(
    window: &AppWindow,
    state: &RefCell<BindingState>,
    failed: bool,
) {
    let (key, english) = if failed {
        (
            "ui.native-observer.stop-failed",
            "Native observer cleanup could not be confirmed. The app stays open; no service state was changed.",
        )
    } else {
        (
            "ui.native-observer.stop-timeout",
            "Native observer cleanup is still pending. The app stays open; no service state was changed.",
        )
    };
    let Ok(message) = UiText::key(key) else {
        super::catalog_workflow::invariant_failed(window, state);
        return;
    };
    write(
        window,
        state,
        FeedbackSlot::ShellFeedback,
        Some(message),
        english.into(),
    );
}

/// Reset only the existing shell's failed exit; caller already owns projection custody.
#[cfg(windows)]
pub(super) fn native_exit_failed(window: &AppWindow, state: &RefCell<BindingState>) {
    let projection = {
        let mut owner = state.borrow_mut();
        let _not_completed = owner.shell.controller.exit_not_completed();
        ShellProjection::from_owner(&owner)
    };
    projection.apply(window, state);
    native_stop_feedback(window, state, true);
}

/// Timer failure routing acquires the same native-only projection guard.
#[cfg(windows)]
pub(super) fn report_native_exit_failure(window: &AppWindow, state: &RefCell<BindingState>) {
    let Some(_guard) = ProjectionGuard::enter_native(state) else {
        return;
    };
    native_exit_failed(window, state);
}

/// A timed-out pending stop releases old consent while retaining native custody.
#[cfg(windows)]
pub(super) fn report_native_exit_pending(window: &AppWindow, state: &RefCell<BindingState>) {
    let Some(_guard) = ProjectionGuard::enter_native(state) else {
        return;
    };
    let projection = {
        let mut owner = state.borrow_mut();
        let _not_completed = owner.shell.controller.exit_not_completed();
        ShellProjection::from_owner(&owner)
    };
    projection.apply(window, state);
    native_stop_feedback(window, state, false);
}

/// A verified terminal native stop may finish the already-consented UI-only exit.
#[cfg(windows)]
pub(super) fn complete_native_exit(window: &AppWindow, state: &RefCell<BindingState>) {
    let Some(_guard) = ProjectionGuard::enter_native(state) else {
        return;
    };
    if state.borrow().shell.controller.exit_requested() {
        let _completed = finish_exit_request(window, state);
    }
}

/// Actual generated failed-copy receivers, with a public software window/proxy.
#[cfg(test)]
#[path = "emergency_tests.rs"]
mod emergency_tests;

//! Contract tests for UI-only tray/quit decisions and unavailable service ports.

use nevrotick_ui::preferences::UiPreferences;
use nevrotick_ui::shell::{
    AdapterAvailability, ClockOwner, FixtureServicePort, InteractionGuard, KeepOpenReason,
    Observation, QuitNotice, QuitServiceState, RuntimeState, ServiceCommand, ServiceManagementPort,
    ServicePairRuntime, ServiceRequestOutcome, SettingsSection, ShellController, ShellOutcome,
    ShellRequest, ShellRoute, SynchronizationState, TrayMenuAction,
};

/// Normal Close requests UI exit only when the accepted tray preference is off.
///
/// # Panics
/// Fails when the stated UI-only contract or a healthy neighboring path regresses.
#[test]
fn ordinary_close_exits_with_tray_off_without_changing_preferences() {
    let preferences = UiPreferences::default();
    let before = preferences.clone();
    let mut shell = ShellController::default();
    assert_eq!(
        shell.request(
            ShellRequest::CloseWindow,
            &preferences,
            AdapterAvailability::Unavailable,
            Observation::Unknown,
            InteractionGuard::Free,
        ),
        ShellOutcome::ExitUiRequested
    );
    assert_eq!(preferences, before);
    assert!(shell.exit_requested());
    assert!(!shell.quit_confirmation_open());
}

/// A saved tray preference never substitutes for current native adapter readiness.
///
/// # Panics
/// Fails when the stated UI-only contract or a healthy neighboring path regresses.
#[test]
fn accepted_tray_preference_is_insufficient_to_hide_without_a_usable_adapter() {
    let mut preferences = UiPreferences::default();
    preferences.set_tray_enabled(true);
    let mut shell = ShellController::default();
    for availability in [
        AdapterAvailability::Unavailable,
        AdapterAvailability::Unknown,
    ] {
        assert_eq!(
            shell.request(
                ShellRequest::CloseWindow,
                &preferences,
                availability,
                Observation::Unknown,
                InteractionGuard::Free,
            ),
            ShellOutcome::KeepOpen(KeepOpenReason::TrayUnavailable)
        );
        assert!(!shell.exit_requested());
    }
    assert_eq!(
        shell.request(
            ShellRequest::CloseWindow,
            &preferences,
            AdapterAvailability::Ready,
            Observation::Unknown,
            InteractionGuard::Free,
        ),
        ShellOutcome::HideToTrayRequested
    );
    assert!(!shell.exit_requested());
}

/// Direct confirmation cannot skip a privately owned warning.
///
/// # Panics
/// Fails when the stated UI-only contract or a healthy neighboring path regresses.
#[test]
fn direct_confirm_cannot_bypass_the_quit_warning() {
    let mut shell = ShellController::default();
    assert_eq!(
        shell.request(
            ShellRequest::ConfirmQuit,
            &UiPreferences::default(),
            AdapterAvailability::Ready,
            Observation::Unknown,
            InteractionGuard::Free,
        ),
        ShellOutcome::KeepOpen(KeepOpenReason::NoQuitConfirmation)
    );
    assert!(!shell.exit_requested());
}

/// Unknown service state stays unknown and cancellation preserves accepted state.
///
/// # Panics
/// Fails when the stated UI-only contract or a healthy neighboring path regresses.
#[test]
fn quit_unknown_warns_and_cancel_keeps_the_observer_and_accepted_preferences() {
    let mut preferences = UiPreferences::default();
    preferences.set_tray_enabled(true);
    assert!(preferences.set_sign_in_enabled(true).is_ok());
    let before = preferences.clone();
    let mut shell = ShellController::default();
    let outcome = shell.request(
        ShellRequest::QuitUi,
        &preferences,
        AdapterAvailability::Ready,
        Observation::Unknown,
        InteractionGuard::Free,
    );
    let notice = ShellController::quit_notice(Observation::Unknown);
    assert_eq!(outcome, ShellOutcome::ShowQuitConfirmation(notice));
    assert_eq!(notice.service_state(), QuitServiceState::Unconfirmed);
    assert_eq!(notice.headline(), "Service state is unconfirmed");
    assert!(QuitNotice::stop_path().ends_with("Background service > Stop"));
    assert!(QuitNotice::disable_path().ends_with("Background service > Disable NevroTick"));
    assert!(QuitNotice::observer_notice().contains("last app or tray observer"));
    assert!(shell.quit_confirmation_open());
    assert_eq!(
        shell.request(
            ShellRequest::CancelQuit,
            &preferences,
            AdapterAvailability::Ready,
            Observation::Unknown,
            InteractionGuard::Free,
        ),
        ShellOutcome::KeepOpen(KeepOpenReason::QuitCancelled)
    );
    assert!(!shell.quit_confirmation_open());
    assert!(!shell.exit_requested());
    assert_eq!(preferences, before);
}

/// Service management from the warning navigates to shared Settings without exit.
///
/// # Panics
/// Fails when the stated UI-only contract or a healthy neighboring path regresses.
#[test]
fn manage_service_dismisses_warning_and_navigates_to_shared_settings_only() {
    let preferences = UiPreferences::default();
    let mut shell = ShellController::default();
    let _warning = shell.request(
        ShellRequest::QuitUi,
        &preferences,
        AdapterAvailability::Unavailable,
        Observation::Unknown,
        InteractionGuard::Free,
    );
    assert_eq!(
        shell.request(
            ShellRequest::ManageService,
            &preferences,
            AdapterAvailability::Unavailable,
            Observation::Unknown,
            InteractionGuard::Free,
        ),
        ShellOutcome::Navigate(ShellRoute::Settings(SettingsSection::BackgroundService))
    );
    assert!(!shell.quit_confirmation_open());
    assert!(!shell.exit_requested());
    assert_eq!(preferences, UiPreferences::default());
}

/// Confirmation requests UI exit once without claiming native completion.
///
/// # Panics
/// Fails when the stated UI-only contract or a healthy neighboring path regresses.
#[test]
fn confirmed_quit_requests_only_ui_exit_and_repeated_requests_do_not_restart_work() {
    let preferences = UiPreferences::default();
    let mut shell = ShellController::default();
    let runtime = Observation::Current(ServicePairRuntime::new(
        RuntimeState::Running,
        RuntimeState::Running,
    ));
    let _warning = shell.request(
        ShellRequest::QuitUi,
        &preferences,
        AdapterAvailability::Unavailable,
        runtime,
        InteractionGuard::Free,
    );
    assert_eq!(
        shell.request(
            ShellRequest::ConfirmQuit,
            &preferences,
            AdapterAvailability::Unavailable,
            runtime,
            InteractionGuard::Free,
        ),
        ShellOutcome::ExitUiRequested
    );
    assert!(!shell.quit_confirmation_open());
    assert!(shell.exit_requested());
    assert_eq!(
        shell.request(
            ShellRequest::QuitUi,
            &preferences,
            AdapterAvailability::Unavailable,
            runtime,
            InteractionGuard::Free,
        ),
        ShellOutcome::KeepOpen(KeepOpenReason::ExitAlreadyRequested)
    );
}

/// A failed host exit leaves the observer available for a fresh explicit request.
///
/// # Panics
/// Fails if a failed effect request is treated as successful termination.
#[test]
fn failed_host_exit_is_not_reported_as_termination_and_can_be_retried_explicitly() {
    let preferences = UiPreferences::default();
    let mut shell = ShellController::default();
    assert_eq!(
        shell.request(
            ShellRequest::CloseWindow,
            &preferences,
            AdapterAvailability::Unavailable,
            Observation::Unknown,
            InteractionGuard::Free,
        ),
        ShellOutcome::ExitUiRequested
    );
    assert_eq!(
        shell.exit_not_completed(),
        ShellOutcome::KeepOpen(KeepOpenReason::ExitNotCompleted)
    );
    assert!(!shell.exit_requested());
    assert_eq!(
        shell.request(
            ShellRequest::QuitUi,
            &preferences,
            AdapterAvailability::Unavailable,
            Observation::Unknown,
            InteractionGuard::Free,
        ),
        ShellOutcome::ShowQuitConfirmation(ShellController::quit_notice(Observation::Unknown))
    );
}

/// Historical running/stopped values cannot supply a current continuity claim.
///
/// # Panics
/// Fails when the stated UI-only contract or a healthy neighboring path regresses.
#[test]
fn stale_running_and_stale_stopped_are_unconfirmed_in_the_current_quit_warning() {
    for state in [RuntimeState::Running, RuntimeState::Stopped] {
        let notice =
            ShellController::quit_notice(Observation::Stale(ServicePairRuntime::new(state, state)));
        assert_eq!(notice.service_state(), QuitServiceState::Unconfirmed);
    }
}

/// Partial and transitioning pairs retain distinct warning meanings.
///
/// # Panics
/// Fails when the stated UI-only contract or a healthy neighboring path regresses.
#[test]
fn current_partial_or_pending_pairs_do_not_claim_all_services_stopped_or_running() {
    assert_eq!(
        ShellController::quit_notice(Observation::Current(ServicePairRuntime::new(
            RuntimeState::Running,
            RuntimeState::Stopped,
        )))
        .service_state(),
        QuitServiceState::PartiallyRunning
    );
    assert_eq!(
        ShellController::quit_notice(Observation::Current(ServicePairRuntime::new(
            RuntimeState::Stopped,
            RuntimeState::StopPending,
        )))
        .service_state(),
        QuitServiceState::Transitioning
    );
    for (state, expected) in [
        (RuntimeState::Running, QuitServiceState::Running),
        (RuntimeState::Stopped, QuitServiceState::Stopped),
    ] {
        assert_eq!(
            ShellController::quit_notice(Observation::Current(ServicePairRuntime::new(
                state, state
            )))
            .service_state(),
            expected
        );
    }
}

/// An open warning can project lost observations without settling any operation.
///
/// # Panics
/// Fails when the stated UI-only contract or a healthy neighboring path regresses.
#[test]
fn quit_warning_is_reprojected_from_current_observation_without_mutating_it() {
    let preferences = UiPreferences::default();
    let mut shell = ShellController::default();
    let _warning = shell.request(
        ShellRequest::QuitUi,
        &preferences,
        AdapterAvailability::Unavailable,
        Observation::Current(ServicePairRuntime::new(
            RuntimeState::Running,
            RuntimeState::Running,
        )),
        InteractionGuard::Free,
    );
    assert!(shell.quit_confirmation_open());
    assert_eq!(
        shell.request(
            ShellRequest::QuitUi,
            &preferences,
            AdapterAvailability::Unavailable,
            Observation::Unknown,
            InteractionGuard::Free,
        ),
        ShellOutcome::ShowQuitConfirmation(ShellController::quit_notice(Observation::Unknown))
    );
    assert_eq!(
        ShellController::quit_notice(Observation::Unknown).service_state(),
        QuitServiceState::Unconfirmed
    );
    assert!(shell.quit_confirmation_open());
    assert!(!shell.exit_requested());
}

/// A different modal retains interaction ownership against all shell entrypoints.
///
/// # Panics
/// Fails when the stated UI-only contract or a healthy neighboring path regresses.
#[test]
fn another_modal_cannot_be_replaced_by_tray_navigation_or_quit() {
    let preferences = UiPreferences::default();
    let mut shell = ShellController::default();
    for request in [
        ShellRequest::CloseWindow,
        ShellRequest::QuitUi,
        ShellRequest::Menu(TrayMenuAction::Diagnostics),
        ShellRequest::ManageService,
    ] {
        assert_eq!(
            shell.request(
                request,
                &preferences,
                AdapterAvailability::Ready,
                Observation::Unknown,
                InteractionGuard::AnotherDialog,
            ),
            ShellOutcome::KeepOpen(KeepOpenReason::AnotherDialog)
        );
    }
    assert!(!shell.quit_confirmation_open());
    assert!(!shell.exit_requested());
}

/// An open quit warning cannot be replaced by an unrelated tray navigation.
///
/// # Panics
/// Fails when the stated UI-only contract or a healthy neighboring path regresses.
#[test]
fn quit_confirmation_blocks_unrelated_menu_navigation_until_resolved() {
    let preferences = UiPreferences::default();
    let mut shell = ShellController::default();
    let _warning = shell.request(
        ShellRequest::QuitUi,
        &preferences,
        AdapterAvailability::Ready,
        Observation::Unknown,
        InteractionGuard::Free,
    );
    assert_eq!(
        shell.request(
            ShellRequest::Menu(TrayMenuAction::About),
            &preferences,
            AdapterAvailability::Ready,
            Observation::Unknown,
            InteractionGuard::Free,
        ),
        ShellOutcome::KeepOpen(KeepOpenReason::QuitConfirmation)
    );
    assert!(shell.quit_confirmation_open());
}

/// Each menu action maps to navigation or unavailable measurements only.
///
/// # Panics
/// Fails when the stated UI-only contract or a healthy neighboring path regresses.
#[test]
fn menu_navigation_and_sync_request_are_separate_from_service_effects() {
    let preferences = UiPreferences::default();
    let mut shell = ShellController::default();
    for (action, route) in [
        (TrayMenuAction::OpenNevroTick, ShellRoute::Status),
        (TrayMenuAction::Diagnostics, ShellRoute::Diagnostics),
        (
            TrayMenuAction::Settings,
            ShellRoute::Settings(SettingsSection::Overview),
        ),
        (
            TrayMenuAction::TimeManagement,
            ShellRoute::Settings(SettingsSection::BackgroundService),
        ),
        (TrayMenuAction::About, ShellRoute::About),
    ] {
        assert_eq!(
            shell.request(
                ShellRequest::Menu(action),
                &preferences,
                AdapterAvailability::Unavailable,
                Observation::Unknown,
                InteractionGuard::Free,
            ),
            ShellOutcome::Navigate(route)
        );
    }
    assert_eq!(
        shell.request(
            ShellRequest::Menu(TrayMenuAction::RequestMeasurements),
            &preferences,
            AdapterAvailability::Unavailable,
            Observation::Unknown,
            InteractionGuard::Free,
        ),
        ShellOutcome::MeasurementsUnavailable
    );
}

/// Every fixture management request returns unavailable and retains all observations.
///
/// # Panics
/// Fails when the stated UI-only contract or a healthy neighboring path regresses.
#[test]
fn unavailable_service_requests_leave_all_four_observation_axes_unknown() {
    let port = FixtureServicePort;
    let before = port.observations();
    assert_eq!(before.runtime(), Observation::Unknown);
    assert_eq!(before.startup(), Observation::Unknown);
    assert_eq!(before.clock_owner(), Observation::<ClockOwner>::Unknown);
    assert_eq!(
        before.synchronization(),
        Observation::<SynchronizationState>::Unknown
    );
    for command in [
        ServiceCommand::Start,
        ServiceCommand::Stop,
        ServiceCommand::Restart,
        ServiceCommand::SetAutomaticStart(true),
        ServiceCommand::SetAutomaticStart(false),
        ServiceCommand::DisableNevroTick,
        ServiceCommand::UseWindowsTime,
        ServiceCommand::UseNevroTick,
    ] {
        let attempt = port.request(command);
        assert_eq!(attempt.command(), command);
        assert_eq!(attempt.outcome(), ServiceRequestOutcome::AdapterUnavailable);
        assert_eq!(port.observations(), before);
    }
}

//! Pure emergency confirmation contract; no native effect or catalog is created.
#![forbid(unsafe_code)]

use nevrotick_ui::preferences::UiPreferences;
use nevrotick_ui::shell::{
    AdapterAvailability, InteractionGuard, KeepOpenReason, Observation, ShellController,
    ShellOutcome, ShellRequest, TrayMenuAction,
};

/// Emergency consent remains separately armed and performs no effect on opening.
///
/// # Panics
/// Fails if another modal or a forged ordinary confirmation can arm or exit.
#[test]
fn emergency_close_requires_free_private_ownership_and_explicit_confirmation() {
    let preferences = UiPreferences::default();
    let mut shell = ShellController::default();
    assert!(!shell.emergency_armed());
    assert_eq!(
        shell.request_emergency_close(InteractionGuard::AnotherDialog),
        ShellOutcome::KeepOpen(KeepOpenReason::AnotherDialog)
    );
    assert!(!shell.emergency_armed());
    assert!(!shell.quit_confirmation_open());
    assert_eq!(
        shell.request(
            ShellRequest::ConfirmQuit,
            &preferences,
            AdapterAvailability::Unavailable,
            Observation::Unknown,
            InteractionGuard::Free
        ),
        ShellOutcome::KeepOpen(KeepOpenReason::NoQuitConfirmation)
    );
    assert!(matches!(
        shell.request_emergency_close(InteractionGuard::Free),
        ShellOutcome::ShowQuitConfirmation(_)
    ));
    assert!(shell.emergency_armed());
    assert!(shell.quit_confirmation_open());
    assert!(!shell.exit_requested());
}

/// Optional tray readiness cannot turn the fixed emergency warning into a trap.
///
/// # Panics
/// Fails if tray preference changes, confirmation hides without an exit request,
/// or an already-requested exit admits another warning.
#[test]
fn confirmed_emergency_requests_ui_exit_even_with_tray_on_and_adapter_unavailable() {
    let mut preferences = UiPreferences::default();
    preferences.set_tray_enabled(true);
    let before = preferences.clone();
    let mut shell = ShellController::default();
    let _warning = shell.request_emergency_close(InteractionGuard::Free);
    assert_eq!(
        shell.request(
            ShellRequest::ConfirmQuit,
            &preferences,
            AdapterAvailability::Unavailable,
            Observation::Unknown,
            InteractionGuard::Free
        ),
        ShellOutcome::ExitUiRequested
    );
    assert!(shell.exit_requested());
    assert!(!shell.emergency_armed());
    assert!(!shell.quit_confirmation_open());
    assert_eq!(preferences, before);
    assert_eq!(
        shell.request_emergency_close(InteractionGuard::Free),
        ShellOutcome::KeepOpen(KeepOpenReason::ExitAlreadyRequested)
    );
}

/// One emergency warning cannot be replaced by ordinary tray or management intents.
///
/// # Panics
/// Fails if hidden/disabled Manage or any menu identity bypasses current ownership.
#[test]
fn emergency_warning_denies_every_non_resolution_request_at_its_controller() {
    let preferences = UiPreferences::default();
    let mut shell = ShellController::default();
    let _warning = shell.request_emergency_close(InteractionGuard::Free);
    for request in [
        ShellRequest::CloseWindow,
        ShellRequest::QuitUi,
        ShellRequest::ManageService,
        ShellRequest::Menu(TrayMenuAction::OpenNevroTick),
        ShellRequest::Menu(TrayMenuAction::RequestMeasurements),
        ShellRequest::Menu(TrayMenuAction::TimeManagement),
        ShellRequest::Menu(TrayMenuAction::Diagnostics),
        ShellRequest::Menu(TrayMenuAction::Settings),
        ShellRequest::Menu(TrayMenuAction::About),
        ShellRequest::Menu(TrayMenuAction::QuitUi),
    ] {
        assert_eq!(
            shell.request(
                request,
                &preferences,
                AdapterAvailability::Ready,
                Observation::Unknown,
                InteractionGuard::Free
            ),
            ShellOutcome::KeepOpen(KeepOpenReason::QuitConfirmation)
        );
        assert!(shell.emergency_armed());
        assert!(shell.quit_confirmation_open());
        assert!(!shell.exit_requested());
    }
}

/// Cancel drops emergency consent; a later failed exit must obtain new consent.
///
/// # Panics
/// Fails if Cancel or a host failure leaves reusable emergency consent behind.
#[test]
fn cancel_and_failed_exit_drop_emergency_consent_without_changing_preferences() {
    let mut preferences = UiPreferences::default();
    preferences.set_tray_enabled(true);
    let before = preferences.clone();
    let mut shell = ShellController::default();
    let _warning = shell.request_emergency_close(InteractionGuard::Free);
    assert_eq!(
        shell.request(
            ShellRequest::CancelQuit,
            &preferences,
            AdapterAvailability::Unavailable,
            Observation::Unknown,
            InteractionGuard::Free
        ),
        ShellOutcome::KeepOpen(KeepOpenReason::QuitCancelled)
    );
    assert!(!shell.emergency_armed());
    assert!(!shell.quit_confirmation_open());
    assert!(!shell.exit_requested());
    // The ordinary healthy close rule is unchanged: no invisible unready tray.
    assert_eq!(
        shell.request(
            ShellRequest::CloseWindow,
            &preferences,
            AdapterAvailability::Unavailable,
            Observation::Unknown,
            InteractionGuard::Free
        ),
        ShellOutcome::KeepOpen(KeepOpenReason::TrayUnavailable)
    );
    let _warning = shell.request_emergency_close(InteractionGuard::Free);
    let _exit = shell.request(
        ShellRequest::ConfirmQuit,
        &preferences,
        AdapterAvailability::Unavailable,
        Observation::Unknown,
        InteractionGuard::Free,
    );
    assert_eq!(
        shell.exit_not_completed(),
        ShellOutcome::KeepOpen(KeepOpenReason::ExitNotCompleted)
    );
    assert!(!shell.emergency_armed());
    assert!(!shell.quit_confirmation_open());
    assert!(!shell.exit_requested());
    assert_eq!(preferences, before);
    assert!(matches!(
        shell.request_emergency_close(InteractionGuard::Free),
        ShellOutcome::ShowQuitConfirmation(_)
    ));
}

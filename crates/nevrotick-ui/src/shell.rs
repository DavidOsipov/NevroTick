//! UI-owned tray/quit decisions and unavailable service presentation interfaces.
//!
//! This is a fixture-only boundary. It performs no OS/window/service/clock work,
//! stores no accepted preferences and creates no observer. A host interprets an
//! outcome as a request, never as evidence that an effect occurred. Native
//! adapters and production snapshot/correlation contracts remain separate work.
#![forbid(unsafe_code)]

use crate::preferences::UiPreferences;

/// Current readiness reported by one separately owned native adapter.
///
/// A saved preference cannot establish readiness. The fixture uses Unavailable;
/// Ready is meaningful only after the corresponding native adapter is qualified.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdapterAvailability {
    /// The adapter is not connected in this fixture or is known unavailable.
    Unavailable,
    /// Readiness has not been established or its observation was lost.
    Unknown,
    /// The adapter currently reports it can perform its narrow operation.
    Ready,
}

/// Presentation freshness supplied by a future canonical observation owner.
///
/// This is not a wire type, timestamp policy or permission. Stale values may be
/// retained for history but must not supply a current positive claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Observation<T> {
    /// No usable observation exists.
    Unknown,
    /// The observation owner currently considers this value usable.
    Current(T),
    /// Retained history; current state is unconfirmed.
    Stale(T),
}

/// Runtime observation for one member of the `NevroTick` service pair.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeState {
    /// The service is currently observed running; this proves no synchronization.
    Running,
    /// The service is currently observed stopped; startup remains independent.
    Stopped,
    /// A start transition is observed and has not settled.
    StartPending,
    /// A stop transition is observed and has not settled.
    StopPending,
}

/// Presentation-only runtime values for the engine and clock controller.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ServicePairRuntime {
    /// Current engine runtime value; freshness is owned by the outer observation.
    engine: RuntimeState,
    /// Current clock-controller runtime value.
    controller: RuntimeState,
}

impl ServicePairRuntime {
    /// Pair already supplied observations without inferring clock ownership.
    #[must_use]
    pub const fn new(engine: RuntimeState, controller: RuntimeState) -> Self {
        Self { engine, controller }
    }
}

/// Current startup configuration for one service, independent of runtime.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StartupState {
    /// Automatic startup has been observed.
    Automatic,
    /// Manual/triggered startup is allowed; current execution is independent.
    Manual,
    /// Startup is disabled; this alone does not prove current execution stopped.
    Disabled,
}

/// Startup values for both services, without collapsing partial configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ServicePairStartup {
    /// Engine startup configuration.
    pub engine: StartupState,
    /// Clock controller startup configuration.
    pub controller: StartupState,
}

/// Separately observed clock ownership; no variant proves synchronization.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClockOwner {
    /// `NevroTick` currently owns clock control.
    NevroTick,
    /// Windows Time currently owns clock control.
    WindowsTime,
    /// Another owner or a conflict has been observed.
    OtherOrConflicting,
}

/// Verification observation, independent of runtime, startup and ownership.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SynchronizationState {
    /// The observation owner supplied current verified synchronization evidence.
    Established,
    /// Synchronization has not been established by the supplied observation.
    NotEstablished,
}

/// Four distinct UI observation axes, never modified by a request result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ServiceObservations {
    /// Service-pair execution and freshness.
    runtime: Observation<ServicePairRuntime>,
    /// Independent pair startup configuration and freshness.
    startup: Observation<ServicePairStartup>,
    /// Independent clock-controller ownership and freshness.
    clock_owner: Observation<ClockOwner>,
    /// Independent synchronization evidence and freshness.
    synchronization: Observation<SynchronizationState>,
}

impl ServiceObservations {
    /// Return runtime evidence without deriving any other observation.
    #[must_use]
    pub const fn runtime(self) -> Observation<ServicePairRuntime> {
        self.runtime
    }

    /// Return startup evidence independently from execution.
    #[must_use]
    pub const fn startup(self) -> Observation<ServicePairStartup> {
        self.startup
    }

    /// Return clock ownership evidence without implying correct time.
    #[must_use]
    pub const fn clock_owner(self) -> Observation<ClockOwner> {
        self.clock_owner
    }

    /// Return synchronization evidence independently from ownership and runtime.
    #[must_use]
    pub const fn synchronization(self) -> Observation<SynchronizationState> {
        self.synchronization
    }
}

/// Closed UI command identity; these values carry no authorization or wire data.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServiceCommand {
    /// Request current service-pair startup.
    Start,
    /// Request temporary service-pair stop, without changing startup preference.
    Stop,
    /// Request current service-pair restart.
    Restart,
    /// Request future automatic startup configuration, independently of runtime.
    SetAutomaticStart(bool),
    /// Request a separately confirmed stop/start inhibition/owned-cleanup flow.
    DisableNevroTick,
    /// Request a separately confirmed Windows Time handover flow.
    UseWindowsTime,
    /// Request a separately confirmed `NevroTick` handover flow.
    UseNevroTick,
}

/// A fixture request result; there is deliberately no fabricated success state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServiceRequestOutcome {
    /// Nothing was submitted because the management adapter is disconnected.
    AdapterUnavailable,
}

/// A separate UI request result that must not overwrite service observations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use]
pub struct ServiceRequestAttempt {
    /// Closed action identity for field-local feedback.
    command: ServiceCommand,
    /// A request result, never an observation update.
    outcome: ServiceRequestOutcome,
}

impl ServiceRequestAttempt {
    /// Return which user action was rejected before submission.
    #[must_use]
    pub const fn command(self) -> ServiceCommand {
        self.command
    }

    /// Return request feedback, independently from all observation axes.
    #[must_use]
    pub const fn outcome(self) -> ServiceRequestOutcome {
        self.outcome
    }
}

/// Narrow presentation/management port with no generic command executor.
///
/// A real implementation requires the existing broker/helper authorization,
/// operation identity and pending/unknown/partial result contracts. This fixture
/// interface does not admit or specify that production implementation.
pub trait ServiceManagementPort {
    /// Observe facts separately from submitting a request.
    fn observations(&self) -> ServiceObservations;

    /// Attempt one typed UI request; an outcome is not a runtime observation.
    fn request(&self, command: ServiceCommand) -> ServiceRequestAttempt;
}

/// Disconnected fixture port: no transport, persistence, elevation or OS effects.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FixtureServicePort;

impl ServiceManagementPort for FixtureServicePort {
    fn observations(&self) -> ServiceObservations {
        ServiceObservations {
            runtime: Observation::Unknown,
            startup: Observation::Unknown,
            clock_owner: Observation::Unknown,
            synchronization: Observation::Unknown,
        }
    }

    fn request(&self, command: ServiceCommand) -> ServiceRequestAttempt {
        ServiceRequestAttempt {
            command,
            outcome: ServiceRequestOutcome::AdapterUnavailable,
        }
    }
}

/// A shared Settings anchor, not an administrative action.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SettingsSection {
    /// Settings entry point.
    Overview,
    /// Background service summary/management section, reachable from both modes.
    BackgroundService,
    /// Local notification preference and availability section.
    Notifications,
    /// Power-mode local profile/recovery previews; no protected activation.
    TimePolicy,
    /// Shared tray/sign-in/language preferences, separate from service startup.
    PersonalPreferences,
}

/// Navigation intent interpreted by the existing checked session owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShellRoute {
    /// Open the shared Status page, keeping the current mode.
    Status,
    /// Open Diagnostics through an explicitly presented Power transition.
    Diagnostics,
    /// Inspect sources through the same explicit, atomic Power transition.
    Sources,
    /// Open the read-only diagnostic correction review, granting no approval.
    CorrectionReview,
    /// Open shared Settings and its named section without forcing Power.
    Settings(SettingsSection),
    /// Open shared About, keeping the current mode.
    About,
}

/// Semantic tray-menu actions; opening the menu itself dispatches none of them.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrayMenuAction {
    /// Open Status.
    OpenNevroTick,
    /// Request measurements only; unavailable in this fixture.
    RequestMeasurements,
    /// Open shared service management without a service command.
    TimeManagement,
    /// Open Diagnostics through the existing explicit Power transition.
    Diagnostics,
    /// Open shared Settings.
    Settings,
    /// Open shared About.
    About,
    /// Begin the UI-only exit confirmation.
    QuitUi,
}

/// A private receiving owner's current interaction guard, not a view flag.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InteractionGuard {
    /// No separate modal owns an in-progress interaction.
    Free,
    /// Another modal must not be replaced by shell navigation or quit.
    AnotherDialog,
}

/// Closed shell intents, with direct confirmation checked against private state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShellRequest {
    /// Native caption Close; differs from an explicit Quit UI action.
    CloseWindow,
    /// Explicit exit intent from a menu or application action.
    QuitUi,
    /// Keep the observer open and dismiss its quit confirmation.
    CancelQuit,
    /// Confirm a currently open warning; no service command is carried.
    ConfirmQuit,
    /// Dismiss the quit warning and open shared service settings.
    ManageService,
    /// Invoke one explicit tray-menu action.
    Menu(TrayMenuAction),
}

/// Why a shell intent retained the existing observer instead of replacing it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeepOpenReason {
    /// A configured tray preference has no confirmed usable adapter.
    TrayUnavailable,
    /// The user cancelled quit; no preference/service state was changed.
    QuitCancelled,
    /// Direct Confirm had no privately owned open quit confirmation.
    NoQuitConfirmation,
    /// Another modal interaction must remain intact.
    AnotherDialog,
    /// The quit confirmation must be resolved before unrelated navigation.
    QuitConfirmation,
    /// UI exit was already requested; no effect is repeated.
    ExitAlreadyRequested,
    /// The host could not complete its exit request; retry remains explicit.
    ExitNotCompleted,
}

/// Truthful current service continuity classification for the exit warning.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuitServiceState {
    /// Both services are currently observed running, without timekeeping claims.
    Running,
    /// Both services are currently observed stopped.
    Stopped,
    /// One service is running while the other is stopped.
    PartiallyRunning,
    /// At least one current service transition remains unsettled.
    Transitioning,
    /// No current pair-runtime observation is available.
    Unconfirmed,
}

/// Code-owned warning meaning; a future catalog supplies its translated text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QuitNotice(
    /// Current service classification, supplied without raw text.
    QuitServiceState,
);

impl QuitNotice {
    /// Return the current, freshness-aware service classification.
    #[must_use]
    pub const fn service_state(self) -> QuitServiceState {
        self.0
    }

    /// Return fixed English fixture copy without embedding untrusted data.
    #[must_use]
    pub const fn headline(self) -> &'static str {
        match self.0 {
            QuitServiceState::Running => "Background services keep running",
            QuitServiceState::Stopped => "Services are currently stopped",
            QuitServiceState::PartiallyRunning => "Part of the service pair is running",
            QuitServiceState::Transitioning => "Service state is changing",
            QuitServiceState::Unconfirmed => "Service state is unconfirmed",
        }
    }

    /// Explain UI-only consequences without reporting an effect or new evidence.
    #[must_use]
    pub const fn consequence(self) -> &'static str {
        match self.0 {
            QuitServiceState::Running => {
                "Quitting closes only the app observer. Running background services continue."
            }
            QuitServiceState::Stopped => {
                "Quitting closes only the app observer. It does not change service startup or clock ownership."
            }
            QuitServiceState::PartiallyRunning => {
                "The service pair is not fully stopped. Quitting closes only the app observer and does not stop either service."
            }
            QuitServiceState::Transitioning => {
                "A service transition is not settled. Quitting the app neither completes nor cancels that operation, and does not establish the result."
            }
            QuitServiceState::Unconfirmed => {
                "Quitting closes only the app observer. It does not stop services or confirm whether they are running or stopped."
            }
        }
    }

    /// Explain the separate temporary-stop path without invoking it.
    #[must_use]
    pub const fn stop_path() -> &'static str {
        "To stop temporarily: Settings > Background service > Stop"
    }

    /// Explain the separate future-start prevention path without invoking it.
    #[must_use]
    pub const fn disable_path() -> &'static str {
        "To prevent future starts: Settings > Background service > Disable NevroTick"
    }

    /// Describe observer availability separately from background timekeeping.
    #[must_use]
    pub const fn observer_notice() -> &'static str {
        "New local notifications stop when the last app or tray observer exits."
    }
}

/// Requested UI presentation/effect; none is evidence that native work happened.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use]
pub enum ShellOutcome {
    /// Keep the current observer, with a fixed actionable explanation.
    KeepOpen(KeepOpenReason),
    /// Present or refresh the privately owned quit warning.
    ShowQuitConfirmation(QuitNotice),
    /// Navigate without a service, clock, policy or notification command.
    Navigate(ShellRoute),
    /// A usable native tray may hide the window; its host owns acknowledgment.
    HideToTrayRequested,
    /// The host may exit the UI; no claim of process exit is made here.
    ExitUiRequested,
    /// No measurement/backend adapter exists in the current fixture.
    MeasurementsUnavailable,
}

/// UI-thread-owned quit state; accepted preferences remain in `UiPreferences`.
#[derive(Debug, Default)]
pub struct ShellController {
    /// Whether this owner has presented an unresolved exit warning.
    quit_confirmation_open: bool,
    /// Whether the host has been asked to exit; this is no completion evidence.
    exit_requested: bool,
    /// Fixed emergency consent armed only by the owner's invariant-failed Close path.
    emergency_armed: bool,
}

impl ShellController {
    /// Return whether this controller owns a pending quit confirmation.
    #[must_use]
    pub const fn quit_confirmation_open(&self) -> bool {
        self.quit_confirmation_open
    }

    /// Whether this same controller owns the fixed emergency confirmation.
    #[must_use]
    pub const fn emergency_armed(&self) -> bool {
        self.emergency_armed
    }

    /// Arm fixed UI-only emergency consent without trusting optional tray readiness.
    ///
    /// The receiving root must independently establish its private programmer-
    /// invariant failure. Ordinary rejected catalog input must never call this.
    /// This only opens the same warning; it does not hide, quit, mutate preferences
    /// or request any service/clock operation. Confirm and Cancel remain separate.
    pub fn request_emergency_close(&mut self, interaction: InteractionGuard) -> ShellOutcome {
        if self.exit_requested {
            return ShellOutcome::KeepOpen(KeepOpenReason::ExitAlreadyRequested);
        }
        if interaction == InteractionGuard::AnotherDialog {
            return ShellOutcome::KeepOpen(KeepOpenReason::AnotherDialog);
        }
        self.emergency_armed = true;
        self.quit_confirmation_open = true;
        ShellOutcome::ShowQuitConfirmation(QuitNotice(QuitServiceState::Unconfirmed))
    }

    /// Return requested exit, not proof that an observer/process has ended.
    #[must_use]
    pub const fn exit_requested(&self) -> bool {
        self.exit_requested
    }

    /// Record a failed host exit attempt without inventing observer termination.
    ///
    /// Only the owning host reports this outcome; it is not a public view callback.
    /// The observer stays available and a later user request may try again.
    pub fn exit_not_completed(&mut self) -> ShellOutcome {
        self.exit_requested = false;
        self.quit_confirmation_open = false;
        self.emergency_armed = false;
        ShellOutcome::KeepOpen(KeepOpenReason::ExitNotCompleted)
    }

    /// Reproject warning from current observations; stale history is unconfirmed.
    #[must_use]
    pub const fn quit_notice(runtime: Observation<ServicePairRuntime>) -> QuitNotice {
        let state = match runtime {
            Observation::Unknown | Observation::Stale(_) => QuitServiceState::Unconfirmed,
            Observation::Current(pair) => match (pair.engine, pair.controller) {
                (RuntimeState::Running, RuntimeState::Running) => QuitServiceState::Running,
                (RuntimeState::Stopped, RuntimeState::Stopped) => QuitServiceState::Stopped,
                (RuntimeState::Running, RuntimeState::Stopped)
                | (RuntimeState::Stopped, RuntimeState::Running) => {
                    QuitServiceState::PartiallyRunning
                }
                _ => QuitServiceState::Transitioning,
            },
        };
        QuitNotice(state)
    }

    /// Decide one guarded UI request with immutable accepted preferences/facts.
    ///
    /// The host supplies its privately owned modal guard and adapter readiness;
    /// public view properties must not supply either. No native effect, service
    /// observation or accepted preference is changed by this method.
    pub fn request(
        &mut self,
        request: ShellRequest,
        preferences: &UiPreferences,
        tray: AdapterAvailability,
        runtime: Observation<ServicePairRuntime>,
        interaction: InteractionGuard,
    ) -> ShellOutcome {
        if self.exit_requested {
            return ShellOutcome::KeepOpen(KeepOpenReason::ExitAlreadyRequested);
        }
        if interaction == InteractionGuard::AnotherDialog {
            return ShellOutcome::KeepOpen(KeepOpenReason::AnotherDialog);
        }
        if self.quit_confirmation_open {
            return self.resolve_quit(request, runtime);
        }
        match request {
            ShellRequest::CloseWindow if !preferences.tray_enabled() => self.request_exit(),
            ShellRequest::CloseWindow if tray == AdapterAvailability::Ready => {
                ShellOutcome::HideToTrayRequested
            }
            ShellRequest::CloseWindow => ShellOutcome::KeepOpen(KeepOpenReason::TrayUnavailable),
            ShellRequest::QuitUi | ShellRequest::Menu(TrayMenuAction::QuitUi) => {
                self.quit_confirmation_open = true;
                ShellOutcome::ShowQuitConfirmation(Self::quit_notice(runtime))
            }
            ShellRequest::ConfirmQuit | ShellRequest::CancelQuit => {
                ShellOutcome::KeepOpen(KeepOpenReason::NoQuitConfirmation)
            }
            ShellRequest::ManageService => {
                ShellOutcome::Navigate(ShellRoute::Settings(SettingsSection::BackgroundService))
            }
            ShellRequest::Menu(action) => Self::menu_action(action),
        }
    }

    /// Resolve only this controller's current warning; unrelated work is held.
    fn resolve_quit(
        &mut self,
        request: ShellRequest,
        runtime: Observation<ServicePairRuntime>,
    ) -> ShellOutcome {
        if self.emergency_armed
            && !matches!(
                request,
                ShellRequest::ConfirmQuit | ShellRequest::CancelQuit
            )
        {
            return ShellOutcome::KeepOpen(KeepOpenReason::QuitConfirmation);
        }
        match request {
            ShellRequest::ConfirmQuit => {
                self.quit_confirmation_open = false;
                self.request_exit()
            }
            ShellRequest::CancelQuit => {
                self.emergency_armed = false;
                self.quit_confirmation_open = false;
                ShellOutcome::KeepOpen(KeepOpenReason::QuitCancelled)
            }
            ShellRequest::ManageService => {
                self.quit_confirmation_open = false;
                ShellOutcome::Navigate(ShellRoute::Settings(SettingsSection::BackgroundService))
            }
            ShellRequest::QuitUi | ShellRequest::Menu(TrayMenuAction::QuitUi) => {
                ShellOutcome::ShowQuitConfirmation(Self::quit_notice(runtime))
            }
            _ => ShellOutcome::KeepOpen(KeepOpenReason::QuitConfirmation),
        }
    }

    /// Record a request only; the actual host/adapter owns exit completion.
    fn request_exit(&mut self) -> ShellOutcome {
        self.emergency_armed = false;
        self.exit_requested = true;
        ShellOutcome::ExitUiRequested
    }

    /// Map explicit menu actions to narrow routes or an honest unavailable result.
    const fn menu_action(action: TrayMenuAction) -> ShellOutcome {
        match action {
            TrayMenuAction::OpenNevroTick => ShellOutcome::Navigate(ShellRoute::Status),
            TrayMenuAction::RequestMeasurements => ShellOutcome::MeasurementsUnavailable,
            TrayMenuAction::TimeManagement => {
                ShellOutcome::Navigate(ShellRoute::Settings(SettingsSection::BackgroundService))
            }
            TrayMenuAction::Diagnostics => ShellOutcome::Navigate(ShellRoute::Diagnostics),
            TrayMenuAction::Settings => {
                ShellOutcome::Navigate(ShellRoute::Settings(SettingsSection::Overview))
            }
            TrayMenuAction::About => ShellOutcome::Navigate(ShellRoute::About),
            TrayMenuAction::QuitUi => ShellOutcome::KeepOpen(KeepOpenReason::NoQuitConfirmation),
        }
    }
}

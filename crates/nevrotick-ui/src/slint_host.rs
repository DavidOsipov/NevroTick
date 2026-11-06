//! UI-thread Slint bindings for the checked, in-memory fixture session.
//!
//! [`SourceDrafts`] is the sole reached source owner; [`NavigationState`] owns
//! mode/page only. Generated properties and observable source/review models are
//! projections; bounded raw drafts have a separate Rust owner.
//! Callbacks retain weak window handles, release session borrows before toolkit
//! notifications, and cannot perform network, service, policy or clock work.

use std::{cell::RefCell, fmt, rc::Rc};

use crate::localization::{MessageId, presentation::UiText};
use catalog_workflow::{FeedbackSlot, write, write_many};
use slint::{ComponentHandle, VecModel};

use crate::policy::PolicyDrafts;
use crate::preferences::{CheckInterval, UiPreferences};
use crate::presentation::{Mode, NavigationState, Page as SessionPage};
use crate::recovery::RecoveryDrafts;
use crate::shell::SettingsSection;
use crate::source_drafts::{ProtocolPortDraft, SourceDraftError, SourceDrafts, SourceId};
use crate::source_inspection::SourceInspection;
use crate::ui_drafts::{IntervalFields, ResetInput, UiDrafts};

/// One protected catalog prepares one complete generated copy value.
mod catalog_projection;
/// Catalog publication shares the existing receiving guards.
mod catalog_workflow;
/// Read-only diagnostic subviews and local collection preference illustrations.
mod diagnostic_workflow;
/// Separate bounded list query/detail projection over the one source owner.
mod inspection_workflow;
/// Explicit local profile preview/review; no policy activation.
mod policy_workflow;
pub use catalog_workflow::CatalogUpdateError;

/// Native keyboard/IME ingress guard; ordinary fixture creation installs no hook.
#[cfg(any(windows, test))]
mod native_input;
/// Pure wake/stop custody controls shared by the Windows native owner.
#[cfg(any(windows, test))]
mod native_lifecycle;
/// Native notification receiver remains dormant for ordinary fixture creation.
mod native_notification_workflow;
/// Windows-only UI-thread ownership; no Root state is sent to native workers.
#[cfg(windows)]
mod native_workflow;
/// Bounded raw drafts and guarded widget resynchronization.
mod raw_workflow;
/// Recovery helper/edit/leave navigation bindings; no live recovery effects.
mod recovery_workflow;
/// Guarded UI exit and shared Settings destinations; native ports stay unavailable.
mod shell_workflow;
#[cfg(windows)]
pub use native_workflow::NativeHostError;
/// Source/editor projection and callbacks; no backend effect paths.
mod source_workflow;
/// Explicit disconnected status input and guarded contextual navigation.
mod status_workflow;
use source_workflow::{SourceProjection, selected_editor};

/// Generated component and presentation types from the maintained Slint entry.
#[expect(
    unused_qualifications,
    reason = "Slint 1.18.1 generated Rust qualifies toolkit-exported standard types through its sp namespace"
)]
#[expect(
    unreachable_pub,
    reason = "Slint 1.18.1 emits public implementation items inside its generated private modules"
)]
#[expect(
    missing_docs,
    reason = "Slint 1.18.1 emits undocumented accessors and presentation types; handwritten owner contracts remain documented outside this module"
)]
#[expect(
    let_underscore_drop,
    reason = "Slint 1.18.1 generated initialization deliberately discards OnceCell set results holding weak global references"
)]
#[expect(
    clippy::as_conversions,
    reason = "Slint 1.18.1 generated Rust uses explicit casts for toolkit property and callback representations; handwritten receiving conversions remain checked"
)]
#[cfg_attr(
    not(test),
    expect(
        clippy::missing_docs_in_private_items,
        reason = "Slint 1.18.1 generates undocumented implementation details; handwritten private items retain their documentation contracts"
    )
)]
#[expect(
    clippy::unwrap_used,
    reason = "Slint 1.18.1 generated component code unwraps initialized globals and weak item-tree handles under toolkit lifecycle invariants"
)]
#[expect(
    clippy::let_underscore_must_use,
    reason = "Slint 1.18.1 generated initialization intentionally discards weak-global OnceCell set results"
)]
#[expect(
    clippy::panic,
    reason = "Slint 1.18.1 generated item-tree vtables panic on invalid internal dynamic-tree indices; raw requests do not supply toolkit indices"
)]
#[expect(
    clippy::todo,
    reason = "Slint 1.18.1 emits an unsupported embedded Rust-component vtable branch; this application uses a top-level generated window only"
)]
mod generated {
    slint::include_modules!();
}

pub use generated::{AppWindow, Page, SourceChangeRow, SourceRow};

/// Fixed rejection when selection would replace an active editor's drafts.
const EDITOR_ACTIVE_MESSAGE: &str =
    "Finish or cancel the current sample edit before selecting a source.";

/// Fixed rejection for direct update calls without an active Rust-owned editor.
const EDITOR_CLOSED_MESSAGE: &str = "Open the sample source editor before updating its preview.";

/// Failure to construct the checked fixture or create/run its toolkit window.
#[derive(Debug)]
pub enum HostError {
    /// Maintained fixture input failed its existing checked constructor.
    Fixture(SourceDraftError),
    /// The selected Slint platform could not create or run the window.
    Platform(slint::PlatformError),
    /// Internal checked fixture values could not seed bounded raw drafts.
    Drafts,
    /// The Windows production observer host could not be safely installed.
    #[cfg(windows)]
    Native(NativeHostError),
}

impl fmt::Display for HostError {
    /// Format a typed, bounded fixture cause or a toolkit platform failure.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fixture(error) => write!(formatter, "Cannot initialize the sample UI: {error}"),
            Self::Platform(error) => write!(formatter, "Cannot run the sample UI: {error}"),
            Self::Drafts => formatter.write_str("Cannot initialize checked input drafts."),
            #[cfg(windows)]
            Self::Native(error) => fmt::Display::fmt(error, formatter),
        }
    }
}

impl std::error::Error for HostError {
    /// Preserve the original typed cause without including raw editor strings.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Fixture(error) => Some(error),
            Self::Platform(error) => Some(error),
            Self::Drafts => None,
            #[cfg(windows)]
            Self::Native(error) => Some(error),
        }
    }
}

/// Current editor target, owned by Rust rather than an externally settable flag.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EditorTarget {
    /// No source input may be submitted while the editor is closed.
    Closed,
    /// A checked candidate will receive a fresh session-local ID.
    Add,
    /// A checked candidate may replace only this currently captured source ID.
    Edit(SourceId),
}

/// An inline owner currently visible on its actual Power route.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InlineOwner {
    /// Current source editor or source-review surface, not retained off-page drafts.
    Sources,
    /// Current accepted Date recovery review, not retained off-page history.
    Recovery,
}

/// Sole accepted state plus callback lifecycle guards, owned on the UI thread.
struct BindingState {
    /// Source-independent checked mode/page transitions.
    navigation: NavigationState,
    /// Immutable baseline and all reached local source candidates/review rows.
    sources: SourceDrafts,
    /// One checked local profile preview, never an active policy revision.
    policy: PolicyDrafts,
    /// Query/details for this same source owner; reset if the owner is replaced.
    inspection: SourceInspection,
    /// Checked source-search expansion, retained across ordinary navigation.
    source_search: raw_workflow::SearchVisibility,
    /// Current stable identity, or None when no effective source remains.
    selected: Option<SourceId>,
    /// Accepted process-local preferences; component properties are projections.
    preferences: UiPreferences,
    /// Sole bounded raw editor owner, separate from accepted preview values.
    raw: UiDrafts,
    /// A failed internal reset closes input admission but preserves emergency Quit.
    raw_admission: raw_workflow::RawAdmission,
    /// One approved catalog, trusted copy and retained semantic feedback owner.
    copy: catalog_workflow::CatalogBindingState,
    /// Quit state, shared Settings destination and fixed unavailable ports.
    shell: shell_workflow::ShellBindingState,
    /// Dormant fixture receiver; only explicit native construction installs ports.
    notifications: native_notification_workflow::NotificationBindingState,
    /// UI-thread-only native resources, retained throughout pending cleanup.
    #[cfg(windows)]
    native: Option<native_workflow::NativeBindingState>,
    /// Known disconnected UI input port; it never derives or refreshes evidence.
    status: crate::status_ui::FixtureStatusPort,
    /// Receiving guard independent of externally settable editor properties.
    editor: EditorTarget,
    /// Accepted editor protocol/port and monotone manual-edit provenance.
    port_draft: Option<ProtocolPortDraft>,
    /// Local review visibility; it has no backend result/authorization meaning.
    review_open: bool,
    /// Sole accepted helper preview; raw helper drafts stay in the raw owner.
    recovery: RecoveryDrafts,
    /// Read-only diagnostic navigation; contains no logging or export authority.
    diagnostics: crate::diagnostics_ui::DiagnosticPreview,
    /// Trusted helper review phase, independent of view affordance properties.
    recovery_phase: recovery_workflow::HelperPhase,
    /// Exact mode/page memory from the context that entered Date recovery.
    recovery_return: Option<recovery_workflow::RecoveryContext>,
    /// First pending leave intent; Keep/Discard/Stay resolve this exact request.
    recovery_leave: Option<recovery_workflow::RecoveryLeave>,
    /// True only after initialization and all callback handlers are complete.
    initialized: bool,
    /// Suppress nested requests until synchronous toolkit projection completes.
    projecting: bool,
}

impl BindingState {
    /// Private admission failure is independent of spoofable presentation flags.
    const fn raw_failed(&self) -> bool {
        matches!(self.raw_admission, raw_workflow::RawAdmission::Failed)
    }

    /// Inline input owns interaction only while its actual Power page is current.
    /// Retained drafts on Settings/About/Back remain untouched and are not modals.
    const fn current_inline_owner(&self) -> Option<InlineOwner> {
        if !matches!(self.navigation.mode(), Mode::Power) {
            return None;
        }
        match self.navigation.page() {
            SessionPage::Sources
                if !matches!(self.editor, EditorTarget::Closed) || self.review_open =>
            {
                Some(InlineOwner::Sources)
            }
            SessionPage::Recovery
                if matches!(self.recovery_phase, recovery_workflow::HelperPhase::Review) =>
            {
                Some(InlineOwner::Recovery)
            }
            _ => None,
        }
    }

    /// Current UI-owned native interaction, independently of view properties.
    #[cfg(any(windows, test))]
    fn current_native_interaction(&self) -> crate::tray_ui::TrayInteraction {
        use crate::tray_ui::TrayInteraction;
        let shell = self.shell.interaction();
        if shell != TrayInteraction::Free {
            return shell;
        }
        if self.copy.failed()
            || self.raw_failed()
            || self.recovery_leave.is_some()
            || self.current_inline_owner().is_some()
        {
            TrayInteraction::DialogOwned
        } else {
            TrayInteraction::Free
        }
    }

    /// Search field presence follows this retained presentation context only.
    const fn source_search_open(&self) -> bool {
        matches!(self.source_search, raw_workflow::SearchVisibility::Expanded)
    }

    /// Keep a surviving stable selection, otherwise choose the first or None.
    fn reconcile_selection(&mut self) {
        if self
            .selected
            .is_some_and(|id| self.sources.source(id.view_id()).is_ok())
        {
            return;
        }
        self.selected = self.sources.preview().first().map(|source| source.id());
    }
}

/// UI-thread transition guard, kept alive across synchronous model notifications.
/// No `RefCell` borrow survives a toolkit setter; nested requests are ignored.
struct ProjectionGuard<'a>(&'a RefCell<BindingState>);

impl<'a> ProjectionGuard<'a> {
    /// Enter only an initialized, idle owner; rejection changes no state.
    fn enter(state: &'a RefCell<BindingState>) -> Option<Self> {
        let mut owner = state.borrow_mut();
        if !owner.initialized
            || owner.copy.failed()
            || owner.raw_failed()
            || owner.projecting
            || owner.shell.blocks_other_requests()
            || owner.recovery_leave.is_some()
        {
            return None;
        }
        owner.projecting = true;
        Some(Self(state))
    }

    /// Enter only raw restore handlers while modal commands remain blocked.
    /// The raw receiver independently refuses mutation outside current context.
    fn enter_raw_restore(state: &'a RefCell<BindingState>) -> Option<Self> {
        let mut owner = state.borrow_mut();
        if !owner.initialized || owner.projecting {
            return None;
        }
        owner.projecting = true;
        Some(Self(state))
    }

    /// Enter the shell owner's own modal callbacks while retaining projection guards.
    fn enter_shell(state: &'a RefCell<BindingState>) -> Option<Self> {
        let mut owner = state.borrow_mut();
        if !owner.initialized
            || owner.projecting
            || (owner.recovery_leave.is_some() && !owner.copy.failed() && !owner.raw_failed())
        {
            return None;
        }
        owner.projecting = true;
        Some(Self(state))
    }

    /// Enter native custody progress even while an existing quit/leave modal owns input.
    /// This guard grants no user action or navigation; those are rechecked after drop.
    #[cfg(windows)]
    fn enter_native(state: &'a RefCell<BindingState>) -> Option<Self> {
        let mut owner = state.borrow_mut();
        if !owner.initialized || owner.projecting {
            return None;
        }
        owner.projecting = true;
        Some(Self(state))
    }

    /// Resolve only recovery's own leave modal, never a pending shell dialog or exit.
    fn enter_recovery(state: &'a RefCell<BindingState>) -> Option<Self> {
        let mut owner = state.borrow_mut();
        if !owner.initialized || owner.projecting || owner.shell.blocks_other_requests() {
            return None;
        }
        owner.projecting = true;
        Some(Self(state))
    }
}

impl Drop for ProjectionGuard<'_> {
    /// End the receiving transition after all notifications and setters return.
    fn drop(&mut self) {
        self.0.borrow_mut().projecting = false;
    }
}

/// Strong UI-thread owner of one fixture window and observable source projections.
///
/// A separately cloned component deliberately extends its lifetime. Callbacks
/// capture weak window handles and cannot retain their own component in a cycle.
pub struct SlintHost {
    /// Root component retained until the owner/event loop finishes.
    window: AppWindow,
    /// Custody of the one checked source owner and local lifecycle guards.
    state: Rc<RefCell<BindingState>>,
    /// Custody of observable effective-source projections.
    _rows: Rc<VecModel<SourceRow>>,
    /// Custody of observable local before/after review projections.
    _changes: Rc<VecModel<SourceChangeRow>>,
}

impl SlintHost {
    /// Borrow the generated root on its owning UI thread.
    #[must_use]
    pub const fn window(&self) -> &AppWindow {
        &self.window
    }

    /// Publish a fully admitted, independently approved catalog in one UI transition.
    ///
    /// The receiving owner retains the store; callers supply neither raw payload
    /// bytes nor an approval override. Busy rejection drops only the candidate.
    ///
    /// # Errors
    /// Rejects a busy owner or failed full preparation, retaining the prior view,
    /// catalog, typed feedback, raw drafts, accepted state and current focus.
    pub fn publish_catalog(
        &self,
        catalog: crate::localization::CatalogStore,
    ) -> Result<(), CatalogUpdateError> {
        catalog_workflow::publish(&self.window, &self.state, catalog)
    }

    /// Run this fixture's window while retaining all UI-owned resources.
    ///
    /// # Errors
    /// Returns the selected platform's event-loop or window failure.
    pub fn run(self) -> Result<(), slint::PlatformError> {
        if let Err(error) = self.window.show() {
            #[cfg(windows)]
            if !native_workflow::event_loop_ended(&self.window, &self.state) {
                return Err(slint::PlatformError::from(
                    "App startup failed before native cleanup could be confirmed. Native state is unconfirmed.",
                ));
            }
            // Slint acquires its strong reference before backend show can fail.
            return match self.window.hide() {
                Ok(()) => Err(error),
                Err(_) => Err(slint::PlatformError::from(
                    "App startup and window cleanup failed. Window state is unconfirmed.",
                )),
            };
        }
        // Explicit ownership prevents a failed hide from queuing automatic exit
        // before its bounded recovery. Successful intended Close requests quit.
        let result = slint::run_event_loop_until_quit();
        #[cfg(windows)]
        if !native_workflow::event_loop_ended(&self.window, &self.state) {
            // A stopped/broken platform loop cannot advance its UI timer. This
            // is an outer platform failure, never a confirmed ordinary exit.
            // Retain/show the observer and report unconfirmed native custody.
            return Err(slint::PlatformError::from(
                "The UI event loop ended before native cleanup was confirmed. The window was retained; native state is unconfirmed.",
            ));
        }
        let cleanup = if self.window.window().is_visible() {
            self.window.hide()
        } else {
            Ok(())
        };
        match (result, cleanup) {
            (Err(_), Err(_)) => Err(slint::PlatformError::from(
                "The app event loop and window cleanup failed. Window state is unconfirmed.",
            )),
            (Err(error), Ok(())) | (Ok(()), Err(error)) => Err(error),
            (Ok(()), Ok(())) => Ok(()),
        }
    }
}

/// Create one checked fixture, bind all real callbacks, then publish readiness.
///
/// No worker, timer, transport, persistent data or OS service is initialized.
///
/// # Errors
/// Returns checked fixture construction or toolkit window creation failure.
pub fn create() -> Result<SlintHost, HostError> {
    let sources = SourceDrafts::fixture().map_err(HostError::Fixture)?;
    let selected = sources.preview().first().map(|source| source.id());
    let state = Rc::new(RefCell::new(BindingState {
        navigation: NavigationState::new(),
        sources,
        policy: PolicyDrafts::fixture(),
        inspection: SourceInspection::new(),
        source_search: raw_workflow::SearchVisibility::Collapsed,
        selected,
        preferences: UiPreferences::default(),
        raw: UiDrafts::default(),
        raw_admission: raw_workflow::RawAdmission::Open,
        copy: catalog_workflow::CatalogBindingState::default(),
        shell: shell_workflow::ShellBindingState::new(),
        notifications: native_notification_workflow::NotificationBindingState::default(),
        #[cfg(windows)]
        native: None,
        status: crate::status_ui::FixtureStatusPort,
        editor: EditorTarget::Closed,
        port_draft: None,
        review_open: false,
        recovery: RecoveryDrafts::default(),
        diagnostics: crate::diagnostics_ui::DiagnosticPreview::default(),
        recovery_phase: recovery_workflow::HelperPhase::Editing,
        recovery_return: None,
        recovery_leave: None,
        initialized: false,
        projecting: false,
    }));
    let initial = selected_editor(&state.borrow());
    let source_projection = SourceProjection::from_owner(&state.borrow());
    let rows = Rc::new(VecModel::from(Vec::<SourceRow>::new()));
    let changes = Rc::new(VecModel::from(Vec::<SourceChangeRow>::new()));
    let window = AppWindow::new().map_err(HostError::Platform)?;
    source_projection.apply(&window, &rows, &changes, &state);
    if !initial.apply(&window, &state) {
        return Err(HostError::Drafts);
    }
    recovery_workflow::initialize(&window, &state);
    policy_workflow::initialize(&window, &state);
    write(
        &window,
        &state,
        FeedbackSlot::EditorFeedback,
        None,
        "".into(),
    );
    window.set_editor_open(false);
    window.set_editor_adding(false);
    window.set_power_mode(false);
    window.set_current_page(Page::Status);
    let initial_preferences = state.borrow().preferences.clone();
    if !reset_interval(&window, &initial_preferences, &state) {
        return Err(HostError::Drafts);
    }
    project_preferences(&window, &initial_preferences, &state);
    shell_workflow::initialize(&window, &state);
    diagnostic_workflow::project(&window, &state);
    status_workflow::project(&window, &state);
    install_callbacks(&window, &state, &rows, &changes);
    if state.borrow().raw_failed() {
        return Err(HostError::Drafts);
    }
    state.borrow_mut().initialized = true;
    window.set_host_ready(true);
    Ok(SlintHost {
        window,
        state,
        _rows: rows,
        _changes: changes,
    })
}

/// Construct the Windows observer host around the same checked fixture owner.
///
/// Native observers remain separate from service/clock authority. This fixture
/// does not establish an unelevated launch token; effective-token and activation
/// qualification remain with the native Windows owner.
/// The tray worker starts only after accepted opt-in; notification identity is
/// observed from the current package and content is an explicit sample example.
/// Ordinary `create()` remains free of workers, timers and native ports.
///
/// # Errors
/// Returns checked fixture, explicit backend selection, toolkit or native-host
/// installation failure. No observer is installed after a failed selection.
#[cfg(windows)]
pub fn create_native() -> Result<SlintHost, HostError> {
    slint::BackendSelector::new()
        .backend_name("winit".into())
        .renderer_name("software".into())
        .select()
        .map_err(HostError::Platform)?;
    let host = create()?;
    native_input::install(host.window.window());
    native_workflow::install(&host.window, &host.state).map_err(HostError::Native)?;
    Ok(host)
}

/// Run the Windows fixture observer owner until its acknowledged UI-only exit.
///
/// # Errors
/// Returns fixture/platform errors without claiming successful native cleanup.
#[cfg(windows)]
pub fn run_native() -> Result<(), HostError> {
    create_native()?.run().map_err(HostError::Platform)
}

/// Construct and run the checked fixture with its complete callback owner.
///
/// # Errors
/// Returns checked fixture, toolkit creation or event-loop failure.
pub fn run() -> Result<(), HostError> {
    create()?.run().map_err(HostError::Platform)
}

/// Install fixture requests; weak captures prevent component ownership cycles.
fn install_callbacks(
    window: &AppWindow,
    state: &Rc<RefCell<BindingState>>,
    rows: &Rc<VecModel<SourceRow>>,
    changes: &Rc<VecModel<SourceChangeRow>>,
) {
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_navigate(move |page| navigate(&weak, &owner, page));
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_mode_requested(move |power| change_mode(&weak, &owner, power));
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window
        .on_interval_save_requested(move |choice, raw| save_interval(&weak, &owner, choice, &raw));
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_interval_cancel_requested(move || cancel_interval(&weak, &owner));
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_notifications_requested(move |value| {
        change_preference(&weak, &owner, PreferenceRequest::Notifications(value))
    });
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_tray_requested(move |value| {
        change_preference(&weak, &owner, PreferenceRequest::Tray(value))
    });
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_sign_in_requested(move |value| {
        change_preference(&weak, &owner, PreferenceRequest::SignIn(value))
    });
    source_workflow::install_callbacks(window, state, rows, changes);
    raw_workflow::install_callbacks(window, state);
    policy_workflow::install_callbacks(window, state);
    inspection_workflow::install_callbacks(window, state, rows, changes);
    recovery_workflow::install_callbacks(window, state);
    shell_workflow::install_callbacks(window, state);
    diagnostic_workflow::install_callbacks(window, state);
    status_workflow::install_callbacks(window, state);
}

/// Closed local preference request; no variant carries an OS command or state.
#[derive(Clone, Copy)]
enum PreferenceRequest {
    /// Change only the notification preference, without delivery evidence.
    Notifications(bool),
    /// Change the tray preference, clearing dependent sign-in when disabling it.
    Tray(bool),
    /// Change sign-in after checking the accepted tray prerequisite.
    SignIn(bool),
}

/// Reproject accepted preferences without overwriting raw interval drafts.
fn project_preferences(
    window: &AppWindow,
    preferences: &UiPreferences,
    state: &RefCell<BindingState>,
) {
    let label = match preferences.interval() {
        CheckInterval::Automatic => "Automatic".to_owned(),
        CheckInterval::TwoMinutes => "Every 2 minutes".to_owned(),
        CheckInterval::FiveMinutes => "Every 5 minutes".to_owned(),
        CheckInterval::FifteenMinutes => "Every 15 minutes".to_owned(),
        CheckInterval::Custom(minutes) if minutes.get() == 1 => {
            "Every 1 minute (custom)".to_owned()
        }
        CheckInterval::Custom(minutes) => format!("Every {minutes} minutes (custom)"),
    };
    let Ok(request) = UiText::interval(preferences.interval()) else {
        catalog_workflow::invariant_failed(window, state);
        return;
    };
    write(
        window,
        state,
        FeedbackSlot::AcceptedIntervalLabel,
        Some(request),
        label.into(),
    );
    window.set_notifications_enabled(preferences.notifications_enabled());
    window.set_tray_enabled(preferences.tray_enabled());
    window.set_sign_in_enabled(preferences.sign_in_enabled());
}

/// Replace raw drafts with accepted values only at initialization, Save or Cancel.
fn reset_interval(
    window: &AppWindow,
    preferences: &UiPreferences,
    state: &RefCell<BindingState>,
) -> bool {
    let choice = match preferences.interval() {
        CheckInterval::Automatic => 0,
        CheckInterval::TwoMinutes => 1,
        CheckInterval::FiveMinutes => 2,
        CheckInterval::FifteenMinutes => 3,
        CheckInterval::Custom(_) => 4,
    };
    let custom = preferences.last_custom_minutes().to_string();
    raw_workflow::reset(
        window,
        state,
        ResetInput {
            interval: Some(IntervalFields {
                choice,
                custom_minutes: &custom,
            }),
            ..ResetInput::default()
        },
    )
}

/// Validate the signed selection and custom raw bytes before atomic local Save.
fn save_interval(
    weak: &slint::Weak<AppWindow>,
    state: &RefCell<BindingState>,
    choice: i32,
    raw: &str,
) {
    let Some(window) = weak.upgrade() else {
        return;
    };
    let Some(_projection) = ProjectionGuard::enter(state) else {
        return;
    };
    if !preference_context_reached(state, SettingsSection::Overview) {
        return;
    }
    if !raw_workflow::interval_matches(&state.borrow(), choice, raw) {
        raw_workflow::reject_stale(&window, state, FeedbackSlot::IntervalError);
        return;
    }
    let (preferences, result) = {
        let mut owner = state.borrow_mut();
        if !owner.initialized {
            return;
        }
        let result = match choice {
            0..=3 => {
                let interval = match choice {
                    0 => CheckInterval::Automatic,
                    1 => CheckInterval::TwoMinutes,
                    2 => CheckInterval::FiveMinutes,
                    _ => CheckInterval::FifteenMinutes,
                };
                owner.preferences.set_interval(interval);
                Ok(())
            }
            4 => owner
                .preferences
                .set_custom_minutes(raw)
                .map(|_| ())
                .map_err(|error| (UiText::preference_error(error), error.text())),
            _ => Err((
                UiText::fixed(MessageId::ERROR_INTERVAL_CHOICE),
                "Choose Automatic, a listed preset, or Custom.",
            )),
        };
        (owner.preferences.clone(), result)
    };
    project_preferences(&window, &preferences, state);
    match result {
        Ok(()) => {
            if !reset_interval(&window, &preferences, state) {
                return;
            }
            write_many(
                &window,
                state,
                &[
                    (FeedbackSlot::IntervalError, None, "".into()),
                    (
                        FeedbackSlot::IntervalFeedback,
                        Some(UiText::fixed(MessageId::FEEDBACK_PREFERENCE_UPDATED)),
                        "Updated for this preview session only.".into(),
                    ),
                ],
            );
        }
        Err((message, fixture)) => {
            raw_workflow::project(&window, state);
            write_many(
                &window,
                state,
                &[
                    (FeedbackSlot::IntervalError, Some(message), fixture.into()),
                    (FeedbackSlot::IntervalFeedback, None, "".into()),
                ],
            );
        }
    }
}

/// Discard interval drafts without touching source edits or other preferences.
fn cancel_interval(weak: &slint::Weak<AppWindow>, state: &RefCell<BindingState>) {
    let Some(window) = weak.upgrade() else {
        return;
    };
    let Some(_projection) = ProjectionGuard::enter(state) else {
        return;
    };
    if !preference_context_reached(state, SettingsSection::Overview) {
        return;
    }
    let preferences = {
        let owner = state.borrow();
        if !owner.initialized {
            return;
        }
        owner.preferences.clone()
    };
    if !reset_interval(&window, &preferences, state) {
        return;
    }
    project_preferences(&window, &preferences, state);
    write_many(
        &window,
        state,
        &[
            (FeedbackSlot::IntervalError, None, "".into()),
            (
                FeedbackSlot::IntervalFeedback,
                Some(UiText::fixed(MessageId::FEEDBACK_INTERVAL_CANCELLED)),
                "Interval draft discarded.".into(),
            ),
        ],
    );
}

/// Apply a typed local request using private accepted state, never view flags.
fn change_preference(
    weak: &slint::Weak<AppWindow>,
    state: &RefCell<BindingState>,
    request: PreferenceRequest,
) -> bool {
    let Some(window) = weak.upgrade() else {
        return preference_for(&state.borrow().preferences, request);
    };
    let Some(_projection) = ProjectionGuard::enter(state) else {
        return preference_for(&state.borrow().preferences, request);
    };
    let section = match request {
        PreferenceRequest::Notifications(_) => SettingsSection::Notifications,
        PreferenceRequest::Tray(_) | PreferenceRequest::SignIn(_) => {
            SettingsSection::PersonalPreferences
        }
    };
    if !preference_context_reached(state, section) {
        return preference_for(&state.borrow().preferences, request);
    }
    let (preferences, result) = {
        let mut owner = state.borrow_mut();
        if !owner.initialized {
            return preference_for(&owner.preferences, request);
        }
        let result = match request {
            PreferenceRequest::Notifications(value) => {
                owner.preferences.set_notifications_enabled(value);
                Ok(())
            }
            PreferenceRequest::Tray(value) => {
                #[cfg(windows)]
                let previous = owner.preferences.tray_enabled();
                owner.preferences.set_tray_enabled(value);
                #[cfg(windows)]
                if let Some(native) = owner.native.as_mut() {
                    native.accept_tray_preference(previous, value);
                }
                Ok(())
            }
            PreferenceRequest::SignIn(value) => owner.preferences.set_sign_in_enabled(value),
        };
        (owner.preferences.clone(), result)
    };
    // This is message context only; accepted preferences retain one Rust owner.
    let feedback_section = match request {
        PreferenceRequest::Notifications(_) => 2,
        PreferenceRequest::Tray(_) | PreferenceRequest::SignIn(_) => 4,
    };
    window.set_preference_feedback_section(feedback_section);
    project_preferences(&window, &preferences, state);
    match result {
        Ok(()) => {
            let message = match request {
                PreferenceRequest::Notifications(_) => {
                    "Notification preference updated for this preview session only."
                }
                PreferenceRequest::Tray(_) => {
                    "Tray preference updated for this preview session only."
                }
                PreferenceRequest::SignIn(_) => {
                    "Sign-in preference updated for this preview session only."
                }
            };
            let id = match request {
                PreferenceRequest::Notifications(_) => {
                    MessageId::RUST_NOTIFICATION_PREFERENCE_UPDATED
                }
                PreferenceRequest::Tray(_) => MessageId::RUST_TRAY_PREFERENCE_UPDATED,
                PreferenceRequest::SignIn(_) => MessageId::RUST_SIGN_IN_PREFERENCE_UPDATED,
            };
            write_many(
                &window,
                state,
                &[
                    (FeedbackSlot::PreferenceError, None, "".into()),
                    (
                        FeedbackSlot::PreferenceFeedback,
                        Some(UiText::fixed(id)),
                        message.into(),
                    ),
                ],
            );
        }
        Err(error) => {
            write_many(
                &window,
                state,
                &[
                    (
                        FeedbackSlot::PreferenceError,
                        Some(UiText::preference_error(error)),
                        error.text().into(),
                    ),
                    (FeedbackSlot::PreferenceFeedback, None, "".into()),
                ],
            );
        }
    }
    preference_for(&preferences, request)
}

/// Read only the accepted preference; no supplied bool or view flag is trusted.
fn preference_for(preferences: &UiPreferences, request: PreferenceRequest) -> bool {
    match request {
        PreferenceRequest::Notifications(_) => preferences.notifications_enabled(),
        PreferenceRequest::Tray(_) => preferences.tray_enabled(),
        PreferenceRequest::SignIn(_) => preferences.sign_in_enabled(),
    }
}

/// Map the closed generated enum to the checked session's navigation type.
const fn session_page(page: Page) -> SessionPage {
    match page {
        Page::Status => SessionPage::Status,
        Page::Sources => SessionPage::Sources,
        Page::Statistics => SessionPage::Statistics,
        Page::Diagnostics => SessionPage::Diagnostics,
        Page::Settings => SessionPage::Settings,
        Page::About => SessionPage::About,
        Page::Recovery => SessionPage::Recovery,
    }
}

/// Publish the accepted mode/page pair without touching editor drafts or guards.
fn project_navigation(window: &AppWindow, mode: Mode, page: SessionPage) {
    let page = match page {
        SessionPage::Status => Page::Status,
        SessionPage::Sources => Page::Sources,
        SessionPage::Statistics => Page::Statistics,
        SessionPage::Diagnostics => Page::Diagnostics,
        SessionPage::Settings => Page::Settings,
        SessionPage::About => Page::About,
        SessionPage::Recovery => Page::Recovery,
    };
    window.set_power_mode(mode == Mode::Power);
    window.set_current_page(page);
}

/// Dispatch page requests through the recovery draft/leave transition guard.
fn navigate(weak: &slint::Weak<AppWindow>, state: &RefCell<BindingState>, page: Page) {
    recovery_workflow::navigate(weak, state, page);
}

/// Change mode while preserving raw source/helper drafts and checked leave intent.
fn change_mode(weak: &slint::Weak<AppWindow>, state: &RefCell<BindingState>, power: bool) {
    recovery_workflow::change_mode(weak, state, power);
}

/// Admit a local preference workflow only at its current trusted Settings section.
///
/// Complexity mode grants no privilege. Each shared section is usable in either
/// mode; projected page/section values cannot erase drafts or alter preferences.
/// This read-only check runs under the caller's root projection/modal guard.
fn preference_context_reached(state: &RefCell<BindingState>, section: SettingsSection) -> bool {
    let owner = state.borrow();
    owner.navigation.page() == SessionPage::Settings && owner.shell.settings_section() == section
}

/// Portable current-page/global-modal counterexamples for native receiving guards.
#[cfg(test)]
mod native_context_tests;

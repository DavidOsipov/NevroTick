//! Pure fixture-only tray menu presentation and guarded navigation intentions.
//!
//! This module creates no native icon, observer, window or effect. It consumes
//! closed presentation context from the owning UI state, not a backend wire
//! record or permission. A saved tray preference and opening this menu prove
//! neither icon availability nor current synchronization. The reached fixture
//! has no measurement adapter, so a check is always rejected with a reason.
#![forbid(unsafe_code)]

use crate::shell::{ShellRequest, TrayMenuAction};

/// Context supplied by the checked UI presentation owner, never a user selector.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrayCheckContext {
    /// Use the familiar request label; it still grants no clock-write authority.
    Synchronization,
    /// Observe-only, blocked, uncertain or Windows-managed context uses this label.
    CheckSourcesOnly,
    /// Currently observed stopped services offer management, not a fresh check.
    ServicesStopped,
}

/// Current private interaction ownership, separate from view visibility flags.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrayInteraction {
    /// No dialog or pending exit currently owns this receiving UI transition.
    Free,
    /// Another dialog must be resolved before any menu intention is accepted.
    DialogOwned,
    /// An already-requested UI exit must not admit another action or navigation.
    ExitRequested,
}

/// Fixed code-owned reason why the fixture cannot request a fresh check.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrayCheckBlock {
    /// The native measurement request adapter is not connected.
    NativeAdapterUnavailable,
    /// The current service or measurement-request context is unconfirmed.
    StateUnconfirmed,
    /// An owned operation is pending; another request must not bypass it.
    OperationPending,
    /// The request owner reports a current limit; no duration is invented here.
    RateLimited,
}

impl TrayCheckBlock {
    /// Stable proposed catalog ID; text never selects an action or duration.
    #[must_use]
    pub const fn catalog_id(self) -> &'static str {
        match self {
            Self::NativeAdapterUnavailable => "ui.tray.block.adapter-unavailable",
            Self::StateUnconfirmed => "ui.tray.block.state-unconfirmed",
            Self::OperationPending => "ui.tray.block.operation-pending",
            Self::RateLimited => "ui.tray.block.rate-limited",
        }
    }

    /// Fixed English preview reason; this does not report request submission.
    #[must_use]
    pub const fn english(self) -> &'static str {
        match self {
            Self::NativeAdapterUnavailable => {
                "Fresh checks are unavailable in this preview. No request was submitted."
            }
            Self::StateUnconfirmed => {
                "Current service state is unconfirmed. Open Diagnostics before requesting a check."
            }
            Self::OperationPending => {
                "A current operation is pending. Opening this menu does not request another check."
            }
            Self::RateLimited => {
                "The current request limit prevents another check. Opening this menu does not reset the limit."
            }
        }
    }
}

/// Stable menu identity; contextual wording never changes the receiver's ID.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrayItemId {
    /// Open the ordinary shared Status page.
    Open,
    /// Request context-specific check presentation or open service management.
    ContextualCheck,
    /// Open Diagnostics through the explicit Power transition.
    Diagnostics,
    /// Open the shared Settings overview.
    Settings,
    /// Open the shared About page.
    About,
    /// Begin the existing UI-only quit confirmation, never confirm it directly.
    QuitUi,
}

impl TrayItemId {
    /// Decode only the six maintained signed callback identities.
    ///
    /// # Errors
    /// Reject unknown identities without an effect, mutation or allocation.
    pub const fn parse(raw: i32) -> Result<Self, TrayRejection> {
        match raw {
            0 => Ok(Self::Open),
            1 => Ok(Self::ContextualCheck),
            2 => Ok(Self::Diagnostics),
            3 => Ok(Self::Settings),
            4 => Ok(Self::About),
            5 => Ok(Self::QuitUi),
            _ => Err(TrayRejection::InvalidItem),
        }
    }

    /// Return the exact signed callback identity without enum casting.
    #[must_use]
    pub const fn view_id(self) -> i32 {
        match self {
            Self::Open => 0,
            Self::ContextualCheck => 1,
            Self::Diagnostics => 2,
            Self::Settings => 3,
            Self::About => 4,
            Self::QuitUi => 5,
        }
    }
}

/// Code-owned label meaning, separate from stable callback/menu identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrayLabel {
    /// Open the application window without requesting measurements.
    Open,
    /// Request fresh measurements under an existing synchronization configuration.
    SyncNow,
    /// Request fresh measurements without implying clock-write permission.
    CheckSources,
    /// Open the existing shared service settings without submitting a command.
    ManageBackgroundService,
    /// Explicitly enter Power user for diagnostics.
    Diagnostics,
    /// Open shared Settings.
    Settings,
    /// Open shared About.
    About,
    /// Open the warning before any UI-only exit confirmation.
    QuitUi,
}

impl TrayLabel {
    /// Stable proposed catalog ID; no catalog string chooses a menu identity.
    #[must_use]
    pub const fn catalog_id(self) -> &'static str {
        match self {
            Self::Open => "ui.tray.menu.open",
            Self::SyncNow => "ui.fixture.sync-now",
            Self::CheckSources => "ui.status.check.sources",
            Self::ManageBackgroundService => "ui.shell.manage-service",
            Self::Diagnostics => "ui.tray.menu.diagnostics",
            Self::Settings => "ui.fixture.settings-tab",
            Self::About => "ui.fixture.about-heading",
            Self::QuitUi => "ui.shell.quit-ui",
        }
    }

    /// Fixed English fixture text; final catalog admission remains separate.
    #[must_use]
    pub const fn english(self) -> &'static str {
        match self {
            Self::Open => "Open NevroTick",
            Self::SyncNow => "Sync now",
            Self::CheckSources => "Check sources",
            Self::ManageBackgroundService => "Manage background service",
            Self::Diagnostics => "Diagnostics (Power user)",
            Self::Settings => "Settings",
            Self::About => "About NevroTick",
            Self::QuitUi => "Quit UI",
        }
    }
}

/// Bounded receiving rejection; it retains no raw identity or external strings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrayRejection {
    /// A signed callback identity did not denote one of the six maintained items.
    InvalidItem,
    /// A different private dialog owns the interaction.
    DialogOwned,
    /// The private shell owner has already requested UI exit.
    ExitRequested,
    /// The current contextual measurement request is unavailable.
    CheckUnavailable(TrayCheckBlock),
}

impl TrayRejection {
    /// Stable proposed catalog ID; a rejected raw identity is never retained.
    #[must_use]
    pub const fn catalog_id(self) -> &'static str {
        match self {
            Self::InvalidItem => "ui.tray.block.invalid-item",
            Self::DialogOwned => "ui.tray.block.dialog-owned",
            Self::ExitRequested => "ui.tray.block.exit-requested",
            Self::CheckUnavailable(reason) => reason.catalog_id(),
        }
    }

    /// Fixed English preview feedback without raw identity or service details.
    #[must_use]
    pub const fn english(self) -> &'static str {
        match self {
            Self::InvalidItem => "Choose a listed tray menu action. No request was submitted.",
            Self::DialogOwned => "Finish or cancel the open dialog before using the tray menu.",
            Self::ExitRequested => {
                "App close is already requested. No further menu action was accepted."
            }
            Self::CheckUnavailable(reason) => reason.english(),
        }
    }
}

/// One immutable menu projection with code-owned identity and label semantics.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TrayEntry {
    /// Identity consumed through the checked receiver rather than row position.
    id: TrayItemId,
    /// Label meaning; translations cannot choose the intention.
    label: TrayLabel,
    /// Current reason why this row cannot accept an intention.
    rejection: Option<TrayRejection>,
}

impl TrayEntry {
    /// Return the stable identity, independent of contextual display wording.
    #[must_use]
    pub const fn id(self) -> TrayItemId {
        self.id
    }

    /// Return code-owned label meaning for a future validated catalog bridge.
    #[must_use]
    pub const fn label(self) -> TrayLabel {
        self.label
    }

    /// Return current presentation availability, never native icon readiness.
    #[must_use]
    pub const fn enabled(self) -> bool {
        self.rejection.is_none()
    }

    /// Explain a disabled row without submitting a request.
    #[must_use]
    pub const fn rejection(self) -> Option<TrayRejection> {
        self.rejection
    }
}

/// Immutable, fixed-size menu preview; it owns no menu-open or native tray state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixtureTrayMenu {
    /// The private UI producer supplies this closed, currently applicable context.
    context: TrayCheckContext,
    /// No fixture context can construct a successful measurement request.
    check_block: TrayCheckBlock,
}

impl FixtureTrayMenu {
    /// Retain finite presentation context, without parsing strings or doing work.
    #[must_use]
    pub const fn new(context: TrayCheckContext, check_block: TrayCheckBlock) -> Self {
        Self {
            context,
            check_block,
        }
    }

    /// Derive exactly six rows in stable order, with no effect or allocation.
    #[must_use]
    pub fn entries(self, interaction: TrayInteraction) -> [TrayEntry; 6] {
        [
            TrayItemId::Open,
            TrayItemId::ContextualCheck,
            TrayItemId::Diagnostics,
            TrayItemId::Settings,
            TrayItemId::About,
            TrayItemId::QuitUi,
        ]
        .map(|id| self.entry(id, interaction))
    }

    /// Receive a signed callback identity and the current private root guard.
    ///
    /// No saved projection, native readiness, checked menu-open flag or label is
    /// accepted as authority. The owning root must still apply its navigation
    /// and quit guards when consuming the returned existing shell request. This module cannot confirm
    /// quit or duplicate the shell controller's navigation/effect decisions.
    ///
    /// # Errors
    /// Reject unknown IDs, private interaction ownership or unavailable checks.
    pub fn request(
        self,
        raw: i32,
        interaction: TrayInteraction,
    ) -> Result<ShellRequest, TrayRejection> {
        let id = TrayItemId::parse(raw)?;
        if let Some(rejection) = self.entry(id, interaction).rejection() {
            return Err(rejection);
        }
        let action = match id {
            TrayItemId::Open => TrayMenuAction::OpenNevroTick,
            TrayItemId::ContextualCheck => match self.context {
                TrayCheckContext::ServicesStopped => TrayMenuAction::TimeManagement,
                TrayCheckContext::Synchronization | TrayCheckContext::CheckSourcesOnly => {
                    TrayMenuAction::RequestMeasurements
                }
            },
            TrayItemId::Diagnostics => TrayMenuAction::Diagnostics,
            TrayItemId::Settings => TrayMenuAction::Settings,
            TrayItemId::About => TrayMenuAction::About,
            TrayItemId::QuitUi => TrayMenuAction::QuitUi,
        };
        Ok(ShellRequest::Menu(action))
    }

    /// Build one row and recheck current private ownership before any intent.
    fn entry(self, id: TrayItemId, interaction: TrayInteraction) -> TrayEntry {
        let label = match id {
            TrayItemId::Open => TrayLabel::Open,
            TrayItemId::ContextualCheck => match self.context {
                TrayCheckContext::Synchronization => TrayLabel::SyncNow,
                TrayCheckContext::CheckSourcesOnly => TrayLabel::CheckSources,
                TrayCheckContext::ServicesStopped => TrayLabel::ManageBackgroundService,
            },
            TrayItemId::Diagnostics => TrayLabel::Diagnostics,
            TrayItemId::Settings => TrayLabel::Settings,
            TrayItemId::About => TrayLabel::About,
            TrayItemId::QuitUi => TrayLabel::QuitUi,
        };
        let rejection = match interaction {
            TrayInteraction::DialogOwned => Some(TrayRejection::DialogOwned),
            TrayInteraction::ExitRequested => Some(TrayRejection::ExitRequested),
            TrayInteraction::Free
                if id == TrayItemId::ContextualCheck
                    && self.context != TrayCheckContext::ServicesStopped =>
            {
                Some(TrayRejection::CheckUnavailable(self.check_block))
            }
            TrayInteraction::Free => None,
        };
        TrayEntry {
            id,
            label,
            rejection,
        }
    }
}

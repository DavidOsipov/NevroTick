//! Portable fixture presentation state for the Slint adapter.
//!
//! [`NavigationState`] owns only mode/page transitions. Legacy [`UiSession`]
//! composes it with the checked [`PreviewModel`] source owner. The new source-draft
//! host can compose navigation with its own single checked source owner instead.
//! Raw editor drafts remain in the view: navigation never replaces them, while
//! successful editor actions return an explicit [`EditorReset`] projection. Rejected requests
//! preserve the entire session and retain no rejected input. This module has no
//! transport, persistence, service, elevation, clock or toolkit effects.
#![forbid(unsafe_code)]

use std::fmt;

use crate::model::{ModelError, PreviewModel, SourcePreview};

/// Fixed feedback after an accepted in-memory preview update.
const UPDATED_MESSAGE: &str =
    "Preview updated in memory only. No connection or policy change was made.";
/// Fixed feedback when the view discards its unaccepted editor text.
const CANCELLED_MESSAGE: &str = "Unaccepted editor changes discarded.";

/// Presentation complexity; neither mode grants backend authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    /// Status, Settings and About are reachable.
    Simple,
    /// Also expose Sources, Statistics, Diagnostics and Date recovery details.
    Power,
}

/// A closed navigation target without a production workflow or service state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Page {
    /// Fixed sample status and unavailable synchronization action.
    Status,
    /// Checked source inventory and transient preview editor.
    Sources,
    /// Fixed sample statistics without calculations or acquisition.
    Statistics,
    /// Fixed sample diagnostics without service or host inspection.
    Diagnostics,
    /// Read-only sample settings and unavailable machine policy action.
    Settings,
    /// Fixture application information.
    About,
    /// Signed-date explanation and explicit unauthenticated helper preview.
    Recovery,
}

impl Page {
    /// Return whether this page requires Power mode; this performs no effects.
    #[must_use]
    pub const fn is_power_only(self) -> bool {
        matches!(
            self,
            Self::Sources | Self::Statistics | Self::Diagnostics | Self::Recovery
        )
    }
}

/// Fixed, redacted request rejection; no variant retains raw editor data.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiError {
    /// Simple mode cannot navigate directly to a Power-only page.
    PageUnavailable,
    /// Existing checked source construction or selection rejected the request.
    Model(ModelError),
}

impl UiError {
    /// Borrow fixed binding feedback without formatting or copying raw input.
    #[must_use]
    pub const fn text(self) -> &'static str {
        match self {
            Self::PageUnavailable => "This sample page is available in Power user mode.",
            Self::Model(ModelError::InvalidLabel) => "The sample display name is invalid.",
            Self::Model(ModelError::InvalidHost) => "The sample server identity is invalid.",
            Self::Model(ModelError::InvalidPort) => "The sample NTS-KE port is invalid.",
            Self::Model(ModelError::InvalidSelection) => "Select one of the sample sources.",
        }
    }
}

impl fmt::Display for UiError {
    /// Emit only fixed binding feedback; rejected bytes are never formatted.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.text())
    }
}

impl std::error::Error for UiError {
    /// Expose only the bounded model cause, which also contains no rejected data.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::PageUnavailable => None,
            Self::Model(error) => Some(error),
        }
    }
}

/// Source-independent mode/page state for the Slint source-draft host.
///
/// The host may compose this with one source owner without constructing the old
/// fixed source fixture. Navigation retains no raw editor text, service state or
/// authority. A host-owned modal/editor transition guard remains mandatory.
/// Private fields prevent bypassing the Simple-mode reachability invariant.
/// ```compile_fail,E0451
/// use nevrotick_ui::presentation::{Mode, NavigationState, Page};
/// let navigation = NavigationState {
///     mode: Mode::Simple,
///     page: Page::Sources,
///     power_page: Page::Status,
/// };
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigationState {
    /// Current presentation mode; supplies no command authority.
    mode: Mode,
    /// Current page, always reachable in the current mode.
    page: Page,
    /// Most recent Power page, retained while Simple shows its fallback Status.
    power_page: Page,
}

impl NavigationState {
    /// Start in Simple mode on Status without constructing or changing sources.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            mode: Mode::Simple,
            page: Page::Status,
            power_page: Page::Status,
        }
    }

    /// Return the current mode without mutating navigation or view drafts.
    #[must_use]
    pub const fn mode(&self) -> Mode {
        self.mode
    }

    /// Return a page guaranteed reachable under the current presentation mode.
    #[must_use]
    pub const fn page(&self) -> Page {
        self.page
    }

    /// Change mode, remember the Power page and preserve shared-page context.
    ///
    /// A repeated mode is a no-op. Entering Simple from a Power-only page shows
    /// Status. Returning to Power restores its remembered page unless Settings
    /// or About is currently open. The adapter owns active-dialog guards.
    pub fn set_mode(&mut self, mode: Mode) {
        if mode == self.mode {
            return;
        }
        match mode {
            Mode::Simple => {
                self.power_page = self.page;
                if self.page.is_power_only() {
                    self.page = Page::Status;
                }
            }
            Mode::Power => {
                if !matches!(self.page, Page::Settings | Page::About) {
                    self.page = self.power_page;
                }
            }
        }
        self.mode = mode;
    }

    /// Navigate only to a page reachable under the current checked mode.
    ///
    /// # Errors
    /// Power-only targets in Simple reject atomically. No method edits source
    /// values or raw drafts, even on a successful navigation request.
    pub fn navigate(&mut self, page: Page) -> Result<(), UiError> {
        if self.mode == Mode::Simple && page.is_power_only() {
            return Err(UiError::PageUnavailable);
        }
        self.page = page;
        if self.mode == Mode::Power {
            self.power_page = page;
        }
        Ok(())
    }
}

impl Default for NavigationState {
    /// Supply the same source-independent Simple/Status initialization as new.
    fn default() -> Self {
        Self::new()
    }
}

/// An explicit instruction to reset view-owned drafts to the accepted source.
///
/// The adapter projects these bounded, checked values while this borrow is
/// valid, then releases its session borrow before changing toolkit properties.
/// Creating this value neither edits view properties nor performs backend work.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use]
pub struct EditorReset<'a> {
    /// Checked source currently selected by the session.
    source: &'a SourcePreview,
    /// Stable index in the fixed, unreordered fixture inventory.
    selected_index: usize,
    /// Code-owned feedback for this editor action.
    message: &'static str,
}

impl<'a> EditorReset<'a> {
    /// Borrow accepted fields for replacing the view's raw draft text.
    #[must_use]
    pub const fn source(&self) -> &'a SourcePreview {
        self.source
    }

    /// Return the accepted source's index in the fixed fixture inventory.
    #[must_use]
    pub const fn selected_index(&self) -> usize {
        self.selected_index
    }

    /// Borrow fixed editor feedback without allocating or exposing raw input.
    #[must_use]
    pub const fn message(&self) -> &'static str {
        self.message
    }
}

/// Checked fixture state and navigation, with no copy of view-owned raw drafts.
///
/// Each new session starts in Simple mode on Status and owns exactly the model's
/// three sample sources. All mutations affect only this in-memory instance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiSession {
    /// Sole owner of accepted source fields and their current selection.
    model: PreviewModel,
    /// Shared source-independent navigation state; no second source owner.
    navigation: NavigationState,
}

impl UiSession {
    /// Construct a fresh checked fixture without reading or writing external state.
    ///
    /// # Errors
    /// Returns a bounded model error if a maintained source fixture is invalid.
    pub fn fixture() -> Result<Self, UiError> {
        Ok(Self {
            model: PreviewModel::fixture().map_err(UiError::Model)?,
            navigation: NavigationState::new(),
        })
    }

    /// Return the current presentation mode without mutating session or drafts.
    #[must_use]
    pub const fn mode(&self) -> Mode {
        self.navigation.mode()
    }

    /// Return the current reachable page without mutating session or drafts.
    #[must_use]
    pub const fn page(&self) -> Page {
        self.navigation.page()
    }

    /// Borrow the checked, fixed source inventory without exposing mutation.
    #[must_use]
    pub fn sources(&self) -> &[SourcePreview] {
        self.model.sources()
    }

    /// Return the checked selection in the fixed, unreordered source inventory.
    #[must_use]
    pub const fn selected_index(&self) -> usize {
        self.model.selected_index()
    }

    /// Borrow the selected accepted source, independently of view-owned drafts.
    #[must_use]
    pub fn selected(&self) -> &SourcePreview {
        self.model.selected()
    }

    /// Change only presentation mode/page, preserving accepted sources and drafts.
    ///
    /// Entering Simple remembers the current Power page and falls back to Status
    /// when necessary. Returning to Power restores that page unless Simple is
    /// showing Settings or About. An unchanged mode is a no-op. This method does
    /// not own a modal editor; an adapter with such a dialog must retain its
    /// dialog transition guard before dispatching a mode change.
    pub fn set_mode(&mut self, mode: Mode) {
        self.navigation.set_mode(mode);
    }

    /// Navigate to an allowed typed target without resetting editor drafts.
    ///
    /// # Errors
    /// Returns `PageUnavailable` for a Power-only target in Simple mode, leaving
    /// the whole session unchanged. Successful navigation changes no sources.
    pub fn navigate(&mut self, page: Page) -> Result<(), UiError> {
        self.navigation.navigate(page)
    }

    /// Select a signed callback row through the existing checked model path.
    ///
    /// Success returns the accepted projection and a draft-reset instruction;
    /// selection itself does not navigate or change accepted fields. No input
    /// index is cast, wrapped or saturated into the fixed inventory.
    ///
    /// # Errors
    /// Negative or out-of-range rows return a bounded model error and preserve
    /// all session state; the adapter must then retain its current raw drafts.
    pub fn select_source(&mut self, index: i32) -> Result<EditorReset<'_>, UiError> {
        let index =
            usize::try_from(index).map_err(|_| UiError::Model(ModelError::InvalidSelection))?;
        self.model.select(index).map_err(UiError::Model)?;
        Ok(self.editor_reset(""))
    }

    /// Borrow raw callback fields into the sole checked, atomic model update.
    ///
    /// Success changes only the selected accepted source and returns its draft
    /// reset projection. Inputs are never copied before the model's admission;
    /// no rejected bytes enter feedback or session state.
    ///
    /// # Errors
    /// Invalid or oversized fields preserve the entire session. The adapter
    /// must display the fixed error while retaining all view-owned raw drafts.
    pub fn update_preview(
        &mut self,
        label: &str,
        host: &str,
        port: &str,
    ) -> Result<EditorReset<'_>, UiError> {
        self.model
            .update_selected(label, host, port)
            .map_err(UiError::Model)?;
        Ok(self.editor_reset(UPDATED_MESSAGE))
    }

    /// Return accepted fields for discarding view-owned drafts without mutation.
    ///
    /// Cancel has no failure or backend effect, changes no selection or source,
    /// and leaves navigation/mode intact. The adapter performs the text reset.
    pub fn cancel(&self) -> EditorReset<'_> {
        self.editor_reset(CANCELLED_MESSAGE)
    }

    /// Project the current checked source and fixed action feedback by borrowing.
    fn editor_reset(&self, message: &'static str) -> EditorReset<'_> {
        EditorReset {
            source: self.model.selected(),
            selected_index: self.model.selected_index(),
            message,
        }
    }
}

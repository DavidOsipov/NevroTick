//! Local diagnostic navigation and illustrative collection preferences.
//!
//! This is a presentation owner, not a logging, export or correction adapter.
//! Closed choices retain no endpoints, usernames, events, deadlines or authority.
#![forbid(unsafe_code)]

/// Reached read-only diagnostic surface; none authorizes an external effect.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DiagnosticView {
    /// Current unavailable observations, event stream and contextual help.
    #[default]
    Overview,
    /// Bounded maintained report, with no live private data.
    Report,
    /// Local scope/duration illustration, with collection unavailable.
    TemporaryCollection,
    /// No received correction proposal; authorization stays unavailable.
    CorrectionReview,
}

/// Component scope requested for a local collection preview only.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CollectionScope {
    /// Ordinary user UI collection; no administrator operation is implied.
    #[default]
    Ui,
    /// Service collection would need a separately authorized backend operation.
    Service,
    /// Each component must report its actual outcome independently.
    UiAndService,
}

/// Invalid closed choice; no raw input is retained or echoed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DiagnosticChoiceError;

/// Sole local diagnostic owner, independent of service and notification facts.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DiagnosticPreview {
    /// Current checked subview, initially the ordinary overview.
    view: DiagnosticView,
    /// Accepted illustrative component selection, never effective collection.
    scope: CollectionScope,
    /// Accepted illustrative duration choice, restricted to the three examples.
    duration_choice: u8,
}

impl DiagnosticPreview {
    /// Open the existing read-only correction surface without an effect or raw ID.
    pub fn open_correction_review(&mut self) {
        self.view = DiagnosticView::CorrectionReview;
    }

    /// Return the current read-only subview.
    #[must_use]
    pub const fn view(&self) -> DiagnosticView {
        self.view
    }

    /// Return the local illustrative selection, not an effective logging scope.
    #[must_use]
    pub const fn scope(&self) -> CollectionScope {
        self.scope
    }

    /// Return the closed example index; it is not a production logging limit.
    #[must_use]
    pub const fn duration_choice(&self) -> u8 {
        self.duration_choice
    }

    /// Select one code-owned subview atomically without invoking an adapter.
    ///
    /// # Errors
    /// Rejects every value outside 0..=3 and preserves all local state.
    pub fn select_view(&mut self, choice: i32) -> Result<(), DiagnosticChoiceError> {
        let view = match choice {
            0 => DiagnosticView::Overview,
            1 => DiagnosticView::Report,
            2 => DiagnosticView::TemporaryCollection,
            3 => DiagnosticView::CorrectionReview,
            _ => return Err(DiagnosticChoiceError),
        };
        self.view = view;
        Ok(())
    }

    /// Update a local illustration only while its own subview is active.
    ///
    /// # Errors
    /// Rejects an inactive surface or any unknown scope/duration without change.
    pub fn select_collection(
        &mut self,
        scope: i32,
        duration: i32,
    ) -> Result<(), DiagnosticChoiceError> {
        if self.view != DiagnosticView::TemporaryCollection {
            return Err(DiagnosticChoiceError);
        }
        let scope = match scope {
            0 => CollectionScope::Ui,
            1 => CollectionScope::Service,
            2 => CollectionScope::UiAndService,
            _ => return Err(DiagnosticChoiceError),
        };
        let duration_choice = u8::try_from(duration).map_err(|_| DiagnosticChoiceError)?;
        if duration_choice > 2 {
            return Err(DiagnosticChoiceError);
        }
        self.scope = scope;
        self.duration_choice = duration_choice;
        Ok(())
    }
}

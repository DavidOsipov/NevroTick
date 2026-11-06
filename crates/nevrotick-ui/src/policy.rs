//! Explicit, UI-only profile drafts; no active policy, source or clock mutation.
//!
//! F-06 and the approved v0.4 editor settle three profile choices. Numeric
//! minimum/majority, provider presets, crypto and correction limits are not
//! editable here: their production contracts remain separately qualified work.
//! The NTS fixture baseline is not a declaration of a shipped default/preset.
#![forbid(unsafe_code)]

use std::fmt;

use crate::source_drafts::Protocol;

/// Closed profile intent; no variant establishes observed authentication/authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolicyProfile {
    /// Only eligible NTS-authenticated sources may supply ordinary votes.
    NtsOnly,
    /// Deliberate unauthenticated NTP, never automatic fallback or mixed voting.
    PlainNtp,
    /// No product clock writes; this choice does not turn Windows Time on.
    ObserveOnly,
}

impl PolicyProfile {
    /// Admit only the code-owned three-choice signed presentation representation.
    ///
    /// # Errors
    /// Unknown choices reject without selecting a fallback profile.
    pub const fn from_view(raw: i32) -> Result<Self, PolicyError> {
        match raw {
            0 => Ok(Self::NtsOnly),
            1 => Ok(Self::PlainNtp),
            2 => Ok(Self::ObserveOnly),
            _ => Err(PolicyError::InvalidChoice),
        }
    }

    /// Return the stable view choice without serializing a backend policy.
    #[must_use]
    pub const fn view_choice(self) -> i32 {
        match self {
            Self::NtsOnly => 0,
            Self::PlainNtp => 1,
            Self::ObserveOnly => 2,
        }
    }

    /// Return an explicitly requested transport, without changing source entries.
    ///
    /// Observe-only does not settle its acquisition/trust mode. None forbids the
    /// presenter from inferring plain NTP or an authentication observation.
    #[must_use]
    pub const fn transport_requirement(self) -> Option<Protocol> {
        match self {
            Self::NtsOnly => Some(Protocol::Nts),
            Self::PlainNtp => Some(Protocol::PlainNtp),
            Self::ObserveOnly => None,
        }
    }

    /// Distinguish the profile's no-write intent, never granting writes otherwise.
    #[must_use]
    pub const fn is_observe_only(self) -> bool {
        matches!(self, Self::ObserveOnly)
    }
}

/// Local editor phase; a view flag alone cannot authorize a transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolicyPhase {
    /// Summary, possibly retaining a checked un-applied profile preview.
    Summary,
    /// One raw selector draft is in the view; it has not been admitted.
    Editing,
    /// A complete checked preview is available for comparison, never Apply.
    Review,
}

/// Fixed redacted local-profile rejection; no variant retains raw input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolicyError {
    /// Raw choice is outside the settled three-option profile set.
    InvalidChoice,
    /// This action requires another trusted editor/review phase.
    WrongPhase,
    /// No retained checked preview exists to open for review.
    NoPreview,
}

impl fmt::Display for PolicyError {
    /// Render code-owned guidance without raw choice values or policy internals.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidChoice => "Choose NTS, explicitly selected NTP, or observe only.",
            Self::WrongPhase => "Finish the current profile editing or review action first.",
            Self::NoPreview => "Choose a profile preview before reviewing it.",
        })
    }
}

impl std::error::Error for PolicyError {}

/// One immutable fixture baseline and one checked local preview, with no Apply.
///
/// Raw choices cannot construct or mutate trusted phase/profile fields:
/// ```compile_fail,E0451
/// use nevrotick_ui::policy::{PolicyDrafts, PolicyPhase, PolicyProfile};
/// let drafts = PolicyDrafts {
///     baseline: PolicyProfile::NtsOnly,
///     preview: Some(PolicyProfile::PlainNtp),
///     phase: PolicyPhase::Review,
/// };
/// ```
/// Local drafts deliberately expose no backend Apply operation:
/// ```compile_fail,E0599
/// use nevrotick_ui::policy::PolicyDrafts;
/// let mut drafts = PolicyDrafts::fixture();
/// drafts.apply();
/// ```
/// A checked legal fixture review remains available:
/// ```
/// use nevrotick_ui::policy::PolicyDrafts;
/// let mut drafts = PolicyDrafts::fixture();
/// assert_eq!(drafts.edit()?, 0);
/// drafts.review(1)?;
/// # Ok::<(), nevrotick_ui::policy::PolicyError>(())
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyDrafts {
    /// Fixture comparison value; it is never modified by a preview operation.
    baseline: PolicyProfile,
    /// Retained candidate, separate from the baseline and view-owned raw choice.
    preview: Option<PolicyProfile>,
    /// Sole authority for local editor/review transition admission.
    phase: PolicyPhase,
}

impl PolicyDrafts {
    /// Start a local fixture comparison using an already typed baseline.
    #[must_use]
    pub const fn new(baseline: PolicyProfile) -> Self {
        Self {
            baseline,
            preview: None,
            phase: PolicyPhase::Summary,
        }
    }

    /// Supply the v0.4 NTS fixture, without adopting a shipped source default.
    #[must_use]
    pub const fn fixture() -> Self {
        Self::new(PolicyProfile::NtsOnly)
    }

    /// Return the immutable fixture profile, never an observed service revision.
    #[must_use]
    pub const fn baseline(&self) -> PolicyProfile {
        self.baseline
    }

    /// Return a retained checked candidate without claiming policy activation.
    #[must_use]
    pub const fn preview(&self) -> Option<PolicyProfile> {
        self.preview
    }

    /// Return the trusted phase; view properties only project this value.
    #[must_use]
    pub const fn phase(&self) -> PolicyPhase {
        self.phase
    }

    /// Open the selector from Summary/Review, returning its accepted reset choice.
    ///
    /// # Errors
    /// Repeated Editing rejects and must preserve the view's current raw choice.
    pub fn edit(&mut self) -> Result<i32, PolicyError> {
        if self.phase == PolicyPhase::Editing {
            return Err(PolicyError::WrongPhase);
        }
        let choice = self.retained_choice();
        self.phase = PolicyPhase::Editing;
        Ok(choice)
    }

    /// Admit a raw choice before atomically retaining its review candidate.
    ///
    /// # Errors
    /// Wrong phase or unknown signed choice leaves every field unchanged.
    pub fn review(&mut self, raw: i32) -> Result<(), PolicyError> {
        if self.phase != PolicyPhase::Editing {
            return Err(PolicyError::WrongPhase);
        }
        let candidate = PolicyProfile::from_view(raw)?;
        self.preview = Some(candidate);
        self.phase = PolicyPhase::Review;
        Ok(())
    }

    /// Cancel raw selector edits, preserving any earlier submitted local preview.
    ///
    /// # Errors
    /// Only the trusted Editing phase may cancel an unsubmitted choice.
    pub fn cancel_edit(&mut self) -> Result<i32, PolicyError> {
        if self.phase != PolicyPhase::Editing {
            return Err(PolicyError::WrongPhase);
        }
        let choice = self.retained_choice();
        self.phase = PolicyPhase::Summary;
        Ok(choice)
    }

    /// Close review while retaining the candidate as an unapplied local preview.
    ///
    /// # Errors
    /// A view flag cannot substitute for the trusted Review phase.
    pub fn keep_preview(&mut self) -> Result<(), PolicyError> {
        if self.phase != PolicyPhase::Review {
            return Err(PolicyError::WrongPhase);
        }
        self.phase = PolicyPhase::Summary;
        Ok(())
    }

    /// Reopen a retained checked preview without admitting a new raw selector.
    ///
    /// # Errors
    /// Editing/Review rejects; Summary without a candidate reports `NoPreview`.
    pub fn open_review(&mut self) -> Result<(), PolicyError> {
        if self.phase != PolicyPhase::Summary {
            return Err(PolicyError::WrongPhase);
        }
        if self.preview.is_none() {
            return Err(PolicyError::NoPreview);
        }
        self.phase = PolicyPhase::Review;
        Ok(())
    }

    /// Discard only the reviewed local candidate; fixture baseline stays intact.
    ///
    /// # Errors
    /// Only trusted Review may discard; no source/service setting is changed.
    pub fn discard_preview(&mut self) -> Result<(), PolicyError> {
        if self.phase != PolicyPhase::Review {
            return Err(PolicyError::WrongPhase);
        }
        self.preview = None;
        self.phase = PolicyPhase::Summary;
        Ok(())
    }

    /// Project a checked candidate or baseline; this grants no backend authority.
    const fn retained_choice(&self) -> i32 {
        match self.preview {
            Some(profile) => profile.view_choice(),
            None => self.baseline.view_choice(),
        }
    }
}

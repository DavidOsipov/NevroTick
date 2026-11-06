//! Closed presenter facts retained independently of a selected catalog revision.
//!
//! No translated words are matched to reconstruct state. A presenter stores these
//! facts and re-renders them through one borrowed snapshot before UI publication.
#![forbid(unsafe_code)]

use std::num::NonZeroU64;

use super::{Error, MessageId, PlainText, Snapshot};
use crate::model::ModelError;
use crate::notifications_ui::NotificationBlock;
use crate::preferences::{CheckInterval, PreferenceError};
use crate::presentation::{Page, UiError};
use crate::recovery::RecoveryError;
use crate::shell::QuitServiceState;
use crate::source_drafts::{ChangeKind, SourceDraftError};

/// A closed, code-owned message request; fields cannot be supplied by a catalog.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiText {
    /// Protected active identity; constructor mappings own its exact meaning.
    id: MessageId,
    /// Present only for a typed positive whole-minute message.
    minutes: Option<NonZeroU64>,
    /// Optional closed second message joined by one inert code-owned space.
    tail: Option<MessageId>,
}

impl UiText {
    /// Retain one closed argument-free mapping for later coherent rendering.
    const fn text(id: MessageId) -> Self {
        Self {
            id,
            minutes: None,
            tail: None,
        }
    }

    /// Request a fixed protected ID; the render receiver checks its exact kind.
    #[must_use]
    pub(crate) const fn fixed(id: MessageId) -> Self {
        Self::text(id)
    }

    /// Resolve only a code-owned static enum identity, never a catalog-selected key.
    ///
    /// # Errors
    /// Rejects an unknown, retired or non-text identity before publication.
    pub(crate) fn key(key: &'static str) -> Result<Self, Error> {
        let id = MessageId::from_key(key)?;
        if id.spec()?.kind != super::contract::Kind::Text {
            return Err(Error::Arguments);
        }
        Ok(Self::text(id))
    }

    /// Include a closed no-submission consequence without a nested raw template.
    #[must_use]
    pub const fn notification_feedback(reason: NotificationBlock) -> Self {
        let mut request = Self::notification_block(reason);
        if !matches!(reason, NotificationBlock::NativeAdapterUnavailable) {
            request.tail = Some(MessageId::FEEDBACK_NO_NOTIFICATION_SUBMITTED);
        }
        request
    }

    /// Retain only the closed sticky native-test meaning and optional repeat tail.
    ///
    /// These booleans are selected from the private attempt enum, never from text.
    /// No contributed key, route or arbitrary tail can enter this constructor.
    ///
    /// # Errors
    /// Rejects an impossible protected-ID or text-kind mismatch before publication.
    #[cfg(any(windows, test))]
    pub(crate) fn notification_test_feedback(
        accepted: bool,
        coalesced: bool,
    ) -> Result<Self, Error> {
        let id = if accepted {
            MessageId::NATIVE_NOTIFICATION_TEST_ACCEPTED
        } else {
            MessageId::NATIVE_NOTIFICATION_TEST_INDETERMINATE
        };
        if id.spec()?.kind != super::contract::Kind::Text {
            return Err(Error::Arguments);
        }
        let mut request = Self::text(id);
        if coalesced {
            let tail = MessageId::NATIVE_NOTIFICATION_TEST_COALESCED;
            if tail.spec()?.kind != super::contract::Kind::Text {
                return Err(Error::Arguments);
            }
            request.tail = Some(tail);
        }
        Ok(request)
    }

    /// Select a redacted checked-field help message, never formatting raw input.
    #[must_use]
    pub const fn model_error(error: ModelError) -> Self {
        Self::text(match error {
            ModelError::InvalidLabel => MessageId::HELP_INVALID_LABEL,
            ModelError::InvalidHost => MessageId::HELP_INVALID_HOST,
            ModelError::InvalidPort => MessageId::HELP_INVALID_PORT,
            ModelError::InvalidSelection => MessageId::ERROR_INVALID_SELECTION,
        })
    }

    /// Preserve the existing navigation/presentation error distinction.
    #[must_use]
    pub const fn ui_error(error: UiError) -> Self {
        Self::text(match error {
            UiError::PageUnavailable => MessageId::ERROR_PAGE_UNAVAILABLE,
            UiError::Model(ModelError::InvalidLabel) => MessageId::ERROR_INVALID_LABEL,
            UiError::Model(ModelError::InvalidHost) => MessageId::ERROR_INVALID_HOST,
            UiError::Model(ModelError::InvalidPort) => MessageId::ERROR_INVALID_PORT,
            UiError::Model(ModelError::InvalidSelection) => MessageId::ERROR_INVALID_SELECTION,
        })
    }

    /// Select a fixed preference reason, independent of raw entered values.
    #[must_use]
    pub const fn preference_error(error: PreferenceError) -> Self {
        Self::text(match error {
            PreferenceError::InvalidCustomMinutes => MessageId::ERROR_INVALID_CUSTOM_MINUTES,
            PreferenceError::TrayRequired => MessageId::ERROR_TRAY_REQUIRED,
        })
    }

    /// Map the closed source failure without recursively interpreting Display.
    #[must_use]
    pub const fn source_error(error: SourceDraftError) -> Self {
        match error {
            SourceDraftError::Field(error) => Self::model_error(error),
            SourceDraftError::InvalidProtocol => Self::text(MessageId::RUST_PROTOCOL_CHOICE),
            SourceDraftError::UnknownSource => Self::text(MessageId::RUST_SOURCE_CHOICE),
            SourceDraftError::DuplicateIdentity => Self::text(MessageId::RUST_DUPLICATE_SOURCE),
            SourceDraftError::Capacity => Self::text(MessageId::RUST_SOURCE_CAPACITY),
            SourceDraftError::IdExhausted => Self::text(MessageId::RUST_SOURCE_ID_EXHAUSTED),
        }
    }

    /// Keep query/filter/member rejection as a closed fact, not cached words.
    #[must_use]
    pub const fn inspection_error(error: crate::source_inspection::InspectionError) -> Self {
        use crate::source_inspection::InspectionError;
        match error {
            InspectionError::InvalidQuery => Self::text(MessageId::INSPECTION_INVALID_QUERY),
            InspectionError::InvalidFilter => Self::text(MessageId::INSPECTION_INVALID_FILTER),
            InspectionError::RolesUnavailable => {
                Self::text(MessageId::INSPECTION_ROLES_UNAVAILABLE)
            }
            InspectionError::Source(error) => Self::source_error(error),
            InspectionError::HiddenSource => Self::text(MessageId::INSPECTION_HIDDEN_SOURCE),
        }
    }

    /// Keep the Date recovery screen as context; no nested `{error}` template.
    #[must_use]
    pub const fn recovery_error(error: RecoveryError) -> Self {
        match error {
            RecoveryError::InvalidCount => Self::text(MessageId::RUST_HELPER_COUNT),
            RecoveryError::Source(error) => Self::source_error(error),
        }
    }

    /// Retain the actual unavailable-submission reason, with no delivery claim.
    #[must_use]
    pub const fn notification_block(reason: NotificationBlock) -> Self {
        Self::text(match reason {
            NotificationBlock::PreferenceOff => MessageId::RUST_NOTIFICATIONS_OFF,
            NotificationBlock::ObserverUnavailable => MessageId::RUST_OBSERVER_UNCONFIRMED,
            NotificationBlock::WindowsUnavailable => MessageId::RUST_NOTIFICATION_OS_UNCONFIRMED,
            NotificationBlock::WindowsBlocked => MessageId::RUST_NOTIFICATIONS_BLOCKED,
            NotificationBlock::NativeAdapterUnavailable => {
                MessageId::RUST_NOTIFICATIONS_NATIVE_UNAVAILABLE
            }
        })
    }

    /// Render service continuity from freshness-aware typed runtime classification.
    #[must_use]
    pub const fn quit_headline(state: QuitServiceState) -> Self {
        Self::text(match state {
            QuitServiceState::Running => MessageId::RUST_QUIT_RUNNING_TITLE,
            QuitServiceState::Stopped => MessageId::RUST_QUIT_STOPPED_TITLE,
            QuitServiceState::PartiallyRunning => MessageId::RUST_QUIT_PARTIAL_TITLE,
            QuitServiceState::Transitioning => MessageId::RUST_QUIT_PENDING_TITLE,
            QuitServiceState::Unconfirmed => MessageId::RUST_QUIT_UNKNOWN_TITLE,
        })
    }

    /// State-dependent UI-only consequences never infer service or clock effects.
    #[must_use]
    pub const fn quit_consequence(state: QuitServiceState) -> Self {
        Self::text(match state {
            QuitServiceState::Running => MessageId::RUST_QUIT_RUNNING_CONSEQUENCE,
            QuitServiceState::Stopped => MessageId::RUST_QUIT_STOPPED_CONSEQUENCE,
            QuitServiceState::PartiallyRunning => MessageId::RUST_QUIT_PARTIAL_CONSEQUENCE,
            QuitServiceState::Transitioning => MessageId::RUST_QUIT_PENDING_CONSEQUENCE,
            QuitServiceState::Unconfirmed => MessageId::SHELL_UNKNOWN_CONSEQUENCE,
        })
    }

    /// The checked return destination chooses words; words never choose a route.
    #[must_use]
    pub const fn recovery_back(page: Page) -> Self {
        Self::text(match page {
            Page::Status => MessageId::RUST_BACK_STATUS,
            Page::Sources => MessageId::RUST_BACK_SOURCES,
            Page::Statistics => MessageId::RUST_BACK_STATISTICS,
            Page::Diagnostics => MessageId::RUST_BACK_DIAGNOSTICS,
            Page::About => MessageId::RUST_BACK_ABOUT,
            Page::Settings | Page::Recovery => MessageId::RUST_BACK_SETTINGS,
        })
    }

    /// `ChangeKind` owns badge/review semantics independently of the display label.
    #[must_use]
    pub const fn source_change(change: Option<ChangeKind>) -> Self {
        Self::text(match change {
            Some(ChangeKind::Add) => MessageId::RUST_CHANGE_ADD,
            Some(ChangeKind::Edit) => MessageId::RUST_CHANGE_EDIT,
            Some(ChangeKind::Remove) => MessageId::RUST_CHANGE_REMOVE,
            None => MessageId::RUST_CHANGE_BASELINE,
        })
    }

    /// Keep the accepted interval as typed facts, retaining the custom distinction.
    ///
    /// # Errors
    /// Rejects an impossible zero or missing checked minute value, without output.
    pub fn interval(interval: CheckInterval) -> Result<Self, Error> {
        let (id, value) = match interval {
            CheckInterval::Automatic => return Ok(Self::text(MessageId::INTERVAL_AUTOMATIC)),
            CheckInterval::TwoMinutes => (MessageId::INTERVAL_EVERY, 2),
            CheckInterval::FiveMinutes => (MessageId::INTERVAL_EVERY, 5),
            CheckInterval::FifteenMinutes => (MessageId::INTERVAL_EVERY, 15),
            CheckInterval::Custom(value) => (MessageId::INTERVAL_CUSTOM, value.get()),
        };
        Ok(Self {
            id,
            minutes: Some(NonZeroU64::new(value).ok_or(Error::Arguments)?),
            tail: None,
        })
    }

    /// Render only from the caller's one coherent immutable snapshot.
    ///
    /// # Errors
    /// Rejects any protected-contract mismatch or bounded formatted-output failure.
    pub fn render(self, snapshot: &Snapshot<'_>) -> Result<PlainText, Error> {
        let mut value = if let Some(minutes) = self.minutes {
            snapshot.minutes(self.id, minutes)?
        } else {
            snapshot.text(self.id)?
        };
        if let Some(tail) = self.tail {
            let tail = snapshot.text(tail)?;
            super::append(&mut value.0, " ")?;
            super::append(&mut value.0, tail.as_str())?;
        }
        Ok(value)
    }
}

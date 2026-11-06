//! Inert local-notification boundary and explicitly labelled fixture previews.
//!
//! The retained M0 port below is always unavailable. Readiness observations, an accepted
//! preference and showing an in-window example never establish OS submission,
//! banner visibility, reading, synchronization or service authority. This module
//! owns no transport, registration, worker, timer, persistence or service state.
#![forbid(unsafe_code)]

/// Portable local-toast admission/correlation candidate; no native effect.
pub mod native_delivery;

use std::num::NonZeroU64;

use crate::preferences::UiPreferences;
use crate::shell::{AdapterAvailability, Observation, SettingsSection, ShellRoute};

/// A Windows notification setting observation, distinct from app preferences.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowsNotificationSetting {
    /// Windows permits submission; it may still suppress a banner.
    Allowed,
    /// User or application settings block notifications.
    BlockedForUser,
    /// Organization policy blocks notifications; this UI cannot override it.
    BlockedByPolicy,
    /// The notification facility is unsupported for the observed configuration.
    Unsupported,
}

/// Whether a user-facing observer is currently available to present new alerts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObserverPresence {
    /// A window or tray observer is running; this grants no service authority.
    Running,
    /// No user observer is running; services may independently continue.
    Stopped,
}

/// Why an inert test-notification request could not submit anything to Windows.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NotificationBlock {
    /// The accepted app preference disables problem notifications.
    PreferenceOff,
    /// A running observer has not been currently established.
    ObserverUnavailable,
    /// The Windows setting is unknown, stale or unsupported.
    WindowsUnavailable,
    /// A current Windows user or policy setting blocks notifications.
    WindowsBlocked,
    /// No native notification adapter is installed in this fixture.
    NativeAdapterUnavailable,
}

impl NotificationBlock {
    /// Return fixed code-owned feedback without any private source/diagnostic data.
    #[must_use]
    pub const fn text(self) -> &'static str {
        match self {
            Self::PreferenceOff => "Problem notifications are off in NevroTick.",
            Self::ObserverUnavailable => {
                "A running NevroTick window or tray observer is not confirmed."
            }
            Self::WindowsUnavailable => {
                "Windows notification availability is unconfirmed or unavailable."
            }
            Self::WindowsBlocked => "Windows settings or organization policy block notifications.",
            Self::NativeAdapterUnavailable => {
                "Windows notifications are unavailable in this fixture. No notification was submitted."
            }
        }
    }
}

/// Fixed unavailable port, without a hidden live implementation or success case.
pub struct UnavailableNotificationPort;

impl UnavailableNotificationPort {
    /// Report actual adapter availability, independently of settings observations.
    #[must_use]
    pub const fn availability() -> AdapterAvailability {
        AdapterAvailability::Unavailable
    }

    /// Check accepted preference/current presentation facts, then reject submission.
    ///
    /// This operation is inert and does not consume an incident's first alert,
    /// change preferences or show an example. Windows permission is separate from
    /// this port's fixed unavailability; no branch returns send/delivery success.
    #[must_use]
    pub fn request_test(
        preferences: &UiPreferences,
        windows: Observation<WindowsNotificationSetting>,
        observer: Observation<ObserverPresence>,
    ) -> NotificationBlock {
        if !preferences.notifications_enabled() {
            return NotificationBlock::PreferenceOff;
        }
        if !matches!(observer, Observation::Current(ObserverPresence::Running)) {
            return NotificationBlock::ObserverUnavailable;
        }
        match windows {
            Observation::Current(WindowsNotificationSetting::Allowed) => {
                NotificationBlock::NativeAdapterUnavailable
            }
            Observation::Current(
                WindowsNotificationSetting::BlockedForUser
                | WindowsNotificationSetting::BlockedByPolicy,
            ) => NotificationBlock::WindowsBlocked,
            Observation::Unknown
            | Observation::Stale(_)
            | Observation::Current(WindowsNotificationSetting::Unsupported) => {
                NotificationBlock::WindowsUnavailable
            }
        }
    }
}

/// Code-owned sample context; no string can supply an action, URI or target.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureNoticeContext {
    /// A sample status-availability issue opens diagnostics for the current facts.
    StatusUnavailable,
    /// A sample service issue opens management without sending a service command.
    ServiceNeedsReview,
    /// A sample test opens notification preferences, never asserting clock health.
    TestExample,
}

impl FixtureNoticeContext {
    /// Choose only existing typed navigation; this performs no navigation or effect.
    const fn route(self) -> ShellRoute {
        match self {
            Self::StatusUnavailable => ShellRoute::Diagnostics,
            Self::ServiceNeedsReview => ShellRoute::Settings(SettingsSection::BackgroundService),
            Self::TestExample => ShellRoute::Settings(SettingsSection::Notifications),
        }
    }
}

/// Checked identity and closed context for one explicitly sample-only notice.
///
/// This is not a wire record or OS activation token. A fixture's maintained
/// producer assigns a new identity for a new sample context; identity is not
/// authorization, freshness, persistence classification or evidence of recovery.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixtureNotice {
    /// Nonzero fixed-size identity; zero cannot enter a maintained notice.
    id: NonZeroU64,
    /// Code-owned navigation context, with no arbitrary text/action payload.
    context: FixtureNoticeContext,
}

impl FixtureNotice {
    /// Admit a nonzero identity without allocation, decoding or OS effects.
    #[must_use]
    pub fn new(id: u64, context: FixtureNoticeContext) -> Option<Self> {
        NonZeroU64::new(id).map(|id| Self { id, context })
    }

    /// Return the retained nonzero identity for a typed preview activation.
    #[must_use]
    pub const fn id(self) -> u64 {
        self.id.get()
    }

    /// Return the closed sample context for code-owned preview presentation.
    #[must_use]
    pub const fn context(self) -> FixtureNoticeContext {
        self.context
    }
}

/// One bounded in-window example, separate from native submission/delivery state.
///
/// Repeated examples with this identity coalesce. Dismissal hides only the card;
/// it preserves context and coalescing. This owner stores no app preferences or
/// service request result: toggles and cancelled service dialogs cannot reset it.
/// A new fixture owner starts a separate sample, not a production incident epoch.
#[derive(Debug, Eq, PartialEq)]
pub struct FixtureNotificationPreview {
    /// Retained identity/context for validation after the card is dismissed.
    notice: FixtureNotice,
    /// Whether this sample has already been previewed, never a native send claim.
    previewed: bool,
    /// Presentation of the in-window example, independent of the current problem.
    visible: bool,
}

impl FixtureNotificationPreview {
    /// Own one checked fixture notice, initially hidden and not yet previewed.
    #[must_use]
    pub const fn new(notice: FixtureNotice) -> Self {
        Self {
            notice,
            previewed: false,
            visible: false,
        }
    }

    /// Return the retained sample identity/context without consuming an alert.
    #[must_use]
    pub const fn notice(&self) -> FixtureNotice {
        self.notice
    }

    /// Report only the visibility of this in-window example.
    #[must_use]
    pub const fn visible(&self) -> bool {
        self.visible
    }

    /// Show the first example once; return false without change for a duplicate.
    ///
    /// The UI must label it as an example, with no system notification submitted.
    /// This method neither checks nor claims native notification readiness.
    #[must_use]
    pub fn show_example(&mut self) -> bool {
        if self.previewed {
            return false;
        }
        self.previewed = true;
        self.visible = true;
        true
    }

    /// Hide the example without forgetting its identity or permitting duplicates.
    pub const fn dismiss(&mut self) {
        self.visible = false;
    }

    /// Resolve a known sample activation against current matching context only.
    ///
    /// Unknown/stale/successor context and unknown IDs open Status. The result is
    /// typed navigation for the shell owner to apply; it cannot approve a clock
    /// correction, repeat a request, change services or elevate the application.
    #[must_use]
    pub fn activate(&self, id: u64, current: Observation<FixtureNotice>) -> ShellRoute {
        if !self.previewed {
            return ShellRoute::Status;
        }
        match current {
            Observation::Current(notice) if notice == self.notice && id == notice.id.get() => {
                notice.context.route()
            }
            Observation::Unknown | Observation::Stale(_) | Observation::Current(_) => {
                ShellRoute::Status
            }
        }
    }
}

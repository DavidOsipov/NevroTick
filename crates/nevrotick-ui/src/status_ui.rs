//! Closed, UI-owned status presentation input and an unavailable fixture port.
//!
//! This is neither a wire format nor another clock-trust state machine. A future
//! adapter maps a coherent, validated canonical observation into these values.
//! No packet, source count, running service, elapsed duration or UI request can
//! produce Tracking here. There is no timer, clock math or transition engine.
#![forbid(unsafe_code)]

use std::fmt;

use crate::shell::{Observation, SettingsSection, ShellRoute};

/// Reported presentation condition; profile and evidence remain independent.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StatusKind {
    /// No current condition has been supplied.
    Unknown,
    /// The local status connection is unavailable; service health is unconfirmed.
    Disconnected,
    /// Configuration/ownership readiness is still being checked.
    Initializing,
    /// Current admissible measurements have not established tracking.
    Acquiring,
    /// The canonical owner explicitly reports qualified current tracking.
    Tracking,
    /// The canonical owner reports a still-usable retained model.
    Holdover,
    /// The previous estimate is no longer usable.
    Expired,
    /// Current sources disagree; disputed evidence grants no new correction.
    Conflict,
    /// A named prerequisite prevents synchronization; inspect current facts.
    Blocked,
    /// The source owner explicitly reports stopped services, not mere disconnect.
    ServiceStopped,
    /// The owner explicitly reports Windows Time management, not verified accuracy.
    WindowsManaged,
    /// Explicit observation presentation; this requires the independent profile.
    ObserveOnly,
    /// A separate correction proposal needs review; a check cannot approve it.
    CorrectionPending,
    /// An operation's actual effect needs reconciliation before retry.
    UncertainResult,
    /// The canonical request owner reports a limit, without a UI-derived timeout.
    RateLimited,
    /// Current-generation evidence is required after a change.
    Reacquiring,
    /// Measurements around a UTC transition are temporarily protected.
    LeapGuard,
    /// Provisional date recovery is in progress; it grants no global clock write.
    DateRecovery,
}

/// Source/actuation profile, independent of service availability and state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StatusProfile {
    /// No active profile has been observed; an editor preview is not this value.
    Unknown,
    /// An NTS-only profile was explicitly reported by its owner.
    Nts,
    /// An explicitly selected plain-NTP profile was reported.
    PlainNtp,
    /// This product's clock writes are disabled; acquisition mode is independent.
    ObserveOnly,
}

/// Authentication supplied by the canonical evidence owner, never inferred here.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthenticationEvidence {
    /// No authentication fact has been supplied.
    Unknown,
    /// Current authentication has not been established.
    NotEstablished,
    /// Current qualified NTS authentication was explicitly reported.
    CurrentNts,
    /// Current plain-NTP evidence is explicitly unauthenticated.
    Unauthenticated,
    /// Only prior NTS authentication is available.
    HistoricalNts,
}

/// Agreement supplied separately from authentication and source inventory size.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgreementEvidence {
    /// No agreement evaluation has been supplied.
    Unknown,
    /// Enough admissible current agreement has not yet been established.
    Waiting,
    /// The canonical owner reports its current agreement policy satisfied.
    CurrentSatisfied,
    /// Current qualified evidence conflicts.
    Conflict,
    /// Retained agreement is historical, not a new vote.
    Historical,
}

/// Usability of the last qualified estimate, not packet or UI snapshot age.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EstimateFreshness {
    /// No qualified estimate or age has been supplied.
    Unknown,
    /// The canonical owner considers the qualified estimate current.
    Current,
    /// The canonical owner considers the prior model within its retained budget.
    RetainedUsable,
    /// Only history is available; current usability is unconfirmed.
    Historical,
    /// The canonical owner reports its quality/age budget expired.
    Expired,
    /// Evidence must be withheld pending agreement, ownership or reconciliation.
    Withheld,
}

/// A contradictory presentation input, without raw data or diagnostic details.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StatusInputError {
    /// Tracking lacked explicit current, profile-compatible qualified evidence.
    UnqualifiedTracking,
    /// Holdover did not carry an explicitly still-usable retained estimate.
    InvalidHoldover,
    /// Expired status did not identify an expired qualified estimate.
    InvalidExpired,
    /// Observe-only presentation contradicted the independently supplied profile.
    InvalidObserveOnly,
    /// Source-conflict presentation lacked explicitly conflicting agreement.
    InvalidConflict,
    /// An uncertain operation cannot publish a usable current clock estimate.
    InvalidUncertainResult,
}

impl fmt::Display for StatusInputError {
    /// Format a fixed input-boundary rejection; this text is engineering-only.
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        output.write_str("Inconsistent status presentation input")
    }
}

impl std::error::Error for StatusInputError {}

/// Immutable checked report from one canonical observation, before staleness.
///
/// The outer [`Observation`] owns snapshot currency. Estimate, authentication and
/// agreement fields have their own meanings and cannot refresh one another.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StatusReport {
    /// Explicit canonical-owner presentation condition.
    kind: StatusKind,
    /// Independently supplied active profile, never a local policy draft.
    profile: StatusProfile,
    /// Authentication does not establish agreement or clock accuracy.
    authentication: AuthenticationEvidence,
    /// Agreement is never calculated from configured identity counts here.
    agreement: AgreementEvidence,
    /// Estimate usability is never recalculated from a UI clock.
    freshness: EstimateFreshness,
}

impl StatusReport {
    /// Admit one closed report without allocating or changing any accepted state.
    ///
    /// Positive Tracking requires all explicit current evidence; this constructor
    /// verifies consistency, not evidence authenticity or production authority.
    /// Observe-only may accompany any condition and never implies Windows Time.
    ///
    /// # Errors
    /// Reject contradictory Tracking, Holdover, Expired, Conflict, uncertain-result
    /// or Observe-only inputs; no rejected field is retained.
    pub const fn new(
        kind: StatusKind,
        profile: StatusProfile,
        authentication: AuthenticationEvidence,
        agreement: AgreementEvidence,
        freshness: EstimateFreshness,
    ) -> Result<Self, StatusInputError> {
        let authentication_matches = match profile {
            StatusProfile::Nts => matches!(authentication, AuthenticationEvidence::CurrentNts),
            StatusProfile::PlainNtp => {
                matches!(authentication, AuthenticationEvidence::Unauthenticated)
            }
            StatusProfile::ObserveOnly => matches!(
                authentication,
                AuthenticationEvidence::CurrentNts | AuthenticationEvidence::Unauthenticated
            ),
            StatusProfile::Unknown => false,
        };
        if matches!(kind, StatusKind::Tracking)
            && (!authentication_matches
                || !matches!(agreement, AgreementEvidence::CurrentSatisfied)
                || !matches!(freshness, EstimateFreshness::Current))
        {
            return Err(StatusInputError::UnqualifiedTracking);
        }
        if matches!(kind, StatusKind::Holdover)
            && (!matches!(freshness, EstimateFreshness::RetainedUsable)
                || matches!(agreement, AgreementEvidence::CurrentSatisfied))
        {
            return Err(StatusInputError::InvalidHoldover);
        }
        if matches!(kind, StatusKind::Expired)
            && (!matches!(freshness, EstimateFreshness::Expired)
                || matches!(agreement, AgreementEvidence::CurrentSatisfied))
        {
            return Err(StatusInputError::InvalidExpired);
        }
        if matches!(kind, StatusKind::ObserveOnly) && !matches!(profile, StatusProfile::ObserveOnly)
        {
            return Err(StatusInputError::InvalidObserveOnly);
        }
        if matches!(kind, StatusKind::Conflict) && !matches!(agreement, AgreementEvidence::Conflict)
        {
            return Err(StatusInputError::InvalidConflict);
        }
        if matches!(kind, StatusKind::UncertainResult)
            && !matches!(
                freshness,
                EstimateFreshness::Unknown
                    | EstimateFreshness::Withheld
                    | EstimateFreshness::Historical
                    | EstimateFreshness::Expired
            )
        {
            return Err(StatusInputError::InvalidUncertainResult);
        }
        Ok(Self {
            kind,
            profile,
            authentication,
            agreement,
            freshness,
        })
    }

    /// Known local absence of a status adapter, without inferring service failure.
    #[must_use]
    pub const fn disconnected() -> Self {
        Self {
            kind: StatusKind::Disconnected,
            profile: StatusProfile::Unknown,
            authentication: AuthenticationEvidence::Unknown,
            agreement: AgreementEvidence::Unknown,
            freshness: EstimateFreshness::Unknown,
        }
    }

    /// Read the reported condition; no state transition or effect is performed.
    #[must_use]
    pub const fn kind(self) -> StatusKind {
        self.kind
    }

    /// Read the independent profile; a view cannot turn it into clock authority.
    #[must_use]
    pub const fn profile(self) -> StatusProfile {
        self.profile
    }
}

/// An ordinary check intention, granting no time target or correction authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StatusCheckIntent {
    /// Fresh measurements under the active synchronization configuration.
    SyncNow,
    /// Fresh measurements without implying product clock writes.
    CheckSources,
}

/// Closed contextual remedy; labels cannot select routes or service operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StatusRemedy {
    /// Inspect current facts and reasons in explicit Power user mode.
    Diagnostics,
    /// Compare source evidence in explicit Power user mode.
    Sources,
    /// Review background service management without submitting a service command.
    BackgroundService,
    /// Read the separate correction review; never authorize or retry a correction.
    CorrectionReview,
}

impl StatusRemedy {
    /// Return only a typed existing UI route, without navigating or effects.
    #[must_use]
    pub const fn route(self) -> ShellRoute {
        match self {
            Self::Diagnostics => ShellRoute::Diagnostics,
            Self::Sources => ShellRoute::Sources,
            Self::BackgroundService => ShellRoute::Settings(SettingsSection::BackgroundService),
            Self::CorrectionReview => ShellRoute::CorrectionReview,
        }
    }
}

/// Fixed unavailable check result; there is no fabricated submitted/success case.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StatusCheckOutcome {
    /// Nothing was submitted; there is no connected measurement-request adapter.
    AdapterUnavailable,
}

/// Narrow UI-input port; implementations must use existing validated authority.
///
/// This DTO interface does not admit a production IPC protocol, freshness rule,
/// service command or scheduler. A caller never derives current Tracking from a
/// packet, a service-running observation or the fact a check was requested.
pub trait StatusPresentationPort {
    /// Read independently owned facts without refreshing them or requesting work.
    fn observe(&self) -> Observation<StatusReport>;
}

/// Reached fixture port: no IPC, service, measurement, clock or native work.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FixtureStatusPort;

impl StatusPresentationPort for FixtureStatusPort {
    fn observe(&self) -> Observation<StatusReport> {
        Observation::Current(StatusReport::disconnected())
    }
}

impl FixtureStatusPort {
    /// Reject even a directly invoked check; opening a page/menu is not a check.
    #[must_use]
    pub const fn request_check(self, _intent: StatusCheckIntent) -> StatusCheckOutcome {
        StatusCheckOutcome::AdapterUnavailable
    }
}

/// Immutable projection; outer historical snapshots suppress current claims.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StatusView {
    /// An optional previously validated report, retained only for presentation.
    report: Option<StatusReport>,
    /// Currency is supplied by the original owner, not computed by this UI.
    current: bool,
}

impl StatusView {
    /// Retain history while making unavailable/current presentation explicit.
    #[must_use]
    pub const fn from_observation(observation: Observation<StatusReport>) -> Self {
        match observation {
            Observation::Unknown => Self {
                report: None,
                current: false,
            },
            Observation::Current(report) => Self {
                report: Some(report),
                current: true,
            },
            Observation::Stale(report) => Self {
                report: Some(report),
                current: false,
            },
        }
    }

    /// Return the current display condition, never stale Tracking as healthy.
    #[must_use]
    pub const fn kind(self) -> StatusKind {
        match (self.current, self.report) {
            (true, Some(report)) => report.kind,
            (false, Some(_)) => StatusKind::Disconnected,
            (_, None) => StatusKind::Unknown,
        }
    }

    /// Preserve reported profile independently of headline/service condition.
    #[must_use]
    pub const fn profile(self) -> StatusProfile {
        match self.report {
            Some(report) => report.profile,
            None => StatusProfile::Unknown,
        }
    }

    /// Whether this view has a currently usable status report, not clock accuracy.
    #[must_use]
    pub const fn has_current_report(self) -> bool {
        self.current && !matches!(self.kind(), StatusKind::Unknown | StatusKind::Disconnected)
    }

    /// Profile wording remains separate from the current condition and remedy.
    #[must_use]
    pub const fn profile_text(self) -> StatusText {
        match self.profile() {
            StatusProfile::Unknown => StatusText::ProfileUnknown,
            StatusProfile::Nts => StatusText::ProfileNts,
            StatusProfile::PlainNtp => StatusText::ProfileNtp,
            StatusProfile::ObserveOnly => StatusText::ProfileObserveOnly,
        }
    }

    /// An explicit observation profile changes check wording, not service health.
    #[must_use]
    pub const fn check_intent(self) -> StatusCheckIntent {
        if matches!(self.profile(), StatusProfile::ObserveOnly)
            || matches!(
                self.kind(),
                StatusKind::Unknown
                    | StatusKind::Disconnected
                    | StatusKind::Blocked
                    | StatusKind::UncertainResult
                    | StatusKind::WindowsManaged
                    | StatusKind::ObserveOnly
            )
        {
            StatusCheckIntent::CheckSources
        } else {
            StatusCheckIntent::SyncNow
        }
    }

    /// Choose contextual navigation only; no branch authorizes a correction.
    #[must_use]
    pub const fn remedy(self) -> StatusRemedy {
        match self.kind() {
            StatusKind::Conflict => StatusRemedy::Sources,
            StatusKind::ServiceStopped | StatusKind::WindowsManaged => {
                StatusRemedy::BackgroundService
            }
            StatusKind::CorrectionPending => StatusRemedy::CorrectionReview,
            _ => StatusRemedy::Diagnostics,
        }
    }

    /// Derive the same contextual check meaning for the inert tray contract.
    #[must_use]
    pub const fn tray_menu(self) -> crate::tray_ui::FixtureTrayMenu {
        use crate::tray_ui::{FixtureTrayMenu, TrayCheckBlock, TrayCheckContext};
        let context = if matches!(self.kind(), StatusKind::ServiceStopped) {
            TrayCheckContext::ServicesStopped
        } else if matches!(self.check_intent(), StatusCheckIntent::CheckSources) {
            TrayCheckContext::CheckSourcesOnly
        } else {
            TrayCheckContext::Synchronization
        };
        let block = match self.kind() {
            StatusKind::Unknown | StatusKind::Disconnected => TrayCheckBlock::StateUnconfirmed,
            StatusKind::UncertainResult | StatusKind::CorrectionPending => {
                TrayCheckBlock::OperationPending
            }
            StatusKind::RateLimited => TrayCheckBlock::RateLimited,
            _ => TrayCheckBlock::NativeAdapterUnavailable,
        };
        FixtureTrayMenu::new(context, block)
    }

    /// Read one code-owned headline meaning, not text selected by a catalog.
    #[must_use]
    pub const fn headline(self) -> StatusText {
        match self.kind() {
            StatusKind::Unknown => StatusText::StatusUnknown,
            StatusKind::Disconnected => StatusText::NoConnection,
            StatusKind::Initializing => StatusText::Initializing,
            StatusKind::Acquiring => StatusText::Acquiring,
            StatusKind::Tracking if matches!(self.profile(), StatusProfile::ObserveOnly) => {
                StatusText::Observing
            }
            StatusKind::Tracking => StatusText::Tracking,
            StatusKind::Holdover => StatusText::Holdover,
            StatusKind::Expired => StatusText::Expired,
            StatusKind::Conflict => StatusText::Conflict,
            StatusKind::Blocked => StatusText::Blocked,
            StatusKind::ServiceStopped => StatusText::ServiceStopped,
            StatusKind::WindowsManaged => StatusText::WindowsManaged,
            StatusKind::ObserveOnly => StatusText::Observing,
            StatusKind::CorrectionPending => StatusText::CorrectionPending,
            StatusKind::UncertainResult => StatusText::UncertainResult,
            StatusKind::RateLimited => StatusText::RateLimited,
            StatusKind::Reacquiring => StatusText::Reacquiring,
            StatusKind::LeapGuard => StatusText::LeapGuard,
            StatusKind::DateRecovery => StatusText::DateRecovery,
        }
    }

    /// Present the checked request meaning; the fixture always disables its effect.
    #[must_use]
    pub const fn check_text(self) -> StatusText {
        match self.check_intent() {
            StatusCheckIntent::SyncNow => StatusText::SyncNow,
            StatusCheckIntent::CheckSources => StatusText::CheckSources,
        }
    }

    /// Present only the safe contextual route's label.
    #[must_use]
    pub const fn remedy_text(self) -> StatusText {
        match self.remedy() {
            StatusRemedy::Diagnostics => StatusText::OpenDiagnostics,
            StatusRemedy::Sources => StatusText::OpenSources,
            StatusRemedy::BackgroundService => StatusText::ManageService,
            StatusRemedy::CorrectionReview => StatusText::OpenCorrectionReview,
        }
    }

    /// Explain the current condition without inventing measurements or effects.
    #[must_use]
    pub const fn consequence(self) -> StatusText {
        if !self.current && self.report.is_some() {
            return StatusText::HistoricalEvidence;
        }
        match self.kind() {
            StatusKind::Unknown | StatusKind::Disconnected => StatusText::UnavailableDetail,
            StatusKind::Initializing
            | StatusKind::Acquiring
            | StatusKind::Reacquiring
            | StatusKind::LeapGuard => StatusText::WaitingDetail,
            StatusKind::DateRecovery => StatusText::DateRecoveryDetail,
            StatusKind::Tracking if matches!(self.profile(), StatusProfile::ObserveOnly) => {
                StatusText::ObserveDetail
            }
            StatusKind::Tracking => StatusText::TrackingDetail,
            StatusKind::Holdover => StatusText::HoldoverDetail,
            StatusKind::Expired | StatusKind::Conflict | StatusKind::Blocked => {
                StatusText::WithheldDetail
            }
            StatusKind::ServiceStopped => StatusText::StoppedDetail,
            StatusKind::WindowsManaged => StatusText::WindowsDetail,
            StatusKind::ObserveOnly => StatusText::ObserveDetail,
            StatusKind::CorrectionPending => StatusText::ReviewDetail,
            StatusKind::UncertainResult => StatusText::UncertainDetail,
            StatusKind::RateLimited => StatusText::LimitedDetail,
        }
    }

    /// Current and historical authentication never imply current agreement.
    #[must_use]
    pub const fn authentication(self) -> StatusText {
        let Some(report) = self.report else {
            return StatusText::NotEstablished;
        };
        if !self.has_current_report()
            && !matches!(
                report.authentication,
                AuthenticationEvidence::Unknown | AuthenticationEvidence::NotEstablished
            )
        {
            return StatusText::HistoricalEvidence;
        }
        match report.authentication {
            AuthenticationEvidence::CurrentNts if self.has_current_report() => {
                StatusText::Authenticated
            }
            AuthenticationEvidence::CurrentNts | AuthenticationEvidence::HistoricalNts => {
                StatusText::HistoricalEvidence
            }
            AuthenticationEvidence::Unauthenticated => StatusText::Unauthenticated,
            AuthenticationEvidence::Unknown | AuthenticationEvidence::NotEstablished => {
                StatusText::NotEstablished
            }
        }
    }

    /// Historical agreement cannot become a current vote on window reopen.
    #[must_use]
    pub const fn agreement(self) -> StatusText {
        let Some(report) = self.report else {
            return StatusText::NotEvaluated;
        };
        if !self.has_current_report() && !matches!(report.agreement, AgreementEvidence::Unknown) {
            return StatusText::HistoricalEvidence;
        }
        match report.agreement {
            AgreementEvidence::CurrentSatisfied if self.has_current_report() => {
                StatusText::AgreementSatisfied
            }
            AgreementEvidence::CurrentSatisfied | AgreementEvidence::Historical => {
                StatusText::HistoricalEvidence
            }
            AgreementEvidence::Conflict => StatusText::Conflict,
            AgreementEvidence::Waiting => StatusText::Acquiring,
            AgreementEvidence::Unknown => StatusText::NotEvaluated,
        }
    }

    /// Estimate usability remains distinct from authentication and snapshot age.
    #[must_use]
    pub const fn freshness(self) -> StatusText {
        let Some(report) = self.report else {
            return StatusText::NotAvailable;
        };
        if !self.has_current_report() && !matches!(report.freshness, EstimateFreshness::Unknown) {
            return StatusText::HistoricalEvidence;
        }
        match report.freshness {
            EstimateFreshness::Current => StatusText::CurrentEstimate,
            EstimateFreshness::RetainedUsable => StatusText::Holdover,
            EstimateFreshness::Historical => StatusText::HistoricalEvidence,
            EstimateFreshness::Expired => StatusText::Expired,
            EstimateFreshness::Withheld => StatusText::WithheldDetail,
            EstimateFreshness::Unknown => StatusText::NotAvailable,
        }
    }
}

/// Code-owned message meaning; translation bytes cannot select state or actions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StatusText {
    /// Status not observed
    StatusUnknown,
    /// No live connection
    NoConnection,
    /// Starting and checking configuration
    Initializing,
    /// Checking time sources
    Acquiring,
    /// Clock changes by `NevroTick` are disabled
    Observing,
    /// Time sync active
    Tracking,
    /// No fresh time check - using the previous model
    Holdover,
    /// No usable current time estimate
    Expired,
    /// Time sources disagree
    Conflict,
    /// Synchronization needs attention
    Blocked,
    /// Background services are stopped
    ServiceStopped,
    /// Windows Time manages the clock
    WindowsManaged,
    /// A clock correction needs separate review
    CorrectionPending,
    /// The operation result is unknown
    UncertainResult,
    /// Time checks are currently limited
    RateLimited,
    /// Rechecking after a change
    Reacquiring,
    /// Rechecking around a UTC transition
    LeapGuard,
    /// Recovering a provisional date
    DateRecovery,
    /// Current synchronization is unconfirmed. A missing status connection does not establish that services stopped. Inspect diagnostics for the available facts.
    UnavailableDetail,
    /// Waiting for fresh admissible evidence. A running service or a reply alone does not establish synchronization.
    WaitingDetail,
    /// Observe only authorizes no product clock writes. It does not turn on Windows Time or settle the acquisition and authentication mode.
    ObserveDetail,
    /// The reported current policy is met. Authentication does not guarantee UTC accuracy; clock effects remain separate observations.
    TrackingDetail,
    /// The owner reports the previous model is still usable. No fresh agreement is available; uncertainty and model limits remain important.
    HoldoverDetail,
    /// This status grants no new clock correction. Review the current prerequisite and evidence before continuing.
    WithheldDetail,
    /// Stopped services cannot perform a fresh time check. Startup configuration, clock cleanup and Windows Time remain separate facts.
    StoppedDetail,
    /// `NevroTick` does not control the clock in this reported mode. Windows Time ownership does not establish verified synchronization.
    WindowsDetail,
    /// Review a current proposal separately. A check, opening this page or dismissing a notification cannot approve a clock change.
    ReviewDetail,
    /// Reconcile the actual result and clock ownership before repeating the operation. No outcome or cleanup is assumed.
    UncertainDetail,
    /// The reported request limit remains in force. Reopening this window does not reset it; no retry duration is invented here.
    LimitedDetail,
    /// Provisional date recovery does not change Windows time or establish authenticated tracking. Full verification and fresh agreement are still required.
    DateRecoveryDetail,
    /// Current NTS authentication established
    Authenticated,
    /// Historical evidence only - current state unconfirmed
    HistoricalEvidence,
    /// NTP - unauthenticated
    Unauthenticated,
    /// Not established
    NotEstablished,
    /// Current agreement policy met
    AgreementSatisfied,
    /// Not evaluated
    NotEvaluated,
    /// Current qualified estimate - age not supplied
    CurrentEstimate,
    /// Not available
    NotAvailable,
    /// Not observed
    ProfileUnknown,
    /// NTS only
    ProfileNts,
    /// NTP - unauthenticated
    ProfileNtp,
    /// Observe only
    ProfileObserveOnly,
    /// Unavailable in this fixture. No measurement or clock change can be requested.
    CheckUnavailable,
    /// Open diagnostics in Power user
    OpenDiagnostics,
    /// Sources
    OpenSources,
    /// Manage background service
    ManageService,
    /// Review clock-correction availability
    OpenCorrectionReview,
    /// Sync now
    SyncNow,
    /// Check sources
    CheckSources,
    /// Open Status before using its contextual action.
    ActionRequired,
    /// Reported profile
    ProfileLabel,
}

impl StatusText {
    /// Stable proposed catalog key, independent of rendered wording.
    #[must_use]
    pub const fn catalog_id(self) -> &'static str {
        match self {
            Self::StatusUnknown => "ui.status.headline.unknown",
            Self::NoConnection => "ui.fixture.no-live-connection",
            Self::Initializing => "ui.status.headline.initializing",
            Self::Acquiring => "ui.status.headline.acquiring",
            Self::Observing => "ui.status.headline.observe-only",
            Self::Tracking => "ui.status.headline.tracking",
            Self::Holdover => "ui.status.headline.holdover",
            Self::Expired => "ui.status.headline.expired",
            Self::Conflict => "ui.status.headline.conflict",
            Self::Blocked => "ui.status.headline.blocked",
            Self::ServiceStopped => "ui.status.headline.service-stopped",
            Self::WindowsManaged => "ui.status.headline.windows-managed",
            Self::CorrectionPending => "ui.status.headline.correction-pending",
            Self::UncertainResult => "ui.status.headline.uncertain-result",
            Self::RateLimited => "ui.status.headline.rate-limited",
            Self::Reacquiring => "ui.status.headline.reacquiring",
            Self::LeapGuard => "ui.status.headline.leap-guard",
            Self::DateRecovery => "ui.status.headline.date-recovery",
            Self::UnavailableDetail => "ui.status.detail.unavailable",
            Self::WaitingDetail => "ui.status.detail.waiting",
            Self::ObserveDetail => "ui.policy.observe-consequence",
            Self::TrackingDetail => "ui.status.detail.tracking",
            Self::HoldoverDetail => "ui.status.detail.holdover",
            Self::WithheldDetail => "ui.status.detail.withheld",
            Self::StoppedDetail => "ui.status.detail.stopped",
            Self::WindowsDetail => "ui.status.detail.windows-managed",
            Self::ReviewDetail => "ui.status.detail.review",
            Self::UncertainDetail => "ui.status.detail.uncertain-result",
            Self::LimitedDetail => "ui.status.detail.rate-limited",
            Self::DateRecoveryDetail => "ui.status.detail.date-recovery",
            Self::Authenticated => "ui.status.evidence.authenticated",
            Self::HistoricalEvidence => "ui.status.evidence.historical",
            Self::Unauthenticated | Self::ProfileNtp => "ui.policy.ntp",
            Self::NotEstablished => "ui.fixture.authentication-value",
            Self::AgreementSatisfied => "ui.status.evidence.agreement-satisfied",
            Self::NotEvaluated => "ui.fixture.not-evaluated",
            Self::CurrentEstimate => "ui.status.evidence.current-estimate",
            Self::NotAvailable => "ui.fixture.unavailable",
            Self::ProfileUnknown => "ui.fixture.runtime-value",
            Self::ProfileNts => "ui.policy.nts",
            Self::ProfileObserveOnly => "ui.policy.observe",
            Self::CheckUnavailable => "ui.fixture.sync-unavailable",
            Self::OpenDiagnostics => "ui.shell.diagnose-power",
            Self::OpenSources => "ui.fixture.sources-tab",
            Self::ManageService => "ui.shell.manage-service",
            Self::OpenCorrectionReview => "ui.diagnostic.correction-open",
            Self::SyncNow => "ui.fixture.sync-now",
            Self::CheckSources => "ui.status.check.sources",
            Self::ActionRequired => "ui.status.action.context-required",
            Self::ProfileLabel => "ui.status.profile.label",
        }
    }

    /// Maintained initial English; production catalog approval is separate.
    #[must_use]
    pub const fn english(self) -> &'static str {
        match self {
            Self::StatusUnknown => "Status not observed",
            Self::NoConnection => "No live connection",
            Self::Initializing => "Starting and checking configuration",
            Self::Acquiring => "Checking time sources",
            Self::Observing => "Clock changes by NevroTick are disabled",
            Self::Tracking => "Time sync active",
            Self::Holdover => "No fresh time check - using the previous model",
            Self::Expired => "No usable current time estimate",
            Self::Conflict => "Time sources disagree",
            Self::Blocked => "Synchronization needs attention",
            Self::ServiceStopped => "Background services are stopped",
            Self::WindowsManaged => "Windows Time manages the clock",
            Self::CorrectionPending => "A clock correction needs separate review",
            Self::UncertainResult => "The operation result is unknown",
            Self::RateLimited => "Time checks are currently limited",
            Self::Reacquiring => "Rechecking after a change",
            Self::LeapGuard => "Rechecking around a UTC transition",
            Self::DateRecovery => "Recovering a provisional date",
            Self::UnavailableDetail => {
                "Current synchronization is unconfirmed. A missing status connection does not establish that services stopped. Inspect diagnostics for the available facts."
            }
            Self::WaitingDetail => {
                "Waiting for fresh admissible evidence. A running service or a reply alone does not establish synchronization."
            }
            Self::ObserveDetail => {
                "Observe only authorizes no product clock writes. It does not turn on Windows Time or settle the acquisition and authentication mode."
            }
            Self::TrackingDetail => {
                "The reported current policy is met. Authentication does not guarantee UTC accuracy; clock effects remain separate observations."
            }
            Self::HoldoverDetail => {
                "The owner reports the previous model is still usable. No fresh agreement is available; uncertainty and model limits remain important."
            }
            Self::WithheldDetail => {
                "This status grants no new clock correction. Review the current prerequisite and evidence before continuing."
            }
            Self::StoppedDetail => {
                "Stopped services cannot perform a fresh time check. Startup configuration, clock cleanup and Windows Time remain separate facts."
            }
            Self::WindowsDetail => {
                "NevroTick does not control the clock in this reported mode. Windows Time ownership does not establish verified synchronization."
            }
            Self::ReviewDetail => {
                "Review a current proposal separately. A check, opening this page or dismissing a notification cannot approve a clock change."
            }
            Self::UncertainDetail => {
                "Reconcile the actual result and clock ownership before repeating the operation. No outcome or cleanup is assumed."
            }
            Self::LimitedDetail => {
                "The reported request limit remains in force. Reopening this window does not reset it; no retry duration is invented here."
            }
            Self::DateRecoveryDetail => {
                "Provisional date recovery does not change Windows time or establish authenticated tracking. Full verification and fresh agreement are still required."
            }
            Self::Authenticated => "Current NTS authentication established",
            Self::HistoricalEvidence => "Historical evidence only - current state unconfirmed",
            Self::Unauthenticated | Self::ProfileNtp => "NTP - unauthenticated",
            Self::NotEstablished => "Not established",
            Self::AgreementSatisfied => "Current agreement policy met",
            Self::NotEvaluated => "Not evaluated",
            Self::CurrentEstimate => "Current qualified estimate - age not supplied",
            Self::NotAvailable => "Not available",
            Self::ProfileUnknown => "Not observed",
            Self::ProfileNts => "NTS only",
            Self::ProfileObserveOnly => "Observe only",
            Self::CheckUnavailable => {
                "Unavailable in this fixture. No measurement or clock change can be requested."
            }
            Self::OpenDiagnostics => "Open diagnostics in Power user",
            Self::OpenSources => "Sources",
            Self::ManageService => "Manage background service",
            Self::OpenCorrectionReview => "Review clock-correction availability",
            Self::SyncNow => "Sync now",
            Self::CheckSources => "Check sources",
            Self::ActionRequired => "Open Status before using its contextual action.",
            Self::ProfileLabel => "Reported profile",
        }
    }
}

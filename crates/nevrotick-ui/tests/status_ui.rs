//! Independent status input/wording checks; no backend or clock is used.
#![forbid(unsafe_code)]

use nevrotick_ui::shell::{Observation, SettingsSection, ShellRoute};
use nevrotick_ui::status_ui::{
    AgreementEvidence as Agreement, AuthenticationEvidence as Authentication,
    EstimateFreshness as Freshness, FixtureStatusPort, StatusCheckIntent, StatusCheckOutcome,
    StatusInputError, StatusKind, StatusPresentationPort, StatusProfile as Profile, StatusRemedy,
    StatusReport, StatusText, StatusView,
};

/// Independent complete expected condition/headline/destination table.
const STATUS_CASES: [StatusCase; 18] = [
    (
        StatusKind::Unknown,
        Profile::Unknown,
        Authentication::Unknown,
        Agreement::Unknown,
        Freshness::Unknown,
        StatusText::StatusUnknown,
        ShellRoute::Diagnostics,
    ),
    (
        StatusKind::Disconnected,
        Profile::Unknown,
        Authentication::Unknown,
        Agreement::Unknown,
        Freshness::Unknown,
        StatusText::NoConnection,
        ShellRoute::Diagnostics,
    ),
    (
        StatusKind::Initializing,
        Profile::Nts,
        Authentication::NotEstablished,
        Agreement::Waiting,
        Freshness::Unknown,
        StatusText::Initializing,
        ShellRoute::Diagnostics,
    ),
    (
        StatusKind::Acquiring,
        Profile::Nts,
        Authentication::CurrentNts,
        Agreement::Waiting,
        Freshness::Unknown,
        StatusText::Acquiring,
        ShellRoute::Diagnostics,
    ),
    (
        StatusKind::Tracking,
        Profile::Nts,
        Authentication::CurrentNts,
        Agreement::CurrentSatisfied,
        Freshness::Current,
        StatusText::Tracking,
        ShellRoute::Diagnostics,
    ),
    (
        StatusKind::Holdover,
        Profile::Nts,
        Authentication::HistoricalNts,
        Agreement::Historical,
        Freshness::RetainedUsable,
        StatusText::Holdover,
        ShellRoute::Diagnostics,
    ),
    (
        StatusKind::Expired,
        Profile::Nts,
        Authentication::HistoricalNts,
        Agreement::Historical,
        Freshness::Expired,
        StatusText::Expired,
        ShellRoute::Diagnostics,
    ),
    (
        StatusKind::Conflict,
        Profile::Nts,
        Authentication::CurrentNts,
        Agreement::Conflict,
        Freshness::Withheld,
        StatusText::Conflict,
        ShellRoute::Sources,
    ),
    (
        StatusKind::Blocked,
        Profile::Nts,
        Authentication::Unknown,
        Agreement::Waiting,
        Freshness::Withheld,
        StatusText::Blocked,
        ShellRoute::Diagnostics,
    ),
    (
        StatusKind::ServiceStopped,
        Profile::Nts,
        Authentication::HistoricalNts,
        Agreement::Historical,
        Freshness::Historical,
        StatusText::ServiceStopped,
        ShellRoute::Settings(SettingsSection::BackgroundService),
    ),
    (
        StatusKind::WindowsManaged,
        Profile::Unknown,
        Authentication::Unknown,
        Agreement::Unknown,
        Freshness::Unknown,
        StatusText::WindowsManaged,
        ShellRoute::Settings(SettingsSection::BackgroundService),
    ),
    (
        StatusKind::ObserveOnly,
        Profile::ObserveOnly,
        Authentication::Unknown,
        Agreement::Unknown,
        Freshness::Unknown,
        StatusText::Observing,
        ShellRoute::Diagnostics,
    ),
    (
        StatusKind::CorrectionPending,
        Profile::Nts,
        Authentication::CurrentNts,
        Agreement::CurrentSatisfied,
        Freshness::Current,
        StatusText::CorrectionPending,
        ShellRoute::CorrectionReview,
    ),
    (
        StatusKind::UncertainResult,
        Profile::Nts,
        Authentication::HistoricalNts,
        Agreement::Historical,
        Freshness::Withheld,
        StatusText::UncertainResult,
        ShellRoute::Diagnostics,
    ),
    (
        StatusKind::RateLimited,
        Profile::Nts,
        Authentication::Unknown,
        Agreement::Waiting,
        Freshness::Unknown,
        StatusText::RateLimited,
        ShellRoute::Diagnostics,
    ),
    (
        StatusKind::Reacquiring,
        Profile::Nts,
        Authentication::HistoricalNts,
        Agreement::Waiting,
        Freshness::Withheld,
        StatusText::Reacquiring,
        ShellRoute::Diagnostics,
    ),
    (
        StatusKind::LeapGuard,
        Profile::Nts,
        Authentication::HistoricalNts,
        Agreement::Waiting,
        Freshness::Withheld,
        StatusText::LeapGuard,
        ShellRoute::Diagnostics,
    ),
    (
        StatusKind::DateRecovery,
        Profile::Nts,
        Authentication::NotEstablished,
        Agreement::Waiting,
        Freshness::Withheld,
        StatusText::DateRecovery,
        ShellRoute::Diagnostics,
    ),
];

/// Seven independent presentation facts and their expected safe destination.
type StatusCase = (
    StatusKind,
    Profile,
    Authentication,
    Agreement,
    Freshness,
    StatusText,
    ShellRoute,
);

/// Legal current-profile authentication pairs, including independent observe-only.
const QUALIFIED_AUTHENTICATION: [(Profile, Authentication); 4] = [
    (Profile::Nts, Authentication::CurrentNts),
    (Profile::PlainNtp, Authentication::Unauthenticated),
    (Profile::ObserveOnly, Authentication::CurrentNts),
    (Profile::ObserveOnly, Authentication::Unauthenticated),
];

/// Missing/contradictory current evidence, with one deliberately bad axis per row.
const REJECTED_TRACKING: [(Profile, Authentication, Agreement, Freshness); 8] = [
    (
        Profile::Unknown,
        Authentication::CurrentNts,
        Agreement::CurrentSatisfied,
        Freshness::Current,
    ),
    (
        Profile::Nts,
        Authentication::Unknown,
        Agreement::CurrentSatisfied,
        Freshness::Current,
    ),
    (
        Profile::Nts,
        Authentication::Unauthenticated,
        Agreement::CurrentSatisfied,
        Freshness::Current,
    ),
    (
        Profile::PlainNtp,
        Authentication::CurrentNts,
        Agreement::CurrentSatisfied,
        Freshness::Current,
    ),
    (
        Profile::Nts,
        Authentication::CurrentNts,
        Agreement::Waiting,
        Freshness::Current,
    ),
    (
        Profile::Nts,
        Authentication::CurrentNts,
        Agreement::CurrentSatisfied,
        Freshness::Historical,
    ),
    (
        Profile::Nts,
        Authentication::HistoricalNts,
        Agreement::CurrentSatisfied,
        Freshness::Current,
    ),
    (
        Profile::ObserveOnly,
        Authentication::Unknown,
        Agreement::CurrentSatisfied,
        Freshness::Current,
    ),
];

/// The outer observation owns snapshot currency; useful axes cannot infer Tracking.
///
/// # Errors
/// Propagates a valid neighboring report's unexpected admission rejection.
///
/// # Panics
/// Fails if missing/incompatible qualified fields can produce current Tracking.
#[test]
fn tracking_requires_explicit_current_profile_compatible_evidence() -> Result<(), StatusInputError>
{
    for (profile, auth) in QUALIFIED_AUTHENTICATION {
        let report = StatusReport::new(
            StatusKind::Tracking,
            profile,
            auth,
            Agreement::CurrentSatisfied,
            Freshness::Current,
        )?;
        let current = StatusView::from_observation(Observation::Current(report));
        assert_eq!(current.kind(), StatusKind::Tracking);
        assert!(current.has_current_report());
        assert_eq!(current.agreement(), StatusText::AgreementSatisfied);
        assert_eq!(current.freshness(), StatusText::CurrentEstimate);
        if profile == Profile::ObserveOnly {
            assert_eq!(current.headline(), StatusText::Observing);
            assert_eq!(current.check_intent(), StatusCheckIntent::CheckSources);
        } else {
            assert_eq!(current.headline(), StatusText::Tracking);
        }
    }
    for (profile, auth, agreement, freshness) in REJECTED_TRACKING {
        assert_eq!(
            StatusReport::new(StatusKind::Tracking, profile, auth, agreement, freshness),
            Err(StatusInputError::UnqualifiedTracking)
        );
    }
    // Merely receiving/authenticating enough individual responses cannot select Tracking.
    let acquiring = StatusReport::new(
        StatusKind::Acquiring,
        Profile::Nts,
        Authentication::CurrentNts,
        Agreement::CurrentSatisfied,
        Freshness::Current,
    )?;
    let view = StatusView::from_observation(Observation::Current(acquiring));
    assert_eq!(view.kind(), StatusKind::Acquiring);
    assert_eq!(view.headline(), StatusText::Acquiring);
    Ok(())
}

/// Retained bytes cannot refresh authentication, agreement or model usability.
///
/// # Errors
/// Propagates unexpected rejection of the explicitly current reference report.
///
/// # Panics
/// Fails when stale Tracking is displayed as a currently qualified positive claim.
#[test]
fn historical_snapshot_suppresses_all_current_positive_claims() -> Result<(), StatusInputError> {
    let report = StatusReport::new(
        StatusKind::Tracking,
        Profile::Nts,
        Authentication::CurrentNts,
        Agreement::CurrentSatisfied,
        Freshness::Current,
    )?;
    let historical = StatusView::from_observation(Observation::Stale(report));
    assert_eq!(historical.kind(), StatusKind::Disconnected);
    assert!(!historical.has_current_report());
    assert_eq!(historical.profile(), Profile::Nts);
    assert_eq!(historical.headline(), StatusText::NoConnection);
    assert_eq!(historical.consequence(), StatusText::HistoricalEvidence);
    assert_eq!(historical.authentication(), StatusText::HistoricalEvidence);
    assert_eq!(historical.agreement(), StatusText::HistoricalEvidence);
    assert_eq!(historical.freshness(), StatusText::HistoricalEvidence);
    assert_eq!(historical.remedy(), StatusRemedy::Diagnostics);
    assert_eq!(historical.check_intent(), StatusCheckIntent::CheckSources);
    // Reading repeatedly is inert: no wall-clock measurement or freshness update.
    for _ in 0..64 {
        assert_eq!(
            StatusView::from_observation(Observation::Stale(report)),
            historical
        );
    }
    let unknown = StatusView::from_observation(Observation::Unknown);
    assert_eq!(unknown.kind(), StatusKind::Unknown);
    assert!(!unknown.has_current_report());
    assert_eq!(unknown.authentication(), StatusText::NotEstablished);
    assert_eq!(unknown.agreement(), StatusText::NotEvaluated);
    assert_eq!(unknown.freshness(), StatusText::NotAvailable);
    Ok(())
}

/// Profile is independent of availability, retained estimate and Windows ownership.
///
/// # Errors
/// Propagates a legal observe-only profile combination's unexpected rejection.
///
/// # Panics
/// Fails if observe-only implies a running service, Windows Time, NTS or a clock write.
#[test]
fn observe_only_coexists_with_disconnected_holdover_and_unconfirmed_authentication()
-> Result<(), StatusInputError> {
    for (kind, freshness, headline) in [
        (
            StatusKind::Disconnected,
            Freshness::Unknown,
            StatusText::NoConnection,
        ),
        (
            StatusKind::Holdover,
            Freshness::RetainedUsable,
            StatusText::Holdover,
        ),
        (
            StatusKind::ObserveOnly,
            Freshness::Unknown,
            StatusText::Observing,
        ),
        (
            StatusKind::ServiceStopped,
            Freshness::Historical,
            StatusText::ServiceStopped,
        ),
    ] {
        let report = StatusReport::new(
            kind,
            Profile::ObserveOnly,
            Authentication::Unknown,
            Agreement::Unknown,
            freshness,
        )?;
        let view = StatusView::from_observation(Observation::Current(report));
        assert_eq!(view.profile(), Profile::ObserveOnly);
        assert_eq!(view.headline(), headline);
        assert_ne!(view.headline(), StatusText::WindowsManaged);
        assert_ne!(view.authentication(), StatusText::Authenticated);
        assert_eq!(view.check_intent(), StatusCheckIntent::CheckSources);
    }
    assert_eq!(
        StatusReport::new(
            StatusKind::ObserveOnly,
            Profile::Nts,
            Authentication::Unknown,
            Agreement::Unknown,
            Freshness::Unknown
        ),
        Err(StatusInputError::InvalidObserveOnly)
    );
    Ok(())
}

/// The model admits declared usability and rejects contradictory positive claims.
///
/// # Errors
/// Propagates legal neighboring report rejection.
///
/// # Panics
/// Fails if Holdover/Expired/Conflict/uncertain results manufacture usable evidence.
#[test]
fn retained_expired_conflicting_and_uncertain_conditions_keep_their_own_rules()
-> Result<(), StatusInputError> {
    for (kind, freshness, agreement, expected) in [
        (
            StatusKind::Holdover,
            Freshness::Expired,
            Agreement::Historical,
            StatusInputError::InvalidHoldover,
        ),
        (
            StatusKind::Holdover,
            Freshness::RetainedUsable,
            Agreement::CurrentSatisfied,
            StatusInputError::InvalidHoldover,
        ),
        (
            StatusKind::Expired,
            Freshness::Current,
            Agreement::Historical,
            StatusInputError::InvalidExpired,
        ),
        (
            StatusKind::Conflict,
            Freshness::Withheld,
            Agreement::CurrentSatisfied,
            StatusInputError::InvalidConflict,
        ),
        (
            StatusKind::UncertainResult,
            Freshness::Current,
            Agreement::Historical,
            StatusInputError::InvalidUncertainResult,
        ),
    ] {
        assert_eq!(
            StatusReport::new(
                kind,
                Profile::Nts,
                Authentication::HistoricalNts,
                agreement,
                freshness
            ),
            Err(expected)
        );
    }
    let holdover = StatusReport::new(
        StatusKind::Holdover,
        Profile::Nts,
        Authentication::HistoricalNts,
        Agreement::Historical,
        Freshness::RetainedUsable,
    )?;
    let expired = StatusReport::new(
        StatusKind::Expired,
        Profile::Nts,
        Authentication::HistoricalNts,
        Agreement::Historical,
        Freshness::Expired,
    )?;
    assert_eq!(
        StatusView::from_observation(Observation::Current(holdover)).freshness(),
        StatusText::Holdover
    );
    assert_eq!(
        StatusView::from_observation(Observation::Current(expired)).freshness(),
        StatusText::Expired
    );
    Ok(())
}

/// Every requested status has a deliberate headline and contextual safe destination.
///
/// # Errors
/// Propagates a legal explicit presentation input's unexpected rejection.
///
/// # Panics
/// Fails when a case aliases a false healthy state or navigates to an effect.
#[test]
fn all_status_conditions_have_explicit_presentation_and_contextual_routes()
-> Result<(), StatusInputError> {
    for (kind, profile, auth, agreement, freshness, headline, route) in STATUS_CASES {
        let report = StatusReport::new(kind, profile, auth, agreement, freshness)?;
        let view = StatusView::from_observation(Observation::Current(report));
        assert_eq!(view.kind(), kind);
        assert_eq!(view.headline(), headline);
        assert_eq!(view.remedy().route(), route);
        assert!(!view.headline().english().contains("Time is correct"));
        assert_ne!(view.consequence().english(), "");
    }
    Ok(())
}

/// The reached input port and directly invoked checks remain inert/unavailable.
///
/// # Panics
/// Fails if reading or requesting can establish tracking, a service stop or submission.
#[test]
fn fixture_port_never_submits_measurements_or_conflates_disconnection_with_stop() {
    let port = FixtureStatusPort;
    let before = port.observe();
    let view = StatusView::from_observation(before);
    assert_eq!(view.kind(), StatusKind::Disconnected);
    assert_eq!(view.profile(), Profile::Unknown);
    assert_eq!(view.remedy(), StatusRemedy::Diagnostics);
    for intent in [StatusCheckIntent::SyncNow, StatusCheckIntent::CheckSources] {
        assert_eq!(
            port.request_check(intent),
            StatusCheckOutcome::AdapterUnavailable
        );
        assert_eq!(port.observe(), before);
    }
    assert_eq!(view.headline(), StatusText::NoConnection);
    assert_ne!(view.headline(), StatusText::ServiceStopped);
}

/// Status and tray share a typed context; neither can manufacture a check effect.
///
/// # Errors
/// Propagates unexpected rejection of independently maintained complete cases.
///
/// # Panics
/// Fails if stopped services allow checks or tray wording contradicts Status.
#[test]
fn every_status_maps_to_the_same_honest_tray_measurement_context() -> Result<(), StatusInputError> {
    use nevrotick_ui::shell::{ShellRequest, TrayMenuAction};
    use nevrotick_ui::tray_ui::{TrayInteraction, TrayLabel};
    for (kind, profile, auth, agreement, freshness, _, _) in STATUS_CASES {
        let report = StatusReport::new(kind, profile, auth, agreement, freshness)?;
        let view = StatusView::from_observation(Observation::Current(report));
        let menu = view.tray_menu();
        let entry = menu.entries(TrayInteraction::Free)[1];
        if kind == StatusKind::ServiceStopped {
            assert_eq!(entry.label(), TrayLabel::ManageBackgroundService);
            assert!(entry.enabled());
            assert_eq!(
                menu.request(1, TrayInteraction::Free),
                Ok(ShellRequest::Menu(TrayMenuAction::TimeManagement))
            );
        } else {
            assert!(!entry.enabled());
            assert!(menu.request(1, TrayInteraction::Free).is_err());
            let label = match view.check_intent() {
                StatusCheckIntent::SyncNow => TrayLabel::SyncNow,
                StatusCheckIntent::CheckSources => TrayLabel::CheckSources,
            };
            assert_eq!(entry.label(), label);
        }
    }
    Ok(())
}

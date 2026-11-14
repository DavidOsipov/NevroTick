// SPDX-License-Identifier: GPL-3.0-only
//! Bounded local-toast preparation and session correlation, without native effects.
//!
//! A native activation is only an intention to navigate. The UI thread must feed
//! its private current dialog/exit guard and fresh notice to `resolve_activation`,
//! then route the returned intention through its existing sole shell/recovery
//! owner. No text, identity, token or successful OS submission grants service,
//! clock, source or policy authority. Production incident admission remains with
//! the future coherent backend owner; this candidate sends explicit tests only.
#![forbid(unsafe_code)]

use std::{fmt, num::NonZeroU128};

use crate::localization::{self, MessageId, Snapshot};
use crate::notifications_ui::{FixtureNotice, FixtureNoticeContext};
use crate::shell::ShellRoute;
#[cfg(any(windows, test))]
use crate::{
    notifications_ui::{NotificationBlock, ObserverPresence},
    preferences::UiPreferences,
    shell::Observation,
    tray_ui::TrayInteraction,
};

/// UTF-16 budget of the single title text node.
pub const TITLE_UNITS: usize = 128;
/// UTF-16 budget of the single body text node.
pub const BODY_UNITS: usize = 512;
/// UTF-16 budget of the sole navigation action label.
pub const ACTION_UNITS: usize = 64;
/// Exact ASCII/UTF-16 length of the versioned activation token.
pub const TOKEN_UNITS: usize = 54;
/// Two new reviewed text IDs; neither can be supplied by translator data.
const TEST_TITLE: &str = "ui.native-notification.test-title";
/// Fixed-body test copy, never synthesized from actual clock/service observations.
const TEST_BODY: &str = "ui.native-notification.test-body";
/// Existing canonical label for navigation to notification preferences.
const SETTINGS_LABEL: &str = "ui.shell.notifications";

/// Fixed, redacted preparation rejection, with no rejected native/contributor text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NoticeError {
    /// A title/body/action exceeds its original UTF-8 or native UTF-16 budget.
    TextBounds,
    /// A required field has no visible admitted content.
    EmptyText,
    /// A field violates the shared selected literal repertoire.
    Character,
    /// The selected catalog is missing a required native text or cannot render it.
    Catalog(localization::Error),
    /// An inert sample context cannot be admitted as a production problem event.
    LiveIncidentOwnerRequired,
}

impl fmt::Display for NoticeError {
    /// Render only a fixed technical diagnostic; GUI wording has the catalog owner.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("The local notification could not be prepared.")
    }
}

impl std::error::Error for NoticeError {}

/// One immutable prepared toast; callers cannot substitute text, URI or command.
///
/// Every string is selected from one immutable admitted snapshot, then bounded
/// for its actual native sink. A language change does not rewrite old OS content.
pub struct PreparedTestNotice {
    /// Checked sample-only context/identity, never a live incident claim.
    notice: FixtureNotice,
    /// Bounded plain title, not XML.
    title: String,
    /// Bounded plain body, not XML.
    body: String,
    /// Bounded localized label for the one fixed foreground navigation action.
    action: String,
}

impl fmt::Debug for PreparedTestNotice {
    /// Do not retain or reflect translated content in incidental diagnostics.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PreparedTestNotice(<redacted>)")
    }
}

impl PreparedTestNotice {
    /// Prepare all fields from one snapshot; no partial native operation occurs.
    ///
    /// # Errors
    /// Rejects non-test sample contexts, missing reviewed IDs, invalid characters,
    /// empty content or any field's native capacity before retaining its String.
    pub fn from_snapshot(
        snapshot: &Snapshot<'_>,
        notice: FixtureNotice,
    ) -> Result<Self, NoticeError> {
        if notice.context() != FixtureNoticeContext::TestExample {
            return Err(NoticeError::LiveIncidentOwnerRequired);
        }
        let render = |key| {
            snapshot
                .text(MessageId::from_key(key).map_err(NoticeError::Catalog)?)
                .map_err(NoticeError::Catalog)
        };
        let title = render(TEST_TITLE)?;
        let body = render(TEST_BODY)?;
        let action = render(SETTINGS_LABEL)?;
        Self::checked_parts(notice, title.as_str(), body.as_str(), action.as_str())
    }

    /// Prepare the exact labelled engineering example, without catalog approval.
    ///
    /// This source-level fixture API creates no native effect or catalog authority.
    /// It is for an explicitly labelled maintained sample-mode host. These
    /// fixed words are separate critical bootstrap review material, not an
    /// English fallback for an incomplete or rejected external catalog.
    ///
    /// # Errors
    /// Rejects a non-test context or any changed fixed word violating sink bounds.
    pub fn engineering_example(notice: FixtureNotice) -> Result<Self, NoticeError> {
        Self::checked_parts(
            notice,
            "NevroTick test notification",
            "This is an example. No clock or service status was checked.",
            "Notifications",
        )
    }

    /// Bound original data before each private allocation; preserve exact bytes.
    ///
    /// # Errors
    /// Rejects any content or sink-budget failure without retaining a partial notice.
    fn checked_parts(
        notice: FixtureNotice,
        title: &str,
        body: &str,
        action: &str,
    ) -> Result<Self, NoticeError> {
        if notice.context() != FixtureNoticeContext::TestExample {
            return Err(NoticeError::LiveIncidentOwnerRequired);
        }
        check_text(title, TITLE_UNITS)?;
        check_text(body, BODY_UNITS)?;
        check_text(action, ACTION_UNITS)?;
        Ok(Self {
            notice,
            title: title.to_owned(),
            body: body.to_owned(),
            action: action.to_owned(),
        })
    }

    /// Borrow the exact bounded literal title without XML interpretation.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Borrow the exact bounded literal body without XML interpretation.
    #[must_use]
    pub fn body(&self) -> &str {
        &self.body
    }

    /// Borrow only display text for the fixed navigation action.
    #[must_use]
    pub fn action_label(&self) -> &str {
        &self.action
    }

    /// Return only the immutable sample identity/context, not delivery evidence.
    #[must_use]
    pub const fn notice(&self) -> FixtureNotice {
        self.notice
    }
}

/// Check one already-rendered literal against the tighter native field boundary.
///
/// # Errors
/// Rejects original length before scalar work, shared character failure and blank
/// content; there is no truncation, normalization, lossy encoding or XML parsing.
fn check_text(raw: &str, units: usize) -> Result<(), NoticeError> {
    if raw.len() > localization::OUTPUT_BYTES || raw.encode_utf16().count() > units {
        return Err(NoticeError::TextBounds);
    }
    if !raw.bytes().any(|byte| byte != b' ') {
        return Err(NoticeError::EmptyText);
    }
    localization::check_fixture_literal(raw).map_err(|_| NoticeError::Character)?;
    Ok(())
}

/// Per-adapter opaque native correlation, never identity/authorization evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SessionNonce(NonZeroU128);

impl SessionNonce {
    /// Admit the OS-generated correlation value; zero rejects without replacement.
    pub(crate) fn new(raw: u128) -> Option<Self> {
        NonZeroU128::new(raw).map(Self)
    }
}

/// Fixed-shape OS activation data, containing no route, URI, file or command.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActivationToken {
    /// Exact current observer instance correlation; not a bearer capability.
    session: SessionNonce,
    /// Nonzero sample ID chosen by the bounded caller-side sequence owner.
    notice_id: u64,
}

impl ActivationToken {
    /// Bind only a checked native session and an existing nonzero notice identity.
    #[cfg(any(windows, test))]
    pub(crate) fn new(session: SessionNonce, notice: FixtureNotice) -> Self {
        Self {
            session,
            notice_id: notice.id(),
        }
    }

    /// Serialize one exact code-owned format, never interpolated XML.
    #[cfg(any(windows, test))]
    pub(crate) fn encode(self) -> String {
        format!("ntn1.{:032x}.{:016x}", self.session.0.get(), self.notice_id)
    }

    /// Decode hostile native HSTRING content without copying its original buffer.
    ///
    /// # Errors
    /// Wrong length/version/separators/case, non-ASCII, zero and overflow reject.
    /// Exactly54 units are examined; no route or action comes from the token.
    pub fn from_utf16(raw: &[u16]) -> Result<Self, ActivationError> {
        if raw.len() != TOKEN_UNITS {
            return Err(ActivationError);
        }
        for (index, expected) in b"ntn1.".iter().enumerate() {
            if raw.get(index).copied() != Some(u16::from(*expected)) {
                return Err(ActivationError);
            }
        }
        if raw.get(37).copied() != Some(u16::from(b'.')) {
            return Err(ActivationError);
        }
        let session = parse_hex(raw.get(5..37).ok_or(ActivationError)?)?;
        let notice = parse_hex(raw.get(38..54).ok_or(ActivationError)?)?;
        let session = SessionNonce::new(session).ok_or(ActivationError)?;
        let notice_id = u64::try_from(notice).map_err(|_| ActivationError)?;
        if notice_id == 0 {
            return Err(ActivationError);
        }
        Ok(Self { session, notice_id })
    }
}

/// Fixed activation rejection; original bytes are never reflected or retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActivationError;

/// Convert a fixed slice of lowercase ASCII hex with checked arithmetic.
///
/// # Errors
/// Nonhexadecimal units and overflow reject before a decoded token is returned.
fn parse_hex(raw: &[u16]) -> Result<u128, ActivationError> {
    raw.iter().try_fold(0_u128, |value, unit| {
        let nibble = match *unit {
            48..=57 => u128::from(*unit - 48),
            97..=102 => u128::from(*unit - 87),
            _ => return Err(ActivationError),
        };
        value
            .checked_mul(16)
            .and_then(|value| value.checked_add(nibble))
            .ok_or(ActivationError)
    })
}

/// A narrow native attempt's result, never banner/read/delivery/health evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Submission {
    /// The OS returned success from Show; a banner may still be suppressed.
    AcceptedByOs,
    /// Show was attempted but its actual acceptance is uncertain; do not retry it.
    Indeterminate,
}

/// Current navigation disposition; caller still uses the sole shell controller.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActivationResolution {
    /// Existing dialog owns this UI; focus it without dismissing or navigating.
    FocusOwnedDialog,
    /// Exit/observer stop forbids any new navigation.
    Ignored,
    /// Only a typed current destination is proposed; this grants no other action.
    Navigate(ShellRoute),
}

/// One bounded correlation/coalescing owner, independent of the shell controller.
#[cfg(any(windows, test))]
pub(crate) struct DeliverySession {
    /// This native observer instance; never a persistent incident generation.
    nonce: SessionNonce,
    /// Highest attempted nonzero sample ID, retained across preference toggles.
    high_water: u64,
    /// Latest current sample actually attempted, plus acceptance certainty.
    latest: Option<(FixtureNotice, Submission)>,
    /// One-shot consumption of this current activation.
    activated: bool,
    /// Explicit close denies further requests before any native cleanup starts.
    pub(crate) stopped: bool,
}

/// One exclusive notice/session admission, without a native attempt yet.
///
/// Dropping preparation leaves its ID reusable. The only consuming operation
/// marks the exact bound notice spent before invoking its one Show callback.
#[cfg(any(windows, test))]
#[must_use]
pub(crate) struct DeliveryAttempt<'session, 'notice> {
    /// Exclusive session custody prevents replacement, regression or a second admission.
    session: &'session mut DeliverySession,
    /// Immutable admitted fields and identity used by native preparation.
    prepared: &'notice PreparedTestNotice,
}

#[cfg(any(windows, test))]
impl DeliveryAttempt<'_, '_> {
    /// Prepare native fields only from the same immutable admitted notice.
    #[must_use]
    pub(crate) const fn prepared(&self) -> &PreparedTestNotice {
        self.prepared
    }

    /// Derive correlation from this notice and this session, without caller labels.
    #[must_use]
    pub(crate) fn activation_token(&self) -> ActivationToken {
        ActivationToken::new(self.session.nonce, self.prepared.notice)
    }

    /// Consume one actual Show callback, including an uncertain return or unwind.
    ///
    /// All fallible preparation belongs before this boundary. The callback is
    /// invoked once; its caller must place only Show and outcome mapping inside.
    /// Before invocation, the bound ID is spent and activation remains uncertain.
    /// No independent notice/session can be supplied during completion.
    ///
    /// # Panics
    /// Propagates a callback unwind while retaining the spent ID and uncertain result.
    pub(crate) fn attempt_show(self, show: impl FnOnce() -> Submission) -> Submission {
        let notice = self.prepared.notice;
        self.session.high_water = notice.id();
        self.session.latest = Some((notice, Submission::Indeterminate));
        self.session.activated = false;
        let submission = show();
        self.session.latest = Some((notice, submission));
        submission
    }
}

#[cfg(any(windows, test))]
impl DeliverySession {
    /// Start an empty coalescing owner from its separately observed native nonce.
    pub(crate) fn new(nonce: SessionNonce) -> Self {
        Self {
            nonce,
            high_water: 0,
            latest: None,
            activated: false,
            stopped: false,
        }
    }

    /// Bind current accepted presentation facts to exclusive native preparation.
    ///
    /// # Errors
    /// Stopped/disabled/stale observers reject; duplicate or old IDs coalesce.
    pub(crate) fn admit<'notice>(
        &mut self,
        prepared: &'notice PreparedTestNotice,
        preferences: &UiPreferences,
        observer: Observation<ObserverPresence>,
    ) -> Result<Option<DeliveryAttempt<'_, 'notice>>, NotificationBlock> {
        if self.stopped || !matches!(observer, Observation::Current(ObserverPresence::Running)) {
            return Err(NotificationBlock::ObserverUnavailable);
        }
        if !preferences.notifications_enabled() {
            return Err(NotificationBlock::PreferenceOff);
        }
        if prepared.notice.id() <= self.high_water {
            return Ok(None);
        }
        Ok(Some(DeliveryAttempt {
            session: self,
            prepared,
        }))
    }

    /// Revalidate identity, current context and the real UI's modal/exit guard.
    pub(crate) fn resolve_activation(
        &mut self,
        token: Result<ActivationToken, ActivationError>,
        current: Observation<FixtureNotice>,
        interaction: TrayInteraction,
        observer_usable: bool,
    ) -> ActivationResolution {
        if self.stopped || interaction == TrayInteraction::ExitRequested {
            return ActivationResolution::Ignored;
        }
        if interaction == TrayInteraction::DialogOwned {
            return ActivationResolution::FocusOwnedDialog;
        }
        if !observer_usable {
            return ActivationResolution::Navigate(ShellRoute::Status);
        }
        if let (Ok(token), Some((notice, Submission::AcceptedByOs)), Observation::Current(current)) =
            (token, self.latest, current)
            && token.session == self.nonce
            && token.notice_id == notice.id()
            && current == notice
            && !self.activated
        {
            self.activated = true;
            return ActivationResolution::Navigate(notice.context.route());
        }
        ActivationResolution::Navigate(ShellRoute::Status)
    }
}

/// Pure healthy/adversarial controls; these do not execute `WinRT`, XML or UI effects.
#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    /// Supply one ordinary checked sample, without fabricating live incident state.
    ///
    /// # Errors
    /// Rejects a zero fixture identity.
    fn notice(id: u64) -> Result<FixtureNotice, Box<dyn Error>> {
        FixtureNotice::new(id, FixtureNoticeContext::TestExample)
            .ok_or_else(|| "nonzero fixture".into())
    }

    /// Supply a checked deterministic pure correlation, not an OS-observation claim.
    ///
    /// # Errors
    /// Rejects zero correlation.
    fn session() -> Result<DeliverySession, Box<dyn Error>> {
        Ok(DeliverySession::new(
            SessionNonce::new(1).ok_or("nonzero correlation")?,
        ))
    }

    /// Construct one bounded plain-text fixture through the actual private admission.
    ///
    /// # Errors
    /// Propagates fixture or text admission failure.
    fn prepared(id: u64) -> Result<PreparedTestNotice, Box<dyn Error>> {
        Ok(PreparedTestNotice::engineering_example(notice(id)?)?)
    }

    /// Admit actual checked test data under explicit enabled/current fixture facts.
    ///
    /// # Errors
    /// Preserve unexpected blocking or coalescing instead of fabricating a permit.
    fn admitted<'session, 'notice>(
        state: &'session mut DeliverySession,
        value: &'notice PreparedTestNotice,
    ) -> Result<DeliveryAttempt<'session, 'notice>, Box<dyn Error>> {
        let mut preferences = UiPreferences::default();
        preferences.set_notifications_enabled(true);
        state
            .admit(
                value,
                &preferences,
                Observation::Current(ObserverPresence::Running),
            )
            .map_err(NotificationBlock::text)?
            .ok_or_else(|| "new fixture notice unexpectedly coalesced".into())
    }

    /// Markup-looking text stays literal data; no parser/action field is present.
    ///
    /// # Errors
    /// Propagates unexpected fixture admission failure.
    ///
    /// # Panics
    /// Fails if literal content is changed or an inappropriate context is admitted.
    #[test]
    fn literal_fields_preserve_exact_bytes_and_refuse_live_problem_promotion()
    -> Result<(), Box<dyn Error>> {
        let text = "x</text><actions>&\" + %n {token}";
        let value = PreparedTestNotice::checked_parts(notice(1)?, text, text, "<&>\"")?;
        assert_eq!(value.title(), text);
        assert_eq!(value.body(), text);
        assert_eq!(value.action_label(), "<&>\"");
        let live = FixtureNotice::new(1, FixtureNoticeContext::ServiceNeedsReview)
            .ok_or("nonzero fixture")?;
        assert!(matches!(
            PreparedTestNotice::checked_parts(live, text, text, "Open"),
            Err(NoticeError::LiveIncidentOwnerRequired)
        ));
        Ok(())
    }

    /// Original-byte, UTF-16, blank and selected-repertoire checks are mandatory.
    ///
    /// # Errors
    /// Propagates unexpected fixture admission failure.
    ///
    /// # Panics
    /// Fails if a neighboring capacity is lost or an invalid field is retained.
    #[test]
    fn exact_native_field_budgets_and_hostile_payloads() -> Result<(), Box<dyn Error>> {
        let id = notice(1)?;
        assert!(
            PreparedTestNotice::checked_parts(
                id,
                &"a".repeat(128),
                &"a".repeat(512),
                &"a".repeat(64)
            )
            .is_ok()
        );
        assert!(matches!(
            PreparedTestNotice::checked_parts(id, &"a".repeat(129), "Body", "Open"),
            Err(NoticeError::TextBounds)
        ));
        assert!(matches!(
            PreparedTestNotice::checked_parts(id, "Title", &"a".repeat(513), "Open"),
            Err(NoticeError::TextBounds)
        ));
        assert!(matches!(
            PreparedTestNotice::checked_parts(id, "Title", "Body", &"a".repeat(65)),
            Err(NoticeError::TextBounds)
        ));
        for invalid in ["", "   "] {
            assert!(matches!(
                PreparedTestNotice::checked_parts(id, invalid, "Body", "Open"),
                Err(NoticeError::EmptyText)
            ));
        }
        for invalid in ["nul\0", "line\n", "\u{202e}", "\u{1f642}"] {
            assert!(PreparedTestNotice::checked_parts(id, "Title", invalid, "Open").is_err());
        }
        // Native UTF-16 and shared literal UTF-8 bounds apply independently.
        let exact_bytes = "·".repeat(512);
        assert_eq!(exact_bytes.len(), 1_024);
        assert!(PreparedTestNotice::checked_parts(id, "Title", &exact_bytes, "Open").is_ok());
        let over_bytes = format!("{}—", "·".repeat(511));
        assert_eq!(over_bytes.len(), 1_025);
        assert_eq!(over_bytes.encode_utf16().count(), 512);
        assert!(matches!(
            PreparedTestNotice::checked_parts(id, "Title", &over_bytes, "Open"),
            Err(NoticeError::Character)
        ));
        let over_units = format!("{}aa", "·".repeat(511));
        assert_eq!(over_units.len(), 1_024);
        assert_eq!(over_units.encode_utf16().count(), 513);
        assert!(matches!(
            PreparedTestNotice::checked_parts(id, "Title", &over_units, "Open"),
            Err(NoticeError::TextBounds)
        ));
        assert!(matches!(
            PreparedTestNotice::checked_parts(id, "Title", &"—".repeat(512), "Open"),
            Err(NoticeError::Character)
        ));
        Ok(())
    }

    /// Tokens have exact lowercase ASCII structure even at both integer maxima.
    ///
    /// # Errors
    /// Propagates unexpected checked fixture rejection.
    ///
    /// # Panics
    /// Fails if encoding/decoding changes bytes or an adversarial token is admitted.
    #[test]
    fn token_decoder_is_exact_bounded_and_noninterpreting() -> Result<(), Box<dyn Error>> {
        let token = ActivationToken::new(
            SessionNonce::new(u128::MAX).ok_or("nonzero")?,
            notice(u64::MAX)?,
        );
        let encoded = token.encode();
        assert_eq!(encoded.len(), TOKEN_UNITS);
        let units: Vec<u16> = encoded.encode_utf16().collect();
        assert_eq!(ActivationToken::from_utf16(&units), Ok(token));
        for index in [0, 4, 5, 37, 38, 53] {
            let mut changed = units.clone();
            changed[index] = 0;
            assert_eq!(ActivationToken::from_utf16(&changed), Err(ActivationError));
        }
        let mut changed = units.clone();
        changed[10] = 0xd800;
        assert_eq!(ActivationToken::from_utf16(&changed), Err(ActivationError));
        let uppercase: Vec<u16> = encoded.to_uppercase().encode_utf16().collect();
        assert_eq!(
            ActivationToken::from_utf16(&uppercase),
            Err(ActivationError)
        );
        for raw in [Vec::new(), vec![48; 53], vec![48; 55], vec![48; 65_536]] {
            assert_eq!(ActivationToken::from_utf16(&raw), Err(ActivationError));
        }
        for raw in [
            "ntn1.00000000000000000000000000000000.0000000000000001",
            "ntn1.00000000000000000000000000000001.0000000000000000",
        ] {
            assert_eq!(
                ActivationToken::from_utf16(&raw.encode_utf16().collect::<Vec<_>>()),
                Err(ActivationError)
            );
        }
        Ok(())
    }

    /// Accepted preferences/current observer are independent and do not reset IDs.
    ///
    /// # Errors
    /// Propagates unexpected checked fixture rejection.
    ///
    /// # Panics
    /// Fails if toggles or stale observers consume/rearm an alert.
    #[test]
    fn coalescing_survives_toggles_and_uncertain_native_attempts() -> Result<(), Box<dyn Error>> {
        let mut state = session()?;
        let value = prepared(7)?;
        let mut preferences = UiPreferences::default();
        let running = Observation::Current(ObserverPresence::Running);
        preferences.set_notifications_enabled(false);
        assert_eq!(
            state.admit(&value, &preferences, running).err(),
            Some(NotificationBlock::PreferenceOff)
        );
        assert_eq!(state.high_water, 0);
        preferences.set_notifications_enabled(true);
        for unavailable in [
            Observation::Unknown,
            Observation::Stale(ObserverPresence::Running),
            Observation::Current(ObserverPresence::Stopped),
        ] {
            assert_eq!(
                state.admit(&value, &preferences, unavailable).err(),
                Some(NotificationBlock::ObserverUnavailable)
            );
        }
        let attempt = state
            .admit(&value, &preferences, running)
            .map_err(NotificationBlock::text)?
            .ok_or("new notice unexpectedly coalesced")?;
        assert_eq!(
            attempt.attempt_show(|| Submission::AcceptedByOs),
            Submission::AcceptedByOs
        );
        preferences.set_notifications_enabled(false);
        preferences.set_notifications_enabled(true);
        assert!(matches!(
            state.admit(&value, &preferences, running),
            Ok(None)
        ));
        assert!(matches!(
            state.admit(&prepared(6)?, &preferences, running),
            Ok(None)
        ));
        let next = prepared(8)?;
        let attempt = state
            .admit(&next, &preferences, running)
            .map_err(NotificationBlock::text)?
            .ok_or("next notice unexpectedly coalesced")?;
        assert_eq!(
            attempt.attempt_show(|| Submission::Indeterminate),
            Submission::Indeterminate
        );
        assert!(matches!(
            state.admit(&next, &preferences, running),
            Ok(None)
        ));
        Ok(())
    }

    /// Abandoned preparation preserves admission; actual entry binds exactly one session/notice.
    ///
    /// # Errors
    /// Preserve checked fixture, nonce and real admission failures.
    ///
    /// # Panics
    /// Detect preparation consumption, cross-session mutation, rebinding or a regressed watermark.
    #[test]
    fn attempt_guard_binds_notice_and_session_without_spending_preparation()
    -> Result<(), Box<dyn Error>> {
        let mut bound = session()?;
        let mut other = DeliverySession::new(SessionNonce::new(2).ok_or("nonzero session")?);
        let value = prepared(7)?;
        let unrelated = prepared(8)?;
        {
            let attempt = admitted(&mut bound, &value)?;
            assert!(std::ptr::eq(attempt.prepared(), &raw const value));
            assert_eq!(
                attempt.activation_token().encode(),
                "ntn1.00000000000000000000000000000001.0000000000000007"
            );
        }
        assert_eq!(bound.high_water, 0);
        assert_eq!(bound.latest, None);
        assert_eq!(other.high_water, 0);
        let mut calls = 0;
        let attempt = admitted(&mut bound, &value)?;
        assert_eq!(
            attempt.attempt_show(|| {
                calls += 1;
                Submission::AcceptedByOs
            }),
            Submission::AcceptedByOs
        );
        assert_eq!(calls, 1);
        assert_eq!(bound.high_water, 7);
        assert_eq!(
            bound.latest,
            Some((value.notice(), Submission::AcceptedByOs))
        );
        assert_eq!(other.high_water, 0);
        assert_eq!(other.latest, None);
        {
            let foreign = admitted(&mut other, &unrelated)?;
            assert_eq!(
                foreign.activation_token().encode(),
                "ntn1.00000000000000000000000000000002.0000000000000008"
            );
        }
        assert_eq!(other.high_water, 0);
        let attempt = admitted(&mut bound, &unrelated)?;
        assert_eq!(
            attempt.attempt_show(|| Submission::Indeterminate),
            Submission::Indeterminate
        );
        assert_eq!(bound.high_water, 8);
        assert_eq!(
            bound.latest,
            Some((unrelated.notice(), Submission::Indeterminate))
        );
        let mut preferences = UiPreferences::default();
        preferences.set_notifications_enabled(true);
        for old in [prepared(6)?, prepared(7)?, prepared(8)?] {
            assert!(matches!(
                bound.admit(
                    &old,
                    &preferences,
                    Observation::Current(ObserverPresence::Running)
                ),
                Ok(None)
            ));
            assert_eq!(bound.high_water, 8);
        }
        assert_eq!(other.high_water, 0);
        Ok(())
    }

    /// Unwinding a portable callback entry cannot restore the consumed attempt.
    ///
    /// This is an injected callback fault, not evidence of a native Show call.
    ///
    /// # Errors
    /// Preserve checked notice, genuine admission and healthy-neighbor failures.
    ///
    /// # Panics
    /// Detect replay credit after unwind, false accepted activation or duplicate callback entry.
    #[test]
    fn attempt_callback_unwind_keeps_id_spent_and_acceptance_indeterminate()
    -> Result<(), Box<dyn Error>> {
        let mut state = session()?;
        let value = prepared(7)?;
        let attempt = admitted(&mut state, &value)?;
        let token = attempt.activation_token();
        let mut calls = 0;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            attempt.attempt_show(|| {
                calls += 1;
                std::panic::resume_unwind(Box::new("bounded attempt callback fault"));
            })
        }));
        assert!(result.is_err());
        assert_eq!(calls, 1);
        assert_eq!(state.high_water, 7);
        assert_eq!(
            state.latest,
            Some((value.notice(), Submission::Indeterminate))
        );
        assert_eq!(
            state.resolve_activation(
                Ok(token),
                Observation::Current(value.notice()),
                TrayInteraction::Free,
                true,
            ),
            ActivationResolution::Navigate(ShellRoute::Status)
        );
        let mut preferences = UiPreferences::default();
        preferences.set_notifications_enabled(true);
        assert!(matches!(
            state.admit(
                &value,
                &preferences,
                Observation::Current(ObserverPresence::Running)
            ),
            Ok(None)
        ));
        let next = prepared(8)?;
        let attempt = admitted(&mut state, &next)?;
        assert_eq!(
            attempt.attempt_show(|| Submission::AcceptedByOs),
            Submission::AcceptedByOs
        );
        assert_eq!(state.high_water, 8);
        assert_eq!(
            state.latest,
            Some((next.notice(), Submission::AcceptedByOs))
        );
        Ok(())
    }

    /// Activation proposes navigation only after actual current matching facts.
    ///
    /// # Errors
    /// Propagates unexpected checked fixture rejection.
    ///
    /// # Panics
    /// Fails if stale/foreign/dialog/exit input can select a past context.
    #[test]
    fn activation_rechecks_current_context_and_existing_dialog_ownership()
    -> Result<(), Box<dyn Error>> {
        let mut state = session()?;
        let value = notice(7)?;
        let prepared = PreparedTestNotice::engineering_example(value)?;
        let attempt = admitted(&mut state, &prepared)?;
        let token = attempt.activation_token();
        assert_eq!(
            attempt.attempt_show(|| Submission::AcceptedByOs),
            Submission::AcceptedByOs
        );
        assert_eq!(
            state.resolve_activation(
                Ok(token),
                Observation::Current(value),
                TrayInteraction::DialogOwned,
                true
            ),
            ActivationResolution::FocusOwnedDialog
        );
        assert!(!state.activated);
        assert_eq!(
            state.resolve_activation(
                Ok(token),
                Observation::Current(value),
                TrayInteraction::ExitRequested,
                true
            ),
            ActivationResolution::Ignored
        );
        for current in [
            Observation::Unknown,
            Observation::Stale(value),
            Observation::Current(notice(8)?),
        ] {
            assert_eq!(
                state.resolve_activation(Ok(token), current, TrayInteraction::Free, true),
                ActivationResolution::Navigate(ShellRoute::Status)
            );
        }
        let foreign = ActivationToken::new(SessionNonce::new(2).ok_or("nonzero")?, value);
        assert_eq!(
            state.resolve_activation(
                Ok(foreign),
                Observation::Current(value),
                TrayInteraction::Free,
                true
            ),
            ActivationResolution::Navigate(ShellRoute::Status)
        );
        assert_eq!(
            state.resolve_activation(
                Err(ActivationError),
                Observation::Current(value),
                TrayInteraction::Free,
                true
            ),
            ActivationResolution::Navigate(ShellRoute::Status)
        );
        assert_eq!(
            state.resolve_activation(
                Ok(token),
                Observation::Current(value),
                TrayInteraction::Free,
                true
            ),
            ActivationResolution::Navigate(ShellRoute::Settings(
                crate::shell::SettingsSection::Notifications
            ))
        );
        assert_eq!(
            state.resolve_activation(
                Ok(token),
                Observation::Current(value),
                TrayInteraction::Free,
                true
            ),
            ActivationResolution::Navigate(ShellRoute::Status)
        );
        state.stopped = true;
        assert_eq!(
            state.resolve_activation(
                Ok(token),
                Observation::Current(value),
                TrayInteraction::Free,
                true
            ),
            ActivationResolution::Ignored
        );
        Ok(())
    }

    /// A closed observer wins over fault fallback, current tokens and owned dialogs.
    ///
    /// # Errors
    /// Propagates unexpected checked fixture rejection.
    ///
    /// # Panics
    /// Fails if a late fault/activation can resurrect navigation after close.
    #[test]
    fn stop_precedes_native_fault_and_dialog_fallback() -> Result<(), Box<dyn Error>> {
        let mut state = session()?;
        let value = notice(7)?;
        let prepared = PreparedTestNotice::engineering_example(value)?;
        let attempt = admitted(&mut state, &prepared)?;
        let token = attempt.activation_token();
        assert_eq!(
            attempt.attempt_show(|| Submission::AcceptedByOs),
            Submission::AcceptedByOs
        );
        assert_eq!(
            state.resolve_activation(
                Ok(token),
                Observation::Current(value),
                TrayInteraction::Free,
                false
            ),
            ActivationResolution::Navigate(ShellRoute::Status)
        );
        assert_eq!(
            state.resolve_activation(
                Ok(token),
                Observation::Current(value),
                TrayInteraction::DialogOwned,
                false
            ),
            ActivationResolution::FocusOwnedDialog
        );
        state.stopped = true;
        for interaction in [
            TrayInteraction::Free,
            TrayInteraction::DialogOwned,
            TrayInteraction::ExitRequested,
        ] {
            for available in [true, false] {
                assert_eq!(
                    state.resolve_activation(
                        Ok(token),
                        Observation::Current(value),
                        interaction,
                        available
                    ),
                    ActivationResolution::Ignored
                );
            }
        }
        Ok(())
    }

    /// Indeterminate native attempts never become accepted activation evidence.
    ///
    /// # Errors
    /// Propagates unexpected checked fixture rejection.
    ///
    /// # Panics
    /// Fails if an uncertain attempt supplies a navigable past context.
    #[test]
    fn uncertain_submission_and_changed_sample_context_open_status() -> Result<(), Box<dyn Error>> {
        let mut state = session()?;
        let value = notice(7)?;
        let first = PreparedTestNotice::engineering_example(value)?;
        let attempt = admitted(&mut state, &first)?;
        let token = attempt.activation_token();
        assert_eq!(
            attempt.attempt_show(|| Submission::Indeterminate),
            Submission::Indeterminate
        );
        assert_eq!(
            state.resolve_activation(
                Ok(token),
                Observation::Current(value),
                TrayInteraction::Free,
                true
            ),
            ActivationResolution::Navigate(ShellRoute::Status)
        );
        let mut preferences = UiPreferences::default();
        preferences.set_notifications_enabled(true);
        assert!(matches!(
            state.admit(
                &first,
                &preferences,
                Observation::Current(ObserverPresence::Running)
            ),
            Ok(None)
        ));
        let next = prepared(8)?;
        let attempt = admitted(&mut state, &next)?;
        let token = attempt.activation_token();
        assert_eq!(
            attempt.attempt_show(|| Submission::AcceptedByOs),
            Submission::AcceptedByOs
        );
        let changed =
            FixtureNotice::new(8, FixtureNoticeContext::ServiceNeedsReview).ok_or("nonzero")?;
        assert_eq!(
            state.resolve_activation(
                Ok(token),
                Observation::Current(changed),
                TrayInteraction::Free,
                true
            ),
            ActivationResolution::Navigate(ShellRoute::Status)
        );
        Ok(())
    }
}

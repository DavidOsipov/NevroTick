//! One catalog/copy/typed-feedback owner and guarded complete publications.
//!
//! The current maintained engineering fixture remains distinct from external
//! catalog activation. Normal CatalogStore construction requires the release
//! semantic ledger; raw input cannot enable catalog-ready or choose text slots.
#![forbid(unsafe_code)]

use super::{
    AppWindow, BindingState, ProjectionGuard, catalog_projection,
    generated::{LocalizationCopy, UiCopy},
};
pub(super) use crate::localization::feedback::FeedbackSlot;
use crate::localization::feedback::UiFeedbackSet;
use crate::localization::presentation::UiText;
use crate::localization::{CatalogStore, Error, check_fixture_literal};
use slint::{ComponentHandle, SharedString};
use std::{cell::RefCell, fmt};

/// Sole custody of immutable active catalog, trusted copy and typed feedback.
#[derive(Default)]
pub(super) struct CatalogBindingState {
    /// Present only after complete approved preparation and publication.
    active: Option<CatalogStore>,
    /// Last prepared output; never reconstructed from externally settable globals.
    value: Option<UiCopy>,
    /// Semantic requests, retained independently of translated words.
    feedback: UiFeedbackSet,
    /// Terminal private invariant failure; never confused with initialization.
    failed: bool,
}

impl CatalogBindingState {
    /// Prepare the exact native tray sinks from this current store or explicit fixture.
    ///
    /// # Errors
    /// A failed/missing external meaning never falls back to engineering words.
    #[cfg(windows)]
    pub(super) fn prepare_native_tray(
        &self,
        menu: crate::tray_ui::FixtureTrayMenu,
        interaction: crate::tray_ui::TrayInteraction,
        tooltip: UiText,
    ) -> Result<crate::native_tray::PreparedTray, crate::native_tray::TrayError> {
        if self.failed {
            return Err(crate::native_tray::TrayError::Catalog);
        }
        if let Some(active) = &self.active {
            crate::native_tray::PreparedTray::catalog(
                menu,
                interaction,
                &active.snapshot(),
                tooltip,
            )
        } else {
            crate::native_tray::PreparedTray::fixture(menu, interaction)
        }
    }

    /// Prepare notification fields only from this exact current admitted snapshot.
    ///
    /// # Errors
    /// Failure is distinct from absent catalog; explicit sample mode owns any example.
    #[cfg(windows)]
    pub(super) fn prepare_native_notification(
        &self,
        notice: crate::notifications_ui::FixtureNotice,
    ) -> Result<
        Option<crate::notifications_ui::native_delivery::PreparedTestNotice>,
        crate::notifications_ui::native_delivery::NoticeError,
    > {
        use crate::notifications_ui::native_delivery::{NoticeError, PreparedTestNotice};
        if self.failed {
            return Err(NoticeError::Catalog(Error::Schema));
        }
        self.active
            .as_ref()
            .map(|active| PreparedTestNotice::from_snapshot(&active.snapshot(), notice))
            .transpose()
    }

    /// Preserve a read-only internal failure marker for the receiving guards.
    #[must_use]
    pub(super) const fn failed(&self) -> bool {
        self.failed
    }
}

/// Fixed publication failure, without contributor values or IO paths.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CatalogUpdateError {
    /// An owned dialog, another projection, initialization or exit blocks update.
    Busy,
    /// A full preparation violated protected argument or output constraints.
    Catalog(Error),
}
impl fmt::Display for CatalogUpdateError {
    /// Keep failure reporting independent of any candidate catalog text.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("The interface catalog could not be published.")
    }
}
impl std::error::Error for CatalogUpdateError {
    /// Expose only a closed redacted technical cause where available.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Busy => None,
            Self::Catalog(error) => Some(error),
        }
    }
}

/// A receiving invariant failure disables handlers and exposes minimal fixed copy.
pub(super) fn invariant_failed(window: &AppWindow, state: &RefCell<BindingState>) {
    {
        let mut owner = state.borrow_mut();
        owner.copy.failed = true;
    }
    window.set_catalog_failure(true);
}

/// Complete preparation precedes ownership replacement and one global setter.
///
/// # Errors
/// Rejects busy state or a failed full preparation, preserving the previous
/// catalog/copy, all raw drafts, accepted preferences, navigation and focus.
pub(super) fn publish(
    window: &AppWindow,
    state: &RefCell<BindingState>,
    catalog: CatalogStore,
) -> Result<(), CatalogUpdateError> {
    let _projection = ProjectionGuard::enter(state).ok_or(CatalogUpdateError::Busy)?;
    let snapshot = catalog.snapshot();
    let feedback = state
        .borrow()
        .copy
        .feedback
        .prepare(&snapshot)
        .map_err(CatalogUpdateError::Catalog)?;
    let prepared =
        catalog_projection::prepare(&snapshot, &feedback).map_err(CatalogUpdateError::Catalog)?;
    catalog_projection::check(&prepared).map_err(CatalogUpdateError::Catalog)?;
    #[cfg(windows)]
    let native_tray = {
        let owner = state.borrow();
        super::native_notification_workflow::prepare_catalog(&owner, &snapshot)
            .map_err(|_| CatalogUpdateError::Catalog(Error::Schema))?;
        super::native_workflow::prepare_catalog(&owner, &snapshot)
            .map_err(|_| CatalogUpdateError::Catalog(Error::Schema))?
    };
    #[cfg(windows)]
    super::native_workflow::publish_catalog(window, state, native_tray)
        .map_err(|_| CatalogUpdateError::Catalog(Error::Schema))?;
    {
        let mut owner = state.borrow_mut();
        owner.copy.active = Some(catalog);
        owner.copy.value = Some(prepared.clone());
    }
    window.global::<LocalizationCopy>().set_value(prepared);
    Ok(())
}

/// Update one semantic slot under the caller's already-held projection guard.
pub(super) fn write(
    window: &AppWindow,
    state: &RefCell<BindingState>,
    slot: FeedbackSlot,
    message: Option<UiText>,
    fixture_text: SharedString,
) {
    write_many(window, state, &[(slot, message, fixture_text)]);
}

/// Prepare a bounded semantic batch before replacing state or notifying toolkit.
///
/// At most29 distinct closed slots may be passed. Empty means absent, not an
/// empty translation. Fixture text is bounded technical engineering copy only;
/// it can never bootstrap `CatalogStore`, mark ready or approve external English.
/// Runtime updates re-render all slots from the privately owned active catalog.
/// Internal mismatch fails closed through the fixed bootstrap/invariant surface.
pub(super) fn write_many(
    window: &AppWindow,
    state: &RefCell<BindingState>,
    updates: &[(FeedbackSlot, Option<UiText>, SharedString)],
) {
    if updates.len() > 29
        || updates
            .iter()
            .enumerate()
            .any(|(i, (slot, _, _))| updates[..i].iter().any(|(old, _, _)| old == slot))
    {
        invariant_failed(window, state);
        return;
    }
    if updates.iter().any(|(_, request, fallback)| {
        request.is_some() == fallback.is_empty() || check_fixture_literal(fallback).is_err()
    }) {
        invariant_failed(window, state);
        return;
    }
    let staged = {
        let owner = state.borrow();
        if owner.copy.failed {
            return;
        }
        if owner.initialized && !owner.projecting {
            drop(owner);
            invariant_failed(window, state);
            return;
        }
        let mut candidate = owner.copy.feedback.clone();
        for (slot, request, _) in updates {
            if candidate.get(*slot) != *request {
                candidate.replace(*slot, *request);
            }
        }
        let value: Result<Option<UiCopy>, Error> = if let Some(catalog) = &owner.copy.active {
            (|| {
                let previous = owner.copy.value.as_ref().ok_or(Error::Schema)?;
                let feedback = candidate.prepare(&catalog.snapshot())?;
                let value = catalog_projection::replace_feedback(previous, &feedback)?;
                catalog_projection::check(&value)?;
                Ok(Some(value))
            })()
        } else {
            Ok(None)
        };
        value.map(|value| (candidate, value))
    };
    let Ok((feedback, value)) = staged else {
        invariant_failed(window, state);
        return;
    };
    {
        let mut owner = state.borrow_mut();
        owner.copy.feedback = feedback;
        if let Some(value) = &value {
            owner.copy.value = Some(value.clone());
        }
    }
    if let Some(value) = value {
        window.global::<LocalizationCopy>().set_value(value);
    } else {
        for (slot, _, fallback) in updates {
            fixture_write(window, *slot, fallback.clone());
        }
    }
}

/// Select a fixed engineering fixture setter; text never chooses its target.
fn fixture_write(window: &AppWindow, slot: FeedbackSlot, value: SharedString) {
    match slot {
        FeedbackSlot::EditorError => window.set_fixture_editor_error(value),
        FeedbackSlot::EditorFeedback => window.set_fixture_editor_feedback(value),
        FeedbackSlot::IntervalError => window.set_fixture_interval_error(value),
        FeedbackSlot::IntervalFeedback => window.set_fixture_interval_feedback(value),
        FeedbackSlot::PreferenceError => window.set_fixture_preference_error(value),
        FeedbackSlot::PreferenceFeedback => window.set_fixture_preference_feedback(value),
        FeedbackSlot::RecoveryError => window.set_fixture_recovery_error(value),
        FeedbackSlot::RecoveryFeedback => window.set_fixture_recovery_feedback(value),
        FeedbackSlot::RecoveryBackLabel => window.set_fixture_recovery_back_label(value),
        FeedbackSlot::QuitHeadline => window.set_fixture_quit_headline(value),
        FeedbackSlot::QuitConsequence => window.set_fixture_quit_consequence(value),
        FeedbackSlot::ShellFeedback => window.set_fixture_shell_feedback(value),
        FeedbackSlot::ServiceFeedback => window.set_fixture_service_feedback(value),
        FeedbackSlot::NotificationFeedback => window.set_fixture_notification_feedback(value),
        FeedbackSlot::DiagnosticFeedback => window.set_fixture_diagnostic_feedback(value),
        FeedbackSlot::PolicyError => window.set_fixture_policy_error(value),
        FeedbackSlot::SourceQueryError => window.set_fixture_source_query_error(value),
        FeedbackSlot::AcceptedIntervalLabel => window.set_fixture_accepted_interval_label(value),
        FeedbackSlot::StatusFeedback => window.set_fixture_status_feedback(value),
        FeedbackSlot::TrayFeedback => window.set_fixture_tray_feedback(value),
        FeedbackSlot::StatusHeadline => window.set_fixture_status_headline(value),
        FeedbackSlot::StatusConsequence => window.set_fixture_status_consequence(value),
        FeedbackSlot::StatusProfileLabel => window.set_fixture_status_profile_label(value),
        FeedbackSlot::StatusProfile => window.set_fixture_status_profile(value),
        FeedbackSlot::StatusAuthentication => window.set_fixture_status_authentication(value),
        FeedbackSlot::StatusAgreement => window.set_fixture_status_agreement(value),
        FeedbackSlot::StatusFreshness => window.set_fixture_status_freshness(value),
        FeedbackSlot::StatusCheckLabel => window.set_fixture_status_check_label(value),
        FeedbackSlot::StatusRemedyLabel => window.set_fixture_status_remedy_label(value),
    }
}

#[cfg(test)]
/// Mechanism controls use an explicitly test-only exact fixed English grant.
mod tests {
    use std::{error::Error, rc::Rc};

    use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
    use slint::platform::{Platform, PlatformError, WindowAdapter};
    use slint::{ComponentHandle, Model};

    use super::catalog_projection::{CopySlot, OUTPUT_SLOT_COUNT};
    use super::{CatalogUpdateError, LocalizationCopy, UiCopy};
    use crate::slint_host::{self, Page};

    /// Portable public platform; no native window or external operation exists.
    struct Headless;

    impl Platform for Headless {
        /// Supply an ordinary public software adapter without a display.
        fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
            Ok(MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer))
        }
    }

    /// A real generated global receives one complete prepared value; drafts stay.
    ///
    /// # Errors
    /// Propagates unexpected platform, host, fixture-admission or preparation errors.
    ///
    /// # Panics
    /// Fails if a partial copy or unrelated state change is observed, or a busy
    /// dialog can be bypassed. Test authority never qualifies release text.
    #[test]
    fn one_generated_copy_commit_preserves_drafts_and_owned_dialogs() -> Result<(), Box<dyn Error>>
    {
        slint::platform::set_platform(Box::new(Headless))?;
        let store = crate::localization::test_only_projection_fixture()?;
        let host = slint_host::create()?;
        let window = host.window();
        window.invoke_mode_requested(true);
        window.invoke_navigate(Page::Sources);
        window.invoke_edit_requested();
        assert_eq!(
            window.invoke_raw_text_edited(0, "Exact raw label".into()),
            "Exact raw label"
        );
        assert_eq!(
            window.invoke_raw_text_edited(1, " raw host ".into()),
            " raw host "
        );
        assert_eq!(window.invoke_raw_text_edited(2, "-1".into()), "-1");
        let selected = window.get_selected_source();
        host.publish_catalog(store)?;
        let global = window.global::<LocalizationCopy>();
        let copy = global.get_value();
        assert_eq!(copy.revision, "2");
        check_flat_rows(&copy);
        check_feedback_replacement(window, &copy);
        assert_eq!(window.get_draft_label(), "Exact raw label");
        assert_eq!(window.get_draft_host(), " raw host ");
        assert_eq!(window.get_draft_port(), "-1");
        assert_eq!(window.get_selected_source(), selected);
        assert_eq!(window.get_current_page(), Page::Sources);
        assert!(window.get_editor_open());

        window.invoke_cancel_requested();
        window.invoke_quit_requested();
        assert!(window.get_quit_warning_open());
        assert_eq!(
            host.publish_catalog(crate::localization::test_only_projection_fixture()?),
            Err(CatalogUpdateError::Busy)
        );
        assert_eq!(global.get_value().revision, copy.revision);
        window.invoke_quit_cancel_requested();
        assert!(window.window().is_visible());
        // Quit showed this manual fixture; Cancel intentionally keeps it shown.
        window.hide()?;
        assert!(!window.window().is_visible());
        let weak = window.as_weak();
        drop(global);
        drop(host);
        assert!(weak.upgrade().is_none());
        check_failed_feedback_preparation_keeps_publication()?;
        Ok(())
    }

    /// Independent slot oracles cover flattened arrays and immutable public rows.
    ///
    /// # Panics
    /// Fails if a slot aliases, a static array entry is absent or rows are mutable.
    fn check_flat_rows(copy: &UiCopy) {
        assert!(copy.catalog_ready);
        // Contract v2 adds native feedback rows; this is an independent fixed oracle.
        assert_eq!(OUTPUT_SLOT_COUNT, 426);
        assert_eq!(copy.texts.row_count(), OUTPUT_SLOT_COUNT);
        for (slot, expected) in [
            (CopySlot::FIXTURE_ABOUT_HEADING, "About NevroTick"),
            (CopySlot::PREFERENCES_INTERVAL_OPTIONS_1, "Every 2 minutes"),
            (CopySlot::PREFERENCES_INTERVAL_OPTIONS_2, "Every 5 minutes"),
            (CopySlot::PREFERENCES_INTERVAL_OPTIONS_3, "Every 15 minutes"),
            (
                CopySlot::PREFERENCES_INTERVAL_OPTIONS_4,
                "Enter my own interval",
            ),
            (
                CopySlot::SOURCE_PROTOCOLS_0,
                "NTS (authenticated responses)",
            ),
            (CopySlot::SOURCE_PROTOCOLS_1, "NTP (unauthenticated)"),
            (CopySlot::RECOVERY_COUNTS_0, "Off"),
            (CopySlot::RECOVERY_COUNTS_1, "One helper"),
            (CopySlot::RECOVERY_COUNTS_2, "Two helpers"),
            (CopySlot::DIAGNOSTIC_DURATIONS_0, "5 minutes"),
        ] {
            assert_eq!(copy.texts.row_data(slot.index()), Some(expected.into()));
        }
        for slot in [
            CopySlot::DIAGNOSTIC_SCOPES_0,
            CopySlot::DIAGNOSTIC_SCOPES_1,
            CopySlot::DIAGNOSTIC_SCOPES_2,
        ] {
            assert!(
                copy.texts
                    .row_data(slot.index())
                    .is_some_and(|text| !text.is_empty())
            );
        }
        copy.texts
            .set_row_data(CopySlot::FIXTURE_ABOUT_HEADING.index(), "Forged row".into());
        assert_eq!(
            copy.texts.row_data(CopySlot::FIXTURE_ABOUT_HEADING.index()),
            Some("About NevroTick".into())
        );
    }

    /// A real typed feedback update cannot mutate a previously published model.
    ///
    /// # Panics
    /// Fails if mutable shared rows overwrite an old snapshot or raw draft.
    fn check_feedback_replacement(window: &slint_host::AppWindow, before: &UiCopy) {
        let slot = CopySlot::FEEDBACK_EDITOR_ERROR.index();
        assert_eq!(before.texts.row_data(slot), Some("".into()));
        assert_eq!(
            window.invoke_raw_text_edited(1, "emoji🙂".into()),
            " raw host "
        );
        let current = window.global::<LocalizationCopy>().get_value();
        assert_eq!(current.revision, before.revision);
        assert!(!window.get_editor_error().is_empty());
        assert_eq!(
            current.texts.row_data(slot),
            Some(window.get_editor_error())
        );
        assert_eq!(before.texts.row_data(slot), Some("".into()));
        assert_eq!(window.get_draft_host(), " raw host ");
    }

    /// An explicit private malformed-last-value fault cannot publish partial rows.
    ///
    /// # Errors
    /// Propagates fixture/host preparation or unexpected transition admission.
    ///
    /// # Panics
    /// Fails if preparation replaces feedback/global state before full validation.
    fn check_failed_feedback_preparation_keeps_publication() -> Result<(), Box<dyn Error>> {
        use super::FeedbackSlot;
        use crate::localization::{MessageId, presentation::UiText};

        let host = slint_host::create()?;
        let window = host.window();
        host.publish_catalog(crate::localization::test_only_projection_fixture()?)?;
        let global = window.global::<LocalizationCopy>();
        let before = global.get_value();
        let rows: Vec<_> = before.texts.iter().collect();
        let previous_feedback = host.state.borrow().copy.feedback.clone();
        let mut malformed = before.clone();
        malformed.texts = slint::ModelRc::default();
        host.state.borrow_mut().copy.value = Some(malformed);
        {
            let _projection =
                super::ProjectionGuard::enter(&host.state).ok_or("Expected idle owner")?;
            super::write(
                window,
                &host.state,
                FeedbackSlot::EditorError,
                Some(UiText::fixed(MessageId::RAW_CONTEXT)),
                "Fixed bounded fixture rejection.".into(),
            );
        }
        assert!(host.state.borrow().copy.failed());
        assert!(window.get_catalog_failure());
        assert_eq!(host.state.borrow().copy.feedback, previous_feedback);
        let after = global.get_value();
        assert_eq!(after.revision, before.revision);
        assert_eq!(after.catalog_ready, before.catalog_ready);
        assert_eq!(after.texts.iter().collect::<Vec<_>>(), rows);
        assert!(!window.window().is_visible());
        let weak = window.as_weak();
        drop(global);
        drop(host);
        assert!(weak.upgrade().is_none());
        Ok(())
    }
}

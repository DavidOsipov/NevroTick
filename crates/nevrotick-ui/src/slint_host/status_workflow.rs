//! One fixed disconnected status input and guarded contextual UI navigation.
#![forbid(unsafe_code)]

use std::{cell::RefCell, rc::Rc};

use slint::ComponentHandle;

use super::catalog_workflow::{FeedbackSlot, invariant_failed, write, write_many};
use super::{AppWindow, BindingState, ProjectionGuard};
use crate::localization::UiText;
use crate::presentation::Page;
use crate::status_ui::{StatusCheckOutcome, StatusPresentationPort, StatusText, StatusView};

/// Read an immutable UI DTO while its actual owner is borrowed, without effects.
fn view(state: &RefCell<BindingState>) -> StatusView {
    StatusView::from_observation(state.borrow().status.observe())
}

/// Project one coherent observation after releasing the state borrow.
pub(super) fn project(window: &AppWindow, state: &RefCell<BindingState>) {
    let status = view(state);
    // A fixed nine-field set is prepared completely before any publication.
    let requested = [
        (FeedbackSlot::StatusHeadline, status.headline()),
        (FeedbackSlot::StatusConsequence, status.consequence()),
        (FeedbackSlot::StatusProfileLabel, StatusText::ProfileLabel),
        (FeedbackSlot::StatusProfile, status.profile_text()),
        (FeedbackSlot::StatusAuthentication, status.authentication()),
        (FeedbackSlot::StatusAgreement, status.agreement()),
        (FeedbackSlot::StatusFreshness, status.freshness()),
        (FeedbackSlot::StatusCheckLabel, status.check_text()),
        (FeedbackSlot::StatusRemedyLabel, status.remedy_text()),
    ];
    let prepared = requested
        .into_iter()
        .map(|(slot, text)| {
            UiText::key(text.catalog_id())
                .map(|message| (slot, Some(message), text.english().into()))
        })
        .collect::<Result<Vec<_>, _>>();
    let Ok(prepared) = prepared else {
        invariant_failed(window, state);
        return;
    };
    write_many(window, state, &prepared);
}

/// Retain only a closed `StatusText` meaning for later coherent catalog replay.
fn feedback(window: &AppWindow, state: &RefCell<BindingState>, text: StatusText) {
    let Ok(message) = UiText::key(text.catalog_id()) else {
        invariant_failed(window, state);
        return;
    };
    write(
        window,
        state,
        FeedbackSlot::StatusFeedback,
        Some(message),
        text.english().into(),
    );
}

/// Bind no-argument actions; raw view text/flags cannot choose routes or authority.
pub(super) fn install_callbacks(window: &AppWindow, state: &Rc<RefCell<BindingState>>) {
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_status_check_requested(move || {
        let Some(window) = weak.upgrade() else {
            return;
        };
        let Some(_projection) = ProjectionGuard::enter(&owner) else {
            return;
        };
        let outcome = {
            let state = owner.borrow();
            if state.navigation.page() == Page::Status {
                let status = StatusView::from_observation(state.status.observe());
                Some(state.status.request_check(status.check_intent()))
            } else {
                None
            }
        };
        let message = match outcome {
            Some(StatusCheckOutcome::AdapterUnavailable) => StatusText::CheckUnavailable,
            None => StatusText::ActionRequired,
        };
        feedback(&window, &owner, message);
    });
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_status_remedy_requested(move || {
        let Some(window) = weak.upgrade() else {
            return;
        };
        let Some(projection_guard) = ProjectionGuard::enter(&owner) else {
            return;
        };
        let route = {
            let state = owner.borrow();
            if state.navigation.page() == Page::Status {
                Some(
                    StatusView::from_observation(state.status.observe())
                        .remedy()
                        .route(),
                )
            } else {
                None
            }
        };
        let Some(route) = route else {
            feedback(&window, &owner, StatusText::ActionRequired);
            return;
        };
        write(
            &window,
            &owner,
            FeedbackSlot::StatusFeedback,
            None,
            "".into(),
        );
        drop(projection_guard);
        super::recovery_workflow::request_shell_route(&window.as_weak(), &owner, route, false);
    });
}

#[cfg(test)]
mod tests {
    //! Replay uses retained typed meanings, not current fixture display strings.

    use std::{error::Error, rc::Rc};

    use slint::ComponentHandle;
    use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
    use slint::platform::{Platform, PlatformError, WindowAdapter};

    use crate::slint_host;
    use crate::status_ui::StatusText;

    /// Public software fixture; no service, timer, browser or event loop exists.
    struct Headless;

    impl Platform for Headless {
        /// Create an ordinary public software window without a native backend.
        fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
            Ok(MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer))
        }
    }

    /// Catalog publication replays one checked status/shell feedback generation.
    ///
    /// # Errors
    /// Propagates platform, checked host, test-only admission or publication errors.
    ///
    /// # Panics
    /// Fails if raw fixture words supply meanings or unrelated drafts are replaced.
    #[test]
    fn catalog_replay_restores_typed_status_shell_service_and_notification_feedback()
    -> Result<(), Box<dyn Error>> {
        slint::platform::set_platform(Box::new(Headless))?;
        let store = crate::localization::test_only_projection_fixture()?;
        let host = slint_host::create()?;
        let window = host.window();
        window.show()?;
        window.invoke_mode_requested(true);
        window.invoke_navigate(slint_host::Page::Sources);
        window.invoke_edit_requested();
        assert_eq!(
            window.invoke_raw_text_edited(1, " retain raw host ".into()),
            " retain raw host "
        );
        window.invoke_navigate(slint_host::Page::Settings);
        assert_eq!(window.invoke_raw_choice_edited(0, 4), 4);
        assert_eq!(window.invoke_raw_text_edited(4, "0".into()), "0");
        window.invoke_settings_section_requested(1);
        window.invoke_service_command_requested(0);
        window.invoke_settings_section_requested(2);
        window.invoke_notification_test_requested();
        window.invoke_navigate(slint_host::Page::Status);
        window.invoke_status_check_requested();
        window.invoke_quit_requested();
        assert!(window.get_quit_warning_open());
        window.invoke_quit_cancel_requested();
        assert!(!window.get_quit_warning_open());
        window.set_fixture_status_headline("Display text cannot establish Tracking".into());
        window.set_fixture_shell_feedback("Display text cannot confirm quitting".into());
        window.set_fixture_service_feedback("Display text cannot start a service".into());
        window
            .set_fixture_notification_feedback("Display text cannot submit a notification".into());
        host.publish_catalog(store)?;
        assert_eq!(
            window.get_status_headline(),
            StatusText::NoConnection.english()
        );
        assert_eq!(
            window.get_status_feedback(),
            StatusText::CheckUnavailable.english()
        );
        assert_eq!(
            window.get_shell_feedback(),
            "Quit cancelled. The app remains open; preview drafts are retained."
        );
        assert_eq!(
            window.get_service_feedback(),
            "Service management is unavailable in this preview. No request was submitted; runtime, startup and clock ownership were not changed."
        );
        assert_eq!(
            window.get_notification_feedback(),
            "A running NevroTick window or tray observer is not confirmed. No Windows notification was submitted."
        );
        assert_eq!(window.get_quit_headline(), "Service state is unconfirmed");
        assert_eq!(
            window.get_quit_consequence(),
            "Quitting closes only the app observer. It does not stop services or confirm whether they are running or stopped."
        );
        assert_eq!(window.get_draft_host(), " retain raw host ");
        assert_eq!(window.get_draft_custom_minutes(), "0");
        assert!(!window.get_ui_exit_requested());
        // The fixture deliberately showed this observer; end that custody here.
        window.hide()?;
        let weak = window.as_weak();
        drop(host);
        assert!(weak.upgrade().is_none());
        Ok(())
    }
}

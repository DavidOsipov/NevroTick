//! Local profile selector/review bindings; no active configuration or Apply port.
#![forbid(unsafe_code)]

use std::{cell::RefCell, rc::Rc};

use slint::ComponentHandle;

use super::catalog_workflow::{FeedbackSlot, write};
use super::{AppWindow, BindingState, EditorTarget, ProjectionGuard, generated::PolicyCopy};
use crate::localization::{MessageId, presentation::UiText};
use crate::policy::{PolicyError, PolicyPhase};
use crate::presentation::{Mode, Page};
use crate::shell::SettingsSection;

/// Accepted projection only; raw selector text/value stays in the root component.
struct PolicyProjection {
    /// Closed Summary/Editing/Review selector, never an authority supplied by view.
    phase: i32,
    /// Distinguishes no preview from a preview equal to its fixture baseline.
    has_preview: bool,
    /// Checked candidate or fixture fallback for presentation only.
    choice: i32,
}

impl PolicyProjection {
    /// Copy trusted state before any synchronous toolkit setter is invoked.
    fn from_owner(owner: &BindingState) -> Self {
        Self {
            phase: match owner.policy.phase() {
                PolicyPhase::Summary => 0,
                PolicyPhase::Editing => 1,
                PolicyPhase::Review => 2,
            },
            has_preview: owner.policy.preview().is_some(),
            choice: owner
                .policy
                .preview()
                .unwrap_or(owner.policy.baseline())
                .view_choice(),
        }
    }

    /// Reproject accepted state without overwriting the raw choice on rejection.
    fn apply(self, window: &AppWindow) {
        window.set_policy_phase(self.phase);
        window.set_policy_has_preview(self.has_preview);
        window.set_policy_accepted_choice(self.choice);
    }
}

/// Seed the fixture presentation before host readiness is published.
pub(super) fn initialize(window: &AppWindow, state: &RefCell<BindingState>) {
    let projection = PolicyProjection::from_owner(&state.borrow());
    if !super::raw_workflow::reset(
        window,
        state,
        crate::ui_drafts::ResetInput {
            policy_choice: Some(projection.choice),
            ..crate::ui_drafts::ResetInput::default()
        },
    ) {
        return;
    }
    projection.apply(window);
}

/// Closed profile transitions; none carries a backend revision, permission or URI.
#[derive(Clone, Copy)]
enum Request {
    /// Open or reopen selector with a checked reset choice.
    Edit,
    /// Admit the untrusted signed presentation choice before review.
    Review(i32),
    /// Drop unsubmitted raw selector changes, retaining an earlier preview.
    Cancel,
    /// Reopen the retained checked preview for review.
    OpenReview,
    /// Close review with the candidate still local and unapplied.
    Keep,
    /// Remove only a candidate currently under review.
    Discard,
}

/// Weak captures share the root owner and cannot create a component cycle.
pub(super) fn install_callbacks(window: &AppWindow, state: &Rc<RefCell<BindingState>>) {
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_policy_edit_requested(move || request(&weak, &owner, Request::Edit));
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_policy_review_requested(move |raw| request(&weak, &owner, Request::Review(raw)));
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_policy_cancel_requested(move || request(&weak, &owner, Request::Cancel));
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_policy_open_review_requested(move || request(&weak, &owner, Request::OpenReview));
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_policy_keep_requested(move || request(&weak, &owner, Request::Keep));
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_policy_discard_requested(move || request(&weak, &owner, Request::Discard));
}

/// Apply the phase/route guard independently of externally writable view flags.
fn request(weak: &slint::Weak<AppWindow>, state: &RefCell<BindingState>, request: Request) {
    let Some(window) = weak.upgrade() else {
        return;
    };
    let Some(_projection) = ProjectionGuard::enter(state) else {
        return;
    };
    let review_present = {
        let owner = state.borrow();
        owner.navigation.page() == Page::Settings
            && owner.navigation.mode() == Mode::Power
            && owner.shell.settings_section() == SettingsSection::TimePolicy
            && owner.editor == EditorTarget::Closed
            && owner.policy.phase() == PolicyPhase::Editing
    };
    let stale_review = matches!(request, Request::Review(choice)
        if review_present && state.borrow().raw.policy_choice() != choice);
    if stale_review {
        super::raw_workflow::reject_stale(&window, state, FeedbackSlot::PolicyError);
        return;
    }
    let source_editor_active = state.borrow().editor != EditorTarget::Closed;
    let outcome = {
        let mut owner = state.borrow_mut();
        if owner.navigation.page() != Page::Settings
            || owner.navigation.mode() != Mode::Power
            || owner.shell.settings_section() != SettingsSection::TimePolicy
            || owner.editor != EditorTarget::Closed
        {
            None
        } else {
            let result = match request {
                Request::Edit => owner.policy.edit().map(Some),
                Request::Review(raw) => owner.policy.review(raw).map(|()| None),
                Request::Cancel => owner.policy.cancel_edit().map(Some),
                Request::OpenReview => owner.policy.open_review().map(|()| None),
                Request::Keep => owner.policy.keep_preview().map(|()| None),
                Request::Discard => owner.policy.discard_preview().map(|()| None),
            };
            Some((result, PolicyProjection::from_owner(&owner)))
        }
    };
    let copy = window.global::<PolicyCopy>();
    if let Some((result, projection)) = outcome {
        projection.apply(&window);
        match result {
            Ok(reset) => {
                if let Some(choice) = reset
                    && !super::raw_workflow::reset(
                        &window,
                        state,
                        crate::ui_drafts::ResetInput {
                            policy_choice: Some(choice),
                            ..crate::ui_drafts::ResetInput::default()
                        },
                    )
                {
                    return;
                }
                write(&window, state, FeedbackSlot::PolicyError, None, "".into());
                window.invoke_focus_settings_section();
            }
            Err(error) => {
                super::raw_workflow::project(&window, state);
                let (id, fallback) = match error {
                    PolicyError::InvalidChoice => {
                        (MessageId::POLICY_INVALID_CHOICE, copy.get_invalid_choice())
                    }
                    PolicyError::WrongPhase => {
                        (MessageId::POLICY_PHASE_REQUIRED, copy.get_phase_required())
                    }
                    PolicyError::NoPreview => (
                        MessageId::POLICY_NO_PREVIEW_ERROR,
                        copy.get_no_preview_error(),
                    ),
                };
                write(
                    &window,
                    state,
                    FeedbackSlot::PolicyError,
                    Some(UiText::fixed(id)),
                    fallback,
                );
            }
        }
    } else {
        let (id, fallback) = if source_editor_active {
            (MessageId::POLICY_EDITOR_ACTIVE, copy.get_editor_active())
        } else {
            (MessageId::POLICY_PAGE_REQUIRED, copy.get_page_required())
        };
        write(
            &window,
            state,
            FeedbackSlot::PolicyError,
            Some(UiText::fixed(id)),
            fallback,
        );
    }
}

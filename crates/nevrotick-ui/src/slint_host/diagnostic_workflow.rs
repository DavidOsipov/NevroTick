//! Checked diagnostic subviews and disconnected effect feedback.
#![forbid(unsafe_code)]

use super::catalog_workflow::{FeedbackSlot, write};
use super::{AppWindow, BindingState, ProjectionGuard};
use crate::diagnostics_ui::{CollectionScope, DiagnosticView};
use crate::localization::{MessageId, presentation::UiText};
use crate::presentation::{Mode, Page};
use slint::ComponentHandle;
use std::{cell::RefCell, rc::Rc};

/// Copy only closed values before any synchronous toolkit property notification.
pub(super) fn project(window: &AppWindow, state: &RefCell<BindingState>) {
    let (view, scope, duration) = {
        let owner = state.borrow();
        let view = match owner.diagnostics.view() {
            DiagnosticView::Overview => 0,
            DiagnosticView::Report => 1,
            DiagnosticView::TemporaryCollection => 2,
            DiagnosticView::CorrectionReview => 3,
        };
        let scope = match owner.diagnostics.scope() {
            CollectionScope::Ui => 0,
            CollectionScope::Service => 1,
            CollectionScope::UiAndService => 2,
        };
        (view, scope, i32::from(owner.diagnostics.duration_choice()))
    };
    window.set_diagnostic_view(view);
    window.set_diagnostic_scope(scope);
    window.set_diagnostic_duration(duration);
}

/// A direct callback cannot edit an inactive or Simple diagnostic surface.
fn reached(owner: &BindingState) -> bool {
    owner.navigation.mode() == Mode::Power && owner.navigation.page() == Page::Diagnostics
}

/// Bind weak local callbacks; no native/logging/clipboard/filesystem adapter exists.
pub(super) fn install_callbacks(window: &AppWindow, state: &Rc<RefCell<BindingState>>) {
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_diagnostic_view_requested(move |choice| {
        let Some(window) = weak.upgrade() else {
            return;
        };
        let Some(_projection) = ProjectionGuard::enter(&owner) else {
            return;
        };
        let result = {
            let mut state = owner.borrow_mut();
            if !reached(&state) {
                return;
            }
            state.diagnostics.select_view(choice)
        };
        project(&window, &owner);
        if result.is_ok() {
            window.set_page_scroll_y(0.0);
            write(
                &window,
                &owner,
                FeedbackSlot::DiagnosticFeedback,
                None,
                "".into(),
            );
        } else {
            write(
                &window,
                &owner,
                FeedbackSlot::DiagnosticFeedback,
                Some(UiText::fixed(MessageId::DIAGNOSTIC_INVALID_CHOICE)),
                "Choose one of the available diagnostic options.".into(),
            );
        }
    });
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_diagnostic_collection_requested(move |scope, duration| {
        let Some(window) = weak.upgrade() else {
            return;
        };
        let Some(_projection) = ProjectionGuard::enter(&owner) else {
            return;
        };
        let result = {
            let mut state = owner.borrow_mut();
            if !reached(&state) {
                return;
            }
            state.diagnostics.select_collection(scope, duration)
        };
        project(&window, &owner);
        let (message, fixture) = if result.is_ok() {
            (None, "")
        } else {
            (
                Some(UiText::fixed(MessageId::DIAGNOSTIC_INVALID_CHOICE)),
                "Choose one of the available diagnostic options.",
            )
        };
        write(
            &window,
            &owner,
            FeedbackSlot::DiagnosticFeedback,
            message,
            fixture.into(),
        );
    });
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_diagnostic_action_requested(move |_choice| {
        let Some(window) = weak.upgrade() else { return; };
        let Some(_projection) = ProjectionGuard::enter(&owner) else { return; };
        if !reached(&owner.borrow()) { return; }
        write(&window, &owner, FeedbackSlot::DiagnosticFeedback, Some(UiText::fixed(MessageId::DIAGNOSTIC_UNAVAILABLE_ACTION)), "This operation is unavailable. No collection, clipboard, file, administrator or clock operation was performed.".into());
    });
}

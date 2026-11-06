//! Bounded search/detail bindings over the root's one retained checked source owner.
#![forbid(unsafe_code)]

use std::{cell::RefCell, rc::Rc};

use slint::{ComponentHandle, VecModel};

use super::catalog_workflow::{FeedbackSlot, write, write_many};
use super::generated::{InspectionCopy, RawCopy, SourceCopy};
use super::source_workflow::SourceProjection;
use super::{AppWindow, BindingState, EditorTarget, ProjectionGuard, SourceChangeRow, SourceRow};
use crate::localization::{MessageId, presentation::UiText};
use crate::presentation::{Mode, Page};
use crate::source_inspection::{InspectionError, SourceDetails};

/// Checked detail snapshot; no untrusted text or cached row index is authoritative.
pub(super) struct InspectionProjection {
    /// Exact bounded accepted query, separate from root raw search text.
    query: String,
    /// Whether the retained editor target still appears in this visible subset.
    selected_visible: bool,
    /// Resolve fresh membership/fields on each mutation rather than cached text.
    details: Result<Option<SourceDetails>, InspectionError>,
}

impl InspectionProjection {
    /// Consume the same sole source owner for this inspection state's entire life.
    pub(super) fn from_owner(owner: &BindingState) -> Self {
        Self {
            query: owner.inspection.query().text().to_owned(),
            selected_visible: owner.selected.is_some_and(|id| {
                owner
                    .inspection
                    .source_visible(&owner.sources, id.view_id())
                    == Ok(true)
            }),
            details: owner.inspection.details(&owner.sources),
        }
    }

    /// Project accepted detail/query facts without admitting any incoming value.
    pub(super) fn apply(self, window: &AppWindow, state: &RefCell<BindingState>) {
        window.set_accepted_source_query(self.query.into());
        window.set_selected_source_visible(self.selected_visible);
        match self.details {
            Ok(Some(details)) => {
                window.set_inspection_detail_source(super::source_workflow::source_row(
                    details.source(),
                    None,
                ));
                window.set_inspection_detail_hidden(details.hidden_by_query());
                window.set_inspection_detail_open(true);
            }
            Ok(None) => {
                window.set_inspection_detail_open(false);
                window.set_inspection_detail_hidden(false);
            }
            Err(error) => {
                window.set_inspection_detail_open(false);
                window.set_inspection_detail_hidden(false);
                show_error(window, state, error);
            }
        }
    }
}

/// Closed local list requests; no search/filter can alter source or Statistics data.
#[derive(Clone, Copy)]
enum Request<'a> {
    /// Validate original ASCII bytes before replacing the accepted query.
    Query(&'a str),
    /// Clear accepted/raw query without touching selections or pending changes.
    Clear,
    /// Reject unavailable Used/Other roles even for direct callback invocation.
    Filter(i32),
    /// Open only this currently visible stable source ID.
    Details(i32),
    /// Close details, preserving query and all source/draft data.
    CloseDetails,
}

/// Bind real generated receivers under the existing root projection/modal guard.
pub(super) fn install_callbacks(
    window: &AppWindow,
    state: &Rc<RefCell<BindingState>>,
    rows: &Rc<VecModel<SourceRow>>,
    changes: &Rc<VecModel<SourceChangeRow>>,
) {
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    let projection = Rc::clone(rows);
    let review = Rc::clone(changes);
    window.on_source_query_requested(move |raw| {
        request(&weak, &owner, &projection, &review, Request::Query(&raw));
    });
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    let projection = Rc::clone(rows);
    let review = Rc::clone(changes);
    window.on_source_query_clear_requested(move || {
        request(&weak, &owner, &projection, &review, Request::Clear);
    });
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    let projection = Rc::clone(rows);
    let review = Rc::clone(changes);
    window.on_source_filter_requested(move |raw| {
        request(&weak, &owner, &projection, &review, Request::Filter(raw));
    });
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    let projection = Rc::clone(rows);
    let review = Rc::clone(changes);
    window.on_source_details_requested(move |id| {
        request(&weak, &owner, &projection, &review, Request::Details(id));
    });
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    let projection = Rc::clone(rows);
    let review = Rc::clone(changes);
    window.on_source_details_close_requested(move || {
        request(&weak, &owner, &projection, &review, Request::CloseDetails);
    });
}

/// Admit one actual route/phase plus raw query/filter/identity before projection.
fn request(
    weak: &slint::Weak<AppWindow>,
    state: &RefCell<BindingState>,
    rows: &Rc<VecModel<SourceRow>>,
    changes: &Rc<VecModel<SourceChangeRow>>,
    request: Request<'_>,
) {
    let Some(window) = weak.upgrade() else {
        return;
    };
    let Some(_projection) = ProjectionGuard::enter(state) else {
        return;
    };
    // A stale duplicate Close must not erase another form's validation.
    let detail_absent = {
        let owner = state.borrow();
        matches!(owner.inspection.details(&owner.sources), Ok(None))
    };
    if matches!(request, Request::CloseDetails) && detail_absent {
        return;
    }
    if !query_matches(&window, state, request) {
        return;
    }
    let Some((result, projection)) = prepare_request(state, request) else {
        show_context_error(&window, state);
        return;
    };
    projection.apply(&window, rows, changes, state);
    super::raw_workflow::project(&window, state);
    match result {
        Ok(()) => {
            if matches!(request, Request::Clear)
                && !super::raw_workflow::reset(
                    &window,
                    state,
                    crate::ui_drafts::ResetInput {
                        source_query: Some(""),
                        ..crate::ui_drafts::ResetInput::default()
                    },
                )
            {
                return;
            }
            clear_feedback(&window, state, request);
            if matches!(request, Request::Details(_) | Request::CloseDetails) {
                window.invoke_focus_source_details();
            }
        }
        Err(error) => show_error(&window, state, error),
    }
}

/// Bind only an actually present query submission to its private raw snapshot.
fn query_matches(window: &AppWindow, state: &RefCell<BindingState>, request: Request<'_>) -> bool {
    let Request::Query(raw) = request else {
        return true;
    };
    let mismatch = {
        let owner = state.borrow();
        super::raw_workflow::source_list_present(&owner)
            && owner.source_search_open()
            && !owner.sources.preview().is_empty()
            && owner.raw.get(crate::ui_drafts::TextField::SourceQuery) != raw
    };
    if mismatch {
        super::raw_workflow::reject_stale(window, state, FeedbackSlot::SourceQueryError);
    }
    !mismatch
}

/// Prepare one contextual operation before all toolkit/model notifications.
fn prepare_request(
    state: &RefCell<BindingState>,
    request: Request<'_>,
) -> Option<(Result<(), InspectionError>, SourceProjection)> {
    let mut owner = state.borrow_mut();
    let query_request = matches!(request, Request::Query(_) | Request::Filter(_));
    let list_request = matches!(
        request,
        Request::Query(_) | Request::Clear | Request::Filter(_) | Request::Details(_)
    );
    if owner.navigation.page() != Page::Sources
        || owner.navigation.mode() != Mode::Power
        || owner.editor != EditorTarget::Closed
        || (list_request && !super::raw_workflow::source_list_present(&owner))
        || (query_request && (!owner.source_search_open() || owner.sources.preview().is_empty()))
        || (matches!(request, Request::Clear) && !clear_present(&owner))
    {
        return None;
    }
    let result = match request {
        Request::Query(raw) => {
            let result = owner.inspection.set_query(raw);
            if result.is_ok() {
                owner.source_search = super::raw_workflow::SearchVisibility::Collapsed;
            }
            result
        }
        Request::Clear => {
            owner.inspection.clear_query();
            Ok(())
        }
        Request::Filter(raw) => owner.inspection.set_filter_choice(raw),
        Request::Details(id) => {
            let BindingState {
                sources,
                inspection,
                ..
            } = &mut *owner;
            inspection.open_details(sources, id)
        }
        Request::CloseDetails => {
            owner.inspection.close_details();
            Ok(())
        }
    };
    Some((result, SourceProjection::from_owner(&owner)))
}

/// Clear exists only in the open editor or its accepted-query summary row.
fn clear_present(owner: &BindingState) -> bool {
    if owner.source_search_open() {
        !owner.sources.preview().is_empty()
    } else {
        !owner.inspection.query().text().is_empty()
    }
}

/// Choose the current route/subflow reason without retaining a rejected payload.
fn show_context_error(window: &AppWindow, state: &RefCell<BindingState>) {
    let (editor_active, route, list_present) = {
        let owner = state.borrow();
        (
            owner.editor != EditorTarget::Closed,
            owner.navigation.page() == Page::Sources && owner.navigation.mode() == Mode::Power,
            super::raw_workflow::source_list_present(&owner),
        )
    };
    let copy = window.global::<InspectionCopy>();
    let (id, fallback) = if editor_active {
        (
            MessageId::INSPECTION_EDITOR_ACTIVE,
            copy.get_editor_active(),
        )
    } else if !route {
        (
            MessageId::INSPECTION_PAGE_REQUIRED,
            copy.get_page_required(),
        )
    } else if !list_present {
        (
            MessageId::SOURCE_LIST_REQUIRED,
            window.global::<SourceCopy>().get_list_required(),
        )
    } else {
        (
            MessageId::RAW_CONTEXT,
            window.global::<RawCopy>().get_context(),
        )
    };
    write(
        window,
        state,
        FeedbackSlot::SourceQueryError,
        Some(UiText::fixed(id)),
        fallback,
    );
}

/// Clear exactly the local feedback slots changed by an admitted list request.
/// The caller retains the projection guard; both query/editor clears publish once.
fn clear_feedback(window: &AppWindow, state: &RefCell<BindingState>, request: Request<'_>) {
    if matches!(
        request,
        Request::Query(_) | Request::Clear | Request::CloseDetails
    ) {
        write_many(
            window,
            state,
            &[
                (FeedbackSlot::SourceQueryError, None, "".into()),
                (FeedbackSlot::EditorError, None, "".into()),
            ],
        );
    } else {
        write(
            window,
            state,
            FeedbackSlot::SourceQueryError,
            None,
            "".into(),
        );
    }
}

/// Map a bounded typed rejection to immutable copy, never rejected payload text.
fn show_error(window: &AppWindow, state: &RefCell<BindingState>, error: InspectionError) {
    let copy = window.global::<InspectionCopy>();
    let (message, fallback) = match error {
        InspectionError::InvalidQuery => (
            UiText::fixed(MessageId::INSPECTION_INVALID_QUERY),
            copy.get_invalid_query(),
        ),
        InspectionError::InvalidFilter => (
            UiText::fixed(MessageId::INSPECTION_INVALID_FILTER),
            copy.get_invalid_filter(),
        ),
        InspectionError::RolesUnavailable => (
            UiText::fixed(MessageId::INSPECTION_ROLES_UNAVAILABLE),
            copy.get_roles_unavailable(),
        ),
        InspectionError::HiddenSource => (
            UiText::fixed(MessageId::INSPECTION_HIDDEN_SOURCE),
            copy.get_hidden_source(),
        ),
        InspectionError::Source(error) => (UiText::source_error(error), error.to_string().into()),
    };
    write(
        window,
        state,
        FeedbackSlot::SourceQueryError,
        Some(message),
        fallback,
    );
}

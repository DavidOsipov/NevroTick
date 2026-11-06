//! Fixture-only source editor/list/review bindings owned by the root UI state.
//! Bounded raw fields and checked `SourceDrafts` candidates stay separate; no effects.
#![forbid(unsafe_code)]

use std::{cell::RefCell, rc::Rc};

use slint::{ComponentHandle, Model, VecModel};

use super::catalog_workflow::{FeedbackSlot, write};
use super::generated::SourceCopy;
use super::{
    AppWindow, BindingState, EDITOR_ACTIVE_MESSAGE, EDITOR_CLOSED_MESSAGE, EditorTarget,
    ProjectionGuard, SourceChangeRow, SourceRow,
};
use crate::localization::{MessageId, presentation::UiText};
use crate::presentation::{Mode, Page};
use crate::source_drafts::{
    ChangeKind, DraftSource, Protocol, ProtocolPortDraft, RawSourceInput, SourceChange,
    SourceDraftError, SourceId,
};

/// Bind only source list/editor/review events under the parent's lifecycle owner.
pub(super) fn install_callbacks(
    window: &AppWindow,
    state: &Rc<RefCell<BindingState>>,
    rows: &Rc<VecModel<SourceRow>>,
    changes: &Rc<VecModel<SourceChangeRow>>,
) {
    install_source_editor_callbacks(window, state, rows, changes);
    install_source_list_callbacks(window, state, rows, changes);
}

/// Owned projection lets every domain borrow end before toolkit mutation.
pub(super) struct EditorProjection {
    /// Accepted or initial editor label; raw rejected input is never retained.
    label: String,
    /// Accepted or empty editor host, with no DNS resolution or URI interpretation.
    host: String,
    /// Accepted/default decimal port for explicit reset points only.
    port: String,
}

impl EditorProjection {
    /// Copy bounded checked source values for an Edit/Cancel reset.
    fn from_source(source: &DraftSource) -> Self {
        Self {
            label: source.fields().label().to_owned(),
            host: source.fields().host().to_owned(),
            port: source.fields().port().to_string(),
        }
    }

    /// Supply an empty Add editor using the explicit untouched NTS default.
    fn for_add() -> Self {
        Self {
            label: String::new(),
            host: String::new(),
            port: Protocol::Nts.default_port().to_string(),
        }
    }

    /// Replace raw drafts only after a checked selection/open/save/cancel action.
    pub(super) fn apply(self, window: &AppWindow, state: &RefCell<BindingState>) -> bool {
        if !super::raw_workflow::reset(
            window,
            state,
            crate::ui_drafts::ResetInput {
                source: Some(crate::ui_drafts::SourceFields {
                    label: &self.label,
                    host: &self.host,
                    port: &self.port,
                }),
                ..crate::ui_drafts::ResetInput::default()
            },
        ) {
            return false;
        }
        write(window, state, FeedbackSlot::EditorError, None, "".into());
        true
    }
}

/// Snapshot of bounded sources/review/selection, independent of `RefCell` borrows.
pub(super) struct SourceProjection {
    /// Effective rows, each carrying a stable positive ID.
    rows: Vec<SourceRow>,
    /// One checked before/after row per pending local change.
    changes: Vec<SourceChangeRow>,
    /// Zero means None; it is never accepted as an authority ID.
    selected: i32,
    /// Root view flags are projections of receiving state only.
    review_open: bool,
    /// Separate filtered source rows; the all-source Statistics model is intact.
    visible_rows: Vec<SourceRow>,
    /// Checked query/detail/selection presence from the same retained owner.
    inspection: super::inspection_workflow::InspectionProjection,
}

impl SourceProjection {
    /// Copy checked domain data before releasing its borrow for model callbacks.
    pub(super) fn from_owner(owner: &BindingState) -> Self {
        let rows: Vec<SourceRow> = owner
            .sources
            .preview()
            .into_iter()
            .map(|source| {
                let kind = owner
                    .sources
                    .review()
                    .iter()
                    .find(|change| change.id() == source.id())
                    .map(SourceChange::kind);
                source_row(source, kind)
            })
            .collect();
        let visible_ids = owner.inspection.visible_ids(&owner.sources);
        let visible_rows = rows
            .iter()
            .filter(|row| visible_ids.iter().any(|id| id.view_id() == row.view_id))
            .cloned()
            .collect();
        Self {
            visible_rows,
            inspection: super::inspection_workflow::InspectionProjection::from_owner(owner),
            rows,
            changes: owner.sources.review().iter().map(change_row).collect(),
            selected: owner.selected.map_or(0, SourceId::view_id),
            review_open: owner.review_open,
        }
    }

    /// Notify changed same-ID rows individually; structural edits reset the model.
    pub(super) fn apply(
        self,
        window: &AppWindow,
        rows: &Rc<VecModel<SourceRow>>,
        changes: &Rc<VecModel<SourceChangeRow>>,
        state: &RefCell<BindingState>,
    ) {
        let stable_order = rows.row_count() == self.rows.len()
            && rows
                .iter()
                .zip(&self.rows)
                .all(|(old, new)| old.view_id == new.view_id);
        if stable_order {
            for (index, row) in self.rows.into_iter().enumerate() {
                if !rows.row_data(index).is_some_and(|old| same_row(&old, &row)) {
                    rows.set_row_data(index, row);
                }
            }
        } else {
            rows.set_vec(self.rows);
        }
        changes.set_vec(self.changes);
        window.set_sources(rows.clone().into());
        window.set_visible_sources(Rc::new(VecModel::from(self.visible_rows)).into());
        self.inspection.apply(window, state);
        window.set_source_changes(changes.clone().into());
        window.set_selected_source(self.selected);
        window.set_review_open(self.review_open);
    }
}

/// Install receiving selection/editor requests independently of view flags.
fn install_source_editor_callbacks(
    window: &AppWindow,
    state: &Rc<RefCell<BindingState>>,
    rows: &Rc<VecModel<SourceRow>>,
    changes: &Rc<VecModel<SourceChangeRow>>,
) {
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_source_selected(move |id| select_source(&weak, &owner, id));
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_edit_requested(move || open_editor(&weak, &owner, false));
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_add_requested(move || open_editor(&weak, &owner, true));
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    let projection = Rc::clone(rows);
    let review = Rc::clone(changes);
    window.on_update_requested(move |label, host, protocol_choice, port| {
        update_preview(
            &weak,
            &owner,
            &projection,
            &review,
            RawSourceInput {
                label: &label,
                host: &host,
                protocol_choice,
                port: &port,
            },
        );
    });
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_cancel_requested(move || cancel_editor(&weak, &owner));
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_protocol_requested(move |choice| change_protocol(&weak, &owner, choice));
    // Obsolete event seam is retained for generated-API compatibility only.
    // Only an admitted raw field2 edit may change manual-port provenance.
    window.on_port_edited(|| {});
}

/// Install bounded list/review requests without exposing a backend Apply command.
fn install_source_list_callbacks(
    window: &AppWindow,
    state: &Rc<RefCell<BindingState>>,
    rows: &Rc<VecModel<SourceRow>>,
    changes: &Rc<VecModel<SourceChangeRow>>,
) {
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    let projection = Rc::clone(rows);
    let review = Rc::clone(changes);
    window.on_remove_requested(move |id| {
        source_request(
            &weak,
            &owner,
            &projection,
            &review,
            SourceRequest::Remove(id),
        );
    });
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    let projection = Rc::clone(rows);
    let review = Rc::clone(changes);
    window.on_undo_requested(move |id| {
        source_request(&weak, &owner, &projection, &review, SourceRequest::Undo(id));
    });
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    let projection = Rc::clone(rows);
    let review = Rc::clone(changes);
    window.on_review_requested(move || {
        source_request(
            &weak,
            &owner,
            &projection,
            &review,
            SourceRequest::Review(true),
        );
    });
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    let projection = Rc::clone(rows);
    let review = Rc::clone(changes);
    window.on_review_close_requested(move || {
        source_request(
            &weak,
            &owner,
            &projection,
            &review,
            SourceRequest::Review(false),
        );
    });
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    let projection = Rc::clone(rows);
    let review = Rc::clone(changes);
    window.on_discard_requested(move || {
        source_request(&weak, &owner, &projection, &review, SourceRequest::Discard);
    });
}

/// Convert checked values to a stable-ID source row without claiming live votes.
pub(super) fn source_row(source: &DraftSource, change: Option<ChangeKind>) -> SourceRow {
    let (protocol_label, port_label) = match source.protocol() {
        Protocol::Nts => ("NTS preview", "NTS-KE port"),
        Protocol::PlainNtp => ("Plain NTP preview", "NTP port"),
    };
    let draft_kind = match change {
        Some(ChangeKind::Add) => "Add",
        Some(ChangeKind::Edit) => "Edit",
        Some(ChangeKind::Remove) | None => "Baseline",
    };
    SourceRow {
        view_id: source.id().view_id(),
        label: source.fields().label().into(),
        host: source.fields().host().into(),
        port: source.fields().port().to_string().into(),
        protocol_choice: source.protocol().view_choice(),
        protocol_label: protocol_label.into(),
        port_label: port_label.into(),
        draft_kind: draft_kind.into(),
        show_draft_badge: change.is_some(),
        change_choice: change_choice(change),
    }
}

/// Compare every reached row field before deciding whether to notify observers.
fn same_row(left: &SourceRow, right: &SourceRow) -> bool {
    left.view_id == right.view_id
        && left.label == right.label
        && left.host == right.host
        && left.port == right.port
        && left.protocol_choice == right.protocol_choice
        && left.protocol_label == right.protocol_label
        && left.port_label == right.port_label
        && left.draft_kind == right.draft_kind
        && left.show_draft_badge == right.show_draft_badge
        && left.change_choice == right.change_choice
}

/// Format only bounded accepted fields for a local before/after review cell.
fn review_value(source: Option<&DraftSource>) -> String {
    match source {
        Some(source) => {
            let protocol = match source.protocol() {
                Protocol::Nts => "NTS",
                Protocol::PlainNtp => "Plain NTP",
            };
            format!(
                "{} · {} · {protocol} · port {}",
                source.fields().label(),
                source.fields().host(),
                source.fields().port()
            )
        }
        None => "None".to_owned(),
    }
}

/// Build a bounded inert review row; text contains no markup or command target.
fn change_row(change: &SourceChange) -> SourceChangeRow {
    let kind = match change.kind() {
        ChangeKind::Add => "Add",
        ChangeKind::Edit => "Edit",
        ChangeKind::Remove => "Remove",
    };
    SourceChangeRow {
        view_id: change.id().view_id(),
        change_choice: change_choice(Some(change.kind())),
        before_present: change.before().is_some(),
        before_source: change
            .before()
            .map_or_else(SourceRow::default, |source| source_row(source, None)),
        after_present: change.after().is_some(),
        after_source: change
            .after()
            .map_or_else(SourceRow::default, |source| source_row(source, None)),
        kind: kind.into(),
        before: review_value(change.before()).into(),
        after: review_value(change.after()).into(),
    }
}

/// Seed accepted selection fields, or an empty NTS editor for an empty preview.
pub(super) fn selected_editor(owner: &BindingState) -> EditorProjection {
    owner
        .selected
        .and_then(|id| owner.sources.source(id.view_id()).ok())
        .map_or_else(EditorProjection::for_add, EditorProjection::from_source)
}

/// Check stable selection and current membership, never treating rows as IDs.
fn select_source(weak: &slint::Weak<AppWindow>, state: &RefCell<BindingState>, view_id: i32) {
    let Some(window) = weak.upgrade() else {
        return;
    };
    let Some(_projection) = ProjectionGuard::enter(state) else {
        return;
    };
    if !sources_reached(&window, state) {
        return;
    }
    if state.borrow().review_open {
        write(
            &window,
            state,
            FeedbackSlot::EditorError,
            Some(UiText::fixed(MessageId::SOURCE_LIST_REQUIRED)),
            window.global::<SourceCopy>().get_list_required(),
        );
        return;
    }
    let outcome = {
        let mut owner = state.borrow_mut();
        if owner.editor == EditorTarget::Closed {
            let source = owner
                .inspection
                .source_visible(&owner.sources, view_id)
                .and_then(|visible| {
                    if visible {
                        Ok(())
                    } else {
                        Err(crate::source_inspection::InspectionError::HiddenSource)
                    }
                })
                .and_then(|()| owner.sources.source(view_id).map_err(Into::into));
            match source {
                Ok(source) => {
                    let id = source.id();
                    let projection = EditorProjection::from_source(source);
                    owner.selected = Some(id);
                    Ok((id, projection))
                }
                Err(error) => Err((UiText::inspection_error(error), error.to_string())),
            }
        } else {
            Err((
                UiText::fixed(MessageId::ERROR_EDIT_BEFORE_SELECT),
                EDITOR_ACTIVE_MESSAGE.to_owned(),
            ))
        }
    };
    match outcome {
        Ok((id, projection)) => {
            window.set_selected_source(id.view_id());
            window.set_selected_source_visible(true);
            if !projection.apply(&window, state) {
                return;
            }
            write(
                &window,
                state,
                FeedbackSlot::EditorFeedback,
                None,
                "".into(),
            );
        }
        Err((message, fixture)) => write(
            &window,
            state,
            FeedbackSlot::EditorError,
            Some(message),
            fixture.into(),
        ),
    }
}

/// Open Add or selected-ID Edit; repeated opens never replace active raw fields.
fn open_editor(weak: &slint::Weak<AppWindow>, state: &RefCell<BindingState>, adding: bool) {
    let Some(window) = weak.upgrade() else {
        return;
    };
    let Some(_projection) = ProjectionGuard::enter(state) else {
        return;
    };
    if !sources_reached(&window, state) {
        return;
    }
    if state.borrow().review_open {
        write(
            &window,
            state,
            FeedbackSlot::EditorError,
            Some(UiText::fixed(MessageId::SOURCE_LIST_REQUIRED)),
            window.global::<SourceCopy>().get_list_required(),
        );
        return;
    }
    let outcome = {
        let mut owner = state.borrow_mut();
        if owner.editor != EditorTarget::Closed {
            return;
        }
        if adding {
            owner.editor = EditorTarget::Add;
            owner.port_draft = Some(ProtocolPortDraft::new(Protocol::Nts));
            Ok(EditorProjection::for_add())
        } else {
            if owner.selected.is_some_and(|id| {
                owner
                    .inspection
                    .source_visible(&owner.sources, id.view_id())
                    == Ok(false)
            }) {
                drop(owner);
                show_hidden_source(&window, state);
                return;
            }
            let source = owner
                .selected
                .ok_or(SourceDraftError::UnknownSource)
                .and_then(|id| {
                    if owner
                        .inspection
                        .source_visible(&owner.sources, id.view_id())
                        == Ok(true)
                    {
                        owner.sources.source(id.view_id())
                    } else {
                        Err(SourceDraftError::UnknownSource)
                    }
                });
            match source {
                Ok(source) => {
                    let target = source.id();
                    let port = ProtocolPortDraft::from_source(source);
                    let projection = EditorProjection::from_source(source);
                    owner.editor = EditorTarget::Edit(target);
                    owner.port_draft = Some(port);
                    Ok(projection)
                }
                Err(error) => Err(error),
            }
        }
    };
    match outcome {
        Ok(projection) => {
            if !projection.apply(&window, state) {
                return;
            }
            write(
                &window,
                state,
                FeedbackSlot::EditorFeedback,
                None,
                "".into(),
            );
            window.set_editor_adding(adding);
            window.set_editor_open(true);
        }
        Err(error) => write(
            &window,
            state,
            FeedbackSlot::EditorError,
            Some(UiText::source_error(error)),
            error.to_string().into(),
        ),
    }
}

/// Admit every raw source field using the Rust-owned Add/Edit target only.
fn update_preview(
    weak: &slint::Weak<AppWindow>,
    state: &RefCell<BindingState>,
    rows: &Rc<VecModel<SourceRow>>,
    changes: &Rc<VecModel<SourceChangeRow>>,
    raw: RawSourceInput<'_>,
) {
    let Some(window) = weak.upgrade() else {
        return;
    };
    let Some(_projection) = ProjectionGuard::enter(state) else {
        return;
    };
    if !sources_reached(&window, state) {
        return;
    }
    if state.borrow().editor != EditorTarget::Closed
        && !super::raw_workflow::source_matches(&state.borrow(), raw)
    {
        super::raw_workflow::reject_stale(&window, state, FeedbackSlot::EditorError);
        return;
    }
    let outcome = {
        let mut owner = state.borrow_mut();
        let result = match owner.editor {
            EditorTarget::Closed => Err((
                UiText::fixed(MessageId::ERROR_EDITOR_CLOSED),
                EDITOR_CLOSED_MESSAGE.to_owned(),
            )),
            EditorTarget::Add => owner
                .sources
                .add(raw)
                .map(Some)
                .map_err(|error| (UiText::source_error(error), error.to_string())),
            EditorTarget::Edit(id) => owner
                .sources
                .edit(id.view_id(), raw)
                .map(|()| Some(id))
                .map_err(|error| (UiText::source_error(error), error.to_string())),
        };
        result.map(|selected| {
            owner.selected = selected;
            owner.editor = EditorTarget::Closed;
            owner.port_draft = None;
            owner.reconcile_selection();
            (
                SourceProjection::from_owner(&owner),
                selected_editor(&owner),
            )
        })
    };
    match outcome {
        Ok((sources, editor)) => {
            sources.apply(&window, rows, changes, state);
            if !editor.apply(&window, state) {
                return;
            }
            window.set_editor_open(false);
            window.set_editor_adding(false);
            write(&window, state, FeedbackSlot::EditorFeedback, Some(UiText::fixed(MessageId::RUST_SOURCE_PREVIEW_UPDATED)), "Preview updated in memory only. The active fixture and service configuration are unchanged.".into());
        }
        Err((message, fixture)) => {
            super::raw_workflow::project(&window, state);
            write(
                &window,
                state,
                FeedbackSlot::EditorError,
                Some(message),
                fixture.into(),
            );
            write(
                &window,
                state,
                FeedbackSlot::EditorFeedback,
                None,
                "".into(),
            );
        }
    }
}

/// Cancel only unsubmitted raw fields; previously submitted local deltas survive.
fn cancel_editor(weak: &slint::Weak<AppWindow>, state: &RefCell<BindingState>) {
    let Some(window) = weak.upgrade() else {
        return;
    };
    let Some(_projection) = ProjectionGuard::enter(state) else {
        return;
    };
    if !sources_reached(&window, state) {
        return;
    }
    let projection = {
        let mut owner = state.borrow_mut();
        if owner.editor == EditorTarget::Closed {
            return;
        }
        owner.editor = EditorTarget::Closed;
        owner.port_draft = None;
        selected_editor(&owner)
    };
    if !projection.apply(&window, state) {
        return;
    }
    window.set_editor_open(false);
    window.set_editor_adding(false);
    write(
        &window,
        state,
        FeedbackSlot::EditorFeedback,
        Some(UiText::fixed(MessageId::FEEDBACK_SOURCE_CANCELLED)),
        "Unaccepted editor changes discarded.".into(),
    );
}

/// Change protocol and only replace an untouched default, retaining invalid drafts.
fn change_protocol(weak: &slint::Weak<AppWindow>, state: &RefCell<BindingState>, choice: i32) {
    super::raw_workflow::protocol_request(weak, state, choice);
}

/// Closed local source-list requests; none represents a backend policy Apply.
#[derive(Clone, Copy)]
enum SourceRequest {
    /// Preview removal of this current positive stable identity.
    Remove(i32),
    /// Restore/drop one pending change by stable identity.
    Undo(i32),
    /// Open/close the inert before/after review panel.
    Review(bool),
    /// Drop all submitted local deltas, preserving preferences and baseline.
    Discard,
}

/// Dispatch list/review actions with an independent active-editor guard.
fn source_request(
    weak: &slint::Weak<AppWindow>,
    state: &RefCell<BindingState>,
    rows: &Rc<VecModel<SourceRow>>,
    changes: &Rc<VecModel<SourceChangeRow>>,
    request: SourceRequest,
) {
    let Some(window) = weak.upgrade() else {
        return;
    };
    let Some(_projection) = ProjectionGuard::enter(state) else {
        return;
    };
    if !sources_reached(&window, state) {
        return;
    }
    if !source_request_ready(&window, state, request) {
        return;
    }
    let outcome = {
        let mut owner = state.borrow_mut();
        if owner.editor == EditorTarget::Closed {
            let result = match request {
                SourceRequest::Remove(id) => owner.sources.remove(id),
                SourceRequest::Undo(id) => owner.sources.undo(id),
                SourceRequest::Review(open) => {
                    owner.review_open = open;
                    Ok(())
                }
                SourceRequest::Discard => {
                    owner.sources.discard();
                    owner.review_open = false;
                    Ok(())
                }
            };
            result
                .map(|()| {
                    owner.reconcile_selection();
                    (
                        SourceProjection::from_owner(&owner),
                        selected_editor(&owner),
                    )
                })
                .map_err(|error| (UiText::source_error(error), error.to_string()))
        } else {
            Err((
                UiText::fixed(MessageId::ERROR_EDIT_BEFORE_SELECT),
                EDITOR_ACTIVE_MESSAGE.to_owned(),
            ))
        }
    };
    match outcome {
        Ok((sources, editor)) => {
            sources.apply(&window, rows, changes, state);
            if !editor.apply(&window, state) {
                return;
            }
            let (message, fixture) = source_feedback(request);
            write(
                &window,
                state,
                FeedbackSlot::EditorFeedback,
                message,
                fixture.into(),
            );
            if matches!(request, SourceRequest::Review(_) | SourceRequest::Discard) {
                window.invoke_focus_source_details();
            }
        }
        Err((message, fixture)) => {
            let reveal = matches!(request, SourceRequest::Undo(_)) && state.borrow().review_open;
            write(
                &window,
                state,
                FeedbackSlot::EditorError,
                Some(message),
                fixture.into(),
            );
            if reveal {
                window.invoke_focus_source_details();
            }
        }
    }
}

/// Select fixed semantic feedback from the closed local source operation.
fn source_feedback(request: SourceRequest) -> (Option<UiText>, &'static str) {
    match request {
        SourceRequest::Review(true) => (
            Some(UiText::fixed(MessageId::RUST_SOURCE_REVIEW_ONLY)),
            "Review local changes only. Apply is unavailable.",
        ),
        SourceRequest::Review(false) => (None, ""),
        SourceRequest::Discard => (
            Some(UiText::fixed(MessageId::RUST_SOURCE_DISCARDED)),
            "All source preview changes discarded.",
        ),
        SourceRequest::Remove(_) | SourceRequest::Undo(_) => (
            Some(UiText::fixed(MessageId::RUST_SOURCE_OPERATION_UPDATED)),
            "Source preview updated. The active fixture is unchanged.",
        ),
    }
}

/// A list/review context refusal, separate from domain operation errors.
enum SourceRequestProblem {
    /// A dedicated Review or Details pane currently hides the source list.
    List,
    /// Undo and Discard require the actual changes-review pane.
    Review,
    /// There is no pending change for a new Review to display.
    Empty,
    /// A query-hidden identity cannot be removed by a stale row action.
    Hidden,
}

/// Check the current subflow before a list/review action mutates the sole owner.
fn source_request_ready(
    window: &AppWindow,
    state: &RefCell<BindingState>,
    request: SourceRequest,
) -> bool {
    let problem = {
        let owner = state.borrow();
        if owner.editor != EditorTarget::Closed {
            None
        } else if owner.review_open && matches!(request, SourceRequest::Remove(_)) {
            Some(SourceRequestProblem::List)
        } else if matches!(request, SourceRequest::Undo(_) | SourceRequest::Discard)
            && !owner.review_open
        {
            Some(SourceRequestProblem::Review)
        } else if matches!(request, SourceRequest::Review(true))
            && owner.sources.review().is_empty()
        {
            Some(SourceRequestProblem::Empty)
        } else if matches!(request, SourceRequest::Remove(id) if owner.inspection.source_visible(&owner.sources, id) == Ok(false))
        {
            Some(SourceRequestProblem::Hidden)
        } else {
            None
        }
    };
    let Some(problem) = problem else {
        return true;
    };
    if matches!(problem, SourceRequestProblem::Hidden) {
        show_hidden_source(window, state);
        return false;
    }
    let copy = window.global::<SourceCopy>();
    let (id, fallback) = match problem {
        SourceRequestProblem::List => (MessageId::SOURCE_LIST_REQUIRED, copy.get_list_required()),
        SourceRequestProblem::Review => (
            MessageId::SOURCE_REVIEW_REQUIRED,
            copy.get_review_required(),
        ),
        SourceRequestProblem::Empty => (MessageId::SOURCE_EMPTY_REVIEW, copy.get_empty_review()),
        SourceRequestProblem::Hidden => return false,
    };
    write(
        window,
        state,
        FeedbackSlot::EditorError,
        Some(UiText::fixed(id)),
        fallback,
    );
    false
}

/// Project the fixed identity-hidden reason without retaining rejected input.
fn show_hidden_source(window: &AppWindow, state: &RefCell<BindingState>) {
    let error = crate::source_inspection::InspectionError::HiddenSource;
    write(
        window,
        state,
        FeedbackSlot::EditorError,
        Some(UiText::inspection_error(error)),
        error.to_string().into(),
    );
}

/// Admit source mutations only while their real Power Sources route is reached.
///
/// This is local workflow context, not a privilege boundary. Navigation retains
/// raw and checked drafts, while hidden callbacks cannot edit or discard them.
fn sources_reached(window: &AppWindow, state: &RefCell<BindingState>) -> bool {
    let (reached, details_open) = {
        let owner = state.borrow();
        (
            owner.navigation.mode() == Mode::Power && owner.navigation.page() == Page::Sources,
            owner.editor == EditorTarget::Closed
                && matches!(owner.inspection.details(&owner.sources), Ok(Some(_))),
        )
    };
    if reached && details_open {
        write(
            window,
            state,
            FeedbackSlot::EditorError,
            Some(UiText::fixed(MessageId::SOURCE_LIST_REQUIRED)),
            window.global::<SourceCopy>().get_list_required(),
        );
        return false;
    }
    if !reached {
        write(
            window,
            state,
            FeedbackSlot::EditorError,
            Some(UiText::fixed(MessageId::SOURCE_PAGE_REQUIRED)),
            window.global::<SourceCopy>().get_page_required(),
        );
    }
    reached
}

/// Fixed badge/review choices; localized words never select a change kind.
const fn change_choice(change: Option<ChangeKind>) -> i32 {
    match change {
        None => 0,
        Some(ChangeKind::Add) => 1,
        Some(ChangeKind::Edit) => 2,
        Some(ChangeKind::Remove) => 3,
    }
}

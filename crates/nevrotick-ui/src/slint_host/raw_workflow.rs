//! Authoritative bounded raw input; widget strings are requests, never draft owners.
//!
//! Restore-only projection remains available while a modal owns commands. Every
//! mutation independently checks actual route/phase/presence, never view flags.
#![forbid(unsafe_code)]

use std::{cell::RefCell, rc::Rc};

use slint::{ComponentHandle, SharedString};

use super::catalog_workflow::{FeedbackSlot, write};
use super::{AppWindow, BindingState, EditorTarget, ProjectionGuard, generated::RawCopy};
use crate::localization::{MessageId, presentation::UiText};
use crate::policy::PolicyPhase;
use crate::presentation::{Mode, Page};
use crate::shell::SettingsSection;
use crate::source_drafts::{DraftSource, PortUpdate, Protocol};
use crate::ui_drafts::{DraftInputError, ResetInput, TextField};

/// A checked reset failure closes ordinary input without disabling emergency UI.
#[derive(Clone, Copy)]
pub(super) enum RawAdmission {
    /// Checked raw buffers are available for ordinary contextual edits.
    Open,
    /// An internal reset invariant failed; only restore and emergency exit remain.
    Failed,
}

/// Current search field presence is explicit private presentation state.
#[derive(Clone, Copy)]
pub(super) enum SearchVisibility {
    /// The compact source list has no editable query field.
    Collapsed,
    /// The source list may show its field while it has at least one source.
    Expanded,
}

impl SearchVisibility {
    /// Convert one closed checkable request without retaining a widget property.
    const fn from_open(open: bool) -> Self {
        if open {
            Self::Expanded
        } else {
            Self::Collapsed
        }
    }
}

/// Owned bounded projection; state borrows end before toolkit callbacks can occur.
struct RawProjection {
    /// Seven strings total at most 5248 content bytes, never arbitrary widget text.
    texts: [SharedString; 7],
    /// Closed raw interval choice, independent of the accepted checking interval.
    interval: i32,
    /// Closed raw helper count, independent of the accepted recovery preview.
    recovery: i32,
    /// Closed raw profile selector; no policy activation or authentication claim.
    policy: i32,
    /// Existing `ProtocolPortDraft` or selected checked source remains its sole owner.
    protocol: i32,
    /// Source search expansion is an owned presentation context.
    search_open: bool,
}

impl RawProjection {
    /// Copy only checked raw values and an existing typed source protocol.
    fn from_owner(owner: &BindingState) -> Self {
        let fields = [
            TextField::SourceLabel,
            TextField::SourceHost,
            TextField::SourcePort,
            TextField::SourceQuery,
            TextField::CustomMinutes,
            TextField::RecoveryFirst,
            TextField::RecoverySecond,
        ];
        Self {
            texts: fields.map(|field| owner.raw.get(field).into()),
            interval: owner.raw.interval_choice(),
            recovery: owner.raw.recovery_count(),
            policy: owner.raw.policy_choice(),
            protocol: source_protocol(owner),
            search_open: owner.source_search_open(),
        }
    }

    /// Force explicit widget resync even if a parent value compares equal.
    fn apply(self, window: &AppWindow) {
        let [label, host, port, query, custom, first, second] = self.texts;
        window.set_draft_label(label);
        window.set_draft_host(host);
        window.set_draft_port(port);
        window.set_draft_source_query(query);
        window.set_draft_custom_minutes(custom);
        window.set_draft_recovery_first(first);
        window.set_draft_recovery_second(second);
        window.set_draft_interval_choice(self.interval);
        window.set_draft_recovery_count(self.recovery);
        window.set_draft_policy_choice(self.policy);
        window.set_draft_protocol_choice(self.protocol);
        window.set_source_search_open(self.search_open);
        // This notification is not an authority or monotonic generation.
        window.set_raw_revision(!window.get_raw_revision());
    }
}

/// Project a checked snapshot under the caller's guard or during initialization.
pub(super) fn project(window: &AppWindow, state: &RefCell<BindingState>) {
    let projection = RawProjection::from_owner(&state.borrow());
    projection.apply(window);
}

/// Atomically reset supplied raw groups from checked workflow values.
/// Returns false and closes input admission if an internal budget invariant fails.
pub(super) fn reset(
    window: &AppWindow,
    state: &RefCell<BindingState>,
    input: ResetInput<'_>,
) -> bool {
    let result = state.borrow_mut().raw.reset_groups(input);
    if result.is_err() {
        state.borrow_mut().raw_admission = RawAdmission::Failed;
        // Keep the already-initialized presentation available for the emergency
        // close warning. Private admission, rather than a spoofable view flag,
        // closes every ordinary receiver after this internal failure.
        // This historically named shared marker selects fixed internal-error
        // presentation only; copy.failed remains a separate catalog authority.
        window.set_catalog_failure(true);
        let copy = window.global::<RawCopy>();
        write(
            window,
            state,
            FeedbackSlot::EditorError,
            Some(UiText::fixed(MessageId::RAW_UNAVAILABLE)),
            copy.get_unavailable(),
        );
        return false;
    }
    project(window, state);
    true
}

/// Weak handlers return only admitted raw text/choices for the sender's resync.
pub(super) fn install_callbacks(window: &AppWindow, state: &Rc<RefCell<BindingState>>) {
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_raw_text_edited(move |field, value| edit_text(&weak, &owner, field, &value));
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_raw_choice_edited(move |field, value| edit_choice(&weak, &owner, field, value));
    let owner = Rc::clone(state);
    window.on_raw_action_permitted(move |field, action| action_permitted(&owner, field, action));
    window.on_raw_menu_key(move |text, shift| menu_key(&text, shift));
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_source_search_requested(move |open| search_requested(&weak, &owner, open));
}

/// Source list controls have no live presence while another inline view owns it.
pub(super) fn source_list_present(owner: &BindingState) -> bool {
    owner.navigation.mode() == Mode::Power
        && owner.navigation.page() == Page::Sources
        && owner.editor == EditorTarget::Closed
        && !owner.review_open
        && !matches!(owner.inspection.details(&owner.sources), Ok(Some(_)))
}

/// Current modal/catalog state is checked even for queued accessibility actions.
fn input_idle(owner: &BindingState) -> bool {
    !owner.copy.failed()
        && !owner.raw_failed()
        && !owner.shell.blocks_other_requests()
        && owner.recovery_leave.is_none()
}

/// Admit the exact visible field independently of disabled/stale widget state.
fn field_present(owner: &BindingState, field: TextField) -> bool {
    match field {
        TextField::SourceLabel | TextField::SourceHost | TextField::SourcePort => {
            owner.navigation.mode() == Mode::Power
                && owner.navigation.page() == Page::Sources
                && owner.editor != EditorTarget::Closed
        }
        TextField::SourceQuery => {
            source_list_present(owner)
                && owner.source_search_open()
                && !owner.sources.preview().is_empty()
        }
        TextField::CustomMinutes => {
            owner.navigation.page() == Page::Settings
                && owner.shell.settings_section() == SettingsSection::Overview
                && owner.raw.interval_choice() == 4
        }
        TextField::RecoveryFirst | TextField::RecoverySecond => {
            let required = if field == TextField::RecoveryFirst {
                1
            } else {
                2
            };
            owner.navigation.mode() == Mode::Power
                && owner.navigation.page() == Page::Recovery
                && owner.recovery_phase == super::recovery_workflow::HelperPhase::Editing
                && owner.raw.recovery_count() >= required
        }
    }
}

/// A field error belongs to its actual existing form, not a new error authority.
const fn text_slot(field: TextField) -> FeedbackSlot {
    match field {
        TextField::SourceLabel | TextField::SourceHost | TextField::SourcePort => {
            FeedbackSlot::EditorError
        }
        TextField::SourceQuery => FeedbackSlot::SourceQueryError,
        TextField::CustomMinutes => FeedbackSlot::IntervalError,
        TextField::RecoveryFirst | TextField::RecoverySecond => FeedbackSlot::RecoveryError,
    }
}

/// Snapshot an actual raw field on rejection; an unknown identity has no target.
fn retained_text(
    state: &RefCell<BindingState>,
    field: Result<TextField, DraftInputError>,
) -> SharedString {
    field.map_or_else(
        |_| SharedString::default(),
        |field| state.borrow().raw.get(field).into(),
    )
}

/// Bound original text before allocation into the raw owner; never trim/normalize.
fn edit_text(
    weak: &slint::Weak<AppWindow>,
    state: &RefCell<BindingState>,
    id: i32,
    value: &str,
) -> SharedString {
    let field = TextField::from_view(id);
    let Some(window) = weak.upgrade() else {
        return SharedString::default();
    };
    let Some(_projection) = ProjectionGuard::enter_raw_restore(state) else {
        return retained_text(state, field);
    };
    let (result, slot, report, reveal_recovery) = {
        let mut owner = state.borrow_mut();
        let report = input_idle(&owner);
        let reveal_recovery = report
            && matches!(
                field,
                Ok(TextField::RecoveryFirst | TextField::RecoverySecond)
            )
            && field.is_ok_and(|field| field_present(&owner, field));
        let slot = field.map_or(FeedbackSlot::EditorError, text_slot);
        let result = field.and_then(|field| {
            if !report || !field_present(&owner, field) {
                return Ok(false);
            }
            owner.raw.replace(field, value)?;
            if field == TextField::SourcePort
                && let Some(port) = &mut owner.port_draft
            {
                port.mark_port_edited();
            }
            Ok(true)
        });
        (result, slot, report, reveal_recovery)
    };
    project(&window, state);
    if report {
        match result {
            Ok(true) => write(&window, state, slot, None, "".into()),
            Ok(false) => show_error(&window, state, slot, None),
            Err(error) => {
                show_error(&window, state, slot, Some(error));
                if reveal_recovery {
                    window.invoke_reveal_recovery_form();
                }
            }
        }
    }
    retained_text(state, field)
}

/// The existing source protocol/manual-port owner supplies this projection.
fn source_protocol(owner: &BindingState) -> i32 {
    owner.port_draft.as_ref().map_or_else(
        || {
            owner
                .selected
                .and_then(|id| owner.sources.source(id.view_id()).ok())
                .map_or(Protocol::Nts, DraftSource::protocol)
                .view_choice()
        },
        |port| port.protocol().view_choice(),
    )
}

/// Resolve closed selector identity without indexing an untrusted integer.
fn retained_choice(owner: &BindingState, field: i32) -> i32 {
    match field {
        0 => owner.raw.interval_choice(),
        1 => owner.raw.recovery_count(),
        2 => source_protocol(owner),
        3 => owner.raw.policy_choice(),
        _ => -1,
    }
}

/// Check selector presence before any raw or protocol provenance mutation.
fn choice_present(owner: &BindingState, field: i32) -> bool {
    match field {
        0 => {
            owner.navigation.page() == Page::Settings
                && owner.shell.settings_section() == SettingsSection::Overview
        }
        1 => {
            owner.navigation.mode() == Mode::Power
                && owner.navigation.page() == Page::Recovery
                && owner.recovery_phase == super::recovery_workflow::HelperPhase::Editing
        }
        2 => field_present(owner, TextField::SourcePort),
        3 => {
            owner.navigation.mode() == Mode::Power
                && owner.navigation.page() == Page::Settings
                && owner.shell.settings_section() == SettingsSection::TimePolicy
                && owner.policy.phase() == PolicyPhase::Editing
                && owner.editor == EditorTarget::Closed
        }
        _ => false,
    }
}

/// Atomically preserve manual ports or replace only the untouched default.
///
/// # Errors
/// Rejects a missing editor protocol, invalid choice or inadmissible default port
/// without committing the candidate protocol/manual-port owner.
fn set_protocol(owner: &mut BindingState, choice: i32) -> Result<(), DraftInputError> {
    let Some(mut candidate) = owner.port_draft.clone() else {
        return Err(DraftInputError::InvalidChoice);
    };
    let update = candidate
        .switch_protocol(choice)
        .map_err(|_| DraftInputError::InvalidChoice)?;
    if let PortUpdate::Default(port) = update {
        owner
            .raw
            .replace(TextField::SourcePort, &port.to_string())?;
    }
    owner.port_draft = Some(candidate);
    Ok(())
}

/// Closed selector requests return authoritative values, including rejected input.
fn edit_choice(
    weak: &slint::Weak<AppWindow>,
    state: &RefCell<BindingState>,
    field: i32,
    value: i32,
) -> i32 {
    let Some(window) = weak.upgrade() else {
        return -1;
    };
    let Some(_projection) = ProjectionGuard::enter_raw_restore(state) else {
        return retained_choice(&state.borrow(), field);
    };
    let (result, report, reveal_recovery) = {
        let mut owner = state.borrow_mut();
        let report = input_idle(&owner);
        let reveal_recovery = report && field == 1 && choice_present(&owner, field);
        let result = if !(0..=3).contains(&field) {
            Err(DraftInputError::InvalidField)
        } else if !report || !choice_present(&owner, field) {
            Ok(false)
        } else {
            match field {
                0 => owner.raw.set_interval_choice(value),
                1 => owner.raw.set_recovery_count(value),
                2 => set_protocol(&mut owner, value),
                3 => owner.raw.set_policy_choice(value),
                _ => Err(DraftInputError::InvalidField),
            }
            .map(|()| true)
        };
        (result, report, reveal_recovery)
    };
    project(&window, state);
    let slot = match field {
        0 => FeedbackSlot::IntervalError,
        1 => FeedbackSlot::RecoveryError,
        3 => FeedbackSlot::PolicyError,
        _ => FeedbackSlot::EditorError,
    };
    if report {
        match result {
            Ok(true) => write(&window, state, slot, None, "".into()),
            Ok(false) => show_error(&window, state, slot, None),
            Err(error) => {
                show_error(&window, state, slot, Some(error));
                if reveal_recovery {
                    window.invoke_reveal_recovery_form();
                }
            }
        }
    }
    retained_choice(&state.borrow(), field)
}

/// Expose protocol changes to the legacy guarded callback without another owner.
pub(super) fn protocol_request(
    weak: &slint::Weak<AppWindow>,
    state: &RefCell<BindingState>,
    choice: i32,
) {
    // The returned value is already reprojected and not an operation acknowledgement.
    let _choice = edit_choice(weak, state, 2, choice);
}

/// Map fixed input failures to reviewed literal text, never to raw payload text.
fn show_error(
    window: &AppWindow,
    state: &RefCell<BindingState>,
    slot: FeedbackSlot,
    error: Option<DraftInputError>,
) {
    let copy = window.global::<RawCopy>();
    let (id, fallback) = match error {
        Some(DraftInputError::TooLong) => (MessageId::RAW_TOO_LONG, copy.get_too_long()),
        Some(DraftInputError::Character) => (MessageId::RAW_CHARACTER, copy.get_character()),
        Some(DraftInputError::InvalidField | DraftInputError::InvalidChoice) => {
            (MessageId::RAW_INVALID_OPTION, copy.get_invalid_option())
        }
        None => (MessageId::RAW_CONTEXT, copy.get_context()),
    };
    write(window, state, slot, Some(UiText::fixed(id)), fallback);
}

/// A submitted source payload is bound to the currently owned editor snapshot.
pub(super) fn source_matches(
    owner: &BindingState,
    raw: crate::source_drafts::RawSourceInput<'_>,
) -> bool {
    owner.raw.get(TextField::SourceLabel) == raw.label
        && owner.raw.get(TextField::SourceHost) == raw.host
        && owner.raw.get(TextField::SourcePort) == raw.port
        && source_protocol(owner) == raw.protocol_choice
}

/// Only an active custom field participates; preset payload text remains inert.
pub(super) fn interval_matches(owner: &BindingState, choice: i32, value: &str) -> bool {
    owner.raw.interval_choice() == choice
        && (choice != 4 || owner.raw.get(TextField::CustomMinutes) == value)
}

/// Unused helper arguments never authorize another count or enter a raw buffer.
pub(super) fn recovery_matches(
    owner: &BindingState,
    count: i32,
    first: &str,
    second: &str,
) -> bool {
    owner.raw.recovery_count() == count
        && (count == 0 || owner.raw.get(TextField::RecoveryFirst) == first)
        && (count < 2 || owner.raw.get(TextField::RecoverySecond) == second)
}

/// Reject stale or forged form payload without interpreting or retaining it.
pub(super) fn reject_stale(window: &AppWindow, state: &RefCell<BindingState>, slot: FeedbackSlot) {
    project(window, state);
    write(
        window,
        state,
        slot,
        Some(UiText::fixed(MessageId::RAW_STALE)),
        window.global::<RawCopy>().get_stale(),
    );
}

/// Current private permission before a widget touches clipboard or undo history.
fn action_permitted(state: &RefCell<BindingState>, field: i32, action: i32) -> bool {
    let owner = state.borrow();
    owner.initialized
        && !owner.projecting
        && input_idle(&owner)
        && (0..=6).contains(&action)
        && TextField::from_view(field).is_ok_and(|field| field_present(&owner, field))
}

/// Recognize stock menu keys without copying adversarial event text.
/// Oversized synthetic text is consumed fail-closed before a bounded scan.
fn menu_key(text: &str, shift: bool) -> bool {
    text.len() > crate::ui_drafts::MAX_RAW_TEXT_BYTES
        || text.contains(char::from(slint::platform::Key::Menu))
        || (shift && text.contains(char::from(slint::platform::Key::F10)))
}

/// Change only the actual source-list search context; stale widgets cannot toggle it.
fn search_requested(
    weak: &slint::Weak<AppWindow>,
    state: &RefCell<BindingState>,
    open: bool,
) -> bool {
    let Some(window) = weak.upgrade() else {
        return state.borrow().source_search_open();
    };
    let Some(_projection) = ProjectionGuard::enter(state) else {
        return state.borrow().source_search_open();
    };
    let allowed = {
        let owner = state.borrow();
        source_list_present(&owner) && (!open || !owner.sources.preview().is_empty())
    };
    if allowed {
        state.borrow_mut().source_search = SearchVisibility::from_open(open);
    }
    project(&window, state);
    state.borrow().source_search_open()
}

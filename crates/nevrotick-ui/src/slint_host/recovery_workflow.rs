//! Checked local helper preview and contextual Date recovery navigation.
//! Raw helper fields live in the bounded Rust owner; no live recovery exists.
#![forbid(unsafe_code)]

use super::catalog_workflow::{FeedbackSlot, write, write_many};
use crate::localization::{MessageId, presentation::UiText};

use std::{cell::RefCell, rc::Rc};

use slint::{ComponentHandle, SharedString, VecModel};

use super::shell_workflow;
use super::{AppWindow, BindingState, Page, ProjectionGuard, project_navigation, session_page};
use crate::presentation::{Mode, NavigationState, Page as SessionPage, UiError};
use crate::shell::{SettingsSection, ShellRoute};

/// Fixed rejection for helper callbacks outside their checked page/phase.
const PAGE_REQUIRED: RecoveryProblem = RecoveryProblem {
    message: UiText::fixed(MessageId::RUST_RECOVERY_PAGE_REQUIRED),
    fixture: "Open Date recovery before editing its helper preview.",
};
/// Fixed instruction before leaving raw helper edits or changing other phases.
const LEAVE_REQUIRED: RecoveryProblem = RecoveryProblem {
    message: UiText::fixed(MessageId::RUST_RECOVERY_LEAVE_REQUIRED),
    fixture: "Choose Keep draft, Discard edits, or Stay before continuing.",
};
/// Fixed rejection when a review-only request did not originate in trusted review.
const REVIEW_REQUIRED: RecoveryProblem = RecoveryProblem {
    message: UiText::fixed(MessageId::RUST_RECOVERY_REVIEW_REQUIRED),
    fixture: "Review a valid helper preview before using this action.",
};
/// Fixed rejection when editing is requested while trusted review remains open.
const EDIT_REQUIRED: RecoveryProblem = RecoveryProblem {
    message: UiText::fixed(MessageId::RUST_RECOVERY_EDIT_REQUIRED),
    fixture: "Choose Keep editing before changing helper fields.",
};

/// Mutually exclusive local helper-edit and accepted-review phases.
#[derive(Clone, Copy, Eq, PartialEq)]
pub(super) enum HelperPhase {
    /// Raw helper fields may be admitted or cancelled.
    Editing,
    /// An accepted local preview is being reviewed, with fields locked.
    Review,
}

/// A code-owned pending destination, never derived from a view string or URI.
#[derive(Clone, Copy)]
pub(super) enum RecoveryLeave {
    /// Restore the exact mode/page memory captured on entry.
    Back,
    /// An ordinary typed page selection, preserving the user's current mode.
    Page(SessionPage),
    /// An ordinary explicit complexity-mode request.
    Mode(Mode),
    /// Preserve a peer route's section and explicit mode until its acceptance.
    Shell {
        /// Closed destination and named Settings section, if applicable.
        route: ShellRoute,
        /// Whether the accepted peer request explicitly enters Power mode.
        power: bool,
    },
}

/// Presentation-only scroll range, not a time/network/policy budget.
const MAX_SCROLL_DISTANCE: f32 = 1_000_000.0;

/// Privately admitted finite logical-pixel position; malformed input means top.
#[derive(Clone, Copy)]
struct ScrollPosition(f32);

impl ScrollPosition {
    /// Capture only bounded ordinary scrolling; no invalid value is restored.
    fn capture(window: &AppWindow) -> Self {
        Self::admit(window.get_page_scroll_y())
    }

    /// Admit only presentation coordinates; invalid values safely restore top.
    fn admit(raw: f32) -> Self {
        if raw.is_finite() && (-MAX_SCROLL_DISTANCE..=0.0).contains(&raw) {
            Self(raw)
        } else {
            Self(0.0)
        }
    }
}

/// Exact code-owned origin with one bounded UI coordinate, never effect authority.
#[derive(Clone)]
pub(super) struct RecoveryContext {
    /// Includes the original mode, shared page and remembered Power page.
    navigation: NavigationState,
    /// Vertical position captured before entering the recovery details.
    scroll: ScrollPosition,
}

/// Temporary raw getter snapshot; no rejected bytes survive the receiving call.
struct RawDraft {
    /// Signed selector, admitted only by the recovery model at submission.
    count: i32,
    /// View-owned raw first identity; a shared string clone copies no host bytes.
    first: SharedString,
    /// View-owned raw second identity, including a currently unused draft.
    second: SharedString,
}

impl RawDraft {
    /// Copy only bounded authoritative raw fields, never spoofable view getters.
    fn from_owner(owner: &BindingState) -> Self {
        Self {
            count: owner.raw.recovery_count(),
            first: owner
                .raw
                .get(crate::ui_drafts::TextField::RecoveryFirst)
                .into(),
            second: owner
                .raw
                .get(crate::ui_drafts::TextField::RecoverySecond)
                .into(),
        }
    }

    /// Detect unsaved raw editing; accepted review has already admitted its value.
    fn dirty(&self, owner: &BindingState) -> bool {
        if owner.recovery_phase == HelperPhase::Review {
            return false;
        }
        let accepted = AcceptedDraft::from_owner(owner);
        self.count != accepted.count
            || self.first.as_str() != accepted.first
            || self.second.as_str() != accepted.second
    }
}

/// Bounded accepted raw reset, used only at initialization/Cancel/Discard.
struct AcceptedDraft {
    /// At most two helpers, guaranteed by the private recovery constructor.
    count: i32,
    /// Checked admitted first identity, or empty when Off.
    first: String,
    /// Checked admitted second identity, or empty when fewer than two are used.
    second: String,
}

impl AcceptedDraft {
    /// Copy only checked helper identities; no original/rejected raw text is used.
    fn from_owner(owner: &BindingState) -> Self {
        let helpers = owner.recovery.preview().helpers();
        // RecoveryPreview's private constructors guarantee exactly 0, 1 or 2.
        let count = match helpers.len() {
            0 => 0,
            1 => 1,
            _ => 2,
        };
        Self {
            count,
            first: helpers
                .first()
                .map_or("", |helper| helper.host())
                .to_owned(),
            second: helpers.get(1).map_or("", |helper| helper.host()).to_owned(),
        }
    }

    /// Reset raw widget fields after the accepted-state borrow has ended.
    fn apply(self, window: &AppWindow, state: &RefCell<BindingState>) -> bool {
        super::raw_workflow::reset(
            window,
            state,
            crate::ui_drafts::ResetInput {
                recovery: Some(crate::ui_drafts::RecoveryFields {
                    count: self.count,
                    first: &self.first,
                    second: &self.second,
                }),
                ..crate::ui_drafts::ResetInput::default()
            },
        )
    }
}

/// Owned projection of helper identities/phase/context, independent of raw drafts.
struct RecoveryProjection {
    /// At most two literal checked hosts; they are not dates or authentication.
    hosts: Vec<SharedString>,
    /// Private review phase projected for display, never read back for admission.
    review_open: bool,
    /// A pending trusted leave request exists; view spoofing cannot create one.
    leave_open: bool,
    /// Code-owned context name; catalog/input bytes cannot choose a destination.
    back_label: &'static str,
    /// Code-owned return destination selected by the retained navigation context.
    back_page: SessionPage,
}

impl RecoveryProjection {
    /// Copy accepted evidence-free values before toolkit callbacks can run.
    fn from_owner(owner: &BindingState) -> Self {
        let back_label = match owner
            .recovery_return
            .as_ref()
            .map(|context| context.navigation.page())
        {
            Some(SessionPage::Status) => "Back to Status",
            Some(SessionPage::Sources) => "Back to Sources",
            Some(SessionPage::Statistics) => "Back to Statistics",
            Some(SessionPage::Diagnostics) => "Back to Diagnostics",
            Some(SessionPage::About) => "Back to About",
            Some(SessionPage::Settings | SessionPage::Recovery) | None => "Back to Settings",
        };
        Self {
            hosts: owner
                .recovery
                .preview()
                .helpers()
                .iter()
                .map(|helper| helper.host().into())
                .collect(),
            review_open: owner.recovery_phase == HelperPhase::Review,
            leave_open: owner.recovery_leave.is_some(),
            back_label,
            back_page: owner
                .recovery_return
                .as_ref()
                .map_or(SessionPage::Settings, |origin| origin.navigation.page()),
        }
    }

    /// Publish only inert identity/phase/context properties under the held guard.
    fn apply(self, window: &AppWindow, state: &RefCell<BindingState>) {
        window.set_recovery_preview_hosts(Rc::new(VecModel::from(self.hosts)).into());
        window.set_recovery_review_open(self.review_open);
        window.set_recovery_leave_open(self.leave_open);
        write(
            window,
            state,
            FeedbackSlot::RecoveryBackLabel,
            Some(UiText::recovery_back(self.back_page)),
            self.back_label.into(),
        );
    }
}

/// Initialize Off/raw empty without constructing another accepted helper owner.
pub(super) fn initialize(window: &AppWindow, state: &RefCell<BindingState>) {
    let (accepted, projection) = {
        let owner = state.borrow();
        (
            AcceptedDraft::from_owner(&owner),
            RecoveryProjection::from_owner(&owner),
        )
    };
    if !accepted.apply(window, state) {
        return;
    }
    projection.apply(window, state);
    write(window, state, FeedbackSlot::RecoveryError, None, "".into());
    write(
        window,
        state,
        FeedbackSlot::RecoveryFeedback,
        None,
        "".into(),
    );
}

/// Bind typed recovery actions with weak captures under the shared root lifecycle.
pub(super) fn install_callbacks(window: &AppWindow, state: &Rc<RefCell<BindingState>>) {
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_recovery_open_requested(move || open(&weak, &owner));
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window
        .on_recovery_back_requested(move || navigation_request(&weak, &owner, RecoveryLeave::Back));
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_recovery_update_requested(move |count, first, second| {
        update(&weak, &owner, count, &first, &second);
    });
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_recovery_cancel_requested(move || local_request(&weak, &owner, LocalRequest::Cancel));
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_recovery_keep_editing_requested(move || {
        local_request(&weak, &owner, LocalRequest::KeepEditing);
    });
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window
        .on_recovery_discard_requested(move || local_request(&weak, &owner, LocalRequest::Discard));
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_recovery_leave_keep_requested(move || {
        resolve_leave(&weak, &owner, LeaveResolution::Keep);
    });
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_recovery_leave_discard_requested(move || {
        resolve_leave(&weak, &owner, LeaveResolution::Discard);
    });
    let weak = window.as_weak();
    let owner = Rc::clone(state);
    window.on_recovery_leave_stay_requested(move || {
        resolve_leave(&weak, &owner, LeaveResolution::Stay);
    });
}

/// Enter Power details explicitly, retaining the exact originating mode/page.
fn open(weak: &slint::Weak<AppWindow>, state: &RefCell<BindingState>) {
    let Some(window) = weak.upgrade() else {
        return;
    };
    let Some(_projection) = ProjectionGuard::enter(state) else {
        return;
    };
    let scroll = ScrollPosition::capture(&window);
    let (projection, mode, page, scroll_update, error) = {
        let mut owner = state.borrow_mut();
        let result = if owner.recovery_leave.is_some() {
            Err(LEAVE_REQUIRED)
        } else {
            let original = owner.navigation.clone();
            let mut candidate = original.clone();
            candidate.set_mode(Mode::Power);
            candidate
                .navigate(SessionPage::Recovery)
                .map_err(RecoveryProblem::navigation)
                .map(|()| {
                    let entering = original.page() != SessionPage::Recovery;
                    if entering {
                        owner.recovery_return = Some(RecoveryContext {
                            navigation: original,
                            scroll,
                        });
                    }
                    owner.shell.reconcile_mode(candidate.mode());
                    owner.navigation = candidate;
                    if entering { Some(0.0) } else { None }
                })
        };
        let (scroll_update, error) = match result {
            Ok(scroll) => (scroll, None),
            Err(error) => (None, Some(error)),
        };
        (
            RecoveryProjection::from_owner(&owner),
            owner.navigation.mode(),
            owner.navigation.page(),
            scroll_update,
            error,
        )
    };
    projection.apply(&window, state);
    project_navigation(&window, mode, page);
    shell_workflow::project_section(&window, state);
    if let Some(scroll) = scroll_update {
        window.set_page_scroll_y(scroll);
    }
    if let Some(error) = error {
        write(
            &window,
            state,
            FeedbackSlot::RecoveryError,
            Some(error.message),
            error.fixture.into(),
        );
    }
}

/// Route the ordinary closed page request through raw-draft leave semantics.
pub(super) fn navigate(weak: &slint::Weak<AppWindow>, state: &RefCell<BindingState>, page: Page) {
    navigation_request(weak, state, RecoveryLeave::Page(session_page(page)));
}

/// Mode requests retain the old navigation semantics unless raw recovery is dirty.
pub(super) fn change_mode(
    weak: &slint::Weak<AppWindow>,
    state: &RefCell<BindingState>,
    power: bool,
) {
    navigation_request(
        weak,
        state,
        RecoveryLeave::Mode(if power { Mode::Power } else { Mode::Simple }),
    );
}

/// Route a peer action only after its own modal/guard has been released.
pub(super) fn request_shell_route(
    weak: &slint::Weak<AppWindow>,
    state: &RefCell<BindingState>,
    route: ShellRoute,
    power: bool,
) {
    navigation_request(weak, state, RecoveryLeave::Shell { route, power });
}

/// Project an accepted peer destination and focus only an actual service anchor.
fn publish_shell_route(
    window: &AppWindow,
    state: &RefCell<BindingState>,
    route: Option<ShellRoute>,
) {
    if let Some(route) = route {
        shell_workflow::project_route(window, state);
        if route == ShellRoute::Settings(SettingsSection::BackgroundService) {
            window.invoke_focus_service();
        } else if matches!(route, ShellRoute::Settings(_)) {
            window.invoke_focus_settings_section();
        }
    }
}

/// Identify only requests that actually leave the current Recovery view.
fn leaves_recovery(owner: &BindingState, target: RecoveryLeave) -> bool {
    if owner.navigation.page() != SessionPage::Recovery {
        return false;
    }
    match target {
        RecoveryLeave::Back | RecoveryLeave::Shell { .. } => true,
        RecoveryLeave::Page(page) => page != SessionPage::Recovery,
        RecoveryLeave::Mode(mode) => mode != owner.navigation.mode(),
    }
}

/// Perform a checked destination; return only a bounded scroll reset/restore.
///
/// # Errors
/// Rejects an unavailable return context or a forbidden page transition before
/// replacing accepted navigation; no service or clock operation is performed.
fn perform_navigation(
    owner: &mut BindingState,
    target: RecoveryLeave,
    scroll: ScrollPosition,
) -> Result<Option<f32>, RecoveryProblem> {
    if let RecoveryLeave::Back = target {
        if owner.navigation.page() != SessionPage::Recovery {
            return Err(PAGE_REQUIRED);
        }
        let (candidate, position) = if let Some(origin) = &owner.recovery_return {
            (origin.navigation.clone(), origin.scroll.0)
        } else {
            let mut fallback = NavigationState::new();
            fallback.set_mode(Mode::Power);
            fallback
                .navigate(SessionPage::Settings)
                .map_err(RecoveryProblem::navigation)?;
            (fallback, 0.0)
        };
        owner.shell.reconcile_mode(candidate.mode());
        owner.navigation = candidate;
        owner.recovery_return = None;
        return Ok(Some(position));
    }
    if let RecoveryLeave::Shell { route, power } = target {
        let original_page = owner.navigation.page();
        shell_workflow::commit_route(owner, route, power).map_err(RecoveryProblem::navigation)?;
        return Ok(if owner.navigation.page() == original_page {
            None
        } else {
            Some(0.0)
        });
    }
    let original = owner.navigation.clone();
    let mut candidate = original.clone();
    match target {
        RecoveryLeave::Page(page) => candidate
            .navigate(page)
            .map_err(RecoveryProblem::navigation)?,
        RecoveryLeave::Mode(mode) => candidate.set_mode(mode),
        RecoveryLeave::Back | RecoveryLeave::Shell { .. } => {}
    }
    let changed_page = original.page() != candidate.page();
    if candidate.page() == SessionPage::Recovery && changed_page {
        owner.recovery_return = Some(RecoveryContext {
            navigation: original,
            scroll,
        });
    }
    owner.shell.reconcile_mode(candidate.mode());
    owner.navigation = candidate;
    Ok(if changed_page { Some(0.0) } else { None })
}

/// Capture the first leave intent; a second request cannot silently redirect it.
fn navigation_request(
    weak: &slint::Weak<AppWindow>,
    state: &RefCell<BindingState>,
    target: RecoveryLeave,
) {
    let Some(window) = weak.upgrade() else {
        return;
    };
    let Some(_projection) = ProjectionGuard::enter(state) else {
        return;
    };
    let raw = RawDraft::from_owner(&state.borrow());
    let scroll = ScrollPosition::capture(&window);
    let (projection, mode, page, scroll_update, accepted_shell, error) = {
        let mut owner = state.borrow_mut();
        let result = if owner.recovery_leave.is_some() {
            Err(LEAVE_REQUIRED)
        } else if leaves_recovery(&owner, target) && raw.dirty(&owner) {
            owner.recovery_leave = Some(target);
            Ok(None)
        } else {
            perform_navigation(&mut owner, target, scroll)
        };
        let accepted_shell = if result.is_ok() && owner.recovery_leave.is_none() {
            match target {
                RecoveryLeave::Shell { route, .. } => Some(route),
                _ => None,
            }
        } else {
            None
        };
        let (scroll_update, error) = match result {
            Ok(scroll) => (scroll, None),
            Err(error) => (None, Some(error)),
        };
        (
            RecoveryProjection::from_owner(&owner),
            owner.navigation.mode(),
            owner.navigation.page(),
            scroll_update,
            accepted_shell,
            error,
        )
    };
    projection.apply(&window, state);
    project_navigation(&window, mode, page);
    shell_workflow::project_section(&window, state);
    if let Some(scroll) = scroll_update {
        window.set_page_scroll_y(scroll);
    }
    publish_shell_route(&window, state, accepted_shell);
    if let Some(error) = error {
        write_many(
            &window,
            state,
            &[
                (
                    FeedbackSlot::RecoveryError,
                    Some(error.message),
                    error.fixture.into(),
                ),
                (
                    FeedbackSlot::EditorError,
                    Some(error.message),
                    error.fixture.into(),
                ),
            ],
        );
    }
}

/// Require actual recovery context and no unresolved local leave decision.
///
/// # Errors
/// Returns fixed guidance when Recovery is inactive or a leave choice is pending.
fn ready(owner: &BindingState) -> Result<(), RecoveryProblem> {
    if owner.navigation.page() != SessionPage::Recovery {
        Err(PAGE_REQUIRED)
    } else if owner.recovery_leave.is_some() {
        Err(LEAVE_REQUIRED)
    } else {
        Ok(())
    }
}

/// Admit all raw helper fields; rejection preserves preview, phase and raw drafts.
fn update(
    weak: &slint::Weak<AppWindow>,
    state: &RefCell<BindingState>,
    count: i32,
    first: &str,
    second: &str,
) {
    let Some(window) = weak.upgrade() else {
        return;
    };
    let Some(_projection) = ProjectionGuard::enter(state) else {
        return;
    };
    let editing_present = {
        let owner = state.borrow();
        owner.navigation.page() == SessionPage::Recovery
            && owner.navigation.mode() == Mode::Power
            && owner.recovery_phase == HelperPhase::Editing
    };
    if editing_present
        && !super::raw_workflow::recovery_matches(&state.borrow(), count, first, second)
    {
        super::raw_workflow::reject_stale(&window, state, FeedbackSlot::RecoveryError);
        window.invoke_reveal_recovery_form();
        return;
    }
    let (projection, result) = {
        let mut owner = state.borrow_mut();
        let result = ready(&owner)
            .map_err(|problem| (problem.message, problem.fixture.to_owned()))
            .and_then(|()| {
                if owner.recovery_phase == HelperPhase::Review {
                    Err((EDIT_REQUIRED.message, EDIT_REQUIRED.fixture.to_owned()))
                } else {
                    owner
                        .recovery
                        .update(count, first, second)
                        .map_err(|error| (UiText::recovery_error(error), error.to_string()))
                        .map(|()| {
                            owner.recovery_phase = HelperPhase::Review;
                        })
                }
            });
        (RecoveryProjection::from_owner(&owner), result)
    };
    projection.apply(&window, state);
    match result {
        Ok(()) => {
            write(&window, state, FeedbackSlot::RecoveryError, None, "".into());
            write(
                &window,
                state,
                FeedbackSlot::RecoveryFeedback,
                Some(UiText::fixed(MessageId::RUST_HELPER_PREVIEW_RETAINED)),
                "Helper preview retained in memory only. Active policy and services are unchanged."
                    .into(),
            );
        }
        Err((message, fixture)) => {
            super::raw_workflow::project(&window, state);
            write_many(
                &window,
                state,
                &[
                    (FeedbackSlot::RecoveryError, Some(message), fixture.into()),
                    (FeedbackSlot::RecoveryFeedback, None, "".into()),
                ],
            );
        }
    }
    if editing_present {
        // This receiving context was checked before any accepted phase change.
        // Typed feedback and all state borrows end before toolkit focus/scroll.
        window.invoke_reveal_recovery_form();
    }
}

/// Closed local phase actions; Cancel and Discard intentionally differ.
#[derive(Clone, Copy)]
enum LocalRequest {
    /// Reset unsubmitted raw fields to the previously accepted local preview.
    Cancel,
    /// Retain exact raw fields while leaving the trusted review phase.
    KeepEditing,
    /// Clear the accepted local helper preview to Off and reset its raw fields.
    Discard,
}

/// Guard phase actions independently of any spoofed view review-open property.
fn local_request(
    weak: &slint::Weak<AppWindow>,
    state: &RefCell<BindingState>,
    request: LocalRequest,
) {
    let Some(window) = weak.upgrade() else {
        return;
    };
    let Some(_projection) = ProjectionGuard::enter(state) else {
        return;
    };
    let (projection, reset, result) = {
        let mut owner = state.borrow_mut();
        let result = ready(&owner).and_then(|()| match request {
            LocalRequest::Cancel if owner.recovery_phase == HelperPhase::Review => {
                Err(EDIT_REQUIRED)
            }
            LocalRequest::KeepEditing | LocalRequest::Discard
                if owner.recovery_phase != HelperPhase::Review =>
            {
                Err(REVIEW_REQUIRED)
            }
            LocalRequest::Cancel | LocalRequest::KeepEditing | LocalRequest::Discard => Ok(()),
        });
        let reset = if result.is_ok() {
            if let LocalRequest::Discard = request {
                owner.recovery.discard();
            }
            owner.recovery_phase = HelperPhase::Editing;
            if let LocalRequest::KeepEditing = request {
                None
            } else {
                Some(AcceptedDraft::from_owner(&owner))
            }
        } else {
            None
        };
        (RecoveryProjection::from_owner(&owner), reset, result)
    };
    projection.apply(&window, state);
    if let Some(reset) = reset
        && !reset.apply(&window, state)
    {
        return;
    }
    match result {
        Ok(()) => {
            write(&window, state, FeedbackSlot::RecoveryError, None, "".into());
            let feedback = match request {
                LocalRequest::Cancel => {
                    "Unsubmitted helper edits discarded. The local preview is unchanged."
                }
                LocalRequest::KeepEditing => "Editing the retained local helper preview.",
                LocalRequest::Discard => {
                    "Helper preview discarded. Active policy and services are unchanged."
                }
            };
            let message = match request {
                LocalRequest::Cancel => MessageId::RUST_HELPER_RAW_DISCARDED,
                LocalRequest::KeepEditing => MessageId::RUST_HELPER_PREVIEW_EDIT,
                LocalRequest::Discard => MessageId::RUST_HELPER_PREVIEW_DISCARDED,
            };
            write(
                &window,
                state,
                FeedbackSlot::RecoveryFeedback,
                Some(UiText::fixed(message)),
                feedback.into(),
            );
        }
        Err(error) => write(
            &window,
            state,
            FeedbackSlot::RecoveryError,
            Some(error.message),
            error.fixture.into(),
        ),
    }
}

/// Resolve only a trusted pending intent; view leave-open cannot invent one.
#[derive(Clone, Copy)]
enum LeaveResolution {
    /// Keep raw fields and move to the previously requested destination.
    Keep,
    /// Reset raw edits to the accepted preview, then leave to that destination.
    Discard,
    /// Drop only the leave intent and keep editing the same exact raw fields.
    Stay,
}

/// Retain preview and unrelated source/preference state through all resolutions.
fn resolve_leave(
    weak: &slint::Weak<AppWindow>,
    state: &RefCell<BindingState>,
    resolution: LeaveResolution,
) {
    let Some(window) = weak.upgrade() else {
        return;
    };
    let Some(_projection) = ProjectionGuard::enter_recovery(state) else {
        return;
    };
    if (state.borrow().copy.failed() || state.borrow().raw_failed())
        && !matches!(resolution, LeaveResolution::Stay)
    {
        return;
    }
    let scroll = ScrollPosition::capture(&window);
    let (projection, reset, mode, page, accepted_shell, result) = {
        let mut owner = state.borrow_mut();
        let requested_shell = match owner.recovery_leave {
            Some(RecoveryLeave::Shell { route, .. }) => Some(route),
            _ => None,
        };
        let result = if owner.navigation.page() == SessionPage::Recovery {
            owner
                .recovery_leave
                .ok_or(LEAVE_REQUIRED)
                .and_then(|target| {
                    if let LeaveResolution::Stay = resolution {
                        owner.recovery_leave = None;
                        Ok(None)
                    } else {
                        perform_navigation(&mut owner, target, scroll).inspect(|_| {
                            owner.recovery_leave = None;
                        })
                    }
                })
        } else {
            Err(PAGE_REQUIRED)
        };
        let accepted_shell = if result.is_ok() && !matches!(resolution, LeaveResolution::Stay) {
            requested_shell
        } else {
            None
        };
        let reset = if result.is_ok() && matches!(resolution, LeaveResolution::Discard) {
            Some(AcceptedDraft::from_owner(&owner))
        } else {
            None
        };
        (
            RecoveryProjection::from_owner(&owner),
            reset,
            owner.navigation.mode(),
            owner.navigation.page(),
            accepted_shell,
            result,
        )
    };
    projection.apply(&window, state);
    if let Some(reset) = reset
        && !reset.apply(&window, state)
    {
        return;
    }
    project_navigation(&window, mode, page);
    shell_workflow::project_section(&window, state);
    publish_shell_route(&window, state, accepted_shell);
    match result {
        Err(error) => write(
            &window,
            state,
            FeedbackSlot::RecoveryError,
            Some(error.message),
            error.fixture.into(),
        ),
        Ok(scroll_update) => {
            if let Some(scroll) = scroll_update {
                window.set_page_scroll_y(scroll);
            }
            if matches!(resolution, LeaveResolution::Stay) {
                // The hidden modal loses toolkit focus; explicitly restore the
                // current Recovery context, then undo focus-induced scrolling.
                window.invoke_focus_recovery_content();
                window.set_page_scroll_y(scroll.0);
            }
            if matches!(resolution, LeaveResolution::Discard) {
                write(&window, state, FeedbackSlot::RecoveryError, None, "".into());
                write(
                    &window,
                    state,
                    FeedbackSlot::RecoveryFeedback,
                    Some(UiText::fixed(MessageId::RUST_HELPER_RAW_DISCARDED)),
                    "Unsubmitted helper edits discarded. The local preview is unchanged.".into(),
                );
            }
        }
    }
}

#[cfg(test)]
mod scroll_tests {
    //! Adversarial geometry is bounded independently of any renderer behavior.

    use super::{MAX_SCROLL_DISTANCE, ScrollPosition};

    /// Invalid UI geometry cannot enter a retained Back context.
    ///
    /// # Panics
    /// Fails when `NaN`/infinity/positive/overbudget values can be restored.
    #[test]
    fn captured_scroll_is_finite_bounded_and_retains_ordinary_neighbors() {
        assert_eq!(
            ScrollPosition::admit(-420.0).0.to_bits(),
            (-420.0_f32).to_bits()
        );
        assert_eq!(
            ScrollPosition::admit(-MAX_SCROLL_DISTANCE).0.to_bits(),
            (-MAX_SCROLL_DISTANCE).to_bits()
        );
        assert_eq!(ScrollPosition::admit(0.0).0.to_bits(), 0.0_f32.to_bits());
        for raw in [
            f32::NAN,
            f32::INFINITY,
            f32::NEG_INFINITY,
            0.5,
            -MAX_SCROLL_DISTANCE - 1.0,
        ] {
            assert_eq!(ScrollPosition::admit(raw).0.to_bits(), 0.0_f32.to_bits());
        }
    }
}

/// Closed recovery/navigation rejection retaining no rejected input.
#[derive(Clone, Copy)]
struct RecoveryProblem {
    /// Protected message identity, independent of rendered English.
    message: UiText,
    /// Maintained engineering fixture text; never an approval or runtime fallback.
    fixture: &'static str,
}
impl RecoveryProblem {
    /// Preserve the existing redacted navigation reason as a typed fact.
    const fn navigation(error: UiError) -> Self {
        Self {
            message: UiText::ui_error(error),
            fixture: error.text(),
        }
    }
}

/// Resolve only the pending leave intent before the trusted emergency quit view.
/// The caller owns `enter_shell`, and raw drafts, accepted helpers/navigation/focus
/// remain unchanged. No ordinary failed locale payload reaches this path.
pub(super) fn stay_for_emergency(window: &AppWindow, state: &RefCell<BindingState>) {
    let projection = {
        let mut owner = state.borrow_mut();
        if (!owner.copy.failed() && !owner.raw_failed()) || !owner.projecting {
            return;
        }
        owner.recovery_leave = None;
        RecoveryProjection::from_owner(&owner)
    };
    projection.apply(window, state);
}

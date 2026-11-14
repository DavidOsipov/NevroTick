// SPDX-License-Identifier: GPL-3.0-only
//! Windows host custody for unelevated tray/toast observers only.
//!
//! Root state stays on the UI thread. A native producer can enqueue only one
//! code-owned wake; the callback obtains the current generation's weak owner and
//! rechecks the current receiving context. This module owns no second dialog or
//! shell controller and cannot invoke a service, clock, network or elevation.
#![forbid(unsafe_code)]

use std::{
    cell::RefCell,
    fmt,
    rc::{Rc, Weak},
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, Instant},
};

use slint::{ComponentHandle, Timer, TimerMode};

use super::{
    AppWindow, BindingState, InlineOwner, ProjectionGuard,
    native_lifecycle::{StopBarrier, StopFacts, StopProgress, StopPurpose, TrayIntent, WakeGate},
    native_notification_workflow::{self, NoticeContent},
};
use crate::{
    localization::{Snapshot, UiText},
    native_notifications_windows::{CloseOutcome, IdentityMode},
    native_tray::{
        NativeTray, PreparedTray, TrayBridge, TrayError, TrayFinish, TrayPixels, UiWake,
    },
    notifications_ui::{ObserverPresence, native_delivery::ActivationResolution},
    shell::{AdapterAvailability, Observation},
    status_ui::{StatusCheckIntent, StatusKind, StatusPresentationPort, StatusView},
    tray_ui::{FixtureTrayMenu, TrayCheckBlock, TrayCheckContext, TrayInteraction, TrayItemId},
};

/// Code-owned poll cadence; it sends no measurement or service request.
const POLL: Duration = Duration::from_millis(100);
/// After this local wait, a pending stop is reported as timed out, never completed.
const STOP_DEADLINE: Duration = Duration::from_secs(5);
/// Process-local registration sequence; exhaustion refuses a new native owner.
static NEXT_GENERATION: AtomicU64 = AtomicU64::new(1);

thread_local! {
    /// Weak UI-local association, never copied to a worker or read through LPARAM.
    static CURRENT: RefCell<Option<Registration>> = const { RefCell::new(None) };
}

/// Current scalar identity and weak UI owner, used only on the UI thread.
struct Registration {
    /// Fresh identity; old callbacks cannot drain a replacement window's ports.
    generation: u64,
    /// A weak component cannot keep its own callback cycle alive.
    window: slint::Weak<AppWindow>,
    /// Root Rc/RefCell is retained only by its actual UI-thread owner.
    state: Weak<RefCell<BindingState>>,
}

/// Fixed native-host installation rejection, independent of external text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeHostError {
    /// One production native owner is already installed on this UI thread.
    AlreadyInstalled,
    /// A fresh process-local wake generation could not be allocated.
    GenerationExhausted,
    /// The public UI timer could not be installed; no worker may start.
    TimerUnavailable,
}
impl fmt::Display for NativeHostError {
    /// Never include a native pointer, contributed value, identity or path.
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        output.write_str("The native observer host could not be installed.")
    }
}
impl std::error::Error for NativeHostError {}

/// Post-only shared tray/toast wake; it contains no Root/component handle.
struct NativeWake {
    /// Code-owned current host identity, not an action or external parameter.
    generation: u64,
    /// One actually enqueued callback across both native producers.
    gate: Arc<WakeGate>,
}
impl UiWake for NativeWake {
    /// The public post port runs only later on the event-loop/UI thread.
    fn enqueue(&self) -> bool {
        self.gate.request(|| {
            let generation = self.generation;
            let gate = Arc::clone(&self.gate);
            slint::invoke_from_event_loop(move || {
                gate.begin_drain();
                dispatch(generation);
            })
            .is_ok()
        })
    }
}

/// One actual native tray owner and its last complete checked projection.
struct OwnedTray {
    /// Owns exactly one worker until terminal cleanup is observed.
    port: NativeTray,
    /// Bounded mailbox/readiness facts from that exact observer lifetime.
    bridge: Arc<TrayBridge>,
    /// Avoid native modifications when unchanged polls read the same model.
    projection: PreparedTray,
}

/// Existing UI-only stop intent and actual local wait start.
struct Stopping {
    /// Only adapter cleanup custody; user consent belongs to `ShellController`.
    barrier: StopBarrier,
    /// A local monotonic deadline supplies no native completion fact.
    started: Instant,
    /// Partial notification cleanup is a failure, not asynchronous progress.
    notifications_failed: bool,
    /// A failed native wake is a known failed stop, never mere elapsed timeout.
    tray_post_failed: bool,
}

/// Independent one-shot presentation receipts for the current stop attempt.
#[derive(Default)]
struct StopNotices {
    /// Timeout wording has already been projected for this pending attempt.
    timeout_reported: bool,
    /// Failure wording has already released the existing shell exit request.
    failure_reported: bool,
}

/// UI-thread-only native resources, absent from ordinary fixture construction.
pub(super) struct NativeBindingState {
    /// Exact UI registration owner, removed only when this generation retires.
    generation: u64,
    /// Shared post-only wake provided to both native adapters.
    wake: Arc<NativeWake>,
    /// Timer remains alive while native cleanup is pending or uncertain.
    timer: Timer,
    /// None until the accepted tray opt-in is actually requested.
    tray: Option<OwnedTray>,
    /// Accepted preference edges cannot disappear between timer polls.
    tray_intent: TrayIntent,
    /// Failed startup/update is not automatically retried every timer tick.
    tray_fault: bool,
    /// Current native stop custody, never a second UI confirmation.
    stopping: Option<Stopping>,
    /// One-shot timeout/failure receipts, independent of actual native cleanup.
    notices: StopNotices,
    /// Terminal native cleanup permits only the already-authorized exit attempt.
    exit_ready: bool,
    /// A failed restore cannot trigger automatic native removal while hidden.
    stop_restore_failed: bool,
}

impl NativeBindingState {
    /// Retain only a bounded accepted preference receipt; this performs no OS call.
    pub(super) fn accept_tray_preference(&mut self, previous: bool, next: bool) {
        self.tray_intent.accept(previous, next);
    }
}

impl Drop for NativeBindingState {
    /// Best effort on abnormal owner destruction; never a completion claim.
    fn drop(&mut self) {
        self.timer.stop();
        CURRENT.with(|slot| {
            let mut slot = slot.borrow_mut();
            if slot
                .as_ref()
                .is_some_and(|entry| entry.generation == self.generation)
            {
                *slot = None;
            }
        });
        if let Some(tray) = &self.tray {
            let _stop_requested = tray.port.request_stop();
        }
    }
}

/// Install only the production UI-thread owner; this starts no tray worker.
///
/// # Errors
/// Refuse a duplicate owner, exhausted generation or unavailable UI timer.
pub(super) fn install(
    window: &AppWindow,
    state: &Rc<RefCell<BindingState>>,
) -> Result<(), NativeHostError> {
    if CURRENT.with(|slot| slot.borrow().is_some()) {
        return Err(NativeHostError::AlreadyInstalled);
    }
    let generation = NEXT_GENERATION
        .try_update(Ordering::AcqRel, Ordering::Acquire, |next| {
            next.checked_add(1)
        })
        .map_err(|_| NativeHostError::GenerationExhausted)?;
    let wake = Arc::new(NativeWake {
        generation,
        gate: Arc::new(WakeGate::default()),
    });
    let timer = Timer::default();
    let weak_state = Rc::downgrade(state);
    let weak_window = window.as_weak();
    timer.start(TimerMode::Repeated, POLL, move || {
        let (Some(window), Some(state)) = (weak_window.upgrade(), weak_state.upgrade()) else {
            return;
        };
        tick(&window, &state);
    });
    if !timer.running() {
        return Err(NativeHostError::TimerUnavailable);
    }
    state.borrow_mut().native = Some(NativeBindingState {
        generation,
        wake: Arc::clone(&wake),
        timer,
        tray: None,
        tray_intent: TrayIntent::default(),
        tray_fault: false,
        stopping: None,
        notices: StopNotices::default(),
        exit_ready: false,
        stop_restore_failed: false,
    });
    CURRENT.with(|slot| {
        *slot.borrow_mut() = Some(Registration {
            generation,
            window: window.as_weak(),
            state: Rc::downgrade(state),
        });
    });
    let wake: Arc<dyn UiWake> = wake;
    // Explicit maintained sample-mode content; never fallback after failed
    // external catalog preparation. Current package identity is observed only.
    let _notification_install = native_notification_workflow::install(
        window,
        state,
        IdentityMode::CurrentPackage,
        NoticeContent::EngineeringExample,
        wake,
    );
    Ok(())
}

/// Resolve only the exact UI generation; no native callback carries a Root Rc.
fn dispatch(generation: u64) {
    let owner = CURRENT.with(|slot| {
        let slot = slot.borrow();
        let entry = slot
            .as_ref()
            .filter(|entry| entry.generation == generation)?;
        Some((entry.window.upgrade()?, entry.state.upgrade()?))
    });
    if let Some((window, state)) = owner {
        tick(&window, &state);
    }
}

/// Include every current private modal/edit/review owner, never view flags.
pub(super) fn interaction(owner: &BindingState) -> TrayInteraction {
    let shell = owner.shell.interaction();
    if shell != TrayInteraction::Free {
        return shell;
    }
    if owner
        .native
        .as_ref()
        .is_some_and(|native| native.stopping.is_some())
    {
        return TrayInteraction::ExitRequested;
    }
    owner.current_native_interaction()
}

/// Read the current existing typed status; the fixture never fabricates a check adapter.
pub(super) fn current_menu(owner: &BindingState) -> FixtureTrayMenu {
    let status = StatusView::from_observation(owner.status.observe());
    let context = if status.kind() == StatusKind::ServiceStopped {
        TrayCheckContext::ServicesStopped
    } else if status.check_intent() == StatusCheckIntent::SyncNow {
        TrayCheckContext::Synchronization
    } else {
        TrayCheckContext::CheckSourcesOnly
    };
    FixtureTrayMenu::new(context, TrayCheckBlock::NativeAdapterUnavailable)
}

/// Only an actual current private bridge can supply caption-close readiness.
pub(super) fn availability(state: &RefCell<BindingState>) -> AdapterAvailability {
    let owner = state.borrow();
    if owner.copy.failed() || owner.raw_failed() || !owner.preferences.tray_enabled() {
        return AdapterAvailability::Unavailable;
    }
    owner
        .native
        .as_ref()
        .filter(|native| {
            native.stopping.is_none()
                && !native.tray_fault
                && !native.exit_ready
                && !native.tray_intent.disable_pending()
        })
        .and_then(|native| native.tray.as_ref())
        .map_or(AdapterAvailability::Unavailable, |tray| {
            tray.bridge.availability()
        })
}

/// Actual visible window or confirmed tray, separate from saved notification preferences.
pub(super) fn observer_presence(
    window: &AppWindow,
    state: &RefCell<BindingState>,
) -> Observation<ObserverPresence> {
    if window.window().is_visible() || availability(state) == AdapterAvailability::Ready {
        Observation::Current(ObserverPresence::Running)
    } else {
        Observation::Unknown
    }
}

/// Prepare the active catalog before replacing either native or Slint presentation.
///
/// # Errors
/// Missing or invalid native text rejects the complete candidate publication.
pub(super) fn prepare_catalog(
    owner: &BindingState,
    snapshot: &Snapshot<'_>,
) -> Result<Option<PreparedTray>, TrayError> {
    if owner
        .native
        .as_ref()
        .is_none_or(|native| native.tray.is_none())
    {
        return Ok(None);
    }
    let status = StatusView::from_observation(owner.status.observe());
    let tooltip = UiText::key(status.headline().catalog_id()).map_err(|_| TrayError::Catalog)?;
    PreparedTray::catalog(current_menu(owner), interaction(owner), snapshot, tooltip).map(Some)
}

/// Publish one prepared native candidate before a new catalog's visible setter.
///
/// # Errors
/// Failed posting leaves the ordinary observer visible and readiness unavailable.
pub(super) fn publish_catalog(
    window: &AppWindow,
    state: &RefCell<BindingState>,
    projection: Option<PreparedTray>,
) -> Result<(), TrayError> {
    let Some(projection) = projection else {
        return Ok(());
    };
    let Some(mut native) = state.borrow_mut().native.take() else {
        return Err(TrayError::BridgeUnavailable);
    };
    let result = if let Some(tray) = native.tray.as_mut() {
        tray.port
            .update(projection.clone())
            .map(|()| tray.projection = projection)
    } else {
        Err(TrayError::BridgeUnavailable)
    };
    native.tray_fault |= result.is_err();
    state.borrow_mut().native = Some(native);
    if result.is_err() {
        let _restored = restore_window(window);
    }
    result
}

/// A normal UI exit is intercepted until every native owner confirms cleanup.
/// The caller already holds the sole shell projection/consent guard.
pub(super) fn begin_exit(window: &AppWindow, state: &RefCell<BindingState>) -> bool {
    if state.borrow().native.is_none() {
        return false;
    }
    if !begin_stop(window, state, StopPurpose::ExitUi) {
        super::shell_workflow::native_exit_failed(window, state);
        return true;
    }
    match poll_stop(window, state) {
        Some(StopProgress::Complete) => false,
        Some(StopProgress::Failed) => {
            if let Some(native) = state.borrow_mut().native.as_mut() {
                native.notices.failure_reported = true;
            }
            super::shell_workflow::native_exit_failed(window, state);
            true
        }
        _ => true,
    }
}

/// Request only the already-owned local stop; never hide before its acknowledgement.
fn begin_stop(window: &AppWindow, state: &RefCell<BindingState>, purpose: StopPurpose) -> bool {
    if !restore_window(window) {
        if let Some(native) = state.borrow_mut().native.as_mut() {
            native.stop_restore_failed = true;
            native.tray_fault = true;
        }
        return false;
    }
    let Some(mut native) = state.borrow_mut().native.take() else {
        return true;
    };
    native.stop_restore_failed = false;
    if purpose == StopPurpose::ExitUi {
        native.notices.failure_reported = false;
        native.notices.timeout_reported = false;
    }
    let retry_notifications = native.stopping.as_ref().is_some_and(|stop| {
        stop.barrier.purpose() == StopPurpose::ExitUi && stop.notifications_failed
    });
    if let Some(stopping) = &mut native.stopping {
        stopping.barrier.promote(purpose);
        if purpose == StopPurpose::ExitUi {
            stopping.started = Instant::now();
        }
    } else {
        native.stopping = Some(Stopping {
            barrier: StopBarrier::new(purpose),
            started: Instant::now(),
            notifications_failed: false,
            tray_post_failed: false,
        });
        native.notices.timeout_reported = false;
    }
    // One bounded re-post per explicit owner request, including a renewed Quit
    // after known failed posting. The timer only polls and never retries this.
    let post_failed = native
        .tray
        .as_ref()
        .is_some_and(|tray| tray.port.request_stop().is_err());
    if let Some(stop) = native.stopping.as_mut() {
        stop.tray_post_failed = post_failed;
    }
    state.borrow_mut().native = Some(native);
    if purpose == StopPurpose::ExitUi {
        let outcome = if retry_notifications {
            native_notification_workflow::retry_stop(window, state)
        } else {
            native_notification_workflow::begin_stop(window, state)
        };
        if let Some(stop) = state
            .borrow_mut()
            .native
            .as_mut()
            .and_then(|native| native.stopping.as_mut())
        {
            stop.notifications_failed = outcome == CloseOutcome::Partial;
        }
    }
    true
}

/// Poll terminal facts; a timeout never fabricates a worker or COM cleanup ack.
fn poll_stop(window: &AppWindow, state: &RefCell<BindingState>) -> Option<StopProgress> {
    let mut native = state.borrow_mut().native.take()?;
    let Some(stopping) = native.stopping.as_ref() else {
        state.borrow_mut().native = Some(native);
        return None;
    };
    let barrier = stopping.barrier;
    let purpose = barrier.purpose();
    let mut failed = stopping.notifications_failed;
    let tray_cleaned = if let Some(tray) = native.tray.as_mut() {
        match tray.port.try_finish() {
            TrayFinish::Pending => false,
            // This barrier consumes cleanup acknowledgement only; the observation
            // retains the joined operation result independently of its success.
            TrayFinish::CleanupConfirmed {
                operation: _operation,
            } => true,
            TrayFinish::CleanupUnconfirmed {
                operation: _operation,
                cause: _cause,
            } => {
                failed = true;
                false
            }
        }
    } else {
        true
    };
    failed |= stopping.tray_post_failed && !tray_cleaned;
    let notifications_cleaned = native_notification_workflow::stopped(state);
    let progress = stopping.barrier.observe(
        StopFacts {
            tray_cleaned,
            notifications_cleaned,
            cleanup_failed: failed,
        },
        stopping.started.elapsed() >= STOP_DEADLINE,
    );
    if tray_cleaned {
        native.tray = None;
    }
    if progress == StopProgress::Complete {
        native.stopping = None;
        if purpose == StopPurpose::DisableTray {
            native.tray_fault = false;
        } else {
            native.exit_ready = barrier.permits_ui_exit(
                progress,
                state.borrow().shell.interaction() == TrayInteraction::ExitRequested,
            );
            native.tray_fault = !native.exit_ready;
        }
    }
    state.borrow_mut().native = Some(native);
    if matches!(progress, StopProgress::TimedOut | StopProgress::Failed) {
        let _visible = restore_window(window);
    }
    Some(progress)
}

/// Drain/refresh under one receiving guard; route intentions only after it drops.
fn tick(window: &AppWindow, state: &Rc<RefCell<BindingState>>) {
    let Some(guard) = ProjectionGuard::enter_native(state) else {
        return;
    };
    if state
        .borrow()
        .native
        .as_ref()
        .is_none_or(|native| native.exit_ready)
    {
        return;
    }
    let action = pump_tray(window, state);
    native_notification_workflow::refresh(window, state);
    let activation = native_notification_workflow::drain(window, state);
    let purpose = state
        .borrow()
        .native
        .as_ref()
        .and_then(|native| native.stopping.as_ref().map(|stop| stop.barrier.purpose()));
    let progress = if purpose.is_some() {
        poll_stop(window, state)
    } else {
        None
    };
    window.set_native_tray_available(availability(state) == AdapterAvailability::Ready);
    let (report_timeout, report_failure) = {
        let mut owner = state.borrow_mut();
        let Some(native) = owner.native.as_mut() else {
            return;
        };
        if progress == Some(StopProgress::TimedOut) && !native.notices.timeout_reported {
            native.notices.timeout_reported = true;
            (true, false)
        } else if progress == Some(StopProgress::Failed) && !native.notices.failure_reported {
            native.notices.failure_reported = true;
            (false, true)
        } else {
            (false, false)
        }
    };
    if report_timeout && purpose == Some(StopPurpose::DisableTray) {
        super::shell_workflow::native_stop_feedback(window, state, false);
    }
    if report_failure && purpose == Some(StopPurpose::DisableTray) {
        super::shell_workflow::native_stop_feedback(window, state, true);
    }
    drop(guard);
    match (purpose, progress) {
        (Some(StopPurpose::ExitUi), Some(StopProgress::Complete)) => {
            super::shell_workflow::complete_native_exit(window, state);
            return;
        }
        (Some(StopPurpose::ExitUi), Some(StopProgress::TimedOut)) if report_timeout => {
            super::shell_workflow::report_native_exit_pending(window, state);
            return;
        }
        (Some(StopPurpose::ExitUi), Some(StopProgress::Failed)) if report_failure => {
            super::shell_workflow::report_native_exit_failure(window, state);
            return;
        }
        _ => {}
    }
    if let Some(action) = action {
        super::shell_workflow::receive_native_action(window, state, action);
    }
    if let Some(activation) = activation {
        match activation {
            ActivationResolution::Navigate(route) => {
                if restore_window(window) && interaction(&state.borrow()) == TrayInteraction::Free {
                    super::recovery_workflow::request_shell_route(
                        &window.as_weak(),
                        state,
                        route,
                        false,
                    );
                } else {
                    focus_owned(window, state);
                }
            }
            ActivationResolution::FocusOwnedDialog => focus_owned(window, state),
            ActivationResolution::Ignored => {}
        }
    }
}

/// Own the tray temporarily so no Root borrow spans toolkit/native calls.
fn pump_tray(window: &AppWindow, state: &RefCell<BindingState>) -> Option<TrayItemId> {
    let mut native = state.borrow_mut().native.take()?;
    let requested = state.borrow().preferences.tray_enabled();
    if native.tray.is_none() && native.stopping.is_none() {
        native.tray_intent.acknowledge_disable();
        if native.tray_intent.take_retry() {
            native.tray_fault = false;
        }
    }
    let mut action = None;
    if let Some(tray) = native.tray.as_mut() {
        if let Ok(event) = tray.bridge.drain() {
            if (event.restore.is_some() || event.native_failure)
                && restore_window(window)
                && let Some(token) = event.restore
            {
                let _ack = tray.bridge.acknowledge_restored_window(token);
            }
            if event.action.is_some() && restore_window(window) {
                native.stop_restore_failed = false;
            }
            action = event.action;
        } else {
            native.tray_fault = true;
            let _visible = restore_window(window);
        }
    }
    if (!requested || native.tray_intent.disable_pending())
        && native.tray.is_some()
        && native.stopping.is_none()
        && !native.stop_restore_failed
    {
        if restore_window(window) {
            native.stopping = Some(Stopping {
                barrier: StopBarrier::new(StopPurpose::DisableTray),
                started: Instant::now(),
                notifications_failed: false,
                tray_post_failed: false,
            });
            let failed = native
                .tray
                .as_ref()
                .is_some_and(|tray| tray.port.request_stop().is_err());
            if let Some(stop) = native.stopping.as_mut() {
                stop.tray_post_failed = failed;
            }
            native.tray_intent.acknowledge_disable();
        } else {
            native.stop_restore_failed = true;
            native.tray_fault = true;
            super::shell_workflow::native_stop_feedback(window, state, true);
        }
    }
    if native.stopping.is_none() && requested && !native.tray_fault {
        let prepared = {
            let owner = state.borrow();
            let status = StatusView::from_observation(owner.status.observe());
            UiText::key(status.headline().catalog_id())
                .map_err(|_| TrayError::Catalog)
                .and_then(|tooltip| {
                    owner.copy.prepare_native_tray(
                        current_menu(&owner),
                        interaction(&owner),
                        tooltip,
                    )
                })
        };
        if let Ok(prepared) = prepared {
            if let Some(tray) = native.tray.as_mut() {
                if tray.projection != prepared {
                    if tray.port.update(prepared.clone()).is_ok() {
                        tray.projection = prepared;
                    } else {
                        native.tray_fault = true;
                        let _visible = restore_window(window);
                    }
                }
            } else {
                let start = TrayPixels::embedded_brand().and_then(|pixels| {
                    let wake: Arc<dyn UiWake> = native.wake.clone();
                    let bridge = TrayBridge::new(prepared.clone(), wake);
                    NativeTray::start(Arc::clone(&bridge), pixels).map(|port| OwnedTray {
                        port,
                        bridge,
                        projection: prepared,
                    })
                });
                if let Ok(tray) = start {
                    native.tray = Some(tray);
                } else {
                    native.tray_fault = true;
                    let _visible = restore_window(window);
                }
            }
        } else {
            native.tray_fault = true;
            let _visible = restore_window(window);
        }
    }
    state.borrow_mut().native = Some(native);
    action
}

/// Outer platform failure is distinct from a user-confirmed successful UI exit.
/// It requests cleanup and retains the visible observer without claiming its result.
pub(super) fn event_loop_ended(window: &AppWindow, state: &RefCell<BindingState>) -> bool {
    if state
        .borrow()
        .native
        .as_ref()
        .is_none_or(|native| native.exit_ready)
    {
        return true;
    }
    let Some(_guard) = ProjectionGuard::enter_native(state) else {
        return false;
    };
    if !begin_stop(window, state, StopPurpose::ExitUi) {
        return false;
    }
    matches!(poll_stop(window, state), Some(StopProgress::Complete))
}

/// A failed final hide/loop request cannot reuse an earlier exit-ready decision.
pub(super) fn exit_not_completed(state: &RefCell<BindingState>) {
    if let Some(native) = state.borrow_mut().native.as_mut() {
        native.exit_ready = false;
        native.tray_fault = true;
    }
}

/// Request public restore/show without retaining a Root borrow across toolkit calls.
/// A successful show is the narrow acknowledgement; native focus/DPI remains qualified separately.
pub(super) fn restore_window(window: &AppWindow) -> bool {
    window.window().set_minimized(false);
    window.show().is_ok()
}

/// Re-focus the existing private owner without a new modal or navigation request.
pub(super) fn focus_owned(window: &AppWindow, state: &RefCell<BindingState>) {
    if !restore_window(window) {
        return;
    }
    let Some(_guard) = ProjectionGuard::enter_native(state) else {
        return;
    };
    let (shell, recovery_leave, inline) = {
        let owner = state.borrow();
        (
            owner.shell.interaction(),
            owner.recovery_leave.is_some(),
            owner.current_inline_owner(),
        )
    };
    if shell == TrayInteraction::DialogOwned {
        window.invoke_focus_quit_warning();
    } else if recovery_leave {
        window.invoke_focus_recovery_leave_safe();
    } else {
        match inline {
            Some(InlineOwner::Recovery) => window.invoke_focus_recovery_content(),
            Some(InlineOwner::Sources) => window.invoke_focus_source_details(),
            None => {}
        }
    }
}

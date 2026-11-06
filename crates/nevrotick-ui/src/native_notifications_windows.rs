// SPDX-License-Identifier: GPL-3.0-only
//! Source-only Windows local-toast transport; no registration or WNS/push client.
//!
//! The UI event-loop thread owns this port and must serialize request/activation
//! handling under its existing private shell projection guard. The native event
//! handlers contain only a one-entry channel, fixed flags and a post-only wake;
//! they never borrow UI state or execute a navigation/service command. All native
//! calls and cleanup below still need the selected Windows qualification lane.

use std::{
    marker::PhantomData,
    panic::{AssertUnwindSafe, catch_unwind},
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::{Receiver, SyncSender, sync_channel},
    },
};

use windows::{
    ApplicationModel::{AppInfo, Package},
    Data::Xml::Dom::XmlDocument,
    Foundation::TypedEventHandler,
    UI::Notifications::{
        NotificationSetting, ToastActivatedEventArgs, ToastFailedEventArgs, ToastNotification,
        ToastNotificationManager, ToastNotifier,
    },
    Win32::System::{
        Com::CoCreateGuid,
        WinRT::{RO_INIT_SINGLETHREADED, RoInitialize, RoUninitialize},
    },
    core::{HSTRING, IInspectable, Interface},
};

use crate::native_tray::UiWake;
use crate::notifications_ui::{
    FixtureNotice, NotificationBlock, ObserverPresence, WindowsNotificationSetting,
};
use crate::preferences::UiPreferences;
use crate::shell::{AdapterAvailability, Observation};
use crate::tray_ui::TrayInteraction;

use crate::notifications_ui::native_delivery::{
    ActivationError, ActivationResolution, ActivationToken, DeliverySession, PreparedTestNotice,
    SessionNonce, Submission,
};

/// Installed identity mode selected by maintained delivery code, never locale/UI data.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdentityMode {
    /// Observe this process's existing package identity; make no registration.
    CurrentPackage,
    /// Requires the future qualified installer-owned desktop identity observer.
    DesktopInstallation,
}

/// Fixed source-bound failure category, with no formatted OS/path/contributor data.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeFailure {
    /// The current UI thread cannot acquire a compatible balanced `WinRT` apartment.
    Apartment,
    /// Existing package identity could not be observed for the selected mode.
    IdentityUnavailable,
    /// Desktop identity/activator/shortcut ownership has not been qualified.
    DesktopIdentityUnqualified,
    /// The current native factory or setting could not be observed.
    FacilityUnavailable,
    /// A document, object or event handler could not be fully prepared before Show.
    Preparation,
    /// Earlier native event/visibility cleanup remains uncertain.
    CleanupPending,
    /// A fresh native correlation could not be established.
    CorrelationUnavailable,
}

/// Submission result, distinct from OS banner display, reading and service health.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestOutcome {
    /// An unchanged or replayed ID has already consumed its one attempt.
    Coalesced,
    /// Accepted preferences/current observer/Windows settings forbid this request.
    Blocked(NotificationBlock),
    /// No Show call was made for this request; native setup/cleanup failed.
    NotSubmitted(NativeFailure),
    /// A Show call consumed this ID; uncertainty cannot authorize automatic retry.
    Attempted(Submission),
}

/// Explicit shutdown result; Drop alone is not native lifecycle evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CloseOutcome {
    /// Matching hide/event-removal calls succeeded for every retained object.
    Complete,
    /// At least one owned native effect or event removal remains unconfirmed.
    Partial,
}

/// Bounded current callback observations; original native strings are never retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EventSnapshot {
    /// At most one typed activation; None includes an empty/disconnected channel.
    pub activation: Option<Result<ActivationToken, ActivationError>>,
    /// A current native Failed event occurred after or during the Show attempt.
    pub failed_after_attempt: bool,
    /// The bounded UI wake failed/panicked; new attempts fail closed.
    pub observer_fault: bool,
}

/// A successful `WinRT` initialization belongs to this exact thread until last drop.
struct Apartment {
    /// Rc affinity forbids Send/Sync without unsafe marker implementations.
    _affinity: PhantomData<Rc<()>>,
}

impl Apartment {
    /// Initialize once without switching a previously incompatible apartment.
    ///
    /// # Errors
    /// Native failure, including a changed-apartment mode, leaves no owned init.
    #[expect(
        unsafe_code,
        reason = "balanced role-local WinRT apartment initialization"
    )]
    fn open() -> Result<Self, NativeFailure> {
        // SAFETY: called on the owning UI thread before creating this port's
        // WinRT objects. Success, including S_FALSE, owns exactly one matching
        // RoUninitialize. An error constructs no guard and performs no uninit.
        unsafe { RoInitialize(RO_INIT_SINGLETHREADED) }.map_err(|_| NativeFailure::Apartment)?;
        Ok(Self {
            _affinity: PhantomData,
        })
    }
}

impl Drop for Apartment {
    /// Balance only our successful call, on the same non-Send owner thread.
    #[expect(
        unsafe_code,
        reason = "matching WinRT apartment release after its owned COM objects"
    )]
    fn drop(&mut self) {
        // SAFETY: only successful open constructs this private guard. Its Rc
        // affinity prevents moving it, and it is the port's last declared field
        // so all owned COM interfaces release before this call.
        unsafe { RoUninitialize() };
    }
}

/// One retained toast and its exact registered-handler removal obligations.
struct ActiveToast {
    /// Owned interface; no foreign-pointer downcast or borrowed COM storage.
    toast: ToastNotification,
    /// Exact activated registration token, retained until confirmed removal.
    activated: Option<i64>,
    /// Exact failed registration token, retained until confirmed removal.
    failed: Option<i64>,
    /// Native failure is correlated to this object, not an old replacement.
    observed_failure: Arc<AtomicBool>,
    /// A successful or uncertain Show still needs matching Hide on replacement.
    shown_or_uncertain: bool,
}

impl ActiveToast {
    /// Try every owned cleanup once; retain unacknowledged obligations for retry.
    fn close(&mut self, notifier: &ToastNotifier) -> bool {
        let mut complete = true;
        if self.shown_or_uncertain {
            if notifier.Hide(&self.toast).is_ok() {
                self.shown_or_uncertain = false;
            } else {
                complete = false;
            }
        }
        if let Some(token) = self.activated {
            if self.toast.RemoveActivated(token).is_ok() {
                self.activated = None;
            } else {
                complete = false;
            }
        }
        if let Some(token) = self.failed {
            if self.toast.RemoveFailed(token).is_ok() {
                self.failed = None;
            } else {
                complete = false;
            }
        }
        complete
    }
}

/// UI-thread-owned local notification adapter, with one retained toast/queue slot.
///
/// This factory never sets an AUMID or registers an activator, shortcut, startup
/// task, service or package. The selected identity must already exist. Caller
/// initialization failure must leave the ordinary GUI usable and unavailable.
pub struct WindowsToastPort {
    /// The sole retained object; cleanup failure blocks replacement allocation.
    active: Option<ActiveToast>,
    /// No callbacks own the receiver or any UI state.
    receiver: Receiver<Result<ActivationToken, ActivationError>>,
    /// One pending activation can be queued; overflow is coalesced by `try_send`.
    sender: SyncSender<Result<ActivationToken, ActivationError>>,
    /// Code-owned post-only wake shared with the native tray bridge.
    wake: Arc<dyn UiWake>,
    /// A callback wake failure denies future native attempts.
    callback_fault: Arc<AtomicBool>,
    /// A failed cleanup denies replacement until explicit close confirms it.
    cleanup_uncertain: bool,
    /// One outstanding UI wake across all current/late callback generations.
    wake_pending: Arc<AtomicBool>,
    /// Close is observed by callbacks before handler detachment attempts.
    stopping: Arc<AtomicBool>,
    /// Current coalescing/correlation; never the shell or incident authority.
    session: DeliverySession,
    /// Current process-local notifier, obtained from existing Windows identity.
    notifier: ToastNotifier,
    /// Must drop last, after all fields owning COM interfaces.
    _apartment: Apartment,
}

impl WindowsToastPort {
    /// Observe one explicitly selected installed identity and prepare native state.
    ///
    /// # Errors
    /// Missing identity, incompatible apartment or native factory/nonce failure
    /// returns fixed unavailable state. Desktop registration is not synthesized.
    pub fn open(mode: IdentityMode, wake: Arc<dyn UiWake>) -> Result<Self, NativeFailure> {
        if mode == IdentityMode::DesktopInstallation {
            return Err(NativeFailure::DesktopIdentityUnqualified);
        }
        let apartment = Apartment::open()?;
        // Current observes OS identity; no package/registry/shortcut is created.
        let _identity = Package::Current().map_err(|_| NativeFailure::IdentityUnavailable)?;
        let application_id = AppInfo::Current()
            .and_then(|info| info.AppUserModelId())
            .map_err(|_| NativeFailure::IdentityUnavailable)?;
        if application_id.is_empty() || application_id.len() > 128 || application_id.contains(&0) {
            return Err(NativeFailure::IdentityUnavailable);
        }
        let notifier = ToastNotificationManager::CreateToastNotifierWithId(&application_id)
            .map_err(|_| NativeFailure::FacilityUnavailable)?;
        // Successful factory creation alone is not a setting or delivery claim.
        notifier
            .Setting()
            .map_err(|_| NativeFailure::FacilityUnavailable)?;
        let nonce = native_nonce()?;
        let (sender, receiver) = sync_channel(1);
        Ok(Self {
            active: None,
            receiver,
            sender,
            wake,
            callback_fault: Arc::new(AtomicBool::new(false)),
            cleanup_uncertain: false,
            wake_pending: Arc::new(AtomicBool::new(false)),
            stopping: Arc::new(AtomicBool::new(false)),
            session: DeliverySession::new(nonce),
            notifier,
            _apartment: apartment,
        })
    }

    /// Report only this adapter's current narrow operation availability.
    #[must_use]
    pub fn availability(&self) -> AdapterAvailability {
        if self.session.stopped || self.callback_fault.load(Ordering::Acquire) {
            AdapterAvailability::Unavailable
        } else if self.cleanup_uncertain {
            AdapterAvailability::Unknown
        } else {
            AdapterAvailability::Ready
        }
    }

    /// Re-observe the native setting; unknown values/errors cannot become Allowed.
    #[must_use]
    pub fn windows_setting(&self) -> Observation<WindowsNotificationSetting> {
        match self.notifier.Setting() {
            Ok(NotificationSetting::Enabled) => {
                Observation::Current(WindowsNotificationSetting::Allowed)
            }
            Ok(
                NotificationSetting::DisabledForApplication | NotificationSetting::DisabledForUser,
            ) => Observation::Current(WindowsNotificationSetting::BlockedForUser),
            Ok(NotificationSetting::DisabledByGroupPolicy) => {
                Observation::Current(WindowsNotificationSetting::BlockedByPolicy)
            }
            Ok(NotificationSetting::DisabledByManifest) => {
                Observation::Current(WindowsNotificationSetting::Unsupported)
            }
            Ok(_) | Err(_) => Observation::Unknown,
        }
    }

    /// Submit one explicit test under the caller's live UI transition guard.
    ///
    /// No production problem can be fabricated from a `FixtureNotice`. Callers
    /// must check actual Settings/Notifications, terminal/modal/exit state before
    /// calling; this method independently rechecks accepted preferences/current
    /// observer and a fresh Windows setting. Hold no `RefCell`/state lock across it.
    #[must_use]
    pub fn request_test(
        &mut self,
        prepared: &PreparedTestNotice,
        preferences: &UiPreferences,
        observer: Observation<ObserverPresence>,
    ) -> RequestOutcome {
        if self.availability() != AdapterAvailability::Ready {
            return RequestOutcome::Blocked(NotificationBlock::NativeAdapterUnavailable);
        }
        match self.session.admit(prepared, preferences, observer) {
            Ok(false) => return RequestOutcome::Coalesced,
            Err(block) => return RequestOutcome::Blocked(block),
            Ok(true) => {}
        }
        match self.windows_setting() {
            Observation::Current(WindowsNotificationSetting::Allowed) => {}
            Observation::Current(
                WindowsNotificationSetting::BlockedForUser
                | WindowsNotificationSetting::BlockedByPolicy,
            ) => {
                return RequestOutcome::Blocked(NotificationBlock::WindowsBlocked);
            }
            Observation::Unknown
            | Observation::Stale(_)
            | Observation::Current(WindowsNotificationSetting::Unsupported) => {
                return RequestOutcome::Blocked(NotificationBlock::WindowsUnavailable);
            }
        }
        if !self.close_active() {
            return RequestOutcome::NotSubmitted(NativeFailure::CleanupPending);
        }
        let token = ActivationToken::new(self.session.nonce, prepared.notice());
        let Ok(document) = document(prepared, token) else {
            return RequestOutcome::NotSubmitted(NativeFailure::Preparation);
        };
        let Ok(toast) = ToastNotification::CreateToastNotification(&document) else {
            return RequestOutcome::NotSubmitted(NativeFailure::Preparation);
        };
        let failure = Arc::new(AtomicBool::new(false));
        let mut active = ActiveToast {
            toast,
            activated: None,
            failed: None,
            observed_failure: Arc::clone(&failure),
            shown_or_uncertain: false,
        };
        if install_handlers(
            &mut active,
            &self.sender,
            &self.wake,
            &self.callback_fault,
            &self.wake_pending,
            &self.stopping,
        )
        .is_err()
        {
            self.active = Some(active);
            self.close_active();
            return RequestOutcome::NotSubmitted(NativeFailure::Preparation);
        }
        // The ID is consumed for every actual Show call, even an uncertain Err.
        active.shown_or_uncertain = true;
        let submission = if self.notifier.Show(&active.toast).is_ok() {
            Submission::AcceptedByOs
        } else {
            Submission::Indeterminate
        };
        self.session.attempted(prepared.notice(), submission);
        self.active = Some(active);
        RequestOutcome::Attempted(submission)
    }

    /// Drain one typed activation and read only the current retained object's flags.
    #[must_use]
    pub fn poll(&self) -> EventSnapshot {
        // Every producer uses an AcqRel RMW, even when already queued. Clearing
        // before reading either observes that publication or lets it queue the
        // one next wake; no callback data is lost behind a stale queued bit.
        let _was_queued = self.wake_pending.swap(false, Ordering::AcqRel);
        EventSnapshot {
            activation: self.receiver.try_recv().ok(),
            failed_after_attempt: self
                .active
                .as_ref()
                .is_some_and(|active| active.observed_failure.load(Ordering::Acquire)),
            observer_fault: self.callback_fault.load(Ordering::Acquire),
        }
    }

    /// Resolve only a typed navigation intention against fresh UI-owned facts.
    #[must_use]
    pub fn resolve_activation(
        &mut self,
        token: Result<ActivationToken, ActivationError>,
        current: Observation<FixtureNotice>,
        interaction: TrayInteraction,
    ) -> ActivationResolution {
        let usable = !self.callback_fault.load(Ordering::Acquire)
            && !self
                .active
                .as_ref()
                .is_some_and(|active| active.observed_failure.load(Ordering::Acquire));
        // The shared pure owner checks stopped/exit and dialog ownership before
        // this availability fallback, so no fault can resurrect a closed UI.
        self.session
            .resolve_activation(token, current, interaction, usable)
    }

    /// Stop new requests first, then attempt matching cleanup once without loops.
    #[must_use]
    pub fn close(&mut self) -> CloseOutcome {
        self.session.stopped = true;
        self.stopping.store(true, Ordering::Release);
        if self.close_active() {
            CloseOutcome::Complete
        } else {
            CloseOutcome::Partial
        }
    }

    /// Keep uncertain obligations; replacement never grows a list of old objects.
    fn close_active(&mut self) -> bool {
        if let Some(active) = &mut self.active
            && !active.close(&self.notifier)
        {
            self.cleanup_uncertain = true;
            return false;
        }
        self.active = None;
        self.cleanup_uncertain = false;
        true
    }
}

impl Drop for WindowsToastPort {
    /// Best effort cannot replace the caller's explicit observed close result.
    fn drop(&mut self) {
        if self.close() == CloseOutcome::Partial {
            self.callback_fault.store(true, Ordering::Release);
        }
    }
}

/// Obtain only per-observer correlation; this is not a cryptographic capability.
///
/// # Errors
/// Failed native generation or an impossible zero result rejects before use.
#[expect(
    unsafe_code,
    reason = "native GUID generation for process-local notification correlation"
)]
fn native_nonce() -> Result<SessionNonce, NativeFailure> {
    // SAFETY: the generated binding owns its fixed-size GUID output buffer;
    // this call runs on the initialized owner thread and returns an owned value.
    let value = unsafe { CoCreateGuid() }.map_err(|_| NativeFailure::CorrelationUnavailable)?;
    SessionNonce::new(value.to_u128()).ok_or(NativeFailure::CorrelationUnavailable)
}

/// Build only a fixed DOM shape. Contributor text enters text/attribute APIs only.
///
/// # Errors
/// Any native construction failure aborts before Show; no XML parser is invoked.
fn document(
    prepared: &PreparedTestNotice,
    token: ActivationToken,
) -> windows::core::Result<XmlDocument> {
    let doc = XmlDocument::new()?;
    let toast = doc.CreateElement(&HSTRING::from("toast"))?;
    toast.SetAttribute(&HSTRING::from("launch"), &HSTRING::from(token.encode()))?;
    toast.SetAttribute(
        &HSTRING::from("activationType"),
        &HSTRING::from("foreground"),
    )?;
    let visual = doc.CreateElement(&HSTRING::from("visual"))?;
    let binding = doc.CreateElement(&HSTRING::from("binding"))?;
    binding.SetAttribute(&HSTRING::from("template"), &HSTRING::from("ToastGeneric"))?;
    for content in [prepared.title(), prepared.body()] {
        let element = doc.CreateElement(&HSTRING::from("text"))?;
        let text = doc.CreateTextNode(&HSTRING::from(content))?;
        element.AppendChild(&text)?;
        binding.AppendChild(&element)?;
    }
    visual.AppendChild(&binding)?;
    toast.AppendChild(&visual)?;
    let actions = doc.CreateElement(&HSTRING::from("actions"))?;
    let action = doc.CreateElement(&HSTRING::from("action"))?;
    action.SetAttribute(
        &HSTRING::from("content"),
        &HSTRING::from(prepared.action_label()),
    )?;
    action.SetAttribute(&HSTRING::from("arguments"), &HSTRING::from(token.encode()))?;
    action.SetAttribute(
        &HSTRING::from("activationType"),
        &HSTRING::from("foreground"),
    )?;
    actions.AppendChild(&action)?;
    toast.AppendChild(&actions)?;
    doc.AppendChild(&toast)?;
    Ok(doc)
}

/// Register only bounded, non-UI, non-command callbacks for this one owned object.
///
/// # Errors
/// Returns setup failure with exact successful tokens retained in `active`.
fn install_handlers(
    active: &mut ActiveToast,
    sender: &SyncSender<Result<ActivationToken, ActivationError>>,
    wake: &Arc<dyn UiWake>,
    fault: &Arc<AtomicBool>,
    pending: &Arc<AtomicBool>,
    stopping: &Arc<AtomicBool>,
) -> windows::core::Result<()> {
    let activation_sender = sender.clone();
    let activation_wake = Arc::clone(wake);
    let activation_fault = Arc::clone(fault);
    let activation_pending = Arc::clone(pending);
    let activation_stopping = Arc::clone(stopping);
    let handler = TypedEventHandler::<ToastNotification, IInspectable>::new(move |_, arguments| {
        if activation_stopping.load(Ordering::Acquire) {
            return Ok(());
        }
        let parsed = arguments
            .as_ref()
            .and_then(|arguments| arguments.cast::<ToastActivatedEventArgs>().ok())
            .and_then(|arguments| arguments.Arguments().ok())
            .ok_or(ActivationError)
            .and_then(|arguments| ActivationToken::from_utf16(&arguments));
        if activation_sender.try_send(parsed).is_ok() {
            post_wake(
                &activation_wake,
                &activation_fault,
                &activation_pending,
                &activation_stopping,
            );
        }
        Ok(())
    });
    active.activated = Some(active.toast.Activated(&handler)?);
    let failed_wake = Arc::clone(wake);
    let failed_fault = Arc::clone(fault);
    let observed_failure = Arc::clone(&active.observed_failure);
    let failed_pending = Arc::clone(pending);
    let failed_stopping = Arc::clone(stopping);
    let failed = TypedEventHandler::<ToastNotification, ToastFailedEventArgs>::new(move |_, _| {
        if !failed_stopping.load(Ordering::Acquire)
            && !observed_failure.swap(true, Ordering::AcqRel)
        {
            post_wake(
                &failed_wake,
                &failed_fault,
                &failed_pending,
                &failed_stopping,
            );
        }
        Ok(())
    });
    active.failed = Some(active.toast.Failed(&failed)?);
    Ok(())
}

/// No callback unwinds through `WinRT`, even if a code-owned wake implementation errs.
fn post_wake(
    wake: &Arc<dyn UiWake>,
    fault: &AtomicBool,
    pending: &AtomicBool,
    stopping: &AtomicBool,
) {
    if stopping.load(Ordering::Acquire) || fault.load(Ordering::Acquire) {
        return;
    }
    if !pending.swap(true, Ordering::AcqRel)
        && !matches!(catch_unwind(AssertUnwindSafe(|| wake.enqueue())), Ok(true))
    {
        fault.store(true, Ordering::Release);
    }
}

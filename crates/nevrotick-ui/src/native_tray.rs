// SPDX-License-Identifier: GPL-3.0-only
//! Bounded native-tray preparation and event custody, never service authority.
//!
//! The Windows worker owns only its hidden top-level window, icon and popup.
//! The actual Slint owner drains one coalesced event, restores a lost observer,
//! then passes each action through the existing `FixtureTrayMenu` and sole
//! `ShellController`. A callback ID, icon-registration acknowledgement or menu
//! label grants no measurement, service, clock or exit-confirmation authority.
#![deny(unsafe_code)]

use std::fmt;
#[cfg(windows)]
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use crate::localization::{Snapshot, UiText, check_fixture_literal};
use crate::shell::AdapterAvailability;
use crate::tray_ui::{FixtureTrayMenu, TrayEntry, TrayInteraction, TrayItemId};

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::{NativeTray, StartError, TrayPixels};

/// Fixed UTF-16 capacity, including the required terminating NUL.
pub const WIDE_CAPACITY: usize = 128;
/// Exactly one fixed native icon identity; it is not a user-selected identifier.
pub const ICON_ID: u16 = 1;

/// Redacted preparation or lifecycle failure without raw text or native pointers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrayError {
    /// The catalog could not provide a required closed text meaning.
    Catalog,
    /// Input exceeded the original UTF-8 or post-escaping native UTF-16 budget.
    TextBounds,
    /// Empty/control/unsupported repertoire was supplied to a native text sink.
    TextCharacters,
    /// A native command did not identify one of the six maintained actions.
    InvalidCommand,
    /// A poisoned bridge cannot provide a trusted current snapshot.
    BridgeUnavailable,
    /// An old/different observer supplied a restoration token.
    StaleObserver,
    /// The native observer has not confirmed cleanup/completion.
    CleanupUnconfirmed,
}

impl fmt::Display for TrayError {
    /// Never format contributor text, native payloads or addresses.
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        output.write_str("The tray observer is unavailable. Keep the application window open.")
    }
}
impl std::error::Error for TrayError {}

/// A fully bounded NUL-terminated literal native string, with redacted Debug.
#[derive(Clone, Eq, PartialEq)]
pub struct NativeText([u16; WIDE_CAPACITY]);
impl fmt::Debug for NativeText {
    /// Never echo catalog content through bridge diagnostics.
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        output.write_str("NativeText(<redacted>)")
    }
}
impl NativeText {
    /// Preserve original text, doubling Win32 menu accelerator markers only.
    ///
    /// # Errors
    /// Reject unsupported characters, blank text or output longer than127 units.
    fn prepare(value: &str, menu: bool) -> Result<Self, TrayError> {
        if value.len() > crate::localization::STRING_BYTES {
            return Err(TrayError::TextBounds);
        }
        if value.is_empty()
            || !value.bytes().any(|byte| byte != b' ')
            || check_fixture_literal(value).is_err()
        {
            return Err(TrayError::TextCharacters);
        }
        let mut result = [0; WIDE_CAPACITY];
        let mut used = 0;
        for unit in value.encode_utf16() {
            if used >= WIDE_CAPACITY - 1 {
                return Err(TrayError::TextBounds);
            }
            result[used] = unit;
            used += 1;
            if menu && unit == u16::from(b'&') {
                if used >= WIDE_CAPACITY - 1 {
                    return Err(TrayError::TextBounds);
                }
                result[used] = unit;
                used += 1;
            }
        }
        Ok(Self(result))
    }
    /// Borrow all128 initialized units, including the guaranteed NUL terminator.
    #[must_use]
    pub const fn units(&self) -> &[u16; WIDE_CAPACITY] {
        &self.0
    }
}

/// One closed, literal popup row; enabled is presentation, not receiving authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NativeItem {
    /// Stable checked semantic identity.
    id: TrayItemId,
    /// Bounded escaped text; never a command identifier.
    text: NativeText,
    /// The current private producer's presentation availability.
    enabled: bool,
}
impl NativeItem {
    /// Return stable action identity, never a translated row position.
    #[must_use]
    pub const fn id(&self) -> TrayItemId {
        self.id
    }
    /// Borrow bounded native menu text.
    #[must_use]
    pub const fn text(&self) -> &NativeText {
        &self.text
    }
    /// Return current presentation availability, not service permission.
    #[must_use]
    pub const fn enabled(&self) -> bool {
        self.enabled
    }
}

/// One immutable, fully prepared six-row native projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedTray {
    /// Fixed native row order and identity; no user-owned collection exists.
    items: [NativeItem; 6],
    /// One bounded literal tooltip, independent of menu accelerator escaping.
    tooltip: NativeText,
}
impl PreparedTray {
    /// Prepare explicit engineering preview text without external catalog admission.
    ///
    /// # Errors
    /// Reject an unexpectedly invalid maintained literal without partial output.
    pub fn fixture(menu: FixtureTrayMenu, interaction: TrayInteraction) -> Result<Self, TrayError> {
        let entries = menu.entries(interaction);
        let labels = entries.map(|entry| entry.label().english());
        Self::prepare(
            entries,
            labels,
            "NevroTick - sample preview - no live connection",
        )
    }
    /// Render all labels and tooltip through one already admitted generation.
    ///
    /// Reserved tray IDs must be activated by the code-owned contract when the
    /// actual native renderer is adopted. Missing text rejects the complete new
    /// projection; this function never changes an existing icon/menu or falls
    /// back from a failed external catalog to engineering strings.
    ///
    /// # Errors
    /// Reject a missing/invalid catalog meaning or native sink budget mismatch.
    pub fn catalog(
        menu: FixtureTrayMenu,
        interaction: TrayInteraction,
        snapshot: &Snapshot<'_>,
        tooltip: UiText,
    ) -> Result<Self, TrayError> {
        let entries = menu.entries(interaction);
        let labels = entries.map(|entry| {
            UiText::key(entry.label().catalog_id())
                .map_err(|_| TrayError::Catalog)
                .and_then(|text| text.render(snapshot).map_err(|_| TrayError::Catalog))
        });
        let [open, check, diagnostics, settings, about, quit] = labels;
        let labels = [open?, check?, diagnostics?, settings?, about?, quit?];
        let tip = tooltip.render(snapshot).map_err(|_| TrayError::Catalog)?;
        Self::prepare(
            entries,
            labels
                .each_ref()
                .map(crate::localization::PlainText::as_str),
            tip.as_str(),
        )
    }
    /// Admit every sink before constructing a publishable native projection.
    ///
    /// # Errors
    /// Any invalid sink rejects the whole candidate, preserving the caller's old one.
    fn prepare(
        entries: [TrayEntry; 6],
        labels: [&str; 6],
        tooltip: &str,
    ) -> Result<Self, TrayError> {
        let next = entries
            .into_iter()
            .zip(labels)
            .map(|(entry, text)| {
                NativeText::prepare(text, true).map(|text| NativeItem {
                    id: entry.id(),
                    text,
                    enabled: entry.enabled(),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let items = next.try_into().map_err(|_| TrayError::InvalidCommand)?;
        Ok(Self {
            items,
            tooltip: NativeText::prepare(tooltip, false)?,
        })
    }
    /// Borrow exactly six admitted rows.
    #[must_use]
    pub const fn items(&self) -> &[NativeItem; 6] {
        &self.items
    }
    /// Borrow the completely initialized native tooltip buffer.
    #[must_use]
    pub const fn tooltip(&self) -> &NativeText {
        &self.tooltip
    }
}

/// Decode the menu return value;0 is cancellation and1..6 are code-owned IDs.
///
/// # Errors
/// Reject every other value without selecting or allocating an action.
pub fn popup_selection(raw: i32) -> Result<Option<TrayItemId>, TrayError> {
    if raw == 0 {
        return Ok(None);
    }
    let code = raw.checked_sub(1).ok_or(TrayError::InvalidCommand)?;
    TrayItemId::parse(code)
        .map(Some)
        .map_err(|_| TrayError::InvalidCommand)
}

/// A private loss generation, returned only by the actual bridge drain.
#[derive(Clone, Debug)]
pub struct RestoreToken {
    /// Loss ordinal belongs only to this instance and never wraps.
    generation: u64,
    /// Holding a unique live allocation prevents pointer-reuse/old-instance ambiguity.
    origin: Arc<()>,
}
impl PartialEq for RestoreToken {
    /// Both current instance and generation are required; empty Arc values are not identities.
    fn eq(&self, other: &Self) -> bool {
        self.generation == other.generation && Arc::ptr_eq(&self.origin, &other.origin)
    }
}
impl Eq for RestoreToken {}

/// One current UI-thread wake result; availability must be queried again on use.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TrayEvent {
    /// Sticky observed icon loss; main window restoration needs actual acknowledgement.
    pub restore: Option<RestoreToken>,
    /// Latest single checked presentation intention, rechecked by the Root receiver.
    pub action: Option<TrayItemId>,
    /// A native operation failed; no successful acknowledgement is inferred.
    pub native_failure: bool,
}

/// A fixed single-slot mailbox; every drain schedules at most one subsequent wake.
struct Mailbox {
    /// The only retained native action; repeated input never grows a queue.
    pending: TrayEvent,
    /// Latest complete native projection, replaced only by prepared values.
    projection: PreparedTray,
}

/// Event-loop scheduling boundary, called without any bridge/state lock held.
///
/// Implementations enqueue one bounded wake onto the actual UI event loop. They
/// must not execute receiving actions on the worker or claim window visibility.
pub trait UiWake: Send + Sync + 'static {
    /// Return true only when that UI wake was successfully enqueued.
    fn enqueue(&self) -> bool;
}

/// Shared bounded custody; no Slint component or native handle is owned here.
pub struct TrayBridge {
    /// Fresh process-local identity retained by every restoration token.
    origin: Arc<()>,
    /// One retained projection and one event only.
    mailbox: Mutex<Mailbox>,
    /// A single outstanding UI wake, reset atomically with mailbox draining.
    queued: AtomicBool,
    /// Native registration/version acknowledgements have completed.
    registered: AtomicBool,
    /// Observed icon loss remains sticky until actual main-window restoration.
    loss_generation: AtomicU64,
    /// Last generation acknowledged after an actual UI-owner show succeeded.
    restored_generation: AtomicU64,
    /// This bridge belongs to exactly one native worker lifetime.
    #[cfg(windows)]
    worker_started: AtomicBool,
    /// Native publication is in flight; old registration is not a current text ack.
    publishing: AtomicBool,
    /// An actual host projection update is pending; a raw wake cannot create one.
    update_pending: AtomicBool,
    /// Private host request, never manufactured by a raw Windows message.
    stopping: AtomicBool,
    /// At most one posted control wake; repeated updates only replace one slot.
    #[cfg(windows)]
    control_queued: AtomicBool,
    /// Owned `HWND` identity expressed as an opaque Windows handle word.
    #[cfg(windows)]
    endpoint: Mutex<usize>,
    /// An Add attempt may have affected the shell and needs explicit deletion.
    #[cfg(windows)]
    icon_attempted: AtomicBool,
    /// Borrowed live icon word, used only on the owning native thread.
    #[cfg(windows)]
    icon: AtomicUsize,
    /// Registered broadcast message identity, not a user action.
    #[cfg(windows)]
    taskbar_created: AtomicUsize,
    /// Native popup nesting; prevents repeated menu creation and orders teardown.
    #[cfg(windows)]
    menu_open: AtomicBool,
    /// Explicit native cleanup failure, latched for truthful final reporting.
    cleanup_failed: AtomicBool,
    /// Only a bounded UI-event scheduling port, never the Root's controller.
    #[cfg(any(windows, test))]
    wake: Arc<dyn UiWake>,
    /// Preserve the scheduling port's lifetime on hosts without a native producer.
    #[cfg(not(any(windows, test)))]
    _wake: Arc<dyn UiWake>,
}
impl TrayBridge {
    /// Retain a prepared immutable initial projection and a bounded wake port.
    #[must_use]
    pub fn new(projection: PreparedTray, wake: Arc<dyn UiWake>) -> Arc<Self> {
        Arc::new(Self {
            origin: Arc::new(()),
            mailbox: Mutex::new(Mailbox {
                pending: TrayEvent::default(),
                projection,
            }),
            queued: AtomicBool::new(false),
            registered: AtomicBool::new(false),
            loss_generation: AtomicU64::new(0),
            restored_generation: AtomicU64::new(0),
            publishing: AtomicBool::new(false),
            update_pending: AtomicBool::new(false),
            #[cfg(windows)]
            worker_started: AtomicBool::new(false),
            stopping: AtomicBool::new(false),
            #[cfg(windows)]
            endpoint: Mutex::new(0),
            #[cfg(windows)]
            control_queued: AtomicBool::new(false),
            #[cfg(windows)]
            icon: AtomicUsize::new(0),
            #[cfg(windows)]
            icon_attempted: AtomicBool::new(false),
            #[cfg(windows)]
            taskbar_created: AtomicUsize::new(0),
            #[cfg(windows)]
            menu_open: AtomicBool::new(false),
            cleanup_failed: AtomicBool::new(false),
            #[cfg(any(windows, test))]
            wake,
            #[cfg(not(any(windows, test)))]
            _wake: wake,
        })
    }
    /// Return current readiness. A saved preference or previous ack is insufficient.
    #[must_use]
    pub fn availability(&self) -> AdapterAvailability {
        // Order is part of the one UI-owner/one native-worker contract. If a
        // worker consumes pending=false, acquiring that value observes the
        // earlier publishing=true barrier. The next load sees that barrier or
        // its acknowledged terminal false; terminal registration/loss reads
        // follow it so old success cannot combine with a later failed ack.
        if !self.update_pending.load(Ordering::Acquire)
            && !self.publishing.load(Ordering::Acquire)
            && self.registered.load(Ordering::Acquire)
            && self.loss_generation.load(Ordering::Acquire)
                == self.restored_generation.load(Ordering::Acquire)
            && !self.stopping.load(Ordering::Acquire)
            && !self.cleanup_failed.load(Ordering::Acquire)
        {
            AdapterAvailability::Ready
        } else {
            AdapterAvailability::Unavailable
        }
    }
    /// Replace only a complete checked projection; Windows update remains unacknowledged.
    ///
    /// # Errors
    /// Poisoned state rejects without modifying the previously retained projection.
    pub fn replace(&self, projection: PreparedTray) -> Result<(), TrayError> {
        self.mailbox
            .lock()
            .map_err(|_| TrayError::BridgeUnavailable)?
            .projection = projection;
        self.update_pending.store(true, Ordering::Release);
        Ok(())
    }
    /// Drain atomically with clearing the outstanding-wake marker.
    ///
    /// The UI owner first shows the window when `restore_required`, acknowledges
    /// only successful restoration, then re-resolves each optional intention
    /// through `FixtureTrayMenu::request` with its current private modal/exit guard.
    ///
    /// # Errors
    /// Poisoned state is unavailable; no stale action is returned as current.
    pub fn drain(&self) -> Result<TrayEvent, TrayError> {
        let mut slot = self
            .mailbox
            .lock()
            .map_err(|_| TrayError::BridgeUnavailable)?;
        let mut next = std::mem::take(&mut slot.pending);
        let loss = self.loss_generation.load(Ordering::Acquire);
        next.restore =
            (loss != self.restored_generation.load(Ordering::Acquire)).then(|| RestoreToken {
                generation: loss,
                origin: Arc::clone(&self.origin),
            });
        self.queued.store(false, Ordering::Release);
        Ok(next)
    }
    /// Acknowledge the UI owner's successful show; this submits no native command.
    ///
    /// # Errors
    /// Reject a token from a replaced/different observer without changing readiness.
    pub fn acknowledge_restored_window(&self, token: RestoreToken) -> Result<(), TrayError> {
        let RestoreToken { generation, origin } = token;
        if !Arc::ptr_eq(&self.origin, &origin) {
            return Err(TrayError::StaleObserver);
        }
        // A later loss stays unequal even if it races this old acknowledgement.
        if generation != self.loss_generation.load(Ordering::Acquire) {
            return Err(TrayError::StaleObserver);
        }
        self.restored_generation
            .fetch_max(generation, Ordering::AcqRel);
        Ok(())
    }
    /// Snapshot only fixed-size prepared data, releasing its lock before native calls.
    ///
    /// # Errors
    /// Poisoned state grants no native update or menu action.
    pub fn prepared_projection(&self) -> Result<PreparedTray, TrayError> {
        self.mailbox
            .lock()
            .map(|slot| slot.projection.clone())
            .map_err(|_| TrayError::BridgeUnavailable)
    }
    /// Advance only on this worker's observed loss, never wrapping a generation.
    #[cfg(any(windows, test))]
    fn record_loss(&self) {
        let old = self.loss_generation.load(Ordering::Acquire);
        if let Some(next) = old.checked_add(1) {
            self.loss_generation.store(next, Ordering::Release);
        } else {
            self.cleanup_failed.store(true, Ordering::Release);
        }
    }
    /// Coalesce a typed event and notify outside the state lock.
    #[cfg(any(windows, test))]
    fn publish(&self, action: Option<TrayItemId>, failure: bool, loss: bool) {
        if loss {
            self.registered.store(false, Ordering::Release);
            self.record_loss();
        }
        let notify = if let Ok(mut slot) = self.mailbox.lock() {
            if action.is_some() {
                slot.pending.action = action;
            }
            slot.pending.native_failure |= failure;
            !self.queued.swap(true, Ordering::AcqRel)
        } else {
            self.registered.store(false, Ordering::Release);
            false
        };
        if notify && !self.wake.enqueue() {
            self.queued.store(false, Ordering::Release);
            self.record_loss();
        }
    }
}

/// Only notification-area operations, never service or clock commands.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IconOperation {
    /// Add the code-owned icon.
    Add,
    /// Set the v4 callback contract after Add acknowledges.
    SetVersion,
    /// Replace already-registered presentation.
    Modify,
}
/// Narrow native call seam for source-bound registration-order controls.
pub trait IconCalls {
    /// Return the actual outcome for this one allowed operation.
    fn notify(&mut self, operation: IconOperation) -> bool;
}
/// A new icon is usable only after both actual Add and `SetVersion` acknowledgements.
/// Existing updates must acknowledge Modify; no failed call is converted to success.
#[must_use]
pub fn registration_update(calls: &mut impl IconCalls, registered: bool) -> bool {
    if registered {
        calls.notify(IconOperation::Modify)
    } else {
        calls.notify(IconOperation::Add) && calls.notify(IconOperation::SetVersion)
    }
}

/// Encode only the six private menu IDs, reserving zero for cancellation.
#[must_use]
pub const fn native_command(id: TrayItemId) -> usize {
    match id {
        TrayItemId::Open => 1,
        TrayItemId::ContextualCheck => 2,
        TrayItemId::Diagnostics => 3,
        TrayItemId::Settings => 4,
        TrayItemId::About => 5,
        TrayItemId::QuitUi => 6,
    }
}

/// Terminal cleanup can be recorded once; failed completion never becomes success.
#[cfg(any(windows, test))]
struct CompletionLatch<E: Copy> {
    /// First actual terminal result, not a timeout or request.
    outcome: Option<Result<(), E>>,
}
#[cfg(any(windows, test))]
impl<E: Copy> CompletionLatch<E> {
    /// Return an actually recorded completion only.
    fn result(&self) -> Option<Result<(), E>> {
        self.outcome
    }
    /// Preserve the first terminal observation, including a failure.
    ///
    /// # Errors
    /// Returns the first recorded failure, without replacing it with later success.
    fn finish(&mut self, value: Result<(), E>) -> Result<(), E> {
        let prior = self.outcome.get_or_insert(value);
        *prior
    }
}

#[cfg(any(windows, test))]
impl<E: Copy> Default for CompletionLatch<E> {
    /// No terminal outcome is manufactured before an actual worker completion.
    fn default() -> Self {
        Self { outcome: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tray_ui::{TrayCheckBlock, TrayCheckContext, TrayRejection};
    use std::sync::atomic::AtomicUsize;

    /// Fixed fake scheduler; no Windows, Slint or clipboard effect exists.
    struct Wake {
        count: AtomicUsize,
        accepted: bool,
    }
    impl UiWake for Wake {
        /// Observe one attempted bounded UI scheduling operation.
        fn enqueue(&self) -> bool {
            self.count.fetch_add(1, Ordering::SeqCst);
            self.accepted
        }
    }
    /// Construct an honest disconnected fixed menu with no native operation.
    fn menu() -> FixtureTrayMenu {
        FixtureTrayMenu::new(
            TrayCheckContext::Synchronization,
            TrayCheckBlock::NativeAdapterUnavailable,
        )
    }
    /// One fresh isolated bounded bridge; tests never claim native registration.
    ///
    /// # Errors
    /// Propagates a changed invalid engineering fixture instead of bypassing admission.
    fn bridge(accepted: bool) -> Result<(Arc<TrayBridge>, Arc<Wake>), TrayError> {
        let wake = Arc::new(Wake {
            count: AtomicUsize::new(0),
            accepted,
        });
        let prepared = PreparedTray::fixture(menu(), TrayInteraction::Free)?;
        {
            let boundary = Arc::clone(&wake);
            Ok((TrayBridge::new(prepared, boundary), wake))
        }
    }

    /// Native formatting preserves literal ampersands and rejects every capacity bypass.
    ///
    /// # Panics
    /// Fails if menu markup changes meaning, buffers truncate, or invalid input survives.
    ///
    /// # Errors
    /// Propagates an unexpected checked fixture or bridge admission failure.
    #[test]
    fn literal_text_and_post_escape_budget_are_checked() -> Result<(), Box<dyn std::error::Error>> {
        let text = NativeText::prepare("A&B", true)?;
        assert_eq!(&text.units()[..5], &[65, 38, 38, 66, 0]);
        assert!(NativeText::prepare(&"x".repeat(127), true).is_ok());
        assert_eq!(
            NativeText::prepare(&"x".repeat(128), false),
            Err(TrayError::TextBounds)
        );
        assert_eq!(
            NativeText::prepare(&"&".repeat(64), true),
            Err(TrayError::TextBounds)
        );
        for bad in ["", "   ", "x\0y", "x\ty", "x\ny", "\u{1f642}", "\u{202e}"] {
            assert_eq!(
                NativeText::prepare(bad, true),
                Err(TrayError::TextCharacters)
            );
        }
        assert_eq!(
            NativeText::prepare(&"x".repeat(1025), true),
            Err(TrayError::TextBounds)
        );
        assert!(NativeText::prepare("NevroTick — unavailable", false).is_ok());
        Ok(())
    }
    /// Every popup command is decoded independently of row labels or earlier enabled state.
    ///
    /// # Panics
    /// Fails on a fallback identity, direct quit confirmation or stale modal bypass.
    ///
    /// # Errors
    /// Propagates an unexpected checked fixture or bridge admission failure.
    #[test]
    fn popup_identity_is_closed_and_receiver_still_rechecks_context()
    -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(popup_selection(0), Ok(None));
        for raw in [i32::MIN, -1, 7, i32::MAX] {
            assert_eq!(popup_selection(raw), Err(TrayError::InvalidCommand));
        }
        let prepared = PreparedTray::fixture(menu(), TrayInteraction::Free)?;
        for item in prepared.items() {
            let native = i32::try_from(native_command(item.id()))?;
            assert_eq!(popup_selection(native), Ok(Some(item.id())));
            assert_eq!(
                menu().request(item.id().view_id(), TrayInteraction::DialogOwned),
                Err(TrayRejection::DialogOwned)
            );
        }
        assert!(!prepared.items()[1].enabled());
        assert!(matches!(
            menu().request(1, TrayInteraction::Free),
            Err(TrayRejection::CheckUnavailable(_))
        ));
        assert_eq!(
            menu().request(5, TrayInteraction::Free),
            Ok(crate::shell::ShellRequest::Menu(
                crate::shell::TrayMenuAction::QuitUi
            ))
        );
        Ok(())
    }
    /// A burst consumes one event slot and one outstanding scheduling operation.
    ///
    /// # Panics
    /// Fails if input grows a queue, fabricates readiness or loses latest bounded action.
    ///
    /// # Errors
    /// Propagates an unexpected checked fixture or bridge admission failure.
    #[test]
    fn bridge_coalesces_and_is_unavailable_without_native_acknowledgement()
    -> Result<(), Box<dyn std::error::Error>> {
        let (bridge, wake) = bridge(true)?;
        assert_eq!(bridge.availability(), AdapterAvailability::Unavailable);
        for _ in 0..1000 {
            bridge.publish(Some(TrayItemId::About), false, false);
        }
        assert_eq!(wake.count.load(Ordering::SeqCst), 1);
        let event = bridge.drain()?;
        assert_eq!(event.action, Some(TrayItemId::About));
        assert_eq!(event.restore, None);
        bridge.publish(Some(TrayItemId::Open), false, false);
        assert_eq!(wake.count.load(Ordering::SeqCst), 2);
        assert_eq!(bridge.drain()?.action, Some(TrayItemId::Open));
        Ok(())
    }
    /// A previous restoration acknowledgement cannot erase a later observed loss.
    ///
    /// # Panics
    /// Fails if stale consent/readiness is accepted or a real successful neighbor stays blocked.
    ///
    /// # Errors
    /// Propagates an unexpected checked fixture or bridge admission failure.
    #[test]
    fn loss_generation_stays_unavailable_until_the_actual_token_is_acknowledged()
    -> Result<(), Box<dyn std::error::Error>> {
        let (bridge, _) = bridge(true)?;
        bridge.registered.store(true, Ordering::Release);
        assert_eq!(bridge.availability(), AdapterAvailability::Ready);
        bridge.publish(None, true, true);
        let old = bridge.drain()?.restore.ok_or("loss missing")?;
        bridge.publish(None, false, true);
        assert_eq!(
            bridge.acknowledge_restored_window(old),
            Err(TrayError::StaleObserver)
        );
        bridge.registered.store(true, Ordering::Release);
        assert_eq!(bridge.availability(), AdapterAvailability::Unavailable);
        let current = bridge.drain()?.restore.ok_or("new loss missing")?;
        bridge.acknowledge_restored_window(current)?;
        assert_eq!(bridge.availability(), AdapterAvailability::Ready);
        bridge.stopping.store(true, Ordering::Release);
        assert_eq!(bridge.availability(), AdapterAvailability::Unavailable);
        Ok(())
    }
    /// Replaced-instance and replayed acknowledgements cannot clear or roll back readiness.
    ///
    /// # Errors
    /// Propagates an unexpected checked fixture or bridge failure.
    ///
    /// # Panics
    /// Fails if another instance's token or stale token changes the accepted generation.
    #[test]
    fn restoration_tokens_bind_instance_and_do_not_roll_back()
    -> Result<(), Box<dyn std::error::Error>> {
        let (first, _) = bridge(true)?;
        let (second, _) = bridge(true)?;
        first.publish(None, false, true);
        second.publish(None, false, true);
        let old = first.drain()?.restore.ok_or("first loss missing")?;
        let foreign = second.drain()?.restore.ok_or("second loss missing")?;
        assert_eq!(
            first.acknowledge_restored_window(foreign),
            Err(TrayError::StaleObserver)
        );
        first.acknowledge_restored_window(old.clone())?;
        first.publish(None, false, true);
        let current = first.drain()?.restore.ok_or("new loss missing")?;
        first.acknowledge_restored_window(current)?;
        assert_eq!(
            first.acknowledge_restored_window(old),
            Err(TrayError::StaleObserver)
        );
        assert_eq!(
            first.loss_generation.load(Ordering::Acquire),
            first.restored_generation.load(Ordering::Acquire)
        );
        Ok(())
    }
    /// Scheduling failure invalidates hide permission and remains a sticky recovery obligation.
    ///
    /// # Panics
    /// Fails if failed delivery appears usable or is cleared by draining alone.
    ///
    /// # Errors
    /// Propagates an unexpected checked fixture or bridge admission failure.
    #[test]
    fn failed_wake_never_looks_like_a_usable_observer() -> Result<(), Box<dyn std::error::Error>> {
        let (bridge, wake) = bridge(false)?;
        bridge.registered.store(true, Ordering::Release);
        bridge.publish(Some(TrayItemId::Open), false, false);
        assert_eq!(wake.count.load(Ordering::SeqCst), 1);
        assert_eq!(bridge.availability(), AdapterAvailability::Unavailable);
        assert!(bridge.drain()?.restore.is_some());
        assert_eq!(bridge.availability(), AdapterAvailability::Unavailable);
        Ok(())
    }
    /// Old registration is insufficient while a new native projection has no ack.
    ///
    /// # Errors
    /// Propagates an unexpected checked fixture or bridge admission failure.
    ///
    /// # Panics
    /// Fails if an update can expose Ready between private consumption and native ack.
    #[test]
    fn native_publication_barrier_preserves_pending_readiness()
    -> Result<(), Box<dyn std::error::Error>> {
        let (bridge, _) = bridge(true)?;
        bridge.registered.store(true, Ordering::Release);
        bridge.replace(PreparedTray::fixture(menu(), TrayInteraction::DialogOwned)?)?;
        assert_eq!(bridge.availability(), AdapterAvailability::Unavailable);
        bridge.publishing.store(true, Ordering::Release);
        bridge.update_pending.store(false, Ordering::Release);
        assert_eq!(bridge.availability(), AdapterAvailability::Unavailable);
        bridge.publishing.store(false, Ordering::Release);
        assert_eq!(bridge.availability(), AdapterAvailability::Ready);
        Ok(())
    }
    /// Generation exhaustion latches unavailable instead of wrapping to an accepted old token.
    ///
    /// # Panics
    /// Fails if finite generation exhaustion can manufacture hide permission.
    ///
    /// # Errors
    /// Propagates an unexpected checked fixture or bridge admission failure.
    #[test]
    fn generation_exhaustion_fails_closed() -> Result<(), Box<dyn std::error::Error>> {
        let (bridge, _) = bridge(true)?;
        bridge.loss_generation.store(u64::MAX, Ordering::Release);
        bridge
            .restored_generation
            .store(u64::MAX, Ordering::Release);
        bridge.registered.store(true, Ordering::Release);
        bridge.publish(None, false, true);
        assert!(bridge.cleanup_failed.load(Ordering::Acquire));
        assert_eq!(bridge.availability(), AdapterAvailability::Unavailable);
        Ok(())
    }

    /// Record the actual registration sequence, with independent fixed outcomes.
    struct Calls {
        results: [bool; 2],
        operations: Vec<IconOperation>,
    }
    impl IconCalls for Calls {
        /// Each fake native outcome is consumed once in order.
        fn notify(&mut self, op: IconOperation) -> bool {
            let index = self.operations.len();
            self.operations.push(op);
            self.results.get(index).copied().unwrap_or(false)
        }
    }
    /// Repeated completion polls never erase a first failed cleanup observation.
    ///
    /// # Panics
    /// Fails if a later success or missing worker can replace the sticky failure.
    #[test]
    fn completion_failure_remains_sticky() {
        let mut completion = CompletionLatch::<u8>::default();
        assert_eq!(completion.result(), None);
        assert_eq!(completion.finish(Err(1)), Err(1));
        assert_eq!(completion.result(), Some(Err(1)));
        assert_eq!(completion.finish(Ok(())), Err(1));
        assert_eq!(completion.result(), Some(Err(1)));
    }
    /// The production sequence requires version acknowledgement and never hides Add failure.
    ///
    /// # Panics
    /// Fails if `SetVersion` is omitted, retried after Add failure or Modify assumes success.
    #[test]
    fn registration_acknowledgements_use_actual_order_and_failure() {
        for (results, expected, ops) in [
            ([false, true], false, vec![IconOperation::Add]),
            (
                [true, false],
                false,
                vec![IconOperation::Add, IconOperation::SetVersion],
            ),
            (
                [true, true],
                true,
                vec![IconOperation::Add, IconOperation::SetVersion],
            ),
        ] {
            let mut calls = Calls {
                results,
                operations: Vec::new(),
            };
            assert_eq!(registration_update(&mut calls, false), expected);
            assert_eq!(calls.operations, ops);
        }
        let mut calls = Calls {
            results: [false, true],
            operations: Vec::new(),
        };
        assert!(!registration_update(&mut calls, true));
        assert_eq!(calls.operations, vec![IconOperation::Modify]);
    }
}

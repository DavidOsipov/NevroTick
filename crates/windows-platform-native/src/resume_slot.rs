// SPDX-License-Identifier: GPL-2.0-only
//! One-shot callback admission. Delayed notifications never reach a later owner.

use std::sync::{Mutex, OnceLock, PoisonError};

/// The observer's exact one-shot invalidation operation.
pub(super) type Action = Box<dyn FnOnce() -> Result<(), ()> + Send>;

/// Notification that Windows is about to suspend (`PBT_APMSUSPEND`).
const SUSPEND: u32 = 0x04;

/// Automatic resume, including wake without interactive user activity.
pub(super) const AUTOMATIC_RESUME: u32 = 0x12;

/// Own one invocation's signal for the entire lifetime of the process.
pub(super) struct ResumeSlot {
    /// Initialization consumes the only registration attempt.
    action: OnceLock<Mutex<Option<Action>>>,
}

impl ResumeSlot {
    /// Create an unused callback slot without requesting Windows effects.
    pub(super) const fn new() -> Self {
        Self {
            action: OnceLock::new(),
        }
    }

    /// Bind the first and only observer, returning whether it was installed.
    pub(super) fn bind(&self, action: Action) -> bool {
        self.action.set(Mutex::new(Some(action))).is_ok()
    }

    /// Consume the invalidation on suspend or automatic resume, serialized with disarm.
    ///
    /// # Errors
    /// Return the observer's failure or a poisoned slot as a callback refusal.
    pub(super) fn notify(&self, event: u32) -> Result<(), ()> {
        if !matches!(event, SUSPEND | AUTOMATIC_RESUME) {
            return Ok(());
        }
        let Some(action) = self.action.get() else {
            return Ok(());
        };
        let mut pending: std::sync::MutexGuard<'_, Option<Action>> =
            action.lock().map_err(|_| ())?;
        let Some(deliver) = pending.take() else {
            return Ok(());
        };
        let result: Result<(), ()> = deliver();
        drop(pending);
        result
    }

    /// Close delivery and return the retired action outside the slot lock.
    pub(super) fn disarm(&self) -> Option<Action> {
        self.action
            .get()
            .and_then(|action: &Mutex<Option<Action>>| {
                action.lock().unwrap_or_else(PoisonError::into_inner).take()
            })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex, TryLockError, Weak};

    use super::{Action, ResumeSlot};

    /// Startup notifications have no delivery target and do not spend admission.
    ///
    /// # Panics
    /// Fail if an unused slot refuses a notification or loses its first bind.
    #[test]
    fn notifications_and_disarm_before_bind_leave_admission_open() {
        let slot: ResumeSlot = ResumeSlot::new();
        assert_eq!(slot.notify(4), Ok(()));
        assert_eq!(slot.notify(18), Ok(()));
        assert!(slot.disarm().is_none());
        assert!(slot.bind(Box::new(|| Err(()))));
        assert_eq!(slot.notify(18), Err(()));
        assert_eq!(slot.notify(4), Ok(()));
        assert!(slot.disarm().is_none());
    }

    /// Observe that delivery holds the slot lock until the action returns.
    ///
    /// # Panics
    /// Fail if delivery runs outside the lock or leaves it unavailable afterward.
    #[test]
    fn notification_serializes_action_with_disarm() {
        let slot: Arc<ResumeSlot> = Arc::new(ResumeSlot::new());
        let callback_slot: Weak<ResumeSlot> = Arc::downgrade(&slot);
        let serialized: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let lock_observation: Arc<AtomicBool> = Arc::clone(&serialized);
        assert!(slot.bind(Box::new(move || {
            let slot: Arc<ResumeSlot> = callback_slot.upgrade().ok_or(())?;
            let action: &Mutex<Option<Action>> = slot.action.get().ok_or(())?;
            lock_observation.store(
                matches!(action.try_lock(), Err(TryLockError::WouldBlock)),
                Ordering::Relaxed,
            );
            Ok(())
        })));
        assert_eq!(slot.notify(4), Ok(()));
        assert!(serialized.load(Ordering::Relaxed));
        assert!(
            slot.action
                .get()
                .is_some_and(|action: &Mutex<Option<Action>>| action.try_lock().is_ok())
        );
        assert_eq!(slot.notify(18), Ok(()));
        assert!(slot.disarm().is_none());
    }

    /// Deliveries refuse a poisoned slot, but retirement can recover its capture.
    ///
    /// # Errors
    /// Return an error if the test cannot access the bound slot.
    ///
    /// # Panics
    /// Fail if a poisoned slot delivers, leaks its capture or permits another bind.
    #[test]
    fn poisoned_slot_refuses_delivery_and_releases_capture() -> Result<(), ()> {
        let slot: ResumeSlot = ResumeSlot::new();
        let owner: Arc<()> = Arc::new(());
        let captured: Arc<()> = Arc::clone(&owner);
        assert!(slot.bind(Box::new(move || {
            drop(captured);
            Ok(())
        })));
        let action: &Mutex<Option<Action>> = slot.action.get().ok_or(())?;
        let failed: std::thread::Result<Result<(), ()>> = std::panic::catch_unwind(|| {
            let _held: std::sync::MutexGuard<'_, Option<Action>> = action.lock().map_err(|_| ())?;
            std::panic::resume_unwind(Box::new(()))
        });
        assert!(failed.is_err());
        assert!(action.is_poisoned());
        assert_eq!(slot.notify(4), Err(()));
        assert_eq!(Arc::strong_count(&owner), 2);
        let retired: Action = slot.disarm().ok_or(())?;
        assert_eq!(Arc::strong_count(&owner), 2);
        drop(retired);
        assert_eq!(Arc::strong_count(&owner), 1);
        assert!(slot.disarm().is_none());
        assert!(!slot.bind(Box::new(|| Ok(()))));
        Ok(())
    }

    /// Exercise both first-event orders using literal Win32 event IDs.
    ///
    /// # Panics
    /// Fail if an unrelated event consumes delivery or either event delivers twice.
    #[test]
    fn suspend_or_automatic_resume_delivers_once() {
        for first in [4_u32, 18] {
            let slot: ResumeSlot = ResumeSlot::new();
            let delivered: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
            let observed: Arc<AtomicUsize> = Arc::clone(&delivered);
            assert!(slot.bind(Box::new(move || {
                let _previous: usize = observed.fetch_add(1, Ordering::Relaxed);
                Ok(())
            })));
            assert_eq!(slot.notify(7), Ok(()));
            assert_eq!(slot.notify(u32::MAX), Ok(()));
            assert_eq!(delivered.load(Ordering::Relaxed), 0);
            assert_eq!(slot.notify(first), Ok(()));
            assert_eq!(delivered.load(Ordering::Relaxed), 1, "first event {first}");
            assert_eq!(slot.notify(4), Ok(()));
            assert_eq!(slot.notify(18), Ok(()));
            assert_eq!(delivered.load(Ordering::Relaxed), 1);
        }
    }

    /// Disarming the original observer leaves a delayed callback inert.
    ///
    /// # Panics
    /// Fail if delivery remains possible or the old slot can be rebound.
    #[test]
    fn disarm_releases_original_action_and_rejects_rebind() {
        let slot: ResumeSlot = ResumeSlot::new();
        let owner: Arc<()> = Arc::new(());
        let captured: Arc<()> = Arc::clone(&owner);
        assert!(slot.bind(Box::new(move || {
            drop(captured);
            Err(())
        })));
        assert_eq!(Arc::strong_count(&owner), 2);
        drop(slot.disarm());
        assert_eq!(Arc::strong_count(&owner), 1);
        assert_eq!(slot.notify(4), Ok(()));
        assert_eq!(slot.notify(18), Ok(()));
        assert!(!slot.bind(Box::new(|| Ok(()))));
    }
}

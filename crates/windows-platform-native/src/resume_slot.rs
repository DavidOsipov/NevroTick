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
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::ResumeSlot;

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

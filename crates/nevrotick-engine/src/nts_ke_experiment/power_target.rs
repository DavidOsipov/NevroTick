// SPDX-License-Identifier: GPL-2.0-only
//! Bind observed power events to at most one original acquisition Stop scope.
//!
//! The process observer and the acquisition owner share this target. Delivery,
//! activation and retirement serialize on its gate. Delivery may take the
//! original Stop gate while holding the target gate. Callers acquire or release
//! a binding without holding a Stop gate. A delivery requests cooperative Stop
//! and acknowledges no worker or native retirement.

use std::sync::{LockResult, Mutex, MutexGuard, PoisonError};

use super::{Failed, Failure, StopHandle, job::SubmissionReservation};

/// One process observer's bounded route to its owner's current invocation.
pub(super) struct PowerTarget {
    /// Serializes idle observations, active delivery and lease retirement.
    state: Mutex<TargetState>,
}

/// Relevant event delivered to the original managed acquisition observer.
#[derive(Clone, Copy)]
pub(super) enum PowerEvent {
    /// A delivered notification of impending suspension.
    Suspending,
    /// A delivered notification of automatic resume.
    AutomaticResume,
}

/// What the original power binding recorded through its serialized retirement.
///
/// A historical observation supplies no clock mapping or current authorization.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use = "use the observation only as historical power-delivery evidence"]
pub enum ManagedPowerObservation {
    /// No event reached the original, available binding before retirement.
    NoEventObserved,
    /// At least one event reached the original binding before retirement.
    Invalidated,
    /// The original binding could not establish even an event-free observation.
    Unavailable,
}

/// Current power epoch and, independently, the one retained active binding.
struct TargetState {
    /// Advance on every delivery and successful activation. Exhaustion is final.
    epoch: Option<u64>,
    /// A delivered suspension still awaits automatic resume; absence establishes no continuity.
    suspended: bool,
    /// The one original Stop scope held until its own lease retires.
    binding: Option<Binding>,
}

/// Preserve the activation identity even if delivery advances the current epoch.
struct Binding {
    /// Unique activation epoch within the target.
    epoch: u64,
    /// The actual acquired invocation latch.
    stop: StopHandle,
    /// Sticky evidence of delivery, independent of Stop and epoch advancement.
    delivered: bool,
}

/// One idle epoch observed before preparing an original invocation.
pub(super) struct PreparedPower<'target> {
    /// Physical target instance observed before preparation.
    target: &'target PowerTarget,
    /// Exact idle epoch that must still be current at activation.
    epoch: u64,
}

/// The unique owner of an invocation's binding until retirement.
#[must_use = "retain the binding through the actual acquisition and its collection"]
pub(super) struct ActivePower<'target> {
    /// Physical owner of the registered binding.
    target: &'target PowerTarget,
    /// Identify this activation until this owner detaches it exactly once.
    epoch: Option<u64>,
}

impl PowerTarget {
    /// Start one idle target with no current acquisition or Stop route.
    pub(super) const fn new() -> Self {
        Self {
            state: Mutex::new(TargetState {
                epoch: Some(0),
                suspended: false,
                binding: None,
            }),
        }
    }

    /// Observe the idle epoch before constructing the acquisition's Control.
    ///
    /// # Errors
    /// Refuse a suspended, active, exhausted or poisoned target without changing it.
    pub(super) fn prepare(&self) -> Result<PreparedPower<'_>, Failure> {
        let state: MutexGuard<'_, TargetState> = self.state.lock().map_err(|_| Failure::Control)?;
        if state.suspended || state.binding.is_some() {
            return Err(Failure::Control);
        }
        let epoch: u64 = state.epoch.ok_or(Failure::Control)?;
        Ok(PreparedPower {
            target: self,
            epoch,
        })
    }

    /// Apply one delivered event at the serialized current Stop scope.
    ///
    /// An idle delivery invalidates any earlier preparation. A delivered suspension
    /// also refuses fresh preparation until automatic resume. The active route stays
    /// attached for later events until its lease retires.
    ///
    /// # Errors
    /// Refuse exhausted or poisoned authority and retain the Stop attempt on a
    /// still-active binding. A failed Stop closes further admission.
    pub(super) fn deliver(&self, event: PowerEvent) -> Result<(), Failure> {
        let locked: LockResult<MutexGuard<'_, TargetState>> = self.state.lock();
        let poisoned: bool = locked.is_err();
        let mut state: MutexGuard<'_, TargetState> = locked.unwrap_or_else(PoisonError::into_inner);
        let stop_failed: bool = state.binding.as_mut().is_some_and(|binding: &mut Binding| {
            binding.delivered = true;
            binding.stop.request_stop().is_err()
        });
        state.suspended = match event {
            PowerEvent::Suspending => true,
            PowerEvent::AutomaticResume => false,
        };
        state.epoch = if poisoned || stop_failed {
            None
        } else {
            state.epoch.and_then(|epoch: u64| epoch.checked_add(1))
        };
        state.epoch.map(|_| ()).ok_or(Failure::Control)
    }
}

impl<'target> PreparedPower<'target> {
    /// Attach the actual reserved invocation's original Stop handle.
    ///
    /// # Errors
    /// Refuse an event crossing preparation, an occupied route, or exhausted
    /// or poisoned authority before publishing the binding.
    pub(super) fn activate(
        self,
        reservation: &SubmissionReservation,
    ) -> Result<(ActivePower<'target>, StopHandle), Failed> {
        let (_, stop): (&super::job::WorkerSlot, StopHandle) = reservation.observer_entry()?;
        let active: ActivePower<'target> =
            self.activate_scope(stop.clone()).map_err(Failed::from)?;
        Ok((active, stop))
    }

    /// Publish the freshly checked original Stop under this preparation's idle epoch.
    /// # Errors
    /// Refuse changed or occupied binding authority while holding the target gate.
    fn activate_scope(self, stop: StopHandle) -> Result<ActivePower<'target>, Failure> {
        let mut state: MutexGuard<'_, TargetState> =
            self.target.state.lock().map_err(|_| Failure::Control)?;
        if state.binding.is_some() || state.epoch != Some(self.epoch) {
            return Err(Failure::Control);
        }
        state.epoch = self.epoch.checked_add(1);
        let epoch: u64 = state.epoch.ok_or(Failure::Control)?;
        state.binding = Some(Binding {
            epoch,
            stop,
            delivered: false,
        });
        Ok(ActivePower {
            target: self.target,
            epoch: Some(epoch),
        })
    }
}

impl ActivePower<'_> {
    /// Observe delivery and detach this exact binding in one target transaction.
    /// Consume the lease without acknowledging any worker or native completion.
    pub(super) fn retire(mut self) -> ManagedPowerObservation {
        self.detach()
    }

    /// Retire once even on a poisoned gate. Keep recorded delivery on refusal.
    /// The owner holds no Stop guard and leaves any other binding untouched.
    fn detach(&mut self) -> ManagedPowerObservation {
        let Some(epoch): Option<u64> = self.epoch.take() else {
            return ManagedPowerObservation::Unavailable;
        };
        let locked: LockResult<MutexGuard<'_, TargetState>> = self.target.state.lock();
        let poisoned: bool = locked.is_err();
        let mut state: MutexGuard<'_, TargetState> = locked.unwrap_or_else(PoisonError::into_inner);
        let Some(binding): Option<&Binding> = state
            .binding
            .as_ref()
            .filter(|binding: &&Binding| binding.epoch == epoch)
        else {
            return ManagedPowerObservation::Unavailable;
        };
        let observation: ManagedPowerObservation = if binding.delivered {
            ManagedPowerObservation::Invalidated
        } else if poisoned || state.epoch != Some(epoch) {
            ManagedPowerObservation::Unavailable
        } else {
            ManagedPowerObservation::NoEventObserved
        };
        state.binding = None;
        observation
    }
}

impl Drop for ActivePower<'_> {
    /// Detach the exact original binding on refusal before a report can be made.
    fn drop(&mut self) {
        let _observation: ManagedPowerObservation = self.detach();
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;
    use std::sync::{Arc, Mutex, MutexGuard};
    use std::thread::{self, Scope, ScopedJoinHandle};

    use super::super::{StopDisposition, StopScope};
    use super::{
        ActivePower, Failure, ManagedPowerObservation, PowerEvent, PowerTarget, PreparedPower,
        StopHandle, TargetState,
    };

    /// Create the original latch used by the actual cooperative Stop path.
    fn original_stop() -> StopHandle {
        StopHandle {
            scope: Arc::new(StopScope::new()),
        }
    }

    /// Exercise the exact observer target from a separate producer thread.
    ///
    /// # Errors
    /// Refuse a failed spawn, join or target delivery.
    fn deliver_from_producer(target: &PowerTarget) -> Result<(), Failure> {
        thread::scope(|scope: &Scope<'_, '_>| -> Result<(), Failure> {
            let callback: ScopedJoinHandle<'_, Result<(), Failure>> = thread::Builder::new()
                .spawn_scoped(scope, || target.deliver(PowerEvent::AutomaticResume))
                .map_err(|_| Failure::Control)?;
            callback.join().map_err(|_| Failure::Control)?
        })
    }

    /// Close the actual gate by unwinding while it is held without changing its value.
    ///
    /// # Errors
    /// Refuse absent actual poison or failed acquisition.
    fn poison<T: std::panic::RefUnwindSafe>(gate: &Mutex<T>) -> Result<(), Failure> {
        let caught: Result<Result<(), Failure>, Box<dyn Any + Send>> =
            std::panic::catch_unwind(|| -> Result<(), Failure> {
                let _guard: MutexGuard<'_, T> = gate.try_lock().map_err(|_| Failure::Control)?;
                std::panic::resume_unwind(Box::new(()));
            });
        if caught.is_err() && gate.is_poisoned() {
            Ok(())
        } else {
            Err(Failure::Control)
        }
    }

    /// A delivery between preparation and attach on another thread refuses the old epoch.
    /// # Errors
    /// Propagate target, binding, or callback-thread refusal.
    /// # Panics
    /// Fail if an idle event leaves the old preparation current or an active event misses its Stop.
    #[test]
    fn idle_delivery_invalidates_earlier_preparation() -> Result<(), Failure> {
        let target: PowerTarget = PowerTarget::new();
        let stale: PreparedPower<'_> = target.prepare()?;
        deliver_from_producer(&target)?;
        let original: StopHandle = original_stop();
        assert!(matches!(
            stale.activate_scope(original.clone()),
            Err(Failure::Control)
        ));
        assert_eq!(original.request_stop(), Ok(StopDisposition::Latched));

        let current: StopHandle = original_stop();
        let active: ActivePower<'_> = target.prepare()?.activate_scope(current.clone())?;
        assert!(!current.scope.lock()?.is_stopped());
        deliver_from_producer(&target)?;
        deliver_from_producer(&target)?;
        assert_eq!(current.request_stop(), Ok(StopDisposition::AlreadyLatched));
        drop(active);
        Ok(())
    }

    /// Refuse a fresh binding between delivered suspend and resume notifications.
    /// # Errors
    /// Preserve attachment or delivery refusal.
    /// # Panics
    /// Fail if a delivered suspend misses the active Stop or permits fresh preparation,
    /// or if the automatic resume reuses the preceding preparation or Stop scope.
    #[test]
    fn observed_suspend_refuses_preparation_until_automatic_resume() -> Result<(), Failure> {
        let target: PowerTarget = PowerTarget::new();
        let stale: PreparedPower<'_> = target.prepare()?;
        let original: StopHandle = original_stop();
        let active: ActivePower<'_> = target.prepare()?.activate_scope(original.clone())?;

        target.deliver(PowerEvent::Suspending)?;
        assert_eq!(original.request_stop(), Ok(StopDisposition::AlreadyLatched));
        drop(active);
        assert!(matches!(target.prepare(), Err(Failure::Control)));
        target.deliver(PowerEvent::Suspending)?;
        assert!(matches!(target.prepare(), Err(Failure::Control)));

        target.deliver(PowerEvent::AutomaticResume)?;
        let successor: StopHandle = original_stop();
        assert!(matches!(
            stale.activate_scope(successor.clone()),
            Err(Failure::Control)
        ));
        let _active: ActivePower<'_> = target.prepare()?.activate_scope(successor.clone())?;
        assert!(!successor.scope.lock()?.is_stopped());
        Ok(())
    }

    /// Keep the first binding exclusive and only Stop the successor after its attachment.
    /// # Errors
    /// Propagate target, binding, or callback-thread refusal.
    /// # Panics
    /// Fail if a retired preparation or Stop handle interferes with the next active scope.
    #[test]
    fn retirement_fences_old_preparation_and_stop_scope() -> Result<(), Failure> {
        let target: PowerTarget = PowerTarget::new();
        let prepared: PreparedPower<'_> = target.prepare()?;
        let duplicate: PreparedPower<'_> = target.prepare()?;
        let first: StopHandle = original_stop();
        let active: ActivePower<'_> = prepared.activate_scope(first.clone())?;
        assert!(matches!(target.prepare(), Err(Failure::Control)));
        assert_eq!(active.retire(), ManagedPowerObservation::NoEventObserved);

        let second: StopHandle = original_stop();
        assert!(matches!(
            duplicate.activate_scope(second.clone()),
            Err(Failure::Control)
        ));
        let successor: ActivePower<'_> = target.prepare()?.activate_scope(second.clone())?;
        assert_eq!(first.request_stop(), Ok(StopDisposition::Latched));
        assert!(!second.scope.lock()?.is_stopped());
        drop(successor);

        let third: StopHandle = original_stop();
        let successor: ActivePower<'_> = target.prepare()?.activate_scope(third.clone())?;
        deliver_from_producer(&target)?;
        assert!(!second.scope.lock()?.is_stopped());
        assert_eq!(third.request_stop(), Ok(StopDisposition::AlreadyLatched));
        assert_eq!(successor.retire(), ManagedPowerObservation::Invalidated);
        Ok(())
    }

    /// The final usable activation still gets stopped when delivery exhausts its epoch.
    /// # Errors
    /// Propagate fixture and original binding refusal.
    /// # Panics
    /// Fail if exhaustion skips the active Stop or reopens the target.
    #[test]
    fn exhausted_epoch_keeps_stop_delivery_and_refuses_reissue() -> Result<(), Failure> {
        let target: PowerTarget = PowerTarget::new();
        target.state.lock().map_err(|_| Failure::Control)?.epoch = Some(u64::MAX - 1);
        let original: StopHandle = original_stop();
        let active: ActivePower<'_> = target.prepare()?.activate_scope(original.clone())?;
        assert!(!original.scope.lock()?.is_stopped());
        assert_eq!(
            target.deliver(PowerEvent::AutomaticResume),
            Err(Failure::Control)
        );
        assert_eq!(original.request_stop(), Ok(StopDisposition::AlreadyLatched));
        assert_eq!(active.retire(), ManagedPowerObservation::Invalidated);
        assert!(matches!(target.prepare(), Err(Failure::Control)));
        assert_eq!(
            target.deliver(PowerEvent::AutomaticResume),
            Err(Failure::Control)
        );
        Ok(())
    }

    /// Poison permits only best-effort withdrawal and original lease retirement.
    /// # Errors
    /// Propagate original attachment or poison setup refusal.
    /// # Panics
    /// Fail if poison prevents the withdrawal attempt, exact detach, or final admission refusal.
    #[test]
    fn poisoned_target_still_requests_stop_without_reopening() -> Result<(), Failure> {
        let target: PowerTarget = PowerTarget::new();
        let original: StopHandle = original_stop();
        let active: ActivePower<'_> = target.prepare()?.activate_scope(original.clone())?;
        poison(&target.state)?;
        assert_eq!(
            target.deliver(PowerEvent::AutomaticResume),
            Err(Failure::Control)
        );
        assert_eq!(original.request_stop(), Ok(StopDisposition::AlreadyLatched));
        assert_eq!(active.retire(), ManagedPowerObservation::Invalidated);
        let state: MutexGuard<'_, TargetState> = target
            .state
            .lock()
            .err()
            .ok_or(Failure::Control)?
            .into_inner();
        assert!(state.binding.is_none());
        drop(state);
        assert!(matches!(target.prepare(), Err(Failure::Control)));
        Ok(())
    }

    /// An unobservable gate may retire its binding but cannot report an event-free history.
    /// # Errors
    /// Propagate setup and original binding refusal.
    /// # Panics
    /// Fail if poisoned evidence establishes an event-free observation or retains the binding.
    #[test]
    fn poisoned_retirement_without_delivery_is_unavailable() -> Result<(), Failure> {
        let target: PowerTarget = PowerTarget::new();
        let active: ActivePower<'_> = target.prepare()?.activate_scope(original_stop())?;
        poison(&target.state)?;
        assert_eq!(active.retire(), ManagedPowerObservation::Unavailable);
        let state: MutexGuard<'_, TargetState> = target
            .state
            .lock()
            .err()
            .ok_or(Failure::Control)?
            .into_inner();
        assert!(state.binding.is_none());
        Ok(())
    }

    /// A delivery to a poisoned Stop gate remains a refusal across retirement.
    /// # Errors
    /// Propagate original attachment or poison setup refusal.
    /// # Panics
    /// Fail if a failed active Stop leaves the target eligible for another preparation.
    #[test]
    fn failed_active_stop_closes_later_admission() -> Result<(), Failure> {
        let target: PowerTarget = PowerTarget::new();
        let original: StopHandle = original_stop();
        let active: ActivePower<'_> = target.prepare()?.activate_scope(original.clone())?;
        poison(&original.scope.stopped)?;
        assert_eq!(
            target.deliver(PowerEvent::AutomaticResume),
            Err(Failure::Control)
        );
        assert_eq!(active.retire(), ManagedPowerObservation::Invalidated);
        assert!(matches!(target.prepare(), Err(Failure::Control)));
        Ok(())
    }
}

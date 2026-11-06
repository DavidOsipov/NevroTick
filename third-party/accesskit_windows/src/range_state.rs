// SPDX-License-Identifier: GPL-3.0-only
// Original NevroTick text-range state hardening additions.
// Existing AccessKit/Chromium grants and notices remain unchanged in other files.

//! Borrow one coherent pair of text-range states without recursively read-locking.
//!
//! Identical lock references supply the same single guard to both operands.
//! Distinct locks remain read-locked together through the receiving comparison;
//! their values are not replaced with independently timed cloned snapshots.
//! This module supplies no COM identity, context, endpoint or action authority.
#![forbid(unsafe_code)]

use std::{
    fmt,
    sync::{RwLock, RwLockReadGuard},
};

/// Closed state-acquisition failure, without retained state or poison payloads.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ReadPairError {
    /// A required read guard could not be acquired through the receiving port.
    Unavailable,
}

impl fmt::Display for ReadPairError {
    /// Emit only fixed technical guidance, never the protected range state.
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        output.write_str("The text-range state could not be read.")
    }
}

impl std::error::Error for ReadPairError {}

/// Borrow both states during one comparison, acquiring an aliased lock only once.
///
/// Both references must identify the actual state owners selected by the caller.
/// The comparison receives immutable borrowed values and runs exactly once after
/// all required acquisitions succeed. All guards are released on return or unwind.
/// Distinct owners retain their existing left-then-right acquisition order and
/// simultaneous read protection. Callers retain their COM/context/endpoint checks.
///
/// # Errors
/// A poisoned read returns a closed error before invoking the comparison. A native
/// caller may preserve its existing poison policy by explicitly unwrapping it.
pub(crate) fn with_read_pair<T, R>(
    left: &RwLock<T>,
    right: &RwLock<T>,
    compare: impl FnOnce(&T, &T) -> R,
) -> Result<R, ReadPairError> {
    with_read_pair_acquire(left, right, read_state, compare)
}

/// Acquire one real shared guard while discarding any state-bearing poison value.
///
/// # Errors
/// Poisoned state is unavailable; no replacement value or comparison is produced.
fn read_state<T>(state: &RwLock<T>) -> Result<RwLockReadGuard<'_, T>, ReadPairError> {
    state.read().map_err(|_| ReadPairError::Unavailable)
}

/// Share exact alias handling with an injected real-lock acquisition observer.
///
/// The acquisition port must return a guard for its supplied owner. Production
/// always supplies `read_state`; tests observe actual shared locks without a
/// second Windows dependency scope. At most two acquisitions occur, with the
/// second omitted for identical references. The comparison never outlives guards.
///
/// # Errors
/// Any acquisition failure releases preceding guards and skips the comparison.
fn with_read_pair_acquire<'a, T, R>(
    left: &'a RwLock<T>,
    right: &'a RwLock<T>,
    mut acquire: impl FnMut(&'a RwLock<T>) -> Result<RwLockReadGuard<'a, T>, ReadPairError>,
    compare: impl FnOnce(&T, &T) -> R,
) -> Result<R, ReadPairError> {
    let left_state = acquire(left)?;
    if std::ptr::eq(left, right) {
        return Ok(compare(&left_state, &left_state));
    }
    let right_state = acquire(right)?;
    Ok(compare(&left_state, &right_state))
}

#[cfg(test)]
mod tests {
    //! Real shared locks exercise the same helper used by the Windows receiver.

    use std::{
        cell::Cell,
        error::Error,
        sync::{RwLock, TryLockError, mpsc::sync_channel},
        thread,
        time::{Duration, Instant},
    };

    use super::{ReadPairError, read_state, with_read_pair, with_read_pair_acquire};

    /// Bound scheduling admission and writer-start waiting without sleeps.
    const WAIT: Duration = Duration::from_secs(5);

    /// Same, distinct-equal and distinct-unequal values retain their identities.
    ///
    /// # Errors
    /// Propagates an unexpected real-lock acquisition failure.
    ///
    /// # Panics
    /// Fails if equality, operand order, alias identity or guard lifetime changes.
    #[test]
    fn public_helper_preserves_alias_and_distinct_comparisons() -> Result<(), ReadPairError> {
        let left = RwLock::new(7_u32);
        let equal = RwLock::new(7_u32);
        let unequal = RwLock::new(11_u32);

        assert!(with_read_pair(&left, &left, |a, b| {
            assert!(std::ptr::eq(a, b));
            assert!(matches!(left.try_write(), Err(TryLockError::WouldBlock)));
            a == b
        })?);
        assert!(with_read_pair(&left, &equal, |a, b| {
            assert!(!std::ptr::eq(a, b));
            assert!(matches!(left.try_write(), Err(TryLockError::WouldBlock)));
            assert!(matches!(equal.try_write(), Err(TryLockError::WouldBlock)));
            a == b
        })?);
        assert_eq!(with_read_pair(&left, &unequal, |a, b| (*a, *b))?, (7, 11));
        assert!(!with_read_pair(&left, &unequal, |a, b| a == b)?);
        assert!(left.try_write().is_ok());
        assert!(equal.try_write().is_ok());
        assert!(unequal.try_write().is_ok());
        Ok(())
    }

    /// Wait until a writer is queued while our caller retains its first guard.
    ///
    /// Since that guard prevents exclusive ownership, `WouldBlock` establishes
    /// reader exclusion by a waiting writer rather than a currently held writer.
    ///
    /// # Errors
    /// Scheduling deadline or poisoned state rejects the controlled interleaving.
    fn wait_for_queued_writer(state: &RwLock<u32>) -> Result<(), ReadPairError> {
        let deadline = Instant::now()
            .checked_add(WAIT)
            .ok_or(ReadPairError::Unavailable)?;
        loop {
            match state.try_read() {
                Err(TryLockError::WouldBlock) => return Ok(()),
                Err(TryLockError::Poisoned(_)) => return Err(ReadPairError::Unavailable),
                Ok(guard) => drop(guard),
            }
            if Instant::now() >= deadline {
                return Err(ReadPairError::Unavailable);
            }
            thread::yield_now();
        }
    }

    /// A queued writer cannot turn an identical-state comparison into a second read.
    ///
    /// The first injected acquisition holds an actual read guard, starts a scoped
    /// writer, and observes that writer's reader exclusion. Any erroneous second
    /// acquisition uses `try_read`, returning a closed failure instead of hanging.
    /// Every error path releases the first guard before joining the scoped writer.
    ///
    /// # Errors
    /// Propagates lock/interleaving failure, writer-start timeout or writer panic.
    ///
    /// # Panics
    /// Fails if alias handling reacquires, skips comparison or prevents the writer
    /// from changing state after the comparison releases its sole guard.
    #[test]
    fn queued_writer_does_not_require_a_second_aliased_read() -> Result<(), Box<dyn Error>> {
        let state = RwLock::new(7_u32);
        let acquisitions = Cell::new(0_u32);
        let comparisons = Cell::new(0_u32);
        thread::scope(|scope| -> Result<(), Box<dyn Error>> {
            let (start, started) = sync_channel(1);
            let writer_state = &state;
            let writer = scope.spawn(move || -> Result<(), Box<dyn Error + Send + Sync>> {
                started.recv_timeout(WAIT)?;
                let mut value = writer_state
                    .write()
                    .map_err(|_| ReadPairError::Unavailable)?;
                *value = 11;
                Ok(())
            });
            let result = with_read_pair_acquire(
                &state,
                &state,
                |lock| {
                    acquisitions.set(acquisitions.get() + 1);
                    let guard = lock.try_read().map_err(|_| ReadPairError::Unavailable)?;
                    if acquisitions.get() == 1 {
                        start.try_send(()).map_err(|_| ReadPairError::Unavailable)?;
                        wait_for_queued_writer(lock)?;
                    }
                    Ok(guard)
                },
                |left, right| {
                    comparisons.set(comparisons.get() + 1);
                    assert!(std::ptr::eq(left, right));
                    (*left, *right)
                },
            );
            // Disconnect an unconsumed start request before joining on failure.
            drop(start);
            let writer_result = writer
                .join()
                .map_err(|_| std::io::Error::other("The range-state writer panicked."))?;
            writer_result.map_err(|error| -> Box<dyn Error> { error })?;
            assert_eq!(acquisitions.get(), 1);
            assert_eq!(comparisons.get(), 1);
            assert_eq!(result?, (7, 7));
            assert_eq!(*read_state(&state)?, 11);
            Ok(())
        })
    }

    /// Either acquisition failure skips comparison and releases every prior guard.
    ///
    /// # Panics
    /// Fails if a rejected pair invokes the consumer or retains the first lock.
    #[test]
    fn acquisition_failure_skips_comparison_and_releases_prior_guard() {
        let left = RwLock::new(7_u32);
        let right = RwLock::new(11_u32);
        let comparisons = Cell::new(0_u32);
        for fail_at in [1, 2] {
            let acquisitions = Cell::new(0_u32);
            let result = with_read_pair_acquire(
                &left,
                &right,
                |lock| {
                    acquisitions.set(acquisitions.get() + 1);
                    if acquisitions.get() == fail_at {
                        Err(ReadPairError::Unavailable)
                    } else {
                        read_state(lock)
                    }
                },
                |a, b| {
                    comparisons.set(comparisons.get() + 1);
                    a == b
                },
            );
            assert_eq!(result, Err(ReadPairError::Unavailable));
            assert_eq!(acquisitions.get(), fail_at);
            assert_eq!(comparisons.get(), 0);
            assert!(left.try_write().is_ok());
            assert!(right.try_write().is_ok());
        }
    }
}

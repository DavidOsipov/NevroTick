// SPDX-License-Identifier: GPL-3.0-only
// Original NevroTick text-range state hardening additions.
// Existing AccessKit/Chromium grants and notices remain unchanged in other files.

//! Borrow one coherent pair of text-range states without recursively read-locking.
//!
//! Identical lock references supply the same single guard to both operands.
//! Distinct locks remain read-locked together through the receiving comparison;
//! their values are not replaced with independently timed cloned snapshots.
//! Their live owner addresses select one acquisition order regardless of the
//! caller's operand order. The comparison retains the caller's original operands.
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
/// Distinct owners are acquired in increasing live-address order, retaining
/// simultaneous read protection and the comparison's original operand order.
/// The addresses are only a lock order for these borrowed owners, not persistent
/// or COM identities. Callers retain their COM/context/endpoint checks.
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

/// Share alias handling and canonical order with a real-lock acquisition observer.
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
    if std::ptr::eq(left, right) {
        let state = acquire(left)?;
        return Ok(compare(&state, &state));
    }

    let left_first = std::ptr::from_ref(left).addr() < std::ptr::from_ref(right).addr();
    let (first, second) = if left_first {
        (left, right)
    } else {
        (right, left)
    };
    let first_state = acquire(first)?;
    let second_state = acquire(second)?;
    if left_first {
        Ok(compare(&first_state, &second_state))
    } else {
        Ok(compare(&second_state, &first_state))
    }
}

#[cfg(test)]
mod tests {
    //! Real shared locks exercise the same helper used by the Windows receiver.

    use std::{
        cell::{Cell, RefCell},
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

    /// Reversing logical operands cannot reverse the physical acquisition order.
    ///
    /// # Errors
    /// Propagates an unexpected real-lock acquisition failure.
    ///
    /// # Panics
    /// Fails if opposite comparisons choose different first owners, reorder the
    /// comparison's operands or release either owner before comparison finishes.
    #[test]
    fn reversed_distinct_arguments_preserve_acquisition_and_operand_order()
    -> Result<(), ReadPairError> {
        let left = RwLock::new(7_u32);
        let right = RwLock::new(11_u32);
        let forward = RefCell::new(Vec::new());
        let reverse = RefCell::new(Vec::new());

        for (a, b, acquisitions, expected) in [
            (&left, &right, &forward, (7, 11)),
            (&right, &left, &reverse, (11, 7)),
        ] {
            let actual = with_read_pair_acquire(
                a,
                b,
                |lock| {
                    acquisitions.borrow_mut().push(std::ptr::from_ref(lock));
                    read_state(lock)
                },
                |a, b| {
                    assert!(matches!(left.try_write(), Err(TryLockError::WouldBlock)));
                    assert!(matches!(right.try_write(), Err(TryLockError::WouldBlock)));
                    (*a, *b)
                },
            )?;
            assert_eq!(actual, expected);
        }

        assert_eq!(*forward.borrow(), *reverse.borrow());
        assert_eq!(forward.borrow().len(), 2);
        assert_ne!(forward.borrow().first(), forward.borrow().last());
        assert!(left.try_write().is_ok());
        assert!(right.try_write().is_ok());
        Ok(())
    }

    /// Opposing comparisons retain the same first owner while writers exclude reads.
    ///
    /// Both comparisons first retain actual read guards before either writer
    /// starts. Writers then exclude further readers on both owners: a writer is
    /// queued wherever a first guard is retained and may own the other lock.
    /// Second acquisitions use bounded `try_read` probes, so a broken ordering
    /// produces a failure observation rather than an indefinitely blocked test.
    /// Every guard is released and every scoped worker joined before assertions.
    ///
    /// # Errors
    /// Rejects scheduling timeouts, failed lock acquisition or a worker panic.
    ///
    /// # Panics
    /// Fails if opposing comparisons retain different first owners or invoke a
    /// comparison before both guards are available; also checks writer progress.
    #[test]
    fn queued_writers_do_not_reverse_opposing_first_guards() -> Result<(), Box<dyn Error>> {
        let left = RwLock::new(7_u32);
        let right = RwLock::new(11_u32);
        thread::scope(|scope| -> Result<(), Box<dyn Error>> {
            let (first_acquired, first_owners) = sync_channel(2);
            let comparers = [(&left, &right), (&right, &left)].map(|(a, b)| {
                let first_acquired = first_acquired.clone();
                let (resume, resumed) = sync_channel(1);
                let comparer = scope.spawn(move || {
                    let acquisitions = Cell::new(0_u32);
                    let comparisons = Cell::new(0_u32);
                    let result = with_read_pair_acquire(
                        a,
                        b,
                        |lock| {
                            acquisitions.set(acquisitions.get() + 1);
                            let guard = lock.try_read().map_err(|_| ReadPairError::Unavailable)?;
                            if acquisitions.get() == 1 {
                                first_acquired
                                    .try_send(std::ptr::from_ref(lock).addr())
                                    .map_err(|_| ReadPairError::Unavailable)?;
                                resumed
                                    .recv_timeout(WAIT)
                                    .map_err(|_| ReadPairError::Unavailable)?;
                            }
                            Ok(guard)
                        },
                        |a, b| {
                            comparisons.set(comparisons.get() + 1);
                            a == b
                        },
                    );
                    (acquisitions.get(), comparisons.get(), result)
                });
                (resume, comparer)
            });
            drop(first_acquired);
            let first = first_owners.recv_timeout(WAIT)?;
            let second = first_owners.recv_timeout(WAIT)?;

            let writers = [&left, &right].map(|lock| {
                let (release, released) = sync_channel(1);
                let writer = scope.spawn(move || -> Result<(), Box<dyn Error + Send + Sync>> {
                    let mut value = lock.write().map_err(|_| ReadPairError::Unavailable)?;
                    released.recv_timeout(WAIT)?;
                    *value += 1;
                    Ok(())
                });
                (release, writer)
            });
            wait_for_reader_exclusion(&left)?;
            wait_for_reader_exclusion(&right)?;
            for (resume, _) in &comparers {
                resume.try_send(())?;
            }
            let outcomes = comparers.map(|(resume, comparer)| {
                drop(resume);
                comparer.join()
            });
            for (release, writer) in writers {
                release.try_send(())?;
                writer
                    .join()
                    .map_err(|_| std::io::Error::other("The range-state writer panicked."))?
                    .map_err(|error| -> Box<dyn Error> { error })?;
            }

            assert_eq!(first, second);
            for outcome in outcomes {
                let (acquisitions, comparisons, result) = outcome
                    .map_err(|_| std::io::Error::other("The range-state comparison panicked."))?;
                assert_eq!(acquisitions, 2);
                assert_eq!(comparisons, 0);
                assert_eq!(result, Err(ReadPairError::Unavailable));
            }
            assert_eq!(*read_state(&left)?, 8);
            assert_eq!(*read_state(&right)?, 12);
            Ok(())
        })
    }

    /// Wait until a waiting or active writer excludes another reader.
    ///
    /// When the caller retains a read guard, `WouldBlock` establishes exclusion
    /// by a waiting writer because the writer cannot yet own the lock.
    ///
    /// # Errors
    /// Scheduling deadline or poisoned state rejects the controlled interleaving.
    fn wait_for_reader_exclusion(state: &RwLock<u32>) -> Result<(), ReadPairError> {
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
                        wait_for_reader_exclusion(lock)?;
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

    /// Either acquisition failure in either operand order releases prior guards.
    ///
    /// # Panics
    /// Fails if a rejected pair invokes the consumer or retains the first lock.
    #[test]
    fn acquisition_failure_skips_comparison_and_releases_prior_guard() {
        let left = RwLock::new(7_u32);
        let right = RwLock::new(11_u32);
        let comparisons = Cell::new(0_u32);
        for (a, b) in [(&left, &right), (&right, &left)] {
            for fail_at in [1, 2] {
                let acquisitions = Cell::new(0_u32);
                let result = with_read_pair_acquire(
                    a,
                    b,
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

    /// Poison rejection releases a successfully acquired neighboring owner.
    ///
    /// # Errors
    /// Propagates an unexpected healthy acquisition failure before poisoning.
    ///
    /// # Panics
    /// Fails if a poisoned first or second owner reaches comparison, retains the
    /// other lock or exposes a state-bearing poison value through the error.
    #[test]
    #[expect(
        clippy::panic,
        reason = "A caught writer panic is required to verify poisoned-owner rejection."
    )]
    fn poisoned_owner_skips_comparison_and_releases_other_owner() -> Result<(), ReadPairError> {
        for poison_first in [true, false] {
            let left = RwLock::new(7_u32);
            let right = RwLock::new(11_u32);
            let first_owner = Cell::new(None);
            with_read_pair_acquire(
                &left,
                &right,
                |lock| {
                    if first_owner.get().is_none() {
                        first_owner.set(Some(lock));
                    }
                    read_state(lock)
                },
                |_, _| (),
            )?;
            let first = first_owner.get().ok_or(ReadPairError::Unavailable)?;
            let second = if std::ptr::eq(std::ptr::from_ref(first), std::ptr::from_ref(&left)) {
                &right
            } else {
                &left
            };
            let (poisoned, healthy) = if poison_first {
                (first, second)
            } else {
                (second, first)
            };
            let panic = std::panic::catch_unwind(|| -> Result<(), ReadPairError> {
                let _guard = poisoned.write().map_err(|_| ReadPairError::Unavailable)?;
                panic!("Poison the controlled range-state owner.");
            });
            assert!(panic.is_err());

            let comparisons = Cell::new(0_u32);
            let result = with_read_pair(&left, &right, |a, b| {
                comparisons.set(comparisons.get() + 1);
                a == b
            });
            assert_eq!(result, Err(ReadPairError::Unavailable));
            assert_eq!(comparisons.get(), 0);
            assert!(healthy.try_write().is_ok());
            assert!(matches!(
                poisoned.try_write(),
                Err(TryLockError::Poisoned(_))
            ));
        }
        Ok(())
    }

    /// Comparison unwinding releases aliased and distinct guards in both orders.
    ///
    /// # Panics
    /// Fails if comparison does not unwind or leaves any owner locked or poisoned.
    #[test]
    #[expect(
        clippy::panic,
        reason = "A caught callback panic is required to verify read-guard unwind cleanup."
    )]
    fn comparison_unwind_releases_all_guards() {
        let left = RwLock::new(7_u32);
        let right = RwLock::new(11_u32);
        for (a, b) in [(&left, &left), (&left, &right), (&right, &left)] {
            let panic = std::panic::catch_unwind(|| {
                with_read_pair::<_, ()>(a, b, |_, _| {
                    panic!("Unwind the controlled range-state comparison.");
                })
            });
            assert!(panic.is_err());
            assert!(left.try_write().is_ok());
            assert!(right.try_write().is_ok());
        }
    }
}

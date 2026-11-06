// SPDX-License-Identifier: GPL-3.0-only
//! Pure custody of UI wake scheduling and native stop acknowledgements.
//!
//! This gate owns no navigation, dialog, service or clock authority. The sole
//! shell controller still owns user consent; these facts only decide whether
//! the host may complete that already-authorized UI exit.
#![forbid(unsafe_code)]

use std::sync::Mutex;

/// One shared post-only wake; different native producers cannot grow a queue.
#[derive(Default)]
pub(super) struct WakeGate {
    /// A callback was enqueued but has not begun draining the current owner.
    queued: Mutex<bool>,
}
impl WakeGate {
    /// Coalesce a repeated request; failed posting remains retryable and false.
    pub(super) fn request(&self, post: impl FnOnce() -> bool) -> bool {
        let Ok(mut queued) = self.queued.lock() else {
            return false;
        };
        if *queued {
            return true;
        }
        // The public post port never runs its callback inline. This dedicated
        // scheduling lock holds no Root/native state and serializes the actual
        // enqueue result, so a concurrent producer cannot inherit a failed post.
        *queued = post();
        *queued
    }

    /// Clear at callback start, before any producer's mailbox is drained.
    pub(super) fn begin_drain(&self) {
        if let Ok(mut queued) = self.queued.lock() {
            *queued = false;
        }
    }
}

/// Bounded accepted preference edges, retained even between native timer polls.
#[derive(Default)]
pub(super) struct TrayIntent {
    /// Every accepted disable is reconciled before a later observer may start.
    disable: bool,
    /// A deliberate off/on edge authorizes retry of an unavailable observer.
    retry: bool,
}
impl TrayIntent {
    /// Record only an accepted change; presentation callbacks cannot set fields.
    pub(super) fn accept(&mut self, previous: bool, next: bool) {
        self.disable |= previous && !next;
        self.retry |= !previous && next;
    }

    /// Whether an earlier accepted disable still needs native reconciliation.
    pub(super) const fn disable_pending(&self) -> bool {
        self.disable
    }

    /// Clear only after no old port exists or its stop was actually requested.
    pub(super) fn acknowledge_disable(&mut self) {
        self.disable = false;
    }

    /// Consume one deliberate retry when no old native ownership remains.
    pub(super) fn take_retry(&mut self) -> bool {
        std::mem::take(&mut self.retry)
    }
}

/// Why the ordinary UI owner is waiting for its existing native ports to stop.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum StopPurpose {
    /// Disable only the tray; the ordinary window must remain available.
    DisableTray,
    /// The sole shell controller already obtained consent for UI-only exit.
    ExitUi,
}

/// Honest lifecycle result, independent of saved preferences or elapsed time.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum StopProgress {
    /// Native completion remains pending; the window/event loop must survive.
    Pending,
    /// The local deadline passed; this supplies no completion acknowledgement.
    TimedOut,
    /// An owned cleanup result is uncertain/failed; exit must be reconciled.
    Failed,
    /// Every required native port positively acknowledged terminal cleanup.
    Complete,
}

/// Independent native cleanup facts, without inferring completion from a timeout.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct StopFacts {
    /// The exact tray worker acknowledged terminal cleanup.
    pub(super) tray_cleaned: bool,
    /// The exact notification owner acknowledged terminal cleanup.
    pub(super) notifications_cleaned: bool,
    /// A retained cleanup obligation is failed or uncertain.
    pub(super) cleanup_failed: bool,
}

/// One stop intent; a later explicit exit may promote an earlier tray disable.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct StopBarrier {
    /// Existing accepted UI-only intention; never a second quit dialog.
    purpose: StopPurpose,
}
impl StopBarrier {
    /// Begin custody after the real owner has accepted its corresponding intent.
    pub(super) const fn new(purpose: StopPurpose) -> Self {
        Self { purpose }
    }

    /// Promote only to the stronger existing UI-exit intention.
    pub(super) const fn promote(&mut self, purpose: StopPurpose) {
        if matches!(purpose, StopPurpose::ExitUi) {
            self.purpose = StopPurpose::ExitUi;
        }
    }

    /// Read the current stop purpose without submitting an operation.
    pub(super) const fn purpose(self) -> StopPurpose {
        self.purpose
    }

    /// Late cleanup cannot revive consent that the sole shell already released.
    pub(super) const fn permits_ui_exit(
        self,
        progress: StopProgress,
        current_consent: bool,
    ) -> bool {
        matches!(self.purpose, StopPurpose::ExitUi)
            && matches!(progress, StopProgress::Complete)
            && current_consent
    }

    /// A timeout, missing port response, or a successful peer never permits exit.
    pub(super) const fn observe(self, facts: StopFacts, deadline_elapsed: bool) -> StopProgress {
        if facts.cleanup_failed {
            StopProgress::Failed
        } else if facts.tray_cleaned
            && (facts.notifications_cleaned || matches!(self.purpose, StopPurpose::DisableTray))
        {
            StopProgress::Complete
        } else if deadline_elapsed {
            StopProgress::TimedOut
        } else {
            StopProgress::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    //! These seams are used by the actual UI-thread owner, without Windows calls.

    use std::cell::Cell;

    use super::{StopBarrier, StopFacts, StopProgress, StopPurpose, TrayIntent, WakeGate};

    /// A completed peer, failed cleanup, or elapsed deadline cannot close the UI.
    ///
    /// # Panics
    /// Fails if a missing native acknowledgement or failed wake is treated as success.
    #[test]
    fn ui_exit_requires_both_actual_cleanup_acknowledgements() {
        let stop = StopBarrier::new(StopPurpose::ExitUi);
        assert_eq!(
            stop.observe(
                StopFacts {
                    tray_cleaned: false,
                    notifications_cleaned: true,
                    cleanup_failed: false
                },
                false
            ),
            StopProgress::Pending
        );
        assert_eq!(
            stop.observe(
                StopFacts {
                    tray_cleaned: true,
                    notifications_cleaned: false,
                    cleanup_failed: false
                },
                false
            ),
            StopProgress::Pending
        );
        assert_eq!(
            stop.observe(
                StopFacts {
                    tray_cleaned: false,
                    notifications_cleaned: false,
                    cleanup_failed: false
                },
                true
            ),
            StopProgress::TimedOut
        );
        assert_eq!(
            stop.observe(
                StopFacts {
                    tray_cleaned: true,
                    notifications_cleaned: true,
                    cleanup_failed: true
                },
                true
            ),
            StopProgress::Failed
        );
        assert_eq!(
            stop.observe(
                StopFacts {
                    tray_cleaned: true,
                    notifications_cleaned: true,
                    cleanup_failed: false
                },
                true
            ),
            StopProgress::Complete
        );
    }

    /// Tray disable leaves notifications/UI running; later quit still waits for both.
    ///
    /// # Panics
    /// Fails if a missing native acknowledgement or failed wake is treated as success.
    #[test]
    fn disable_can_only_promote_to_the_existing_explicit_exit() {
        let mut stop = StopBarrier::new(StopPurpose::DisableTray);
        assert_eq!(
            stop.observe(
                StopFacts {
                    tray_cleaned: true,
                    notifications_cleaned: false,
                    cleanup_failed: false
                },
                false
            ),
            StopProgress::Complete
        );
        stop.promote(StopPurpose::ExitUi);
        stop.promote(StopPurpose::DisableTray);
        assert_eq!(stop.purpose(), StopPurpose::ExitUi);
        assert_eq!(
            stop.observe(
                StopFacts {
                    tray_cleaned: true,
                    notifications_cleaned: false,
                    cleanup_failed: false
                },
                false
            ),
            StopProgress::Pending
        );
    }

    /// A late completion after timeout/failure cannot close newly resumed UI work.
    ///
    /// # Panics
    /// Fails if old consent is revived, pending work exits, or new consent is ignored.
    #[test]
    fn late_cleanup_needs_current_explicit_ui_exit_consent() {
        let stop = StopBarrier::new(StopPurpose::ExitUi);
        assert!(!stop.permits_ui_exit(StopProgress::Complete, false));
        assert!(!stop.permits_ui_exit(StopProgress::Pending, true));
        assert!(stop.permits_ui_exit(StopProgress::Complete, true));
        assert!(
            !StopBarrier::new(StopPurpose::DisableTray)
                .permits_ui_exit(StopProgress::Complete, true)
        );
    }

    /// Rapid accepted off/on edges survive polling without spawning parallel owners.
    ///
    /// # Panics
    /// Fails if the disable/retry edge disappears or retry is consumed twice.
    #[test]
    fn accepted_preference_edges_survive_an_off_on_pair_between_polls() {
        let mut intent = TrayIntent::default();
        intent.accept(true, false);
        intent.accept(false, true);
        assert!(intent.disable_pending());
        intent.acknowledge_disable();
        assert!(!intent.disable_pending());
        assert!(intent.take_retry());
        assert!(!intent.take_retry());
    }

    /// Shared producers coalesce; clearing before draining permits a later wake.
    ///
    /// # Panics
    /// Fails if a missing native acknowledgement or failed wake is treated as success.
    #[test]
    fn one_shared_wake_is_bounded_without_losing_a_post_drain_request() {
        let gate = WakeGate::default();
        let posts = Cell::new(0);
        for _ in 0..1000 {
            assert!(gate.request(|| {
                posts.set(posts.get() + 1);
                true
            }));
        }
        assert_eq!(posts.get(), 1);
        gate.begin_drain();
        assert!(gate.request(|| {
            posts.set(posts.get() + 1);
            true
        }));
        assert_eq!(posts.get(), 2);
    }

    /// Failed enqueue is reported and cannot suppress the next actual producer.
    ///
    /// # Panics
    /// Fails if a missing native acknowledgement or failed wake is treated as success.
    #[test]
    fn failed_shared_post_is_not_an_acknowledged_wake() {
        let gate = WakeGate::default();
        assert!(!gate.request(|| false));
        let posts = Cell::new(0);
        assert!(gate.request(|| {
            posts.set(posts.get() + 1);
            true
        }));
        assert_eq!(posts.get(), 1);
    }
}

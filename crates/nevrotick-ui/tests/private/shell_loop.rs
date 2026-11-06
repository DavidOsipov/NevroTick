//! Bounded public event-loop proxy for source/lifecycle tests, without a real loop.
//!
//! Posting queues a callback; only the owning test's explicit drain executes it.
//! Quit records a request or an injected error, never native-loop termination.
#![forbid(unsafe_code)]

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

use slint::EventLoopError;
use slint::platform::EventLoopProxy;

/// Maximum retained callback count; captures are owned by their posting caller.
const MAX_PENDING_JOBS: usize = 64;
/// Maximum callbacks executed by one drain, including callbacks posted by jobs.
const MAX_DRAINED_JOBS: usize = 128;

/// Shared bounded queue and independent quit-request observations.
#[derive(Default)]
pub(crate) struct LoopControls {
    /// Count attempted quit requests, including deliberately rejected requests.
    pub(crate) quit_requests: AtomicU32,
    /// Reject quit requests while true; no termination is manufactured.
    pub(crate) quit_fails: AtomicBool,
    /// Owned pending callbacks; neither posting nor queue inspection executes them.
    jobs: Mutex<VecDeque<Box<dyn FnOnce() + Send>>>,
}

impl LoopControls {
    /// Execute at most 128 queued callbacks, releasing the queue lock before each.
    ///
    /// Callback panics propagate as test failures; they cannot poison this queue
    /// because the callback runs after the guard has been dropped. The count bound
    /// does not claim a duration bound for arbitrary caller-owned callback code.
    ///
    /// # Errors
    /// Returns the public termination error for a poisoned queue or remaining work
    /// after the finite drain budget. Unexecuted callbacks remain queued.
    pub(crate) fn drain(&self) -> Result<usize, EventLoopError> {
        for completed in 0..MAX_DRAINED_JOBS {
            let next = {
                let mut jobs = self
                    .jobs
                    .lock()
                    .map_err(|_| EventLoopError::EventLoopTerminated)?;
                jobs.pop_front()
            };
            let Some(job) = next else {
                return Ok(completed);
            };
            job();
        }
        let empty = self
            .jobs
            .lock()
            .map_err(|_| EventLoopError::EventLoopTerminated)?
            .is_empty();
        if empty {
            Ok(MAX_DRAINED_JOBS)
        } else {
            Err(EventLoopError::EventLoopTerminated)
        }
    }
}

/// Thread-safe recording proxy implementing only Slint's public trait.
pub(crate) struct RecordingProxy {
    /// Shared observations/queue; no UI handle, native proxy or component cycle.
    controls: Arc<LoopControls>,
}

impl RecordingProxy {
    /// Retain one shared control owner without creating a loop or invoking work.
    #[must_use]
    pub(crate) fn new(controls: Arc<LoopControls>) -> Self {
        Self { controls }
    }
}

impl EventLoopProxy for RecordingProxy {
    /// Record one checked attempt, then honor the independently armed failure.
    ///
    /// # Errors
    /// Rejects an exhausted attempt counter or an injected quit failure. Neither
    /// path wraps observations or claims native-loop completion.
    fn quit_event_loop(&self) -> Result<(), EventLoopError> {
        self.controls
            .quit_requests
            .try_update(Ordering::SeqCst, Ordering::SeqCst, |current| {
                current.checked_add(1)
            })
            .map_err(|_| EventLoopError::EventLoopTerminated)?;
        if self.controls.quit_fails.load(Ordering::SeqCst) {
            Err(EventLoopError::EventLoopTerminated)
        } else {
            Ok(())
        }
    }

    /// Admit a callback into the bounded queue without executing it inline.
    ///
    /// # Errors
    /// Rejects poison, capacity exhaustion or allocation failure. Slint's public
    /// error type has no custom queue-error variant, so these remain explicit
    /// termination errors rather than successful posts.
    fn invoke_from_event_loop(
        &self,
        event: Box<dyn FnOnce() + Send>,
    ) -> Result<(), EventLoopError> {
        let mut jobs = self
            .controls
            .jobs
            .lock()
            .map_err(|_| EventLoopError::EventLoopTerminated)?;
        if jobs.len() >= MAX_PENDING_JOBS || jobs.try_reserve(1).is_err() {
            // Drop before rejecting the caller-owned closure and its captures.
            drop(jobs);
            return Err(EventLoopError::EventLoopTerminated);
        }
        jobs.push_back(event);
        Ok(())
    }
}

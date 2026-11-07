// SPDX-License-Identifier: GPL-2.0-only
//! Serialized lifecycle and acknowledged status for one service invocation.
//!
//! A future narrow SCM adapter owns the reporter and calls this owner after
//! identity, policy and readiness checks. The owner never restarts: a successor
//! needs a new owner and must independently reconcile durable clock authority.
//! `Running` describes availability only. Callers supply actual monotonic times,
//! genuine progress and completed native operations; host tests cannot prove them.
//!
//! Reporting failures permanently close work admission but permit cleanup.
//! Cancellation, elapsed deadlines and dropped permits never complete work.
//! Storage is fixed apart from one shared counter and one handle per admitted job;
//! the explicit cap bounds outstanding handles. There are no timers or I/O here.

use std::{
    error::Error,
    fmt,
    num::NonZeroU32,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
    time::{Duration, Instant},
};

use garde::Validate;

/// The two separate process roles; neither variant confers operating-system rights.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServiceRole {
    /// Network-facing acquisition without clock privilege.
    Engine,
    /// Narrow clock broker with independent recovery obligations.
    ClockBroker,
}

/// Service states that can be reported; no pause or shutdown handler is enabled.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServiceState {
    /// Initialization is incomplete and accepts no SCM controls.
    StartPending,
    /// Declared readiness was reached and Stop is accepted.
    Running,
    /// Work admission is closed and actual cleanup is pending.
    StopPending,
    /// The sole terminal report has been attempted; never reuse its context.
    Stopped,
}

/// Explicit laboratory or qualified per-role pending-phase limits; no defaults.
#[derive(Clone, Copy, Debug, Validate)]
pub struct PendingBudget {
    /// Nonzero representable SCM wait hint in milliseconds, bounded by the phase.
    #[garde(range(min = 1), custom(hint_within_timeout(&self.timeout_ms)))]
    wait_hint_ms: u32,
    /// Nonzero absolute phase allowance in milliseconds, never renewed by progress.
    #[garde(range(min = 1))]
    timeout_ms: u32,
}

/// Reject zero or contradictory pending-phase limits without retaining raw input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BudgetError;

impl fmt::Display for BudgetError {
    /// Render a fixed safe explanation without caller-supplied values.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid service pending budget")
    }
}

impl Error for BudgetError {}

impl PendingBudget {
    /// Admit nonzero millisecond limits with `wait_hint_ms <= timeout_ms`.
    ///
    /// # Errors
    /// Returns `BudgetError` for either zero or a hint exceeding the deadline.
    pub fn new(wait_hint_ms: u32, timeout_ms: u32) -> Result<Self, BudgetError> {
        let budget = Self {
            wait_hint_ms,
            timeout_ms,
        };
        budget.validate().map_err(|_| BudgetError)?;
        Ok(budget)
    }
}

/// Enforce the relationship inside mandatory Garde construction, without effects.
fn hint_within_timeout(timeout: &u32) -> impl FnOnce(&u32, &()) -> garde::Result + '_ {
    move |hint, ()| {
        if hint <= timeout {
            Ok(())
        } else {
            Err(garde::Error::new("wait hint exceeds timeout"))
        }
    }
}

/// Terminal service outcome after actual jobs and required recovery handling finish.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StopOutcome {
    /// All required cleanup completed successfully.
    Clean,
    /// Startup, reporting or cleanup failed; required recovery handling is complete.
    /// A cleanup failure needs durable handoff to its recovery owner; code is nonzero.
    Failed(NonZeroU32),
}

/// Immutable status produced only by the lifecycle owner, never a raw SCM DTO.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ServiceStatus {
    /// State selected by the serialized owner.
    state: ServiceState,
    /// Nonzero only during a pending phase; increments on explicit real progress.
    checkpoint: u32,
    /// Millisecond hint, zero in stable states.
    wait_hint_ms: u32,
    /// Present only on the terminal report.
    outcome: Option<StopOutcome>,
}

impl ServiceStatus {
    /// Read the reported state without manufacturing readiness or authority.
    #[must_use]
    pub const fn state(self) -> ServiceState {
        self.state
    }

    /// Read pending progress; stable states always return zero.
    #[must_use]
    pub const fn checkpoint(self) -> u32 {
        self.checkpoint
    }

    /// Read the pending millisecond hint; stable states always return zero.
    #[must_use]
    pub const fn wait_hint_ms(self) -> u32 {
        self.wait_hint_ms
    }

    /// Stop is the only enabled control and only while Running.
    #[must_use]
    pub const fn accepts_stop(self) -> bool {
        matches!(self.state, ServiceState::Running)
    }

    /// Read the completed cleanup outcome, present only for Stopped.
    #[must_use]
    pub const fn outcome(self) -> Option<StopOutcome> {
        self.outcome
    }
}

/// Checked effect seam for the future SCM adapter; no automatic Running report.
pub trait StatusReporter {
    /// Preserve the actual typed status API failure, captured before other calls.
    type Error: Error;

    /// Attempt one report, returning success only on actual acknowledgement.
    ///
    /// # Errors
    /// Must return the underlying failure; success cannot mean merely queued.
    fn report(&mut self, status: ServiceStatus) -> Result<(), Self::Error>;
}

/// Bounded lifecycle refusals; reporting errors retain their original typed cause.
#[derive(Debug, Eq, PartialEq)]
pub enum LifecycleError<E> {
    /// The requested operation does not belong to the current phase.
    InvalidTransition,
    /// A prior failed status permanently fenced readiness and new work.
    ReportingFailed,
    /// A supplied monotonic observation moved backwards.
    TimeReversed,
    /// A fixed pending deadline elapsed; resources still require real cleanup.
    DeadlineElapsed,
    /// The host cannot represent the requested absolute deadline.
    DeadlineOverflow,
    /// Pending progress exhausted its representable checkpoint.
    CheckpointOverflow,
    /// Actual outstanding jobs prohibit readiness or terminal completion.
    OutstandingWork,
    /// The explicit outstanding-job cap was reached.
    WorkLimit,
    /// Earlier reporting or deadline failure requires a nonzero terminal failure.
    FailureOutcomeRequired,
    /// No acknowledgement was obtained for this attempted status.
    Report(E),
}

impl<E> fmt::Display for LifecycleError<E> {
    /// Format only the bounded refusal category, leaving causes for explicit use.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidTransition => "invalid service lifecycle transition",
            Self::ReportingFailed => "service status reporting failed",
            Self::TimeReversed => "service monotonic time reversed",
            Self::DeadlineElapsed => "service pending deadline elapsed",
            Self::DeadlineOverflow => "service pending deadline is unrepresentable",
            Self::CheckpointOverflow => "service checkpoint exhausted",
            Self::OutstandingWork => "service work is still outstanding",
            Self::WorkLimit => "service work capacity exhausted",
            Self::FailureOutcomeRequired => "service failure requires a failed exit",
            Self::Report(_) => "service status was not acknowledged",
        })
    }
}

impl<E: Error + 'static> Error for LifecycleError<E> {
    /// Expose the original reporter cause without formatting or logging it.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let Self::Report(error) = self {
            Some(error)
        } else {
            None
        }
    }
}

/// Completion custody for one actual job, never cloned or completed by Drop.
///
/// The native adapter must retain this permit until the real operation completes.
/// Losing it permanently charges that owner and prevents successful Stopped.
#[derive(Debug)]
#[must_use = "retain until actual work completes; dropping never releases capacity"]
pub struct WorkPermit {
    /// Shared counter survives either owner's or permit's early drop.
    outstanding: Arc<AtomicU32>,
}

impl WorkPermit {
    /// Release this job exactly once after its actual completion, never cancellation.
    ///
    /// This is an adapter assertion of physical completion, not an OS observation.
    pub fn complete(self) {
        self.outstanding.fetch_sub(1, Ordering::AcqRel);
    }
}

/// Single non-cloneable owner for one process invocation and its status reporter.
pub struct ServiceLifecycle<R: StatusReporter> {
    /// Fixed role; no arbitrary service name, executable or management command.
    role: ServiceRole,
    /// Sole status-effect owner, inaccessible to workers.
    reporter: R,
    /// Current intended state, distinct from the last acknowledged status.
    current: Option<ServiceStatus>,
    /// Last successfully acknowledged report; never advanced on reporter failure.
    acknowledged: Option<ServiceStatus>,
    /// Permanent fence after the first failed report.
    reporting_failed: bool,
    /// Prior reporting or deadline failure must survive otherwise successful cleanup.
    terminal_failure: bool,
    /// Explicit startup pending limits.
    startup: PendingBudget,
    /// Explicit shutdown pending limits.
    shutdown: PendingBudget,
    /// Fixed deadline for the current pending phase.
    deadline: Option<Instant>,
    /// Last accepted monotonic observation, for reversal detection.
    observed: Option<Instant>,
    /// Actual completion counter; cancellation and deadline do not decrement it.
    outstanding: Arc<AtomicU32>,
    /// Explicit maximum outstanding jobs, with no invented product default.
    max_work: NonZeroU32,
}

impl<R: StatusReporter> ServiceLifecycle<R> {
    /// Construct an inert owner; performs no report, work admission or OS call.
    #[must_use]
    pub fn new(
        role: ServiceRole,
        reporter: R,
        startup: PendingBudget,
        shutdown: PendingBudget,
        max_work: NonZeroU32,
    ) -> Self {
        Self {
            role,
            reporter,
            current: None,
            acknowledged: None,
            reporting_failed: false,
            terminal_failure: false,
            startup,
            shutdown,
            deadline: None,
            observed: None,
            outstanding: Arc::new(AtomicU32::new(0)),
            max_work,
        }
    }

    /// Read the fixed process role; this carries no privilege or authentication.
    #[must_use]
    pub const fn role(&self) -> ServiceRole {
        self.role
    }

    /// Read intended state, including a terminal attempt whose report failed.
    #[must_use]
    pub fn state(&self) -> Option<ServiceState> {
        self.current.map(ServiceStatus::state)
    }

    /// Read only actual successful status acknowledgement, or None before any.
    #[must_use]
    pub const fn acknowledged(&self) -> Option<ServiceStatus> {
        self.acknowledged
    }

    /// Count work still charged until explicit actual completion.
    #[must_use]
    pub fn outstanding_work(&self) -> u32 {
        self.outstanding.load(Ordering::Acquire)
    }

    /// Attempt initial `StartPending` with no accepted controls and checkpoint one.
    ///
    /// # Errors
    /// Reject reuse/overflow before reporting; a report failure fences new work.
    pub fn start(&mut self, now: Instant) -> Result<(), LifecycleError<R::Error>> {
        if self.current.is_some() {
            return Err(LifecycleError::InvalidTransition);
        }
        self.begin_pending(ServiceState::StartPending, self.startup, now)
    }

    /// Report Running only after caller-owned readiness and completed initialization.
    ///
    /// # Errors
    /// Reject wrong phase, failed reporting, reversal, deadline or outstanding jobs.
    /// Reporting failure never acknowledges Running and permanently fences work.
    pub fn ready(&mut self, now: Instant) -> Result<(), LifecycleError<R::Error>> {
        if self.state() != Some(ServiceState::StartPending) {
            return Err(LifecycleError::InvalidTransition);
        }
        if self.reporting_failed {
            return Err(LifecycleError::ReportingFailed);
        }
        self.check_pending(now)?;
        if self.outstanding_work() != 0 {
            return Err(LifecycleError::OutstandingWork);
        }
        self.publish(ServiceStatus {
            state: ServiceState::Running,
            checkpoint: 0,
            wait_hint_ms: 0,
            outcome: None,
        })
    }

    /// Admit a bounded actual job only during healthy startup or Running.
    ///
    /// # Errors
    /// Reject inactive/stopping/terminal state, failed reporting, stale time or cap.
    pub fn begin_work(&mut self, now: Instant) -> Result<WorkPermit, LifecycleError<R::Error>> {
        if !matches!(
            self.state(),
            Some(ServiceState::StartPending | ServiceState::Running)
        ) {
            return Err(LifecycleError::InvalidTransition);
        }
        if self.reporting_failed {
            return Err(LifecycleError::ReportingFailed);
        }
        if self.state() == Some(ServiceState::StartPending) {
            self.check_pending(now)?;
        } else {
            self.observe(now)?;
        }
        self.outstanding
            .try_update(Ordering::AcqRel, Ordering::Acquire, |count| {
                if count < self.max_work.get() {
                    count.checked_add(1)
                } else {
                    None
                }
            })
            .map_err(|_| LifecycleError::WorkLimit)?;
        Ok(WorkPermit {
            outstanding: Arc::clone(&self.outstanding),
        })
    }

    /// Close work admission and attempt `StopPending` once, retaining all job charges.
    /// Duplicate internal stop requests return false without renewing the deadline.
    ///
    /// # Errors
    /// Reject unused/terminal owners, time reversal or unrepresentable deadline.
    /// Failed reporting leaves cleanup active and admission closed.
    pub fn request_stop(&mut self, now: Instant) -> Result<bool, LifecycleError<R::Error>> {
        match self.state() {
            Some(ServiceState::StopPending) => {
                self.observe(now)?;
                Ok(false)
            }
            Some(ServiceState::StartPending | ServiceState::Running) => {
                self.begin_pending(ServiceState::StopPending, self.shutdown, now)?;
                Ok(true)
            }
            None | Some(ServiceState::Stopped) => Err(LifecycleError::InvalidTransition),
        }
    }

    /// Report genuine pending progress without extending the fixed phase deadline.
    ///
    /// # Errors
    /// Reject stable phases, backwards/expired time or checkpoint exhaustion.
    /// Reporter failure remains explicit and never prevents independent cleanup.
    pub fn progress(&mut self, now: Instant) -> Result<(), LifecycleError<R::Error>> {
        if !matches!(
            self.state(),
            Some(ServiceState::StartPending | ServiceState::StopPending)
        ) {
            return Err(LifecycleError::InvalidTransition);
        }
        self.check_pending(now)?;
        let Some(mut status) = self.current else {
            return Err(LifecycleError::InvalidTransition);
        };
        status.checkpoint = status
            .checkpoint
            .checked_add(1)
            .ok_or(LifecycleError::CheckpointOverflow)?;
        self.publish(status)
    }

    /// Observe a stuck pending phase without fabricating progress or reporting.
    /// Deadline expiry retains every actual work charge for the recovery owner.
    ///
    /// # Errors
    /// Reject stable phases, backwards time or the exact/elapsed fixed deadline.
    pub fn check_deadline(&mut self, now: Instant) -> Result<(), LifecycleError<R::Error>> {
        if !matches!(
            self.state(),
            Some(ServiceState::StartPending | ServiceState::StopPending)
        ) {
            return Err(LifecycleError::InvalidTransition);
        }
        self.check_pending(now)
    }

    /// Attempt the sole Stopped report after actual jobs and required recovery finish.
    /// Cleanup may finish after its deadline; the elapsed deadline releases nothing.
    ///
    /// # Errors
    /// Reject wrong phase, time reversal, outstanding work or a clean outcome
    /// hiding a previous reporting or deadline failure. The final report is
    /// never retried, even on failure, because its native context may be released.
    pub fn finish_stop(
        &mut self,
        now: Instant,
        outcome: StopOutcome,
    ) -> Result<(), LifecycleError<R::Error>> {
        if self.state() != Some(ServiceState::StopPending) {
            return Err(LifecycleError::InvalidTransition);
        }
        self.observe(now)?;
        if self.outstanding_work() != 0 {
            return Err(LifecycleError::OutstandingWork);
        }
        if self.terminal_failure && outcome == StopOutcome::Clean {
            return Err(LifecycleError::FailureOutcomeRequired);
        }
        self.publish(ServiceStatus {
            state: ServiceState::Stopped,
            checkpoint: 0,
            wait_hint_ms: 0,
            outcome: Some(outcome),
        })
    }

    /// Start one pending phase after checked deadline calculation; errors are explicit.
    ///
    /// # Errors
    /// Reject an unrepresentable deadline, reversed time or failed status report.
    fn begin_pending(
        &mut self,
        state: ServiceState,
        budget: PendingBudget,
        now: Instant,
    ) -> Result<(), LifecycleError<R::Error>> {
        let deadline = now
            .checked_add(Duration::from_millis(u64::from(budget.timeout_ms)))
            .ok_or(LifecycleError::DeadlineOverflow)?;
        self.observe(now)?;
        self.deadline = Some(deadline);
        self.publish(ServiceStatus {
            state,
            checkpoint: 1,
            wait_hint_ms: budget.wait_hint_ms,
            outcome: None,
        })
    }

    /// Check monotonic observations and latch any current pending deadline failure.
    /// This never acquires time or performs a status/native effect.
    ///
    /// # Errors
    /// Reject time earlier than the last observation, leaving it unchanged.
    fn observe(&mut self, now: Instant) -> Result<(), LifecycleError<R::Error>> {
        if self.observed.is_some_and(|previous| now < previous) {
            return Err(LifecycleError::TimeReversed);
        }
        self.observed = Some(now);
        if matches!(
            self.state(),
            Some(ServiceState::StartPending | ServiceState::StopPending)
        ) && self.deadline.is_some_and(|deadline| now >= deadline)
        {
            self.terminal_failure = true;
        }
        Ok(())
    }

    /// Enforce the fixed deadline, including equality; never free work on timeout.
    ///
    /// # Errors
    /// Reject backwards time or an elapsed pending deadline.
    fn check_pending(&mut self, now: Instant) -> Result<(), LifecycleError<R::Error>> {
        self.observe(now)?;
        if self.deadline.is_some_and(|deadline| now >= deadline) {
            return Err(LifecycleError::DeadlineElapsed);
        }
        Ok(())
    }

    /// Attempt a status once and preserve the last actual acknowledgement on error.
    ///
    /// # Errors
    /// Return the reporter's original cause and permanently fence work on failure.
    fn publish(&mut self, status: ServiceStatus) -> Result<(), LifecycleError<R::Error>> {
        self.current = Some(status);
        match self.reporter.report(status) {
            Ok(()) => {
                self.acknowledged = Some(status);
                Ok(())
            }
            Err(error) => {
                self.reporting_failed = true;
                self.terminal_failure = true;
                Err(LifecycleError::Report(error))
            }
        }
    }
}

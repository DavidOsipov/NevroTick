// SPDX-License-Identifier: GPL-2.0-only
//! One invocation's real admission and receiving owner; public execution is one-shot.
//!
//! The private component experiment may replace one exhausted established context
//! before its first protected success. Public execution remains one-shot. Counters
//! bound transaction observations, not TLS traffic, native work or rendered bytes.

use std::io;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[cfg(test)]
use garde::Validate;

use super::association::AssociationOwner;
use super::association::ledger::RecoveryIntent;
use super::association::ledger::retirement::{PastRequests, RetiredContext};
use super::datagram::{CommittedProtected, ProtectedCompletion};
use super::process::ProviderScope;
use super::{
    CancelOutcome, Config, Control, EXCHANGE_TIME, ExchangeMode, Failed, Failure,
    InvocationCompletion, StopHandle, StopScope,
};

/// Fixed private experiment ceilings; none are shipped retry defaults.
#[cfg(test)]
const TRANSACTIONS: u8 = 4;
/// Refusals count requests denied before entering the KE producer.
#[cfg(test)]
const REFUSALS: u8 = 8;
/// Admission and eventual outcome reserve two observations per transaction.
#[cfg(test)]
const OBSERVATIONS: u8 = 16;
/// No third successfully established context is admitted by the private slice.
const CONTEXTS: u8 = 2;

/// Closed terminal reason, separate from the original operation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum End {
    /// One established association finished its sole selected mode.
    Completed,
    /// Invocation stop forbids new work without acknowledging actual completion.
    Stopped,
    /// The invocation scope gate is unavailable; no stopped state is inferred.
    Control,
    /// An operation or cleanup failed without an admitted recovery route.
    Failure,
    /// The immutable observation window cannot admit useful work.
    Window,
    /// All permitted KE producer invocations are charged.
    Transactions,
    /// Two successful contexts have already used the entire private allowance.
    Contexts,
    /// The final permitted scheduling refusal closed this invocation.
    Refusals,
    /// A complete admission/outcome reservation cannot fit.
    Observations,
    /// Checked deadline or delay arithmetic refused its representation.
    Arithmetic,
    /// A separately observed draw-source failure never replaces the KE primary.
    Jitter(Failure),
    /// Preserve a new retirement/attachment cause beside the original failure.
    Context(ContextStage, Failure),
}

/// Closed context-transfer stages; no payload or secret material is retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ContextStage {
    /// The original exhausted owner or its fresh receiving projection refused.
    Retire,
    /// A replacement could not accept its mandatory predecessor history.
    Attach,
}

/// An inert plan cannot charge work, construct a Control or publish a result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Plan {
    /// Fresh actual admission must still run before any effect.
    Due,
    /// No producer invocation is allowed before this absolute monotonic instant.
    Deferred(Instant),
    /// No subsequent step may reopen this invocation.
    Closed(End),
}

/// The only observable result of one shared, bounded driver step.
pub(super) enum Step {
    /// The caller must park rather than immediately resubmit.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "the private experiment parks at this boundary; public one-shot execution admits no native retry scheduler"
        )
    )]
    Deferred(Instant),
    /// Owned terminal-mode result, never fresh-synchronization authority.
    Complete(InvocationCompletion),
    /// A closed invocation creates no additional observation or producer call.
    Closed(End),
}

/// One actual atomic admission, or its bounded no-producer disposition.
enum Admission {
    /// Counters and original control custody were committed under the scope gate.
    Ready {
        /// Original or replacement Control with the same invocation scope and deadlines.
        control: Control,
        /// Prospective ordinal, advanced only after establishment actually succeeds.
        contexts: u8,
    },
    /// No new producer entry is authorized by this closed or deferred outcome.
    Finished(Step),
}

/// Inert direct scheduling observation after genuine component setup.
#[cfg(test)]
pub(super) type StepProbeResult = Result<Step, Failed>;

/// Inert calculated boundary; it grants no admission or state publication.
#[cfg(test)]
pub(super) type BoundaryProbeResult = Result<(Duration, Instant), End>;

/// Immutable checked experimental delay parameters, all expressed in durations.
#[derive(Clone, Copy)]
pub(super) struct Backoff {
    /// The first eligible failure's strictly positive base delay.
    minimum: Duration,
    /// Upper bound on the entire delay, including its optional draw.
    maximum: Duration,
    /// Integer factor; one means a constant positive delay.
    growth: u32,
    /// A draw can use only the remaining headroom below maximum.
    jitter_max: Duration,
}

impl Backoff {
    /// Admit explicit experiment parameters without selecting production defaults.
    /// # Errors
    /// Refuse zero/reversed bounds, zero growth and jitter exceeding headroom.
    #[cfg(test)]
    pub(super) fn new(
        minimum: Duration,
        maximum: Duration,
        growth: u32,
        jitter_max: Duration,
    ) -> Result<Self, Failure> {
        BackoffCandidate {
            values: (minimum, maximum, growth, jitter_max),
        }
        .validate()
        .map_err(|_| Failure::Limits)?;
        Ok(Self {
            minimum,
            maximum,
            growth,
            jitter_max,
        })
    }

    /// Compute one base without mutating current scheduling state.
    /// # Errors
    /// Refuse an unrepresentable intermediate even if an algebraic cap might fit.
    pub(super) fn base_after(&self, previous: Option<Duration>) -> Result<Duration, Failure> {
        match previous {
            None => Ok(self.minimum),
            Some(value) if value >= self.minimum && value <= self.maximum => value
                .checked_mul(self.growth)
                .map(|next| next.min(self.maximum))
                .ok_or(Failure::Deadline),
            Some(_) => Err(Failure::Limits),
        }
    }

    /// Bound a draw by both its policy ceiling and the actual remaining headroom.
    /// # Errors
    /// Refuse a base not produced within this checked policy.
    pub(super) fn draw_upper(&self, base: Duration) -> Result<Duration, Failure> {
        if base < self.minimum || base > self.maximum {
            return Err(Failure::Limits);
        }
        self.maximum
            .checked_sub(base)
            .map(|headroom| headroom.min(self.jitter_max))
            .ok_or(Failure::Deadline)
    }

    /// Check a single supplied draw; this method performs no entropy operation.
    /// # Errors
    /// Refuse excess jitter, an invalid base and addition overflow.
    pub(super) fn delay(&self, base: Duration, draw: Duration) -> Result<Duration, Failure> {
        if draw > self.draw_upper(base)? {
            return Err(Failure::Limits);
        }
        base.checked_add(draw).ok_or(Failure::Deadline)
    }
}

/// Fallible raw experiment construction shares mandatory Garde validation.
#[cfg(test)]
#[derive(Validate)]
struct BackoffCandidate {
    /// The custom rule owns all cross-field duration and growth constraints.
    #[garde(custom(backoff_rule))]
    values: (Duration, Duration, u32, Duration),
}

/// Validate the exact relation before any private policy value can escape.
/// # Errors
/// Return only a fixed redacted rejection for inconsistent bounds.
#[cfg(test)]
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde custom rules borrow fields and their unit context"
)]
fn backoff_rule(values: &(Duration, Duration, u32, Duration), (): &()) -> garde::Result {
    let (minimum, maximum, growth, jitter) = *values;
    if minimum.is_zero()
        || maximum < minimum
        || growth == 0
        || maximum
            .checked_sub(minimum)
            .is_none_or(|headroom| jitter > headroom)
    {
        return Err(garde::Error::new("invalid experimental delay bounds"));
    }
    Ok(())
}

/// Checked private limits; one-shot has no retry policy or jitter source.
#[derive(Clone, Copy)]
pub(super) struct Limits {
    /// The invocation's immutable admission/publication window.
    window: Duration,
    /// KE producer entries, including immediate establishment refusals.
    transactions: u8,
    /// No-producer scheduling refusals; the last one is terminal.
    refusals: u8,
    /// Transaction-level observations, never encoded-byte or CPU claims.
    observations: u8,
    /// Only the private experiment may possess an explicit retry policy.
    backoff: Option<Backoff>,
    /// Explicit shorter experiment context; absent for public one-shot execution.
    context_lifetime: Option<Duration>,
}

impl Limits {
    /// Admit a bounded private experiment, not an owner-selected product profile.
    /// # Errors
    /// Refuse ceilings outside the fixed experiment envelope or maximum above window.
    #[cfg(test)]
    pub(super) fn new(
        window: Duration,
        transactions: u8,
        refusals: u8,
        observations: u8,
        backoff: Backoff,
    ) -> Result<Self, Failure> {
        LimitsCandidate {
            values: (
                window,
                transactions,
                refusals,
                observations,
                backoff.maximum,
            ),
        }
        .validate()
        .map_err(|_| Failure::Limits)?;
        Ok(Self {
            window,
            transactions,
            refusals,
            observations,
            backoff: Some(backoff),
            context_lifetime: None,
        })
    }

    /// Select only a checked private context cap within the original outer window.
    /// # Errors
    /// Refuse zero or over-window lifetime without publishing changed limits.
    #[cfg(test)]
    pub(super) fn with_context_lifetime(mut self, lifetime: Duration) -> Result<Self, Failure> {
        ContextCandidate {
            values: (lifetime, self.window),
        }
        .validate()
        .map_err(|_| Failure::Limits)?;
        self.context_lifetime = Some(lifetime);
        Ok(self)
    }

    /// Match the existing sole public operation without a new retry default.
    const fn one_shot() -> Self {
        Self {
            window: EXCHANGE_TIME,
            transactions: 1,
            refusals: 1,
            observations: 2,
            backoff: None,
            context_lifetime: None,
        }
    }
}

/// Context limits are checked independently from the unchanged outer envelope.
#[cfg(test)]
#[derive(Validate)]
struct ContextCandidate {
    /// The custom rule owns positivity and the exact containing-window relation.
    #[garde(custom(context_rule))]
    values: (Duration, Duration),
}

/// Admit no context capable of extending the invocation's original window.
/// # Errors
/// Return a fixed redacted rejection for zero or excessive lifetime.
#[cfg(test)]
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde custom rules borrow fields and their unit context"
)]
fn context_rule(values: &(Duration, Duration), (): &()) -> garde::Result {
    let (lifetime, window) = *values;
    if lifetime.is_zero() || lifetime > window {
        return Err(garde::Error::new("invalid experimental context lifetime"));
    }
    Ok(())
}

/// Construct one original context deadline without extending the immutable cutoff.
/// # Errors
/// Refuse duration arithmetic or an already closed original window before work.
fn context_control(
    limits: &Limits,
    origin: Instant,
    cutoff: Instant,
    stop_scope: Arc<StopScope>,
) -> Result<Control, Failure> {
    let expiry = match limits.context_lifetime {
        Some(lifetime) => origin
            .checked_add(lifetime)
            .ok_or(Failure::Deadline)?
            .min(cutoff),
        None => cutoff,
    };
    Control::clamped_in(origin, expiry, stop_scope)
}

/// Garde is invoked inside the only raw experimental limit constructor.
#[cfg(test)]
#[derive(Validate)]
struct LimitsCandidate {
    /// This rule owns the complete cross-field cap/window relation.
    #[garde(custom(limits_rule))]
    values: (Duration, u8, u8, u8, Duration),
}

/// Keep each raw ceiling inside the actual bounded experiment envelope.
/// # Errors
/// Return a fixed redacted violation without retaining caller data.
#[cfg(test)]
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde custom rules borrow fields and their unit context"
)]
fn limits_rule(values: &(Duration, u8, u8, u8, Duration), (): &()) -> garde::Result {
    let (window, transactions, refusals, observations, maximum) = *values;
    if window.is_zero()
        || window > EXCHANGE_TIME
        || maximum > window
        || transactions == 0
        || transactions > TRANSACTIONS
        || refusals == 0
        || refusals > REFUSALS
        || !(2..=OBSERVATIONS).contains(&observations)
    {
        return Err(garde::Error::new("invalid experimental invocation limits"));
    }
    Ok(())
}

/// One real Config/claim/provider invocation; no raw association identity is minted.
pub(super) struct Invocation<'scope> {
    /// Immutable admitted input used by every producer entry.
    config: &'scope Config,
    /// The same actual provider/claim lifetime cannot be replaced between steps.
    provider: &'scope ProviderScope<'scope>,
    /// The original Control's immutable scope also binds every later context.
    stop_scope: Arc<StopScope>,
    /// Original checked Control's origin, never a caller-supplied scheduling time.
    origin: Instant,
    /// Same historical chart for every actual context, created before acquisition.
    run_context: Arc<crate::RunClockContext>,
    /// Immutable total window shared by all private experimental attempts.
    cutoff: Instant,
    /// Retain the original first Control instead of rebasing it at the first step.
    first_control: Option<Control>,
    /// Checked operation/count limits.
    limits: Limits,
    /// Entire next transaction plan; Closed never becomes Due again.
    plan: Plan,
    /// Persistent invocation-owned base, separate from transaction provenance.
    base: Option<Duration>,
    /// The KE failure's committed earliest retry boundary.
    route_not_before: Instant,
    /// Independent constraints are never cleared by successful authentication.
    source_not_before: Instant,
    /// Independent aggregate boundary, distinct from KE backoff.
    global_not_before: Instant,
    /// Original association expiry received from a genuine inert intent.
    intent_not_before: Option<Instant>,
    /// Producer entries irreversibly charged before calling it.
    transactions: u8,
    /// Scheduling refusals, never charged as remote reliability failures.
    refusals: u8,
    /// Charged observations include the current operation's reserved outcome.
    observations: u8,
    /// Retain the original failure independently of terminal scheduling reasons.
    last_failure: Option<Failed>,
    /// Successful original-context publications; never reset by KE refusals.
    contexts: u8,
    /// One retired context's history remains owned through establishment failures.
    previous: Option<PastRequests<'scope>>,
}

impl<'scope> Invocation<'scope> {
    /// Borrow the original pre-acquisition chart for actual component identity checks.
    #[cfg(test)]
    pub(super) const fn run_context(&self) -> &Arc<crate::RunClockContext> {
        &self.run_context
    }

    /// Receive the original checked public Control created before process admission.
    /// # Errors
    /// Refuse stale/closed original authority; no new process claim is created.
    pub(super) fn one_shot(
        config: &'scope Config,
        provider: &'scope ProviderScope<'scope>,
        control: Control,
    ) -> Result<Self, Failure> {
        control.remaining(super::IoPhase::Handshake)?;
        let limits = Limits::one_shot();
        let cutoff = control
            .origin
            .checked_add(limits.window)
            .ok_or(Failure::Deadline)?;
        if control.expiry > cutoff {
            return Err(Failure::Deadline);
        }
        let cutoff = control.expiry;
        Self::from_control(config, provider, limits, control, cutoff)
    }

    /// Bind explicit checked fixture parameters to one real shared invocation.
    /// # Errors
    /// Refuse cutoff/phase arithmetic before constructing usable custody.
    #[cfg(test)]
    pub(super) fn experimental(
        config: &'scope Config,
        provider: &'scope ProviderScope<'scope>,
        limits: Limits,
    ) -> Result<Self, Failure> {
        Self::with_limits(config, provider, limits)
    }

    /// Construct all immutable timing state from one internally observed origin.
    /// # Errors
    /// Refuse an unrepresentable cutoff or first Control before effects.
    #[cfg(test)]
    fn with_limits(
        config: &'scope Config,
        provider: &'scope ProviderScope<'scope>,
        limits: Limits,
    ) -> Result<Self, Failure> {
        let origin = Instant::now();
        let cutoff = origin.checked_add(limits.window).ok_or(Failure::Deadline)?;
        let control = context_control(&limits, origin, cutoff, Arc::new(StopScope::new()))?;
        Self::from_control(config, provider, limits, control, cutoff)
    }

    /// Move actual Control custody and bind every step to these borrowed owners.
    /// # Errors
    /// Refuse an excessive outer window or a context outside that window.
    fn from_control(
        config: &'scope Config,
        provider: &'scope ProviderScope<'scope>,
        limits: Limits,
        control: Control,
        cutoff: Instant,
    ) -> Result<Self, Failure> {
        let origin = control.origin;
        let maximum = origin.checked_add(limits.window).ok_or(Failure::Deadline)?;
        if origin >= control.expiry || control.expiry > cutoff || cutoff > maximum {
            return Err(Failure::Deadline);
        }
        Ok(Self {
            config,
            provider,
            stop_scope: Arc::clone(&control.stop_scope),
            origin,
            run_context: Arc::new(crate::RunClockContext { origin }),
            cutoff,
            first_control: Some(control),
            limits,
            plan: Plan::Due,
            base: None,
            route_not_before: origin,
            source_not_before: origin,
            global_not_before: origin,
            intent_not_before: None,
            transactions: 0,
            refusals: 0,
            observations: 0,
            last_failure: None,
            contexts: 0,
            previous: None,
        })
    }

    /// Derive only a stop capability from this invocation's actual original scope.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "the selected stop endpoint currently has only component-controller callers"
        )
    )]
    pub(super) fn stop_handle(&self) -> StopHandle {
        StopHandle {
            scope: Arc::clone(&self.stop_scope),
        }
    }

    /// Exercise the original checked cutoff boundary with genuine fixture owners.
    /// # Errors
    /// Preserve every mandatory original-control and outer-window refusal.
    #[cfg(test)]
    pub(super) fn checked_control_fixture(
        config: &'scope Config,
        provider: &'scope ProviderScope<'scope>,
        limits: Limits,
        control: Control,
        cutoff: Instant,
    ) -> Result<Self, Failure> {
        Self::from_control(config, provider, limits, control, cutoff)
    }

    /// Retain an actual charged Ready control inside this module's component probe.
    /// No control, receiver or prospective context ordinal escapes this boundary.
    /// # Errors
    /// Preserve admission failure; a deferred/closed step cannot supply authority.
    #[cfg(test)]
    fn probe_control(&mut self) -> Result<Control, Failed> {
        match self.admit()? {
            Admission::Ready { control, .. } => Ok(control),
            Admission::Finished(_) => Err(Failed::from(Failure::Control)),
        }
    }

    /// Observe genuine transient establishment failure and its actual local cancel.
    /// Keep the charged control private; neither setup nor cancellation schedules.
    /// # Errors
    /// Refuse nonrecoverable or unexpectedly successful setup without inventing facts.
    #[cfg(test)]
    fn probe_failed_control<P: Producer>(
        &mut self,
        producer: &mut P,
    ) -> Result<(Control, Failed, Backoff), Failed> {
        let policy = self.limits.backoff.ok_or(Failed::from(Failure::Control))?;
        let control = self.probe_control()?;
        let failure = match producer.establish(self.config, &control, self.provider) {
            Err(failure) => failure,
            Ok(association) => {
                drop(association);
                let mut receiver = ActiveReceiver {
                    invocation: self,
                    control: &control,
                };
                return Err(receiver.mode_failed(Failure::Control));
            }
        };
        let failure = {
            let receiver = ActiveReceiver {
                invocation: self,
                control: &control,
            };
            let (failure, cancellation) = receiver.close_admission(failure);
            receiver.invocation.last_failure = Some(failure);
            if receiver.invocation.close_after_scope(End::Failure) != End::Failure
                || failure.cleanup.is_some()
                || failure.admission_cleanup.is_some()
                || cancellation != Some(CancelOutcome::Cancelled)
                || !eligible(failure.primary)
            {
                return Err(failure);
            }
            failure
        };
        Ok((control, failure, policy))
    }

    /// Probe final scheduling publication after actual failed establishment/cancel.
    /// The trusted bounded nonreentrant observer may close, never admit, later work.
    /// # Errors
    /// The outer failure means setup did not reach the direct helper observation.
    #[cfg(test)]
    pub(super) fn probe_defer_observing<P: Producer>(
        &mut self,
        producer: &mut P,
        observe: impl FnOnce() -> Instant,
    ) -> Result<StepProbeResult, Failed> {
        let (control, failure, policy) = self.probe_failed_control(producer)?;
        let mut receiver = ActiveReceiver {
            invocation: self,
            control: &control,
        };
        Ok(receiver.defer_observing(policy, failure, producer, observe))
    }

    /// Inspect the direct retry result before any final defer check can mask it.
    /// Genuine failure/cancel setup spends work but returns no effect capability.
    /// # Errors
    /// The outer failure means actual setup never reached the boundary helper.
    #[cfg(test)]
    pub(super) fn probe_retry_boundary<P: Producer>(
        &mut self,
        producer: &mut P,
    ) -> Result<BoundaryProbeResult, Failed> {
        let (control, _failure, policy) = self.probe_failed_control(producer)?;
        let receiver = ActiveReceiver {
            invocation: self,
            control: &control,
        };
        Ok(receiver.retry_boundary(policy, producer))
    }

    /// Present a genuinely retired successor before ordinary history attachment.
    /// Only real admission and producer/ledger owners create the negative input;
    /// preserve the first context ordinal and history rather than fabricating them.
    /// # Errors
    /// The outer failure identifies setup that did not reach the receiving helper.
    #[cfg(test)]
    pub(super) fn probe_retired_successor<P: Producer>(
        &mut self,
        producer: &mut P,
    ) -> Result<StepProbeResult, Failed> {
        let control = self.probe_control()?;
        let config = self.config;
        let provider = self.provider;
        let mut association = producer.establish(config, &control, provider)?;
        let primary = match producer.exchange(&mut association, &control) {
            Err(failure) => failure,
            Ok(completion) => {
                drop(completion);
                drop(association);
                let mut receiver = ActiveReceiver {
                    invocation: self,
                    control: &control,
                };
                return Err(receiver.mode_failed(Failure::Control));
            }
        };
        if !matches!(
            primary,
            Failure::Ledger(super::association::ledger::LedgerError::Reply(
                super::response::ReplyError::Cookies
            ))
        ) {
            drop(association);
            let mut receiver = ActiveReceiver {
                invocation: self,
                control: &control,
            };
            return Err(receiver.mode_failed(primary));
        }
        let retired = association
            .retire_for_recovery(config, &control, provider)
            .map_err(Failed::from)?;
        let mut receiver = ActiveReceiver {
            invocation: self,
            control: &control,
        };
        Ok(receiver.exhausted(retired, primary, producer))
    }

    /// Return the unchanged original cause without granting another operation.
    pub(super) const fn last_failure(&self) -> Option<&Failed> {
        self.last_failure.as_ref()
    }

    /// Close admission without replacing an already observed producer/mode failure.
    /// Only an invocation with no prior failure records this admission cause.
    /// This transition creates no Control, producer call, charge or refund.
    pub(super) fn admission_failed(&mut self, failure: Failure, end: End) -> Failed {
        self.plan = Plan::Closed(end);
        if let Some(existing) = self.last_failure {
            existing
        } else {
            let failed = Failed::from(failure);
            self.last_failure = Some(failed);
            failed
        }
    }

    /// Close a returned failure under the actual scope without replacing its facts.
    fn close_after_scope(&mut self, fallback: End) -> End {
        let scope = Arc::clone(&self.stop_scope);
        if let Ok(guard) = scope.lock() {
            let end = if guard.is_stopped() {
                End::Stopped
            } else {
                fallback
            };
            self.plan = Plan::Closed(end);
            end
        } else {
            self.plan = Plan::Closed(End::Control);
            End::Control
        }
    }

    /// Admit one actual step atomically under the original invocation scope.
    /// Release all gates before any producer, entropy or native operation starts.
    /// # Errors
    /// Preserve prior failure facts on unavailable scope or original-control refusal.
    fn admit(&mut self) -> Result<Admission, Failed> {
        if let Plan::Closed(end) = self.plan {
            return Ok(Admission::Finished(Step::Closed(end)));
        }
        let scope = Arc::clone(&self.stop_scope);
        let scope_guard = match scope.lock() {
            Ok(guard) => guard,
            Err(failure) => return Err(self.admission_failed(failure, End::Control)),
        };
        if scope_guard.is_stopped() {
            self.plan = Plan::Closed(End::Stopped);
            return Ok(Admission::Finished(Step::Closed(End::Stopped)));
        }
        match self.plan_at_inner(Instant::now()) {
            Plan::Closed(end) => {
                self.plan = Plan::Closed(end);
                return Ok(Admission::Finished(Step::Closed(end)));
            }
            Plan::Deferred(boundary) => return Ok(Admission::Finished(self.refuse(boundary))),
            Plan::Due => {}
        }
        let (transactions, observations) = match self.reservation() {
            Ok(reservation) => reservation,
            Err(end) => {
                self.plan = Plan::Closed(end);
                return Ok(Admission::Finished(Step::Closed(end)));
            }
        };
        let Some(contexts) = self
            .contexts
            .checked_add(1)
            .filter(|next| *next <= CONTEXTS)
        else {
            self.plan = Plan::Closed(End::Contexts);
            return Ok(Admission::Finished(Step::Closed(End::Contexts)));
        };
        let control = match self.first_control.take() {
            Some(control) => control,
            None => match context_control(
                &self.limits,
                Instant::now(),
                self.cutoff,
                Arc::clone(&self.stop_scope),
            ) {
                Ok(control) => control,
                Err(failure) => return Err(self.admission_failed(failure, End::Arithmetic)),
            },
        };
        let guard = match control.guard_in(scope_guard) {
            Ok(guard) => guard,
            Err(failure) => return Err(self.admission_failed(failure, End::Control)),
        };
        if let Err(failure) = guard.remaining(super::IoPhase::Handshake, Instant::now()) {
            return Err(self.admission_failed(failure, End::Window));
        }
        self.transactions = transactions;
        self.observations = observations;
        // A producer unwind cannot leave an uncharged or newly admissible owner.
        self.plan = Plan::Closed(End::Failure);
        drop(guard);
        Ok(Admission::Ready { control, contexts })
    }

    /// Combine obligations without treating a copied intent as an admission permit.
    fn not_before(&self) -> Instant {
        self.route_not_before
            .max(self.source_not_before)
            .max(self.global_not_before)
            .max(self.intent_not_before.unwrap_or(self.origin))
    }

    /// Inspect an inert decision; only the real step may perform fresh admission.
    fn plan_at_inner(&self, now: Instant) -> Plan {
        if let Plan::Closed(end) = self.plan {
            return Plan::Closed(end);
        }
        if now < self.origin || now >= self.cutoff || self.not_before() >= self.cutoff {
            return Plan::Closed(End::Window);
        }
        if self.transactions >= self.limits.transactions {
            return Plan::Closed(End::Transactions);
        }
        if self
            .observations
            .checked_add(2)
            .is_none_or(|next| next > self.limits.observations)
        {
            return Plan::Closed(End::Observations);
        }
        let not_before = self.not_before();
        if now < not_before {
            Plan::Deferred(not_before)
        } else {
            Plan::Due
        }
    }

    /// Exercise the same planning comparisons without creating any capability.
    #[cfg(test)]
    pub(super) fn plan_at(&self, now: Instant) -> Plan {
        if let Plan::Closed(end) = self.plan {
            return Plan::Closed(end);
        }
        match self.stop_scope.lock() {
            Ok(scope) => {
                if scope.is_stopped() {
                    Plan::Closed(End::Stopped)
                } else {
                    self.plan_at_inner(now)
                }
            }
            Err(_) => Plan::Closed(End::Control),
        }
    }

    /// Tighten a fixture's real owner state; there is no operation-granting inverse.
    #[cfg(test)]
    pub(super) fn constrain_source_until(&mut self, instant: Instant) {
        self.source_not_before = self.source_not_before.max(instant);
    }

    /// Tighten an independent fixture boundary without changing any spent allowance.
    #[cfg(test)]
    pub(super) fn constrain_global_until(&mut self, instant: Instant) {
        self.global_not_before = self.global_not_before.max(instant);
    }

    /// Commit one no-producer refusal atomically; the last refusal is terminal.
    fn refuse(&mut self, not_before: Instant) -> Step {
        let Some(refusals) = self
            .refusals
            .checked_add(1)
            .filter(|next| *next <= self.limits.refusals)
        else {
            self.plan = Plan::Closed(End::Refusals);
            return Step::Closed(End::Refusals);
        };
        let Some(observations) = self
            .observations
            .checked_add(1)
            .filter(|next| *next <= self.limits.observations)
        else {
            self.plan = Plan::Closed(End::Observations);
            return Step::Closed(End::Observations);
        };
        self.refusals = refusals;
        self.observations = observations;
        if refusals == self.limits.refusals {
            self.plan = Plan::Closed(End::Refusals);
            Step::Closed(End::Refusals)
        } else {
            self.plan = Plan::Deferred(not_before);
            Step::Deferred(not_before)
        }
    }

    /// Reserve both observations and the producer entry, without partial mutation.
    /// # Errors
    /// Refuse a complete reservation outside either immutable count ceiling.
    fn reservation(&self) -> Result<(u8, u8), End> {
        let transactions = self
            .transactions
            .checked_add(1)
            .filter(|next| *next <= self.limits.transactions)
            .ok_or(End::Transactions)?;
        let observations = self
            .observations
            .checked_add(2)
            .filter(|next| *next <= self.limits.observations)
            .ok_or(End::Observations)?;
        Ok((transactions, observations))
    }

    /// Inspect finite state only; no test can mutate authority through this view.
    #[cfg(test)]
    pub(super) fn snapshot(&self) -> Snapshot {
        Snapshot {
            origin: self.origin,
            cutoff: self.cutoff,
            not_before: self.not_before(),
            source_not_before: self.source_not_before,
            global_not_before: self.global_not_before,
            intent_not_before: self.intent_not_before,
            transactions: self.transactions,
            refusals: self.refusals,
            observations: self.observations,
            contexts: self.contexts,
            has_previous: self.previous.is_some(),
            base: self.base,
            end: match self.plan {
                Plan::Closed(end) => Some(end),
                Plan::Due | Plan::Deferred(_) => None,
            },
        }
    }
}

/// Test-only immutable inspection of the actual driver owner.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Snapshot {
    /// Original internally captured origin.
    pub(super) origin: Instant,
    /// Original absolute invocation cutoff.
    pub(super) cutoff: Instant,
    /// Maximum applicable scheduling obligation.
    pub(super) not_before: Instant,
    /// Independently tightened source obligation.
    pub(super) source_not_before: Instant,
    /// Independently tightened global obligation.
    pub(super) global_not_before: Instant,
    /// Genuine intent's original not-before, if observed.
    pub(super) intent_not_before: Option<Instant>,
    /// Irreversibly charged KE producer entries.
    pub(super) transactions: u8,
    /// Recorded scheduling refusals.
    pub(super) refusals: u8,
    /// Charged/reserved transaction observations.
    pub(super) observations: u8,
    /// Successfully established contexts, independent of charged producer entries.
    pub(super) contexts: u8,
    /// Historical custody parked before replacement; never an admission permit.
    pub(super) has_previous: bool,
    /// Current route's capped base, never an association identity.
    pub(super) base: Option<Duration>,
    /// Terminal state cannot be reopened by another step.
    pub(super) end: Option<End>,
}

/// Private static adapter: native production and Script exercise one owner.
pub(super) trait Producer {
    /// Establish one checked association borrowing only these actual inputs.
    /// # Errors
    /// Preserve original establishment and separately observed cleanup failures.
    fn establish<'tx>(
        &mut self,
        config: &'tx Config,
        control: &'tx Control,
        provider: &'tx ProviderScope<'tx>,
    ) -> Result<AssociationOwner<'tx>, Failed>;

    /// Retire actual backend/runtime before returning an opaque completion.
    /// An error also returns only after actual backend/runtime retirement; logical
    /// ledger exhaustion alone cannot establish that physical completion fact.
    /// # Errors
    /// Preserve the existing protected-operation refusal and consumed budgets.
    fn exchange<'tx>(
        &mut self,
        association: &mut AssociationOwner<'tx>,
        control: &'tx Control,
    ) -> Result<ProtectedCompletion<'tx>, Failure>;

    /// Fill only the existing fixed-shape never-send preview entropy batch.
    /// # Errors
    /// Refuse the actual entropy operation without an alternative draw.
    fn preview_entropy(&mut self, output: &mut [u8; 48]) -> Result<(), Failure>;

    /// Supply one explicit private experiment draw, checked by the receiving owner.
    /// # Errors
    /// Refuse source failure; there is no selected native retry entropy source.
    fn jitter(&mut self, upper: Duration) -> Result<Duration, Failure>;
}

/// Exclusive current transaction receiver; no raw or numeric identity is retained.
struct ActiveReceiver<'owner, 'control, 'scope> {
    /// Exclusive access lasts through actual result acceptance.
    invocation: &'owner mut Invocation<'scope>,
    /// Actual locally owned Control whose lifetime bounds every fact.
    control: &'control Control,
}

impl ActiveReceiver<'_, '_, '_> {
    /// Detach a current genuine fact before closing this actual receiving owner.
    /// # Errors
    /// Reject a foreign actual Control without resetting route state.
    fn accept(
        &mut self,
        committed: CommittedProtected<'_>,
        population: (usize, usize),
    ) -> Result<InvocationCompletion, Failure> {
        committed.check_control(self.control)?;
        let history = committed.into_history(Arc::clone(&self.invocation.run_context), population);
        let completion = InvocationCompletion::Protected(Box::new(history));
        self.invocation.base = None;
        self.invocation.route_not_before = self.invocation.origin;
        self.invocation.plan = Plan::Closed(End::Completed);
        Ok(completion)
    }

    /// Consume only a genuine association's constraint; it cannot enable KE here.
    fn observe_intent(&mut self, intent: RecoveryIntent) {
        self.observe_boundary(intent.not_before());
    }

    /// Tighten only the original intent boundary received through an actual owner.
    fn observe_boundary(&mut self, boundary: Instant) {
        self.invocation.intent_not_before = Some(
            self.invocation
                .intent_not_before
                .map_or(boundary, |existing| existing.max(boundary)),
        );
    }

    /// Preserve one observed cancellation disposition beside unchanged failure facts.
    /// A previous cleanup error suppresses another observation; an actual new
    /// error occupies only the previously empty admission-cleanup slot.
    fn close_admission(&self, mut failure: Failed) -> (Failed, Option<CancelOutcome>) {
        if failure.admission_cleanup.is_some() {
            return (failure, None);
        }
        match self.control.cancel() {
            Ok(outcome) => (failure, Some(outcome)),
            Err(error) => {
                failure.admission_cleanup = Some(error);
                (failure, None)
            }
        }
    }

    /// Preserve the primary facts, then determine only direct-establishment recovery.
    /// # Errors
    /// Return the unchanged primary/cleanup facts whenever recovery is unavailable.
    fn establish_failed<P: Producer>(
        &mut self,
        failure: Failed,
        producer: &mut P,
    ) -> Result<Step, Failed> {
        let (failure, cancellation) = self.close_admission(failure);
        self.invocation.last_failure = Some(failure);
        if self.invocation.close_after_scope(End::Failure) != End::Failure {
            return Err(failure);
        }
        let Some(policy) = self.invocation.limits.backoff else {
            return Err(failure);
        };
        if failure.cleanup.is_some()
            || failure.admission_cleanup.is_some()
            || cancellation != Some(CancelOutcome::Cancelled)
            || !eligible(failure.primary)
        {
            return Err(failure);
        }
        self.defer(policy, failure, producer)
    }

    /// Schedule only an already admitted recovery disposition without refunding work.
    /// # Errors
    /// Preserve the original failure when count/window/draw constraints refuse.
    fn defer<P: Producer>(
        &mut self,
        policy: Backoff,
        failure: Failed,
        producer: &mut P,
    ) -> Result<Step, Failed> {
        self.defer_observing(policy, failure, producer, Instant::now)
    }

    /// Publish scheduling only after the final fresh observation under the scope.
    /// The trusted local observer is bounded and nonreentrant while the gate is held;
    /// it cannot replace actual admission or authorize a subsequent producer call.
    /// # Errors
    /// Preserve the original failure on scope, boundary, count or draw refusal.
    fn defer_observing<P: Producer>(
        &mut self,
        policy: Backoff,
        failure: Failed,
        producer: &mut P,
        observe: impl FnOnce() -> Instant,
    ) -> Result<Step, Failed> {
        let decision = self.retry_boundary(policy, producer);
        let scope = Arc::clone(&self.invocation.stop_scope);
        let Ok(guard) = scope.lock() else {
            self.invocation.plan = Plan::Closed(End::Control);
            return Err(failure);
        };
        if guard.is_stopped() {
            self.invocation.plan = Plan::Closed(End::Stopped);
            return Err(failure);
        }
        match decision {
            Ok((base, not_before)) => {
                if observe() >= self.invocation.cutoff || not_before >= self.invocation.cutoff {
                    self.invocation.plan = Plan::Closed(End::Window);
                    return Err(failure);
                }
                self.invocation.base = Some(base);
                self.invocation.route_not_before = not_before;
                self.invocation.plan = Plan::Deferred(not_before);
                Ok(Step::Deferred(not_before))
            }
            Err(end) => {
                self.invocation.plan = Plan::Closed(end);
                Err(failure)
            }
        }
    }

    /// Preserve a new context-transfer refusal separately from the current primary.
    fn context_failed(&mut self, stage: ContextStage, cause: Failure) -> Failed {
        let failure = self
            .invocation
            .last_failure
            .unwrap_or_else(|| Failed::from(cause));
        let failure = self.close_admission(failure).0;
        self.invocation.last_failure = Some(failure);
        // This actual context-stage refusal is already terminal history. A later
        // sticky stop still closes admission but must not erase the observed cause.
        self.invocation.plan = Plan::Closed(End::Context(stage, cause));
        failure
    }

    /// Consume only an exhausted first context from this actual transaction.
    /// # Errors
    /// Preserve the mode primary and independent retirement/cancellation failures.
    fn exhausted<P: Producer>(
        &mut self,
        retired: RetiredContext<'_>,
        primary: Failure,
        producer: &mut P,
    ) -> Result<Step, Failed> {
        self.invocation.last_failure = Some(Failed::from(primary));
        if self.invocation.contexts != 1 || self.invocation.previous.is_some() {
            return Err(self.context_failed(ContextStage::Retire, Failure::Control));
        }
        let (boundary, previous) = match retired.receive(
            self.invocation.config,
            self.control,
            self.invocation.provider,
        ) {
            Ok(retired) => retired,
            Err(cause) => return Err(self.context_failed(ContextStage::Retire, cause)),
        };
        self.observe_boundary(boundary);
        let (failure, cancellation) = self.close_admission(Failed::from(primary));
        self.invocation.last_failure = Some(failure);
        if cancellation != Some(CancelOutcome::Cancelled) {
            self.invocation.close_after_scope(End::Failure);
            return Err(failure);
        }
        let Some(policy) = self.invocation.limits.backoff else {
            self.invocation.close_after_scope(End::Failure);
            return Err(failure);
        };
        self.invocation.previous = Some(previous);
        self.defer(policy, failure, producer)
    }

    /// Calculate then commit scheduling as one transition, drawing at most once.
    /// # Errors
    /// Refuse exhausted observation/window capacity, invalid draws and arithmetic.
    fn retry_boundary<P: Producer>(
        &self,
        policy: Backoff,
        producer: &mut P,
    ) -> Result<(Duration, Instant), End> {
        let scope = self
            .invocation
            .stop_scope
            .lock()
            .map_err(|_| End::Control)?;
        if scope.is_stopped() {
            return Err(End::Stopped);
        }
        if self.invocation.transactions >= self.invocation.limits.transactions {
            return Err(End::Transactions);
        }
        let now = Instant::now();
        if self
            .invocation
            .observations
            .checked_add(2)
            .is_none_or(|next| next > self.invocation.limits.observations)
        {
            return Err(End::Observations);
        }
        let base = policy
            .base_after(self.invocation.base)
            .map_err(|_| End::Arithmetic)?;
        let minimum = now.checked_add(base).ok_or(End::Arithmetic)?;
        let minimum = minimum.max(self.invocation.not_before());
        if now >= self.invocation.cutoff || minimum >= self.invocation.cutoff {
            return Err(End::Window);
        }
        let upper = policy.draw_upper(base).map_err(|_| End::Arithmetic)?;
        // This admits the one draw, not its physical start or completion. Keep
        // neither gate across the actual source callback, including a late return.
        drop(scope);
        let draw = if upper.is_zero() {
            Duration::ZERO
        } else {
            producer.jitter(upper).map_err(End::Jitter)?
        };
        let delay = policy.delay(base, draw).map_err(|_| End::Arithmetic)?;
        let not_before = now
            .checked_add(delay)
            .ok_or(End::Arithmetic)?
            .max(self.invocation.not_before());
        if not_before >= self.invocation.cutoff || Instant::now() >= self.invocation.cutoff {
            return Err(End::Window);
        }
        Ok((base, not_before))
    }

    /// Every unselected established-mode refusal is terminal.
    fn mode_failed(&mut self, failure: Failure) -> Failed {
        let failure = self.close_admission(Failed::from(failure)).0;
        self.invocation.last_failure = Some(failure);
        self.invocation.close_after_scope(End::Failure);
        failure
    }
}

/// Recognize only native causes returned directly by the establishment producer.
fn eligible(failure: Failure) -> bool {
    match failure {
        Failure::Connect(kind)
        | Failure::Handshake(kind)
        | Failure::Request(kind)
        | Failure::Response(kind) => {
            matches!(
                kind,
                io::ErrorKind::TimedOut
                    | io::ErrorKind::ConnectionRefused
                    | io::ErrorKind::ConnectionReset
                    | io::ErrorKind::ConnectionAborted
            )
        }
        _ => false,
    }
}

/// Run one actual connected admission/producer/receiver transaction.
/// Only internally sampled fresh time can authorize this step's producer call.
/// # Errors
/// Preserve original producer/mode/cleanup facts; no failed step replenishes work.
pub(super) fn step_with<P: Producer>(
    invocation: &mut Invocation<'_>,
    producer: &mut P,
) -> Result<Step, Failed> {
    let (control, contexts) = match invocation.admit()? {
        Admission::Ready { control, contexts } => (control, contexts),
        Admission::Finished(result) => return Ok(result),
    };
    let config = invocation.config;
    let provider = invocation.provider;
    let mut receiver = ActiveReceiver {
        invocation,
        control: &control,
    };
    let mut association = match producer.establish(config, &control, provider) {
        Ok(association) => association,
        Err(failure) => return receiver.establish_failed(failure, producer),
    };
    receiver.invocation.contexts = contexts;
    if let Some(previous) = receiver.invocation.previous.take() {
        association = match association.install_previous(config, &control, provider, previous) {
            Ok(association) => association,
            Err(cause) => return Err(receiver.context_failed(ContextStage::Attach, cause)),
        };
    }
    let result = match config.mode {
        ExchangeMode::Preview => {
            super::preview_once(&mut association, |output| producer.preview_entropy(output))
                .map(InvocationCompletion::Preview)
        }
        ExchangeMode::ProtectedDatagram => super::protected_receiving(
            &mut association,
            |owner| producer.exchange(owner, &control),
            |committed, population| receiver.accept(committed, population),
        ),
    };
    match result {
        Ok(summary) => {
            drop(association);
            receiver.invocation.plan = Plan::Closed(End::Completed);
            Ok(Step::Complete(summary))
        }
        Err(failure)
            if receiver.invocation.limits.context_lifetime.is_some()
                && receiver.invocation.contexts == 1
                && matches!(
                    failure,
                    Failure::Ledger(super::association::ledger::LedgerError::Reply(
                        super::response::ReplyError::Cookies
                    ))
                ) =>
        {
            receiver.invocation.last_failure = Some(Failed::from(failure));
            match association.retire_for_recovery(config, &control, provider) {
                Ok(retired) => receiver.exhausted(retired, failure, producer),
                Err(cause) => Err(receiver.context_failed(ContextStage::Retire, cause)),
            }
        }
        Err(failure) => {
            if let Some(intent) = association.take_recovery_intent() {
                receiver.observe_intent(intent);
            }
            drop(association);
            Err(receiver.mode_failed(failure))
        }
    }
}

/// The sole normal adapter; this selects no native repeat or jitter source.
#[cfg(windows)]
pub(super) struct Native;

#[cfg(windows)]
impl Producer for Native {
    fn establish<'tx>(
        &mut self,
        config: &'tx Config,
        control: &'tx Control,
        provider: &'tx ProviderScope<'tx>,
    ) -> Result<AssociationOwner<'tx>, Failed> {
        super::transport::establish(config, control, provider)
    }

    fn exchange<'tx>(
        &mut self,
        association: &mut AssociationOwner<'tx>,
        control: &'tx Control,
    ) -> Result<ProtectedCompletion<'tx>, Failure> {
        super::datagram::exchange(association, control, |output| {
            use rustls::crypto::SecureRandom;
            rustls_cng_crypto::SecureRandom
                .fill(output)
                .map_err(|_| Failure::Entropy)
        })
    }

    fn preview_entropy(&mut self, output: &mut [u8; 48]) -> Result<(), Failure> {
        use rustls::crypto::SecureRandom;
        rustls_cng_crypto::SecureRandom
            .fill(output)
            .map_err(|_| Failure::Entropy)
    }

    fn jitter(&mut self, _upper: Duration) -> Result<Duration, Failure> {
        // Public one-shot admission terminates before asking for any retry draw.
        Err(Failure::Control)
    }
}

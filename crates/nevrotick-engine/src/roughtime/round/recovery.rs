// SPDX-License-Identifier: GPL-2.0-only
//! Retain the first signed contradiction across one controlled recovery invocation.
//!
//! The invocation owns the history and lends out one actual round at a time.
//! A caller may select one follow-up within the original budget. No decision
//! here clears a conflict or grants TLS or clock authority.

use super::{
    Conflict, OrderedRound, PEERS, ProbeProgress, RetryRoundPeer, RoundControl, RoundError,
    RoundPeer, RoundStatus, round_cutoff, selection,
};
use crate::native_network_job::JoinWitness;
use crate::roughtime::{
    exchange::{ExchangeError, Receipt, Withdrawal},
    interval::{ControlledDate, IntervalError, IntervalPolicy},
};
use std::{
    mem,
    net::SocketAddr,
    num::NonZeroU64,
    sync::{Mutex, MutexGuard},
    time::{Duration, Instant},
};
use time_domain::{LeapDateScope, LeapKnowledge};
use tokio::{
    sync::{Notify, futures::Notified},
    time,
};

mod proof;

pub use proof::{ConflictProof, ConflictProofError};

/// Live state of one exact controlled invocation.
pub(super) enum RecoveryPhase {
    /// The scheduler has not issued this invocation.
    Unused,
    /// The original invocation may continue within its admitted limits.
    Active(Option<JoinWitness>),
    /// The first acknowledged change permanently retired this authority.
    Withdrawn(Withdrawal),
}

/// One-use authority shared by the controlled scheduler and its receiver.
pub struct RecoveryControl {
    /// Serializes withdrawal with admission, native attempts and publication.
    phase: Mutex<RecoveryPhase>,
    /// Wakes a scheduled follow-up after the first withdrawal is recorded.
    withdrawn: Notify,
}

impl Default for RecoveryControl {
    fn default() -> Self {
        Self::new()
    }
}

impl RecoveryControl {
    /// Create one unused authority without admitting any network work.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            phase: Mutex::new(RecoveryPhase::Unused),
            withdrawn: Notify::const_new(),
        }
    }

    /// Permanently withdraw both rounds, preserving the first acknowledged cause.
    ///
    /// # Errors
    /// A contended or unavailable lock means the change was not acknowledged.
    pub fn withdraw(&self, cause: Withdrawal) -> Result<(), RoundError> {
        let mut guard: MutexGuard<'_, RecoveryPhase> = self.lock()?;
        if matches!(*guard, RecoveryPhase::Withdrawn(_)) {
            return Ok(());
        }
        if let RecoveryPhase::Active(Some(worker)) = &*guard {
            worker.cancel_dispatch();
        }
        *guard = RecoveryPhase::Withdrawn(cause);
        drop(guard);
        self.withdrawn.notify_waiters();
        Ok(())
    }

    /// Acquire the original cell without blocking.
    ///
    /// # Errors
    /// Refuses contention or poisoning.
    fn lock(&self) -> Result<MutexGuard<'_, RecoveryPhase>, RoundError> {
        self.phase
            .try_lock()
            .map_err(|_| RoundError::AuthorityUnavailable)
    }

    /// Hold this exact invocation live throughout an authorized effect.
    ///
    /// # Errors
    /// Refuses an unused, withdrawn or unavailable authority.
    pub(super) fn current(&self) -> Result<MutexGuard<'_, RecoveryPhase>, RoundError> {
        let guard: MutexGuard<'_, RecoveryPhase> = self.lock()?;
        match *guard {
            RecoveryPhase::Active(_) => Ok(guard),
            RecoveryPhase::Unused => Err(RoundError::Finished),
            RecoveryPhase::Withdrawn(cause) => Err(RoundError::Withdrawn(cause)),
        }
    }
}

/// One controlled invocation's immutable date interpretation inputs.
#[derive(Clone, Copy)]
pub struct RecoveryDatePolicy<'a> {
    /// Fixed projection and usefulness policy for all its receipts.
    interval: IntervalPolicy,
    /// Fixed date enclosure required before using the supplied calendar.
    scope: LeapDateScope,
    /// Exact immutable calendar supplied when the invocation is issued.
    calendar: &'a LeapKnowledge<'a>,
}

impl<'a> RecoveryDatePolicy<'a> {
    /// Bind the already checked controlled inputs before admitting any round.
    #[must_use]
    pub const fn new(
        interval: IntervalPolicy,
        scope: LeapDateScope,
        calendar: &'a LeapKnowledge<'a>,
    ) -> Self {
        Self {
            interval,
            scope,
            calendar,
        }
    }
}

/// Historical record of one selected peer's endpoint, root and declared groups.
#[derive(Clone, Copy)]
pub struct SelectedPeer {
    /// Exact destination in the original roster.
    endpoint: SocketAddr,
    /// Root selected before any send to this peer.
    root: [u8; 32],
    /// Declared operator group in the original controlled roster.
    operator: NonZeroU64,
    /// Declared upstream clock group in the original controlled roster.
    clock: NonZeroU64,
}

impl SelectedPeer {
    /// Read the original selected destination.
    #[must_use]
    pub const fn endpoint(self) -> SocketAddr {
        self.endpoint
    }

    /// Borrow the exact selected root representation.
    #[must_use]
    pub const fn root_key_bytes(&self) -> &[u8; 32] {
        &self.root
    }

    /// Read the original declared operator group.
    #[must_use]
    pub const fn operator(self) -> NonZeroU64 {
        self.operator
    }

    /// Read the original declared clock group.
    #[must_use]
    pub const fn clock(self) -> NonZeroU64 {
        self.clock
    }
}

/// Original signed conflict and the complete prefix that establishes causality.
#[must_use = "retain the original authenticated conflict and complete chain"]
pub struct ConflictEvidence {
    /// First conflicting positions in the original receipt sequence.
    conflict: Conflict,
    /// Exact receipts from the first request through the contradictory response.
    receipts: Vec<Receipt>,
    /// Original full roster, including a peer the returned prefix may not reach.
    selected: [SelectedPeer; PEERS],
    /// Original absolute deadline for the decision and all its exchanges.
    cutoff: Instant,
    /// Original declared upper per-probe delay limit.
    max_delay: Duration,
}

impl ConflictEvidence {
    /// Read the authenticated contradiction's positions in the retained prefix.
    #[must_use]
    pub const fn conflict(&self) -> Conflict {
        self.conflict
    }

    /// Borrow the original requests, signed responses, captures and chain inputs.
    #[must_use]
    pub fn receipts(&self) -> &[Receipt] {
        &self.receipts
    }

    /// Read one member of the original fixed selection in roster order.
    #[must_use]
    pub fn selected_peer(&self, index: usize) -> Option<SelectedPeer> {
        self.selected.get(index).copied()
    }

    /// Read the original round's exclusive cutoff.
    #[must_use]
    pub const fn cutoff(&self) -> Instant {
        self.cutoff
    }

    /// Read the original upper per-probe delay limit.
    #[must_use]
    pub const fn max_delay(&self) -> Duration {
        self.max_delay
    }
}

/// Own one controlled initial and follow-up round under a single original cutoff.
#[must_use = "retain the recovery owner through its controlled round sequence"]
pub struct ControlledRecovery<'a> {
    /// Authority shared with the scheduler's withdrawal path.
    control: &'a RecoveryControl,
    /// Original inputs for interpreting a date from either round.
    date: RecoveryDatePolicy<'a>,
    /// Exact bound selection held across both attempts when retry admission is enabled.
    retry_peers: Option<[RetryRoundPeer<'a>; PEERS]>,
    /// Optional one-shot controlled route change after observed UDP silence.
    fallback: Option<FallbackPlan>,
    /// Immutable exclusive expiry for every round in this invocation.
    cutoff: Instant,
    /// Each admitted round spends one slot, even if the caller later drops it.
    remaining_rounds: u8,
    /// The first authenticated contradiction, retained until this owner settles.
    first_conflict: Option<ConflictEvidence>,
}

impl<'a> ControlledRecovery<'a> {
    /// Establish an invocation deadline without issuing any network work.
    ///
    /// # Errors
    /// Refuses an invalid budget or an already issued, withdrawn or unavailable control.
    pub fn new(
        control: &'a RecoveryControl,
        budget: Duration,
        date: RecoveryDatePolicy<'a>,
    ) -> Result<Self, RoundError> {
        let cutoff: Instant = round_cutoff(budget, budget)?;
        Self::new_until(control, cutoff, date)
    }

    /// Claim the original invocation with an already captured exclusive cutoff.
    ///
    /// # Errors
    /// Refuses expired, already issued, withdrawn or unavailable control.
    fn new_until(
        control: &'a RecoveryControl,
        cutoff: Instant,
        date: RecoveryDatePolicy<'a>,
    ) -> Result<Self, RoundError> {
        let mut phase: MutexGuard<'_, RecoveryPhase> = control.lock()?;
        match *phase {
            RecoveryPhase::Unused if Instant::now() < cutoff => {
                *phase = RecoveryPhase::Active(None);
            }
            RecoveryPhase::Unused => return Err(RoundError::Expired),
            RecoveryPhase::Active(_) => return Err(RoundError::Finished),
            RecoveryPhase::Withdrawn(cause) => return Err(RoundError::Withdrawn(cause)),
        }
        Ok(Self {
            control,
            date,
            retry_peers: None,
            fallback: None,
            cutoff,
            remaining_rounds: 2,
            first_conflict: None,
        })
    }

    /// Establish a recovery invocation with one exact set of borrowed retry bindings.
    ///
    /// The invocation retains the selection until both rounds settle or it is dropped.
    ///
    /// # Errors
    /// Refuses the original invocation budget or control.
    pub fn new_with_retry(
        control: &'a RecoveryControl,
        budget: Duration,
        date: RecoveryDatePolicy<'a>,
        peers: [RetryRoundPeer<'a>; PEERS],
    ) -> Result<Self, RoundError> {
        let mut invocation: Self = Self::new(control, budget, date)?;
        invocation.retry_peers = Some(peers);
        Ok(invocation)
    }

    /// Select a fresh controlled measurement over advertised TCP after UDP silence.
    ///
    /// The caller drives bounded polls and the blocking connect path. The second
    /// round starts a fresh chain and changes only the unanswered peer's route.
    ///
    /// # Errors
    /// Refuses an invalid wait, missing TCP selection, original budget or control.
    pub fn new_with_fallback(
        control: &'a RecoveryControl,
        budget: Duration,
        date: RecoveryDatePolicy<'a>,
        peers: [RetryRoundPeer<'a>; PEERS],
        udp_wait: Duration,
    ) -> Result<Self, RoundError> {
        require_response_wait(udp_wait, budget)?;
        let fallback: FallbackPlan = FallbackPlan::new(&peers, udp_wait)?;
        let mut invocation: Self = Self::new_with_retry(control, budget, date, peers)?;
        invocation.fallback = Some(fallback);
        Ok(invocation)
    }

    /// Select one retry-ready independent trio and order from the controlled roster.
    ///
    /// The original deadline covers selection. The chosen order and bindings
    /// stay with the invocation for both rounds. A supplied UDP wait selects
    /// a trio with an eligible advertised TCP fallback.
    ///
    /// # Errors
    /// Refuses invalid inputs, no feasible trio, unavailable entropy, expiry or control.
    pub fn new_randomized(
        control: &'a RecoveryControl,
        budget: Duration,
        date: RecoveryDatePolicy<'a>,
        candidates: &'a mut [RetryRoundPeer<'_>],
        udp_wait: Option<Duration>,
    ) -> Result<Self, RoundError> {
        let cutoff: Instant = round_cutoff(budget, budget)?;
        if let Some(wait) = udp_wait {
            require_response_wait(wait, budget)?;
        }
        let observed_at: Instant = Instant::now();
        if observed_at >= cutoff {
            return Err(RoundError::Expired);
        }
        let peers: [RetryRoundPeer<'a>; PEERS] =
            selection::select(candidates, observed_at, udp_wait.is_some())?;
        let fallback: Option<FallbackPlan> = udp_wait
            .map(|wait| FallbackPlan::new(&peers, wait))
            .transpose()?;
        let mut invocation: Self = Self::new_until(control, cutoff, date)?;
        invocation.retry_peers = Some(peers);
        invocation.fallback = fallback;
        Ok(invocation)
    }

    /// Admit one selected round within the original invocation's time and attempts.
    ///
    /// Its exclusive borrow prevents overlapping rounds under this invocation.
    /// A successful admission spends a slot before any transmission is possible.
    ///
    /// ```compile_fail,E0499
    /// # use std::time::Duration;
    /// # use nevrotick_engine::roughtime::round::{RoundControl, RoundError, RoundPeer};
    /// # use nevrotick_engine::roughtime::round::recovery::ControlledRecovery;
    /// # fn overlapping(
    /// #     invocation: &mut ControlledRecovery<'_>, first_control: &RoundControl,
    /// #     second_control: &RoundControl, peers: [RoundPeer<'_>; 3],
    /// # ) -> Result<(), RoundError> {
    /// let stage = Duration::from_secs(1);
    /// let first = invocation.begin(first_control, peers, stage, stage)?;
    /// let second = invocation.begin(second_control, peers, stage, stage)?;
    /// drop((first, second));
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// Refuses exhausted rounds, elapsed deadlines or original round admission failure.
    pub fn begin<'round>(
        &'round mut self,
        control: &'round RoundControl,
        peers: [RoundPeer<'round>; PEERS],
        budget: Duration,
        max_delay: Duration,
    ) -> Result<RecoveryRound<'round>, RoundError> {
        self.begin_round(control, Some(peers), budget, max_delay)
    }

    /// Reborrow the exact invocation retry selection for the next controlled round.
    ///
    /// # Errors
    /// Refuses an unbound invocation or the original slot, deadline or round admission.
    pub fn begin_bound<'round>(
        &'round mut self,
        control: &'round RoundControl,
        budget: Duration,
        max_delay: Duration,
    ) -> Result<RecoveryRound<'round>, RoundError> {
        self.begin_round(control, None, budget, max_delay)
    }

    /// Wait for the original fallback retry floor and admit the one follow-up.
    ///
    /// The first poll fixes the deadline. Dropping a pending wait leaves the
    /// original retry state and round slot intact. The returned round submits
    /// TCP connects through the shared native worker lane.
    ///
    /// # Errors
    /// Refuses absent silence or route, a spent slot, deadline, withdrawn or
    /// unavailable control, or the original round admission failure.
    ///
    /// # Panics
    /// A deferred retry requires the Engine runtime's Tokio time driver.
    pub async fn begin_scheduled_fallback<'round>(
        &'round mut self,
        control: &'round RoundControl,
        budget: Duration,
        max_delay: Duration,
    ) -> Result<RecoveryRound<'round>, RoundError> {
        if self.remaining_rounds == 0 {
            return Err(RoundError::Finished);
        }
        if self.remaining_rounds != 1 || self.fallback.is_none() {
            return Err(RoundError::FallbackUnavailable);
        }
        let cutoff: Instant = round_cutoff(budget, max_delay)?.min(self.cutoff);
        // Subscribe before checking the phase so a concurrent withdrawal cannot
        // be lost between the floor check and the first timer poll.
        let withdrawal: Notified<'_> = self.control.withdrawn.notified();
        match self.fallback_route(cutoff) {
            Err(RoundError::Exchange(ExchangeError::RetryDeferred { until })) => {
                tokio::select! {
                    biased;
                    () = withdrawal => {}
                    () = time::sleep_until(time::Instant::from_std(until)) => {}
                }
            }
            result => {
                result?;
            }
        }
        let mut round: RecoveryRound<'round> =
            self.begin_round_until(control, None, cutoff, max_delay)?;
        round.round.worker_connect = true;
        Ok(round)
    }

    /// Admit exactly one original round and lend it the invocation's history.
    ///
    /// # Errors
    /// Refuses a selection-mode mismatch, original slot or round admission failure.
    fn begin_round<'round>(
        &'round mut self,
        control: &'round RoundControl,
        peers: Option<[RoundPeer<'round>; PEERS]>,
        budget: Duration,
        max_delay: Duration,
    ) -> Result<RecoveryRound<'round>, RoundError> {
        if self.remaining_rounds == 0 {
            return Err(RoundError::Finished);
        }
        let cutoff: Instant = round_cutoff(budget, max_delay)?.min(self.cutoff);
        self.begin_round_until(control, peers, cutoff, max_delay)
    }

    /// Admit against the fixed effective cutoff without starting a new timer.
    ///
    /// # Errors
    /// Refuses a selection-mode mismatch or the original round admission failure.
    fn begin_round_until<'round>(
        &'round mut self,
        control: &'round RoundControl,
        peers: Option<[RoundPeer<'round>; PEERS]>,
        cutoff: Instant,
        max_delay: Duration,
    ) -> Result<RecoveryRound<'round>, RoundError> {
        if peers.is_some() == self.retry_peers.is_some() {
            return Err(RoundError::Configuration);
        }
        let route: Option<(usize, SocketAddr)> = self.fallback_route(cutoff)?;
        let mut round: OrderedRound<'round> = match (peers, self.retry_peers.as_mut()) {
            (Some(peers), None) => {
                OrderedRound::start_until(control, Some(self.control), peers, cutoff, max_delay)
            }
            (None, Some([first, second, third])) => {
                let mut selected: [RetryRoundPeer<'round>; PEERS] =
                    [first.reborrow(), second.reborrow(), third.reborrow()];
                if let Some((slot, endpoint)) = route {
                    selected[slot].peer.endpoint = endpoint;
                }
                OrderedRound::start_bound_until(
                    control,
                    Some(self.control),
                    selected,
                    cutoff,
                    max_delay,
                )
            }
            _ => return Err(RoundError::Configuration),
        }?;
        round.tcp_slot = route.map(|(slot, _)| slot);
        if self.remaining_rounds == 2 {
            round.udp_wait = self.fallback.as_ref().map(|plan| plan.udp_wait);
        }
        self.remaining_rounds -= 1;
        Ok(RecoveryRound {
            round,
            date: self.date,
            first_conflict: &mut self.first_conflict,
            fallback: self.fallback.as_mut(),
        })
    }

    /// Select only the recorded advertised route once its original floor is due.
    ///
    /// # Errors
    /// Refuses missing silence evidence, unavailable authority, deadline or floor.
    fn fallback_route(&self, cutoff: Instant) -> Result<Option<(usize, SocketAddr)>, RoundError> {
        let Some(plan): Option<&FallbackPlan> = self.fallback.as_ref() else {
            return Ok(None);
        };
        if self.remaining_rounds == 2 {
            return Ok(None);
        }
        let unanswered: &UnansweredRound = plan
            .unanswered
            .as_ref()
            .ok_or(RoundError::FallbackUnavailable)?;
        let endpoint: SocketAddr =
            plan.tcp[unanswered.slot].ok_or(RoundError::FallbackUnavailable)?;
        let peers: &[RetryRoundPeer<'_>; PEERS] =
            self.retry_peers.as_ref().ok_or(RoundError::Configuration)?;
        let _current: MutexGuard<'_, RecoveryPhase> = self.control.current()?;
        let now: Instant = Instant::now();
        if now >= cutoff {
            return Err(RoundError::Expired);
        }
        match peers[unanswered.slot].retry.check(now) {
            Err(ExchangeError::RetryDeferred { until }) if until >= cutoff => {
                Err(RoundError::Expired)
            }
            Err(error) => Err(RoundError::Exchange(error)),
            Ok(()) => Ok(Some((unanswered.slot, endpoint))),
        }
    }

    /// Borrow the retired UDP prefix separately from the fresh fallback chain.
    #[must_use]
    pub fn fallback_prefix(&self) -> Option<&[Receipt]> {
        self.fallback
            .as_ref()?
            .unanswered
            .as_ref()
            .map(|round| round.receipts.as_slice())
    }

    /// Borrow the first historical signed conflict, even after its round expires.
    #[must_use]
    pub const fn first_conflict(&self) -> Option<&ConflictEvidence> {
        self.first_conflict.as_ref()
    }
}

/// Require a positive child response wait strictly shorter than the original budget.
///
/// # Errors
/// Refuses a zero or whole-budget response wait.
fn require_response_wait(wait: Duration, budget: Duration) -> Result<(), RoundError> {
    if wait.is_zero() || wait >= budget {
        return Err(RoundError::Configuration);
    }
    Ok(())
}

/// Original controlled fallback parameters and first settled silence evidence.
struct FallbackPlan {
    /// Original per-leg UDP response wait.
    udp_wait: Duration,
    /// Preselected TCP routes in the same operator order.
    tcp: [Option<SocketAddr>; PEERS],
    /// Retained prefix and unanswered origin for the only follow-up.
    unanswered: Option<UnansweredRound>,
}

impl FallbackPlan {
    /// Copy only the selected advertised alternatives into the invocation.
    ///
    /// # Errors
    /// Refuses a roster with no selected TCP route.
    fn new(peers: &[RetryRoundPeer<'_>; PEERS], udp_wait: Duration) -> Result<Self, RoundError> {
        let tcp: [Option<SocketAddr>; PEERS] = peers.each_ref().map(|peer| peer.tcp);
        if tcp.iter().all(Option::is_none) {
            return Err(RoundError::Configuration);
        }
        Ok(Self {
            udp_wait,
            tcp,
            unanswered: None,
        })
    }
}

/// Historical prefix from the original unanswered UDP attempt's round.
struct UnansweredRound {
    /// Exact logical peer of the final unanswered attempt.
    slot: usize,
    /// Original authenticated prefix, unavailable to the fresh round's admission.
    receipts: Vec<Receipt>,
}

/// Sole receiving and date boundary for a controlled invocation's active round.
#[must_use = "drive or settle the one admitted round before advancing recovery"]
pub struct RecoveryRound<'a> {
    /// Exact original bounded round; no unguarded date evaluation is exposed.
    round: OrderedRound<'a>,
    /// Original date inputs bound by the same one-use invocation.
    date: RecoveryDatePolicy<'a>,
    /// The same invocation history before, during and after this round.
    first_conflict: &'a mut Option<ConflictEvidence>,
    /// Original invocation's optional controlled fallback evidence slot.
    fallback: Option<&'a mut FallbackPlan>,
}

impl RecoveryRound<'_> {
    /// Submit the next request through the original round's send guard.
    ///
    /// # Errors
    /// Returns the original operation's refusal.
    pub fn start_next(&mut self) -> Result<SocketAddr, RoundError> {
        self.round.start_next()
    }

    /// Settle the active read and retain its original chain if it is the first conflict.
    ///
    /// # Errors
    /// Returns the original operation's refusal, including the specific contradiction.
    pub fn poll(&mut self) -> Result<ProbeProgress, RoundError> {
        let result: Result<ProbeProgress, RoundError> = self.round.poll();
        if matches!(
            &result,
            Err(RoundError::Exchange(ExchangeError::Unanswered))
        ) && let Some(plan) = self.fallback.as_mut()
            && plan.unanswered.is_none()
        {
            plan.unanswered = Some(UnansweredRound {
                slot: self.round.receipts.len() % PEERS,
                receipts: mem::take(&mut self.round.receipts),
            });
        }
        if let Err(RoundError::Conflict(conflict)) = &result
            && self.first_conflict.is_none()
        {
            let selected: [SelectedPeer; PEERS] = self.round.peers.map(|peer| SelectedPeer {
                endpoint: peer.endpoint,
                root: *peer.root.0.as_bytes(),
                operator: peer.operator,
                clock: peer.clock,
            });
            *self.first_conflict = Some(ConflictEvidence {
                conflict: *conflict,
                receipts: mem::take(&mut self.round.receipts),
                selected,
                cutoff: self.round.cutoff,
                max_delay: self.round.max_delay,
            });
        }
        result
    }

    /// Read the current round's disposition independently of historical conflict.
    #[must_use]
    pub const fn status(&self) -> RoundStatus {
        self.round.status()
    }

    /// Interpret a complete date for readback without releasing retry debt.
    ///
    /// # Errors
    /// Refuses a retained contradiction or an original date-evaluation refusal.
    pub fn evaluate_date(&self) -> Result<ControlledDate<'_>, IntervalError> {
        self.require_no_conflict()?;
        self.round
            .evaluate_date(&self.date.interval, self.date.scope, self.date.calendar)
    }

    /// Admit the complete bound date once while the invocation has no signed conflict.
    ///
    /// # Errors
    /// Refuses a retained contradiction or an original bound date admission failure.
    pub fn admit_date(&mut self) -> Result<ControlledDate<'_>, IntervalError> {
        self.require_no_conflict()?;
        self.round
            .admit_date(&self.date.interval, self.date.scope, self.date.calendar)
    }

    /// Require that no retained signed contradiction bars the date path.
    ///
    /// # Errors
    /// Returns the exact first retained conflict.
    fn require_no_conflict(&self) -> Result<(), IntervalError> {
        if let Some(evidence) = self.first_conflict.as_ref() {
            return Err(IntervalError::Conflict(evidence.conflict));
        }
        Ok(())
    }
}

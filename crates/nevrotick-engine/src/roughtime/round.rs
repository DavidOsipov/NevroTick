// SPDX-License-Identifier: GPL-2.0-only
//! A bounded three-traversal Roughtime round on one selected loopback roster.
//!
//! The controlled caller supplies declared operator and clock provenance.
//! This owner checks distinct declarations and consumes actual authenticated
//! exchanges. Its result is historical causal evidence for later policy.

use super::{
    RootKey,
    exchange::{
        ExchangeError, ExchangeTransport, PendingExchange, Receipt, RoundAuthority, StagedReceipt,
        Withdrawal,
        retry::{RetryBinding, RetryLease, RetryState},
        tcp::{
            BeforeConnectDispatch, ConnectAttempt, PendingConnect, PendingTcpExchange,
            connect_native,
        },
    },
    interval::{self, ControlledAgreement, ControlledDate, IntervalError, IntervalPolicy},
};
use crate::{MAX_TIMEOUT, native_network_job::JoinWitness};
use recovery::{RecoveryControl, RecoveryPhase};
use std::{
    io,
    net::{SocketAddr, TcpStream},
    num::NonZeroU64,
    sync::{Mutex, MutexGuard},
    task::Poll,
    time::{Duration, Instant},
};
use time_domain::{LeapDateScope, LeapKnowledge};

pub mod recovery;
mod selection;

#[cfg(test)]
mod date_tests;
#[cfg(test)]
mod worker_tests;

/// The minimum number of selected peer identities in the controlled round.
pub(super) const PEERS: usize = 3;
/// Visit the same selected order in all three traversals.
pub(super) const EXCHANGES: usize = 3 * PEERS;

/// A selected endpoint, root and two independently declared dependency groups.
#[derive(Clone, Copy)]
pub struct RoundPeer<'a> {
    /// The precise destination and expected responding endpoint.
    endpoint: SocketAddr,
    /// Long-term key selected for this peer.
    root: &'a RootKey,
    /// Declared operator group within the selected scope.
    operator: NonZeroU64,
    /// Declared upstream clock group within the selected scope.
    clock: NonZeroU64,
}

impl<'a> RoundPeer<'a> {
    /// Describe a selected peer for validation together with its fixed roster.
    #[must_use]
    pub const fn new(
        endpoint: SocketAddr,
        root: &'a RootKey,
        operator: NonZeroU64,
        clock: NonZeroU64,
    ) -> Self {
        Self {
            endpoint,
            root,
            operator,
            clock,
        }
    }

    /// Check both exact peer identities before granting independent support.
    fn distinct_identity(self, other: Self) -> bool {
        self.endpoint != other.endpoint && self.root.0.as_bytes() != other.root.0.as_bytes()
    }

    /// Require distinct peer identities and both declared dependency groups.
    fn independent(self, other: Self) -> bool {
        self.distinct_identity(other)
            && self.operator != other.operator
            && self.clock != other.clock
    }
}

/// One controlled round selection with its exact exclusive retry state.
pub struct RetryRoundPeer<'a> {
    /// Original alias, bound root and declared groups used for every visit.
    peer: RoundPeer<'a>,
    /// State retained by the original scheduler binding outside this round.
    retry: &'a mut RetryState,
    /// Optional preselected TCP route under the same root and retry state.
    tcp: Option<SocketAddr>,
}

impl<'a> RetryRoundPeer<'a> {
    /// Select the original binding's UDP route before a native attempt.
    ///
    /// # Errors
    /// Refuses an endpoint or transport outside the original binding.
    pub fn new(
        binding: &'a mut RetryBinding<'_>,
        endpoint: SocketAddr,
        operator: NonZeroU64,
        clock: NonZeroU64,
    ) -> Result<Self, ExchangeError> {
        let (root, retry): (&RootKey, &mut RetryState) =
            binding.selection(endpoint, ExchangeTransport::Udp)?;
        Ok(Self {
            peer: RoundPeer::new(endpoint, root, operator, clock),
            retry,
            tcp: None,
        })
    }

    /// Select UDP and advertised TCP routes under the same original binding.
    ///
    /// # Errors
    /// Refuses either endpoint and transport outside the original route set.
    pub fn new_with_tcp(
        binding: &'a mut RetryBinding<'_>,
        udp: SocketAddr,
        tcp: SocketAddr,
        operator: NonZeroU64,
        clock: NonZeroU64,
    ) -> Result<Self, ExchangeError> {
        binding.require_route(tcp, ExchangeTransport::Tcp)?;
        let mut selected: Self = Self::new(binding, udp, operator, clock)?;
        selected.tcp = Some(tcp);
        Ok(selected)
    }

    /// Reborrow this exact selection for one short-lived invocation round.
    fn reborrow(&mut self) -> RetryRoundPeer<'_> {
        RetryRoundPeer {
            peer: self.peer,
            retry: self.retry,
            tcp: self.tcp,
        }
    }
}

/// The first contradictory pair of authenticated responses in receipt order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Conflict {
    /// Earlier authenticated response with a lower bound beyond the later upper bound.
    earlier: usize,
    /// Later authenticated response with an upper bound below the earlier lower bound.
    later: usize,
}

impl Conflict {
    /// Return the two zero-based positions in the retained receipt sequence.
    #[must_use]
    pub const fn positions(self) -> (usize, usize) {
        (self.earlier, self.later)
    }

    /// Find the first earlier signed lower bound strictly beyond this upper bound.
    fn against(later: usize, mut earlier: impl Iterator<Item = u64>, upper: u64) -> Option<Self> {
        let earlier: usize = earlier.position(|lower| lower > upper)?;
        Some(Self { earlier, later })
    }
}

/// Disposition of this exact controlled round.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RoundStatus {
    /// The fixed chain may still accept a new receipt.
    Active,
    /// All nine ordered, delay-bounded responses completed under current authority.
    Complete,
    /// A refusal ended this round without complete causal evidence.
    Unavailable,
    /// A signed contradiction ended this round and retains the original evidence.
    Conflict(Conflict),
}

/// Progress at the active receive boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use = "continue or settle this round according to its progress"]
pub enum ProbeProgress {
    /// The current bounded read left the original probe pending.
    Waiting,
    /// One authenticated receipt was appended and the next probe may start.
    Accepted,
    /// The final receipt completed the fixed sequence.
    Complete,
}

/// Reasons the controlled round cannot perform its requested transition.
#[derive(Debug)]
#[non_exhaustive]
pub enum RoundError {
    /// An endpoint, group, root or budget is unsuitable for this roster.
    Configuration,
    /// No retry-ready independent trio can satisfy the requested round mode.
    SelectionUnavailable,
    /// No original unanswered UDP attempt admitted the configured fallback.
    FallbackUnavailable,
    /// The bounded receipt storage could not be reserved before sending.
    Storage,
    /// The round control lock could not be acquired without waiting.
    AuthorityUnavailable,
    /// The owner permanently withdrew the round.
    Withdrawn(Withdrawal),
    /// A probe is already awaiting its disposition.
    InFlight,
    /// A receive was requested between probes.
    NoProbe,
    /// This round or its one-use control has already advanced or settled.
    Finished,
    /// No positive receipt can be published past the original round cutoff.
    Expired,
    /// The original send-to-receive bracket exceeded the declared delay.
    Delay,
    /// The actual one-shot exchange refused.
    Exchange(ExchangeError),
    /// A retained pair's signed bounds contradict causal ordering.
    Conflict(Conflict),
}

/// Race-safe external control data for one exact round.
struct ControlState {
    /// Irreversibly claimed when this control is used to construct the round.
    started: bool,
    /// The first acknowledged authority change.
    withdrawn: Option<Withdrawal>,
    /// Exact pending native dispatch, cancellable before the worker claims it.
    connect: Option<JoinWitness>,
}

/// Original round and optional invocation authority held for one effect.
struct EffectGuards<'a> {
    /// The exact original per-round authority.
    round: MutexGuard<'a, ControlState>,
    /// When controlled by recovery, the exact invocation is checked first.
    recovery: Option<MutexGuard<'a, RecoveryPhase>>,
}

impl EffectGuards<'_> {
    /// Enroll the same dispatch latch in the two original withdrawal owners.
    fn track_connect(&mut self, worker: JoinWitness) {
        if let Some(RecoveryPhase::Active(current)) = self.recovery.as_deref_mut() {
            *current = Some(worker.clone());
        }
        self.round.connect = Some(worker);
    }
}

/// Acquire the original authorities in invocation-to-round order.
///
/// # Errors
/// Refuses a parent or round that is withdrawn, spent or unavailable.
fn effect_guards<'a>(
    round: &'a RoundControl,
    recovery: Option<&'a RecoveryControl>,
) -> Result<EffectGuards<'a>, RoundError> {
    let recovery_guard: Option<MutexGuard<'a, RecoveryPhase>> =
        recovery.map(RecoveryControl::current).transpose()?;
    let round_guard: MutexGuard<'a, ControlState> = round.current()?;
    Ok(EffectGuards {
        round: round_guard,
        recovery: recovery_guard,
    })
}

/// A single round authority with nine fixed subordinate send opportunities.
pub struct RoundControl {
    /// Serializes receipt publication and send admission against withdrawal.
    control: Mutex<ControlState>,
    /// Original per-position send and receive authorities.
    probes: [RoundAuthority; EXCHANGES],
}

impl Default for RoundControl {
    fn default() -> Self {
        Self::new()
    }
}

impl RoundControl {
    /// Create an unused authority without sending or allocating sockets.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            control: Mutex::new(ControlState {
                started: false,
                withdrawn: None,
                connect: None,
            }),
            probes: [const { RoundAuthority::new() }; EXCHANGES],
        }
    }

    /// Permanently withdraw this round, preserving the first delivered cause.
    ///
    /// # Errors
    /// Reports contention or an unavailable lock. The caller must settle an
    /// undelivered withdrawal before treating it as acknowledged.
    pub fn withdraw(&self, cause: Withdrawal) -> Result<(), RoundError> {
        let mut control: MutexGuard<'_, ControlState> = self.control()?;
        if control.withdrawn.is_none() {
            if let Some(worker) = control.connect.take() {
                worker.cancel_dispatch();
            }
            control.withdrawn = Some(cause);
        }
        Ok(())
    }

    /// Acquire the round guard without blocking.
    ///
    /// # Errors
    /// Refuses a contended or poisoned lock.
    fn control(&self) -> Result<MutexGuard<'_, ControlState>, RoundError> {
        self.control
            .try_lock()
            .map_err(|_| RoundError::AuthorityUnavailable)
    }

    /// Acquire the round guard only while the original authority stays live.
    ///
    /// # Errors
    /// Refuses unavailable or withdrawn authority.
    fn current(&self) -> Result<MutexGuard<'_, ControlState>, RoundError> {
        let control: MutexGuard<'_, ControlState> = self.control()?;
        if let Some(cause) = control.withdrawn {
            return Err(RoundError::Withdrawn(cause));
        }
        Ok(control)
    }
}

/// Fixed-roster owner of exactly one live probe and up to nine original receipts.
#[must_use = "retain the ordered round until its original authority settles"]
pub struct OrderedRound<'a> {
    /// Exact control shared with the current round owner's withdrawal path.
    control: &'a RoundControl,
    /// Exact one-use scheduling authority when part of a controlled recovery.
    recovery: Option<&'a RecoveryControl>,
    /// Immutable order reused on all three traversals.
    peers: [RoundPeer<'a>; PEERS],
    /// Original round cutoff covering every send, read and acceptance.
    cutoff: Instant,
    /// Per-leg upper delay bound inside the overall lifetime.
    max_delay: Duration,
    /// The only possible submitted probe, at the next index in the sequence.
    pending: Option<PendingExchange<'a>>,
    /// The same one-attempt ownership when the selected route uses TCP.
    pending_tcp: Option<PendingTcpExchange<'a>>,
    /// Exact one-worker connect ahead of any I/O on the selected stream.
    pending_connect: Option<PendingConnect<'a>>,
    /// Scheduled rounds dispatch TCP through the shared worker lane.
    worker_connect: bool,
    /// Dispatch preparation transferred only to this round's next selected worker.
    before_connect_dispatch: BeforeConnectDispatch,
    /// The one peer scheduled over TCP in a fresh fallback round.
    tcp_slot: Option<usize>,
    /// Dedicated UDP response window for the original round, when enabled.
    udp_wait: Option<Duration>,
    /// Bound retry states. Only the original live leg temporarily takes its slot.
    retries: Option<[Option<&'a mut RetryState>; PEERS]>,
    /// Original authenticated evidence, including a retained conflict or delay witness.
    /// The complete capacity is reserved fallibly before any send.
    receipts: Vec<Receipt>,
    /// Current progress or absorbing terminal disposition.
    status: RoundStatus,
}

impl<'a> OrderedRound<'a> {
    /// Bind a fixed controlled selection and an absolute round deadline.
    ///
    /// The selected order may be supplied by the caller. Catalogue validation
    /// and random selection from a larger eligible roster belong to that caller.
    ///
    /// # Errors
    /// Refuses mixed or invalid roster identity, timing limits, or an already
    /// started, withdrawn or unavailable control, or failure to reserve storage.
    pub fn new(
        control: &'a RoundControl,
        peers: [RoundPeer<'a>; PEERS],
        budget: Duration,
        max_delay: Duration,
    ) -> Result<Self, RoundError> {
        let cutoff: Instant = round_cutoff(budget, max_delay)?;
        Self::start_until(control, None, peers, cutoff, max_delay)
    }

    /// Admit a controlled round selected from three exclusive retry bindings.
    ///
    /// The previous debt of each signed leg remains until complete date admission.
    ///
    /// # Errors
    /// Refuses the original round's limits, selection, control or storage failure.
    pub fn new_with_retry(
        control: &'a RoundControl,
        peers: [RetryRoundPeer<'a>; PEERS],
        budget: Duration,
        max_delay: Duration,
    ) -> Result<Self, RoundError> {
        let cutoff: Instant = round_cutoff(budget, max_delay)?;
        Self::start_bound_until(control, None, peers, cutoff, max_delay)
    }

    /// Start one exact bound selection under an optional original invocation.
    ///
    /// # Errors
    /// Refuses the original round's selection, deadline, control or storage failure.
    fn start_bound_until(
        control: &'a RoundControl,
        recovery: Option<&'a RecoveryControl>,
        peers: [RetryRoundPeer<'a>; PEERS],
        cutoff: Instant,
        max_delay: Duration,
    ) -> Result<Self, RoundError> {
        let [first, second, third]: [RetryRoundPeer<'a>; PEERS] = peers;
        let mut round: Self = Self::start_until(
            control,
            recovery,
            [first.peer, second.peer, third.peer],
            cutoff,
            max_delay,
        )?;
        round.retries = Some([Some(first.retry), Some(second.retry), Some(third.retry)]);
        Ok(round)
    }

    /// Claim the original round control with a checked absolute cutoff.
    ///
    /// # Errors
    /// Refuses invalid peers, expired, spent or unavailable authority, or storage failure.
    fn start_until(
        control: &'a RoundControl,
        recovery: Option<&'a RecoveryControl>,
        peers: [RoundPeer<'a>; PEERS],
        cutoff: Instant,
        max_delay: Duration,
    ) -> Result<Self, RoundError> {
        for (index, peer) in peers.iter().enumerate() {
            let noncanonical: bool = matches!(peer.endpoint, SocketAddr::V6(address)
                if address.flowinfo() != 0 || address.scope_id() != 0);
            if noncanonical
                || !peer.endpoint.ip().is_loopback()
                || peer.endpoint.port() == 0
                || peers[..index]
                    .iter()
                    .any(|earlier| !peer.independent(*earlier))
            {
                return Err(RoundError::Configuration);
            }
        }
        let mut guards: EffectGuards<'_> = effect_guards(control, recovery)?;
        if guards.round.started {
            return Err(RoundError::Finished);
        }
        if Instant::now() >= cutoff {
            return Err(RoundError::Expired);
        }
        let mut receipts: Vec<Receipt> = Vec::new();
        receipts
            .try_reserve_exact(EXCHANGES)
            .map_err(|_| RoundError::Storage)?;
        guards.round.started = true;
        Ok(Self {
            control,
            recovery,
            peers,
            cutoff,
            max_delay,
            pending: None,
            pending_tcp: None,
            pending_connect: None,
            worker_connect: false,
            before_connect_dispatch: BeforeConnectDispatch::READY,
            tcp_slot: None,
            udp_wait: None,
            retries: None,
            receipts,
            status: RoundStatus::Active,
        })
    }

    /// Submit the next original probe using the previous complete response.
    ///
    /// # Errors
    /// Refuses a duplicate, spent, expired or withdrawn step, unavailable
    /// authority, or the original send failure. A failed send ends the round.
    pub fn start_next(&mut self) -> Result<SocketAddr, RoundError> {
        self.start_next_with(connect_native)
    }

    /// Submit the original next leg using the closed native connector route.
    ///
    /// # Errors
    /// Returns an original refusal before successor work is possible.
    fn start_next_with(
        &mut self,
        connect: impl FnOnce(SocketAddr, Duration) -> io::Result<TcpStream> + Send + 'static,
    ) -> Result<SocketAddr, RoundError> {
        self.require_active()?;
        if self.pending.is_some() || self.pending_tcp.is_some() || self.pending_connect.is_some() {
            return Err(RoundError::InFlight);
        }
        let control: &'a RoundControl = self.control;
        let mut current: EffectGuards<'a> = match self.current() {
            Ok(guard) => guard,
            Err(error) => return self.unavailable(error),
        };
        let position: usize = self.receipts.len();
        let attempt: &'a RoundAuthority = match control.probes.get(position) {
            Some(attempt) => attempt,
            None => return self.unavailable(RoundError::Finished),
        };
        let slot: usize = position % PEERS;
        let peer: RoundPeer<'a> = self.peers[slot];
        let retry: Option<&'a mut RetryState> = match self.retries.as_mut() {
            Some(retries) => match retries[slot].take() {
                Some(retry) => Some(retry),
                None => return self.unavailable(RoundError::Exchange(ExchangeError::Spent)),
            },
            None => None,
        };
        if self.tcp_slot == Some(slot) {
            let Some(state): Option<&'a mut RetryState> = retry else {
                return self.unavailable(RoundError::Exchange(ExchangeError::Spent));
            };
            return self.start_tcp(attempt, peer, state, &mut current, connect);
        }
        let sent: Result<PendingExchange<'a>, ExchangeError> = PendingExchange::send_bound(
            attempt,
            peer.endpoint,
            peer.root,
            self.cutoff,
            &self.receipts,
            retry,
            self.udp_wait,
        );
        match sent {
            Ok(pending) => {
                self.pending = Some(pending);
                Ok(peer.endpoint)
            }
            Err(error) => self.unavailable(RoundError::Exchange(error)),
        }
    }

    /// Submit the selected TCP leg under the original parent and round guards.
    ///
    /// # Errors
    /// Returns the original reservation or connector refusal.
    fn start_tcp(
        &mut self,
        attempt: &'a RoundAuthority,
        peer: RoundPeer<'a>,
        retry: &'a mut RetryState,
        current: &mut EffectGuards<'a>,
        connect: impl FnOnce(SocketAddr, Duration) -> io::Result<TcpStream> + Send + 'static,
    ) -> Result<SocketAddr, RoundError> {
        if self.worker_connect {
            let pending: PendingConnect<'a> = match PendingConnect::start(
                attempt,
                peer.endpoint,
                peer.root,
                self.cutoff,
                &self.receipts,
                retry,
                ConnectAttempt {
                    connect,
                    before_dispatch: std::mem::replace(
                        &mut self.before_connect_dispatch,
                        BeforeConnectDispatch::READY,
                    ),
                },
            ) {
                Ok(pending) => pending,
                Err(error) => return self.unavailable(RoundError::Exchange(error)),
            };
            current.track_connect(pending.witness());
            self.pending_connect = Some(pending);
        } else {
            let pending: PendingTcpExchange<'a> = match PendingTcpExchange::connect_bound(
                attempt,
                peer.endpoint,
                peer.root,
                self.cutoff,
                &self.receipts,
                retry,
            ) {
                Ok(pending) => pending,
                Err(error) => return self.unavailable(RoundError::Exchange(error)),
            };
            self.pending_tcp = Some(pending);
        }
        Ok(peer.endpoint)
    }

    /// Poll the original leg and accept only under the re-acquired round guard.
    ///
    /// The round owns the native poll and bounded verification until they
    /// return. A withdrawal delivered before the next guard prevents
    /// receipt publication and successor sends.
    ///
    /// # Errors
    /// Refuses a missing or settled probe, ended authority, original exchange
    /// failure, excessive delay, or an authenticated causal contradiction.
    pub fn poll(&mut self) -> Result<ProbeProgress, RoundError> {
        self.require_active()?;
        let read_guard: EffectGuards<'a> = match self.current() {
            Ok(guard) => guard,
            Err(error) => return self.unavailable(error),
        };
        if self.pending_connect.is_some() {
            return self.poll_connect();
        }
        let observed: Result<Option<StagedReceipt<'a>>, ExchangeError> =
            match (self.pending.as_mut(), self.pending_tcp.as_mut()) {
                (Some(udp), None) => udp.try_receive(),
                (None, Some(tcp)) => tcp.poll(),
                _ => return Err(RoundError::NoProbe),
            };
        drop(read_guard);
        let _current: EffectGuards<'a> = match self.current() {
            Ok(guard) => guard,
            Err(error) => return self.unavailable(error),
        };
        let staged: StagedReceipt<'a> = match observed {
            Ok(Some(receipt)) => receipt,
            Ok(None) => return Ok(ProbeProgress::Waiting),
            Err(error) => return self.unavailable(RoundError::Exchange(error)),
        };
        drop(self.pending.take());
        drop(self.pending_tcp.take());
        let (receipt, lease): (Receipt, Option<RetryLease<'a>>) = match staged.publish_tentative() {
            Ok(received) => received,
            Err(error) => return self.unavailable(RoundError::Exchange(error)),
        };
        let slot: usize = self.receipts.len() % PEERS;
        let progress: ProbeProgress = self.accept(receipt)?;
        match (self.retries.as_mut(), lease) {
            (Some(retries), Some(lease)) if retries[slot].is_none() => match lease.tentative() {
                Ok(state) => retries[slot] = Some(state),
                Err(error) => return self.unavailable(RoundError::Exchange(error)),
            },
            (None, None) => {}
            _ => return self.unavailable(RoundError::Exchange(ExchangeError::Spent)),
        }
        Ok(progress)
    }

    /// Settle or adopt the worker result under the original live outer guards.
    ///
    /// # Errors
    /// Refuses elapsed custody, unexpected missing state or original worker failure.
    fn poll_connect(&mut self) -> Result<ProbeProgress, RoundError> {
        if Instant::now() >= self.cutoff {
            return self.unavailable(RoundError::Expired);
        }
        let pending: &mut PendingConnect<'a> =
            self.pending_connect.as_mut().ok_or(RoundError::NoProbe)?;
        let stream: TcpStream = match pending.try_join() {
            Poll::Pending => return Ok(ProbeProgress::Waiting),
            Poll::Ready(Ok(stream)) => stream,
            Poll::Ready(Err(error)) => return self.unavailable(RoundError::Exchange(error)),
        };
        let pending: PendingConnect<'a> = self.pending_connect.take().ok_or(RoundError::NoProbe)?;
        match pending.adopt(stream) {
            Ok(exchange) => {
                self.pending_tcp = Some(exchange);
                Ok(ProbeProgress::Waiting)
            }
            Err(error) => self.unavailable(RoundError::Exchange(error)),
        }
    }

    /// Read the disposition last observed by this owner.
    #[must_use]
    pub const fn status(&self) -> RoundStatus {
        self.status
    }

    /// Borrow original evidence at the specified zero-based sequence position.
    #[must_use]
    pub fn receipt(&self, index: usize) -> Option<&Receipt> {
        self.receipts.get(index)
    }

    /// Evaluate the original complete round using one current monotonic sample.
    ///
    /// The returned interval is conditional on this controlled roster and the
    /// declared capture and rate model. Re-evaluation retains the original ages.
    ///
    /// # Errors
    /// Refuses incomplete, conflicted, withdrawn, expired or stale evidence,
    /// unavailable authority, or a failed projection or agreement check.
    pub fn evaluate(
        &self,
        policy: &IntervalPolicy,
    ) -> Result<ControlledAgreement<'_>, IntervalError> {
        self.evaluate_with(policy, Ok)
    }

    /// Evaluate and interpret the complete interval in the supplied date scope.
    ///
    /// The original round guard covers the evaluation and interpretation. The
    /// calendar answer belongs to the supplied ledger and grants no clock effect.
    ///
    /// # Errors
    /// Refuses failed round evaluation, unrepresentable coordinates or a date
    /// interval outside the supplied scope.
    pub fn evaluate_date(
        &self,
        policy: &IntervalPolicy,
        scope: LeapDateScope,
        calendar: &LeapKnowledge<'_>,
    ) -> Result<ControlledDate<'_>, IntervalError> {
        self.evaluate_with(policy, |agreement: ControlledAgreement<'_>| {
            ControlledDate::admit(agreement, scope, calendar)
        })
    }

    /// Admit one complete guarded date and reset its three original retry states.
    ///
    /// A refused whole-date interpretation spends this admission without reset.
    ///
    /// # Errors
    /// Refuses incomplete, conflicting, withdrawn, unbound or inadmissible evidence.
    pub fn admit_date(
        &mut self,
        policy: &IntervalPolicy,
        scope: LeapDateScope,
        calendar: &LeapKnowledge<'_>,
    ) -> Result<ControlledDate<'_>, IntervalError> {
        self.admit_date_observed(policy, scope, calendar, || Some(Instant::now()))
    }

    /// Complete the one-use admission under the original guard and owner clock.
    ///
    /// # Errors
    /// Refuses an unavailable observation or any failed date admission without resetting debt.
    fn admit_date_observed(
        &mut self,
        policy: &IntervalPolicy,
        scope: LeapDateScope,
        calendar: &LeapKnowledge<'_>,
        observe: impl FnMut() -> Option<Instant>,
    ) -> Result<ControlledDate<'_>, IntervalError> {
        let _current: EffectGuards<'a> = self.evaluation_guard()?;
        let [Some(first), Some(second), Some(third)]: [Option<&'a mut RetryState>; PEERS] =
            self.retries.take().ok_or(IntervalError::RetryAdmission)?
        else {
            return Err(IntervalError::RetryAdmission);
        };
        let date: ControlledDate<'_> = Self::evaluate_original(
            &self.receipts,
            self.peers.map(|peer| (peer.operator, peer.clock)),
            self.cutoff,
            policy,
            |agreement: ControlledAgreement<'_>| ControlledDate::admit(agreement, scope, calendar),
            observe,
        )?;
        for retry in [first, second, third] {
            retry.reset();
        }
        Ok(date)
    }

    /// Complete one pure evaluation and optional transformation under the guard.
    ///
    /// # Errors
    /// Returns a round, projection or consumer refusal before returning evidence.
    fn evaluate_with<'round, T>(
        &'round self,
        policy: &IntervalPolicy,
        consume: impl FnOnce(ControlledAgreement<'round>) -> Result<T, IntervalError>,
    ) -> Result<T, IntervalError> {
        let _current: EffectGuards<'a> = self.evaluation_guard()?;
        Self::evaluate_original(
            &self.receipts,
            self.peers.map(|peer| (peer.operator, peer.clock)),
            self.cutoff,
            policy,
            consume,
            || Some(Instant::now()),
        )
    }

    /// Hold the exact completed round's current authority during evaluation.
    ///
    /// # Errors
    /// Refuses incomplete, conflicting, withdrawn or unavailable authority.
    fn evaluation_guard(&self) -> Result<EffectGuards<'a>, IntervalError> {
        match self.status {
            RoundStatus::Complete => {}
            RoundStatus::Conflict(conflict) => return Err(IntervalError::Conflict(conflict)),
            RoundStatus::Active | RoundStatus::Unavailable => {
                return Err(IntervalError::NotComplete);
            }
        }
        self.current().map_err(|error| match error {
            RoundError::Withdrawn(cause) => IntervalError::Withdrawn(cause),
            _ => IntervalError::AuthorityUnavailable,
        })
    }

    /// Evaluate exact receipts and their consumer under the already-held guard.
    ///
    /// # Errors
    /// Refuses a projection, interpretation or original time failure.
    fn evaluate_original<'round, T>(
        receipts: &'round [Receipt],
        groups: [(NonZeroU64, NonZeroU64); PEERS],
        cutoff: Instant,
        policy: &IntervalPolicy,
        consume: impl FnOnce(ControlledAgreement<'round>) -> Result<T, IntervalError>,
        mut observe: impl FnMut() -> Option<Instant>,
    ) -> Result<T, IntervalError> {
        let evaluated_at: Instant = observe().ok_or(IntervalError::ClockUnavailable)?;
        let agreement: ControlledAgreement<'round> =
            interval::evaluate(receipts, groups, evaluated_at, cutoff, policy)?;
        let result: T = consume(agreement)?;
        let returned_at: Instant = observe().ok_or(IntervalError::ClockUnavailable)?;
        if returned_at < evaluated_at {
            return Err(IntervalError::CaptureOrder);
        }
        if returned_at >= cutoff {
            return Err(IntervalError::Expired);
        }
        Ok(result)
    }

    /// Acquire every original authority needed for publication or native work.
    ///
    /// # Errors
    /// Returns the exact parent or round refusal.
    fn current(&self) -> Result<EffectGuards<'a>, RoundError> {
        effect_guards(self.control, self.recovery)
    }

    /// Require an unsettled round before undertaking any new effect.
    ///
    /// # Errors
    /// Refuses every terminal disposition.
    fn require_active(&self) -> Result<(), RoundError> {
        if self.status != RoundStatus::Active {
            return Err(RoundError::Finished);
        }
        Ok(())
    }

    /// Settle without granting complete evidence and retire any owned socket.
    ///
    /// # Errors
    /// Returns the refusal that ended the active round.
    fn unavailable<T>(&mut self, refusal: RoundError) -> Result<T, RoundError> {
        self.status = RoundStatus::Unavailable;
        drop(self.pending.take());
        drop(self.pending_tcp.take());
        drop(self.pending_connect.take());
        Err(refusal)
    }

    /// Append one authenticated receipt and compare all earlier signed bounds.
    ///
    /// # Errors
    /// Preserves the conflicting pair, or refuses an out-of-delay or late result.
    fn accept(&mut self, receipt: Receipt) -> Result<ProbeProgress, RoundError> {
        let later: usize = self.receipts.len();
        let (_, upper): (u64, u64) = receipt.bounds_seconds();
        let conflict: Option<Conflict> = Conflict::against(
            later,
            self.receipts
                .iter()
                .map(|previous| previous.bounds_seconds().0),
            upper,
        );
        let ((send_before, _), (_, receive_after)): ((Instant, Instant), (Instant, Instant)) =
            receipt.captures();
        let delay: Option<Duration> = receive_after.checked_duration_since(send_before);
        self.receipts.push(receipt);
        if let Some(conflict) = conflict {
            self.status = RoundStatus::Conflict(conflict);
            return Err(RoundError::Conflict(conflict));
        }
        if delay.is_none_or(|actual| actual > self.max_delay) {
            return self.unavailable(RoundError::Delay);
        }
        if Instant::now() >= self.cutoff {
            return self.unavailable(RoundError::Expired);
        }
        if self.receipts.len() == EXCHANGES {
            self.status = RoundStatus::Complete;
            Ok(ProbeProgress::Complete)
        } else {
            Ok(ProbeProgress::Accepted)
        }
    }
}

/// Check relative round limits and sample their original absolute deadline.
///
/// # Errors
/// Refuses an invalid duration pair or unrepresentable deadline.
fn round_cutoff(budget: Duration, max_delay: Duration) -> Result<Instant, RoundError> {
    if budget.is_zero() || budget > MAX_TIMEOUT || max_delay.is_zero() || max_delay > budget {
        return Err(RoundError::Configuration);
    }
    Instant::now()
        .checked_add(budget)
        .ok_or(RoundError::Configuration)
}

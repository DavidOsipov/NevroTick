// SPDX-License-Identifier: GPL-2.0-only
//! One Engine-owned Roughtime transmission and signed receipt on loopback.
//!
//! The scheduling owner withdraws this one-shot authority on any change to
//! its selected round, catalogue, key, policy, or clock frame. Its lifetime
//! grants one send and at most one cryptographic response check. This module
//! produces historical single-peer evidence for a later aggregation decision.

use super::{PendingRequest, RootKey, VerifiedResponse, VerifyError, request, truncated_hash};
use crate::{IoCause, MAX_TIMEOUT, RECEIVE_CAP};
use ntp_wire::roughtime::MAX_RESPONSE_BYTES;
use retry::{RetryBinding, RetryLease, RetryState};
use std::{
    io,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket},
    sync::{Mutex, MutexGuard},
    time::{Duration, Instant},
};

pub mod retry;
pub mod tcp;

/// Leave room to distinguish an oversized prefix from an exact-size reply.
const RECEIVE_BYTES: usize = MAX_RESPONSE_BYTES + 1;

/// Which live constraint withdrew the original round opportunity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Withdrawal {
    /// The scheduling owner replaced the round.
    Round,
    /// The selected catalogue changed.
    Catalogue,
    /// The selected key record changed, even if its bytes remained equal.
    Key,
    /// The effective acquisition policy changed.
    Policy,
    /// The original monotonic clock frame or its validity changed.
    ClockFrame,
    /// The owner requested stop.
    Stop,
}

/// The first refusal at the controlled one-exchange boundary.
#[derive(Debug)]
#[non_exhaustive]
pub enum ExchangeError {
    /// The endpoint or time budget is outside this controlled scope.
    Configuration,
    /// The exact authority could not be acquired without blocking.
    AuthorityUnavailable,
    /// An authority change permanently retired this opportunity.
    Withdrawn(Withdrawal),
    /// This opportunity already started or the original response was consumed.
    Spent,
    /// The original exclusive cutoff elapsed.
    Expired,
    /// A submitted UDP request's dedicated response window closed with an empty read.
    Unanswered,
    /// The selected logical peer's original retry floor is still in force.
    RetryDeferred {
        /// Earliest monotonic instant at which a new attempt may be charged.
        until: Instant,
    },
    /// The original charged attempt has no observed settlement.
    RetryInFlight,
    /// The original binding cannot represent a safe retry floor in this clock frame.
    RetryClockUnavailable,
    /// The shared native network worker lane is still occupied.
    ConnectBusy,
    /// The original native connect worker failed to start or settle.
    ConnectWorker,
    /// System entropy did not supply fresh bytes for this chain position.
    Entropy,
    /// A native operation failed with a bounded error code.
    Socket(IoCause),
    /// A send failed to confirm the whole original datagram.
    PartialSend,
    /// The original capture brackets do not have a usable order.
    CaptureOrder,
    /// All selected read attempts have been charged.
    ReceiveBudget,
    /// The actual source address differs from the selected peer.
    ForeignPeer,
    /// The received frame exceeded its transport size limit or was truncated.
    Truncated,
    /// The stream ended before transmitting or receiving a complete frame.
    IncompleteFrame,
    /// The original frame or raw answer failed its cryptographic checks.
    Verification(VerifyError),
}

/// Lifetime state shared by the sender and the final receiving effect.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
enum Phase {
    /// Nothing has started.
    #[default]
    Ready,
    /// Preparation was charged and the original request has not finished sending.
    Preparing,
    /// The exact native send completed and the original response may be read.
    Awaiting,
    /// The original read opportunity is consumed, with one possible transfer.
    Verifying,
    /// The attempt has settled without any remaining effects.
    Spent,
    /// The first acknowledged withdrawal permanently prevents effects.
    Withdrawn(Withdrawal),
}

/// Unique, nonrenewable authority for one controlled attempt.
///
/// The owner routes withdrawal to the exact object borrowed by the attempt.
/// Callers may create a new owner for a distinct attempt. Each actual send
/// generates its own nonce, so a fresh authority cannot reuse stored bytes.
#[derive(Default)]
pub struct RoundAuthority {
    /// Serialize one native submission, read and final publication against withdrawal.
    phase: Mutex<Phase>,
}

impl RoundAuthority {
    /// Create one unused opportunity with no socket or packet effects.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            phase: Mutex::new(Phase::Ready),
        }
    }

    /// Withdraw the original opportunity before its next guarded effect.
    ///
    /// # Errors
    /// Reports contention or an unavailable lock. The owner must retain that
    /// withdrawal request until delivery is acknowledged or the attempt ends.
    pub fn withdraw(&self, cause: Withdrawal) -> Result<(), ExchangeError> {
        let mut phase: MutexGuard<'_, Phase> = self.phase()?;
        if !matches!(*phase, Phase::Withdrawn(_) | Phase::Spent) {
            *phase = Phase::Withdrawn(cause);
        }
        Ok(())
    }

    /// Acquire the exact serialization guard without waiting.
    ///
    /// # Errors
    /// Refuses a contended or poisoned lock.
    fn phase(&self) -> Result<MutexGuard<'_, Phase>, ExchangeError> {
        self.phase
            .try_lock()
            .map_err(|_| ExchangeError::AuthorityUnavailable)
    }
}

/// Require an exact predecessor phase and retain a specific withdrawal cause.
///
/// # Errors
/// Refuses withdrawn or already advanced authority.
fn require_phase(current: Phase, expected: Phase) -> Result<(), ExchangeError> {
    match current {
        Phase::Withdrawn(cause) => Err(ExchangeError::Withdrawn(cause)),
        phase if phase == expected => Ok(()),
        _ => Err(ExchangeError::Spent),
    }
}

/// Ordered monotonic brackets for an actual socket operation.
#[derive(Clone, Copy)]
struct Capture {
    /// Monotonic observation immediately before the native operation.
    before: Instant,
    /// Monotonic observation immediately after the native operation.
    after: Instant,
}

/// Request bound to the original endpoint and selected key before submission.
struct OriginalRequest<'a> {
    /// Exact checked destination and expected response source.
    peer: SocketAddr,
    /// Key selected before serializing the request.
    root: &'a RootKey,
    /// Original packet transport, preserved into the authenticated receipt.
    transport: ExchangeTransport,
    /// Complete immutable bytes selected for this transmission.
    frame: [u8; request::REQUEST_BYTES],
    /// Fresh random bytes used with the previous complete frame, if chained.
    chaining_randomness: Option<[u8; 32]>,
}

/// The only original sent datagram and its still-owned socket.
struct Sent<'a> {
    /// One socket bound for this transmission and its response.
    socket: UdpSocket,
    /// Exact submitted request and its immutable source binding.
    request: OriginalRequest<'a>,
    /// Monotonic bracket of the actual successful send.
    capture: Capture,
    /// When scheduled, original retry custody through failure or publication.
    retry: Option<RetryLease<'a>>,
}

/// An actual full loopback send awaiting its first consumed datagram.
#[must_use = "retain and settle the original Roughtime attempt"]
pub struct PendingExchange<'a> {
    /// The exact authority reserved by the send.
    authority: &'a RoundAuthority,
    /// Taken on the first consumed reply or terminal refusal.
    sent: Option<Sent<'a>>,
    /// Exclusive cutoff established before setup.
    cutoff: Instant,
    /// Optional earlier response-window cutoff for a qualified transport fallback.
    response_cutoff: Option<Instant>,
    /// Nonblocking native read calls remaining in this attempt.
    reads_left: u16,
}

impl<'a> PendingExchange<'a> {
    /// Charge one attempt and submit a fresh Roughtime request to a loopback peer.
    ///
    /// The original cutoff covers entropy, setup, submission, receipt and
    /// publication. The nonblocking send has no retransmission.
    ///
    /// # Errors
    /// Refuses invalid configuration, unavailable or withdrawn authority,
    /// entropy or socket failure, expiry, or an incomplete native send.
    pub fn send(
        authority: &'a RoundAuthority,
        peer: SocketAddr,
        root: &'a RootKey,
        budget: Duration,
    ) -> Result<Self, ExchangeError> {
        let cutoff: Instant = exchange_cutoff(budget)?;
        Self::send_bound(authority, peer, root, cutoff, &[], None, None)
    }

    /// Submit one controlled UDP request from the original scheduler binding.
    ///
    /// Its alias and root are checked together. A real native attempt retains
    /// exclusive custody until retirement or current single-response publication.
    /// This entry grants only one exchange and supplies no round policy.
    ///
    /// # Errors
    /// Refuses an ineligible alias, original retry floor, or exchange admission failure.
    pub fn send_with_retry(
        authority: &'a RoundAuthority,
        binding: &'a mut RetryBinding<'_>,
        peer: SocketAddr,
        budget: Duration,
    ) -> Result<Self, ExchangeError> {
        let cutoff: Instant = exchange_cutoff(budget)?;
        let (root, state): (&RootKey, &mut RetryState) = binding.admission(peer)?;
        Self::send_bound(authority, peer, root, cutoff, &[], Some(state), None)
    }

    /// Prepare one original UDP send and charge any bound retry at native entry.
    ///
    /// # Errors
    /// Refuses preparation, deadline, retry-floor or native send failures.
    pub(super) fn send_bound(
        authority: &'a RoundAuthority,
        peer: SocketAddr,
        root: &'a RootKey,
        cutoff: Instant,
        history: &[Receipt],
        retry: Option<&'a mut RetryState>,
        response_wait: Option<Duration>,
    ) -> Result<Self, ExchangeError> {
        if !peer.ip().is_loopback() || peer.port() == 0 {
            return Err(ExchangeError::Configuration);
        }
        let request: OriginalRequest<'a> = prepare(
            authority,
            peer,
            root,
            cutoff,
            history,
            ExchangeTransport::Udp,
        )?;
        let local: IpAddr = match peer.ip() {
            IpAddr::V4(_) => IpAddr::V4(Ipv4Addr::LOCALHOST),
            IpAddr::V6(_) => IpAddr::V6(Ipv6Addr::LOCALHOST),
        };
        let socket: UdpSocket =
            UdpSocket::bind(SocketAddr::new(local, 0)).map_err(|error| socket_error(&error))?;
        socket
            .set_nonblocking(true)
            .map_err(|error| socket_error(&error))?;
        let mut phase: MutexGuard<'_, Phase> = authority.phase()?;
        require_phase(*phase, Phase::Preparing)?;
        let before: Instant = Instant::now();
        if before >= cutoff {
            return Err(ExchangeError::Expired);
        }
        let mut sent: Sent<'a> = Sent {
            socket,
            request,
            capture: Capture {
                before,
                after: before,
            },
            retry: retry.map(|state| state.charge(before)).transpose()?,
        };
        let written: usize = sent
            .socket
            .send_to(&sent.request.frame, peer)
            .map_err(|error| socket_error(&error))?;
        let after: Instant = Instant::now();
        if written != sent.request.frame.len() {
            return Err(ExchangeError::PartialSend);
        }
        if after < before {
            return Err(ExchangeError::CaptureOrder);
        }
        if after >= cutoff {
            return Err(ExchangeError::Expired);
        }
        let response_cutoff: Option<Instant> = response_wait
            .and_then(|wait| after.checked_add(wait))
            .filter(|instant| *instant < cutoff);
        sent.capture.after = after;
        *phase = Phase::Awaiting;
        Ok(Self {
            authority,
            sent: Some(sent),
            cutoff,
            response_cutoff,
            reads_left: RECEIVE_CAP,
        })
    }

    /// Poll one nonblocking read and authenticate the first complete selected answer.
    ///
    /// An empty socket returns `Ok(None)` only while another read remains
    /// available before the cutoff. Every attempt charges the read budget.
    /// Any other native outcome spends the original socket and request.
    ///
    /// # Errors
    /// Refuses expired, spent, exhausted, withdrawn, foreign or malformed
    /// input, as well as native failure and failed signature verification.
    pub fn try_receive(&mut self) -> Result<Option<StagedReceipt<'a>>, ExchangeError> {
        let mut frame: [u8; RECEIVE_BYTES] = [0; RECEIVE_BYTES];
        let mut phase: MutexGuard<'_, Phase> = self.authority.phase()?;
        if let Err(error) = require_phase(*phase, Phase::Awaiting) {
            drop(phase);
            drop(self.sent.take());
            return Err(error);
        }
        let sent: &Sent<'a> = self.sent.as_ref().ok_or(ExchangeError::Spent)?;
        let before: Instant = Instant::now();
        if before >= self.cutoff || self.reads_left == 0 {
            *phase = Phase::Spent;
            drop(phase);
            drop(self.sent.take());
            return Err(if before >= self.cutoff {
                ExchangeError::Expired
            } else {
                ExchangeError::ReceiveBudget
            });
        }
        self.reads_left -= 1;
        let received: Result<(usize, SocketAddr), io::Error> = sent.socket.recv_from(&mut frame);
        let after: Instant = Instant::now();
        if received
            .as_ref()
            .is_err_and(|error| error.kind() == io::ErrorKind::WouldBlock)
        {
            let error: ExchangeError = if after < before {
                ExchangeError::CaptureOrder
            } else if after >= self.cutoff {
                ExchangeError::Expired
            } else if self.response_cutoff.is_some_and(|cutoff| before >= cutoff) {
                ExchangeError::Unanswered
            } else if self.reads_left == 0 {
                ExchangeError::ReceiveBudget
            } else {
                return Ok(None);
            };
            *phase = Phase::Spent;
            drop(phase);
            drop(self.sent.take());
            return Err(error);
        }
        *phase = Phase::Verifying;
        let sent: Sent<'a> = self.sent.take().ok_or(ExchangeError::Spent)?;
        drop(phase);
        self.stage(sent, &frame, received, Capture { before, after })
            .map(Some)
    }

    /// Retire the exact socket and authenticate its one consumed datagram.
    ///
    /// # Errors
    /// Refuses a native error, foreign source, oversized response, invalid
    /// capture, elapsed cutoff, malformed frame or failed authentication.
    fn stage(
        &self,
        sent: Sent<'a>,
        frame: &[u8; RECEIVE_BYTES],
        received: Result<(usize, SocketAddr), io::Error>,
        capture: Capture,
    ) -> Result<StagedReceipt<'a>, ExchangeError> {
        drop(sent.socket);
        let (length, peer): (usize, SocketAddr) =
            received.map_err(|error| receive_error(&error))?;
        if peer != sent.request.peer {
            return Err(ExchangeError::ForeignPeer);
        }
        if length > MAX_RESPONSE_BYTES {
            return Err(ExchangeError::Truncated);
        }
        if self
            .response_cutoff
            .is_some_and(|cutoff| capture.after >= cutoff)
        {
            return Err(ExchangeError::Expired);
        }
        let mut staged: StagedReceipt<'a> = StagedReceipt::verify(
            self.authority,
            self.cutoff,
            &sent.request,
            sent.capture,
            frame,
            length,
            capture,
        )?;
        staged.retry = sent.retry;
        Ok(staged)
    }
}

/// Sample a bounded original deadline.
///
/// # Errors
/// Refuses a zero, over-limit or unrepresentable budget.
fn exchange_cutoff(budget: Duration) -> Result<Instant, ExchangeError> {
    if budget.is_zero() || budget > MAX_TIMEOUT {
        return Err(ExchangeError::Configuration);
    }
    Instant::now()
        .checked_add(budget)
        .ok_or(ExchangeError::Configuration)
}

/// Charge the original authority and bind freshly generated request bytes.
///
/// # Errors
/// Refuses unavailable, elapsed or spent authority, or entropy failure.
fn prepare<'a>(
    authority: &RoundAuthority,
    peer: SocketAddr,
    root: &'a RootKey,
    cutoff: Instant,
    history: &[Receipt],
    transport: ExchangeTransport,
) -> Result<OriginalRequest<'a>, ExchangeError> {
    {
        let mut phase: MutexGuard<'_, Phase> = authority.phase()?;
        require_phase(*phase, Phase::Ready)?;
        if Instant::now() >= cutoff {
            return Err(ExchangeError::Expired);
        }
        *phase = Phase::Preparing;
    }
    let mut random: [u8; 32] = [0; 32];
    getrandom::fill(&mut random).map_err(|_| ExchangeError::Entropy)?;
    let nonce: [u8; 32] = chained_nonce(history, &random)?;
    Ok(OriginalRequest {
        peer,
        root,
        transport,
        frame: request::encode(root, &nonce),
        chaining_randomness: (!history.is_empty()).then_some(random),
    })
}

/// Bind one draw to the actual predecessor and the retained linking values.
///
/// # Errors
/// Refuses a repeated linking value before encoding or sending a successor.
fn chained_nonce(history: &[Receipt], random: &[u8; 32]) -> Result<[u8; 32], ExchangeError> {
    if history
        .iter()
        .any(|receipt| receipt.chaining_randomness() == Some(random))
    {
        return Err(ExchangeError::Entropy);
    }
    Ok(history.last().map_or(*random, |receipt| {
        truncated_hash(&[receipt.response_frame(), random])
    }))
}

impl<'a> StagedReceipt<'a> {
    /// Verify a complete original response before its one possible publication.
    ///
    /// # Errors
    /// Refuses invalid capture order, the elapsed cutoff, or failed authentication.
    fn verify(
        authority: &'a RoundAuthority,
        cutoff: Instant,
        request: &OriginalRequest<'_>,
        sent: Capture,
        frame: &[u8; RECEIVE_BYTES],
        length: usize,
        capture: Capture,
    ) -> Result<Self, ExchangeError> {
        if capture.before < sent.after || capture.after < capture.before {
            return Err(ExchangeError::CaptureOrder);
        }
        if capture.after >= cutoff {
            return Err(ExchangeError::Expired);
        }
        let response: &[u8] = frame.get(..length).ok_or(ExchangeError::Truncated)?;
        let verified: VerifiedResponse<'_, '_> = PendingRequest::new(&request.frame, request.root)
            .and_then(|pending| pending.verify(response))
            .map_err(ExchangeError::Verification)?;
        let bounds: (u64, u64) = verified.bounds_seconds();
        let root_key: [u8; 32] = *verified.root_key_bytes();
        Ok(StagedReceipt {
            authority,
            cutoff,
            retry: None,
            receipt: Receipt {
                peer: request.peer,
                root_key,
                transport: request.transport,
                request: request.frame,
                chaining_randomness: request.chaining_randomness,
                response: *frame,
                length,
                bounds,
                sent,
                received: capture,
            },
        })
    }
}

/// Bind failure information to the actual native call.
fn socket_error(error: &io::Error) -> ExchangeError {
    ExchangeError::Socket(IoCause::capture(error))
}

/// Recognize the Winsock consumed-oversize result as a truncation.
fn receive_error(error: &io::Error) -> ExchangeError {
    if cfg!(windows) && error.raw_os_error() == Some(10040) {
        ExchangeError::Truncated
    } else {
        socket_error(error)
    }
}

/// A checked response awaiting one current-authority publication.
///
/// External consumers cannot duplicate this publication opportunity.
///
/// ```compile_fail,E0382
/// use nevrotick_engine::roughtime::exchange::StagedReceipt;
/// fn twice(staged: StagedReceipt<'_>) {
///     drop(staged.publish());
///     drop(staged.publish());
/// }
/// ```
#[must_use = "publish the receipt only through its original authority"]
pub struct StagedReceipt<'a> {
    /// The original authority, shared with its current withdrawal owner.
    authority: &'a RoundAuthority,
    /// The send's original exclusive cutoff.
    cutoff: Instant,
    /// Verified data held until current publication succeeds.
    receipt: Receipt,
    /// Original scheduler custody until a current signed publication.
    retry: Option<RetryLease<'a>>,
}

impl<'a> StagedReceipt<'a> {
    /// Recheck authority and current time under the joined publication guard.
    ///
    /// # Errors
    /// Refuses withdrawn or unavailable authority, an out-of-order current
    /// observation, or a publication at or beyond the original cutoff.
    pub fn publish(self) -> Result<Receipt, ExchangeError> {
        self.publish_with(|receipt: Receipt, retry: Option<RetryLease<'_>>| {
            if let Some(retry) = retry {
                retry.authenticated();
            }
            receipt
        })
    }

    /// Transfer the current signed leg and original lease to its round owner.
    ///
    /// # Errors
    /// Returns the original staged publication refusal.
    pub(super) fn publish_tentative(
        self,
    ) -> Result<(Receipt, Option<RetryLease<'a>>), ExchangeError> {
        self.publish_with(|receipt: Receipt, retry: Option<RetryLease<'a>>| (receipt, retry))
    }

    /// Commit one staged result under its original current authority.
    ///
    /// # Errors
    /// Refuses withdrawn, stale or out-of-order original evidence.
    fn publish_with<T>(
        self,
        commit: impl FnOnce(Receipt, Option<RetryLease<'a>>) -> T,
    ) -> Result<T, ExchangeError> {
        let mut phase: MutexGuard<'_, Phase> = self.authority.phase()?;
        require_phase(*phase, Phase::Verifying)?;
        *phase = Phase::Spent;
        let now: Instant = Instant::now();
        if now >= self.cutoff {
            return Err(ExchangeError::Expired);
        }
        if now < self.receipt.received.after {
            return Err(ExchangeError::CaptureOrder);
        }
        Ok(commit(self.receipt, self.retry))
    }
}

/// The actual transport that carried one authenticated exchange.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExchangeTransport {
    /// One datagram request and response.
    Udp,
    /// One request and response on a stream.
    Tcp,
}

/// Historical evidence from one authenticated response, with original packet custody.
#[must_use = "retain the signed evidence with its original frame and captures"]
pub struct Receipt {
    /// Selected and observed source address of the authenticated response.
    peer: SocketAddr,
    /// Actual transport of both original frames.
    transport: ExchangeTransport,
    /// Long-term key selected before the request.
    root_key: [u8; 32],
    /// Original complete request, including framing and padding.
    request: [u8; request::REQUEST_BYTES],
    /// Random value paired with the previous complete response, if chained.
    chaining_randomness: Option<[u8; 32]>,
    /// Owned native receive storage.
    response: [u8; RECEIVE_BYTES],
    /// Checked length of the complete verified response within that storage.
    length: usize,
    /// Open signed interval in Unix-epoch seconds.
    bounds: (u64, u64),
    /// Original native send bracket.
    sent: Capture,
    /// Original native receive bracket.
    received: Capture,
}

impl Receipt {
    /// Return the peer selected and observed at the actual receive boundary.
    #[must_use]
    pub const fn peer(&self) -> SocketAddr {
        self.peer
    }

    /// Return the transport of the original request and response.
    #[must_use]
    pub const fn transport(&self) -> ExchangeTransport {
        self.transport
    }

    /// Borrow the exact root used for verification.
    #[must_use]
    pub const fn root_key_bytes(&self) -> &[u8; 32] {
        &self.root_key
    }

    /// Borrow the complete transmitted request.
    #[must_use]
    pub const fn request_frame(&self) -> &[u8] {
        &self.request
    }

    /// Borrow the exact random chain input for this request, if it had a predecessor.
    #[must_use]
    pub const fn chaining_randomness(&self) -> Option<&[u8; 32]> {
        self.chaining_randomness.as_ref()
    }

    /// Borrow the complete verified response for a later nonce chain.
    #[must_use]
    pub fn response_frame(&self) -> &[u8] {
        &self.response[..self.length]
    }

    /// Return the signed open interval endpoints in Unix-epoch seconds.
    #[must_use]
    pub const fn bounds_seconds(&self) -> (u64, u64) {
        self.bounds
    }

    /// Return the actual monotonic send and receive brackets, in that order.
    #[must_use]
    pub const fn captures(&self) -> ((Instant, Instant), (Instant, Instant)) {
        (
            (self.sent.before, self.sent.after),
            (self.received.before, self.received.after),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Capture, ExchangeError, ExchangeTransport, RECEIVE_BYTES, Receipt, chained_nonce, request,
    };
    use std::{net::SocketAddr, time::Instant};

    /// Supply literal history for the pure nonce calculation without performing I/O.
    fn observation(link: [u8; 32], body: [u8; 2], at: Instant) -> Receipt {
        let mut response: [u8; RECEIVE_BYTES] = [0; RECEIVE_BYTES];
        response[..2].copy_from_slice(&body);
        let capture: Capture = Capture {
            before: at,
            after: at,
        };
        Receipt {
            peer: SocketAddr::from(([127, 0, 0, 1], 1)),
            transport: ExchangeTransport::Udp,
            root_key: [0; 32],
            request: [0; request::REQUEST_BYTES],
            chaining_randomness: Some(link),
            response,
            length: 2,
            bounds: (0, 2),
            sent: capture,
            received: capture,
        }
    }

    /// Reject a nonadjacent repeated input and hash a fresh input with the actual predecessor.
    ///
    /// # Errors
    /// Returns an unexpected refusal of a fresh fixture link.
    ///
    /// # Panics
    /// Panics if the earlier linking value is reused or the fresh hash differs.
    #[test]
    fn refuses_nonadjacent_repeated_linking_randomness() -> Result<(), &'static str> {
        let elapsed: Instant = Instant::now();
        let earlier: [u8; 32] = [0x41; 32];
        let fresh: [u8; 32] = [0x42; 32];
        let history: [Receipt; 2] = [
            observation(earlier, *b"XX", elapsed),
            observation([0x43; 32], *b"RT", elapsed),
        ];
        assert!(matches!(
            chained_nonce(&history, &earlier),
            Err(ExchangeError::Entropy)
        ));
        let nonce: [u8; 32] =
            chained_nonce(&history, &fresh).map_err(|_| "fresh fixture draw refused")?;
        // Independent SHA-512 known answer for RT followed by 32 bytes of 0x42.
        assert_eq!(
            nonce,
            [
                0x8a, 0xab, 0x10, 0x82, 0x2a, 0xa1, 0xe0, 0xb5, 0x86, 0x34, 0xc2, 0x8b, 0xa4, 0x43,
                0x49, 0x7b, 0xb7, 0x3b, 0xa1, 0xe8, 0x1b, 0x9f, 0x31, 0x4d, 0x32, 0xad, 0x5e, 0x62,
                0x15, 0x77, 0xac, 0xc4,
            ]
        );
        Ok(())
    }
}

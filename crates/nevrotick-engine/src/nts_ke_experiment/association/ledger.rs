// SPDX-License-Identifier: GPL-2.0-only
//! Original-context cookie, request and receive-work custody.
//!
//! The sole normal issuer is the parent association's actual KE receiver. This
//! ledger owns archive and reply custody. The closed datagram owner alone
//! performs the synchronous send and retains its actual return observations.

use std::io;
use std::net::SocketAddr;
use std::num::NonZeroU64;
use std::sync::Arc;
use std::time::Instant;

use ntp_wire::{FrameError, NtpTimestamp, NtsRequestEncodeError, RawHeader, decode_fixed_header};
use zeroize::Zeroizing;

#[cfg(test)]
use super::super::datagram::{Attempts, RetiredProtectedControl};
#[cfg(any(windows, test))]
use super::super::datagram::{GuardedSender, RetiredReply, RetiredReplyParts};
use super::super::handoff::{self, PreparedRequest};
use super::super::request::{DirectionalKeys, RequestError};
#[cfg(test)]
use super::super::response::{
    ProtectedControlKind, StagedSuccessorResponse, stage_successor_response,
};
use super::super::response::{ReplyError, ReplyFields, StagedReply, stage_reply};
use super::{AssociationOwner, COOKIE_BYTES_LIMIT, COOKIE_LIMIT, Control, Failure};
use crate::SoftwareClock;
use crate::nts_ke_experiment::DatagramAuthorityGuard;
use crate::source_policy::{ProtectedEndpoint, ProtectedSendStamp};

pub(in super::super) mod retirement;

/// Nonrenewable checkout and entropy attempts in one original context.
const ATTEMPTS: usize = 32;
/// Nonrenewable datagram work in that same original context.
const FRAMES: usize = 32;
/// Aggregate raw received octets, including rejected datagrams.
const RECEIVE_BYTES: usize = 131_072;
/// One bounded private request and one current datagram.
const PACKET_BYTES: usize = 4096;

/// Owned opaque cookie entries with zeroization on retirement.
type CookiePool = Vec<Zeroizing<Vec<u8>>>;

/// Closed redacted ledger refusals; no variant owns packet or secret bytes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in super::super) enum LedgerError {
    /// Another ticket or unresolved invocation still owns the slot.
    Busy,
    /// The original checkout allowance is exhausted.
    Attempts,
    /// No never-used cookie remains.
    Cookies,
    /// The ticket belongs to another association or retired request.
    Ticket,
    /// The requested operation is unavailable in the current phase.
    Phase,
    /// An entropy/preparation attempt has already begun.
    PreparationSpent,
    /// The entropy owner repeated an already issued UID.
    RepeatedUid,
    /// The one observed entropy batch supplied an unusable all-zero wire token.
    ZeroWireToken,
    /// An unsendable laboratory preview cannot enter a sender.
    Preview,
    /// A synchronous adapter returned a short or otherwise wrong byte count.
    ShortSend,
    /// Preserve the adapter's bounded original error category.
    Send(io::ErrorKind),
    /// A component reports no completion; its occupied slot remains unavailable.
    #[cfg(test)]
    Unknown,
    /// The original aggregate receive allowance is exhausted.
    ReceiveWork,
    /// The reply came from another endpoint.
    Endpoint,
    /// Arrival/processing does not fit the original request time interval.
    Time,
    /// A checked bounded internal storage operation cannot be completed.
    Capacity,
    /// Preserve the maintained response consumer's redacted refusal.
    Reply(ReplyError),
}

/// Explicitly distinct preview and packet correlation inputs.
#[derive(Clone, Copy)]
pub(in super::super) enum RequestInput {
    /// Zero-T1 laboratory packet; no sender may expose its bytes.
    Preview,
    /// The concrete adapter supplies actual captures; preparation issues only wire correlation.
    Observed,
    /// Save the random wire token separately from the declared local T1.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "EP2 protected-datagram adapter remains held; packet input is exercised by component tests"
        )
    )]
    Packet {
        /// Nonzero wire correlation bits, never interpreted as local time.
        wire: NonZeroU64,
        /// Separately supplied local timestamp; no clock provenance is inferred.
        local_t1: NtpTimestamp,
    },
}

/// One same-Control request capability. Drop/forget cannot release its slot.
pub(in super::super) struct Ticket<'a> {
    /// Borrow the actual association's original control.
    control: &'a Control,
    /// Monotonic within the finite original-context checkout allowance.
    serial: usize,
}

/// Synchronous component-adapter outcomes, never native-completion evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in super::super) enum SendOutcome {
    /// The synchronous adapter returned a completed local byte count.
    Completed(usize),
    /// The synchronous adapter returned this original bounded error.
    Failed(io::ErrorKind),
    /// A component reports no completion; custody and Busy remain sticky.
    #[cfg(test)]
    Unknown,
}

/// Observable logical phases; none asserts native completion or freshness.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in super::super) enum RequestPhase {
    /// No request owns the logical slot.
    Idle,
    /// Cookie reserved; no entropy call has begun.
    Reserved,
    /// One preparation attempt has begun and cannot be repeated.
    Preparing,
    /// Private packet exists, with no permission for an arbitrary observer.
    Prepared,
    /// Cookie archived before adapter exposure; completion is unresolved.
    Invoked,
    /// Exact local submission returned; a bounded reply may be admitted.
    Pending,
}

/// Bounded recovery reason; it grants no KE invocation or fallback.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in super::super) enum RecoveryReason {
    /// No never-used cookie remains.
    Cookies,
    /// The original checkout allowance is spent.
    Attempts,
    /// The original receive allowance is spent.
    ReceiveWork,
}

/// One coalesced, consumable intent tied to the original expiry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in super::super) struct RecoveryIntent {
    /// No earlier retry/context is authorized by this inert intent.
    not_before: Instant,
    /// Fixed redacted reason for the first recovery need.
    reason: RecoveryReason,
}

impl RecoveryIntent {
    /// Return the unchanged original expiry, without scheduling work.
    pub(in super::super) const fn not_before(self) -> Instant {
        self.not_before
    }

    /// Return the first coalesced bounded reason.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "EP2 recovery scheduling consumer remains held; component tests inspect the inert intent"
        )
    )]
    pub(in super::super) const fn reason(self) -> RecoveryReason {
        self.reason
    }
}

/// Historical packet admission only; no time-sample or freshness authority.
#[derive(Clone, Copy, Eq, PartialEq)]
pub(in super::super) struct Receipt {
    /// Original finite request identity.
    serial: usize,
    /// Optional declared component T1; actual adapter captures are retained separately.
    local_t1: Option<NtpTimestamp>,
    /// Authenticated inert header fields from the maintained consumer.
    fields: ReplyFields,
    /// Fresh original-context processing observation at packet admission.
    accepted_at: Instant,
    /// Number of encrypted Cookie fields, including unusable repeated values.
    received_cookies: usize,
    /// Incoming entries eligible when admitted, excluding used values.
    retained_cookies: usize,
}

impl Receipt {
    /// Expose bounded historical request identity only.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "EP2 production receipt consumer remains held; component tests inspect historical metadata"
        )
    )]
    pub(in super::super) const fn serial(self) -> usize {
        self.serial
    }

    /// Preserve only a declared component T1; observed capture provenance stays separate.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "EP2 production receipt consumer remains held; component tests inspect historical metadata"
        )
    )]
    pub(in super::super) const fn local_t1(self) -> Option<NtpTimestamp> {
        self.local_t1
    }

    /// Preserve authenticated inert fields without time eligibility.
    pub(in super::super) const fn fields(self) -> ReplyFields {
        self.fields
    }

    /// Return the original publication observation, never a renewed deadline.
    pub(in super::super) const fn accepted_at(self) -> Instant {
        self.accepted_at
    }

    /// Return received and retained entry counts, not distinct issuable values.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "EP2 production receipt consumer remains held; component tests inspect historical metadata"
        )
    )]
    pub(in super::super) const fn cookie_counts(self) -> (usize, usize) {
        (self.received_cookies, self.retained_cookies)
    }
}

/// Secret-free inspection of current finite custody and allowances.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in super::super) struct Status {
    /// Logical phase; Invoked keeps unresolved custody occupied.
    pub(in super::super) phase: RequestPhase,
    /// Remaining original-context checkouts, never refunded.
    pub(in super::super) attempts: usize,
    /// Remaining charged datagrams, never renewed by another request.
    pub(in super::super) frames: usize,
    /// Remaining charged raw bytes.
    pub(in super::super) bytes: usize,
    /// Distinct unused values in the pool; packet encodability is checked later.
    pub(in super::super) available: usize,
    /// Known possibly-sent bodies, never evicted in this context.
    pub(in super::super) used: usize,
    /// All issued UIDs, including the current UID exactly once.
    pub(in super::super) uids: usize,
}

/// Private packet variants keep preview bytes out of the sender boundary.
enum ReadyPacket<'a> {
    /// The existing opaque never-send result, with its own ordinary wipe owner.
    Preview(PreparedRequest<'a>),
    /// One packet owned through invocation and actual synchronous return.
    Packet {
        /// Entire original storage, including the unused suffix, is wiped.
        storage: Zeroizing<[u8; PACKET_BYTES]>,
        /// Maintained formatter's exact complete packet length.
        length: usize,
    },
}

/// Fully checked replacement storage, still inert until the final owner guard.
struct StagedReplacement {
    /// Authenticated raw coordinates from the maintained response consumer.
    fields: ReplyFields,
    /// Bounded replacement withheld from the live pool.
    replacement: CookieReplacement,
    /// All encrypted Cookie fields, including known-used values.
    received_cookies: usize,
    /// Never-used incoming values counted under the original archive.
    retained: usize,
}

/// Checked cookie transfer from the original selected reply, with no send authority.
pub(in super::super) struct CookieReplacement {
    /// Complete owned incoming population, wiped on every ordinary refusal.
    cookies: CookiePool,
    /// Empty destination with room for the complete next bounded pool.
    next: CookiePool,
    /// Checked retention of each original pool entry.
    retain_existing: [bool; COOKIE_LIMIT],
    /// Checked retention of each incoming entry in packet order.
    retain_incoming: [bool; COOKIE_LIMIT],
    /// Checked resulting entry count and aggregate opaque byte length.
    population: (usize, usize),
}

impl CookieReplacement {
    /// The checked counts describe the resulting pool if this plan commits.
    pub(in super::super) const fn population(&self) -> (usize, usize) {
        self.population
    }
}

/// Original adapter custody transferred only by a genuinely submitted request.
#[must_use = "the retired reply must retain the original submission"]
pub(in super::super) struct PendingSubmission {
    /// The allocation owned by the actual socket or Script backend.
    identity: Arc<()>,
    /// Present only when the same sender charged managed source and endpoint floors.
    stamp: Option<ProtectedSendStamp>,
}

/// Select how a genuine retired reply completes its original Control.
#[cfg_attr(not(test), derive(Clone, Copy))]
pub(in super::super) enum RetiredCommit {
    /// The ordinary one-shot reply is also terminal for the invocation.
    Terminal,
    /// The private two-request fixture keeps the same invocation alive.
    #[cfg(test)]
    FirstRequest(Attempts),
}

/// First authenticated request committed on this exact managed association.
#[cfg(test)]
struct PriorAuthentication {
    /// Exact original packet receipt and charged ordinal.
    receipt: Receipt,
    /// The first request's exact backend identity and policy epochs.
    submission: PendingSubmission,
    /// Original driver allowance, consumed once before entering the successor.
    attempts: Option<Attempts>,
}

/// Concrete submissions and the diagnostic callback are mutually exclusive.
enum SubmissionOrigin {
    /// The concrete backend's identity and its managed policy stamp, if required.
    Adapter(PendingSubmission),
    /// The component ledger's diagnostic path has no concrete backend.
    #[cfg(test)]
    Component,
}

impl SubmissionOrigin {
    /// A receive observation belongs only to the adapter that submitted this request.
    fn matches(&self, observed: Option<&Arc<()>>) -> bool {
        match (self, observed) {
            (Self::Adapter(submission), Some(backend)) => {
                Arc::ptr_eq(&submission.identity, backend)
            }
            #[cfg(test)]
            (Self::Component, None) => true,
            _ => false,
        }
    }

    /// Borrow the original pair for the current managed receive guard.
    fn policy_provenance(&self) -> Option<(&ProtectedSendStamp, &Arc<()>)> {
        match self {
            Self::Adapter(submission) => submission.policy_provenance(),
            #[cfg(test)]
            Self::Component => None,
        }
    }
}

impl PendingSubmission {
    /// Borrow the exact issued stamp and its original identity as one pairing.
    fn policy_provenance(&self) -> Option<(&ProtectedSendStamp, &Arc<()>)> {
        self.stamp
            .as_ref()
            .map(|stamp: &ProtectedSendStamp| (stamp, &self.identity))
    }

    /// Observe the identity through the sole owning submission before commit.
    #[cfg(test)]
    pub(in super::super) const fn identity(&self) -> &Arc<()> {
        &self.identity
    }
}

/// One occupied reservation. It remains in the ledger when a ticket is dropped.
struct Active<'a> {
    /// Exact finite identity, checked against the non-Clone ticket.
    serial: usize,
    /// Fixed input kind/correlation, unchanged across all transitions.
    input: RequestInput,
    /// Moved exact cookie until archived or explicitly returned.
    cookie: Option<Zeroizing<Vec<u8>>>,
    /// State owner for the irreversible preparation/invocation transitions.
    phase: RequestPhase,
    /// UID history position; no second live UID copy is retained.
    uid: Option<usize>,
    /// Checked nonzero wire token issued only by observed preparation.
    observed_wire: Option<NonZeroU64>,
    /// One private complete packet, absent on preparation failure.
    ready: Option<ReadyPacket<'a>>,
    /// Recorded request-bound response allowance.
    maximum_cookies: usize,
    /// Fresh final invocation observation; no peer-delivery claim follows.
    sent_at: Option<Instant>,
    /// Actual submission attached only when the full result makes this request Pending.
    submission: Option<SubmissionOrigin>,
}

/// The unvalidated readings of one actual receive-return bracket.
/// Copying them carries no current authority.
#[derive(Clone, Copy)]
struct ReceiveSpan {
    /// Reading before the original wall-clock capture.
    leading: Instant,
    /// Reading after the original wall-clock capture.
    trailing: Instant,
}

impl ReceiveSpan {
    /// Check the original bracket against the send, processing and exclusive cutoff.
    /// The caller checks current Control and its processing deadline separately.
    /// # Errors
    /// Preserve the leading time refusal before trailing cutoff and ordering refusals.
    fn check(self, sent_at: Instant, now: Instant, expiry: Instant) -> Result<(), Failure> {
        check_receive_times(sent_at, self.leading, now)?;
        if self.trailing >= expiry {
            return Err(Failure::Deadline);
        }
        if self.trailing < self.leading || self.trailing > now {
            return Err(refused(LedgerError::Time));
        }
        Ok(())
    }

    /// Preserve both readings without sampling or repairing their order.
    fn from_read(read: &SoftwareClock) -> Self {
        Self {
            leading: read.monotonic(),
            trailing: read.monotonic_after(),
        }
    }
}

/// Coordinates captured from the same Pending slot as its receive-work debit.
struct ChargedReply {
    /// Exact still-outstanding reservation ordinal.
    serial: usize,
    /// Position of the original issued UID.
    uid_index: usize,
    /// Exact previous wire transmit token.
    wire: NonZeroU64,
    /// Declared T1 for the diagnostic component path.
    local_t1: Option<NtpTimestamp>,
    /// Original monotonic sender observation.
    sent_at: Instant,
    /// Request-bound response cookie limit.
    maximum_cookies: usize,
    /// The receiving backend is the exact former submitting backend.
    matches_submission: bool,
}

/// One charged, authenticated control from the exact still-outstanding successor.
/// The transfer itself cannot change policy or consume the Pending slot.
#[cfg(test)]
#[must_use = "retain the exact control ticket through backend retirement"]
pub(in super::super) struct StagedProtectedControl<'control> {
    /// Exact authenticated wire code.
    kind: ProtectedControlKind,
    /// Actual original invocation the packet reached.
    control: &'control Control,
    /// Exact successor reservation ordinal.
    serial: usize,
    /// Original outstanding Unique Identifier archive position.
    uid_index: usize,
    /// Exact outstanding request origin token.
    wire: NonZeroU64,
    /// Adapter allocation that submitted this original Pending request.
    backend: Arc<()>,
    /// Original monotonic send/archive observation.
    sent_at: Instant,
    /// Leading reading of the original selected receive bracket.
    arrival: Instant,
    /// Trailing reading of that same bracket, never refreshed by retirement.
    read_after: Instant,
}

/// The protected control already published through both original owners.
#[cfg(test)]
#[must_use = "retain the applied control as a historical observation"]
pub(in super::super) struct AppliedProtectedControl {
    /// Exact authenticated control already applied to the managed source and endpoint.
    kind: ProtectedControlKind,
    /// Original fully admitted managed definition, detached before the effect.
    configuration: Box<crate::SourceConfigurationSnapshot>,
    /// Exact negotiated wire destination of the consumed successor.
    remote: SocketAddr,
    /// Exact successor ordinal consumed by the effect.
    serial: usize,
    /// Original currentness observation used for the committed decision.
    decided_at: Instant,
}

#[cfg(test)]
impl AppliedProtectedControl {
    /// Observe the original control without interpreting it as time or permission.
    pub(in super::super) const fn kind(&self) -> ProtectedControlKind {
        self.kind
    }

    /// Borrow the original source definition without reopening policy.
    pub(in super::super) fn configuration(&self) -> &crate::SourceConfigurationSnapshot {
        &self.configuration
    }

    /// Read the original wire destination without granting a source binding.
    pub(in super::super) const fn remote(&self) -> SocketAddr {
        self.remote
    }

    /// Observe the committed ordinal without opening a new request.
    pub(in super::super) const fn serial(&self) -> usize {
        self.serial
    }

    /// Observe the exact historical decision independently of future policy.
    pub(in super::super) const fn decided_at(&self) -> Instant {
        self.decided_at
    }
}

/// The actual distinct historical and outstanding protected send proofs.
#[cfg(test)]
struct ControlSubmissions<'a> {
    /// Send evidence retained by the previously authenticated request.
    prior: (&'a ProtectedSendStamp, &'a Arc<()>),
    /// Exact current outstanding successor submission.
    pending: (&'a ProtectedSendStamp, &'a Arc<()>),
}

/// One ledger-verified retired control for its original authority transaction.
/// Only the retired receiver can mint it; current policy is checked at consumption.
#[cfg(test)]
pub(crate) struct VerifiedRetiredControl<'a> {
    /// Exact authenticated wire control owned by the retired request.
    kind: ProtectedControlKind,
    /// Original invocation matched when the retired ticket was consumed.
    control: &'a Control,
    /// Both exact send proofs from the same association.
    submissions: ControlSubmissions<'a>,
    /// Original monotonic decision observation checked against receive time.
    decided_at: Instant,
}

#[cfg(test)]
impl<'a> VerifiedRetiredControl<'a> {
    /// Read the authenticated control kind carried by this move-only authorization.
    pub(crate) const fn kind(&self) -> ProtectedControlKind {
        self.kind
    }

    /// Match the actual original invocation without transferring its authority.
    pub(in super::super) fn matches_control(&self, control: &Control) -> bool {
        std::ptr::eq(self.control, control)
    }

    /// Return the same checked decision observation to both authority guards.
    pub(crate) const fn decided_at(&self) -> Instant {
        self.decided_at
    }

    /// Consume the authorization and expose the original prior and Pending stamps.
    pub(crate) fn into_submissions(self) -> [(&'a ProtectedSendStamp, &'a Arc<()>); 2] {
        [self.submissions.prior, self.submissions.pending]
    }
}

/// The exact charged successor admitted by the shared protected receiver.
#[cfg(test)]
pub(in super::super) enum StagedProtectedSuccessor<'control> {
    /// The ordinary reply has consumed its Pending submission and produced history.
    Ordinary(Receipt, CookieReplacement, PendingSubmission),
    /// The control still requires the original owner's final currentness transaction.
    Control(StagedProtectedControl<'control>),
}

/// Actual final authenticated count refusal, still requiring current-time admission.
#[derive(Clone, Copy)]
enum ExhaustedResponse {
    /// Saved local submission and actual caller observation remain distinct.
    Cookies {
        /// Original monotonic observation taken before invoking the sender.
        sent_at: Instant,
        /// Original receive bracket, rechecked by consuming retirement.
        span: ReceiveSpan,
    },
}

/// Original-context state; the parent creates it before publishing KE.
pub(in super::super) struct LedgerState<'a> {
    /// One reserved/prepared/invoked/pending request, never inferred from Drop.
    active: Option<Active<'a>>,
    /// Whole original checkout allowance.
    attempts: usize,
    /// Whole original datagram allowance.
    frames: usize,
    /// Whole original raw-byte allowance.
    bytes: usize,
    /// Moved possibly-sent values; capacity is established before publication.
    used: Vec<Zeroizing<Vec<u8>>>,
    /// Issued UIDs, including the current one, with ordinary zeroization.
    uids: Zeroizing<[[u8; 32]; ATTEMPTS]>,
    /// Checked initialized UID prefix; retirement never decreases it.
    uid_count: usize,
    /// Last accepted receipt; failed attempts never replace it.
    last: Option<Receipt>,
    /// Only the private first-request retired commit can create this witness.
    #[cfg(test)]
    prior_authentication: Option<PriorAuthentication>,
    /// The first coalesced intent, available for one explicit take.
    recovery: Option<RecoveryIntent>,
    /// Sticky even after the intent is taken; repeated calls cannot mint more.
    recovery_issued: bool,
    /// One exact predecessor; it cannot enlarge this context's own allowances.
    previous: Option<retirement::PastRequests<'a>>,
    /// Produced only by the real final authenticated cookie-count refusal.
    exhausted_response: Option<ExhaustedResponse>,
}

impl LedgerState<'_> {
    /// Preallocate only private archive metadata before KE publication.
    ///
    /// The original cookie Vec is neither borrowed nor reallocated here.
    /// # Errors
    /// Preserve bounded allocation refusal before publishing an association.
    pub(in super::super) fn new() -> Result<Self, Failure> {
        let mut used = Vec::new();
        used.try_reserve_exact(ATTEMPTS)
            .map_err(|_| Failure::Allocation)?;
        Ok(Self {
            active: None,
            attempts: ATTEMPTS,
            frames: FRAMES,
            bytes: RECEIVE_BYTES,
            used,
            uids: Zeroizing::new([[0; 32]; ATTEMPTS]),
            uid_count: 0,
            last: None,
            #[cfg(test)]
            prior_authentication: None,
            recovery: None,
            recovery_issued: false,
            previous: None,
            exhausted_response: None,
        })
    }

    /// Inspect both immutable predecessor and local possibly-sent cookie history.
    /// This grants no reservation, return or permission to expose packet bytes.
    fn cookie_was_used(&self, body: &[u8]) -> bool {
        was_used(&self.used, body)
            || self
                .previous
                .as_ref()
                .is_some_and(|past| past.cookie_was_used(body))
    }

    /// Inspect issued UID prefixes without retiring a new value or drawing entropy.
    fn uid_was_issued(&self, uid: &[u8; 32]) -> bool {
        self.uids.iter().take(self.uid_count).any(|old| old == uid)
            || self
                .previous
                .as_ref()
                .is_some_and(|past| past.uid_was_issued(uid))
    }
}

/// Compare exact opaque values only against this context's possibly-sent archive.
fn was_used(used: &[Zeroizing<Vec<u8>>], body: &[u8]) -> bool {
    used.iter().any(|value| value.as_slice() == body)
}

/// Preserve one bounded failure without formatting input or secret bytes.
const fn refused(error: LedgerError) -> Failure {
    Failure::Ledger(error)
}

/// Check only the inclusive ordering of three supplied monotonic observations.
///
/// This pure check supplies no clock provenance, current authority or expiry.
/// The actual receiving owner supplies fresh processing time inside its final
/// Control guard and retains its separate deadline/currentness checks.
/// # Errors
/// Refuse arrival before send or after processing, without changing any owner.
fn check_receive_times(
    sent_at: Instant,
    arrival: Instant,
    processing_now: Instant,
) -> Result<(), Failure> {
    if arrival < sent_at || arrival > processing_now {
        return Err(refused(LedgerError::Time));
    }
    Ok(())
}

/// Try at most eight immutable placeholder counts with one entropy value.
///
/// Descending counts make the finite retry bound explicit without a mutable
/// decrement. The maintained formatter rejects capacity before encryption, so
/// only the first fitting layout can enter actual sealing. Any sealing or
/// non-capacity failure stops immediately. Return packet length and one plus
/// the exact encoded placeholder count; no request state or send is published.
/// # Errors
/// Refuse an allowance above seven before work. Preserve the exact formatter
/// error at zero placeholders, other formatting errors and actual seal errors.
/// A defensive exhausted-iterator refusal publishes no completed packet.
fn encode_packet(
    keys: &DirectionalKeys,
    header: &RawHeader,
    cookie: &[u8],
    entropy: &[u8; 48],
    maximum_placeholders: usize,
    storage: &mut [u8; PACKET_BYTES],
) -> Result<(usize, usize), Failure> {
    if maximum_placeholders > 7 {
        return Err(Failure::Packet(RequestError::Limits(
            FrameError::InvalidLimits,
        )));
    }
    for placeholders in (0..=maximum_placeholders).rev() {
        match keys.seal_context(header, cookie, entropy, placeholders, storage) {
            Ok(packet) => return Ok((packet.packet_bytes().len(), placeholders + 1)),
            Err(RequestError::Format(
                NtsRequestEncodeError::OutputTooSmall
                | NtsRequestEncodeError::FrameLimit(FrameError::ByteLimit),
            )) if placeholders > 0 => {}
            Err(error) => return Err(Failure::Packet(error)),
        }
    }
    Err(refused(LedgerError::Capacity))
}

impl<'a> AssociationOwner<'a> {
    /// Coalesce only a genuine checkout refusal from current owner state.
    ///
    /// Call after the Busy check but before charging the next checkout, preserving
    /// the original refusal priority. A fresh or occupied owner creates no intent.
    /// No lock is acquired: ordinary admission callers already hold Published.
    /// The inert original-expiry record grants no retry, return or renewed credit.
    fn recover_checkout(&mut self) {
        if self.ledger.active.is_some() || self.ledger.recovery_issued {
            return;
        }
        let reason = if self.ledger.attempts == 0 {
            RecoveryReason::Attempts
        } else if self.ledger.frames == 0 || self.ledger.bytes == 0 {
            RecoveryReason::ReceiveWork
        } else if !self
            .material
            .cookies
            .iter()
            .any(|body| !self.ledger.cookie_was_used(body))
        {
            RecoveryReason::Cookies
        } else {
            return;
        };
        self.ledger.recovery = Some(RecoveryIntent {
            not_before: self.material.control.expiry,
            reason,
        });
        self.ledger.recovery_issued = true;
    }

    /// Retire exhausted logical reply custody and coalesce its actual work cause.
    ///
    /// This observes only the unchanged local budgets; it grants no physical
    /// completion fact. No lock/time admission is added, so cleanup also runs
    /// after cancellation or expiry. An earlier owned intent remains unchanged.
    fn retire_exhausted(&mut self) {
        if self.ledger.frames == 0 || self.ledger.bytes == 0 {
            self.ledger.active = None;
            if !self.ledger.recovery_issued {
                self.ledger.recovery = Some(RecoveryIntent {
                    not_before: self.material.control.expiry,
                    reason: RecoveryReason::ReceiveWork,
                });
                self.ledger.recovery_issued = true;
            }
        }
    }

    /// Reserve one never-used cookie under the current original-context guard.
    ///
    /// Busy has no entropy or checkout effect. Every admitted scan/failed
    /// checkout charges one attempt. Returning a cookie cannot refund it.
    /// # Errors
    /// Preserve currentness, Busy, spent budgets and empty fresh inventory.
    pub(in super::super) fn reserve(&mut self, input: RequestInput) -> Result<Ticket<'a>, Failure> {
        match (&self.material.config.mode, input) {
            (super::super::ExchangeMode::ManagedPreview(_), _)
            | (
                super::super::ExchangeMode::ManagedProtected(_),
                RequestInput::Preview | RequestInput::Packet { .. },
            ) => Err(Failure::Control),
            _ => self.reserve_inner(input),
        }
    }

    /// Share actual reservation accounting only inside the association ledger.
    /// Managed Preview reaches this kernel only after its owner charged its allowance.
    /// # Errors
    /// Preserve published-state, slot, work and cookie refusals without refunding scans.
    fn reserve_inner(&mut self, input: RequestInput) -> Result<Ticket<'a>, Failure> {
        let control: &Control = self.material.control;
        control.with_published(|_| {
            if self.ledger.active.is_some() {
                return Err(refused(LedgerError::Busy));
            }
            self.recover_checkout();
            if self.ledger.attempts == 0 {
                return Err(refused(LedgerError::Attempts));
            }
            self.ledger.attempts -= 1;
            if self.ledger.frames == 0 || self.ledger.bytes == 0 {
                return Err(refused(LedgerError::ReceiveWork));
            }
            let ledger: &LedgerState<'a> = &self.ledger;
            self.material
                .cookies
                .retain(|body| !ledger.cookie_was_used(body));
            if self.material.cookies.is_empty() {
                return Err(refused(LedgerError::Cookies));
            }
            let cookie: Zeroizing<Vec<u8>> = self.material.cookies.remove(0);
            let serial: usize = ATTEMPTS - self.ledger.attempts;
            self.ledger.active = Some(Active {
                serial,
                input,
                cookie: Some(cookie),
                phase: RequestPhase::Reserved,
                uid: None,
                observed_wire: None,
                ready: None,
                maximum_cookies: 0,
                sent_at: None,
                submission: None,
            });
            Ok(Ticket { control, serial })
        })
    }

    /// Check actual owner provenance and the current occupied serial.
    /// # Errors
    /// Refuse foreign or retired tickets without changing either owner's state.
    fn check_ticket(&self, ticket: &Ticket<'_>) -> Result<(), Failure> {
        if !std::ptr::eq(ticket.control, self.material.control)
            || self.ledger.active.as_ref().map(|active| active.serial) != Some(ticket.serial)
        {
            return Err(refused(LedgerError::Ticket));
        }
        Ok(())
    }

    /// Bind a concrete adapter to this actual owner and current publication.
    /// # Errors
    /// Refuse a foreign control, cancellation, expiry or unpublished context.
    pub(in super::super) fn check_context(&self, control: &Control) -> Result<SocketAddr, Failure> {
        if !std::ptr::eq(control, self.material.control) {
            return Err(Failure::Control);
        }
        let guard: DatagramAuthorityGuard<'_> =
            DatagramAuthorityGuard::lock(control, self.protected_endpoint()?)?;
        let peer: SocketAddr = self.material.udp;
        guard.endpoint(peer)?;
        Ok(peer)
    }

    /// Spend one preparation attempt only for the selected request representation.
    /// # Errors
    /// Refuse foreign/spent tickets, mode mismatch or stale publication before entropy.
    fn begin_preparation(&mut self, ticket: &Ticket<'_>, observed: bool) -> Result<(), Failure> {
        self.check_ticket(ticket)?;
        self.material.control.with_published(|_| {
            let active = self
                .ledger
                .active
                .as_mut()
                .ok_or(refused(LedgerError::Ticket))?;
            if active.phase != RequestPhase::Reserved {
                return Err(refused(LedgerError::PreparationSpent));
            }
            if matches!(active.input, RequestInput::Observed) != observed {
                return Err(refused(LedgerError::Phase));
            }
            active.phase = RequestPhase::Preparing;
            Ok(())
        })
    }

    /// Acquire one UID/nonce batch for a declared component packet or preview.
    ///
    /// Failed entropy, formatting or encryption never restores preparation credit.
    /// Capacity retries reuse entropy and reach sealing only once.
    /// # Errors
    /// Preserve currentness, ticket/mode, repeated UID and actual codec failures.
    pub(in super::super) fn prepare(
        &mut self,
        ticket: &Ticket<'_>,
        fill: impl FnOnce(&mut [u8; 48]) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        self.begin_preparation(ticket, false)?;
        let mut entropy = Zeroizing::new([0; 48]);
        fill(&mut entropy)?;
        self.finish_preparation(&entropy)
    }

    /// Issue wire correlation, UID and nonce from one observed-request entropy batch.
    ///
    /// The first eight bytes are big-endian wire bits, followed by UID32 and nonce16.
    /// Zero wire bits refuse without resampling, refund or a fabricated local T1.
    /// The actual adapter separately retains its own software-clock observations.
    /// # Errors
    /// Preserve ticket/currentness/mode, entropy, zero token, UID and codec refusal.
    pub(in super::super) fn prepare_observed(
        &mut self,
        ticket: &Ticket<'_>,
        fill: impl FnOnce(&mut [u8; 56]) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        self.begin_preparation(ticket, true)?;
        let mut entropy = Zeroizing::new([0; 56]);
        fill(&mut entropy)?;
        let bytes = entropy
            .first_chunk::<8>()
            .ok_or(refused(LedgerError::Capacity))?;
        let wire = NonZeroU64::new(u64::from_be_bytes(*bytes))
            .ok_or(refused(LedgerError::ZeroWireToken))?;
        let material: &[u8; 48] = entropy
            .get(8..)
            .and_then(|bytes| bytes.try_into().ok())
            .ok_or(refused(LedgerError::Capacity))?;
        self.ledger
            .active
            .as_mut()
            .ok_or(refused(LedgerError::Ticket))?
            .observed_wire = Some(wire);
        self.finish_preparation(material)
    }

    /// Retire a new UID once, then stage and publish this request's private packet.
    /// # Errors
    /// Preserve spent-phase, UID history, currentness and encoding refusals.
    fn finish_preparation(&mut self, entropy: &[u8; 48]) -> Result<(), Failure> {
        let active = self
            .ledger
            .active
            .as_ref()
            .ok_or(refused(LedgerError::Ticket))?;
        if active.phase != RequestPhase::Preparing || active.uid.is_some() {
            return Err(refused(LedgerError::PreparationSpent));
        }
        let uid = entropy
            .first_chunk::<32>()
            .ok_or(refused(LedgerError::Capacity))?;
        if self.ledger.uid_was_issued(uid) {
            return Err(refused(LedgerError::RepeatedUid));
        }
        let index = self.ledger.uid_count;
        let saved = self
            .ledger
            .uids
            .get_mut(index)
            .ok_or(refused(LedgerError::Capacity))?;
        saved.copy_from_slice(uid);
        self.ledger.uid_count += 1;
        self.ledger
            .active
            .as_mut()
            .ok_or(refused(LedgerError::Ticket))?
            .uid = Some(index);
        let control = self.material.control;
        control.with_published(|_| Ok(()))?;
        let (ready, maximum_cookies) = self.stage_ready(entropy)?;
        control.with_published(|_| {
            let active = self
                .ledger
                .active
                .as_mut()
                .ok_or(refused(LedgerError::Ticket))?;
            active.ready = Some(ready);
            active.maximum_cookies = maximum_cookies;
            active.phase = RequestPhase::Prepared;
            Ok(())
        })
    }

    /// Encode only the occupied preparing request with its retired UID and own cookie.
    /// # Errors
    /// Refuse missing custody/observed wire, capacity or actual formatter/sealer failure.
    fn stage_ready(&self, entropy: &[u8; 48]) -> Result<(ReadyPacket<'a>, usize), Failure> {
        let active = self
            .ledger
            .active
            .as_ref()
            .ok_or(refused(LedgerError::Ticket))?;
        if active.phase != RequestPhase::Preparing || active.uid.is_none() {
            return Err(refused(LedgerError::Phase));
        }
        let cookie = active.cookie.as_ref().ok_or(refused(LedgerError::Phase))?;
        let wire = match active.input {
            RequestInput::Preview => {
                let prepared = handoff::prepare(
                    self.material.control,
                    &self.material.keys,
                    std::slice::from_ref(cookie),
                    |output| {
                        output.copy_from_slice(entropy);
                        Ok(())
                    },
                )?;
                return Ok((ReadyPacket::Preview(prepared), 1));
            }
            RequestInput::Packet { wire, local_t1: _ } => wire,
            RequestInput::Observed => active.observed_wire.ok_or(refused(LedgerError::Phase))?,
        };
        let mut raw_header = [0; 48];
        raw_header[0] = 0x23;
        raw_header[40..].copy_from_slice(&wire.get().to_be_bytes());
        let header = decode_fixed_header(&raw_header)
            .map_err(|error| Failure::Packet(RequestError::Header(error)))?;
        let mut storage = Zeroizing::new([0; PACKET_BYTES]);
        let vacancies = COOKIE_LIMIT
            .checked_sub(self.material.cookies.len())
            .ok_or(refused(LedgerError::Capacity))?;
        let maximum_placeholders = vacancies.saturating_sub(1).min(7);
        let (length, maximum_cookies) = encode_packet(
            &self.material.keys,
            &header,
            cookie,
            entropy,
            maximum_placeholders,
            &mut storage,
        )?;
        Ok((ReadyPacket::Packet { storage, length }, maximum_cookies))
    }

    /// Explicitly retire an uninvoked UID and return its exact moved cookie.
    ///
    /// A dropped/forgotten ticket does not call this operation. No UID, attempt
    /// or deadline is restored. Failure returns no cookie to any pool.
    /// # Errors
    /// Refuse foreign, stale, invoked or structurally inconsistent returns.
    #[expect(
        clippy::needless_pass_by_value,
        reason = "consuming the non-Clone ticket prevents reuse after an explicit return attempt"
    )]
    pub(in super::super) fn return_unused(&mut self, ticket: Ticket<'_>) -> Result<(), Failure> {
        self.check_ticket(&ticket)?;
        let control = self.material.control;
        control.with_published(|_| {
            let active = self
                .ledger
                .active
                .as_ref()
                .ok_or(refused(LedgerError::Ticket))?;
            if !matches!(
                active.phase,
                RequestPhase::Reserved | RequestPhase::Preparing | RequestPhase::Prepared
            ) {
                return Err(refused(LedgerError::Phase));
            }
            let cookie = active.cookie.as_ref().ok_or(refused(LedgerError::Phase))?;
            if self.ledger.cookie_was_used(cookie) {
                return Err(refused(LedgerError::Phase));
            }
            let bytes = self
                .material
                .cookies
                .iter()
                .try_fold(cookie.len(), |total, body| total.checked_add(body.len()))
                .ok_or(refused(LedgerError::Capacity))?;
            if self.material.cookies.len() >= COOKIE_LIMIT
                || bytes > COOKIE_BYTES_LIMIT
                || self.material.cookies.capacity() <= self.material.cookies.len()
            {
                return Err(refused(LedgerError::Capacity));
            }
            let mut active = self
                .ledger
                .active
                .take()
                .ok_or(refused(LedgerError::Ticket))?;
            let cookie = active.cookie.take().ok_or(refused(LedgerError::Phase))?;
            self.material.cookies.insert(0, cookie);
            Ok(())
        })
    }

    /// Exercise the same reservation/retirement owner for the unsendable preview.
    ///
    /// Only the original opaque handoff value leaves; packet bytes stay private.
    /// # Errors
    /// Preserve actual reservation, entropy, codec and explicit-return refusal.
    pub(super) fn ledger_preview(
        &mut self,
        fill: impl FnOnce(&mut [u8; 48]) -> Result<(), Failure>,
    ) -> Result<PreparedRequest<'a>, Failure> {
        let owner: super::PreviewOwner<'a> = self.preview_owner()?;
        let ticket: Ticket<'a> = self.reserve_inner(RequestInput::Preview)?;
        self.prepare(&ticket, fill)?;
        let ready: ReadyPacket<'a> = self
            .ledger
            .active
            .as_mut()
            .and_then(|active| active.ready.take())
            .ok_or(refused(LedgerError::Phase))?;
        let ReadyPacket::Preview(preview) = ready else {
            return Err(refused(LedgerError::Preview));
        };
        self.return_unused(ticket)?;
        preview.bind_owner(owner)
    }

    /// Match the exact current ticket and original Control without touching either.
    /// # Errors
    /// Refuse foreign or retired request authority before any adapter work.
    pub(in super::super) fn check_send_owner(
        &self,
        ticket: &Ticket<'_>,
        control: &Control,
    ) -> Result<(), Failure> {
        self.check_ticket(ticket)?;
        if !std::ptr::eq(self.material.control, control) {
            return Err(Failure::Control);
        }
        Ok(())
    }

    /// Archive under the single concrete sender's actual held Control.
    /// A refusal after archive retains Invoked custody and the original cause.
    /// # Errors
    /// Preserve actual request, currentness, archive and adapter refusals.
    #[cfg(any(windows, test))]
    #[expect(
        clippy::needless_pass_by_value,
        reason = "consuming the non-Clone ticket prevents return or repeated invocation after submission"
    )]
    pub(in super::super) fn invoke_guarded(
        &mut self,
        ticket: Ticket<'_>,
        sender: GuardedSender<'_, '_>,
    ) -> Result<(), Failure> {
        self.check_send_owner(&ticket, sender.control())?;
        let at: Instant = sender.archive_entry()?;
        let identity: Arc<()> = Arc::clone(sender.request_identity());
        let remote: SocketAddr = self.material.udp;
        let packet: &[u8] = self.archive_request(at)?;
        let expected: usize = packet.len();
        let (outcome, stamp): (SendOutcome, Option<ProtectedSendStamp>) =
            sender.submit(packet, remote)?;
        self.finish_submission(
            outcome,
            expected,
            SubmissionOrigin::Adapter(PendingSubmission { identity, stamp }),
        )
    }

    /// Exercise component callback outcomes without making them a production sender.
    /// A panic or Unknown return leaves Invoked and packet custody occupied.
    /// # Errors
    /// Preserve the same original archive and outcome rules as the concrete path.
    #[cfg(test)]
    #[expect(
        clippy::needless_pass_by_value,
        reason = "consuming the non-Clone ticket prevents return or repeated invocation after submission"
    )]
    pub(in super::super) fn invoke_sender(
        &mut self,
        ticket: Ticket<'_>,
        send: impl FnOnce(&[u8], SocketAddr) -> SendOutcome,
    ) -> Result<(), Failure> {
        if self.protected_endpoint()?.is_some() {
            return Err(Failure::Control);
        }
        self.check_ticket(&ticket)?;
        let control: &Control = self.material.control;
        let remote: SocketAddr = self.material.udp;
        let guard: super::super::ControlGuard<'_, '_> = control.guard()?;
        let at: Instant = Instant::now();
        guard.published(at)?;
        let packet: &[u8] = self.archive_request(at)?;
        let expected: usize = packet.len();
        drop(guard);
        let outcome: SendOutcome = send(packet, remote);
        self.finish_submission(outcome, expected, SubmissionOrigin::Component)
    }

    /// Archive the exact current Prepared ticket under the checked held Control.
    /// The caller has already checked original authority and ticket ownership.
    /// No packet view leaves until its cookie is unavailable for return or reuse.
    /// # Errors
    /// Refuse missing state, an unsendable preview or an invalid archive/packet.
    #[cfg(any(windows, test))]
    fn archive_request(&mut self, at: Instant) -> Result<&[u8], Failure> {
        let active: &mut Active<'_> = self
            .ledger
            .active
            .as_mut()
            .ok_or(refused(LedgerError::Ticket))?;
        if active.phase != RequestPhase::Prepared {
            return Err(refused(LedgerError::Phase));
        }
        let Some(ReadyPacket::Packet { storage, length }) = active.ready.as_ref() else {
            return Err(refused(LedgerError::Preview));
        };
        let cookie: &Zeroizing<Vec<u8>> =
            active.cookie.as_ref().ok_or(refused(LedgerError::Phase))?;
        let bytes: usize = self
            .ledger
            .used
            .iter()
            .try_fold(cookie.len(), |total: usize, body: &Zeroizing<Vec<u8>>| {
                total.checked_add(body.len())
            })
            .ok_or(refused(LedgerError::Capacity))?;
        if self.ledger.used.len() >= ATTEMPTS
            || bytes > COOKIE_BYTES_LIMIT
            || self.ledger.used.capacity() <= self.ledger.used.len()
            || was_used(&self.ledger.used, cookie)
            || self
                .ledger
                .previous
                .as_ref()
                .is_some_and(|past| past.cookie_was_used(cookie))
        {
            return Err(refused(LedgerError::Capacity));
        }
        let cookie: Zeroizing<Vec<u8>> = active.cookie.take().ok_or(refused(LedgerError::Phase))?;
        self.ledger.used.push(cookie);
        active.phase = RequestPhase::Invoked;
        active.sent_at = Some(at);
        storage.get(..*length).ok_or(refused(LedgerError::Capacity))
    }

    /// Apply the original synchronous disposition after the adapter guard dropped.
    /// Full length admits Pending only after a fresh original-Control check.
    /// # Errors
    /// Preserve short, failed, unknown and late-stop outcomes without reuse.
    #[cfg(any(windows, test))]
    fn finish_submission(
        &mut self,
        outcome: SendOutcome,
        expected: usize,
        submission: SubmissionOrigin,
    ) -> Result<(), Failure> {
        let control: &Control = self.material.control;
        match outcome {
            #[cfg(test)]
            SendOutcome::Unknown => Err(refused(LedgerError::Unknown)),
            SendOutcome::Failed(kind) => {
                self.ledger.active = None;
                Err(refused(LedgerError::Send(kind)))
            }
            SendOutcome::Completed(length) if length != expected => {
                self.ledger.active = None;
                Err(refused(LedgerError::ShortSend))
            }
            SendOutcome::Completed(_) => {
                let result: Result<(), Failure> = (|| {
                    let guard: DatagramAuthorityGuard<'_> =
                        DatagramAuthorityGuard::lock(control, self.protected_endpoint.as_ref())?;
                    guard.receiving(submission.policy_provenance())?;
                    guard.control.published(Instant::now())?;
                    let active: &mut Active<'_> = self
                        .ledger
                        .active
                        .as_mut()
                        .ok_or(refused(LedgerError::Ticket))?;
                    active.phase = RequestPhase::Pending;
                    active.submission = Some(submission);
                    Ok(())
                })();
                if result.is_err() {
                    self.ledger.active = None;
                }
                result
            }
        }
    }

    /// Stage bounded response data with this owner's keys and saved coordinates.
    ///
    /// The supplied UID position, wire token and allowance come from the caller's
    /// charged request snapshot. This pure helper creates no receipt, marker,
    /// charge, currentness or publication authority; only the receiver may commit.
    /// # Errors
    /// Refuse raw extent, absent issued UID, actual framing/AEAD/count admission
    /// or bounded replacement staging without changing any owner or Control state.
    fn stage_current_reply(
        &self,
        raw: &[u8],
        uid_index: usize,
        wire: NonZeroU64,
        maximum_cookies: usize,
    ) -> Result<StagedReplacement, Failure> {
        if raw.len() > PACKET_BYTES {
            return Err(refused(LedgerError::Capacity));
        }
        let uid: &[u8; 32] = self.issued_uid(uid_index)?;
        let reply: StagedReply =
            stage_reply(&self.material.keys, raw, uid, wire.get(), maximum_cookies)
                .map_err(|error: ReplyError| refused(LedgerError::Reply(error)))?;
        self.stage_replacement(reply)
    }

    /// Borrow a UID only from the original context's already-issued history.
    /// # Errors
    /// Refuse a position outside the actually issued prefix.
    fn issued_uid(&self, uid_index: usize) -> Result<&[u8; 32], Failure> {
        self.ledger
            .uids
            .get(..self.ledger.uid_count)
            .and_then(|issued: &[[u8; 32]]| issued.get(uid_index))
            .ok_or(refused(LedgerError::Capacity))
    }

    /// Preallocate a complete bounded replacement without changing live custody.
    ///
    /// Input already passed actual S2C and complete response admission. Archive
    /// filtering and pool arithmetic remain inert; final currentness and all
    /// cookie moves still belong to the receiving transaction's owner guard.
    /// # Errors
    /// Preserve allocation or aggregate-capacity refusal with the old pool intact.
    fn stage_replacement(&self, reply: StagedReply) -> Result<StagedReplacement, Failure> {
        let received_cookies: usize = reply.cookie_count();
        let (fields, cookies): (ReplyFields, CookiePool) = reply.into_parts();
        if self.material.cookies.len() > COOKIE_LIMIT || cookies.len() > COOKIE_LIMIT {
            return Err(refused(LedgerError::Capacity));
        }
        let mut next: CookiePool = Vec::new();
        next.try_reserve_exact(COOKIE_LIMIT)
            .map_err(|_| Failure::Allocation)?;
        let mut retain_existing: [bool; COOKIE_LIMIT] = [false; COOKIE_LIMIT];
        let mut retain_incoming: [bool; COOKIE_LIMIT] = [false; COOKIE_LIMIT];
        let (count, bytes): (usize, usize) = self
            .material
            .cookies
            .iter()
            .zip(&mut retain_existing)
            .chain(cookies.iter().zip(&mut retain_incoming))
            .try_fold(
                (0_usize, 0_usize),
                |(count, bytes): (usize, usize),
                 (body, retain): (&Zeroizing<Vec<u8>>, &mut bool)| {
                    if self.ledger.cookie_was_used(body) {
                        return Some((count, bytes));
                    }
                    *retain = true;
                    Some((count.checked_add(1)?, bytes.checked_add(body.len())?))
                },
            )
            .ok_or(refused(LedgerError::Capacity))?;
        if count > COOKIE_LIMIT || bytes > COOKIE_BYTES_LIMIT {
            return Err(refused(LedgerError::Capacity));
        }
        Ok(StagedReplacement {
            fields,
            replacement: CookieReplacement {
                cookies,
                next,
                retain_existing,
                retain_incoming,
                population: (count, bytes),
            },
            received_cookies,
            retained: retain_incoming
                .into_iter()
                .filter(|retain: &bool| *retain)
                .count(),
        })
    }

    /// Apply a prepared pool after confirming no newer ledger work changed its inputs.
    /// Only move selected entries into already reserved storage. The caller
    /// releases both old buffers and all rejected values outside Control custody.
    fn apply_replacement(&mut self, replacement: CookieReplacement) -> [CookiePool; 2] {
        let CookieReplacement {
            mut cookies,
            mut next,
            retain_existing,
            retain_incoming,
            ..
        }: CookieReplacement = replacement;
        for (body, retain) in self
            .material
            .cookies
            .iter_mut()
            .zip(retain_existing)
            .chain(cookies.iter_mut().zip(retain_incoming))
        {
            if retain {
                next.push(std::mem::replace(body, Zeroizing::new(Vec::new())));
            }
        }
        [std::mem::replace(&mut self.material.cookies, next), cookies]
    }

    /// Charge a consumed datagram whose original extent cannot be established.
    ///
    /// A full truncation-detecting buffer or a qualified consumed-size error must
    /// enter here once, rather than passing a manufactured prefix to the parser.
    /// Spend one remaining frame and all remaining bytes, then retire Pending and
    /// coalesce recovery. No cookie, receipt, deadline or request credit is restored.
    /// # Errors
    /// Refuse stale/wrong phase unchanged; a charged retirement returns `ReceiveWork`.
    pub(in super::super) fn charge_consumed_unknown(&mut self) -> Result<(), Failure> {
        let control = self.material.control;
        control.with_published(|_| {
            let active = self
                .ledger
                .active
                .as_ref()
                .ok_or(refused(LedgerError::Phase))?;
            if active.phase != RequestPhase::Pending {
                return Err(refused(LedgerError::Phase));
            }
            self.ledger.frames = self.ledger.frames.saturating_sub(1);
            self.ledger.bytes = 0;
            self.retire_exhausted();
            Err(refused(LedgerError::ReceiveWork))
        })
    }

    /// Charge the known extent of a datagram that was consumed and discarded.
    /// Reuse the ordinary Pending debit and retire the exhausted allowance.
    /// # Errors
    /// Preserve original Control, outstanding request and receive-budget refusals.
    pub(in super::super) fn charge_discarded_reply(
        &mut self,
        length: usize,
        backend: &Arc<()>,
    ) -> Result<(), Failure> {
        self.charge_reply(length, Some(backend))?;
        self.retire_exhausted();
        Ok(())
    }

    /// Charge and authenticate the concrete datagram once and retain its history.
    ///
    /// The original submission and replacement remain private until retired commit.
    /// # Errors
    /// Preserve admission, original budgets, endpoint/time, AEAD and shape errors.
    pub(in super::super) fn stage_observed_reply(
        &mut self,
        raw: &[u8],
        remote: SocketAddr,
        read: &SoftwareClock,
        backend: &Arc<()>,
    ) -> Result<(Receipt, CookieReplacement, PendingSubmission), Failure> {
        let (receipt, replacement, submission): (Receipt, CookieReplacement, SubmissionOrigin) =
            self.receive_reply_with(
                raw,
                remote,
                ReceiveSpan::from_read(read),
                Some(backend),
                |_, receipt, replacement, submission| (receipt, replacement, submission),
            )?;
        match submission {
            SubmissionOrigin::Adapter(submission) => Ok((receipt, replacement, submission)),
            #[cfg(test)]
            SubmissionOrigin::Component => Err(refused(LedgerError::Ticket)),
        }
    }

    /// Charge once before selecting an ordinary response or a narrow protected control.
    /// Ordinary admission consumes the request; control staging preserves it for the later owner.
    /// # Errors
    /// Preserve byte/frame, peer, backend, UID, S2C, time and policy refusals.
    #[cfg(test)]
    pub(in super::super) fn stage_protected_successor<'control>(
        &mut self,
        control: &'control Control,
        raw: &[u8],
        remote: SocketAddr,
        read: &SoftwareClock,
        backend: &Arc<()>,
    ) -> Result<StagedProtectedSuccessor<'control>, Failure> {
        if !std::ptr::eq(control, self.material.control) {
            return Err(Failure::Control);
        }
        let charged: ChargedReply = self.charge_reply(raw.len(), Some(backend))?;
        let span: ReceiveSpan = ReceiveSpan::from_read(read);
        let arrival: Instant = span.leading;
        let response: StagedSuccessorResponse = (|| {
            self.check_charged_datagram(&charged, raw.len(), remote)?;
            let uid: &[u8; 32] = self.issued_uid(charged.uid_index)?;
            stage_successor_response(
                &self.material.keys,
                raw,
                uid,
                charged.wire.get(),
                charged.maximum_cookies,
            )
            .map_err(|error: ReplyError| refused(LedgerError::Reply(error)))
        })()
        .inspect_err(|&error: &Failure| self.reject_staging(error, charged.sent_at, span))?;
        match response {
            StagedSuccessorResponse::Ordinary(reply) => {
                let staged: StagedReplacement =
                    self.stage_replacement(reply)
                        .inspect_err(|&error: &Failure| {
                            self.reject_staging(error, charged.sent_at, span);
                        })?;
                let (receipt, replacement, submission): (
                    Receipt,
                    CookieReplacement,
                    SubmissionOrigin,
                ) = self.consume_staged_reply(
                    &charged,
                    staged,
                    span,
                    |_, receipt, replacement, submission| (receipt, replacement, submission),
                )?;
                match submission {
                    SubmissionOrigin::Adapter(submission) => Ok(
                        StagedProtectedSuccessor::Ordinary(receipt, replacement, submission),
                    ),
                    SubmissionOrigin::Component => Err(refused(LedgerError::Ticket)),
                }
            }
            StagedSuccessorResponse::Control(kind) => {
                let candidate: StagedProtectedControl<'control> = StagedProtectedControl {
                    kind,
                    control,
                    serial: charged.serial,
                    uid_index: charged.uid_index,
                    wire: charged.wire,
                    backend: Arc::clone(backend),
                    sent_at: charged.sent_at,
                    arrival,
                    read_after: read.monotonic_after(),
                };
                self.check_protected_control_current(&candidate)
                    .inspect_err(|_| self.retire_exhausted())?;
                Ok(StagedProtectedSuccessor::Control(candidate))
            }
        }
    }

    /// Inspect one staged or retired ticket without consuming it or changing policy.
    /// # Errors
    /// Refuse a foreign or superseded request, late Stop, expiry or stale policy.
    #[cfg(test)]
    pub(in super::super) fn check_protected_control_current(
        &self,
        candidate: &StagedProtectedControl<'_>,
    ) -> Result<(), Failure> {
        if !std::ptr::eq(candidate.control, self.material.control) {
            return Err(Failure::Control);
        }
        let lease: &ProtectedEndpoint = self.protected_endpoint()?.ok_or(Failure::Control)?;
        let guard: DatagramAuthorityGuard<'_> =
            DatagramAuthorityGuard::lock(candidate.control, Some(lease))?;
        let submissions: ControlSubmissions<'_> = self.control_submissions(candidate)?;
        guard.receiving(Some(submissions.prior))?;
        guard.receiving(Some(submissions.pending))?;
        let now: Instant = Instant::now();
        guard.control.published(now)?;
        check_receive_times(candidate.sent_at, candidate.arrival, now)
    }

    /// Match the exact prior receipt and active successor under the original locks.
    /// # Errors
    /// Refuse missing or replaced request history, submission or managed stamps.
    #[cfg(test)]
    fn control_submissions(
        &self,
        candidate: &StagedProtectedControl<'_>,
    ) -> Result<ControlSubmissions<'_>, Failure> {
        let prior: &PriorAuthentication = self
            .ledger
            .prior_authentication
            .as_ref()
            .ok_or(Failure::Control)?;
        let active: &Active<'_> = self
            .ledger
            .active
            .as_ref()
            .ok_or(refused(LedgerError::Phase))?;
        if prior.attempts.is_some()
            || self.ledger.last.as_ref() != Some(&prior.receipt)
            || prior.receipt.serial.checked_add(1) != Some(candidate.serial)
            || active.phase != RequestPhase::Pending
            || active.serial != candidate.serial
            || active.uid != Some(candidate.uid_index)
            || active.observed_wire != Some(candidate.wire)
            || active.sent_at != Some(candidate.sent_at)
        {
            return Err(refused(LedgerError::Ticket));
        }
        let submission: &SubmissionOrigin = active
            .submission
            .as_ref()
            .ok_or(refused(LedgerError::Ticket))?;
        if !submission.matches(Some(&candidate.backend)) {
            return Err(refused(LedgerError::Ticket));
        }
        Ok(ControlSubmissions {
            prior: prior
                .submission
                .policy_provenance()
                .ok_or(Failure::Control)?,
            pending: submission.policy_provenance().ok_or(Failure::Control)?,
        })
    }

    /// Consume one actually retired request under the original Control and policy.
    /// No caller can open this effect with a merely staged ticket.
    /// # Errors
    /// Preserve original authority, binding and identity refusals before any
    /// mutation. The original receive bracket must be ordered, fully observed
    /// by processing time, and before cutoff. Refusal preserves the charged
    /// outstanding request and both policy records.
    #[cfg(test)]
    pub(in super::super) fn commit_retired_protected_control(
        &mut self,
        retired: RetiredProtectedControl<'_>,
    ) -> Result<AppliedProtectedControl, Failure> {
        let candidate: StagedProtectedControl<'_> = retired.into_staged();
        if !std::ptr::eq(candidate.control, self.material.control) {
            return Err(Failure::Control);
        }
        let lease: ProtectedEndpoint = self.protected_endpoint()?.ok_or(Failure::Control)?.clone();
        let configuration: Box<crate::SourceConfigurationSnapshot> =
            Box::new(lease.configuration().ok_or(Failure::Control)?.clone());
        let mut guard: DatagramAuthorityGuard<'_> =
            DatagramAuthorityGuard::lock(candidate.control, Some(&lease))?;
        let submissions: ControlSubmissions<'_> = self.control_submissions(&candidate)?;
        let now: Instant = Instant::now();
        check_receive_times(candidate.sent_at, candidate.arrival, now)?;
        if candidate.read_after >= candidate.control.expiry || now >= candidate.control.expiry {
            return Err(Failure::Deadline);
        }
        if candidate.read_after < candidate.arrival || candidate.read_after > now {
            return Err(refused(LedgerError::Time));
        }
        let applied: AppliedProtectedControl = AppliedProtectedControl {
            kind: candidate.kind,
            configuration,
            remote: self.material.udp,
            serial: candidate.serial,
            decided_at: now,
        };
        let authorization: VerifiedRetiredControl<'_> = VerifiedRetiredControl {
            kind: candidate.kind,
            control: candidate.control,
            submissions,
            decided_at: now,
        };
        guard.commit_protected_control(authorization)?;
        let request: Option<Active<'_>> = self.ledger.active.take();
        drop(guard);
        drop(request);
        Ok(applied)
    }

    /// Component-only receiver without a concrete socket or runtime to retire.
    /// # Errors
    /// Preserve the same admission and work accounting as the concrete receiver.
    #[cfg(test)]
    pub(in super::super) fn receive_reply(
        &mut self,
        raw: &[u8],
        remote: SocketAddr,
        arrival: Instant,
    ) -> Result<Receipt, Failure> {
        if self.protected_endpoint()?.is_some() {
            return Err(Failure::Control);
        }
        let (receipt, retired): (Receipt, [CookiePool; 2]) = self.receive_reply_with(
            raw,
            remote,
            ReceiveSpan {
                leading: arrival,
                trailing: arrival,
            },
            None,
            |owner, receipt, replacement, _submission| {
                (receipt, owner.apply_replacement(replacement))
            },
        )?;
        drop(retired);
        Ok(receipt)
    }

    /// Charge the actual datagram and sample its current Pending coordinates.
    /// # Errors
    /// Preserve original control, phase and aggregate budget refusals.
    fn charge_reply(
        &mut self,
        length: usize,
        backend: Option<&Arc<()>>,
    ) -> Result<ChargedReply, Failure> {
        let control: &Control = self.material.control;
        control.with_published(|_| {
            let active: &Active<'_> = self
                .ledger
                .active
                .as_ref()
                .ok_or(refused(LedgerError::Phase))?;
            if active.phase != RequestPhase::Pending {
                return Err(refused(LedgerError::Phase));
            }
            let (wire, local_t1): (NonZeroU64, Option<NtpTimestamp>) = match active.input {
                RequestInput::Packet { wire, local_t1 } => (wire, Some(local_t1)),
                RequestInput::Observed => (
                    active.observed_wire.ok_or(refused(LedgerError::Phase))?,
                    None,
                ),
                RequestInput::Preview => return Err(refused(LedgerError::Preview)),
            };
            let uid_index: usize = active.uid.ok_or(refused(LedgerError::Phase))?;
            let sent_at: Instant = active.sent_at.ok_or(refused(LedgerError::Phase))?;
            if self.ledger.frames == 0 || length > self.ledger.bytes {
                self.ledger.frames = self.ledger.frames.saturating_sub(1);
                self.ledger.bytes = 0;
                self.retire_exhausted();
                return Err(refused(LedgerError::ReceiveWork));
            }
            self.ledger.frames -= 1;
            self.ledger.bytes -= length;
            Ok(ChargedReply {
                serial: active.serial,
                uid_index,
                wire,
                local_t1,
                sent_at,
                maximum_cookies: active.maximum_cookies,
                matches_submission: active
                    .submission
                    .as_ref()
                    .is_some_and(|submission: &SubmissionOrigin| submission.matches(backend)),
            })
        })
    }

    /// Charge before parsing or opening, then consume the exact original Pending submission.
    /// Rejection retains the original deadline and reduces the remaining allowance.
    /// # Errors
    /// Preserve admission, budget, backend/endpoint/time, AEAD and shape errors.
    fn receive_reply_with<T>(
        &mut self,
        raw: &[u8],
        remote: SocketAddr,
        span: ReceiveSpan,
        backend: Option<&Arc<()>>,
        admit: impl FnOnce(&mut Self, Receipt, CookieReplacement, SubmissionOrigin) -> T,
    ) -> Result<T, Failure> {
        let charged: ChargedReply = self.charge_reply(raw.len(), backend)?;
        let staged: StagedReplacement = (|| {
            self.check_charged_datagram(&charged, raw.len(), remote)?;
            self.stage_current_reply(
                raw,
                charged.uid_index,
                charged.wire,
                charged.maximum_cookies,
            )
        })()
        .inspect_err(|&error: &Failure| {
            self.reject_staging(error, charged.sent_at, span);
        })?;
        self.consume_staged_reply(&charged, staged, span, admit)
    }

    /// Check the extent and submitter of a datagram after its exact ledger debit.
    /// # Errors
    /// Refuse an oversized datagram, foreign peer or different submission backend.
    fn check_charged_datagram(
        &self,
        charged: &ChargedReply,
        length: usize,
        remote: SocketAddr,
    ) -> Result<(), Failure> {
        if length > PACKET_BYTES {
            return Err(refused(LedgerError::Capacity));
        }
        if remote != self.material.udp {
            return Err(refused(LedgerError::Endpoint));
        }
        if !charged.matches_submission {
            return Err(refused(LedgerError::Ticket));
        }
        Ok(())
    }

    /// Keep the semantic cause of a final count refusal and enforce original exhaustion.
    fn reject_staging(&mut self, error: Failure, sent_at: Instant, span: ReceiveSpan) {
        if error == refused(LedgerError::Reply(ReplyError::Cookies)) && self.ledger.frames == 0 {
            self.ledger.exhausted_response = Some(ExhaustedResponse::Cookies { sent_at, span });
        }
        self.retire_exhausted();
    }

    /// Consume the exact charged ordinary request only under the original Control.
    /// # Errors
    /// Refuse expired, stopped or replaced Pending custody before moving any state.
    /// Both original receive readings must be ordered and observed by processing
    /// time inside the original exclusive cutoff. Work already charged stays spent.
    fn consume_staged_reply<T>(
        &mut self,
        charged: &ChargedReply,
        staged: StagedReplacement,
        span: ReceiveSpan,
        admit: impl FnOnce(&mut Self, Receipt, CookieReplacement, SubmissionOrigin) -> T,
    ) -> Result<T, Failure> {
        let control: &Control = self.material.control;
        let StagedReplacement {
            fields,
            replacement,
            received_cookies,
            retained,
        }: StagedReplacement = staged;
        let result: Result<T, Failure> = (|| {
            let guard: super::super::ControlGuard<'_, '_> = control.guard()?;
            let now: Instant = Instant::now();
            guard.published(now)?;
            let active: &mut Active<'_> = self
                .ledger
                .active
                .as_mut()
                .ok_or(refused(LedgerError::Phase))?;
            if active.serial != charged.serial || active.phase != RequestPhase::Pending {
                return Err(refused(LedgerError::Ticket));
            }
            span.check(charged.sent_at, now, control.expiry)?;
            let receipt: Receipt = Receipt {
                serial: charged.serial,
                local_t1: charged.local_t1,
                fields,
                accepted_at: now,
                received_cookies,
                retained_cookies: retained,
            };
            let submission: SubmissionOrigin = active
                .submission
                .take()
                .ok_or(refused(LedgerError::Ticket))?;
            self.ledger.active = None;
            self.ledger.last = Some(receipt);
            Ok(admit(self, receipt, replacement, submission))
        })();
        result.inspect_err(|_| self.retire_exhausted())
    }

    /// Inspect finite metadata without exposing cookies, UIDs, packets or keys.
    pub(in super::super) fn status(&self) -> Status {
        let available = self
            .material
            .cookies
            .iter()
            .enumerate()
            .filter(|(index, body)| {
                !self.ledger.cookie_was_used(body.as_slice())
                    && !self
                        .material
                        .cookies
                        .iter()
                        .take(*index)
                        .any(|earlier| earlier.as_slice() == body.as_slice())
            })
            .count();
        Status {
            phase: self
                .ledger
                .active
                .as_ref()
                .map_or(RequestPhase::Idle, |active| active.phase),
            attempts: self.ledger.attempts,
            frames: self.ledger.frames,
            bytes: self.ledger.bytes,
            available,
            used: self.ledger.used.len(),
            uids: self.ledger.uid_count,
        }
    }

    /// Return the last accepted historical receipt without renewing its age.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "EP2 production observation consumer remains held; component tests inspect retained receipts"
        )
    )]
    pub(in super::super) const fn last_receipt(&self) -> Option<Receipt> {
        self.ledger.last
    }

    /// Consume the sole coalesced intent; repeated calls cannot mint another.
    pub(in super::super) fn take_recovery_intent(&mut self) -> Option<RecoveryIntent> {
        self.ledger.recovery.take()
    }

    /// Consume the actual retired reply under one original Control transaction.
    ///
    /// The receipt's serial is the charged reservation ordinal, not a new identity.
    /// No later charged attempt may follow it, including a failed or returned one.
    /// Only this admitted transfer installs replacement cookies in the live pool.
    /// The observer only samples the processing clock under the original guards.
    /// # Errors
    /// Refuse foreign/stale context, an active slot, or a noncurrent receipt/ordinal
    /// or processing time outside the accepted receipt and original cutoff.
    /// Refusal preserves the pool, work spent and historical receipt.
    pub(in super::super) fn commit_retired_reply(
        &mut self,
        reply: RetiredReply<'_>,
        disposition: RetiredCommit,
        observe_now: impl FnOnce() -> Instant,
    ) -> Result<Arc<()>, Failure> {
        let (control, receipt, replacement, submission): RetiredReplyParts<'_> = reply.into_parts();
        if !std::ptr::eq(control, self.material.control) {
            return Err(Failure::Control);
        }
        let lease: Option<ProtectedEndpoint> = self.protected_endpoint()?.cloned();
        let mut guard: DatagramAuthorityGuard<'_> =
            DatagramAuthorityGuard::lock(control, lease.as_ref())?;
        self.check_completed_receipt(&receipt)?;
        guard.receiving(submission.policy_provenance())?;
        #[cfg(test)]
        if matches!(disposition, RetiredCommit::FirstRequest(_))
            && (lease.is_none()
                || receipt.serial != 1
                || self.ledger.prior_authentication.is_some())
        {
            return Err(Failure::Control);
        }
        let now: Instant = observe_now();
        if now < receipt.accepted_at() {
            // Preserve current Control and original deadline refusal precedence.
            guard.control.published(now)?;
            return Err(refused(LedgerError::Time));
        }
        let identity: Arc<()> = match disposition {
            RetiredCommit::Terminal => {
                guard.control.commit(now)?;
                submission.identity
            }
            #[cfg(test)]
            RetiredCommit::FirstRequest(attempts) => {
                guard.control.published(now)?;
                let identity: Arc<()> = Arc::clone(&submission.identity);
                self.ledger.prior_authentication = Some(PriorAuthentication {
                    receipt,
                    submission,
                    attempts: Some(attempts),
                });
                identity
            }
        };
        let retired: [CookiePool; 2] = self.apply_replacement(replacement);
        drop(guard);
        drop(retired);
        Ok(identity)
    }

    /// After a private first commit, only the original remaining driver can run.
    /// # Errors
    /// Refuse a fresh allowance on an association that already earned a successor.
    #[cfg(test)]
    pub(in super::super) fn check_fresh_exchange(&self) -> Result<(), Failure> {
        if self.ledger.prior_authentication.is_some() {
            return Err(Failure::Control);
        }
        Ok(())
    }

    /// Spend the one continuation before entering a second backend or entropy.
    /// # Errors
    /// Refuse missing, foreign, stale, already spent or expired authority.
    #[cfg(test)]
    pub(in super::super) fn take_authenticated_successor(
        &mut self,
        control: &Control,
    ) -> Result<Attempts, Failure> {
        if !std::ptr::eq(control, self.material.control) {
            return Err(Failure::Control);
        }
        let lease: ProtectedEndpoint = self.protected_endpoint()?.ok_or(Failure::Control)?.clone();
        let guard: DatagramAuthorityGuard<'_> =
            DatagramAuthorityGuard::lock(control, Some(&lease))?;
        let prior: &mut PriorAuthentication = self
            .ledger
            .prior_authentication
            .as_mut()
            .ok_or(Failure::Control)?;
        if self.ledger.active.is_some()
            || self.ledger.last.as_ref() != Some(&prior.receipt)
            || ATTEMPTS.checked_sub(self.ledger.attempts) != Some(prior.receipt.serial)
        {
            return Err(refused(LedgerError::Ticket));
        }
        guard.receiving(prior.submission.policy_provenance())?;
        guard.control.published(Instant::now())?;
        prior.attempts.take().ok_or(refused(LedgerError::Ticket))
    }

    /// Exercise the receipt check without granting a retired reply or a commit.
    /// # Errors
    /// Preserve foreign, expired or stale receipt refusal.
    #[cfg(test)]
    pub(in super::super) fn check_completed(
        &self,
        control: &Control,
        receipt: &Receipt,
    ) -> Result<(), Failure> {
        if !std::ptr::eq(control, self.material.control) {
            return Err(Failure::Control);
        }
        control.with_published(|_| self.check_completed_receipt(receipt))
    }

    /// Require the exact original completed ordinal with no subsequent work.
    /// # Errors
    /// A missing receipt or newer charged slot makes this result stale.
    fn check_completed_receipt(&self, receipt: &Receipt) -> Result<(), Failure> {
        if self.ledger.active.is_some()
            || self.ledger.last.as_ref() != Some(receipt)
            || ATTEMPTS.checked_sub(self.ledger.attempts) != Some(receipt.serial)
        {
            return Err(refused(LedgerError::Ticket));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;

// SPDX-License-Identifier: GPL-2.0-only
//! Original-context cookie, request and receive-work custody.
//!
//! The sole normal issuer is the parent association's actual KE receiver. These
//! synchronous component boundaries perform no UDP/native operation. A future
//! native adapter must independently establish actual completion and custody.

use std::io;
use std::net::SocketAddr;
use std::num::NonZeroU64;
use std::time::Instant;

use ntp_wire::{FrameError, NtpTimestamp, NtsRequestEncodeError, RawHeader, decode_fixed_header};
use zeroize::Zeroizing;

use super::super::handoff::{self, PreparedRequest};
use super::super::request::{DirectionalKeys, RequestError};
use super::super::response::{ReplyError, ReplyFields, StagedReply, stage_reply};
use super::{AssociationOwner, COOKIE_BYTES_LIMIT, COOKIE_LIMIT, Control, Failure};

pub(in super::super) mod retirement;

/// Nonrenewable checkout and entropy attempts in one original context.
const ATTEMPTS: usize = 32;
/// Nonrenewable datagram work in that same original context.
const FRAMES: usize = 32;
/// Aggregate raw received octets, including rejected datagrams.
const RECEIVE_BYTES: usize = 131_072;
/// One bounded private request and one current datagram.
const PACKET_BYTES: usize = 4096;

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
    /// Physical completion is unknown; the occupied slot remains unavailable.
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
    /// No completion fact exists; custody and Busy remain sticky.
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
#[derive(Clone, Copy)]
pub(in super::super) struct Receipt {
    /// Original finite request identity.
    serial: usize,
    /// Optional declared component T1; actual adapter captures are retained separately.
    local_t1: Option<NtpTimestamp>,
    /// Authenticated inert header fields from the maintained consumer.
    fields: ReplyFields,
    /// Fresh original-context processing observation at atomic publication.
    accepted_at: Instant,
    /// Number of encrypted Cookie fields, including unusable repeated values.
    received_cookies: usize,
    /// Retained cookie entries excluding used values; duplicates count separately.
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
    /// Complete owned incoming population, wiped on every ordinary refusal.
    cookies: Vec<Zeroizing<Vec<u8>>>,
    /// Empty destination with room for the complete next bounded pool.
    next: Vec<Zeroizing<Vec<u8>>>,
    /// All encrypted Cookie fields, including known-used values.
    received_cookies: usize,
    /// Never-used incoming values counted under the original archive.
    retained: usize,
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
}

/// Actual final authenticated count refusal, still requiring current-time admission.
#[derive(Clone, Copy)]
enum ExhaustedResponse {
    /// Saved local submission and actual caller observation remain distinct.
    Cookies {
        /// Original monotonic observation taken before invoking the sender.
        sent_at: Instant,
        /// Supplied receive observation, rechecked by consuming retirement.
        arrival: Instant,
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
        let control = self.material.control;
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
            let ledger = &self.ledger;
            self.material
                .cookies
                .retain(|body| !ledger.cookie_was_used(body));
            if self.material.cookies.is_empty() {
                return Err(refused(LedgerError::Cookies));
            }
            let cookie = self.material.cookies.remove(0);
            let serial = ATTEMPTS - self.ledger.attempts;
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
        control.with_published(|_| Ok(self.material.udp))
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
    pub(in super::super) fn ledger_preview(
        &mut self,
        fill: impl FnOnce(&mut [u8; 48]) -> Result<(), Failure>,
    ) -> Result<PreparedRequest<'_>, Failure> {
        let ticket = self.reserve(RequestInput::Preview)?;
        self.prepare(&ticket, fill)?;
        let ready = self
            .ledger
            .active
            .as_mut()
            .and_then(|active| active.ready.take())
            .ok_or(refused(LedgerError::Phase))?;
        let ReadyPacket::Preview(preview) = ready else {
            return Err(refused(LedgerError::Preview));
        };
        self.return_unused(ticket)?;
        Ok(preview)
    }

    /// Archive before exposing packet bytes to the selected synchronous adapter.
    ///
    /// A panic or Unknown return leaves Invoked and packet custody occupied.
    /// Known synchronous returns never reissue the archived cookie. No native
    /// completion claim follows from these component callback outcomes.
    /// # Errors
    /// Preserve guards, preview refusal, short/error/unknown and late controls.
    #[expect(
        clippy::needless_pass_by_value,
        reason = "consuming the non-Clone ticket prevents return or repeated invocation after submission"
    )]
    pub(in super::super) fn invoke_sender(
        &mut self,
        ticket: Ticket<'_>,
        send: impl FnOnce(&[u8], SocketAddr) -> SendOutcome,
    ) -> Result<(), Failure> {
        self.check_ticket(&ticket)?;
        let control = self.material.control;
        control.with_published(|now| {
            let active = self
                .ledger
                .active
                .as_mut()
                .ok_or(refused(LedgerError::Ticket))?;
            if active.phase != RequestPhase::Prepared {
                return Err(refused(LedgerError::Phase));
            }
            if !matches!(active.ready, Some(ReadyPacket::Packet { .. })) {
                return Err(refused(LedgerError::Preview));
            }
            let cookie = active.cookie.as_ref().ok_or(refused(LedgerError::Phase))?;
            let bytes = self
                .ledger
                .used
                .iter()
                .try_fold(cookie.len(), |total, body| total.checked_add(body.len()))
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
            let cookie = active.cookie.take().ok_or(refused(LedgerError::Phase))?;
            self.ledger.used.push(cookie);
            active.phase = RequestPhase::Invoked;
            active.sent_at = Some(now);
            Ok(())
        })?;
        let active = self
            .ledger
            .active
            .as_ref()
            .ok_or(refused(LedgerError::Ticket))?;
        let Some(ReadyPacket::Packet { storage, length }) = active.ready.as_ref() else {
            return Err(refused(LedgerError::Phase));
        };
        let packet = storage
            .get(..*length)
            .ok_or(refused(LedgerError::Capacity))?;
        let expected = packet.len();
        let outcome = send(packet, self.material.udp);
        match outcome {
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
                let result = control.with_published(|_| {
                    let active = self
                        .ledger
                        .active
                        .as_mut()
                        .ok_or(refused(LedgerError::Ticket))?;
                    active.phase = RequestPhase::Pending;
                    Ok(())
                });
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
        let uid = self
            .ledger
            .uids
            .get(..self.ledger.uid_count)
            .and_then(|issued| issued.get(uid_index))
            .ok_or(refused(LedgerError::Capacity))?;
        let reply = stage_reply(&self.material.keys, raw, uid, wire.get(), maximum_cookies)
            .map_err(|error| refused(LedgerError::Reply(error)))?;
        self.stage_replacement(reply)
    }

    /// Preallocate a complete bounded replacement without changing live custody.
    ///
    /// Input already passed actual S2C and complete response admission. Archive
    /// filtering and pool arithmetic remain inert; final currentness and all
    /// cookie moves still belong to the receiving transaction's owner guard.
    /// # Errors
    /// Preserve allocation or aggregate-capacity refusal with the old pool intact.
    fn stage_replacement(&self, reply: StagedReply) -> Result<StagedReplacement, Failure> {
        let received_cookies = reply.cookie_count();
        let (fields, cookies) = reply.into_parts();
        let mut next = Vec::new();
        next.try_reserve_exact(COOKIE_LIMIT)
            .map_err(|_| Failure::Allocation)?;
        let retained = cookies
            .iter()
            .filter(|body| !self.ledger.cookie_was_used(body))
            .count();
        let (count, bytes) = self
            .material
            .cookies
            .iter()
            .chain(cookies.iter())
            .filter(|body| !self.ledger.cookie_was_used(body))
            .try_fold((0_usize, 0_usize), |(count, bytes), body| {
                Some((count.checked_add(1)?, bytes.checked_add(body.len())?))
            })
            .ok_or(refused(LedgerError::Capacity))?;
        if count > COOKIE_LIMIT || bytes > COOKIE_BYTES_LIMIT {
            return Err(refused(LedgerError::Capacity));
        }
        Ok(StagedReplacement {
            fields,
            cookies,
            next,
            received_cookies,
            retained,
        })
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

    /// Charge one complete raw component datagram before parse or decryption.
    ///
    /// Rejection preserves the original pending deadline and decreased work.
    /// Last-allowance success may commit; rejection at exhaustion retires it.
    /// # Errors
    /// Preserve admission, original budgets, endpoint/time, AEAD and shape errors.
    pub(in super::super) fn receive_reply(
        &mut self,
        raw: &[u8],
        remote: SocketAddr,
        arrival: Instant,
    ) -> Result<Receipt, Failure> {
        let control = self.material.control;
        let (serial, uid_index, wire, local_t1, sent_at, maximum_cookies) = control
            .with_published(|_| {
                let active = self
                    .ledger
                    .active
                    .as_ref()
                    .ok_or(refused(LedgerError::Phase))?;
                if active.phase != RequestPhase::Pending {
                    return Err(refused(LedgerError::Phase));
                }
                let (wire, local_t1) = match active.input {
                    RequestInput::Packet { wire, local_t1 } => (wire, Some(local_t1)),
                    RequestInput::Observed => (
                        active.observed_wire.ok_or(refused(LedgerError::Phase))?,
                        None,
                    ),
                    RequestInput::Preview => return Err(refused(LedgerError::Preview)),
                };
                let uid = active.uid.ok_or(refused(LedgerError::Phase))?;
                let sent_at = active.sent_at.ok_or(refused(LedgerError::Phase))?;
                if self.ledger.frames == 0 || raw.len() > self.ledger.bytes {
                    self.ledger.frames = self.ledger.frames.saturating_sub(1);
                    self.ledger.bytes = 0;
                    self.retire_exhausted();
                    return Err(refused(LedgerError::ReceiveWork));
                }
                self.ledger.frames -= 1;
                self.ledger.bytes -= raw.len();
                Ok((
                    active.serial,
                    uid,
                    wire,
                    local_t1,
                    sent_at,
                    active.maximum_cookies,
                ))
            })?;
        let staged = (|| {
            if raw.len() > PACKET_BYTES {
                return Err(refused(LedgerError::Capacity));
            }
            if remote != self.material.udp {
                return Err(refused(LedgerError::Endpoint));
            }
            self.stage_current_reply(raw, uid_index, wire, maximum_cookies)
        })()
        .inspect_err(|&error| {
            // Only the actual staging call returns this authenticated count cause.
            if error == refused(LedgerError::Reply(ReplyError::Cookies)) && self.ledger.frames == 0
            {
                self.ledger.exhausted_response =
                    Some(ExhaustedResponse::Cookies { sent_at, arrival });
            }
            self.retire_exhausted();
        })?;
        let StagedReplacement {
            fields,
            cookies,
            mut next,
            received_cookies,
            retained,
        } = staged;
        let result = control.with_published(|now| {
            let active = self
                .ledger
                .active
                .as_ref()
                .ok_or(refused(LedgerError::Phase))?;
            if active.serial != serial || active.phase != RequestPhase::Pending {
                return Err(refused(LedgerError::Ticket));
            }
            check_receive_times(sent_at, arrival, now)?;
            for body in self.material.cookies.drain(..).chain(cookies) {
                if !self.ledger.cookie_was_used(&body) {
                    next.push(body);
                }
            }
            self.material.cookies = next;
            let receipt = Receipt {
                serial,
                local_t1,
                fields,
                accepted_at: now,
                received_cookies,
                retained_cookies: retained,
            };
            self.ledger.active = None;
            self.ledger.last = Some(receipt);
            Ok(receipt)
        });
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

    /// Check an opaque completion against this actual, still-published owner.
    ///
    /// The receipt's serial is the charged reservation ordinal, not a new identity.
    /// No later charged attempt may follow it, including a failed or returned one.
    /// This check grants no committed fact and performs no final Control commit.
    /// The consuming caller retains exclusive owner borrowing through that commit.
    /// # Errors
    /// Refuse foreign/stale context, an active slot, or a noncurrent receipt/ordinal
    /// without changing request state, budgets, history or either Control.
    #[cfg(any(windows, test))]
    pub(in super::super) fn check_completed(
        &self,
        control: &Control,
        receipt: &Receipt,
    ) -> Result<(), Failure> {
        if !std::ptr::eq(control, self.material.control) {
            return Err(Failure::Control);
        }
        control.with_published(|_| {
            if self.ledger.active.is_some()
                || self.ledger.last.map(|last| last.serial) != Some(receipt.serial)
                || ATTEMPTS.checked_sub(self.ledger.attempts) != Some(receipt.serial)
            {
                return Err(refused(LedgerError::Ticket));
            }
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests;

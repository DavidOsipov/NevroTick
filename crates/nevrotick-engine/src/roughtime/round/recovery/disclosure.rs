// SPDX-License-Identifier: GPL-2.0-only
//! Controlled local disclosure of the original retained Roughtime conflict.

use super::{ConflictEvidence, ConflictProof, ConflictProofError};
use crate::MAX_TIMEOUT;
use ntp_wire::roughtime::MAX_RESPONSE_BYTES;
use std::{
    collections::TryReserveError,
    sync::{
        Mutex, MutexGuard,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

/// Maximum number of encoded report bytes accepted by the controlled recipient.
const LIMIT: usize = 65_536;
/// Exact complete framed request length produced by the controlled sender.
const REQUEST_BYTES: usize = 1_036;
/// Start of the RFC report object and its ordered response array.
const PREFIX: &[u8] = b"{\"responses\":[";
/// Start of one entry and its expected long-term public-key field.
const ROOT: &[u8] = b"{\"publicKey\":\"";
/// Field delimiter preceding the original framed request.
const REQUEST: &[u8] = b"\",\"request\":\"";
/// Field delimiter preceding the original framed response.
const RESPONSE: &[u8] = b"\",\"response\":\"";
/// Field delimiter preceding a later entry's chaining randomness.
const RAND: &[u8] = b"\",\"rand\":\"";
/// Terminator of the last encoded value and its response entry.
const END_ENTRY: &[u8] = b"\"}";
/// Terminator of the response array and report object.
const END_REPORT: &[u8] = b"]}";
/// Standard Base64 alphabet used with explicit equals-sign padding.
const BASE64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// A bounded refusal at the controlled local disclosure boundary.
#[derive(Debug)]
#[non_exhaustive]
pub enum DisclosureError {
    /// The session is retired, its mutex is unavailable, or its counter is exhausted.
    Unavailable,
    /// The selected action is stale, withdrawn, or beyond its exclusive cutoff.
    Stale,
    /// The original encoder or recipient still holds its single slot.
    Busy,
    /// The proposed lifetime is outside the controlled finite budget.
    Configuration,
    /// Original historical evidence no longer reconstructs its first conflict.
    Proof(ConflictProofError),
    /// The exact frames or encoded output exceed the controlled profile.
    Size,
    /// The bounded output buffer could not be allocated.
    Allocation,
    /// A required private clock observation was unavailable.
    Clock,
}

/// One confirmation tied to the original controlled evidence and recipient.
///
/// This controlled action does not establish a production user identity or IPC grant.
///
/// ```compile_fail,E0451
/// use nevrotick_engine::roughtime::round::recovery::{ConflictEvidence, DisclosureAction};
/// fn forged(evidence: &ConflictEvidence) -> DisclosureAction<'_> {
///     DisclosureAction { evidence, generation: 1 }
/// }
/// ```
///
/// ```no_run
/// use nevrotick_engine::roughtime::round::recovery::{DisclosureAction, DisclosureError};
/// fn consume(action: DisclosureAction<'_>) -> Result<(), DisclosureError> {
///     action.submit()
/// }
/// ```
#[must_use = "submit the original confirmation or allow it to expire"]
pub struct DisclosureAction<'a> {
    /// Original evidence and disclosure owner tied to this confirmation.
    evidence: &'a ConflictEvidence,
    /// Nonreused owner-issued generation of this confirmation.
    generation: u64,
}

/// Current confirmation's identity and monotonic validity interval.
struct Current {
    /// Generation bound to its original action.
    generation: u64,
    /// Owner clock observation when the action was issued.
    issued: Instant,
    /// Exclusive owner-clock deadline for the transfer.
    cutoff: Instant,
}

/// Guarded confirmation and single-recipient state for one evidence owner.
struct State {
    /// Last issued generation, retained when the current action is dismissed.
    generation: u64,
    /// Permanent session retirement, which refuses later confirmations.
    retired: bool,
    /// Current action and its original time bounds, when one is confirmed.
    current: Option<Current>,
    /// The one report transferred to this controlled recipient, if occupied.
    recipient: Option<Vec<u8>>,
}

/// Serializes this evidence's single controlled session, encoder and recipient.
pub(super) struct StateOwner {
    /// Serializes confirmation, retirement and recipient transfer.
    state: Mutex<State>,
    /// Retains the encoder's single buffer until its actual drop or transfer.
    reserved: AtomicBool,
}

impl StateOwner {
    /// Create an unused disclosure owner with no action or recipient bytes.
    pub(super) const fn new() -> Self {
        Self {
            state: Mutex::new(State {
                generation: 0,
                retired: false,
                current: None,
                recipient: None,
            }),
            reserved: AtomicBool::new(false),
        }
    }

    /// Try to serialize an operation with this evidence's original state.
    ///
    /// # Errors
    /// Refuses contention and poisoning without granting a state guard.
    fn lock(&self) -> Result<MutexGuard<'_, State>, DisclosureError> {
        self.state
            .try_lock()
            .map_err(|_| DisclosureError::Unavailable)
    }
}

/// A borrowed view of the controlled recipient's one retained report.
#[must_use = "observe or retire the original recipient report"]
pub struct DisclosureView<'a> {
    /// Keeps the recipient slot stable while its report is borrowed.
    state: MutexGuard<'a, State>,
}

impl DisclosureView<'_> {
    /// Borrow the exact recipient bytes while retaining its slot and guard.
    #[must_use]
    pub fn bytes(&self) -> Option<&[u8]> {
        self.state.recipient.as_deref()
    }
}

impl ConflictEvidence {
    /// Confirm disclosure to this evidence's fixed controlled local recipient.
    /// A new confirmation revokes any older one, including one being encoded.
    ///
    /// # Errors
    /// Refuses invalid duration, a retired session, an unavailable guard or overflow.
    pub fn confirm_disclosure(
        &self,
        lifetime: Duration,
    ) -> Result<DisclosureAction<'_>, DisclosureError> {
        if lifetime.is_zero() || lifetime > MAX_TIMEOUT {
            return Err(DisclosureError::Configuration);
        }
        let mut state: MutexGuard<'_, State> = self.disclosure.lock()?;
        if state.retired {
            return Err(DisclosureError::Unavailable);
        }
        let generation: u64 = state
            .generation
            .checked_add(1)
            .ok_or(DisclosureError::Unavailable)?;
        let issued: Instant = Instant::now();
        let cutoff: Instant = issued
            .checked_add(lifetime)
            .ok_or(DisclosureError::Configuration)?;
        state.generation = generation;
        state.current = Some(Current {
            generation,
            issued,
            cutoff,
        });
        Ok(DisclosureAction {
            evidence: self,
            generation,
        })
    }

    /// Revoke pending and in-flight confirmations without clearing the conflict.
    ///
    /// # Errors
    /// Refuses if this evidence's original guard cannot be acquired.
    pub fn dismiss_disclosure(&self) -> Result<(), DisclosureError> {
        let mut state: MutexGuard<'_, State> = self.disclosure.lock()?;
        state.current = None;
        Ok(())
    }

    /// Permanently retire the controlled disclosure session.
    ///
    /// # Errors
    /// Refuses if this evidence's original guard cannot be acquired.
    pub fn retire_disclosure(&self) -> Result<(), DisclosureError> {
        let mut state: MutexGuard<'_, State> = self.disclosure.lock()?;
        state.current = None;
        state.retired = true;
        Ok(())
    }

    /// Borrow the controlled recipient's existing report, if any.
    ///
    /// # Errors
    /// Refuses if this evidence's original guard cannot be acquired.
    pub fn disclosure_report(&self) -> Result<Option<DisclosureView<'_>>, DisclosureError> {
        let state: MutexGuard<'_, State> = self.disclosure.lock()?;
        if state.recipient.is_none() {
            return Ok(None);
        }
        Ok(Some(DisclosureView { state }))
    }

    /// Retire the original recipient buffer so a later confirmation can use the slot.
    ///
    /// # Errors
    /// Refuses if this evidence's original guard cannot be acquired.
    pub fn clear_disclosure_report(&self) -> Result<(), DisclosureError> {
        let mut state: MutexGuard<'_, State> = self.disclosure.lock()?;
        state.recipient = None;
        Ok(())
    }
}

/// Retain the unique encoding reservation until transfer or buffer destruction.
struct Reservation<'a> {
    /// The same evidence owner that charged the encoding reservation.
    owner: &'a StateOwner,
}

impl Drop for Reservation<'_> {
    fn drop(&mut self) {
        self.owner.reserved.store(false, Ordering::Release);
    }
}

impl DisclosureAction<'_> {
    /// Consume this confirmation and transfer one complete report to its recipient.
    ///
    /// # Errors
    /// Refuses stale authority, occupied capacity, invalid evidence or allocation failure.
    pub fn submit(self) -> Result<(), DisclosureError> {
        self.submit_using(
            || Some(Instant::now()),
            || {},
            LIMIT,
            Vec::try_reserve_exact,
        )
    }

    /// Execute the transition with private clock, reservation and precommit operations.
    ///
    /// The production caller supplies the owner clock, a no-op hook, the full
    /// limit and the vector's exact fallible reservation.
    /// The reservation remains held while reconstructed evidence is encoded and
    /// the completed buffer is transferred or dropped.
    ///
    /// # Errors
    /// Refuses unavailable state or clock, stale confirmation, occupied capacity,
    /// invalid evidence, size overflow or allocation failure without a handoff.
    fn submit_using(
        self,
        mut clock: impl FnMut() -> Option<Instant>,
        before_commit: impl FnOnce(),
        limit: usize,
        reserve: impl FnOnce(&mut Vec<u8>, usize) -> Result<(), TryReserveError>,
    ) -> Result<(), DisclosureError> {
        let owner: &StateOwner = &self.evidence.disclosure;
        let entered: Instant;
        {
            let state: MutexGuard<'_, State> = owner.lock()?;
            let observed: Instant = clock().ok_or(DisclosureError::Clock)?;
            require_current(&state, self.generation, observed)?;
            if owner.reserved.load(Ordering::Acquire) || state.recipient.is_some() {
                return Err(DisclosureError::Busy);
            }
            owner.reserved.store(true, Ordering::Release);
            entered = observed;
        }
        let reservation: Reservation<'_> = Reservation { owner };
        let proof: ConflictProof<'_> = self
            .evidence
            .reconstruct()
            .map_err(DisclosureError::Proof)?;
        let output: Vec<u8> = encode(proof.evidence(), limit, reserve)?;
        before_commit();
        {
            let mut state: MutexGuard<'_, State> = owner.lock()?;
            let observed: Instant = clock().ok_or(DisclosureError::Clock)?;
            if observed < entered {
                return Err(DisclosureError::Clock);
            }
            require_current(&state, self.generation, observed)?;
            if state.generation != self.generation
                || !owner.reserved.load(Ordering::Acquire)
                || state.recipient.is_some()
            {
                return Err(DisclosureError::Stale);
            }
            state.recipient = Some(output);
            state.current = None;
        }
        drop(reservation);
        Ok(())
    }
}

/// Check the action generation and owner-clock interval under the state guard.
///
/// # Errors
/// Refuses missing or retired authority, a foreign generation or an observation
/// before issuance or at or after the exclusive cutoff.
fn require_current(state: &State, generation: u64, now: Instant) -> Result<(), DisclosureError> {
    let current: &Current = state.current.as_ref().ok_or(DisclosureError::Stale)?;
    if state.retired
        || current.generation != generation
        || now < current.issued
        || now >= current.cutoff
    {
        return Err(DisclosureError::Stale);
    }
    Ok(())
}

/// Return the padded Base64 length for `len` bytes, or `None` on overflow.
fn base64_len(len: usize) -> Option<usize> {
    len.checked_add(2)?.checked_div(3)?.checked_mul(4)
}

/// Encode the original bounded receipt prefix as one compact RFC report.
///
/// The caller reconstructs the evidence and retains the encoding reservation.
/// `limit` is an additional byte ceiling; the owner limit applies independently.
/// `reserve` guarantees the requested capacity on success without adding bytes.
/// The returned buffer contains complete ordered fields and is not yet published.
///
/// # Errors
/// Refuses an unsupported prefix or frame shape, size overflow, excess output,
/// or failure to allocate the required buffer.
fn encode(
    evidence: &ConflictEvidence,
    limit: usize,
    reserve: impl FnOnce(&mut Vec<u8>, usize) -> Result<(), TryReserveError>,
) -> Result<Vec<u8>, DisclosureError> {
    let entries = evidence.receipts();
    if !(2..=9).contains(&entries.len()) {
        return Err(DisclosureError::Size);
    }
    let mut required: usize = PREFIX.len() + END_REPORT.len();
    for (index, entry) in entries.iter().enumerate() {
        let request: &[u8] = entry.request_frame();
        let response: &[u8] = entry.response_frame();
        if request.len() != REQUEST_BYTES
            || response.len() > MAX_RESPONSE_BYTES
            || entry.chaining_randomness().is_some() != (index != 0)
        {
            return Err(DisclosureError::Size);
        }
        let lengths: [usize; 6] = [
            ROOT.len(),
            REQUEST.len(),
            RESPONSE.len(),
            END_ENTRY.len(),
            usize::from(index != 0),
            if index == 0 { 0 } else { RAND.len() },
        ];
        for length in lengths {
            required = required.checked_add(length).ok_or(DisclosureError::Size)?;
        }
        for length in [
            32,
            request.len(),
            response.len(),
            if index == 0 { 0 } else { 32 },
        ] {
            required = required
                .checked_add(base64_len(length).ok_or(DisclosureError::Size)?)
                .ok_or(DisclosureError::Size)?;
        }
    }
    if required > LIMIT || required > limit {
        return Err(DisclosureError::Size);
    }
    let mut output: Vec<u8> = Vec::new();
    reserve(&mut output, required).map_err(|_| DisclosureError::Allocation)?;
    output.extend_from_slice(PREFIX);
    for (index, entry) in entries.iter().enumerate() {
        if index != 0 {
            output.push(b',');
        }
        output.extend_from_slice(ROOT);
        base64(&mut output, entry.root_key_bytes());
        output.extend_from_slice(REQUEST);
        base64(&mut output, entry.request_frame());
        output.extend_from_slice(RESPONSE);
        base64(&mut output, entry.response_frame());
        if let Some(random) = entry.chaining_randomness() {
            output.extend_from_slice(RAND);
            base64(&mut output, random);
        }
        output.extend_from_slice(END_ENTRY);
    }
    output.extend_from_slice(END_REPORT);
    if output.len() != required {
        return Err(DisclosureError::Size);
    }
    Ok(output)
}

/// Append padded standard Base64 for `input` to the caller's reserved output.
///
/// The caller reserves the complete checked report size before encoding fields.
fn base64(output: &mut Vec<u8>, input: &[u8]) {
    for chunk in input.chunks(3) {
        let first: u8 = chunk[0];
        let second: u8 = chunk.get(1).copied().unwrap_or(0);
        let third: u8 = chunk.get(2).copied().unwrap_or(0);
        output.push(BASE64[usize::from(first >> 2)]);
        output.push(BASE64[usize::from(((first & 3) << 4) | (second >> 4))]);
        output.push(if chunk.len() > 1 {
            BASE64[usize::from(((second & 15) << 2) | (third >> 6))]
        } else {
            b'='
        });
        output.push(if chunk.len() > 2 {
            BASE64[usize::from(third & 63)]
        } else {
            b'='
        });
    }
}

#[cfg(test)]
mod tests;

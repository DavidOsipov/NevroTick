// SPDX-License-Identifier: GPL-2.0-only
//! Reconstruct the original historical contradiction using retained frames and roots.

use super::super::EXCHANGES;
use super::{Conflict, ConflictEvidence, PEERS, Receipt, SelectedPeer};
use crate::roughtime::{PendingRequest, RootKey, VerifiedResponse, VerifyError, truncated_hash};
use std::net::SocketAddr;

#[cfg(test)]
mod tests;

/// A refusal to reconstruct the controlled round's original signed conflict.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ConflictProofError {
    /// The original prefix or a required later linking value is missing.
    Incomplete,
    /// The prefix length or claimed positions are outside the controlled shape.
    Shape,
    /// The observed endpoint or stored root differs from the originally selected slot.
    Identity {
        /// Zero-based position in the retained prefix.
        position: usize,
    },
    /// The selected root or one of the original frames fails keyed verification.
    Verification {
        /// Zero-based position in the retained prefix.
        position: usize,
        /// Exact bounded verifier refusal without packet or key bytes.
        cause: VerifyError,
    },
    /// The link is unexpected, repeated or inconsistent with the request nonce.
    Link {
        /// Zero-based position in the retained prefix.
        position: usize,
    },
    /// The verified prefix does not reproduce the exact first conflicting pair.
    Witness,
}

/// Borrowed proof of the first contradiction in one original controlled round.
///
/// This establishes historical cryptographic evidence. It grants no current
/// time, source, export or clock authority.
///
/// ```compile_fail,E0451
/// use nevrotick_engine::roughtime::round::recovery::{ConflictEvidence, ConflictProof};
/// fn bypass(evidence: &ConflictEvidence) -> ConflictProof<'_> {
///     ConflictProof { evidence }
/// }
/// ```
#[must_use = "retain the proof with its original conflict evidence"]
pub struct ConflictProof<'a> {
    /// The exact original evidence checked during construction.
    evidence: &'a ConflictEvidence,
}

impl<'a> ConflictProof<'a> {
    /// Borrow the original witnessed positions, selected peers and complete frames.
    #[must_use]
    pub const fn evidence(&self) -> &'a ConflictEvidence {
        self.evidence
    }
}

impl ConflictEvidence {
    /// Recheck every original frame, selected root and nonce link in the retained prefix.
    ///
    /// The historical check performs no I/O, observes no live clock and leaves
    /// the retained conflict unchanged on either result.
    ///
    /// # Errors
    /// Refuses missing, mismatched or invalid evidence and a prefix that does
    /// not reproduce the claimed first conflicting pair.
    pub fn reconstruct(&self) -> Result<ConflictProof<'_>, ConflictProofError> {
        verify_prefix(
            self.conflict,
            &self.selected,
            self.receipts.iter().map(ProofEntry::from),
        )?;
        Ok(ConflictProof { evidence: self })
    }
}

/// Borrow the actual received identity and original frame bytes without copying them.
#[derive(Clone, Copy)]
struct ProofEntry<'a> {
    /// Endpoint observed by the original transport.
    peer: SocketAddr,
    /// Root recorded by that receive opportunity.
    root: &'a [u8; 32],
    /// Complete original framed request.
    request: &'a [u8],
    /// Complete original framed response.
    response: &'a [u8],
    /// Random value linking this request to the preceding response, when present.
    random: Option<&'a [u8; 32]>,
}

impl<'a> From<&'a Receipt> for ProofEntry<'a> {
    fn from(receipt: &'a Receipt) -> Self {
        Self {
            peer: receipt.peer(),
            root: receipt.root_key_bytes(),
            request: receipt.request_frame(),
            response: receipt.response_frame(),
            random: receipt.chaining_randomness(),
        }
    }
}

/// Verify the finite original prefix and its claimed first conflicting pair.
///
/// # Errors
/// Refuses extra or missing entries, wrong identities, failed verification,
/// invalid or missing links, or a different causal outcome.
fn verify_prefix<'a>(
    conflict: Conflict,
    selected: &[SelectedPeer; PEERS],
    entries: impl ExactSizeIterator<Item = ProofEntry<'a>>,
) -> Result<(), ConflictProofError> {
    let (earlier, later): (usize, usize) = conflict.positions();
    let count: usize = entries.len();
    if later >= EXCHANGES || earlier >= later || count > later + 1 {
        return Err(ConflictProofError::Shape);
    }
    if count < later + 1 {
        return Err(ConflictProofError::Incomplete);
    }

    let mut lower_bounds: [u64; EXCHANGES] = [0; EXCHANGES];
    let mut links: [Option<&[u8; 32]>; EXCHANGES] = [None; EXCHANGES];
    let mut previous_response: Option<&[u8]> = None;
    for (position, entry) in entries.enumerate() {
        let expected: SelectedPeer = selected[position % PEERS];
        if entry.peer != expected.endpoint() || entry.root != expected.root_key_bytes() {
            return Err(ConflictProofError::Identity { position });
        }
        let refuse = |cause: VerifyError| ConflictProofError::Verification { position, cause };
        let root: RootKey = RootKey::from_bytes(expected.root_key_bytes()).map_err(refuse)?;
        let pending: PendingRequest<'_> =
            PendingRequest::new(entry.request, &root).map_err(refuse)?;
        match (previous_response, entry.random) {
            (Some(previous), Some(random)) => {
                if links[..position].contains(&Some(random))
                    || pending.request.nonce() != &truncated_hash(&[previous, random])
                {
                    return Err(ConflictProofError::Link { position });
                }
            }
            (Some(_), None) => return Err(ConflictProofError::Incomplete),
            (None, Some(_)) => return Err(ConflictProofError::Link { position }),
            (None, None) => {}
        }
        let verified: VerifiedResponse<'_, '_> = pending.verify(entry.response).map_err(refuse)?;
        let (lower, upper): (u64, u64) = verified.bounds_seconds();
        if let Some(reconstructed) =
            Conflict::against(position, lower_bounds[..position].iter().copied(), upper)
        {
            return if reconstructed == conflict {
                Ok(())
            } else {
                Err(ConflictProofError::Witness)
            };
        }
        lower_bounds[position] = lower;
        links[position] = entry.random;
        previous_response = Some(verified.frame());
    }
    Err(ConflictProofError::Witness)
}

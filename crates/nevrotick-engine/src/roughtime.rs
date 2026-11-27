// SPDX-License-Identifier: GPL-2.0-only
//! Verify an RFC 10049 v1 response against one original request and selected root.
//!
//! This boundary checks cryptographic evidence. The acquisition owner retains
//! responsibility for nonce generation, endpoint and key admission, the
//! original deadline, and current round and clock policy.

use ed25519_dalek::{Signature, VerifyingKey};
use ntp_wire::roughtime::{
    DecodeError, MAX_RESPONSE_BYTES, Request, Response, decode_request, decode_response,
};
use sha2::{Digest, Sha512};

pub mod exchange;
pub mod interval;
mod request;
pub mod round;

/// The v1 context prefix for the delegation signed by the long-term key.
const DELEGATION_CONTEXT: &[u8] = b"Roughtime v1 delegation signature\0";
/// The v1 context prefix for the response signed by the delegated key.
const RESPONSE_CONTEXT: &[u8] = b"Roughtime v1 response signature\0";

/// A refusal of the selected key or one request-bound response.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum VerifyError {
    /// The supplied long-term key is malformed, noncanonical, or weak.
    RootKey,
    /// The original request does not match the selected wire profile.
    Request(DecodeError),
    /// The request server commitment names a different long-term key.
    ServerCommitment,
    /// The response has invalid framing, fields, or numerical bounds.
    Response(DecodeError),
    /// The response does not echo the original nonce.
    Nonce,
    /// The delegation signature failed under the selected root.
    DelegationSignature,
    /// The signed delegated key is malformed, noncanonical, or weak.
    DelegatedKey,
    /// The response signature failed under the signed delegated key.
    ResponseSignature,
    /// The signed Merkle root does not contain the complete original request.
    RequestCommitment,
}

/// A checked Ed25519 key supplied by the caller as the selected long-term root.
///
/// Key decoding establishes point validity. Source and key-generation
/// authorization belong to the caller's admission policy.
pub struct RootKey(VerifyingKey);

impl RootKey {
    /// Check the exact ordinary Ed25519 public-key representation.
    ///
    /// # Errors
    /// Refuses malformed, noncanonical, or small-order points.
    pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self, VerifyError> {
        checked_key(bytes).map(Self).ok_or(VerifyError::RootKey)
    }
}

/// One verification opportunity for an immutable original request.
///
/// The caller retains custody of the original transmission. Reconstructing
/// a pending request from historical bytes cannot establish a fresh send.
#[must_use = "retain the original request until its response is verified"]
pub struct PendingRequest<'a> {
    /// The exact original framed request with the selected v1 fields.
    request: Request<'a>,
    /// The long-term root selected before response processing.
    root: &'a RootKey,
}

impl<'a> PendingRequest<'a> {
    /// Bind the exact original request to its independently selected root.
    ///
    /// # Errors
    /// Refuses invalid request fields or a server commitment for another root.
    ///
    /// ```compile_fail,E0061
    /// use nevrotick_engine::roughtime::PendingRequest;
    /// fn without_root(frame: &[u8]) {
    ///     drop(PendingRequest::new(frame));
    /// }
    /// ```
    pub fn new(frame: &'a [u8], root: &'a RootKey) -> Result<Self, VerifyError> {
        let request: Request<'a> = decode_request(frame).map_err(VerifyError::Request)?;
        let commitment: [u8; 32] = truncated_hash(&[&[0xff], root.0.as_bytes()]);
        if request.server_commitment() != &commitment {
            return Err(VerifyError::ServerCommitment);
        }
        Ok(Self { request, root })
    }

    /// Consume this opportunity and check one response against the original bytes.
    ///
    /// # Errors
    /// Refuses malformed responses, inconsistent nonces, invalid keys or
    /// signatures, and missing inclusion of the original request.
    ///
    /// ```compile_fail,E0382
    /// use nevrotick_engine::roughtime::PendingRequest;
    /// fn twice(pending: PendingRequest<'_>, packet: &[u8]) {
    ///     drop(pending.verify(packet));
    ///     drop(pending.verify(packet));
    /// }
    /// ```
    pub fn verify<'response>(
        self,
        frame: &'response [u8],
    ) -> Result<VerifiedResponse<'a, 'response>, VerifyError> {
        let response: Response<'response> =
            decode_response(frame).map_err(VerifyError::Response)?;
        if self.request.nonce() != response.nonce() {
            return Err(VerifyError::Nonce);
        }
        if !verify_signature(
            &self.root.0,
            DELEGATION_CONTEXT,
            response.signed_delegation(),
            response.delegation_signature(),
        ) {
            return Err(VerifyError::DelegationSignature);
        }
        let delegated: VerifyingKey =
            checked_key(response.delegated_key()).ok_or(VerifyError::DelegatedKey)?;
        if !verify_signature(
            &delegated,
            RESPONSE_CONTEXT,
            response.signed_response(),
            response.response_signature(),
        ) {
            return Err(VerifyError::ResponseSignature);
        }
        if response.merkle_root() != &request_root(self.request.frame(), &response) {
            return Err(VerifyError::RequestCommitment);
        }
        Ok(VerifiedResponse {
            request: self.request,
            response,
            root: self.root,
        })
    }
}

/// A signed v1 interval and its original request and response custody.
///
/// The interval is the selected server's signed claim under the chosen root.
/// A later admission step must check current authority, timing and agreement.
#[must_use = "retain the verified interval with its original frames and root"]
pub struct VerifiedResponse<'request, 'response> {
    /// The complete original request authenticated by Merkle inclusion.
    request: Request<'request>,
    /// The complete validated and authenticated response.
    response: Response<'response>,
    /// The selected long-term root that authenticated the delegation.
    root: &'request RootKey,
}

impl VerifiedResponse<'_, '_> {
    /// Borrow the original framed request.
    #[must_use]
    pub const fn request_frame(&self) -> &[u8] {
        self.request.frame()
    }

    /// Borrow the original framed response, including every byte used by chaining.
    #[must_use]
    pub const fn frame(&self) -> &[u8] {
        self.response.frame()
    }

    /// Return the endpoints of the signed open interval in Unix-epoch seconds.
    #[must_use]
    pub const fn bounds_seconds(&self) -> (u64, u64) {
        self.response.bounds_seconds()
    }

    /// Borrow the exact long-term public-key bytes bound to the verification.
    #[must_use]
    pub fn root_key_bytes(&self) -> &[u8; 32] {
        self.root.0.as_bytes()
    }
}

/// Check the actual compressed bytes and reject weak public-key points.
fn checked_key(bytes: &[u8; 32]) -> Option<VerifyingKey> {
    let key: VerifyingKey = VerifyingKey::from_bytes(bytes).ok()?;
    if key.is_weak() || key.to_edwards().compress().as_bytes() != bytes {
        return None;
    }
    Some(key)
}

/// Verify plain Ed25519 over the complete context and exact tag bytes.
///
/// The bounded buffer holds the largest possible decoded tag and context.
fn verify_signature(
    key: &VerifyingKey,
    context: &[u8],
    message: &[u8],
    signature: &[u8; 64],
) -> bool {
    let mut buffer: [u8; MAX_RESPONSE_BYTES + DELEGATION_CONTEXT.len()] =
        [0; MAX_RESPONSE_BYTES + DELEGATION_CONTEXT.len()];
    let Some(length): Option<usize> = context.len().checked_add(message.len()) else {
        return false;
    };
    let Some(input): Option<&mut [u8]> = buffer.get_mut(..length) else {
        return false;
    };
    let (prefix, value): (&mut [u8], &mut [u8]) = input.split_at_mut(context.len());
    prefix.copy_from_slice(context);
    value.copy_from_slice(message);
    key.verify_strict(input, &Signature::from_bytes(signature))
        .is_ok()
}

/// Hash the framed request as a leaf and combine the decoded siblings in index order.
fn request_root(frame: &[u8], response: &Response<'_>) -> [u8; 32] {
    let mut current: [u8; 32] = truncated_hash(&[&[0x00], frame]);
    let mut index: u32 = response.merkle_index();
    for sibling in response.merkle_path() {
        let (left, right): (&[u8; 32], &[u8; 32]) = if index & 1 == 0 {
            (&current, sibling)
        } else {
            (sibling, &current)
        };
        current = truncated_hash(&[&[0x01], left, right]);
        index >>= 1;
    }
    current
}

/// Hash each segment in order and take the first 32 SHA-512 output bytes.
fn truncated_hash(parts: &[&[u8]]) -> [u8; 32] {
    let mut state: Sha512 = Sha512::new();
    for part in parts {
        state.update(part);
    }
    let digest: [u8; 64] = state.finalize().into();
    let mut output: [u8; 32] = [0; 32];
    output.copy_from_slice(&digest[..32]);
    output
}

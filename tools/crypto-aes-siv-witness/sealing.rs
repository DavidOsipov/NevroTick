// SPDX-License-Identifier: GPL-2.0-only
//! Laboratory algorithm-15 sealing through the existing inert request formatter.
//!
//! This narrow experiment supports an empty encrypted extension-field sequence
//! and a supplied 16-byte nonce. Supplied keys, nonce, UID and cookie carry no
//! entropy, TLS, association, permission, freshness or send-authority guarantee.
//! No network, allocation, RNG, logging or Windows operation occurs here. The
//! provider and main-product adoption remain held by architecture section 18.7.3.

use aes_siv::{KeyInit, siv::Aes128Siv};
use ntp_wire::{
    FrameLimits, NtsClientRequest, NtsRequestEncodeError, NtsRequestShape, PreparedNtsRequest,
    RawHeader, prepare_nts_client_request,
};

/// Redacted sealing failures; no key, cookie, nonce, UID or packet is retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestSealError {
    /// The maintained AEAD implementation refused the fixed-shape operation.
    Encryption,
    /// The existing formatter refused finalization before writing its tail.
    Format(NtsRequestEncodeError),
}

/// Exclusive prepared storage whose AEAD dimensions were derived internally.
///
/// Dropping this owner leaves the formatter's prepared prefix and untouched tail,
/// without publishing completed bytes. It exposes no raw plan, mutable AAD,
/// payload-bearing `Debug`, clone, default or unchecked constructor.
///
/// ```compile_fail,E0451
/// use nevrotick_s115_aes_siv_fixture::sealing::PreparedAlgorithm15Request;
/// use ntp_wire::PreparedNtsRequest;
/// fn bypass(plan: PreparedNtsRequest<'_>) {
///     let _ = PreparedAlgorithm15Request { plan };
/// }
/// ```
/// A prepared owner cannot be sealed twice:
///
/// ```compile_fail,E0382
/// use nevrotick_s115_aes_siv_fixture::sealing::PreparedAlgorithm15Request;
/// fn twice(plan: PreparedAlgorithm15Request<'_>) {
///     let _ = plan.seal(&[0; 32], &[0; 16]);
///     let _ = plan.seal(&[0; 32], &[0; 16]);
/// }
/// ```
/// Neither a short key nor a short nonce can enter this profile:
///
/// ```compile_fail,E0308
/// use nevrotick_s115_aes_siv_fixture::sealing::PreparedAlgorithm15Request;
/// fn key_size(plan: PreparedAlgorithm15Request<'_>) {
///     let _ = plan.seal(&[0; 31], &[0; 16]);
/// }
/// ```
/// ```compile_fail,E0308
/// use nevrotick_s115_aes_siv_fixture::sealing::PreparedAlgorithm15Request;
/// fn nonce_size(plan: PreparedAlgorithm15Request<'_>) {
///     let _ = plan.seal(&[0; 32], &[0; 15]);
/// }
/// ```
/// Prepared packet contents have no payload-bearing debug formatter:
///
/// ```compile_fail,E0277
/// use nevrotick_s115_aes_siv_fixture::sealing::PreparedAlgorithm15Request;
/// fn leak(plan: PreparedAlgorithm15Request<'_>) { let _ = format!("{plan:?}"); }
/// ```
pub struct PreparedAlgorithm15Request<'a> {
    /// The actual wire owner retains the destination's exclusive mutable borrow.
    plan: PreparedNtsRequest<'a>,
}

impl<'a> PreparedAlgorithm15Request<'a> {
    /// Authenticate the exact prepared AAD followed by nonce and finish once.
    ///
    /// The supplied algorithm-15 key is exactly 32 bytes. This fixed laboratory
    /// profile encrypts no extension fields, so its complete ciphertext is the
    /// 16-byte SIV tag. A 16-byte supplied nonce is a selected outgoing shape,
    /// not the algorithm's maximum or evidence of uniqueness. Empty plaintext
    /// has no CTR output; tag tests therefore cannot detect changes to K2 alone.
    /// The complete supplied key enters the maintained provider unchanged.
    ///
    /// Successful output is the existing immutable borrowed wire view. It grants
    /// no authenticated-association, cookie-use, generation or send authority.
    /// Key storage remains caller-owned; provider key-schedule zeroization and
    /// elimination of all prior copies/abort state remain separate obligations.
    ///
    /// # Errors
    /// Refuse encryption or finalization without returning completed bytes.
    /// Preparation already wrote its prefix; a later refusal does not promise
    /// whole-destination rollback. The formatter preserves its tail on refusal.
    pub fn seal(
        self,
        key: &[u8; 32],
        nonce: &[u8; 16],
    ) -> Result<NtsClientRequest<'a>, RequestSealError> {
        let mut cipher = Aes128Siv::new(key.into());
        let mut plaintext = [];
        let tag = cipher
            .encrypt_inout_detached(
                [self.plan.associated_data(), nonce.as_slice()],
                (&mut plaintext[..]).into(),
            )
            .map_err(|_| RequestSealError::Encryption)?;
        self.plan
            .finish(nonce, tag.as_slice())
            .map_err(RequestSealError::Format)
    }
}

/// Preflight the entire declared algorithm-15 request before writing its AAD.
///
/// Reuse the wire owner's v4/client header, exact UID/cookie shape, placeholder,
/// byte/field budget and capacity checks. The private returned owner fixes nonce
/// and ciphertext lengths at 16 bytes each. RFC 5297's unbounded `N_MAX` maps to the
/// largest representable allowance; the formatter's `min(16, N_MAX)` then gives
/// the exact RFC 8915 additional-padding requirement. Its existing u16 and packet
/// limits remain enforced. This profile restricts neither the wire reader nor
/// the separate staged decoder's received nonce shapes.
///
/// No allocation or effects beyond writing the checked AAD prefix occur. Cookie
/// provenance, reservation, nonce/UID entropy and final current-send admission
/// remain with their actual future owners.
///
/// # Errors
/// Any shape, header, budget or capacity refusal leaves caller storage unchanged.
///
/// A legal external consumer can seal, inspect and then reuse its storage:
///
/// ```
/// use nevrotick_s115_aes_siv_fixture::sealing::prepare_algorithm15_request;
/// use ntp_wire::{FrameLimits, RawHeader};
/// pub fn use_storage(header: &RawHeader, limits: FrameLimits) -> Result<(), &'static str> {
///     let mut storage = [0; 256];
///     {
///         let plan = prepare_algorithm15_request(
///             header, &[1; 32], &[2; 12], 0, &mut storage, limits)
///             .map_err(|_| "prepare")?;
///         let request = plan.seal(&[3; 32], &[4; 16]).map_err(|_| "seal")?;
///         assert_eq!(request.packet_bytes().len(), 140);
///     }
///     storage.fill(0);
///     Ok(())
/// }
/// ```
/// A prepared owner keeps storage exclusively borrowed through sealing:
///
/// ```compile_fail,E0506
/// use nevrotick_s115_aes_siv_fixture::sealing::prepare_algorithm15_request;
/// use ntp_wire::{FrameLimits, RawHeader};
/// fn alias(header: &RawHeader, limits: FrameLimits) {
///     let mut storage = [0; 256];
///     let plan = prepare_algorithm15_request(
///         header, &[1; 32], &[2; 12], 0, &mut storage, limits).unwrap();
///     storage[0] = 0;
///     let _ = plan.seal(&[3; 32], &[4; 16]);
/// }
/// ```
/// Completed bytes cannot escape the destination's lifetime:
///
/// ```compile_fail,E0515
/// use nevrotick_s115_aes_siv_fixture::sealing::{prepare_algorithm15_request, RequestSealError};
/// use ntp_wire::{FrameLimits, NtsClientRequest, RawHeader};
/// fn escape(header: &RawHeader, limits: FrameLimits)
///     -> Result<NtsClientRequest<'static>, RequestSealError>
/// {
///     let mut storage = [0; 256];
///     prepare_algorithm15_request(header, &[1; 32], &[2; 12], 0, &mut storage, limits)
///         .map_err(RequestSealError::Format)?.seal(&[3; 32], &[4; 16])
/// }
/// ```
pub fn prepare_algorithm15_request<'a>(
    header: &RawHeader,
    uid: &[u8],
    cookie: &[u8],
    placeholder_count: usize,
    storage: &'a mut [u8],
    limits: FrameLimits,
) -> Result<PreparedAlgorithm15Request<'a>, NtsRequestEncodeError> {
    let shape = NtsRequestShape::new(16, 16, usize::MAX)?;
    let plan = prepare_nts_client_request(
        header,
        uid,
        cookie,
        placeholder_count,
        shape,
        storage,
        limits,
    )?;
    Ok(PreparedAlgorithm15Request { plan })
}

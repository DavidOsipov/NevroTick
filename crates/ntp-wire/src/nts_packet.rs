// SPDX-License-Identifier: GPL-2.0-only
//! Inert RFC 8915 sections 5.2/5.6/5.7 authenticator-envelope projections.
//!
//! Complete outer framing precedes partitioning at the first authenticator.
//! The returned bytes have no authentication, mode/version, UID, key, endpoint,
//! cookie, or sample authority. Correlated NTS NAK/KoD handling is a separate
//! control-only path; absence of this envelope is not a product-wide rejection.

use crate::{FIXED_HEADER_LEN, FrameError, FrameLimits, frame_no_legacy_mac_datagram};

/// Fixed redacted structural errors without packet, cookie, or key material.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NtsAuthenticatorError {
    /// The complete datagram failed the existing outer-framing contract.
    Frame(FrameError),
    /// No authenticator field exists; separate NAK/KoD handling remains possible.
    MissingAuthenticator,
    /// Declared nonce/ciphertext lengths and their rounded spans do not fit.
    InvalidAuthenticatorLength,
}

/// Immutable original-input slices from one structurally admitted envelope.
///
/// Only [`read_nts_authenticator_parts`] constructs this value. The exact prefix
/// is candidate AEAD associated data, not evidence of successful authentication.
/// Nonce and ciphertext exclude their alignment padding. No suffix is exposed.
/// All slices remain tied to the immutable input, independently of this view.
///
/// ```
/// use ntp_wire::{FrameLimits, NtsAuthenticatorError, read_nts_authenticator_parts};
/// # fn main() -> Result<(), NtsAuthenticatorError> {
/// let mut input = [0_u8; 76];
/// input[48..52].copy_from_slice(&[4, 4, 0, 28]);
/// let limits = FrameLimits::new(76, 1).map_err(NtsAuthenticatorError::Frame)?;
/// let aad = {
///     let parts = read_nts_authenticator_parts(&input, limits)?;
///     assert!(parts.nonce().is_empty());
///     assert!(parts.ciphertext().is_empty());
///     parts.associated_data()
/// };
/// assert_eq!(aad, &input[..48]);
/// # Ok(())
/// # }
/// ```
///
/// ```compile_fail,E0451
/// use ntp_wire::NtsAuthenticatorParts;
/// let _ = NtsAuthenticatorParts { associated_data: &[], nonce: &[], ciphertext: &[] };
/// ```
///
/// ```compile_fail,E0616
/// use ntp_wire::NtsAuthenticatorParts;
/// fn replace(parts: &mut NtsAuthenticatorParts<'_>) { parts.nonce = &[]; }
/// ```
///
/// ```compile_fail,E0277
/// use ntp_wire::NtsAuthenticatorParts;
/// let _: NtsAuthenticatorParts<'static> = Default::default();
/// ```
///
/// ```compile_fail,E0277
/// use ntp_wire::NtsAuthenticatorParts;
/// fn needs_debug<T: core::fmt::Debug>() {}
/// needs_debug::<NtsAuthenticatorParts<'static>>();
/// ```
///
/// ```compile_fail,E0506
/// use ntp_wire::{FrameLimits, NtsAuthenticatorError, read_nts_authenticator_parts};
/// # fn main() -> Result<(), NtsAuthenticatorError> {
/// let mut input = [0_u8; 76];
/// input[48..52].copy_from_slice(&[4, 4, 0, 28]);
/// let limits = FrameLimits::new(76, 1).map_err(NtsAuthenticatorError::Frame)?;
/// let parts = read_nts_authenticator_parts(&input, limits)?;
/// input[0] = 1;
/// assert_eq!(parts.associated_data()[0], 0);
/// # Ok(())
/// # }
/// ```
///
/// ```compile_fail,E0515
/// use ntp_wire::{FrameLimits, NtsAuthenticatorError, NtsAuthenticatorParts,
///     read_nts_authenticator_parts};
/// fn escape() -> Result<NtsAuthenticatorParts<'static>, NtsAuthenticatorError> {
///     let mut input = [0_u8; 76];
///     input[48..52].copy_from_slice(&[4, 4, 0, 28]);
///     let limits = FrameLimits::new(76, 1).map_err(NtsAuthenticatorError::Frame)?;
///     read_nts_authenticator_parts(&input, limits)
/// }
/// ```
pub struct NtsAuthenticatorParts<'a> {
    /// Exact contiguous input prefix ending before the first authenticator.
    associated_data: &'a [u8],
    /// Declared nonce octets, without alignment padding or algorithm admission.
    nonce: &'a [u8],
    /// Declared opaque ciphertext octets, without alignment padding or decryption.
    ciphertext: &'a [u8],
}

impl<'a> NtsAuthenticatorParts<'a> {
    /// Borrow the exact original candidate associated-data prefix.
    #[must_use]
    pub const fn associated_data(&self) -> &'a [u8] {
        self.associated_data
    }

    /// Borrow only the declared nonce; zero length is structurally representable.
    ///
    /// ```compile_fail,E0594
    /// use ntp_wire::NtsAuthenticatorParts;
    /// fn replace(parts: &NtsAuthenticatorParts<'_>) { parts.nonce()[0] = 1; }
    /// ```
    #[must_use]
    pub const fn nonce(&self) -> &'a [u8] {
        self.nonce
    }

    /// Borrow opaque ciphertext; its inner fields use a different RFC grammar.
    #[must_use]
    pub const fn ciphertext(&self) -> &'a [u8] {
        self.ciphertext
    }
}

/// Frame a complete datagram, then partition its first type-0x0404 envelope.
///
/// The complete existing no-legacy-MAC framer runs before any projection. Its
/// explicit byte/count ceilings bound all work to two traversals of at most 64
/// fields and constant-size output metadata. Well-framed trailing fields,
/// including another authenticator with arbitrary contents, have no influence
/// on the returned slices. Ciphertext is opaque and is never outer-framed.
///
/// This structural reader permits zero lengths and ignores all padding values.
/// RFC 8915 section 5.6 prescribes sender zero-padding and algorithm-dependent
/// requirements; accepting these bytes here is not an RFC-conformance claim.
/// Mode/version, AEAD nonce/tag limits, padding policy, authentication and the
/// separate correlated NAK/KoD path belong to later owners. No heap, I/O, unsafe
/// code, mutation or retained state is introduced; safe borrows guard input life.
///
/// # Errors
/// Return an outer [`FrameError`] first, then absence of an authenticator or
/// lengths whose four-octet-rounded ranges exceed its completely framed body.
pub fn read_nts_authenticator_parts(
    input: &[u8],
    limits: FrameLimits,
) -> Result<NtsAuthenticatorParts<'_>, NtsAuthenticatorError> {
    let datagram =
        frame_no_legacy_mac_datagram(input, limits).map_err(NtsAuthenticatorError::Frame)?;
    let mut offset = FIXED_HEADER_LEN;
    for field in datagram.extensions() {
        if field.field_type() == 0x0404 {
            let body = field.body();
            let lengths = body
                .first_chunk::<4>()
                .ok_or(NtsAuthenticatorError::InvalidAuthenticatorLength)?;
            let nonce_len = u16::from_be_bytes([lengths[0], lengths[1]]);
            let ciphertext_len = u16::from_be_bytes([lengths[2], lengths[3]]);
            let components = body
                .get(4..)
                .ok_or(NtsAuthenticatorError::InvalidAuthenticatorLength)?;
            let (nonce_block, remaining) = components
                .split_at_checked(padded_span(nonce_len)?)
                .ok_or(NtsAuthenticatorError::InvalidAuthenticatorLength)?;
            let (ciphertext_block, _padding) = remaining
                .split_at_checked(padded_span(ciphertext_len)?)
                .ok_or(NtsAuthenticatorError::InvalidAuthenticatorLength)?;
            let nonce = nonce_block
                .get(..usize::from(nonce_len))
                .ok_or(NtsAuthenticatorError::InvalidAuthenticatorLength)?;
            let ciphertext = ciphertext_block
                .get(..usize::from(ciphertext_len))
                .ok_or(NtsAuthenticatorError::InvalidAuthenticatorLength)?;
            let associated_data = input
                .get(..offset)
                .ok_or(NtsAuthenticatorError::InvalidAuthenticatorLength)?;
            return Ok(NtsAuthenticatorParts {
                associated_data,
                nonce,
                ciphertext,
            });
        }
        offset = offset
            .checked_add(field.bytes().len())
            .ok_or(NtsAuthenticatorError::InvalidAuthenticatorLength)?;
    }
    Err(NtsAuthenticatorError::MissingAuthenticator)
}

/// Round an unsigned wire length up to a four-octet span with checked arithmetic.
///
/// Every `u16` input is representable; no AEAD-specific length is admitted.
/// The result is at most 65,536 and is checked against the actual body by its
/// mandatory caller before slicing. No padding byte or external state is read.
///
/// # Errors
/// Return a redacted length failure if the target cannot represent the addition.
fn padded_span(length: u16) -> Result<usize, NtsAuthenticatorError> {
    usize::from(length)
        .checked_add(3)
        .map(|value| value & !3)
        .ok_or(NtsAuthenticatorError::InvalidAuthenticatorLength)
}

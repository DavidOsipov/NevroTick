// SPDX-License-Identifier: GPL-2.0-only
//! Bounded two-stage inert NTS client request serialization.
//!
//! RFC 8915 sections 5.3–5.7 define the byte layout and additional padding.
//! Preparation owns caller storage, admits the complete declared layout before
//! writing, and exposes only immutable associated data. Consuming finalization
//! checks both supplied lengths before writing the authenticator. Neither stage
//! authenticates anything, selects an algorithm, reserves a cookie or sends data.
//! The future engine/AEAD owners supply entropy, truthful algorithm dimensions,
//! keys, ciphertext and current association/send authority. No heap, I/O or unsafe.

use crate::{FIXED_HEADER_LEN, FrameError, FrameLimits, RawHeader};

/// Payload-free formatting refusals. Failure preserves that stage's input buffer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NtsRequestEncodeError {
    /// Only NTP version 4, client mode 3 is supported by this request writer.
    InvalidClientHeader,
    /// Exact-copy UIDs must be word-aligned, at least 32 bytes and fit a field.
    UnsupportedUniqueIdentifier,
    /// Exact-copy opaque cookies must be word-aligned, at least 12 bytes and fit a field.
    UnsupportedCookie,
    /// The nonce length exceeds its explicit allowance or the encoded u16 length.
    InvalidNonceLength,
    /// The ciphertext length exceeds the encoded u16 length.
    InvalidCiphertextLength,
    /// The complete padded authenticator exceeds its encoded u16 length.
    AuthenticatorTooLong,
    /// The complete packet exceeds the explicit byte or field budget.
    FrameLimit(FrameError),
    /// Caller storage cannot hold the entire declared request.
    OutputTooSmall,
    /// Supplied nonce length differs from the frozen declaration.
    NonceLengthMismatch,
    /// Supplied ciphertext length differs from the frozen declaration.
    CiphertextLengthMismatch,
}

/// Checked immutable lengths, without an algorithm/default or cryptographic claim.
///
/// `maximum_nonce_length` must come from the separately admitted AEAD owner.
/// Accepting this declaration does not establish its truth or nonce uniqueness.
/// Zero lengths remain structurally representable; the AEAD owner must enforce
/// its actual nonce, tag and plaintext rules before integration.
///
/// ```compile_fail,E0451
/// use ntp_wire::NtsRequestShape;
/// let _ = NtsRequestShape { nonce_length: 0, ciphertext_length: 0,
///     nonce_width: 0, authenticator_length: 0 };
/// ```
/// ```compile_fail,E0616
/// use ntp_wire::NtsRequestShape;
/// fn mutate(mut shape: NtsRequestShape) { shape.nonce_length = 0; }
/// ```
/// ```compile_fail,E0277
/// use ntp_wire::NtsRequestShape;
/// let _: NtsRequestShape = Default::default();
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NtsRequestShape {
    /// Unpadded nonce length, checked against the explicit negotiated allowance.
    nonce_length: u16,
    /// Unpadded opaque AEAD output length.
    ciphertext_length: u16,
    /// Word-aligned nonce storage; its padding is always written as zero.
    nonce_width: usize,
    /// Complete word-aligned final field, including its header and all padding.
    authenticator_length: u16,
}

impl NtsRequestShape {
    /// Check dimensions and freeze the complete padded authenticator shape.
    ///
    /// Reserve enough trailing zero padding for both the RFC 8915 section 5.6
    /// nonce allowance and RFC 7822's 28-byte minimum final-field length. Work is
    /// constant, allocates nothing and has no effects. The packet budget is
    /// checked later together with the exact UID, cookie and placeholder count.
    /// # Errors
    /// Refuse an over-allowance/unencodable nonce, unencodable ciphertext or
    /// unencodable complete field, in that order. No unchecked shape is returned.
    pub fn new(
        nonce_length: usize,
        ciphertext_length: usize,
        maximum_nonce_length: usize,
    ) -> Result<Self, NtsRequestEncodeError> {
        if nonce_length > maximum_nonce_length {
            return Err(NtsRequestEncodeError::InvalidNonceLength);
        }
        let nonce_length =
            u16::try_from(nonce_length).map_err(|_| NtsRequestEncodeError::InvalidNonceLength)?;
        let ciphertext_length = u16::try_from(ciphertext_length)
            .map_err(|_| NtsRequestEncodeError::InvalidCiphertextLength)?;
        // Both encoded lengths are u16 and the required allowance is at most 16:
        // these additions fit usize on every supported target, before narrowing.
        let nonce_width = (usize::from(nonce_length) + 3) & !3;
        let ciphertext_width = (usize::from(ciphertext_length) + 3) & !3;
        let required_nonce_width = (maximum_nonce_length.min(16) + 3) & !3;
        let authenticator_length =
            (8 + nonce_width.max(required_nonce_width) + ciphertext_width).max(28);
        let authenticator_length = u16::try_from(authenticator_length)
            .map_err(|_| NtsRequestEncodeError::AuthenticatorTooLong)?;
        Ok(Self {
            nonce_length,
            ciphertext_length,
            nonce_width,
            authenticator_length,
        })
    }
}

/// Exclusive prepared destination; only its immutable AAD prefix is exposed.
///
/// Dropping this plan leaves the prepared prefix in caller storage and the tail
/// untouched, without returning a completed request. No payload-bearing Debug,
/// clone, unchecked construction, mutation or default is provided.
///
/// ```compile_fail,E0451
/// use ntp_wire::{NtsRequestShape, PreparedNtsRequest};
/// fn bypass(bytes: &mut [u8], shape: NtsRequestShape) {
///     let _ = PreparedNtsRequest { packet: bytes, aad_length: 0, shape };
/// }
/// ```
/// ```compile_fail,E0382
/// use ntp_wire::PreparedNtsRequest;
/// fn twice(plan: PreparedNtsRequest<'_>) {
///     let _ = plan.finish(&[], &[]);
///     let _ = plan.finish(&[], &[]);
/// }
/// ```
/// The AAD borrow must end before consuming finalization can mutate the tail:
/// ```compile_fail,E0505
/// use ntp_wire::PreparedNtsRequest;
/// fn retain_aad(plan: PreparedNtsRequest<'_>) {
///     let aad = plan.associated_data();
///     let _ = plan.finish(&[], &[]);
///     let _ = aad.len();
/// }
/// ```
/// ```compile_fail,E0594
/// use ntp_wire::PreparedNtsRequest;
/// fn mutate_aad(plan: &PreparedNtsRequest<'_>) { plan.associated_data()[0] = 0; }
/// ```
/// ```compile_fail,E0277
/// use ntp_wire::PreparedNtsRequest;
/// let _: PreparedNtsRequest<'_> = Default::default();
/// ```
/// ```compile_fail,E0277
/// use ntp_wire::PreparedNtsRequest;
/// fn leak(plan: PreparedNtsRequest<'_>) { let _ = format!("{plan:?}"); }
/// ```
pub struct PreparedNtsRequest<'a> {
    /// Exactly the admitted complete packet span, exclusively borrowed.
    packet: &'a mut [u8],
    /// Complete admitted prefix boundary, strictly before the authenticator header.
    aad_length: usize,
    /// Checked lengths whose complete authenticator span is reserved in packet.
    shape: NtsRequestShape,
}

impl<'a> PreparedNtsRequest<'a> {
    /// Borrow the exact prefix to authenticate, excluding the authenticator header.
    /// The borrow is tied to this plan, never permitting concurrent finalization.
    #[must_use]
    pub fn associated_data(&self) -> &[u8] {
        &self.packet[..self.aad_length]
    }

    /// Consume this plan, check actual lengths, then write the complete final field.
    ///
    /// Exact nonce/ciphertext bytes are copied and every padding byte is zeroed.
    /// Success returns only the admitted packet span, retaining caller-storage
    /// lifetime but no mutable access or cryptographic authority. Work is bounded
    /// by the admitted packet, with no heap, I/O or other effects.
    /// # Errors
    /// Check nonce length before ciphertext length. Either refusal leaves the
    /// prepared prefix and untouched reserved tail byte-identical, and consumes
    /// the plan. No fallible operation follows the first write.
    pub fn finish(
        self,
        nonce: &[u8],
        ciphertext: &[u8],
    ) -> Result<NtsClientRequest<'a>, NtsRequestEncodeError> {
        if nonce.len() != usize::from(self.shape.nonce_length) {
            return Err(NtsRequestEncodeError::NonceLengthMismatch);
        }
        if ciphertext.len() != usize::from(self.shape.ciphertext_length) {
            return Err(NtsRequestEncodeError::CiphertextLengthMismatch);
        }
        let field = &mut self.packet[self.aad_length..];
        field.fill(0);
        let [length_hi, length_lo] = self.shape.authenticator_length.to_be_bytes();
        let [nonce_hi, nonce_lo] = self.shape.nonce_length.to_be_bytes();
        let [cipher_hi, cipher_lo] = self.shape.ciphertext_length.to_be_bytes();
        field[..8].copy_from_slice(&[
            4, 4, length_hi, length_lo, nonce_hi, nonce_lo, cipher_hi, cipher_lo,
        ]);
        field[8..8 + nonce.len()].copy_from_slice(nonce);
        let ciphertext_start = 8 + self.shape.nonce_width;
        field[ciphertext_start..ciphertext_start + ciphertext.len()].copy_from_slice(ciphertext);
        Ok(NtsClientRequest {
            packet: self.packet,
        })
    }
}

/// Complete immutable inert request bytes, carrying no authentication authority.
///
/// Only consuming finalization constructs this view. It borrows caller storage;
/// no payload-bearing Debug, default, mutation or raw construction is available.
/// ```compile_fail,E0451
/// use ntp_wire::NtsClientRequest;
/// let _ = NtsClientRequest { packet: &[0; 128] };
/// ```
/// ```compile_fail,E0277
/// use ntp_wire::NtsClientRequest;
/// fn leak(packet: NtsClientRequest<'_>) { let _ = format!("{packet:?}"); }
/// ```
/// ```compile_fail,E0616
/// use ntp_wire::NtsClientRequest;
/// fn mutate(mut packet: NtsClientRequest<'_>) { packet.packet = &[]; }
/// ```
/// ```compile_fail
/// use ntp_wire::NtsClientRequest;
/// fn escape(packet: NtsClientRequest<'_>) -> &'static [u8] { packet.packet_bytes() }
/// ```
/// ```compile_fail,E0277
/// use ntp_wire::NtsClientRequest;
/// let _: NtsClientRequest<'_> = Default::default();
/// ```
pub struct NtsClientRequest<'a> {
    /// Exact originally reserved packet prefix, now completely written.
    packet: &'a [u8],
}

impl<'a> NtsClientRequest<'a> {
    /// Borrow the complete packet; view destruction does not end storage's lifetime.
    #[must_use]
    pub const fn packet_bytes(&self) -> &'a [u8] {
        self.packet
    }
}

/// Prepare header → UID → Cookie → zero placeholders, reserving the final field.
///
/// The complete declared layout, budgets and output capacity are admitted before
/// the first write. UID and Cookie are copied exactly, never padded/normalized;
/// placeholder bodies have exactly the Cookie length and contain only zeroes.
/// The authenticator and any destination suffix remain untouched. Inputs may
/// expire after preparation because only the destination is borrowed by the plan.
/// No heap, I/O, provider selection or association effect occurs.
/// # Errors
/// Refuse a non-v4/client header, unsupported exact-copy UID/Cookie shape,
/// unrepresentable/excessive packet/count or insufficient capacity. Failure
/// leaves the entire destination byte-identical and publishes no plan.
///
/// ```
/// use ntp_wire::{FrameLimits, NtsRequestEncodeError, NtsRequestShape,
///     decode_fixed_header, prepare_nts_client_request};
/// # fn main() -> Result<(), NtsRequestEncodeError> {
/// let mut header_bytes = [0; 48];
/// header_bytes[0] = 0x23;
/// let header = decode_fixed_header(&header_bytes)
///     .map_err(|_| NtsRequestEncodeError::InvalidClientHeader)?;
/// let limits = FrameLimits::new(128, 3).map_err(NtsRequestEncodeError::FrameLimit)?;
/// let shape = NtsRequestShape::new(0, 0, 0)?;
/// let mut storage = [0xa5; 128];
/// let plan = prepare_nts_client_request(
///     &header, &[0x11; 32], &[0x22; 12], 0, shape, &mut storage, limits)?;
/// assert_eq!(plan.associated_data().len(), 100);
/// let request = plan.finish(&[], &[])?;
/// assert_eq!(request.packet_bytes().len(), 128);
/// # Ok(())
/// # }
/// ```
///
/// The destination cannot be mutated while the prepared plan is still used:
/// ```compile_fail,E0499
/// use ntp_wire::{FrameLimits, NtsRequestEncodeError, NtsRequestShape, RawHeader,
///     prepare_nts_client_request};
/// fn mutate_storage(header: &RawHeader, shape: NtsRequestShape, limits: FrameLimits)
///     -> Result<(), NtsRequestEncodeError> {
///     let mut storage = [0; 128];
///     let plan = prepare_nts_client_request(
///         header, &[0x11; 32], &[0x22; 12], 0, shape, &mut storage, limits)?;
///     storage.fill(0);
///     let _ = plan.associated_data();
///     Ok(())
/// }
/// ```
pub fn prepare_nts_client_request<'a>(
    header: &RawHeader,
    unique_identifier: &[u8],
    cookie: &[u8],
    placeholder_count: usize,
    shape: NtsRequestShape,
    output: &'a mut [u8],
    limits: FrameLimits,
) -> Result<PreparedNtsRequest<'a>, NtsRequestEncodeError> {
    if header.version() != 4 || header.mode() != 3 {
        return Err(NtsRequestEncodeError::InvalidClientHeader);
    }
    if unique_identifier.len() < 32 || !unique_identifier.len().is_multiple_of(4) {
        return Err(NtsRequestEncodeError::UnsupportedUniqueIdentifier);
    }
    let uid_field_length = unique_identifier
        .len()
        .checked_add(4)
        .and_then(|length| u16::try_from(length).ok())
        .ok_or(NtsRequestEncodeError::UnsupportedUniqueIdentifier)?;
    if cookie.len() < 12 || !cookie.len().is_multiple_of(4) {
        return Err(NtsRequestEncodeError::UnsupportedCookie);
    }
    let cookie_field_length = cookie
        .len()
        .checked_add(4)
        .and_then(|length| u16::try_from(length).ok())
        .ok_or(NtsRequestEncodeError::UnsupportedCookie)?;
    let field_count = placeholder_count
        .checked_add(3)
        .ok_or(NtsRequestEncodeError::FrameLimit(
            FrameError::TooManyExtensions,
        ))?;
    let cookie_fields_bytes = (field_count - 2)
        .checked_mul(usize::from(cookie_field_length))
        .ok_or(NtsRequestEncodeError::FrameLimit(FrameError::ByteLimit))?;
    let aad_length = FIXED_HEADER_LEN
        .checked_add(usize::from(uid_field_length))
        .and_then(|length| length.checked_add(cookie_fields_bytes))
        .ok_or(NtsRequestEncodeError::FrameLimit(FrameError::ByteLimit))?;
    let packet_length = aad_length
        .checked_add(usize::from(shape.authenticator_length))
        .ok_or(NtsRequestEncodeError::FrameLimit(FrameError::ByteLimit))?;
    limits
        .check_message_budget(packet_length, field_count)
        .map_err(NtsRequestEncodeError::FrameLimit)?;
    let packet = output
        .get_mut(..packet_length)
        .ok_or(NtsRequestEncodeError::OutputTooSmall)?;
    let (aad, _) = packet.split_at_mut(aad_length);
    let (header_bytes, fields) = aad.split_at_mut(FIXED_HEADER_LEN);
    header_bytes.copy_from_slice(header.wire_bytes());

    let (uid, fields) = fields.split_at_mut(usize::from(uid_field_length));
    let [uid_hi, uid_lo] = uid_field_length.to_be_bytes();
    uid[..4].copy_from_slice(&[1, 4, uid_hi, uid_lo]);
    uid[4..].copy_from_slice(unique_identifier);

    let (cookie_field, placeholders) = fields.split_at_mut(usize::from(cookie_field_length));
    let [cookie_hi, cookie_lo] = cookie_field_length.to_be_bytes();
    cookie_field[..4].copy_from_slice(&[2, 4, cookie_hi, cookie_lo]);
    cookie_field[4..].copy_from_slice(cookie);
    for placeholder in placeholders.chunks_exact_mut(usize::from(cookie_field_length)) {
        placeholder[..4].copy_from_slice(&[3, 4, cookie_hi, cookie_lo]);
        placeholder[4..].fill(0);
    }
    Ok(PreparedNtsRequest {
        packet,
        aad_length,
        shape,
    })
}

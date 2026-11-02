// SPDX-License-Identifier: GPL-2.0-only
//! Inert framing of the NTS encrypted-extension plaintext sequence.
//!
//! The complete sequence is validated before any fields are exposed. This
//! parser neither decrypts nor authenticates its input and grants no cookie,
//! association, endpoint, sample or time authority. Field bodies include all
//! opaque type-specific bytes, including any padding.

/// Fixed structural errors without plaintext or cookie data.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NtsPlaintextError {
    /// The explicit experimental byte or field ceiling is out of range.
    InvalidLimits,
    /// The complete input exceeds the supplied byte ceiling.
    ByteLimit,
    /// Another field is present after the supplied field ceiling is exhausted.
    TooManyFields,
    /// A field prefix is incomplete or its declared length is short or unaligned.
    InvalidFieldLength,
    /// A complete prefix declares more bytes than remain in the input.
    TruncatedField,
}

/// Explicit ceilings for an inert plaintext sequence, not production defaults.
///
/// Both ceilings may be zero. At most 4096 bytes and 1024 fields are accepted;
/// each complete field has at least four bytes. The caller must independently
/// bound the actual authenticated plaintext and its later semantic consumer.
///
/// ```compile_fail,E0451
/// use ntp_wire::NtsPlaintextLimits;
/// let _ = NtsPlaintextLimits { max_bytes: 1, max_fields: 1 };
/// ```
#[derive(Clone, Copy)]
pub struct NtsPlaintextLimits {
    /// Caller-selected byte ceiling, checked to be at most 4096.
    max_bytes: usize,
    /// Caller-selected field ceiling, checked to be at most 1024.
    max_fields: usize,
}

impl NtsPlaintextLimits {
    /// Check explicit independent byte and field budgets.
    ///
    /// # Errors
    /// Returns [`NtsPlaintextError::InvalidLimits`] when either experimental
    /// ceiling exceeds its implementation maximum. No input is read.
    pub fn new(max_bytes: usize, max_fields: usize) -> Result<Self, NtsPlaintextError> {
        if max_bytes > 4096 || max_fields > 1024 {
            return Err(NtsPlaintextError::InvalidLimits);
        }
        Ok(Self {
            max_bytes,
            max_fields,
        })
    }
}

/// A fully framed immutable sequence borrowing exactly the supplied input.
///
/// No authentication is implied. The private fields and immutable borrow keep
/// the proven framing intact for every fresh traversal.
///
/// ```compile_fail,E0451
/// use ntp_wire::RawNtsPlaintext;
/// let _ = RawNtsPlaintext { bytes: &[], count: 0 };
/// ```
///
/// ```compile_fail,E0515
/// use ntp_wire::{frame_nts_plaintext, NtsPlaintextError, NtsPlaintextLimits, RawNtsPlaintext};
/// fn escape() -> Result<RawNtsPlaintext<'static>, NtsPlaintextError> {
///     let bytes = [0x12, 0x34, 0, 4];
///     frame_nts_plaintext(&bytes, NtsPlaintextLimits::new(4, 1)?)
/// }
/// ```
///
/// ```compile_fail,E0506
/// use ntp_wire::{frame_nts_plaintext, NtsPlaintextError, NtsPlaintextLimits};
/// # fn main() -> Result<(), NtsPlaintextError> {
/// let mut bytes = [0x12, 0x34, 0, 4];
/// let frame = frame_nts_plaintext(&bytes, NtsPlaintextLimits::new(4, 1)?)?;
/// bytes[0] = 0;
/// assert_eq!(frame.fields().next().unwrap().field_type(), 0x1234);
/// # Ok(())
/// # }
/// ```
pub struct RawNtsPlaintext<'a> {
    /// Original immutable input, validated as a complete field sequence.
    bytes: &'a [u8],
    /// Number of fields in that fully validated sequence.
    count: usize,
}

impl<'a> RawNtsPlaintext<'a> {
    /// Return the number of fields in the completely admitted sequence.
    #[must_use]
    pub const fn field_count(&self) -> usize {
        self.count
    }

    /// Start an independent traversal of the original checked input.
    #[must_use]
    pub const fn fields(&self) -> NtsPlaintextFields<'a> {
        NtsPlaintextFields {
            remaining: self.bytes,
        }
    }
}

/// One fully checked field; its body includes opaque field-specific padding.
///
/// ```compile_fail,E0451
/// use ntp_wire::RawNtsPlaintextField;
/// let _ = RawNtsPlaintextField { bytes: &[0; 4] };
/// ```
pub struct RawNtsPlaintextField<'a> {
    /// Complete admitted field, at least four bytes with length divisible by four.
    bytes: &'a [u8],
}

impl<'a> RawNtsPlaintextField<'a> {
    /// Return the complete network-order field type without interpretation.
    #[must_use]
    pub const fn field_type(&self) -> u16 {
        u16::from_be_bytes([self.bytes[0], self.bytes[1]])
    }

    /// Borrow the exact original field including its prefix and padding.
    #[must_use]
    pub const fn bytes(&self) -> &'a [u8] {
        self.bytes
    }

    /// Borrow the exact opaque body including any type-specific padding.
    #[must_use]
    pub fn body(&self) -> &'a [u8] {
        &self.bytes[4..]
    }
}

/// Infallible traversal of a previously checked immutable plaintext sequence.
///
/// ```compile_fail,E0451
/// use ntp_wire::NtsPlaintextFields;
/// let _ = NtsPlaintextFields { remaining: &[0; 4] };
/// ```
pub struct NtsPlaintextFields<'a> {
    /// Empty or a completely framed suffix beginning at a proven field boundary.
    remaining: &'a [u8],
}

impl<'a> Iterator for NtsPlaintextFields<'a> {
    type Item = RawNtsPlaintextField<'a>;

    /// Consume a proven complete field, or remain exhausted at the end.
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining.is_empty() {
            return None;
        }
        let length = usize::from(u16::from_be_bytes([self.remaining[2], self.remaining[3]]));
        let (bytes, remaining) = self.remaining.split_at(length);
        self.remaining = remaining;
        Some(RawNtsPlaintextField { bytes })
    }
}

/// Frame all fields in the supplied candidate NTS plaintext.
///
/// The caller supplies an immutable byte slice and checked byte/count ceilings.
/// Empty input is valid. Each field has a four-byte type/length prefix, a total
/// length of at least four bytes and a length divisible by four; the length
/// includes the prefix and opaque padding. No outer-datagram or final-field
/// minimum applies. At most 1024 bounded iterations run and output is constant
/// size. No heap, I/O, mutation, unsafe code or protected effect occurs.
///
/// This function does not decrypt or prove authentication. Parsing success does
/// not authorize interpretation or publication; the actual AEAD and semantic
/// owners must enforce their own success, privacy and lifecycle contracts.
///
/// # Errors
/// Rejects a byte-budget excess, an exhausted field budget, an incomplete
/// prefix, a short or unaligned length, or a declared span beyond the input.
/// A failure exposes no partial sequence or plaintext in the error.
pub fn frame_nts_plaintext(
    input: &[u8],
    limits: NtsPlaintextLimits,
) -> Result<RawNtsPlaintext<'_>, NtsPlaintextError> {
    if input.len() > limits.max_bytes {
        return Err(NtsPlaintextError::ByteLimit);
    }
    let mut remaining = input;
    let mut count = 0;
    while !remaining.is_empty() {
        if count >= limits.max_fields {
            return Err(NtsPlaintextError::TooManyFields);
        }
        let prefix = remaining
            .first_chunk::<4>()
            .ok_or(NtsPlaintextError::InvalidFieldLength)?;
        let length = usize::from(u16::from_be_bytes([prefix[2], prefix[3]]));
        if length < 4 || !length.is_multiple_of(4) {
            return Err(NtsPlaintextError::InvalidFieldLength);
        }
        let (_, rest) = remaining
            .split_at_checked(length)
            .ok_or(NtsPlaintextError::TruncatedField)?;
        remaining = rest;
        count += 1;
    }
    Ok(RawNtsPlaintext {
        bytes: input,
        count,
    })
}

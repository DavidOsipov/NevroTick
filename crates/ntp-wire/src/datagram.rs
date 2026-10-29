// SPDX-License-Identifier: GPL-2.0-only
// Protocol-framing context: Chrony 4.9 ntp_ext.c at
// 8df4f1263e206585e6c4e61352d042c2dd12916a; retained notice: ../PROVENANCE.md.

//! Bounded, inert outer-datagram views for an explicit no-legacy-MAC grammar.
//!
//! The framer owns complete length/count admission before constructing borrowed
//! immutable views. A successful tail contains complete word-aligned fields of
//! at least 16 bytes, with a final field of at least 28 bytes. Private immutable
//! construction carries this invariant to the iterator and field projections;
//! unchanged bytes need no second parser. Bodies include opaque padding.
//! No field type, NTS authentication, exchange, era or sample policy is admitted.
//! Parsing allocates nothing and introduces no I/O, state or clock effects.

use crate::{FIXED_HEADER_LEN, RawHeader, decode_fixed_header};

/// Fixed redacted admission failures; no variant retains packet or cookie bytes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameError {
    /// The requested experimental byte/count ceiling is outside its allowed range.
    InvalidLimits,
    /// Input exceeds the explicitly supplied byte ceiling.
    ByteLimit,
    /// Fewer than 48 bytes are available for the fixed header.
    HeaderTooShort,
    /// Another field is present after the supplied count budget is exhausted.
    TooManyExtensions,
    /// A prefix or declared length violates the complete outer-field grammar.
    InvalidExtensionLength,
    /// A complete prefix declares more bytes than remain in the datagram.
    TruncatedExtension,
    /// The complete final field without a legacy MAC is shorter than 28 bytes.
    FinalExtensionTooShort,
}

/// Explicit experimental ceilings; these are neither UDP limits nor defaults.
///
/// Only [`Self::new`] constructs this immutable value. Byte ceilings are in
/// `48..=4096`, including unaligned ceilings; count ceilings are in `0..=64`.
/// No default or mutable/unchecked construction route is provided.
///
/// ```
/// use ntp_wire::{FrameError, FrameLimits};
/// # fn main() -> Result<(), FrameError> {
/// let _limits = FrameLimits::new(4096, 64)?;
/// # Ok(())
/// # }
/// ```
///
/// ```compile_fail,E0451
/// use ntp_wire::FrameLimits;
/// let _limits = FrameLimits { max_datagram_bytes: 4097, max_extension_fields: 65 };
/// ```
///
/// ```compile_fail,E0616
/// use ntp_wire::{FrameError, FrameLimits};
/// # fn main() -> Result<(), FrameError> {
/// let mut limits = FrameLimits::new(48, 0)?;
/// limits.max_datagram_bytes = usize::MAX;
/// # Ok(())
/// # }
/// ```
///
/// ```compile_fail,E0277
/// use ntp_wire::FrameLimits;
/// let _limits: FrameLimits = Default::default();
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FrameLimits {
    /// Checked maximum whole-datagram length, including the fixed header.
    max_datagram_bytes: usize,
    /// Checked maximum number of complete outer extension fields.
    max_extension_fields: usize,
}

impl FrameLimits {
    /// Construct explicit, privately checked experimental parser ceilings.
    ///
    /// Valid byte ceilings are `48..=4096`; field ceilings are `0..=64`. A byte
    /// ceiling need not be aligned. Failure publishes no usable limits.
    ///
    /// # Errors
    /// Returns [`FrameError::InvalidLimits`] for an out-of-range byte or count
    /// ceiling. These are isolated experiment limits, not transport defaults.
    pub fn new(max_datagram_bytes: usize, max_extension_fields: usize) -> Result<Self, FrameError> {
        if !(48..=4096).contains(&max_datagram_bytes) || max_extension_fields > 64 {
            return Err(FrameError::InvalidLimits);
        }
        Ok(Self {
            max_datagram_bytes,
            max_extension_fields,
        })
    }
}

/// A completely framed immutable datagram; success conveys no protocol trust.
///
/// The fixed header is an owned copy, while every field borrows the original
/// input. Private construction after complete admission preserves all tail
/// boundaries and the count. No packet-bearing `Debug` implementation is exposed.
///
/// ```
/// use ntp_wire::{FrameError, FrameLimits, frame_no_legacy_mac_datagram};
/// # fn main() -> Result<(), FrameError> {
/// let bytes = [0; 48];
/// let datagram = frame_no_legacy_mac_datagram(&bytes, FrameLimits::new(48, 0)?)?;
/// assert_eq!(datagram.extension_count(), 0);
/// assert_eq!(datagram.header().version(), 0);
/// assert!(datagram.extensions().next().is_none());
/// # Ok(())
/// # }
/// ```
///
/// ```compile_fail,E0451
/// use ntp_wire::{HeaderError, RawDatagram, decode_fixed_header};
/// # fn main() -> Result<(), HeaderError> {
/// let header = decode_fixed_header(&[0; 48])?;
/// let _datagram = RawDatagram { header, tail: &[], count: 0 };
/// # Ok(())
/// # }
/// ```
///
/// ```compile_fail,E0616
/// use ntp_wire::{FrameError, FrameLimits, frame_no_legacy_mac_datagram};
/// # fn main() -> Result<(), FrameError> {
/// let bytes = [0; 48];
/// let mut datagram = frame_no_legacy_mac_datagram(&bytes, FrameLimits::new(48, 0)?)?;
/// datagram.count = 64;
/// # Ok(())
/// # }
/// ```
///
/// The returned view cannot outlive the input:
///
/// ```compile_fail,E0597
/// use ntp_wire::{FrameError, FrameLimits, frame_no_legacy_mac_datagram};
/// # fn main() -> Result<(), FrameError> {
/// let datagram;
/// {
///     let bytes = [0; 48];
///     datagram = frame_no_legacy_mac_datagram(&bytes, FrameLimits::new(48, 0)?)?;
/// }
/// assert_eq!(datagram.extension_count(), 0);
/// # Ok(())
/// # }
/// ```
///
/// Live borrowed views also prevent safe mutation of the admitted input:
///
/// ```compile_fail,E0506
/// use ntp_wire::{FrameError, FrameLimits, frame_no_legacy_mac_datagram};
/// # fn main() -> Result<(), FrameError> {
/// let mut bytes = [0; 48];
/// let datagram = frame_no_legacy_mac_datagram(&bytes, FrameLimits::new(48, 0)?)?;
/// bytes[0] = 0xff;
/// assert_eq!(datagram.header().version(), 0);
/// # Ok(())
/// # }
/// ```
pub struct RawDatagram<'a> {
    /// Owned exact fixed-header snapshot, independently decoded by its owner.
    header: RawHeader,
    /// Fully admitted immutable complete extension chain from the input.
    tail: &'a [u8],
    /// Exact number of fields in `tail`, within the admitted count ceiling.
    count: usize,
}

impl<'a> RawDatagram<'a> {
    /// Borrow the owned raw header without admitting its field semantics.
    #[must_use]
    pub const fn header(&self) -> &RawHeader {
        &self.header
    }

    /// Return the fully checked chain's field count, at most 64.
    #[must_use]
    pub const fn extension_count(&self) -> usize {
        self.count
    }

    /// Start fresh, allocation-free iteration over the complete immutable tail.
    #[must_use]
    pub const fn extensions(&self) -> ExtensionFields<'a> {
        ExtensionFields {
            remaining: self.tail,
        }
    }
}

/// One complete checked field, borrowing its type/length/body/padding bytes.
///
/// Only iteration of a completely checked datagram constructs this view. Its
/// private slice has at least 16 bytes and a matching aligned declared length.
/// Opaque bytes and their type grant no authenticated or semantic authority.
///
/// ```compile_fail,E0451
/// use ntp_wire::RawExtension;
/// let _field = RawExtension { bytes: &[0; 4] };
/// ```
///
/// ```compile_fail,E0616
/// use ntp_wire::RawExtension;
/// fn replace_bytes(field: &mut RawExtension<'_>) {
///     field.bytes = &[];
/// }
/// ```
pub struct RawExtension<'a> {
    /// Complete admitted field, including its four-byte prefix and padding.
    bytes: &'a [u8],
}

impl<'a> RawExtension<'a> {
    /// Return all 16 network-order type bits without recognizing a field policy.
    #[must_use]
    pub const fn field_type(&self) -> u16 {
        u16::from_be_bytes([self.bytes[0], self.bytes[1]])
    }

    /// Borrow the exact complete field bytes, including prefix and padding.
    #[must_use]
    pub const fn bytes(&self) -> &'a [u8] {
        self.bytes
    }

    /// Borrow everything after the prefix, including opaque field-specific padding.
    #[must_use]
    pub fn body(&self) -> &'a [u8] {
        &self.bytes[4..]
    }
}

/// Fresh bounded iteration over an already checked immutable field chain.
///
/// Private construction admits only complete chains. Each step consumes at least
/// 16 bytes, preserves the checked suffix invariant and performs no allocation.
/// It never reparses changed input or converts a parser failure into end-of-input.
///
/// ```compile_fail,E0451
/// use ntp_wire::ExtensionFields;
/// let _fields = ExtensionFields { remaining: &[0; 4] };
/// ```
pub struct ExtensionFields<'a> {
    /// Empty or a complete admitted suffix of the original checked tail.
    remaining: &'a [u8],
}

impl<'a> Iterator for ExtensionFields<'a> {
    type Item = RawExtension<'a>;

    /// Consume one proven complete field; empty input remains exhausted.
    ///
    /// The private construction invariant guarantees the indexed prefix and
    /// complete declared span before this infallible projection executes.
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining.is_empty() {
            return None;
        }
        let length = usize::from(u16::from_be_bytes([self.remaining[2], self.remaining[3]]));
        let (bytes, remaining) = self.remaining.split_at(length);
        self.remaining = remaining;
        Some(RawExtension { bytes })
    }
}

/// Frame one complete header and outer extension chain with no legacy MAC.
///
/// The immutable returned views borrow `input`; no partial view is published on
/// failure. The selected grammar never guesses a legacy MAC/crypto-NAK trailer.
/// Unknown types and syntactically valid MAC-looking fields remain opaque; this
/// entry point makes no authentication, NTS-ordering, exchange or time claim.
///
/// The byte ceiling is checked before reading the header or tail. Each admitted
/// loop step consumes at least 16 bytes, and the count ceiling is checked before
/// inspecting another prefix. Output metadata is constant-size, with no heap,
/// I/O, retained state or protected effect. Safe borrows preserve checked bytes.
///
/// # Errors
/// Rejects excess bytes, short headers, exhausted field budgets, short/unaligned
/// prefixes or lengths, truncated fields and a final field shorter than 28 bytes.
/// Errors contain no input data and publish no partial frame.
pub fn frame_no_legacy_mac_datagram(
    input: &[u8],
    limits: FrameLimits,
) -> Result<RawDatagram<'_>, FrameError> {
    if input.len() > limits.max_datagram_bytes {
        return Err(FrameError::ByteLimit);
    }
    let (header_bytes, tail) = input
        .split_at_checked(FIXED_HEADER_LEN)
        .ok_or(FrameError::HeaderTooShort)?;
    let header = decode_fixed_header(header_bytes).map_err(|_| FrameError::HeaderTooShort)?;
    let mut remaining = tail;
    let mut count = 0;
    while !remaining.is_empty() {
        if count >= limits.max_extension_fields {
            return Err(FrameError::TooManyExtensions);
        }
        let prefix = remaining
            .first_chunk::<4>()
            .ok_or(FrameError::InvalidExtensionLength)?;
        let length = usize::from(u16::from_be_bytes([prefix[2], prefix[3]]));
        if length < 16 || !length.is_multiple_of(4) {
            return Err(FrameError::InvalidExtensionLength);
        }
        let (_, rest) = remaining
            .split_at_checked(length)
            .ok_or(FrameError::TruncatedExtension)?;
        if rest.is_empty() && length < 28 {
            return Err(FrameError::FinalExtensionTooShort);
        }
        remaining = rest;
        count += 1;
    }
    Ok(RawDatagram {
        header,
        tail,
        count,
    })
}

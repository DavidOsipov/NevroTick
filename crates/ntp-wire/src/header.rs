// SPDX-License-Identifier: GPL-2.0-only
// Protocol-layout context: Chrony 4.9 ntp.h at
// 8df4f1263e206585e6c4e61352d042c2dd12916a; retained notice: ../PROVENANCE.md.

//! Exactly sized immutable wire values and their nonsemantic field projections.
//!
//! Input length is the only admission performed here. Every method is pure and
//! bounded by a fixed array shape. Raw bits convey no current exchange, trust,
//! UTC, sampling or clock-effect authority. There is no implicit normalization.

/// Number of octets in the fixed NTP header, independent of extension support.
pub const FIXED_HEADER_LEN: usize = 48;

/// Fixed redacted framing failure; contains no input bytes or allocated message.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeaderError {
    /// Input is not exactly one fixed header; no partial output is returned.
    WrongLength,
}

/// An immutable exact-size copy of an NTP header, with all raw patterns retained.
///
/// Obtain it through [`decode_fixed_header`]. No version/mode/LI, timestamp-zero,
/// stratum, profile, era or authentication validation is implied. There is no
/// public raw constructor or mutable access to the retained representation.
///
/// Legal construction uses the length-checking decoder, even for all-zero bits:
///
/// ```
/// use ntp_wire::{HeaderError, decode_fixed_header};
/// # fn main() -> Result<(), HeaderError> {
/// let header = decode_fixed_header(&[0; 48])?;
/// assert_eq!(header.version(), 0);
/// assert_eq!(header.origin_timestamp().seconds(), 0);
/// # Ok(())
/// # }
/// ```
///
/// External callers cannot construct the private representation:
///
/// ```compile_fail,E0451
/// use ntp_wire::RawHeader;
/// let _header = RawHeader { bytes: [0; 48] };
/// ```
///
/// Nor can they mutate it after decoding:
///
/// ```compile_fail,E0616
/// use ntp_wire::{HeaderError, decode_fixed_header};
/// # fn main() -> Result<(), HeaderError> {
/// let mut header = decode_fixed_header(&[0; 48])?;
/// header.bytes[0] = 0xff;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RawHeader {
    /// Owned exact-size wire bytes; only the decoder constructs this value.
    bytes: [u8; FIXED_HEADER_LEN],
}

impl RawHeader {
    /// Return the two raw leap-indicator bits without synchronized-state admission.
    #[must_use]
    pub const fn leap_indicator(&self) -> u8 {
        self.bytes[0] >> 6
    }

    /// Return the three raw version bits; no supported-version decision is made.
    #[must_use]
    pub const fn version(&self) -> u8 {
        (self.bytes[0] >> 3) & 7
    }

    /// Return the three raw mode bits; no server/client profile is admitted.
    #[must_use]
    pub const fn mode(&self) -> u8 {
        self.bytes[0] & 7
    }

    /// Return the raw stratum octet; zero and 16 or greater remain representable.
    #[must_use]
    pub const fn stratum(&self) -> u8 {
        self.bytes[1]
    }

    /// Return the signed polling exponent exactly; no scheduling policy is applied.
    #[must_use]
    pub const fn poll(&self) -> i8 {
        i8::from_be_bytes([self.bytes[2]])
    }

    /// Return the signed precision exponent exactly; no uncertainty is computed.
    #[must_use]
    pub const fn precision(&self) -> i8 {
        i8::from_be_bytes([self.bytes[3]])
    }

    /// Return the 32 network-order root-delay bits without numeric interpretation.
    #[must_use]
    pub const fn root_delay_bits(&self) -> u32 {
        u32::from_be_bytes([self.bytes[4], self.bytes[5], self.bytes[6], self.bytes[7]])
    }

    /// Return the 32 network-order root-dispersion bits without quality admission.
    #[must_use]
    pub const fn root_dispersion_bits(&self) -> u32 {
        u32::from_be_bytes([self.bytes[8], self.bytes[9], self.bytes[10], self.bytes[11]])
    }

    /// Return the opaque network-order reference identifier, including `KoD` codes.
    #[must_use]
    pub const fn reference_id(&self) -> u32 {
        u32::from_be_bytes([
            self.bytes[12],
            self.bytes[13],
            self.bytes[14],
            self.bytes[15],
        ])
    }

    /// Return the exact reference timestamp bits; no age or era is inferred.
    #[must_use]
    pub const fn reference_timestamp(&self) -> NtpTimestamp {
        NtpTimestamp {
            bytes: [
                self.bytes[16],
                self.bytes[17],
                self.bytes[18],
                self.bytes[19],
                self.bytes[20],
                self.bytes[21],
                self.bytes[22],
                self.bytes[23],
            ],
        }
    }

    /// Return the exact origin timestamp/token bits; no request correlation is made.
    #[must_use]
    pub const fn origin_timestamp(&self) -> NtpTimestamp {
        NtpTimestamp {
            bytes: [
                self.bytes[24],
                self.bytes[25],
                self.bytes[26],
                self.bytes[27],
                self.bytes[28],
                self.bytes[29],
                self.bytes[30],
                self.bytes[31],
            ],
        }
    }

    /// Return the exact receive timestamp bits; zero remains an inert value.
    #[must_use]
    pub const fn receive_timestamp(&self) -> NtpTimestamp {
        NtpTimestamp {
            bytes: [
                self.bytes[32],
                self.bytes[33],
                self.bytes[34],
                self.bytes[35],
                self.bytes[36],
                self.bytes[37],
                self.bytes[38],
                self.bytes[39],
            ],
        }
    }

    /// Return the exact transmit timestamp bits; no UTC or currentness is implied.
    #[must_use]
    pub const fn transmit_timestamp(&self) -> NtpTimestamp {
        NtpTimestamp {
            bytes: [
                self.bytes[40],
                self.bytes[41],
                self.bytes[42],
                self.bytes[43],
                self.bytes[44],
                self.bytes[45],
                self.bytes[46],
                self.bytes[47],
            ],
        }
    }
}

/// An immutable NTP seconds/fraction bit pattern with no calendar-era authority.
///
/// These are the exact 32.32 wire fields, including zero. There is no conversion
/// to UTC, nanoseconds, floating point or a trusted timestamp in this owner.
/// Only fixed-header projections construct it; fields are not publicly writable.
///
/// ```compile_fail,E0451
/// use ntp_wire::NtpTimestamp;
/// let _timestamp = NtpTimestamp { bytes: [0; 8] };
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NtpTimestamp {
    /// Exact eight wire octets; all representation patterns are permitted.
    bytes: [u8; 8],
}

impl NtpTimestamp {
    /// Return the unsigned seconds field exactly, without choosing a calendar era.
    #[must_use]
    pub const fn seconds(&self) -> u32 {
        u32::from_be_bytes([self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3]])
    }

    /// Return the exact fraction numerator with denominator 2^32 seconds.
    #[must_use]
    pub const fn fraction(&self) -> u32 {
        u32::from_be_bytes([self.bytes[4], self.bytes[5], self.bytes[6], self.bytes[7]])
    }

    /// Return the exact combined 64 wire bits, not an unfolded absolute time.
    #[must_use]
    pub const fn bits(&self) -> u64 {
        u64::from_be_bytes(self.bytes)
    }
}

/// Decode exactly one fixed header into an owned immutable raw representation.
///
/// Work and output are fixed at 48 bytes, without allocation, state mutation or
/// any external effect. The caller owns bounding a larger datagram before it
/// selects this header-only entry point. Longer input is not silently truncated.
/// Every bit pattern of an exactly sized input is representable; subsequent
/// protocol/domain/exchange admission must independently enforce its contracts.
///
/// # Errors
/// Returns [`HeaderError::WrongLength`] when the input is not exactly 48 bytes.
pub fn decode_fixed_header(input: &[u8]) -> Result<RawHeader, HeaderError> {
    let bytes: &[u8; FIXED_HEADER_LEN] = input.try_into().map_err(|_| HeaderError::WrongLength)?;
    Ok(RawHeader { bytes: *bytes })
}

//! Pure bounded NTP wire representation decoding and outer framing.
//!
//! S1.1a handles exactly one 48-byte fixed header. Its immutable outputs retain
//! every raw field without admitting a time sample, peer, profile, authentication
//! state or calendar era. S1.1b adds explicit no-legacy-MAC outer framing and
//! immutable opaque extension views. Exchange, NTS semantics, authentication and
//! sample admission remain separately bounded owners.
//!
//! The library has no heap, I/O, ambient clock, mutable retained state or unsafe
//! code. Source provenance and the retained upstream notice are in
//! `crates/ntp-wire/PROVENANCE.md`.
#![no_std]
#![forbid(unsafe_code)]

mod datagram;
mod header;

pub use datagram::{
    ExtensionFields, FrameError, FrameLimits, RawDatagram, RawExtension,
    frame_no_legacy_mac_datagram,
};
pub use header::{FIXED_HEADER_LEN, HeaderError, NtpTimestamp, RawHeader, decode_fixed_header};

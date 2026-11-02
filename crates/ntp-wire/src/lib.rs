//! Pure bounded NTP wire decoding and inert NTS-KE request/record processing.
//!
//! S1.1a handles exactly one 48-byte fixed header. Its immutable outputs retain
//! every raw field without admitting a time sample, peer, profile, authentication
//! state or calendar era. S1.1b adds explicit no-legacy-MAC outer framing and
//! immutable opaque extension views. Exchange, NTS semantics, authentication and
//! sample admission remain separately bounded owners.
//!
//! The library has no heap, I/O, ambient clock or unsafe code. Incremental framing
//! retains bounded state only within its caller-storage borrow. Exporter contexts
//! own inert metadata and confer no TLS or association authority. Provenance is in
//! `crates/ntp-wire/PROVENANCE.md`.
#![no_std]
#![forbid(unsafe_code)]

mod datagram;
mod header;
mod nts_ke;
mod nts_negotiation;
mod nts_packet;
mod nts_plaintext;

pub use datagram::{
    ExtensionFields, FrameError, FrameLimits, RawDatagram, RawExtension,
    frame_no_legacy_mac_datagram,
};
pub use header::{FIXED_HEADER_LEN, HeaderError, NtpTimestamp, RawHeader, decode_fixed_header};

pub use nts_ke::{
    NtsKeAccumulator, NtsKeAccumulatorError, NtsKeFrameError, NtsKeFrameLimits, NtsKeProgress,
    NtsKeRecords, RawNtsKeMessage, RawNtsKeRecord, frame_nts_ke_message,
};

pub use nts_negotiation::{
    NtsKeCookies, NtsKeExporterContext, NtsKeExporterDirection, NtsKeNegotiationError,
    NtsKeNtpv4Negotiation, NtsKeNtpv4Offer, NtsKeRequestEncodeError, encode_nts_ke_ntpv4_request,
    negotiate_nts_ke_ntpv4, read_nts_ke_ntpv4_offer,
};

pub use nts_packet::{NtsAuthenticatorError, NtsAuthenticatorParts, read_nts_authenticator_parts};

pub use nts_plaintext::{
    NtsPlaintextError, NtsPlaintextFields, NtsPlaintextLimits, RawNtsPlaintext,
    RawNtsPlaintextField, frame_nts_plaintext,
};

// SPDX-License-Identifier: GPL-2.0-only

//! Pure NTPv4-client negotiation over complete, already framed NTS-KE messages.
//!
//! The checked offer is derived from the actual request buffer. This current
//! client profile offers only protocol zero; repeated zero identifiers remain
//! meaningful offered identifiers. Responses may repeat that offered protocol
//! or refuse it with an empty list. Algorithm identifiers are compared with the
//! offer, not treated as proof of an admitted cryptographic implementation.
//!
//! A successful result is pending negotiation data, never an authenticated or
//! usable association. TLS identity/closure, exporter keys, provider support,
//! endpoint syntax and destination policy, current generations and publication
//! remain separate owners. Endpoint strings retain only the pending ASCII shape
//! documented below; an omitted server means the saved actual KE peer IP, never
//! a fresh resolution of its hostname. An omitted port is interpreted as 123 by
//! that later owner. Duplicate port records are rejected as a local ambiguity
//! rule; RFC 8915 does not specify their cardinality.
//!
//! Every scan is linear in the two bounded immutable buffers. The framing owner
//! bounds each buffer to 131,072 bytes and 32,768 records. No heap, retained
//! mutable state, I/O, clock, authentication, diagnostics or external effect is
//! present, and rejection exposes no partial negotiation or payload bytes.

use crate::{NtsKeRecords, RawNtsKeMessage, RawNtsKeRecord};

/// Closed negotiation rejection with at most one bounded peer error code.
///
/// No input, cookie, endpoint string or cryptographic material is retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NtsKeNegotiationError {
    /// An unsupported record has its critical bit set.
    UnsupportedCriticalRecord,
    /// A non-error message lacks protocol negotiation.
    MissingProtocolNegotiation,
    /// A message includes more than one protocol-negotiation record.
    DuplicateProtocolNegotiation,
    /// Protocol negotiation has invalid criticality, parity or request length.
    InvalidProtocolNegotiation,
    /// A protocol identifier is outside this NTPv4-only client profile.
    UnsupportedProtocol,
    /// An `NTPv4` message lacks its required algorithm-negotiation record.
    MissingAeadNegotiation,
    /// A message includes more than one algorithm-negotiation record.
    DuplicateAeadNegotiation,
    /// Algorithm negotiation has invalid parity or direction-specific length.
    InvalidAeadNegotiation,
    /// The request includes an Error, Warning or New Cookie record.
    ForbiddenRequestRecord,
    /// An Error record lacks criticality or its exact two-byte code.
    InvalidErrorRecord,
    /// A Warning record lacks criticality or its exact two-byte code.
    InvalidWarningRecord,
    /// The first well-shaped Error record aborts this response.
    PeerError {
        /// The first peer error code in wire order; no retry policy is implied.
        code: u16,
    },
    /// This profile recognizes no Warning codes, so the first one aborts.
    UnsupportedWarning {
        /// The first warning code in wire order.
        code: u16,
    },
    /// The server's empty protocol list is a negotiation refusal.
    NoCommonProtocol,
    /// A legal empty algorithm list refuses the offered nonmandatory choices.
    NoCommonAead,
    /// `NTPv4` was selected but the mandatory offered algorithm 15 was refused.
    MandatoryAeadMissing,
    /// The selected algorithm did not occur in this actual request.
    UnofferedAead,
    /// More than one server-name record appears in a message.
    DuplicateServer,
    /// The pending server hint fails its explicitly limited ASCII shape.
    InvalidServerHint,
    /// More than one port record violates this candidate's ambiguity policy.
    DuplicatePort,
    /// A port record is not exactly two bytes.
    InvalidPort,
    /// Successful `NTPv4` and algorithm negotiation supplies no cookie record.
    MissingCookie,
}

/// Immutable proof of the actual request's supported `NTPv4` negotiation shape.
///
/// Only [`read_nts_ke_ntpv4_offer`] constructs this value. It preserves the
/// original nonempty even algorithm list, including caller preference order;
/// provider support and transmission of these request bytes remain unproved.
/// The original buffer must remain immutable and alive while any view is used.
///
/// ```compile_fail,E0451
/// use ntp_wire::NtsKeNtpv4Offer;
/// let _ = NtsKeNtpv4Offer { request: &[], aeads: &[] };
/// ```
/// ```compile_fail,E0277
/// use ntp_wire::NtsKeNtpv4Offer;
/// let _: NtsKeNtpv4Offer<'static> = Default::default();
/// ```
/// ```compile_fail,E0277
/// use ntp_wire::NtsKeNtpv4Offer;
/// fn needs_debug<T: core::fmt::Debug>() {}
/// needs_debug::<NtsKeNtpv4Offer<'static>>();
/// ```
pub struct NtsKeNtpv4Offer<'a> {
    /// Exact original request, including its final EOM.
    request: &'a [u8],
    /// Nonempty even bytes of the sole algorithm record in preference order.
    aeads: &'a [u8],
}

impl<'a> NtsKeNtpv4Offer<'a> {
    /// Borrow the exact checked request bytes; this proves no actual send.
    #[must_use]
    pub const fn request_bytes(&self) -> &'a [u8] {
        self.request
    }

    /// Borrow complete big-endian two-byte algorithm identifiers in wire order.
    ///
    /// The slice is nonempty and even; identifiers do not prove provider support.
    #[must_use]
    pub const fn offered_aead_bytes(&self) -> &'a [u8] {
        self.aeads
    }

    /// Compare one response identifier with the immutable, even offered list.
    ///
    /// This single bounded scan has no state or effect and cannot match a partial
    /// identifier. The constructor has proved there is no trailing odd byte.
    fn offers_aead(&self, candidate: u16) -> bool {
        self.aeads
            .as_chunks::<2>()
            .0
            .iter()
            .any(|word| u16::from_be_bytes(*word) == candidate)
    }
}

/// A complete pending `NTPv4` negotiation, without authentication or authority.
///
/// The sole constructor checks the whole response against a checked actual
/// request. Cookie bodies remain opaque, even when empty. The response view is
/// owned here while all its bytes continue to borrow the original input, so
/// fresh cookie traversals reuse the framing owner's admitted iterator.
///
/// A server hint is only a nonempty printable-ASCII candidate without whitespace
/// or `/`, `\`, `@`, `%`, `[` or `]`. Colons remain possible for IPv6. This is
/// **not** DNS/IDNA/IP validation, absolute-name resolution or destination-policy
/// approval. A later endpoint owner must validate the complete syntax, treat a
/// domain name as absolute even without a trailing dot, and check actual egress.
/// No result may be published as a usable association on this proof alone.
///
/// ```compile_fail,E0616
/// use ntp_wire::NtsKeNtpv4Negotiation;
/// fn change(mut value: NtsKeNtpv4Negotiation<'_>) { value.aead_id = 0; }
/// ```
/// ```compile_fail,E0277
/// use ntp_wire::NtsKeNtpv4Negotiation;
/// let _: NtsKeNtpv4Negotiation<'static> = Default::default();
/// ```
/// ```compile_fail,E0277
/// use ntp_wire::NtsKeNtpv4Negotiation;
/// fn needs_debug<T: core::fmt::Debug>() {}
/// needs_debug::<NtsKeNtpv4Negotiation<'static>>();
/// ```
pub struct NtsKeNtpv4Negotiation<'a> {
    /// The whole immutable admitted response, retaining all original spans.
    message: RawNtsKeMessage<'a>,
    /// The one selected identifier, proven to occur in the actual offer.
    aead_id: u16,
    /// Exact number of opaque cookie records; at least one is present.
    cookie_count: usize,
    /// Pending printable ASCII only; not an admitted endpoint or DNS name.
    server_hint: Option<&'a str>,
    /// Pending exact wire port; absence means the later RFC default of 123.
    port_hint: Option<u16>,
}

impl<'a> NtsKeNtpv4Negotiation<'a> {
    /// Borrow the complete original response; no normalization or copy occurs.
    #[must_use]
    pub const fn bytes(&self) -> &'a [u8] {
        self.message.bytes()
    }

    /// Return the selected offered identifier, without provider qualification.
    #[must_use]
    pub const fn aead_id(&self) -> u16 {
        self.aead_id
    }

    /// Return the exact positive cookie-record count, including empty bodies.
    #[must_use]
    pub const fn cookie_count(&self) -> usize {
        self.cookie_count
    }

    /// Begin a fresh bounded traversal of original opaque cookie bodies.
    #[must_use]
    pub const fn cookies(&self) -> NtsKeCookies<'a> {
        NtsKeCookies {
            records: self.message.records(),
        }
    }

    /// Return a pending ASCII candidate; absence means the saved KE peer IP.
    ///
    /// This grants no DNS, IP-syntax, identity, scope or network authority.
    #[must_use]
    pub const fn server_hint(&self) -> Option<&'a str> {
        self.server_hint
    }

    /// Return the exact pending port, preserving zero and absence distinctly.
    ///
    /// The later endpoint owner applies default 123 only to absence, validates
    /// allowed destinations/ports and does not turn this value into authority.
    #[must_use]
    pub const fn port_hint(&self) -> Option<u16> {
        self.port_hint
    }
}

/// Immutable cookie-body iterator with private construction and stable exhaustion.
///
/// Every yielded slice is an original body span, possibly empty or unaligned.
/// No semantic interpretation, allocation, payload `Debug` or mutable factory is
/// provided. The iterator may outlive the negotiation while its input lives.
///
/// ```compile_fail,E0277
/// use ntp_wire::NtsKeCookies;
/// let _: NtsKeCookies<'static> = Default::default();
/// ```
/// ```compile_fail,E0277
/// use ntp_wire::NtsKeCookies;
/// fn needs_debug<T: core::fmt::Debug>() {}
/// needs_debug::<NtsKeCookies<'static>>();
/// ```
pub struct NtsKeCookies<'a> {
    /// Fresh bounded iterator over an entirely checked immutable response.
    records: NtsKeRecords<'a>,
}

impl<'a> Iterator for NtsKeCookies<'a> {
    type Item = &'a [u8];

    /// Skip non-cookie records and yield each exact body in wire order.
    ///
    /// At most the remaining admitted record count is examined. Exhaustion of
    /// the underlying immutable whole-message iterator is permanent.
    fn next(&mut self) -> Option<Self::Item> {
        for record in self.records.by_ref() {
            if record.record_type() == 5 {
                return Some(record.body());
            }
        }
        None
    }
}

/// The framing iterator never resumes after exhaustion, and filtering preserves it.
impl core::iter::FusedIterator for NtsKeCookies<'_> {}

/// Direction-specific forbidden records are checked during the single scan.
#[derive(Clone, Copy)]
enum Direction {
    /// A locally prepared request, before it can establish an offer proof.
    Request,
    /// A peer response, before it can establish any pending negotiation.
    Response,
}

/// Private staged metadata; incomplete fields are never exposed as negotiation.
struct Fields<'a> {
    /// Sole even protocol-list body, if present.
    protocols: Option<&'a [u8]>,
    /// Sole even algorithm-list body, if present.
    aeads: Option<&'a [u8]>,
    /// Exact cookie count, bounded by the framing owner's record count.
    cookie_count: usize,
    /// Sole pending limited-ASCII server hint, if present.
    server_hint: Option<&'a str>,
    /// Sole two-byte pending port, if present.
    port_hint: Option<u16>,
    /// First well-shaped peer Error code; every later record is still scanned.
    peer_error: Option<u16>,
    /// First well-shaped Warning code; this profile recognizes none.
    warning: Option<u16>,
}

/// Store a sole even protocol list after checking its required critical bit.
///
/// `slot` is private staging. Failure leaves it unchanged; body emptiness and
/// supported identifiers are checked by the direction-specific consumer.
///
/// # Errors
/// Reject a duplicate list, a missing critical bit or an odd body length.
fn protocol_record<'a>(
    slot: &mut Option<&'a [u8]>,
    record: &RawNtsKeRecord<'a>,
) -> Result<(), NtsKeNegotiationError> {
    if slot.is_some() {
        return Err(NtsKeNegotiationError::DuplicateProtocolNegotiation);
    }
    if !record.critical() || !record.body().len().is_multiple_of(2) {
        return Err(NtsKeNegotiationError::InvalidProtocolNegotiation);
    }
    *slot = Some(record.body());
    Ok(())
}

/// Store a sole even algorithm list; either critical-bit value is legal.
///
/// Responses permit at most one identifier, including on an abort or refusal.
/// Failure preserves the private staging slot. Request nonemptiness is checked
/// by its offer consumer before any public result.
///
/// # Errors
/// Reject duplicate or odd lists, and response lists longer than one identifier.
fn aead_record<'a>(
    slot: &mut Option<&'a [u8]>,
    record: &RawNtsKeRecord<'a>,
    direction: Direction,
) -> Result<(), NtsKeNegotiationError> {
    if slot.is_some() {
        return Err(NtsKeNegotiationError::DuplicateAeadNegotiation);
    }
    if !record.body().len().is_multiple_of(2)
        || (matches!(direction, Direction::Response) && record.body().len() > 2)
    {
        return Err(NtsKeNegotiationError::InvalidAeadNegotiation);
    }
    *slot = Some(record.body());
    Ok(())
}

/// Read one exact big-endian code, enforcing required criticality if selected.
///
/// The caller supplies only a closed shape error. Rejection exposes no bytes and
/// performs no mutation, allocation, formatting or external effect.
///
/// # Errors
/// Return the supplied shape error for invalid criticality or a non-two-byte body.
fn record_code(
    record: &RawNtsKeRecord<'_>,
    require_critical: bool,
    error: NtsKeNegotiationError,
) -> Result<u16, NtsKeNegotiationError> {
    if require_critical && !record.critical() {
        return Err(error);
    }
    let [high, low] = record.body() else {
        return Err(error);
    };
    Ok(u16::from_be_bytes([*high, *low]))
}

/// Borrow only the candidate's nonempty printable ASCII server-hint shape.
///
/// Spaces, control/non-ASCII bytes and URI, userinfo, bracket or zone separators
/// are rejected. Colons and dots remain uninterpreted for the later endpoint
/// parser; neither valid hostname/IP syntax nor destination authority is proved.
/// UTF-8 decoding is checked, borrows input unchanged and never normalizes it.
///
/// # Errors
/// Reject empty, nonprintable, non-ASCII or excluded-separator input.
fn server_hint(body: &[u8]) -> Result<&str, NtsKeNegotiationError> {
    if body.is_empty()
        || body.iter().any(|byte| {
            !byte.is_ascii_graphic() || matches!(*byte, b'/' | b'\\' | b'@' | b'%' | b'[' | b']')
        })
    {
        return Err(NtsKeNegotiationError::InvalidServerHint);
    }
    core::str::from_utf8(body).map_err(|_| NtsKeNegotiationError::InvalidServerHint)
}

/// Scan every framed record into constant-size private staging before publishing.
///
/// This validates known record shape, cardinality and request direction, and
/// rejects unknown critical records while ignoring noncritical ones. Error and
/// Warning codes are staged without short-circuiting the remaining scan. A
/// malformed later record still rejects the entire message. Cookie counting is
/// bounded by the existing maximum of 32,768 admitted records, so cannot overflow.
///
/// # Errors
/// Reject unsupported critical records, forbidden request records, invalid known
/// record shapes and duplicate singleton records without exposing staged fields.
fn scan_message<'a>(
    message: &RawNtsKeMessage<'a>,
    direction: Direction,
) -> Result<Fields<'a>, NtsKeNegotiationError> {
    let mut fields = Fields {
        protocols: None,
        aeads: None,
        cookie_count: 0,
        server_hint: None,
        port_hint: None,
        peer_error: None,
        warning: None,
    };
    for record in message.records() {
        match record.record_type() {
            0 => {}
            1 => protocol_record(&mut fields.protocols, &record)?,
            4 => aead_record(&mut fields.aeads, &record, direction)?,
            2 | 3 | 5 if matches!(direction, Direction::Request) => {
                return Err(NtsKeNegotiationError::ForbiddenRequestRecord);
            }
            2 => {
                let code = record_code(&record, true, NtsKeNegotiationError::InvalidErrorRecord)?;
                if fields.peer_error.is_none() {
                    fields.peer_error = Some(code);
                }
            }
            3 => {
                let code = record_code(&record, true, NtsKeNegotiationError::InvalidWarningRecord)?;
                if fields.warning.is_none() {
                    fields.warning = Some(code);
                }
            }
            5 => fields.cookie_count += 1,
            6 => {
                if fields.server_hint.is_some() {
                    return Err(NtsKeNegotiationError::DuplicateServer);
                }
                fields.server_hint = Some(server_hint(record.body())?);
            }
            7 => {
                if fields.port_hint.is_some() {
                    return Err(NtsKeNegotiationError::DuplicatePort);
                }
                fields.port_hint = Some(record_code(
                    &record,
                    false,
                    NtsKeNegotiationError::InvalidPort,
                )?);
            }
            _ if record.critical() => {
                return Err(NtsKeNegotiationError::UnsupportedCriticalRecord);
            }
            _ => {}
        }
    }
    Ok(fields)
}

/// Check an actual framed request and borrow its immutable NTPv4-only offer.
///
/// The request needs exactly one critical nonempty even protocol list containing
/// only identifier zero and exactly one nonempty even algorithm list. Repeated
/// zero protocol identifiers and repeated offered algorithms remain legal. Known
/// direction/shape rules and unknown critical records are checked over the whole
/// message before construction. This private proof carries no provider, send,
/// TLS, identity, endpoint or association guarantee and has no external effect.
///
/// # Errors
/// Distinguish unsupported profile identifiers, invalid shapes/cardinality,
/// forbidden response-only records and missing mandatory records. Failure
/// returns no usable offer and changes no caller or retained state.
pub fn read_nts_ke_ntpv4_offer<'a>(
    message: &RawNtsKeMessage<'a>,
) -> Result<NtsKeNtpv4Offer<'a>, NtsKeNegotiationError> {
    let fields = scan_message(message, Direction::Request)?;
    let protocols = fields
        .protocols
        .ok_or(NtsKeNegotiationError::MissingProtocolNegotiation)?;
    if protocols.is_empty() {
        return Err(NtsKeNegotiationError::InvalidProtocolNegotiation);
    }
    if protocols.iter().any(|byte| *byte != 0) {
        return Err(NtsKeNegotiationError::UnsupportedProtocol);
    }
    let aeads = fields
        .aeads
        .ok_or(NtsKeNegotiationError::MissingAeadNegotiation)?;
    if aeads.is_empty() {
        return Err(NtsKeNegotiationError::InvalidAeadNegotiation);
    }
    Ok(NtsKeNtpv4Offer {
        request: message.bytes(),
        aeads,
    })
}

/// Check a complete response against the actual checked request before return.
///
/// The inert response view is consumed; its original bytes remain borrowed and
/// can support independent fresh cookie traversals. After every record shape is
/// checked, the first Error code takes precedence over Warning and negotiation;
/// Error responses may omit protocol negotiation. Any Warning aborts. Ordinary
/// responses require protocol zero and one offered algorithm, plus at least one
/// opaque cookie. Empty protocol/algorithm lists return explicit refusals. If
/// algorithm 15 was offered, selecting `NTPv4` with an empty algorithm list is a
/// protocol violation. Repeating protocol zero is legal, and either critical bit
/// is accepted on algorithm, cookie, server and port records.
///
/// The checked choice is not proof that a provider implements it. A return value
/// contains pending endpoint hints only and grants no authentication, exporter,
/// clock/network effect, usable association or publication authority. No current
/// association is passed in or mutated, including on rejection.
///
/// # Errors
/// Reject malformed/duplicate/unsupported records before interpreting staged
/// aborts; then peer errors/warnings, protocol refusal, algorithm refusal or
/// mismatch and absent cookies. Returned errors contain no payload bytes.
///
/// ```
/// use ntp_wire::{NtsKeFrameLimits, frame_nts_ke_message,
///     read_nts_ke_ntpv4_offer, negotiate_nts_ke_ntpv4};
/// # fn main() -> Result<(), ()> {
/// let request = [0x80, 1, 0, 2, 0, 0, 0x80, 4, 0, 2, 0, 15, 0x80, 0, 0, 0];
/// let response = [0x80, 1, 0, 2, 0, 0, 0, 4, 0, 2, 0, 15,
///                 0, 5, 0, 1, 0xaa, 0x80, 0, 0, 0];
/// let limits = NtsKeFrameLimits::new(64, 8).map_err(|_| ())?;
/// let raw_request = frame_nts_ke_message(&request, limits).map_err(|_| ())?;
/// let offer = read_nts_ke_ntpv4_offer(&raw_request).map_err(|_| ())?;
/// let raw_response = frame_nts_ke_message(&response, limits).map_err(|_| ())?;
/// let pending = negotiate_nts_ke_ntpv4(&offer, raw_response).map_err(|_| ())?;
/// assert_eq!(pending.aead_id(), 15);
/// assert_eq!(pending.cookies().next(), Some(&[0xaa][..]));
/// assert_eq!(pending.server_hint(), None);
/// assert_eq!(pending.port_hint(), None);
/// # Ok(())
/// # }
/// ```
pub fn negotiate_nts_ke_ntpv4<'a>(
    offer: &NtsKeNtpv4Offer<'_>,
    message: RawNtsKeMessage<'a>,
) -> Result<NtsKeNtpv4Negotiation<'a>, NtsKeNegotiationError> {
    let fields = scan_message(&message, Direction::Response)?;
    if let Some(code) = fields.peer_error {
        return Err(NtsKeNegotiationError::PeerError { code });
    }
    if let Some(code) = fields.warning {
        return Err(NtsKeNegotiationError::UnsupportedWarning { code });
    }
    let protocols = fields
        .protocols
        .ok_or(NtsKeNegotiationError::MissingProtocolNegotiation)?;
    if protocols.is_empty() {
        return Err(NtsKeNegotiationError::NoCommonProtocol);
    }
    if protocols.iter().any(|byte| *byte != 0) {
        return Err(NtsKeNegotiationError::UnsupportedProtocol);
    }
    let aeads = fields
        .aeads
        .ok_or(NtsKeNegotiationError::MissingAeadNegotiation)?;
    if aeads.is_empty() {
        return Err(if offer.offers_aead(15) {
            NtsKeNegotiationError::MandatoryAeadMissing
        } else {
            NtsKeNegotiationError::NoCommonAead
        });
    }
    let [high, low] = aeads else {
        return Err(NtsKeNegotiationError::InvalidAeadNegotiation);
    };
    let aead_id = u16::from_be_bytes([*high, *low]);
    if !offer.offers_aead(aead_id) {
        return Err(NtsKeNegotiationError::UnofferedAead);
    }
    if fields.cookie_count == 0 {
        return Err(NtsKeNegotiationError::MissingCookie);
    }
    Ok(NtsKeNtpv4Negotiation {
        message,
        aead_id,
        cookie_count: fields.cookie_count,
        server_hint: fields.server_hint,
        port_hint: fields.port_hint,
    })
}

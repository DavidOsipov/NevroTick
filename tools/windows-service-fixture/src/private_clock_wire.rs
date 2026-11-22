// SPDX-License-Identifier: GPL-2.0-only
//! Bounded complete-frame codec for the private laboratory availability read.
//!
//! The fixed 16-byte header identifies version 1.0, direction, body length
//! and a nonzero correlation ID. Its three reserved bytes are zero. Body
//! admission uses a closed JSON object within a 64-byte limit. Connection
//! identity, request authority and work budgets belong to the caller.

use std::{fmt, mem, num::NonZeroUsize};

use garde::Validate;
use serde::de::{Deserializer as _, Error as _, IgnoredAny, MapAccess, Visitor};

/// Exact version 1.0 envelope size in bytes.
const HEADER_LEN: usize = 16;
/// Fixture body ceiling before JSON decoding.
const MAX_BODY_LEN: usize = 64;
/// Maximum complete frame within the fixed transport storage.
pub(super) const MAX_FRAME_LEN: usize = HEADER_LEN + MAX_BODY_LEN;
/// Exact supported major and minor version.
const VERSION: [u8; 2] = [1, 0];
/// Direction tag for an Engine request.
const REQUEST: u8 = 1;
/// Direction tag for its broker reply.
const REPLY: u8 = 2;

/// Redacted refusal categories. The original payload and parser error stay private.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum FrameError {
    /// The header is missing bytes or has a nonzero reserved field.
    Header,
    /// The version is outside the exact supported version.
    Version,
    /// The frame direction differs from the requested direction.
    Kind,
    /// The declared body exceeds the cap or differs from the delivered length.
    Length,
    /// The correlation identifier is zero.
    RequestId,
    /// The frame body differs from the complete expected JSON object.
    Body,
    /// The reply identifies a different request.
    Correlation,
}

/// A nonzero wire correlation identifier with no connection authority.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Validate)]
pub(super) struct RequestId {
    /// Exact nonzero identifier represented as an unsigned wire integer.
    #[garde(range(min = 1))]
    value: u64,
}

impl RequestId {
    /// Construct an ID through the shared nonzero validation rule.
    ///
    /// # Errors
    ///
    /// Returns `FrameError::RequestId` for zero.
    pub(super) fn new(value: u64) -> Result<Self, FrameError> {
        let id: Self = Self { value };
        id.validate().map_err(|_| FrameError::RequestId)?;
        Ok(id)
    }
}

/// One complete frame candidate in fixed storage.
/// Received bytes still require request or reply decoding before dispatch.
#[must_use]
pub(super) struct Frame {
    /// Complete frame bytes followed by private zeroed storage.
    bytes: [u8; MAX_FRAME_LEN],
    /// Header plus body length within the fixed storage.
    len: usize,
}

impl Frame {
    /// Expose the exact complete frame bytes. Spare buffer bytes stay private.
    #[must_use]
    pub(super) fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

/// An operation disagreed with the one outstanding bounded reply write.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct WriteProgressError;

/// Custody of the one fragment issued from the immutable reply.
enum ReplyWritePhase {
    /// No fragment is outstanding. A nonempty remainder may be reserved.
    Ready,
    /// The exact extent that the live transport may report as completed.
    Pending(NonZeroUsize),
    /// An ambiguous completion permanently fences further writes.
    Ambiguous,
}

/// Own one reply's exact bytes and logical progress independently of native I/O.
/// The transport retains the actual operation through terminal observation.
#[must_use]
pub(super) struct ReplyWrite {
    /// Original bounded response selected by the broker.
    frame: Frame,
    /// Number of bytes from the original prefix confirmed by the transport.
    written: usize,
    /// Whether an extent is queued or its outcome is ambiguous.
    phase: ReplyWritePhase,
}

impl ReplyWrite {
    /// Put one intact response under the serialized write owner.
    pub(super) const fn new(frame: Frame) -> Self {
        Self {
            frame,
            written: 0,
            phase: ReplyWritePhase::Ready,
        }
    }

    /// Check progress without spending a reservation before the owner guard.
    pub(super) const fn can_reserve(&self) -> bool {
        matches!(self.phase, ReplyWritePhase::Ready) && self.written < self.frame.len
    }

    /// Reserve a nonempty prefix of the unwritten bytes on this exact reply.
    /// # Errors
    /// Refuses an outstanding, complete or ambiguous write.
    pub(super) fn reserve(
        &mut self,
        limit: NonZeroUsize,
    ) -> Result<NonZeroUsize, WriteProgressError> {
        if !self.can_reserve() {
            return Err(WriteProgressError);
        }
        let extent: NonZeroUsize =
            NonZeroUsize::new((self.frame.len - self.written).min(limit.get()))
                .ok_or(WriteProgressError)?;
        self.phase = ReplyWritePhase::Pending(extent);
        Ok(extent)
    }

    /// Lend the original immutable fragment reserved for the next native submission.
    pub(super) fn pending_bytes(&self) -> Option<&[u8]> {
        let ReplyWritePhase::Pending(extent) = self.phase else {
            return None;
        };
        self.frame
            .as_bytes()
            .get(self.written..self.written + extent.get())
    }

    /// Account for exactly the reported physical prefix of the reserved fragment.
    /// # Errors
    /// A missing reservation, zero result or overreported count fences this reply.
    pub(super) fn complete(&mut self, count: usize) -> Result<(), WriteProgressError> {
        let phase: ReplyWritePhase = mem::replace(&mut self.phase, ReplyWritePhase::Ambiguous);
        let ReplyWritePhase::Pending(extent) = phase else {
            return Err(WriteProgressError);
        };
        if count == 0 || count > extent.get() {
            return Err(WriteProgressError);
        }
        self.written = self.written.checked_add(count).ok_or(WriteProgressError)?;
        self.phase = ReplyWritePhase::Ready;
        Ok(())
    }

    /// Observe whether the entire reply prefix was physically accounted for.
    pub(super) const fn is_complete(&self) -> bool {
        matches!(self.phase, ReplyWritePhase::Ready) && self.written == self.frame.len
    }
}

/// Own one bounded incomplete frame for an exact wire direction.
#[must_use]
pub(super) struct FrameAssembly {
    /// Fixed storage for the consecutive completed prefix.
    bytes: [u8; MAX_FRAME_LEN],
    /// Bytes appended from completed transport reads.
    used: usize,
    /// Direction required at header admission.
    expected_kind: u8,
    /// Exact total size after the complete header passes envelope checks.
    frame_len: Option<usize>,
}

/// A checked fragment either leaves partial custody or completes one frame.
#[must_use]
pub(super) enum AssemblyProgress {
    /// Keep the accumulated bytes private until the exact frame end.
    Pending(FrameAssembly),
    /// Release one whole frame candidate to the ordinary decoder.
    Complete(Frame),
}

impl FrameAssembly {
    /// Begin an empty assembly for one Engine request.
    pub(super) const fn request() -> Self {
        Self::new(REQUEST)
    }

    /// Begin an empty assembly for one broker reply.
    pub(super) const fn reply() -> Self {
        Self::new(REPLY)
    }

    /// Initialize fixed empty storage for the selected wire direction.
    const fn new(expected_kind: u8) -> Self {
        Self {
            bytes: [0; MAX_FRAME_LEN],
            used: 0,
            expected_kind,
            frame_len: None,
        }
    }

    /// Append bytes observed in one completed read on the same connection.
    /// A refusal consumes the assembly. A returned frame has checked extent
    /// and direction but leaves JSON and correlation admission to the decoder.
    ///
    /// # Errors
    ///
    /// Refuses empty completion, excess bytes or an invalid complete header.
    pub(super) fn push(mut self, completed: &[u8]) -> Result<AssemblyProgress, FrameError> {
        let end: usize = self
            .used
            .checked_add(completed.len())
            .ok_or(FrameError::Length)?;
        if completed.is_empty() || end > self.frame_len.unwrap_or(MAX_FRAME_LEN) {
            return Err(FrameError::Length);
        }
        self.bytes[self.used..end].copy_from_slice(completed);
        self.used = end;

        if self.frame_len.is_none()
            && let Some((header, _)) = self.bytes[..end].split_first_chunk::<HEADER_LEN>()
        {
            let envelope: Envelope = parse_envelope(header, self.expected_kind)?;
            // Envelope admission bounds the sum to the fixed frame capacity.
            let frame_len: usize = HEADER_LEN + envelope.body_len;
            if end > frame_len {
                return Err(FrameError::Length);
            }
            self.frame_len = Some(frame_len);
        }
        if self.frame_len == Some(end) {
            return Ok(AssemblyProgress::Complete(Frame {
                bytes: self.bytes,
                len: end,
            }));
        }
        Ok(AssemblyProgress::Pending(self))
    }
}

/// Closed encoder input. Every body is a literal that fits the fixed buffer.
enum Message {
    /// Request for the current broker availability decision.
    Request,
    /// Redacted result for the matching request.
    Reply(bool),
}

impl Message {
    /// Write the fixed envelope around a body selected only by the closed enum.
    fn encode(self, id: RequestId) -> Frame {
        let (kind, body): (u8, &[u8]) = match self {
            Self::Request => (REQUEST, b"{}"),
            Self::Reply(true) => (REPLY, br#"{"available":true}"#),
            Self::Reply(false) => (REPLY, br#"{"available":false}"#),
        };
        // Every literal is at most 19 bytes, so these two octets preserve its exact length.
        let [length_lo, length_hi, ..] = body.len().to_le_bytes();
        let mut bytes: [u8; MAX_FRAME_LEN] = [0; MAX_FRAME_LEN];
        let (header, body_buffer): (&mut [u8], &mut [u8]) = bytes.split_at_mut(HEADER_LEN);
        header[..8].copy_from_slice(&[VERSION[0], VERSION[1], kind, 0, length_lo, length_hi, 0, 0]);
        header[8..].copy_from_slice(&id.value.to_le_bytes());
        body_buffer[..body.len()].copy_from_slice(body);
        Frame {
            bytes,
            len: HEADER_LEN + body.len(),
        }
    }
}

/// Encode the canonical availability request for an already admitted ID.
pub(super) fn encode_request(id: RequestId) -> Frame {
    Message::Request.encode(id)
}

/// Encode a reply bearing only the original request ID and its redacted status.
pub(super) fn encode_reply(id: RequestId, available: bool) -> Frame {
    Message::Reply(available).encode(id)
}

/// Check the exact bounded envelope before interpreting any JSON bytes.
///
/// # Errors
///
/// Returns the first header, version, direction, length, or identifier refusal.
fn decode_header(frame: &[u8], expected_kind: u8) -> Result<(RequestId, &[u8]), FrameError> {
    let (header, body): (&[u8; HEADER_LEN], &[u8]) = frame
        .split_first_chunk::<HEADER_LEN>()
        .ok_or(FrameError::Header)?;
    let Envelope { body_len, id } = parse_envelope(header, expected_kind)?;
    if body_len != body.len() {
        return Err(FrameError::Length);
    }
    Ok((RequestId::new(u64::from_le_bytes(id))?, body))
}

/// Checked envelope fields consumed by frame assembly and full decoding.
struct Envelope {
    /// Declared body length within the exact protocol cap.
    body_len: usize,
    /// Raw correlation bytes reserved for complete-frame admission.
    id: [u8; 8],
}

/// Check attributes available as soon as the fixed header is complete.
///
/// # Errors
///
/// Refuses unsupported version, wrong direction, reserved bits or excessive length.
fn parse_envelope(header: &[u8; HEADER_LEN], expected_kind: u8) -> Result<Envelope, FrameError> {
    let [
        major,
        minor,
        kind,
        reserved,
        lo,
        hi,
        reserved_lo,
        reserved_hi,
        id @ ..,
    ] = *header;
    if [major, minor] != VERSION {
        return Err(FrameError::Version);
    }
    if kind != expected_kind {
        return Err(FrameError::Kind);
    }
    if [reserved, reserved_lo, reserved_hi] != [0; 3] {
        return Err(FrameError::Header);
    }
    let body_len: usize = usize::from(u16::from_le_bytes([lo, hi]));
    if body_len > MAX_BODY_LEN {
        return Err(FrameError::Length);
    }
    Ok(Envelope { body_len, id })
}

/// Deserialize exactly one map and admit only JSON whitespace after it.
///
/// # Errors
///
/// Returns the redacted body refusal for either schema or completion failure.
fn decode_body<'a, V: Visitor<'a>>(body: &'a [u8], visitor: V) -> Result<V::Value, FrameError> {
    let mut decoder: serde_json::Deserializer<_> = serde_json::Deserializer::from_slice(body);
    let value: V::Value = decoder
        .deserialize_map(visitor)
        .map_err(|_| FrameError::Body)?;
    decoder.end().map_err(|_| FrameError::Body)?;
    Ok(value)
}

/// Map visitor for the request's exact empty object.
struct RequestBody;

impl<'de> Visitor<'de> for RequestBody {
    type Value = ();

    fn expecting(&self, writer: &mut fmt::Formatter<'_>) -> fmt::Result {
        writer.write_str("an empty availability request object")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        if map.next_key::<IgnoredAny>()?.is_some() {
            return Err(A::Error::custom("unexpected request field"));
        }
        Ok(())
    }
}

/// Map visitor for the reply's single required boolean.
struct ReplyBody;

impl<'de> Visitor<'de> for ReplyBody {
    type Value = bool;

    fn expecting(&self, writer: &mut fmt::Formatter<'_>) -> fmt::Result {
        writer.write_str("an availability reply object with one boolean field")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let key: Option<String> = map.next_key()?;
        if key.as_deref() != Some("available") {
            return Err(A::Error::custom("expected availability field"));
        }
        let available: bool = map.next_value()?;
        if map.next_key::<IgnoredAny>()?.is_some() {
            return Err(A::Error::custom("unexpected extra reply field"));
        }
        Ok(available)
    }
}

/// Decode one complete request to its checked correlation ID.
///
/// # Errors
///
/// Returns a redacted refusal for an invalid or incomplete frame.
pub(super) fn decode_request(frame: &[u8]) -> Result<RequestId, FrameError> {
    let (id, body): (RequestId, &[u8]) = decode_header(frame, REQUEST)?;
    decode_body(body, RequestBody)?;
    Ok(id)
}

/// Decode one complete reply for the caller's expected request ID.
///
/// # Errors
///
/// Returns a redacted refusal for an invalid, incomplete or unpaired reply.
pub(super) fn decode_reply(frame: &[u8], expected: RequestId) -> Result<bool, FrameError> {
    let (id, body): (RequestId, &[u8]) = decode_header(frame, REPLY)?;
    if id != expected {
        return Err(FrameError::Correlation);
    }
    decode_body(body, ReplyBody)
}

/// Independent literal and boundary controls for the codec.
#[cfg(test)]
#[path = "private_clock_wire_tests.rs"]
mod tests;

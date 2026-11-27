// SPDX-License-Identifier: GPL-2.0-only
//! Bounded RFC 10049 v1 request and response decoding with borrowed wire inputs.
//!
//! Decoded messages carry only wire facts. Root-key admission, signature
//! verification, request correlation and current-source policy belong to the
//! receiving Engine.

/// Complete request size ceiling, including the twelve-byte transport header.
pub const MAX_REQUEST_BYTES: usize = 4_096;
/// Complete response size ceiling, including the twelve-byte transport header.
pub const MAX_RESPONSE_BYTES: usize = 4_096;
/// Number of bytes in the common packet prefix before the message body.
pub const FRAME_HEADER_BYTES: usize = 12;
/// Encoded little-endian table word.
type Word = [u8; 4];
/// Width shared by this profile's hash, nonce and public-key fields.
type Hash = [u8; 32];
/// Ed25519 signature width.
type Signature = [u8; 64];

/// A structural or numerical refusal with no peer-supplied payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DecodeError {
    /// The complete frame exceeds the decode budget.
    FrameLimit,
    /// The packet magic or exact declared length is invalid.
    Frame,
    /// The count, tags, or offsets do not define a valid message table.
    TagTable,
    /// A required field is absent.
    MissingField,
    /// A required fixed-width field has the wrong size.
    FieldSize,
    /// The packet type is outside the request profile.
    RequestType,
    /// The packet type is outside the response profile.
    ResponseType,
    /// The selected version is unsupported or its advertised list is invalid.
    Version,
    /// The claimed radius is zero.
    Radius,
    /// The signed interval cannot be represented in wire-epoch seconds.
    Interval,
    /// The claimed midpoint lies outside the encoded delegation span.
    Delegation,
    /// The path is misaligned or exceeds 32 nodes.
    MerklePath,
    /// The leaf index has bits beyond the supplied path.
    MerkleIndex,
}

/// A complete request in the v1 profile that commits to a server's root key.
///
/// ```compile_fail,E0616
/// use ntp_wire::roughtime::Request;
/// fn replace_nonce(mut request: Request<'_>) {
///     request.nonce = &[0; 32];
/// }
/// ```
///
/// ```compile_fail,E0599
/// use ntp_wire::roughtime::Request;
/// fn duplicate(request: Request<'_>) {
///     let _duplicate = request.clone();
/// }
/// ```
#[must_use = "retain the checked request for response correlation"]
pub struct Request<'a> {
    /// Entire original framed packet.
    frame: &'a [u8],
    /// Original 32-byte NONC field.
    nonce: &'a Hash,
    /// Original 32-byte SRV field, pending comparison with the configured key.
    server_commitment: &'a Hash,
}

impl<'a> Request<'a> {
    /// Borrow the complete framed request exactly as encoded.
    #[must_use]
    pub const fn frame(&self) -> &'a [u8] {
        self.frame
    }

    /// Borrow the nonce that a corresponding response must echo.
    #[must_use]
    pub const fn nonce(&self) -> &'a [u8; 32] {
        self.nonce
    }

    /// Borrow the claimed commitment to the server's long-term public key.
    #[must_use]
    pub const fn server_commitment(&self) -> &'a [u8; 32] {
        self.server_commitment
    }
}

/// A complete v1 response with validated wire fields and original signed bytes.
///
/// ```compile_fail,E0451
/// use ntp_wire::roughtime::Response;
/// fn replace(response: Response<'_>) {
///     let _ = Response { frame: &[], ..response };
/// }
/// ```
///
/// ```compile_fail,E0616
/// use ntp_wire::roughtime::Response;
/// fn replace_frame(mut response: Response<'_>) {
///     response.frame = &[];
/// }
/// ```
#[must_use = "retain the checked response for request-bound verification"]
pub struct Response<'a> {
    /// Entire original framed packet used for a subsequent chained nonce.
    frame: &'a [u8],
    /// Echoed nonce, pending comparison with the actual request.
    nonce: &'a Hash,
    /// Ordered sibling nodes from the original PATH field.
    path: &'a [Hash],
    /// Leaf index whose bits are limited by the original path depth.
    index: u32,
    /// Original signed response payload.
    signed: SignedResponse<'a>,
    /// Original delegation payload and both unverified signatures.
    certificate: Certificate<'a>,
}

impl<'a> Response<'a> {
    /// Borrow the complete framed response exactly as received.
    #[must_use]
    pub const fn frame(&self) -> &'a [u8] {
        self.frame
    }

    /// Borrow the echoed nonce for the pending request comparison.
    #[must_use]
    pub const fn nonce(&self) -> &'a [u8; 32] {
        self.nonce
    }

    /// Borrow sibling nodes in encoded traversal order.
    #[must_use]
    pub const fn merkle_path(&self) -> &'a [[u8; 32]] {
        self.path
    }

    /// Return the original request leaf index.
    #[must_use]
    pub const fn merkle_index(&self) -> u32 {
        self.index
    }

    /// Borrow the claimed Merkle root for request commitment verification.
    #[must_use]
    pub const fn merkle_root(&self) -> &'a [u8; 32] {
        self.signed.root
    }

    /// Borrow the exact SREP value, including any unknown fields.
    #[must_use]
    pub const fn signed_response(&self) -> &'a [u8] {
        self.signed.bytes
    }

    /// Borrow the exact DELE value, including any unknown fields.
    #[must_use]
    pub const fn signed_delegation(&self) -> &'a [u8] {
        self.certificate.delegation
    }

    /// Borrow the unverified signature over SREP.
    #[must_use]
    pub const fn response_signature(&self) -> &'a [u8; 64] {
        self.certificate.response_signature
    }

    /// Borrow the unverified signature over DELE.
    #[must_use]
    pub const fn delegation_signature(&self) -> &'a [u8; 64] {
        self.certificate.delegation_signature
    }

    /// Borrow the unverified delegated Ed25519 public key.
    #[must_use]
    pub const fn delegated_key(&self) -> &'a [u8; 32] {
        self.certificate.key
    }

    /// Return the claimed midpoint in Unix-epoch seconds.
    #[must_use]
    pub const fn midpoint_seconds(&self) -> u64 {
        self.signed.midpoint
    }

    /// Return the nonzero claimed radius in whole seconds.
    #[must_use]
    pub const fn radius_seconds(&self) -> u32 {
        self.signed.radius
    }

    /// Return the two endpoint values of the protocol's open timing interval.
    #[must_use]
    pub const fn bounds_seconds(&self) -> (u64, u64) {
        self.signed.bounds
    }

    /// Return the inclusive encoded delegation endpoints in Unix-epoch seconds.
    #[must_use]
    pub const fn delegation_seconds(&self) -> (u64, u64) {
        self.certificate.validity
    }
}

/// Decode exactly one bounded request in the single-version, server-committed v1 profile.
///
/// Preserve all original bytes. UDP padding policy belongs to the sending path.
///
/// # Errors
/// Refuses invalid framing, absent or malformed required fields, type, or version.
pub fn decode_request(input: &[u8]) -> Result<Request<'_>, DecodeError> {
    let message: Message<'_> = frame_message(input, MAX_REQUEST_BYTES)?;
    if message.u32_field(*b"TYPE")? != 0 {
        return Err(DecodeError::RequestType);
    }
    if message.u32_field(*b"VER\0")? != 1 {
        return Err(DecodeError::Version);
    }
    Ok(Request {
        frame: input,
        nonce: message.fixed(*b"NONC")?,
        server_commitment: message.fixed(*b"SRV\0")?,
    })
}

/// Decode exactly one bounded RFC 10049 v1 response from immutable storage.
///
/// No allocation, cryptographic work, or external effects occur.
///
/// # Errors
/// Refuses invalid framing, required fields, version, path, or interval.
pub fn decode_response(input: &[u8]) -> Result<Response<'_>, DecodeError> {
    let message: Message<'_> = frame_message(input, MAX_RESPONSE_BYTES)?;
    if message.u32_field(*b"TYPE")? != 1 {
        return Err(DecodeError::ResponseType);
    }
    let (path, partial_node): (&[Hash], &[u8]) = message.field(*b"PATH")?.as_chunks();
    if path.len() > 32 || !partial_node.is_empty() {
        return Err(DecodeError::MerklePath);
    }
    let index: u32 = message.u32_field(*b"INDX")?;
    // Widen before shifting so the complete 32-node path accepts every u32 index.
    if u64::from(index) >> path.len() != 0 {
        return Err(DecodeError::MerkleIndex);
    }
    let signed: SignedResponse<'_> = SignedResponse::parse(message.field(*b"SREP")?)?;
    let certificate: Certificate<'_> = Certificate::parse(
        message.field(*b"CERT")?,
        message.fixed(*b"SIG\0")?,
        signed.midpoint,
    )?;
    Ok(Response {
        frame: input,
        nonce: message.fixed(*b"NONC")?,
        path,
        index,
        signed,
        certificate,
    })
}

/// Bound the declared complete response size before reading a stream body.
///
/// # Errors
/// Refuses invalid magic or a response larger than the complete-frame limit.
pub fn response_frame_length(header: &[u8; FRAME_HEADER_BYTES]) -> Result<usize, DecodeError> {
    let body: usize = declared_body_length(header)?;
    if body > MAX_RESPONSE_BYTES - FRAME_HEADER_BYTES {
        return Err(DecodeError::FrameLimit);
    }
    Ok(FRAME_HEADER_BYTES + body)
}

/// Read the shared packet prefix.
///
/// # Errors
/// Refuses invalid magic or an unrepresentable length.
fn declared_body_length(header: &[u8; FRAME_HEADER_BYTES]) -> Result<usize, DecodeError> {
    let [magic @ .., first, second, third, fourth]: [u8; FRAME_HEADER_BYTES] = *header;
    if magic != *b"ROUGHTIM" {
        return Err(DecodeError::Frame);
    }
    usize::try_from(u32::from_le_bytes([first, second, third, fourth]))
        .map_err(|_| DecodeError::Frame)
}

/// Validate one complete transport frame and interpret its borrowed message.
///
/// # Errors
/// Refuses an oversized or invalid frame or malformed message table.
fn frame_message(input: &[u8], maximum: usize) -> Result<Message<'_>, DecodeError> {
    if input.len() > maximum {
        return Err(DecodeError::FrameLimit);
    }
    let Some((header, body)): Option<(&[u8; FRAME_HEADER_BYTES], &[u8])> =
        input.split_first_chunk()
    else {
        return Err(DecodeError::Frame);
    };
    if declared_body_length(header)? != body.len() {
        return Err(DecodeError::Frame);
    }
    Message::parse(body)
}

/// Exact signed response bytes and their checked time fields.
struct SignedResponse<'a> {
    /// Original value including all tags, offsets and opaque values.
    bytes: &'a [u8],
    /// Root to be matched by the verifier.
    root: &'a Hash,
    /// Claimed processing instant in Unix-epoch seconds.
    midpoint: u64,
    /// Claimed nonzero whole seconds around the midpoint.
    radius: u32,
    /// Exact representable endpoints of the claimed open interval.
    bounds: (u64, u64),
}

impl<'a> SignedResponse<'a> {
    /// Decode SREP and check the full version list and claimed interval.
    ///
    /// # Errors
    /// Refuses absent or malformed fields, version, radius, or interval.
    fn parse(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        let message: Message<'_> = Message::parse(bytes)?;
        let (versions, partial_word): (&[Word], &[u8]) = message.field(*b"VERS")?.as_chunks();
        if message.u32_field(*b"VER\0")? != 1
            || versions.len() > 32
            || !partial_word.is_empty()
            || !versions
                .is_sorted_by(|left, right| u32::from_le_bytes(*left) <= u32::from_le_bytes(*right))
            || !versions.contains(&1_u32.to_le_bytes())
        {
            return Err(DecodeError::Version);
        }
        let midpoint: u64 = message.u64_field(*b"MIDP")?;
        let radius: u32 = message.u32_field(*b"RADI")?;
        if radius == 0 {
            return Err(DecodeError::Radius);
        }
        let start: u64 = midpoint
            .checked_sub(u64::from(radius))
            .ok_or(DecodeError::Interval)?;
        let end: u64 = midpoint
            .checked_add(u64::from(radius))
            .ok_or(DecodeError::Interval)?;
        Ok(Self {
            bytes,
            root: message.fixed(*b"ROOT")?,
            midpoint,
            radius,
            bounds: (start, end),
        })
    }
}

/// Original certificate fields retained for root-key and delegated-key verification.
struct Certificate<'a> {
    /// Signature over SREP stored in the enclosing response.
    response_signature: &'a Signature,
    /// Signature over DELE stored in CERT.
    delegation_signature: &'a Signature,
    /// Exact DELE value including opaque fields.
    delegation: &'a [u8],
    /// Claimed online signing key.
    key: &'a Hash,
    /// Inclusive stated delegation interval in Unix-epoch seconds.
    validity: (u64, u64),
}

impl<'a> Certificate<'a> {
    /// Decode CERT and its DELE and require the claimed midpoint inside the span.
    ///
    /// # Errors
    /// Refuses absent or malformed fields or an invalid delegation span.
    fn parse(
        bytes: &'a [u8],
        response_signature: &'a Signature,
        midpoint: u64,
    ) -> Result<Self, DecodeError> {
        let certificate: Message<'_> = Message::parse(bytes)?;
        let delegation: &'a [u8] = certificate.field(*b"DELE")?;
        let fields: Message<'_> = Message::parse(delegation)?;
        let start: u64 = fields.u64_field(*b"MINT")?;
        let end: u64 = fields.u64_field(*b"MAXT")?;
        if !(start..=end).contains(&midpoint) {
            return Err(DecodeError::Delegation);
        }
        Ok(Self {
            response_signature,
            delegation_signature: certificate.fixed(*b"SIG\0")?,
            delegation,
            key: fields.fixed(*b"PUBK")?,
            validity: (start, end),
        })
    }
}

/// A complete checked tag table with offsets relative to the borrowed values.
struct Message<'a> {
    /// Explicit starts, excluding the first value's implicit start at zero.
    offsets: &'a [Word],
    /// Distinct tags in ascending little-endian numerical order.
    tags: &'a [Word],
    /// Exact values section, including unknown fields.
    values: &'a [u8],
}

impl<'a> Message<'a> {
    /// Partition a bounded message and validate all tags and offsets before lookup.
    ///
    /// # Errors
    /// Refuses an invalid header extent, tag order, or value offset.
    fn parse(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        let (word, rest): (&Word, &[u8]) =
            bytes.split_first_chunk().ok_or(DecodeError::TagTable)?;
        let count: usize =
            usize::try_from(u32::from_le_bytes(*word)).map_err(|_| DecodeError::TagTable)?;
        if count == 0 || count > bytes.len() / 8 {
            return Err(DecodeError::TagTable);
        }
        let (offset_bytes, remainder): (&[u8], &[u8]) = rest.split_at((count - 1) * 4);
        let (tag_bytes, values): (&[u8], &[u8]) = remainder.split_at(count * 4);
        let (offsets, _): (&[Word], &[u8]) = offset_bytes.as_chunks();
        let (tags, _): (&[Word], &[u8]) = tag_bytes.as_chunks();
        if !tags.iter().copied().all(valid_tag)
            || !tags
                .is_sorted_by(|left, right| u32::from_le_bytes(*left) < u32::from_le_bytes(*right))
        {
            return Err(DecodeError::TagTable);
        }
        let mut previous: usize = 0;
        for raw in offsets.iter().copied() {
            let offset: usize = Self::offset(raw)?;
            if !offset.is_multiple_of(4) || offset < previous || offset > values.len() {
                return Err(DecodeError::TagTable);
            }
            previous = offset;
        }
        Ok(Self {
            offsets,
            tags,
            values,
        })
    }

    /// Convert an encoded offset to an index in the borrowed values section.
    ///
    /// # Errors
    /// Refuses an offset outside the target's index representation.
    fn offset(bytes: Word) -> Result<usize, DecodeError> {
        usize::try_from(u32::from_le_bytes(bytes)).map_err(|_| DecodeError::TagTable)
    }

    /// Return the exact required value. Unknown tags remain opaque.
    ///
    /// # Errors
    /// Refuses an absent field or a value range outside the checked table.
    fn field(&self, tag: Word) -> Result<&'a [u8], DecodeError> {
        let index: usize = self
            .tags
            .binary_search_by_key(&u32::from_le_bytes(tag), |raw| u32::from_le_bytes(*raw))
            .map_err(|_| DecodeError::MissingField)?;
        let start: usize = match index.checked_sub(1) {
            Some(previous) => Self::offset(self.offsets[previous])?,
            None => 0,
        };
        let end: usize = match self.offsets.get(index) {
            Some(next) => Self::offset(*next)?,
            None => self.values.len(),
        };
        self.values.get(start..end).ok_or(DecodeError::TagTable)
    }

    /// Require an exact-width field and preserve its original bytes.
    ///
    /// # Errors
    /// Refuses an absent field or a value with a different width.
    fn fixed<const N: usize>(&self, tag: Word) -> Result<&'a [u8; N], DecodeError> {
        self.field(tag)?
            .try_into()
            .map_err(|_| DecodeError::FieldSize)
    }

    /// Read one required little-endian unsigned 32-bit field.
    ///
    /// # Errors
    /// Refuses an absent field or a value with a different width.
    fn u32_field(&self, tag: Word) -> Result<u32, DecodeError> {
        Ok(u32::from_le_bytes(*self.fixed(tag)?))
    }

    /// Read one required little-endian unsigned 64-bit field.
    ///
    /// # Errors
    /// Refuses an absent field or a value with a different width.
    fn u64_field(&self, tag: Word) -> Result<u64, DecodeError> {
        Ok(u64::from_le_bytes(*self.fixed(tag)?))
    }
}

/// Accept one to four uppercase ASCII letters followed by zero padding.
fn valid_tag(tag: Word) -> bool {
    let length: usize = tag
        .iter()
        .take_while(|byte| byte.is_ascii_uppercase())
        .count();
    length != 0 && tag[length..].iter().all(|byte| *byte == 0)
}

// SPDX-License-Identifier: GPL-2.0-only
//! Independent RFC 10049 fixtures through the public inert decoders.
#![forbid(unsafe_code)]

use ntp_wire::roughtime::{
    DecodeError, MAX_REQUEST_BYTES, MAX_RESPONSE_BYTES, Request, Response, decode_request,
    decode_response,
};

#[path = "roughtime/hostile.rs"]
mod hostile;

use hostile::check_untrusted_frame;

/// A fixture field in caller-supplied wire tag order.
type Field<'a> = ([u8; 4], &'a [u8]);

/// A nonce used in both request and response fixtures.
const NONCE: [u8; 32] = [0x23; 32];
/// Inert commitment bytes distinct from the nonce and all response fields.
const SERVER_COMMITMENT: [u8; 32] = [0xcd; 32];
/// Single-version request fields in numerical wire-tag order.
const REQUEST_FIELDS: [Field<'static>; 4] = [
    (*b"VER\0", &[1, 0, 0, 0]),
    (*b"SRV\0", &SERVER_COMMITMENT),
    (*b"NONC", &NONCE),
    (*b"TYPE", &[0, 0, 0, 0]),
];
/// Keep the Merkle root distinct from the nonce and key.
const ROOT: [u8; 32] = [0x45; 32];
/// Distinguish the response signature from the delegation signature.
const RESPONSE_SIGNATURE: [u8; 64] = [0x67; 64];
/// Stand-in bytes for the delegation signature.
const DELEGATION_SIGNATURE: [u8; 64] = [0x89; 64];
/// Stand-in bytes for the delegated key.
const DELEGATED_KEY: [u8; 32] = [0xab; 32];

/// Optional opaque fields exercise distinct parts of the message.
#[derive(Clone, Copy)]
enum ExtraTags<'a> {
    /// Only the required fields.
    None,
    /// Two adjacent unknown fields at the outer level.
    OuterPair,
    /// One opaque field in each signed message.
    Signed,
    /// Opaque outer suffix to reach an exact byte boundary.
    OuterPadding(&'a [u8]),
}

/// Values for synthetic responses. The signature and key bytes are inert fixtures.
#[derive(Clone, Copy)]
struct Fixture<'a> {
    /// Claimed Unix seconds.
    midpoint: u64,
    /// Claimed whole-second radius.
    radius: u32,
    /// Earliest delegated timestamp.
    minimum: u64,
    /// Latest delegated timestamp.
    maximum: u64,
    /// Complete encoded version-list payload.
    versions: &'a [u8],
    /// Flat, opaque bytes for the sibling nodes.
    path: &'a [u8],
    /// Position in the Merkle tree.
    index: u32,
    /// Optional fields for offset, cap and signed-byte tests.
    extra: ExtraTags<'a>,
}

impl Fixture<'_> {
    /// A valid baseline with one version, zero path depth and a bounded interval.
    const BASE: Self = Self {
        midpoint: 1_000,
        radius: 2,
        minimum: 990,
        maximum: 1_010,
        versions: &1_u32.to_le_bytes(),
        path: &[],
        index: 0,
        extra: ExtraTags::None,
    };
}

/// Encode a synthetic message from fields already arranged in wire order.
///
/// # Errors
/// Return an error if the fixture's field count or an offset exceeds a wire word.
fn message(fields: &[Field<'_>]) -> Result<Vec<u8>, String> {
    let count: u32 = u32::try_from(fields.len()).map_err(|error| error.to_string())?;
    let mut bytes: Vec<u8> = count.to_le_bytes().to_vec();
    let mut offset: u32 = 0;
    for (_, value) in fields.iter().take(fields.len().saturating_sub(1)) {
        let length: u32 = u32::try_from(value.len()).map_err(|error| error.to_string())?;
        offset = offset
            .checked_add(length)
            .ok_or("fixture offset overflow")?;
        bytes.extend_from_slice(&offset.to_le_bytes());
    }
    for (tag, _) in fields {
        bytes.extend_from_slice(tag);
    }
    for (_, value) in fields {
        bytes.extend_from_slice(value);
    }
    Ok(bytes)
}

/// Add a transport header to the independent fixture message.
///
/// # Errors
/// Propagates a fixture size that exceeds a wire word.
fn frame(fields: &[Field<'_>]) -> Result<Vec<u8>, String> {
    let message: Vec<u8> = message(fields)?;
    let length: u32 = u32::try_from(message.len()).map_err(|error| error.to_string())?;
    let mut frame: Vec<u8> = b"ROUGHTIM".to_vec();
    frame.extend_from_slice(&length.to_le_bytes());
    frame.extend_from_slice(&message);
    Ok(frame)
}

/// The selected profile borrows its fields and permits opaque fields and padding.
///
/// # Errors
/// Propagates an unexpected fixture construction or decoder failure.
///
/// # Panics
/// Fails if the original byte ranges, fields or padding behavior differ.
#[test]
fn decodes_v1_request_and_borrows_original_fields() -> Result<(), String> {
    let unpadded: Vec<u8> = frame(&REQUEST_FIELDS)?;
    assert_eq!(unpadded.len(), 116);
    assert!(decode_request(&unpadded).is_ok());

    let mut fields: Vec<Field<'_>> = REQUEST_FIELDS.to_vec();
    let padding: [u8; 888] = [0x5a; 888];
    fields.insert(2, (*b"AAAA", b"more"));
    fields.push((*b"ZZZZ", &padding));
    let padded: Vec<u8> = frame(&fields)?;
    let request: Request<'_> = decode_request(&padded).map_err(|error| format!("{error:?}"))?;
    assert_eq!(padded.len(), 1_024);
    assert!(core::ptr::eq(request.frame(), padded.as_slice()));
    assert_eq!(request.server_commitment(), &SERVER_COMMITMENT);
    assert_eq!(
        request.server_commitment().as_ptr(),
        padded[64..96].as_ptr()
    );
    assert_eq!(request.nonce(), &NONCE);
    assert_eq!(request.nonce().as_ptr(), padded[100..132].as_ptr());
    assert_eq!(
        decode_response(&padded).err(),
        Some(DecodeError::ResponseType)
    );
    Ok(())
}

/// Distinguish absent or malformed fields from unsupported request type or version.
///
/// # Errors
/// Propagates an unexpected fixture construction failure.
///
/// # Panics
/// Fails if an invalid request is admitted or refused for a different reason.
#[test]
fn rejects_missing_and_invalid_request_fields() -> Result<(), String> {
    for omitted in 0..REQUEST_FIELDS.len() {
        let mut fields: Vec<Field<'_>> = REQUEST_FIELDS.to_vec();
        let _ = fields.remove(omitted);
        assert_eq!(
            decode_request(&frame(&fields)?).err(),
            Some(DecodeError::MissingField)
        );
    }

    let cases: [(usize, &[u8], DecodeError); 6] = [
        (0, &[2, 0, 0, 0], DecodeError::Version),
        (0, &[1, 0, 0, 0, 2, 0, 0, 0], DecodeError::FieldSize),
        (1, &[0xcd; 28], DecodeError::FieldSize),
        (2, &[0x23; 36], DecodeError::FieldSize),
        (3, &[1, 0, 0, 0], DecodeError::RequestType),
        (3, &[0, 0, 0], DecodeError::FieldSize),
    ];
    for case in cases {
        let (index, value, expected): (usize, &[u8], DecodeError) = case;
        let mut fields: [Field<'_>; 4] = REQUEST_FIELDS;
        fields[index].1 = value;
        assert_eq!(decode_request(&frame(&fields)?).err(), Some(expected));
    }
    Ok(())
}

/// Check header length, exact consumption, table structure and the request byte limit.
///
/// # Errors
/// Propagates an unexpected fixture construction failure.
///
/// # Panics
/// Fails if valid boundary inputs are refused or malformed framing is admitted.
#[test]
fn requires_exact_request_frame_and_bounds_opaque_padding() -> Result<(), String> {
    let request: Vec<u8> = frame(&REQUEST_FIELDS)?;
    for end in [7_usize, 11, 115] {
        assert_eq!(
            decode_request(&request[..end]).err(),
            Some(DecodeError::Frame)
        );
    }
    let mut suffixed: Vec<u8> = request.clone();
    suffixed.push(0);
    assert_eq!(decode_request(&suffixed).err(), Some(DecodeError::Frame));
    let mut invalid_magic: Vec<u8> = request;
    invalid_magic[0] = b'r';
    assert_eq!(
        decode_request(&invalid_magic).err(),
        Some(DecodeError::Frame)
    );

    let mut duplicate_tag: Vec<Field<'_>> = REQUEST_FIELDS.to_vec();
    duplicate_tag.insert(3, (*b"NONC", &NONCE));
    assert_eq!(
        decode_request(&frame(&duplicate_tag)?).err(),
        Some(DecodeError::TagTable)
    );

    assert_eq!(MAX_REQUEST_BYTES, 4_096);
    let padding: [u8; 3_973] = [0x5a; 3_973];
    let mut padded: Vec<Field<'_>> = REQUEST_FIELDS.to_vec();
    padded.push((*b"ZZZZ", &padding[..3_972]));
    let at_cap: Vec<u8> = frame(&padded)?;
    assert_eq!(at_cap.len(), 4_096);
    assert!(decode_request(&at_cap).is_ok());
    padded[4].1 = &padding;
    let over_cap: Vec<u8> = frame(&padded)?;
    assert_eq!(over_cap.len(), 4_097);
    assert_eq!(
        decode_request(&over_cap).err(),
        Some(DecodeError::FrameLimit)
    );
    Ok(())
}

/// Encode the complete v1 response with independent fixture constants.
///
/// # Errors
/// Propagate a fixture length or offset that fails to fit a wire word.
fn response(fixture: Fixture<'_>) -> Result<Vec<u8>, String> {
    let radius: [u8; 4] = fixture.radius.to_le_bytes();
    let midpoint: [u8; 8] = fixture.midpoint.to_le_bytes();
    let minimum: [u8; 8] = fixture.minimum.to_le_bytes();
    let maximum: [u8; 8] = fixture.maximum.to_le_bytes();
    let opaque: [u8; 4] = [0xff; 4];
    let mut signed: Vec<Field<'_>> = vec![
        (*b"VER\0", &[1, 0, 0, 0]),
        (*b"RADI", &radius),
        (*b"MIDP", &midpoint),
        (*b"VERS", fixture.versions),
        (*b"ROOT", &ROOT),
    ];
    let mut delegated: Vec<Field<'_>> = vec![
        (*b"PUBK", &DELEGATED_KEY),
        (*b"MINT", &minimum),
        (*b"MAXT", &maximum),
    ];
    if matches!(fixture.extra, ExtraTags::Signed) {
        signed.insert(1, (*b"AAAA", &opaque));
        delegated.insert(0, (*b"AAAA", &opaque));
    }
    let srep: Vec<u8> = message(&signed)?;
    let dele: Vec<u8> = message(&delegated)?;
    let cert: Vec<u8> = message(&[(*b"SIG\0", &DELEGATION_SIGNATURE), (*b"DELE", &dele)])?;
    let index: [u8; 4] = fixture.index.to_le_bytes();
    let mut outer: Vec<Field<'_>> = vec![
        (*b"SIG\0", &RESPONSE_SIGNATURE),
        (*b"NONC", &NONCE),
        (*b"TYPE", &[1, 0, 0, 0]),
        (*b"PATH", fixture.path),
        (*b"SREP", &srep),
        (*b"CERT", &cert),
        (*b"INDX", &index),
    ];
    match fixture.extra {
        ExtraTags::None | ExtraTags::Signed => {}
        ExtraTags::OuterPair => {
            outer.insert(1, (*b"AAAA", &opaque));
            outer.insert(2, (*b"BBBB", b"more"));
        }
        ExtraTags::OuterPadding(padding) => outer.push((*b"YYYY", padding)),
    }
    frame(&outer)
}

/// Observe the actual public rejection without requiring a debug view of a response.
///
/// # Panics
/// Fails if the decoder admits the input or produces another error class.
fn assert_rejected(input: &[u8], expected: DecodeError) {
    assert_eq!(decode_response(input).err(), Some(expected));
}

/// Ordinary replies preserve values, absolute bounds and the original bytes.
///
/// # Errors
/// Propagates an unexpected fixture construction or decoder failure.
///
/// # Panics
/// Fails if any inert field or borrowed span differs from its independent expectation.
#[test]
fn decodes_single_response_and_borrows_exact_signed_fields() -> Result<(), String> {
    let frame: Vec<u8> = response(Fixture::BASE)?;
    let decoded: Response<'_> = decode_response(&frame).map_err(|error| format!("{error:?}"))?;
    assert_eq!(frame.len(), 416);
    assert_eq!(decoded.frame(), frame.as_slice());
    assert_eq!(decoded.frame().as_ptr(), frame.as_ptr());
    assert_eq!(decoded.nonce(), &NONCE);
    let empty_path: &[[u8; 32]] = &[];
    assert_eq!(decoded.merkle_path(), empty_path);
    assert_eq!(decoded.merkle_index(), 0);
    assert_eq!(decoded.merkle_root(), &ROOT);
    assert_eq!(decoded.response_signature(), &RESPONSE_SIGNATURE);
    assert_eq!(decoded.delegation_signature(), &DELEGATION_SIGNATURE);
    assert_eq!(decoded.delegated_key(), &DELEGATED_KEY);
    assert_eq!(decoded.midpoint_seconds(), 1_000);
    assert_eq!(decoded.radius_seconds(), 2);
    assert_eq!(decoded.bounds_seconds(), (998, 1_002));
    assert_eq!(decoded.delegation_seconds(), (990, 1_010));
    assert_eq!(decoded.signed_response(), &frame[168..260]);
    assert_eq!(decoded.signed_response().as_ptr(), frame[168..260].as_ptr());
    assert_eq!(decoded.signed_delegation(), &frame[340..412]);
    assert_eq!(
        decoded.signed_delegation().as_ptr(),
        frame[340..412].as_ptr()
    );
    Ok(())
}

/// A complete opaque tail reaches the cap, while framing still consumes every byte.
///
/// # Errors
/// Propagates an unexpected fixture construction failure.
///
/// # Panics
/// Fails on cap rejection at the exact boundary or on malformed frame admission.
#[test]
fn requires_exact_frame_and_checks_byte_cap() -> Result<(), String> {
    let mut frame: Vec<u8> = response(Fixture::BASE)?;
    assert_rejected(&frame[..7], DecodeError::Frame);
    assert_rejected(&frame[..11], DecodeError::Frame);
    assert_rejected(&frame[..415], DecodeError::Frame);
    frame.push(0);
    assert_rejected(&frame, DecodeError::Frame);
    let mut invalid_magic: Vec<u8> = response(Fixture::BASE)?;
    invalid_magic[0] = b'r';
    assert_rejected(&invalid_magic, DecodeError::Frame);
    let opaque_tail: Vec<u8> = response(Fixture {
        extra: ExtraTags::OuterPadding(&[0xff; 3]),
        ..Fixture::BASE
    })?;
    assert_eq!(opaque_tail.len(), 427);
    assert!(decode_response(&opaque_tail).is_ok());
    let empty_tail: Vec<u8> = response(Fixture {
        extra: ExtraTags::OuterPadding(&[]),
        ..Fixture::BASE
    })?;
    assert_eq!(empty_tail.len(), 424);
    assert!(decode_response(&empty_tail).is_ok());

    assert_eq!(MAX_RESPONSE_BYTES, 4_096);
    let mut padding: Vec<u8> = vec![0xff; 3_672];
    let at_cap: Vec<u8> = response(Fixture {
        extra: ExtraTags::OuterPadding(&padding),
        ..Fixture::BASE
    })?;
    assert_eq!(at_cap.len(), MAX_RESPONSE_BYTES);
    assert!(decode_response(&at_cap).is_ok());
    padding.push(0xff);
    let mut over_cap: Vec<u8> = response(Fixture {
        extra: ExtraTags::OuterPadding(&padding),
        ..Fixture::BASE
    })?;
    assert_eq!(over_cap.len(), MAX_RESPONSE_BYTES + 1);
    assert_rejected(&over_cap, DecodeError::FrameLimit);
    over_cap[0] = b'r';
    assert_rejected(&over_cap, DecodeError::FrameLimit);
    Ok(())
}

/// Generate a repeatable raw and length-correct sample at every admitted size.
///
/// # Errors
/// Returns an unexpected conversion failure in the bounded fixture length.
///
/// # Panics
/// Panics on a parser failure or violation of the independent transport check.
#[test]
fn untrusted_raw_and_framed_inputs_cover_every_length() -> Result<(), String> {
    let mut data: [u8; 4_097] = [0; 4_097];
    let mut seed: u32 = 0x18c7_a391;
    for length in 0..=data.len() {
        let sample: &mut [u8] = &mut data[..length];
        for byte in sample.iter_mut() {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            *byte = seed.to_be_bytes()[0];
        }
        check_untrusted_frame(sample);
        if length >= 12 {
            let body: u32 = u32::try_from(length - 12).map_err(|error| error.to_string())?;
            sample[..8].copy_from_slice(b"ROUGHTIM");
            sample[8..12].copy_from_slice(&body.to_le_bytes());
            check_untrusted_frame(sample);
        }
    }
    Ok(())
}

/// Cross every path depth with timestamp and radius limits and opaque signed fields.
///
/// # Errors
/// Returns an unexpected construction or valid-response refusal.
///
/// # Panics
/// Panics on incorrect accepted field values or a malformed neighbor's acceptance.
#[test]
fn structured_wire_corpus_pairs_boundary_responses_with_malformed_neighbors() -> Result<(), String>
{
    let path: [u8; 1_028] = [0x9b; 1_028];
    for depth in 0..=32_u32 {
        let nodes: usize = usize::try_from(depth).map_err(|error| error.to_string())?;
        let index: u32 = u32::MAX.checked_shr(32 - depth).unwrap_or_default();
        for radius in [1_u32, 17, 65_535, u32::MAX] {
            let radius_seconds: u64 = u64::from(radius);
            for (midpoint, bounds) in [
                (radius_seconds, (0, radius_seconds * 2)),
                (
                    u64::MAX - radius_seconds,
                    (u64::MAX - radius_seconds * 2, u64::MAX),
                ),
            ] {
                for extra in [ExtraTags::None, ExtraTags::Signed] {
                    let fixture: Fixture<'_> = Fixture {
                        midpoint,
                        radius,
                        minimum: midpoint,
                        maximum: midpoint,
                        path: &path[..nodes * 32],
                        index,
                        extra,
                        ..Fixture::BASE
                    };
                    let valid: Vec<u8> = response(fixture)?;
                    let parsed: Response<'_> =
                        decode_response(&valid).map_err(|error| format!("{error:?}"))?;
                    assert!(core::ptr::eq(parsed.frame(), valid.as_slice()));
                    assert_eq!(parsed.bounds_seconds(), bounds);
                    assert_eq!(parsed.delegation_seconds(), (midpoint, midpoint));
                    assert_eq!(parsed.merkle_index(), index);
                    assert_eq!(parsed.merkle_path().as_flattened(), fixture.path);

                    let incomplete_node: Vec<u8> = response(Fixture {
                        // Keep following message offsets word-aligned so the
                        // malformed path reaches the inner PATH length check.
                        path: &path[..nodes * 32 + 4],
                        ..fixture
                    })?;
                    assert_rejected(&incomplete_node, DecodeError::MerklePath);
                    if depth < 32 {
                        let extra_index_bit: Vec<u8> = response(Fixture {
                            index: index + 1,
                            ..fixture
                        })?;
                        assert_rejected(&extra_index_bit, DecodeError::MerkleIndex);
                    }
                    let outside_delegation: Vec<u8> = response(Fixture {
                        minimum: 0,
                        maximum: midpoint - 1,
                        ..fixture
                    })?;
                    assert_rejected(&outside_delegation, DecodeError::Delegation);
                }
            }
        }
    }
    Ok(())
}

/// Equal offsets preserve empty values while aligned, ordered ranges remain mandatory.
///
/// # Errors
/// Propagates an unexpected fixture construction failure.
///
/// # Panics
/// Fails if valid unknown fields are rejected or a malformed tag table is admitted.
#[test]
fn admits_opaque_fields_and_rejects_bad_offsets_and_tags() -> Result<(), String> {
    let frame: Vec<u8> = response(Fixture {
        extra: ExtraTags::OuterPair,
        ..Fixture::BASE
    })?;
    assert_eq!(frame.len(), 440);
    assert_eq!(&frame[16..28], &[64, 0, 0, 0, 68, 0, 0, 0, 72, 0, 0, 0]);
    assert!(decode_response(&frame).is_ok());

    let mut mutated: Vec<u8> = frame.clone();
    mutated[20..24].copy_from_slice(&64_u32.to_le_bytes());
    assert!(decode_response(&mutated).is_ok());
    for invalid_offset in [67_u32, 76, u32::MAX - 3] {
        mutated[20..24].copy_from_slice(&invalid_offset.to_le_bytes());
        assert_rejected(&mutated, DecodeError::TagTable);
    }
    mutated[20..24].copy_from_slice(&68_u32.to_le_bytes());
    mutated[56..60].copy_from_slice(b"AAAA");
    assert_rejected(&mutated, DecodeError::TagTable);
    mutated[52..56].copy_from_slice(b"BBBB");
    assert_rejected(&mutated, DecodeError::TagTable);
    mutated[52..56].copy_from_slice(b"AAAA");
    mutated[56..60].copy_from_slice(b"bBBB");
    assert_rejected(&mutated, DecodeError::TagTable);

    let mut impossible_count: Vec<u8> = frame;
    impossible_count[12..16].copy_from_slice(&u32::MAX.to_le_bytes());
    assert_rejected(&impossible_count, DecodeError::TagTable);
    impossible_count[12..16].copy_from_slice(&0_u32.to_le_bytes());
    assert_rejected(&impossible_count, DecodeError::TagTable);
    Ok(())
}

/// Opaque signed values remain in both borrowed messages.
///
/// # Errors
/// Propagates an unexpected fixture construction or decoder failure.
///
/// # Panics
/// Fails if traversal rebuilds either signed message or changes the known fields.
#[test]
fn preserves_unknown_fields_inside_exact_signed_slices() -> Result<(), String> {
    let frame: Vec<u8> = response(Fixture {
        extra: ExtraTags::Signed,
        ..Fixture::BASE
    })?;
    let decoded: Response<'_> = decode_response(&frame).map_err(|error| format!("{error:?}"))?;
    assert_eq!(frame.len(), 440);
    assert_eq!(&frame[220..224], &[0xff; 4]);
    assert_eq!(&frame[384..388], &[0xff; 4]);
    assert_eq!(decoded.signed_response(), &frame[168..272]);
    assert_eq!(decoded.signed_response().as_ptr(), frame[168..272].as_ptr());
    assert_eq!(decoded.signed_delegation(), &frame[352..436]);
    assert_eq!(
        decoded.signed_delegation().as_ptr(),
        frame[352..436].as_ptr()
    );
    assert_eq!(decoded.bounds_seconds(), (998, 1_002));
    assert_eq!(decoded.delegated_key(), &DELEGATED_KEY);
    Ok(())
}

/// Required fields, their widths, and the response type each have a separate refusal.
///
/// # Errors
/// Propagates an unexpected fixture construction failure.
///
/// # Panics
/// Fails if a required field, fixed width, or response discriminant is bypassed.
#[test]
fn requires_known_fields_widths_and_response_discriminant() -> Result<(), String> {
    let frame: Vec<u8> = response(Fixture::BASE)?;
    assert!(decode_response(&frame).is_ok());
    let mut without_nonce: Vec<u8> = frame.clone();
    without_nonce[44..48].copy_from_slice(b"MONC");
    assert_rejected(&without_nonce, DecodeError::MissingField);
    let mut request_type: Vec<u8> = frame;
    request_type[164..168].copy_from_slice(&0_u32.to_le_bytes());
    assert_rejected(&request_type, DecodeError::ResponseType);

    let mut short_signature: Vec<u8> = response(Fixture {
        extra: ExtraTags::OuterPair,
        ..Fixture::BASE
    })?;
    short_signature[16..20].copy_from_slice(&60_u32.to_le_bytes());
    assert_rejected(&short_signature, DecodeError::FieldSize);
    Ok(())
}

/// Inspect the entire ordered list and include the negotiated v1 on both sides.
///
/// # Errors
/// Propagates an unexpected fixture construction failure.
///
/// # Panics
/// Fails if the parser silently drops a malformed suffix or accepts a bad version.
#[test]
fn validates_the_entire_raw_version_list() -> Result<(), String> {
    let nine: Vec<u8> = (1_u32..=9).flat_map(u32::to_le_bytes).collect();
    let frame: Vec<u8> = response(Fixture {
        versions: &nine,
        ..Fixture::BASE
    })?;
    assert!(decode_response(&frame).is_ok());

    let mut wrong_selected: Vec<u8> = frame;
    wrong_selected[208..212].copy_from_slice(&2_u32.to_le_bytes());
    assert_rejected(&wrong_selected, DecodeError::Version);
    let unsorted: Vec<u8> = [1_u32, 2, 3, 4, 5, 6, 7, 9, 8]
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect();
    let missing_v1: Vec<u8> = (2_u32..=10).flat_map(u32::to_le_bytes).collect();
    let max: Vec<u8> = (1_u32..=32).flat_map(u32::to_le_bytes).collect();
    let over_max: Vec<u8> = (1_u32..=33).flat_map(u32::to_le_bytes).collect();
    let max_frame: Vec<u8> = response(Fixture {
        versions: &max,
        ..Fixture::BASE
    })?;
    assert!(decode_response(&max_frame).is_ok());
    let repeated: Vec<u8> = [1_u32, 1, 256]
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect();
    let repeated_frame: Vec<u8> = response(Fixture {
        versions: &repeated,
        ..Fixture::BASE
    })?;
    assert!(decode_response(&repeated_frame).is_ok());
    for invalid in [&unsorted[..], &missing_v1[..], &over_max[..], &[]] {
        let invalid_frame: Vec<u8> = response(Fixture {
            versions: invalid,
            ..Fixture::BASE
        })?;
        assert_rejected(&invalid_frame, DecodeError::Version);
    }
    Ok(())
}

/// Decode both arithmetic endpoints and refuse invalid signed numeric claims.
///
/// # Errors
/// Propagates an unexpected fixture construction or decoder failure.
///
/// # Panics
/// Fails on wrapped, saturated, narrowed or zero-radius bounds.
#[test]
fn checks_radius_and_interval_arithmetic() -> Result<(), String> {
    for (midpoint, expected) in [
        (1_u64, (0_u64, 2_u64)),
        (u64::MAX - 1, (u64::MAX - 2, u64::MAX)),
    ] {
        let frame: Vec<u8> = response(Fixture {
            midpoint,
            radius: 1,
            minimum: 0,
            maximum: u64::MAX,
            ..Fixture::BASE
        })?;
        let decoded: Response<'_> =
            decode_response(&frame).map_err(|error| format!("{error:?}"))?;
        assert_eq!(decoded.bounds_seconds(), expected);
    }
    for (midpoint, radius, error) in [
        (0_u64, 1_u32, DecodeError::Interval),
        (u64::MAX, 1, DecodeError::Interval),
        (1_000, 0, DecodeError::Radius),
    ] {
        let frame: Vec<u8> = response(Fixture {
            midpoint,
            radius,
            minimum: 0,
            maximum: u64::MAX,
            ..Fixture::BASE
        })?;
        assert_rejected(&frame, error);
    }
    Ok(())
}

/// Delegation admits both endpoints and excludes neighboring midpoints.
///
/// # Errors
/// Propagates an unexpected fixture construction or decoder failure.
///
/// # Panics
/// Fails if an endpoint is excluded or an invalid delegation is accepted.
#[test]
fn checks_inclusive_delegation_span() -> Result<(), String> {
    for midpoint in [990_u64, 1_010] {
        let frame: Vec<u8> = response(Fixture {
            midpoint,
            ..Fixture::BASE
        })?;
        let decoded: Response<'_> =
            decode_response(&frame).map_err(|error| format!("{error:?}"))?;
        assert_eq!(decoded.midpoint_seconds(), midpoint);
        assert_eq!(decoded.delegation_seconds(), (990, 1_010));
    }
    for midpoint in [989_u64, 1_011] {
        let frame: Vec<u8> = response(Fixture {
            midpoint,
            ..Fixture::BASE
        })?;
        assert_rejected(&frame, DecodeError::Delegation);
    }
    let inverted: Vec<u8> = response(Fixture {
        minimum: 1_010,
        maximum: 990,
        ..Fixture::BASE
    })?;
    assert_rejected(&inverted, DecodeError::Delegation);
    Ok(())
}

/// Check the index boundary from empty through full-depth paths.
///
/// # Errors
/// Propagates an unexpected fixture construction or decoder failure.
///
/// # Panics
/// Fails for dropped node bytes, unused index-bit acceptance or an invalid path width.
#[test]
fn checks_merkle_path_lengths_and_used_index_bits() -> Result<(), String> {
    let mut siblings: [[u8; 32]; 32] = [[0x9e; 32]; 32];
    siblings[0] = [0x63; 32];
    siblings[30] = [0xd1; 32];
    siblings[31] = [0xf5; 32];
    for (depth, index) in [(0_usize, 0_u32), (1, 1), (31, 0x7fff_ffff), (32, u32::MAX)] {
        let frame: Vec<u8> = response(Fixture {
            path: siblings[..depth].as_flattened(),
            index,
            ..Fixture::BASE
        })?;
        let decoded: Response<'_> =
            decode_response(&frame).map_err(|error| format!("{error:?}"))?;
        assert_eq!(decoded.merkle_path(), &siblings[..depth]);
        assert_eq!(decoded.merkle_index(), index);
    }
    for (depth, index) in [(0_usize, 1_u32), (1, 2), (31, 0x8000_0000)] {
        let frame: Vec<u8> = response(Fixture {
            path: siblings[..depth].as_flattened(),
            index,
            ..Fixture::BASE
        })?;
        assert_rejected(&frame, DecodeError::MerkleIndex);
    }
    let misaligned_node: Vec<u8> = response(Fixture {
        path: &[0x9e; 28],
        ..Fixture::BASE
    })?;
    assert_rejected(&misaligned_node, DecodeError::MerklePath);
    let too_deep: Vec<u8> = response(Fixture {
        path: &[0x9e; 1_056],
        ..Fixture::BASE
    })?;
    assert_rejected(&too_deep, DecodeError::MerklePath);
    Ok(())
}

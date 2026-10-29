//! Independent complete-datagram facts through the actual public framing seam.
#![forbid(unsafe_code)]

use ntp_wire::{
    ExtensionFields, FrameError, FrameLimits, RawDatagram, RawExtension,
    frame_no_legacy_mac_datagram,
};

/// Literal fixed-header facts; intentionally includes semantically inadmissible bits.
const HEADER: [u8; 48] = [
    0xff, 0xfe, 0x80, 0x7f, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x13, 0x57, 0x9b, 0xdf,
    0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
    0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32, 0x10, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];

/// Literal final field: unknown full-width type, 28 total bytes, opaque padding.
const FINAL_FIELD: [u8; 28] = [
    0x80, 0x01, 0x00, 0x1c, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
    0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x00, 0x00, 0x00, 0x00,
];

/// Literal permitted nonfinal field of 16 bytes, with no semantic type admission.
const FIRST_FIELD: [u8; 16] = [
    0xff, 0xff, 0x00, 0x10, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab,
];

/// Build bounded fixture bytes, never using any production encoder or decoder.
///
/// The tuple supplies raw type, declared/actual field length and body-fill octet.
/// Test callers supply at most 65 fields, each of 4..=4052 bytes, and a complete
/// fixture at most 4200 bytes. Deliberately invalid lengths retain their raw bits.
///
/// # Panics
/// Fails if a test attempts to exceed the predeclared fixture-building bounds.
fn packet(fields: &[(u16, u16, u8)]) -> Vec<u8> {
    assert!(fields.len() <= 65);
    let mut bytes = HEADER.to_vec();
    for &(field_type, length, fill) in fields {
        assert!((4..=4052).contains(&length));
        bytes.extend_from_slice(&field_type.to_be_bytes());
        bytes.extend_from_slice(&length.to_be_bytes());
        bytes.resize(bytes.len() + usize::from(length) - 4, fill);
    }
    assert!(bytes.len() <= 4200);
    bytes
}

/// Build one 76-byte literal frame without a production-formatting oracle.
fn literal_single() -> Vec<u8> {
    let mut bytes = HEADER.to_vec();
    bytes.extend_from_slice(&FINAL_FIELD);
    bytes
}

/// Compare a finite independent span list and require stable iterator exhaustion.
///
/// The loop is bounded by the expected fixtures, never by iterator progress.
/// A non-advancing iterator fails a field/exhaustion assertion without hanging.
/// Body expectations include all literal padding after the four-byte prefix.
///
/// # Panics
/// Fails on count, ordering, type, span, padding, advancement or exhaustion errors.
fn assert_fields(datagram: &RawDatagram<'_>, expected: &[(u16, &[u8])]) {
    assert_eq!(datagram.extension_count(), expected.len());
    let mut fields = datagram.extensions();
    for &(field_type, bytes) in expected {
        assert!(bytes.len() >= 16);
        let actual = fields
            .next()
            .map(|field| (field.field_type(), field.bytes(), field.body()));
        assert_eq!(actual, Some((field_type, bytes, &bytes[4..])));
    }
    assert!(fields.next().is_none());
    assert!(fields.next().is_none());
}

/// Reject outside constructor ranges while accepting both endpoints and odd caps.
///
/// # Errors
/// Propagates an unexpected valid-ceiling rejection, providing constructor RED.
///
/// # Panics
/// Fails if any valid constructor range endpoint is rejected or altered.
#[test]
fn frame_limits_valid_endpoints_and_unaligned_ceiling() -> Result<(), FrameError> {
    for (bytes, count) in [(48, 0), (48, 64), (75, 1), (4096, 0), (4096, 64)] {
        let limits = FrameLimits::new(bytes, count)?;
        assert_eq!(limits, FrameLimits::new(bytes, count)?);
    }
    Ok(())
}

/// Every immediate invalid range neighbor rejects without exposing usable limits.
///
/// # Panics
/// Fails if a byte/count endpoint can bypass checked private construction.
#[test]
fn frame_limits_invalid_neighbors_are_rejected() {
    for (bytes, count) in [
        (0, 0),
        (47, 0),
        (4097, 0),
        (usize::MAX, 0),
        (48, 65),
        (48, usize::MAX),
    ] {
        assert_eq!(
            FrameLimits::new(bytes, count),
            Err(FrameError::InvalidLimits)
        );
    }
}

/// A bare literal header is legal with zero fields and preserves inert raw bits.
///
/// # Errors
/// Propagates unexpected valid limits or complete bare-header rejection.
///
/// # Panics
/// Fails on framing, raw-header changes or an invented extension.
#[test]
fn bare_header_is_a_complete_inert_datagram() -> Result<(), FrameError> {
    let datagram = frame_no_legacy_mac_datagram(&HEADER, FrameLimits::new(48, 0)?)?;
    assert_fields(&datagram, &[]);
    assert_eq!(datagram.header().leap_indicator(), 3);
    assert_eq!(datagram.header().version(), 7);
    assert_eq!(datagram.header().mode(), 7);
    assert_eq!(datagram.header().stratum(), 254);
    assert_eq!(datagram.header().root_delay_bits(), 0x0123_4567);
    assert_eq!(
        datagram.header().origin_timestamp().bits(),
        0x8899_aabb_ccdd_eeff
    );
    assert_eq!(datagram.header().transmit_timestamp().bits(), u64::MAX);
    Ok(())
}

/// Exactly 76 bytes retain one complete 28-byte field and its opaque padding.
///
/// # Errors
/// Propagates an unexpected healthy single-field rejection.
///
/// # Panics
/// Fails if literal field spans, padding or header values change.
#[test]
fn literal_single_field_and_padding_are_preserved() -> Result<(), FrameError> {
    let bytes = literal_single();
    assert_eq!(bytes.len(), 76);
    let datagram = frame_no_legacy_mac_datagram(&bytes, FrameLimits::new(76, 1)?)?;
    assert_fields(&datagram, &[(0x8001, &FINAL_FIELD)]);
    assert_eq!(datagram.header().stratum(), 254);
    assert_eq!(bytes[72..76], [0, 0, 0, 0]);
    Ok(())
}

/// Exactly 92 bytes admit a short nonfinal field followed by the final minimum.
///
/// # Errors
/// Propagates an unexpected healthy multi-field rejection.
///
/// # Panics
/// Fails if distinct literal types/order/spans are lost or iteration shares state.
#[test]
fn literal_multiple_fields_keep_order_and_repeat_iteration() -> Result<(), FrameError> {
    let mut bytes = HEADER.to_vec();
    bytes.extend_from_slice(&FIRST_FIELD);
    bytes.extend_from_slice(&FINAL_FIELD);
    assert_eq!(bytes.len(), 92);
    let datagram = frame_no_legacy_mac_datagram(&bytes, FrameLimits::new(92, 2)?)?;
    let expected: [(u16, &[u8]); 2] = [(0xffff, &FIRST_FIELD), (0x8001, &FINAL_FIELD)];
    assert_fields(&datagram, &expected);
    assert_fields(&datagram, &expected);
    let mut first = datagram.extensions();
    let mut second = datagram.extensions();
    assert_eq!(first.next().map(|field| field.field_type()), Some(0xffff));
    assert_eq!(first.next().map(|field| field.field_type()), Some(0x8001));
    assert_eq!(second.next().map(|field| field.field_type()), Some(0xffff));
    assert_eq!(second.next().map(|field| field.field_type()), Some(0x8001));
    assert!(first.next().is_none());
    assert!(second.next().is_none());
    Ok(())
}

/// Unknown full-width types and an opaque Authenticator-looking prefix all survive.
///
/// # Errors
/// Propagates unexpected rejection of a structurally valid unknown-type chain.
///
/// # Panics
/// Fails on type narrowing, recognition-based filtering or requiring 0x0404 last.
#[test]
fn unknown_types_and_nonfinal_authenticator_type_are_opaque() -> Result<(), FrameError> {
    let bytes = packet(&[(0x0404, 16, 0x41), (0xffff, 16, 0x42), (0x8001, 28, 0x43)]);
    let datagram = frame_no_legacy_mac_datagram(&bytes, FrameLimits::new(108, 3)?)?;
    assert_fields(
        &datagram,
        &[
            (0x0404, &bytes[48..64]),
            (0xffff, &bytes[64..80]),
            (0x8001, &bytes[80..108]),
        ],
    );
    assert_eq!(u16::from_be_bytes([bytes[48], bytes[49]]), 0x0404);
    assert_eq!(u16::from_be_bytes([bytes[64], bytes[65]]), 0xffff);
    assert_eq!(u16::from_be_bytes([bytes[80], bytes[81]]), 0x8001);
    Ok(())
}

/// Every short fixed header rejects beside the exact 48-byte healthy control.
///
/// # Errors
/// Propagates unexpected valid limits or healthy-header rejection.
///
/// # Panics
/// Fails when any short header is admitted or the error loses its bounded class.
#[test]
fn every_short_header_rejects_with_exact_healthy_neighbor() -> Result<(), FrameError> {
    let limits = FrameLimits::new(76, 1)?;
    for length in 0..48 {
        assert_eq!(
            frame_no_legacy_mac_datagram(&HEADER[..length], limits).err(),
            Some(FrameError::HeaderTooShort),
            "header length {length}"
        );
    }
    assert_fields(&frame_no_legacy_mac_datagram(&HEADER, limits)?, &[]);
    Ok(())
}

/// Every proper prefix after a complete header rejects; the whole frame succeeds.
///
/// # Errors
/// Propagates unexpected valid limits or complete literal-frame rejection.
///
/// # Panics
/// Fails on prefix truncation acceptance or missing complete consumption.
#[test]
fn all_extension_prefix_truncations_reject() -> Result<(), FrameError> {
    let bytes = literal_single();
    let limits = FrameLimits::new(76, 1)?;
    for length in 49..52 {
        assert_eq!(
            frame_no_legacy_mac_datagram(&bytes[..length], limits).err(),
            Some(FrameError::InvalidExtensionLength),
            "incomplete prefix {length}"
        );
    }
    for length in 52..76 {
        assert_eq!(
            frame_no_legacy_mac_datagram(&bytes[..length], limits).err(),
            Some(FrameError::TruncatedExtension),
            "truncated field {length}"
        );
    }
    assert_fields(
        &frame_no_legacy_mac_datagram(&bytes, limits)?,
        &[(0x8001, &FINAL_FIELD)],
    );
    Ok(())
}

/// Reject malformed alignment/minima/endpoints beside legal 16+28 and final 28.
///
/// # Errors
/// Propagates unexpected valid limits or valid neighboring frame rejection.
///
/// # Panics
/// Fails on a wrong endpoint, truncated span, alignment rule or final minimum.
#[test]
fn declared_length_endpoints_and_aligned_neighbors() -> Result<(), FrameError> {
    let limits = FrameLimits::new(128, 2)?;
    for length in [0_u16, 4, 12, 15, 16, 27, 29, 65532, 65535] {
        let mut bytes = literal_single();
        bytes[50..52].copy_from_slice(&length.to_be_bytes());
        assert!(
            frame_no_legacy_mac_datagram(&bytes, limits).is_err(),
            "length {length}"
        );
    }
    for length in [16_u16, 20, 24] {
        let bytes = packet(&[(1, length, 0x5a)]);
        assert_eq!(
            frame_no_legacy_mac_datagram(&bytes, limits).err(),
            Some(FrameError::FinalExtensionTooShort)
        );
    }
    // Complete two-field chains isolate the nonfinal minimum independently of
    // truncation, leftover bytes or the final minimum.
    for first_length in [4_u16, 8, 12, 15] {
        let bytes = packet(&[(1, first_length, 0x5a), (2, 28, 0xa5)]);
        assert_eq!(
            frame_no_legacy_mac_datagram(&bytes, limits).err(),
            Some(FrameError::InvalidExtensionLength)
        );
    }
    // Both extents fit and meet the minima. Their combined tail is aligned,
    // so only per-field alignment rejects these complete partitions.
    for (first_length, final_length) in [
        (17, 31),
        (18, 30),
        (19, 29),
        (21, 31),
        (22, 30),
        (23, 29),
        (25, 31),
        (26, 30),
        (27, 29),
    ] {
        let bytes = packet(&[(1, first_length, 0x5a), (2, final_length, 0xa5)]);
        assert_eq!(
            frame_no_legacy_mac_datagram(&bytes, limits).err(),
            Some(FrameError::InvalidExtensionLength)
        );
    }
    let bytes = packet(&[(1, 16, 0x5a), (2, 28, 0xa5)]);
    assert_fields(
        &frame_no_legacy_mac_datagram(&bytes, limits)?,
        &[(1, &bytes[48..64]), (2, &bytes[64..92])],
    );
    let single = literal_single();
    assert_fields(
        &frame_no_legacy_mac_datagram(&single, limits)?,
        &[(0x8001, &FINAL_FIELD)],
    );
    Ok(())
}

/// A valid prefix cannot conceal malformed trailing bytes or a short final field.
///
/// # Errors
/// Propagates unexpected valid limits or healthy complete-chain rejection.
///
/// # Panics
/// Fails if a partial chain is published or extra tail bytes are discarded.
#[test]
fn malformed_suffix_after_good_prefix_rejects_atomically() -> Result<(), FrameError> {
    let limits = FrameLimits::new(128, 3)?;
    let single = literal_single();
    for suffix in [
        &[0xab][..],
        &[0xab, 0xcd][..],
        &[0xab, 0xcd, 0xef][..],
        &[0, 1, 0, 0][..],
    ] {
        let mut bytes = single.clone();
        bytes.extend_from_slice(suffix);
        assert_eq!(
            frame_no_legacy_mac_datagram(&bytes, limits).err(),
            Some(FrameError::InvalidExtensionLength)
        );
    }
    let short_final = packet(&[(1, 28, 0xaa), (2, 16, 0xbb)]);
    assert_eq!(
        frame_no_legacy_mac_datagram(&short_final, limits).err(),
        Some(FrameError::FinalExtensionTooShort)
    );
    assert_fields(
        &frame_no_legacy_mac_datagram(&single, limits)?,
        &[(0x8001, &FINAL_FIELD)],
    );
    Ok(())
}

/// No 4/20/24-byte trailer is guessed away; valid MAC-looking syntax stays opaque.
///
/// # Errors
/// Propagates unexpected valid limits or opaque valid-neighbor rejection.
///
/// # Panics
/// Fails if legacy trailer guessing truncates input or invents authentication.
#[test]
fn explicit_no_mac_grammar_rejects_trailers_and_preserves_ambiguity() -> Result<(), FrameError> {
    let limits = FrameLimits::new(128, 2)?;
    for trailer_len in [4, 20, 24] {
        let mut bare = HEADER.to_vec();
        bare.resize(48 + trailer_len, 0);
        assert!(frame_no_legacy_mac_datagram(&bare, limits).is_err());
        let mut after_field = literal_single();
        after_field.resize(76 + trailer_len, 0);
        assert!(frame_no_legacy_mac_datagram(&after_field, limits).is_err());
    }
    for length in [20_u16, 24] {
        let declared_short = packet(&[(0x1234, length, 0xff)]);
        assert_eq!(
            frame_no_legacy_mac_datagram(&declared_short, limits).err(),
            Some(FrameError::FinalExtensionTooShort)
        );
    }
    let opaque_mac_lookalike = packet(&[(1, 28, 0xff)]);
    assert_fields(
        &frame_no_legacy_mac_datagram(&opaque_mac_lookalike, limits)?,
        &[(1, &opaque_mac_lookalike[48..76])],
    );
    Ok(())
}

/// Whole-input ceilings apply before traversal, including odd ceilings and maxima.
///
/// # Errors
/// Propagates unexpected valid limits or accepted-byte-neighbor rejection.
///
/// # Panics
/// Fails if B-1/B/B+1, a ceiling endpoint or rejection precedence changes.
#[test]
fn byte_budget_neighbors_and_experimental_maximum() -> Result<(), FrameError> {
    let bytes = literal_single();
    assert_eq!(
        frame_no_legacy_mac_datagram(&bytes, FrameLimits::new(75, 1)?).err(),
        Some(FrameError::ByteLimit)
    );
    for ceiling in [76, 77] {
        assert_fields(
            &frame_no_legacy_mac_datagram(&bytes, FrameLimits::new(ceiling, 1)?)?,
            &[(0x8001, &FINAL_FIELD)],
        );
    }
    let limits = FrameLimits::new(76, 1)?;
    assert_eq!(
        frame_no_legacy_mac_datagram(&bytes[..75], limits).err(),
        Some(FrameError::TruncatedExtension)
    );
    let mut oversized = bytes.clone();
    oversized.push(0);
    assert_eq!(
        frame_no_legacy_mac_datagram(&oversized, limits).err(),
        Some(FrameError::ByteLimit)
    );
    for (length, total) in [(4044_u16, 4092), (4048, 4096)] {
        let large = packet(&[(1, length, 0x5a)]);
        assert_eq!(large.len(), total);
        assert_fields(
            &frame_no_legacy_mac_datagram(&large, FrameLimits::new(4096, 1)?)?,
            &[(1, &large[48..])],
        );
    }
    let too_large = packet(&[(1, 4052, 0x5a)]);
    assert_eq!(too_large.len(), 4100);
    assert_eq!(
        frame_no_legacy_mac_datagram(&too_large, FrameLimits::new(4096, 1)?).err(),
        Some(FrameError::ByteLimit)
    );
    let mut malformed_oversize = too_large;
    malformed_oversize[50..52].copy_from_slice(&0_u16.to_be_bytes());
    assert_eq!(
        frame_no_legacy_mac_datagram(&malformed_oversize, FrameLimits::new(4096, 0)?).err(),
        Some(FrameError::ByteLimit)
    );
    Ok(())
}

/// Count ceilings 0/1/63/64 reject the next field, with 65 never a valid budget.
///
/// # Errors
/// Propagates unexpected valid limits or legal count-neighbor rejection.
///
/// # Panics
/// Fails on counting, count-budget precedence or loss of the 64-field ceiling.
#[test]
fn extension_count_neighbors_and_experimental_maximum() -> Result<(), FrameError> {
    assert_fields(
        &frame_no_legacy_mac_datagram(&HEADER, FrameLimits::new(4096, 0)?)?,
        &[],
    );
    let single = literal_single();
    assert_eq!(
        frame_no_legacy_mac_datagram(&single, FrameLimits::new(4096, 0)?).err(),
        Some(FrameError::TooManyExtensions)
    );
    assert_fields(
        &frame_no_legacy_mac_datagram(&single, FrameLimits::new(4096, 1)?)?,
        &[(0x8001, &FINAL_FIELD)],
    );
    for count in [63, 64, 65] {
        let mut declarations = vec![(1, 16, 0x51); count - 1];
        declarations.push((2, 28, 0x52));
        let bytes = packet(&declarations);
        assert_eq!(bytes.len(), 16 * count + 60);
        if count <= 64 {
            let datagram = frame_no_legacy_mac_datagram(&bytes, FrameLimits::new(4096, count)?)?;
            let mut expected = Vec::new();
            for index in 0..count - 1 {
                expected.push((1, &bytes[48 + 16 * index..64 + 16 * index]));
            }
            expected.push((2, &bytes[bytes.len() - 28..]));
            assert_fields(&datagram, &expected);
            assert_eq!(
                frame_no_legacy_mac_datagram(&bytes, FrameLimits::new(4096, count - 1)?).err(),
                Some(FrameError::TooManyExtensions)
            );
        } else {
            assert_eq!(bytes.len(), 1100);
            assert_eq!(
                frame_no_legacy_mac_datagram(&bytes, FrameLimits::new(4096, 64)?).err(),
                Some(FrameError::TooManyExtensions)
            );
        }
    }
    let malformed_first = packet(&[(1, 4, 0)]);
    assert_eq!(
        frame_no_legacy_mac_datagram(&malformed_first, FrameLimits::new(4096, 0)?).err(),
        Some(FrameError::TooManyExtensions)
    );
    Ok(())
}

/// Metadata remains fixed-size and drop-free, independent of retained packet size.
///
/// # Panics
/// Fails if views acquire packet copies, owned heap resources or large error data.
#[test]
fn packet_views_and_errors_have_bounded_metadata() {
    let word = size_of::<usize>();
    assert!(size_of::<RawDatagram<'_>>() <= 48 + 3 * word);
    assert!(size_of::<RawExtension<'_>>() <= 2 * word);
    assert!(size_of::<ExtensionFields<'_>>() <= 2 * word);
    assert!(size_of::<FrameError>() <= word);
    assert!(!core::mem::needs_drop::<RawDatagram<'_>>());
    assert!(!core::mem::needs_drop::<RawExtension<'_>>());
    assert!(!core::mem::needs_drop::<ExtensionFields<'_>>());
    let error = FrameError::TruncatedExtension;
    assert_eq!(format!("{error:?}"), "TruncatedExtension");
}

/// Exhaust all 65536 declared length bit patterns against one fixed 76-byte input.
///
/// Independent truth is exactly one field spanning all 28 tail bytes; every
/// other declaration has a short, unaligned, truncated or leftover representation.
/// The test bounds allocation and iterator steps regardless of decoder behavior.
///
/// # Errors
/// Propagates unexpected valid limits or the unique valid length rejection.
///
/// # Panics
/// Fails on an accepted invalid u16 length, byte order or a wrong complete span.
#[test]
fn exhaustive_u16_declared_lengths_have_one_valid_projection() -> Result<(), FrameError> {
    let limits = FrameLimits::new(76, 1)?;
    let mut bytes = literal_single();
    for length in 0_u16..=u16::MAX {
        bytes[50..52].copy_from_slice(&length.to_be_bytes());
        let result = frame_no_legacy_mac_datagram(&bytes, limits);
        if length == 28 {
            assert_fields(&result?, &[(0x8001, &FINAL_FIELD)]);
        } else {
            assert!(result.is_err(), "declared length {length}");
        }
    }
    bytes[50..52].copy_from_slice(&[0x1c, 0x00]);
    assert_eq!(
        frame_no_legacy_mac_datagram(&bytes, limits).err(),
        Some(FrameError::TruncatedExtension)
    );
    bytes[50..52].copy_from_slice(&[0x00, 0x1c]);
    assert_fields(
        &frame_no_legacy_mac_datagram(&bytes, limits)?,
        &[(0x8001, &FINAL_FIELD)],
    );
    Ok(())
}

/// Finite generated chains preserve independent offsets and reject every suffix.
///
/// # Errors
/// Propagates unexpected valid limits or legal generated-chain rejection.
///
/// # Panics
/// Fails on chain boundaries, complete consumption or a malformed final neighbor.
#[test]
fn finite_generated_chains_have_exact_spans_and_complete_consumption() -> Result<(), FrameError> {
    let limits = FrameLimits::new(4096, 64)?;
    for first_length in [16_u16, 20, 24, 28, 32] {
        for final_length in [28_u16, 32, 36] {
            let bytes = packet(&[(0x8001, first_length, 0xa5), (0xffff, final_length, 0x5a)]);
            let boundary = 48 + usize::from(first_length);
            let datagram = frame_no_legacy_mac_datagram(&bytes, limits)?;
            assert_fields(
                &datagram,
                &[(0x8001, &bytes[48..boundary]), (0xffff, &bytes[boundary..])],
            );
            let mut truncated = bytes.clone();
            assert_eq!(truncated.pop(), Some(0x5a));
            assert_eq!(
                frame_no_legacy_mac_datagram(&truncated, limits).err(),
                Some(FrameError::TruncatedExtension)
            );
            let mut suffixed = bytes;
            suffixed.push(0x80);
            assert_eq!(
                frame_no_legacy_mac_datagram(&suffixed, limits).err(),
                Some(FrameError::InvalidExtensionLength)
            );
        }
    }
    Ok(())
}

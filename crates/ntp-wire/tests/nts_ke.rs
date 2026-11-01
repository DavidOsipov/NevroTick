//! Independent literal NTS-KE envelope facts; no TLS or negotiated-state claim.
#![forbid(unsafe_code)]

use ntp_wire::{NtsKeFrameError, NtsKeFrameLimits, RawNtsKeMessage, frame_nts_ke_message};

/// Protocol0/AEAD15-shaped bytes, used only as independent generic framing facts.
static NEGOTIATION: [u8; 16] = [
    0x80, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x04, 0x00, 0x02, 0x00, 0x0f, 0x80, 0x00, 0x00, 0x00,
];
/// The independent final critical empty type0 record.
static EOM: [u8; 4] = [0x80, 0x00, 0x00, 0x00];

/// Inspect a finite literal span list without relying on iterator progress.
/// # Panics
/// Exact count/type/critical/body/spans and stable exhaustion must match.
fn assert_records(frame: &RawNtsKeMessage<'_>, expected: &[(u16, bool, &[u8])]) {
    assert_eq!(frame.record_count(), expected.len());
    let mut records = frame.records();
    for &(kind, critical, bytes) in expected {
        assert_eq!(
            records.next().map(|record| {
                assert!(core::ptr::eq(record.bytes().as_ptr(), bytes.as_ptr()));
                assert!(core::ptr::eq(record.body().as_ptr(), bytes[4..].as_ptr()));
                (
                    record.record_type(),
                    record.critical(),
                    record.bytes(),
                    record.body(),
                )
            }),
            Some((kind, critical, bytes, &bytes[4..]))
        );
    }
    assert!(records.next().is_none());
    assert!(records.next().is_none());
}

/// Check real range neighbors without installing a parser default.
/// # Errors
/// Preserve an unexpected rejection of a valid private limits constructor.
/// # Panics
/// Valid endpoints/odd ceilings must admit; invalid neighbors must reject.
#[test]
fn ke_framing_limits_have_explicit_checked_endpoints() -> Result<(), NtsKeFrameError> {
    for (bytes, records) in [(4, 1), (9, 2), (65_536, 16_384), (131_072, 32_768)] {
        let _limits = NtsKeFrameLimits::new(bytes, records)?;
    }
    for (bytes, records) in [
        (0, 1),
        (3, 1),
        (131_073, 1),
        (usize::MAX, 1),
        (4, 0),
        (4, 32_769),
    ] {
        assert_eq!(
            NtsKeFrameLimits::new(bytes, records),
            Err(NtsKeFrameError::InvalidLimits)
        );
    }
    Ok(())
}

/// Independent complete bytes retain body-only lengths and a separate critical bit.
/// # Errors
/// Preserve an unexpected framing rejection of the literal complete message.
/// # Panics
/// Record boundaries and unmodified input borrowing must match the literals.
#[test]
fn ke_literal_message_preserves_original_record_spans() -> Result<(), NtsKeFrameError> {
    let frame = frame_nts_ke_message(&NEGOTIATION, NtsKeFrameLimits::new(16, 3)?)?;
    assert!(core::ptr::eq(frame.bytes().as_ptr(), NEGOTIATION.as_ptr()));
    assert_records(
        &frame,
        &[
            (1, true, &NEGOTIATION[..6]),
            (4, false, &NEGOTIATION[6..12]),
            (0, true, &NEGOTIATION[12..]),
        ],
    );
    let end_only = frame_nts_ke_message(&EOM, NtsKeFrameLimits::new(4, 1)?)?;
    assert_records(&end_only, &[(0, true, &EOM)]);
    Ok(())
}

/// Unknown records and odd bodies remain raw rather than gaining receiver approval.
/// # Errors
/// Preserve valid structural neighbors; semantics are a later separate owner.
/// # Panics
/// Both critical states and all15 type bits survive without alignment rounding.
#[test]
fn ke_unknown_record_representation_and_unaligned_bodies_are_inert() -> Result<(), NtsKeFrameError>
{
    for first in [0x12, 0x92] {
        let bytes = [
            first, 0x34, 0x00, 0x03, 0xaa, 0xbb, 0xcc, 0x80, 0x00, 0x00, 0x00,
        ];
        let frame = frame_nts_ke_message(&bytes, NtsKeFrameLimits::new(11, 2)?)?;
        assert_records(
            &frame,
            &[(0x1234, first == 0x92, &bytes[..7]), (0, true, &bytes[7..])],
        );
    }
    for first in [0x7f, 0xff] {
        let bytes = [first, 0xff, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00];
        let frame = frame_nts_ke_message(&bytes, NtsKeFrameLimits::new(8, 2)?)?;
        assert_records(
            &frame,
            &[(0x7fff, first == 0xff, &bytes[..4]), (0, true, &bytes[4..])],
        );
    }
    // One body octet occupies five total bytes, unlike NTP extension lengths.
    let bytes = [0x00, 0x05, 0x00, 0x01, 0x55, 0x80, 0x00, 0x00, 0x00];
    let frame = frame_nts_ke_message(&bytes, NtsKeFrameLimits::new(9, 2)?)?;
    assert_records(&frame, &[(5, false, &bytes[..5]), (0, true, &bytes[5..])]);
    Ok(())
}

/// Every strict prefix has an independently specified finite-buffer failure.
/// # Errors
/// Preserve an unexpected rejection of the full healthy control.
/// # Panics
/// Prefix/body/EOM absence classifications must match, not imply transport EOF.
#[test]
fn ke_finite_buffer_prefixes_distinguish_header_body_and_missing_eom() -> Result<(), NtsKeFrameError>
{
    use NtsKeFrameError::{
        MissingEndOfMessage as Missing, TruncatedBody as Body, TruncatedHeader as Header,
    };
    let expected = [
        Missing, Header, Header, Header, Body, Body, Missing, Header, Header, Header, Body, Body,
        Missing, Header, Header, Header,
    ];
    let limits = NtsKeFrameLimits::new(16, 3)?;
    for (cut, error) in expected.iter().copied().enumerate() {
        assert_eq!(
            frame_nts_ke_message(&NEGOTIATION[..cut], limits).err(),
            Some(error)
        );
    }
    let _healthy = frame_nts_ke_message(&NEGOTIATION, limits)?;
    Ok(())
}

/// Final marker shape and whole-buffer consumption are independent guards.
/// # Errors
/// Preserve a valid empty EOM neighbor after each hostile complete buffer.
/// # Panics
/// Noncritical/nonempty EOM and any trailing data must reject without a message.
#[test]
fn ke_eom_shape_duplicate_and_trailing_bytes_reject() -> Result<(), NtsKeFrameError> {
    let limits = NtsKeFrameLimits::new(32, 8)?;
    for invalid in [
        &[0x00, 0x00, 0x00, 0x00][..],
        &[0x80, 0x00, 0x00, 0x01, 0x55][..],
    ] {
        assert_eq!(
            frame_nts_ke_message(invalid, limits).err(),
            Some(NtsKeFrameError::InvalidEndOfMessage)
        );
        let _healthy = frame_nts_ke_message(&EOM, limits)?;
    }
    for invalid in [
        &[0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00][..],
        &[0x80, 0x00, 0x00, 0x00, 0x55][..],
    ] {
        assert_eq!(
            frame_nts_ke_message(invalid, limits).err(),
            Some(NtsKeFrameError::TrailingData)
        );
        let _healthy = frame_nts_ke_message(&EOM, limits)?;
    }
    Ok(())
}

/// Byte/count ceilings reject before another unavailable or malformed prefix.
/// # Errors
/// Preserve the full valid neighboring selection under the larger actual budgets.
/// # Panics
/// Explicit byte and count exhaustion must not be disguised as another error.
#[test]
fn ke_byte_and_record_limits_have_distinct_healthy_neighbors() -> Result<(), NtsKeFrameError> {
    assert_eq!(
        frame_nts_ke_message(&NEGOTIATION, NtsKeFrameLimits::new(15, 3)?).err(),
        Some(NtsKeFrameError::ByteLimit)
    );
    assert_eq!(
        frame_nts_ke_message(&NEGOTIATION, NtsKeFrameLimits::new(16, 2)?).err(),
        Some(NtsKeFrameError::TooManyRecords)
    );
    let _healthy = frame_nts_ke_message(&NEGOTIATION, NtsKeFrameLimits::new(16, 3)?)?;
    let prefix = [0x12, 0x34, 0x00, 0x00, 0xff];
    assert_eq!(
        frame_nts_ke_message(&prefix, NtsKeFrameLimits::new(5, 1)?).err(),
        Some(NtsKeFrameError::TooManyRecords)
    );
    assert_eq!(
        frame_nts_ke_message(&[0x00; 5], NtsKeFrameLimits::new(4, 1)?).err(),
        Some(NtsKeFrameError::ByteLimit)
    );
    Ok(())
}

/// Wire maximum body length and the accepted65,536-byte profile remain distinct.
/// # Errors
/// Preserve real large-record bounds and the exact whole-buffer acceptance target.
/// # Panics
/// The wire body's maximum is65,535; length does not include its four-byte prefix.
#[test]
fn ke_large_body_and_response_acceptance_boundary() -> Result<(), NtsKeFrameError> {
    let mut bytes = vec![0x5a; 65_543];
    bytes[..4].copy_from_slice(&[0x12, 0x34, 0xff, 0xff]);
    bytes[65_539..].copy_from_slice(&EOM);
    let limits = NtsKeFrameLimits::new(65_543, 2)?;
    let frame = frame_nts_ke_message(&bytes, limits)?;
    assert_large_records(
        &frame,
        &bytes,
        &[(0x1234, false, 0, 65_535), (0, true, 65_539, 0)],
    );
    assert_eq!(
        frame_nts_ke_message(&bytes, NtsKeFrameLimits::new(65_542, 2)?).err(),
        Some(NtsKeFrameError::ByteLimit)
    );
    // One fewer body octet is truncated; the complete first record lacks EOM.
    assert_eq!(
        frame_nts_ke_message(&bytes[..65_538], limits).err(),
        Some(NtsKeFrameError::TruncatedBody)
    );
    assert_eq!(
        frame_nts_ke_message(&bytes[..65_539], limits).err(),
        Some(NtsKeFrameError::MissingEndOfMessage)
    );
    let mut exact = vec![0x5a; 65_536];
    exact[..4].copy_from_slice(&[0x12, 0x34, 0xff, 0xf8]);
    exact[65_532..].copy_from_slice(&EOM);
    let accepted = frame_nts_ke_message(&exact, NtsKeFrameLimits::new(65_536, 2)?)?;
    assert_eq!(accepted.record_count(), 2);
    assert_eq!(accepted.bytes().len(), 65_536);
    Ok(())
}

/// Zero-body unknowns still make four-byte progress at the full record budget.
/// # Errors
/// Preserve exact128KiB framing without an accidental64-record extension cap.
/// # Panics
/// A bounded32,768-step observer must reach its final EOM and stable exhaustion.
#[test]
fn ke_record_count_bound_preserves_short_record_messages() -> Result<(), NtsKeFrameError> {
    let mut bytes = vec![0; 131_072];
    for raw in bytes[..131_068].as_chunks_mut::<4>().0 {
        raw.copy_from_slice(&[0x12, 0x34, 0x00, 0x00]);
    }
    bytes[131_068..].copy_from_slice(&EOM);
    let frame = frame_nts_ke_message(&bytes, NtsKeFrameLimits::new(131_072, 32_768)?)?;
    assert_eq!(frame.record_count(), 32_768);
    let mut records = frame.records();
    for index in 0..32_768 {
        let expected = if index == 32_767 { 0 } else { 0x1234 };
        assert_eq!(
            records.next().map(|record| record.record_type()),
            Some(expected)
        );
    }
    assert!(records.next().is_none());
    assert!(records.next().is_none());
    Ok(())
}

/// Inspect original large spans without formatting their payload on failure.
/// # Panics
/// Literal type/critical/offset/body-length facts and finite exhaustion must hold.
fn assert_large_records(
    frame: &RawNtsKeMessage<'_>,
    input: &[u8],
    expected: &[(u16, bool, usize, usize)],
) {
    assert_eq!(frame.bytes().len(), input.len());
    assert!(core::ptr::eq(frame.bytes().as_ptr(), input.as_ptr()));
    assert_eq!(frame.record_count(), expected.len());
    let mut records = frame.records();
    for &(kind, critical, offset, body_length) in expected {
        assert_eq!(
            records.next().map(|record| {
                assert_eq!(record.bytes().len(), body_length + 4);
                assert_eq!(record.body().len(), body_length);
                assert!(core::ptr::eq(
                    record.bytes().as_ptr(),
                    input[offset..].as_ptr()
                ));
                assert!(core::ptr::eq(
                    record.body().as_ptr(),
                    input[offset + 4..].as_ptr()
                ));
                if body_length != 0 {
                    assert_eq!(record.body().first(), Some(&0x5a));
                    assert_eq!(record.body().last(), Some(&0x5a));
                }
                (record.record_type(), record.critical())
            }),
            Some((kind, critical))
        );
    }
    assert!(records.next().is_none());
    assert!(records.next().is_none());
}

/// Complete literal envelopes straddle the canonical 65,536-byte target.
/// # Errors
/// Preserve every healthy well-framed neighbor under the larger explicit budget.
/// # Panics
/// Header lengths exclude four-byte prefixes and the separate final EOM.
#[test]
fn ke_complete_message_lengths_straddle_65536() -> Result<(), NtsKeFrameError> {
    let larger = NtsKeFrameLimits::new(65_537, 2)?;
    let boundary = NtsKeFrameLimits::new(65_536, 2)?;
    for (length, body_low, body_length, end_offset, expected_error) in [
        (65_535, 0xf7, 65_527, 65_531, None),
        (65_536, 0xf8, 65_528, 65_532, None),
        (
            65_537,
            0xf9,
            65_529,
            65_533,
            Some(NtsKeFrameError::ByteLimit),
        ),
    ] {
        let mut bytes = vec![0x5a; length];
        bytes[..4].copy_from_slice(&[0x12, 0x34, 0xff, body_low]);
        bytes[end_offset..].copy_from_slice(&EOM);
        let frame = frame_nts_ke_message(&bytes, larger)?;
        assert_eq!(frame.bytes().len(), length);
        assert_large_records(
            &frame,
            &bytes,
            &[(0x1234, false, 0, body_length), (0, true, end_offset, 0)],
        );
        assert_eq!(frame_nts_ke_message(&bytes, boundary).err(), expected_error);
    }
    Ok(())
}

/// Independently complete three-record buffers straddle the absolute byte cap.
/// # Errors
/// Preserve construction of the explicit 128-KiB experiment ceiling.
/// # Panics
/// The first two complete messages admit; the plus-one buffer rejects by bytes.
#[test]
fn ke_complete_message_lengths_straddle_128_kib() -> Result<(), NtsKeFrameError> {
    let limits = NtsKeFrameLimits::new(131_072, 3)?;
    for (length, body_low, second_body, end_offset, expected_error) in [
        (131_071, 0xf4, 65_524, 131_067, None),
        (131_072, 0xf5, 65_525, 131_068, None),
        (
            131_073,
            0xf6,
            65_526,
            131_069,
            Some(NtsKeFrameError::ByteLimit),
        ),
    ] {
        let mut bytes = vec![0x5a; length];
        bytes[..4].copy_from_slice(&[0x12, 0x34, 0xff, 0xff]);
        bytes[65_539..65_543].copy_from_slice(&[0x12, 0x35, 0xff, body_low]);
        bytes[end_offset..].copy_from_slice(&EOM);
        match frame_nts_ke_message(&bytes, limits) {
            Ok(frame) => {
                assert_eq!(expected_error, None);
                assert_eq!(frame.bytes().len(), length);
                assert_large_records(
                    &frame,
                    &bytes,
                    &[
                        (0x1234, false, 0, 65_535),
                        (0x1235, false, 65_539, second_body),
                        (0, true, end_offset, 0),
                    ],
                );
            }
            Err(error) => assert_eq!(Some(error), expected_error),
        }
    }
    Ok(())
}

/// Count-limit neighbors all fit the independent sixteen-byte ceiling.
/// # Errors
/// Preserve the checked count-two experiment without adopting a parser default.
/// # Panics
/// One/two records admit; three reject by count without a masking byte rejection.
#[test]
fn ke_isolated_record_limit_has_below_exact_and_above_neighbors() -> Result<(), NtsKeFrameError> {
    let two = [0x12, 0x34, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00];
    let three = [
        0x12, 0x34, 0x00, 0x00, 0x12, 0x34, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00,
    ];
    let limits = NtsKeFrameLimits::new(16, 2)?;
    for (bytes, count, expected_error) in [
        (&EOM[..], 1, None),
        (&two[..], 2, None),
        (&three[..], 3, Some(NtsKeFrameError::TooManyRecords)),
    ] {
        match frame_nts_ke_message(bytes, limits) {
            Ok(frame) => {
                assert_eq!(expected_error, None);
                assert_eq!(frame.record_count(), count);
            }
            Err(error) => assert_eq!(Some(error), expected_error),
        }
    }
    Ok(())
}

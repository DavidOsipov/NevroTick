// SPDX-License-Identifier: GPL-2.0-only
//! Independent literal NTS plaintext-framing neighbors and boundary cases.
#![forbid(unsafe_code)]

use ntp_wire::{NtsPlaintextError, NtsPlaintextLimits, RawNtsPlaintext, frame_nts_plaintext};

/// Construct explicit fixture budgets while preserving the actual fixed error.
/// # Errors
/// Propagate an unexpected rejection of these independently selected budgets.
fn limits(bytes: usize, fields: usize) -> Result<NtsPlaintextLimits, String> {
    NtsPlaintextLimits::new(bytes, fields).map_err(|error| format!("limits: {error:?}"))
}

/// Exercise the real reader; error variants contain no input or plaintext data.
/// # Errors
/// Preserve checked-budget and complete-framing rejection as test failures.
fn framed(input: &[u8], bytes: usize, fields: usize) -> Result<RawNtsPlaintext<'_>, String> {
    frame_nts_plaintext(input, limits(bytes, fields)?)
        .map_err(|error| format!("plaintext: {error:?}"))
}

/// Empty input remains valid with both budgets zero.
/// # Errors
/// Propagate unexpected checked-budget or empty-input rejection.
/// # Panics
/// Require no fields and rejection of the neighboring nonempty input.
#[test]
fn empty_sequence_and_zero_budgets() -> Result<(), String> {
    let zero = limits(0, 0)?;
    let frame = framed(&[], 0, 0)?;
    assert_eq!(frame.field_count(), 0);
    assert!(frame.fields().next().is_none());
    assert!(matches!(
        frame_nts_plaintext(&[0x12, 0x34, 0, 4], zero),
        Err(NtsPlaintextError::ByteLimit)
    ));
    Ok(())
}

/// Independent short literals preserve their exact borrowed byte ranges.
/// # Errors
/// Propagate actual rejection or absence of an independently expected field.
/// # Panics
/// Require literal types, original pointers, opaque bodies and repeatable traversal.
#[test]
fn independent_short_fields_retain_original_bytes() -> Result<(), String> {
    let input = [0x12, 0x34, 0, 4, 0x02, 0x04, 0, 8, 0xde, 0xad, 0xbe, 0xef];
    let frame = framed(&input, 12, 2)?;
    assert_eq!(frame.field_count(), 2);
    let mut fields = frame.fields();
    let first = fields.next().ok_or("missing first literal field")?;
    assert_eq!(first.field_type(), 0x1234);
    assert_eq!(first.bytes(), &input[..4]);
    assert_eq!(first.bytes().as_ptr(), input.as_ptr());
    assert_eq!(first.body(), []);
    let second = fields.next().ok_or("missing second literal field")?;
    assert_eq!(second.field_type(), 0x0204);
    assert_eq!(second.bytes(), &input[4..]);
    assert_eq!(second.bytes().as_ptr(), input[4..].as_ptr());
    assert_eq!(second.body(), &[0xde, 0xad, 0xbe, 0xef]);
    assert!(fields.next().is_none());
    assert!(fields.next().is_none());
    assert_eq!(frame.fields().count(), 2);
    Ok(())
}

/// Malformed first or late fields expose no partially validated sequence.
/// # Errors
/// Propagate an unexpected rejection of the explicitly valid fixture budgets.
/// # Panics
/// Require the independently specified length or truncation cause for every case.
#[test]
fn malformed_lengths_and_incomplete_prefixes_never_yield_a_frame() -> Result<(), String> {
    let budget = limits(64, 8)?;
    for input in [
        &b"\x12"[..],
        &b"\x12\x34"[..],
        &b"\x12\x34\x00"[..],
        &b"\x12\x34\x00\x00"[..],
        &b"\x12\x34\x00\x02"[..],
        &b"\x12\x34\x00\x05\x00"[..],
        &b"\x12\x34\x00\x04\x01\x02\x03"[..],
        &b"\x12\x34\x00\x04\x01\x02\x00\x00"[..],
    ] {
        assert!(matches!(
            frame_nts_plaintext(input, budget),
            Err(NtsPlaintextError::InvalidFieldLength)
        ));
    }
    for input in [
        &b"\x12\x34\x00\x08\xaa\xbb"[..],
        &b"\x12\x34\x00\x04\x01\x02\x00\x08\xaa\xbb"[..],
        &b"\x12\x34\xff\xfc"[..],
    ] {
        assert!(matches!(
            frame_nts_plaintext(input, budget),
            Err(NtsPlaintextError::TruncatedField)
        ));
    }
    Ok(())
}

/// Exact budgets succeed while their immediate smaller neighbors reject.
/// # Errors
/// Propagate unexpected rejection of a healthy input or in-range budget.
/// # Panics
/// Require byte-before-field precedence and rejection above implementation ceilings.
#[test]
fn explicit_budget_boundaries_and_error_precedence() -> Result<(), String> {
    let input = [0x12, 0x34, 0, 4, 0x56, 0x78, 0, 4];
    assert!(frame_nts_plaintext(&input, limits(8, 2)?).is_ok());
    assert!(matches!(
        frame_nts_plaintext(&input, limits(7, 2)?),
        Err(NtsPlaintextError::ByteLimit)
    ));
    assert!(matches!(
        frame_nts_plaintext(&input, limits(8, 1)?),
        Err(NtsPlaintextError::TooManyFields)
    ));
    assert!(matches!(
        frame_nts_plaintext(&input[..4], limits(4, 0)?),
        Err(NtsPlaintextError::TooManyFields)
    ));
    assert!(matches!(
        NtsPlaintextLimits::new(4097, 1),
        Err(NtsPlaintextError::InvalidLimits)
    ));
    assert!(matches!(
        NtsPlaintextLimits::new(4, 1025),
        Err(NtsPlaintextError::InvalidLimits)
    ));
    Ok(())
}

/// The complete maximum-size sequence contains 1024 four-byte fields.
/// # Errors
/// Propagate unexpected maximum-input or in-range-budget rejection.
/// # Panics
/// Require the exact count, full traversal and both immediate budget rejections.
#[test]
fn maximum_sequence_contains_1024_four_byte_fields() -> Result<(), String> {
    let mut input = [0_u8; 4096];
    for field in input.as_chunks_mut::<4>().0 {
        field.copy_from_slice(&[0x12, 0x34, 0, 4]);
    }
    let frame = framed(&input, 4096, 1024)?;
    assert_eq!(frame.field_count(), 1024);
    assert_eq!(frame.fields().count(), 1024);
    assert!(matches!(
        frame_nts_plaintext(&input, limits(4096, 1023)?),
        Err(NtsPlaintextError::TooManyFields)
    ));
    assert!(matches!(
        frame_nts_plaintext(&input, limits(4095, 1024)?),
        Err(NtsPlaintextError::ByteLimit)
    ));
    Ok(())
}

/// A 256-byte literal requires the nonzero high byte of the network-order length.
/// # Errors
/// Propagate actual rejection or absence of the independently expected field.
/// # Panics
/// Require the single original span/body and rejection at the 255-byte ceiling.
#[test]
fn single_field_with_nonzero_length_high_byte() -> Result<(), String> {
    let mut input = [0x5a; 256];
    input[..4].copy_from_slice(&[0x12, 0x34, 0x01, 0x00]);
    let frame = framed(&input, 256, 1)?;
    assert_eq!(frame.field_count(), 1);
    let mut fields = frame.fields();
    let field = fields.next().ok_or("missing 256-byte literal field")?;
    assert_eq!(field.field_type(), 0x1234);
    assert_eq!(field.bytes(), &input[..]);
    assert_eq!(field.bytes().as_ptr(), input.as_ptr());
    assert_eq!(field.body(), &input[4..]);
    assert_eq!(field.body().as_ptr(), input[4..].as_ptr());
    assert!(fields.next().is_none());
    assert!(matches!(
        frame_nts_plaintext(&input, limits(255, 1)?),
        Err(NtsPlaintextError::ByteLimit)
    ));
    Ok(())
}

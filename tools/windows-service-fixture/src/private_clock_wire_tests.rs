// SPDX-License-Identifier: GPL-2.0-only

use super::{FrameError, RequestId, decode_reply, decode_request, encode_reply, encode_request};

/// Asymmetric ID tests packing order in both directions.
const NUMBER: u64 = 0xefcd_ab89_6745_2301;
/// Independently specified version 1.0 request bytes.
const REQUEST: &[u8] = b"\x01\x00\x01\x00\x02\x00\x00\x00\x01\x23\x45\x67\x89\xab\xcd\xef{}";
/// Independently specified available answer for the original ID.
const TRUE_REPLY: &[u8] =
    b"\x01\x00\x02\x00\x12\x00\x00\x00\x01\x23\x45\x67\x89\xab\xcd\xef{\"available\":true}";
/// Independently specified unavailable answer for the original ID.
const FALSE_REPLY: &[u8] =
    b"\x01\x00\x02\x00\x13\x00\x00\x00\x01\x23\x45\x67\x89\xab\xcd\xef{\"available\":false}";

/// Decode independent literals and compare each canonical sender byte-for-byte.
///
/// # Errors
///
/// Fails if the known nonzero identifiers do not construct.
///
/// # Panics
///
/// Fails on changed literal bytes, direction, ordering, or correlation.
#[test]
fn literal_frames_have_exact_direction_layout_and_pairing() -> Result<(), FrameError> {
    let id: RequestId = RequestId::new(NUMBER)?;
    assert_eq!(decode_request(REQUEST), Ok(id));
    assert_eq!(encode_request(id).as_bytes(), REQUEST);
    assert_eq!(decode_reply(TRUE_REPLY, id), Ok(true));
    assert_eq!(encode_reply(id, true).as_bytes(), TRUE_REPLY);
    assert_eq!(decode_reply(FALSE_REPLY, id), Ok(false));
    assert_eq!(encode_reply(id, false).as_bytes(), FALSE_REPLY);
    assert_eq!(decode_request(TRUE_REPLY), Err(FrameError::Kind));
    assert_eq!(decode_reply(REQUEST, id), Err(FrameError::Kind));
    assert_eq!(
        decode_reply(TRUE_REPLY, RequestId::new(NUMBER + 1)?),
        Err(FrameError::Correlation)
    );
    assert_eq!(RequestId::new(0), Err(FrameError::RequestId));
    let largest: RequestId = RequestId::new(u64::MAX)?;
    assert!(largest > id);
    let max_wire: &[u8] = b"\x01\x00\x01\x00\x02\x00\x00\x00\xff\xff\xff\xff\xff\xff\xff\xff{}";
    assert_eq!(decode_request(max_wire), Ok(largest));
    Ok(())
}

/// Change one header field at a time while retaining the healthy body and ID.
///
/// # Panics
///
/// Fails if an altered header no longer produces its intended refusal.
#[test]
fn envelope_rejects_partial_version_reserved_length_and_zero_id() {
    for (offset, changed, error) in [
        (0, 2, FrameError::Version),
        (1, 1, FrameError::Version),
        (2, 3, FrameError::Kind),
        (3, 1, FrameError::Header),
        (4, 3, FrameError::Length),
        (5, 1, FrameError::Length),
        (6, 1, FrameError::Header),
        (7, 1, FrameError::Header),
    ] {
        let mut frame: [u8; 18] = [0; 18];
        frame.copy_from_slice(REQUEST);
        frame[offset] = changed;
        assert_eq!(decode_request(&frame), Err(error));
    }
    assert_eq!(decode_request(&REQUEST[..15]), Err(FrameError::Header));
    assert_eq!(decode_request(&REQUEST[..17]), Err(FrameError::Length));
    let mut zero_id: [u8; 18] = [0; 18];
    zero_id.copy_from_slice(REQUEST);
    zero_id[8..16].fill(0);
    assert_eq!(decode_request(&zero_id), Err(FrameError::RequestId));
    let mut extended: [u8; 19] = [0; 19];
    extended[..18].copy_from_slice(REQUEST);
    assert_eq!(decode_request(&extended), Err(FrameError::Length));
}

/// Independent fixture envelope with up to 65 bytes for the cap neighbor.
///
/// # Panics
///
/// Fails if a test supplies more than its fixed 65-byte fixture capacity.
fn with_body<T>(kind: u8, body: &[u8], observe: impl FnOnce(&[u8]) -> T) -> T {
    let mut frame: [u8; 81] = [0; 81];
    frame[..16]
        .copy_from_slice(b"\x01\x00\x00\x00\x00\x00\x00\x00\x01\x23\x45\x67\x89\xab\xcd\xef");
    frame[2] = kind;
    frame[4] = body.len().to_le_bytes()[0];
    let end: usize = 16 + body.len();
    frame[16..end].copy_from_slice(body);
    observe(&frame[..end])
}

/// Exercise the exact cap on otherwise valid request JSON and a one-byte neighbor.
///
/// # Errors
///
/// Fails if the known nonzero identifier does not construct.
///
/// # Panics
///
/// Fails if an otherwise valid request bypasses or falls short of the byte cap.
#[test]
fn frame_cap_precedes_json() -> Result<(), FrameError> {
    let mut body: [u8; 65] = [b' '; 65];
    body[..2].copy_from_slice(b"{}");
    let id: RequestId = RequestId::new(NUMBER)?;
    assert_eq!(with_body(1, &body[..64], decode_request), Ok(id));
    assert_eq!(with_body(1, &body, decode_request), Err(FrameError::Length));
    Ok(())
}

/// Challenge map shape and document completion using request bytes with valid envelopes.
///
/// # Errors
///
/// Fails if the known nonzero identifier does not construct.
///
/// # Panics
///
/// Fails if empty map admission or any rejection expectation changes.
#[test]
fn request_requires_exact_empty_object() -> Result<(), FrameError> {
    let id: RequestId = RequestId::new(NUMBER)?;
    assert_eq!(with_body(1, b" \r\n{ \t} \t", decode_request), Ok(id));
    let invalid: [&[u8]; 9] = [
        b"",
        b"[]",
        b"true",
        b"null",
        b"{",
        b"{}{}",
        b"{} 1",
        b"{} !",
        br#"{"role":"engine"}"#,
    ];
    for body in invalid {
        assert_eq!(with_body(1, body, decode_request), Err(FrameError::Body));
    }
    Ok(())
}

/// Refuse missing, duplicate, unknown and wrong-typed reply fields within the cap.
///
/// # Errors
///
/// Fails if the known nonzero identifier does not construct.
///
/// # Panics
///
/// Fails if any alternate valid spelling or malformed field changes disposition.
#[test]
fn reply_requires_one_boolean_map_field_and_complete_json() -> Result<(), FrameError> {
    let id: RequestId = RequestId::new(NUMBER)?;
    let observe = |bytes: &[u8]| decode_reply(bytes, id);
    let invalid: [&[u8]; 12] = [
        b"{}",
        b"[true]",
        b"true",
        br#"{"available":null}"#,
        br#"{"available":"true"}"#,
        br#"{"available":1}"#,
        br#"{"other":true}"#,
        br#"{"available":true,"other":false}"#,
        br#"{"available":true,"available":false}"#,
        br#"{"available":true,"av\u0061ilable":false}"#,
        br#"{"available":true}null"#,
        b"{\"available\":\xff}",
    ];
    for body in invalid {
        assert_eq!(with_body(2, body, observe), Err(FrameError::Body));
    }
    assert_eq!(
        with_body(2, br#"{"av\u0061ilable":true}"#, observe),
        Ok(true)
    );
    assert_eq!(
        with_body(2, b"\r\n{ \"available\" : false }\t ", observe),
        Ok(false)
    );
    Ok(())
}

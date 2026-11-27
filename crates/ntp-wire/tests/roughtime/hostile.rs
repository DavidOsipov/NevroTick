// SPDX-License-Identifier: GPL-2.0-only
//! Independent transport and projection assertions for untrusted wire bytes.

use ntp_wire::roughtime::{
    DecodeError, Request, Response, decode_request, decode_response, response_frame_length,
};

/// Check transport facts and immutable projections after arbitrary parser input.
///
/// # Panics
/// Panics on a transport-guard mismatch, lost frame custody or invalid projection.
pub(crate) fn check_untrusted_frame(input: &[u8]) {
    let request: Result<Request<'_>, DecodeError> = decode_request(input);
    let response: Result<Response<'_>, DecodeError> = decode_response(input);
    let header: Option<&[u8; 12]> = input.get(..12).and_then(|bytes| bytes.try_into().ok());
    let declared_size: Option<u64> = header.and_then(|header| {
        let [magic @ .., first, second, third, fourth]: [u8; 12] = *header;
        (magic == *b"ROUGHTIM")
            .then(|| u64::from(u32::from_le_bytes([first, second, third, fourth])) + 12)
    });
    let exact_body: bool =
        declared_size.is_some() && declared_size == u64::try_from(input.len()).ok();
    if input.len() > 4_096 {
        assert_eq!(request.err(), Some(DecodeError::FrameLimit));
        assert_eq!(response.err(), Some(DecodeError::FrameLimit));
    } else if !exact_body {
        assert_eq!(request.err(), Some(DecodeError::Frame));
        assert_eq!(response.err(), Some(DecodeError::Frame));
    } else {
        if let Ok(request) = request {
            assert!(core::ptr::eq(request.frame(), input));
        }
        if let Ok(response) = response {
            assert!(core::ptr::eq(response.frame(), input));
            let midpoint: u64 = response.midpoint_seconds();
            let radius: u64 = u64::from(response.radius_seconds());
            assert_ne!(radius, 0);
            assert_eq!(
                midpoint
                    .checked_sub(radius)
                    .zip(midpoint.checked_add(radius)),
                Some(response.bounds_seconds())
            );
            assert!(response.merkle_path().len() <= 32);
            assert!(u64::from(response.merkle_index()) < (1_u64 << response.merkle_path().len()));
        }
    }
    if let Some(header) = header {
        let expected: Result<usize, DecodeError> = match declared_size {
            None => Err(DecodeError::Frame),
            Some(total) if total > 4_096 => Err(DecodeError::FrameLimit),
            Some(total) => usize::try_from(total).map_err(|_| DecodeError::Frame),
        };
        assert_eq!(response_frame_length(header), expected);
    }
}

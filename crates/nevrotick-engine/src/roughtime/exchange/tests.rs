// SPDX-License-Identifier: GPL-2.0-only
//! Roughtime exchange controls for chained requests.

use super::{
    Capture, ExchangeError, ExchangeTransport, RECEIVE_BYTES, Receipt, chained_nonce, request,
};
use std::{net::SocketAddr, time::Instant};

/// Supply literal history for the pure nonce calculation without performing I/O.
fn observation(link: [u8; 32], body: [u8; 2], at: Instant) -> Receipt {
    let mut response: [u8; RECEIVE_BYTES] = [0; RECEIVE_BYTES];
    response[..2].copy_from_slice(&body);
    let capture: Capture = Capture {
        before: at,
        after: at,
    };
    Receipt {
        peer: SocketAddr::from(([127, 0, 0, 1], 1)),
        transport: ExchangeTransport::Udp,
        root_key: [0; 32],
        request: [0; request::REQUEST_BYTES],
        chaining_randomness: Some(link),
        response,
        length: 2,
        bounds: (0, 2),
        sent: capture,
        received: capture,
    }
}

/// Reject a nonadjacent repeated input and hash a fresh input with the actual predecessor.
///
/// # Errors
/// Returns an unexpected refusal of a fresh fixture link.
///
/// # Panics
/// Panics if the earlier linking value is reused or the fresh hash differs.
#[test]
fn refuses_nonadjacent_repeated_linking_randomness() -> Result<(), &'static str> {
    let elapsed: Instant = Instant::now();
    let earlier: [u8; 32] = [0x41; 32];
    let fresh: [u8; 32] = [0x42; 32];
    let history: [Receipt; 2] = [
        observation(earlier, *b"XX", elapsed),
        observation([0x43; 32], *b"RT", elapsed),
    ];
    assert!(matches!(
        chained_nonce(&history, &earlier),
        Err(ExchangeError::Entropy)
    ));
    let nonce: [u8; 32] =
        chained_nonce(&history, &fresh).map_err(|_| "fresh fixture draw refused")?;
    // Independent SHA-512 known answer for RT followed by 32 bytes of 0x42.
    assert_eq!(
        nonce,
        [
            0x8a, 0xab, 0x10, 0x82, 0x2a, 0xa1, 0xe0, 0xb5, 0x86, 0x34, 0xc2, 0x8b, 0xa4, 0x43,
            0x49, 0x7b, 0xb7, 0x3b, 0xa1, 0xe8, 0x1b, 0x9f, 0x31, 0x4d, 0x32, 0xad, 0x5e, 0x62,
            0x15, 0x77, 0xac, 0xc4,
        ]
    );
    Ok(())
}

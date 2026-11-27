// SPDX-License-Identifier: GPL-2.0-only
//! Complete RFC 10049 Appendix B and reported erratum 9238 controls.
//! Upstream binary fixtures carry the attribution and notice in README.md.

use nevrotick_engine::roughtime::{PendingRequest, RootKey, VerifiedResponse, VerifyError};
use ntp_wire::roughtime::{Request, decode_request};
use sha2::{Digest, Sha512, digest::Output};

/// The public root, complete request frame, and complete response frame.
type Exchange = (&'static [u8; 32], &'static [u8; 1036], &'static [u8; 416]);

/// Appendix B of RFC 10049, in its published order.
const PUBLISHED: [Exchange; 3] = [
    (
        include_bytes!("appendix-b-original-root.bin"),
        include_bytes!("appendix-b-original-request.bin"),
        include_bytes!("appendix-b-original-response.bin"),
    ),
    (
        include_bytes!("appendix-b-original-2-root.bin"),
        include_bytes!("appendix-b-original-2-request.bin"),
        include_bytes!("appendix-b-original-2-response.bin"),
    ),
    (
        include_bytes!("appendix-b-original-3-root.bin"),
        include_bytes!("appendix-b-original-3-request.bin"),
        include_bytes!("appendix-b-original-3-response.bin"),
    ),
];

/// The proposed replacement from the reported erratum, in its proposed order.
const PROPOSED: [Exchange; 3] = [
    (
        include_bytes!("erratum-9238-proposal-root.bin"),
        include_bytes!("erratum-9238-proposal-request.bin"),
        include_bytes!("erratum-9238-proposal-response.bin"),
    ),
    (
        include_bytes!("erratum-9238-proposal-2-root.bin"),
        include_bytes!("erratum-9238-proposal-2-request.bin"),
        include_bytes!("erratum-9238-proposal-2-response.bin"),
    ),
    (
        include_bytes!("erratum-9238-proposal-3-root.bin"),
        include_bytes!("erratum-9238-proposal-3-request.bin"),
        include_bytes!("erratum-9238-proposal-3-response.bin"),
    ),
];

/// The published `rand` bytes linking each proposed request to its predecessor.
const PROPOSED_RAND: [&[u8; 32]; 2] = [
    include_bytes!("erratum-9238-proposal-2-rand.bin"),
    include_bytes!("erratum-9238-proposal-3-rand.bin"),
];

/// Each published exchange reaches the final RFC profile's delegation signature check.
///
/// # Errors
/// Returns a root or request refusal if a fixture fails before response verification.
///
/// # Panics
/// Panics if any published response fails to produce the expected signature refusal.
#[test]
fn published_appendix_b_refuses_final_signature_context() -> Result<(), VerifyError> {
    for (position, (key, request, response)) in PUBLISHED.into_iter().enumerate() {
        let root: RootKey = RootKey::from_bytes(key)?;
        assert_eq!(
            PendingRequest::new(request, &root)?.verify(response).err(),
            Some(VerifyError::DelegationSignature),
            "published exchange at index {position}",
        );
    }
    Ok(())
}

/// Verify the proposed exchanges and independently check their nonce links and time order.
///
/// # Errors
/// Returns a root, request, or response verification refusal.
///
/// # Panics
/// Panics on a changed frame, root, authenticated interval, nonce link, or causal comparison.
#[test]
fn erratum_chain_preserves_authenticated_time_conflict() -> Result<(), VerifyError> {
    let mut bounds: [(u64, u64); 3] = [(0, 0); 3];
    for (slot, (key, request, response)) in bounds.iter_mut().zip(PROPOSED) {
        let root: RootKey = RootKey::from_bytes(key)?;
        let verified: VerifiedResponse<'_, '_> =
            PendingRequest::new(request, &root)?.verify(response)?;
        assert_eq!(verified.root_key_bytes(), key);
        assert!(core::ptr::eq(verified.request_frame(), request.as_slice()));
        assert!(core::ptr::eq(verified.frame(), response.as_slice()));
        *slot = verified.bounds_seconds();
    }
    assert_eq!(
        bounds,
        [
            (1_773_685_568, 1_773_685_574),
            (1_773_685_568, 1_773_685_574),
            (1_773_599_168, 1_773_599_174),
        ],
    );

    for (exchanges, rand) in PROPOSED.windows(2).zip(PROPOSED_RAND) {
        let request: Request<'_> = decode_request(exchanges[1].1).map_err(VerifyError::Request)?;
        let mut state: Sha512 = Sha512::new();
        state.update(exchanges[0].2);
        state.update(rand);
        let nonce_hash: Output<Sha512> = state.finalize();
        assert_eq!(&nonce_hash[..32], request.nonce());
    }

    let [first, second, third]: [(u64, u64); 3] = bounds;
    assert!(first.0 <= second.1);
    assert!(first.0 > third.1);
    assert!(second.0 > third.1);
    Ok(())
}

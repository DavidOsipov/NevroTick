// SPDX-License-Identifier: GPL-2.0-only
//! Independent historical proof controls from the reported RFC 10049 erratum.

use super::{
    Conflict, ConflictProofError, EXCHANGES, PEERS, ProofEntry, SelectedPeer, verify_prefix,
};
use crate::roughtime::VerifyError;
use ntp_wire::roughtime::{DecodeError, MAX_RESPONSE_BYTES};
use std::{array, net::SocketAddr, num::NonZeroU64};

/// The first two intervals overlap; both earlier lower bounds exceed the third upper bound.
const FIRST_CONFLICT: Conflict = Conflict {
    earlier: 0,
    later: 2,
};

/// First byte of SIG after the header, count, six offsets and seven tags in this fixture.
const RESPONSE_SIGNATURE_START: usize = 12 + 4 + 6 * 4 + 7 * 4;

/// Root, complete request and response, and optional preceding link of a published exchange.
type ReportedExchange = (
    &'static [u8; 32],
    &'static [u8; 1036],
    &'static [u8; 416],
    Option<&'static [u8; 32]>,
);

/// The complete proposed replacement in reported erratum 9238.
const PROPOSED: [ReportedExchange; PEERS] = [
    (
        include_bytes!("../../../../../tests/fixtures/roughtime/erratum-9238-proposal-root.bin"),
        include_bytes!("../../../../../tests/fixtures/roughtime/erratum-9238-proposal-request.bin"),
        include_bytes!(
            "../../../../../tests/fixtures/roughtime/erratum-9238-proposal-response.bin"
        ),
        None,
    ),
    (
        include_bytes!("../../../../../tests/fixtures/roughtime/erratum-9238-proposal-2-root.bin"),
        include_bytes!(
            "../../../../../tests/fixtures/roughtime/erratum-9238-proposal-2-request.bin"
        ),
        include_bytes!(
            "../../../../../tests/fixtures/roughtime/erratum-9238-proposal-2-response.bin"
        ),
        Some(include_bytes!(
            "../../../../../tests/fixtures/roughtime/erratum-9238-proposal-2-rand.bin"
        )),
    ),
    (
        include_bytes!("../../../../../tests/fixtures/roughtime/erratum-9238-proposal-3-root.bin"),
        include_bytes!(
            "../../../../../tests/fixtures/roughtime/erratum-9238-proposal-3-request.bin"
        ),
        include_bytes!(
            "../../../../../tests/fixtures/roughtime/erratum-9238-proposal-3-response.bin"
        ),
        Some(include_bytes!(
            "../../../../../tests/fixtures/roughtime/erratum-9238-proposal-3-rand.bin"
        )),
    ),
];

/// Attach distinct fixture endpoints and group declarations to the unchanged signed bytes.
fn reported_prefix() -> ([SelectedPeer; PEERS], [ProofEntry<'static>; PEERS]) {
    let addresses: [SocketAddr; PEERS] = [
        SocketAddr::from(([127, 0, 0, 1], 2002)),
        SocketAddr::from(([127, 0, 0, 2], 2002)),
        SocketAddr::from(([127, 0, 0, 3], 2002)),
    ];
    let groups: [NonZeroU64; PEERS] = [
        NonZeroU64::MIN,
        NonZeroU64::MIN.saturating_add(1),
        NonZeroU64::MIN.saturating_add(2),
    ];
    let selected: [SelectedPeer; PEERS] = array::from_fn(|index: usize| SelectedPeer {
        endpoint: addresses[index],
        root: *PROPOSED[index].0,
        operator: groups[index],
        clock: groups[index],
    });
    let prefix: [ProofEntry<'static>; PEERS] = array::from_fn(|index: usize| {
        let (root, request, response, random): ReportedExchange = PROPOSED[index];
        ProofEntry {
            peer: addresses[index],
            root,
            request,
            response,
            random,
        }
    });
    (selected, prefix)
}

/// Reproduce the first authenticated contradiction and reject a signed consistent neighbor.
///
/// # Panics
/// Panics if the proof receiver accepts a false contradiction or refuses the original.
#[test]
fn reported_prefix_proves_the_first_contradiction() {
    let (selected, prefix): ([SelectedPeer; PEERS], [ProofEntry<'_>; PEERS]) = reported_prefix();
    assert_eq!(
        verify_prefix(FIRST_CONFLICT, &selected, prefix.into_iter()),
        Ok(())
    );
    assert_eq!(
        verify_prefix(
            Conflict {
                earlier: 0,
                later: 1,
            },
            &selected,
            prefix[..2].iter().copied(),
        ),
        Err(ConflictProofError::Witness)
    );
    assert_eq!(
        verify_prefix(
            Conflict {
                earlier: 1,
                later: 2,
            },
            &selected,
            prefix.into_iter(),
        ),
        Err(ConflictProofError::Witness)
    );
}

/// Challenge only the count, claimed positions, endpoint, root, or roster order.
///
/// # Panics
/// Panics if incomplete, extra or substituted entries reach a valid proof.
#[test]
fn proof_rejects_wrong_shape_witness_and_roster() {
    let (selected, prefix): ([SelectedPeer; PEERS], [ProofEntry<'_>; PEERS]) = reported_prefix();
    assert_eq!(
        verify_prefix(FIRST_CONFLICT, &selected, prefix[..2].iter().copied()),
        Err(ConflictProofError::Incomplete)
    );
    assert_eq!(
        verify_prefix(
            FIRST_CONFLICT,
            &selected,
            [prefix[0]; EXCHANGES + 1].into_iter()
        ),
        Err(ConflictProofError::Shape)
    );
    assert_eq!(
        verify_prefix(
            Conflict {
                earlier: 2,
                later: 2
            },
            &selected,
            prefix.into_iter(),
        ),
        Err(ConflictProofError::Shape)
    );
    let [first, second, third]: [ProofEntry<'_>; PEERS] = prefix;
    let wrong_root: ProofEntry<'_> = ProofEntry {
        root: second.root,
        ..first
    };
    assert_eq!(
        verify_prefix(
            FIRST_CONFLICT,
            &selected,
            [wrong_root, second, third].into_iter()
        ),
        Err(ConflictProofError::Identity { position: 0 })
    );
    let wrong_endpoint: ProofEntry<'_> = ProofEntry {
        peer: third.peer,
        ..second
    };
    assert_eq!(
        verify_prefix(
            FIRST_CONFLICT,
            &selected,
            [first, wrong_endpoint, third].into_iter()
        ),
        Err(ConflictProofError::Identity { position: 1 })
    );
    assert_eq!(
        verify_prefix(
            FIRST_CONFLICT,
            &selected,
            [second, first, third].into_iter()
        ),
        Err(ConflictProofError::Identity { position: 0 })
    );
}

/// Challenge one link or one response at a time, retaining the original roster.
///
/// # Panics
/// Panics if a missing or altered nonce link or invalid frame can establish a proof.
#[test]
fn proof_rejects_corrupted_frames_and_nonce_links() {
    let (selected, prefix): ([SelectedPeer; PEERS], [ProofEntry<'_>; PEERS]) = reported_prefix();
    let [first, second, third]: [ProofEntry<'_>; PEERS] = prefix;
    let missing: ProofEntry<'_> = ProofEntry {
        random: None,
        ..second
    };
    assert_eq!(
        verify_prefix(
            FIRST_CONFLICT,
            &selected,
            [first, missing, third].into_iter()
        ),
        Err(ConflictProofError::Incomplete)
    );
    let unexpected: ProofEntry<'_> = ProofEntry {
        random: second.random,
        ..first
    };
    assert_eq!(
        verify_prefix(
            FIRST_CONFLICT,
            &selected,
            [unexpected, second, third].into_iter()
        ),
        Err(ConflictProofError::Link { position: 0 })
    );
    let altered: ProofEntry<'_> = ProofEntry {
        random: third.random,
        ..second
    };
    assert_eq!(
        verify_prefix(
            FIRST_CONFLICT,
            &selected,
            [first, altered, third].into_iter()
        ),
        Err(ConflictProofError::Link { position: 1 })
    );

    let mut signature: [u8; 416] = *PROPOSED[2].2;
    signature[RESPONSE_SIGNATURE_START] ^= 1;
    let oversized: [u8; MAX_RESPONSE_BYTES + 1] = [0; MAX_RESPONSE_BYTES + 1];
    let cases: [(usize, &[u8], VerifyError); 4] = [
        (0, second.response, VerifyError::Nonce),
        (
            2,
            &third.response[..third.response.len() - 1],
            VerifyError::Response(DecodeError::Frame),
        ),
        (
            2,
            &oversized,
            VerifyError::Response(DecodeError::FrameLimit),
        ),
        (2, &signature, VerifyError::ResponseSignature),
    ];
    for (position, response, cause) in cases {
        let mut changed: [ProofEntry<'_>; PEERS] = prefix;
        changed[position].response = response;
        assert_eq!(
            verify_prefix(FIRST_CONFLICT, &selected, changed.into_iter()),
            Err(ConflictProofError::Verification { position, cause })
        );
    }
}

// SPDX-License-Identifier: GPL-2.0-only
//! Historical proof controls from the reported RFC 10049 erratum and local signers.

use super::{
    Conflict, ConflictProof, ConflictProofError, EXCHANGES, PEERS, ProofEntry, SelectedPeer,
    verify_prefix,
};
use crate::roughtime::{
    PendingRequest, RootKey, VerifiedResponse, VerifyError,
    interval::IntervalPolicy,
    round::{
        ProbeProgress, RoundControl, RoundError, RoundPeer,
        recovery::{
            ConflictEvidence, ControlledRecovery, RecoveryControl, RecoveryDatePolicy,
            RecoveryRound,
        },
    },
    signed_reply,
};
use ed25519_dalek::SigningKey;
use ntp_wire::roughtime::{DecodeError, MAX_RESPONSE_BYTES};
use std::{
    array,
    error::Error,
    io,
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    num::{NonZeroU32, NonZeroU64},
    ops::Range,
    ptr, thread,
    time::{Duration, Instant},
};
use time_domain::{EraWindow, LeapDateScope, LeapKnowledge, bundled_leap_knowledge};

/// The first two intervals overlap; both earlier lower bounds exceed the third upper bound.
const FIRST_CONFLICT: Conflict = Conflict {
    earlier: 0,
    later: 2,
};

/// First byte of SIG after the header, count, six offsets and seven tags in this fixture.
const RESPONSE_SIGNATURE_START: usize = 12 + 4 + 6 * 4 + 7 * 4;

/// Complete 32-byte `SRV` value in the published request template.
const REQUEST_SERVER: Range<usize> = 56..88;
/// Complete 32-byte nonce value in the same template.
const REQUEST_NONCE: Range<usize> = 88..120;

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
/// # Errors
/// Returns fixture setup or an unexpected controlled exchange refusal.
///
/// # Panics
/// Panics if incomplete, extra or substituted entries reach a valid proof.
#[test]
fn proof_rejects_wrong_shape_witness_and_roster() -> Result<(), Box<dyn Error>> {
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
    check_public_witness()
}

/// Confront the public receiver with a genuine prefix and a different contradictory pair.
///
/// # Errors
/// Returns socket, fixture-admission or unexpected controlled exchange failures.
///
/// # Panics
/// Panics if a receipt changes its position or a later pair establishes the first witness.
fn check_public_witness() -> Result<(), Box<dyn Error>> {
    let mut evidence: ConflictEvidence = signed_recovery_evidence()?;
    let proof: ConflictProof<'_> = evidence
        .reconstruct()
        .map_err(|cause| format!("original witness: {cause:?}"))?;
    assert!(ptr::eq(proof.evidence(), ptr::from_ref(&evidence)));
    assert_eq!(proof.evidence().conflict(), FIRST_CONFLICT);
    // Both earlier intervals contradict the third. The earlier slot is the only changed field.
    evidence.conflict.earlier = 1;
    assert!(matches!(
        evidence.reconstruct(),
        Err(ConflictProofError::Witness)
    ));
    Ok(())
}

/// Retain genuine signed receipts through the recovery owner's first contradiction.
///
/// # Errors
/// Returns setup, transport, or unexpected recovery failures.
///
/// # Panics
/// Panics if any probe changes position or misses the expected disposition.
pub(in crate::roughtime::round::recovery) fn signed_recovery_evidence()
-> Result<ConflictEvidence, Box<dyn Error>> {
    /// Original five-second round and invocation cutoff budget.
    const BUDGET: Duration = Duration::from_secs(5);
    /// Independent fixture limit for receiving an already submitted datagram.
    const RECEIVE_WAIT: Duration = Duration::from_secs(1);
    /// With radius three, each of the first two lower endpoints exceeds the last upper endpoint.
    const MIDPOINTS: [u64; PEERS] = [1_020, 1_019, 1_009];

    let [first, second, third]: [Result<UdpSocket, io::Error>; PEERS] =
        [(); PEERS].map(|()| UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)));
    let sockets: [UdpSocket; PEERS] = [first?, second?, third?];
    let [first, second, third]: [Result<SocketAddr, io::Error>; PEERS] =
        sockets.each_ref().map(UdpSocket::local_addr);
    let endpoints: [SocketAddr; PEERS] = [first?, second?, third?];
    let anchors: [SigningKey; PEERS] =
        [1_u8, 2, 3].map(|seed: u8| SigningKey::from_bytes(&[seed; 32]));
    let delegated: SigningKey = SigningKey::from_bytes(&[4; 32]);
    let [first, second, third]: [Result<RootKey, String>; PEERS] =
        anchors.each_ref().map(|key: &SigningKey| {
            RootKey::from_bytes(key.verifying_key().as_bytes())
                .map_err(|cause: VerifyError| format!("proof fixture root: {cause:?}"))
        });
    let roots: [RootKey; PEERS] = [first?, second?, third?];
    let groups: [NonZeroU64; PEERS] =
        [0_u64, 1, 2].map(|offset: u64| NonZeroU64::MIN.saturating_add(offset));
    let peers: [RoundPeer<'_>; PEERS] = array::from_fn(|slot: usize| {
        RoundPeer::new(endpoints[slot], &roots[slot], groups[slot], groups[slot])
    });

    let rate: NonZeroU32 = NonZeroU32::new(1_000_000_000).ok_or("zero fixture rate")?;
    let interval: IntervalPolicy = IntervalPolicy::new([rate; 2], Duration::ZERO, BUDGET, BUDGET)
        .map_err(|cause| format!("proof fixture policy: {cause:?}"))?;
    let scope: LeapDateScope = LeapDateScope::new(EraWindow::new(
        3_692_217_600_i128 << 32,
        3_692_217_601_i128 << 32,
    )?)?;
    let calendar: LeapKnowledge<'static> = bundled_leap_knowledge()?;
    let invocation: RecoveryControl = RecoveryControl::new();
    let round_control: RoundControl = RoundControl::new();
    let mut recovery: ControlledRecovery<'_> = ControlledRecovery::new(
        &invocation,
        BUDGET,
        RecoveryDatePolicy::new(interval, scope, &calendar),
    )
    .map_err(|cause| format!("proof fixture recovery: {cause:?}"))?;
    {
        let mut round: RecoveryRound<'_> = recovery
            .begin(&round_control, peers, BUDGET, BUDGET)
            .map_err(|cause| format!("proof fixture round: {cause:?}"))?;
        for (position, midpoint) in MIDPOINTS.into_iter().enumerate() {
            let socket: &UdpSocket = &sockets[position];
            socket.set_read_timeout(Some(RECEIVE_WAIT))?;
            let sent: SocketAddr = round
                .start_next()
                .map_err(|cause| format!("proof fixture send {position}: {cause:?}"))?;
            assert_eq!(sent, endpoints[position]);
            let mut buffer: [u8; 1037] = [0; 1037];
            let (length, sender): (usize, SocketAddr) = socket.recv_from(&mut buffer)?;
            let frame: &[u8] = &buffer[..length];
            assert_eq!(length, 1036);
            let nonce: [u8; 32] = frame[REQUEST_NONCE].try_into()?;
            let response: [u8; 476] = signed_reply::sign_reply(
                &anchors[position],
                &delegated,
                frame,
                nonce,
                midpoint,
                3,
            )?;
            assert_eq!(socket.send_to(&response, sender)?, response.len());
            let started: Instant = Instant::now();
            let outcome: Result<ProbeProgress, RoundError> = loop {
                match round.poll() {
                    Ok(ProbeProgress::Waiting) if started.elapsed() < RECEIVE_WAIT => {
                        thread::yield_now();
                    }
                    Ok(ProbeProgress::Waiting) => {
                        return Err("proof fixture reply timed out".into());
                    }
                    result => break result,
                }
            };
            if position < PEERS - 1 {
                assert!(
                    matches!(outcome, Ok(ProbeProgress::Accepted)),
                    "{outcome:?}"
                );
            } else {
                assert!(
                    matches!(outcome, Err(RoundError::Conflict(FIRST_CONFLICT))),
                    "{outcome:?}"
                );
            }
        }
    }
    recovery
        .first_conflict
        .take()
        .ok_or_else(|| "missing conflict".into())
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

/// Re-sign the last exchange for each link so the duplicate itself causes refusal.
///
/// # Errors
/// Returns a fixture signing or unexpected standalone keyed-verification refusal.
/// # Panics
/// Panics if signed bounds, the distinct-link witness or the duplicate refusal differ.
#[test]
fn correctly_chained_proof_refuses_nonadjacent_repeated_randomness() -> Result<(), Box<dyn Error>> {
    /// Three distinct peers and one return to the first peer.
    const LENGTH: usize = 4;
    /// Positive signed radius in seconds for every fixture date.
    const RADIUS: u32 = 3;
    /// The first three positions are causally consistent. Position three revisits
    /// the first peer and contradicts position zero before any other pair.
    const DATES: [(u64, (u64, u64)); LENGTH] = [
        (1000, (997, 1003)),
        (1010, (1007, 1013)),
        (1020, (1017, 1023)),
        (900, (897, 903)),
    ];
    let seeds: [u8; PEERS] = [1, 2, 3];
    let anchors: [SigningKey; PEERS] = seeds.map(|seed: u8| SigningKey::from_bytes(&[seed; 32]));
    let delegated: [SigningKey; PEERS] =
        seeds.map(|seed: u8| SigningKey::from_bytes(&[seed + 16; 32]));
    let (mut selected, _): ([SelectedPeer; PEERS], [ProofEntry<'_>; PEERS]) = reported_prefix();
    for (peer, anchor) in selected.iter_mut().zip(&anchors) {
        peer.root = anchor.verifying_key().to_bytes();
    }
    let conflict: Conflict = Conflict {
        earlier: 0,
        later: 3,
    };
    let first_link: [u8; 32] = [41; 32];
    let middle_link: [u8; 32] = [42; 32];
    let new_link: [u8; 32] = [43; 32];
    let cases: [(&[u8; 32], Result<(), ConflictProofError>); 2] = [
        (&new_link, Ok(())),
        (&first_link, Err(ConflictProofError::Link { position: 3 })),
    ];
    for (last_link, expected) in cases {
        let links: [Option<&[u8; 32]>; LENGTH] =
            [None, Some(&first_link), Some(&middle_link), Some(last_link)];
        let mut requests: [[u8; 1036]; LENGTH] = [*PROPOSED[0].1; LENGTH];
        let mut responses: [[u8; 476]; LENGTH] = [[0; 476]; LENGTH];
        for (position, request) in requests.iter_mut().enumerate() {
            let slot: usize = position % PEERS;
            let root_bytes: &[u8; 32] = selected[slot].root_key_bytes();
            let commitment: [u8; 32] = signed_reply::digest(&[&[0xff], root_bytes]);
            let nonce: [u8; 32] = match links[position] {
                Some(random) => signed_reply::digest(&[&responses[position - 1], random]),
                None => [40; 32],
            };
            request[REQUEST_SERVER].copy_from_slice(&commitment);
            request[REQUEST_NONCE].copy_from_slice(&nonce);
            let (midpoint, bounds): (u64, (u64, u64)) = DATES[position];
            responses[position] = signed_reply::sign_reply(
                &anchors[slot],
                &delegated[slot],
                request,
                nonce,
                midpoint,
                RADIUS,
            )?;

            let root: RootKey = RootKey::from_bytes(root_bytes)
                .map_err(|cause: VerifyError| format!("fixture root {slot}: {cause:?}"))?;
            let verified: VerifiedResponse<'_, '_> = PendingRequest::new(request, &root)
                .and_then(|pending: PendingRequest<'_>| pending.verify(&responses[position]))
                .map_err(|cause: VerifyError| format!("fixture exchange {position}: {cause:?}"))?;
            assert_eq!(verified.bounds_seconds(), bounds);
        }
        let prefix: [ProofEntry<'_>; LENGTH] = array::from_fn(|position: usize| {
            let peer: &SelectedPeer = &selected[position % PEERS];
            ProofEntry {
                peer: peer.endpoint(),
                root: peer.root_key_bytes(),
                request: &requests[position],
                response: &responses[position],
                random: links[position],
            }
        });
        assert_eq!(
            verify_prefix(conflict, &selected, prefix.into_iter()),
            expected
        );
    }
    Ok(())
}

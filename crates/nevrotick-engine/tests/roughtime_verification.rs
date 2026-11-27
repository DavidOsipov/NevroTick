// SPDX-License-Identifier: GPL-2.0-only
//! Independent signed RFC 10049 controls for the actual verifier.
//!
//! The `appendix-b-original` files are the first vector in
//! [RFC 10049 Appendix B](https://www.rfc-editor.org/rfc/rfc10049.html#appendix-B).
//! The `erratum-9238-proposal` files are the first proposed replacement in
//! [RFC Editor erratum 9238](https://errata.rfc-editor.org/eid9238/).
//! These upstream vectors are attributed to the IETF. Copyright (c) 2026
//! IETF Trust and the persons identified as authors of the code.
//! Redistribution and use of the upstream vectors are governed by the
//! Revised BSD License in section 4.c of the
//! [IETF Trust Legal Provisions](https://trustee.ietf.org/license-info/).
//!
//! The `tree` files were produced with Python's SHA-512 and OpenSSL Ed25519,
//! using public fixture seeds 0..32 for the root, 32..64 for the delegated
//! key, and 64..96 for the foreign root. The signed delegation covers
//! 999 through 1000 seconds. Both valid replies claim midpoint 1000 and
//! radius 5 and carry opposite paths in one two-leaf tree.

#![cfg(feature = "roughtime-verification")]

use nevrotick_engine::roughtime::{PendingRequest, RootKey, VerifiedResponse, VerifyError};
use ntp_wire::roughtime::DecodeError;

/// Long-term key that signed the two-leaf tree's delegated key.
const TREE_ROOT: &[u8; 32] = include_bytes!("fixtures/roughtime/tree-root.bin");
/// Original request matching Merkle leaf zero.
const LEFT_REQUEST: &[u8; 1036] = include_bytes!("fixtures/roughtime/tree-left-request.bin");
/// Original request matching Merkle leaf one.
const RIGHT_REQUEST: &[u8] = include_bytes!("fixtures/roughtime/tree-right-request.bin");
/// Signed response with the right sibling and index zero.
const LEFT_RESPONSE: &[u8; 476] = include_bytes!("fixtures/roughtime/tree-left-response.bin");
/// Signed response with the left sibling and index one.
const RIGHT_RESPONSE: &[u8] = include_bytes!("fixtures/roughtime/tree-right-response.bin");

/// Accept the proposed lowercase-context vector and refuse the published context.
///
/// # Errors
/// Returns an error if either root or request is refused, or the corrected reply fails verification.
///
/// # Panics
/// Panics if the corrected interval or borrowed frames differ, or the original reply is refused for a different reason.
#[test]
fn appendix_b_contexts() -> Result<(), VerifyError> {
    let corrected_key: &[u8; 32] =
        include_bytes!("fixtures/roughtime/erratum-9238-proposal-root.bin");
    let corrected_request: &[u8] =
        include_bytes!("fixtures/roughtime/erratum-9238-proposal-request.bin");
    let corrected_response: &[u8] =
        include_bytes!("fixtures/roughtime/erratum-9238-proposal-response.bin");
    let corrected_root: RootKey = RootKey::from_bytes(corrected_key)?;
    let verified: VerifiedResponse<'_, '_> =
        PendingRequest::new(corrected_request, &corrected_root)?.verify(corrected_response)?;
    assert_eq!(verified.bounds_seconds(), (1_773_685_568, 1_773_685_574));
    assert_eq!(verified.root_key_bytes(), corrected_key);
    assert!(core::ptr::eq(verified.request_frame(), corrected_request));
    assert!(core::ptr::eq(verified.frame(), corrected_response));

    let original_key: &[u8; 32] = include_bytes!("fixtures/roughtime/appendix-b-original-root.bin");
    let original_request: &[u8] =
        include_bytes!("fixtures/roughtime/appendix-b-original-request.bin");
    let original_response: &[u8] =
        include_bytes!("fixtures/roughtime/appendix-b-original-response.bin");
    let original_root: RootKey = RootKey::from_bytes(original_key)?;
    assert_eq!(
        PendingRequest::new(original_request, &original_root)?
            .verify(original_response)
            .err(),
        Some(VerifyError::DelegationSignature),
    );
    Ok(())
}

/// Accept both sibling orders and preserve the complete registered frames.
///
/// # Errors
/// Returns an error if either independently signed exchange fails verification.
///
/// # Panics
/// Panics if either verified interval, root, or borrowed frame differs from the fixture.
#[test]
fn signed_tree_accepts_both_leaf_positions() -> Result<(), VerifyError> {
    let root: RootKey = RootKey::from_bytes(TREE_ROOT)?;
    let cases: [(&[u8], &[u8]); 2] = [
        (LEFT_REQUEST, LEFT_RESPONSE),
        (RIGHT_REQUEST, RIGHT_RESPONSE),
    ];
    for (request, response) in cases {
        let verified: VerifiedResponse<'_, '_> =
            PendingRequest::new(request, &root)?.verify(response)?;
        assert_eq!(verified.bounds_seconds(), (995, 1005));
        assert_eq!(verified.root_key_bytes(), TREE_ROOT);
        assert!(core::ptr::eq(verified.request_frame(), request));
        assert!(core::ptr::eq(verified.frame(), response));
    }
    Ok(())
}

/// Check the advertised server and the actual root signature independently.
///
/// # Errors
/// Returns an error if the foreign root or the request that names it is refused.
///
/// # Panics
/// Panics if either exchange fails to produce its distinct commitment or signature refusal.
#[test]
fn different_root_cannot_verify_the_delegation() -> Result<(), VerifyError> {
    let foreign_bytes: &[u8; 32] = include_bytes!("fixtures/roughtime/tree-foreign-root.bin");
    let foreign_root: RootKey = RootKey::from_bytes(foreign_bytes)?;
    assert_eq!(
        PendingRequest::new(LEFT_REQUEST, &foreign_root).err(),
        Some(VerifyError::ServerCommitment),
    );

    // The selected foreign key and request SRV match. The response echoes the
    // unchanged nonce but the delegation is still signed by the original root.
    let foreign_request: &[u8] = include_bytes!("fixtures/roughtime/tree-foreign-request.bin");
    assert_eq!(
        PendingRequest::new(foreign_request, &foreign_root)?
            .verify(LEFT_RESPONSE)
            .err(),
        Some(VerifyError::DelegationSignature),
    );
    Ok(())
}

/// The unsigned echo must match the exact pending request.
///
/// # Errors
/// Returns an error if the root or either original request is refused.
///
/// # Panics
/// Panics if the altered echo or the replay does not produce a nonce refusal.
#[test]
fn nonce_mismatch_rejects_altered_echo_and_replay() -> Result<(), VerifyError> {
    let root: RootKey = RootKey::from_bytes(TREE_ROOT)?;
    let mut altered_echo: [u8; 476] = *LEFT_RESPONSE;
    altered_echo[132] ^= 1;
    assert_eq!(
        PendingRequest::new(LEFT_REQUEST, &root)?
            .verify(&altered_echo)
            .err(),
        Some(VerifyError::Nonce),
    );
    assert_eq!(
        PendingRequest::new(RIGHT_REQUEST, &root)?
            .verify(LEFT_RESPONSE)
            .err(),
        Some(VerifyError::Nonce),
    );
    Ok(())
}

/// Bind the unknown request tag and valid zero padding to the Merkle leaf.
///
/// # Errors
/// Returns an error if the root or either altered request is refused during request decoding or binding.
///
/// # Panics
/// Panics if either changed frame fails to produce a request-commitment refusal.
#[test]
fn same_nonce_with_changed_request_bytes_has_no_inclusion() -> Result<(), VerifyError> {
    let root: RootKey = RootKey::from_bytes(TREE_ROOT)?;
    let mut changed_unknown_tag: [u8; 1036] = *LEFT_REQUEST;
    changed_unknown_tag[132] ^= 1; // First byte of the independent SEEN value.
    let extended_padding: &[u8] =
        include_bytes!("fixtures/roughtime/tree-extended-padding-request.bin");
    let changed_requests: [&[u8]; 2] = [&changed_unknown_tag, extended_padding];
    for request in changed_requests {
        assert_eq!(
            PendingRequest::new(request, &root)?
                .verify(LEFT_RESPONSE)
                .err(),
            Some(VerifyError::RequestCommitment),
        );
    }
    Ok(())
}

/// Validate both Merkle path bytes and the direction of each sibling.
///
/// # Errors
/// Returns an error if the root or original request is refused.
///
/// # Panics
/// Panics if a changed sibling, direction, or unused index bit produces a different result than expected.
#[test]
fn wrong_merkle_sibling_or_index_is_refused() -> Result<(), VerifyError> {
    // Independent fixture layout: PATH begins at 168 and INDX at 472.
    const PATH_OFFSET: usize = 168;
    const INDEX_OFFSET: usize = 472;
    let root: RootKey = RootKey::from_bytes(TREE_ROOT)?;
    let mut wrong_sibling: [u8; 476] = *LEFT_RESPONSE;
    wrong_sibling[PATH_OFFSET] ^= 1;
    let mut wrong_direction: [u8; 476] = *LEFT_RESPONSE;
    wrong_direction[INDEX_OFFSET] = 1;
    let mut unused_index_bit: [u8; 476] = *LEFT_RESPONSE;
    unused_index_bit[INDEX_OFFSET] = 2;
    let failures: [(&str, &[u8], VerifyError); 3] = [
        ("sibling", &wrong_sibling, VerifyError::RequestCommitment),
        (
            "direction",
            &wrong_direction,
            VerifyError::RequestCommitment,
        ),
        (
            "unused bit",
            &unused_index_bit,
            VerifyError::Response(DecodeError::MerkleIndex),
        ),
    ];
    for (case, response, error) in failures {
        assert_eq!(
            PendingRequest::new(LEFT_REQUEST, &root)?
                .verify(response)
                .err(),
            Some(error),
            "Merkle {case}",
        );
    }
    Ok(())
}

/// Include opaque nested fields in both signatures and demand the v1 context.
///
/// # Errors
/// Returns an error if the root or original request is refused.
///
/// # Panics
/// Panics if a changed signed field or obsolete context fails to produce its signature refusal.
#[test]
fn signatures_cover_exact_nested_messages_and_context() -> Result<(), VerifyError> {
    let root: RootKey = RootKey::from_bytes(TREE_ROOT)?;
    let mut changed_delegation: [u8; 476] = *LEFT_RESPONSE;
    changed_delegation[420] ^= 1; // First byte of the independent DELE NOTE value.
    let mut changed_response: [u8; 476] = *LEFT_RESPONSE;
    changed_response[252] ^= 1; // First byte of the independent SREP META value.
    let old_context: &[u8] = include_bytes!("fixtures/roughtime/tree-old-response-context.bin");
    let failures: [(&[u8], VerifyError); 3] = [
        (&changed_delegation, VerifyError::DelegationSignature),
        (&changed_response, VerifyError::ResponseSignature),
        (old_context, VerifyError::ResponseSignature),
    ];
    for (response, error) in failures {
        assert_eq!(
            PendingRequest::new(LEFT_REQUEST, &root)?
                .verify(response)
                .err(),
            Some(error),
        );
    }
    Ok(())
}

/// A valid signature does not legalize claims outside the selected wire profile.
///
/// # Errors
/// Returns an error if the root or original request is refused.
///
/// # Panics
/// Panics if the invalid version list, radius, or delegation span fails to produce its wire refusal.
#[test]
fn signed_but_invalid_fields_are_refused() -> Result<(), VerifyError> {
    let root: RootKey = RootKey::from_bytes(TREE_ROOT)?;
    let failures: [(&[u8], DecodeError); 3] = [
        (
            include_bytes!("fixtures/roughtime/tree-invalid-versions.bin"),
            DecodeError::Version,
        ),
        (
            include_bytes!("fixtures/roughtime/tree-zero-radius.bin"),
            DecodeError::Radius,
        ),
        (
            include_bytes!("fixtures/roughtime/tree-outside-delegation.bin"),
            DecodeError::Delegation,
        ),
    ];
    for (response, error) in failures {
        assert_eq!(
            PendingRequest::new(LEFT_REQUEST, &root)?
                .verify(response)
                .err(),
            Some(VerifyError::Response(error)),
        );
    }
    Ok(())
}

/// Refuse a small-order key both as a trust anchor and in a signed delegation.
///
/// # Errors
/// Returns an error if the valid root or original request is refused.
///
/// # Panics
/// Panics if the small-order root or delegated key fails to produce its key refusal.
#[test]
fn small_order_keys_are_refused() -> Result<(), VerifyError> {
    let mut identity: [u8; 32] = [0; 32];
    identity[0] = 1;
    assert_eq!(
        RootKey::from_bytes(&identity).err(),
        Some(VerifyError::RootKey),
    );

    let root: RootKey = RootKey::from_bytes(TREE_ROOT)?;
    let response: &[u8] = include_bytes!("fixtures/roughtime/tree-weak-delegation.bin");
    assert_eq!(
        PendingRequest::new(LEFT_REQUEST, &root)?
            .verify(response)
            .err(),
        Some(VerifyError::DelegatedKey),
    );
    Ok(())
}

/// Require the canonical representation of an otherwise usable Edwards point.
///
/// # Panics
/// Panics if the canonical key is refused or the unreduced encoding fails to produce a root-key refusal.
#[test]
fn root_key_rejects_noncanonical_encoding_of_usable_point() {
    // Both encodings describe y = 3 if the decoder reduces modulo 2^255 - 19.
    let mut canonical: [u8; 32] = [0; 32];
    canonical[0] = 3;
    let mut unreduced: [u8; 32] = [0xff; 32];
    unreduced[0] = 0xf0;
    unreduced[31] = 0x7f;
    assert!(RootKey::from_bytes(&canonical).is_ok());
    assert_eq!(
        RootKey::from_bytes(&unreduced).err(),
        Some(VerifyError::RootKey),
    );
}

/// Controlled UDP peer and end-to-end receipt checks for the actual exchange.
mod exchange {
    use std::error::Error;
    use std::io::{self, ErrorKind};
    use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
    use std::thread;
    use std::time::{Duration, Instant};

    use ed25519_dalek::{Signature, Signer, SigningKey};
    use nevrotick_engine::roughtime::exchange::{
        ExchangeError, PendingExchange, Receipt, RoundAuthority, StagedReceipt, Withdrawal,
        retry::RetryBinding,
    };
    use sha2::{Digest, Sha512};

    use super::{LEFT_REQUEST, LEFT_RESPONSE, RootKey, TREE_ROOT, VerifyError};

    /// Maximum wait for a single native UDP loopback exchange in this fixture.
    const BUDGET: Duration = Duration::from_secs(2);
    /// The public test seed that produced the certified delegated key.
    const DELEGATED_SEED: [u8; 32] = *b" !\"#$%&'()*+,-./0123456789:;<=>?";
    /// Exact zero-terminated RFC v1 response domain separator.
    const RESPONSE_CONTEXT: &[u8] = b"Roughtime v1 response signature\0";

    /// Failures in fixture plumbing carry diagnostics independently of exchange refusals.
    pub(super) type TestResult<T> = Result<T, Box<dyn Error>>;

    /// Preserve the fixture's unexpected exchange error in the failure report.
    ///
    /// # Errors
    /// Returns the original error's bounded diagnostic.
    pub(super) fn checked<T>(result: Result<T, ExchangeError>) -> TestResult<T> {
        result.map_err(|error| format!("unexpected Roughtime exchange result: {error:?}").into())
    }

    /// Read one little-endian field from the fixture's bounded tag table.
    fn word(bytes: &[u8], offset: usize) -> Option<u32> {
        let word: [u8; 4] = bytes.get(offset..offset.checked_add(4)?)?.try_into().ok()?;
        Some(u32::from_le_bytes(word))
    }

    /// Locate only `NONC` in the observed request, independently of the product decoder.
    pub(super) fn observed_nonce(frame: &[u8]) -> Option<[u8; 32]> {
        let message: &[u8] = frame.get(12..)?;
        let count: usize = usize::try_from(word(message, 0)?).ok()?;
        let tags_start: usize = count.checked_mul(4)?;
        let values_start: usize = count.checked_mul(8)?;
        let tags: &[u8] = message.get(tags_start..values_start)?;
        let values: &[u8] = message.get(values_start..)?;
        let nonce: usize = tags
            .as_chunks::<4>()
            .0
            .iter()
            .position(|tag| tag == b"NONC")?;
        let start: usize = if nonce == 0 {
            0
        } else {
            usize::try_from(word(message, nonce * 4)?).ok()?
        };
        let next: usize = nonce + 1;
        let end: usize = if next == count {
            values.len()
        } else {
            usize::try_from(word(message, next * 4)?).ok()?
        };
        values.get(start..end)?.try_into().ok()
    }

    /// Adapt the independent fixture's signed response to the request actually sent.
    ///
    /// # Errors
    /// Refuses a request without the expected nonce field.
    pub(super) fn signed_reply(request: &[u8]) -> TestResult<[u8; 476]> {
        let nonce: [u8; 32] =
            observed_nonce(request).ok_or("could not read the sent request nonce")?;
        let mut response: [u8; 476] = *LEFT_RESPONSE;
        response[132..164].copy_from_slice(&nonce);

        let mut leaf: Sha512 = Sha512::new();
        leaf.update([0]);
        leaf.update(request);
        let leaf: [u8; 64] = leaf.finalize().into();
        let mut root: Sha512 = Sha512::new();
        root.update([1]);
        root.update(&leaf[..32]);
        root.update(&response[168..200]);
        let root: [u8; 64] = root.finalize().into();
        response[276..308].copy_from_slice(&root[..32]);

        let mut payload: [u8; RESPONSE_CONTEXT.len() + 108] = [0; RESPONSE_CONTEXT.len() + 108];
        let (context, value): (&mut [u8], &mut [u8]) = payload.split_at_mut(RESPONSE_CONTEXT.len());
        context.copy_from_slice(RESPONSE_CONTEXT);
        value.copy_from_slice(&response[200..308]);
        let delegated: SigningKey = SigningKey::from_bytes(&DELEGATED_SEED);
        let signature: Signature = delegated.sign(&payload);
        response[68..132].copy_from_slice(&signature.to_bytes());
        Ok(response)
    }

    /// Open exactly one nonblocking IPv4 loopback peer at an ephemeral port.
    ///
    /// # Errors
    /// Returns the original socket setup or address error.
    pub(super) fn peer() -> io::Result<(UdpSocket, SocketAddr)> {
        let socket: UdpSocket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0))?;
        socket.set_nonblocking(true)?;
        let address: SocketAddr = socket.local_addr()?;
        Ok((socket, address))
    }

    /// Hear the actual request and recover only the observed sender endpoint.
    ///
    /// # Errors
    /// Returns the original socket failure or the bounded fixture timeout.
    pub(super) fn receive_request(peer: &UdpSocket) -> io::Result<(Vec<u8>, SocketAddr)> {
        let started: Instant = Instant::now();
        let mut packet: [u8; 4097] = [0; 4097];
        while started.elapsed() < BUDGET {
            match peer.recv_from(&mut packet) {
                Ok((length, sender)) => return Ok((packet[..length].to_vec(), sender)),
                Err(error) if error.kind() == ErrorKind::WouldBlock => thread::yield_now(),
                Err(error) => return Err(error),
            }
        }
        Err(io::Error::new(
            ErrorKind::TimedOut,
            "Roughtime request not observed",
        ))
    }

    /// Return a real receive disposition, refusing a fixture timeout separately.
    ///
    /// # Errors
    /// Returns a fixture timeout if the actual exchange never yields a disposition.
    fn receive<'a>(
        pending: &mut PendingExchange<'a>,
    ) -> TestResult<Result<StagedReceipt<'a>, ExchangeError>> {
        let started: Instant = Instant::now();
        while started.elapsed() < BUDGET {
            match pending.try_receive() {
                Ok(Some(staged)) => return Ok(Ok(staged)),
                Ok(None) => thread::yield_now(),
                Err(error) => return Ok(Err(error)),
            }
        }
        Err("Roughtime response did not produce a disposition".into())
    }

    /// Accept exactly one loopback response and refuse its replay for a new request.
    ///
    /// # Errors
    /// Returns fixture socket, signing, verification, or unexpected exchange errors.
    ///
    /// # Panics
    /// Panics if a receipt changes the original exchange or if either attempt can be reused.
    #[test]
    fn publishes_original_receipt_once_and_refuses_old_datagram() -> TestResult<()> {
        assert_eq!(signed_reply(LEFT_REQUEST)?, *LEFT_RESPONSE);
        let root: RootKey = RootKey::from_bytes(TREE_ROOT).map_err(|error| format!("{error:?}"))?;
        let authority: RoundAuthority = RoundAuthority::new();
        let (peer, address): (UdpSocket, SocketAddr) = peer()?;
        let mut original: PendingExchange<'_> =
            checked(PendingExchange::send(&authority, address, &root, BUDGET))?;
        let (request, sender): (Vec<u8>, SocketAddr) = receive_request(&peer)?;
        let reply: [u8; 476] = signed_reply(&request)?;
        assert_eq!(peer.send_to(&reply, sender)?, reply.len());
        let staged: StagedReceipt<'_> = checked(receive(&mut original)?)?;
        assert!(matches!(original.try_receive(), Err(ExchangeError::Spent)));
        let receipt: Receipt = checked(staged.publish())?;
        assert_eq!(receipt.request_frame(), request.as_slice());
        assert_eq!(receipt.response_frame(), reply.as_slice());
        assert_eq!(receipt.bounds_seconds(), (995, 1005));
        assert_eq!(receipt.peer(), address);
        assert_eq!(receipt.root_key_bytes(), TREE_ROOT);
        let ((send_before, send_after), (read_before, read_after)): (
            (Instant, Instant),
            (Instant, Instant),
        ) = receipt.captures();
        assert!(
            send_before <= send_after && send_after <= read_before && read_before <= read_after
        );
        assert!(matches!(
            PendingExchange::send(&authority, address, &root, BUDGET),
            Err(ExchangeError::Spent)
        ));

        let next_authority: RoundAuthority = RoundAuthority::new();
        let mut successor: PendingExchange<'_> = checked(PendingExchange::send(
            &next_authority,
            address,
            &root,
            BUDGET,
        ))?;
        let (new_request, new_sender): (Vec<u8>, SocketAddr) = receive_request(&peer)?;
        let new_nonce: [u8; 32] =
            observed_nonce(&new_request).ok_or("missing new request nonce")?;
        assert_ne!(Some(new_nonce), observed_nonce(&request));
        assert_eq!(peer.send_to(&reply, new_sender)?, reply.len());
        assert!(matches!(
            receive(&mut successor)?,
            Err(ExchangeError::Verification(VerifyError::Nonce))
        ));
        assert!(matches!(successor.try_receive(), Err(ExchangeError::Spent)));
        Ok(())
    }

    /// Refuse an already verified loopback response when any owner withdraws before publication.
    ///
    /// # Errors
    /// Returns fixture socket, signing, verification, or unexpected exchange errors.
    ///
    /// # Panics
    /// Panics if any withdrawn authority permits publication or loses its cause.
    #[test]
    fn staged_receipt_checks_each_withdrawal_before_publication() -> TestResult<()> {
        let root: RootKey = RootKey::from_bytes(TREE_ROOT).map_err(|error| format!("{error:?}"))?;
        let (peer, address): (UdpSocket, SocketAddr) = peer()?;
        let withdrawals: [Withdrawal; 6] = [
            Withdrawal::Round,
            Withdrawal::Catalogue,
            Withdrawal::Key,
            Withdrawal::Policy,
            Withdrawal::ClockFrame,
            Withdrawal::Stop,
        ];
        for cause in withdrawals {
            let authority: RoundAuthority = RoundAuthority::new();
            let mut pending: PendingExchange<'_> =
                checked(PendingExchange::send(&authority, address, &root, BUDGET))?;
            let (request, sender): (Vec<u8>, SocketAddr) = receive_request(&peer)?;
            let reply: [u8; 476] = signed_reply(&request)?;
            assert_eq!(peer.send_to(&reply, sender)?, reply.len());
            let staged: StagedReceipt<'_> = checked(receive(&mut pending)?)?;
            checked(authority.withdraw(cause))?;
            assert!(
                matches!(staged.publish(), Err(ExchangeError::Withdrawn(actual)) if actual == cause)
            );
        }
        Ok(())
    }

    /// Keep a prior withdrawal from transmitting and refuse an authentic reply from a foreign peer.
    ///
    /// # Errors
    /// Returns fixture socket, signing, verification, or unexpected exchange errors.
    ///
    /// # Panics
    /// Panics if a withdrawn attempt transmits or a different source port can satisfy the selection.
    #[test]
    fn withdrawal_prevents_send_and_foreign_peer_spends_receive() -> TestResult<()> {
        let root: RootKey = RootKey::from_bytes(TREE_ROOT).map_err(|error| format!("{error:?}"))?;
        let (selected, address): (UdpSocket, SocketAddr) = peer()?;
        let stopped: RoundAuthority = RoundAuthority::new();
        checked(stopped.withdraw(Withdrawal::Stop))?;
        assert!(matches!(
            PendingExchange::send(&stopped, address, &root, BUDGET),
            Err(ExchangeError::Withdrawn(Withdrawal::Stop))
        ));
        let mut unexpected: [u8; 1] = [0; 1];
        assert!(
            matches!(selected.recv_from(&mut unexpected), Err(error) if error.kind() == ErrorKind::WouldBlock)
        );

        let live: RoundAuthority = RoundAuthority::new();
        let (foreign, foreign_address): (UdpSocket, SocketAddr) = peer()?;
        assert_ne!(foreign_address, address);
        let mut pending: PendingExchange<'_> =
            checked(PendingExchange::send(&live, address, &root, BUDGET))?;
        let (request, sender): (Vec<u8>, SocketAddr) = receive_request(&selected)?;
        let reply: [u8; 476] = signed_reply(&request)?;
        assert_eq!(foreign.send_to(&reply, sender)?, reply.len());
        assert!(matches!(
            receive(&mut pending)?,
            Err(ExchangeError::ForeignPeer)
        ));
        assert!(matches!(pending.try_receive(), Err(ExchangeError::Spent)));
        Ok(())
    }

    /// Complete a sent retry attempt with an intentionally short wire response.
    ///
    /// # Errors
    /// Returns fixture socket or unexpected exchange failures.
    ///
    /// # Panics
    /// Panics if the selected peer cannot submit its reply or decoding accepts it.
    pub(super) fn malformed_retry(
        binding: &mut RetryBinding<'_>,
        selected: &UdpSocket,
        address: SocketAddr,
    ) -> TestResult<()> {
        let authority: RoundAuthority = RoundAuthority::new();
        let mut pending: PendingExchange<'_> = checked(PendingExchange::send_with_retry(
            &authority, binding, address, BUDGET,
        ))?;
        let (_request, sender): (Vec<u8>, SocketAddr) = receive_request(selected)?;
        let malformed: [u8; 4] = [0; 4];
        assert_eq!(selected.send_to(&malformed, sender)?, malformed.len());
        assert!(matches!(
            receive(&mut pending)?,
            Err(ExchangeError::Verification(VerifyError::Response(_)))
        ));
        Ok(())
    }

    /// Verify an actual request-bound reply and retain the original publication guard.
    ///
    /// # Errors
    /// Returns fixture socket, signing, or unexpected exchange failures.
    ///
    /// # Panics
    /// Panics if the selected peer cannot submit the complete reply.
    fn stage_retry<'a>(
        authority: &'a RoundAuthority,
        binding: &'a mut RetryBinding<'_>,
        selected: &UdpSocket,
        address: SocketAddr,
    ) -> TestResult<StagedReceipt<'a>> {
        let mut pending: PendingExchange<'_> = checked(PendingExchange::send_with_retry(
            authority, binding, address, BUDGET,
        ))?;
        let (request, sender): (Vec<u8>, SocketAddr) = receive_request(selected)?;
        let response: [u8; 476] = signed_reply(&request)?;
        assert_eq!(selected.send_to(&response, sender)?, response.len());
        checked(receive(&mut pending)?)
    }

    /// Read the actual refusal and verify the attempted route remains quiet.
    ///
    /// # Errors
    /// Returns an unexpected typed refusal or an unrepresentable fixture bound.
    ///
    /// # Panics
    /// Panics if the observed floor falls outside the independently timed window
    /// or retry admission transmits from a new authority before that floor.
    pub(super) fn refused_retry_floor(
        binding: &mut RetryBinding<'_>,
        selected: &UdpSocket,
        address: SocketAddr,
        terminal: (Instant, Instant),
        wait: Duration,
    ) -> TestResult<Instant> {
        let authority: RoundAuthority = RoundAuthority::new();
        let until: Instant =
            match PendingExchange::send_with_retry(&authority, binding, address, BUDGET) {
                Err(ExchangeError::RetryDeferred { until }) => until,
                Err(error) => return Err(format!("unexpected retry refusal: {error:?}").into()),
                Ok(_) => return Err("new authority advanced before the binding's floor".into()),
            };
        assert_retry_wait(until, terminal, wait)?;
        let mut unexpected: [u8; 1] = [0; 1];
        assert!(
            matches!(selected.recv_from(&mut unexpected), Err(error) if error.kind() == ErrorKind::WouldBlock)
        );
        Ok(until)
    }

    /// Compare the returned retry floor with an independently observed terminal bracket.
    ///
    /// # Errors
    /// Returns an unrepresentable fixture bound.
    ///
    /// # Panics
    /// Panics if the returned floor does not correspond to the expected wait.
    pub(super) fn assert_retry_wait(
        until: Instant,
        terminal: (Instant, Instant),
        wait: Duration,
    ) -> TestResult<()> {
        let earliest: Instant = terminal
            .0
            .checked_add(wait)
            .ok_or("invalid first floor bound")?;
        let latest: Instant = terminal
            .1
            .checked_add(wait)
            .ok_or("invalid last floor bound")?;
        assert!((earliest..=latest).contains(&until));
        Ok(())
    }

    /// Observe owner-local admission, signed-but-withdrawn settlement, and a real reset.
    ///
    /// # Errors
    /// Returns a fixture, signing, socket, or unexpected retry result.
    ///
    /// # Panics
    /// Panics if the same binding changes its floor across route or authority changes,
    /// if a noncurrent reply resets it, or if a current publication leaves failures charged.
    #[test]
    fn retry_floor_follows_aliases_until_current_signed_publication() -> TestResult<()> {
        let root: RootKey = RootKey::from_bytes(TREE_ROOT).map_err(|error| format!("{error:?}"))?;
        let (primary, primary_address): (UdpSocket, SocketAddr) = peer()?;
        let (alias, alias_address): (UdpSocket, SocketAddr) = peer()?;
        let mut binding: RetryBinding<'_> =
            checked(RetryBinding::new(&root, [primary_address, alias_address]))?;
        let (other, other_address): (UdpSocket, SocketAddr) = peer()?;
        let (_other_alias, other_alias_address): (UdpSocket, SocketAddr) = peer()?;
        let other_key: [u8; 32] = SigningKey::from_bytes(&[0xfa; 32])
            .verifying_key()
            .to_bytes();
        assert_ne!(&other_key, TREE_ROOT);
        let other_root: RootKey =
            RootKey::from_bytes(&other_key).map_err(|error| format!("{error:?}"))?;
        let mut other_binding: RetryBinding<'_> = checked(RetryBinding::new(
            &other_root,
            [other_address, other_alias_address],
        ))?;

        let first_start: Instant = Instant::now();
        malformed_retry(&mut binding, &primary, primary_address)?;
        let first_terminal: (Instant, Instant) = (first_start, Instant::now());
        let first_until: Instant = refused_retry_floor(
            &mut binding,
            &alias,
            alias_address,
            first_terminal,
            Duration::from_secs(1),
        )?;
        let other_authority: RoundAuthority = RoundAuthority::new();
        let other_pending: PendingExchange<'_> = checked(PendingExchange::send_with_retry(
            &other_authority,
            &mut other_binding,
            other_address,
            BUDGET,
        ))?;
        let (other_request, _): (Vec<u8>, SocketAddr) = receive_request(&other)?;
        assert!(observed_nonce(&other_request).is_some());
        drop(other_pending);
        assert_eq!(
            refused_retry_floor(
                &mut binding,
                &primary,
                primary_address,
                first_terminal,
                Duration::from_secs(1),
            )?,
            first_until
        );

        thread::sleep(first_until.saturating_duration_since(Instant::now()));
        let withdrawn: RoundAuthority = RoundAuthority::new();
        let stale: StagedReceipt<'_> =
            stage_retry(&withdrawn, &mut binding, &alias, alias_address)?;
        checked(withdrawn.withdraw(Withdrawal::Policy))?;
        let second_start: Instant = Instant::now();
        assert!(matches!(
            stale.publish(),
            Err(ExchangeError::Withdrawn(Withdrawal::Policy))
        ));
        let second_terminal: (Instant, Instant) = (second_start, Instant::now());
        let second_until: Instant = refused_retry_floor(
            &mut binding,
            &primary,
            primary_address,
            second_terminal,
            Duration::from_millis(1_500),
        )?;

        thread::sleep(second_until.saturating_duration_since(Instant::now()));
        let current: RoundAuthority = RoundAuthority::new();
        let staged: StagedReceipt<'_> =
            stage_retry(&current, &mut binding, &primary, primary_address)?;
        let accepted: Receipt = checked(staged.publish())?;
        assert_eq!(accepted.peer(), primary_address);
        assert_eq!(accepted.root_key_bytes(), TREE_ROOT);

        // The next failed exchange must start a new one-second sequence.
        // An immediate send alone could pass against the already elapsed floor.
        let reset_start: Instant = Instant::now();
        malformed_retry(&mut binding, &alias, alias_address)?;
        let reset_terminal: (Instant, Instant) = (reset_start, Instant::now());
        let _reset_until: Instant = refused_retry_floor(
            &mut binding,
            &primary,
            primary_address,
            reset_terminal,
            Duration::from_secs(1),
        )?;
        Ok(())
    }

    /// Settle the last empty native read and let a valid last-slot answer through.
    ///
    /// # Errors
    /// Returns fixture socket, signing, verification, or unexpected exchange errors.
    ///
    /// # Panics
    /// Panics if the empty attempt stays live or the last-slot reply is refused.
    #[test]
    fn last_read_preserves_budget_and_valid_neighbor() -> TestResult<()> {
        let root: RootKey = RootKey::from_bytes(TREE_ROOT).map_err(|error| format!("{error:?}"))?;
        let (selected, address): (UdpSocket, SocketAddr) = peer()?;
        for deliver in [false, true] {
            let authority: RoundAuthority = RoundAuthority::new();
            let mut pending: PendingExchange<'_> =
                checked(PendingExchange::send(&authority, address, &root, BUDGET))?;
            let (request, sender): (Vec<u8>, SocketAddr) = receive_request(&selected)?;
            for _ in 0..63 {
                assert!(checked(pending.try_receive())?.is_none());
            }
            if deliver {
                let reply: [u8; 476] = signed_reply(&request)?;
                assert_eq!(selected.send_to(&reply, sender)?, reply.len());
                let staged: StagedReceipt<'_> = checked(receive(&mut pending)?)?;
                let receipt: Receipt = checked(staged.publish())?;
                assert_eq!(receipt.response_frame(), reply.as_slice());
            } else {
                assert!(matches!(
                    pending.try_receive(),
                    Err(ExchangeError::ReceiveBudget)
                ));
            }
            assert!(matches!(pending.try_receive(), Err(ExchangeError::Spent)));
        }
        Ok(())
    }
}

/// Controlled signed TCP peer tests for the complete stream exchange.
mod tcp_exchange {
    use std::io::{self, ErrorKind, Read, Write};
    use std::net::{Ipv4Addr, Shutdown, SocketAddr, TcpListener, TcpStream, UdpSocket};
    use std::thread;
    use std::time::{Duration, Instant};

    use nevrotick_engine::roughtime::exchange::{
        ExchangeError, ExchangeTransport, Receipt, RoundAuthority, StagedReceipt, Withdrawal,
        retry::RetryBinding, tcp::PendingTcpExchange,
    };

    use super::exchange::{
        TestResult, assert_retry_wait, checked, malformed_retry, peer, refused_retry_floor,
        signed_reply,
    };
    use super::{LEFT_REQUEST, RootKey, TREE_ROOT};

    /// Limit both the local stream fixture and the original inquiry's lifetime.
    const BUDGET: Duration = Duration::from_secs(2);
    /// Limit observation calls independently of the product's poll accounting.
    const OBSERVATIONS: u8 = 32;

    /// Establish exactly one controlled client and server socket pair.
    ///
    /// # Errors
    /// Returns the original bounded connection or fixture socket setup error.
    fn sockets() -> io::Result<(TcpStream, TcpStream, SocketAddr)> {
        let listener: TcpListener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;
        let address: SocketAddr = listener.local_addr()?;
        let client: TcpStream = TcpStream::connect_timeout(&address, BUDGET)?;
        listener.set_nonblocking(true)?;
        let peer: TcpStream = accept_peer(&listener)?;
        Ok((client, peer, address))
    }

    /// Take one already submitted fixture connection and bound its subsequent I/O.
    ///
    /// # Errors
    /// Returns the original accept or socket option error.
    pub(super) fn accept_peer(listener: &TcpListener) -> io::Result<TcpStream> {
        let (peer, _): (TcpStream, SocketAddr) = listener.accept()?;
        peer.set_nodelay(true)?;
        peer.set_read_timeout(Some(BUDGET))?;
        peer.set_write_timeout(Some(BUDGET))?;
        Ok(peer)
    }

    /// Bind a fixture UDP alias at a different address from the selected TCP listener.
    ///
    /// # Errors
    /// Returns the original UDP fixture setup or address error.
    fn distinct_udp_peer(tcp_address: SocketAddr) -> io::Result<(UdpSocket, SocketAddr)> {
        let (socket, address): (UdpSocket, SocketAddr) = peer()?;
        if address == tcp_address {
            // Hold the first UDP socket while the OS assigns another UDP port.
            return peer();
        }
        Ok((socket, address))
    }

    /// Drive submission and read the original framed request at the other endpoint.
    ///
    /// # Errors
    /// Returns a fixture timeout or original socket or exchange failure.
    ///
    /// # Panics
    /// Panics if the product publishes before the fixture sends any response.
    fn submit(
        pending: &mut PendingTcpExchange<'_>,
        peer: &mut TcpStream,
    ) -> TestResult<[u8; LEFT_REQUEST.len()]> {
        submit_request(peer, || {
            assert!(checked(pending.poll())?.is_none());
            Ok(())
        })
    }

    /// Observe one bounded request on the stream while driving its original owner.
    ///
    /// # Errors
    /// Returns a socket error, advancement refusal, incomplete request or observation timeout.
    pub(super) fn submit_request(
        peer: &mut TcpStream,
        mut advance: impl FnMut() -> TestResult<()>,
    ) -> TestResult<[u8; LEFT_REQUEST.len()]> {
        let mut request: [u8; LEFT_REQUEST.len()] = [0; LEFT_REQUEST.len()];
        let mut received: usize = 0;
        peer.set_nonblocking(true)?;
        for _ in 0..OBSERVATIONS {
            advance()?;
            match peer.read(&mut request[received..]) {
                Ok(0) => return Err("TCP client closed before sending the original request".into()),
                Ok(length) => received += length,
                Err(error) if error.kind() == ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(1));
                }
                Err(error) => return Err(error.into()),
            }
            if received == request.len() {
                peer.set_nonblocking(false)?;
                return Ok(request);
            }
        }
        Err("complete original TCP request not observed".into())
    }

    /// Wait within the selected fixture calls for a terminal disposition.
    ///
    /// # Errors
    /// Returns a distinct fixture failure if the actual exchange never settles.
    fn receive<'a>(
        pending: &mut PendingTcpExchange<'a>,
    ) -> TestResult<Result<StagedReceipt<'a>, ExchangeError>> {
        for _ in 0..OBSERVATIONS {
            match pending.poll() {
                Ok(None) => thread::sleep(Duration::from_millis(1)),
                Ok(Some(staged)) => return Ok(Ok(staged)),
                Err(error) => return Ok(Err(error)),
            }
        }
        Err("TCP response did not produce a disposition".into())
    }

    /// Observe retirement on the actual other endpoint while the pending owner lives.
    ///
    /// # Errors
    /// Returns the original bounded socket failure.
    ///
    /// # Panics
    /// Panics if the connection sends further bytes or lacks a clean close.
    pub(super) fn assert_retired(peer: &mut TcpStream) -> io::Result<()> {
        let mut tail: [u8; 1] = [0; 1];
        assert_eq!(peer.read(&mut tail)?, 0);
        Ok(())
    }

    /// Observe an actual connection and retire its stream before receipt publication.
    ///
    /// # Errors
    /// Returns a fixture socket, signing, or unexpected exchange error.
    fn stage_connected_retry<'a>(
        authority: &'a RoundAuthority,
        binding: &'a mut RetryBinding<'_>,
        listener: &TcpListener,
        address: SocketAddr,
    ) -> TestResult<StagedReceipt<'a>> {
        let mut pending: PendingTcpExchange<'_> = checked(PendingTcpExchange::connect_with_retry(
            authority, binding, address, BUDGET,
        ))?;
        let mut server: TcpStream = accept_peer(listener)?;
        let request: [u8; LEFT_REQUEST.len()] = submit(&mut pending, &mut server)?;
        let reply: [u8; 476] = signed_reply(&request)?;
        server.write_all(&reply)?;
        let staged: StagedReceipt<'_> = checked(receive(&mut pending)?)?;
        assert_retired(&mut server)?;
        Ok(staged)
    }

    /// Carry the same binding from a UDP failure to TCP and back to UDP.
    ///
    /// # Errors
    /// Returns a fixture socket, signing, or unexpected exchange result.
    ///
    /// # Panics
    /// Panics if either early transport emits work, a noncurrent signed frame
    /// resets the floor, or a current TCP publication leaves the failures charged.
    #[test]
    fn retry_floor_follows_udp_and_tcp_until_signed_publication() -> TestResult<()> {
        let root: RootKey = RootKey::from_bytes(TREE_ROOT).map_err(|error| format!("{error:?}"))?;
        let listener: TcpListener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;
        let tcp_address: SocketAddr = listener.local_addr()?;
        let (udp, udp_address): (UdpSocket, SocketAddr) = distinct_udp_peer(tcp_address)?;
        listener.set_nonblocking(true)?;
        let mut binding: RetryBinding<'_> =
            checked(RetryBinding::new(&root, [udp_address, tcp_address]))?;

        let first_start: Instant = Instant::now();
        malformed_retry(&mut binding, &udp, udp_address)?;
        let first_terminal: (Instant, Instant) = (first_start, Instant::now());
        let early: RoundAuthority = RoundAuthority::new();
        let tcp_until: Instant =
            match PendingTcpExchange::connect_with_retry(&early, &mut binding, tcp_address, BUDGET)
            {
                Err(ExchangeError::RetryDeferred { until }) => until,
                Err(error) => {
                    return Err(format!("unexpected early TCP retry result: {error:?}").into());
                }
                Ok(_) => return Err("TCP connected before the shared retry floor".into()),
            };
        assert_retry_wait(tcp_until, first_terminal, Duration::from_secs(1))?;
        assert!(
            listener
                .accept()
                .is_err_and(|error| error.kind() == ErrorKind::WouldBlock)
        );

        thread::sleep(tcp_until.saturating_duration_since(Instant::now()));
        let denied: RoundAuthority = RoundAuthority::new();
        checked(denied.withdraw(Withdrawal::Policy))?;
        assert!(matches!(
            PendingTcpExchange::connect_with_retry(&denied, &mut binding, tcp_address, BUDGET),
            Err(ExchangeError::Withdrawn(Withdrawal::Policy))
        ));
        assert!(
            listener
                .accept()
                .is_err_and(|error| error.kind() == ErrorKind::WouldBlock)
        );

        let withdrawn: RoundAuthority = RoundAuthority::new();
        let stale: StagedReceipt<'_> =
            stage_connected_retry(&withdrawn, &mut binding, &listener, tcp_address)?;
        checked(withdrawn.withdraw(Withdrawal::Policy))?;
        let second_start: Instant = Instant::now();
        assert!(matches!(
            stale.publish(),
            Err(ExchangeError::Withdrawn(Withdrawal::Policy))
        ));
        let second_terminal: (Instant, Instant) = (second_start, Instant::now());
        let udp_until: Instant = refused_retry_floor(
            &mut binding,
            &udp,
            udp_address,
            second_terminal,
            Duration::from_millis(1_500),
        )?;

        thread::sleep(udp_until.saturating_duration_since(Instant::now()));
        let current: RoundAuthority = RoundAuthority::new();
        let staged: StagedReceipt<'_> =
            stage_connected_retry(&current, &mut binding, &listener, tcp_address)?;
        let accepted: Receipt = checked(staged.publish())?;
        assert_eq!(accepted.peer(), tcp_address);
        assert_eq!(accepted.root_key_bytes(), TREE_ROOT);
        assert_eq!(accepted.transport(), ExchangeTransport::Tcp);

        let reset_start: Instant = Instant::now();
        malformed_retry(&mut binding, &udp, udp_address)?;
        let reset_terminal: (Instant, Instant) = (reset_start, Instant::now());
        let _reset_until: Instant = refused_retry_floor(
            &mut binding,
            &udp,
            udp_address,
            reset_terminal,
            Duration::from_secs(1),
        )?;
        Ok(())
    }

    /// Observe a real loopback connect refusal and the resulting UDP floor.
    ///
    /// # Errors
    /// Returns a socket fixture or unexpected retry result.
    ///
    /// # Panics
    /// Panics if the submitted connect did not report the expected native refusal.
    #[test]
    fn refused_native_connect_charges_the_shared_retry_floor() -> TestResult<()> {
        let root: RootKey = RootKey::from_bytes(TREE_ROOT).map_err(|error| format!("{error:?}"))?;
        let closed: TcpListener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;
        let tcp_address: SocketAddr = closed.local_addr()?;
        let (udp, udp_address): (UdpSocket, SocketAddr) = distinct_udp_peer(tcp_address)?;
        let mut binding: RetryBinding<'_> =
            checked(RetryBinding::new(&root, [udp_address, tcp_address]))?;
        let authority: RoundAuthority = RoundAuthority::new();
        // Release only the selected TCP listener immediately before the owned connect.
        drop(closed);
        let started: Instant = Instant::now();
        assert!(matches!(
            PendingTcpExchange::connect_with_retry(&authority, &mut binding, tcp_address, BUDGET),
            Err(ExchangeError::Socket(cause)) if cause.kind() == ErrorKind::ConnectionRefused,
        ));
        let terminal: (Instant, Instant) = (started, Instant::now());
        let _until: Instant = refused_retry_floor(
            &mut binding,
            &udp,
            udp_address,
            terminal,
            Duration::from_secs(1),
        )?;
        Ok(())
    }

    /// Preserve complete signed bytes across both header and body fragments.
    ///
    /// # Errors
    /// Returns an original fixture, socket, verification, or publication failure.
    ///
    /// # Panics
    /// Panics if an incomplete fragment publishes, or receipt custody differs.
    #[test]
    fn publishes_fragmented_signed_frame_and_retires_stream() -> TestResult<()> {
        let root: RootKey = RootKey::from_bytes(TREE_ROOT).map_err(|error| format!("{error:?}"))?;
        let authority: RoundAuthority = RoundAuthority::new();
        let (client, mut peer, address): (TcpStream, TcpStream, SocketAddr) = sockets()?;
        let mut pending: PendingTcpExchange<'_> = checked(PendingTcpExchange::start(
            &authority, client, address, &root, BUDGET,
        ))?;
        let request: [u8; LEFT_REQUEST.len()] = submit(&mut pending, &mut peer)?;
        let reply: [u8; 476] = signed_reply(&request)?;

        peer.write_all(&reply[..7])?;
        assert!(checked(pending.poll())?.is_none());
        peer.write_all(&reply[7..24])?;
        assert!(checked(pending.poll())?.is_none());
        for byte in &reply[24..96] {
            peer.write_all(std::slice::from_ref(byte))?;
            assert!(checked(pending.poll())?.is_none());
        }
        let final_fragment_before: Instant = Instant::now();
        peer.write_all(&reply[96..])?;

        let staged: StagedReceipt<'_> = checked(receive(&mut pending)?)?;
        assert_retired(&mut peer)?;
        assert!(matches!(pending.poll(), Err(ExchangeError::Spent)));
        let receipt: Receipt = checked(staged.publish())?;
        assert_eq!(receipt.request_frame(), request.as_slice());
        assert_eq!(receipt.response_frame(), reply.as_slice());
        assert_eq!(receipt.bounds_seconds(), (995, 1005));
        assert_eq!(receipt.root_key_bytes(), TREE_ROOT);
        assert_eq!(receipt.peer(), address);
        assert_eq!(receipt.transport(), ExchangeTransport::Tcp);
        let ((send_before, send_after), (read_before, read_after)): (
            (Instant, Instant),
            (Instant, Instant),
        ) = receipt.captures();
        assert!(
            send_before <= send_after && send_after <= read_before && read_before <= read_after
        );
        assert!(final_fragment_before <= read_after);
        Ok(())
    }

    /// Require refusal from a length prefix alone and from EOF mid-body.
    ///
    /// # Errors
    /// Returns the original fixture, socket, signing, or exchange failure.
    ///
    /// # Panics
    /// Panics if invalid framing advances the exchange or leaves its socket active.
    #[test]
    fn refuses_oversized_and_incomplete_frames() -> TestResult<()> {
        let root: RootKey = RootKey::from_bytes(TREE_ROOT).map_err(|error| format!("{error:?}"))?;
        let authority: RoundAuthority = RoundAuthority::new();
        let (client, mut peer, address): (TcpStream, TcpStream, SocketAddr) = sockets()?;
        let mut pending: PendingTcpExchange<'_> = checked(PendingTcpExchange::start(
            &authority, client, address, &root, BUDGET,
        ))?;
        let _request: [u8; LEFT_REQUEST.len()] = submit(&mut pending, &mut peer)?;
        let mut oversized: [u8; 12] = *b"ROUGHTIM\0\0\0\0";
        oversized[8..].copy_from_slice(&4096_u32.to_le_bytes());
        // The fixture leaves its write side open so the header alone must settle the exchange.
        peer.write_all(&oversized)?;
        assert!(matches!(
            receive(&mut pending)?,
            Err(ExchangeError::Truncated)
        ));
        assert_retired(&mut peer)?;

        let authority: RoundAuthority = RoundAuthority::new();
        let (client, mut peer, address): (TcpStream, TcpStream, SocketAddr) = sockets()?;
        let mut pending: PendingTcpExchange<'_> = checked(PendingTcpExchange::start(
            &authority, client, address, &root, BUDGET,
        ))?;
        let request: [u8; LEFT_REQUEST.len()] = submit(&mut pending, &mut peer)?;
        let reply: [u8; 476] = signed_reply(&request)?;
        peer.write_all(&reply[..31])?;
        peer.shutdown(Shutdown::Write)?;
        assert!(matches!(
            receive(&mut pending)?,
            Err(ExchangeError::IncompleteFrame)
        ));
        assert_retired(&mut peer)?;
        Ok(())
    }

    /// Refuse publication when the owner withdraws after full verification.
    ///
    /// # Errors
    /// Returns the original fixture, socket, verification, or withdrawal failure.
    ///
    /// # Panics
    /// Panics if a verified receipt can publish after the authority changes.
    #[test]
    fn withdrawal_refuses_staged_signed_frame() -> TestResult<()> {
        let root: RootKey = RootKey::from_bytes(TREE_ROOT).map_err(|error| format!("{error:?}"))?;
        let authority: RoundAuthority = RoundAuthority::new();
        let (client, mut peer, address): (TcpStream, TcpStream, SocketAddr) = sockets()?;
        let mut pending: PendingTcpExchange<'_> = checked(PendingTcpExchange::start(
            &authority, client, address, &root, BUDGET,
        ))?;
        let request: [u8; LEFT_REQUEST.len()] = submit(&mut pending, &mut peer)?;
        let reply: [u8; 476] = signed_reply(&request)?;
        peer.write_all(&reply)?;
        let staged: StagedReceipt<'_> = checked(receive(&mut pending)?)?;
        assert_retired(&mut peer)?;

        checked(authority.withdraw(Withdrawal::ClockFrame))?;
        assert!(matches!(
            staged.publish(),
            Err(ExchangeError::Withdrawn(Withdrawal::ClockFrame))
        ));
        Ok(())
    }
}

/// Independent signed peers for the Engine's ordered Roughtime round.
mod ordered_round {
    use std::future::{Future, ready};
    use std::io::{ErrorKind, Write};
    use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream, UdpSocket};
    use std::num::{NonZeroU32, NonZeroU64};
    use std::pin::Pin;
    use std::thread;
    use std::time::{Duration, Instant};

    use ed25519_dalek::{Signature, Signer, SigningKey};
    use nevrotick_engine::roughtime::exchange::{
        ExchangeError, ExchangeTransport, PendingExchange, Receipt, RoundAuthority, Withdrawal,
        retry::RetryBinding,
    };
    use nevrotick_engine::roughtime::interval::{
        ControlledAgreement, ControlledDate, IntervalError, IntervalPolicy,
    };
    use nevrotick_engine::roughtime::round::recovery::{
        ConflictEvidence, ControlledRecovery, RecoveryControl, RecoveryDatePolicy, RecoveryRound,
        SelectedPeer,
    };
    use nevrotick_engine::roughtime::round::{
        OrderedRound, ProbeProgress, RetryRoundPeer, RoundControl, RoundError, RoundPeer,
        RoundStatus,
    };
    use sha2::{Digest, Sha512};
    use time_domain::{
        EraWindow, LeapDataIdentity, LeapDateScope, LeapEventKnowledge, LeapHistoryBefore,
        LeapInterpretation, LeapKnowledge, LeapOffsetKnowledge, bundled_leap_knowledge,
    };

    use tokio::runtime::{Builder, Runtime};
    use tokio::time::{Instant as TokioInstant, error::Elapsed};

    use super::exchange::{
        TestResult, assert_retry_wait, checked as checked_exchange, malformed_retry,
        observed_nonce, peer, receive_request, refused_retry_floor,
    };
    use super::tcp_exchange::{accept_peer, assert_retired, submit_request};
    use super::{LEFT_REQUEST, LEFT_RESPONSE, RootKey, VerifyError};

    /// Total duration for nine actual loopback probes.
    const ROUND_BUDGET: Duration = Duration::from_secs(6);
    /// Conservative original send-to-read budget per probe.
    const PROBE_BUDGET: Duration = Duration::from_secs(2);
    /// Original UDP child deadline, strictly inside the outer probe and round bounds.
    const UDP_WAIT: Duration = Duration::from_millis(500);
    /// First failed native attempt's independent retry wait in this fixture.
    const FIRST_RETRY_WAIT: Duration = Duration::from_secs(1);
    /// 2017-01-01 00:01:40 UTC, safely beyond the New Year leap boundary.
    const DATE_UNIX_SECONDS: u64 = 1_483_228_900;
    /// Literal NTP epoch coordinate for the same known date.
    const DATE_NTP_SECONDS: i128 = 3_692_217_700;
    /// Fractional ticks in one NTP second.
    const Q32_SECOND: i128 = 1_i128 << 32;
    /// Nanoseconds in one second, for the independent rational comparison.
    const NANOSECOND_DIVISOR: i128 = 1_000_000_000;
    /// Certificate domain separator with its RFC terminator.
    const DELEGATION_CONTEXT: &[u8] = b"Roughtime v1 delegation signature\0";
    /// Response domain separator with its RFC terminator.
    const RESPONSE_CONTEXT: &[u8] = b"Roughtime v1 response signature\0";

    /// One fixture operator, with independent root and delegated test keys.
    struct Server {
        /// Selected peer socket, bound on IPv4 loopback.
        socket: UdpSocket,
        /// Actual numeric address of the selected socket.
        address: SocketAddr,
        /// Long-term test key used to sign the delegation.
        trust_anchor: SigningKey,
        /// Separate test key used to sign the response.
        response_key: SigningKey,
        /// Checked public trust anchor handed to the Engine.
        root: RootKey,
    }

    impl Server {
        /// Bind a loopback fixture issuer with independent public test seeds.
        ///
        /// # Errors
        /// Returns fixture socket setup or root validation failure.
        fn bind(root_seed: u8, response_seed: u8) -> TestResult<Self> {
            let (socket, address): (UdpSocket, SocketAddr) = peer()?;
            let trust_anchor: SigningKey = SigningKey::from_bytes(&[root_seed; 32]);
            let response_key: SigningKey = SigningKey::from_bytes(&[response_seed; 32]);
            let root: RootKey = RootKey::from_bytes(trust_anchor.verifying_key().as_bytes())
                .map_err(|error| format!("invalid fixture root: {error:?}"))?;
            Ok(Self {
                socket,
                address,
                trust_anchor,
                response_key,
                root,
            })
        }

        /// Send a signed reply to the source of the exact observed request.
        ///
        /// # Errors
        /// Returns socket, missing nonce, or short native send failure.
        fn respond(&self, midpoint: u64) -> TestResult<(Vec<u8>, [u8; 476])> {
            self.respond_with_radius(midpoint, 5)
        }

        /// Sign the exact received request with the fixture's controlled midpoint and radius.
        ///
        /// # Errors
        /// Returns socket, missing nonce, or short native send failure.
        fn respond_with_radius(
            &self,
            midpoint: u64,
            radius: u32,
        ) -> TestResult<(Vec<u8>, [u8; 476])> {
            let (request, sender): (Vec<u8>, SocketAddr) = receive_request(&self.socket)?;
            let response: [u8; 476] = self.signed_response(&request, midpoint, radius)?;
            if self.socket.send_to(&response, sender)? != response.len() {
                return Err("short fixture response send".into());
            }
            Ok((request, response))
        }

        /// Sign the exact UDP or TCP request received by this issuer.
        ///
        /// # Errors
        /// Refuses a request without its nonce or an unrepresentable delegation range.
        fn signed_response(
            &self,
            request: &[u8],
            midpoint: u64,
            radius: u32,
        ) -> TestResult<[u8; 476]> {
            let nonce: [u8; 32] = observed_nonce(request).ok_or("missing fixture nonce")?;
            let mut response: [u8; 476] = *LEFT_RESPONSE;
            response[132..164].copy_from_slice(&nonce);
            response[256..260].copy_from_slice(&radius.to_le_bytes());
            response[260..268].copy_from_slice(&midpoint.to_le_bytes());
            let leaf: [u8; 32] = digest(&[&[0], request]);
            let merkle_root: [u8; 32] = digest(&[&[1], &leaf, &response[168..200]]);
            response[276..308].copy_from_slice(&merkle_root);

            let earliest: u64 = midpoint
                .checked_sub(100)
                .ok_or("fixture delegation starts too early")?;
            let latest: u64 = midpoint
                .checked_add(100)
                .ok_or("fixture delegation ends too late")?;
            response[424..456].copy_from_slice(self.response_key.verifying_key().as_bytes());
            response[456..464].copy_from_slice(&earliest.to_le_bytes());
            response[464..472].copy_from_slice(&latest.to_le_bytes());
            let delegation: Signature = self
                .trust_anchor
                .sign(&[DELEGATION_CONTEXT, &response[388..472]].concat());
            response[324..388].copy_from_slice(&delegation.to_bytes());
            let signature: Signature = self
                .response_key
                .sign(&[RESPONSE_CONTEXT, &response[200..308]].concat());
            response[68..132].copy_from_slice(&signature.to_bytes());
            Ok(response)
        }
    }

    /// Independently hash the unmodified transmitted or received bytes.
    fn digest(parts: &[&[u8]]) -> [u8; 32] {
        let mut state: Sha512 = Sha512::new();
        for part in parts {
            state.update(part);
        }
        let bytes: [u8; 64] = state.finalize().into();
        let mut truncated: [u8; 32] = [0; 32];
        truncated.copy_from_slice(&bytes[..32]);
        truncated
    }

    /// Build all three distinct fixture issuers with bounded socket effects.
    ///
    /// # Errors
    /// Returns the first fixture setup failure.
    fn operators() -> TestResult<[Server; 3]> {
        Ok([
            Server::bind(11, 61)?,
            Server::bind(22, 62)?,
            Server::bind(33, 63)?,
        ])
    }

    /// Give each selected fixture issuer distinct declared group labels.
    ///
    /// # Errors
    /// Refuses invalid fixture labels.
    fn roster(servers: &[Server; 3]) -> TestResult<[RoundPeer<'_>; 3]> {
        let primary: NonZeroU64 = NonZeroU64::MIN;
        let secondary: NonZeroU64 = NonZeroU64::new(2).ok_or("zero fixture group")?;
        let tertiary: NonZeroU64 = NonZeroU64::new(3).ok_or("zero fixture group")?;
        Ok([
            RoundPeer::new(servers[0].address, &servers[0].root, primary, primary),
            RoundPeer::new(servers[1].address, &servers[1].root, secondary, secondary),
            RoundPeer::new(servers[2].address, &servers[2].root, tertiary, tertiary),
        ])
    }

    /// Preserve the actual unexpected round refusal in the test result.
    ///
    /// # Errors
    /// Returns the bounded diagnostic for the original refusal.
    fn checked<T>(result: Result<T, RoundError>) -> TestResult<T> {
        result.map_err(|error| format!("unexpected Roughtime round result: {error:?}").into())
    }

    /// Observe the actual round receipt or terminal datagram refusal.
    ///
    /// # Errors
    /// Returns a separate fixture timeout if no settlement is observed.
    fn poll_to_settlement(
        round: &mut OrderedRound<'_>,
    ) -> TestResult<Result<ProbeProgress, RoundError>> {
        let started: Instant = Instant::now();
        while started.elapsed() < PROBE_BUDGET {
            match round.poll() {
                Ok(ProbeProgress::Waiting) => thread::yield_now(),
                result => return Ok(result),
            }
        }
        Err("Roughtime round did not settle its reply".into())
    }

    /// Drive one actual signed three-traversal round with independently supplied claims.
    ///
    /// # Errors
    /// Returns the first fixture or unexpected round failure.
    ///
    /// # Panics
    /// Panics if the selected receivers or observed sequence do not complete.
    fn completed_round<'a>(
        control: &'a RoundControl,
        servers: &'a [Server; 3],
        claims: [(u64, u32); 9],
    ) -> TestResult<OrderedRound<'a>> {
        let mut round: OrderedRound<'a> = checked(OrderedRound::new(
            control,
            roster(servers)?,
            ROUND_BUDGET,
            PROBE_BUDGET,
        ))?;
        for (position, (midpoint, radius)) in claims.into_iter().enumerate() {
            let selected: &Server = &servers[position % 3];
            assert_eq!(checked(round.start_next())?, selected.address);
            let (_request, _response): (Vec<u8>, [u8; 476]) =
                selected.respond_with_radius(midpoint, radius)?;
            let expected: ProbeProgress = if position == 8 {
                ProbeProgress::Complete
            } else {
                ProbeProgress::Accepted
            };
            assert_eq!(checked(poll_to_settlement(&mut round)?)?, expected);
        }
        assert_eq!(round.status(), RoundStatus::Complete);
        Ok(round)
    }

    /// Build the fixture's exact-rate policy with independently specified age and width limits.
    ///
    /// # Errors
    /// Returns the first fixture policy or constant conversion failure.
    fn agreement_policy(max_age: Duration, max_width: Duration) -> TestResult<IntervalPolicy> {
        let unity: NonZeroU32 = NonZeroU32::new(1_000_000_000).ok_or("zero fixture rate")?;
        IntervalPolicy::new([unity; 2], Duration::ZERO, max_age, max_width)
            .map_err(|error| format!("invalid fixture interval policy: {error:?}").into())
    }

    /// Select inclusive fixture limits relative to the independently known date.
    ///
    /// # Errors
    /// Returns unexpected refusal of the literal finite calendar scope.
    fn date_scope(earliest_seconds: i128, latest_seconds: i128) -> TestResult<LeapDateScope> {
        Ok(LeapDateScope::new(EraWindow::new(
            (DATE_NTP_SECONDS + earliest_seconds) * Q32_SECOND,
            (DATE_NTP_SECONDS + latest_seconds) * Q32_SECOND,
        )?)?)
    }

    /// Independent expected date-admission decision for a signed round fixture.
    enum RetryRoundOutcome {
        /// Two complete traversals leave the round incomplete.
        Incomplete,
        /// All replies arrive, but the declared scope truncates the date interval.
        OutsideDateScope,
        /// The complete round yields an interval within its declared scope.
        AdmittedDate,
    }

    /// Couple the fixture's three distinct peers with their original retry bindings.
    ///
    /// # Errors
    /// Returns a fixture group or selected-alias refusal.
    fn bound_retry_roster<'a>(
        bindings: &'a mut [RetryBinding<'_>; 3],
        servers: &[Server; 3],
        second_tcp: Option<SocketAddr>,
    ) -> TestResult<[RetryRoundPeer<'a>; 3]> {
        let groups: [NonZeroU64; 3] = [
            NonZeroU64::MIN,
            NonZeroU64::new(2).ok_or("zero fixture second group")?,
            NonZeroU64::new(3).ok_or("zero fixture third group")?,
        ];
        let [first, second, third]: &mut [RetryBinding<'_>; 3] = bindings;
        let second: RetryRoundPeer<'a> = checked_exchange(match second_tcp {
            Some(tcp) => {
                RetryRoundPeer::new_with_tcp(second, servers[1].address, tcp, groups[1], groups[1])
            }
            None => RetryRoundPeer::new(second, servers[1].address, groups[1], groups[1]),
        })?;
        Ok([
            checked_exchange(RetryRoundPeer::new(
                first,
                servers[0].address,
                groups[0],
                groups[0],
            ))?,
            second,
            checked_exchange(RetryRoundPeer::new(
                third,
                servers[2].address,
                groups[2],
                groups[2],
            ))?,
        ])
    }

    /// Drive the actual retry-bound round to the expected date outcome.
    ///
    /// # Errors
    /// Returns fixture preparation, signing or unexpected round failure.
    ///
    /// # Panics
    /// Panics if the observed sequence or date admission differs from the fixture case.
    fn settle_retry_fixture(
        bindings: &mut [RetryBinding<'_>; 3],
        servers: &[Server; 3],
        outcome: &RetryRoundOutcome,
        policy: &IntervalPolicy,
        calendar: &LeapKnowledge<'_>,
    ) -> TestResult<()> {
        let (legs, upper_date_offset): (usize, i128) = match outcome {
            RetryRoundOutcome::Incomplete => (6, 30),
            RetryRoundOutcome::OutsideDateScope => (9, 10),
            RetryRoundOutcome::AdmittedDate => (9, 30),
        };

        let control: RoundControl = RoundControl::new();
        let peers: [RetryRoundPeer<'_>; 3] = bound_retry_roster(bindings, servers, None)?;
        let mut round: OrderedRound<'_> = checked(OrderedRound::new_with_retry(
            &control,
            peers,
            ROUND_BUDGET,
            PROBE_BUDGET,
        ))?;
        for position in 0..legs {
            let selected: &Server = &servers[position % 3];
            assert_eq!(checked(round.start_next())?, selected.address);
            let (_request, _response): (Vec<u8>, [u8; 476]) =
                selected.respond_with_radius(DATE_UNIX_SECONDS, 20)?;
            let expected: ProbeProgress = if position == 8 {
                ProbeProgress::Complete
            } else {
                ProbeProgress::Accepted
            };
            assert_eq!(checked(poll_to_settlement(&mut round)?)?, expected);
        }
        let scope: LeapDateScope = date_scope(-30, upper_date_offset)?;
        let result: Result<ControlledDate<'_>, IntervalError> =
            round.admit_date(policy, scope, calendar);
        match outcome {
            RetryRoundOutcome::Incomplete => {
                assert!(matches!(result, Err(IntervalError::NotComplete)));
            }
            RetryRoundOutcome::OutsideDateScope => {
                assert!(matches!(result, Err(IntervalError::DateScope)));
                let wider: LeapDateScope = date_scope(-30, 30)?;
                assert!(matches!(
                    round.admit_date(policy, wider, calendar),
                    Err(IntervalError::RetryAdmission)
                ));
            }
            RetryRoundOutcome::AdmittedDate => {
                let date: ControlledDate<'_> =
                    result.map_err(|error| format!("unexpected retry date refusal: {error:?}"))?;
                assert_eq!(date.scope(), scope);
                assert_eq!(date.agreement().supporting_groups(), 3);
                assert!(date.agreement().receipt(8).is_some());
            }
        }
        Ok(())
    }

    /// Establish one observed failure on every original binding and wait until eligible.
    ///
    /// # Errors
    /// Returns a fixture setup, socket or unexpected retry result.
    fn seeded_retry_bindings<'a>(
        servers: &'a [Server; 3],
        aliases: &[(UdpSocket, SocketAddr); 3],
    ) -> TestResult<[RetryBinding<'a>; 3]> {
        let mut bindings: [RetryBinding<'_>; 3] = [
            checked_exchange(RetryBinding::new(
                &servers[0].root,
                [servers[0].address, aliases[0].1],
            ))?,
            checked_exchange(RetryBinding::new(
                &servers[1].root,
                [servers[1].address, aliases[1].1],
            ))?,
            checked_exchange(RetryBinding::new(
                &servers[2].root,
                [servers[2].address, aliases[2].1],
            ))?,
        ];
        let one_second: Duration = Duration::from_secs(1);
        let mut due: Instant = Instant::now();
        for ((binding, server), (alias, address)) in bindings.iter_mut().zip(servers).zip(aliases) {
            let first_start: Instant = Instant::now();
            malformed_retry(binding, &server.socket, server.address)?;
            let terminal: (Instant, Instant) = (first_start, Instant::now());
            let floor: Instant =
                refused_retry_floor(binding, alias, *address, terminal, one_second)?;
            due = due.max(floor);
        }
        thread::sleep(due.saturating_duration_since(Instant::now()));
        Ok(bindings)
    }

    /// Observe retry debt on every selected binding after three date-admission outcomes.
    ///
    /// # Errors
    /// Returns fixture preparation, socket, signing, or unexpected round failure.
    ///
    /// # Panics
    /// Panics if a signed prefix or a refused date clears binding debt, or if an
    /// admitted date leaves debt on any of its three participating bindings.
    #[test]
    fn only_an_admitted_complete_date_resets_retry_debt() -> TestResult<()> {
        let one_second: Duration = Duration::from_secs(1);
        let one_and_a_half_seconds: Duration = Duration::from_millis(1_500);
        let cases: [(RetryRoundOutcome, Duration); 3] = [
            (RetryRoundOutcome::Incomplete, one_and_a_half_seconds),
            (RetryRoundOutcome::OutsideDateScope, one_and_a_half_seconds),
            (RetryRoundOutcome::AdmittedDate, one_second),
        ];
        let policy: IntervalPolicy = agreement_policy(ROUND_BUDGET, Duration::from_secs(50))?;
        let calendar: LeapKnowledge<'static> = bundled_leap_knowledge()?;

        for (outcome, expected_wait) in cases {
            let servers: [Server; 3] = operators()?;
            let aliases: [(UdpSocket, SocketAddr); 3] = [peer()?, peer()?, peer()?];
            let mut bindings: [RetryBinding<'_>; 3] = seeded_retry_bindings(&servers, &aliases)?;

            settle_retry_fixture(&mut bindings, &servers, &outcome, &policy, &calendar)?;

            for ((binding, server), (alias, address)) in
                bindings.iter_mut().zip(&servers).zip(&aliases)
            {
                let last_start: Instant = Instant::now();
                malformed_retry(binding, alias, *address)?;
                let terminal: (Instant, Instant) = (last_start, Instant::now());
                let _floor: Instant = refused_retry_floor(
                    binding,
                    &server.socket,
                    server.address,
                    terminal,
                    expected_wait,
                )?;
            }
        }
        Ok(())
    }

    /// Retain exact signed evidence while enclosing the entire date on the Q32 lattice.
    ///
    /// # Errors
    /// Returns fixture setup or unexpected date or withdrawal failure.
    ///
    /// # Panics
    /// Panics if the result loses the date scope, calendar, receipt identity, or frame guard.
    #[test]
    fn date_preserves_whole_interval_and_receipts() -> TestResult<()> {
        let servers: [Server; 3] = operators()?;
        let control: RoundControl = RoundControl::new();
        let round: OrderedRound<'_> =
            completed_round(&control, &servers, [(DATE_UNIX_SECONDS, 20); 9])?;
        let policy: IntervalPolicy = agreement_policy(ROUND_BUDGET, Duration::from_secs(50))?;
        let scope: LeapDateScope = date_scope(-30, 30)?;
        let calendar: LeapKnowledge<'static> = bundled_leap_knowledge()?;
        let date: ControlledDate<'_> = round
            .evaluate_date(&policy, scope, &calendar)
            .map_err(|error| format!("unexpected controlled date refusal: {error:?}"))?;
        assert_eq!(date.scope(), scope);
        let calendar_result: LeapInterpretation = date.interpretation();
        assert_eq!(calendar_result.identity(), calendar.identity());
        assert_eq!(calendar_result.offset(), LeapOffsetKnowledge::Known(37));
        assert_eq!(calendar_result.events(), LeapEventKnowledge::NoEvents);

        let dates: EraWindow = calendar_result.dates();
        let midpoint: i128 = DATE_NTP_SECONDS * Q32_SECOND;
        assert!(dates.earliest().ticks() < midpoint);
        assert!(dates.latest().ticks() > midpoint);
        let agreement: &ControlledAgreement<'_> = date.agreement();
        let (unix_lower, unix_upper): (u128, u128) = agreement.bounds_unix_ns();
        let unix_anchor: i128 = i128::from(DATE_UNIX_SECONDS) * NANOSECOND_DIVISOR;
        // Compare the endpoints as exact rationals, relative to the independently
        // known date in both epochs. Unity rate makes the input bounds integral ns.
        let lower_residual: i128 = (i128::try_from(unix_lower)? - unix_anchor) * Q32_SECOND
            - (dates.earliest().ticks() - midpoint) * NANOSECOND_DIVISOR;
        let upper_residual: i128 = (dates.latest().ticks() - midpoint) * NANOSECOND_DIVISOR
            - (i128::try_from(unix_upper)? - unix_anchor) * Q32_SECOND;
        assert!((0..NANOSECOND_DIVISOR).contains(&lower_residual));
        assert!((0..NANOSECOND_DIVISOR).contains(&upper_residual));

        assert_eq!(agreement.supporting_groups(), 3);
        for index in 0..9 {
            let original: &Receipt = round
                .receipt(index)
                .ok_or("missing original date receipt")?;
            let retained: &Receipt = agreement
                .receipt(index)
                .ok_or("missing retained date receipt")?;
            assert!(std::ptr::eq(retained, original));
        }

        // This finite ledger has no event or offset coverage after New Year 2017.
        let finite_calendar: LeapKnowledge<'_> = LeapKnowledge::new(
            &[(2_272_060_800, 10)],
            LeapHistoryBefore::new(3_692_217_600)?,
            None,
            None,
            LeapDataIdentity::new(2, [1; 32], [2; 32])?,
        )?;
        let finite_date: ControlledDate<'_> = round
            .evaluate_date(&policy, scope, &finite_calendar)
            .map_err(|error| format!("finite calendar rejected the date interval: {error:?}"))?;
        let finite_result: LeapInterpretation = finite_date.interpretation();
        assert_eq!(finite_result.identity(), finite_calendar.identity());
        assert_eq!(finite_result.offset(), LeapOffsetKnowledge::Unknown);
        assert_eq!(finite_result.events(), LeapEventKnowledge::Unknown);

        checked(control.withdraw(Withdrawal::ClockFrame))?;
        assert!(matches!(
            round.evaluate_date(&policy, scope, &calendar),
            Err(IntervalError::Withdrawn(Withdrawal::ClockFrame))
        ));
        Ok(())
    }

    /// Refuse either out-of-scope bound even with one bound and midpoint inside.
    ///
    /// # Errors
    /// Returns fixture setup or failure of the wider healthy date scope.
    ///
    /// # Panics
    /// Panics if a partially overlapping interval is treated as wholly supported.
    #[test]
    fn date_refuses_partially_outside_scope() -> TestResult<()> {
        let servers: [Server; 3] = operators()?;
        let control: RoundControl = RoundControl::new();
        let round: OrderedRound<'_> =
            completed_round(&control, &servers, [(DATE_UNIX_SECONDS, 20); 9])?;
        let policy: IntervalPolicy = agreement_policy(ROUND_BUDGET, Duration::from_secs(50))?;
        let calendar: LeapKnowledge<'static> = bundled_leap_knowledge()?;
        // The six-second deadline keeps the projected lower end below -14s,
        // the upper end above +20s, and the midpoint between 0s and +6s.
        for (lower, upper) in [(-30, 10), (-10, 30)] {
            let scope: LeapDateScope = date_scope(lower, upper)?;
            assert!(matches!(
                round.evaluate_date(&policy, scope, &calendar),
                Err(IntervalError::DateScope)
            ));
        }
        assert!(
            round
                .evaluate_date(&policy, date_scope(-30, 30)?, &calendar)
                .is_ok()
        );
        Ok(())
    }

    /// Preserve evidence on both sides of the known New Year 2017 leap boundary.
    ///
    /// # Errors
    /// Returns fixture setup or unexpected rejection of the bounded date evidence.
    ///
    /// # Panics
    /// Panics if a point or different era hides the effective event.
    #[test]
    fn date_preserves_known_leap_inside_the_interval() -> TestResult<()> {
        let servers: [Server; 3] = operators()?;
        let control: RoundControl = RoundControl::new();
        let round: OrderedRound<'_> =
            completed_round(&control, &servers, [(1_483_228_800, 20); 9])?;
        let policy: IntervalPolicy = agreement_policy(ROUND_BUDGET, Duration::from_secs(50))?;
        let scope: LeapDateScope = date_scope(-130, -70)?;
        let calendar: LeapKnowledge<'static> = bundled_leap_knowledge()?;
        let date: ControlledDate<'_> =
            round
                .evaluate_date(&policy, scope, &calendar)
                .map_err(|error| {
                    format!("unexpected controlled leap-spanning date refusal: {error:?}")
                })?;
        let interpretation: LeapInterpretation = date.interpretation();
        let boundary: i128 = 3_692_217_600 * Q32_SECOND;
        assert!(interpretation.dates().earliest().ticks() < boundary);
        assert!(interpretation.dates().latest().ticks() > boundary);
        assert_eq!(interpretation.offset(), LeapOffsetKnowledge::Varying);
        assert_eq!(interpretation.events(), LeapEventKnowledge::KnownEvents);
        assert_eq!(interpretation.known_transition_count(), 1);
        Ok(())
    }

    /// Use all three observations of each peer without assigning extra support.
    ///
    /// # Errors
    /// Returns the first fixture or unexpected evaluation failure.
    ///
    /// # Panics
    /// Panics if the common interval, original receipt identity, or count differs.
    #[test]
    fn projects_nine_original_receipts_as_three_groups() -> TestResult<()> {
        let servers: [Server; 3] = operators()?;
        let control: RoundControl = RoundControl::new();
        let round: OrderedRound<'_> = completed_round(&control, &servers, [(1_000, 5); 9])?;
        let policy: IntervalPolicy = agreement_policy(ROUND_BUDGET, Duration::from_secs(20))?;
        let agreement: ControlledAgreement<'_> = round
            .evaluate(&policy)
            .map_err(|error| format!("unexpected fixture evaluation refusal: {error:?}"))?;
        assert_eq!(agreement.supporting_groups(), 3);
        for index in 0..9 {
            let original: &Receipt = round.receipt(index).ok_or("missing original receipt")?;
            let retained: &Receipt = agreement
                .receipt(index)
                .ok_or("missing agreement receipt")?;
            assert!(std::ptr::eq(retained, original));
        }
        assert!(agreement.receipt(9).is_none());

        // Identical signed bounds make the earliest receive and latest send the
        // controlling extremes. All nine actual exchanges are strictly sequential.
        let first: &Receipt = round.receipt(0).ok_or("missing earliest receipt")?;
        let last: &Receipt = round.receipt(8).ok_or("missing latest receipt")?;
        let (_, (_, first_read)): ((Instant, Instant), (Instant, Instant)) = first.captures();
        let ((last_send, _), _): ((Instant, Instant), (Instant, Instant)) = last.captures();
        let evaluated_at: Instant = agreement.evaluated_at();
        let since_first_read: Duration = evaluated_at
            .checked_duration_since(first_read)
            .ok_or("evaluation precedes first read")?;
        let since_last_send: Duration = evaluated_at
            .checked_duration_since(last_send)
            .ok_or("evaluation precedes last send")?;
        assert_eq!(
            agreement.bounds_unix_ns(),
            (
                995_000_000_000 + since_first_read.as_nanos(),
                1_005_000_000_000 + since_last_send.as_nanos(),
            )
        );
        Ok(())
    }

    /// Make a broad observation overlap every narrow observation without granting credit.
    ///
    /// # Errors
    /// Returns the first fixture or policy setup failure.
    ///
    /// # Panics
    /// Panics if a narrow common intersection masks a broad original observation.
    #[test]
    fn rejects_overbroad_observations_before_cross_group_narrowing() -> TestResult<()> {
        let servers: [Server; 3] = operators()?;
        let control: RoundControl = RoundControl::new();
        let claims: [(u64, u32); 9] = [
            (1_000, 30),
            (1_000, 5),
            (1_000, 5),
            (1_000, 30),
            (1_000, 5),
            (1_000, 5),
            (1_000, 30),
            (1_000, 5),
            (1_000, 5),
        ];
        let round: OrderedRound<'_> = completed_round(&control, &servers, claims)?;
        let policy: IntervalPolicy = agreement_policy(ROUND_BUDGET, Duration::from_secs(20))?;
        assert!(matches!(
            round.evaluate(&policy),
            Err(IntervalError::TooWide)
        ));
        Ok(())
    }

    /// Distinguish a peer's own inconsistent reports from disjoint group support.
    ///
    /// # Errors
    /// Returns the first fixture or policy setup failure.
    ///
    /// # Panics
    /// Panics if a causally complete round grants agreement despite either disagreement.
    #[test]
    fn distinguishes_within_group_and_cross_group_disagreement() -> TestResult<()> {
        let servers: [Server; 3] = operators()?;
        let policy: IntervalPolicy = agreement_policy(ROUND_BUDGET, Duration::from_secs(70))?;
        let within_control: RoundControl = RoundControl::new();
        let within: OrderedRound<'_> = completed_round(
            &within_control,
            &servers,
            [
                (900, 5),
                (920, 5),
                (940, 5),
                (960, 5),
                (980, 5),
                (1_000, 5),
                (1_020, 5),
                (1_040, 5),
                (1_060, 5),
            ],
        )?;
        assert!(matches!(
            within.evaluate(&policy),
            Err(IntervalError::GroupDisagreement)
        ));

        let across_control: RoundControl = RoundControl::new();
        let across: OrderedRound<'_> = completed_round(
            &across_control,
            &servers,
            [
                (1_000, 30),
                (1_000, 30),
                (1_000, 30),
                (980, 5),
                (1_000, 5),
                (1_020, 5),
                (1_000, 30),
                (1_000, 30),
                (1_000, 30),
            ],
        )?;
        assert!(matches!(
            across.evaluate(&policy),
            Err(IntervalError::Disagreement)
        ));
        Ok(())
    }

    /// Require the third visit to agree with the first two visits to its group.
    ///
    /// # Errors
    /// Returns the first fixture or unexpected round failure.
    ///
    /// # Panics
    /// Panics if a causally ordered third traversal is omitted from group intersection.
    #[test]
    fn third_traversal_can_refuse_a_previously_compatible_group() -> TestResult<()> {
        let servers: [Server; 3] = operators()?;
        let control: RoundControl = RoundControl::new();
        // The first six signed windows are all 995..1005. The last three are
        // 1015..1025, which satisfy raw causal order while remaining disjoint
        // after at most six seconds of projection.
        let claims: [(u64, u32); 9] = [
            (1_000, 5),
            (1_000, 5),
            (1_000, 5),
            (1_000, 5),
            (1_000, 5),
            (1_000, 5),
            (1_020, 5),
            (1_020, 5),
            (1_020, 5),
        ];
        let round: OrderedRound<'_> = completed_round(&control, &servers, claims)?;
        let policy: IntervalPolicy = agreement_policy(ROUND_BUDGET, Duration::from_secs(20))?;
        assert!(matches!(
            round.evaluate(&policy),
            Err(IntervalError::GroupDisagreement)
        ));
        Ok(())
    }

    /// Require complete evidence and refuse its use after freshness or frame loss.
    ///
    /// # Errors
    /// Returns the first fixture or unexpected round failure.
    ///
    /// # Panics
    /// Panics if incomplete, stale, or clock-frame-withdrawn evidence evaluates.
    #[test]
    fn rejects_incomplete_stale_and_withdrawn_round_evidence() -> TestResult<()> {
        let servers: [Server; 3] = operators()?;
        let open_control: RoundControl = RoundControl::new();
        let incomplete: OrderedRound<'_> = checked(OrderedRound::new(
            &open_control,
            roster(&servers)?,
            ROUND_BUDGET,
            PROBE_BUDGET,
        ))?;
        let current: IntervalPolicy = agreement_policy(ROUND_BUDGET, Duration::from_secs(20))?;
        assert!(matches!(
            incomplete.evaluate(&current),
            Err(IntervalError::NotComplete)
        ));

        let complete_control: RoundControl = RoundControl::new();
        let completed: OrderedRound<'_> =
            completed_round(&complete_control, &servers, [(1_000, 5); 9])?;
        let brief: IntervalPolicy =
            agreement_policy(Duration::from_millis(5), Duration::from_secs(20))?;
        thread::sleep(Duration::from_millis(10));
        assert!(matches!(
            completed.evaluate(&brief),
            Err(IntervalError::Stale)
        ));
        checked(complete_control.withdraw(Withdrawal::ClockFrame))?;
        assert!(matches!(
            completed.evaluate(&current),
            Err(IntervalError::Withdrawn(Withdrawal::ClockFrame))
        ));
        assert_eq!(completed.status(), RoundStatus::Complete);
        Ok(())
    }

    /// Check three traversals in the original order and the whole-frame chain.
    ///
    /// # Errors
    /// Returns the first fixture or unexpected round failure.
    ///
    /// # Panics
    /// Panics if selection, chained nonce, signed interval or receipt custody differs.
    #[test]
    fn collects_three_same_order_passes_with_exact_response_chaining() -> TestResult<()> {
        let servers: [Server; 3] = operators()?;
        let control: RoundControl = RoundControl::new();
        let mut round: OrderedRound<'_> = checked(OrderedRound::new(
            &control,
            roster(&servers)?,
            ROUND_BUDGET,
            PROBE_BUDGET,
        ))?;
        let policy: IntervalPolicy = agreement_policy(ROUND_BUDGET, Duration::from_secs(20))?;
        let scope: LeapDateScope = date_scope(-30, 30)?;
        let calendar: LeapKnowledge<'static> = bundled_leap_knowledge()?;
        let mut previous: Option<[u8; 476]> = None;
        let mut link_values: Vec<[u8; 32]> = Vec::with_capacity(8);
        let sequence: [(usize, u64); 9] = [
            (0, 1_020),
            (1, 1_010),
            (2, 1_015),
            (0, 1_016),
            (1, 1_017),
            (2, 1_018),
            (0, 1_019),
            (1, 1_020),
            (2, 1_021),
        ];
        for (position, (operator, midpoint)) in sequence.into_iter().enumerate() {
            let selected: &Server = &servers[operator];
            assert_eq!(checked(round.start_next())?, selected.address);
            assert!(matches!(round.start_next(), Err(RoundError::InFlight)));
            let (request, response): (Vec<u8>, [u8; 476]) = selected.respond(midpoint)?;
            let progress: ProbeProgress = checked(poll_to_settlement(&mut round)?)?;
            let expected: ProbeProgress = if position == 8 {
                ProbeProgress::Complete
            } else {
                ProbeProgress::Accepted
            };
            assert_eq!(progress, expected);
            let receipt: &Receipt = round.receipt(position).ok_or("missing round receipt")?;
            assert_eq!(receipt.peer(), selected.address);
            assert_eq!(
                receipt.root_key_bytes(),
                selected.trust_anchor.verifying_key().as_bytes()
            );
            assert_eq!(receipt.bounds_seconds(), (midpoint - 5, midpoint + 5));
            assert_eq!(receipt.request_frame(), request.as_slice());
            assert_eq!(receipt.response_frame(), response.as_slice());
            if let Some(predecessor) = &previous {
                let salt: &[u8; 32] = receipt.chaining_randomness().ok_or("missing chain salt")?;
                assert_eq!(observed_nonce(&request), Some(digest(&[predecessor, salt])));
                assert!(!link_values.contains(salt));
                link_values.push(*salt);
            } else {
                assert!(observed_nonce(&request).is_some());
                assert!(receipt.chaining_randomness().is_none());
            }
            previous = Some(response);

            if matches!(position, 5 | 7) {
                assert_eq!(round.status(), RoundStatus::Active);
                assert!(round.receipt(position + 1).is_none());
                assert!(matches!(
                    round.evaluate(&policy),
                    Err(IntervalError::NotComplete)
                ));
                assert!(matches!(
                    round.evaluate_date(&policy, scope, &calendar),
                    Err(IntervalError::NotComplete)
                ));
            }
        }
        assert_eq!(link_values.len(), 8);
        assert_eq!(round.status(), RoundStatus::Complete);
        assert!(matches!(round.start_next(), Err(RoundError::Finished)));
        assert!(round.receipt(9).is_none());
        for server in &servers {
            let mut unexpected: [u8; 1] = [0];
            assert!(
                matches!(server.socket.recv_from(&mut unexpected), Err(error) if error.kind() == ErrorKind::WouldBlock)
            );
        }
        Ok(())
    }

    /// Observe a contradiction only detectable when checking a nonadjacent pair.
    ///
    /// # Errors
    /// Returns the first fixture or unexpected round failure.
    ///
    /// # Panics
    /// Panics if the nonadjacent signed contradiction or its original receipt is erased.
    #[test]
    fn retains_nonadjacent_signed_conflict_after_stop() -> TestResult<()> {
        let servers: [Server; 3] = operators()?;
        // A later upper bound of 1014 contradicts only the first receipt's
        // lower bound of 1015. Exercise both the first and third traversals.
        for terminal in [2, 6] {
            let control: RoundControl = RoundControl::new();
            let mut round: OrderedRound<'_> = checked(OrderedRound::new(
                &control,
                roster(&servers)?,
                ROUND_BUDGET,
                PROBE_BUDGET,
            ))?;
            for position in 0..=terminal {
                let midpoint: u64 = match position {
                    0 => 1_020,
                    position if position == terminal => 1_009,
                    _ => 1_010,
                };
                let selected: &Server = &servers[position % 3];
                assert_eq!(checked(round.start_next())?, selected.address);
                let (_request, response): (Vec<u8>, [u8; 476]) = selected.respond(midpoint)?;
                let progress: Result<ProbeProgress, RoundError> = poll_to_settlement(&mut round)?;
                if position < terminal {
                    assert!(matches!(progress, Ok(ProbeProgress::Accepted)));
                } else {
                    assert!(
                        matches!(progress, Err(RoundError::Conflict(conflict)) if conflict.positions() == (0, terminal))
                    );
                }
                let original: &Receipt =
                    round.receipt(position).ok_or("missing signed evidence")?;
                assert_eq!(original.response_frame(), response.as_slice());
            }
            checked(control.withdraw(Withdrawal::Stop))?;
            assert!(round.start_next().is_err());
            let policy: IntervalPolicy = agreement_policy(ROUND_BUDGET, Duration::from_secs(20))?;
            assert!(
                matches!(round.evaluate(&policy), Err(IntervalError::Conflict(conflict)) if conflict.positions() == (0, terminal))
            );
            assert!(
                matches!(round.status(), RoundStatus::Conflict(conflict) if conflict.positions() == (0, terminal))
            );
            assert_eq!(
                round.receipt(0).map(Receipt::bounds_seconds),
                Some((1_015, 1_025))
            );
            assert_eq!(
                round.receipt(terminal).map(Receipt::bounds_seconds),
                Some((1_004, 1_014))
            );
            assert!(round.receipt(terminal + 1).is_none());
        }
        Ok(())
    }

    /// Independently reject duplicate operator and clock labels before sending.
    ///
    /// # Errors
    /// Returns fixture socket and checked label failures.
    ///
    /// # Panics
    /// Panics if three distinct addresses and keys hide a shared declared group.
    #[test]
    fn rejects_shared_operator_or_clock_with_distinct_roots() -> TestResult<()> {
        let servers: [Server; 3] = operators()?;
        let primary: NonZeroU64 = NonZeroU64::MIN;
        let secondary: NonZeroU64 = NonZeroU64::new(2).ok_or("zero fixture group")?;
        let tertiary: NonZeroU64 = NonZeroU64::new(3).ok_or("zero fixture group")?;
        for (operator, clock) in [(primary, secondary), (secondary, primary)] {
            let selected: [RoundPeer<'_>; 3] = [
                RoundPeer::new(servers[0].address, &servers[0].root, primary, primary),
                RoundPeer::new(servers[1].address, &servers[1].root, operator, clock),
                RoundPeer::new(servers[2].address, &servers[2].root, tertiary, tertiary),
            ];
            let control: RoundControl = RoundControl::new();
            assert!(matches!(
                OrderedRound::new(&control, selected, ROUND_BUDGET, PROBE_BUDGET),
                Err(RoundError::Configuration)
            ));
        }
        Ok(())
    }

    /// Withdraw the round after its first receipt and observe no successor send.
    ///
    /// # Errors
    /// Returns the first fixture or unexpected round failure.
    ///
    /// # Panics
    /// Panics if withdrawal allows the next peer to receive a request.
    #[test]
    fn withdrawal_blocks_a_successor_after_the_first_receipt() -> TestResult<()> {
        let servers: [Server; 3] = operators()?;
        let control: RoundControl = RoundControl::new();
        let mut round: OrderedRound<'_> = checked(OrderedRound::new(
            &control,
            roster(&servers)?,
            ROUND_BUDGET,
            PROBE_BUDGET,
        ))?;
        assert_eq!(checked(round.start_next())?, servers[0].address);
        let (_request, _response): (Vec<u8>, [u8; 476]) = servers[0].respond(1_000)?;
        assert!(matches!(
            checked(poll_to_settlement(&mut round)?)?,
            ProbeProgress::Accepted
        ));
        checked(control.withdraw(Withdrawal::Key))?;
        assert!(matches!(
            round.start_next(),
            Err(RoundError::Withdrawn(Withdrawal::Key))
        ));
        let mut unexpected: [u8; 1] = [0; 1];
        assert!(
            matches!(servers[1].socket.recv_from(&mut unexpected), Err(error) if error.kind() == ErrorKind::WouldBlock)
        );
        assert!(matches!(round.status(), RoundStatus::Unavailable));
        assert!(round.receipt(0).is_some());
        Ok(())
    }

    /// Distinguish a delayed signed reply from expiry of the original whole round.
    ///
    /// # Errors
    /// Returns the first fixture or unexpected round failure.
    ///
    /// # Panics
    /// Panics if delay grants credit or a successor renews the elapsed cutoff.
    #[test]
    fn enforces_per_probe_delay_and_original_cutoff() -> TestResult<()> {
        let servers: [Server; 3] = operators()?;
        let delay_control: RoundControl = RoundControl::new();
        let mut delayed: OrderedRound<'_> = checked(OrderedRound::new(
            &delay_control,
            roster(&servers)?,
            ROUND_BUDGET,
            Duration::from_millis(20),
        ))?;
        assert_eq!(checked(delayed.start_next())?, servers[0].address);
        thread::sleep(Duration::from_millis(30));
        let (_request, response): (Vec<u8>, [u8; 476]) = servers[0].respond(1_000)?;
        assert!(matches!(
            poll_to_settlement(&mut delayed)?,
            Err(RoundError::Delay)
        ));
        assert_eq!(delayed.status(), RoundStatus::Unavailable);
        assert_eq!(
            delayed.receipt(0).map(Receipt::response_frame),
            Some(response.as_slice())
        );
        assert!(matches!(delayed.start_next(), Err(RoundError::Finished)));

        let cutoff_control: RoundControl = RoundControl::new();
        let budget: Duration = Duration::from_secs(1);
        let mut expired: OrderedRound<'_> = checked(OrderedRound::new(
            &cutoff_control,
            roster(&servers)?,
            budget,
            budget,
        ))?;
        assert_eq!(checked(expired.start_next())?, servers[0].address);
        let (_request, _response): (Vec<u8>, [u8; 476]) = servers[0].respond(1_000)?;
        assert_eq!(
            checked(poll_to_settlement(&mut expired)?)?,
            ProbeProgress::Accepted
        );
        thread::sleep(budget);
        assert!(matches!(
            expired.start_next(),
            Err(RoundError::Exchange(ExchangeError::Expired))
        ));
        assert_eq!(expired.status(), RoundStatus::Unavailable);
        assert!(expired.receipt(1).is_none());
        let mut unexpected: [u8; 1] = [0; 1];
        assert!(
            matches!(servers[1].socket.recv_from(&mut unexpected), Err(error) if error.kind() == ErrorKind::WouldBlock)
        );
        Ok(())
    }

    /// The exact request seen by the peer and its signed response.
    type SignedFrames = (Vec<u8>, [u8; 476]);

    /// Send one signed reply through the guarded recovery round.
    ///
    /// # Errors
    /// Returns the original send or fixture failure, or a distinct observer timeout.
    ///
    /// # Panics
    /// Panics if the round selects a different endpoint from the expected peer.
    fn recovery_leg(
        round: &mut RecoveryRound<'_>,
        selected: &Server,
        midpoint: u64,
    ) -> TestResult<(SignedFrames, Result<ProbeProgress, RoundError>)> {
        assert_eq!(checked(round.start_next())?, selected.address);
        let frames: SignedFrames = selected.respond(midpoint)?;
        Ok((frames, poll_recovery_to_settlement(round)?))
    }

    /// Observe an actual signed reply's recovery disposition.
    ///
    /// # Errors
    /// Returns a distinct observer timeout if the fixture never settles.
    fn poll_recovery_to_settlement(
        round: &mut RecoveryRound<'_>,
    ) -> TestResult<Result<ProbeProgress, RoundError>> {
        let started: Instant = Instant::now();
        while started.elapsed() < PROBE_BUDGET {
            match round.poll() {
                Ok(ProbeProgress::Waiting) => thread::yield_now(),
                result => return Ok(result),
            }
        }
        Err("Roughtime recovery round did not settle its reply".into())
    }

    /// Derive the fixed fixture date policy independently of the acquired replies.
    ///
    /// # Errors
    /// Returns an invalid fixture policy or date-scope declaration.
    fn recovery_date_policy<'calendar>(
        calendar: &'calendar LeapKnowledge<'calendar>,
    ) -> TestResult<RecoveryDatePolicy<'calendar>> {
        let scope: LeapDateScope = date_scope(-30, 30)?;
        let policy: IntervalPolicy = agreement_policy(ROUND_BUDGET, Duration::from_secs(20))?;
        Ok(RecoveryDatePolicy::new(policy, scope, calendar))
    }

    /// Complete the healthy nine-leg date with each selected fixture operator.
    ///
    /// # Errors
    /// Returns the first fixture or unexpected recovery error.
    ///
    /// # Panics
    /// Panics if the sequence does not complete all original exchanges.
    fn complete_recovery_date_round(
        round: &mut RecoveryRound<'_>,
        servers: &[Server; 3],
    ) -> TestResult<()> {
        for position in 0..9 {
            let (_frames, progress): (SignedFrames, Result<ProbeProgress, RoundError>) =
                recovery_leg(round, &servers[position % 3], DATE_UNIX_SECONDS)?;
            let expected: ProbeProgress = if position < 8 {
                ProbeProgress::Accepted
            } else {
                ProbeProgress::Complete
            };
            assert_eq!(checked(progress)?, expected);
        }
        assert_eq!(round.status(), RoundStatus::Complete);
        Ok(())
    }

    /// Drive both bound recovery rounds and check whether the follow-up admits a date.
    ///
    /// # Errors
    /// Returns fixture admission, socket, signing, timing or recovery failure.
    ///
    /// # Panics
    /// Panics if the explicit roster can replace a bound one or the follow-up ignores conflict.
    fn settle_bound_recovery_fixture(
        bindings: &mut [RetryBinding<'_>; 3],
        servers: &[Server; 3],
        retain_conflict: bool,
    ) -> TestResult<()> {
        let calendar: LeapKnowledge<'static> = bundled_leap_knowledge()?;
        let policy: RecoveryDatePolicy<'_> = recovery_date_policy(&calendar)?;
        let control: RecoveryControl = RecoveryControl::new();
        let peers: [RetryRoundPeer<'_>; 3] = bound_retry_roster(bindings, servers, None)?;
        let mut recovery: ControlledRecovery<'_> = checked(ControlledRecovery::new_with_retry(
            &control,
            ROUND_BUDGET + ROUND_BUDGET,
            policy,
            peers,
        ))?;
        let first_control: RoundControl = RoundControl::new();
        assert!(matches!(
            recovery.begin(&first_control, roster(servers)?, ROUND_BUDGET, PROBE_BUDGET),
            Err(RoundError::Configuration)
        ));
        {
            let mut first: RecoveryRound<'_> =
                checked(recovery.begin_bound(&first_control, ROUND_BUDGET, PROBE_BUDGET))?;
            if retain_conflict {
                // Radius 5. The second upper bound equals the first lower bound.
                // Only the third reply contradicts the first.
                let (_frames, first_reply): (SignedFrames, Result<ProbeProgress, RoundError>) =
                    recovery_leg(&mut first, &servers[0], 1_020)?;
                assert_eq!(checked(first_reply)?, ProbeProgress::Accepted);
                let (_frames, second_reply): (SignedFrames, Result<ProbeProgress, RoundError>) =
                    recovery_leg(&mut first, &servers[1], 1_010)?;
                assert_eq!(checked(second_reply)?, ProbeProgress::Accepted);
                let (_frames, conflict_reply): (SignedFrames, Result<ProbeProgress, RoundError>) =
                    recovery_leg(&mut first, &servers[2], 1_009)?;
                assert!(
                    matches!(conflict_reply, Err(RoundError::Conflict(conflict)) if conflict.positions() == (0, 2))
                );
            } else {
                complete_recovery_date_round(&mut first, servers)?;
            }
        }
        if retain_conflict {
            // A previously failed binding charges its second failure on the
            // rejected leg. Waiting from the observed rejection is conservative.
            thread::sleep(Duration::from_millis(1_500));
        }
        let follow_up_control: RoundControl = RoundControl::new();
        let mut follow_up: RecoveryRound<'_> =
            checked(recovery.begin_bound(&follow_up_control, ROUND_BUDGET, PROBE_BUDGET))?;
        complete_recovery_date_round(&mut follow_up, servers)?;
        let admitted: Result<ControlledDate<'_>, IntervalError> = follow_up.admit_date();
        if retain_conflict {
            assert!(
                matches!(admitted, Err(IntervalError::Conflict(conflict)) if conflict.positions() == (0, 2))
            );
        } else {
            let date: ControlledDate<'_> = admitted
                .map_err(|error| format!("unexpected bound recovery date refusal: {error:?}"))?;
            assert_eq!(date.agreement().supporting_groups(), 3);
            assert!(date.agreement().receipt(8).is_some());
            assert!(matches!(
                follow_up.admit_date(),
                Err(IntervalError::RetryAdmission)
            ));
        }
        Ok(())
    }

    /// Observe retry debt after equivalent follow-ups with and without a signed conflict.
    ///
    /// # Errors
    /// Returns a fixture, socket or unexpected recovery or retry result.
    ///
    /// # Panics
    /// Panics if the healthy follow-up clears a retained conflict or fails to reset clean debt.
    #[test]
    fn bound_recovery_retains_retry_debt_across_signed_conflict() -> TestResult<()> {
        let cases: [(bool, [Duration; 3]); 2] = [
            (false, [Duration::from_secs(1); 3]),
            (
                true,
                [
                    Duration::from_millis(1_500),
                    Duration::from_millis(1_500),
                    Duration::from_millis(2_250),
                ],
            ),
        ];
        for (retain_conflict, expected_waits) in cases {
            let servers: [Server; 3] = operators()?;
            let aliases: [(UdpSocket, SocketAddr); 3] = [peer()?, peer()?, peer()?];
            let mut bindings: [RetryBinding<'_>; 3] = seeded_retry_bindings(&servers, &aliases)?;

            settle_bound_recovery_fixture(&mut bindings, &servers, retain_conflict)?;

            for (((binding, server), (alias, address)), expected_wait) in bindings
                .iter_mut()
                .zip(&servers)
                .zip(&aliases)
                .zip(expected_waits)
            {
                let observed: Instant = Instant::now();
                malformed_retry(binding, alias, *address)?;
                let terminal: (Instant, Instant) = (observed, Instant::now());
                let _floor: Instant = refused_retry_floor(
                    binding,
                    &server.socket,
                    server.address,
                    terminal,
                    expected_wait,
                )?;
            }
        }
        Ok(())
    }

    /// Reserve a TCP listener distinct from the three selected UDP endpoints.
    ///
    /// # Errors
    /// Returns the original loopback socket error or inability to reserve a distinct endpoint.
    fn fallback_listener(servers: &[Server; 3]) -> TestResult<TcpListener> {
        let mut overlapping: Vec<TcpListener> = Vec::with_capacity(servers.len());
        for _ in 0..=servers.len() {
            let listener: TcpListener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))?;
            let endpoint: SocketAddr = listener.local_addr()?;
            if servers.iter().all(|server| server.address != endpoint) {
                listener.set_nonblocking(true)?;
                return Ok(listener);
            }
            // Keep an overlapping TCP port reserved while trying the next one.
            overlapping.push(listener);
        }
        Err("no distinct fixture TCP address".into())
    }

    /// Bind the full fixture roster with TCP advertised only for its second peer.
    ///
    /// # Errors
    /// Returns an original alias or root binding refusal.
    fn fallback_bindings(
        servers: &[Server; 3],
        tcp: SocketAddr,
        other_aliases: [SocketAddr; 2],
    ) -> TestResult<[RetryBinding<'_>; 3]> {
        Ok([
            checked_exchange(RetryBinding::new(
                &servers[0].root,
                [servers[0].address, other_aliases[0]],
            ))?,
            checked_exchange(RetryBinding::new(
                &servers[1].root,
                [servers[1].address, tcp],
            ))?,
            checked_exchange(RetryBinding::new(
                &servers[2].root,
                [servers[2].address, other_aliases[1]],
            ))?,
        ])
    }

    /// Submit one original TCP request, sign that request, and observe stream retirement.
    ///
    /// # Errors
    /// Returns an unexpected round, fixture, or native socket failure.
    ///
    /// # Panics
    /// Panics if the round sends to an unexpected peer or completes before a response.
    fn tcp_recovery_leg(
        round: &mut RecoveryRound<'_>,
        selected: &Server,
        listener: &TcpListener,
        runtime: Option<&Runtime>,
    ) -> TestResult<(SignedFrames, Result<ProbeProgress, RoundError>)> {
        let endpoint: SocketAddr = match runtime {
            Some(runtime) => checked(runtime.block_on(async { round.start_next() }))?,
            None => checked(round.start_next())?,
        };
        assert_eq!(endpoint, listener.local_addr()?);
        let mut peer: TcpStream = match runtime {
            Some(runtime) => runtime.block_on(accept_worker_peer(listener))?,
            None => accept_peer(listener)?,
        };
        let request: [u8; LEFT_REQUEST.len()] = submit_request(&mut peer, || {
            assert_eq!(checked(round.poll())?, ProbeProgress::Waiting);
            Ok(())
        })?;
        let response: [u8; 476] = selected.signed_response(&request, DATE_UNIX_SECONDS, 5)?;
        peer.write_all(&response)?;
        let progress: Result<ProbeProgress, RoundError> = poll_recovery_to_settlement(round)?;
        assert_retired(&mut peer)?;
        Ok(((Vec::from(request), response), progress))
    }

    /// Wait for the listener to observe the worker's actual connection.
    ///
    /// # Errors
    /// Returns a native listener failure or a distinct fixture watchdog result.
    async fn accept_worker_peer(listener: &TcpListener) -> TestResult<TcpStream> {
        let start: Instant = Instant::now();
        while start.elapsed() < PROBE_BUDGET {
            match accept_peer(listener) {
                Ok(peer) => return Ok(peer),
                Err(error) if error.kind() == ErrorKind::WouldBlock => {
                    tokio::time::sleep(Duration::from_millis(1)).await;
                }
                Err(error) => return Err(error.into()),
            }
        }
        Err("worker TCP connection did not reach the selected listener".into())
    }

    /// Verify the exact frames and causal sequence admitted after abandoning an old UDP prefix.
    ///
    /// # Errors
    /// Returns the original fixture failure or an unexpected round or date refusal.
    ///
    /// # Panics
    /// Panics if an early TCP connection is observed, the floor moves, an old frame
    /// enters the date, or the restarted round fails to complete all nine legs.
    #[test]
    fn unanswered_udp_restarts_complete_chain_over_advertised_tcp() -> TestResult<()> {
        let servers: [Server; 3] = operators()?;
        let listener: TcpListener = fallback_listener(&servers)?;
        let tcp: SocketAddr = listener.local_addr()?;
        let aliases: [(UdpSocket, SocketAddr); 2] = [peer()?, peer()?];
        let mut bindings: [RetryBinding<'_>; 3] =
            fallback_bindings(&servers, tcp, [aliases[0].1, aliases[1].1])?;
        let calendar: LeapKnowledge<'static> = bundled_leap_knowledge()?;
        let authority: RecoveryControl = RecoveryControl::new();
        let controls: [RoundControl; 2] = [RoundControl::new(), RoundControl::new()];
        let peers: [RetryRoundPeer<'_>; 3] =
            bound_retry_roster(&mut bindings, &servers, Some(tcp))?;
        let mut recovery: ControlledRecovery<'_> = checked(ControlledRecovery::new_with_fallback(
            &authority,
            ROUND_BUDGET + ROUND_BUDGET,
            recovery_date_policy(&calendar)?,
            peers,
            UDP_WAIT,
        ))?;
        let ((old_request, old_response), terminal): (SignedFrames, (Instant, Instant)) = {
            let mut first: RecoveryRound<'_> =
                checked(recovery.begin_bound(&controls[0], ROUND_BUDGET, PROBE_BUDGET))?;
            let (frames, progress): (SignedFrames, Result<ProbeProgress, RoundError>) =
                recovery_leg(&mut first, &servers[0], DATE_UNIX_SECONDS)?;
            assert_eq!(checked(progress)?, ProbeProgress::Accepted);
            assert_eq!(checked(first.start_next())?, servers[1].address);
            let (_silent_request, _sender): (Vec<u8>, SocketAddr) =
                receive_request(&servers[1].socket)?;
            assert_eq!(checked(first.poll())?, ProbeProgress::Waiting);
            assert!(
                matches!(listener.accept(), Err(error) if error.kind() == ErrorKind::WouldBlock)
            );
            thread::sleep(UDP_WAIT);
            let before: Instant = Instant::now();
            assert!(matches!(
                first.poll(),
                Err(RoundError::Exchange(ExchangeError::Unanswered))
            ));
            assert_eq!(first.status(), RoundStatus::Unavailable);
            assert!(matches!(
                first.admit_date(),
                Err(IntervalError::NotComplete)
            ));
            (frames, (before, Instant::now()))
        };
        let prefix: &[Receipt] = recovery
            .fallback_prefix()
            .ok_or("missing original UDP prefix")?;
        assert_eq!(prefix.len(), 1);
        assert_eq!(prefix[0].request_frame(), old_request);
        assert_eq!(prefix[0].response_frame(), old_response);
        let until: Instant = match recovery.begin_bound(&controls[1], ROUND_BUDGET, PROBE_BUDGET) {
            Err(RoundError::Exchange(ExchangeError::RetryDeferred { until })) => until,
            Err(error) => {
                return Err(format!("unexpected fallback retry refusal: {error:?}").into());
            }
            Ok(_) => return Err("fallback began before the original retry floor".into()),
        };
        assert_retry_wait(until, terminal, FIRST_RETRY_WAIT)?;
        let short_budget: Duration = Duration::from_millis(10);
        assert!(matches!(
            recovery.begin_bound(&controls[1], short_budget, short_budget),
            Err(RoundError::Expired)
        ));
        assert!(matches!(listener.accept(), Err(error) if error.kind() == ErrorKind::WouldBlock));
        thread::sleep(until.saturating_duration_since(Instant::now()));

        let mut round: RecoveryRound<'_> =
            checked(recovery.begin_bound(&controls[1], ROUND_BUDGET, PROBE_BUDGET))?;
        let frames: Vec<ObservedFallbackLeg> =
            drive_fallback_round(&mut round, &servers, &listener, None)?;

        let date: ControlledDate<'_> = round
            .admit_date()
            .map_err(|error| format!("unexpected fallback date refusal: {error:?}"))?;
        assert_fresh_fallback_chain(date.agreement(), &frames, &old_request)?;
        assert!(matches!(listener.accept(), Err(error) if error.kind() == ErrorKind::WouldBlock));
        for server in &servers {
            let mut unexpected: [u8; 1] = [0];
            assert!(
                matches!(server.socket.recv_from(&mut unexpected), Err(error) if error.kind() == ErrorKind::WouldBlock)
            );
        }
        assert!(matches!(
            round.admit_date(),
            Err(IntervalError::RetryAdmission)
        ));
        Ok(())
    }

    /// Frames and transport observed by the fixture for one restarted visit.
    type ObservedFallbackLeg = (SignedFrames, SocketAddr, ExchangeTransport);

    /// Collect the complete sequence from the selected TCP and UDP fixture peers.
    ///
    /// # Errors
    /// Returns an unexpected round, fixture or socket failure.
    ///
    /// # Panics
    /// Panics if the round accepts or completes at a different position.
    fn drive_fallback_round(
        round: &mut RecoveryRound<'_>,
        servers: &[Server; 3],
        listener: &TcpListener,
        runtime: Option<&Runtime>,
    ) -> TestResult<Vec<ObservedFallbackLeg>> {
        let tcp: SocketAddr = listener.local_addr()?;
        let mut frames: Vec<ObservedFallbackLeg> = Vec::with_capacity(9);
        for position in 0..9 {
            let slot: usize = position % servers.len();
            let server: &Server = &servers[slot];
            let (observed, progress, endpoint, transport): (
                SignedFrames,
                Result<ProbeProgress, RoundError>,
                SocketAddr,
                ExchangeTransport,
            ) = if slot == 1 {
                let (observed, progress): (SignedFrames, Result<ProbeProgress, RoundError>) =
                    tcp_recovery_leg(round, server, listener, runtime)?;
                (observed, progress, tcp, ExchangeTransport::Tcp)
            } else {
                let (observed, progress): (SignedFrames, Result<ProbeProgress, RoundError>) =
                    recovery_leg(round, server, DATE_UNIX_SECONDS)?;
                (observed, progress, server.address, ExchangeTransport::Udp)
            };
            let expected: ProbeProgress = if position == 8 {
                ProbeProgress::Complete
            } else {
                ProbeProgress::Accepted
            };
            assert_eq!(checked(progress)?, expected);
            frames.push((observed, endpoint, transport));
        }
        Ok(frames)
    }

    /// Check all nine exact frames, their observed transports and fresh chain ancestry.
    ///
    /// # Errors
    /// Returns a missing expected receipt or its chaining salt.
    ///
    /// # Panics
    /// Panics when the new roster, frames or nonce history differ from fixture observations.
    fn assert_fresh_fallback_chain(
        agreement: &ControlledAgreement<'_>,
        frames: &[ObservedFallbackLeg],
        old_request: &[u8],
    ) -> TestResult<()> {
        assert_eq!(agreement.supporting_groups(), 3);
        let mut previous: Option<&[u8; 476]> = None;
        for (position, ((request, response), endpoint, transport)) in frames.iter().enumerate() {
            let receipt: &Receipt = agreement
                .receipt(position)
                .ok_or("missing fallback receipt")?;
            assert_eq!(receipt.peer(), *endpoint);
            assert_eq!(receipt.transport(), *transport);
            assert_eq!(receipt.request_frame(), request);
            assert_eq!(receipt.response_frame(), response);
            if let Some(predecessor) = previous {
                let salt: &[u8; 32] = receipt
                    .chaining_randomness()
                    .ok_or("missing fallback salt")?;
                assert_eq!(observed_nonce(request), Some(digest(&[predecessor, salt])));
            } else {
                assert!(receipt.chaining_randomness().is_none());
                assert_ne!(observed_nonce(request), observed_nonce(old_request));
            }
            previous = Some(response);
        }
        assert!(agreement.receipt(9).is_none());
        Ok(())
    }

    /// Verify that a received invalid answer cannot authorize a TCP continuation.
    ///
    /// # Errors
    /// Returns the original fixture failure or an unexpected round refusal.
    ///
    /// # Panics
    /// Panics if the invalid answer changes the refusal kind or triggers any TCP connection.
    #[test]
    fn malformed_udp_does_not_authorize_tcp_fallback() -> TestResult<()> {
        let servers: [Server; 3] = operators()?;
        let listener: TcpListener = fallback_listener(&servers)?;
        let tcp: SocketAddr = listener.local_addr()?;
        let aliases: [(UdpSocket, SocketAddr); 2] = [peer()?, peer()?];
        let mut bindings: [RetryBinding<'_>; 3] =
            fallback_bindings(&servers, tcp, [aliases[0].1, aliases[1].1])?;
        let calendar: LeapKnowledge<'static> = bundled_leap_knowledge()?;
        let authority: RecoveryControl = RecoveryControl::new();
        let controls: [RoundControl; 2] = [RoundControl::new(), RoundControl::new()];
        let peers: [RetryRoundPeer<'_>; 3] =
            bound_retry_roster(&mut bindings, &servers, Some(tcp))?;
        let mut recovery: ControlledRecovery<'_> = checked(ControlledRecovery::new_with_fallback(
            &authority,
            ROUND_BUDGET + ROUND_BUDGET,
            recovery_date_policy(&calendar)?,
            peers,
            UDP_WAIT,
        ))?;
        {
            let mut first: RecoveryRound<'_> =
                checked(recovery.begin_bound(&controls[0], ROUND_BUDGET, PROBE_BUDGET))?;
            let (_frames, progress): (SignedFrames, Result<ProbeProgress, RoundError>) =
                recovery_leg(&mut first, &servers[0], DATE_UNIX_SECONDS)?;
            assert_eq!(checked(progress)?, ProbeProgress::Accepted);
            assert_eq!(checked(first.start_next())?, servers[1].address);
            let (_request, sender): (Vec<u8>, SocketAddr) = receive_request(&servers[1].socket)?;
            let malformed: [u8; 4] = [0; 4];
            assert_eq!(
                servers[1].socket.send_to(&malformed, sender)?,
                malformed.len()
            );
            assert!(matches!(
                poll_recovery_to_settlement(&mut first)?,
                Err(RoundError::Exchange(ExchangeError::Verification(
                    VerifyError::Response(_)
                )))
            ));
            assert_eq!(first.status(), RoundStatus::Unavailable);
        }
        thread::sleep(FIRST_RETRY_WAIT);
        assert!(matches!(
            recovery.begin_bound(&controls[1], ROUND_BUDGET, PROBE_BUDGET),
            Err(RoundError::FallbackUnavailable)
        ));
        assert!(matches!(listener.accept(), Err(error) if error.kind() == ErrorKind::WouldBlock));
        Ok(())
    }

    /// Exercise scheduler admission once both real silence and the retry floor exist.
    ///
    /// # Errors
    /// Returns an unexpected fixture or scheduled recovery refusal.
    #[test]
    fn scheduled_fallback_waits_for_original_retry_floor() -> TestResult<()> {
        exercise_scheduled_fallback(ScheduledScenario::Due)
    }

    /// Stop a pending scheduler wait and verify original retry debt still refuses work.
    ///
    /// # Errors
    /// Returns an unexpected fixture, withdrawal or retry result.
    #[test]
    fn scheduled_fallback_stop_retires_pending_wait() -> TestResult<()> {
        exercise_scheduled_fallback(ScheduledScenario::Stop)
    }

    /// Delay polling after the first wait to exercise the already armed cutoff.
    ///
    /// # Errors
    /// Returns an unexpected fixture, deadline or scheduled recovery result.
    #[test]
    fn scheduled_fallback_preserves_armed_cutoff() -> TestResult<()> {
        exercise_scheduled_fallback(ScheduledScenario::LateWake)
    }

    /// Independent expected scheduler disposition after genuine UDP silence.
    #[derive(Clone, Copy)]
    enum ScheduledScenario {
        /// The exact previously observed retry floor becomes due.
        Due,
        /// An acknowledged invocation withdrawal wakes the original pending wait.
        Stop,
        /// The original wait next runs after its separate child cutoff.
        LateWake,
    }

    /// Supply the same signed prefix, native silence and exact floor to each case.
    ///
    /// # Errors
    /// Returns an unexpected fixture, scheduling or subsequent admission result.
    ///
    /// # Panics
    /// Panics if withdrawal discards the original debt or allows more traffic.
    fn exercise_scheduled_fallback(scenario: ScheduledScenario) -> TestResult<()> {
        let servers: [Server; 3] = operators()?;
        let listener: TcpListener = fallback_listener(&servers)?;
        let tcp: SocketAddr = listener.local_addr()?;
        let aliases: [(UdpSocket, SocketAddr); 2] = [peer()?, peer()?];
        let mut bindings: [RetryBinding<'_>; 3] =
            fallback_bindings(&servers, tcp, [aliases[0].1, aliases[1].1])?;
        let calendar: LeapKnowledge<'static> = bundled_leap_knowledge()?;
        let authority: RecoveryControl = RecoveryControl::new();
        let controls: [RoundControl; 2] = [RoundControl::new(), RoundControl::new()];
        let runtime: Runtime = Builder::new_current_thread().enable_time().build()?;
        let peers: [RetryRoundPeer<'_>; 3] =
            bound_retry_roster(&mut bindings, &servers, Some(tcp))?;
        let mut recovery: ControlledRecovery<'_> = checked(ControlledRecovery::new_with_fallback(
            &authority,
            ROUND_BUDGET + ROUND_BUDGET,
            recovery_date_policy(&calendar)?,
            peers,
            UDP_WAIT,
        ))?;
        let (first_request, until): (Vec<u8>, Instant) =
            observe_scheduled_silence(&mut recovery, &controls, &servers)?;
        match scenario {
            ScheduledScenario::Due => check_scheduled_start(
                &mut recovery,
                &controls[1],
                &servers,
                &listener,
                &runtime,
                &first_request,
                until,
            ),
            ScheduledScenario::Stop => {
                check_scheduled_stop(
                    &mut recovery,
                    &authority,
                    &controls[1],
                    &listener,
                    &runtime,
                    until,
                )?;
                drop(recovery);
                let probe: RoundAuthority = RoundAuthority::new();
                let refused: Result<PendingExchange<'_>, ExchangeError> =
                    PendingExchange::send_with_retry(
                        &probe,
                        &mut bindings[1],
                        servers[1].address,
                        PROBE_BUDGET,
                    );
                assert!(matches!(
                    refused,
                    Err(ExchangeError::RetryDeferred { until: retained }) if retained == until
                ));
                let mut packet: [u8; 1] = [0];
                assert!(
                    matches!(servers[1].socket.recv_from(&mut packet), Err(error) if error.kind() == ErrorKind::WouldBlock)
                );
                Ok(())
            }
            ScheduledScenario::LateWake => {
                check_scheduled_cutoff(&mut recovery, &controls[1], &listener, &runtime, until)
            }
        }
    }

    /// Establish the pending timer through one signed reply and a real empty UDP read.
    ///
    /// # Errors
    /// Returns an unexpected fixture or refusal result.
    ///
    /// # Panics
    /// Panics if the original reply, silence or retry floor differs from the independent fixture.
    fn observe_scheduled_silence(
        recovery: &mut ControlledRecovery<'_>,
        controls: &[RoundControl; 2],
        servers: &[Server; 3],
    ) -> TestResult<(Vec<u8>, Instant)> {
        let (request, terminal): (Vec<u8>, (Instant, Instant)) = {
            let mut first: RecoveryRound<'_> =
                checked(recovery.begin_bound(&controls[0], ROUND_BUDGET, PROBE_BUDGET))?;
            let ((request, _), progress): (SignedFrames, Result<ProbeProgress, RoundError>) =
                recovery_leg(&mut first, &servers[0], DATE_UNIX_SECONDS)?;
            assert_eq!(checked(progress)?, ProbeProgress::Accepted);
            assert_eq!(checked(first.start_next())?, servers[1].address);
            let (_request, _sender): (Vec<u8>, SocketAddr) = receive_request(&servers[1].socket)?;
            thread::sleep(UDP_WAIT);
            let before: Instant = Instant::now();
            assert!(matches!(
                first.poll(),
                Err(RoundError::Exchange(ExchangeError::Unanswered))
            ));
            (request, (before, Instant::now()))
        };
        let until: Instant = match recovery.begin_bound(&controls[1], ROUND_BUDGET, PROBE_BUDGET) {
            Err(RoundError::Exchange(ExchangeError::RetryDeferred { until })) => until,
            Err(error) => return Err(format!("unexpected initial retry result: {error:?}").into()),
            Ok(_) => return Err("fallback was ready before waiting for the original floor".into()),
        };
        assert_retry_wait(until, terminal, FIRST_RETRY_WAIT)?;
        Ok((request, until))
    }

    /// Observe Pending on the same future that will later resume.
    ///
    /// # Errors
    /// Returns an error if the future has already settled.
    fn require_scheduled_pending<F: Future>(
        runtime: &Runtime,
        mut waiter: Pin<&mut F>,
    ) -> TestResult<()> {
        runtime.block_on(async {
            tokio::select! {
                biased;
                _ = waiter.as_mut() => Err("scheduled fallback did not await the original floor".into()),
                () = ready(()) => Ok(()),
            }
        })
    }

    /// Compare the due round and all its replies with the independent fixture.
    ///
    /// # Errors
    /// Returns an unexpected fixture, admission or deadline result.
    ///
    /// # Panics
    /// Panics if the scheduler starts native work, renews the floor or alters the chain.
    fn check_scheduled_start(
        recovery: &mut ControlledRecovery<'_>,
        control: &RoundControl,
        servers: &[Server; 3],
        listener: &TcpListener,
        runtime: &Runtime,
        first_request: &[u8],
        until: Instant,
    ) -> TestResult<()> {
        let short: Duration = Duration::from_millis(10);
        assert!(matches!(
            runtime.block_on(recovery.begin_scheduled_fallback(control, short, short)),
            Err(RoundError::Expired)
        ));
        let waiter = recovery.begin_scheduled_fallback(control, ROUND_BUDGET, PROBE_BUDGET);
        tokio::pin!(waiter);
        require_scheduled_pending(runtime, waiter.as_mut())?;
        assert!(matches!(listener.accept(), Err(error) if error.kind() == ErrorKind::WouldBlock));
        let mut round: RecoveryRound<'_> = checked(runtime.block_on(waiter))?;
        assert!(Instant::now() >= until);
        assert!(matches!(listener.accept(), Err(error) if error.kind() == ErrorKind::WouldBlock));

        let frames: Vec<ObservedFallbackLeg> =
            drive_fallback_round(&mut round, servers, listener, Some(runtime))?;
        let date: ControlledDate<'_> = round
            .admit_date()
            .map_err(|error| format!("unexpected scheduled date refusal: {error:?}"))?;
        assert_fresh_fallback_chain(date.agreement(), &frames, first_request)
    }

    /// Deliver Stop only after the wait has had a chance to enroll its wakeup.
    ///
    /// # Errors
    /// Returns an unexpected fixture or withdrawal result.
    ///
    /// # Panics
    /// Panics if Stop fails to wake the waiter before the floor, or allows another round.
    fn check_scheduled_stop(
        recovery: &mut ControlledRecovery<'_>,
        authority: &RecoveryControl,
        control: &RoundControl,
        listener: &TcpListener,
        runtime: &Runtime,
        until: Instant,
    ) -> TestResult<()> {
        let (stopped, withdrew): (
            Result<Result<RecoveryRound<'_>, RoundError>, Elapsed>,
            TestResult<()>,
        ) = runtime.block_on(async {
            let waiter = recovery.begin_scheduled_fallback(control, ROUND_BUDGET, PROBE_BUDGET);
            let stop = async {
                tokio::task::yield_now().await;
                assert!(Instant::now() < until);
                assert!(
                    matches!(listener.accept(), Err(error) if error.kind() == ErrorKind::WouldBlock)
                );
                checked(authority.withdraw(Withdrawal::Stop))
            };
            tokio::join! {
                biased;
                tokio::time::timeout_at(TokioInstant::from_std(until), waiter),
                stop,
            }
        });
        withdrew?;
        assert!(matches!(
            stopped,
            Ok(Err(RoundError::Withdrawn(Withdrawal::Stop)))
        ));
        assert!(Instant::now() < until);
        drop(stopped);
        assert!(matches!(
            recovery.begin_bound(control, ROUND_BUDGET, PROBE_BUDGET),
            Err(RoundError::Withdrawn(Withdrawal::Stop))
        ));
        assert!(matches!(listener.accept(), Err(error) if error.kind() == ErrorKind::WouldBlock));
        Ok(())
    }

    /// Resume the original wait after its child expiry and check the unspent slot.
    ///
    /// # Errors
    /// Returns an unexpected fixture, deadline or subsequent admission refusal.
    ///
    /// # Panics
    /// Panics if late polling renews the cutoff or opens a TCP connection.
    fn check_scheduled_cutoff(
        recovery: &mut ControlledRecovery<'_>,
        control: &RoundControl,
        listener: &TcpListener,
        runtime: &Runtime,
        until: Instant,
    ) -> TestResult<()> {
        let child_budget: Duration =
            until.saturating_duration_since(Instant::now()) + Duration::from_millis(50);
        {
            let waiter = recovery.begin_scheduled_fallback(control, child_budget, child_budget);
            tokio::pin!(waiter);
            require_scheduled_pending(runtime, waiter.as_mut())?;
            // The first poll has already fixed the child deadline. Waiting one
            // more full child budget outside the runtime is beyond that deadline.
            thread::sleep(child_budget);
            assert!(matches!(runtime.block_on(waiter), Err(RoundError::Expired)));
        }
        assert!(matches!(listener.accept(), Err(error) if error.kind() == ErrorKind::WouldBlock));
        let next: RecoveryRound<'_> =
            checked(runtime.block_on(recovery.begin_scheduled_fallback(
                control,
                ROUND_BUDGET,
                PROBE_BUDGET,
            )))?;
        assert_eq!(next.status(), RoundStatus::Active);
        assert!(matches!(listener.accept(), Err(error) if error.kind() == ErrorKind::WouldBlock));
        Ok(())
    }

    /// Retain a contradictory prefix with separate operator and clock groups.
    ///
    /// # Errors
    /// Returns the first fixture or unexpected recovery error.
    ///
    /// # Panics
    /// Panics if the actual signed sequence settles at a different pair or keeps sending.
    fn first_recovery_conflict(
        recovery: &mut ControlledRecovery<'_>,
        servers: &[Server; 3],
    ) -> TestResult<Vec<SignedFrames>> {
        let mut peers: [RoundPeer<'_>; 3] = roster(servers)?;
        for (index, (operator, clock)) in [(1, 31), (2, 32), (3, 33)].into_iter().enumerate() {
            let operator: NonZeroU64 = NonZeroU64::new(operator).ok_or("zero fixture operator")?;
            let clock: NonZeroU64 = NonZeroU64::new(clock).ok_or("zero fixture clock")?;
            peers[index] = RoundPeer::new(
                servers[index].address,
                &servers[index].root,
                operator,
                clock,
            );
        }
        let cutoff_control: RoundControl = RoundControl::new();
        let mut round: RecoveryRound<'_> =
            checked(recovery.begin(&cutoff_control, peers, ROUND_BUDGET, PROBE_BUDGET))?;
        let mut observed: Vec<SignedFrames> = Vec::with_capacity(3);
        for (position, midpoint) in [1_020, 1_010, 1_009].into_iter().enumerate() {
            let (frames, progress): (SignedFrames, Result<ProbeProgress, RoundError>) =
                recovery_leg(&mut round, &servers[position], midpoint)?;
            if position < 2 {
                assert_eq!(checked(progress)?, ProbeProgress::Accepted);
            } else {
                assert!(
                    matches!(progress, Err(RoundError::Conflict(conflict)) if conflict.positions() == (0, 2))
                );
            }
            observed.push(frames);
        }
        assert!(
            matches!(round.status(), RoundStatus::Conflict(conflict) if conflict.positions() == (0, 2))
        );
        checked(cutoff_control.withdraw(Withdrawal::Stop))?;
        assert!(matches!(round.start_next(), Err(RoundError::Finished)));
        Ok(observed)
    }

    /// Compare the received frames, intermediary chain and original roster independently.
    ///
    /// # Errors
    /// Returns a missing historical record or chain input.
    ///
    /// # Panics
    /// Panics if the signed conflict no longer identifies the original controlled attempt.
    fn assert_original_recovery_conflict(
        recovery: &ControlledRecovery<'_>,
        servers: &[Server; 3],
        observed: &[SignedFrames],
    ) -> TestResult<()> {
        let history: &ConflictEvidence =
            recovery.first_conflict().ok_or("missing signed conflict")?;
        assert_eq!(history.conflict().positions(), (0, 2));
        assert_eq!(history.receipts().len(), 3);
        assert_eq!(history.max_delay(), PROBE_BUDGET);
        assert!(history.selected_peer(3).is_none());
        let mut predecessor: Option<&[u8; 476]> = None;
        for (index, ((receipt, (request, response)), (operator, clock))) in history
            .receipts()
            .iter()
            .zip(observed)
            .zip([(1, 31), (2, 32), (3, 33)])
            .enumerate()
        {
            let selected: SelectedPeer = history
                .selected_peer(index)
                .ok_or("missing selected peer")?;
            assert_eq!(selected.endpoint(), servers[index].address);
            assert_eq!(
                selected.root_key_bytes(),
                servers[index].trust_anchor.verifying_key().as_bytes()
            );
            assert_eq!(selected.operator().get(), operator);
            assert_eq!(selected.clock().get(), clock);
            assert_eq!(receipt.peer(), selected.endpoint());
            assert_eq!(receipt.root_key_bytes(), selected.root_key_bytes());
            assert_eq!(receipt.request_frame(), request.as_slice());
            assert_eq!(receipt.response_frame(), response.as_slice());
            if let Some(prior) = predecessor {
                let salt: &[u8; 32] = receipt.chaining_randomness().ok_or("missing chain salt")?;
                assert_eq!(observed_nonce(request), Some(digest(&[prior, salt])));
            } else {
                assert!(receipt.chaining_randomness().is_none());
            }
            predecessor = Some(response);
        }
        Ok(())
    }

    /// Observe a real follow-up send and expiry under the original aggregate cutoff.
    ///
    /// # Errors
    /// Returns the first fixture, clock, or unexpected recovery failure.
    ///
    /// # Panics
    /// Panics if the follow-up renews the deadline or loses the original signed chain.
    #[test]
    fn recovery_retains_original_chain_after_followup_timeout() -> TestResult<()> {
        let servers: [Server; 3] = operators()?;
        let aggregate: Duration = Duration::from_millis(1_500);
        let recovery_control: RecoveryControl = RecoveryControl::new();
        let calendar: LeapKnowledge<'static> = bundled_leap_knowledge()?;
        let policy: RecoveryDatePolicy<'_> = recovery_date_policy(&calendar)?;
        let earliest: Instant = Instant::now()
            .checked_add(aggregate)
            .ok_or("cutoff overflow")?;
        let mut recovery: ControlledRecovery<'_> = checked(ControlledRecovery::new(
            &recovery_control,
            aggregate,
            policy,
        ))?;
        let latest: Instant = Instant::now()
            .checked_add(aggregate)
            .ok_or("cutoff overflow")?;
        let observed: Vec<SignedFrames> = first_recovery_conflict(&mut recovery, &servers)?;
        let initial_cutoff: Instant = recovery
            .first_conflict()
            .ok_or("missing signed cutoff")?
            .cutoff();
        assert!((earliest..=latest).contains(&initial_cutoff));

        {
            let control: RoundControl = RoundControl::new();
            let mut followup: RecoveryRound<'_> =
                checked(recovery.begin(&control, roster(&servers)?, ROUND_BUDGET, PROBE_BUDGET))?;
            assert_eq!(checked(followup.start_next())?, servers[0].address);
            let (request, _sender): (Vec<u8>, SocketAddr) = receive_request(&servers[0].socket)?;
            assert!(observed_nonce(&request).is_some());
            thread::sleep(aggregate);
            assert!(matches!(
                followup.poll(),
                Err(RoundError::Exchange(ExchangeError::Expired))
            ));
            assert_eq!(followup.status(), RoundStatus::Unavailable);
            assert!(matches!(followup.start_next(), Err(RoundError::Finished)));
        }
        assert_original_recovery_conflict(&recovery, &servers, &observed)?;
        assert_eq!(
            recovery.first_conflict().map(ConflictEvidence::cutoff),
            Some(initial_cutoff)
        );
        Ok(())
    }

    /// Drive the matched nine-leg route in sessions that differ by a prior conflict.
    ///
    /// # Errors
    /// Returns fixture, calendar or unexpected recovery failure.
    ///
    /// # Panics
    /// Panics if prior signed conflict is cleared by an otherwise usable date.
    #[test]
    fn recovery_conflict_refuses_an_otherwise_healthy_date() -> TestResult<()> {
        let servers: [Server; 3] = operators()?;
        let calendar: LeapKnowledge<'static> = bundled_leap_knowledge()?;
        let aggregate: Duration = Duration::from_secs(18);
        for prior_conflict in [false, true] {
            let recovery_control: RecoveryControl = RecoveryControl::new();
            let policy: RecoveryDatePolicy<'_> = recovery_date_policy(&calendar)?;
            let mut recovery: ControlledRecovery<'_> = checked(ControlledRecovery::new(
                &recovery_control,
                aggregate,
                policy,
            ))?;
            let observed: Option<Vec<SignedFrames>> = if prior_conflict {
                Some(first_recovery_conflict(&mut recovery, &servers)?)
            } else {
                None
            };
            {
                let control: RoundControl = RoundControl::new();
                let mut round: RecoveryRound<'_> = checked(recovery.begin(
                    &control,
                    roster(&servers)?,
                    ROUND_BUDGET,
                    PROBE_BUDGET,
                ))?;
                complete_recovery_date_round(&mut round, &servers)?;
                let result: Result<ControlledDate<'_>, IntervalError> = round.evaluate_date();
                if prior_conflict {
                    assert!(
                        matches!(result, Err(IntervalError::Conflict(conflict)) if conflict.positions() == (0, 2))
                    );
                } else {
                    let date: ControlledDate<'_> = result
                        .map_err(|error| format!("healthy recovery date refused: {error:?}"))?;
                    let dates: EraWindow = date.interpretation().dates();
                    let midpoint: i128 = DATE_NTP_SECONDS * Q32_SECOND;
                    assert!(
                        dates.earliest().ticks() < midpoint && dates.latest().ticks() > midpoint
                    );
                    assert_eq!(date.agreement().supporting_groups(), 3);
                }
            }
            if let Some(original) = observed {
                assert_original_recovery_conflict(&recovery, &servers, &original)?;
                let extra: RoundControl = RoundControl::new();
                assert!(matches!(
                    recovery.begin(&extra, roster(&servers)?, ROUND_BUDGET, PROBE_BUDGET),
                    Err(RoundError::Finished)
                ));
            } else {
                assert!(recovery.first_conflict().is_none());
            }
        }
        Ok(())
    }

    /// A distinct second signature contradiction must not replace the first witness.
    ///
    /// # Errors
    /// Returns the first fixture or unexpected recovery failure.
    ///
    /// # Panics
    /// Panics if the latest conflict is hidden or recorded over the original chain.
    #[test]
    fn recovery_keeps_first_conflict_after_a_distinct_second_conflict() -> TestResult<()> {
        let servers: [Server; 3] = operators()?;
        let recovery_control: RecoveryControl = RecoveryControl::new();
        let calendar: LeapKnowledge<'static> = bundled_leap_knowledge()?;
        let policy: RecoveryDatePolicy<'_> = recovery_date_policy(&calendar)?;
        let mut recovery: ControlledRecovery<'_> = checked(ControlledRecovery::new(
            &recovery_control,
            Duration::from_secs(18),
            policy,
        ))?;
        let original: Vec<SignedFrames> = first_recovery_conflict(&mut recovery, &servers)?;
        {
            let control: RoundControl = RoundControl::new();
            let mut round: RecoveryRound<'_> =
                checked(recovery.begin(&control, roster(&servers)?, ROUND_BUDGET, PROBE_BUDGET))?;
            let (_first, progress): (SignedFrames, Result<ProbeProgress, RoundError>) =
                recovery_leg(&mut round, &servers[0], 1_040)?;
            assert_eq!(checked(progress)?, ProbeProgress::Accepted);
            let (_second, progress): (SignedFrames, Result<ProbeProgress, RoundError>) =
                recovery_leg(&mut round, &servers[1], 1_020)?;
            assert!(
                matches!(progress, Err(RoundError::Conflict(conflict)) if conflict.positions() == (0, 1))
            );
            assert!(
                matches!(round.status(), RoundStatus::Conflict(conflict) if conflict.positions() == (0, 1))
            );
        }
        assert_original_recovery_conflict(&recovery, &servers, &original)?;
        Ok(())
    }

    /// A queued valid response cannot outlive the invocation's current authority.
    ///
    /// # Errors
    /// Returns the first fixture or unexpected recovery error.
    ///
    /// # Panics
    /// Panics if the withdrawn invocation advances or the healthy control fails.
    #[test]
    fn recovery_withdrawal_refuses_queued_reply_and_successor() -> TestResult<()> {
        let servers: [Server; 3] = operators()?;
        let calendar: LeapKnowledge<'static> = bundled_leap_knowledge()?;
        for withdraw in [true, false] {
            let authority: RecoveryControl = RecoveryControl::new();
            let policy: RecoveryDatePolicy<'_> = recovery_date_policy(&calendar)?;
            let mut recovery: ControlledRecovery<'_> =
                checked(ControlledRecovery::new(&authority, ROUND_BUDGET, policy))?;
            {
                let control: RoundControl = RoundControl::new();
                let mut round: RecoveryRound<'_> = checked(recovery.begin(
                    &control,
                    roster(&servers)?,
                    ROUND_BUDGET,
                    PROBE_BUDGET,
                ))?;
                assert_eq!(checked(round.start_next())?, servers[0].address);
                let _frames: SignedFrames = servers[0].respond(DATE_UNIX_SECONDS)?;
                if withdraw {
                    checked(authority.withdraw(Withdrawal::Stop))?;
                    assert!(matches!(
                        round.poll(),
                        Err(RoundError::Withdrawn(Withdrawal::Stop))
                    ));
                    assert_eq!(round.status(), RoundStatus::Unavailable);
                } else {
                    assert_eq!(
                        checked(poll_recovery_to_settlement(&mut round)?)?,
                        ProbeProgress::Accepted
                    );
                }
            }
            {
                let successor: RoundControl = RoundControl::new();
                let result: Result<RecoveryRound<'_>, RoundError> =
                    recovery.begin(&successor, roster(&servers)?, ROUND_BUDGET, PROBE_BUDGET);
                if withdraw {
                    assert!(matches!(
                        result,
                        Err(RoundError::Withdrawn(Withdrawal::Stop))
                    ));
                } else {
                    let round: RecoveryRound<'_> = checked(result)?;
                    assert_eq!(round.status(), RoundStatus::Active);
                }
            }
            assert!(recovery.first_conflict().is_none());
        }
        Ok(())
    }

    /// Recheck authorization when an already complete round is used for its date.
    ///
    /// # Errors
    /// Returns the first fixture or unexpected recovery error.
    ///
    /// # Panics
    /// Panics if withdrawn clock evidence stays usable or an authority is reused.
    #[test]
    fn recovery_clock_frame_withdrawal_refuses_a_completed_date() -> TestResult<()> {
        let servers: [Server; 3] = operators()?;
        let calendar: LeapKnowledge<'static> = bundled_leap_knowledge()?;
        let authority: RecoveryControl = RecoveryControl::new();
        let policy: RecoveryDatePolicy<'_> = recovery_date_policy(&calendar)?;
        let mut recovery: ControlledRecovery<'_> =
            checked(ControlledRecovery::new(&authority, ROUND_BUDGET, policy))?;
        assert!(matches!(
            ControlledRecovery::new(&authority, ROUND_BUDGET, recovery_date_policy(&calendar)?),
            Err(RoundError::Finished)
        ));
        let control: RoundControl = RoundControl::new();
        let mut round: RecoveryRound<'_> =
            checked(recovery.begin(&control, roster(&servers)?, ROUND_BUDGET, PROBE_BUDGET))?;
        complete_recovery_date_round(&mut round, &servers)?;
        let date: ControlledDate<'_> = round
            .evaluate_date()
            .map_err(|error| format!("current recovery date refused: {error:?}"))?;
        assert_eq!(date.agreement().supporting_groups(), 3);
        checked(authority.withdraw(Withdrawal::ClockFrame))?;
        assert_eq!(round.status(), RoundStatus::Complete);
        assert!(matches!(
            round.evaluate_date(),
            Err(IntervalError::Withdrawn(Withdrawal::ClockFrame))
        ));
        Ok(())
    }
}

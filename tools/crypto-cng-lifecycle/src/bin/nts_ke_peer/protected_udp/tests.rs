// SPDX-License-Identifier: GPL-2.0-only
//! Real socket controls using public fixture values in a private loopback namespace.

use std::cell::Cell;
use std::io::ErrorKind;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use aes_siv::{KeyInit, siv::Aes128Siv};
use nevrotick_s115_aes_siv_fixture::sealing::{
    PreparedAlgorithm15Request, prepare_algorithm15_request,
};
use ntp_wire::{
    FrameLimits, NtsClientRequest, NtsKeFrameLimits, NtsKeNtpv4Negotiation, NtsKeNtpv4Offer,
    RawHeader, RawNtsKeMessage, decode_fixed_header, frame_nts_ke_message, negotiate_nts_ke_ntpv4,
    read_nts_ke_ntpv4_offer,
};
use zeroize::Zeroizing;

use super::super::{OFFER, protected_response};
use super::{Budget, ClientExit, Failure, PacketFailure, ProtectedUdp, SuccessorReply};

/// Public request authentication key for the controlled socket tests.
const C2S: [u8; 32] = [0x51; 32];
/// Separate public response authentication key.
const S2C: [u8; 32] = [0xbc; 32];
/// Nonzero request transmit field to echo as the response's origin.
const TRANSMIT: [u8; 8] = [0x01, 0x32, 0x54, 0x76, 0x98, 0xba, 0xdc, 0xfe];
/// One controlled client correlation identifier.
const UID: [u8; 32] = [0x86; 32];
/// Public initial cookie expected by the standalone KE peer.
const COOKIE: [u8; 12] = [
    0xaa, 0xbb, 0xcc, 0xdd, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80,
];
/// Fixed public nonce for checking the supplied response nonce path.
const SERVER_NONCE: [u8; 16] = [0x93; 16];

/// Closed diagnostic labels for failed test setup or reads.
type TestResult<T = ()> = Result<T, &'static str>;

/// Prepare the exact 252-byte shape with the Engine's underlying request formatter.
///
/// # Errors
/// Preserve a failure in fixture encoding, bounds or authentication setup.
fn request() -> TestResult<[u8; 252]> {
    request_with(TRANSMIT, &UID, &COOKIE, &[0x34; 16])
}

/// Use the actual client encoder for a second cookie and correlation.
///
/// # Errors
/// Preserve fixture header, limits, sealing and output-extent refusals.
fn request_with(
    transmit: [u8; 8],
    uid: &[u8; 32],
    cookie: &[u8; 12],
    nonce: &[u8; 16],
) -> TestResult<[u8; 252]> {
    let mut header_bytes: [u8; 48] = [0; 48];
    header_bytes[0] = 0x23;
    header_bytes[40..48].copy_from_slice(&transmit);
    let header: RawHeader = decode_fixed_header(&header_bytes).map_err(|_| "fixture-header")?;
    let limits: FrameLimits = FrameLimits::new(252, 10).map_err(|_| "fixture-limits")?;
    let mut storage: [u8; 252] = [0; 252];
    let prepared: PreparedAlgorithm15Request<'_> =
        prepare_algorithm15_request(&header, uid, cookie, 7, &mut storage, limits)
            .map_err(|_| "fixture-prepare")?;
    let sealed: NtsClientRequest<'_> = prepared.seal(&C2S, nonce).map_err(|_| "fixture-seal")?;
    sealed
        .packet_bytes()
        .try_into()
        .map_err(|_| "fixture-length")
}

/// Prepare the sole client socket with a bounded verification read.
///
/// # Errors
/// Refuse failure to bind or configure the private receiver.
fn client() -> TestResult<UdpSocket> {
    let client: UdpSocket =
        UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0)).map_err(|_| "client-bind")?;
    client
        .set_read_timeout(Some(Duration::from_millis(100)))
        .map_err(|_| "client-timeout")?;
    Ok(client)
}

/// Send test input exactly once before executing the owner's bounded receive.
///
/// # Errors
/// Refuse a failed or partial fixture send.
fn send(client: &UdpSocket, bytes: &[u8], endpoint: SocketAddrV4) -> TestResult {
    let sent: usize = client.send_to(bytes, endpoint).map_err(|_| "client-send")?;
    if sent != bytes.len() {
        return Err("client-short-send");
    }
    Ok(())
}

/// Observe a bounded interval in which the receiver obtains no further datagram.
///
/// # Panics
/// Panic if a datagram is delivered or the socket fails for another reason.
fn assert_no_reply(client: &UdpSocket) {
    let mut data: [u8; 141] = [0; 141];
    assert!(matches!(
        client.recv_from(&mut data),
        Err(error) if matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut)
    ));
}

/// Verify the initial reply and return its literal replacement cookie.
///
/// # Errors
/// Preserve an invalid shape or a failure to open the received bytes.
///
/// # Panics
/// Panic if the reply fails to carry the expected correlation or cookie.
fn check_reply(reply: &[u8; 140]) -> TestResult<[u8; 12]> {
    let cookie: [u8; 12] = open_ordinary(reply, TRANSMIT, &UID, &SERVER_NONCE)?;
    assert_eq!(
        cookie,
        [
            0x83, 0x73, 0x63, 0x53, 0x43, 0x33, 0x23, 0x13, 0x91, 0x92, 0x93, 0x94
        ]
    );
    Ok(cookie)
}

/// Open the whole ordinary reply against independently selected correlation and nonce.
///
/// # Errors
/// Refuse invalid tag or cookie extents or a directional opening failure.
///
/// # Panics
/// Panic if the envelope, correlation or opened cookie field differs.
fn open_ordinary(
    reply: &[u8; 140],
    origin: [u8; 8],
    uid: &[u8; 32],
    nonce: &[u8; 16],
) -> TestResult<[u8; 12]> {
    assert_eq!(&reply[..2], &[0x24, 2]);
    assert_eq!(&reply[24..32], origin);
    assert_eq!(&reply[48..52], &[1, 4, 0, 36]);
    assert_eq!(&reply[52..84], uid);
    assert_eq!(&reply[84..92], &[4, 4, 0, 56, 0, 16, 0, 32]);
    assert_eq!(&reply[92..108], nonce);

    let tag: &[u8; 16] = reply[108..124].try_into().map_err(|_| "reply-tag")?;
    let mut body: [u8; 16] = reply[124..].try_into().map_err(|_| "reply-body")?;
    Aes128Siv::new((&S2C).into())
        .decrypt_inout_detached(
            [&reply[..84], &reply[92..108]],
            (&mut body[..]).into(),
            tag.into(),
        )
        .map_err(|_| "reply-authentication")?;
    assert_eq!(&body[..4], &[2, 4, 0, 16]);
    body[4..].try_into().map_err(|_| "replacement-size")
}

/// The peer responds once from its advertised port and releases its socket.
///
/// # Errors
/// Preserve setup, send, exchange, receive, framing or release failures.
///
/// # Panics
/// Panic if the selected source, correlation or reply count differs.
#[test]
fn protected_udp_replies_once_from_advertised_port() -> TestResult {
    let budget: Budget = Budget::new(Instant::now()).map_err(|_| "fixture-budget")?;
    let server: ProtectedUdp = ProtectedUdp::bind(&budget).map_err(|_| "server-bind")?;
    let limits: NtsKeFrameLimits = NtsKeFrameLimits::new(38, 5).map_err(|_| "ke-limits")?;
    let offer_frame: RawNtsKeMessage<'_> =
        frame_nts_ke_message(&OFFER, limits).map_err(|_| "ke-offer-frame")?;
    let offer: NtsKeNtpv4Offer<'_> =
        read_nts_ke_ntpv4_offer(&offer_frame).map_err(|_| "ke-offer")?;
    let response: [u8; 38] = protected_response(server.port());
    let response_frame: RawNtsKeMessage<'_> =
        frame_nts_ke_message(&response, limits).map_err(|_| "ke-response-frame")?;
    let negotiation: NtsKeNtpv4Negotiation<'_> =
        negotiate_nts_ke_ntpv4(&offer, response_frame).map_err(|_| "ke-negotiation")?;
    let endpoint: SocketAddrV4 = SocketAddrV4::new(
        Ipv4Addr::LOCALHOST,
        negotiation.port_hint().ok_or("ke-port")?,
    );
    assert_eq!(endpoint.port(), server.port().get());
    let client: UdpSocket = client()?;
    let packet: [u8; 252] = request()?;
    send(&client, &packet, endpoint)?;
    send(&client, &packet, endpoint)?;

    server
        .exchange(&budget, &C2S, &S2C, |nonce| {
            *nonce = SERVER_NONCE;
            Ok(())
        })
        .map_err(|_| "server-exchange")?;
    let mut buffer: [u8; 141] = [0; 141];
    let (length, sender): (usize, SocketAddr) = client
        .recv_from(&mut buffer)
        .map_err(|_| "client-receive")?;
    assert_eq!(length, 140);
    assert_eq!(sender, SocketAddr::V4(endpoint));
    let reply: &[u8; 140] = buffer[..length].try_into().map_err(|_| "reply-size")?;
    let _replacement: [u8; 12] = check_reply(reply)?;
    assert_no_reply(&client);

    let recovered: UdpSocket = UdpSocket::bind(endpoint).map_err(|_| "socket-still-held")?;
    drop(recovered);
    Ok(())
}

/// A forged first datagram prevents replies even with a valid neighbor queued.
///
/// # Errors
/// Preserve setup and fixture sends.
///
/// # Panics
/// Panic if the first packet fails to terminate the exchange or any reply arrives.
#[test]
fn protected_udp_rejects_tamper_and_does_not_read_valid_neighbor() -> TestResult {
    let budget: Budget = Budget::new(Instant::now()).map_err(|_| "fixture-budget")?;
    let server: ProtectedUdp = ProtectedUdp::bind(&budget).map_err(|_| "server-bind")?;
    let endpoint: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, server.port().get());
    let client: UdpSocket = client()?;
    let packet: [u8; 252] = request()?;
    let mut tampered: [u8; 252] = packet;
    tampered[251] ^= 1;
    send(&client, &tampered, endpoint)?;
    send(&client, &packet, endpoint)?;

    assert_eq!(
        server.exchange(&budget, &C2S, &S2C, |nonce| {
            *nonce = SERVER_NONCE;
            Ok(())
        }),
        Err(Failure::Datagram(PacketFailure::Authentication))
    );
    assert_no_reply(&client);
    Ok(())
}

/// A larger datagram with an exact authenticated prefix fails before nonce use.
///
/// # Errors
/// Preserve setup, fixture construction and send failures.
///
/// # Panics
/// Panic if truncation admits the prefix or if the peer publishes any reply.
#[test]
fn protected_udp_rejects_overlong_authenticated_prefix() -> TestResult {
    let budget: Budget = Budget::new(Instant::now()).map_err(|_| "fixture-budget")?;
    let server: ProtectedUdp = ProtectedUdp::bind(&budget).map_err(|_| "server-bind")?;
    let endpoint: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, server.port().get());
    let client: UdpSocket = client()?;
    let mut overlong: [u8; 254] = [0; 254];
    overlong[..252].copy_from_slice(&request()?);
    send(&client, &overlong, endpoint)?;

    let attempted_fill: Cell<bool> = Cell::new(false);
    let result: Result<(), Failure> = server.exchange(&budget, &C2S, &S2C, |_| {
        attempted_fill.set(true);
        Err(Failure::Entropy)
    });
    assert_eq!(result, Err(Failure::Datagram(PacketFailure::Length)));
    assert!(!attempted_fill.get());
    assert_no_reply(&client);
    Ok(())
}

/// Reaching the original deadline during nonce work prevents the send.
///
/// # Errors
/// Preserve setup, fixture construction and send failures.
///
/// # Panics
/// Panic if expiration is mishandled or the peer publishes a datagram.
#[test]
fn protected_udp_refuses_at_original_deadline_after_entropy() -> TestResult {
    let origin: Instant = Instant::now();
    let budget: Budget = Budget::new(origin).map_err(|_| "fixture-budget")?;
    let server: ProtectedUdp = ProtectedUdp::bind(&budget).map_err(|_| "server-bind")?;
    let endpoint: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, server.port().get());
    let client: UdpSocket = client()?;
    send(&client, &request()?, endpoint)?;

    let observed: Cell<Instant> = Cell::new(origin);
    let result: Result<(), Failure> = server.exchange_with_clock(
        &budget,
        &C2S,
        &S2C,
        |nonce| {
            *nonce = SERVER_NONCE;
            observed.set(budget.deadline);
            Ok(())
        },
        || observed.get().max(Instant::now()),
    );
    assert_eq!(observed.get(), budget.deadline);
    assert_eq!(result, Err(Failure::Deadline));
    assert_no_reply(&client);
    Ok(())
}

/// A later budget cannot extend the lifetime of an already advertised port.
///
/// # Errors
/// Preserve setup, fixture construction and send failures.
/// # Panics
/// Fail if the different budget allows nonce work or a response.
#[test]
fn protected_udp_refuses_replacement_deadline() -> TestResult {
    let origin: Instant = Instant::now();
    let budget: Budget = Budget::new(origin).map_err(|_| "fixture-budget")?;
    let later: Instant = origin.checked_add(Duration::from_secs(1)).ok_or("later")?;
    let replacement: Budget = Budget::new(later).map_err(|_| "replacement-budget")?;
    let server: ProtectedUdp = ProtectedUdp::bind(&budget).map_err(|_| "server-bind")?;
    let endpoint: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, server.port().get());
    let client: UdpSocket = client()?;
    send(&client, &request()?, endpoint)?;

    let attempted_fill: Cell<bool> = Cell::new(false);
    let result: Result<(), Failure> = server.exchange(&replacement, &C2S, &S2C, |_| {
        attempted_fill.set(true);
        Err(Failure::Entropy)
    });
    assert_eq!(result, Err(Failure::Deadline));
    assert!(!attempted_fill.get());
    assert_no_reply(&client);
    Ok(())
}

/// A second client port uses the first reply's cookie and receives a bound DENY.
///
/// # Errors
/// Preserve socket, scoped worker, packet and response-opening failures.
/// # Panics
/// Fail if the endpoint, correlation, envelope, response count or nonce use differs.
#[test]
fn protected_udp_successor_uses_issued_cookie_from_another_client_port() -> TestResult {
    let now: Instant = Instant::now();
    let mut budget: Budget = Budget::new(now).map_err(|_| "fixture-budget")?;
    budget.deadline = now
        .checked_add(Duration::from_secs(5))
        .ok_or("fixture-deadline")?;
    let server: ProtectedUdp = ProtectedUdp::bind(&budget).map_err(|_| "server-bind")?;
    let endpoint: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, server.port().get());
    let first_client: UdpSocket = client()?;
    let successor_client: UdpSocket = client()?;
    assert_ne!(
        first_client.local_addr().map_err(|_| "first-address")?,
        successor_client
            .local_addr()
            .map_err(|_| "successor-address")?
    );
    for client_socket in [&first_client, &successor_client] {
        client_socket
            .set_read_timeout(Some(Duration::from_secs(2)))
            .map_err(|_| "client-timeout")?;
    }
    let fresh_origin: [u8; 8] = 0x1122_3344_eedd_ccbb_u64.to_be_bytes();
    let fresh_uid: [u8; 32] = [0x23; 32];
    let deny_nonce: [u8; 16] = [0x77; 16];
    std::thread::scope(|scope| -> TestResult {
        let peer: std::thread::ScopedJoinHandle<'_, TestResult> = scope.spawn(move || {
            let mut nonces: std::array::IntoIter<[u8; 16], 2> =
                [SERVER_NONCE, deny_nonce].into_iter();
            let result: Result<(), Failure> =
                server.exchange_successor_deny(&budget, &C2S, &S2C, |nonce: &mut [u8; 16]| {
                    *nonce = nonces.next().ok_or(Failure::Entropy)?;
                    Ok(())
                });
            result.map_err(|_| "peer-exchange")?;
            if nonces.next().is_some() {
                return Err("unspent-fixture-nonce");
            }
            Ok(())
        });
        send(&first_client, &request()?, endpoint)?;
        let mut buffer: [u8; 141] = [0; 141];
        let (length, sender): (usize, SocketAddr) = first_client
            .recv_from(&mut buffer)
            .map_err(|_| "first-receive")?;
        assert_eq!((length, sender), (140, SocketAddr::V4(endpoint)));
        let first_reply: &[u8; 140] = buffer[..length].try_into().map_err(|_| "reply-length")?;
        let cookie: [u8; 12] = check_reply(first_reply)?;
        assert_ne!(cookie, COOKIE);
        let successor: [u8; 252] = request_with(fresh_origin, &fresh_uid, &cookie, &[0x69; 16])?;
        send(&successor_client, &successor, endpoint)?;
        let (length, sender): (usize, SocketAddr) = successor_client
            .recv_from(&mut buffer)
            .map_err(|_| "deny-receive")?;
        assert_eq!((length, sender), (124, SocketAddr::V4(endpoint)));
        let reply: &[u8] = &buffer[..length];
        assert_eq!(&reply[..2], &[0x24, 0]);
        assert_eq!(&reply[12..16], b"DENY");
        assert_eq!(&reply[24..32], &fresh_origin);
        assert_eq!(&reply[48..52], &[1, 4, 0, 36]);
        assert_eq!(&reply[52..84], &fresh_uid);
        assert_eq!(&reply[84..92], &[4, 4, 0, 40, 0, 16, 0, 16]);
        assert_eq!(&reply[92..108], &deny_nonce);
        let tag: &[u8; 16] = reply[108..124].try_into().map_err(|_| "deny-tag")?;
        Aes128Siv::new((&S2C).into())
            .decrypt_inout_detached(
                [&reply[..84], &reply[92..108]],
                (&mut [][..]).into(),
                tag.into(),
            )
            .map_err(|_| "deny-authentication")?;
        peer.join().map_err(|_| "peer-panic")??;
        Ok(())
    })?;

    for client_socket in [&first_client, &successor_client] {
        client_socket
            .set_read_timeout(Some(Duration::from_millis(100)))
            .map_err(|_| "client-timeout")?;
        assert_no_reply(client_socket);
    }
    let recovered: UdpSocket = UdpSocket::bind(endpoint).map_err(|_| "socket-still-held")?;
    drop(recovered);
    Ok(())
}

/// One protected request earns one authenticated cookie-free DENY before client exit.
///
/// # Errors
/// Preserve local socket setup, exchange or response-opening failures.
/// # Panics
/// Fail on changed correlation, response count, observation or key retirement.
#[test]
fn protected_udp_initial_deny_replies_once_and_observes_client_exit() -> TestResult {
    let budget: Budget = Budget::new(Instant::now()).map_err(|_| "fixture-budget")?;
    let server: ProtectedUdp = ProtectedUdp::bind(&budget).map_err(|_| "server-bind")?;
    let endpoint: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, server.port().get());
    let client: UdpSocket = client()?;
    let mut c2s: Zeroizing<[u8; 32]> = Zeroizing::new(C2S);
    let mut s2c: Zeroizing<[u8; 32]> = Zeroizing::new(S2C);
    let mut marker_reads: u8 = 0;
    send(&client, &request()?, endpoint)?;
    server
        .exchange_initial_deny_observe_exit(
            &budget,
            &mut c2s,
            &mut s2c,
            |nonce: &mut [u8; 16]| {
                *nonce = SERVER_NONCE;
                Ok(())
            },
            || {
                marker_reads += 1;
                if marker_reads == 1 {
                    Ok(None)
                } else {
                    ClientExit::admit(b"client-exited=1\n").map(Some)
                }
            },
        )
        .map_err(|_| "initial-deny-exchange")?;
    assert_eq!(marker_reads, 2);
    assert_eq!(*c2s, [0; 32]);
    assert_eq!(*s2c, [0; 32]);

    let reply: [u8; 124] = receive_reply(&client, endpoint)?;
    assert_eq!(&reply[..2], &[0x24, 0]);
    assert_eq!(&reply[12..16], b"DENY");
    assert_eq!(&reply[24..32], &TRANSMIT);
    assert_eq!(&reply[48..52], &[1, 4, 0, 36]);
    assert_eq!(&reply[52..84], &UID);
    assert_eq!(&reply[84..92], &[4, 4, 0, 40, 0, 16, 0, 16]);
    assert_eq!(&reply[92..108], &SERVER_NONCE);
    let tag: &[u8; 16] = reply[108..124].try_into().map_err(|_| "deny-tag")?;
    Aes128Siv::new((&S2C).into())
        .decrypt_inout_detached(
            [&reply[..84], &reply[92..108]],
            (&mut [][..]).into(),
            tag.into(),
        )
        .map_err(|_| "deny-authentication")?;
    assert_no_reply(&client);
    let recovered: UdpSocket = UdpSocket::bind(endpoint).map_err(|_| "socket-still-held")?;
    drop(recovered);
    Ok(())
}

/// Both new sequences use the actual first cookie and listen until acknowledgement.
///
/// # Errors
/// Preserve socket, packet, observer or response-opening failures.
/// # Panics
/// Fail on wrong endpoints, order, reply correlation, cookies or key retention.
#[test]
fn protected_udp_ordinary_successors_observe_exact_expected_replies() -> TestResult {
    for reply_mode in [SuccessorReply::Ordinary, SuccessorReply::NakThenOrdinary] {
        let start: Instant = Instant::now();
        let mut budget: Budget = Budget::new(start).map_err(|_| "fixture-budget")?;
        budget.deadline = start
            .checked_add(Duration::from_secs(5))
            .ok_or("fixture-deadline")?;
        let server: ProtectedUdp = ProtectedUdp::bind(&budget).map_err(|_| "server-bind")?;
        let endpoint: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, server.port().get());
        let client: UdpSocket = client()?;
        client
            .set_read_timeout(Some(Duration::from_secs(2)))
            .map_err(|_| "client-timeout")?;
        let (signal, acknowledgement): (mpsc::SyncSender<()>, mpsc::Receiver<()>) =
            mpsc::sync_channel(1);
        let terminal_origin: [u8; 8] = 0x1122_3344_eedd_ccbb_u64.to_be_bytes();
        let terminal_uid: [u8; 32] = [0x23; 32];
        let terminal_nonce: [u8; 16] = [0x77; 16];
        let client_socket: &UdpSocket = &client;
        std::thread::scope(move |scope| -> TestResult {
            let peer: std::thread::ScopedJoinHandle<'_, TestResult> = scope.spawn(move || {
                let mut c2s: Zeroizing<[u8; 32]> = Zeroizing::new(C2S);
                let mut s2c: Zeroizing<[u8; 32]> = Zeroizing::new(S2C);
                let mut nonces: std::array::IntoIter<[u8; 16], 2> =
                    [SERVER_NONCE, terminal_nonce].into_iter();
                server
                    .exchange_successor_observe_exit(
                        &budget,
                        &mut c2s,
                        &mut s2c,
                        |nonce| {
                            *nonce = nonces.next().ok_or(Failure::Entropy)?;
                            Ok(())
                        },
                        || match acknowledgement.try_recv() {
                            Ok(()) => ClientExit::admit(b"client-exited=1\n").map(Some),
                            Err(mpsc::TryRecvError::Empty) => Ok(None),
                            Err(mpsc::TryRecvError::Disconnected) => {
                                Err(Failure::ExitAcknowledgement)
                            }
                        },
                        reply_mode,
                    )
                    .map_err(|_| "peer-exchange")?;
                assert_eq!(*c2s, [0; 32]);
                assert_eq!(*s2c, [0; 32]);
                assert_eq!(nonces.next(), None);
                Ok(())
            });
            send(client_socket, &request()?, endpoint)?;
            let first: [u8; 140] = receive_reply(client_socket, endpoint)?;
            let cookie: [u8; 12] = check_reply(&first)?;
            let second: [u8; 252] =
                request_with(terminal_origin, &terminal_uid, &cookie, &[0x69; 16])?;
            send(client_socket, &second, endpoint)?;
            if reply_mode == SuccessorReply::NakThenOrdinary {
                let nak: [u8; 84] = receive_reply(client_socket, endpoint)?;
                assert_eq!(&nak[..2], &[0x24, 0]);
                assert_eq!(&nak[12..16], b"NTSN");
                assert_eq!(&nak[24..32], &terminal_origin);
                assert_eq!(&nak[48..52], &[1, 4, 0, 36]);
                assert_eq!(&nak[52..], &terminal_uid);
            }
            let terminal: [u8; 140] = receive_reply(client_socket, endpoint)?;
            assert_eq!(
                open_ordinary(&terminal, terminal_origin, &terminal_uid, &terminal_nonce)?,
                [
                    0x12, 0x24, 0x36, 0x48, 0x5a, 0x6c, 0x7e, 0x90, 0xa2, 0xb4, 0xc6, 0xd8
                ]
            );
            signal.send(()).map_err(|_| "client-acknowledgement")?;
            peer.join().map_err(|_| "peer-panic")??;
            Ok(())
        })?;
        client
            .set_read_timeout(Some(Duration::from_millis(100)))
            .map_err(|_| "client-timeout")?;
        assert_no_reply(&client);
    }
    Ok(())
}

/// Read and independently attribute one complete loopback reply of the expected extent.
///
/// # Errors
/// Refuse a different response length, endpoint or receive failure.
fn receive_reply<const N: usize>(
    client: &UdpSocket,
    endpoint: SocketAddrV4,
) -> TestResult<[u8; N]> {
    let mut buffer: [u8; 141] = [0; 141];
    let (length, sender): (usize, SocketAddr) = client
        .recv_from(&mut buffer)
        .map_err(|_| "client-receive")?;
    if length != N || sender != SocketAddr::V4(endpoint) {
        return Err("reply-shape");
    }
    buffer[..length].try_into().map_err(|_| "reply-length")
}

/// The real two-leg peer detects a packet queued while the marker is read.
///
/// # Errors
/// Preserve socket setup, packet preparation or receipt failures.
/// # Panics
/// Fail if the old socket is released early or any extra datagram goes unnoticed
/// or receives a third reply.
#[test]
fn protected_udp_observation_checks_queue_again_after_client_exit() -> TestResult {
    let request: [u8; 252] = request()?;
    let replacement: [u8; 12] = [
        0x83, 0x73, 0x63, 0x53, 0x43, 0x33, 0x23, 0x13, 0x91, 0x92, 0x93, 0x94,
    ];
    let origin: [u8; 8] = 0x1122_3344_eedd_ccbb_u64.to_be_bytes();
    let successor: [u8; 252] = request_with(origin, &[0x23; 32], &replacement, &[0x69; 16])?;
    let oversized: [u8; 4096] = [0xa5; 4096];
    let plain: [u8; 48] = [0x23; 48];
    for extra in [
        None,
        Some(&[][..]),
        Some(oversized.as_slice()),
        Some(plain.as_slice()),
        Some(successor.as_slice()),
    ] {
        let budget: Budget = Budget::new(Instant::now()).map_err(|_| "fixture-budget")?;
        let server: ProtectedUdp = ProtectedUdp::bind(&budget).map_err(|_| "server-bind")?;
        let endpoint: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, server.port().get());
        let client: UdpSocket = client()?;
        send(&client, &request, endpoint)?;
        send(&client, &successor, endpoint)?;
        let did_read: Cell<bool> = Cell::new(false);
        let mut c2s: Zeroizing<[u8; 32]> = Zeroizing::new(C2S);
        let mut s2c: Zeroizing<[u8; 32]> = Zeroizing::new(S2C);
        let mut nonces: std::array::IntoIter<[u8; 16], 2> = [SERVER_NONCE, [0x77; 16]].into_iter();
        let result: Result<(), Failure> = server.exchange_successor_observe_exit(
            &budget,
            &mut c2s,
            &mut s2c,
            |nonce| {
                *nonce = nonces.next().ok_or(Failure::Entropy)?;
                Ok(())
            },
            || {
                did_read.set(true);
                if let Some(bytes) = extra {
                    send(&client, bytes, endpoint)
                        .map_err(|_| Failure::Io(ErrorKind::WriteZero))?;
                }
                ClientExit::admit(b"client-exited=1\n").map(Some)
            },
            SuccessorReply::Deny,
        );
        assert!(did_read.get());
        assert_eq!(*c2s, [0_u8; 32]);
        assert_eq!(*s2c, [0_u8; 32]);
        assert_eq!(nonces.next(), None);
        assert_eq!(
            result,
            extra.map_or(Ok(()), |_| Err(Failure::UnexpectedDatagram))
        );
        let mut buffer: [u8; 141] = [0; 141];
        for expected_length in [140, 124] {
            let (length, sender): (usize, SocketAddr) = client
                .recv_from(&mut buffer)
                .map_err(|_| "client-receive")?;
            assert_eq!(
                (length, sender),
                (expected_length, SocketAddr::V4(endpoint))
            );
        }
        assert_no_reply(&client);
    }
    Ok(())
}

/// An acknowledgement at the exact original cutoff cannot complete observation.
///
/// # Errors
/// Preserve setup failures.
/// # Panics
/// Fail if deadline equality is admitted.
#[test]
fn protected_udp_observation_refuses_acknowledgement_at_original_deadline() -> TestResult {
    let origin: Instant = Instant::now();
    let budget: Budget = Budget::new(origin).map_err(|_| "fixture-budget")?;
    let server: ProtectedUdp = ProtectedUdp::bind(&budget).map_err(|_| "server-bind")?;
    server
        .socket
        .set_nonblocking(true)
        .map_err(|_| "server-nonblocking")?;
    let observed: Cell<Instant> = Cell::new(origin);
    let result: Result<(), Failure> = server.observe_client_exit(
        &budget,
        || {
            observed.set(budget.deadline);
            ClientExit::admit(b"client-exited=1\n").map(Some)
        },
        || observed.get(),
    );
    assert_eq!(result, Err(Failure::Deadline));
    Ok(())
}

/// An absent marker leaves the actual peer socket waiting for the client's exit.
///
/// # Errors
/// Preserve local socket setup or observation failure.
/// # Panics
/// Fail if the observer ends on absence or reads the marker again after admission.
#[test]
fn protected_udp_observation_waits_for_client_exit() -> TestResult {
    let start: Instant = Instant::now();
    let budget: Budget = Budget::new(start).map_err(|_| "budget")?;
    let server: ProtectedUdp = ProtectedUdp::bind(&budget).map_err(|_| "bind")?;
    server
        .socket
        .set_nonblocking(true)
        .map_err(|_| "nonblocking")?;
    let mut polls: u8 = 0;
    server
        .observe_client_exit(
            &budget,
            || {
                polls += 1;
                if polls == 1 {
                    Ok(None)
                } else {
                    ClientExit::admit(b"client-exited=1\n").map(Some)
                }
            },
            || start,
        )
        .map_err(|_| "observation")?;
    assert_eq!(polls, 2);
    Ok(())
}

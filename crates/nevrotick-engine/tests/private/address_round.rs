// SPDX-License-Identifier: GPL-2.0-only
//! Independent configuration, actual-loopback and terminal-history controls.
//!
//! These fixtures exercise the fixed laboratory owner only. Native fixture errors
//! remain errors; no route, authenticated source, Core admission or AFD completion
//! is inferred. Later execution still requires the configured outer wall limit.

use super::*;
use std::{
    cell::Cell,
    error::Error,
    io,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddrV6, UdpSocket},
    sync::atomic::{AtomicBool, Ordering},
    thread::{self, JoinHandle},
};

/// Preserve actual fixture/driver failures separately from semantic assertions.
type Check<T> = Result<T, Box<dyn Error>>;

/// Independently assert a pure known-valid result before preserving its type.
/// # Errors
/// Return the original product error if the expected success was absent.
/// # Panics
/// Reject a refusal of the explicitly valid inputs named by the static fact.
fn product_ok<T, E>(result: Result<T, E>, fact: &'static str) -> Result<T, E> {
    assert!(result.is_ok(), "{fact}");
    result
}

/// Public fixed endpoint used only by effects-free configuration controls.
fn declared_peer(port: u16) -> SocketAddr {
    SocketAddr::from((Ipv4Addr::LOCALHOST, port))
}

/// One checked plan with independent finite durations and no native effects.
/// # Errors
/// Preserve unexpected rejection of the valid fixed plan.
/// # Panics
/// Reject refusal of the explicitly valid configuration.
fn checked_config(peers: &[SocketAddr]) -> Result<AddressRoundConfig, AddressRoundConfigError> {
    product_ok(
        AddressRoundConfig::new(peers, Duration::from_millis(200), Duration::from_secs(2)),
        "known-valid address-round configuration was refused",
    )
}

/// Supply three literal, distinct nonzero tokens in exactly one entropy fill.
fn fixed_entropy(bytes: &mut [u8; 24]) {
    bytes[..8].fill(1);
    bytes[8..16].fill(2);
    bytes[16..].fill(3);
}

/// Build a genuine current-thread fixture runtime without hiding native failure.
/// # Errors
/// Preserve the actual runtime construction cause.
fn runtime() -> io::Result<tokio::runtime::Runtime> {
    tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()
}

/// Own a real nonblocking endpoint before any driver or peer task starts.
/// IPv6 admission failure remains an environment error, never a silent pass.
/// # Errors
/// Preserve bind, nonblocking and actual-address errors.
fn bind_peer(ip: IpAddr) -> io::Result<UdpSocket> {
    let socket = UdpSocket::bind(SocketAddr::new(ip, 0))?;
    socket.set_nonblocking(true)?;
    Ok(socket)
}

/// Capture an actual native refusal while the conflicting endpoint remains owned.
/// # Errors
/// Preserve fixture setup failure or an environment that admits the second bind.
fn occupied_bind_refusal() -> io::Result<io::Error> {
    let occupied = bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?;
    match UdpSocket::bind(occupied.local_addr()?) {
        Err(error) => Ok(error),
        Ok(socket) => {
            drop(socket);
            Err(io::Error::other(
                "platform did not admit the occupied-bind error fixture",
            ))
        }
    }
}

/// A fixture's closed response choice; none changes the product's state directly.
#[derive(Clone, Copy)]
enum ReplyKind {
    /// A literal ordinary server header with matching origin and nonzero times.
    Ordinary,
    /// A literal RATE control; no address retry may evade its terminal result.
    Control,
    /// Wrong origin and wrong mode precede the valid neighboring ordinary header.
    RejectedThenOrdinary,
    /// Sixty-three wrong-mode datagrams precede the valid sixty-fourth reply.
    WrongMode63ThenOrdinary,
    /// Exactly sixty-four wrong-mode datagrams provide no valid first-address reply.
    WrongMode64,
}

/// Independently retained bytes of the actual query seen by one owned peer.
struct SeenQuery {
    /// Overflow room makes a non-header query observable without truncating it.
    bytes: [u8; 49],
    /// Actual returned length, never calculated from a product constant.
    length: usize,
    /// Actual endpoint needed only to send the corresponding fixture reply.
    source: SocketAddr,
    /// Peer-side observation time, distinct from product capture/processing time.
    received: Instant,
}

/// One bounded peer task; its owner always stops and joins it before propagation.
struct ReplyPeer {
    /// Cooperative termination avoids waiting for a request after an early refusal.
    stop: Arc<AtomicBool>,
    /// The actual task owns its socket and retains native fixture failures.
    job: Option<JoinHandle<io::Result<Option<SeenQuery>>>>,
}

impl ReplyPeer {
    /// Spawn one nonblocking peer with an independent five-second polling limit.
    /// # Errors
    /// Preserve actual thread creation failure.
    fn spawn(socket: UdpSocket, kind: ReplyKind) -> io::Result<Self> {
        let stop = Arc::new(AtomicBool::new(false));
        let task_stop = Arc::clone(&stop);
        let job = thread::Builder::new()
            .name("address-round-peer".into())
            .spawn(move || serve_once(&socket, kind, &task_stop))?;
        Ok(Self {
            stop,
            job: Some(job),
        })
    }

    /// Stop and join the actual peer before any driver or peer error escapes.
    /// # Errors
    /// Preserve native I/O failure or classify an owned fixture panic.
    fn finish(mut self) -> io::Result<Option<SeenQuery>> {
        self.stop.store(true, Ordering::Release);
        let job = self
            .job
            .take()
            .ok_or_else(|| io::Error::other("owned address-round peer was already joined"))?;
        job.join()
            .map_err(|_| io::Error::other("owned address-round peer panicked"))?
    }
}

impl Drop for ReplyPeer {
    /// Reap the independently bounded task during unwinding without replacing it.
    /// The ordinary finish method retains errors; this fallback never panics.
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(job) = self.job.take() {
            match job.join() {
                Ok(Ok(_)) => {}
                Ok(Err(error)) => {
                    // Best-effort bounded diagnostics cannot replace an unwinding failure.
                    let _diagnostic = io::Write::write_fmt(
                        &mut io::stderr().lock(),
                        format_args!(
                            "ADDRESS_ROUND_PEER_DROP kind={:?} native_code={:?}\n",
                            error.kind(),
                            error.raw_os_error(),
                        ),
                    );
                }
                Err(_) => {
                    let _diagnostic = io::Write::write_all(
                        &mut io::stderr().lock(),
                        b"ADDRESS_ROUND_PEER_DROP panic=1\n",
                    );
                }
            }
        }
    }
}

/// Retain each closed native/cleanup cause before choosing a fixture error.
/// Semantic invariant failures remain available to the independent assertions.
/// # Errors
/// Propagate actual setup/datagram/cleanup failure without scoring it as a guard.
fn classify_native_report(report: &AddressRoundReport) -> Check<()> {
    let primary = report.primary_error();
    let native_primary = primary.filter(|error| {
        matches!(
            error,
            PilotError::Bind(_)
                | PilotError::Nonblocking(_)
                | PilotError::LocalAddress(_)
                | PilotError::Registration(_)
                | PilotError::RuntimeBuild(_)
        )
    });
    let cleanup = report.cleanup_errors();
    let attempt_io = report
        .attempts()
        .iter()
        .any(|attempt| attempt.report().status() == Status::Io);
    if native_primary.is_none() && cleanup.iter().all(Option::is_none) && !attempt_io {
        return Ok(());
    }
    if let Some(error) = primary {
        let summary = FailureSummary::capture(error);
        let cause = summary.native_cause();
        eprintln!(
            "ADDRESS_ROUND_PRIMARY code={} kind={:?} native_code={:?}",
            summary.code(),
            cause.map(IoCause::kind),
            cause.and_then(IoCause::native_code)
        );
    }
    for (position, summary) in cleanup.into_iter().enumerate() {
        if let Some(summary) = summary {
            let cause = summary.native_cause();
            eprintln!(
                "ADDRESS_ROUND_CLEANUP position={position} code={} kind={:?} native_code={:?}",
                summary.code(),
                cause.map(IoCause::kind),
                cause.and_then(IoCause::native_code)
            );
        }
    }
    for (position, attempt) in report.attempts().iter().enumerate() {
        if attempt.report().status() == Status::Io {
            let cause = attempt.report().io_cause();
            eprintln!(
                "ADDRESS_ROUND_ATTEMPT_IO position={position} kind={:?} native_code={:?}",
                cause.map(IoCause::kind),
                cause.and_then(IoCause::native_code)
            );
        }
    }
    if let Some(error) = native_primary {
        return Err(error.into());
    }
    if cleanup.iter().any(Option::is_some) {
        return Err(io::Error::other(
            "actual address-round cleanup failed; closed causes retained",
        )
        .into());
    }
    for attempt in report.attempts() {
        if let Some(cause) = attempt.report().io_cause() {
            return Err(cause
                .native_code()
                .map_or_else(
                    || io::Error::new(cause.kind(), "actual address-round datagram failed"),
                    io::Error::from_raw_os_error,
                )
                .into());
        }
    }
    Err(io::Error::other("actual address-round datagram anomaly; closed cause retained").into())
}

/// Preserve a joined peer's bounded native facts even beside a driver failure.
/// # Errors
/// Select the driver error first only after retaining the independent peer cause.
fn joined_result(
    driver: Result<AddressRoundReport, PilotError>,
    peer: io::Result<Option<SeenQuery>>,
) -> Check<(AddressRoundReport, Option<SeenQuery>)> {
    if let Err(error) = &peer {
        eprintln!(
            "ADDRESS_ROUND_PEER kind={:?} native_code={:?}",
            error.kind(),
            error.raw_os_error()
        );
    }
    let report = driver?;
    classify_native_report(&report)?;
    Ok((report, peer?))
}

/// Observe one request and independently construct its reply using fixed offsets.
/// # Errors
/// Preserve native receive/send failure and the independent fixture deadline.
fn serve_once(
    socket: &UdpSocket,
    kind: ReplyKind,
    stop: &AtomicBool,
) -> io::Result<Option<SeenQuery>> {
    let deadline = Instant::now()
        .checked_add(Duration::from_secs(5))
        .ok_or_else(|| io::Error::other("fixture deadline overflow"))?;
    let mut bytes = [0; 49];
    loop {
        match socket.recv_from(&mut bytes) {
            Ok((length, source)) => {
                let seen = SeenQuery {
                    bytes,
                    length,
                    source,
                    received: Instant::now(),
                };
                let mut reply = [0; 48];
                reply[0] = 0x24; // LI=0, VN=4, server mode=4.
                reply[1] = 2;
                reply[24..32].copy_from_slice(&seen.bytes[40..48]);
                reply[32..40].copy_from_slice(&1_u64.to_be_bytes());
                reply[40..48].copy_from_slice(&2_u64.to_be_bytes());
                if matches!(kind, ReplyKind::RejectedThenOrdinary) {
                    let mut wrong_origin = reply;
                    wrong_origin[24] ^= 0xff;
                    send_fixture(socket, &wrong_origin, source, deadline)?;
                    let mut wrong_mode = reply;
                    wrong_mode[0] = 0x23;
                    send_fixture(socket, &wrong_mode, source, deadline)?;
                }
                if matches!(kind, ReplyKind::Control) {
                    reply[1] = 0;
                    reply[12..16].copy_from_slice(b"RATE");
                }
                let wrong_modes = match kind {
                    ReplyKind::WrongMode63ThenOrdinary => 63,
                    ReplyKind::WrongMode64 => 64,
                    _ => 0,
                };
                let mut wrong_mode = reply;
                wrong_mode[0] = 0x23;
                for _ in 0..wrong_modes {
                    send_fixture(socket, &wrong_mode, source, deadline)?;
                }
                if matches!(kind, ReplyKind::WrongMode64) {
                    return Ok(Some(seen));
                }
                send_fixture(socket, &reply, source, deadline)?;
                return Ok(Some(seen));
            }
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
            Err(error) => return Err(error),
        }
        if stop.load(Ordering::Acquire) {
            return Ok(None);
        }
        if Instant::now() >= deadline {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "fixture saw no query",
            ));
        }
        thread::sleep(Duration::from_millis(1));
    }
}

/// Bound fixture sending independently; a short send remains a fixture failure.
/// # Errors
/// Preserve native send errors, deadline or anomalous send length.
fn send_fixture(
    socket: &UdpSocket,
    bytes: &[u8; 48],
    target: SocketAddr,
    deadline: Instant,
) -> io::Result<()> {
    loop {
        match socket.send_to(bytes, target) {
            Ok(48) => return Ok(()),
            Ok(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "short fixture send",
                ));
            }
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
            Err(error) => return Err(error),
        }
        if Instant::now() >= deadline {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "fixture send deadline",
            ));
        }
        thread::sleep(Duration::from_millis(1));
    }
}

/// Read queued traffic for a fixed interval, retaining a bounded count and token.
/// # Errors
/// Preserve actual receive errors and checked fixture deadline construction.
/// # Panics
/// Reject an actual query whose length differs from the independent header size.
fn observe_queries(socket: &UdpSocket) -> io::Result<(u16, [u8; 8])> {
    let deadline = Instant::now()
        .checked_add(Duration::from_millis(20))
        .ok_or_else(|| io::Error::other("observation deadline overflow"))?;
    let mut count = 0_u16;
    let mut token = [0; 8];
    let mut bytes = [0; 49];
    loop {
        match socket.recv_from(&mut bytes) {
            Ok((length, _)) => {
                assert_eq!(length, 48, "peer observed a non-header query");
                count = count
                    .checked_add(1)
                    .ok_or_else(|| io::Error::other("fixture observation count exceeded"))?;
                token.copy_from_slice(&bytes[40..48]);
            }
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                if Instant::now() >= deadline {
                    return Ok((count, token));
                }
                thread::sleep(Duration::from_millis(1));
            }
            Err(error) => return Err(error),
        }
        if Instant::now() >= deadline {
            return Ok((count, token));
        }
    }
}

/// Assert literal request framing and retain no inferred authentication meaning.
/// # Panics
/// Reject a malformed actual query, missing token or non-loopback source.
fn check_query(seen: &SeenQuery) {
    assert_eq!(
        seen.length, 48,
        "actual query length differs from the independent header size"
    );
    assert_eq!(
        seen.bytes[0], 0x23,
        "actual query has the wrong LI/version/mode"
    );
    assert!(seen.bytes[1..40].iter().all(|byte| *byte == 0));
    assert!(seen.bytes[40..48].iter().any(|byte| *byte != 0));
    assert!(seen.source.ip().is_loopback() && seen.source.port() != 0);
}

/// Require a response peer to have observed the request without printing its bytes.
/// # Errors
/// Return a bounded fixture error only after the actual peer has been joined.
/// # Panics
/// Reject an absent query only after native driver/peer causes were classified.
fn require_query(seen: Option<SeenQuery>) -> io::Result<SeenQuery> {
    assert!(seen.is_some(), "the actual response peer received no query");
    seen.ok_or_else(|| io::Error::other("response query absent"))
}

/// Configuration rejects exact malformed neighbors before any preparation effect.
/// # Panics
/// Require closed independent categories and preserve the count/endpoint precedence.
#[test]
fn configuration_rejects_raw_counts_endpoints_and_insufficient_reservations() {
    let peers = [
        declared_peer(41001),
        declared_peer(41002),
        declared_peer(41003),
    ];
    let request = Duration::from_nanos(10);
    let total = Duration::from_nanos(30);
    let bad_counts = [Vec::new(), vec![peers[0]; 4]];
    for raw in bad_counts {
        assert!(matches!(
            AddressRoundConfig::new(&raw, request, total),
            Err(AddressRoundConfigError::Count)
        ));
    }
    for raw in [
        vec![declared_peer(0)],
        vec![SocketAddr::from(([192, 0, 2, 1], 123))],
        vec![SocketAddr::V6(SocketAddrV6::new(
            Ipv6Addr::LOCALHOST,
            123,
            1,
            0,
        ))],
        vec![SocketAddr::V6(SocketAddrV6::new(
            Ipv6Addr::LOCALHOST,
            123,
            0,
            1,
        ))],
    ] {
        assert!(matches!(
            AddressRoundConfig::new(&raw, request, total),
            Err(AddressRoundConfigError::InvalidPeer)
        ));
    }
    assert!(matches!(
        AddressRoundConfig::new(&[peers[0], peers[0]], request, total),
        Err(AddressRoundConfigError::DuplicatePeer)
    ));
    for (per, whole) in [
        (Duration::ZERO, total),
        (request, Duration::ZERO),
        (request, Duration::from_nanos(29)),
        (Duration::MAX, Duration::from_secs(30)),
        (
            Duration::from_secs(10),
            Duration::from_secs(30) + Duration::from_nanos(1),
        ),
    ] {
        assert!(matches!(
            AddressRoundConfig::new(&peers, per, whole),
            Err(AddressRoundConfigError::InvalidTimeout)
        ));
    }
}

/// Exact reservation and thirty-second boundaries remain healthy neighbors.
/// # Errors
/// Preserve unexpected pure constructor refusal.
/// # Panics
/// Require unchanged input order, owned copying and the exact boundary acceptance.
#[test]
fn configuration_accepts_one_to_three_candidates_and_exact_duration_edges() -> Check<()> {
    let mut peers = [
        declared_peer(41003),
        declared_peer(41001),
        declared_peer(41002),
    ];
    let original = peers;
    let config = product_ok(
        AddressRoundConfig::new(&peers, Duration::from_secs(10), Duration::from_secs(30)),
        "exact three-candidate reservation was refused",
    )?;
    peers[0] = declared_peer(41999);
    assert_ne!(peers, original);
    assert_eq!(config.peers.as_slice(), original.as_slice());
    assert_eq!(config.request_timeout, Duration::from_secs(10));
    assert_eq!(config.round_timeout, Duration::from_secs(30));
    for count in 1..=3 {
        let per = Duration::from_nanos(1);
        let whole = Duration::from_nanos(u64::try_from(count)?);
        let _config = product_ok(
            AddressRoundConfig::new(&original[..count], per, whole),
            "exact positive nanosecond reservation was refused",
        )?;
    }
    let _single = product_ok(
        AddressRoundConfig::new(
            &original[..1],
            Duration::from_secs(30),
            Duration::from_secs(30),
        ),
        "single candidate at the laboratory ceiling was refused",
    )?;
    Ok(())
}

/// Entropy failure, zero and every duplicate pair stop without a second fill.
/// # Errors
/// Preserve unexpected valid-plan rejection.
/// # Panics
/// Require the original closed cause and exactly one supplied entropy call.
#[test]
fn entropy_is_called_once_and_invalid_tokens_are_not_regenerated() -> Check<()> {
    let calls = Cell::new(0_u8);
    let failed = prepare_with_entropy(checked_config(&[declared_peer(41001)])?, |_| {
        calls.set(calls.get() + 1);
        Err(PilotError::Entropy)
    });
    assert!(matches!(failed, Err(PilotError::Entropy)));
    assert_eq!(calls.get(), 1);
    for tokens in [
        [0, 2, 3],
        [1, 0, 3],
        [1, 2, 0],
        [1, 1, 3],
        [1, 2, 1],
        [1, 2, 2],
    ] {
        let calls = Cell::new(0_u8);
        let result = prepare_with_entropy(checked_config(&[declared_peer(41001)])?, |bytes| {
            calls.set(calls.get() + 1);
            for (part, value) in bytes.as_chunks_mut::<8>().0.iter_mut().zip(tokens) {
                part.fill(value);
            }
            Ok(())
        });
        if tokens.contains(&0) {
            assert!(matches!(result, Err(PilotError::ZeroToken)));
        } else {
            assert!(matches!(result, Err(PilotError::DuplicateToken)));
        }
        assert_eq!(calls.get(), 1);
    }
    let (prepared, handle) = product_ok(
        prepare_with_entropy(checked_config(&[declared_peer(41001)])?, |bytes| {
            fixed_entropy(bytes);
            Ok(())
        }),
        "independent distinct nonzero tokens were refused",
    )?;
    for (packet, value) in prepared.packets.iter().zip([1_u8, 2, 3]) {
        assert_eq!(packet[0], 0x23);
        assert!(packet[1..40].iter().all(|byte| *byte == 0));
        assert!(packet[40..48].iter().all(|byte| *byte == value));
    }
    assert_eq!(handle.sender.receiver_count(), 1);
    drop(prepared);
    assert_eq!(handle.sender.receiver_count(), 0);
    Ok(())
}

/// Entered-runtime refusal occurs before the entropy callback is invoked.
/// # Errors
/// Preserve actual fixture runtime construction and valid configuration failure.
/// # Panics
/// Require the exact context refusal and zero entropy calls.
#[test]
fn entered_runtime_refuses_before_entropy() -> Check<()> {
    let runtime = runtime()?;
    let calls = Cell::new(0_u8);
    let config = checked_config(&[declared_peer(41001)])?;
    let result = runtime.block_on(async {
        prepare_with_entropy(config, |bytes| {
            calls.set(calls.get() + 1);
            fixed_entropy(bytes);
            Ok(())
        })
    });
    assert!(matches!(result, Err(PilotError::EnteredRuntime)));
    assert_eq!(calls.get(), 0);
    Ok(())
}

/// Exercise actual alternatives without turning native setup errors into assertions.
/// # Errors
/// Preserve bound-address, preparation, driver and joined peer failures.
/// # Panics
/// Require one timed-out first query, a second reply and no third send.
fn blackhole_then_reply(first_ip: IpAddr, second_ip: IpAddr) -> Check<()> {
    let first = bind_peer(first_ip)?;
    let second = bind_peer(second_ip)?;
    let third = bind_peer(first_ip)?;
    let peers = [
        first.local_addr()?,
        second.local_addr()?,
        third.local_addr()?,
    ];
    let config = checked_config(&peers)?;
    let (prepared, _control) = prepare_address_round(config)?;
    let schedule = product_ok(ControlSchedule::new(None, None), "quiet schedule refused")?;
    let responder = ReplyPeer::spawn(second, ReplyKind::Ordinary)?;
    let started = Instant::now();
    let result = execute_address_round(prepared, &schedule);
    let seen = responder.finish();
    let (report, seen) = joined_result(result, seen)?;
    let seen = require_query(seen)?;
    check_query(&seen);
    assert!(seen.received < started + Duration::from_secs(2));
    assert_eq!(report.end(), AddressRoundEnd::Reply);
    assert!(report.primary_error().is_none());
    assert!(report.cleanup_errors().iter().all(Option::is_none));
    assert_eq!(report.attempts().len(), 2);
    let [first_report, second_report] = report.attempts() else {
        return Err(io::Error::other("expected exactly two historical attempts").into());
    };
    assert_eq!(first_report.peer(), peers[0]);
    assert_eq!(first_report.report().status(), Status::Timeout);
    assert!(first_report.report().sent());
    assert_eq!(second_report.peer(), peers[1]);
    assert_eq!(second_report.report().status(), Status::Header);
    assert_eq!(second_report.report().stratum(), Some(2));
    let (first_count, first_token) = observe_queries(&first)?;
    assert_eq!(
        first_count, 1,
        "first address was revisited or never attempted"
    );
    assert!(first_token.iter().any(|byte| *byte != 0));
    assert!(
        !first_token.iter().eq(seen.bytes[40..48].iter()),
        "actual request token was reused across addresses"
    );
    assert_eq!(
        observe_queries(&third)?.0,
        0,
        "reply did not prevent the third send"
    );
    Ok(())
}

/// A same-family alternative gets real traffic after the first actual blackhole.
/// # Errors
/// Preserve native fixture/driver failure.
#[test]
fn same_family_blackhole_yields_second_reply_without_third_send() -> Check<()> {
    blackhole_then_reply(
        IpAddr::V4(Ipv4Addr::LOCALHOST),
        IpAddr::V4(Ipv4Addr::LOCALHOST),
    )
}

/// IPv4 then IPv6 uses real family-specific sockets where the environment admits them.
/// # Errors
/// Preserve unavailable IPv6 or other native fixture/driver failure without skipping.
#[test]
fn ipv4_blackhole_yields_ipv6_reply_without_third_send() -> Check<()> {
    blackhole_then_reply(
        IpAddr::V4(Ipv4Addr::LOCALHOST),
        IpAddr::V6(Ipv6Addr::LOCALHOST),
    )
}

/// Reversed family order has the same independently observed progress requirement.
/// # Errors
/// Preserve unavailable IPv6 or other native fixture/driver failure without skipping.
#[test]
fn ipv6_blackhole_yields_ipv4_reply_without_third_send() -> Check<()> {
    blackhole_then_reply(
        IpAddr::V6(Ipv6Addr::LOCALHOST),
        IpAddr::V4(Ipv4Addr::LOCALHOST),
    )
}

/// A correlated RATE control stops the actual round instead of trying its neighbor.
/// # Errors
/// Preserve native fixture/driver failure after joining the actual response peer.
/// # Panics
/// Require the original control code, one terminal record and no alternative query.
#[test]
fn correlated_control_ends_the_round_without_address_evasion() -> Check<()> {
    let first = bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?;
    let next = bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?;
    let peers = [first.local_addr()?, next.local_addr()?];
    let (prepared, _control) = prepare_address_round(checked_config(&peers)?)?;
    let schedule = product_ok(ControlSchedule::new(None, None), "quiet schedule refused")?;
    let responder = ReplyPeer::spawn(first, ReplyKind::Control)?;
    let result = execute_address_round(prepared, &schedule);
    let seen = responder.finish();
    let (report, seen) = joined_result(result, seen)?;
    check_query(&require_query(seen)?);
    assert_eq!(report.end(), AddressRoundEnd::Control);
    assert_eq!(report.attempts().len(), 1);
    let first = report
        .attempts()
        .first()
        .ok_or_else(|| io::Error::other("control record absent"))?;
    assert_eq!(first.report().status(), Status::Control);
    assert_eq!(
        first.report().control_code(),
        Some(u32::from_be_bytes(*b"RATE"))
    );
    assert!(first.report().facts().is_none());
    assert_eq!(observe_queries(&next)?.0, 0);
    Ok(())
}

/// Rejected packets leave the actual first request available to its valid neighbor.
/// # Errors
/// Preserve native fixture/driver failure after joining the peer.
/// # Panics
/// Require both independent rejections, one first-address reply and no successor.
#[test]
fn wrong_origin_and_mode_do_not_advance_before_the_valid_reply() -> Check<()> {
    let first = bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?;
    let next = bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?;
    let peers = [first.local_addr()?, next.local_addr()?];
    let (prepared, _control) = prepare_address_round(checked_config(&peers)?)?;
    let schedule = product_ok(ControlSchedule::new(None, None), "quiet schedule refused")?;
    let responder = ReplyPeer::spawn(first, ReplyKind::RejectedThenOrdinary)?;
    let result = execute_address_round(prepared, &schedule);
    let seen = responder.finish();
    let (report, seen) = joined_result(result, seen)?;
    check_query(&require_query(seen)?);
    assert_eq!(report.end(), AddressRoundEnd::Reply);
    assert_eq!(report.attempts().len(), 1);
    let record = report
        .attempts()
        .first()
        .ok_or_else(|| io::Error::other("header record absent"))?;
    assert_eq!(record.report().status(), Status::Header);
    assert_eq!(record.report().rejected(), 2);
    assert_eq!(record.report().received(), 3);
    assert_eq!(observe_queries(&next)?.0, 0);
    Ok(())
}

/// Exhaustion records every original candidate once and preserves distinct tokens.
/// # Errors
/// Preserve actual socket/preparation/driver/observation failures.
/// # Panics
/// Require three real timeout queries, stable order and no extra send per address.
#[test]
fn three_blackholes_exhaust_exactly_the_frozen_set() -> Check<()> {
    let sockets = [
        bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?,
        bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?,
        bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?,
    ];
    let peers = [
        sockets[0].local_addr()?,
        sockets[1].local_addr()?,
        sockets[2].local_addr()?,
    ];
    let (prepared, _control) = prepare_address_round(checked_config(&peers)?)?;
    let schedule = product_ok(ControlSchedule::new(None, None), "quiet schedule refused")?;
    let report = execute_address_round(prepared, &schedule)?;
    classify_native_report(&report)?;
    assert_eq!(report.end(), AddressRoundEnd::Exhausted);
    assert_eq!(report.attempts().len(), 3);
    let mut tokens = Vec::new();
    for ((attempt, socket), peer) in report.attempts().iter().zip(&sockets).zip(peers) {
        assert_eq!(attempt.peer(), peer);
        assert_eq!(attempt.report().status(), Status::Timeout);
        assert!(attempt.report().sent());
        let (count, token) = observe_queries(socket)?;
        assert_eq!(count, 1);
        assert!(token.iter().any(|byte| *byte != 0));
        assert!(
            !tokens.contains(&token),
            "request token was reused across addresses"
        );
        tokens.push(token);
    }
    Ok(())
}

/// A control admitted before execution creates no socket attempt or datagram.
/// # Errors
/// Preserve actual fixture/preparation/control/driver failure.
/// # Panics
/// Require distinct stop/invalidation summaries, zero history and zero traffic.
#[test]
fn preexisting_stop_and_invalidation_prevent_every_send() -> Check<()> {
    for invalidated in [false, true] {
        let socket = bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?;
        let peer = socket.local_addr()?;
        let (prepared, control) = prepare_address_round(checked_config(&[peer])?)?;
        if invalidated {
            control.invalidate()?;
        } else {
            control.stop();
        }
        let schedule = product_ok(ControlSchedule::new(None, None), "quiet schedule refused")?;
        let report = execute_address_round(prepared, &schedule)?;
        classify_native_report(&report)?;
        assert_eq!(
            report.end(),
            if invalidated {
                AddressRoundEnd::Stale
            } else {
                AddressRoundEnd::Stopped
            }
        );
        assert!(report.attempts().is_empty());
        assert!(report.primary_error().is_none());
        assert_eq!(observe_queries(&socket)?.0, 0);
    }
    Ok(())
}

/// A genuine terminal header survives controls at both sides of the summary sample.
/// # Errors
/// Preserve actual runtime, peer, control and driver failures after joining.
/// # Panics
/// Require the exact historical header and phase-dependent current summary.
#[test]
fn actual_header_survives_stop_and_invalidation_at_the_final_checkpoint() -> Check<()> {
    for invalidated in [false, true] {
        for phase in [RoundCheckpoint::Before, RoundCheckpoint::After] {
            let first = bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?;
            let next = bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?;
            let peers = [first.local_addr()?, next.local_addr()?];
            let (prepared, control) = prepare_address_round(checked_config(&peers)?)?;
            let runtime = runtime()?;
            let origin = Instant::now();
            let deadline = origin
                .checked_add(Duration::from_secs(2))
                .ok_or(PilotError::DeadlineOverflow)?;
            let responder = ReplyPeer::spawn(first, ReplyKind::Ordinary)?;
            let mut control_failure = None;
            let mut seen_checkpoint = false;
            let report = runtime.block_on(run_round_with(
                prepared,
                origin,
                deadline,
                &mut setup_address_request,
                &mut |index, reached| {
                    if index == 0 && reached == phase {
                        seen_checkpoint = true;
                        if invalidated {
                            control_failure = control.invalidate().err();
                        } else {
                            control.stop();
                        }
                    }
                },
            ));
            drop(runtime);
            let seen = responder.finish();
            let (report, seen) = joined_result(Ok(report), seen)?;
            if let Some(error) = control_failure {
                return Err(error.into());
            }
            check_query(&require_query(seen)?);
            assert!(
                seen_checkpoint,
                "actual driver omitted the terminal checkpoint"
            );
            assert_eq!(report.attempts().len(), 1);
            let actual = report
                .attempts()
                .first()
                .ok_or_else(|| io::Error::other("historical header absent"))?;
            assert_eq!(actual.report().status(), Status::Header);
            assert!(actual.report().facts().is_some());
            let expected = if phase == RoundCheckpoint::After {
                AddressRoundEnd::Reply
            } else if invalidated {
                AddressRoundEnd::Stale
            } else {
                AddressRoundEnd::Stopped
            };
            assert_eq!(report.end(), expected);
            assert_eq!(observe_queries(&next)?.0, 0);
        }
    }
    Ok(())
}

/// Actual native setup followed by control cannot escape through a stale send.
/// # Errors
/// Preserve native socket/runtime/preparation and control failures.
/// # Panics
/// Require one retired historical attempt and no actual received query.
#[test]
fn control_after_actual_setup_prevents_the_first_native_send() -> Check<()> {
    for invalidated in [false, true] {
        let socket = bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?;
        let peer = socket.local_addr()?;
        let (prepared, control) = prepare_address_round(checked_config(&[peer])?)?;
        let runtime = runtime()?;
        let origin = Instant::now();
        let deadline = origin
            .checked_add(Duration::from_secs(2))
            .ok_or(PilotError::DeadlineOverflow)?;
        let mut control_failure = None;
        let mut setup_count = 0_u8;
        let report = runtime.block_on(run_round_with(
            prepared,
            origin,
            deadline,
            &mut |destination, packet, context, receiver, request_end, round_end| {
                let outcome = setup_address_request(
                    destination,
                    packet,
                    context,
                    receiver,
                    request_end,
                    round_end,
                )?;
                setup_count += 1;
                if invalidated {
                    control_failure = control.invalidate().err();
                } else {
                    control.stop();
                }
                Ok(outcome)
            },
            &mut |_, _| {},
        ));
        drop(runtime);
        classify_native_report(&report)?;
        if let Some(error) = control_failure {
            return Err(error.into());
        }
        assert_eq!(setup_count, 1);
        assert_eq!(
            report.end(),
            if invalidated {
                AddressRoundEnd::Stale
            } else {
                AddressRoundEnd::Stopped
            }
        );
        assert_eq!(report.attempts().len(), 1);
        let first = report
            .attempts()
            .first()
            .ok_or_else(|| io::Error::other("retired attempt absent"))?;
        assert_eq!(
            first.report().status(),
            if invalidated {
                Status::Stale
            } else {
                Status::Stopped
            }
        );
        assert!(!first.report().sent());
        assert_eq!(observe_queries(&socket)?.0, 0);
    }
    Ok(())
}

/// A second setup error retains the first actual timeout and stops further setup.
/// The injected error is a captured native refusal, not a claimed failure of the
/// default ephemeral-bind adapter.
/// # Errors
/// Preserve native fixture/runtime failure or an unexpected actual first setup error.
/// # Panics
/// Require one real first query, unchanged primary attribution and no later query.
#[test]
fn setup_failure_retains_prior_timeout_and_the_actual_injected_cause() -> Check<()> {
    let sockets = [
        bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?,
        bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?,
        bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?,
    ];
    let peers = [
        sockets[0].local_addr()?,
        sockets[1].local_addr()?,
        sockets[2].local_addr()?,
    ];
    let actual = occupied_bind_refusal()?;
    let cause = IoCause::capture(&actual);
    let (prepared, _control) = prepare_address_round(checked_config(&peers)?)?;
    let runtime = runtime()?;
    let origin = Instant::now();
    let deadline = origin
        .checked_add(Duration::from_secs(2))
        .ok_or(PilotError::DeadlineOverflow)?;
    let mut setup_count = 0_u8;
    let mut unexpected_setup_error = None;
    let report = runtime.block_on(run_round_with(
        prepared,
        origin,
        deadline,
        &mut |destination, packet, context, receiver, request_end, round_end| {
            setup_count += 1;
            if setup_count == 2 {
                return Err(PilotError::Bind(cause));
            }
            let result = setup_address_request(
                destination,
                packet,
                context,
                receiver,
                request_end,
                round_end,
            );
            if let Err(error) = &result {
                unexpected_setup_error = Some(*error);
            }
            result
        },
        &mut |_, _| {},
    ));
    drop(runtime);
    if let Some(error) = unexpected_setup_error {
        return Err(error.into());
    }
    for attempt in report.attempts() {
        if attempt.report().status() == Status::Io {
            let actual = attempt.report().io_cause();
            eprintln!(
                "ADDRESS_ROUND_UNEXPECTED_ATTEMPT kind={:?} native_code={:?}",
                actual.map(IoCause::kind),
                actual.and_then(IoCause::native_code)
            );
            return Err(
                io::Error::other("actual first attempt failed before injected setup").into(),
            );
        }
    }
    assert_eq!(setup_count, 2);
    assert_eq!(report.end(), AddressRoundEnd::Failure);
    assert_eq!(report.attempts().len(), 1);
    assert!(matches!(report.primary_error(), Some(PilotError::Bind(_))));
    let Some(PilotError::Bind(primary)) = report.primary_error() else {
        return Err(io::Error::other("injected native primary absent").into());
    };
    assert_eq!(primary.kind(), cause.kind());
    assert_eq!(primary.native_code(), cause.native_code());
    let first = report
        .attempts()
        .first()
        .ok_or_else(|| io::Error::other("prior timeout absent"))?;
    assert_eq!(first.report().status(), Status::Timeout);
    assert!(report.cleanup_errors().iter().all(Option::is_none));
    assert_eq!(observe_queries(&sockets[0])?.0, 1);
    assert_eq!(observe_queries(&sockets[1])?.0, 0);
    assert_eq!(observe_queries(&sockets[2])?.0, 0);
    Ok(())
}

/// Cleanup projection retains a genuine captured native cause and both secondaries.
/// This tests the result boundary, not a native timer-failure or bind-adapter claim.
/// # Errors
/// Preserve actual fixture binding or the inability to establish the error control.
/// # Panics
/// Require primary-first attribution and independent bounded cleanup facts.
#[test]
fn cleanup_keeps_actual_native_primary_and_both_closed_secondary_causes() -> Check<()> {
    for (stop, invalidate, expected_end, expected_causes) in [
        (Ok(()), Ok(()), AddressRoundEnd::Exhausted, [None, None]),
        (
            Err(PilotError::ControlTask),
            Ok(()),
            AddressRoundEnd::Failure,
            [Some("control_task"), None],
        ),
        (
            Ok(()),
            Err(PilotError::GenerationExhausted),
            AddressRoundEnd::Failure,
            [None, Some("generation_exhausted")],
        ),
        (
            Err(PilotError::ControlTask),
            Err(PilotError::GenerationExhausted),
            AddressRoundEnd::Failure,
            [Some("control_task"), Some("generation_exhausted")],
        ),
    ] {
        let observed = reconcile_cleanup(AddressRoundReport::empty(), stop, invalidate);
        assert_eq!(observed.end(), expected_end);
        assert!(observed.primary_error().is_none());
        assert_eq!(
            observed
                .cleanup_errors()
                .map(|cause| cause.map(FailureSummary::code)),
            expected_causes,
        );
    }
    let actual = occupied_bind_refusal()?;
    let kind = actual.kind();
    let native = actual.raw_os_error();
    let report = reconcile_cleanup(
        AddressRoundReport::empty().fail(PilotError::Bind(IoCause::capture(&actual))),
        Err(PilotError::ControlTask),
        Err(PilotError::GenerationExhausted),
    );
    assert_eq!(report.end(), AddressRoundEnd::Failure);
    assert!(matches!(report.primary_error(), Some(PilotError::Bind(_))));
    let Some(PilotError::Bind(primary)) = report.primary_error() else {
        return Err(io::Error::other("native primary cause was replaced").into());
    };
    assert_eq!(primary.kind(), kind);
    assert_eq!(primary.native_code(), native);
    assert!(report.cleanup_errors().iter().all(Option::is_some));
    let [Some(stop), Some(invalidate)] = report.cleanup_errors() else {
        return Err(io::Error::other("independent cleanup cause was lost").into());
    };
    assert_eq!(stop.code(), PilotError::ControlTask.as_str());
    assert_eq!(invalidate.code(), PilotError::GenerationExhausted.as_str());
    assert!(stop.native_cause().is_none() && invalidate.native_cause().is_none());
    Ok(())
}

/// Exact global cutoffs are inclusive, independently of real clock scheduling.
/// # Errors
/// Preserve checked fixture instant construction failure.
/// # Panics
/// Require the declared just-before, exact and just-after classifications.
#[test]
fn global_summary_cutoff_distinguishes_one_nanosecond_before_at_and_after() -> Check<()> {
    let cutoff = Instant::now()
        .checked_add(Duration::from_secs(1))
        .ok_or(PilotError::DeadlineOverflow)?;
    let before = cutoff
        .checked_sub(Duration::from_nanos(1))
        .ok_or(PilotError::DeadlineOverflow)?;
    let after = cutoff
        .checked_add(Duration::from_nanos(1))
        .ok_or(PilotError::DeadlineOverflow)?;
    let quiet = Control {
        stop: false,
        epoch: 0,
    };
    assert_eq!(end_at(quiet, before, cutoff), None);
    assert_eq!(
        end_at(quiet, cutoff, cutoff),
        Some(AddressRoundEnd::Deadline)
    );
    assert_eq!(
        end_at(quiet, after, cutoff),
        Some(AddressRoundEnd::Deadline)
    );
    Ok(())
}

/// The real default setup distinguishes a reached request cutoff from global expiry.
/// Waiting is a fixture action after actual claim admission; neither original
/// cutoff is changed. An overscheduled request-only fixture is incomplete.
/// # Errors
/// Preserve native fixture/driver failures and request-only fixture overshoot.
/// # Panics
/// Require the actual setup branch, distinct summary and absence of later traffic.
#[test]
fn default_setup_reports_request_deadline_separately_from_original_global_deadline() -> Check<()> {
    for global_expiry in [false, true] {
        let first = bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?;
        let second = bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?;
        let peers = [first.local_addr()?, second.local_addr()?];
        let whole = if global_expiry {
            Duration::from_millis(200)
        } else {
            Duration::from_secs(2)
        };
        let config = product_ok(
            AddressRoundConfig::new(&peers, Duration::from_millis(40), whole),
            "known-valid setup-cutoff fixture was refused",
        )?;
        let (prepared, _control) = prepare_address_round(config)?;
        let runtime = runtime()?;
        let origin = Instant::now();
        let deadline = origin
            .checked_add(whole)
            .ok_or(PilotError::DeadlineOverflow)?;
        let mut setup_count = 0_u8;
        let mut request_fixture_overshot = false;
        let mut original_limits = None;
        let mut actual_setup_end = None;
        let report = runtime.block_on(run_round_with(
            prepared,
            origin,
            deadline,
            &mut |destination, packet, context, receiver, request_end, round_end| {
                setup_count += 1;
                original_limits = Some((request_end, round_end));
                let cutoff = if global_expiry {
                    round_end
                } else {
                    request_end
                };
                while let Some(remaining) = cutoff.checked_duration_since(Instant::now()) {
                    if remaining.is_zero() {
                        break;
                    }
                    thread::sleep(remaining);
                }
                let result = setup_address_request(
                    destination,
                    packet,
                    context,
                    receiver,
                    request_end,
                    round_end,
                );
                if !global_expiry && Instant::now() >= round_end {
                    request_fixture_overshot = true;
                }
                if let Ok(SetupOutcome::Retired(end)) = &result {
                    actual_setup_end = Some(*end);
                }
                result
            },
            &mut |_, _| {},
        ));
        drop(runtime);
        classify_native_report(&report)?;
        if setup_count == 0 && Instant::now() >= deadline {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "setup-cutoff fixture expired before reaching the actual setup seam",
            )
            .into());
        }
        if request_fixture_overshot {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "request-cutoff fixture also reached the original global deadline",
            )
            .into());
        }
        assert_eq!(setup_count, 1, "default setup was not reached exactly once");
        let Some((request_end, round_end)) = original_limits else {
            return Err(io::Error::other("default setup never received original cutoffs").into());
        };
        assert_eq!(round_end, deadline);
        assert!(request_end <= deadline);
        let expected = if global_expiry {
            AddressRoundEnd::Deadline
        } else {
            AddressRoundEnd::SetupDeadline
        };
        assert_eq!(actual_setup_end, Some(expected));
        assert_eq!(report.end(), expected);
        assert!(report.attempts().is_empty());
        assert!(report.primary_error().is_none());
        assert!(report.cleanup_errors().iter().all(Option::is_none));
        assert_eq!(observe_queries(&first)?.0, 0);
        assert_eq!(observe_queries(&second)?.0, 0);
    }
    Ok(())
}

/// Classify actual final-send I/O before applying the semantic guard assertions.
/// The healthy fixture's readiness rearm is incomplete, not a semantic refusal.
/// # Errors
/// Preserve actual native, short-send or readiness-rearm facts with bounded output.
fn classify_writable_result(slot: &LiveSlot, expected: Option<Status>) -> Check<()> {
    if let Some(TerminalOutcome::Io(IoOutcome::Native(cause))) = slot.terminal() {
        eprintln!(
            "ADDRESS_ROUND_WRITABLE_IO kind={:?} native_code={:?}",
            cause.kind(),
            cause.native_code()
        );
        return Err(cause
            .native_code()
            .map_or_else(
                || io::Error::new(cause.kind(), "actual guarded writable send failed"),
                io::Error::from_raw_os_error,
            )
            .into());
    }
    if matches!(
        slot.terminal(),
        Some(TerminalOutcome::Io(IoOutcome::ShortSend))
    ) {
        return Err(io::Error::other("actual guarded writable short-send anomaly").into());
    }
    if expected.is_none() && slot.send_attempts == 1 && slot.can_send() {
        return Err(io::Error::new(
            io::ErrorKind::WouldBlock,
            "actual writable neighbor required readiness rearming before emission",
        )
        .into());
    }
    Ok(())
}

/// The final writable guard sees controls after real setup/readiness, without
/// relying on the request loop's earlier refresh. Quiet control sends once.
/// # Errors
/// Preserve real setup/readiness/datagram errors and independent fixture overshoot.
/// # Panics
/// Require zero control-retired sends/charges beside one genuine healthy query.
#[test]
fn final_writable_guard_rechecks_controls_after_actual_socket_readiness() -> Check<()> {
    for expected in [None, Some(Status::Stopped), Some(Status::Stale)] {
        let remote = bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?;
        let peer = remote.local_addr()?;
        let (mut prepared, control) = prepare_address_round(checked_config(&[peer])?)?;
        let expected_token = prepared.packets[0][40..48].to_vec();
        let runtime = runtime()?;
        let origin = Instant::now();
        let deadline = origin
            .checked_add(Duration::from_secs(2))
            .ok_or(PilotError::DeadlineOverflow)?;
        let context = Arc::new(RunClockContext { origin });
        let result: Check<()> = runtime.block_on(async {
            let outcome = setup_address_request(
                peer,
                prepared.packets[0],
                context,
                &mut prepared.control,
                deadline,
                deadline,
            )?;
            if Instant::now() >= deadline {
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "writable fixture setup deadline",
                )
                .into());
            }
            assert!(
                matches!(&outcome, SetupOutcome::Ready(_)),
                "quiet default setup did not admit its real socket"
            );
            let SetupOutcome::Ready(mut slot) = outcome else {
                return Err(io::Error::other("quiet setup did not return its socket").into());
            };
            tokio::time::timeout(Duration::from_secs(1), slot.socket.writable()).await??;
            if Instant::now() >= deadline {
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "writable fixture readiness deadline",
                )
                .into());
            }
            assert!(slot.can_send());
            assert_eq!(slot.send_attempts, 0);
            match expected {
                Some(Status::Stopped) => control.stop(),
                Some(Status::Stale) => {
                    control.invalidate()?;
                }
                None => {}
                Some(_) => {
                    return Err(io::Error::other("invalid closed writable fixture case").into());
                }
            }
            guarded_writable(&mut slot, &mut prepared.control, Ok(()), deadline);
            if Instant::now() >= deadline {
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "writable fixture guard deadline",
                )
                .into());
            }
            classify_writable_result(&slot, expected)?;
            if let Some(status) = expected {
                assert_eq!(slot.terminal().map(TerminalOutcome::status), Some(status));
                assert!(!slot.sent);
                assert_eq!(slot.send_attempts, 0);
            } else {
                assert!(
                    slot.sent,
                    "quiet writable neighbor never sent its actual query"
                );
                assert_eq!(slot.send_attempts, 1);
                assert!(matches!(&slot.phase, crate::Phase::Pending(_)));
            }
            drop(slot);
            Ok(())
        });
        drop(runtime);
        result?;
        let (queries, token) = observe_queries(&remote)?;
        assert_eq!(queries, u16::from(expected.is_none()));
        if expected.is_none() {
            assert!(
                token.iter().eq(expected_token.iter()),
                "actual guarded query token differs from its prepared token"
            );
        }
    }
    Ok(())
}

/// Public error/configuration/preparation formatting has a fixed redacted shape.
/// # Errors
/// Preserve unexpected pure preparation refusal of the literal valid inputs.
/// # Panics
/// Reject payload-bearing, unbounded or changed public diagnostic projections.
#[test]
fn public_configuration_errors_and_preparation_format_without_payloads() -> Check<()> {
    let endpoint = declared_peer(45678);
    for (peers, per, category, display, debug) in [
        (
            Vec::new(),
            Duration::from_millis(1),
            AddressRoundConfigError::Count,
            "invalid address-round candidate count",
            "Count",
        ),
        (
            vec![SocketAddr::from(([192, 0, 2, 1], 45678))],
            Duration::from_millis(1),
            AddressRoundConfigError::InvalidPeer,
            "invalid address-round endpoint",
            "InvalidPeer",
        ),
        (
            vec![endpoint, endpoint],
            Duration::from_millis(1),
            AddressRoundConfigError::DuplicatePeer,
            "duplicate address-round endpoint",
            "DuplicatePeer",
        ),
        (
            vec![endpoint],
            Duration::ZERO,
            AddressRoundConfigError::InvalidTimeout,
            "invalid address-round duration budget",
            "InvalidTimeout",
        ),
    ] {
        let result = AddressRoundConfig::new(&peers, per, Duration::from_secs(1));
        assert!(result.is_err(), "invalid public configuration was accepted");
        let error = result
            .err()
            .ok_or_else(|| io::Error::other("expected configuration refusal absent"))?;
        assert_eq!(error, category);
        assert!(error.source().is_none());
        for (rendered, expected) in [(error.to_string(), display), (format!("{error:?}"), debug)] {
            assert!(rendered.is_ascii() && rendered.len() <= 64);
            assert!(
                rendered.bytes().eq(expected.bytes()),
                "public error formatting exposed or changed payload"
            );
        }
    }
    let config = checked_config(&[endpoint])?;
    let config_text = format!("{config:?}");
    let (prepared, _control) = product_ok(
        prepare_with_entropy(config, |bytes| {
            fixed_entropy(bytes);
            Ok(())
        }),
        "literal nonzero distinct preparation was refused",
    )?;
    for (rendered, expected) in [
        (config_text, "AddressRoundConfig { .. }"),
        (format!("{prepared:?}"), "PreparedAddressRound { .. }"),
    ] {
        assert!(rendered.is_ascii() && rendered.len() <= 64);
        assert!(
            rendered.bytes().eq(expected.bytes()),
            "public owner formatting exposed configuration or entropy"
        );
    }
    Ok(())
}

/// Preparing outside a runtime cannot authorize nested execution inside one.
/// # Errors
/// Preserve actual fixture/runtime/preparation or legal-neighbor acquisition failure.
/// # Panics
/// Require the entered-runtime refusal with no traffic beside a genuine legal reply.
#[test]
fn execute_refuses_entered_runtime_after_external_preparation_with_legal_neighbor() -> Check<()> {
    let remote = bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?;
    let peer = remote.local_addr()?;
    let (prepared, rejected_control) = prepare_address_round(checked_config(&[peer])?)?;
    let schedule = product_ok(
        ControlSchedule::new(Some(Duration::from_secs(5)), Some(Duration::from_secs(6))),
        "future control schedule refused",
    )?;
    let runtime = runtime()?;
    let rejected = runtime.block_on(async { execute_address_round(prepared, &schedule) });
    assert!(matches!(rejected, Err(PilotError::EnteredRuntime)));
    assert_eq!(runtime.metrics().num_alive_tasks(), 0);
    assert_eq!(rejected_control.sender.sender_count(), 1);
    assert_eq!(observe_queries(&remote)?.0, 0);
    drop(runtime);
    let (prepared, _control) = prepare_address_round(checked_config(&[peer])?)?;
    let responder = ReplyPeer::spawn(remote, ReplyKind::Ordinary)?;
    let result = execute_address_round(prepared, &schedule);
    let seen = responder.finish();
    let (report, seen) = joined_result(result, seen)?;
    check_query(&require_query(seen)?);
    assert_eq!(report.end(), AddressRoundEnd::Reply);
    assert_eq!(report.attempts().len(), 1);
    assert!(report.cleanup_errors().iter().all(Option::is_none));
    Ok(())
}

/// Run real Some-timer ownership while keeping its current-thread runtime alive.
/// Future timers lie beyond the whole admitted round; due controls are deterministic.
/// # Errors
/// Preserve native fixture/driver/peer errors and checked deadline construction.
/// # Panics
/// Require the expected terminal result, no surviving tasks/senders and no late actions.
fn owned_timer_case(expected: AddressRoundEnd) -> Check<()> {
    let remote = bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?;
    let peer = remote.local_addr()?;
    let (prepared, control) = prepare_address_round(checked_config(&[peer])?)?;
    let runtime = runtime()?;
    let origin = Instant::now();
    let deadline = origin
        .checked_add(Duration::from_secs(2))
        .ok_or(PilotError::DeadlineOverflow)?;
    let future_stop = origin
        .checked_add(Duration::from_secs(5))
        .ok_or(PilotError::DeadlineOverflow)?;
    let future_invalidate = origin
        .checked_add(Duration::from_secs(6))
        .ok_or(PilotError::DeadlineOverflow)?;
    let (stop_when, invalidate_when) = match expected {
        AddressRoundEnd::Reply => (future_stop, future_invalidate),
        AddressRoundEnd::Stopped => (origin, future_invalidate),
        AddressRoundEnd::Stale => (future_stop, origin),
        _ => return Err(io::Error::other("invalid closed timer fixture case").into()),
    };
    let (responder, quiet) = if expected == AddressRoundEnd::Reply {
        (Some(ReplyPeer::spawn(remote, ReplyKind::Ordinary)?), None)
    } else {
        (None, Some(remote))
    };
    let report = runtime.block_on(run_owned(
        prepared,
        origin,
        deadline,
        Some(stop_when),
        Some(invalidate_when),
    ));
    let seen = if let Some(responder) = responder {
        responder.finish()
    } else {
        Ok(None)
    };
    let (report, seen) = joined_result(Ok(report), seen)?;
    assert_eq!(
        runtime.metrics().num_alive_tasks(),
        0,
        "timer task survived owned completion"
    );
    assert_eq!(
        control.sender.sender_count(),
        1,
        "timer sender survived owned completion"
    );
    assert_eq!(report.end(), expected);
    assert!(report.cleanup_errors().iter().all(Option::is_none));
    let final_control = *control.sender.borrow();
    assert_eq!(final_control.stop, expected == AddressRoundEnd::Stopped);
    assert_eq!(
        final_control.epoch,
        u64::from(expected == AddressRoundEnd::Stale)
    );
    drop(runtime);
    if expected == AddressRoundEnd::Reply {
        check_query(&require_query(seen)?);
        assert_eq!(report.attempts().len(), 1);
    } else {
        assert!(report.attempts().is_empty());
        let socket = quiet.ok_or_else(|| io::Error::other("quiet timer peer absent"))?;
        assert_eq!(observe_queries(&socket)?.0, 0);
    }
    Ok(())
}

/// Timer cleanup is observed before runtime teardown, not hidden by its destructor.
/// # Errors
/// Preserve actual fixture/driver errors from each closed case.
/// # Panics
/// Require reaping for future-both reply, due stop and due invalidation.
#[test]
fn owned_timers_are_reaped_before_the_live_runtime_is_dropped() -> Check<()> {
    for expected in [
        AddressRoundEnd::Reply,
        AddressRoundEnd::Stopped,
        AddressRoundEnd::Stale,
    ] {
        owned_timer_case(expected)?;
    }
    Ok(())
}

/// Admit the native in-use control for one actual socket's exact local endpoint.
/// # Errors
/// Preserve native failures or classify a platform without exclusive fixture binding.
fn require_local_socket_in_use(local: SocketAddr) -> io::Result<()> {
    match UdpSocket::bind(local) {
        Err(error) if error.kind() == io::ErrorKind::AddrInUse => Ok(()),
        Err(error) => Err(error),
        Ok(socket) => {
            drop(socket);
            Err(io::Error::other(
                "platform did not admit the in-use socket control",
            ))
        }
    }
}

/// Rebind the actual previous endpoint at the next setup boundary after real timeout.
/// # Errors
/// Preserve native fixture errors or insufficient time to reach the successor.
/// # Panics
/// Require the in-use neighbor and completed socket release before the successor setup.
#[test]
fn previous_socket_is_closed_before_the_next_actual_setup() -> Check<()> {
    let first = bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?;
    let second = bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?;
    let peers = [first.local_addr()?, second.local_addr()?];
    let (prepared, _control) = prepare_address_round(checked_config(&peers)?)?;
    let runtime = runtime()?;
    let origin = Instant::now();
    let deadline = origin
        .checked_add(Duration::from_secs(2))
        .ok_or(PilotError::DeadlineOverflow)?;
    let mut setup_count = 0_u8;
    let mut first_local = None;
    let mut in_use_observed = false;
    let mut released_before_next = false;
    let mut still_in_use = false;
    let mut fixture_error = None;
    let report = runtime.block_on(run_round_with(
        prepared,
        origin,
        deadline,
        &mut |destination, packet, context, receiver, request_end, round_end| {
            setup_count += 1;
            if setup_count == 2 {
                let Some(local) = first_local else {
                    return Err(PilotError::InternalState);
                };
                match UdpSocket::bind(local) {
                    Ok(socket) => {
                        drop(socket);
                        released_before_next = true;
                    }
                    Err(error) if error.kind() == io::ErrorKind::AddrInUse => {
                        eprintln!(
                            "ADDRESS_ROUND_SOCKET_RETAINED kind={:?} native_code={:?}",
                            error.kind(),
                            error.raw_os_error()
                        );
                        still_in_use = true;
                        return Err(PilotError::InternalState);
                    }
                    Err(error) => {
                        fixture_error = Some(error);
                        return Err(PilotError::InternalState);
                    }
                }
            }
            let outcome = setup_address_request(
                destination,
                packet,
                context,
                receiver,
                request_end,
                round_end,
            )?;
            if let (1, SetupOutcome::Ready(slot)) = (setup_count, &outcome) {
                first_local = Some(slot.local);
                if let Err(error) = require_local_socket_in_use(slot.local) {
                    fixture_error = Some(error);
                    return Err(PilotError::InternalState);
                }
                in_use_observed = true;
            }
            Ok(outcome)
        },
        &mut |_, _| {},
    ));
    drop(runtime);
    classify_native_report(&report)?;
    fixture_error.map_or(Ok(()), Err)?;
    if setup_count < 2 && Instant::now() >= deadline {
        return Err(io::Error::new(
            io::ErrorKind::TimedOut,
            "socket-release fixture did not reach successor",
        )
        .into());
    }
    assert!(
        in_use_observed,
        "actual first socket never established its in-use neighbor"
    );
    assert!(
        !still_in_use,
        "previous actual socket remained bound at successor setup"
    );
    assert!(
        released_before_next,
        "successor setup did not observe the previous socket release"
    );
    assert_eq!(setup_count, 2);
    assert_eq!(report.end(), AddressRoundEnd::Exhausted);
    assert_eq!(report.attempts().len(), 2);
    assert!(
        report
            .attempts()
            .iter()
            .all(|attempt| attempt.report().status() == Status::Timeout)
    );
    assert_eq!(observe_queries(&first)?.0, 1);
    assert_eq!(observe_queries(&second)?.0, 1);
    Ok(())
}

/// Publish control only after the driver's outer check, at the actual setup seam.
/// # Errors
/// Preserve native setup/peer failures and a quiet neighbor's setup overshoot.
/// # Panics
/// Require the handler's own retirement, or actual Ready plus one real reply.
fn setup_handler_control_case(expected: Option<AddressRoundEnd>) -> Check<()> {
    let remote = bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?;
    let next = bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?;
    let peers = [remote.local_addr()?, next.local_addr()?];
    let (prepared, control) = prepare_address_round(checked_config(&peers)?)?;
    let runtime = runtime()?;
    let origin = Instant::now();
    let deadline = origin
        .checked_add(Duration::from_secs(2))
        .ok_or(PilotError::DeadlineOverflow)?;
    let responder = ReplyPeer::spawn(remote, ReplyKind::Ordinary)?;
    let mut calls = 0_u8;
    let mut handler_outcome = None;
    let mut handler_checked = None;
    let report = runtime.block_on(run_round_with(
        prepared,
        origin,
        deadline,
        &mut |peer, packet, context, receiver, request_end, round_end| {
            calls += 1;
            match expected {
                Some(AddressRoundEnd::Stopped) => control.stop(),
                Some(AddressRoundEnd::Stale) => control.invalidate()?,
                None => {}
                Some(_) => return Err(PilotError::InternalState),
            }
            let outcome =
                setup_address_request(peer, packet, context, receiver, request_end, round_end)?;
            handler_checked = Some((request_end, Instant::now()));
            handler_outcome = Some(match &outcome {
                SetupOutcome::Ready(_) => None,
                SetupOutcome::Retired(end) => Some(*end),
            });
            Ok(outcome)
        },
        &mut |_, _| {},
    ));
    let returned = Instant::now();
    let seen = responder.finish();
    let (report, seen) = joined_result(Ok(report), seen)?;
    if (report.end() == AddressRoundEnd::Deadline && returned >= deadline)
        || (expected.is_none()
            && handler_outcome == Some(Some(AddressRoundEnd::SetupDeadline))
            && handler_checked.is_some_and(|(cutoff, checked)| checked >= cutoff))
    {
        return Err(io::Error::new(
            io::ErrorKind::TimedOut,
            "quiet setup-handler fixture exceeded its actual setup cutoff",
        )
        .into());
    }
    assert_eq!(calls, 1, "the actual setup handler was not reached once");
    assert_eq!(handler_outcome, Some(expected));
    assert_eq!(report.end(), expected.unwrap_or(AddressRoundEnd::Reply));
    assert!(report.primary_error().is_none());
    assert!(report.cleanup_errors().iter().all(Option::is_none));
    if expected.is_some() {
        assert!(report.attempts().is_empty());
        assert!(seen.is_none(), "a handler-retired address received a query");
    } else {
        check_query(&require_query(seen)?);
        assert_eq!(report.attempts().len(), 1);
        assert_eq!(report.attempts()[0].report().status(), Status::Header);
    }
    assert_eq!(observe_queries(&next)?.0, 0);
    assert_eq!(runtime.metrics().num_alive_tasks(), 0);
    Ok(())
}

/// The actual setup handler independently checks controls after the outer guard.
/// # Errors
/// Preserve real fixture failures rather than treating them as control rejection.
#[test]
fn setup_handler_rechecks_stop_and_invalidation_with_a_real_ready_neighbor() -> Check<()> {
    for expected in [
        None,
        Some(AddressRoundEnd::Stopped),
        Some(AddressRoundEnd::Stale),
    ] {
        setup_handler_control_case(expected)?;
    }
    Ok(())
}

/// Seed only the private counter; this does not execute 2^64 public invalidations.
/// # Errors
/// Preserve preparation/runtime/native observation failures or fixture overshoot.
/// # Panics
/// Require checked overflow attribution and reaping of the already-started stop job.
fn owned_counter_boundary_case(seed: u64) -> Check<()> {
    let remote = bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?;
    let (prepared, control) = prepare_address_round(checked_config(&[remote.local_addr()?])?)?;
    control.sender.send_modify(|value| value.epoch = seed);
    let runtime = runtime()?;
    let origin = Instant::now();
    let deadline = origin
        .checked_add(Duration::from_secs(2))
        .ok_or(PilotError::DeadlineOverflow)?;
    let stop_when = origin
        .checked_add(Duration::from_secs(5))
        .ok_or(PilotError::DeadlineOverflow)?;
    let report = runtime.block_on(run_owned(
        prepared,
        origin,
        deadline,
        Some(stop_when),
        Some(origin),
    ));
    classify_native_report(&report)?;
    if Instant::now() >= stop_when {
        return Err(io::Error::new(
            io::ErrorKind::TimedOut,
            "private counter fixture exceeded the future-stop admission window",
        )
        .into());
    }
    assert!(report.attempts().is_empty());
    assert!(report.cleanup_errors().iter().all(Option::is_none));
    assert_eq!(runtime.metrics().num_alive_tasks(), 0);
    assert_eq!(control.sender.sender_count(), 1);
    let state = *control.sender.borrow();
    assert_eq!(state.epoch, u64::MAX);
    assert_eq!(state.stop, seed == u64::MAX);
    if seed == u64::MAX {
        assert_eq!(report.end(), AddressRoundEnd::Failure);
        assert!(matches!(
            report.primary_error(),
            Some(PilotError::GenerationExhausted)
        ));
    } else {
        assert_eq!(report.end(), AddressRoundEnd::Stale);
        assert!(report.primary_error().is_none());
    }
    assert_eq!(observe_queries(&remote)?.0, 0);
    Ok(())
}

/// A private MAX/MAX-1 seed bypasses historical repetition, not checked arithmetic.
/// # Errors
/// Preserve actual fixture failures from both boundary rows.
#[test]
fn private_counter_seed_run_owned_reaps_stop_on_due_invalidation_overflow() -> Check<()> {
    for seed in [u64::MAX, u64::MAX - 1] {
        owned_counter_boundary_case(seed)?;
    }
    Ok(())
}

/// Await an actual future invalidation job on a current-thread runtime.
/// The private seed skips 2^64 public history; no task panic is manufactured.
/// # Errors
/// Preserve genuine watch/timeout failures and an expired-before-spawn fixture.
/// # Panics
/// Require actual notification, completion before reaping and the exact checked result.
fn future_invalidation_boundary_case(seed: u64) -> Check<()> {
    let (mut prepared, control) = prepare_address_round(checked_config(&[declared_peer(41001)])?)?;
    control.sender.send_modify(|value| value.epoch = seed);
    let initial = *prepared.control.borrow_and_update();
    assert_eq!(initial.epoch, seed);
    assert!(!initial.stop);
    let runtime = runtime()?;
    let outcome: Check<_> = runtime.block_on(async {
        let when = Instant::now()
            .checked_add(Duration::from_millis(100))
            .ok_or(PilotError::DeadlineOverflow)?;
        let started = start_timer(Some(when), control.clone(), TimerAction::Invalidate);
        if Instant::now() >= when && !matches!(&started, Ok(Some(_))) {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "future invalidation fixture expired before an owned job was established",
            )
            .into());
        }
        let job = product_ok(started, "future invalidation returned an immediate error")?;
        assert!(
            job.is_some(),
            "future invalidation did not start its real job"
        );
        let job = job.ok_or_else(|| io::Error::other("future invalidation job absent"))?;
        let notified =
            tokio::time::timeout(Duration::from_secs(2), prepared.control.changed()).await;
        let completed = job.is_finished();
        let reaped = finish_timer(Some(job)).await;
        Ok((notified, completed, reaped))
    });
    let (notified, completed, reaped) = outcome?;
    let state = *prepared.control.borrow_and_update();
    drop(prepared);
    if let Err(error) = reaped {
        let expected_boundary =
            seed == u64::MAX && matches!(error, PilotError::GenerationExhausted);
        eprintln!(
            "ADDRESS_ROUND_TIMER_RESULT expected_boundary={expected_boundary} code={}",
            error.as_str()
        );
    }
    notified??;
    assert!(
        completed,
        "actual invalidation job was not complete before reaping"
    );
    assert_eq!(runtime.metrics().num_alive_tasks(), 0);
    assert_eq!(control.sender.sender_count(), 1);
    assert_eq!(state.epoch, u64::MAX);
    assert_eq!(state.stop, seed == u64::MAX);
    if seed == u64::MAX {
        assert!(matches!(reaped, Err(PilotError::GenerationExhausted)));
    } else {
        product_ok(
            reaped,
            "MAX-1 future invalidation refused its final valid increment",
        )?;
    }
    Ok(())
}

/// Both private counter boundaries exercise the real future timer body and watch.
/// # Errors
/// Preserve actual runtime/watch errors without claiming public-history coverage.
#[test]
fn private_counter_seed_future_invalidation_notifies_and_retains_returned_result() -> Check<()> {
    for seed in [u64::MAX, u64::MAX - 1] {
        future_invalidation_boundary_case(seed)?;
    }
    Ok(())
}

/// Check only real transferred receive history, without assigning packet counters.
/// # Errors
/// Preserve a native/deadline fixture that did not establish sixty-four reads.
/// # Panics
/// Require the literal 63/64 acceptance boundary and ordered Budget continuation.
fn check_receive_boundary_history(
    report: &AddressRoundReport,
    invalid_count: u16,
    peers: &[SocketAddr; 3],
    deadline: Instant,
    returned: Instant,
    latest_setup_cutoff: Option<Instant>,
) -> Check<()> {
    if (report.end() == AddressRoundEnd::Deadline && returned >= deadline)
        || (report.end() == AddressRoundEnd::SetupDeadline
            && latest_setup_cutoff.is_some_and(|cutoff| returned >= cutoff))
    {
        return Err(io::Error::new(
            io::ErrorKind::TimedOut,
            "receive-boundary fixture exceeded its actual original cutoff",
        )
        .into());
    }
    assert!(
        !report.attempts().is_empty(),
        "receive-boundary driver omitted its required historical attempt"
    );
    let first = report
        .attempts()
        .first()
        .ok_or_else(|| io::Error::other("receive-boundary fixture produced no first history"))?;
    if first.report().received() < 64
        && (first.report().status() == Status::Timeout
            || (invalid_count == 63 && first.report().status() == Status::Header))
    {
        eprintln!(
            "ADDRESS_ROUND_RECEIVE_INCOMPLETE status={:?} received={} rejected={}",
            first.report().status(),
            first.report().received(),
            first.report().rejected()
        );
        return Err(io::Error::new(
            io::ErrorKind::TimedOut,
            "native receive fixture did not establish its sixty-four-read boundary",
        )
        .into());
    }
    assert_eq!(report.end(), AddressRoundEnd::Reply);
    assert!(report.primary_error().is_none());
    assert!(report.cleanup_errors().iter().all(Option::is_none));
    assert_eq!(
        report.attempts().len(),
        if invalid_count == 63 { 1 } else { 2 }
    );
    assert_eq!(first.peer(), peers[0]);
    assert_eq!(first.report().received(), 64);
    assert_eq!(first.report().rejected(), invalid_count);
    assert!(first.report().sent());
    assert_eq!(
        first.report().status(),
        if invalid_count == 63 {
            Status::Header
        } else {
            Status::Budget
        }
    );
    if invalid_count == 64 {
        let second = report
            .attempts()
            .get(1)
            .ok_or_else(|| io::Error::other("receive-budget successor history absent"))?;
        assert_eq!(second.peer(), peers[1]);
        assert_eq!(second.report().status(), Status::Header);
        assert_eq!(second.report().received(), 1);
        assert_eq!(second.report().rejected(), 0);
        assert!(second.report().sent());
    }
    Ok(())
}

/// Send exactly sixty-four literal datagrams only after observing the real query.
/// # Errors
/// Preserve native driver/peer errors and queue/deadline admission failures.
/// # Panics
/// Require an unchanged cutoff, actual boundary history and no third-address send.
fn receive_budget_case(invalid_count: u16) -> Check<()> {
    let first = bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?;
    let second = bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?;
    let third = bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?;
    let peers = [
        first.local_addr()?,
        second.local_addr()?,
        third.local_addr()?,
    ];
    let kind = match invalid_count {
        63 => ReplyKind::WrongMode63ThenOrdinary,
        64 => ReplyKind::WrongMode64,
        _ => return Err(io::Error::other("invalid closed receive-boundary row").into()),
    };
    let config = product_ok(
        AddressRoundConfig::new(&peers, Duration::from_secs(1), Duration::from_secs(3)),
        "explicit one-second receive reservation was refused",
    )?;
    let (prepared, _control) = prepare_address_round(config)?;
    let runtime = runtime()?;
    let origin = Instant::now();
    let deadline = origin
        .checked_add(Duration::from_secs(3))
        .ok_or(PilotError::DeadlineOverflow)?;
    let first_peer = ReplyPeer::spawn(first, kind)?;
    let second_peer = ReplyPeer::spawn(second, ReplyKind::Ordinary)?;
    let mut first_cutoff = None;
    let mut latest_setup_cutoff = None;
    let mut first_transferred = None;
    let mut setup_count = 0_u8;
    let report = runtime.block_on(run_round_with(
        prepared,
        origin,
        deadline,
        &mut |peer, packet, context, receiver, request_end, round_end| {
            setup_count += 1;
            latest_setup_cutoff = Some(request_end);
            if setup_count == 1 {
                first_cutoff = Some(request_end);
            }
            setup_address_request(peer, packet, context, receiver, request_end, round_end)
        },
        &mut |index, phase| {
            if index == 0 && phase == RoundCheckpoint::Before {
                first_transferred = Some(Instant::now());
            }
        },
    ));
    let returned = Instant::now();
    let first_seen = first_peer.finish();
    let second_seen = second_peer.finish();
    if let Err(error) = &second_seen {
        eprintln!(
            "ADDRESS_ROUND_PEER kind={:?} native_code={:?}",
            error.kind(),
            error.raw_os_error()
        );
    }
    let (report, first_seen) = joined_result(Ok(report), first_seen)?;
    let second_seen = second_seen?;
    check_receive_boundary_history(
        &report,
        invalid_count,
        &peers,
        deadline,
        returned,
        latest_setup_cutoff,
    )?;
    check_query(&require_query(first_seen)?);
    let cutoff = first_cutoff.ok_or_else(|| io::Error::other("actual first cutoff absent"))?;
    let transferred = first_transferred
        .ok_or_else(|| io::Error::other("actual first transfer notification absent"))?;
    if invalid_count == 64 {
        assert!(
            transferred >= cutoff,
            "Budget appeared before its original request cutoff"
        );
        let seen = require_query(second_seen)?;
        check_query(&seen);
        assert!(seen.received >= cutoff);
    } else {
        assert!(
            second_seen.is_none(),
            "valid sixty-fourth reply did not stop fallback"
        );
    }
    assert_eq!(setup_count, if invalid_count == 63 { 1 } else { 2 });
    assert_eq!(observe_queries(&third)?.0, 0);
    assert_eq!(runtime.metrics().num_alive_tasks(), 0);
    Ok(())
}

/// The sixty-fourth read may commit a reply; sixty-four rejections defer to cutoff.
/// # Errors
/// Preserve native fixture inability to establish the queue/deadline boundary.
#[test]
fn real_receive_budget_boundary_distinguishes_valid_sixty_fourth_from_all_rejected() -> Check<()> {
    for invalid_count in [63, 64] {
        receive_budget_case(invalid_count)?;
    }
    Ok(())
}

/// Admit only the exact first unsent Io intentionally injected at writable handling.
/// All other native/setup/peer/cleanup failures retain the ordinary classification.
/// # Errors
/// Preserve every actual failure outside the one explicitly admitted injection.
fn classify_expected_readiness_io(report: &AddressRoundReport, expected: IoCause) -> Check<()> {
    let only_expected_io = report
        .attempts()
        .iter()
        .enumerate()
        .all(|(index, attempt)| {
            let actual = attempt.report();
            actual.status() != Status::Io
                || (index == 0
                    && !actual.sent()
                    && actual.received() == 0
                    && actual.rejected() == 0
                    && actual.io_cause().is_some_and(|cause| {
                        cause.kind() == expected.kind()
                            && cause.native_code() == expected.native_code()
                    }))
        });
    // Keep the proven cursor-invariant primary for the history assertions below;
    // it cannot turn this exact deliberately injected Io into a native failure.
    if matches!(
        report.primary_error(),
        None | Some(PilotError::InternalState)
    ) && report.cleanup_errors().iter().all(Option::is_none)
        && only_expected_io
    {
        Ok(())
    } else {
        classify_native_report(report)
    }
}

/// Independently require the injected historical Io and the real next-address reply.
/// # Errors
/// Preserve absent expected records after native causes have been classified.
/// # Panics
/// Require exact cause, zero first send/reads and ordered unchanged-driver fallback.
fn check_readiness_io_history(
    report: &AddressRoundReport,
    expected: IoCause,
    peers: &[SocketAddr; 3],
    deadline: Instant,
    returned: Instant,
    latest_setup_cutoff: Option<Instant>,
    injection_window: Option<(Instant, Instant, Instant)>,
) -> Check<()> {
    let expired_injection = report
        .attempts()
        .first()
        .is_some_and(|first| first.report().status() == Status::Timeout)
        && injection_window.is_some_and(|(cutoff, _before, after)| after >= cutoff);
    if (report.end() == AddressRoundEnd::Deadline && returned >= deadline)
        || (report.end() == AddressRoundEnd::SetupDeadline
            && latest_setup_cutoff.is_some_and(|cutoff| returned >= cutoff))
        || expired_injection
    {
        if let Some((cutoff, before, after)) = injection_window {
            eprintln!(
                "ADDRESS_ROUND_INJECTION_WINDOW started_before_cutoff={} ended_before_cutoff={}",
                before < cutoff,
                after < cutoff
            );
        }
        return Err(io::Error::new(
            io::ErrorKind::TimedOut,
            "readiness-Io fixture exceeded its actual original cutoff",
        )
        .into());
    }
    assert_eq!(report.end(), AddressRoundEnd::Reply);
    assert!(report.primary_error().is_none());
    assert!(report.cleanup_errors().iter().all(Option::is_none));
    assert_eq!(report.attempts().len(), 2);
    let [first, second] = report.attempts() else {
        return Err(io::Error::other("readiness-Io history does not contain two attempts").into());
    };
    assert_eq!(first.peer(), peers[0]);
    assert_eq!(first.report().status(), Status::Io);
    assert!(!first.report().sent());
    assert_eq!(first.report().received(), 0);
    assert_eq!(first.report().rejected(), 0);
    assert!(first.report().io_cause().is_some());
    let cause = first
        .report()
        .io_cause()
        .ok_or_else(|| io::Error::other("injected writable cause absent"))?;
    assert_eq!(cause.kind(), expected.kind());
    assert_eq!(cause.native_code(), expected.native_code());
    assert_eq!(second.peer(), peers[1]);
    assert_eq!(second.report().status(), Status::Header);
    assert!(second.report().sent());
    assert_eq!(second.report().received(), 1);
    assert_eq!(second.report().rejected(), 0);
    Ok(())
}

/// Inject an actual captured refusal into writable handling on the real owned slot.
/// This is controlled error propagation, not reproduction of an OS readiness fault.
/// # Errors
/// Preserve all other native/setup/peer/cleanup failures before semantic assertions.
/// # Panics
/// Require unsent first Io, a genuine second reply and no first/third query.
#[test]
fn injected_writable_io_keeps_its_actual_cause_and_continues_to_the_real_reply() -> Check<()> {
    let first = bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?;
    let second = bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?;
    let third = bind_peer(IpAddr::V4(Ipv4Addr::LOCALHOST))?;
    let peers = [
        first.local_addr()?,
        second.local_addr()?,
        third.local_addr()?,
    ];
    let refusal = occupied_bind_refusal()?;
    let cause = IoCause::capture(&refusal);
    let mut refusal = Some(refusal);
    let (prepared, _control) = prepare_address_round(checked_config(&peers)?)?;
    let runtime = runtime()?;
    let origin = Instant::now();
    let deadline = origin
        .checked_add(Duration::from_secs(2))
        .ok_or(PilotError::DeadlineOverflow)?;
    let responder = ReplyPeer::spawn(second, ReplyKind::Ordinary)?;
    let mut calls = 0_u8;
    let mut injected = false;
    let mut latest_setup_cutoff = None;
    let mut injection_window = None;
    let report = runtime.block_on(run_round_with(
        prepared,
        origin,
        deadline,
        &mut |peer, packet, context, receiver, request_end, round_end| {
            calls += 1;
            latest_setup_cutoff = Some(request_end);
            let mut outcome =
                setup_address_request(peer, packet, context, receiver, request_end, round_end)?;
            if calls == 1
                && let SetupOutcome::Ready(slot) = &mut outcome
            {
                let error = refusal.take().ok_or(PilotError::InternalState)?;
                let before = Instant::now();
                guarded_writable(slot, receiver, Err(error), request_end);
                injection_window = Some((request_end, before, Instant::now()));
                injected = true;
            }
            Ok(outcome)
        },
        &mut |_, _| {},
    ));
    let returned = Instant::now();
    let seen = responder.finish();
    if let Err(error) = &seen {
        eprintln!(
            "ADDRESS_ROUND_PEER kind={:?} native_code={:?}",
            error.kind(),
            error.raw_os_error()
        );
    }
    classify_expected_readiness_io(&report, cause)?;
    let seen = seen?;
    check_readiness_io_history(
        &report,
        cause,
        &peers,
        deadline,
        returned,
        latest_setup_cutoff,
        injection_window,
    )?;
    assert!(
        injected,
        "the actual writable handler did not receive the captured refusal"
    );
    assert!(refusal.is_none());
    assert_eq!(calls, 2);
    check_query(&require_query(seen)?);
    assert_eq!(observe_queries(&first)?.0, 0);
    assert_eq!(observe_queries(&third)?.0, 0);
    assert_eq!(runtime.metrics().num_alive_tasks(), 0);
    Ok(())
}

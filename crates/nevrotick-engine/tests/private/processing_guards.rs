// SPDX-License-Identifier: GPL-2.0-only
//! Private current-context and terminal-outcome checks; no sample or AFD qualification.

use super::*;
use std::error::Error;

/// Maximum fixture I/O wait; unrelated to the processing deadline under test.
const IO_LIMIT: Duration = Duration::from_secs(2);

/// Independently expected lifecycle, payload and rejection-history outcomes.
#[derive(Clone, Copy)]
enum Guard {
    /// Receipt precedes the deadline, but fresh processing cannot precede it.
    Expired,
    /// A future deadline admits the same ordinary header.
    Future,
    /// A delivered stop must retire the still-pending slot.
    Stop,
    /// A delivered new generation must retire the still-pending slot.
    Invalidate,
    /// A correlated stratum-zero reply owns only its opaque control code.
    Control,
    /// A real origin rejection survives the later valid ordinary header.
    RejectedThenHeader,
}

/// Original bounded receive buffer and observations from its actual socket read.
struct Received {
    /// Original bounded buffer, with only the returned prefix meaningful.
    bytes: [u8; RECEIVE_BYTES],
    /// Actual copied byte count.
    length: usize,
    /// Actual source returned by the native receive.
    source: SocketAddr,
    /// Adjacent software readings captured after that receive returned.
    read: SoftwareClock,
}

impl Received {
    /// Retain this actual receive in the selected zero-or-one fixture slot.
    /// Every caller supplies its original constant association index.
    fn wake(&self, slot: usize) -> Wake {
        Wake::Read {
            slot,
            result: classify_read_result(
                NativeDatagramPlatform::current(),
                Ok((self.length, self.source)),
            ),
            read: self.read,
        }
    }
}

/// Two genuine privately prepared associations and their parent-owned peers.
struct Fixture {
    /// Nonblocking native client sockets with actual OS correlation nonces.
    prepared: [PreparedSlot; 2],
    /// Distinct peers whose endpoints passed the checked laboratory configuration.
    peers: [StdSocket; 2],
}

/// Construct only valid private inputs, without repeating public preparation.
///
/// # Errors
/// Preserve entropy/socket/configuration failures; never regenerate unusable nonces.
///
/// # Panics
/// Fail if the two actual prepared local endpoints alias.
fn fixture() -> Result<Fixture, Box<dyn Error>> {
    let peers = [
        StdSocket::bind("127.0.0.1:0")?,
        StdSocket::bind("127.0.0.1:0")?,
    ];
    let config = Config::new(
        [peers[0].local_addr()?, peers[1].local_addr()?],
        MAX_TIMEOUT,
    )?;
    let mut entropy = [0_u8; 16];
    getrandom::fill(&mut entropy)?;
    if entropy[..8] == [0_u8; 8] || entropy[8..] == [0_u8; 8] || entropy[..8] == entropy[8..] {
        return Err(io::Error::other("fixture nonce prerequisite unavailable").into());
    }
    let mut packets = [[0_u8; PACKET_BYTES]; 2];
    for (index, packet) in packets.iter_mut().enumerate() {
        packet[0] = 0x23;
        packet[40..48].copy_from_slice(&entropy[index * 8..index * 8 + 8]);
    }
    let prepared = [
        prepare_slot(config.peers[0], packets[0])?,
        prepare_slot(config.peers[1], packets[1])?,
    ];
    assert_ne!(prepared[0].local, prepared[1].local);
    for peer in &peers {
        peer.set_nonblocking(true)?;
    }
    Ok(Fixture { prepared, peers })
}

/// Observe one confirmed full query, retaining its actual native send bracket.
///
/// # Errors
/// Preserve bounded readiness/I/O failures or reject an unexpected actual query.
async fn send_query(
    slot: &mut LiveSlot,
    peer: &UdpSocket,
    deadline: Instant,
) -> Result<[u8; RECEIVE_BYTES], Box<dyn Error>> {
    Ok(tokio::time::timeout(IO_LIMIT, async {
        // An independent finite ceiling also bounds mutations that keep readiness
        // immediately ready forever; an async timeout cannot preempt that loop.
        for _ in 0..65 {
            if !slot.can_send() {
                break;
            }
            slot.socket.writable().await?;
            attempt_send(slot, deadline);
        }
        if slot.can_send() {
            return Err(io::Error::other(
                "fixture send exceeded sixty-five readiness decisions",
            ));
        }
        if slot.sent && !(1..=64).contains(&slot.send_attempts) {
            return Err(io::Error::other(
                "confirmed send lacks a bounded actual attempt count",
            ));
        }
        if !slot.sent || !matches!(&slot.phase, Phase::Pending(_)) {
            return Err(
                match slot
                    .terminal()
                    .and_then(TerminalOutcome::io_cause)
                    .and_then(IoCause::native_code)
                {
                    Some(code) => io::Error::from_raw_os_error(code),
                    None => io::Error::other("fixture query did not become pending"),
                },
            );
        }
        let mut query = [0_u8; RECEIVE_BYTES];
        let (length, source) = peer.recv_from(&mut query).await?;
        if length != PACKET_BYTES || source != slot.local || query[..length] != slot.packet {
            return Err(io::Error::other(
                "fixture peer observed an unexpected query",
            ));
        }
        Ok(query)
    })
    .await??)
}

/// Send a literal peer response and retain observations from its actual native read.
/// `request` is the previously observed immutable query; no new query is sent.
///
/// # Errors
/// Preserve the bounded response send/read failures or an anomalous send length.
async fn response(
    slot: &LiveSlot,
    peer: &UdpSocket,
    request: &[u8; RECEIVE_BYTES],
    stratum: u8,
    wrong_origin: bool,
) -> Result<Received, Box<dyn Error>> {
    Ok(tokio::time::timeout(IO_LIMIT, async {
        let mut bytes = [0_u8; RECEIVE_BYTES];
        bytes[0] = 0x24; // LI=0, VN=4, server mode.
        bytes[1] = stratum;
        bytes[12..16].copy_from_slice(&[1, 2, 3, 4]);
        bytes[24..32].copy_from_slice(&request[40..48]);
        if wrong_origin {
            bytes[24] ^= 1;
        }
        bytes[32..40].copy_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);
        bytes[40..48].copy_from_slice(&[9, 10, 11, 12, 13, 14, 15, 16]);
        if peer.send_to(&bytes[..PACKET_BYTES], slot.local).await? != PACKET_BYTES {
            return Err(io::Error::other("fixture reply was not fully sent"));
        }
        let (length, source) = slot.socket.recv_from(&mut bytes).await?;
        let read = SoftwareClock::capture();
        Ok(Received {
            bytes,
            length,
            source,
            read,
        })
    })
    .await??)
}

/// Send exactly one real query and return its first real literal response.
///
/// # Errors
/// Preserve actual query/response I/O and finite fixture timeout failures.
async fn receive_reply(
    slot: &mut LiveSlot,
    peer: &UdpSocket,
    deadline: Instant,
    stratum: u8,
    wrong_origin: bool,
) -> Result<(Received, [u8; RECEIVE_BYTES]), Box<dyn Error>> {
    let query = send_query(slot, peer, deadline).await?;
    Ok((
        response(slot, peer, &query, stratum, wrong_origin).await?,
        query,
    ))
}

/// Check only the selected slot's coupled outcome and independent rejection history.
///
/// # Errors
/// Reject a missing terminal result instead of assuming a manufactured success.
///
/// # Panics
/// Fail if status, payload or receive/rejection history differs from the case.
fn check_target(slot: &LiveSlot, guard: Guard, expected: Status) -> Result<(), Box<dyn Error>> {
    let target = slot
        .terminal()
        .ok_or_else(|| io::Error::other("target outcome absent"))?;
    assert_eq!(target.status(), expected);
    assert_eq!(
        slot.received,
        if matches!(guard, Guard::RejectedThenHeader) {
            2
        } else {
            1
        }
    );
    assert_eq!(
        slot.rejected,
        u16::from(!matches!(guard, Guard::Future | Guard::Control))
    );
    assert_eq!(
        target.stratum(),
        matches!(guard, Guard::Future | Guard::RejectedThenHeader).then_some(1)
    );
    assert_eq!(
        target.facts().is_some(),
        matches!(guard, Guard::Future | Guard::RejectedThenHeader)
    );
    assert_eq!(
        target.control_code(),
        matches!(guard, Guard::Control).then_some(0x0102_0304)
    );
    assert!(target.io_cause().is_none());
    assert_eq!(
        slot.last_rejection,
        matches!(guard, Guard::RejectedThenHeader).then_some(Rejection::Origin)
    );
    Ok(())
}

/// Check the target outcome and the neighbor's saved native/software observations.
/// This helper only borrows the owner; it performs no I/O or state transition.
///
/// # Errors
/// Return the fixed missing-facts failure if the committed facts disappeared.
///
/// # Panics
/// Fail if the target bypasses its guard or the neighbor's terminal status,
/// counters, send state, causes, clocks, processing time or literal bits changed.
fn check_outcomes(
    slots: &[LiveSlot; 2],
    guard: Guard,
    expected: Status,
    received_read: Instant,
    before: &TransportFacts,
    attempts: u16,
) -> Result<(), Box<dyn Error>> {
    check_target(&slots[0], guard, expected)?;
    let slot = &slots[1];
    let outcome = slot
        .terminal()
        .ok_or_else(|| io::Error::other("neighbor outcome absent"))?;
    assert_eq!(outcome.status(), Status::Header);
    assert_eq!(
        (slot.received, slot.rejected, outcome.stratum()),
        (1, 0, Some(1))
    );
    let facts = outcome
        .facts()
        .ok_or_else(|| io::Error::other("neighbor facts lost"))?;
    assert_eq!(facts.processing, before.processing);
    assert_eq!(
        [
            (facts.send_before.mono, facts.send_before.utc),
            (facts.send_after.mono, facts.send_after.utc),
            (facts.read_return.mono, facts.read_return.utc),
        ],
        [
            (before.send_before.mono, before.send_before.utc),
            (before.send_after.mono, before.send_after.utc),
            (before.read_return.mono, before.read_return.utc),
        ]
    );
    assert!(slot.sent);
    assert_eq!(slot.send_attempts, attempts);
    assert!(outcome.control_code().is_none());
    assert!(outcome.io_cause().is_none());
    assert!(slot.last_rejection.is_none());
    assert_eq!(facts.read_return.mono, received_read);
    assert_eq!(facts.peer_receive_bits, before.peer_receive_bits);
    assert_eq!(facts.peer_transmit_bits, before.peer_transmit_bits);
    assert_eq!(facts.peer_receive_bits, 0x0102_0304_0506_0708);
    assert_eq!(facts.peer_transmit_bits, 0x090a_0b0c_0d0e_0f10);
    Ok(())
}

/// Copy every immutable observation from an actual committed header for comparison.
///
/// # Errors
/// Return the fixed missing-header failure; never invent replacement facts.
fn snapshot_facts(slot: &LiveSlot) -> Result<TransportFacts, Box<dyn Error>> {
    let facts = slot
        .terminal()
        .and_then(TerminalOutcome::facts)
        .ok_or_else(|| io::Error::other("neighbor facts absent"))?;
    Ok(TransportFacts {
        send_before: facts.send_before,
        send_after: facts.send_after,
        read_return: facts.read_return,
        processing: facts.processing,
        peer_receive_bits: facts.peer_receive_bits,
        peer_transmit_bits: facts.peer_transmit_bits,
    })
}

/// Apply a guard to a valid stored read after committing its healthy neighbor.
/// Return the neighbor's earlier saved observations for public projection checks.
///
/// # Errors
/// Preserve bounded fixture I/O, checked-deadline, wire-setup, generation or missing-fact failure.
///
/// # Panics
/// Fail if current context is bypassed or historical neighbor facts change.
async fn check_guard(
    guard: Guard,
    slots: &mut [LiveSlot; 2],
    peers: &[UdpSocket; 2],
    send_deadline: Instant,
) -> Result<TransportFacts, Box<dyn Error>> {
    let (neighbor, _) = receive_reply(&mut slots[1], &peers[1], send_deadline, 1, false).await?;
    let mut buffers = [[0_u8; RECEIVE_BYTES], neighbor.bytes];
    let limits = FrameLimits::new(PACKET_BYTES, 0).map_err(|_| PilotError::WireSetup)?;
    let future = Instant::now()
        .checked_add(MAX_TIMEOUT)
        .ok_or_else(|| io::Error::other("fixture deadline overflow"))?;
    let (sender, mut receiver) = watch::channel(Control {
        stop: false,
        epoch: 0,
    });
    let control = ControlHandle { sender };
    let mut control_open = true;
    handle_wake(
        slots,
        &mut receiver,
        &mut control_open,
        neighbor.wake(1),
        &buffers,
        limits,
        future,
    );
    assert_eq!(
        slots[1].terminal().map(TerminalOutcome::status),
        Some(Status::Header)
    );
    let neighbor_snapshot = snapshot_facts(&slots[1])?;
    let neighbor_attempts = slots[1].send_attempts;
    // The serialized owner handles this read before selecting another one.
    let stratum = u8::from(!matches!(guard, Guard::Control));
    let (mut first, query) = receive_reply(
        &mut slots[0],
        &peers[0],
        send_deadline,
        stratum,
        matches!(guard, Guard::RejectedThenHeader),
    )
    .await?;
    buffers[0] = first.bytes;
    if matches!(guard, Guard::RejectedThenHeader) {
        handle_wake(
            slots,
            &mut receiver,
            &mut control_open,
            first.wake(0),
            &buffers,
            limits,
            future,
        );
        assert!(matches!(&slots[0].phase, Phase::Pending(_)));
        assert_eq!(slots[0].last_rejection, Some(Rejection::Origin));
        assert_eq!((slots[0].received, slots[0].rejected), (1, 1));
        first = response(&slots[0], &peers[0], &query, 1, false).await?;
        buffers[0] = first.bytes;
    }
    assert!(
        stage_reply(
            &slots[0],
            &first.bytes[..first.length],
            first.source,
            first.read,
            limits
        )
        .is_ok()
    );
    let (deadline, expected) = match guard {
        Guard::Expired => {
            let deadline = Instant::now();
            assert!(first.read.mono < deadline);
            (deadline, Status::Timeout)
        }
        Guard::Future | Guard::RejectedThenHeader => (future, Status::Header),
        Guard::Control => (future, Status::Control),
        Guard::Stop => {
            control.stop();
            (future, Status::Stopped)
        }
        Guard::Invalidate => {
            control.invalidate()?;
            (future, Status::Stale)
        }
    };
    handle_wake(
        slots,
        &mut receiver,
        &mut control_open,
        first.wake(0),
        &buffers,
        limits,
        deadline,
    );
    check_outcomes(
        slots,
        guard,
        expected,
        neighbor.read.mono,
        &neighbor_snapshot,
        neighbor_attempts,
    )?;
    Ok(neighbor_snapshot)
}

/// A valid read cannot bypass fresh processing time or current owner requests.
/// Every case first commits the other association as a healthy historical neighbor.
/// Private preparation uses real OS nonces and sockets, without calling public
/// once-per-executable preparation or inventing a pending/send bracket.
///
/// # Errors
/// Preserve entropy, loopback, registration, deadline and bounded fixture I/O failures.
///
/// # Panics
/// Fail if a valid literal packet cannot stage, a fresh guard is bypassed,
/// terminal neighbor facts change, saved read ordering fails, or prepared locals alias.
#[test]
fn processing_uses_fresh_deadline_and_current_control() -> Result<(), Box<dyn Error>> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .max_blocking_threads(2)
        .build()?;
    for guard in [
        Guard::Expired,
        Guard::Future,
        Guard::Stop,
        Guard::Invalidate,
        Guard::Control,
        Guard::RejectedThenHeader,
    ] {
        let Fixture { prepared, peers } = fixture()?;
        runtime.block_on(async {
            let [a, b] = prepared;
            let mut slots = [LiveSlot::from_prepared(a)?, LiveSlot::from_prepared(b)?];
            let [a, b] = peers;
            let peers = [UdpSocket::from_std(a)?, UdpSocket::from_std(b)?];
            let send_deadline = Instant::now()
                .checked_add(MAX_TIMEOUT)
                .ok_or_else(|| io::Error::other("fixture deadline overflow"))?;
            let saved_neighbor = check_guard(guard, &mut slots, &peers, send_deadline).await?;
            let locals = [slots[0].local, slots[1].local];
            let [a, b] = slots;
            let first_report = a.into_report(0)?;
            let neighbor_report = b.into_report(1)?;
            assert_eq!(first_report.slot(), 0);
            assert_eq!(first_report.local_endpoint(), locals[0]);
            assert_eq!(neighbor_report.slot(), 1);
            assert_eq!(neighbor_report.local_endpoint(), locals[1]);
            check_report_projection(&first_report, guard)?;
            assert_eq!(neighbor_report.status(), Status::Header);
            assert_eq!(neighbor_report.stratum(), Some(1));
            assert!(neighbor_report.facts().is_some());
            assert!(neighbor_report.control_code().is_none());
            assert!(neighbor_report.io_cause().is_none());
            let facts = neighbor_report
                .facts()
                .ok_or_else(|| io::Error::other("public neighbor facts absent"))?;
            check_fact_getters(facts, &saved_neighbor);
            Ok::<(), Box<dyn Error>>(())
        })?;
    }
    Ok(())
}

/// Compare public getters with the observations saved before the later control.
/// These actual software/native-read values establish no clock mapping or accuracy.
///
/// # Panics
/// Fail if a getter replaces, swaps or loses its original observation.
fn check_fact_getters(facts: &TransportFacts, saved: &TransportFacts) {
    for (actual, expected) in [
        (facts.send_before(), saved.send_before),
        (facts.send_after(), saved.send_after),
        (facts.read_return(), saved.read_return),
    ] {
        assert_eq!(actual.monotonic(), expected.mono);
        assert_eq!(actual.utc(), expected.utc);
    }
    assert_eq!(facts.processing(), saved.processing);
    assert_eq!(facts.peer_receive_bits(), 0x0102_0304_0506_0708);
    assert_eq!(facts.peer_transmit_bits(), 0x090a_0b0c_0d0e_0f10);
}

/// Require the unchanged public terminal getters after consuming the native owner.
///
/// # Errors
/// Return a fixed failure when an ordinary result lost its required payload.
///
/// # Panics
/// Fail if status and optional public projections differ from the independent case.
fn check_report_projection(report: &SlotReport, guard: Guard) -> Result<(), Box<dyn Error>> {
    let expected = match guard {
        Guard::Expired => Status::Timeout,
        Guard::Stop => Status::Stopped,
        Guard::Invalidate => Status::Stale,
        Guard::Control => Status::Control,
        Guard::Future | Guard::RejectedThenHeader => Status::Header,
    };
    assert_eq!(report.status(), expected);
    assert!(report.sent());
    assert_eq!(
        report.received(),
        if matches!(guard, Guard::RejectedThenHeader) {
            2
        } else {
            1
        }
    );
    assert_eq!(
        report.rejected(),
        u16::from(!matches!(guard, Guard::Future | Guard::Control))
    );
    assert_eq!(
        report.last_rejection(),
        matches!(guard, Guard::RejectedThenHeader).then_some(Rejection::Origin)
    );
    assert_eq!(
        report.control_code(),
        matches!(guard, Guard::Control).then_some(0x0102_0304)
    );
    assert!(report.io_cause().is_none());
    if expected == Status::Header {
        assert_eq!(report.stratum(), Some(1));
        let facts = report
            .facts()
            .ok_or_else(|| io::Error::other("report header facts absent"))?;
        assert_eq!(facts.peer_receive_bits(), 0x0102_0304_0506_0708);
        assert_eq!(facts.peer_transmit_bits(), 0x090a_0b0c_0d0e_0f10);
    } else {
        assert!(report.stratum().is_none());
        assert!(report.facts().is_none());
    }
    Ok(())
}

/// Distinct synthetic operation failures and an anomalous-send construction control.
/// None claims that the native socket actually produced the injected failure.
#[derive(Clone, Copy)]
enum IoCase {
    /// A failed readiness result before any actual send, retaining a raw code.
    NativeCode,
    /// A failed read after a confirmed real query, without a raw OS code.
    NativeWithoutCode,
    /// The private anomalous-length outcome deliberately has no native cause.
    AnomalousSend,
}

/// Inject only explicitly synthetic typed failures into genuine active slot inputs.
///
/// # Errors
/// Preserve real send/fixture failures; capture no localized error text in outcomes.
async fn inject_io_case(
    slots: &mut [LiveSlot; 2],
    peer: &UdpSocket,
    case: IoCase,
    deadline: Instant,
) -> Result<io::ErrorKind, Box<dyn Error>> {
    let limits = FrameLimits::new(PACKET_BYTES, 0).map_err(|_| PilotError::WireSetup)?;
    let (sender, mut receiver) = watch::channel(Control {
        stop: false,
        epoch: 0,
    });
    let _sender = sender;
    let mut control_open = true;
    let buffers = [[0_u8; RECEIVE_BYTES]; 2];
    let expected_kind = io::Error::from_raw_os_error(5).kind();
    match case {
        IoCase::NativeCode => handle_wake(
            slots,
            &mut receiver,
            &mut control_open,
            Wake::Writable {
                slot: 0,
                result: Err(io::Error::from_raw_os_error(5)),
            },
            &buffers,
            limits,
            deadline,
        ),
        IoCase::NativeWithoutCode => {
            let _query = send_query(&mut slots[0], peer, deadline).await?;
            handle_wake(
                slots,
                &mut receiver,
                &mut control_open,
                Wake::Read {
                    slot: 0,
                    result: classify_read_result(
                        NativeDatagramPlatform::current(),
                        Err(io::Error::other("synthetic read failure")),
                    ),
                    read: SoftwareClock::capture(),
                },
                &buffers,
                limits,
                deadline,
            );
        }
        IoCase::AnomalousSend => slots[0].finish(TerminalOutcome::Io(IoOutcome::ShortSend)),
    }
    Ok(expected_kind)
}

/// Require exact public I/O/cause projections and the independently retained send flag.
///
/// # Errors
/// Reject a lost captured cause, including a cause whose raw platform code is absent.
///
/// # Panics
/// Fail if terminal history, payload exclusions or neighboring retirement changed.
fn check_io_report(
    report: &SlotReport,
    neighbor: &SlotReport,
    case: IoCase,
    expected_kind: io::ErrorKind,
) -> Result<(), Box<dyn Error>> {
    assert_eq!(report.status(), Status::Io);
    assert_eq!(report.sent(), matches!(case, IoCase::NativeWithoutCode));
    assert_eq!((report.received(), report.rejected()), (0, 0));
    assert!(
        report.stratum().is_none() && report.facts().is_none() && report.control_code().is_none()
    );
    assert!(report.last_rejection().is_none());
    match case {
        IoCase::NativeCode => {
            let captured_error = report
                .io_cause()
                .ok_or_else(|| io::Error::other("coded cause lost"))?;
            assert_eq!(captured_error.native_code(), Some(5));
            assert_eq!(captured_error.kind(), expected_kind);
        }
        IoCase::NativeWithoutCode => {
            let captured_error = report
                .io_cause()
                .ok_or_else(|| io::Error::other("uncoded cause lost"))?;
            assert_eq!(captured_error.native_code(), None);
            assert_eq!(captured_error.kind(), io::ErrorKind::Other);
        }
        IoCase::AnomalousSend => assert!(report.io_cause().is_none()),
    }
    assert_eq!(neighbor.status(), Status::Stopped);
    assert!(!neighbor.sent());
    assert!(
        neighbor.stratum().is_none()
            && neighbor.facts().is_none()
            && neighbor.control_code().is_none()
            && neighbor.io_cause().is_none()
    );
    Ok(())
}

/// Observe coupled I/O projections and one-way history before/after actual sending.
/// A short UDP send is not induced; that variant is a typed construction control.
///
/// # Errors
/// Preserve actual fixture setup/send/registration failures and absent terminal state.
///
/// # Panics
/// Fail if cause absence is confused with no cause, sent history changes, or
/// later retirement overwrites the terminal I/O outcome or its public projections.
#[test]
fn terminal_io_causes_and_send_history_remain_distinct() -> Result<(), Box<dyn Error>> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .max_blocking_threads(2)
        .build()?;
    for case in [
        IoCase::NativeCode,
        IoCase::NativeWithoutCode,
        IoCase::AnomalousSend,
    ] {
        let Fixture { prepared, peers } = fixture()?;
        runtime.block_on(async {
            let [a, b] = prepared;
            let mut slots = [LiveSlot::from_prepared(a)?, LiveSlot::from_prepared(b)?];
            let [a, b] = peers;
            let peers = [UdpSocket::from_std(a)?, UdpSocket::from_std(b)?];
            let deadline = Instant::now()
                .checked_add(MAX_TIMEOUT)
                .ok_or_else(|| io::Error::other("fixture deadline overflow"))?;
            let expected_kind = inject_io_case(&mut slots, &peers[0], case, deadline).await?;
            let outcome = slots[0]
                .terminal()
                .ok_or_else(|| io::Error::other("I/O outcome absent"))?;
            assert!(match case {
                IoCase::NativeCode | IoCase::NativeWithoutCode =>
                    matches!(outcome, TerminalOutcome::Io(IoOutcome::Native(_))),
                IoCase::AnomalousSend =>
                    matches!(outcome, TerminalOutcome::Io(IoOutcome::ShortSend)),
            });
            retire_for_control(
                &mut slots,
                Control {
                    stop: true,
                    epoch: 0,
                },
            );
            slots[0].expire();
            let [a, b] = slots;
            let report = a.into_report(0)?;
            let neighbor = b.into_report(1)?;
            check_io_report(&report, &neighbor, case, expected_kind)?;
            Ok::<(), Box<dyn Error>>(())
        })?;
    }
    Ok(())
}

/// Direct construction rejects invalid scope/flow and exact timeout neighbors.
/// No entropy, sockets or runtime are needed for this public admission path.
///
/// # Errors
/// Preserve unexpected construction failures for the independently valid neighbors.
///
/// # Panics
/// Fail if raw invalid endpoints/timeouts bypass construction or valid values change.
#[test]
fn configuration_enforces_endpoint_and_timeout_boundaries() -> Result<(), Box<dyn Error>> {
    let peers = [
        SocketAddr::from(([127, 0, 0, 1], 41001)),
        SocketAddr::from(([127, 0, 0, 1], 41002)),
    ];
    for timeout in [Duration::from_nanos(1), Duration::from_secs(30)] {
        let admitted = Config::new(peers, timeout)?;
        assert_eq!(admitted.peers, peers);
        assert_eq!(admitted.timeout, timeout);
    }
    for timeout in [Duration::ZERO, Duration::new(30, 1)] {
        assert!(matches!(
            Config::new(peers, timeout),
            Err(PilotError::InvalidTimeout)
        ));
    }
    assert!(matches!(
        Config::new([peers[0], peers[0]], Duration::from_secs(1)),
        Err(PilotError::DuplicatePeer)
    ));
    for bad in [
        SocketAddr::from(([192, 0, 2, 1], 41003)),
        SocketAddr::from(([127, 0, 0, 1], 0)),
    ] {
        for position in 0..2 {
            let mut input = peers;
            input[position] = bad;
            assert!(matches!(
                Config::new(input, Duration::from_secs(1)),
                Err(PilotError::InvalidPeer)
            ));
        }
    }
    let v6 = [
        SocketAddr::from(std::net::SocketAddrV6::new(
            Ipv6Addr::LOCALHOST,
            41001,
            0,
            0,
        )),
        SocketAddr::from(std::net::SocketAddrV6::new(
            Ipv6Addr::LOCALHOST,
            41002,
            0,
            0,
        )),
    ];
    assert_eq!(Config::new(v6, Duration::from_secs(1))?.peers, v6);
    for bad in [
        SocketAddr::from(std::net::SocketAddrV6::new(
            Ipv6Addr::LOCALHOST,
            41003,
            0,
            1,
        )),
        SocketAddr::from(std::net::SocketAddrV6::new(
            Ipv6Addr::LOCALHOST,
            41003,
            1,
            0,
        )),
    ] {
        for position in 0..2 {
            let mut input = v6;
            input[position] = bad;
            assert!(matches!(
                Config::new(input, Duration::from_secs(1)),
                Err(PilotError::InvalidPeer)
            ));
        }
    }
    Ok(())
}

/// Zero control delays and exact thirty-second delays are legal; either excess rejects.
/// Construction itself starts no timer and changes no watch state.
///
/// # Errors
/// Preserve an unexpected rejection of a legal boundary value.
///
/// # Panics
/// Fail if either option loses its value or its exclusive upper neighbor is admitted.
#[test]
fn control_schedule_preserves_legal_zero_and_maximum_delays() -> Result<(), Box<dyn Error>> {
    for (stop, invalidate) in [
        (None, None),
        (Some(Duration::ZERO), None),
        (None, Some(Duration::ZERO)),
        (Some(Duration::from_secs(30)), None),
        (None, Some(Duration::from_secs(30))),
        (Some(Duration::from_secs(30)), Some(Duration::from_secs(30))),
    ] {
        let schedule = ControlSchedule::new(stop, invalidate)?;
        assert_eq!(schedule.stop_after, stop);
        assert_eq!(schedule.invalidate_after, invalidate);
    }
    for (stop, invalidate) in [
        (Some(Duration::new(30, 1)), None),
        (None, Some(Duration::new(30, 1))),
        (Some(Duration::ZERO), Some(Duration::new(30, 1))),
        (Some(Duration::new(30, 1)), Some(Duration::ZERO)),
    ] {
        assert!(matches!(
            ControlSchedule::new(stop, invalidate),
            Err(PilotError::InvalidTimeout)
        ));
    }
    Ok(())
}

/// Real watch delivery is latched across clones; overflow stops without wrapping.
/// The near-maximum initial epoch is a private representation fixture, not a claim
/// that the test delivered nearly 2^64 prior invalidations or acknowledged cleanup.
///
/// # Errors
/// Preserve an unexpected failure of a legal generation increment.
///
/// # Panics
/// Fail if delivery loses a stop, mutates an already stopped epoch or wraps it.
#[test]
fn watch_controls_latch_stop_and_reject_generation_wrap() -> Result<(), Box<dyn Error>> {
    let (sender, mut receiver) = watch::channel(Control {
        stop: false,
        epoch: 0,
    });
    let handle = ControlHandle { sender };
    let cloned = handle.clone();
    handle.invalidate()?;
    let state = latest(&mut receiver);
    assert_eq!((state.stop, state.epoch), (false, 1));
    cloned.stop();
    handle.invalidate()?;
    handle.stop();
    let state = latest(&mut receiver);
    assert_eq!((state.stop, state.epoch), (true, 1));

    let (sender, mut receiver) = watch::channel(Control {
        stop: false,
        epoch: u64::MAX - 1,
    });
    let handle = ControlHandle { sender };
    handle.invalidate()?;
    let state = latest(&mut receiver);
    assert_eq!((state.stop, state.epoch), (false, u64::MAX));
    assert!(matches!(
        handle.invalidate(),
        Err(PilotError::GenerationExhausted)
    ));
    let state = latest(&mut receiver);
    assert_eq!((state.stop, state.epoch), (true, u64::MAX));
    handle.invalidate()?;
    let state = latest(&mut receiver);
    assert_eq!((state.stop, state.epoch), (true, u64::MAX));
    Ok(())
}

/// The synthetic native error contains text that no typed projection may retain.
const REDACTION_SECRET: &str = "test-only secret cookie=must-not-appear\n";

/// Require literal public codes, safe formatting and preserved bounded native fields.
/// This checks a synthetic error representation, not an induced native OS failure.
///
/// # Errors
/// Reject loss of a required captured native category without manufacturing a cause.
///
/// # Panics
/// Fail if public code, optional cause or formatting differs from the fixed contract.
fn check_failure_projection(
    error: PilotError,
    code: &str,
    native: Option<io::ErrorKind>,
) -> Result<(), Box<dyn Error>> {
    let summary = FailureSummary::capture(error);
    assert_eq!(error.as_str(), code);
    assert_eq!(format!("{error}"), code);
    assert_eq!(summary.code(), code);
    if let Some(kind) = native {
        let captured = summary
            .native_cause()
            .ok_or_else(|| io::Error::other("summary cause lost"))?;
        assert_eq!(captured.kind(), kind);
        assert_eq!(captured.native_code(), None);
    } else {
        assert!(summary.native_cause().is_none());
    }
    assert!(!format!("{error:?} {summary:?}").contains(REDACTION_SECRET));
    assert!(error.source().is_none());
    Ok(())
}

/// Typed errors retain safe categories and distinct cleanup summaries, never secret text.
/// Every operation failure here is synthetic; no real bind/runtime fault is claimed.
///
/// # Errors
/// Preserve missing required summary data instead of treating it as successful capture.
///
/// # Panics
/// Fail if codes change, a bounded cause is erased or cleanup summaries are conflated.
#[test]
fn failure_summaries_preserve_fixed_redacted_cause_projections() -> Result<(), Box<dyn Error>> {
    let original = io::Error::new(io::ErrorKind::PermissionDenied, REDACTION_SECRET);
    let cause = IoCause::capture(&original);
    let native = Some(io::ErrorKind::PermissionDenied);
    let cleanup = PilotError::ControlCleanup {
        primary: Some(FailureSummary::capture(PilotError::Bind(cause))),
        stop: Some(FailureSummary::capture(PilotError::ControlTask)),
        invalidate: Some(FailureSummary::capture(PilotError::GenerationExhausted)),
    };
    for (error, code, expected_native) in [
        (PilotError::InvalidPeer, "invalid_peer", None),
        (PilotError::DuplicatePeer, "duplicate_peer", None),
        (PilotError::InvalidTimeout, "invalid_timeout", None),
        (PilotError::Entropy, "entropy_failure", None),
        (PilotError::ZeroToken, "zero_random_token", None),
        (PilotError::DuplicateToken, "duplicate_random_token", None),
        (PilotError::Bind(cause), "bind_io", native),
        (PilotError::Nonblocking(cause), "nonblocking_io", native),
        (PilotError::LocalAddress(cause), "local_address_io", native),
        (PilotError::Registration(cause), "registration_io", native),
        (
            PilotError::DuplicateLocalAddress,
            "duplicate_local_address",
            None,
        ),
        (PilotError::DeadlineOverflow, "deadline_overflow", None),
        (
            PilotError::GenerationExhausted,
            "generation_exhausted",
            None,
        ),
        (PilotError::InternalState, "internal_state", None),
        (PilotError::WireSetup, "wire_setup", None),
        (PilotError::EnteredRuntime, "entered_runtime", None),
        (PilotError::RuntimeBuild(cause), "runtime_build", native),
        (PilotError::ControlTask, "control_task", None),
        (cleanup, "control_cleanup", None),
    ] {
        check_failure_projection(error, code, expected_native)?;
    }
    let PilotError::ControlCleanup {
        primary: Some(primary),
        stop: Some(stop),
        invalidate: Some(invalidate),
    } = cleanup
    else {
        return Err(io::Error::other("cleanup fixture summaries absent").into());
    };
    assert_eq!(
        (primary.code(), stop.code(), invalidate.code()),
        ("bind_io", "control_task", "generation_exhausted")
    );
    assert_eq!(primary.native_cause().map(IoCause::kind), native);
    assert!(stop.native_cause().is_none() && invalidate.native_cause().is_none());
    Ok(())
}

/// Immediate actions deliver directly; future jobs are cancelled and reaped without sleeping.
/// This is owned Tokio-task behavior, not Windows native completion or retention evidence.
///
/// # Errors
/// Preserve runtime/setup/deadline or bounded timer-cleanup errors.
///
/// # Panics
/// Fail if immediate actions allocate jobs or a cancelled future changes its watch value.
#[test]
fn owned_timers_deliver_immediate_actions_and_cancel_future_actions() -> Result<(), Box<dyn Error>>
{
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .max_blocking_threads(2)
        .build()?;
    runtime.block_on(async {
        for action in [TimerAction::Stop, TimerAction::Invalidate] {
            let (sender, mut receiver) = watch::channel(Control {
                stop: false,
                epoch: 0,
            });
            let handle = ControlHandle { sender };
            assert!(start_timer(None, handle.clone(), action)?.is_none());
            let state = latest(&mut receiver);
            assert_eq!((state.stop, state.epoch), (false, 0));
            assert!(start_timer(Some(Instant::now()), handle, action)?.is_none());
            let state = latest(&mut receiver);
            assert_eq!(
                (state.stop, state.epoch),
                if matches!(action, TimerAction::Stop) {
                    (true, 0)
                } else {
                    (false, 1)
                }
            );

            let (sender, mut receiver) = watch::channel(Control {
                stop: false,
                epoch: 0,
            });
            let handle = ControlHandle { sender };
            let later = Instant::now()
                .checked_add(Duration::from_secs(30))
                .ok_or_else(|| io::Error::other("future timer deadline overflow"))?;
            let Some(job) = start_timer(Some(later), handle, action)? else {
                return Err(io::Error::other("future timer has no owned job").into());
            };
            let completion = job.abort_handle();
            tokio::time::timeout(IO_LIMIT, finish_timer(Some(job))).await??;
            assert!(completion.is_finished());
            let state = latest(&mut receiver);
            assert_eq!((state.stop, state.epoch), (false, 0));
        }
        Ok::<(), Box<dyn Error>>(())
    })
}

/// Mutate only literal header fields while retaining the real request/source/read.
/// The deliberately earlier software observation is a synthetic invalid boundary
/// input; it is not described as an actual reversal of the native read clock.
///
/// # Errors
/// Preserve missing pending ownership or an unavailable checked predecessor instant.
///
/// # Panics
/// Fail if an exact rejection category differs or the actual healthy neighbor fails.
fn check_literal_stage_rejections(
    slot: &LiveSlot,
    received: &Received,
    limits: FrameLimits,
) -> Result<(), Box<dyn Error>> {
    for (offset, value, expected) in [
        (0, 0x1c, Rejection::VersionOrMode), // VN=3, server.
        (0, 0x23, Rejection::VersionOrMode), // VN=4, client.
        (0, 0xe4, Rejection::Quality),       // LI=3, VN=4, server.
        (1, 16, Rejection::Quality),
    ] {
        let mut bytes = received.bytes;
        bytes[offset] = value;
        assert!(matches!(
            stage_reply(slot, &bytes[..received.length], received.source, received.read, limits),
            Err(reason) if reason == expected
        ));
    }
    for (start, end) in [(32, 40), (40, 48)] {
        let mut bytes = received.bytes;
        bytes[start..end].fill(0);
        assert!(matches!(
            stage_reply(
                slot,
                &bytes[..received.length],
                received.source,
                received.read,
                limits
            ),
            Err(Rejection::Timestamp)
        ));
    }
    let Phase::Pending(sent) = &slot.phase else {
        return Err(io::Error::other("literal fixture lost actual pending send").into());
    };
    let earlier = sent
        .after
        .mono
        .checked_sub(Duration::from_nanos(1))
        .ok_or_else(|| io::Error::other("send observation has no checked predecessor"))?;
    assert!(matches!(
        stage_reply(
            slot,
            &received.bytes[..received.length],
            received.source,
            SoftwareClock {
                mono: earlier,
                utc: received.read.utc
            },
            limits
        ),
        Err(Rejection::ReadBeforeSend)
    ));
    // Equality is a synthetic exact boundary using the saved actual send instant;
    // it does not claim the native read actually returned at that instant.
    assert!(
        stage_reply(
            slot,
            &received.bytes[..received.length],
            received.source,
            SoftwareClock {
                mono: sent.after.mono,
                utc: received.read.utc
            },
            limits
        )
        .is_ok()
    );
    assert!(
        stage_reply(
            slot,
            &received.bytes[..received.length],
            received.source,
            received.read,
            limits
        )
        .is_ok()
    );
    assert!(matches!(&slot.phase, Phase::Pending(_)));
    assert_eq!((slot.received, slot.rejected), (0, 0));
    Ok(())
}

/// A genuine pending query/read supplies the valid neighbor for hostile raw fields.
/// Staging invalid copied inputs has no socket effect; only the original actual
/// received datagram is later charged and committed through the real wake handler.
///
/// # Errors
/// Preserve bounded native fixture I/O, registration, parser setup or missing report errors.
///
/// # Panics
/// Fail if any literal rejection changes or it prevents the original healthy commit.
#[test]
fn literal_header_rejections_preserve_the_actual_pending_neighbor() -> Result<(), Box<dyn Error>> {
    let Fixture { prepared, peers } = fixture()?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .max_blocking_threads(2)
        .build()?;
    runtime.block_on(async {
        let [a, b] = prepared;
        let mut slots = [LiveSlot::from_prepared(a)?, LiveSlot::from_prepared(b)?];
        let [peer, _other_peer] = peers;
        let peer = UdpSocket::from_std(peer)?;
        let deadline = Instant::now()
            .checked_add(Duration::from_secs(30))
            .ok_or_else(|| io::Error::other("literal fixture deadline overflow"))?;
        let (datagram, _) = receive_reply(&mut slots[0], &peer, deadline, 1, false).await?;
        let limits = FrameLimits::new(48, 0).map_err(|_| PilotError::WireSetup)?;
        check_literal_stage_rejections(&slots[0], &datagram, limits)?;
        let (sender, mut receiver) = watch::channel(Control {
            stop: false,
            epoch: 0,
        });
        let _sender = sender;
        let mut control_open = true;
        let buffers = [datagram.bytes, [0_u8; RECEIVE_BYTES]];
        handle_wake(
            &mut slots,
            &mut receiver,
            &mut control_open,
            datagram.wake(0),
            &buffers,
            limits,
            deadline,
        );
        let [completed, unsent_neighbor] = slots;
        let report = completed.into_report(0)?;
        check_report_projection(&report, Guard::Future)?;
        assert!(matches!(&unsent_neighbor.phase, Phase::Sending));
        Ok::<(), Box<dyn Error>>(())
    })
}

/// Literal buffers exercise post-RNG admission without claiming an entropy failure.
/// Every successful packet byte is compared with an independently written request.
///
/// # Errors
/// Preserve rejection of the independently valid nonzero distinct halves.
///
/// # Panics
/// Fail if one zero half, equal halves, token placement or other request bytes change.
#[test]
fn entropy_admission_checks_each_half_and_exact_request_bytes() -> Result<(), Box<dyn Error>> {
    let valid = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    for entropy in [
        [0, 0, 0, 0, 0, 0, 0, 0, 9, 10, 11, 12, 13, 14, 15, 16],
        [1, 2, 3, 4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0],
        [0; 16],
    ] {
        assert!(matches!(
            request_packets(&entropy),
            Err(PilotError::ZeroToken)
        ));
    }
    assert!(matches!(
        request_packets(&[1; 16]),
        Err(PilotError::DuplicateToken)
    ));
    let actual = request_packets(&valid)?;
    let mut expected = [[0_u8; 48]; 2];
    expected[0][0] = 0x23;
    expected[1][0] = 0x23;
    expected[0][40..48].copy_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);
    expected[1][40..48].copy_from_slice(&[9, 10, 11, 12, 13, 14, 15, 16]);
    assert_eq!(actual, expected);
    assert_eq!(
        valid,
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
    );
    Ok(())
}

/// Synthetic endpoint values exercise the defensive check of an actual bind result.
/// Passing this pure helper alone establishes no OS socket/address ownership.
///
/// # Errors
/// Preserve rejection of the independently valid IPv4/IPv6 loopback neighbors.
///
/// # Panics
/// Fail if either invalid predicate alone bypasses admission or an endpoint changes.
#[test]
fn local_endpoint_admission_requires_both_loopback_and_nonzero_port() -> Result<(), Box<dyn Error>>
{
    for endpoint in [
        SocketAddr::from(([127, 0, 0, 1], 1)),
        SocketAddr::from(([127, 0, 0, 2], 65535)),
        SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 123),
    ] {
        assert_eq!(checked_local_endpoint(endpoint)?, endpoint);
    }
    for endpoint in [
        SocketAddr::from(([127, 0, 0, 1], 0)),
        SocketAddr::from(([192, 0, 2, 1], 123)),
        SocketAddr::from(([192, 0, 2, 1], 0)),
        SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 0),
        SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 123),
    ] {
        assert!(matches!(
            checked_local_endpoint(endpoint),
            Err(PilotError::InvalidPeer)
        ));
    }
    Ok(())
}

/// Explicit platform alternatives classify synthetic native-code representations.
/// No error is induced in an OS receive; Windows behavior still needs native evidence.
///
/// # Errors
/// Reject a result variant that lost its required bounded native cause.
///
/// # Panics
/// Fail if success metadata changes, uncoded/neighbor errors imply consumption,
/// or a non-Windows 10040 is incorrectly treated as a consumed datagram.
#[test]
fn read_classification_keeps_platform_oversize_and_native_errors_distinct()
-> Result<(), Box<dyn Error>> {
    #[cfg(windows)]
    assert!(matches!(
        NativeDatagramPlatform::current(),
        NativeDatagramPlatform::Windows
    ));
    #[cfg(not(windows))]
    assert!(matches!(
        NativeDatagramPlatform::current(),
        NativeDatagramPlatform::Other
    ));
    let source = SocketAddr::from(([127, 0, 0, 1], 41001));
    for platform in [
        NativeDatagramPlatform::Windows,
        NativeDatagramPlatform::Other,
    ] {
        assert!(matches!(classify_read_result(platform, Ok((49, source))),
            NativeReadOutcome::Datagram { length: 49, source: actual } if actual == source));
        for code in [10039, 10040, 10041] {
            let expected_kind = io::Error::from_raw_os_error(code).kind();
            let result = classify_read_result(platform, Err(io::Error::from_raw_os_error(code)));
            if matches!(platform, NativeDatagramPlatform::Windows) && code == 10040 {
                assert!(matches!(result, NativeReadOutcome::ConsumedOversize));
            } else {
                let NativeReadOutcome::Io(cause) = result else {
                    return Err(io::Error::other(
                        "synthetic ordinary error was treated as datagram consumption",
                    )
                    .into());
                };
                assert_eq!(cause.native_code(), Some(code));
                assert_eq!(cause.kind(), expected_kind);
            }
        }
        let result = classify_read_result(
            platform,
            Err(io::Error::other("synthetic uncoded receive error")),
        );
        let NativeReadOutcome::Io(cause) = result else {
            return Err(io::Error::other(
                "uncoded synthetic error was treated as datagram consumption",
            )
            .into());
        };
        assert_eq!(cause.native_code(), None);
        assert_eq!(cause.kind(), io::ErrorKind::Other);
    }
    Ok(())
}

/// Check one synthetic completion against fixed lifecycle/bracket expectations.
///
/// # Errors
/// Reject a lost captured cause or pending bracket instead of assuming success.
///
/// # Panics
/// Fail if `WouldBlock` retires, a short send rearms, full send loses its bracket,
/// native causes disappear, or a later completion rewrites pending/terminal state.
fn check_send_completion(
    slot: &mut LiveSlot,
    scenario: u8,
    bracket: SendBracket,
) -> Result<(), Box<dyn Error>> {
    let result = match scenario {
        0 => Err(io::Error::new(
            io::ErrorKind::WouldBlock,
            "synthetic readiness race",
        )),
        1 => Err(io::Error::from_raw_os_error(5)),
        2 => Ok(0),
        3 => Ok(47),
        4 => Ok(49),
        _ => Ok(48),
    };
    complete_send(slot, result, bracket);
    assert_eq!(
        (slot.received, slot.rejected, slot.send_attempts),
        (0, 0, 0)
    );
    match scenario {
        0 => assert!(slot.can_send() && !slot.sent && slot.terminal().is_none()),
        1 => {
            let cause = slot
                .terminal()
                .and_then(TerminalOutcome::io_cause)
                .ok_or_else(|| io::Error::other("synthetic completed send lost native cause"))?;
            assert_eq!(cause.native_code(), Some(5));
            assert_eq!(cause.kind(), io::Error::from_raw_os_error(5).kind());
            assert!(!slot.sent && !slot.can_send());
        }
        2..=4 => {
            assert!(matches!(
                slot.terminal(),
                Some(TerminalOutcome::Io(IoOutcome::ShortSend))
            ));
            assert!(!slot.sent && !slot.can_send());
        }
        _ => {
            let Phase::Pending(saved) = &slot.phase else {
                return Err(io::Error::other("full completion did not become pending").into());
            };
            assert!(slot.sent && !slot.can_send());
            assert_eq!(saved.before.mono, bracket.before.mono);
            assert_eq!(saved.before.utc, bracket.before.utc);
            assert_eq!(saved.after.mono, bracket.after.mono);
            assert_eq!(saved.after.utc, bracket.after.utc);
            complete_send(slot, Ok(0), bracket);
            assert!(matches!(&slot.phase, Phase::Pending(_)));
        }
    }
    if scenario != 0 {
        complete_send(slot, Ok(48), bracket);
        assert_eq!(slot.sent, scenario == 5);
        if scenario != 5 {
            assert!(slot.done());
        }
    }
    Ok(())
}

/// Synthetic completed results exercise the mandatory send acknowledgement owner.
/// Its locally constructed software bracket is not claimed as a native send;
/// existing real-query fixtures independently exercise the actual send path.
///
/// # Errors
/// Preserve genuine fixture socket/registration/runtime or missing-payload failures.
///
/// # Panics
/// Fail if a completion reopens or loses its independently expected lifecycle state.
#[test]
fn completed_send_results_preserve_rearm_causes_and_one_way_history() -> Result<(), Box<dyn Error>>
{
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .max_blocking_threads(2)
        .build()?;
    for scenario in 0..6 {
        let Fixture {
            prepared,
            peers: _peers,
        } = fixture()?;
        runtime.block_on(async {
            let [prepared, _neighbor] = prepared;
            let mut slot = LiveSlot::from_prepared(prepared)?;
            let bracket = SendBracket {
                before: SoftwareClock::capture(),
                after: SoftwareClock::capture(),
            };
            check_send_completion(&mut slot, scenario, bracket)
        })?;
    }
    Ok(())
}

/// Commit one real ordinary neighbor before another slot's adversarial work.
/// The fixed literal stratum-one peer packet is independently received and staged.
///
/// # Errors
/// Preserve bounded actual I/O, wire setup or missing committed facts.
///
/// # Panics
/// Fail if the independent neighbor cannot commit as an ordinary header.
async fn commit_neighbor(
    slots: &mut [LiveSlot; 2],
    peer: &UdpSocket,
    receiver: &mut watch::Receiver<Control>,
    deadline: Instant,
) -> Result<TransportFacts, Box<dyn Error>> {
    let (datagram, _) = receive_reply(&mut slots[1], peer, deadline, 1, false).await?;
    let limits = FrameLimits::new(48, 0).map_err(|_| PilotError::WireSetup)?;
    let mut open = true;
    handle_wake(
        slots,
        receiver,
        &mut open,
        datagram.wake(1),
        &[[0_u8; RECEIVE_BYTES], datagram.bytes],
        limits,
        deadline,
    );
    assert_eq!(
        slots[1].terminal().map(TerminalOutcome::status),
        Some(Status::Header)
    );
    assert_eq!((slots[1].received, slots[1].rejected), (1, 0));
    snapshot_facts(&slots[1])
}

/// Read exactly the finite supplied count of real wrong-origin datagrams.
/// Sixty-three remains selectable; sixty-four waits for owner retirement as Budget.
///
/// # Errors
/// Preserve finite native I/O, parser setup, registration and missing facts.
///
/// # Panics
/// Fail if either read boundary, pending state or committed neighbor changes.
async fn check_receive_cap(
    prepared: [PreparedSlot; 2],
    peers: [StdSocket; 2],
    count: u16,
) -> Result<(), Box<dyn Error>> {
    let [a, b] = prepared;
    let mut slots = [LiveSlot::from_prepared(a)?, LiveSlot::from_prepared(b)?];
    assert!(!slots[0].can_receive());
    let [a, b] = peers;
    let peers = [UdpSocket::from_std(a)?, UdpSocket::from_std(b)?];
    let deadline = Instant::now()
        .checked_add(MAX_TIMEOUT)
        .ok_or(PilotError::DeadlineOverflow)?;
    let (sender, mut receiver) = watch::channel(Control {
        stop: false,
        epoch: 0,
    });
    let _sender = sender;
    let saved_neighbor = commit_neighbor(&mut slots, &peers[1], &mut receiver, deadline).await?;
    assert!(!slots[1].can_receive());
    let query = send_query(&mut slots[0], &peers[0], deadline).await?;
    let limits = FrameLimits::new(48, 0).map_err(|_| PilotError::WireSetup)?;
    let mut open = true;
    for selected in 1..=count {
        assert!(slots[0].can_receive());
        let datagram = response(&slots[0], &peers[0], &query, 1, true).await?;
        handle_wake(
            &mut slots,
            &mut receiver,
            &mut open,
            datagram.wake(0),
            &[datagram.bytes, [0_u8; RECEIVE_BYTES]],
            limits,
            deadline,
        );
        assert_eq!((slots[0].received, slots[0].rejected), (selected, selected));
        assert_eq!(slots[0].last_rejection, Some(Rejection::Origin));
        assert!(matches!(&slots[0].phase, Phase::Pending(_)));
    }
    assert_eq!(slots[0].can_receive(), count == 63);
    slots[0].expire();
    assert_eq!(
        slots[0].terminal().map(TerminalOutcome::status),
        Some(if count == 63 {
            Status::Timeout
        } else {
            Status::Budget
        })
    );
    check_fact_getters(&snapshot_facts(&slots[1])?, &saved_neighbor);
    assert_eq!((slots[1].received, slots[1].rejected), (1, 0));
    Ok(())
}

/// Real selected receives distinguish the last legal read from exhausted ownership.
///
/// # Errors
/// Preserve runtime/setup and finite real peer I/O failures.
///
/// # Panics
/// Fail if count63/64 selection, Budget retirement or the healthy neighbor differs.
#[test]
fn receive_cap_preserves_sixty_three_and_retires_sixty_four() -> Result<(), Box<dyn Error>> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .max_blocking_threads(2)
        .build()?;
    for count in [63, 64] {
        let Fixture { prepared, peers } = fixture()?;
        runtime.block_on(check_receive_cap(prepared, peers, count))?;
    }
    Ok(())
}

/// Establish two real prior origin rejections without replacing the single query.
/// The returned query was independently observed by this fixture's owned peer.
///
/// # Errors
/// Preserve bounded native send/read and fixed wire-setup failures.
///
/// # Panics
/// Fail if either actual rejection is lost or retires the pending association.
async fn two_origin_rejections(
    slots: &mut [LiveSlot; 2],
    peer: &UdpSocket,
    receiver: &mut watch::Receiver<Control>,
    deadline: Instant,
) -> Result<[u8; RECEIVE_BYTES], Box<dyn Error>> {
    let query = send_query(&mut slots[0], peer, deadline).await?;
    let limits = FrameLimits::new(48, 0).map_err(|_| PilotError::WireSetup)?;
    let mut open = true;
    for count in [1, 2] {
        let datagram = response(&slots[0], peer, &query, 1, true).await?;
        handle_wake(
            slots,
            receiver,
            &mut open,
            datagram.wake(0),
            &[datagram.bytes, [0_u8; RECEIVE_BYTES]],
            limits,
            deadline,
        );
        assert_eq!((slots[0].received, slots[0].rejected), (count, count));
        assert_eq!(slots[0].last_rejection, Some(Rejection::Origin));
        assert!(matches!(&slots[0].phase, Phase::Pending(_)));
    }
    Ok(query)
}

/// A real read selected while pending remains chargeable after owner retirement.
/// The later uncoded I/O result is synthetic and establishes no native error.
///
/// # Errors
/// Preserve actual fixture I/O and missing committed neighbor facts.
///
/// # Panics
/// Fail if retirement loses a selected read, overwrites terminal history, or
/// charges an unconsumed ordinary error as another datagram.
async fn check_retired_selected_read(
    prepared: [PreparedSlot; 2],
    peers: [StdSocket; 2],
) -> Result<(), Box<dyn Error>> {
    let [a, b] = prepared;
    let mut slots = [LiveSlot::from_prepared(a)?, LiveSlot::from_prepared(b)?];
    let [a, b] = peers;
    let peers = [UdpSocket::from_std(a)?, UdpSocket::from_std(b)?];
    let deadline = Instant::now()
        .checked_add(MAX_TIMEOUT)
        .ok_or(PilotError::DeadlineOverflow)?;
    let (sender, mut receiver) = watch::channel(Control {
        stop: false,
        epoch: 0,
    });
    let control = ControlHandle { sender };
    let saved_neighbor = commit_neighbor(&mut slots, &peers[1], &mut receiver, deadline).await?;
    let query = two_origin_rejections(&mut slots, &peers[0], &mut receiver, deadline).await?;
    let selected = response(&slots[0], &peers[0], &query, 15, false).await?;
    assert!(slots[0].can_receive());
    control.stop();
    refresh_owner(&mut slots, &mut receiver, deadline);
    assert_eq!(
        slots[0].terminal().map(TerminalOutcome::status),
        Some(Status::Stopped)
    );
    assert!(!slots[0].can_receive());
    let limits = FrameLimits::new(48, 0).map_err(|_| PilotError::WireSetup)?;
    let mut open = true;
    handle_wake(
        &mut slots,
        &mut receiver,
        &mut open,
        selected.wake(0),
        &[selected.bytes, [0_u8; RECEIVE_BYTES]],
        limits,
        deadline,
    );
    assert_eq!((slots[0].received, slots[0].rejected), (3, 3));
    assert_eq!(slots[0].last_rejection, Some(Rejection::Origin));
    assert_eq!(
        slots[0].terminal().map(TerminalOutcome::status),
        Some(Status::Stopped)
    );
    assert!(slots[0].sent);
    handle_wake(
        &mut slots,
        &mut receiver,
        &mut open,
        Wake::Read {
            slot: 0,
            result: classify_read_result(
                NativeDatagramPlatform::Other,
                Err(io::Error::other("synthetic post-retirement error")),
            ),
            read: SoftwareClock::capture(),
        },
        &[[0_u8; RECEIVE_BYTES]; 2],
        limits,
        deadline,
    );
    assert_eq!((slots[0].received, slots[0].rejected), (3, 3));
    assert_eq!(
        slots[0].terminal().map(TerminalOutcome::status),
        Some(Status::Stopped)
    );
    check_fact_getters(&snapshot_facts(&slots[1])?, &saved_neighbor);
    assert_eq!((slots[1].received, slots[1].rejected), (1, 0));
    Ok(())
}

/// The typed Windows oversize result is synthetic; all preceding query/origin
/// rejections and the following stratum-fifteen neighbor are real loopback I/O.
/// No native Windows consumption/release claim follows from this handler check.
///
/// # Errors
/// Preserve finite native I/O, fixed wire setup or missing committed facts.
///
/// # Panics
/// Fail if consumed oversize loses either counter, parses partial bytes, retires
/// the pending slot, or later overwrites its healthy historical result.
async fn check_typed_oversize_accounting(
    prepared: [PreparedSlot; 2],
    peers: [StdSocket; 2],
) -> Result<(), Box<dyn Error>> {
    let [a, b] = prepared;
    let mut slots = [LiveSlot::from_prepared(a)?, LiveSlot::from_prepared(b)?];
    let [a, b] = peers;
    let peers = [UdpSocket::from_std(a)?, UdpSocket::from_std(b)?];
    let deadline = Instant::now()
        .checked_add(MAX_TIMEOUT)
        .ok_or(PilotError::DeadlineOverflow)?;
    let (sender, mut receiver) = watch::channel(Control {
        stop: false,
        epoch: 0,
    });
    let _sender = sender;
    let saved_neighbor = commit_neighbor(&mut slots, &peers[1], &mut receiver, deadline).await?;
    let query = two_origin_rejections(&mut slots, &peers[0], &mut receiver, deadline).await?;
    let limits = FrameLimits::new(48, 0).map_err(|_| PilotError::WireSetup)?;
    let mut open = true;
    handle_wake(
        &mut slots,
        &mut receiver,
        &mut open,
        Wake::Read {
            slot: 0,
            result: classify_read_result(
                NativeDatagramPlatform::Windows,
                Err(io::Error::from_raw_os_error(10040)),
            ),
            read: SoftwareClock::capture(),
        },
        &[[0xff; RECEIVE_BYTES]; 2],
        limits,
        deadline,
    );
    assert_eq!((slots[0].received, slots[0].rejected), (3, 3));
    assert_eq!(slots[0].last_rejection, Some(Rejection::Length));
    assert!(slots[0].can_receive());
    let actual = response(&slots[0], &peers[0], &query, 15, false).await?;
    handle_wake(
        &mut slots,
        &mut receiver,
        &mut open,
        actual.wake(0),
        &[actual.bytes, [0_u8; RECEIVE_BYTES]],
        limits,
        deadline,
    );
    assert_eq!((slots[0].received, slots[0].rejected), (4, 3));
    assert_eq!(
        slots[0].terminal().and_then(TerminalOutcome::stratum),
        Some(15)
    );
    let committed = snapshot_facts(&slots[0])?;
    assert_eq!(committed.read_return.mono, actual.read.mono);
    assert_eq!(committed.read_return.utc, actual.read.utc);
    check_fact_getters(&snapshot_facts(&slots[1])?, &saved_neighbor);
    assert_eq!((slots[1].received, slots[1].rejected), (1, 0));
    Ok(())
}

/// Selected data and consumed oversize retain charges across one-way retirement.
///
/// # Errors
/// Preserve actual runtime/loopback fixture failures.
///
/// # Panics
/// Fail if the typed accounting or independently committed neighbor differs.
#[test]
fn selected_read_accounting_survives_retirement_and_typed_oversize() -> Result<(), Box<dyn Error>> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .max_blocking_threads(2)
        .build()?;
    let Fixture { prepared, peers } = fixture()?;
    runtime.block_on(check_retired_selected_read(prepared, peers))?;
    let Fixture { prepared, peers } = fixture()?;
    runtime.block_on(check_typed_oversize_accounting(prepared, peers))?;
    Ok(())
}

/// Use real request/read facts with explicitly synthetic exact processing instants.
/// A deliberately earlier processing value is not an observed native clock reversal.
///
/// # Errors
/// Preserve actual I/O, wire setup, checked instant and missing fact failures.
///
/// # Panics
/// Fail if just-below accepts, equality rejects, rejection arithmetic/history
/// changes, stratum15 is lost, or the earlier healthy neighbor is overwritten.
async fn check_commit_processing_boundaries(
    prepared: [PreparedSlot; 2],
    peers: [StdSocket; 2],
    equal: bool,
) -> Result<(), Box<dyn Error>> {
    let [a, b] = prepared;
    let mut slots = [LiveSlot::from_prepared(a)?, LiveSlot::from_prepared(b)?];
    let [a, b] = peers;
    let peers = [UdpSocket::from_std(a)?, UdpSocket::from_std(b)?];
    let deadline = Instant::now()
        .checked_add(MAX_TIMEOUT)
        .ok_or(PilotError::DeadlineOverflow)?;
    let (sender, mut receiver) = watch::channel(Control {
        stop: false,
        epoch: 0,
    });
    let _sender = sender;
    let saved_neighbor = commit_neighbor(&mut slots, &peers[1], &mut receiver, deadline).await?;
    let query = two_origin_rejections(&mut slots, &peers[0], &mut receiver, deadline).await?;
    let datagram = response(&slots[0], &peers[0], &query, 15, false).await?;
    let limits = FrameLimits::new(48, 0).map_err(|_| PilotError::WireSetup)?;
    let staged = stage_reply(
        &slots[0],
        &datagram.bytes[..datagram.length],
        datagram.source,
        datagram.read,
        limits,
    )
    .map_err(|_| io::Error::other("real stratum-fifteen boundary neighbor did not stage"))?;
    let earlier = datagram
        .read
        .mono
        .checked_sub(Duration::from_nanos(1))
        .ok_or_else(|| io::Error::other("actual read has no checked predecessor"))?;
    let selected = classify_read_result(
        NativeDatagramPlatform::current(),
        Ok((datagram.length, datagram.source)),
    );
    assert!(charge_selected_datagram(&mut slots[0], &selected));
    assert_eq!((slots[0].received, slots[0].rejected), (3, 2));
    commit_reply(
        &mut slots[0],
        &staged,
        if equal { datagram.read.mono } else { earlier },
    );
    assert_eq!(slots[0].last_rejection, Some(Rejection::Origin));
    if equal {
        assert_eq!(
            slots[0].terminal().and_then(TerminalOutcome::stratum),
            Some(15)
        );
        assert_eq!((slots[0].received, slots[0].rejected), (3, 2));
        let facts = snapshot_facts(&slots[0])?;
        assert_eq!(facts.processing, datagram.read.mono);
        assert_eq!(facts.read_return.mono, datagram.read.mono);
        assert_eq!(facts.read_return.utc, datagram.read.utc);
        assert_eq!(facts.send_before.mono, staged.sent.before.mono);
        assert_eq!(facts.send_after.mono, staged.sent.after.mono);
        assert_eq!(facts.peer_receive_bits, 0x0102_0304_0506_0708);
        assert_eq!(facts.peer_transmit_bits, 0x090a_0b0c_0d0e_0f10);
    } else {
        assert!(matches!(&slots[0].phase, Phase::Pending(_)));
        assert_eq!((slots[0].received, slots[0].rejected), (3, 3));
    }
    check_fact_getters(&snapshot_facts(&slots[1])?, &saved_neighbor);
    Ok(())
}

/// Exact processing boundaries use the real packet/send/read as their healthy facts.
/// Each fresh case charges its actual third selected read before direct commit.
///
/// # Errors
/// Preserve actual fixture/runtime and bounded I/O failures.
///
/// # Panics
/// Fail if exact equality, one-nanosecond predecessor or neighboring history differs.
#[test]
fn commit_processing_accepts_equal_read_and_rejects_its_predecessor() -> Result<(), Box<dyn Error>>
{
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .max_blocking_threads(2)
        .build()?;
    for equal in [false, true] {
        let Fixture { prepared, peers } = fixture()?;
        runtime.block_on(check_commit_processing_boundaries(prepared, peers, equal))?;
    }
    Ok(())
}

/// Produce a genuine stopped-slot report after both Rust socket owners are dropped.
/// No packet is sent; this reports Rust drop return, not native AFD completion.
/// The caller has already entered its owned I/O-enabled runtime.
///
/// # Errors
/// Preserve actual registration or terminal-report construction failures.
fn stopped_report(prepared: [PreparedSlot; 2]) -> Result<PilotReport, PilotError> {
    let started = Instant::now();
    let [a, b] = prepared;
    let mut slots = [LiveSlot::from_prepared(a)?, LiveSlot::from_prepared(b)?];
    retire_for_control(
        &mut slots,
        Control {
            stop: true,
            epoch: 0,
        },
    );
    let [a, b] = slots;
    let reports = [a.into_report(0)?, b.into_report(1)?];
    let sockets_drop_returned = Instant::now();
    Ok(PilotReport {
        slots: reports,
        done_elapsed_ns: sockets_drop_returned.duration_since(started).as_nanos(),
        sockets_drop_returned,
    })
}

/// Require independent redacted summaries for all three completed result positions.
/// The input native codes are synthetic representations, not induced OS failures.
///
/// # Errors
/// Reject a missing aggregate or typed native cause instead of assuming success.
///
/// # Panics
/// Fail if cleanup combines, swaps or drops either error or its typed native cause.
fn check_completed_cleanup_error(error: PilotError, mask: u8) -> Result<(), Box<dyn Error>> {
    let PilotError::ControlCleanup {
        primary,
        stop,
        invalidate,
    } = error
    else {
        return Err(
            io::Error::other("completed cleanup error lost its structured aggregate").into(),
        );
    };
    for (summary, present, expected_code, native_code) in [
        (primary, mask & 4 != 0, "bind_io", 5),
        (stop, mask & 1 != 0, "nonblocking_io", 6),
        (invalidate, mask & 2 != 0, "local_address_io", 7),
    ] {
        assert_eq!(summary.is_some(), present);
        if let Some(summary) = summary {
            assert_eq!(summary.code(), expected_code);
            let cause = summary
                .native_cause()
                .ok_or_else(|| io::Error::other("completed result lost its captured cause"))?;
            assert_eq!(cause.native_code(), Some(native_code));
            assert_eq!(
                cause.kind(),
                io::Error::from_raw_os_error(native_code).kind()
            );
        }
    }
    Ok(())
}

/// Every combination of completed cleanup results preserves distinct primary causes.
/// Errors here are synthetic result inputs; actual task reaping stays in `run_owned`.
/// Healthy primary payloads contain real owned sockets whose Rust drops returned.
///
/// # Errors
/// Preserve actual fixture/runtime/report construction failures.
///
/// # Panics
/// Fail if either cleanup alone is ignored, a primary failure becomes success,
/// or successful cleanup replaces the original report's endpoint/history payload.
#[test]
fn completed_cleanup_results_preserve_primary_and_each_independent_error()
-> Result<(), Box<dyn Error>> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .max_blocking_threads(2)
        .build()?;
    for mask in 0_u8..8 {
        let Fixture {
            prepared,
            peers: _peers,
        } = fixture()?;
        let locals = [prepared[0].local, prepared[1].local];
        runtime.block_on(async {
            let report = stopped_report(prepared)?;
            let primary = if mask & 4 != 0 {
                Err(PilotError::Bind(IoCause::capture(
                    &io::Error::from_raw_os_error(5),
                )))
            } else {
                Ok(report)
            };
            let stop = if mask & 1 != 0 {
                Err(PilotError::Nonblocking(IoCause::capture(
                    &io::Error::from_raw_os_error(6),
                )))
            } else {
                Ok(())
            };
            let invalidate = if mask & 2 != 0 {
                Err(PilotError::LocalAddress(IoCause::capture(
                    &io::Error::from_raw_os_error(7),
                )))
            } else {
                Ok(())
            };
            let result = finish_run(primary, stop, invalidate);
            if mask & 3 != 0 {
                check_completed_cleanup_error(
                    result
                        .err()
                        .ok_or_else(|| io::Error::other("cleanup failure returned success"))?,
                    mask,
                )?;
            } else if mask == 4 {
                assert!(
                    matches!(result, Err(PilotError::Bind(cause)) if cause.native_code() == Some(5))
                );
            } else {
                let report = result?;
                for (index, slot) in report.slots().iter().enumerate() {
                    assert_eq!(slot.local_endpoint(), locals[index]);
                    assert_eq!(slot.status(), Status::Stopped);
                    assert!(!slot.sent());
                    assert_eq!((slot.received(), slot.rejected()), (0, 0));
                }
            }
            Ok::<(), Box<dyn Error>>(())
        })?;
    }
    Ok(())
}

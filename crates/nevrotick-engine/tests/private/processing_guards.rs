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
            result: Ok((self.length, self.source)),
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
        while slot.can_send() {
            slot.socket.writable().await?;
            attempt_send(slot, deadline);
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
) -> Result<(), Box<dyn Error>> {
    let (neighbor, _) = receive_reply(&mut slots[1], &peers[1], send_deadline, 1, false).await?;
    let mut buffers = [[0_u8; RECEIVE_BYTES]; 2];
    buffers[1] = neighbor.bytes;
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
    )
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
            check_guard(guard, &mut slots, &peers, send_deadline).await?;
            let [a, b] = slots;
            let first_report = a.into_report(0)?;
            let neighbor_report = b.into_report(1)?;
            check_report_projection(&first_report, guard)?;
            assert_eq!(neighbor_report.status(), Status::Header);
            assert_eq!(neighbor_report.stratum(), Some(1));
            assert!(neighbor_report.facts().is_some());
            assert!(neighbor_report.control_code().is_none());
            assert!(neighbor_report.io_cause().is_none());
            Ok::<(), Box<dyn Error>>(())
        })?;
    }
    Ok(())
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
                    result: Err(io::Error::other("synthetic read failure")),
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

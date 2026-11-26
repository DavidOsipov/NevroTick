// SPDX-License-Identifier: GPL-2.0-only
//! Independent same-process `KoD` controls through bounded laboratory owners.
//!
//! REQ-8/BND-2; CHECK-6/8/9/17/21/27. Kernel controls use the production allocator
//! and transition implementation with an isolated laboratory cell; concrete
//! send/receive cases use genuine loopback sockets and owner controls.

use super::*;
use std::{
    error::Error,
    io,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV6},
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::{Receiver, RecvTimeoutError, SyncSender, TryRecvError},
    },
    time::{Duration, Instant},
};

use crate::{
    AcquisitionReport, Control, LiveSlot, NativeDatagramPlatform, PACKET_BYTES, Phase,
    RECEIVE_BYTES, RECEIVE_CAP, Rejection, RunClockContext, SlotReport, SoftwareClock, Status,
    TerminalOutcome, Wake,
};
use tokio::{net::UdpSocket, sync::watch};

type Check<T> = Result<T, Box<dyn Error>>;

/// One checked public round; these explicit fixture durations are not defaults.
/// # Errors
/// Preserve the actual fixed-round constructor refusal.
fn managed_round_config(
    peers: &[SocketAddr],
) -> Result<crate::AddressRoundConfig, crate::AddressRoundConfigError> {
    crate::AddressRoundConfig::new(peers, Duration::from_millis(400), Duration::from_secs(2))
}

/// Retain an actual query and send one independently encoded header/control.
/// # Errors
/// Preserve native datagram failure or reject malformed/incomplete fixture traffic.
/// # Panics
/// Reject malformed queries or incomplete actual reply writes.
fn managed_round_reply(
    peer: &std::net::UdpSocket,
    query: &[u8; RECEIVE_BYTES],
    length: usize,
    client: SocketAddr,
    code: Option<u32>,
) -> io::Result<()> {
    assert_eq!(length, 48, "managed round must emit one exact NTP header");
    assert_eq!(query[0], 0x23);
    assert_ne!(&query[40..48], &[0; 8]);
    let mut response = [0_u8; 48];
    response[0] = 0x24;
    response[1] = if code.is_some() { 0 } else { 2 };
    if let Some(code) = code {
        response[12..16].copy_from_slice(&code.to_be_bytes());
    }
    response[24..32].copy_from_slice(&query[40..48]);
    response[32..40].copy_from_slice(&1_u64.to_be_bytes());
    response[40..48].copy_from_slice(&2_u64.to_be_bytes());
    assert_eq!(peer.send_to(&response, client)?, 48);
    Ok(())
}

/// Drive the real public preparation/execution while one finite peer replies.
/// Every scoped peer is joined before an error or assertion escapes.
/// # Errors
/// Preserve runtime, preparation/execution, join and native peer failures.
/// # Panics
/// Reject a broken retirement prerequisite or facts with the wrong source owner.
fn managed_round_once(
    source: &SourceHandle,
    peers: &[SocketAddr],
    responder: std::net::UdpSocket,
    code: Option<u32>,
) -> Check<(crate::AddressRoundReport, std::net::UdpSocket)> {
    drop(runtime()?);
    responder.set_read_timeout(Some(Duration::from_secs(2)))?;
    let (prepared, control) = source.prepare_address_round(managed_round_config(peers)?)?;
    std::thread::scope(|scope| {
        let peer = scope.spawn(move || -> io::Result<std::net::UdpSocket> {
            let mut query = [0; RECEIVE_BYTES];
            let (length, client) = responder.recv_from(&mut query)?;
            managed_round_reply(&responder, &query, length, client, code)?;
            Ok(responder)
        });
        let result =
            crate::execute_address_round(prepared, &crate::ControlSchedule::new(None, None)?);
        let peer_result = peer.join().map_err(|_| "managed round peer panicked")?;
        drop(control);
        let responder = peer_result?;
        let report = result?;
        let expected = source.identity();
        for attempt in report.attempts() {
            if attempt.report().status() == Status::Header {
                let observed = attempt
                    .report()
                    .facts()
                    .ok_or("actual managed round header lost its facts")?;
                assert!(
                    observed
                        .source_identity()
                        .is_some_and(|identity| identity.same_source(&expected))
                );
            } else {
                assert!(attempt.report().facts().is_none());
            }
        }
        Ok((report, responder))
    })
}

/// Atomic batch construction retains existing records and deduplicates aliases.
/// # Errors
/// Preserve isolated allocation, binding, target and complete-state observations.
/// # Panics
/// Reject any partially published key, reset fact, or extra canonical endpoint slot.
#[test]
fn managed_round_batch_binding_is_atomic_and_reuses_canonical_capacity() -> Check<()> {
    let boundary_authority = isolated_authority(limits(1, 4)?)?;
    let boundary_source = boundary_authority.source(0)?;
    let peers = [
        endpoint(43011),
        endpoint(43012),
        endpoint(43013),
        endpoint(43014),
    ];
    let existing = boundary_source.bind(peers[0])?;
    let existing_before = facts(&existing)?;
    let invalid_selections: [&[SocketAddr]; 2] = [&[], &peers];
    for selection in invalid_selections {
        assert!(
            matches!(
                boundary_source.bind_round(selection),
                Err(SourcePolicyError::InvalidSelection)
            ),
            "batch contract rejects empty or more than three peers"
        );
        for peer in &peers[1..] {
            assert!(matches!(
                boundary_authority.endpoint_target(*peer),
                Err(SourcePolicyError::UnknownEndpoint)
            ));
        }
        assert_eq!(facts(&existing)?, existing_before);
    }
    let single = boundary_source.bind_round(&peers[..1])?;
    assert!(single.mode(peers[0]).is_ok());
    let triple = boundary_source.bind_round(&peers[..3])?;
    for peer in &peers[..3] {
        assert!(triple.mode(*peer).is_ok());
    }
    // The fourth endpoint fits the authority; only a four-peer batch is invalid.
    assert!(boundary_source.bind(peers[3]).is_ok());
    assert_eq!(facts(&existing)?, existing_before);

    let authority = isolated_authority(limits(2, 2)?)?;
    let source = authority.source(0)?;
    let first = source.bind(endpoint(43001))?;
    let before = facts(&first)?;
    assert!(matches!(
        source.bind_round(&[endpoint(43001), endpoint(43002), endpoint(43003)]),
        Err(SourcePolicyError::Capacity)
    ));
    assert!(matches!(
        authority.endpoint_target(endpoint(43002)),
        Err(SourcePolicyError::UnknownEndpoint)
    ));
    assert!(matches!(
        authority.endpoint_target(endpoint(43003)),
        Err(SourcePolicyError::UnknownEndpoint)
    ));
    assert_eq!(facts(&first)?, before);
    let aliases = [
        endpoint(43001),
        SocketAddr::new(Ipv4Addr::LOCALHOST.to_ipv6_mapped().into(), 43001),
        endpoint(43002),
    ];
    let batch = source.bind_round(&aliases)?;
    assert!(batch.mode(endpoint(43001)).is_ok());
    assert!(batch.mode(endpoint(43002)).is_ok());
    assert!(matches!(
        batch.mode(endpoint(43003)),
        Err(SourcePolicyError::Foreign)
    ));
    assert!(
        source
            .bind_round(&[endpoint(43002), endpoint(43001)])
            .is_ok()
    );
    assert_eq!(facts(&first)?, before);
    let invalid = SocketAddr::from((Ipv4Addr::LOCALHOST, 0));
    assert!(matches!(
        source.bind_round(&[invalid]),
        Err(SourcePolicyError::InvalidEndpoint)
    ));
    assert_eq!(facts(&first)?, before);
    Ok(())
}

/// Genuine A/E controls preserve A/F blocking, B/E skip and independent B/F use.
/// # Errors
/// Preserve public managed preparation/execution and all joined native peer errors.
/// # Panics
/// Reject source evasion, transitive state, fabricated attempts or missing healthy traffic.
#[test]
fn public_managed_round_restrictions_skip_only_the_endpoint_and_preserve_absent_tail() -> Check<()>
{
    for code in [DENY, RSTR] {
        let authority = isolated_authority(SourcePolicyLimits::new(
            2,
            2,
            Duration::from_millis(1),
            Duration::from_millis(4),
        )?)?;
        let a = authority.source(0)?;
        let b = authority.source(1)?;
        let e_socket = std::net::UdpSocket::bind(endpoint(0))?;
        let f_socket = std::net::UdpSocket::bind(endpoint(0))?;
        let e = e_socket.local_addr()?;
        let f = f_socket.local_addr()?;
        let (control_report, e_socket) = managed_round_once(&a, &[e], e_socket, Some(code))?;
        assert_eq!(control_report.end(), crate::AddressRoundEnd::Control);
        assert_eq!(control_report.attempts().len(), 1);
        assert_eq!(
            control_report.attempts()[0].report().control_code(),
            Some(code)
        );
        let ae = a.bind(e)?;
        let be = b.bind(e)?;
        let restricted = facts(&ae)?;
        let (prepared, control) = a.prepare_address_round(managed_round_config(&[f])?)?;
        let stopped =
            crate::execute_address_round(prepared, &crate::ControlSchedule::new(None, None)?)?;
        drop(control);
        assert_eq!(stopped.end(), crate::AddressRoundEnd::SourceRestricted);
        assert!(stopped.attempts().is_empty());
        assert_eq!(stopped.policy_refusals().len(), 1);
        assert_eq!(
            stopped.policy_refusals()[0].scope(),
            crate::AddressPolicyRefusalScope::Source
        );
        f_socket.set_nonblocking(true)?;
        let mut bytes = [0; RECEIVE_BYTES];
        assert!(
            matches!(f_socket.recv_from(&mut bytes), Err(error) if error.kind() == io::ErrorKind::WouldBlock)
        );
        f_socket.set_nonblocking(false)?;
        let (healthy, f_socket) = managed_round_once(&b, &[e, f], f_socket, None)?;
        assert_eq!(healthy.end(), crate::AddressRoundEnd::Reply);
        assert_eq!(healthy.policy_refusals().len(), 1);
        assert_eq!(healthy.policy_refusals()[0].index(), 0);
        assert_eq!(
            healthy.policy_refusals()[0].scope(),
            crate::AddressPolicyRefusalScope::Endpoint
        );
        assert_eq!(healthy.attempts().len(), 1);
        assert_eq!(healthy.attempts()[0].peer(), f);
        assert_eq!(healthy.attempts()[0].report().status(), Status::Header);
        assert!(healthy.attempts()[0].report().sent());
        assert_eq!(facts(&ae)?, restricted);
        assert_eq!(facts(&be)?[1], restricted[1]);
        assert!(!facts(&be)?[0].2, "B must never inherit A's restriction");
        e_socket.set_nonblocking(true)?;
        assert!(
            matches!(e_socket.recv_from(&mut bytes), Err(error) if error.kind() == io::ErrorKind::WouldBlock)
        );
        // A stopped before index1 must not manufacture either tail representation.
        let (prepared, handle) = b.prepare_address_round(managed_round_config(&[f, e])?)?;
        handle.stop();
        let stopped =
            crate::execute_address_round(prepared, &crate::ControlSchedule::new(None, None)?)?;
        assert_eq!(stopped.end(), crate::AddressRoundEnd::Stopped);
        assert!(stopped.attempts().is_empty() && stopped.policy_refusals().is_empty());
        drop((f_socket, handle));
    }
    Ok(())
}

/// Project only the bounded historical facts needed to diagnose a round assertion.
fn managed_round_attempt_diagnostics(report: &crate::AddressRoundReport) -> impl fmt::Debug {
    report
        .attempts()
        .iter()
        .map(|attempt| {
            let slot: &SlotReport = attempt.report();
            (
                slot.status(),
                slot.sent(),
                slot.received(),
                slot.rejected(),
                slot.last_rejection(),
                slot.source_policy_refusal(),
            )
        })
        .collect::<Vec<_>>()
}

/// A real A/E control interrupts live B/E without transitive B/F refusal.
/// # Errors
/// Preserve bounded query handshakes, public owners and every joined fixture result.
/// # Panics
/// Reject live-scope invention, current-invocation fallback or inherited B restriction.
#[test]
fn public_managed_round_live_endpoint_refusal_preserves_history_and_later_source_use() -> Check<()>
{
    drop(runtime()?);
    let authority = isolated_authority(SourcePolicyLimits::new(
        2,
        2,
        Duration::from_millis(1),
        Duration::from_millis(4),
    )?)?;
    let a = authority.source(0)?;
    let b = authority.source(1)?;
    let e_socket = std::net::UdpSocket::bind(endpoint(0))?;
    e_socket.set_read_timeout(Some(Duration::from_secs(2)))?;
    let f_socket = std::net::UdpSocket::bind(endpoint(0))?;
    let e = e_socket.local_addr()?;
    let f = f_socket.local_addr()?;
    let (a_plan, a_control) = a.prepare_address_round(managed_round_config(&[e])?)?;
    let (b_plan, b_control) = b.prepare_address_round(managed_round_config(&[e, f])?)?;
    let (signal, first_seen) = std::sync::mpsc::sync_channel(1);
    let measured_from: Instant = Instant::now();
    let joined = std::thread::scope(|scope| -> Check<_> {
        let server = scope.spawn(move || -> io::Result<_> {
            let mut first = [0; RECEIVE_BYTES];
            let (length, client) = e_socket.recv_from(&mut first)?;
            let a_query: Duration = measured_from.elapsed();
            signal
                .send(())
                .map_err(|_| io::Error::other("query observer closed"))?;
            let mut second = [0; RECEIVE_BYTES];
            let (second_length, second_client) = e_socket.recv_from(&mut second)?;
            let b_query: Duration = measured_from.elapsed();
            assert_eq!(second_length, 48);
            assert_ne!(client, second_client, "A and B must own separate sockets");
            managed_round_reply(&e_socket, &first, length, client, Some(DENY))?;
            let deny_sent: Duration = measured_from.elapsed();
            Ok((e_socket, [a_query, b_query, deny_sent]))
        });
        let a_job = scope.spawn(move || -> Result<_, crate::PilotError> {
            let schedule: crate::ControlSchedule = crate::ControlSchedule::new(None, None)?;
            let a_start: Duration = measured_from.elapsed();
            Ok((crate::execute_address_round(a_plan, &schedule)?, a_start))
        });
        first_seen.recv_timeout(Duration::from_secs(2))?;
        let b_job = scope.spawn(move || {
            crate::execute_address_round(b_plan, &crate::ControlSchedule::new(None, None)?)
        });
        let a_result = a_job.join().map_err(|_| "A round worker panicked")?;
        let b_result = b_job.join().map_err(|_| "B round worker panicked")?;
        let peer_result = server
            .join()
            .map_err(|_| "shared endpoint worker panicked")?;
        Ok((a_result?, b_result?, peer_result?))
    })?;
    let ((a_report, a_start), b_report, (e_socket, [a_query, b_query, deny_sent])) = joined;
    drop((a_control, b_control));
    assert_eq!(
        (a_report.end(), b_report.end()),
        (
            crate::AddressRoundEnd::Control,
            crate::AddressRoundEnd::Policy
        ),
        "A started at {a_start:?}; queries A {a_query:?}, B {b_query:?}; DENY sent at {deny_sent:?}; \
         attempts (status, sent, received, rejected, last rejection, policy refusal): A {:?}, B {:?}",
        managed_round_attempt_diagnostics(&a_report),
        managed_round_attempt_diagnostics(&b_report),
    );
    assert!(b_report.policy_refusals().is_empty());
    assert_eq!(b_report.attempts().len(), 1);
    assert_eq!(b_report.attempts()[0].peer(), e);
    assert_eq!(
        b_report.attempts()[0].report().source_policy_refusal(),
        Some(SourcePolicyError::Disabled)
    );
    assert!(!facts(&b.bind(e)?)?[0].2);
    f_socket.set_nonblocking(true)?;
    let mut bytes = [0; RECEIVE_BYTES];
    assert!(
        matches!(f_socket.recv_from(&mut bytes), Err(error) if error.kind() == io::ErrorKind::WouldBlock)
    );
    f_socket.set_nonblocking(false)?;
    let (later, f_socket) = managed_round_once(&b, &[f], f_socket, None)?;
    assert_eq!(later.end(), crate::AddressRoundEnd::Reply);
    assert_eq!(later.attempts().len(), 1);
    assert_eq!(later.attempts()[0].report().status(), Status::Header);
    drop((f_socket, e_socket));
    Ok(())
}

/// Schedule applied at the real final receive guard for one selected datagram.
#[derive(Clone, Copy, Debug)]
enum FinalPolicySchedule {
    /// No other owner holds the policy cell at admission.
    Free,
    /// Release the acquired policy cell immediately after the first attempt.
    Release,
    /// Latch Stop while the actual policy owner keeps the cell.
    Stop,
    /// Invalidate the invocation while the actual policy owner keeps the cell.
    Invalidate,
    /// Hold the policy cell through retirement at the original cutoff.
    Cutoff,
}

impl FinalPolicySchedule {
    /// Whether the independent holder keeps custody until actual execution returns.
    fn hold_until_return(self) -> bool {
        matches!(self, Self::Stop | Self::Invalidate | Self::Cutoff)
    }

    /// The required terminal outcome of the original one-datagram history.
    fn expected(self) -> Status {
        match self {
            Self::Free | Self::Release => Status::Control,
            Self::Stop => Status::Stopped,
            Self::Invalidate => Status::Stale,
            Self::Cutoff => Status::Timeout,
        }
    }
}

/// Wall-wait bound for each fixture handshake and the original live request.
const FINAL_POLICY_WAIT: Duration = Duration::from_secs(2);

/// Bind observations to the real driver's original staged reply checkpoints.
/// # Panics
/// Fail the fixture if a pulse is missing or the requested invalidation fails.
fn observe_final_policy_schedule(
    plan: &mut crate::PreparedSeries,
    schedule: FinalPolicySchedule,
    mark: SyncSender<bool>,
    guard_ready: Receiver<()>,
) {
    let guard_ready = Mutex::new(guard_ready);
    crate::series::set_test_final_guard_observer(plan, move |event, handle| {
        use crate::series::TestFinalGuardEvent;
        match event {
            TestFinalGuardEvent::ReplyBefore => {
                assert!(mark.try_send(false).is_ok());
                assert!(
                    guard_ready
                        .lock()
                        .is_ok_and(|state| { state.recv_timeout(FINAL_POLICY_WAIT).is_ok() })
                );
            }
            TestFinalGuardEvent::ReplyAfter => {
                match schedule {
                    FinalPolicySchedule::Stop => handle.stop(),
                    FinalPolicySchedule::Invalidate => assert!(handle.invalidate().is_ok()),
                    FinalPolicySchedule::Free
                    | FinalPolicySchedule::Release
                    | FinalPolicySchedule::Cutoff => {}
                }
                assert!(mark.try_send(true).is_ok());
            }
            TestFinalGuardEvent::WritableBefore | TestFinalGuardEvent::WritableAfter => {}
        }
    });
}

/// Check the original transferred accounting and absence of a successor send.
/// # Errors
/// Preserve absent slot, failed policy observation or native peer refusal.
/// # Panics
/// Fail on changed terminal outcome, duplicated receive or changed restriction.
fn check_final_policy_reply(
    schedule: FinalPolicySchedule,
    report: &crate::SeriesReport,
    peer: &std::net::UdpSocket,
    binding: &SourceBinding,
) -> Check<()> {
    let slot: &SlotReport = report
        .attempts()
        .first()
        .and_then(crate::SeriesAttempt::report)
        .ok_or("first selected datagram lost its slot report")?;
    let expected: Status = schedule.expected();
    let applied: bool = expected == Status::Control;
    assert_eq!(
        (
            slot.status(),
            slot.sent(),
            slot.received(),
            slot.rejected(),
            slot.control_code()
        ),
        (
            expected,
            true,
            1,
            u16::from(!applied),
            applied.then_some(DENY)
        ),
        "final policy schedule {schedule:?}; last rejection: {:?}",
        slot.last_rejection(),
    );
    assert!(facts(binding)?.iter().all(|record| record.2 == applied));
    peer.set_nonblocking(true)?;
    let mut bytes: [u8; RECEIVE_BYTES] = [0; RECEIVE_BYTES];
    assert!(matches!(
        peer.recv_from(&mut bytes),
        Err(error) if error.kind() == io::ErrorKind::WouldBlock
    ));
    Ok(())
}

/// Check the independent peer and holder observations against the original limits.
/// # Panics
/// Fail on a missed release, control wake, or premature/renewed owner cutoff.
fn check_policy_fixture_timing(
    mode: FinalPolicySchedule,
    cutoff: Option<Instant>,
    returned_at: Instant,
    released_at: Instant,
    timely_before: Instant,
) {
    match mode {
        FinalPolicySchedule::Free | FinalPolicySchedule::Release => assert!(
            released_at < timely_before,
            "fixture missed its conservative cutoff"
        ),
        FinalPolicySchedule::Stop | FinalPolicySchedule::Invalidate => assert!(
            returned_at < timely_before,
            "control did not retire the retained reply before the original timer"
        ),
        FinalPolicySchedule::Cutoff => assert!(
            cutoff.is_some_and(|value| value <= returned_at) && returned_at < timely_before,
            "retained reply did not retire at the earlier original owner cutoff"
        ),
    }
}

/// Drive one real correlated `DENY` under each original final-guard schedule.
/// # Errors
/// Preserve preparation, socket, synchronization and joined worker errors.
/// # Panics
/// Require ordered holder observations, fresh authority and exact slot accounting.
#[test]
fn managed_series_retains_one_deny_across_final_policy_release() -> Check<()> {
    drop(runtime()?);
    for mode in [
        FinalPolicySchedule::Free,
        FinalPolicySchedule::Release,
        FinalPolicySchedule::Stop,
        FinalPolicySchedule::Invalidate,
        FinalPolicySchedule::Cutoff,
    ] {
        let peer: std::net::UdpSocket = std::net::UdpSocket::bind(endpoint(0))?;
        peer.set_read_timeout(Some(FINAL_POLICY_WAIT))?;
        peer.set_write_timeout(Some(FINAL_POLICY_WAIT))?;
        let destination: SocketAddr = peer.local_addr()?;
        let authority: SourcePolicyAuthority = isolated_authority(limits(1, 1)?)?;
        let binding: SourceBinding = authority.source(0)?.bind(destination)?;
        let config: SeriesConfig =
            SeriesConfig::new(destination, FINAL_POLICY_WAIT, Duration::from_secs(4))?;
        let (mut plan, control) = crate::series::prepare_series_for_source(
            &config,
            SourceMode::Managed(binding.clone()),
        )?;
        let (mark, stages) = std::sync::mpsc::sync_channel(2);
        let (ready, guard_ready) = std::sync::mpsc::sync_channel(1);
        let (returned, execution_return) = std::sync::mpsc::sync_channel(1);
        observe_final_policy_schedule(&mut plan, mode, mark, guard_ready);
        let schedule: crate::ControlSchedule = crate::ControlSchedule::new(None, None)?;
        let (report, peer) = std::thread::scope(|scope| -> Check<_> {
            let server = scope.spawn(move || -> io::Result<_> {
                let mut bytes: [u8; RECEIVE_BYTES] = [0; RECEIVE_BYTES];
                let (length, client) = peer.recv_from(&mut bytes)?;
                managed_round_reply(&peer, &bytes, length, client, Some(DENY))?;
                Ok(peer)
            });
            let cell: Arc<PolicyCell> = Arc::clone(&binding.source.cell);
            let gate = scope.spawn(move || -> io::Result<Instant> {
                assert!(
                    !stages
                        .recv_timeout(FINAL_POLICY_WAIT)
                        .map_err(io::Error::other)?
                );
                let guard = if matches!(mode, FinalPolicySchedule::Free) {
                    None
                } else {
                    Some(cell.lock().map_err(io::Error::other)?)
                };
                ready.try_send(()).map_err(io::Error::other)?;
                assert!(
                    stages
                        .recv_timeout(FINAL_POLICY_WAIT)
                        .map_err(io::Error::other)?
                );
                if mode.hold_until_return() {
                    execution_return
                        .recv_timeout(Duration::from_secs(5))
                        .map_err(io::Error::other)?;
                }
                drop(guard);
                Ok(Instant::now())
            });
            let execution_start: Instant = Instant::now();
            let timely_before: Instant = execution_start
                .checked_add(FINAL_POLICY_WAIT)
                .ok_or(crate::PilotError::DeadlineOverflow)?;
            // Put the owner cutoff well before the configured request timer.
            let cutoff: Option<Instant> = if matches!(mode, FinalPolicySchedule::Cutoff) {
                Some(
                    execution_start
                        .checked_add(Duration::from_secs(1))
                        .ok_or(crate::PilotError::DeadlineOverflow)?,
                )
            } else {
                None
            };
            let report = crate::series::execute_series_with_cutoff(plan, &schedule, cutoff);
            let returned_at: Instant = Instant::now();
            let notified = if mode.hold_until_return() {
                returned.try_send(())
            } else {
                Ok(())
            };
            let gate_result = gate.join();
            let server_result = server.join();
            notified?;
            let released_at: Instant =
                gate_result.map_err(|_| "final policy lock worker panicked")??;
            check_policy_fixture_timing(mode, cutoff, returned_at, released_at, timely_before);
            Ok((
                report?,
                server_result.map_err(|_| "single DENY peer panicked")??,
            ))
        })?;
        drop(control);
        check_final_policy_reply(mode, &report, &peer, &binding)?;
    }
    Ok(())
}

/// Independently declared public refusal labels; no formatter supplies this oracle.
const POLICY_DIAGNOSTICS: [(SourcePolicyError, &str, &str); 17] = [
    (
        SourcePolicyError::InvalidLimits,
        "invalid source policy limits",
        "InvalidLimits",
    ),
    (
        SourcePolicyError::AlreadyInstalled,
        "source policy already installed",
        "AlreadyInstalled",
    ),
    (
        SourcePolicyError::Allocation,
        "source policy allocation refused",
        "Allocation",
    ),
    (
        SourcePolicyError::UnknownSource,
        "unknown source policy source",
        "UnknownSource",
    ),
    (
        SourcePolicyError::UnknownEndpoint,
        "unknown source policy endpoint",
        "UnknownEndpoint",
    ),
    (
        SourcePolicyError::InvalidEndpoint,
        "invalid source policy endpoint",
        "InvalidEndpoint",
    ),
    (
        SourcePolicyError::Capacity,
        "source policy capacity exhausted",
        "Capacity",
    ),
    (
        SourcePolicyError::Contended,
        "source policy contended",
        "Contended",
    ),
    (
        SourcePolicyError::Poisoned,
        "source policy poisoned",
        "Poisoned",
    ),
    (
        SourcePolicyError::Foreign,
        "foreign source policy owner",
        "Foreign",
    ),
    (
        SourcePolicyError::Stale,
        "stale source policy identity",
        "Stale",
    ),
    (
        SourcePolicyError::Duplicate,
        "duplicate source policy target",
        "Duplicate",
    ),
    (
        SourcePolicyError::InvalidSelection,
        "invalid source policy selection",
        "InvalidSelection",
    ),
    (
        SourcePolicyError::Disabled,
        "source policy refused",
        "Disabled",
    ),
    (
        SourcePolicyError::Unavailable,
        "source policy unavailable",
        "Unavailable",
    ),
    (SourcePolicyError::NotDue, "source policy not due", "NotDue"),
    (
        SourcePolicyError::UnhandledControl,
        "unhandled source control",
        "UnhandledControl",
    ),
];

/// Keep public diagnostics fixed while real checked owners retain private policy facts.
/// # Errors
/// Preserve checked limits/configuration, isolated allocation and actual source/target failures.
/// # Panics
/// Fail if a public label, source chain or redacted owner/target diagnostic differs.
#[test]
fn public_policy_diagnostics_are_literal_bounded_and_redacted() -> Check<()> {
    for (error, expected_display, expected_debug) in POLICY_DIAGNOSTICS {
        let display = error.to_string();
        let debug = format!("{error:?}");
        assert_eq!(display, expected_display);
        assert_eq!(debug, expected_debug);
        assert!(display.len() <= 64 && debug.len() <= 32);
        assert!(error.source().is_none());
    }
    let checked_limits =
        SourcePolicyLimits::new(2, 2, Duration::from_secs(1), Duration::from_secs(4))?;
    assert_eq!(format!("{checked_limits:?}"), "SourcePolicyLimits { .. }");
    let authority = isolated_authority(checked_limits)?;
    let source = authority.source(0)?;
    // The public session path binds this endpoint before its target is issued;
    // no native socket, process installation or successful network result is invented.
    let destination = endpoint(41001);
    let _session = source.session(SeriesConfig::new(
        destination,
        Duration::from_millis(100),
        Duration::from_secs(1),
    )?)?;
    let source_target = authority.source_target(&source)?;
    let endpoint_target = authority.endpoint_target(destination)?;
    assert_eq!(format!("{authority:?}"), "SourcePolicyAuthority { .. }");
    assert_eq!(format!("{source:?}"), "SourceHandle { .. }");
    assert_eq!(format!("{source_target:?}"), "SourcePolicyTarget { .. }");
    assert_eq!(format!("{endpoint_target:?}"), "SourcePolicyTarget { .. }");
    Ok(())
}

/// Explicit test bounds select no shipped polling interval or production source.
/// # Errors
/// Return the original checked bounds refusal.
fn limits(sources: usize, endpoints: usize) -> Result<SourcePolicyLimits, SourcePolicyError> {
    SourcePolicyLimits::new(
        sources,
        endpoints,
        Duration::from_secs(1),
        Duration::from_secs(4),
    )
}

fn endpoint(port: u16) -> SocketAddr {
    SocketAddr::from((Ipv4Addr::LOCALHOST, port))
}

const RATE: u32 = u32::from_be_bytes(*b"RATE");
const DENY: u32 = u32::from_be_bytes(*b"DENY");
const RSTR: u32 = u32::from_be_bytes(*b"RSTR");
const UNKNOWN: u32 = u32::from_be_bytes(*b"TEST");

type RecordFacts = (Duration, Option<Instant>, bool, u64, u64, bool);

/// Observe every retained field; a new snapshot field requires an explicit oracle.
/// # Errors
/// Preserve actual policy lock, binding and record-observation refusal.
fn facts(binding: &SourceBinding) -> Result<[RecordFacts; 2], SourcePolicyError> {
    let records = record_snapshots(binding)?;
    Ok(records.map(|record| {
        let RecordSnapshot {
            interval,
            floor,
            refused,
            revision,
            epoch,
            unavailable,
        } = record;
        (interval, floor, refused, revision, epoch, unavailable)
    }))
}

/// Accept only positive bounded capacities and ordered representable intervals.
/// # Panics
/// Fail if valid bounds are rejected or invalid bounds are admitted.
#[test]
fn laboratory_bounds_require_positive_capacity_and_ordered_intervals() {
    assert!(limits(1, 1).is_ok());
    assert!(limits(32, 64).is_ok());
    for (sources, endpoints) in [(0, 1), (1, 0), (33, 1), (1, 65)] {
        assert!(limits(sources, endpoints).is_err());
    }
    assert!(SourcePolicyLimits::new(1, 1, Duration::ZERO, Duration::from_secs(1)).is_err());
    assert!(SourcePolicyLimits::new(1, 1, Duration::from_secs(2), Duration::from_secs(1)).is_err());
    assert!(SourcePolicyLimits::new(1, 1, Duration::from_secs(1), Duration::from_secs(1)).is_ok());
    assert!(SourcePolicyLimits::new(1, 1, Duration::from_secs(1), Duration::MAX).is_err());
}

/// Keep the installed root and RATE state across facade/session destruction.
/// # Errors
/// Preserve constructor, root, binding, request-transition and configuration failures.
/// # Panics
/// Fail if another installation succeeds or retained record facts change.
#[test]
fn process_root_cannot_be_reinstalled_after_authority_and_session_drop() -> Check<()> {
    // Every other test uses isolated_authority. Only this case touches the real
    // process OnceLock, so no fixture reset can manufacture a fresh authority.
    let authority = SourcePolicyAuthority::install(limits(2, 2)?)?;
    let source = authority.source(0)?;
    let binding = source.bind(endpoint(41001))?;
    let now = Instant::now();
    let mut request = fixture_send(&binding, &Arc::new(()), now)?;
    fixture_control(&binding, &mut request, RATE, now)?;
    let retained = facts(&binding)?;
    let config = SeriesConfig::new(
        endpoint(41001),
        Duration::from_millis(100),
        Duration::from_secs(1),
    )?;
    let session = source.session(config)?;
    assert!(SourcePolicyAuthority::install(limits(2, 2)?).is_err());
    drop(session);
    drop(authority);
    assert!(source.bind(endpoint(41001)).is_ok());
    assert_eq!(facts(&source.bind(endpoint(41001))?)?, retained);
    assert!(SourcePolicyAuthority::install(limits(2, 2)?).is_err());
    Ok(())
}

/// Retain fixed catalogue positions and endpoint slots after handle destruction.
/// # Errors
/// Preserve valid limits, root construction and initial binding failures.
/// # Panics
/// Fail if invalid positions/capacity are admitted or a retained binding disappears.
#[test]
fn fixed_catalogue_and_endpoint_capacity_never_evict_existing_bindings() -> Check<()> {
    let authority = isolated_authority(limits(2, 2)?)?;
    let a = authority.source(0)?;
    let b = authority.source(1)?;
    assert!(authority.source(2).is_err());
    assert!(a.bind(endpoint(0)).is_err());
    let ae = a.bind(endpoint(41001))?;
    let be = b.bind(endpoint(41001))?;
    let af = a.bind(endpoint(41002))?;
    assert!(b.bind(endpoint(41003)).is_err());
    drop((ae, be, af));
    assert!(b.bind(endpoint(41003)).is_err());
    assert!(a.bind(endpoint(41001)).is_ok());
    assert!(b.bind(endpoint(41002)).is_ok());
    Ok(())
}

/// Share one endpoint slot between native and mapped IPv4 aliases.
/// # Errors
/// Preserve valid root, source and initial endpoint binding failures.
/// # Panics
/// Fail if an alias requires another slot or a different port reuses that slot.
#[test]
fn canonical_mapped_ipv4_alias_shares_one_exact_remote_endpoint() -> Check<()> {
    let authority = isolated_authority(limits(2, 1)?)?;
    let a = authority.source(0)?;
    let b = authority.source(1)?;
    let mapped = SocketAddr::V6(SocketAddrV6::new(
        Ipv4Addr::LOCALHOST.to_ipv6_mapped(),
        41001,
        0,
        0,
    ));
    let _native = a.bind(endpoint(41001))?;
    assert!(b.bind(mapped).is_ok());
    assert!(b.bind(endpoint(41002)).is_err());
    assert!(a.bind(endpoint(41001)).is_ok());
    Ok(())
}

/// Distinguish required IPv6 zones and remote UDP ports.
/// # Errors
/// Preserve valid root/source construction and initial scoped binding failures.
/// # Panics
/// Fail if distinct keys alias, an existing key disappears or a required zone is absent.
#[test]
fn required_ipv6_zone_and_remote_port_remain_distinct() -> Check<()> {
    let authority = isolated_authority(limits(1, 3)?)?;
    let source = authority.source(0)?;
    let address = Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1);
    let scoped = |port, zone| SocketAddr::V6(SocketAddrV6::new(address, port, 0, zone));
    let _first = source.bind(scoped(41001, 1))?;
    let _other_zone = source.bind(scoped(41001, 2))?;
    let _other_port = source.bind(scoped(41002, 1))?;
    assert!(source.bind(scoped(41001, 1)).is_ok());
    assert!(source.bind(scoped(41002, 2)).is_err());
    assert!(source.bind(scoped(41001, 0)).is_err());
    Ok(())
}

/// Restrict exactly the directly affected source and endpoint.
/// # Errors
/// Preserve root/binding, request, control and record-observation failures.
/// # Panics
/// Fail if direct records stay enabled or the healthy independent pair changes.
#[test]
fn direct_refusal_changes_a_and_e_without_transitive_alias_propagation() -> Check<()> {
    for code in [DENY, RSTR] {
        let authority = isolated_authority(limits(2, 2)?)?;
        let a = authority.source(0)?;
        let b = authority.source(1)?;
        let ae = a.bind(endpoint(41001))?;
        let be = b.bind(endpoint(41001))?;
        let af = a.bind(endpoint(41002))?;
        let bf = b.bind(endpoint(41002))?;
        let healthy_before = facts(&bf)?;
        let now = Instant::now();
        let mut request = fixture_send(&ae, &Arc::new(()), now)?;
        fixture_control(&ae, &mut request, code, now)?;
        assert!(facts(&ae)?.iter().all(|record| record.2));
        assert_eq!(facts(&bf)?, healthy_before);
        assert!(fixture_send(&be, &Arc::new(()), now).is_err());
        assert!(fixture_send(&af, &Arc::new(()), now).is_err());
        assert_eq!(facts(&bf)?, healthy_before);
        assert!(fixture_send(&bf, &Arc::new(()), now).is_ok());
    }
    Ok(())
}

/// Double each direct record without propagating inherited effective intervals.
/// # Errors
/// Preserve construction, request/control, record and checked-time failures.
/// # Panics
/// Fail if independent intervals/floors differ from their explicit expectations.
#[test]
fn rate_doubles_each_own_record_and_does_not_copy_an_effective_interval() -> Check<()> {
    let authority = isolated_authority(limits(2, 2)?)?;
    let a = authority.source(0)?;
    let b = authority.source(1)?;
    let ae = a.bind(endpoint(41001))?;
    let be = b.bind(endpoint(41001))?;
    let af = a.bind(endpoint(41002))?;
    let bf = b.bind(endpoint(41002))?;
    let now = Instant::now();
    let mut first = fixture_send(&af, &Arc::new(()), now)?;
    fixture_control(&af, &mut first, RATE, now)?;
    let second_at = now
        .checked_add(Duration::from_secs(2))
        .ok_or("fixture time overflow")?;
    let mut second = fixture_send(&ae, &Arc::new(()), second_at)?;
    fixture_control(&ae, &mut second, RATE, second_at)?;
    let [a_state, e_state] = facts(&ae)?;
    assert_eq!(
        (a_state.0, e_state.0),
        (Duration::from_secs(4), Duration::from_secs(2))
    );
    assert_eq!(a_state.1, second_at.checked_add(Duration::from_secs(4)));
    assert_eq!(e_state.1, second_at.checked_add(Duration::from_secs(2)));
    let third_at = second_at
        .checked_add(Duration::from_secs(2))
        .ok_or("fixture time overflow")?;
    let mut third = fixture_send(&be, &Arc::new(()), third_at)?;
    fixture_control(&be, &mut third, RATE, third_at)?;
    let [b_state, shared_e] = facts(&be)?;
    assert_eq!(
        (b_state.0, shared_e.0),
        (Duration::from_secs(2), Duration::from_secs(4))
    );
    assert_eq!(facts(&af)?[1].0, Duration::from_secs(2));
    assert_eq!(facts(&bf)?[0].0, Duration::from_secs(2));
    assert_eq!(facts(&ae)?[0], a_state);
    let unaffected = facts(&ae)?;
    let [own_b, own_f] = facts(&bf)?;
    let own_due = own_b.1.max(own_f.1).ok_or("independent B/F floor absent")?;
    assert!(own_due < shared_e.1.ok_or("shared E floor absent")?);
    let mut independent_probe = fixture_send(&bf, &Arc::new(()), own_due)?;
    fixture_receive(&bf, &mut independent_probe)?;
    assert_eq!(facts(&ae)?, unaffected);
    Ok(())
}

/// Keep capped RATE probes available without resetting policy after success.
/// # Errors
/// Preserve root/binding, request/control, record and missing-floor failures.
/// # Panics
/// Fail if saturation disables probes or success changes retained restriction facts.
#[test]
fn rate_at_cap_continues_probes_and_success_never_resets_retained_state() -> Check<()> {
    let authority = isolated_authority(limits(1, 1)?)?;
    let binding = authority.source(0)?.bind(endpoint(41001))?;
    let mut now = Instant::now();
    for expected in [2, 4, 4, 4] {
        let mut request = fixture_send(&binding, &Arc::new(()), now)?;
        fixture_control(&binding, &mut request, RATE, now)?;
        let current = facts(&binding)?;
        for record in current {
            assert_eq!(record.0, Duration::from_secs(expected));
            assert!(!record.2 && !record.5);
            assert_eq!(record.1, now.checked_add(Duration::from_secs(expected)));
        }
        now = current[0].1.ok_or("RATE floor absent")?;
    }
    let mut healthy = fixture_send(&binding, &Arc::new(()), now)?;
    let retained = facts(&binding)?;
    fixture_receive(&binding, &mut healthy)?;
    assert_eq!(facts(&binding)?, retained);
    let next = retained[0].1.ok_or("send floor absent")?;
    assert!(fixture_send(&binding, &Arc::new(()), next).is_ok());
    Ok(())
}

/// Reject unknown controls and recognize an admitted control at most once.
/// # Errors
/// Preserve construction, admitted control, record and healthy continuation failures.
/// # Panics
/// Fail if rejection changes state or the consumed request can act again.
#[test]
fn unknown_controls_and_recognized_replay_cannot_consume_or_apply_twice() -> Check<()> {
    let authority = isolated_authority(limits(1, 1)?)?;
    let binding = authority.source(0)?.bind(endpoint(41001))?;
    let now = Instant::now();
    let mut request = fixture_send(&binding, &Arc::new(()), now)?;
    let before = facts(&binding)?;
    assert!(matches!(
        fixture_control(&binding, &mut request, UNKNOWN, now),
        Err(SourcePolicyError::UnhandledControl)
    ));
    assert_eq!(facts(&binding)?, before);
    fixture_control(&binding, &mut request, RATE, now)?;
    let once = facts(&binding)?;
    assert!(fixture_control(&binding, &mut request, RATE, now).is_err());
    assert!(fixture_receive(&binding, &mut request).is_err());
    assert_eq!(facts(&binding)?, once);
    let due = once[0].1.ok_or("RATE floor absent")?;
    let mut healthy = fixture_send(&binding, &Arc::new(()), due)?;
    fixture_receive(&binding, &mut healthy)?;
    Ok(())
}

/// Clear only current selected refusals and reject complete invalid selections.
/// # Errors
/// Preserve valid construction, control, target, re-enable and record failures.
/// # Panics
/// Fail if an invalid selection partially acts or unrelated intervals/floors change.
#[test]
fn targeted_reenable_is_atomic_for_stale_foreign_duplicate_and_partial_selections() -> Check<()> {
    let mut authority = isolated_authority(limits(2, 2)?)?;
    let source = authority.source(0)?;
    let binding = source.bind(endpoint(41001))?;
    let neighbor = authority.source(1)?.bind(endpoint(41002))?;
    let now = Instant::now();
    let mut rate = fixture_send(&binding, &Arc::new(()), now)?;
    fixture_control(&binding, &mut rate, RATE, now)?;
    let due = facts(&binding)?[0].1.ok_or("RATE floor absent")?;
    let mut request = fixture_send(&binding, &Arc::new(()), due)?;
    fixture_control(&binding, &mut request, DENY, due)?;
    let original = facts(&binding)?;
    let untouched = facts(&neighbor)?;
    let stale_source = authority.source_target(&source)?;
    authority.reenable(std::slice::from_ref(&stale_source))?;
    let source_cleared = facts(&binding)?;
    assert!(!source_cleared[0].2 && source_cleared[1].2);
    assert_eq!(
        (source_cleared[0].0, source_cleared[0].1),
        (original[0].0, original[0].1)
    );
    assert_eq!(
        source_cleared[0].4,
        original[0]
            .4
            .checked_add(1)
            .ok_or("fixture epoch overflow")?
    );
    assert_eq!(source_cleared[1], original[1]);
    let current_e = authority.endpoint_target(endpoint(41001))?;
    assert!(authority.reenable(&[current_e, stale_source]).is_err());
    assert_eq!(facts(&binding)?, source_cleared);
    let foreign = isolated_authority(limits(1, 1)?)?;
    let foreign_source = foreign.source(0)?;
    let current_e = authority.endpoint_target(endpoint(41001))?;
    assert!(
        authority
            .reenable(&[current_e, foreign.source_target(&foreign_source)?])
            .is_err()
    );
    assert_eq!(facts(&binding)?, source_cleared);
    assert!(
        authority
            .reenable(&[
                authority.endpoint_target(endpoint(41001))?,
                authority.endpoint_target(endpoint(41001))?
            ])
            .is_err()
    );
    assert_eq!(facts(&binding)?, source_cleared);
    authority.reenable(&[authority.endpoint_target(endpoint(41001))?])?;
    let cleared = facts(&binding)?;
    assert!(!cleared[0].2 && !cleared[1].2);
    assert_eq!(cleared[0], source_cleared[0]);
    assert_eq!((cleared[1].0, cleared[1].1), (original[1].0, original[1].1));
    assert_eq!(facts(&neighbor)?, untouched);
    Ok(())
}

/// Keep old pending epochs stale after a refusal and explicit re-enable.
/// # Errors
/// Preserve construction, request/control, target, re-enable and record failures.
/// # Panics
/// Fail if old work reenters or its refusal changes retained policy facts.
#[test]
fn old_pending_cannot_become_current_after_refusal_and_targeted_reenable() -> Check<()> {
    let mut authority = isolated_authority(limits(1, 1)?)?;
    let source = authority.source(0)?;
    let binding = source.bind(endpoint(41001))?;
    let now = Instant::now();
    let mut old = fixture_send(&binding, &Arc::new(()), now)?;
    let due = facts(&binding)?[0].1.ok_or("send floor absent")?;
    let mut control = fixture_send(&binding, &Arc::new(()), due)?;
    fixture_control(&binding, &mut control, RSTR, due)?;
    authority.reenable(&[
        authority.source_target(&source)?,
        authority.endpoint_target(endpoint(41001))?,
    ])?;
    let retained = facts(&binding)?;
    assert!(fixture_receive(&binding, &mut old).is_err());
    assert_eq!(facts(&binding)?, retained);
    let next = retained[0].1.ok_or("retained floor absent")?;
    let mut healthy = fixture_send(&binding, &Arc::new(()), next)?;
    fixture_receive(&binding, &mut healthy)?;
    Ok(())
}

/// Find a representability boundary using the platform's own checked operation,
/// independently of the policy implementation and without a platform constant.
/// # Errors
/// Reject a platform boundary without the required neighboring representable instants.
/// # Panics
/// Fail if the independently located two/four-second neighbors are inconsistent.
fn near_instant_limit(base: Instant) -> Check<Instant> {
    let (mut low, mut high) = (0_u64, u64::MAX);
    while low < high {
        let distance = high - low;
        let middle = low + distance / 2 + distance % 2;
        if base.checked_add(Duration::from_secs(middle)).is_some() {
            low = middle;
        } else {
            high = middle - 1;
        }
    }
    let seconds = low
        .checked_sub(3)
        .ok_or("fixture has no overflow boundary neighbor")?;
    let near = base
        .checked_add(Duration::from_secs(seconds))
        .ok_or("fixture time overflow")?;
    assert!(near.checked_add(Duration::from_secs(2)).is_some());
    assert!(near.checked_add(Duration::from_secs(4)).is_none());
    Ok(near)
}

/// Retain exhausted state and preserve the independent healthy record.
/// # Errors
/// Preserve construction, request/control, record and representability-neighbor failures.
/// # Panics
/// Fail if exhaustion is cleared, prior facts are refunded or failed sends charge a neighbor.
#[test]
fn arithmetic_exhaustion_is_sticky_and_does_not_refund_or_charge_a_healthy_record() -> Check<()> {
    for admit_control in [true, false] {
        let mut authority = isolated_authority(limits(1, 3)?)?;
        let source = authority.source(0)?;
        let ae = source.bind(endpoint(41001))?;
        let af = source.bind(endpoint(41002))?;
        let ag = source.bind(endpoint(41003))?;
        let now = Instant::now();
        let mut first = fixture_send(&af, &Arc::new(()), now)?;
        fixture_control(&af, &mut first, RATE, now)?;
        let next = facts(&af)?[0].1.ok_or("source floor absent")?;
        let mut second = fixture_send(&ag, &Arc::new(()), next)?;
        fixture_control(&ag, &mut second, RATE, next)?;
        let near = near_instant_limit(now)?;
        if admit_control {
            let due = facts(&ae)?[0].1.ok_or("source floor absent")?;
            let mut control = fixture_send(&ae, &Arc::new(()), due)?;
            let before = facts(&ae)?;
            fixture_control(&ae, &mut control, RATE, near)?;
            let after = facts(&ae)?;
            assert!(after[0].5 && !after[1].5);
            assert_eq!((after[0].0, after[0].1), (before[0].0, before[0].1));
            assert_eq!(after[1].0, Duration::from_secs(2));
            assert_eq!(after[1].1, near.checked_add(Duration::from_secs(2)));
        } else {
            let before = facts(&ae)?;
            assert!(fixture_send(&ae, &Arc::new(()), near).is_err());
            let after = facts(&ae)?;
            assert!(after[0].5 && !after[1].5);
            assert_eq!(after[1], before[1]);
            assert_eq!((after[0].0, after[0].1), (before[0].0, before[0].1));
        }
        let _reenable_result = authority.reenable(&[authority.source_target(&source)?]);
        assert!(facts(&ae)?[0].5);
        let before_refused_send = facts(&ae)?;
        assert!(fixture_send(&ae, &Arc::new(()), near).is_err());
        assert_eq!(facts(&ae)?, before_refused_send);
    }
    Ok(())
}

struct IoFixture {
    authority: SourcePolicyAuthority,
    binding: SourceBinding,
    slots: [LiveSlot; 2],
    peers: [UdpSocket; 2],
    control_owner: watch::Sender<Control>,
    control: watch::Receiver<Control>,
    deadline: Instant,
}

/// Build the bounded current-thread runtime used by actual socket controls.
/// # Errors
/// Preserve original runtime construction and nested socket-registration errors.
/// # Panics
/// Reject a broken one-way retirement prerequisite before a dependent scenario.
fn runtime() -> io::Result<tokio::runtime::Runtime> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .max_blocking_threads(2)
        .build()?;
    crate::processing_guard_tests::require_retirement_contract(&runtime)
        .map_err(io::Error::other)?;
    Ok(runtime)
}

/// Create two genuine managed sockets and one original invocation control owner.
/// # Errors
/// Preserve runtime registration, socket, bounds, policy, conversion and deadline errors.
fn io_fixture() -> Check<IoFixture> {
    io_fixture_with_intervals(Duration::from_secs(30), Duration::from_secs(30))
}

/// Reuse the genuine two-socket fixture with explicit checked modeled intervals.
/// # Errors
/// Preserve the same allocation, configuration, socket and registration failures.
fn io_fixture_with_intervals(initial: Duration, maximum: Duration) -> Check<IoFixture> {
    let authority = isolated_authority(SourcePolicyLimits::new(2, 2, initial, maximum)?)?;
    let first_peer = std::net::UdpSocket::bind(endpoint(0))?;
    let second_peer = std::net::UdpSocket::bind(endpoint(0))?;
    first_peer.set_nonblocking(true)?;
    second_peer.set_nonblocking(true)?;
    let peers = [
        UdpSocket::from_std(first_peer)?,
        UdpSocket::from_std(second_peer)?,
    ];
    let binding = authority.source(0)?.bind(peers[0].local_addr()?)?;
    let neighbor = authority.source(1)?.bind(peers[1].local_addr()?)?;
    let context = Arc::new(RunClockContext {
        origin: Instant::now(),
    });
    let mut prepared = Vec::with_capacity(2);
    for (index, policy) in [binding.clone(), neighbor].into_iter().enumerate() {
        let mut packet = [0_u8; PACKET_BYTES];
        packet[0] = 0x23;
        packet[40..48].fill(u8::try_from(index + 1)?);
        let mut slot = crate::prepare_slot(peers[index].local_addr()?, packet)?;
        slot.source_policy = SourceMode::Managed(policy);
        prepared.push(LiveSlot::from_prepared(slot, Arc::clone(&context))?);
    }
    let slots: [LiveSlot; 2] = prepared
        .try_into()
        .map_err(|_| "fixture must own exactly two slots")?;
    let (control_owner, control) = watch::channel(Control {
        stop: false,
        epoch: 0,
    });
    let deadline = Instant::now()
        .checked_add(crate::MAX_TIMEOUT)
        .ok_or("fixture deadline overflow")?;
    Ok(IoFixture {
        authority,
        binding,
        slots,
        peers,
        control_owner,
        control,
        deadline,
    })
}

/// Invoke the real guarded sender and observe its exact complete peer datagram.
/// # Errors
/// Preserve policy, readiness, native receive and bounded-timeout failures.
/// # Panics
/// Fail if the actual send/pending state, source, length or packet differs.
async fn query(fixture: &mut IoFixture) -> Check<[u8; RECEIVE_BYTES]> {
    {
        let slot = &mut fixture.slots[0];
        slot.policy.refresh(&slot.identity)?;
    }
    fixture.slots[0].socket.writable().await?;
    crate::attempt_send(&mut fixture.slots[0], fixture.deadline, &fixture.control);
    read_emitted_query(fixture).await
}

/// Observe the actual complete query from either capture path.
/// # Errors
/// Preserve native receive and bounded-timeout failures.
/// # Panics
/// Fail if the actual send/pending state, source, length or packet differs.
async fn read_emitted_query(fixture: &IoFixture) -> Check<[u8; RECEIVE_BYTES]> {
    assert!(fixture.slots[0].sent);
    assert!(matches!(fixture.slots[0].phase, Phase::Pending(_)));
    let mut packet = [0_u8; RECEIVE_BYTES];
    let (length, source) = tokio::time::timeout(
        Duration::from_secs(2),
        fixture.peers[0].recv_from(&mut packet),
    )
    .await??;
    assert_eq!(length, PACKET_BYTES);
    assert_eq!(source, fixture.slots[0].local);
    assert_eq!(&packet[..length], &fixture.slots[0].packet);
    Ok(packet)
}

/// Use the actual selected-datagram/parse/commit driver, including its charge.
/// # Errors
/// Preserve native send/receive, bounded-timeout and frame-limit failures.
/// # Panics
/// Fail if the literal reply is not submitted as one complete datagram.
async fn reply(
    fixture: &mut IoFixture,
    query: &[u8; RECEIVE_BYTES],
    stratum: u8,
    code: u32,
) -> Check<()> {
    reply_observing(fixture, query, stratum, code, std::convert::identity).await
}

/// Model only the receive capture while keeping actual packet, charge and commit.
/// # Errors
/// Preserve native send/receive, bounded-timeout and frame-limit failures.
/// # Panics
/// Fail if the literal reply is not submitted as one complete datagram.
async fn reply_observing(
    fixture: &mut IoFixture,
    query: &[u8; RECEIVE_BYTES],
    stratum: u8,
    code: u32,
    observe: impl FnOnce(SoftwareClock) -> SoftwareClock,
) -> Check<()> {
    let mut packet = [0_u8; PACKET_BYTES];
    packet[0] = 0x24;
    packet[1] = stratum;
    packet[12..16].copy_from_slice(&code.to_be_bytes());
    packet[24..32].copy_from_slice(&query[40..48]);
    packet[32..40].copy_from_slice(&1_u64.to_be_bytes());
    packet[40..48].copy_from_slice(&2_u64.to_be_bytes());
    let mut buffers = [[0_u8; RECEIVE_BYTES]; 2];
    let (length, source) = tokio::time::timeout(Duration::from_secs(2), async {
        assert_eq!(
            fixture.peers[0]
                .send_to(&packet, fixture.slots[0].local)
                .await?,
            PACKET_BYTES
        );
        fixture.slots[0].socket.recv_from(&mut buffers[0]).await
    })
    .await??;
    let read: SoftwareClock = observe(SoftwareClock::capture());
    let result =
        crate::classify_read_result(NativeDatagramPlatform::current(), Ok((length, source)));
    let mut control_open = true;
    crate::handle_wake(
        &mut fixture.slots,
        &mut fixture.control,
        &mut control_open,
        Wake::Read {
            slot: 0,
            result,
            read: ApplicationClockCapture::software_only(read),
        },
        &buffers,
        ntp_wire::FrameLimits::new(PACKET_BYTES, 0).map_err(|_| "fixture frame limit refused")?,
        fixture.deadline,
    );
    Ok(())
}

/// Original or modeled coordinate placement at the same receive boundary.
#[derive(Clone, Copy)]
enum ReceiveCutoffCapture {
    /// Use the actual healthy bracket.
    Original,
    /// Model a reversed bracket whose first reading is exactly at cutoff.
    FirstAt,
    /// Model a reversed bracket whose first reading is one nanosecond over.
    FirstOver,
    /// Model an ordered bracket whose last reading is exactly at cutoff.
    LastAt,
}

/// The original receive cutoff dominates publication and policy effect.
/// # Errors
/// Preserve actual socket, control, policy snapshot and cutoff fixture failures.
/// # Panics
/// Fail on incorrect terminal precedence, repeated accounting or policy change.
#[test]
fn managed_plain_receive_cutoff_charges_once_before_policy_effect() -> Check<()> {
    runtime()?.block_on(async {
        use ReceiveCutoffCapture::{FirstAt, FirstOver, LastAt, Original};

        let stop: Control = Control {
            stop: true,
            epoch: 0,
        };
        let stale: Control = Control {
            stop: false,
            epoch: 1,
        };
        let cases: [(&str, ReceiveCutoffCapture, Option<Control>, Status); 6] = [
            ("healthy", Original, None, Status::Control),
            ("first equals cutoff", FirstAt, None, Status::Timeout),
            ("first exceeds cutoff", FirstOver, None, Status::Timeout),
            ("last equals cutoff", LastAt, None, Status::Timeout),
            (
                "stop before late reply",
                FirstAt,
                Some(stop),
                Status::Stopped,
            ),
            (
                "invalidation before late reply",
                FirstAt,
                Some(stale),
                Status::Stale,
            ),
        ];
        for (name, capture, current, expected) in cases {
            let mut fixture: IoFixture = io_fixture()?;
            let packet: [u8; RECEIVE_BYTES] = query(&mut fixture).await?;
            let before: [RecordFacts; 2] = facts(&fixture.binding)?;
            assert!(before.iter().all(|record: &RecordFacts| !record.2));
            let deadline: Instant = fixture.deadline;
            let after_deadline: Instant = deadline
                .checked_add(Duration::from_nanos(1))
                .ok_or("one-nanosecond cutoff neighbor is unavailable")?;
            if let Some(current) = current {
                let _previous: Control = fixture.control_owner.send_replace(current);
            }
            reply_observing(&mut fixture, &packet, 0, DENY, |read: SoftwareClock| {
                assert!(read.mono <= read.mono_after && read.mono_after < deadline);
                match capture {
                    Original => read,
                    FirstAt => SoftwareClock {
                        mono: deadline,
                        ..read
                    },
                    FirstOver => SoftwareClock {
                        mono: after_deadline,
                        ..read
                    },
                    LastAt => SoftwareClock {
                        mono_after: deadline,
                        ..read
                    },
                }
            })
            .await?;
            assert!(
                Instant::now() < deadline,
                "fixture processing reached native cutoff"
            );
            let [slot, _neighbor]: [LiveSlot; 2] = fixture.slots;
            assert_eq!(
                slot.terminal().map(TerminalOutcome::status),
                Some(expected),
                "receive capture cutoff disposition differs for {name}"
            );
            let report: SlotReport = slot.into_report(0)?;
            assert_eq!(
                (report.received(), report.rejected()),
                (1, u16::from(expected != Status::Control)),
                "receive capture cutoff charge differs for {name}"
            );
            assert!(report.facts().is_none());
            let after: [RecordFacts; 2] = facts(&fixture.binding)?;
            if expected == Status::Control {
                assert!(after.iter().all(|record: &RecordFacts| record.2));
            } else {
                assert_eq!(
                    after, before,
                    "late receive changed managed policy for {name}"
                );
            }
        }
        Ok(())
    })
}

/// A reversed bracket wholly before cutoff retains the original numerical refusal.
/// # Errors
/// Preserve actual socket, profile, cutoff and historical report failures.
/// # Panics
/// Fail if receipt accounting or original capture-order data changes.
#[test]
fn managed_plain_reversed_receive_before_cutoff_keeps_header_history() -> Check<()> {
    runtime()?.block_on(async {
        let mut fixture: IoFixture = io_fixture()?;
        fixture.slots[0].profile = Some(crate::ExperimentalProfile::new(
            2_272_060_800,
            6_311_433_599,
            30_000_000_000,
        )?);
        let packet: [u8; RECEIVE_BYTES] = query(&mut fixture).await?;
        let before: [RecordFacts; 2] = facts(&fixture.binding)?;
        let deadline: Instant = fixture.deadline;
        let lower: Instant = deadline
            .checked_sub(Duration::from_nanos(1))
            .ok_or("one-nanosecond pre-cutoff neighbor is unavailable")?;
        reply_observing(&mut fixture, &packet, 2, 0, |read: SoftwareClock| {
            assert!(read.mono_after < lower && lower < deadline);
            SoftwareClock {
                mono: lower,
                ..read
            }
        })
        .await?;
        assert!(
            Instant::now() < deadline,
            "fixture processing reached native cutoff"
        );
        let [slot, _neighbor]: [LiveSlot; 2] = fixture.slots;
        let report: SlotReport = slot.into_report(0)?;
        assert_eq!(report.status(), Status::Header);
        assert_eq!((report.received(), report.rejected()), (1, 0));
        let historical: &crate::TransportFacts = report
            .facts()
            .ok_or("pre-cutoff header lost its original facts")?;
        assert_eq!(historical.read_return().monotonic(), lower);
        assert!(historical.read_return().monotonic_after() < lower);
        assert!(matches!(
            historical.measurement(),
            Err(crate::MeasurementRefusal::CaptureOrder)
        ));
        assert_eq!(facts(&fixture.binding)?, before);
        Ok(())
    })
}

/// Check the writer against the original committed readings with no mapping profile.
/// # Errors
/// Preserve actual socket, capture-neighbor, retained report and writer failures.
/// # Panics
/// Fail if the original readings, numerical refusal or exact report bytes differ.
#[test]
fn managed_header_writes_receive_to_commit_age_only_for_ordered_capture() -> Check<()> {
    use std::fmt::Write as _;

    const PREFIX: &str = "event=measurement slot=0 status=unavailable reason=mapping_unavailable authentication=none eligible=0 capture=software_bracketed mapping=absent wire_error=unknown";

    runtime()?.block_on(async {
        for case in ["ordered", "receive-reversed", "send-return-reversed"] {
            let receive_reversed: bool = case == "receive-reversed";
            let send_return_reversed: bool = case == "send-return-reversed";
            let mut fixture: IoFixture = io_fixture()?;
            assert!(fixture.slots[0].profile.is_none());
            let packet: [u8; RECEIVE_BYTES] = query(&mut fixture).await?;
            let Phase::Pending(sent) = &mut fixture.slots[0].phase else {
                return Err("actual send is not pending".into());
            };
            let earlier: Instant = sent.after.software().mono_after;
            let later: Instant = earlier
                .checked_add(Duration::from_nanos(1))
                .ok_or("one-nanosecond receive capture neighbor is unavailable")?;
            let (leading, trailing): (Instant, Instant) = if receive_reversed {
                (later, earlier)
            } else if send_return_reversed {
                // Reverse only the middle bracket after the real send returned.
                sent.after = ApplicationClockCapture::software_only(SoftwareClock {
                    mono: later,
                    ..sent.after.software()
                });
                (later, later)
            } else {
                (earlier, later)
            };
            reply_observing(&mut fixture, &packet, 2, 0, |read: SoftwareClock| {
                assert!(later < read.mono && read.mono <= read.mono_after);
                SoftwareClock {
                    mono: leading,
                    mono_after: trailing,
                    ..read
                }
            })
            .await?;

            let [slot, _neighbor]: [LiveSlot; 2] = fixture.slots;
            let report: SlotReport = slot.into_report(0)?;
            assert_eq!(report.status(), Status::Header);
            assert_eq!((report.received(), report.rejected()), (1, 0));
            let historical: &crate::TransportFacts = report
                .facts()
                .ok_or("committed header lost its original facts")?;
            let read: SoftwareClock = historical.read_return();
            let committed: Instant = historical.processing();
            assert_eq!(
                (read.monotonic(), read.monotonic_after()),
                (leading, trailing)
            );
            assert!(later < committed);
            assert!(matches!(
                historical.measurement(),
                Err(crate::MeasurementRefusal::Unavailable)
            ));

            let mut expected: String = String::from(PREFIX);
            if !receive_reversed {
                let elapsed_ns: u128 = (committed - trailing).as_nanos();
                write!(expected, " receive_to_commit_ns={elapsed_ns}")?;
            }
            if case == "ordered" {
                let first: SoftwareClock = historical.send_before();
                let lower_ns: u128 = (leading - first.monotonic_after()).as_nanos();
                let upper_ns: u128 = (trailing - first.monotonic()).as_nanos();
                write!(
                    expected,
                    " app_wall_elapsed_min_ns={lower_ns} app_wall_elapsed_max_ns={upper_ns}"
                )?;
            }
            expected.push('\n');
            let mut written: Vec<u8> = Vec::new();
            report.write_measurement(&mut written)?;
            assert_eq!(std::str::from_utf8(&written)?, expected, "case={case}");
        }
        Ok(())
    })
}

/// Reject reversed or pre-send receive readings before a managed control changes policy.
/// # Errors
/// Preserve actual socket, control and policy-observation failures.
/// # Panics
/// Fail if chronology refusal changes the pending request, accounting or policy,
/// or if an ordered reply cannot complete the original request.
#[test]
fn managed_plain_reversed_receive_leaves_control_pending_until_ordered_reply() -> Check<()> {
    let runtime: tokio::runtime::Runtime = runtime()?;
    let reversed_bracket: Check<()> = runtime.block_on(async {
        let mut fixture: IoFixture = io_fixture()?;
        let packet: [u8; RECEIVE_BYTES] = query(&mut fixture).await?;
        let deadline: Instant = fixture.deadline;
        let Phase::Pending(sent) = &fixture.slots[0].phase else {
            return Err("actual send is not pending".into());
        };
        let send_span: (Instant, Instant) = (
            sent.before.software().mono,
            sent.after.software().mono_after,
        );
        let before: [RecordFacts; 2] = facts(&fixture.binding)?;
        assert!(
            before
                .iter()
                .all(|record: &RecordFacts| record.1.is_some() && !record.2)
        );

        reply_observing(&mut fixture, &packet, 0, DENY, |read: SoftwareClock| {
            assert!(send_span.1 < read.mono && read.mono <= read.mono_after);
            assert!(read.mono_after < deadline);
            SoftwareClock {
                mono_after: send_span.1,
                ..read
            }
        })
        .await?;

        let pending: &LiveSlot = &fixture.slots[0];
        assert_eq!(
            pending.terminal().map(TerminalOutcome::status),
            None,
            "reversed receive capture must leave the original control request pending"
        );
        assert!(matches!(&pending.phase, Phase::Pending(saved)
            if (saved.before.software().mono, saved.after.software().mono_after) == send_span));
        assert_eq!(pending.packet[40..48], packet[40..48]);
        assert_eq!(
            (pending.send_attempts, pending.received, pending.rejected),
            (1, 1, 1)
        );
        assert_eq!(fixture.deadline, deadline);
        assert_eq!(facts(&fixture.binding)?, before);

        reply(&mut fixture, &packet, 0, DENY).await?;
        let [slot, _neighbor]: [LiveSlot; 2] = fixture.slots;
        assert_eq!(slot.send_attempts, 1);
        let report: SlotReport = slot.into_report(0)?;
        assert_eq!(
            (report.status(), report.control_code()),
            (Status::Control, Some(DENY))
        );
        assert_eq!((report.received(), report.rejected()), (2, 1));
        assert!(report.facts().is_none());
        let after: [RecordFacts; 2] = facts(&fixture.binding)?;
        for (prior, denied) in before.iter().zip(&after) {
            assert_eq!((denied.1, denied.2), (prior.1, true));
        }
        Ok(())
    });
    reversed_bracket?;
    runtime.block_on(managed_plain_receive_start_boundary())
}

/// Compare the original reserved send with the receive start under actual policy custody.
/// # Errors
/// Preserve socket, control, checked capture arithmetic and policy-observation failures.
/// # Panics
/// Fail if a receive that starts before the send changes Pending, work or policy,
/// or if exact equality cannot complete the original request.
async fn managed_plain_receive_start_boundary() -> Check<()> {
    let mut fixture: IoFixture = io_fixture()?;
    fixture.slots[0].socket.writable().await?;
    let deadline: Instant = fixture.deadline;
    let sent_at: Instant = Instant::now();
    let send: SoftwareClock = SoftwareClock {
        mono: sent_at,
        utc: std::time::SystemTime::UNIX_EPOCH,
        mono_after: sent_at,
    };
    let early: Instant = sent_at
        .checked_sub(Duration::from_micros(100))
        .ok_or("modeled receive start underflow")?;
    let regressed_return: SoftwareClock = SoftwareClock {
        mono: early,
        mono_after: early,
        ..send
    };
    let initial: [RecordFacts; 2] = facts(&fixture.binding)?;
    let (captures, count): (observations::SendReads, observations::ReadCount) =
        observations::SendReads::fixed_raw(send, regressed_return);
    let current = fixture.control.borrow();
    let effect: ManagedEffect =
        attempt_managed_send_observing(&mut fixture.slots[0], deadline, captures);
    drop(current);
    assert!(matches!(effect, ManagedEffect::Handled));
    assert_eq!(count.reads(), 2);
    let packet: [u8; RECEIVE_BYTES] = read_emitted_query(&fixture).await?;
    assert_modeled_send_custody(&mut fixture, send, regressed_return, initial)?;
    let retained: [RecordFacts; 2] = facts(&fixture.binding)?;

    reply_observing(&mut fixture, &packet, 0, DENY, |read: SoftwareClock| {
        assert!(early < send.mono_after && send.mono_after <= read.mono);
        assert!(read.mono_after < deadline);
        SoftwareClock {
            mono: early,
            mono_after: send.mono_after,
            ..read
        }
    })
    .await?;

    let pending: &LiveSlot = &fixture.slots[0];
    assert_eq!(
        pending.terminal().map(TerminalOutcome::status),
        None,
        "receive started before the reserved send and must leave the original control pending"
    );
    assert!(matches!(&pending.phase, Phase::Pending(saved)
        if saved.before.software().mono_after == send.mono_after && saved.after.software().mono_after == early));
    assert_eq!(pending.packet[40..48], packet[40..48]);
    assert_eq!(
        (pending.send_attempts, pending.received, pending.rejected),
        (1, 1, 1)
    );
    assert_eq!(fixture.deadline, deadline);
    assert_eq!(facts(&fixture.binding)?, retained);

    reply_observing(&mut fixture, &packet, 0, DENY, |read: SoftwareClock| {
        SoftwareClock {
            mono: send.mono_after,
            mono_after: send.mono_after,
            ..read
        }
    })
    .await?;
    let [slot, _neighbor]: [LiveSlot; 2] = fixture.slots;
    assert_eq!(slot.send_attempts, 1);
    let report: SlotReport = slot.into_report(0)?;
    assert_eq!(
        (report.status(), report.control_code()),
        (Status::Control, Some(DENY))
    );
    assert_eq!((report.received(), report.rejected()), (2, 1));
    assert!(report.facts().is_none());
    let denied: [RecordFacts; 2] = facts(&fixture.binding)?;
    for (before, after) in retained.iter().zip(&denied) {
        assert_eq!((after.1, after.2), (before.1, true));
    }
    Ok(())
}

/// Retain the real pending request after an unknown control and accept its healthy reply.
/// # Errors
/// Preserve runtime, socket, timeout, policy and missing-pending failures.
/// # Panics
/// Fail if rejection refreshes accounting/deadline or a future send floor rejects the reply.
#[test]
fn actual_unknown_control_retains_pending_budget_and_accepts_reply_before_send_floor() -> Check<()>
{
    runtime()?.block_on(async {
        let mut fixture = io_fixture()?;
        let packet = query(&mut fixture).await?;
        let original_deadline = fixture.deadline;
        let Phase::Pending(sent) = &fixture.slots[0].phase else {
            return Err("actual send is not pending".into());
        };
        let send_coordinates: (Instant, Instant) = (
            sent.before.software().mono,
            sent.after.software().mono_after,
        );
        let retained = facts(&fixture.binding)?;
        reply(&mut fixture, &packet, 0, UNKNOWN).await?;
        let slot = &fixture.slots[0];
        let Phase::Pending(saved) = &slot.phase else {
            return Err("unknown code consumed Pending".into());
        };
        assert_eq!(
            (
                saved.before.software().mono,
                saved.after.software().mono_after
            ),
            send_coordinates
        );
        assert_eq!((slot.received, slot.rejected), (1, 1));
        assert_eq!(slot.last_rejection, Some(Rejection::UnhandledControl));
        assert_eq!(fixture.deadline, original_deadline);
        assert_eq!(facts(&fixture.binding)?, retained);
        assert!(Instant::now() < retained[0].1.ok_or("actual send floor absent")?);
        reply(&mut fixture, &packet, 2, 0).await?;
        assert_eq!(
            fixture.slots[0].terminal().map(TerminalOutcome::status),
            Some(Status::Header)
        );
        assert_eq!(
            (fixture.slots[0].received, fixture.slots[0].rejected),
            (2, 1)
        );
        assert_eq!(facts(&fixture.binding)?, retained);
        Ok(())
    })
}

/// Exhaust the original receive allowance with actual rejected controls.
/// # Errors
/// Preserve runtime, socket, timeout and policy failures.
/// # Panics
/// Fail if received work is refunded or exhaustion leaves the request active.
#[test]
fn actual_repeated_unknown_controls_spend_the_original_receive_allowance() -> Check<()> {
    runtime()?.block_on(async {
        let mut fixture = io_fixture()?;
        let packet = query(&mut fixture).await?;
        let retained = facts(&fixture.binding)?;
        let original_deadline = fixture.deadline;
        for charged in 1..=RECEIVE_CAP {
            reply(&mut fixture, &packet, 0, UNKNOWN).await?;
            assert_eq!(fixture.slots[0].received, charged);
            assert_eq!(fixture.slots[0].rejected, charged);
            assert!(matches!(fixture.slots[0].phase, Phase::Pending(_)));
            assert_eq!(fixture.deadline, original_deadline);
        }
        assert!(!fixture.slots[0].can_receive());
        fixture.slots[0].expire();
        assert_eq!(
            fixture.slots[0].terminal().map(TerminalOutcome::status),
            Some(Status::Budget)
        );
        assert_eq!(facts(&fixture.binding)?, retained);
        Ok(())
    })
}

/// Consume one real RATE control without publishing time facts or replaying it.
/// # Errors
/// Preserve runtime, socket, timeout, policy and record-observation failures.
/// # Panics
/// Fail if replay mutates policy or a control yields ordinary transport facts.
#[test]
fn actual_recognized_replay_cannot_apply_rate_or_publish_facts_twice() -> Check<()> {
    runtime()?.block_on(async {
        let mut fixture = io_fixture()?;
        let packet = query(&mut fixture).await?;
        reply(&mut fixture, &packet, 0, RATE).await?;
        assert_eq!(
            fixture.slots[0].terminal().map(TerminalOutcome::status),
            Some(Status::Control)
        );
        assert!(
            fixture.slots[0]
                .terminal()
                .and_then(TerminalOutcome::facts)
                .is_none()
        );
        let once = facts(&fixture.binding)?;
        reply(&mut fixture, &packet, 0, RATE).await?;
        assert_eq!(facts(&fixture.binding)?, once);
        assert!(
            fixture.slots[0]
                .terminal()
                .and_then(TerminalOutcome::facts)
                .is_none()
        );
        Ok(())
    })
}

/// Recheck stop and source restrictions after actual writable readiness.
/// # Errors
/// Preserve runtime, socket, readiness, policy and healthy-neighbor failures.
/// # Panics
/// Fail if the restricted slot sends or its no-send path changes retained debits.
#[test]
fn stop_or_restriction_after_readiness_prevents_the_concrete_managed_send() -> Check<()> {
    runtime()?.block_on(async {
        for stop in [true, false] {
            let mut fixture = io_fixture()?;
            {
                let slot = &mut fixture.slots[0];
                slot.policy.refresh(&slot.identity)?;
            }
            fixture.slots[0].socket.writable().await?;
            if stop {
                fixture.control_owner.send_replace(Control { stop: true, epoch: 0 });
            } else {
                let now = Instant::now();
                let mut control = fixture_send(&fixture.binding, &Arc::new(()), now)?;
                fixture_control(&fixture.binding, &mut control, DENY, now)?;
            }
            let retained = facts(&fixture.binding)?;
            crate::attempt_send(&mut fixture.slots[0], fixture.deadline, &fixture.control);
            assert!(!fixture.slots[0].sent);
            assert_eq!(facts(&fixture.binding)?, retained);
            let mut bytes = [0_u8; RECEIVE_BYTES];
            assert!(matches!(fixture.peers[0].try_recv_from(&mut bytes), Err(error) if error.kind() == io::ErrorKind::WouldBlock));
            assert!(fixture.slots[0].terminal().is_some());
        }
        let mut healthy = io_fixture()?;
        let _actual_query = query(&mut healthy).await?;
        Ok(())
    })
}

/// Keep one source refusal across replacement managed sessions.
/// # Errors
/// Preserve root/binding, socket, configuration, control and record failures.
/// # Panics
/// Fail if new preparation bypasses refusal, emits a packet or resets policy.
#[test]
fn new_managed_sessions_and_series_cannot_reset_a_retained_refusal() -> Check<()> {
    let peer = std::net::UdpSocket::bind(endpoint(0))?;
    peer.set_nonblocking(true)?;
    let authority = isolated_authority(limits(1, 1)?)?;
    let source = authority.source(0)?;
    let binding = source.bind(peer.local_addr()?)?;
    let now = Instant::now();
    let mut control = fixture_send(&binding, &Arc::new(()), now)?;
    fixture_control(&binding, &mut control, DENY, now)?;
    let retained = facts(&binding)?;
    for _ in 0..2 {
        let config = SeriesConfig::new(
            peer.local_addr()?,
            Duration::from_millis(100),
            Duration::from_secs(1),
        )?;
        let mut session = source.session(config)?;
        assert!(matches!(
            session.prepare(),
            Err(crate::AcquisitionError::SourcePolicy(
                SourcePolicyError::Disabled
            ))
        ));
        // Pre-entropy refusal leaves this same session reusable as an owner,
        // but another preparation still cannot bypass the retained restriction.
        assert!(matches!(
            session.prepare(),
            Err(crate::AcquisitionError::SourcePolicy(
                SourcePolicyError::Disabled
            ))
        ));
        assert_eq!(facts(&binding)?, retained);
        let mut bytes = [0_u8; RECEIVE_BYTES];
        assert!(
            matches!(peer.recv_from(&mut bytes), Err(error) if error.kind() == io::ErrorKind::WouldBlock)
        );
    }
    Ok(())
}

/// Reject a newly restricted source at the actual prepared-series setup.
/// # Errors
/// Preserve preparation, socket, configuration, control and execution failures.
/// # Panics
/// Fail if refusal invents a socket report, sends a packet or changes policy facts.
#[test]
fn admitted_managed_plan_rechecks_restriction_before_series_socket_setup() -> Check<()> {
    let peer = std::net::UdpSocket::bind(endpoint(0))?;
    peer.set_nonblocking(true)?;
    let authority = isolated_authority(limits(1, 1)?)?;
    let source = authority.source(0)?;
    let binding = source.bind(peer.local_addr()?)?;
    let config = SeriesConfig::new(
        peer.local_addr()?,
        Duration::from_millis(100),
        Duration::from_secs(1),
    )?;
    let mut session = source.session(config)?;
    let (prepared, _control_owner) = session.prepare()?;
    let now = Instant::now();
    let mut control = fixture_send(&binding, &Arc::new(()), now)?;
    fixture_control(&binding, &mut control, DENY, now)?;
    let retained = facts(&binding)?;
    let report = prepared.execute(&crate::ControlSchedule::new(None, None)?)?;
    assert_eq!(report.series().end(), crate::SeriesEnd::Failure);
    assert!(report.series().primary_failure().is_some());
    assert!(
        report
            .series()
            .attempts()
            .iter()
            .all(|attempt| attempt.report().is_none())
    );
    assert_eq!(facts(&binding)?, retained);
    let mut bytes = [0_u8; RECEIVE_BYTES];
    assert!(
        matches!(peer.recv_from(&mut bytes), Err(error) if error.kind() == io::ErrorKind::WouldBlock)
    );
    Ok(())
}

/// Refresh a genuinely pending slot after current-revision operator re-enable.
/// A matching healthy reply remains historical after the same later operation.
/// # Errors
/// Preserve actual socket, selected-reply, target and policy observation failures.
/// # Panics
/// Reject lost charged work, stale publication, changed floors or an extra query.
async fn active_refresh_keeps_charged_history(invalidate_pending: bool) -> Check<()> {
    let mut fixture = io_fixture()?;
    let packet = query(&mut fixture).await?;
    let original_deadline = fixture.deadline;
    for _ in 0..3 {
        reply(&mut fixture, &packet, 0, UNKNOWN).await?;
    }
    let retained = facts(&fixture.binding)?;
    let neighbor = fixture
        .authority
        .source(1)?
        .bind(fixture.peers[1].local_addr()?)?;
    let neighbor_facts = facts(&neighbor)?;
    crate::refresh_source_policy(&mut fixture.slots[0]);
    assert!(matches!(&fixture.slots[0].phase, Phase::Pending(_)));
    assert_eq!(
        (fixture.slots[0].received, fixture.slots[0].rejected),
        (3, 3)
    );
    assert_eq!(
        fixture.slots[0].last_rejection,
        Some(Rejection::UnhandledControl)
    );
    assert_eq!(facts(&fixture.binding)?, retained);
    let historical = if invalidate_pending {
        None
    } else {
        reply(&mut fixture, &packet, 2, 0).await?;
        let result = fixture.slots[0]
            .terminal()
            .and_then(TerminalOutcome::facts)
            .ok_or("healthy pending neighbor did not commit its header")?;
        Some((
            result.processing,
            result.read_return.mono_after,
            Arc::clone(&result.identity),
            result
                .source_identity()
                .ok_or("committed managed history lost its source identity")?
                .clone(),
        ))
    };
    let source = fixture.authority.source(0)?;
    let target = fixture.authority.source_target(&source)?;
    fixture.authority.reenable(&[target])?;
    let changed = facts(&fixture.binding)?;
    assert_eq!(changed[0].0, retained[0].0);
    assert_eq!(changed[0].1, retained[0].1);
    assert_eq!(changed[0].3, retained[0].3 + 1);
    assert_eq!(changed[0].4, retained[0].4 + 1);
    assert_eq!(changed[1], retained[1]);
    for _ in 0..2 {
        crate::refresh_source_policy(&mut fixture.slots[0]);
        let slot = &fixture.slots[0];
        if let Some((processing, read, identity, source_identity)) = &historical {
            assert_eq!((slot.received, slot.rejected), (4, 3));
            let result = slot
                .terminal()
                .and_then(TerminalOutcome::facts)
                .ok_or("later refresh rewrote the historical header")?;
            assert_eq!(
                (result.processing, result.read_return.mono_after),
                (*processing, *read)
            );
            assert!(Arc::ptr_eq(&result.identity, identity));
            assert!(source.identity().same_source(source_identity));
            assert!(
                result
                    .source_identity()
                    .is_some_and(|retained| retained.same_source(source_identity))
            );
        } else {
            assert!(
                matches!(
                    slot.terminal(),
                    Some(TerminalOutcome::SourcePolicy(SourcePolicyError::Stale))
                ),
                "active policy refresh must retire the genuine stale pending request"
            );
            assert_eq!((slot.received, slot.rejected), (3, 3));
        }
        assert_eq!(slot.last_rejection, Some(Rejection::UnhandledControl));
        assert!(slot.sent);
        assert_eq!(slot.send_attempts, 1);
        assert_eq!(fixture.deadline, original_deadline);
        assert_eq!(facts(&fixture.binding)?, changed);
        assert_eq!(facts(&neighbor)?, neighbor_facts);
    }
    let mut extra = [0_u8; RECEIVE_BYTES];
    assert!(
        matches!(fixture.peers[0].try_recv_from(&mut extra), Err(error) if error.kind() == io::ErrorKind::WouldBlock)
    );
    Ok(())
}

/// Reject an actual old reply after targeted source/endpoint re-enable.
/// # Errors
/// Preserve runtime, socket, timeout, policy, target and record failures.
/// # Panics
/// Fail if stale work publishes facts or changes the re-enabled policy.
#[test]
fn actual_old_reply_cannot_reenter_after_both_correlation_epochs_advance() -> Check<()> {
    runtime()?.block_on(async {
        for invalidate_pending in [false, true] {
            active_refresh_keeps_charged_history(invalidate_pending).await?;
        }
        let mut fixture = io_fixture()?;
        let packet = query(&mut fixture).await?;
        let due = facts(&fixture.binding)?[0]
            .1
            .ok_or("actual send floor absent")?;
        let mut later_control = fixture_send(&fixture.binding, &Arc::new(()), due)?;
        fixture_control(&fixture.binding, &mut later_control, DENY, due)?;
        let source = fixture.authority.source(0)?;
        fixture.authority.reenable(&[
            fixture.authority.source_target(&source)?,
            fixture
                .authority
                .endpoint_target(fixture.peers[0].local_addr()?)?,
        ])?;
        let retained = facts(&fixture.binding)?;
        reply(&mut fixture, &packet, 2, 0).await?;
        assert!(
            fixture.slots[0]
                .terminal()
                .and_then(TerminalOutcome::facts)
                .is_none()
        );
        assert_eq!(
            fixture.slots[0].terminal().map(TerminalOutcome::status),
            Some(Status::SourcePolicy)
        );
        assert_eq!(fixture.slots[0].received, 1);
        assert_eq!(facts(&fixture.binding)?, retained);
        Ok(())
    })
}

/// Retain a registered contention wake even when release precedes polling.
/// # Errors
/// Preserve runtime, root/binding, lock and bounded-wait failures.
/// # Panics
/// Fail if contention is writable or the released waiter does not complete.
#[test]
fn contended_wait_is_registered_before_lock_and_observes_release_before_poll() -> Check<()> {
    runtime()?.block_on(async {
        let authority = isolated_authority(limits(1, 1)?)?;
        let binding = authority.source(0)?.bind(endpoint(41001))?;
        let request = Arc::new(());
        let mut live = LivePolicy::from_mode(SourceMode::Managed(binding.clone()));
        let guard = binding.source.cell.lock()?;
        live.refresh(&request)?;
        assert!(!live.wants_writable());
        // Release before wake is first polled: registering after try_lock would
        // lose this actual notification and make the bounded wait expire.
        drop(guard);
        tokio::time::timeout(Duration::from_secs(2), live.wake()).await?;
        live.refresh(&request)?;
        assert!(live.wants_writable());
        Ok(())
    })
}

/// Separate read-only release notifications from actual future-floor changes.
/// # Errors
/// Preserve runtime, root/binding, reservation, refresh and record failures.
/// # Panics
/// Fail if a future floor becomes writable, self-wakes or changes retained facts.
#[test]
fn read_only_floor_observations_do_not_wake_their_own_future_floor_wait() -> Check<()> {
    runtime()?.block_on(async {
        let authority = isolated_authority(SourcePolicyLimits::new(
            1,
            1,
            Duration::from_secs(30),
            Duration::from_secs(30),
        )?)?;
        let binding = authority.source(0)?.bind(endpoint(41001))?;
        let request = Arc::new(());
        let _reservation = fixture_send(&binding, &request, Instant::now())?;
        let retained = facts(&binding)?;
        let mut live = LivePolicy::from_mode(SourceMode::Managed(binding.clone()));
        for _ in 0..3 {
            live.refresh(&request)?;
            assert!(!live.wants_writable());
            assert!(
                tokio::time::timeout(Duration::from_millis(10), live.wake())
                    .await
                    .is_err()
            );
            assert_eq!(facts(&binding)?, retained);
        }
        Ok(())
    })
}

/// Retain real reservation debits when modeling a native `WouldBlock` result.
/// # Errors
/// Preserve runtime, socket, policy, record and missing-floor failures.
/// # Panics
/// Fail if modeled completion refunds either debit or bypasses the retained send floor.
#[test]
fn modeled_would_block_keeps_both_real_kernel_reservation_debits() -> Check<()> {
    runtime()?.block_on(async {
        let mut fixture = io_fixture()?;
        {
            let slot = &mut fixture.slots[0];
            slot.policy.refresh(&slot.identity)?;
            assert!(slot.can_send());
        }
        let now = Instant::now();
        let _reserved_request = fixture_send(&fixture.binding, &fixture.slots[0].identity, now)?;
        let debited = facts(&fixture.binding)?;
        for record in debited {
            assert_eq!(record.1, now.checked_add(Duration::from_secs(30)));
        }
        // The reservation is the actual production kernel. Only the native
        // return is modeled here; this asserts no observed OS WouldBlock fact.
        let before = SoftwareClock::capture();
        let after = SoftwareClock::capture();
        complete_send(
            &mut fixture.slots[0],
            Err(io::Error::from(io::ErrorKind::WouldBlock)),
            SendBracket {
                before: ApplicationClockCapture::software_only(before),
                after: ApplicationClockCapture::software_only(after),
            },
        );
        assert!(matches!(fixture.slots[0].phase, Phase::Sending));
        assert!(!fixture.slots[0].sent);
        assert_eq!(facts(&fixture.binding)?, debited);
        {
            let slot = &mut fixture.slots[0];
            slot.policy.refresh(&slot.identity)?;
            assert!(!slot.can_send());
        }
        assert!(matches!(
            fixture_send(&fixture.binding, &Arc::new(()), now),
            Err(SourcePolicyError::NotDue)
        ));
        assert_eq!(facts(&fixture.binding)?, debited);
        let due = debited[0].1.ok_or("charged send floor absent")?;
        let mut healthy = fixture_send(&fixture.binding, &Arc::new(()), due)?;
        fixture_receive(&fixture.binding, &mut healthy)?;
        Ok(())
    })
}

/// Capture one genuine selected response at the staging seam so proposal-only
/// substitutions can be checked without replacing any actual request identity.
/// # Errors
/// Preserve actual socket, timeout, frame-limit and checked staging failures.
/// # Panics
/// Fail if the complete literal reply cannot be sent as expected.
async fn ordinary_candidate(
    fixture: &mut IoFixture,
    query: &[u8; RECEIVE_BYTES],
) -> Check<StagedReply> {
    ordinary_candidate_observing(fixture, query, None).await
}

/// Stage genuine response bytes with a hardware or complete modeled read bracket.
/// # Errors
/// Preserve actual receive/charge/staging failures from the shared fixture path.
/// # Panics
/// Fail if the selected response or explicitly modeled read bracket is incoherent.
async fn ordinary_candidate_observing(
    fixture: &mut IoFixture,
    query: &[u8; RECEIVE_BYTES],
    modeled_read: Option<SoftwareClock>,
) -> Check<StagedReply> {
    let mut packet = [0_u8; PACKET_BYTES];
    packet[0] = 0x24;
    packet[1] = 2;
    packet[24..32].copy_from_slice(&query[40..48]);
    packet[32..40].copy_from_slice(&1_u64.to_be_bytes());
    packet[40..48].copy_from_slice(&2_u64.to_be_bytes());
    let mut bytes = [0_u8; RECEIVE_BYTES];
    let (length, source) = tokio::time::timeout(Duration::from_secs(2), async {
        assert_eq!(
            fixture.peers[0]
                .send_to(&packet, fixture.slots[0].local)
                .await?,
            PACKET_BYTES
        );
        fixture.slots[0].socket.recv_from(&mut bytes).await
    })
    .await??;
    let read = modeled_read.unwrap_or_else(SoftwareClock::capture);
    assert!(
        read.mono <= read.mono_after,
        "fixture read bracket is inverted"
    );
    let selected =
        crate::classify_read_result(NativeDatagramPlatform::current(), Ok((length, source)));
    assert!(crate::charge_selected_datagram(
        &mut fixture.slots[0],
        &selected
    ));
    crate::stage_reply(
        &fixture.slots[0],
        &bytes[..length],
        source,
        ApplicationClockCapture::software_only(read),
        ntp_wire::FrameLimits::new(PACKET_BYTES, 0).map_err(|_| "fixture frame limit refused")?,
    )
    .map_err(|_| "genuine managed response did not stage".into())
}

/// Reject mismatched policy proposals while preserving the healthy actual request.
/// # Errors
/// Preserve runtime, socket, timeout, staging, policy and record failures.
/// # Panics
/// Fail if a proposal is accepted, closes healthy work or changes policy facts.
#[test]
fn foreign_or_unmanaged_policy_proposal_cannot_bypass_or_close_healthy_managed_pending() -> Check<()>
{
    runtime()?.block_on(async {
        for unmanaged in [true, false] {
            let mut fixture = io_fixture()?;
            let packet = query(&mut fixture).await?;
            let mut candidate = ordinary_candidate(&mut fixture, &packet).await?;
            assert!(Arc::ptr_eq(&fixture.slots[0].identity, &candidate.identity));
            let substituted = if unmanaged {
                LivePolicy::from_mode(SourceMode::UnmanagedLab)
                    .stage(&fixture.slots[0].identity, &candidate.header)?
            } else {
                let foreign = isolated_authority(SourcePolicyLimits::new(
                    2,
                    2,
                    Duration::from_secs(30),
                    Duration::from_secs(30),
                )?)?;
                let binding = foreign.source(0)?.bind(fixture.peers[0].local_addr()?)?;
                let StagedKind::Managed { stamp, .. } = &candidate.policy.kind else {
                    return Err("genuine managed candidate lost its policy stamp".into());
                };
                // All request/key/epoch/time facts agree; only the actual cell
                // is foreign. Obtain that stamp through its genuine kernel.
                let request = fixture_send(&binding, &fixture.slots[0].identity, stamp.sent_at)?;
                StagedPolicy {
                    kind: StagedKind::Managed {
                        stamp: request.stamp,
                        control: None,
                    },
                }
            };
            let genuine = std::mem::replace(&mut candidate.policy, substituted);
            let retained = facts(&fixture.binding)?;
            crate::commit_reply(
                &mut fixture.slots[0],
                &candidate,
                Instant::now(),
                fixture.deadline,
            );
            assert!(matches!(fixture.slots[0].phase, Phase::Pending(_)));
            assert_eq!(
                (fixture.slots[0].received, fixture.slots[0].rejected),
                (1, 1)
            );
            assert_eq!(
                fixture.slots[0].last_rejection,
                Some(Rejection::SourcePolicy(SourcePolicyError::Foreign))
            );
            assert_eq!(facts(&fixture.binding)?, retained);
            candidate.policy = genuine;
            crate::commit_reply(
                &mut fixture.slots[0],
                &candidate,
                Instant::now(),
                fixture.deadline,
            );
            assert_eq!(
                fixture.slots[0].terminal().map(TerminalOutcome::status),
                Some(Status::Header)
            );
            assert_eq!(
                (fixture.slots[0].received, fixture.slots[0].rejected),
                (1, 1)
            );
            assert_eq!(facts(&fixture.binding)?, retained);
        }
        Ok(())
    })
}

/// Observe the actual setup retry leaf and then admit the same healthy binding.
/// # Errors
/// Preserve root/binding, lock, policy and healthy request failures.
/// # Panics
/// Fail if the retry skips contention or fails to become ready after release.
#[test]
fn setup_contention_release_hint_allows_fresh_healthy_admission() -> Check<()> {
    use std::{
        future::Future,
        task::{Context, Poll, Waker},
    };

    let authority = isolated_authority(limits(1, 1)?)?;
    let binding = authority.source(0)?.bind(endpoint(41001))?;
    let mode = SourceMode::Managed(binding.clone());
    let retained = facts(&binding)?;
    let guard = binding.source.cell.lock()?;
    let mut retry = Box::pin(mode.retry_after_contention());
    let mut context = Context::from_waker(Waker::noop());
    assert!(matches!(retry.as_mut().poll(&mut context), Poll::Pending));
    // This has actually reached Contended and enabled the release observer.
    // Releasing before the next poll must preserve the hint, not fabricate an
    // authorization: the receiving setup still re-admits below.
    drop(guard);
    runtime()?.block_on(async {
        tokio::time::timeout(Duration::from_secs(2), retry).await??;
        mode.admit()?;
        Ok::<(), Box<dyn Error>>(())
    })?;
    assert_eq!(facts(&binding)?, retained);
    let mut healthy = fixture_send(&binding, &Arc::new(()), Instant::now())?;
    fixture_receive(&binding, &mut healthy)?;
    Ok(())
}

/// Bound the public setup wait by its original deadline or explicit stop.
/// # Errors
/// Preserve socket, root/binding, preparation, schedule, execution and record failures.
/// # Panics
/// Fail if waiting invents reports/packets, misreports its end or changes policy.
#[test]
fn held_setup_contention_expires_or_stops_without_inventing_a_socket_record() -> Check<()> {
    for stop in [false, true] {
        let peer = std::net::UdpSocket::bind(endpoint(0))?;
        peer.set_nonblocking(true)?;
        let authority = isolated_authority(limits(1, 1)?)?;
        let source = authority.source(0)?;
        let binding = source.bind(peer.local_addr()?)?;
        let config = SeriesConfig::new(
            peer.local_addr()?,
            Duration::from_millis(50),
            Duration::from_secs(1),
        )?;
        let mut session = source.session(config)?;
        let (prepared, _control_owner) = session.prepare()?;
        let retained = facts(&binding)?;
        let guard = binding.source.cell.lock()?;
        let schedule = crate::ControlSchedule::new(stop.then_some(Duration::from_millis(5)), None)?;
        let report = prepared.execute(&schedule)?;
        assert!(
            report
                .series()
                .attempts()
                .iter()
                .all(|attempt| attempt.report().is_none())
        );
        if stop {
            assert_eq!(report.series().end(), crate::SeriesEnd::Stopped);
            assert!(report.series().primary_failure().is_none());
        } else {
            assert_eq!(report.series().end(), crate::SeriesEnd::Failure);
            assert_eq!(
                report
                    .series()
                    .primary_failure()
                    .map(|failure| failure.code),
                Some("source_policy_admission_deadline")
            );
        }
        let mut bytes = [0_u8; RECEIVE_BYTES];
        assert!(
            matches!(peer.recv_from(&mut bytes), Err(error) if error.kind() == io::ErrorKind::WouldBlock)
        );
        drop(guard);
        assert_eq!(facts(&binding)?, retained);
        assert!(SourceMode::Managed(binding).admit().is_ok());
    }
    Ok(())
}

/// Compute remaining fixture time from one original deadline; never renew a wait.
/// # Errors
/// Refuse an elapsed fixture deadline rather than granting another allowance.
fn setup_fixture_remaining(deadline: Instant) -> io::Result<Duration> {
    deadline
        .checked_duration_since(Instant::now())
        .filter(|remaining| !remaining.is_zero())
        .ok_or_else(|| io::Error::other("setup fixture deadline elapsed"))
}

/// Arm the existing one-shot observation before permitting the next retry hint.
/// The probe mutex is released before this helper returns to its notifier.
/// # Errors
/// Preserve a poisoned fixture-probe failure without acknowledging a policy release.
fn arm_setup_pulse(cell: &PolicyCell, sender: &SyncSender<()>) -> io::Result<()> {
    *cell
        .setup_contention_probe
        .lock()
        .map_err(|_| io::Error::other("fixture contention probe is poisoned"))? =
        Some(sender.clone());
    Ok(())
}

/// Hold actual policy custody while supplying at most 64 explicitly stale hints.
/// Only real Contended pulses are counted; the completion channel never adds one.
/// # Errors
/// Preserve actual custody, probe, channel and original fixture-deadline failures.
/// A failed observation still waits for caller completion within the same bound.
fn observe_setup_hint_budget(
    cell: &PolicyCell,
    pulse_sender: &SyncSender<()>,
    pulse_receiver: &Receiver<()>,
    ready: &SyncSender<()>,
    completed: &Receiver<()>,
    deadline: Instant,
) -> io::Result<(usize, bool)> {
    let guard = cell
        .lock()
        .map_err(|_| io::Error::other("fixture could not acquire actual policy custody"))?;
    let mut caller_completed = false;
    let observed = (|| -> io::Result<usize> {
        arm_setup_pulse(cell, pulse_sender)?;
        ready
            .send(())
            .map_err(|_| io::Error::other("fixture readiness receiver closed"))?;
        let mut pulses = 0;
        while pulses < 64 {
            match completed.try_recv() {
                Ok(()) => {
                    caller_completed = true;
                    return Ok(pulses);
                }
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {
                    return Err(io::Error::other("fixture completion sender closed"));
                }
            }
            // These ticks inspect explicit completion only. Time supplies no
            // ordering or pulse oracle, and every tick shares the original end.
            let remaining = setup_fixture_remaining(deadline)?.min(Duration::from_millis(10));
            match pulse_receiver.recv_timeout(remaining) {
                Ok(()) => pulses += 1,
                Err(RecvTimeoutError::Timeout) => continue,
                Err(RecvTimeoutError::Disconnected) => {
                    return Err(io::Error::other("fixture pulse sender closed"));
                }
            }
            // Re-arm even after pulse 64 so an improper 65th wait is observable.
            // This helper drops the probe mutex before the stale wake is sent.
            arm_setup_pulse(cell, pulse_sender)?;
            cell.released.notify_waiters();
        }
        Ok(pulses)
    })();
    let completion = (|| -> io::Result<()> {
        if !caller_completed {
            completed
                .recv_timeout(setup_fixture_remaining(deadline)?)
                .map_err(|_| io::Error::other("caller completion was not acknowledged"))?;
        }
        Ok(())
    })();
    let extra = match pulse_receiver.try_recv() {
        Ok(()) => Ok(true),
        Err(TryRecvError::Empty) => Ok(false),
        Err(TryRecvError::Disconnected) => Err(io::Error::other("fixture pulse channel closed")),
    };
    // Every fallible observation above retains custody through caller completion
    // or the same finite failure bound. An error never becomes a healthy release.
    drop(guard);
    completion?;
    Ok((observed?, extra?))
}

/// Exhaust the public setup hint allowance without releasing actual policy custody.
/// # Errors
/// Preserve checked setup, retained policy, thread, channel and execution failures
/// after joining the owned worker; no timeout becomes a successful observation.
/// # Panics
/// Fail if 64 actual pulses admit a 65th wait, lose the original budget cause,
/// invent a socket/send, change retained policy facts or prevent healthy cleanup.
#[test]
fn public_managed_setup_exhausts_64_stale_hints_without_a_send_or_refund() -> Check<()> {
    use std::{
        panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
        sync::mpsc::sync_channel,
        thread,
    };

    let peer = std::net::UdpSocket::bind(endpoint(0))?;
    peer.set_nonblocking(true)?;
    let destination = peer.local_addr()?;
    let authority = isolated_authority(limits(1, 1)?)?;
    let source = authority.source(0)?;
    let binding = source.bind(destination)?;
    let now = Instant::now();
    let mut prior = fixture_send(&binding, &Arc::new(()), now)?;
    fixture_control(&binding, &mut prior, RATE, now)?;
    let retained = facts(&binding)?;
    let mut session = source.session(SeriesConfig::new(
        destination,
        Duration::from_secs(10),
        Duration::from_secs(20),
    )?)?;
    // All checked preparation precedes the worker's continuous actual custody.
    let (prepared, _control_owner) = session.prepare()?;
    let schedule = crate::ControlSchedule::new(None, None)?;
    let deadline = Instant::now()
        .checked_add(Duration::from_secs(25))
        .ok_or("fixture deadline overflow")?;
    let (pulse_sender, pulse_receiver) = sync_channel(1);
    let (ready_sender, ready_receiver) = sync_channel(1);
    let (completed_sender, completed_receiver) = sync_channel(1);
    let cell = Arc::clone(&binding.source.cell);
    let worker = thread::Builder::new()
        .name("source-policy-stale-setup-hints".into())
        .spawn(move || {
            observe_setup_hint_budget(
                &cell,
                &pulse_sender,
                &pulse_receiver,
                &ready_sender,
                &completed_receiver,
                deadline,
            )
        })?;
    let execution = catch_unwind(AssertUnwindSafe(|| {
        ready_receiver
            .recv_timeout(setup_fixture_remaining(deadline)?)
            .map_err(|_| io::Error::other("actual policy holder did not become ready"))?;
        prepared
            .execute(&schedule)
            .map_err(|error| -> Box<dyn Error> { Box::new(error) })
    }));
    let completion_signal = completed_sender.send(());
    let worker_result = worker.join();
    // No assertion, propagated error or resumed panic precedes the owned join.
    let execution = match execution {
        Ok(result) => result,
        Err(payload) => resume_unwind(payload),
    };
    let report = execution?;
    completion_signal
        .map_err(|_| io::Error::other("policy worker ended before caller completion"))?;
    let (pulses, extra) =
        worker_result.map_err(|_| io::Error::other("policy hint worker panicked"))??;
    assert_eq!(pulses, 64);
    assert!(
        !extra,
        "a 65th setup wait reached the real contention probe"
    );
    assert_eq!(report.series().end(), crate::SeriesEnd::Failure);
    assert_eq!(
        report
            .series()
            .primary_failure()
            .map(|failure| failure.code),
        Some("source_policy_admission_budget")
    );
    assert_eq!(report.series().attempts().len(), 3);
    assert!(
        report
            .series()
            .attempts()
            .iter()
            .all(|attempt| attempt.report().is_none())
    );
    assert!(report.series().stop_cleanup_failure().is_none());
    assert!(report.series().invalidate_cleanup_failure().is_none());
    let mut bytes = [0_u8; RECEIVE_BYTES];
    assert!(
        matches!(peer.recv_from(&mut bytes), Err(error) if error.kind() == io::ErrorKind::WouldBlock)
    );
    assert_eq!(facts(&binding)?, retained);
    assert!(SourceMode::Managed(binding).admit().is_ok());
    Ok(())
}

/// Answer exactly three genuine queries with independent literal header facts.
/// The supplied socket already has the fixture's finite read/write timeouts.
/// # Errors
/// Preserve native I/O errors and reject invalid queries or incomplete response sends.
fn serve_three_public_queries(
    peer: std::net::UdpSocket,
) -> io::Result<(Vec<[u8; PACKET_BYTES]>, std::net::UdpSocket)> {
    let mut queries = Vec::with_capacity(3);
    for _ in 0..3 {
        let mut bytes = [0_u8; RECEIVE_BYTES];
        let (length, client) = peer.recv_from(&mut bytes)?;
        if length != PACKET_BYTES || bytes[0] != 0x23 || bytes[40..48] == [0_u8; 8] {
            return Err(io::Error::other("server received an invalid actual query"));
        }
        let mut query = [0_u8; PACKET_BYTES];
        query.copy_from_slice(&bytes[..PACKET_BYTES]);
        let mut response = [0_u8; PACKET_BYTES];
        response[0] = 0x24;
        response[1] = 2;
        response[24..32].copy_from_slice(&query[40..48]);
        response[32..40].copy_from_slice(&1_u64.to_be_bytes());
        response[40..48].copy_from_slice(&2_u64.to_be_bytes());
        if peer.send_to(&response, client)? != PACKET_BYTES {
            return Err(io::Error::other(
                "server did not send its complete literal header",
            ));
        }
        queries.push(query);
    }
    Ok((queries, peer))
}

/// Observe three distinct actual correlation tokens after every worker has joined.
/// # Panics
/// Fail if the peer observed the wrong query count or any reused token.
fn assert_three_distinct_queries(queries: &[[u8; PACKET_BYTES]]) {
    assert_eq!(queries.len(), 3);
    for first in 0..3 {
        for second in first + 1..3 {
            assert!(
                queries[first][40..48] != queries[second][40..48],
                "actual query correlation tokens were reused"
            );
        }
    }
}

/// Inspect the actual joined driver result before a peer error can hide a
/// failed continuation. Error context retains only bounded kind/OS facts.
/// # Errors
/// Preserve an absent actual slot report instead of inventing completion.
/// # Panics
/// Fail if the real series lacks three completed Header facts or clean cleanup.
fn assert_three_public_headers(
    report: &AcquisitionReport,
    peer_error: Option<&io::Error>,
) -> Check<()> {
    let peer_context = peer_error.map(|error| (error.kind(), error.raw_os_error()));
    assert_eq!(
        report.series().end(),
        crate::SeriesEnd::Completed,
        "actual series did not complete; joined peer error={peer_context:?}"
    );
    assert!(report.series().primary_failure().is_none());
    assert!(report.series().stop_cleanup_failure().is_none());
    assert!(report.series().invalidate_cleanup_failure().is_none());
    for attempt in report.series().attempts() {
        let record = attempt
            .report()
            .ok_or("actual query lacks its slot report")?;
        assert_eq!(
            record.status(),
            Status::Header,
            "actual query did not commit a Header; joined peer error={peer_context:?}"
        );
        assert!(record.sent());
        assert_eq!(record.received(), 1);
        assert_eq!(record.stratum(), Some(2));
        assert!(record.facts().is_some());
    }
    Ok(())
}

/// The public path must survive the actual pre-socket contention branch and
/// continue with genuine socket completions, rather than merely its retry leaf.
/// # Errors
/// Preserve socket, preparation, thread, timeout and execution failures after joining workers.
/// # Panics
/// Fail if actual contention, distinct queries, completed reports or no-extra-send facts differ.
#[test]
fn public_managed_series_continues_after_observed_setup_contention_release() -> Check<()> {
    use std::{
        panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
        sync::mpsc::sync_channel,
        thread,
    };

    const FIXTURE_WAIT: Duration = Duration::from_secs(5);
    drop(runtime()?);
    let peer = std::net::UdpSocket::bind(endpoint(0))?;
    peer.set_read_timeout(Some(FIXTURE_WAIT))?;
    peer.set_write_timeout(Some(FIXTURE_WAIT))?;
    let destination = peer.local_addr()?;
    let authority = isolated_authority(SourcePolicyLimits::new(
        1,
        1,
        Duration::from_millis(1),
        Duration::from_millis(4),
    )?)?;
    let source = authority.source(0)?;
    let binding = source.bind(destination)?;
    let config = SeriesConfig::new(destination, Duration::from_secs(1), Duration::from_secs(3))?;
    let mut session = source.session(config)?;
    // Preparation and its actual entropy admission precede the worker's lock.
    let (prepared, _control_owner) = session.prepare()?;
    let schedule = crate::ControlSchedule::new(None, None)?;
    let (pulse_sender, pulse_receiver) = sync_channel(1);
    *binding
        .source
        .cell
        .setup_contention_probe
        .lock()
        .map_err(|_| io::Error::other("fixture contention probe is poisoned"))? =
        Some(pulse_sender);
    let (ready_sender, ready_receiver) = sync_channel(1);

    let server = thread::Builder::new()
        .name("source-policy-loopback-replies".into())
        .spawn(move || serve_three_public_queries(peer))?;
    let cell = Arc::clone(&binding.source.cell);
    let worker = match thread::Builder::new()
        .name("source-policy-held-setup".into())
        .spawn(move || -> io::Result<bool> {
            let guard = cell.lock().map_err(|_| {
                io::Error::other("fixture could not acquire the actual policy lock")
            })?;
            ready_sender
                .send(())
                .map_err(|_| io::Error::other("fixture lock readiness receiver closed"))?;
            // Only the one-shot pulse inside the real Contended branch can
            // release this lock. No sleep substitutes for observing that path.
            let observed = pulse_receiver.recv_timeout(FIXTURE_WAIT);
            drop(guard);
            observed
                .map(|()| true)
                .map_err(|_| io::Error::other("actual setup contention pulse was not observed"))
        }) {
        Ok(worker) => worker,
        Err(error) => {
            // The server is the only already-owned thread on this setup error.
            let _server_cleanup = server.join();
            return Err(error.into());
        }
    };

    // Catch an application unwind only to join both owned threads first; it is
    // resumed afterward and never becomes a successful fixture observation.
    let execution = catch_unwind(AssertUnwindSafe(|| {
        ready_receiver
            .recv_timeout(FIXTURE_WAIT)
            .map_err(|_| io::Error::other("actual lock holder did not become ready"))?;
        prepared
            .execute(&schedule)
            .map_err(|error| -> Box<dyn Error> { Box::new(error) })
    }));
    let worker_result = worker.join();
    let server_result = server.join();
    // All joins precede every assertion, error propagation and resumed panic.
    let execution = match execution {
        Ok(result) => result,
        Err(payload) => resume_unwind(payload),
    };
    let worker_result = worker_result.map_err(|_| io::Error::other("policy lock worker panicked"));
    let server_result =
        server_result.map_err(|_| io::Error::other("loopback server worker panicked"));
    let report = execution?;
    let contention_observed = worker_result??;
    assert!(contention_observed);
    let peer_error = match &server_result {
        Err(error) | Ok(Err(error)) => Some(error),
        Ok(Ok(_)) => None,
    };
    assert_three_public_headers(&report, peer_error)?;
    let (queries, peer) = server_result??;
    assert_three_distinct_queries(&queries);
    peer.set_nonblocking(true)?;
    let mut extra = [0_u8; RECEIVE_BYTES];
    assert!(
        matches!(peer.recv_from(&mut extra), Err(error) if error.kind() == io::ErrorKind::WouldBlock)
    );
    Ok(())
}

/// Test-owned wake observer; it neither mutates policy nor supplies a release.
#[derive(Default)]
struct PolicyReleaseWakeCounter {
    /// Actual calls to the registered Waker's wake method.
    wakes: AtomicUsize,
}

impl std::task::Wake for PolicyReleaseWakeCounter {
    /// Count only an actual notification of this registered observer.
    fn wake(self: Arc<Self>) {
        let _previous = self.wakes.fetch_add(1, Ordering::SeqCst);
    }
}

/// Disarming actual managed custody cancels the old registered notification.
/// # Errors
/// Preserve checked root/binding construction and actual same-cell lock failures.
/// # Panics
/// Fail if a genuine registered neighbor is not woken, a cancelled Waker is
/// retained/notified, or any observer reference remains after local custody ends.
#[test]
fn managed_release_disarm_cancels_registered_waker_without_a_spurious_wake() -> Check<()> {
    use std::{
        future::Future,
        task::{Context, Poll, Waker},
    };

    let authority = isolated_authority(limits(1, 1)?)?;
    let binding = authority.source(0)?.bind(endpoint(41001))?;
    let mut policy = LivePolicy::from_mode(SourceMode::Managed(binding));
    let LiveMode::Managed(live) = &mut policy.mode else {
        return Err("checked managed construction lost its live owner".into());
    };
    let cell = Arc::clone(&live.binding.source.cell);
    let counter = Arc::new(PolicyReleaseWakeCounter::default());
    {
        let waker = Waker::from(Arc::clone(&counter));
        let baseline_refs = Arc::strong_count(&counter);
        let mut context = Context::from_waker(&waker);
        // The real un-cancelled registration is a healthy neighbor first.
        live.arm_release();
        assert!(matches!(
            live.released.as_mut().poll(&mut context),
            Poll::Pending
        ));
        assert!(Arc::strong_count(&counter) > baseline_refs);
        let guard = cell.lock()?;
        drop(guard);
        assert!(counter.wakes.load(Ordering::SeqCst) >= 1);
        assert!(matches!(
            live.released.as_mut().poll(&mut context),
            Poll::Ready(())
        ));
        assert_eq!(Arc::strong_count(&counter), baseline_refs);

        // Reset only the observer count; actual notification custody is issued
        // by the same production arm/disarm and PolicyGuard Drop operations.
        counter.wakes.store(0, Ordering::SeqCst);
        live.arm_release();
        assert!(matches!(
            live.released.as_mut().poll(&mut context),
            Poll::Pending
        ));
        assert!(Arc::strong_count(&counter) > baseline_refs);
        let guard = cell.lock()?;
        live.disarm_release();
        let refs_after_disarm = Arc::strong_count(&counter);
        drop(guard);
        // Assertions follow real lock release even for a no-op disarm fault.
        assert_eq!(refs_after_disarm, baseline_refs);
        assert_eq!(counter.wakes.load(Ordering::SeqCst), 0);
    }
    assert_eq!(Arc::strong_count(&counter), 1);
    Ok(())
}

/// Reject the empty public selection before attempting actual policy custody.
/// # Errors
/// Preserve actual checked bounds, root, target and record-observation failures.
/// # Panics
/// Fail if empty input becomes contention/success or changes any retained fact.
#[test]
fn public_reenable_empty_selection_refuses_before_actual_policy_lock() -> Check<()> {
    let mut authority = isolated_authority(limits(1, 1)?)?;
    let binding = authority.source(0)?.bind(endpoint(41201))?;
    let before = facts(&binding)?;
    let cell = Arc::clone(&authority.cell);
    let held = cell.lock()?;
    assert_eq!(
        authority.reenable(&[]),
        Err(SourcePolicyError::InvalidSelection)
    );
    drop(held);
    assert_eq!(facts(&binding)?, before);
    // The actual same authority remains usable after rejection.
    authority.reenable(&[authority.endpoint_target(endpoint(41201))?])?;
    let after = facts(&binding)?;
    assert_eq!(after[0], before[0]);
    let mut expected = before[1];
    expected.3 = expected
        .3
        .checked_add(1)
        .ok_or("fixture revision overflow")?;
    expected.4 = expected.4.checked_add(1).ok_or("fixture epoch overflow")?;
    assert_eq!(after[1], expected);
    Ok(())
}

/// Reject 97 genuine targets before lock acquisition or duplicate analysis.
/// # Errors
/// Preserve checked root, target and complete record-observation failures.
/// # Panics
/// Fail if oversized input reports contention/duplicate/success or changes facts.
#[test]
fn public_reenable_97_genuine_target_clones_refuses_before_actual_policy_lock() -> Check<()> {
    let mut authority = isolated_authority(limits(1, 1)?)?;
    let source = authority.source(0)?;
    let binding = source.bind(endpoint(41202))?;
    let before = facts(&binding)?;
    let target = authority.source_target(&source)?;
    let oversized = vec![target.clone(); 97];
    assert_eq!(oversized.len(), 97);
    let cell = Arc::clone(&authority.cell);
    let held = cell.lock()?;
    assert_eq!(
        authority.reenable(&oversized),
        Err(SourcePolicyError::InvalidSelection)
    );
    drop(held);
    assert_eq!(facts(&binding)?, before);
    // Removing contention must not substitute the later duplicate diagnostic.
    assert_eq!(
        authority.reenable(&oversized),
        Err(SourcePolicyError::InvalidSelection)
    );
    assert_eq!(facts(&binding)?, before);
    authority.reenable(std::slice::from_ref(&target))?;
    let after = facts(&binding)?;
    let mut expected = before[0];
    expected.3 = expected
        .3
        .checked_add(1)
        .ok_or("fixture revision overflow")?;
    expected.4 = expected.4.checked_add(1).ok_or("fixture epoch overflow")?;
    assert_eq!(after[0], expected);
    assert_eq!(after[1], before[1]);
    Ok(())
}

/// Admit exactly 32 genuine sources plus all 64 independently bound endpoints.
/// # Errors
/// Preserve construction, actual reservation/control, targets and observation errors.
/// # Panics
/// Fail if the full legal selection refuses, misses/duplicates an update, resets
/// RATE/floors, keeps refusal, or changes sticky availability.
#[test]
fn public_reenable_96_distinct_current_targets_preserves_all_record_facts() -> Check<()> {
    let mut authority = isolated_authority(limits(32, 64)?)?;
    let mut bindings = Vec::with_capacity(64);
    let mut targets = Vec::with_capacity(96);
    let now = Instant::now();
    for index in 0..32 {
        let source = authority.source(index)?;
        let port = 42000_u16
            .checked_add(
                u16::try_from(index)?
                    .checked_mul(2)
                    .ok_or("fixture port overflow")?,
            )
            .ok_or("fixture port overflow")?;
        let first = source.bind(endpoint(port))?;
        let second_port = port.checked_add(1).ok_or("fixture port overflow")?;
        let second = source.bind(endpoint(second_port))?;
        // Real production reservation/control kernels create different own
        // intervals and floors. No private field write supplies the oracle.
        let mut rate = fixture_send(&first, &Arc::new(()), now)?;
        fixture_control(&first, &mut rate, RATE, now)?;
        let due = facts(&first)?[0].1.ok_or("RATE floor absent")?;
        let mut denial = fixture_send(&second, &Arc::new(()), due)?;
        fixture_control(&second, &mut denial, DENY, due)?;
        targets.push(authority.source_target(&source)?);
        targets.push(authority.endpoint_target(endpoint(port))?);
        targets.push(authority.endpoint_target(endpoint(second_port))?);
        bindings.push(first);
        bindings.push(second);
    }
    assert_eq!(targets.len(), 96);
    assert_eq!(bindings.len(), 64);
    let before = bindings.iter().map(facts).collect::<Result<Vec<_>, _>>()?;
    let (pairs, remainder) = before.as_chunks::<2>();
    let expected_remainder: &[[RecordFacts; 2]] = &[];
    assert_eq!(remainder, expected_remainder);
    for [first, second] in pairs {
        assert_eq!(first[0], second[0]);
        assert!(first[0].2 && !first[1].2 && second[1].2);
        assert_eq!(first[0].0, Duration::from_secs(2));
        assert_eq!(first[1].0, Duration::from_secs(2));
        assert_eq!(second[1].0, Duration::from_secs(1));
        assert!(
            first
                .iter()
                .chain(second)
                .all(|record| record.1.is_some() && !record.5)
        );
    }
    authority.reenable(&targets)?;
    for (binding, previous) in bindings.iter().zip(before) {
        let after = facts(binding)?;
        for (actual, mut expected) in after.into_iter().zip(previous) {
            expected.2 = false;
            expected.3 = expected
                .3
                .checked_add(1)
                .ok_or("fixture revision overflow")?;
            expected.4 = expected.4.checked_add(1).ok_or("fixture epoch overflow")?;
            assert_eq!(actual, expected);
        }
    }
    Ok(())
}

/// A real panic while holding policy custody poisons subsequent public preparation.
/// # Errors
/// Preserve checked construction, socket setup, healthy preparation and observation errors.
/// # Panics
/// Fail if a poisoned owner prepares, loses its exact refusal, changes any record,
/// emits a datagram, clears poison, or prevents the independent healthy owner.
#[test]
fn actual_poisoned_policy_refuses_public_session_prepare_without_udp_or_state_change() -> Check<()>
{
    use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

    // Both are isolated real allocator/cells. Never install/reset PROCESS_POLICY:
    // its one real-installation test remains the only process-global writer.
    let poisoned_peer = std::net::UdpSocket::bind(endpoint(0))?;
    let healthy_peer = std::net::UdpSocket::bind(endpoint(0))?;
    poisoned_peer.set_nonblocking(true)?;
    healthy_peer.set_nonblocking(true)?;
    let poisoned_authority = isolated_authority(limits(1, 1)?)?;
    let poisoned_source = poisoned_authority.source(0)?;
    let poisoned_binding = poisoned_source.bind(poisoned_peer.local_addr()?)?;
    let mut poisoned_session = poisoned_source.session(SeriesConfig::new(
        poisoned_peer.local_addr()?,
        Duration::from_millis(100),
        Duration::from_secs(1),
    )?)?;
    let healthy_authority = isolated_authority(limits(1, 1)?)?;
    let healthy_source = healthy_authority.source(0)?;
    let healthy_binding = healthy_source.bind(healthy_peer.local_addr()?)?;
    let mut healthy_session = healthy_source.session(SeriesConfig::new(
        healthy_peer.local_addr()?,
        Duration::from_millis(100),
        Duration::from_secs(1),
    )?)?;
    let poisoned_before = facts(&poisoned_binding)?;
    let healthy_before = facts(&healthy_binding)?;
    let cell = Arc::clone(&poisoned_authority.cell);
    let held = cell.lock()?;
    // resume_unwind deliberately starts the real unwind without a panic-hook
    // side effect; the actual MutexGuard is dropped while the thread unwinds.
    let unwind: Result<(), _> = catch_unwind(AssertUnwindSafe(move || {
        let _held_during_unwind = held;
        resume_unwind(Box::new("owned source-policy poisoning specimen"));
    }));
    assert!(unwind.is_err());
    assert!(cell.state.is_poisoned());
    for _ in 0..2 {
        assert!(matches!(
            poisoned_session.prepare(),
            Err(crate::AcquisitionError::SourcePolicy(
                SourcePolicyError::Poisoned
            ))
        ));
    }
    // Read the real poisoned state only for inert observation. Never clear its
    // poison bit, recover it into authority, or write any field.
    let after = match cell.state.try_lock() {
        Err(TryLockError::Poisoned(error)) => {
            let state = error.get_ref();
            [
                state.record(RecordKey::Source(poisoned_source.index))?,
                state.record(RecordKey::Endpoint(poisoned_binding.endpoint))?,
            ]
            .map(|record| {
                (
                    record.interval,
                    record.floor,
                    record.refused,
                    record.revision,
                    record.epoch,
                    record.unavailable,
                )
            })
        }
        Ok(_) | Err(TryLockError::WouldBlock) => {
            return Err("actual policy poison was not retained".into());
        }
    };
    assert_eq!(after, poisoned_before);
    assert!(cell.state.is_poisoned());
    // Real legal public preparation and unused-plan drop still work in another
    // root. A second successful preparation proves the first released capacity.
    for _ in 0..2 {
        let (prepared, control) = healthy_session.prepare()?;
        drop(prepared);
        drop(control);
        assert_eq!(facts(&healthy_binding)?, healthy_before);
    }
    for peer in [&poisoned_peer, &healthy_peer] {
        let mut bytes = [0_u8; RECEIVE_BYTES];
        assert!(matches!(
            peer.recv_from(&mut bytes),
            Err(error) if error.kind() == io::ErrorKind::WouldBlock
        ));
    }
    Ok(())
}

/// One independently mismatched identity at the actual receiving kernel.
#[derive(Clone, Copy, Debug)]
enum ReceivingIdentityMismatch {
    /// A genuine stamp from another actual root, with matching catalogue positions.
    Owner,
    /// Another registered source sharing the exact endpoint in the same root.
    Source,
    /// Another endpoint under the same registered source and root.
    Endpoint,
    /// Another actual allocation supplied as the consumer's request identity.
    Request,
}

/// Reject genuine but unrelated request contexts without spending their history.
/// # Errors
/// Preserve checked construction, binding, reservation and actual receiving failures.
/// # Panics
/// Fail if the intended one-hot mismatch is absent, refusal changes policy or
/// consumes the genuine request, or its matching owner cannot receive it once.
#[test]
fn receiving_rejects_each_genuine_identity_mismatch_without_spending() -> Check<()> {
    for mismatch in [
        ReceivingIdentityMismatch::Owner,
        ReceivingIdentityMismatch::Source,
        ReceivingIdentityMismatch::Endpoint,
        ReceivingIdentityMismatch::Request,
    ] {
        let authority = isolated_authority(limits(2, 2)?)?;
        let source = authority.source(0)?;
        let receiver = source.bind(endpoint(41001))?;
        let foreign = isolated_authority(limits(2, 2)?)?;
        let producer = match mismatch {
            ReceivingIdentityMismatch::Owner => foreign.source(0)?.bind(endpoint(41001))?,
            ReceivingIdentityMismatch::Source => authority.source(1)?.bind(endpoint(41001))?,
            ReceivingIdentityMismatch::Endpoint => source.bind(endpoint(41002))?,
            ReceivingIdentityMismatch::Request => receiver.clone(),
        };
        let request = Arc::new(());
        let other_request = Arc::new(());
        let mut pending = fixture_send(&producer, &request, Instant::now())?;
        let actual_request = match mismatch {
            ReceivingIdentityMismatch::Request => &other_request,
            _ => &request,
        };
        let recipient_before = facts(&receiver)?;
        let producer_before = facts(&producer)?;
        // Both records remain available and their epochs agree. Only the
        // independently selected identity below may cause the refusal.
        SourceMode::Managed(receiver.clone()).admit()?;
        assert_eq!(pending.stamp.source_epoch, recipient_before[0].4);
        assert_eq!(pending.stamp.endpoint_epoch, recipient_before[1].4);
        let observed = [
            !Arc::ptr_eq(&receiver.source.cell, &pending.stamp.cell),
            receiver.source.index != pending.stamp.source,
            receiver.endpoint != pending.stamp.endpoint,
            !Arc::ptr_eq(actual_request, &pending.stamp.request),
        ];
        let expected = match mismatch {
            ReceivingIdentityMismatch::Owner => [true, false, false, false],
            ReceivingIdentityMismatch::Source => [false, true, false, false],
            ReceivingIdentityMismatch::Endpoint => [false, false, true, false],
            ReceivingIdentityMismatch::Request => [false, false, false, true],
        };
        assert_eq!(
            observed, expected,
            "receiving mismatch fixture selected the wrong role"
        );
        let refusal = match mismatch {
            ReceivingIdentityMismatch::Request => {
                // The convenience fixture always supplies its own identity;
                // reach the actual production receiver to challenge that input.
                let guard = receiver.source.cell.lock()?;
                guard
                    .state()?
                    .receiving(&receiver, &pending.stamp, actual_request)
            }
            _ => fixture_receive(&receiver, &mut pending),
        };
        assert!(matches!(refusal, Err(SourcePolicyError::Foreign)));
        assert!(!pending.consumed);
        assert_eq!(facts(&receiver)?, recipient_before);
        assert_eq!(facts(&producer)?, producer_before);
        // The same genuine request is still usable by its original owner.
        fixture_receive(&producer, &mut pending)?;
        assert!(pending.consumed);
        assert!(matches!(
            fixture_receive(&producer, &mut pending),
            Err(SourcePolicyError::Stale)
        ));
        assert_eq!(facts(&receiver)?, recipient_before);
        assert_eq!(facts(&producer)?, producer_before);
    }
    Ok(())
}

/// Select exactly one actual public re-enable target, never a raw record index.
#[derive(Clone, Copy, Debug)]
enum ReceivingEpochSide {
    /// Invalidate only the logical source's old outstanding epoch.
    Source,
    /// Invalidate only the canonical endpoint's old outstanding epoch.
    Endpoint,
}

/// Reject an old request if either individual receiving epoch changes.
/// # Errors
/// Preserve checked construction, binding, reservation, public re-enable,
/// checked expected-successor arithmetic and genuine receiving failures.
/// # Panics
/// Fail if re-enable changes other facts, one stale epoch is admitted, rejected
/// work is consumed or changes policy, or a new matching request is refused.
#[test]
fn receiving_rejects_either_individually_reenabled_epoch_without_spending() -> Check<()> {
    for side in [ReceivingEpochSide::Source, ReceivingEpochSide::Endpoint] {
        let mut authority = isolated_authority(limits(1, 1)?)?;
        let source = authority.source(0)?;
        let binding = source.bind(endpoint(41001))?;
        let mut old = fixture_send(&binding, &Arc::new(()), Instant::now())?;
        let before = facts(&binding)?;
        // Public re-enable is valid for the exact current revision, even when
        // the record is available. It still invalidates its old correlations.
        let (target, selected) = match side {
            ReceivingEpochSide::Source => (authority.source_target(&source)?, 0),
            ReceivingEpochSide::Endpoint => (authority.endpoint_target(endpoint(41001))?, 1),
        };
        authority.reenable(std::slice::from_ref(&target))?;
        let retained = facts(&binding)?;
        let mut expected = before;
        expected[selected].3 = before[selected]
            .3
            .checked_add(1)
            .ok_or("fixture revision overflow")?;
        expected[selected].4 = before[selected]
            .4
            .checked_add(1)
            .ok_or("fixture epoch overflow")?;
        assert_eq!(retained, expected);
        SourceMode::Managed(binding.clone()).admit()?;
        assert!(matches!(
            fixture_receive(&binding, &mut old),
            Err(SourcePolicyError::Stale)
        ));
        assert!(!old.consumed);
        assert_eq!(facts(&binding)?, retained);
        // A stale control cannot acquire a second effect path either.
        let old_sent_at = old.stamp.sent_at;
        assert!(matches!(
            fixture_control(&binding, &mut old, RATE, old_sent_at),
            Err(SourcePolicyError::Stale)
        ));
        assert!(!old.consumed);
        assert_eq!(facts(&binding)?, retained);
        let due = retained[0]
            .1
            .max(retained[1].1)
            .ok_or("retained send floor absent")?;
        let mut healthy = fixture_send(&binding, &Arc::new(()), due)?;
        fixture_receive(&binding, &mut healthy)?;
        assert!(healthy.consumed);
        assert!(matches!(
            fixture_receive(&binding, &mut healthy),
            Err(SourcePolicyError::Stale)
        ));
    }
    Ok(())
}

/// Observe one actual managed owner's contention, then its concrete release.
/// The owned policy cell is derived from the slot itself, not supplied separately.
/// # Errors
/// Preserve absent managed ownership, actual policy custody and refresh failures.
fn observe_live_contention_episode(slot: &mut LiveSlot) -> Check<(bool, bool)> {
    use std::{
        future::Future,
        task::{Context, Poll, Waker},
    };

    let cell = match &slot.policy.mode {
        LiveMode::Managed(live) => Arc::clone(&live.binding.source.cell),
        LiveMode::UnmanagedLab => return Err("fixture has no actual managed live owner".into()),
    };
    let guard = cell.lock()?;
    slot.policy.refresh(&slot.identity)?;
    let mut context = Context::from_waker(Waker::noop());
    let mut wake = std::pin::pin!(slot.policy.wake());
    let pending_while_held = matches!(wake.as_mut().poll(&mut context), Poll::Pending);
    drop(guard);
    // A completed async future must never be polled again. Preserve a premature
    // completion as an explicit failed observation after actual guard cleanup.
    let ready_after_release =
        pending_while_held && matches!(wake.as_mut().poll(&mut context), Poll::Ready(()));
    Ok((pending_while_held, ready_after_release))
}

/// Reset actual live notification custody for a second distinct contention episode.
/// # Errors
/// Preserve runtime, checked fixture, native I/O, timeout and actual policy failures.
/// # Panics
/// Fail if a held transaction produces readiness, real release loses its wake,
/// policy or send history changes before admission, or healthy completion differs.
#[test]
fn repeated_actual_live_contention_waits_for_each_release_before_a_healthy_send() -> Check<()> {
    runtime()?.block_on(async {
        tokio::time::timeout(Duration::from_secs(2), async {
            let mut fixture = io_fixture()?;
            let retained = facts(&fixture.binding)?;
            for _episode in 0..2 {
                let observed = observe_live_contention_episode(&mut fixture.slots[0])?;
                assert_eq!(observed, (true, true));
                assert!(matches!(fixture.slots[0].phase, Phase::Sending));
                assert!(!fixture.slots[0].sent);
                assert_eq!(fixture.slots[0].send_attempts, 0);
                assert_eq!(facts(&fixture.binding)?, retained);
                let mut bytes = [0_u8; RECEIVE_BYTES];
                assert!(matches!(fixture.peers[0].try_recv_from(&mut bytes),
                    Err(error) if error.kind() == io::ErrorKind::WouldBlock));
                // Do not perform a successful live refresh between episodes:
                // that would let disarm_release mask a missing arm reset.
            }
            let actual_query = query(&mut fixture).await?;
            assert_eq!(fixture.slots[0].send_attempts, 1);
            reply(&mut fixture, &actual_query, 2, 0).await?;
            assert_eq!(
                fixture.slots[0].terminal().map(TerminalOutcome::status),
                Some(Status::Header)
            );
            assert!(
                fixture.slots[0]
                    .terminal()
                    .and_then(TerminalOutcome::facts)
                    .is_some()
            );
            assert_eq!(fixture.slots[0].received, 1);
            Ok::<(), Box<dyn Error>>(())
        })
        .await?
    })
}

/// A floor appearing after readiness keeps the real concrete sender waiting.
/// # Errors
/// Preserve runtime, fixture, actual readiness/I/O, policy reservation and timeout failures.
/// # Panics
/// Fail if a stale readiness hint sends, retires with `NotDue`, charges work or
/// changes a retained floor, or the independent healthy actual send is unaccounted.
#[test]
fn actual_send_rechecks_a_late_future_floor_without_retirement_or_an_attempt() -> Check<()> {
    runtime()?.block_on(async {
        tokio::time::timeout(Duration::from_secs(2), async {
            let mut fixture = io_fixture()?;
            let original_deadline = fixture.deadline;
            {
                let slot = &mut fixture.slots[0];
                slot.policy.refresh(&slot.identity)?;
            }
            assert!(fixture.slots[0].can_send());
            fixture.slots[0].socket.writable().await?;
            // This is a genuine separate reservation through the production
            // kernel AFTER readiness; no native-send fact is asserted for it.
            let mut competing = fixture_send(&fixture.binding, &Arc::new(()), Instant::now())?;
            let retained = facts(&fixture.binding)?;
            assert!(
                retained
                    .iter()
                    .all(|record| record.1.is_some_and(|floor| floor > Instant::now()))
            );
            assert!(fixture.slots[0].can_send());
            // Deliberately preserve the stale cached Ready hint: a refresh here
            // would mask the actual final sender's required fresh floor guard.
            crate::attempt_send(&mut fixture.slots[0], original_deadline, &fixture.control);
            assert!(matches!(fixture.slots[0].phase, Phase::Sending));
            assert!(fixture.slots[0].terminal().is_none());
            assert!(!fixture.slots[0].sent);
            assert_eq!(fixture.slots[0].send_attempts, 0);
            assert_eq!(
                (fixture.slots[0].received, fixture.slots[0].rejected),
                (0, 0)
            );
            assert!(!fixture.slots[0].policy.wants_writable());
            assert_eq!(fixture.deadline, original_deadline);
            assert_eq!(facts(&fixture.binding)?, retained);
            let mut bytes = [0_u8; RECEIVE_BYTES];
            assert!(matches!(fixture.peers[0].try_recv_from(&mut bytes),
                Err(error) if error.kind() == io::ErrorKind::WouldBlock));
            fixture_receive(&fixture.binding, &mut competing)?;
            assert_eq!(facts(&fixture.binding)?, retained);
            // A separate genuine managed owner is the actual-send neighbor;
            // never wait out, shorten or refund the original future floor.
            let mut healthy = io_fixture()?;
            let actual_query = query(&mut healthy).await?;
            assert_eq!(healthy.slots[0].send_attempts, 1);
            reply(&mut healthy, &actual_query, 2, 0).await?;
            assert_eq!(
                healthy.slots[0].terminal().map(TerminalOutcome::status),
                Some(Status::Header)
            );
            assert_eq!(healthy.slots[0].received, 1);
            Ok::<(), Box<dyn Error>>(())
        })
        .await?
    })
}

/// Seed nonzero budgets through three actual selected UNKNOWN UDP replies, then
/// stage the fourth actual datagram without manually writing either counter.
/// # Errors
/// Preserve real query/reply, timeout, staging and policy failures.
/// # Panics
/// Fail if actual admission does not retain Pending with exactly (4, 3).
async fn seeded_final_commit_candidate(fixture: &mut IoFixture) -> Check<StagedReply> {
    let packet = query(fixture).await?;
    for _ in 0..3 {
        reply(fixture, &packet, 0, UNKNOWN).await?;
    }
    assert_eq!(
        (fixture.slots[0].received, fixture.slots[0].rejected),
        (3, 3)
    );
    assert!(matches!(fixture.slots[0].phase, Phase::Pending(_)));
    let candidate = ordinary_candidate(fixture, &packet).await?;
    assert_eq!(
        (fixture.slots[0].received, fixture.slots[0].rejected),
        (4, 3)
    );
    Ok(candidate)
}

/// Enter the actual final leaf while retaining the invocation's real watch Ref.
/// Deliberately omit the parent's earlier deadline/read-time prechecks: they
/// otherwise mask the independent final leaf's current-owner obligations.
fn final_leaf_with_control(fixture: &mut IoFixture, candidate: &StagedReply) -> ManagedEffect {
    let current_control = fixture.control.borrow();
    let effect = commit_managed_reply(&mut fixture.slots[0], candidate, fixture.deadline);
    drop(current_control);
    effect
}

/// One changed field in an otherwise genuine well-typed staged proposal.
/// These are proposal transfers, not whole genuine foreign native histories.
#[derive(Clone, Copy, Debug)]
enum FinalStampTransfer {
    /// Transfer only a genuinely foreign root allocation.
    Cell,
    /// Transfer only another real request allocation.
    Request,
    /// Transfer only another checked source index.
    Source,
    /// Transfer only another checked endpoint position.
    Endpoint,
    /// Transfer only a genuine neighbor source's newer epoch.
    SourceEpoch,
    /// Transfer only a genuine neighbor endpoint's newer epoch.
    EndpointEpoch,
    /// Transfer only a different checked reservation coordinate.
    SentAt,
}

/// Transfer one selected field and independently require the exact mismatch mask.
/// # Panics
/// Fail if the generated proposal changes a different role or more than one role.
fn final_stamp_transfer_proposal(
    field: FinalStampTransfer,
    genuine: &Correlation,
    alternate: &Correlation,
    foreign: &Correlation,
) -> Correlation {
    let mut proposal = genuine.clone();
    let expected = match field {
        FinalStampTransfer::Cell => {
            proposal.cell = Arc::clone(&foreign.cell);
            [true, false, false, false, false, false, false]
        }
        FinalStampTransfer::Request => {
            proposal.request = Arc::clone(&alternate.request);
            [false, true, false, false, false, false, false]
        }
        FinalStampTransfer::Source => {
            proposal.source = alternate.source;
            [false, false, true, false, false, false, false]
        }
        FinalStampTransfer::Endpoint => {
            proposal.endpoint = alternate.endpoint;
            [false, false, false, true, false, false, false]
        }
        FinalStampTransfer::SourceEpoch => {
            proposal.source_epoch = alternate.source_epoch;
            [false, false, false, false, true, false, false]
        }
        FinalStampTransfer::EndpointEpoch => {
            proposal.endpoint_epoch = alternate.endpoint_epoch;
            [false, false, false, false, false, true, false]
        }
        FinalStampTransfer::SentAt => {
            proposal.sent_at = alternate.sent_at;
            [false, false, false, false, false, false, true]
        }
    };
    assert_eq!(
        [
            !Arc::ptr_eq(&proposal.cell, &genuine.cell),
            !Arc::ptr_eq(&proposal.request, &genuine.request),
            proposal.source != genuine.source,
            proposal.endpoint != genuine.endpoint,
            proposal.source_epoch != genuine.source_epoch,
            proposal.endpoint_epoch != genuine.endpoint_epoch,
            proposal.sent_at != genuine.sent_at,
        ],
        expected,
        "wrong one-hot final proposal: {field:?}"
    );
    proposal
}

/// Challenge every final-correlation equality with only its one field changed.
/// # Errors
/// Preserve real fixture, checked target, kernel reservation and staging failures.
/// # Panics
/// Fail if the proposal is not one-hot, is accepted, closes Pending, refunds
/// accounting, mutates policy/deadline or prevents the restored genuine commit.
async fn final_stamp_transfer_case(field: FinalStampTransfer) -> Check<()> {
    let mut fixture = io_fixture()?;
    let original_deadline = fixture.deadline;
    let mut candidate = seeded_final_commit_candidate(&mut fixture).await?;
    let StagedKind::Managed {
        stamp: genuine,
        control: None,
    } = &candidate.policy.kind
    else {
        return Err("ordinary genuine candidate lost its managed stamp".into());
    };
    let genuine = genuine.clone();
    let neighbor_source = fixture.authority.source(1)?;
    let neighbor = neighbor_source.bind(fixture.peers[1].local_addr()?)?;
    let source_target = fixture.authority.source_target(&neighbor_source)?;
    let endpoint_target = fixture
        .authority
        .endpoint_target(fixture.peers[1].local_addr()?)?;
    fixture
        .authority
        .reenable(&[source_target, endpoint_target])?;
    let later = genuine
        .sent_at
        .checked_add(Duration::from_secs(1))
        .ok_or("fixture alternate send coordinate overflow")?;
    // Alternate primitive roles/epochs/time come from genuine checked neighbor
    // ownership and the real debit kernel. Only one field is transferred below.
    let alternate = fixture_send(&neighbor, &fixture.slots[1].identity, later)?.stamp;
    let foreign = isolated_authority(limits(1, 1)?)?;
    let foreign_binding = foreign.source(0)?.bind(fixture.peers[0].local_addr()?)?;
    let foreign_request = fixture_send(&foreign_binding, &genuine.request, genuine.sent_at)?;
    let proposal =
        final_stamp_transfer_proposal(field, &genuine, &alternate, &foreign_request.stamp);
    let before = [
        facts(&fixture.binding)?,
        facts(&neighbor)?,
        facts(&foreign_binding)?,
    ];
    candidate.policy = StagedPolicy {
        kind: StagedKind::Managed {
            stamp: proposal,
            control: None,
        },
    };
    assert!(!fixture.control.borrow().stop);
    assert!(matches!(
        final_leaf_with_control(&mut fixture, &candidate),
        ManagedEffect::Handled
    ));
    assert!(matches!(fixture.slots[0].phase, Phase::Pending(_)));
    assert!(
        fixture.slots[0]
            .terminal()
            .and_then(TerminalOutcome::facts)
            .is_none(),
        "a foreign proposal cannot publish historical source provenance"
    );
    assert_eq!(
        (fixture.slots[0].received, fixture.slots[0].rejected),
        (4, 4)
    );
    assert_eq!(
        fixture.slots[0].last_rejection,
        Some(Rejection::SourcePolicy(SourcePolicyError::Foreign))
    );
    assert_eq!(fixture.deadline, original_deadline);
    assert_eq!(
        [
            facts(&fixture.binding)?,
            facts(&neighbor)?,
            facts(&foreign_binding)?
        ],
        before
    );
    candidate.policy = StagedPolicy {
        kind: StagedKind::Managed {
            stamp: genuine,
            control: None,
        },
    };
    assert_final_genuine_commit_once(&mut fixture, &candidate)?;
    assert_eq!(
        [
            facts(&fixture.binding)?,
            facts(&neighbor)?,
            facts(&foreign_binding)?
        ],
        before
    );
    Ok(())
}

/// Consume the restored real candidate once; a repeated leaf cannot rewrite it.
/// # Errors
/// Reject a missing ordinary terminal fact rather than manufacturing completion.
/// # Panics
/// Fail if completion changes charged accounting or a repeated leaf republishes.
fn assert_final_genuine_commit_once(fixture: &mut IoFixture, candidate: &StagedReply) -> Check<()> {
    let expected_source = fixture.authority.source(0)?.identity();
    assert!(matches!(
        final_leaf_with_control(fixture, candidate),
        ManagedEffect::Handled
    ));
    assert_eq!(
        fixture.slots[0].terminal().map(TerminalOutcome::status),
        Some(Status::Header)
    );
    assert_eq!(
        (fixture.slots[0].received, fixture.slots[0].rejected),
        (4, 4)
    );
    let committed = fixture.slots[0]
        .terminal()
        .and_then(TerminalOutcome::facts)
        .ok_or("genuine final commit has no ordinary facts")?;
    let processing = committed.processing;
    let committed_source = committed
        .source_identity()
        .ok_or("genuine managed final commit has no logical source")?
        .clone();
    assert!(committed_source.same_source(&expected_source));
    assert!(matches!(
        final_leaf_with_control(fixture, candidate),
        ManagedEffect::Handled
    ));
    let saved = fixture.slots[0]
        .terminal()
        .and_then(TerminalOutcome::facts)
        .ok_or("repeated final leaf lost historical facts")?;
    assert_eq!(saved.processing, processing);
    assert!(
        saved
            .source_identity()
            .is_some_and(|source| source.same_source(&committed_source))
    );
    assert_eq!(
        (fixture.slots[0].received, fixture.slots[0].rejected),
        (4, 4)
    );
    Ok(())
}

/// Exercise the actual final consuming owner, not only the earlier kernel check.
/// # Errors
/// Preserve runtime, bounded scenario, query, proposal and healthy commit failures.
/// # Panics
/// Fail if any individually transferred field bypasses the intended Foreign guard.
#[test]
fn final_commit_rejects_each_one_hot_policy_stamp_transfer_without_refund() -> Check<()> {
    runtime()?.block_on(async {
        for field in [
            FinalStampTransfer::Cell,
            FinalStampTransfer::Request,
            FinalStampTransfer::Source,
            FinalStampTransfer::Endpoint,
            FinalStampTransfer::SourceEpoch,
            FinalStampTransfer::EndpointEpoch,
            FinalStampTransfer::SentAt,
        ] {
            tokio::time::timeout(Duration::from_secs(2), final_stamp_transfer_case(field))
                .await??;
        }
        Ok(())
    })
}

/// Independently selected final-leaf guard; the parent cannot supply its failure.
#[derive(Clone, Copy, Debug)]
enum FinalLeafGuard {
    /// Hold the actual policy guard through the leaf admission.
    Contention,
    /// Wait past the original cutoff admitted before any request work.
    OriginalDeadline,
    /// Present an explicitly invalid well-typed future read coordinate.
    FutureReadProposal,
}

/// Preserve charged work on final-leaf refusal and preserve healthy continuation.
/// # Errors
/// Preserve real setup, bounded UDP/staging, custody and deadline arithmetic errors.
/// # Panics
/// Fail if a leaf refunds the seeded counters, refreshes expiry, mutates policy
/// or commits an invalid proposal; all assertions follow actual guard release.
async fn final_leaf_guard_case(guard: FinalLeafGuard) -> Check<()> {
    let mut fixture = io_fixture()?;
    if matches!(guard, FinalLeafGuard::OriginalDeadline) {
        // This becomes the ORIGINAL request cutoff before actual query/staging.
        fixture.deadline = Instant::now()
            .checked_add(Duration::from_millis(500))
            .ok_or("fixture original deadline overflow")?;
    }
    let original_deadline = fixture.deadline;
    let mut candidate = seeded_final_commit_candidate(&mut fixture).await?;
    let genuine_read = candidate.read;
    let before = facts(&fixture.binding)?;
    assert!(!fixture.control.borrow().stop);
    assert!(candidate.read.software().mono_after < original_deadline);
    assert!(
        Instant::now() < original_deadline,
        "fixture did not stage before its original expiry"
    );
    let effect = match guard {
        FinalLeafGuard::Contention => {
            let cell = Arc::clone(&fixture.binding.source.cell);
            let held = cell.lock()?;
            let effect = final_leaf_with_control(&mut fixture, &candidate);
            drop(held);
            effect
        }
        FinalLeafGuard::OriginalDeadline => {
            tokio::time::sleep_until(original_deadline.into()).await;
            assert!(Instant::now() >= original_deadline);
            final_leaf_with_control(&mut fixture, &candidate)
        }
        FinalLeafGuard::FutureReadProposal => {
            // Deliberately invalid well-typed proposal, not a native observation.
            candidate.read = ApplicationClockCapture::software_only(SoftwareClock {
                mono_after: Instant::now()
                    .checked_add(Duration::from_secs(30))
                    .ok_or("fixture future read coordinate overflow")?,
                ..candidate.read.software()
            });
            final_leaf_with_control(&mut fixture, &candidate)
        }
    };
    assert!(matches!(effect, ManagedEffect::Handled));
    assert_eq!(
        (fixture.slots[0].received, fixture.slots[0].rejected),
        (4, 4)
    );
    assert_eq!(
        fixture.slots[0].last_rejection,
        Some(Rejection::UnhandledControl)
    );
    assert_eq!(facts(&fixture.binding)?, before);
    assert_eq!(fixture.deadline, original_deadline);
    match guard {
        FinalLeafGuard::Contention | FinalLeafGuard::FutureReadProposal => {
            assert!(matches!(fixture.slots[0].phase, Phase::Pending(_)));
            candidate.read = genuine_read;
            assert_final_genuine_commit_once(&mut fixture, &candidate)?;
            assert_eq!(facts(&fixture.binding)?, before);
        }
        FinalLeafGuard::OriginalDeadline => {
            assert_eq!(
                fixture.slots[0].terminal().map(TerminalOutcome::status),
                Some(Status::Timeout)
            );
            assert!(
                fixture.slots[0]
                    .terminal()
                    .and_then(TerminalOutcome::facts)
                    .is_none()
            );
            // A separate genuine owner remains usable; it gets its own original
            // deadline, never a replacement cutoff for the expired request.
            let mut healthy = io_fixture()?;
            let healthy_candidate = seeded_final_commit_candidate(&mut healthy).await?;
            let healthy_before = facts(&healthy.binding)?;
            assert!(matches!(
                final_leaf_with_control(&mut healthy, &healthy_candidate),
                ManagedEffect::Handled
            ));
            assert_eq!(
                healthy.slots[0].terminal().map(TerminalOutcome::status),
                Some(Status::Header)
            );
            assert_eq!(
                (healthy.slots[0].received, healthy.slots[0].rejected),
                (4, 3)
            );
            assert_eq!(facts(&healthy.binding)?, healthy_before);
        }
    }
    Ok(())
}

/// Leaf contention, original expiry and future-read proposals spend rejection work.
/// # Errors
/// Preserve runtime and each independently bounded two-second scenario's failure.
/// # Panics
/// Fail if any guard invents completion/refunds work or loses its valid neighbor.
#[test]
fn final_commit_leaf_guards_preserve_nonzero_receive_accounting_and_original_expiry() -> Check<()> {
    runtime()?.block_on(async {
        for guard in [
            FinalLeafGuard::Contention,
            FinalLeafGuard::OriginalDeadline,
            FinalLeafGuard::FutureReadProposal,
        ] {
            tokio::time::timeout(Duration::from_secs(2), final_leaf_guard_case(guard)).await??;
        }
        Ok(())
    })
}

/// One actual current-policy failure introduced after native reply selection.
#[derive(Clone, Copy, Debug)]
enum FinalPolicyFailure {
    /// Unwind while the actual policy mutex guard retains custody.
    Poisoned,
    /// Advance the actual source's current revision and correlation epoch.
    SourceReenabled,
    /// Advance the actual endpoint's current revision and correlation epoch.
    EndpointReenabled,
}

/// Commit a genuine fresh neighboring reply with no invented rejection.
/// # Errors
/// Preserve real setup, query, selected receive, staging and policy failures.
/// # Panics
/// Fail if an unchanged current policy rejects work or publishes no header facts.
async fn final_policy_failure_healthy_neighbor() -> Check<()> {
    let mut healthy = io_fixture()?;
    let original_deadline = healthy.deadline;
    let packet = query(&mut healthy).await?;
    let candidate = ordinary_candidate(&mut healthy, &packet).await?;
    let retained = facts(&healthy.binding)?;
    assert_eq!(
        (healthy.slots[0].received, healthy.slots[0].rejected),
        (1, 0)
    );
    assert!(matches!(
        final_leaf_with_control(&mut healthy, &candidate),
        ManagedEffect::Handled
    ));
    assert_eq!(
        healthy.slots[0].terminal().map(TerminalOutcome::status),
        Some(Status::Header)
    );
    assert!(
        healthy.slots[0]
            .terminal()
            .and_then(TerminalOutcome::facts)
            .is_some()
    );
    assert_eq!(
        (healthy.slots[0].received, healthy.slots[0].rejected),
        (1, 0)
    );
    assert_eq!(facts(&healthy.binding)?, retained);
    assert_eq!(healthy.deadline, original_deadline);
    Ok(())
}

/// Introduce the selected failure through this actual fixture's policy owner.
/// # Errors
/// Preserve actual policy custody, checked target and re-enable failures.
/// # Panics
/// Fail if unwinding the actual held guard does not retain policy poisoning.
fn introduce_final_policy_failure(
    fixture: &mut IoFixture,
    failure: FinalPolicyFailure,
) -> Check<SourcePolicyError> {
    use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

    let expected_error = match failure {
        FinalPolicyFailure::Poisoned => {
            let cell = Arc::clone(&fixture.binding.source.cell);
            let held = cell.lock()?;
            // This is the real owner guard unwinding, not a forged poison flag.
            let unwind: Result<(), _> = catch_unwind(AssertUnwindSafe(move || {
                let _held_during_unwind = held;
                resume_unwind(Box::new("selected-reply policy poisoning specimen"));
            }));
            assert!(unwind.is_err());
            assert!(cell.state.is_poisoned());
            SourcePolicyError::Poisoned
        }
        FinalPolicyFailure::SourceReenabled => {
            let source = fixture.authority.source(0)?;
            let target = fixture.authority.source_target(&source)?;
            fixture.authority.reenable(&[target])?;
            SourcePolicyError::Stale
        }
        FinalPolicyFailure::EndpointReenabled => {
            let target = fixture
                .authority
                .endpoint_target(fixture.peers[0].local_addr()?)?;
            fixture.authority.reenable(&[target])?;
            SourcePolicyError::Stale
        }
    };
    Ok(expected_error)
}

/// Observe the actual retained records without recovering poisoned authority.
/// # Errors
/// Preserve checked record failures or refuse lost actual poison custody.
fn final_policy_failure_retained_facts(
    fixture: &IoFixture,
    failure: FinalPolicyFailure,
) -> Check<[RecordFacts; 2]> {
    if !matches!(failure, FinalPolicyFailure::Poisoned) {
        return Ok(facts(&fixture.binding)?);
    }
    // Read-only observation of the poisoned state: never clear or recover it
    // into live authority, and never write the records or their counters.
    match fixture.binding.source.cell.state.try_lock() {
        Err(TryLockError::Poisoned(error)) => {
            let state = error.get_ref();
            Ok([
                state.record(RecordKey::Source(fixture.binding.source.index))?,
                state.record(RecordKey::Endpoint(fixture.binding.endpoint))?,
            ]
            .map(|record| {
                (
                    record.interval,
                    record.floor,
                    record.refused,
                    record.revision,
                    record.epoch,
                    record.unavailable,
                )
            }))
        }
        Ok(_) | Err(TryLockError::WouldBlock) => {
            Err("final policy refusal lost actual poison custody".into())
        }
    }
}

/// Refuse one selected native reply after a real final policy-state change.
/// Nonzero history comes from three real UNKNOWN datagrams, never counter writes.
/// # Errors
/// Preserve actual UDP/charge/staging, checked re-enable and record observations.
/// # Panics
/// Fail if the exact final refusal loses one reject, refunds prior work, changes
/// policy custody, publishes header facts or renews the original deadline.
async fn final_policy_failure_case(
    failure: FinalPolicyFailure,
    with_rejected_history: bool,
) -> Check<()> {
    let mut fixture = io_fixture()?;
    let original_deadline = fixture.deadline;
    let candidate = if with_rejected_history {
        seeded_final_commit_candidate(&mut fixture).await?
    } else {
        let packet = query(&mut fixture).await?;
        ordinary_candidate(&mut fixture, &packet).await?
    };
    let expected_before = if with_rejected_history {
        (4, 3)
    } else {
        (1, 0)
    };
    assert_eq!(
        (fixture.slots[0].received, fixture.slots[0].rejected),
        expected_before
    );
    {
        let slot = &mut fixture.slots[0];
        slot.policy.refresh(&slot.identity)?;
    }
    assert!(matches!(fixture.slots[0].phase, Phase::Pending(_)));
    assert!(candidate.read.software().mono_after < original_deadline);
    let last_rejection = fixture.slots[0].last_rejection;
    let before = facts(&fixture.binding)?;
    let expected_error = introduce_final_policy_failure(&mut fixture, failure)?;
    let retained = if matches!(failure, FinalPolicyFailure::Poisoned) {
        before
    } else {
        facts(&fixture.binding)?
    };
    assert!(matches!(
        final_leaf_with_control(&mut fixture, &candidate),
        ManagedEffect::Handled
    ));
    assert!(matches!(
        fixture.slots[0].terminal(),
        Some(TerminalOutcome::SourcePolicy(error)) if *error == expected_error
    ));
    assert!(
        fixture.slots[0]
            .terminal()
            .and_then(TerminalOutcome::facts)
            .is_none()
    );
    let expected_after = if with_rejected_history {
        (4, 4)
    } else {
        (1, 1)
    };
    assert_eq!(
        (fixture.slots[0].received, fixture.slots[0].rejected),
        expected_after,
        "the genuine selected native reply must add exactly one final-policy rejection"
    );
    for _ in 0..2 {
        crate::refresh_source_policy(&mut fixture.slots[0]);
        assert!(matches!(
            fixture.slots[0].terminal(),
            Some(TerminalOutcome::SourcePolicy(error)) if *error == expected_error
        ));
        assert_eq!(
            (fixture.slots[0].received, fixture.slots[0].rejected),
            expected_after
        );
        assert_eq!(fixture.slots[0].last_rejection, last_rejection);
        assert!(fixture.slots[0].sent);
        assert_eq!(fixture.slots[0].send_attempts, 1);
        assert_eq!(fixture.deadline, original_deadline);
    }
    let after = final_policy_failure_retained_facts(&fixture, failure)?;
    assert_eq!(after, retained);
    let mut extra = [0_u8; RECEIVE_BYTES];
    for peer in &fixture.peers {
        assert!(matches!(
            peer.try_recv_from(&mut extra),
            Err(error) if error.kind() == io::ErrorKind::WouldBlock
        ));
    }
    Ok(())
}

/// The final non-Contended lock error rejects its genuinely selected reply once.
/// # Errors
/// Preserve runtime and each finite actual-receive/poisoning scenario's failure.
/// # Panics
/// Fail if poison loses the exact reason, selected rejection or valid neighbor.
#[test]
fn final_commit_poisoned_policy_counts_the_selected_reply_once() -> Check<()> {
    runtime()?.block_on(async {
        final_policy_failure_healthy_neighbor().await?;
        for with_rejected_history in [false, true] {
            tokio::time::timeout(
                Duration::from_secs(2),
                final_policy_failure_case(FinalPolicyFailure::Poisoned, with_rejected_history),
            )
            .await??;
        }
        Ok(())
    })
}

/// A checked current-revision change rejects its genuinely selected old reply once.
/// # Errors
/// Preserve runtime and each finite native/checked-re-enable scenario's failure.
/// # Panics
/// Fail if either record's final Stale refusal loses work or creates header facts.
#[test]
fn final_commit_reenabled_policy_counts_the_selected_reply_once() -> Check<()> {
    runtime()?.block_on(async {
        final_policy_failure_healthy_neighbor().await?;
        for failure in [
            FinalPolicyFailure::SourceReenabled,
            FinalPolicyFailure::EndpointReenabled,
        ] {
            for with_rejected_history in [false, true] {
                tokio::time::timeout(
                    Duration::from_secs(2),
                    final_policy_failure_case(failure, with_rejected_history),
                )
                .await??;
            }
        }
        Ok(())
    })
}

/// Independently specified floor neighbors; equality is an admitted send point.
/// # Errors
/// Refuse fixture coordinate overflow instead of wrapping its modeled boundary.
fn modeled_floor_neighbors(floor: Instant) -> Check<[(Instant, bool); 3]> {
    Ok([
        (
            floor
                .checked_sub(Duration::from_nanos(1))
                .ok_or("modeled floor predecessor overflow")?,
            false,
        ),
        (floor, true),
        (
            floor
                .checked_add(Duration::from_nanos(1))
                .ok_or("modeled floor successor overflow")?,
            true,
        ),
    ])
}

/// Model complete coherent software brackets, not native timestamp evidence.
/// # Errors
/// Preserve fixture coordinate overflow before issuing any capture capability.
fn modeled_sender_captures(at: Instant) -> Check<(SoftwareClock, SoftwareClock)> {
    let after = at
        .checked_add(Duration::from_nanos(1))
        .ok_or("modeled capture overflow")?;
    Ok((
        SoftwareClock {
            mono: at,
            utc: std::time::SystemTime::UNIX_EPOCH,
            mono_after: at,
        },
        SoftwareClock {
            mono: after,
            utc: std::time::SystemTime::UNIX_EPOCH,
            mono_after: after,
        },
    ))
}

/// Exact modeled floor neighbors preserve actual records and early read custody.
/// # Errors
/// Preserve checked construction, real reservation, custody and modeled bounds errors.
/// # Panics
/// Fail if equality is refused, an early return samples time, or refresh changes debit facts.
#[test]
fn modeled_refresh_floor_neighbors_preserve_records_and_deferred_read_counts() -> Check<()> {
    let authority = isolated_authority(limits(1, 1)?)?;
    let binding = authority.source(0)?.bind(endpoint(41001))?;
    let request = Arc::new(());
    let reserved = fixture_send(&binding, &request, Instant::now())?;
    let retained = facts(&binding)?;
    let floor = retained[0].1.ok_or("actual reserved floor absent")?;
    for (at, due) in modeled_floor_neighbors(floor)? {
        let mut policy = LivePolicy::from_mode(SourceMode::Managed(binding.clone()));
        let (read, count) = observations::InstantRead::fixed(at);
        assert_eq!(count.reads(), 0);
        policy.refresh_observing(&request, read)?;
        assert_eq!(count.reads(), 1);
        assert_eq!(policy.wants_writable(), due);
        assert_eq!(facts(&binding)?, retained);
    }
    let mut unmanaged = LivePolicy::from_mode(SourceMode::UnmanagedLab);
    let (read, count) = observations::InstantRead::fixed(floor);
    unmanaged.refresh_observing(&request, read)?;
    assert_eq!(count.reads(), 0);
    let mut managed = LivePolicy::from_mode(SourceMode::Managed(binding.clone()));
    let cell = Arc::clone(&binding.source.cell);
    let held = cell.lock()?;
    let (read, count) = observations::InstantRead::fixed(floor);
    let result = managed.refresh_observing(&request, read);
    drop(held);
    result?;
    assert_eq!(count.reads(), 0);
    assert!(!managed.wants_writable());
    assert!(!reserved.consumed);
    assert_eq!(facts(&binding)?, retained);
    Ok(())
}

/// Check retained complete modeled brackets and the genuine concrete-send stamp.
/// # Errors
/// Preserve actual missing Pending/managed ownership and checked floor arithmetic errors.
/// # Panics
/// Fail if either capture/correlation/debit changes or pending refresh reads unused time.
fn assert_modeled_send_custody(
    fixture: &mut IoFixture,
    before: SoftwareClock,
    after: SoftwareClock,
    retained: [RecordFacts; 2],
) -> Check<()> {
    let Phase::Pending(bracket) = &fixture.slots[0].phase else {
        return Err("modeled boundary did not retain the actual complete send".into());
    };
    assert_eq!(
        (
            bracket.before.software().mono,
            bracket.before.software().mono_after,
            bracket.before.software().utc
        ),
        (before.mono, before.mono_after, before.utc)
    );
    assert_eq!(
        (
            bracket.after.software().mono,
            bracket.after.software().mono_after,
            bracket.after.software().utc
        ),
        (after.mono, after.mono_after, after.utc)
    );
    let LiveMode::Managed(live) = &fixture.slots[0].policy.mode else {
        return Err("actual send lost its checked managed owner".into());
    };
    let stamp = live
        .pending
        .as_ref()
        .ok_or("actual send correlation absent")?;
    assert!(Arc::ptr_eq(&stamp.cell, &fixture.binding.source.cell));
    assert!(Arc::ptr_eq(&stamp.request, &fixture.slots[0].identity));
    assert_eq!(
        (stamp.source, stamp.endpoint),
        (fixture.binding.source.index, fixture.binding.endpoint)
    );
    assert_eq!(
        (stamp.source_epoch, stamp.endpoint_epoch),
        (retained[0].4, retained[1].4)
    );
    assert_eq!(stamp.sent_at, before.mono_after);
    let mut expected = retained;
    for record in &mut expected {
        record.1 = Some(
            before
                .mono_after
                .checked_add(record.0)
                .ok_or("modeled debit overflow")?,
        );
        record.3 = record.3.checked_add(1).ok_or("modeled revision overflow")?;
    }
    assert_eq!(facts(&fixture.binding)?, expected);
    let (read, count) = observations::InstantRead::fixed(after.mono_after);
    {
        let slot = &mut fixture.slots[0];
        slot.policy.refresh_observing(&slot.identity, read)?;
    }
    assert_eq!(
        count.reads(),
        0,
        "current pending admission has no floor-time read"
    );
    assert_eq!(facts(&fixture.binding)?, expected);
    Ok(())
}

/// Actual concrete send at modeled floor equality/after spends exactly one effect.
/// # Errors
/// Preserve runtime/UDP/readiness, real debit, checked modeled clocks and timeout errors.
/// # Panics
/// Fail if before-floor sends/retires/refunds, equality is refused, or actual
/// datagram/debit/correlation/bracket/read counts differ from the independent contract.
#[test]
fn modeled_send_floor_neighbors_keep_one_datagram_debit_and_coherent_captures() -> Check<()> {
    runtime()?.block_on(async {
        for boundary in 0..3 {
            let mut fixture = io_fixture_with_intervals(Duration::from_secs(1), Duration::from_secs(4))?;
            let request = Arc::new(());
            let _reserved = fixture_send(&fixture.binding, &request, Instant::now())?;
            let retained = facts(&fixture.binding)?;
            let floor = retained[0].1.ok_or("actual modeled-send floor absent")?;
            let (at, due) = modeled_floor_neighbors(floor)?[boundary];
            let (before, after) = modeled_sender_captures(at)?;
            assert!(after.mono_after < fixture.deadline);
            let (captures, count) = observations::SendReads::fixed(before, after);
            assert_eq!(count.reads(), 0);
            fixture.slots[0].socket.writable().await?;
            let control = fixture.control.borrow();
            let effect = attempt_managed_send_observing(&mut fixture.slots[0], fixture.deadline, captures);
            drop(control);
            assert!(matches!(effect, ManagedEffect::Handled));
            let mut bytes = [0_u8; RECEIVE_BYTES];
            if due {
                assert_eq!(count.reads(), 2);
                assert!(fixture.slots[0].sent);
                assert_eq!(fixture.slots[0].send_attempts, 1);
                let (length, source) = tokio::time::timeout(Duration::from_secs(2), fixture.peers[0].recv_from(&mut bytes)).await??;
                assert_eq!(length, PACKET_BYTES);
                assert_eq!(source, fixture.slots[0].local);
                assert_eq!(&bytes[..length], &fixture.slots[0].packet);
                assert_modeled_send_custody(&mut fixture, before, after, retained)?;
            } else {
                assert_eq!(count.reads(), 1);
                assert!(matches!(fixture.slots[0].phase, Phase::Sending));
                assert!(!fixture.slots[0].sent);
                assert_eq!(fixture.slots[0].send_attempts, 0);
                assert!(!fixture.slots[0].policy.wants_writable());
                assert_eq!(facts(&fixture.binding)?, retained);
            }
            assert!(matches!(fixture.peers[0].try_recv_from(&mut bytes), Err(error) if error.kind() == io::ErrorKind::WouldBlock));
        }
        Ok(())
    })
}

/// Drive a real managed request with a reversed bracket at a known cutoff distance.
/// # Errors
/// Preserve the actual runtime, socket, policy and modeled time failures.
/// # Panics
/// Fail if a payload, post-send read, debit, floor change or wrong outcome occurs.
fn check_managed_reversed_send_capture(
    lower_before_cutoff: Duration,
    expected_status: Status,
    expected_capture: Option<crate::MeasurementRefusal>,
) -> Check<()> {
    runtime()?.block_on(async {
        let mut fixture: IoFixture =
            io_fixture_with_intervals(Duration::from_secs(1), Duration::from_secs(4))?;
        let request: Arc<()> = Arc::new(());
        let _reserved: FixtureRequest = fixture_send(&fixture.binding, &request, Instant::now())?;
        let retained: [RecordFacts; 2] = facts(&fixture.binding)?;
        let deadline: Instant = fixture.deadline;
        let lower: Instant = deadline
            .checked_sub(lower_before_cutoff)
            .ok_or("fixture cutoff predecessor unavailable")?;
        let earlier: Instant = lower
            .checked_sub(Duration::from_nanos(1))
            .ok_or("fixture reversed capture predecessor unavailable")?;
        assert!(retained.iter().all(|record: &RecordFacts| {
            record.1.is_some_and(|floor: Instant| floor < earlier)
        }));
        let before: SoftwareClock = SoftwareClock {
            mono: lower,
            utc: std::time::SystemTime::UNIX_EPOCH,
            mono_after: earlier,
        };
        let after: SoftwareClock = SoftwareClock { mono_after: lower, ..before };
        let (captures, count): (observations::SendReads, observations::ReadCount) =
            observations::SendReads::fixed_raw(before, after);
        tokio::time::timeout(Duration::from_secs(2), fixture.slots[0].socket.writable()).await??;

        let current: watch::Ref<'_, Control> = fixture.control.borrow();
        let effect: ManagedEffect =
            attempt_managed_send_observing(&mut fixture.slots[0], deadline, captures);
        assert!(!current.stop);
        assert_eq!(current.epoch, 0);
        drop(current);
        let [peer, _]: [UdpSocket; 2] = fixture.peers;
        let native_peer: std::net::UdpSocket = peer.into_std()?;
        let mut bytes: [u8; RECEIVE_BYTES] = [0; RECEIVE_BYTES];
        assert!(
            matches!(native_peer.recv_from(&mut bytes), Err(error) if error.kind() == io::ErrorKind::WouldBlock),
            "a reversed pre-send capture allowed a managed loopback packet"
        );
        assert!(matches!(effect, ManagedEffect::Handled));
        assert_eq!(count.reads(), 1);
        let [slot, _]: [LiveSlot; 2] = fixture.slots;
        assert_eq!(slot.send_attempts, 0);
        let report: SlotReport = slot.into_report(0)?;
        assert!(!report.sent());
        assert_eq!((report.received(), report.rejected()), (0, 0));
        assert_eq!(report.status(), expected_status);
        assert_eq!(report.capture_refusal(), expected_capture);
        assert!(report.io_cause().is_none() && report.facts().is_none());
        assert_eq!(facts(&fixture.binding)?, retained);
        Ok(())
    })
}

/// A reversed bracket whose lower reading reaches expiry preserves Timeout.
/// # Errors
/// Preserve the managed sender fixture's actual setup and custody failures.
#[test]
fn managed_send_cutoff_uses_both_clock_readings() -> Check<()> {
    check_managed_reversed_send_capture(Duration::ZERO, Status::Timeout, None)
}

/// An inversion wholly before expiry refuses without a managed reservation.
/// # Errors
/// Preserve the managed sender fixture's actual setup and custody failures.
#[test]
fn managed_inverted_pre_send_capture_refuses_before_reservation() -> Check<()> {
    check_managed_reversed_send_capture(
        Duration::from_nanos(1),
        Status::Io,
        Some(crate::MeasurementRefusal::CaptureOrder),
    )
}

/// One independent final-commit boundary, never a production trusted-clock input.
#[derive(Clone, Copy)]
enum ModeledCommitBoundary {
    /// A modeled final reading strictly before the genuine read return.
    BeforeRead,
    /// Exactly the genuine selected read-return upper bound.
    AtRead,
    /// One nanosecond after that upper bound.
    AfterRead,
    /// Exactly the original, unchanged request deadline.
    AtDeadline,
}

/// Invoke the real guarded owner while retaining actual invocation control custody.
fn modeled_final_leaf(
    fixture: &mut IoFixture,
    candidate: &StagedReply,
    read: observations::InstantRead,
) -> ManagedEffect {
    let control = fixture.control.borrow();
    let effect =
        commit_managed_reply_observing(&mut fixture.slots[0], candidate, fixture.deadline, read);
    drop(control);
    effect
}

/// Check a genuine staged reply against independently modeled final owner time.
/// # Errors
/// Preserve genuine query/staging, original custody, modeled coordinate and absent-fact errors.
/// # Panics
/// Fail if earlier read/expiry admits, a legal equality refuses, spent budgets
/// change, or a repeated terminal observation rewrites the committed history.
async fn modeled_commit_boundary_case(boundary: ModeledCommitBoundary) -> Check<()> {
    let mut fixture = io_fixture()?;
    let original_deadline = fixture.deadline;
    let candidate = seeded_final_commit_candidate(&mut fixture).await?;
    let read_return: Instant = candidate.read.software().mono_after;
    let at = match boundary {
        ModeledCommitBoundary::BeforeRead => read_return
            .checked_sub(Duration::from_nanos(1))
            .ok_or("modeled read predecessor overflow")?,
        ModeledCommitBoundary::AtRead => read_return,
        ModeledCommitBoundary::AfterRead => read_return
            .checked_add(Duration::from_nanos(1))
            .ok_or("modeled read successor overflow")?,
        ModeledCommitBoundary::AtDeadline => original_deadline,
    };
    let StagedKind::Managed { stamp, .. } = &candidate.policy.kind else {
        return Err("genuine modeled candidate lost its pending stamp".into());
    };
    assert!(at >= stamp.sent_at);
    assert!(read_return < original_deadline);
    let retained = facts(&fixture.binding)?;
    let (read, count) = observations::InstantRead::fixed(at);
    assert_eq!(count.reads(), 0);
    assert!(matches!(
        modeled_final_leaf(&mut fixture, &candidate, read),
        ManagedEffect::Handled
    ));
    assert_eq!(count.reads(), 1);
    assert_eq!(fixture.deadline, original_deadline);
    assert_eq!(facts(&fixture.binding)?, retained);
    if matches!(boundary, ModeledCommitBoundary::AtDeadline) {
        assert_eq!(
            fixture.slots[0].terminal().map(TerminalOutcome::status),
            Some(Status::Timeout)
        );
        assert_eq!(
            (fixture.slots[0].received, fixture.slots[0].rejected),
            (4, 4)
        );
        assert!(
            fixture.slots[0]
                .terminal()
                .and_then(TerminalOutcome::facts)
                .is_none()
        );
        return Ok(());
    }
    let (processing, expected_rejected) = if matches!(boundary, ModeledCommitBoundary::BeforeRead) {
        assert!(matches!(fixture.slots[0].phase, Phase::Pending(_)));
        assert_eq!(
            (fixture.slots[0].received, fixture.slots[0].rejected),
            (4, 4)
        );
        assert_eq!(
            fixture.slots[0].last_rejection,
            Some(Rejection::UnhandledControl)
        );
        let (read, count) = observations::InstantRead::fixed(read_return);
        assert!(matches!(
            modeled_final_leaf(&mut fixture, &candidate, read),
            ManagedEffect::Handled
        ));
        assert_eq!(count.reads(), 1);
        (read_return, 4)
    } else {
        (at, 3)
    };
    assert_modeled_committed_history(&mut fixture, &candidate, processing, 4, expected_rejected)?;
    assert_eq!(facts(&fixture.binding)?, retained);
    Ok(())
}

/// Observe the actual historical Header once and preserve it on repeated consumption.
/// # Errors
/// Preserve missing actual Header/history failures rather than inventing facts.
/// # Panics
/// Fail if genuine captures/identity/processing or spent counters change on replay.
fn assert_modeled_committed_history(
    fixture: &mut IoFixture,
    candidate: &StagedReply,
    processing: Instant,
    expected_received: u16,
    expected_rejected: u16,
) -> Check<()> {
    let terminal = fixture.slots[0]
        .terminal()
        .ok_or("modeled valid reply did not finish")?;
    assert_eq!(terminal.status(), Status::Header);
    let historical = terminal
        .facts()
        .ok_or("modeled valid reply lost genuine Header facts")?;
    assert_eq!(historical.processing, processing);
    assert_eq!(
        historical.read_return.mono_after,
        candidate.read.software().mono_after
    );
    assert!(Arc::ptr_eq(&historical.identity, &candidate.identity));
    assert_eq!(
        (fixture.slots[0].received, fixture.slots[0].rejected),
        (expected_received, expected_rejected)
    );
    let (repeat, count) = observations::InstantRead::fixed(processing);
    assert!(matches!(
        modeled_final_leaf(fixture, candidate, repeat),
        ManagedEffect::Handled
    ));
    assert_eq!(count.reads(), 1);
    assert_eq!(
        fixture.slots[0]
            .terminal()
            .and_then(TerminalOutcome::facts)
            .ok_or("repeat lost history")?
            .processing,
        processing
    );
    assert_eq!(
        (fixture.slots[0].received, fixture.slots[0].rejected),
        (expected_received, expected_rejected)
    );
    Ok(())
}

/// Admit equal modeled send/read/final coordinates through genuine UDP custody.
/// # Errors
/// Preserve actual send/receive/staging, checked time and historical owner failures.
/// # Panics
/// Fail if equality is rejected, a genuine debit/correlation changes, extra UDP
/// appears, or the later-time healthy neighbor fails through the same kernels.
async fn modeled_send_time_commit_boundary(later: bool) -> Check<()> {
    let mut fixture = io_fixture()?;
    let original_deadline = fixture.deadline;
    let at = Instant::now();
    let capture = SoftwareClock {
        mono: at,
        utc: std::time::SystemTime::UNIX_EPOCH,
        mono_after: at,
    };
    let retained_before = facts(&fixture.binding)?;
    let (captures, capture_count) = observations::SendReads::fixed(capture, capture);
    fixture.slots[0].socket.writable().await?;
    let current = fixture.control.borrow();
    let effect = attempt_managed_send_observing(&mut fixture.slots[0], original_deadline, captures);
    drop(current);
    assert!(matches!(effect, ManagedEffect::Handled));
    assert_eq!(capture_count.reads(), 2);
    assert!(fixture.slots[0].sent);
    assert_eq!(fixture.slots[0].send_attempts, 1);
    let mut query = [0_u8; RECEIVE_BYTES];
    let (length, source) = tokio::time::timeout(
        Duration::from_secs(2),
        fixture.peers[0].recv_from(&mut query),
    )
    .await??;
    assert_eq!(length, PACKET_BYTES);
    assert_eq!(source, fixture.slots[0].local);
    assert_eq!(&query[..length], &fixture.slots[0].packet);
    assert_modeled_send_custody(&mut fixture, capture, capture, retained_before)?;
    let retained = facts(&fixture.binding)?;
    let processing = if later {
        at.checked_add(Duration::from_nanos(1))
            .ok_or("modeled send-time successor overflow")?
    } else {
        at
    };
    assert!(processing < original_deadline);
    let read = SoftwareClock {
        mono: processing,
        utc: capture.utc,
        mono_after: processing,
    };
    let candidate = ordinary_candidate_observing(&mut fixture, &query, Some(read)).await?;
    let StagedKind::Managed { stamp, .. } = &candidate.policy.kind else {
        return Err("genuine equal-time candidate lost its managed stamp".into());
    };
    assert_eq!(stamp.sent_at, at);
    assert_eq!(candidate.sent.before.software().mono_after, at);
    assert_eq!(candidate.sent.after.software().mono_after, at);
    assert_eq!(candidate.read.software().mono, processing);
    assert_eq!(candidate.read.software().mono_after, processing);
    assert_eq!(
        (fixture.slots[0].received, fixture.slots[0].rejected),
        (1, 0)
    );
    let (read, count) = observations::InstantRead::fixed(processing);
    assert!(matches!(
        modeled_final_leaf(&mut fixture, &candidate, read),
        ManagedEffect::Handled
    ));
    assert_eq!(count.reads(), 1);
    assert_eq!(
        fixture.slots[0].terminal().map(TerminalOutcome::status),
        Some(Status::Header),
        "modeled stamp-time boundary must retain Header; later={later}"
    );
    assert_modeled_committed_history(&mut fixture, &candidate, processing, 1, 0)?;
    assert_eq!(facts(&fixture.binding)?, retained);
    assert_eq!(fixture.deadline, original_deadline);
    assert!(matches!(
        fixture.peers[0].try_recv_from(&mut query),
        Err(error) if error.kind() == io::ErrorKind::WouldBlock
    ));
    Ok(())
}

/// Model exact read-return and original-cutoff decisions through the real final kernel.
/// # Errors
/// Preserve runtime and independently bounded genuine UDP/owner scenario failures.
/// # Panics
/// Fail on incorrect boundary admission, refunded rejection or invented history.
#[test]
fn modeled_final_commit_read_equality_and_original_deadline_preserve_history() -> Check<()> {
    runtime()?.block_on(async {
        for boundary in [
            ModeledCommitBoundary::BeforeRead,
            ModeledCommitBoundary::AtRead,
            ModeledCommitBoundary::AfterRead,
            ModeledCommitBoundary::AtDeadline,
        ] {
            tokio::time::timeout(
                Duration::from_secs(2),
                modeled_commit_boundary_case(boundary),
            )
            .await??;
        }
        // Under a faulty stamp-time guard, the later-time neighbor must finish
        // before the exact equality assertion can diagnose its distinct failure.
        for later in [true, false] {
            tokio::time::timeout(
                Duration::from_secs(2),
                modeled_send_time_commit_boundary(later),
            )
            .await??;
        }
        Ok(())
    })
}

/// Sealed modeled scripts sample only on use and fail explicitly after two captures.
/// # Errors
/// Preserve checked modeled-coordinate construction failures.
/// # Panics
/// Fail if construction consumes a read, two coherent captures change, or a third
/// read/incoherent supplied bracket is admitted by the test-only fixture owner.
#[test]
fn modeled_observation_scripts_are_deferred_coherent_and_bounded() -> Check<()> {
    use std::panic::{AssertUnwindSafe, catch_unwind};
    let at = Instant::now();
    let (before, after) = modeled_sender_captures(at)?;
    let (single, count) = observations::InstantRead::fixed(at);
    assert_eq!(count.reads(), 0);
    assert_eq!(single.read(), at);
    assert_eq!(count.reads(), 1);
    let (mut script, count) = observations::SendReads::fixed(before, after);
    assert_eq!(count.reads(), 0);
    assert_eq!(script.capture().software().mono_after, before.mono_after);
    assert_eq!(script.capture().software().mono_after, after.mono_after);
    assert_eq!(count.reads(), 2);
    let exhausted = catch_unwind(AssertUnwindSafe(|| script.capture()));
    assert!(exhausted.is_err());
    assert_eq!(count.reads(), 2);
    let incoherent = catch_unwind(AssertUnwindSafe(|| {
        observations::SendReads::fixed(after, before)
    }));
    assert!(incoherent.is_err());
    Ok(())
}

/// Real contention refuses observations before the original guarded read points.
/// # Errors
/// Preserve runtime, actual lock, UDP/staging and healthy continuation failures.
/// # Panics
/// Fail if refused setup/commit reads a scripted clock, changes record custody,
/// refunds receive work or prevents the genuine later completion.
#[test]
fn modeled_contended_send_and_commit_leave_deferred_observations_unread() -> Check<()> {
    runtime()?.block_on(async {
        let mut fixture = io_fixture()?;
        let retained = facts(&fixture.binding)?;
        let (before, after) = modeled_sender_captures(Instant::now())?;
        let (captures, count) = observations::SendReads::fixed(before, after);
        let cell = Arc::clone(&fixture.binding.source.cell);
        let held = cell.lock()?;
        let control = fixture.control.borrow();
        let effect =
            attempt_managed_send_observing(&mut fixture.slots[0], fixture.deadline, captures);
        drop(control);
        drop(held);
        assert!(matches!(effect, ManagedEffect::Handled));
        assert_eq!(count.reads(), 0);
        assert!(!fixture.slots[0].sent);
        assert_eq!(fixture.slots[0].send_attempts, 0);
        assert!(matches!(fixture.slots[0].phase, Phase::Sending));
        assert_eq!(facts(&fixture.binding)?, retained);
        let candidate = seeded_final_commit_candidate(&mut fixture).await?;
        let retained = facts(&fixture.binding)?;
        let (read, count): (observations::InstantRead, observations::ReadCount) =
            observations::InstantRead::fixed(candidate.read.software().mono_after);
        let held = cell.lock()?;
        let effect = modeled_final_leaf(&mut fixture, &candidate, read);
        drop(held);
        assert!(matches!(effect, ManagedEffect::Handled));
        assert_eq!(count.reads(), 0);
        assert!(matches!(fixture.slots[0].phase, Phase::Pending(_)));
        assert_eq!(
            (fixture.slots[0].received, fixture.slots[0].rejected),
            (4, 4)
        );
        assert_eq!(facts(&fixture.binding)?, retained);
        assert_final_genuine_commit_once(&mut fixture, &candidate)?;
        assert_eq!(facts(&fixture.binding)?, retained);
        Ok(())
    })
}

/// Exhaust the actual round setup loop using 64 observed stale release hints.
/// # Errors
/// Preserve real policy custody, bounded worker handshakes and joined execution errors.
/// # Panics
/// Reject a 65th wait, any query/refund or fabricated terminal attempt.
#[test]
fn public_managed_round_exhausts_64_actual_setup_hints_without_a_send_or_refund() -> Check<()> {
    use std::{
        panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
        sync::mpsc::sync_channel,
        thread,
    };

    drop(runtime()?);
    let peer = std::net::UdpSocket::bind(endpoint(0))?;
    peer.set_nonblocking(true)?;
    let destination = peer.local_addr()?;
    let authority = isolated_authority(limits(1, 1)?)?;
    let source = authority.source(0)?;
    let binding = source.bind(destination)?;
    let now = Instant::now();
    let mut prior = fixture_send(&binding, &Arc::new(()), now)?;
    fixture_control(&binding, &mut prior, RATE, now)?;
    let retained = facts(&binding)?;
    let (prepared, _control_owner) =
        source.prepare_address_round(crate::AddressRoundConfig::new(
            &[destination],
            Duration::from_secs(10),
            Duration::from_secs(20),
        )?)?;
    let schedule = crate::ControlSchedule::new(None, None)?;
    let deadline = Instant::now()
        .checked_add(Duration::from_secs(25))
        .ok_or("fixture deadline overflow")?;
    let (pulse_sender, pulse_receiver) = sync_channel(1);
    let (ready_sender, ready_receiver) = sync_channel(1);
    let (completed_sender, completed_receiver) = sync_channel(1);
    let cell = Arc::clone(&binding.source.cell);
    let worker = thread::Builder::new()
        .name("source-policy-stale-setup-hints".into())
        .spawn(move || {
            observe_setup_hint_budget(
                &cell,
                &pulse_sender,
                &pulse_receiver,
                &ready_sender,
                &completed_receiver,
                deadline,
            )
        })?;
    let execution = catch_unwind(AssertUnwindSafe(|| {
        ready_receiver
            .recv_timeout(setup_fixture_remaining(deadline)?)
            .map_err(|_| io::Error::other("actual policy holder did not become ready"))?;
        crate::execute_address_round(prepared, &schedule)
            .map_err(|error| -> Box<dyn Error> { Box::new(error) })
    }));
    let completion_signal = completed_sender.send(());
    let worker_result = worker.join();
    // No assertion, propagated error or resumed panic precedes the owned join.
    let execution = match execution {
        Ok(result) => result,
        Err(payload) => resume_unwind(payload),
    };
    let report = execution?;
    completion_signal
        .map_err(|_| io::Error::other("policy worker ended before caller completion"))?;
    let (pulses, extra) =
        worker_result.map_err(|_| io::Error::other("policy hint worker panicked"))??;
    assert_eq!(pulses, 64);
    assert!(
        !extra,
        "a 65th setup wait reached the real contention probe"
    );
    assert_eq!(report.end(), crate::AddressRoundEnd::Failure);
    assert!(matches!(
        report.primary_error(),
        Some(crate::PilotError::PolicyAdmissionBudget)
    ));
    assert!(report.attempts().is_empty() && report.policy_refusals().is_empty());
    assert!(report.cleanup_errors().iter().all(Option::is_none));
    let mut bytes = [0_u8; RECEIVE_BYTES];
    assert!(
        matches!(peer.recv_from(&mut bytes), Err(error) if error.kind() == io::ErrorKind::WouldBlock)
    );
    assert_eq!(facts(&binding)?, retained);
    assert!(SourceMode::Managed(binding).admit().is_ok());
    Ok(())
}

/// Observe one query without replying; retain its bound socket in the scoped result.
/// # Errors
/// Preserve native timeout/receive failures; absent traffic cannot pass.
/// # Panics
/// Reject a malformed query or missing opaque origin token.
fn managed_three_blackhole(peer: std::net::UdpSocket) -> io::Result<std::net::UdpSocket> {
    peer.set_read_timeout(Some(Duration::from_secs(2)))?;
    let mut query = [0; RECEIVE_BYTES];
    let (length, _) = peer.recv_from(&mut query)?;
    assert_eq!(length, 48);
    assert_eq!(query[0], 0x23);
    assert_ne!(&query[40..48], &[0; 8]);
    Ok(peer)
}

/// Inspect the original three positions without fabricating a skipped `SlotReport`.
/// # Panics
/// Reject missing/duplicate dispositions, wrong peer order or false completion.
fn assert_managed_three_sequence(report: &crate::AddressRoundReport, peers: &[SocketAddr; 3]) {
    assert_eq!(report.end(), crate::AddressRoundEnd::Reply);
    assert!(report.primary_error().is_none());
    assert!(report.cleanup_errors().iter().all(Option::is_none));
    assert_eq!(report.policy_refusals().len(), 1);
    assert_eq!(report.policy_refusals()[0].index(), 0);
    assert_eq!(
        report.policy_refusals()[0].scope(),
        crate::AddressPolicyRefusalScope::Endpoint
    );
    let attempts = report.attempts();
    assert_eq!(attempts.len(), 2);
    assert_eq!(
        (attempts[0].peer(), attempts[0].report().slot()),
        (peers[1], 1)
    );
    assert_eq!(
        (attempts[1].peer(), attempts[1].report().slot()),
        (peers[2], 2)
    );
    assert_eq!(attempts[0].report().status(), Status::Timeout);
    assert_eq!(attempts[1].report().status(), Status::Header);
    assert!(attempts.iter().all(|attempt| attempt.report().sent()));
    assert_eq!(
        (
            attempts[0].report().received(),
            attempts[0].report().rejected()
        ),
        (0, 0)
    );
    assert_eq!(
        (
            attempts[1].report().received(),
            attempts[1].report().rejected()
        ),
        (1, 0)
    );
    assert_eq!(attempts[1].report().stratum(), Some(2));
}

/// Run genuine E refusal, F blackhole and G reply in one admitted address family.
/// # Errors
/// Preserve unavailable-family admission, native traffic and all scoped joins.
/// # Panics
/// Reject transitive state, changed retained floors or extra native queries.
fn managed_three_candidate_family(family: IpAddr) -> Check<()> {
    let authority = isolated_authority(SourcePolicyLimits::new(
        2,
        3,
        Duration::from_millis(1),
        Duration::from_millis(4),
    )?)?;
    let a = authority.source(0)?;
    let b = authority.source(1)?;
    let e_socket = std::net::UdpSocket::bind(SocketAddr::new(family, 0))?;
    let f_socket = std::net::UdpSocket::bind(SocketAddr::new(family, 0))?;
    let g_socket = std::net::UdpSocket::bind(SocketAddr::new(family, 0))?;
    let peers = [
        e_socket.local_addr()?,
        f_socket.local_addr()?,
        g_socket.local_addr()?,
    ];
    let (denied, e_socket) = managed_round_once(&a, &peers[..1], e_socket, Some(DENY))?;
    assert_eq!(denied.end(), crate::AddressRoundEnd::Control);
    assert_eq!(denied.attempts().len(), 1);
    assert_eq!(denied.attempts()[0].report().control_code(), Some(DENY));
    let ae = a.bind(peers[0])?;
    let be = b.bind(peers[0])?;
    let retained = facts(&ae)?;
    assert!(matches!(
        authority.endpoint_target(peers[1]),
        Err(SourcePolicyError::UnknownEndpoint)
    ));
    assert!(matches!(
        authority.endpoint_target(peers[2]),
        Err(SourcePolicyError::UnknownEndpoint)
    ));
    let (report, f_socket, g_socket) = std::thread::scope(|scope| -> Check<_> {
        let blackhole = scope.spawn(move || managed_three_blackhole(f_socket));
        // Public preparation admits both fresh F/G records in the two remaining
        // slots. The blackhole socket stays bound until this whole round returns.
        let result = managed_round_once(&b, &peers, g_socket, None);
        let observed = blackhole
            .join()
            .map_err(|_| "managed blackhole peer panicked")?;
        let (report, g_socket) = result?;
        assert_managed_three_sequence(&report, &peers);
        Ok((report, observed?, g_socket))
    })?;
    assert_managed_three_sequence(&report, &peers);
    assert_eq!(facts(&ae)?, retained);
    let b_facts = facts(&be)?;
    let f_facts = facts(&b.bind(peers[1])?)?;
    let g_facts = facts(&b.bind(peers[2])?)?;
    assert_eq!(b_facts[1], retained[1]);
    assert!(!b_facts[0].2 && !f_facts[1].2 && !g_facts[1].2);
    assert_eq!(b_facts[0].0, Duration::from_millis(1));
    assert!(f_facts[1].1.is_some() && g_facts[1].1.is_some());
    assert!(b_facts[0].1 >= f_facts[1].1);
    assert_eq!(b_facts[0].1, g_facts[1].1);
    for peer in [e_socket, f_socket, g_socket] {
        peer.set_nonblocking(true)?;
        let mut extra = [0; RECEIVE_BYTES];
        assert!(
            matches!(peer.recv_from(&mut extra), Err(error) if error.kind() == io::ErrorKind::WouldBlock)
        );
    }
    Ok(())
}

/// Exercise the complete public managed three-candidate path in both families.
/// # Errors
/// Propagate unavailable IPv6 or any fixture/product failure; never skip a family.
/// # Panics
/// Reject wrong actual order, traffic, refusal/history cardinality or retained state.
#[test]
fn public_managed_round_three_candidates_preserve_actual_fallback_in_both_families() -> Check<()> {
    for family in [
        IpAddr::V4(Ipv4Addr::LOCALHOST),
        IpAddr::V6(Ipv6Addr::LOCALHOST),
    ] {
        managed_three_candidate_family(family)?;
    }
    Ok(())
}

/// Complete public record observations, excluding the separately captured wait.
type VisibleRestriction = (Duration, Duration, Option<Instant>, bool, bool, u64, bool);

/// Borrow the public values without consulting the private record as an oracle.
fn visible_restriction(record: &SourceRestrictionSnapshot) -> VisibleRestriction {
    (
        record.interval(),
        record.maximum_interval(),
        record.floor(),
        record.refused(),
        record.unavailable(),
        record.revision(),
        record.at_maximum(),
    )
}

/// Observe real direct refusals and exact selective clearing without transitivity.
/// # Errors
/// Preserve checked policy, binding, reservation, control and target failures.
/// # Panics
/// Fail if a snapshot merges scopes, clears a floor, follows later mutations or
/// makes an already observed revision target current again.
fn snapshot_direct_refusal_case(code: u32) -> Check<()> {
    let mut authority = isolated_authority(limits(2, 2)?)?;
    let a = authority.source(0)?;
    let b = authority.source(1)?;
    let e = endpoint(43101);
    let f = endpoint(43102);
    let ae = a.bind(e)?;
    let _be = b.bind(e)?;
    let _af = a.bind(f)?;
    let _bf = b.bind(f)?;
    let now = Instant::now();
    let floor = now
        .checked_add(Duration::from_secs(1))
        .ok_or("snapshot floor overflow")?;
    let mut request = fixture_send(&ae, &Arc::new(()), now)?;
    fixture_control(&ae, &mut request, code, now)?;
    let before = Instant::now();
    let historical = a.policy_snapshot(e)?;
    assert!(before <= historical.captured_at() && historical.captured_at() <= Instant::now());
    let refused = (
        Duration::from_secs(1),
        Duration::from_secs(4),
        Some(floor),
        true,
        false,
        3,
        false,
    );
    let clear = (
        Duration::from_secs(1),
        Duration::from_secs(4),
        None,
        false,
        false,
        1,
        false,
    );
    assert_eq!(visible_restriction(historical.source()), refused);
    assert_eq!(visible_restriction(historical.endpoint()), refused);
    let be = b.policy_snapshot(e)?;
    let bf = b.policy_snapshot(f)?;
    let af = a.policy_snapshot(f)?;
    assert_eq!(visible_restriction(be.source()), clear);
    assert_eq!(visible_restriction(be.endpoint()), refused);
    assert_eq!(visible_restriction(bf.source()), clear);
    assert_eq!(visible_restriction(bf.endpoint()), clear);
    assert_eq!(visible_restriction(af.source()), refused);
    assert_eq!(visible_restriction(af.endpoint()), clear);
    reenable_visible_refusals(&mut authority, &a, &b, [e, f], floor, &historical)
}

/// Consume exact current targets while preserving independently observed history.
/// # Errors
/// Preserve actual target issuance, selective clearing and snapshot failures.
/// # Panics
/// Fail if unselected state, a floor or an earlier observation changes.
fn reenable_visible_refusals(
    authority: &mut SourcePolicyAuthority,
    a: &SourceHandle,
    b: &SourceHandle,
    [e, f]: [SocketAddr; 2],
    floor: Instant,
    historical: &SourcePolicySnapshot,
) -> Check<()> {
    let refused = (
        Duration::from_secs(1),
        Duration::from_secs(4),
        Some(floor),
        true,
        false,
        3,
        false,
    );
    let clear = (
        Duration::from_secs(1),
        Duration::from_secs(4),
        None,
        false,
        false,
        1,
        false,
    );
    let old_source_target = authority.source_target(a)?;
    authority.reenable(&[authority.endpoint_target(e)?])?;
    let endpoint_cleared = a.policy_snapshot(e)?;
    assert_eq!(visible_restriction(endpoint_cleared.source()), refused);
    assert_eq!(
        visible_restriction(endpoint_cleared.endpoint()),
        (
            Duration::from_secs(1),
            Duration::from_secs(4),
            Some(floor),
            false,
            false,
            4,
            false
        )
    );
    assert!(a.policy_snapshot(f)?.source().refused());
    assert!(!b.policy_snapshot(e)?.source().refused());
    authority.reenable(&[authority.source_target(a)?])?;
    let cleared = a.policy_snapshot(e)?;
    assert!(!cleared.source().refused() && !cleared.endpoint().refused());
    assert_eq!(cleared.source().floor(), Some(floor));
    assert_eq!(cleared.endpoint().floor(), Some(floor));
    assert_eq!(cleared.source().revision(), 4);
    assert!(matches!(
        authority.reenable(&[old_source_target]),
        Err(SourcePolicyError::Stale)
    ));
    assert_eq!(visible_restriction(historical.source()), refused);
    assert_eq!(visible_restriction(historical.endpoint()), refused);
    assert_eq!(visible_restriction(b.policy_snapshot(f)?.source()), clear);
    Ok(())
}

/// Public observations preserve both direct control kinds and exact endpoint scope.
/// # Errors
/// Preserve each real kernel/target scenario's failure.
/// # Panics
/// Fail on transitive restrictions or changing historical observations.
#[test]
fn policy_snapshot_distinguishes_source_endpoint_and_selective_reenable() -> Check<()> {
    for code in [DENY, RSTR] {
        snapshot_direct_refusal_case(code)?;
    }
    Ok(())
}

/// A snapshot reports own intervals rather than an inherited effective maximum.
/// # Errors
/// Preserve checked allocation and actual RATE/reservation failures.
/// # Panics
/// Fail if a higher source interval is copied into its endpoint or another source.
#[test]
fn policy_snapshot_preserves_independent_rate_intervals() -> Check<()> {
    let authority = isolated_authority(limits(2, 2)?)?;
    let a = authority.source(0)?;
    let b = authority.source(1)?;
    let af = a.bind(endpoint(43112))?;
    let ae = a.bind(endpoint(43111))?;
    let _be = b.bind(endpoint(43111))?;
    let now = Instant::now();
    let mut first = fixture_send(&af, &Arc::new(()), now)?;
    fixture_control(&af, &mut first, RATE, now)?;
    let later = now
        .checked_add(Duration::from_secs(2))
        .ok_or("snapshot time overflow")?;
    let mut second = fixture_send(&ae, &Arc::new(()), later)?;
    fixture_control(&ae, &mut second, RATE, later)?;
    let observed = a.policy_snapshot(endpoint(43111))?;
    assert_eq!(observed.source().interval(), Duration::from_secs(4));
    assert_eq!(observed.endpoint().interval(), Duration::from_secs(2));
    assert!(observed.source().at_maximum());
    assert!(!observed.endpoint().at_maximum());
    assert_eq!(
        observed.source().floor(),
        later.checked_add(Duration::from_secs(4))
    );
    assert_eq!(
        observed.endpoint().floor(),
        later.checked_add(Duration::from_secs(2))
    );
    let alias = b.policy_snapshot(endpoint(43111))?;
    assert_eq!(alias.source().interval(), Duration::from_secs(1));
    assert_eq!(alias.source().floor(), None);
    assert_eq!(alias.endpoint().interval(), Duration::from_secs(2));
    assert!(!alias.source().refused() && !alias.endpoint().refused());
    Ok(())
}

/// Elapsed observation time and a successful reply cannot reset a capped RATE.
/// # Errors
/// Preserve genuine kernel requests, observations and checked modeled times.
/// # Panics
/// Fail if elapsed time mutates a captured wait, clears a cap or refunds a floor.
#[test]
fn policy_snapshot_capped_rate_and_wait_are_historical() -> Check<()> {
    let authority = isolated_authority(limits(1, 1)?)?;
    let source = authority.source(0)?;
    let peer = endpoint(43121);
    let binding = source.bind(peer)?;
    // Explicit modeled kernel times keep the floor ahead of the hardware capture;
    // these calls are not claimed as native send/receive observations.
    let mut now = Instant::now()
        .checked_add(Duration::from_secs(60))
        .ok_or("snapshot time overflow")?;
    for seconds in [2, 4, 4] {
        let mut request = fixture_send(&binding, &Arc::new(()), now)?;
        fixture_control(&binding, &mut request, RATE, now)?;
        now = now
            .checked_add(Duration::from_secs(seconds))
            .ok_or("snapshot floor overflow")?;
    }
    let historical = source.policy_snapshot(peer)?;
    let saved_wait = historical.source().remaining_at_capture();
    assert!(!saved_wait.is_zero());
    assert_eq!(historical.source().floor(), Some(now));
    assert_eq!(historical.source().revision(), 7);
    assert_eq!(historical.endpoint().revision(), 7);
    for record in [historical.source(), historical.endpoint()] {
        assert_eq!(record.interval(), Duration::from_secs(4));
        assert_eq!(record.maximum_interval(), Duration::from_secs(4));
        assert!(record.at_maximum());
        assert!(!record.refused() && !record.unavailable());
    }
    std::thread::sleep(Duration::from_millis(2));
    let later = source.policy_snapshot(peer)?;
    assert!(later.captured_at() >= historical.captured_at());
    assert!(later.source().remaining_at_capture() <= saved_wait);
    assert_eq!(historical.source().remaining_at_capture(), saved_wait);
    let mut healthy = fixture_send(&binding, &Arc::new(()), now)?;
    fixture_receive(&binding, &mut healthy)?;
    let after_success = source.policy_snapshot(peer)?;
    assert_eq!(after_success.source().interval(), Duration::from_secs(4));
    assert_eq!(after_success.endpoint().interval(), Duration::from_secs(4));
    assert_eq!(
        after_success.source().floor(),
        now.checked_add(Duration::from_secs(4))
    );
    assert_eq!(after_success.source().revision(), 8);
    assert_eq!(historical.source().revision(), 7);
    assert_eq!(historical.source().floor(), Some(now));
    let elapsed_authority = isolated_authority(limits(1, 1)?)?;
    let elapsed_source = elapsed_authority.source(0)?;
    let elapsed_peer = endpoint(43122);
    let elapsed_binding = elapsed_source.bind(elapsed_peer)?;
    // Admit a real kernel reservation at an explicit modeled past time.
    let past = Instant::now()
        .checked_sub(Duration::from_secs(2))
        .ok_or("snapshot past time overflow")?;
    let elapsed_floor = past
        .checked_add(Duration::from_secs(1))
        .ok_or("snapshot elapsed floor overflow")?;
    let _elapsed_request = fixture_send(&elapsed_binding, &Arc::new(()), past)?;
    let retained = facts(&elapsed_binding)?;
    let elapsed = elapsed_source.policy_snapshot(elapsed_peer)?;
    assert!(elapsed.captured_at() > elapsed_floor);
    for record in [elapsed.source(), elapsed.endpoint()] {
        assert_eq!(record.floor(), Some(elapsed_floor));
        assert_eq!(record.remaining_at_capture(), Duration::ZERO);
        assert_eq!(record.interval(), Duration::from_secs(1));
        assert_eq!(record.revision(), 2);
    }
    assert_eq!(facts(&elapsed_binding)?, retained);
    Ok(())
}

/// Reads preserve change versions while retaining ordinary contention-release wakes.
/// # Errors
/// Preserve runtime, actual policy custody, reservation and watcher failures.
/// # Panics
/// Fail on a changed record, stranded release waiter or future-floor self-wake.
#[test]
fn policy_snapshot_does_not_publish_changes_or_wake_a_future_floor() -> Check<()> {
    use std::{
        future::Future,
        task::{Context, Poll, Waker},
    };

    runtime()?.block_on(async {
        let authority = isolated_authority(SourcePolicyLimits::new(
            1,
            1,
            Duration::from_secs(30),
            Duration::from_secs(30),
        )?)?;
        let source = authority.source(0)?;
        let peer = endpoint(43131);
        let binding = source.bind(peer)?;
        let request = Arc::new(());
        let _reservation = fixture_send(&binding, &request, Instant::now())?;
        let retained = facts(&binding)?;
        let mut live = LivePolicy::from_mode(SourceMode::Managed(binding.clone()));
        live.refresh(&request)?;
        let (source_changes, endpoint_changes) = {
            let guard = authority.cell.lock()?;
            (
                guard
                    .state()?
                    .record(RecordKey::Source(0))?
                    .changes
                    .subscribe(),
                guard
                    .state()?
                    .record(RecordKey::Endpoint(binding.endpoint))?
                    .changes
                    .subscribe(),
            )
        };
        for _ in 0..2 {
            let mut released = std::pin::pin!(authority.cell.released.notified());
            assert!(!released.as_mut().enable());
            let observed = source.policy_snapshot(peer)?;
            assert_eq!(observed.source().revision(), 2);
            let mut context = Context::from_waker(Waker::noop());
            assert!(matches!(
                released.as_mut().poll(&mut context),
                Poll::Ready(())
            ));
            assert!(!source_changes.has_changed()? && !endpoint_changes.has_changed()?);
            assert!(
                tokio::time::timeout(Duration::from_millis(10), live.wake())
                    .await
                    .is_err()
            );
        }
        assert_eq!(facts(&binding)?, retained);
        assert!(!live.wants_writable());
        Ok(())
    })
}

/// Unknown reads consume no canonical capacity; aliases retain the actual record.
/// # Errors
/// Preserve real allocator/binding and checked endpoint observation failures.
/// # Panics
/// Fail on hidden endpoint binding, alias duplication or loss of required IPv6 zone.
#[test]
fn policy_snapshot_canonical_aliases_and_unknown_reads_do_not_allocate() -> Check<()> {
    let authority = isolated_authority(limits(1, 2)?)?;
    let source = authority.source(0)?;
    let peer = endpoint(43141);
    let binding = source.bind(peer)?;
    let before = facts(&binding)?;
    for _ in 0..3 {
        assert!(matches!(
            source.policy_snapshot(endpoint(43142)),
            Err(SourcePolicyError::UnknownEndpoint)
        ));
    }
    assert!(matches!(
        source.policy_snapshot(endpoint(0)),
        Err(SourcePolicyError::InvalidEndpoint)
    ));
    let mapped = SocketAddr::new(Ipv4Addr::LOCALHOST.to_ipv6_mapped().into(), 43141);
    assert_eq!(
        visible_restriction(source.policy_snapshot(mapped)?.endpoint()),
        (
            Duration::from_secs(1),
            Duration::from_secs(4),
            None,
            false,
            false,
            1,
            false
        )
    );
    let address = "fe80::1".parse::<Ipv6Addr>()?;
    let scoped = SocketAddr::V6(SocketAddrV6::new(address, 43143, 0, 7));
    let _scoped_binding = source.bind(scoped)?;
    // That second binding fits only if all unknown diagnostic reads were inert.
    assert!(source.policy_snapshot(scoped).is_ok());
    let different_zone = SocketAddr::V6(SocketAddrV6::new(address, 43143, 0, 8));
    let missing_zone = SocketAddr::V6(SocketAddrV6::new(address, 43143, 0, 0));
    assert!(matches!(
        source.policy_snapshot(different_zone),
        Err(SourcePolicyError::UnknownEndpoint)
    ));
    assert!(matches!(
        source.policy_snapshot(missing_zone),
        Err(SourcePolicyError::InvalidEndpoint)
    ));
    assert_eq!(facts(&binding)?, before);
    Ok(())
}

/// Genuine arithmetic exhaustion remains readable without recovering authority.
/// # Errors
/// Preserve policy transitions and independently located representability bounds.
/// # Panics
/// Fail if sticky unavailable state is hidden, cleared, or confused with refusal.
#[test]
fn policy_snapshot_keeps_unavailable_records_as_data() -> Check<()> {
    let authority = isolated_authority(limits(1, 1)?)?;
    let source = authority.source(0)?;
    let peer = endpoint(43151);
    let binding = source.bind(peer)?;
    let start = Instant::now();
    for now in [
        start,
        start
            .checked_add(Duration::from_secs(2))
            .ok_or("snapshot time overflow")?,
    ] {
        let mut request = fixture_send(&binding, &Arc::new(()), now)?;
        fixture_control(&binding, &mut request, RATE, now)?;
    }
    let retained_floor = start
        .checked_add(Duration::from_secs(6))
        .ok_or("snapshot floor overflow")?;
    assert!(matches!(
        fixture_send(&binding, &Arc::new(()), near_instant_limit(start)?),
        Err(SourcePolicyError::Unavailable)
    ));
    let observed = source.policy_snapshot(peer)?;
    for record in [observed.source(), observed.endpoint()] {
        assert_eq!(record.interval(), Duration::from_secs(4));
        assert_eq!(record.floor(), Some(retained_floor));
        assert_eq!(record.revision(), 5);
        assert!(record.at_maximum() && record.unavailable());
        assert!(!record.refused());
    }
    let held = authority.cell.lock()?;
    assert!(matches!(
        source.policy_snapshot(peer),
        Err(SourcePolicyError::Contended)
    ));
    drop(held);
    assert!(source.policy_snapshot(peer)?.source().unavailable());
    Ok(())
}

/// A genuinely poisoned cell refuses observation; another owner remains readable.
/// # Errors
/// Preserve real policy setup, lock and healthy neighboring observation failures.
/// # Panics
/// Fail if observation hides poisoning, repairs it or poisons another root.
#[test]
fn policy_snapshot_preserves_poison_and_the_healthy_owner() -> Check<()> {
    use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};

    let authority = isolated_authority(limits(1, 1)?)?;
    let source = authority.source(0)?;
    let peer = endpoint(43161);
    let _binding = source.bind(peer)?;
    let session = source.session(SeriesConfig::new(
        peer,
        Duration::from_millis(400),
        Duration::from_secs(2),
    )?)?;
    let historical = source.policy_snapshot(peer)?;
    let cell = Arc::clone(&authority.cell);
    let held = cell.lock()?;
    assert!(matches!(
        session.policy_snapshot(),
        Err(SourcePolicyError::Contended)
    ));
    let unwind: Result<(), _> = catch_unwind(AssertUnwindSafe(move || {
        let _held_during_unwind = held;
        resume_unwind(Box::new("snapshot policy poisoning specimen"));
    }));
    assert!(unwind.is_err());
    assert!(cell.state.is_poisoned());
    assert!(matches!(
        source.policy_snapshot(peer),
        Err(SourcePolicyError::Poisoned)
    ));
    assert!(matches!(
        session.policy_snapshot(),
        Err(SourcePolicyError::Poisoned)
    ));
    assert!(!historical.source().unavailable());
    let healthy = isolated_authority(limits(1, 1)?)?;
    let neighbor = healthy.source(0)?;
    let _neighbor_binding = neighbor.bind(peer)?;
    assert_eq!(neighbor.policy_snapshot(peer)?.source().revision(), 1);
    let healthy_session = neighbor.session(SeriesConfig::new(
        peer,
        Duration::from_millis(400),
        Duration::from_secs(2),
    )?)?;
    assert!(
        healthy_session
            .policy_snapshot()?
            .ok_or("healthy managed policy absent")?
            .endpoint()
            .floor()
            .is_none()
    );
    assert!(cell.state.is_poisoned());
    Ok(())
}

/// Drive an actual public managed session while a bounded scoped peer responds.
/// # Errors
/// Preserve setup, public preparation/execution, native reply and joined peer errors.
fn snapshot_public_session_exchange(
    session: &mut AcquisitionSession,
    peer: &std::net::UdpSocket,
    code: Option<u32>,
    responses: usize,
) -> Check<AcquisitionReport> {
    drop(runtime()?);
    peer.set_read_timeout(Some(Duration::from_secs(2)))?;
    let (prepared, control) = session.prepare()?;
    std::thread::scope(|scope| {
        let responder = scope.spawn(|| -> io::Result<()> {
            for _ in 0..responses {
                let mut packet = [0; RECEIVE_BYTES];
                let (length, client) = peer.recv_from(&mut packet)?;
                managed_round_reply(peer, &packet, length, client, code)?;
            }
            Ok(())
        });
        let result = prepared.execute(&crate::ControlSchedule::new(None, None)?);
        let peer_result = responder
            .join()
            .map_err(|_| "snapshot session peer panicked")?;
        drop(control);
        peer_result?;
        Ok(result?)
    })
}

/// Inspect every actual ordinary result against its explicitly configured owner.
/// # Errors
/// Preserve missing real terminal reports or transport facts as fixture failures.
/// # Panics
/// Reject wrong status, accounting, source identity or fabricated unmanaged identity.
fn committed_source_provenance<'a>(
    report: &'a AcquisitionReport,
    expected: Option<&SourceIdentity>,
) -> Check<&'a crate::TransportFacts> {
    assert_eq!(report.series().end(), crate::SeriesEnd::Completed);
    for attempt in report.series().attempts() {
        let result = attempt.report().ok_or("actual source report missing")?;
        assert_eq!(result.status(), Status::Header);
        assert_eq!((result.received(), result.rejected()), (1, 0));
        let observed = result.facts().ok_or("actual source facts missing")?;
        match (observed.source_identity(), expected) {
            (Some(actual), Some(expected)) => assert!(actual.same_source(expected)),
            (None, None) => {}
            _ => {
                return Err("actual source ownership presence disagrees with configuration".into());
            }
        }
    }
    report.series().attempts()[0]
        .report()
        .and_then(SlotReport::facts)
        .ok_or_else(|| "first actual source fact missing".into())
}

/// Logical source identity survives endpoint/session changes without endpoint votes.
/// # Errors
/// Preserve actual socket, checked owner/configuration and joined acquisition errors.
/// # Panics
/// Reject endpoint/index-based identity, invented unmanaged provenance or retained
/// mutable policy custody after all actual owners have been dropped.
#[test]
fn managed_source_provenance_follows_actual_owner_across_sessions_and_endpoints() -> Check<()> {
    let authority = isolated_authority(SourcePolicyLimits::new(
        2,
        2,
        Duration::from_millis(1),
        Duration::from_millis(4),
    )?)?;
    let a = authority.source(0)?;
    let b = authority.source(1)?;
    let e = std::net::UdpSocket::bind(endpoint(0))?;
    let f = std::net::UdpSocket::bind(endpoint(0))?;
    let config = |peer: &std::net::UdpSocket| -> Check<SeriesConfig> {
        Ok(SeriesConfig::new(
            peer.local_addr()?,
            Duration::from_millis(400),
            Duration::from_secs(2),
        )?)
    };
    let a_identity = a.identity();
    let b_identity = b.identity();
    assert!(!a_identity.same_source(&b_identity));
    assert!(a.identity().same_source(&a_identity));
    assert_eq!(format!("{a_identity:?}"), "SourceIdentity { .. }");
    let mut ae = a.session(config(&e)?)?;
    let mut af = a.session(config(&f)?)?;
    let mut be = b.session(config(&e)?)?;
    let original_report = snapshot_public_session_exchange(&mut ae, &e, None, 3)?;
    let moved_report = snapshot_public_session_exchange(&mut af, &f, None, 3)?;
    let be_report = snapshot_public_session_exchange(&mut be, &e, None, 3)?;
    let original_facts = committed_source_provenance(&original_report, Some(&a_identity))?;
    let moved_facts = committed_source_provenance(&moved_report, Some(&a_identity))?;
    let be_facts = committed_source_provenance(&be_report, Some(&b_identity))?;
    assert!(!original_report.same_session(&moved_report));
    assert!(!original_facts.same_exchange(moved_facts));
    assert!(!original_facts.same_exchange(be_facts));
    assert_eq!(original_facts.peer_endpoint(), be_facts.peer_endpoint());
    assert_ne!(original_facts.peer_endpoint(), moved_facts.peer_endpoint());

    // Equal source indices and endpoints in independently owned domains are not
    // the same logical source; actual report consumption must preserve that.
    let foreign = isolated_authority(SourcePolicyLimits::new(
        1,
        1,
        Duration::from_millis(1),
        Duration::from_millis(4),
    )?)?;
    let foreign_source = foreign.source(0)?;
    let foreign_identity = foreign_source.identity();
    assert!(!a_identity.same_source(&foreign_identity));
    let mut foreign_session = foreign_source.session(config(&e)?)?;
    let foreign_report = snapshot_public_session_exchange(&mut foreign_session, &e, None, 3)?;
    let foreign_facts = committed_source_provenance(&foreign_report, Some(&foreign_identity))?;
    assert_eq!(
        foreign_facts.peer_endpoint(),
        original_facts.peer_endpoint()
    );
    assert!(!foreign_facts.same_exchange(original_facts));
    let mut unmanaged = AcquisitionSession::new(config(&e)?);
    let unmanaged_report = snapshot_public_session_exchange(&mut unmanaged, &e, None, 3)?;
    committed_source_provenance(&unmanaged_report, None)?;

    ae.retire();
    assert!(matches!(
        ae.prepare(),
        Err(crate::AcquisitionError::Retired)
    ));
    committed_source_provenance(&original_report, Some(&a_identity))?;
    let old_cell = Arc::downgrade(&a.cell);
    drop((ae, af, be, a, b, authority));
    assert!(old_cell.upgrade().is_none());
    // Historical token/report retention carries no live policy lock or records.
    committed_source_provenance(&original_report, Some(&a_identity))?;
    committed_source_provenance(&moved_report, Some(&a_identity))?;
    Ok(())
}

/// Public session status consumes genuine control traffic without granting retry.
/// # Errors
/// Preserve actual sockets, public managed/unmanaged owners and joined acquisitions.
/// # Panics
/// Fail on fabricated clear state, hidden restriction, lost report or invented traffic.
#[test]
fn policy_snapshot_public_sessions_observe_real_restriction_and_unmanaged_absence() -> Check<()> {
    let authority = isolated_authority(SourcePolicyLimits::new(
        2,
        2,
        Duration::from_millis(1),
        Duration::from_millis(4),
    )?)?;
    let a = authority.source(0)?;
    let b = authority.source(1)?;
    let e = std::net::UdpSocket::bind(endpoint(0))?;
    let f = std::net::UdpSocket::bind(endpoint(0))?;
    let config = || -> Check<SeriesConfig> {
        Ok(SeriesConfig::new(
            e.local_addr()?,
            Duration::from_millis(400),
            Duration::from_secs(2),
        )?)
    };
    let mut session = a.session(config()?)?;
    let historical = session
        .policy_snapshot()?
        .ok_or("managed policy observation absent")?;
    assert!(!historical.source().refused() && !historical.endpoint().refused());
    let report = snapshot_public_session_exchange(&mut session, &e, Some(DENY), 1)?;
    let first = report.series().attempts()[0]
        .report()
        .ok_or("actual DENY report missing")?;
    assert_eq!(first.status(), Status::Control);
    assert_eq!(first.control_code(), Some(DENY));
    assert_eq!((first.received(), first.rejected()), (1, 0));
    let restricted = session
        .policy_snapshot()?
        .ok_or("managed restriction hidden")?;
    assert!(restricted.source().refused() && restricted.endpoint().refused());
    assert!(!historical.source().refused() && !historical.endpoint().refused());
    session.retire();
    assert!(
        session
            .policy_snapshot()?
            .ok_or("retired diagnostic absent")?
            .source()
            .refused()
    );
    let mut replacement = a.session(config()?)?;
    assert!(matches!(
        replacement.prepare(),
        Err(crate::AcquisitionError::SourcePolicy(
            SourcePolicyError::Disabled
        ))
    ));
    check_healthy_snapshot_session(&b, &f)?;
    assert_eq!((first.received(), first.rejected()), (1, 0));
    e.set_nonblocking(true)?;
    let mut extra = [0; RECEIVE_BYTES];
    assert!(
        matches!(e.recv_from(&mut extra), Err(error) if error.kind() == io::ErrorKind::WouldBlock)
    );
    let mut unmanaged = AcquisitionSession::new(config()?);
    assert!(unmanaged.policy_snapshot()?.is_none());
    unmanaged.retire();
    assert!(unmanaged.policy_snapshot()?.is_none());
    Ok(())
}

/// Pair the refused public session with three actual healthy public acquisitions.
/// # Errors
/// Preserve actual configuration, joined replies and managed observation failures.
/// # Panics
/// Fail on missing/extra attempts, rejected healthy replies or hidden policy data.
fn check_healthy_snapshot_session(source: &SourceHandle, peer: &std::net::UdpSocket) -> Check<()> {
    let mut healthy = source.session(SeriesConfig::new(
        peer.local_addr()?,
        Duration::from_millis(400),
        Duration::from_secs(2),
    )?)?;
    let initial = healthy
        .policy_snapshot()?
        .ok_or("healthy initial policy absent")?;
    // E already occupies the refused first record; this session owns distinct F.
    for record in [initial.source(), initial.endpoint()] {
        assert_eq!(
            visible_restriction(record),
            (
                Duration::from_millis(1),
                Duration::from_millis(4),
                None,
                false,
                false,
                1,
                false
            )
        );
    }
    let report = snapshot_public_session_exchange(&mut healthy, peer, None, 3)?;
    assert_eq!(report.series().end(), crate::SeriesEnd::Completed);
    assert_eq!(report.series().attempts().len(), 3);
    assert!(report.series().attempts().iter().all(|attempt| {
        attempt
            .report()
            .is_some_and(|value| value.status() == Status::Header)
    }));
    let observed = healthy.policy_snapshot()?.ok_or("healthy policy absent")?;
    for record in [observed.source(), observed.endpoint()] {
        assert_eq!(record.interval(), Duration::from_millis(1));
        assert_eq!(record.maximum_interval(), Duration::from_millis(4));
        assert!(record.floor().is_some());
        assert!(!record.refused() && !record.unavailable() && !record.at_maximum());
        assert_eq!(record.revision(), 4);
    }
    Ok(())
}

/// A finite prefix sink reports its original error instead of completing a row.
struct SnapshotPrefixSink {
    /// Actual bytes accepted before the explicit failure boundary.
    bytes: Vec<u8>,
    /// Maximum accepted prefix for this one diagnostic operation.
    limit: usize,
}

impl io::Write for SnapshotPrefixSink {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let remaining = self.limit.saturating_sub(self.bytes.len());
        if remaining == 0 {
            return Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "snapshot sink refused",
            ));
        }
        let count = remaining.min(bytes.len());
        self.bytes.extend_from_slice(&bytes[..count]);
        Ok(count)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Independent static fields and the modeled retained floor for one output row.
type ExpectedSnapshotStatus = (&'static str, &'static str, Option<Instant>);

/// Check literal fields and elapsed wait from a known kernel floor, not getters.
/// # Errors
/// Preserve writer/UTF-8/decimal failures and missing output fields.
/// # Panics
/// Fail on wrong scope/flags/interval, extra output or an incorrect captured wait.
fn check_snapshot_status_rows(
    snapshot: &SourcePolicySnapshot,
    expected: [ExpectedSnapshotStatus; 2],
) -> Check<()> {
    let mut bytes = Vec::new();
    snapshot.write_status(&mut bytes)?;
    assert!(bytes.len() <= 512);
    let mut rows = std::str::from_utf8(&bytes)?.lines();
    for (prefix, suffix, floor) in expected {
        let row = rows.next().ok_or("snapshot status row absent")?;
        let (actual_prefix, remainder) = row
            .split_once(" remaining_at_capture_ns=")
            .ok_or("snapshot wait field absent")?;
        let (wait, actual_suffix) = remainder.split_once(' ').ok_or("snapshot flags absent")?;
        assert_eq!(actual_prefix, prefix);
        assert_eq!(actual_suffix, suffix);
        let elapsed_wait = floor.map_or(Duration::ZERO, |at| {
            at.saturating_duration_since(snapshot.captured_at())
        });
        assert_eq!(wait.parse::<u128>()?, elapsed_wait.as_nanos());
    }
    assert!(rows.next().is_none());
    Ok(())
}

/// Render actual capped RATE, direct refusal and arithmetic exhaustion states.
/// # Errors
/// Preserve real policy kernels, observations and independently checked times.
/// # Panics
/// Fail if a positive flag is hardcoded clear or an endpoint copies source state.
fn check_positive_snapshot_status_fields() -> Check<()> {
    const SOURCE_CAP: &str =
        "event=source_policy scope=source interval_ns=4000000000 maximum_ns=4000000000";
    const ENDPOINT_CAP: &str =
        "event=source_policy scope=endpoint interval_ns=4000000000 maximum_ns=4000000000";
    const ENDPOINT_INITIAL: &str =
        "event=source_policy scope=endpoint interval_ns=1000000000 maximum_ns=4000000000";
    const CAP: &str = "refused=0 unavailable=0 at_maximum=1 eligible=0 fresh_sync=0";
    const REFUSED: &str = "refused=1 unavailable=0 at_maximum=1 eligible=0 fresh_sync=0";
    const CLEAR: &str = "refused=0 unavailable=0 at_maximum=0 eligible=0 fresh_sync=0";
    const UNAVAILABLE: &str = "refused=0 unavailable=1 at_maximum=1 eligible=0 fresh_sync=0";
    let mut authority = isolated_authority(limits(1, 2)?)?;
    let source = authority.source(0)?;
    let e = endpoint(43181);
    let f = endpoint(43182);
    let binding = source.bind(e)?;
    let _other_endpoint = source.bind(f)?;
    let start = Instant::now();
    let base = start
        .checked_add(Duration::from_secs(60))
        .ok_or("status time overflow")?;
    // These are real policy transitions with explicit modeled times, not UDP work.
    for now in [
        base,
        base.checked_add(Duration::from_secs(2))
            .ok_or("status time overflow")?,
    ] {
        let mut request = fixture_send(&binding, &Arc::new(()), now)?;
        fixture_control(&binding, &mut request, RATE, now)?;
    }
    let capped_floor = base
        .checked_add(Duration::from_secs(6))
        .ok_or("status floor overflow")?;
    check_snapshot_status_rows(
        &source.policy_snapshot(e)?,
        [
            (SOURCE_CAP, CAP, Some(capped_floor)),
            (ENDPOINT_CAP, CAP, Some(capped_floor)),
        ],
    )?;
    let mut request = fixture_send(&binding, &Arc::new(()), capped_floor)?;
    fixture_control(&binding, &mut request, DENY, capped_floor)?;
    let refused_floor = base
        .checked_add(Duration::from_secs(10))
        .ok_or("status floor overflow")?;
    check_snapshot_status_rows(
        &source.policy_snapshot(f)?,
        [
            (SOURCE_CAP, REFUSED, Some(refused_floor)),
            (ENDPOINT_INITIAL, CLEAR, None),
        ],
    )?;
    authority.reenable(&[
        authority.source_target(&source)?,
        authority.endpoint_target(e)?,
    ])?;
    assert!(matches!(
        fixture_send(&binding, &Arc::new(()), near_instant_limit(start)?),
        Err(SourcePolicyError::Unavailable)
    ));
    check_snapshot_status_rows(
        &source.policy_snapshot(e)?,
        [
            (SOURCE_CAP, UNAVAILABLE, Some(refused_floor)),
            (ENDPOINT_CAP, UNAVAILABLE, Some(refused_floor)),
        ],
    )?;
    Ok(())
}

/// Exact status fields omit peer/owner/revision data and survive partial sink failure.
/// # Errors
/// Preserve real snapshot capture and successful neighboring output failures.
/// # Panics
/// Fail on extra fields, unbounded output, swallowed sink error or policy mutation.
#[test]
fn policy_snapshot_status_is_bounded_payload_free_and_preserves_sink_errors() -> Check<()> {
    check_positive_snapshot_status_fields()?;
    let authority = isolated_authority(limits(1, 1)?)?;
    let source = authority.source(0)?;
    let peer = endpoint(43171);
    let binding = source.bind(peer)?;
    let observed = source.policy_snapshot(peer)?;
    let retained = facts(&binding)?;
    let expected = concat!(
        "event=source_policy scope=source interval_ns=1000000000 maximum_ns=4000000000 remaining_at_capture_ns=0 refused=0 unavailable=0 at_maximum=0 eligible=0 fresh_sync=0\n",
        "event=source_policy scope=endpoint interval_ns=1000000000 maximum_ns=4000000000 remaining_at_capture_ns=0 refused=0 unavailable=0 at_maximum=0 eligible=0 fresh_sync=0\n",
    );
    let mut bytes = Vec::new();
    observed.write_status(&mut bytes)?;
    assert_eq!(bytes, expected.as_bytes());
    assert!(bytes.len() <= 512);
    assert_eq!(format!("{observed:?}"), "SourcePolicySnapshot { .. }");
    assert_eq!(
        format!("{:?}", observed.source()),
        "SourceRestrictionSnapshot { .. }"
    );
    for limit in [
        0,
        1,
        expected.len().checked_sub(1).ok_or("empty status oracle")?,
    ] {
        let mut sink = SnapshotPrefixSink {
            bytes: Vec::new(),
            limit,
        };
        let Err(failure) = observed.write_status(&mut sink) else {
            return Err("partial status unexpectedly succeeded".into());
        };
        assert_eq!(failure.kind(), io::ErrorKind::BrokenPipe);
        assert_eq!(sink.bytes, expected.as_bytes()[..limit]);
        assert_eq!(facts(&binding)?, retained);
        assert_eq!(observed.source().revision(), 1);
    }
    let mut healthy = Vec::new();
    observed.write_status(&mut healthy)?;
    assert_eq!(healthy, expected.as_bytes());
    assert_eq!(facts(&binding)?, retained);
    Ok(())
}

#[path = "source_configuration.rs"]
mod configuration;

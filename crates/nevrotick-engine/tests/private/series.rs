// SPDX-License-Identifier: GPL-2.0-only
//! Private raw-field, actual request-identity and atomic receiving-state controls.

use super::*;

#[path = "series_process.rs"]
mod series_process;
use std::{
    error::Error,
    net::{Ipv4Addr, Ipv6Addr, SocketAddrV6},
};

/// Explicit laboratory endpoint; fixture sockets never send to it in these tests.
fn peer() -> SocketAddr {
    SocketAddr::from((Ipv4Addr::LOCALHOST, 41001))
}

/// Own a bounded I/O-enabled runtime for genuine socket registration fixtures.
/// # Errors
/// Preserve the actual runtime construction failure.
fn runtime() -> Result<tokio::runtime::Runtime, std::io::Error> {
    tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()
}

/// Retain a real origin without asserting a production mapping or clock generation.
fn context() -> Arc<RunClockContext> {
    Arc::new(RunClockContext {
        origin: Instant::now(),
    })
}

/// Copy an actual request descriptor for a replay/substitution fault control.
fn copy_owner(owner: &RequestOwner) -> RequestOwner {
    RequestOwner {
        index: owner.index,
        identity: Arc::clone(&owner.identity),
        context: Arc::clone(&owner.context),
        peer: owner.peer,
        local: owner.local,
        profile: owner.profile,
    }
}

/// Register one genuine fresh sending socket without a network operation.
/// # Errors
/// Preserve actual socket preparation/registration failures.
/// # Panics
/// Require admission of the independently fixed loopback endpoint.
fn sending_slot(shared: Arc<RunClockContext>) -> Result<LiveSlot, PilotError> {
    let mut packet = [0; PACKET_BYTES];
    packet[0] = 0x23;
    packet[40..48].copy_from_slice(&[1; 8]);
    let prepared = prepare_slot(peer(), packet);
    assert!(
        !matches!(&prepared, Err(PilotError::InvalidPeer)),
        "fixed loopback preparation rejected its bound local endpoint"
    );
    let prepared = prepared?;
    LiveSlot::from_prepared(prepared, shared)
}

/// Register and retire one genuine socket without a network send.
/// # Errors
/// Preserve actual socket preparation/registration failures.
/// # Panics
/// Require admission of the independently fixed loopback endpoint.
fn terminal_slot(shared: Arc<RunClockContext>) -> Result<LiveSlot, PilotError> {
    let mut slot = sending_slot(shared)?;
    slot.finish(TerminalOutcome::Timeout);
    Ok(slot)
}

/// Move an actual registered terminal owner through the shared report consumer.
/// # Errors
/// Preserve socket, registration and report-domain failures.
/// # Panics
/// Require a terminal owner at a legal position to publish its report.
fn completion(index: u8, shared: Arc<RunClockContext>) -> Result<Completion, PilotError> {
    let slot = terminal_slot(shared)?;
    let owner = RequestOwner::from_slot(index, &slot);
    let report = slot.into_report(index);
    if index <= 2 {
        assert!(report.is_ok(), "legal terminal position lost its report");
    }
    Ok(Completion {
        owner,
        report: report?,
    })
}

/// Forge a duplicate transfer retaining genuine original identity for guard tests.
/// This synthetic timeout record is not independent native acquisition evidence.
fn duplicate_timeout(original: &Completion) -> Completion {
    Completion {
        owner: copy_owner(&original.owner),
        report: SlotReport {
            slot: original.report.slot,
            local: original.report.local,
            outcome: TerminalOutcome::Timeout,
            sent: original.report.sent,
            received: original.report.received,
            rejected: original.report.rejected,
            last_rejection: original.report.last_rejection,
        },
    }
}

/// Copy a genuine header only to substitute one private receiving-boundary field.
/// The copied record supplies no new native acquisition or sample authority.
/// # Errors
/// Preserve absence of the required original header payload.
fn duplicate_header(original: &Completion) -> Result<Completion, PilotError> {
    let TerminalOutcome::Header { stratum, facts } = &original.report.outcome else {
        return Err(PilotError::InternalState);
    };
    Ok(Completion {
        owner: copy_owner(&original.owner),
        report: SlotReport {
            slot: original.report.slot,
            local: original.report.local,
            outcome: TerminalOutcome::Header {
                stratum: *stratum,
                facts: Box::new(crate::TransportFacts {
                    send_before: facts.send_before,
                    send_after: facts.send_after,
                    read_return: facts.read_return,
                    processing: facts.processing,
                    peer_receive_bits: facts.peer_receive_bits,
                    peer_transmit_bits: facts.peer_transmit_bits,
                    header: facts.header,
                    peer: facts.peer,
                    local: facts.local,
                    identity: Arc::clone(&facts.identity),
                    run_context: Arc::clone(&facts.run_context),
                    measurement: facts.measurement,
                    profile: facts.profile,
                }),
            },
            sent: original.report.sent,
            received: original.report.received,
            rejected: original.report.rejected,
            last_rejection: original.report.last_rejection,
        },
    })
}

/// Exchange one real loopback request and queue its valid reply without reading it.
/// The fixed two-second waits bound fixture acquisition, not product time policy.
/// # Errors
/// Preserve actual runtime/socket/readiness/I/O failures.
/// # Panics
/// Require quiet control/deadline/issue prerequisites, the predeadline charge,
/// and the independently observed query's pending owner.
async fn queued_native_reply() -> Result<(LiveSlot, Instant), Box<dyn Error>> {
    let lifetime = Duration::from_secs(2);
    let remote = crate::StdSocket::bind(SocketAddr::from((Ipv4Addr::LOCALHOST, 0)))?;
    remote.set_read_timeout(Some(lifetime))?;
    remote.set_write_timeout(Some(lifetime))?;
    let destination = remote.local_addr()?;
    let mut packet = [0; PACKET_BYTES];
    packet[0] = 0x23;
    packet[40..48].copy_from_slice(&[7; 8]);
    // The actual peer was bound to IPv4 loopback above. An InvalidPeer domain
    // refusal is distinct from the still-fallible native preparation operations.
    let prepared = prepare_slot(destination, packet);
    assert!(
        !matches!(&prepared, Err(PilotError::InvalidPeer)),
        "bound loopback preparation rejected its local endpoint"
    );
    let prepared = prepared?;
    let mut slot = LiveSlot::from_prepared(prepared, context())?;
    let before = Instant::now();
    let deadline = before
        .checked_add(lifetime)
        .ok_or(PilotError::DeadlineOverflow)?;
    let quiet = Control {
        stop: false,
        epoch: 0,
    };
    assert_eq!(series_end(quiet, before, deadline), None);
    refresh_slot(&mut slot, quiet, before, deadline);
    assert!(matches!(&slot.phase, crate::Phase::Sending));
    // This separate descriptor owner borrows no socket and cannot alter the
    // genuine slot. Its pure issue prerequisite precedes dependent peer work.
    let mut boundary = SeriesReceiver::new(Arc::clone(&slot.run_context), slot.peer, slot.profile);
    let issued = boundary.issue(RequestOwner::from_slot(0, &slot));
    assert!(
        issued.is_ok(),
        "genuine request owner was refused before native work"
    );
    issued?;
    drop(boundary);
    tokio::time::timeout(lifetime, slot.socket.writable()).await??;
    let (_sender, mut control) = watch::channel(quiet);
    let observed = handle_series_writable(&mut slot, &mut control, Ok(()), deadline, quiet);
    assert!(!observed.stop && observed.epoch == 0);
    if Instant::now() < deadline {
        assert_eq!(
            slot.send_attempts, 1,
            "fresh predeadline send was not charged once"
        );
        assert!(
            !matches!(
                &slot.phase,
                crate::Phase::Done(TerminalOutcome::Budget | TerminalOutcome::Timeout)
            ),
            "fresh predeadline send retired without an exhausted budget"
        );
    }
    // The fixture observes a 48-byte protocol query plus one overflow byte;
    // mutating the product's receive capacity must not shrink this peer buffer.
    let mut received = [0; 49];
    let (length, client) = remote.recv_from(&mut received)?;
    assert_eq!(length, 48, "peer observed a non-header query length");
    assert!(
        matches!(&slot.phase, crate::Phase::Pending(_)),
        "observed complete query did not leave its owner pending"
    );
    assert!(
        slot.can_receive(),
        "fresh pending query cannot receive its reply"
    );
    let mut reply = [0; PACKET_BYTES];
    reply[0] = 0x24;
    reply[1] = 2;
    reply[24..32].copy_from_slice(&received[40..48]);
    reply[32..40].copy_from_slice(&1_u64.to_be_bytes());
    reply[40..48].copy_from_slice(&2_u64.to_be_bytes());
    if remote.send_to(&reply, client)? != PACKET_BYTES {
        return Err(PilotError::InternalState.into());
    }
    Ok((slot, deadline))
}

/// A genuine selected native datagram, retained separately from later publication.
struct SelectedReply {
    /// Actual pending request/socket that produced this receive.
    slot: LiveSlot,
    /// Actual native read classification and its adjacent software bracket.
    selected: SeriesRead,
    /// Original bounded receive bytes, never printed by assertions.
    buffer: [u8; RECEIVE_BYTES],
    /// Original bounded request deadline.
    deadline: Instant,
}

/// Select exactly one newly observed kernel datagram without charging/publishing it.
/// # Errors
/// Preserve actual queueing/receive failures; no synthetic read is counted as native.
async fn selected_native_reply() -> Result<SelectedReply, Box<dyn Error>> {
    let (slot, deadline) = queued_native_reply().await?;
    let mut received = [0; RECEIVE_BYTES];
    let lifetime = Duration::from_secs(2);
    let result = tokio::time::timeout(lifetime, slot.socket.recv_from(&mut received)).await?;
    let read = SoftwareClock::capture();
    let result = classify_read_result(NativeDatagramPlatform::current(), Ok(result?));
    Ok(SelectedReply {
        slot,
        selected: SeriesRead { result, read },
        buffer: received,
        deadline,
    })
}

/// Stage one real selected loopback reply without changing the pending request.
/// # Errors
/// Preserve actual fixture failures and typed rejection of unexpected reply facts.
/// # Panics
/// Require the genuine valid datagram to retain its classification and staging.
async fn staged_native_reply() -> Result<(LiveSlot, StagedReply, Instant), Box<dyn Error>> {
    let SelectedReply {
        slot,
        selected,
        buffer,
        deadline,
    } = selected_native_reply().await?;
    assert!(
        matches!(&selected.result, NativeReadOutcome::Datagram { .. }),
        "successful native reply lost its datagram classification"
    );
    let NativeReadOutcome::Datagram { length, source } = selected.result else {
        return Err(PilotError::InternalState.into());
    };
    let limits = FrameLimits::new(PACKET_BYTES, 0).map_err(|_| PilotError::WireSetup)?;
    let staged = stage_reply(&slot, &buffer[..length], source, selected.read, limits);
    assert!(
        staged.is_ok(),
        "genuine valid reply was refused during pure staging"
    );
    let staged = staged.map_err(|_| PilotError::InternalState)?;
    Ok((slot, staged, deadline))
}

/// Controlled change applied after the genuine reply has already been staged.
#[derive(Clone, Copy)]
enum PublicationCase {
    /// No intervening change: the valid neighboring header commits.
    Healthy,
    /// Publish a stop after staging.
    Stop,
    /// Publish an invalidated epoch after staging.
    Invalidate,
    /// Supply an already reached processing deadline after staging.
    Expired,
}

/// The production post-staging guard independently rejects each intervening change.
/// # Errors
/// Preserve actual fixture failures; the test performs no retirement/refresh itself.
/// # Panics
/// Require an unchanged healthy neighbor and causal stop/stale/deadline refusal.
#[test]
fn actual_publication_rechecks_control_and_deadline_after_staging() -> Result<(), Box<dyn Error>> {
    let runtime = runtime()?;
    for (case, expected) in [
        (PublicationCase::Healthy, Status::Header),
        (PublicationCase::Stop, Status::Stopped),
        (PublicationCase::Invalidate, Status::Stale),
        (PublicationCase::Expired, Status::Timeout),
    ] {
        let (mut slot, staged, mut deadline) = runtime.block_on(staged_native_reply())?;
        let (sender, mut control) = watch::channel(Control {
            stop: false,
            epoch: 0,
        });
        let handle = ControlHandle { sender };
        match case {
            PublicationCase::Healthy => {}
            PublicationCase::Stop => handle.stop(),
            PublicationCase::Invalidate => handle.invalidate()?,
            PublicationCase::Expired => deadline = Instant::now(),
        }
        let observed = publish_series_reply(&mut slot, &mut control, Ok(staged), deadline);
        assert_eq!(
            slot.terminal().ok_or(PilotError::InternalState)?.status(),
            expected
        );
        if expected == Status::Header {
            assert_eq!(slot.rejected, 0);
            assert!(matches!(
                slot.terminal()
                    .and_then(TerminalOutcome::facts)
                    .ok_or(PilotError::InternalState)?
                    .measurement(),
                Err(crate::MeasurementRefusal::Unavailable)
            ));
        } else {
            assert_eq!(slot.rejected, 1);
            assert!(slot.terminal().and_then(TerminalOutcome::facts).is_none());
        }
        assert_eq!(observed.stop, matches!(case, PublicationCase::Stop));
        assert_eq!(
            observed.epoch,
            u64::from(matches!(case, PublicationCase::Invalidate))
        );
    }
    Ok(())
}

/// Actual mandatory config/token constructors reject bypasses and accept neighbors.
/// # Errors
/// Preserve an unexpected rejection of independent valid laboratory facts.
/// # Panics
/// Require endpoint precedence, complete duration bounds and every token position.
#[test]
fn raw_config_and_tokens_have_mandatory_bounded_admission() -> Result<(), PilotError> {
    let one = Duration::from_secs(1);
    for invalid in [
        SocketAddr::from(([192, 0, 2, 1], 1)),
        SocketAddr::from(([127, 0, 0, 1], 0)),
        SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 1, 1, 0)),
        SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 1, 0, 1)),
    ] {
        assert!(matches!(
            SeriesConfig::new(invalid, Duration::ZERO, Duration::ZERO),
            Err(PilotError::InvalidPeer)
        ));
    }
    for (request, series) in [
        (Duration::ZERO, one),
        (one, Duration::ZERO),
        (one + one, one),
        (one, MAX_TIMEOUT + Duration::from_nanos(1)),
    ] {
        assert!(matches!(
            SeriesConfig::new(peer(), request, series),
            Err(PilotError::InvalidTimeout)
        ));
    }
    let plan = SeriesConfig::new(peer(), MAX_TIMEOUT, MAX_TIMEOUT)?;
    assert_eq!(plan.peer, peer());
    assert_eq!(plan.request_timeout, MAX_TIMEOUT);
    assert_eq!(plan.series_timeout, MAX_TIMEOUT);
    assert!(
        ConfigCandidate {
            values: (peer(), one, one)
        }
        .validate()
        .is_ok()
    );
    assert!(
        ConfigCandidate {
            values: (peer(), one + one, one)
        }
        .validate()
        .is_err()
    );
    let mut bytes = [0; 24];
    bytes[..8].fill(1);
    bytes[8..16].fill(2);
    bytes[16..].fill(3);
    let packets = series_packets(&bytes)?;
    for (index, packet) in packets.iter().enumerate() {
        assert!(packet[0] == 0x23, "fixed request header changed");
        assert!(packet[1..40] == [0; 39], "fixed request prefix changed");
        assert!(
            packet[40..48] == bytes[index * 8..index * 8 + 8],
            "request token association changed"
        );
    }
    for index in 0..3 {
        let mut zero = bytes;
        zero[index * 8..index * 8 + 8].fill(0);
        assert!(matches!(series_packets(&zero), Err(PilotError::ZeroToken)));
    }
    for (left, right) in [(0, 1), (0, 2), (1, 2)] {
        let mut duplicate = [[1; 8], [2; 8], [3; 8]];
        duplicate[right] = duplicate[left];
        assert!(TokenCandidate { tokens: duplicate }.validate().is_err());
        let mut raw = [0; 24];
        let (token_slots, _) = raw.as_chunks_mut::<8>();
        for (target, token) in token_slots.iter_mut().zip(duplicate) {
            *target = token;
        }
        assert!(matches!(
            series_packets(&raw),
            Err(PilotError::DuplicateToken)
        ));
    }
    Ok(())
}

/// Actual shared report admission accepts all three positions and rejects larger ones.
/// # Errors
/// Preserve runtime/socket fixture failures.
/// # Panics
/// Require index two success and typed refusal of three and the maximum raw u8.
#[test]
fn genuine_terminal_report_admits_only_the_three_position_domain() -> Result<(), Box<dyn Error>> {
    let runtime = runtime()?;
    let _entered = runtime.enter();
    for index in [0, 1, 2] {
        let report = terminal_slot(context())?.into_report(index);
        assert!(report.is_ok(), "legal terminal index was refused");
        let report = report?;
        assert_eq!(report.slot(), index);
        assert_eq!(report.status(), Status::Timeout);
    }
    for index in [3, u8::MAX] {
        assert!(matches!(
            terminal_slot(context())?.into_report(index),
            Err(PilotError::InternalState)
        ));
    }
    Ok(())
}

/// Refuse isolated issue faults before retaining the genuine first owner.
/// # Errors
/// Preserve actual socket/profile construction failures.
/// # Panics
/// Require refusal without mutation and a positive genuine issue.
fn issue_boundary_neighbors(
    receiver: &mut SeriesReceiver,
    first: &Completion,
) -> Result<(), Box<dyn Error>> {
    // Each refused issue differs in one independent guard operand, with no
    // historical identity reuse or foreign context masking that operand.
    let future = completion(1, Arc::clone(&first.owner.context))?;
    assert!(matches!(
        receiver.issue(copy_owner(&future.owner)),
        Err(PilotError::InternalState)
    ));
    assert_eq!(receiver.next, 0);
    assert!(receiver.issued.is_none());
    let declared = ExperimentalProfile::new(3_000_000_000, 4_500_000_000, 30_000_000_000);
    assert!(declared.is_ok(), "independently valid profile was refused");
    let mut wrong_profile = copy_owner(&first.owner);
    wrong_profile.profile = Some(declared?);
    assert!(matches!(
        receiver.issue(wrong_profile),
        Err(PilotError::InternalState)
    ));
    assert_eq!(receiver.next, 0);
    assert!(receiver.issued.is_none());
    receiver.issue(copy_owner(&first.owner))?;
    let outstanding = completion(0, Arc::clone(&first.owner.context))?;
    assert!(matches!(
        receiver.issue(copy_owner(&outstanding.owner)),
        Err(PilotError::InternalState)
    ));
    assert_eq!(receiver.next, 0);
    assert!(
        receiver
            .issued
            .as_ref()
            .is_some_and(|issued| { Arc::ptr_eq(&issued.identity, &first.owner.identity) })
    );
    Ok(())
}

/// Identity substitution/replay cannot replace prior records or reuse request owners.
/// # Errors
/// Preserve actual fixture failures and unexpected healthy receiving rejection.
/// # Panics
/// Require unchanged earlier records and healthy completion after each guard fault.
#[test]
fn receiver_rejects_duplicate_foreign_and_reused_identity_atomically() -> Result<(), Box<dyn Error>>
{
    let runtime = runtime()?;
    let _entered = runtime.enter();
    let shared = context();
    let mut receiver = SeriesReceiver::new(Arc::clone(&shared), peer(), None);
    let first = completion(0, Arc::clone(&shared))?;
    let replay = duplicate_timeout(&first);
    let mut reused_owner = copy_owner(&first.owner);
    issue_boundary_neighbors(&mut receiver, &first)?;
    receiver.receive(
        first,
        Control {
            stop: false,
            epoch: 0,
        },
    )?;
    assert!(matches!(
        receiver.receive(
            replay,
            Control {
                stop: false,
                epoch: 0
            }
        ),
        Err(PilotError::InternalState)
    ));
    assert_eq!(receiver.next, 1);
    reused_owner.index = 1;
    assert!(matches!(
        receiver.issue(reused_owner),
        Err(PilotError::InternalState)
    ));
    let foreign = completion(1, context())?;
    assert!(matches!(
        receiver.issue(copy_owner(&foreign.owner)),
        Err(PilotError::InternalState)
    ));
    let second = completion(1, Arc::clone(&shared))?;
    receiver.issue(copy_owner(&second.owner))?;
    let mut substituted = duplicate_timeout(&second);
    substituted.owner.identity = Arc::new(());
    assert!(matches!(
        receiver.receive(
            substituted,
            Control {
                stop: false,
                epoch: 0
            }
        ),
        Err(PilotError::InternalState)
    ));
    assert_eq!(receiver.next, 1);
    assert!(receiver.issued.is_some());
    receiver.receive(
        second,
        Control {
            stop: false,
            epoch: 0,
        },
    )?;
    let report = receiver.finish(SeriesEnd::Failure, Some(PilotError::WireSetup));
    assert_eq!(
        report.attempts()[0]
            .report()
            .ok_or(PilotError::InternalState)?
            .status(),
        Status::Timeout
    );
    assert_eq!(
        report.attempts()[1]
            .report()
            .ok_or(PilotError::InternalState)?
            .status(),
        Status::Timeout
    );
    assert_eq!(
        report.attempts()[2].not_started_cause(),
        Some(SeriesNotStartedCause::PrimaryFailure)
    );
    assert_eq!(
        report
            .primary_failure()
            .ok_or(PilotError::InternalState)?
            .code(),
        "wire_setup"
    );
    assert_eq!(report.admitted_sample_count(), 0);
    assert_eq!(report.eligible_sample_count(), 0);
    Ok(())
}

/// Preserve the exact prior snapshot when a writable result belongs to a retired owner.
/// # Errors
/// Preserve actual native fixture preparation failures.
/// # Panics
/// Require no changed control, native error replacement or additional send.
fn retired_writable_neighbor(shared: &Arc<RunClockContext>) -> Result<(), PilotError> {
    // A completed owner must not consume a later watch snapshot or an observed
    // error. This is a direct writable-boundary control, not another native send.
    let mut already_done = terminal_slot(Arc::clone(shared))?;
    let quiet = Control {
        stop: false,
        epoch: 0,
    };
    let (sender, mut control) = watch::channel(Control {
        stop: true,
        epoch: 1,
    });
    let observed = handle_series_writable(
        &mut already_done,
        &mut control,
        Err(std::io::Error::from(std::io::ErrorKind::PermissionDenied)),
        Instant::now(),
        quiet,
    );
    assert!(!observed.stop && observed.epoch == 0);
    assert_eq!(
        already_done.terminal().map(TerminalOutcome::status),
        Some(Status::Timeout)
    );
    assert_eq!(already_done.send_attempts, 0);
    assert!(!already_done.sent);
    drop(sender);
    Ok(())
}

/// Exercise current stop/stale/deadline and modeled native-error writable branches.
/// # Errors
/// Preserve actual socket setup and host deadline expiry as fixture failures.
/// # Panics
/// Require exact retirement and zero native send attempts after those guards.
fn writable_retirement_neighbors(shared: &Arc<RunClockContext>) -> Result<(), Box<dyn Error>> {
    let quiet = Control {
        stop: false,
        epoch: 0,
    };
    for (state, expired, expected) in [
        (
            Control {
                stop: true,
                epoch: 0,
            },
            false,
            Status::Stopped,
        ),
        (
            Control {
                stop: false,
                epoch: 1,
            },
            false,
            Status::Stale,
        ),
        (quiet, true, Status::Timeout),
        (quiet, false, Status::Io),
    ] {
        let mut active = sending_slot(Arc::clone(shared))?;
        let start = Instant::now();
        let deadline = if expired {
            start
        } else {
            start
                .checked_add(Duration::from_secs(2))
                .ok_or(PilotError::DeadlineOverflow)?
        };
        let (sender, mut control) = watch::channel(state);
        // This modeled error is an input to the product's real native-error
        // branch; it is not an assertion that this host produced that error.
        let observed = handle_series_writable(
            &mut active,
            &mut control,
            Err(std::io::Error::from(std::io::ErrorKind::PermissionDenied)),
            deadline,
            quiet,
        );
        if !expired && Instant::now() >= deadline {
            return Err(std::io::Error::other("writable neighbor fixture deadline elapsed").into());
        }
        assert_eq!(observed.stop, state.stop);
        assert_eq!(observed.epoch, state.epoch);
        assert_eq!(
            active.terminal().map(TerminalOutcome::status),
            Some(expected)
        );
        assert_eq!(active.send_attempts, 0);
        assert!(!active.sent);
        if expected == Status::Io {
            assert_eq!(
                active
                    .terminal()
                    .and_then(TerminalOutcome::io_cause)
                    .map(IoCause::kind),
                Some(std::io::ErrorKind::PermissionDenied)
            );
        } else {
            assert!(
                active
                    .terminal()
                    .and_then(TerminalOutcome::io_cause)
                    .is_none()
            );
        }
        drop(sender);
    }
    Ok(())
}

/// Current controls reject stale transfers and cannot undo an already retired slot.
/// # Errors
/// Preserve fixture failures or a healthy transfer's unexpected rejection.
/// # Panics
/// Require actual same-slot retirement, preserved timeout and explicit later causes.
#[test]
fn transfer_control_and_irreversible_retirement_are_distinct() -> Result<(), Box<dyn Error>> {
    let runtime = runtime()?;
    let _entered = runtime.enter();
    let shared = context();
    let mut receiver = SeriesReceiver::new(Arc::clone(&shared), peer(), None);
    let first = completion(0, Arc::clone(&shared))?;
    receiver.issue(copy_owner(&first.owner))?;
    for state in [
        Control {
            stop: true,
            epoch: 0,
        },
        Control {
            stop: false,
            epoch: 1,
        },
    ] {
        assert!(matches!(
            receiver.receive(duplicate_timeout(&first), state),
            Err(PilotError::InternalState)
        ));
        assert_eq!(receiver.next, 0);
    }
    receiver.receive(
        first,
        Control {
            stop: false,
            epoch: 0,
        },
    )?;
    let mut retired = terminal_slot(Arc::clone(&shared))?;
    let now = Instant::now();
    refresh_slot(
        &mut retired,
        Control {
            stop: true,
            epoch: 1,
        },
        now,
        now,
    );
    assert_eq!(
        retired
            .terminal()
            .ok_or(PilotError::InternalState)?
            .status(),
        Status::Timeout
    );
    let report = receiver.finish(SeriesEnd::Stopped, None);
    assert!(report.attempts()[0].report().is_some());
    assert_eq!(
        report.attempts()[1].not_started_cause(),
        Some(SeriesNotStartedCause::Stopped)
    );
    assert_eq!(
        report.attempts()[2].not_started_cause(),
        Some(SeriesNotStartedCause::Stopped)
    );
    assert!(report.primary_failure().is_none());
    retired_writable_neighbor(&shared)?;
    writable_retirement_neighbors(&shared)?;
    Ok(())
}

/// Refuse one synthetic receiving-field substitution at a time, without mutation.
/// # Errors
/// Preserve absence of genuine header facts or checked profile construction.
/// # Panics
/// Require every independent owner/facts/report predicate and intact pending state.
fn receive_boundary_neighbors(
    receiver: &mut SeriesReceiver,
    completed: &Completion,
    observed: Control,
) -> Result<(), Box<dyn Error>> {
    assert_eq!(completed.report.status(), Status::Header);
    assert!(completed.report.facts().is_some());
    let declared = ExperimentalProfile::new(3_000_000_000, 4_500_000_000, 30_000_000_000);
    assert!(declared.is_ok(), "independently valid profile was refused");
    let declared = declared?;
    // Synthetic substitutions retain the actual owner and all other genuine
    // header fields. They are raw private guard inputs, not new acquisitions.
    for fault in 0..7 {
        let mut substituted = duplicate_header(completed)?;
        if fault == 5 {
            substituted.report.slot = 1;
        } else if fault == 6 {
            substituted.report.local = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 0));
        } else {
            let TerminalOutcome::Header { facts, .. } = &mut substituted.report.outcome else {
                return Err(PilotError::InternalState.into());
            };
            match fault {
                0 => facts.identity = Arc::new(()),
                1 => facts.run_context = context(),
                2 => facts.peer = SocketAddr::from((Ipv4Addr::UNSPECIFIED, facts.peer.port())),
                3 => facts.local = SocketAddr::from((Ipv4Addr::UNSPECIFIED, facts.local.port())),
                _ => facts.profile = Some(declared),
            }
        }
        assert!(matches!(
            receiver.receive(substituted, observed),
            Err(PilotError::InternalState)
        ));
        assert_eq!(receiver.next, 0);
        assert!(
            receiver
                .issued
                .as_ref()
                .is_some_and(|issued| { Arc::ptr_eq(&issued.identity, &completed.owner.identity) })
        );
        assert!(receiver.attempts.iter().all(|attempt| {
            matches!(&attempt.state, AttemptState::NotStarted(_)) && attempt.identity.is_none()
        }));
    }
    Ok(())
}

/// A post-commit control cannot erase a genuine consumed header with unavailable arithmetic.
/// # Errors
/// Preserve actual registration/parser/setup and healthy transfer failures.
/// # Panics
/// Require irreversible header retirement, retained refusal and next-attempt stop.
#[test]
fn post_commit_control_preserves_header_and_measurement_refusal() -> Result<(), Box<dyn Error>> {
    let runtime = runtime()?;
    let (mut slot, staged, deadline) = runtime.block_on(staged_native_reply())?;
    let shared = Arc::clone(&slot.run_context);
    let destination = slot.peer;
    let snapshot = Control {
        stop: false,
        epoch: 0,
    };
    let (sender, mut control) = watch::channel(snapshot);
    let handle = ControlHandle { sender };
    let observed = publish_series_reply(&mut slot, &mut control, Ok(staged), deadline);
    assert!(slot.done());
    let mut receiver = SeriesReceiver::new(shared, destination, None);
    receiver.issue(RequestOwner::from_slot(0, &slot))?;
    let owner = RequestOwner::from_slot(0, &slot);
    let completed = Completion {
        owner,
        report: slot.into_report(0)?,
    };
    receive_boundary_neighbors(&mut receiver, &completed, observed)?;
    handle.stop();
    // No new observation belongs between terminal commit and its private transfer.
    receiver.receive(completed, observed)?;
    let current = latest(&mut control);
    let now = Instant::now();
    assert_eq!(series_end(current, now, now), Some(SeriesEnd::Stopped));
    let report = receiver.finish(SeriesEnd::Stopped, None);
    let first = report.attempts()[0]
        .report()
        .ok_or(PilotError::InternalState)?;
    assert_eq!(first.status(), Status::Header);
    assert!(matches!(
        first
            .facts()
            .ok_or(PilotError::InternalState)?
            .measurement(),
        Err(crate::MeasurementRefusal::Unavailable)
    ));
    assert_eq!(
        report.attempts()[1].not_started_cause(),
        Some(SeriesNotStartedCause::Stopped)
    );
    assert_eq!(
        report.attempts()[2].not_started_cause(),
        Some(SeriesNotStartedCause::Stopped)
    );
    Ok(())
}

/// Independent exact deadline/control facts preserve precedence and total bounds.
/// # Errors
/// Preserve impossible checked deadline construction failures.
/// # Panics
/// Require inclusive expiration, request clipping and stop-before-stale precedence.
#[test]
fn deadlines_and_controls_do_not_reset_the_series_origin() -> Result<(), PilotError> {
    let origin = Instant::now();
    let total = origin
        .checked_add(Duration::from_secs(2))
        .ok_or(PilotError::DeadlineOverflow)?;
    assert_eq!(
        request_deadline(origin, Duration::from_secs(1), total)?,
        origin + Duration::from_secs(1)
    );
    assert_eq!(
        request_deadline(
            origin + Duration::from_secs(1),
            Duration::from_secs(2),
            total
        )?,
        total
    );
    assert_eq!(
        series_end(
            Control {
                stop: false,
                epoch: 0
            },
            origin,
            total
        ),
        None
    );
    assert_eq!(
        series_end(
            Control {
                stop: false,
                epoch: 0
            },
            total,
            total
        ),
        Some(SeriesEnd::SeriesDeadline)
    );
    assert_eq!(
        series_end(
            Control {
                stop: false,
                epoch: 1
            },
            total,
            total
        ),
        Some(SeriesEnd::Stale)
    );
    assert_eq!(
        series_end(
            Control {
                stop: true,
                epoch: 1
            },
            total,
            total
        ),
        Some(SeriesEnd::Stopped)
    );
    Ok(())
}

/// Keep each cleanup cause independent of an otherwise nonfailure partial report.
/// # Errors
/// Preserve actual registered-owner construction or receiving failures.
/// # Panics
/// Require either single cleanup error to mark Failure, with all earlier rows intact.
fn cleanup_boundary_neighbors() -> Result<(), Box<dyn Error>> {
    for (stop_failed, invalidate_failed) in
        [(false, false), (true, false), (false, true), (true, true)]
    {
        let shared = context();
        let mut receiver = SeriesReceiver::new(Arc::clone(&shared), peer(), None);
        let first = completion(0, shared)?;
        receiver.issue(copy_owner(&first.owner))?;
        receiver.receive(
            first,
            Control {
                stop: false,
                epoch: 0,
            },
        )?;
        let partial = receiver.finish(SeriesEnd::Stopped, None);
        let report = reconcile_cleanup(
            partial,
            if stop_failed {
                Err(PilotError::ControlTask)
            } else {
                Ok(())
            },
            if invalidate_failed {
                Err(PilotError::GenerationExhausted)
            } else {
                Ok(())
            },
        );
        assert_eq!(
            report.end(),
            if stop_failed || invalidate_failed {
                SeriesEnd::Failure
            } else {
                SeriesEnd::Stopped
            }
        );
        assert!(report.primary_failure().is_none());
        assert_eq!(
            report.stop_cleanup_failure().map(FailureSummary::code),
            stop_failed.then_some("control_task")
        );
        assert_eq!(
            report
                .invalidate_cleanup_failure()
                .map(FailureSummary::code),
            invalidate_failed.then_some("generation_exhausted")
        );
        assert_eq!(
            report.attempts()[0].report().map(SlotReport::status),
            Some(Status::Timeout)
        );
        for later in &report.attempts()[1..] {
            assert_eq!(
                later.not_started_cause(),
                Some(SeriesNotStartedCause::Stopped)
            );
        }
    }
    Ok(())
}

/// Cleanup failures retain all earlier records and every independently typed cause.
/// # Errors
/// Preserve actual fixture and healthy receiving failures.
/// # Panics
/// Require primary/stop/invalidation coexistence and reject fake complete reports.
#[test]
fn partial_report_preserves_primary_and_both_cleanup_causes() -> Result<(), Box<dyn Error>> {
    let runtime = runtime()?;
    let _entered = runtime.enter();
    let shared = context();
    let mut receiver = SeriesReceiver::new(Arc::clone(&shared), peer(), None);
    let first = completion(0, shared)?;
    receiver.issue(copy_owner(&first.owner))?;
    receiver.receive(
        first,
        Control {
            stop: false,
            epoch: 0,
        },
    )?;
    let partial = receiver.finish(SeriesEnd::Failure, Some(PilotError::WireSetup));
    let report = reconcile_cleanup(
        partial,
        Err(PilotError::ControlTask),
        Err(PilotError::GenerationExhausted),
    );
    assert_eq!(report.end(), SeriesEnd::Failure);
    assert_eq!(
        report
            .primary_failure()
            .ok_or(PilotError::InternalState)?
            .code(),
        "wire_setup"
    );
    assert_eq!(
        report
            .stop_cleanup_failure()
            .ok_or(PilotError::InternalState)?
            .code(),
        "control_task"
    );
    assert_eq!(
        report
            .invalidate_cleanup_failure()
            .ok_or(PilotError::InternalState)?
            .code(),
        "generation_exhausted"
    );
    assert!(report.attempts()[0].report().is_some());
    for attempt in &report.attempts()[1..] {
        assert_eq!(
            attempt.not_started_cause(),
            Some(SeriesNotStartedCause::PrimaryFailure)
        );
    }
    let invalid = SeriesReceiver::new(context(), peer(), None).finish(SeriesEnd::Completed, None);
    assert_eq!(invalid.end(), SeriesEnd::Failure);
    assert_eq!(
        invalid
            .primary_failure()
            .ok_or(PilotError::InternalState)?
            .code(),
        "internal_state"
    );
    let healthy = reconcile_cleanup(invalid, Ok(()), Ok(()));
    assert!(healthy.stop_cleanup_failure().is_none());
    assert!(healthy.invalidate_cleanup_failure().is_none());
    assert_eq!(
        healthy
            .primary_failure()
            .ok_or(PilotError::InternalState)?
            .code(),
        "internal_state"
    );

    cleanup_boundary_neighbors()?;
    Ok(())
}

/// Assert fixed redaction without rendering either payload on a failing assertion.
/// # Panics
/// Report only fixed text if any Debug implementation exposes additional content.
fn assert_fixed_redaction(actual: &str, expected: &str) {
    assert!(
        actual == expected,
        "fixed acquisition redaction output changed"
    );
}

/// Four new Debug implementations keep exact fixed text despite retained facts/tokens.
/// # Errors
/// Preserve actual acquisition and healthy construction/publication failures.
/// # Panics
/// Fixed assertion messages never include token, packet, endpoint or captured payloads.
#[test]
fn all_series_debug_outputs_are_exact_fixed_redactions() -> Result<(), Box<dyn Error>> {
    let runtime = runtime()?;
    let (mut slot, staged, deadline) = runtime.block_on(staged_native_reply())?;
    let config = SeriesConfig::new(slot.peer, Duration::from_secs(1), Duration::from_secs(2))?;
    let (prepared, handle) = prepare_series_with_entropy(&config, |entropy| {
        for (position, token) in entropy.as_chunks_mut::<8>().0.iter_mut().enumerate() {
            *token = [u8::try_from(position + 1).map_err(|_| PilotError::InternalState)?; 8];
        }
        Ok(())
    })?;
    assert_fixed_redaction(&format!("{config:?}"), "SeriesConfig { .. }");
    assert_fixed_redaction(&format!("{prepared:?}"), "PreparedSeries { .. }");
    let mut receiver = SeriesReceiver::new(Arc::clone(&slot.run_context), slot.peer, None);
    let (sender, mut control) = watch::channel(Control {
        stop: false,
        epoch: 0,
    });
    let checkpoint = publish_series_reply(&mut slot, &mut control, Ok(staged), deadline);
    receiver.issue(RequestOwner::from_slot(0, &slot))?;
    let owner = RequestOwner::from_slot(0, &slot);
    receiver.receive(
        Completion {
            owner,
            report: slot.into_report(0)?,
        },
        checkpoint,
    )?;
    let report = receiver.finish(SeriesEnd::Stopped, None);
    assert_fixed_redaction(
        &format!("{:?}", report.attempts()[0]),
        "SeriesAttempt { .. }",
    );
    assert_fixed_redaction(&format!("{report:?}"), "SeriesReport { .. }");
    drop((sender, handle, prepared));
    Ok(())
}

/// Read-cap checks model prior counters; only the new queued datagram is native evidence.
/// # Errors
/// Preserve actual queueing, driver and post-driver observation failures.
/// # Panics
/// Require the 64th selected read to work and a queued 65th to remain unconsumed.
#[test]
fn single_slot_driver_enforces_receive_cap_with_valid_boundary_neighbor()
-> Result<(), Box<dyn Error>> {
    if series_process::parent_or_child(
        "series::tests::single_slot_driver_enforces_receive_cap_with_valid_boundary_neighbor",
    )? {
        return Ok(());
    }
    let runtime = runtime()?;
    runtime.block_on(async {
        let limits = FrameLimits::new(PACKET_BYTES, 0).map_err(|_| PilotError::WireSetup)?;
        let (mut neighbor, deadline) = queued_native_reply().await?;
        // Modeled prior history: these assignments are not 63 observed kernel reads.
        neighbor.received = 63;
        neighbor.rejected = 63;
        let (_sender, mut control) = watch::channel(Control {
            stop: false,
            epoch: 0,
        });
        let (neighbor, _) = run_request(neighbor, &mut control, deadline, limits).await;
        assert_eq!(neighbor.received, 64);
        assert_eq!(neighbor.rejected, 63);
        assert_eq!(
            neighbor
                .terminal()
                .ok_or(PilotError::InternalState)?
                .status(),
            Status::Header
        );
        drop(neighbor);

        let (mut capped, _) = queued_native_reply().await?;
        // Modeled exhausted history: no claim of 64 prior real receives is made.
        capped.received = 64;
        capped.rejected = 64;
        let deadline = Instant::now()
            .checked_add(Duration::from_secs(2))
            .ok_or(PilotError::DeadlineOverflow)?;
        let (_sender, mut control) = watch::channel(Control {
            stop: false,
            epoch: 0,
        });
        let (capped, _) = run_request(capped, &mut control, deadline, limits).await;
        assert_eq!(capped.received, 64);
        assert_eq!(capped.rejected, 64);
        assert_eq!(
            capped.terminal().ok_or(PilotError::InternalState)?.status(),
            Status::Budget
        );
        let mut bytes = [0; RECEIVE_BYTES];
        // This observer's read is after the driver returned, outside product counters.
        let (length, _) = capped.socket.try_recv_from(&mut bytes)?;
        assert_eq!(length, PACKET_BYTES);
        assert!(capped.terminal().and_then(TerminalOutcome::facts).is_none());
        Ok::<(), Box<dyn Error>>(())
    })
}

/// Reply to exactly one newly observed kernel query through a separately owned peer.
/// # Errors
/// Preserve bounded actual receive/send errors; never print opaque query bytes.
/// # Panics
/// Require the independently observed query to have the exact protocol length.
async fn reply_once(remote: &tokio::net::UdpSocket) -> Result<(), Box<dyn Error>> {
    let mut query = [0; 49];
    let (length, source) =
        tokio::time::timeout(Duration::from_secs(2), remote.recv_from(&mut query)).await??;
    assert_eq!(length, 48, "peer observed a non-header query length");
    let mut reply = [0; PACKET_BYTES];
    reply[0] = 0x24;
    reply[1] = 2;
    reply[24..32].copy_from_slice(&query[40..48]);
    reply[32..40].copy_from_slice(&1_u64.to_be_bytes());
    reply[40..48].copy_from_slice(&2_u64.to_be_bytes());
    if remote.send_to(&reply, source).await? != PACKET_BYTES {
        return Err(PilotError::InternalState.into());
    }
    Ok(())
}

/// Require no additional queued query after the product driver has returned.
/// # Errors
/// Preserve unexpected native observation failures for incomplete classification.
/// # Panics
/// Report only fixed text; opaque packet contents never enter assertion output.
fn assert_no_extra_query(remote: &tokio::net::UdpSocket) -> std::io::Result<()> {
    let mut bytes = [0; 49];
    let observed = remote.try_recv_from(&mut bytes);
    assert!(observed.is_err(), "unexpected extra acquisition query");
    match observed {
        Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => Ok(()),
        other => other.map(|_| ()),
    }
}

/// Exercise the actual driver with modeled prior send-attempt state and fresh I/O.
/// # Errors
/// Preserve actual peer/request setup, response and driver prerequisites.
/// # Panics
/// Require one new send at modeled63 and zero new sends at modeled64.
async fn send_cap_case(prior: u16) -> Result<(), Box<dyn Error>> {
    let remote = tokio::net::UdpSocket::bind(SocketAddr::from((Ipv4Addr::LOCALHOST, 0))).await?;
    let mut packet = [0; PACKET_BYTES];
    packet[0] = 0x23;
    packet[40..48].copy_from_slice(&[19; 8]);
    let destination = remote.local_addr()?;
    let request = prepare_slot(destination, packet);
    assert!(
        !matches!(&request, Err(PilotError::InvalidPeer)),
        "bound loopback send-cap preparation rejected its local endpoint"
    );
    let request = request?;
    let mut slot = LiveSlot::from_prepared(request, context())?;
    // Prior attempts are modeled, not claimed to be actual failed native sends.
    slot.send_attempts = prior;
    let deadline = Instant::now()
        .checked_add(Duration::from_secs(2))
        .ok_or(PilotError::DeadlineOverflow)?;
    let limits = FrameLimits::new(PACKET_BYTES, 0).map_err(|_| PilotError::WireSetup)?;
    let (_sender, mut control) = watch::channel(Control {
        stop: false,
        epoch: 0,
    });
    let slot = if prior == 63 {
        let ((slot, _), response) = tokio::join!(
            run_request(slot, &mut control, deadline, limits),
            reply_once(&remote)
        );
        response?;
        assert_eq!(
            slot.terminal().ok_or(PilotError::InternalState)?.status(),
            Status::Header
        );
        assert!(slot.sent, "valid boundary query was not sent");
        slot
    } else {
        let (slot, _) = run_request(slot, &mut control, deadline, limits).await;
        assert_eq!(
            slot.terminal().ok_or(PilotError::InternalState)?.status(),
            Status::Budget
        );
        assert!(
            !slot.sent,
            "exhausted send budget performed a native query send"
        );
        slot
    };
    assert_eq!(slot.send_attempts, 64);
    assert_no_extra_query(&remote)?;
    Ok(())
}

/// Actual single-slot driver respects the last legal native send-attempt boundary.
/// # Errors
/// Preserve actual fixture/native failures.
/// # Panics
/// Require valid modeled63/new64 success and modeled64/no65 refusal.
#[test]
fn single_slot_driver_enforces_send_cap_with_valid_boundary_neighbor() -> Result<(), Box<dyn Error>>
{
    if series_process::parent_or_child(
        "series::tests::single_slot_driver_enforces_send_cap_with_valid_boundary_neighbor",
    )? {
        return Ok(());
    }
    let runtime = runtime()?;
    // This separate peer/socket proves the send primitive's positive prerequisite;
    // it is dropped before either modeled driver case and is not a series row.
    drop(runtime.block_on(queued_native_reply())?);
    runtime.block_on(send_cap_case(63))?;
    runtime.block_on(send_cap_case(64))
}

/// A genuine already-selected datagram is charged after retirement without publication.
/// # Errors
/// Preserve actual selected-read/fixture failures.
/// # Panics
/// Require one actual new charge/rejection and an active neighboring header commit.
#[test]
fn selected_native_datagram_after_retirement_is_charged_without_publication()
-> Result<(), Box<dyn Error>> {
    let runtime = runtime()?;
    for retired in [false, true] {
        let SelectedReply {
            mut slot,
            selected,
            buffer,
            deadline,
        } = runtime.block_on(selected_native_reply())?;
        let state = Control {
            stop: retired,
            epoch: 0,
        };
        if retired {
            // Establish the handler's retired-entry precondition, not its publication guard.
            slot.finish(TerminalOutcome::Stopped);
        }
        let (sender, mut control) = watch::channel(state);
        let limits = FrameLimits::new(PACKET_BYTES, 0).map_err(|_| PilotError::WireSetup)?;
        let observed = handle_series_read(
            &mut slot,
            &mut control,
            selected,
            &buffer,
            limits,
            deadline,
            state,
        );
        assert_eq!(slot.received, 1);
        assert_eq!(observed.stop, retired);
        if retired {
            assert_eq!(slot.rejected, 1);
            assert_eq!(
                slot.terminal().ok_or(PilotError::InternalState)?.status(),
                Status::Stopped
            );
            assert!(slot.terminal().and_then(TerminalOutcome::facts).is_none());
            assert!(slot.last_rejection.is_none());
        } else {
            assert_eq!(slot.rejected, 0);
            assert_eq!(
                slot.terminal().ok_or(PilotError::InternalState)?.status(),
                Status::Header
            );
        }
        drop(sender);
    }
    runtime.block_on(modeled_read_boundary_neighbors())?;
    Ok(())
}

/// Supply explicitly modeled raw receive classifications with a genuine buffer origin.
/// # Errors
/// Preserve actual fixture send/read errors before introducing the modeled input.
/// # Panics
/// Require bounded charge/rejection without publishing any refused prefix.
async fn modeled_read_boundary_neighbors() -> Result<(), Box<dyn Error>> {
    // Keep the genuine selected datagram and original buffer as the fixture
    // origin, then explicitly model a malformed length or platform classification
    // at this private handler boundary. No kernel result or Windows run is claimed.
    for modeled_length in [Some(49), Some(50), Some(usize::MAX), None] {
        let SelectedReply {
            mut slot,
            mut selected,
            buffer,
            deadline,
        } = selected_native_reply().await?;
        selected.result = match modeled_length {
            Some(length) => NativeReadOutcome::Datagram {
                length,
                source: slot.peer,
            },
            None => classify_read_result(
                NativeDatagramPlatform::Windows,
                Err(std::io::Error::from_raw_os_error(10040)),
            ),
        };
        // Three prior consumed/rejected datagrams are modeled, not observed.
        // Starting above zero distinguishes both subtraction and multiplication.
        slot.received = 3;
        slot.rejected = 3;
        let state = Control {
            stop: false,
            epoch: 0,
        };
        let (sender, mut control) = watch::channel(state);
        let limits = FrameLimits::new(PACKET_BYTES, 0).map_err(|_| PilotError::WireSetup)?;
        let observed = handle_series_read(
            &mut slot,
            &mut control,
            selected,
            &buffer,
            limits,
            deadline,
            state,
        );
        if Instant::now() >= deadline {
            return Err(std::io::Error::other("modeled receive fixture deadline elapsed").into());
        }
        assert_eq!(slot.received, 4);
        assert_eq!(slot.rejected, 4);
        assert_eq!(slot.last_rejection, Some(Rejection::Length));
        assert!(
            !slot.done(),
            "refused prefix unexpectedly published a terminal result"
        );
        assert!(!observed.stop && observed.epoch == 0);
        drop(sender);
    }
    Ok(())
}

/// Deterministic preparation failures never retry or publish failed entropy bytes.
/// # Errors
/// Preserve valid fixture preparation/runtime setup failures.
/// # Panics
/// Require one failing provider call and zero entropy calls inside an entered runtime.
#[test]
fn entropy_failure_and_entered_runtime_precede_all_acquisition_work() -> Result<(), Box<dyn Error>>
{
    let remote = crate::StdSocket::bind(SocketAddr::from((Ipv4Addr::LOCALHOST, 0)))?;
    remote.set_nonblocking(true)?;
    let config = SeriesConfig::new(
        remote.local_addr()?,
        Duration::from_millis(100),
        Duration::from_secs(1),
    )?;
    let calls = std::cell::Cell::new(0);
    let failed = prepare_series_with_entropy(&config, |entropy| {
        calls.set(calls.get() + 1);
        entropy.fill(91);
        Err(PilotError::Entropy)
    });
    assert!(matches!(failed, Err(PilotError::Entropy)));
    assert_eq!(calls.get(), 1);
    let (prepared, handle) = prepare_series_with_entropy(&config, |entropy| {
        *entropy = [
            1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3,
        ];
        Ok(())
    })?;
    let runtime = runtime()?;
    let entered = runtime.enter();
    calls.set(0);
    assert!(matches!(
        prepare_series_with_entropy(&config, |_| {
            calls.set(calls.get() + 1);
            Err(PilotError::Entropy)
        }),
        Err(PilotError::EnteredRuntime)
    ));
    assert_eq!(calls.get(), 0);
    let schedule = ControlSchedule::new(None, None)?;
    assert!(matches!(
        execute_series(prepared, &schedule),
        Err(PilotError::EnteredRuntime)
    ));
    let mut bytes = [0; RECEIVE_BYTES];
    assert!(
        matches!(remote.recv_from(&mut bytes), Err(error) if error.kind() == std::io::ErrorKind::WouldBlock),
        "entered-runtime rejection unexpectedly sent a query"
    );
    drop((entered, handle));
    Ok(())
}

/// Inject one second setup failure after the actual first native completion.
/// Injected causes establish propagation/retention, not an OS/provider fault observation.
/// # Errors
/// Preserve real first acquisition, peer and checked preparation failures.
/// # Panics
/// Require the first record intact, the exact primary cause and no later send.
async fn setup_failure_case(
    cause: PilotError,
    prepared: PreparedSeries,
    handle: ControlHandle,
    remote: crate::StdSocket,
) -> Result<(), Box<dyn Error>> {
    let remote = tokio::net::UdpSocket::from_std(remote)?;
    let destination = prepared.peer;
    let origin = Instant::now();
    let receiver = SeriesReceiver::new(Arc::new(RunClockContext { origin }), destination, None);
    let deadline = origin
        .checked_add(Duration::from_secs(2))
        .ok_or(PilotError::DeadlineOverflow)?;
    let mut calls = 0;
    let mut setup = |request| {
        calls += 1;
        if calls == 2 {
            Err(cause)
        } else {
            setup_request(request)
        }
    };
    let (report, response) = tokio::join!(
        run_series(prepared, receiver, deadline, &mut setup),
        reply_once(&remote)
    );
    response?;
    assert_eq!(calls, 2);
    assert_eq!(report.end(), SeriesEnd::Failure);
    let primary = report.primary_failure();
    assert!(
        primary.is_some(),
        "injected setup failure lost its primary cause"
    );
    assert_eq!(
        primary.ok_or(PilotError::InternalState)?.code(),
        cause.as_str()
    );
    let native = report
        .primary_failure()
        .and_then(FailureSummary::native_cause);
    assert!(
        native.is_some(),
        "injected setup failure lost its native cause"
    );
    assert_eq!(
        native.ok_or(PilotError::InternalState)?.kind(),
        std::io::ErrorKind::PermissionDenied
    );
    let first = report.attempts()[0].report();
    assert!(
        first.is_some(),
        "later setup failure lost the first completed report"
    );
    let first = first.ok_or(PilotError::InternalState)?;
    assert_eq!(first.status(), Status::Header);
    let facts = first.facts();
    assert!(
        facts.is_some(),
        "first completed header lost its acquisition facts"
    );
    assert!(matches!(
        facts.ok_or(PilotError::InternalState)?.measurement(),
        Err(crate::MeasurementRefusal::Unavailable)
    ));
    for later in &report.attempts()[1..] {
        assert_eq!(
            later.not_started_cause(),
            Some(SeriesNotStartedCause::PrimaryFailure)
        );
    }
    assert_no_extra_query(&remote)?;
    drop(handle);
    Ok(())
}

/// Both bind and registration causes preserve an actual first completed observation.
/// # Errors
/// Preserve real acquisition and typed injected-error control failures.
/// # Panics
/// Require no retry or third setup/send after the injected second-operation failure.
#[test]
fn setup_and_registration_failures_preserve_actual_prior_completion() -> Result<(), Box<dyn Error>>
{
    if series_process::parent_or_child(
        "series::tests::setup_and_registration_failures_preserve_actual_prior_completion",
    )? {
        return Ok(());
    }
    let runtime = runtime()?;
    // Use a distinct peer/socket to establish the native-send prerequisite before
    // either whole-series scenario. Dropping it adds no row to those real runs.
    drop(runtime.block_on(queued_native_reply())?);
    for registration in [false, true] {
        let remote = crate::StdSocket::bind(SocketAddr::from((Ipv4Addr::LOCALHOST, 0)))?;
        remote.set_nonblocking(true)?;
        let config = SeriesConfig::new(
            remote.local_addr()?,
            Duration::from_secs(1),
            Duration::from_secs(2),
        );
        if config.is_err() {
            series_process::semantic_check_failed(
                "independently valid series configuration was refused",
            );
        }
        assert!(
            config.is_ok(),
            "independently valid series configuration was refused"
        );
        let config = config?;
        let preparation = prepare_series_with_entropy(&config, |entropy| {
            *entropy = [
                1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3,
            ];
            Ok(())
        });
        assert!(
            preparation.is_ok(),
            "independently valid fixed-entropy preparation was refused"
        );
        let (prepared, handle) = preparation?;
        let native = IoCause::capture(&std::io::Error::from(std::io::ErrorKind::PermissionDenied));
        let cause = if registration {
            PilotError::Registration(native)
        } else {
            PilotError::Bind(native)
        };
        runtime.block_on(setup_failure_case(cause, prepared, handle, remote))?;
    }
    Ok(())
}

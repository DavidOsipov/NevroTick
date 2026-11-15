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
    net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV6},
    sync::mpsc::{Receiver, RecvTimeoutError, SyncSender, TryRecvError},
    time::{Duration, Instant},
};

use crate::{
    Control, LiveSlot, NativeDatagramPlatform, PACKET_BYTES, Phase, RECEIVE_BYTES, RECEIVE_CAP,
    Rejection, RunClockContext, SoftwareClock, Status, TerminalOutcome, Wake,
};
use tokio::{net::UdpSocket, sync::watch};

type Check<T> = Result<T, Box<dyn Error>>;

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
/// Preserve the runtime builder's original setup error.
fn runtime() -> io::Result<tokio::runtime::Runtime> {
    tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .max_blocking_threads(2)
        .build()
}

/// Create two genuine managed sockets and one original invocation control owner.
/// # Errors
/// Preserve runtime registration, socket, bounds, policy, conversion and deadline errors.
fn io_fixture() -> Check<IoFixture> {
    let authority = isolated_authority(SourcePolicyLimits::new(
        2,
        2,
        Duration::from_secs(30),
        Duration::from_secs(30),
    )?)?;
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
    let read = SoftwareClock::capture();
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
            read,
        },
        &buffers,
        ntp_wire::FrameLimits::new(PACKET_BYTES, 0).map_err(|_| "fixture frame limit refused")?,
        fixture.deadline,
    );
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
        let send_coordinates = (sent.before.mono, sent.after.mono_after);
        let retained = facts(&fixture.binding)?;
        reply(&mut fixture, &packet, 0, UNKNOWN).await?;
        let slot = &fixture.slots[0];
        let Phase::Pending(saved) = &slot.phase else {
            return Err("unknown code consumed Pending".into());
        };
        assert_eq!(
            (saved.before.mono, saved.after.mono_after),
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

/// Reject an actual old reply after targeted source/endpoint re-enable.
/// # Errors
/// Preserve runtime, socket, timeout, policy, target and record failures.
/// # Panics
/// Fail if stale work publishes facts or changes the re-enabled policy.
#[test]
fn actual_old_reply_cannot_reenter_after_both_correlation_epochs_advance() -> Check<()> {
    runtime()?.block_on(async {
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
            SendBracket { before, after },
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
    let read = SoftwareClock::capture();
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
        read,
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
    let (queries, peer) = server_result??;
    assert!(contention_observed);
    assert_three_distinct_queries(&queries);
    peer.set_nonblocking(true)?;
    let mut extra = [0_u8; RECEIVE_BYTES];
    assert!(
        matches!(peer.recv_from(&mut extra), Err(error) if error.kind() == io::ErrorKind::WouldBlock)
    );
    assert_eq!(report.series().end(), crate::SeriesEnd::Completed);
    assert!(report.series().primary_failure().is_none());
    assert!(report.series().stop_cleanup_failure().is_none());
    assert!(report.series().invalidate_cleanup_failure().is_none());
    for attempt in report.series().attempts() {
        let record = attempt
            .report()
            .ok_or("actual query lacks its slot report")?;
        assert_eq!(record.status(), Status::Header);
        assert!(record.sent());
        assert_eq!(record.received(), 1);
        assert_eq!(record.stratum(), Some(2));
        assert!(record.facts().is_some());
    }
    Ok(())
}

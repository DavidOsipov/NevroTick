// SPDX-License-Identifier: GPL-2.0-only
//! Real ordered address acquisition through the existing service invocation.

use std::{
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    rc::Rc,
    sync::MutexGuard,
    time::{Duration, Instant},
};

use nevrotick_engine::{
    AcquisitionError, AddressAttempt, AddressRoundConfig, AddressRoundEnd, AddressRoundReport,
    BoundedAcquisitionError, SourceAddressSet, SourceConfigurationSnapshot, SourceHandle,
    SourcePolicyError, Status,
};
use windows_platform::{
    diagnostics::{ConsumerState, MemoryConsumer},
    service::ServiceState,
};

use super::{
    Admission, Check, EngineWorker, OwnerClock, PACKET_BYTES, PEER_RECEIVE, PEER_TRANSMIT, Pause,
    Rig, Sinks, TerminalTrace, WAIT, exclusive_case, fixture_sources, peer_has_no_datagram,
    receive_query, respond, source_and_peer,
};
use crate::engine_worker::round_operation_failed;
use crate::{FixtureError, error::WorkerError, host::InvocationPoll};

/// Keep the original ordered candidates under one finite two-second allowance.
/// # Errors
/// Preserve the real constructor's endpoint, count and duration refusals.
fn round_config(peers: &[SocketAddr]) -> Check<AddressRoundConfig> {
    Ok(AddressRoundConfig::new(
        peers,
        Duration::from_millis(300),
        Duration::from_secs(2),
    )?)
}

/// Borrow the genuine report from the worker's single typed publication slot.
/// # Errors
/// Preserve absent or failed acquisition as an explicit fixture failure.
fn report(rig: &Rig) -> Check<&AddressRoundReport> {
    rig.worker
        .round_result()
        .ok_or("missing original address-round result")?
        .as_ref()
        .map_err(|_| "actual address round failed before a report".into())
}

/// A same-address successor must refuse the originally selected startup revision.
/// # Errors
/// Preserve real service retirement and both peer observation failures.
/// # Panics
/// Fail if stale startup reaches Running, loses its cause or emits any datagram.
fn stale_startup(worker: EngineWorker, first: &Rc<UdpSocket>, second: &UdpSocket) -> Check {
    let mut rig: Rig = Rig::start(worker, Admission::Accept, first)?;
    rig.finish()?;
    assert!(!rig.states().contains(&ServiceState::Running));
    assert_eq!(rig.invocation.outstanding_work(), 0);
    assert!(matches!(
        rig.worker.round_result(),
        Some(Err(BoundedAcquisitionError::Acquisition(
            AcquisitionError::SourcePolicy(SourcePolicyError::ConfigurationChanged)
        )))
    ));
    assert_eq!(
        rig.invocation
            .take_receipt()
            .map(crate::host::HostReceipt::into_result),
        Some(Err(FixtureError::Worker(WorkerError::Preparation)))
    );
    assert!(peer_has_no_datagram(first)?);
    assert!(peer_has_no_datagram(second)?);
    Ok(())
}

/// Stop acknowledged with Running must prevent the new round from starting.
/// # Errors
/// Preserve checked worker creation, service drain and native queue observations.
/// # Panics
/// Fail if Stop starts acquisition, publishes a result or leaves a work permit.
fn stopped_startup(
    source: SourceHandle,
    selected: SourceConfigurationSnapshot,
    peers: &[SocketAddr],
    first: &Rc<UdpSocket>,
    second: &UdpSocket,
) -> Check {
    let worker: EngineWorker =
        EngineWorker::new_address_round(source, selected, round_config(peers)?)?;
    let mut rig: Rig = Rig::start(worker, Admission::StopAtRunning, first)?;
    rig.finish()?;
    assert!(rig.invocation.admission_closed());
    assert_eq!(rig.invocation.outstanding_work(), 0);
    assert!(rig.worker.result.get().is_none());
    assert_eq!(
        rig.invocation
            .take_receipt()
            .map(crate::host::HostReceipt::into_result),
        Some(Ok(()))
    );
    assert!(peer_has_no_datagram(first)?);
    assert!(peer_has_no_datagram(second)?);
    Ok(())
}

/// Stop before the original control publication must reach that same prepared round.
/// # Errors
/// Preserve worker construction, the bounded pause and actual joined retirement.
/// # Panics
/// Fail for premature release, a missing genuine Stopped report or any UDP send.
fn stopped_active(
    source: SourceHandle,
    selected: SourceConfigurationSnapshot,
    peers: &[SocketAddr],
    first: &Rc<UdpSocket>,
    second: &UdpSocket,
) -> Check {
    let mut worker: EngineWorker =
        EngineWorker::new_address_round(source, selected, round_config(peers)?)?;
    let (prepared, hook): (Pause, Box<dyn FnOnce() + Send>) = Pause::new();
    worker.hooks.before_control = Some(hook);
    let mut rig: Rig = Rig::start(worker, Admission::Accept, first)?;
    assert!(matches!(rig.step(), InvocationPoll::Pending(_)));
    prepared.wait()?;
    assert_eq!(rig.invocation.state(), Some(ServiceState::Running));
    assert!(rig.worker.result.get().is_none());
    assert!(
        rig.worker
            .gate
            .lock()
            .is_ok_and(|gate| gate.control.is_none())
    );
    assert!(peer_has_no_datagram(first)?);
    assert!(peer_has_no_datagram(second)?);

    rig.request_stop();
    assert!(matches!(rig.step(), InvocationPoll::Pending(_)));
    assert_eq!(rig.invocation.state(), Some(ServiceState::StopPending));
    assert_eq!(rig.invocation.outstanding_work(), 1);
    assert!(rig.invocation.take_receipt().is_none());
    assert!(
        rig.worker
            .gate
            .lock()
            .is_ok_and(|gate| gate.cancelled && gate.control.is_none())
    );
    assert!(peer_has_no_datagram(first)?);
    assert!(peer_has_no_datagram(second)?);
    prepared.release()?;
    rig.finish()?;

    let actual: &AddressRoundReport = report(&rig)?;
    assert_eq!(actual.round_end(), AddressRoundEnd::Stopped);
    assert_eq!(actual.end(), AddressRoundEnd::Stopped);
    assert!(actual.attempts().is_empty());
    assert!(actual.policy_refusals().is_empty());
    assert!(actual.primary_error().is_none());
    assert!(actual.cleanup_errors().iter().all(Option::is_none));
    assert!(actual.refresh().is_none());
    assert!(!actual.native_refresh_pending());
    assert_eq!(rig.invocation.outstanding_work(), 0);
    assert_eq!(
        rig.invocation
            .take_receipt()
            .map(crate::host::HostReceipt::into_result),
        Some(Ok(()))
    );
    assert!(peer_has_no_datagram(first)?);
    assert!(peer_has_no_datagram(second)?);
    Ok(())
}

/// Exhaust one real candidate, retain the second reply, then require physical join.
/// # Errors
/// Preserve source, UDP, original report, pause and service-retirement failures.
/// # Panics
/// Fail for premature traffic/completion, wrong order/provenance or extra sends.
fn fallback_and_join(
    source: SourceHandle,
    selected: &SourceConfigurationSnapshot,
    peers: &[SocketAddr],
    first: &Rc<UdpSocket>,
    second: &UdpSocket,
) -> Check {
    let mut worker: EngineWorker =
        EngineWorker::new_address_round(source, selected.clone(), round_config(peers)?)?;
    let (tail, hook): (Pause, Box<dyn FnOnce() + Send>) = Pause::new();
    worker.hooks.after_return = Some(hook);
    let mut rig: Rig = Rig::start(worker, Admission::Accept, first)?;
    assert!(peer_has_no_datagram(first)?);
    assert!(matches!(rig.step(), InvocationPoll::Pending(_)));
    let (first_query, _first_client): ([u8; PACKET_BYTES], SocketAddr) = receive_query(first)?;
    let (second_query, second_client): ([u8; PACKET_BYTES], SocketAddr) = receive_query(second)?;
    assert_ne!(&first_query[40..48], &second_query[40..48]);
    respond(second, &second_query, second_client)?;
    tail.wait()?;
    let actual: &AddressRoundReport = report(&rig)?;
    assert_eq!(actual.round_end(), AddressRoundEnd::Reply);
    assert_eq!(actual.end(), AddressRoundEnd::Reply);
    assert!(actual.primary_error().is_none());
    assert!(actual.cleanup_errors().iter().all(Option::is_none));
    let attempts: &[AddressAttempt] = actual.attempts();
    let [blackhole, reply] = attempts else {
        return Err("expected exactly the original two attempts".into());
    };
    assert_eq!(blackhole.peer(), peers[0]);
    assert_eq!(blackhole.report().status(), Status::Timeout);
    assert!(blackhole.report().sent());
    assert_eq!(reply.peer(), peers[1]);
    assert_eq!(reply.report().status(), Status::Header);
    let facts: &nevrotick_engine::TransportFacts = reply
        .report()
        .facts()
        .ok_or("missing genuine reply facts")?;
    assert!(
        facts
            .source_configuration()
            .is_some_and(|snapshot| snapshot.same_configuration(selected))
    );
    assert_eq!(facts.header().receive_timestamp().bits(), PEER_RECEIVE);
    assert_eq!(facts.header().transmit_timestamp().bits(), PEER_TRANSMIT);
    assert!(matches!(
        facts.measurement(),
        Err(nevrotick_engine::MeasurementRefusal::Unavailable)
    ));
    rig.request_stop();
    assert!(matches!(rig.step(), InvocationPoll::JoinTail(_)));
    assert_eq!(rig.invocation.outstanding_work(), 1);
    assert!(rig.invocation.take_receipt().is_none());
    tail.release()?;
    rig.finish()?;
    assert_eq!(rig.invocation.outstanding_work(), 0);
    assert_eq!(
        rig.invocation
            .take_receipt()
            .map(crate::host::HostReceipt::into_result),
        Some(Ok(()))
    );
    assert_eq!(report(&rig)?.round_end(), AddressRoundEnd::Reply);
    assert!(peer_has_no_datagram(first)?);
    assert!(peer_has_no_datagram(second)?);
    Ok(())
}

/// Reach actual ordered fallback through the original startup and Stop owner.
/// # Errors
/// Preserve policy installation, real loopback effects and worker retirement.
#[test]
fn managed_round_fallback_retains_source_and_join_with_stale_and_stopped_neighbors() -> Check {
    let _case: MutexGuard<'_, ()> = exclusive_case();
    let (source, first): (SourceHandle, Rc<UdpSocket>) = source_and_peer()?;
    let second: UdpSocket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0))?;
    second.set_read_timeout(Some(WAIT))?;
    let peers: [SocketAddr; 2] = [first.local_addr()?, second.local_addr()?];
    let original: SourceConfigurationSnapshot = fixture_sources()?
        .authority
        .install_source_configuration(&source, SourceAddressSet::new(&peers)?)?;
    let obsolete: EngineWorker =
        EngineWorker::new_address_round(source.clone(), original.clone(), round_config(&peers)?)?;
    let selected: SourceConfigurationSnapshot = fixture_sources()?
        .authority
        .replace_source_configuration(&original, SourceAddressSet::new(&peers)?)?;
    stale_startup(obsolete, &first, &second)?;
    stopped_startup(source.clone(), selected.clone(), &peers, &first, &second)?;
    stopped_active(source.clone(), selected.clone(), &peers, &first, &second)?;
    fallback_and_join(source, &selected, &peers, &first, &second)
}

/// A delayed host observation cannot renew the new round's original cutoff.
/// # Errors
/// Preserve checked source/worker creation and actual joined service retirement.
/// # Panics
/// Fail for a renewed deadline, packet, fabricated report or premature permit release.
#[test]
fn managed_round_expired_service_cutoff_refuses_without_entropy_or_traffic() -> Check {
    let _case: MutexGuard<'_, ()> = exclusive_case();
    let (source, peer): (SourceHandle, Rc<UdpSocket>) = source_and_peer()?;
    let endpoint: SocketAddr = peer.local_addr()?;
    let selected: SourceConfigurationSnapshot = fixture_sources()?
        .authority
        .install_source_configuration(&source, SourceAddressSet::new(&[endpoint])?)?;
    let mut worker: EngineWorker =
        EngineWorker::new_address_round(source, selected, round_config(&[endpoint])?)?;
    let (tail, hook): (Pause, Box<dyn FnOnce() + Send>) = Pause::new();
    worker.hooks.after_return = Some(hook);
    let observed: Instant = Instant::now()
        .checked_sub(Duration::from_secs(90))
        .ok_or("fixture cannot represent the original observation")?;
    let mut rig: Rig = Rig::start_observing_with_sinks(
        worker,
        Admission::Accept,
        &peer,
        TerminalTrace::Silent,
        OwnerClock(observed),
        Sinks::new(MemoryConsumer::new(ConsumerState::Available), None),
    )?;
    assert_ne!(rig.step(), InvocationPoll::Finished);
    tail.wait()?;
    assert!(matches!(
        rig.worker.round_result(),
        Some(Err(BoundedAcquisitionError::OwnerDeadlineElapsed))
    ));
    assert!(peer_has_no_datagram(&peer)?);
    assert!(
        rig.worker
            .gate
            .lock()
            .is_ok_and(|gate| gate.control.is_none())
    );
    assert!(matches!(rig.step(), InvocationPoll::JoinTail(_)));
    assert_eq!(rig.invocation.outstanding_work(), 1);
    assert!(rig.invocation.take_receipt().is_none());
    tail.release()?;
    rig.finish()?;
    assert_eq!(rig.invocation.outstanding_work(), 0);
    assert_eq!(
        rig.invocation
            .take_receipt()
            .map(crate::host::HostReceipt::into_result),
        Some(Err(FixtureError::Worker(WorkerError::Running)))
    );
    assert!(peer_has_no_datagram(&peer)?);
    Ok(())
}

/// Reported bounded dispositions do not imply authentication, success or a worker fault.
/// # Panics
/// Fail if a genuine stale/error decision is suppressed or an ordinary refusal is relabelled.
#[test]
fn round_health_keeps_bounded_dispositions_separate_from_operation_failures() {
    let cases: [(AddressRoundEnd, bool); 10] = [
        (AddressRoundEnd::SourceRestricted, false),
        (AddressRoundEnd::Policy, false),
        (AddressRoundEnd::Reply, false),
        (AddressRoundEnd::Control, false),
        (AddressRoundEnd::Exhausted, false),
        (AddressRoundEnd::Stopped, false),
        (AddressRoundEnd::Deadline, false),
        (AddressRoundEnd::SetupDeadline, false),
        (AddressRoundEnd::Stale, true),
        (AddressRoundEnd::Failure, true),
    ];
    for (end, expected) in cases {
        assert_eq!(round_operation_failed(end), expected);
    }
}

// SPDX-License-Identifier: GPL-2.0-only
//! Real ordered address acquisition through the existing service invocation.

use std::{
    cell::RefCell,
    net::{Ipv4Addr, SocketAddr, UdpSocket},
    rc::Rc,
    sync::MutexGuard,
    time::{Duration, Instant},
};

use nevrotick_engine::{
    AcquisitionError, AddressAttempt, AddressRoundConfig, AddressRoundEnd, AddressRoundReport,
    BoundedAcquisitionError, PilotError, SourceAddressSet, SourceConfigurationSnapshot,
    SourceHandle, SourcePolicyError, Status,
};
use windows_platform::{
    diagnostics::{ConsumerState, MemoryConsumer},
    service::{ServiceState, ServiceStatus},
};

use super::{
    Admission, Check, EngineWorker, OwnerClock, PACKET_BYTES, PEER_RECEIVE, PEER_TRANSMIT, Pause,
    Rig, Sinks, TerminalTrace, WAIT, exclusive_case, fixture_sources, peer_has_no_datagram,
    poison_control_gate, receive_query, respond, source_and_peer,
};
use crate::engine_worker::round_operation_failed;
use crate::{
    FixtureError,
    error::WorkerError,
    host::{Clock, InvocationPoll},
};

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

    assert!(matches!(rig.worker.round_result(), Some(Ok(_))));
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

/// Refused control publication must retire the prepared round without publishing a result.
/// # Errors
/// Preserve worker construction, injected gate failure, peer observation and joined retirement.
/// # Panics
/// Fail for premature release, any round result, traffic or the wrong terminal cause.
fn refused_active_publication(
    source: SourceHandle,
    selected: &SourceConfigurationSnapshot,
    peers: &[SocketAddr],
    first: &Rc<UdpSocket>,
    second: &UdpSocket,
) -> Check {
    let mut worker: EngineWorker =
        EngineWorker::new_address_round(source, selected.clone(), round_config(peers)?)?;
    let (prepared, hook): (Pause, Box<dyn FnOnce() + Send>) = Pause::new();
    worker.hooks.before_control = Some(hook);
    let (tail, hook): (Pause, Box<dyn FnOnce() + Send>) = Pause::new();
    worker.hooks.after_return = Some(hook);
    let mut rig: Rig = Rig::start(worker, Admission::Accept, first)?;
    // Drop both pause senders before Rig drains the worker on an early error.
    let (prepared, tail): (Pause, Pause) = (prepared, tail);
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

    poison_control_gate(&rig)?;
    prepared.release()?;
    tail.wait()?;
    let join: InvocationPoll = rig.step();
    let admission_closed: bool = rig.invocation.admission_closed();
    let charged: u32 = rig.invocation.outstanding_work();
    let premature_receipt: bool = rig.invocation.take_receipt().is_some();
    let stopped_before_join: bool = rig.states().contains(&ServiceState::Stopped);
    let published_before_join: bool = rig.worker.result.get().is_some();
    let first_quiet: bool = peer_has_no_datagram(first)?;
    let second_quiet: bool = peer_has_no_datagram(second)?;
    tail.release()?;
    rig.finish()?;

    assert!(matches!(join, InvocationPoll::JoinTail(_)));
    assert!(admission_closed);
    assert_eq!(charged, 1);
    assert!(!premature_receipt);
    assert!(!stopped_before_join);
    assert!(!published_before_join);
    assert!(first_quiet && second_quiet);
    assert_eq!(rig.invocation.outstanding_work(), 0);
    assert!(rig.worker.result.get().is_none());
    assert_eq!(
        rig.invocation
            .take_receipt()
            .map(crate::host::HostReceipt::into_result),
        Some(Err(FixtureError::Worker(WorkerError::Running)))
    );
    assert!(peer_has_no_datagram(first)?);
    assert!(peer_has_no_datagram(second)?);
    Ok(())
}

/// Replace the selected revision at the host's first observation after Running.
struct ReplaceAfterRunning<'a> {
    now: Instant,
    acknowledged: &'a RefCell<Vec<ServiceStatus>>,
    selected: &'a SourceConfigurationSnapshot,
    replacement: Option<Check<SourceConfigurationSnapshot>>,
}

impl Clock for ReplaceAfterRunning<'_> {
    /// Retain the actual issuer outcome once and return the original host time.
    fn now(&mut self) -> Instant {
        let running: bool = self
            .acknowledged
            .borrow()
            .last()
            .is_some_and(|status| status.state() == ServiceState::Running);
        if running && self.replacement.is_none() {
            self.replacement = Some(fixture_sources().and_then(|mut sources| {
                Ok(sources.authority.replace_source_configuration(
                    self.selected,
                    self.selected.addresses().clone(),
                )?)
            }));
        }
        self.now
    }
}

/// Preserve the Engine's preparation refusal when Running outlives a source revision.
/// # Errors
/// Preserve worker construction, actual revision replacement and joined retirement failures.
/// # Panics
/// Fail if the stale revision publishes a control, loses its typed cause or releases early.
fn replaced_after_running(
    source: SourceHandle,
    selected: &SourceConfigurationSnapshot,
    peers: &[SocketAddr],
    first: &Rc<UdpSocket>,
    second: &UdpSocket,
) -> Check<SourceConfigurationSnapshot> {
    let mut worker: EngineWorker =
        EngineWorker::new_address_round(source, selected.clone(), round_config(peers)?)?;
    let (tail, hook): (Pause, Box<dyn FnOnce() + Send>) = Pause::new();
    worker.hooks.after_return = Some(hook);
    let mut rig: Rig = Rig::start(worker, Admission::Accept, first)?;
    // Release the worker's pause before Rig drains on early exit.
    let return_tail: Pause = tail;
    let mut clock: ReplaceAfterRunning<'_> = ReplaceAfterRunning {
        now: rig.clock.0,
        acknowledged: &rig.acknowledged,
        selected,
        replacement: None,
    };
    let first_poll: InvocationPoll = rig.invocation.poll(&mut clock, &rig.stop);
    let replacement: SourceConfigurationSnapshot = clock
        .replacement
        .ok_or("host did not observe Running before activation")??;
    return_tail.wait()?;
    let join: InvocationPoll = rig.step();
    let charged: u32 = rig.invocation.outstanding_work();
    let admission_closed: bool = rig.invocation.admission_closed();
    let premature_receipt: bool = rig.invocation.take_receipt().is_some();
    let stopped_before_join: bool = rig.states().contains(&ServiceState::Stopped);
    let retained_driver_refusal: bool = matches!(
        rig.worker.round_result(),
        Some(Err(BoundedAcquisitionError::Acquisition(
            AcquisitionError::Driver(PilotError::SourcePolicy(
                SourcePolicyError::ConfigurationChanged
            ))
        )))
    );
    let no_control: bool = rig
        .worker
        .gate
        .lock()
        .is_ok_and(|gate| gate.control.is_none());
    let peers_quiet: bool = peer_has_no_datagram(first)? && peer_has_no_datagram(second)?;
    return_tail.release()?;
    rig.finish()?;

    assert!(
        replacement
            .source_identity()
            .same_source(selected.source_identity())
    );
    assert!(!replacement.same_configuration(selected));
    assert_eq!(replacement.addresses().len(), peers.len());
    assert!(
        peers
            .iter()
            .all(|peer| replacement.addresses().allows(*peer))
    );
    assert!(matches!(
        first_poll,
        InvocationPoll::Pending(_) | InvocationPoll::JoinTail(_)
    ));
    assert!(matches!(join, InvocationPoll::JoinTail(_)));
    assert!(admission_closed);
    assert_eq!(charged, 1);
    assert!(!premature_receipt && !stopped_before_join);
    assert!(no_control && peers_quiet);
    assert!(retained_driver_refusal);
    assert!(rig.worker.round_result().is_some());
    assert_eq!(rig.invocation.outstanding_work(), 0);
    assert_eq!(
        rig.invocation
            .take_receipt()
            .map(crate::host::HostReceipt::into_result),
        Some(Err(FixtureError::Worker(WorkerError::Running)))
    );
    assert!(peer_has_no_datagram(first)?);
    assert!(peer_has_no_datagram(second)?);
    Ok(replacement)
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
    refused_active_publication(source.clone(), &selected, &peers, &first, &second)?;
    let successor: SourceConfigurationSnapshot =
        replaced_after_running(source.clone(), &selected, &peers, &first, &second)?;
    fallback_and_join(source, &successor, &peers, &first, &second)
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

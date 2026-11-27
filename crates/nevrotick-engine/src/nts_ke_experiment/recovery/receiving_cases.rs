// SPDX-License-Identifier: GPL-2.0-only
//! Controlled supplier cases through the real managed join and historical report.

use std::num::NonZeroU64;
use std::sync::{Arc, OnceLock, Weak};
use std::time::{Duration, Instant};

use super::{
    Failure, Hooks, LoopbackSuccessor, PairConfigurations, PairLoopbackPeer, PairPeerJoin,
    ProcessAdmission, WorkerSlot, assert_pair_loopback_report, pair_loopback_authority,
    pair_loopback_config, pair_loopback_peer, pair_loopback_peers, spawn_pair_loopback_peer,
    submit_pair_loopback,
};
use crate::nts_ke_experiment::datagram::{ReceivedInvocation, tests::LoopbackPeerLeg};
use crate::nts_ke_experiment::job::{ManagedWorker, PreparedInvocation};
use crate::nts_ke_experiment::power_target::PowerTarget;
use crate::nts_ke_experiment::receiving::{
    FixtureMapping, FixtureMappingOwner, ManagedReceivingContext,
};
use crate::nts_ke_experiment::{
    Config, ManagedObservedJob, RunFailed, finish_observing, prepare_managed_observed,
    receive_managed_submission,
};
use crate::{
    ManagedProtectedReceipt, ManagedProtectedReport, ManagedProtectedTerminal,
    ProtectedReceiptAdmission, SourceConfigurationSnapshot, SourcePolicyAuthority,
};

/// The fixture independently selects a bounded uncertainty allowance.
const UNCERTAINTY_LIMIT: Duration = Duration::from_nanos(20);
/// Long enough for one or two jobs in the original loopback fixture.
const WINDOW: Duration = Duration::from_secs(10);

/// Inputs borrowed or owned by one actual worker dispatch and joined take.
struct ReceivingCase<'a> {
    /// Original managed worker and provider across permitted reissues.
    worker: &'a ManagedWorker,
    /// Actual live power binding until after the receiving decision.
    power: &'a PowerTarget,
    /// Original independent authenticated exchange peer.
    peer: &'a PairLoopbackPeer,
    /// Current controlled mapping issuer whose revision is bound before submission.
    mapping: &'a Arc<FixtureMappingOwner>,
    /// Exclusive cutoff shared by the peer and the actual prepared job.
    cutoff: Instant,
    /// Optional controlled monotonic coordinate for the guarded final sample.
    receiving_at: Option<Instant>,
    /// Select another genuine prepared invocation for the identity control.
    foreign_context: Option<ManagedReceivingContext>,
    /// Optional change delivered after packet commit, before worker retirement.
    hooks: Hooks,
}

/// Drive real original peer packets, worker retirement, joined take and report.
/// # Errors
/// Preserve the actual source, preparation, peer or receiving fixture failure.
fn receive_originals(
    authority: &SourcePolicyAuthority,
    admission: &'static ProcessAdmission,
    case: ReceivingCase<'_>,
) -> Result<ManagedProtectedReport, Failure> {
    let ReceivingCase {
        worker,
        power,
        peer,
        mapping,
        cutoff,
        receiving_at,
        foreign_context,
        hooks,
    }: ReceivingCase<'_> = case;
    let config: Config = pair_loopback_config(authority, &peer.expected)?;
    let ManagedObservedJob {
        reservation,
        active,
        stop,
        receiving,
    }: ManagedObservedJob<'_> = prepare_managed_observed(config, Some(cutoff), worker, power)
        .map_err(|_| Failure::Control)?;
    let receiving: ManagedReceivingContext = foreign_context
        .unwrap_or(receiving)
        .with_fixture_mapping(mapping);
    let receiving: ManagedReceivingContext = match receiving_at {
        Some(now) => receiving.with_observed_at(now),
        None => receiving,
    };
    std::thread::scope(|scope| {
        let peer_job: PairPeerJoin<'_> =
            spawn_pair_loopback_peer(scope, &peer.socket, LoopbackSuccessor::ApplyOrdinary)?;
        let result: Result<ReceivedInvocation, RunFailed> = receive_managed_submission(
            || {
                submit_pair_loopback(
                    reservation,
                    admission,
                    Arc::new(OnceLock::<Weak<()>>::new()),
                    cutoff,
                    peer.remote,
                    hooks,
                )
            },
            &receiving,
            #[cfg(windows)]
            &mut None,
        );
        let report: ManagedProtectedReport = ManagedProtectedReport::from_observed(Ok(
            finish_observing(result, &stop, Some(active), || Ok(())),
        ))
        .map_err(|_| Failure::Control)?;
        let peer_history: [LoopbackPeerLeg<Instant>; 2] =
            peer_job.join().map_err(|_| Failure::Control)??;
        assert_pair_loopback_report(&report, &peer.expected, cutoff, peer_history)?;
        Ok(report)
    })
}

/// Independently observe one decision for each actual joined ordinary receipt.
/// # Errors
/// Refuse missing first or ordinary terminal history.
/// # Panics
/// Fail when either original or repeated reporting differs from the selected outcome.
fn assert_admissions(
    report: &ManagedProtectedReport,
    expected: ProtectedReceiptAdmission,
) -> Result<(), Failure> {
    let first: &ManagedProtectedReceipt = report.first().ok_or(Failure::Control)?;
    let ManagedProtectedTerminal::Ordinary(terminal) = report.terminal() else {
        return Err(Failure::Control);
    };
    assert_eq!([first.admission(), terminal.admission()], [expected; 2]);
    assert!(!first.same_exchange(terminal));
    assert_eq!([first.admission(), terminal.admission()], [expected; 2]);
    Ok(())
}

/// Withdraw the first lease, reissue the same evidence and acquire genuinely new receipts.
/// # Errors
/// Preserve equivalent failures from the actual controlled worker and peer.
/// # Panics
/// Fail if reissue revives old receipts or refuses the fresh ones.
#[test]
fn managed_joined_receiving_requires_new_receipts_after_mapping_reissue() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let worker: ManagedWorker = ManagedWorker::new(&SLOT);
    let power: PowerTarget = PowerTarget::new();
    let mut authority: SourcePolicyAuthority = pair_loopback_authority()?;
    let [first, fresh]: [PairLoopbackPeer; 2] =
        pair_loopback_peers(&mut authority, PairConfigurations::Shared)?;
    let start: Instant = Instant::now();
    let cutoff: Instant = start.checked_add(WINDOW).ok_or(Failure::Deadline)?;
    let definition: SourceConfigurationSnapshot = first.expected.clone();
    let evidence = move || {
        FixtureMapping::new(
            definition.clone(),
            start..=cutoff,
            UNCERTAINTY_LIMIT,
            UNCERTAINTY_LIMIT,
            WINDOW,
        )
    };
    let mapping: Arc<FixtureMappingOwner> =
        Arc::new(FixtureMappingOwner::new(NonZeroU64::MIN, evidence()));
    let replaced: Arc<FixtureMappingOwner> = Arc::clone(&mapping);
    let mut hooks: Hooks = Hooks::none();
    hooks.before_retire = Some(Box::new(move || {
        assert_eq!(replaced.replace(None), Ok(()));
        assert_eq!(replaced.replace(Some(evidence())), Ok(()));
    }));
    let old: ManagedProtectedReport = receive_originals(
        &authority,
        &ADMISSION,
        ReceivingCase {
            worker: &worker,
            power: &power,
            peer: &first,
            mapping: &mapping,
            cutoff,
            receiving_at: None,
            foreign_context: None,
            hooks,
        },
    )?;
    assert_admissions(&old, ProtectedReceiptAdmission::MappingWithdrawn)?;
    let new: ManagedProtectedReport = receive_originals(
        &authority,
        &ADMISSION,
        ReceivingCase {
            worker: &worker,
            power: &power,
            peer: &fresh,
            mapping: &mapping,
            cutoff,
            receiving_at: None,
            foreign_context: None,
            hooks: Hooks::none(),
        },
    )?;
    assert_admissions(&new, ProtectedReceiptAdmission::FixtureHandoff)?;
    assert_admissions(&old, ProtectedReceiptAdmission::MappingWithdrawn)?;
    let old_first: &ManagedProtectedReceipt = old.first().ok_or(Failure::Control)?;
    let new_first: &ManagedProtectedReceipt = new.first().ok_or(Failure::Control)?;
    assert!(
        old_first
            .configuration()
            .same_configuration(new_first.configuration())
    );
    assert!(!old_first.same_invocation(new_first));
    assert!(!old_first.same_exchange(new_first));
    assert!(old_first.committed_at() <= new_first.committed_at());
    Ok(())
}

/// Each case changes one input while the independent UDP observations stay ordinary.
#[derive(Clone, Copy)]
enum ReceivingFault {
    /// Same-shaped evidence from another actual source slot.
    Foreign,
    /// Context from another genuinely prepared job using the exact same definition.
    ForeignInvocation,
    /// The validity interval starts at the original exclusive cutoff.
    Validity,
    /// Error one nanosecond above the independent fixture limit.
    Uncertainty,
    /// Joined reading at the original exclusive deadline.
    Late,
    /// The original packet acceptance lies outside the selected age allowance.
    Age,
    /// Exhaustion during eviction permanently withdraws a previously issued revision.
    Exhausted,
    /// Healthy neighboring controlled handoff.
    Healthy,
}

/// Bind the selected single difference and check the receipt's actual disposition.
/// # Errors
/// Preserve actual controlled worker and configuration failures.
/// # Panics
/// Fail if the reached receiving guard differs from the independent expected outcome.
fn check_receiving_fault(
    fault: ReceivingFault,
    slot: &'static WorkerSlot,
    admission: &'static ProcessAdmission,
) -> Result<(), Failure> {
    let worker: ManagedWorker = ManagedWorker::new(slot);
    let power: PowerTarget = PowerTarget::new();
    let mut authority: SourcePolicyAuthority = pair_loopback_authority()?;
    let peer: PairLoopbackPeer = pair_loopback_peer(&mut authority, 0)?;
    let mapping_source: SourceConfigurationSnapshot = if matches!(fault, ReceivingFault::Foreign) {
        pair_loopback_peer(&mut authority, 1)?.expected
    } else {
        peer.expected.clone()
    };
    let start: Instant = Instant::now();
    let cutoff: Instant = start.checked_add(WINDOW).ok_or(Failure::Deadline)?;
    let foreign_context: Option<ManagedReceivingContext> =
        if matches!(fault, ReceivingFault::ForeignInvocation) {
            let foreign: PreparedInvocation = PreparedInvocation::prepare_with_cutoff(
                pair_loopback_config(&authority, &peer.expected)?,
                Some(cutoff),
            )
            .map_err(|_| Failure::Control)?;
            Some(foreign.managed_receiving().ok_or(Failure::Control)?)
        } else {
            None
        };
    let mapping_start: Instant = if matches!(fault, ReceivingFault::Validity) {
        cutoff
    } else {
        start
    };
    let uncertainty: Duration = if matches!(fault, ReceivingFault::Uncertainty) {
        Duration::from_nanos(21)
    } else {
        UNCERTAINTY_LIMIT
    };
    let age_limit: Duration = if matches!(fault, ReceivingFault::Age) {
        Duration::ZERO
    } else {
        WINDOW
    };
    let age_read: Instant = cutoff
        .checked_sub(Duration::from_nanos(1))
        .ok_or(Failure::Deadline)?;
    let (receiving_at, expected): (Option<Instant>, ProtectedReceiptAdmission) = match fault {
        ReceivingFault::Foreign => (None, ProtectedReceiptAdmission::ForeignMapping),
        ReceivingFault::ForeignInvocation => (None, ProtectedReceiptAdmission::ForeignReceipt),
        ReceivingFault::Validity => (None, ProtectedReceiptAdmission::MappingValidity),
        ReceivingFault::Uncertainty => (None, ProtectedReceiptAdmission::MappingUncertainty),
        ReceivingFault::Late => (Some(cutoff), ProtectedReceiptAdmission::Deadline),
        ReceivingFault::Age => (Some(age_read), ProtectedReceiptAdmission::ReceiptAge),
        ReceivingFault::Exhausted => (None, ProtectedReceiptAdmission::MappingWithdrawn),
        ReceivingFault::Healthy => (None, ProtectedReceiptAdmission::FixtureHandoff),
    };
    let revision: NonZeroU64 = if matches!(fault, ReceivingFault::Exhausted) {
        NonZeroU64::MAX
    } else {
        NonZeroU64::MIN
    };
    let mapping: Arc<FixtureMappingOwner> = Arc::new(FixtureMappingOwner::new(
        revision,
        FixtureMapping::new(
            mapping_source,
            mapping_start..=cutoff,
            uncertainty,
            UNCERTAINTY_LIMIT,
            age_limit,
        ),
    ));
    let hooks: Hooks = if matches!(fault, ReceivingFault::Exhausted) {
        exhaust_before_retirement(&mapping, peer.expected.clone(), start, cutoff)
    } else {
        Hooks::none()
    };
    let report: ManagedProtectedReport = receive_originals(
        &authority,
        admission,
        ReceivingCase {
            worker: &worker,
            power: &power,
            peer: &peer,
            mapping: &mapping,
            cutoff,
            receiving_at,
            foreign_context,
            hooks,
        },
    )?;
    assert_admissions(&report, expected)
}

/// Withdraw the last revision after transport commit and challenge sticky exhaustion.
/// # Panics
/// The hook fails if the exhausted owner permits withdrawal with a new revision or reissue.
fn exhaust_before_retirement(
    mapping: &Arc<FixtureMappingOwner>,
    attempted_reissue: SourceConfigurationSnapshot,
    start: Instant,
    cutoff: Instant,
) -> Hooks {
    let closed: Arc<FixtureMappingOwner> = Arc::clone(mapping);
    let mut hooks: Hooks = Hooks::none();
    hooks.before_retire = Some(Box::new(move || {
        assert_eq!(
            closed.replace(None),
            Err(ProtectedReceiptAdmission::MappingWithdrawn)
        );
        let new: FixtureMapping = FixtureMapping::new(
            attempted_reissue,
            start..=cutoff,
            UNCERTAINTY_LIMIT,
            UNCERTAINTY_LIMIT,
            WINDOW,
        );
        assert_eq!(
            closed.replace(Some(new)),
            Err(ProtectedReceiptAdmission::MappingWithdrawn)
        );
    }));
    hooks
}

/// Exercise changed authority, validity, error, time and revision cases through actual receiving.
/// # Errors
/// Preserve the first failing genuine join or peer observation.
#[test]
fn managed_joined_receiving_checks_mapping_bounds_and_sticky_revision_exhaustion()
-> Result<(), Failure> {
    static SLOTS: [WorkerSlot; 8] = [const { WorkerSlot::new() }; 8];
    static ADMISSIONS: [ProcessAdmission; 8] = [const { ProcessAdmission::new() }; 8];
    let cases: [ReceivingFault; 8] = [
        ReceivingFault::Foreign,
        ReceivingFault::ForeignInvocation,
        ReceivingFault::Validity,
        ReceivingFault::Uncertainty,
        ReceivingFault::Late,
        ReceivingFault::Age,
        ReceivingFault::Exhausted,
        ReceivingFault::Healthy,
    ];
    for ((fault, slot), admission) in cases.into_iter().zip(&SLOTS).zip(&ADMISSIONS) {
        check_receiving_fault(fault, slot, admission)?;
    }
    Ok(())
}

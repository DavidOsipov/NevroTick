// SPDX-License-Identifier: GPL-2.0-only
//! Source-issued KE controls through the actual worker and joined receiver.
//!
//! Public fixed component material establishes no TLS, certificate, exporter or
//! Windows-provider success. Protected controls use the closed datagram backend.

use std::ffi::OsString;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::num::NonZeroU64;
use std::sync::mpsc::{Receiver, SyncSender, TrySendError, sync_channel};
use std::sync::{Arc, Weak};
use std::time::{Duration, Instant};
use zeroize::Zeroizing;

use super::association::ledger::{RequestInput, RequestPhase, Status, Ticket};
use super::association::{AssociationCandidate, AssociationOwner};
use super::datagram::tests::{managed_component_exchange, managed_component_keys};
use super::datagram::{ProtectedCompletion, ReceivedCompletion};
use super::handoff::ManagedPreviewHistory;
use super::job::{Hooks, Observation, Operation, PreparedInvocation, Termination, WorkerSlot};
use super::managed_preview::SourceProtectedProbeFailure;
use super::process::{ProcessAdmission, ProcessClaim, ProviderScope};
use super::profile::StrictProfile;
use super::recovery::{End, Invocation, Producer, Snapshot, Step, step_with};
use super::request::DirectionalKeys;
use super::{Config, Control, Failed, Failure, RunFailed, Summary, output, run_synchronously_with};
use crate::source_policy::{FixtureRequest, SourceBinding, fixture_control, fixture_send};
use crate::{
    KePreviewTarget, KePreviewTargetError, SourceConfigurationSnapshot, SourceHandle,
    SourcePolicyAuthority, SourcePolicyError, SourcePolicyLimits, SourcePolicySnapshot,
};

/// Finite component observation only; it supplies no product operation deadline.
const WAIT: Duration = Duration::from_secs(2);

/// Shared UDP peer in the fixed component material.
const COMPONENT_UDP: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 123));

/// Configured neighbor with no outstanding request or retained endpoint slot.
const ALTERNATE_UDP: SocketAddr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 124));

/// Select a fault in component work, never source identity or permission.
#[derive(Clone, Copy)]
enum Scenario {
    /// Complete under the genuine unchanged definition.
    Healthy,
    /// Replace the actual issued revision while entropy is outside all guards.
    ReplaceDuringEntropy,
    /// Replace after complete material preparation, before the real publisher.
    ReplaceBeforePublication,
    /// Replace after the successful commit, before worker publication and collection.
    ReplaceAfterCommit,
    /// Offer an otherwise valid endpoint outside the installed UDP set.
    OutsideEndpoint,
    /// Replace a protected source just before the actual send.
    ProtectedBeforeSend,
    /// Replace a protected source after actual exchange retirement.
    ProtectedBeforeReplyCommit,
    /// Replace a protected source after commit, before joined reconstruction.
    ProtectedAfterCommit,
}

impl Scenario {
    /// The original fixture retains a genuine Preview unless explicitly protected.
    const fn protected(self) -> bool {
        matches!(
            self,
            Self::ProtectedBeforeSend
                | Self::ProtectedBeforeReplyCommit
                | Self::ProtectedAfterCommit
        )
    }
}

/// Actual checked source owner and definition before preparation.
struct Issued {
    /// Non-clonable authority used by the same real configuration transaction.
    authority: SourcePolicyAuthority,
    /// Actual catalogue handle; it alone captures the managed Preview context.
    source: SourceHandle,
    /// Original immutable issuer snapshot, used only as historical expectation.
    configuration: SourceConfigurationSnapshot,
    /// Genuine checked TLS target reused by equal-value replacement.
    target: KePreviewTarget,
    /// Sole configured UDP endpoint; Preview must not create its endpoint slot.
    allowed: SocketAddr,
}

/// Admit literal loopback component inputs through the real checked issuers.
/// # Errors
/// Preserve source/configuration refusal or the bounded invalid-target category.
fn issued(scenario: Scenario) -> Result<Issued, Failure> {
    let limits: SourcePolicyLimits =
        SourcePolicyLimits::new(1, 1, Duration::from_millis(1), Duration::from_millis(4))
            .map_err(Failure::SourcePolicy)?;
    let mut authority: SourcePolicyAuthority =
        crate::source_policy::isolated_authority(limits).map_err(Failure::SourcePolicy)?;
    let source: SourceHandle = authority.source(0).map_err(Failure::SourcePolicy)?;
    let tls: SocketAddr = SocketAddr::from((Ipv4Addr::LOCALHOST, 4460));
    let target: KePreviewTarget = KePreviewTarget::new(tls, "Time.Example")
        .map_err(|_: KePreviewTargetError| Failure::Arguments)?;
    let allowed: SocketAddr = SocketAddr::from((
        Ipv4Addr::LOCALHOST,
        if matches!(scenario, Scenario::OutsideEndpoint) {
            124_u16
        } else {
            123_u16
        },
    ));
    let addresses: crate::SourceAddressSet =
        crate::SourceAddressSet::new(&[allowed]).map_err(Failure::SourcePolicy)?;
    let configuration: SourceConfigurationSnapshot = authority
        .install_ke_preview_configuration(&source, addresses, target.clone())
        .map_err(Failure::SourcePolicy)?;
    Ok(Issued {
        authority,
        source,
        configuration,
        target,
        allowed,
    })
}

/// One actual component producer; every context still comes from the issuer.
struct PreviewProducer {
    /// Real authority moved to the worker for the entropy-boundary replacement.
    authority: SourcePolicyAuthority,
    /// Actual initial revision against which replacement is conditional.
    configuration: SourceConfigurationSnapshot,
    /// Checked original target; no caller-selected TLS override enters Config.
    target: KePreviewTarget,
    /// Explicit test action, with no authority-bearing data.
    scenario: Scenario,
    /// Genuine newer snapshot returned by the actual replacement transaction.
    replacement: Option<SourceConfigurationSnapshot>,
    /// Actual establishment/entropy/protected-operation/jitter entries.
    calls: [usize; 4],
}

impl PreviewProducer {
    /// Replace the actual definition with equal target/address values once.
    /// # Errors
    /// Preserve genuine stale/foreign, contention or exhaustion refusal.
    /// # Panics
    /// Reject an issuer that fails to advance the genuine configuration identity.
    fn replace_definition(&mut self) -> Result<(), Failure> {
        let newer: SourceConfigurationSnapshot = self
            .authority
            .replace_ke_preview_configuration(
                &self.configuration,
                self.configuration.addresses().clone(),
                self.target.clone(),
            )
            .map_err(Failure::SourcePolicy)?;
        assert!(!newer.same_configuration(&self.configuration));
        self.replacement = Some(newer);
        Ok(())
    }
}

impl Producer for PreviewProducer {
    fn establish<'tx>(
        &mut self,
        config: &'tx Config,
        control: &'tx Control,
        provider: &'tx ProviderScope<'tx>,
    ) -> Result<AssociationOwner<'tx>, Failed> {
        self.calls[0] += 1;
        assert_eq!(config.peer, self.target.peer());
        assert_eq!(config.identity, self.target.identity());
        let candidate: AssociationCandidate<'tx> =
            component_candidate(config, control, provider, COMPONENT_UDP);
        if matches!(self.scenario, Scenario::ReplaceBeforePublication) {
            self.replace_definition().map_err(Failed::from)?;
        }
        AssociationOwner::receive(config, control, provider, candidate).map_err(Failed::from)
    }

    fn exchange<'tx>(
        &mut self,
        association: &mut AssociationOwner<'tx>,
        control: &'tx Control,
    ) -> Result<ProtectedCompletion<'tx>, Failure> {
        self.calls[2] += 1;
        if !self.scenario.protected() {
            return Err(Failure::Control);
        }
        let completion: ProtectedCompletion<'tx> =
            managed_component_exchange(association, control, || {
                if matches!(self.scenario, Scenario::ProtectedBeforeSend) {
                    self.replace_definition()?;
                }
                Ok(())
            })?;
        assert_eq!(association.cookie_population()?, (0, 0));
        if matches!(self.scenario, Scenario::ProtectedBeforeReplyCommit) {
            self.replace_definition()?;
        }
        Ok(completion)
    }

    fn preview_entropy(&mut self, output: &mut [u8; 48]) -> Result<(), Failure> {
        self.calls[1] += 1;
        if matches!(self.scenario, Scenario::ReplaceDuringEntropy) {
            self.replace_definition()?;
        }
        output.fill(0x42_u8);
        Ok(())
    }

    fn jitter(&mut self, _upper: Duration) -> Result<Duration, Failure> {
        self.calls[3] += 1;
        Err(Failure::Control)
    }
}

/// Supply fixed public component material to the same actual receiving constructor.
/// The candidate claims no TLS or exporter qualification and retains all real owners.
fn component_candidate<'a>(
    config: &'a Config,
    control: &'a Control,
    provider: &'a ProviderScope<'a>,
    udp: SocketAddr,
) -> AssociationCandidate<'a> {
    let profile: StrictProfile = StrictProfile::component();
    let keys: DirectionalKeys = if matches!(&config.mode, super::ExchangeMode::ManagedProtected(_))
    {
        managed_component_keys()
    } else {
        DirectionalKeys::new(Zeroizing::new([0x11_u8; 32]), Zeroizing::new([0x22_u8; 32]))
    };
    let cookies: Vec<Zeroizing<Vec<u8>>> = vec![Zeroizing::new(vec![0xa5_u8; 12])];
    AssociationCandidate::new(config, control, provider, udp, profile, keys, cookies)
}

/// Publish one genuine component owner for direct guarded-entry controls.
/// # Errors
/// Preserve actual control/provider/publication and exercised consumer refusals.
fn with_owner(
    config: &Config,
    udp: SocketAddr,
    exercise: impl FnOnce(&mut AssociationOwner<'_>, &Control) -> Result<(), Failure>,
) -> Result<(), Failure> {
    let control: Control = Control::new(Instant::now())?;
    let admission: ProcessAdmission = ProcessAdmission::new();
    let claim: ProcessClaim<'_> = admission.claim()?;
    let provider: ProviderScope<'_> = ProviderScope::component(claim);
    let candidate: AssociationCandidate<'_> = component_candidate(config, &control, &provider, udp);
    let mut owner: AssociationOwner<'_> =
        AssociationOwner::receive(config, &control, &provider, candidate)?;
    exercise(&mut owner, &control)
}

/// Legal input neighbors cover both direct protected reservation representations.
/// # Errors
/// Preserve maintained timestamp decoding or the checked nonzero wire-token refusal.
fn protected_inputs() -> Result<[RequestInput; 2], Failure> {
    let header: ntp_wire::RawHeader =
        ntp_wire::decode_fixed_header(&[0_u8; 48]).map_err(|_| Failure::Control)?;
    let wire: NonZeroU64 = NonZeroU64::new(0x1122_3344_5566_7788).ok_or(Failure::Control)?;
    Ok([
        RequestInput::Observed,
        RequestInput::Packet {
            wire,
            local_t1: header.transmit_timestamp(),
        },
    ])
}

/// Both diagnostic modes retain their existing legal adapter/ledger entries.
/// # Errors
/// Preserve actual admission, publication, adapter context and reservation refusal.
/// # Panics
/// Reject a blanket mode refusal or incorrect original protected checkout accounting.
fn assert_protected_neighbor(config: &Config) -> Result<(), Failure> {
    let inputs: [RequestInput; 2] = protected_inputs()?;
    for input in inputs {
        with_owner(
            config,
            COMPONENT_UDP,
            |owner: &mut AssociationOwner<'_>, control: &Control| {
                let before: Status = owner.status();
                let expected: SocketAddr = SocketAddr::from((Ipv4Addr::LOCALHOST, 123));
                assert_eq!(owner.check_context(control)?, expected);
                assert_eq!(owner.status(), before);
                let _ticket: Ticket<'_> = owner.reserve(input)?;
                let after: Status = owner.status();
                assert_eq!(after.phase, RequestPhase::Reserved);
                assert_eq!(
                    (
                        after.attempts,
                        after.frames,
                        after.bytes,
                        after.available,
                        after.used,
                        after.uids
                    ),
                    (31, 32, 131_072, 0, 0, 0)
                );
                Ok(())
            },
        )?;
    }
    Ok(())
}

/// Post-step observations sent before the real worker records and retires its result.
struct Work {
    /// Original immutable invocation coordinates and initially unspent counts.
    before: Snapshot,
    /// The same invocation after its one actual transaction.
    after: Snapshot,
    /// Original complete failure retained by the invocation, if any.
    last_failure: Option<Failed>,
    /// Actual producer entry counts, including forbidden UDP/jitter controls.
    calls: [usize; 4],
    /// Actual replacement result, never a fabricated revision label.
    replacement: Option<SourceConfigurationSnapshot>,
    /// Weak observation of this actual component provider's destruction.
    provider_lifetime: Weak<()>,
}

/// Use the existing component provider without invoking Windows/CNG effects.
/// # Errors
/// This factory is infallible but retains the real worker's fallible signature.
#[expect(
    clippy::unnecessary_wraps,
    reason = "The actual worker accepts its existing fallible provider-factory type."
)]
fn component_provider(claim: ProcessClaim<'_>) -> Result<ProviderScope<'_>, Failure> {
    Ok(ProviderScope::component(claim))
}

/// Execute the production invocation/association/sealing machinery on the real worker.
/// # Errors
/// Preserve genuine primary/cleanup results; observation failure is a fixture refusal.
/// # Panics
/// Reject a reopened terminal invocation or changed original work accounting.
fn component_operation<'scope>(
    config: &'scope Config,
    provider: &'scope ProviderScope<'scope>,
    control: Control,
    mut producer: PreviewProducer,
    observed: &SyncSender<Work>,
) -> Operation {
    let mut invocation: Invocation<'scope> =
        Invocation::one_shot(config, provider, control).map_err(Failed::from)?;
    let before: Snapshot = invocation.snapshot();
    let provider_lifetime: Weak<()> = provider.retirement_witness();
    let outcome: Result<Step, Failed> = step_with(&mut invocation, &mut producer);
    if matches!(
        producer.scenario,
        Scenario::ReplaceAfterCommit | Scenario::ProtectedAfterCommit
    ) && matches!(&outcome, Ok(Step::Complete(_)))
    {
        producer.replace_definition().map_err(Failed::from)?;
    }
    let after: Snapshot = invocation.snapshot();
    let last_failure: Option<Failed> = invocation.last_failure().copied();
    let terminal: End = after.end.ok_or_else(|| Failed::from(Failure::Control))?;
    let closed: Result<Step, Failed> = step_with(&mut invocation, &mut producer);
    assert!(matches!(closed, Ok(Step::Closed(end)) if end == terminal));
    assert_eq!(invocation.snapshot(), after);
    drop(invocation);
    let operation: Operation = match outcome {
        Ok(Step::Complete(completion)) => Ok(completion),
        Err(failure) => Err(failure),
        Ok(Step::Closed(_) | Step::Deferred(_)) => Err(Failed::from(Failure::Control)),
    };
    let work: Work = Work {
        before,
        after,
        last_failure,
        calls: producer.calls,
        replacement: producer.replacement,
        provider_lifetime,
    };
    observed
        .try_send(work)
        .map_err(|_: TrySendError<Work>| Failed::from(Failure::Control))?;
    operation
}

/// Joined result coupled to independent original-context expectations.
struct Run {
    /// Actual joined receiver result including a possible abnormal termination.
    outcome: Result<ReceivedCompletion, RunFailed>,
    /// Actual completed invocation observations and returned issuer.
    work: Work,
    /// Original source handle permits read-only state checks after real retirement.
    source: SourceHandle,
    /// Original configuration is historical expectation, never fresh permission.
    expected: SourceConfigurationSnapshot,
}

/// Drain unexpected real work before failing a facade-custody assertion.
/// # Errors
/// Preserve unavailable actual collection; never infer completion from observation.
fn drain(slot: &'static WorkerSlot, observation: Observation) -> Result<(), Failure> {
    match observation {
        Observation::Running(_) => {
            slot.collect().map_err(|_| Failure::Control)?;
            drop(slot.take_fixture_result().map_err(|_| Failure::Control)?);
        }
        Observation::Joined => drop(slot.take_fixture_result().map_err(|_| Failure::Control)?),
        Observation::Taken | Observation::Unused | Observation::NotSubmitted(_) => {}
        Observation::Busy | Observation::Joining | Observation::Control | Observation::TimedOut => {
            return Err(Failure::Control);
        }
    }
    Ok(())
}

/// Run one source-issued configuration through the real synchronous facade and slot.
/// # Errors
/// Preserve typed source failure, actual worker/observation refusal and bounded diagnostics.
/// # Panics
/// Reject unjoined custody, retained provider/Config, endpoint-slot creation or work reset.
fn run(
    slot: &'static WorkerSlot,
    admission: &'static ProcessAdmission,
    scenario: Scenario,
) -> Result<Run, Failure> {
    run_with_hooks(slot, admission, scenario, Hooks::none())
}

/// Install the selected worker hook while exercising the real source and joined result.
/// # Errors
/// Preserve source, worker, observation and output refusal.
/// # Panics
/// Reject unjoined or retained real resources and lost policy or work accounting.
fn run_with_hooks(
    slot: &'static WorkerSlot,
    admission: &'static ProcessAdmission,
    scenario: Scenario,
    hooks: Hooks,
) -> Result<Run, Failure> {
    let fixture: Issued = issued(scenario)?;
    let expected: SourceConfigurationSnapshot = fixture.configuration.clone();
    let config: Config = if scenario.protected() {
        Config::from_managed_protected(
            fixture
                .authority
                .authorize_protected_ke(&expected)
                .map_err(Failure::SourcePolicy)?,
        )
    } else {
        Config::from_managed_preview(
            fixture
                .source
                .capture_ke_preview()
                .map_err(Failure::SourcePolicy)?,
        )
    };
    let config_lifetime: Weak<()> = Arc::downgrade(&config.lifetime);
    let producer: PreviewProducer = PreviewProducer {
        authority: fixture.authority,
        configuration: fixture.configuration,
        target: fixture.target,
        scenario,
        replacement: None,
        calls: [0; 4],
    };
    let (observed, receiver): (SyncSender<Work>, Receiver<Work>) = sync_channel(1);
    let result: Result<ReceivedCompletion, RunFailed> = run_synchronously_with(
        config,
        slot,
        None,
        move |actual_slot: &'static WorkerSlot, prepared: PreparedInvocation| {
            actual_slot.submit_fixture(
                prepared,
                admission,
                component_provider,
                move |original: &Config, provider: &ProviderScope<'_>, control: Control| {
                    component_operation(original, provider, control, producer, &observed)
                },
                hooks,
            )
        },
    );
    let observation_end: Instant = Instant::now().checked_add(WAIT).ok_or(Failure::Deadline)?;
    let observation: Observation = slot.observer().poll_until(observation_end);
    drain(slot, observation)?;
    if let Err(failure) = &result {
        let mut diagnostic: Vec<u8> = Vec::new();
        output::write_run_failure(failure, &mut diagnostic).map_err(|_| Failure::Control)?;
        eprintln!(
            "KE_MANAGED_PREVIEW_RUN {}",
            String::from_utf8_lossy(&diagnostic)
        );
    }
    assert_eq!(observation, Observation::Taken);
    assert_eq!(config_lifetime.strong_count(), 0);
    assert!(matches!(admission.claim(), Err(Failure::ProcessUsed)));
    let work: Work = receiver.recv_timeout(WAIT).map_err(|_| Failure::Control)?;
    assert_eq!(work.provider_lifetime.strong_count(), 0);
    let checked_peers: [SocketAddr; 2] = [
        fixture.allowed,
        SocketAddr::from((Ipv4Addr::LOCALHOST, 123)),
    ];
    if scenario.protected() {
        let actual: SourcePolicySnapshot = fixture
            .source
            .policy_snapshot(fixture.allowed)
            .map_err(Failure::SourcePolicy)?;
        let should_debit: bool = !matches!(scenario, Scenario::ProtectedBeforeSend);
        assert_eq!(actual.source().floor().is_some(), should_debit);
        assert_eq!(actual.endpoint().floor().is_some(), should_debit);
    } else {
        for peer in checked_peers {
            assert!(matches!(
                fixture.source.policy_snapshot(peer),
                Err(SourcePolicyError::UnknownEndpoint)
            ));
        }
    }
    assert_work_coordinates(&work);
    Ok(Run {
        outcome: result,
        work,
        source: fixture.source,
        expected,
    })
}

/// Keep the normal operation's original cause in a fixture failure.
fn operation_primary(failure: &RunFailed) -> Failure {
    match failure {
        RunFailed::Operation(operation) => operation.primary,
        _ => Failure::Control,
    }
}

/// Compare original coordinates and initially unspent counters without reading fresh time.
/// # Panics
/// Reject a renewed cutoff, uncharged original state or introduced recovery history.
fn assert_work_coordinates(work: &Work) {
    assert_eq!(
        (
            work.before.transactions,
            work.before.refusals,
            work.before.observations,
            work.before.contexts
        ),
        (0, 0, 0, 0)
    );
    assert_eq!(
        (work.after.origin, work.after.cutoff),
        (work.before.origin, work.before.cutoff)
    );
    assert_eq!(
        (
            work.after.not_before,
            work.after.source_not_before,
            work.after.global_not_before,
            work.after.intent_not_before
        ),
        (
            work.before.not_before,
            work.before.source_not_before,
            work.before.global_not_before,
            work.before.intent_not_before
        )
    );
    assert!(!work.after.has_previous && work.after.base.is_none());
}

/// Inspect genuine joined history and independent fixed framing/count expectations.
/// # Errors
/// Preserve the absence of the required managed historical projection.
/// # Panics
/// Reject an unmanaged substitute, replaced original context or UDP effect.
fn assert_history(
    completion: ReceivedCompletion,
    expected: &SourceConfigurationSnapshot,
) -> Result<(), Failure> {
    assert!(completion.protected().is_none());
    let history: Box<ManagedPreviewHistory> = completion.into_managed_preview()?;
    let summary: &Summary = history.summary();
    assert!(history.configuration().same_configuration(expected));
    assert_eq!(
        (summary.cookies, summary.cookie_bytes, summary.packet_bytes),
        (1, 12, 140)
    );
    assert!(!summary.transmitted);
    Ok(())
}

/// An issued legal definition crosses the actual worker/sealer/joined receiver once.
/// # Errors
/// Preserve actual issuer, worker, provider and managed receiving refusal.
/// # Panics
/// Reject a missing healthy completion, changed counts or reset work.
#[test]
fn managed_ke_preview_worker_healthy_preserves_issued_history() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let run: Run = run(&SLOT, &ADMISSION, Scenario::Healthy)?;
    assert!(
        run.outcome.is_ok(),
        "issued legal Preview must complete through its actual joined receiver"
    );
    let completion: ReceivedCompletion = run
        .outcome
        .map_err(|failure: RunFailed| operation_primary(&failure))?;
    assert_history(completion, &run.expected)?;
    assert_eq!(run.work.calls, [1, 1, 0, 0]);
    assert_eq!(
        (
            run.work.after.transactions,
            run.work.after.refusals,
            run.work.after.observations,
            run.work.after.contexts
        ),
        (1, 0, 2, 1)
    );
    assert_eq!(run.work.after.end, Some(End::Completed));
    assert!(run.work.last_failure.is_none() && run.work.replacement.is_none());
    Ok(())
}

/// A protected source commits under its original definition, then retains that history.
/// # Errors
/// Preserve actual issuer, worker, adapter or joined receiving refusal.
/// # Panics
/// Reject premature cookie installation, absent probing or postcommit relabelling.
#[test]
fn managed_protected_worker_preserves_committed_definition_through_join() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let run: Run = run(&SLOT, &ADMISSION, Scenario::ProtectedAfterCommit)?;
    let completion: ReceivedCompletion = run
        .outcome
        .map_err(|failure: RunFailed| operation_primary(&failure))?;
    let (configuration, summary): (SourceConfigurationSnapshot, Summary) =
        completion.into_managed_protected()?;
    assert!(summary.transmitted);
    assert!(configuration.same_configuration(&run.expected));
    assert_eq!(run.work.calls, [1, 0, 1, 0]);
    assert_eq!(run.work.after.end, Some(End::Completed));
    let newer: &SourceConfigurationSnapshot =
        run.work.replacement.as_ref().ok_or(Failure::Control)?;
    assert!(!newer.same_configuration(&configuration));
    Ok(())
}

/// A real post-record panic retains only the already committed source revision.
/// # Errors
/// Preserve unexpected operation, observation or absent sealed history.
/// # Panics
/// Reject joined-success projection, revision substitution or invented retirement.
#[test]
fn managed_protected_abnormal_join_preserves_sealed_configuration() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let mut hooks: Hooks = Hooks::none();
    hooks.before_retire = Some(Box::new(|| {
        std::panic::resume_unwind(Box::new("managed protected fixture after record"));
    }));
    let run: Run = run_with_hooks(&SLOT, &ADMISSION, Scenario::ProtectedAfterCommit, hooks)?;
    let Err(failure) = run.outcome else {
        return Err(Failure::Control);
    };
    let RunFailed::Abnormal(joined) = &failure else {
        return Err(Failure::Control);
    };
    assert_eq!(joined.termination, Termination::Panicked);
    assert!(!joined.retired);
    assert_eq!(SLOT.sealed_count().map_err(|_| Failure::Control)?, 1);
    assert_eq!(run.work.calls, [1, 0, 1, 0]);
    assert_eq!(run.work.after.end, Some(End::Completed));
    let projection: SourceProtectedProbeFailure =
        SourceProtectedProbeFailure::from_run_failure(failure);
    assert_eq!(projection.labels(), ("worker-termination", "panicked"));
    assert!(projection.primary_labels().is_none());
    let original: &SourceConfigurationSnapshot = projection
        .retained_configuration()
        .ok_or(Failure::Control)?;
    assert!(original.same_configuration(&run.expected));
    let newer: &SourceConfigurationSnapshot =
        run.work.replacement.as_ref().ok_or(Failure::Control)?;
    let current: SourceConfigurationSnapshot = run
        .source
        .configuration_snapshot()
        .map_err(Failure::SourcePolicy)?
        .ok_or(Failure::Control)?;
    assert!(current.same_configuration(newer));
    assert!(!original.same_configuration(&current));
    Ok(())
}

/// A same-byte replacement refuses the guarded send or the final retired reply.
/// # Errors
/// Preserve unexpected worker or lack of any failure.
/// # Panics
/// Reject admitted stale effects, a lost polling floor or an automatic retry.
#[test]
fn managed_protected_worker_refuses_replacement_at_both_effects() -> Result<(), Failure> {
    static SEND_SLOT: WorkerSlot = WorkerSlot::new();
    static SEND_ADMISSION: ProcessAdmission = ProcessAdmission::new();
    static REPLY_SLOT: WorkerSlot = WorkerSlot::new();
    static REPLY_ADMISSION: ProcessAdmission = ProcessAdmission::new();
    for (slot, admission, scenario) in [
        (&SEND_SLOT, &SEND_ADMISSION, Scenario::ProtectedBeforeSend),
        (
            &REPLY_SLOT,
            &REPLY_ADMISSION,
            Scenario::ProtectedBeforeReplyCommit,
        ),
    ] {
        let run: Run = run(slot, admission, scenario)?;
        let Err(RunFailed::Operation(failure)) = run.outcome else {
            return Err(Failure::Control);
        };
        assert_eq!(
            failure.primary,
            Failure::SourcePolicy(SourcePolicyError::ConfigurationChanged)
        );
        assert_eq!(run.work.calls, [1, 0, 1, 0]);
        assert_eq!(run.work.after.end, Some(End::Failure));
        assert!(run.work.replacement.is_some());
        let projected: SourceProtectedProbeFailure =
            SourceProtectedProbeFailure::from_run_failure(RunFailed::Operation(failure));
        assert!(projected.retained_configuration().is_none());
    }
    Ok(())
}

/// Direct Preview callers cannot convert the managed protected association.
/// # Errors
/// Preserve fixture, publication or genuine managed Preview refusal.
/// # Panics
/// Reject entropy or checkout on the wrong mode and any blanket Preview refusal.
#[test]
fn managed_protected_refuses_preview_downgrade_before_charging() -> Result<(), Failure> {
    let fixture: Issued = issued(Scenario::Healthy)?;
    let protected: Config = Config::from_managed_protected(
        fixture
            .authority
            .authorize_protected_ke(&fixture.configuration)
            .map_err(Failure::SourcePolicy)?,
    );
    with_owner(
        &protected,
        COMPONENT_UDP,
        |owner: &mut AssociationOwner<'_>, _control: &Control| {
            let before: Status = owner.status();
            let mut entropy_calls: usize = 0;
            assert!(matches!(
                owner.prepare_preview(|_: &mut [u8; 48]| {
                    entropy_calls += 1;
                    Ok(())
                }),
                Err(Failure::Control)
            ));
            assert_eq!(entropy_calls, 0);
            assert!(matches!(
                owner.preview_owner_for_test(),
                Err(Failure::Control)
            ));
            assert!(matches!(
                owner.reserve(RequestInput::Preview),
                Err(Failure::Control)
            ));
            assert_eq!(owner.status(), before);
            Ok(())
        },
    )?;
    let preview: Config = Config::from_managed_preview(
        fixture
            .source
            .capture_ke_preview()
            .map_err(Failure::SourcePolicy)?,
    );
    with_owner(
        &preview,
        COMPONENT_UDP,
        |owner: &mut AssociationOwner<'_>, _control: &Control| {
            let prepared: super::handoff::PreparedRequest<'_> =
                owner.prepare_preview(|output: &mut [u8; 48]| {
                    output.fill(0x42);
                    Ok(())
                })?;
            let history: ManagedPreviewHistory = prepared.finish_managed()?;
            assert!(
                history
                    .configuration()
                    .same_configuration(&fixture.configuration)
            );
            assert!(!history.summary().transmitted);
            Ok(())
        },
    )
}

/// Replacing the real definition during entropy refuses final publication without refunds.
/// # Errors
/// Preserve actual setup/worker refusal or absence of the intended operation failure.
/// # Panics
/// Reject substituted failure causes, lost charges or fabricated replacement history.
#[test]
fn managed_ke_preview_worker_entropy_replacement_retains_failure_and_charges() -> Result<(), Failure>
{
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let run: Run = run(&SLOT, &ADMISSION, Scenario::ReplaceDuringEntropy)?;
    let Err(RunFailed::Operation(failure)) = run.outcome else {
        return Err(Failure::Control);
    };
    assert_eq!(
        failure.primary,
        Failure::SourcePolicy(SourcePolicyError::ConfigurationChanged)
    );
    assert!(failure.cleanup.is_none() && failure.admission_cleanup.is_none());
    assert_eq!(run.work.last_failure, Some(failure));
    assert_eq!(run.work.calls, [1, 1, 0, 0]);
    assert_eq!(
        (
            run.work.after.transactions,
            run.work.after.refusals,
            run.work.after.observations,
            run.work.after.contexts
        ),
        (1, 0, 2, 1)
    );
    assert_eq!(run.work.after.end, Some(End::Failure));
    let replacement: &SourceConfigurationSnapshot =
        run.work.replacement.as_ref().ok_or(Failure::Control)?;
    assert!(!replacement.same_configuration(&run.expected));
    assert!(
        run.source
            .configuration_snapshot()
            .map_err(Failure::SourcePolicy)?
            .as_ref()
            .is_some_and(
                |actual: &SourceConfigurationSnapshot| actual.same_configuration(replacement)
            )
    );
    Ok(())
}

/// A genuine association outside the issued UDP set refuses before Preview entropy.
/// # Errors
/// Preserve actual issuer/worker refusal or absence of the intended operation failure.
/// # Panics
/// Reject publication/sealing of the outside endpoint or lost failure/charge facts.
#[test]
fn managed_ke_preview_worker_outside_endpoint_never_enters_sealing() -> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let run: Run = run(&SLOT, &ADMISSION, Scenario::OutsideEndpoint)?;
    let Err(RunFailed::Operation(failure)) = run.outcome else {
        return Err(Failure::Control);
    };
    assert_eq!(
        failure.primary,
        Failure::SourcePolicy(SourcePolicyError::Foreign)
    );
    assert!(failure.cleanup.is_none() && failure.admission_cleanup.is_none());
    assert_eq!(run.work.last_failure, Some(failure));
    assert_eq!(run.work.calls, [1, 0, 0, 0]);
    assert_eq!(
        (
            run.work.after.transactions,
            run.work.after.refusals,
            run.work.after.observations,
            run.work.after.contexts
        ),
        (1, 0, 2, 0)
    );
    assert_eq!(run.work.after.end, Some(End::Failure));
    Ok(())
}

/// A real replacement after material preparation prevents association publication.
/// # Errors
/// Preserve actual issuer/worker refusal or absence of the intended operation failure.
/// # Panics
/// Reject stale publication, entropy entry, lost work or an unchanged issued revision.
#[test]
fn managed_ke_preview_worker_replacement_before_publication_never_enters_sealing()
-> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let run: Run = run(&SLOT, &ADMISSION, Scenario::ReplaceBeforePublication)?;
    let Err(RunFailed::Operation(failure)) = run.outcome else {
        return Err(Failure::Control);
    };
    assert_eq!(
        failure.primary,
        Failure::SourcePolicy(SourcePolicyError::ConfigurationChanged)
    );
    assert!(failure.cleanup.is_none() && failure.admission_cleanup.is_none());
    assert_eq!(run.work.last_failure, Some(failure));
    assert_eq!(run.work.calls, [1, 0, 0, 0]);
    assert_eq!(
        (
            run.work.after.transactions,
            run.work.after.refusals,
            run.work.after.observations,
            run.work.after.contexts
        ),
        (1, 0, 2, 0)
    );
    assert_eq!(run.work.after.end, Some(End::Failure));
    let replacement: &SourceConfigurationSnapshot =
        run.work.replacement.as_ref().ok_or(Failure::Control)?;
    assert!(!replacement.same_configuration(&run.expected));
    Ok(())
}

/// Replacement before worker publication cannot relabel the subsequent joined history.
/// # Errors
/// Preserve actual issuer, worker, receiver or postcommit replacement refusal.
/// # Panics
/// Reject revised historical context, UDP effects or missing completed state.
#[test]
fn managed_ke_preview_worker_postcommit_replacement_preserves_original_history()
-> Result<(), Failure> {
    static SLOT: WorkerSlot = WorkerSlot::new();
    static ADMISSION: ProcessAdmission = ProcessAdmission::new();
    let run: Run = run(&SLOT, &ADMISSION, Scenario::ReplaceAfterCommit)?;
    assert!(run.outcome.is_ok());
    let completion: ReceivedCompletion = run
        .outcome
        .map_err(|failure: RunFailed| operation_primary(&failure))?;
    let newer: &SourceConfigurationSnapshot =
        run.work.replacement.as_ref().ok_or(Failure::Control)?;
    assert!(!newer.same_configuration(&run.expected));
    assert_history(completion, &run.expected)?;
    assert!(
        run.source
            .capture_ke_preview()
            .map_err(Failure::SourcePolicy)?
            .snapshot()
            .same_configuration(newer)
    );
    assert_eq!(run.work.calls, [1, 1, 0, 0]);
    assert_eq!(run.work.after.end, Some(End::Completed));
    Ok(())
}

/// Direct managed protected-entry attempts refuse before adapter setup or ledger spending.
/// # Errors
/// Preserve actual checked issuance, publication and legal input construction refusal.
/// # Panics
/// Reject a bypass through either protected representation or any pre-refusal debit.
#[test]
fn managed_ke_preview_direct_protected_entries_refuse_without_spending() -> Result<(), Failure> {
    let fixture: Issued = issued(Scenario::Healthy)?;
    let config: Config = Config::from_managed_preview(
        fixture
            .source
            .capture_ke_preview()
            .map_err(Failure::SourcePolicy)?,
    );
    with_owner(
        &config,
        COMPONENT_UDP,
        |owner: &mut AssociationOwner<'_>, control: &Control| {
            let before: Status = owner.status();
            assert_eq!(before.phase, RequestPhase::Idle);
            assert_eq!(
                (
                    before.attempts,
                    before.frames,
                    before.bytes,
                    before.available,
                    before.used,
                    before.uids
                ),
                (32, 32, 131_072, 1, 0, 0)
            );
            assert_eq!(owner.check_context(control), Err(Failure::Control));
            assert_eq!(owner.status(), before);
            assert!(matches!(
                owner.reserve(RequestInput::Preview),
                Err(Failure::Control)
            ));
            assert_eq!(owner.status(), before);
            let inputs: [RequestInput; 2] = protected_inputs()?;
            for input in inputs {
                assert!(matches!(owner.reserve(input), Err(Failure::Control)));
                assert_eq!(owner.status(), before);
            }
            Ok(())
        },
    )?;
    assert!(matches!(
        fixture.source.policy_snapshot(fixture.allowed),
        Err(SourcePolicyError::UnknownEndpoint)
    ));
    Ok(())
}

/// Existing diagnostic Preview and explicit protected mode remain legal direct neighbors.
/// # Errors
/// Preserve checked diagnostic configuration and exercised real owner/ledger refusals.
/// # Panics
/// Reject a blanket protected-entry ban or an altered finite checkout charge.
#[test]
fn managed_ke_preview_direct_protected_entries_keep_diagnostic_neighbors() -> Result<(), Failure> {
    let preview: Config = Config::new("127.0.0.1:4460".into(), "Time.Example".into())?;
    assert_protected_neighbor(&preview)?;
    let arguments: [OsString; 3] = [
        OsString::from("127.0.0.1:4460"),
        OsString::from("Time.Example"),
        OsString::from("--protected-datagram"),
    ];
    let protected: Config = Config::from_arguments(arguments.into_iter())?;
    assert_protected_neighbor(&protected)
}

/// Exercise same-Control attachment and the distinct managed and diagnostic finalizers.
/// # Errors
/// Preserve actual issuer, publication and preparation refusals before finalization.
/// # Panics
/// Reject foreign attachment, diagnostic publication of managed work or a blanket finalizer ban.
#[test]
fn managed_ke_preview_ordinary_finish_refuses_with_diagnostic_neighbor() -> Result<(), Failure> {
    let fixture: Issued = issued(Scenario::Healthy)?;
    let managed: Config = Config::from_managed_preview(
        fixture
            .source
            .capture_ke_preview()
            .map_err(Failure::SourcePolicy)?,
    );
    with_owner(
        &managed,
        COMPONENT_UDP,
        |owner: &mut AssociationOwner<'_>, control: &Control| {
            let raw_keys: DirectionalKeys =
                DirectionalKeys::new(Zeroizing::new([0x11_u8; 32]), Zeroizing::new([0x22_u8; 32]));
            let raw_cookies: Vec<Zeroizing<Vec<u8>>> = vec![Zeroizing::new(vec![0xa5_u8; 12])];
            let raw: super::handoff::PreparedRequest<'_> = super::handoff::prepare(
                control,
                &raw_keys,
                &raw_cookies,
                |bytes: &mut [u8; 48]| {
                    bytes.fill(0x42_u8);
                    Ok(())
                },
            )?;
            assert!(matches!(raw.finish(), Err(Failure::Control)));

            let foreign_control: Control = Control::new(control.origin)?;
            foreign_control.publish_ke()?;
            let before: Status = owner.status();
            let attachments: [(&Control, bool); 2] = [(&foreign_control, false), (control, true)];
            for (packet_control, same_owner) in attachments {
                let raw: super::handoff::PreparedRequest<'_> = super::handoff::prepare(
                    packet_control,
                    &raw_keys,
                    &raw_cookies,
                    |bytes: &mut [u8; 48]| {
                        bytes.fill(0x42_u8);
                        Ok(())
                    },
                )?;
                let origin: super::association::PreviewOwner<'_> =
                    owner.preview_owner_for_test()?;
                assert!(matches!(
                    (same_owner, raw.bind_owner(origin)),
                    (false, Err(Failure::Control)) | (true, Ok(_))
                ));
            }
            assert_eq!(owner.status(), before);
            assert_eq!(foreign_control.with_published(|_| Ok(())), Ok(()));
            assert_eq!(control.with_published(|_| Ok(())), Ok(()));

            let prepared: super::handoff::PreparedRequest<'_> =
                owner.prepare_preview(|bytes: &mut [u8; 48]| {
                    bytes.fill(0x42_u8);
                    Ok(())
                })?;
            assert!(matches!(prepared.finish(), Err(Failure::Control)));
            assert_eq!(control.with_published(|_| Ok(())), Ok(()));
            Ok(())
        },
    )?;
    let diagnostic: Config = Config::new("127.0.0.1:4460".into(), "Time.Example".into())?;
    with_owner(
        &diagnostic,
        COMPONENT_UDP,
        |owner: &mut AssociationOwner<'_>, control: &Control| {
            let duplicate: super::association::PreviewOwner<'_> = owner.preview_owner_for_test()?;
            let prepared: super::handoff::PreparedRequest<'_> =
                owner.prepare_preview(|bytes: &mut [u8; 48]| {
                    bytes.fill(0x42_u8);
                    Ok(())
                })?;
            assert!(matches!(
                prepared.bind_owner(duplicate),
                Err(Failure::Control)
            ));
            assert_eq!(control.with_published(|_| Ok(())), Ok(()));
            Ok(())
        },
    )?;
    with_owner(
        &diagnostic,
        COMPONENT_UDP,
        |owner: &mut AssociationOwner<'_>, _control: &Control| {
            let prepared: super::handoff::PreparedRequest<'_> =
                owner.prepare_preview(|bytes: &mut [u8; 48]| {
                    bytes.fill(0x42_u8);
                    Ok(())
                })?;
            let summary: Summary = prepared.finish()?;
            assert_eq!(
                (summary.cookies, summary.cookie_bytes, summary.packet_bytes),
                (1, 12, 140)
            );
            assert!(!summary.transmitted);
            Ok(())
        },
    )
}

/// Select when the same checked alias request delivers its restriction.
#[derive(Clone, Copy)]
enum AliasRestrictionTiming {
    /// Restrict the endpoint after preparation and before final admission.
    BeforeCommit,
    /// Restrict the endpoint after the actual final admission.
    AfterCommit,
}

/// One actual prepared Preview observes only its selected endpoint at final commit.
/// # Errors
/// Preserve genuine policy/association setup or healthy completion failures.
/// # Panics
/// Fail on a restriction bypass, transitive refusal, renewed budget or rewritten history.
fn check_managed_preview_alias_restriction(
    code: u32,
    udp: SocketAddr,
    timing: AliasRestrictionTiming,
) -> Result<(), Failure> {
    let interval: Duration = Duration::from_millis(1);
    let limits: SourcePolicyLimits =
        SourcePolicyLimits::new(2, 1, interval, Duration::from_millis(4))
            .map_err(Failure::SourcePolicy)?;
    let mut authority: SourcePolicyAuthority =
        crate::source_policy::isolated_authority(limits).map_err(Failure::SourcePolicy)?;
    let source: SourceHandle = authority.source(0).map_err(Failure::SourcePolicy)?;
    let alias: SourceHandle = authority.source(1).map_err(Failure::SourcePolicy)?;
    let tls: SocketAddr = SocketAddr::from((Ipv4Addr::LOCALHOST, 4460));
    let target: KePreviewTarget = KePreviewTarget::new(tls, "Time.Example")
        .map_err(|_: KePreviewTargetError| Failure::Arguments)?;
    let addresses: crate::SourceAddressSet =
        crate::SourceAddressSet::new(&[COMPONENT_UDP, ALTERNATE_UDP])
            .map_err(Failure::SourcePolicy)?;
    let original: SourceConfigurationSnapshot = authority
        .install_ke_preview_configuration(&source, addresses, target)
        .map_err(Failure::SourcePolicy)?;
    let alias_addresses: crate::SourceAddressSet =
        crate::SourceAddressSet::new(&[COMPONENT_UDP]).map_err(Failure::SourcePolicy)?;
    let _alias_configuration: SourceConfigurationSnapshot = authority
        .install_source_configuration(&alias, alias_addresses)
        .map_err(Failure::SourcePolicy)?;
    let binding: SourceBinding = alias.bind(COMPONENT_UDP).map_err(Failure::SourcePolicy)?;
    let now: Instant = Instant::now();
    let floor: Instant = now.checked_add(interval).ok_or(Failure::Deadline)?;
    let request: Arc<()> = Arc::new(());
    let mut pending: FixtureRequest =
        fixture_send(&binding, &request, now).map_err(Failure::SourcePolicy)?;
    let baseline: SourcePolicySnapshot = source
        .policy_snapshot(COMPONENT_UDP)
        .map_err(Failure::SourcePolicy)?;
    assert_eq!(baseline.endpoint().floor(), Some(floor));
    assert!(!baseline.endpoint().refused());

    let config: Config =
        Config::from_managed_preview(source.capture_ke_preview().map_err(Failure::SourcePolicy)?);
    with_owner(
        &config,
        udp,
        |owner: &mut AssociationOwner<'_>, control: &Control| {
            let prepared: super::handoff::PreparedRequest<'_> =
                owner.prepare_preview(|bytes: &mut [u8; 48]| {
                    bytes.fill(0x42_u8);
                    Ok(())
                })?;
            let charged: Status = owner.status();
            assert_eq!(
                (charged.attempts, charged.available, charged.used),
                (31, 1, 0)
            );
            let before_commit: bool = matches!(timing, AliasRestrictionTiming::BeforeCommit);
            if before_commit {
                fixture_control(&binding, &mut pending, code, now)
                    .map_err(Failure::SourcePolicy)?;
            }
            let result: Result<ManagedPreviewHistory, Failure> = prepared.finish_managed();
            if !before_commit {
                fixture_control(&binding, &mut pending, code, now)
                    .map_err(Failure::SourcePolicy)?;
            }
            if before_commit && udp == COMPONENT_UDP {
                assert!(
                    matches!(
                        result,
                        Err(Failure::SourcePolicy(SourcePolicyError::Disabled))
                    ),
                    "current shared-endpoint restriction must veto Preview commit"
                );
                assert_eq!(control.with_published(|_| Ok(())), Ok(()));
            } else {
                let history: ManagedPreviewHistory = result?;
                let summary: &Summary = history.summary();
                assert!(history.configuration().same_configuration(&original));
                assert_eq!(
                    (summary.cookies, summary.cookie_bytes, summary.packet_bytes),
                    (1, 12, 140)
                );
                assert!(!summary.transmitted);
                assert_eq!(control.with_published(|_| Ok(())), Err(Failure::Control));
            }
            assert_eq!(owner.status(), charged);
            Ok(())
        },
    )?;

    assert_alias_restriction_state(
        &source,
        &binding,
        &original,
        baseline.source().revision(),
        floor,
    )
}

/// Verify the source and shared endpoint records after the alias control.
/// # Errors
/// Preserve actual snapshot refusal.
/// # Panics
/// Fail on transitive source refusal, shortened floor or a new independent slot.
fn assert_alias_restriction_state(
    source: &SourceHandle,
    binding: &SourceBinding,
    original: &SourceConfigurationSnapshot,
    original_source_revision: u64,
    floor: Instant,
) -> Result<(), Failure> {
    let current: SourceConfigurationSnapshot = source
        .configuration_snapshot()
        .map_err(Failure::SourcePolicy)?
        .ok_or(Failure::Control)?;
    assert!(current.same_configuration(original));
    let policy: SourcePolicySnapshot = source
        .policy_snapshot(COMPONENT_UDP)
        .map_err(Failure::SourcePolicy)?;
    assert!(!policy.source().refused());
    assert_eq!(policy.source().floor(), None);
    assert_eq!(policy.source().revision(), original_source_revision);
    assert!(policy.endpoint().refused());
    assert_eq!(policy.endpoint().floor(), Some(floor));
    let alias_policy: SourcePolicySnapshot =
        binding.policy_snapshot().map_err(Failure::SourcePolicy)?;
    assert!(alias_policy.source().refused());
    assert_eq!(alias_policy.source().floor(), Some(floor));
    assert!(matches!(
        source.policy_snapshot(ALTERNATE_UDP),
        Err(SourcePolicyError::UnknownEndpoint)
    ));
    Ok(())
}

/// Exercise both restriction codes against both selected endpoints on either side of commit.
/// # Errors
/// Preserve the first unsuccessful component history.
#[test]
fn managed_ke_preview_alias_endpoint_restriction_preserves_commit_history() -> Result<(), Failure> {
    let codes: [u32; 2] = [u32::from_be_bytes(*b"DENY"), u32::from_be_bytes(*b"RSTR")];
    let timings: [AliasRestrictionTiming; 2] = [
        AliasRestrictionTiming::BeforeCommit,
        AliasRestrictionTiming::AfterCommit,
    ];
    for code in codes {
        for udp in [COMPONENT_UDP, ALTERNATE_UDP] {
            for timing in timings {
                check_managed_preview_alias_restriction(code, udp, timing)?;
            }
        }
    }
    Ok(())
}

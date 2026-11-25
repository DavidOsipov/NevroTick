// SPDX-License-Identifier: GPL-2.0-only
//! Joined resolver results and precredited later-round ownership.

#[cfg(any(windows, test))]
#[path = "refresh/native_job.rs"]
mod native_job;
#[cfg(windows)]
#[path = "refresh/windows.rs"]
mod windows;

use std::{
    net::SocketAddr,
    time::{Duration, Instant},
};

use tokio::{
    sync::{OwnedSemaphorePermit, watch},
    task::JoinHandle,
    time::{Instant as TokioInstant, sleep_until},
};

use super::{
    AddressRoundConfig, AddressRoundConfigError, AddressRoundEnd, AddressRoundReport, CAPACITY,
    Control, ControlHandle, PACKET_BYTES, PilotError, PreparedAddressRound, RoundBindings,
    current_end, end_at, prepare_address_round, run_round, series_packets,
};
use crate::{SourceHandle, SourcePolicyError};

/// One selected source of a checked future answer, with original work reserved.
pub(super) enum QueryPlan {
    /// A declared loopback answer for the cooperative laboratory path.
    Lab(RefreshPlan),
    /// Address lookup admitted by the actual Windows source-policy authority.
    #[cfg(windows)]
    Windows(Box<windows::WindowsRefreshPlan>),
}

/// Independent resolver disposition retained beside the original round history.
pub enum AddressRoundRefreshOutcome {
    /// The original round reached a terminal outcome other than exhaustion.
    NotNeeded,
    /// The joined answer retained the existing canonical membership and order.
    Unchanged,
    /// One changed answer used its already reserved successor allowance.
    Successor(Box<AddressRoundReport>),
    /// Current control, source revision or the original cutoff forbade adoption.
    Retired(AddressRoundEnd),
    /// Resolver or policy failure prevented adoption without erasing round history.
    Failed(PilotError),
}

/// Complete checked lookup result and its independently preadmitted work.
pub(super) struct RefreshPlan {
    /// One whole checked answer with original per-request and operation budgets.
    config: AddressRoundConfig,
    /// Fresh packets that do not reuse any earlier correlation token.
    packets: [[u8; PACKET_BYTES]; CAPACITY],
}

/// Both actual job owners converge at the same checked result consumer.
enum RefreshJob {
    /// Cooperative laboratory task actually joined by the round.
    Lab(JoinHandle<RefreshPlan>),
    /// Blocking Windows submission and callback wait under process custody.
    #[cfg(windows)]
    Native(native_job::NativeResolverJob),
}

/// Preserve the original retirement and any outstanding native custody.
struct RetiredRefresh {
    /// Typed reason for refusing or discarding the answer.
    outcome: AddressRoundRefreshOutcome,
    /// A native thread remains owned by the process reaper.
    native_pending: bool,
}

impl RetiredRefresh {
    /// Construct an outcome from an already settled or unselected job.
    const fn settled(outcome: AddressRoundRefreshOutcome) -> Self {
        Self {
            outcome,
            native_pending: false,
        }
    }
}

/// Reserve both packet batches and the original whole-operation budget.
/// # Errors
/// Refuse insufficient combined time, entered runtime, entropy or source policy.
fn prepare_with_budget(
    source: &SourceHandle,
    initial: AddressRoundConfig,
    answer_capacity: usize,
) -> Result<
    (
        PreparedAddressRound,
        ControlHandle,
        [[u8; PACKET_BYTES]; CAPACITY],
    ),
    PilotError,
> {
    let count: u32 = u32::try_from(initial.peers.len() + answer_capacity)
        .map_err(|_| PilotError::InvalidTimeout)?;
    let required: Duration = initial
        .request_timeout
        .checked_mul(count)
        .ok_or(PilotError::InvalidTimeout)?;
    if required > initial.round_timeout {
        return Err(PilotError::InvalidTimeout);
    }
    let (mut prepared, handle): (PreparedAddressRound, ControlHandle) =
        prepare_address_round(initial)?;
    let mut entropy: [u8; 24] = [0; 24];
    getrandom::fill(&mut entropy).map_err(|_| PilotError::Entropy)?;
    let packets: [[u8; PACKET_BYTES]; CAPACITY] = series_packets(&entropy)?;
    if packets
        .iter()
        .any(|packet| prepared.packets.contains(packet))
    {
        return Err(PilotError::DuplicateToken);
    }
    prepared.source_policy = Some(
        source
            .bind_round_for_refresh(&prepared.config.peers)
            .map_err(PilotError::SourcePolicy)?,
    );
    Ok((prepared, handle, packets))
}

impl SourceHandle {
    /// Prepare one opted-in round and one possible successor from a bounded
    /// laboratory resolver declaration. Both rounds spend the original budget.
    ///
    /// # Errors
    /// Refuse invalid or missing membership, invalid resolver candidates,
    /// insufficient combined time, entered runtime, or entropy failure.
    pub fn prepare_address_round_refresh(
        &self,
        initial: AddressRoundConfig,
        resolved: &[SocketAddr],
    ) -> Result<(PreparedAddressRound, ControlHandle), PilotError> {
        let answer: AddressRoundConfig =
            AddressRoundConfig::new(resolved, initial.request_timeout, initial.round_timeout)
                .map_err(|error| match error {
                    AddressRoundConfigError::InvalidTimeout => PilotError::InvalidTimeout,
                    _ => PilotError::SourcePolicy(SourcePolicyError::InvalidSelection),
                })?;
        let (mut prepared, handle, packets) =
            prepare_with_budget(self, initial, answer.peers.len())?;
        prepared.refresh = Some(QueryPlan::Lab(RefreshPlan {
            config: answer,
            packets,
        }));
        Ok((prepared, handle))
    }
}

/// Preserve source replacement as a retirement and other policy failures verbatim.
fn policy_refusal(error: SourcePolicyError) -> AddressRoundRefreshOutcome {
    match error {
        SourcePolicyError::ConfigurationChanged => {
            AddressRoundRefreshOutcome::Retired(AddressRoundEnd::Stale)
        }
        error => AddressRoundRefreshOutcome::Failed(PilotError::SourcePolicy(error)),
    }
}

/// Reap the cooperative task or retain outstanding native completion custody.
async fn retire(job: RefreshJob, disposition: AddressRoundRefreshOutcome) -> RetiredRefresh {
    match job {
        RefreshJob::Lab(job) => {
            job.abort();
            RetiredRefresh::settled(match job.await {
                Err(error) if !error.is_cancelled() => {
                    AddressRoundRefreshOutcome::Failed(PilotError::ResolverTask)
                }
                _ => disposition,
            })
        }
        #[cfg(windows)]
        RefreshJob::Native(job) => RetiredRefresh {
            outcome: disposition,
            native_pending: job.retire(),
        },
    }
}

/// Join the actual selected worker and collect its one result.
/// # Errors
/// Preserve worker failure or the checked producer's refusal.
async fn wait(job: &mut RefreshJob) -> Result<RefreshPlan, AddressRoundRefreshOutcome> {
    match job {
        RefreshJob::Lab(job) => job
            .await
            .map_err(|_| AddressRoundRefreshOutcome::Failed(PilotError::ResolverTask)),
        #[cfg(windows)]
        RefreshJob::Native(job) => job.wait().await,
    }
}

/// Join one actual job or retire it under the invocation's original guards.
/// # Errors
/// Return a terminal disposition after reaping, with no usable late result.
async fn collect(
    mut job: RefreshJob,
    source: &RoundBindings,
    changes: &mut watch::Receiver<()>,
    control: &mut watch::Receiver<Control>,
    deadline: Instant,
) -> Result<RefreshPlan, RetiredRefresh> {
    let cutoff: tokio::time::Sleep = sleep_until(TokioInstant::from_std(deadline));
    tokio::pin!(cutoff);
    let disposition: AddressRoundRefreshOutcome = loop {
        if let Some(end) = current_end(control, deadline) {
            break AddressRoundRefreshOutcome::Retired(end);
        }
        if let Err(error) = source.resolution_current() {
            break policy_refusal(error);
        }
        let open: bool = tokio::select! {
            biased;
            signal = control.changed() => signal.is_ok(),
            signal = changes.changed() => signal.is_ok(),
            () = &mut cutoff => true,
            result = wait(&mut job) => return result.map_err(RetiredRefresh::settled),
        };
        if !open {
            break AddressRoundRefreshOutcome::Failed(PilotError::ResolverTask);
        }
    };
    Err(retire(job, disposition).await)
}

/// Adopt a joined answer with the real control guard held across publication.
/// # Errors
/// Refuse retired, stale, unadmitted or unrepresentable transitions.
fn prepare_successor(
    source: &RoundBindings,
    permit: &OwnedSemaphorePermit,
    answer: RefreshPlan,
    control: watch::Receiver<Control>,
    handle: ControlHandle,
    deadline: Instant,
) -> Result<Option<PreparedAddressRound>, AddressRoundRefreshOutcome> {
    let result: Result<Option<RoundBindings>, PilotError> = {
        let current: watch::Ref<'_, Control> = control.borrow();
        if let Some(end) = end_at(*current, Instant::now(), deadline) {
            return Err(AddressRoundRefreshOutcome::Retired(end));
        }
        source.adopt_resolution(&answer.config.peers, deadline, permit, &current)
    };
    let bindings: Option<RoundBindings> = result.map_err(|error| match error {
        PilotError::PolicyAdmissionDeadline => {
            AddressRoundRefreshOutcome::Retired(AddressRoundEnd::Deadline)
        }
        PilotError::SourcePolicy(error) => policy_refusal(error),
        error => AddressRoundRefreshOutcome::Failed(error),
    })?;
    Ok(bindings.map(|source_policy| PreparedAddressRound {
        source_policy: Some(source_policy),
        config: answer.config,
        original_cutoff: None,
        packets: answer.packets,
        control,
        handle,
        refresh: None,
    }))
}

/// Consume at most one precredited successor after the first owner returns.
async fn successor(
    source: &RoundBindings,
    permit: &OwnedSemaphorePermit,
    answer: RefreshPlan,
    control: watch::Receiver<Control>,
    handle: ControlHandle,
    origin: Instant,
    deadline: Instant,
) -> AddressRoundRefreshOutcome {
    match prepare_successor(source, permit, answer, control, handle, deadline) {
        Ok(Some(prepared)) => AddressRoundRefreshOutcome::Successor(Box::new(
            run_round(prepared, origin, deadline).await,
        )),
        Ok(None) => AddressRoundRefreshOutcome::Unchanged,
        Err(outcome) => outcome,
    }
}

/// Keep the active cursor and original clock while one admitted resolver runs.
pub(super) async fn run(
    mut prepared: PreparedAddressRound,
    permit: Option<&OwnedSemaphorePermit>,
    origin: Instant,
    deadline: Instant,
) -> AddressRoundReport {
    let Some(plan): Option<QueryPlan> = prepared.refresh.take() else {
        return run_round(prepared, origin, deadline).await;
    };
    let Some(permit): Option<&OwnedSemaphorePermit> = permit else {
        return AddressRoundReport::empty().fail(PilotError::InternalState);
    };
    let Some(source): Option<RoundBindings> = prepared.source_policy.clone() else {
        return AddressRoundReport::empty().fail(PilotError::InternalState);
    };
    let mut control: watch::Receiver<Control> = prepared.control.clone();
    if let Some(end) = current_end(&mut control, deadline) {
        let mut report: AddressRoundReport = AddressRoundReport::empty();
        report.round_end = end;
        report.refresh = Some(AddressRoundRefreshOutcome::Retired(end));
        return report;
    }
    let mut changes: watch::Receiver<()> = match source.resolution_changes() {
        Ok(changes) => changes,
        Err(error) => return AddressRoundReport::empty().fail(PilotError::SourcePolicy(error)),
    };
    let handle: ControlHandle = prepared.handle.clone();
    let job: Result<RefreshJob, PilotError> = match plan {
        QueryPlan::Lab(answer) => Ok(RefreshJob::Lab(tokio::spawn(async move {
            tokio::task::yield_now().await;
            answer
        }))),
        #[cfg(windows)]
        QueryPlan::Windows(plan) => {
            windows::start(*plan, source.clone(), control.clone(), deadline).map(RefreshJob::Native)
        }
    };
    let mut report: AddressRoundReport = run_round(prepared, origin, deadline).await;
    let settled: RetiredRefresh = match job {
        Ok(job) if report.round_end == AddressRoundEnd::Exhausted => {
            match collect(job, &source, &mut changes, &mut control, deadline).await {
                Ok(answer) => RetiredRefresh::settled(
                    successor(&source, permit, answer, control, handle, origin, deadline).await,
                ),
                Err(disposition) => disposition,
            }
        }
        Ok(job) => retire(job, AddressRoundRefreshOutcome::NotNeeded).await,
        Err(error) => RetiredRefresh::settled(AddressRoundRefreshOutcome::Failed(error)),
    };
    report.refresh = Some(settled.outcome);
    report.native_refresh_pending = settled.native_pending;
    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{SourceAddressSet, SourcePolicyLimits, source_policy::isolated_authority};
    use std::{error::Error, future::pending};
    use tokio::sync::oneshot;

    /// Observe actual worker-owned value destruction on retirement.
    struct Returned(
        /// One outcome sender, consumed by Drop.
        Option<oneshot::Sender<()>>,
    );

    impl Drop for Returned {
        fn drop(&mut self) {
            if let Some(sender) = self.0.take() {
                let _closed = sender.send(());
            }
        }
    }

    /// Trigger Stop, same-address replacement or cutoff while collection is
    /// pending, then observe that the worker has returned custody.
    /// # Errors
    /// Return unexpected fixture and actual runtime/authority errors.
    /// # Panics
    /// Reject a wrong owner disposition or a worker still running on return.
    #[test]
    fn pending_resolver_is_joined_after_stop_or_replacement() -> Result<(), Box<dyn Error>> {
        for expected in [
            AddressRoundEnd::Stopped,
            AddressRoundEnd::Stale,
            AddressRoundEnd::Deadline,
        ] {
            let mut authority = isolated_authority(SourcePolicyLimits::new(
                1,
                2,
                Duration::from_millis(1),
                Duration::from_millis(4),
            )?)?;
            let source = authority.source(0)?;
            let first: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 41001));
            let second: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 41002));
            let envelope = SourceAddressSet::new(&[first, second])?;
            let snapshot = authority.install_source_configuration(&source, envelope.clone())?;
            authority
                .select_initial_round_membership(&snapshot, SourceAddressSet::new(&[first])?)?;
            let config = AddressRoundConfig::new(
                &[first],
                Duration::from_millis(20),
                Duration::from_secs(2),
            )?;
            let (mut prepared, handle) = source.prepare_address_round_refresh(config, &[second])?;
            let original = prepared
                .source_policy
                .take()
                .ok_or(PilotError::InternalState)?;
            let Some(QueryPlan::Lab(plan)) = prepared.refresh.take() else {
                return Err(PilotError::InternalState.into());
            };
            let mut changes = original.resolution_changes()?;
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_time()
                .build()?;
            runtime.block_on(async {
                let (entered, entered_rx) = oneshot::channel();
                let (returned, mut returned_rx) = oneshot::channel();
                let job = tokio::spawn(async move {
                    let _returned = Returned(Some(returned));
                    let _closed = entered.send(());
                    pending::<()>().await;
                    plan
                });
                entered_rx.await?;
                let remaining: Duration = if expected == AddressRoundEnd::Deadline {
                    Duration::from_millis(20)
                } else {
                    Duration::from_secs(2)
                };
                let deadline: Instant = Instant::now() + remaining;
                let (observed, retirement) = tokio::join!(
                    biased;
                    collect(RefreshJob::Lab(job), &original, &mut changes, &mut prepared.control, deadline),
                    async {
                        match expected {
                            AddressRoundEnd::Stale => {
                                let successor =
                                    authority.replace_source_configuration(&snapshot, envelope)?;
                                assert!(!snapshot.same_configuration(&successor));
                            }
                            AddressRoundEnd::Stopped => handle.stop(),
                            _ => {}
                        }
                        Ok::<(), SourcePolicyError>(())
                    }
                );
                retirement?;
                assert!(matches!(observed, Err(RetiredRefresh { outcome: AddressRoundRefreshOutcome::Retired(end), native_pending: false }) if end == expected));
                returned_rx.try_recv()?;
                Ok::<(), Box<dyn Error>>(())
            })?;
            assert!(matches!(
                source.policy_snapshot(second),
                Err(SourcePolicyError::UnknownEndpoint)
            ));
        }
        Ok(())
    }

    /// Reject an original native answer at adoption after equal-valued definition replacement.
    /// # Errors
    /// Preserve native setup, original-round, resolver and authority failures.
    /// # Panics
    /// Reject wrong answer or replacement identity, or stale membership publication.
    #[cfg(windows)]
    #[test]
    #[ignore = "requires an isolated offline Windows process for native DNS custody"]
    #[expect(
        clippy::too_many_lines,
        reason = "keep the original resolver and UDP round ownership through the final adoption case"
    )]
    fn windows_joined_dns_answer_refuses_equal_valued_source_replacement()
    -> Result<(), Box<dyn Error>> {
        use crate::{SourceConfigurationSnapshot, SourcePolicyAuthority};
        use std::{
            io,
            net::{Ipv4Addr, UdpSocket},
            num::NonZeroU16,
        };
        use windows_platform_native::{DnsAddressFamily, DnsQueryName};

        let mut authority: SourcePolicyAuthority = isolated_authority(SourcePolicyLimits::new(
            1,
            2,
            Duration::from_millis(1),
            Duration::from_millis(4),
        )?)?;
        let source: SourceHandle = authority.source(0)?;
        let blackhole: UdpSocket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0))?;
        let candidate: UdpSocket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0))?;
        let initial: SocketAddr = blackhole.local_addr()?;
        let resolved: SocketAddr = candidate.local_addr()?;
        let installed: SourceConfigurationSnapshot = authority
            .install_source_configuration(&source, SourceAddressSet::new(&[initial, resolved])?)?;
        authority
            .select_initial_round_membership(&installed, SourceAddressSet::new(&[initial])?)?;
        let request_timeout: Duration = Duration::from_millis(200);
        let round_timeout: Duration = Duration::from_secs(2);
        let (mut prepared, handle): (PreparedAddressRound, ControlHandle) = authority
            .prepare_windows_address_round_refresh(
                &source,
                AddressRoundConfig::new(&[initial], request_timeout, round_timeout)?,
                DnsQueryName::new("localhost.")?,
                DnsAddressFamily::V4,
                NonZeroU16::new(resolved.port()).ok_or(PilotError::InternalState)?,
            )?;
        let original_packet: [u8; PACKET_BYTES] = prepared.packets[0];
        let original: RoundBindings = prepared
            .source_policy
            .clone()
            .ok_or(PilotError::InternalState)?;
        let permit: OwnedSemaphorePermit = original
            .reserve_resolution_driver()?
            .ok_or(PilotError::InternalState)?;
        let mut changes: watch::Receiver<()> = original.resolution_changes()?;
        let mut control: watch::Receiver<Control> = prepared.control.clone();
        let Some(QueryPlan::Windows(plan)) = prepared.refresh.take() else {
            return Err(PilotError::InternalState.into());
        };
        let runtime: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()?;
        let origin: Instant = Instant::now();
        let deadline: Instant = origin
            .checked_add(round_timeout)
            .ok_or(PilotError::DeadlineOverflow)?;
        let (report, replacement): (AddressRoundReport, SourceConfigurationSnapshot) = runtime
            .block_on(async {
                let job: RefreshJob = RefreshJob::Native(windows::start(
                    *plan,
                    original.clone(),
                    control.clone(),
                    deadline,
                )?);
                let mut report: AddressRoundReport = run_round(prepared, origin, deadline).await;
                if report.round_end() != AddressRoundEnd::Exhausted {
                    let retired: RetiredRefresh =
                        retire(job, AddressRoundRefreshOutcome::NotNeeded).await;
                    if let Some(error) = report.primary_error() {
                        return Err::<_, Box<dyn Error>>(error.into());
                    }
                    return Err(io::Error::other(format!(
                        "original round did not exhaust: {:?}; native pending: {}",
                        report.round_end(),
                        retired.native_pending,
                    ))
                    .into());
                }
                let answer: RefreshPlan =
                    match collect(job, &original, &mut changes, &mut control, deadline).await {
                        Ok(answer) => answer,
                        Err(RetiredRefresh {
                            outcome: AddressRoundRefreshOutcome::Failed(error),
                            ..
                        }) => return Err(error.into()),
                        Err(_) => return Err(io::Error::other("native result did not join").into()),
                    };
                assert_eq!(answer.config.peers, [resolved]);
                let replacement: SourceConfigurationSnapshot =
                    authority.replace_source_scale(&installed, installed.declared_scale())?;
                assert!(!installed.same_configuration(&replacement));
                report.refresh = Some(
                    successor(
                        &original, &permit, answer, control, handle, origin, deadline,
                    )
                    .await,
                );
                Ok((report, replacement))
            })?;

        assert!(matches!(
            source.policy_snapshot(resolved),
            Err(SourcePolicyError::UnknownEndpoint)
        ));
        assert!(
            source
                .address_round_for_configuration(
                    &replacement,
                    AddressRoundConfig::new(&[initial], request_timeout, round_timeout)?,
                )
                .is_ok()
        );

        assert!(source.bind_round_for_refresh(&[initial]).is_ok());

        require_stale_native_dns_outcome(&report, &blackhole, &candidate, &original_packet)
    }

    /// Independently observe the original timeout and both owned UDP sockets.
    /// # Errors
    /// Preserve missing history, unexpected traffic and native socket errors.
    /// # Panics
    /// Reject altered original completion, request bytes or resolver disposition.
    #[cfg(windows)]
    fn require_stale_native_dns_outcome(
        report: &AddressRoundReport,
        blackhole: &std::net::UdpSocket,
        candidate: &std::net::UdpSocket,
        original_packet: &[u8; PACKET_BYTES],
    ) -> Result<(), Box<dyn Error>> {
        use crate::{AddressAttempt, Status};
        use std::io;

        assert_eq!(report.end(), AddressRoundEnd::Exhausted);
        assert!(report.primary_error().is_none());
        assert!(report.policy_refusals().is_empty());
        assert!(matches!(
            report.refresh(),
            Some(AddressRoundRefreshOutcome::Retired(AddressRoundEnd::Stale))
        ));
        let [attempt]: &[AddressAttempt] = report.attempts() else {
            return Err(io::Error::other("expected one original attempt").into());
        };
        assert_eq!(attempt.peer(), blackhole.local_addr()?);
        assert_eq!(attempt.report().status(), Status::Timeout);
        assert!(attempt.report().sent());
        blackhole.set_read_timeout(Some(Duration::from_millis(200)))?;
        let mut wire: [u8; PACKET_BYTES + 1] = [0; PACKET_BYTES + 1];
        let (length, sender): (usize, SocketAddr) = blackhole.recv_from(&mut wire)?;
        assert_eq!(length, PACKET_BYTES);
        assert_eq!(&wire[..length], original_packet);
        assert!(sender.ip().is_loopback());
        for socket in [blackhole, candidate] {
            socket.set_nonblocking(true)?;
            match socket.recv_from(&mut wire) {
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
                Err(error) => return Err(error.into()),
                Ok(_) => {
                    return Err(
                        io::Error::other("unexpected original or successor UDP traffic").into(),
                    );
                }
            }
        }
        Ok(())
    }

    /// Verify that rejected publication preserves both membership and capacity.
    /// # Errors
    /// Return genuine ownership, setup or positive control failure.
    /// # Panics
    /// Reject mutation by the wrong permit, foreign peer, or elapsed cutoff.
    #[test]
    fn adoption_requires_original_permit_envelope_and_cutoff() -> Result<(), Box<dyn Error>> {
        use std::sync::Arc;
        use tokio::sync::Semaphore;

        let mut authority = isolated_authority(SourcePolicyLimits::new(
            1,
            2,
            Duration::from_millis(1),
            Duration::from_millis(4),
        )?)?;
        let source = authority.source(0)?;
        let first: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 41011));
        let successor: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 41012));
        let third: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 41013));
        let outside: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 41014));
        let snapshot = authority.install_source_configuration(
            &source,
            SourceAddressSet::new(&[first, successor, third])?,
        )?;
        authority.select_initial_round_membership(&snapshot, SourceAddressSet::new(&[first])?)?;
        let config =
            AddressRoundConfig::new(&[first], Duration::from_millis(20), Duration::from_secs(2))?;
        let (prepared, _handle) = source.prepare_address_round_refresh(config, &[successor])?;
        let original = prepared
            .source_policy
            .as_ref()
            .ok_or(PilotError::InternalState)?;
        let permit = original
            .reserve_resolution_driver()?
            .ok_or(PilotError::InternalState)?;
        assert!(matches!(
            original.reserve_resolution_driver(),
            Err(PilotError::ResolutionBusy)
        ));
        let foreign = Arc::new(Semaphore::new(1)).try_acquire_owned()?;
        let deadline: Instant = Instant::now() + Duration::from_secs(2);
        let current = prepared.control.borrow();
        assert!(matches!(
            original.adopt_resolution(&[successor], deadline, &foreign, &current),
            Err(PilotError::SourcePolicy(SourcePolicyError::Foreign)),
        ));
        assert!(matches!(
            original.adopt_resolution(&[outside], deadline, &permit, &current),
            Err(PilotError::SourcePolicy(SourcePolicyError::Foreign)),
        ));
        assert!(matches!(
            original.adopt_resolution(&[successor, third], deadline, &permit, &current),
            Err(PilotError::SourcePolicy(SourcePolicyError::Capacity)),
        ));
        assert!(matches!(
            original.adopt_resolution(&[successor], Instant::now(), &permit, &current),
            Err(PilotError::PolicyAdmissionDeadline),
        ));
        for peer in [successor, third] {
            assert!(matches!(
                source.policy_snapshot(peer),
                Err(SourcePolicyError::UnknownEndpoint)
            ));
        }
        original.resolution_current()?;
        let joined = original
            .adopt_resolution(&[successor], deadline, &permit, &current)?
            .ok_or(PilotError::InternalState)?;
        assert!(matches!(
            original.resolution_current(),
            Err(SourcePolicyError::ConfigurationChanged)
        ));
        joined.resolution_current()?;
        Ok(())
    }
}

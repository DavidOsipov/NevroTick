// SPDX-License-Identifier: GPL-2.0-only
//! Join an own-process Running SCM report to one retained, query-only process.

use core::num::NonZeroU32;

#[cfg(windows)]
use std::os::windows::io::BorrowedHandle;

#[cfg(any(windows, test))]
use crate::service_role_sid::TokenModifiedId;
#[cfg(windows)]
use crate::{
    PackageFullNameError, PeerPrimaryToken, PeerPrimaryTokenError, PeerProcess, PeerProcessError,
    PipeServerPidError, ProcessImageNameError, ReportedPackageFullName, ReportedProcessImageName,
    RunningServiceRefusal, ServiceQueryError, ServiceRoleSid, ServiceRoleSidObservation,
    ServiceRoleSidObservationError, ServiceStatusQuery, TokenPrivilegeObservation,
    TokenPrivilegeObservationError, read_connected_pipe_server_pid,
};

/// Refusal while binding or reobserving one reported service process.
#[cfg(windows)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServiceProcessObservationError {
    /// The retained service status query failed.
    Status(ServiceQueryError),
    /// The report did not supply an own-process Running PID.
    IneligibleStatus(RunningServiceRefusal),
    /// Opening the reported process or polling its liveness failed.
    Process(PeerProcessError),
    /// The original service handle reported a different eligible PID.
    ChangedProcess,
}

#[cfg(windows)]
impl core::fmt::Display for ServiceProcessObservationError {
    /// Describe a bounded refusal without including process or service identifiers.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Status(cause) => write!(formatter, "read retained service status: {cause}"),
            Self::IneligibleStatus(reason) => write!(
                formatter,
                "service status did not supply an eligible process ({reason:?})"
            ),
            Self::Process(cause) => write!(formatter, "observe retained process: {cause}"),
            Self::ChangedProcess => formatter.write_str("reported service process changed"),
        }
    }
}

#[cfg(windows)]
impl std::error::Error for ServiceProcessObservationError {}

/// A retained process consistent with samples from one original SCM status handle.
///
/// SCM reports can lag. Matching samples and a live process handle establish
/// neither the identity of an installed component nor the owner of a connected
/// pipe. The IPC receiver must separately admit the actual pipe, process token,
/// component identity and current cutoff, including races before process open.
#[cfg(windows)]
#[must_use = "retain the original process and status query through dependent observations"]
pub struct ObservedServiceProcess<'query> {
    /// The original status-only query, kept open through every observation.
    query: &'query ServiceStatusQuery,
    /// The exact process opened during acquisition.
    process: PeerProcess,
}

#[cfg(windows)]
impl<'query> ObservedServiceProcess<'query> {
    /// Bind a reported Running PID to one limited-query, noninherited process.
    ///
    /// The caller selects the intended service query and owns its synchronous
    /// operation budget. This only reads status and acquires a process handle.
    ///
    /// # Errors
    /// Preserve the original status/open/wait refusal or reject a changed PID.
    pub fn observe(
        query: &'query ServiceStatusQuery,
    ) -> Result<Self, ServiceProcessObservationError> {
        Self::observe_from_query(query, None)
    }

    /// Borrow the original process for independent pipe, token or component checks.
    #[must_use]
    pub const fn process(&self) -> &PeerProcess {
        &self.process
    }

    /// Reobserve the same service and process after dependent observations.
    ///
    /// A refusal consumes this observation and drops its process handle. This call
    /// never reopens a PID or a service. Success describes only the samples made
    /// during the call. The receiving owner still checks its cutoff at use.
    ///
    /// # Errors
    /// Preserve a status or liveness refusal, or reject a changed PID.
    pub fn reobserve(self) -> Result<Self, ServiceProcessObservationError> {
        self.check_current()?;
        Ok(self)
    }

    /// Use the same ordered process observation with an optional original pipe PID.
    /// # Errors
    /// Preserve status, liveness, process open or the caller's pre-open refusal.
    fn observe_from_query<E: From<ServiceProcessObservationError>>(
        query: &'query ServiceStatusQuery,
        pipe: Option<(NonZeroU32, E)>,
    ) -> Result<Self, E> {
        let process: PeerProcess = observe_process(
            || read_running_pid(query).map_err(E::from),
            |pid: NonZeroU32| open_reported_process(pid).map_err(E::from),
            |process: &PeerProcess| check_service_process_live(process).map_err(E::from),
            pipe,
            E::from(ServiceProcessObservationError::ChangedProcess),
        )?;
        Ok(Self { query, process })
    }

    /// Sample the retained SCM report inside the original process liveness checks.
    /// # Errors
    /// Preserve a native failure or a changed or ineligible SCM report.
    fn check_current(&self) -> Result<(), ServiceProcessObservationError> {
        check_process(
            self.process.pid(),
            &self.process,
            || read_running_pid(self.query),
            check_service_process_live,
            ServiceProcessObservationError::ChangedProcess,
        )
    }
}

/// Refusal while observing the primary token of one retained process.
#[cfg(windows)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ProcessPrimaryTokenObservationError {
    /// The retained process exited or its liveness could not be sampled.
    Process(PeerProcessError),
    /// The query-only token or its statistics could not be observed.
    Token(PeerPrimaryTokenError),
    /// The primary-token object or its original modification generation changed.
    ChangedToken,
}

#[cfg(windows)]
impl core::fmt::Display for ProcessPrimaryTokenObservationError {
    /// Report the stage without including process or token identifiers.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Process(cause) => write!(formatter, "observe retained process: {cause}"),
            Self::Token(cause) => write!(formatter, "observe primary token: {cause}"),
            Self::ChangedToken => formatter.write_str("retained process primary token changed"),
        }
    }
}

#[cfg(windows)]
impl std::error::Error for ProcessPrimaryTokenObservationError {}

/// Original query-only token and modification generation on the retained process.
#[cfg(windows)]
struct RetainedPrimaryToken {
    /// Kernel token object observed at acquisition.
    token: PeerPrimaryToken,
    /// Modification generation observed on the original token.
    generation: TokenModifiedId,
}

#[cfg(windows)]
impl RetainedPrimaryToken {
    /// Sample the original token and its modification generation.
    /// # Errors
    /// Preserve the first process or token refusal.
    fn observe(process: &PeerProcess) -> Result<Self, ProcessPrimaryTokenObservationError> {
        let token: PeerPrimaryToken = Self::open(process)?;
        let generation: TokenModifiedId = token
            .generation()
            .map_err(ProcessPrimaryTokenObservationError::Token)?;
        Ok(Self { token, generation })
    }

    /// Compare the current token with the retained object and generation.
    /// # Errors
    /// Preserve a query or liveness failure, or report changed token evidence.
    fn check(&self, process: &PeerProcess) -> Result<(), ProcessPrimaryTokenObservationError> {
        check_token_generation(
            &self.token,
            self.generation,
            || Self::open(process),
            PeerPrimaryToken::same_object,
            |token: &PeerPrimaryToken| {
                token
                    .generation()
                    .map_err(ProcessPrimaryTokenObservationError::Token)
            },
            ProcessPrimaryTokenObservationError::ChangedToken,
        )
    }

    /// Check the original process before acquiring only query rights on its token.
    /// # Errors
    /// Preserve a liveness or native token query refusal.
    fn open(
        process: &PeerProcess,
    ) -> Result<PeerPrimaryToken, ProcessPrimaryTokenObservationError> {
        process
            .check_live()
            .map_err(ProcessPrimaryTokenObservationError::Process)?;
        process
            .retain_primary_token()
            .map_err(ProcessPrimaryTokenObservationError::Token)
    }
}

/// A retained token sample tied to one independently selected process handle.
///
/// A successful recheck compares samples taken during that call. The receiving
/// owner must independently bind the process to the original connection and
/// enforce its deadline, role, installed identity and current authorization.
#[cfg(windows)]
#[must_use = "keep the original token generation for checks at the receiving boundary"]
pub struct ObservedProcessPrimaryToken<'process> {
    /// Original process, never reopened by this observation.
    process: &'process PeerProcess,
    /// Original kernel token and its initial generation.
    primary: RetainedPrimaryToken,
}

#[cfg(windows)]
impl<'process> ObservedProcessPrimaryToken<'process> {
    /// Retain the query-only primary token on the original live process.
    /// # Errors
    /// Preserve the first process or token refusal.
    pub fn observe(
        process: &'process PeerProcess,
    ) -> Result<Self, ProcessPrimaryTokenObservationError> {
        let primary: RetainedPrimaryToken = RetainedPrimaryToken::observe(process)?;
        process
            .check_live()
            .map_err(ProcessPrimaryTokenObservationError::Process)?;
        Ok(Self { process, primary })
    }

    /// Borrow the original process for independent connection checks.
    #[must_use]
    pub const fn process(&self) -> &PeerProcess {
        self.process
    }

    /// Resample the original process's primary-token object and generation.
    /// # Errors
    /// Preserve a query or liveness refusal, or reject changed token evidence.
    pub fn reobserve(&self) -> Result<(), ProcessPrimaryTokenObservationError> {
        self.primary.check(self.process)?;
        self.process
            .check_live()
            .map_err(ProcessPrimaryTokenObservationError::Process)
    }
}

/// Refusal while relating a connected pipe to a retained service process and token.
#[cfg(windows)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ServicePeerObservationError {
    /// The connected pipe failed to report a candidate server PID.
    Pipe(PipeServerPidError),
    /// The SCM report or its retained service process could not be confirmed.
    Service(ServiceProcessObservationError),
    /// The pipe and the SCM process observation disagree.
    PipeProcessMismatch,
    /// A query-only primary-token observation failed.
    Token(PeerPrimaryTokenError),
    /// A fresh token observation did not match the retained object or its original generation.
    ChangedToken,
}

#[cfg(windows)]
impl From<ServiceProcessObservationError> for ServicePeerObservationError {
    /// Retain the original stage within a service observation refusal.
    fn from(error: ServiceProcessObservationError) -> Self {
        Self::Service(error)
    }
}

#[cfg(windows)]
impl From<ProcessPrimaryTokenObservationError> for ServicePeerObservationError {
    /// Retain the original process, token query or continuity refusal.
    fn from(error: ProcessPrimaryTokenObservationError) -> Self {
        match error {
            ProcessPrimaryTokenObservationError::Process(cause) => {
                Self::Service(ServiceProcessObservationError::Process(cause))
            }
            ProcessPrimaryTokenObservationError::Token(cause) => Self::Token(cause),
            ProcessPrimaryTokenObservationError::ChangedToken => Self::ChangedToken,
        }
    }
}

#[cfg(windows)]
impl core::fmt::Display for ServicePeerObservationError {
    /// Report the stage and cause without including observed identifiers.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Pipe(cause) => write!(formatter, "observe connected pipe: {cause}"),
            Self::Service(cause) => write!(formatter, "observe retained service: {cause}"),
            Self::PipeProcessMismatch => formatter.write_str("pipe and service process differ"),
            Self::Token(cause) => write!(formatter, "observe primary token: {cause}"),
            Self::ChangedToken => formatter.write_str("retained service primary token changed"),
        }
    }
}

#[cfg(windows)]
impl std::error::Error for ServicePeerObservationError {}

/// Refusal while observing component names on a retained service peer.
#[cfg(windows)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServicePeerComponentObservationError {
    /// A peer continuity check refused before or after the component reads.
    Peer(ServicePeerObservationError),
    /// The retained process's image name could not be read.
    Image(ProcessImageNameError),
    /// The retained process's package identity could not be read.
    Package(PackageFullNameError),
}

#[cfg(windows)]
impl core::fmt::Display for ServicePeerComponentObservationError {
    /// Format the bounded failure without including a reported component name.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Peer(cause) => write!(formatter, "check service peer continuity: {cause}"),
            Self::Image(cause) => write!(formatter, "observe the peer image name: {cause}"),
            Self::Package(cause) => write!(formatter, "observe the peer package identity: {cause}"),
        }
    }
}

#[cfg(windows)]
impl std::error::Error for ServicePeerComponentObservationError {}

/// Raw image and package names sampled on one retained service process.
///
/// These observations convey no installed component admission or effect
/// authority. The receiver still checks the approved component, role and
/// effective protections and reobserves the peer at its original cutoff.
#[cfg(windows)]
#[must_use = "check the observed component names against the intended installed component"]
pub struct ServicePeerComponentObservation {
    /// Bounded native image name from the retained process.
    image_name: ReportedProcessImageName,
    /// The bounded native package full name, or absence for an unpackaged process.
    package_full_name: Option<ReportedPackageFullName>,
}

#[cfg(windows)]
impl ServicePeerComponentObservation {
    /// Borrow the literal reported image name.
    #[must_use]
    pub const fn image_name(&self) -> &ReportedProcessImageName {
        &self.image_name
    }

    /// Borrow the literal package full name if Windows reported one.
    #[must_use]
    pub const fn package_full_name(&self) -> Option<&ReportedPackageFullName> {
        self.package_full_name.as_ref()
    }
}

/// An observation of matching pipe, SCM and retained process identifiers and token.
///
/// The caller supplies the original connected client and fixed service query.
/// The sampled relation cannot exclude the pre-open PID race or establish
/// installed component identity, token attributes, current pipe I/O or authority.
/// Token continuity describes the observed samples only.
/// The receiving owner enforces its absolute cutoff and effect guards separately.
#[cfg(windows)]
#[must_use = "retain the original pipe, service, process and token for reobservation"]
pub struct ObservedServicePeer<'query, 'pipe> {
    /// Exact client handle, kept borrowed without reconnecting or driving the pipe.
    client: BorrowedHandle<'pipe>,
    /// Original query and query-only process handle.
    service: ObservedServiceProcess<'query>,
    /// Original token and acquired generation, held for continuity and role observation.
    primary: RetainedPrimaryToken,
}

#[cfg(windows)]
impl<'query, 'pipe> ObservedServicePeer<'query, 'pipe> {
    /// Observe a matching server PID before opening the SCM process or its token.
    ///
    /// # Errors
    /// Preserve a pipe, service or token refusal, or reject inconsistent observations.
    pub fn observe(
        query: &'query ServiceStatusQuery,
        client: BorrowedHandle<'pipe>,
    ) -> Result<Self, ServicePeerObservationError> {
        let (service, primary): (ObservedServiceProcess<'query>, RetainedPrimaryToken) =
            observe_service_peer(
                || {
                    read_connected_pipe_server_pid(client)
                        .map_err(ServicePeerObservationError::Pipe)
                },
                |pid: NonZeroU32| {
                    ObservedServiceProcess::observe_from_query(
                        query,
                        Some((pid, ServicePeerObservationError::PipeProcessMismatch)),
                    )
                },
                |service: &ObservedServiceProcess<'query>| {
                    RetainedPrimaryToken::observe(service.process())
                        .map_err(ServicePeerObservationError::from)
                },
                |service: &ObservedServiceProcess<'query>| {
                    service
                        .check_current()
                        .map_err(ServicePeerObservationError::Service)
                },
                ServicePeerObservationError::PipeProcessMismatch,
            )?;
        Ok(Self {
            client,
            service,
            primary,
        })
    }

    /// Borrow the retained process for independent component identity observations.
    #[must_use]
    pub const fn process(&self) -> &PeerProcess {
        self.service.process()
    }

    /// Read both component names on the original process between peer checks.
    ///
    /// Success retains the original pipe, SCM query, process and token. A
    /// missing package name records an unpackaged process. The caller applies
    /// its original cutoff and independently checks installed identity and
    /// current authority before any effect.
    ///
    /// # Errors
    /// Preserve the first peer, image or package refusal. Refusal consumes the
    /// peer and yields no partial observations.
    pub fn observe_component(
        self,
    ) -> Result<(Self, ServicePeerComponentObservation), ServicePeerComponentObservationError> {
        let (peer, image_name, package_full_name): (
            Self,
            ReportedProcessImageName,
            Option<ReportedPackageFullName>,
        ) = observe_component_between_peer_checks(
            self,
            |peer: Self| {
                peer.reobserve()
                    .map_err(ServicePeerComponentObservationError::Peer)
            },
            |peer: &Self| {
                peer.process()
                    .image_name()
                    .map_err(ServicePeerComponentObservationError::Image)
            },
            |peer: &Self| {
                peer.process()
                    .package_full_name()
                    .map_err(ServicePeerComponentObservationError::Package)
            },
        )?;
        Ok((
            peer,
            ServicePeerComponentObservation {
                image_name,
                package_full_name,
            },
        ))
    }

    /// Read the expected role SID on the original primary token at its acquired generation.
    ///
    /// The receiving owner still reobserves the peer at its admission cutoffs
    /// and binds these samples to current installed component and actor policy.
    ///
    /// # Errors
    /// Preserve the earliest statistics or SID-list refusal, or report a changed token.
    pub fn observe_role_sid<'sid>(
        &self,
        expected: ServiceRoleSid<'sid>,
    ) -> Result<ServiceRoleSidObservation<'sid>, ServiceRoleSidObservationError> {
        self.primary
            .token
            .observe_role_sid(expected, self.primary.generation)
    }

    /// Capture the original peer token's privileges at its acquisition generation.
    ///
    /// The receiving owner decodes and compares the complete profile, binds the
    /// installed component evidence and reobserves the peer at the effect boundary.
    /// # Errors
    /// Preserve the first query or extent refusal, or report a changed generation.
    pub fn observe_privileges(
        &self,
    ) -> Result<TokenPrivilegeObservation, TokenPrivilegeObservationError> {
        self.primary
            .token
            .observe_privileges_at(self.primary.generation)
    }

    /// Reobserve the original pipe, service, process, primary-token object and generation.
    ///
    /// Refusal consumes the observation. This call never reopens a process or
    /// service, reconnects a pipe or substitutes the freshly queried token.
    ///
    /// # Errors
    /// Preserve the original pipe, service or token refusal and reject any mismatch.
    pub fn reobserve(self) -> Result<Self, ServicePeerObservationError> {
        observe_between_pipe_samples(
            self.process().pid(),
            || {
                read_connected_pipe_server_pid(self.client)
                    .map_err(ServicePeerObservationError::Pipe)
            },
            || {
                self.primary
                    .check(self.process())
                    .map_err(ServicePeerObservationError::from)
            },
            || {
                self.service
                    .check_current()
                    .map_err(ServicePeerObservationError::Service)
            },
            ServicePeerObservationError::PipeProcessMismatch,
        )?;
        Ok(self)
    }
}

/// Publish both checked name reads only after a second check on the owned peer.
/// # Errors
/// Preserve the first refusal and consume the original peer and any names read.
#[cfg(any(windows, test))]
fn observe_component_between_peer_checks<P, I, N, E>(
    peer: P,
    mut reobserve: impl FnMut(P) -> Result<P, E>,
    read_image: impl FnOnce(&P) -> Result<I, E>,
    read_package: impl FnOnce(&P) -> Result<Option<N>, E>,
) -> Result<(P, I, Option<N>), E> {
    let peer: P = reobserve(peer)?;
    let image: I = read_image(&peer)?;
    let package: Option<N> = read_package(&peer)?;
    let current: P = reobserve(peer)?;
    Ok((current, image, package))
}

/// Require a fresh reference to the same object, then sample the original token's generation.
/// # Errors
/// Preserve acquisition or statistics failure, or reject a changed object or generation.
#[cfg(any(windows, test))]
fn check_token_generation<T, E>(
    original: &T,
    generation: TokenModifiedId,
    acquire_current: impl FnOnce() -> Result<T, E>,
    same_object: impl FnOnce(&T, &T) -> bool,
    sample_generation: impl FnOnce(&T) -> Result<TokenModifiedId, E>,
    changed: E,
) -> Result<(), E> {
    let current: T = acquire_current()?;
    if !same_object(original, &current) || sample_generation(original)? != generation {
        return Err(changed);
    }
    Ok(())
}

/// Acquire limited query and synchronization access to the reported process.
/// # Errors
/// Preserve the original native process open or PID refusal.
#[cfg(windows)]
fn open_reported_process(pid: NonZeroU32) -> Result<PeerProcess, ServiceProcessObservationError> {
    PeerProcess::open(pid).map_err(ServiceProcessObservationError::Process)
}

/// Check liveness on the original process handle.
/// # Errors
/// Preserve the original exit or wait refusal.
#[cfg(windows)]
fn check_service_process_live(process: &PeerProcess) -> Result<(), ServiceProcessObservationError> {
    process
        .check_live()
        .map_err(ServiceProcessObservationError::Process)
}

/// Require agreement with the originally observed server PID.
/// # Errors
/// Return the caller's typed mismatch when the identifiers differ.
#[cfg(any(windows, test))]
fn require_same_pipe_process<E>(
    reported: NonZeroU32,
    original: NonZeroU32,
    mismatch: E,
) -> Result<(), E> {
    if reported == original {
        Ok(())
    } else {
        Err(mismatch)
    }
}

/// Join the original pipe and service observations before acquiring any token.
/// # Errors
/// Preserve the initial pipe or process refusal and all subsequent observation refusals.
#[cfg(any(windows, test))]
fn observe_service_peer<P, T, E: Copy>(
    mut pipe_pid: impl FnMut() -> Result<NonZeroU32, E>,
    observe_process: impl FnOnce(NonZeroU32) -> Result<P, E>,
    observe_token: impl FnOnce(&P) -> Result<T, E>,
    check_service: impl FnOnce(&P) -> Result<(), E>,
    mismatch: E,
) -> Result<(P, T), E> {
    let original: NonZeroU32 = pipe_pid()?;
    let process: P = observe_process(original)?;
    let token: T = observe_between_pipe_samples(
        original,
        pipe_pid,
        || observe_token(&process),
        || check_service(&process),
        mismatch,
    )?;
    Ok((process, token))
}

/// Bracket a token observation and current SCM sample with the original pipe PID.
/// # Errors
/// Preserve the first refusal and yield no observation after a changed pipe.
#[cfg(any(windows, test))]
fn observe_between_pipe_samples<T, E: Copy>(
    original: NonZeroU32,
    mut pipe_pid: impl FnMut() -> Result<NonZeroU32, E>,
    observe_token: impl FnOnce() -> Result<T, E>,
    check_service: impl FnOnce() -> Result<(), E>,
    mismatch: E,
) -> Result<T, E> {
    require_same_pipe_process(pipe_pid()?, original, mismatch)?;
    let token: T = observe_token()?;
    check_service()?;
    require_same_pipe_process(pipe_pid()?, original, mismatch)?;
    Ok(token)
}

/// Read the candidate PID through the original service handle.
/// # Errors
/// Preserve the native query refusal or the checked status refusal.
#[cfg(windows)]
fn read_running_pid(
    query: &ServiceStatusQuery,
) -> Result<NonZeroU32, ServiceProcessObservationError> {
    let status: crate::ReportedServiceStatus = query
        .query()
        .map_err(ServiceProcessObservationError::Status)?;
    status
        .reported_running_process()
        .map_err(ServiceProcessObservationError::IneligibleStatus)
}

/// Check any original pipe PID, then open the matching candidate exactly once.
/// # Errors
/// Propagate the first observation/open refusal or the explicit PID mismatch.
fn observe_process<P, E>(
    mut read_running_pid: impl FnMut() -> Result<NonZeroU32, E>,
    open: impl FnOnce(NonZeroU32) -> Result<P, E>,
    check_live: impl FnMut(&P) -> Result<(), E>,
    pipe: Option<(NonZeroU32, E)>,
    changed: E,
) -> Result<P, E> {
    let pid: NonZeroU32 = read_running_pid()?;
    if let Some((original, mismatch)) = pipe {
        require_same_pipe_process(pid, original, mismatch)?;
    }
    let process: P = open(pid)?;
    check_process(pid, &process, read_running_pid, check_live, changed)?;
    Ok(process)
}

/// Require liveness on both sides of a fresh status sample for the original PID.
/// # Errors
/// Preserve the first query/wait refusal or reject a changed PID without proceeding.
fn check_process<P, E>(
    pid: NonZeroU32,
    process: &P,
    mut read_running_pid: impl FnMut() -> Result<NonZeroU32, E>,
    mut check_live: impl FnMut(&P) -> Result<(), E>,
    changed: E,
) -> Result<(), E> {
    check_live(process)?;
    if read_running_pid()? != pid {
        return Err(changed);
    }
    check_live(process)
}

#[cfg(test)]
mod tests;

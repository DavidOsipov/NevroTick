// SPDX-License-Identifier: GPL-2.0-only
//! Join an own-process Running SCM report to one retained, query-only process.

use core::num::NonZeroU32;

#[cfg(windows)]
use crate::{
    PeerProcess, PeerProcessError, RunningServiceRefusal, ServiceQueryError, ServiceStatusQuery,
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
        let process: PeerProcess = observe_process(
            || read_running_pid(query),
            |pid: NonZeroU32| {
                PeerProcess::open(pid).map_err(ServiceProcessObservationError::Process)
            },
            |process: &PeerProcess| {
                process
                    .check_live()
                    .map_err(ServiceProcessObservationError::Process)
            },
            ServiceProcessObservationError::ChangedProcess,
        )?;
        Ok(Self { query, process })
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
        check_process(
            self.process.pid(),
            &self.process,
            || read_running_pid(self.query),
            |process: &PeerProcess| {
                process
                    .check_live()
                    .map_err(ServiceProcessObservationError::Process)
            },
            ServiceProcessObservationError::ChangedProcess,
        )?;
        Ok(self)
    }
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

/// Open exactly the reported candidate once, then observe its continued consistency.
/// # Errors
/// Propagate the first observation/open refusal or the explicit changed-PID error.
fn observe_process<P, E>(
    mut read_running_pid: impl FnMut() -> Result<NonZeroU32, E>,
    open: impl FnOnce(NonZeroU32) -> Result<P, E>,
    check_live: impl FnMut(&P) -> Result<(), E>,
    changed: E,
) -> Result<P, E> {
    let pid: NonZeroU32 = read_running_pid()?;
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

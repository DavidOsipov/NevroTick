// SPDX-License-Identifier: GPL-2.0-only
//! Source-owned laboratory KE results after actual worker retirement.

use std::error::Error;
use std::fmt;

use super::{
    Failed, Failure, InvocationCompletion, RunFailed, failure_labels, handoff, io_label, job,
};
use crate::{SourceConfigurationSnapshot, SourcePolicyError};

/// A genuine managed Preview completed and its actual worker returned and joined.
///
/// The retained configuration is historical. It grants no current permission,
/// authenticated sample, protected UDP transmission or clock authority.
#[must_use = "inspect or retain the joined historical Preview result"]
pub struct SourceKePreviewReport {
    /// Only the private packet commit and normal joined receiver can issue this.
    history: Box<handoff::ManagedPreviewHistory>,
}

impl SourceKePreviewReport {
    /// Borrow the exact configuration checked by the original final packet commit.
    #[must_use]
    pub const fn configuration(&self) -> &SourceConfigurationSnapshot {
        self.history.configuration()
    }

    /// Return the complete checked cookie population before Preview preparation.
    #[must_use]
    pub const fn cookie_count(&self) -> usize {
        self.history.summary().cookies
    }

    /// Return the checked aggregate bytes in that original cookie population.
    #[must_use]
    pub const fn cookie_bytes(&self) -> usize {
        self.history.summary().cookie_bytes
    }

    /// Return the actual sealed request length; the packet was never transmitted.
    #[must_use]
    pub const fn packet_bytes(&self) -> usize {
        self.history.summary().packet_bytes
    }
}

impl fmt::Debug for SourceKePreviewReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SourceKePreviewReport { joined: true, transmitted: false, .. }")
    }
}

/// A bounded source, submission, operation or actual worker-custody failure.
///
/// Primary and cleanup observations remain distinct. A failed join or collection
/// supplies no normal retirement acknowledgement and triggers no automatic retry.
#[must_use = "inspect the original Preview failure and independent cleanup observations"]
pub struct SourceKePreviewFailure {
    /// Preserve the actual complete typed disposition without exposing native text.
    failure: RunFailed,
}

impl SourceKePreviewFailure {
    /// Return a closed stage and cause for the actual outer disposition.
    #[must_use]
    pub fn labels(&self) -> (&'static str, &'static str) {
        match &self.failure {
            RunFailed::CallerContext => ("caller-context", "entered-runtime"),
            RunFailed::Preparation(operation) | RunFailed::Operation(operation) => {
                failure_labels(operation.primary)
            }
            RunFailed::Submission(submission) => (
                "worker-submission",
                match submission {
                    job::SubmitError::Busy => "busy",
                    job::SubmitError::Used => "used",
                    job::SubmitError::Control => "control",
                    job::SubmitError::Spawn(kind) => io_label(*kind),
                },
            ),
            RunFailed::Collection(collection) => (
                "worker-collection",
                match collection {
                    job::CollectError::Busy => "busy",
                    job::CollectError::NotReady => "not-ready",
                    job::CollectError::SelfJoin => "self-join",
                    job::CollectError::EnteredRuntime => "entered-runtime",
                    job::CollectError::Control => "control",
                },
            ),
            RunFailed::Abnormal(joined) => (
                "worker-termination",
                match joined.termination {
                    job::Termination::Panicked => "panicked",
                    job::Termination::Contract(_) => "contract",
                    job::Termination::Returned => "unconfirmed-retirement",
                },
            ),
        }
    }

    /// Borrow the original operation failure even when later retirement failed.
    fn operation_failure(&self) -> Option<&Failed> {
        match &self.failure {
            RunFailed::Preparation(failure) | RunFailed::Operation(failure) => Some(failure),
            RunFailed::Abnormal(joined) => match &joined.operation {
                Some(Err(failure)) => Some(failure),
                Some(Ok(_)) | None => None,
            },
            RunFailed::CallerContext | RunFailed::Submission(_) | RunFailed::Collection(_) => None,
        }
    }

    /// Return the original operation's closed labels, separate from worker failure.
    #[must_use]
    pub fn primary_labels(&self) -> Option<(&'static str, &'static str)> {
        self.operation_failure()
            .map(|failure: &Failed| failure_labels(failure.primary))
    }

    /// Return independent TLS cleanup refusal without replacing the primary cause.
    #[must_use]
    pub fn cleanup_labels(&self) -> Option<(&'static str, &'static str)> {
        self.operation_failure()
            .and_then(|failure: &Failed| failure.cleanup)
            .map(failure_labels)
    }

    /// Return independent publication-admission cleanup refusal, when observed.
    #[must_use]
    pub fn admission_cleanup_labels(&self) -> Option<(&'static str, &'static str)> {
        self.operation_failure()
            .and_then(|failure: &Failed| failure.admission_cleanup)
            .map(failure_labels)
    }

    /// Retain a typed source refusal without exposing configured names or endpoints.
    #[must_use]
    pub fn source_policy_error(&self) -> Option<SourcePolicyError> {
        match self
            .operation_failure()
            .map(|failure: &Failed| failure.primary)
        {
            Some(Failure::SourcePolicy(error)) => Some(error),
            Some(_) | None => None,
        }
    }

    /// Borrow a real committed configuration recorded before abnormal retirement.
    /// This preserves history but supplies no normal joined success report.
    #[must_use]
    pub fn retained_configuration(&self) -> Option<&SourceConfigurationSnapshot> {
        let Some(InvocationCompletion::ManagedPreview(history)) = self.retained_completion() else {
            return None;
        };
        Some(history.configuration())
    }

    /// Borrow only the real sealed operation success from an abnormal joined outcome.
    fn retained_completion(&self) -> Option<&InvocationCompletion> {
        let RunFailed::Abnormal(joined) = &self.failure else {
            return None;
        };
        match &joined.operation {
            Some(Ok(completion)) => Some(completion),
            Some(Err(_)) | None => None,
        }
    }
}

impl fmt::Debug for SourceKePreviewFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (stage, cause): (&'static str, &'static str) = self.labels();
        formatter
            .debug_struct("SourceKePreviewFailure")
            .field("stage", &stage)
            .field("cause", &cause)
            .finish_non_exhaustive()
    }
}

impl fmt::Display for SourceKePreviewFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (stage, cause): (&'static str, &'static str) = self.labels();
        write!(formatter, "source KE Preview refused: {stage}/{cause}")
    }
}

impl Error for SourceKePreviewFailure {}

/// One explicitly source-authorized protected laboratory exchange, after normal join.
#[must_use = "retain the source revision of the joined laboratory exchange"]
pub struct SourceProtectedProbeReport {
    /// Exact original definition returned only after commit and normal join.
    configuration: SourceConfigurationSnapshot,
    /// Observed local protected request extent, without peer-delivery claims.
    packet_bytes: usize,
}

impl SourceProtectedProbeReport {
    /// The configuration admitted at the original protected reply commit.
    #[must_use]
    pub const fn configuration(&self) -> &SourceConfigurationSnapshot {
        &self.configuration
    }

    /// The exact protected request length submitted by the local adapter.
    #[must_use]
    pub const fn packet_bytes(&self) -> usize {
        self.packet_bytes
    }
}

impl fmt::Debug for SourceProtectedProbeReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SourceProtectedProbeReport { joined: true, transmitted: true, .. }")
    }
}

/// Bounded source, worker or operation refusal from the explicit protected probe.
pub struct SourceProtectedProbeFailure(
    /// Shared typed worker and operation failure projection.
    SourceKePreviewFailure,
);

impl SourceProtectedProbeFailure {
    /// Preserve the actual worker or operation failure without projecting success.
    pub(super) const fn from_run_failure(failure: RunFailed) -> Self {
        Self(SourceKePreviewFailure { failure })
    }

    /// The closed labels for the original outer failure.
    #[must_use]
    pub fn labels(&self) -> (&'static str, &'static str) {
        self.0.labels()
    }

    /// The original operation failure even if worker retirement also fails.
    #[must_use]
    pub fn primary_labels(&self) -> Option<(&'static str, &'static str)> {
        self.0.primary_labels()
    }

    /// The independent TLS cleanup refusal, when observed.
    #[must_use]
    pub fn cleanup_labels(&self) -> Option<(&'static str, &'static str)> {
        self.0.cleanup_labels()
    }

    /// The independent operation-admission cleanup refusal, when observed.
    #[must_use]
    pub fn admission_cleanup_labels(&self) -> Option<(&'static str, &'static str)> {
        self.0.admission_cleanup_labels()
    }

    /// The original typed source policy refusal when that owner denied the operation.
    #[must_use]
    pub fn source_policy_error(&self) -> Option<SourcePolicyError> {
        self.0.source_policy_error()
    }

    /// Borrow the committed revision from a sealed managed result after abnormal join.
    /// This is historical data and grants no current authority or joined success.
    #[must_use]
    pub fn retained_configuration(&self) -> Option<&SourceConfigurationSnapshot> {
        let Some(InvocationCompletion::ManagedProtected(history)) = self.0.retained_completion()
        else {
            return None;
        };
        Some(history.configuration())
    }
}

impl fmt::Debug for SourceProtectedProbeFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (stage, cause): (&'static str, &'static str) = self.labels();
        formatter
            .debug_struct("SourceProtectedProbeFailure")
            .field("stage", &stage)
            .field("cause", &cause)
            .finish_non_exhaustive()
    }
}

impl fmt::Display for SourceProtectedProbeFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (stage, cause): (&'static str, &'static str) = self.labels();
        write!(formatter, "source protected probe refused: {stage}/{cause}")
    }
}

impl Error for SourceProtectedProbeFailure {}

#[cfg(windows)]
impl crate::SourcePolicyAuthority {
    /// Run one explicit protected loopback exchange at the authority's original revision.
    ///
    /// The installed definition supplies TLS and UDP inputs. Shares the original
    /// experimental worker slot and permits no retry, clock effect or plain fallback.
    ///
    /// # Errors
    /// Preserve configuration, source/endpoint, caller, worker and exchange refusals.
    pub fn probe_nts_protected(
        &self,
        expected: &SourceConfigurationSnapshot,
    ) -> Result<SourceProtectedProbeReport, SourceProtectedProbeFailure> {
        let source: crate::source_policy::ProtectedKeSource = self
            .authorize_protected_ke(expected)
            .map_err(|error: SourcePolicyError| {
                SourceProtectedProbeFailure::from_run_failure(RunFailed::Preparation(Failed::from(
                    Failure::SourcePolicy(error),
                )))
            })?;
        let config: super::Config = super::Config::from_managed_protected(source);
        let result: super::datagram::ReceivedCompletion =
            super::run(config).map_err(SourceProtectedProbeFailure::from_run_failure)?;
        let (configuration, summary): (SourceConfigurationSnapshot, super::Summary) = result
            .into_managed_protected()
            .map_err(|failure: Failure| {
                SourceProtectedProbeFailure::from_run_failure(RunFailed::Operation(Failed::from(
                    failure,
                )))
            })?;
        Ok(SourceProtectedProbeReport {
            configuration,
            packet_bytes: summary.packet_bytes,
        })
    }
}

#[cfg(windows)]
impl crate::SourceHandle {
    /// Exercise one source-owned KE Preview through the actual synchronous worker.
    ///
    /// The installed target supplies TLS inputs. The selected UDP endpoint is
    /// checked against the original installed set but is never sent a datagram.
    /// Only normal joined retirement produces a report; no automatic retry occurs.
    ///
    /// # Errors
    /// Refuse absent/replaced/restricted configuration, an entered Tokio runtime,
    /// spent process admission, actual KE failure or unconfirmed worker custody.
    pub fn preview_nts_ke(&self) -> Result<SourceKePreviewReport, SourceKePreviewFailure> {
        let context: crate::source_policy::ConfiguredKePreview = self
            .capture_ke_preview()
            .map_err(|error: SourcePolicyError| SourceKePreviewFailure {
                failure: RunFailed::Preparation(Failed::from(Failure::SourcePolicy(error))),
            })?;
        let config: super::Config = super::Config::from_managed_preview(context);
        let received: super::datagram::ReceivedCompletion =
            super::run(config).map_err(|failure: RunFailed| SourceKePreviewFailure { failure })?;
        let history: Box<handoff::ManagedPreviewHistory> = received
            .into_managed_preview()
            .map_err(|failure: Failure| SourceKePreviewFailure {
                failure: RunFailed::Operation(Failed::from(failure)),
            })?;
        Ok(SourceKePreviewReport { history })
    }
}

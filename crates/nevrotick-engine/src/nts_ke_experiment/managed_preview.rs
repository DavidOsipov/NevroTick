// SPDX-License-Identifier: GPL-2.0-only
//! Source-owned laboratory KE results after actual worker retirement.

use std::error::Error;
use std::fmt;
use std::num::NonZeroU32;
use std::time::{Duration, Instant};

use super::{
    Failed, Failure, InvocationCompletion, ObservedInvocation, ObserverStartError,
    PowerObserverCleanup, RunFailed, StopHandle, failure_labels, handoff, io_label, job,
    receive_probe,
};
use crate::{
    ExperimentalMeasurement, MeasurementRefusal, SourceConfigurationSnapshot, SourcePolicyError,
};

/// A genuine managed Preview completed and its actual worker returned and joined.
///
/// The retained configuration is historical. It grants no current permission,
/// authenticated sample, protected UDP transmission or clock authority.
#[must_use = "inspect or retain the joined historical Preview result"]
pub struct SourceKePreviewReport {
    /// Only the private packet commit and normal joined receiver can issue this.
    history: Box<handoff::ManagedPreviewHistory>,
    /// Exact nonzero close status, independent of the joined outcome.
    power_unregistration_error: Option<NonZeroU32>,
}

impl SourceKePreviewReport {
    /// Consume the genuine normally joined managed Preview result.
    /// # Errors
    /// Preserve the original run refusal or reject another completion mode.
    pub(super) fn from_run(
        result: Result<super::datagram::ReceivedCompletion, RunFailed>,
        cleanup: PowerObserverCleanup,
    ) -> Result<Self, SourceKePreviewFailure> {
        let received: super::datagram::ReceivedCompletion =
            result.map_err(|failure: RunFailed| SourceKePreviewFailure { failure, cleanup })?;
        let history: Box<handoff::ManagedPreviewHistory> = received
            .into_managed_preview()
            .map_err(|failure: Failure| SourceKePreviewFailure {
                failure: RunFailed::Operation(Failed::from(failure)),
                cleanup,
            })?;
        Ok(Self {
            history,
            power_unregistration_error: cleanup.unregistration_error,
        })
    }

    /// Return the exact nonzero power observer unregistration status on failure.
    #[must_use]
    pub const fn power_unregistration_error(&self) -> Option<NonZeroU32> {
        self.power_unregistration_error
    }

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
    /// Independent cleanup after successful power observer registration.
    cleanup: PowerObserverCleanup,
}

impl SourceKePreviewFailure {
    /// Whether requesting Stop after an unconfirmed collection failed.
    #[must_use]
    pub const fn power_stop_failed(&self) -> bool {
        self.cleanup.stop_failed
    }

    /// Return the exact nonzero power observer unregistration status on failure.
    #[must_use]
    pub const fn power_unregistration_error(&self) -> Option<NonZeroU32> {
        self.cleanup.unregistration_error
    }

    /// Return a closed stage and cause for the actual outer disposition.
    #[must_use]
    pub fn labels(&self) -> (&'static str, &'static str) {
        match &self.failure {
            RunFailed::CallerContext => ("caller-context", "entered-runtime"),
            RunFailed::PowerRegistration => ("power-registration", "unavailable"),
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
            RunFailed::CallerContext
            | RunFailed::PowerRegistration
            | RunFailed::Submission(_)
            | RunFailed::Collection(_) => None,
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
    /// Original authenticated peer leap indicator, with unsynchronized replies excluded.
    leap_indicator: u8,
    /// Original monotonic observation admitted by the guarded terminal commit.
    committed_at: Instant,
    /// Checked elapsed monotonic duration from original receive trailing to commit.
    receive_to_commit_delay: Option<Duration>,
    /// Inclusive elapsed bounds from the original application wall-read brackets.
    application_wall_elapsed_bounds: Option<[Duration; 2]>,
    /// Original conditional arithmetic or refusal from the same joined packet history.
    measurement: Result<ExperimentalMeasurement, MeasurementRefusal>,
    /// A failed observer close leaves the original joined report intact.
    power_unregistration_error: Option<NonZeroU32>,
}

impl SourceProtectedProbeReport {
    /// Consume only the genuine normally joined managed Protected result.
    /// # Errors
    /// Refuse another completion mode without manufacturing managed history.
    fn from_completion(
        result: super::datagram::ReceivedCompletion,
        cleanup: PowerObserverCleanup,
    ) -> Result<Self, Failure> {
        let (
            configuration,
            summary,
            leap_indicator,
            committed_at,
            receive_to_commit_delay,
            application_wall_elapsed_bounds,
            measurement,
        ): super::datagram::ManagedProtectedProjection = result.into_managed_protected()?;
        Ok(Self {
            configuration,
            packet_bytes: summary.packet_bytes,
            leap_indicator,
            committed_at,
            receive_to_commit_delay,
            application_wall_elapsed_bounds,
            measurement,
            power_unregistration_error: cleanup.unregistration_error,
        })
    }

    /// Keep the actual worker outcome and independently observed power cleanup.
    ///
    /// # Errors
    /// Preserve pre-submission, worker or joined-mode refusal with its cleanup facts.
    pub(super) fn from_observed(
        observed: Result<ObservedInvocation, ObserverStartError>,
    ) -> Result<Self, SourceProtectedProbeFailure> {
        let ObservedInvocation { result, cleanup }: ObservedInvocation =
            observed.map_err(|error: ObserverStartError| {
                SourceProtectedProbeFailure::from_run_failure(match error {
                    ObserverStartError::Preparation(failure) => failure,
                    ObserverStartError::Registration => RunFailed::PowerRegistration,
                })
            })?;
        let completion: super::datagram::ReceivedCompletion = receive_probe(result)
            .map_err(|failure| SourceProtectedProbeFailure::with_cleanup(failure, cleanup))?;
        Self::from_completion(completion, cleanup).map_err(|failure| {
            SourceProtectedProbeFailure::with_cleanup(
                RunFailed::Operation(Failed::from(failure)),
                cleanup,
            )
        })
    }

    /// Return the exact nonzero power observer unregistration status on failure.
    /// An observer failure does not change the admitted configuration or measurement.
    #[must_use]
    pub const fn power_unregistration_error(&self) -> Option<NonZeroU32> {
        self.power_unregistration_error
    }

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

    /// Return the original authenticated reply's leap indicator (0, 1 or 2).
    ///
    /// This historical peer claim is independent of source scale and measurement
    /// availability. It grants no calendar, current sample or actuation authority.
    #[must_use]
    pub const fn leap_indicator(&self) -> u8 {
        self.leap_indicator
    }

    /// Return the process-local monotonic observation accepted by the original
    /// guarded reply commit. The measurement keeps its original packet receipt
    /// as its age reference. This historical observation grants no current authority.
    #[must_use]
    pub const fn committed_at(&self) -> Instant {
        self.committed_at
    }

    /// Return the checked monotonic interval from the original receive bracket's
    /// trailing observation to the guarded terminal decision, when representable.
    /// This historical delay is independent of numerical measurement or eligibility.
    #[must_use]
    pub const fn receive_to_commit_delay(&self) -> Option<Duration> {
        self.receive_to_commit_delay
    }

    /// Return the inclusive minimum and maximum monotonic elapsed times between
    /// the original pre-send and receive-return wall reads of this joined exchange.
    /// The minimum pairs the leading receive reading with the trailing pre-send
    /// reading. The maximum pairs the trailing receive and leading pre-send readings.
    ///
    /// The terminal observation participates in order checking. Unavailable bounds
    /// leave the historical exchange and its conditional measurement intact.
    #[must_use]
    pub const fn application_wall_elapsed_bounds(&self) -> Option<[Duration; 2]> {
        self.application_wall_elapsed_bounds
    }

    /// Borrow the original conditional nominal arithmetic, without refreshing its age.
    /// Neither success nor protected completion supplies current sample eligibility.
    ///
    /// # Errors
    /// Preserve the original absent mapping, capture, date, protocol or arithmetic refusal.
    pub fn measurement(&self) -> Result<&ExperimentalMeasurement, MeasurementRefusal> {
        self.measurement.as_ref().map_err(|reason| *reason)
    }
}

/// Forward actual joined component results through the private production projection.
/// # Errors
/// Preserve refusal of an unmanaged or no-send completion.
#[cfg(test)]
pub(super) fn protected_report(
    result: super::datagram::ReceivedCompletion,
) -> Result<SourceProtectedProbeReport, Failure> {
    SourceProtectedProbeReport::from_completion(result, PowerObserverCleanup::default())
}

impl fmt::Debug for SourceProtectedProbeReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SourceProtectedProbeReport { joined: true, transmitted: true, .. }")
    }
}

/// Bounded source, worker or operation refusal from the explicit protected probe.
#[must_use = "inspect the original Protected failure and independent cleanup observations"]
pub struct SourceProtectedProbeFailure {
    /// Shared typed worker and operation failure projection.
    worker: SourceKePreviewFailure,
}

impl SourceProtectedProbeFailure {
    /// Preserve the actual worker or operation failure without projecting success.
    pub(super) fn from_run_failure(failure: RunFailed) -> Self {
        Self::with_cleanup(failure, PowerObserverCleanup::default())
    }

    /// Attach the observed secondary facts without changing the original failure.
    fn with_cleanup(failure: RunFailed, cleanup: PowerObserverCleanup) -> Self {
        Self {
            worker: SourceKePreviewFailure { failure, cleanup },
        }
    }

    /// Whether requesting Stop after an unconfirmed collection failed.
    #[must_use]
    pub const fn power_stop_failed(&self) -> bool {
        self.worker.power_stop_failed()
    }

    /// Return the exact nonzero power observer unregistration status on failure.
    #[must_use]
    pub const fn power_unregistration_error(&self) -> Option<NonZeroU32> {
        self.worker.power_unregistration_error()
    }

    /// The closed labels for the original outer failure.
    #[must_use]
    pub fn labels(&self) -> (&'static str, &'static str) {
        self.worker.labels()
    }

    /// The original operation failure even if worker retirement also fails.
    #[must_use]
    pub fn primary_labels(&self) -> Option<(&'static str, &'static str)> {
        self.worker.primary_labels()
    }

    /// The independent TLS cleanup refusal, when observed.
    #[must_use]
    pub fn cleanup_labels(&self) -> Option<(&'static str, &'static str)> {
        self.worker.cleanup_labels()
    }

    /// The independent operation-admission cleanup refusal, when observed.
    #[must_use]
    pub fn admission_cleanup_labels(&self) -> Option<(&'static str, &'static str)> {
        self.worker.admission_cleanup_labels()
    }

    /// The original typed source policy refusal when that owner denied the operation.
    #[must_use]
    pub fn source_policy_error(&self) -> Option<SourcePolicyError> {
        self.worker.source_policy_error()
    }

    /// Borrow the committed revision from a sealed managed result after abnormal join.
    /// This is historical data and grants no current authority or joined success.
    #[must_use]
    pub fn retained_configuration(&self) -> Option<&SourceConfigurationSnapshot> {
        let Some(InvocationCompletion::ManagedProtected(history)) =
            self.worker.retained_completion()
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
    /// A delivered suspend or automatic-resume event requests Stop on that invocation.
    ///
    /// # Errors
    /// Preserve source, caller, worker, observer registration and exchange refusals.
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
        SourceProtectedProbeReport::from_observed(super::run_with_power_observer(config))
    }
}

#[cfg(windows)]
impl crate::SourceHandle {
    /// Exercise one source-owned KE Preview through the actual synchronous worker.
    ///
    /// The installed target supplies TLS inputs. The selected UDP endpoint is
    /// checked against the original installed set but is never sent a datagram.
    /// A delivered suspend or automatic-resume event requests Stop on that invocation.
    /// Only normal joined retirement produces a report; no automatic retry occurs.
    ///
    /// # Errors
    /// Refuse absent/replaced/restricted configuration, an entered Tokio runtime,
    /// spent process admission, observer registration failure, actual KE failure
    /// or unconfirmed worker custody.
    pub fn preview_nts_ke(&self) -> Result<SourceKePreviewReport, SourceKePreviewFailure> {
        self.preview_nts_ke_with_observer(
            &super::PROCESS_WORKER,
            super::subscribe_power_observer,
            job::SubmissionReservation::try_submit,
        )
    }
}

#[cfg(any(windows, test))]
impl crate::SourceHandle {
    /// Capture the original issued definition and run its single synchronous Preview.
    /// # Errors
    /// Preserve source, observer registration, submission, collection and joined
    /// operation refusals, including independent power observer cleanup.
    pub(super) fn preview_nts_ke_with_observer<Close: FnOnce() -> Result<(), NonZeroU32>>(
        &self,
        slot: &'static job::WorkerSlot,
        subscribe: impl FnOnce(StopHandle) -> Result<Close, ()>,
        submit: impl FnOnce(job::SubmissionReservation) -> Result<(), job::SubmitError>,
    ) -> Result<SourceKePreviewReport, SourceKePreviewFailure> {
        let context: crate::source_policy::ConfiguredKePreview = self
            .capture_ke_preview()
            .map_err(|error: SourcePolicyError| SourceKePreviewFailure {
                failure: RunFailed::Preparation(Failed::from(Failure::SourcePolicy(error))),
                cleanup: PowerObserverCleanup::default(),
            })?;
        let config: super::Config = super::Config::from_managed_preview(context);
        let ObservedInvocation { result, cleanup }: ObservedInvocation = super::run_observing(
            config, slot, subscribe, submit,
        )
        .map_err(|error: ObserverStartError| SourceKePreviewFailure {
            failure: match error {
                ObserverStartError::Preparation(failure) => failure,
                ObserverStartError::Registration => RunFailed::PowerRegistration,
            },
            cleanup: PowerObserverCleanup::default(),
        })?;
        SourceKePreviewReport::from_run(receive_probe(result), cleanup)
    }
}

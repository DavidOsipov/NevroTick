// SPDX-License-Identifier: GPL-2.0-only
//! Source-owned laboratory KE results after actual worker retirement.

use std::error::Error;
use std::fmt;
use std::net::SocketAddr;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[cfg(windows)]
use crate::{
    ProtectedNativeCaptureRefusal, ProtectedNativeCaptureSpan, ProtectedNativeClockObservation,
};
use ntp_wire::{NtpTimestamp, RawHeader};

use super::{
    Failed, Failure, InvocationCompletion, ObservedInvocation, ObserverStartError,
    PowerObserverCleanup, RunFailed, StopHandle,
    association::ledger::{AppliedProtectedControl, Receipt},
    datagram::{ManagedProtectedHistory, ReceivedInvocation},
    failure_labels, handoff, io_label, job, receive_probe,
    response::{ControlProtection, ProtectedControlKind},
};
pub use super::{power_target::ManagedPowerObservation, profile::CertificateTimeProvenance};
use crate::{
    ExperimentalMeasurement, ExperimentalProfile, ExperimentalReferenceNs, MeasurementRefusal,
    RelativeMeasurementRefusal, RunClockContext, SoftwareClock, SourceConfigurationSnapshot,
    SourcePolicyError, relative::reference_from_instants,
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
                Some(Ok(InvocationCompletion::ManagedSuccessor(terminal))) => {
                    terminal.result.as_ref().err()
                }
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
            observed.map_err(SourceProtectedProbeFailure::from_observer_start)?;
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

/// Exercise the same private receipt projection on actual joined component history.
#[cfg(test)]
pub(super) fn protected_receipt(
    result: &super::datagram::ReceivedCompletion,
) -> Option<ManagedProtectedReceipt> {
    result
        .protected()?
        .managed_history()
        .map(ManagedProtectedReceipt::from_history)
}

impl fmt::Debug for SourceProtectedProbeReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SourceProtectedProbeReport { joined: true, transmitted: true, .. }")
    }
}

/// Explicit finite operation plan for the source-owned protected laboratory invocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ManagedProtectedPlan {
    /// One fresh verified KE, one initial protected request and at most one
    /// successor earned by the first committed ordinary authenticated reply.
    OneAuthenticatedSuccessor,
}

/// A historical ordinary authenticated reply committed under the original source.
///
/// The receipt records the original packet operation, authenticated peer claims
/// and process-local monotonic observations. It supplies no sample eligibility,
/// measurement or current authority.
#[must_use = "retain the original joined protected receipt"]
pub struct ManagedProtectedReceipt {
    /// Immutable configuration checked by the actual send and final commit.
    configuration: SourceConfigurationSnapshot,
    /// The original historical exchange and its bound endpoints.
    history: Arc<ManagedProtectedHistory>,
    /// Actual complete local request length.
    packet_bytes: usize,
    /// Checked population before the operation's first checkout.
    initial_cookie_population: (usize, usize),
    /// Authenticated peer leap indicator accepted by ordinary response admission.
    leap_indicator: u8,
    /// Original authenticated stratum octet.
    peer_stratum: u8,
    /// Original authenticated signed precision exponent.
    peer_precision_exponent: i8,
    /// Original authenticated root-delay bits.
    peer_root_delay_bits: u32,
    /// Original authenticated root-dispersion bits.
    peer_root_dispersion_bits: u32,
    /// Original server receive and transmit wire values, in that order.
    peer_timestamps: [NtpTimestamp; 2],
    /// Original guarded operation commit time.
    committed_at: Instant,
    /// Trailing observation of the original receive-return bracket.
    received_at: Instant,
    /// Original pre-send, send-return and selected receive-return brackets.
    application_clock_captures: [SoftwareClock; 3],
    /// Original pre-acquisition chart shared by this invocation's receipts.
    run_context: Arc<RunClockContext>,
    /// Original native disposition for each corresponding software observation.
    #[cfg(windows)]
    application_native_clock_captures: [Option<ProtectedNativeClockObservation>; 3],
    /// Inclusive elapsed bounds from the original pre-send and receive-return reads.
    application_wall_elapsed_bounds: Option<[Duration; 2]>,
}

impl ManagedProtectedReceipt {
    /// Copy only the bounded public facts from genuine joined history.
    fn from_history(managed: &Arc<ManagedProtectedHistory>) -> Self {
        let history: &super::datagram::ProtectedHistory = managed.history();
        let receipt: Receipt = history.receipt();
        let header: RawHeader = receipt.fields().header();
        let summary: &super::Summary = managed.summary();
        Self {
            configuration: managed.configuration().clone(),
            history: Arc::clone(managed),
            packet_bytes: summary.packet_bytes,
            initial_cookie_population: (summary.cookies, summary.cookie_bytes),
            leap_indicator: header.leap_indicator(),
            peer_stratum: header.stratum(),
            peer_precision_exponent: header.precision(),
            peer_root_delay_bits: header.root_delay_bits(),
            peer_root_dispersion_bits: header.root_dispersion_bits(),
            peer_timestamps: [header.receive_timestamp(), header.transmit_timestamp()],
            committed_at: history.committed_at(),
            received_at: history.received_at(),
            application_clock_captures: history.captures(),
            run_context: Arc::clone(history.run_context()),
            #[cfg(windows)]
            application_native_clock_captures: history.native_captures(),
            application_wall_elapsed_bounds: history.application_wall_elapsed_bounds(),
        }
    }

    /// Calculate conditional numerics solely from this original committed receipt.
    /// No clock reading, current authority, retry or packet effect follows.
    /// # Errors
    /// Preserve the shared measurement owner's checked refusal, including missing declarations.
    fn conditional_measurement(
        &self,
        profile: Option<ExperimentalProfile>,
    ) -> Result<ExperimentalMeasurement, MeasurementRefusal> {
        let history: &super::datagram::ProtectedHistory = self.history.history();
        let [send_before, send_return, receive_return]: [SoftwareClock; 3] = history.captures();
        let receipt: Receipt = history.receipt();
        crate::measurement::measure_for_source(
            profile,
            Some(self.history.configuration()),
            send_before,
            send_return,
            receive_return,
            receipt.accepted_at(),
            receipt.fields().header(),
        )
    }

    /// Borrow the original source configuration used by send and commit.
    #[must_use]
    pub const fn configuration(&self) -> &SourceConfigurationSnapshot {
        &self.configuration
    }

    /// Return the original local endpoint for pairing with an independent packet observation.
    #[must_use]
    pub fn local_endpoint(&self) -> SocketAddr {
        self.history.history().endpoints().0
    }

    /// Return the peer endpoint checked for this original authenticated reply.
    #[must_use]
    pub fn peer_endpoint(&self) -> SocketAddr {
        self.history.history().endpoints().1
    }

    /// Compare the allocation created for the original committed exchange.
    ///
    /// The same endpoint pair may occur in separate exchanges. This comparison
    /// identifies historical receipt custody and supplies no admission authority.
    #[must_use]
    pub fn same_exchange(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.history, &other.history)
    }

    /// Check whether both receipts came from the same published KE association.
    ///
    /// Distinct exchanges can use one association. Equality records historical
    /// provenance only and grants no current trust or admission authority.
    #[must_use]
    pub fn same_ke_association(&self, other: &Self) -> bool {
        self.history
            .history()
            .same_ke_association(other.history.history())
    }

    /// Return the verifier and time-input classification of the original KE.
    /// This historical receipt supplies no independent date or current trust.
    #[must_use]
    pub fn certificate_time_provenance(&self) -> CertificateTimeProvenance {
        self.history.history().certificate_time_provenance()
    }

    /// Return the complete locally submitted protected request length in bytes.
    #[must_use]
    pub const fn packet_bytes(&self) -> usize {
        self.packet_bytes
    }

    /// Count and total bytes of the pool before this operation checked out its cookie.
    #[must_use]
    pub const fn initial_cookie_population(&self) -> (usize, usize) {
        self.initial_cookie_population
    }

    /// Return the original authenticated peer claim (0, 1 or 2).
    #[must_use]
    pub const fn leap_indicator(&self) -> u8 {
        self.leap_indicator
    }

    /// Return the original authenticated peer stratum octet.
    #[must_use]
    pub const fn peer_stratum(&self) -> u8 {
        self.peer_stratum
    }

    /// Return the original authenticated precision claim as a signed base-two exponent.
    #[must_use]
    pub const fn peer_precision_exponent(&self) -> i8 {
        self.peer_precision_exponent
    }

    /// Return the original root-delay bits without numeric interpretation.
    #[must_use]
    pub const fn peer_root_delay_bits(&self) -> u32 {
        self.peer_root_delay_bits
    }

    /// Return the original root-dispersion bits without admitting a quality bound.
    #[must_use]
    pub const fn peer_root_dispersion_bits(&self) -> u32 {
        self.peer_root_dispersion_bits
    }

    /// Return the original authenticated server receive and transmit values (T2 and T3).
    ///
    /// The wire values retain all bits, including zero. This historical pair
    /// supplies no era interpretation, clock mapping, freshness or sample eligibility.
    #[must_use]
    pub const fn peer_timestamps(&self) -> [NtpTimestamp; 2] {
        self.peer_timestamps
    }

    /// Return the monotonic observation used by the guarded commit.
    #[must_use]
    pub const fn committed_at(&self) -> Instant {
        self.committed_at
    }

    /// Return the original trailing monotonic receive-return observation.
    #[must_use]
    pub const fn received_at(&self) -> Instant {
        self.received_at
    }

    /// Return the pre-send, same-send return and selected receive-return readings,
    /// in that order. Each value holds the original local wall reading and its
    /// two process-local monotonic bracket endpoints.
    ///
    /// These are historical application observations. The wall values supply no
    /// UTC accuracy, trusted date, packet placement or clock mapping.
    #[must_use]
    pub const fn application_clock_captures(&self) -> [SoftwareClock; 3] {
        self.application_clock_captures
    }

    /// Derive the exact nominal reference in this receipt's original invocation chart.
    ///
    /// Use the midpoint of the leading pre-send and receive-return observations,
    /// preserving a half nanosecond. The receipt retains the original age and
    /// chart. This calculation does not observe a clock or establish capture
    /// accuracy, mapping validity, current authority or sample eligibility.
    ///
    /// # Errors
    /// Preserve the shared midpoint owner's order, origin, range and checked-output refusal.
    pub fn nominal_reference(&self) -> Result<ExperimentalReferenceNs, RelativeMeasurementRefusal> {
        let [send, _, read]: [SoftwareClock; 3] = self.application_clock_captures;
        reference_from_instants(self.run_context.origin, send.monotonic(), read.monotonic())
    }

    /// Compare the original chart allocation of two historical receipts.
    ///
    /// Equal numeric origins alone do not identify the same invocation. Shared
    /// chart identity permits comparing nominal references within that chart
    /// and conveys no live association, clock mapping or admission authority.
    #[must_use]
    pub fn same_invocation(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.run_context, &other.run_context)
    }

    /// Independently check preservation of the actual history's chart allocation.
    #[cfg(all(test, any(windows, target_os = "linux")))]
    pub(super) fn retains_run_context(&self, original: &Arc<RunClockContext>) -> bool {
        Arc::ptr_eq(&self.run_context, original)
    }

    /// Return the original native outcomes in the same order as the software brackets.
    ///
    /// Absence identifies an alternate backend that made no native read at
    /// this point. None of the outcomes confers measurement or current authority.
    #[cfg(windows)]
    #[must_use]
    pub const fn application_native_clock_captures(
        &self,
    ) -> [Option<ProtectedNativeClockObservation>; 3] {
        self.application_native_clock_captures
    }

    /// Admit the original application brackets for a QPC span comparison.
    ///
    /// # Errors
    /// Refuse missing or failed native observations, a frequency change or
    /// unordered brackets. The authenticated receipt and its original age
    /// remain available on refusal.
    #[cfg(windows)]
    pub fn native_capture_span(
        &self,
    ) -> Result<ProtectedNativeCaptureSpan, ProtectedNativeCaptureRefusal> {
        ProtectedNativeCaptureSpan::from_observations(self.application_native_clock_captures)
    }

    /// Return the inclusive minimum and maximum monotonic intervals between
    /// this exchange's original pre-send and receive-return wall reads.
    /// The minimum pairs the leading receive reading with the trailing pre-send
    /// reading. The maximum pairs the trailing receive and leading pre-send readings.
    ///
    /// Unavailable bounds leave the historical receipt intact. These intervals
    /// include application placement and scheduling delay and supply no UTC mapping.
    #[must_use]
    pub const fn application_wall_elapsed_bounds(&self) -> Option<[Duration; 2]> {
        self.application_wall_elapsed_bounds
    }
}

impl fmt::Debug for ManagedProtectedReceipt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ManagedProtectedReceipt")
            .field("packet_bytes", &self.packet_bytes)
            .field("leap_indicator", &self.leap_indicator)
            .finish_non_exhaustive()
    }
}

/// A historical restriction already applied to the original managed source and peer.
///
/// This carries a restriction decision and how its packet was admitted.
#[must_use = "retain the original applied protected restriction"]
pub struct ManagedProtectedRestriction {
    /// Detached policy effect created only by the original checked owners.
    applied: AppliedProtectedControl,
}

impl ManagedProtectedRestriction {
    /// Return the closed wire code of the applied restriction.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self.applied.kind() {
            ProtectedControlKind::Rate => "RATE",
            ProtectedControlKind::Deny => "DENY",
            ProtectedControlKind::Restrict => "RSTR",
        }
    }

    /// Whether the control packet itself passed S2C authentication.
    #[must_use]
    pub const fn authenticated(&self) -> bool {
        matches!(self.applied.protection(), ControlProtection::Authenticated)
    }

    /// Borrow the exact checked source configuration used by the policy commit.
    #[must_use]
    pub fn configuration(&self) -> &SourceConfigurationSnapshot {
        self.applied.configuration()
    }

    /// Return the exact negotiated endpoint of the consumed protected request.
    #[must_use]
    pub const fn remote(&self) -> SocketAddr {
        self.applied.remote()
    }

    /// Return the original ordinal of the consumed protected request.
    #[must_use]
    pub const fn serial(&self) -> usize {
        self.applied.serial()
    }

    /// Return the monotonic observation used for the original policy decision.
    #[must_use]
    pub const fn decided_at(&self) -> Instant {
        self.applied.decided_at()
    }
}

impl fmt::Debug for ManagedProtectedRestriction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ManagedProtectedRestriction")
            .field("code", &self.code())
            .field("authenticated", &self.authenticated())
            .finish_non_exhaustive()
    }
}

/// Original terminal outcome after normal retirement and physical join.
#[derive(Debug)]
#[expect(
    clippy::large_enum_variant,
    reason = "retain bounded historical receipt facts inline without adding an allocation when projecting completed work"
)]
#[must_use = "inspect the original terminal protected operation outcome"]
pub enum ManagedProtectedTerminal {
    /// The successor produced a committed ordinary authenticated receipt.
    Ordinary(ManagedProtectedReceipt),
    /// Either protected request produced an applied restriction.
    Restriction(ManagedProtectedRestriction),
    /// The selected packet operation refused and its original cause was retained.
    Refused(SourceProtectedProbeFailure),
}

/// Fixed inline storage ceiling for one historical terminal observation.
const MAX_INLINE_MANAGED_TERMINAL_BYTES: usize = 512;

/// Require renewed layout review before a terminal result exceeds its fixed budget.
const _: () = assert!(size_of::<ManagedProtectedTerminal>() <= MAX_INLINE_MANAGED_TERMINAL_BYTES);

/// A joined Windows report lacks the observations required by the timing export.
/// The original report retains both the transport facts and raw observations.
#[cfg(windows)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use = "handle the historical timing observation refusal"]
pub enum ProtectedTimingObservationRefusal {
    /// The original power binding did not supply an observable result.
    PowerUnavailable,
    /// The original binding received a power event before retirement.
    PowerInvalidated,
    /// The job supplied no pair of process presentation queries.
    PresentationUnavailable,
    /// At least one presentation query failed and its raw result is retained.
    PresentationReadFailed,
    /// At least one successful query reported a mode other than Compatibility.
    UnsupportedPresentation,
}

/// Joined historical results for the explicit managed protected invocation.
///
/// A refused terminal operation can accompany an earlier committed receipt.
/// The counts describe the original invocation's finite aggregate work.
#[must_use = "inspect the original protected receipts and terminal outcome"]
pub struct ManagedProtectedReport {
    /// Only a genuine first commit recorded before successor admission can supply this.
    first: Option<ManagedProtectedReceipt>,
    /// The actual normally joined outcome, independent of earlier receipt custody.
    terminal: ManagedProtectedTerminal,
    /// Original checked experimental declaration retained by the joined receiver.
    profile: Option<ExperimentalProfile>,
    /// First admitted correlated NTS NAK from the pending successor request.
    first_nak_at: Option<Instant>,
    /// A failed observer close is retained independently of normal retirement.
    power_unregistration_error: Option<NonZeroU32>,
    /// Delivery observed through retirement of this invocation's own power binding.
    power_observation: Option<ManagedPowerObservation>,
    /// Actual calls to the local send operation.
    attempted_sends: usize,
    /// Actual complete local datagram submissions.
    fully_submitted_sends: usize,
    /// Unspent work retained from the original driver allowance.
    remaining_driver_work: usize,
    /// Remaining checkout, frame and byte allowance of the admitted association.
    remaining_association_work: Option<(usize, usize, usize)>,
    /// Raw current-process observations enclosing the original job, if read.
    #[cfg(windows)]
    process_leap_presentation: Option<[super::ProcessLeapPresentationRead; 2]>,
}

impl ManagedProtectedReport {
    /// Consume only joined custody for the explicitly selected successor invocation.
    ///
    /// # Errors
    /// Preserve pre-worker, worker and abnormal-join refusals and observer cleanup.
    /// Reject a joined result produced by a different invocation mode.
    pub(super) fn from_observed(
        observed: Result<ObservedInvocation, ObserverStartError>,
    ) -> Result<Self, SourceProtectedProbeFailure> {
        let ObservedInvocation { result, cleanup }: ObservedInvocation =
            observed.map_err(SourceProtectedProbeFailure::from_observer_start)?;
        let received: ReceivedInvocation = result
            .map_err(|failure| SourceProtectedProbeFailure::with_cleanup(failure, cleanup))?;
        let ReceivedInvocation::ManagedSuccessor(received) = received else {
            return Err(SourceProtectedProbeFailure::with_cleanup(
                RunFailed::Operation(Failure::Control.into()),
                cleanup,
            ));
        };
        #[cfg(windows)]
        let process_leap_presentation: Option<[super::ProcessLeapPresentationRead; 2]> =
            received.process_leap_presentation();
        let profile: Option<ExperimentalProfile> = received.profile();
        let (first, terminal, (attempted_sends, fully_submitted_sends, remaining_driver_work)): (
            Option<Arc<ManagedProtectedHistory>>,
            super::ManagedSuccessorTerminal,
            (usize, usize, usize),
        ) = received.into_parts();
        let super::ManagedSuccessorTerminal {
            result,
            first_nak_at,
            remaining_association_work,
        }: super::ManagedSuccessorTerminal = terminal;
        let terminal: ManagedProtectedTerminal = match result {
            Ok(super::ManagedSuccessorEffect::Ordinary(history)) => {
                ManagedProtectedTerminal::Ordinary(ManagedProtectedReceipt::from_history(&history))
            }
            Ok(super::ManagedSuccessorEffect::Restriction(applied)) => {
                ManagedProtectedTerminal::Restriction(ManagedProtectedRestriction { applied })
            }
            Err(failure) => ManagedProtectedTerminal::Refused(
                SourceProtectedProbeFailure::with_cleanup(RunFailed::Operation(failure), cleanup),
            ),
        };
        Ok(Self {
            first: first.as_ref().map(ManagedProtectedReceipt::from_history),
            terminal,
            profile,
            first_nak_at,
            power_unregistration_error: cleanup.unregistration_error,
            power_observation: cleanup.managed_observation,
            attempted_sends,
            fully_submitted_sends,
            remaining_driver_work,
            remaining_association_work,
            #[cfg(windows)]
            process_leap_presentation,
        })
    }

    /// Return the original Windows process presentation before submission and after joined take.
    ///
    /// Each outcome retains the native query refusal or observed mode. Absence means
    /// the backend did not take these observations. Equal modes describe only the two
    /// reads and supply no continuity, clock mapping or sample eligibility.
    #[cfg(windows)]
    #[must_use]
    pub const fn process_leap_presentation(
        &self,
    ) -> Option<
        [Result<
            windows_platform_native::LeapPresentation,
            windows_platform_native::LeapPresentationError,
        >; 2],
    > {
        self.process_leap_presentation
    }

    /// Borrow the genuine first committed receipt, when the worker recorded it.
    #[must_use]
    pub const fn first(&self) -> Option<&ManagedProtectedReceipt> {
        self.first.as_ref()
    }

    /// Borrow the terminal outcome after normal retirement and actual join.
    #[must_use]
    pub const fn terminal(&self) -> &ManagedProtectedTerminal {
        &self.terminal
    }

    /// Calculate nominal arithmetic for the original first receipt, if present.
    /// Uses the retained declaration and preserves that exchange's original age.
    /// A present receipt with no declaration returns `Some(Err(Unavailable))`.
    /// The calculation supplies no calibrated mapping, freshness or eligibility.
    #[must_use]
    pub fn first_measurement(&self) -> Option<Result<ExperimentalMeasurement, MeasurementRefusal>> {
        self.first
            .as_ref()
            .map(|receipt: &ManagedProtectedReceipt| receipt.conditional_measurement(self.profile))
    }

    /// Calculate nominal arithmetic for an ordinary terminal receipt.
    /// Uses the retained declaration and preserves that exchange's original age.
    /// A restriction or operation refusal returns no terminal measurement.
    /// The calculation supplies no calibrated mapping, freshness or eligibility.
    #[must_use]
    pub fn terminal_measurement(
        &self,
    ) -> Option<Result<ExperimentalMeasurement, MeasurementRefusal>> {
        match &self.terminal {
            ManagedProtectedTerminal::Ordinary(receipt) => {
                Some(receipt.conditional_measurement(self.profile))
            }
            ManagedProtectedTerminal::Restriction(_) | ManagedProtectedTerminal::Refused(_) => None,
        }
    }

    /// Return the first admitted NTS NAK's original monotonic arrival.
    ///
    /// If the terminal operation refused, recovery following this observation is
    /// unavailable within the invocation's single-KE allowance.
    #[must_use]
    pub const fn first_nak_at(&self) -> Option<Instant> {
        self.first_nak_at
    }

    /// The original NAK remained unresolved when the bounded operation refused.
    /// The single-KE plan cannot perform the required recovery or start a retry.
    #[must_use]
    pub const fn recovery_unavailable(&self) -> bool {
        self.first_nak_at.is_some() && matches!(self.terminal, ManagedProtectedTerminal::Refused(_))
    }

    /// Return the exact nonzero power observer unregistration status on failure.
    #[must_use]
    pub const fn power_unregistration_error(&self) -> Option<NonZeroU32> {
        self.power_unregistration_error
    }

    /// Return historical delivery through retirement of this invocation's binding.
    ///
    /// Absence means this acquisition path supplied no binding observation.
    /// Even an event-free observation supplies no physical continuity or mapping.
    #[must_use]
    pub const fn power_observation(&self) -> Option<ManagedPowerObservation> {
        self.power_observation
    }

    /// Check the original power and presentation observations for relative timing export.
    ///
    /// Success confirms no delivery to the available original binding before
    /// retirement and two successful Compatibility reads.
    /// It grants no clock continuity, calibrated mapping or sample authority.
    /// This check reads only this report and preserves all historical outcomes.
    ///
    /// # Errors
    /// Refuse a missing or invalidated power observation before checking the original
    /// presentation reads. Absent, failed and unsupported presentations stay distinct.
    #[cfg(windows)]
    pub const fn check_timing_observations(&self) -> Result<(), ProtectedTimingObservationRefusal> {
        use windows_platform_native::LeapPresentation::Compatibility;

        match self.power_observation() {
            Some(ManagedPowerObservation::NoEventObserved) => (),
            Some(ManagedPowerObservation::Invalidated) => {
                return Err(ProtectedTimingObservationRefusal::PowerInvalidated);
            }
            Some(ManagedPowerObservation::Unavailable) | None => {
                return Err(ProtectedTimingObservationRefusal::PowerUnavailable);
            }
        }

        match self.process_leap_presentation() {
            Some([Ok(Compatibility), Ok(Compatibility)]) => Ok(()),
            Some([Ok(_), Ok(_)]) => Err(ProtectedTimingObservationRefusal::UnsupportedPresentation),
            Some([Err(_), _] | [_, Err(_)]) => {
                Err(ProtectedTimingObservationRefusal::PresentationReadFailed)
            }
            None => Err(ProtectedTimingObservationRefusal::PresentationUnavailable),
        }
    }

    /// Return the total number of actual local send calls across both operations.
    #[must_use]
    pub const fn attempted_sends(&self) -> usize {
        self.attempted_sends
    }

    /// Return the number of complete local submissions across both operations.
    #[must_use]
    pub const fn fully_submitted_sends(&self) -> usize {
        self.fully_submitted_sends
    }

    /// Return the unspent part of the original invocation's driver allowance.
    #[must_use]
    pub const fn remaining_driver_work(&self) -> usize {
        self.remaining_driver_work
    }

    /// Unspent checkout, datagram and byte allowances, in that order.
    /// An invocation that established no association supplies no observation.
    #[must_use]
    pub const fn remaining_association_work(&self) -> Option<(usize, usize, usize)> {
        self.remaining_association_work
    }
}

impl fmt::Debug for ManagedProtectedReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ManagedProtectedReport")
            .field("first", &self.first)
            .field("terminal", &self.terminal)
            .field("attempted_sends", &self.attempted_sends)
            .field("fully_submitted_sends", &self.fully_submitted_sends)
            .field("remaining_driver_work", &self.remaining_driver_work)
            .finish_non_exhaustive()
    }
}

/// Joined history for an explicit two-job protected acquisition.
#[derive(Debug)]
#[must_use = "inspect both original jobs and the owning observer close"]
pub struct ManagedProtectedPairReport {
    /// First actual normally joined invocation.
    first: ManagedProtectedReport,
    /// Independent second outcome if the first job earned the second attempt.
    second: Option<Result<ManagedProtectedReport, SourceProtectedProbeFailure>>,
    /// Status of the owning observer's single close attempt.
    power_unregistration_error: Option<NonZeroU32>,
}

impl ManagedProtectedPairReport {
    /// Borrow the first normally joined result, including any terminal refusal.
    #[must_use]
    pub const fn first(&self) -> &ManagedProtectedReport {
        &self.first
    }

    /// Borrow the independent outcome of the second attempt, when earned.
    /// A restriction or terminal refusal in the first job leaves this absent.
    #[must_use]
    pub fn second(&self) -> Option<Result<&ManagedProtectedReport, &SourceProtectedProbeFailure>> {
        self.second.as_ref().map(Result::as_ref)
    }

    /// Return the exact nonzero status of the pair's single observer close on failure.
    #[must_use]
    pub const fn power_unregistration_error(&self) -> Option<NonZeroU32> {
        self.power_unregistration_error
    }
}

/// Receive the original submitted job and detach its own power lease.
/// # Errors
/// Preserve the actual submission, collection or receiving refusal and Stop status.
fn run_pair_member(
    job: super::ManagedObservedJob<'_>,
    submit: &mut impl FnMut(job::SubmissionReservation) -> Result<job::SubmittedJob, job::SubmitError>,
    #[cfg(windows)] read_presentation: &mut Option<
        &mut dyn FnMut() -> super::ProcessLeapPresentationRead,
    >,
) -> Result<ManagedProtectedReport, SourceProtectedProbeFailure> {
    let super::ManagedObservedJob {
        reservation,
        active,
        stop,
    } = job;
    let result: Result<ReceivedInvocation, RunFailed> = super::receive_managed_submission(
        || submit(reservation),
        #[cfg(windows)]
        read_presentation,
    );
    ManagedProtectedReport::from_observed(Ok(super::finish_observing(
        result,
        &stop,
        Some(active),
        || Ok(()),
    )))
}

/// Run two explicit checked jobs within the original cutoff and one owning scope.
///
/// Each job uses its own original reservation and Stop binding. Only two
/// genuine ordinary receipts from the first job admit the second authorization.
/// Any unconfirmed collection retains actual job and provider custody in the slot.
///
/// # Errors
/// Preserve first preparation, registration, submission and collection refusals.
/// Retain terminal operation history and independent observer close status.
pub(super) fn run_managed_pair_with<Close: FnOnce() -> Result<(), NonZeroU32>>(
    first: super::Config,
    second: impl FnOnce() -> Result<super::Config, SourceProtectedProbeFailure>,
    cutoff: Instant,
    slot: &'static job::WorkerSlot,
    subscribe: impl FnOnce(Arc<super::power_target::PowerTarget>) -> Result<Close, ()>,
    mut submit: impl FnMut(job::SubmissionReservation) -> Result<job::SubmittedJob, job::SubmitError>,
    #[cfg(windows)] mut read_presentation: Option<
        &mut dyn FnMut() -> super::ProcessLeapPresentationRead,
    >,
) -> Result<ManagedProtectedPairReport, SourceProtectedProbeFailure> {
    let worker: job::ManagedWorker = job::ManagedWorker::new(slot);
    let target: Arc<super::power_target::PowerTarget> =
        Arc::new(super::power_target::PowerTarget::new());
    let first_job: super::ManagedObservedJob<'_> =
        super::prepare_managed_observed(first, Some(cutoff), &worker, &target)
            .map_err(SourceProtectedProbeFailure::from_observer_start)?;
    let close: Close = subscribe(Arc::clone(&target)).map_err(|()| {
        SourceProtectedProbeFailure::from_observer_start(ObserverStartError::Registration)
    })?;
    let mut result: Result<ManagedProtectedPairReport, SourceProtectedProbeFailure> =
        run_pair_member(
            first_job,
            &mut submit,
            #[cfg(windows)]
            &mut read_presentation,
        )
        .map(|first: ManagedProtectedReport| {
            let second: Option<Result<ManagedProtectedReport, SourceProtectedProbeFailure>> =
                (first.first().is_some()
                    && matches!(first.terminal(), ManagedProtectedTerminal::Ordinary(_)))
                .then(|| {
                    let config: super::Config = second()?;
                    let job: super::ManagedObservedJob<'_> =
                        super::prepare_managed_observed(config, Some(cutoff), &worker, &target)
                            .map_err(SourceProtectedProbeFailure::from_observer_start)?;
                    run_pair_member(
                        job,
                        &mut submit,
                        #[cfg(windows)]
                        &mut read_presentation,
                    )
                });
            ManagedProtectedPairReport {
                first,
                second,
                power_unregistration_error: None,
            }
        });
    drop(worker);
    let status: Option<NonZeroU32> = close().err();
    match &mut result {
        Ok(report) => report.power_unregistration_error = status,
        Err(failure) => failure.worker.cleanup.unregistration_error = status,
    }
    result
}

/// Bounded source, worker or operation refusal from the explicit protected invocation.
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

    /// Project failure before the observer could submit the worker.
    fn from_observer_start(error: ObserverStartError) -> Self {
        Self::from_run_failure(match error {
            ObserverStartError::Preparation(failure) => failure,
            ObserverStartError::Registration => RunFailed::PowerRegistration,
        })
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

    /// Read the genuine first receipt retained after abnormal join.
    ///
    /// This is historical data. The failed worker has no normal retirement report.
    #[must_use]
    pub fn retained_first(&self) -> Option<ManagedProtectedReceipt> {
        let RunFailed::Abnormal(joined) = &self.worker.failure else {
            return None;
        };
        joined
            .first_receipt()
            .map(ManagedProtectedReceipt::from_history)
    }

    /// Read the genuine first NTS NAK mark retained after abnormal join.
    #[must_use]
    pub fn retained_first_nak_at(&self) -> Option<Instant> {
        match &self.worker.failure {
            RunFailed::Abnormal(joined) => joined.first_nak_at(),
            _ => None,
        }
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
        let config: super::Config = self.protected_config(expected)?;
        SourceProtectedProbeReport::from_observed(super::run_with_power_observer(config))
    }

    /// Run the explicitly selected protected loopback plan at the original revision.
    ///
    /// The plan uses one verified KE and the original finite worker allowance.
    /// The first committed ordinary reply can earn one successor. The joined
    /// report retains original packet outcomes and independent observer cleanup.
    ///
    /// # Errors
    /// Preserve source, caller, submission, observer and abnormal worker refusals.
    /// A normally joined packet refusal is retained in the report's terminal outcome.
    pub fn invoke_nts_protected(
        &self,
        expected: &SourceConfigurationSnapshot,
        plan: ManagedProtectedPlan,
    ) -> Result<ManagedProtectedReport, SourceProtectedProbeFailure> {
        self.invoke_nts_protected_with_cutoff(expected, plan, None)
    }

    /// Run the selected protected loopback plan before an owner's absolute cutoff.
    ///
    /// The original cutoff can only shorten the handshake and exchange windows.
    /// It also bounds the successor and the final receive and commit guards.
    /// Actual worker retirement and observer closure retain their original
    /// custody. An elapsed cutoff does not imply that synchronous collection
    /// has finished.
    ///
    /// # Errors
    /// Preserve the original failure stages of [`Self::invoke_nts_protected`].
    /// An already elapsed cutoff refuses preparation before observer registration.
    /// A later deadline refusal preserves the actual joined operation outcome.
    pub fn invoke_nts_protected_before(
        &self,
        expected: &SourceConfigurationSnapshot,
        plan: ManagedProtectedPlan,
        cutoff: Instant,
    ) -> Result<ManagedProtectedReport, SourceProtectedProbeFailure> {
        self.invoke_nts_protected_with_cutoff(expected, plan, Some(cutoff))
    }

    /// Run up to two explicitly selected protected jobs under one native observer.
    ///
    /// A first job with both ordinary authenticated receipts earns the second job.
    /// Recheck the second source revision at that boundary. Both jobs use the same
    /// cutoff for effects, one provider scope and distinct original Control owners.
    /// Synchronous collection can outlast the cutoff. A collection refusal requests
    /// Stop and retains unconfirmed worker custody. Joined receipts are historical
    /// and carry no clock or tracking authority.
    ///
    /// # Errors
    /// Preserve first source, preparation, registration or worker failure.
    /// The pair retains any second refusal and its independent observer close.
    pub fn invoke_nts_protected_pair_before(
        &self,
        first: &SourceConfigurationSnapshot,
        second: &SourceConfigurationSnapshot,
        cutoff: Instant,
    ) -> Result<ManagedProtectedPairReport, SourceProtectedProbeFailure> {
        run_managed_pair_with(
            self.protected_config(first)?,
            || self.protected_config(second),
            cutoff,
            &super::PROCESS_WORKER,
            super::subscribe_managed_power,
            job::SubmissionReservation::try_submit_successor,
            Some(&mut windows_platform_native::read_leap_presentation),
        )
    }

    /// Bind the exact installed definition before preparing either invocation window.
    /// # Errors
    /// Preserve source, preparation, observer and worker failures at their existing owners.
    fn invoke_nts_protected_with_cutoff(
        &self,
        expected: &SourceConfigurationSnapshot,
        plan: ManagedProtectedPlan,
        owner_cutoff: Option<Instant>,
    ) -> Result<ManagedProtectedReport, SourceProtectedProbeFailure> {
        let config: super::Config = self.protected_config(expected)?;
        match plan {
            ManagedProtectedPlan::OneAuthenticatedSuccessor => {
                ManagedProtectedReport::from_observed(
                    super::run_managed_successor_with_power_observer(config, owner_cutoff),
                )
            }
        }
    }

    /// Authorize the installed configuration before preparing either protected mode.
    ///
    /// # Errors
    /// Preserve the original typed refusal from the source policy owner.
    fn protected_config(
        &self,
        expected: &SourceConfigurationSnapshot,
    ) -> Result<super::Config, SourceProtectedProbeFailure> {
        self.authorize_protected_ke(expected)
            .map(super::Config::from_managed_protected)
            .map_err(|error: SourcePolicyError| {
                SourceProtectedProbeFailure::from_run_failure(RunFailed::Preparation(Failed::from(
                    Failure::SourcePolicy(error),
                )))
            })
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

#[cfg(test)]
mod nominal_reference_tests {
    //! Fixed endpoints challenge the actual receipt accessors without touching a clock owner.

    use std::time::UNIX_EPOCH;

    use super::{
        Arc, Duration, ExperimentalReferenceNs, Failure, Instant, ManagedProtectedReceipt,
        RelativeMeasurementRefusal, RunClockContext, SoftwareClock,
    };
    use crate::nts_ke_experiment::{
        job::WorkerSlot, managed_preview_tests::component_protected_receipt,
        process::ProcessAdmission,
    };

    /// Check a literal odd midpoint, equal-valued foreign chart, and reversed endpoints.
    /// # Errors
    /// Preserve source configuration failure or refuse an unrepresentable fixture instant.
    /// # Panics
    /// Fail if rounding, substituted captures or numeric equality override the receipt.
    #[test]
    fn protected_nominal_reference_rejects_reversal_and_distinguishes_chart_identity()
    -> Result<(), Failure> {
        static SELECTED_SLOT: WorkerSlot = WorkerSlot::new();
        static SELECTED_ADMISSION: ProcessAdmission = ProcessAdmission::new();
        static FOREIGN_SLOT: WorkerSlot = WorkerSlot::new();
        static FOREIGN_ADMISSION: ProcessAdmission = ProcessAdmission::new();
        let mut selected: ManagedProtectedReceipt =
            component_protected_receipt(&SELECTED_SLOT, &SELECTED_ADMISSION)?;
        let mut foreign: ManagedProtectedReceipt =
            component_protected_receipt(&FOREIGN_SLOT, &FOREIGN_ADMISSION)?;
        let origin: Instant = Instant::now();
        let send: Instant = origin
            .checked_add(Duration::from_nanos(10))
            .ok_or(Failure::Control)?;
        let read: Instant = origin
            .checked_add(Duration::from_nanos(11))
            .ok_or(Failure::Control)?;
        let trailing: Instant = origin
            .checked_add(Duration::from_nanos(12))
            .ok_or(Failure::Control)?;
        let captures: [SoftwareClock; 3] = [
            SoftwareClock {
                mono: send,
                utc: UNIX_EPOCH,
                mono_after: read,
            },
            SoftwareClock {
                mono: read,
                utc: UNIX_EPOCH,
                mono_after: read,
            },
            SoftwareClock {
                mono: read,
                utc: UNIX_EPOCH,
                mono_after: trailing,
            },
        ];
        // Change only the test projections' chart and software readings to
        // exercise the independent midpoint and historical identity boundaries.
        selected.application_clock_captures = captures;
        foreign.application_clock_captures = captures;
        selected.run_context = Arc::new(RunClockContext { origin });
        foreign.run_context = Arc::new(RunClockContext { origin });
        let parts = |value: ExperimentalReferenceNs| {
            (value.whole_nanoseconds(), value.has_half_nanosecond())
        };
        assert_eq!(selected.nominal_reference().map(parts), Ok((10, true)));
        assert_eq!(foreign.nominal_reference().map(parts), Ok((10, true)));
        assert!(!selected.same_invocation(&foreign));
        assert_eq!(selected.local_endpoint(), foreign.local_endpoint());
        assert_eq!(selected.peer_endpoint(), foreign.peer_endpoint());
        assert_eq!(
            selected.certificate_time_provenance(),
            foreign.certificate_time_provenance()
        );
        assert!(!selected.same_exchange(&foreign));

        foreign.run_context = Arc::clone(&selected.run_context);
        assert!(selected.same_invocation(&foreign));
        assert!(!selected.same_exchange(&foreign));
        assert!(!selected.same_ke_association(&foreign));

        selected.application_clock_captures.swap(0, 2);
        assert_eq!(
            selected.nominal_reference().map(parts),
            Err(RelativeMeasurementRefusal::ReferenceOrder)
        );
        Ok(())
    }
}

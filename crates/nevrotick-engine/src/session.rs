// SPDX-License-Identifier: GPL-2.0-only
//! Persistent chart provenance around explicitly requested controlled acquisitions.
//!
//! One owner retains an immutable, already checked configuration and creates its
//! chart before preparation. An exclusive prepared guard permits at most one
//! prepared or executing series. Reports retain immutable provenance only: they
//! supply no administrator approval, clock continuity, mapping, date/scale,
//! calibration, current precision, statistical admission or current authority.
//! Each execution still owns the original three-attempt driver and its cleanup.
#![forbid(unsafe_code)]

use std::{error::Error, fmt, sync::Arc, time::Instant};

use super::{
    ControlHandle, ControlSchedule, ExperimentalMeasurement, ExperimentalProfile,
    ExperimentalReferenceNs, MeasurementRefusal, PilotError, PreparedSeries,
    RelativeMeasurementRefusal, SeriesAttempt, SeriesConfig, SeriesEnd, SeriesReport, SlotReport,
    TransportFacts, execute_series, measurement::series_coherence_residuals, prepare_series,
    relative::reference_from_instants,
};

/// Exclusive issuer lifecycle; private transitions own capacity and retirement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SessionState {
    /// No prepared plan or active execution is owned.
    Idle,
    /// Exactly one guard owns the prepared plan, or that guard was forgotten.
    Prepared,
    /// The consuming guard entered the driver and cleanup has not returned.
    Executing,
    /// Terminal refusal of all future preparations by this owner.
    Retired,
}

/// Immutable chart created by the actual session constructor before preparation.
/// Its allocation identity binds provenance; the origin supplies no UTC mapping.
struct SessionChart {
    /// Actual session-construction monotonic reading, never refreshed by reports.
    origin: Instant,
}

/// Fixed session failures, retaining an original driver error as a typed source.
/// No variant carries a peer, chart, request, raw packet or formatted native text.
pub enum AcquisitionError {
    /// A prepared or executing acquisition already owns the session's capacity.
    Busy,
    /// This owner has been terminally retired.
    Retired,
    /// A private guard/state association contradicted its construction contract.
    Invariant,
    /// Preserve the original bounded preparation or execution failure.
    Driver(PilotError),
}

impl fmt::Debug for AcquisitionError {
    /// Redact the variant and its retained driver cause with fixed text.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("AcquisitionError { .. }")
    }
}

impl fmt::Display for AcquisitionError {
    /// Render a bounded operation description without repeating its source.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Busy => "acquisition session busy",
            Self::Retired => "acquisition session retired",
            Self::Invariant => "acquisition session invariant",
            Self::Driver(_) => "acquisition driver failed",
        })
    }
}

impl Error for AcquisitionError {
    /// Borrow only the original bounded driver cause; expose no other payload.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Driver(cause) => Some(cause),
            Self::Busy | Self::Retired | Self::Invariant => None,
        }
    }
}

/// One controlled source configuration and genuine pre-acquisition chart.
///
/// `SeriesConfig`'s mandatory Garde constructor owns all configuration limits.
/// This owner moves that checked value unchanged; it introduces no raw input or
/// second rule. Private construction, exclusive borrowing and terminal retirement
/// own the chart/state invariants. A new configuration or association requires a
/// new session; retained reports cannot reopen this owner or confer authority.
///
/// Preparation, unused-plan drop and idempotent retirement execute without
/// creating a socket or runtime:
/// ```
/// use std::{net::SocketAddr, time::Duration};
/// use nevrotick_engine::{AcquisitionError, AcquisitionSession, SeriesConfig};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let config = SeriesConfig::new(
///     SocketAddr::from(([127, 0, 0, 1], 41001)),
///     Duration::from_millis(100), Duration::from_secs(1),
/// )?;
/// let mut session = AcquisitionSession::new(config);
/// let (prepared, control) = session.prepare()?;
/// drop(prepared);
/// drop(control);
/// session.retire();
/// session.retire();
/// assert!(matches!(session.prepare(), Err(AcquisitionError::Retired)));
/// # Ok(()) }
/// ```
/// Existing chart/state cannot be paired with another checked configuration:
/// ```compile_fail,E0451
/// use nevrotick_engine::{AcquisitionSession, SeriesConfig};
/// fn forge(base: AcquisitionSession, config: SeriesConfig) -> AcquisitionSession {
///     AcquisitionSession { config, ..base }
/// }
/// ```
/// ```compile_fail,E0616
/// use nevrotick_engine::{AcquisitionSession, SeriesConfig};
/// fn mutate(mut owner: AcquisitionSession, config: SeriesConfig) {
///     owner.config = config;
/// }
/// ```
/// ```compile_fail,E0599
/// use nevrotick_engine::AcquisitionSession;
/// let _ = AcquisitionSession::default();
/// ```
pub struct AcquisitionSession {
    /// Immutable admitted configuration; only `SeriesConfig` owns raw validation.
    config: SeriesConfig,
    /// Actual shared chart, created before any plan can be prepared.
    chart: Arc<SessionChart>,
    /// Current exclusive capacity/retirement owner; never externally writable.
    state: SessionState,
}

impl AcquisitionSession {
    /// Move one already checked configuration and capture its chart exactly once.
    /// No entropy, socket, runtime, mapping evidence or authority is created here.
    #[must_use]
    pub fn new(config: SeriesConfig) -> Self {
        Self {
            config,
            chart: Arc::new(SessionChart {
                origin: Instant::now(),
            }),
            state: SessionState::Idle,
        }
    }

    /// Prepare one actual plan without creating a socket or runtime.
    /// The original driver owns its single entropy operation and token admission.
    /// # Errors
    /// Refuse Busy/Retired before entropy. An original preparation failure leaves
    /// Idle unchanged; it neither retries nor invents a prepared capability.
    pub fn prepare(
        &mut self,
    ) -> Result<(PreparedAcquisition<'_>, ControlHandle), AcquisitionError> {
        match self.state {
            SessionState::Idle => {}
            SessionState::Prepared | SessionState::Executing => {
                return Err(AcquisitionError::Busy);
            }
            SessionState::Retired => return Err(AcquisitionError::Retired),
        }
        let chart = Arc::clone(&self.chart);
        let (prepared, control) = prepare_series(&self.config).map_err(AcquisitionError::Driver)?;
        self.state = SessionState::Prepared;
        Ok((
            PreparedAcquisition {
                session: self,
                prepared: Some(prepared),
                chart,
            },
            control,
        ))
    }

    /// Idempotently and terminally retire this quiescent owner.
    ///
    /// A live guard prevents this mutable borrow. A forgotten unexecuted guard
    /// leaves Busy state but no socket/runtime; retirement never releases that
    /// forgotten capacity for reuse. Replacement requires a new session/chart.
    pub fn retire(&mut self) {
        self.state = SessionState::Retired;
    }
}

impl fmt::Debug for AcquisitionSession {
    /// Redact configuration, lifecycle and chart origin/identity with fixed text.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("AcquisitionSession { .. }")
    }
}

/// Exclusive, consume-once acquisition prepared by its genuine session owner.
///
/// An unused drop releases only the actual unexecuted preparation. Forgetting a
/// guard leaves the owner's Prepared state Busy; only terminal retirement or
/// dropping the whole owner remains. Execution retains this mutable borrow until
/// the original driver has returned through timer/runtime cleanup. An unexpected
/// unwind propagates while `Drop` retires the owner; it is not caught or retried.
///
/// A consumed plan cannot be executed twice:
/// ```compile_fail,E0382
/// use nevrotick_engine::{ControlSchedule, PreparedAcquisition};
/// fn twice(plan: PreparedAcquisition<'_>, schedule: &ControlSchedule) {
///     let _first = plan.execute(schedule);
///     let _second = plan.execute(schedule);
/// }
/// ```
/// One live plan prevents a second preparation or retirement:
/// ```compile_fail,E0499
/// use nevrotick_engine::{AcquisitionError, AcquisitionSession};
/// fn overlap(owner: &mut AcquisitionSession) -> Result<(), AcquisitionError> {
///     let (first, control) = owner.prepare()?;
///     let second = owner.prepare()?;
///     drop((first, control, second));
///     Ok(())
/// }
/// ```
/// ```compile_fail,E0499
/// use nevrotick_engine::{AcquisitionError, AcquisitionSession};
/// fn retire_live(owner: &mut AcquisitionSession) -> Result<(), AcquisitionError> {
///     let (plan, control) = owner.prepare()?;
///     owner.retire();
///     drop((plan, control));
///     Ok(())
/// }
pub struct PreparedAcquisition<'owner> {
    /// Exclusive actual issuer, retained through execution and real cleanup.
    session: &'owner mut AcquisitionSession,
    /// Actual original preparation; `take` permits exactly one driver invocation.
    prepared: Option<PreparedSeries>,
    /// Immutable chart captured before this plan's acquisition.
    chart: Arc<SessionChart>,
}

impl PreparedAcquisition<'_> {
    /// Validate the private owner association and enter Executing before extraction.
    /// # Errors
    /// Retire contradictory state/chart/missing plan before socket/runtime work.
    fn begin_execution(&mut self) -> Result<PreparedSeries, AcquisitionError> {
        if self.session.state != SessionState::Prepared
            || !Arc::ptr_eq(&self.chart, &self.session.chart)
        {
            self.session.state = SessionState::Retired;
            return Err(AcquisitionError::Invariant);
        }
        self.session.state = SessionState::Executing;
        let Some(prepared) = self.prepared.take() else {
            self.session.state = SessionState::Retired;
            return Err(AcquisitionError::Invariant);
        };
        Ok(prepared)
    }

    /// Consume the actual plan and wrap only its genuine returned report inline.
    ///
    /// Preserve all three attempts, original run/request identities, capture ages,
    /// deadlines and control/cleanup cutoffs. Clean Completed/Stopped/deadline
    /// outcomes permit another explicitly requested series unless genuine retained
    /// captures contradict that run's declared coherence or cannot be checked.
    /// No automatic retry or report queue is introduced. Other returned outcomes
    /// retain their original report and retire this owner. Assessment reads no
    /// clock and establishes no continuity or finite physical error, even on success.
    /// # Errors
    /// Retire on a structural contradiction or original outer driver error. An
    /// unwind propagates and leaves Executing for `Drop` to retire fail-closed.
    pub fn execute(
        mut self,
        schedule: &ControlSchedule,
    ) -> Result<AcquisitionReport, AcquisitionError> {
        let prepared = self.begin_execution()?;
        let series = execute_series(prepared, schedule).map_err(AcquisitionError::Driver)?;
        let coherence = series_coherence(self.session.config.experimental_profile(), &series);
        self.session.state = completion_state(
            series.end(),
            series.primary_failure().is_some(),
            series.stop_cleanup_failure().is_some(),
            series.invalidate_cleanup_failure().is_some(),
            coherence,
        );
        Ok(AcquisitionReport {
            series,
            chart: Arc::clone(&self.chart),
            coherence,
        })
    }
}

impl fmt::Debug for PreparedAcquisition<'_> {
    /// Redact issuer, control preparation and chart with fixed text.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("PreparedAcquisition { .. }")
    }
}

impl Drop for PreparedAcquisition<'_> {
    /// Release unused preparation before Idle; retire unfinished execution instead.
    /// Contradictory private representations cannot reopen this owner's capacity.
    fn drop(&mut self) {
        if !Arc::ptr_eq(&self.chart, &self.session.chart) {
            self.session.state = SessionState::Retired;
            return;
        }
        match self.session.state {
            SessionState::Prepared => {
                if let Some(prepared) = self.prepared.take() {
                    drop(prepared);
                    self.session.state = SessionState::Idle;
                } else {
                    self.session.state = SessionState::Retired;
                }
            }
            SessionState::Executing => self.session.state = SessionState::Retired,
            SessionState::Idle => {
                if self.prepared.is_some() {
                    self.session.state = SessionState::Retired;
                }
            }
            SessionState::Retired => {}
        }
    }
}

/// Select reuse only for a clean normal terminal outcome, without effects.
/// Failure flags represent the actual three independently retained driver causes;
/// every combination is admitted, and any cause retires regardless of the label.
/// A checked capture failure or observed inconsistency adds a veto; missing
/// profile/facts and compatible observations cannot override existing retirement.
fn completion_state(
    end: SeriesEnd,
    primary_failed: bool,
    stop_cleanup_failed: bool,
    invalidate_cleanup_failed: bool,
    coherence: SeriesCoherence,
) -> SessionState {
    match end {
        SeriesEnd::Completed | SeriesEnd::Stopped | SeriesEnd::SeriesDeadline => {
            if primary_failed
                || stop_cleanup_failed
                || invalidate_cleanup_failed
                || matches!(
                    coherence,
                    SeriesCoherence::ObservedInconsistent
                        | SeriesCoherence::Unavailable(CoherenceUnavailable::Capture(_))
                )
            {
                SessionState::Retired
            } else {
                SessionState::Idle
            }
        }
        SeriesEnd::Stale | SeriesEnd::Failure => SessionState::Retired,
    }
}

/// Fixed absence or checked refusal of a one-run observation assessment.
/// These typed reasons retain no captures, native text or validation report.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoherenceUnavailable {
    /// This execution had no explicit one-run experimental declaration.
    NoProfile,
    /// No original terminal facts retained any capture triplet.
    NoFacts,
    /// Original local captures failed checked chronology or representation.
    /// The actual assessor produces `CaptureOrder` or `Arithmetic` only.
    Capture(MeasurementRefusal),
}

/// Rejection-only comparison of one returned run's genuine retained captures.
///
/// Compatible means only that each retained capture's possible wall-progress
/// interval agrees with the fixed first retained capture within the original
/// declared allowance. It establishes neither all-pairs agreement, missing-capture
/// coverage, uninterrupted mapping, trusted UTC nor finite physical accuracy.
/// No status grants current statistical or clock authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SeriesCoherence {
    /// Every retained baseline comparison was within the declared inclusive limit.
    ObservedCompatible,
    /// A valid retained baseline comparison exceeded the declared limit.
    ObservedInconsistent,
    /// The declaration/facts were absent, or checked capture assessment refused.
    Unavailable(CoherenceUnavailable),
}

/// Assess only original captures from this genuine report under its one-run model.
///
/// Missing profile precedes facts. The fixed three-position shape prevents silent
/// omission if the plan size changes. At most nine captures are assessed without
/// allocation, clock reads or measurement/header/date revalidation. Complete raw
/// chronology and checked arithmetic precede threshold classification. Refusal
/// changes no original report, result, capture, age or session state.
fn series_coherence(
    profile: Option<ExperimentalProfile>,
    series: &SeriesReport,
) -> SeriesCoherence {
    let Some(profile) = profile else {
        return SeriesCoherence::Unavailable(CoherenceUnavailable::NoProfile);
    };
    let [first, second, third] = series.attempts();
    let captures = [first, second, third].map(|attempt| {
        attempt
            .report()
            .and_then(SlotReport::facts)
            .map(|facts| [facts.send_before(), facts.send_after(), facts.read_return()])
    });
    capture_coherence(profile, &captures)
}

/// Classify a complete checked capture assessment against its original declaration.
///
/// Accept every raw fixed triplet array under an already checked profile. The
/// measurement owner validates the entire chronology/conversion before this
/// owner compares its maximum gap with the inclusive declared limit. Date/header
/// admission is separate; no state, capture, model or physical claim is changed.
fn capture_coherence(
    profile: ExperimentalProfile,
    captures: &[Option<[super::SoftwareClock; 3]>; 3],
) -> SeriesCoherence {
    match series_coherence_residuals(captures) {
        Ok(None) => SeriesCoherence::Unavailable(CoherenceUnavailable::NoFacts),
        Ok(Some(residual)) if residual > profile.maximum_residual_ns() => {
            SeriesCoherence::ObservedInconsistent
        }
        Ok(Some(_)) => SeriesCoherence::ObservedCompatible,
        Err(reason) => SeriesCoherence::Unavailable(CoherenceUnavailable::Capture(reason)),
    }
}

/// A genuine returned series paired only with its pre-acquisition chart provenance.
///
/// Retention owns no session borrow, socket, runtime or current permission. The
/// sole constructor is the consuming guard's actual driver-return path; public
/// callers cannot attach a report, replace it or supply an equal-origin chart.
/// ```compile_fail,E0451
/// use nevrotick_engine::{AcquisitionReport, SeriesReport};
/// fn attach(base: AcquisitionReport, series: SeriesReport) -> AcquisitionReport {
///     AcquisitionReport { series, ..base }
/// }
/// ```
/// ```compile_fail,E0616
/// use nevrotick_engine::{AcquisitionReport, SeriesReport};
/// fn substitute(mut report: AcquisitionReport, series: SeriesReport) {
///     report.series = series;
/// }
/// ```
pub struct AcquisitionReport {
    /// Genuine returned report, retaining every actual execution/request record.
    series: SeriesReport,
    /// Same immutable chart captured by the guard before preparation.
    chart: Arc<SessionChart>,
    /// Sole assessment of this run's original retained captures, never a mapping claim.
    coherence: SeriesCoherence,
}

impl AcquisitionReport {
    /// Borrow the actual series without moving or replacing its original facts.
    #[must_use]
    pub const fn series(&self) -> &SeriesReport {
        &self.series
    }

    /// Copy the one-run observed assessment without revising historical facts/ages.
    /// This conveys neither present permission nor uninterrupted mapping evidence.
    /// The returned series and historical reference views remain available even
    /// when the assessment retired future preparation.
    /// ```compile_fail,E0616
    /// use nevrotick_engine::{AcquisitionReport, SeriesCoherence};
    /// fn falsify(mut report: AcquisitionReport) {
    ///     report.coherence = SeriesCoherence::ObservedCompatible;
    /// }
    /// ```
    #[must_use]
    pub const fn coherence(&self) -> SeriesCoherence {
        self.coherence
    }

    /// Return the original session chart reference, without conversion or renewal.
    #[must_use]
    pub fn chart_origin(&self) -> Instant {
        self.chart.origin
    }

    /// Compare actual chart allocation identity, never endpoints or origin labels.
    /// Equality establishes shared historical provenance only, not currentness,
    /// clock/mapping continuity, production admission or cross-run conversion.
    #[must_use]
    pub fn same_session(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.chart, &other.chart)
    }

    /// Derive one original successful measurement's exact session-chart midpoint.
    ///
    /// Every `u8` position is checked against the original three-attempt plan.
    /// Only this report's own immutable facts, successful measurement and private
    /// session origin participate. The existing checked midpoint owner preserves
    /// exact whole/half nanoseconds. No ambient clock is read, no age is renewed,
    /// and no state or statistical-admission permission changes. Repeated calls,
    /// including after session retirement, retain historical meaning only.
    /// Success allocates nothing; rejected midpoint admission may temporarily
    /// allocate the existing bounded Garde report.
    /// # Errors
    /// In order: reject an absent position, absent facts, the original measurement
    /// refusal, or checked reference chronology/range/arithmetic/output refusal.
    pub fn reference_at(
        &self,
        position: u8,
    ) -> Result<AcquisitionReference<'_>, AcquisitionReferenceRefusal> {
        let attempt = self
            .series
            .attempts()
            .get(usize::from(position))
            .ok_or(AcquisitionReferenceRefusal::AttemptPosition)?;
        let facts = attempt
            .report()
            .and_then(SlotReport::facts)
            .ok_or(AcquisitionReferenceRefusal::FactsUnavailable)?;
        let measurement = facts
            .measurement()
            .map_err(AcquisitionReferenceRefusal::Measurement)?;
        let midpoint = reference_from_instants(
            self.chart.origin,
            facts.send_before().monotonic(),
            facts.read_return().monotonic(),
        )
        .map_err(AcquisitionReferenceRefusal::Reference)?;
        Ok(AcquisitionReference {
            report: self,
            attempt,
            facts,
            measurement,
            midpoint,
        })
    }
}

impl fmt::Debug for AcquisitionReport {
    /// Redact original report, chart origin and identity with fixed text.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("AcquisitionReport { .. }")
    }
}

/// Fixed refusal of a historical session-reference view, without input payloads.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum AcquisitionReferenceRefusal {
    /// The requested original position is outside the fixed three-attempt plan.
    AttemptPosition,
    /// This original position is unstarted or its actual terminal outcome has no facts.
    FactsUnavailable,
    /// Preserve exactly the original facts' conditional-measurement refusal.
    Measurement(MeasurementRefusal),
    /// Preserve the checked midpoint owner's chronology, range or output refusal.
    /// This path produces no run mismatch or row/anchor measurement variants.
    Reference(RelativeMeasurementRefusal),
}

impl fmt::Debug for AcquisitionReferenceRefusal {
    /// Keep even the bounded typed cause outside automatic diagnostic formatting.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("AcquisitionReferenceRefusal { .. }")
    }
}

impl fmt::Display for AcquisitionReferenceRefusal {
    /// Write fixed operation text; retain original causes only through typed access.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::AttemptPosition => "session reference attempt position unavailable",
            Self::FactsUnavailable => "session reference facts unavailable",
            Self::Measurement(_) => "session reference measurement unavailable",
            Self::Reference(_) => "session reference midpoint unavailable",
        })
    }
}

impl Error for AcquisitionReferenceRefusal {
    /// Borrow the exact bounded cause without copying reports, captures or input.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Measurement(reason) => Some(reason),
            Self::Reference(reason) => Some(reason),
            Self::AttemptPosition | Self::FactsUnavailable => None,
        }
    }
}

/// A report-owned original measurement with an exact historical session reference.
///
/// Only [`AcquisitionReport::reference_at`] constructs this immutable association.
/// The private factory retains the genuine report, attempt, facts, successful
/// measurement and its checked midpoint; neither matching lifetimes nor a copied
/// scalar establishes that association. A retained view supplies no currentness,
/// clock/mapping continuity, calibration, authentication or statistical authority.
///
/// A legal consumer borrows the original position and exact parts:
/// ```
/// use nevrotick_engine::AcquisitionReference;
/// fn parts(view: &AcquisitionReference<'_>) -> (u8, u64, bool) {
///     let midpoint = view.midpoint();
///     (view.attempt().index(), midpoint.whole_nanoseconds(), midpoint.has_half_nanosecond())
/// }
/// let _consumer = parts;
/// ```
/// Independent objects cannot manufacture an associated view:
/// ```compile_fail,E0451
/// use nevrotick_engine::{AcquisitionReference, AcquisitionReport, ExperimentalMeasurement,
///     ExperimentalReferenceNs, SeriesAttempt, TransportFacts};
/// fn forge<'a>(report: &'a AcquisitionReport, attempt: &'a SeriesAttempt,
///     facts: &'a TransportFacts, measurement: &'a ExperimentalMeasurement,
///     midpoint: ExperimentalReferenceNs) -> AcquisitionReference<'a> {
///     AcquisitionReference { report, attempt, facts, measurement, midpoint }
/// }
/// ```
/// Even an existing view cannot have its original facts replaced:
/// ```compile_fail,E0616
/// use nevrotick_engine::{AcquisitionReference, TransportFacts};
/// fn substitute<'a>(mut view: AcquisitionReference<'a>, facts: &'a TransportFacts) {
///     view.facts = facts;
/// }
/// ```
/// Its owning report must outlive the view:
/// ```compile_fail,E0515
/// use nevrotick_engine::{AcquisitionReference, AcquisitionReferenceRefusal, AcquisitionReport};
/// fn escape<'a>(report: AcquisitionReport)
///     -> Result<AcquisitionReference<'a>, AcquisitionReferenceRefusal> {
///     report.reference_at(0)
/// }
/// ```
pub struct AcquisitionReference<'report> {
    /// Genuine immutable wrapper that owns this association and session chart.
    report: &'report AcquisitionReport,
    /// That wrapper's actual original selected position, never relabelled.
    attempt: &'report SeriesAttempt,
    /// Original facts held by that position's terminal report.
    facts: &'report TransportFacts,
    /// Already successful immutable measurement owned by those exact facts.
    measurement: &'report ExperimentalMeasurement,
    /// Checked exact midpoint of the facts' lower captures in the private chart.
    midpoint: ExperimentalReferenceNs,
}

impl<'report> AcquisitionReference<'report> {
    /// Borrow the original wrapper and its historical chart association.
    #[must_use]
    pub const fn report(&self) -> &'report AcquisitionReport {
        self.report
    }

    /// Borrow the original position, including its unchanged request association.
    #[must_use]
    pub const fn attempt(&self) -> &'report SeriesAttempt {
        self.attempt
    }

    /// Borrow the exact original facts without replacing captures or acquisition age.
    #[must_use]
    pub const fn facts(&self) -> &'report TransportFacts {
        self.facts
    }

    /// Borrow the already successful original conditional nominal measurement.
    #[must_use]
    pub const fn measurement(&self) -> &'report ExperimentalMeasurement {
        self.measurement
    }

    /// Copy the exact whole/half-nanosecond midpoint without copying its provenance.
    #[must_use]
    pub const fn midpoint(&self) -> ExperimentalReferenceNs {
        self.midpoint
    }
}

impl fmt::Debug for AcquisitionReference<'_> {
    /// Redact report, original captures/measurement and scalar with fixed text.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("AcquisitionReference { .. }")
    }
}

/// Direct lifecycle controls and independently observed genuine session acquisitions.
#[cfg(test)]
#[path = "../tests/private/session.rs"]
mod tests;

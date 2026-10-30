// SPDX-License-Identifier: GPL-2.0-only
//! Local S1.1e fixture driver for CHECK-6/7/25, authored 2026-10-29.
//!
//! Inputs and schedules come from the preserved reviewed S1.1e card. No upstream
//! algorithm is copied. This module never imports the independent expectations,
//! product arithmetic, parser, or era-unfolding helpers. It builds complete wire
//! inputs, calls only public association operations, and projects actual outputs.
//! Four explicit fixture coordinates and fixed arrays own all work and storage.
//! A fully checked prepared script exists before the first product construction.
//! Returned observations intentionally retain bounded evidence; live packet slots
//! and pending work are observed before the fresh owner is dropped.

use std::{fmt, net::SocketAddr, num::NonZeroU64};

use ntp_exchange::{
    ArmError, Association, AssociationContext, AssociationGeneration, Binding, CancelError,
    ClockGeneration, CompletionFacts, ConfigurationGeneration, ContextError, ExpireError,
    FixtureEvent, Generations, PeerEraContext, ReceiptFacts, ReceiveError, RequestId,
    RequestTiming, SocketId, SourceId, UnauthenticatedFixtureObservation, WireToken,
};
use time_core::{
    FixtureContext, FixtureHistory, FixtureObservation, HistoryError, ObservationError,
};
use time_domain::{ActiveAwakeNs, EraWindow, MonotonicNs, UtcCoordinateQ32};

/// Literal selected run identities; no implicit subruns are created.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum CaseId {
    /// Symmetric healthy exchange.
    R01,
    /// Early peer receipt and longer reverse path.
    R02,
    /// Later peer receipt and shorter reverse path.
    R03,
    /// Explicit local oscillator slope.
    R04,
    /// Peer transmit timestamp fault.
    R05,
    /// Dropped reply, explicit expiry, recovery.
    R06,
    /// Cancel/rearm and reordered old reply.
    R07,
    /// Queued reply processed one ns before deadline.
    R08,
    /// Exact deadline rejection, expiry, no-pending replay.
    R09,
    /// One ns late processing and explicit expiry.
    R10,
    /// Configuration advance and healthy successor.
    R11,
    /// Binding and association advance.
    R12,
    /// Rejected configuration regression.
    R13,
    /// Rejected binding change without association advance.
    R14,
    /// Positive local UTC step and mapping retirement.
    R15,
    /// Negative local UTC step and mapping retirement.
    R16,
    /// Suspend/resume and generation retirement.
    R17,
    /// Suspend/resume and monotonic expiry retirement.
    R18,
    /// Hostile packet, healthy packet, control, recovery.
    R19,
}

/// Exactly the reviewed nineteen baseline scripts, in their declared order.
pub(super) const ALL_CASES: [CaseId; 19] = [
    CaseId::R01,
    CaseId::R02,
    CaseId::R03,
    CaseId::R04,
    CaseId::R05,
    CaseId::R06,
    CaseId::R07,
    CaseId::R08,
    CaseId::R09,
    CaseId::R10,
    CaseId::R11,
    CaseId::R12,
    CaseId::R13,
    CaseId::R14,
    CaseId::R15,
    CaseId::R16,
    CaseId::R17,
    CaseId::R18,
    CaseId::R19,
];

/// Baseline and bounded reset replays are separate run inventories.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum RunId {
    /// One of the nineteen reviewed baseline cases.
    Baseline(CaseId),
    /// Separate fixed reset script, counted outside the baseline inventory.
    ResetRecovery,
    /// Separately selected exchange-to-history fixture, outside the baseline inventory.
    Composition,
}

/// Exact fixture seconds relative to B, reduced with a positive denominator.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Fraction {
    /// Signed numerator of seconds relative to B.
    pub numerator: i128,
    /// Positive reduced denominator.
    pub denominator: i128,
}

/// Independent readings; neither UTC reading supplies elapsed time to D.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ClockReading {
    /// Reference truth, independent of local oscillator/step.
    pub true_utc: Fraction,
    /// Observed local UTC mapping, including explicit slope/step.
    pub local_utc: Fraction,
    /// Event/deadline reading, advancing through suspension.
    pub monotonic_ns: u64,
    /// Separately initialized awake reading, excluding declared suspension.
    pub active_awake_ns: u64,
    /// Trusted fixture mapping generation at this event.
    pub mapping_generation: u64,
}

/// Pure public-getter observation, without token or private pending contents.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct StateSnapshot {
    /// Real owner's immutable binding and generations.
    pub context: AssociationContext,
    /// Actual pending request ID, if present.
    pub pending: Option<u64>,
    /// Actual next request ID, preserving exhaustion.
    pub next_request: Option<u64>,
}

/// Immutable complete packet capture. `length` is the actual receive slice size.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Capture {
    /// Complete captured header buffer, including the unused last malformed byte.
    pub packet: [u8; 48],
    /// Actual slice length admitted to `receive`.
    pub length: usize,
    /// Transport fixture binding saved at capture.
    pub binding: Binding,
    /// Saved mapping generation; never refreshed at processing.
    pub mapping_generation: u64,
    /// Saved monotonic arrival instant.
    pub arrival_ns: u64,
    /// Saved exact local T4 Q32 ticks.
    pub local_t4: i128,
    /// Four trajectory readings at original capture.
    pub clocks: ClockReading,
}

/// Every dispatched fixture event, including events that do not call D.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum EventKind {
    /// Fresh association construction, charged as one D action.
    Construct,
    /// Explicit request admission.
    Arm,
    /// Queue-only packet capture, without receive.
    Capture,
    /// Same-instant capture and receive, charged as two events.
    CaptureReceive,
    /// Receive using an already saved capture.
    Receive,
    /// Trusted explicit request cancellation.
    Cancel,
    /// Trusted explicit monotonic expiry.
    Expire,
    /// Explicit lifecycle context update.
    AdvanceContext,
    /// Local UTC-only fixture discontinuity.
    Step,
    /// Awake-trajectory suspension marker.
    Suspend,
    /// Awake-trajectory resume marker.
    Resume,
    /// Explicit lost-packet slot removal.
    Drop,
    /// Explicit release of all fixture packet slots.
    Release,
}

/// Immutable projection of all actual completion identity and timing getters.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct FactsProjection {
    /// Completed request's actual binding.
    pub binding: Binding,
    /// Actual completed request ID.
    pub request: u64,
    /// Actual committed generation tuple.
    pub generations: Generations,
    /// Actual committed receipt mapping generation.
    pub mapping_generation: u64,
    /// Saved monotonic send reading.
    pub send_ns: u64,
    /// Saved monotonic arrival reading.
    pub arrival_ns: u64,
    /// Explicit receive processing instant.
    pub processing_ns: u64,
    /// Exclusive monotonic deadline.
    pub deadline_ns: u64,
}

/// All time-result getters, including signed raw arithmetic and peer claims.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct TimeProjection {
    /// Actual immutable completion facts.
    pub facts: FactsProjection,
    /// Actual separately saved local-send Q32 ticks.
    pub t1: i128,
    /// Actual unfolded peer receive Q32 ticks.
    pub t2: i128,
    /// Actual unfolded peer transmit Q32 ticks.
    pub t3: i128,
    /// Actual captured local receive Q32 ticks.
    pub t4: i128,
    /// Actual signed Q33 numerator, positive when local is behind.
    pub offset: i128,
    /// Actual signed raw RTT Q32 numerator, without clamping.
    pub rtt: i128,
    /// Raw peer leap indicator claim.
    pub leap: u8,
    /// Raw peer stratum claim.
    pub stratum: u8,
    /// Raw signed peer poll claim.
    pub poll: i8,
    /// Raw signed peer precision claim.
    pub precision: i8,
    /// Uninterpreted wire root-delay bits.
    pub root_delay: u32,
    /// Uninterpreted wire root-dispersion bits.
    pub root_dispersion: u32,
    /// Unauthenticated reference identifier claim.
    pub reference_id: u32,
    /// Uninterpreted reference timestamp bits.
    pub reference_timestamp: u64,
}

/// Opaque control result has no invented time or scheduling fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ControlProjection {
    /// Actual control completion facts.
    pub facts: FactsProjection,
    /// Opaque reference-ID control bits.
    pub code: u32,
}

/// Projection only; a completed product event is never manufactured by fixtures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Completed {
    /// Actual unauthenticated time result.
    Time(TimeProjection),
    /// Actual opaque stratum-zero control result.
    Control(ControlProjection),
}

/// Real public-operation return values, without interpretation by an oracle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[expect(
    clippy::large_enum_variant,
    reason = "Actual receive facts remain inline in an allocation-free bounded trace; its measured type size is independently checked."
)]
pub(super) enum Outcome {
    /// Fixture-only event made no D call.
    None,
    /// Fresh D owner was constructed.
    Constructed,
    /// Actual request admission return value.
    Armed(Result<u64, ArmError>),
    /// Actual complete-byte receive return value.
    Received(Result<Completed, ReceiveError>),
    /// Actual explicit cancellation return value.
    Cancelled(Result<u64, CancelError>),
    /// Actual explicit expiry return value.
    Expired(Result<u64, ExpireError>),
    /// Actual context-update return value.
    Advanced(Result<Option<u64>, ContextError>),
}

/// One immutable action record. Clocks are read after that explicit fixture event.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Observation {
    /// Dispatched input event identity.
    pub kind: EventKind,
    /// Literal tie-break ordinal at the event's monotonic instant.
    pub ordinal: u8,
    /// Post-event independent trajectory readings.
    pub clocks: ClockReading,
    /// Getter state before the event; absent only before construction.
    pub before: Option<StateSnapshot>,
    /// Getter state immediately after the event.
    pub after: StateSnapshot,
    /// Actual return value or explicit absence of D action.
    pub outcome: Outcome,
    /// Actual captured/received bytes and saved receipt facts when applicable.
    pub capture: Option<Capture>,
    /// Actual occupied packet slots after the event.
    pub queue_len: usize,
}

/// Actual counts and pre-teardown state of one fresh, bounded public owner.
#[derive(Debug)]
pub(super) struct RunReport {
    /// Baseline identity or separate reset-envelope identity.
    pub run_id: RunId,
    /// Only the first `event_count` records are populated.
    pub observations: [Option<Observation>; 32],
    /// Actual observation/trace records; a combined capture/receive has one record.
    pub event_count: usize,
    /// Every capture and D action is charged separately, even in one record.
    pub charged_event_count: usize,
    /// Association-owner construction/mutators/receives, excluding typed inputs.
    pub product_action_count: usize,
    /// Actual logical trace-array bytes; excludes other stack/RSS/allocator costs.
    pub trace_storage_bytes: usize,
    /// Actual getter state immediately before owner teardown.
    pub final_state: StateSnapshot,
    /// Actual packet-slot occupancy immediately before queue teardown.
    pub final_queue_len: usize,
}

/// Actual, exact committed row projection, without reconstructing a domain value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct HistoryRow {
    /// Complete fixture assertion retained by this row.
    pub context: FixtureContext,
    /// Exact monotonic receipt.
    pub arrival_ns: u64,
    /// Exact phase numerator over 2^33.
    pub phase: i128,
    /// Unchanged divisor bits.
    pub divisor_bits: u64,
}

/// All public report associations and binary64 payload bits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct HistoryStatistics {
    /// Actual report context.
    pub context: FixtureContext,
    /// Actual report reference.
    pub reference_ns: u64,
    /// Actual statistics frame.
    pub frame: u64,
    /// Actual statistics population.
    pub count: usize,
    /// Actual residual degrees of freedom.
    pub degrees: usize,
    /// Fitted phase bits.
    pub phase_bits: u64,
    /// Fitted rate bits.
    pub rate_bits: u64,
    /// Weighted residual sum bits.
    pub sse_bits: u64,
    /// Residual scale bits.
    pub scale_bits: u64,
    /// Normalized variance bits.
    pub variance_bits: u64,
}

/// Observed state after one script action; four rows are the declared run maximum.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct HistorySnapshot {
    /// Actual history assertion.
    pub context: FixtureContext,
    /// Actual committed row count.
    pub count: usize,
    /// Exact newest retained time.
    pub reference_ns: Option<u64>,
    /// Actual bounded committed rows.
    pub rows: [Option<HistoryRow>; 4],
    /// Actual optional report projection.
    pub report: Option<HistoryStatistics>,
    /// Actual observation-construction attempts.
    pub constructions: usize,
    /// Actual history append attempts.
    pub appends: usize,
    /// Actual successful commits.
    pub commits: usize,
    /// Actual rejected history appends.
    pub history_rejections: usize,
    /// Actual row getter calls across all snapshots so far.
    pub row_getters: usize,
    /// Actual present reports projected across snapshots so far.
    pub present_reports: usize,
}

/// A completed request's distinct downstream outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum DeliveryResult {
    /// At least one complete identity role differs.
    BindingMismatch,
    /// Actual checked construction rejected.
    Observation(ObservationError),
    /// Actual history append rejected.
    History(HistoryError),
    /// The append committed.
    Committed,
}

/// Actual completion identity accompanies every attempt, including failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Delivery {
    /// Actual completion identity and timing.
    pub facts: FactsProjection,
    /// Actual downstream outcome.
    pub result: DeliveryResult,
}

/// Frozen scenario binding; trajectory/origin are explicit fixture assertions.
struct Composition {
    /// Complete declared binding and generation tuple.
    context: AssociationContext,
    /// Separately asserted receipt mapping.
    mapping: ClockGeneration,
    /// Fixed origin and fixture-clock trajectory.
    timeline: FixtureTimeline,
    /// Sole history owner.
    history: FixtureHistory,
    /// Actual bounded ordered delivery ledger.
    deliveries: [Option<Delivery>; 5],
    /// Number of observed completed deliveries.
    delivery_count: usize,
    /// Number of observation constructor calls.
    constructions: usize,
    /// Number of history append calls.
    appends: usize,
    /// Number of successful history commits.
    commits: usize,
    /// Number of rejected history appends.
    history_rejections: usize,
    /// Actual row getter calls while recording.
    row_getters: usize,
    /// Actual present report projections while recording.
    present_reports: usize,
    /// Actual post-action history evidence.
    snapshots: [Option<HistorySnapshot>; EVENT_CAP],
}

/// Declared fixture origin and trajectory; these are not native clock identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct FixtureTimeline {
    /// Exact declared UTC-coordinate base.
    pub base: i128,
    /// Declared initial monotonic reading.
    pub initial_monotonic_ns: u64,
    /// Declared initial active-awake reading.
    pub initial_awake_ns: u64,
    /// Whether the declared oscillator slope is enabled.
    pub oscillator: bool,
}

/// Bounded actual evidence, separate from the existing baseline report.
pub(super) struct CompositionReport {
    /// Real association and fixture action trace.
    pub run: RunReport,
    /// Declared trajectory bound to this run.
    pub timeline: FixtureTimeline,
    /// Actual completion ledger.
    pub deliveries: [Option<Delivery>; 5],
    /// Actual populated ledger count.
    pub delivery_count: usize,
    /// Actual history snapshots after each action.
    pub snapshots: [Option<HistorySnapshot>; EVENT_CAP],
    /// Logical size of the added trace and ledger, excluding owner scratch.
    pub history_trace_bytes: usize,
    /// Logical composition-owner size, excluding nested append scratch.
    pub history_owner_bytes: usize,
}

/// Admit one ledger slot before any observation construction or history effect.
/// Duplicate identity takes precedence over an out-of-range slot index.
/// # Errors
/// Rejects a previously recorded request or an out-of-range slot index.
fn admit_delivery(
    deliveries: &[Option<Delivery>; 5],
    count: usize,
    request: u64,
) -> Result<(), HarnessError> {
    if deliveries
        .iter()
        .flatten()
        .any(|entry| entry.facts.request == request)
    {
        return Err(HarnessError::InvalidFixture);
    }
    if deliveries.get(count).is_none() {
        return Err(HarnessError::TraceCapacity);
    }
    Ok(())
}

/// Check the last valid and first out-of-range indices against an empty ledger.
/// This calls no exchange, observation constructor or history operation.
pub(super) fn composition_ledger_capacity_control()
-> (Result<(), HarnessError>, Result<(), HarnessError>) {
    let ledger = [None; 5];
    (admit_delivery(&ledger, 4, 1), admit_delivery(&ledger, 5, 1))
}

impl Composition {
    /// Bind one fixture context to one complete declared transport/context identity.
    /// # Errors
    /// Rejects invalid fixture labels or binding without creating an owner.
    fn new() -> Result<Self, HarnessError> {
        let labels = FixtureContext::new(nonzero(17)?, nonzero(23)?, nonzero(29)?);
        Ok(Self {
            context: context(false, 1, 1, 1)?,
            mapping: ClockGeneration::new(1),
            timeline: FixtureTimeline {
                base: BASE,
                initial_monotonic_ns: START,
                initial_awake_ns: SECOND / 4,
                oscillator: false,
            },
            history: FixtureHistory::new(labels),
            deliveries: [None; 5],
            delivery_count: 0,
            constructions: 0,
            appends: 0,
            commits: 0,
            history_rejections: 0,
            row_getters: 0,
            present_reports: 0,
            snapshots: [None; EVENT_CAP],
        })
    }

    /// Consume exactly one actual completed event; never retry or return its payload.
    /// # Errors
    /// Rejects duplicate delivery or an out-of-range ledger index, without retry.
    #[expect(
        clippy::needless_pass_by_value,
        reason = "The real non-Clone completion is consumed once at this fixture boundary."
    )]
    fn deliver(&mut self, time: UnauthenticatedFixtureObservation) -> Result<(), HarnessError> {
        let actual = facts(time.facts());
        let id = actual.request;
        admit_delivery(&self.deliveries, self.delivery_count, id)?;
        // Stage every potentially needed increment before the first downstream effect.
        let next_delivery = self
            .delivery_count
            .checked_add(1)
            .ok_or(HarnessError::TraceCapacity)?;
        let next_constructions = self
            .constructions
            .checked_add(1)
            .ok_or(HarnessError::TraceCapacity)?;
        let next_appends = self
            .appends
            .checked_add(1)
            .ok_or(HarnessError::TraceCapacity)?;
        let next_commits = self
            .commits
            .checked_add(1)
            .ok_or(HarnessError::TraceCapacity)?;
        let next_rejections = self
            .history_rejections
            .checked_add(1)
            .ok_or(HarnessError::TraceCapacity)?;
        // Reserve the actual slot before any downstream effect; exclusive field
        // borrowing prevents intervening ledger mutation or failed late admission.
        let slot = self
            .deliveries
            .get_mut(self.delivery_count)
            .ok_or(HarnessError::TraceCapacity)?;
        let result = if time.facts().binding() != self.context.binding()
            || time.facts().generations() != self.context.generations()
            || time.facts().mapping_generation() != self.mapping
        {
            DeliveryResult::BindingMismatch
        } else {
            self.constructions = next_constructions;
            match FixtureObservation::new(
                self.history.context(),
                time.facts().arrival(),
                time.offset(),
                1.0,
            ) {
                Err(error) => DeliveryResult::Observation(error),
                Ok(observation) => {
                    self.appends = next_appends;
                    match self.history.try_append(observation) {
                        Ok(()) => {
                            self.commits = next_commits;
                            DeliveryResult::Committed
                        }
                        Err(error) => {
                            self.history_rejections = next_rejections;
                            DeliveryResult::History(error)
                        }
                    }
                }
            }
        };
        *slot = Some(Delivery {
            facts: actual,
            result,
        });
        self.delivery_count = next_delivery;
        Ok(())
    }

    /// Read actual public history state once after each declared fixture action.
    /// # Errors
    /// Rejects inconsistent rows or exhaustion of the fixed evidence capacity.
    fn record(&mut self, index: usize) -> Result<(), HarnessError> {
        let slot = self
            .snapshots
            .get_mut(index)
            .ok_or(HarnessError::TraceCapacity)?;
        let count = self.history.sample_count();
        if count > 4 {
            return Err(HarnessError::TraceCapacity);
        }
        let next_row_getters = self
            .row_getters
            .checked_add(count)
            .ok_or(HarnessError::TraceCapacity)?;
        let mut rows = [None; 4];
        for (index, slot) in rows.iter_mut().take(count).enumerate() {
            let row = self
                .history
                .observation(index)
                .ok_or(HarnessError::InvalidFixture)?;
            *slot = Some(HistoryRow {
                context: row.context(),
                arrival_ns: row.observed_at().nanoseconds(),
                phase: row.phase().numerator(),
                divisor_bits: row.variance_divisor().to_bits(),
            });
        }
        let report = self.history.report().map(|report| {
            let stats = report.statistics();
            HistoryStatistics {
                context: report.context(),
                reference_ns: report.reference().nanoseconds(),
                frame: stats.frame().get(),
                count: stats.sample_count(),
                degrees: stats.degrees_of_freedom(),
                phase_bits: stats.line().phase_seconds().to_bits(),
                rate_bits: stats.line().phase_rate_seconds_per_second().to_bits(),
                sse_bits: stats.weighted_sse_seconds_squared().to_bits(),
                scale_bits: stats.residual_scale_seconds_squared().to_bits(),
                variance_bits: stats.normalized_data_variance_seconds_squared().to_bits(),
            }
        });
        let next_present_reports = self
            .present_reports
            .checked_add(usize::from(report.is_some()))
            .ok_or(HarnessError::TraceCapacity)?;
        let snapshot = HistorySnapshot {
            context: self.history.context(),
            count,
            reference_ns: self.history.reference().map(MonotonicNs::nanoseconds),
            rows,
            report,
            constructions: self.constructions,
            appends: self.appends,
            commits: self.commits,
            history_rejections: self.history_rejections,
            row_getters: next_row_getters,
            present_reports: next_present_reports,
        };
        self.row_getters = next_row_getters;
        self.present_reports = next_present_reports;
        *slot = Some(snapshot);
        Ok(())
    }
}

/// Fixture failures are distinct from all product rejections.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum HarnessError {
    /// More than twenty literal baseline runs.
    RunCapacity,
    /// Too many charged events or raw event slots.
    EventCapacity,
    /// Too many held packets or invalid packet-slot index.
    QueueCapacity,
    /// Event precedes the start or exceeds the eight-second horizon.
    Horizon,
    /// Too many retained observation records.
    TraceCapacity,
    /// Fixture UTC does not have an exact Q32 representation.
    InexactCoordinate,
    /// Total order is not strictly increasing.
    Order,
    /// Illegal slope or suspend/resume relationship.
    ClockSegment,
    /// Integer overflow or invalid rational denominator.
    Arithmetic,
    /// Absolute UTC coordinate is outside its checked domain.
    Coordinate,
    /// Wire timestamp cannot be encoded without truncation.
    Wire,
    /// Script attempts an action before owner construction.
    MissingOwner,
    /// Script references an empty or absent packet slot.
    MissingCapture,
    /// Internal input-table relationship is inconsistent.
    InvalidFixture,
    /// Public immutable-input domain construction rejected fixture data.
    Domain,
}

impl fmt::Display for HarnessError {
    /// Emit a fixed variant name; no external payload or effects are involved.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for HarnessError {}

/// Independent raw inputs for validation-only exact/over-capacity controls.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ValidationInput {
    /// Raw declared literal-run count.
    pub run_count: usize,
    /// Raw declared expanded charged-event count.
    pub event_count: usize,
    /// Raw declared held-packet count.
    pub queued_count: usize,
    /// Modeled elapsed horizon in monotonic nanoseconds.
    pub horizon_ns: u64,
    /// Raw declared observation-record count.
    pub trace_count: usize,
    /// Exact relative-UTC seconds numerator to challenge conversion.
    pub utc_numerator: i128,
    /// Positive relative-UTC seconds denominator.
    pub utc_denominator: i128,
}

/// These paths never construct or call a product association.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ValidationReport {
    /// Actual pure-admission result, independent of product behavior.
    pub error: Option<HarnessError>,
    /// Zero Association construction/mutator/receive calls; typed inputs differ.
    pub product_action_count: usize,
}

/// Named validation helpers; input-driven controls remain available separately.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ValidationControl {
    /// Exactly twenty declared literal runs.
    ExactRunCap,
    /// Twenty-one declared literal runs.
    RunOverflow,
    /// Exactly thirty-two charged events.
    ExactEventCap,
    /// Thirty-three charged events.
    EventOverflow,
    /// Exactly two held packet slots.
    ExactQueueCap,
    /// Three held packet slots.
    QueueOverflow,
    /// Exactly eight modeled seconds.
    ExactHorizon,
    /// Eight seconds plus one nanosecond.
    HorizonOverflow,
    /// Exactly thirty-two trace records.
    ExactTraceCap,
    /// Thirty-three trace records.
    TraceOverflow,
    /// Exact dyadic relative UTC conversion.
    ExactCapture,
    /// One-nanosecond relative UTC conversion, inexact in Q32.
    InexactCapture,
    /// Real raw script contains a regressing total-order element.
    InvalidOrder,
    /// Real raw script resumes without first suspending.
    InvalidSegment,
    /// Relative coordinate overflows when the epoch is added.
    CoordinateOverflow,
    /// Negative full wire timestamp cannot be encoded.
    WireOverflow,
}

/// Explicit fixture epoch in Q32 ticks, never an interpreted date or trust anchor.
const BASE: i128 = 0xe000_0000_0000_0000;
/// Integer nanoseconds per second, independent of Q32 conversion.
const SECOND: u64 = 1_000_000_000;
/// Initial monotonic instant, distinct from initial active-awake quarter second.
const START: u64 = SECOND;
/// Exact path units shared only by fixture input construction.
const SIXTEENTH: u64 = 62_500_000;
/// Fixed maximum dispatch/observation slots; no allocation or recursion.
const EVENT_CAP: usize = 32;

/// Reduce a signed rational with checked absolute value and positive denominator.
/// Failure returns no partial value; work is bounded by integer width.
/// # Errors
/// Rejects nonpositive denominators or unrepresentable absolute numerators.
fn fraction(numerator: i128, denominator: i128) -> Result<Fraction, HarnessError> {
    if denominator <= 0 {
        return Err(HarnessError::Arithmetic);
    }
    let mut left = numerator.checked_abs().ok_or(HarnessError::Arithmetic)?;
    let mut right = denominator;
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    Ok(Fraction {
        numerator: numerator / left,
        denominator: denominator / left,
    })
}

/// Convert exact fixture seconds relative to B; reject any fractional Q32 tick.
/// Checked product coordinate construction is retained; no rounding rule exists.
/// # Errors
/// Rejects invalid denominators, overflow, fractional Q32 ticks or domain range.
fn utc(value: Fraction) -> Result<UtcCoordinateQ32, HarnessError> {
    if value.denominator <= 0 {
        return Err(HarnessError::Arithmetic);
    }
    let scaled = value
        .numerator
        .checked_mul(1_i128 << 32)
        .ok_or(HarnessError::Arithmetic)?;
    if scaled % value.denominator != 0 {
        return Err(HarnessError::InexactCoordinate);
    }
    let ticks = BASE
        .checked_add(scaled / value.denominator)
        .ok_or(HarnessError::Coordinate)?;
    UtcCoordinateQ32::new(ticks).map_err(|_| HarnessError::Coordinate)
}

/// Check every resource dimension before considering the exact UTC conversion.
/// This is pure validation, so both valid and invalid inputs perform zero D work.
/// # Errors
/// Rejects any exceeded resource cap or invalid/inexact UTC conversion.
fn validate_input(input: ValidationInput) -> Result<(), HarnessError> {
    if input.run_count > 20 {
        return Err(HarnessError::RunCapacity);
    }
    if input.event_count > EVENT_CAP {
        return Err(HarnessError::EventCapacity);
    }
    if input.queued_count > 2 {
        return Err(HarnessError::QueueCapacity);
    }
    if input.horizon_ns > 8 * SECOND {
        return Err(HarnessError::Horizon);
    }
    if input.trace_count > EVENT_CAP {
        return Err(HarnessError::TraceCapacity);
    }
    utc(fraction(input.utc_numerator, input.utc_denominator)?)?;
    Ok(())
}

/// Validate caller-supplied fixture bounds/conversion without any product action.
pub(super) fn validate_only(input: ValidationInput) -> ValidationReport {
    ValidationReport {
        error: validate_input(input).err(),
        product_action_count: 0,
    }
}

/// Reject a wire timestamp outside the complete unsigned 64-bit fixture era.
/// This encoder check is pure and never wraps or truncates a coordinate.
/// # Errors
/// Rejects negative timestamps and values larger than the wire representation.
fn wire_ticks(value: i128) -> Result<u64, HarnessError> {
    u64::try_from(value).map_err(|_| HarnessError::Wire)
}

/// Verify one literal total-order edge; ties require strictly increasing ordinal.
/// # Errors
/// Rejects any non-increasing monotonic/ordinal total-order pair.
fn ordered(previous: (u64, u8), next: (u64, u8)) -> Result<(), HarnessError> {
    if next <= previous {
        return Err(HarnessError::Order);
    }
    Ok(())
}

/// Independent fixture mapping state, changed only by explicit fixture events.
#[derive(Clone, Copy)]
struct FixtureClock {
    /// Positive rational local oscillator slope numerator.
    slope_numerator: i128,
    /// Positive rational local oscillator slope denominator.
    slope_denominator: i128,
    /// Sum of explicit local UTC steps, in nanoseconds.
    local_step_ns: i128,
    /// Explicit trusted fixture mapping generation.
    mapping: u64,
    /// Open suspension's monotonic start, if any.
    suspended_at: Option<u64>,
    /// Closed suspensions' total monotonic duration.
    completed_suspend_ns: u64,
}

impl FixtureClock {
    /// Start separate explicit trajectories. No ambient clock, heap or D calls.
    fn new(oscillator: bool) -> Self {
        Self {
            slope_numerator: if oscillator { 17 } else { 1 },
            slope_denominator: if oscillator { 16 } else { 1 },
            local_step_ns: 0,
            mapping: 1,
            suspended_at: None,
            completed_suspend_ns: 0,
        }
    }

    /// Evaluate exact U/L fractions and separate M/A at an admitted event instant.
    /// Reject illegal segments/arithmetic before returning any reading.
    /// # Errors
    /// Rejects invalid slope, backwards time, illegal suspension or checked overflow.
    fn read(self, at: u64) -> Result<ClockReading, HarnessError> {
        if self.slope_numerator <= 0 || self.slope_denominator <= 0 {
            return Err(HarnessError::ClockSegment);
        }
        let elapsed = at.checked_sub(START).ok_or(HarnessError::ClockSegment)?;
        let true_numerator = i128::from(elapsed)
            .checked_mul(16)
            .and_then(|value| value.checked_add(i128::from(SECOND)))
            .ok_or(HarnessError::Arithmetic)?;
        let local_numerator = i128::from(elapsed)
            .checked_mul(self.slope_numerator)
            .and_then(|value| {
                self.local_step_ns
                    .checked_mul(self.slope_denominator)
                    .and_then(|step| value.checked_add(step))
            })
            .ok_or(HarnessError::Arithmetic)?;
        let local_denominator = i128::from(SECOND)
            .checked_mul(self.slope_denominator)
            .ok_or(HarnessError::Arithmetic)?;
        let current_suspend = match self.suspended_at {
            Some(start) => at.checked_sub(start).ok_or(HarnessError::ClockSegment)?,
            None => 0,
        };
        let sleeping = self
            .completed_suspend_ns
            .checked_add(current_suspend)
            .ok_or(HarnessError::Arithmetic)?;
        let awake = elapsed
            .checked_sub(sleeping)
            .and_then(|value| value.checked_add(SECOND / 4))
            .ok_or(HarnessError::ClockSegment)?;
        let monotonic = MonotonicNs::new(at);
        let active_awake = ActiveAwakeNs::new(awake);
        Ok(ClockReading {
            true_utc: fraction(true_numerator, i128::from(SECOND) * 16)?,
            local_utc: fraction(local_numerator, local_denominator)?,
            monotonic_ns: monotonic.nanoseconds(),
            active_awake_ns: active_awake.nanoseconds(),
            mapping_generation: self.mapping,
        })
    }

    /// Open exactly one pause; reject nested pause without changing prior state.
    /// # Errors
    /// Rejects nested suspension or an invalid current trajectory without mutation.
    fn suspend(&mut self, at: u64) -> Result<(), HarnessError> {
        if self.suspended_at.is_some() {
            return Err(HarnessError::ClockSegment);
        }
        self.read(at)?;
        self.suspended_at = Some(at);
        Ok(())
    }

    /// Close an existing pause with checked elapsed custody, atomically on success.
    /// # Errors
    /// Rejects missing/backwards suspension or duration overflow without mutation.
    fn resume(&mut self, at: u64) -> Result<(), HarnessError> {
        let start = self.suspended_at.ok_or(HarnessError::ClockSegment)?;
        let elapsed = at.checked_sub(start).ok_or(HarnessError::ClockSegment)?;
        let completed = self
            .completed_suspend_ns
            .checked_add(elapsed)
            .ok_or(HarnessError::Arithmetic)?;
        self.completed_suspend_ns = completed;
        self.suspended_at = None;
        Ok(())
    }
}

/// Run named pure controls through the same validators used by script preparation.
/// No branch constructs a D owner, even on success or malformed fixture input.
pub(super) fn validate_control(control: ValidationControl) -> ValidationReport {
    let mut input = ValidationInput {
        run_count: 19,
        event_count: 1,
        queued_count: 1,
        horizon_ns: 0,
        trace_count: 1,
        utc_numerator: 1,
        utc_denominator: 16,
    };
    let direct = match control {
        ValidationControl::ExactRunCap => {
            input.run_count = 20;
            None
        }
        ValidationControl::RunOverflow => {
            input.run_count = 21;
            None
        }
        ValidationControl::ExactEventCap => {
            input.event_count = 32;
            None
        }
        ValidationControl::EventOverflow => {
            input.event_count = 33;
            None
        }
        ValidationControl::ExactQueueCap => {
            input.queued_count = 2;
            None
        }
        ValidationControl::QueueOverflow => {
            input.queued_count = 3;
            None
        }
        ValidationControl::ExactHorizon => {
            input.horizon_ns = 8 * SECOND;
            None
        }
        ValidationControl::HorizonOverflow => {
            input.horizon_ns = 8 * SECOND + 1;
            None
        }
        ValidationControl::ExactTraceCap => {
            input.trace_count = 32;
            None
        }
        ValidationControl::TraceOverflow => {
            input.trace_count = 33;
            None
        }
        ValidationControl::ExactCapture => None,
        ValidationControl::InexactCapture => {
            input.utc_denominator = i128::from(SECOND);
            None
        }
        ValidationControl::InvalidOrder => Some(invalid_script_control(false)),
        ValidationControl::InvalidSegment => Some(invalid_script_control(true)),
        ValidationControl::CoordinateOverflow => Some(
            utc(Fraction {
                numerator: i128::MAX >> 32,
                denominator: 1,
            })
            .map(|_| ()),
        ),
        ValidationControl::WireOverflow => Some(wire_ticks(-1).map(|_| ())),
    };
    ValidationReport {
        error: direct.unwrap_or_else(|| validate_input(input)).err(),
        product_action_count: 0,
    }
}

/// Build a positive fixture identity through the actual checked identifier seam.
/// # Errors
/// Rejects zero instead of creating an unchecked fixture identifier.
fn nonzero(value: u64) -> Result<NonZeroU64, HarnessError> {
    NonZeroU64::new(value).ok_or(HarnessError::InvalidFixture)
}

/// Construct explicit documentation-only endpoints and independently named IDs.
/// # Errors
/// Propagates an invalid explicit nonzero source/socket fixture identity.
fn binding(second: bool) -> Result<Binding, HarnessError> {
    let id = if second { 2 } else { 1 };
    let endpoint = SocketAddr::from(([192, 0, 2, id], 123));
    Ok(Binding::new(
        SourceId::new(nonzero(u64::from(id))?),
        SocketId::new(nonzero(u64::from(id))?),
        endpoint,
    ))
}

/// Preserve the three explicitly supplied generation roles without arithmetic.
/// # Errors
/// Propagates fixture binding construction failure; no owner is changed.
fn context(
    second: bool,
    configuration: u64,
    association: u64,
    clock: u64,
) -> Result<AssociationContext, HarnessError> {
    Ok(AssociationContext::new(
        binding(second)?,
        Generations::new(
            ConfigurationGeneration::new(configuration),
            AssociationGeneration::new(association),
            ClockGeneration::new(clock),
        ),
    ))
}

/// Admit the exact inclusive [B,B+16s] peer windows; this grants no era trust.
/// # Errors
/// Rejects arithmetic overflow or an invalid exact peer-era window.
fn peer_eras() -> Result<PeerEraContext, HarnessError> {
    let last = BASE
        .checked_add(16_i128 << 32)
        .ok_or(HarnessError::Arithmetic)?;
    let window = EraWindow::new(BASE, last).map_err(|_| HarnessError::Domain)?;
    Ok(PeerEraContext::new(window, window))
}

/// Independent physical paths and explicit wire variation; all durations are ns.
#[derive(Clone, Copy)]
struct Reply {
    /// Independently declared physical send instant M.
    send: u64,
    /// Physical forward path duration.
    forward: u64,
    /// Physical peer processing duration.
    processing: u64,
    /// Physical reverse path duration.
    reverse: u64,
    /// Explicit wire correlation bits, never used as saved T1.
    token: u64,
    /// Whether transport fixture uses the declared second binding.
    second_binding: bool,
    /// Explicit input packet variation.
    variant: PacketVariant,
}

/// Packet variations are explicit inputs, with no product-derived defaults.
#[derive(Clone, Copy)]
enum PacketVariant {
    /// Complete ordinary time header.
    Ordinary,
    /// Explicit false peer-transmit timestamp.
    PeerFault,
    /// Truncated forty-seven-byte input.
    Malformed47,
    /// Explicit origin bits two against armed token one.
    WrongOrigin,
    /// Stratum-zero opaque control with ignored peer timestamps.
    Control,
}

/// Make the stated symmetric physical path; capture time is checked separately.
fn symmetric(send: u64, token: u64) -> Reply {
    Reply {
        send,
        forward: 2 * SIXTEENTH,
        processing: SIXTEENTH,
        reverse: 2 * SIXTEENTH,
        token,
        second_binding: false,
        variant: PacketVariant::Ordinary,
    }
}

/// Checked immutable capture plus typed coordinates, reusable unchanged at receive.
#[derive(Clone, Copy)]
struct CheckedCapture {
    /// Complete immutable raw capture projection.
    raw: Capture,
    /// Checked saved local coordinate for infallible receipt construction.
    local_t4: UtcCoordinateQ32,
    /// Typed immutable captured mapping generation.
    mapping: ClockGeneration,
}

/// Encode complete wire bytes from independent physical timestamp inputs.
/// Caller supplies a prevalidated clock at the capture instant; failures have no
/// effects, no partial packet publication and no D calls.
/// # Errors
/// Rejects inconsistent arrival, inexact UTC, invalid domains or wire overflow.
fn capture(reply: Reply, at: u64, clock: FixtureClock) -> Result<CheckedCapture, HarnessError> {
    let peer_receive_at = reply
        .send
        .checked_add(reply.forward)
        .ok_or(HarnessError::Arithmetic)?;
    let peer_transmit_at = peer_receive_at
        .checked_add(reply.processing)
        .ok_or(HarnessError::Arithmetic)?;
    let arrival = peer_transmit_at
        .checked_add(reply.reverse)
        .ok_or(HarnessError::Arithmetic)?;
    if arrival != at {
        return Err(HarnessError::InvalidFixture);
    }
    let clocks = clock.read(at)?;
    let t4 = utc(clocks.local_utc)?;
    let t2 = utc(clock.read(peer_receive_at)?.true_utc)?;
    let t3 = match reply.variant {
        PacketVariant::PeerFault => utc(fraction(9, 16)?)?,
        _ => utc(clock.read(peer_transmit_at)?.true_utc)?,
    };
    let mut packet = [0_u8; 48];
    packet[0] = 0x24;
    packet[1] = 2;
    packet[2] = 6;
    packet[3] = 0xec;
    packet[12..16].copy_from_slice(&0x4750_5300_u32.to_be_bytes());
    packet[16..24].copy_from_slice(&0xdfff_ffff_0000_0000_u64.to_be_bytes());
    let origin = match reply.variant {
        PacketVariant::WrongOrigin => 2,
        _ => reply.token,
    };
    WireToken::new(origin).map_err(|_| HarnessError::Domain)?;
    packet[24..32].copy_from_slice(&origin.to_be_bytes());
    packet[32..40].copy_from_slice(&wire_ticks(t2.ticks())?.to_be_bytes());
    packet[40..48].copy_from_slice(&wire_ticks(t3.ticks())?.to_be_bytes());
    if matches!(reply.variant, PacketVariant::Control) {
        packet[0] = 0xe4;
        packet[1] = 0;
        packet[12..16].copy_from_slice(&0x5241_5445_u32.to_be_bytes());
        packet[32..40].copy_from_slice(&0_u64.to_be_bytes());
        packet[40..48].copy_from_slice(&u64::MAX.to_be_bytes());
    }
    Ok(CheckedCapture {
        raw: Capture {
            packet,
            length: if matches!(reply.variant, PacketVariant::Malformed47) {
                47
            } else {
                48
            },
            binding: binding(reply.second_binding)?,
            mapping_generation: clock.mapping,
            arrival_ns: at,
            local_t4: t4.ticks(),
            clocks,
        },
        local_t4: t4,
        mapping: ClockGeneration::new(clock.mapping),
    })
}

/// Build receipt facts from saved capture plus explicit processing M only.
/// Typed saved coordinates are immutable; current UTC/mapping are never read.
fn receipt(saved: &CheckedCapture, processing: MonotonicNs) -> ReceiptFacts {
    ReceiptFacts::new(
        saved.raw.binding,
        saved.mapping,
        MonotonicNs::new(saved.raw.arrival_ns),
        processing,
        saved.local_t4,
    )
}

/// Raw fixture actions. All fallible construction is completed in preparation.
#[derive(Clone, Copy)]
enum Action {
    /// Construct the sole fresh owner.
    Construct,
    /// Arm on the current independent local mapping.
    Arm {
        /// Explicit nonzero wire token.
        token: u64,
        /// Exclusive absolute monotonic deadline.
        deadline: u64,
    },
    /// Capture without processing.
    Capture {
        /// Fixed packet-slot index.
        slot: usize,
        /// Independent physical path and packet inputs.
        reply: Reply,
    },
    /// Capture then receive at the same explicit instant.
    CaptureReceive {
        /// Fixed packet-slot index, allowing explicit replay-copy replacement.
        slot: usize,
        /// Independent physical path and packet inputs.
        reply: Reply,
    },
    /// Receive an unchanged saved capture.
    Receive {
        /// Fixed packet-slot index.
        slot: usize,
    },
    /// Fault a temporary wire copy, retaining all saved capture time facts.
    ReceiveVariant {
        /// Fixed source capture slot.
        slot: usize,
        /// Explicit raw packet fault.
        variant: PacketVariant,
    },
    /// Trusted explicit cancellation.
    Cancel {
        /// Exact request identity to cancel.
        request: u64,
    },
    /// Trusted explicit expiry using event M.
    Expire,
    /// Attempt the named lifecycle context, independently of packet claims.
    Advance {
        /// Whether to use the declared second binding.
        second: bool,
        /// Explicit configuration generation.
        configuration: u64,
        /// Explicit association generation.
        association: u64,
        /// Explicit clock generation.
        clock: u64,
        /// Trusted fixture mapping change, when the script declares one.
        mapping: Option<u64>,
    },
    /// Change only the local UTC trajectory.
    Step {
        /// Signed exact local UTC discontinuity.
        nanoseconds: i128,
    },
    /// Open a fixture suspension.
    Suspend,
    /// Close a fixture suspension.
    Resume,
    /// Lose a queued reply without receive.
    Drop {
        /// Exact slot to release.
        slot: usize,
    },
    /// Release every held replay/queued packet copy.
    Release,
}

/// Explicit total-order element. No implicit packet priority or timer exists.
#[derive(Clone, Copy)]
struct Event {
    /// Literal absolute monotonic instant.
    at: u64,
    /// Strict tie-order ordinal.
    ordinal: u8,
    /// Explicit fixture/product action.
    action: Action,
}

/// Fixed input table owner; `push` rejects capacity before publishing an event.
struct Script {
    /// Fixed raw event slots, without dynamic storage.
    events: [Option<Event>; EVENT_CAP],
    /// Actual populated raw records.
    len: usize,
}

impl Script {
    /// Create an empty fixed table; no product owner or dynamic storage exists.
    fn new() -> Self {
        Self {
            events: [None; EVENT_CAP],
            len: 0,
        }
    }

    /// Append one explicitly ordered event; reject overflow before mutation.
    /// # Errors
    /// Rejects table overflow before adding an event; prior entries survive.
    fn push(&mut self, at: u64, ordinal: u8, action: Action) -> Result<(), HarnessError> {
        let slot = self
            .events
            .get_mut(self.len)
            .ok_or(HarnessError::EventCapacity)?;
        *slot = Some(Event {
            at,
            ordinal,
            action,
        });
        self.len += 1;
        Ok(())
    }

    /// Append capture/process, explicit replay and explicit release in tie order.
    /// # Errors
    /// Propagates fixed-table overflow; callers discard the partial input table.
    fn complete(&mut self, at: u64, slot: usize, reply: Reply) -> Result<(), HarnessError> {
        self.push(at, 1, Action::CaptureReceive { slot, reply })?;
        self.push(at, 2, Action::Receive { slot })?;
        self.push(at, 3, Action::Release)
    }
}

/// Make the reset script's packet variation on an immutable capture copy only.
/// Arrival, T4, mapping and binding are unchanged; every raw byte change is an
/// explicit script input, checked before execution and retained in observation.
/// # Errors
/// Rejects unsupported packet variants or invalid explicit origin construction.
fn vary_capture(
    mut saved: CheckedCapture,
    variant: PacketVariant,
) -> Result<CheckedCapture, HarnessError> {
    match variant {
        PacketVariant::Malformed47 => saved.raw.length = 47,
        PacketVariant::WrongOrigin => {
            let origin = 2_u64;
            WireToken::new(origin).map_err(|_| HarnessError::Domain)?;
            saved.raw.packet[24..32].copy_from_slice(&origin.to_be_bytes());
        }
        PacketVariant::Ordinary => {}
        _ => return Err(HarnessError::InvalidFixture),
    }
    Ok(saved)
}

/// Thirteen separately charged reset events: nine D operations, two captures,
/// and two explicit releases. Fault receive copies do not refresh the capture.
/// # Errors
/// Propagates fixed-table overflow; no partial script is executed.
fn reset_script() -> Result<Script, HarnessError> {
    let mut result = Script::new();
    result.push(START, 1, Action::Construct)?;
    result.push(
        START,
        2,
        Action::Arm {
            token: 1,
            deadline: 2 * SECOND,
        },
    )?;
    result.push(
        21 * SIXTEENTH,
        1,
        Action::Capture {
            slot: 0,
            reply: symmetric(START, 1),
        },
    )?;
    result.push(
        21 * SIXTEENTH,
        2,
        Action::ReceiveVariant {
            slot: 0,
            variant: PacketVariant::Malformed47,
        },
    )?;
    result.push(
        21 * SIXTEENTH,
        3,
        Action::ReceiveVariant {
            slot: 0,
            variant: PacketVariant::WrongOrigin,
        },
    )?;
    result.push(21 * SIXTEENTH, 4, Action::Receive { slot: 0 })?;
    result.push(21 * SIXTEENTH, 5, Action::Receive { slot: 0 })?;
    result.push(21 * SIXTEENTH, 6, Action::Release)?;
    result.push(
        3 * SECOND,
        1,
        Action::Arm {
            token: 2,
            deadline: 4 * SECOND,
        },
    )?;
    result.push(
        53 * SIXTEENTH,
        1,
        Action::Capture {
            slot: 0,
            reply: symmetric(3 * SECOND, 2),
        },
    )?;
    result.push(53 * SIXTEENTH, 2, Action::Receive { slot: 0 })?;
    result.push(53 * SIXTEENTH, 3, Action::Receive { slot: 0 })?;
    result.push(53 * SIXTEENTH, 4, Action::Release)?;
    Ok(result)
}

/// Arithmetic paths and explicit oscillator/peer-fault inputs.
/// # Errors
/// Rejects a wrong family or fixed-table overflow; callers discard partial tables.
fn script_arithmetic(script: &mut Script, case: CaseId) -> Result<(), HarnessError> {
    if !matches!(
        case,
        CaseId::R01 | CaseId::R02 | CaseId::R03 | CaseId::R04 | CaseId::R05
    ) {
        return Err(HarnessError::InvalidFixture);
    }
    let mut reply = symmetric(START, 1);
    match case {
        CaseId::R02 => {
            reply.forward = SIXTEENTH;
            reply.reverse = 3 * SIXTEENTH;
        }
        CaseId::R03 => {
            reply.forward = 3 * SIXTEENTH;
            reply.reverse = SIXTEENTH;
        }
        CaseId::R05 => reply.variant = PacketVariant::PeerFault,
        _ => {}
    }
    script.complete(21 * SIXTEENTH, 0, reply)?;
    Ok(())
}

/// Dropped reply, monotonic expiry and fresh recovery inputs.
/// # Errors
/// Rejects fixed-table overflow; callers discard the partial input table.
fn script_loss(script: &mut Script) -> Result<(), HarnessError> {
    script.push(
        21 * SIXTEENTH,
        1,
        Action::Capture {
            slot: 0,
            reply: symmetric(START, 1),
        },
    )?;
    script.push(21 * SIXTEENTH, 2, Action::Drop { slot: 0 })?;
    script.push(2 * SECOND - 1, 1, Action::Expire)?;
    script.push(2 * SECOND, 1, Action::Expire)?;
    script.push(
        3 * SECOND,
        1,
        Action::Arm {
            token: 2,
            deadline: 4 * SECOND,
        },
    )?;
    script.complete(53 * SIXTEENTH, 0, symmetric(3 * SECOND, 2))?;
    Ok(())
}

/// Explicit trusted replacement events for R07/R11/R12/R15/R16.
/// # Errors
/// Rejects a different case or fixed-table overflow; prior table entries survive.
fn script_replacement(script: &mut Script, case: CaseId) -> Result<(), HarnessError> {
    match case {
        CaseId::R07 => script.push(24 * SIXTEENTH, 1, Action::Cancel { request: 1 })?,
        CaseId::R11 => script.push(
            24 * SIXTEENTH,
            1,
            Action::Advance {
                second: false,
                configuration: 2,
                association: 1,
                clock: 1,
                mapping: None,
            },
        )?,
        CaseId::R12 => script.push(
            24 * SIXTEENTH,
            1,
            Action::Advance {
                second: true,
                configuration: 1,
                association: 2,
                clock: 1,
                mapping: None,
            },
        )?,
        CaseId::R15 | CaseId::R16 => {
            script.push(
                24 * SIXTEENTH,
                1,
                Action::Step {
                    nanoseconds: if case == CaseId::R15 {
                        500_000_000
                    } else {
                        -500_000_000
                    },
                },
            )?;
            script.push(
                24 * SIXTEENTH,
                2,
                Action::Advance {
                    second: false,
                    configuration: 1,
                    association: 1,
                    clock: 2,
                    mapping: Some(2),
                },
            )?;
        }
        _ => return Err(HarnessError::InvalidFixture),
    }
    Ok(())
}

/// Queued old reply, explicit replacement and fresh recovery inputs.
/// # Errors
/// Rejects a wrong family or fixed-table overflow; callers discard partial tables.
fn script_reorder(script: &mut Script, case: CaseId) -> Result<(), HarnessError> {
    if !matches!(
        case,
        CaseId::R07 | CaseId::R11 | CaseId::R12 | CaseId::R15 | CaseId::R16
    ) {
        return Err(HarnessError::InvalidFixture);
    }
    script.push(
        21 * SIXTEENTH,
        1,
        Action::Capture {
            slot: 0,
            reply: symmetric(START, 1),
        },
    )?;
    script_replacement(script, case)?;
    let ordinal = if matches!(case, CaseId::R15 | CaseId::R16) {
        3
    } else {
        2
    };
    script.push(
        24 * SIXTEENTH,
        ordinal,
        Action::Arm {
            token: 2,
            deadline: 40 * SIXTEENTH,
        },
    )?;
    script.push(25 * SIXTEENTH, 1, Action::Receive { slot: 0 })?;
    let mut reply = symmetric(24 * SIXTEENTH, 2);
    reply.second_binding = case == CaseId::R12;
    script.push(29 * SIXTEENTH, 1, Action::CaptureReceive { slot: 1, reply })?;
    script.push(
        29 * SIXTEENTH,
        2,
        Action::Receive {
            slot: usize::from(case != CaseId::R07),
        },
    )?;
    script.push(29 * SIXTEENTH, 3, Action::Release)?;
    Ok(())
}

/// Exclusive monotonic deadline neighbors with saved dyadic receipt.
/// # Errors
/// Rejects a wrong family or fixed-table overflow; callers discard partial tables.
fn script_deadline(script: &mut Script, case: CaseId) -> Result<(), HarnessError> {
    if !matches!(case, CaseId::R08 | CaseId::R09 | CaseId::R10) {
        return Err(HarnessError::InvalidFixture);
    }
    script.push(
        21 * SIXTEENTH,
        1,
        Action::Capture {
            slot: 0,
            reply: symmetric(START, 1),
        },
    )?;
    let processing = match case {
        CaseId::R08 => 2 * SECOND - 1,
        CaseId::R09 => 2 * SECOND,
        _ => 2 * SECOND + 1,
    };
    script.push(processing, 1, Action::Receive { slot: 0 })?;
    if case == CaseId::R08 {
        script.push(2 * SECOND, 1, Action::Receive { slot: 0 })?;
        script.push(2 * SECOND, 2, Action::Release)?;
    } else {
        script.push(processing, 2, Action::Expire)?;
        if case == CaseId::R09 {
            script.push(processing, 3, Action::Receive { slot: 0 })?;
        }
        script.push(processing, 4, Action::Release)?;
    }
    Ok(())
}

/// Rejected context-update inputs with original-request recovery.
/// # Errors
/// Rejects a wrong family or fixed-table overflow; callers discard partial tables.
fn script_rejected_update(script: &mut Script, case: CaseId) -> Result<(), HarnessError> {
    if !matches!(case, CaseId::R13 | CaseId::R14) {
        return Err(HarnessError::InvalidFixture);
    }
    script.push(
        21 * SIXTEENTH,
        1,
        Action::Capture {
            slot: 0,
            reply: symmetric(START, 1),
        },
    )?;
    script.push(
        24 * SIXTEENTH,
        1,
        Action::Advance {
            second: case == CaseId::R14,
            configuration: if case == CaseId::R13 { 0 } else { 2 },
            association: 1,
            clock: 1,
            mapping: None,
        },
    )?;
    script.push(25 * SIXTEENTH, 1, Action::Receive { slot: 0 })?;
    script.push(25 * SIXTEENTH, 2, Action::Receive { slot: 0 })?;
    script.push(25 * SIXTEENTH, 3, Action::Release)?;
    Ok(())
}

/// Pre-suspend capture, distinct resume retirement order and recovery.
/// # Errors
/// Rejects a wrong family or fixed-table overflow; callers discard partial tables.
fn script_suspend(script: &mut Script, case: CaseId) -> Result<(), HarnessError> {
    if !matches!(case, CaseId::R17 | CaseId::R18) {
        return Err(HarnessError::InvalidFixture);
    }
    let mut reply = symmetric(START, 1);
    reply.forward = SIXTEENTH / 2;
    reply.processing = 0;
    reply.reverse = SIXTEENTH / 2;
    script.push(17 * SIXTEENTH, 1, Action::Capture { slot: 0, reply })?;
    script.push(17 * SIXTEENTH, 2, Action::Suspend)?;
    script.push(25 * SIXTEENTH, 0, Action::Resume)?;
    if case == CaseId::R18 {
        script.push(25 * SIXTEENTH, 1, Action::Expire)?;
    }
    script.push(
        25 * SIXTEENTH,
        if case == CaseId::R17 { 1 } else { 2 },
        Action::Advance {
            second: false,
            configuration: 1,
            association: 1,
            clock: 2,
            mapping: Some(2),
        },
    )?;
    if case == CaseId::R18 {
        script.push(25 * SIXTEENTH, 3, Action::Receive { slot: 0 })?;
    }
    script.push(
        26 * SIXTEENTH,
        1,
        Action::Arm {
            token: 2,
            deadline: 42 * SIXTEENTH,
        },
    )?;
    if case == CaseId::R17 {
        script.push(27 * SIXTEENTH, 1, Action::Receive { slot: 0 })?;
    }
    script.complete(31 * SIXTEENTH, 1, symmetric(26 * SIXTEENTH, 2))?;
    Ok(())
}

/// Complete hostile, healthy, opaque-control and recovery inputs.
/// # Errors
/// Rejects fixed-table overflow; callers discard the partial input table.
fn script_hostile(script: &mut Script) -> Result<(), HarnessError> {
    for (ordinal, variant) in [
        (1, PacketVariant::Malformed47),
        (2, PacketVariant::WrongOrigin),
        (3, PacketVariant::Ordinary),
    ] {
        let mut reply = symmetric(START, 1);
        reply.variant = variant;
        script.push(
            21 * SIXTEENTH,
            ordinal,
            Action::CaptureReceive { slot: 0, reply },
        )?;
    }
    script.push(21 * SIXTEENTH, 4, Action::Receive { slot: 0 })?;
    script.push(21 * SIXTEENTH, 5, Action::Release)?;
    script.push(
        2 * SECOND,
        1,
        Action::Arm {
            token: 2,
            deadline: 3 * SECOND,
        },
    )?;
    let mut control = symmetric(2 * SECOND, 2);
    control.variant = PacketVariant::Control;
    script.complete(37 * SIXTEENTH, 0, control)?;
    script.push(
        3 * SECOND,
        1,
        Action::Arm {
            token: 3,
            deadline: 4 * SECOND,
        },
    )?;
    script.complete(53 * SIXTEENTH, 0, symmetric(3 * SECOND, 3))?;
    Ok(())
}

/// Select the reviewed finite input family after explicit owner/send events.
/// No completed facts or arithmetic expectations are authored here.
/// # Errors
/// Rejects a wrong helper family or fixed-table overflow before execution.
fn script(case: CaseId) -> Result<Script, HarnessError> {
    let mut script = Script::new();
    script.push(START, 1, Action::Construct)?;
    let deadline = if case == CaseId::R18 {
        24 * SIXTEENTH
    } else {
        2 * SECOND
    };
    script.push(START, 2, Action::Arm { token: 1, deadline })?;
    match case {
        CaseId::R01 | CaseId::R02 | CaseId::R03 | CaseId::R04 | CaseId::R05 => {
            script_arithmetic(&mut script, case)?;
        }
        CaseId::R06 => script_loss(&mut script)?,
        CaseId::R07 | CaseId::R11 | CaseId::R12 | CaseId::R15 | CaseId::R16 => {
            script_reorder(&mut script, case)?;
        }
        CaseId::R08 | CaseId::R09 | CaseId::R10 => script_deadline(&mut script, case)?,
        CaseId::R13 | CaseId::R14 => script_rejected_update(&mut script, case)?,
        CaseId::R17 | CaseId::R18 => script_suspend(&mut script, case)?,
        CaseId::R19 => script_hostile(&mut script)?,
    }
    Ok(script)
}

/// Domain-admitted actions; packet slots are still changed only by dispatch.
#[derive(Clone, Copy)]
enum PreparedAction {
    /// Fully checked fresh-owner inputs.
    Construct {
        /// Initial fixture binding and generations.
        context: AssociationContext,
        /// Explicit checked initial request identity.
        first: RequestId,
    },
    /// Fully checked request inputs, without a product outcome.
    Arm {
        /// Exact separately saved local send coordinate.
        t1: UtcCoordinateQ32,
        /// Checked explicit correlation bits.
        token: WireToken,
        /// Checked positive bounded monotonic lifetime.
        timing: RequestTiming,
        /// Explicit checked peer-era windows.
        eras: PeerEraContext,
    },
    /// Immutable prepared capture to install at dispatch.
    Capture {
        /// Fixed target packet slot.
        slot: usize,
        /// Fully admitted capture bytes/time facts.
        saved: CheckedCapture,
        /// Whether this event explicitly also calls receive.
        receive: bool,
    },
    /// Receive uses the real dispatched packet slot.
    Receive {
        /// Fixed packet-slot index.
        slot: usize,
    },
    /// Temporary fault packet without a new capture or mapping.
    ReceiveVariant {
        /// Fully admitted varied copy of saved capture.
        saved: CheckedCapture,
    },
    /// Checked exact cancellation identity.
    Cancel(RequestId),
    /// Explicit expiry using event monotonic instant.
    Expire,
    /// Checked proposed immutable context.
    Advance(AssociationContext),
    /// Explicit fixture-only event, with trajectory already admitted.
    Marker(EventKind),
    /// Explicit packet loss without receive.
    Drop {
        /// Admitted occupied packet slot.
        slot: usize,
    },
    /// Explicit release of every live fixture slot.
    Release,
}

/// Immutable checked event, carrying fixture readings but no expected result.
#[derive(Clone, Copy)]
struct PreparedEvent {
    /// Typed exact processing/event monotonic instant.
    at: MonotonicNs,
    /// Admitted strict tie ordinal.
    ordinal: u8,
    /// Admitted independent post-event fixture trajectories.
    clocks: ClockReading,
    /// Fully checked fixture/product input action.
    action: PreparedAction,
}

/// Private admission owner: no unchecked prepared table can reach execution.
struct PreparedScript {
    /// Private immutable admitted input records.
    events: [Option<PreparedEvent>; EVENT_CAP],
    /// Actual populated trace records.
    len: usize,
    /// Expanded event charge, counting capture and receive separately.
    charged: usize,
}

/// Read a checked queue slot without defaulting missing captures.
/// # Errors
/// Rejects absent or out-of-range captures instead of supplying a default.
fn queued(
    queue: &[Option<CheckedCapture>; 2],
    slot: usize,
) -> Result<CheckedCapture, HarnessError> {
    queue
        .get(slot)
        .copied()
        .flatten()
        .ok_or(HarnessError::MissingCapture)
}

/// Replace one bounded slot explicitly, rejecting out-of-range indices first.
/// A combined capture/receive may replace its previous replay copy; plain queued
/// captures require an empty slot, so no unobserved packet is overwritten.
/// # Errors
/// Rejects out-of-range slots or forbidden replacement before changing the queue.
fn store(
    queue: &mut [Option<CheckedCapture>; 2],
    slot: usize,
    saved: &CheckedCapture,
    replace: bool,
) -> Result<(), HarnessError> {
    let destination = queue.get_mut(slot).ok_or(HarnessError::QueueCapacity)?;
    if destination.is_some() && !replace {
        return Err(HarnessError::QueueCapacity);
    }
    *destination = Some(*saved);
    Ok(())
}

/// Count actual held packet slots; this getter cannot release or receive work.
fn queue_len(queue: &[Option<CheckedCapture>; 2]) -> usize {
    queue.iter().filter(|slot| slot.is_some()).count()
}

/// Admit an explicit fixture-only clock/packet lifecycle event.
/// # Errors
/// Rejects unsupported actions, illegal pause ordering, overflow or missing slots.
/// Partial fixture state is private and discarded with the whole rejected table.
fn prepare_fixture_action(
    action: &Action,
    clock: &mut FixtureClock,
    queue: &mut [Option<CheckedCapture>; 2],
    at: u64,
) -> Result<PreparedAction, HarnessError> {
    let prepared = match *action {
        Action::Step { nanoseconds } => {
            clock.local_step_ns = clock
                .local_step_ns
                .checked_add(nanoseconds)
                .ok_or(HarnessError::Arithmetic)?;
            PreparedAction::Marker(EventKind::Step)
        }
        Action::Suspend => {
            clock.suspend(at)?;
            PreparedAction::Marker(EventKind::Suspend)
        }
        Action::Resume => {
            clock.resume(at)?;
            PreparedAction::Marker(EventKind::Resume)
        }
        Action::Drop { slot } => {
            queued(queue, slot)?;
            queue[slot] = None;
            PreparedAction::Drop { slot }
        }
        Action::Release => {
            *queue = [None; 2];
            PreparedAction::Release
        }
        _ => return Err(HarnessError::InvalidFixture),
    };
    Ok(prepared)
}

/// Admit one raw event using checked domain construction and private fixture state.
/// Only the whole-table owner may publish these prepared events to execution.
/// # Errors
/// Rejects invalid owner order, checked input, clock segments or packet slots.
/// Failure discards the private partial table before any Association action.
fn prepare_action(
    event: &Event,
    clock: &mut FixtureClock,
    queue: &mut [Option<CheckedCapture>; 2],
    constructed: &mut bool,
    index: usize,
) -> Result<PreparedAction, HarnessError> {
    let action = match event.action {
        Action::Construct => {
            if *constructed || index != 0 {
                return Err(HarnessError::InvalidFixture);
            }
            *constructed = true;
            PreparedAction::Construct {
                context: context(false, 1, 1, 1)?,
                first: RequestId::new(nonzero(1)?),
            }
        }
        _ if !*constructed => return Err(HarnessError::MissingOwner),
        Action::Arm { token, deadline } => PreparedAction::Arm {
            t1: utc(clock.read(event.at)?.local_utc)?,
            token: WireToken::new(token).map_err(|_| HarnessError::Domain)?,
            timing: RequestTiming::new(MonotonicNs::new(event.at), MonotonicNs::new(deadline))
                .map_err(|_| HarnessError::Domain)?,
            eras: peer_eras()?,
        },
        Action::Capture { slot, reply } | Action::CaptureReceive { slot, reply } => {
            let receive = matches!(event.action, Action::CaptureReceive { .. });
            let saved = capture(reply, event.at, *clock)?;
            store(queue, slot, &saved, receive)?;
            PreparedAction::Capture {
                slot,
                saved,
                receive,
            }
        }
        Action::Receive { slot } => {
            queued(queue, slot)?;
            PreparedAction::Receive { slot }
        }
        Action::ReceiveVariant { slot, variant } => {
            let saved = vary_capture(queued(queue, slot)?, variant)?;
            PreparedAction::ReceiveVariant { saved }
        }
        Action::Cancel { request } => PreparedAction::Cancel(RequestId::new(nonzero(request)?)),
        Action::Expire => PreparedAction::Expire,
        Action::Advance {
            second,
            configuration,
            association,
            clock: generation,
            mapping,
        } => {
            if let Some(mapping) = mapping {
                clock.mapping = mapping;
            }
            PreparedAction::Advance(context(second, configuration, association, generation)?)
        }
        Action::Step { .. }
        | Action::Suspend
        | Action::Resume
        | Action::Drop { .. }
        | Action::Release => prepare_fixture_action(&event.action, clock, queue, event.at)?,
    };
    Ok(action)
}

/// Admit the complete raw table before any Association owner exists.
/// Only this function constructs the private immutable execution plan.
/// # Errors
/// Rejects invalid order, resource caps, clock/byte/domain inputs or owner structure.
fn prepare(run_id: RunId, input: &Script) -> Result<PreparedScript, HarnessError> {
    let compound = input
        .events
        .iter()
        .take(input.len)
        .filter(|event| {
            matches!(
                event,
                Some(Event {
                    action: Action::CaptureReceive { .. },
                    ..
                })
            )
        })
        .count();
    let charged = input
        .len
        .checked_add(compound)
        .ok_or(HarnessError::EventCapacity)?;
    let charged_cap = if run_id == RunId::ResetRecovery {
        16
    } else {
        EVENT_CAP
    };
    if charged > charged_cap {
        return Err(HarnessError::EventCapacity);
    }
    let mut result = PreparedScript {
        events: [None; EVENT_CAP],
        len: input.len,
        charged,
    };
    let mut clock = FixtureClock::new(run_id == RunId::Baseline(CaseId::R04));
    let mut queue = [None; 2];
    let mut previous = None;
    let mut constructed = false;
    for (index, event) in input.events.iter().take(input.len).enumerate() {
        let event = event.ok_or(HarnessError::InvalidFixture)?;
        if let Some(before) = previous {
            ordered(before, (event.at, event.ordinal))?;
        }
        previous = Some((event.at, event.ordinal));
        let horizon = event.at.checked_sub(START).ok_or(HarnessError::Horizon)?;
        validate_input(ValidationInput {
            run_count: ALL_CASES.len() + usize::from(run_id == RunId::Composition),
            event_count: charged,
            queued_count: queue_len(&queue),
            horizon_ns: horizon,
            trace_count: input.len,
            utc_numerator: 0,
            utc_denominator: 1,
        })?;
        let action = prepare_action(&event, &mut clock, &mut queue, &mut constructed, index)?;
        result.events[index] = Some(PreparedEvent {
            at: MonotonicNs::new(event.at),
            ordinal: event.ordinal,
            clocks: clock.read(event.at)?,
            action,
        });
    }
    if !constructed || clock.suspended_at.is_some() {
        return Err(HarnessError::ClockSegment);
    }
    Ok(result)
}

/// Challenge the actual private-plan admission path with malformed neighbors.
/// The returned prepared plan is discarded; no execution function is called.
/// # Errors
/// Returns the actual malformed raw-script admission error, without execution.
fn invalid_script_control(segment: bool) -> Result<(), HarnessError> {
    let mut input = script(CaseId::R01)?;
    let event = input.events[2]
        .as_mut()
        .ok_or(HarnessError::InvalidFixture)?;
    if segment {
        event.action = Action::Resume;
    } else {
        event.at = START;
        event.ordinal = 1;
    }
    prepare(RunId::Baseline(CaseId::R01), &input).map(|_| ())
}

/// Project public getters without observing or changing private pending contents.
fn snapshot(owner: &Association) -> StateSnapshot {
    StateSnapshot {
        context: owner.current_context(),
        pending: owner.pending_request_id().map(|id| id.value().get()),
        next_request: owner.next_request_id().map(|id| id.value().get()),
    }
}

/// Copy all actual completion getters; no fixture data supplies a completed fact.
fn facts(facts: &CompletionFacts) -> FactsProjection {
    FactsProjection {
        binding: facts.binding(),
        request: facts.request_id().value().get(),
        generations: facts.generations(),
        mapping_generation: facts.mapping_generation().value(),
        send_ns: facts.send().nanoseconds(),
        arrival_ns: facts.arrival().nanoseconds(),
        processing_ns: facts.processing_now().nanoseconds(),
        deadline_ns: facts.deadline().nanoseconds(),
    }
}

/// Consume a real D event and copy every available typed fact/quality getter.
/// This contains no formula, normalization, eligibility decision or expected value.
fn completed(event: &FixtureEvent) -> Completed {
    match event {
        FixtureEvent::Time(time) => Completed::Time(TimeProjection {
            facts: facts(time.facts()),
            t1: time.saved_real_t1().ticks(),
            t2: time.peer_t2().ticks(),
            t3: time.peer_t3().ticks(),
            t4: time.local_t4().ticks(),
            offset: time.offset().numerator(),
            rtt: time.signed_round_trip().numerator(),
            leap: time.leap_indicator(),
            stratum: time.stratum(),
            poll: time.poll(),
            precision: time.precision(),
            root_delay: time.root_delay_bits(),
            root_dispersion: time.root_dispersion_bits(),
            reference_id: time.reference_id(),
            reference_timestamp: time.reference_timestamp_bits(),
        }),
        FixtureEvent::Control(control) => Completed::Control(ControlProjection {
            facts: facts(control.facts()),
            code: control.code_bits(),
        }),
    }
}

/// One actual dispatch result; capture storage is immutable projected evidence.
struct DispatchResult {
    /// Actual dispatched event kind.
    kind: EventKind,
    /// Actual public return value.
    outcome: Outcome,
    /// Whether this dispatch called one Association operation.
    product_action: bool,
    /// Actual packet bytes/facts touched by this dispatch.
    captured: Option<Capture>,
}

/// Receive complete saved bytes using only explicit processing M.
/// # Errors
/// Rejects absent fixture owner; all product rejections stay inside the result.
fn receive_captured(
    owner: &mut Option<Association>,
    saved: &CheckedCapture,
    at: MonotonicNs,
    composition: Option<&mut Composition>,
) -> Result<Result<Completed, ReceiveError>, HarnessError> {
    let event = owner
        .as_mut()
        .ok_or(HarnessError::MissingOwner)?
        .receive(&saved.raw.packet[..saved.raw.length], receipt(saved, at));
    match event {
        Ok(event) => {
            let projection = completed(&event);
            if let (Some(consumer), FixtureEvent::Time(time)) = (composition, event) {
                consumer.deliver(time)?;
            }
            Ok(Ok(projection))
        }
        Err(error) => Ok(Err(error)),
    }
}

/// Execute one admitted Association arm/cancel/expire/context action.
/// # Errors
/// Rejects a different prepared action; actual product errors remain in Outcome.
fn dispatch_lifecycle(
    action: &PreparedAction,
    owner: &mut Association,
    at: MonotonicNs,
) -> Result<(EventKind, Outcome), HarnessError> {
    let result = match *action {
        PreparedAction::Arm {
            t1,
            token,
            timing,
            eras,
        } => {
            let result = owner
                .arm(t1, token, timing, eras)
                .map(|id| id.value().get());
            (EventKind::Arm, Outcome::Armed(result))
        }
        PreparedAction::Cancel(request) => {
            let result = owner.cancel(request).map(|id| id.value().get());
            (EventKind::Cancel, Outcome::Cancelled(result))
        }
        PreparedAction::Expire => {
            let result = owner.expire(at).map(|id| id.value().get());
            (EventKind::Expire, Outcome::Expired(result))
        }
        PreparedAction::Advance(next) => {
            let result = owner
                .advance_context(next)
                .map(|retired| retired.map(|id| id.value().get()));
            (EventKind::AdvanceContext, Outcome::Advanced(result))
        }
        _ => return Err(HarnessError::InvalidFixture),
    };
    Ok(result)
}

/// Dispatch one admitted event, preserving the actual return and capture inputs.
/// Queue/marker events cannot call Association or silently retire pending work.
/// # Errors
/// Rejects missing owner/capture or invalid private action/slot custody.
fn dispatch(
    event: &PreparedEvent,
    owner: &mut Option<Association>,
    queue: &mut [Option<CheckedCapture>; 2],
    composition: Option<&mut Composition>,
) -> Result<DispatchResult, HarnessError> {
    let mut captured = None;
    let (kind, outcome, product_action) = match event.action {
        PreparedAction::Construct { context, first } => {
            *owner = Some(Association::new_plain_header_fixture(context, first));
            (EventKind::Construct, Outcome::Constructed, true)
        }
        PreparedAction::Arm { .. }
        | PreparedAction::Cancel(_)
        | PreparedAction::Expire
        | PreparedAction::Advance(_) => {
            let current = owner.as_mut().ok_or(HarnessError::MissingOwner)?;
            let (kind, outcome) = dispatch_lifecycle(&event.action, current, event.at)?;
            (kind, outcome, true)
        }
        PreparedAction::Capture {
            slot,
            saved,
            receive,
        } => {
            store(queue, slot, &saved, receive)?;
            captured = Some(saved.raw);
            if receive {
                let result = receive_captured(owner, &saved, event.at, composition)?;
                (EventKind::CaptureReceive, Outcome::Received(result), true)
            } else {
                (EventKind::Capture, Outcome::None, false)
            }
        }
        PreparedAction::Receive { slot } => {
            let saved = queued(queue, slot)?;
            captured = Some(saved.raw);
            let result = receive_captured(owner, &saved, event.at, composition)?;
            (EventKind::Receive, Outcome::Received(result), true)
        }
        PreparedAction::ReceiveVariant { saved } => {
            captured = Some(saved.raw);
            let result = receive_captured(owner, &saved, event.at, composition)?;
            (EventKind::Receive, Outcome::Received(result), true)
        }

        PreparedAction::Marker(kind) => (kind, Outcome::None, false),
        PreparedAction::Drop { slot } => {
            captured = Some(queued(queue, slot)?.raw);
            queue[slot] = None;
            (EventKind::Drop, Outcome::None, false)
        }
        PreparedAction::Release => {
            *queue = [None; 2];
            (EventKind::Release, Outcome::None, false)
        }
    };
    Ok(DispatchResult {
        kind,
        outcome,
        product_action,
        captured,
    })
}

/// Execute a checked table, retaining actual outcomes and pre-teardown state.
/// Logical trace-array bytes are bounded separately from total stack or RSS.
/// # Errors
/// Rejects invalid private-plan custody, missing captures, or trace-budget mismatch.
fn execute(
    run_id: RunId,
    input: &PreparedScript,
    mut composition: Option<&mut Composition>,
) -> Result<RunReport, HarnessError> {
    let mut owner: Option<Association> = None;
    let mut queue = [None; 2];
    let trace_storage_bytes = size_of::<[Option<Observation>; EVENT_CAP]>();
    if trace_storage_bytes > 32 * 1024 {
        return Err(HarnessError::TraceCapacity);
    }
    #[expect(
        clippy::large_stack_arrays,
        reason = "Fixed 32-slot evidence storage avoids heap retention; its actual logical size is admitted below 32 KiB and independently checked."
    )]
    let mut observations = [None; EVENT_CAP];
    let mut event_count = 0;
    let mut charged_event_count = 0;
    let mut product_action_count = 0;
    for event in input.events.iter().take(input.len) {
        let event = event.ok_or(HarnessError::InvalidFixture)?;
        let before = owner.as_ref().map(snapshot);
        let DispatchResult {
            kind,
            outcome,
            product_action,
            captured,
        } = dispatch(&event, &mut owner, &mut queue, composition.as_deref_mut())?;
        if let Some(consumer) = composition.as_deref_mut() {
            consumer.record(event_count)?;
        }
        if product_action {
            product_action_count += 1;
        }
        charged_event_count += if kind == EventKind::CaptureReceive {
            2
        } else {
            1
        };
        let after = snapshot(owner.as_ref().ok_or(HarnessError::MissingOwner)?);
        let destination = observations
            .get_mut(event_count)
            .ok_or(HarnessError::TraceCapacity)?;
        *destination = Some(Observation {
            kind,
            ordinal: event.ordinal,
            clocks: event.clocks,
            before,
            after,
            outcome,
            capture: captured,
            queue_len: queue_len(&queue),
        });
        event_count += 1;
    }
    let final_state = snapshot(owner.as_ref().ok_or(HarnessError::MissingOwner)?);
    let final_queue_len = queue_len(&queue);
    if charged_event_count != input.charged {
        return Err(HarnessError::InvalidFixture);
    }
    Ok(RunReport {
        run_id,
        observations,
        event_count,
        charged_event_count,
        product_action_count,
        trace_storage_bytes,
        final_state,
        final_queue_len,
    })
}

/// Run one named script from a fresh owner after complete fixture admission.
/// No expectations are consulted; product rejections remain report observations.
/// # Errors
/// Propagates fixture admission or dispatch errors; product rejection stays reported.
pub(super) fn run_case(case: CaseId) -> Result<RunReport, HarnessError> {
    let run_id = RunId::Baseline(case);
    let prepared = prepare(run_id, &script(case)?)?;
    execute(run_id, &prepared, None)
}

/// Fresh fixed fault/healthy/recovery replay: thirteen charged dispatches, with
/// construction, six receives, two arms, two captures and two explicit releases.
/// Caller owns the 256-replay envelope; no hidden loop or extra owner exists here.
/// # Errors
/// Propagates fixture admission or dispatch errors; product rejection stays reported.
pub(super) fn run_reset_recovery() -> Result<RunReport, HarnessError> {
    let run_id = RunId::ResetRecovery;
    let prepared = prepare(run_id, &reset_script()?)?;
    execute(run_id, &prepared, None)
}

/// One fixed 27-record composition plan; all capture and receive work is charged.
/// # Errors
/// Rejects a plan exceeding the fixed event table.
#[expect(
    clippy::too_many_lines,
    reason = "The explicit total-ordered 27-record schedule remains one reviewable fixture."
)]
fn composition_script() -> Result<Script, HarnessError> {
    let q = SIXTEENTH;
    let mut result = Script::new();
    result.push(16 * q, 1, Action::Construct)?;
    result.push(
        16 * q,
        2,
        Action::Arm {
            token: 1,
            deadline: 32 * q,
        },
    )?;
    result.push(
        21 * q,
        1,
        Action::Capture {
            slot: 0,
            reply: symmetric(16 * q, 1),
        },
    )?;
    result.push(
        22 * q,
        1,
        Action::ReceiveVariant {
            slot: 0,
            variant: PacketVariant::Malformed47,
        },
    )?;
    result.push(
        22 * q,
        2,
        Action::ReceiveVariant {
            slot: 0,
            variant: PacketVariant::WrongOrigin,
        },
    )?;
    result.push(22 * q, 3, Action::Receive { slot: 0 })?;
    result.push(22 * q, 4, Action::Release)?;
    result.push(
        32 * q,
        1,
        Action::Arm {
            token: 2,
            deadline: 48 * q,
        },
    )?;
    let mut control = symmetric(32 * q, 2);
    control.variant = PacketVariant::Control;
    result.push(
        37 * q,
        1,
        Action::CaptureReceive {
            slot: 0,
            reply: control,
        },
    )?;
    result.push(37 * q, 2, Action::Release)?;
    result.push(
        48 * q,
        1,
        Action::Arm {
            token: 3,
            deadline: 64 * q,
        },
    )?;
    result.push(
        53 * q,
        1,
        Action::Capture {
            slot: 0,
            reply: symmetric(48 * q, 3),
        },
    )?;
    result.push(53 * q, 2, Action::Drop { slot: 0 })?;
    result.push(64 * q, 1, Action::Expire)?;
    result.push(
        64 * q,
        2,
        Action::Arm {
            token: 4,
            deadline: 80 * q,
        },
    )?;
    result.push(
        69 * q,
        1,
        Action::CaptureReceive {
            slot: 0,
            reply: symmetric(64 * q, 4),
        },
    )?;
    result.push(69 * q, 2, Action::Release)?;
    result.push(
        80 * q,
        1,
        Action::Arm {
            token: 5,
            deadline: 96 * q,
        },
    )?;
    result.push(
        85 * q,
        1,
        Action::CaptureReceive {
            slot: 0,
            reply: symmetric(80 * q, 5),
        },
    )?;
    result.push(85 * q, 2, Action::Release)?;
    result.push(
        85 * q,
        3,
        Action::Arm {
            token: 6,
            deadline: 96 * q,
        },
    )?;
    let mut duplicate = symmetric(85 * q, 6);
    duplicate.forward = 0;
    duplicate.processing = 0;
    duplicate.reverse = 0;
    result.push(
        85 * q,
        4,
        Action::CaptureReceive {
            slot: 0,
            reply: duplicate,
        },
    )?;
    result.push(85 * q, 5, Action::Receive { slot: 0 })?;
    result.push(85 * q, 6, Action::Release)?;
    result.push(
        96 * q,
        1,
        Action::Arm {
            token: 7,
            deadline: 112 * q,
        },
    )?;
    let mut recovery = symmetric(96 * q, 7);
    recovery.forward = q;
    recovery.reverse = 3 * q;
    result.push(
        101 * q,
        1,
        Action::CaptureReceive {
            slot: 0,
            reply: recovery,
        },
    )?;
    result.push(101 * q, 2, Action::Release)?;
    Ok(result)
}

/// Execute only the separately selected, fully prepared composition run.
/// # Errors
/// Preserves fixture preflight, association and bounded trace errors.
pub(super) fn run_composition() -> Result<CompositionReport, HarnessError> {
    let run_id = RunId::Composition;
    let prepared = prepare(run_id, &composition_script()?)?;
    let history_trace_bytes =
        size_of::<[Option<HistorySnapshot>; EVENT_CAP]>() + size_of::<[Option<Delivery>; 5]>();
    let history_owner_bytes = size_of::<Composition>();
    if history_trace_bytes > 32 * 1024 || history_owner_bytes > 64 * 1024 {
        return Err(HarnessError::TraceCapacity);
    }
    let mut composition = Composition::new()?;
    let run = execute(run_id, &prepared, Some(&mut composition))?;
    Ok(CompositionReport {
        run,
        timeline: composition.timeline,
        deliveries: composition.deliveries,
        delivery_count: composition.delivery_count,
        snapshots: composition.snapshots,
        history_trace_bytes,
        history_owner_bytes,
    })
}

/// Seven independently selected expected-identity variations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum GuardRole {
    /// Source identity only.
    Source,
    /// Socket identity only.
    Socket,
    /// Endpoint only.
    Endpoint,
    /// Configuration generation only.
    Configuration,
    /// Association generation only.
    Association,
    /// Clock generation only.
    Clock,
    /// Receipt mapping generation only.
    Mapping,
}

/// The actual association and fixture failure owners remain distinguishable.
#[derive(Debug)]
pub(super) enum GuardRunError {
    /// Fixture setup or evidence admission failed.
    Fixture(HarnessError),
    /// The actual association arm rejected.
    Arm(ArmError),
    /// The actual complete-byte receive rejected.
    Receive(ReceiveError),
    /// The actual association produced control in place of time.
    UnexpectedControl,
}

impl fmt::Display for GuardRunError {
    /// Preserve the actual closed error category and cause.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fixture(error) => write!(formatter, "fixture: {error}"),
            Self::Arm(error) => write!(formatter, "arm: {error}"),
            Self::Receive(error) => write!(formatter, "receive: {error}"),
            Self::UnexpectedControl => formatter.write_str("unexpected control"),
        }
    }
}

impl std::error::Error for GuardRunError {}

/// One real exchange and the separately asserted consumer binding used for it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct GuardStep {
    /// Actual arm identity.
    pub armed: u64,
    /// Actual completed observation projection.
    pub time: TimeProjection,
    /// Actual post-receive association state.
    pub state: StateSnapshot,
    /// Fixture binding/generation assertion tested by the consumer.
    pub expected_context: AssociationContext,
    /// Fixture receipt-mapping assertion tested by the consumer.
    pub expected_mapping: ClockGeneration,
}

/// Exactly one case: three fresh exchanges and three history snapshots.
pub(super) struct GuardReport {
    /// Independently selected role varied once.
    pub role: GuardRole,
    /// Three actual, ordered exchange results.
    pub steps: [GuardStep; 3],
    /// Three actual delivery outcomes and completion identities.
    pub deliveries: [Option<Delivery>; 3],
    /// Post-delivery actual history projections.
    pub snapshots: [Option<HistorySnapshot>; 3],
    /// Duplicate and out-of-range index checks against a one-entry ledger.
    pub ledger_controls: (Result<(), HarnessError>, Result<(), HarnessError>),
}

/// Change exactly one retained assertion; never alter the association or packet.
/// # Errors
/// Rejects the impossible nonzero fixture ID conversion.
fn vary_guard(role: GuardRole, owner: &mut Composition) -> Result<(), HarnessError> {
    let original = owner.context;
    let binding = original.binding();
    let generations = original.generations();
    let two = nonzero(2)?;
    let new_binding = Binding::new(
        if role == GuardRole::Source {
            SourceId::new(two)
        } else {
            binding.source_id()
        },
        if role == GuardRole::Socket {
            SocketId::new(two)
        } else {
            binding.socket_id()
        },
        if role == GuardRole::Endpoint {
            SocketAddr::from(([192, 0, 2, 2], 123))
        } else {
            binding.endpoint()
        },
    );
    let new_generations = Generations::new(
        if role == GuardRole::Configuration {
            ConfigurationGeneration::new(2)
        } else {
            generations.configuration()
        },
        if role == GuardRole::Association {
            AssociationGeneration::new(2)
        } else {
            generations.association()
        },
        if role == GuardRole::Clock {
            ClockGeneration::new(2)
        } else {
            generations.clock()
        },
    );
    owner.context = AssociationContext::new(new_binding, new_generations);
    if role == GuardRole::Mapping {
        owner.mapping = ClockGeneration::new(2);
    }
    Ok(())
}

/// Run one explicit arm, capture, receive and sole typed delivery.
/// # Errors
/// Preserves fixture, arm, receive and unexpected-control failures.
fn guard_step(
    owner: &mut Association,
    consumer: &mut Composition,
    clock: FixtureClock,
    send: u64,
    token: u64,
    index: usize,
) -> Result<GuardStep, GuardRunError> {
    let t1 = utc(clock.read(send).map_err(GuardRunError::Fixture)?.local_utc)
        .map_err(GuardRunError::Fixture)?;
    let deadline = send
        .checked_add(16 * SIXTEENTH)
        .ok_or(GuardRunError::Fixture(HarnessError::Arithmetic))?;
    let timing = RequestTiming::new(MonotonicNs::new(send), MonotonicNs::new(deadline))
        .map_err(|_| GuardRunError::Fixture(HarnessError::Domain))?;
    let wire = WireToken::new(token).map_err(|_| GuardRunError::Fixture(HarnessError::Domain))?;
    let armed = owner
        .arm(
            t1,
            wire,
            timing,
            peer_eras().map_err(GuardRunError::Fixture)?,
        )
        .map_err(GuardRunError::Arm)?;
    let arrival = send
        .checked_add(5 * SIXTEENTH)
        .ok_or(GuardRunError::Fixture(HarnessError::Arithmetic))?;
    let saved = capture(symmetric(send, token), arrival, clock).map_err(GuardRunError::Fixture)?;
    let event = owner
        .receive(
            &saved.raw.packet[..saved.raw.length],
            receipt(&saved, MonotonicNs::new(arrival)),
        )
        .map_err(GuardRunError::Receive)?;
    let projection = completed(&event);
    let FixtureEvent::Time(time) = event else {
        return Err(GuardRunError::UnexpectedControl);
    };
    let Completed::Time(projection) = projection else {
        return Err(GuardRunError::UnexpectedControl);
    };
    let expected_context = consumer.context;
    let expected_mapping = consumer.mapping;
    consumer.deliver(time).map_err(GuardRunError::Fixture)?;
    consumer.record(index).map_err(GuardRunError::Fixture)?;
    Ok(GuardStep {
        armed: armed.value().get(),
        time: projection,
        state: snapshot(owner),
        expected_context,
        expected_mapping,
    })
}

/// Execute one role case: exactly three arms/receives, one owner and no replay.
/// Seven caller-selected cases have a maximum of 21 arms and receives, seven
/// association constructions, 21 completed events and 14 history appends.
/// The mismatch is rejected before its observation constructor; three snapshots
/// per case and two immutable ledger checks are additional bounded observer work.
/// # Errors
/// Propagates the actual failed operation; no recovery or reset is implicit.
pub(super) fn run_guard_case(role: GuardRole) -> Result<GuardReport, GuardRunError> {
    let original = context(false, 1, 1, 1).map_err(GuardRunError::Fixture)?;
    let mut association = Association::new_plain_header_fixture(
        original,
        RequestId::new(nonzero(1).map_err(GuardRunError::Fixture)?),
    );
    let mut consumer = Composition::new().map_err(GuardRunError::Fixture)?;
    let clock = FixtureClock::new(false);
    let first = guard_step(&mut association, &mut consumer, clock, 16 * SIXTEENTH, 1, 0)?;
    let ledger_controls = (
        admit_delivery(&consumer.deliveries, 5, 1),
        admit_delivery(&consumer.deliveries, 5, 2),
    );
    vary_guard(role, &mut consumer).map_err(GuardRunError::Fixture)?;
    let changed = guard_step(&mut association, &mut consumer, clock, 32 * SIXTEENTH, 2, 1)?;
    consumer.context = original;
    consumer.mapping = ClockGeneration::new(1);
    let recovery = guard_step(&mut association, &mut consumer, clock, 48 * SIXTEENTH, 3, 2)?;
    Ok(GuardReport {
        role,
        steps: [first, changed, recovery],
        deliveries: [
            consumer.deliveries[0],
            consumer.deliveries[1],
            consumer.deliveries[2],
        ],
        snapshots: [
            consumer.snapshots[0],
            consumer.snapshots[1],
            consumer.snapshots[2],
        ],
        ledger_controls,
    })
}

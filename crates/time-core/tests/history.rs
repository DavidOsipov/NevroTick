// SPDX-License-Identifier: GPL-2.0-only
//! Independent external S1.2d history consumers; pure CHECK-7/CHECK-25 fixtures.
//!
//! Literal truth: history-transition-facts.json SHA-256
//! ec6a1b96806c6219bd2b74c37b737fde7cbfddafe6cf2f07b12cb14d1d9c92d1;
//! contract53ce5e686cbb88175bd7138302f35c5d02939316615134451ba74c3fee847ea0.
//! No history implementation body or duplicate numerical/state model is an oracle.
//!
//! There are17 actual scopes:15 trace scopes (the coexisting pair shares one),
//! one genuine privacy scope and one zero-product-call manual-error scope.
//! Complete prospective actual external counts in the disjoint history family:
//! histories17 appends144 observations144 contexts21 monotonic144 coordinates576
//! exchanges144 getters32786 normalizations0 stats0 fits0 points0.
//! Each product getter is charged before its actual call, including setup/domain
//! getters and the retained caller `Copy`. Standard-library calls are separately
//! source-audited. One assembled `Drop` `write_all` row survives assertion unwinding.
//!
//! The frozen16 streams execute140 append attempts,160 snapshots,2768 actual
//! prefix-row views and104 actual report views. The two quarter histories remain
//! alive together; four peer reads do not reconstruct either history or replay
//! setup/appends. Privacy adds only the frozen changing-data4 setup, three
//! snapshots and one report borrow: H1/A4/O4/C1/M4/U16/E4/G154.
//! The manual error test makes no value getter/constructor/solver request.
//!
//! History-internal normalization/statistics costs are SOURCE UPPERS, not these
//! observed external counters:134 normalization,95 statistics/inner fits,
//! 2568 duration operations,2382 generated points and97 input constructions.
//! No direct prepare/statistics/fit/point call, new doctest, retry, global counter,
//! ambient time, native authority, age/prune or implementation-derived tolerance.
#![forbid(unsafe_code)]

#[path = "support/history_facts.rs"]
mod history_facts;

use std::{
    cell::Cell,
    error::Error,
    fmt::{self, Debug},
    io::{self, Write},
    num::NonZeroU64,
};

use history_facts::{
    Expected, PAIRED_POSITIVE, ReportFact, RowFact, SnapshotFact, StepFact, TRACES, TraceFact,
};
use time_core::{
    FitError, FixtureContext, FixtureHistory, FixtureHistoryReport, FixtureObservation,
    HistoryError, NormalizationError, ResidualError,
};
use time_domain::{
    ExchangeArithmetic, MonotonicNs, PhaseOffsetQ33, UtcCoordinateQ32, exchange_arithmetic,
};

/// A harness/setup failure stays separate from an actual semantic assertion.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
/// One immutable typed identity already constructed through its public owner.
type ContextEntry = ([u64; 3], FixtureContext);
/// Stable independent assertion marker, also useful for bounded source-fault observers.
const ORACLE: &str = "independent history transition oracle";

/// One actual external-public-call owner per test scope; no parent family is charged.
struct Profile {
    /// Actual history constructor calls.
    histories: Cell<u64>,
    /// Actual append attempts, including rejected ones.
    appends: Cell<u64>,
    /// Actual checked observation constructors.
    observations: Cell<u64>,
    /// Actual asserted-context constructors.
    contexts: Cell<u64>,
    /// Actual exact monotonic wrappers.
    monotonic: Cell<u64>,
    /// Actual checked UTC coordinate constructors.
    coordinates: Cell<u64>,
    /// Actual inert exchange-arithmetic calls.
    exchanges: Cell<u64>,
    /// Every actual external product getter request, including setup/domain values.
    getters: Cell<u64>,
    /// Direct external normalization attempts; none in this file.
    normalizations: Cell<u64>,
    /// Direct external statistics attempts; none in this file.
    stats: Cell<u64>,
    /// Direct external fit attempts; none in this file.
    fits: Cell<u64>,
    /// Direct external point attempts; none in this file.
    points: Cell<u64>,
}

impl Profile {
    /// Start one all-zero local scope; no product operation, failure or external effect.
    const fn new() -> Self {
        Self {
            histories: Cell::new(0),
            appends: Cell::new(0),
            observations: Cell::new(0),
            contexts: Cell::new(0),
            monotonic: Cell::new(0),
            coordinates: Cell::new(0),
            exchanges: Cell::new(0),
            getters: Cell::new(0),
            normalizations: Cell::new(0),
            stats: Cell::new(0),
            fits: Cell::new(0),
            points: Cell::new(0),
        }
    }

    /// Construct three real nonzero tags, then charge before the actual context call.
    /// # Errors
    /// Rejects invalid literal tags or a counter ceiling; no replacement identity.
    /// # Resources
    /// Three standard constructors and exactly one actual product context attempt.
    fn context(&self, raw: [u64; 3]) -> TestResult<FixtureContext> {
        let source = nonzero(raw[0])?;
        let frame = nonzero(raw[1])?;
        let generation = nonzero(raw[2])?;
        charge(&self.contexts, 64)?;
        Ok(FixtureContext::new(source, frame, generation))
    }

    /// Charge immediately before one actual exact monotonic constructor; no clock read.
    /// # Errors
    /// Returns a bounded harness counter error only.
    fn monotonic(&self, value: u64) -> TestResult<MonotonicNs> {
        charge(&self.monotonic, 256)?;
        Ok(MonotonicNs::new(value))
    }

    /// Charge before one checked real UTC constructor for legitimate phase setup.
    /// # Errors
    /// Preserves actual domain or counter rejection; no retagging/raw Q33 constructor.
    fn coordinate(&self, value: i128) -> TestResult<UtcCoordinateQ32> {
        charge(&self.coordinates, 1024)?;
        Ok(UtcCoordinateQ32::new(value)?)
    }

    /// Charge before one actual four-coordinate inert exchange operation.
    /// # Errors
    /// Preserves actual domain or counter failure; constant bounded work.
    fn exchange(&self, values: [UtcCoordinateQ32; 4]) -> TestResult<ExchangeArithmetic> {
        charge(&self.exchanges, 256)?;
        Ok(exchange_arithmetic(
            values[0], values[1], values[2], values[3],
        )?)
    }

    /// Charge immediately before actual scalar admission of one typed observation.
    /// # Errors
    /// Preserves mandatory domain/garde rejection or the harness counter error.
    fn observation(
        &self,
        context: FixtureContext,
        at: MonotonicNs,
        phase: PhaseOffsetQ33,
        divisor: f64,
    ) -> TestResult<FixtureObservation> {
        charge(&self.observations, 256)?;
        Ok(FixtureObservation::new(context, at, phase, divisor)?)
    }

    /// Charge before empty history construction, with no hidden fit/statistics call.
    /// # Errors
    /// Returns only a counter failure; context is already privately admitted.
    fn history(&self, context: FixtureContext) -> TestResult<FixtureHistory> {
        charge(&self.histories, 32)?;
        Ok(FixtureHistory::new(context))
    }

    /// Charge before the actual sole receiving mutation; preserve its typed result.
    /// # Errors
    /// Returns only a counter failure outside the unchanged actual product result.
    /// # Resources
    /// One append request; internal normalization/statistics are not observed here.
    fn append(
        &self,
        history: &mut FixtureHistory,
        candidate: FixtureObservation,
    ) -> TestResult<Result<(), HistoryError>> {
        charge(&self.appends, 160)?;
        Ok(history.try_append(candidate))
    }

    /// Charge BEFORE exactly one external product getter, preserving its borrow/value.
    /// # Errors
    /// Rejects a counter failure before calling the supplied getter.
    /// # Panics
    /// An actual getter contract panic propagates; none is expected for these fixtures.
    /// # Resources
    /// One immediate getter; the closure contains no second product call.
    fn getter<T>(&self, getter: impl FnOnce() -> T) -> TestResult<T> {
        charge(&self.getters, 65_536)?;
        Ok(getter())
    }
}

impl Drop for Profile {
    /// Emit one complete best-effort `write_all` row, including during assertion unwind.
    /// A failed/missing sink remains incomplete supervisor evidence.
    fn drop(&mut self) {
        let record = format!(
            "CORE_HISTORY_PROFILE_COUNTS histories={} appends={} observations={} contexts={} monotonic={} coordinates={} exchanges={} getters={} normalizations={} stats={} fits={} points={}\n",
            self.histories.get(),
            self.appends.get(),
            self.observations.get(),
            self.contexts.get(),
            self.monotonic.get(),
            self.coordinates.get(),
            self.exchanges.get(),
            self.getters.get(),
            self.normalizations.get(),
            self.stats.get(),
            self.fits.get(),
            self.points.get()
        );
        let _emission_result = io::stderr().lock().write_all(record.as_bytes());
    }
}

/// Charge only a real upcoming call; overflow/ceiling failure performs no product call.
/// # Errors
/// Returns a bounded harness error; constant work, no retry or shared state.
fn charge(counter: &Cell<u64>, maximum: u64) -> TestResult {
    let next = counter
        .get()
        .checked_add(1)
        .ok_or_else(|| io::Error::other("history counter overflow"))?;
    if next > maximum {
        return Err(io::Error::other("history counter ceiling").into());
    }
    counter.set(next);
    Ok(())
}

/// Construct an actual standard tag with no unchecked zero fallback.
/// # Errors
/// Rejects a zero literal; constant bounded work.
fn nonzero(value: u64) -> TestResult<NonZeroU64> {
    Ok(NonZeroU64::new(value).ok_or_else(|| io::Error::other("zero history fixture tag"))?)
}

/// Read all three immutable tags; each product getter is independently charged.
/// # Errors
/// Propagates only a getter counter failure; three standard gets are not product calls.
fn context_bits(profile: &Profile, value: FixtureContext) -> TestResult<[u64; 3]> {
    Ok([
        profile.getter(|| value.source_id())?.get(),
        profile.getter(|| value.frame())?.get(),
        profile.getter(|| value.generation())?.get(),
    ])
}

/// Observe the actual raw immutable value with nine product/four standard getters.
/// # Errors
/// Propagates only counter errors; no constructor, conversion or numerical solver.
fn observation_bits(profile: &Profile, value: &FixtureObservation) -> TestResult<RowFact> {
    let context = profile.getter(|| value.context())?;
    let at = profile.getter(|| value.observed_at())?;
    let phase = profile.getter(|| value.phase())?;
    let divisor = profile.getter(|| value.variance_divisor())?;
    Ok(RowFact {
        context: context_bits(profile, context)?,
        observed_ns: profile.getter(|| at.nanoseconds())?,
        phase_q33: profile.getter(|| phase.numerator())?,
        divisor_bits: divisor.to_bits(),
    })
}

/// Obtain exact admitted Q33 via four genuine UTC constructors and existing exchange.
/// # Errors
/// Preserves actual domain/counter failure; no raw phase or integer-to-float model.
/// # Panics
/// The independent exact Q33 numerator must be retained before observation admission.
/// # Resources
/// Four UTC/one exchange attempts, plus offset/numerator getter requests.
fn phase(profile: &Profile, numerator: i128) -> TestResult<PhaseOffsetQ33> {
    let values = [
        profile.coordinate(0)?,
        profile.coordinate(numerator)?,
        profile.coordinate(0)?,
        profile.coordinate(0)?,
    ];
    let exchange = profile.exchange(values)?;
    let phase = profile.getter(|| exchange.offset())?;
    assert_eq!(
        profile.getter(|| phase.numerator())?,
        numerator,
        "{ORACLE}: exact phase setup"
    );
    Ok(phase)
}

/// Reuse a cataloged typed context without additional product calls or validation.
/// # Errors
/// Rejects a missing literal cache identity; bounded search through at most four entries.
fn cached_context(cache: &[ContextEntry], raw: [u64; 3]) -> TestResult<FixtureContext> {
    cache
        .iter()
        .find(|(labels, _)| *labels == raw)
        .map(|(_, value)| *value)
        .ok_or_else(|| io::Error::other("missing history context fact").into())
}

/// Create the actual typed incoming value once, preserving the raw literal recipe.
/// # Errors
/// Propagates actual domain/observation/counter admission failure; no fallback.
/// # Resources
/// One monotonic/four UTC/one exchange/one observation attempt and two setup getters.
fn admit(
    profile: &Profile,
    cache: &[ContextEntry],
    row: RowFact,
) -> TestResult<FixtureObservation> {
    let context = cached_context(cache, row.context)?;
    let at = profile.monotonic(row.observed_ns)?;
    let phase = phase(profile, row.phase_q33)?;
    profile.observation(context, at, phase, f64::from_bits(row.divisor_bits))
}

/// Check complete report association and every literal line/statistic output field.
/// # Errors
/// Propagates counter failures only.
/// # Panics
/// All context/reference/frame/count/dof and five exact scalar bits must match facts.
/// # Resources
/// Seventeen product getters and ten standard gets/bit views; no recomputation.
fn inspect_report(
    profile: &Profile,
    report: &FixtureHistoryReport,
    expected: ReportFact,
) -> TestResult {
    let context = profile.getter(|| report.context())?;
    assert_eq!(
        context_bits(profile, context)?,
        expected.context,
        "{ORACLE}: report context"
    );
    let reference = profile.getter(|| report.reference())?;
    assert_eq!(
        profile.getter(|| reference.nanoseconds())?,
        expected.reference,
        "{ORACLE}: report reference value"
    );
    let statistics = profile.getter(|| report.statistics())?;
    let frame = profile.getter(|| statistics.frame())?;
    assert_eq!(frame.get(), expected.frame, "{ORACLE}: statistics frame");
    assert_eq!(
        profile.getter(|| statistics.sample_count())?,
        expected.sample_count,
        "{ORACLE}: statistics count"
    );
    assert_eq!(
        profile.getter(|| statistics.degrees_of_freedom())?,
        expected.degrees_of_freedom,
        "{ORACLE}: statistics dof"
    );
    let line = profile.getter(|| statistics.line())?;
    assert_eq!(
        profile.getter(|| line.frame())?.get(),
        expected.frame,
        "{ORACLE}: actual line frame"
    );
    let actual = [
        profile.getter(|| line.phase_seconds())?.to_bits(),
        profile
            .getter(|| line.phase_rate_seconds_per_second())?
            .to_bits(),
        profile
            .getter(|| statistics.weighted_sse_seconds_squared())?
            .to_bits(),
        profile
            .getter(|| statistics.residual_scale_seconds_squared())?
            .to_bits(),
        profile
            .getter(|| statistics.normalized_data_variance_seconds_squared())?
            .to_bits(),
    ];
    assert_eq!(actual, expected.bits, "{ORACLE}: same-input numeric report");
    Ok(())
}

/// Inspect every actual stored prefix member and association using independent length.
/// # Errors
/// Propagates counter failures or a missing fixture/product borrow after an assertion.
/// # Panics
/// Compares every raw row/report field; arbitrary indices count/MAX must return None.
/// # Resources
/// One bounded expected-prefix sweep, no constructors or direct solver requests.
fn inspect_snapshot(
    profile: &Profile,
    history: &FixtureHistory,
    expected: &SnapshotFact,
) -> TestResult {
    let context = profile.getter(|| history.context())?;
    assert_eq!(
        context_bits(profile, context)?,
        expected.context,
        "{ORACLE}: immutable history context"
    );
    assert_eq!(
        profile.getter(|| history.sample_count())?,
        expected.sample_count,
        "{ORACLE}: committed count"
    );
    let reference = profile.getter(|| history.reference())?;
    let reference_bits = match reference {
        Some(value) => Some(profile.getter(|| value.nanoseconds())?),
        None => None,
    };
    assert_eq!(
        reference_bits, expected.reference,
        "{ORACLE}: committed reference value"
    );
    let report = profile.getter(|| history.report())?;
    assert_eq!(
        report.is_some(),
        expected.report.is_some(),
        "{ORACLE}: report presence"
    );
    if let Some(expected_report) = expected.report {
        inspect_report(
            profile,
            report.ok_or_else(|| io::Error::other("missing actual history report"))?,
            expected_report,
        )?;
    }
    assert_eq!(
        expected.rows.len(),
        expected.sample_count,
        "{ORACLE}: literal snapshot shape"
    );
    for (index, row) in expected.rows.iter().enumerate() {
        let actual = profile.getter(|| history.observation(index))?;
        assert!(actual.is_some(), "{ORACLE}: missing real prefix row{index}");
        let actual = actual.ok_or_else(|| io::Error::other("missing actual history row"))?;
        assert_eq!(
            observation_bits(profile, actual)?,
            *row,
            "{ORACLE}: actual retained row{index}"
        );
    }
    assert!(
        profile
            .getter(|| history.observation(expected.sample_count))?
            .is_none(),
        "{ORACLE}: prefix end"
    );
    assert!(
        profile
            .getter(|| history.observation(usize::MAX))?
            .is_none(),
        "{ORACLE}: arbitrary index"
    );
    Ok(())
}

/// Require the exact first typed result; no temporary missing behavior is a success.
/// # Panics
/// A wrong error, wrong staged index or rejected legitimate append fails independently.
/// # Resources
/// One bounded match on the returned error, with no further public product request.
fn inspect_outcome(result: &Result<(), HistoryError>, step: &StepFact) {
    let matches = match step.expected {
        Expected::Commit => result.is_ok(),
        Expected::ContextMismatch => matches!(result, Err(HistoryError::ContextMismatch)),
        Expected::DuplicateTime => matches!(result, Err(HistoryError::DuplicateTime)),
        Expected::OutOfOrderTime => matches!(result, Err(HistoryError::OutOfOrderTime)),
        Expected::ElapsedRange { index: expected } => matches!(
            result, Err(HistoryError::Normalization { source: NormalizationError::ElapsedRange { index } }) if *index == expected),
        Expected::SpanTooShort => matches!(
            result,
            Err(HistoryError::Normalization {
                source: NormalizationError::SpanTooShort
            })
        ),
        Expected::InsufficientSpread => matches!(
            result,
            Err(HistoryError::Normalization {
                source: NormalizationError::Input(FitError::InsufficientSpread)
            })
        ),
    };
    assert!(matches, "{ORACLE}: {} first result {result:?}", step.name);
}

/// Enter one actual mutation, check caller `Copy` on either return and inspect whole state.
/// # Errors
/// Propagates real typed setup/admission or bounded counter failure.
/// # Panics
/// The expected first outcome and complete post-state must match independent facts.
/// # Resources
/// One append, one fresh admitted candidate, nine caller-`Copy` getters plus one snapshot.
fn run_step(
    profile: &Profile,
    history: &mut FixtureHistory,
    cache: &[ContextEntry],
    step: &StepFact,
) -> TestResult {
    let candidate = admit(profile, cache, step.candidate)?;
    let retained_copy = candidate;
    let result = profile.append(history, candidate)?;
    assert_eq!(
        observation_bits(profile, &retained_copy)?,
        step.candidate,
        "{ORACLE}: immutable caller input"
    );
    inspect_outcome(&result, step);
    inspect_snapshot(profile, history, step.after)
}

/// Start one actual empty owner and inspect it, without replaying a trace or using defaults.
/// # Errors
/// Rejects an invalid catalog or an actual constructor/counter failure.
/// # Panics
/// Actual immutable constructor labels and empty state must match literal observations.
/// # Resources
/// One history, one context per catalog entry (at most four), and one empty snapshot.
fn start_trace(
    profile: &Profile,
    trace: &TraceFact,
) -> TestResult<(FixtureHistory, Vec<ContextEntry>)> {
    if trace.contexts.is_empty() || trace.contexts.len() > 4 {
        return Err(io::Error::other("history context catalog bound").into());
    }
    let mut cache = Vec::with_capacity(trace.contexts.len());
    for raw in trace.contexts {
        let value = profile.context(*raw)?;
        assert_eq!(
            context_bits(profile, value)?,
            *raw,
            "{ORACLE}: context constructor retains labels"
        );
        cache.push((*raw, value));
    }
    let context = cached_context(&cache, trace.initial.context)?;
    let history = profile.history(context)?;
    inspect_snapshot(profile, &history, trace.initial)?;
    Ok((history, cache))
}

/// Locate the frozen named trace; no program output or implementation supplies facts.
/// # Errors
/// Returns a bounded harness error on a missing transcription.
fn fact(name: &str) -> TestResult<&'static TraceFact> {
    TRACES
        .iter()
        .find(|trace| trace.name == name)
        .ok_or_else(|| io::Error::other("missing independent history trace").into())
}

/// Execute exactly one finite stream in its owning profile, retaining each committed state.
/// # Errors
/// Propagates actual setup/admission/counter errors.
/// # Panics
/// All published states and results must match independent frozen literals.
/// # Resources
/// At most70 attempts, one history and one snapshot after each actual call.
fn run_trace(profile: &Profile, name: &str) -> TestResult {
    let trace = fact(name)?;
    let (mut history, cache) = start_trace(profile, trace)?;
    for step in trace.steps {
        run_step(profile, &mut history, &cache, step)?;
    }
    Ok(())
}

/// Exact constructor labels at legal u64 endpoints; actual zero-count getters remain total.
/// # Errors
/// Propagates actual setup or counter rejection.
/// # Panics
/// The empty prefix/reference/report and out-of-prefix views must match facts.
/// # Resources
/// One local scope; its exact bounded public-call recipe is in the source audit.
#[test]
fn empty_history_retains_immutable_endpoint_labels_and_real_prefix() -> TestResult {
    let profile = Profile::new();
    run_trace(&profile, "empty_immutable_endpoint_labels")
}

/// Context wins before duplicate/older time; all invalid attempts preserve each raw field.
/// # Errors
/// Propagates setup/counter failure.
/// # Panics
/// The full typed first result and retained snapshots differ on any wrong transition.
/// # Resources
/// One local scope; its exact bounded public-call recipe is in the source audit.
#[test]
fn partial_history_guards_preserve_rows_and_first_error_precedence() -> TestResult {
    let profile = Profile::new();
    run_trace(&profile, "partial_guards_and_labels")
}

/// Exact near-u64 origin preserves raw readings and the independent quarter summary.
/// # Errors
/// Propagates setup/counter failure.
/// # Panics
/// The actual newest reference and every line/statistic bit must match literal facts.
/// # Resources
/// One local scope; its exact bounded public-call recipe is in the source audit.
#[test]
fn near_u64_max_origin_has_exact_reference_and_positive_quarter_report() -> TestResult {
    let profile = Profile::new();
    run_trace(&profile, "origin_u64max_positive_quarter")
}

/// A first reading at `u64::MAX` is legal; duplicate/older attempts cannot reset it.
/// # Errors
/// Propagates setup/counter failure.
/// # Panics
/// The exact one-row insufficient state must survive both rejected attempts.
/// # Resources
/// One local scope; its exact bounded public-call recipe is in the source audit.
#[test]
fn first_u64_max_reading_survives_duplicate_and_older_attempts() -> TestResult {
    let profile = Profile::new();
    run_trace(&profile, "u64max_single_then_rejections")
}

/// Below/exact/above sufficient span admission enters the actual receiving history.
/// # Errors
/// Propagates setup/counter failure.
/// # Panics
/// Minimum-span failure preserves the two-row state, then both valid neighbors commit.
/// # Resources
/// One local scope; its exact bounded public-call recipe is in the source audit.
#[test]
fn minimum_span_neighbors_preserve_insufficient_then_commit_reports() -> TestResult {
    let profile = Profile::new();
    run_trace(&profile, "minimum_span_neighbors")
}

/// Maximum-window rejection also applies at count two, before insufficient publication.
/// # Errors
/// Propagates setup/counter failure.
/// # Panics
/// The failed attempt cannot become the ordering reference; below/exact neighbors work.
/// # Resources
/// One local scope; its exact bounded public-call recipe is in the source audit.
#[test]
fn two_row_maximum_window_neighbors_use_committed_reference() -> TestResult {
    let profile = Profile::new();
    run_trace(&profile, "two_row_maximum_neighbors")
}

/// The exact two-row window remains insufficient without pruning or a manufactured report.
/// # Errors
/// Propagates setup/counter failure.
/// # Panics
/// A later over-window append must preserve both original rows and their exact reference.
/// # Resources
/// One local scope; its exact bounded public-call recipe is in the source audit.
#[test]
fn exact_two_row_maximum_does_not_prune_on_later_window_failure() -> TestResult {
    let profile = Profile::new();
    run_trace(&profile, "two_row_exact_maximum_no_prune")
}

/// A failed numerical transaction retains the existing report's original exact reference.
/// # Errors
/// Propagates setup/counter failure.
/// # Panics
/// Window failure preserves every field; a healthy earlier-than-rejected retry succeeds.
/// # Resources
/// One local scope; its exact bounded public-call recipe is in the source audit.
#[test]
fn window_failure_preserves_report_and_healthy_retry_reference() -> TestResult {
    let profile = Profile::new();
    run_trace(&profile, "report_window_failure_then_neighbor")
}

/// Full-buffer context/order/window/span failures retain all64 rows before one valid replacement.
/// # Errors
/// Propagates setup/counter failure.
/// # Panics
/// The actual stored prefix and report must match every frozen state, including oldest-only eviction.
/// # Resources
/// Exactly70 appends; source uppers67 normalization/63 statistics, never observed direct calls.
#[test]
fn full_history_replacement_is_atomic_across_all_selected_failures() -> TestResult {
    let profile = Profile::new();
    run_trace(&profile, "full64_replace_and_failures")
}

/// The published computed-C below neighbor rejects, then a same-state healthy divisor commits.
/// # Errors
/// Propagates setup/counter failure.
/// # Panics
/// The actual nested spread error and complete old-state/new-report facts must match.
/// # Resources
/// One local scope; its exact bounded public-call recipe is in the source audit.
#[test]
fn calculated_spread_below_preserves_state_then_neighbor_commits() -> TestResult {
    let profile = Profile::new();
    run_trace(&profile, "spread_calculated_below")
}

/// The exact computed-C boundary remains an admitted report-producing transaction.
/// # Errors
/// Propagates setup/counter failure.
/// # Panics
/// All actual held raw fields and exact zero-report bits match frozen literals.
/// # Resources
/// One local scope; its exact bounded public-call recipe is in the source audit.
#[test]
fn calculated_spread_exact_boundary_commits_actual_report() -> TestResult {
    let profile = Profile::new();
    run_trace(&profile, "spread_calculated_exact")
}

/// The published computed-C above neighbor remains healthy.
/// # Errors
/// Propagates setup/counter failure.
/// # Panics
/// The actual committed prefix and report match the independent upper neighbor.
/// # Resources
/// One local scope; its exact bounded public-call recipe is in the source audit.
#[test]
fn calculated_spread_above_boundary_commits_actual_report() -> TestResult {
    let profile = Profile::new();
    run_trace(&profile, "spread_calculated_above")
}

/// A calculated-spread failure after17 committed rows preserves the existing paired report.
/// # Errors
/// Propagates setup/counter failure.
/// # Panics
/// Every old row/reference/report survives, then the healthy17-second retry commits.
/// # Resources
/// One local scope; its exact bounded public-call recipe is in the source audit.
#[test]
fn spread_failure_preserves_existing_report_and_healthy_retry() -> TestResult {
    let profile = Profile::new();
    run_trace(&profile, "report_spread_failure_then_neighbor")
}

/// The same-frame 3-to-4 trajectory discriminates an old summary with refreshed labels.
/// # Errors
/// Propagates setup/counter failure.
/// # Panics
/// Old zero report must become exact A=3/8, B=1/8, E=5/32 and scale/variance=5/64.
/// # Resources
/// One local scope; its exact bounded public-call recipe is in the source audit.
#[test]
fn changing_data_same_frame_commits_fresh_associated_statistics() -> TestResult {
    let profile = Profile::new();
    run_trace(&profile, "changing_data_same_frame_report")
}

/// Keep both actual quarter histories alive; peer construction/appends cannot rewrite the first.
/// # Errors
/// Propagates setup/counter failure.
/// # Panics
/// Four genuine positive-history re-reads remain unchanged while the negative history grows.
/// # Resources
/// Two original streams, two histories, six appends; no reconstruction/replayed constructor.
#[test]
fn independent_same_frame_histories_coexist_with_distinct_reports() -> TestResult {
    let profile = Profile::new();
    let positive = fact("origin_zero_positive_quarter")?;
    let negative = fact("same_frame_negative_quarter")?;
    let (mut first, first_cache) = start_trace(&profile, positive)?;
    for step in positive.steps {
        run_step(&profile, &mut first, &first_cache, step)?;
    }
    let (mut second, second_cache) = start_trace(&profile, negative)?;
    inspect_snapshot(&profile, &first, PAIRED_POSITIVE)?;
    for step in negative.steps {
        run_step(&profile, &mut second, &second_cache, step)?;
        inspect_snapshot(&profile, &first, PAIRED_POSITIVE)?;
    }
    Ok(())
}

/// Expected diagnostic role and presence facts, independent of the formatter under test.
#[derive(Clone, Copy)]
enum DebugKind {
    /// A real history at an independently specified committed state.
    History {
        /// Whether the frozen fixture has a committed reference.
        reference_present: bool,
        /// Whether the frozen fixture has a sufficient committed report.
        report_present: bool,
    },
    /// A real sufficient report always has an exact reference value.
    Report,
}

/// Read diagnostic words/numbers without assigning public wire or layout semantics.
///
/// Accept any borrowed text and return its lazy nonempty alphanumeric/underscore
/// tokens. Iteration borrows the text, ignores whitespace/punctuation and takes
/// linear work in its actual length, without allocation, mutation or product
/// calls. Each consumer enforces its own1024-byte limit before traversal. Empty
/// input gives an empty iterator; the borrow cannot outlive the input.
fn debug_tokens(text: &str) -> impl Iterator<Item = &str> {
    text.split(|character: char| !character.is_ascii_alphanumeric() && character != '_')
        .filter(|token| !token.is_empty())
}

/// Require one named diagnostic value, allowing whitespace and field-order changes.
///
/// Inputs are the already bounded diagnostic text and fixed nonempty fixture
/// field/value tokens. This is a test observer, not a product decoder or format
/// stability promise. It scans borrowed tokens once and makes no product call.
/// # Panics
/// Rejects a missing, duplicate, incomplete or differently valued named field.
fn inspect_debug_field(text: &str, name: &str, expected: &str) {
    assert!(
        text.len() <= 1024,
        "{ORACLE}: bounded diagnostic field input"
    );
    assert!(
        !name.is_empty() && name.len() <= 32 && !expected.is_empty() && expected.len() <= 32,
        "{ORACLE}: named diagnostic fixture"
    );
    let mut pending = false;
    let mut found = false;
    for token in debug_tokens(text) {
        if pending {
            assert_eq!(token, expected, "{ORACLE}: diagnostic field {name}");
            pending = false;
            found = true;
        } else if token == name {
            assert!(!found, "{ORACLE}: duplicate diagnostic field {name}");
            pending = true;
        }
    }
    assert!(
        found && !pending,
        "{ORACLE}: missing diagnostic field {name}"
    );
}

/// Require useful bounded metadata without raw rows, times or report numbers.
///
/// These selected fixtures have independently fixed17/23/29 labels. Expected
/// kind/count/presence comes from their literal facts, never product getters.
/// Named semantic metadata is checked without freezing a complete Debug string,
/// field order, whitespace or a public product wire representation.
/// # Panics
/// Rejects absent/wrong metadata, numeric payloads and retained-data containers.
/// # Resources
/// One at-most1024-byte accepted string and fixed bounded token scans. Formatting
/// is not a product-value getter, constructor, normalizer or solver request.
fn inspect_private_debug(value: &impl Debug, count: usize, kind: DebugKind) {
    let text = format!("{value:?}");
    assert!(text.len() <= 1024, "{ORACLE}: bounded private Debug");
    for forbidden in [
        "FixtureObservation",
        "MonotonicNs",
        "PhaseOffsetQ33",
        "FixtureLine",
        "FixtureResidualStatistics",
        "observations",
        "variance_divisor",
    ] {
        assert!(
            !text.contains(forbidden),
            "{ORACLE}: hidden Debug container"
        );
    }
    let count_text = count.to_string();
    for token in text
        .split(|character: char| !character.is_ascii_digit())
        .filter(|token| !token.is_empty())
    {
        assert!(
            matches!(token, "0" | "17" | "23" | "29") || token == count_text,
            "{ORACLE}: numeric Debug input/output"
        );
    }
    let (label, reference_present) = match kind {
        DebugKind::History {
            reference_present, ..
        } => ("FixtureHistory", reference_present),
        DebugKind::Report => ("FixtureHistoryReport", true),
    };
    assert_eq!(
        debug_tokens(&text).next(),
        Some(label),
        "{ORACLE}: diagnostic type"
    );
    inspect_debug_field(&text, "context", "FixtureContext");
    inspect_debug_field(&text, "source_id", "17");
    inspect_debug_field(&text, "frame", "23");
    inspect_debug_field(&text, "generation", "29");
    inspect_debug_field(&text, "sample_count", &count_text);
    inspect_debug_field(
        &text,
        "reference_present",
        if reference_present { "true" } else { "false" },
    );
    if let DebugKind::History { report_present, .. } = kind {
        inspect_debug_field(
            &text,
            "report_present",
            if report_present { "true" } else { "false" },
        );
    }
}

/// Zero-product-call formatter fixture, including deliberately incomplete metadata.
struct DebugSample {
    /// One fixed bounded control literal; no product formatter creates this text.
    text: &'static str,
}

impl Debug for DebugSample {
    /// Emit the exact supplied control literal without adding metadata or numbers.
    /// # Errors
    /// Preserve the formatter sink's result; no other effects or product calls.
    /// # Resources
    /// One write bounded by the fixed control literals, each below1024 bytes.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.text)
    }
}

/// Challenge the actual observer with independent healthy and rejected neighbors.
///
/// Fixed mock text performs zero product requests. Positive text varies field
/// order/whitespace to avoid an exact formatter mirror. Each rejected formatter
/// is actually inspected under `catch_unwind`; acceptance is a failing control,
/// and the ordinary panic hook is unchanged.
/// # Panics
/// Rejects a failed positive or an unexpectedly accepted negative control.
/// # Resources
/// Two healthy and ten rejected bounded Debug literals; no extra test/profile.
fn debug_observer_controls() {
    let history = DebugKind::History {
        reference_present: true,
        report_present: true,
    };
    inspect_private_debug(
        &DebugSample {
            text: "FixtureHistory { report_present: true, context: FixtureContext { generation: 29, source_id: 17, frame: 23, .. }, reference_present: true, sample_count: 4, .. }",
        },
        4,
        history,
    );
    inspect_private_debug(
        &DebugSample {
            text: "FixtureHistoryReport{sample_count:4,reference_present:true,context:FixtureContext{frame:23,generation:29,source_id:17}}",
        },
        4,
        DebugKind::Report,
    );
    for text in [
        "",
        "FixtureHistory",
        "FixtureHistory { context: FixtureContext { source_id:17, frame:23, generation:29 }, sample_count:0, reference_present:true, report_present:true }",
        "FixtureHistory { context: FixtureContext { source_id:17, frame:23 }, sample_count:4, reference_present:true, report_present:true }",
        "FixtureHistory { context: FixtureContext { source_id:23, frame:17, generation:29 }, sample_count:4, reference_present:true, report_present:true }",
        "FixtureHistory { context: FixtureContext { source_id:17, frame:23, generation:29 }, sample_count:4, reference_present:false, report_present:true }",
        "FixtureHistory { context: FixtureContext { source_id:17, frame:23, generation:29 }, sample_count:4, reference_present:true, report_present:false }",
        "FixtureHistory { context: FixtureContext { source_id:17, frame:23, generation:29 }, sample_count:4, reference_present:true, report_present:true, FixtureObservation }",
        "FixtureHistory { context: FixtureContext { source_id:17, frame:23, generation:29 }, sample_count:4, reference_present:true, report_present:true, raw_phase:375 }",
        "FixtureHistoryReport { context: FixtureContext { source_id:17, frame:23, generation:29 }, sample_count:4, reference_present:true, report_present:true }",
    ] {
        let rejected =
            std::panic::catch_unwind(|| inspect_private_debug(&DebugSample { text }, 4, history));
        assert!(
            rejected.is_err(),
            "{ORACLE}: diagnostic observer accepted invalid metadata"
        );
    }
}

/// Actual empty/insufficient/sufficient histories and nonzero report preserve numeric privacy.
/// # Errors
/// Propagates real setup/admission/counter failure.
/// # Panics
/// The real nonconstant report exists and every `Debug` exposes only permitted bounded metadata.
/// # Resources
/// H1/A4/O4/C1/M4/U16/E4/G154, with no direct normalization/statistics/fit/point calls.
#[test]
fn history_and_nonzero_report_debug_preserve_numeric_privacy() -> TestResult {
    let profile = Profile::new();
    let trace = fact("changing_data_same_frame_report")?;
    let (mut history, cache) = start_trace(&profile, trace)?;
    inspect_private_debug(
        &history,
        0,
        DebugKind::History {
            reference_present: false,
            report_present: false,
        },
    );
    for (index, step) in trace.steps.iter().enumerate() {
        let candidate = admit(&profile, &cache, step.candidate)?;
        let retained_copy = candidate;
        let result = profile.append(&mut history, candidate)?;
        assert_eq!(
            observation_bits(&profile, &retained_copy)?,
            step.candidate,
            "{ORACLE}: private caller input"
        );
        inspect_outcome(&result, step);
        if index == 1 || index == 3 {
            inspect_snapshot(&profile, &history, step.after)?;
            inspect_private_debug(
                &history,
                step.after.sample_count,
                DebugKind::History {
                    reference_present: step.after.reference.is_some(),
                    report_present: step.after.report.is_some(),
                },
            );
        }
    }
    let report = profile.getter(|| history.report())?;
    assert!(report.is_some(), "{ORACLE}: private report must exist");
    inspect_private_debug(
        report.ok_or_else(|| io::Error::other("missing private actual report"))?,
        4,
        DebugKind::Report,
    );
    debug_observer_controls();
    Ok(())
}

/// Check bounded manual combinations and their genuine typed std-error source chains.
/// # Errors
/// Reports a missing/wrong typed source as a semantic test failure.
/// # Panics
/// Useful closed context, total formatting and exact nested cause identity must be preserved.
/// # Resources
/// Only standard formatting/Error calls, no public value constructor/getter/solver request.
fn inspect_manual_error(error: &HistoryError) -> TestResult {
    let display = error.to_string();
    let debug = format!("{error:?}");
    assert!(
        !display.is_empty() && display.len() <= 1024 && debug.len() <= 1024,
        "{ORACLE}: bounded manual error"
    );
    let lower = display.to_ascii_lowercase();
    if matches!(error, HistoryError::ContextMismatch) {
        assert!(
            lower.contains("context"),
            "{ORACLE}: useful context mismatch"
        );
        assert!(error.source().is_none(), "{ORACLE}: leaf error source");
    } else if matches!(error, HistoryError::DuplicateTime) {
        assert!(
            lower.contains("duplicate"),
            "{ORACLE}: useful duplicate context"
        );
        assert!(error.source().is_none(), "{ORACLE}: leaf error source");
    } else if matches!(error, HistoryError::OutOfOrderTime) {
        assert!(lower.contains("order"), "{ORACLE}: useful ordering context");
        assert!(error.source().is_none(), "{ORACLE}: leaf error source");
    } else if matches!(error, HistoryError::StateInvariant) {
        assert!(
            lower.contains("state") || lower.contains("invariant"),
            "{ORACLE}: useful state invariant context"
        );
        assert!(
            error.source().is_none(),
            "{ORACLE}: state invariant is a leaf"
        );
    } else if let HistoryError::Normalization { source } = error {
        assert!(
            lower.contains("normal") || lower.contains("prepar"),
            "{ORACLE}: useful normalization context"
        );
        let actual = error
            .source()
            .and_then(|cause| cause.downcast_ref::<NormalizationError>())
            .ok_or_else(|| io::Error::other("missing typed normalization source"))?;
        assert_eq!(
            format!("{actual:?}"),
            format!("{source:?}"),
            "{ORACLE}: actual normalization cause"
        );
        if matches!(source, NormalizationError::Input(_)) {
            assert!(
                actual.source().is_some_and(<dyn Error>::is::<FitError>),
                "{ORACLE}: nested fit source"
            );
        }
    } else if let HistoryError::Statistics { source } = error {
        assert!(
            lower.contains("stat") || lower.contains("resid"),
            "{ORACLE}: useful statistics context"
        );
        let actual = error
            .source()
            .and_then(|cause| cause.downcast_ref::<ResidualError>())
            .ok_or_else(|| io::Error::other("missing typed statistics source"))?;
        assert_eq!(
            format!("{actual:?}"),
            format!("{source:?}"),
            "{ORACLE}: actual statistics cause"
        );
        if matches!(source, ResidualError::Fit(_)) {
            assert!(
                actual.source().is_some_and(<dyn Error>::is::<FitError>),
                "{ORACLE}: nested fit source"
            );
        }
    } else {
        return Err(io::Error::other("nonfinal error is not an independent manual fact").into());
    }
    Ok(())
}

/// Genuine manual public-error consumers remain total at `usize::MAX` and retain typed causes.
/// # Errors
/// Reports an actual source-chain violation; no fixture constructor or history update is performed.
/// # Panics
/// All actual `Display`/`Debug`/source obligations must hold; temporary missing behavior is excluded.
/// # Resources
/// One real all-zero counter scope and nine closed error combinations.
#[test]
fn manual_history_errors_preserve_bounded_context_and_typed_sources() -> TestResult {
    let _profile = Profile::new();
    let errors = [
        HistoryError::ContextMismatch,
        HistoryError::DuplicateTime,
        HistoryError::OutOfOrderTime,
        HistoryError::StateInvariant,
        HistoryError::Normalization {
            source: NormalizationError::ElapsedRange { index: usize::MAX },
        },
        HistoryError::Normalization {
            source: NormalizationError::ContextMismatch { index: usize::MAX },
        },
        HistoryError::Normalization {
            source: NormalizationError::Input(FitError::InsufficientSpread),
        },
        HistoryError::Statistics {
            source: ResidualError::Fit(FitError::InsufficientSpread),
        },
        HistoryError::Statistics {
            source: ResidualError::CountInvariant,
        },
    ];
    for error in &errors {
        inspect_manual_error(error)?;
    }
    Ok(())
}

// SPDX-License-Identifier: GPL-2.0-only
//! Independent S1.2c external normalization consumers; pure CHECK-7/CHECK-25 scope.
//!
//! Truth: normalization-facts-v2.json SHA-256
//! 2c165c30ea2d6c1ea650d6c8bf6921d3c007870de43c9dce89cad34e2abef839;
//! contract d21c51fcb115be0d3ab346da7053db6801aa0a6750931dee7b781819ddd25d30.
//! No future implementation body or Rust conversion model supplies expectations.
//!
//! A complete source profile has 12 scopes and actual public attempts:
//! normalizations113 observations4449 contexts201 monotonic4449 coordinates17796
//! exchanges4449 getters186 stats3 points0. The 61 frozen sets expand into 124
//! individually charged evaluations; the 64-context family rebuilds each input.
//! Extra scopes add 11 accepted observations, two ownership requests/four
//! observations/15 getters/one stats request, and two privacy requests/three
//! observations. There are 603 literal `NonZeroU64` constructions inside 201 context
//! recipes; these standard-library constructors are outside the nine-field record.
//! Internal duration/generated-point/`FitInput` work remains source-owned upper
//! bounds 64/64/1 per actual normalization request, never observed point telemetry.
//! There are no direct fit/point calls, doctests, ambient clocks, global counters,
//! network/native effects, retries or implementation-derived oracle calculations.
//! Every scope emits its actual prefix through one assembled `Drop` `write_all` record.
//!
//! Complete static recipes, in counter order N/O/C/M/U/E/G/S/P:
//! constructor errors=0/30/15/30/120/30/0/0/0;
//! healthy constructors=0/11/1/11/44/11/0/0/0;
//! count65=1/65/2/65/260/65/0/0/0; empty=1/0/1/0/0/0/0/0/0;
//! nonempty insufficient=4/6/4/6/24/6/0/0/0;
//! structural errors=16/104/25/104/416/104/0/0/0;
//! prepared facts=20/121/20/121/484/121/161/2/0;
//! spread neighbors=3/9/3/9/36/9/10/0/0;
//! context family=64/4096/128/4096/16384/4096/0/0/0;
//! ownership=2/4/1/4/16/4/15/1/0; privacy=2/3/1/3/12/3/0/0/0;
//! manual error formatting=0/0/0/0/0/0/0/0/0.
#![forbid(unsafe_code)]

#[path = "support/normalization_facts.rs"]
mod normalization_facts;

use std::{
    cell::Cell,
    error::Error,
    fmt::Debug,
    io::{self, Write},
    mem,
    num::NonZeroU64,
};

use normalization_facts::{CASES, CONTEXT_FAMILY, Case, Kind, RawRow};
use time_core::{
    ArithmeticIssue, ArithmeticOperation, ArithmeticStage, FitError, FitInput, FitPoint,
    FixtureContext, FixtureNormalization, FixtureObservation, FixtureResidualStatistics,
    NormalizationComponent, NormalizationError, NormalizationStep, ObservationError, PointField,
    PointIssue, PreparedFixtureInput, ResidualError, prepare_fixture_input, residual_statistics,
};
use time_domain::{
    DomainError, ExchangeArithmetic, MonotonicNs, PhaseOffsetQ33, UtcCoordinateQ32,
    exchange_arithmetic,
};

/// Harness failures stay separate from actual semantic assertions and product errors.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
/// Immutable scalar snapshot of one admitted source observation.
type ObservationSnapshot = ([u64; 3], u64, i128, u64);
/// One owned constructor cache entry, with no public raw context factory.
type ContextEntry = ([u64; 3], FixtureContext);

/// One test-local actual-public-attempt owner; no counter is shared between scopes.
struct Profile {
    /// Actual prepare attempts, including rejected ones.
    normalizations: Cell<u64>,
    /// Actual observation-constructor attempts, including rejected ones.
    observations: Cell<u64>,
    /// Actual three-tag context-constructor attempts.
    contexts: Cell<u64>,
    /// Actual exact monotonic wrapper constructions.
    monotonic: Cell<u64>,
    /// Actual UTC coordinate-constructor attempts for legitimate phase setup.
    coordinates: Cell<u64>,
    /// Actual four-coordinate exchange-arithmetic attempts.
    exchanges: Cell<u64>,
    /// Actual public held-point getter requests, including invalid indices.
    getters: Cell<u64>,
    /// Actual downstream residual-statistics requests.
    stats: Cell<u64>,
    /// Actual direct external point attempts; this file performs none.
    points: Cell<u64>,
}

impl Profile {
    /// Start one all-zero scope, with no product construction or external effect.
    const fn new() -> Self {
        Self {
            normalizations: Cell::new(0),
            observations: Cell::new(0),
            contexts: Cell::new(0),
            monotonic: Cell::new(0),
            coordinates: Cell::new(0),
            exchanges: Cell::new(0),
            getters: Cell::new(0),
            stats: Cell::new(0),
            points: Cell::new(0),
        }
    }

    /// Build three real nonzero labels, then charge immediately before actual context construction.
    /// # Errors
    /// Rejects invalid fixture labels or a counter ceiling; no fallback identity.
    /// # Resources
    /// Exactly three standard-library nonzero constructors and one public context attempt.
    fn context(&self, raw: [u64; 3]) -> TestResult<FixtureContext> {
        let source = nonzero(raw[0])?;
        let frame = nonzero(raw[1])?;
        let generation = nonzero(raw[2])?;
        charge(&self.contexts, 256)?;
        Ok(FixtureContext::new(source, frame, generation))
    }

    /// Charge before the real exact monotonic wrapper constructor; no clock is read.
    /// # Errors
    /// Returns only a harness counter failure; constant bounded work.
    fn monotonic(&self, raw: u64) -> TestResult<MonotonicNs> {
        charge(&self.monotonic, 8320)?;
        Ok(MonotonicNs::new(raw))
    }

    /// Charge before one actual checked UTC coordinate construction.
    /// # Errors
    /// Preserves actual domain/setup rejection or the counter error; no retagging.
    fn coordinate(&self, ticks: i128) -> TestResult<UtcCoordinateQ32> {
        charge(&self.coordinates, 33_280)?;
        Ok(UtcCoordinateQ32::new(ticks)?)
    }

    /// Charge before actual inert exchange arithmetic; no raw Q33 value is fabricated.
    /// # Errors
    /// Preserves exact domain/setup rejection or the counter error.
    fn exchange(&self, values: [UtcCoordinateQ32; 4]) -> TestResult<ExchangeArithmetic> {
        charge(&self.exchanges, 8320)?;
        Ok(exchange_arithmetic(
            values[0], values[1], values[2], values[3],
        )?)
    }

    /// Charge immediately before the sole actual observation constructor call.
    /// # Errors
    /// Returns a harness error or preserves the actual mandatory typed product result.
    fn observation(
        &self,
        context: FixtureContext,
        at: MonotonicNs,
        phase: PhaseOffsetQ33,
        divisor: f64,
    ) -> TestResult<Result<FixtureObservation, ObservationError>> {
        charge(&self.observations, 8320)?;
        Ok(FixtureObservation::new(context, at, phase, divisor))
    }

    /// Charge before one actual normalization request, preserving its immutable source.
    /// # Errors
    /// Returns a harness error or the unchanged actual product result.
    /// # Panics
    /// Asserts exact source preservation after either actual product return.
    fn normalize(
        &self,
        context: FixtureContext,
        rows: &[FixtureObservation],
    ) -> TestResult<Result<FixtureNormalization, NormalizationError>> {
        let before: Vec<_> = rows.iter().map(observation_snapshot).collect();
        charge(&self.normalizations, 128)?;
        let result = prepare_fixture_input(context, rows);
        assert_eq!(
            rows.iter().map(observation_snapshot).collect::<Vec<_>>(),
            before
        );
        Ok(result)
    }

    /// Charge before the actual held-prefix getter, including arbitrary invalid indices.
    /// # Errors
    /// Returns only a counter error; the public getter must be total for every index.
    /// # Resources
    /// One bounded lookup with a borrow tied to the actual prepared payload.
    fn point<'a>(
        &self,
        prepared: &'a PreparedFixtureInput,
        index: usize,
    ) -> TestResult<Option<&'a FitPoint>> {
        charge(&self.getters, 8448)?;
        Ok(prepared.point(index))
    }

    /// Charge before the sole downstream actual statistics call; never invoke direct fit.
    /// # Errors
    /// Returns a harness error or the unchanged typed product result.
    fn statistics(
        &self,
        input: &FitInput,
    ) -> TestResult<Result<FixtureResidualStatistics, ResidualError>> {
        charge(&self.stats, 16)?;
        Ok(residual_statistics(input))
    }
}

impl Drop for Profile {
    /// Emit one whole best-effort `write_all` record, also during assertion unwinding.
    /// A missing/failed sink remains incomplete supervisor evidence.
    fn drop(&mut self) {
        let record = format!(
            "CORE_NORMALIZATION_PROFILE_COUNTS normalizations={} observations={} contexts={} monotonic={} coordinates={} exchanges={} getters={} stats={} points={}\n",
            self.normalizations.get(),
            self.observations.get(),
            self.contexts.get(),
            self.monotonic.get(),
            self.coordinates.get(),
            self.exchanges.get(),
            self.getters.get(),
            self.stats.get(),
            self.points.get()
        );
        let _emission_result = io::stderr().lock().write_all(record.as_bytes());
    }
}

/// Charge before its actual operation; reject without calling the product on failure.
/// # Errors
/// Rejects overflow or a scope ceiling; constant work, no retry or shared state.
fn charge(counter: &Cell<u64>, maximum: u64) -> TestResult {
    let next = counter
        .get()
        .checked_add(1)
        .ok_or_else(|| io::Error::other("normalization profile overflow"))?;
    if next > maximum {
        return Err(io::Error::other("normalization profile ceiling").into());
    }
    counter.set(next);
    Ok(())
}

/// Construct the actual standard-library tag, never invent a zero-label fallback.
/// # Errors
/// Rejects a zero fixture label; constant bounded work.
fn nonzero(raw: u64) -> TestResult<NonZeroU64> {
    Ok(NonZeroU64::new(raw).ok_or_else(|| io::Error::other("zero fixture tag"))?)
}

/// Observe each immutable exact context component with no construction or validation.
fn context_bits(context: FixtureContext) -> [u64; 3] {
    [
        context.source_id().get(),
        context.frame().get(),
        context.generation().get(),
    ]
}

/// Snapshot the real source observation, including unchanged divisor and clock bits.
fn observation_snapshot(observation: &FixtureObservation) -> ObservationSnapshot {
    (
        context_bits(observation.context()),
        observation.observed_at().nanoseconds(),
        observation.phase().numerator(),
        observation.variance_divisor().to_bits(),
    )
}

/// Reuse one already validated immutable context per distinct asserted label tuple.
/// # Errors
/// Rejects a malformed >2-context fixture or an actual counted constructor failure.
/// # Resources
/// At most two cache entries; no global/default/raw identity.
/// # Panics
/// Asserts that the real context constructor retains the supplied label tuple.
fn cached_context(
    profile: &Profile,
    cache: &mut Vec<ContextEntry>,
    raw: [u64; 3],
) -> TestResult<FixtureContext> {
    if let Some((_, value)) = cache.iter().find(|(labels, _)| *labels == raw) {
        return Ok(*value);
    }
    if cache.len() >= 2 {
        return Err(io::Error::other("fixture context cache ceiling").into());
    }
    let value = profile.context(raw)?;
    assert_eq!(context_bits(value), raw);
    cache.push((raw, value));
    Ok(value)
}

/// Produce a real broad typed Q33 phase through four actual checked coordinates.
/// # Errors
/// Propagates real setup/domain/counter rejection; no raw phase constructor is used.
/// # Panics
/// Asserts the independent exact exchange numerator before the observation boundary.
/// # Resources
/// Exactly four coordinate attempts and one exchange attempt, with no clock effects.
fn phase(profile: &Profile, numerator: i128) -> TestResult<PhaseOffsetQ33> {
    let coordinates = [
        profile.coordinate(0)?,
        profile.coordinate(numerator)?,
        profile.coordinate(0)?,
        profile.coordinate(0)?,
    ];
    let value = profile.exchange(coordinates)?.offset();
    assert_eq!(value.numerator(), numerator);
    Ok(value)
}

/// Reach real typed observation admission and check exact accepted source getters.
/// # Errors
/// Preserves actual observation rejection; setup/counter errors remain separate.
/// # Panics
/// Asserts context, exact integer time/Q33 and divisor-bit retention after admission.
/// # Resources
/// One monotonic/four coordinate/one exchange/one observation attempt.
fn admit_observation(
    profile: &Profile,
    context: FixtureContext,
    raw: &RawRow,
) -> TestResult<Result<FixtureObservation, ObservationError>> {
    let at = profile.monotonic(raw.observed_ns)?;
    let exact_phase = phase(profile, raw.phase_q33)?;
    let result = profile.observation(context, at, exact_phase, f64::from_bits(raw.divisor_bits))?;
    if let Ok(value) = &result {
        assert_eq!(
            observation_snapshot(value),
            (
                raw.context,
                raw.observed_ns,
                raw.phase_q33,
                raw.divisor_bits
            )
        );
    }
    Ok(result)
}

/// Identify fixture-only observation categories without interpreting product text.
fn is_observation_error(kind: Kind) -> bool {
    matches!(
        kind,
        Kind::ObservationPhase
            | Kind::ObservationNonFinite
            | Kind::ObservationSubnormal
            | Kind::ObservationRange
    )
}

/// Match the independently expected real mandatory-constructor error category.
fn observation_error_matches(error: ObservationError, kind: Kind) -> bool {
    matches!(
        (kind, error),
        (Kind::ObservationPhase, ObservationError::PhaseOutOfRange)
            | (
                Kind::ObservationNonFinite,
                ObservationError::Divisor {
                    issue: PointIssue::NonFinite
                }
            )
            | (
                Kind::ObservationSubnormal,
                ObservationError::Divisor {
                    issue: PointIssue::Subnormal
                }
            )
            | (
                Kind::ObservationRange,
                ObservationError::Divisor {
                    issue: PointIssue::OutOfRange
                }
            )
    )
}

/// Match exact first preparation owner/index; temporary stubs never satisfy a case.
fn normalization_error_matches(
    error: Option<&NormalizationError>,
    kind: Kind,
    index: Option<usize>,
) -> bool {
    match (kind, error) {
        (Kind::Count, Some(NormalizationError::CountOutOfRange))
        | (Kind::Span, Some(NormalizationError::SpanTooShort))
        | (Kind::Spread, Some(NormalizationError::Input(FitError::InsufficientSpread))) => true,
        (Kind::Context, Some(NormalizationError::ContextMismatch { index: actual }))
        | (Kind::Order, Some(NormalizationError::NonIncreasingTime { index: actual }))
        | (Kind::Elapsed, Some(NormalizationError::ElapsedRange { index: actual })) => {
            Some(*actual) == index
        }
        _ => false,
    }
}

/// Observe the actual held chronological prefix directly, not via a constant fit.
/// # Errors
/// Propagates counter failures or missing actual held points.
/// # Panics
/// Asserts context/frame/reference/count, every literal x/y/d bit and both invalid indices.
/// # Resources
/// Exactly n+2 public point requests, n<=64, with no reconstruction or solver.
fn inspect_prepared(
    profile: &Profile,
    prepared: &PreparedFixtureInput,
    expected_context: [u64; 3],
    reference: u64,
    expected: &[[u64; 3]],
) -> TestResult {
    assert_eq!(context_bits(prepared.context()), expected_context);
    assert_eq!(prepared.reference().nanoseconds(), reference);
    assert_eq!(prepared.sample_count(), expected.len());
    assert_eq!(prepared.input().sample_count(), expected.len());
    assert_eq!(prepared.input().frame().get(), expected_context[1]);
    for (index, wanted) in expected.iter().enumerate() {
        let point = profile
            .point(prepared, index)?
            .ok_or_else(|| io::Error::other("missing actual held prefix point"))?;
        let actual = [
            point.relative_seconds().to_bits(),
            point.phase_seconds().to_bits(),
            point.variance_divisor().to_bits(),
        ];
        assert_eq!(
            actual, *wanted,
            "frozen actual held point bits at index {index}"
        );
    }
    assert!(
        profile.point(prepared, expected.len())?.is_none(),
        "unused fixed slot must not escape"
    );
    assert!(
        profile.point(prepared, usize::MAX)?.is_none(),
        "arbitrary getter index must be total"
    );
    Ok(())
}

/// Check one separately charged exact dyadic consumer through its owned input.
/// # Errors
/// Propagates the actual downstream statistics rejection or counter failure.
/// # Panics
/// Asserts independent phase1/512, rate1/1024 and exact positive-zero E/s/v.
/// # Resources
/// Exactly one statistics request, no direct fit or point construction.
fn inspect_dyadic_statistics(profile: &Profile, prepared: &PreparedFixtureInput) -> TestResult {
    let report = profile.statistics(prepared.input())??;
    assert_eq!(report.frame(), prepared.context().frame());
    assert_eq!(report.line().frame(), prepared.context().frame());
    assert_eq!(report.sample_count(), 3);
    assert_eq!(report.degrees_of_freedom(), 1);
    assert_eq!(
        report.line().phase_seconds().to_bits(),
        0x3f60_0000_0000_0000
    );
    assert_eq!(
        report.line().phase_rate_seconds_per_second().to_bits(),
        0x3f50_0000_0000_0000
    );
    assert_eq!(report.weighted_sse_seconds_squared().to_bits(), 0);
    assert_eq!(report.residual_scale_seconds_squared().to_bits(), 0);
    assert_eq!(
        report.normalized_data_variance_seconds_squared().to_bits(),
        0
    );
    Ok(())
}

/// Execute one literal fixture using legitimate domain constructors and actual consumers.
/// # Errors
/// Propagates setup/counter errors or a genuinely unexpected product failure.
/// # Panics
/// Asserts independent first outcomes and unchanged source on both preparation returns.
/// # Resources
/// At most65 observations; no normalization occurs after an observation rejection.
fn run_case(
    profile: &Profile,
    case: &Case,
    rows: &[RawRow],
    expected_index: Option<usize>,
) -> TestResult {
    let mut cache = Vec::with_capacity(2);
    let expected_context = cached_context(profile, &mut cache, case.context)?;
    let mut observations = Vec::with_capacity(rows.len());
    for (index, raw) in rows.iter().enumerate() {
        let context = cached_context(profile, &mut cache, raw.context)?;
        match admit_observation(profile, context, raw)? {
            Ok(value) => observations.push(value),
            Err(error) => {
                assert!(
                    is_observation_error(case.kind),
                    "unexpected observation rejection: {}",
                    case.name
                );
                assert_eq!(Some(index), expected_index);
                assert!(
                    observation_error_matches(error, case.kind),
                    "wrong observation precedence: {}",
                    case.name
                );
                return Ok(());
            }
        }
    }
    assert!(
        !is_observation_error(case.kind),
        "invalid raw observation was accepted: {}",
        case.name
    );
    let before: Vec<_> = observations.iter().map(observation_snapshot).collect();
    let result = profile.normalize(expected_context, &observations)?;
    assert_eq!(
        observations
            .iter()
            .map(observation_snapshot)
            .collect::<Vec<_>>(),
        before
    );
    if matches!(case.kind, Kind::Prepared | Kind::Insufficient) {
        match result? {
            FixtureNormalization::Insufficient(value) => {
                assert!(
                    matches!(case.kind, Kind::Insufficient),
                    "prepared fixture became insufficient"
                );
                assert_eq!(context_bits(value.context()), case.context);
                assert_eq!(value.sample_count(), rows.len());
                assert_eq!(
                    value.reference().map(MonotonicNs::nanoseconds),
                    case.reference
                );
            }
            FixtureNormalization::Prepared(value) => {
                assert!(
                    matches!(case.kind, Kind::Prepared),
                    "insufficient fixture manufactured a prepared input"
                );
                let reference = case
                    .reference
                    .ok_or_else(|| io::Error::other("missing prepared reference fact"))?;
                inspect_prepared(profile, &value, case.context, reference, case.prepared_bits)?;
                if case.downstream {
                    inspect_dyadic_statistics(profile, &value)?;
                }
            }
        }
    } else {
        assert!(
            normalization_error_matches(result.as_ref().err(), case.kind, expected_index),
            "wrong first normalization failure: {} {:?}",
            case.name,
            result.as_ref().err()
        );
    }
    Ok(())
}

/// Find one frozen literal without silently replacing a missing fixture.
/// # Errors
/// Rejects an unknown source fact name; bounded search across60 named entries.
fn fact(name: &str) -> TestResult<&'static Case> {
    Ok(CASES
        .iter()
        .find(|case| case.name == name)
        .ok_or_else(|| io::Error::other("missing named normalization fact"))?)
}

/// Extract a successful prepared payload without accepting a fabricated fallback.
/// # Errors
/// Propagates actual normalization failure or an unexpected insufficient result.
fn prepared(
    result: Result<FixtureNormalization, NormalizationError>,
) -> TestResult<PreparedFixtureInput> {
    match result? {
        FixtureNormalization::Prepared(value) => Ok(value),
        FixtureNormalization::Insufficient(_) => {
            Err(io::Error::other("expected actual prepared payload").into())
        }
    }
}

/// Constructor-only facts: all 15 first-error/precedence cases, no preparation.
/// # Errors
/// Propagates real setup/counter failures.
/// # Panics
/// Requires actual phase-before-divisor and nonfinite/subnormal/range distinctions.
#[test]
fn observation_rejections_enforce_frozen_field_precedence() -> TestResult {
    let profile = Profile::new();
    for case in CASES.iter().filter(|case| is_observation_error(case.kind)) {
        run_case(&profile, case, case.rows, case.error_index)?;
    }
    Ok(())
}

/// Eleven independent healthy neighbors prevent rejection-only constructor proof.
/// # Errors
/// Propagates any real healthy constructor/setup/counter failure.
/// # Panics
/// Asserts retained exact getter values at all phase/divisor endpoints and neighbors.
/// # Resources
/// One context, 11 observations/monotonic/exchanges, 44 coordinates, no normalization.
#[test]
fn observation_healthy_boundaries_preserve_exact_typed_values() -> TestResult {
    let profile = Profile::new();
    let context = profile.context([17, 23, 29])?;
    for numerator in [
        0,
        1,
        -1,
        1_i128 << 33,
        -(1_i128 << 33),
        (1_i128 << 33) - 1,
        -(1_i128 << 33) + 1,
    ] {
        let raw = RawRow {
            context: [17, 23, 29],
            observed_ns: u64::MAX,
            phase_q33: numerator,
            divisor_bits: 0x3ff0_0000_0000_0000,
        };
        let _value = admit_observation(&profile, context, &raw)??;
    }
    for divisor_bits in [
        0x3fb0_0000_0000_0000,
        0x4030_0000_0000_0000,
        0x3fb0_0000_0000_0001,
        0x402f_ffff_ffff_ffff,
    ] {
        let raw = RawRow {
            context: [17, 23, 29],
            observed_ns: 0,
            phase_q33: 0,
            divisor_bits,
        };
        let _value = admit_observation(&profile, context, &raw)??;
    }
    Ok(())
}

/// Exactly 65 checked rows must reject count before context/order/minimum span.
/// # Errors
/// Propagates setup/counter failures.
/// # Panics
/// Asserts the sole real preparation boundary that the compiling RED stub implements.
#[test]
fn oversized_count_precedes_context_order_and_span() -> TestResult {
    let profile = Profile::new();
    let case = fact("count_65_precedes_context_order_and_span")?;
    run_case(&profile, case, case.rows, case.error_index)
}

/// Empty input must publish typed insufficient metadata, never a successful line.
/// # Errors
/// Propagates actual setup/preparation failure; a temporary stub remains RED.
#[test]
fn empty_input_has_typed_insufficient_without_a_reference() -> TestResult {
    let profile = Profile::new();
    let case = fact("count_zero")?;
    run_case(&profile, case, case.rows, case.error_index)
}

/// Four valid nonempty insufficient facts retain exact newest references and counts.
/// # Errors
/// Propagates actual setup/preparation failure; no minimum span is invented.
#[test]
fn nonempty_insufficient_inputs_preserve_context_reference_and_count() -> TestResult {
    let profile = Profile::new();
    for case in CASES
        .iter()
        .filter(|case| matches!(case.kind, Kind::Insufficient) && !case.rows.is_empty())
    {
        run_case(&profile, case, case.rows, case.error_index)?;
    }
    Ok(())
}

/// Sixteen first structural failures retain context-before-order/window precedence.
/// # Errors
/// Propagates setup/counter failures.
/// # Panics
/// Requires the independent first reason/index and exact unchanged source observations.
#[test]
fn structural_rejections_preserve_first_error_and_immutable_source() -> TestResult {
    let profile = Profile::new();
    for case in CASES.iter().filter(|case| {
        matches!(
            case.kind,
            Kind::Context | Kind::Order | Kind::Elapsed | Kind::Span
        )
    }) {
        run_case(&profile, case, case.rows, case.error_index)?;
    }
    Ok(())
}

/// Twenty prepared facts directly inspect every actual held x/y/d bit and invalid index.
/// # Errors
/// Propagates actual setup/normalization/statistics failure; no fallback is accepted.
/// # Panics
/// Includes one-ns point1, exact endpoint/Q33 phases, origin translation and two dyadic consumers.
#[test]
fn prepared_inputs_expose_frozen_actual_rows_and_exact_dyadic_consumers() -> TestResult {
    let profile = Profile::new();
    for case in CASES.iter().filter(|case| {
        matches!(case.kind, Kind::Prepared) && !case.name.starts_with("spread_calculated_")
    }) {
        run_case(&profile, case, case.rows, case.error_index)?;
    }
    Ok(())
}

/// Actual computed-binary64 spread neighbors require reject/accept/accept unchanged.
/// # Errors
/// Propagates setup/counter failures or rejection of either healthy neighbor.
/// # Panics
/// One source-owned input error and two real held-prefix controls discriminate the cutoff.
#[test]
fn calculated_spread_neighbors_reject_accept_accept() -> TestResult {
    let profile = Profile::new();
    for name in [
        "spread_calculated_below",
        "spread_calculated_exact",
        "spread_calculated_above",
    ] {
        let case = fact(name)?;
        run_case(&profile, case, case.rows, case.error_index)?;
    }
    Ok(())
}

/// Every 64-row mismatch position rejects before any duration/conversion/public result.
/// # Errors
/// Propagates setup/counter failures, with no context inference from equal frames.
/// # Panics
/// Asserts the exact index 0..63 through 64 independent requests.
/// # Resources
/// Rebuilds 64 observations per subcase:64 normalizations,4096 observations,128 contexts.
#[test]
fn every_context_mismatch_position_is_checked_before_other_preparation() -> TestResult {
    let profile = Profile::new();
    let expected = Case {
        name: "all_64_context_mismatch_positions",
        context: [17, 23, 29],
        rows: CONTEXT_FAMILY,
        kind: Kind::Context,
        error_index: None,
        reference: None,
        prepared_bits: &[],
        downstream: false,
    };
    for index in 0..64 {
        let mut rows = CONTEXT_FAMILY.to_vec();
        rows.get_mut(index)
            .ok_or_else(|| io::Error::other("missing mismatch fixture row"))?
            .context = [19, 23, 29];
        run_case(&profile, &expected, &rows, Some(index))?;
    }
    Ok(())
}

/// Same-frame changed observations cannot replace held input association or ownership.
/// # Errors
/// Propagates any actual constructor/normalization/statistics failure.
/// # Panics
/// Held A keeps original bits after caller replacement and drop; B has fresh phase bits.
/// # Resources
/// Two requests/four observations/one context/15 getters/one downstream statistics request.
#[test]
fn prepared_payload_owns_rows_and_same_frame_keeps_fresh_input_association() -> TestResult {
    let profile = Profile::new();
    let case = fact("dyadic_line_small_origin")?;
    let context = profile.context(case.context)?;
    let (first, second) = {
        let mut observations = Vec::with_capacity(3);
        for raw in case.rows {
            observations.push(admit_observation(&profile, context, raw)??);
        }
        let first = prepared(profile.normalize(context, &observations)?)?;
        inspect_prepared(
            &profile,
            &first,
            case.context,
            2_000_000_000,
            case.prepared_bits,
        )?;
        let mut replacement = *case
            .rows
            .first()
            .ok_or_else(|| io::Error::other("missing owned source fact"))?;
        replacement.phase_q33 = 1_i128 << 23;
        *observations
            .first_mut()
            .ok_or_else(|| io::Error::other("missing owned observation"))? =
            admit_observation(&profile, context, &replacement)??;
        let before: Vec<_> = observations.iter().map(observation_snapshot).collect();
        let second = prepared(profile.normalize(context, &observations)?)?;
        assert_eq!(
            observations
                .iter()
                .map(observation_snapshot)
                .collect::<Vec<_>>(),
            before
        );
        let changed = [
            [
                0xc000_0000_0000_0000,
                0x3f50_0000_0000_0000,
                0x3ff0_0000_0000_0000,
            ],
            [
                0xbff0_0000_0000_0000,
                0x3f50_0000_0000_0000,
                0x3ff0_0000_0000_0000,
            ],
            [0, 0x3f60_0000_0000_0000, 0x3ff0_0000_0000_0000],
        ];
        inspect_prepared(&profile, &second, case.context, 2_000_000_000, &changed)?;
        (first, second)
    };
    inspect_prepared(
        &profile,
        &first,
        case.context,
        2_000_000_000,
        case.prepared_bits,
    )?;
    assert_eq!(
        context_bits(first.context()),
        context_bits(second.context())
    );
    inspect_dyadic_statistics(&profile, &first)?;
    Ok(())
}

/// Require bounded metadata-only `Debug` tokens, never raw clocks/phases/divisors.
/// # Panics
/// Rejects unexpected identifiers or numeric payloads even if valid metadata is present.
/// # Resources
/// One bounded string; no product constructor, point getter or solver request.
fn assert_private_debug(value: &impl Debug) {
    let text = format!("{value:?}");
    assert!(text.len() <= 512);
    for token in text
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
        .filter(|s| !s.is_empty())
    {
        let token = token.to_ascii_lowercase();
        assert!(
            matches!(
                token.as_str(),
                "fixturecontext"
                    | "fixtureobservation"
                    | "insufficientfixture"
                    | "preparedfixtureinput"
                    | "input"
                    | "fitinput"
                    | "context"
                    | "source_id"
                    | "frame"
                    | "generation"
                    | "sample_count"
                    | "count"
                    | "reference"
                    | "reference_present"
                    | "has_reference"
                    | "some"
                    | "none"
                    | "nonzerou64"
                    | "true"
                    | "false"
                    | "0"
                    | "3"
                    | "17"
                    | "23"
                    | "29"
            ),
            "normalization Debug exposed non-metadata token"
        );
    }
}

/// Three new containers expose only bounded context/count/reference-presence metadata.
/// # Errors
/// Propagates actual setup/normalization failure.
/// # Resources
/// One context/three observations/two normalization requests; no point/statistics request.
#[test]
fn observation_insufficient_and_prepared_debug_preserve_numeric_privacy() -> TestResult {
    let profile = Profile::new();
    let context = profile.context([17, 23, 29])?;
    let mut observations = Vec::with_capacity(3);
    for (observed_ns, phase_q33) in [
        (317_250_000, 123_456_789),
        (1_317_250_000, 1_i128 << 23),
        (2_317_250_000, 1_i128 << 24),
    ] {
        let raw = RawRow {
            context: [17, 23, 29],
            observed_ns,
            phase_q33,
            divisor_bits: 0x3ff0_0000_0000_0000,
        };
        let observation = admit_observation(&profile, context, &raw)??;
        assert_private_debug(&observation);
        observations.push(observation);
    }
    match profile.normalize(context, &[])?? {
        FixtureNormalization::Insufficient(value) => assert_private_debug(&value),
        FixtureNormalization::Prepared(_) => {
            return Err(io::Error::other("empty privacy fixture was prepared").into());
        }
    }
    let value = prepared(profile.normalize(context, &observations)?)?;
    assert_private_debug(&value);
    Ok(())
}

/// Compact only allowlisted typed-context spelling, independent of punctuation.
fn compact(value: &str) -> String {
    value
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .flat_map(char::to_lowercase)
        .collect()
}

/// Assert bounded useful formatting without raw fixture coordinates or library reports.
/// # Panics
/// Requires all typed context codes and bounded `Display`/`Debug`; no public calls.
fn assert_error_context(error: &impl Error, codes: &[&str]) {
    let display = error.to_string();
    let debug = format!("{error:?}");
    assert!(display.len() <= 512 && debug.len() <= 512);
    let normalized = compact(&display);
    for code in codes {
        assert!(
            normalized.contains(code),
            "missing closed normalization error context: {code}"
        );
    }
    for forbidden in [
        "317250000",
        "123456789",
        "0.01437",
        "garde::",
        "validation report",
    ] {
        assert!(!display.contains(forbidden) && !debug.contains(forbidden));
    }
}

/// Construct a fresh component without requiring an additional public `Copy` trait.
type ComponentCase = (
    fn() -> NormalizationComponent,
    ArithmeticStage,
    &'static str,
);
/// Construct a fresh step without requiring an additional public `Copy` trait.
type StepCase = (fn() -> NormalizationStep, ArithmeticOperation, &'static str);

/// Independent component-to-existing-stage cases, with no product call.
fn component_cases() -> [ComponentCase; 2] {
    [
        (
            || NormalizationComponent::Time,
            ArithmeticStage::Geometry,
            "time",
        ),
        (
            || NormalizationComponent::Phase,
            ArithmeticStage::Response,
            "phase",
        ),
    ]
}

/// Independent step-to-operation cases, with no product call.
fn step_cases() -> [StepCase; 3] {
    [
        (
            || NormalizationStep::IntegerHighProduct,
            ArithmeticOperation::Multiply,
            "integerhighproduct",
        ),
        (
            || NormalizationStep::IntegerLowSum,
            ArithmeticOperation::Add,
            "integerlowsum",
        ),
        (
            || NormalizationStep::SecondsDivide,
            ArithmeticOperation::Divide,
            "secondsdivide",
        ),
    ]
}

/// Compare only enum variant identity without an unadvertised `Eq`/`Copy` requirement.
fn same_variant<T>(actual: &T, expected: &T) -> bool {
    mem::discriminant(actual) == mem::discriminant(expected)
}

/// Check every observation error's typed formatting and absent source.
/// # Panics
/// Requires useful phase/divisor/validation context, bounded strings and no source.
/// # Resources
/// Five manual variants only; no public product call.
fn inspect_manual_observation_errors() {
    for (error, codes) in [
        (ObservationError::PhaseOutOfRange, &["phase", "range"][..]),
        (
            ObservationError::Divisor {
                issue: PointIssue::NonFinite,
            },
            &["divisor", "nonfinite"][..],
        ),
        (
            ObservationError::Divisor {
                issue: PointIssue::Subnormal,
            },
            &["divisor", "subnormal"][..],
        ),
        (
            ObservationError::Divisor {
                issue: PointIssue::OutOfRange,
            },
            &["divisor", "range"][..],
        ),
        (
            ObservationError::ValidationInvariant,
            &["validation", "invariant"][..],
        ),
    ] {
        assert_error_context(&error, codes);
        assert!(error.source().is_none());
    }
}

/// Check closed structural/conversion errors with arbitrary manual safe indices.
/// # Panics
/// Requires bounded useful context and no fabricated nested source.
/// # Resources
/// Nine manual variants only; no public product call.
fn inspect_manual_closed_errors() {
    for (error, codes) in [
        (NormalizationError::CountOutOfRange, &["count"][..]),
        (
            NormalizationError::ContextMismatch { index: usize::MAX },
            &["context", "mismatch"][..],
        ),
        (
            NormalizationError::NonIncreasingTime { index: usize::MAX },
            &["increasing"][..],
        ),
        (
            NormalizationError::ElapsedRange { index: usize::MAX },
            &["elapsed"][..],
        ),
        (NormalizationError::SpanTooShort, &["span"][..]),
    ] {
        assert_error_context(&error, codes);
        assert!(error.source().is_none());
    }
    for (make_component, _, component_code) in component_cases() {
        let range = NormalizationError::IntegerRange {
            component: make_component(),
        };
        assert_error_context(&range, &[component_code, "integer", "range"]);
        assert!(range.source().is_none());
        let invariant = NormalizationError::ConversionInvariant {
            component: make_component(),
        };
        assert_error_context(&invariant, &[component_code, "conversion", "invariant"]);
        assert!(invariant.source().is_none());
    }
}

/// Check all component/step contexts with actual and atypical public typed causes.
/// # Panics
/// Requires precise preserved `FitError` sources and useful bounded context.
/// # Resources
/// Twelve manual combinations only; no public product call.
fn inspect_manual_arithmetic_sources() {
    for (make_component, stage, component_code) in component_cases() {
        for (make_step, operation, step_code) in step_cases() {
            let error = NormalizationError::Arithmetic {
                component: make_component(),
                step: make_step(),
                source: FitError::Arithmetic {
                    stage,
                    operation,
                    issue: ArithmeticIssue::Subnormal,
                },
            };
            assert_error_context(&error, &[component_code, step_code, "subnormal"]);
            let source = error
                .source()
                .and_then(|source| source.downcast_ref::<FitError>());
            assert!(
                matches!(source, Some(FitError::Arithmetic { stage: actual_stage, operation: actual_operation, issue: ArithmeticIssue::Subnormal })
                if same_variant(actual_stage, &stage) && same_variant(actual_operation, &operation))
            );
            let atypical = NormalizationError::Arithmetic {
                component: make_component(),
                step: make_step(),
                source: FitError::CountOutOfRange,
            };
            assert_error_context(&atypical, &[component_code, step_code, "count"]);
            assert!(matches!(
                atypical
                    .source()
                    .and_then(|source| source.downcast_ref::<FitError>()),
                Some(FitError::CountOutOfRange)
            ));
        }
    }
}

/// Check exact domain/point/input source types even under arbitrary manual indices.
/// # Panics
/// Requires total bounded formatting and the original typed nested cause.
/// # Resources
/// Three manual variants only; no public product call.
fn inspect_manual_indexed_sources() {
    let domain = NormalizationError::Domain {
        index: usize::MAX,
        source: DomainError::DurationOutOfOrder,
    };
    assert_error_context(&domain, &["domain", "duration"]);
    assert!(matches!(
        domain
            .source()
            .and_then(|source| source.downcast_ref::<DomainError>()),
        Some(DomainError::DurationOutOfOrder)
    ));
    let point = NormalizationError::Point {
        index: usize::MAX,
        source: FitError::Point {
            field: PointField::PhaseSeconds,
            issue: PointIssue::OutOfRange,
        },
    };
    assert_error_context(&point, &["point", "phase", "range"]);
    assert!(matches!(
        point
            .source()
            .and_then(|source| source.downcast_ref::<FitError>()),
        Some(FitError::Point {
            field: PointField::PhaseSeconds,
            issue: PointIssue::OutOfRange
        })
    ));
    let input = NormalizationError::Input(FitError::InsufficientSpread);
    assert_error_context(&input, &["input", "spread"]);
    assert!(matches!(
        input
            .source()
            .and_then(|source| source.downcast_ref::<FitError>()),
        Some(FitError::InsufficientSpread)
    ));
}

/// Every closed manual error remains bounded/total and preserves actual typed causes.
/// # Panics
/// Rejects missing/wrong sources or lost component/step/index context.
/// # Resources
/// Zero normalization/setup/getter/statistics/point calls; one complete zero record.
#[test]
fn manual_errors_preserve_bounded_total_context_and_typed_sources() {
    let _profile = Profile::new();
    inspect_manual_observation_errors();
    inspect_manual_closed_errors();
    inspect_manual_arithmetic_sources();
    inspect_manual_indexed_sources();
}

// SPDX-License-Identifier: GPL-2.0-only
//! Independent external CHECK-7/8/14/25 fixture-projection consumers.
//!
//! All expected phases, rates, ages and distances below are inline analytic facts.
//! Actual history reports supply the consumer's input, never its expected center.
//! Every direct product constructor, append, projection and getter is precharged;
//! append-internal normalization/statistics and projection-internal checked work
//! belong to separately reserved source bounds, not these external observations.
//! Each named scope emits one complete prefix record, including during unwind.
//! These authored tests provide no execution, production-freshness or clock claim.
#![forbid(unsafe_code)]

use std::{
    cell::Cell,
    error::Error,
    io::{self, Write},
    num::NonZeroU64,
};

use time_core::{
    ArithmeticIssue, ArithmeticOperation, ArithmeticStage, FitError, FixtureAnchorProjection,
    FixtureContext, FixtureHistory, FixtureObservation, FixtureProjectionError,
    FixtureProjectionField, FixtureProjectionIssue, FixtureProjectionStep, FixtureProjectionTerms,
    HistoryError, NormalizationComponent, NormalizationError, project_fixture_anchor,
};
use time_domain::{
    ExchangeArithmetic, MonotonicNs, PhaseOffsetQ33, UtcCoordinateQ32, exchange_arithmetic,
};

/// Setup/counter failures remain distinct from a returned product rejection.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
/// Literal monotonic nanoseconds and exact phase numerator in units of 2^-33 s.
type Row = (u64, i128);
/// One full immutable observation view, without converting its exact phase.
type RowBits = ([u64; 3], u64, i128, u64);

/// Source-owned finite ceilings in the marker's exact closed field order.
/// Fields: history, append, observation, context, monotonic, UTC, exchange,
/// getter, term constructor, projection. No other public entry point is used.
type Recipe = [u64; 10];

/// Per-case actual external attempts, charged before the call without shared state.
struct Profile {
    /// Fixed allowlisted label; never populated from a product error or raw input.
    case: &'static str,
    /// Independently fixed maximum attempted public calls for this named scope.
    ceiling: Recipe,
    /// Ten disjoint counters in the recipe's documented order.
    counts: [Cell<u64>; 10],
}

impl Profile {
    /// Start one local all-zero scope; no product calls, effects or allocations.
    const fn new(case: &'static str, ceiling: Recipe) -> Self {
        Self {
            case,
            ceiling,
            counts: [const { Cell::new(0) }; 10],
        }
    }

    /// Charge one impending call; a rejected charge performs no product operation.
    ///
    /// # Errors
    /// Rejects an invalid counter slot, overflow or exhausted fixed case ceiling.
    fn charge(&self, slot: usize) -> TestResult {
        let counter = self
            .counts
            .get(slot)
            .ok_or_else(|| io::Error::other("projection counter slot"))?;
        let ceiling = self
            .ceiling
            .get(slot)
            .ok_or_else(|| io::Error::other("projection ceiling slot"))?;
        let next = counter
            .get()
            .checked_add(1)
            .ok_or_else(|| io::Error::other("projection counter overflow"))?;
        if next > *ceiling {
            return Err(io::Error::other("projection counter ceiling").into());
        }
        counter.set(next);
        Ok(())
    }

    /// Admit exact nonzero literal tags, then call the actual immutable owner.
    ///
    /// # Errors
    /// Rejects a zero test tag or exhausted context budget, without substitution.
    fn context(&self, raw: [u64; 3]) -> TestResult<FixtureContext> {
        let tag = |value| {
            NonZeroU64::new(value).ok_or_else(|| io::Error::other("zero projection fixture tag"))
        };
        let source = tag(raw[0])?;
        let frame = tag(raw[1])?;
        let generation = tag(raw[2])?;
        self.charge(3)?;
        Ok(FixtureContext::new(source, frame, generation))
    }

    /// Wrap exact fixture nanoseconds without reading an ambient clock.
    ///
    /// # Errors
    /// Rejects only an exhausted constructor budget before the actual call.
    fn monotonic(&self, raw: u64) -> TestResult<MonotonicNs> {
        self.charge(4)?;
        Ok(MonotonicNs::new(raw))
    }

    /// Call checked UTC admission for genuine public Q33 phase setup.
    ///
    /// # Errors
    /// Propagates the real domain or counter rejection, with no raw phase factory.
    fn coordinate(&self, raw: i128) -> TestResult<UtcCoordinateQ32> {
        self.charge(5)?;
        Ok(UtcCoordinateQ32::new(raw)?)
    }

    /// Invoke one actual inert exchange on four already admitted coordinates.
    ///
    /// # Errors
    /// Propagates its real checked domain rejection or exhausted exchange budget.
    fn exchange(&self, values: [UtcCoordinateQ32; 4]) -> TestResult<ExchangeArithmetic> {
        self.charge(6)?;
        Ok(exchange_arithmetic(
            values[0], values[1], values[2], values[3],
        )?)
    }

    /// Admit one actual checked observation, with an explicit unit weight.
    ///
    /// # Errors
    /// Propagates mandatory observation or counter rejection before any append.
    fn observation(
        &self,
        context: FixtureContext,
        at: MonotonicNs,
        phase: PhaseOffsetQ33,
    ) -> TestResult<FixtureObservation> {
        self.charge(2)?;
        Ok(FixtureObservation::new(context, at, phase, 1.0)?)
    }

    /// Construct one genuine empty history; no hidden report or fit is requested.
    ///
    /// # Errors
    /// Rejects an exhausted constructor budget before creating the history.
    fn history(&self, context: FixtureContext) -> TestResult<FixtureHistory> {
        self.charge(0)?;
        Ok(FixtureHistory::new(context))
    }

    /// Reach the actual sole history mutation, with its result left unchanged.
    ///
    /// # Errors
    /// The outer error is accounting; the inner error is the actual append result.
    fn append(
        &self,
        history: &mut FixtureHistory,
        row: FixtureObservation,
    ) -> TestResult<Result<(), HistoryError>> {
        self.charge(1)?;
        Ok(history.try_append(row))
    }

    /// Charge immediately before one product getter; closures contain one call.
    ///
    /// # Errors
    /// Rejects exhausted accounting before invoking the actual immutable getter.
    fn getter<T>(&self, getter: impl FnOnce() -> T) -> TestResult<T> {
        self.charge(7)?;
        Ok(getter())
    }

    /// Reach unavoidable raw term admission without pre-validating its arguments.
    ///
    /// # Errors
    /// The outer error is accounting; the inner error is actual mandatory admission.
    fn terms(
        &self,
        raw: [f64; 3],
    ) -> TestResult<Result<FixtureProjectionTerms, FixtureProjectionError>> {
        self.charge(8)?;
        Ok(FixtureProjectionTerms::new(raw[0], raw[1], raw[2]))
    }

    /// Invoke exactly one public projection against its real borrowed history.
    ///
    /// # Errors
    /// The outer error is accounting; the inner error preserves the consumer's stage.
    fn project(
        &self,
        history: &FixtureHistory,
        context: FixtureContext,
        index: usize,
        expected_anchor: MonotonicNs,
        at: MonotonicNs,
        terms: FixtureProjectionTerms,
    ) -> TestResult<Result<FixtureAnchorProjection, FixtureProjectionError>> {
        self.charge(9)?;
        Ok(project_fixture_anchor(
            history,
            context,
            index,
            expected_anchor,
            at,
            terms,
        ))
    }
}

impl Drop for Profile {
    /// One assembled best-effort record preserves partial prefixes on assertion unwind.
    /// A failed/missing sink remains an incomplete supervisor observation.
    fn drop(&mut self) {
        let c = &self.counts;
        let row = format!(
            "CORE_PROJECTION_PROFILE case={} histories={} appends={} observations={} contexts={} monotonic={} coordinates={} exchanges={} getters={} terms={} projections={}\n",
            self.case,
            c[0].get(),
            c[1].get(),
            c[2].get(),
            c[3].get(),
            c[4].get(),
            c[5].get(),
            c[6].get(),
            c[7].get(),
            c[8].get(),
            c[9].get()
        );
        let _emission_result = io::stderr().lock().write_all(row.as_bytes());
    }
}

/// Read all three admitted context labels, with three precharged product getters.
///
/// # Errors
/// Propagates a getter-budget failure; standard nonzero reads are not product calls.
fn context_bits(profile: &Profile, context: FixtureContext) -> TestResult<[u64; 3]> {
    Ok([
        profile.getter(|| context.source_id())?.get(),
        profile.getter(|| context.frame())?.get(),
        profile.getter(|| context.generation())?.get(),
    ])
}

/// Obtain exact phase through legal public UTC/exchange construction, then verify it.
///
/// # Errors
/// Preserves domain/accounting failure; never forges a private checked phase.
///
/// # Panics
/// The actual setup must retain the independently supplied exact Q33 numerator.
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
        "exact phase setup"
    );
    Ok(phase)
}

/// Populate one real history from at most 65 inline rows, retaining accepted inputs.
///
/// # Errors
/// Propagates every real setup/append failure; no skipped row or fallback fit.
fn make_history(
    profile: &Profile,
    context: FixtureContext,
    rows: &[Row],
) -> TestResult<FixtureHistory> {
    if rows.len() > 65 {
        return Err(io::Error::other("projection row recipe ceiling").into());
    }
    let mut history = profile.history(context)?;
    for &(at, numerator) in rows {
        let at = profile.monotonic(at)?;
        let phase = phase(profile, numerator)?;
        let row = profile.observation(context, at, phase)?;
        profile.append(&mut history, row)??;
    }
    Ok(history)
}

/// Complete actual immutable report view, used only to detect history mutation.
#[derive(Debug, PartialEq)]
struct ReportBits {
    /// Actual complete report assertion.
    context: [u64; 3],
    /// Exact committed fit-reference nanoseconds.
    reference: u64,
    /// Actual statistics frame tag.
    frame: u64,
    /// Actual statistics input count.
    sample_count: usize,
    /// Actual statistics degrees of freedom.
    dof: usize,
    /// Actual contained line's frame tag.
    line_frame: u64,
    /// Phase, rate, SSE, residual scale, normalized variance, each as raw bits.
    scalars: [u64; 5],
}

/// Complete public history state before/after consumer calls, including row bits.
#[derive(Debug, PartialEq)]
struct HistoryBits {
    /// Actual immutable history assertion.
    context: [u64; 3],
    /// Actual committed prefix count.
    count: usize,
    /// Exact newest accepted time, when present.
    reference: Option<u64>,
    /// Complete visible prefix, preserving each raw observation bit.
    rows: Vec<RowBits>,
    /// Entire committed report view, when sufficient input exists.
    report: Option<ReportBits>,
}

/// Inspect all observable rows/report without computing a replacement expected fit.
///
/// # Errors
/// Rejects unexpected missing rows, a count above the fixed capacity or accounting.
///
/// # Panics
/// Both count and maximum indices must be hidden outside the committed prefix.
///
/// # Resources
/// At most 64 rows; 27+10n getters with a report, 10+10n without, 9 when empty.
fn history_bits(profile: &Profile, history: &FixtureHistory) -> TestResult<HistoryBits> {
    let context = profile.getter(|| history.context())?;
    let context = context_bits(profile, context)?;
    let count = profile.getter(|| history.sample_count())?;
    if count > 64 {
        return Err(io::Error::other("projection history capacity").into());
    }
    let reference = profile.getter(|| history.reference())?;
    let reference = reference
        .map(|value| profile.getter(|| value.nanoseconds()))
        .transpose()?;
    let report = profile.getter(|| history.report())?;
    let report = if let Some(report) = report {
        let context = profile.getter(|| report.context())?;
        let reference = profile.getter(|| report.reference())?;
        let statistics = profile.getter(|| report.statistics())?;
        let line = profile.getter(|| statistics.line())?;
        Some(ReportBits {
            context: context_bits(profile, context)?,
            reference: profile.getter(|| reference.nanoseconds())?,
            frame: profile.getter(|| statistics.frame())?.get(),
            sample_count: profile.getter(|| statistics.sample_count())?,
            dof: profile.getter(|| statistics.degrees_of_freedom())?,
            line_frame: profile.getter(|| line.frame())?.get(),
            scalars: [
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
            ],
        })
    } else {
        None
    };
    let mut rows = Vec::with_capacity(count);
    for index in 0..count {
        let row = profile
            .getter(|| history.observation(index))?
            .ok_or_else(|| io::Error::other("missing projection fixture row"))?;
        let context = profile.getter(|| row.context())?;
        let at = profile.getter(|| row.observed_at())?;
        let phase = profile.getter(|| row.phase())?;
        rows.push((
            context_bits(profile, context)?,
            profile.getter(|| at.nanoseconds())?,
            profile.getter(|| phase.numerator())?,
            profile.getter(|| row.variance_divisor())?.to_bits(),
        ));
    }
    assert!(profile.getter(|| history.observation(count))?.is_none());
    assert!(
        profile
            .getter(|| history.observation(usize::MAX))?
            .is_none()
    );
    Ok(HistoryBits {
        context,
        count,
        reference,
        rows,
        report,
    })
}

/// Obtain all three raw supplied bits through three actual immutable getters.
///
/// # Errors
/// Propagates only a getter-accounting failure, without arithmetic or validation.
fn term_bits(profile: &Profile, terms: FixtureProjectionTerms) -> TestResult<[u64; 3]> {
    Ok([
        profile.getter(|| terms.half_delay_seconds())?.to_bits(),
        profile.getter(|| terms.dispersion_seconds())?.to_bits(),
        profile
            .getter(|| terms.skew_seconds_per_second())?
            .to_bits(),
    ])
}

/// Owned scalar observation; independently literal facts below establish correctness.
#[derive(Debug, PartialEq)]
struct ProjectionBits {
    /// Actual complete borrowed history assertion.
    context: [u64; 3],
    /// Actual selected original observation time.
    anchor: u64,
    /// Actual newest accepted observation time.
    latest: u64,
    /// Actual committed fit-reference time.
    fit_reference: u64,
    /// Requested projection instant, without refreshing any observation.
    at: u64,
    /// Exact elapsed integer nanoseconds since the selected anchor.
    anchor_ns: u64,
    /// Exact elapsed integer nanoseconds since the latest accepted observation.
    latest_ns: u64,
    /// Positive anchor age in seconds, observed as its raw binary64 bits.
    elapsed_seconds: u64,
    /// Projected anchor-phase center in seconds, as raw binary64 bits.
    center: u64,
    /// Same-history fitted phase derivative in seconds per second, as raw bits.
    rate: u64,
    /// Supplied half-delay, dispersion and skew bits, kept separate from growth.
    terms: [u64; 3],
    /// Computed skew growth seconds, as raw binary64 bits.
    growth: u64,
    /// Total numeric distance seconds, as raw binary64 bits.
    distance: u64,
}

/// Read every immutable output field once with exactly 25 actual product getters.
///
/// # Errors
/// Propagates only accounting failure; all nested getters are separately precharged.
fn projection_bits(
    profile: &Profile,
    value: &FixtureAnchorProjection,
) -> TestResult<ProjectionBits> {
    let context = profile.getter(|| value.context())?;
    let anchor = profile.getter(|| value.anchor_observed_at())?;
    let latest = profile.getter(|| value.latest_observed_at())?;
    let reference = profile.getter(|| value.fit_reference())?;
    let at = profile.getter(|| value.projected_at())?;
    let anchor_elapsed = profile.getter(|| value.anchor_elapsed())?;
    let latest_elapsed = profile.getter(|| value.latest_elapsed())?;
    let terms = profile.getter(|| value.terms())?;
    Ok(ProjectionBits {
        context: context_bits(profile, context)?,
        anchor: profile.getter(|| anchor.nanoseconds())?,
        latest: profile.getter(|| latest.nanoseconds())?,
        fit_reference: profile.getter(|| reference.nanoseconds())?,
        at: profile.getter(|| at.nanoseconds())?,
        anchor_ns: profile.getter(|| anchor_elapsed.nanoseconds())?,
        latest_ns: profile.getter(|| latest_elapsed.nanoseconds())?,
        elapsed_seconds: profile.getter(|| value.anchor_elapsed_seconds())?.to_bits(),
        center: profile.getter(|| value.center_seconds())?.to_bits(),
        rate: profile
            .getter(|| value.phase_rate_seconds_per_second())?
            .to_bits(),
        terms: term_bits(profile, terms)?,
        growth: profile.getter(|| value.growth_seconds())?.to_bits(),
        distance: profile.getter(|| value.distance_seconds())?.to_bits(),
    })
}

/// Reject with the exact owning stage, and retain only bounded diagnostic metadata.
///
/// # Errors
/// Returns a harness error if the intended product rejection unexpectedly succeeds.
///
/// # Panics
/// Expected typed error and bounded redacted Debug/Display must both hold.
fn rejected<T>(
    result: Result<T, FixtureProjectionError>,
    expected: FixtureProjectionError,
) -> TestResult {
    let actual = result
        .err()
        .ok_or_else(|| io::Error::other("projection unexpectedly accepted"))?;
    assert_eq!(actual, expected, "independent projection rejection stage");
    let display = actual.to_string();
    let debug = format!("{actual:?}");
    assert!(
        display.len() <= 160 && debug.len() <= 256,
        "bounded projection rejection"
    );
    for forbidden in [
        "179769313486",
        "2.225073858",
        "900719925474",
        "184467440737",
    ] {
        assert!(
            !display.contains(forbidden) && !debug.contains(forbidden),
            "raw operand in error"
        );
    }
    Ok(())
}

/// Literal response-stage arithmetic expectation, without invoking the numerical owner.
const fn arithmetic_error(
    step: FixtureProjectionStep,
    operation: ArithmeticOperation,
    issue: ArithmeticIssue,
) -> FixtureProjectionError {
    FixtureProjectionError::Arithmetic {
        step,
        source: FitError::Arithmetic {
            stage: ArithmeticStage::Response,
            operation,
            issue,
        },
    }
}

/// Exact Q33 numerators: 3/32, 7/64, 1/8 seconds, and their exact monotonic times.
const LINEAR: [Row; 3] = [
    (98_000_000_000, 805_306_368),
    (99_000_000_000, 939_524_096),
    (100_000_000_000, 1_073_741_824),
];
/// All histories in a scope share this assertion; no authentication is claimed.
const CONTEXT: [u64; 3] = [41, 43, 47];
/// Half-delay/dispersion in seconds and skew in seconds per second.
const TERMS: [f64; 3] = [1.0 / 64.0, 1.0 / 64.0, 1.0 / 128.0];

/// Real raw term admission must preserve zeros and reject invalid neighboring fields.
///
/// # Errors
/// Propagates accounting or valid-control admission failures and unexpected acceptance.
///
/// # Panics
/// Raw bits, declared field-order rejection and bounded redacted diagnostics must match.
#[test]
fn terms_admit_signed_zeros_and_reject_each_raw_invalid_field_in_order() -> TestResult {
    let profile = Profile::new("terms_admission", [0, 0, 0, 0, 0, 0, 0, 36, 48, 0]);
    // Both zero signs in every field survive construction independently.
    for mask in 0_u8..8 {
        let raw = [
            if mask & 1 == 0 { 0.0 } else { -0.0 },
            if mask & 2 == 0 { 0.0 } else { -0.0 },
            if mask & 4 == 0 { 0.0 } else { -0.0 },
        ];
        let terms = profile.terms(raw)??;
        assert_eq!(term_bits(&profile, terms)?, raw.map(f64::to_bits));
    }
    for index in 0..3 {
        let mut raw = [0.0; 3];
        raw[index] = f64::MIN_POSITIVE;
        let terms = profile.terms(raw)??;
        assert_eq!(term_bits(&profile, terms)?, raw.map(f64::to_bits));
    }
    let maximum = profile.terms([f64::MAX; 3])??;
    assert_eq!(term_bits(&profile, maximum)?, [f64::MAX.to_bits(); 3]);
    let debug = format!("{maximum:?}");
    assert!(debug.len() <= 96 && !debug.contains("1.797693") && !debug.contains("179769313"));
    let invalid = [
        (0x7ff8_0000_0000_0041, FixtureProjectionIssue::NonFinite),
        (0xfff8_0000_0000_0043, FixtureProjectionIssue::NonFinite),
        (0x7ff0_0000_0000_0000, FixtureProjectionIssue::NonFinite),
        (0xfff0_0000_0000_0000, FixtureProjectionIssue::NonFinite),
        (0x0000_0000_0000_0001, FixtureProjectionIssue::Subnormal),
        (0x8000_0000_0000_0001, FixtureProjectionIssue::Subnormal),
        (0x000f_ffff_ffff_ffff, FixtureProjectionIssue::Subnormal),
        (0x800f_ffff_ffff_ffff, FixtureProjectionIssue::Subnormal),
        (0x8010_0000_0000_0000, FixtureProjectionIssue::Negative),
        (0xbff0_0000_0000_0000, FixtureProjectionIssue::Negative),
        (0xffef_ffff_ffff_ffff, FixtureProjectionIssue::Negative),
    ];
    let fields = [
        FixtureProjectionField::HalfDelaySeconds,
        FixtureProjectionField::DispersionSeconds,
        FixtureProjectionField::SkewSecondsPerSecond,
    ];
    for (index, field) in fields.into_iter().enumerate() {
        for (bits, issue) in invalid {
            let mut raw = TERMS;
            raw[index] = f64::from_bits(bits);
            rejected(
                profile.terms(raw)?,
                FixtureProjectionError::InvalidTerm { field, issue },
            )?;
        }
    }
    // Simultaneously invalid neighboring fields must retain declared field order.
    for (raw, field, issue) in [
        (
            [f64::NAN, -1.0, f64::INFINITY],
            FixtureProjectionField::HalfDelaySeconds,
            FixtureProjectionIssue::NonFinite,
        ),
        (
            [0.0, f64::from_bits(1), -1.0],
            FixtureProjectionField::DispersionSeconds,
            FixtureProjectionIssue::Subnormal,
        ),
        (
            [0.0, 0.0, -1.0],
            FixtureProjectionField::SkewSecondsPerSecond,
            FixtureProjectionIssue::Negative,
        ),
    ] {
        rejected(
            profile.terms(raw)?,
            FixtureProjectionError::InvalidTerm { field, issue },
        )?;
    }
    Ok(())
}

/// Supplied zero signs survive the real consumer and its separate IEEE operations.
///
/// # Errors
/// Propagates actual history, term, projection or accounting failures in valid controls.
///
/// # Panics
/// Supplied bits, exact ages, separate zero arithmetic and unchanged history must match.
#[test]
fn signed_zero_terms_reach_projection_without_clamping_or_losing_supplied_bits() -> TestResult {
    let profile = Profile::new(
        "signed_zero_consumption",
        [1, 3, 3, 1, 5, 12, 3, 520, 8, 16],
    );
    let context = profile.context(CONTEXT)?;
    let history = make_history(
        &profile,
        context,
        &[
            (98_000_000_000, 1_073_741_824),
            (99_000_000_000, 1_073_741_824),
            (100_000_000_000, 1_073_741_824),
        ],
    )?;
    let before = history_bits(&profile, &history)?;
    let anchor = profile.monotonic(100_000_000_000)?;
    let later = profile.monotonic(101_000_000_000)?;
    for mask in 0_u8..8 {
        let raw = [
            if mask & 1 == 0 { 0.0 } else { -0.0 },
            if mask & 2 == 0 { 0.0 } else { -0.0 },
            if mask & 4 == 0 { 0.0 } else { -0.0 },
        ];
        let terms = profile.terms(raw)??;
        for (at, age_ns, age_seconds) in [(anchor, 0, 0.0_f64), (later, 1_000_000_000, 1.0)] {
            let output = profile.project(&history, context, 2, anchor, at, terms)??;
            let actual = projection_bits(&profile, &output)?;
            assert_eq!(
                actual.terms,
                raw.map(f64::to_bits),
                "supplied zero bits survive consumer"
            );
            assert_eq!((actual.anchor_ns, actual.latest_ns), (age_ns, age_ns));
            assert_eq!(
                actual.elapsed_seconds,
                age_seconds.to_bits(),
                "integer zero converts positive"
            );
            let growth = if mask & 4 == 0 { 0.0_f64 } else { -0.0_f64 };
            let distance = if mask == 7 { -0.0_f64 } else { 0.0_f64 };
            assert_eq!(
                actual.growth,
                growth.to_bits(),
                "separate multiply retains zero sign"
            );
            assert_eq!(
                actual.distance,
                distance.to_bits(),
                "two separately rounded zero sums"
            );
            assert_eq!(actual.center, (1.0_f64 / 8.0).to_bits());
        }
    }
    assert_eq!(history_bits(&profile, &history)?, before);
    Ok(())
}

/// Inline analytic anchor time, projection time, anchor/latest ages and three scalars.
/// Times and ages are integer nanoseconds; center, growth and distance are seconds.
type LinearFact = (u64, u64, u64, u64, f64, f64, f64);

/// Check all actual output fields against one supplied independent analytic fact.
///
/// # Errors
/// Propagates getter accounting failure without constructing or projecting a value.
///
/// # Panics
/// All immutable fields and bounded redacted diagnostics must match the literal fact.
///
/// # Resources
/// Exactly 25 actual product getters, including the retained same-fit rate.
fn inspect_linear_projection(
    profile: &Profile,
    output: &FixtureAnchorProjection,
    fact: LinearFact,
) -> TestResult<ProjectionBits> {
    let (anchor_raw, at_raw, anchor_age, latest_age, center, growth, distance) = fact;
    let debug = format!("{output:?}");
    assert!(debug.len() <= 128);
    for forbidden in ["100000000000", "0.15625", "0.015625"] {
        assert!(
            !debug.contains(forbidden),
            "projection debug exposes numeric data"
        );
    }
    let actual = projection_bits(profile, output)?;
    let expected = ProjectionBits {
        context: CONTEXT,
        anchor: anchor_raw,
        latest: 100_000_000_000,
        fit_reference: 100_000_000_000,
        at: at_raw,
        anchor_ns: anchor_age,
        latest_ns: latest_age,
        elapsed_seconds: match anchor_age {
            0 => 0.0_f64.to_bits(),
            2_000_000_000 => 2.0_f64.to_bits(),
            _ => 4.0_f64.to_bits(),
        },
        center: center.to_bits(),
        rate: (1.0_f64 / 64.0).to_bits(),
        terms: TERMS.map(f64::to_bits),
        growth: growth.to_bits(),
        distance: distance.to_bits(),
    };
    assert_eq!(actual, expected, "independent exact linear projection");
    Ok(actual)
}

/// Append a genuine changed fit while an earlier owned projection stays intact.
///
/// # Errors
/// Preserve setup, append, projection and observation accounting failures.
///
/// # Panics
/// The new independent rate/anchor facts and the complete old snapshot must hold.
///
/// # Resources
/// One append at occupied length4, one projection and35 public getters.
fn retained_rate_after_append(
    profile: &Profile,
    history: &mut FixtureHistory,
    context: FixtureContext,
    terms: FixtureProjectionTerms,
    snapshot: (FixtureAnchorProjection, ProjectionBits),
) -> TestResult {
    let (previous, previous_bits) = snapshot;
    let at = profile.monotonic(102_000_000_000)?;
    let phase = phase(profile, 6_039_797_760)?;
    let row = profile.observation(context, at, phase)?;
    profile.append(history, row)??;
    // Times0,1,2,4 and phases(6,7,8,45)/64 have Sxx35/4 and
    // covariance175/128: rate5/32. H4 normalization stays exactly dyadic.
    let report = profile
        .getter(|| history.report())?
        .ok_or_else(|| io::Error::other("missing changed-fit report"))?;
    let statistics = profile.getter(|| report.statistics())?;
    let line = profile.getter(|| statistics.line())?;
    let actual_rate = profile.getter(|| line.phase_rate_seconds_per_second())?;
    assert_eq!(actual_rate.to_bits(), (5.0_f64 / 32.0).to_bits());
    assert_eq!(
        projection_bits(profile, &previous)?,
        previous_bits,
        "an owned projection keeps its original committed rate and metadata"
    );
    let current = profile.project(history, context, 3, at, at, terms)??;
    assert_eq!(
        profile
            .getter(|| current.phase_rate_seconds_per_second())?
            .to_bits(),
        actual_rate.to_bits(),
        "projection retains exactly the rate used by its same-history fit"
    );
    // Latest anchor phase45/64 differs from the new fitted intercept39/64.
    assert_eq!(
        profile.getter(|| current.center_seconds())?.to_bits(),
        (45.0_f64 / 64.0).to_bits()
    );
    assert_eq!(profile.getter(|| current.fit_reference())?, at);
    assert_eq!(profile.getter(|| current.latest_observed_at())?, at);
    Ok(())
}

/// Snapshot the original literal linear history and verify its independent rate.
///
/// # Errors
/// Preserve complete-history getter and accounting failures.
///
/// # Panics
/// The original three-row linear fixture must retain rate `1/64`.
fn original_linear_history_bits(
    profile: &Profile,
    history: &FixtureHistory,
) -> TestResult<HistoryBits> {
    let before = history_bits(profile, history)?;
    assert_eq!(
        before.report.as_ref().map(|r| r.scalars[1]),
        Some((1.0_f64 / 64.0).to_bits())
    );
    Ok(before)
}

/// Independent exact line facts challenge both anchor identity and repeated read behavior.
///
/// # Errors
/// Propagates actual setup, append, term, projection or accounting failures.
///
/// # Panics
/// Fitted rate, literal projection fields, repeated identity and unchanged history must match.
#[test]
fn linear_anchors_agree_and_repeated_projection_preserves_original_identity_and_age() -> TestResult
{
    let profile = Profile::new("linear_identity", [1, 4, 4, 1, 8, 16, 4, 280, 1, 6]);
    let context = profile.context(CONTEXT)?;
    let mut history = make_history(&profile, context, &LINEAR)?;
    let before = original_linear_history_bits(&profile, &history)?;
    let terms = profile.terms(TERMS)??;
    let oldest = profile.monotonic(98_000_000_000)?;
    let newest = profile.monotonic(100_000_000_000)?;
    let now = profile.monotonic(102_000_000_000)?;
    let later = profile.monotonic(104_000_000_000)?;
    let mut repeated = None;
    let mut retained_snapshot = None;
    for (index, anchor, at, anchor_raw, at_raw, anchor_age, latest_age, center, growth, distance) in [
        (
            2,
            newest,
            newest,
            100_000_000_000,
            100_000_000_000,
            0,
            0,
            1.0 / 8.0,
            0.0,
            2.0 / 64.0,
        ),
        (
            2,
            newest,
            now,
            100_000_000_000,
            102_000_000_000,
            2_000_000_000,
            2_000_000_000,
            5.0 / 32.0,
            1.0 / 64.0,
            3.0 / 64.0,
        ),
        (
            2,
            newest,
            now,
            100_000_000_000,
            102_000_000_000,
            2_000_000_000,
            2_000_000_000,
            5.0 / 32.0,
            1.0 / 64.0,
            3.0 / 64.0,
        ),
        (
            0,
            oldest,
            now,
            98_000_000_000,
            102_000_000_000,
            4_000_000_000,
            2_000_000_000,
            5.0 / 32.0,
            2.0 / 64.0,
            4.0 / 64.0,
        ),
        (
            2,
            newest,
            later,
            100_000_000_000,
            104_000_000_000,
            4_000_000_000,
            4_000_000_000,
            3.0 / 16.0,
            2.0 / 64.0,
            4.0 / 64.0,
        ),
    ] {
        let output = profile.project(&history, context, index, anchor, at, terms)??;
        let actual = inspect_linear_projection(
            &profile,
            &output,
            (
                anchor_raw, at_raw, anchor_age, latest_age, center, growth, distance,
            ),
        )?;
        if index == 2 && at_raw == 102_000_000_000 {
            if let Some(first) = &repeated {
                assert_eq!(&actual, first);
            }
            repeated = Some(actual);
        } else if at_raw == 104_000_000_000 {
            retained_snapshot = Some((output, actual));
        }
    }
    assert_eq!(
        history_bits(&profile, &history)?,
        before,
        "projection cannot refresh history"
    );
    let snapshot = retained_snapshot.ok_or_else(|| io::Error::other("missing owned projection"))?;
    retained_rate_after_append(&profile, &mut history, context, terms, snapshot)?;
    Ok(())
}

/// Nonlinear and descending phases distinguish anchor projection from a fitted intercept.
///
/// # Errors
/// Propagates actual setup, append, term, projection or accounting failures.
///
/// # Panics
/// The independent zero/negative rates, anchor centers, ages and unchanged history must match.
#[test]
fn nonlinear_anchor_is_not_fitted_intercept_and_negative_rate_keeps_its_sign() -> TestResult {
    let profile = Profile::new("shape_controls", [2, 6, 6, 1, 8, 24, 6, 290, 1, 2]);
    let context = profile.context(CONTEXT)?;
    let terms = profile.terms(TERMS)??;
    let anchor = profile.monotonic(100_000_000_000)?;
    let at = profile.monotonic(102_000_000_000)?;
    let nonlinear = [
        (98_000_000_000, 8_589_934_592),
        (99_000_000_000, 0),
        (100_000_000_000, 8_589_934_592),
    ];
    let negative = [
        (98_000_000_000, 1_073_741_824),
        (99_000_000_000, 939_524_096),
        (100_000_000_000, 805_306_368),
    ];
    for (rows, expected_rate, expected_center) in [
        (&nonlinear, 0.0_f64, 1.0_f64),
        (&negative, -1.0 / 64.0, 1.0 / 16.0),
    ] {
        let history = make_history(&profile, context, rows)?;
        let before = history_bits(&profile, &history)?;
        assert_eq!(
            before.report.as_ref().map(|r| r.scalars[1]),
            Some(expected_rate.to_bits())
        );
        let output = profile.project(&history, context, 2, anchor, at, terms)??;
        let actual = projection_bits(&profile, &output)?;
        assert_eq!(
            actual.rate,
            expected_rate.to_bits(),
            "retained positive-behind phase derivative, in seconds per second"
        );
        // [1,0,1] at [-2,-1,0] has mathematical intercept 2/3 and rate 0.
        // The independent expected anchor center is 1; no returned intercept is an oracle.
        assert_eq!(
            actual.center,
            expected_center.to_bits(),
            "actual anchor plus signed rate"
        );
        assert_eq!(actual.growth, (1.0_f64 / 64.0).to_bits());
        assert_eq!(actual.distance, (3.0_f64 / 64.0).to_bits());
        assert_eq!(actual.anchor_ns, 2_000_000_000);
        assert_eq!(actual.latest_ns, 2_000_000_000);
        assert_eq!(history_bits(&profile, &history)?, before);
    }
    Ok(())
}

/// Exact integer subtraction must precede conversion at both large timestamp origins.
///
/// # Errors
/// Propagates actual setup, append, term, projection or accounting failures.
///
/// # Panics
/// Original times, one-nanosecond ages, converted bits and unchanged history must match.
#[test]
fn large_monotonic_origins_retain_one_nanosecond_without_absolute_float_conversion() -> TestResult {
    let profile = Profile::new("origin_durations", [2, 6, 6, 1, 10, 24, 6, 290, 1, 2]);
    let context = profile.context(CONTEXT)?;
    let terms = profile.terms([1.0 / 64.0, 1.0 / 64.0, 0.0])??;
    for latest in [9_223_372_036_854_775_808_u64, 18_446_744_073_709_551_614] {
        let rows = [
            (latest - 2_000_000_000, 1_073_741_824),
            (latest - 1_000_000_000, 1_073_741_824),
            (latest, 1_073_741_824),
        ];
        let history = make_history(&profile, context, &rows)?;
        let before = history_bits(&profile, &history)?;
        let anchor = profile.monotonic(latest)?;
        let at = profile.monotonic(latest + 1)?;
        let output = profile.project(&history, context, 2, anchor, at, terms)??;
        let actual = projection_bits(&profile, &output)?;
        assert_eq!(actual.anchor, latest);
        assert_eq!(actual.latest, latest);
        assert_eq!(actual.fit_reference, latest);
        assert_eq!(actual.at, latest + 1);
        assert_eq!((actual.anchor_ns, actual.latest_ns), (1, 1));
        assert_eq!(actual.elapsed_seconds, 0.000_000_001_f64.to_bits());
        assert_eq!(actual.center, (1.0_f64 / 8.0).to_bits());
        assert_eq!(actual.growth, 0.0_f64.to_bits());
        assert_eq!(actual.distance, (1.0_f64 / 32.0).to_bits());
        assert_eq!(history_bits(&profile, &history)?, before);
    }
    Ok(())
}

/// Inclusive integer conversion bounds supply no synthetic observation lifetime.
///
/// # Errors
/// Propagates valid-control admission/accounting failures or unexpected over-limit acceptance.
///
/// # Panics
/// Exact duration neighbors, independent converted bits, rejection stage and history must match.
#[test]
fn duration_conversion_accepts_two_to_53_and_rejects_only_its_next_integer() -> TestResult {
    let profile = Profile::new("integer_envelope", [1, 3, 3, 1, 7, 12, 3, 170, 1, 3]);
    let context = profile.context(CONTEXT)?;
    let history = make_history(
        &profile,
        context,
        &[
            (0, 1_073_741_824),
            (1_000_000_000, 1_073_741_824),
            (2_000_000_000, 1_073_741_824),
        ],
    )?;
    let before = history_bits(&profile, &history)?;
    let anchor = profile.monotonic(2_000_000_000)?;
    let terms = profile.terms([1.0 / 64.0, 1.0 / 64.0, 0.0])??;
    // Independent exact rational rounding: exponent 23, significand obtained by
    // nearest-even integer division of duration * 2^29 by 10^9. These literal
    // adjacent binary64 results distinguish both inclusive integer endpoints.
    for (duration, expected_seconds) in [
        (9_007_199_254_740_991_u64, Some(0x4161_2e0b_e826_d694_u64)),
        (9_007_199_254_740_992, Some(0x4161_2e0b_e826_d695)),
        (9_007_199_254_740_993, None),
    ] {
        let at = profile.monotonic(2_000_000_000 + duration)?;
        let result = profile.project(&history, context, 2, anchor, at, terms)?;
        if let Some(expected_seconds) = expected_seconds {
            let actual = projection_bits(&profile, &result?)?;
            assert_eq!((actual.anchor_ns, actual.latest_ns), (duration, duration));
            assert_eq!(actual.elapsed_seconds, expected_seconds);
            assert_eq!(actual.latest, 2_000_000_000);
            assert_eq!(actual.center, (1.0_f64 / 8.0).to_bits());
            assert_eq!(actual.distance, (1.0_f64 / 32.0).to_bits());
        } else {
            rejected(
                result,
                FixtureProjectionError::Conversion {
                    source: NormalizationError::IntegerRange {
                        component: NormalizationComponent::Time,
                    },
                },
            )?;
        }
    }
    assert_eq!(history_bits(&profile, &history)?, before);
    Ok(())
}

/// Simultaneous invalid relationships must reject in the declared receiving order.
///
/// # Errors
/// Propagates setup/accounting failures or unexpected acceptance of a declared rejection.
///
/// # Panics
/// Ordered typed errors, bounded diagnostics, the healthy center and all history views must match.
#[test]
fn relationship_errors_precede_numeric_work_and_preserve_every_history_view() -> TestResult {
    let profile = Profile::new("error_precedence", [3, 4, 4, 4, 9, 16, 4, 205, 2, 12]);
    let context = profile.context(CONTEXT)?;
    let wrong = [
        profile.context([42, 43, 47])?,
        profile.context([41, 44, 47])?,
        profile.context([41, 43, 48])?,
    ];
    let empty = make_history(&profile, context, &[])?;
    let single = make_history(&profile, context, &[(100_000_000_000, 1_073_741_824)])?;
    let full = make_history(&profile, context, &LINEAR)?;
    let empty_before = history_bits(&profile, &empty)?;
    let single_before = history_bits(&profile, &single)?;
    let full_before = history_bits(&profile, &full)?;
    let zero = profile.monotonic(0)?;
    let oldest = profile.monotonic(98_000_000_000)?;
    let earlier = profile.monotonic(99_000_000_000)?;
    let latest = profile.monotonic(100_000_000_000)?;
    let at = profile.monotonic(102_000_000_000)?;
    // These valid terms would overflow numerically if preceding guards were bypassed.
    let overflow = profile.terms([f64::MAX, f64::MAX, 0.0])??;
    for wrong_context in wrong {
        rejected(
            profile.project(&empty, wrong_context, usize::MAX, latest, zero, overflow)?,
            FixtureProjectionError::ContextMismatch,
        )?;
    }
    for history in [&empty, &single] {
        rejected(
            profile.project(history, context, usize::MAX, zero, zero, overflow)?,
            FixtureProjectionError::MissingReport,
        )?;
    }
    for index in [3, usize::MAX] {
        rejected(
            profile.project(&full, context, index, zero, zero, overflow)?,
            FixtureProjectionError::MissingAnchor,
        )?;
    }
    rejected(
        profile.project(&full, context, 2, earlier, zero, overflow)?,
        FixtureProjectionError::AnchorMismatch,
    )?;
    for (index, anchor) in [(0, oldest), (2, latest)] {
        rejected(
            profile.project(&full, context, index, anchor, earlier, overflow)?,
            FixtureProjectionError::BackwardProjection,
        )?;
    }
    rejected(
        profile.project(&full, context, 2, latest, at, overflow)?,
        arithmetic_error(
            FixtureProjectionStep::BaseDistance,
            ArithmeticOperation::Add,
            ArithmeticIssue::NonFinite,
        ),
    )?;
    let healthy = profile.terms(TERMS)??;
    let output = profile.project(&full, context, 2, latest, at, healthy)??;
    assert_eq!(
        projection_bits(&profile, &output)?.center,
        (5.0_f64 / 32.0).to_bits()
    );
    assert_eq!(history_bits(&profile, &empty)?, empty_before);
    assert_eq!(history_bits(&profile, &single)?, single_before);
    assert_eq!(history_bits(&profile, &full)?, full_before);
    Ok(())
}

/// An exact original anchor time prevents stale-index retargeting after capacity eviction.
///
/// # Errors
/// Propagates setup/accounting failures, valid-control rejection or stale-anchor acceptance.
///
/// # Panics
/// Eviction facts, rejection identity, projected fields and unchanged history must match.
#[test]
fn eviction_cannot_retarget_a_stale_index_to_a_different_observation() -> TestResult {
    let profile = Profile::new("evicted_anchor", [1, 65, 65, 1, 68, 260, 65, 1489, 1, 2]);
    let context = profile.context(CONTEXT)?;
    let rows: Vec<Row> = (1_u64..=65)
        .map(|second| (second * 1_000_000_000, 1_073_741_824))
        .collect();
    let history = make_history(&profile, context, &rows)?;
    let before = history_bits(&profile, &history)?;
    assert_eq!(before.count, 64);
    assert_eq!(before.rows.first().map(|row| row.1), Some(2_000_000_000));
    assert_eq!(before.reference, Some(65_000_000_000));
    let old = profile.monotonic(1_000_000_000)?;
    let current = profile.monotonic(2_000_000_000)?;
    let at = profile.monotonic(66_000_000_000)?;
    let terms = profile.terms(TERMS)??;
    rejected(
        profile.project(&history, context, 0, old, at, terms)?,
        FixtureProjectionError::AnchorMismatch,
    )?;
    let output = profile.project(&history, context, 0, current, at, terms)??;
    let actual = projection_bits(&profile, &output)?;
    assert_eq!(actual.anchor, 2_000_000_000);
    assert_eq!(actual.latest, 65_000_000_000);
    assert_eq!(
        (actual.anchor_ns, actual.latest_ns),
        (64_000_000_000, 1_000_000_000)
    );
    assert_eq!(actual.center, (1.0_f64 / 8.0).to_bits());
    assert_eq!(actual.growth, 0.5_f64.to_bits());
    assert_eq!(actual.distance, (17.0_f64 / 32.0).to_bits());
    assert_eq!(history_bits(&profile, &history)?, before);
    Ok(())
}

/// Each arithmetic step must reject invalid intermediates before a later sum can hide them.
///
/// # Errors
/// Propagates setup/accounting failures, valid-control rejection or invalid-result acceptance.
///
/// # Panics
/// Exact arithmetic errors, healthy neighboring outputs and complete preserved history must match.
#[test]
fn separately_checked_growth_and_sums_reject_hidden_subnormal_and_overflow() -> TestResult {
    let profile = Profile::new("numeric_failures", [1, 3, 3, 1, 7, 12, 3, 170, 5, 6]);
    let context = profile.context(CONTEXT)?;
    let history = make_history(
        &profile,
        context,
        &[
            (98_000_000_000, 1_073_741_824),
            (99_000_000_000, 1_073_741_824),
            (100_000_000_000, 1_073_741_824),
        ],
    )?;
    let before = history_bits(&profile, &history)?;
    let anchor = profile.monotonic(100_000_000_000)?;
    let nanosecond = profile.monotonic(100_000_000_001)?;
    let second = profile.monotonic(101_000_000_000)?;
    let two_seconds = profile.monotonic(102_000_000_000)?;
    let tiny = profile.terms([1.0 / 64.0, 1.0 / 64.0, f64::MIN_POSITIVE])??;
    rejected(
        profile.project(&history, context, 2, anchor, nanosecond, tiny)?,
        arithmetic_error(
            FixtureProjectionStep::Growth,
            ArithmeticOperation::Multiply,
            ArithmeticIssue::Subnormal,
        ),
    )?;
    let healthy_tiny = profile.project(&history, context, 2, anchor, second, tiny)??;
    let actual = projection_bits(&profile, &healthy_tiny)?;
    assert_eq!(
        actual.growth,
        f64::MIN_POSITIVE.to_bits(),
        "one-second minimum-normal neighbor"
    );
    assert_eq!(actual.distance, (1.0_f64 / 32.0).to_bits());
    let base_overflow = profile.terms([f64::MAX, f64::MAX, 0.0])??;
    rejected(
        profile.project(&history, context, 2, anchor, second, base_overflow)?,
        arithmetic_error(
            FixtureProjectionStep::BaseDistance,
            ArithmeticOperation::Add,
            ArithmeticIssue::NonFinite,
        ),
    )?;
    let base_neighbor = profile.terms([f64::MAX / 2.0, f64::MAX / 2.0, 0.0])??;
    let healthy_base = profile.project(&history, context, 2, anchor, second, base_neighbor)??;
    assert_eq!(
        projection_bits(&profile, &healthy_base)?.distance,
        f64::MAX.to_bits()
    );
    let growth_overflow = profile.terms([0.0, 0.0, f64::MAX])??;
    rejected(
        profile.project(&history, context, 2, anchor, two_seconds, growth_overflow)?,
        arithmetic_error(
            FixtureProjectionStep::Growth,
            ArithmeticOperation::Multiply,
            ArithmeticIssue::NonFinite,
        ),
    )?;
    let distance_overflow = profile.terms([f64::MAX / 2.0, 0.0, f64::MAX / 2.0])??;
    rejected(
        profile.project(&history, context, 2, anchor, two_seconds, distance_overflow)?,
        arithmetic_error(
            FixtureProjectionStep::Distance,
            ArithmeticOperation::Add,
            ArithmeticIssue::NonFinite,
        ),
    )?;
    assert_eq!(
        history_bits(&profile, &history)?,
        before,
        "numeric failures preserve all accepted state"
    );
    Ok(())
}

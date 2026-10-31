// SPDX-License-Identifier: GPL-2.0-only
//! Independent external REQ-10/DEC-8 and CHECK-7/8/14/25 Q33 enclosure facts.
//!
//! Real checked UTC/exchange phases feed real observations and history before
//! projection and enclosure. Expected endpoints are inline exact dyadic facts;
//! neither the fit nor the bridge computes its own oracle. Every direct product
//! constructor, append, projection, enclosure and getter is precharged. Internal
//! checked work has separate source-owned bounds, not external-attempt counts.
//! Each named scope emits its complete attempted prefix, including on unwind.
//! These fixture tests claim no physical confidence, freshness or clock effect.
#![forbid(unsafe_code)]

use std::{
    cell::Cell,
    error::Error,
    io::{self, Write},
    num::NonZeroU64,
};

use time_core::{
    FixtureAnchorProjection, FixtureContext, FixtureEnclosureError, FixtureEndpointRounding,
    FixtureHistory, FixtureObservation, FixtureProjectedInterval, FixtureProjectionError,
    FixtureProjectionTerms, HistoryError, enclose_fixture_projection, project_fixture_anchor,
};
use time_domain::{
    ExchangeArithmetic, MonotonicNs, PhaseOffsetQ33, UtcCoordinateQ32, exchange_arithmetic,
};

/// Harness/accounting failures remain distinct from actual product rejection.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
/// Exact monotonic nanoseconds and phase numerator in units of 2^-33 seconds.
type Row = (u64, i128);
/// Complete immutable observation view, preserving its original scalar bits.
type RowBits = ([u64; 3], u64, i128, u64);
/// Closed marker order: histories, appends, observations, contexts, monotonic,
/// coordinates, exchanges, getters, terms, projections, enclosures.
type Recipe = [u64; 11];

/// Local bounded attempts; no shared counters or product-error-derived labels.
struct Profile {
    /// Fixed source-owned case label.
    case: &'static str,
    /// Independent maximum attempts in the marker's closed field order.
    ceiling: Recipe,
    /// Actual attempts, charged before the corresponding public operation.
    counts: [Cell<u64>; 11],
}

impl Profile {
    /// Start one all-zero local scope without product calls or effects.
    const fn new(case: &'static str, ceiling: Recipe) -> Self {
        Self {
            case,
            ceiling,
            counts: [const { Cell::new(0) }; 11],
        }
    }

    /// Charge one impending call; failed admission invokes no product operation.
    /// # Errors
    /// Rejects an invalid slot, overflow or exhausted fixed ceiling.
    fn charge(&self, slot: usize) -> TestResult {
        let counter = self
            .counts
            .get(slot)
            .ok_or_else(|| io::Error::other("enclosure counter slot"))?;
        let ceiling = self
            .ceiling
            .get(slot)
            .ok_or_else(|| io::Error::other("enclosure ceiling slot"))?;
        let next = counter
            .get()
            .checked_add(1)
            .ok_or_else(|| io::Error::other("enclosure counter overflow"))?;
        if next > *ceiling {
            return Err(io::Error::other("enclosure counter ceiling").into());
        }
        counter.set(next);
        Ok(())
    }

    /// Admit explicit nonzero fixture tags and call the real context constructor.
    /// # Errors
    /// Rejects a zero setup tag or exhausted constructor budget.
    fn context(&self, raw: [u64; 3]) -> TestResult<FixtureContext> {
        let tag = |value| {
            NonZeroU64::new(value).ok_or_else(|| io::Error::other("zero enclosure fixture tag"))
        };
        let source = tag(raw[0])?;
        let frame = tag(raw[1])?;
        let generation = tag(raw[2])?;
        self.charge(3)?;
        Ok(FixtureContext::new(source, frame, generation))
    }

    /// Construct exact fixture nanoseconds without an ambient clock.
    /// # Errors
    /// Rejects exhausted constructor accounting before the actual call.
    fn monotonic(&self, raw: u64) -> TestResult<MonotonicNs> {
        self.charge(4)?;
        Ok(MonotonicNs::new(raw))
    }

    /// Invoke checked UTC admission for the real phase producer.
    /// # Errors
    /// Propagates its actual domain rejection or exhausted accounting.
    fn coordinate(&self, raw: i128) -> TestResult<UtcCoordinateQ32> {
        self.charge(5)?;
        Ok(UtcCoordinateQ32::new(raw)?)
    }

    /// Run one real inert exchange on four already checked UTC coordinates.
    /// # Errors
    /// Propagates its actual arithmetic rejection or exhausted accounting.
    fn exchange(&self, values: [UtcCoordinateQ32; 4]) -> TestResult<ExchangeArithmetic> {
        self.charge(6)?;
        Ok(exchange_arithmetic(
            values[0], values[1], values[2], values[3],
        )?)
    }

    /// Construct one mandatory checked observation with literal unit weight.
    /// # Errors
    /// Propagates actual observation rejection or exhausted accounting.
    fn observation(
        &self,
        context: FixtureContext,
        at: MonotonicNs,
        phase: PhaseOffsetQ33,
    ) -> TestResult<FixtureObservation> {
        self.charge(2)?;
        Ok(FixtureObservation::new(context, at, phase, 1.0)?)
    }

    /// Create one actual empty history without requesting a fit or report.
    /// # Errors
    /// Rejects exhausted history-constructor accounting before the call.
    fn history(&self, context: FixtureContext) -> TestResult<FixtureHistory> {
        self.charge(0)?;
        Ok(FixtureHistory::new(context))
    }

    /// Reach the real sole history mutation without changing its returned result.
    /// # Errors
    /// The outer error is accounting; the inner error is actual append admission.
    fn append(
        &self,
        history: &mut FixtureHistory,
        row: FixtureObservation,
    ) -> TestResult<Result<(), HistoryError>> {
        self.charge(1)?;
        Ok(history.try_append(row))
    }

    /// Precharge exactly one product getter before invoking its closure.
    /// # Errors
    /// Rejects exhausted getter accounting before the actual observation.
    fn getter<T>(&self, getter: impl FnOnce() -> T) -> TestResult<T> {
        self.charge(7)?;
        Ok(getter())
    }

    /// Invoke actual mandatory three-term admission on raw supplied values.
    /// # Errors
    /// The outer error is accounting; the inner error preserves real admission.
    fn terms(
        &self,
        raw: [f64; 3],
    ) -> TestResult<Result<FixtureProjectionTerms, FixtureProjectionError>> {
        self.charge(8)?;
        Ok(FixtureProjectionTerms::new(raw[0], raw[1], raw[2]))
    }

    /// Project one actual retained anchor using its real same-history report.
    /// # Errors
    /// The outer error is accounting; the inner error is the actual consumer result.
    fn project(
        &self,
        history: &FixtureHistory,
        context: FixtureContext,
        index: usize,
        anchor: MonotonicNs,
        at: MonotonicNs,
        terms: FixtureProjectionTerms,
    ) -> TestResult<Result<FixtureAnchorProjection, FixtureProjectionError>> {
        self.charge(9)?;
        Ok(project_fixture_anchor(
            history, context, index, anchor, at, terms,
        ))
    }

    /// Call the public bridge with the actual private checked projection.
    /// # Errors
    /// The outer error is accounting; the inner error is the actual bridge result.
    fn enclose(
        &self,
        projection: &FixtureAnchorProjection,
    ) -> TestResult<Result<FixtureProjectedInterval, FixtureEnclosureError>> {
        self.charge(10)?;
        Ok(enclose_fixture_projection(projection))
    }
}

impl Drop for Profile {
    /// Emit one best-effort complete prefix even during assertion unwind.
    /// A failed/missing sink remains incomplete supervisor evidence.
    fn drop(&mut self) {
        let c = &self.counts;
        let row = format!(
            "CORE_ENCLOSURE_PROFILE case={} histories={} appends={} observations={} contexts={} monotonic={} coordinates={} exchanges={} getters={} terms={} projections={} enclosures={}\n",
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
            c[9].get(),
            c[10].get()
        );
        let _emission_result = io::stderr().lock().write_all(row.as_bytes());
    }
}

/// Read the three admitted context tags with three actual precharged getters.
/// # Errors
/// Propagates getter accounting; standard nonzero reads are not product calls.
fn context_bits(profile: &Profile, context: FixtureContext) -> TestResult<[u64; 3]> {
    Ok([
        profile.getter(|| context.source_id())?.get(),
        profile.getter(|| context.frame())?.get(),
        profile.getter(|| context.generation())?.get(),
    ])
}

/// Produce an exact phase through checked UTC admission and real exchange math.
/// # Errors
/// Propagates actual domain/accounting failure without a raw phase factory.
/// # Panics
/// The real producer must retain the independently supplied exact numerator.
fn phase(profile: &Profile, numerator: i128) -> TestResult<PhaseOffsetQ33> {
    let values = [
        profile.coordinate(0)?,
        profile.coordinate(numerator)?,
        profile.coordinate(0)?,
        profile.coordinate(0)?,
    ];
    let exchange = profile.exchange(values)?;
    let phase = profile.getter(|| exchange.offset())?;
    assert_eq!(profile.getter(|| phase.numerator())?, numerator);
    Ok(phase)
}

/// Populate exactly three real observations; every recipe spans two seconds.
/// # Errors
/// Propagates actual constructor/append/accounting failure without replacement.
fn make_history(
    profile: &Profile,
    context: FixtureContext,
    rows: &[Row; 3],
) -> TestResult<FixtureHistory> {
    let mut history = profile.history(context)?;
    for &(at, numerator) in rows {
        let at = profile.monotonic(at)?;
        let phase = phase(profile, numerator)?;
        let row = profile.observation(context, at, phase)?;
        profile.append(&mut history, row)??;
    }
    Ok(history)
}

/// Entire visible committed report, used only to observe mutation/provenance.
#[derive(Debug, PartialEq)]
struct ReportBits {
    /// Complete unchanged report tags.
    context: [u64; 3],
    /// Exact committed report reference.
    reference: u64,
    /// Statistics frame tag.
    frame: u64,
    /// Statistics input count.
    count: usize,
    /// Statistics degrees of freedom.
    dof: usize,
    /// Contained line frame tag.
    line_frame: u64,
    /// Phase, rate, SSE, residual scale and normalized variance bits.
    scalars: [u64; 5],
}

/// Complete public history snapshot, preserving exact rows and report scalars.
#[derive(Debug, PartialEq)]
struct HistoryBits {
    /// Entire immutable assertion.
    context: [u64; 3],
    /// Current admitted prefix length.
    count: usize,
    /// Exact latest accepted time, if any.
    reference: Option<u64>,
    /// Every visible row's original fields.
    rows: Vec<RowBits>,
    /// Entire actual committed report, if any.
    report: Option<ReportBits>,
}

/// Observe every public row/report field without deriving an expected fit.
/// # Errors
/// Rejects a count outside this three-row recipe, missing row or spent budget.
/// # Panics
/// Neither the prefix-end index nor maximum index may expose a row.
/// # Resources
/// Exactly 57 getters for each admitted three-row history used below.
fn history_bits(profile: &Profile, history: &FixtureHistory) -> TestResult<HistoryBits> {
    let context = profile.getter(|| history.context())?;
    let context = context_bits(profile, context)?;
    let count = profile.getter(|| history.sample_count())?;
    if count != 3 {
        return Err(io::Error::other("enclosure history recipe count").into());
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
            count: profile.getter(|| statistics.sample_count())?,
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
            .ok_or_else(|| io::Error::other("missing enclosure fixture row"))?;
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

/// Complete original projection view; floats retain their exact binary64 bits.
#[derive(Debug, PartialEq)]
struct ProjectionBits {
    /// All original fixture assertions, granting no source trust.
    context: [u64; 3],
    /// Actual anchor, latest, fit-reference and evaluation nanoseconds.
    instants: [u64; 4],
    /// Exact anchor and latest-observation ages in nanoseconds.
    ages: [u64; 2],
    /// Original anchor age, center, half-delay, dispersion, skew, growth, distance.
    scalars: [u64; 7],
}

/// Read every original projection getter with 24 actual precharged calls.
/// # Errors
/// Propagates exhausted accounting, including each nested getter separately.
fn projection_bits(
    profile: &Profile,
    value: &FixtureAnchorProjection,
) -> TestResult<ProjectionBits> {
    let context = profile.getter(|| value.context())?;
    let anchor = profile.getter(|| value.anchor_observed_at())?;
    let latest = profile.getter(|| value.latest_observed_at())?;
    let reference = profile.getter(|| value.fit_reference())?;
    let at = profile.getter(|| value.projected_at())?;
    let anchor_age = profile.getter(|| value.anchor_elapsed())?;
    let latest_age = profile.getter(|| value.latest_elapsed())?;
    let terms = profile.getter(|| value.terms())?;
    Ok(ProjectionBits {
        context: context_bits(profile, context)?,
        instants: [
            profile.getter(|| anchor.nanoseconds())?,
            profile.getter(|| latest.nanoseconds())?,
            profile.getter(|| reference.nanoseconds())?,
            profile.getter(|| at.nanoseconds())?,
        ],
        ages: [
            profile.getter(|| anchor_age.nanoseconds())?,
            profile.getter(|| latest_age.nanoseconds())?,
        ],
        scalars: [
            profile.getter(|| value.anchor_elapsed_seconds())?.to_bits(),
            profile.getter(|| value.center_seconds())?.to_bits(),
            profile.getter(|| terms.half_delay_seconds())?.to_bits(),
            profile.getter(|| terms.dispersion_seconds())?.to_bits(),
            profile
                .getter(|| terms.skew_seconds_per_second())?
                .to_bits(),
            profile.getter(|| value.growth_seconds())?.to_bits(),
            profile.getter(|| value.distance_seconds())?.to_bits(),
        ],
    })
}

/// Full immutable bridge output plus every original projection field.
#[derive(Debug, PartialEq)]
struct EnclosureBits {
    /// Actual source-derived interval label.
    label: u64,
    /// Inclusive checked Q33 endpoints.
    bounds: [i64; 2],
    /// Independently separate lower and upper rounding metadata.
    rounding: [FixtureEndpointRounding; 2],
    /// Exact quantum seconds, preserved as binary64 bits.
    quantum: u64,
    /// Complete original input projection retained by the wrapper.
    projection: ProjectionBits,
}

/// Observe one entire enclosure view with exactly 32 product getters.
/// # Errors
/// Propagates exhausted accounting before each immutable public observation.
fn enclosure_bits(
    profile: &Profile,
    value: &FixtureProjectedInterval,
) -> TestResult<EnclosureBits> {
    let interval = profile.getter(|| value.interval())?;
    let projection = profile.getter(|| value.projection())?;
    Ok(EnclosureBits {
        label: profile.getter(|| interval.label())?.get(),
        bounds: [
            profile.getter(|| interval.lower())?,
            profile.getter(|| interval.upper())?,
        ],
        rounding: [
            profile.getter(|| value.lower_rounding())?,
            profile.getter(|| value.upper_rounding())?,
        ],
        quantum: profile.getter(|| value.quantum_seconds())?.to_bits(),
        projection: projection_bits(profile, projection)?,
    })
}

/// Enclose a real projection, then inspect both its original and copied output.
/// # Errors
/// Propagates actual enclosure rejection or exhausted accounting.
/// # Panics
/// Copy/repeated reads retain every original field; Debug has the fixed redacted form.
/// # Resources
/// One enclosure, 88 getters and one bounded Debug format; no refit or mutation.
fn inspect_enclosure(
    profile: &Profile,
    projection: &FixtureAnchorProjection,
) -> TestResult<EnclosureBits> {
    let original = projection_bits(profile, projection)?;
    let value = profile.enclose(projection)??;
    assert_eq!(format!("{value:?}"), "FixtureProjectedInterval { .. }");
    let copied = value;
    let first = enclosure_bits(profile, &value)?;
    let repeated = enclosure_bits(profile, &copied)?;
    assert_eq!(first, repeated, "copy and repeated reads preserve output");
    assert_eq!(
        first.projection, original,
        "entire original projection retained"
    );
    Ok(first)
}

/// Literal mathematical facts supplied independently of fit/bridge execution.
#[derive(Clone, Copy)]
struct EnclosureFact {
    /// Exact expected represented fixture center in seconds.
    center: f64,
    /// Exact expected represented fixture radius in seconds.
    distance: f64,
    /// Expected outward integral Q33 endpoints.
    bounds: [i64; 2],
    /// Exactness/fraction metadata for each independent endpoint.
    rounding: [FixtureEndpointRounding; 2],
}

/// Check a three-row producer-to-enclosure journey against inline analytic facts.
/// # Errors
/// Propagates real setup/append/projection/enclosure or accounting failure.
/// # Panics
/// Exact center, radius, endpoint metadata, original identity and history must match.
/// # Resources
/// One history, three appends/observations/exchanges, twelve UTC coordinates,
/// one context, five monotonic values, one term/projection/enclosure, 208 getters.
fn check_fixture(
    profile: &Profile,
    context: [u64; 3],
    rows: &[Row; 3],
    evaluation: (usize, u64, u64),
    terms: [f64; 3],
    fact: EnclosureFact,
) -> TestResult<EnclosureBits> {
    let raw_context = context;
    let context = profile.context(raw_context)?;
    let history = make_history(profile, context, rows)?;
    let before = history_bits(profile, &history)?;
    let admitted_terms = profile.terms(terms)??;
    let anchor = profile.monotonic(evaluation.1)?;
    let at = profile.monotonic(evaluation.2)?;
    let projection =
        profile.project(&history, context, evaluation.0, anchor, at, admitted_terms)??;
    let actual = inspect_enclosure(profile, &projection)?;
    assert_eq!(actual.label, raw_context[0], "actual source owns label");
    assert_eq!(actual.projection.context, raw_context);
    assert_eq!(actual.projection.scalars[1], fact.center.to_bits());
    assert_eq!(actual.projection.scalars[6], fact.distance.to_bits());
    assert_eq!(
        &actual.projection.scalars[2..5],
        &terms.map(f64::to_bits),
        "supplied terms retain original bits"
    );
    assert_eq!(actual.bounds, fact.bounds);
    assert_eq!(actual.rounding, fact.rounding);
    assert_eq!(actual.quantum, (1.0_f64 / 8_589_934_592.0).to_bits());
    assert_eq!(history_bits(profile, &history)?, before);
    Ok(actual)
}

/// Exact phase numerators 3/32, 7/64 and 1/8 seconds at 98, 99 and 100 seconds.
const LINEAR: [Row; 3] = [
    (98_000_000_000, 805_306_368),
    (99_000_000_000, 939_524_096),
    (100_000_000_000, 1_073_741_824),
];
/// Constant 1-second phase, inclusive at the existing observation magnitude limit.
const CONSTANT_ONE: [Row; 3] = [
    (0, 8_589_934_592),
    (1_000_000_000, 8_589_934_592),
    (2_000_000_000, 8_589_934_592),
];
/// Constant phase 1/8 second supplies exact positive and signed-zero controls.
const CONSTANT_EIGHTH: [Row; 3] = [
    (0, 1_073_741_824),
    (1_000_000_000, 1_073_741_824),
    (2_000_000_000, 1_073_741_824),
];
/// Exact slope +1 Q33 tick/second and latest phase zero, with a legal 2-second span.
const RISING_TICK: [Row; 3] = [(0, -2), (1_000_000_000, -1), (2_000_000_000, 0)];
/// Exact slope -1 Q33 tick/second and latest phase zero.
const FALLING_TICK: [Row; 3] = [(0, 2), (1_000_000_000, 1), (2_000_000_000, 0)];
/// Complete fixture assertions; these labels establish no authentication.
const CONTEXT: [u64; 3] = [41, 43, 47];
/// Separate half-delay, dispersion and skew inputs in their declared units.
const TERMS: [f64; 3] = [1.0 / 64.0, 1.0 / 64.0, 1.0 / 128.0];
/// Both endpoints are already exactly represented integral Q33 values.
const EXACT: [FixtureEndpointRounding; 2] = [FixtureEndpointRounding::Exact; 2];
/// Both endpoints need a strictly fractional outward extension.
const OUTWARD: [FixtureEndpointRounding; 2] = [FixtureEndpointRounding::OutwardFraction; 2];

/// Latest anchor at 100s projects to 5/32s with radius 3/64s at 102s.
/// # Errors
/// Propagates the actual checked producer/consumer or accounting failure.
/// # Panics
/// Exact endpoints [7*2^27,13*2^27] and original ages must match.
#[test]
fn latest_linear_anchor_has_exact_q33_endpoints() -> TestResult {
    let profile = Profile::new("linear_latest", [1, 3, 3, 1, 5, 12, 3, 208, 1, 1, 1]);
    let actual = check_fixture(
        &profile,
        CONTEXT,
        &LINEAR,
        (2, 100_000_000_000, 102_000_000_000),
        TERMS,
        EnclosureFact {
            center: 5.0 / 32.0,
            distance: 3.0 / 64.0,
            bounds: [939_524_096, 1_744_830_464],
            rounding: EXACT,
        },
    )?;
    assert_eq!(actual.projection.ages, [2_000_000_000; 2]);
    Ok(())
}

/// Older anchor at 98s has the same center but radius 4/64s at 102s.
/// # Errors
/// Propagates the actual checked producer/consumer or accounting failure.
/// # Panics
/// Exact endpoints [6*2^27,14*2^27] retain distinct actual anchor/latest ages.
#[test]
fn older_linear_anchor_retains_its_larger_distance_and_original_age() -> TestResult {
    let profile = Profile::new("linear_older", [1, 3, 3, 1, 5, 12, 3, 208, 1, 1, 1]);
    let actual = check_fixture(
        &profile,
        CONTEXT,
        &LINEAR,
        (0, 98_000_000_000, 102_000_000_000),
        TERMS,
        EnclosureFact {
            center: 5.0 / 32.0,
            distance: 4.0 / 64.0,
            bounds: [805_306_368, 1_879_048_192],
            rounding: EXACT,
        },
    )?;
    assert_eq!(actual.projection.ages, [4_000_000_000, 2_000_000_000]);
    Ok(())
}

/// Exact represented radius 2^-80 is smaller than binary64 addition resolves at 1.
/// # Errors
/// Propagates actual checked producer/consumer or accounting failure.
/// # Panics
/// Ordinary rounded endpoints lose the radius; the exact bridge must retain it.
#[test]
fn tiny_radius_is_enclosed_before_binary64_endpoint_rounding_can_erase_it() -> TestResult {
    let profile = Profile::new("tiny_radius", [1, 3, 3, 1, 5, 12, 3, 208, 1, 1, 1]);
    let tiny = f64::from_bits(943_u64 << 52);
    assert_eq!((1.0_f64 - tiny).to_bits(), 1.0_f64.to_bits());
    assert_eq!((1.0_f64 + tiny).to_bits(), 1.0_f64.to_bits());
    check_fixture(
        &profile,
        CONTEXT,
        &CONSTANT_ONE,
        (2, 2_000_000_000, 2_000_000_000),
        [tiny, 0.0, 0.0],
        EnclosureFact {
            center: 1.0,
            distance: tiny,
            bounds: [8_589_934_591, 8_589_934_593],
            rounding: OUTWARD,
        },
    )?;
    Ok(())
}

/// Legal unit-tick slopes projected half a second produce centers +/-2^-34s.
/// # Errors
/// Propagates actual checked producer/consumer or accounting failure.
/// # Panics
/// Directed rounding must distinguish positive [0,1] from negative [-1,0].
#[test]
fn public_half_tick_centers_round_outward_for_both_signs() -> TestResult {
    let profile = Profile::new("fractional_signs", [2, 6, 6, 2, 10, 24, 6, 416, 2, 2, 2]);
    for (rows, center, bounds) in [
        (&RISING_TICK, 1.0 / 17_179_869_184.0, [0, 1]),
        (&FALLING_TICK, -1.0 / 17_179_869_184.0, [-1, 0]),
    ] {
        check_fixture(
            &profile,
            CONTEXT,
            rows,
            (2, 2_000_000_000, 2_500_000_000),
            [0.0; 3],
            EnclosureFact {
                center,
                distance: 0.0,
                bounds,
                rounding: OUTWARD,
            },
        )?;
    }
    Ok(())
}

/// Quarter-tick centers/radii cancel one endpoint exactly and leave the other half.
/// # Errors
/// Propagates actual checked producer/consumer or accounting failure.
/// # Panics
/// Lower and upper rounding metadata must remain independent after cancellation.
#[test]
fn public_fraction_cancellation_has_independent_endpoint_exactness() -> TestResult {
    let profile = Profile::new(
        "asymmetric_endpoints",
        [2, 6, 6, 2, 10, 24, 6, 416, 2, 2, 2],
    );
    let quarter = 1.0 / 34_359_738_368.0;
    for (rows, center, bounds, rounding) in [
        (
            &RISING_TICK,
            quarter,
            [0, 1],
            [
                FixtureEndpointRounding::Exact,
                FixtureEndpointRounding::OutwardFraction,
            ],
        ),
        (
            &FALLING_TICK,
            -quarter,
            [-1, 0],
            [
                FixtureEndpointRounding::OutwardFraction,
                FixtureEndpointRounding::Exact,
            ],
        ),
    ] {
        check_fixture(
            &profile,
            CONTEXT,
            rows,
            (2, 2_000_000_000, 2_250_000_000),
            [quarter, 0.0, 0.0],
            EnclosureFact {
                center,
                distance: quarter,
                bounds,
                rounding,
            },
        )?;
    }
    Ok(())
}

/// Same-instant quarter/three-quarter tick centers have disjoint eighth-tick radii.
/// # Errors
/// Propagates actual checked producer/consumer or accounting failure.
/// # Panics
/// Their disjoint [1/8,3/8] and [5/8,7/8] tick inputs both enclose to [0,1].
#[test]
fn disjoint_subtick_fixture_operands_can_have_overlapping_integer_enclosures() -> TestResult {
    let profile = Profile::new("subtick_overlap", [2, 6, 6, 2, 10, 24, 6, 416, 2, 2, 2]);
    let rising_three = [(0, -6), (1_000_000_000, -3), (2_000_000_000, 0)];
    let first = check_fixture(
        &profile,
        CONTEXT,
        &RISING_TICK,
        (2, 2_000_000_000, 2_250_000_000),
        [1.0 / 68_719_476_736.0, 0.0, 0.0],
        EnclosureFact {
            center: 1.0 / 34_359_738_368.0,
            distance: 1.0 / 68_719_476_736.0,
            bounds: [0, 1],
            rounding: OUTWARD,
        },
    )?;
    let second = check_fixture(
        &profile,
        [53, 43, 47],
        &rising_three,
        (2, 2_000_000_000, 2_250_000_000),
        [1.0 / 68_719_476_736.0, 0.0, 0.0],
        EnclosureFact {
            center: 3.0 / 34_359_738_368.0,
            distance: 1.0 / 68_719_476_736.0,
            bounds: [0, 1],
            rounding: OUTWARD,
        },
    )?;
    assert_eq!(
        first.bounds, second.bounds,
        "rounding overlap is not physical agreement"
    );
    assert_eq!(first.projection.instants[3], second.projection.instants[3]);
    Ok(())
}

/// Both signed-zero term triples are legal; all-negative inputs produce -0 radius.
/// # Errors
/// Propagates actual checked producer/consumer or accounting failure.
/// # Panics
/// Original term/radius signs survive while both enclosures remain exact singletons.
#[test]
fn signed_zero_terms_and_computed_radius_survive_original_projection_retention() -> TestResult {
    let profile = Profile::new("signed_zero", [2, 6, 6, 2, 10, 24, 6, 416, 2, 2, 2]);
    for zero in [0.0_f64, -0.0] {
        check_fixture(
            &profile,
            CONTEXT,
            &CONSTANT_EIGHTH,
            (2, 2_000_000_000, 2_000_000_000),
            [zero; 3],
            EnclosureFact {
                center: 1.0 / 8.0,
                distance: zero,
                bounds: [1_073_741_824; 2],
                rounding: EXACT,
            },
        )?;
    }
    Ok(())
}

/// Projection's valid finite term range is wider than the enclosure's i64 range.
/// # Errors
/// Propagates actual checked setup/projection/control failures or spent accounting.
/// # Panics
/// A valid projection is rejected as Unrepresentable, unchanged; its neighbor succeeds.
#[test]
fn valid_large_radius_projection_is_rejected_without_changing_history_or_input() -> TestResult {
    let profile = Profile::new("large_radius", [1, 3, 3, 1, 7, 12, 3, 256, 2, 2, 2]);
    let context = profile.context(CONTEXT)?;
    let history = make_history(&profile, context, &CONSTANT_EIGHTH)?;
    let before = history_bits(&profile, &history)?;
    let terms = profile.terms([1_073_741_824.0, 0.0, 0.0])??;
    let anchor = profile.monotonic(2_000_000_000)?;
    let at = profile.monotonic(2_000_000_000)?;
    let projection = profile.project(&history, context, 2, anchor, at, terms)??;
    let original = projection_bits(&profile, &projection)?;
    assert_eq!(original.scalars[1], (1.0_f64 / 8.0).to_bits());
    assert_eq!(original.scalars[6], 1_073_741_824.0_f64.to_bits());
    let error = profile
        .enclose(&projection)?
        .err()
        .ok_or_else(|| io::Error::other("large enclosure unexpectedly accepted"))?;
    assert_eq!(error, FixtureEnclosureError::Unrepresentable);
    assert_eq!(projection_bits(&profile, &projection)?, original);
    let terms = profile.terms([1.0 / 64.0, 0.0, 0.0])??;
    let anchor = profile.monotonic(2_000_000_000)?;
    let at = profile.monotonic(2_000_000_000)?;
    let healthy = profile.project(&history, context, 2, anchor, at, terms)??;
    let actual = inspect_enclosure(&profile, &healthy)?;
    assert_eq!(actual.bounds, [939_524_096, 1_207_959_552]);
    assert_eq!(actual.rounding, EXACT);
    assert_eq!(history_bits(&profile, &history)?, before);
    Ok(())
}

/// Identical numerical histories retain each actual nonzero source/frame/generation.
/// # Errors
/// Propagates actual checked producer/consumer or accounting failure.
/// # Panics
/// Each label comes from its source, including `u64::MAX`; reads create no observation.
#[test]
fn actual_source_labels_and_complete_context_survive_repeated_reads_and_copy() -> TestResult {
    let profile = Profile::new("identity_labels", [3, 9, 9, 3, 15, 36, 9, 624, 3, 3, 3]);
    for context in [CONTEXT, [53, 59, 61], [u64::MAX, 67, 71]] {
        let actual = check_fixture(
            &profile,
            context,
            &CONSTANT_EIGHTH,
            (2, 2_000_000_000, 3_000_000_000),
            [0.0; 3],
            EnclosureFact {
                center: 1.0 / 8.0,
                distance: 0.0,
                bounds: [1_073_741_824; 2],
                rounding: EXACT,
            },
        )?;
        assert_eq!(
            actual.projection.instants,
            [2_000_000_000, 2_000_000_000, 2_000_000_000, 3_000_000_000]
        );
        assert_eq!(actual.projection.ages, [1_000_000_000; 2]);
    }
    Ok(())
}

// SPDX-License-Identifier: GPL-2.0-only
//! Independent CHECK-7/8/14/25 producer-to-compatible-batch fixture facts.
//!
//! Genuine checked UTC/exchange phases feed observations, history, projection
//! and enclosure before the new consumer. Literal dyadic facts own the numeric
//! oracle; complete bit snapshots own retention checks. Counters precharge
//! direct public attempts and retain their completed prefix on unwind. Internal
//! consumer work has separate source-owned bounds. These fixtures establish no
//! physical confidence, source independence, freshness policy or clock authority.
#![forbid(unsafe_code)]

use std::{
    cell::Cell,
    error::Error,
    io::{self, Write},
    num::NonZeroU64,
};

use time_core::{
    BatchError, FixtureAnchorProjection, FixtureContext, FixtureEnclosureError,
    FixtureEndpointRounding, FixtureHistory, FixtureMinimum, FixtureObservation,
    FixtureOutcomeKind, FixtureProjectedBatch, FixtureProjectedBatchError,
    FixtureProjectedInterval, FixtureProjectionError, FixtureProjectionMismatchKind,
    FixtureProjectionTerms, FixtureSelection, HistoryError, enclose_fixture_projection,
    project_fixture_anchor,
};
use time_domain::{
    ExchangeArithmetic, MonotonicNs, PhaseOffsetQ33, UtcCoordinateQ32, exchange_arithmetic,
};

/// Harness failures remain distinct from actual product rejections.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
/// Exact monotonic nanoseconds and phase numerator in Q33 units.
type Row = (u64, i128);
/// Closed public-attempt order agreed with the accounting owner.
type Recipe = [u64; 14];
/// Constant one-eighth-second enclosure: [7T,9T], T=2^27 Q33 units.
const CONSTANT_BOUNDS: [i64; 2] = [939_524_096, 1_207_959_552];
/// Latest-linear enclosure: [7T,13T].
const LINEAR_BOUNDS: [i64; 2] = [939_524_096, 1_744_830_464];
/// Older-linear enclosure: [6T,14T].
const OLDER_BOUNDS: [i64; 2] = [805_306_368, 1_879_048_192];
/// Constant-one-second enclosure: [63T,65T].
const FAR_BOUNDS: [i64; 2] = [8_455_716_864, 8_724_152_320];
/// Genuine linear history spanning two seconds.
const LINEAR_ROWS: [Row; 3] = [
    (98_000_000_000, 3_i128 << 28),
    (99_000_000_000, 7_i128 << 27),
    (100_000_000_000, 1_i128 << 30),
];
/// Genuine constant history spanning two seconds.
const CONSTANT_ROWS: [Row; 3] = [
    (0, 1_i128 << 30),
    (1_000_000_000, 1_i128 << 30),
    (2_000_000_000, 1_i128 << 30),
];

/// Literal checked header plus its independently supplied raw expected view.
#[derive(Clone, Copy)]
struct Expected {
    /// Nonzero shape is owned by the standard checked type.
    frame: NonZeroU64,
    /// Mapping-generation assertion, never a peer generation.
    generation: NonZeroU64,
    /// Supplied monotonic reading without an ambient clock.
    at: MonotonicNs,
    /// Original literal arguments used for preservation assertions.
    raw: [u64; 3],
}

/// One bounded local attempt scope with no shared or product-derived counters.
struct Profile {
    /// Fixed source-owned marker label.
    case: &'static str,
    /// Independent per-field source upper, not an execution observation.
    ceiling: Recipe,
    /// Actual public attempts, charged before their corresponding operation.
    counts: [Cell<u64>; 14],
}

impl Profile {
    /// Start a fixed all-zero local scope without product calls or effects.
    const fn new(case: &'static str, ceiling: Recipe) -> Self {
        Self {
            case,
            ceiling,
            counts: [const { Cell::new(0) }; 14],
        }
    }

    /// Precharge one impending call without performing it on accounting failure.
    /// # Errors
    /// Rejects an invalid slot, overflow or exhausted source ceiling.
    fn charge(&self, slot: usize) -> TestResult {
        let counter = self
            .counts
            .get(slot)
            .ok_or_else(|| io::Error::other("batch counter slot"))?;
        let ceiling = self
            .ceiling
            .get(slot)
            .ok_or_else(|| io::Error::other("batch ceiling slot"))?;
        let next = counter
            .get()
            .checked_add(1)
            .ok_or_else(|| io::Error::other("batch counter overflow"))?;
        if next > *ceiling {
            return Err(io::Error::other("batch counter ceiling").into());
        }
        counter.set(next);
        Ok(())
    }

    /// Admit nonzero tags and invoke the real context constructor.
    /// # Errors
    /// Rejects a zero setup tag or exhausted constructor accounting.
    fn context(&self, raw: [u64; 3]) -> TestResult<FixtureContext> {
        let source = tag(raw[0])?;
        let frame = tag(raw[1])?;
        let generation = tag(raw[2])?;
        self.charge(3)?;
        Ok(FixtureContext::new(source, frame, generation))
    }

    /// Tag literal fixture nanoseconds without an ambient reading.
    /// # Errors
    /// Rejects exhausted accounting before the product constructor.
    fn monotonic(&self, raw: u64) -> TestResult<MonotonicNs> {
        self.charge(4)?;
        Ok(MonotonicNs::new(raw))
    }

    /// Admit one real UTC coordinate for the phase producer.
    /// # Errors
    /// Preserves actual domain rejection or accounting exhaustion.
    fn coordinate(&self, raw: i128) -> TestResult<UtcCoordinateQ32> {
        self.charge(5)?;
        Ok(UtcCoordinateQ32::new(raw)?)
    }

    /// Reach real exchange arithmetic on four checked coordinates.
    /// # Errors
    /// Preserves actual arithmetic rejection or accounting exhaustion.
    fn exchange(&self, values: [UtcCoordinateQ32; 4]) -> TestResult<ExchangeArithmetic> {
        self.charge(6)?;
        Ok(exchange_arithmetic(
            values[0], values[1], values[2], values[3],
        )?)
    }

    /// Construct one checked observation with literal unit variance weight.
    /// # Errors
    /// Preserves actual observation rejection or accounting exhaustion.
    fn observation(
        &self,
        context: FixtureContext,
        at: MonotonicNs,
        phase: PhaseOffsetQ33,
    ) -> TestResult<FixtureObservation> {
        self.charge(2)?;
        Ok(FixtureObservation::new(context, at, phase, 1.0)?)
    }

    /// Create the real empty history with no fit or report substitution.
    /// # Errors
    /// Rejects exhausted constructor accounting before the call.
    fn history(&self, context: FixtureContext) -> TestResult<FixtureHistory> {
        self.charge(0)?;
        Ok(FixtureHistory::new(context))
    }

    /// Reach the sole history mutation, preserving its actual result.
    /// # Errors
    /// Outer failure is accounting; inner failure is actual append admission.
    fn append(
        &self,
        history: &mut FixtureHistory,
        row: FixtureObservation,
    ) -> TestResult<Result<(), HistoryError>> {
        self.charge(1)?;
        Ok(history.try_append(row))
    }

    /// Charge one explicit public getter before invoking its closure.
    /// # Errors
    /// Rejects exhausted accounting before the immutable observation.
    fn getter<T>(&self, getter: impl FnOnce() -> T) -> TestResult<T> {
        self.charge(7)?;
        Ok(getter())
    }

    /// Construct actual supplied terms without normalization.
    /// # Errors
    /// Outer failure is accounting; inner failure is actual term admission.
    fn terms(
        &self,
        raw: [f64; 3],
    ) -> TestResult<Result<FixtureProjectionTerms, FixtureProjectionError>> {
        self.charge(8)?;
        Ok(FixtureProjectionTerms::new(raw[0], raw[1], raw[2]))
    }

    /// Project a real retained anchor using its genuine same-history report.
    /// # Errors
    /// Outer failure is accounting; inner failure is actual projection admission.
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

    /// Enclose a genuinely constructed immutable projection.
    /// # Errors
    /// Outer failure is accounting; inner failure is actual enclosure admission.
    fn enclose(
        &self,
        projection: &FixtureAnchorProjection,
    ) -> TestResult<Result<FixtureProjectedInterval, FixtureEnclosureError>> {
        self.charge(10)?;
        Ok(enclose_fixture_projection(projection))
    }

    /// Construct the real borrowed consumer without optional caller validation.
    /// # Errors
    /// Outer failure is accounting; inner failure is the actual consumer result.
    fn batch<'a>(
        &self,
        expected: Expected,
        values: &'a [FixtureProjectedInterval],
    ) -> TestResult<Result<FixtureProjectedBatch<'a>, FixtureProjectedBatchError>> {
        self.charge(11)?;
        Ok(FixtureProjectedBatch::new(
            expected.frame,
            expected.generation,
            expected.at,
            values,
        ))
    }

    /// Admit the supplied minimum without clamping it to population.
    /// # Errors
    /// Preserves actual minimum rejection or accounting exhaustion.
    fn minimum(&self, raw: usize) -> TestResult<FixtureMinimum> {
        self.charge(12)?;
        Ok(FixtureMinimum::new(raw)?)
    }

    /// Invoke the real typed consumer with the unchanged checked minimum.
    /// # Errors
    /// Preserves actual selection rejection or accounting exhaustion.
    fn select(
        &self,
        batch: &FixtureProjectedBatch<'_>,
        minimum: FixtureMinimum,
    ) -> TestResult<FixtureSelection> {
        self.charge(13)?;
        Ok(batch.select(minimum)?)
    }
}

impl Drop for Profile {
    /// Emit the complete attempted prefix even during assertion unwind.
    /// A missing/failed sink remains incomplete supervisor evidence.
    fn drop(&mut self) {
        let c = &self.counts;
        let row = format!(
            "CORE_PROJECTED_BATCH_PROFILE case={} histories={} appends={} observations={} contexts={} monotonic={} coordinates={} exchanges={} getters={} terms={} projections={} enclosures={} batches={} minima={} selections={}\n",
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
            c[10].get(),
            c[11].get(),
            c[12].get(),
            c[13].get()
        );
        let _emission_result = io::stderr().lock().write_all(row.as_bytes());
    }
}

/// Admit a nonzero fixture tag without a fallback or default.
/// # Errors
/// Rejects zero before any product context or header construction.
fn tag(value: u64) -> TestResult<NonZeroU64> {
    NonZeroU64::new(value)
        .ok_or_else(|| io::Error::other("zero projected-batch fixture tag").into())
}

/// Tag one literal expected header, retaining its raw facts separately.
/// # Errors
/// Propagates tag or monotonic-constructor accounting failure.
fn expected(profile: &Profile, raw: [u64; 3]) -> TestResult<Expected> {
    Ok(Expected {
        frame: tag(raw[0])?,
        generation: tag(raw[1])?,
        at: profile.monotonic(raw[2])?,
        raw,
    })
}

/// Produce an exact Q33 phase through real UTC admission and exchange math.
/// # Errors
/// Preserves domain/accounting failure without a raw phase factory.
/// # Panics
/// The real phase producer must retain the literal requested numerator.
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

/// Populate a genuine three-row history, then project and enclose its actual fit.
///
/// The array fixes the population; every supplied recipe spans two seconds.
/// Actual constructors own tag, phase, chronology and numerical admission.
/// # Errors
/// Rejects an unavailable setup anchor or preserves actual producer rejection.
fn genuine(
    profile: &Profile,
    raw_context: [u64; 3],
    rows: &[Row; 3],
    anchor_index: usize,
    raw_at: u64,
    raw_terms: [f64; 3],
) -> TestResult<FixtureProjectedInterval> {
    let anchor_raw = rows
        .get(anchor_index)
        .ok_or_else(|| io::Error::other("missing fixture anchor"))?
        .0;
    let context = profile.context(raw_context)?;
    let mut history = profile.history(context)?;
    for &(raw_at, numerator) in rows {
        let at = profile.monotonic(raw_at)?;
        let phase = phase(profile, numerator)?;
        let row = profile.observation(context, at, phase)?;
        profile.append(&mut history, row)??;
    }
    let anchor = profile.monotonic(anchor_raw)?;
    let at = profile.monotonic(raw_at)?;
    let terms = profile.terms(raw_terms)??;
    let projection = profile.project(&history, context, anchor_index, anchor, at, terms)??;
    Ok(profile.enclose(&projection)??)
}

/// Build the constant one-eighth-second neighboring control.
/// # Errors
/// Preserves genuine producer rejection without replacing fixture data.
fn constant(profile: &Profile, tags: [u64; 3], at: u64) -> TestResult<FixtureProjectedInterval> {
    genuine(profile, tags, &CONSTANT_ROWS, 0, at, [1.0 / 64.0, 0.0, 0.0])
}

/// Complete original projection fields, preserving exact float representations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ProjectionBits {
    /// Source, asserted frame and asserted mapping generation.
    context: [u64; 3],
    /// Anchor, latest, fit reference and evaluation nanoseconds.
    instants: [u64; 4],
    /// Exact anchor/latest ages, never refreshed by the batch.
    ages: [u64; 2],
    /// Age seconds, center, fitted rate, delay, dispersion, skew, growth, distance.
    scalars: [u64; 8],
}

/// Complete wrapper data, never recomputed by the new consumer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct EnclosureBits {
    /// Actual source-derived label.
    label: u64,
    /// Checked inclusive endpoints.
    bounds: [i64; 2],
    /// Separate endpoint rounding metadata.
    rounding: [FixtureEndpointRounding; 2],
    /// Exact quantum representation.
    quantum: u64,
    /// Complete retained original projection.
    projection: ProjectionBits,
}

/// Read all three context tags with exactly three explicit getters.
/// # Errors
/// Propagates precharged accounting failure.
fn context_bits(profile: &Profile, value: FixtureContext) -> TestResult<[u64; 3]> {
    Ok([
        profile.getter(|| value.source_id())?.get(),
        profile.getter(|| value.frame())?.get(),
        profile.getter(|| value.generation())?.get(),
    ])
}

/// Read every original projection field with exactly 25 public getters.
/// # Errors
/// Propagates accounting before each immutable observation.
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
            profile
                .getter(|| value.phase_rate_seconds_per_second())?
                .to_bits(),
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

/// Read the complete wrapper with exactly 33 public getters.
/// # Errors
/// Propagates accounting before every immutable observation.
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

/// Snapshot a fixture slice bounded to the seventeen-entry reject neighbor.
/// # Errors
/// Rejects an out-of-scope length before allocation or exhausted accounting.
fn all_bits(
    profile: &Profile,
    values: &[FixtureProjectedInterval],
) -> TestResult<Vec<EnclosureBits>> {
    if values.len() > 17 {
        return Err(io::Error::other("batch snapshot fixture capacity").into());
    }
    values
        .iter()
        .map(|value| enclosure_bits(profile, value))
        .collect()
}

/// Complete bounded witness metadata.
#[derive(Debug, Eq, PartialEq)]
struct WitnessBits {
    /// Inclusive maximum component.
    component: [i64; 2],
    /// Its actual chosen witness.
    point: i64,
    /// Full supporting intersection.
    intersection: [i64; 2],
    /// Sorted actual labels, at most sixteen by the existing checked owner.
    supporters: Vec<u64>,
}

/// Complete observable selection metadata and optional witness.
#[derive(Debug, Eq, PartialEq)]
struct SelectionBits {
    /// Population, configured minimum and actual depth.
    counts: [usize; 3],
    /// Strict-majority and minimum-met predicates.
    predicates: [bool; 2],
    /// Actual closed outcome.
    outcome: FixtureOutcomeKind,
    /// Only unsupported ambiguity exposes this count in the existing API.
    component_count: Option<usize>,
    /// Entire optional numeric witness.
    witness: Option<WitnessBits>,
}

/// Read eight selection getters, plus six when a witness is present.
/// # Errors
/// Propagates accounting without inventing a witness or substitute result.
fn selection_bits(profile: &Profile, value: &FixtureSelection) -> TestResult<SelectionBits> {
    let counts = [
        profile.getter(|| value.population())?,
        profile.getter(|| value.minimum())?,
        profile.getter(|| value.maximum_depth())?,
    ];
    let predicates = [
        profile.getter(|| value.strict_majority())?,
        profile.getter(|| value.minimum_met())?,
    ];
    let outcome = profile.getter(|| value.outcome())?;
    let component_count = profile.getter(|| value.component_count())?;
    let witness = if let Some(witness) = profile.getter(|| value.witnessed())? {
        Some(WitnessBits {
            component: [
                profile.getter(|| witness.component_lower())?,
                profile.getter(|| witness.component_upper())?,
            ],
            point: profile.getter(|| witness.witness())?,
            intersection: [
                profile.getter(|| witness.intersection_lower())?,
                profile.getter(|| witness.intersection_upper())?,
            ],
            supporters: profile
                .getter(|| witness.supporters())?
                .map(NonZeroU64::get)
                .collect(),
        })
    } else {
        None
    };
    Ok(SelectionBits {
        counts,
        predicates,
        outcome,
        component_count,
        witness,
    })
}

/// Prepare independent literal witnessed facts without reading product data.
/// # Errors
/// Rejects more than sixteen expected labels before a bounded vector allocation.
fn witnessed(
    population: usize,
    minimum: usize,
    depth: usize,
    bounds: [i64; 2],
    labels: &[u64],
) -> TestResult<SelectionBits> {
    if labels.len() > 16 {
        return Err(io::Error::other("expected witness capacity").into());
    }
    Ok(SelectionBits {
        counts: [population, minimum, depth],
        predicates: [true, true],
        outcome: FixtureOutcomeKind::WitnessedFixture,
        component_count: None,
        witness: Some(WitnessBits {
            component: bounds,
            point: bounds[0],
            intersection: bounds,
            supporters: labels.to_vec(),
        }),
    })
}

/// Copy planned non-witness facts with an explicitly unmet configured minimum.
/// No expected predicate or coordinate is derived from actual product output.
fn unwitnessed(
    population: usize,
    minimum: usize,
    depth: usize,
    majority: bool,
    outcome: FixtureOutcomeKind,
) -> SelectionBits {
    SelectionBits {
        counts: [population, minimum, depth],
        predicates: [majority, false],
        outcome,
        component_count: None,
        witness: None,
    }
}

/// Assert header, original borrow, input order and all original fields.
/// # Errors
/// Preserves accounting or bounded snapshot failure.
/// # Panics
/// Any header, prefix, order, storage-copy or original-bit change fails.
/// # Resources
/// Seven getters plus 33 per entry; one fixed redacted Debug observation.
fn assert_batch_view(
    profile: &Profile,
    batch: &FixtureProjectedBatch<'_>,
    original: &[FixtureProjectedInterval],
    before: &[EnclosureBits],
    expected: Expected,
) -> TestResult {
    let frame = profile.getter(|| batch.frame())?;
    let generation = profile.getter(|| batch.mapping_generation())?;
    let at = profile.getter(|| batch.projected_at())?;
    assert_eq!(
        [
            frame.get(),
            generation.get(),
            profile.getter(|| at.nanoseconds())?
        ],
        expected.raw
    );
    assert_eq!(profile.getter(|| batch.len())?, original.len());
    assert_eq!(profile.getter(|| batch.is_empty())?, original.is_empty());
    let entries = profile.getter(|| batch.entries())?;
    assert_eq!(entries.len(), original.len());
    if !original.is_empty() {
        assert!(
            std::ptr::eq(entries, original),
            "retained nonempty entries must borrow the original slice"
        );
    }
    let observed = all_bits(profile, entries)?;
    assert_eq!(observed.as_slice(), before);
    assert_eq!(format!("{batch:?}"), "FixtureProjectedBatch { .. }");
    Ok(())
}

/// Assert the exact prescribed mismatch without an invented underlying cause.
/// # Errors
/// Rejects unexpected admission or preserves the error-source getter's accounting failure.
/// # Panics
/// Wrong index/kind or a retained collaborator cause fail.
fn expect_mismatch(
    profile: &Profile,
    result: Result<FixtureProjectedBatch<'_>, FixtureProjectedBatchError>,
    index: usize,
    kind: FixtureProjectionMismatchKind,
) -> TestResult {
    let error = result
        .err()
        .ok_or_else(|| io::Error::other("incompatible original context must be rejected"))?;
    assert_eq!(error, FixtureProjectedBatchError::Mismatch { index, kind });
    assert!(profile.getter(|| error.source())?.is_none());
    Ok(())
}

/// A genuine compatible enclosure must enter the batch and unchanged selector.
/// # Errors
/// Preserves actual producer/consumer rejection without replacing expected facts.
/// # Panics
/// Retains the initial RED's success assertion and literal interval oracle.
#[test]
fn genuine_compatible_projection_reaches_real_batch_selection() -> TestResult {
    let profile = Profile::new(
        "single_compatible",
        [1, 3, 3, 1, 6, 12, 3, 24, 1, 1, 1, 1, 1, 1],
    );
    let values = [constant(&profile, [41, 43, 47], 3_000_000_000)?];
    let lower = profile.getter(|| values[0].interval())?;
    assert_eq!(profile.getter(|| lower.lower())?, 939_524_096);
    let upper = profile.getter(|| values[0].interval())?;
    assert_eq!(profile.getter(|| upper.upper())?, 1_207_959_552);
    let header = expected(&profile, [43, 47, 3_000_000_000])?;
    let result = profile.batch(header, &values)?;
    assert!(
        result.is_ok(),
        "genuine same-context projection must be admitted"
    );
    let batch = result?;
    let minimum = profile.minimum(1)?;
    let selected = profile.select(&batch, minimum)?;
    assert_eq!(
        selection_bits(&profile, &selected)?,
        witnessed(1, 1, 1, CONSTANT_BOUNDS, &[41])?
    );
    Ok(())
}

/// Construct the three genuine independent numeric scenarios for linear selection.
/// # Errors
/// Preserves checked producer rejection without substituting a fit or interval.
fn linear_values(profile: &Profile) -> TestResult<[FixtureProjectedInterval; 3]> {
    let terms = [1.0 / 64.0, 1.0 / 64.0, 1.0 / 128.0];
    let far = [
        (98_000_000_000, 1_i128 << 33),
        (99_000_000_000, 1_i128 << 33),
        (100_000_000_000, 1_i128 << 33),
    ];
    Ok([
        genuine(
            profile,
            [41, 43, 47],
            &LINEAR_ROWS,
            2,
            102_000_000_000,
            terms,
        )?,
        genuine(
            profile,
            [53, 43, 47],
            &LINEAR_ROWS,
            0,
            102_000_000_000,
            terms,
        )?,
        genuine(
            profile,
            [59, 43, 47],
            &far,
            2,
            102_000_000_000,
            [1.0 / 64.0, 0.0, 0.0],
        )?,
    ])
}

/// Differently aged sources share the supplied instant and real selector.
/// # Errors
/// Preserves actual producer, batch or selector rejection.
/// # Panics
/// Rejects dropped sources, refreshed ages, minimum reduction or wrong witnesses.
#[test]
fn linear_sources_preserve_original_ages_and_configured_minimum() -> TestResult {
    let profile = Profile::new(
        "linear_selection",
        [3, 9, 9, 3, 16, 36, 9, 525, 3, 3, 3, 2, 2, 3],
    );
    let values = linear_values(&profile)?;
    let before = all_bits(&profile, &values)?;
    assert_eq!(
        [before[0].bounds, before[1].bounds, before[2].bounds],
        [LINEAR_BOUNDS, OLDER_BOUNDS, FAR_BOUNDS]
    );
    assert_eq!(before[0].projection.ages, [2_000_000_000, 2_000_000_000]);
    assert_eq!(before[1].projection.ages, [4_000_000_000, 2_000_000_000]);
    let header = expected(&profile, [43, 47, 102_000_000_000])?;
    let batch = profile.batch(header, &values)??;
    assert_batch_view(&profile, &batch, &values, &before, header)?;
    let two = profile.minimum(2)?;
    let three = profile.minimum(3)?;
    assert_eq!(
        selection_bits(&profile, &profile.select(&batch, two)?)?,
        witnessed(3, 2, 2, LINEAR_BOUNDS, &[41, 53])?
    );
    assert_eq!(
        selection_bits(&profile, &profile.select(&batch, three)?)?,
        unwitnessed(3, 3, 2, true, FixtureOutcomeKind::InsufficientAgreement)
    );
    let apart = [values[0], values[2]];
    let original = [before[0], before[2]];
    let apart_batch = profile.batch(header, &apart)??;
    assert_batch_view(&profile, &apart_batch, &apart, &original, header)?;
    assert_eq!(
        selection_bits(&profile, &profile.select(&apart_batch, two)?)?,
        unwitnessed(2, 2, 1, false, FixtureOutcomeKind::NoMajority)
    );
    assert_eq!(all_bits(&profile, &values)?, before);
    Ok(())
}

/// Assert admitted 0/15/16 prefixes against the unchanged minimum sixteen.
/// # Errors
/// Preserves actual consumer rejection or accounting exhaustion.
/// # Panics
/// Neither omitted entries nor a silently reduced minimum may satisfy the oracle.
fn admitted_capacity_neighbors(
    profile: &Profile,
    values: &[FixtureProjectedInterval],
    before: &[EnclosureBits],
    header: Expected,
) -> TestResult {
    if values.len() != 16 || before.len() != 16 {
        return Err(io::Error::other("capacity setup requires sixteen entries").into());
    }
    let minimum = profile.minimum(16)?;
    let empty = profile.batch(header, &values[..0])??;
    assert_batch_view(profile, &empty, &values[..0], &before[..0], header)?;
    assert_eq!(
        selection_bits(profile, &profile.select(&empty, minimum)?)?,
        unwitnessed(0, 16, 0, false, FixtureOutcomeKind::Empty)
    );
    let fifteen = profile.batch(header, &values[..15])??;
    assert_batch_view(profile, &fifteen, &values[..15], &before[..15], header)?;
    assert_eq!(
        selection_bits(profile, &profile.select(&fifteen, minimum)?)?,
        unwitnessed(15, 16, 15, true, FixtureOutcomeKind::InsufficientPopulation)
    );
    let sixteen = profile.batch(header, values)??;
    assert_batch_view(profile, &sixteen, values, before, header)?;
    let labels: Vec<u64> = (1..=16).collect();
    assert_eq!(
        selection_bits(profile, &profile.select(&sixteen, minimum)?)?,
        witnessed(16, 16, 16, CONSTANT_BOUNDS, &labels)?
    );
    Ok(())
}

/// Capacity neighbors retain all entries and validate the last occupied position.
/// # Errors
/// Rejects unexpected admission or preserves producer/consumer/accounting failure.
/// # Panics
/// Capacity wins for seventeen; a compatible prefix cannot hide index-fifteen failure.
#[test]
fn capacity_neighbors_keep_every_entry_and_the_original_minimum() -> TestResult {
    let profile = Profile::new(
        "capacity_neighbors",
        [17, 51, 51, 17, 86, 204, 51, 2234, 17, 17, 17, 5, 1, 3],
    );
    let mut values = Vec::with_capacity(16);
    for source in 1..=16 {
        values.push(constant(&profile, [source, 43, 47], 3_000_000_000)?);
    }
    let wrong = constant(&profile, [1, 44, 48], 3_000_000_001)?;
    let before = all_bits(&profile, &values)?;
    assert!(before.iter().all(|value| value.bounds == CONSTANT_BOUNDS));
    let header = expected(&profile, [43, 47, 3_000_000_000])?;
    admitted_capacity_neighbors(&profile, &values, &before, header)?;
    let first = *values
        .first()
        .ok_or_else(|| io::Error::other("missing capacity fixture"))?;
    let mut over = [first; 17];
    over[..16].copy_from_slice(&values);
    over[0] = wrong;
    let Err(error) = profile.batch(header, &over)? else {
        return Err(io::Error::other("seventeen entries must reject").into());
    };
    assert_eq!(error, FixtureProjectedBatchError::CapacityExceeded);
    assert!(profile.getter(|| error.source())?.is_none());
    let mut last_wrong = values.clone();
    last_wrong[15] = wrong;
    expect_mismatch(
        &profile,
        profile.batch(header, &last_wrong)?,
        15,
        FixtureProjectionMismatchKind::Frame,
    )?;
    assert_eq!(all_bits(&profile, &values)?, before);
    Ok(())
}

/// Build genuine same-number values with individual and combined context faults.
/// # Errors
/// Preserves checked producer admission, never forging projection context fields.
fn context_values(profile: &Profile) -> TestResult<[FixtureProjectedInterval; 6]> {
    Ok([
        constant(profile, [1, 43, 47], 3_000_000_000)?,
        constant(profile, [2, 44, 47], 3_000_000_000)?,
        constant(profile, [3, 43, 48], 3_000_000_000)?,
        constant(profile, [4, 43, 47], 3_000_000_001)?,
        constant(profile, [5, 44, 48], 3_000_000_001)?,
        constant(profile, [6, 43, 48], 3_000_000_001)?,
    ])
}

/// Challenge per-entry field priority and index priority using the same real values.
/// # Errors
/// Preserves accounting failure; every expected rejection enters the real consumer.
/// # Panics
/// Wrong field/index, unexpected success or an invented cause fails.
fn context_rejection_table(
    profile: &Profile,
    values: &[FixtureProjectedInterval; 6],
    header: Expected,
) -> TestResult {
    for (index, kind) in [
        (1, FixtureProjectionMismatchKind::Frame),
        (2, FixtureProjectionMismatchKind::MappingGeneration),
        (3, FixtureProjectionMismatchKind::EvaluationTime),
        (4, FixtureProjectionMismatchKind::Frame),
        (5, FixtureProjectionMismatchKind::MappingGeneration),
    ] {
        expect_mismatch(
            profile,
            profile.batch(header, &values[index..=index])?,
            0,
            kind,
        )?;
    }
    for (order, index, kind) in [
        (
            [2, 1, 0],
            0,
            FixtureProjectionMismatchKind::MappingGeneration,
        ),
        ([3, 5, 0], 0, FixtureProjectionMismatchKind::EvaluationTime),
        (
            [0, 2, 1],
            1,
            FixtureProjectionMismatchKind::MappingGeneration,
        ),
        ([1, 0, 2], 0, FixtureProjectionMismatchKind::Frame),
    ] {
        let input = [values[order[0]], values[order[1]], values[order[2]]];
        expect_mismatch(profile, profile.batch(header, &input)?, index, kind)?;
    }
    Ok(())
}

/// Adjacent high-origin instants stay distinct despite their equal floating aliases.
/// # Errors
/// Preserves actual producer, batch or accounting rejection.
/// # Panics
/// Equal interval numbers cannot hide the exact one-nanosecond mismatch.
fn high_origin_time_neighbor(profile: &Profile) -> TestResult {
    let rows = [
        (u64::MAX - 3_000_000_000, 1_i128 << 30),
        (u64::MAX - 2_000_000_000, 1_i128 << 30),
        (u64::MAX - 1_000_000_000, 1_i128 << 30),
    ];
    let values = [
        genuine(
            profile,
            [71, 43, 47],
            &rows,
            0,
            u64::MAX - 1,
            [1.0 / 64.0, 0.0, 0.0],
        )?,
        genuine(
            profile,
            [73, 43, 47],
            &rows,
            0,
            u64::MAX,
            [1.0 / 64.0, 0.0, 0.0],
        )?,
    ];
    let before = all_bits(profile, &values)?;
    assert_eq!([before[0].bounds, before[1].bounds], [CONSTANT_BOUNDS; 2]);
    assert_eq!(
        [
            before[0].projection.instants[3],
            before[1].projection.instants[3]
        ],
        [u64::MAX - 1, u64::MAX]
    );
    let header = expected(profile, [43, 47, u64::MAX - 1])?;
    let good = profile.batch(header, &values[..1])??;
    assert_batch_view(profile, &good, &values[..1], &before[..1], header)?;
    expect_mismatch(
        profile,
        profile.batch(header, &values[1..])?,
        0,
        FixtureProjectionMismatchKind::EvaluationTime,
    )?;
    assert_eq!(all_bits(profile, &values)?, before);
    Ok(())
}

/// Exact context relationships win by index, then frame/generation/evaluation.
/// # Errors
/// Preserves actual checked fixture or consumer failures.
/// # Panics
/// Equal numbers cannot bypass any relationship, including high-origin one-ns time.
#[test]
fn equal_numeric_intervals_still_require_exact_ordered_context_admission() -> TestResult {
    let profile = Profile::new(
        "context_admission",
        [8, 24, 24, 8, 42, 96, 24, 680, 8, 8, 8, 12, 1, 1],
    );
    let values = context_values(&profile)?;
    let before = all_bits(&profile, &values)?;
    assert!(before.iter().all(|value| value.bounds == CONSTANT_BOUNDS
        && value.projection.scalars[1] == (1.0_f64 / 8.0).to_bits()
        && value.projection.scalars[7] == (1.0_f64 / 64.0).to_bits()));
    let header = expected(&profile, [43, 47, 3_000_000_000])?;
    let good = profile.batch(header, &values[..1])??;
    assert_batch_view(&profile, &good, &values[..1], &before[..1], header)?;
    let minimum = profile.minimum(1)?;
    assert_eq!(
        selection_bits(&profile, &profile.select(&good, minimum)?)?,
        witnessed(1, 1, 1, CONSTANT_BOUNDS, &[1])?
    );
    context_rejection_table(&profile, &values, header)?;
    assert_eq!(all_bits(&profile, &values)?, before);
    high_origin_time_neighbor(&profile)?;
    Ok(())
}

/// Compatibility completes before actual duplicate-label admission.
/// # Errors
/// Rejects unexpected admission or preserves fixture, batch, minimum or selector rejection.
/// # Panics
/// Refuses per-index duplicate short-circuit, fabricated causes and silent dedup.
#[test]
fn context_failure_precedes_duplicates_and_real_duplicate_cause_is_preserved() -> TestResult {
    let profile = Profile::new(
        "duplicate_precedence",
        [3, 9, 9, 3, 16, 36, 9, 305, 3, 3, 3, 3, 1, 1],
    );
    let values = [
        constant(&profile, [41, 43, 47], 3_000_000_000)?,
        constant(&profile, [53, 43, 47], 3_000_000_000)?,
        constant(&profile, [59, 44, 47], 3_000_000_000)?,
    ];
    let before = all_bits(&profile, &values)?;
    let header = expected(&profile, [43, 47, 3_000_000_000])?;
    let incompatible = [values[0], values[0], values[2]];
    expect_mismatch(
        &profile,
        profile.batch(header, &incompatible)?,
        2,
        FixtureProjectionMismatchKind::Frame,
    )?;
    let duplicate = [values[0], values[1], values[0], values[1]];
    let Err(error) = profile.batch(header, &duplicate)? else {
        return Err(
            io::Error::other("compatible duplicate must reach actual batch admission").into(),
        );
    };
    let cause = BatchError::DuplicateLabel { later_index: 2 };
    assert_eq!(error, FixtureProjectedBatchError::Batch { source: cause });
    let source = profile
        .getter(|| error.source())?
        .ok_or_else(|| io::Error::other("missing actual batch cause"))?;
    assert_eq!(source.downcast_ref::<BatchError>(), Some(&cause));
    let good = profile.batch(header, &values[..2])??;
    assert_batch_view(&profile, &good, &values[..2], &before[..2], header)?;
    let minimum = profile.minimum(2)?;
    assert_eq!(
        selection_bits(&profile, &profile.select(&good, minimum)?)?,
        witnessed(2, 2, 2, CONSTANT_BOUNDS, &[41, 53])?
    );
    assert_eq!(all_bits(&profile, &values)?, before);
    Ok(())
}

/// Produce equal intervals with different anchors, terms, signs and source labels.
/// # Errors
/// Preserves actual checked producer rejection, with no raw output construction.
fn retention_values(profile: &Profile) -> TestResult<[FixtureProjectedInterval; 3]> {
    let rows = [
        (98_000_000_000, 5_i128 << 28),
        (99_000_000_000, 5_i128 << 28),
        (100_000_000_000, 5_i128 << 28),
    ];
    Ok([
        genuine(
            profile,
            [u64::MAX, 43, 47],
            &LINEAR_ROWS,
            2,
            102_000_000_000,
            [1.0 / 64.0, 1.0 / 64.0, 1.0 / 128.0],
        )?,
        genuine(
            profile,
            [7, 43, 47],
            &LINEAR_ROWS,
            0,
            102_000_000_000,
            [3.0 / 64.0, -0.0, 0.0],
        )?,
        genuine(
            profile,
            [41, 43, 47],
            &rows,
            2,
            102_000_000_000,
            [3.0 / 64.0, 0.0, -0.0],
        )?,
    ])
}

/// Permutations and repeated selections keep every original bit and borrowed entry.
/// # Errors
/// Preserves actual producer/consumer rejection or accounting failure.
/// # Panics
/// Refuses copying/reordering, normalized signed-zero terms or refreshed ages.
#[test]
fn permutations_and_repeated_selection_retain_the_original_complete_borrow() -> TestResult {
    let profile = Profile::new(
        "permutation_retention",
        [3, 9, 9, 3, 16, 36, 9, 618, 3, 3, 3, 3, 1, 6],
    );
    let values = retention_values(&profile)?;
    let before = all_bits(&profile, &values)?;
    assert!(before.iter().all(|value| value.bounds == LINEAR_BOUNDS));
    assert_eq!(
        [before[0].projection.ages[0], before[1].projection.ages[0]],
        [2_000_000_000, 4_000_000_000]
    );
    assert_eq!(before[1].projection.scalars[4], (-0.0_f64).to_bits());
    assert_eq!(before[2].projection.scalars[5], (-0.0_f64).to_bits());
    let header = expected(&profile, [43, 47, 102_000_000_000])?;
    let minimum = profile.minimum(3)?;
    for order in [[0, 1, 2], [2, 1, 0], [1, 2, 0]] {
        let input = [values[order[0]], values[order[1]], values[order[2]]];
        let original = [before[order[0]], before[order[1]], before[order[2]]];
        let batch = profile.batch(header, &input)??;
        assert_batch_view(&profile, &batch, &input, &original, header)?;
        for _ in 0..2 {
            let selected = profile.select(&batch, minimum)?;
            assert_eq!(
                selection_bits(&profile, &selected)?,
                witnessed(3, 3, 3, LINEAR_BOUNDS, &[7, 41, u64::MAX])?
            );
        }
    }
    assert_eq!(all_bits(&profile, &values)?, before);
    Ok(())
}

/// Outward quantization overlap remains a numerical fixture without trust claims.
/// # Errors
/// Preserves actual producer or new-consumer rejection.
/// # Panics
/// The known disjoint represented intervals must retain their original float bits.
#[test]
fn disjoint_subtick_inputs_keep_rounding_overlap_without_physical_claims() -> TestResult {
    let profile = Profile::new(
        "subtick_overlap",
        [2, 6, 6, 2, 11, 24, 6, 231, 2, 2, 2, 1, 1, 1],
    );
    let rows_a = [(0, -2), (1_000_000_000, -1), (2_000_000_000, 0)];
    let rows_b = [(0, -6), (1_000_000_000, -3), (2_000_000_000, 0)];
    let radius = f64::from_bits(987_u64 << 52);
    let values = [
        genuine(
            &profile,
            [61, 43, 47],
            &rows_a,
            2,
            2_250_000_000,
            [radius, 0.0, 0.0],
        )?,
        genuine(
            &profile,
            [67, 43, 47],
            &rows_b,
            2,
            2_250_000_000,
            [radius, 0.0, 0.0],
        )?,
    ];
    let before = all_bits(&profile, &values)?;
    // In Q33 ticks the original intervals are [1/8,3/8] and [5/8,7/8].
    assert_eq!(
        before[0].projection.scalars[1],
        f64::from_bits(988_u64 << 52).to_bits()
    );
    assert_eq!(
        before[1].projection.scalars[1],
        f64::from_bits((989_u64 << 52) | (1_u64 << 51)).to_bits()
    );
    assert!(before.iter().all(|value| value.bounds == [0, 1]
        && value.rounding == [FixtureEndpointRounding::OutwardFraction; 2]
        && value.projection.scalars[7] == radius.to_bits()));
    let header = expected(&profile, [43, 47, 2_250_000_000])?;
    let batch = profile.batch(header, &values)??;
    assert_batch_view(&profile, &batch, &values, &before, header)?;
    let minimum = profile.minimum(2)?;
    assert_eq!(
        selection_bits(&profile, &profile.select(&batch, minimum)?)?,
        witnessed(2, 2, 2, [0, 1], &[61, 67])?
    );
    assert_eq!(all_bits(&profile, &values)?, before);
    Ok(())
}

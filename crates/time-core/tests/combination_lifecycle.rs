// SPDX-License-Identifier: GPL-2.0-only
//! Twelve independent CHECK-7/8/14/25 lifecycle cases through real producers.
//!
//! UTC/exchange arithmetic supplies exact Q33 phases; real histories own fits,
//! original projections/enclosures own interval witnesses, and the real principal
//! owns update credit. No decision, mask, checked state or public overflow is
//! fabricated. All time is explicit fixture time, with no external effects.
//!
//! The authored public-suite upper is 57 histories, 232 append attempts,
//! 199 projections/enclosures, 88 batches, 31 seeds, 167 lifecycle advances,
//! 49 reachability constructors, 42 reachability records and 199 uncertainty
//! constructors. Each source has at most 47 rows; each test at most 98 advances.
//! Literal combination-parameter constructors total49:28 fixture constructions
//! and21 explicit raw-admission attempts. The generic seed-derived reservation
//! conservatively allows52 because the capacity case reuses immutable parameters
//! across four seed attempts. Private guard/setup bounds are recorded separately.
//! These are source reservations, not observed execution. An append reserves at
//! most one sufficient fit of 64 points; hence <=232 fits/14848 points. A
//! projection does not fit. Each advance reserves its real principal transition,
//! selector, <=16-source scans, <=16 compatibility visits and, when reached,
//! <=16*18+5 binary reduction calls plus three roots. Getter/equality calls have
//! separate precharged finite ceilings below. Actual prefixes emit on unwind.
//!
//! The exact per-test Limits are maintained beside the eleven selected test names.
//! Root owns admission, formatter/compiler/test execution and any later campaign.
#![forbid(unsafe_code)]

use std::{
    cell::Cell,
    error::Error,
    io::{self, Write},
    num::NonZeroU64,
};

use time_core::{
    ArithmeticIssue, ArithmeticOperation, ArithmeticStage, FitError,
    FixtureCombinationDenominatorField, FixtureCombinationError, FixtureCombinationLifecycleError,
    FixtureCombinationLifecycleStep, FixtureCombinationOutcome, FixtureCombinationParameterField,
    FixtureCombinationParameterIssue, FixtureCombinationParameters, FixtureCombinationSourceStatus,
    FixtureCombinationState, FixtureContext, FixtureHistory, FixtureMinimum, FixtureObservation,
    FixtureOutcomeKind, FixturePrincipalError, FixturePrincipalOutcomeKind,
    FixturePrincipalParameters, FixturePrincipalStratum, FixtureProjectedBatch,
    FixtureProjectedInterval, FixtureProjectionTerms, FixtureReachabilityUpdate,
    FixtureSupporterUncertainty, enclose_fixture_projection, project_fixture_anchor,
};
use time_domain::{MonotonicNs, PhaseOffsetQ33, UtcCoordinateQ32, exchange_arithmetic};

/// Harness/accounting errors remain distinct from actual product rejections.
type TestResult<T = ()> = Result<T, Box<dyn Error>>;
/// Number of individually named public-operation counters.
const COUNTERS: usize = 22;
/// Literal uncertainty assertion, separate from projection growth terms.
const STANDARD: [f64; 3] = [0.125, 1.0, 0.015_625];
/// Rounded reciprocal of the separately rounded square root of two.
const TWO_RATE_SD: u64 = 0x3fe6_a09e_667f_3bcc;
/// The same unfused inverse-root operation scaled by exactly 2^-6.
const TWO_SKEW: u64 = 0x3f86_a09e_667f_3bcc;

/// Closed source-owned operation names; none is a positional call-site literal.
#[derive(Clone, Copy)]
enum Call {
    /// Empty history construction.
    History,
    /// Real history append, including rejected attempts.
    Append,
    /// Checked observation construction.
    Observation,
    /// Checked fixture context construction.
    Context,
    /// Exact monotonic reading construction.
    Monotonic,
    /// Checked UTC coordinate construction.
    Coordinate,
    /// Real exchange arithmetic.
    Exchange,
    /// One ordinary immutable product accessor.
    Getter,
    /// Projection-term construction.
    Terms,
    /// Real same-history projection.
    Projection,
    /// Original projection enclosure.
    Enclosure,
    /// Checked borrowed projected batch.
    Batch,
    /// Configured fixture minimum construction.
    Minimum,
    /// Principal parameter construction.
    PrincipalParameters,
    /// Original source stratum construction.
    Stratum,
    /// Combination parameter construction.
    CombinationParameters,
    /// Real lifecycle state seed.
    Seed,
    /// Checked reachability update construction.
    ReachabilityUpdate,
    /// Actual reachability state transition.
    ReachabilityRecord,
    /// Real lifecycle advance.
    Advance,
    /// Checked original-entry uncertainty construction.
    Uncertainty,
    /// Bit-preserving immutable parameter or complete lifecycle-state comparison.
    Equality,
}

impl Call {
    /// Map the closed enum to its complete fixed counter inventory.
    const fn index(self) -> usize {
        match self {
            Self::History => 0,
            Self::Append => 1,
            Self::Observation => 2,
            Self::Context => 3,
            Self::Monotonic => 4,
            Self::Coordinate => 5,
            Self::Exchange => 6,
            Self::Getter => 7,
            Self::Terms => 8,
            Self::Projection => 9,
            Self::Enclosure => 10,
            Self::Batch => 11,
            Self::Minimum => 12,
            Self::PrincipalParameters => 13,
            Self::Stratum => 14,
            Self::CombinationParameters => 15,
            Self::Seed => 16,
            Self::ReachabilityUpdate => 17,
            Self::ReachabilityRecord => 18,
            Self::Advance => 19,
            Self::Uncertainty => 20,
            Self::Equality => 21,
        }
    }
}

/// Explicit finite major-call envelope for one authored trajectory.
#[derive(Clone, Copy)]
struct Limits {
    /// At most sixteen original sources per independently seeded fixture.
    histories: u64,
    /// Appends including the initial three observations per history.
    appends: u64,
    /// Complete original projections, each with one enclosure and stratum.
    projections: u64,
    /// Common-time builds, each with one batch and monotonic construction.
    batches: u64,
    /// Lifecycle owners, each with its own checked parameter/minimum construction.
    seeds: u64,
    /// Complete real advances across both candidate and healthy control owners.
    advances: u64,
    /// Update constructor attempts, including rejected ordinal zero.
    reachability_updates: u64,
    /// Actual record attempts, including contextual rejection.
    reachability_records: u64,
    /// Original uncertainty constructor attempts.
    uncertainties: u64,
    /// Maximum immutable parameter or whole-state comparisons.
    equalities: u64,
}

/// Independent finite raw-constructor/comparison envelope; no seed or fit.
#[derive(Clone, Copy)]
struct RawLimits {
    /// Actual raw combination-parameter constructor attempts.
    parameters: u64,
    /// Actual raw reachability-update constructor attempts.
    ordinals: u64,
    /// Direct immutable parameter comparisons, charged with the state upper.
    equalities: u64,
}

impl Limits {
    /// Expand the authored envelope without observing product work.
    /// Every observation group has fewer than 64 getter/check calls per charged
    /// append/projection/advance/reachability/seed; the extra group covers final
    /// fixed observers. No helper contains an unbounded product-call loop.
    const fn counters(self) -> [u64; COUNTERS] {
        let observers = 64
            * (self.appends
                + self.projections
                + self.advances
                + self.reachability_records
                + self.seeds
                + 1);
        [
            self.histories,
            self.appends,
            self.appends,
            self.histories,
            self.appends + self.batches,
            2 * self.appends,
            self.appends,
            observers,
            self.projections,
            self.projections,
            self.projections,
            self.batches,
            self.seeds,
            self.seeds,
            self.projections,
            self.seeds,
            self.seeds,
            self.reachability_updates,
            self.reachability_records,
            self.advances,
            self.uncertainties,
            self.equalities,
        ]
    }
}

/// Local actual-attempt profile with no shared/global generator or allowance.
struct Profile {
    /// Fixed selected test identity.
    case: &'static str,
    /// Complete authored ceilings before any product invocation.
    upper: [u64; COUNTERS],
    /// Actual attempts, charged before the corresponding call.
    actual: [Cell<u64>; COUNTERS],
}

impl Profile {
    /// Start the fixed scope before history or state construction.
    const fn new(case: &'static str, limits: Limits) -> Self {
        Self {
            case,
            upper: limits.counters(),
            actual: [const { Cell::new(0) }; COUNTERS],
        }
    }

    /// Reserve raw constructor calls independently of lifecycle seed work.
    /// The fixed raw case performs 24 immutable getters within the otherwise
    /// unused 64-getter observation group; it constructs no history or owner.
    fn raw(case: &'static str, limits: RawLimits) -> Self {
        let mut profile = Self::new(
            case,
            Limits {
                histories: 0,
                appends: 0,
                projections: 0,
                batches: 0,
                seeds: 0,
                advances: 0,
                reachability_updates: limits.ordinals,
                reachability_records: 0,
                uncertainties: 0,
                equalities: limits.equalities,
            },
        );
        profile.upper[Call::CombinationParameters.index()] = limits.parameters;
        profile
    }

    /// Precharge one operation and preserve its original result.
    /// # Errors
    /// Exhaustion or counter overflow rejects before entering the product.
    fn call<T>(&self, call: Call, action: impl FnOnce() -> T) -> TestResult<T> {
        let index = call.index();
        let next = self.actual[index]
            .get()
            .checked_add(1)
            .ok_or_else(|| io::Error::other("lifecycle fixture counter overflow"))?;
        if next > self.upper[index] {
            return Err(io::Error::other("lifecycle fixture ceiling exhausted").into());
        }
        self.actual[index].set(next);
        Ok(action())
    }

    /// Precharge one immutable product accessor.
    /// # Errors
    /// Preserve pre-call accounting failure without invoking the accessor.
    fn get<T>(&self, action: impl FnOnce() -> T) -> TestResult<T> {
        self.call(Call::Getter, action)
    }
}

impl Drop for Profile {
    /// Preserve actual attempted prefixes on normal return or assertion unwind.
    /// Emission failure is not interpreted as successful accounting evidence.
    fn drop(&mut self) {
        let names = [
            "histories",
            "appends",
            "observations",
            "contexts",
            "monotonic",
            "coordinates",
            "exchanges",
            "getters",
            "terms",
            "projections",
            "enclosures",
            "batches",
            "minima",
            "principal_parameters",
            "strata",
            "combination_parameters",
            "seeds",
            "reachability_updates",
            "reachability_records",
            "advances",
            "uncertainties",
            "equalities",
        ];
        let fields = names
            .iter()
            .zip(self.actual.iter())
            .map(|(name, value)| format!("{name}={}", value.get()))
            .collect::<Vec<_>>()
            .join(" ");
        let _emission = writeln!(
            io::stderr().lock(),
            "CORE_COMBINATION_LIFECYCLE_PROFILE case={} {fields}",
            self.case
        );
    }
}

/// Admit a literal nonzero harness tag without substituting a product identity.
/// # Errors
/// A zero harness tag fails before any product call.
fn tag(raw: u64) -> TestResult<NonZeroU64> {
    NonZeroU64::new(raw).ok_or_else(|| io::Error::other("zero lifecycle fixture tag").into())
}

/// Construct exact Q33 phase through checked UTC coordinates and exchange math.
/// # Errors
/// Preserve real constructor/exchange and pre-call accounting failures.
/// # Panics
/// The supported exchange must preserve the independently authored numerator.
fn phase(p: &Profile, numerator: i128) -> TestResult<PhaseOffsetQ33> {
    let zero = p.call(Call::Coordinate, || UtcCoordinateQ32::new(0))??;
    let shifted = p.call(Call::Coordinate, || UtcCoordinateQ32::new(numerator))??;
    let result = p.call(Call::Exchange, || {
        exchange_arithmetic(zero, shifted, zero, zero)
    })??;
    let phase = p.get(|| result.offset())?;
    assert_eq!(p.get(|| phase.numerator())?, numerator);
    Ok(phase)
}

/// One actual history with independently evolving observation time.
struct Source {
    /// Original literal source label.
    label: NonZeroU64,
    /// Original checked context, fixed for the complete trajectory.
    context: FixtureContext,
    /// Actual producer-owned rows/report; never reconstructed from a decision.
    history: FixtureHistory,
    /// Exact time of the latest successfully appended row.
    latest: MonotonicNs,
}

impl Source {
    /// Build exactly three real observations at 0/1/2 seconds.
    /// # Errors
    /// Preserve actual producer/accounting failures without unchecked values.
    fn new(p: &Profile, label: u64, numerators: [i128; 3]) -> TestResult<Self> {
        let label = tag(label)?;
        let frame = tag(7)?;
        let generation = tag(11)?;
        let context = p.call(Call::Context, || {
            FixtureContext::new(label, frame, generation)
        })?;
        let history = p.call(Call::History, || FixtureHistory::new(context))?;
        // The first append supplies the first actual reading before projection.
        let first = p.call(Call::Monotonic, || MonotonicNs::new(0))?;
        let mut source = Self {
            label,
            context,
            history,
            latest: first,
        };
        source.append_at(p, first, numerators[0])?;
        source.append(p, 1_000_000_000, numerators[1])?;
        source.append(p, 2_000_000_000, numerators[2])?;
        Ok(source)
    }

    /// Append one already checked exact reading and authored phase.
    /// # Errors
    /// Keep real admission/fit errors; only successful appends update the mirror.
    fn append_at(&mut self, p: &Profile, at: MonotonicNs, numerator: i128) -> TestResult {
        let phase = phase(p, numerator)?;
        let row = p.call(Call::Observation, || {
            FixtureObservation::new(self.context, at, phase, 1.0)
        })??;
        p.call(Call::Append, || self.history.try_append(row))??;
        self.latest = at;
        Ok(())
    }

    /// Append one genuinely later observation without changing common context.
    /// # Errors
    /// Preserve actual observation/history/accounting failures.
    fn append(&mut self, p: &Profile, raw_at: u64, numerator: i128) -> TestResult {
        let at = p.call(Call::Monotonic, || MonotonicNs::new(raw_at))?;
        self.append_at(p, at, numerator)
    }
}

/// Original immutable batch storage plus separately checked source strata.
struct Built {
    /// Complete ordered original entries, not copied from any selection result.
    entries: Vec<FixtureProjectedInterval>,
    /// Same-order checked stratum metadata.
    strata: Vec<FixturePrincipalStratum>,
    /// Original fixed literal roster.
    roster: Vec<NonZeroU64>,
    /// Shared exact evaluation reading.
    at: MonotonicNs,
}

impl Built {
    /// Project <=16 actual histories at a single explicit common instant.
    /// Growth terms are zero; separately asserted tracking skew never ages twice.
    /// # Errors
    /// Reject impossible harness capacity/anchor facts or real producer failure.
    fn new(p: &Profile, sources: &[(&Source, f64, u8)], raw_at: u64) -> TestResult<Self> {
        if sources.is_empty() || sources.len() > 16 {
            return Err(io::Error::other("lifecycle fixture source capacity").into());
        }
        let at = p.call(Call::Monotonic, || MonotonicNs::new(raw_at))?;
        let mut built = Self {
            entries: Vec::with_capacity(sources.len()),
            strata: Vec::with_capacity(sources.len()),
            roster: Vec::with_capacity(sources.len()),
            at,
        };
        for &(source, distance, stratum) in sources {
            let count = p.get(|| source.history.sample_count())?;
            let anchor = count
                .checked_sub(1)
                .ok_or_else(|| io::Error::other("missing real fixture anchor"))?;
            let terms = p.call(Call::Terms, || {
                FixtureProjectionTerms::new(distance, 0.0, 0.0)
            })??;
            let projection = p.call(Call::Projection, || {
                project_fixture_anchor(
                    &source.history,
                    source.context,
                    anchor,
                    source.latest,
                    at,
                    terms,
                )
            })??;
            built
                .entries
                .push(p.call(Call::Enclosure, || enclose_fixture_projection(&projection))??);
            built.strata.push(p.call(Call::Stratum, || {
                FixturePrincipalStratum::new(source.label, stratum)
            })??);
            built.roster.push(source.label);
        }
        Ok(built)
    }

    /// Borrow these exact original entries into the real checked batch.
    /// # Errors
    /// Preserve actual batch admission/accounting failures.
    fn batch(&self, p: &Profile) -> TestResult<FixtureProjectedBatch<'_>> {
        let frame = tag(7)?;
        let generation = tag(11)?;
        Ok(p.call(Call::Batch, || {
            FixtureProjectedBatch::new(frame, generation, self.at, &self.entries)
        })??)
    }
}

/// Seed an actual lifecycle owner with independently configured minimum.
/// # Errors
/// Preserve real parameter/minimum/seed failures, with no public unchecked state.
fn seed(
    p: &Profile,
    built: &Built,
    batch: &FixtureProjectedBatch<'_>,
    minimum: usize,
    principal: [f64; 2],
) -> TestResult<FixtureCombinationState> {
    seed_with_combination(p, built, batch, minimum, principal, [2.0, 0.03125])
}

/// Seed one real owner with explicit immutable fixture parameter assertions.
/// # Errors
/// Preserve the same parameter/minimum/seed failures as the ordinary seed helper.
fn seed_with_combination(
    p: &Profile,
    built: &Built,
    batch: &FixtureProjectedBatch<'_>,
    minimum: usize,
    principal: [f64; 2],
    combination: [f64; 2],
) -> TestResult<FixtureCombinationState> {
    let parameters = p.call(Call::PrincipalParameters, || {
        FixturePrincipalParameters::new(principal[0], principal[1])
    })??;
    let combination = p.call(Call::CombinationParameters, || {
        FixtureCombinationParameters::new(combination[0], combination[1])
    })??;
    let minimum = p.call(Call::Minimum, || FixtureMinimum::new(minimum))??;
    Ok(p.call(Call::Seed, || {
        FixtureCombinationState::seed(
            &built.roster,
            parameters,
            combination,
            minimum,
            batch,
            &built.strata,
        )
    })??)
}

/// Admit original-entry uncertainty records for the authored original prefix.
/// A shorter prefix is used only for actual outside-witness/absent-principal facts.
/// # Errors
/// Reject impossible harness length and preserve real constructor failures.
fn records<'a>(
    p: &Profile,
    built: &'a Built,
    raw: &[[f64; 3]],
) -> TestResult<Vec<FixtureSupporterUncertainty<'a>>> {
    if raw.len() > built.entries.len() {
        return Err(io::Error::other("lifecycle fixture metadata prefix").into());
    }
    let mut values = Vec::with_capacity(raw.len());
    for (entry, values_raw) in built.entries.iter().zip(raw) {
        values.push(p.call(Call::Uncertainty, || {
            FixtureSupporterUncertainty::new(entry, values_raw[0], values_raw[1], values_raw[2])
        })??);
    }
    Ok(values)
}

/// Complete independent expected round observation.
#[derive(Clone, Copy)]
struct ExpectedRound {
    /// Actual real-principal result kind.
    kind: FixturePrincipalOutcomeKind,
    /// Expected current principal, independently authored.
    principal: Option<u64>,
    /// Genuine witness size, absent only without a witness.
    witness: Option<usize>,
    /// Mathematical contributors, absent without a new combination.
    contributors: Option<usize>,
    /// New emission ordinal, absent for all nonready outcomes.
    ordinal: Option<u64>,
}

/// Inspect round/kind/count/authority separation without causing transitions.
/// # Errors
/// Preserve only pre-call accounting failure.
/// # Panics
/// All independently expected fields and estimate presence must match.
fn observe_round(
    p: &Profile,
    outcome: &FixtureCombinationOutcome<'_>,
    expected: ExpectedRound,
) -> TestResult {
    let principal = p.get(|| outcome.principal())?;
    assert_eq!(p.get(|| principal.kind())?, expected.kind);
    assert_eq!(
        p.get(|| principal.principal_label())?.map(NonZeroU64::get),
        expected.principal
    );
    assert_eq!(
        p.get(|| principal.reference_ready())?,
        expected.ordinal.is_some()
    );
    assert_eq!(
        p.get(|| outcome.emitted_round())?.map(NonZeroU64::get),
        expected.ordinal
    );
    assert_eq!(p.get(|| outcome.witness_count())?, expected.witness);
    assert_eq!(
        p.get(|| outcome.contributor_count())?,
        expected.contributors
    );
    assert_eq!(
        p.get(|| outcome.combined())?.is_some(),
        expected.ordinal.is_some()
    );
    Ok(())
}

/// Expected ready A round with independently stated actual contributor count.
fn ready(ordinal: u64, contributors: usize, witness: usize) -> ExpectedRound {
    ready_for(101, ordinal, contributors, witness)
}

/// Expected ready round with an independently authored principal identity.
fn ready_for(principal: u64, ordinal: u64, contributors: usize, witness: usize) -> ExpectedRound {
    ExpectedRound {
        kind: FixturePrincipalOutcomeKind::ReferenceReady,
        principal: Some(principal),
        witness: Some(witness),
        contributors: Some(contributors),
        ordinal: Some(ordinal),
    }
}

/// Expected retained A round without mathematical contributor fabrication.
fn retained(witness: usize) -> ExpectedRound {
    ExpectedRound {
        kind: FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate,
        principal: Some(101),
        witness: Some(witness),
        contributors: None,
        ordinal: None,
    }
}

/// Independent complete scalar bits plus original context and contributor labels.
/// # Errors
/// Preserve accounting failure and reject an unexpectedly absent estimate.
/// # Panics
/// All five scalar bits, labels/count and original context must match.
fn estimate(
    p: &Profile,
    outcome: &FixtureCombinationOutcome<'_>,
    built: &Built,
    expected: [u64; 5],
    labels: &[u64],
) -> TestResult {
    let value = p
        .get(|| outcome.combined())?
        .ok_or_else(|| io::Error::other("missing lifecycle estimate"))?;
    let actual = [
        p.get(|| value.phase_seconds())?.to_bits(),
        p.get(|| value.phase_rate_seconds_per_second())?.to_bits(),
        p.get(|| value.phase_standard_deviation_seconds())?
            .to_bits(),
        p.get(|| value.rate_standard_deviation_seconds_per_second())?
            .to_bits(),
        p.get(|| value.tracking_skew_seconds_per_second())?
            .to_bits(),
    ];
    assert_eq!(actual, expected);
    assert_eq!(p.get(|| value.supporter_count())?, labels.len());
    let labels_actual = p
        .get(|| value.supporters())?
        .map(NonZeroU64::get)
        .collect::<Vec<_>>();
    assert_eq!(labels_actual, labels);
    assert_eq!(p.get(|| value.frame())?, tag(7)?);
    assert_eq!(p.get(|| value.mapping_generation())?, tag(11)?);
    assert_eq!(p.get(|| value.projected_at())?, built.at);
    Ok(())
}

/// Read-only source facts, including event watermark after maturity saturation.
#[derive(Debug, Eq, PartialEq)]
struct SourceView {
    /// Reachability-update count saturated at eight.
    maturity: u8,
    /// Current compatibility penalty.
    penalty: u8,
    /// Last accepted exact-next ordinal, which does not saturate with maturity.
    ordinal: u64,
}

/// Observe the actual original source slot, never mutate through a test seam.
/// # Errors
/// Preserve accounting failure or report a missing expected registered source.
/// # Panics
/// The registered slot must preserve its original source label.
fn source_view(p: &Profile, state: &FixtureCombinationState, label: u64) -> TestResult<SourceView> {
    let label = tag(label)?;
    let source = p
        .get(|| state.source_state(label))?
        .ok_or_else(|| io::Error::other("missing lifecycle source"))?;
    assert_eq!(p.get(|| source.source_id())?, label);
    Ok(SourceView {
        maturity: p.get(|| source.reachability_updates())?,
        penalty: p.get(|| source.penalty())?,
        ordinal: p.get(|| source.last_reachability_ordinal())?,
    })
}

/// Observe principal-owned pending credit through its real immutable view.
/// # Errors
/// Preserve accounting failure or report a missing expected source.
fn pending(p: &Profile, state: &FixtureCombinationState, label: u64) -> TestResult<bool> {
    let principal = p.get(|| state.principal_state())?;
    let label = tag(label)?;
    let source = p
        .get(|| principal.source_state(label))?
        .ok_or_else(|| io::Error::other("missing principal source"))?;
    p.get(|| source.pending())
}

/// Observe one fixed roster status without constructing a new transition.
/// # Errors
/// Preserve only literal-tag or pre-call accounting failures.
fn status(
    p: &Profile,
    outcome: &FixtureCombinationOutcome<'_>,
    label: u64,
) -> TestResult<Option<FixtureCombinationSourceStatus>> {
    let label = tag(label)?;
    p.get(|| outcome.source_status(label))
}

/// Apply an actual explicit reachability event with precharged constructor/record.
/// # Errors
/// Preserve real ordinal/admission errors rather than converting them to success.
fn reach(
    p: &Profile,
    state: &mut FixtureCombinationState,
    label: u64,
    ordinal: u64,
    reachable: bool,
) -> TestResult {
    let label = tag(label)?;
    let update = p.call(Call::ReachabilityUpdate, || {
        FixtureReachabilityUpdate::new(label, ordinal, reachable)
    })??;
    p.call(Call::ReachabilityRecord, || {
        state.record_reachability(update)
    })??;
    Ok(())
}

/// Two independently seeded public owners provide bit-exact rollback truth.
struct Owners {
    /// Candidate receiving the rejection probes.
    state: FixtureCombinationState,
    /// Healthy parallel owner that never receives rejected advances.
    control: FixtureCombinationState,
}

impl Owners {
    /// Construct two independent owners against the same actual original batch.
    /// # Errors
    /// Preserve real seed/parameter/admission failures.
    fn new(p: &Profile, built: &Built, batch: &FixtureProjectedBatch<'_>) -> TestResult<Self> {
        Self::with_combination(p, built, batch, [2.0, 0.03125])
    }

    /// Independently seed two owners with the same fixed explicit parameters.
    /// # Errors
    /// Preserve actual constructor/admission failures without cloning live state.
    fn with_combination(
        p: &Profile,
        built: &Built,
        batch: &FixtureProjectedBatch<'_>,
        combination: [f64; 2],
    ) -> TestResult<Self> {
        Ok(Self {
            state: seed_with_combination(p, built, batch, 2, [0.125, 0.0], combination)?,
            control: seed_with_combination(p, built, batch, 2, [0.125, 0.0], combination)?,
        })
    }

    /// Compare complete owner state; no public Clone or partial observer oracle.
    /// # Errors
    /// Preserve accounting failure before comparison.
    /// # Panics
    /// Candidate must equal the independently advanced healthy owner bit for bit.
    fn same(&self, p: &Profile) -> TestResult {
        p.call(Call::Equality, || assert_eq!(self.state, self.control))
    }

    /// Apply one successful genuine event separately to both complete owners.
    /// # Errors
    /// Preserve real advance/admission failures; assertions never create state.
    fn advance<'b>(
        &mut self,
        p: &Profile,
        batch: &'b FixtureProjectedBatch<'_>,
        strata: &[FixturePrincipalStratum],
        uncertainty: &[FixtureSupporterUncertainty<'_>],
    ) -> TestResult<FixtureCombinationOutcome<'b>> {
        let result = p.call(Call::Advance, || {
            self.state.advance(batch, strata, uncertainty)
        })??;
        let _control_result = p.call(Call::Advance, || {
            self.control.advance(batch, strata, uncertainty)
        })??;
        self.same(p)?;
        Ok(result)
    }

    /// Record the same accepted explicit reachability event in both owners.
    /// # Errors
    /// Preserve actual event errors and compare full independently owned states.
    fn reach(&mut self, p: &Profile, ordinal: u64, reachable: bool) -> TestResult {
        reach(p, &mut self.state, 202, ordinal, reachable)?;
        reach(p, &mut self.control, 202, ordinal, reachable)?;
        self.same(p)
    }
}

/// One authored common-time two-source event expected to be ready or retained.
#[derive(Clone, Copy)]
struct PairFact {
    /// Explicit common evaluation instant in nanoseconds.
    at: u64,
    /// Explicit B distance at that common instant.
    distance: f64,
    /// Actual post-event compatibility penalty.
    penalty: u8,
    /// New ordinal, or no new principal-ready event.
    round: Option<u64>,
}

/// Project real histories, advance both owners, then inspect one independent fact.
/// # Errors
/// Preserve actual producer/admission failures with no subset/decision forgery.
/// # Panics
/// The independently specified round and post-event penalty must match.
fn pair_round(
    p: &Profile,
    owners: &mut Owners,
    sources: [&Source; 2],
    fact: PairFact,
) -> TestResult {
    let built = Built::new(
        p,
        &[(sources[0], 0.125, 1), (sources[1], fact.distance, 1)],
        fact.at,
    )?;
    let batch = built.batch(p)?;
    let metadata = records(p, &built, &[STANDARD; 2])?;
    let outcome = owners.advance(p, &batch, &built.strata, &metadata)?;
    let expected = if let Some(round) = fact.round {
        ready(round, if fact.penalty == 0 { 2 } else { 1 }, 2)
    } else {
        retained(2)
    };
    observe_round(p, &outcome, expected)?;
    assert_eq!(source_view(p, &owners.state, 202)?.penalty, fact.penalty);
    Ok(())
}

/// Exercise the three strict distance neighbors with the original principal.
/// # Errors
/// Preserve the original producer/admission and pre-call accounting failures.
/// # Panics
/// Every unchanged independent trajectory assertion must hold.
fn distance_boundary_neighbors(p: &Profile) -> TestResult {
    for distance in [31.0 / 64.0, 0.5, 33.0 / 64.0] {
        let a = Source::new(p, 101, [0; 3])?;
        let b = Source::new(p, 202, [0; 3])?;
        let built = Built::new(p, &[(&a, 0.125, 2), (&b, distance, 1)], 2_000_000_000)?;
        let batch = built.batch(p)?;
        let mut state = seed(p, &built, &batch, 2, [0.125, 0.125])?;
        let metadata = records(p, &built, &[STANDARD; 2])?;
        let outcome = p.call(Call::Advance, || {
            state.advance(&batch, &built.strata, &metadata)
        })??;
        let compatible = distance <= 0.5;
        observe_round(p, &outcome, ready(1, if compatible { 2 } else { 1 }, 2))?;
        assert_eq!(
            status(p, &outcome, 202)?,
            Some(if compatible {
                FixtureCombinationSourceStatus::Contributor
            } else {
                FixtureCombinationSourceStatus::DistanceExcluded
            })
        );
        estimate(
            p,
            &outcome,
            &built,
            [
                0,
                0,
                0x3fc0_0000_0000_0000,
                if compatible {
                    TWO_RATE_SD
                } else {
                    0x3ff0_0000_0000_0000
                },
                if compatible {
                    TWO_SKEW
                } else {
                    0x3f90_0000_0000_0000
                },
            ],
            if compatible { &[101, 202] } else { &[101] },
        )?;
    }
    Ok(())
}

/// Exercise both signs of the three strict original-principal rate neighbors.
/// # Errors
/// Preserve the original producer/admission and pre-call accounting failures.
/// # Panics
/// Every unchanged independent trajectory assertion must hold.
fn rate_boundary_neighbors(p: &Profile) -> TestResult {
    for (signed_k, rate_bits, combined_bits) in [
        (7_i128, 0x3fbc_0000_0000_0000, 0x3fac_0000_0000_0000),
        (8, 0x3fc0_0000_0000_0000, 0x3fb0_0000_0000_0000),
        (9, 0x3fc2_0000_0000_0000, 0),
        (-7, 0xbfbc_0000_0000_0000, 0xbfac_0000_0000_0000),
        (-8, 0xbfc0_0000_0000_0000, 0xbfb0_0000_0000_0000),
        (-9, 0xbfc2_0000_0000_0000, 0),
    ] {
        let a = Source::new(p, 101, [0; 3])?;
        let unit = signed_k * (1_i128 << 27);
        let b = Source::new(p, 202, [-2 * unit, -unit, 0])?;
        let built = Built::new(p, &[(&a, 0.125, 1), (&b, 0.25, 1)], 2_000_000_000)?;
        let projection = p.get(|| built.entries[1].projection())?;
        assert_eq!(p.get(|| projection.center_seconds())?.to_bits(), 0);
        assert_eq!(
            p.get(|| projection.phase_rate_seconds_per_second())?
                .to_bits(),
            rate_bits
        );
        let batch = built.batch(p)?;
        let mut state = seed(p, &built, &batch, 2, [0.125, 0.0])?;
        let metadata = records(p, &built, &[STANDARD; 2])?;
        let outcome = p.call(Call::Advance, || {
            state.advance(&batch, &built.strata, &metadata)
        })??;
        let compatible = signed_k.abs() <= 8;
        observe_round(p, &outcome, ready(1, if compatible { 2 } else { 1 }, 2))?;
        assert_eq!(
            status(p, &outcome, 202)?,
            Some(if compatible {
                FixtureCombinationSourceStatus::Contributor
            } else {
                FixtureCombinationSourceStatus::RateExcluded
            })
        );
        estimate(
            p,
            &outcome,
            &built,
            [
                0,
                combined_bits,
                0x3fc0_0000_0000_0000,
                if compatible {
                    TWO_RATE_SD
                } else {
                    0x3ff0_0000_0000_0000
                },
                if compatible {
                    TWO_SKEW
                } else {
                    0x3f90_0000_0000_0000
                },
            ],
            if compatible { &[101, 202] } else { &[101] },
        )?;
    }
    Ok(())
}

/// Reach maturity through real events and preserve complete state on rejected ordinals.
/// # Errors
/// Preserve the original producer/admission and pre-call accounting failures.
/// # Panics
/// Every unchanged independent trajectory assertion must hold.
fn maturity_and_event_order(
    p: &Profile,
    owners: &mut Owners,
    a: &mut Source,
    b: &Source,
    built: &Built,
    batch: &FixtureProjectedBatch<'_>,
) -> TestResult {
    for ordinal in 1..=7 {
        owners.reach(p, ordinal, ordinal % 2 == 0)?;
    }
    assert_eq!(
        source_view(p, &owners.state, 202)?,
        SourceView {
            maturity: 7,
            penalty: 0,
            ordinal: 7
        }
    );
    let metadata = records(p, built, &[STANDARD; 2])?;
    let initial = owners.advance(p, batch, &built.strata, &metadata)?;
    observe_round(p, &initial, ready(1, 1, 2))?;
    assert_eq!(source_view(p, &owners.state, 202)?.penalty, 1);
    owners.reach(p, 8, false)?;
    a.append(p, 3_000_000_000, 0)?;
    pair_round(
        p,
        owners,
        [a, b],
        PairFact {
            at: 3_000_000_000,
            distance: 33.0 / 64.0,
            penalty: 32,
            round: Some(2),
        },
    )?;
    owners.reach(p, 9, true)?;
    assert_eq!(
        source_view(p, &owners.state, 202)?,
        SourceView {
            maturity: 8,
            penalty: 32,
            ordinal: 9
        }
    );
    for (label, ordinal) in [(202, 9), (202, 8), (202, 11), (999, 1)] {
        let label = tag(label)?;
        let update = p.call(Call::ReachabilityUpdate, || {
            FixtureReachabilityUpdate::new(label, ordinal, true)
        })??;
        let rejected = p.call(Call::ReachabilityRecord, || {
            owners.state.record_reachability(update)
        })?;
        assert!(matches!(
            rejected,
            Err(FixtureCombinationLifecycleError::ReachabilityOrder
                | FixtureCombinationLifecycleError::UnknownSource)
        ));
        owners.same(p)?;
    }
    let b_label = tag(202)?;
    let ordinal_zero = p.call(Call::ReachabilityUpdate, || {
        FixtureReachabilityUpdate::new(b_label, 0, true)
    })?;
    assert!(matches!(
        ordinal_zero,
        Err(FixtureCombinationLifecycleError::InvalidReachabilityOrdinal)
    ));
    Ok(())
}

/// Run the first ten compatible rounds, retained pause and fresh mature relapse.
/// # Errors
/// Preserve the original producer/admission and pre-call accounting failures.
/// # Panics
/// Every unchanged independent trajectory assertion must hold.
fn mature_penalty_relapse(
    p: &Profile,
    owners: &mut Owners,
    a: &mut Source,
    b: &mut Source,
) -> TestResult {
    b.append(p, 4_000_000_000, 0)?;
    pair_round(
        p,
        owners,
        [a, b],
        PairFact {
            at: 4_000_000_000,
            distance: 0.25,
            penalty: 32,
            round: None,
        },
    )?;
    for index in 0..10_u64 {
        let at = (5 + index) * 1_000_000_000;
        a.append(p, at, 0)?;
        pair_round(
            p,
            owners,
            [a, b],
            PairFact {
                at,
                distance: 0.25,
                penalty: u8::try_from(31 - index)?,
                round: Some(3 + index),
            },
        )?;
        if index == 0 {
            // A retained evaluation cannot spend one of the mature countdowns.
            let pause = Built::new(p, &[(a, 0.125, 1), (b, 0.25, 1)], at)?;
            let pause_batch = pause.batch(p)?;
            let pause_metadata = records(p, &pause, &[STANDARD; 2])?;
            let retained_outcome =
                owners.advance(p, &pause_batch, &pause.strata, &pause_metadata)?;
            observe_round(p, &retained_outcome, retained(2))?;
            assert_eq!(source_view(p, &owners.state, 202)?.penalty, 31);
        }
    }
    assert_eq!(source_view(p, &owners.state, 202)?.penalty, 22);
    b.append(p, 15_000_000_000, 0)?;
    pair_round(
        p,
        owners,
        [a, b],
        PairFact {
            at: 15_000_000_000,
            distance: 33.0 / 64.0,
            penalty: 22,
            round: None,
        },
    )?;
    a.append(p, 16_000_000_000, 0)?;
    pair_round(
        p,
        owners,
        [a, b],
        PairFact {
            at: 16_000_000_000,
            distance: 33.0 / 64.0,
            penalty: 32,
            round: Some(13),
        },
    )?;
    b.append(p, 17_000_000_000, 0)?;
    pair_round(
        p,
        owners,
        [a, b],
        PairFact {
            at: 17_000_000_000,
            distance: 0.25,
            penalty: 32,
            round: None,
        },
    )?;
    Ok(())
}

/// Lose real majority, then restore a witness without inventing principal credit.
/// # Errors
/// Preserve the original producer/admission and pre-call accounting failures.
/// # Panics
/// Every unchanged independent trajectory assertion must hold.
fn disagreement_then_waiting(
    p: &Profile,
    state: &mut FixtureCombinationState,
    a: &Source,
    b: &mut Source,
    c: &Source,
) -> TestResult {
    b.append(p, 3_000_000_000, 1_i128 << 31)?;
    let lost = Built::new(
        p,
        &[(a, 0.03125, 1), (b, 0.0625, 1), (c, 0.03125, 1)],
        3_000_000_000,
    )?;
    let lost_batch = lost.batch(p)?;
    let unavailable = p.call(Call::Advance, || {
        state.advance(&lost_batch, &lost.strata, &[])
    })??;
    observe_round(
        p,
        &unavailable,
        ExpectedRound {
            kind: FixturePrincipalOutcomeKind::Unavailable,
            principal: None,
            witness: None,
            contributors: None,
            ordinal: None,
        },
    )?;
    let principal = p.get(|| unavailable.principal())?;
    let selection = p.get(|| principal.selection())?;
    assert_eq!(
        p.get(|| selection.outcome())?,
        FixtureOutcomeKind::NoMajority
    );
    assert_eq!(p.get(|| selection.minimum())?, 2);
    assert_eq!(source_view(p, state, 101)?.penalty, 0);
    assert_eq!(source_view(p, state, 202)?.penalty, 0);
    assert_eq!(source_view(p, state, 303)?.penalty, 32);
    b.append(p, 4_000_000_000, 0)?;
    let waiting = Built::new(
        p,
        &[(a, 0.03125, 1), (b, 0.0625, 1), (c, 0.03125, 1)],
        4_000_000_000,
    )?;
    let waiting_batch = waiting.batch(p)?;
    let wait = p.call(Call::Advance, || {
        state.advance(&waiting_batch, &waiting.strata, &[])
    })??;
    observe_round(
        p,
        &wait,
        ExpectedRound {
            kind: FixturePrincipalOutcomeKind::WaitingForUpdate,
            principal: None,
            witness: Some(2),
            contributors: None,
            ordinal: None,
        },
    )?;
    assert_eq!(source_view(p, state, 303)?.penalty, 32);
    Ok(())
}

/// Return an outside-witness source and count down only on genuine principal credit.
/// # Errors
/// Preserve the original producer/admission and pre-call accounting failures.
/// # Panics
/// Every unchanged independent trajectory assertion must hold.
fn return_penalized_source(
    p: &Profile,
    state: &mut FixtureCombinationState,
    a: &mut Source,
    b: &Source,
    c: &mut Source,
    raw: &[[f64; 3]; 3],
) -> TestResult {
    c.append(p, 5_000_000_000, 0)?;
    let returning = Built::new(
        p,
        &[(a, 0.03125, 1), (b, 0.0625, 1), (c, 0.03125, 1)],
        5_000_000_000,
    )?;
    let returning_batch = returning.batch(p)?;
    let returning_metadata = records(p, &returning, raw)?;
    let pause = p.call(Call::Advance, || {
        state.advance(&returning_batch, &returning.strata, &returning_metadata)
    })??;
    observe_round(p, &pause, retained(3))?;
    assert_eq!(source_view(p, state, 303)?.penalty, 32);
    a.append(p, 5_000_000_000, 0)?;
    let fresh = Built::new(
        p,
        &[(a, 0.03125, 1), (b, 0.0625, 1), (c, 0.03125, 1)],
        5_000_000_000,
    )?;
    let fresh_batch = fresh.batch(p)?;
    let fresh_metadata = records(p, &fresh, raw)?;
    let returning_round = p.call(Call::Advance, || {
        state.advance(&fresh_batch, &fresh.strata, &fresh_metadata)
    })??;
    observe_round(p, &returning_round, ready(3, 2, 3))?;
    assert_eq!(
        source_view(p, state, 303)?,
        SourceView {
            maturity: 0,
            penalty: 31,
            ordinal: 0
        }
    );
    assert_eq!(
        status(p, &returning_round, 303)?,
        Some(FixtureCombinationSourceStatus::PenaltyCountdown)
    );
    Ok(())
}

/// Mature and penalize B/C, then preserve B's real pending credit on a retained A round.
/// # Errors
/// Preserve the original producer/admission and pre-call accounting failures.
/// # Panics
/// Every unchanged independent trajectory assertion must hold.
fn prepare_penalized_replacement(
    p: &Profile,
    state: &mut FixtureCombinationState,
    a: &Source,
    b: &mut Source,
    c: &Source,
    initial: &Built,
    initial_batch: &FixtureProjectedBatch<'_>,
) -> TestResult {
    for label in [202, 303] {
        for ordinal in 1..=8 {
            reach(p, state, label, ordinal, ordinal % 2 == 0)?;
        }
    }
    let initial_metadata = records(p, initial, &[STANDARD; 3])?;
    let first = p.call(Call::Advance, || {
        state.advance(initial_batch, &initial.strata, &initial_metadata)
    })??;
    observe_round(p, &first, ready_for(101, 1, 1, 3))?;
    assert_eq!(
        source_view(p, state, 202)?,
        SourceView {
            maturity: 8,
            penalty: 32,
            ordinal: 8,
        }
    );
    assert_eq!(
        source_view(p, state, 303)?,
        SourceView {
            maturity: 8,
            penalty: 32,
            ordinal: 8,
        }
    );
    b.append(p, 3_000_000_000, 0)?;
    let waiting = Built::new(
        p,
        &[(a, 0.125, 1), (b, 0.125, 1), (c, 33.0 / 64.0, 1)],
        3_000_000_000,
    )?;
    let waiting_batch = waiting.batch(p)?;
    let waiting_metadata = records(p, &waiting, &[STANDARD; 3])?;
    let retained_a = p.call(Call::Advance, || {
        state.advance(&waiting_batch, &waiting.strata, &waiting_metadata)
    })??;
    observe_round(p, &retained_a, retained(3))?;
    assert_eq!(source_view(p, state, 202)?.penalty, 32);
    assert_eq!(source_view(p, state, 303)?.penalty, 32);
    assert!(pending(p, state, 202)?);
    assert!(!pending(p, state, 101)?);
    Ok(())
}

/// Challenge both raw parameter fields and retain constructor error precedence.
/// # Errors
/// Preserve the original producer/admission and pre-call accounting failures.
/// # Panics
/// Every unchanged independent trajectory assertion must hold.
fn reject_raw_parameter_neighbors(p: &Profile) -> TestResult {
    for (raw, issue) in [
        (-1.0, FixtureCombinationParameterIssue::Negative),
        (f64::NAN, FixtureCombinationParameterIssue::NonFinite),
        (f64::INFINITY, FixtureCombinationParameterIssue::NonFinite),
        (
            f64::NEG_INFINITY,
            FixtureCombinationParameterIssue::NonFinite,
        ),
        (
            f64::from_bits(1),
            FixtureCombinationParameterIssue::Subnormal,
        ),
        (
            f64::from_bits(0x8000_0000_0000_0001),
            FixtureCombinationParameterIssue::Subnormal,
        ),
        (0.0, FixtureCombinationParameterIssue::NonPositive),
        (-0.0, FixtureCombinationParameterIssue::NonPositive),
    ] {
        let rejected = p.call(Call::CombinationParameters, || {
            FixtureCombinationParameters::new(raw, 0.03125)
        })?;
        assert!(
            matches!(rejected, Err(FixtureCombinationLifecycleError::InvalidParameter {
            field: FixtureCombinationParameterField::CombineLimit, issue: actual,
        }) if actual == issue)
        );
    }
    for (raw, issue) in [
        (-1.0, FixtureCombinationParameterIssue::Negative),
        (f64::NAN, FixtureCombinationParameterIssue::NonFinite),
        (f64::INFINITY, FixtureCombinationParameterIssue::NonFinite),
        (
            f64::NEG_INFINITY,
            FixtureCombinationParameterIssue::NonFinite,
        ),
        (
            f64::from_bits(1),
            FixtureCombinationParameterIssue::Subnormal,
        ),
        (
            f64::from_bits(0x8000_0000_0000_0001),
            FixtureCombinationParameterIssue::Subnormal,
        ),
    ] {
        let rejected = p.call(Call::CombinationParameters, || {
            FixtureCombinationParameters::new(2.0, raw)
        })?;
        assert!(
            matches!(rejected, Err(FixtureCombinationLifecycleError::InvalidParameter {
            field: FixtureCombinationParameterField::MaxClockError, issue: actual,
        }) if actual == issue)
        );
    }
    let compound = p.call(Call::CombinationParameters, || {
        FixtureCombinationParameters::new(-0.0, f64::NAN)
    })?;
    assert!(matches!(
        compound,
        Err(FixtureCombinationLifecycleError::InvalidParameter {
            field: FixtureCombinationParameterField::CombineLimit,
            issue: FixtureCombinationParameterIssue::NonPositive,
        })
    ));
    Ok(())
}

/// Assert both automatic diagnostic modes use the same fixed redacted projection.
/// # Panics
/// A raw label, scalar, counter or payload in either projection fails equality.
fn assert_fixed_debug(value: &impl std::fmt::Debug, expected: &str) {
    assert_eq!(format!("{value:?}"), expected);
    assert_eq!(format!("{value:#?}"), expected);
}

/// Preserve one stable lifecycle code and the exact borrowed typed cause.
///
/// Public error enums are constructed only to test their diagnostic projection;
/// this does not claim a reached producer failure or manufacture checked state.
/// Across the eleven cases this performs fourteen source probes, three concrete
/// downcasts/pointer checks and thirty-three top-level formatting calls.
/// # Panics
/// A flattened/wrong cause, unexpected leaf cause or changed projection fails.
fn assert_lifecycle_diagnostic(error: &FixtureCombinationLifecycleError, expected: &str) {
    assert_eq!(error.to_string(), expected);
    assert_fixed_debug(error, "FixtureCombinationLifecycleError { .. }");
    let actual = error.source();
    match error {
        FixtureCombinationLifecycleError::Principal { source } => {
            assert!(actual.is_some_and(|cause| {
                cause
                    .downcast_ref::<FixturePrincipalError>()
                    .is_some_and(|inner| std::ptr::eq(inner, source))
            }));
        }
        FixtureCombinationLifecycleError::Combination { source } => {
            assert!(actual.is_some_and(|cause| {
                cause
                    .downcast_ref::<FixtureCombinationError>()
                    .is_some_and(|inner| std::ptr::eq(inner, source))
            }));
        }
        FixtureCombinationLifecycleError::Arithmetic { source, .. } => {
            assert!(actual.is_some_and(|cause| {
                cause
                    .downcast_ref::<FitError>()
                    .is_some_and(|inner| std::ptr::eq(inner, source))
            }));
        }
        _ => assert!(actual.is_none()),
    }
    if let Some(cause) = actual {
        assert!(cause.source().is_none());
    }
}

/// Check every nondelegating lifecycle code, including invariant-only projections.
/// # Panics
/// Exact public codes, redaction and absence of a cause must remain distinct.
fn lifecycle_leaf_diagnostics() {
    for (error, expected) in [
        (
            FixtureCombinationLifecycleError::InvalidParameter {
                field: FixtureCombinationParameterField::MaxClockError,
                issue: FixtureCombinationParameterIssue::Negative,
            },
            "FIXTURE_COMBINATION_PARAMETER_MaxClockError_Negative",
        ),
        (
            FixtureCombinationLifecycleError::InvalidReachabilityOrdinal,
            "FIXTURE_COMBINATION_EVENT_ZERO",
        ),
        (
            FixtureCombinationLifecycleError::UnknownSource,
            "FIXTURE_COMBINATION_UNKNOWN_SOURCE",
        ),
        (
            FixtureCombinationLifecycleError::ReachabilityOrder,
            "FIXTURE_COMBINATION_EVENT_ORDER",
        ),
        (
            FixtureCombinationLifecycleError::CounterExhausted,
            "FIXTURE_COMBINATION_COUNTER_EXHAUSTED",
        ),
        (
            FixtureCombinationLifecycleError::CapacityExceeded,
            "FIXTURE_COMBINATION_CAPACITY",
        ),
        (
            FixtureCombinationLifecycleError::Invariant,
            "FIXTURE_COMBINATION_INVARIANT",
        ),
        (
            FixtureCombinationLifecycleError::ValidationInvariant,
            "FIXTURE_COMBINATION_VALIDATION",
        ),
    ] {
        assert_lifecycle_diagnostic(&error, expected);
    }
}

/// Keep delegated causes useful without formatting their externally supplied data.
/// # Panics
/// The outer code/debug output is fixed even for public error indices at MAX.
fn lifecycle_cause_diagnostics() {
    for (error, expected) in [
        (
            FixtureCombinationLifecycleError::Principal {
                source: FixturePrincipalError::UnknownSource { index: usize::MAX },
            },
            "FIXTURE_COMBINATION_PRINCIPAL",
        ),
        (
            FixtureCombinationLifecycleError::Combination {
                source: FixtureCombinationError::EntryMismatch { index: usize::MAX },
            },
            "FIXTURE_COMBINATION_KERNEL",
        ),
        (
            FixtureCombinationLifecycleError::Arithmetic {
                step: FixtureCombinationLifecycleStep::DistanceLimit,
                source: FitError::Arithmetic {
                    stage: ArithmeticStage::Combination,
                    operation: ArithmeticOperation::Multiply,
                    issue: ArithmeticIssue::NonFinite,
                },
            },
            "FIXTURE_COMBINATION_ARITHMETIC_DistanceLimit",
        ),
    ] {
        assert_lifecycle_diagnostic(&error, expected);
    }
}

/// Read the genuine ready owner without creating another observation or round.
///
/// This adds eight precharged immutable getters and no producers. Together with
/// the existing raw-neighbor loop additions, diagnostics make at most sixty-one
/// top-level format calls, each expected string below sixty-four UTF-8 bytes.
/// Thus returned formatted content is at most 3,904 bytes cumulatively; this is
/// a source-work bound, not an allocator/RSS limit or an observed format counter.
/// # Errors
/// Preserve pre-call getter accounting failure and an absent original source.
/// # Panics
/// Exact redaction, immutable parameter bits and original source counters match.
fn lifecycle_value_diagnostics(
    p: &Profile,
    state: &FixtureCombinationState,
    outcome: &FixtureCombinationOutcome<'_>,
) -> TestResult {
    let parameters = p.get(|| state.parameters())?;
    assert_eq!(
        p.get(|| parameters.combine_limit())?.to_bits(),
        2.0_f64.to_bits()
    );
    assert_eq!(
        p.get(|| parameters.max_clock_error_seconds_per_second())?
            .to_bits(),
        0.031_25_f64.to_bits()
    );
    let label = tag(101)?;
    let source = p
        .get(|| state.source_state(label))?
        .ok_or_else(|| io::Error::other("missing original diagnostic source"))?;
    assert_eq!(p.get(|| source.source_id())?, tag(101)?);
    assert_eq!(p.get(|| source.reachability_updates())?, 0);
    assert_eq!(p.get(|| source.penalty())?, 0);
    assert_eq!(p.get(|| source.last_reachability_ordinal())?, 0);
    assert_fixed_debug(&parameters, "FixtureCombinationParameters { .. }");
    assert_fixed_debug(source, "FixtureCombinationSourceState { .. }");
    assert_fixed_debug(state, "FixtureCombinationState { .. }");
    assert_fixed_debug(outcome, "FixtureCombinationOutcome { .. }");
    lifecycle_leaf_diagnostics();
    lifecycle_cause_diagnostics();
    Ok(())
}

/// Strict dyadic boundaries reject neither equality nor real agreement.
/// # Errors
/// Preserve actual genuine-producer/accounting failures.
/// # Panics
/// Membership, statuses and all independently specified scalar bits must match.
#[test]
fn lifecycle_strict_boundaries_and_original_principal() -> TestResult {
    let p = Profile::new(
        "lifecycle_strict_boundaries_and_original_principal",
        Limits {
            histories: 21,
            appends: 63,
            projections: 21,
            batches: 10,
            seeds: 10,
            advances: 10,
            reachability_updates: 0,
            reachability_records: 0,
            uncertainties: 21,
            equalities: 0,
        },
    );
    distance_boundary_neighbors(&p)?;
    rate_boundary_neighbors(&p)?;
    let a = Source::new(&p, 101, [0; 3])?;
    let b = Source::new(&p, 202, [-(1_i128 << 31), -(1_i128 << 30), 0])?;
    let c = Source::new(&p, 303, [-5 * (1_i128 << 29), -5 * (1_i128 << 28), 0])?;
    let built = Built::new(
        &p,
        &[(&a, 0.125, 1), (&b, 0.25, 1), (&c, 0.25, 1)],
        2_000_000_000,
    )?;
    let batch = built.batch(&p)?;
    let mut state = seed(&p, &built, &batch, 2, [0.125, 0.0])?;
    let metadata = records(&p, &built, &[STANDARD; 3])?;
    let outcome = p.call(Call::Advance, || {
        state.advance(&batch, &built.strata, &metadata)
    })??;
    observe_round(&p, &outcome, ready(1, 2, 3))?;
    assert_eq!(
        status(&p, &outcome, 303)?,
        Some(FixtureCombinationSourceStatus::RateExcluded)
    );
    estimate(
        &p,
        &outcome,
        &built,
        [
            0,
            0x3fb0_0000_0000_0000,
            0x3fc0_0000_0000_0000,
            TWO_RATE_SD,
            TWO_SKEW,
        ],
        &[101, 202],
    )?;
    lifecycle_value_diagnostics(&p, &state, &outcome)?;
    Ok(())
}

/// Startup incompatibility, retained/projection events and supporter-only credit.
/// # Errors
/// Preserve actual genuine-producer/accounting failures.
/// # Panics
/// Only a real new principal-ready event may count down or emit.
#[test]
fn lifecycle_startup_replay_and_supporter_only_pauses() -> TestResult {
    let p = Profile::new(
        "lifecycle_startup_replay_and_supporter_only_pauses",
        Limits {
            histories: 2,
            appends: 9,
            projections: 10,
            batches: 5,
            seeds: 1,
            advances: 7,
            reachability_updates: 0,
            reachability_records: 0,
            uncertainties: 10,
            equalities: 0,
        },
    );
    let mut a = Source::new(&p, 101, [0; 3])?;
    let mut b = Source::new(&p, 202, [0; 3])?;
    let built = Built::new(&p, &[(&a, 0.125, 1), (&b, 33.0 / 64.0, 1)], 2_000_000_000)?;
    let batch = built.batch(&p)?;
    let mut state = seed(&p, &built, &batch, 2, [0.125, 0.0])?;
    let metadata = records(&p, &built, &[STANDARD; 2])?;
    let first = p.call(Call::Advance, || {
        state.advance(&batch, &built.strata, &metadata)
    })??;
    observe_round(&p, &first, ready(1, 1, 2))?;
    let principal = p.get(|| first.principal())?;
    assert_eq!(p.get(|| principal.updated_source())?, None);
    assert_eq!(source_view(&p, &state, 202)?.penalty, 1);
    let replay = p.call(Call::Advance, || {
        state.advance(&batch, &built.strata, &metadata)
    })??;
    observe_round(&p, &replay, retained(2))?;
    assert_eq!(source_view(&p, &state, 202)?.penalty, 1);
    a.append(&p, 3_000_000_000, 0)?;
    let bad = Built::new(&p, &[(&a, 0.125, 1), (&b, 33.0 / 64.0, 1)], 3_000_000_000)?;
    let bad_batch = bad.batch(&p)?;
    let bad_metadata = records(&p, &bad, &[STANDARD; 2])?;
    let bad_round = p.call(Call::Advance, || {
        state.advance(&bad_batch, &bad.strata, &bad_metadata)
    })??;
    observe_round(&p, &bad_round, ready(2, 1, 2))?;
    assert_eq!(source_view(&p, &state, 202)?.penalty, 1);
    let projected = Built::new(&p, &[(&a, 0.125, 1), (&b, 33.0 / 64.0, 1)], 4_000_000_000)?;
    let projected_batch = projected.batch(&p)?;
    let projected_metadata = records(&p, &projected, &[STANDARD; 2])?;
    let pause = p.call(Call::Advance, || {
        state.advance(&projected_batch, &projected.strata, &projected_metadata)
    })??;
    observe_round(&p, &pause, retained(2))?;
    b.append(&p, 4_000_000_000, 0)?;
    let supporter = Built::new(&p, &[(&a, 0.125, 1), (&b, 0.25, 1)], 4_000_000_000)?;
    let supporter_batch = supporter.batch(&p)?;
    let supporter_metadata = records(&p, &supporter, &[STANDARD; 2])?;
    let supporter_pause = p.call(Call::Advance, || {
        state.advance(&supporter_batch, &supporter.strata, &supporter_metadata)
    })??;
    observe_round(&p, &supporter_pause, retained(2))?;
    assert_eq!(source_view(&p, &state, 202)?.penalty, 1);
    assert!(pending(&p, &state, 202)?);
    assert!(!pending(&p, &state, 101)?);
    a.append(&p, 5_000_000_000, 0)?;
    let good = Built::new(&p, &[(&a, 0.125, 1), (&b, 0.25, 1)], 5_000_000_000)?;
    let good_batch = good.batch(&p)?;
    let good_metadata = records(&p, &good, &[STANDARD; 2])?;
    let recovered = p.call(Call::Advance, || {
        state.advance(&good_batch, &good.strata, &good_metadata)
    })??;
    observe_round(&p, &recovered, ready(3, 2, 2))?;
    assert_eq!(source_view(&p, &state, 202)?.penalty, 0);
    assert!(!pending(&p, &state, 101)?);
    assert!(!pending(&p, &state, 202)?);
    let replay = p.call(Call::Advance, || {
        state.advance(&good_batch, &good.strata, &good_metadata)
    })??;
    observe_round(&p, &replay, retained(2))?;
    assert_eq!(p.get(|| state.emitted_rounds())?, 3);
    assert_eq!(source_view(&p, &state, 202)?.penalty, 0);
    Ok(())
}

/// Reachability maturity, 32-event countdown, relapse and accepted event custody.
/// # Errors
/// Preserve actual genuine-producer/accounting failures.
/// # Panics
/// Exact penalties/ordinals and whole-state healthy-owner equality must hold.
#[test]
fn lifecycle_maturity_countdown_relapse_and_event_order() -> TestResult {
    let p = Profile::new(
        "lifecycle_maturity_countdown_relapse_and_event_order",
        Limits {
            histories: 2,
            appends: 53,
            projections: 98,
            batches: 49,
            seeds: 3,
            advances: 98,
            reachability_updates: 23,
            reachability_records: 22,
            uncertainties: 98,
            equalities: 64,
        },
    );
    let mut a = Source::new(&p, 101, [0; 3])?;
    let mut b = Source::new(&p, 202, [0; 3])?;
    let built = Built::new(&p, &[(&a, 0.125, 1), (&b, 33.0 / 64.0, 1)], 2_000_000_000)?;
    let batch = built.batch(&p)?;
    let mut owners = Owners::new(&p, &built, &batch)?;
    maturity_and_event_order(&p, &mut owners, &mut a, &b, &built, &batch)?;
    mature_penalty_relapse(&p, &mut owners, &mut a, &mut b)?;
    for index in 0..32_u64 {
        let at = (18 + index) * 1_000_000_000;
        a.append(&p, at, 0)?;
        pair_round(
            &p,
            &mut owners,
            [&a, &b],
            PairFact {
                at,
                distance: 0.25,
                penalty: u8::try_from(31 - index)?,
                round: Some(14 + index),
            },
        )?;
    }
    assert_eq!(p.get(|| owners.state.emitted_rounds())?, 45);
    assert_eq!(
        source_view(&p, &owners.state, 202)?,
        SourceView {
            maturity: 8,
            penalty: 0,
            ordinal: 9
        }
    );
    // Explicit reseeding constructs a distinct complete fixture instance.
    let reseeded = seed(&p, &built, &batch, 2, [0.125, 0.0])?;
    assert_eq!(p.get(|| reseeded.emitted_rounds())?, 0);
    let principal = p.get(|| reseeded.principal_state())?;
    assert_eq!(p.get(|| principal.principal())?, None);
    assert_eq!(
        source_view(&p, &reseeded, 202)?,
        SourceView {
            maturity: 0,
            penalty: 0,
            ordinal: 0
        }
    );
    assert!(pending(&p, &reseeded, 101)?);
    assert!(pending(&p, &reseeded, 202)?);
    p.call(Call::Equality, || assert_ne!(reseeded, owners.state))?;
    Ok(())
}

/// Full metadata admission, short-circuit arithmetic and same-event atomic retry.
/// # Errors
/// Preserve actual genuine-producer/accounting failures.
/// # Panics
/// All rejected calls preserve complete state; the corrected event commits once.
#[test]
fn lifecycle_rejection_rolls_back_and_same_event_recovers() -> TestResult {
    let p = Profile::new(
        "lifecycle_rejection_rolls_back_and_same_event_recovers",
        Limits {
            histories: 2,
            appends: 7,
            projections: 8,
            batches: 4,
            seeds: 2,
            advances: 8,
            reachability_updates: 4,
            reachability_records: 4,
            uncertainties: 8,
            equalities: 16,
        },
    );
    let mut a = Source::new(&p, 101, [0; 3])?;
    let b = Source::new(&p, 202, [0; 3])?;
    let built = Built::new(&p, &[(&a, 0.125, 1), (&b, 33.0 / 64.0, 1)], 2_000_000_000)?;
    let batch = built.batch(&p)?;
    let mut owners = Owners::new(&p, &built, &batch)?;
    let raw = [STANDARD, [0.0, 0.0, f64::MAX]];
    let metadata = records(&p, &built, &raw)?;
    let initial = owners.advance(&p, &batch, &built.strata, &metadata)?;
    observe_round(&p, &initial, ready(1, 1, 2))?;
    owners.reach(&p, 1, false)?;
    a.append(&p, 3_000_000_000, 0)?;
    let fresh_bad = Built::new(&p, &[(&a, 0.125, 1), (&b, 33.0 / 64.0, 1)], 3_000_000_000)?;
    let fresh_bad_batch = fresh_bad.batch(&p)?;
    let fresh_bad_metadata = records(&p, &fresh_bad, &raw)?;
    let missing = p.call(Call::Advance, || {
        owners.state.advance(
            &fresh_bad_batch,
            &fresh_bad.strata,
            &fresh_bad_metadata[..1],
        )
    })?;
    assert!(matches!(
        missing,
        Err(FixtureCombinationLifecycleError::Combination {
            source: FixtureCombinationError::MetadataCountMismatch
        })
    ));
    owners.same(&p)?;
    owners.reach(&p, 2, true)?;
    let overflow = Built::new(&p, &[(&a, 0.125, 1), (&b, 0.25, 1)], 3_000_000_000)?;
    let overflow_batch = overflow.batch(&p)?;
    let overflow_metadata = records(&p, &overflow, &raw)?;
    let rejected = p.call(Call::Advance, || {
        owners
            .state
            .advance(&overflow_batch, &overflow.strata, &overflow_metadata)
    })?;
    assert!(matches!(
        rejected,
        Err(FixtureCombinationLifecycleError::Arithmetic {
            step: FixtureCombinationLifecycleStep::RateLimit,
            source: FitError::Arithmetic {
                stage: ArithmeticStage::Combination,
                operation: ArithmeticOperation::Multiply,
                issue: ArithmeticIssue::NonFinite
            }
        })
    ));
    owners.same(&p)?;
    assert_eq!(
        source_view(&p, &owners.state, 202)?,
        SourceView {
            maturity: 2,
            penalty: 1,
            ordinal: 2
        }
    );
    let good = Built::new(&p, &[(&a, 0.125, 1), (&b, 0.25, 1)], 3_000_000_000)?;
    let good_batch = good.batch(&p)?;
    let good_metadata = records(&p, &good, &[STANDARD; 2])?;
    let recovered = owners.advance(&p, &good_batch, &good.strata, &good_metadata)?;
    observe_round(&p, &recovered, ready(2, 2, 2))?;
    assert_eq!(
        source_view(&p, &owners.state, 202)?,
        SourceView {
            maturity: 2,
            penalty: 0,
            ordinal: 2
        }
    );
    let replay = owners.advance(&p, &good_batch, &good.strata, &good_metadata)?;
    observe_round(&p, &replay, retained(2))?;
    assert_eq!(p.get(|| owners.state.emitted_rounds())?, 2);
    Ok(())
}

/// A filtered mathematical singleton retains the original two-source agreement.
/// # Errors
/// Preserve actual genuine-producer/accounting failures.
/// # Panics
/// Computed zero bits/denominator guards differ from genuine singleton copying.
#[test]
fn lifecycle_filtered_one_keeps_multiwitness_arithmetic() -> TestResult {
    let p = Profile::new(
        "lifecycle_filtered_one_keeps_multiwitness_arithmetic",
        Limits {
            histories: 2,
            appends: 6,
            projections: 5,
            batches: 3,
            seeds: 3,
            advances: 6,
            reachability_updates: 0,
            reachability_records: 0,
            uncertainties: 9,
            equalities: 8,
        },
    );
    let a = Source::new(&p, 101, [0; 3])?;
    let b = Source::new(&p, 202, [0; 3])?;
    let bad = Built::new(&p, &[(&a, 0.0, 1), (&b, 33.0 / 64.0, 1)], 2_000_000_000)?;
    let bad_batch = bad.batch(&p)?;
    let mut owners = Owners::new(&p, &bad, &bad_batch)?;
    let zero_distance = records(&p, &bad, &[[-0.0, 1.0, 0.015_625], [-0.0; 3]])?;
    let rejected = p.call(Call::Advance, || {
        owners
            .state
            .advance(&bad_batch, &bad.strata, &zero_distance)
    })?;
    assert!(matches!(
        rejected,
        Err(FixtureCombinationLifecycleError::Combination {
            source: FixtureCombinationError::ZeroDenominator {
                index: 0,
                field: FixtureCombinationDenominatorField::RootDistance
            }
        })
    ));
    owners.same(&p)?;
    let good = Built::new(&p, &[(&a, 0.125, 1), (&b, 33.0 / 64.0, 1)], 2_000_000_000)?;
    let good_batch = good.batch(&p)?;
    for (raw, field) in [
        (
            [-0.0, 0.0, 0.015_625],
            FixtureCombinationDenominatorField::RateStandardDeviation,
        ),
        (
            [-0.0, 1.0, 0.0],
            FixtureCombinationDenominatorField::TrackingSkew,
        ),
    ] {
        let metadata = records(&p, &good, &[raw, [-0.0; 3]])?;
        let rejected = p.call(Call::Advance, || {
            owners.state.advance(&good_batch, &good.strata, &metadata)
        })?;
        assert!(
            matches!(rejected, Err(FixtureCombinationLifecycleError::Combination {
            source: FixtureCombinationError::ZeroDenominator { index: 0, field: actual }
        }) if actual == field)
        );
        owners.same(&p)?;
    }
    let good_metadata = records(&p, &good, &[[-0.0, 1.0, 0.015_625], [-0.0; 3]])?;
    let computed = owners.advance(&p, &good_batch, &good.strata, &good_metadata)?;
    observe_round(&p, &computed, ready(1, 1, 2))?;
    let principal = p.get(|| computed.principal())?;
    let selection = p.get(|| principal.selection())?;
    assert_eq!(p.get(|| selection.minimum())?, 2);
    estimate(
        &p,
        &computed,
        &good,
        [0, 0, 0, 0x3ff0_0000_0000_0000, 0x3f90_0000_0000_0000],
        &[101],
    )?;
    let single = Built::new(&p, &[(&a, 0.0, 1)], 2_000_000_000)?;
    let single_batch = single.batch(&p)?;
    let mut singleton = seed(&p, &single, &single_batch, 1, [0.125, 0.0])?;
    let metadata = records(&p, &single, &[[-0.0; 3]])?;
    let copied = p.call(Call::Advance, || {
        singleton.advance(&single_batch, &single.strata, &metadata)
    })??;
    observe_round(&p, &copied, ready(1, 1, 1))?;
    estimate(
        &p,
        &copied,
        &single,
        [
            0,
            0,
            0x8000_0000_0000_0000,
            0x8000_0000_0000_0000,
            0x8000_0000_0000_0000,
        ],
        &[101],
    )?;
    Ok(())
}

/// Outside-witness scoring reset, actual no-majority, waiting and reacquisition.
/// # Errors
/// Preserve actual genuine-producer/accounting failures.
/// # Panics
/// Ordered resets, honest absent states and original minimum must match.
#[test]
fn lifecycle_outside_witness_loss_waiting_and_reacquisition() -> TestResult {
    let p = Profile::new(
        "lifecycle_outside_witness_loss_waiting_and_reacquisition",
        Limits {
            histories: 3,
            appends: 14,
            projections: 18,
            batches: 6,
            seeds: 1,
            advances: 8,
            reachability_updates: 0,
            reachability_records: 0,
            uncertainties: 10,
            equalities: 0,
        },
    );
    let mut a = Source::new(&p, 101, [0; 3])?;
    let mut b = Source::new(&p, 202, [0; 3])?;
    let mut c = Source::new(&p, 303, [1_i128 << 32; 3])?;
    let raw = [[0.125, 1.0, 1.0]; 3];
    let built = Built::new(
        &p,
        &[(&a, 0.03125, 1), (&b, 0.0625, 1), (&c, 0.03125, 1)],
        2_000_000_000,
    )?;
    let batch = built.batch(&p)?;
    let mut state = seed(&p, &built, &batch, 2, [0.125, 0.0])?;
    let metadata = records(&p, &built, &raw[..2])?;
    let first = p.call(Call::Advance, || {
        state.advance(&batch, &built.strata, &metadata)
    })??;
    observe_round(&p, &first, ready(1, 2, 2))?;
    assert_eq!(source_view(&p, &state, 303)?.penalty, 0);
    let pause = p.call(Call::Advance, || {
        state.advance(&batch, &built.strata, &metadata)
    })??;
    observe_round(&p, &pause, retained(2))?;
    assert_eq!(source_view(&p, &state, 303)?.penalty, 32);
    assert_eq!(
        status(&p, &pause, 303)?,
        Some(FixtureCombinationSourceStatus::OutsideWitness)
    );
    disagreement_then_waiting(&p, &mut state, &a, &mut b, &c)?;
    a.append(&p, 4_000_000_000, 0)?;
    let reacquired = Built::new(
        &p,
        &[(&a, 0.03125, 1), (&b, 0.0625, 1), (&c, 0.03125, 1)],
        4_000_000_000,
    )?;
    let reacquired_batch = reacquired.batch(&p)?;
    let reacquired_metadata = records(&p, &reacquired, &raw[..2])?;
    let ready_again = p.call(Call::Advance, || {
        state.advance(&reacquired_batch, &reacquired.strata, &reacquired_metadata)
    })??;
    observe_round(&p, &ready_again, ready(2, 2, 2))?;
    assert_eq!(source_view(&p, &state, 303)?.penalty, 0);
    let pause = p.call(Call::Advance, || {
        state.advance(&reacquired_batch, &reacquired.strata, &reacquired_metadata)
    })??;
    observe_round(&p, &pause, retained(2))?;
    assert_eq!(source_view(&p, &state, 303)?.penalty, 32);
    return_penalized_source(&p, &mut state, &mut a, &b, &mut c, &raw)?;
    Ok(())
}

/// A committed direct principal replacement clears all penalties before filtering.
///
/// At the final real event the old principal distance is four and each candidate
/// denominator is 1/8+1/8=1/4. Both candidate scores become exactly sixteen,
/// above the fixed threshold ten. Roster order chooses B; its previously pending
/// real observation permits the direct Some(A)->Some(B) transition. C's mature
/// penalty32 must clear to zero rather than merely decrementing to31, while A is
/// freshly distance-excluded against B's original threshold1/2 that same round.
/// # Errors
/// Preserve actual producer, constructor and lifecycle admission failures.
/// # Panics
/// Independent principal/contributor identities, scores' consequences, penalties
/// and estimate bits must match the authored trajectory.
#[test]
fn lifecycle_direct_replacement_clears_all_then_rechecks_supporters() -> TestResult {
    let p = Profile::new(
        "lifecycle_direct_replacement_clears_all_then_rechecks_supporters",
        Limits {
            histories: 3,
            appends: 11,
            projections: 9,
            batches: 3,
            seeds: 1,
            advances: 3,
            reachability_updates: 16,
            reachability_records: 16,
            uncertainties: 9,
            equalities: 0,
        },
    );
    let mut a = Source::new(&p, 101, [0; 3])?;
    let mut b = Source::new(&p, 202, [0; 3])?;
    let c = Source::new(&p, 303, [0; 3])?;
    let initial = Built::new(
        &p,
        &[(&a, 0.125, 1), (&b, 33.0 / 64.0, 1), (&c, 33.0 / 64.0, 1)],
        2_000_000_000,
    )?;
    let initial_batch = initial.batch(&p)?;
    let mut state = seed(&p, &initial, &initial_batch, 2, [0.125, 0.0])?;
    prepare_penalized_replacement(&p, &mut state, &a, &mut b, &c, &initial, &initial_batch)?;
    a.append(&p, 4_000_000_000, 0)?;
    let switched = Built::new(
        &p,
        &[(&a, 4.0, 1), (&b, 0.125, 1), (&c, 0.125, 1)],
        4_000_000_000,
    )?;
    let switched_batch = switched.batch(&p)?;
    let switched_metadata = records(&p, &switched, &[STANDARD; 3])?;
    let replacement = p.call(Call::Advance, || {
        state.advance(&switched_batch, &switched.strata, &switched_metadata)
    })??;
    observe_round(&p, &replacement, ready_for(202, 2, 2, 3))?;
    assert_eq!(
        status(&p, &replacement, 101)?,
        Some(FixtureCombinationSourceStatus::DistanceExcluded)
    );
    assert_eq!(
        status(&p, &replacement, 202)?,
        Some(FixtureCombinationSourceStatus::Principal)
    );
    assert_eq!(
        status(&p, &replacement, 303)?,
        Some(FixtureCombinationSourceStatus::Contributor)
    );
    assert_eq!(source_view(&p, &state, 101)?.penalty, 1);
    assert_eq!(source_view(&p, &state, 202)?.penalty, 0);
    assert_eq!(source_view(&p, &state, 303)?.penalty, 0);
    for label in [101, 202, 303] {
        assert!(!pending(&p, &state, label)?);
    }
    let principal = p.get(|| replacement.principal())?;
    let selection = p.get(|| principal.selection())?;
    assert_eq!(p.get(|| selection.minimum())?, 2);
    let combined = p
        .get(|| replacement.combined())?
        .ok_or_else(|| io::Error::other("missing replacement estimate"))?;
    assert_eq!(p.get(|| combined.principal_label())?, tag(202)?);
    estimate(
        &p,
        &replacement,
        &switched,
        [0, 0, 0x3fc0_0000_0000_0000, TWO_RATE_SD, TWO_SKEW],
        &[202, 303],
    )?;
    assert_eq!(p.get(|| state.emitted_rounds())?, 2);
    Ok(())
}

/// Compare genuine admitted values whose independently varied fields matter.
///
/// Each attempt keeps the existing conservative complete-state comparison upper;
/// direct parameter equality actually inspects only its two immutable bit fields.
/// # Errors
/// Reject incomplete fixture capture and preserve pre-call accounting failure.
/// # Panics
/// Signed-zero or one-field differences must not disappear; identity stays equal.
fn compare_parameter_neighbors(
    p: &Profile,
    values: [Option<FixtureCombinationParameters>; 6],
) -> TestResult {
    let [
        Some(ordinary),
        Some(negative_zero),
        Some(tiny),
        Some(huge),
        Some(error),
        Some(other),
    ] = values
    else {
        return Err(io::Error::other("missing admitted parameter neighbor").into());
    };
    for (left, right, expected) in [
        (ordinary, ordinary, true),
        (ordinary, negative_zero, false),
        (ordinary, other, false),
        (ordinary, error, false),
        (ordinary, tiny, false),
        (ordinary, huge, false),
    ] {
        assert_eq!(p.call(Call::Equality, || left == right)?, expected);
    }
    Ok(())
}

/// Raw parameter/ordinal constructors enforce supported scalars and preserve bits.
///
/// Constructor-admitted ordinalMAX is only a shape neighbor: this case records
/// no event and claims no contextual exact-next or public overflow execution.
/// # Errors
/// Preserve harness/accounting failure and unexpected healthy-constructor errors.
/// # Panics
/// Invalid scalars/ordinalzero must reject with their independent field/cause;
/// supported neighboring values, signed zero and ordinal payloads retain bits.
#[test]
fn lifecycle_raw_parameter_and_ordinal_admission_neighbors() -> TestResult {
    let p = Profile::raw(
        "lifecycle_raw_parameter_and_ordinal_admission_neighbors",
        RawLimits {
            parameters: 21,
            ordinals: 6,
            equalities: 6,
        },
    );
    reject_raw_parameter_neighbors(&p)?;
    let mut values = [None; 6];
    for (slot, (combine, error)) in values.iter_mut().zip([
        (2.0, 0.0),
        (2.0, -0.0),
        (f64::MIN_POSITIVE, f64::MIN_POSITIVE),
        (f64::MAX, f64::MAX),
        (2.0, 0.03125),
        (4.0, 0.0),
    ]) {
        let admitted = p.call(Call::CombinationParameters, || {
            FixtureCombinationParameters::new(combine, error)
        })??;
        assert_eq!(
            p.get(|| admitted.combine_limit())?.to_bits(),
            combine.to_bits()
        );
        assert_eq!(
            p.get(|| admitted.max_clock_error_seconds_per_second())?
                .to_bits(),
            error.to_bits()
        );
        assert_fixed_debug(&admitted, "FixtureCombinationParameters { .. }");
        *slot = Some(admitted);
    }
    compare_parameter_neighbors(&p, values)?;
    let label = tag(202)?;
    for reachable in [false, true] {
        let rejected = p.call(Call::ReachabilityUpdate, || {
            FixtureReachabilityUpdate::new(label, 0, reachable)
        })?;
        assert!(matches!(
            rejected,
            Err(FixtureCombinationLifecycleError::InvalidReachabilityOrdinal)
        ));
        for ordinal in [1, u64::MAX] {
            let admitted = p.call(Call::ReachabilityUpdate, || {
                FixtureReachabilityUpdate::new(label, ordinal, reachable)
            })??;
            assert_eq!(p.get(|| admitted.source_id())?, label);
            assert_eq!(p.get(|| admitted.ordinal())?, ordinal);
            assert_eq!(p.get(|| admitted.reachable())?, reachable);
            assert_fixed_debug(&admitted, "FixtureReachabilityUpdate { .. }");
        }
    }
    Ok(())
}

/// Exact reached checked-primitive failure expected from genuine admitted input.
#[derive(Clone, Copy)]
struct OverflowFact {
    /// Public compatibility stage owning the unsupported computed value.
    step: FixtureCombinationLifecycleStep,
    /// Independently expected primitive, without rejected operand retention.
    operation: ArithmeticOperation,
}

/// Commit the healthy seed round and establish spent original observation credit.
/// # Errors
/// Preserve constructor, real advance and pre-call accounting failures.
/// # Panics
/// Both independent owners must produce round one and consume A/B seed credit.
fn committed_owners(
    p: &Profile,
    built: &Built,
    batch: &FixtureProjectedBatch<'_>,
    combination: [f64; 2],
) -> TestResult<Owners> {
    let mut owners = Owners::with_combination(p, built, batch, combination)?;
    let metadata = records(p, built, &[STANDARD; 2])?;
    let initial = owners.advance(p, batch, &built.strata, &metadata)?;
    observe_round(p, &initial, ready(1, 2, 2))?;
    assert!(!pending(p, &owners.state, 101)?);
    assert!(!pending(p, &owners.state, 202)?);
    Ok(owners)
}

/// Reject one genuine fresh-principal event and compare complete entry state.
/// # Errors
/// Preserve pre-call accounting failure; actual product errors remain observable.
/// # Panics
/// The exact compatibility step/primitive/cause must reject, and every owner bit
/// must equal the independent healthy owner that never received the failed call.
fn reject_overflow_event(
    p: &Profile,
    owners: &mut Owners,
    batch: &FixtureProjectedBatch<'_>,
    strata: &[FixturePrincipalStratum],
    metadata: &[FixtureSupporterUncertainty<'_>],
    fact: OverflowFact,
) -> TestResult {
    let rejected = p.call(Call::Advance, || {
        owners.state.advance(batch, strata, metadata)
    })?;
    assert!(matches!(rejected,
        Err(FixtureCombinationLifecycleError::Arithmetic {
            step,
            source: FitError::Arithmetic {
                stage: ArithmeticStage::Combination,
                operation,
                issue: ArithmeticIssue::NonFinite,
            },
        }) if step == fact.step && operation == fact.operation));
    owners.same(p)
}

/// Correct and retry the same event on unchanged owners, then replay it once.
/// # Errors
/// Preserve original-entry construction, real advance and accounting failures.
/// # Panics
/// Only the corrected genuine A event may emit round two; replay creates no
/// estimate/ordinal, all credit is spent, and scalar bits/labels remain analytic.
fn corrected_event_and_replay(
    p: &Profile,
    owners: &mut Owners,
    built: &Built,
    batch: &FixtureProjectedBatch<'_>,
) -> TestResult {
    let metadata = records(p, built, &[STANDARD; 2])?;
    let recovered = owners.advance(p, batch, &built.strata, &metadata)?;
    observe_round(p, &recovered, ready(2, 2, 2))?;
    let principal = p.get(|| recovered.principal())?;
    assert_eq!(p.get(|| principal.updated_source())?, Some(tag(101)?));
    estimate(
        p,
        &recovered,
        built,
        [0, 0, 0x3fc0_0000_0000_0000, TWO_RATE_SD, TWO_SKEW],
        &[101, 202],
    )?;
    assert_eq!(source_view(p, &owners.state, 101)?.penalty, 0);
    assert_eq!(source_view(p, &owners.state, 202)?.penalty, 0);
    assert!(!pending(p, &owners.state, 101)?);
    assert!(!pending(p, &owners.state, 202)?);
    let replay = owners.advance(p, batch, &built.strata, &metadata)?;
    observe_round(p, &replay, retained(2))?;
    assert_eq!(p.get(|| owners.state.emitted_rounds())?, 2);
    Ok(())
}

/// Fixed MAX multiplier rejects MAX*9/8, then the same event admits MAX/4.
///
/// Two original three-row histories supply seed credit; only A receives the one
/// genuine new observation. Source-complete calls include seven appends, six
/// projections, seven advances and eighty immutable getters; these are authored
/// recipe counts rather than runtime observations.
/// # Errors
/// Preserve genuine producer/admission and pre-call accounting failures.
/// # Panics
/// DistanceLimit/Multiply/NonFinite must roll back before the corrected same
/// event commits once, preserving the real witness and fixed owner parameters.
#[test]
fn lifecycle_distance_limit_overflow_rolls_back_and_recovers() -> TestResult {
    let p = Profile::new(
        "lifecycle_distance_limit_overflow_rolls_back_and_recovers",
        Limits {
            histories: 2,
            appends: 7,
            projections: 6,
            batches: 3,
            seeds: 2,
            advances: 7,
            reachability_updates: 0,
            reachability_records: 0,
            uncertainties: 6,
            equalities: 4,
        },
    );
    let mut a = Source::new(&p, 101, [0; 3])?;
    let b = Source::new(&p, 202, [0; 3])?;
    let initial = Built::new(&p, &[(&a, 0.125, 1), (&b, 0.125, 1)], 2_000_000_000)?;
    let initial_batch = initial.batch(&p)?;
    let mut owners = committed_owners(&p, &initial, &initial_batch, [f64::MAX, 0.03125])?;
    a.append(&p, 3_000_000_000, 0)?;
    let bad = Built::new(&p, &[(&a, 1.0, 1), (&b, 1.0, 1)], 3_000_000_000)?;
    let bad_batch = bad.batch(&p)?;
    let bad_metadata = records(&p, &bad, &[STANDARD; 2])?;
    reject_overflow_event(
        &p,
        &mut owners,
        &bad_batch,
        &bad.strata,
        &bad_metadata,
        OverflowFact {
            step: FixtureCombinationLifecycleStep::DistanceLimit,
            operation: ArithmeticOperation::Multiply,
        },
    )?;
    let good = Built::new(&p, &[(&a, 0.125, 1), (&b, 0.125, 1)], 3_000_000_000)?;
    let good_batch = good.batch(&p)?;
    corrected_event_and_replay(&p, &mut owners, &good, &good_batch)
}

/// Two individually admitted MAX skews overflow their checked sum.
///
/// The same original fresh batch recovers with STANDARD metadata on unchanged
/// owners. The complete authored recipe has seven appends, four projections,
/// seven advances and seventy-eight immutable getters.
/// # Errors
/// Preserve genuine producer/admission and pre-call accounting failures.
/// # Panics
/// SkewSum/Add/NonFinite must preserve complete state and permit one corrected
/// emission with an inert replay, without replacing owner parameters or credit.
#[test]
fn lifecycle_skew_sum_overflow_rolls_back_and_recovers() -> TestResult {
    let p = Profile::new(
        "lifecycle_skew_sum_overflow_rolls_back_and_recovers",
        Limits {
            histories: 2,
            appends: 7,
            projections: 4,
            batches: 2,
            seeds: 2,
            advances: 7,
            reachability_updates: 0,
            reachability_records: 0,
            uncertainties: 6,
            equalities: 4,
        },
    );
    let mut a = Source::new(&p, 101, [0; 3])?;
    let b = Source::new(&p, 202, [0; 3])?;
    let initial = Built::new(&p, &[(&a, 0.125, 1), (&b, 0.125, 1)], 2_000_000_000)?;
    let initial_batch = initial.batch(&p)?;
    let mut owners = committed_owners(&p, &initial, &initial_batch, [2.0, 0.03125])?;
    a.append(&p, 3_000_000_000, 0)?;
    let fresh = Built::new(&p, &[(&a, 0.125, 1), (&b, 0.125, 1)], 3_000_000_000)?;
    let fresh_batch = fresh.batch(&p)?;
    let bad_metadata = records(&p, &fresh, &[[0.125, 1.0, f64::MAX]; 2])?;
    reject_overflow_event(
        &p,
        &mut owners,
        &fresh_batch,
        &fresh.strata,
        &bad_metadata,
        OverflowFact {
            step: FixtureCombinationLifecycleStep::SkewSum,
            operation: ArithmeticOperation::Add,
        },
    )?;
    corrected_event_and_replay(&p, &mut owners, &fresh, &fresh_batch)
}

/// An admitted MAX skew sum plus fixed MAX clock error overflows separately.
///
/// Each bad skew is exactly MAX/2; their sum is supported MAX. Only the next
/// checked addition rejects. STANDARD metadata restores a finite MAX/2 rate
/// limit under the same multiplier1/2 and max-clock-errorMAX owner parameters.
/// Seven appends, four projections, seven advances and seventy-eight getters
/// form the complete authored source recipe, not observed internal work.
/// # Errors
/// Preserve genuine producer/admission and pre-call accounting failures.
/// # Panics
/// MaxClockErrorSum/Add/NonFinite must preserve all state, then the same event
/// commits once while its replay produces no estimate, count change or credit.
#[test]
fn lifecycle_max_clock_error_sum_overflow_rolls_back_and_recovers() -> TestResult {
    let p = Profile::new(
        "lifecycle_max_clock_error_sum_overflow_rolls_back_and_recovers",
        Limits {
            histories: 2,
            appends: 7,
            projections: 4,
            batches: 2,
            seeds: 2,
            advances: 7,
            reachability_updates: 0,
            reachability_records: 0,
            uncertainties: 6,
            equalities: 4,
        },
    );
    let mut a = Source::new(&p, 101, [0; 3])?;
    let b = Source::new(&p, 202, [0; 3])?;
    let initial = Built::new(&p, &[(&a, 0.125, 1), (&b, 0.125, 1)], 2_000_000_000)?;
    let initial_batch = initial.batch(&p)?;
    let mut owners = committed_owners(&p, &initial, &initial_batch, [0.5, f64::MAX])?;
    a.append(&p, 3_000_000_000, 0)?;
    let fresh = Built::new(&p, &[(&a, 0.125, 1), (&b, 0.125, 1)], 3_000_000_000)?;
    let fresh_batch = fresh.batch(&p)?;
    let half_max = f64::from_bits(0x7fdf_ffff_ffff_ffff);
    let bad_metadata = records(&p, &fresh, &[[0.125, 1.0, half_max]; 2])?;
    reject_overflow_event(
        &p,
        &mut owners,
        &fresh_batch,
        &fresh.strata,
        &bad_metadata,
        OverflowFact {
            step: FixtureCombinationLifecycleStep::MaxClockErrorSum,
            operation: ArithmeticOperation::Add,
        },
    )?;
    corrected_event_and_replay(&p, &mut owners, &fresh, &fresh_batch)
}

/// Reject each one-over seed input before deeper roster/metadata admission.
///
/// Parameters/minimum are genuine immutable values constructed once and reused.
/// Four seed attempts include two capacity rejections and two healthy owners.
/// # Errors
/// Preserve real construction/accounting failures and unexpected legal rejection.
/// # Panics
/// Each raw seventeen-item input must report lifecycle capacity precedence.
fn seed_capacity_owners(
    p: &Profile,
    built: &Built,
    batch: &FixtureProjectedBatch<'_>,
    oversized_roster: &[NonZeroU64],
    oversized_strata: &[FixturePrincipalStratum],
) -> TestResult<Owners> {
    let principal = p.call(Call::PrincipalParameters, || {
        FixturePrincipalParameters::new(0.125, 0.0)
    })??;
    let combination = p.call(Call::CombinationParameters, || {
        FixtureCombinationParameters::new(2.0, 0.03125)
    })??;
    let minimum = p.call(Call::Minimum, || FixtureMinimum::new(16))??;
    for (roster, strata) in [
        (oversized_roster, built.strata.as_slice()),
        (built.roster.as_slice(), oversized_strata),
    ] {
        let rejected = p.call(Call::Seed, || {
            FixtureCombinationState::seed(roster, principal, combination, minimum, batch, strata)
        })?;
        assert!(matches!(
            rejected,
            Err(FixtureCombinationLifecycleError::CapacityExceeded)
        ));
    }
    let state = p.call(Call::Seed, || {
        FixtureCombinationState::seed(
            &built.roster,
            principal,
            combination,
            minimum,
            batch,
            &built.strata,
        )
    })??;
    let control = p.call(Call::Seed, || {
        FixtureCombinationState::seed(
            &built.roster,
            principal,
            combination,
            minimum,
            batch,
            &built.strata,
        )
    })??;
    Ok(Owners { state, control })
}

/// Reject each one-over advance while the original observation credit is pending.
/// # Errors
/// Preserve pre-call accounting failures and full-state comparison failure.
/// # Panics
/// Capacity wins over duplicate/unknown metadata and both failures preserve credit.
fn reject_capacity_advances(
    p: &Profile,
    owners: &mut Owners,
    built: &Built,
    batch: &FixtureProjectedBatch<'_>,
    metadata: &[FixtureSupporterUncertainty<'_>],
    oversized_strata: &[FixturePrincipalStratum],
    oversized_metadata: &[FixtureSupporterUncertainty<'_>],
) -> TestResult {
    for (strata, uncertainties) in [
        (oversized_strata, metadata),
        (built.strata.as_slice(), oversized_metadata),
    ] {
        let rejected = p.call(Call::Advance, || {
            owners.state.advance(batch, strata, uncertainties)
        })?;
        assert!(matches!(
            rejected,
            Err(FixtureCombinationLifecycleError::CapacityExceeded)
        ));
        owners.same(p)?;
    }
    Ok(())
}

/// Check the independent sixteen-way agreement and contributor/retained meanings.
/// # Errors
/// Preserve pre-call accounting and actual estimate-observer failures.
/// # Panics
/// Minimum/population/depth remain sixteen, principal101 is stable and no padding
/// source appears. Sixteen equal phases/uncertainties give exact dyadic results.
fn observe_capacity_round(
    p: &Profile,
    outcome: &FixtureCombinationOutcome<'_>,
    built: &Built,
    expected: ExpectedRound,
) -> TestResult {
    observe_round(p, outcome, expected)?;
    let decision = p.get(|| outcome.principal())?;
    let selection = p.get(|| decision.selection())?;
    assert_eq!(p.get(|| selection.population())?, 16);
    assert_eq!(p.get(|| selection.minimum())?, 16);
    assert_eq!(p.get(|| selection.maximum_depth())?, 16);
    assert!(p.get(|| selection.strict_majority())?);
    assert!(p.get(|| selection.minimum_met())?);
    for label in 101_u64..117 {
        let expected_status = if expected.ordinal.is_none() {
            FixtureCombinationSourceStatus::NotEvaluated
        } else if label == 101 {
            FixtureCombinationSourceStatus::Principal
        } else {
            FixtureCombinationSourceStatus::Contributor
        };
        assert_eq!(status(p, outcome, label)?, Some(expected_status));
    }
    assert_eq!(status(p, outcome, 117)?, None);
    if expected.ordinal.is_some() {
        let labels = (101_u64..117).collect::<Vec<_>>();
        estimate(
            p,
            outcome,
            built,
            [
                0,
                0,
                0x3fc0_0000_0000_0000,
                0x3fd0_0000_0000_0000,
                0x3f70_0000_0000_0000,
            ],
            &labels,
        )?;
    }
    Ok(())
}

/// Advance candidate/control from the same genuine fixture without cloning credit.
/// # Errors
/// Preserve actual transition, observer and complete-state comparison failures.
fn capacity_rounds(
    p: &Profile,
    owners: &mut Owners,
    built: &Built,
    batch: &FixtureProjectedBatch<'_>,
    metadata: &[FixtureSupporterUncertainty<'_>],
    expected: ExpectedRound,
) -> TestResult {
    for state in [&mut owners.state, &mut owners.control] {
        let outcome = p.call(Call::Advance, || {
            state.advance(batch, &built.strata, metadata)
        })??;
        observe_capacity_round(p, &outcome, built, expected)?;
    }
    owners.same(p)
}

/// Exercise the exact supported capacity beside every raw one-over admission.
///
/// One sixteen-source fixture owns 48 appends and sixteen projections. The exact
/// actual-call vector is (16,48,48,16,49,96,48,264,16,16,16,1,1,1,16,1,4,0,0,6,16,4).
/// The generic four-seed ceiling conservatively reserves four parameter/minimum
/// constructors; only one of each occurs, because immutable checked values are
/// reused. Added numerical work is at most48 fits and3,072 point admissions.
/// # Errors
/// Preserve genuine producer/constructor/transition and pre-call accounting errors.
/// # Panics
/// Capacity precedence, atomicity and exactly-once round emission must hold.
#[test]
fn lifecycle_capacity_neighbors_preserve_pending_credit() -> TestResult {
    let p = Profile::new(
        "lifecycle_capacity_neighbors_preserve_pending_credit",
        Limits {
            histories: 16,
            appends: 48,
            projections: 16,
            batches: 1,
            seeds: 4,
            advances: 6,
            reachability_updates: 0,
            reachability_records: 0,
            uncertainties: 16,
            equalities: 4,
        },
    );
    let sources = (101_u64..117)
        .map(|label| Source::new(&p, label, [0; 3]))
        .collect::<TestResult<Vec<_>>>()?;
    let specification = sources
        .iter()
        .map(|source| (source, 0.125, 1))
        .collect::<Vec<_>>();
    let built = Built::new(&p, &specification, 2_000_000_000)?;
    let batch = built.batch(&p)?;
    let metadata = records(&p, &built, &[STANDARD; 16])?;
    let mut oversized_roster = built.roster.clone();
    oversized_roster.push(tag(117)?);
    let mut oversized_strata = built.strata.clone();
    let extra_stratum = built
        .strata
        .first()
        .copied()
        .ok_or_else(|| io::Error::other("missing capacity stratum"))?;
    oversized_strata.push(extra_stratum);
    let mut oversized_metadata = metadata.clone();
    let extra_metadata = metadata
        .first()
        .copied()
        .ok_or_else(|| io::Error::other("missing capacity uncertainty"))?;
    oversized_metadata.push(extra_metadata);
    let mut owners =
        seed_capacity_owners(&p, &built, &batch, &oversized_roster, &oversized_strata)?;
    reject_capacity_advances(
        &p,
        &mut owners,
        &built,
        &batch,
        &metadata,
        &oversized_strata,
        &oversized_metadata,
    )?;
    capacity_rounds(&p, &mut owners, &built, &batch, &metadata, ready(1, 16, 16))?;
    capacity_rounds(&p, &mut owners, &built, &batch, &metadata, retained(16))?;
    assert_eq!(p.get(|| owners.state.emitted_rounds())?, 1);
    assert_eq!(p.get(|| owners.control.emitted_rounds())?, 1);
    Ok(())
}

/// Independent S152 fitted-tracking journeys over genuine adaptive owners.
///
/// All rows, scores, intervals and outcomes come through public checked owners;
/// no product field, tracking payload, witness or lifecycle state is fabricated.
/// Source-owned finite work is accounted separately from the original profiles.
mod fitted_tracking_journey {
    use std::{error::Error, io, num::NonZeroU64};

    use time_core::{
        FixtureAdaptiveHistory, FixtureAnchorProjection, FixtureCombinationDenominatorField,
        FixtureCombinationError, FixtureCombinationLifecycleError, FixtureCombinationOutcome,
        FixtureCombinationParameters, FixtureCombinationSourceStatus, FixtureCombinationState,
        FixtureContext, FixtureMinimum, FixtureObservation, FixturePrincipalOutcomeKind,
        FixturePrincipalParameters, FixturePrincipalStratum, FixtureProjectedBatch,
        FixtureProjectedInterval, FixtureProjectionError, FixtureProjectionIssue,
        FixtureProjectionTerms, FixtureSupporterUncertainty, FixtureTrackingCombinationError,
        FixtureTrackingField, FixtureTrackingLifecycleError, FixtureTrackingProjectionError,
        combine_fixture_supporters, combine_fixture_tracking_supporters,
        enclose_fixture_projection, project_fixture_adaptive_anchor,
        project_fixture_adaptive_tracking,
    };
    use time_domain::{MonotonicNs, UtcCoordinateQ32, exchange_arithmetic};

    type TestResult<T = ()> = Result<T, Box<dyn Error>>;
    const SECOND: u64 = 1_000_000_000;
    const RELATIVE_TOLERANCE: f64 = 1.0 / 274_877_906_944.0;

    /// Admit a fixed nonzero fixture label without effects.
    /// # Errors
    /// Reject zero before publication.
    fn label(value: u64) -> TestResult<NonZeroU64> {
        NonZeroU64::new(value).ok_or_else(|| io::Error::other("zero label").into())
    }

    /// Compare nonzero finite fixture outputs against a pre-frozen relative bound.
    /// # Panics
    /// Reject zero, nonfinite values or relative error above 2^-38.
    fn near(actual: f64, expected: f64) {
        assert!(actual.is_finite() && actual != 0.0 && expected != 0.0);
        assert!((actual / expected - 1.0).abs() <= RELATIVE_TOLERANCE);
    }

    /// One genuine adaptive owner and its unchanged caller-asserted identity.
    struct Source {
        context: FixtureContext,
        history: FixtureAdaptiveHistory,
    }

    impl Source {
        /// Build at most nine exact, chronologically ordered Q32 input phases.
        /// # Errors
        /// Preserve all genuine coordinate, observation, append and fit failures.
        fn new(id: u64, values: &[i32], shift: u32, divisors: &[f64]) -> TestResult<Self> {
            if values.len() != divisors.len() || values.len() > 9 {
                return Err(io::Error::other("fixture row bound").into());
            }
            let context = FixtureContext::new(label(id)?, label(17)?, label(23)?);
            let mut result = Self {
                context,
                history: FixtureAdaptiveHistory::new(context, 3)?,
            };
            for (index, (&value, &divisor)) in values.iter().zip(divisors).enumerate() {
                result.append(u64::try_from(index)?, value, shift, divisor)?;
            }
            Ok(result)
        }

        /// Append one checked exact source observation at a supplied second.
        /// # Errors
        /// Reject overflow or preserve the actual domain and owner failures.
        fn append(&mut self, second: u64, value: i32, shift: u32, divisor: f64) -> TestResult {
            let ticks = i128::from(value)
                .checked_shl(shift)
                .ok_or_else(|| io::Error::other("fixture shift"))?;
            let time = second
                .checked_mul(SECOND)
                .ok_or_else(|| io::Error::other("time overflow"))?;
            let zero = UtcCoordinateQ32::new(0)?;
            let shifted = UtcCoordinateQ32::new(ticks)?;
            let phase = exchange_arithmetic(zero, shifted, shifted, zero)?.offset();
            let row =
                FixtureObservation::new(self.context, MonotonicNs::new(time), phase, divisor)?;
            self.history.try_append(row)?;
            Ok(())
        }

        /// Project the actual requested fitted anchor with or without tracking.
        /// # Errors
        /// Preserve genuine projection, skew and standard-error failures.
        fn project(
            &self,
            index: usize,
            anchor: u64,
            at: u64,
            terms: FixtureProjectionTerms,
            tracked: bool,
        ) -> TestResult<FixtureAnchorProjection> {
            let anchor = MonotonicNs::new(
                anchor
                    .checked_mul(SECOND)
                    .ok_or_else(|| io::Error::other("anchor overflow"))?,
            );
            let at = MonotonicNs::new(
                at.checked_mul(SECOND)
                    .ok_or_else(|| io::Error::other("projection overflow"))?,
            );
            if tracked {
                Ok(project_fixture_adaptive_tracking(
                    &self.history,
                    self.context,
                    index,
                    anchor,
                    at,
                    terms,
                    0.125,
                )?)
            } else {
                Ok(project_fixture_adaptive_anchor(
                    &self.history,
                    self.context,
                    index,
                    anchor,
                    at,
                    terms,
                )?)
            }
        }
    }

    /// Build the exact pair into final contiguous owned storage.
    /// # Errors
    /// Preserve actual producer and enclosure failures; no batch is forged.
    fn entries(
        a: &Source,
        b: &Source,
        at: u64,
        a_terms: FixtureProjectionTerms,
        b_terms: FixtureProjectionTerms,
        a_tracking: bool,
        b_tracking: bool,
    ) -> TestResult<[FixtureProjectedInterval; 2]> {
        let ap = a.project(2, 2, at, a_terms, a_tracking)?;
        let bp = b.project(2, 2, at, b_terms, b_tracking)?;
        Ok([
            enclose_fixture_projection(&ap)?,
            enclose_fixture_projection(&bp)?,
        ])
    }

    /// Create a genuine pair batch borrowing only its final original slots.
    /// # Errors
    /// Preserve batch owner failures; no copied entries are substituted.
    fn batch(
        entries: &[FixtureProjectedInterval],
        at: u64,
    ) -> TestResult<FixtureProjectedBatch<'_>> {
        let nanos = at
            .checked_mul(SECOND)
            .ok_or_else(|| io::Error::other("batch time overflow"))?;
        Ok(FixtureProjectedBatch::new(
            label(17)?,
            label(23)?,
            MonotonicNs::new(nanos),
            entries,
        )?)
    }

    /// Seed an independent real lifecycle owner using only fixed explicit parameters.
    /// # Errors
    /// Preserve actual principal, minimum and lifecycle admission failures.
    fn seed(
        batch: &FixtureProjectedBatch<'_>,
        roster: &[NonZeroU64],
        strata: &[FixturePrincipalStratum],
        minimum: usize,
    ) -> TestResult<FixtureCombinationState> {
        Ok(FixtureCombinationState::seed(
            roster,
            FixturePrincipalParameters::new(1.0 / 1024.0, 0.0)?,
            FixtureCombinationParameters::new(2.0, 0.0)?,
            FixtureMinimum::new(minimum)?,
            batch,
            strata,
        )?)
    }

    /// Borrow the required actual tracking attachment without inventing a fallback.
    /// # Errors
    /// Reject an absent attachment before any numerical assertion.
    fn tracking(
        projection: &FixtureAnchorProjection,
    ) -> TestResult<&time_core::FixtureTrackingEstimate> {
        projection
            .tracking()
            .ok_or_else(|| io::Error::other("tracking absent").into())
    }

    /// Observe the exact raw interval and attached fitted facts for both sources.
    /// # Errors
    /// Reject a missing genuine attachment.
    /// # Panics
    /// Reject altered endpoint, raw phase, fitted phase, SD or skew facts.
    fn check_pair(entries: &[FixtureProjectedInterval; 2]) -> TestResult {
        assert_eq!(
            (entries[0].interval().lower(), entries[0].interval().upper()),
            (-3_579_139_414, 5_010_795_179)
        );
        assert_eq!(
            (entries[1].interval().lower(), entries[1].interval().upper()),
            (-3_042_268_502, 5_547_666_091)
        );
        for (entry, raw, fitted) in [
            (&entries[0], 1.0 / 12.0, 7.0 / 36.0),
            (&entries[1], 7.0 / 48.0, 37.0 / 144.0),
        ] {
            near(entry.projection().center_seconds(), raw);
            let attached = tracking(entry.projection())?;
            near(attached.phase_seconds(), fitted);
            near(
                attached.phase_standard_deviation_seconds(),
                0.290_892_665_416_655,
            );
            near(
                attached.rate_standard_deviation_seconds_per_second(),
                1.0 / 12.0,
            );
            assert_eq!(
                attached.tracking_skew_seconds_per_second().to_bits(),
                0.125_f64.to_bits()
            );
        }
        Ok(())
    }

    /// Inspect an actual ready outcome and both explicit combination modes.
    /// # Errors
    /// Preserve actual missing-witness, metadata and kernel failures.
    /// # Panics
    /// Reject a fabricated principal/round, witness or either numerical route.
    fn check_ready(
        ready: &FixtureCombinationOutcome<'_>,
        actual_batch: &FixtureProjectedBatch<'_>,
        roster: [NonZeroU64; 2],
    ) -> TestResult {
        assert_eq!(
            ready.principal().kind(),
            FixturePrincipalOutcomeKind::ReferenceReady
        );
        assert_eq!(ready.principal().principal_label(), Some(roster[0]));
        assert_eq!(ready.witness_count(), Some(2));
        assert_eq!(ready.contributor_count(), Some(2));
        assert_eq!(ready.emitted_round(), Some(label(1)?));
        assert_eq!(
            ready.source_status(roster[0]),
            Some(FixtureCombinationSourceStatus::Principal)
        );
        assert_eq!(
            ready.source_status(roster[1]),
            Some(FixtureCombinationSourceStatus::Contributor)
        );
        let witness = ready
            .principal()
            .selection()
            .witnessed()
            .ok_or_else(|| io::Error::other("missing witness"))?;
        assert_eq!(
            (
                witness.component_lower(),
                witness.component_upper(),
                witness.witness()
            ),
            (-3_042_268_502, 5_010_795_179, -3_042_268_502)
        );
        let combined = ready
            .combined()
            .ok_or_else(|| io::Error::other("missing estimate"))?;
        near(combined.phase_seconds(), 65.0 / 288.0);
        near(
            combined.phase_standard_deviation_seconds(),
            0.294_230_637_074_397_8,
        );
        near(combined.phase_rate_seconds_per_second(), 1.0 / 24.0);
        near(
            combined.rate_standard_deviation_seconds_per_second(),
            0.058_925_565_098_878_96,
        );
        near(
            combined.tracking_skew_seconds_per_second(),
            0.088_388_347_648_318_45,
        );
        let direct = combine_fixture_tracking_supporters(ready.principal())?;
        near(
            direct
                .combined()
                .ok_or_else(|| io::Error::other("missing direct estimate"))?
                .phase_seconds(),
            65.0 / 288.0,
        );
        let legacy = [
            FixtureSupporterUncertainty::new(&actual_batch.entries()[0], 0.25, 0.125, 0.125)?,
            FixtureSupporterUncertainty::new(&actual_batch.entries()[1], 0.25, 0.125, 0.125)?,
        ];
        let legacy_result = combine_fixture_supporters(ready.principal(), &legacy)?;
        near(
            legacy_result
                .combined()
                .ok_or_else(|| io::Error::other("missing legacy estimate"))?
                .phase_seconds(),
            11.0 / 96.0,
        );
        Ok(())
    }

    /// Observe source watermark, pending credit and penalty without mutation.
    /// # Errors
    /// Reject a label absent from either actual owner view.
    fn credit(
        owner: &FixtureCombinationState,
        label: NonZeroU64,
    ) -> TestResult<(Option<MonotonicNs>, bool, u8)> {
        let principal = owner
            .principal_state()
            .source_state(label)
            .ok_or_else(|| io::Error::other("missing principal source"))?;
        let lifecycle = owner
            .source_state(label)
            .ok_or_else(|| io::Error::other("missing lifecycle source"))?;
        Ok((
            principal.latest_observed_at(),
            principal.pending(),
            lifecycle.penalty(),
        ))
    }

    /// The full adaptive→copied enclosure→batch→principal/kernel/lifecycle route.
    /// # Errors
    /// Preserve all genuine producer, admission and lifecycle failures.
    /// # Panics
    /// Reject changed raw witness, fitted facts, legacy route or observation credit.
    #[test]
    fn fitted_journey_preserves_raw_witness_legacy_mode_and_real_credit() -> TestResult {
        let mut a = Source::new(11, &[0, 2, 0], 28, &[1.0, 1.0, 4.0])?;
        let b = Source::new(22, &[1, 3, 1], 28, &[1.0, 1.0, 4.0])?;
        let terms = FixtureProjectionTerms::new(0.25, 0.0, 0.125)?;
        let original = entries(&a, &b, 4, terms, terms, true, true)?;
        let actual_batch = batch(&original, 4)?;
        let strata = [
            FixturePrincipalStratum::new(label(11)?, 2)?,
            FixturePrincipalStratum::new(label(22)?, 2)?,
        ];
        let roster = [label(11)?, label(22)?];
        check_pair(&original)?;
        let mut owner = seed(&actual_batch, &roster, &strata, 2)?;
        let ready = owner.advance_tracking(&actual_batch, &strata)?;
        check_ready(&ready, &actual_batch, roster)?;
        let credits = [credit(&owner, roster[0])?, credit(&owner, roster[1])?];
        let later = entries(&a, &b, 5, terms, terms, true, true)?;
        let later_batch = batch(&later, 5)?;
        let retained = owner.advance_tracking(&later_batch, &strata)?;
        assert_eq!(
            retained.principal().kind(),
            FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate
        );
        assert_eq!(retained.emitted_round(), None);
        assert_eq!(
            [credit(&owner, roster[0])?, credit(&owner, roster[1])?],
            credits
        );
        assert_eq!(owner.emitted_rounds(), 1);
        a.append(3, 0, 28, 1.0)?;
        let fresh_a = a.project(3, 3, 5, terms, true)?;
        let fresh_b = b.project(2, 2, 5, terms, true)?;
        let fresh = [
            enclose_fixture_projection(&fresh_a)?,
            enclose_fixture_projection(&fresh_b)?,
        ];
        let fresh_batch = batch(&fresh, 5)?;
        let next = owner.advance_tracking(&fresh_batch, &strata)?;
        assert_eq!(next.principal().updated_source(), Some(roster[0]));
        assert_eq!(next.emitted_round(), Some(label(2)?));
        Ok(())
    }

    /// Existing raw-only producers and legacy lifecycle remain a healthy control.
    /// # Errors
    /// Preserve actual source, batch, metadata and lifecycle failures.
    /// # Panics
    /// Reject a tracking attachment, changed legacy center or missing round.
    #[test]
    fn legacy_raw_only_neighbor_remains_unchanged() -> TestResult {
        let a = Source::new(11, &[0, 2, 0], 28, &[1.0, 1.0, 4.0])?;
        let b = Source::new(22, &[1, 3, 1], 28, &[1.0, 1.0, 4.0])?;
        let terms = FixtureProjectionTerms::new(0.25, 0.0, 0.125)?;
        let original = entries(&a, &b, 4, terms, terms, false, false)?;
        assert!(original[0].projection().tracking().is_none());
        assert!(original[1].projection().tracking().is_none());
        let actual_batch = batch(&original, 4)?;
        let strata = [
            FixturePrincipalStratum::new(label(11)?, 2)?,
            FixturePrincipalStratum::new(label(22)?, 2)?,
        ];
        let roster = [label(11)?, label(22)?];
        let metadata = [
            FixtureSupporterUncertainty::new(&actual_batch.entries()[0], 0.25, 0.125, 0.125)?,
            FixtureSupporterUncertainty::new(&actual_batch.entries()[1], 0.25, 0.125, 0.125)?,
        ];
        let mut owner = seed(&actual_batch, &roster, &strata, 2)?;
        let ready = owner.advance(&actual_batch, &strata, &metadata)?;
        assert_eq!(ready.emitted_round(), Some(label(1)?));
        near(
            ready
                .combined()
                .ok_or_else(|| io::Error::other("missing legacy estimate"))?
                .phase_seconds(),
            11.0 / 96.0,
        );
        Ok(())
    }

    /// Selected post-prune geometry excludes the earlier auxiliary decision row.
    /// # Errors
    /// Preserve genuine history, projection and tracking failures.
    /// # Panics
    /// Reject wrong fitted population, raw-anchor identity or negative tracked phase.
    #[test]
    fn post_prune_tracking_uses_fitted_suffix_and_oldest_fitted_anchor() -> TestResult {
        let source = Source::new(
            11,
            &[512, -21, 27, 27, 27, -27, -27, -27, -27],
            23,
            &[1.0; 9],
        )?;
        assert_eq!(
            (
                source.history.sample_count(),
                source.history.auxiliary_count()
            ),
            (8, 1)
        );
        let terms = FixtureProjectionTerms::new(0.25, 0.0, 0.125)?;
        let projected = source.project(0, 1, 9, terms, true)?;
        near(projected.center_seconds(), -499.0 / 3584.0);
        let attached = tracking(&projected)?;
        near(attached.phase_seconds(), -15.0 / 224.0);
        near(
            attached.phase_standard_deviation_seconds(),
            0.038_245_511_221_332_42,
        );
        Ok(())
    }

    /// Two genuine same-context histories cannot be conflated by their metadata.
    /// # Errors
    /// Preserve genuine history and projection failures.
    /// # Panics
    /// Reject an inferred attachment, rebound estimate or changed raw projection.
    #[test]
    fn distinct_same_context_histories_keep_their_own_tracking_payloads() -> TestResult {
        assert!(size_of::<Option<time_core::FixtureTrackingEstimate>>() <= 40);
        let a = Source::new(11, &[0, 2, 0], 28, &[1.0, 1.0, 4.0])?;
        let twin = Source::new(11, &[1, 3, 1], 28, &[1.0, 1.0, 4.0])?;
        assert_eq!(a.context, twin.context);
        let terms = FixtureProjectionTerms::new(0.25, 0.0, 0.125)?;
        let ap = a.project(2, 2, 4, terms, true)?;
        let tp = twin.project(2, 2, 4, terms, true)?;
        let raw = a.project(2, 2, 4, terms, false)?;
        assert!(raw.tracking().is_none());
        near(tracking(&ap)?.phase_seconds(), 7.0 / 36.0);
        near(tracking(&tp)?.phase_seconds(), 37.0 / 144.0);
        let copied = enclose_fixture_projection(&ap)?;
        let original = tracking(&ap)?;
        let retained = tracking(copied.projection())?;
        assert_eq!(
            retained.phase_seconds().to_bits(),
            original.phase_seconds().to_bits()
        );
        assert_eq!(
            retained.phase_standard_deviation_seconds().to_bits(),
            original.phase_standard_deviation_seconds().to_bits()
        );
        assert_eq!(
            retained
                .rate_standard_deviation_seconds_per_second()
                .to_bits(),
            original
                .rate_standard_deviation_seconds_per_second()
                .to_bits()
        );
        assert_eq!(
            retained.tracking_skew_seconds_per_second().to_bits(),
            original.tracking_skew_seconds_per_second().to_bits()
        );
        Ok(())
    }

    /// Whole witness admission precedes filtering and missing tracking rolls back.
    /// # Errors
    /// Preserve genuine seed/projection construction failures.
    /// # Panics
    /// Reject an omitted later-excluded entry, fabricated partial result or commit.
    #[test]
    fn missing_later_excluded_tracking_preserves_complete_lifecycle() -> TestResult {
        let a = Source::new(11, &[0, 2, 0], 28, &[1.0, 1.0, 4.0])?;
        let b = Source::new(22, &[1, 3, 1], 28, &[1.0, 1.0, 4.0])?;
        let a_terms = FixtureProjectionTerms::new(0.25, 0.0, 0.125)?;
        let b_terms = FixtureProjectionTerms::new(1.5, 0.0, 0.0)?;
        let actual = entries(&a, &b, 4, a_terms, b_terms, true, false)?;
        let mixed_batch = batch(&actual, 4)?;
        let strata = [
            FixturePrincipalStratum::new(label(11)?, 2)?,
            FixturePrincipalStratum::new(label(22)?, 2)?,
        ];
        let roster = [label(11)?, label(22)?];
        let complete = entries(&a, &b, 4, a_terms, b_terms, true, true)?;
        let complete_batch = batch(&complete, 4)?;
        let mut healthy = seed(&complete_batch, &roster, &strata, 2)?;
        let ready = healthy.advance_tracking(&complete_batch, &strata)?;
        assert_eq!(ready.principal().principal_label(), Some(roster[0]));
        assert_eq!(ready.witness_count(), Some(2));
        assert_eq!(ready.contributor_count(), Some(1));
        assert_eq!(
            ready.source_status(roster[1]),
            Some(FixtureCombinationSourceStatus::DistanceExcluded)
        );
        near(
            ready
                .combined()
                .ok_or_else(|| io::Error::other("missing excluded-source estimate"))?
                .phase_seconds(),
            7.0 / 36.0,
        );
        let mut owner = seed(&mixed_batch, &roster, &strata, 2)?;
        let control = seed(&mixed_batch, &roster, &strata, 2)?;
        let rejected = owner.advance_tracking(&mixed_batch, &strata).err();
        let expected = Some(FixtureTrackingLifecycleError::MissingTracking { index: 1 });
        assert_eq!(rejected, expected);
        assert_eq!(owner, control);
        Ok(())
    }

    /// An unavailable genuine decision does not fabricate tracking requirements.
    /// # Errors
    /// Preserve genuine seed, projection and decision failures.
    /// # Panics
    /// Reject invented principal, combined result or observation credit.
    #[test]
    fn unavailable_principal_preserves_no_estimate_with_mixed_attachments() -> TestResult {
        let a = Source::new(11, &[-14, -14, -14], 28, &[1.0; 3])?;
        let b = Source::new(22, &[14, 14, 14], 28, &[1.0; 3])?;
        let terms = FixtureProjectionTerms::new(1.0 / 32.0, 0.0, 0.0)?;
        let actual = entries(&a, &b, 4, terms, terms, true, false)?;
        let batch = batch(&actual, 4)?;
        let strata = [
            FixturePrincipalStratum::new(label(11)?, 2)?,
            FixturePrincipalStratum::new(label(22)?, 2)?,
        ];
        let roster = [label(11)?, label(22)?];
        let mut owner = seed(&batch, &roster, &strata, 2)?;
        let unavailable = owner.advance_tracking(&batch, &strata)?;
        assert_eq!(
            unavailable.principal().kind(),
            FixturePrincipalOutcomeKind::Unavailable
        );
        assert_eq!(unavailable.principal().principal_label(), None);
        assert!(unavailable.combined().is_none());
        assert_eq!(unavailable.emitted_round(), None);
        let direct = combine_fixture_tracking_supporters(unavailable.principal())?;
        assert!(direct.combined().is_none());
        assert_eq!(owner.emitted_rounds(), 0);
        Ok(())
    }

    /// An untracked entry outside the genuine witness remains a legal nonvoter.
    /// # Errors
    /// Preserve genuine producer, batch and lifecycle failures.
    /// # Panics
    /// Reject a fabricated requirement on a nonwitness or changed combined result.
    #[test]
    fn untracked_nonwitness_does_not_block_actual_witness() -> TestResult {
        let a = Source::new(11, &[0, 2, 0], 28, &[1.0, 1.0, 4.0])?;
        let b = Source::new(22, &[1, 3, 1], 28, &[1.0, 1.0, 4.0])?;
        let outsider = Source::new(33, &[-16, -16, -16], 28, &[1.0; 3])?;
        let pair_terms = FixtureProjectionTerms::new(0.25, 0.0, 0.125)?;
        let outsider_terms = FixtureProjectionTerms::new(1.0 / 32.0, 0.0, 0.0)?;
        let pair = entries(&a, &b, 4, pair_terms, pair_terms, true, true)?;
        let absent = outsider.project(2, 2, 4, outsider_terms, false)?;
        let actual = [pair[0], pair[1], enclose_fixture_projection(&absent)?];
        let batch = batch(&actual, 4)?;
        let roster = [label(11)?, label(22)?, label(33)?];
        let strata = [
            FixturePrincipalStratum::new(label(11)?, 2)?,
            FixturePrincipalStratum::new(label(22)?, 2)?,
            FixturePrincipalStratum::new(label(33)?, 2)?,
        ];
        let mut owner = seed(&batch, &roster, &strata, 2)?;
        let ready = owner.advance_tracking(&batch, &strata)?;
        assert_eq!(ready.witness_count(), Some(2));
        assert_eq!(
            ready.source_status(roster[2]),
            Some(FixtureCombinationSourceStatus::OutsideWitness)
        );
        near(
            ready
                .combined()
                .ok_or_else(|| io::Error::other("missing estimate"))?
                .phase_seconds(),
            65.0 / 288.0,
        );
        Ok(())
    }

    /// Check both public tracking error wrappers against an actual delegated cause.
    ///
    /// The caller supplies a genuine failed tracking lifecycle reduction. Three
    /// source requests/downcasts retain that exact lifecycle and combination cause;
    /// the copied public error variant constructs no checked estimator or state.
    /// # Errors
    /// Reject an unexpected wrapper or a missing correctly typed delegated source.
    /// # Panics
    /// Reject any delegated cause that differs from its actual stored value.
    fn check_tracking_failure_sources(error: &FixtureTrackingLifecycleError) -> TestResult {
        let FixtureTrackingLifecycleError::Lifecycle { source: lifecycle } = error else {
            return Err(io::Error::other("missing lifecycle failure").into());
        };
        let actual_lifecycle = error
            .source()
            .and_then(|source| source.downcast_ref::<FixtureCombinationLifecycleError>())
            .ok_or_else(|| io::Error::other("lost lifecycle cause"))?;
        assert_eq!(actual_lifecycle, lifecycle);
        let FixtureCombinationLifecycleError::Combination {
            source: combination,
        } = lifecycle
        else {
            return Err(io::Error::other("missing combination failure").into());
        };
        let actual_combination = actual_lifecycle
            .source()
            .and_then(|source| source.downcast_ref::<FixtureCombinationError>())
            .ok_or_else(|| io::Error::other("lost combination cause"))?;
        assert_eq!(actual_combination, combination);
        let tracking = FixtureTrackingCombinationError::Combination {
            source: *actual_combination,
        };
        assert_eq!(
            tracking
                .source()
                .and_then(|source| source.downcast_ref::<FixtureCombinationError>()),
            Some(combination)
        );
        Ok(())
    }

    /// A real singleton preserves zero, while an original pair keeps denominator checks.
    /// # Errors
    /// Preserve genuine owner construction and healthy singleton failures.
    /// # Panics
    /// Reject a zero floor, pair-to-singleton shortcut or partial lifecycle commit.
    #[test]
    fn singleton_zero_and_original_pair_denominators_are_distinct() -> TestResult {
        let zero = Source::new(11, &[0, 0, 0], 28, &[1.0; 3])?;
        let b = Source::new(22, &[1, 3, 1], 28, &[1.0, 1.0, 4.0])?;
        let terms = FixtureProjectionTerms::new(0.5, 0.0, 0.0)?;
        let one_projection = zero.project(2, 2, 4, terms, true)?;
        let one = [enclose_fixture_projection(&one_projection)?];
        let one_batch = batch(&one, 4)?;
        let roster = [label(11)?];
        let strata = [FixturePrincipalStratum::new(label(11)?, 2)?];
        let mut singleton = seed(&one_batch, &roster, &strata, 1)?;
        let ready = singleton.advance_tracking(&one_batch, &strata)?;
        assert_eq!(ready.emitted_round(), Some(label(1)?));
        let combined = ready
            .combined()
            .ok_or_else(|| io::Error::other("missing singleton"))?;
        assert_eq!(
            combined
                .rate_standard_deviation_seconds_per_second()
                .to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(
            combined.phase_standard_deviation_seconds().to_bits(),
            0.0_f64.to_bits()
        );
        for b_distance in [0.5, 1.5] {
            let other_terms = FixtureProjectionTerms::new(b_distance, 0.0, 0.0)?;
            let actual = entries(&zero, &b, 4, terms, other_terms, true, true)?;
            let pair = batch(&actual, 4)?;
            let roster = [label(11)?, label(22)?];
            let strata = [
                FixturePrincipalStratum::new(label(11)?, 2)?,
                FixturePrincipalStratum::new(label(22)?, 2)?,
            ];
            let mut owner = seed(&pair, &roster, &strata, 2)?;
            let control = seed(&pair, &roster, &strata, 2)?;
            let actual = owner.advance_tracking(&pair, &strata).err();
            let expected = Some(FixtureTrackingLifecycleError::Lifecycle {
                source: FixtureCombinationLifecycleError::Combination {
                    source: FixtureCombinationError::ZeroDenominator {
                        index: 0,
                        field: FixtureCombinationDenominatorField::RateStandardDeviation,
                    },
                },
            });
            assert_eq!(actual, expected);
            check_tracking_failure_sources(
                actual
                    .as_ref()
                    .ok_or_else(|| io::Error::other("missing actual denominator failure"))?,
            )?;
            assert_eq!(owner, control);
        }
        Ok(())
    }

    /// Raw projection association errors retain precedence over invalid new skew.
    /// # Errors
    /// Preserve the genuine checked source and term construction failures.
    /// # Panics
    /// Reject changed typed precedence, new attachment on an old path or mutation.
    #[test]
    fn projection_errors_precede_invalid_tracking_skew() -> TestResult {
        let source = Source::new(11, &[0, 2, 0], 28, &[1.0, 1.0, 4.0])?;
        let terms = FixtureProjectionTerms::new(0.25, 0.0, 0.125)?;
        let rejected = project_fixture_adaptive_tracking(
            &source.history,
            source.context,
            2,
            MonotonicNs::new(2 * SECOND),
            MonotonicNs::new(SECOND),
            terms,
            f64::NAN,
        )
        .err();
        let expected = Some(FixtureTrackingProjectionError::Projection {
            source: FixtureProjectionError::BackwardProjection,
        });
        assert_eq!(rejected, expected);
        let rejected = project_fixture_adaptive_tracking(
            &source.history,
            source.context,
            2,
            MonotonicNs::new(2 * SECOND),
            MonotonicNs::new(4 * SECOND),
            terms,
            f64::NAN,
        )
        .err();
        let expected = Some(FixtureTrackingProjectionError::InvalidScalar {
            field: FixtureTrackingField::TrackingSkew,
            issue: FixtureProjectionIssue::NonFinite,
        });
        assert_eq!(rejected, expected);
        for (skew, issue) in [
            (f64::INFINITY, FixtureProjectionIssue::NonFinite),
            (f64::NEG_INFINITY, FixtureProjectionIssue::NonFinite),
            (f64::from_bits(1), FixtureProjectionIssue::Subnormal),
            (-f64::from_bits(1), FixtureProjectionIssue::Subnormal),
            (-1.0, FixtureProjectionIssue::Negative),
        ] {
            let actual = project_fixture_adaptive_tracking(
                &source.history,
                source.context,
                2,
                MonotonicNs::new(2 * SECOND),
                MonotonicNs::new(4 * SECOND),
                terms,
                skew,
            )
            .err();
            let expected = Some(FixtureTrackingProjectionError::InvalidScalar {
                field: FixtureTrackingField::TrackingSkew,
                issue,
            });
            assert_eq!(actual, expected);
        }
        for skew in [0.0_f64, -0.0, f64::MIN_POSITIVE] {
            let projection = project_fixture_adaptive_tracking(
                &source.history,
                source.context,
                2,
                MonotonicNs::new(2 * SECOND),
                MonotonicNs::new(4 * SECOND),
                terms,
                skew,
            )?;
            assert_eq!(
                tracking(&projection)?
                    .tracking_skew_seconds_per_second()
                    .to_bits(),
                skew.to_bits()
            );
        }
        let old = source.project(2, 2, 4, terms, false)?;
        assert!(old.tracking().is_none());
        near(old.center_seconds(), 1.0 / 12.0);
        Ok(())
    }

    /// Preserve all sixteen genuine tracked supporters at the supported capacity.
    ///
    /// Sixteen bounded three-row adaptive owners supply final contiguous entries
    /// before the single batch is borrowed. One lifecycle advance and one direct
    /// kernel consume its same genuine decision; both build sixteen-record holders.
    /// The independently frozen equal-source oracle has no principal disagreement.
    /// # Errors
    /// Preserve genuine history, projection, enclosure, batch and reduction failures.
    /// # Panics
    /// Reject a lost witness, capacity off-by-one, raw-phase substitution, changed
    /// canonical contributor set, principal, event ordinal or combined uncertainty.
    #[test]
    fn sixteen_tracking_supporters_preserve_complete_witness() -> TestResult {
        let mut actual = Vec::with_capacity(16);
        let mut roster = Vec::with_capacity(16);
        let mut strata = Vec::with_capacity(16);
        let terms = FixtureProjectionTerms::new(0.25, 0.0, 0.125)?;
        for id in 1..=16 {
            let source = Source::new(id, &[0, 2, 0], 28, &[1.0, 1.0, 4.0])?;
            let source_label = source.context.source_id();
            let projection = source.project(2, 2, 4, terms, true)?;
            near(projection.center_seconds(), 1.0 / 12.0);
            actual.push(enclose_fixture_projection(&projection)?);
            roster.push(source_label);
            strata.push(FixturePrincipalStratum::new(
                source_label,
                if id == 1 { 1 } else { 2 },
            )?);
        }
        let actual_batch = batch(&actual, 4)?;
        let mut owner = FixtureCombinationState::seed(
            &roster,
            FixturePrincipalParameters::new(1.0 / 1024.0, 1.0 / 1024.0)?,
            FixtureCombinationParameters::new(2.0, 0.0)?,
            FixtureMinimum::new(16)?,
            &actual_batch,
            &strata,
        )?;
        let ready = owner.advance_tracking(&actual_batch, &strata)?;
        let decision = ready.principal();
        assert_eq!(decision.kind(), FixturePrincipalOutcomeKind::ReferenceReady);
        assert_eq!(decision.principal_label(), Some(roster[0]));
        assert_eq!(ready.witness_count(), Some(16));
        assert_eq!(ready.contributor_count(), Some(16));
        assert_eq!(ready.emitted_round(), Some(label(1)?));
        let selection = decision.selection();
        assert_eq!(selection.population(), 16);
        assert_eq!(selection.minimum(), 16);
        let witness = selection
            .witnessed()
            .ok_or_else(|| io::Error::other("missing capacity witness"))?;
        assert_eq!(witness.supporters().collect::<Vec<_>>(), roster);
        let direct = combine_fixture_tracking_supporters(decision)?;
        for result in [ready.combined(), direct.combined()] {
            let combined = result.ok_or_else(|| io::Error::other("missing capacity estimate"))?;
            assert_eq!(combined.supporter_count(), 16);
            assert_eq!(combined.supporters().collect::<Vec<_>>(), roster);
            assert_eq!(combined.principal_label(), roster[0]);
            near(combined.phase_seconds(), 7.0 / 36.0);
            near(combined.phase_rate_seconds_per_second(), 1.0 / 24.0);
            near(
                combined.phase_standard_deviation_seconds(),
                0.290_892_665_416_655,
            );
            near(
                combined.rate_standard_deviation_seconds_per_second(),
                1.0 / 48.0,
            );
            near(combined.tracking_skew_seconds_per_second(), 1.0 / 32.0);
        }
        Ok(())
    }
}

// SPDX-License-Identifier: GPL-2.0-only
//! Eleven independent CHECK-7/8/14/25 lifecycle cases through real producers.
//!
//! UTC/exchange arithmetic supplies exact Q33 phases; real histories own fits,
//! original projections/enclosures own interval witnesses, and the real principal
//! owns update credit. No decision, mask, checked state or public overflow is
//! fabricated. All time is explicit fixture time, with no external effects.
//!
//! The authored complete-suite upper is 41 histories, 184 append attempts,
//! 183 projections/enclosures, 87 batches, 27 seeds, 161 lifecycle advances,
//! 49 reachability constructors, 42 reachability records and 183 uncertainty
//! constructors. Each source has at most 47 rows; each test at most 98 advances.
//! Combination-parameter constructors have a separate raw-case cap: 47 total,
//! comprising 27 seed constructors and 20 explicit raw-admission attempts.
//! These are source reservations, not observed execution. An append reserves at
//! most one sufficient fit of 64 points; hence <=184 fits/11776 points. A
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
    /// Whole lifecycle-state bit-preserving comparison.
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
    /// At most three original sources per independently seeded fixture.
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
    /// Maximum whole-state comparisons across independent owners.
    equalities: u64,
}

/// Independent finite constructor-only envelope; it reserves no seed or fit.
#[derive(Clone, Copy)]
struct RawLimits {
    /// Actual raw combination-parameter constructor attempts.
    parameters: u64,
    /// Actual raw reachability-update constructor attempts.
    ordinals: u64,
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
    /// The fixed raw case performs 22 immutable getters within the otherwise
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
                equalities: 0,
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
    /// Project <=3 actual histories at a single explicit common instant.
    /// Growth terms are zero; separately asserted tracking skew never ages twice.
    /// # Errors
    /// Reject impossible harness capacity/anchor facts or real producer failure.
    fn new(p: &Profile, sources: &[(&Source, f64, u8)], raw_at: u64) -> TestResult<Self> {
        if sources.is_empty() || sources.len() > 3 {
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
/// the existing raw-neighbor loop additions, diagnostics make at most fifty-nine
/// top-level format calls, each expected string below sixty-four UTF-8 bytes.
/// Thus returned formatted content is at most 3,776 bytes cumulatively; this is
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
            parameters: 20,
            ordinals: 6,
        },
    );
    reject_raw_parameter_neighbors(&p)?;
    for (combine, error) in [
        (2.0, 0.0),
        (2.0, -0.0),
        (f64::MIN_POSITIVE, f64::MIN_POSITIVE),
        (f64::MAX, f64::MAX),
        (2.0, 0.03125),
    ] {
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
    }
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
